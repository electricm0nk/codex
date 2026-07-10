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
  'class:fighter': {
    1: ['Bonus combat feat'],
    2: ['Bravery +1', 'Bonus combat feat'],
    3: ['Armor training 1'],
    4: ['Bonus combat feat'],
    5: ['Weapon training 1'],
    6: ['Bravery +2', 'Bonus combat feat'],
    7: ['Armor training 2'],
    8: ['Bonus combat feat'],
  },
};

export interface HeldClass {
  classId: string;
  classLabel: string;
  level: number;
}

export interface LevelBenefit {
  classId: string;
  classLabel: string;
  level: number;
  /** Base skill ranks for this class before the Intelligence modifier. */
  skillPointsBase: number;
  features: string[];
}

export function classSkillPointsBase(classId: string): number {
  return CLASS_SKILL_POINTS[classId] ?? 2;
}

/** PF1 grants a feat at every odd level and an ability score increase every 4th level. */
function generalBenefits(level: number): string[] {
  const benefits: string[] = [];
  if (level % 2 === 1) {
    benefits.push('Feat');
  }
  if (level % 4 === 0) {
    benefits.push('Ability score increase');
  }
  return benefits;
}

export function classLevelBenefit(classId: string, classLabel: string, level: number): LevelBenefit {
  const features = CLASS_FEATURES[classId]?.[level] ?? [`${classLabel} class features`];
  return {
    classId,
    classLabel,
    level,
    skillPointsBase: classSkillPointsBase(classId),
    features: [...features, ...generalBenefits(level)],
  };
}

/**
 * Parse the classes and levels a character holds from its `classSummary`
 * (e.g. `class:fighter:1`). Currently a character carries a single class; the
 * return type is an array so multiclass characters slot in without a rewrite.
 */
export function parseHeldClasses(classSummary: string): HeldClass[] {
  const parts = classSummary.split(':');
  const level = Number(parts[parts.length - 1]) || 1;
  const classId = parts.slice(0, -1).join(':');
  const option = CLASS_OPTIONS.find((entry) => entry.id === classId);
  const derivedLabel = parts
    .slice(1, -1)
    .join(' ')
    .replace(/\b\w/g, (c) => c.toUpperCase());
  const classLabel = option?.label ?? (derivedLabel || 'Adventurer');
  return [{ classId, classLabel, level }];
}

/** Total skill ranks per level for this character: base + Int modifier (+1 for humans), floored at 1. */
export function totalSkillPoints(base: number, intelligenceModifier: number, isHuman: boolean): number {
  return Math.max(1, base + intelligenceModifier + (isHuman ? 1 : 0));
}
