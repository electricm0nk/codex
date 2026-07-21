import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { SpellCatalogResponse } from './loadSpellCatalog';

/**
 * Read-only desktop boundary over the filtered CRB spell catalog.
 *
 * Invokes the `list_spells` Tauri command (Criterion 19), which narrows
 * `list_spell_catalog`'s full corpus by `nameContains` (case-insensitive
 * substring against the spell's `key`) and/or `school` (exact match against
 * the `Pf1SchoolId` variant name verbatim, e.g. "Evocation"). Both fields
 * are optional — an all-`null` filter is equivalent to the unfiltered
 * `loadSpellCatalog`. Distinct call site from `loadSpellCatalog`: this is
 * what the `ItemPickerModal` uses when the user opens the Add Spell picker.
 */

export interface SpellCatalogFilter {
  nameContains: string | null;
  school: string | null;
}

export async function listSpells(filter: SpellCatalogFilter): Promise<SpellCatalogResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for listing spells');
  }

  try {
    return await invoke<SpellCatalogResponse>('list_spells', { filter });
  } catch (cause: unknown) {
    throw new Error(`Failed to list spells: ${formatError(cause)}`);
  }
}
