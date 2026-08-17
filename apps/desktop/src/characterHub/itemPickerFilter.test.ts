import {
  describeFeatCatalogCoverage,
  filterItemPickerEntries,
  mapEquipmentCatalogEntries,
  mapFeatCatalogEntries,
  mapSpellCatalogEntries,
  summariseItemDescription,
  ITEM_PICKER_DESCRIPTION_MAX_CHARS,
} from './itemPickerFilter';
import type { EquipmentCatalogEntryDto } from '../boundary/loadEquipmentCatalog';
import type { SpellCatalogEntryDto } from '../boundary/loadSpellCatalog';
import type { FeatCatalogEntryDto } from '../boundary/listFeats';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * Real catalog rows. `description` is each record's verbatim corpus `DESC:`
 * prose as `equipment_catalog.rs` serves it (`crb::equipment_data`), except
 * Banded Mail, which is `null` here to keep a no-description row in every
 * mapping assertion — 974 of the 3830 served records are in that state.
 */
const EQUIPMENT_ENTRIES: EquipmentCatalogEntryDto[] = [
  { key: 'equipment:longsword', category: 'ArmsArmor', name: 'Longsword', costGp: 15, book: 'CRB', description: 'This sword is about 3-1/2 feet in length.' },
  { key: 'equipment:banded_mail', category: 'ArmsArmor', name: 'Banded Mail', costGp: 250, book: 'CRB', description: null },
  { key: 'equipment:potion_of_cure_light_wounds', category: 'MagicItems', name: 'Potion of Cure Light Wounds', costGp: 50, book: 'CRB', description: 'Cures 1d8+1 damage.' },
];

const SPELL_ENTRIES: SpellCatalogEntryDto[] = [
  { key: 'spell:magic_missile', book: 'CRB', school: 'Evocation', level: 1, description: 'A missile of magical energy.', duration: null, range: null },
  { key: 'spell:fireball', book: 'CRB', school: 'Evocation', level: 3, description: 'A burst of flame.', duration: null, range: null },
  { key: 'spell:cure_light_wounds', book: 'CRB', school: 'Conjuration', level: 1, description: 'Heals wounds.', duration: null, range: null },
  // A real `apg_spells.lst` gap shape: resolves, but the corpus row
  // carries no SCHOOL:/CLASSES:/DESC: token.
  { key: 'spell:corpus_gap', book: 'APG', school: null, level: null, description: null, duration: null, range: null },
];

/**
 * Real records as `list_feats` serves them, one per ingested book plus an
 * unknown-book row. `Extra Hex` / `Extra Panache` / `Elemental Fist` are
 * verbatim from `apg_feats.lst` and `acg_feats.lst`; `Elemental Fist` is
 * the one ingested APG record whose corpus row genuinely carries no
 * `DESC:` token.
 */
const FEAT_ENTRIES: FeatCatalogEntryDto[] = [
  { key: 'Power Attack', category: 'Combat', name: 'Power Attack', description: 'You can make exceptionally deadly melee attacks by sacrificing accuracy for strength.', source: 'Crb', chooserTargetKind: null },
  { key: 'Extra Hex', category: 'General', name: 'Extra Hex', description: 'You have learned the secrets of a new hex.', source: 'Apg', chooserTargetKind: null },
  { key: 'Elemental Fist', category: 'Combat', name: 'Elemental Fist', description: null, source: 'Apg', chooserTargetKind: null },
  { key: 'Extra Panache', category: 'Panache', name: 'Extra Panache', description: 'You have more panache than the ordinary swashbuckler.', source: 'Acg', chooserTargetKind: null },
  // `Angel Wings` (ARG) and `Champion of Tyranny` (PU) are the two records
  // `feat_catalog.rs`'s `the_picker_offers_the_newly_ingested_books_records`
  // names by hand; both books were ingested after this mapper was written.
  { key: 'Angel Wings', category: 'General', name: 'Angel Wings', description: 'You have a pair of feathered wings.', source: 'Arg', chooserTargetKind: null },
  { key: 'Champion of Tyranny', category: 'Alignment', name: 'Champion of Tyranny', description: null, source: 'Pu', chooserTargetKind: null },
];

/**
 * The picker's search box filters an already-loaded entry list client-side
 * (the backend `list_equipment`/`list_spells` filter narrows the initial
 * load by category/school; the search box narrows further by name/detail).
 */
function verifiesFilterMatchesEntryNameCaseInsensitively() {
  const entries = mapEquipmentCatalogEntries(EQUIPMENT_ENTRIES);
  const result = filterItemPickerEntries(entries, 'LONGSWORD');
  assertEqual(result.length, 1, 'one entry matches a case-insensitive name search');
  assertEqual(result[0].key, 'equipment:longsword', 'the matching entry is the longsword');
}

