# Cycle 2 — Epic 6 (PCGen exit) / AT-35-E6-001

- **Commit SHA:** the cycle is the range `c729c0f659..HEAD` (cycle start `c729c0f659`).
  Three commits: `5a2ff6f08c` fixture-check oracle family, `74abb60839` class-chassis family,
  `e190b343a4` mapping-row correction + two self-heals + this receipt, plus this one-line
  correction to the SHA above (a receipt cannot name the commit that carries it).
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
- **Files touched:** 3 new
  (`src/oracle_validation/race_trait_formula_bar_check.rs`,
  `src/rules_core/pilot_compute/class_chassis_sheet_rules.rs`, this receipt), 11 edited
  (`scripts/transcribe_monster_tables.py`; `src/oracle_validation/mod.rs`;
  `src/rules_core/derived_evaluator_fixture_check.rs`;
  `src/rules_core/rules_tables/book_of_the_damned_volume_2/monster_data.rs`;
  `src/pcgen_import/sheet_rule/{convert,table}.rs`;
  `src/rules_core/pilot_compute/{mod,crb_untabled_class_chassis,generic_class_chassis}.rs`;
  `apps/desktop/src-tauri/src/{class_catalog_generic,character_hub}.rs`;
  `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/token-mapping/mapping-table.v1.json`),
  plus 142 regenerated `data/sheet_rules/**/class/*.json` records (one line each — the added
  `applies` ceiling) and `docs/retro/events/at-35-e6-001.jsonl`.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS. Re-derive over this cycle's own diff:
  ```
  SCOPED="src/rules_core/pilot_compute src/rules_core/derived_evaluator_fixture_check.rs \
          src/rules_core/feat_prereqs.rs src/pcgen_import src/oracle_validation apps/desktop \
          scripts/transcribe_monster_tables.py"
  git diff --unified=0 c729c0f659..HEAD -- $SCOPED ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -cE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'   ->  0
  ```
  The branch-wide grep (`fe5ae6cd4a...HEAD`) still returns the 1 pre-existing line cycle 1's
  receipt already recorded: a doc citation of the test file name
  `tests/sd27_feat_prerequisite_enforcement.rs`. No identifier in shipping code carries a
  bundle tag.
- **Wired-integration audit result:** OK_NO_TOKENS.
  ```
  git diff --unified=0 c729c0f659..HEAD -- $SCOPED ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -cE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'   ->  0
  ```
  The same grep over the whole branch diff (`fe5ae6cd4a...HEAD`) is also `0`.
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
  since=c729c0f659fd02d0a98e7922f8df6e6fb27b5b36 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=2127 ratio=n/a builds_recorded=1 pcgen_live_files=254
  ```
  `closed=0` / `relabeled=0` is correct and expected: the unit population was already `0`
  non-DONE when the cycle started. `ratio` is `n/a`, a division by zero, never `0.0`.
- **PCGen residue:**
  ```
  live_files=253 live_hits=12354 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Down on both axes from cycle 1's `254 / 12396`, up on neither. The three identifiers this
  criterion owns, at cycle 1's close and at HEAD:

  | identifier | end of cycle 1 | at HEAD | remaining files |
  |---|---|---|---|
  | `PcgenFormulaEvaluator` | files=5 hits=28 | **files=1 hits=6** | `pilot_compute/class_feature_grant_consumer.rs` |
  | `bonus_stack_reader` | files=2 hits=13 | **files=1 hits=9** | `pilot_compute/class_feature_grant_consumer.rs` |
  | `pre_tokens` | files=4 hits=19 | **files=4 hits=19** | `feat_prereqs.rs`, `pilot_compute/prestige_class_entry_gate.rs`, desktop `feat_catalog.rs`, desktop `character_hub.rs` |

  **Not zero. The criterion stays `partial`** — two of cycle 1's four named families closed,
  two remain, named by mechanism below. The baseline file is deliberately NOT rebaselined:
  `--rebaseline` is the epic's own step, and a mid-epic ratchet would hide a later cycle's
  regression.
