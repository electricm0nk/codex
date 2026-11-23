import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import type { MarketEntryDetailDto } from '../boundary/describeMarketEntry';
import type { MarketEntryDto } from '../boundary/listMarket';
import { assert, assertEqual } from '../testSupport/asserts';
import {
  filterItemPickerEntries,
  formatMarketPrice,
  mapMarketEntries,
  marketPlacementWords,
  marketMoneyMoved,
  marketPriceGp,
  marketFilterColumns,
  marketShownRows,
  marketWindow,
  MARKET_ROW_PX,
} from './itemPickerFilter';
import { MARKET_CATEGORIES, MarketDialogView, sellValueGp, rowOfHeldCopy, type MarketDealsProps, type MarketDialogViewProps } from './MarketDialog';
import { mapMarketTableRows } from './marketModel';
import { buildOf, BuyCustomDialogView, EMPTY_BUY_CUSTOM } from './BuyCustomDialog';
import { createItemFields, CreateItemDialogView, EMPTY_CREATE_ITEM } from './CreateItemDialog';
import type { SizeCategory } from '../boundary/listCustomEquipmentOptions';

const SWORD: MarketEntryDto = {
  key: 'Longsword (Base)',
  name: 'Longsword (Base)',
  category: 'ArmsArmor',
  costGp: 15,
  weightLbs: 4,
  description: null,
  types: ['Weapon', 'Melee'],
  variants: [
    { bonus: 1, name: '+1 Longsword (Base)', priceGp: 2315, modifierKey: 'Special Ability ~ +1 ~ Weapon' },
    { bonus: 2, name: '+2 Longsword (Base)', priceGp: 8315, modifierKey: 'Special Ability ~ +2 ~ Weapon' },
  ],
} as MarketEntryDto;
const ROPE: MarketEntryDto = { ...SWORD, key: 'Rope (Hemp)', name: 'Rope (Hemp)', category: 'General', costGp: null, variants: [] };

const BASE: MarketDialogViewProps = {
  category: 'armor',
  onCategory: () => {},
  search: '',
  onSearch: () => {},
  rows: mapMarketEntries([SWORD, ROPE]),
  loading: false,
  loadError: null,
  selectedKey: null,
  onSelectRow: () => {},
  bonus: 0,
  onBonus: () => {},
  currency: 'gp',
  balance: 2000,
  onBuy: () => {},
  inventory: [{ itemId: 'Longsword (Base)', name: '+1 Longsword', bonus: 1 }],
  onSell: () => {},
  priceMode: 'standard',
  onPriceModeChange: () => {},
  busy: false,
  status: null,
  error: null,
  onClose: () => {},
};
const render = (overrides: Partial<MarketDialogViewProps>) => renderToStaticMarkup(createElement(MarketDialogView, { ...BASE, ...overrides }));

// The row's own price and the offers cross unchanged; the detail line is the catalog picker's.
{
  const [row, rope] = mapMarketEntries([SWORD, ROPE]);
  assertEqual(row.key, 'Longsword (Base)', 'the row buys the base key');
  assertEqual(row.priceGp, 15, 'the plain price is the catalog price');
  assertEqual(row.variants.length, 2, 'the offers stay on the row');
  assert(row.detail.includes('15 gp'), 'the detail line is the catalog picker line');
  assertEqual(rope.priceGp, null, 'no catalog price stays absent, never 0');
  assertEqual(filterItemPickerEntries([row, rope], 'rope').length, 1, 'the picker search narrows Market rows');
  assertEqual(mapMarketEntries([ROPE, SWORD])[0].key, 'Longsword (Base)', 'the list reads by name, not in catalog order');
}

// The price shown is the offer's, whole; nothing here adds an enhancement to a base price.
{
  const [row] = mapMarketEntries([SWORD]);
  assertEqual(marketPriceGp(row, 0), 15, 'plain');
  assertEqual(marketPriceGp(row, 2), 8315, '+2 is the offer price');
  assertEqual(marketPriceGp(row, 5), null, 'a bonus not on offer has no price');
  assertEqual(formatMarketPrice(2315, 'gp'), '2,315 gp', 'thousands and the currency label');
  assertEqual(formatMarketPrice(0.5, 'gp'), '0.5 gp', 'a fraction stays');
  assertEqual(formatMarketPrice(2315, 'credits'), '2,315 credits', 'the label is the caller\'s');
  assertEqual(formatMarketPrice(null, 'gp'), 'No listed price', 'no price is said, not zero');
}

