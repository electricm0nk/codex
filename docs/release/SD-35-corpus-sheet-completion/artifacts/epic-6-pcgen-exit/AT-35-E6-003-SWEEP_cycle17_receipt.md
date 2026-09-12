# Cycle AT-35-E6-003-SWEEP cycle 17 — Epic 6 PCGen exit / AT-35-E6-003-SWEEP

- **Commit SHA:** `9713a6f347` (code), plus this receipt's own commit.

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by
  design; decisions.md §2, workflow-instruction.md §6 step 1)`

  Both gates ran anyway, at the cycle's start tree `776a151d34`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree:
  ```
  live_files=46 live_hits=359 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

  **The dispatch's cycle number was wrong for the seventh consecutive cycle.** It said
  "CYCLE NUMBER FOR THIS CRITERION: 15". Sixteen receipts are on disk, so this is
  cycle **17**, and the refused-token line it carried (summing to 359) was cycle 15's,
  which cycle 16's receipt already restates. Re-derived, not assumed:
  ```bash
  ls docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle*_receipt.md | wc -l
  # -> 16, plus one
  ```
  Recorded as `correction 1789206037230-at-35-e6-003-sweep-c0f3a2`. Cycles 11–16 each
  recorded this same defect; `AGENTS.md` rule 8 says a seventh recurrence is a missing
  mechanism, and the mechanism is still one line — derive the number from the receipt
  directory at dispatch time.

- **What this cycle did.** Cycle 16 removed the converter-side blocker and named the
  unblocked job in three numbered steps. This cycle ran all three, and the criterion
  clause they serve is now met.

  > `render_pcgen_desc` is deleted from the live side; its `%N` substitution already
  > happened in the converter.

  `pattern render_pcgen_desc files=3 hits=39` → `files=0 hits=0`, and with it **every
  identifier pattern the gate scans is 0 on the live side**
  (`identifier_files=0 identifier_hits=0`).

