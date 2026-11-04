import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CreateCharacterOutcome } from './loadCreateCharacter';

/**
 * Write desktop boundary for importing a PCGen character file.
 *
 * Invokes the `import_pcg` Tauri command, which runs the `pcg_import` sidecar on `filePath` (the app
 * itself never parses the file), imports the sidecar's JSON the way `import_character` imports a Codex
 * file, writes the character's bio, money and custom records, and returns the outcome with the report
 * of what became of each line of the file. A file the engine refuses saves nothing and still reports.
 * A file the sidecar refuses, or a missing sidecar, is a rejected promise naming the reason.
 */

export interface ImportPcgRequest {
  filePath: string;
  savedAt: string;
}

export interface PcgReportLine {
  key: string;
  from: string;
  outcome: string;
  to: string;
}

export interface PcgImportReport {
  sourceFile: string;
  gameMode: string;
  unknownKeys: string[];
  unmappedClasses: string[];
  unmappedRace: string | null;
  warnings: string[];
  lines: PcgReportLine[];
}

export interface ImportPcgResponse {
  outcome: CreateCharacterOutcome;
  report: PcgImportReport;
}

export const PCG_FILE_FILTER = { name: 'PCGen character', extensions: ['pcg'] };

/** How many lines of each outcome the report holds, in the order the outcomes first appear. */
export function pcgReportCounts(report: PcgImportReport): { outcome: string; count: number }[] {
  const counts = new Map<string, number>();
  for (const line of report.lines) {
    counts.set(line.outcome, (counts.get(line.outcome) ?? 0) + 1);
  }
  return [...counts].map(([outcome, count]) => ({ outcome, count }));
}

export async function importPcg(request: ImportPcgRequest): Promise<ImportPcgResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for importing a PCGen file');
  }

  try {
    return await invoke<ImportPcgResponse>('import_pcg', { request });
  } catch (cause: unknown) {
    throw new Error(formatError(cause));
  }
}
