import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary over the full spell catalog.
 *
 * Invokes the `list_spell_catalog` Tauri command, which returns every real
 * corpus record across all four ingested books verbatim — CRB (652), APG
 * (297), ACG (144) and ARG (92), 1185 in total — not a per-character
 * sample. Those counts are pinned Rust-side by
 * `the_catalog_serves_every_ingested_book_not_only_crb` in
 * `spell_catalog.rs`. Pathfinder Unchained is absent because its
 * `pu_spells.lst` is entirely commented out, so the book defines no spell
 * of its own. Distinct from the Character Sheet's Spells tab data, which
 * reflects only what one character has selected.
 */

/** `"CRB"`, `"APG"`, `"ACG"` or `"ARG"` — see `spell_catalog.rs`. */
export type SpellBookDto = string;

export interface SpellCatalogEntryDto {
  /**
   * The record's corpus identity — its `KEY:` token when the corpus row
   * carries one, else its display name. Unique across all four books, so
   * it is safe to use as a resolution key and a React list key.
   */
  key: string;
  book: SpellBookDto;
  /**
   * The `Pf1SchoolId` variant name verbatim, e.g. "Abjuration".
   *
   * `null` for the 16 APG records ingested without one — a genuine
   * absence in the data, not a loading state. Render the absence; never
   * substitute a plausible-looking school.
   */
  school: string | null;
  /** `null` for the 41 APG records ingested without a spell level. */
  level: number | null;
  /** `null` for the 12 APG records ingested without description text. */
  description: string | null;
  /**
   * The corpus's own `DURATION:` formula, rendered as literal text ("N
   * <unit> per caster level") when it matches a caster-level-LINEAR shape
   * (SD31-E6-F2-006). `null` both for a flat/instantaneous/permanent
   * duration (most records) and for a formula this catalog does not
   * attempt (`min(`/`max(`/an additive term) — never a resolved live
   * number, since a spell's actual duration depends on the casting
   * character's caster level, which this reference catalog has no
   * character context for.
   */
  duration: string | null;
  /**
   * The corpus's own `RANGE:` keyword, rendered as literal text ("N ft. +
   * N ft. per [N] caster level(s)") when it names one of the three PF1
   * caster-level-linear range keywords — `Close`, `Medium`, `Long`
   * (SD31-E6-F2-008). `null` both for a range that is not one of those
   * three keywords (`Personal`, `Touch`, a literal distance, "See text",
   * ...) and — same posture as `duration` above — never a resolved live
   * number, since the casting character's actual range in feet depends on
   * a caster level this reference catalog has no character context for.
   */
  range: string | null;
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
