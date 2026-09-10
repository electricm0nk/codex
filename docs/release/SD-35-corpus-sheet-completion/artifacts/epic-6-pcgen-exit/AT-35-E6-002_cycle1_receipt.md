# Cycle 1 — Epic 6 (PCGen exit) / AT-35-E6-002

This cycle delivers the criterion's **body** — the relocation — and **one of its two Evidence
clauses**. `gen_book_cache` output is byte-identical before and after; the `raw_tokens` clause is
**not** met and is not gameable by a comment sweep, so the status is **partial** with the
remainder enumerated file-by-file below.

- **Commit SHA:** `b91d16a66b` — the relocation, the 63 rewritten import sites, the emptied
  residue-gate carve-out and this cycle's deferral event. A second commit carries this receipt,
  the `progress.md` / `kanban.md` rows and the two folded working-tree files, and pins that SHA
  into this line (a receipt cannot name the commit that carries it). Cycle start `c03e35f8c0`.
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
- **Files touched:** **78** (`git show --stat b91d16a66b | tail -1`).
  - **Relocated, 17 files, `git mv` (renames, not rewrites):** `src/rules_core/cache_gen/` → 
    `src/pcgen_import/cache_gen/` (16 files: `mod.rs`, `acg.rs`, `apg.rs`, `beastiary1.rs`,
    `class_feature.rs`, `class_feature_grants.rs`, `enrich_raw_tokens_shared.rs`,
    `equipment_copy_citation_repair.rs`, `equipment_gap.rs`, `feat_gap.rs`,
    `hand_authored_equipment.rs`, `hand_authored_feat_dump.rs`, `lst_provenance_repair.rs`,
    `spell_lane_dump.rs`, `spell_mod_access.rs`, `ultimate_equipment.rs`) and
    `src/rules_core/wiring_class.rs` → `src/pcgen_import/wiring_class.rs`.
  - **Module declarations, 2 files:** `src/rules_core/mod.rs` (two `pub mod` lines removed),
    `src/pcgen_import/mod.rs` (both added, with the `decisions.md §11` citation).
  - **Import-path rewrites, 63 files / 102 occurrences:** every `src/bin/gen_*`, `enrich_*`,
    `ingest_*`, `repair_*`, `restamp_wiring_class.rs`, `corpus_literal_sweep.rs`,
    `v06_work_inventory.rs`; `src/rules_core/{corpus_literal_sweep,shape_b_v1}.rs`,
    `rules_tables/{crb/json_cache,feats_all}.rs`; `src/pcgen_import/{corpus_traps,
    sheet_rule/prose}.rs`; 10 files under `tests/`; 2 desktop-crate doc citations.
  - **Stale path literals, 6 files:** `tests/generator_name_key_screening_static_audit.rs`
    (which `stat`s the real directory — a functional break if left), and doc/comment path
    citations in `scripts/{ground_truth_evidence_guard,derive_spell_range_fixtures,
    derive_derived_evaluator_fixtures,sd32-t2a-residual-census,observer/pi_redaction}.py`.
  - **Instrument, 2 files:** `scripts/pcgen_residue_gate.py` and
    `scripts/tests/test_pcgen_residue_gate.py` — see **Notes**.
  - **Retro:** `docs/retro/events/at-35-e6-002.jsonl`; folded working-tree appends
    `docs/retro/events/root.jsonl` (a `reclaim.sh` note) and, in this receipt's commit,
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (the
    `derived_at` re-stamp `completion_atlas.py --check` writes).
  - **Zero `data/` files changed.** `git status --porcelain -- data/` is empty; the two
    `gen_book_cache` runs' `LICENSE.json` writes were restored with `git checkout --` after each.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS in shipping code — **1 match, attributed, not a
  violation, and not this cycle's.**
  ```
  BASE_BRANCH=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47   # git merge-base HEAD origin/develop
  CODE="src/rules_core src/pcgen_import src/bin src/oracle_validation apps/desktop/src-tauri/src"
  git diff --unified=0 "${BASE_BRANCH}...HEAD" -- $CODE ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  ->  15482:+/// (`tests/sd34_wave51_racial_sla_catalog_matches_the_corpus.rs`); this function is its
  ```
  A doc-comment citation of a real test file's path in `src/rules_core/racial_sla.rs`, pre-existing
  and unchanged here — the same single match and the same disposition every `AT-35-E5-*` and
  `AT-35-E6-001` receipt recorded. **This cycle contributes 0.**
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
  | `cache_gen/**` relocated to `src/pcgen_import/` | 16 files, `git mv` renames | **yes** |
  | `wiring_class.rs` relocated | `src/pcgen_import/wiring_class.rs` | **yes** |
  | behaviour-identical | lib 3261 / full 8,772 passed, both identical to the pre-move tree | **yes** |
  | every `src/bin` generator's import path follows | 63 files, 102 occurrences, 0 residual | **yes** |
  | `gen_book_cache` byte-identical on one book | 2,207 records, same manifest sha256 | **yes** |
  | zero `raw_tokens` hits under `src/rules_core/` | **33 files, 202 matches** | **no** |

  `ls src/rules_core/ | grep -E 'cache_gen|wiring_class' | wc -l` → **0**.
  `grep -rn 'rules_core::cache_gen\|rules_core::wiring_class' --include=*.rs src/ apps/ tests/ | wc -l`
  → **0**.
