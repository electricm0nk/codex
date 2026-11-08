import { assert, assertEqual } from '../testSupport/asserts';
import { chipStyle, selectedChipKey } from './customChip';

const keys = (style: object) => Object.keys(style).sort().join(',');
const same = (a: object, b: object) => JSON.stringify(a, Object.keys(a).sort()) === JSON.stringify(b, Object.keys(b).sort());

function deselectedChipEqualsRestingChip() {
  const resting = chipStyle({ selected: false, focused: false });
  const afterSelect = chipStyle({ selected: true, focused: false });
  const afterDeselect = chipStyle({ selected: false, focused: false });
  assert(same(afterDeselect, resting), 'selected -> deselected returns exactly the resting style');
  assert(!same(afterSelect, resting), 'a selected chip looks different from a resting one');
  // Same property set on every state, so React clears nothing and a longhand never fights a shorthand.
  assertEqual(keys(afterSelect), keys(resting), 'every state sets the same properties');
  assertEqual(resting.backgroundColor, 'var(--color-surface-2)', 'resting fill is the surface token');
  assertEqual(resting.color, 'var(--color-text)', 'resting text is the text token, not the selected text colour');
  assertEqual('background' in resting, false, 'no background shorthand beside backgroundColor');
}

function selectedChipUsesThemeTokensOnly() {
  const selected = chipStyle({ selected: true, focused: false });
  assertEqual(selected.backgroundColor, 'var(--color-accent)', 'selected fill is the accent token');
  assertEqual(selected.color, 'var(--color-on-accent)', 'selected text is the on-accent token');
  for (const style of [selected, chipStyle({ selected: false, focused: true })]) {
    for (const value of Object.values(style)) {
      assert(typeof value !== 'string' || !/#[0-9a-f]{3,8}\b|rgb\(/i.test(value), `no literal colour (${String(value)})`);
    }
  }
}

function focusRingIsDistinctFromSelection() {
  const focusedResting = chipStyle({ selected: false, focused: true });
  const resting = chipStyle({ selected: false, focused: false });
  const selected = chipStyle({ selected: true, focused: false });
  assertEqual(focusedResting.backgroundColor, resting.backgroundColor, 'focus does not fill the chip like selection does');
  assert(focusedResting.outline !== 'none' && focusedResting.outline !== resting.outline, 'focus draws an outline');
  assertEqual(selected.outline, resting.outline, 'selection draws no outline');
  const both = chipStyle({ selected: true, focused: true });
  assertEqual(both.backgroundColor, selected.backgroundColor, 'focus keeps the selected fill');
  assert(both.outline === focusedResting.outline, 'focus ring is the same on a selected chip');
}

function exactlyOneChipIsSelected() {
  const tabs = ['grants', 'feats', 'equipment', 'spells', 'devices'];
  for (const current of tabs) {
    const selectedCount = tabs.filter((tab) => selectedChipKey(current, tab)).length;
    assertEqual(selectedCount, 1, `exactly one chip selected when ${current} is current`);
  }
}

deselectedChipEqualsRestingChip();
selectedChipUsesThemeTokensOnly();
focusRingIsDistinctFromSelection();
exactlyOneChipIsSelected();
console.log('customChip.test.ts: all assertions passed');
