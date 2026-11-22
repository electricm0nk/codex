import { assert, assertEqual } from '../testSupport/asserts';
import type { EquipmentCatalogEntryDto } from '../boundary/loadEquipmentCatalog';
import {
  filterItemPickerEntriesInGroup,
  itemPickerGroupCounts,
  mapEquipmentCatalogEntries,
  type ItemPickerEntry,
} from './itemPickerFilter';

const row = (key: string, name: string, group?: string): ItemPickerEntry => ({ key, name, detail: `${name} detail`, group });

const ENTRIES: ItemPickerEntry[] = [
  row('a', 'Longsword', 'Weapons'),
  row('b', 'Longbow', 'Weapons'),
  row('c', 'Wand of Light', 'Wands'),
  row('d', 'Wand of Longstride', 'Wands'),
  row('e', 'Mystery Box', 'Zebra Category'),
  row('f', 'Untyped Thing'),
];

function verifiesAllGroupsShowEverything() {
  assertEqual(filterItemPickerEntriesInGroup(ENTRIES, '', null).length, ENTRIES.length, 'no group, no search shows all rows');
}

function verifiesGroupNarrows() {
  const names = filterItemPickerEntriesInGroup(ENTRIES, '', 'Wands').map((e) => e.name);
  assertEqual(names.join('|'), 'Wand of Light|Wand of Longstride', 'only wands');
}

function verifiesSearchAndGroupCombine() {
  const names = filterItemPickerEntriesInGroup(ENTRIES, 'long', 'Weapons').map((e) => e.name);
  assertEqual(names.join('|'), 'Longsword|Longbow', 'search narrows within the group');
  assertEqual(filterItemPickerEntriesInGroup(ENTRIES, 'long', 'Wands').length, 1, 'only the wand that matches the search');
}

function verifiesCountsFollowTheSearchAndTheDisplayOrder() {
  const counts = itemPickerGroupCounts(ENTRIES, 'long');
  assertEqual(JSON.stringify(counts), JSON.stringify([
    { group: 'Weapons', count: 2 },
    { group: 'Wands', count: 1 },
  ]), 'counts reflect the search; groups with no matches are omitted');
  const all = itemPickerGroupCounts(ENTRIES, '').map((c) => c.group);
  assertEqual(all.join('|'), 'Weapons|Wands|Zebra Category|Uncategorized', 'display order first, unknown groups after, ungrouped rows as Uncategorized');
}

function verifiesUngroupedRowsAreReachableThroughUncategorized() {
  const names = filterItemPickerEntriesInGroup(ENTRIES, '', 'Uncategorized').map((e) => e.name);
  assertEqual(names.join('|'), 'Untyped Thing', 'a row with no group is found under Uncategorized');
}

function verifiesCatalogRowsCarryTheirCategory() {
  const dto = (over: Partial<EquipmentCatalogEntryDto>): EquipmentCatalogEntryDto => ({
    key: 'k', category: 'MagicItems', name: 'n', costGp: 1, weightLbs: 1, book: 'CRB', description: null, types: [], ...over,
  });
  const [wand, mod, untyped] = mapEquipmentCatalogEntries([
    dto({ name: 'Wand of Light', types: ['Magic', 'Wand'] }),
    dto({ name: 'Keen', category: 'Equipmods' }),
    dto({ name: 'Mystery', category: 'General' }),
  ]);
  assertEqual(wand.group, 'Wands', 'a typed row is grouped by its PCGen type');
  assertEqual(mod.group, 'Equipment Mods', 'a modifier is its own group');
  assertEqual(untyped.group, 'Uncategorized', 'an untyped row is Uncategorized');
  assert(wand.detail.length > 0, 'the detail line is still produced');
}

function verifiesAFailedTypeSidecarIsStatedOnTheRows() {
  const dto = { key: 'k', category: 'General', name: 'n', costGp: 1, weightLbs: 1, book: 'CRB', description: null, types: [] } as EquipmentCatalogEntryDto;
  const [row] = mapEquipmentCatalogEntries([dto], 'could not read data/equipment_types.json');
  assert((row.unverifiedNote ?? '').includes('could not read data/equipment_types.json'), 'the reason categories are missing is shown, not hidden');
  assertEqual(mapEquipmentCatalogEntries([dto])[0].unverifiedNote, undefined, 'no note when the sidecar loaded');
}

verifiesAllGroupsShowEverything();
verifiesGroupNarrows();
verifiesSearchAndGroupCombine();
verifiesCountsFollowTheSearchAndTheDisplayOrder();
verifiesUngroupedRowsAreReachableThroughUncategorized();
verifiesCatalogRowsCarryTheirCategory();
verifiesAFailedTypeSidecarIsStatedOnTheRows();
console.log('itemPickerGroups.test.ts: all assertions passed');
