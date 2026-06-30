import { invoke } from '@tauri-apps/api/core';
import type {
  Sd11TesterChannelLabel,
  Sd11UpdateReleaseTruth,
} from '../sd11/update/updateActionModel';

/**
 * Narrow read-only boundary for the SD-11 check/update action.
 *
 * It mirrors the established preferred-Tauri-command-plus-explicit-fallback pattern used by
 * `loadGe08AuthoringWorkbench.ts` and `loadPilotShellSnapshot.ts`. It returns governed SD-12
 * release truth from the runtime when available, and an explicit guarded result otherwise. It must
 * never counterfeit a successful update check: when the runtime cannot prove official release truth,
 * it surfaces `check-failed` rather than fabricating `up-to-date`.
 */
export interface Sd11UpdateActionRequest {
  buildVersion: string;
  buildLabel: string;
  platformLabel: string;
  testerChannelLabel: Sd11TesterChannelLabel;
}

function hasTauriRuntime(): boolean {
  return typeof window !== 'undefined' && ('__TAURI_INTERNALS__' in window || '__TAURI__' in window);
}

function formatError(cause: unknown): string {
  return cause instanceof Error ? cause.message : String(cause);
}

export async function loadSd11UpdateAction(
  request: Sd11UpdateActionRequest
): Promise<Sd11UpdateReleaseTruth> {
  if (!hasTauriRuntime()) {
    return {
      kind: 'check-failed',
      reason:
        'Desktop runtime boundary is unavailable, so governed SD-12 release truth cannot be proven from this context.',
      buildLabel: request.buildLabel,
      version: request.buildVersion,
    };
  }

  try {
    return await invoke<Sd11UpdateReleaseTruth>('sd11_update_action', { request });
  } catch (cause: unknown) {
    return {
      kind: 'check-failed',
      reason: `SD-11 update-action command failed: ${formatError(cause)}`,
      buildLabel: request.buildLabel,
      version: request.buildVersion,
    };
  }
}