// The status line says where the buy went, and the backend's refusal in its own words.
assertEqual(marketPlacementWords('EquippedActive', null), 'worn', 'its slot was free');
assertEqual(
  marketPlacementWords('SelectedInactive', 'head slot is taken by Hat of Disguise'),
  'carried: head slot is taken by Hat of Disguise',
  'its slot was held',
);
assertEqual(marketPlacementWords('SelectedInactive', null), 'carried', 'an item with no slot refuses nothing');

// What the status line says came from two balances the backend returned.
{
  assertEqual(marketMoneyMoved(300000, 68500), 2315, 'a buy of 231,500 cp reads as gp');
  assertEqual(marketMoneyMoved(68500, 184250), 1157.5, 'a sale reads as gp, whichever way it moved');
  assertEqual(marketMoneyMoved(100, 100), 0, 'cashless moved nothing');
}

// No cap: every row is shown (the list is windowed, not cut), and the header says N of N.
{
  const many = Array.from({ length: 3680 }, (_, index) => ({ ...SWORD, key: `k${index}`, name: `n${String(index).padStart(4, '0')}` }));
  const rows = mapMarketEntries(many);
  assertEqual(marketShownRows(rows, 'weapon', {}).length, 3680, 'every row is shown');
  const html = renderToStaticMarkup(createElement(MarketDialogView, { ...BASE, rows, totalRows: rows.length }));
  assert(!html.includes('Showing the first'), 'no cut notice');
  assert(html.includes('data-testid="market-count">3,680 of 3,680</'), 'header N of N');
  // Windowed: only the rows in view are mounted, the rest are spacer height.
  const mounted = (html.match(/aria-selected=/g) ?? []).length;
  assert(mounted > 0 && mounted < 200, `a window of rows is mounted (${mounted})`);
  assert(html.includes('n0000') && !html.includes('n3679'), 'the first rows, not the last');
  const end = marketWindow(3680, 3679 * MARKET_ROW_PX, 600);
  assertEqual(end.end, 3680, 'scrolled to the bottom, the last row is in the window');
  assert(end.start > 3600, 'and the first rows are not');
  const start = marketWindow(3680, 0, 600);
  assertEqual(start.start, 0, 'top');
  assertEqual(marketWindow(3, 0, 600).end, 3, 'a short list mounts every row');
  // Filtering on a column narrows the shown rows; the header counts what is shown.
  const melee = { ...SWORD, key: 'Club', name: 'Club', types: ['Weapon', 'Melee', 'Bludgeoning'] };
  const bow = { ...SWORD, key: 'Longbow', name: 'Longbow', types: ['Weapon', 'Ranged', 'Piercing'] };
  const two = mapMarketEntries([melee, bow]);
  assertEqual(marketShownRows(two, 'weapon', { Kind: 'Ranged' }).map((row) => row.key).join(','), 'Longbow', 'a filter value narrows');
  assertEqual(marketShownRows(two, 'weapon', { Kind: null }).length, 2, 'All is every row');
}

// The filter column: one block per column with All and each value with its count; the chosen value is pressed.
{
  const club = { ...SWORD, key: 'Club', name: 'Club', types: ['Weapon', 'Melee', 'Bludgeoning'] };
  const bow = { ...SWORD, key: 'Longbow', name: 'Longbow', types: ['Weapon', 'Ranged', 'Piercing'] };
  const rows = mapMarketEntries([club, bow]);
  const html = renderToStaticMarkup(
    createElement(MarketDialogView, {
      ...BASE,
      category: 'weapon',
      rows: marketShownRows(rows, 'weapon', { Kind: 'Ranged' }),
      totalRows: 2,
      filterColumns: marketFilterColumns('weapon', rows, { Kind: 'Ranged' }),
      onFilter: () => {},
    }),
  );
  assert(html.includes('aria-label="Filters"'), 'the filter column');
  assert(html.includes('>Kind</'), 'a Kind column');
  assert(html.includes('>Melee (1)</button>') && html.includes('>Ranged (1)</button>'), 'values with counts');
  assert(/aria-pressed="true"[^>]*>Ranged \(1\)/.test(html), 'the chosen value is pressed');
  assert(html.includes('>All (2)</button>'), 'All with every row');
  assert(html.includes('data-testid="market-count">1 of 2</'), 'header counts the shown rows');
}

