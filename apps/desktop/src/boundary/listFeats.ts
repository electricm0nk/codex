import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary over the filtered feat catalog.
 *
 * Invokes the `list_feats` Tauri command (mirrors `listEquipment`'s exact
 * shape), which narrows the full 690-record catalog
 * (`list_feat_catalog`'s unfiltered source: 185 CRB + 172 APG + 129 ACG +
 * 187 ARG + 17 PU) by `nameContains` (case-insensitive substring against
 * `name`), `category` (exact match against the source book's
 * `FeatCategory` variant name verbatim, e.g. "Combat") and/or `source`
 * (exact match against the `RuleSetId` variant name, i.e. "Crb" / "Apg" /
 * "Acg" / "Arg" / "Pu"). Every field is optional and `null` matches
 * everything.
 */

export interface FeatCatalogEntryDto {
  key: string;
  /**
   * The source book's own `FeatCategory` variant name verbatim, e.g.
   * "Combat". `"Panache"` only ever appears on ACG records, `"Teamwork"`
   * only on APG/ACG/ARG ones, and `"Alignment"` / `"CombatStamina"` /
   * `"WoundThreshold"` only on PU ones (that book groups its feats by the
   * corpus file's own block markers, not by a `TYPE:` facet).
   */
  category: string;
  name: string;
  description: string | null;
  /**
   * Which book this feat is from: `"Crb"`, `"Apg"`, `"Acg"`, `"Arg"` or
   * `"Pu"`. Also the only thing separating the catalog's one repeated
   * key — `Endurance`, which PU re-lists from the Core Rulebook.
   */
  source: string;
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
  /**
   * `"Crb"` / `"Apg"` / `"Acg"` / `"Arg"` / `"Pu"`, or `null` to span every
   * book.
   */
  source?: string | null;
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
