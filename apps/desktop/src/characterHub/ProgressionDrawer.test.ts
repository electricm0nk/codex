import { readFileSync } from 'node:fs';
import { createElement, isValidElement, type ReactElement, type ReactNode } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert, assertEqual } from '../testSupport/asserts';
import type { ProgressionLedgerDto, ProgressionLevelDto } from '../boundary/listProgressionLedger';
import { buildPrintCss } from './printLayout';
import {
  PROGRESSION_DRAWER_CLOSED,
  ProgressionDrawerPanel,
  ProgressionLedgerView,
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
  assert(/progressionDrawer\.open \? \(\s*<ProgressionDrawer onAction=\{dispatchProgressionDrawer\}>\s*<ProgressionLedger characterId=\{props\.row\.characterId\} heldClasses=\{heldClasses\} \/>\s*\{progressionNext\}\s*<\/ProgressionDrawer>/.test(box), 'the drawer holds the ledger, then what the next level grants');
  assert(/printing \? progressionContent : null/.test(box), 'print draws the history in the rail');
  assert(buildPrintCss().includes('[role="dialog"]'), 'print hides dialogs, which is why the rail draws the history then');
  assert(/if \(progressionDrawer\.returnFocus\) \{\s*progressionButtonRef\.current\?\.focus\(\);/.test(source), 'the sheet acts on returnFocus');
}

const LEVEL: ProgressionLevelDto = {
  characterLevel: 1,
  classId: 'class:fighter',
  classLevel: 1,
  hitDieResult: null,
  feats: [{ id: 'feat:power_attack', name: 'Power Attack', target: null }],
  spells: [],
  ranks: [],
  abilityIncreases: [],
  classFeatures: [],
  abilityIncreaseDue: false,
  unspent: { feats: 0, spells: 0, ranks: 0 },
  assignedOnUpgrade: true,
};
const NOTHING = { feats: [], spells: [], ranks: [], abilityIncreases: [] };
const HELD = [{ classId: 'class:fighter', classLabel: 'Fighter', level: 3 }];

function viewHtml(state: Parameters<typeof ProgressionLedgerView>[0]['state']) {
  return renderToStaticMarkup(createElement(ProgressionLedgerView, { state, heldClasses: HELD }));
}

function verifiesTheDrawerShowsALevelAndWhatItsRowsAreMarked() {
  const ledger: ProgressionLedgerDto = { levels: [LEVEL, { ...LEVEL, characterLevel: 2, classLevel: 2, assignedOnUpgrade: false }], withoutLevel: NOTHING };
  const html = viewHtml({ kind: 'loaded', ledger });
  assert(html.includes('Level 1'), 'the first level is headed');
  assert(html.includes('Fighter 1'), 'with the class level taken');
  assert(html.includes('Feats: Power Attack'), 'its feat by name');
  assertEqual(html.split('Assigned on upgrade').length - 1, 1, 'only the level whose rows were worked out says so');
  assert(!html.includes('Not placed'), 'nothing to set apart, no heading for it');
}

function verifiesWhatHoldsNoLevelIsShownAndNotLost() {
  const html = viewHtml({ kind: 'loaded', ledger: { levels: [LEVEL], withoutLevel: { ...NOTHING, feats: [{ id: 'feat:toughness', name: 'Toughness', target: null }] } } });
  assert(html.includes('Not placed at a level'), 'the group is named');
  assert(html.includes('Feats: Toughness'), 'the feat is listed there');
}

function verifiesLoadingAndFailureEachSaySo() {
  assert(viewHtml({ kind: 'loading' }).includes('Reading the level history'), 'it says it is reading');
  const failed = viewHtml({ kind: 'failed', reason: 'invalid ledger JSON' });
  assert(failed.includes('invalid ledger JSON'), 'a failure names its reason');
  assert(failed.includes('role="alert"'), 'and is announced');
  assert(!failed.includes('Level 1'), 'never an empty history in its place');
}

verifiesEachOfTheThreeClosePathsClosesAndReturnsFocus();
verifiesAStrayCloseOnAClosedDrawerTakesNoFocus();
verifiesOnlyEscapeIsACloseKey();
verifiesThePanelIsAModalDialogOnTheLeft();
verifiesThePanelWiresTheXAndTheOutsideClick();
verifiesTheSheetHostsTheDrawerAndPrintKeepsTheHistory();
verifiesTheDrawerShowsALevelAndWhatItsRowsAreMarked();
verifiesWhatHoldsNoLevelIsShownAndNotLost();
verifiesLoadingAndFailureEachSaySo();

console.log('ProgressionDrawer.test.ts: all assertions passed');
