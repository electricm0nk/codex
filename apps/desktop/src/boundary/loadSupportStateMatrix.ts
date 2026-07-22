import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary over the support-state matrix.
 *
 * This loader is documentary/control-plane only: it invokes the
 * `load_support_state_matrix` Tauri command and returns the seeded
 * truth verbatim. It performs no rules computation, filtering, promotion, or
 * feedback/issue-transport work, and it must never treat app/build success as
 * proof that a roster row is `supported`.
 *
 * Capturing evidence, submitting issues, persisting support truth, and
 * coupling matrix debt to update behavior remain explicitly deferred.
 */

export interface SupportStateRow {
  rowId: string;
  subjectType: string;
  subjectId: string;
  dimension: string;
  supportState: string;
  evidenceTier: string;
  /**
   * Evidence-freshness token projected verbatim from the support-state
   * carrier (`refreshable-from-live-proof` | `awaiting-initial-evidence`). It is
   * never reinterpreted here; the carrier owns freshness truth.
   */
  evidenceFreshness: string;
  /** Support-state-owned refresh-audit wording; both current postures are refresh-required. */
  refreshAuditLabel: string;
  testerFacingStateLabel: string;
  groundingRef: string;
  blockerOrLossinessNote: string;
  nextRequiredUplift: string;
}

export interface SupportStateMatrixSnapshot {
  rows: SupportStateRow[];
  dataSource: string;
  note: string;
}

export async function loadSupportStateMatrix(): Promise<SupportStateMatrixSnapshot> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for the support-state matrix');
  }

  try {
    return await invoke<SupportStateMatrixSnapshot>('load_support_state_matrix');
  } catch (cause: unknown) {
    throw new Error(`Failed to load support-state matrix: ${formatError(cause)}`);
  }
}
