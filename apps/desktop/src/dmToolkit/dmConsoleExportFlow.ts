import { save } from '@tauri-apps/plugin-dialog';
import { exportDmConsole, type ExportDmConsoleRequest } from '../boundary/exportDmConsole';
import { hasTauriRuntime } from '../boundary/runtime';

/** The Export flow behind the console's Export button (v0.8 D-5), deps injected for tests. */
export interface DmConsoleExportDeps {
  hasRuntime: () => boolean;
  save: (defaultFileName: string) => Promise<string | null>;
  exportDmConsole: (request: ExportDmConsoleRequest) => Promise<void>;
}

export type DmConsoleExportOutcome = { kind: 'Exported'; message: string } | { kind: 'Cancelled' } | { kind: 'Failed'; message: string };

export function dmConsoleFileName(campaignName: string): string {
  const safe = campaignName.replace(/[^a-z0-9-_ ]/gi, '').replace(/\s+/g, ' ').trim() || 'campaign';
  return `${safe} console.html`;
}

export async function runDmConsoleExport(
  target: { campaignName: string; html: string },
  deps: DmConsoleExportDeps,
): Promise<DmConsoleExportOutcome> {
  if (!deps.hasRuntime()) {
    return { kind: 'Failed', message: 'Exporting requires the desktop runtime.' };
  }
  try {
    const filePath = await deps.save(dmConsoleFileName(target.campaignName));
    if (!filePath) {
      return { kind: 'Cancelled' };
    }
    await deps.exportDmConsole({ filePath, html: target.html });
    return { kind: 'Exported', message: `Exported the console to ${filePath}.` };
  } catch (cause: unknown) {
    return { kind: 'Failed', message: cause instanceof Error ? cause.message : 'Could not export the console.' };
  }
}

/** Production wiring: real dialog, real runtime check, real command. */
export const DESKTOP_DM_CONSOLE_EXPORT_DEPS: DmConsoleExportDeps = {
  hasRuntime: hasTauriRuntime,
  save: (defaultFileName) => save({ defaultPath: defaultFileName, filters: [{ name: 'HTML', extensions: ['html'] }] }),
  exportDmConsole,
};
