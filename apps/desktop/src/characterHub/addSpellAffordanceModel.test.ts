import { resolveAddSpellAffordance } from './addSpellAffordanceModel';
import type { ClassSpellLevelsDto } from '../boundary/loadClassSpellLevels';
import { assertEqual } from '../testSupport/asserts';

/**
 * F-10 → F-16 (scout audit item 45): the Spells tab decides whether to
 * offer "Add Spell" from the engine's `spellcasting` status on
 * `list_class_spell_levels` (v0.8 B-9, read from the corpus record's
 * `FACT:SpellType` token). The hand-assembled TS caster set that F-10
 * shipped under disclosure is gone; no class name appears in this module.
 *
 * `classNotInCorpus` is unknown, not "non-caster": it keeps failing open,
 * like a pending fetch, so an unrecognised id never silently loses the
 * affordance.
 */
function entry(classId: string, spellcasting: ClassSpellLevelsDto['spellcasting'], known: boolean): ClassSpellLevelsDto {
  return { classId, known, entries: known ? [{ key: 'Magic Missile', level: 1 }] : [], spellcasting, spellType: null };
}
const levels: ClassSpellLevelsDto[] = [
  entry('class:wizard', 'listIngested', true),
  entry('class:fighter', 'nonCaster', false),
  entry('class:oracle', 'casterListNotIngested', false),
  entry('class:mystery', 'classNotInCorpus', false),
];

assertEqual(resolveAddSpellAffordance({ classId: 'class:wizard', classLabel: 'Wizard' }, levels).kind, 'Offered', 'listIngested → offered');

const fighter = resolveAddSpellAffordance({ classId: 'class:fighter', classLabel: 'Fighter' }, levels);
assertEqual(fighter.kind, 'Withheld', 'nonCaster → withheld');
assertEqual(fighter.kind === 'Withheld' ? fighter.message : '', 'Fighter does not cast spells, so there is nothing to add.', 'names the class and the engine fact');

const oracle = resolveAddSpellAffordance({ classId: 'class:oracle', classLabel: 'Oracle' }, levels);
assertEqual(oracle.kind, 'OfferedUningested', 'casterListNotIngested → offered with a note');
assertEqual(oracle.kind === 'OfferedUningested' ? oracle.note : '', "Oracle casts, but this caster's spell list is not in the loaded books, so the picker offers every record.", 'honest note');

assertEqual(resolveAddSpellAffordance({ classId: 'class:mystery', classLabel: 'Mystery' }, levels).kind, 'Offered', 'classNotInCorpus → unknown, fail open');
assertEqual(resolveAddSpellAffordance({ classId: 'class:cleric', classLabel: 'Cleric' }, levels).kind, 'Offered', 'no answer yet → fail open');
assertEqual(resolveAddSpellAffordance(null, levels).kind, 'Withheld', 'no held class → nothing to attribute a spell to');

// The old hand-assembled exception must be gone: a nonCaster verdict wins
// even for a class name F-10 used to special-case.
const summoner = resolveAddSpellAffordance({ classId: 'class:summoner', classLabel: 'Summoner' }, [entry('class:summoner', 'nonCaster', false)]);
assertEqual(summoner.kind, 'Withheld', 'no TS class-name list overrides the engine verdict');

console.log('addSpellAffordanceModel tests passed');
