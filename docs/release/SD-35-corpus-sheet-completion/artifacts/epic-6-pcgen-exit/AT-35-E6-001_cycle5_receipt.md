# Cycle 5 — Epic 6 (PCGen exit) / AT-35-E6-001

**This cycle ships no code.** It is a **re-dispatch of an already-complete criterion**: the
dispatch prompt said `CYCLE NUMBER FOR THIS CRITERION: 1`, but cycles 1–4 had already run and
`kanban.md` row 24 already read `done`. The criterion's three identifiers were at
`files=0 hits=0` **before** this cycle touched anything. What this cycle owes, and delivers, is
every clause of the criterion's own **Evidence** sentence **re-derived at HEAD** — nothing redone.
Logged as `incident 1789065539103-at-35-e6-001-cc46e7`, recurrence key
`stale-census-in-dispatch-prompt`.

- **Commit SHA:** `0afbd036b8` — the commit carrying this receipt, the two folded working-tree
  files, this cycle's retro events, and the `progress.md` / `kanban.md` rows. A second commit
  pins that SHA into this line (a receipt cannot name the commit that carries it). Cycle start
  `1e982aa94b`.
- **Scope gate:**
  `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design, decisions.md §2)`.
  Run anyway, for the record — `python3 scripts/cycle_scope_gate.py --min 500`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  `remaining_non_done=0` — the corpus reached `DONE 49438 of 49438` at `AT-35-E5-005`. Epic 6
  moves no unit; it removes the ingest-format engine from the live side.
