# Cycle AT-35-E6-003-RULED cycle 3 — Epic 6 PCGen exit / AT-35-E6-003-RULED

- **Commit SHA:** `381f33bc84`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design;
  decisions.md §2, workflow-instruction.md §6 step 1)`

  It ran anyway, at the cycle's start tree `b9e5956261`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree and passed:
  ```
  live_files=24 live_hits=53 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

- **Files touched:**
  - `src/pcgen_import/pcc_package_loader.rs` — **new, 331 lines.** The PCC → parse → IR
    convenience loader, moved verbatim off the live side: `load_composed_core_rulebook`,
    `project_corpus_from_owned`, `drain_into`, `ComposedCoreRulebookOwnedInputs`,
    `ComposedCoreRulebookLoadResult`, `ComposedInputDiagnostic::include_resolution_failure`, and
    the one test that exercised the projector. Same functions, same order, same diagnostics, same
    borrow discipline. `decisions.md §11` KEEPS this code — it is converter-side, and this is
    where converter-side code lives.
  - `src/rules_core/composed_input.rs` — **all eight** `pcgen_import` reads gone, and the module
    is now what its own doc says it is. The doc's justifying sentence ("it does no parsing, no
    include resolution, no IR conversion of its own") was true of `compose` and false of the file;
    the paragraph now records that and points at the new home.
  - `src/pcgen_import/mod.rs` — registers `pcc_package_loader` with the same KEPT-not-deleted note
    `pcgen_desc` and `pool_member_tokens` already carry.
  - `src/rules_core/rules_tables/crb/json_cache.rs` — `CorpusRecord::rename` is
    `Option<RenameInfo>` against a **locally declared** two-string struct instead of
    `Option<cache_gen::equipment_gap::RenameInfo>`. `cache_gen` already keeps three separate local
    copies of that shape under the no-shared-types-file convention `equipment_gap.rs`'s own doc
    comment establishes; this is the fourth, owned by the side that reads it. The wire shape is
    byte-identical (`{reason, coordinate}`, `skip_serializing_if = "Option::is_none"`).
  - `src/bin/gen_core_rulebook_cache.rs` — the generator maps the converter's `RenameInfo` to the
    reader's at the two sites that construct a `CorpusRecord`. **This was a self-heal:** the first
    verification run failed with two `E0308`s here, which is the `count-change-needs-a-sweep`
    lesson in type form — the live struct's field type moved and its only writer was a `src/bin`
    target the `--lib` check does not build.
  - `tests/sd18_preloop_consumer_compose.rs` — imports the loader from its new path.
  - `…/AT-35-E6-003-RULED_cycle3_runtime_import_census.py` / `.json` — **new.** Imports cycle 2's
    census whole (which imports cycle 1's) and rewrites one group's reason with this cycle's
    measurement.
  - `docs/retro/events/at-35-e6-003-ruled.jsonl` — 2 `correction`, 1 `deferral`, 1 `incident`.
  - `progress.md`, `kanban.md`, this receipt.
  - **Folded from the shared checkout, not this cycle's work:**
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (one field,
    `derived_at`, restamped by this cycle's own `completion_atlas.py --check`) and
    `docs/retro/events/sd31-transcribe.jsonl` (appended lines from another session on the shared
    checkout). Committed rather than filtered away, per the standing "clean tree = unfiltered
    `git status` empty" rule.

  **No `data/` file and no corpus record was changed**, so the converted package and the work
  inventory are byte-identical to the cycle's start tree. **`apps/` was not touched**, so the
  desktop crate and the frontend run at the epic wrap-up, not here (`§6` step 3).

- **Identifier audit result:** OK_NO_BUNDLE_TAGS. Over this cycle's own added lines
  (`git diff --unified=0 -- src/ apps/ tests/ | grep '^+'`),
  `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` returns exactly one line and it is
  the diff's own `+++ b/tests/sd18_preloop_consumer_compose.rs` header — an integration-test
  FILENAME, the disposition every Epic 6 receipt has recorded.

- **Wired-integration audit result:** OK_NO_TOKENS. The same added-line set against
  `\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b` returns nothing. No `"Would
  …"` string, no inline mock, no fixture-only data path: this cycle moved working code between
  modules and re-declared one two-field struct.

- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E6-003`):

  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers
  > of `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc`
  > is deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  Plus the `-RULED` dispatch's own bar, which is the two rulings applied **and the call sites the
  corrected gate now sees cleared**.

  **Not met.** `apps/desktop/` is `files=3 hits=7`, unchanged by this cycle — the three files left
  there (`corpus_fixtures.rs`, `feat_catalog.rs`, `race_trait_picker.rs`) all need a converted
  equivalent the package does not carry. What moved is the `src/rules_core/` half: `21 / 46` →
  `19 / 37`.

- **Receipt rows (mechanical):**
  ```
  since=b9e595626eff8f0b87477695a275d9b1a280ce32 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=697 ratio=n/a builds_recorded=1 pcgen_live_files=22
  ```
  Re-derived at HEAD after the cycle's commit. The same command run against the uncommitted
  working tree printed `rust_lines_changed=366`; the committed figure is larger because a
  relocation is counted on **both** sides — the 249 lines that left `composed_input.rs` and the
  331-line file they arrived in are two diffs, not one. `closed=0` is correct and expected: Epic 6
  closes zero corpus units by design and no `data/` file changed, so
  `docs/work-inventory.json` is byte-identical before and after.

- **PCGen residue:** `live_files=22 live_hits=44 baseline_files=260 baseline_hits=12736 verdict=PASS`
  — down from cycle 2's `24 / 53` on both axes, and **the instrument was not touched this cycle**
  (`scripts/pcgen_residue_gate.py` and `scripts/pcgen-residue-baseline.env` are both unchanged in
  this cycle's diff), so the `−9 / −2` is entirely code. Per-root, the figure the criterion's
  Evidence sentence asks for:
  ```
  root src/rules_core        files=21 hits=46  ->  files=19 hits=37
  root src/saved_character   files=0  hits=0
  root src/campaign          files=0  hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop          files=3  hits=7   ->  files=3  hits=7   (unchanged)
  ```
  `apps/desktop` is still **not** zero.

- **Oracle parity:** N/A. No `Number` mapping was added and no rendered value changed: the eight
  loader reads were a module move with no call-order change, and the `RenameInfo` swap is a
  two-string provenance field whose serialized shape is identical. The standing renderer-group
  parity measurement from cycle 2 (`compared=660320 agree=562988 disagree=97332`) is unchanged and
  still refuses that group; it is re-derivable by the command in the figures table.

- **Movement, four buckets:**
  - **closure:** **none in corpus units** (Epic 6 closes zero by design). On the B16 population:
    `ir_converter` `6 → 3`, `lst_parser_types` `22 → 16`; 9 hits and 2 whole files cleared.
  - **relabel:** none.
  - **reachability:** none — no rendered sheet line moved.
  - **instrument-correction:** **none.** The gate script, its baseline file and its patterns are
    untouched. Every hit that left did so because the code that made it left the live roots.

- **Refused tokens:** **44 hits across 22 files, six groups**, same six as cycle 2:
  ```
  renderer=8, lst_parser_types=16, ingest_record_tokens=7, trait_and_pool_tokens=5,
  ir_converter=3, source_content_payload=5
  ```
  `8+16+7+5+3+5 = 44`. Six groups, under `§8`'s limit of ten. Recorded as
  `deferral 1789251035947-at-35-e6-003-ruled-8a0324`; every line named with file, line and reason
  in `…_cycle3_runtime_import_census.json` and re-derivable by its script.

  **Why each group did not go.** `renderer` (8) is refused by cycle 2's measurement, not by
  difficulty: the converted candidate exists and runs, and it disagrees with the live path on
  97,332 of 660,320 renderings across 2,443 record keys, every shape of it converter-side.
  `lst_parser_types` (16) + `ingest_record_tokens` (7) + `ir_converter` (3) +
  `source_content_payload` (5) = 31 hits are one piece of work, unchanged in shape from cycle 2 and
  one third smaller: the live side still **owns** `EquipmentRecord` / `LstSpellRecord` as its own
  data types across 12 files and `corpus_loader.rs` still **runs**
  `ir_converter::convert_equipment_record` / `convert_spell_record` at run time rather than reading
  their output. `trait_and_pool_tokens` (5) wants the converted rule's `applies` gates and a
  converted pool-member table.

- **Discoveries:** two, both emitted as `correction` retro events.
  - `1789251026013-at-35-e6-003-ruled-5d9ef9` — **`composed_input.rs`'s own module doc disclaimed
    the three things the module did.** The paragraph headed "Why this lives here (not in
    `pcgen_import`)" states the composer "does no parsing, no include resolution, no IR conversion
    of its own", and that sentence is the stated reason the file sits in a live root. It was true
    of `compose` and false of the file: the loader beside it resolved a PCC include graph, ran all
    six B-family LST parsers and ran the IR converter, at run time. Eight of the 53 live reads —
    **the largest single-file concentration in the whole B16 population** — and no earlier Epic 6
    cycle opened the file, because the doc said there was nothing there.
  - `1789251026155-at-35-e6-003-ruled-a9e18b` — **the relocation that would have been a blind
    spot, refused before it was written.** `src/rules_core/derived_evaluator_fixture_check.rs`
    presents as the same shape `AT-35-E6-001` already moved (`race_trait_formula_bar_check` →
    `src/oracle_validation/`): a bar-check harness reading a committed fixture, sitting in a live
    root, with its `ingest_record` calls in private corpus walks. It is not. Three desktop catalogs
    import eleven live rendering symbols out of it at run time — `monster_catalog.rs`
    (`spell_like_ability_caster_level`, `spell_like_ability_save_dc`), `spell_catalog.rs`
    (`all_spell_caster_level_durations`, `all_spell_caster_level_ranges`, two formatters, two
    types, one dir resolver) and `companion_catalog.rs` (six `parse_`/`format_companion_*`) — and
    those reach the very walks that make the calls. Moving the file would have lowered the gate by
    1 and changed nothing in the shipping binary: the blind-spot shape `decisions.md §19` was
    ruled to end. The refutation is recorded in the census, at the group, so no later cycle pays
    to re-discover it.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=22 live_hits=44 baseline_files=260 baseline_hits=12736 verdict=PASS` | every source file (`.rs .ts .tsx .js .jsx .mjs .cjs`) under the five live roots, comment lines excluded (B14) and `#[cfg(test)]` regions excluded (B15) | `python3 scripts/pcgen_residue_gate.py --check` |
  | `root src/rules_core files=19 hits=37`; `root apps/desktop files=3 hits=7` | the same, restricted to that root | `python3 scripts/pcgen_residue_gate.py --check` |
  | `pcgen_import_hits=44 files=22`; `by_root=apps/desktop=7, src/rules_core=37`; the six group sizes | the shipping lines under the five live roots naming `pcgen_import` | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle3_runtime_import_census.py` |
  | 3 desktop files importing 11 live symbols from `derived_evaluator_fixture_check` | `apps/desktop/src-tauri/src/**` | `grep -rn 'rules_core::derived_evaluator_fixture_check' --include=*.rs apps/` |
  | `records=16508 compared=660320 agree=562988 both_none=242160 disagree=97332` (the renderer group's standing refusal, measured by cycle 2, unchanged here) | every `class_feature` record `class_feature_record_tokens()` carries, × levels 1..=20 × 2 ability probes | `AT35_E6_PROSE_PARITY=docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle2_prose_parity_census.json cargo test --locked --lib -j 6 -- --ignored --nocapture class_feature_grant_consumer::tests::class_feature_prose_parity_census` |
  | `0` files under `data/sheet_rules/` carrying ingest syntax | the whole converted package | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | `done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0` | the completion atlas | `python3 scripts/completion_atlas.py --check` |
  | `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=233 shapes=1 verdict=PASS` | token coverage | `python3 scripts/token_coverage.py --check` |
  | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` | shape/engine boundary | `python3 scripts/shape_engine_boundary.py --check` |
  | `population=0 kinds=0 citation_failures=0` | missing engine tables | `python3 scripts/missing_engine_tables.py --check` |
  | `files_checked=133 violations=0` | the bundle package's markdown | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` |
  | `Ran 78 tests ... OK` | the two gate scripts' own unit tests, which pin B15 and B16 | `python3 -m unittest scripts.tests.test_pcgen_residue_gate scripts.tests.test_cycle_scope_gate` |
  | `RESULT: PASS` (`pi-sweep`) | the Product-Identity sweep stage | `bash scripts/verify.sh --only pi-sweep` |
  | `records=49438 converted=49296 refused=142 rules=70135 var_tables=5293 verdict=PASS` (114.3s) | the whole converted package | `cargo run --locked --bin sheet_rule_convert -- --check` |
  | `NO_RUN_EXIT=0`; lib `3341 passed; 0 failed; 16 ignored`; full workspace `FULL_EXIT=101` — **418 targets, 8,869 passed, 1 failed, 69 ignored, exactly one `test result: FAILED` line, attributed below** | the whole root workspace | `cargo test --locked --no-run -j 6`; `cargo test --locked --lib -j 6`; `cargo test --locked --no-fail-fast -j 6` |
  | root-workspace clippy **0 warnings** | the root workspace with tests | `cargo clippy --locked --tests -j 6` |
  | `closed=0 relabeled=0 rust_lines_changed=697 ratio=n/a builds_recorded=1 pcgen_live_files=22` | `docs/work-inventory.json` before vs after | `python3 scripts/cycle_scope_gate.py --receipt --since b9e595626eff8f0b87477695a275d9b1a280ce32 --before /tmp/wi-before-AT-35-E6-003-RULED.json --after docs/work-inventory.json` |

- **Build scope verified:** **the whole root workspace, at the final tree.** The desktop crate and
  the frontend are at **epic cadence** this cycle: `apps/` is not in this cycle's diff at all
  (`git diff --name-only -- apps/` is empty), which is the condition `§6` step 3 states for
  skipping them.

  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`.
  - `cargo test --locked --lib -j 6` → `3341 passed; 0 failed; 16 ignored` — identical to cycle 2's.
  - `cargo test --locked --no-fail-fast -j 6` → `FULL_EXIT=101` — **418 targets, 8,869 passed, 1 failed, 69 ignored, exactly one `test result: FAILED` line, attributed below**.
  - `cargo clippy --locked --tests -j 6` → **0 warnings** (the command emitted no `warning:` and no `error:` line at all).

  **The one `test result: FAILED`, attributed rather than bucketed.**
  `tests/sd17_b5_equipment.rs:463`,
  `parse_runs_in_linear_time_on_a_synthetic_large_file`: *"5k equipment records should parse in
  well under 2s, took 2.374002021s"*. It is a **wall-clock budget**, not a correctness assertion,
  and it is **not this cycle's**: `git diff --stat HEAD -- tests/sd17_b5_equipment.rs
  src/pcgen_import/lst_parser/equipment.rs` is **empty** — neither the test nor the parser it
  times appears in this cycle's diff, which only moved one of that parser's callers between
  modules. Re-run alone at the same tree,
  `cargo test --locked --test sd17_b5_equipment -j 2` → `25 passed; 0 failed; 0 ignored`, the same
  assertion finishing in **1.13 s** against its 2 s budget. A 2-second absolute budget measured
  under six parallel cargo jobs is a threshold, not a regression detector.
  `incident 1789256480840-at-35-e6-003-ruled-bd3974`, recurrence key
  `wall-clock-perf-assertion-under-parallel-load`. The other **417** targets and **8,869** tests
  passed.

  **One red round, self-healed, recorded rather than hidden.** The first verification run at this
  cycle's tree failed to build `src/bin/gen_core_rulebook_cache.rs` with two `E0308`s: the
  `RenameInfo` ownership swap moved a live struct's field type and that bin is its only writer.
  `cargo check --lib` was green throughout — the exact "verify at the widest build scope" failure
  `AGENTS.md` names. Fixed at the generator (where the converter's copy and the reader's copy meet)
  and the whole gauntlet re-run from scratch at the corrected tree; every figure above is from the
  second run.

- **Sweep population:** N/A — no corpus record changed, so `corpus_literal_sweep` would re-examine
  a byte-identical `data/`.

- **Oracle pin:** N/A. No figure in this receipt came from the pinned PCGen checkout;
  `scripts/pcgen-oracle-pin.env` is unchanged.

- **Status:** **partial.** The criterion's population is not zero at HEAD: 44 hits across 22 files
  remain, `apps/desktop` among them.

- **Notes:**

  The two moves are one rule applied twice — **the live root stops hosting code that is not the
  live side's**. Neither is a rename: the loader's eight reads did not move to another live file,
  they moved to `src/pcgen_import/`, where `decisions.md §11` says converter code belongs and where
  they are kept for Starfinder; the `RenameInfo` declaration did not move at all, it was
  re-declared by the reader under a convention `cache_gen` already applies to itself three times.
  The test that distinguishes both from gate-gaming is the one that refused the third move: does
  the shipping binary change? For the loader, `src/rules_core/` no longer links the LST parsers on
  the live path; for `derived_evaluator_fixture_check.rs` it would not have, so it stayed.

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design)`.
  The remainder is unchanged in shape from cycle 2's and one fifth smaller:

  1. **The `renderer` group (8) is a CONVERTER cycle.** Three named populations, enumerated by key
     in `AT-35-E6-003-RULED_cycle2_prose_parity_census.json`: 1,351 keys whose converted rule
     carries extra `Desc` segments; 718 keys the converter renders and the live path does not; 374
     the converted rule cannot render. The live swap is one commit once the census reads
     `disagree=0`.
  2. **`lst_parser_types` (16) + `ingest_record_tokens` (7) + `ir_converter` (3) +
     `source_content_payload` (5) = 31 hits, one piece of work:** a converted equipment/spell
     record shape the live side owns, so `EquipmentRecord` and `LstSpellRecord` stop being live
     types and `corpus_loader.rs` reads a converted artefact instead of running the conversion.
     `source_content_payload.rs`'s own module doc states why the enum cannot move alone. This also
     unblocks `apps/desktop/src-tauri/src/corpus_fixtures.rs` (5 of `apps/desktop`'s 7), which
     parses bundled LST text at run time and is still on the live command path through
     `character_hub.rs` and `pf1_adapter.rs`.
  3. **`trait_and_pool_tokens` (5)** — `race_trait_picker.rs`'s `PRE`-gate reads want the converted
     rule's `applies`; `class_feature_pool_catalog.rs` wants a converted pool-member table.

  `AT-35-E6-004`'s `--check --closure` bar is unchanged.
