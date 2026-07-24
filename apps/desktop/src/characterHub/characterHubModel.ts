/**
 * Data-driven catalogues for the Character Hub picker.
 *
 * These are structured as arrays/lookups (not hardcoded UI branches) so
 * later phases can widen them (more races, more game systems) without a
 * component redesign.
 */

export const ABILITY_KEYS = ['strength', 'dexterity', 'constitution', 'intelligence', 'wisdom', 'charisma'] as const;
export type AbilityKey = (typeof ABILITY_KEYS)[number];

export const ABILITY_ABBREVIATIONS: Record<AbilityKey, string> = {
  strength: 'STR',
  dexterity: 'DEX',
  constitution: 'CON',
  intelligence: 'INT',
  wisdom: 'WIS',
  charisma: 'CHA',
};

export type Sex = 'male' | 'female';

/**
 * Random height/weight profile for one sex of a race (approximate PF1 Core
 * values). Height is `baseHeightInches` plus a roll of `heightModDice`; weight
 * is `baseWeightLb` plus a roll of the same dice times `weightMultiplierLb`.
 */
export interface BodyProfile {
  baseHeightInches: number;
  heightModDice: { count: number; sides: number };
  baseWeightLb: number;
  weightMultiplierLb: number;
}

export interface RaceOption {
  id: string;
  label: string;
  /** Fixed PF1 racial ability modifiers applied to every member of the race. */
  abilityAdjustments: Partial<Record<AbilityKey, number>>;
  /**
   * Floating ability points the player distributes freely (PF1's "+2 to one
   * ability" races: Human, Half-Elf, Half-Orc). `0` means the race has no
   * player-selectable enhancement.
   */
  floatingBonusPoints: number;
  size: 'Small' | 'Medium';
  vision: string;
  body: Record<Sex, BodyProfile>;
}

/** The full PF1 core rulebook race roster, with core ability modifiers and physical profiles. */
export const RACE_OPTIONS: RaceOption[] = [
  {
    id: 'race:human',
    label: 'Human',
    abilityAdjustments: {},
    floatingBonusPoints: 2,
    size: 'Medium',
    vision: 'Normal',
    body: {
      male: { baseHeightInches: 58, heightModDice: { count: 2, sides: 10 }, baseWeightLb: 120, weightMultiplierLb: 5 },
      female: { baseHeightInches: 53, heightModDice: { count: 2, sides: 10 }, baseWeightLb: 85, weightMultiplierLb: 5 },
    },
  },
  {
    id: 'race:dwarf',
    label: 'Dwarf',
    abilityAdjustments: { constitution: 2, wisdom: 2, charisma: -2 },
    floatingBonusPoints: 0,
    size: 'Medium',
    vision: 'Darkvision 60 ft.',
    body: {
      male: { baseHeightInches: 45, heightModDice: { count: 2, sides: 4 }, baseWeightLb: 150, weightMultiplierLb: 7 },
      female: { baseHeightInches: 43, heightModDice: { count: 2, sides: 4 }, baseWeightLb: 120, weightMultiplierLb: 7 },
    },
  },
  {
    id: 'race:elf',
    label: 'Elf',
    abilityAdjustments: { dexterity: 2, intelligence: 2, constitution: -2 },
    floatingBonusPoints: 0,
    size: 'Medium',
    vision: 'Low-light vision',
    body: {
      male: { baseHeightInches: 60, heightModDice: { count: 2, sides: 6 }, baseWeightLb: 100, weightMultiplierLb: 3 },
      female: { baseHeightInches: 60, heightModDice: { count: 2, sides: 6 }, baseWeightLb: 90, weightMultiplierLb: 3 },
    },
  },
  {
    id: 'race:gnome',
    label: 'Gnome',
    abilityAdjustments: { constitution: 2, charisma: 2, strength: -2 },
    floatingBonusPoints: 0,
    size: 'Small',
    vision: 'Low-light vision',
    body: {
      male: { baseHeightInches: 36, heightModDice: { count: 2, sides: 4 }, baseWeightLb: 35, weightMultiplierLb: 1 },
      female: { baseHeightInches: 34, heightModDice: { count: 2, sides: 4 }, baseWeightLb: 30, weightMultiplierLb: 1 },
    },
  },
  {
    id: 'race:half-elf',
    label: 'Half-Elf',
    abilityAdjustments: {},
    floatingBonusPoints: 2,
    size: 'Medium',
    vision: 'Low-light vision',
    body: {
      male: { baseHeightInches: 55, heightModDice: { count: 2, sides: 8 }, baseWeightLb: 110, weightMultiplierLb: 5 },
      female: { baseHeightInches: 55, heightModDice: { count: 2, sides: 8 }, baseWeightLb: 90, weightMultiplierLb: 5 },
    },
  },
  {
    id: 'race:half-orc',
    label: 'Half-Orc',
    abilityAdjustments: {},
    floatingBonusPoints: 2,
    size: 'Medium',
    vision: 'Darkvision 60 ft.',
    body: {
      male: { baseHeightInches: 58, heightModDice: { count: 2, sides: 12 }, baseWeightLb: 150, weightMultiplierLb: 7 },
      female: { baseHeightInches: 58, heightModDice: { count: 2, sides: 12 }, baseWeightLb: 120, weightMultiplierLb: 7 },
    },
  },
  {
    id: 'race:halfling',
    label: 'Halfling',
    abilityAdjustments: { dexterity: 2, charisma: 2, strength: -2 },
    floatingBonusPoints: 0,
    size: 'Small',
    vision: 'Normal',
    body: {
      male: { baseHeightInches: 32, heightModDice: { count: 2, sides: 4 }, baseWeightLb: 30, weightMultiplierLb: 1 },
      female: { baseHeightInches: 30, heightModDice: { count: 2, sides: 4 }, baseWeightLb: 25, weightMultiplierLb: 1 },
    },
  },
];

