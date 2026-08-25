# Cycle t9-onboarding-row21-cycle2-aftermath-closure/1 — Gate 3 closure invariant / row 21's four aftermath items

- **Card ID:** `epic-11-ingest-token-loss` (kanban row 21)
- **Actor:** `t9-onboarding`
- **Base:** `PIN=51af7085d740011a7f3b516e8ad70cbf68e27ec2`, confirmed the tip of `origin/tranche/12` (no rebase needed).
- **Files touched:**
  - `src/rules_core/pilot_compute/class_feature_grant_consumer.rs` — deleted the last-write-wins
    `parse_bonus_var_tokens` and its duplicate table-builder; `class_feature_record_tokens()` is now
    a thin alias for `class_feature_record_tokens_pre_gate_safe()`; added widening 3 to
    `parse_bonus_var_tokens_pre_gate_safe` (fall back to a target's sole ungated row when
    `extract_addends` refuses an unsupported multi-PRE-tag shape).
  - `src/bin/enrich_class_raw_tokens.rs` (new) — additive, non-destructive `raw_tokens` enrichment
    for the 27 `class` records shape (b) named; mirrors `enrich_equipment_raw_tokens.rs`'s shape.
  - `src/bin/ingest_pu_classes.rs` — `raw_tokens_excluding_bonus`/`raw_bonus_chains` now read the
    `.MOD`/`.COPY=` closure (`WiringClassIndex::closure_rows`) instead of a single base row, at
    both call sites (class variant, class feature).
  - `src/bin/enrich_pu_class_feature_mod_closure.rs` (new) — additive, non-destructive patch-in-place
    for exactly the 9 `pathfinder_unchained` `class_feature` records the row 21 cycle 1 blast-radius
    re-scan named; built because re-running `ingest_pu_classes`'s own `main()` wipes 540 files it
    does not own (see incident log below).
  - `data/corpus/{advanced_class_guide,advanced_players_guide,core_rulebook}/class/*.json` — 27
    files, `data.raw_tokens` added, nothing else changed.
  - `data/corpus/pathfinder_unchained/class_feature/{barbarian_unchained_class,monk_unchained_class,rogue_unchained_class,summoner_unchained_class}/*.json`
    — 9 files, `data.raw_tokens`/`data.raw_bonus_chains` replaced with the full closure, nothing
    else changed.
  - `src/rules_core/rules_tables/pathfinder_unchained/monk_features.rs` — one stale pinned
    assertion fixed (swept, per the standing rule): `every_modelled_formula_is_byte_exact_against_
    the_ingested_corpus_record` asserted `bonus_tokens(Flurry of Blows).is_empty()` with a comment
    explicitly documenting the very `.MOD`-block-not-folded-in defect this cycle fixes; the record
    now genuinely carries those tokens, so the test now asserts the real, restored 10-token content
    (verified against the sibling ignored test's own independent pin of the same raw `.lst` rows,
    which now also passes with `PCGEN_CORPUS_ROOT` set).
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 21 cycle 2 update appended.
- **Corpus SHA:** oracle bootstrapped fresh this worktree via `scripts/fetch-pcgen-oracle.sh --dest`
  (git-ignored slot, confirmed empty before bootstrap).
- **Status:** all four items closed (see kanban row 21 cycle 2 entry for full detail):
  1. `derived_evaluator_fixture_check` real defect: FIXED (5/5 green, was 4/5).
  2. `the_live_scale_of_this_waves_widening_is_measured_and_pinned`: re-derived, GREEN unmoved at
     the existing pin — the brief's predicted movement did not occur (Bond Senses was never
     affected by the `.MOD`-collision defect; single ungated row).
  3. Shape (b), 27/168 `class` records: FIXED, 27/27 now carry `raw_tokens`. Prior deferral closed
     via `scripts/retro.py correction --corrects row21-shape-b-class-raw-tokens-not-fixed`.
  4. The 9 `pathfinder_unchained` records: confirmed genuinely affected (real `.MOD`-appended
     `BONUS:VAR` content silently dropped, not merely inert tokens), fixed in the owning generator.
- **PI screening**: `declared_pi_shipping_audit` run before (corpus reverted to pre-cycle-2 HEAD)
  and after (full fix applied) this cycle's corpus changes — see §3 below for both results. A
  scoped `pi_scrub.py::blacklist_term_hit_including_concatenated` pass over every field of all 36
  touched records found 0 hits.
- **Notes:** two real near-misses caught before commit, both logged via `scripts/retro.py incident`
  (see §2).

## §1. Re-derivation of the brief's own figures

- Shape (b) count: re-derived independently (`python3` one-off over `data/corpus/*/class/*.json`),
  confirmed **141 present / 27 absent**, matching the brief and row 21 cycle 1's own figure.
