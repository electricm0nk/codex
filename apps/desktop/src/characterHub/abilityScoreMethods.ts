import { ABILITY_KEYS, rollDice, type AbilityKey } from './characterHubModel';

/**
 * PF1 core-rulebook ability score generation methods, plus a few common
 * house-rule dice variants referenced in GM guidance. `kind` drives how the
 * Create Character form renders each method:
 *  - `manual`: free-typed numbers (today's only behavior, kept as the default).
 *  - `pool`: six values generated up front, then assigned to abilities by hand.
 *  - `straight`: six values generated and assigned automatically in
 *    Str/Dex/Con/Int/Wis/Cha order — no rearranging.
 *  - `pointBuy`: every ability starts at 10 and is bought up/down from a
 *    shared point pool using `POINT_BUY_COST_TABLE`.
 */
export type AbilityScoreMethodId =
  | 'manual'
  | 'standardRoll'
  | 'eliteArray'
  | 'roll3d6Straight'
  | 'pointBuy'
  | 'roll2d6Plus6'
  | 'roll4d6RerollOnes'
  | 'roll4d6RerollIfWeak';

export type AbilityScoreMethodKind = 'manual' | 'pool' | 'straight' | 'pointBuy';

export interface AbilityScoreMethodOption {
  id: AbilityScoreMethodId;
  label: string;
  description: string;
  kind: AbilityScoreMethodKind;
}

export const ABILITY_SCORE_METHOD_OPTIONS: AbilityScoreMethodOption[] = [
  {
    id: 'manual',
    label: 'Manual entry',
    description: 'Type each ability score directly.',
    kind: 'manual',
  },
  {
    id: 'standardRoll',
    label: '4d6, drop lowest (Standard)',
    description: 'Roll 4d6 six times, dropping the lowest die each time, then assign the six results as you like.',
    kind: 'pool',
  },
  {
    id: 'eliteArray',
    label: 'Elite Array',
    description: 'Fixed array 15, 14, 13, 12, 10, 8 — assign as you like. No luck involved; used for most published NPCs.',
    kind: 'pool',
  },
  {
    id: 'roll3d6Straight',
    label: '3d6, straight down (Random)',
    description: 'Roll 3d6 six times, assigned in order: Str, Dex, Con, Int, Wis, Cha. No rearranging — higher variance, higher risk.',
    kind: 'straight',
  },
  {
    id: 'pointBuy',
    label: 'Point Buy',
    description: 'Spend a point pool to buy scores from 7 to 18; higher scores cost progressively more points.',
    kind: 'pointBuy',
  },
  {
    id: 'roll2d6Plus6',
    label: '2d6 + 6 (house rule)',
    description: 'Smooths out the low end — no ability below 8. Assign the six results as you like.',
    kind: 'pool',
  },
  {
    id: 'roll4d6RerollOnes',
    label: '4d6 drop lowest, reroll 1s (house rule)',
    description: 'Like Standard, but any die showing a 1 is rerolled once before dropping the lowest. Pushes the average up.',
    kind: 'pool',
  },
  {
    id: 'roll4d6RerollIfWeak',
    label: '4d6 drop lowest, reroll if weak (house rule)',
    description: 'Like Standard, but the whole set of six is rerolled if the total ability modifier is 0 or negative — a "no dud characters" safeguard.',
    kind: 'pool',
  },
];

export function abilityScoreMethodOption(id: AbilityScoreMethodId): AbilityScoreMethodOption {
  return ABILITY_SCORE_METHOD_OPTIONS.find((option) => option.id === id) ?? ABILITY_SCORE_METHOD_OPTIONS[0];
}

/** PF1 core rulebook "Purchasing Ability Scores" cost table. */
export const POINT_BUY_COST_TABLE: Record<number, number> = {
  7: -4,
  8: -2,
  9: -1,
  10: 0,
  11: 1,
  12: 2,
  13: 3,
  14: 5,
  15: 7,
  16: 10,
  17: 13,
  18: 17,
};

export const POINT_BUY_MIN_SCORE = 7;
export const POINT_BUY_MAX_SCORE = 18;
export const POINT_BUY_DEFAULT_SCORE = 10;

export function pointBuyCost(score: number): number {
  return POINT_BUY_COST_TABLE[score] ?? 0;
}

export const POINT_BUY_POOL_PRESETS: ReadonlyArray<{ label: string; points: number }> = [
  { label: 'Low Fantasy (15)', points: 15 },
  { label: 'Standard (20)', points: 20 },
  { label: 'High Fantasy (25)', points: 25 },
  { label: 'Epic (30)', points: 30 },
];

export const POINT_BUY_DEFAULT_POOL = 20;

function rollFourDropLowest(): number {
  const dice = Array.from({ length: 4 }, () => Math.floor(Math.random() * 6) + 1);
  dice.sort((a, b) => a - b);
  return dice[1] + dice[2] + dice[3];
}

function rollFourDropLowestRerollOnes(): number {
  const dice = Array.from({ length: 4 }, () => {
    const first = Math.floor(Math.random() * 6) + 1;
    return first === 1 ? Math.floor(Math.random() * 6) + 1 : first;
  });
  dice.sort((a, b) => a - b);
  return dice[1] + dice[2] + dice[3];
}

function totalAbilityModifier(scores: number[]): number {
  return scores.reduce((sum, score) => sum + Math.floor((score - 10) / 2), 0);
}

// Defensive cap so a pathological run of bad luck can't loop forever — in
// practice a non-positive six-ability modifier total is rare and this
// resolves in a handful of iterations.
const REROLL_IF_WEAK_MAX_ATTEMPTS = 200;

function rollStandardSetRerollIfWeak(): number[] {
  let scores: number[] = [];
  for (let attempt = 0; attempt < REROLL_IF_WEAK_MAX_ATTEMPTS; attempt += 1) {
    scores = Array.from({ length: 6 }, rollFourDropLowest);
    if (totalAbilityModifier(scores) > 0) {
      return scores;
    }
  }
  return scores;
}

/** Generates six values (unassigned) for every `kind: 'pool'` method. */
export function generateAbilityScorePool(methodId: AbilityScoreMethodId): number[] {
  switch (methodId) {
    case 'standardRoll':
      return Array.from({ length: 6 }, rollFourDropLowest);
    case 'eliteArray':
      return [15, 14, 13, 12, 10, 8];
    case 'roll2d6Plus6':
      return Array.from({ length: 6 }, () => rollDice(2, 6) + 6);
    case 'roll4d6RerollOnes':
      return Array.from({ length: 6 }, rollFourDropLowestRerollOnes);
    case 'roll4d6RerollIfWeak':
      return rollStandardSetRerollIfWeak();
    default:
      return [];
  }
}

/** Rolls 3d6 six times straight into ability order — no rearranging. */
export function rollStraightAbilityScores(): Record<AbilityKey, number> {
  const values = Array.from({ length: 6 }, () => rollDice(3, 6));
  return ABILITY_KEYS.reduce(
    (scores, key, index) => ({ ...scores, [key]: values[index] }),
    {} as Record<AbilityKey, number>
  );
}