/**
 * `full` — reaches `Computed` for any race in `RACE_OPTIONS`.
 * `partial-human-only` — reaches `Computed` for `race:human` only; every
 * other race falls back to the same 4 generic diagnostics as a `none`
 * class.
 * `human-diagnostics-only` — never reaches `Computed` for any race,
 * including Human — the compute engine's named class seam only grounds
 * explanation/diagnostic *text* for `race:human` (e.g. naming the specific
 * missing rage-execution or spellcasting burden), not an actual computed
 * build. Every other race falls back to the same 4 generic diagnostics as
 * a `none` class. Do not present this as "partial support" — Human never
 * produces a savable build here either, it only explains why in more
 * detail.
 * `none` — no dedicated compute seam exists; every race produces the same 4
 * generic diagnostics.
 */
export type ClassSupportLevel = 'full' | 'partial-human-only' | 'human-diagnostics-only' | 'none';

export interface ClassOption {
  id: string;
  label: string;
  supportLevel: ClassSupportLevel;
  levelOptions: number[];
  /** PF1 hit die size (e.g. 10 for a d10). Level-1 HP is the max of this plus the CON modifier. */
  hitDie: number;
}

/**
 * The full PF1 core rulebook class roster.
 *
 * `supportLevel` reflects the compute engine's real gating, not just
 * whether it recognizes the class — verified directly against
 * `pilot_compute.rs`'s per-class `explain_*` functions (each of Paladin,
 * Ranger, Sorcerer, Bard, Barbarian, Monk, Cleric, and Druid carries its own
 * "This deliberately does not compute a supported ... chassis/surface"
 * doc comment, and the compute path stays claim-blocked for Human exactly
 * as it does for every other race) and live-verified for Barbarian
 * specifically (a fresh Human Barbarian creation attempt still returns
 * `Blocked`, just with named rage-burden diagnostics instead of the 4
 * generic ones a non-Human race or a truly unrecognized class gets).
 *
 * Wizard and Rogue are `full`, not `partial-human-only` as this file
 * previously (incorrectly) had them: `supported_wizard_level` /
 * `supported_rogue_level` and every gate downstream of them
 * (`wizard_has_canonical_specialization_selections`,
 * `unmet_wizard_spellbook_conditions`, `compose_character_input`'s choice
 * seeding) check only `class_id` and level, never `race_id` — confirmed via
 * `git log -p` on the seeding block back to its original commit (3484b5d4),
 * which never had a Human condition. Live-verified end-to-end via the real
 * creation UI: a fresh Elf Wizard 1 and a fresh Elf Rogue 1 both reached
 * `Computed`/`Saved` with real, distinct stat blocks, disk-confirmed
 * (`race_id=race:elf` alongside a real `class_level` entry). The "Human
 * only" framing was a stale/unverified assumption baked in when the Wizard
 * spellbook fix first landed — the doc comments on the compute-path
 * functions reference "Human Wizard"/"Human Rogue" descriptively (naming
 * the deterministic baseline that was built and tested first), not as an
 * enforced restriction.
 */
export const CLASS_OPTIONS: ClassOption[] = [
  { id: 'class:fighter', label: 'Fighter', supportLevel: 'full', levelOptions: [1, 2, 3], hitDie: 10 },
  { id: 'class:paladin', label: 'Paladin', supportLevel: 'human-diagnostics-only', levelOptions: [1], hitDie: 10 },
  { id: 'class:ranger', label: 'Ranger', supportLevel: 'human-diagnostics-only', levelOptions: [1], hitDie: 10 },
  { id: 'class:sorcerer', label: 'Sorcerer', supportLevel: 'human-diagnostics-only', levelOptions: [1], hitDie: 6 },
  { id: 'class:wizard', label: 'Wizard', supportLevel: 'full', levelOptions: [1], hitDie: 6 },
  { id: 'class:bard', label: 'Bard', supportLevel: 'human-diagnostics-only', levelOptions: [1], hitDie: 8 },
  { id: 'class:barbarian', label: 'Barbarian', supportLevel: 'human-diagnostics-only', levelOptions: [1], hitDie: 12 },
  { id: 'class:rogue', label: 'Rogue', supportLevel: 'full', levelOptions: [1], hitDie: 8 },
  { id: 'class:cleric', label: 'Cleric', supportLevel: 'human-diagnostics-only', levelOptions: [1], hitDie: 8 },
  { id: 'class:druid', label: 'Druid', supportLevel: 'human-diagnostics-only', levelOptions: [1], hitDie: 8 },
  { id: 'class:monk', label: 'Monk', supportLevel: 'human-diagnostics-only', levelOptions: [1], hitDie: 8 },
];

