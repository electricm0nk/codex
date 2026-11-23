import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert, assertEqual } from '../testSupport/asserts';
import type { PlacedEquipmentCopyDto } from '../boundary/listHeldEquipment';
import { SellDialogView, type SellDialogViewProps } from './SellDialog';
import { DEFAULT_SELL_OPTION, SELL_OPTIONS, sellAmountGp, sellAmountText, sellBundleGp, sellNegotiatedGp } from './sellDialogModel';

// The window's three choices, in order (`decisions.md §10`): Cashless is left out, "Normal" never appears.
assertEqual(SELL_OPTIONS.map((option) => option.label).join('|'), 'Market Rate|Build|Negotiate', 'the option list');
assertEqual(DEFAULT_SELL_OPTION, 'marketRate', 'the window opens on Market Rate');
assertEqual(SELL_OPTIONS.find((option) => option.value === DEFAULT_SELL_OPTION)?.label, 'Market Rate', 'the default is labelled Market Rate');

// The amount is the sell rule (`sellNegotiateGp`) on the pieces sold; it scales with the count.
assertEqual(sellAmountGp(15, 1, 1, 'marketRate'), 7.5, 'Market Rate returns half');
assertEqual(sellAmountGp(15, 1, 1, 'build'), 15, 'Build returns all');
assertEqual(sellAmountGp(15, 3, 1, 'marketRate'), 22.5, 'a stack of 3 at Market Rate');
assertEqual(sellAmountGp(15, 3, 1, 'build'), 45, 'a stack of 3 at Build');
assertEqual(sellAmountGp(1, 30, 20, 'marketRate'), 0.75, '30 arrows of a 1 gp bundle of 20');
assertEqual(sellAmountGp(15, 3, 1, 'negotiate'), 22.5, 'Negotiate prefills the Market Rate amount');

// No listed price: nothing to show, and Sell still removes the item.
assertEqual(sellAmountGp(null, 3, 1, 'marketRate'), null, 'null price');
assertEqual(sellAmountText(null), '', 'null price shows blank');
assertEqual(sellAmountText(22.5), '22.5', 'an amount shows as typed');

// What the Sell write is sent: the computed amount, the typed one for Negotiate, or a refusal.
assertEqual(sellNegotiatedGp('marketRate', 22.5, ''), 22.5, 'Market Rate sends its amount');
assertEqual(sellNegotiatedGp('build', 45, ''), 45, 'Build sends its amount');
assertEqual(sellNegotiatedGp('marketRate', null, ''), undefined, 'no price: Sell sends nothing to pay');
assertEqual(sellNegotiatedGp('negotiate', 22.5, '30'), 30, 'Negotiate sends the typed total');
assertEqual(sellNegotiatedGp('negotiate', 22.5, '0'), 0, 'a zero total is allowed');
for (const bad of ['', '  ', '-1', 'abc']) {
  const result = sellNegotiatedGp('negotiate', 22.5, bad);
  assert(typeof result === 'string' && result.startsWith('Enter the agreed total'), `"${bad}" is refused`);
}
assert(!JSON.stringify(SELL_OPTIONS).includes('Normal'), 'no "Normal" label');

// The price the stack is listed at: its override, its record's price, or the Market's +N offer.
const longsword = { key: 'Longsword', name: 'Longsword', priceGp: 15, variants: [{ bonus: 1, name: '+1 Longsword', priceGp: 2315, modifierKey: 'PLUS1' }] } as never;
assertEqual(sellBundleGp({ bonus: 0 }, 15, null), 15, 'a plain copy: the record price');
assertEqual(sellBundleGp({ bonus: 1 }, 15, longsword), 2315, 'a +1 copy: the Market offer');
assertEqual(sellBundleGp({ bonus: 1 }, 15, null), null, 'a +1 copy the Market does not list: no price');
assertEqual(sellBundleGp({ bonus: 0, costOverrideCopper: 1000 }, 15, null), 10, 'a cost override wins');
assertEqual(sellBundleGp({ bonus: 0 }, null, null), null, 'no price listed');

// The window as it renders: three radios, Market Rate checked, How many? for a stack, the amount for it.
const daggers: PlacedEquipmentCopyDto = {
  itemId: 'Dagger', name: 'Dagger', baseName: 'Dagger', bonus: 0, activeState: 'SelectedInactive', slot: 'none',
  location: { kind: 'carried' }, ordinal: 1, quantity: 3, copies: [{ itemId: 'Dagger', bonus: 0, ordinal: 1 }], baseQuantity: 1,
};
const noop = () => {};
const view = (over: Partial<SellDialogViewProps> = {}) =>
  renderToStaticMarkup(createElement(SellDialogView, { label: 'Dagger', held: daggers, option: 'marketRate', count: '3', negotiated: '', bundleGp: 2, priceError: null, error: null, busy: false, onOption: noop, onCount: noop, onNegotiated: noop, onSell: noop, onCancel: noop, ...over }));
const html = view();
assert(html.includes('Market Rate') && html.includes('Build') && html.includes('Negotiate'), 'view: the three options');
assert(/data-testid="sell-option-marketRate"[^>]*checked/.test(html) || /checked[^>]*data-testid="sell-option-marketRate"/.test(html), 'view: Market Rate checked');
assert(html.includes('How many?') && html.includes('data-testid="sell-count"'), 'view: How many? for a stack');
assert(html.includes('Returns: 3 gp'), 'view: 3 daggers at 2 gp, half back');
assert(view({ option: 'build' }).includes('Returns: 6 gp'), 'view: Build returns all');
assert(view({ count: '1' }).includes('Returns: 1 gp'), 'view: one of the stack');
assert(view({ bundleGp: null }).includes('Returns: \u2014'), 'view: no price shows a dash');
assert(view({ option: 'negotiate', negotiated: '4' }).includes('data-testid="sell-negotiated"'), 'view: Negotiate takes a typed total');
assert(!html.includes('Normal') && !html.includes('pricing above') && !html.includes('Cashless'), 'view: no Normal, no pricing above, no Cashless');

console.log('sellDialogModel.test.ts: all assertions passed');
