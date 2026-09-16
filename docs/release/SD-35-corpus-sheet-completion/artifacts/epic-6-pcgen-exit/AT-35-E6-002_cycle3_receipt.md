# Cycle 3 — Epic 6 (PCGen exit) / AT-35-E6-002

Cycle 1 delivered the criterion's **body** and its second Evidence clause. Cycle 2 took the
first Evidence clause — *zero `raw_tokens` hits under `src/rules_core/`* — from **202 to 110**
and enumerated the 110 as **38 production reads / 36 test-code / 36 doc-comment**, naming the
38 production reads as cycle 3's real job. This cycle closes **all 38**. The clause goes
**110 → 40** (9 files → 5), and **zero production reads of an ingested token array remain
anywhere under `src/rules_core/`**. Status is **partial**: the 40 that survive are the ingest
schema's own field declarations, test fixtures, and doc comments, enumerated below with the
mechanism each needs.

- **Commit SHA:** `b1c0eb9870` — the two new tool-side modules, the live-side re-pointings, the
  two struct narrowings, and this cycle's `deferral` + `incident` + `verification` events.
  `0d960703ff` carries this receipt and the `progress.md` / `kanban.md` rows; this line is
  pinned by a third commit (a receipt cannot name the commit that carries it). Cycle start
  `4cff876d7b`.
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
  moves no unit; it takes the ingest format off the live side.
- **Files touched:** **15** (`git show --stat b1c0eb9870 | tail -1` → `15 files changed, 921
  insertions(+), 656 deletions(-)`) — 13 modified, 2 added, 0 renamed, 0 deleted.
  - **Added, 2 files (both tool side):** `src/pcgen_import/race_trait_tokens.rs` (452 lines),
    `src/pcgen_import/pool_member_tokens.rs` (270 lines).
  - **Module declaration, 1 file:** `src/pcgen_import/mod.rs` (two `pub mod` lines, each with
    its `decisions.md §11` / `technical-design.md §0` citation).
  - **Tool-side accessor widened, 1 file:** `src/pcgen_import/ingest_record.rs`
    (`type_token_suffix` + its unit test).
  - **Live modules re-pointed, 5 files:** `src/rules_core/{race_resolver, race_creation,
    trait_pool, class_feature_pool_catalog, derived_evaluator_fixture_check}.rs`.
  - **Desktop, 1 file:** `apps/desktop/src-tauri/src/race_catalog.rs` (reads
    `ResolvedTrait::declared_vision` / `declared_walk_speed_ft` instead of the token array).
  - **Tool import path followed, 1 file:** `src/bin/v06_work_inventory.rs`.
  - **Integration test followed, 1 file:** `tests/sd27_crb_race_corpus_pin.rs`.
  - **Retro, 1 file:** `docs/retro/events/at-35-e6-002.jsonl` (this cycle's `deferral`,
    `incident` and the `verify.sh` `verification` event — see **Notes**).
  - **Folded working-tree appends, 2 files:**
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/{completion-atlas.json,
    shape-engine-boundary.md}` (the `derived_at` re-stamps `completion_atlas.py --check` and
    `shape_engine_boundary.py --check` write).
  - **Zero `data/` files changed.** `git status --porcelain -- data/` is empty.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS in shipping code — **1 match, attributed, not a
  violation, and not this cycle's.**
  ```
  BASE_BRANCH=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47   # git merge-base HEAD origin/develop
  CODE="src/rules_core src/pcgen_import src/bin src/oracle_validation apps/desktop/src-tauri/src"
  git diff --unified=0 "${BASE_BRANCH}...HEAD" -- $CODE ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  ->  15723:+/// (`tests/sd34_wave51_racial_sla_catalog_matches_the_corpus.rs`); this function is its
  ```
  The same doc-comment citation of a real test file's path in `src/rules_core/racial_sla.rs`
  that every `AT-35-E5-*` and `AT-35-E6-*` receipt has recorded — pre-existing and unchanged
  here. **This cycle contributes 0.**
- **Wired-integration audit result:** OK_NO_TOKENS.
  ```
  git diff --unified=0 "${BASE_BRANCH}...HEAD" -- $CODE ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'   ->  OK_NO_TOKENS
  ```
- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E6-002`):
  > `src/rules_core/cache_gen/**` and `wiring_class.rs` relocate to `src/pcgen_import/`
  > behavior-identically (they are converter code that lives on the wrong side). Every `src/bin`
  > generator's import path follows.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero `raw_tokens` hits under
  > `src/rules_core/`; `gen_book_cache` output byte-identical before and after on one book.

  | clause | at HEAD | met? |
  |---|---|---|
  | `cache_gen/**` relocated to `src/pcgen_import/` | 16 files, cycle 1 | **yes** |
  | `wiring_class.rs` relocated | `src/pcgen_import/wiring_class.rs`, cycle 1 | **yes** |
  | behaviour-identical | both suites at the pre-cycle figures plus this cycle's 10 named new tests, below | **yes** |
  | every `src/bin` generator's import path follows | cycle 1's 63 files, cycle 2's 9, this cycle's 1, 0 residual | **yes** |
  | `gen_book_cache` byte-identical on one book | cycle 1: 2,207 records, same manifest sha256 | **yes** |
  | zero `raw_tokens` hits under `src/rules_core/` | **5 files, 40 matches** (was 9 / 110) | **no** |
