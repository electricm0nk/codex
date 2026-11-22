import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { EquipmentCatalogResponse } from './loadEquipmentCatalog';

/**
 * Read-only desktop boundary over the filtered cross-book equipment catalog.
 *
 * Invokes the `list_equipment` Tauri command (Criterion 19), which narrows
 * `list_equipment_catalog`'s full corpus — all 3830 records across CRB, APG,
 * ACG, Bestiary 1, ARG and Pathfinder Unchained — by `nameContains`
 * (case-insensitive substring against `name`), `category` (exact match
 * against the `EquipmentCategory` variant name verbatim, e.g. "ArmsArmor")
 * and/or `book` (exact match against a wire code in
 * `EQUIPMENT_CATALOG_BOOKS`). Every field is optional — an all-`null` filter
 * is equivalent to the unfiltered `loadEquipmentCatalog`. Distinct call site
 * from `loadEquipmentCatalog`: this is what the `ItemPickerModal` uses to
 * narrow the catalog to a single category (e.g. arms & armor) before the
 * user searches further by name.
 */

export interface EquipmentCatalogFilter {
  nameContains: string | null;
  category: string | null;
  /**
   * Exact match against one of `equipment_catalog.rs`'s
   * `EQUIPMENT_CATALOG_BOOKS` codes ("CRB", "APG", "ACG", "B1", "ARG",
   * "PU"). Omitted or `null` spans every book, which is what every caller
   * that does not send this field gets — the same additive shape the Rust
   * `EquipmentCatalogFilter::book` uses.
   */
  book?: string | null;
}

export async function listEquipment(filter: EquipmentCatalogFilter): Promise<EquipmentCatalogResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for listing equipment');
  }

  try {
    return await invoke<EquipmentCatalogResponse>('list_equipment', { filter });
  } catch (cause: unknown) {
    throw new Error(`Failed to list equipment: ${formatError(cause)}`);
  }
}
