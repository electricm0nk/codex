import { readFileSync } from 'node:fs';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert, assertEqual } from '../testSupport/asserts';
import { MarketDialogView, type MarketDialogViewProps } from './MarketDialog';
import { copiesOfState, CYBERPUNK_MARKET_CATEGORIES, marketLine, rowsOfShell, STARFINDER_MARKET_CATEGORIES } from './SystemMarketDialog';

const source = (name: string) => readFileSync(new URL(name, import.meta.url), 'utf8');

const SHELL_ROWS = rowsOfShell([
  { key: 'core:equipment:battery', name: 'Battery', detail: 'Level: 1', price: 60 },
  { key: 'gear|Rope|', name: 'Rope', detail: '20eb', price: null },
]);

const BASE: MarketDialogViewProps = {
  category: 'weapon',
  onCategory: () => undefined,
  categories: CYBERPUNK_MARKET_CATEGORIES,
  search: '',
  onSearch: () => undefined,
  rows: SHELL_ROWS,
  loading: false,
  loadError: null,
  selectedKey: 'core:equipment:battery',
  onSelectRow: () => undefined,
  bonus: 0,
  onBonus: () => undefined,
  currency: 'eddies',
  balance: 1450,
  onBuy: () => undefined,
  inventory: copiesOfState({ currency: 'eddies', balance: 1450, held: [{ key: '0|Heavy Pistol', name: 'Heavy Pistol' }] }),
  onSell: () => undefined,
  priceMode: 'standard',
  onPriceModeChange: () => undefined,
  busy: false,
  status: null,
  error: null,
  onClose: () => undefined,
};

const html = renderToStaticMarkup(createElement(MarketDialogView, BASE));

// The same dialog, in the system's currency, off the system's own lists.
assert(html.includes('Balance: 1,450 eddies'), 'the balance carries the currency the backend named');
assert(html.includes('60 eddies'), 'the price carries it too');
assert(html.includes('Sell Heavy Pistol'), 'the inventory row sells its copy');
assert(html.includes('Cashless'), 'the price-mode control is the same one');
assertEqual(BASE.inventory?.[0].itemId, '0|Heavy Pistol', 'a copy names itself to the sale by its key');

// No +N picker: a shell row offers no variant, so the row has no Plain / +1 choice.
assert(!html.includes('Plain'), 'no Plain choice');
assert(!html.includes('>+1<'), 'no +1 choice');
assertEqual(SHELL_ROWS.every((row) => row.variants.length === 0), true, 'a shell row has no variants');

// The tabs a system's Market shows are its own.
assert(html.includes('Weapons') && html.includes('Shields'), 'Cyberpunk tabs');
assert(!html.includes('Magic items'), 'Cyberpunk RED has no magic items tab');
assertEqual(STARFINDER_MARKET_CATEGORIES.map((c) => c.value).join(), 'weapon,armor,shield,gear,magicItem', 'Starfinder tabs');
assertEqual(CYBERPUNK_MARKET_CATEGORIES.map((c) => c.value).join(), 'weapon,armor,shield,gear', 'Cyberpunk tabs');

// What the player is told, from what the backend moved.
assertEqual(marketLine('buy', 'Battery', 60, 'credits'), 'Bought Battery for 60 credits.', 'a paid buy');
assertEqual(marketLine('buy', 'Battery', 0, 'credits'), 'Added Battery; no money moved.', 'a cashless buy');
assertEqual(marketLine('sell', 'Flak', 250, 'eddies'), 'Sold Flak for 250 eddies.', 'a paid sale');
assertEqual(marketLine('sell', 'Flak', 0, 'eddies'), 'Removed Flak; no money moved.', 'a cashless sale');

// The sheets wire it: one Market button, and the sheets' old ways to add gear are gone.
for (const sheet of ['./StarfinderCharacterSheet.tsx', './CyberpunkCharacterSheet.tsx']) {
  const text = source(sheet);
  assert(text.includes('<SystemMarketDialog'), `${sheet} renders the Market`);
  assert(/>\s*Market\s*</.test(text), `${sheet} has a Market button`);
  assert(!/Add (Weapon|Armor|Gear)/.test(text), `${sheet} has no Add button`);
}
for (const dialog of ['./StarfinderChoicesDialog.tsx', './StarfinderLevelUpDialog.tsx']) {
  assert(!/equipment=\{/.test(source(dialog)), `${dialog} offers no equipment list: gear comes from the Market`);
}
// Creation has no saved character to buy for, so it keeps its list.
assert(/equipment=\{/.test(source('./StarfinderCreateForm.tsx')), 'the creation form keeps its equipment list');
assert(/props\.equipment \?/.test(source('./StarfinderChoicesPanel.tsx')), 'the panel draws the Add buttons only with a list');