const DEFAULT_LEVEL_OPTIONS: number[] = [1];

export function getLevelOptionsForClass(classId: string): number[] {
  return CLASS_OPTIONS.find((option) => option.id === classId)?.levelOptions ?? DEFAULT_LEVEL_OPTIONS;
}

export function describeClassSupportLevel(supportLevel: ClassSupportLevel, classLabel: string): string {
  switch (supportLevel) {
    case 'full':
      return `${classLabel} is fully computed at every level offered here.`;
    case 'partial-human-only':
      return `${classLabel} is only computed for Human today — other races show what's still missing.`;
    case 'human-diagnostics-only':
      return `${classLabel} isn't computed by the engine for any race yet, including Human — Human just shows more specific detail about what's missing.`;
    case 'none':
      return `${classLabel} isn't computed by the engine yet.`;
  }
}

/** The dropdown-option suffix for `CreateCharacterForm`'s Class select — kept alongside `describeClassSupportLevel` so both stay honest about the same distinction. */
export function classSupportLevelSuffix(supportLevel: ClassSupportLevel): string {
  switch (supportLevel) {
    case 'full':
      return '';
    case 'partial-human-only':
      return ' (Human only, partial)';
    case 'human-diagnostics-only':
      return ' (not yet computed for any race)';
    case 'none':
      return ' (not yet computed)';
  }
}

export const DEFAULT_ABILITY_SCORES = {
  strength: 16,
  dexterity: 14,
  constitution: 14,
  intelligence: 10,
  wisdom: 12,
  charisma: 8,
};

export const GAME_SYSTEM_LABELS: Record<string, string> = {
  pf1: 'Pathfinder 1st Edition',
};

/** The nine standard PF1 alignments. */
export const ALIGNMENT_OPTIONS: readonly string[] = [
  'Lawful Good',
  'Neutral Good',
  'Chaotic Good',
  'Lawful Neutral',
  'True Neutral',
  'Chaotic Neutral',
  'Lawful Evil',
  'Neutral Evil',
  'Chaotic Evil',
];

export const PHYSICAL_ABILITIES: readonly AbilityKey[] = ['strength', 'dexterity', 'constitution'];
export const MENTAL_ABILITIES: readonly AbilityKey[] = ['intelligence', 'wisdom', 'charisma'];

export type AgeCategory = 'Adult' | 'Middle Age' | 'Old' | 'Venerable';
export const AGE_OPTIONS: readonly AgeCategory[] = ['Adult', 'Middle Age', 'Old', 'Venerable'];

/**
 * Cumulative PF1 aging ability modifiers: physical abilities (Str/Dex/Con)
 * take the penalty, mental abilities (Int/Wis/Cha) take the bonus.
 */
export const AGE_EFFECTS: Record<AgeCategory, { physical: number; mental: number }> = {
  Adult: { physical: 0, mental: 0 },
  'Middle Age': { physical: -1, mental: 1 },
  Old: { physical: -3, mental: 2 },
  Venerable: { physical: -6, mental: 3 },
};

export function ageEffectForAbility(age: AgeCategory, ability: AbilityKey): number {
  const effect = AGE_EFFECTS[age];
  if (PHYSICAL_ABILITIES.includes(ability)) {
    return effect.physical;
  }
  if (MENTAL_ABILITIES.includes(ability)) {
    return effect.mental;
  }
  return 0;
}

/** PF1 ability modifier: floor((score - 10) / 2). */
export function abilityModifier(score: number): number {
  return Math.floor((score - 10) / 2);
}

/** Level-1 HP: the class hit die maximum plus the constitution modifier (floored at 1). */
export function maxHitPointsAtLevelOne(hitDie: number, constitutionScore: number): number {
  return Math.max(1, hitDie + abilityModifier(constitutionScore));
}

/** Sum of `count` dice with `sides` faces. */
export function rollDice(count: number, sides: number): number {
  let total = 0;
  for (let index = 0; index < count; index += 1) {
    total += Math.floor(Math.random() * sides) + 1;
  }
  return total;
}

/** Inches to a `f'i"` string. */
export function formatHeight(totalInches: number): string {
  const feet = Math.floor(totalInches / 12);
  const inches = totalInches % 12;
  return `${feet}'${inches}"`;
}
