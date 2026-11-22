# Cycle receipt — `sd32-desktop-count-resweep`

Territory: `apps/desktop/src-tauri/src/**` (no `apps/desktop/src-tauri/tests/**` directory exists in
this workspace). Resolves the `closure-epilogue`-filed "desktop cargo suite regression,
`beginner_box` not swept into `CORPUS_BOOK_IDS`" blocker (`progress.md` `## Open blockers`).

## Reproduction — re-derived, not transcribed

`cd apps/desktop/src-tauri && cargo test --locked --bin codex-desktop` (env: `PCGEN_ORACLE_SHA`,
repo-local `PCGEN_CORPUS_ROOT`, `CARGO_INCREMENTAL=0`, scratch `CARGO_TARGET_DIR`) at the branch tip
before any edit: **542 passed, 6 failed** — not the filed blocker's 541/7. The 7th named test,
`corpus_ingest_diagnostic::tests::the_two_ingested_books_totals_reconcile_with_their_license_
artifacts`, already passed at reproduction time; its own count source (`rules_tables`, not a
filesystem walk of `data/corpus/`) is unaffected by either population change. **Instrument
correction, not a fix of mine**: the filed blocker's own count was one test stale by the time this
cycle picked it up. The 6 real failures:

- `equipment_catalog::tests::catalog_spans_every_ingested_book_with_their_real_counts` — 8119 vs
  pinned 8100
- `equipment_catalog::tests::description_coverage_is_pinned_per_book` — 4769 vs pinned 4756
- `equipment_catalog::tests::filter_equipment_catalog_matches_category_exactly_across_every_book` —
  1097 vs pinned 1095
- `equipment_catalog::tests::keys_do_not_collide_across_books_and_crbs_own_duplicates_are_pinned` —
  230 vs pinned 225
- `reach_gate::tests::dispatch_gap_race_and_monster_families_all_have_book_level_reach_arms` —
  `data/corpus/beginner_box/` unnamed
- `reach_gate::tests::the_inventory_is_populated_from_all_three_live_sources` — same

All six failed for the stated count/registration reason, not a logic reason — confirmed by reading
each panic message before touching code.

## Root cause, confirmed against the live tree

Only ONE of the two population changes named in the brief actually touches this workspace:
`beginner_box`'s ingestion. `advanced_race_guide`'s feat-self-erasure fix (48 `feat_gap` records)
touches `rules_tables`/`corpus_ingest_diagnostic.rs` machinery this territory does not own, and no
failing test here referenced an `advanced_race_guide` count — `corpus_ingest_diagnostic::tests::
advanced_race_guide_counts_match_the_real_underlying_tables` passed both before and after this
cycle's edits. That half of the brief's premise did not reproduce in this territory; reported as a
finding, not silently dropped.

`data/corpus/beginner_box/equipment/` holds 19 JSON files, already routed through the corpus gap
lane under book code `BB` (`src/rules_core/rules_tables/equipment_gap_tables.rs`, generated,
outside this territory) — but `beginner_box` was absent from `apps/desktop/src-tauri/src/
reach_gate.rs::CORPUS_BOOK_IDS`, and `reach_of` had no `("beginner_box", "equipment")` match arm.
`equipment_catalog.rs`'s book roster is derived (`equipment_catalog_books()`), so its BB rows
already flowed through automatically once the resolver had them — only the *pinned counts*
needed re-deriving there, not a registration.

No other roster in this workspace (`race_catalog.rs`, `feat_catalog.rs`, `spell_catalog.rs`,
`companion_catalog.rs`, `race_trait_picker.rs`, `corpus_ingest_diagnostic.rs`) enumerates
`beginner_box` or needs to: its only ingested content kind is `equipment`.

## Fix

- `reach_gate.rs`: added `("beginner_box", "beginner_box")` to `CORPUS_BOOK_IDS`, and a
  `("beginner_box", "equipment") => Some(equipment_reach("BB", BTreeSet::new()))` arm in `reach_of`
  — the identical no-hand-authored-table shape already proven for `UW`/`AG`/`ISM`.
- `equipment_catalog.rs`: re-derived every pin `beginner_box`'s 19 new `BB` rows shift, each with
  its own old→new and command, none deleted (per-file diff below).

## Every count changed — bucket: **reachability** (a real, already-ingested population reaching a
player surface for the first time; not a reclassification or an instrument fix)

| Assertion | Old | New | Command |
|---|---|---|---|
| `equipment_catalog::count_by_book(&response, "BB")` | (absent) | 19 | `grep -c 'book: "BB"' src/rules_core/rules_tables/equipment_gap_tables.rs` |
| `equipment_catalog::response.entries.len()` (total) | 8100 | 8119 | `cargo test --locked --bin codex-desktop equipment_catalog -- --nocapture` (per-book dump) |
| `equipment_catalog::with_description("BB")` | (absent) | 13 | `grep 'book: "BB"' equipment_gap_tables.rs \| grep -c 'description: Some'` |
| `equipment_catalog` description total | 4756 | 4769 | same per-book dump |
| `filter_equipment_catalog` ArmsArmor total | 1095 | 1097 | same dump (BB has 2 ArmsArmor rows) |
| `keys_do_not_collide` `cross_book.len()` | 225 | 230 | temporary `cross_book.difference(&expected_cross_book)` dump — 5 new BB-involving keys (`Bandages of Rapid Recovery`, `Campfire Bead`, `Dawnflower Sash`, `Flying Ointment`, `Glowing Glove`), every one verified against `equipment_gap_rows()` by the test's own existing loop |

Cross-checked: `keys_do_not_collide`'s `intra_book_dupes` (317) and
`intra_book_dupes_outside_crb` (`UE`/`Masterwork Tool` only) were unaffected — BB has no internal
key duplicates.

## Verification

`cd apps/desktop/src-tauri && cargo test --locked --bin codex-desktop` → **548 passed, 0 failed, 0
ignored** (≥548, matching "nothing was removed" — no test deleted, one book newly registered).

## Dual-audit gate (own diff, working-tree vs pre-edit HEAD)

`git diff --unified=0 -- apps/desktop/src-tauri/src/equipment_catalog.rs apps/desktop/src-tauri/src/
reach_gate.rs` scanned for bundle tags and stub tokens: `OK_NO_BUNDLE_TAGS`, `OK_NO_TOKENS`.

## Out-of-territory note

`declared-pi-audit` timeout (the second `## Open blockers` entry filed the same day) is lane H's
(`scripts/**`) — not touched here.

## Commit

`21bef06d95` — "fix(desktop): register beginner_box in CORPUS_BOOK_IDS, re-derive shifted
equipment_catalog pins"