// Tabs, search, inventory Sell, the reused price control under its own radio group.
{
  const html = render({});
  for (const { label } of MARKET_CATEGORIES) {
    assert(html.includes(`>${label}</button>`), `the ${label} category tab`);
  }
  assert(html.includes('aria-selected="true"') && html.includes('role="tab"'), 'the current category is marked');
  assert(html.includes('placeholder="Search the market…"'), 'search box');
  assert(html.includes('>Sell +1 Longsword</button>'), 'an inventory row has a Sell action naming the copy');
  const away = render({ inventory: [{ itemId: 'Dagger (Base)', name: 'Dagger', bonus: 0, activeState: 'Absent' }, { itemId: 'Dagger (Base)', name: 'Dagger', bonus: 0, activeState: 'SelectedInactive' }] });
  assert(away.includes('>Sell Dagger (left behind)</button>'), 'a copy left behind can be sold, and two copies read apart');
  assert(away.includes('>Sell Dagger</button>'), 'a copy in hand reads as it always did');
  assert(html.includes('aria-label="Equipment pricing"'), 'the price mode control is reused');
  assert(html.includes('name="market-price-mode"'), 'its radios are not the sheet tab\'s group');
  assert(html.includes('2,000 gp'), 'the balance carries the currency label');
  assert(/<button[^>]*disabled=""[^>]*>Buy<\/button>/.test(html), 'Buy waits for a pick');
}

// E1.8: the inventory row's value cell is the engine's sell rule on the whole stack, under the live price option.
{
  const [row] = mapMarketEntries([{ ...SWORD, costGp: 10 } as MarketEntryDto]);
  const stack = { itemId: 'Longsword (Base)', name: 'Longsword (Base)', bonus: 0, quantity: 3, baseQuantity: 1 };
  assertEqual(sellValueGp(row, stack, 'standard'), 15, 'a 3-stack of 10 gp sells for 50 % of 30 gp');
  assertEqual(sellValueGp(row, stack, 'cashless'), 0, 'cashless sells for nothing');
  assertEqual(sellValueGp(null, stack, 'standard'), null, 'an unlisted copy has no value');
  const view = (mode: 'standard' | 'cashless') =>
    render({ priceMode: mode, inventory: [stack], sellValue: (copy) => sellValueGp(row, copy, mode) });
  assert(/data-testid="market-sell-value"[^>]*>15 gp<\/td>/.test(view('standard')), 'standard shows the 50 % total');
  assert(/data-testid="market-sell-value"[^>]*>\u2014<\/td>/.test(view('cashless')), 'cashless shows a dash');
  assert(/data-testid="market-sell-value"[^>]*>\u2014<\/td>/.test(render({ inventory: [stack], sellValue: () => null })), 'no price shows a dash');
}

// A starter character holds the legacy id `item:longsword`, which is the Market's `Longsword (Base)`; a
// copy must find its row by that id or the value cell reads a dash (seen live in the E1.8 repair).
{
  const rows = mapMarketEntries([{ ...SWORD, costGp: 15, name: 'Longsword' } as MarketEntryDto]);
  const legacy = { itemId: 'item:longsword', name: 'Longsword', bonus: 0 };
  assert(rowOfHeldCopy(rows, legacy) === rows[0], 'a legacy item: id finds its Market row');
  assert(rowOfHeldCopy(rows, { ...legacy, itemId: 'item:longspear', name: 'Longspear' }) === null, 'another item does not');
  assert(rowOfHeldCopy(rows, { ...legacy, name: 'Mine' }) === null, 'a custom name does not');
}

// A pick offers its bonuses and prices the one chosen.
{
  const html = render({ selectedKey: 'Longsword (Base)', bonus: 2 });
  for (const label of ['Plain', '+1', '+2']) {
    assert(html.includes(`>${label}</button>`), `the ${label} choice`);
  }
  assert(!html.includes('>+3</button>'), 'only what the row offers');
  assert(html.includes('8,315 gp'), 'the price of the chosen bonus, with its currency');
  assert(!/<button[^>]*disabled=""[^>]*>Buy<\/button>/.test(html), 'Buy is live');
}

// A row with no offers has no picker at all (armor spikes, rope; and every SF/CPR row later).
{
  const html = render({ selectedKey: 'Rope (Hemp)' });
  assert(!html.includes('>Plain</button>'), 'no +N picker without an offer');
  assert(html.includes('No listed price'), 'an unpriced row says so');
}

