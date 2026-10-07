/**
 * Custom: the GM's grants and house-rule records for a character (`custom.json`).
 *
 * Grants change numbers: an ability grant is baked into the saved ability score by the backend (so
 * every derived number follows from the engine), while `hit_points` and `skill_points` grants are
 * added by the sheet to the hit point total and the skill point pool. Records (custom feats,
 * equipment, spells and magic devices) are a name, a description and free-form stat lines; the
 * engine does not compute from them, they are listed and printed. The backend validates all of it;
 * `customProblems` is the same check run before sending so the dialog can say what to fix.
 */

export interface CustomGrant {
  id: string;
  label: string;
  /** `ability:<name>`, `hit_points` or `skill_points`. */
  target: string;
  value: number;
  reason: string;
}

export interface CustomStat {
  label: string;
  value: string;
}

export interface CustomRecord {
  id: string;
  name: string;
  description: string;
  stats: CustomStat[];
}

export type CustomRecordKind = 'feats' | 'equipment' | 'spells' | 'devices';

export interface CharacterCustom {
  grants: CustomGrant[];
  feats: CustomRecord[];
  equipment: CustomRecord[];
  spells: CustomRecord[];
  devices: CustomRecord[];
}

export const EMPTY_CUSTOM: CharacterCustom = { grants: [], feats: [], equipment: [], spells: [], devices: [] };

export const CUSTOM_RECORD_KINDS: ReadonlyArray<{ kind: CustomRecordKind; singular: string; plural: string }> = [
  { kind: 'feats', singular: 'feat', plural: 'Feats' },
  { kind: 'equipment', singular: 'equipment', plural: 'Equipment' },
  { kind: 'spells', singular: 'spell', plural: 'Spells' },
  { kind: 'devices', singular: 'magic device', plural: 'Magic devices' },
];

export const ABILITY_NAMES = ['strength', 'dexterity', 'constitution', 'intelligence', 'wisdom', 'charisma'] as const;
export type AbilityName = (typeof ABILITY_NAMES)[number];

const MAX_GRANT_VALUE = 12;
const MAX_NAME = 120;
const MAX_DESCRIPTION = 10_000;

const titleCase = (word: string): string => word.charAt(0).toUpperCase() + word.slice(1);

export const GRANT_TARGETS: ReadonlyArray<{ value: string; label: string }> = [
  ...ABILITY_NAMES.map((name) => ({ value: `ability:${name}`, label: `${titleCase(name)} score` })),
  { value: 'hit_points', label: 'Hit points' },
  { value: 'skill_points', label: 'Skill points' },
];

export function grantTargetLabel(target: string): string {
  return GRANT_TARGETS.find((option) => option.value === target)?.label ?? target;
}

export function newGrant(id: string): CustomGrant {
  return { id, label: '', target: 'ability:wisdom', value: 1, reason: '' };
}

export function newRecord(id: string): CustomRecord {
  return { id, name: '', description: '', stats: [] };
}

export interface GrantTotals {
  hitPoints: number;
  skillPoints: number;
  abilities: Record<AbilityName, number>;
}

export function grantTotals(custom: CharacterCustom): GrantTotals {
  const totals: GrantTotals = {
    hitPoints: 0,
    skillPoints: 0,
    abilities: { strength: 0, dexterity: 0, constitution: 0, intelligence: 0, wisdom: 0, charisma: 0 },
  };
  for (const grant of custom.grants) {
    if (grant.target === 'hit_points') {
      totals.hitPoints += grant.value;
    } else if (grant.target === 'skill_points') {
      totals.skillPoints += grant.value;
    } else if (grant.target.startsWith('ability:')) {
      const name = grant.target.slice('ability:'.length) as AbilityName;
      if (name in totals.abilities) {
        totals.abilities[name] += grant.value;
      }
    }
  }
  return totals;
}

/** The sheet's maximum hit points with the GM's hit point grants; an unknown total stays unknown. */
export function withCustomHitPoints(total: number | null, custom: CharacterCustom): number | null {
  return total === null ? null : total + grantTotals(custom).hitPoints;
}

/** The skill point pool with the GM's grants (never below zero); an unknown pool stays unknown. */
export function withCustomSkillPoints(total: number | null, custom: CharacterCustom): number | null {
  return total === null ? null : Math.max(0, total + grantTotals(custom).skillPoints);
}

export function customIsEmpty(custom: CharacterCustom): boolean {
  return (
    custom.grants.length === 0 &&
    custom.feats.length === 0 &&
    custom.equipment.length === 0 &&
    custom.spells.length === 0 &&
    custom.devices.length === 0
  );
}

/** What to fix before saving, one sentence each; empty when the backend will accept it. */
export function customProblems(custom: CharacterCustom): string[] {
  const problems: string[] = [];
  for (const grant of custom.grants) {
    if (grant.label.trim() === '') {
      problems.push('Every grant needs a label (what it is for).');
    }
    if (!Number.isInteger(grant.value) || Math.abs(grant.value) > MAX_GRANT_VALUE) {
      problems.push(`The grant "${grant.label || 'unnamed'}" must be a whole number from -${MAX_GRANT_VALUE} to ${MAX_GRANT_VALUE}.`);
    }
  }
  for (const { kind, singular } of CUSTOM_RECORD_KINDS) {
    for (const record of custom[kind]) {
      if (record.name.trim() === '') {
        problems.push(`Every custom ${singular} needs a name.`);
      } else if (record.name.length > MAX_NAME) {
        problems.push(`"${record.name.slice(0, 30)}…" has a name over ${MAX_NAME} characters.`);
      }
      if (record.description.length > MAX_DESCRIPTION) {
        problems.push(`The description of "${record.name}" is over ${MAX_DESCRIPTION} characters.`);
      }
      if (record.stats.some((stat) => stat.label.trim() === '' || stat.value.trim() === '')) {
        problems.push(`"${record.name || 'A custom ' + singular}" has a stat line with no label or no value.`);
      }
    }
  }
  return problems;
}

/** One short line per entry, for the Manage box. */
export function customSummaryLines(custom: CharacterCustom): string[] {
  const lines = custom.grants.map((grant) => `${grant.label}: ${grant.value >= 0 ? '+' : ''}${grant.value} ${grantTargetLabel(grant.target)}`);
  for (const { kind, singular } of CUSTOM_RECORD_KINDS) {
    for (const record of custom[kind]) {
      lines.push(`${titleCase(singular)}: ${record.name}`);
    }
  }
  return lines;
}
