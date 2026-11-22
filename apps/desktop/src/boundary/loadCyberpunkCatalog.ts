import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Desktop boundary over the Cyberpunk RED catalogs (SD-38 E6.1):
 * `list_cyberpunk_catalog` (`apps/desktop/src-tauri/src/cpr_catalog.rs`) returns every
 * record of one corpus MANIFEST kind from `data/cyberpunk-red/corpus`. Records are the
 * corpus rows as they are; a kind the MANIFEST does not list is refused by the command.
 */

export type CyberpunkRecord = { kind: string; name?: string; [field: string]: unknown };

export interface CyberpunkCatalogResponse {
  kind: string;
  /** The corpus root the records were read from (repo-relative). */
  source: string;
  /** The MANIFEST's pinned record count for the kind. */
  expected: number;
  /** A print-only kind is a GM reference page rather than a player-facing catalog. */
  printOnly: boolean;
  records: CyberpunkRecord[];
}

export async function loadCyberpunkCatalog(kind: string): Promise<CyberpunkCatalogResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for loading the Cyberpunk RED catalog');
  }

  try {
    return await invoke<CyberpunkCatalogResponse>('list_cyberpunk_catalog', { kind });
  } catch (cause: unknown) {
    throw new Error(`Failed to load the Cyberpunk RED catalog: ${formatError(cause)}`);
  }
}
