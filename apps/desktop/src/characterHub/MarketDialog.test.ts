import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import type { MarketEntryDto } from '../boundary/listMarket';
import { assert, assertEqual } from '../testSupport/asserts';
import {
  filterItemPickerEntries,
  formatMarketPrice,
  mapMarketEntries,
  marketActiveState,
  marketMoneyMoved,
  marketPriceGp,
  marketShownRows,
  MARKET_ROW_LIMIT,
} from './itemPickerFilter';
import { MARKET_CATEGORIES, MarketDialogView, type MarketDialogViewProps } from './MarketDialog';

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

// A weapon goes in the pack, and the player takes it in hand from the Weapons tab.
for (const category of MARKET_CATEGORIES) {
  assertEqual(
    marketActiveState(category.value),
    category.value === 'weapon' ? 'SelectedInactive' : 'EquippedActive',
    `${category.value} active state`,
  );
}

// What the status line says came from two balances the backend returned.
{
  assertEqual(marketMoneyMoved(300000, 68500), 2315, 'a buy of 231,500 cp reads as gp');
  assertEqual(marketMoneyMoved(68500, 184250), 1157.5, 'a sale reads as gp, whichever way it moved');
  assertEqual(marketMoneyMoved(100, 100), 0, 'cashless moved nothing');
}

// A long list renders a bounded window and says so.
{
  const many = Array.from({ length: MARKET_ROW_LIMIT + 5 }, (_, index) => ({ ...SWORD, key: `k${index}`, name: `n${index}` }));
  const shown = marketShownRows(mapMarketEntries(many));
  assertEqual(shown.length, MARKET_ROW_LIMIT, 'bounded');
  assert(
    renderToStaticMarkup(createElement(MarketDialogView, { ...BASE, rows: mapMarketEntries(many) })).includes(`Showing the first ${MARKET_ROW_LIMIT} of ${many.length}.`),
    'the dialog says the list is cut',
  );
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
  assert(html.includes('aria-label="Equipment pricing"'), 'the price mode control is reused');
  assert(html.includes('name="market-price-mode"'), 'its radios are not the sheet tab\'s group');
  assert(html.includes('2,000 gp'), 'the balance carries the currency label');
  assert(/<button[^>]*disabled=""[^>]*>Buy<\/button>/.test(html), 'Buy waits for a pick');
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
  assert(sheet.includes('handleRemoveEquipment(copy.itemId, copy.bonus)'), 'Sell names the copy');
  assert(sheet.includes('listHeldEquipment('), 'the Sell list is every held copy, not the demo corpus\'s equippedItems');
}

console.log('MarketDialog.test.ts: all assertions passed');