- **Receipt rows (mechanical):**
  ```
  since=4cff876d7b922a1390795665d95ef862d472ed7b target_dir=/tmp/cargo-sd35-AT-35-E6-002 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=1550 ratio=n/a builds_recorded=2 pcgen_live_files=248
  ```
  `closed=0` / `relabeled=0` is correct: the unit population was already `0` non-DONE at cycle
  start. `ratio` is `n/a`, a division by zero, never `0.0`. `pcgen_live_files=248` is **one
  below** cycle 2's `249` and above nothing. **`builds_recorded=2` in the cycle's own target dir**
  — this cycle ran one verification sequence there (`--no-run`, `--lib`, `--no-fail-fast`,
  `sheet_rule_convert --check`) after two `cargo check` passes taken during development, which
  is what "one build per cycle" (`decisions.md §3`) forbids per *item*, not per *iteration*.
  `cargo clippy` and the desktop crate ran in their own target dirs
  (`…-clippy`, `…-desktop`) so neither contended on the workspace run's cargo lock.
- **PCGen residue** (`python3 scripts/pcgen_residue_gate.py --check`, at `b1c0eb9870`):
  ```
  pattern raw_tokens files=12 hits=71
  pattern PcgenFormulaEvaluator files=0 hits=0
  pattern render_pcgen_desc files=17 hits=109
  pattern bonus_stack_reader files=0 hits=0
  pattern pre_tokens files=0 hits=0
  pattern BONUS: files=138 hits=2221
  pattern DEFINE: files=26 hits=116
  pattern PRE[A-Z]+: files=119 hits=7932
  pattern SAB: files=0 hits=0
  pattern DESC: files=137 hits=565
  pattern %CHOICE files=8 hits=66
  pattern %LIST files=24 hits=139
  pattern TYPE= files=68 hits=737
  root src/rules_core files=197 hits=11480
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=51 hits=476
  identifier_files=26 identifier_hits=180
  live_files=248 live_hits=11956 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  **Down on both axes and up on neither** — `249 → 248` files, `12,049 → 11,956` hits against
  cycle 2. The gate's own `raw_tokens` pattern fell **17 files / 142 hits → 12 / 71**; the
  `src/rules_core/` share of it is **9 → 5 files, 110 → 40 hits**, and `apps/desktop/`'s share
  fell **26 → 25**. The baseline file is deliberately NOT rebaselined; `--rebaseline` is
  `AT-35-E6-004`'s step.
- **Oracle parity:** **N/A for this cycle, and correct by construction.** Epic 6 touches a live
  path, so the row is owed an answer; this cycle's answer is that the live path's *behaviour* is
  unchanged, which is stronger than agreement within tolerance. Every reading moved to
  `src/pcgen_import/` is byte-for-byte the same predicate over the same token array — the
  `DEFINE`/`DESC`/`ABILITY`/`CHOOSE`/`PREFACT`/`MOVE`/`TEMPLATE`/`VISION` filters, the four
  pool predicates, and the four private parsers are transcribed unchanged, and their existing
  unit tests moved with them and still pass. The two struct narrowings resolve the same facts at
  construction time that the removed methods resolved on demand. The full workspace suite
  reproduces the pre-cycle figure exactly apart from this cycle's own **+10** new tests, all
  named below. No `Number` mapping was added, so the fixture-roster oracle comparison is not
  triggered (`§6` step 3). `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`,
  unchanged.
- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** 0 — the population was already 0 non-DONE at cycle start.
  - **relabel (bucket to bucket):** 0. No unit changed bucket.
  - **reachability:** 0. This cycle evaluated nothing and converted nothing.
  - **instrument-correction:** 0. `pcgen_residue_gate.py` is untouched by this cycle
    (`git diff --stat 4cff876d7b..HEAD -- scripts/pcgen_residue_gate.py` is empty); the fall in
    its output is entirely the code moving.
- **Refused tokens:** **none.** This cycle added no converter refusal and cleared none; the
  refused set is unchanged at **142** records, one shape, `refused_non_done=0`
  (`token_coverage.py --check`). It shipped no converter mapping row at all. The 40-match
  remainder below is **live-side schema, fixtures and prose, not a converter refusal**, so §8's
  "more than 10 distinct refused token types" escalation does not apply.
- **What the three mechanisms were, and why none is a rename.**
  1. **`race_trait_tokens.rs` — the race/race-trait readings leave the live side (39 matches).**
     `race_resolver.rs` walked the ingested token array in **six** places, keyed by PCGen token
     name, to answer six different questions. All six are now one named function each on the
     tool side (`same_row_defines`, `description_segments`, `automatic_ability_grants`,
     `choice_pool_suffix`, `positive_prefact_flag`, plus `declared_walk_speed_ft` /
     `declared_size` / `declared_vision_segments`), reached through an `IngestTokens` trait
     implemented **in `src/pcgen_import/`** for the `shape_b_v1` cache structs — so the field
     read itself sits on the tool side of `technical-design.md §0`'s path boundary and the live
     caller never names the ingest field. The four private parsers
     (`automatic_grant_targets`, `first_ability_flag`, `size_from_size_template`,
     `walk_speed_from_move`) and `declared_template_bonus_languages` moved with them, bodies
     and doc comments unchanged, and so did their **12 existing unit tests**; 8 further tests
     were added for the new named readers. `src/bin/v06_work_inventory.rs` — the only consumer
     of `declared_template_bonus_languages`, and itself tool side — follows the import.
  2. **Two live structs stop carrying the token array (a narrowing, not an indirection).**
     `ResolvedTrait` held `raw_tokens: Vec<RawToken>`, a clone of a whole ingested row, and
     three consumers read PCGen keys out of it (`declared_walk_speed_ft` and `declared_size` in
     `race_resolver`, `VISION` in `race_creation`, `VISION` again in the desktop
     `race_catalog`). It now holds three resolved facts — `declared_walk_speed_ft: Option<i32>`,
     `declared_size: Option<SizeCategory>`, `declared_vision: Vec<String>` — read **once**, at
     resolution time, through the tool side. `TraitPoolRecord` held the array and only ever had
     its `TYPE:` third dot-segment read out of it; it now holds `race_trait_pool:
     Option<String>`, resolved at load time through the new
     `ingest_record::type_token_suffix`. Both changes **remove** a PCGen-shaped field from a
     live type rather than renaming one, which is why `apps/desktop` loses a hit as a
     side-effect: the desktop had no other way to ask.
  3. **`pool_member_tokens.rs` — the pool-membership predicates leave the live side (27 matches).**
     `class_feature_pool_catalog.rs` decided pool membership by four predicates over the raw
     token array (`has_no_engine_effect_token`, `is_archetype_locked`,
     `raw_tokens_carry_more_than_one_desc_segment`,
     `shipped_description_is_the_already_regenerated_safe_multi_desc_join`) plus
     `ENGINE_EFFECT_TOKEN_KEYS`. All five are ingest-format questions — *which token keys does
     this row carry* — so all five moved, with their four unit tests and every doc comment
     recording **why** each refusal exists. One identifier was renamed in the move:
     `raw_tokens_carry_more_than_one_desc_segment` → `carries_more_than_one_desc_segment`, because
     the identifier itself named the ingest field. A fifth test was added for the thin-record
     (no token array) case the `Value`-shaped predicates now inherit from `ingest_record`.
     Separately, the module's **four ground-truth corpus assertions** and
     `derived_evaluator_fixture_check.rs`'s **three corpus-wide sweeps** stopped open-coding
     `json["data"]["raw_tokens"].as_array().expect(…)` and call `ingest_record` instead — the
     accessor cycle 2 built for exactly this.
- **Why the two comment rewrites are not a comment sweep.** Cycle 2's rule was: *no comment is
  reworded in any file that still carries a code read.* This cycle reworded comments in exactly
  two files — `race_resolver.rs` (2 comments) and `class_feature_pool_catalog.rs` (4) — and in
  both the code half was reduced to **zero** first, by the moves above, in the same commit. Every
  other surviving comment sits in a file that still carries a code read and was left alone. The
  one comment cycle 2 deliberately left, `damage_total.rs:917`'s runnable `python3 -c` re-derive
  command, is left again for the same reason: editing it would break a command a reader can run
  (`AGENTS.md` §9) to move a grep count by one.
- **The remainder, enumerated.** Denominator: the whole of `src/rules_core/` at `b1c0eb9870` —
  **5 files / 40 gate matches**, of which **5 schema declarations**, **13 test code**, **22
  doc-comment**, and **0 production reads**. Two independent derivations agree on 40:
  `grep -rho '\braw_tokens\b' --include=*.rs src/rules_core/ | wc -l` and a Python
  `re.findall(r'\braw_tokens\b')` pass (`AGENTS.md` §Concurrency).

  | file (under `src/rules_core/`) | matches | schema | test | comment | what it is, and what it needs |
  |---|---:|---:|---:|---:|---|
  | `shape_b_v1.rs` | 13 | 5 | 4 | 4 | The **ingest cache schema itself** (5 × `pub raw_tokens: Vec<RawToken>`). Not relocated: its consumers include live `rules_tables` and the desktop `race_catalog`, so moving the file relocates a hit without relocating the dependency (cycle 2's refusal, unchanged). Needs live code to stop deserializing ingest records at all. |
  | `corpus_loader.rs` | 16 | 0 | 2 | 14 | **Production side clear since cycle 2.** 14 doc comments plus a 2-match test fixture that pins the literal on-disk record shape for `equipment_record_from_json`. The comments stay because the file still carries that code read. |
  | `derived_evaluator_fixture_check.rs` | 7 | 0 | 6 | 1 | **Production side clear this cycle** (3 sweeps re-pointed). What remains is 6 ingest-shaped JSON test fixtures written to scratch files by its own tests, and 1 comment. |
  | `pi_screening.rs` | 3 | 0 | 1 | 2 | A screened-field-list **string** (`"description,name,raw_tokens"`) in a test, plus two comments — the field name is data here, not a read. |
  | `damage_total.rs` | 1 | 0 | 0 | 1 | The runnable re-derive command above. |
  | **total** | **40** | **5** | **13** | **22** | |

  Sums verified independently: `13+16+7+3+1 = 40` and `5+13+22 = 40`.
  Recorded as `deferral 1789079233462-at-35-e6-002-afab04`, not as a silent gap. Nothing here is
  a carve-out: 40 is a number to close.
- **Discoveries:** **one.** Cycle 2 classified the 110 as 38 production / 36 test / 36 comment.
  Re-derived at HEAD after this cycle's moves, the same classifier reads **5 / 13 / 22** — the
  38 production reads are gone, but the test and comment halves also fell by 23 and 14, because
  a moved reading takes its own tests and its own justifying doc comments with it rather than
  leaving them behind. That is not a correction to a published figure (cycle 2's 38/36/36 was
  correct for the tree it measured, and this receipt re-derives it with the same classifier), so
  no `correction` event is owed; it is a **category** finding, and the category matters for
  scoping cycle 4: the remainder is no longer "readers that need converting" at all. It is the
  ingest schema (`shape_b_v1`'s 5 field declarations) and the fixtures and prose that cite it.
  **Cycle 4's job is a different shape of work from cycles 1–3**, and it is the same work
  `AT-35-E6-003` owes for `apps/desktop`.
- **Figures + their re-derive commands:** every row carries its own command. The unit denominator
  where one applies is the whole corpus, all books — **49,438** (`jq '.units | length' docs/work-inventory.json`).

  | figure | value | command | denominator |
  |---|---|---|---|
  | `raw_tokens` under `src/rules_core/`, files | **5** (from 9) | `grep -rl '\braw_tokens\b' --include=*.rs src/rules_core/ \| wc -l` | 197 live `src/rules_core` files the gate scans |
  | `raw_tokens` under `src/rules_core/`, matches | **40** (from 110) | `grep -rho '\braw_tokens\b' --include=*.rs src/rules_core/ \| wc -l` | as above |
  | of which schema / test / comment | **5 / 13 / 22** | the classifier in this receipt's Notes (comment = a line whose first non-space is `//`, `/*` or `*`; test = a code line at or below the file's first `#[cfg(test)]`) | 40 matches |
  | production reads remaining | **0** (from 38) | same classifier, `prod` column, over `src/rules_core/`; the 5 non-test non-comment matches are `pub raw_tokens: Vec<RawToken>` field declarations in `shape_b_v1.rs` | 40 matches |
  | `raw_tokens` under `apps/desktop/`, matches | **25** (from 26) | `grep -rho '\braw_tokens\b' --include=*.rs apps/desktop/ \| wc -l` | 51 live `apps/desktop` files the gate scans |
  | live PCGen files / hits | **248 / 11,956** (from 249 / 12,049) | `python3 scripts/pcgen_residue_gate.py --check`, last line | 49,438 units |
  | gate's `raw_tokens` pattern, all live roots | **12 files / 71 hits** (from 17 / 142) | same command, first line | 5 live roots |
  | new tool-side lines | **753 insertions, 0 deletions, 0 files removed** | `git diff --numstat --find-renames 4cff876d7b..HEAD -- src/pcgen_import scripts/oracle_harness src/oracle_validation`; `… --name-status … \| grep -c '^D'` | files in that window |
  | `data/sheet_rules/` token leaks | **0** | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | 49,438 units |
  | sheet-rule package | `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS (112.8s)` | `cargo run --locked --bin sheet_rule_convert -- --check` | 49,438 units |
  | atlas | `population=49438 buckets=10 unclassified=0 overlap=0 done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`; `DONE: 49438`, every other bucket `0` | `python3 scripts/completion_atlas.py --check` | 49,438 units |
  | token coverage | `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS` | `python3 scripts/token_coverage.py --check` | 49,438 units |
  | shape/engine boundary | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` | `python3 scripts/shape_engine_boundary.py --check` | 49,438 units |
  | missing engine tables | `population=0 kinds=0 citation_failures=0` | `python3 scripts/missing_engine_tables.py --check` | 49,438 units |
  | denominator gate | `files_checked=89 violations=0` (88 before this receipt landed) | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` | 89 bundle docs |
  | figure provenance | `files_checked=206 figures_examined=443 violations=0` (205 / 435 before this receipt landed) | `python3 scripts/denominator_gate.py --check-provenance` | 206 docs |
  | site dashboard pin | `input pin matches docs/work-inventory.json (5a0a0787312b5181d41214cb52abcd6e0c250fc409a75675ed6e839b4142e36f)` | `./scripts/publish-site-dashboard.sh --check-pin` | 1 feed input |
