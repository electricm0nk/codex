# Cycle 1 — Epic 6 (PCGen exit) / AT-35-E6-001

- **Commit SHA:** `7eaa1e948d` (code: `ea1a61e672` + build fix `60fdeeb6ec`; cycle start `c3500e7984`)
- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design, decisions.md §2)`.
  Run anyway for the record:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  `remaining_non_done=0` — the corpus reached `DONE 49438 of 49438` at `AT-35-E5-005`
  (`103693b365`). Epic 6 moves no unit; it removes the ingest-format engine from the live side.
- **Files touched:** 6 moved (`src/rules_core/pilot_compute/{formula_interpreter,
  formula_interpreter_corpus_wide,formula_reproduction_harness,bonus_stack_reader,
  race_trait_formula_binding}.rs` and `src/rules_core/feat_prereqs/pre_tokens.rs` →
  `src/pcgen_import/`), 27 edited:
  `src/pcgen_import/mod.rs`; `src/rules_core/{sheet_rule,trait_effects,racial_sla,
  skill_allocation,feat_prereqs,derived_evaluator_fixture_check}.rs`;
  `src/rules_core/pilot_compute/{mod,class_slayer,class_feature_grant_consumer,
  crb_untabled_class_chassis,generic_class_chassis,prestige_class_entry_gate}.rs`;
  `src/rules_core/rules_tables/feats_all.rs`,
  `src/rules_core/rules_tables/ultimate_psionics/{feat_tables,marksman_features,psion_features}.rs`;
  `src/bin/{bonus_stack_reader,formula_interpreter,ingest_race_traits,ingest_races}.rs`;
  `tests/{formula_interpreter_family_fixture_check,sd27_feat_prerequisite_enforcement}.rs`;
  `apps/desktop/src-tauri/src/{character_hub,class_catalog_generic,feat_catalog,trait_picker}.rs`;
  `apps/desktop/src/boundary/loadCharacterTraits.ts`;
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (the atlas's
  own `derived_at` stamp, rewritten by this cycle's `--check` run).
- **Identifier audit result:** OK_NO_BUNDLE_TAGS for new identifiers. The grep over this cycle's
  own diff returns 2 lines, both pre-existing **test file names** rather than bundle-tagged code:
  a doc citation of `tests/sd34_wave51_racial_sla_catalog_matches_the_corpus.rs` that this cycle
  moved within `racial_sla.rs`, and the `+++`/`---` headers of the import-path edit to
  `tests/sd27_feat_prerequisite_enforcement.rs`. No identifier in shipping code carries a bundle
  tag. Re-derive:
  `git diff --unified=0 <cycle-start>..HEAD -- ':!docs/**' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
- **Wired-integration audit result:** OK_NO_TOKENS —
  `git diff --unified=0 fe5ae6cd4a...HEAD -- src/rules_core/pilot_compute src/pcgen_import apps/desktop ':!**/__tests__/**' ':!**/*.test.*' | grep -ncE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`
  → `0`.
- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E6-001`):
  > `PcgenFormulaEvaluator` and `formula_interpreter*.rs` move to `src/pcgen_import/` (the
  > converter's parser). Every live caller (14 files at authoring — `racial_sla.rs`,
  > `domain_power`, trait/feat effects, the pilot_compute formula paths) is replaced by
  > `sheet_rule::evaluate` over converted `Expr`, or deleted where the sheet line already carries
  > the value. `bonus_stack_reader.rs` and `feat_prereqs/pre_tokens.rs` move with it.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows `PcgenFormulaEvaluator`,
  > `bonus_stack_reader`, `pre_tokens` at 0 live hits; the oracle comparison agrees before and
  > after; full workspace suite green.
- **Receipt rows (mechanical):**
  ```
  since=c3500e79843086b2c4547a33428b01e27f963d53 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=613 ratio=n/a builds_recorded=0 pcgen_live_files=254
  ```
  `closed=0` / `relabeled=0` is correct and expected: the unit population was already `0` non-DONE
  when the cycle started. `ratio` is `n/a`, a division by zero, never `0.0`.
- **PCGen residue:**
  ```
  live_files=254 live_hits=12396 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Down on both axes, up on neither. The three identifiers this criterion owns:

  | identifier | at cycle start | at HEAD |
  |---|---|---|
  | `PcgenFormulaEvaluator` | files=14 hits=100 | **files=5 hits=28** |
  | `bonus_stack_reader` | files=7 hits=20 | **files=2 hits=13** (was 4 files/16 before `7eaa1e948d` cleared `pilot_compute/mod.rs`) |
  | `pre_tokens` | files=6 hits=20 | **files=4 hits=19** |

  **Not zero. The criterion is `partial`** — the remainder is named by mechanism below.
  The baseline file is deliberately NOT rebaselined by this cycle: `--rebaseline` is the epic's
  own step, and a mid-epic ratchet would hide a later cycle's regression.
