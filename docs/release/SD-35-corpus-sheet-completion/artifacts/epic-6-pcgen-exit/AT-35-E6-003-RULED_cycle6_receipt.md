# Cycle AT-35-E6-003-RULED cycle 6 — Epic 6 PCGen exit / AT-35-E6-003-RULED

- **Commit SHA:** `1cf3f4bc7b` (the code, the two census/parity scripts and their JSON, the
  retro events), cycle start `f760493300`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design;
  decisions.md §2, workflow-instruction.md §6 step 1)`

  It ran anyway, at the cycle's start tree `f760493300`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree and passed at exactly
  cycle 5's closing figure — nothing drifted between the two cycles:
  ```
  live_files=18 live_hits=36 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

- **Files touched:**
  - `src/rules_core/rules_tables/simple_kind_tables.rs` — **+84 lines, −5** (one `.rs` file,
    the whole of this cycle's Rust). `SimpleKindRecord::raw_token_count: usize` becomes
    `converted_rule_count: Option<usize>`. The retired field was
    `pcgen_import::ingest_record::token_count(data)` — the **length of the corpus record's own
    PCGen `raw_tokens` array**, read at table-load time inside the crate that prints a
    character sheet, and printed by `--epic2-table-transcript` as `ingest_tokens=N`. The new
    field is filled through **cycle 5's closure-row join**: the record states its own
    `source.path` / `source.line`, every rule the converter wrote from that row names the same
    `path:line` in `provenance.closure_rows`, and
    `SheetRulePackage::rules_for_closure_row(path, line)` returns them. The package is resolved
    **once per table load**, not per record (it is a process-wide `OnceLock`).

    `None` is printed as `not-converted`, **never as `0`** — `0` would read as a converted
    record the converter gave no rules, which is a different fact. The transcript line's field
    is renamed with the quantity: `ingest_tokens=N` → `converted_rules=N`. Nothing outside this
    module reads either the field or the line (`grep -rn 'raw_token_count\|transcript_line'
    --include=*.rs src/ apps/ tests/` → one consumer, `src/bin/v06_work_inventory.rs:19804`,
    which prints it).

    The module no longer names `pcgen_import` at all.
  - `…/AT-35-E6-003-RULED_cycle6_runtime_import_census.py` / `.json` — **new.** Imports cycle
    5's census whole (which imports cycle 4's, … back to cycle 1's) and rewrites two groups'
    reasons with this cycle's measurements. It asserts its own total against the gate's
    (`gate_agreement=OK (35 == 35)`) so the two cannot disagree silently.
  - `…/AT-35-E6-003-RULED_cycle6_pool_guard_parity.py` / `.json` — **new.** The measurement
    behind this cycle's refusal; see **Discoveries**.
  - `docs/retro/events/at-35-e6-003-ruled.jsonl` — 1 `correction`, 1 `deferral`, plus the
    `pi-sweep` stage's own derived `verification` event, which landed **under this actor**
    (see **Notes**).
  - `progress.md`, `kanban.md`, this receipt.
  - **Folded from this cycle's own instruments, not authored work:**
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (one
    field, `derived_at`, restamped by this cycle's `completion_atlas.py --check`). Committed
    rather than filtered away, per the standing "clean tree = unfiltered `git status` empty"
    rule.

  **No `data/` file and no corpus record was changed** (`git status --porcelain data/` empty at
  the final tree), so `data/sheet_rules/` and `docs/work-inventory.json` are byte-identical to
  the cycle's start tree. **`apps/` was NOT touched**, so the desktop crate and the frontend run
  at the epic wrap-up (`§6` step 3), not here.

- **Identifier audit result:** OK_NO_BUNDLE_TAGS. Over this cycle's own added lines
  (`git diff f760493300 -- src/ | grep -E '^\+'`),
  `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` returns nothing.

  The cumulative `${BASE_BRANCH}...HEAD` form over the epic's file-touch set returns only
  pre-existing `tests/sd27_*` / `tests/sd34_*` / `tests/sd35_*` **filenames quoted inside
  earlier cycles' receipt prose** — the disposition every Epic 6 receipt has recorded. No
  identifier in shipping code carries a bundle tag.

- **Wired-integration audit result:** OK_NO_TOKENS, first run, no self-heal. This cycle's added
  Rust lines and both new Python instruments under
  `grep -nEi '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` return nothing.
  No `"Would …"` string, no inline mock, no fixture-only data path: the new field is computed
  from the live converted package and the transcript states `not-converted` rather than
  inventing a number when the package holds nothing.

- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E6-003`):

  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers
  > of `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc`
  > is deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  Plus the `-RULED` dispatch's own bar, which is the two rulings applied **and the call sites the
  corrected gate now sees cleared**.

  The Evidence sentence's `apps/desktop` clause stays met (`root apps/desktop files=0 hits=0`,
  first met in cycle 4, not regressed here — this cycle wrote no `apps/` file). The criterion as
  a whole is **not** met: 35 hits across 17 files remain under `src/rules_core/`, and
  `render_pcgen_desc_with_values` is still called there.

- **Receipt rows (mechanical):**
  ```
  since=f76049330053cb9888ec1627e091cca587908b5b residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=89 ratio=n/a builds_recorded=1 pcgen_live_files=17
  ```
  `closed=0` is correct and expected: Epic 6 closes zero corpus units by design and no `data/`
  file changed, so `docs/work-inventory.json` is byte-identical before and after.

- **PCGen residue:** `live_files=17 live_hits=35 baseline_files=260 baseline_hits=12736
  verdict=PASS` — down from cycle 5's `18 / 36` on both axes, and **the instrument was not
  touched this cycle** (`git diff --name-only f760493300..HEAD -- scripts/` is empty), so the
  `−1 / −1` is entirely code. Per-root:
  ```
  root src/rules_core        files=18 hits=36  ->  files=17 hits=35
  root src/saved_character   files=0  hits=0
  root src/campaign          files=0  hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop          files=0  hits=0   (unchanged — no apps/ file written)
  ```
  **One hit, one whole file.** Not gate-gaming: nothing renamed to duck a regex, no path
  exempted, no `use` collapsed, no rebaseline. The 27 gate unit tests that pin B14, B15 and B16
  are still green.

- **Oracle parity:** N/A for the pinned PCGen oracle — no `Number` mapping was added and no
  rendered value changed. The parity this swap turns on is **totality of the join**, measured
  over the whole seven-kind corpus rather than a fixture: `records=8486 resolved=8486
  disagree=0`, with the join recomputed in-test from each record's stored
  `(source_path, source_line)` as the field's oracle, so the stored value and the join cannot
  drift silently. **Mutation-proved:** joining on `source_line + 1` makes the test fail naming
  real records (`ability advanced_class_guide/"Arcanist Exploit ~ School Understanding":
  field=Some(2) join=Some(3)`).

- **Movement, four buckets:**
  - **closure:** **none in corpus units** (Epic 6 closes zero by design). On the B16 population:
    `src/rules_core` `36 → 35` hits and `18 → 17` files; by group, `ingest_record_tokens`
    `6 → 5`. `rules_tables/simple_kind_tables.rs` leaves the census entirely.
  - **relabel:** none. No hit moved between files or groups.
  - **reachability:** none — no rendered sheet line moved. The seven-kind tables resolve the
    same records from the same directories; only the transcript's trailing field changed, and it
    is evidence output, not a sheet line.
  - **instrument-correction:** **none.** The gate script, its baseline file and its patterns are
    untouched; the census's group table is imported from cycle 5, not rewritten. Two group
    *reasons* were rewritten — that is a corrected explanation, and it moved no number.

- **Refused tokens:** **35 hits across 17 files, six groups**, the same six as cycles 4 and 5:
  ```
  renderer=5, lst_parser_types=12, ingest_record_tokens=5, trait_and_pool_tokens=4,
  ir_converter=4, source_content_payload=5
  ```
  `5+12+5+4+4+5 = 35`. Six groups, under `§8`'s limit of ten. Recorded as
  `deferral 1789269274004-at-35-e6-003-ruled-f6f1e2`; every line named with file, line and
  reason in `…_cycle6_runtime_import_census.json` and re-derivable by its script.

  **Why each group did not go.**

  - `trait_and_pool_tokens` (4) — **this cycle MEASURED it and refuses it on the number**, which
    is the cycle's other product. See **Discoveries**. It is stated, not netted: the group is
    unchanged at 4, and this cycle's one cleared hit was classified under
    `ingest_record_tokens`, not here.
  - `renderer` (5) — refused by cycle 2's measurement, not by difficulty: the converted
    candidate exists, runs, and disagrees with the live path on 97,332 of 660,320 renderings
    across 2,443 record keys, every shape of it converter-side.
  - `lst_parser_types` (12) + `ingest_record_tokens` (5) + `ir_converter` (4) +
    `source_content_payload` (5) = **26 hits are one piece of work**, unchanged in kind by this
    cycle: those callers do not want a *fact about* a record, they **own** `EquipmentRecord` /
    `LstSpellRecord` / `SourceContentPayload` as their own data types across 13 files and read
    the PCGen `BONUS:` chains and `KEY:VAL` tokens on them directly. That clears when
    `sheet_rule_convert` emits an equipment / equipment-modifier / spell rule shape
    `equipment_effects` can read, not when a lookup exists.
  - Within `ingest_record_tokens`, `derived_evaluator_fixture_check.rs` stays **measured
    non-relocatable** and this cycle re-checked why rather than inheriting the sentence: its
    three token reads sit inside the private corpus sweeps `load_spell_durations`,
    `load_spell_ranges` and `load_class_feature_bonus_vars`, which are reached by the eleven
    live renderers three desktop catalogs import out of the module
    (`monster_catalog`, `companion_catalog`, `spell_catalog`). Moving the module would lower the
    gate by 1 and change nothing in the shipping binary.

- **Discoveries:** one, emitted as a `correction` retro event.

  `1789266033634-at-35-e6-003-ruled-e44c73` (`correction`) — **a group reason carried unmeasured
  for five cycles named the wrong dependency.** Every census from cycle 1 through cycle 5
  recorded that `class_feature_pool_catalog`'s four `pool_member_tokens` guards "read a record's
  DESC/effect token array rather than one classified fact" and so do not ride on the closure-row
  join. Cycle 5's own lesson is that such a sentence is an assertion about a dependency and has
  to be checked against **what the caller needs**. This cycle checked it, over the whole live
  `class_feature` corpus, rather than writing it down a seventh time.

  **The join is not the blocker.** 18,043 of 18,074 live `class_feature` corpus records resolve
  through `rules_for_closure_row`. **The converted rule is.** Computing each guard's ingest form
  from the record's own `raw_tokens` and its best converted-side form from every rule the
  package wrote from that record's source row:

  | guard | population | agree | disagree | verdict |
  |---|---|---|---|---|
  | `has_no_engine_effect_token` | 18,043 | 11,549 | **6,494** | REFUSED |
  | `is_archetype_locked` | 18,043 | 17,124 | 919 | REFUSED |
  | `carries_more_than_one_desc_segment` | 18,043 | 17,954 | 89 | REFUSED |

  The first is the guard that decides the swap — it is the one that refuses a record carrying a
  mechanic — and **the mechanism behind its failure is exact, not vague**: PCGen's `ABILITY:`
  token maps to `MapsTo::Applies` in `src/pcgen_import/sheet_rule/table.rs`, i.e. to a
  *prerequisite*, and `BONUS:VAR|…` lands in `data/sheet_rules/_vars/` rather than on the rule
  object. So `occult_adventures:class_feature:elemental_ascetic_elemental_flurry`, whose corpus
  row carries `ABILITY:Feat|AUTOMATIC|Improved Unarmed Strike` and
  `ABILITY:Special Ability|AUTOMATIC|Wild Talent ~ kinetic fist`, converts to a rule with
  `grants: null`, `target: None`, `value: "Text"` — on the converted side, byte-identical to a
  record that really is prose only.

  **The swap is refused on the number, not deferred on effort** — the disposition cycle 2
  reached for the renderer group. The remedy is converter-side and is named so a later cycle
  does not re-derive it: `ABILITY`'s mapping row, and a `BONUS:VAR` reference on the rule that
  declares it.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=17 live_hits=35 baseline_files=260 baseline_hits=12736 verdict=PASS` | every source file (`.rs .ts .tsx .js .jsx .mjs .cjs`) under the five live roots, comment lines excluded (B14) and `#[cfg(test)]` regions excluded (B15) | `python3 scripts/pcgen_residue_gate.py --check` |
  | `root src/rules_core files=17 hits=35`; `root apps/desktop files=0 hits=0` | the same, restricted to that root | `python3 scripts/pcgen_residue_gate.py --check` |
  | `pcgen_import_hits=35 files=17`; `apps_desktop_hits=0 evidence_sentence_met=YES`; the six group sizes; `gate_agreement=OK (35 == 35)` | the shipping lines under the five live roots naming `pcgen_import` | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle6_runtime_import_census.py` |
  | the gate script and its baseline are absent from this cycle's diff | `scripts/` | `git diff --name-only f76049330053cb9888ec1627e091cca587908b5b..HEAD -- scripts/` |
  | `records=8486 resolved=8486 disagree=0` (the stored `converted_rule_count` vs the join recomputed in-test) | every `data/corpus/*/{ability,template,trait_generic,deity,domain,skill,language}/**/*.json` record | `cargo test --locked --lib -j 6 rules_core::rules_tables::simple_kind_tables::tests::every_seven_kind_record_resolves_to_its_own_converted_rules` |
  | `8486` seven-kind corpus records | the same | `python3 -c "import glob; print(sum(len(glob.glob(f'data/corpus/*/{d}/*.json'))+len(glob.glob(f'data/corpus/*/{d}/*/*.json')) for d in ['ability','template','trait_generic','deity','domain','skill','language']))"` |
  | the same test red under `source_line` → `source_line + 1`, naming real records | the same | the mutation, applied and reverted in-cycle |
  | `corpus_class_feature_records=18074 joined=18043 unjoined=31`; `P1 agree=11549 disagree=6494`, `P2 agree=17124 disagree=919`, `P3 agree=17954 disagree=89`; `swap_verdict=REFUSED` | every `data/corpus/*/class_feature/**/*.json` record carrying `raw_tokens` | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle6_pool_guard_parity.py` |
  | `ABILITY` maps to `MapsTo::Applies` | the converter's token-mapping table | `grep -n 'token_type: "ABILITY"' src/pcgen_import/sheet_rule/table.rs` |
  | `records=49438 converted=49296 refused=142 rules=70135 var_tables=5293 verdict=PASS` (115.3 s) | the whole converted package | `cargo run --locked --bin sheet_rule_convert -- --check` |
  | `0` files under `data/sheet_rules/` carrying ingest syntax | the whole converted package | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | `missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0` | the completion atlas | `python3 scripts/completion_atlas.py --check` |
  | `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=233 shapes=1 verdict=PASS` | token coverage | `python3 scripts/token_coverage.py --check` |
  | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` | shape/engine boundary | `python3 scripts/shape_engine_boundary.py --check` |
  | `population=0 kinds=0 citation_failures=0` | missing engine tables | `python3 scripts/missing_engine_tables.py --check` |
  | `files_checked=135 violations=0` | the bundle package's markdown | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` |
  | `Ran 27 tests … OK` | the residue gate's own unit tests, which pin B14, B15 and B16 | `python3 -m unittest scripts.tests.test_pcgen_residue_gate` |
  | `RESULT: PASS` (`pi-sweep`) | the Product-Identity sweep stage | `RETRO_ACTOR=AT-35-E6-003-RULED bash scripts/verify.sh --only pi-sweep` |
  | `NO_RUN_EXIT=0`; lib `3345 passed; 0 failed; 16 ignored` (43.0 s); full workspace `FULL_EXIT=0` — 418 targets, **8,874 passed, 0 failed, 69 ignored**, zero `test result: FAILED` lines | the whole root workspace | `cargo test --locked --no-run -j 6`; `cargo test --locked --lib -j 6`; `cargo test --locked --no-fail-fast -j 6` |
  | root-workspace clippy **0 warnings** | the root workspace with tests | `cargo clippy --locked --tests -j 6` |
  | `closed=0 relabeled=0 rust_lines_changed=89 ratio=n/a builds_recorded=1 pcgen_live_files=17` | `docs/work-inventory.json` before vs after | `python3 scripts/cycle_scope_gate.py --receipt --since f76049330053cb9888ec1627e091cca587908b5b --before /tmp/wi-before-AT-35-E6-003-RULED.json --after docs/work-inventory.json` |

  The lib count moved `3344 → 3345` and the workspace total `8,873 → 8,874`: exactly this
  cycle's one new corpus-wide test, and nothing else moved.

- **Build scope verified:** **the whole root workspace**, at the final tree. `apps/` is absent
  from this cycle's diff (`git diff --name-only f760493300..HEAD -- apps/` is empty), which is
  the condition `§6` step 3 states for leaving the desktop crate and the frontend to the epic
  wrap-up.

  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`.
  - `cargo test --locked --lib -j 6` → `3345 passed; 0 failed; 16 ignored` (43.0 s).
  - `cargo test --locked --no-fail-fast -j 6` → `FULL_EXIT=0` — **418 targets, 8,874 passed,
    0 failed, 69 ignored**, zero `test result: FAILED` lines.
  - `cargo clippy --locked --tests -j 6` → **0 warnings**, first run, no self-heal.

- **Sweep population:** N/A — no corpus record changed, so `corpus_literal_sweep` would
  re-examine a byte-identical `data/`.

- **Oracle pin:** N/A. No figure in this receipt came from the pinned PCGen checkout;
  `scripts/pcgen-oracle-pin.env` is unchanged.

- **Status:** **partial.** The criterion's population is not zero at HEAD: 35 hits across 17
  files remain under `src/rules_core/`.

- **Notes:**

  Two judgment calls.

  **The field was renamed with the quantity, not silently reused.** `raw_token_count` and
  `converted_rule_count` are different numbers about the same record, and the transcript's field
  name moved with them (`ingest_tokens=` → `converted_rules=`). Keeping the old name over the new
  quantity would have been the cheaper diff and a lie in the evidence line.

  **The `RETRO_ACTOR` control cycle 5 named was NOT built, and the reason is scope.** Cycle 5
  recorded the mechanical fix as one condition in `scripts/verify.sh` — refuse to emit a derived
  retro event when `RETRO_ACTOR` is unset, rather than silently falling back. `scripts/verify.sh`
  is **not** in this cycle's granted file-touch set (`scripts/pcgen_residue_gate.py` and its test
  are the only `scripts/` paths named), so the change is **prepared and escalated, not made**:
  guard `emit_retro_event` and `emit_disk_pressure_event` with
  `[[ -n "${RETRO_ACTOR:-}" ]] || return 0`. This cycle's own events landed correctly because
  every command that could emit one exported `RETRO_ACTOR` in the same `Bash` call —
  `docs/retro/events/sd31-transcribe.jsonl` is untouched at the final tree — but that is again a
  habit, and `AGENTS.md` rule 8 is explicit that a habit is not a control. **The operator ruling
  needed is one line of write scope to `scripts/verify.sh`.**

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design)`.
  The remainder is 35 hits / 17 files, all under `src/rules_core/`, and after this cycle it is
  **three** pieces rather than two, because the pool guards separated out:

  1. **`lst_parser_types` (12) + `ingest_record_tokens` (5) + `ir_converter` (4) +
     `source_content_payload` (5) = 26 hits are ONE piece of work**, and the largest item left in
     Epic 6: `sheet_rule_convert` must emit an equipment / equipment-modifier / spell rule shape
     the live side owns, and `equipment_effects`, `encumbrance`, `damage_total`,
     `equipment_resolver`, `spell_resolver` and `corpus_loader` must read that instead of
     `EquipmentRecord.tokens` / `.bonus_chains`. The converted package already holds 6,223
     equipment and 1,532 equipment-modifier records, and cycle 5's closure-row join is how those
     13 files reach them.
  2. **The `renderer` group (5) is a CONVERTER cycle.** Three named populations, enumerated by
     key in `AT-35-E6-003-RULED_cycle2_prose_parity_census.json`: 1,351 keys whose converted rule
     carries extra `Desc` segments; 718 keys the converter renders and the live path does not;
     374 the converted rule cannot render. The live swap in `class_feature_grant_consumer.rs` is
     one commit once that census reads `disagree=0`.
  3. **`trait_and_pool_tokens` (4) is now a NAMED CONVERTER DEFECT, not an unknown.** One hit
     (`class_feature_pool_catalog.rs`) clears when `ABILITY:` stops mapping to `MapsTo::Applies`
     and the rule that declares a `BONUS:VAR` references it, so
     `has_no_engine_effect_token`'s converted form stops disagreeing on 6,494 of 18,043 records.
     The other three (`race_resolver` ×2, `skinwalker_change_shape`) ride on item 1's race-trait
     rule shape.
