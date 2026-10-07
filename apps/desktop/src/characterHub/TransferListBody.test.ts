import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert, assertEqual } from '../testSupport/asserts';
import { TransferListBody } from './TransferListDialog';
import type { TransferItem } from './transferListModel';

const item = (id: string, over: Partial<TransferItem> = {}): TransferItem => ({ id, label: `Item ${id}`, description: `About ${id}.`, qualified: true, ...over });
const ITEMS = [item('a'), item('b'), item('c', { qualified: false, unqualifiedReason: 'Needs BAB +6' }), item('d', { innate: true })];

const render = (over: Partial<Parameters<typeof TransferListBody>[0]> = {}) =>
  renderToStaticMarkup(
    createElement(TransferListBody, {
      items: ITEMS,
      selected: [],
      focusedId: null,
      qualifiedOnly: true,
      limit: 2,
      remainingNoun: 'selections',
      onFocus: () => {},
      onMoveRight: () => {},
      onMoveLeft: () => {},
      onQualifiedChange: () => {},
      ...over,
    }),
  );

function verifiesTheRemainingCountIsAtTheTop() {
  const html = render({ selected: ['a'] });
  assert(html.includes('1 of 2 selections remaining'), 'remaining count with its noun');
  assert(!render({ limit: null }).includes('remaining'), 'no count when there is no limit');
}

function verifiesQualifiedIsCheckedByDefaultAndFiltersTheOptions() {
  const on = render();
  assert(/type="checkbox"[^>]*checked/.test(on) || /checked=""[^>]*type="checkbox"/.test(on), 'Qualified is checked');
  assert(on.includes('Item a') && on.includes('Item b'), 'qualified options show');
  assert(!on.includes('Item c'), 'the unqualified option is hidden while Qualified is on');
  const off = render({ qualifiedOnly: false });
  assert(off.includes('Item c'), 'turning Qualified off shows everything');
}

function verifiesThreeColumnsWhenThereAreInnateItems() {
  const html = render({ selected: ['b'] });
  assert(html.includes('Options') && html.includes('Selected'), 'options and selected columns');
  assert(html.includes('Innate') && html.includes('Item d'), 'the innate column lists what comes with the character');
  const none = render({ items: [item('a')] });
  assert(!none.includes('Innate'), 'no innate column when nothing is innate');
}

function verifiesMovementButtonsFollowTheFocus() {
  const none = render();
  assert(/aria-label="Move to selected"[^>]*disabled/.test(none), '> is disabled with nothing focused');
  const focusedOption = render({ focusedId: 'a' });
  assert(!/aria-label="Move to selected"[^>]*disabled/.test(focusedOption), '> is enabled for a focused qualified option');
  const focusedSelected = render({ selected: ['b'], focusedId: 'b' });
  assert(!/aria-label="Move to options"[^>]*disabled/.test(focusedSelected), '< is enabled for a focused selected item');
  const full = render({ selected: ['a', 'b'], focusedId: 'c', qualifiedOnly: false });
  assert(/aria-label="Move to selected"[^>]*disabled/.test(full), '> is disabled when nothing remains');
}

function verifiesTheDescriptionPaneShowsTheClickedItemAndWhyItCannotBeTaken() {
  const html = render({ focusedId: 'c', qualifiedOnly: false });
  assert(html.includes('Item c'), 'the title');
  assert(html.includes('About c.'), 'the full description');
  assert(html.includes('Needs BAB +6'), 'the reason it cannot be taken');
  assert(render().includes('Click an item'), 'a hint when nothing is focused');
}

verifiesTheRemainingCountIsAtTheTop();
verifiesQualifiedIsCheckedByDefaultAndFiltersTheOptions();
verifiesThreeColumnsWhenThereAreInnateItems();
verifiesMovementButtonsFollowTheFocus();
verifiesTheDescriptionPaneShowsTheClickedItemAndWhyItCannotBeTaken();
console.log('TransferListBody.test.ts: all assertions passed');
