import type { MarketEntryDto } from '../boundary/listMarket';
import { assert, assertEqual } from '../testSupport/asserts';
import {
  affordable,
  buyNegotiateGp,
  buyRoute,
  mapMarketTableRows,
  parseQuantity,
  quantityTotalLabel,
  sellNegotiateGp,
  sortEntries,
  type MarketSortColumn,
} from './marketModel';

const entry = (key: string, costGp: number | null, weightLbs: number | null, slot: MarketEntryDto['slot'], description: string | null = null): MarketEntryDto =>
  ({ key, name: key, category: 'ArmsArmor', costGp, weightLbs, description, types: ['Weapon'], slot, variants: [] }) as unknown as MarketEntryDto;

const SWORD = { ...entry('Longsword', 15, 4, 'wield_main', 'A heavy blade.'), variants: [{ bonus: 1, name: '+1 Longsword', priceGp: 2315, modifierKey: 'k' }] } as MarketEntryDto;
const DAGGER = entry('Dagger', 2, 1, 'wield_main');
const PLATE = entry('Full Plate', 1500, 50, 'armor');
const RING = entry('Ring of Mystery', null, null, 'ring');
const ROWS = mapMarketTableRows([SWORD, DAGGER, PLATE, RING]);
const keys = (column: MarketSortColumn, direction: 'asc' | 'desc') => sortEntries(ROWS, column, direction).map((row) => row.key).join();

// The table row carries the columns and the description box's text.
{
  const sword = ROWS.find((row) => row.key === 'Longsword');
  assertEqual(sword?.weightLbs, 4, 'weight column');
  assertEqual(sword?.slot, 'wield_main', 'slot column');
  assertEqual(sword?.description, 'A heavy blade.', 'the description box reads the whole description');
  assertEqual(ROWS.map((row) => row.key).join(), 'Dagger,Full Plate,Longsword,Ring of Mystery', 'mapped by name');
}

// Sort by each column both ways; a row with no value sits last either way.
assertEqual(keys('item', 'asc'), 'Dagger,Full Plate,Longsword,Ring of Mystery', 'item ascending');
assertEqual(keys('item', 'desc'), 'Ring of Mystery,Longsword,Full Plate,Dagger', 'item descending');
assertEqual(keys('price', 'asc'), 'Dagger,Longsword,Full Plate,Ring of Mystery', 'price ascending, unpriced last');
assertEqual(keys('price', 'desc'), 'Full Plate,Longsword,Dagger,Ring of Mystery', 'price descending, unpriced still last');
assertEqual(keys('weight', 'asc'), 'Dagger,Longsword,Full Plate,Ring of Mystery', 'weight ascending');
assertEqual(keys('weight', 'desc'), 'Full Plate,Longsword,Dagger,Ring of Mystery', 'weight descending');
assertEqual(keys('slot', 'asc'), 'Full Plate,Dagger,Longsword,Ring of Mystery', 'slot ascending by label (Armor, Main hand, Ring), ties by name');
assertEqual(keys('slot', 'desc'), 'Ring of Mystery,Dagger,Longsword,Full Plate', 'slot descending, ties still by name');
assertEqual(ROWS[0].key, 'Dagger', 'sorting returns a new list');

// Only Show Affordable at the three price modes.
{
  const plate = ROWS.find((row) => row.key === 'Full Plate')!;
  const sword = ROWS.find((row) => row.key === 'Longsword')!;
  const ring = ROWS.find((row) => row.key === 'Ring of Mystery')!;
  assertEqual(affordable(plate, 0, 'standard', 100_000), false, 'standard: 1,500 gp over 1,000 gp of funds');
  assertEqual(affordable(plate, 0, 'standard', 150_000), true, 'standard: exactly the funds');
  assertEqual(affordable(plate, 0, 'characterBuild', 100_000), false, 'character build pays 100% too');
  assertEqual(affordable(plate, 0, 'cashless', 0), true, 'cashless: everything is affordable');
  assertEqual(affordable(sword, 1, 'standard', 200_000), false, 'the chosen +N is the price compared');
  assertEqual(affordable(ring, 0, 'standard', 0), true, 'a row with no price is never hidden: it buys through Negotiate');
}

// Quantity: the total label, and the field's parse.
assertEqual(quantityTotalLabel(2, 3, 'gp'), '3 × 2 gp = 6 gp', 'three bundles');
assertEqual(quantityTotalLabel(2315, 1, 'gp'), '2,315 gp', 'one reads as the price');
assertEqual(quantityTotalLabel(null, 3, 'gp'), 'No listed price', 'unpriced');
assertEqual(parseQuantity('3'), 3, 'a count');
assertEqual(parseQuantity(''), 1, 'blank is one');
assertEqual(parseQuantity('0'), 1, 'never below one');
assertEqual(parseQuantity('2.7'), 2, 'whole bundles');

// An unpriced row routes to Negotiate; the Negotiate field is prefilled at the mode.
{
  const ring = ROWS.find((row) => row.key === 'Ring of Mystery')!;
  const dagger = ROWS.find((row) => row.key === 'Dagger')!;
  assertEqual(buyRoute(ring, 0), 'negotiate', 'no price: Buy opens Negotiate');
  assertEqual(buyRoute(dagger, 0), 'buy', 'a priced row buys');
  assertEqual(buyRoute(ROWS.find((row) => row.key === 'Longsword')!, 3), 'negotiate', 'a +N not on offer has no price');
  assertEqual(buyNegotiateGp(dagger, 0, 3, 'standard'), 6, 'listed price × quantity');
  assertEqual(buyNegotiateGp(dagger, 0, 3, 'cashless'), 0, 'cashless pays nothing');
  assertEqual(buyNegotiateGp(ring, 0, 1, 'standard'), null, 'nothing to prefill');
}

// The Sell Negotiate prefill is the refund rule on the pieces' share of the bundle price.
assertEqual(sellNegotiateGp(1, 30, 20, 'standard'), 0.75, '30 arrows of a 1 gp bundle of 20 at 50%');
assertEqual(sellNegotiateGp(2315, 1, 1, 'characterBuild'), 2315, 'character build returns all');
assertEqual(sellNegotiateGp(15, 1, 1, 'cashless'), 0, 'cashless returns nothing');
assertEqual(sellNegotiateGp(null, 1, 1, 'standard'), null, 'no price: nothing to prefill');
assert(sellNegotiateGp(0.05, 1, 20, 'standard') === 0, 'floored to a copper before the refund');

console.log('marketModel.test.ts: all assertions passed');