- **Files touched:**
  - `apps/desktop/src-tauri/src/converted_prose.rs` → `src/rules_core/converted_prose.rs`
    (step 1 of cycle 16's list: the join moved where both crates reach it). The desktop
    crate re-exports it under its original path, so no call site in that crate changes.
    Gains **step 5** of the join and an id-returning twin of each step.
  - `src/rules_core/class_feature_pool_catalog.rs` — reads the converted package instead
    of rendering `data.description` at run time; the stub-marker guard made
    case-insensitive and moved onto the served words.
  - `src/rules_core/pilot_compute/class_feature_grant_consumer.rs` — same rewire, through
    `settled_description_for`.
  - `src/rules_core/sheet_rule_catalog.rs` — `prose_has_a_slot_no_character_settles`, the
    predicate that separates the two questions.
  - `src/rules_core/pcgen_desc.rs` → `src/pcgen_import/pcgen_desc.rs` (step 2), with every
    import path followed: `src/rules_core/{pilot_compute/mod,race_resolver}.rs`,
    `src/pcgen_import/class_feature_vars.rs`, four `src/bin/` generators, nine
    `apps/desktop/src-tauri/src/` catalogs, two `tests/`.
  - `tests/sd35_class_feature_catalogs_read_converted_prose.rs` — **new**, the two
    corpus-wide gates, RED first.
  - `docs/retro/events/at-35-e6-003-sweep.jsonl`, this receipt,
    `docs/release/SD-35-corpus-sheet-completion/{progress.md,kanban.md}`.
  - Folded, not this cycle's work, under the standing operator ruling on live appends:
    `docs/retro/events/sd31-transcribe.jsonl` (one append by another session) and
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    (this cycle's own `--check` `derived_at` stamp).

  **`data/corpus/**` and `data/sheet_rules/**` are byte-identical at HEAD.** No corpus
  record and no converted rule changed; the corpus and the package were read, never
  written.

- **Identifier audit result:** **OK_NO_BUNDLE_TAGS**, on this cycle's own diff. The
  hits the grep returns are entirely the documented citation-exclusion class that
  cycles 9–16 recorded — a test **file name** quoted inside a doc comment
  (`tests/sd35_class_feature_catalogs_read_converted_prose.rs`), and `diff --git` /
  `---` / `+++` headers naming `tests/sd27_*` and `tests/sd35_*` files. No identifier
  in shipping code:
  ```bash
  git diff --unified=0 776a151d34..HEAD -- src/rules_core src/pcgen_import src/bin \
    apps/desktop/src-tauri/src tests ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' \
    | grep -vE '^[0-9]+:(diff --git|---|\+\+\+)|/// `tests/'
  # -> no output
  ```

- **Wired-integration audit result:** **OK_NO_TOKENS**, after a self-heal named here
  rather than suppressed. The first run of the full suite failed
  `tests/sd24_wired_integration_audit.rs::placeholder_findings_are_ui_text_prose_or_the_one_documented_deferral`
  on **two strings this cycle wrote itself** — assertion messages using the word
  "placeholder" for the ingest format's `%N` marker. The audit was right to refuse
  them: it cannot tell a new stub marker from a new use of the word. Reworded to
  `%N slot marker` (the helper renamed with them), and the audit is green:
  `5 passed; 0 failed`. A single-token audit violation is `workflow-instruction.md §8`'s
  first self-healable case.

- **Acceptance criterion** (verbatim, `epic-breakdown.md`):

  > ### AT-35-E6-003 — the desktop crate and the prose renderer leave PCGen behind
  >
  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge /
  > `reach_gate.rs` readers of `raw_tokens` read `SheetRule.applies` and
  > `SheetRule.prose` instead. `render_pcgen_desc` is deleted from the live side;
  > its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under
  > `apps/desktop/`; desktop crate and frontend suites green; the 19 on-screen
  > tests still pass.

  All three of `AT-35-E6-003`'s evidence clauses hold, and its second sentence — open
  since the criterion was written — is met here for the first time.
  `AT-35-E6-004`'s closure bar (operator ruling B14, `decisions.md §17`:
  `live_files=0 live_hits=0`), which this sweep carries, is not met; the remainder is
  304 and is named by mechanism under **Refused tokens**.

- **Receipt rows (mechanical):**
  ```
  since=776a151d34 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=687 ratio=n/a builds_recorded=1 pcgen_live_files=45
  ```
  (`builds_recorded` reads 0 from the script because this cycle's build counter file
  is not the one it inspects; one full build+test pass was run and is quoted under
  **Build scope verified**.)

- **PCGen residue: `live_files=45 live_hits=304`, down from cycle 16's 46 / 359.**
  ```
  pattern raw_tokens files=0 hits=0
  pattern raw_bonus_chains files=0 hits=0
  pattern PcgenFormulaEvaluator files=0 hits=0
  pattern render_pcgen_desc files=0 hits=0
  pattern bonus_stack_reader files=0 hits=0
  pattern pre_tokens files=0 hits=0
  pattern BONUS: files=14 hits=91
  pattern DEFINE: files=0 hits=0
  pattern PRE[A-Z]+: files=17 hits=51
  pattern SAB: files=0 hits=0
  pattern DESC: files=26 hits=59
  pattern %CHOICE files=1 hits=3
  pattern %LIST files=0 hits=0
  pattern TYPE= files=20 hits=100
  root src/rules_core files=45 hits=304
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=0 hits=0
  identifier_files=0 identifier_hits=0
  live_files=45 live_hits=304 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Four patterns moved: `render_pcgen_desc` 39 → **0**, `%LIST` 1 → **0**, `%CHOICE`
  8 → **3**, `PRE[A-Z]+:` 61 → **51**. The last three moved because they were inside
  `pcgen_desc.rs`, which is now converter code; `render_pcgen_desc` moved for that
  reason **and** because its two live call sites were rewired first. Stated in both
  directions: **of the 55 hits removed, 51 left the live side by relocation and 4 by
  the rewire** — see the split immediately below, which is the figure that matters.

- **The reachable remainder went 8 → 4, and the 4 that closed are the whole
  code-bearing job cycle 16 named.**
  ```
  live_hits=304  live_files=45
  hits_inside_cfg_test=300  hits_outside=4
  files_test_only=44  files_with_non_test_hits=1
  ```
  Cycle 16 measured `hits_inside_cfg_test=351 hits_outside=8`, the 8 in 4 files. This
  cycle closed the 4 that were real code reads:

  | job | cycle 16 | now |
  |---|---|---|
  | the `render_pcgen_desc` catalog rewire (`class_feature_pool_catalog.rs`, `pcgen_desc.rs`, `pilot_compute/class_feature_grant_consumer.rs`) | 4 hits, 3 files | **0** |
  | the `PU_*_DESC_TOKEN` verbatim corpus transcriptions (`pilot_compute/mod.rs`) | 4 hits, 1 file | 4 hits, 1 file |

  The four `PU_*_DESC_TOKEN` constants are the book's own words, pinned byte-for-byte
  against the `.lst` files by
  `sd27_pu_class_feature_descriptions_carry_the_characters_numbers`. Cycles 10–16
  declined to reword them and this cycle declines for the same reason. They are
  **counted** in the remainder, never exempted (`NO CARVE-OUTS`). Re-derive:
  ```bash
  python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle5_test_region_census.py
  ```

- **Oracle parity:** **not run, and why.** No `Number` mapping was added
  (`workflow-instruction.md §6` step 3 runs the oracle comparison when one is); this
  cycle adds no magnitude and no formula. No corpus record and no converted rule
  changed — `data/sheet_rules/**` is byte-identical, so nothing in this cycle can
  move a parity artifact. Cycle 13's `…_cycle13_sheet-parity-after.json` remains the
  standing one.

- **Two defects found, both measured before being fixed.**

  1. **32 of the 4,463 records the two `class_feature` catalogs served printed a stub
     marker on a character sheet.** `carries_unimplemented_marker` compared the
     **lowercase** spellings only, and the corpus states both cases: 85 `class_feature`
     records carry a marker, 68 of them in a case the guard could not see. A player's
     sheet read *"[NOT IMPLEMENTED] A magic warrior learns ancient secrets…"* — the
     exact defect `docs/governance/no-stub-mvp-doctrine.md` exists to catch, shipped by
     the guard written to prevent it. Recorded as
     `correction 1789206037367-at-35-e6-003-sweep-c0c035`.

     The fix is case-insensitivity **and** a change of surface: the guard now asks
     about the words that print, not the corpus row they came from. That second half
     is load-bearing and measured — for **30 of the 32** the converter's prose comes
     from a source row that never carried the marker, so refusing on the raw row would
     have thrown away a clean sentence over an annotation that never reaches the page.
     Now **0 of 5,256**.

  2. **A blocker document's mechanism was stale — and so was a test, red for two
     cycles in a crate nobody ran.**
     `converted_prose::a_restricted_printing_serves_its_base_record_and_an_unheld_key_serves_nothing`
     asserted that `Nondetection (self only)` serves `Nondetection`'s words. The
     package holds `core_rulebook:spell:nondetection_self_only` as a record of its own,
     so step 1 of the join answers directly and the restricted text is served — which
     is correct, and better. The test has been RED since cycle 16 regenerated 310 rule
     files; it lived in the desktop crate, which cycle 16 did not run. **Moving the
     module into the library is what surfaced it**, which is cycle 15's own finding
     (epic cadence hides a red crate) firing a third time. Recorded as
     `correction 1789206037495-at-35-e6-003-sweep-a7b20f`. Rewritten against the
     package rather than against its own memory, and it now exercises step 4 on a
     qualifier the package genuinely holds nowhere — `Planar Binding (Demons Only)` is
     deliberately **not** the example, because `advanced_players_guide` holds it and
     step 3 answers it.

- **RED → GREEN, in that order.** Both gates in
  `tests/sd35_class_feature_catalogs_read_converted_prose.rs` were written and run
  **before** a line of `src/` was edited:
  ```
  class_feature catalog records served=4463 without converted prose=3
  3 of 4463 ... e.g. ["core_rulebook:Wizard ~ Spells",
    "inner_sea_intrigue:Codex-Named Unit (class_feature_inner_sea_intrigue_isi_abilities_class_lst_187)",
    "ultimate_combat:Master Of Many Styles ~ Perfect Style"]
  class_feature catalog records served=4463 printing a stub marker=32
  test result: FAILED. 0 passed; 2 failed
  ```
  After the rewire, the join's step 5 and the guard fix:
  ```
  class_feature catalog records served=5256 without converted prose=0
  class_feature catalog records served=5256 printing a stub marker=0
  test result: ok. 2 passed; 0 failed
  ```

- **793 more records now state their description, and the reason is the sheet rule.**
  The catalogs serve **5,256**, up from **4,463**. The old render-and-refuse pair threw
  away the whole sentence whenever a `%N` argument could not be substituted; the
  converted rule carries the same magnitude as a **typed** hole and
  `sheet_rule_catalog::render` prints an unsettled hole as the term's own words —
  `decisions.md §1`'s third permitted printed form. `Rogue Talent ~ Bleeding Attack`
  is the worked example, and its test is rewritten to assert what must still never
  happen: a gap where the magnitude was, a guessed number, or a `%N` marker on the
  page.

- **How losslessness is proved** — two gates over the live corpus and the live
  converted package, not a fixture (`decisions.md §4`). Both walk every record the two
  catalogs serve, join it to the package, and refuse to pass on an empty walk
  (`served > 500` must hold, so a walk that silently stopped finding records cannot
  agree with itself about nothing). The figures are printed, never asserted as
  constants: ingesting a book moves them and this file must not have to change.

- **What the proof does NOT cover** (`AGENTS.md` rule 7).
  1. The two gates cover the **`class_feature`** kind only — the kind the two rewired
     call sites read. Every other kind's catalog already read the converted package
     before this cycle and is covered by its own crate's tests, but nothing in this
     cycle re-proves them.
  2. **Step 5 of the join is proved on the two live collisions and no others.** Both
     are a record and its own token-only twin, where "the variant that has words" names
     a single record. A future collision whose variants *both* state prose resolves to
     `None` by construction, which is a refusal and not a wrong answer — but no test
     exercises that arm against a real package record, because the package currently
     holds none. Re-derive the population:
     ```bash
     ls data/sheet_rules/*/*/*__*.json | wc -l
     ```
  3. The **69 records cycle 16 identified as fixed-but-ungated** (corpus slug ≠ rule
     slug) are still ungated. This cycle did not build the slug-independent join cycle
     16 named; step 5 addresses a different shape (a shared key), not that one.

- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** none. Epic 6 closes no corpus unit, by design.
  - **relabel (bucket to bucket):** none in the committed inventory —
    `regressed=0 added=0 dropped=0`, and `docs/work-inventory.json` is byte-identical
    to `776a151d34`'s. See the row below for the candidate run's measured movement and
    why it was not written.
  - **reachability:** **55 residue hits removed, 45 live files from 46**, and the
    code-reachable remainder 8 → 4. The honest split is stated above: 4 by rewire,
    51 by relocating `pcgen_desc.rs` to the converter side where `decisions.md §11`
    says it belongs.
  - **instrument-correction:** two corpus-wide gates added; a stub-marker guard
    corrected in case and in surface; a two-cycle-red test corrected against the
    package; a pinned census re-derived.

- **`docs/work-inventory.json` was NOT written, and here is exactly what the candidate
  said.** The guarded run refused, as it should:
  ```
  refusing to write ...: this run would drop 230 of the 32617 verification stamp(s)
  ```
  `--allow-stamp-loss` is forbidden (`workflow-instruction.md §6` step 3), so instead
  the candidate was produced with `--stdout-only` and diffed against the committed
  file. **Its only movement:**
  ```
  old units: 49438   new units: 49438
  status transitions (old -> new):
       230  sheet-complete -> text-complete
  evidence transitions at unchanged status:
        92  sheet_rule_rendered:words -> sheet_rule_rendered:number
  old engine-does-not-hold: 0   new: 0
  ```
  Both statuses are DONE and **non-DONE is 0 before and 0 after — nothing regressed.**
  The 230 are the catalog widening shadowing a *stronger* evidence rung with a weaker
  one: `v06_work_inventory` decides `text-complete` before a unit reaches the
  `sheet-complete` rung, so a record the widened catalog now serves loses a
  `sheet_rule_rendered` stamp it still earns. Writing that would record weaker evidence
  for 230 units that have the stronger one, so it was not written. The fix is a rung
  ordering change in `src/bin/v06_work_inventory.rs`, **outside this epic's file-touch
  set**; deferred as `deferral 1789209249119-at-35-e6-003-sweep-3fbe05`, with the
  92 `words → number` upgrades noted as the improvement it is currently withholding.

- **Refused tokens** — five types, summing to **304**, the gate's own `live_hits`:
  ```
  TYPE==100, BONUS:=91, DESC:=59, PRE[A-Z]+:=51, %CHOICE=3
  ```
  Two of cycle 16's seven types are gone entirely (`render_pcgen_desc`, `%LIST`).
  Recorded as `deferral 1789209249246-at-35-e6-003-sweep-b13e8b`. **300 of the 304 sit
  inside a `#[cfg(test)]` module of a live file** and no code work moves them without
  the operator ruling on whether such a region is live code — asked for since cycle 10,
  now eight cycles running. The 4 outside are the `PU_*_DESC_TOKEN` book transcriptions.

- **Discoveries:** three, each emitted as a `correction` retro event, and none of them
  predicted by `token-coverage.json` or the atlas.
  1. The dispatch's cycle number, wrong for the seventh consecutive cycle (`…-c0f3a2`).
  2. A stub marker printing on real character sheets for 32 records, because a guard
     against exactly that compared one case of a two-case corpus (`…-c0c035`).
  3. A test red for two cycles in a crate the epic's cadence had stopped running, found
     only because the module moved into a crate that does get run (`…-a7b20f`).

  The lesson across 2 and 3 is one lesson, and it is this repo's standing one:
  **a guard that is never asked about the real population is not a guard.** Both
  defects were invisible to presence checks and both were found by a walk over the live
  corpus.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=45 live_hits=304`, from 46 / 359 | every source file under the five live roots, code lines only (ruling B14) | `python3 scripts/pcgen_residue_gate.py --check` |
  | `hits_outside=4` in 1 file, from 8 in 4 | the 304 hits, split on `#[cfg(test)]` membership | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle5_test_region_census.py` |
  | served 4,463 → 5,256; without converted prose 3 → 0; printing a stub marker 32 → 0 | every record `load_pool_catalog` + `load_standalone_class_feature_catalog` return | `cargo test --locked --test sd35_class_feature_catalogs_read_converted_prose -- --nocapture` |
  | 85 `class_feature` records carry a marker, 68 in a case the old guard could not see | every `data/corpus/*/class_feature/**/*.json` whose `data.description` is a string | `grep -ric '\[not implemented\]\|\[not enforced\]' data/corpus/*/class_feature/` vs. the case-sensitive form |
  | 230 `sheet-complete` → `text-complete`, 92 `words` → `number`, non-DONE 0 → 0 | all 49,438 inventory units | `cargo run --locked --bin v06_work_inventory -- --stdout-only` (with both stamp reports set), diffed against `docs/work-inventory.json` |
  | grant-consumer census (136,32,0,43,1) → (131,37,0,43,1), sum unchanged at 168 | this module's own probe over `unambiguous_grants()` | `cargo test --locked --lib -- ...::the_live_scale_of_this_waves_widening_is_measured_and_pinned -- --nocapture` |
  | `records=49438 converted=49296 refused=142 rules=70135 var_tables=5293` | every corpus record the sheet-rule converter reads | `cargo run --locked --bin sheet_rule_convert -- --check` |
  | `rust_lines_changed=687` | the cycle's own diff since `776a151d34` | `python3 scripts/cycle_scope_gate.py --receipt --since 776a151d34 --before /tmp/wi-before-at-35-e6-003-sweep.json --after docs/work-inventory.json` |
  | 16 prior receipts, so this is cycle 17 | this criterion's receipts on disk | `ls docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle*_receipt.md \| wc -l` |

- **Build scope verified** — **once**, after the last figure-moving edit, at `9713a6f347`:

  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`
  - `cargo test --locked --lib -j 6` → **3,341 passed / 0 failed / 15 ignored**, `LIB_EXIT=0`
    (cycle 16 recorded 3,335; the **+6** are this cycle's own tests — two new corpus-wide
    gates and four rewritten in place, which is the arithmetic that says no target was lost)
  - `cargo test --locked --no-fail-fast -j 6` → **394 targets / 8,656 passed / 0 failed /
    67 ignored**, zero `FAILED` lines after the single-token audit self-heal named above
  - `cargo clippy --locked --tests -j 6` → **`CLIPPY_EXIT=0`, 0 warnings, 0 errors**
    (one `dead_code` warning on this cycle's own test helper, fixed by `#[cfg(test)]`
    in the same cycle, not carried)
  - `cargo run --locked --bin sheet_rule_convert -- --check` →
    `records=49438 converted=49296 refused=142 rules=70135 var_tables=5293 verdict=PASS`,
    **identical to cycles 3–16** — this cycle converts no record and changes no rule,
    which is why every one of those five figures is unmoved
  - `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → **0**
  - `python3 scripts/pcgen_residue_gate.py --check` → `live_files=45 live_hits=304 verdict=PASS`,
    never above the start tree at any point in the cycle
  - `python3 scripts/completion_atlas.py --check` → `population=49438 buckets=10
    unclassified=0 overlap=0 DONE=49438`, `done_evidence_violations=0
    missing_clearing_mechanisms=0 citation_failures=0`, exit 0
  - `python3 scripts/token_coverage.py --check` → `non_done=0 tokened=0 token_less=0
    refused=142 refused_non_done=0 token_types=233 shapes=1 verdict=PASS`, exit 0
  - `python3 scripts/shape_engine_boundary.py --check` → `magnitude_bearing=26396
    not_held_by_engine=0 citation_ok=True`, exit 0
  - `python3 scripts/missing_engine_tables.py --check` → `population=0 kinds=0
    citation_failures=0`, exit 0
  - `scripts/verify.sh --only pi-sweep` → **RESULT: PASS** (1 stage passed)
  - `python3 scripts/denominator_gate.py --check` over this package and its artifacts —
    run after this receipt was written; see the progress row for its output
  - **Desktop crate:** this cycle touched nine files under `apps/`, so the crate's own
    suite was run rather than deferred to epic cadence — see the progress row.
  - `cargo run --locked --bin corpus_literal_sweep -- --json-out` → **CLEAN**, 48,706
    records examined of 51,476 read, 413,314 tokens compared, **0 findings**. Run as an
    input to the guarded inventory attempt, not because corpus records changed
    (`git status --porcelain data/corpus` is empty).

- **Sweep population:** 48,706 examined of 51,476 read, 0 findings — unchanged corpus,
  swept only to feed the inventory's stamp guard.

- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.

- **Status: `partial`.** The criterion's population is **304**, not zero, at HEAD.
  `AT-35-E6-003`'s own evidence clauses are met and its `render_pcgen_desc` clause is
  met for the first time; `AT-35-E6-004`'s closure bar, which this sweep carries, is
  not.

- **Notes:** the judgment call was not writing `docs/work-inventory.json`. The candidate
  regresses nothing and improves 92 rows, and writing it would have made this receipt
  look like it moved the inventory — but it would also have recorded weaker evidence for
  230 units, and the only flag that writes it is one the brief forbids. Refusing costs a
  row in the next cycle's scope and no correctness.

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle)`; the whole remainder,
  **304 code hits in 45 files**, of which **4 hits in 1 file** are reachable without an
  operator ruling.

  **There is no code-bearing sweep job left in this criterion.** Cycle 16 said a further
  sweep pass would close nothing and named the rewire instead; that rewire is done, and
  the census now reads `files_with_non_test_hits=1` — the four `PU_*_DESC_TOKEN`
  constants, which are the book's own words and which seven cycles have declined to
  reword. Dispatching another sweep against this remainder will close zero hits, and
  will do so for a reason that is now structural rather than incidental.

  What is actually left, in the order it can be taken:
  1. **The operator ruling on `#[cfg(test)]` regions inside live files.** It stands
     between the gate and **300 of the 304**. Eight cycles have asked. This is not a
     cycle-sized task and no dispatch can substitute for it.
  2. **The inventory rung ordering** (`src/bin/v06_work_inventory.rs`): `text-complete`
     is decided before the `sheet-complete` rung, so a widened catalog costs 230 units
     their stronger evidence. Needs a file-touch set that includes that binary. It is
     also what currently withholds 92 `words → number` upgrades from the committed
     inventory.
  3. **The slug-independent join** cycle 16 named, which would take its description gate
     from 241 of 310 records to all 310. Smaller than either of the above, and
     independent of both.
