import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import type { AdmittedRecordDto } from '../boundary/loadSavedCharacterDetail';
import { assert, assertEqual } from '../testSupport/asserts';
import { OVERRIDE_TOGGLES } from './customModel';
import { OverrideTag, classLevelsAdmitted, flagsOf, overrideTagText } from './OverrideTag';

assertEqual(overrideTagText('featsWithoutPrerequisites'), 'Override: Feats without prerequisites', 'the tag names the toggle that let it in');
for (const { key, label } of OVERRIDE_TOGGLES) {
  assert(overrideTagText(key).includes(label), `the tag for ${key} reads as its toggle`);
}

const ADMITTED: AdmittedRecordDto[] = [
  { kind: 'feat', id: 'Improved Two-Weapon Fighting', override: 'ignoreFeatBudget' },
  { kind: 'feat', id: 'Improved Two-Weapon Fighting', override: 'featsWithoutPrerequisites' },
  { kind: 'spell', id: 'Bless', override: 'spellsOutsideClassList' },
  { kind: 'class_level', id: 'class:paladin', override: 'ignoreAlignmentRestrictions' },
  { kind: 'class_level', id: 'class:paladin', override: 'ignoreAlignmentRestrictions' },
  { kind: 'class_level', id: 'class:antipaladin', override: 'ignoreAlignmentRestrictions' },
];
assertEqual(flagsOf(ADMITTED, 'feat', 'Improved Two-Weapon Fighting').join(), 'ignoreFeatBudget,featsWithoutPrerequisites', 'a feat two rules refused shows both');
assertEqual(flagsOf(ADMITTED, 'feat', 'Toughness').length, 0, 'a feat that qualified shows none');
assertEqual(flagsOf(ADMITTED, 'spell', 'Improved Two-Weapon Fighting').length, 0, 'a row is of its own kind');
assertEqual(flagsOf(undefined, 'feat', 'Toughness').length, 0, 'a character with none sends no field');

const levels = classLevelsAdmitted(ADMITTED, [
  { classId: 'class:paladin', classLabel: 'Paladin' },
  { classId: 'class:fighter', classLabel: 'Fighter' },
]);
assertEqual(levels.map((row) => `${row.label} x${row.levels}`).join(), 'Paladin x2,class:antipaladin x1', 'one line a class, counted; a class with no label reads as its id');

const html = renderToStaticMarkup(createElement(OverrideTag, { flag: 'ignoreSlotLimits' }));
assert(html.includes('Override: Ignore body slot limits'), 'the badge says what let it in');
