import { assert, assertEqual } from '../testSupport/asserts';
import {
  availableItems,
  canMoveRight,
  describeFocus,
  innateItems,
  moveLeft,
  moveRight,
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
console.log('transferListModel.test.ts: all assertions passed');
