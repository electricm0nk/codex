import { assert, assertEqual } from '../testSupport/asserts';
import {
  availableItems,
  canMoveRight,
  describeFocus,
  innateItems,
  moveLeft,
  moveRight,
  remainingByGroup,
  remainingSelections,
  selectedItems,
  type TransferItem,
} from './transferListModel';

const item = (id: string, over: Partial<TransferItem> = {}): TransferItem => ({ id, label: id.toUpperCase(), description: `about ${id}`, qualified: true, ...over });
const ITEMS: TransferItem[] = [
  item('a'),
  item('b'),
  item('c', { qualified: false, unqualifiedReason: 'Needs BAB +6' }),
  item('d', { innate: true }),
];

// Spells are limited per spell level: two level-0 picks and one level-1 pick, say, not three anywhere.
function verifiesPerGroupLimits() {
  const spells: TransferItem[] = [
    item('c1', { group: 'wiz:0' }), item('c2', { group: 'wiz:0' }), item('c3', { group: 'wiz:0' }),
    item('l1a', { group: 'wiz:1' }), item('l1b', { group: 'wiz:1' }),
    item('x', { group: 'cle:0' }),
  ];
  const limits = { 'wiz:0': 2, 'wiz:1': 1 };
  assertEqual(moveRight(spells, ['c1'], 'c2', null, limits).join(','), 'c1,c2', 'room in the group');
  assertEqual(moveRight(spells, ['c1', 'c2'], 'c3', null, limits).join(','), 'c1,c2', 'a full group takes no more');
  assertEqual(moveRight(spells, ['c1', 'c2'], 'l1a', null, limits).join(','), 'c1,c2,l1a', 'another group is unaffected');
  const full = canMoveRight(spells, ['c1', 'c2'], 'c3', null, limits);
  assertEqual(full.ok, false, 'blocked');
  assert((full.reason ?? '').includes('wiz:0'), `the reason names the group, got ${full.reason}`);
  assertEqual(moveRight(spells, [], 'x', null, limits).join(','), '', 'a group with no quota takes nothing');
  assertEqual(JSON.stringify(remainingByGroup(spells, ['c1', 'l1a'], limits)), JSON.stringify([
    { key: 'wiz:0', limit: 2, remaining: 1 },
    { key: 'wiz:1', limit: 1, remaining: 0 },
  ]), 'remaining per group, in the limits\' order');
  assertEqual(canMoveRight(spells, ['c1'], 'c2', 5, limits).ok, true, 'a global limit and group limits can both apply');
  assertEqual(canMoveRight(spells, ['c1', 'l1a'], 'c2', 2, limits).ok, false, 'the global limit still wins');
}

function verifiesAvailableShowsEverythingUnselectedExceptInnate() {
  assertEqual(availableItems(ITEMS, [], false).map((i) => i.id).join(''), 'abc', 'everything selectable, qualified or not, but not innate');
  assertEqual(availableItems(ITEMS, ['a'], false).map((i) => i.id).join(''), 'bc', 'a selected item leaves the options');
}

function verifiesQualifiedOnlyHidesWhatTheCharacterMayNotTake() {
  assertEqual(availableItems(ITEMS, [], true).map((i) => i.id).join(''), 'ab', 'qualified-only hides c');
}

function verifiesSelectedAndInnateColumns() {
  assertEqual(selectedItems(ITEMS, ['b', 'a']).map((i) => i.id).join(''), 'ba', 'selected keeps the order they were chosen in');
  assertEqual(innateItems(ITEMS).map((i) => i.id).join(''), 'd', 'innate abilities get their own column');
  assertEqual(selectedItems(ITEMS, ['zzz']).length, 0, 'an id that is not an option is ignored');
}

function verifiesMovingRespectsTheLimitAndQualification() {
  assertEqual(moveRight(ITEMS, [], 'a', 2).join(''), 'a', 'a qualified item moves');
  assertEqual(moveRight(ITEMS, ['a', 'b'], 'c', 2).join(''), 'ab', 'nothing moves past the limit');
  assertEqual(moveRight(ITEMS, [], 'c', 2).join(''), '', 'an unqualified item cannot be selected');
  assertEqual(moveRight(ITEMS, [], 'd', 2).join(''), '', 'an innate item cannot be moved');
  assertEqual(moveRight(ITEMS, ['a'], 'a', 2).join(''), 'a', 'moving twice does not duplicate');
  assertEqual(moveLeft(['a', 'b'], 'a').join(''), 'b', 'moving back removes it');
  assertEqual(moveLeft(['a', 'b'], 'zz').join(''), 'ab', 'moving back something not selected changes nothing');
}

function verifiesAnUnlimitedListHasNoCap() {
  assertEqual(moveRight(ITEMS, ['a', 'b'], 'c', null).join(''), 'ab', 'c is still unqualified');
  const many = Array.from({ length: 30 }, (_, i) => item(`i${i}`));
  let sel: string[] = [];
  for (const m of many) sel = moveRight(many, sel, m.id, null);
  assertEqual(sel.length, 30, 'null limit means no cap');
}

function verifiesRemainingSelections() {
  assertEqual(remainingSelections(3, ['a']), 2, 'limit minus selected');
  assertEqual(remainingSelections(1, ['a', 'b']), 0, 'never negative');
  assertEqual(remainingSelections(null, ['a']), null, 'no limit, nothing to count down');
}

function verifiesCanMoveRightExplainsWhy() {
  assertEqual(canMoveRight(ITEMS, ['a', 'b'], 'a', 2).ok, false, 'already selected');
  const full = canMoveRight(ITEMS, ['a', 'b'], 'c', 2);
  assertEqual(full.ok, false, 'at the limit');
  assert((full.reason ?? '').toLowerCase().includes('no selections'), `names the limit, got ${full.reason}`);
  const unq = canMoveRight(ITEMS, [], 'c', 2);
  assertEqual(unq.reason, 'Needs BAB +6', 'an unqualified item says why');
  assertEqual(canMoveRight(ITEMS, [], 'a', 2).ok, true, 'a qualified item with room can move');
}

function verifiesTheDescriptionFollowsWhateverIsClickedOnEitherSide() {
  const focus = describeFocus(ITEMS, 'c');
  assertEqual(focus?.title, 'C', 'the clicked item titles the pane');
  assert((focus?.body ?? '').includes('about c'), 'with its description');
  assert((focus?.note ?? '').includes('Needs BAB +6'), 'an unqualified item states why it cannot be taken');
  assertEqual(describeFocus(ITEMS, 'a')?.note, undefined, 'a qualified item has no warning');
  assertEqual(describeFocus(ITEMS, 'nope'), null, 'nothing focused, nothing shown');
  assertEqual(describeFocus([item('x', { description: undefined })], 'x')?.body, 'No description available.', 'a missing description is stated, not blank');
}

verifiesAvailableShowsEverythingUnselectedExceptInnate();
verifiesQualifiedOnlyHidesWhatTheCharacterMayNotTake();
verifiesSelectedAndInnateColumns();
verifiesMovingRespectsTheLimitAndQualification();
verifiesAnUnlimitedListHasNoCap();
verifiesRemainingSelections();
verifiesCanMoveRightExplainsWhy();
verifiesTheDescriptionFollowsWhateverIsClickedOnEitherSide();
verifiesPerGroupLimits();
console.log('transferListModel.test.ts: all assertions passed');
