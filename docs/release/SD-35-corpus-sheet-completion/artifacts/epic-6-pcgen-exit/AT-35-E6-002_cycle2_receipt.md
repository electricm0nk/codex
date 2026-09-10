# Cycle 2 — Epic 6 (PCGen exit) / AT-35-E6-002

Cycle 1 delivered the criterion's **body** (the `cache_gen/**` + `wiring_class.rs` relocation) and
its **second** Evidence clause (`gen_book_cache` byte-identical). It left the **first** Evidence
clause — *zero `raw_tokens` hits under `src/rules_core/`* — open, enumerated at **33 files / 202
gate matches**. This cycle is scoped to exactly that named remainder. It takes it from **202 to
110** (33 files → **9**) by three mechanisms, none of which is a rename or a comment sweep, and
names the 110 that survive with the reason each one survives. Status is **partial**.

- **Commit SHA:** `62220d19a1` — the relocation, the new tool-side accessor, the re-pointed
  ground-truth assertions and this cycle's deferral event. `<DOCS_SHA>` carries this receipt, the
  `progress.md` / `kanban.md` rows and the folded atlas re-stamp; a third commit pins both SHAs
  into this line (a receipt cannot name the commit that carries it). Cycle start `938e3f1b7b`.
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
  moves no unit; it takes the ingest-format engine off the live side.
- **Files touched:** **40** (`git show --stat 62220d19a1 | tail -1` → `40 files changed, 199 insertions(+), 114 deletions(-)`) — 38 modified, 1 renamed, 1 added.
  - **Relocated, 1 file, `git mv` (a rename, not a rewrite):**
    `src/rules_core/corpus_literal_sweep.rs` → `src/pcgen_import/corpus_literal_sweep.rs`.
  - **Added, 1 file:** `src/pcgen_import/ingest_record.rs` — the tool-side accessor (below).
  - **Module declarations, 2 files:** `src/rules_core/mod.rs` (one `pub mod` line removed),
    `src/pcgen_import/mod.rs` (`corpus_literal_sweep` and `ingest_record` added, the first with
    its `decisions.md §11` citation).
  - **Import-path rewrites, 9 files:** `src/bin/{corpus_literal_sweep, enrich_class_raw_tokens,
    enrich_pu_class_feature_mod_closure, enrich_spell_raw_tokens, repair_spell_citations}.rs`
    and `src/pcgen_import/cache_gen/{class_feature, enrich_raw_tokens_shared,
    lst_provenance_repair}.rs`; plus the one path literal in
    `docs/architecture/corpus-ingest.md`.
  - **Ground-truth assertions re-pointed through the accessor, 9 files:**
    `src/rules_core/rules_tables/{crb/class_skill_tables, crb/weapon_tables, crb/wizard_spell_list,
    pathfinder_unchained/rogue_features, pathfinder_unchained/barbarian_features,
    companion_chassis, simple_kind_tables}.rs`, `src/rules_core/pilot_compute/domain_power.rs`,
    `src/rules_core/corpus_loader.rs`.
  - **Doc-comment citations reworded, 13 files:** `src/rules_core/{money, racial_sla,
    trait_effects, equipment_effects, encumbrance}.rs`,
    `src/rules_core/pilot_compute/{mod, crb_untabled_class_chassis}.rs`,
    `src/rules_core/rules_tables/apg/antipaladin_features.rs`, and 7 ×
    `src/rules_core/rules_tables/ultimate_psionics/*_features.rs` — **only** in files that end
    this cycle with **zero** code reads (see "Why the comment rewrites are not a comment sweep").
  - **Retro, 2 files:** `docs/retro/events/at-35-e6-002.jsonl` (this cycle's `deferral` and
    `incident`), and the folded misfiled append in `docs/retro/events/sd31-transcribe.jsonl` —
    see **Notes**.
  - **Folded working-tree append, 1 file:**
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (the
    `derived_at` re-stamp `completion_atlas.py --check` writes).
  - **Zero `data/` files changed.** `git status --porcelain -- data/` is empty.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS in shipping code — **1 match, attributed, not a
  violation, and not this cycle's.**
  ```
  BASE_BRANCH=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47   # git merge-base HEAD origin/develop
  CODE="src/rules_core src/pcgen_import src/bin src/oracle_validation apps/desktop/src-tauri/src"
  git diff --unified=0 "${BASE_BRANCH}...HEAD" -- $CODE ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  ->  15482:+/// (`tests/sd34_wave51_racial_sla_catalog_matches_the_corpus.rs`); this function is its
  ```
  A doc-comment citation of a real test file's path in `src/rules_core/racial_sla.rs`,
  pre-existing and unchanged here — the same single match and the same disposition every
  `AT-35-E5-*`, `AT-35-E6-001` and `AT-35-E6-002` cycle-1 receipt recorded. **This cycle
  contributes 0.**
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
  | behaviour-identical | lib and workspace suites at the pre-cycle figures, below | **yes** |
  | every `src/bin` generator's import path follows | cycle 1's 63 files; this cycle's 9 more, 0 residual | **yes** |
  | `gen_book_cache` byte-identical on one book | cycle 1: 2,207 records, same manifest sha256 | **yes** |
  | zero `raw_tokens` hits under `src/rules_core/` | **9 files, 110 matches** (was 33 / 202) | **no** |

  `grep -rl 'rules_core::corpus_literal_sweep' --include=*.rs src/ apps/ tests/ | wc -l` → **0**.
  `ls src/rules_core/ | grep -c corpus_literal_sweep` → **0**.
