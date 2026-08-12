import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary over the full cross-book equipment catalog.
 *
 * Invokes the `list_equipment_catalog` Tauri command, which returns every
 * real corpus record across all six ingested books verbatim — CRB (2977),
 * APG (338), ACG (269), Bestiary 1 (4), ARG (200) and Pathfinder Unchained
 * (42), 3830 in total — not a per-character sample. Those per-book totals
 * are pinned by `equipment_catalog.rs`'s
 * `catalog_spans_every_ingested_book_with_their_real_counts`. Distinct from
 * `loadSavedCharacterDetail`'s Gear tab data, which reflects only what one
 * character has equipped.
 */

/**
 * One of the wire codes `equipment_catalog.rs` exports as
 * `EQUIPMENT_CATALOG_BOOKS`: `"CRB"`, `"APG"`, `"ACG"`, `"B1"`, `"ARG"` or
 * `"PU"`. Left as `string` rather than a closed union so a book added on the
 * Rust side arrives intact instead of failing to type — the screen falls back
 * to rendering the raw code when it has no label for one.
 */
export type EquipmentBookDto = string;

export interface EquipmentCatalogEntryDto {
  /**
   * The record's corpus identity — its `KEY:` token when the row carries
   * one, else its display name. Unique across books, but **not** unique
   * within CRB: 316 CRB keys appear twice (e.g. `Holy Symbol (Silver)`), a
   * pre-existing property of `crb::equipment_tables` pinned by
   * `keys_do_not_collide_across_books_and_crbs_own_duplicates_are_pinned`.
   * So `key` alone is not a safe React list key here.
   */
  key: string;
  /**
   * The `EquipmentCategory` variant name verbatim, e.g. "ArmsArmor".
   * Always `"Equipmods"` for `PU`, whose only ingested equipment content is
   * `pu_equipmods.lst` and which has no category enum of its own.
   */
  category: string;
  name: string;
  /**
   * `null` where the corpus row carries no flat gp cost — genuinely absent,
   * never a fabricated 0. All 42 PU rows are `null` for this reason.
   */
  costGp: number | null;
  /** Which ingested book this record came from. */
  book: EquipmentBookDto;
  /**
   * The record's corpus `DESC:` prose, already rendered on the Rust side by
   * `equipment_catalog.rs`'s `serve_description` (the same
   * `render_pcgen_desc` treatment the spell catalog uses, which is what
   * strips the raw `%%` escapes 54 records used to leak). **Safe to render
   * verbatim — do not re-process it here.**
   *
   * `null` where the corpus row genuinely carries no description, which is
   * a real and documented gap for template/bookkeeping rows, never a
   * fabricated placeholder. 2856 of the 3830 served records carry one;
   * the remaining 974 are honestly `null` and must render as *nothing*
   * rather than as an invented line of text.
   *
   * **Required, not optional.** An optional field would let a fixture omit
   * it and still type-check, which is precisely how this field came to be
   * rendered by the Rust adapter and read by nothing: it crossed the IPC
   * boundary and no consumer was obliged to notice. `null` is the way to
   * say "this record has no description"; leaving it out is not.
   */
  description: string | null;
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
