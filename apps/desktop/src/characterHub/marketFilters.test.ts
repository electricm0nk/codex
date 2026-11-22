import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import type { MarketCategory } from '../boundary/listMarket';
import { assert, assertEqual } from '../testSupport/asserts';
import { applyMarketFilters, filterOptions, filterValuesOf, MARKET_FILTERS, OTHER, type FilterEntry } from './marketFilters';

// E0.2's per-list shown rows, written by `cargo test --test market_counts` from `list_market`.
const artifacts = join(dirname(fileURLToPath(import.meta.url)), '../../../../docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_0');
type Row = FilterEntry & { list: string; key: string };
const rows: Row[] = readFileSync(join(artifacts, 'market-rows.jsonl'), 'utf8')
  .split('\n')
  .filter((line) => line.length > 0)
  .map((line) => JSON.parse(line) as Row);
const counts = JSON.parse(readFileSync(join(artifacts, 'market-counts.json'), 'utf8')) as { lists: Array<{ list: string; shown: number }> };

const LISTS: Array<[MarketCategory, string]> = [
  ['weapon', 'Weapons'],
  ['armor', 'Armor'],
  ['shield', 'Shields'],
  ['gear', 'Gear'],
  ['magicItem', 'Magic'],
];

for (const [category, list] of LISTS) {
  const listRows = rows.filter((row) => row.list === list);
  const shown = counts.lists.find((entry) => entry.list === list)?.shown;
  assertEqual(listRows.length, shown, `${list}: rows in market-rows.jsonl vs E0.2's shown count`);
  assertEqual(applyMarketFilters(category, listRows, {}).length, listRows.length, `${list}: All returns every row`);

  for (const col of MARKET_FILTERS[category]) {
    const options = filterOptions(col, listRows);
    const sum = options.reduce((total, option) => total + option.count, 0);
    // Two-tag rule: a row counts once under each value it carries, so the sum is >= the rows.
    assert(sum >= listRows.length, `${list}/${col.label}: value counts + Other ${sum} < rows ${listRows.length}`);
    // Every row is under some option, counted independently of filterOptions.
    const unmatched = listRows.filter((row) => !col.values.some((value) => value.matches(row)));
    assertEqual(options.find((o) => o.label === OTHER)?.count ?? 0, unmatched.length, `${list}/${col.label}: Other = rows no value matches`);
    const reached = new Set<string>();
    for (const option of options) {
      const picked = applyMarketFilters(category, listRows, { [col.label]: option.label });
      assertEqual(picked.length, option.count, `${list}/${col.label}=${option.label}: filtered rows = its count`);
      picked.forEach((row) => reached.add(`${row.key}`));
    }
    assertEqual(reached.size, new Set(listRows.map((row) => row.key)).size, `${list}/${col.label}: every row reachable through a value or Other`);
    for (const row of listRows) assert(filterValuesOf(col, row).length > 0, `${list}/${col.label}: ${row.key} under no value`);
    // Confirmed against the tag counts: a tag value with no row on its list is a dead value. Gear's
    // kinds come from equipmentCategories.ts and are listed only when they have rows.
    if (category !== 'gear') {
      for (const value of col.values) {
        assert(options.some((o) => o.label === value.label), `${list}/${col.label}: value ${value.label} has no row`);
      }
    }
    console.log(`${list}\t${listRows.length}\t${col.label}\tsum ${sum}\t${options.map((o) => `${o.label} ${o.count}`).join(', ')}`);
  }
}

// A piercing-and-slashing weapon is under both.
const weapons = rows.filter((row) => row.list === 'Weapons');
const both = weapons.find((row) => row.key === 'Halberd (Base)' && row.types.includes('Piercing') && row.types.includes('Slashing'));
assert(both !== undefined, 'a Weapons row tagged Piercing and Slashing');
const damage = MARKET_FILTERS.weapon.find((col) => col.label === 'Damage')!;
assertEqual(filterValuesOf(damage, both!).join(','), 'Piercing,Slashing', `${both!.key}: Damage values`);
for (const label of ['Piercing', 'Slashing']) {
  assert(applyMarketFilters('weapon', weapons, { Damage: label }).some((row) => row.key === both!.key), `${both!.key} under ${label}`);
}

// Columns combine (AND); `null` is All.
const melee = applyMarketFilters('weapon', weapons, { Kind: 'Melee', Damage: null });
const meleePiercing = applyMarketFilters('weapon', weapons, { Kind: 'Melee', Damage: 'Piercing' });
assert(meleePiercing.length > 0 && meleePiercing.length < melee.length, 'Kind and Damage combine');
assert(meleePiercing.every((row) => row.types.includes('Melee') && row.types.includes('Piercing')), 'combined rows carry both tags');

// `SLOT_` tags read as the slot.
const slot = MARKET_FILTERS.magicItem.find((col) => col.label === 'Slot')!;
assertEqual(filterValuesOf(slot, { category: 'MagicItems', types: ['Magic', 'SLOT_Neck'] }).join(','), 'Neck', 'SLOT_Neck reads Neck');

console.log(`marketFilters.test.ts: ok (${rows.length} rows, piercing-and-slashing: ${both!.key})`);
