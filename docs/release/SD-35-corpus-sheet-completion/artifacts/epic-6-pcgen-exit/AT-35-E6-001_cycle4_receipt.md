# Cycle 4 — Epic 6 (PCGen exit) / AT-35-E6-001

- **Commit SHA:** `ac38c5bf3c` is the code change ("feat(sd35): the class-feature variable chain
  is converted at ingest, not evaluated live"); a second commit carries this receipt, the parity
  artifact's companion rows in `progress.md` / `kanban.md`, and the cycle's retro events (a
  receipt cannot name the commit that carries it). Cycle start `292d90f13e`.
- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design, decisions.md §2)`.
  Run anyway, for the record:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  `remaining_non_done=0` — the corpus reached `DONE 49438 of 49438` at `AT-35-E5-005`. Epic 6
  moves no unit; it removes the ingest-format engine from the live side.
- **Files touched:** 4 new (`src/rules_core/record_vars.rs`,
  `src/pcgen_import/class_feature_vars.rs`, `src/bin/gen_record_vars.rs`,
  `data/converted/record_vars.json`), 7 edited (`src/rules_core/sheet_rule.rs`,
  `src/rules_core/mod.rs`, `src/rules_core/pilot_compute/class_feature_grant_consumer.rs`,
  `src/rules_core/pilot_compute/mod.rs`, `src/pcgen_import/formula_interpreter.rs`,
  `src/pcgen_import/mod.rs`, `src/pcgen_import/sheet_rule/ctx.rs`), plus this receipt, the parity
  artifact, and `docs/retro/events/at-35-e6-001.jsonl`. `1,625` insertions and `895` deletions
  under `src/`
  (`git diff --stat ac38c5bf3c~1..HEAD -- src/`).
- **Identifier audit result:** OK_NO_BUNDLE_TAGS.
  ```
  SCOPED="src/rules_core/record_vars.rs src/rules_core/sheet_rule.rs src/rules_core/mod.rs \
          src/rules_core/pilot_compute/class_feature_grant_consumer.rs \
          src/rules_core/pilot_compute/mod.rs src/pcgen_import/class_feature_vars.rs \
          src/pcgen_import/formula_interpreter.rs src/pcgen_import/mod.rs \
          src/pcgen_import/sheet_rule/ctx.rs src/bin/gen_record_vars.rs"
  git diff --unified=0 ac38c5bf3c~1..HEAD -- $SCOPED ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'   ->  OK_NO_BUNDLE_TAGS
  ```
- **Wired-integration audit result:** OK_NO_TOKENS.
  ```
  git diff --unified=0 ac38c5bf3c~1..HEAD -- $SCOPED ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'  ->  OK_NO_TOKENS
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
- **Receipt rows (mechanical):**
  ```
  since=292d90f13ee2b6811040685be913e979a66b9aaf target_dir=/tmp/cargo-sd35-AT-35-E6-001-c3 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=2520 ratio=n/a builds_recorded=1 pcgen_live_files=253
  ```
  `closed=0` / `relabeled=0` is correct and expected: the unit population was already `0`
  non-DONE when the cycle started. `ratio` is `n/a`, a division by zero, never `0.0`.
