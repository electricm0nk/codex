import {
  filterItemPickerEntries,
  mapEquipmentCatalogEntries,
  mapFeatCatalogEntries,
  mapSpellCatalogEntries,
} from './itemPickerFilter';
import type { EquipmentCatalogEntryDto } from '../boundary/loadEquipmentCatalog';
import type { SpellCatalogEntryDto } from '../boundary/loadSpellCatalog';
import type { FeatCatalogEntryDto } from '../boundary/listFeats';
import { assert, assertEqual } from '../testSupport/asserts';

const EQUIPMENT_ENTRIES: EquipmentCatalogEntryDto[] = [
  { key: 'equipment:longsword', category: 'ArmsArmor', name: 'Longsword', costGp: 15, book: 'CRB' },
  { key: 'equipment:banded_mail', category: 'ArmsArmor', name: 'Banded Mail', costGp: 250, book: 'CRB' },
  { key: 'equipment:potion_of_cure_light_wounds', category: 'MagicItems', name: 'Potion of Cure Light Wounds', costGp: 50, book: 'CRB' },
];

const SPELL_ENTRIES: SpellCatalogEntryDto[] = [
  { key: 'spell:magic_missile', book: 'CRB', school: 'Evocation', level: 1, description: 'A missile of magical energy.' },
  { key: 'spell:fireball', book: 'CRB', school: 'Evocation', level: 3, description: 'A burst of flame.' },
  { key: 'spell:cure_light_wounds', book: 'CRB', school: 'Conjuration', level: 1, description: 'Heals wounds.' },
  // A real `apg_spells.lst` gap shape: resolves, but the corpus row
  // carries no SCHOOL:/CLASSES:/DESC: token.
  { key: 'spell:corpus_gap', book: 'APG', school: null, level: null, description: null },
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
  assertEqual(mapped.detail, 'Arms & Armor', 'detail is the friendly label for the ArmsArmor category');
}

function verifiesEquipmentMappingFallsBackToRawCategoryForUnknownVariant() {
  const [mapped] = mapEquipmentCatalogEntries([{ key: 'equipment:mystery', category: 'SomeNewCategory', name: 'Mystery Item', costGp: null, book: 'CRB' }]);
  assertEqual(mapped.detail, 'SomeNewCategory', 'unmapped categories fall back to the raw variant string, never a fabricated label');
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
 * The feat catalog spans CRB, APG and ACG since the APG/ACG ingest, so a
 * feat row has to name its book the same way a spell row does — otherwise
 * a player cannot tell "Extra Hex" (APG) from a core feat.
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

function main() {
  verifiesFilterMatchesEntryNameCaseInsensitively();
  verifiesFilterMatchesEntryDetailToo();
  verifiesEmptySearchReturnsEveryEntry();
  verifiesNoMatchesReturnsEmptyArray();
  verifiesEquipmentMappingUsesFriendlyCategoryLabel();
  verifiesEquipmentMappingFallsBackToRawCategoryForUnknownVariant();
  verifiesSpellMappingUsesKeyAsNameAndCombinesBookSchoolAndLevel();
  verifiesSpellMappingOmitsSchoolAndLevelTheCorpusDoesNotHave();
  verifiesFilterOverSpellEntriesMatchesBySchool();
  verifiesFeatMappingLeadsWithTheBookThenCategoryThenDescription();
  verifiesFeatMappingOmitsADescriptionTheCorpusDoesNotHave();
  verifiesFeatMappingFallsBackToTheRawBookForAnUnknownVariant();
  verifiesFilterOverFeatEntriesMatchesByBook();
}

main();
