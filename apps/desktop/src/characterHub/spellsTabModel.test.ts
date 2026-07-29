import {
  describeSpellAcquisition,
  describeSpellSchoolAndLevel,
  resolveSelectedSpellEntries,
} from './spellsTabModel';
import type { SpellCatalogEntryDto } from '../boundary/loadSpellCatalog';
import type { SpellSelectionDto } from '../boundary/loadSavedCharacterDetail';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * The Spells tab previously rendered ONLY `corpusDerived.schoolCoverage`,
 * which `load_saved_character` resolves against the desktop crate's
 * two-record bundled fixture bundle (`corpus_fixtures.rs`'s
 * `SPELL_FIXTURES` — `spell_abjuration.txt`, `spell_illusion.txt`). Every
 * other real, persisted spell a character held fell through to
 * `unresolvedSpellIds` and reached the player as a bare internal id string
 * — the exact shape of the feats defect, with all 652 records present and
 * correct in `SPELL_LIST` and already served by the working `list_spells`
 * command the Add Spell picker uses.
 *
 * These tests pin the resolution the tab now performs against that real
 * catalog: name, school, level and effect text, with a raw-id fallback that
 * never hides a selection.
 */

const CATALOG: SpellCatalogEntryDto[] = [
  {
    key: 'Shield',
    book: 'CRB',
    school: 'Abjuration',
    level: 1,
    description: 'Invisible disc gives +4 to AC, blocks magic missiles.',
  },
  {
    key: 'Magic Missile',
    book: 'CRB',
    school: 'Evocation',
    level: 1,
    description: '1d4+1 damage; +1 missile per two levels above 1st (max 5).',
  },
  {
    key: "Mage's Disjunction",
    book: 'CRB',
    school: 'Abjuration',
    level: 9,
    description: 'Dispels magic, disenchants magic items.',
  },
  // Shape of a real `apg_spells.lst` gap: a record that exists and
  // resolves, but whose corpus row carries no SCHOOL:/CLASSES:/DESC:.
  {
    key: 'Corpus Gap Spell',
    book: 'APG',
    school: null,
    level: null,
    description: null,
  },
];

function selection(spellId: string, acquisitionMode: SpellSelectionDto['acquisitionMode'] = 'Known'): SpellSelectionDto {
  return { spellId, sourceClassId: 'class:wizard', acquisitionMode };
}

function verifiesASelectedSpellResolvesToItsRealNameSchoolLevelAndEffectText() {
  const [row] = resolveSelectedSpellEntries([selection('Magic Missile')], CATALOG);
  assertEqual(row.name, 'Magic Missile', 'a resolved spell renders its real name, not the raw id');
  assertEqual(row.school, 'Evocation', 'a resolved spell carries its real school');
  assertEqual(row.level, 1, 'a resolved spell carries its real level');
  assertEqual(
    row.effectText,
    '1d4+1 damage; +1 missile per two levels above 1st (max 5).',
    'the effect text from SPELL_LIST reaches the row verbatim'
  );
  assert(row.resolved, 'a spell present in the catalog is marked resolved');
}

function verifiesAnUnresolvableSpellFallsBackToTheRawIdRatherThanBeingHidden() {
  const rows = resolveSelectedSpellEntries([selection('Not A Real Spell')], CATALOG);
  assertEqual(rows.length, 1, 'an unresolvable selection still produces a row — never silently dropped');
  assertEqual(rows[0].name, 'Not A Real Spell', 'an unresolvable selection falls back to its raw id as the name');
  assertEqual(rows[0].school, null, 'an unresolvable selection fabricates no school');
  assertEqual(rows[0].level, null, 'an unresolvable selection fabricates no level');
  assertEqual(rows[0].effectText, null, 'an unresolvable selection fabricates no effect text');
  assert(!rows[0].resolved, 'an unresolvable selection is marked unresolved');
}

function verifiesEverySelectionProducesExactlyOneRowInInputOrder() {
  const rows = resolveSelectedSpellEntries(
    [selection('Magic Missile'), selection('Nonsense'), selection('Shield')],
    CATALOG
  );
  assertEqual(rows.length, 3, 'one row per selection, never fewer');
  assertEqual(rows[0].name, 'Magic Missile', 'input order is preserved');
  assertEqual(rows[1].name, 'Nonsense', 'input order is preserved across an unresolved row');
  assertEqual(rows[2].name, 'Shield', 'input order is preserved');
}