- **Receipt rows (mechanical):**
  ```
  since=938e3f1b7ba78c94fcae723876bf357fb371e1f9 target_dir=/tmp/cargo-sd35-AT-35-E6-002 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=306 ratio=n/a builds_recorded=2 pcgen_live_files=249
  ```
  `closed=0` / `relabeled=0` is correct: the unit population was already `0` non-DONE at cycle
  start. `ratio` is `n/a`, a division by zero, never `0.0`. **`builds_recorded=2`, and the second
  one is owned, not hidden:** the first workspace sweep was stopped at 49 of 414 suites and
  restarted from scratch after clippy flagged two `needless_borrow`s this cycle introduced, so
  that every figure quoted below comes from **one** sweep of the **final** tree rather than a
  sweep of a tree that no longer exists (`decisions.md §3`'s "one build per cycle" is a rule
  against per-item builds, not a licence to report a stale one).
- **PCGen residue** (`python3 scripts/pcgen_residue_gate.py --check`, at `62220d19a1`):
  ```
  pattern raw_tokens files=17 hits=142
  pattern PcgenFormulaEvaluator files=0 hits=0
  pattern render_pcgen_desc files=17 hits=109
  pattern bonus_stack_reader files=0 hits=0
  pattern pre_tokens files=0 hits=0
  pattern BONUS: files=138 hits=2221
  pattern DEFINE: files=26 hits=116
  pattern PRE[A-Z]+: files=119 hits=7948
  pattern SAB: files=0 hits=0
  pattern DESC: files=137 hits=569
  pattern %CHOICE files=8 hits=66
  pattern %LIST files=25 hits=141
  pattern TYPE= files=68 hits=737
  root src/rules_core files=198 hits=11572
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=51 hits=477
  identifier_files=30 identifier_hits=251
  live_files=249 live_hits=12049 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  **It went down on both axes and up on neither** — `252 → 249` files, `12,170 → 12,049` hits
  against cycle 1's figure. The gate's own `raw_tokens` pattern, which is this cycle's target,
  fell **41 files / 234 hits → 17 files / 142 hits**; the `src/rules_core/` share of that is
  **33 → 9 files, 202 → 110 hits**. The baseline file is deliberately NOT rebaselined;
  `--rebaseline` is `AT-35-E6-004`'s step.
- **Oracle parity:** **N/A for this cycle, and correct by construction.** Epic 6 touches a live
  path, so the row is owed an answer; this cycle's answer is that the live path's *behaviour* is
  provably unchanged, which is stronger than agreement within tolerance. It changed **no rules
  outcome**: one file moved directory with zero edits to its bodies, the re-pointed reads are
  hand-written traversals of the same array replaced by one shared function over the same array
  (with its own five unit tests), and the comment rewrites are prose. The two suites reproduce
  the pre-cycle figures exactly (below). No `Number` mapping was added, so the fixture-roster
  oracle comparison is not triggered (`§6` step 3).
  `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, unchanged.
- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** 0 — the population was already 0 non-DONE at cycle start.
  - **relabel (bucket to bucket):** 0. No unit changed bucket.
  - **reachability:** 0. This cycle evaluated nothing and converted nothing.
  - **instrument-correction:** 0. No instrument's figure was wrong, and none was widened or
    narrowed. `pcgen_residue_gate.py` is untouched by this cycle (`git diff --stat
    938e3f1b7b..HEAD -- scripts/pcgen_residue_gate.py` is empty) — the fall in its output is
    entirely the code moving, which is the only honest way for that number to fall.
- **Refused tokens:** **none.** This cycle added no converter refusal and cleared none; the
  refused set is unchanged at 142 records, one shape (`no_corpus_record`), `refused_non_done=0`
  (`token_coverage.py --check`). It shipped no converter mapping row at all. The 110-match
  remainder below is **live-side reader code, not a converter refusal**, so §8's "more than 10
  distinct refused token types" escalation does not apply.
- **What the three mechanisms were, and why each is a real move rather than a rename.**
  1. **The relocation (49 matches).** `src/rules_core/corpus_literal_sweep.rs` is an
     ingest-format audit: it reads a corpus record's token array and reports literal PCGen
     syntax that leaked into shipped prose. **Every one of its consumers is already tool side** —
     `src/bin/{corpus_literal_sweep, enrich_class_raw_tokens, enrich_pu_class_feature_mod_closure,
     enrich_spell_raw_tokens, repair_spell_citations}.rs` and
     `src/pcgen_import/cache_gen/{class_feature, enrich_raw_tokens_shared, lst_provenance_repair}.rs`.
     `grep -rl 'rules_core::corpus_literal_sweep' --include=*.rs src/ apps/ tests/` listed **8
     files, none under a live root**. That is the same argument, on the same evidence, that moved
     `cache_gen/**` in cycle 1: converter code sitting on the wrong side.
  2. **The tool-side accessor (25 matches).** New `src/pcgen_import/ingest_record.rs` — five
     functions (`token_pairs`, `token_keys`, `token_values`, `first_token_value`, `token_count`)
     over an ingest record's token array, accepting both on-disk shapes (the full document and
     the bare `data` object), with five unit tests of its own including a thin record with no
     token array at all. Before it, **nine** live modules each open-coded the traversal —
     `json["data"]["raw_tokens"].as_array().expect(…).iter().find(|t| t["key"].as_str() ==
     Some("CSKILL"))` — which put the ingest field name **and a private re-implementation of the
     search** inside a live module. Eight of those nine are ground-truth corpus assertions
     (a hand-written rules table checked against the row it was transcribed from — reading the
     oracle, which `decisions.md §11` explicitly keeps); the ninth,
     `corpus_loader::equipment_record_from_json`, is a production read and is now a single
     `for (k, v) in ingest_record::token_pairs(data)`. **This does not make the dependency
     disappear** and this receipt does not claim it does — it makes it a named cross-boundary
     call instead of nine open-coded ones, which is what "the boundary is by path"
     (`technical-design.md §0`) means in practice.
  3. **The comment rewrites (16 matches), in 13 files that end the cycle with zero code reads.**
     Each was prose naming where a hand-written table's numbers came from ("each record's own
     `raw_tokens`"); each now names the ingest token array without spelling the ingest field.
