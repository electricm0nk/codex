/**
 * Self-executing test for `apps/desktop/scripts/ui-smoke/lib/probe.mjs`'s
 * target/select lookup — the piece the whole harness depends on to click by
 * DOM-reported name instead of a hard-coded pixel coordinate. A regression
 * here (e.g. picking a disabled duplicate over an enabled one, or matching
 * by substring instead of exact name) would make every row in spec.json
 * click the wrong thing without any of them saying so.
 */
import { assert, assertEqual } from './asserts';
// eslint-disable-next-line import/extensions -- plain Node ESM lib, see mjsModules.d.ts
import { findSelect, findTarget, findTargets, screenContains } from '../../scripts/ui-smoke/lib/probe.mjs';

function target(name: string, overrides: Partial<{ disabled: boolean; y: number }> = {}) {
  return {
    name,
    tag: 'button',
    disabled: overrides.disabled ?? false,
    rect: { x: 0, y: overrides.y ?? 0, w: 10, h: 10 },
  };
}

function verifiesExactNameMatchOnly() {
  const snapshot = {
    ts: 1,
    headings: [],
    bodyText: '',
    dialogs: [],
    selects: [],
    targets: [target('Back'), target('Back to landing')],
  };
  assertEqual(findTargets(snapshot, 'Back').length, 1, 'exact match only, not substring');
  assertEqual(findTarget(snapshot, 'Back')?.name, 'Back', 'returns the exact match');
  assertEqual(findTarget(snapshot, 'Nonexistent'), undefined, 'no match returns undefined');
}

function verifiesEnabledPreferredOverDisabledDuplicate() {
  const snapshot = {
    ts: 1,
    headings: [],
    bodyText: '',
    dialogs: [],
    selects: [],
    targets: [target('Load', { disabled: true }), target('Load', { disabled: false })],
  };
  const found = findTarget(snapshot, 'Load');
  assert(found !== undefined, 'a match is found');
  assertEqual(found?.disabled, false, 'the enabled duplicate is preferred over the disabled one');
}

function verifiesFallsBackToFirstWhenAllDisabled() {
  const snapshot = {
    ts: 1,
    headings: [],
    bodyText: '',
    dialogs: [],
    selects: [],
    targets: [target('Delete', { disabled: true })],
  };
  const found = findTarget(snapshot, 'Delete');
  assertEqual(found?.name, 'Delete', 'a disabled-only match is still returned, not dropped');
}

function verifiesSelectLookupByExactName() {
  const snapshot = {
    ts: 1,
    headings: [],
    bodyText: '',
    dialogs: [],
    targets: [],
    selects: [
      { name: 'select#1', optionCount: 2 },
      { name: 'character-race', optionCount: 0 },
    ],
  };
  assertEqual(findSelect(snapshot, 'select#1')?.optionCount, 2, 'positional select name resolves');
  assertEqual(findSelect(snapshot, 'character-race')?.optionCount, 0, 'id-fallback select name resolves, including a zero count');
  assertEqual(findSelect(snapshot, 'missing'), undefined, 'unknown select name resolves to undefined');
}

function verifiesScreenContainsChecksHeadingsBodyAndDialogs() {
  const bodySnapshot = { ts: 1, headings: [], bodyText: 'Corpus Ingest Diagnostic', dialogs: [], targets: [], selects: [] };
  assert(screenContains(bodySnapshot, 'Corpus Ingest'), 'matches inside bodyText');

  const headingSnapshot = { ts: 1, headings: ['Race Traits'], bodyText: '', dialogs: [], targets: [], selects: [] };
  assert(screenContains(headingSnapshot, 'Race Traits'), 'matches inside headings');

  const dialogSnapshot = { ts: 1, headings: [], bodyText: '', dialogs: ['Add Armor'], targets: [], selects: [] };
  assert(screenContains(dialogSnapshot, 'Add Armor'), 'matches inside dialog labels (portaled dialogs)');

  assert(!screenContains(dialogSnapshot, 'Nowhere'), 'absent string is correctly reported absent');
  assert(!screenContains(null, 'anything'), 'a null snapshot never matches (never a silent false-pass)');
}

function main() {
  verifiesExactNameMatchOnly();
  verifiesEnabledPreferredOverDisabledDuplicate();
  verifiesFallsBackToFirstWhenAllDisabled();
  verifiesSelectLookupByExactName();
  verifiesScreenContainsChecksHeadingsBodyAndDialogs();
}

main();
