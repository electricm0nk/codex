import { CLASS_OPTIONS } from './characterHubModel';

/**
 * Pathfinder 1e level progression: per-class skill points and class features,
 * plus the universal PF1 benefits (a feat at every odd level, an ability score
 * increase every 4th level). Used by the character sheet to list what each
 * current level granted and what the next level of each held class offers.
 */

/** Base skill ranks per level for each class, before the Intelligence modifier. */
const CLASS_SKILL_POINTS: Record<string, number> = {
  'class:barbarian': 4,
  'class:bard': 6,
  'class:cleric': 2,
  'class:druid': 4,
  'class:fighter': 2,
  'class:monk': 4,
  'class:paladin': 2,
  'class:ranger': 6,
  'class:rogue': 8,
  'class:sorcerer': 2,
  'class:wizard': 2,
};

/**
 * Class features by class level. Fighter is detailed (it is the fully-computed
 * class); other classes fall back to a generic descriptor until their tables
 * are added.
 */
const CLASS_FEATURES: Record<string, Record<number, string[]>> = {
  // Full PF1 Core Fighter progression (bonus combat feat at 1st and every even level).
  'class:fighter': {
    1: ['Bonus combat feat'],
    2: ['Bravery +1', 'Bonus combat feat'],
    3: ['Armor training 1'],
    4: ['Bonus combat feat'],
    5: ['Weapon training 1'],
    6: ['Bravery +2', 'Bonus combat feat'],
    7: ['Armor training 2'],
    8: ['Bonus combat feat'],
    9: ['Weapon training 2'],
    10: ['Bravery +3', 'Bonus combat feat'],
    11: ['Armor training 3'],
    12: ['Bonus combat feat'],
    13: ['Weapon training 3'],
    14: ['Bravery +4', 'Bonus combat feat'],
    15: ['Armor training 4'],
    16: ['Bonus combat feat'],
    17: ['Weapon training 4'],
    18: ['Bravery +5', 'Bonus combat feat'],
    19: ['Armor mastery'],
    20: ['Weapon mastery', 'Bonus combat feat'],
  },
  // Wizard: arcane school & bond at 1st, Scribe Scroll bonus feat, bonus feats every 5 levels.
  'class:wizard': {
    1: ['Arcane bond', 'Arcane school', 'Cantrips', 'Scribe Scroll'],
    5: ['Bonus feat'],
    10: ['Bonus feat'],
    15: ['Bonus feat'],
    20: ['Bonus feat'],
  },
};

// PF1 Core Wizard spells per day by class level (index 0 = cantrips), before the
// Intelligence bonus spells. Enough levels for realistic testing.
const WIZARD_SPELLS_PER_DAY: Record<number, readonly number[]> = {
  1: [3, 1],
  2: [4, 2],
  3: [4, 2, 1],
  4: [4, 3, 2],
  5: [4, 3, 2, 1],
  6: [4, 3, 3, 2],
  7: [4, 4, 3, 2, 1],
  8: [4, 4, 3, 3, 2],
  9: [4, 4, 4, 3, 2, 1],
};

const SPELL_LEVEL_ORDINALS = ['1st', '2nd', '3rd', '4th', '5th', '6th', '7th', '8th', '9th'];

function spellLevelName(index: number): string {
  return index === 0 ? '0' : SPELL_LEVEL_ORDINALS[index - 1] ?? `${index}th`;
}

/** Spellcasting lines (spells/day + spells learned) gained at a caster class level. */
function casterLines(classId: string, classLevel: number): string[] {
  if (classId !== 'class:wizard') {
    return [];
  }
  const lines: string[] = [];
  const perDay = WIZARD_SPELLS_PER_DAY[classLevel];
  if (perDay) {
    lines.push(`Spells/day — ${perDay.map((count, index) => `${spellLevelName(index)}: ${count}`).join(', ')}`);
  }
  lines.push(classLevel === 1 ? 'Spellbook: all 0-level + (3 + Int mod) 1st-level spells' : 'Spellbook: learns 2 new spells');
  return lines;
}

export interface WeaponProficiency {
  simple: boolean;
  martial: boolean;
  exotic: boolean;
}

// PF1 martial-weapon classes. Exotic weapons always require a feat, so no class
// grants them by default. Restricted-list casters (wizard, druid, monk) are
// approximated as simple-proficient at the category level.
const MARTIAL_WEAPON_CLASSES = new Set(['class:fighter', 'class:barbarian', 'class:paladin', 'class:ranger']);

export function classWeaponProficiency(classId: string): WeaponProficiency {
  return { simple: true, martial: MARTIAL_WEAPON_CLASSES.has(classId), exotic: false };
}

export interface HeldClass {
  classId: string;
  classLabel: string;
  level: number;
}

export interface LevelEntry {
  /** Character level at which this class level was taken (drives feats / ability increases). */
  characterLevel: number;
  classId: string;
  classLabel: string;
  /** Which level of this class this entry represents (drives class features & spells). */
  classLevel: number;
  /** Base skill ranks for this class before the Intelligence modifier. */
  skillPointsBase: number;
  features: string[];
}

export function classSkillPointsBase(classId: string): number {
  return CLASS_SKILL_POINTS[classId] ?? 2;
}

/**
 * PF1 grants a feat at every odd character level and an ability score increase
 * every 4th character level (both keyed to total level, not class level).
 */
function generalBenefits(characterLevel: number): string[] {
  const benefits: string[] = [];
  if (characterLevel % 2 === 1) {
    benefits.push('Feat');
  }
  if (characterLevel % 4 === 0) {
    benefits.push('Ability score increase');
  }
  return benefits;
}