function verifiesFilterMatchesEntryDetailToo() {
  const entries = mapEquipmentCatalogEntries(EQUIPMENT_ENTRIES);
  const result = filterItemPickerEntries(entries, 'magic items');
  assertEqual(result.length, 1, 'one entry matches a detail-field search');
  assertEqual(result[0].key, 'equipment:potion_of_cure_light_wounds', 'the matching entry is the potion');
}

function verifiesEmptySearchReturnsEveryEntry() {
  const entries = mapEquipmentCatalogEntries(EQUIPMENT_ENTRIES);
  const result = filterItemPickerEntries(entries, '   ');
  assertEqual(result.length, entries.length, 'a blank search returns every entry unfiltered');
}

function verifiesNoMatchesReturnsEmptyArray() {
  const entries = mapEquipmentCatalogEntries(EQUIPMENT_ENTRIES);
  const result = filterItemPickerEntries(entries, 'nonexistent-item-xyz');
  assertEqual(result.length, 0, 'a search with no matches returns an empty list, not a fallback');
}

function verifiesEquipmentMappingUsesFriendlyCategoryLabel() {
  const [mapped] = mapEquipmentCatalogEntries([EQUIPMENT_ENTRIES[0]]);
  assertEqual(mapped.name, 'Longsword', 'name comes from the catalog entry name');
  assertEqual(
    mapped.detail,
    'Arms & Armor · This sword is about 3-1/2 feet in length.',
    'detail is the friendly category label followed by the record’s real corpus description'
  );
}

function verifiesEquipmentMappingFallsBackToRawCategoryForUnknownVariant() {
  const [mapped] = mapEquipmentCatalogEntries([{ key: 'equipment:mystery', category: 'SomeNewCategory', name: 'Mystery Item', costGp: null, book: 'CRB', description: null }]);
  assertEqual(mapped.detail, 'SomeNewCategory', 'unmapped categories fall back to the raw variant string, never a fabricated label');
}

/**
 * The hop this test exists for: `equipment_catalog.rs` renders a real
 * `description` and, until now, `mapEquipmentCatalogEntries` dropped it on
 * the floor. A player opening Add Item saw a bare category label.
 */
function verifiesEquipmentMappingCarriesTheCorpusDescriptionOntoTheRow() {
  const mapped = mapEquipmentCatalogEntries(EQUIPMENT_ENTRIES);
  assert(
    mapped[0].detail.includes('This sword is about 3-1/2 feet in length.'),
    'the longsword row carries its corpus description'
  );
  assert(
    mapped[2].detail.includes('Cures 1d8+1 damage.'),
    'the potion row carries its corpus description'
  );
}

/** A record the corpus gives no description gets no separator and no filler. */
function verifiesEquipmentMappingOmitsADescriptionTheCorpusDoesNotHave() {
  const [, bandedMail] = mapEquipmentCatalogEntries(EQUIPMENT_ENTRIES);
  assertEqual(
    bandedMail.detail,
    'Arms & Armor',
    'a record with no corpus description shows only its category — no dangling separator, no invented text'
  );
}

/** Blank-but-present prose is the same absence as `null`, treated the same. */
function verifiesEquipmentMappingTreatsBlankDescriptionAsAbsent() {
  const [mapped] = mapEquipmentCatalogEntries([
    { key: 'equipment:blank', category: 'General', name: 'Blank', costGp: 1, book: 'CRB', description: '   ' },
  ]);
  assertEqual(mapped.detail, 'General', 'whitespace-only description is treated as no description');
}

/** The search box reaches description text, not only name and category. */
function verifiesFilterOverEquipmentEntriesMatchesByDescription() {
  const entries = mapEquipmentCatalogEntries(EQUIPMENT_ENTRIES);
  const result = filterItemPickerEntries(entries, '3-1/2 feet');
  assertEqual(result.length, 1, 'one entry matches a description-text search');
  assertEqual(result[0].key, 'equipment:longsword', 'the matching entry is the longsword');
}

/**
 * The bound, on both sides. Short prose is returned untouched — no ellipsis
 * it did not earn — and long prose is cut on a word boundary and marked.
 */
