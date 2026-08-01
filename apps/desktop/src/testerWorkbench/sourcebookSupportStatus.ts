/**
 * Which Pathfinder 1e sourcebooks the app actually carries content for,
 * for the Support Debt panel's "Other sourcebooks" list.
 *
 * ## Why this module exists
 *
 * The panel used to render a single hardcoded `NOT_STARTED_SOURCEBOOKS`
 * array that included the Advanced Player's Guide and the Advanced Class
 * Guide, each labelled "NOT STARTED". Both books had been ingested by
 * then — a tester reading that panel was told the app had no APG content
 * while the Spell, Equipment and Feat catalogs were serving hundreds of
 * APG records. Two more books (Advanced Race Guide, Pathfinder Unchained)
 * and Bestiary 1 were ingested afterwards and appeared on neither list, so
 * the panel simply did not mention them.
 *
 * Every count below is derived from the shipping tables, not from prose:
 *
 * - spells — the `SPELL_LIST` arrays in
 *   `src/rules_core/rules_tables/<book>/spell_list.rs`
 *   (CRB 652, APG 297, ACG 144, ARG 92; 1185 total, the number
 *   `spell_catalog.rs` pins). Bestiary 1 and Pathfinder Unchained have no
 *   `spell_list.rs` at all — PU's `pu_spells.lst` is entirely commented
 *   out in the corpus.
 * - equipment — the `equipment_data` tables per book
 *   (CRB 2977, APG 338, ACG 269, B1 4, ARG 200, PU 42; 3830 total, pinned
 *   by `equipment_catalog.rs`'s
 *   `catalog_spans_every_ingested_book_with_their_real_counts`).
 * - feats — pinned per-source by `feat_catalog.rs`
 *   (Crb 185, Apg 172, Acg 129, Arg 187, Pu 17; 690 total).
 * - races — the rows `race_catalog.rs` serves out of `data/corpus/<book>/`
 *   through `list_race_catalog`, pinned by its
 *   `every_book_code_is_a_declared_one_and_every_declared_code_is_present`
 *   (CRB 67, B1 106; 173 total across 18 races). Only the Core Rulebook and
 *   Bestiary 1 declare races; the Advanced Race Guide declares none of its
 *   own (SD-27 `decisions.md` §25) and so gains no race line here. The 153
 *   *alternate* racial traits ARG does declare are loaded but reach no
 *   catalog surface yet, so claiming them would overstate what a tester can
 *   actually browse.
 *
 * ## What "Ingested" claims, and what it deliberately does not
 *
 * "Ingested" says only that the book's listed records ship in the app and
 * are reachable in the catalogs. It makes no support-state claim: the
 * panel's audited rows are Core Rulebook subjects, so these books have no
 * support-debt rows yet, and the label must not imply they were assessed
 * and passed.
 */

export interface IngestedSourcebook {
  name: string;
  /** What this book actually contributes, with real counts. */
  contributes: string;
}

/**
 * Books with real, browsable content in the app. The Core Rulebook is not
 * here because the panel audits it directly in its own section above.
 */
export const INGESTED_SOURCEBOOKS: readonly IngestedSourcebook[] = [
  {
    name: "Advanced Player's Guide",
    contributes: '297 spells, 338 equipment records, 172 feats',
  },
  {
    name: 'Advanced Class Guide',
    contributes: '144 spells, 269 equipment records, 129 feats',
  },
  {
    name: 'Advanced Race Guide',
    contributes: '92 spells, 200 equipment records, 187 feats',
  },
  {
    name: 'Pathfinder Unchained',
    contributes: '42 equipment records, 17 feats (no spells — its spell list is commented out in the corpus)',
  },
  {
    name: 'Bestiary 1',
    contributes: '4 equipment records, 11 races (106 racial trait rows)',
  },
];

/**
 * Books with no rules table module in `src/rules_core/rules_tables/` at
 * all — genuinely not started, listed honestly rather than given a
 * fabricated per-item breakdown for content that does not exist.
 */
export const NOT_STARTED_SOURCEBOOKS: readonly string[] = [
  'Ultimate Combat',
  'Ultimate Magic',
  'Ultimate Equipment',
];

export interface SourcebookStatusRow {
  name: string;
  status: 'Ingested' | 'Not started';
  /** Empty for a not-started book: there is nothing real to describe. */
  detail: string;
}

/** The panel's rendered rows: every ingested book, then every untouched one. */
export const SOURCEBOOK_STATUS_ROWS: readonly SourcebookStatusRow[] = [
  ...INGESTED_SOURCEBOOKS.map(
    (book): SourcebookStatusRow => ({
      name: book.name,
      status: 'Ingested',
      detail: book.contributes,
    })
  ),
  ...NOT_STARTED_SOURCEBOOKS.map(
    (name): SourcebookStatusRow => ({ name, status: 'Not started', detail: '' })
  ),
];
