import type { ClassSpellLevelsDto } from '../boundary/loadClassSpellLevels';

/**
 * Whether the Spells tab should offer "Add Spell" for the class a pick
 * would be attributed to (v0.8 F-10, scout audit item 45).
 *
 * The rule-bearing fact is the engine's: `list_class_spell_levels` returns
 * `known: false` when it holds no spell list for a class. That flag alone
 * does not say *why* — a Fighter has no list because Fighters do not cast;
 * an Oracle has none because the list is not ingested (see
 * `loadClassSpellLevels.ts`). The classes in `UNINGESTED_CASTERS` are the
 * ones the boundary doc itself names as real casters with no ingested
 * list; they stay offered with a note, everything else `known: false` is
 * withheld. No caster/non-caster judgement is made here beyond that named
 * exception list.
 */
const UNINGESTED_CASTERS: ReadonlySet<string> = new Set([
  'class:magus',
  'class:oracle',
  'class:summoner',
  'class:unchained_summoner',
]);

export type AddSpellAffordance =
  | { kind: 'Offered' }
  | { kind: 'OfferedUningested'; note: string }
  | { kind: 'Withheld'; message: string };

export function resolveAddSpellAffordance(
  routedClass: { classId: string; classLabel: string } | null,
  classSpellLevels: readonly ClassSpellLevelsDto[],
): AddSpellAffordance {
  if (routedClass === null) {
    return { kind: 'Withheld', message: 'This character has no class to learn a spell from yet.' };
  }
  const entry = classSpellLevels.find((candidate) => candidate.classId === routedClass.classId);
  if (entry === undefined || entry.known) {
    // No answer yet (still loading, or the fetch failed) is not evidence of
    // anything; withholding on it would hide a legal action.
    return { kind: 'Offered' };
  }
  if (UNINGESTED_CASTERS.has(routedClass.classId)) {
    return {
      kind: 'OfferedUningested',
      note: `${routedClass.classLabel} is a caster whose spell list is not ingested yet, so the picker offers every record.`,
    };
  }
  return {
    kind: 'Withheld',
    message: `${routedClass.classLabel} has no spell list in the loaded books, so there is nothing to add.`,
  };
}
