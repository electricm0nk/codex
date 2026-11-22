import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary over the full race trait catalog.
 *
 * Invokes the `list_race_catalog` Tauri command, which returns every real
 * corpus-grounded trait row the adapter serves, verbatim — not a
 * per-character sample. Distinct from the Character Sheet's race data,
 * which reflects only one character's own chosen race.
 *
 * No roster or row count is pinned here on purpose. The adapter's race
 * coverage widens (SD-27 takes it past the original 7 Core Rulebook races),
 * and `RaceCatalogScreen` derives its race list, per-race counts and summary
 * line from the rows that actually arrive rather than from a compiled-in
 * list, so nothing on the frontend needs to be re-edited to stay true.
 */

export interface RaceCatalogEntryDto {
  /**
   * The race's corpus key with separators removed, e.g. "HalfElf",
   * "Svirfneblin". For the seven Core Rulebook races this is exactly the
   * `RaceId` variant name.
   */
  raceId: string;
  traitName: string;
  value: number;
  detail: string;
  /**
   * The short code of the ingested book this race came from — "CRB" or
   * "B1" today, the same codes the Equipment and Spell catalogs emit.
   * Added by `race_catalog.rs` when the catalog widened past the Core
   * Rulebook, and always present on the wire; a surface that wants to label
   * or filter by sourcebook reads it rather than guessing.
   */
  book: string;
}

export interface RaceCatalogResponse {
  entries: RaceCatalogEntryDto[];
  /**
   * Corpus files the adapter could not read. Empty in a healthy checkout —
   * carried so a shrunken catalog can report why it shrank instead of
   * silently serving less than it claims to.
   */
  diagnostics: string[];
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
