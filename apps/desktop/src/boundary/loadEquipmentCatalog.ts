import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary over the full CRB equipment catalog.
 *
 * Invokes the `list_equipment_catalog` Tauri command, which returns every
 * real corpus record in `rules_tables::crb::equipment_tables` (all ~2,977
 * records across all 4 core-rulebook categories) verbatim — not a
 * per-character sample. Distinct from `loadSavedCharacterDetail`'s Gear
 * tab data, which reflects only what one character has equipped.
 */

export interface EquipmentCatalogEntryDto {
  key: string;
  /** The `EquipmentCategory` variant name verbatim, e.g. "ArmsArmor". */
  category: string;
  name: string;
  costGp: number | null;
}

export interface EquipmentCatalogResponse {
  entries: EquipmentCatalogEntryDto[];
}

export async function loadEquipmentCatalog(): Promise<EquipmentCatalogResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for loading the equipment catalog');
  }

  try {
    return await invoke<EquipmentCatalogResponse>('list_equipment_catalog');
  } catch (cause: unknown) {
    throw new Error(`Failed to load equipment catalog: ${formatError(cause)}`);
  }
}
