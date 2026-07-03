import { assertEqual } from '../../testSupport/asserts';
import { defaultPendingRollbackDiagnostics } from './diagnostics';
import type { PendingRollbackDiagnostics } from './diagnostics';

/**
 * AV-DIAG-3 cross-cite (F3b owns the `pending/rollback.*` model shape). This
 * group has a soft dependency on E7 (the backup filesystem lives in E7's
 * transaction module); the assertion is that every key is surfaced with a
 * deterministic placeholder, not that the values are non-zero. F3c renders these
 * keys into `#pending-rollback-panel`.
 */

const EXPECTED_KEYS = [
  'pending_update_state',
  'previous_version_available',
  'rollback_state',
  'backup_count',
  'retained_update_storage_bytes',
];

function main() {
  const model: PendingRollbackDiagnostics = defaultPendingRollbackDiagnostics();

  assertEqual(
    Object.keys(model).sort().join(','),
    [...EXPECTED_KEYS].sort().join(','),
    'pending/rollback diagnostics exposes exactly the five documented keys'
  );

  assertEqual(typeof model.pending_update_state, 'string', 'pending_update_state is a label');
  assertEqual(typeof model.previous_version_available, 'boolean', 'previous_version_available is a boolean');
  assertEqual(typeof model.rollback_state, 'string', 'rollback_state is a label');
  assertEqual(typeof model.backup_count, 'number', 'backup_count is a number');
  assertEqual(typeof model.retained_update_storage_bytes, 'number', 'retained_update_storage_bytes is a number');

  // Deterministic placeholders while E7 has not landed (E6 -> E8 gate).
  assertEqual(model.pending_update_state, 'none', 'pending_update_state defaults to none');
  assertEqual(model.previous_version_available, false, 'previous_version_available defaults to false');
  assertEqual(model.rollback_state, 'unknown', 'rollback_state defaults to unknown');
  assertEqual(model.backup_count, 0, 'backup_count defaults to 0');
  assertEqual(model.retained_update_storage_bytes, 0, 'retained_update_storage_bytes defaults to 0');

  console.log('pendingRollbackPanel.test.ts: pending/rollback.* model shape verified');
}

main();
