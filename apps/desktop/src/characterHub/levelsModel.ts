/**
 * The character's levels as the Create screen builds them: one entry per character level, in the
 * order the player added them. Hit points follow PF1's rule: the first character level takes the
 * class's full hit die; every later level defaults to the die's average (half the die, plus 1) and
 * can be rerolled. The player's rolls are kept, so the HP box shows exactly what the list shows.
 *
 * Pure and immutable: every function returns a new list.
 */

export interface CreationLevel {
  classId: string;
  /** The class's hit die size (d6 = 6). Kept on the entry so a level stays valid if the roster reloads. */
  hitDie: number;
  /** The die result for this level (before the Constitution modifier). */
  value: number;
  /** True when the player rerolled this level; false for the maximum and for the default average. */
  rolled: boolean;
}

/** Half the die, plus 1: d6 4, d8 5, d10 6, d12 7. */
export function averageHitDieValue(die: number): number {
  return Math.floor(die / 2) + 1;
}

/** One die roll, 1..=die. `rng` is injectable for tests. */
export function rollHitDie(die: number, rng: () => number = Math.random): number {
  return Math.min(die, Math.floor(rng() * die) + 1);
}

/** Adds one level of `classId`: the maximum for the character's first level, the average otherwise. */
export function addLevel(entries: CreationLevel[], classId: string, hitDie: number): CreationLevel[] {
  const first = entries.length === 0;
  return [...entries, { classId, hitDie, value: first ? hitDie : averageHitDieValue(hitDie), rolled: false }];
}

/** Rerolls one level. The first character level is always the maximum, so it is left alone. */
export function rerollLevel(entries: CreationLevel[], index: number, rng: () => number = Math.random): CreationLevel[] {
  if (index <= 0 || index >= entries.length) {
    return entries;
  }
  return entries.map((entry, i) => (i === index ? { ...entry, value: rollHitDie(entry.hitDie, rng), rolled: true } : entry));
}

/** Removes one level; if that was the first, the new first level takes the full die. */
export function removeLevel(entries: CreationLevel[], index: number): CreationLevel[] {
  if (index < 0 || index >= entries.length) {
    return entries;
  }
  const rest = entries.filter((_, i) => i !== index);
  if (index === 0 && rest.length > 0) {
    rest[0] = { ...rest[0], value: rest[0].hitDie, rolled: false };
  }
  return rest;
}

/** What one level contributes: its die result plus the Constitution modifier, never less than 1. */
export function levelHitPoints(entry: CreationLevel, constitutionModifier: number): number {
  return Math.max(1, entry.value + constitutionModifier);
}

export function totalHitPoints(entries: CreationLevel[], constitutionModifier: number): number {
  return entries.reduce((sum, entry) => sum + levelHitPoints(entry, constitutionModifier), 0);
}

export function characterLevel(entries: CreationLevel[]): number {
  return entries.length;
}

/** Classes in order of first appearance, each with how many levels it holds. */
export function heldClassesOf(entries: CreationLevel[]): Array<{ classId: string; level: number }> {
  const held: Array<{ classId: string; level: number }> = [];
  for (const entry of entries) {
    const existing = held.find((h) => h.classId === entry.classId);
    if (existing) {
      existing.level += 1;
    } else {
      held.push({ classId: entry.classId, level: 1 });
    }
  }
  return held;
}

/**
 * How the list maps onto the create command: the first level's class is the primary class and
 * carries all of its own levels; every level of another class arrives, in order, as an additional
 * level. Every level's die result is sent so the saved character keeps the player's rolls.
 */
export function creationRequestShape(entries: CreationLevel[]): {
  primaryClassId: string;
  primaryLevel: number;
  additionalLevels: string[];
  hitPointLevels: Array<{ classId: string; value: number }>;
} | null {
  if (entries.length === 0) {
    return null;
  }
  const primaryClassId = entries[0].classId;
  return {
    primaryClassId,
    primaryLevel: entries.filter((entry) => entry.classId === primaryClassId).length,
    additionalLevels: entries.filter((entry) => entry.classId !== primaryClassId).map((entry) => entry.classId),
    hitPointLevels: entries.map((entry) => ({ classId: entry.classId, value: entry.value })),
  };
}

/** PF1 characters top out at 20 levels. */
export const MAX_CHARACTER_LEVEL = 20;

/** The slice of a class option the Levels list needs. */
export interface LevelClassOption {
  id: string;
  label: string;
  /** `null` when the class states no hit die: its hit points are Unknown and a level cannot be added. */
  hitDie: number | null;
  /** The levels the engine computes for this class (1..max). */
  levelOptions: readonly number[];
}

/** Whether one more level of `option` can be added, and if not, why. */
export function canAddLevel(entries: CreationLevel[], option: LevelClassOption): { ok: boolean; reason?: string } {
  if (option.hitDie === null) {
    return { ok: false, reason: `${option.label} has no known hit die, so its hit points cannot be worked out.` };
  }
  if (entries.length >= MAX_CHARACTER_LEVEL) {
    return { ok: false, reason: `A character has at most ${MAX_CHARACTER_LEVEL} levels.` };
  }
  const held = entries.filter((entry) => entry.classId === option.id).length;
  const highest = option.levelOptions.length > 0 ? Math.max(...option.levelOptions) : 0;
  if (held >= highest) {
    return { ok: false, reason: `${option.label} is already at its highest level (${highest}) in this app.` };
  }
  return { ok: true };
}
