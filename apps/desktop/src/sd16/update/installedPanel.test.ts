import { assert, assertEqual } from '../../testSupport/asserts';
import { defaultInstalledDiagnostics } from './diagnostics';
import type { InstalledDiagnostics } from './diagnostics';

/**
 * AV-DIAG-1 cross-cite (F3b owns the `installed.*` model shape). Field-presence
 * only: every documented key exists with the documented type, defaulting to a
 * deterministic sentinel. F3c renders these keys into `#installed-panel`.
 */

const EXPECTED_KEYS = [
  'channel',
  'version',
  'source_commit',
  'artifact_sha256',
  'install_kind',
  'managed_executable_path',
  'update_eligible',
  'ineligible_reason',
];

function main() {
  const model: InstalledDiagnostics = defaultInstalledDiagnostics();

  assertEqual(
    Object.keys(model).sort().join(','),
    [...EXPECTED_KEYS].sort().join(','),
    'installed diagnostics exposes exactly the eight documented keys'
  );

  assertEqual(typeof model.channel, 'string', 'channel is a string label');
  assertEqual(typeof model.version, 'string', 'version is a string');
  assertEqual(typeof model.source_commit, 'string', 'source_commit is a string');
  assertEqual(typeof model.artifact_sha256, 'string', 'artifact_sha256 is a string');
  assertEqual(typeof model.install_kind, 'string', 'install_kind is a string label');
  assert(
    model.managed_executable_path === null || typeof model.managed_executable_path === 'string',
    'managed_executable_path is string | null'
  );
  assertEqual(typeof model.update_eligible, 'boolean', 'update_eligible is a boolean');
  assert(
    model.ineligible_reason === null || typeof model.ineligible_reason === 'string',
    'ineligible_reason is string | null'
  );

  // Deterministic sentinels when E7 has not landed.
  assertEqual(model.version, '', 'version defaults to empty string');
  assertEqual(model.install_kind, 'unknown', 'install_kind defaults to the unknown sentinel');
  assertEqual(model.managed_executable_path, null, 'managed_executable_path defaults to null');
  assertEqual(model.update_eligible, false, 'update_eligible defaults to false (fail-closed)');
  assertEqual(model.ineligible_reason, null, 'ineligible_reason defaults to null');

  console.log('installedPanel.test.ts: installed.* model shape verified');
}

main();