- **Receipt rows (mechanical):**
  ```
  since=c03e35f8c0a93e60f233051a2432fb3b34728ddd target_dir=/tmp/cargo-sd35-AT-35-E6-002 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=223 ratio=n/a builds_recorded=1 pcgen_live_files=252
  ```
  `closed=0` / `relabeled=0` is correct: the unit population was already `0` non-DONE at cycle
  start. `ratio` is `n/a`, a division by zero, never `0.0`. `rust_lines_changed=223` is low for a
  78-file commit because git scores the 17 relocations as renames (0 changed lines) and the
  remaining edits are one-line import rewrites.
- **PCGen residue** (`python3 scripts/pcgen_residue_gate.py --check`, at `b91d16a66b`):
  ```
  pattern raw_tokens files=41 hits=234
  pattern PcgenFormulaEvaluator files=0 hits=0
  pattern render_pcgen_desc files=17 hits=109
  pattern bonus_stack_reader files=0 hits=0
  pattern pre_tokens files=0 hits=0
  pattern BONUS: files=139 hits=2226
  pattern DEFINE: files=26 hits=116
  pattern PRE[A-Z]+: files=120 hits=7960
  pattern SAB: files=0 hits=0
  pattern DESC: files=138 hits=576
  pattern %CHOICE files=8 hits=66
  pattern %LIST files=25 hits=141
  pattern TYPE= files=69 hits=742
  root src/rules_core files=201 hits=11693
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=51 hits=477
  identifier_files=54 identifier_hits=343
  live_files=252 live_hits=12170 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  **It went down on both axes and up on neither** — `253 → 252` files, `12,256 → 12,170` hits
  against `AT-35-E6-001` cycle 5's figure. That fall is `wiring_class.rs` alone (1 file, 86
  hits); `cache_gen/**` was already carved out of the live count and so contributed nothing to
  the drop, which is exactly why this cycle also **emptied the carve-out** rather than leaving a
  now-dead exclusion for `AT-35-E7-001` to find. `raw_tokens` is flat at `41 / 234` for the same
  reason: `wiring_class.rs` never read one. The baseline file is deliberately NOT rebaselined;
  `--rebaseline` is `AT-35-E6-004`'s step.
- **Oracle parity:** **N/A for a re-run, and correct by construction.** Epic 6 touched a live
  path, so the row is owed an answer — this cycle's answer is that the live path's *behaviour* is
  provably unchanged, which is a stronger claim than agreement within tolerance: the relocation
  changed **zero function bodies**, and the two suites reproduce the pre-move tree's figures
  exactly (lib `3261 passed / 0 failed / 15 ignored`; workspace `414 targets / 8,772 passed /
  0 failed / 68 ignored` — every figure identical to `AT-35-E6-001` cycle 5's on the pre-move
  tree). The converter-side generator was re-run for real and its output compared byte-for-byte
  (below). `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, unchanged. The epic's
  PCGen-oracle before/after comparison belongs to `AT-35-E6-004`.
- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** 0 — the population was already 0 non-DONE at cycle start.
  - **relabel (bucket to bucket):** 0. No unit changed bucket.
  - **reachability:** 0. This cycle evaluated nothing and converted nothing.
  - **instrument-correction:** **1, and it is a tightening, not a fix.**
    `pcgen_residue_gate.py`'s `EXCLUDED_PREFIXES` went from `("src/rules_core/cache_gen/",)` to
    `()`. No figure it ever printed was wrong; the carve-out simply has nothing left to exclude
    now that the directory sits on the tool side. See **Notes**.
- **Refused tokens:** **none.** This cycle added no converter refusal and cleared none; the
  refused set is unchanged at 142 records, one shape (`no_corpus_record`), `refused_non_done=0`
  (`token_coverage.py --check`). It shipped no converter mapping row at all.
- **Discoveries:** **one, and it is the reason this receipt says `partial`.** The criterion's
  Evidence sentence reads as if relocating `cache_gen/**` were what clears `raw_tokens` from
  `src/rules_core/`. It is not, and cannot be: the gate has excluded
  `src/rules_core/cache_gen/` from the live count since `AT-35-E1-005`, so the `raw_tokens`
  figure never contained a single `cache_gen` hit. The 33 files it *does* contain are ordinary
  live rules_core modules that read a corpus record's `raw_tokens` at run time. Recorded as
  `deferral 1789071123321-at-35-e6-002-6b9285` with the full census, not as a silent gap.
  No `correction` event is owed: no previously published figure was wrong.

  **The remainder, enumerated** (`grep -rl '\braw_tokens\b' --include=*.rs src/rules_core/`;
  per-file totals from `grep -o … | wc -l`, code lines = matches on a line that does not begin
  with `//`, `/*` or `*`). Denominator: **33 files / 202 matches / 106 code lines**, the whole of
  `src/rules_core/` at `b91d16a66b`.

  | file (under `src/rules_core/`) | gate matches | of which code |
  |---|---|---|
  | `corpus_literal_sweep.rs` | 49 | 12 |
  | `class_feature_pool_catalog.rs` | 34 | 27 |
  | `race_resolver.rs` | 21 | 13 |
  | `corpus_loader.rs` | 19 | 4 |
  | `shape_b_v1.rs` | 13 | 8 |
  | `trait_pool.rs` | 10 | 9 |
  | `derived_evaluator_fixture_check.rs` | 10 | 9 |
  | `rules_tables/pathfinder_unchained/rogue_features.rs` | 4 | 4 |
  | `rules_tables/crb/class_skill_tables.rs` | 4 | 4 |
  | `pilot_compute/domain_power.rs` | 4 | 4 |
  | `rules_tables/crb/wizard_spell_list.rs` | 4 | 2 |
  | `pi_screening.rs` | 3 | 1 |
  | `rules_tables/simple_kind_tables.rs` | 2 | 2 |
  | `rules_tables/pathfinder_unchained/barbarian_features.rs` | 2 | 2 |
  | `rules_tables/crb/weapon_tables.rs` | 2 | 2 |
  | `race_creation.rs` | 2 | 2 |
  | `rules_tables/companion_chassis.rs` | 2 | 1 |
  | `pilot_compute/mod.rs` | 2 | 0 |
  | 15 further files, 1 match each (`trait_effects.rs`, `racial_sla.rs`, `money.rs`, `equipment_effects.rs`, `encumbrance.rs`, `damage_total.rs`, `pilot_compute/crb_untabled_class_chassis.rs`, `rules_tables/apg/antipaladin_features.rs`, 7 × `rules_tables/ultimate_psionics/*_features.rs`) | 15 | 0 |
  | **total** | **202** | **106** |

  Sums verified independently: `49+34+21+19+13+10+10+4+4+4+4+3+2·6+1·15 = 202`, and the
  `awk`-derived line count (191 lines carrying ≥1 match) is consistent with 202 matches.
- **Figures + their re-derive commands:** every row carries its own command. The unit denominator
  where one applies is the whole corpus, all books — **49,438** (`jq '.units | length' docs/work-inventory.json`).

  | figure | value | command | denominator |
  |---|---|---|---|
  | `gen_book_cache` record files, before ≡ after | **2,207 files, sha256 `c69500e001442c8d787cecba1decce0a39841a9bb7f6d70ecaeb1c06948715dc` on both runs** | `find data/corpus/advanced_race_guide -name '*.json' ! -name LICENSE.json -not -path '*/_parity/*' \| sort \| xargs sha256sum \| sha256sum`, run after a `gen_book_cache advanced_race_guide` on each tree | 2,207 ARG records |
  | `gen_book_cache` `LICENSE.json`, before ≡ after | **sha256 `22e3676763a116c202983d2e5bd64e9ad04a773a2167720fd4faee4d640b544a` on both runs**, after normalizing the two `…T..:..:..Z` run timestamps | `python3 -c "import re;print(re.sub(r'\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z','<TS>',open('data/corpus/advanced_race_guide/LICENSE.json').read()))" \| sha256sum` | 1 file |
  | files relocated | **17** | `git show --name-status b91d16a66b \| grep -c '^R'` | 17 moved modules |
  | import sites rewritten | **63 files / 102 occurrences** | the rewrite pass's own count; re-derive the residual with `grep -rn 'rules_core::cache_gen\|rules_core::wiring_class' --include=*.rs src/ apps/ tests/ \| wc -l` → **0** | 63 referencing files |
  | live PCGen files / hits | **252 / 12,170** (from 253 / 12,256) | `python3 scripts/pcgen_residue_gate.py --check`, last line | 49,438 units |
  | `raw_tokens` under `src/rules_core/` | **33 files / 202 matches / 106 code lines** | `grep -rl '\braw_tokens\b' --include=*.rs src/rules_core/ \| wc -l`; `grep -rho '\braw_tokens\b' --include=*.rs src/rules_core/ \| wc -l` | 201 live `src/rules_core` files the gate scans |
  | live paths exempt from the gate | **0** (was 1) | `python3 -c "import sys;sys.path.insert(0,'scripts');import pcgen_residue_gate as p;print(list(p.EXCLUDED_PREFIXES))"` | 5 live roots |
  | `data/sheet_rules/` token leaks | **0** | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | 49,438 units |
  | sheet-rule package | `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS (20.1s)` | `cargo run --locked --release --bin sheet_rule_convert -- --check` | 49,438 units |
  | atlas | `population=49438 buckets=10 unclassified=0 overlap=0 done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`; `DONE: 49438`, every other bucket `0` | `python3 scripts/completion_atlas.py --check` | 49,438 units |
  | token coverage | `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS` | `python3 scripts/token_coverage.py --check` | 49,438 units |
  | shape/engine boundary | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` | `python3 scripts/shape_engine_boundary.py --check` | 49,438 units |
  | missing engine tables | `population=0 kinds=0 citation_failures=0` | `python3 scripts/missing_engine_tables.py --check` | 49,438 units |
  | denominator gate | `files_checked=87 violations=0` (86 before this receipt landed) | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` | 86 bundle docs |
  | figure provenance | `files_checked=204 figures_examined=426 violations=0` (`203 / 416` before this receipt landed) | `python3 scripts/denominator_gate.py --check-provenance` | 416 figures |
  | tool side intact — net deletions of function bodies | **17,271 insertions, 4 deletions, 0 files removed** | `git diff --numstat --find-renames c03e35f8c0..HEAD -- src/pcgen_import scripts/oracle_harness src/oracle_validation`; `… --name-status … \| grep -c '^D'` | 20 files in that window |
  | kept tools still build / run | `Finished dev profile`; `ORACLE_HARNESS_HELP_EXIT=0`; pin file unchanged | `cargo build --locked --bin sheet_rule_convert --bin gen_book_cache`; `python3 scripts/oracle_harness/run.py --help`; `git diff --stat HEAD -- scripts/pcgen-oracle-pin.env \| wc -l` → 0 | 2 bins + 1 harness |
- **Build scope verified**, all at `b91d16a66b` (the relocation commit — the tree this receipt's
  own commit does not change in any compiled input), target dir `/tmp/cargo-sd35-AT-35-E6-002`,
  `CARGO_INCREMENTAL=0`:
  - `cargo test --locked --no-run -j 6` → **`NO_RUN_EXIT=0`**, **413 `Executable` lines**
    (`grep -c '^  Executable' /tmp/e6002-norun.log`) — unchanged from cycle 5, as a pure
    relocation must leave it.
  - `cargo test --locked --lib -j 6` → **`ok. 3261 passed; 0 failed; 15 ignored`** (39.77 s),
    `LIB_EXIT=0` — identical to the pre-move figure.
  - `cargo test --locked --no-fail-fast -j 6` → **`FULL_EXIT=0`**, **414 `test result` lines,
    8,772 passed, 0 failed, 68 ignored, ZERO failing suites** —
    `grep -cE '^test result: FAILED' /tmp/e6002-full.log` → **0**, and
    `grep -cE '^(error|warning)' /tmp/e6002-full.log` → **0**. Totals derived with `awk` over the
    `test result` lines, not `grep -o` (`AGENTS.md` §Concurrency). Identical to cycle 5's
    414 / 8,772 / 68 / 0 on the pre-move tree — the behaviour-identity proof.
  - **Desktop crate: run, not deferred to epic cadence** — this cycle touched two files under
    `apps/`. `cd apps/desktop/src-tauri && cargo test --locked --no-fail-fast -j 4` →
    **`DESKTOP_EXIT=0`, 576 passed, 0 failed, 0 ignored**, 0 FAILED suites, in its own target dir
    `/tmp/cargo-sd35-AT-35-E6-002-desktop` (a separate workspace — the root sweep does not reach
    it). The frontend is untouched — `git show --stat b91d16a66b -- apps/desktop/src` prints
    **nothing** — and runs at the epic wrap-up.
  - `cargo clippy --locked --tests -j 4` → **exit 0, 0 warnings, 0 errors** (1 m 31 s), in its own
    target dir `/tmp/cargo-sd35-AT-35-E6-002-clippy` so it did not contend on the workspace run's
    cargo lock.
  - `python3 scripts/tests/test_pcgen_residue_gate.py` → **`Ran 15 tests … OK`** — the gate's own
    suite, re-run after the carve-out was emptied and its pin inverted.
  - `scripts/verify.sh --only pi-sweep` → **`RESULT: PASS`** (1 stage; 11 hits over
    `src/rules_core/rules_tables`, 11 baseline rows).
  - `cargo run --locked --bin v06_work_inventory`: **not run, deliberately.** This cycle changed
    no corpus record, so `corpus_literal_sweep` is guarded off (`§6` step 3), so the two reports
    the binary rebuilds its verification stamps from do not exist for this tree — it would refuse
    to write, and `--allow-stamp-loss` is forbidden. `git status --porcelain --
    docs/work-inventory.json` is empty and the `--receipt` rows above were computed against the
    file on disk.
- **Sweep population:** N/A — no corpus record changed (`git status --porcelain -- data/` empty
  at every checkpoint), so `corpus_literal_sweep` was correctly not run (`§6` step 3's guard).
  The one `data/` write either `gen_book_cache` run made (`LICENSE.json`) was restored with
  `git checkout --` immediately after its hash was taken.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`, unchanged by this cycle).
- **Status:** **partial.** The criterion's body and its second Evidence clause are met and proven;
  its first Evidence clause (`zero raw_tokens hits under src/rules_core/`) is not, with the
  remainder enumerated above — **33 files, 202 gate matches, 106 code lines**. Deferred
  explicitly, not silently: `deferral 1789071123321-at-35-e6-002-6b9285`.
  **Refused tokens: none** — the remainder is live-side reader code, not a converter refusal, so
  it does not fall under §8's "more than 10 distinct refused token types" escalation and is not a
  blocked condition. Nothing here is a carve-out: 33 is a number to close, and cycle 2 closes it.
- **Notes:**
  - **The gate's last carve-out is gone.** `EXCLUDED_PREFIXES` held exactly one entry,
    `src/rules_core/cache_gen/`, and this cycle moved that directory to the tool side — where the
    gate never scans anyway. Leaving the entry behind would have been a hardcoded exclusion list
    aimed at a path that no longer exists, and `AT-35-E7-001` greps the closure instruments for
    precisely that. It is now `()`, and `test_pcgen_residue_gate.py`'s pin was inverted from
    "only cache_gen is carved out" to "no live path is carved out" so one cannot be re-added
    quietly (`acceptance-and-verification.md` §3a). The change **tightens** the gate; the count
    is unaffected, because the excluded directory is no longer under a live root.
  - **Why the byte-identity proof compares generated-to-generated, not generated-to-committed.**
    `gen_book_cache` rewrites `LICENSE.json`'s `classified_at` on every run and, on this book,
    also rewrites a reconciled `screening_method_note` and `records_redacted` (11 → 10) that a
    later hand-reconciliation had put there — the known "regenerating corpus destroys license/PI
    fields" hazard. Comparing a fresh run against the committed file would therefore report a
    difference that has nothing to do with the relocation. Both runs were made and hashed; the
    committed file was restored after each; `data/` ends clean.
- **Next-cycle scope:** `AT-35-E6-002` **cycle 2** — clear `raw_tokens` from `src/rules_core/`.
  Scope flags: `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero units by design, decisions.md §2)`.
  Population is the table above: **33 files / 202 gate matches / 106 code lines**, with 7 files
  carrying 8 or more code reads (`class_feature_pool_catalog.rs` 27, `race_resolver.rs` 13,
  `corpus_literal_sweep.rs` 12, `trait_pool.rs` 9, `derived_evaluator_fixture_check.rs` 9,
  `shape_b_v1.rs` 8, `corpus_loader.rs` 4) and 15 carrying comment citations only. Two shapes to
  separate first: readers that must be re-pointed at `SheetRule.applies` / `prose`, and
  converter-or-instrument modules (`corpus_literal_sweep.rs`,
  `derived_evaluator_fixture_check.rs`) that may belong on the tool side for the same reason
  `cache_gen/**` did — decide that before rewriting either. `render_pcgen_desc` (17 files, 109
  hits) stays `AT-35-E6-003`'s.
