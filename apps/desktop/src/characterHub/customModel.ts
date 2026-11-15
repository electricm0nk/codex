/**
 * Custom: the GM's grants and house-rule records for a character (`custom.json`).
 *
 * Grants change numbers: an ability grant is baked into the saved ability score by the backend (so
 * every derived number follows from the engine), while `hit_points` and `skill_points` grants are
 * added by the sheet to the hit point total and the skill point pool. Records (custom feats,
 * equipment, spells and magic devices) are a name, a description and free-form stat lines, listed
 * and printed. Custom equipment may also state typed bonuses and a body slot; worn, the backend
 * applies them, so no total is added up here (`character_custom.rs`). The backend validates all of
 * it; `customProblems` is the same check run before sending so the dialog can say what to fix.
 */

import type { BodySlotDto } from '../boundary/listHeldEquipment';
import type { ExplanationDto } from '../boundary/loadSavedCharacterDetail';
import { SKILLS, skillIdFor } from './skillsModel';
import { UNKNOWN_RACE_TRAIT } from './characterHubModel';

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

/** One number a custom item states, applied while it is worn. */
export interface CustomBonus {
  target: string;
  bonusType: string;
  value: number;
}

export interface CustomRecord {
  id: string;
  name: string;
  description: string;
  stats: CustomStat[];
  /** Custom equipment only, as are `slot` and `worn`; the backend leaves all three off a text record. */
  bonuses?: CustomBonus[];
  slot?: BodySlotDto | null;
  worn?: boolean;
  /** Owned and not on the character; never with `worn`. */
  leftBehind?: boolean;
}

export type CustomRecordKind = 'feats' | 'equipment' | 'spells' | 'devices';

export interface CharacterCustom {
  grants: CustomGrant[];
  feats: CustomRecord[];
  equipment: CustomRecord[];
  spells: CustomRecord[];
  devices: CustomRecord[];
  /** Conditions the character is under (`'Shaken'`); empty means the sheet shows `Normal`. */
  conditions: string[];
  /** What the GM lets this character do without qualifying; the backend sends all eight. */
  overrides?: CustomOverrides;
}

/** The backend's `Overrides`: each flag is read by the command enforcing that rule. */
export interface CustomOverrides {
  featsWithoutPrerequisites: boolean;
  weaponsWithoutProficiency: boolean;
  spellsOutsideClassList: boolean;
  armorWithoutProficiency: boolean;
  ignoreSlotLimits: boolean;
  ignoreFeatBudget: boolean;
  ignoreAlignmentRestrictions: boolean;
  ignoreEncumbrance: boolean;
}

export const NO_OVERRIDES: CustomOverrides = {
  featsWithoutPrerequisites: false,
  weaponsWithoutProficiency: false,
  spellsOutsideClassList: false,
  armorWithoutProficiency: false,
  ignoreSlotLimits: false,
  ignoreFeatBudget: false,
  ignoreAlignmentRestrictions: false,
  ignoreEncumbrance: false,
};

/** The Custom tab's Overrides group, one checkbox each. */
export const OVERRIDE_TOGGLES: ReadonlyArray<{ key: keyof CustomOverrides; label: string }> = [
  { key: 'featsWithoutPrerequisites', label: 'Feats without prerequisites' },
  { key: 'weaponsWithoutProficiency', label: 'Weapons without proficiency' },
  { key: 'spellsOutsideClassList', label: 'Spells outside the class list or level' },
  { key: 'armorWithoutProficiency', label: 'Armor and shields without proficiency' },
  { key: 'ignoreSlotLimits', label: 'Ignore body slot limits' },
  { key: 'ignoreFeatBudget', label: 'Ignore the feat budget' },
  { key: 'ignoreAlignmentRestrictions', label: 'Ignore alignment restrictions' },
  { key: 'ignoreEncumbrance', label: 'Ignore encumbrance' },
];

/** The character's flags, all off for data saved before they existed. */
export function customOverrides(custom: CharacterCustom): CustomOverrides {
  return { ...NO_OVERRIDES, ...custom.overrides };
}

export function withOverride(custom: CharacterCustom, key: keyof CustomOverrides, on: boolean): CharacterCustom {
  return { ...custom, overrides: { ...customOverrides(custom), [key]: on } };
}

export const EMPTY_CUSTOM: CharacterCustom = { grants: [], feats: [], equipment: [], spells: [], devices: [], conditions: [] };

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

/** The backend's `bonus_target_is_known`, as the form offers it. */
export const BONUS_TARGETS: ReadonlyArray<{ value: string; label: string }> = [
  { value: 'ac', label: 'AC' },
  { value: 'attack', label: 'Attack' },
  { value: 'damage', label: 'Damage' },
  { value: 'save:fortitude', label: 'Fortitude save' },
  { value: 'save:reflex', label: 'Reflex save' },
  { value: 'save:will', label: 'Will save' },
  { value: 'initiative', label: 'Initiative' },
  { value: 'speed', label: 'Speed' },
  { value: 'hit_points', label: 'Hit points' },
  { value: 'cmb', label: 'CMB' },
  { value: 'cmd', label: 'CMD' },
  ...ABILITY_NAMES.map((name) => ({ value: `ability:${name}`, label: `${titleCase(name)} score` })),
  ...SKILLS.map((skill) => ({ value: skillIdFor(skill.name), label: skill.name })),
];