- **Files touched:** **zero code, zero data.** This receipt (new); `progress.md` and `kanban.md`
  (rows); `docs/retro/events/at-35-e6-001.jsonl` (this cycle's two incidents); and two files the
  cycle's own read-only instruments wrote and it folds rather than leaves loose
  (`clean tree = unfiltered git status empty`):
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (the
  `derived_at` re-stamp `completion_atlas.py --check` writes) and
  `docs/retro/events/sd31-transcribe.jsonl` (the derived verify event `scripts/verify.sh`
  appended — misfiled there, see Notes).

  **Proof that no code moved:** `git diff --name-only 9f54c1e490..HEAD -- src/ data/ apps/
  Cargo.toml Cargo.lock` prints **nothing**. `9f54c1e490` is cycle 4's final tree — the tree cycle
  4 ran its lib, `--no-run` and full workspace suites on. The tree under test in this receipt is
  byte-identical to it in every compiled input.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS in shipping code — **1 match, attributed, not a
  violation.**
  ```
  BASE_BRANCH=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47   # git merge-base HEAD origin/develop
  CODE="src/pcgen_import src/rules_core src/bin src/oracle_validation apps/desktop/src-tauri/src"
  git diff --unified=0 "${BASE_BRANCH}...HEAD" -- $CODE ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  ->  14968:+/// (`tests/sd34_wave51_racial_sla_catalog_matches_the_corpus.rs`); this function is its
  ```
  That is a **doc-comment citation of a real test file's path** in `src/rules_core/racial_sla.rs`
  (lines 98 and 869), not a bundle-tagged identifier. Same disposition every sibling cycle 2
  receipt recorded (`AT-35-E5-001` … `AT-35-E5-005` cycle 2). **This cycle contributes 0** — it
  changed no `.rs` line. Over the epic-6 artifact directory as well, the only further matches are
  earlier receipts' own prose quoting those same pre-existing test filenames.
- **Wired-integration audit result:** OK_NO_TOKENS.
  ```
  git diff --unified=0 "${BASE_BRANCH}...HEAD" -- $CODE ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'   ->  OK_NO_TOKENS
  ```
- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E6-001`):
  > `PcgenFormulaEvaluator` and `formula_interpreter*.rs` move to `src/pcgen_import/` (the
  > converter's parser). Every live caller (14 files at authoring — `racial_sla.rs`,
  > `domain_power`, trait/feat effects, the pilot_compute formula paths) is replaced by
  > `sheet_rule::evaluate` over converted `Expr`, or deleted where the sheet line already
  > carries the value. `bonus_stack_reader.rs` and `feat_prereqs/pre_tokens.rs` move with it.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows `PcgenFormulaEvaluator`,
  > `bonus_stack_reader`, `pre_tokens` at 0 live hits; the oracle comparison agrees before and
  > after; full workspace suite green.

  **The move itself, re-derived at HEAD** (`git ls-files | grep -E
  'formula_interpreter|bonus_stack_reader|pre_tokens'`):

  | file | lives at HEAD | side |
  |---|---|---|
  | `formula_interpreter.rs` | `src/pcgen_import/` | converter |
  | `formula_interpreter_corpus_wide.rs` | `src/pcgen_import/` | converter |
  | `bonus_stack_reader.rs` | `src/pcgen_import/` | converter |
  | `pre_tokens.rs` | `src/pcgen_import/` | converter |
  | `src/bin/formula_interpreter.rs`, `src/bin/bonus_stack_reader.rs` | `src/bin/` | converter tooling, **kept** (`decisions.md §11`) |

  `ls src/rules_core/ | grep -E 'formula_interpreter|bonus_stack_reader' | wc -l` → **0**.
  `src/rules_core/feat_prereqs/` still exists (`combat.rs`, `converted_gate.rs`, `general.rs`,
  `item_creation.rs`, `metamagic.rs`, …) and no longer carries `pre_tokens.rs`.
- **Receipt rows (mechanical):**
  ```
  since=1e982aa94b4ecd2034d5c611b7433f1c32589f68 target_dir=/tmp/cargo-sd35-AT-35-E6-001 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=1 pcgen_live_files=253
  ```
  `closed=0` / `relabeled=0` is correct and expected: the unit population was already `0`
  non-DONE when the cycle started. `rust_lines_changed=0` is this cycle's own signature — it
  shipped no Rust. `ratio` is `n/a`, a division by zero, never `0.0`.
- **PCGen residue** (`python3 scripts/pcgen_residue_gate.py --check`, run at HEAD `1e982aa94b`):
  ```
  pattern PcgenFormulaEvaluator files=0 hits=0
  pattern bonus_stack_reader   files=0 hits=0
  pattern pre_tokens           files=0 hits=0
  live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  **All three identifiers this criterion owns are at zero live hits**, and the overall figure is
  **flat** at cycle 4's `253 / 12,256` — this cycle raised nothing. The baseline file is still
  deliberately NOT rebaselined; `--rebaseline` is `AT-35-E6-004`'s step.

  | identifier | end of cycle 4 | at HEAD (this cycle) |
  |---|---|---|
  | `PcgenFormulaEvaluator` | files=0 hits=0 | files=0 hits=0 |
  | `bonus_stack_reader` | files=0 hits=0 | files=0 hits=0 |
  | `pre_tokens` | files=0 hits=0 | files=0 hits=0 |

  Independent second derivation, agreeing at **0** live caller files across all three:
  ```
  for p in PcgenFormulaEvaluator bonus_stack_reader pre_tokens; do
    grep -rl "\b$p\b" src/rules_core src/saved_character src/campaign src/homebrew_authoring apps/desktop \
      --include=*.rs --include=*.ts --include=*.tsx | grep -v '^src/rules_core/cache_gen/'; done | sort -u | wc -l
  ->  0
  ```
- **Oracle parity:** unchanged and re-read at HEAD from the committed artifact — this cycle
  evaluated nothing new, so agreement cannot have moved. `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.
  ```
  jq -c '.verdict, .population' AT-35-E6-001_cycle4_varchain-parity.json
  {"agree":199610,"disagree":1070,"lost":0,"gained":80,"records_moved":23,
   "records_only_before":65,"resolved_values_inside_records_only_before":0}
  {"records_before":4140,"records_after":4075,
   "probes":["zero (every ability modifier 0)","spread (Str1 Dex2 Con3 Int4 Wis5 Cha6)"],
   "levels":"1..=20","value_pairs_compared":200680}
  ```
  `compared=200680 agree=199610 disagree=1070 lost=0 gained=80`. Every disagreement and every
  gain is the one mechanism cycle 4 named and pinned against the published rule (the retired
  evaluator matched variable names by exact bytes; the rule source is case-insensitive). The
  epic's PCGen-**oracle** before/after comparison belongs to `AT-35-E6-004` and is not this
  criterion's to close.
- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** 0 — the population was already 0 non-DONE at cycle start.
  - **relabel (bucket to bucket):** 0. No unit changed bucket.
  - **reachability:** 0. This cycle evaluated nothing and converted nothing.
  - **instrument-correction:** 0. No figure in cycle 4's receipt was found wrong; every one
    re-derived to the same value at HEAD.
- **Refused tokens:** none — this cycle added no converter refusal and cleared none. The refused
  set is unchanged at 142 records, one shape (`no_corpus_record`), `refused_non_done=0`.
- **Discoveries:** none. That is the intended outcome of a re-verification cycle: every Evidence
  clause re-derived to the value the previous receipt recorded. The one thing worth writing down
  is not a discovery about the corpus but about the dispatch — see Notes.
- **Figures + their re-derive commands:** every row carries its own command. The unit denominator
  throughout is the whole corpus, all books — `jq '.units | length' docs/work-inventory.json` → **49,438**.

  | figure | value | command | denominator |
  |---|---|---|---|
  | `PcgenFormulaEvaluator` live files / hits | **0 / 0** | `python3 scripts/pcgen_residue_gate.py --check`, `pattern PcgenFormulaEvaluator` line | 49,438 units |
  | `bonus_stack_reader` live files / hits | **0 / 0** | same, `pattern bonus_stack_reader` line | 49,438 units |
  | `pre_tokens` live files / hits | **0 / 0** | same, `pattern pre_tokens` line | 49,438 units |
  | live PCGen files / hits overall | **253 / 12,256** (flat vs cycle 4) | `python3 scripts/pcgen_residue_gate.py --check`, last line | 49,438 units |
  | live caller files remaining, all three identifiers | **0** | the `for p in …` loop above | 49,438 units |
  | files under `src/rules_core/` still named for a moved module | **0** | `ls src/rules_core/ \| grep -E 'formula_interpreter\|bonus_stack_reader' \| wc -l` | 4 moved modules |
  | compiled inputs changed since cycle 4's verified tree | **0 files** | `git diff --name-only 9f54c1e490..HEAD -- src/ data/ apps/ Cargo.toml Cargo.lock` | 53 files changed overall in that window, all docs/scripts/site |
  | chain values compared / agree / disagree / lost / gained | **200,680 / 199,610 / 1,070 / 0 / 80** | `jq -c '.verdict, .population' docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-001_cycle4_varchain-parity.json` | 4,075 records × 20 levels × 2 probes |
  | sheet-rule package | `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS` | `cargo run --locked --release --bin sheet_rule_convert -- --check` (20.1 s) | 49,438 units |
  | converted variable artifact | `class_feature_described=4075 class_feature_any=4445 class_records=27 domain_records=25 converted_vars=9566 var_defaults=350 verdict=PASS` | `cargo run --locked --release --bin gen_record_vars -- --check` | 18,263 corpus `class`/`class_feature` records |
  | `data/sheet_rules/` token leaks | **0** | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | 49,438 units |
  | atlas | `population=49438 buckets=10 unclassified=0 overlap=0 done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`; `DONE: 49438`, every other bucket `0` | `python3 scripts/completion_atlas.py --check` | 49,438 units |
  | token coverage | `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS` | `python3 scripts/token_coverage.py --check` | 49,438 units |
  | shape/engine boundary | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` | `python3 scripts/shape_engine_boundary.py --check` | 49,438 units |
  | missing engine tables | `population=0 kinds=0 citation_failures=0` | `python3 scripts/missing_engine_tables.py --check` | 49,438 units |
  | denominator gate | `files_checked=86 violations=0` (86 after this receipt landed; 85 before) | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` | 86 bundle docs |
  | figure provenance | `files_checked=203 figures_examined=416 violations=0` (after this receipt landed; `202 / 402` before) | `python3 scripts/denominator_gate.py --check-provenance` | 416 figures |
  | tool side intact — net deletions of function bodies | **630 insertions, 9 deletions**, zero net deletions | `git diff --stat 292d90f13e..HEAD -- src/pcgen_import scripts/oracle_harness src/oracle_validation` | 4 files in that window |
  | oracle harness present | `ORACLE_HARNESS_PRESENT` | `test -f scripts/oracle_harness/run.py` | — |
- **Build scope verified**, all run at HEAD `1e982aa94b`, cold target dir
  `/tmp/cargo-sd35-AT-35-E6-001`, `CARGO_INCREMENTAL=0`:
  - `cargo test --locked --no-run -j 6` → **`NO_RUN_EXIT=0`**, **413 `Executable` lines** — every
    bin and test target links (`grep -c '^  Executable' <log>`).
  - `cargo test --locked --lib -j 6` → **`ok. 3261 passed; 0 failed; 15 ignored`** (41.45 s),
    `LIB_EXIT=0` — identical to cycle 4's figure, on an identical tree.
  - `cargo test --locked --no-fail-fast -j 6` → **`FULL_EXIT=0`**, **414 `test result` lines
    (413 test/bin targets plus the lib), 8,772 passed, 0 failed, 68 ignored, and ZERO failing
    suites** — `grep -cE '^test result: FAILED' /tmp/e6c5-full.log` → **0**, and
    `grep -cE '^(error|warning)' /tmp/e6c5-full.log` → **0**. Totals derived with `awk` over the
    `test result` lines, not `grep -o` (`AGENTS.md` §Concurrency). **This is a real run at HEAD,
    not an inherited claim** — and it reproduces cycle 4's figure exactly (414 / 8,772 / 68 / 0),
    which is what an unchanged compiled tree must do. The count stays at 414 because this cycle
    added no target; nothing in `tests/`, `src/`, `apps/` or `scripts/` asserts either 413 or 414.
  - `cargo run --locked --release --bin sheet_rule_convert -- --check` → `verdict=PASS`, figures
    above; the shipped `data/sheet_rules/` package is untouched by this cycle.
  - `cargo run --locked --release --bin gen_record_vars -- --check` → `verdict=PASS`, figures
    above; the shipped `data/converted/record_vars.json` is fresh against the corpus.
  - `cargo clippy --locked --tests -j 4` (its own target dir `/tmp/cargo-sd35-AT-35-E6-001-clippy`,
    so it did not contend on the workspace run's cargo lock) → **0 warnings, 0 errors**,
    `Finished dev profile in 1m 44s`.
  - `scripts/verify.sh --only pi-sweep` → **`RESULT: PASS`** (1 stage; 11 hits over
    `src/rules_core/rules_tables`, 11 baseline rows).
  - `cargo run --locked --bin v06_work_inventory`: **not run, deliberately.** This cycle changed
    no corpus record, so `corpus_literal_sweep` is guarded off (`§6` step 3), so the two reports
    the binary rebuilds its 32,617 verification stamps from do not exist for this tree — it would
    refuse to write, exactly as it did in cycle 4, and `--allow-stamp-loss` is forbidden.
    `git status --porcelain -- docs/work-inventory.json` is empty and the `--receipt` rows above
    were computed against the file on disk.
  - `apps/desktop/src-tauri` and the frontend: **epic cadence** — this cycle touched no file under
    `apps/`, and neither did cycle 4.
- **Sweep population:** N/A — no corpus record changed this cycle (no `data/` file changed at
  all), so `corpus_literal_sweep` was correctly not run (`§6` step 3's guard).
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`, unchanged by this cycle).
- **Status:** **complete** — every clause of the criterion's Evidence sentence is true at HEAD and
  re-derived here: the residue gate shows `PcgenFormulaEvaluator`, `bonus_stack_reader` and
  `pre_tokens` at `files=0 hits=0`; the parity comparison agrees before and after with 0 losses
  and every moved value attributed to one named mechanism; the full workspace suite is green.
