import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary over the full CRB race trait catalog.
 *
 * Invokes the `list_race_catalog` Tauri command, which returns every real
 * corpus-grounded trait row in `rules_tables::crb::race_tables::race_traits()`
 * (all 49 rows across all 7 races) verbatim — not a per-character sample.
 * Distinct from the Character Sheet's race data, which reflects only one
 * character's own chosen race.
 */

export interface RaceCatalogEntryDto {
  /** The `RaceId` variant name verbatim, e.g. "HalfElf". */
  raceId: string;
  traitName: string;
  value: number;
  detail: string;
}

export interface RaceCatalogResponse {
  entries: RaceCatalogEntryDto[];
}

export async function loadRaceCatalog(): Promise<RaceCatalogResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for loading the race catalog');
  }

  try {
    return await invoke<RaceCatalogResponse>('list_race_catalog');
  } catch (cause: unknown) {
    throw new Error(`Failed to load race catalog: ${formatError(cause)}`);
  }
}