- **Build scope verified**, all at `b1c0eb9870`, `CARGO_INCREMENTAL=0`:
  - `cargo test --locked --no-run -j 6` (target dir `/tmp/cargo-sd35-AT-35-E6-002`) → `NO_RUN_EXIT=0`, **413 `Executable` lines** (`grep -c '^  Executable' /tmp/e6002c3-norun.log`) — unchanged from cycles 1 and 2, as two new modules inside an existing crate must leave it. `grep -cE '^(error|warning)'` → **0**. Cold wall clock **2 m 50 s**, max RSS 2.2 GB.
  - `cargo test --locked --lib -j 6` → `ok. 3276 passed; 0 failed; 15 ignored` (40.30 s), `LIB_EXIT=0`. **+10 against cycle 2's 3,266**, and the ten are named: 8 in `pcgen_import::race_trait_tokens::reader_tests` (`defines_read_name_and_integer_base_and_report_an_unresolvable_base_as_none`, `both_vision_shapes_flatten_to_the_same_segment_list`, `a_row_with_no_vision_token_declares_no_sense`, `walk_speed_and_size_read_their_own_token_and_nothing_else`, `automatic_grants_stop_at_the_first_gate_and_skip_the_choice_placeholder`, `a_positive_gate_names_its_flag_and_a_negated_one_is_not_a_token_this_reads`, `a_choice_selector_yields_the_pool_suffix_after_its_prefix`, `description_segments_preserve_file_order_and_are_empty_when_the_row_states_none`), 1 in `pcgen_import::ingest_record::tests` (`a_type_token_yields_only_the_tail_behind_its_prefix`), 1 in `pcgen_import::pool_member_tokens::tests` (`a_thin_record_with_no_token_array_is_prose_only_and_unlocked`). The 16 tests that moved modules kept their names and their results. No pre-existing test changed its result.
  - `cargo test --locked --no-fail-fast -j 6` → `FULL_EXIT=0`, **414 `test result` lines, 8,787 passed, 0 failed, 68 ignored, ZERO failing suites** — `grep -cE '^test result: FAILED'` → **0**, `grep -cE '^(error|warning)'` → **0**. Totals derived with `awk` over the `test result` lines, not `grep -o` (`AGENTS.md` §Concurrency). Against cycle 2's 414 / 8,777 / 68 / 0 the **only** movement is the same **+10**; the target count is unchanged at 414.
  - `cargo clippy --locked --tests -j 6` → **exit 0, 0 warnings, 0 errors**, in its own target dir `/tmp/cargo-sd35-AT-35-E6-002-clippy`.
  - **Desktop crate and frontend RAN HERE, not at the epic wrap-up**, because this cycle touched `apps/` (`git diff --name-only 4cff876d7b..HEAD -- apps/` lists `apps/desktop/src-tauri/src/race_catalog.rs`): `cd apps/desktop/src-tauri && cargo test --locked -j 4` → `ok. 576 passed; 0 failed; 0 ignored` (87.68 s), `DESKTOP_EXIT=0`, own target dir `/tmp/cargo-sd35-AT-35-E6-002-desktop`; `cd apps/desktop && npm test` → `101/101 test files passed`, `FRONTEND_EXIT=0`; `npm run typecheck` → `TSC_EXIT=0`.
  - `scripts/verify.sh --only pi-sweep` → `RESULT: PASS` (1 stage, 11 hits over `src/rules_core/rules_tables`, 11 baseline rows).
  - `cargo run --locked --bin v06_work_inventory`: **not run, deliberately.** This cycle changed no corpus record, so `corpus_literal_sweep` is guarded off (`§6` step 3), so the two reports the binary rebuilds its verification stamps from do not exist for this tree — it would refuse to write, and `--allow-stamp-loss` is forbidden. `git status --porcelain -- docs/work-inventory.json` is empty and the `--receipt` rows above were computed against the file on disk.