function verifiesDescriptionSummaryIsBoundedAndMarksItsOwnTruncation() {
  const short = 'This sword is about 3-1/2 feet in length.';
  assertEqual(
    summariseItemDescription(short),
    short,
    'prose already inside the bound is returned byte-for-byte, with no truncation mark'
  );

  const long = `${'word '.repeat(80)}end`;
  const summary = summariseItemDescription(long);
  assert(
    summary.length <= ITEM_PICKER_DESCRIPTION_MAX_CHARS + 1,
    `summary stays inside the bound (was ${summary.length})`
  );
  assert(summary.endsWith('…'), 'a truncated summary announces itself with an ellipsis');
  assert(!summary.includes('  '), 'the summary is cut on a word boundary, not mid-word');

  // A newline-bearing description collapses onto one line for the row.
  assertEqual(
    summariseItemDescription('First line.\n\nSecond line.'),
    'First line. Second line.',
    'newlines collapse to spaces for a single-line picker row'
  );

  // A single unbroken token longer than the bound still gets bounded rather
  // than escaping it for want of a space to cut on.
  const unbroken = 'x'.repeat(400);
  const cut = summariseItemDescription(unbroken);
  assert(
    cut.length <= ITEM_PICKER_DESCRIPTION_MAX_CHARS + 1,
    `an unbroken token is still bounded (was ${cut.length})`
  );
}

function verifiesSpellMappingUsesKeyAsNameAndCombinesBookSchoolAndLevel() {
  const [mapped] = mapSpellCatalogEntries([SPELL_ENTRIES[1]]);
  assertEqual(mapped.key, 'spell:fireball', 'key is preserved');
  assertEqual(mapped.name, 'spell:fireball', 'the spell catalog has no separate name field, so key doubles as the display name');
  assert(mapped.detail.includes('CRB'), 'detail names the book the spell comes from');
  assert(mapped.detail.includes('Evocation'), 'detail includes the school');
  assert(mapped.detail.includes('3'), 'detail includes the spell level');
  // The catalog record's level is the minimum across every class on the
  // record, and this picker browses every class at once, so the number is
  // labelled for what it is rather than presented as the reader's own
  // class's level. The Spells tab, which does know a row's class, shows
  // that class's real level instead (see `spellsTabModel.ts`).
  assertEqual(
    mapped.detail,
    'CRB · Evocation · Lowest class level 3',
    'a cross-class browse labels the level as the record minimum, never a bare "Level N"'
  );
}

function verifiesSpellMappingOmitsSchoolAndLevelTheCorpusDoesNotHave() {
  const [mapped] = mapSpellCatalogEntries([SPELL_ENTRIES[3]]);
  assertEqual(
    mapped.detail,
    'APG',
    'a record whose corpus row has no school or level shows only its book, never a fabricated school/level'
  );
}

function verifiesFilterOverSpellEntriesMatchesBySchool() {
  const entries = mapSpellCatalogEntries(SPELL_ENTRIES);
  const result = filterItemPickerEntries(entries, 'conjuration');
  assertEqual(result.length, 1, 'one spell matches a school search');
  assertEqual(result[0].key, 'spell:cure_light_wounds', 'the matching entry is the conjuration spell');
}

/**
 * The feat catalog spans CRB, APG, ACG, ARG and PU, so a feat row has to
 * name its book the same way a spell row does — otherwise a player cannot
 * tell "Extra Hex" (APG) from a core feat.
 */
function verifiesFeatMappingLeadsWithTheBookThenCategoryThenDescription() {
  const [mapped] = mapFeatCatalogEntries([FEAT_ENTRIES[1]]);
  assertEqual(mapped.key, 'Extra Hex', 'key is preserved');
  assertEqual(mapped.name, 'Extra Hex', 'name is preserved');
  assertEqual(
    mapped.detail,
    'APG · General · You have learned the secrets of a new hex.',
    'detail leads with the book, then the category, then the real corpus description'
  );
}

function verifiesFeatMappingOmitsADescriptionTheCorpusDoesNotHave() {
  const [mapped] = mapFeatCatalogEntries([FEAT_ENTRIES[2]]);
  assertEqual(
    mapped.detail,
    'APG · Combat',
    'a record whose corpus row has no DESC: shows only book and category, never fabricated text'
  );
}

/**
 * Every book `list_feat_catalog` can actually serve needs a label. Its
 * `source` is a `RuleSetId` variant name (`"Arg"`, `"Pu"`), not the book
 * code a player recognises, and 204 of the catalog's 690 records are ARG
 * (187) or PU (17) — pinned by `feat_catalog.rs`'s
 * `catalog_spans_every_ingested_book_with_their_real_counts`. Without a
 * label those rows reach the picker as a raw wire variant sitting beside
 * properly-coded CRB/APG/ACG rows.
 */
