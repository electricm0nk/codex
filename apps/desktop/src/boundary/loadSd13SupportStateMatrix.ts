import { invoke } from '@tauri-apps/api/core';

/**
 * Read-only desktop boundary over the SD-13 support-state matrix.
 *
 * This loader is documentary/control-plane only: it invokes the
 * `load_sd13_support_state_matrix` Tauri command and returns the seeded SD-13
 * truth verbatim. It performs no rules computation, filtering, promotion, or
 * feedback/issue-transport work, and it must never treat app/build success as
 * proof that a roster row is `supported`.
 */

export type Sd13SupportStateToken =
  | 'supported'
  | 'partial'
  | 'lossy'
  | 'blocked'
  | 'unverified';

export interface Sd13SupportStateRow {
  rowId: string;
  subjectType: string;
  subjectId: string;
  dimension: string;
  supportState: string;
  evidenceTier: string;
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

function hasTauriRuntime(): boolean {
  return typeof window !== 'undefined' && ('__TAURI_INTERNALS__' in window || '__TAURI__' in window);
}

function formatError(cause: unknown): string {
  return cause instanceof Error ? cause.message : String(cause);
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
