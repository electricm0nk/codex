import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary over the full class progression catalog.
 *
 * Invokes the `list_class_catalog` Tauri command, which returns every real
 * corpus-grounded chassis row the command exposes, level 1 through each
 * class's `max_supported_level` ceiling, verbatim — not a per-character
 * sample. Distinct from the Character Sheet's chassis data, which reflects
 * only one character's own level.
 *
 * Sources, as of SD-27 2026-07-31: `rules_tables::crb::class_tables` (the
 * 11 CRB classes) and `rules_tables::pathfinder_unchained::class_chassis`
 * (the 4 Unchained classes, which are REPLACEMENTS for four of those and
 * appear beside them, never instead of them). The 16 APG/ACG classes are
 * not in this catalog yet. No row count is written down here on purpose —
 * the screen reads it off the response.
 */

export interface ClassCatalogEntryDto {
  /**
   * The class's display name — a `ClassId` variant name verbatim for the
   * CRB rows (e.g. "Fighter"), or the corpus `name` for a Pathfinder
   * Unchained row (e.g. "Unchained Monk"). Distinct per class, which is
   * what keeps a replacement pair from collapsing into one filter button.
   */
  classId: string;
  level: number;
  baseAttackBonus: number;
  fortSave: number;
  refSave: number;
  willSave: number;
}

export interface ClassCatalogResponse {
  entries: ClassCatalogEntryDto[];
}

export async function loadClassCatalog(): Promise<ClassCatalogResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for loading the class catalog');
  }

  try {
    return await invoke<ClassCatalogResponse>('list_class_catalog');
  } catch (cause: unknown) {
    throw new Error(`Failed to load class catalog: ${formatError(cause)}`);
  }
}