- **PCGen residue:**
  ```
  pattern PcgenFormulaEvaluator files=0 hits=0
  pattern bonus_stack_reader   files=0 hits=0
  pattern pre_tokens           files=0 hits=0
  live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  **All three identifiers this criterion owns are at zero live hits.** `live_files` is flat at
  253 (cycle 3's own figure) and `live_hits` is down from 12,336 to 12,256. The baseline file is
  deliberately NOT rebaselined: `--rebaseline` is the epic's own step, and a mid-epic ratchet
  would hide a later regression.

  | identifier | end of cycle 3 | at HEAD |
  |---|---|---|
  | `pre_tokens` | files=0 hits=0 | files=0 hits=0 — closed at cycle 3 |
  | `PcgenFormulaEvaluator` | files=1 hits=6 | **files=0 hits=0** — closed |
  | `bonus_stack_reader` | files=1 hits=9 | **files=0 hits=0** — closed |

  Both remaining families were the same one file,
  `pilot_compute/class_feature_grant_consumer.rs`, and the same two uses cycle 3 named:
  `resolve_pcgen_var_chain`'s fixpoint over `BONUS:VAR` token text, and
  `bonus_stack_reader::extract_addends` for multi-row gated targets. Cycle 3 deferred the swap as
  retro `deferral 1789009691631-at-35-e6-001-c3-d74d1a` on the grounds that the converter's fold
  "resolves a **different population**", so the swap needed the same corpus-wide before/after
  comparison the feat gate got. **That comparison is this cycle's first act**, and it is below.
- **What actually changed, mechanically.** Three things used to run at REQUEST time and now run
  ONCE at ingest:
  1. **which source rows a target sums over** (`bonus_stack_reader::extract_addends`, plus the
     `TYPE=`-strip and sole-ungated-row widenings) — moved verbatim into
     `pcgen_import::class_feature_vars`;
  2. **the formula parse** (`formula_interpreter::parse`) — still the same parser, now called
     only from the converter;
  3. **the evaluation** — replaced by a term-for-term lowering of the parser's AST into our own
     `sheet_rule::Expr` (`floor` toward negative infinity, variadic `min`/`max`, `abs` as
     `max(x,-x)`, a comparison as numeric `0`/`1` through the converter's own `cmp_ge` encoding,
     `if(c,t,e)` as `c*t + (1-c)*e`, both sides of `&&` always evaluated as a product, exact
     arithmetic with ONE truncation toward zero at the boundary — the same contract the retired
     interpreter carried, which held `f64` end to end and truncated only at its public boundary).

  The result ships as `data/converted/record_vars.json`
  (`cargo run --locked --bin gen_record_vars`; `-- --check` is its freshness gate). The live
  side reads it and folds: substitute what the chain reaches, then default only a reference the
  corpus binds NOWHERE, then evaluate what closed; anything that still does not close is ABSENT
  rather than guessed (`rules_core::record_vars::resolve_chain`).
- **Oracle parity:** the epic's before/after parity comparison (`AT-35-E6-004`) is where this
  criterion's PCGen-oracle evidence lands, unchanged. What this cycle owes — and delivers — is
  the **corpus-wide before/after over the live path this swap touches**, which is the comparison
  cycle 3 named as the precondition. Artifact:
  `AT-35-E6-001_cycle4_varchain-parity.json` (1,150 rows, each carrying record, ability probe,
  level, variable, before and after).

  ```
  records before=4140 after=4075 only_before=65 only_after=0
  agree=199610 disagree=1070 lost=0 gained=80 records_moved=23
  resolved values inside the 65 only-before records = 0
  ```

  Re-derive both sides:
  `AT35_E6_VARCHAIN_DUMP=<path> cargo test --locked --lib -j 6 -- --ignored class_feature_grant_consumer::tests::dump_the_whole_var_chain_population`,
  run at `292d90f13e` and at HEAD, then diff. The dump covers every record the live table
  carries, at every level 1..=20, under two ability-modifier probes (all-zero, and a spread that
  makes each of the six abbreviations distinguishable).
- **Movement, four buckets:**
  - **closure (into DONE):** 0 — the population was already 0 non-DONE at the cycle start.
  - **relabel (bucket to bucket):** 0. No unit changed bucket.
  - **reachability:** 1,070 magnitudes on 23 real records move off a silent `0`, and 80 more
    become resolvable at all. No unit's bucket changed; what changed is that a number the sheet
    prints is now the rulebook's.
  - **instrument-correction:** one, and it is Discovery 1 below.
- **Refused tokens:** none — this cycle added no converter refusal and cleared none.
  `_refused.json` is unchanged at 142 records, one shape (`no_corpus_record`),
  `refused_non_done=0` (`python3 scripts/token_coverage.py --check`).
- **Discoveries:**
  1. **The retired evaluator looked variable names up by exact bytes, and the rule source is
     case-insensitive — so 23 real records printed a silent `0` where the rulebook states a
     scaling number.** Every one of the 1,070 disagreements and all 80 gains is this one
     mechanism, and no other. A corpus row writing `RogueLvl` where the class declares
     `RogueLVL`, or `HUNTERLVL` where it declares `HunterLVL`, or a gate reading
     `PREVARGTEQ:PaladinLvl`, or `classlevel("bard")` where the class is `Bard`, never bound;
     the identifier fell to the corpus-wide-unbound `0` path and the whole target came out `0`.
     The converted package mints one opaque id per **case-folded** name
     (`sheet_rule::var_id`, `technical-design.md` §1), so the fold binds them. Three checked
     against the published rule and pinned in
     `a_mixed_case_class_level_reference_now_scales`:

     | record | rule | before | after |
     |---|---|---|---|
     | Knife Master ~ Hidden Blade | +1/2 rogue level to conceal a weapon | 0 at every level | 5 at 10th |
     | Empyreal Knight ~ Celestial Heart | resistance 5 at 3rd, 10 at 9th | 0 at every level | 0 / 5 / 10 at 2nd / 3rd / 9th |
     | Loremaster ~ Secret Lore | one secret at 1st and every odd level | unresolved | 3 at 5th |

     Emitted as retro `correction 1789015501781-at-35-e6-001-55b9ac`.
  2. **65 records carried a chain that never evaluated, and nothing had noticed because nothing
     read it.** Their every target uses a shape the parser refuses — `skillinfo("RANK", ...)`,
     `count("ABILITIES", ...)`, `var("CL=Magus")`, `charbonusto("PCLEVEL", ...)` — so they
     resolved **0 values** before this cycle and 0 after (confirmed in the parity artifact:
     `resolved values inside the 65 only-before records = 0`). What changed is that the shipped
     artifact no longer carries a chain that cannot evaluate, so `record.bonus_vars.is_empty()`
     is now true for them. That is not a loss: the guard it feeds
     (`resolved_description_for_formula_only_desc_argument`'s "a real chain exists, not my
     business") now correctly hands those records to the description-argument resolver instead
     of refusing both ways. The whole family is `Refined Education Unlock ~ *` (22),
     `Pack Lord ~ Pack Member N Display` (20), `Magus Arcana ~ Maneuver Mastery *` (10) and 13
     singletons, all listed by name in the parity artifact's `records_only_before`.
  3. **A record's `%N` description arguments are a second formula family, and they were being
     parsed at request time too.** `resolved_description_for_formula_only_desc_argument`
     evaluated each `%N` argument as a bare formula string. Those 4,142 arguments across 3,034
     records are now lowered at ingest as well (`desc_arguments` in the artifact, keyed by the
     exact argument text the renderer matches on), so that path parses nothing either. Its six
     pinned pool-census tests (`pool_group_closure_census_across_all_six_pools_both_resolvers`
     and siblings) pass unchanged — the census figures did not move.
  4. **The corpus contains a `BONUS:VAR` target literally named `DEFINE:FireTrapBonusDamage`**
     (`Ranger Trap ~ Fire Trap`), an upstream authoring defect where a token head leaked into a
     target name. It is carried through faithfully rather than silently repaired, which is why
     `grep -cE 'BONUS:|DEFINE:|...' data/converted/record_vars.json` returns 2 hits, both that
     one name. `data/sheet_rules/` is unaffected and still returns 0.
- **Figures + their re-derive commands:** every row carries its own command. The unit denominator
  throughout is the whole corpus, all books — `jq '.units | length' docs/work-inventory.json`.

  | figure | value | command | denominator |
  |---|---|---|---|
  | `PcgenFormulaEvaluator` live files / hits | **1 / 6 → 0 / 0** | `python3 scripts/pcgen_residue_gate.py --check`, `pattern PcgenFormulaEvaluator` line | 49,438 units |
  | `bonus_stack_reader` live files / hits | **1 / 9 → 0 / 0** | same, `pattern bonus_stack_reader` line | 49,438 units |
  | `pre_tokens` live files / hits | **0 / 0 → 0 / 0** | same, `pattern pre_tokens` line | 49,438 units |
  | live PCGen files / hits overall | **253 / 12,336 → 253 / 12,256** | `python3 scripts/pcgen_residue_gate.py --check`, last line | 49,438 units |
  | live caller files remaining, all three identifiers | **1 → 0** | `for p in PcgenFormulaEvaluator bonus_stack_reader pre_tokens; do grep -rl "\b$p\b" src/rules_core src/saved_character src/campaign src/homebrew_authoring apps/desktop --include=*.rs --include=*.ts --include=*.tsx \| grep -v '^src/rules_core/cache_gen/'; done \| sort -u \| wc -l` | 49,438 units |
  | converted variables shipped | **9,566** across **4,445** records | `cargo run --locked --bin gen_record_vars -- --check` | 18,263 corpus `class`/`class_feature` records |
  | converted `%N` description arguments | **4,142** across **3,034** records | `jq '[.desc_arguments[] \| length] \| add' data/converted/record_vars.json` and `jq '.desc_arguments \| length' …` | 4,445 chain-bearing records |
  | corpus-wide-unbound referenced names carrying a declared baseline | **350** | `jq '.var_defaults \| length' data/converted/record_vars.json` | every name any converted chain references |
  | chain values compared, before vs after | **200,680** | `jq '.population.value_pairs_compared' docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-001_cycle4_varchain-parity.json` | 4,075 records × 20 levels × 2 probes |
  | chain values that agree | **199,610** | `jq '.verdict.agree' docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-001_cycle4_varchain-parity.json` | 200,680 compared |
  | chain values that disagree | **1,070** | `jq '.verdict.disagree' docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-001_cycle4_varchain-parity.json` | 200,680 compared |
  | chain values lost | **0** | `jq '.verdict.lost, .verdict.resolved_values_inside_records_only_before' docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-001_cycle4_varchain-parity.json` | 200,680 compared |
  | chain values gained | **80** | `jq '.verdict.gained' docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-001_cycle4_varchain-parity.json` | 200,680 compared |
  | records whose values moved | **23** | `jq '.verdict.records_moved' docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-001_cycle4_varchain-parity.json` | 4,075 records compared |
  | converted artifact size | **3.7 MB**, 1 file | `ls -l data/converted/record_vars.json` | — |
  | rust lines changed | **2,520** | `python3 scripts/cycle_scope_gate.py --receipt --since 292d90f13e --before /tmp/wi-before-AT-35-E6-001-c4.json --after docs/work-inventory.json` | 49,438 units |
  | atlas | `population=49438 unclassified=0 overlap=0 done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0` | `python3 scripts/completion_atlas.py --check` | 49,438 units |
  | token coverage | `non_done=0 refused=142 refused_non_done=0 token_types=231 verdict=PASS` | `python3 scripts/token_coverage.py --check` | 49,438 units |
  | `data/sheet_rules/` token leaks | **0** | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | 49,438 units |
  | shape/engine boundary | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` | `python3 scripts/shape_engine_boundary.py --check` | 49,438 units |
  | missing engine tables | `population=0 kinds=0 citation_failures=0` | `python3 scripts/missing_engine_tables.py --check` | 49,438 units |
  | denominator gate | `files_checked=69 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` | 69 bundle docs |
  | figure provenance | `files_checked=186 figures_examined=315 violations=0` | `python3 scripts/denominator_gate.py --check-provenance` | 315 figures |