function verifiesFeatMappingLabelsEveryBookTheCatalogActuallyServes() {
  const [arg] = mapFeatCatalogEntries([FEAT_ENTRIES[4]]);
  assertEqual(
    arg.detail,
    'ARG · General · You have a pair of feathered wings.',
    'an Advanced Race Guide feat names its book as ARG, not the raw RuleSetId variant "Arg"'
  );
  const [pu] = mapFeatCatalogEntries([FEAT_ENTRIES[5]]);
  assertEqual(
    pu.detail,
    'PU · Alignment',
    'a Pathfinder Unchained feat names its book as PU, not the raw RuleSetId variant "Pu"'
  );
}

function verifiesFeatMappingFallsBackToTheRawBookForAnUnknownVariant() {
  // Deliberately synthetic: `RuleSetId` already names `Um` as a future
  // variant, and this pins that a book this frontend has no label for
  // still renders its raw variant string rather than a blank or a
  // fabricated label. Mirrors
  // `verifiesEquipmentMappingFallsBackToRawCategoryForUnknownVariant`.
  const [mapped] = mapFeatCatalogEntries([
    { key: 'Some Future Feat', category: 'General', name: 'Some Future Feat', description: null, source: 'Um', chooserTargetKind: null },
  ]);
  assertEqual(
    mapped.detail,
    'Um · General',
    'an unknown/future book falls back to the raw RuleSetId variant rather than a fabricated label'
  );
}

function verifiesFilterOverFeatEntriesMatchesByBook() {
  const entries = mapFeatCatalogEntries(FEAT_ENTRIES);
  const result = filterItemPickerEntries(entries, 'acg');
  assertEqual(result.length, 1, 'one feat matches an ACG book search');
  assertEqual(result[0].key, 'Extra Panache', 'the matching entry is the ACG panache feat');
}

/**
 * The Feats tab's caption used to read "Add feats from the real CRB feat
 * catalog" long after four more books landed. The replacement is derived from
 * the response so it cannot go stale the same way; these pin that it counts
 * what it was actually handed.
 */
function verifiesCatalogCoverageNamesEveryBookInTheResponse() {
  const sentence = describeFeatCatalogCoverage(FEAT_ENTRIES);
  assertEqual(
    sentence,
    'Add feats from the real feat catalog: 6 feats across 5 books (CRB, APG, ACG, ARG, PU).',
    'the caption counts the records it was handed and names each book once, in response order'
  );
}

function verifiesCatalogCoverageFallsBackToTheRawBookForAnUnknownVariant() {
  const sentence = describeFeatCatalogCoverage([
    { key: 'Future Feat', category: 'General', name: 'Future Feat', description: null, source: 'Um', chooserTargetKind: null },
  ]);
  assertEqual(
    sentence,
    'Add feats from the real feat catalog: 1 feat across 1 book (Um).',
    'an unknown/future book is still counted, under its raw RuleSetId variant'
  );
}

function verifiesCatalogCoverageRefusesToDescribeAnEmptyResponse() {
  assertEqual(
    describeFeatCatalogCoverage([]),
    null,
    'an empty response means the catalog failed to load or is empty; the caption must not guess which'
  );
}

function main() {
  verifiesFilterMatchesEntryNameCaseInsensitively();
  verifiesFilterMatchesEntryDetailToo();
  verifiesEmptySearchReturnsEveryEntry();
  verifiesNoMatchesReturnsEmptyArray();
  verifiesEquipmentMappingUsesFriendlyCategoryLabel();
  verifiesEquipmentMappingFallsBackToRawCategoryForUnknownVariant();
  verifiesEquipmentMappingCarriesTheCorpusDescriptionOntoTheRow();
  verifiesEquipmentMappingOmitsADescriptionTheCorpusDoesNotHave();
  verifiesEquipmentMappingTreatsBlankDescriptionAsAbsent();
  verifiesFilterOverEquipmentEntriesMatchesByDescription();
  verifiesDescriptionSummaryIsBoundedAndMarksItsOwnTruncation();
  verifiesSpellMappingUsesKeyAsNameAndCombinesBookSchoolAndLevel();
  verifiesSpellMappingOmitsSchoolAndLevelTheCorpusDoesNotHave();
  verifiesFilterOverSpellEntriesMatchesBySchool();
  verifiesFeatMappingLeadsWithTheBookThenCategoryThenDescription();
  verifiesFeatMappingOmitsADescriptionTheCorpusDoesNotHave();
  verifiesFeatMappingLabelsEveryBookTheCatalogActuallyServes();
  verifiesFeatMappingFallsBackToTheRawBookForAnUnknownVariant();
  verifiesFilterOverFeatEntriesMatchesByBook();
  verifiesCatalogCoverageNamesEveryBookInTheResponse();
  verifiesCatalogCoverageFallsBackToTheRawBookForAnUnknownVariant();
  verifiesCatalogCoverageRefusesToDescribeAnEmptyResponse();
}

main();
