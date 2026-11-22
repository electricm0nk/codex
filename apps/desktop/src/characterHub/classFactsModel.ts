import type { ClassFactsDto, ListClassFactsResponse } from '../boundary/listClassFacts';
import type { HeldClass } from './characterProgression';

/**
 * SD-36 Epic F6a: the sheet's per-class facts -- weapon proficiency, caster level, class skills --
 * as the engine serves them (`list_class_facts`). This module decides nothing about any class: it
 * folds the served answers across the classes a character holds (PF1's union for proficiencies
 * and class skills; one caster level per casting class) and names every class the engine could
 * not answer. The hand-kept `MARTIAL_WEAPON_CLASSES` (5 ids), `CASTER_CLASSES` (6 ids) and
 * `CLASS_SKILLS` (12 lists) tables this replaces are deleted.
 */

export type ClassFactsState =
  | { kind: 'loading' }
  | { kind: 'loaded'; byClassId: ReadonlyMap<string, ClassFactsDto> }
  /** The command failed: every fact prints Unknown and the sheet shows `notice`. */
  | { kind: 'failed'; notice: string };

export const LOADING_CLASS_FACTS: ClassFactsState = { kind: 'loading' };

export function loadedClassFacts(response: ListClassFactsResponse): ClassFactsState {
  return { kind: 'loaded', byClassId: new Map(response.classes.map((facts) => [facts.classId, facts])) };
}

export function failedClassFacts(cause: unknown): ClassFactsState {
  const message = cause instanceof Error ? cause.message : String(cause);
  return { kind: 'failed', notice: `class facts unavailable: ${message}` };
}

/** The visible notice for a failed command; `null` otherwise. */
export function classFactsNotice(state: ClassFactsState): string | null {
  return state.kind === 'failed' ? state.notice : null;
}

/** The queries `list_class_facts` is asked for: each held class at its own level. */
export function classFactsQueries(heldClasses: readonly HeldClass[]): Array<{ classId: string; level: number }> {
  return heldClasses.map((held) => ({ classId: held.classId, level: held.level }));
}

/** The served facts for `held`, or `null` while loading / after a failure / when not served. */
function factsFor(state: ClassFactsState, held: HeldClass): ClassFactsDto | null {
  if (state.kind !== 'loaded') {
    return null;
  }
  const facts = state.byClassId.get(held.classId);
  return facts && facts.level === held.level ? facts : null;
}

function unanswered(state: ClassFactsState): string {
  return state.kind === 'loading' ? 'loading' : state.kind === 'failed' ? state.notice : 'not served';
}

/** A proficiency or class-skill verdict: decided either way, or not decidable from the answers. */
export type Verdict = 'yes' | 'no' | 'unknown';

export const WEAPON_TIERS = ['Simple', 'Martial', 'Exotic'] as const;
export type WeaponTier = (typeof WEAPON_TIERS)[number];

export interface WeaponProficiencySummary {
  tiers: ReadonlyArray<{ label: WeaponTier; proficient: Verdict }>;
  /** Named weapons, weapon groups and weapon-set members the classes grant beyond their tiers. */
  alsoProficientWith: string[];
  /** Grants printed with their condition, never counted. */
  printed: string[];
  /** `"<class label>: <reason>"` for every held class the engine could not answer. */
  unknown: string[];
}

/**
 * PF1 weapon proficiency is the union across held classes: a tier is `yes` when any class grants
 * it, `unknown` when none does but some class is unanswered, else `no`.
 */
export function summarizeWeaponProficiency(heldClasses: readonly HeldClass[], state: ClassFactsState): WeaponProficiencySummary {
  const granted = new Set<string>();
  const also = new Set<string>();
  const printed: string[] = [];
  const unknown: string[] = [];
  for (const held of heldClasses) {
    const facts = factsFor(state, held)?.weaponProficiency;
    if (!facts || facts.status !== 'known') {
      unknown.push(`${held.classLabel}: ${facts?.reason ?? unanswered(state)}`);
      continue;
    }
    facts.tiers.forEach((tier) => granted.add(tier));
    facts.named.forEach((name) => also.add(name));
    facts.groups.forEach((group) => also.add(`${group} weapon group`));
    // A set's label is the selector's name (display only); the rule grants its members.
    facts.sets.forEach((set) => set.members.forEach((member) => also.add(member)));
    facts.printed.forEach((line) => printed.push(`${held.classLabel}: ${line}`));
  }
  const tiers = WEAPON_TIERS.map((label) => ({
    label,
    proficient: (granted.has(label) ? 'yes' : unknown.length > 0 ? 'unknown' : 'no') as Verdict,
  }));
  return { tiers, alsoProficientWith: [...also].sort(), printed, unknown };
}

export interface CasterLevelSummary {
  /** What the Caster Level box prints. */
  display: string;
  casters: Array<{ classLabel: string; casterLevel: number }>;
  unknown: string[];
}

/**
 * PF1 caster level is per casting class (a Wizard 3 / Cleric 2 casts wizard spells at CL 3 and
 * cleric spells at CL 2). One caster prints its number; several print `Wizard 3 / Cleric 2`; no
 * casting class prints `—`; any unanswered class prints Unknown.
 */
export function summarizeCasterLevel(heldClasses: readonly HeldClass[], state: ClassFactsState): CasterLevelSummary {
  const casters: CasterLevelSummary['casters'] = [];
  const unknown: string[] = [];
  for (const held of heldClasses) {
    const facts = factsFor(state, held)?.casterLevel;
    if (!facts || facts.status === 'unknown') {
      unknown.push(`${held.classLabel}: ${facts?.source ?? unanswered(state)}`);
    } else if (facts.status === 'caster' && facts.value !== null) {
      casters.push({ classLabel: held.classLabel, casterLevel: facts.value });
    }
  }
  let display: string;
  if (state.kind === 'loading') {
    display = '…';
  } else if (unknown.length > 0) {
    display = 'Unknown';
  } else if (casters.length === 0) {
    display = '—';
  } else if (casters.length === 1) {
    display = String(casters[0].casterLevel);
  } else {
    display = casters.map((entry) => `${entry.classLabel} ${entry.casterLevel}`).join(' / ');
  }
  return { display, casters, unknown };
}