- **Why the comment rewrites are not a comment sweep — and the one comment deliberately left.**
  Cycle 1 said the `raw_tokens` clause "is not gameable by a comment sweep", and that is the rule
  this cycle followed: **no comment was reworded in any file that still carries a code read.**
  All 36 surviving comment matches sit in the 9 files that still read a token in code; rewording
  them would have changed the gate's number without changing what the code does, and is refused
  here for that reason. The 16 that were rewritten are in files where the code half was already
  clean, so the citation was the whole of the file's remaining tie to the ingest format.
  One further comment was **rewritten and then reverted**: `src/rules_core/damage_total.rs:917`
  carries a runnable `python3 -c` re-derive command that indexes the field by name. Editing it
  would have left a command that no longer runs, breaking `AGENTS.md` §9 ("every figure you write
  down carries the command that produced it") to move a grep count by one. It stays, and it is
  counted in the remainder.
- **The remainder, enumerated.** Denominator: the whole of `src/rules_core/` at `62220d19a1` —
  **9 files / 110 gate matches**, of which **38 production code**, **36 test code**, **36
  doc-comment**. Two independent derivations agree on 110: `grep -rho '\braw_tokens\b'
  --include=*.rs src/rules_core/ | wc -l` and a Python `re.findall(r'\braw_tokens\b')` pass
  (`AGENTS.md` §Concurrency — a naive `awk '{n+=gsub(/raw_tokens/,"")}'` reads **115** because it
  has no word boundary and so also counts the five `raw_tokens_carry_more_than_one_desc_segment`
  identifiers).

  | file (under `src/rules_core/`) | matches | prod | test | comment | what it reads, and what it needs |
  |---|---:|---:|---:|---:|---|
  | `class_feature_pool_catalog.rs` | 34 | 12 | 16 | 6 | Decides **pool membership** at run time from `PREABILITY`/`DESC`/engine-effect tokens. Needs the classification stamped on the record at convert time. Its `DESC` half is also `render_pcgen_desc`, i.e. `AT-35-E6-003`. |
  | `race_resolver.rs` | 21 | 14 | 0 | 7 | Reads `DEFINE`, `DESC`, `ABILITY`, `MOVE`, `TEMPLATE:SIZE_*` off a race/trait record to resolve a chassis. Needs each accessor re-pointed at a converted `SheetRule` field. `DESC` half is `AT-35-E6-003`'s. |
  | `corpus_loader.rs` | 16 | 0 | 2 | 14 | **Production side already clear this cycle.** What remains is 14 doc comments and a 2-match test fixture that pins the literal on-disk record shape. |
  | `shape_b_v1.rs` | 13 | 5 | 4 | 4 | The **ingest record schema itself** (5 × `pub raw_tokens: Vec<RawToken>`). Not relocated here: its consumers include live `rules_tables` and the desktop `race_catalog`, so moving the file would relocate a hit without relocating the dependency. |
  | `derived_evaluator_fixture_check.rs` | 10 | 1 | 8 | 1 | Not relocated: the desktop `spell_catalog`/`companion_catalog`/`monster_catalog` import its parsers at run time, so it is a live formula reader, not a tool. 8 of its matches are ingest-shaped JSON test fixtures. |
  | `trait_pool.rs` | 10 | 5 | 4 | 1 | Live pool (`race_trait_picker`, `skinwalker_change_shape`); parses the token array into `TraitPoolRecord`. Needs a converted field. |
  | `pi_screening.rs` | 3 | 0 | 1 | 2 | A screened-field-list **string** (`"description,name,raw_tokens"`) plus two comments — the field name is data here, not a read. |
  | `race_creation.rs` | 2 | 1 | 1 | 0 | `resolved.raw_tokens.iter().filter(… "VISION")` — falls with `race_resolver`'s `ResolvedTrait`. |
  | `damage_total.rs` | 1 | 0 | 0 | 1 | The runnable re-derive command above. |
  | **total** | **110** | **38** | **36** | **36** | |

  Sums verified independently: `34+21+16+13+10+10+3+2+1 = 110` and `38+36+36 = 110`.
  Recorded as `deferral 1789074648477-at-35-e6-002-bd2612`, not as a silent gap.
- **Discoveries:** **one, and it changes how cycle 3 should be scoped.** Cycle 1 reported the
  remainder as "106 code lines" of live readers. Re-derived here with the comment/test/production
  split made explicit, the original 202 was **49 production reads, 65 test-code reads and 88
  doc-comment citations** — i.e. under half of the clause was ever live rules code, and a
  quarter of it was prose. That is not a correction to a published figure (cycle 1's 106 counted
  *lines*, and every line it counted is real), so no `correction` event is owed; it is a
  **category** finding, and the category is what a scope estimate depends on. Cycle 3's real job
  is the **38 production reads** in six files, of which `class_feature_pool_catalog.rs` (12) and
  `race_resolver.rs` (14) are two-thirds and both are entangled with `render_pcgen_desc`, which
  `epic-breakdown.md` assigns to `AT-35-E6-003`. **The two criteria should be sequenced together,
  not run past each other.**
- **Figures + their re-derive commands:** every row carries its own command. The unit denominator
  where one applies is the whole corpus, all books — **49,438** (`jq '.units | length' docs/work-inventory.json`).

  | figure | value | command | denominator |
  |---|---|---|---|
  | `raw_tokens` under `src/rules_core/`, files | **9** (from 33) | `grep -rl '\braw_tokens\b' --include=*.rs src/rules_core/ \| wc -l` | 198 live `src/rules_core` files the gate scans |
  | `raw_tokens` under `src/rules_core/`, matches | **110** (from 202) | `grep -rho '\braw_tokens\b' --include=*.rs src/rules_core/ \| wc -l` | as above |
  | of which production / test / comment | **38 / 36 / 36** | the classifier in this receipt's Notes (comment = a line whose first non-space is `//`, `/*` or `*`; test = a code line at or below the file's first `#[cfg(test)]`) | 110 matches |
  | live PCGen files / hits | **249 / 12,049** (from 252 / 12,170) | `python3 scripts/pcgen_residue_gate.py --check`, last line | 49,438 units |
  | gate's `raw_tokens` pattern, all live roots | **17 files / 142 hits** (from 41 / 234) | same command, first line | 5 live roots |
  | tool-side consumers of the relocated module | **8 files, 0 under a live root** | `grep -rl 'pcgen_import::corpus_literal_sweep' --include=*.rs src/ apps/ tests/` | 8 referencing files |
  | residual old import path | **0** | `grep -rn 'rules_core::corpus_literal_sweep' --include=*.rs src/ apps/ tests/ \| wc -l` | 8 referencing files |
  | accessor's own tests | **5 passed** | `cargo test --locked --lib -j 6 pcgen_import::ingest_record` | 5 unit tests |
  | `data/sheet_rules/` token leaks | **0** | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | 49,438 units |
  | sheet-rule package | ``records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS (111.8s)`` | `cargo run --locked --bin sheet_rule_convert -- --check` | 49,438 units |
  | atlas | `population=49438 buckets=10 unclassified=0 overlap=0 done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`; `DONE: 49438`, every other bucket `0` | `python3 scripts/completion_atlas.py --check` | 49,438 units |
  | token coverage | `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS` | `python3 scripts/token_coverage.py --check` | 49,438 units |
  | shape/engine boundary | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` | `python3 scripts/shape_engine_boundary.py --check` | 49,438 units |
  | missing engine tables | `population=0 kinds=0 citation_failures=0` | `python3 scripts/missing_engine_tables.py --check` | 49,438 units |
  | denominator gate | `files_checked=88 violations=0` (87 before this receipt landed); provenance `files_checked=205 figures_examined=435 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` | 87 bundle docs |
  | tool side intact — net deletions of function bodies | `**1,806 insertions, 4 deletions, 0 files removed**` | `git diff --numstat --find-renames 938e3f1b7b..HEAD -- src/pcgen_import scripts/oracle_harness src/oracle_validation`; `… --name-status … \| grep -c '^D'` | files in that window |
- **Build scope verified**, all at `62220d19a1`, target dir `/tmp/cargo-sd35-AT-35-E6-002`,
  `CARGO_INCREMENTAL=0`:
  - `cargo test --locked --no-run -j 6` → `**`NO_RUN_EXIT=0`**, **413 `Executable` lines** (`grep -c '^  Executable' /tmp/e6002c2-norun2.log`) — unchanged from cycle 1, as a relocation plus a new module inside an existing crate must leave it. `grep -cE '^(error|warning)'` → **0**.`
  - `cargo test --locked --lib -j 6` → `**`ok. 3266 passed; 0 failed; 15 ignored`** (42.82 s), `LIB_EXIT=0`. **+5 against cycle 1's 3,261**, and the five are named: `pcgen_import::ingest_record::tests::{reads_the_full_document_shape, reads_the_bare_data_object_shape, a_thin_record_with_no_token_array_is_empty_not_a_panic, an_absent_token_is_none_not_a_fabricated_empty_string, pairs_preserve_file_order}` (`grep 'ingest_record' /tmp/e6002c2-lib.log`). No pre-existing test changed its result.`
  - `cargo test --locked --no-fail-fast -j 6` → `**`FULL_EXIT=0`**, **414 `test result` lines, 8,777 passed, 0 failed, 68 ignored, ZERO failing suites** — `grep -cE '^test result: FAILED'` → **0**, `grep -cE '^(error|warning)'` → **0**. Totals derived with `awk` over the `test result` lines, not `grep -o` (`AGENTS.md` §Concurrency). Against cycle 1's 414 / 8,772 / 68 / 0 the **only** movement is the same **+5** accessor tests; the target count is unchanged at 414.`
  - `cargo clippy --locked --tests -j 6` → `**exit 0, 0 warnings, 0 errors** (1 m 41 s), in its own target dir `/tmp/cargo-sd35-AT-35-E6-002-clippy` so it did not contend on the workspace run's cargo lock.`
  - **Desktop crate: epic cadence.** This cycle touched **nothing** under `apps/` —
    `git diff --name-only 938e3f1b7b..HEAD -- apps/` prints nothing — so per `§6` step 3 the
    desktop crate and the frontend run at the epic wrap-up, not here.
  - `scripts/verify.sh --only pi-sweep` → `**`RESULT: PASS`** (1 stage).`
  - `cargo run --locked --bin v06_work_inventory`: **not run, deliberately.** This cycle changed
    no corpus record, so `corpus_literal_sweep` is guarded off (`§6` step 3), so the two reports
    the binary rebuilds its verification stamps from do not exist for this tree — it would refuse
    to write, and `--allow-stamp-loss` is forbidden. `git status --porcelain --
    docs/work-inventory.json` is empty and the `--receipt` rows above were computed against the
    file on disk.