- **Notes:**
  - **Re-dispatch of an already-complete criterion.** `incident
    1789065539103-at-35-e6-001-cc46e7`, recurrence key `stale-census-in-dispatch-prompt`. The
    prompt's `CYCLE NUMBER FOR THIS CRITERION: 1` was stale by four cycles; this receipt is
    numbered **5**. No other criterion's card was touched.
  - **`RETRO_ACTOR` lost between bash calls, second occurrence.** `scripts/verify.sh` appended its
    derived verify event to `docs/retro/events/sd31-transcribe.jsonl` rather than this criterion's
    log, because the variable does not survive between separate shell invocations in this harness.
    Cycle 4 hit the same thing (`1789015512888-at-35-e6-001-67f8e2`). Re-logged as `incident
    1789065549602-at-35-e6-001-cbd578`, recurrence key `retro-actor-lost-between-bash-calls`. The
    log is append-only, so the line is left where it fell and folded into this cycle's commit.
    **Two occurrences in two cycles is a missing mechanism, not bad luck** (`AGENTS.md` §8): the
    fix is for `verify.sh` and `retro.py` to derive the actor from the cycle's receipt path or a
    written-down file rather than an environment variable, and it belongs to whoever owns the
    instrument, not to this criterion.
- **Next-cycle scope:** **criterion at zero.** Its three named identifiers are `files=0 hits=0`
  and its files live on the converter side. What remains live belongs to its siblings:

  | family | live files | live hits | owner |
  |---|---|---|---|
  | `raw_tokens` | 41 | 234 | `AT-35-E6-002` / `AT-35-E6-003` |
  | `render_pcgen_desc` | 17 | 109 | `AT-35-E6-002` / `AT-35-E6-003` |
  | source-token literals in live code | 253 files total | 12,256 | `AT-35-E6-002`..`004`; `--closure` at `AT-35-E6-004` |
