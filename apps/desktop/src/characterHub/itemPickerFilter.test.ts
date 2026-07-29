import { filterItemPickerEntries, mapEquipmentCatalogEntries, mapSpellCatalogEntries } from './itemPickerFilter';
import type { SpellCatalogEntryDto } from '../boundary/loadSpellCatalog';
import { assert, assertEqual } from '../testSupport/asserts';

const EQUIPMENT_ENTRIES = [
  { key: 'equipment:longsword', category: 'ArmsArmor', name: 'Longsword', costGp: 15 },
  { key: 'equipment:banded_mail', category: 'ArmsArmor', name: 'Banded Mail', costGp: 250 },
  { key: 'equipment:potion_of_cure_light_wounds', category: 'MagicItems', name: 'Potion of Cure Light Wounds', costGp: 50 },
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
  const [mapped] = mapEquipmentCatalogEntries([{ key: 'equipment:mystery', category: 'SomeNewCategory', name: 'Mystery Item', costGp: null }]);
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
}

main();
