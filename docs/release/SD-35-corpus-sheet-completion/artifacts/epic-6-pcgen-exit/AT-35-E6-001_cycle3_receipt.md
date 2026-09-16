# Cycle 3 — Epic 6 (PCGen exit) / AT-35-E6-001

- **Commit SHA:** the cycle is the range `da5e9f8d3c..HEAD` (cycle start `da5e9f8d3c`).
  `3eefd5cedc` is the code change ("feat(sd35): feat and prestige prerequisites read the
  converted gate, not the source tokens"); `5a7d476542` folds the Epic 5 wrap-up report and
  re-derives the two stale ledgers; `4f8e68b09b` adds `scripts/doubled_gate_census.py`; a final
  commit carries the self-heal of `tests/sd27_feat_prerequisite_enforcement.rs`, this receipt and
  the `progress.md` / `kanban.md` rows (a receipt cannot name the commit that carries it).
- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design, decisions.md §2)`.
  Run anyway for the record:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  `remaining_non_done=0` — the corpus reached `DONE 49438 of 49438` at `AT-35-E5-005`. Epic 6
  moves no unit; it removes the ingest-format engine from the live side.
- **Files touched:** 1 new (`src/rules_core/feat_prereqs/converted_gate.rs`), 7 edited
  (`src/rules_core/feat_prereqs.rs`; `src/rules_core/pilot_compute/prestige_class_entry_gate.rs`;
  `src/rules_core/corpus_loader.rs`; `src/rules_core/sheet_rule.rs`;
  `apps/desktop/src-tauri/src/{feat_catalog,character_hub}.rs`;
  `tests/sd27_feat_prerequisite_enforcement.rs`), plus 1,926 regenerated `data/sheet_rules/**`
  records and `docs/retro/events/at-35-e6-001-c3.jsonl`. The docs commit adds this receipt and
  refreshes `artifacts/epic-2-sheet-rule/token-coverage.json` (MAXLEVEL's mapping row, cycle 2's
  correction, never folded into the ledger) and
  `../SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (`derived_at` stamp).
- **Identifier audit result:** OK_NO_BUNDLE_TAGS. Re-derive over this cycle's own diff:
  ```
  SCOPED="src/rules_core/feat_prereqs.rs src/rules_core/feat_prereqs \
          src/rules_core/pilot_compute/prestige_class_entry_gate.rs \
          src/rules_core/corpus_loader.rs src/rules_core/sheet_rule.rs \
          apps/desktop/src-tauri/src/feat_catalog.rs apps/desktop/src-tauri/src/character_hub.rs"
  git diff --unified=0 da5e9f8d3c..3eefd5cedc -- $SCOPED ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -cE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'   ->  0
  ```
  The branch-wide grep (`fe5ae6cd4a...HEAD`) returns 1 line, and it is a **deletion**: this cycle
  removed the doc line citing the test file name `tests/sd27_feat_prerequisite_enforcement.rs`.
  No identifier in shipping code carries a bundle tag.
- **Wired-integration audit result:** OK_NO_TOKENS.
  ```
  git diff --unified=0 da5e9f8d3c..3eefd5cedc -- $SCOPED ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -cE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'   ->  0
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
  since=da5e9f8d3c55694141e609bbe76e1c03a74c5ad0 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=1168 ratio=n/a builds_recorded=1 pcgen_live_files=253
  ```
  `closed=0` / `relabeled=0` is correct and expected: the unit population was already `0`
  non-DONE when the cycle started. `ratio` is `n/a`, a division by zero, never `0.0`.
- **PCGen residue:**
  ```
  live_files=253 live_hits=12336 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Down on hits from cycle 2's `253 / 12,354`, up on neither axis. The three identifiers this
  criterion owns, at cycle 2's close and at HEAD:

  | identifier | end of cycle 2 | at HEAD | remaining files |
  |---|---|---|---|
  | `pre_tokens` | files=4 hits=19 | **files=0 hits=0** | — **closed** |
  | `PcgenFormulaEvaluator` | files=1 hits=6 | files=1 hits=6 | `pilot_compute/class_feature_grant_consumer.rs` |
  | `bonus_stack_reader` | files=1 hits=9 | files=1 hits=9 | `pilot_compute/class_feature_grant_consumer.rs` |

  **Not zero. The criterion stays `partial`** — one of cycle 2's two named families closed, one
  remains, named by mechanism below. The baseline file is deliberately NOT rebaselined:
  `--rebaseline` is the epic's own step, and a mid-epic ratchet would hide a later regression.
- **Oracle parity:** N/A for this cycle — it added no `Number` mapping and touched no live
  arithmetic path. `Applies::all`'s dedupe changes a gate's *term list*, never a gate's verdict
  (`A and A` is `A`) and never a rendered magnitude; the sheet-parity roster compares rendered
  lines and chassis rows, both unchanged. The epic's before/after parity comparison
  (`AT-35-E6-004`) is where the criterion's parity evidence lands. Cycle 2's committed
  `AT-35-E6-001_cycle2_sheet-parity-after.json` remains the standing "after".
- **Movement, four buckets:**
  - **closure (into DONE):** 0 — the population was already 0 non-DONE at the cycle start.
  - **relabel (bucket to bucket):** 0. No unit changed bucket.
  - **reachability:** the feat catalog's per-character verdicts move — a starting Fighter's
    eligible count goes **755 → 549 of 2,227** (see Discoveries 2) and 21 catalog records are
    named as carrying no converted rule. No unit's bucket changed; what changed is which live
    module reads what, and how honestly it answers.
  - **instrument-correction:** two. `token-coverage.json`'s `MAXLEVEL` row is refreshed to the
    `Applies(class level ceiling)` mapping cycle 2 corrected in the JSON table but never
    re-derived into the ledger; the completion atlas's `derived_at` stamp advances.
- **Refused tokens:** none — this cycle added no converter refusal and cleared none.
  `_refused.json` is unchanged at 142 records, one shape (`no_corpus_record`),
  `refused_non_done=0` (`python3 scripts/token_coverage.py --check`).
- **Discoveries:**
  1. **The converter stated 963 of 1,830 gated feat records' prerequisites exactly twice, and
     nothing had noticed because it never changed a verdict.** `sheet_rule_convert` conjoins a
     record's own gate onto every line it emits (`Applies::all(vec![record_applies, line.applies])`
     in `src/pcgen_import/sheet_rule/convert.rs`); for a record whose only line-level gate IS the
     record gate, the conjunction stated each requirement twice. `A and A` is `A`, so every
     evaluator agreed — but every consumer that *reports* a gate printed each requirement twice,
     and a consumer that counts terms counted six where the record has three. Fixed at the source:
     `Applies::all` no longer adds a term already in the conjunction. `data/sheet_rules/`
     regenerated, 1,926 files changed. Emitted as retro
     `correction 1789009672393-at-35-e6-001-c3-b551d0`.
  2. **The token evaluator was passing prerequisites it could not read, and the converted gate
     denies them.** A starting Fighter (Human, Str 14 / Dex 13 / Con 12 / Int 10, no feats, no
     ranks) qualified for 755 of 2,227 catalog feats and now qualifies for 549. Two families
     account for nearly all of it, each spot-checked against the published rules: ability-score
     and rank thresholds the token evaluator passed (**Combat Expertise** requires Int 13 and this
     build has Int 10; **Desert Dweller** requires Con 13 and 1 Survival rank and this build has
     12 and 0), and holdings named as one concrete record (**Extra Rage Power**, **Extra
     Discovery**, **Extra Grit**, the Ultimate Psionics families) that the held set's grant
     fixpoint could have granted and did not. Emitted as retro
     `correction 1789009680157-at-35-e6-001-c3-e44296`.
  3. **A `PRELEVEL:MAX` ceiling is now enforced, and `Fey Foundling` is the second record it
     catches.** `Wilding` was the whole known set; `Fey Foundling` carries the same "you must take
     this at 1st level" ceiling and was offered to a 6th-level character until this cycle, because
     maximum-level ceilings were on the token evaluator's unmodelled list. Named in
     `a_stronger_build_is_eligible_for_a_superset_of_a_weaker_ones_feats`'s own exception set
     rather than silently excluded, so a third one fails the test instead of being absorbed.
  4. **The held set's class-feature roster is real but partial, and the boundary is a measurement,
     not a guess.** The package's grant edges reach a Barbarian's Rage, a Bard's Bardic Performance
     and a Paladin's Lay on Hands — but not a Cleric's Channel Positive Energy, whose grant is
     conditioned on an alignment the character record has no field for. So a `Holds` counting a
     **pool** by name ("1 Channel Energy from special ability") is reported, never refused, while a
     `Holds` naming **one concrete record** is decided. The first draft of this cycle refused both
     and would have denied a real Cleric the Extra Channel feat on a path that refuses a *save*,
     not merely a picker row. The boundary is pinned by
     `a_class_feature_pool_holding_is_reported_not_refused` over those four classes.
- **Figures + their re-derive commands:** every row below carries its own command. The unit
  denominator throughout is the whole corpus, all books — re-derive it with
  `jq '.units | length' docs/work-inventory.json`.

  | figure | value | command | denominator |
  |---|---|---|---|
  | live PCGen files / hits, cycle 2 → HEAD | **253 / 12,354 → 253 / 12,336** | `python3 scripts/pcgen_residue_gate.py --check` | 49,438 units |
  | `pre_tokens` live files / hits | **4 / 19 → 0 / 0** | same, `pattern pre_tokens` line | 49,438 units |
  | `PcgenFormulaEvaluator` live files / hits | **1 / 6 → 1 / 6** (untouched this cycle) | same, `pattern PcgenFormulaEvaluator` line | 49,438 units |
  | `bonus_stack_reader` live files / hits | **1 / 9 → 1 / 9** (untouched this cycle) | same, `pattern bonus_stack_reader` line | 49,438 units |
  | live caller files remaining, all three identifiers | **5 → 1** | `for p in PcgenFormulaEvaluator bonus_stack_reader pre_tokens; do grep -rl "\b$p\b" src/rules_core src/saved_character src/campaign src/homebrew_authoring apps/desktop --include=*.rs --include=*.ts --include=*.tsx \| grep -v '^src/rules_core/cache_gen/'; done \| sort -u \| wc -l` | 49,438 units |
  | gated feat records whose converted gate stated every requirement twice | **963 of 1,830** | `python3 scripts/doubled_gate_census.py da5e9f8d3c` → `gated=1830 doubled=963` (reads the PRE-fix tree out of git, never the working tree — never `git stash` on a shared checkout) | 1,830 gated feat records at `da5e9f8d3c` |
  | the same census at HEAD | **0 of 1,514** | `python3 scripts/doubled_gate_census.py` → `gated=1514 doubled=0`. The denominator moves because a gate that was two identical terms is now ONE term, and `Applies::all` returns a single term as itself rather than as an `All` — 316 records left the "gated `All`" denominator without losing a requirement | 1,514 gated feat records at HEAD |
  | converted records regenerated | **1,926** | `git show --stat 3eefd5cedc -- data/sheet_rules \| tail -1` | 49,438 units |
  | a starting Fighter's eligible catalog feats | **755 → 549** | `cargo test --locked --lib -j 6 rules_core::feat_prereqs::prerequisite_tests::a_starting_fighter_keeps_a_real_catalog_and_every_denial_states_why` | 2,227 catalog records |
  | catalog records with no converted rule | **21** | `cargo test --locked --lib -j 6 rules_core::feat_prereqs::prerequisite_tests::a_starting_fighter_keeps_a_real_catalog_and_every_denial_states_why` (its `unconverted` assertion) | 2,227 catalog records |
  | prestige registry rows resolving to a converted class record | **74 of 74** | `cargo test --locked --lib -j 6 prestige_class_entry_gate_tests::registry_loads_and_matches_the_re_derive_command` | 74 census'd prestige classes |
  | rust lines changed | **1,168** | `python3 scripts/cycle_scope_gate.py --receipt --since da5e9f8d3c --before /tmp/wi-before-AT-35-E6-001-c3.json --after docs/work-inventory.json` | 49,438 units |
  | atlas | `population=49438 unclassified=0 overlap=0 done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0` | `python3 scripts/completion_atlas.py --check` | 49,438 units |
  | token coverage | `non_done=0 refused=142 refused_non_done=0 token_types=231 verdict=PASS` | `python3 scripts/token_coverage.py --check` | 49,438 units |
  | converter re-derivation | `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS` (20.3 s) | `cargo run --locked --release --bin sheet_rule_convert -- --check` | 49,438 units |
  | `data/sheet_rules/` token leaks | **0** | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | 49,438 units |
  | shape/engine boundary | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` | `python3 scripts/shape_engine_boundary.py --check` | 49,438 units |
  | missing engine tables | `population=0 kinds=0 citation_failures=0` | `python3 scripts/missing_engine_tables.py --check` | 49,438 units |
  | denominator gate | `files_checked=69 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` | 68 bundle docs |
  | figure provenance | `files_checked=186 figures_examined=315 violations=0` | `python3 scripts/denominator_gate.py --check-provenance` | 297 figures |
  | PI sweep | `RESULT: PASS` (1 stage, `pi-sweep`; 11 hits, 11 baseline rows) | `scripts/verify.sh --only pi-sweep` | 49,438 units |
- **Build scope verified**, run at `3eefd5cedc`:
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`
  - `cargo test --locked --lib -j 6` → **`ok. 3244 passed; 0 failed; 14 ignored`**
  - `cargo test --locked --no-fail-fast -j 6` → **413 `test result` lines (412 targets plus the
    lib), 8,752 passed, 67 ignored, and exactly ONE failing suite**:
    `tests/sd27_feat_prerequisite_enforcement.rs`, 3 of its 12 tests, every one an assertion this
    cycle's own change moved and none a defect in shipping code — two pinned the token
    evaluator's wording (`"base attack bonus +6"`, `"rank(s) in"`) and one required every
    "not verified" note to contain the literal string `PRE`, which is precisely what stopped
    being read. Self-healed in the same cycle and **strengthened, not relaxed**: the denial must
    now also carry the character's own value, and a note must name the requirement in the rule's
    own words rather than merely mention a token. Re-run of that binary at HEAD:
    `cargo test --locked --test sd27_feat_prerequisite_enforcement -j 6` →
    **`ok. 9 passed; 0 failed; 3 ignored`** (the 3 are the standing `PCGEN_CORPUS_ROOT`-gated
    oracle-corpus checks). The fix is confined to that one file under `tests/`, so no other
    target's result can have moved; the 412 other suites' green above stands.
  - The strongest single row in that suite is one that needed no edit and passed unchanged:
    **`the_verdicts_match_the_published_core_rulebook_for_a_starting_fighter`** — 25 well-known
    CRB feats, each with its published eligibility and reason, and the converted gate agrees with
    the rulebook on every one.
  - `cd apps/desktop/src-tauri && cargo test --locked -j 4` → **`ok. 576 passed; 0 failed;
    0 ignored`** — run here, not at epic cadence, because this cycle touched `apps/`. Its first
    run failed one assertion this cycle's own change moved
    (`character_hub::tests::a_fighter_1_is_refused_improved_two_weapon_fighting_with_the_reasons`,
    which pinned the token evaluator's wording); the assertions are updated to the converted
    gate's words **and strengthened** — the line must now also carry the character's own value —
    in the same commit, and the re-run is the green above.
  - `cd apps/desktop && npm run typecheck` → `TYPECHECK_EXIT=0`; `npm test` → **`101/101 test
    files passed`** — run here for the same reason, although this cycle changed no frontend
    source: the Tauri DTOs it touched (`FeatEligibilityDto`) kept their wire shape, and the
    typecheck is what proves that rather than a claim that it did.
  - `cargo clippy --locked --tests -j 6` (own `CARGO_TARGET_DIR`, so it did not contend with the
    workspace suite's cargo lock) → **0 warnings, 0 errors**.
  - `cargo run --locked --bin v06_work_inventory` → see the "Inventory" line below.
- **Suite wall time, attributed rather than assumed.** The workspace run is slow on this box and
  the first suspicion was this cycle: `feat_prereqs` and the prestige gate now read a 52,000-file
  package, and a `OnceLock` is per PROCESS, so a per-binary load would cost the suite hours.
  Measured instead of assumed: `strace -f -e trace=openat` on `sd13_bard_spell_save_dcs` (12.8 s,
  a representative mid-suite binary) counts **87,987 opens under `data/corpus/` and 0 under
  `data/sheet_rules/`**. The gate never loads: `evaluate_prestige_class_entry` consults its
  in-memory census registry first and returns before touching the package for any class id that
  is not one of the 74. The suite's cost is the pre-existing corpus load, and this cycle adds
  none of it. Re-derive:
  `strace -f -e trace=openat -o /tmp/st.txt <binary>; grep -c data/sheet_rules /tmp/st.txt`.
- **Sweep population:** N/A — no corpus record changed this cycle, so `corpus_literal_sweep` was
  not re-run (`§6` step 3's guard: "only when corpus records changed").
  `git diff --stat da5e9f8d3c..3eefd5cedc -- data/corpus` is empty; the only `data/` change is the
  regenerated `data/sheet_rules/`.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`, unchanged by this cycle).
- **Status:** **partial**
- **Notes:**
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/EPIC-5_wrapup_gate_report.md`
    was untracked in this shared checkout at the cycle start and belongs to the Epic 5 wrap-up
    worker. Committed by this cycle rather than left as litter (a clean tree is the unfiltered
    `git status`), with no edit to its content.
- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design)`.
  One of cycle 2's two families remains, 1 file, 15 identifier hits:

  | family | files | identifier hits | the mechanism that closes it |
  |---|---|---|---|
  | **class-feature var chain** | `pilot_compute/class_feature_grant_consumer.rs` | `PcgenFormulaEvaluator` 6, `bonus_stack_reader` 9 | Two real uses: `resolve_pcgen_var_chain`'s fixpoint over `BONUS:VAR` token text, and `bonus_stack_reader::extract_addends` for multi-row gated targets. Both feed live class-feature magnitudes through `pilot_compute/mod.rs` (`resolve_class_feature_bonus_var`, `pool_member_terminal_targets_and_resolved_vars`). The replacement is the converter's own `_vars/<VarId>.json` `VarTable` fold that `sheet_rule::evaluate` already reads — but it resolves a **different population**, so the swap needs the same corpus-wide before/after comparison this cycle ran for the feat gate, not a same-cycle drop-in. Deferred deliberately, with that reason, as retro `deferral 1789009691631-at-35-e6-001-c3-d74d1a`. |