// Refusals and outcomes are shown inside the modal, which covers the sheet's own error line.
{
  const html = render({ error: 'Not enough money', status: 'Bought it' });
  assert(html.includes('Not enough money') && html.includes('Bought it'), 'both reach the dialog');
  assert(render({ busy: true }).includes('disabled=""'), 'busy blocks a second charge');
  assert(render({ loadError: 'catalog broke' }).includes('catalog broke'), 'a load failure is said');
  assert(render({ rows: [], search: 'zzz' }).includes('No matches.'), 'an empty search says so');
  assert(render({ inventory: [] }).includes('Nothing held yet.'), 'an empty inventory says so');
  assert(!render({ inventory: null }).includes('Nothing held yet.'), 'an inventory still loading does not read as empty');
}

// The sheet: one Market button per tab; the three Add buttons are gone, in the source and its comments.
{
  const here = dirname(fileURLToPath(import.meta.url));
  const sheet = readFileSync(join(here, 'CharacterSheet.tsx'), 'utf8');
  assertEqual(sheet.match(/Add Weapon|Add Armor|Add Gear/g)?.length ?? 0, 0, 'the old names');
  assertEqual(sheet.match(/>\s*Market\s*</g)?.length, 3, 'a Market button on each of Weapons, Defense and Gear');
  for (const category of ['weapon', 'armor', 'gear']) {
    assert(sheet.includes(`onOpenMarket={() => setMarketCategory('${category}')}`), `a tab opens on ${category}`);
  }
  assert(sheet.includes('<MarketDialog'), 'the sheet renders the dialog');
  assert(sheet.includes('listMarket('), 'its rows are list_market\'s, not a bundled list');
  assert(sheet.includes('handleRemoveEquipment(copy.itemId, copy.bonus, copy.activeState, deal)'), 'Sell names the copy, with its quantity and any negotiated price');
  assert(sheet.includes('mapMarketTableRows(response.entries)'), 'the table rows carry weight, slot and description');
  assert(/takesDeals\s+onBuyCustom=\{handleMarketBuyCustom\}\s+onCreate=\{handleMarketCreate\}/.test(sheet), 'the sheet wires Quantity, Negotiate, Buy Custom and Create');
  assert(sheet.includes('purchaseCustomEquipment({') && sheet.includes('createCustomItem({'), 'Buy Custom and Create reach their commands');
  assert(sheet.includes('listHeldEquipment('), 'the Sell list is every held copy, not the demo corpus\'s equippedItems');
}

// SD-41 (`decisions.md §24–§26, §28`): the table, its sortable headers, the description box, Only Show
// Affordable, Quantity, Negotiate, Buy Custom and Create.
{
  const deals: MarketDealsProps = {
    quantity: '3',
    onQuantity: () => {},
    negotiation: null,
    negotiatedGp: '',
    onNegotiatedGp: () => {},
    onNegotiateBuy: () => {},
    onNegotiateSell: () => {},
    onConfirmNegotiation: () => {},
    onCancelNegotiation: () => {},
  };
  const rows = mapMarketTableRows([{ ...SWORD, slot: 'wield_main', description: 'A heavy blade.' } as MarketEntryDto, { ...ROPE, slot: 'none' } as MarketEntryDto]);
  const full = { rows, sort: { column: 'price' as const, direction: 'desc' as const }, onSort: () => {}, onlyAffordable: true, onOnlyAffordable: () => {}, deals, onBuyCustom: () => {}, onCreate: () => {} };
  const html = render({ ...full, selectedKey: 'Longsword (Base)' });
  for (const id of ['market-table', 'market-sort-item', 'market-sort-price', 'market-sort-weight', 'market-sort-slot', 'market-description', 'market-only-affordable', 'market-quantity', 'market-negotiate', 'market-buy-custom', 'market-create']) {
    assert(html.includes(`data-testid="${id}"`), `the ${id} control`);
  }
  for (const header of ['Item', 'Price', 'Weight', 'Slot']) {
    assert(html.includes(`>${header}`), `the ${header} column`);
  }
  assert(html.includes('Price<!-- --> ▼') || html.includes('Price ▼'), 'the sorted column says which way');
  assert(html.includes('4 lb') && html.includes('Main hand'), 'weight and slot cells');
  assert(html.includes('A heavy blade.'), 'the description box shows the highlighted row');
  // The box stays in view under the table: the table column and the inventory sit in one single-line
  // row, so the column takes the dialog's height and only the table's own list scrolls. A wrapping
  // row grew to the 200 rows' height and put the box below them, out of sight (sd41-e3.mc screenshot).
  const body = /<div data-testid="market-body" style="([^"]*)"/.exec(html);
  assert(body !== null, 'the table and inventory row is marked');
  assert(!body![1].includes('flex-wrap:wrap'), 'the table and inventory row does not wrap');
  assert(html.includes('checked=""'), 'Only Show Affordable reflects its state');
  assert(html.includes('3 × 15 gp = 45 gp'), 'the price beside Buy is the quantity total');
  assert(html.includes('>Negotiate</button>'), 'Negotiate on the inventory rows too');
  const unpicked = render(full);
  assert(unpicked.includes('Pick an item to read its description.'), 'no pick, no description');
  assert(/<button[^>]*data-testid="market-buy-custom"[^>]*disabled=""/.test(unpicked), 'Buy Custom needs a base row');
  const negotiating = render({ ...full, selectedKey: 'Rope (Hemp)', deals: { ...deals, negotiation: { kind: 'buy' } } });
  assert(negotiating.includes('aria-label="Negotiated price"') && negotiating.includes('required'), 'an unpriced row is bought at a price the player names');
  assert(/<button[^>]*disabled=""[^>]*>Buy at this price<\/button>/.test(negotiating), 'not until a price is named');
  // A system shell passes none of these: the dialog shows none of them.
  const shell = render({});
  for (const id of ['market-only-affordable', 'market-quantity', 'market-negotiate', 'market-buy-custom', 'market-create']) {
    assert(!shell.includes(`data-testid="${id}"`), `no ${id} without its handler`);
  }
  assert(shell.includes('data-testid="market-table"'), 'the table itself is every shell\'s');
}

