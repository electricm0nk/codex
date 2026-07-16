import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary over the full CRB spell catalog.
 *
 * Invokes the `list_spell_catalog` Tauri command, which returns every real
 * corpus record in `rules_tables::crb::spell_list::SPELL_LIST` (all 652
 * records across all 9 PF1 strict schools) verbatim — not a per-character
 * sample. Distinct from the Character Sheet's Spells tab data, which
 * reflects only what one character has selected.
 */

export interface SpellCatalogEntryDto {
  key: string;
  /** The `Pf1SchoolId` variant name verbatim, e.g. "Abjuration". */
  school: string;
  level: number;
  description: string;
}

export interface SpellCatalogResponse {
  entries: SpellCatalogEntryDto[];
}

export async function loadSpellCatalog(): Promise<SpellCatalogResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for loading the spell catalog');
  }

  try {
    return await invoke<SpellCatalogResponse>('list_spell_catalog');
  } catch (cause: unknown) {
    throw new Error(`Failed to load spell catalog: ${formatError(cause)}`);
  }
}
