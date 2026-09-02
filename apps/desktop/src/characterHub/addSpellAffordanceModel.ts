import type { ClassSpellLevelsDto } from '../boundary/loadClassSpellLevels';

/**
 * Whether the Spells tab should offer "Add Spell" for the class a pick
 * would be attributed to (v0.8 F-10, cleaned up in F-16 once B-9 landed).
 *
 * Every rule-bearing fact is the engine's `spellcasting` verdict on
 * `list_class_spell_levels`, read from the corpus record's
 * `FACT:SpellType` token. No class name is special-cased here — F-10's
 * disclosed hand-assembled caster set is deleted. `classNotInCorpus` and a
 * missing/pending answer both fail open: withholding on a fact the engine
 * has not established would hide a legal action.
 */
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
  if (entry === undefined) {
    return { kind: 'Offered' };
  }
  switch (entry.spellcasting) {
    case 'nonCaster':
      return { kind: 'Withheld', message: `${routedClass.classLabel} does not cast spells, so there is nothing to add.` };
    case 'casterListNotIngested':
      return {
        kind: 'OfferedUningested',
        note: `${routedClass.classLabel} casts, but this caster's spell list is not in the loaded books, so the picker offers every record.`,
      };
    case 'listIngested':
    case 'classNotInCorpus':
      return { kind: 'Offered' };
  }
}
