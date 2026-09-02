import { resolveAddSpellAffordance } from './addSpellAffordanceModel';
import type { ClassSpellLevelsDto } from '../boundary/loadClassSpellLevels';
import { assertEqual } from '../testSupport/asserts';

/**
 * F-10 (scout audit item 45): the Spells tab offered "Add Spell" to every
 * character, and for a Fighter the picker fell back to all 1185 records.
 * The engine's `list_class_spell_levels` answers `known: false` for a
 * class it has no spell list for; that is the signal. It does not separate
 * "no list because not a caster" from "caster whose list is not ingested"
 * (Magus / Summoner / Oracle), so those named classes stay offered with an
 * honest note rather than being locked out by a fact the engine has not
 * established.
 */
const levels: ClassSpellLevelsDto[] = [
  { classId: 'class:wizard', known: true, entries: [{ key: 'Magic Missile', level: 1 }] },
  { classId: 'class:fighter', known: false, entries: [] },
  { classId: 'class:oracle', known: false, entries: [] },
];

const wizard = resolveAddSpellAffordance({ classId: 'class:wizard', classLabel: 'Wizard' }, levels);
assertEqual(wizard.kind, 'Offered', 'a class with an ingested list is offered');

const fighter = resolveAddSpellAffordance({ classId: 'class:fighter', classLabel: 'Fighter' }, levels);
assertEqual(fighter.kind, 'Withheld', 'known:false and not a named uningested caster → withheld');
assertEqual(
  fighter.kind === 'Withheld' ? fighter.message : '',
  'Fighter has no spell list in the loaded books, so there is nothing to add.',
  'the message names the class and the engine fact',
);

const oracle = resolveAddSpellAffordance({ classId: 'class:oracle', classLabel: 'Oracle' }, levels);
assertEqual(oracle.kind, 'OfferedUningested', 'named uningested caster stays offered');
assertEqual(
  oracle.kind === 'OfferedUningested' ? oracle.note : '',
  'Oracle is a caster whose spell list is not ingested yet, so the picker offers every record.',
  'with an honest note',
);

const pending = resolveAddSpellAffordance({ classId: 'class:cleric', classLabel: 'Cleric' }, levels);
assertEqual(pending.kind, 'Offered', 'no answer for the class yet → do not withhold on missing data');

const none = resolveAddSpellAffordance(null, levels);
assertEqual(none.kind, 'Withheld', 'no held class → nothing to attribute a spell to');

console.log('addSpellAffordanceModel tests passed');
