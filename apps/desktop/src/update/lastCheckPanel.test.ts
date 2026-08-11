import { renderToStaticMarkup } from 'react-dom/server';
import { LastCheckPanel, LAST_CHECK_PANEL_ID } from './lastCheckPanel';
import {
  buildUnwiredUpdateDeps,
  emptyLastCheckState,
  type UpdateControllerDeps,
  type LastCheckState,
} from './updateModel';
import { assert, assertEqual } from '../testSupport/asserts';

function assertContains(actual: string, needle: string, message: string) {
  if (!actual.includes(needle)) {
    throw new Error(`${message}: expected markup to contain "${needle}"`);
  }
}

function render(deps: UpdateControllerDeps): string {
  return renderToStaticMarkup(LastCheckPanel({ deps }));
}

function testPanelRendersWithCanonicalId() {
  const html = render(buildUnwiredUpdateDeps());
  assertContains(
    html,
    `id="${LAST_CHECK_PANEL_ID}"`,
    'AV-DIAG-2: last-check panel must render with id="#last-check-panel"',
  );
}

function testAllEightLastCheckFieldsExposed() {
  const html = render(buildUnwiredUpdateDeps());
  const expectedRows = [
    'selected-channel',
    'index-url',
    'index-status',
    'manifest-status',
    'release-version',
    'release-notes',
    'eligibility',
    'install-disabled-reason',
  ];
  for (const needle of expectedRows) {
    assertContains(
      html,
      `data-last-check-row="${needle}"`,
      `AV-DIAG-2: last-check panel must expose row for "${needle}"`,
    );
  }
}

function testFieldValuesReflectSuppliedState() {
  const lastCheck: LastCheckState = {
    ...emptyLastCheckState(),
    selectedChannel: 'beta',
    indexUrl:
      'https://raw.githubusercontent.com/electricm0nk/codex/update-index/channels/beta.json',
    indexStatus: 'ok',
    manifestStatus: 'ok',
    releaseVersion: '2.0.1',
    releaseNotesStatus: 'loaded',
    eligibilityResult: 'ineligible',
    installDisabledReason: 'installed version is at or above manifest version',
  };
  const deps: UpdateControllerDeps = {
    ...buildUnwiredUpdateDeps(),
    lastCheck,
  };
  const html = render(deps);
  assertContains(html, '>beta<', 'AV-DIAG-2: selected channel value is rendered');
  assertContains(
    html,
    '/update-index/channels/beta.json',
    'AV-DIAG-2: index URL is rendered',
  );
  assertContains(html, '>ok<', 'AV-DIAG-2: ok status is rendered for both fetches');
  assertContains(html, '>2.0.1<', 'AV-DIAG-2: release version is rendered');
  assertContains(
    html,
    '>ineligible<',
    'AV-DIAG-2: eligibility value is rendered',
  );
  assertContains(
    html,
    'installed version is at or above manifest version',
    'AV-DIAG-2: disabled-reason from F1 closure decision table is rendered',
  );
}

function testEmptyStateSurfacesNotYetCheckedReason() {
  const html = render(buildUnwiredUpdateDeps());
  assertContains(
    html,
    'check has not been run yet',
    'AV-DIAG-2: empty last-check state must surface the explicit not-checked reason',
  );
}

testPanelRendersWithCanonicalId();
testAllEightLastCheckFieldsExposed();
testFieldValuesReflectSuppliedState();
testEmptyStateSurfacesNotYetCheckedReason();

console.log('lastCheckPanel.test.ts: 4/4 assertions passed');
