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
