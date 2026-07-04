import { renderToStaticMarkup } from 'react-dom/server';
import { Sd16CheckPanel } from './CheckPanel';
import {
  buildUnwiredUpdateDeps,
  emptyLastCheckState,
  SD16_UNWIRED_CONTROLLER,
} from './updateModel';
import type {
  Sd16EligibilityResult,
  Sd16LastCheckState,
  Sd16PendingRollbackState,
  Sd16ReleaseNotes,
  Sd16UpdateController,
  Sd16UpdateControllerDeps,
} from './updateModel';
import { assert, assertEqual } from '../../testSupport/asserts';

function assertContains(actual: string, needle: string, message: string) {
  if (!actual.includes(needle)) {
    throw new Error(`${message}: expected markup to contain "${needle}"`);
  }
}

function render(deps: Sd16UpdateControllerDeps, checkInProgress: boolean): string {
  return renderToStaticMarkup(
    Sd16CheckPanel({
      deps,
      checkInProgress,
      onCheck: () => undefined,
    }),
  );
}


function withLastCheck(overrides: Partial<Sd16LastCheckState>): Sd16LastCheckState {
  return { ...emptyLastCheckState(), ...overrides };
}

function testReleaseNotesPanelAlwaysVisible() {
  const html = render(buildUnwiredUpdateDeps(), false);
  assertContains(
    html,
    'id="release-notes"',
    'AV-UI-3: release notes panel must always render with id="release-notes"',
  );
  assertContains(
    html,
    'No release notes available yet. Run Check to load them.',
    'AV-UI-3: empty state must surface an explicit "not loaded" message',
  );
}

function testReleaseNotesPanelSurfacesLoadedNotesAfterCheck() {
  const notes: Sd16ReleaseNotes = {
    releaseVersion: '1.2.3',
    body: '## Highlights\n- add update UI\n- surface diagnostics',
  };
  const deps: Sd16UpdateControllerDeps = {
    ...buildUnwiredUpdateDeps(),
    lastCheck: withLastCheck({
      indexStatus: 'ok',
      manifestStatus: 'ok',
      releaseVersion: '1.2.3',
      releaseNotesStatus: 'loaded',
      eligibilityResult: 'eligible',
    }),
    releaseNotes: notes,
  };
  const html = render(deps, false);
  assertContains(
    html,
    '## Highlights',
    'AV-UI-3: rendered release notes must contain the loaded body verbatim',
  );
  assertContains(
    html,
    'Release notes for 1.2.3',
    'AV-UI-3: panel aria-label must include the loaded release version',
  );
  assert(
    !html.includes('No release notes available yet'),
    'AV-UI-3: empty-state placeholder must be hidden once notes load',
  );
}

function testReleaseNotesNullBodyFallsBackToPlaceholder() {
  const deps: Sd16UpdateControllerDeps = {
    ...buildUnwiredUpdateDeps(),
    lastCheck: withLastCheck({
      indexStatus: 'ok',
      manifestStatus: 'ok',
      releaseVersion: '1.2.4',
      releaseNotesStatus: 'unavailable',
    }),
    releaseNotes: { releaseVersion: '1.2.4', body: null },
  };
  const html = render(deps, false);
  assertContains(
    html,
    '(release notes body is unavailable)',
    'AV-UI-3: null body must fall back to the explicit unavailable placeholder',
  );
}

function testCheckButtonDisabledWhileCheckInProgress() {
  const html = render(buildUnwiredUpdateDeps(), true);
  assertContains(
    html,
    'disabled=""',
    'AV-UI-3: Check button must be disabled while a check is in progress',
  );
  assertContains(html, 'Checking…', 'AV-UI-3: button label must read "Checking…" while busy');
}

function testCheckButtonEnabledWhenIdle() {
  const html = render(buildUnwiredUpdateDeps(), false);
  assertEqual(
    html.includes('disabled=""'),
    false,
    'AV-UI-3: Check button must not be disabled in the idle state',
  );
  assertContains(html, '>Check<', 'AV-UI-3: button must read "Check" when idle');
}

testReleaseNotesPanelAlwaysVisible();
testReleaseNotesPanelSurfacesLoadedNotesAfterCheck();
testReleaseNotesNullBodyFallsBackToPlaceholder();
testCheckButtonDisabledWhileCheckInProgress();
testCheckButtonEnabledWhenIdle();

console.log('releaseNotesDisplay.test.ts: 5/5 assertions passed');