function makeLevelEntry(classId: string, classLabel: string, classLevel: number, characterLevel: number): LevelEntry {
  const features = CLASS_FEATURES[classId]?.[classLevel] ?? [`${classLabel} class features`];
  return {
    characterLevel,
    classId,
    classLabel,
    classLevel,
    skillPointsBase: classSkillPointsBase(classId),
    features: [...features, ...casterLines(classId, classLevel), ...generalBenefits(characterLevel)],
  };
}

/** Ordered entries for every class level already taken, numbered by character level. */
export function buildLevelEntries(heldClasses: HeldClass[]): LevelEntry[] {
  const entries: LevelEntry[] = [];
  let characterLevel = 0;
  for (const held of heldClasses) {
    for (let classLevel = 1; classLevel <= held.level; classLevel += 1) {
      characterLevel += 1;
      entries.push(makeLevelEntry(held.classId, held.classLabel, classLevel, characterLevel));
    }
  }
  return entries;
}

/** The next level available for each held class — all at the next character level. */
export function buildNextEntries(heldClasses: HeldClass[]): LevelEntry[] {
  const totalLevel = heldClasses.reduce((sum, held) => sum + held.level, 0);
  return heldClasses.map((held) => makeLevelEntry(held.classId, held.classLabel, held.level + 1, totalLevel + 1));
}

/**
 * What taking the next character level in `classId` would grant, whether it's
 * a class the character already holds (levels up by one) or a brand-new class
 * (starts at class level 1) — either way at the next total character level.
 */
export function previewLevelUp(heldClasses: HeldClass[], classId: string): LevelEntry {
  const totalLevel = heldClasses.reduce((sum, held) => sum + held.level, 0);
  const held = heldClasses.find((entry) => entry.classId === classId);
  const option = CLASS_OPTIONS.find((entry) => entry.id === classId);
  const classLabel = held?.classLabel ?? option?.label ?? 'Adventurer';
  return makeLevelEntry(classId, classLabel, (held?.level ?? 0) + 1, totalLevel + 1);
}

function parseOneClass(segment: string): HeldClass {
  const parts = segment.split(':');
  const level = Number(parts[parts.length - 1]) || 1;
  const classId = parts.slice(0, -1).join(':');
  const option = CLASS_OPTIONS.find((entry) => entry.id === classId);
  const derivedLabel = parts
    .slice(1, -1)
    .join(' ')
    .replace(/\b\w/g, (c) => c.toUpperCase());
  const classLabel = option?.label ?? (derivedLabel || 'Adventurer');
  return { classId, classLabel, level };
}

/**
 * Parse the classes and levels a character holds from its `classSummary`.
 * Single class: `class:fighter:1`. Multiclass: comma-separated segments,
 * e.g. `class:fighter:3,class:wizard:1` — matches `summarize_envelope`'s
 * own `.join(",")` in `character_hub.rs` exactly (verified against a real
 * multiclass save; the previous `/`-separator assumption here never
 * matched the actual wire format, so every multiclass character's HP,
 * skill points, caster level, and Progression rail silently computed
 * against a single garbled pseudo-class instead of the real two).
 */
export function parseHeldClasses(classSummary: string): HeldClass[] {
  return classSummary
    .split(',')
    .map((segment) => parseOneClass(segment.trim()))
    .filter((held) => held.classId);
}

/** Combined class label, e.g. `Fighter 3 / Wizard 1`. */
export function formatHeldClasses(classSummary: string): string {
  return parseHeldClasses(classSummary)
    .map((held) => `${held.classLabel} ${held.level}`)
    .join(' / ');
}

/** Total character level across all held classes. */
export function totalCharacterLevel(classSummary: string): number {
  return parseHeldClasses(classSummary).reduce((sum, held) => sum + held.level, 0);
}

// PF1 full spellcasting classes — their levels sum into the caster level.
const CASTER_CLASSES = new Set(['class:wizard', 'class:sorcerer', 'class:cleric', 'class:druid', 'class:bard']);

/** Caster level: total levels in full spellcasting classes (0 for a non-caster). */
export function casterLevel(classSummary: string): number {
  return parseHeldClasses(classSummary)
    .filter((held) => CASTER_CLASSES.has(held.classId))
    .reduce((sum, held) => sum + held.level, 0);
}

export function classHitDie(classId: string): number {
  return CLASS_OPTIONS.find((option) => option.id === classId)?.hitDie ?? 8;
}

/**
 * PF1 max HP: the very first character level takes the maximum hit die; every
 * level after takes the class hit die's average (half + 1). The constitution
 * modifier applies at every level. Floored at 1.
 */
export function maxHitPoints(heldClasses: HeldClass[], constitutionModifier: number): number {
  let hitPoints = 0;
  let isFirstLevel = true;
  for (const held of heldClasses) {
    const hitDie = classHitDie(held.classId);
    for (let levelIndex = 0; levelIndex < held.level; levelIndex += 1) {
      hitPoints += isFirstLevel ? hitDie : Math.floor(hitDie / 2) + 1;
      hitPoints += constitutionModifier;
      isFirstLevel = false;
    }
  }
  return Math.max(1, hitPoints);
}

/** Total skill ranks per level for this character: base + Int modifier (+1 for humans), floored at 1. */
export function totalSkillPoints(base: number, intelligenceModifier: number, isHuman: boolean): number {
  return Math.max(1, base + intelligenceModifier + (isHuman ? 1 : 0));
}