function verifiesResolutionToleratesCaseAndPunctuationDriftInTheStoredId() {
  const [row] = resolveSelectedSpellEntries([selection('mages disjunction')], CATALOG);
  assertEqual(
    row.name,
    "Mage's Disjunction",
    'a stored id differing only by case/punctuation still resolves to the real catalog record'
  );
  assertEqual(row.level, 9, 'the fold-matched record carries its real level');
}

function verifiesTheRawStoredIdIsAlwaysPreservedForProvenance() {
  const [row] = resolveSelectedSpellEntries([selection('mages disjunction')], CATALOG);
  assertEqual(row.raw, 'mages disjunction', 'the raw persisted id is preserved verbatim on every row');
}

function verifiesAcquisitionAndSourceClassReachTheRow() {
  const [row] = resolveSelectedSpellEntries([selection('Shield', 'Prepared')], CATALOG);
  assertEqual(row.acquisitionMode, 'Prepared', 'the persisted acquisition mode reaches the row');
  assertEqual(row.sourceClassId, 'class:wizard', 'the persisted source class reaches the row');
}

function verifiesAcquisitionDescriptionReadsForAPlayer() {
  assertEqual(
    describeSpellAcquisition(resolveSelectedSpellEntries([selection('Shield', 'Prepared')], CATALOG)[0]),
    'Prepared · Wizard',
    'the acquisition line names the mode and the human-readable source class'
  );
}

function verifiesAnEmptyCatalogStillRendersEverySelectionAsRawIds() {
  const rows = resolveSelectedSpellEntries([selection('Shield'), selection('Magic Missile')], []);
  assertEqual(rows.length, 2, 'a failed catalog load hides nothing');
  assertEqual(rows[0].name, 'Shield', 'a failed catalog load falls back to raw ids, which are still honest data');
  assert(!rows[0].resolved, 'a failed catalog load marks rows unresolved rather than claiming detail it lacks');
}

function verifiesTheSchoolAndLevelLineNamesTheBook() {
  assertEqual(
    describeSpellSchoolAndLevel(resolveSelectedSpellEntries([selection('Magic Missile')], CATALOG)[0]),
    'CRB · Evocation · Level 1',
    'a fully populated row names its book, school and level'
  );
}

function verifiesARecordWithRealCorpusGapsClaimsNoSchoolOrLevel() {
  const [row] = resolveSelectedSpellEntries([selection('Corpus Gap Spell')], CATALOG);
  assert(row.resolved, 'a record that exists but has corpus gaps still counts as resolved');
  assertEqual(row.school, null, 'a corpus gap is reported as absent, not filled in');
  assertEqual(row.level, null, 'a corpus gap is reported as absent, not filled in');
  assertEqual(row.effectText, null, 'a corpus gap is reported as absent, not filled in');
  assertEqual(
    describeSpellSchoolAndLevel(row),
    'APG',
    'a row with a known book but no school/level names only the book, claiming neither'
  );
}

function verifiesAnUnresolvedRowMakesNoSchoolLevelOrBookClaim() {
  const [row] = resolveSelectedSpellEntries([selection('Not A Real Spell')], CATALOG);
  assertEqual(row.book, null, 'an unresolvable selection fabricates no book');
  assertEqual(describeSpellSchoolAndLevel(row), null, 'an unresolvable selection renders no detail line');
}

async function main() {
  verifiesASelectedSpellResolvesToItsRealNameSchoolLevelAndEffectText();
  verifiesTheSchoolAndLevelLineNamesTheBook();
  verifiesARecordWithRealCorpusGapsClaimsNoSchoolOrLevel();
  verifiesAnUnresolvedRowMakesNoSchoolLevelOrBookClaim();
  verifiesAnUnresolvableSpellFallsBackToTheRawIdRatherThanBeingHidden();
  verifiesEverySelectionProducesExactlyOneRowInInputOrder();
  verifiesResolutionToleratesCaseAndPunctuationDriftInTheStoredId();
  verifiesTheRawStoredIdIsAlwaysPreservedForProvenance();
  verifiesAcquisitionAndSourceClassReachTheRow();
  verifiesAcquisitionDescriptionReadsForAPlayer();
  verifiesAnEmptyCatalogStillRendersEverySelectionAsRawIds();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
