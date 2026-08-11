import { assert, assertEqual } from '../testSupport/asserts';
import {
  DIAGNOSTIC_PLACEHOLDER,
  defaultInstalledDiagnostics,
  defaultLastCheckDiagnostics,
  defaultPendingRollbackDiagnostics,
  renderInstalledDiagnostics,
} from './diagnostics';
import type { InstalledDiagnostics } from './diagnostics';

/**
 * Umbrella cross-cite for AV-DIAG-1..3. F3b owns the three diagnostics model
 * shapes and the deterministic render helper F3c consumes. Field-presence and
 * deterministic-placeholder behaviour are asserted; values are not required to
 * be non-zero (E7 carries the real values via the E6 -> E8 gate).
 */

function main() {
  verifiesThreeGroupsSurfaceEveryKey();
  verifiesRenderFillsPlaceholders();
  verifiesRenderPassesThroughKnownValues();
  console.log('diagnostics.test.ts: all diagnostics model assertions passed');
}

function verifiesThreeGroupsSurfaceEveryKey() {
  assertEqual(
    Object.keys(defaultInstalledDiagnostics()).length,
    8,
    'installed.* has eight fields'
  );
  assertEqual(
    Object.keys(defaultLastCheckDiagnostics()).length,
    8,
    'last_check.* has eight fields'
  );
  assertEqual(
    Object.keys(defaultPendingRollbackDiagnostics()).length,
    5,
    'pending/rollback.* has five fields'
  );
}

function verifiesRenderFillsPlaceholders() {
  // A default (unwired) installed model renders every key with a deterministic
  // placeholder — null/empty become the sentinel, never `undefined`.
  const view = renderInstalledDiagnostics(defaultInstalledDiagnostics());
  const expectedKeys = [
    'channel',
    'version',
    'source_commit',
    'artifact_sha256',
    'install_kind',
    'managed_executable_path',
    'update_eligible',
    'ineligible_reason',
  ];
  assertEqual(
    Object.keys(view).sort().join(','),
    [...expectedKeys].sort().join(','),
    'render view keys match the installed.* field set exactly'
  );
  const viewRecord = view as unknown as Record<string, string>;
  for (const key of expectedKeys) {
    const value = viewRecord[key];
    assertEqual(typeof value, 'string', `render view ${key} is a render-ready string`);
    assert(value.length > 0, `render view ${key} is never an empty/undefined placeholder`);
  }
  assertEqual(
    view.managed_executable_path,
    DIAGNOSTIC_PLACEHOLDER,
    'null managed path renders the deterministic placeholder'
  );
  assertEqual(
    view.ineligible_reason,
    DIAGNOSTIC_PLACEHOLDER,
    'null ineligible reason renders the deterministic placeholder'
  );
  assertEqual(view.update_eligible, 'no', 'false eligibility renders as "no"');
}

function verifiesRenderPassesThroughKnownValues() {
  const model: InstalledDiagnostics = {
    channel: 'beta',
    version: '2.3.4',
    source_commit: 'abc1234',
    artifact_sha256: 'deadbeef',
    install_kind: 'appimage',
    managed_executable_path: '/opt/codex/codex.AppImage',
    update_eligible: true,
    ineligible_reason: null,
  };
  const view = renderInstalledDiagnostics(model);
  assertEqual(view.channel, 'beta', 'known channel passes through');
  assertEqual(view.version, '2.3.4', 'known version passes through');
  assertEqual(view.managed_executable_path, '/opt/codex/codex.AppImage', 'known path passes through');
  assertEqual(view.update_eligible, 'yes', 'true eligibility renders as "yes"');
  assertEqual(view.ineligible_reason, DIAGNOSTIC_PLACEHOLDER, 'null reason still renders the placeholder');
}

main();