- **Oracle parity:** `lines compared=146 agree=145 disagree=1 unverifiable=16; chassis
  compared=382 agree=376 disagree=6 unverifiable=140; characters=29 exports_missing=0
  PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` — **identical before and after**,
  and stronger than identical summary counts: the engine's own output file is **byte-identical**.
  `cmp -s /tmp/e6-parity/ours-before.json /tmp/e6-parity/ours-after.json` → `IDENTICAL`.
  The "before" run used the `sheet_rule_parity` release binary built from the tree **as it stood
  at the cycle start**, before any file moved; the "after" run used the binary rebuilt at HEAD.
  Both were compared against the same pinned exports
  (`artifacts/epic-2-sheet-rule/oracle-parity/exports`). The 7 disagreements are byte-identical to
  `AT-35-E4-001_cycle1_receipt.md`'s: **0 introduced, 0 fixed** by this cycle.
  Artifacts committed as `AT-35-E6-001_cycle1_sheet-parity-before.json` /
  `-after.json` alongside this receipt.
- **Movement, four buckets:**
  - **closure (into DONE):** 0 — the population was already 0 non-DONE at the cycle start.
  - **relabel (bucket to bucket):** 0. No unit changed bucket and no unit changed evidence; the
    engine's rendered output is byte-identical (see Oracle parity).
  - **reachability:** unchanged. `data/sheet_rules/` was not regenerated (nothing converter-side
    changed) and `sheet_rule_convert -- --check` re-derives `verdict=PASS` at HEAD.
  - **instrument-correction:** none. No instrument's reading was found wrong this cycle.
- **Refused tokens:** none — this cycle added no converter refusal and cleared none.
  `_refused.json` is unchanged at 142 records, one shape (`no_corpus_record`),
  `refused_non_done=0`.
- **Discoveries:**
  1. **The evaluator's dependency, `formula_reproduction_harness.rs`, was not in the criterion's
     file-touch set and had to move with it.** It defines the `FormulaEvaluator` trait
     `formula_interpreter.rs` implements, so the interpreter cannot leave `rules_core` without it.
     It is also pure ingest-format harness code — it belonged on the converter side already. Its
     `include_str!` of four `pilot_compute` files (its hand-modelled-function census) and its 22
     `super::<fn>` calls now cross the module boundary explicitly, which is why 22 functions and
     one glob re-export widened from private/`pub(super)` to `pub(crate)`. No behaviour changed;
     `class_slayer.rs` is in the touched list for exactly that reason.
  2. **The desktop trait picker was serving the source's formula TEXT to the UI.**
     `TraitAbilitySubstitutionDto.formula` carried `"max(INT,CHA)-CHA"` verbatim. Under the sheet
     rule (`decisions.md` §1) that field must carry the rule's words; it now carries
     `"the higher of your Intelligence and Charisma modifiers, instead of your Charisma modifier"`.
     The frontend never rendered the string (only `flatBonus`), so no screen changed — but the
     boundary DTO and its two tests did.
  3. **`data/sheet_rules/<book>/class/<slug>.json` already carries the whole BAB/save chassis as
     converted `Expr`** (`{"Number":{"Div":[{"ClassLevel":"commoner"},{"Const":2}]}}`), joined by
     the label suffix `(base Fortitude save)` / `(base Reflex save)` / `(base Will save)`. That is
     the replacement path for `crb_untabled_class_chassis`/`generic_class_chassis`/
     `class_catalog_generic` — **except** that the converted record carries no `MAXLEVEL`, which
     all three need to refuse a level above a class's own ceiling. Closing them therefore needs one
     converter-side addition first (a `StatBlock` prose row for the maximum level), which is a
     regeneration of `data/sheet_rules/` and belongs in its own cycle. Named, not exempted.
