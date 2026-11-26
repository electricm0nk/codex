import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert, assertEqual } from '../testSupport/asserts';
import { EMPTY_CUSTOM, OVERRIDE_TOGGLES, customOverrides, withOverride } from './customModel';
import { CUSTOM_TAB_TOGGLES, HOUSE_RULE_FLAGS, HP_METHODS, hitPointEntryFor, houseRulesFor } from './houseRulesModel';
import { HouseRulesDialog } from './HouseRulesDialog';

// A Cyberpunk RED character shows no rules; Starfinder none either (E0.5 §5: no flag has a Starfinder consumer).
assertEqual(houseRulesFor('cyberpunk-red').length, 0, 'CPR shows no house rules');
assertEqual(houseRulesFor('starfinder-1e').length, 0, 'SF shows no house rules (E0.5 §5)');

// A Pathfinder character shows six: the four existing flags and the two whose engine cards follow.
const pf = houseRulesFor('pathfinder-1e');
assertEqual(pf.length, 6, 'a PF character shows six house rules');
assertEqual(pf.filter((rule) => rule.flag !== null).map((rule) => rule.flag).join(), 'ignoreSlotLimits,featsWithoutPrerequisites,weaponsWithoutProficiency,armorWithoutProficiency', 'the four existing flags, in the intake order');
assertEqual(pf.filter((rule) => rule.flag === null && rule.choice === null).map((rule) => rule.notice?.includes('E6.4')).join(), 'true', 'Ambidextrous still names the card that brings it');
assertEqual(pf.filter((rule) => rule.choice === 'hpMethod').map((rule) => rule.label).join(), 'Hit points at level-up', 'E6.3: the HP rule is a choice, no notice');

// E6.3: the seven methods, in the intake order; two of them ask the player for a number at level-up.
assertEqual(HP_METHODS.map((method) => method.label).join('|'), 'User Rolled|Random|Random, 50% floor|Average|Average, round up|Maximum|Percentage', 'the seven methods');
assertEqual(HP_METHODS.map((method) => hitPointEntryFor(method.value)).join(), 'roll,,,,,,percent', 'User Rolled asks the roll, Percentage the percentage');
assertEqual(hitPointEntryFor(undefined), null, 'no house rule: the level-up asks nothing');
assertEqual(EMPTY_CUSTOM.hpMethod, undefined, 'a new character has no HP method (the rules default)');
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
const pfHtml = renderToStaticMarkup(createElement(HouseRulesDialog, { gameSystem: 'pathfinder-1e', custom: characterA, error: null, onChange: noop, onHpMethodChange: noop, onClose: noop }));
assertEqual(pfHtml.split('type="checkbox"').length - 1, 4, 'PF: four checkboxes');
assert(pfHtml.includes('checked=""'), 'PF: the flag set on A is ticked');
assertEqual(pfHtml.split('<option').length - 1, 8, 'PF: the HP select offers the rules default and the seven');
const rolledHtml = renderToStaticMarkup(createElement(HouseRulesDialog, { gameSystem: 'pathfinder-1e', custom: { ...EMPTY_CUSTOM, hpMethod: 'randomHalfFloor' }, error: null, onChange: noop, onHpMethodChange: noop, onClose: noop }));
assert(rolledHtml.includes('value="randomHalfFloor" selected=""'), 'the stored method is selected');
for (const rule of pf) {
  assert(pfHtml.includes(rule.label), `PF: ${rule.label} is listed`);
}
const cprHtml = renderToStaticMarkup(createElement(HouseRulesDialog, { gameSystem: 'cyberpunk-red', custom: EMPTY_CUSTOM, error: null, onChange: noop, onHpMethodChange: noop, onClose: noop }));
assertEqual(cprHtml.split('type="checkbox"').length - 1, 0, 'CPR: no checkbox');
assert(cprHtml.includes('No house rules'), 'CPR: says there are none');

console.log('houseRulesModel.test.ts: ok');
