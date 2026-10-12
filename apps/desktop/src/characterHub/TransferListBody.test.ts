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

// Brief 1 F6: the spells dialog shows one labelled Options/Selected pair per spell level, the level
// on the far left and "N of M picks left" on each pair.
function verifiesOnePairPerGroupInGroupedMode() {
  const spells = [
    item('c1', { label: 'Light', group: 'wizard:0' }),
    item('s1', { label: 'Sleep', group: 'wizard:1' }),
    item('s2', { label: 'Magic Missile', group: 'wizard:1' }),
    item('i1', { label: 'Dancing Lights', innate: true }),
  ];
  const html = render({
    items: spells,
    selected: ['s1'],
    limit: null,
    groupLimits: { 'wizard:0': 1, 'wizard:1': 2 },
    groupLabels: { 'wizard:0': 'Wizard level 0', 'wizard:1': 'Wizard level 1' },
    pairPerGroup: true,
  });
  assertEqual((html.match(/role="group"/g) ?? []).length, 2, 'one pair per spell level');
  assert(html.includes('aria-label="Wizard level 0 options"') && html.includes('aria-label="Wizard level 1 selected"'), 'each list is named for its level');
  assert(html.indexOf('Wizard level 0') < html.indexOf('aria-label="Wizard level 0 options"'), 'the level label comes first, on the left');
  assert(html.includes('1 of 1 picks left') && html.includes('1 of 2 picks left'), 'each pair says how many picks are left');
  const level1 = html.slice(html.indexOf('aria-label="Wizard level 1 group"'));
  assert(level1.includes('Magic Missile') && !level1.slice(0, level1.indexOf('Wizard level 1 selected')).includes('>Light<'), 'a level lists only its own spells');
  assertEqual((html.match(/Dancing Lights/g) ?? []).length, 1, 'innate abilities are listed once, not per level');
}

function verifiesTheSinglePairStaysWithoutGroupedMode() {
  const html = render({ groupLimits: { traits: 2 }, groupLabels: { traits: 'Traits' } });
  assertEqual((html.match(/role="group"/g) ?? []).length, 0, 'Traits, Feats and Racial traits keep one pair');
  assert(html.includes('aria-label="Options"') && html.includes('aria-label="Selected"'), 'the plain Options and Selected lists');
}

verifiesOnePairPerGroupInGroupedMode();
verifiesTheSinglePairStaysWithoutGroupedMode();
verifiesTheRemainingCountIsAtTheTop();
verifiesQualifiedIsCheckedByDefaultAndFiltersTheOptions();
verifiesThreeColumnsWhenThereAreInnateItems();
verifiesMovementButtonsFollowTheFocus();
verifiesTheDescriptionPaneShowsTheClickedItemAndWhyItCannotBeTaken();
console.log('TransferListBody.test.ts: all assertions passed');