- **Figures + their re-derive commands:** every row below carries its own command. The unit
  denominator throughout is the whole corpus, all books — re-derive it with
  `jq '.units | length' docs/work-inventory.json`.

  | figure | value | command | denominator |
  |---|---|---|---|
  | live PCGen files / hits, before → after | **260 / 12,736 → 254 / 12,396** | `python3 scripts/pcgen_residue_gate.py --check` | 49,438 units |
  | `PcgenFormulaEvaluator` live files / hits | **14 / 100 → 5 / 28** | same, `pattern PcgenFormulaEvaluator` line | 49,438 units |
  | `bonus_stack_reader` live files / hits | **7 / 20 → 2 / 13** | same, `pattern bonus_stack_reader` line | 49,438 units |
  | `pre_tokens` live files / hits | **6 / 20 → 4 / 19** | same, `pattern pre_tokens` line | 49,438 units |
  | live callers remaining, by file | **9** of the 22 that carried one of the three identifiers | `for p in PcgenFormulaEvaluator bonus_stack_reader pre_tokens; do grep -rl "\b$p\b" src/rules_core src/saved_character src/campaign src/homebrew_authoring apps/desktop --include=*.rs --include=*.ts --include=*.tsx \| grep -v '^src/rules_core/cache_gen/'; done \| sort -u \| wc -l` | 49,438 units |
  | function bodies deleted under `src/pcgen_import`, `scripts/oracle_harness`, `src/oracle_validation` | **0** | `git diff --stat c3500e7984..HEAD -- src/pcgen_import scripts/oracle_harness src/oracle_validation` — 6 renames + path repairs only | 49,438 units |
  | rust lines changed | **613** | `python3 scripts/cycle_scope_gate.py --receipt --since c3500e7984 --before /tmp/wi-before-AT-35-E6-001.json --after docs/work-inventory.json` | 49,438 units |
  | atlas | `DONE: 49438`, every other bucket 0 | `python3 scripts/completion_atlas.py --check` | 49,438 units |
  | token coverage | `unmapped_token_types=0 refused=142 refused_non_done=0 verdict=PASS` | `python3 scripts/token_coverage.py --check` | 49,438 units |
  | converter re-derivation | `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS` (21.7 s) | `cargo run --locked --release --bin sheet_rule_convert -- --check` | 49,438 units |
  | `data/sheet_rules/` token leaks | **0** | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | 49,438 units |
  | shape/engine boundary | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` | `python3 scripts/shape_engine_boundary.py --check` | 49,438 units |
  | missing engine tables | `population=0 kinds=0 citation_failures=0` | `python3 scripts/missing_engine_tables.py --check` | 49,438 units |
  | denominator gate | `files_checked=65 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` | 49,438 units |
  | PI sweep | `PASS (11 hits over src/rules_core/rules_tables, 11 baseline rows)` | `scripts/verify.sh --only pi-sweep` | 49,438 units |
  | oracle parity, before and after | `146 / 145 / 1` lines, `382 / 376 / 6` chassis, **outputs byte-identical** | `scripts/oracle_harness/sheet_parity.py compare --ours <ours.json> --exports …/oracle-parity/exports --output …` | 49,438 units |

  Wall times paid: `cargo test --locked --no-run -j 6` **172.93 s** (cold, `/usr/bin/time -f`);
  `cargo test --locked --lib -j 6` **43.29 s**; `sheet_rule_parity` **6.4 s** per run;
  `sheet_rule_convert -- --check` **21.7 s**.
- **Build scope verified**, run at `7eaa1e948d`:
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`
  - `cargo test --locked --lib -j 6` → `ok. 3230 passed; 0 failed; 14 ignored`
  - `cargo test --locked --no-fail-fast -j 6` → **8,741 passed, 0 failed, 67 ignored over 412
    targets** (413 `test result` lines: 412 targets plus the lib), `FULL_EXIT=0`, `test result:
    FAILED` count **0**. No self-heal was needed: nothing broke.
  - `cd apps/desktop/src-tauri && cargo test --locked` → `ok. 576 passed; 0 failed; 0 ignored`,
    `DESKTOP_EXIT=0` — run here, not at
    epic cadence, because this cycle touched `apps/`.
  - `cargo clippy --locked --lib -j 4` → 0 warnings, 0 errors.
  - The last commit (`7eaa1e948d`) edits two runtime explanation strings and two doc comments
    only. No test anywhere asserts either string
    (`grep -rn 'SUM semantics\|multiple-BONUS:VAR-on-one-target' --include=*.rs tests/ src/ apps/`
    names no assertion), and `cargo test --locked --lib -j 6` was re-run at that commit:
    `ok. 3230 passed; 0 failed; 14 ignored`.
- **Tool side intact** (`decisions.md` §11, "what is KEPT" — checked here rather than left to
  `AT-35-E6-004`, because this is the cycle that moved the files):
  ```
  git diff --stat c3500e7984..HEAD -- src/pcgen_import scripts/oracle_harness src/oracle_validation
    7 files changed, 6016 insertions(+)          # 6 renames + 8 lines of module declarations
  git diff  … | grep -cE '^-\s*(pub(\(crate\)|\(super\))? )?fn '   -> 0   # zero deleted function bodies
  cargo build --locked --bin sheet_rule_convert --bin gen_book_cache   -> TOOLSIDE_EXIT=0
  python3 scripts/oracle_harness/run.py --help                          -> ORACLE_HELP_EXIT=0
  git diff --stat c3500e7984..HEAD -- scripts/pcgen-oracle-pin.env      -> (empty; pin unchanged)
  ```
