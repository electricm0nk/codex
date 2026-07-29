import {
  describeSpellAcquisition,
  describeSpellSchoolAndLevel,
  resolveSelectedSpellEntries,
  spellSourceClassIds,
} from './spellsTabModel';
import type { SpellCatalogEntryDto } from '../boundary/loadSpellCatalog';
import type { ClassSpellLevelsDto } from '../boundary/loadClassSpellLevels';
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

/**
 * The canonical example of the per-class level defect. Its real corpus tag
 * is `CLASSES:Bard=1|Sorcerer,Wizard=2`, so the catalog record's own
 * `level` is 1 — the Bard level — and a Wizard's sheet showed "Level 1"
 * for a spell a Wizard learns at 2.
 */
const HIDEOUS_LAUGHTER: SpellCatalogEntryDto = {
  key: 'Hideous Laughter',
  book: 'CRB',
  school: 'Enchantment',
  level: 1,
  description: 'The subject perceives everything as hilariously funny.',
};

const CATALOG_WITH_HIDEOUS_LAUGHTER: SpellCatalogEntryDto[] = [...CATALOG, HIDEOUS_LAUGHTER];

/** Shaped exactly like the `list_class_spell_levels` response's rows. */
const CLASS_LEVELS: ClassSpellLevelsDto[] = [
  {
    classId: 'class:wizard',
    known: true,
    entries: [
      { key: 'Hideous Laughter', level: 2 },
      { key: 'Magic Missile', level: 1 },
      { key: "Mage's Disjunction", level: 9 },
      { key: 'Shield', level: 1 },
    ],
  },
  {
    classId: 'class:bard',
    known: true,
    entries: [{ key: 'Hideous Laughter', level: 1 }],
  },
  // A real class the engine has ingested no spell list for. It names
  // itself in genuine corpus `CLASSES:` tags, so its levels are knowable —
  // they just are not known here, and must not be guessed.
  { classId: 'class:magus', known: false, entries: [] },
];

