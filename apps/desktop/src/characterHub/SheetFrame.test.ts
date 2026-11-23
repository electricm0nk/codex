import { readFileSync } from 'node:fs';
import { createElement, type CSSProperties } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert, assertEqual } from '../testSupport/asserts';
import { SHEET_COLUMN_BOTTOM_PADDING, SHEET_FRAME_STYLES, SheetFrame } from './SheetFrame';
import { SHEET_LAYOUT, layoutBox } from './sheetLayout';

// SD-43 E2.1 (items 17 + 21, decisions.md §24): the sheet fits the real window. The root is the
// viewport (`100dvh`, no page scroll), the bar stays put, the centre is the main scroller and each
// side column scrolls inside itself when taller than the window (Q-17.1). No DOM on this box: the
// test pins the style model and the static markup.

function scrolls(style: CSSProperties): boolean {
  return [style.overflow, style.overflowY].some((value) => value === 'auto' || value === 'scroll');
}

function verifiesTheRootIsTheViewportAndNeverScrolls() {
  const root = SHEET_FRAME_STYLES.root;
  assertEqual(root.height, '100dvh', 'the root is the real viewport height, not an assumed pixel number');
  assertEqual(root.overflow, 'hidden', 'the root itself never scrolls');
  assertEqual(root.position, 'fixed', 'the root is pinned to the window, so the page under it has nothing to scroll');
  assertEqual(root.inset, 0, 'the root covers the whole window (no -3rem / 100vw escape from the page padding)');
  assertEqual(root.display, 'grid', 'the root is a grid');
  assertEqual(root.gridTemplateRows, 'auto minmax(0, 1fr)', 'row 1 the bar, row 2 the columns, which may shrink below their content');
  assert(!/px$/.test(String(root.height)) && root.maxHeight === undefined, 'no fixed pixel height anywhere on the root');
}

function verifiesOnlyTheThreeColumnsScroll() {
  const scrollers = Object.entries(SHEET_FRAME_STYLES)
    .filter(([, style]) => scrolls(style))
    .map(([name]) => name)
    .sort();
  assertEqual(scrollers.join('|'), 'centre|left|right', 'the centre and the two side columns are the only scrolling regions');
  for (const name of ['left', 'centre', 'right'] as const) {
    assertEqual(SHEET_FRAME_STYLES[name].minHeight, 0, `${name} may shrink to the window, so its overflow scrolls inside it`);
    assert(SHEET_FRAME_STYLES[name].paddingBottom !== undefined, `${name} keeps bottom padding, so its last control clears the window edge`);
  }
  assertEqual(SHEET_FRAME_STYLES.columns.minHeight, 0, 'the column row fits its grid track');
  assertEqual(SHEET_FRAME_STYLES.columns.overflow, 'hidden', 'the column row clips; its columns scroll');
}

function verifiesTheBarIsOpaqueAndBelowEveryDialog() {
  const bar = SHEET_FRAME_STYLES.bar;
  assertEqual(bar.borderBottom, '2px solid var(--color-accent)', 'the bar keeps its accent line');
  assert(typeof bar.backgroundColor === 'string' && bar.backgroundColor.startsWith('var(--color-'), 'the bar has an opaque theme background');
  assertEqual(bar.zIndex, undefined, 'the bar takes no z-index, so dialogs and the progression drawer (fixed, z-index >= 60) stay above it');
  assertEqual(bar.position, undefined, 'the bar is a grid row, not a sticky or fixed layer');
}

function verifiesTheMarkupPlacesEachSlot() {
  const html = renderToStaticMarkup(
    createElement(SheetFrame, {
      bar: createElement('span', null, 'BAR'),
      left: createElement('span', null, 'LEFT'),
      leftWidth: '240px',
      centre: createElement('span', null, 'CENTRE'),
      right: createElement('span', null, 'RIGHT'),
      rightWidth: '300px',
      children: createElement('span', null, 'DIALOG'),
    }),
  );
  assert(html.includes('height:100dvh'), 'the rendered root carries 100dvh');
  const order = ['BAR', 'LEFT', 'CENTRE', 'RIGHT', 'DIALOG'].map((label) => html.indexOf(label));
  assert(order.every((at) => at >= 0), 'every slot renders');
  assert(order.every((at, index) => index === 0 || at > order[index - 1]), 'bar, left, centre, right, then the overlays');
  assert(/class="sheet-left"[^>]*flex:0 0 240px/.test(html), 'the left column takes the width the sheet asks for');
  assert(/class="sheet-right"[^>]*flex:0 0 300px/.test(html), 'the right column takes the width the sheet asks for');
}

function verifiesThePathfinderSheetRendersInsideTheFrame() {
  const source = readFileSync(new URL('./CharacterSheet.tsx', import.meta.url), 'utf8');
  assert(source.includes('<SheetFrame'), 'the PF sheet renders inside SheetFrame');
  assert(!source.includes("marginTop: '-3rem'"), 'the PF sheet no longer escapes the page padding by hand');
  const left = source.slice(source.indexOf('left={'), source.indexOf('centre={'));
  assert(left.includes("renderSheetRegion('leftRail')"), 'the rail (with Show Progression) is the left column');
}

function verifiesShowProgressionStaysInView() {
  // Item 21: the button showed cut off under the dock. The rail now scrolls inside itself (Q-17.1), and the
  // button's box sticks to the rail's bottom edge, so it shows in full at any window height.
  const box = layoutBox(SHEET_LAYOUT, 'showProgression').boxStyle;
  assertEqual(box.position, 'sticky', 'the Show Progression box sticks inside the scrolling rail');
  assertEqual(box.bottom, `calc(-1 * ${SHEET_COLUMN_BOTTOM_PADDING})`, 'it sticks to the window edge, over the rail\'s bottom padding band');
  assert(typeof box.backgroundColor === 'string' && box.backgroundColor.startsWith('var(--color-'), 'it is opaque over the rail content scrolling under it');
}

verifiesTheRootIsTheViewportAndNeverScrolls();
verifiesShowProgressionStaysInView();
verifiesOnlyTheThreeColumnsScroll();
verifiesTheBarIsOpaqueAndBelowEveryDialog();
verifiesTheMarkupPlacesEachSlot();
verifiesThePathfinderSheetRendersInsideTheFrame();
console.log('SheetFrame.test.ts: all assertions passed');
