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

/**
 * One feat's prerequisite verdict for the character the picker is open for.
 *
 * Present only on `listFeatsForCharacter` responses; `list_feats` /
 * `list_feat_catalog` omit the key entirely (not `null`) because they are
 * served with no character context at all, and a `null` would read to a
 * `!== undefined` check as "checked, and fine".
 *
 * `eligible: false` is the only thing that greys a row out. `unverified`
 * lists prerequisites the engine could not evaluate — those never block, and
 * are shown as a note so the player is told what was not checked rather than
 * quietly allowed or quietly denied.
 */
export interface FeatEligibilityDto {
  eligible: boolean;
  /** One joined line for the greyed row. `null` exactly when `eligible`. */
  unavailableReason: string | null;
  met: string[];
  unmet: string[];
  unverified: string[];
  /** `0` means the corpus record genuinely carries no prerequisite. */
  prerequisiteCount: number;
}

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
  /**
   * Present only when the catalog was requested for a specific saved
   * character (`listFeatsForCharacter`). Absent on the character-less
   * `listFeats`.
   */
  eligibility?: FeatEligibilityDto;
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

/**
 * The same catalog, evaluated against one saved character's real corpus feat
 * prerequisites.
 *
 * **This is what closes the "no feat prerequisite enforcement anywhere"
 * defect at the UI.** Before it, the picker offered all 690 feats to every
 * character and the mutation accepted every one — a Fighter 1 with a +1 base
 * attack bonus could take Improved Two-Weapon Fighting (BAB +6, Dex 17,
 * Two-Weapon Fighting).
 *
 * Every one of the 690 records still comes back. An unavailable feat must be
 * *visibly* unavailable with its reason, not missing from the list: dropping
 * the rows would hide the rules from the player instead of teaching them.
 */
export async function listFeatsForCharacter(
  characterId: string,
  filter: FeatCatalogFilter
): Promise<FeatCatalogResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for listing feats');
  }

  try {
    return await invoke<FeatCatalogResponse>('list_feats_for_character', { request: { characterId, filter } });
  } catch (cause: unknown) {
    throw new Error(`Failed to list feats for this character: ${formatError(cause)}`);
  }
}
