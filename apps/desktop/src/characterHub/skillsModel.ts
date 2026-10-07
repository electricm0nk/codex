import type { AbilityScoresDto } from '../boundary/loadCreateCharacter';
import type { ClassSkillFactsDto } from '../boundary/listClassFacts';
import type { FeatSkillBonusesDto } from '../boundary/loadSavedCharacterDetail';
import { buildLevelEntries, totalSkillPoints, type HeldClass } from './characterProgression';
import type { ClassFactsState } from './classFactsModel';

/** The full PF1 core rulebook skill list with governing ability. */
export const SKILLS: ReadonlyArray<{ name: string; ability: keyof AbilityScoresDto }> = [
  { name: 'Acrobatics', ability: 'dexterity' },
  { name: 'Appraise', ability: 'intelligence' },
  { name: 'Bluff', ability: 'charisma' },
  { name: 'Climb', ability: 'strength' },
  { name: 'Craft', ability: 'intelligence' },
  { name: 'Diplomacy', ability: 'charisma' },
  { name: 'Disable Device', ability: 'dexterity' },
  { name: 'Disguise', ability: 'charisma' },
  { name: 'Escape Artist', ability: 'dexterity' },
  { name: 'Fly', ability: 'dexterity' },
  { name: 'Handle Animal', ability: 'charisma' },
  { name: 'Heal', ability: 'wisdom' },
  { name: 'Intimidate', ability: 'charisma' },
  { name: 'Knowledge (Arcana)', ability: 'intelligence' },
  { name: 'Knowledge (Dungeoneering)', ability: 'intelligence' },
  { name: 'Knowledge (Engineering)', ability: 'intelligence' },
  { name: 'Knowledge (Geography)', ability: 'intelligence' },
  { name: 'Knowledge (History)', ability: 'intelligence' },
  { name: 'Knowledge (Local)', ability: 'intelligence' },
  { name: 'Knowledge (Nature)', ability: 'intelligence' },
  { name: 'Knowledge (Nobility)', ability: 'intelligence' },
  { name: 'Knowledge (Planes)', ability: 'intelligence' },
  { name: 'Knowledge (Religion)', ability: 'intelligence' },
  { name: 'Linguistics', ability: 'intelligence' },
  { name: 'Perception', ability: 'wisdom' },
  { name: 'Perform', ability: 'charisma' },
  { name: 'Profession', ability: 'wisdom' },
  { name: 'Ride', ability: 'dexterity' },
  { name: 'Sense Motive', ability: 'wisdom' },
  { name: 'Sleight of Hand', ability: 'dexterity' },
  { name: 'Spellcraft', ability: 'intelligence' },
  { name: 'Stealth', ability: 'dexterity' },
  { name: 'Survival', ability: 'wisdom' },
  { name: 'Swim', ability: 'strength' },
  { name: 'Use Magic Device', ability: 'charisma' },
];

/**
 * Maps a `SKILLS` display name to the `skill:<snake_case>` wire id the
 * `set_skill_allocations` Tauri command expects (`SkillAllocation.skill_id`
 * in `character_input.rs`). Only 5 ids are actually recognized by the
 * compute engine today (`skill:climb`, `skill:swim`, `skill:intimidate`,
 * `skill:diplomacy`, `skill:disable_device` — see
 * `src/rules_core/skill_allocation.rs`'s `skill_key_ability_modifier`), and
 * those 5 confirm this exact convention (lowercase, spaces/parens to
 * underscores). The other 30 ids are this same convention extended by
 * inference, not confirmed against any canonical backend list — backend
 * flagged the same uncertainty from their side when they shipped the
 * command. Unrecognized ids are inert on the backend (no modifier
 * fabricated, no rejection), so sending them is safe either way.
 */
export function skillIdFor(skillName: string): string {
  const normalized = skillName
    .toLowerCase()
    .replace(/[()]/g, '')
    .trim()
    .replace(/[^a-z0-9]+/g, '_')
    .replace(/^_+|_+$/g, '');
  return `skill:${normalized}`;
}

/**
 * Whether the served class-skill answer grants `skillName` (a `SKILLS` display name): named
 * directly by its package id (`skillIdFor` without the `skill:` prefix), or a member of a granted
 * family (`Knowledge (Nature)` under `Knowledge`, `Craft` itself under `Craft`) -- the engine's own
 * `ClassSkillView::contains` rule.
 */
