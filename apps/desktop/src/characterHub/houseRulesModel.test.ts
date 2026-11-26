import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert, assertEqual } from '../testSupport/asserts';
import { EMPTY_CUSTOM, OVERRIDE_TOGGLES, customOverrides, withOverride } from './customModel';
import { CUSTOM_TAB_TOGGLES, HOUSE_RULE_FLAGS, houseRulesFor } from './houseRulesModel';
import { HouseRulesDialog } from './HouseRulesDialog';

// A Cyberpunk RED character shows no rules; Starfinder none either (E0.5 §5: no flag has a Starfinder consumer).
assertEqual(houseRulesFor('cyberpunk-red').length, 0, 'CPR shows no house rules');
assertEqual(houseRulesFor('starfinder-1e').length, 0, 'SF shows no house rules (E0.5 §5)');

// A Pathfinder character shows six: the four existing flags and the two whose engine cards follow.
const pf = houseRulesFor('pathfinder-1e');
assertEqual(pf.length, 6, 'a PF character shows six house rules');
assertEqual(pf.filter((rule) => rule.flag !== null).map((rule) => rule.flag).join(), 'ignoreSlotLimits,featsWithoutPrerequisites,weaponsWithoutProficiency,armorWithoutProficiency', 'the four existing flags, in the intake order');
assertEqual(pf.filter((rule) => rule.flag === null).map((rule) => rule.notice?.includes('E6.3') || rule.notice?.includes('E6.4')).join(), 'true,true', 'a rule with no engine yet names the card that brings it');
for (const rule of pf) {
  if (rule.flag !== null) {
    assertEqual(rule.label, OVERRIDE_TOGGLES.find((toggle) => toggle.key === rule.flag)?.label, `${rule.flag}: the House Rules label is the override tag's label`);
  }
}

// A new character has every rule off; setting one on character A leaves B off.
for (const flag of HOUSE_RULE_FLAGS) {
  assertEqual(customOverrides(EMPTY_CUSTOM)[flag], false, `${flag} is off on a new character`);
}
const characterA = withOverride(EMPTY_CUSTOM, 'ignoreSlotLimits', true);
const characterB = EMPTY_CUSTOM;
assertEqual(customOverrides(characterA).ignoreSlotLimits, true, 'A has it on');
assertEqual(customOverrides(characterB).ignoreSlotLimits, false, 'B stays off');

// The four flags leave the Custom tab (Q-3.4): one setting, one home.
assertEqual(CUSTOM_TAB_TOGGLES.length, OVERRIDE_TOGGLES.length - 4, 'the Custom tab keeps the other overrides');
assert(CUSTOM_TAB_TOGGLES.every((toggle) => !HOUSE_RULE_FLAGS.includes(toggle.key)), 'no house-rule flag on the Custom tab');

// The dialog: six rows and four checkboxes for PF, a sentence and no checkbox for CPR.
const noop = () => undefined;
const pfHtml = renderToStaticMarkup(createElement(HouseRulesDialog, { gameSystem: 'pathfinder-1e', custom: characterA, error: null, onChange: noop, onClose: noop }));
assertEqual(pfHtml.split('type="checkbox"').length - 1, 4, 'PF: four checkboxes');
assert(pfHtml.includes('checked=""'), 'PF: the flag set on A is ticked');
for (const rule of pf) {
  assert(pfHtml.includes(rule.label), `PF: ${rule.label} is listed`);
}
const cprHtml = renderToStaticMarkup(createElement(HouseRulesDialog, { gameSystem: 'cyberpunk-red', custom: EMPTY_CUSTOM, error: null, onChange: noop, onClose: noop }));
assertEqual(cprHtml.split('type="checkbox"').length - 1, 0, 'CPR: no checkbox');
assert(cprHtml.includes('No house rules'), 'CPR: says there are none');

console.log('houseRulesModel.test.ts: ok');