/** The backend's `BONUS_TYPES`, the sheet engine's words for a bonus's type. */
export const BONUS_TYPES: readonly string[] = [
  'Untyped', 'Alchemical', 'Armor', 'Circumstance', 'Competence', 'Deflection', 'Dodge', 'Enhancement', 'Insight', 'Luck',
  'Morale', 'NaturalArmor', 'Profane', 'Racial', 'Resistance', 'Sacred', 'Shield', 'Size',
];

const MAX_BONUS_VALUE = 100;

export function newBonus(): CustomBonus {
  return { target: 'ac', bonusType: 'Untyped', value: 1 };
}

/** `NaturalArmor` as a player says it. */
export function bonusTypeLabel(bonusType: string): string {
  return bonusType === 'NaturalArmor' ? 'natural armor' : bonusType.toLowerCase();
}

/** `+1 deflection to AC`; an untyped bonus says no type. */
export function bonusLine(bonus: CustomBonus): string {
  const type = bonus.bonusType === 'Untyped' ? '' : `${bonusTypeLabel(bonus.bonusType)} `;
  const target = BONUS_TARGETS.find((option) => option.value === bonus.target)?.label ?? bonus.target;
  return `${bonus.value >= 0 ? '+' : ''}${bonus.value} ${type}to ${target}`;
}

/** Where a custom item is, or `null` for a text-only record, which is on no one's body. */
export function customItemPlace(record: CustomRecord): 'worn' | 'carried' | 'left_behind' | null {
  if (!record.slot && (record.bonuses ?? []).length === 0) {
    return null;
  }
  return record.worn ? 'worn' : record.leftBehind ? 'left_behind' : 'carried';
}

const engineRowValue = (explanations: readonly ExplanationDto[], id: string): number | null =>
  explanations.find((row) => row.id === id)?.value ?? null;

/**
 * The land speed the Speed tile shows: the engine's `speed.land` row (the race's base speed with what
 * worn items add, computed in Rust, the value the printed sheet carries too), or `Unknown` when the
 * engine states none (a race with no creation chassis; the browser preview).
 */
export function sheetLandSpeed(explanations: readonly ExplanationDto[]): string {
  const feet = engineRowValue(explanations, 'speed.land');
  return feet === null ? UNKNOWN_RACE_TRAIT : `${feet} ft.`;
}

/** The screen's no-value mark for a movement mode the engine states no row for (paper keeps an empty box). */
const NO_SPEED = '—';

/** The Swim tile (SD-40 E2.24): the engine's `speed.swim` row, the value the printed Speed box carries. */
export function sheetSwimSpeed(explanations: readonly ExplanationDto[]): string {
  const feet = engineRowValue(explanations, 'speed.swim');
  return feet === null ? NO_SPEED : `${feet} ft.`;
}

/**
 * The Fly tile (SD-40 E2.24): the engine's `speed.fly` row followed by the load response's
 * `flyManeuverability` words, as the printed Speed box writes it (`60 ft. (average)`).
 */
export function sheetFlySpeed(explanations: readonly ExplanationDto[], maneuverability: string | null | undefined): string {
  const feet = engineRowValue(explanations, 'speed.fly');
  return feet === null ? NO_SPEED : maneuverability ? `${feet} ft. (${maneuverability})` : `${feet} ft.`;
}

/**
 * The Intelligence modifier the skill point budget and languages read: the saved score's, the engine's
 * `ability_modifier.intelligence.saved` row, when a worn item raises the sheet's.
 */
export function savedIntelligenceModifier(sheetModifier: number, explanations: readonly ExplanationDto[]): number {
  return explanations.find((row) => row.id === 'ability_modifier.intelligence.saved')?.value ?? sheetModifier;
}

/**
 * The maximum hit points the Hit Points tile shows: the engine's `hit_points.max` row (the class
 * levels, saved rolls, Constitution, the GM's grants and worn items, computed in Rust, the value the
 * printed sheet carries too), or `null` (Unknown) when the engine states none.
 */
export function sheetHitPoints(explanations: readonly ExplanationDto[]): number | null {
  return engineRowValue(explanations, 'hit_points.max');
}

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
      for (const bonus of record.bonuses ?? []) {
        if (!Number.isInteger(bonus.value) || bonus.value === 0 || Math.abs(bonus.value) > MAX_BONUS_VALUE) {
          problems.push(`A bonus on "${record.name}" must be a whole number from -${MAX_BONUS_VALUE} to ${MAX_BONUS_VALUE}, and not 0.`);
        }
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
