import { readFileSync } from 'node:fs';
import { createElement, isValidElement, type ReactElement, type ReactNode } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert, assertEqual } from '../testSupport/asserts';
import { buildPrintCss } from './printLayout';
import {
  PROGRESSION_DRAWER_CLOSED,
  ProgressionDrawerPanel,
  progressionDrawerActionForKey,
  progressionDrawerReducer,
  type ProgressionDrawerAction,
  type ProgressionDrawerState,
} from './ProgressionDrawer';

const OPEN: ProgressionDrawerState = progressionDrawerReducer(PROGRESSION_DRAWER_CLOSED, { type: 'open' });

function verifiesEachOfTheThreeClosePathsClosesAndReturnsFocus() {
  assertEqual(PROGRESSION_DRAWER_CLOSED.open, false, 'the drawer starts closed');
  assertEqual(PROGRESSION_DRAWER_CLOSED.returnFocus, false, 'a sheet that just opened takes no focus');
  assertEqual(OPEN.open, true, 'the button opens it');
  assertEqual(OPEN.returnFocus, false, 'opening sends no focus to the button');
  for (const via of ['x', 'click-outside', 'escape'] as const) {
    const closed = progressionDrawerReducer(OPEN, { type: 'close', via });
    assertEqual(closed.open, false, `${via} closes the drawer`);
    assertEqual(closed.returnFocus, true, `${via} returns focus to the button`);
    const reopened = progressionDrawerReducer(closed, { type: 'open' });
    assertEqual(reopened.open, true, `the drawer opens again after ${via}`);
    assertEqual(reopened.returnFocus, false, `reopening after ${via} sends no focus to the button`);
  }
}

function verifiesAStrayCloseOnAClosedDrawerTakesNoFocus() {
  // The same object: React runs no effect, so a late event cannot pull focus to the button.
  for (const via of ['x', 'click-outside', 'escape'] as const) {
    assert(progressionDrawerReducer(PROGRESSION_DRAWER_CLOSED, { type: 'close', via }) === PROGRESSION_DRAWER_CLOSED, `${via} on a closed drawer changes nothing`);
  }
  assert(progressionDrawerReducer(OPEN, { type: 'open' }) === OPEN, 'opening an open drawer changes nothing');
}

function verifiesOnlyEscapeIsACloseKey() {
  assertEqual(JSON.stringify(progressionDrawerActionForKey('Escape')), JSON.stringify({ type: 'close', via: 'escape' }), 'Escape closes');
  for (const key of ['Enter', ' ', 'Tab', 'x', 'esc']) {
    assertEqual(progressionDrawerActionForKey(key), null, `${JSON.stringify(key)} does not close`);
  }
}

const panelProps = (onAction: (action: ProgressionDrawerAction) => void = () => {}) => ({
  onAction,
  dialogRef: { current: null },
  children: createElement('p', null, 'Hit die: d10'),
});

function verifiesThePanelIsAModalDialogOnTheLeft() {
  const html = renderToStaticMarkup(createElement(ProgressionDrawerPanel, panelProps()));
  assert(html.includes('role="dialog"'), 'the panel is a dialog');
  assert(html.includes('aria-modal="true"'), 'the dialog is modal');
  assert(html.includes('aria-label="Progression"'), 'the dialog is named');
  assert(html.includes('aria-label="Close"'), 'the X is named');
  assert(html.includes('Hit die: d10'), 'the dialog holds what the sheet hands it');
  assert(/justify-content:flex-start/.test(html), 'the drawer sits at the left edge');
}

function collect(node: ReactNode, into: ReactElement<Record<string, unknown>>[] = []) {
  if (Array.isArray(node)) {
    node.forEach((child) => collect(child, into));
  } else if (isValidElement(node)) {
    const element = node as ReactElement<Record<string, unknown>>;
    into.push(element);
    collect(element.props.children as ReactNode, into);
  }
  return into;
}

function verifiesThePanelWiresTheXAndTheOutsideClick() {
  // The panel has no hooks, so it can be called as a function and its real handlers fired.
  const seen: ProgressionDrawerAction[] = [];
  const elements = collect(ProgressionDrawerPanel(panelProps((action) => seen.push(action))));
  const byProp = (name: string, value: string) => elements.filter((element) => element.props[name] === value);

  const [backdrop] = byProp('role', 'presentation');
  const [dialog] = byProp('role', 'dialog');
  const [x] = byProp('aria-label', 'Close');
  (backdrop.props.onClick as () => void)();
  assertEqual(JSON.stringify(seen), JSON.stringify([{ type: 'close', via: 'click-outside' }]), 'a click on the backdrop closes');

  seen.length = 0;
  let stopped = 0;
  (dialog.props.onClick as (event: { stopPropagation: () => void }) => void)({ stopPropagation: () => { stopped += 1; } });
  assertEqual(stopped, 1, 'a click inside the drawer does not reach the backdrop');
  assertEqual(seen.length, 0, 'a click inside the drawer does not close it');

  (x.props.onClick as () => void)();
  assertEqual(JSON.stringify(seen), JSON.stringify([{ type: 'close', via: 'x' }]), 'the X closes');
  assert(x.props.autoFocus === true, 'focus lands on the X when the drawer opens');
}

function verifiesTheSheetHostsTheDrawerAndPrintKeepsTheHistory() {
  const source = readFileSync(new URL('./CharacterSheet.tsx', import.meta.url), 'utf8');
  const box = source.slice(source.indexOf("case 'showProgression':"), source.indexOf("case 'squareBlock':"));
  assert(box.includes('ref={progressionButtonRef}'), 'focus returns to the Show Progression button');
  assert(box.includes("dispatchProgressionDrawer({ type: 'open' })"), 'the button opens the drawer');
  assert(/progressionDrawer\.open \? <ProgressionDrawer onAction=\{dispatchProgressionDrawer\}>\{progressionContent\}<\/ProgressionDrawer>/.test(box), 'the drawer holds the former rail content');
  assert(/printing \? progressionContent : null/.test(box), 'print draws the history in the rail');
  assert(buildPrintCss().includes('[role="dialog"]'), 'print hides dialogs, which is why the rail draws the history then');
  assert(/if \(progressionDrawer\.returnFocus\) \{\s*progressionButtonRef\.current\?\.focus\(\);/.test(source), 'the sheet acts on returnFocus');
}

verifiesEachOfTheThreeClosePathsClosesAndReturnsFocus();
verifiesAStrayCloseOnAClosedDrawerTakesNoFocus();
verifiesOnlyEscapeIsACloseKey();
verifiesThePanelIsAModalDialogOnTheLeft();
verifiesThePanelWiresTheXAndTheOutsideClick();
verifiesTheSheetHostsTheDrawerAndPrintKeepsTheHistory();

console.log('ProgressionDrawer.test.ts: all assertions passed');