// The builder and Create dialogs show the backend's quote and send what the player typed.
{
  const options = {
    abilities: [{ key: 'FLAMING', name: 'Flaming', description: null, plus: 1, unpriced: false }, { key: 'ODD', name: 'Odd', description: null, plus: 0, unpriced: true }],
    materials: [{ key: 'MITHRAL', name: 'Mithral', description: null, plus: 0, unpriced: false }],
    sizes: ['Small', 'Large'] as SizeCategory[],
  };
  const base = { baseName: 'Longsword', currency: 'gp', options, draft: { ...EMPTY_BUY_CUSTOM, quantity: '2' }, onDraft: () => {}, error: null, busy: false, onBuy: () => {}, onClose: () => {} };
  const priced = renderToStaticMarkup(createElement(BuyCustomDialogView, { ...base, quote: { modifiers: [], priceGp: 10015, unpriced: [], weightLbs: 2 } }));
  assert(priced.includes('Total: 2 × 10,015 gp = 20,030 gp'), 'the running total is the quote times the quantity');
  // A long material label (a requirement list) must not widen the select past the dialog (sd41-e3.mc screenshot).
  assert(/<select aria-label="Material"[^>]*style="[^"]*max-width:100%/.test(priced), 'the Material select fits the dialog');
  assert(priced.includes('Flaming (+1 equivalent)') && priced.includes('Odd (price: negotiate)') && priced.includes('Mithral') && priced.includes('Large'), 'abilities, materials and sizes from the options');
  const unpriced = renderToStaticMarkup(createElement(BuyCustomDialogView, { ...base, quote: { modifiers: [], priceGp: null, unpriced: ['ODD'], weightLbs: 4 } }));
  assert(unpriced.includes('price: negotiate (no price for ODD)'), 'an unpriced quote says so, and which part');
  assert(/<button[^>]*disabled=""[^>]*>Buy custom<\/button>/.test(unpriced), 'an unpriced build buys only at a named price');
  assertEqual(JSON.stringify(buildOf('Longsword (Base)', { ...EMPTY_BUY_CUSTOM, name: ' Dawn ', enhancement: 1, abilities: ['FLAMING'], material: 'MITHRAL' })), JSON.stringify({ baseItemId: 'Longsword (Base)', name: 'Dawn', masterwork: false, enhancement: 1, specialAbilities: ['FLAMING'], material: 'MITHRAL' }), 'the build the quote and the buy take');
  const fields = createItemFields({ ...EMPTY_CREATE_ITEM, name: 'Food', description: 'A pouch', weight: '0.5', container: true, capacity: '10', slot: 'belt' });
  assertEqual(JSON.stringify(fields), JSON.stringify({ name: 'Food', description: 'A pouch', weightLbs: 0.5, priceGp: 0, containerCapacityLbs: 10, slot: 'belt' }), 'Create sends name, description, weight, price 0, capacity, slot');
  assertEqual(createItemFields({ ...EMPTY_CREATE_ITEM, name: 'Rock', capacity: '10' }).containerCapacityLbs, null, 'no capacity without Container');
  const create = renderToStaticMarkup(createElement(CreateItemDialogView, { currency: 'gp', draft: EMPTY_CREATE_ITEM, onDraft: () => {}, error: null, busy: false, onCreate: () => {}, onClose: () => {} }));
  assert(create.includes('value="0"') && /<button[^>]*disabled=""[^>]*>Create item<\/button>/.test(create), 'price starts at 0; a name is required');
}