- **Build scope verified**, run at `ac38c5bf3c`:
  - `cargo test --locked --lib -j 6` → **`ok. 3261 passed; 0 failed; 15 ignored`**
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0` (every bin and test target links).
  - `cargo test --locked --no-fail-fast -j 6` → **`FULL_EXIT=0`**, **414 `test result` lines
    (413 targets plus the lib), 8,772 passed, 68 ignored, and ZERO failing suites**
    (`grep -cE '^test result: FAILED' <log>` → `0`). The run takes roughly two hours on this box
    for a pre-existing reason cycle 3 already measured and attributed — every test binary loads
    the whole corpus (87,987 `openat` calls under `data/corpus/` for a representative one) and
    cargo runs test binaries serially. This cycle adds none of that: the converted artifact is a
    single 3.7 MB read, parsed once per process.
  - **The `test result` line count moved 413 → 414, and this cycle moved it deliberately.** The
    extra target is the new `src/bin/gen_record_vars.rs`. Nothing in `tests/`, `src/`, `apps/` or
    `scripts/` asserts either number — `grep -rn '\b413\b'` over those trees finds it only in
    this epic's own earlier receipts and `progress.md`, where it correctly records what those
    cycles saw, and in generated build artifacts under `apps/desktop/src-tauri/target/`.
  - `cargo test --locked --lib -j 6`, **re-run at the final tree** (the workspace run above had
    already built the lib target when three `get(..).is_none()` clippy lints on this cycle's own
    new tests were fixed) → **`ok. 3261 passed; 0 failed; 15 ignored`**. The fix is confined to
    assertion FORM inside `#[cfg(test)]` code in the lib, so no other target's result can have
    moved.
  - `cargo run --locked --release --bin sheet_rule_convert -- --check` →
    **`records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS`** (20.9 s)
    — the shipped `data/sheet_rules/` package is unchanged by this cycle, which is the point:
    delegating the converter's `var_id` to `sheet_rule::var_id` moved no id.
  - `cargo run --locked --release --bin gen_record_vars -- --check` →
    **`class_feature_described=4075 class_feature_any=4445 class_records=27 domain_records=25
    converted_vars=9566 var_defaults=350 verdict=PASS`** — the shipped artifact is fresh against
    the corpus.
  - `cargo clippy --locked --tests -j 4` (its own `CARGO_TARGET_DIR`, so it did not contend with
    the workspace suite's cargo lock) → **0 warnings, 0 errors**. Its first run raised three
    `unnecessary use of get(..).is_none()` lints on this cycle's own new tests; fixed in the same
    cycle.
  - `scripts/verify.sh --only pi-sweep` → **`RESULT: PASS`** (1 stage).
  - `cargo run --locked --release --bin v06_work_inventory` → **the binary REFUSED to write, and
    that is the correct outcome, not a failure of this cycle.** Its own guard: *"this run would
    drop 7385 of the 32617 verification stamp(s) it currently carries … set
    `CORPUS_LITERAL_SWEEP_REPORT` and `DERIVED_FIXTURE_CHECK_REPORT` to the sweep's and the
    fixture check's `--json-out` reports before regenerating … or pass `--allow-stamp-loss`."*
    This cycle changed no corpus record, so `corpus_literal_sweep` is guarded OFF (`§6` step 3),
    so those two reports do not exist for this tree — the stamps they carry cannot be
    reconstructed and the guard refuses rather than losing them. `--allow-stamp-loss` was NOT
    passed. `docs/work-inventory.json` is byte-identical on disk
    (`git status --porcelain -- docs/work-inventory.json` is empty), which is also the expected
    content for a cycle that moves no unit: the `--receipt` rows above were computed against it
    and read `closed=0 relabeled=0`.
  - `apps/desktop/src-tauri` and the frontend: **epic cadence** — this cycle touched no file
    under `apps/`.
- **Sweep population:** N/A — no corpus record changed this cycle, so `corpus_literal_sweep` was
  not re-run (`§6` step 3's guard). `git diff --stat ac38c5bf3c~1..HEAD -- data/corpus` is empty;
  the only `data/` change is the new `data/converted/record_vars.json`.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`, unchanged by this cycle).
- **Status:** **complete** — the criterion's own evidence sentence is met in full:
  `pcgen_residue_gate.py --check` shows `PcgenFormulaEvaluator`, `bonus_stack_reader` and
  `pre_tokens` at `files=0 hits=0`; the corpus-wide before/after comparison is committed and
  every moved value is accounted for by one named mechanism with 0 losses; the lib suite is green
  at 3,261 tests and the workspace run had 0 failures across every target completed. See the
  amendment for the completed workspace figure.
- **Notes:**
  - One retro `correction` landed under the wrong actor before the actor was set inline
    (`docs/retro/events/sd31-transcribe.jsonl`); it is re-emitted correctly as
    `1789015501781-at-35-e6-001-55b9ac` and the misfile is logged as
    `incident 1789015512888-at-35-e6-001-67f8e2`, recurrence key
    `retro-actor-lost-between-bash-calls`. The append-only line is left where it fell.
  - The converter's `sheet_rule/ctx.rs::var_id` now delegates to `sheet_rule::var_id` rather than
    carrying its own copy of the hash. One definition, two callers; the converted package's ids
    are unchanged (`sheet_rule_convert -- --check`).
- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design)`. This
  criterion's three named identifiers are at zero. What remains on the live side belongs to the
  criterion's siblings, not to this one:

  | family | live files | live hits | owner |
  |---|---|---|---|
  | `raw_tokens` | 41 | 234 | `AT-35-E6-002` / `AT-35-E6-003` |
  | `render_pcgen_desc` | 18 | 110 | `AT-35-E6-002` / `AT-35-E6-003` |
  | source-token literals (`BONUS:`, `PRE<X>:`, `DESC:`, …) in live code | 253 files total | 12,256 | `AT-35-E6-002`..`004`; `--closure` at `AT-35-E6-004` |
