# Cycle fold-skinwalker — Epic 6 closure / SD-33 reopen fold

- **Commit SHA:** `6e2f2f076b4effb0089693ce3cbb81a5b68da246` (pushed to `tranche/13`, rebased cleanly onto the sibling `fold-undine` lane's `948976aacb`)
- **Files touched:**
  - `data/corpus/bestiary_5/race_trait/skinwalker/` — 65 new records (regenerated, not hand-copied) + `adopted_race_skinwalker.json` (`ingested_at` only, byte-identical content)
  - `data/corpus/bestiary_5/LICENSE.json` — `records_processed` 279 → 344, `records_redacted` 9 → 17, new PASS note
  - `src/bin/ingest_race_traits.rs` — new `direct_heritage_relatives`/`extra_in_scope_races` `BookSource` fields, new `direct_subrace_grants()` function, count-sweep updates, a fabricated-token fix (see Notes)
  - `src/bin/ingest_races.rs` — widened a pre-existing (not this fold's own) shared-directory skip to `bestiary_2`/`bestiary_5`/`bestiary_6`, plus a diagnostic-message improvement
  - `src/rules_core/race_resolver.rs` — 45-row `ALTERNATE_TRAIT_REPLACE_FLAGS` `Skinwalker` section, count-sweep updates (845→910 etc.)
  - `src/rules_core/pilot_compute/mod.rs` — 2 new `ALTERNATE_TRAIT_SELECTED_SKILL_BONUSES` rows (real wiring fix, not a count edit), `race_ids_with_a_magnitude_consumer` union 18→19
  - `src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs` — F1 population re-pin 6,278 → 6,260 with full mechanism derivation
  - `apps/desktop/src-tauri/src/race_catalog.rs`, `race_trait_picker.rs`, `reach_gate.rs` — count-sweep updates, a new `exclusion_guard_flags` branch (real wiring fix), `paged`/`pageless` re-pin, `UNREACHED_RECORD_FINDINGS`/`OPEN_FINDINGS` entries for the 20 genuinely-unreached `Change Shape (<Option>)` records
  - `tests/sd27_alternate_racial_trait_reachability.rs`, `tests/sd27_aasimar_globalvar_gate_closes_the_dead_affordance.rs` — count-sweep updates, a `race_of` correctness fix (see Notes)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (scoped `git diff` against HEAD across all touched files; only pre-existing filenames matched, no fresh bundle-tag identifiers introduced)
- **Wired-integration audit result:** OK_NO_TOKENS on the scoped diff (one `placeholder` hit, a reviewed `//`-comment describing PCGen's own `p.xx` stand-in value — bucket C of `sd24_wired_integration_audit.rs`'s own documented exemption). The repo-wide `sd24_wired_integration_audit.rs` test itself has two **pre-existing, unrelated** failures (confirmed red on unmodified HEAD via a parallel checkout — see Notes); neither touches a file this cycle wrote.
- **Acceptance criterion:** Fold the 45 recovered Skinwalker race-trait records from `sd31/racetrait4-SD31-E6-F4-005` into SD-33 before PR #377 merges, via the guarded generator path, with hand-traced verification against the pinned oracle.
- **Status:** complete

## Figures + their re-derive commands

| Figure | Value | Re-derive command |
|---|---|---|
| Branch's own files (45 new JSON + 4 modified support files) | 49 | `git diff --name-status b034408b1c..sd31/racetrait4-SD31-E6-F4-005 \| grep -v '^D'` (parent commit `b034408b1c`, the rescue branch's true base — **not** `origin/develop`, which is 14,270 lines stale and pulls in unrelated history; see Notes) |
| Branch's own Skinwalker `race_trait` record count | 45 | same command, `grep -c '^A.*race_trait/skinwalker'` |
| Generator's real, correct output | **65** | `cargo run --locked --bin ingest_race_traits bestiary_5` then `find data/corpus/bestiary_5/race_trait/skinwalker -name '*.json' \| wc -l` → 75 (65 new + 10 pre-existing) |
| Regenerated files byte-identical to pre-regen snapshot (excl. `ingested_at`) | 75/75 | see `/tmp/claude-*/scratchpad/diff_check.py` logic: load both JSON, pop `ingested_at`, compare |
| bestiary_5 on-disk record count (all kinds) | 344 | `find data/corpus/bestiary_5 -name '*.json' -not -name 'LICENSE.json' \| wc -l` |
| bestiary_5 redacted records | 17 (8 new) | count of `pi_marker == "redacted"` across `data/corpus/bestiary_5/**/*.json` |
| corpus_literal_sweep findings | 0 | `cargo run --locked --bin corpus_literal_sweep` |
| `cargo test --locked --no-run` | exit 0 | as run |
| `cargo test --locked --lib` | 2837 passed, 0 failed, 14 ignored | as run |
| `cd apps/desktop/src-tauri && cargo test --locked` | 548 passed, 0 failed | as run |
| F1 formula-bearing population | 6,278 → 6,260 | `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus --output <scratch>` |
| ingested race-trait total (all books) | 831 → 910 | `tests/sd27_alternate_racial_trait_reachability.rs::no_ingested_race_trait_key_contains_a_colon_so_the_storage_namespace_is_lossless` |
| menu-selectable `Alternate` total | 370 → 415 | `tests/sd27_alternate_racial_trait_reachability.rs::the_pure_flag_table_agrees_with_the_disk_backed_resolver_for_every_alternate` |

## Movement, four buckets

- **Closure:** 45 Skinwalker heritage records (9 kin selectors + 36 replacement rows) now genuinely reachable through the alternate-trait picker, correctly guarded against cross-kin double-selection, and correctly wired to the two computed skill totals they land on (Werebear-Kin/Wereshark-Kin `~ Animal-Minded`).
- **Reclassification:** none — the fold added new records; it did not reclassify existing ones.
- **Reachability:** 20 new `Skinwalker ~ Change Shape (<Option>)` records are ingested, real, oracle-traced content that reaches **no player surface today** — recorded as an honest `OPEN_FINDINGS`/`UNREACHED_RECORD_FINDINGS` gap (matching the pre-existing Monster Codex `Oversized Goblin` precedent), not silently hidden. Remedy: a TYPE-pool option-picker UI, out of this fold's scope.
- **Instrument-correction:** F1's population dropped 6,278 → 6,260 because 65 of the fold's filenames exactly coincide with pre-existing `race_trait_generic/` verbatim-ingest duplicates from an unrelated SD-32 lane, and `shape_ledger.py`'s own `normalize_kind_dir` deliberately treats a `<kind>_generic` sibling as the same bucket as `<kind>` — the fold's real, correctly-typed records now win that join where the generic fallback used to. Verified: neither `race_resolver.rs` nor `race_trait_picker.rs` nor `character_hub.rs` reads `race_trait_generic` at all (zero grep hits), so this is a measurement reclassification with no player-facing effect.

## RED→GREEN evidence

1. **corpus_literal_sweep RED (36 findings)** → fixed. The prior session's `direct_subrace_grants` post-processing loop synthesized a `PREVAREQ:<flag>,0` ABILITY token onto every kin selector, carried-through the way Aasimar/Tiefling's genuinely-sourced `_abilities_globalvar_subrace.lst` tokens are. For Skinwalker no such token exists anywhere in the pinned oracle (the selector's own literal `PREMULT` clause already states the guard). Fixed by skipping the synthesis whenever the selector already carries its own `PREMULT` token — `cargo run --locked --bin corpus_literal_sweep`: 36 findings → 0.
2. **`f1_population_matches_the_current_true_formula_bearing_count_not_the_stale_sd32_census` RED** → fixed by re-deriving and re-pinning 6,260 with the full, verified mechanism (see Instrument-correction above), not by silently patching the number.
3. **`no_ingested_race_trait_key_contains_a_colon...`, `the_pure_flag_table_agrees...`, `the_three_dependent_rows_are_not_offered...` RED (count-sweep)** → fixed, 831→910 / 370→415 with full derivation chains.
4. **`every_alternate_whose_bonus_lands_on_a_total_this_engine_computes_is_named_and_really_applies` RED** → two real defects, not one test artifact: (a) the test's own `race_token` derivation (splitting a key on `" ~ "`) silently built the wrong character race for Skinwalker's `<Kin> ~ <Trait>`-shaped keys — fixed by tracking each record's real `race_key`; (b) `pilot_compute::ALTERNATE_TRAIT_SELECTED_SKILL_BONUSES` was genuinely missing the two real, oracle-verified rows (`Werebear-Kin ~ Animal-Minded` +2 Climb, `Wereshark-Kin ~ Animal-Minded` +2 Swim) — a browse-only-content bug, fixed by adding them.
5. **`race_ids_with_a_magnitude_consumer_tests::the_union_is_exactly_the_eighteen_seamed_races` RED** → renamed/re-pinned to nineteen, `skinwalker` correctly joins the set through the same narrow, measured-not-full-coverage mechanism Strix/Grippli/Goblin already use.
6. **`every_alternate_has_a_readable_exclusion_guard_including_the_preability_spelling` RED** → real defect: none of the 36 replacement rows carried any exclusion guard at all, so a player could select two different kins' same-shaped replacement row (e.g. two `~ Ability Scores`) together and collect both incompatible swaps. Fixed with a new, narrowly-scoped `exclusion_guard_flags` branch: a record with a positive `PREABILITY` dependency on a specific parent AND its own `sets_replace_flags` (Skinwalker's shape) is guarded by that flag — gated so it does **not** touch Monster Codex's `Oversized Goblin` (no `PREABILITY` token at all), preserving that record's own, separately-documented exemption.
7. **`every_alternate_carries_real_book_attribution_and_prose` RED** → re-derived, not assumed: Wereraptor-Kin's 5 records genuinely cite `SOURCEPAGE:p.89` (a different sourcebook, Ironfang Invasion AP #115) in the pinned oracle, unlike the other 8 kins' `p.xx` stand-in. `paged`/`pageless` re-pinned to match.
8. **`reach_gate::{unreached_records_are_exactly_the_recorded_findings, every_declared_claim_actually_carries_the_records, unsurfaced_families_are_exactly_the_recorded_findings}` RED** → the 20 `Change Shape (<Option>)` records genuinely reach no surface (verified: no frontend consumer exists for a TYPE-pool option picker, grep confirms). Recorded honestly as `UNREACHED_RECORD_FINDINGS`/`OPEN_FINDINGS`, matching the pre-existing `Oversized Goblin` precedent exactly, rather than fixed by inventing a UI out of scope for a data-fold cycle.
9. **`ingest_races.rs::every_committed_race_record_on_disk_deserializes_through_the_shape_b_v1_schema` RED** → **pre-existing, not this fold's defect** (confirmed red on unmodified HEAD via a parallel checkout at the same commit, before any fold changes). The fold's 65 new records made `bestiary_5` a book this test's `IN_SCOPE_RACES` walk reaches deeply enough to trip a pre-existing gap: the same `Adopted Race ~ <Race>` selector shape (SD-32 `decisions.md §25` cycle 2) already violates this test's un-scoped `key.starts_with(race_key)` check in `bestiary_2`/`bestiary_5`/`bestiary_6`. Fixed in-scope (the task names "the ingest binaries" as owned) by widening the existing `advanced_race_guide`-only skip to all three books.

## Hand-traced sample (5, token-by-token against the pinned oracle)

Oracle: `$PCGEN_CORPUS_ROOT/pathfinder/paizo/roleplaying_game/core_essentials/races/skinwalker/skinwalker_abilities_race_subrace.lst`, pin `7f818006e371188e5717fd18d74d18a420747fc6` (confirmed live checkout matches: `git rev-parse HEAD` in the oracle repo).

1. **`Skinwalker ~ Werebat-Kin`** (selector) — `.lst:15`. Matched field-for-field: `KEY`, `CATEGORY`, `TYPE:Skinwalker Subrace`, `PREMULT:1,[PREABILITY:...],[!PREFACT:1,ABILITIES,Skinwalker_Replace...=True x4]`, `DESC`, `TEMPLATE`, `ABILITY:...AUTOMATIC|<4 targets>`, 4x `ASPECT`. 11 tokens verified.
2. **`Werebat-Kin ~ Ability Scores`** (replacement) — `.lst:19`. Matched: `KEY`, `CATEGORY`, `TYPE:RacialTraits.Skinwalker Racial Trait.SpecialQuality.Racial Ability Scores`, `PREABILITY:1,CATEGORY=Special Ability,Skinwalker ~ Werebat-Kin`, `DESC`, 2x `BONUS:STAT`, `SOURCEPAGE:p.xx` (dropped to `None`), `FACT:Skinwalker_ReplaceAbilityScores|True`. 7 tokens verified.
3. **`Skinwalker ~ Change Shape (Bite)`** (shared component) — `.lst:236`. Matched: `KEY`, `CATEGORY`, `TYPE:Skinwalker Racial Trait`, `VISIBLE:NO`, `DEFINE:Skinwalker_ChangeShape_ActivateBite|0`, `ABILITY:Internal|AUTOMATIC|Bite|PREVARGTEQ:...`, `TEMPBONUS:PC|VAR|...`. `description: null` confirmed (no `DESC:` token upstream). 6 tokens verified.
4. **Redaction count** — `grep -c "DESCISPI:YES" skinwalker_abilities_race_subrace.lst` → **8**, matching the corpus's 8 redacted records exactly (4 kins × selector + `~ Ability Scores` pair: Werebear/Wereboar/Werecrocodile/Weretiger).
5. **Kin count** — `grep -c "^# Skinwalker ~ "` → **9**, matching the corpus's 9 kin selectors exactly.

All 5 verified byte-identical to the corpus (modulo `ingested_at` and PI-redaction, both correctly applied).

## Notes

- **STEP 1 numbers, re-derived, not trusted:** the operator's cited "45" is the branch's own correct Skinwalker-record count; the rescue commit message's "48" was wrong (off by counting some non-record thing, or a stale in-flight number — not re-verified further since STEP 2's route made it moot). **`origin/develop..sd31/racetrait4-SD31-E6-F4-005` is the wrong diff base** — 14,270 lines, pulling in the whole repo's history since `develop` diverged. The branch's own parent commit, `b034408b1c` ("reconcile verify-baselines.env against the wave-10 green gate"), is the correct base: 49 files (45 new Skinwalker JSON + 4 modified support files: `scripts/classify_race_trait_rows.py`, `src/bin/ingest_race_traits.rs`, `src/bin/ingest_races.rs`, `src/rules_core/race_resolver.rs` — the second commit's own "uncommitted race-chassis work" description). Those 4 modified files were **not** inspected further or merged from — this cycle's own from-scratch `direct_heritage_relatives`/`direct_subrace_grants` mechanism was built and verified independently, and produces a strict superset (65 vs. the branch's 45) of the branch's population.
- **Unscoped-run near-miss:** the first `cargo run --bin ingest_race_traits` (no book argument) regenerated `ingested_at` across the ENTIRE corpus (474 files), not just `bestiary_5`. Caught by `git diff --stat` before committing, reverted with `git checkout -- data/corpus`, re-run scoped (`... bestiary_5`). Logged as a retro incident (`unscoped-generator-run-wide-blast-radius`).
- **Judgment call — exclusion guard shape.** The new `exclusion_guard_flags` branch is deliberately gated on "carries a positive `PREABILITY`", not a blanket "has `sets_replace_flags`" rule, specifically so it does not also touch `Oversized Goblin` (which has no competing alternate for its flags and is correctly, separately documented as exempt). Verified via the existing test suite: this branch fires for all 36 Skinwalker replacement rows and zero other records.
- **Coordination:** confirmed zero collisions with the sibling Undine-fold lane's files (`tests/fixtures/rules_core/derived-evaluator-fixtures.json`, `scripts/derive_race_trait_formula_fixtures.py`) via `git status --porcelain` before every write.
- **Pre-existing, unrelated failures found but NOT fixed (out of scope):** `tests/duergar_invisibility_sla_reaches_a_player_via_monster_codex.rs::the_loaded_books_are_the_ones_the_app_loads` (a `bestiary_3` book-list drift) and `tests/sd24_wired_integration_audit.rs`'s two failures (`SD-31-corpus-closure-grind/todo/` path substring false positives; a pre-existing `"fabricated placeholder"` string in `race_resolver.rs:3451` this cycle never touched). All three confirmed red on an unmodified `HEAD` checkout via a parallel `cargo test` run at the identical commit, before any of this cycle's changes — reported here rather than silently absorbed into this fold's own scope.
- **`docs/work-inventory.json` was not regenerated**, per instruction — a later lane owns it.

## Next-cycle plan

- A later lane regenerates `docs/work-inventory.json` to include the 65 new bestiary_5 units and re-derive the SD-33 final-acceptance scan / release notes, per the reopen ruling.
- The 20 `Change Shape (<Option>)` records' TYPE-pool option-picker UI is real, scoped, out-of-this-cycle product work — tracked via `OPEN_FINDINGS`/`UNREACHED_RECORD_FINDINGS`, not silently deferred.
- The three pre-existing unrelated failures noted above are available for a future cycle or operator triage; none blocks this fold.
