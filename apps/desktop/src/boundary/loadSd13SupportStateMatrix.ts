import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary over the SD-13 support-state matrix.
 *
 * This loader is documentary/control-plane only: it invokes the
 * `load_sd13_support_state_matrix` Tauri command and returns the seeded SD-13
 * truth verbatim. It performs no rules computation, filtering, promotion, or
 * feedback/issue-transport work, and it must never treat app/build success as
 * proof that a roster row is `supported`.
 *
 * SD13-E6-F12 remains explicitly deferred: this boundary does not capture
 * evidence, submit issues, persist support truth, or couple matrix debt to
 * update behavior.
 */

export interface Sd13SupportStateRow {
  rowId: string;
  subjectType: string;
  subjectId: string;
  dimension: string;
  supportState: string;
  evidenceTier: string;
  /**
   * SD13-E7-F13 evidence-freshness token projected verbatim from the SD-13
   * carrier (`refreshable-from-live-proof` | `awaiting-initial-evidence`). It is
   * never reinterpreted here; the carrier owns freshness truth.
   */
  evidenceFreshness: string;
  /** SD-13-owned refresh-audit wording; both current postures are refresh-required. */
  refreshAuditLabel: string;
  testerFacingStateLabel: string;
  groundingRef: string;
  blockerOrLossinessNote: string;
  nextRequiredUplift: string;
}

export interface Sd13SupportStateMatrixSnapshot {
  rows: Sd13SupportStateRow[];
  dataSource: string;
  note: string;
}

export async function loadSd13SupportStateMatrix(): Promise<Sd13SupportStateMatrixSnapshot> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for the SD-13 support-state matrix');
  }

  try {
    return await invoke<Sd13SupportStateMatrixSnapshot>('load_sd13_support_state_matrix');
  } catch (cause: unknown) {
    throw new Error(`Failed to load SD-13 support-state matrix: ${formatError(cause)}`);
  }
}
