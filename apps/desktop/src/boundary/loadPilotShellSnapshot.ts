import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

export type ReceiptStatus = 'Computed' | 'Blocked' | 'Unknown/Unavailable';

export interface PilotShellSnapshot {
  caseId: string;
  sourcePackageId: string;
  receiptStatus: ReceiptStatus;
  summaryValues: Record<string, string> | null;
  diagnostics: string[];
  explanationRefs: string[];
  dataSource: 'scaffold-placeholder' | 'tauri-command';
  note: string;
}

const scaffoldPlaceholder: PilotShellSnapshot = {
  caseId: 'ge07-e1-scaffold-placeholder',
  sourcePackageId: 'pending-real-ge06-source-package',
  receiptStatus: 'Unknown/Unavailable',
  summaryValues: null,
  diagnostics: [
    'This scaffold is additive only.',
    'Real GE-06 pilot data is not wired in this slice.',
  ],
  explanationRefs: ['future/load_pilot_shell_snapshot'],
  dataSource: 'scaffold-placeholder',
  note: 'Future slices should replace this placeholder with a read-only Tauri command backed by the headless core.',
};

export async function loadPilotShellSnapshot(): Promise<PilotShellSnapshot> {
  if (!hasTauriRuntime()) {
    return scaffoldPlaceholder;
  }

  try {
    return await invoke<PilotShellSnapshot>('load_pilot_shell_snapshot');
  } catch (cause: unknown) {
    return {
      ...scaffoldPlaceholder,
      diagnostics: [...scaffoldPlaceholder.diagnostics, `Tauri boundary fallback: ${formatError(cause)}`],
    };
  }
}