- The 9 `pathfinder_unchained` coordinates: re-derived independently by cross-referencing every
  PU-owned (`data.class_key` present) `class_feature` record's `key` against every real `.MOD`
  target in `pu_abilities_class.lst` (CATEGORY-prefix stripped the same way
  `wiring_class::mod_base_name` does) — found the SAME 9 keys the row 21 cycle 1 receipt named,
  independently, before reading that receipt's own list.
- The `(136,20,11,9,36) -> (136,21,11,8,36)` predicted pin movement: did NOT reproduce. Re-run of
  `the_live_scale_of_this_waves_widening_is_measured_and_pinned` against the fully-fixed corpus is
  GREEN at the unmoved `(136,20,11,9,36)` pin. `Summoner ~ Bond Senses` was never affected by the
  `.MOD`-collision defect (its corpus record carries exactly one ungated `BONUS:VAR` row) — the
  brief's figure did not survive re-derivation (`§17a`), reported as found, pin left unmoved (a real
  observation, not a loosening).
- `apps/desktop/src-tauri` baseline: brief states 538/0 before row 21 landed. Current state: 536
  passed / 2 failed (`class_feature_feat_bridge`'s two corpus-wide-population assertions, pinned
  `613`, observed `612`). Isolated by reverting this cycle's 36 corpus files to their pre-cycle-2
  HEAD content and re-running just those two tests: IDENTICAL failure, confirming this drift is
  pre-existing and unrelated to this cycle (likely a sibling lane's, "row 19"'s own recent
  `471 -> 613` pin against a corpus that has since moved again) — not this row's defect, not fixed
  here, named for the operator/owning lane.

## §2. Incidents caught before commit

1. **`ingest_pu_classes.rs` directory-wipe near-miss.** Running its own `main()` (even after fixing
   its `raw_tokens_excluding_bonus`/`raw_bonus_chains` functions) to regenerate the 9 records wiped
   the WHOLE `data/corpus/pathfinder_unchained/{class,class_feature}` trees first
   (`fs::remove_dir_all`) — but `class_feature/` is SHARED with the generic
   `class_feature::generate()` pipeline's own 540 non-foreign records. `git status --porcelain`
   showed 540 `D` + 68 `M` immediately after the run, not the expected 9 `M`. Reverted in full
   (`git checkout -- data/corpus/pathfinder_unchained/`); built the scoped, non-destructive
   `enrich_pu_class_feature_mod_closure.rs` instead. Logged:
   `scripts/retro.py incident`, dedupe-key `t9-onboarding-ingest-pu-classes-wipe-near-miss`.
2. **Double-skip content-loss bug in this cycle's own tools.** Both new enrichment tools' first
   draft called `tab_tokens(row).into_iter().skip(1)` — `corpus_literal_sweep::tab_tokens` already
   performs its own `.skip(1)` internally, so the extra skip silently dropped each record's FIRST
   real token (`core_rulebook/class/fighter.json` lost its own `HD:10`). Caught by a manual
   field-list diff against the known raw `.lst` line order before any commit; both corpus writes
   reverted, the bug fixed (single `.skip(1)`), both tools re-run clean. Logged:
   `scripts/retro.py incident`, dedupe-key `t9-onboarding-tab-tokens-double-skip`.

## §3. Test results (real commands, real output)

```
cargo test --locked --test derived_evaluator_fixture_check
  -> 5 passed; 0 failed (was 4 passed, 1 failed before this cycle's fix)

cargo test --locked --lib class_feature_grant_consumer
  -> 28 passed; 0 failed

cargo test --locked --lib bonus_stack_reader
  -> 21 passed; 0 failed

cargo test --locked --lib derived_evaluator_fixture_check
  -> 121 passed; 0 failed

cargo test --locked --lib pilot_compute
  -> 947 passed; 0 failed

cargo test --locked --lib wiring_class
  -> 62 passed; 0 failed

cargo test --locked --lib cache_gen::class_feature -- --include-ignored (PCGEN_CORPUS_ROOT set)
  -> 87 passed; 0 failed

cargo test --locked --bin ingest_pu_classes
  -> 24 passed; 0 failed

cargo test --locked --lib class_feature_pool_catalog
  -> 23 passed; 0 failed

cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml (SEPARATE cargo workspace)
  -> 536 passed; 2 failed (pre-existing, isolated and confirmed unrelated -- see §1)
```

`declared_pi_shipping_audit`, before (corpus reverted to pre-cycle-2 HEAD) and after (full fix
applied): both CLEAN — "declared-pi-audit: CLEAN — no shipped record contradicts its own corpus
row's PI declaration".

`df -h /`: 968G size, 456G used, 513G available, 48% use.