export function classSkillFactsGrant(facts: ClassSkillFactsDto, skillName: string): boolean {
  if (facts.status !== 'known') {
    return false;
  }
  const id = skillIdFor(skillName).slice('skill:'.length);
  return (
    facts.skills.includes(id) ||
    facts.groups.some((group) => {
      const family = group.toLowerCase();
      return id === family || id.startsWith(`${family}_`);
    })
  );
}

/**
 * SD-36 F6a: which skills are class skills for the classes a character holds, read from the
 * engine's class-skill reader (`list_class_facts`). PF1's union rule: a skill is a class skill when
 * ANY held class grants it. A held class the engine cannot answer contributes nothing and is named
 * in `unanswered` (the Skills panel prints it) -- never silently scored all-cross-class.
 */
/**
 * SD-36 F7c: the marker printed beside a class skill held only through a Path-A canonical seed --
 * the engine's `class_seeds::DEFAULT_PICK_MARKER`, the same words the Weapons tab's seeded-pick
 * lines end with.
 */
export const DEFAULT_PICK_MARKER = 'default pick';

/**
 * Whether the served answer grants `skillName` ONLY as a canonical default pick: granted, and
 * every grant of it is a `defaultPicks` / `defaultPickGroups` entry.
 */
export function classSkillFactsDefaultPick(facts: ClassSkillFactsDto, skillName: string): boolean {
  if (!classSkillFactsGrant(facts, skillName)) {
    return false;
  }
  const fixed: ClassSkillFactsDto = {
    ...facts,
    skills: facts.skills.filter((skill) => !(facts.defaultPicks ?? []).includes(skill)),
    groups: facts.groups.filter((group) => !(facts.defaultPickGroups ?? []).includes(group)),
  };
  return !classSkillFactsGrant(fixed, skillName);
}

export interface ClassSkillLookup {
  isClassSkill: (skillName: string) => boolean;
  /**
   * SD-36 F7c: a class skill that holds only because the engine applied a Path-A canonical seed in
   * every held class that grants it (no held class grants it outright) -- printed with
   * `(default pick)`.
   */
  isDefaultPick: (skillName: string) => boolean;
  /** `{ classLabel, reason }` for each held class with no served class-skill answer. */
  unanswered: Array<{ classLabel: string; reason: string }>;
}

export function classSkillLookup(heldClasses: readonly HeldClass[], state: ClassFactsState): ClassSkillLookup {
  const known: ClassSkillFactsDto[] = [];
  const unanswered: ClassSkillLookup['unanswered'] = [];
  for (const held of heldClasses) {
    const served = state.kind === 'loaded' ? state.byClassId.get(held.classId) : undefined;
    const facts = served && served.level === held.level ? served.classSkills : undefined;
    if (facts && facts.status === 'known') {
      known.push(facts);
    } else {
      const reason =
        facts?.reason ?? (state.kind === 'loading' ? 'loading' : state.kind === 'failed' ? state.notice : 'not served');
      unanswered.push({ classLabel: held.classLabel, reason });
    }
  }
  return {
    isClassSkill: (skillName) => known.some((facts) => classSkillFactsGrant(facts, skillName)),
    isDefaultPick: (skillName) =>
      known.some((facts) => classSkillFactsGrant(facts, skillName)) &&
      known.every((facts) => !classSkillFactsGrant(facts, skillName) || classSkillFactsDefaultPick(facts, skillName)),
    unanswered,
  };
}

/** Whether `skillName` is a class skill under `lookup` (see {@link classSkillLookup}). */
export function isClassSkill(lookup: ClassSkillLookup, skillName: string): boolean {
  return lookup.isClassSkill(skillName);
}

/**
 * PF1: a skill's total modifier is ability mod + ranks + (a +3 class-skill bonus once at least 1
 * rank is invested) + the feat bonus the engine folded for it ({@link featSkillBonusFor}).
 */
export function skillModifier(abilityModifier: number, ranks: number, classSkill: boolean, featBonus = 0): number {
  return abilityModifier + ranks + (classSkill && ranks > 0 ? 3 : 0) + featBonus;
}

/**
 * SD-36 F6b: the feat bonus the engine folded for `skillName` (a `SKILLS` display name): the
 * served per-skill total for its package id (`skillIdFor` without `skill:`), plus every served
 * family total the skill belongs to (`knowledge` for `Knowledge (Local)`) -- the same membership
 * rule as {@link classSkillFactsGrant}. Nothing is computed here; the engine folded by bonus type.
 */