- **Oracle parity:** `lines compared=146 agree=145 disagree=1 unverifiable=16; chassis
  compared=382 agree=376 disagree=6 unverifiable=140; characters=29 exports_missing=0
  PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` — **byte-identical to cycle 1's
  after-run**, and stronger than matching summary counts: the seven disagreements are the same
  seven `(character, family, unit, ours, oracle)` tuples, **0 introduced and 0 fixed**.
  Re-derive:
  ```
  cargo run --locked --release --bin sheet_rule_parity -- \
    --roster docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/oracle-parity/roster \
    --output <ours.json>
  python3 scripts/oracle_harness/sheet_parity.py compare --ours <ours.json> \
    --exports docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/oracle-parity/exports \
    --output <after.json>
  ```
  Committed alongside this receipt as `AT-35-E6-001_cycle2_sheet-parity-after.json`; the
  "before" is cycle 1's own committed `AT-35-E6-001_cycle1_sheet-parity-after.json`, which is
  the tree this cycle started from.
- **Movement, four buckets:**
  - **closure (into DONE):** 0 — the population was already 0 non-DONE at the cycle start.
  - **relabel (bucket to bucket):** 0. No unit changed bucket.
  - **reachability:** two movements inside the class-chassis population, both named and
    re-derivable (see Discoveries 2 and 3): the conventional-class chassis count moves 61 → 62,
    and 142 converted class records gain a level-ceiling gate they did not carry. No unit's
    bucket changed; what changed is which live module reads what.
  - **instrument-correction:** one — `mapping-table.v1.json`'s `MAXLEVEL` row, corrected from
    `Metadata(ignored-by-sheet)` to `Applies(class level ceiling)`. Emitted as retro
    `correction 1789003329266-at-35-e6-001-4e75d3`.
- **Refused tokens:** none — this cycle added no converter refusal and cleared none.
  `_refused.json` is unchanged at 142 records, one shape (`no_corpus_record`),
  `refused_non_done=0` (`python3 scripts/token_coverage.py --check`).
- **Discoveries:**
  1. **The one arithmetic SLA caster level in the whole corpus is a constant, so it belongs at
     ingest.** `spell_like_ability_caster_level` ran a formula through the interpreter at render
     time for exactly one row — `book_of_the_damned_volume_2`'s Demon (Vermlek), three quarters
     of its own 4 racial Hit Dice. A monster's racial Hit Dice are a fixed property of its row,
     so the value is `3` and always was. `transcribe_monster_tables.py` resolves it now
     (`resolve_sla_cl_arithmetic`, its own small converter-side evaluator over the two names a
     monster row can carry) and the transcribed table states the number. Re-derive the
     population this touched: `grep -rn 'sla_cl_token: Some("' src/rules_core/rules_tables/ |
     grep -vE 'Some\("[0-9]+"\)|Some\("HD"\)|max\(TL,1\)'` → 0 rows.
  2. **`MAXLEVEL` was mapped `Metadata` and that is what blocked cycle 1's class-chassis
     family.** The mapping table files it under `Family::Prereq` — and a level ceiling IS a
     gate, not metadata. Leaving it unconverted is precisely what forced three live modules to
     keep re-reading the raw token. It now converts to
     `Compare { ClassLevel(<slug>), Lte, Const(<n>) }` folded into the record's `applies`. 142
     of the 184 rows carrying the token gain a gate; the 38 `NOLIMIT` rows (which state there is
     no ceiling) and 4 unreadable values gain none rather than a false one. Emitted as a
     `correction`; the mapping JSON is updated in the same commit so
     `table_is_a_transcription_of_the_json` stays the enforcement.
  3. **The conventional-class population is 62, not 61**, and neither half of that is a relabel:
     - **+2** — `adventurers_guide`'s Pathfinder Delver and Pathfinder Savant.
       `data/corpus/adventurers_guide/class/` holds 9 records and neither of these; the
       converter reads the pinned oracle corpus directly, so `data/sheet_rules/` carries both,
       each with a complete chassis and a 10-level ceiling.
       Re-derive: `ls data/corpus/adventurers_guide/class/ | wc -l` (9) against
       `ls data/sheet_rules/adventurers_guide/class/ | wc -l` (25).
     - **−1** — `inner_sea_gods`'s Evangelist. Its converted record carries a degradation on
       another of its own tokens, and the converter's standing policy drops every magnitude on a
       degraded record to the rule's own WORDS rather than folding a partly-read number into a
       sheet total. Under `decisions.md` §1 that record is done as prose; it is not a chassis,
       and the live reader refuses it rather than inventing one. Re-derive: the four
       `inner_sea_gods:class:evangelist` rules carry `"value":"Text"` and `"target":null`.
     Emitted as retro `correction 1789001691183` (logged under `sd31-transcribe` — `RETRO_ACTOR`
     was not exported in that one shell; the two later events carry the right actor).
  4. **A confidently-wrong number caught before it shipped.** The first draft of
     `class_chassis_sheet_rules` bound `CharacterFacts.class_levels` to the converted record's
     FILE slug. A record whose class name is redacted spells a codex-neutral id inside its own
     `Expr::ClassLevel`, so the file slug bound nothing, every level evaluated to `0`, and
     `row_at` still returned `Some(row)` — a plausible progression, not a refusal. The binding
     is read off the expressions themselves now, and the guard is corpus-wide, not on the one
     record that found it: `no_class_resolves_a_degenerate_all_zero_progression` asserts every
     one of the 62 has a non-zero base attack bonus at its own ceiling. Emitted as retro
     `correction 1789002108603-at-35-e6-001-2b8774`.
