import { assert, assertEqual } from '../testSupport/asserts';
import { SHEET_TABS } from './CharacterSheet';
import {
  PRINT_COLUMNS_IN,
  PRINT_MARGIN_IN,
  PRINT_PAGE_IN,
  PRINT_TAB_ORDER,
  buildPrintCss,
  printContentWidthIn,
  printableTabs,
} from './printLayout';

const css = buildPrintCss();

function verifiesTheSheetIsALetterSizePage() {
  assertEqual(PRINT_PAGE_IN.width, 8.5, 'US letter width');
  assertEqual(PRINT_PAGE_IN.height, 11, 'US letter height');
  assert(css.includes('@page'), 'a page rule is emitted');
  assert(/size:\s*letter/.test(css), 'the page is letter sized');
  assert(css.includes(`margin: ${PRINT_MARGIN_IN}in`), 'the page margin is the declared one');
  assert(css.includes('@media print'), 'everything applies to print only');
}

function verifiesTheThreeColumnsFitInsideTheMargins() {
  const content = printContentWidthIn();
  assertEqual(content, PRINT_PAGE_IN.width - 2 * PRINT_MARGIN_IN, 'content width is the page minus both margins');
  const fixed = PRINT_COLUMNS_IN.left + PRINT_COLUMNS_IN.right + 2 * PRINT_COLUMNS_IN.gap;
  assert(fixed < content, `fixed columns (${fixed}in) leave room for the middle column inside ${content}in`);
  assert(content - fixed >= 3, 'the middle column keeps at least 3 inches');
}

function verifiesPrintIsBlackOnWhiteRegardlessOfTheTheme() {
  for (const token of ['--color-page', '--color-surface', '--color-text']) {
    assert(css.includes(token), `${token} is overridden for print`);
  }
  assert(/--color-page:\s*#fff\s*!important/.test(css), 'the page background is forced white, over any community theme');
  assert(/--color-text:\s*#000\s*!important/.test(css), 'text is forced black');
}

function verifiesInteractiveControlsDoNotPrint() {
  for (const selector of ['button', '[role="dialog"]', '.no-print']) {
    assert(css.includes(selector), `${selector} is hidden when printing`);
  }
  assert(/display:\s*none\s*!important/.test(css), 'hidden means display none');
}

function verifiesTabsFlowAfterPageOneInsteadOfOnePageEach() {
  assert(/\.print-tab-first\s*\{[^}]*break-before:\s*page/.test(css), 'the first printed tab starts a fresh page after the sheet');
  assert(!/\.print-tab\s*\{[^}]*break-before:\s*page/.test(css), 'later tabs follow on, they do not each take a page');
  assert(/\.print-tab\s*\{[^}]*break-inside:\s*avoid/.test(css), 'a tab that fits on a page is not split across two');
  assert(/\.print-tab h2[^}]*break-after:\s*avoid/.test(css), 'a tab heading is never stranded at the foot of a page');
}

function verifiesPrintedTabsCoverEveryTabAndSkipAnEmptyPetsPage() {
  assertEqual([...PRINT_TAB_ORDER].sort().join('|'), [...SHEET_TABS].sort().join('|'), 'the print order lists exactly the sheet\'s tabs');
  assert(printableTabs({ hasPets: true, hasCustom: false }).includes('Pets'), 'pets print when the character has any');
  assert(!printableTabs({ hasPets: false, hasCustom: false }).includes('Pets'), 'an empty Pets page is not printed');
  assert(printableTabs({ hasPets: false, hasCustom: true }).includes('Custom'), 'custom grants and records print when there are any');
  assert(!printableTabs({ hasPets: false, hasCustom: false }).includes('Custom'), 'an empty Custom page is not printed');
  assertEqual(printableTabs({ hasPets: true, hasCustom: true })[0], 'Weapons', 'printing starts with Weapons');
}

verifiesTheSheetIsALetterSizePage();
verifiesTheThreeColumnsFitInsideTheMargins();
verifiesPrintIsBlackOnWhiteRegardlessOfTheTheme();
verifiesInteractiveControlsDoNotPrint();
verifiesTabsFlowAfterPageOneInsteadOfOnePageEach();
verifiesPrintedTabsCoverEveryTabAndSkipAnEmptyPetsPage();
console.log('printLayout.test.ts: all assertions passed');