- **Sweep population:** N/A — no corpus record changed (`git status --porcelain -- data/` empty
  at every checkpoint), so `corpus_literal_sweep` was correctly not run (`§6` step 3's guard).
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`, unchanged by this cycle).
- **Status:** **partial.** The criterion's body and its second Evidence clause were met in cycle
  1; its first Evidence clause is **not yet met** — **5 files, 40 gate matches** remain, down
  from 9 / 110, with **0 production reads**. Deferred explicitly, not silently:
  `deferral 1789079233462-at-35-e6-002-afab04`.
- **Notes:**
  - **The `verify.sh` retro-actor misfiling did NOT recur.** Cycle 2 recorded it three times
    (`retro-actor-lost-between-bash-calls`); this cycle exported `RETRO_ACTOR` in the same shell
    invocation as `scripts/verify.sh` and the derived `verification` event landed in
    `docs/retro/events/at-35-e6-002.jsonl` correctly. **That is a workaround, not the mechanism**
    — the instrument should derive its actor from the cycle's receipt path — so the key stays
    open at 3 occurrences.
  - **One incident this cycle could not clear, reported for a ruling.** The shared checkout
    carries an untracked, un-gitignored `.worktrees/` directory holding another session's live
    git worktree, so `git status --porcelain` is non-empty for every cycle on `tranche/15` and
    the standing "clean tree = unfiltered status empty" rule is unreachable. It is deliberately
    **not** committed (a git worktree is not repo content) and deliberately **not** `.gitignore`d
    (a shared root file outside this criterion's file-touch set). One `.worktrees/` line in
    `.gitignore` is the mechanism. `incident 1789079245735-at-35-e6-002-507e75`, recurrence key
    `untracked-worktrees-dir-on-shared-checkout`.
  - **Two build errors self-healed inside the cycle, before any commit**, both surfaced by
    `cargo check --lib --tests`: the moved private parsers left four test call sites in
    `race_resolver.rs` unresolved (the tests moved with them), and `tests/sd27_crb_race_corpus_pin.rs`
    read `ResolvedTrait::raw_tokens` (it reads `declared_vision` now, the same VISION values).
    §8's self-heal posture, not an escalation.
- **Next-cycle scope:** `AT-35-E6-002` **cycle 4**, still sequenced **with** `AT-35-E6-003`.
  Scope flags: `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design, decisions.md §2)`.
  Population is the table above: **5 files / 40 gate matches / 0 production reads**. The work is
  no longer "convert a reader": it is `shape_b_v1.rs`'s 5 ingest-schema field declarations and
  the 35 fixtures and doc comments that cite them. Take them in this order: decide the schema's
  home first (it is the only item with live consumers — `rules_tables` and the desktop
  `race_catalog` — and `AT-35-E6-003` must decide the same thing for `apps/desktop`'s 25), then
  `corpus_loader.rs` (16) and `derived_evaluator_fixture_check.rs` (7) whose fixtures follow the
  schema's decision, then `pi_screening.rs` (3) and `damage_total.rs` (1).