// SD-43 E1.9 (`decisions.md §13`): the description box shows the engine's stats, through the gear
// panel's renderer, stats first, then the prose; a stat the record does not state reads `—`.
{
  const field = (markup: string, id: string) => markup.match(new RegExp(`data-testid="${id}"[^>]*>([^<]*)<`))?.[1];
  const bastard: MarketEntryDetailDto = {
    entry: { key: 'Sword (Bastard)', category: 'ArmsArmor', name: 'Sword (Bastard)', costGp: 35, weightLbs: 6, book: 'CRB', types: ['Weapon', 'Melee'], description: 'Bastard swords are about 4 feet in length.', slot: 'wield_main' } as MarketEntryDetailDto['entry'],
    weapon: {
      weaponItemId: 'Sword (Bastard)', weaponRecordKey: 'Sword (Bastard)', baseDice: { count: 1, dieSize: 10 }, strDamageModifier: 0, wieldCategory: 'OneHanded',
      enhancementAttackBonus: null, enhancementDamageBonus: null, criticalThreatRange: [19, 20], criticalMultiplier: 2, rangeIncrement: null, featEffects: [],
      inHand: true, unarmed: false, nonlethal: false, usesAmmunition: false, ammunitionCarried: 0,
    },
    armor: null,
    container: null,
  };
  const sword = render({ selectedKey: 'Longsword (Base)', stats: bastard });
  assertEqual(field(sword, 'gear-detail-damage'), '1d10', 'the bastard sword states its die');
  assertEqual(field(sword, 'gear-detail-critical'), '19–20/×2', 'and its critical');
  assertEqual(field(sword, 'gear-detail-ac'), '—', 'a weapon states no armor bonus: a dash');
  assert(sword.indexOf('data-testid="gear-detail-damage"') < sword.indexOf('Bastard swords are about'), 'stats first, then the prose');
  assert(sword.includes('data-testid="market-description"') && sword.indexOf('data-testid="market-description"') < sword.indexOf('data-testid="gear-detail-damage"'), 'inside the description box');

  const plate: MarketEntryDetailDto = {
    entry: { key: 'Full Plate', category: 'ArmsArmor', name: '+2 Full Plate', costGp: 5650, weightLbs: 50, book: 'CRB', types: ['Armor', 'Heavy'], description: null, slot: 'armor' } as MarketEntryDetailDto['entry'],
    weapon: null,
    armor: { itemId: 'Full Plate', equipmentRecordKey: 'Full Plate', category: 'ArmsArmor', armorClassBonus: 9, armorClassEnhancement: 2, maxDex: 1, armorCheckPenalty: -6, spellFailure: 35 },
    container: null,
  };
  const armor = render({ selectedKey: 'Longsword (Base)', stats: plate });
  const expected: Array<[string, string]> = [
    ['gear-detail-name', '+2 Full Plate'],
    ['gear-detail-cost', '5650 gp'],
    ['gear-detail-ac', '+9'],
    ['gear-detail-enhancement', '+2'],
    ['gear-detail-max-dex', '+1'],
    ['gear-detail-acp', '-6'],
    ['gear-detail-asf', '35%'],
    ['gear-detail-damage', '—'],
    ['gear-detail-critical', '—'],
    ['gear-detail-description', '—'],
  ];
  for (const [id, text] of expected) {
    assertEqual(field(armor, id), text, `full plate: ${id}`);
  }
  // Until the stats arrive (and in a system shell, which reads none) the box keeps its prose.
  const prose = render({ selectedKey: 'Longsword (Base)' });
  assert(prose.includes('data-testid="market-description"') && !prose.includes('data-testid="gear-detail-damage"'), 'no stats: the prose alone');
  const sheet = readFileSync(join(dirname(fileURLToPath(import.meta.url)), 'CharacterSheet.tsx'), 'utf8');
  assert(sheet.includes('describeEntry={(row, bonus) => describeMarketEntry({ characterId: props.row.characterId, itemId: row.key, bonus })}'), 'the sheet reads the stats through the boundary');
}

console.log('MarketDialog.test.ts: all assertions passed');