function selection(
  spellId: string,
  acquisitionMode: SpellSelectionDto['acquisitionMode'] = 'Known',
  sourceClassId = 'class:wizard'
): SpellSelectionDto {
  return { spellId, sourceClassId, acquisitionMode };
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
  // With the per-class response present, the level is stated as that
  // class's level. The bare "Level 1" this once read was the record's
  // minimum across classes wearing a label it had not earned.
  assertEqual(
    describeSpellSchoolAndLevel(
      resolveSelectedSpellEntries([selection('Magic Missile')], CATALOG, CLASS_LEVELS)[0]
    ),
    'CRB · Evocation · Wizard level 1',
    'a fully populated row names its book, school and the level for its own class'
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

/**
 * The bug, at the surface a player reads. Before this, both rows below
 * rendered "Level 1" — the catalog record's minimum-across-classes level.
 */
function verifiesAWizardSeesHideousLaughterAtLevelTwoAndABardAtLevelOne() {
  const [wizardRow, bardRow] = resolveSelectedSpellEntries(
    [
      selection('Hideous Laughter', 'Known', 'class:wizard'),
      selection('Hideous Laughter', 'Known', 'class:bard'),
    ],
    CATALOG_WITH_HIDEOUS_LAUGHTER,
    CLASS_LEVELS
  );

  assertEqual(wizardRow.level, 1, "the catalog record's own level is still reported verbatim");
  assertEqual(wizardRow.classLevel, 2, 'a Wizard learns Hideous Laughter at 2, not the record-wide 1');
  assertEqual(wizardRow.classLevelStatus, 'class-level', 'the Wizard level is genuinely known');
  assertEqual(
    describeSpellSchoolAndLevel(wizardRow),
    'CRB · Enchantment · Wizard level 2',
    'the level line names the class it applies to and shows that class’s real level'
  );

  assertEqual(bardRow.classLevel, 1, 'a Bard learns Hideous Laughter at 1');
  assertEqual(
    describeSpellSchoolAndLevel(bardRow),
    'CRB · Enchantment · Bard level 1',
    'the same spell reads differently for a different source class — no arbitration needed'
  );
}

/**
 * The multiclass rule: each row is resolved against its OWN persisted
 * `sourceClassId`, so a character holding two classes never needs a
 * "primary class" tiebreak. Both rows above are on one character here.
 */
function verifiesEachRowUsesItsOwnSourceClassOnAMulticlassCharacter() {
  const rows = resolveSelectedSpellEntries(
    [
      selection('Magic Missile', 'Known', 'class:wizard'),
      selection('Hideous Laughter', 'Known', 'class:bard'),
      selection('Hideous Laughter', 'Prepared', 'class:wizard'),
    ],
    CATALOG_WITH_HIDEOUS_LAUGHTER,
    CLASS_LEVELS
  );
  assertEqual(rows[0].classLevel, 1, 'Magic Missile is a 1st-level Wizard spell');
  assertEqual(rows[1].classLevel, 1, 'the Bard row takes the Bard level');
  assertEqual(rows[2].classLevel, 2, 'the Wizard row takes the Wizard level, on the same character');
}

/**
 * A class the engine has no ingested list for must not be handed the
 * catalog level as if it were that class's level. The record's own number
 * is still real data, so it is shown — explicitly labelled as what it is.
 */
function verifiesAnUningestedSourceClassLabelsTheRecordLevelRatherThanClaimingIt() {
  const [row] = resolveSelectedSpellEntries(
    [selection('Hideous Laughter', 'Known', 'class:magus')],
    CATALOG_WITH_HIDEOUS_LAUGHTER,
    CLASS_LEVELS
  );
  assertEqual(row.classLevel, null, 'no per-class level is invented for an uningested class');
  assertEqual(row.classLevelStatus, 'class-list-unknown', 'the gap is named as a gap');
  assertEqual(
    describeSpellSchoolAndLevel(row),
    'CRB · Enchantment · Lowest class level 1',
    'the record level is shown only under a label saying what it actually measures'
  );
}

/** A class id absent from the response entirely behaves like an unknown class. */
function verifiesASourceClassMissingFromTheResponseIsTreatedAsUnknown() {
  const [row] = resolveSelectedSpellEntries(
    [selection('Hideous Laughter', 'Known', 'class:fighter')],
    CATALOG_WITH_HIDEOUS_LAUGHTER,
    CLASS_LEVELS
  );
  assertEqual(row.classLevelStatus, 'class-list-unknown', 'an unrequested class claims no level');
  assertEqual(row.classLevel, null, 'an unrequested class fabricates no level');
}

/**
 * A known class whose list simply does not contain the spell is a
 * different fact from "we have no list for this class", and reads
 * differently: no Bard casts Mage's Disjunction at any level.
 */
function verifiesASpellOffTheClassListSaysSoRatherThanShowingALevel() {
  const [row] = resolveSelectedSpellEntries(
    [selection("Mage's Disjunction", 'Known', 'class:bard')],
    CATALOG_WITH_HIDEOUS_LAUGHTER,
    CLASS_LEVELS
  );
  assertEqual(row.classLevel, null, 'a spell off the class list has no level for that class');
  assertEqual(row.classLevelStatus, 'not-on-class-list', 'the two kinds of absence stay distinguishable');
  assertEqual(
    describeSpellSchoolAndLevel(row),
    'CRB · Abjuration · Not on the Bard spell list',
    'a spell no Bard can cast says so instead of showing the record level'
  );
}

/** Before the per-class response arrives, no row may claim a class level. */
function verifiesNoClassLevelIsClaimedBeforeTheResponseLoads() {
  const [row] = resolveSelectedSpellEntries(
    [selection('Hideous Laughter')],
    CATALOG_WITH_HIDEOUS_LAUGHTER,
    []
  );
  assertEqual(row.classLevel, null, 'nothing is claimed while the per-class data is still loading');
  assertEqual(row.classLevelStatus, 'class-list-unknown', 'the loading state reads as unknown, not as a level');
  assertEqual(
    describeSpellSchoolAndLevel(row),
    'CRB · Enchantment · Lowest class level 1',
    'the pre-load line still labels the record level honestly'
  );
}

/** An unresolved selection claims nothing at all, class level included. */
function verifiesAnUnresolvedRowClaimsNoClassLevelEither() {
  const [row] = resolveSelectedSpellEntries(
    [selection('Not A Real Spell')],
    CATALOG_WITH_HIDEOUS_LAUGHTER,
    CLASS_LEVELS
  );
  assertEqual(row.classLevel, null, 'an unresolvable selection fabricates no class level');
  assertEqual(row.classLevelStatus, 'class-list-unknown', 'an unresolvable selection makes no class claim');
  assertEqual(describeSpellSchoolAndLevel(row), null, 'an unresolvable selection still renders no detail line');
}

/** The class-level lookup tolerates the same id drift the catalog fold does. */
function verifiesClassLevelResolutionToleratesTheSameIdDriftAsTheCatalog() {
  const [row] = resolveSelectedSpellEntries(
    [selection('hideous laughter')],
    CATALOG_WITH_HIDEOUS_LAUGHTER,
    CLASS_LEVELS
  );
  assertEqual(row.classLevel, 2, 'a case-drifted stored id still reaches the real per-class level');
}

/**
 * What the tab must request from the backend: the distinct source classes
 * its own rows actually reference, so a character never pulls lists it has
 * no spells from.
 */
function verifiesTheDistinctSourceClassesAreDerivedFromTheSelectionsThemselves() {
  const ids = spellSourceClassIds([
    selection('Magic Missile', 'Known', 'class:wizard'),
    selection('Hideous Laughter', 'Known', 'class:bard'),
    selection('Shield', 'Prepared', 'class:wizard'),
  ]);
  assertEqual(ids.length, 2, 'each source class is requested once, not once per spell');
  assertEqual(ids[0], 'class:bard', 'the request list is sorted so it is stable across renders');
  assertEqual(ids[1], 'class:wizard', 'the request list is sorted so it is stable across renders');
  assertEqual(spellSourceClassIds([]).length, 0, 'a character with no spells requests nothing');
}

async function main() {
  verifiesAWizardSeesHideousLaughterAtLevelTwoAndABardAtLevelOne();
  verifiesEachRowUsesItsOwnSourceClassOnAMulticlassCharacter();
  verifiesAnUningestedSourceClassLabelsTheRecordLevelRatherThanClaimingIt();
  verifiesASourceClassMissingFromTheResponseIsTreatedAsUnknown();
  verifiesASpellOffTheClassListSaysSoRatherThanShowingALevel();
  verifiesNoClassLevelIsClaimedBeforeTheResponseLoads();
  verifiesAnUnresolvedRowClaimsNoClassLevelEither();
  verifiesClassLevelResolutionToleratesTheSameIdDriftAsTheCatalog();
  verifiesTheDistinctSourceClassesAreDerivedFromTheSelectionsThemselves();
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