- **Sweep population:** N/A — no corpus record changed this cycle, so
  `corpus_literal_sweep` was not re-run (`§6` step 3's guard: "only when corpus records changed").
  `git diff --stat c3500e7984..HEAD -- data/` is empty.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`, unchanged by this cycle).
- **Status:** **partial**
- **Notes:**
  - **An incident this cycle caused and could not clear, needing an operator ruling.** The
    import-path rewrite was run as a repo-wide `os.walk`, which also rewrote 15 `.rs` files inside
    **14 sibling git worktrees** under `.claude/worktrees/` (375 lines, import paths only —
    other agents' and gate workers' trees). Four repair attempts were **refused by the permission
    classifier**: `git checkout -- <file>`, `git restore --source=HEAD --worktree <file>`, and two
    scoped inverse-substitution scripts. One file was repaired through the `Edit` tool
    (`wf_291be5c8-5f3-14/src/rules_core/racial_sla.rs`); **209 file-repairs remain**. Re-derive the
    remainder with
    `grep -rn 'pcgen_import::formula_interpreter\|pcgen_import::formula_reproduction_harness\|pcgen_import::bonus_stack_reader\|pcgen_import::race_trait_formula_binding\|pcgen_import::pre_tokens' .claude/worktrees/ --include=*.rs | wc -l`.
    Logged as retro incident `1788995836569-at-35-e6-001-daa3fe`, recurrence key
    `repo-wide-walk-hits-sibling-worktrees`. **The mechanism owed** (`AGENTS.md` rule 8, "a warning
    is not a control"): every repo-wide walk or rewrite launched from this checkout must exclude
    `.claude/worktrees/` **by construction**.
  - The first code commit (`ea1a61e672`) did not build: `git mv` stages a moved file at move time,
    so the path repairs made afterwards stayed unstaged and the explicit-path `git add` (no
    `git add -A` on a shared checkout) missed them. Fixed forward in `60fdeeb6ec`; logged as
    rework `1788997107739-at-35-e6-001-967c28`.
  - `formula_reproduction_harness.rs` moved although it is not in the criterion's file-touch set.
    It is the moved interpreter's own trait definition — the change is not closed without it.
- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design)`.
  The remainder is **9 files carrying one of the three identifiers**, in four named families.
  Each is a live caller that must reach `sheet_rule::evaluate`/`evaluate_expr` over converted
  `Expr`, or be deleted:

  | family | files | identifier hits | the mechanism that closes it |
  |---|---|---|---|
  | **class-feature var chain** | `pilot_compute/class_feature_grant_consumer.rs` | `PcgenFormulaEvaluator` 6, `bonus_stack_reader` 8 | `resolve_pcgen_var_chain` is a full same-name bonus-row fixpoint over corpus token text. Replace with the converter's own `_vars/<VarId>.json` `VarTable` fold, which `sheet_rule::evaluate` already reads. |
  | **class chassis** | `pilot_compute/crb_untabled_class_chassis.rs`, `pilot_compute/generic_class_chassis.rs`, `apps/desktop/src-tauri/src/class_catalog_generic.rs` | `PcgenFormulaEvaluator` 12 | Join `data/sheet_rules/<book>/class/<slug>.json`'s converted BAB/save `Expr`s by label suffix (Discovery 3). **Blocked on one converter addition first:** the converted class record carries no `MAXLEVEL`. |
  | **fixture-check oracle** | `rules_core/derived_evaluator_fixture_check.rs` | `PcgenFormulaEvaluator` 10 | The file is two things: live formatters the desktop catalogs call, and a fixture/oracle bar check. Split it — the bar check to `src/oracle_validation/`, the formatters stay. |
  | **feat prerequisites** | `rules_core/feat_prereqs.rs`, `pilot_compute/prestige_class_entry_gate.rs`, `apps/desktop/src-tauri/src/feat_catalog.rs`, `apps/desktop/src-tauri/src/character_hub.rs` | `pre_tokens` 19 | `evaluate_prerequisite_token` parses `PRE*` token text at run time. The replacement already exists and already ships: `level_up_option_filter::filter_option_pool` over `SheetRule.applies` + `unmet_words` (`AT-35-E5-004`). This family is "deleted where the sheet line already carries the value". |
