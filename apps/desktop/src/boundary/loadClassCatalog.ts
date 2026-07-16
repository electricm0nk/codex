import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary over the full CRB class progression catalog.
 *
 * Invokes the `list_class_catalog` Tauri command, which returns every real
 * corpus-grounded row in `rules_tables::crb::class_tables::class_tables()`
 * (all 207 rows across all 11 classes, level 1 through each class's
 * `max_supported_level` ceiling) verbatim — not a per-character sample.
 * Distinct from the Character Sheet's chassis data, which reflects only one
 * character's own level.
 */

export interface ClassCatalogEntryDto {
  /** The `ClassId` variant name verbatim, e.g. "Fighter". */
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
