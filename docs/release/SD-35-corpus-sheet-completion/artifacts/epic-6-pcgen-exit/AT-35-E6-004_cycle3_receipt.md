# Cycle 3 — Epic 6 (PCGen exit) / AT-35-E6-004

- **Commit SHA:** `PENDING_SHA` (cycle start `7913ffba49`), progress + kanban at `PENDING_DOCS_SHA`
- **Why a cycle 3 exists:** cycle 2 certified this criterion at `5da55c42e3`. **Fourteen commits
  landed on `tranche/15` after it**, and they are not bystanders — they changed files under the
  very live roots this criterion's gate reads and the very corpus its parity roster renders:
  `src/rules_core/feat_prereqs.rs`, `src/rules_core/trait_effects.rs`,
  `apps/desktop/src-tauri/src/{feat_catalog,reach_gate,class_feature_descriptions}.rs`,
  `src/bin/v06_work_inventory.rs` (+314), twelve `data/sheet_rules/` records and
  `data/sheet_rules/_tokens.json`. The largest, `e58e5a9ce5`, is **operator ruling B18**
  (`decisions.md §21`), which widened the shape filter and moved the inventory
  **49,438 → 49,450**. A closure certificate taken before its own live roots were edited is a
  certificate about a different tree. This cycle **re-runs every Evidence clause from scratch at
  the current HEAD**; nothing below is carried over from cycle 1's or cycle 2's receipt. It moves
  no unit and changes no shipping file.
- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 closure cycle — closes zero units by design,
  decisions.md §2)`. Run anyway for the record:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  `remaining_non_done=0` — the corpus stands at `DONE 49450 of 49450` after ruling B18
  (`kanban.md` row 104), not cycle 2's `49438 of 49438`.
- **Residue baseline at cycle start:** `python3 scripts/pcgen_residue_gate.py --check` →
  `live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736 verdict=PASS`, exit 0, taken
  **before** anything in this cycle was touched.
- **Files touched:** 4 added/edited, **0 outside the bundle package** except two derived files
  folded per the standing clean-tree rule (below). This receipt,
  `AT-35-E6-004_cycle3_sheet-parity-after.json`, the `progress.md` entry, the `kanban.md` row, and
  the retro event log. **No file under `src/`, `apps/` or `scripts/` changed** — this is a
  verification cycle, and it deliberately leaves the instrument it is auditing untouched.
  Folded derived files: `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
  (`derived_at` restamp `8f0206551c` → this HEAD, written by `completion_atlas.py --check`) and
  `docs/retro/events/{root,sd31-transcribe}.jsonl` (cron `reclaim.sh` appends). Neither is this
  cycle's work; both are folded rather than left dirty (`clean-tree-means-unfiltered-status`).
