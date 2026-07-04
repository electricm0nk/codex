<<<<<<< HEAD
import { assert, assertEqual } from '../../testSupport/asserts';
import { defaultLastCheckDiagnostics } from './diagnostics';
import type { LastCheckDiagnostics } from './diagnostics';

/**
 * AV-DIAG-2 cross-cite (F3b owns the `last_check.*` model shape). Field-presence
 * only: every documented key exists with the documented type, defaulting to a
 * deterministic sentinel. F3c renders these keys into `#last-check-panel`.
 */

const EXPECTED_KEYS = [
  'selected_channel',
  'index_url',
  'index_status',
  'manifest_status',
  'release_version',
  'release_notes_status',
  'eligibility_result',
  'install_disabled_reason',
];

function main() {
  const model: LastCheckDiagnostics = defaultLastCheckDiagnostics();

  assertEqual(
    Object.keys(model).sort().join(','),
    [...EXPECTED_KEYS].sort().join(','),
    'last_check diagnostics exposes exactly the eight documented keys'
  );

  assertEqual(typeof model.selected_channel, 'string', 'selected_channel is a string label');
  assertEqual(typeof model.index_url, 'string', 'index_url is a string');
  assertEqual(typeof model.index_status, 'string', 'index_status is a FetchStatus label');
  assertEqual(typeof model.manifest_status, 'string', 'manifest_status is a FetchStatus label');
  assert(
    model.release_version === null || typeof model.release_version === 'string',
    'release_version is string | null'
  );
  assertEqual(typeof model.release_notes_status, 'string', 'release_notes_status is a FetchStatus label');
  assertEqual(typeof model.eligibility_result, 'string', 'eligibility_result is an EligibilityResult label');
  assert(
    model.install_disabled_reason === null || typeof model.install_disabled_reason === 'string',
    'install_disabled_reason is string | null'
  );

  // Deterministic sentinels when no check has run yet (fail-closed posture).
  assertEqual(model.index_url, '', 'index_url defaults to empty string');
  assertEqual(model.release_version, null, 'release_version defaults to null');
  assertEqual(model.eligibility_result, 'unknown', 'eligibility_result defaults to unknown (Install not enabled)');
  assertEqual(model.install_disabled_reason, null, 'install_disabled_reason defaults to null');

  console.log('lastCheckPanel.test.ts: last_check.* model shape verified');
}

main();
=======
import { renderToStaticMarkup } from 'react-dom/server';
import { Sd16LastCheckPanel, LAST_CHECK_PANEL_ID } from './lastCheckPanel';
import {
  buildUnwiredUpdateDeps,
  emptyLastCheckState,
  type Sd16UpdateControllerDeps,
  type Sd16LastCheckState,
} from './updateModel';
import { assert, assertEqual } from '../../testSupport/asserts';

function assertContains(actual: string, needle: string, message: string) {
  if (!actual.includes(needle)) {
    throw new Error(`${message}: expected markup to contain "${needle}"`);
  }
}

function render(deps: Sd16UpdateControllerDeps): string {
  return renderToStaticMarkup(Sd16LastCheckPanel({ deps }));
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
  const lastCheck: Sd16LastCheckState = {
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
  const deps: Sd16UpdateControllerDeps = {
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
>>>>>>> origin/develop