export function featSkillBonusFor(bonuses: FeatSkillBonusesDto, skillName: string): number {
  const id = skillIdFor(skillName).slice('skill:'.length);
  let total = bonuses.skills[id] ?? 0;
  for (const [family, value] of Object.entries(bonuses.groups)) {
    if (id === family || id.startsWith(`${family}_`)) {
      total += value;
    }
  }
  return total;
}

/**
 * Folded feat bonuses whose skill has no row on the panel (`craft_alchemy`, `perform_oratory`,
 * `knowledge_psionics`, ...): printed as a note under the panel so no served bonus is dropped.
 */
export function featSkillBonusesWithoutARow(bonuses: FeatSkillBonusesDto): Array<{ skill: string; value: number; labels: string[] }> {
  const rowIds = new Set(SKILLS.map((skill) => skillIdFor(skill.name).slice('skill:'.length)));
  return Object.entries(bonuses.skills)
    .filter(([skill]) => !rowIds.has(skill))
    .map(([skill, value]) => ({
      skill,
      value,
      labels: bonuses.contributions.filter((c) => !c.group && c.skill === skill).map((c) => c.label),
    }));
}

/** Max ranks investable in a class skill at the given total character level. */
export function maxClassSkillRanks(characterLevel: number): number {
  return characterLevel + 3;
}

/** Max ranks investable in a cross-class skill — half the class-skill max, per PF1 core rules. */
export function maxCrossClassSkillRanks(characterLevel: number): number {
  return Math.floor((characterLevel + 3) / 2);
}

/**
 * Skill points spent: PF1 (CRB Chapter 4, Acquiring Skills) buys one rank with one point, class
 * skill or not -- a class skill adds +3 instead of costing less (3.5's two-point cross-class rank
 * is not a PF1 rule). Every allocation entry counts, including an id with no panel row.
 */
export function skillPointsSpent(allocation: Record<string, number>): number {
  return Object.values(allocation).reduce((sum, ranks) => sum + ranks, 0);
}

/**
 * The persisted `chosen.skill_allocations` (`LoadSavedCharacterResponse.skillAllocations`) keyed
 * by panel row name (`skill:knowledge_arcana` -> `Knowledge (Arcana)`). An id with no panel row
 * keeps its wire id as the key, so it is still counted and written back unchanged.
 */
export function allocationFromPersisted(entries: ReadonlyArray<{ skillId: string; ranks: number }>): Record<string, number> {
  const nameById = new Map(SKILLS.map((skill) => [skillIdFor(skill.name), skill.name]));
  const allocation: Record<string, number> = {};
  for (const entry of entries) {
    const key = nameById.get(entry.skillId) ?? entry.skillId;
    allocation[key] = (allocation[key] ?? 0) + entry.ranks;
  }
  return allocation;
}

/** The inverse of {@link allocationFromPersisted}: the wire list `set_skill_allocations` takes. */
export function persistedFromAllocation(allocation: Record<string, number>): Array<{ skillId: string; ranks: number }> {
  return Object.entries(allocation)
    .filter(([, ranks]) => ranks > 0)
    .map(([key, ranks]) => ({ skillId: key.startsWith('skill:') ? key : skillIdFor(key), ranks }));
}

/**
 * Total skill points earned across every class level already taken; `null` when any level's
 * class states no skill ranks (the roster is loading, or the record states none).
 */
export function totalSkillPointsAvailable(heldClasses: HeldClass[], intelligenceModifier: number, isHuman: boolean): number | null {
  let total = 0;
  for (const entry of buildLevelEntries(heldClasses)) {
    const points = totalSkillPoints(entry.skillPointsBase, intelligenceModifier, isHuman);
    if (points === null) {
      return null;
    }
    total += points;
  }
  return total;
}

/**
 * What the skills dialogs print at the top: how many points are left of how many. `total` is `null`
 * when a held class states no skill ranks per level, which is Unknown, never a guessed number.
 */
export function skillPointsStatus(total: number | null, remaining: number): { text: string; tone: 'ok' | 'over' | 'unknown' } {
  if (total === null) {
    return { text: 'Skill points Unknown: a held class states no skill ranks per level', tone: 'unknown' };
  }
  return { text: `${remaining} of ${total} points remaining`, tone: remaining >= 0 ? 'ok' : 'over' };
}