- **Identifier audit result:** OK_NO_BUNDLE_TAGS on this cycle's own diff.
  `git diff --unified=0 HEAD -- <this cycle's written paths> ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  → no match, before and after this cycle's writes.
- **Wired-integration audit result:** OK_NO_TOKENS. Same diff and exclusions,
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → no match.
  Run at the **epic-wide** base as well
  (`BASE_BRANCH=$(git merge-base HEAD origin/develop)` = `fe5ae6cd4a`, scoped paths
  `src/rules_core src/pcgen_import apps/desktop/src-tauri/src src/bin scripts/pcgen_residue_gate.py scripts/verify.sh <epic dir>`):
  both greps match **only prose** — `tests/sd27_feat_prerequisite_enforcement.rs`-style citations
  of test *filenames* in doc comments and receipts, and the word "placeholder" inside doc comments
  describing a *dropped* placeholder. **No identifier in shipping code carries a bundle tag and no
  shipping code path is a stub**, which is the property the two audits exist to assert.
- **Acceptance criterion** (verbatim, `epic-breakdown.md`):

  > ### AT-35-E6-004 — the gate reads zero
  >
  > **"Zero" means zero CODE hits** — operator ruling B14, 2026-09-11 (`decisions.md §17`). A
  > live-side doc comment that quotes an ingest-format token is provenance, not a read, and the
  > gate no longer counts one. Cited census, which is what forced the ruling:
  > `artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle1_receipt.md` and its
  > `…_cycle1_residue_shape_census.py` — `comment_hits=2686 code_hits=3628`,
  > `files_comment_only=114`, `max_files_clearable_by_code_work_alone=10`, which made the old
  > reading of this criterion unreachable by any code work. The gate's own `live_files=` /
  > `live_hits=` lines are now code-only counts; nothing else about this criterion changes, and
  > the resulting 197→81 drop is an instrument correction that closes nothing.
  >
  > **Evidence:** `python3 scripts/pcgen_residue_gate.py --check --closure` → `live_files=0
  > live_hits=0 verdict=PASS`, wired as the stage's closure mode from this cycle on. The oracle
  > comparison at the end of the epic agrees with the one at its start. `cargo tree` for the
  > desktop crate shows no dependency on the converter modules. **And the tool side is intact**
  > (`decisions.md §11`, what is kept): `cargo build --locked --bin sheet_rule_convert --bin
  > gen_book_cache` exits 0; `python3 scripts/oracle_harness/run.py --help` exits 0;
  > `scripts/pcgen-oracle-pin.env` unchanged; `git diff --stat <epic-6-start-sha>..HEAD --
  > src/pcgen_import scripts/oracle_harness src/oracle_validation` shows moves and additions,
  > **zero net deletions of function bodies** (a moved file is not a deleted one).

- **Receipt rows (mechanical):**
  ```
  since=7913ffba4913710f9643ce0bb99a92de627c6d4b target_dir=/tmp/cargo-sd35-AT-35-E6-004 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=1 pcgen_live_files=0
  ```

- **PCGen residue:** zero on every axis, in both modes, at HEAD.
  ```
  pattern raw_tokens files=0 hits=0            pattern %CHOICE files=0 hits=0
  pattern raw_bonus_chains files=0 hits=0      pattern %LIST files=0 hits=0
  pattern PcgenFormulaEvaluator files=0 hits=0 pattern TYPE= files=0 hits=0
  pattern render_pcgen_desc files=0 hits=0     pattern pcgen_import files=0 hits=0
  pattern bonus_stack_reader files=0 hits=0    root src/rules_core files=0 hits=0
  pattern pre_tokens files=0 hits=0            root src/saved_character files=0 hits=0
  pattern BONUS: files=0 hits=0                root src/campaign files=0 hits=0
  pattern DEFINE: files=0 hits=0               root src/homebrew_authoring files=0 hits=0
  pattern PRE[A-Z]+: files=0 hits=0            root apps/desktop files=0 hits=0
  pattern SAB: files=0 hits=0                  identifier_files=0 identifier_hits=0
  pattern DESC: files=0 hits=0                 shipped_data_files=0 shipped_data_hits=0 shipped_scanned=11
  live_files=0 live_hits=0 verdict=PASS
  ```
  `--check --closure` → exit 0. `--check` (ratchet) prints the same breakdown plus
  `baseline_files=260 baseline_hits=12736 verdict=PASS`, exit 0. The baseline file is untouched.

- **Did the gate earn its zero?** This criterion's job is to catch a green gate that does not
  measure what it claims, so the zero was taken on evidence, not on the gate's word. Six checks,
  all re-run at **this** HEAD, against a gate whose live roots were edited after cycle 2 certified
  them.

  1. **B14, B15 and B16 are implemented, not merely documented.** Read at
     `scripts/pcgen_residue_gate.py`: `_live_lines()` drops a line whose left-stripped form starts
     with `//` and scans every other line **whole**, so a trailing `// …` never shields the code
     before it (B14); `cfg_test_ranges()` computes the `#[cfg(test)]` **item** region with brace
     matching over `mask_non_code()` output — handling the braceless `;` case first so a
     `#[cfg(test)] use …;` cannot swallow the rest of the file — and `_live_lines()` drops it
     (B15); `RUNTIME_IMPORT_PATTERNS = {"pcgen_import": r"\bpcgen_import\b"}` is merged into
     `PATTERNS` and deliberately kept out of `IDENTIFIER_PATTERNS` (B16).
  2. **B16 actually reaches the totals, which is the claim worth doubting.** Being listed in
     `PATTERNS` is not the same as being counted. Traced in `scan()` at
     `scripts/pcgen_residue_gate.py:564-577`: the loop iterates `_COMPILED` — built from
     `PATTERNS`, so including `pcgen_import` — and every match adds into `file_hits`, which is what
     increments `res.live_files` / `res.live_hits` and the per-root counters. Only
     `IDENTIFIER_PATTERNS` matches are *additionally* split out into `identifier_hits`
     (`:570`). So a run-time `pcgen_import::` call site raises the closure bar's own numbers, from
     any of the five roots. Proved by execution in probe 1 below, not only by reading.
  3. **No live path is carved out.** `EXCLUDED_PREFIXES` is the empty tuple (`:205`) and
     `LIVE_ROOTS` is all five roots **including `apps/desktop`** (`:193-199`), which covers both
     `src-tauri/` and the `src/` frontend via `SOURCE_EXTENSIONS`
     `.rs .ts .tsx .js .jsx .mjs .cjs`. `python3 -m unittest scripts.tests.test_pcgen_residue_gate`
     → **Ran 39 tests, OK**.
  4. **An independent census, different algorithm, agrees at this HEAD.**
     `AT-35-E6-004_cycle1_independent_residue_census.py` re-reads the live side with a
     character-level Rust scanner that understands string literals, raw strings, char literals and
     block comments, borrowing nothing from the gate but its root list and patterns:
     ```
     gate_skipped_lines=90467 independent_skipped_lines=90467 files_with_boundary_mismatch=0
     INDEPENDENT live_files=0 live_hits=0
     over_skip_hits_hidden_by_gate=0
     lines_gate_counts_that_independent_calls_test=0
     ```
     `gate_skipped_lines` moved 90,434 (cycle 2) → 90,467, which is **33 lines of `#[cfg(test)]`
     code added by the fourteen intervening commits, not a boundary moving**: the independent
     scanner reports the identical 90,467 and `files_with_boundary_mismatch=0`.
  5. **The wiring fails on a real re-entry — four probes, four different failure shapes.** Each was
     planted, the gate run, and the probe removed; `git status --porcelain` was verified clean
     before and after the set.

     | probe | where | gate output | exit |
     |---|---|---|---|
     | a run-time converter call, `codex::pcgen_import::cache_gen::probe_marker()` | `apps/desktop/src-tauri/src/at35_e6_004_c3_probe.rs` (B16's root) | `pattern pcgen_import files=1 hits=1` / `root apps/desktop files=1 hits=1` / `live_files=1 live_hits=1 verdict=FAIL` | 1 |
     | an ingest token in a string literal, `"BONUS:COMBAT\|TOHIT\|1"` | `src/rules_core/at35_e6_004_c3_probe.rs` | `pattern BONUS: files=1 hits=1` / `root src/rules_core files=1 hits=1` / `live_files=1 live_hits=1 verdict=FAIL` | 1 |
     | **the same token planted inside the window the old B15 bug used to blank** — line 1900 of `apps/desktop/src-tauri/src/update/transaction.rs` | shipping code between two `#[cfg(test)]` modules | `pattern BONUS: files=1 hits=1` / `root apps/desktop files=1 hits=1` / `live_files=1 live_hits=1 verdict=FAIL` | 1 |
     | **NEW this cycle — the B17 shipped-data class, which no previous cycle probed:** `"raw_tokens": ["BONUS:COMBAT\|TOHIT\|1"]` added to a shipped resource | `apps/desktop/src-tauri/resources/corpus_fixtures/spell/spell_abjuration.json` | `shipped_data_files=1 shipped_data_hits=3` / `live_files=1 live_hits=3 verdict=FAIL` | 1 |
     | all four removed | — | `live_files=0 live_hits=0 verdict=PASS` | 0 |

     Probe 3 is the inherited one that matters. Cycle 1 fixed a line-level brace counter that let
     `b"not-json-{garbage"` run a `#[cfg(test)]` skip on past its module and blank 618 lines of
     shipping code. Re-derived at **this** HEAD, `cfg_test_ranges()` on that file returns the
     0-based `[(1061,1443), (1446,1498), (1501,1867), (2486,2884)]` over `total_lines=2885` — the
     third region still ends at 1867, not 2884 — and a token planted at line 1900 is **caught**.
     The fix is still in force and is proved by execution, not by reading the diff that made it.
     Probe 4 is this cycle's addition: ruling B17 folded `shipped_data_*` into the totals the
     closure bar reads, and cycle 2 certified that fold without ever making it fire. It fires.
  6. **What the gate does NOT measure, stated rather than assumed.** The B17 shipped-data class
     derives its population from `bundle.resources` in `apps/desktop/src-tauri/tauri.conf.json`
     (`shipped_resource_files()`, `:457`) — derived, never a hard-coded list, which is the right
     design — but that manifest lists only the authoring-workbench package and the corpus fixtures,
     so `shipped_scanned=11`. **`data/sheet_rules/` is not in it**, because it is not an installer
     resource: `corpus_loader::live_sheet_rules()` (`src/rules_core/corpus_loader.rs:350`) resolves
     it from `env!("CARGO_MANIFEST_DIR")` at run time. That surface is therefore **not** covered by
     the residue gate; it is covered by the criterion's own separate check,
     `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → **0**, re-run
     at this HEAD *after* B18 added twelve records and twelve `_tokens.json` rows. Naming the seam
     is the point: a reader must not take `live_files=0` as a statement about the converted corpus,
     which it is not.

- **Closure mode is the stage's mode.** `scripts/verify.sh:1480` —
  `if [[ "${PCGEN_RESIDUE_GATE_CLOSURE:-1}" == 1 ]]`. The default is `1`, so the
  `pcgen-residue-gate` stage runs `--check --closure`; the ratchet remains reachable as
  `PCGEN_RESIDUE_GATE_CLOSURE=0` as a diagnostic aid, never the shipping posture
  (`scripts/verify.sh:1468-1472`).

- **Oracle parity: re-run at HEAD, and it agrees with the epic's start.**
  `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`,
  unchanged since `c3500e7984` — `git diff --stat c3500e7984..HEAD -- scripts/pcgen-oracle-pin.env`
  is empty).
  ```
  sheet_parity: lines compared=156 agree=154 disagree=2 unverifiable=67;
                chassis compared=382 agree=376 disagree=6 unverifiable=140;
                characters=29 lines=505
  ```
  | | at AT-35-E6-001 (epic start) | at HEAD (this cycle) |
  |---|---|---|
  | pin | `7f818006e3` | `7f818006e3` |
  | roster | 29 characters | 29, identical list |
  | chassis compared/agree/disagree | 382 / 376 / 6 | **382 / 376 / 6** |
  | distinct disagreements | 6 | 7 |
  | `gone_since_start` | — | **0** |
  | `new_since_cycle2` | — | **0** |

  The parity document at HEAD is **byte-identical** to `AT-35-E6-004_cycle2_sheet-parity-after.json`
  (`head == cycle2 doc: True`), so the fourteen commits that landed after cycle 2 — **including
  ruling B18's shape-filter widening and its twelve new records** — moved not one rendered number
  on the roster. The one disagreement new *since the epic's start* is
  `half_elf_fighter_l1 target:Pool:favored_class ours=1 oracle=2`, named and attributed in
  `AT-35-E6-003-SWEEP_cycle13_receipt.md`, not this cycle's. Artifact:
  `AT-35-E6-004_cycle3_sheet-parity-after.json`.

- **The tool side is intact** (`decisions.md §11` — KEPT for Starfinder):
  ```
  cargo build --locked --bin sheet_rule_convert --bin gen_book_cache   -> TOOLSIDE_EXIT=0 (1m11s)
  cargo build --locked --release --bin sheet_rule_parity               -> PARITY_BUILD_EXIT=0 (1m53s)
  python3 scripts/oracle_harness/run.py --help                         -> ORACLE_HELP_EXIT=0
  git diff --stat c3500e7984..HEAD -- scripts/pcgen-oracle-pin.env     -> (empty; pin unchanged)
  git diff --stat c3500e7984..HEAD -- src/pcgen_import scripts/oracle_harness src/oracle_validation
                                                                       -> 64 files changed, 38468 insertions(+), 237 deletions(-)
  ```
  **Zero net deletions of function bodies**, proved by name rather than by diffstat — a diffstat
  cannot tell a move from a removal. Every `fn`/`def` name defined under `src/pcgen_import/**`,
  `scripts/oracle_harness/**` and `src/oracle_validation/**` at `c3500e7984` was re-derived at this
  HEAD:
  ```
  tool_side_files start=67 head=116
  fn/def definitions start=488 head=1453
  names_present_at_start_and_absent_at_HEAD=0
  ```
  The `tool_side_files` figure counts **every** file under the three prefixes (67 → 116), where
  cycle 2's scanner counted only the source-extension subset (39 → 88); the two are not comparable
  and are not being compared. The load-bearing figure is
  `names_present_at_start_and_absent_at_HEAD`, which is **0**, and the distinct-name counts
  (488 → 1,453) reproduce cycle 2's exactly.

- **`cargo tree` for the desktop crate — and what it can and cannot prove.** Run from
  `apps/desktop/src-tauri` (a separate workspace; the root sweep does not reach it, and
  `cargo tree -p codex-desktop` from the repo root exits 101):
  ```
  codex-desktop v0.15.0 (apps/desktop/src-tauri)
  ├── base64 v0.22.1        ├── sha2 v0.10.9              └── uuid v1.24.0
  ├── codex v0.1.0 (…)      ├── tauri v2.11.5             [build-dependencies]
  ├── serde v1.0.229        ├── tauri-plugin-dialog v2.7.2 └── tauri-build v2.6.3
  ├── serde_json v1.0.151   └── tauri-plugin-opener v2.5.4
  ```
  No converter crate is a dependency — but `src/pcgen_import/` is a **module of the `codex`
  crate**, not a crate of its own, and `codex` *is* a dependency. `cargo tree` is crate-grained and
  therefore **cannot** settle module-level independence; reporting it as if it could would be the
  same false-confidence move ruling B16 exists to prevent. The module-level claim is carried by
  probe 1 above, by `root apps/desktop files=0 hits=0`, and by `pattern pcgen_import files=0
  hits=0`.

- **Movement, four buckets:**
  - **closure (into DONE, by id-set):** none. Epic 6 closes no corpus unit; `closed=0`.
  - **relabel (bucket to bucket):** none. `regressed=0 added=0 dropped=0`;
    `docs/work-inventory.json` is byte-identical to `7913ffba49`'s.
  - **reachability:** none moved. The live side was already at zero when this cycle opened.
  - **instrument-correction:** none. This cycle changed no instrument — that is what makes it an
    audit. The `gate_skipped_lines` move 90,434 → 90,467 is 33 lines of `#[cfg(test)]` code added
    by intervening commits, corroborated by the independent census at the identical figure.
- **Refused tokens:** none. `token_coverage.py --check` → `refused=142 refused_non_done=0
  token_types=233 verdict=PASS`; every refused token sits on a unit already `DONE` under the sheet
  rule (it renders as words). Zero units were in this cycle's scope, so no `deferral` event is owed
  (`decisions.md §2`).
- **Discoveries:** two, both this cycle's own.
  1. **The B17 shipped-data class had never been made to fire.** Cycle 2 certified the fold of
     `shipped_data_*` into `live_files=`/`live_hits=` by reading the code. Probe 4 above executes
     it: a `"raw_tokens"` array planted in a shipped resource produces
     `shipped_data_files=1 shipped_data_hits=3 → live_files=1 verdict=FAIL`, exit 1. No defect —
     the wiring is real — but the check was owed and is now paid.
  2. **A backgrounded `cargo test --lib` on this shared checkout produced a false RED.** It linked
     **3,337** tests against the same HEAD's **3,406** and failed
     `pcgen_import::companion_pcgen_guards::tests::every_converted_guard_round_trips_from_the_live_typed_form`
     on a `crb` vs `core_rulebook` book slug that appears in **neither** side's source at
     `7913ffba49` — both `COMPANION_GUARD_TAILS`
     (`src/pcgen_import/companion_pcgen_guards.rs:70-72`) and `COMPANION_BOOKS`
     (`src/rules_core/rules_tables/companion_chassis.rs:732`) read `core_rulebook`. Re-run in the
     foreground on a verified-clean tree at the same HEAD: `test result: ok. 3390 passed; 0 failed;
     16 ignored`, and the single test in isolation `ok. 1 passed; 3405 filtered out`. A torn build,
     not a defect — and the reason this receipt reports the **foreground** figures. `incident
     1789416374947-at-35-e6-004-8c276e`. No `correction` event is owed: no figure published
     anywhere was wrong, the red never left this cycle.
- **Figures + their re-derive commands:**

  | figure | value | command | denominator |
  |---|---|---|---|
  | live PCGen surface, closure mode | `live_files=0 live_hits=0 verdict=PASS` | `python3 scripts/pcgen_residue_gate.py --check --closure` | the 5 live roots, `.rs .ts .tsx .js .jsx .mjs .cjs`, comments (B14) and `#[cfg(test)]` regions (B15) excluded, run-time `pcgen_import` imports (B16) and the 11 manifest-derived shipped files (B17) included |
  | live PCGen surface, ratchet mode | `live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736 verdict=PASS` | `python3 scripts/pcgen_residue_gate.py --check` | same population, measured against `scripts/pcgen-residue-baseline.env` |
  | independent census | `live_files=0 live_hits=0`, `files_with_boundary_mismatch=0` | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-004_cycle1_independent_residue_census.py` | same population, different algorithm |
  | lines both censuses skip as `#[cfg(test)]` | gate `90467`, independent `90467` | the same census run's last line | the live files carrying a `#[cfg(test)]` item |
  | `#[cfg(test)]` regions in `update/transaction.rs` | 0-based `[(1061,1443),(1446,1498),(1501,1867),(2486,2884)]` | `python3 -c "import sys;sys.path.insert(0,'scripts');import pcgen_residue_gate as g;print(g.cfg_test_ranges(open('apps/desktop/src-tauri/src/update/transaction.rs').read().splitlines()))"` | `total_lines=2885` in that file |
  | shipped files the B17 class scans | **11** | `python3 -c "import sys;sys.path.insert(0,'scripts');import pcgen_residue_gate as g;print(len(g.shipped_resource_files('.')))"` | `bundle.resources` in `apps/desktop/src-tauri/tauri.conf.json`, walked recursively |
  | gate unit tests | Ran **39** tests, OK | `python3 -m unittest scripts.tests.test_pcgen_residue_gate` | the gate's own suite |
  | ingest tokens shipped in `data/sheet_rules/` | **0** | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | every converted rule file, after ruling B18's twelve additions |
  | tool-side function bodies lost | **0** (488 → 1,453 distinct names, 67 → 116 files) | the `fn`/`def` name census quoted above, `git ls-tree` at `c3500e7984` vs `HEAD` | `src/pcgen_import` + `scripts/oracle_harness` + `src/oracle_validation` |
  | oracle parity at HEAD | 156/154/2 lines, 382/376/6 chassis, 29 characters, 505 lines | `sheet_rule_parity --roster artifacts/epic-2-sheet-rule/oracle-parity/roster --output <ours>` then `python3 scripts/oracle_harness/sheet_parity.py compare --ours <ours> --exports artifacts/epic-2-sheet-rule/oracle-parity/exports --output <out>` | the 29-character fixture roster, pin `7f818006e3` |
  | parity disagreements cleared or created since the epic start | `gone_since_start=0`, `new_since_cycle2=0` | `python3` set-difference over the three parity JSONs (start / cycle 2 after / this cycle) | the 505 rendered lines across the 29-character roster |
  | converter `--check` | `CONVERT_CHECK_EXIT=0` | `cargo run --locked --bin sheet_rule_convert -- --check` | `data/sheet_rules/` |
  | lib suite | `3390 passed; 0 failed; 16 ignored` | `cargo test --locked --lib -j 6` (foreground) | the `codex` lib's 3,406 unit tests |
  | full workspace suite | **420 suites, 8,926 passed, 0 failed, 69 ignored** | `cargo test --locked --no-fail-fast -j 6` | every root-workspace test target |
  | desktop crate suite | **570 passed, 0 failed** | `cargo test --locked -j 4` from `apps/desktop/src-tauri` | the separate `codex-desktop` workspace |
  | test executables linked | **419** | `cargo test --locked --no-run -j 6` | the root workspace |
  | denominator gate | `files_checked=166 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` | the SD-35 bundle's markdown only |
  | figure provenance | `figures_examined=496 violations=0` | `python3 scripts/denominator_gate.py --check-provenance '<same two globs>'` | same 166 files |

- **Build scope verified:** **no Rust source changed this cycle** (`rust_lines_changed=0`), so the
  compiled surface is `7913ffba49`'s — re-proved rather than assumed, on a `CARGO_TARGET_DIR`
  scoped to this cycle. `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`, **419 test
  executables** linked; `cargo test --locked --lib -j 6` → `LIB_EXIT=0`,
  `test result: ok. 3390 passed; 0 failed; 16 ignored` (45.00 s).
  `cargo test --locked --no-fail-fast -j 6` → `FULL_EXIT=0`, **420 suites, 8,926 passed, 0 failed,
  69 ignored**, `test result: FAILED` count **0**. Its stated trigger ("`src/` or the classifier
  changed") is not met by *this* cycle, but it is met by the fourteen commits this cycle certifies,
  so it was run rather than inherited. **The desktop crate was run for the same reason** — it is a
  separate workspace the root sweep does not reach, and `apps/desktop/src-tauri/src/{feat_catalog,
  reach_gate,class_feature_descriptions}.rs` changed after cycle 2: from `apps/desktop/src-tauri`,
  `cargo test --locked -j 4` → `DESKTOP_EXIT=0`, `test result: ok. 570 passed; 0 failed; 0 ignored`
  (1486.45 s). `cargo run --locked --bin sheet_rule_convert -- --check` → `CONVERT_CHECK_EXIT=0`
  (`kind spell: records=2845 converted=2845 refused=0`, `template 2248/2248`, `trait 487/487`).
  `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → **0**.
  `cargo clippy` is not run: its input this cycle is four markdown/JSON files and no Rust target.
  Gates run at HEAD: `completion_atlas.py --check` → `done_evidence_violations=0
  missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`;
  `token_coverage.py --check` → `verdict=PASS`; `shape_engine_boundary.py --check` →
  `magnitude_bearing=26397 not_held_by_engine=0 citation_ok=True`;
  `missing_engine_tables.py --check` → `population=0 kinds=0 citation_failures=0`;
  `denominator_gate.py --check` → `files_checked=166 violations=0`;
  `scripts/verify.sh --only pi-sweep` → `RESULT: PASS`.
- **Sweep population:** N/A — no corpus record changed this cycle, so `corpus_literal_sweep` is not
  re-run.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, unchanged since
  `c3500e7984`.
- **Status:** complete.
- **Notes:** one judgment call, inherited and re-affirmed: `cargo tree` is reported for what it can
  settle (no converter **crate** in the desktop dependency graph) rather than for what the Evidence
  sentence reads as (no converter **module**), because `pcgen_import` is a module of the `codex`
  crate and no crate-grained tool can separate the two. The module claim rests on the probes and
  the gate instead.
- **Next-cycle scope:** criterion at zero. Epic 6 is `complete`; the closure mode is the
  `pcgen-residue-gate` stage's default, so Epic 7 re-runs it at HEAD as one of its own gates
  without needing a flag. **A future cycle that edits a live root re-opens this certificate** — the
  gate is a standing stage, not a one-time proof, which is why this cycle exists at all.
