import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary over the filtered CRB feat catalog.
 *
 * Invokes the `list_feats` Tauri command (mirrors `listEquipment`'s exact
 * shape), which narrows the full 185-record CRB feat catalog
 * (`list_feat_catalog`'s unfiltered source) by `nameContains`
 * (case-insensitive substring against `name`) and/or `category` (exact
 * match against the `FeatCategory` variant name verbatim, e.g. "Combat").
 * Both fields are optional.
 */

export interface FeatCatalogEntryDto {
  key: string;
  /** The `FeatCategory` variant name verbatim, e.g. "Combat". */
  category: string;
  name: string;
  description: string | null;
  /**
   * `'Weapon'`, `'Skill'` or `'SpellSchool'` for a feat whose chosen target
   * the engine consumes; `null` for every other feat.
   *
   * Narrower than the corpus on purpose: more feats carry a `CHOOSE:` token,
   * but only these have a producer that reads the target, so a prompt shown
   * for one always leads to real arithmetic.
   */
  chooserTargetKind: string | null;
}

export interface FeatCatalogFilter {
  nameContains: string | null;
  category: string | null;
}

export interface FeatCatalogResponse {
  entries: FeatCatalogEntryDto[];
}

export async function listFeats(filter: FeatCatalogFilter): Promise<FeatCatalogResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for listing feats');
  }

  try {
    return await invoke<FeatCatalogResponse>('list_feats', { filter });
  } catch (cause: unknown) {
    throw new Error(`Failed to list feats: ${formatError(cause)}`);
  }
}