- **Figures + their re-derive commands:** every row below carries its own command. The unit
  denominator throughout is the whole corpus, all books — re-derive it with
  `jq '.units | length' docs/work-inventory.json`.

  | figure | value | command | denominator |
  |---|---|---|---|
  | live PCGen files / hits, cycle 1 → HEAD | **254 / 12,396 → 253 / 12,354** | `python3 scripts/pcgen_residue_gate.py --check` | 49,438 units |
  | `PcgenFormulaEvaluator` live files / hits | **5 / 28 → 1 / 6** | same, `pattern PcgenFormulaEvaluator` line | 49,438 units |
  | `bonus_stack_reader` live files / hits | **2 / 13 → 1 / 9** | same, `pattern bonus_stack_reader` line | 49,438 units |
  | `pre_tokens` live files / hits | **4 / 19 → 4 / 19** (untouched this cycle) | same, `pattern pre_tokens` line | 49,438 units |
  | live caller files remaining, all three identifiers | **9 → 5** | `for p in PcgenFormulaEvaluator bonus_stack_reader pre_tokens; do grep -rl "\b$p\b" src/rules_core src/saved_character src/campaign src/homebrew_authoring apps/desktop --include=*.rs --include=*.ts --include=*.tsx \| grep -v '^src/rules_core/cache_gen/'; done \| sort -u \| wc -l` | 49,438 units |
  | converted class records gaining a ceiling gate | **142** of 184 rows carrying the token | `git show --stat HEAD~1 -- data/sheet_rules \| tail -1` | 184 rows |
  | conventional-class chassis population | **61 → 62** | `cargo test --locked --lib -j 6 generic_class_chassis::tests::all_sixty_two_conventional_classes_resolve` | 96 converted class records over the 14 books |
  | rows in `src/rules_core/rules_tables/` still stating an arithmetic SLA caster level | **1 → 0** | `grep -rn 'sla_cl_token: Some("' src/rules_core/rules_tables/ \| grep -vE 'Some\("[0-9]+"\)\|Some\("HD"\)\|max\(TL,1\)'` | 220 rows stating one |
  | rust lines changed | **2,127** | `python3 scripts/cycle_scope_gate.py --receipt --since c729c0f659 --before /tmp/wi-before-AT-35-E6-001-c2.json --after docs/work-inventory.json` | 49,438 units |
  | atlas | `missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0` | `python3 scripts/completion_atlas.py --check` | 49,438 units |
  | token coverage | `non_done=0 refused=142 refused_non_done=0 token_types=231 verdict=PASS` | `python3 scripts/token_coverage.py --check` | 49,438 units |
  | converter re-derivation | `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS` (20.6 s) | `cargo run --locked --release --bin sheet_rule_convert -- --check` | 49,438 units |
  | `data/sheet_rules/` token leaks | **0** | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | 49,438 units |
  | shape/engine boundary | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` | `python3 scripts/shape_engine_boundary.py --check` | 49,438 units |
  | missing engine tables | `population=0 kinds=0 citation_failures=0` | `python3 scripts/missing_engine_tables.py --check` | 49,438 units |
  | denominator gate | `files_checked=67 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` | 67 bundle docs |
  | PI sweep | `RESULT: PASS` (1 stage, `pi-sweep`) | `scripts/verify.sh --only pi-sweep` | 49,438 units |
- **Build scope verified**, run at `74abb60839` for the suite and re-run at HEAD for the two
  stages the last two commits could move:
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`
  - `cargo test --locked --no-fail-fast -j 6` → **413 `test result` lines (412 targets plus the
    lib), 5,511 passed, 53 ignored, and exactly ONE failure**:
    `pcgen_import::sheet_rule::table::tests::table_is_a_transcription_of_the_json`, asserting
    `MAXLEVEL: maps_to drifted from the JSON  left: Applies  right: Metadata`. That is the
    transcription gate doing its job — the Rust table changed and its JSON source of truth had
    not yet. Self-healed in the same cycle by correcting the JSON row (see Discoveries 2), and
    re-run at HEAD: `cargo test --locked --lib -j 6` → **`ok. 3238 passed; 0 failed; 14
    ignored`**, that suite being the only one that reads the mapping JSON
    (`grep -rln 'mapping-table.v1.json' --include=*.rs src tests apps` names one file).
  - `cd apps/desktop/src-tauri && cargo test --locked -j 4` → `ok. 576 passed; 0 failed;
    0 ignored`, `DESKTOP_EXIT=0` — run here, not at epic cadence, because this cycle touched
    `apps/`. Its first run failed one assertion this cycle's own change moved
    (`class_catalog::tests::catalog_contains_all_eleven_classes_and_expected_counts`, 1108 →
    1128); the count is updated with its four-term derivation in the same commit and the
    re-run is the green above.
  - `cargo clippy --locked --tests -j 6` → **0 warnings, 0 errors**. The first run raised two
    `clippy::type_complexity` warnings, both in this cycle's own new module; fixed with two
    type aliases (`ProgressionRow`, `RecordCache`) in the same cycle.
  - `cargo run --locked --bin v06_work_inventory` → `INV_EXIT=1`, **deliberately**: the stamp
    guard refused to write, because a bare re-run without `CORPUS_LITERAL_SWEEP_REPORT` and
    `DERIVED_FIXTURE_CHECK_REPORT` in hand would drop 7,385 of the 32,617 verification stamps
    the inventory carries. `--allow-stamp-loss` is forbidden, so the inventory is UNCHANGED —
    which is the right answer for a cycle that moved no unit. The run's own re-derivation of
    the package agrees with the converter: `49296 rule files, 68976 rules, 142 refused ids`.
