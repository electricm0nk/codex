import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { EquipmentCatalogResponse } from './loadEquipmentCatalog';

/**
 * Read-only desktop boundary over the filtered CRB equipment catalog.
 *
 * Invokes the `list_equipment` Tauri command (Criterion 19), which narrows
 * `list_equipment_catalog`'s full corpus by `nameContains` (case-insensitive
 * substring against `name`) and/or `category` (exact match against the
 * `EquipmentCategory` variant name verbatim, e.g. "ArmsArmor"). Both fields
 * are optional — an all-`null` filter is equivalent to the unfiltered
 * `loadEquipmentCatalog`. Distinct call site from `loadEquipmentCatalog`:
 * this is what the `ItemPickerModal` uses to narrow the catalog to a single
 * category (e.g. arms & armor) before the user searches further by name.
 */

export interface EquipmentCatalogFilter {
  nameContains: string | null;
  category: string | null;
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
