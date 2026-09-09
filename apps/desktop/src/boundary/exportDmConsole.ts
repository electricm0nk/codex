import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Write boundary for the DM console's standalone HTML export (v0.8 D-5 →
 * B-12 `export_dm_console`, `dm_console_export.rs`). Writes `html` verbatim
 * to `filePath`; the content is built entirely on this side by
 * `dmToolkit/dmConsoleExport.ts`.
 */
export interface ExportDmConsoleRequest {
  filePath: string;
  html: string;
}

export async function exportDmConsole(request: ExportDmConsoleRequest): Promise<void> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for exporting the DM console');
  }
  try {
    await invoke('export_dm_console', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to export the DM console: ${formatError(cause)}`);
  }
}