- **Sweep population:** N/A — no corpus record changed this cycle, so `corpus_literal_sweep` was
  not re-run (`§6` step 3's guard: "only when corpus records changed").
  `git diff --stat c729c0f659..HEAD -- data/corpus` is empty; the only `data/` change is the
  regenerated `data/sheet_rules/`.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`, unchanged by this cycle).
- **Status:** **partial**
- **Notes:**
  - `docs/retro/events/sd31-transcribe.jsonl` carries this cycle's first `correction` because
    `RETRO_ACTOR` is not inherited across shells here; the event content is correct and the two
    later corrections carry `at-35-e6-001`. The mechanism owed: export `RETRO_ACTOR` inside the
    same command that calls `retro.py`, never in a separate one.
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/EPIC-5_wrapup_gate_report.md`
    is untracked in this shared checkout and belongs to another live cycle. Left alone (one
    writer per file); named here so it is not mistaken for this cycle's litter.
- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design)`.
  Two of cycle 1's four families remain, 5 files, 34 identifier hits:

  | family | files | identifier hits | the mechanism that closes it |
  |---|---|---|---|
  | **class-feature var chain** | `pilot_compute/class_feature_grant_consumer.rs` | `PcgenFormulaEvaluator` 6, `bonus_stack_reader` 9 | `resolve_pcgen_var_chain` is a same-name bonus-row fixpoint over corpus token text. Replace with the converter's own `_vars/<VarId>.json` `VarTable` fold, which `sheet_rule::evaluate` already reads. Its one non-test consumer is `derived_evaluator_fixture_check::run_class_feature_description_bar_check`, itself oracle-side work that can follow the race-trait bar check into `src/oracle_validation/`. |
  | **feat prerequisites** | `rules_core/feat_prereqs.rs`, `pilot_compute/prestige_class_entry_gate.rs`, desktop `feat_catalog.rs`, desktop `character_hub.rs` | `pre_tokens` 19 | `evaluate_prerequisite_token` parses `PRE*` token text at run time. The replacement already exists and already ships: `level_up_option_filter::filter_option_pool` over `SheetRule.applies` + `unmet_words` (`AT-35-E5-004`). This family is "deleted where the sheet line already carries the value", and it is the larger of the two — `character_hub.rs` and `feat_catalog.rs` build a `CharacterPrereqFacts` that becomes `CharacterFacts`. |