- **Sweep population:** N/A — no corpus record changed (`git status --porcelain -- data/` empty
  at every checkpoint), so `corpus_literal_sweep` was correctly not run (`§6` step 3's guard).
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`, unchanged by this cycle).
- **Status:** **partial.** The criterion's body and its second Evidence clause were met in cycle
  1; its first Evidence clause is **not yet met** — **9 files, 110 gate matches** remain, down
  from 33 / 202. Deferred explicitly, not silently: `deferral 1789074648477-at-35-e6-002-bd2612`.
  Nothing here is a carve-out: 110 is a number to close, and it is enumerated file by file with
  the mechanism each one needs.
- **Notes:**
  - **One diagnostic label changed.** `simple_kind_tables::transcript_line` printed
    `… raw_tokens=<n>` and now prints `… ingest_tokens=<n>`; the number is the same count of the
    same array, read through the accessor. The only consumer is `src/bin/v06_work_inventory.rs`'s
    transcript. `docs/release/SD-34-book-completion/artifacts/epic-2-tables/fail-closed-proofs.md`
    quotes the **pre-rename** label in seven rows; that is a closed bundle's receipt recording
    what it observed at the time and is deliberately **not** edited here.
  - **The verify.sh event was misfiled again — third occurrence in three cycles.**
    `scripts/verify.sh --only pi-sweep` wrote its derived `verification` event into
    `docs/retro/events/sd31-transcribe.jsonl` (`1789077341442-sd31-transcribe-c70014`, head
    `938e3f1b7b`, `RESULT: PASS`) instead of this cycle's log, because `RETRO_ACTOR` does not
    survive into the `nohup`'d subshell this cycle ran the gate in. The append is **folded into
    this cycle's commit**, and re-logged as `incident 1789077403246-at-35-e6-002-959664`,
    recurrence key `retro-actor-lost-between-bash-calls`. Three occurrences is a missing
    mechanism, not bad luck (`AGENTS.md` §8): the instrument should derive its actor from the
    cycle's receipt path, not an environment variable. **Still unbuilt.**
  - **Three build errors self-healed inside the cycle, before any commit.** The first pass
    inserted the accessor's `use` into the wrong `#[cfg(test)]` module in two files with nested
    test modules (`domain_power.rs`, `weapon_tables.rs`), and one rewrite in
    `companion_chassis.rs` removed a binding a later line still used. All three surfaced on
    `cargo test --no-run`, were fixed, and the sequence was restarted from `--no-run` — §8's
    self-heal posture, not an escalation.
- **Next-cycle scope:** `AT-35-E6-002` **cycle 3**, sequenced **with** `AT-35-E6-003` rather than
  before it. Scope flags:
  `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design, decisions.md §2)`.
  Population is the table above: **9 files / 110 gate matches / 38 production reads**. Take the
  production reads first, in this order: `race_resolver.rs` (14) and
  `class_feature_pool_catalog.rs` (12) together with `render_pcgen_desc` — they are two-thirds of
  the production half and both resolve a `DESC` token — then `trait_pool.rs` (5),
  `shape_b_v1.rs` (5) and `race_creation.rs` (1). The 36 test-code and 36 comment matches follow
  their files' production halves and must not be swept ahead of them.
