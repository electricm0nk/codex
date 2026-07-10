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
 * `partial-human-only` — the compute engine's named class seam only fires
 * for `race:human`; every other race falls back to the same 4 generic
 * diagnostics as a `none` class.
 * `none` — no dedicated compute seam exists; every race produces the same 4
 * generic diagnostics.
 */
export type ClassSupportLevel = 'full' | 'partial-human-only' | 'none';

export interface ClassOption {
  id: string;
  label: string;
  supportLevel: ClassSupportLevel;
  levelOptions: number[];
  /** PF1 hit die size (e.g. 10 for a d10). Level-1 HP is the max of this plus the CON modifier. */
  hitDie: number;
}

/** The full PF1 core rulebook class roster. */
export const CLASS_OPTIONS: ClassOption[] = [
  { id: 'class:fighter', label: 'Fighter', supportLevel: 'full', levelOptions: [1, 2, 3], hitDie: 10 },
  { id: 'class:paladin', label: 'Paladin', supportLevel: 'partial-human-only', levelOptions: [1], hitDie: 10 },
  { id: 'class:ranger', label: 'Ranger', supportLevel: 'partial-human-only', levelOptions: [1], hitDie: 10 },
  { id: 'class:sorcerer', label: 'Sorcerer', supportLevel: 'partial-human-only', levelOptions: [1], hitDie: 6 },
  { id: 'class:wizard', label: 'Wizard', supportLevel: 'partial-human-only', levelOptions: [1], hitDie: 6 },
  { id: 'class:bard', label: 'Bard', supportLevel: 'partial-human-only', levelOptions: [1], hitDie: 8 },
  { id: 'class:barbarian', label: 'Barbarian', supportLevel: 'partial-human-only', levelOptions: [1], hitDie: 12 },
  { id: 'class:rogue', label: 'Rogue', supportLevel: 'partial-human-only', levelOptions: [1], hitDie: 8 },
  { id: 'class:cleric', label: 'Cleric', supportLevel: 'partial-human-only', levelOptions: [1], hitDie: 8 },
  { id: 'class:druid', label: 'Druid', supportLevel: 'partial-human-only', levelOptions: [1], hitDie: 8 },
  { id: 'class:monk', label: 'Monk', supportLevel: 'partial-human-only', levelOptions: [1], hitDie: 8 },
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
    case 'none':
      return `${classLabel} isn't computed by the engine yet.`;
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
