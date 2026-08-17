import { buildSpellPickerOffering } from './spellsTabModel';
import type { SpellCatalogEntryDto } from '../boundary/loadSpellCatalog';
import type { ClassSpellLevelsDto } from '../boundary/loadClassSpellLevels';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * The Add Spell picker offered every character all 1185 catalog records.
 * For a Wizard 1, 543 of them are on no wizard list in any ingested book,
 * and every one was accepted and persisted under `class:wizard` — the
 * picker was not merely misleading, it was the delivery mechanism for an
 * illegal character.
 *
 * These tests pin the two halves of the rule apart, because they are
 * different facts:
 *
 * - **membership is filtered** — a spell off the routed class's list can
 *   never become legal, so it must not be offered;
 * - **spell level is not filtered** — PF1 CRB places no character-level
 *   restriction on what a wizard may copy into her spellbook, so a Wizard
 *   1 must still be able to reach a 9th-level wizard spell. Hiding those
 *   rows would remove a legal action.
 *
 * They also pin that the level shown is the **class's** level, not the
 * catalog record's minimum-across-classes one, and that a class with no
 * ingested list is not narrowed at all.
 */

const CATALOG: SpellCatalogEntryDto[] = [
  // On the wizard list at the character's own castable level.
  { key: 'Magic Missile', book: 'CRB', school: 'Evocation', level: 1, description: '1d4+1 force damage.', duration: null, range: null },
  // On the wizard list, far above a Wizard 1's castable level. Legal to
  // scribe, so it must survive the filter.
  { key: 'Tsunami', book: 'APG', school: 'Conjuration', level: 9, description: 'A wave of water.', duration: null, range: null },
  // The minimum-across-classes defect in one row: the record says 1
  // (Bard's level), a Wizard learns it at 2.
  { key: 'Hideous Laughter', book: 'CRB', school: 'Enchantment', level: 1, description: 'Subject loses actions.', duration: null, range: null },
  // Cleric/Druid/Bard spell. On no wizard list — must be removed.
  { key: 'Cure Light Wounds', book: 'CRB', school: 'Conjuration', level: 1, description: 'Cures 1d8 damage.', duration: null, range: null },
  // Druid-only. Must be removed.
  { key: 'Antilife Shell', book: 'CRB', school: 'Abjuration', level: 6, description: '10-ft. field hedges out living creatures.', duration: null, range: null },
  // A real `apg_spells.lst` gap: resolves, but the corpus row carries no
  // SCHOOL: token. It is on the wizard list, so it stays — with no school
  // fabricated into its detail line.
  { key: 'Corpus Gap Spell', book: 'APG', school: null, level: null, description: null, duration: null, range: null },
];

const CLASS_SPELL_LEVELS: ClassSpellLevelsDto[] = [
  {
    classId: 'class:wizard',
    known: true,
    entries: [
      { key: 'Magic Missile', level: 1 },
      { key: 'Tsunami', level: 9 },
      { key: 'Hideous Laughter', level: 2 },
      { key: 'Corpus Gap Spell', level: 4 },
    ],
  },
  {
    classId: 'class:cleric',
    known: true,
    entries: [{ key: 'Cure Light Wounds', level: 1 }],
  },
  // A real class the engine has ingested no spell list for.
  { classId: 'class:magus', known: false, entries: [] },
];

async function main() {
  verifiesSpellsOffTheRoutedClassListAreNotOffered();
  verifiesAnAboveCastableLevelSpellIsStillOfferedBecauseScribingItIsLegal();
  verifiesTheLevelShownIsTheClassLevelNotTheRecordMinimum();
  verifiesADifferentClassGetsItsOwnList();
  verifiesAClassWithNoIngestedListIsNotNarrowed();
  verifiesNoRoutedClassIsNotNarrowed();
  verifiesASchoolIsNeverFabricatedForACorpusGapRow();
}

function offering(classId: string | null) {
  return buildSpellPickerOffering(CATALOG, CLASS_SPELL_LEVELS, classId);
}

function verifiesSpellsOffTheRoutedClassListAreNotOffered() {
  const keys = offering('class:wizard').map((entry) => entry.key);
  assert(
    !keys.includes('Cure Light Wounds'),
    'a Cleric/Druid/Bard spell is on no wizard list and must not be offered to a wizard'
  );
  assert(
    !keys.includes('Antilife Shell'),
    'a Druid-only spell is on no wizard list and must not be offered to a wizard'
  );
  assertEqual(keys.length, 4, 'only the four wizard-list records survive the filter');
}

function verifiesAnAboveCastableLevelSpellIsStillOfferedBecauseScribingItIsLegal() {
  const row = offering('class:wizard').find((entry) => entry.key === 'Tsunami');
  assert(
    row !== undefined,
    'PF1 CRB places no character-level restriction on copying a spell into a spellbook, so a ' +
      '9th-level wizard spell must stay reachable — filtering it out would remove a legal action'
  );
  assertEqual(
    row?.detail,
    'APG · Conjuration · Wizard level 9',
    'and the row must say plainly that it is a 9th-level wizard spell'
  );
}

function verifiesTheLevelShownIsTheClassLevelNotTheRecordMinimum() {
  const row = offering('class:wizard').find((entry) => entry.key === 'Hideous Laughter');
  assertEqual(
    row?.detail,
    'CRB · Enchantment · Wizard level 2',
    "the record's own level is 1 (Bard's), but a Wizard learns Hideous Laughter at 2 — the " +
      'picker must show the class level, never the minimum across classes'
  );
}

function verifiesADifferentClassGetsItsOwnList() {
  const entries = offering('class:cleric');
  assertEqual(entries.length, 1, "a cleric is offered the cleric list, not the wizard's");
  assertEqual(entries[0].key, 'Cure Light Wounds', 'and it is the one on the cleric list');
  assertEqual(
    entries[0].detail,
    'CRB · Conjuration · Cleric level 1',
    "the row is labelled with the cleric's own level"
  );
}

function verifiesAClassWithNoIngestedListIsNotNarrowed() {
  const result = offering('class:magus');
  assertEqual(
    result.length,
    CATALOG.length,
    'no ingested list means no membership fact exists here — narrowing on a rule this app ' +
      'does not have would be inventing one'
  );
  const row = result.find((entry) => entry.key === 'Hideous Laughter');
  assertEqual(
    row?.detail,
    'CRB · Enchantment · Lowest class level 1',
    "an unnarrowed row keeps the honest 'Lowest class level' label rather than claiming the " +
      "record's number is this class's"
  );
}

function verifiesNoRoutedClassIsNotNarrowed() {
  assertEqual(
    offering(null).length,
    CATALOG.length,
    'a character with no held class narrows nothing — there is no list to narrow against'
  );
}

function verifiesASchoolIsNeverFabricatedForACorpusGapRow() {
  const row = offering('class:wizard').find((entry) => entry.key === 'Corpus Gap Spell');
  assertEqual(
    row?.detail,
    'APG · Wizard level 4',
    'a corpus row with no SCHOOL: token omits the school rather than inventing one, while ' +
      'still carrying the class level the class list does state'
  );
}

void main();
