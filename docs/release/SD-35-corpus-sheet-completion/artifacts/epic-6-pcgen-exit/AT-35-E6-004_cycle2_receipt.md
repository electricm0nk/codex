# Cycle 2 — Epic 6 (PCGen exit) / AT-35-E6-004

- **Commit SHA:** `5da55c42e3` (cycle start `7fadc67843`), progress + kanban at `af4e315b84`
- **Why a cycle 2 exists:** cycle 1 certified this criterion at `77e8d3919a`. Three commits
  landed on `tranche/15` after it — `fdc90243f4`, `2f824171b5` and `7fadc67843`
  (`AT-35-E6-005-SHIPPED-DATA`, ruling **B17**, which *widened the gate* to scan shipped installer
  data and folded `shipped_data_files`/`shipped_data_hits` into the very `live_files=`/`live_hits=`
  totals this criterion reads). A closure certificate taken before its own gate was widened is
  a certificate about a different instrument. This cycle re-certifies at the current HEAD and
  **re-runs every Evidence clause from scratch**; nothing below is carried over from cycle 1's
  receipt. It moves no unit and changes no shipping file.
- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 closure cycle — closes zero units by design,
  decisions.md §2)`. Run anyway for the record:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  `remaining_non_done=0` — the corpus reached `DONE 49438 of 49438` at `AT-35-E5-005`.
- **Files touched:** 4 added, 0 edited outside the bundle package. This receipt,
  `AT-35-E6-004_cycle2_sheet-parity-after.json`, the `progress.md` entry, the `kanban.md` row, and
  the retro event log. **No file under `src/`, `apps/`, `scripts/` or `data/` changed** — this is a
  verification cycle, and it deliberately leaves the instrument it is auditing untouched.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS.
  `git diff --unified=0 HEAD -- scripts/pcgen_residue_gate.py scripts/tests/test_pcgen_residue_gate.py scripts/verify.sh docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  → no match, before and after this cycle's writes.
- **Wired-integration audit result:** OK_NO_TOKENS. Same diff and same exclusions,
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → no match.
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
  since=7fadc67843433ceb899ec42025b99fa0e2faf491 target_dir=/tmp/cargo-sd35-AT-35-E6-004 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=1 pcgen_live_files=0
  ```
  `builds_recorded=1` only when `CARGO_TARGET_DIR` is exported into the receipt command; without
  it the script reads whatever scratch target dir the ambient environment names (on this shared
  checkout, another session's) and prints `builds_recorded=0`. Both runs were taken; the row above
  is the one whose denominator is **this cycle's** target dir.

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
  measure what it claims, so the zero was taken on evidence, not on the gate's word. Five checks,
  all re-run at this HEAD:

  1. **B14, B15 and B16 are implemented, not merely documented.** Read at
     `scripts/pcgen_residue_gate.py`: `code_only()` drops a line whose left-stripped form starts
     with `//` and scans every other line **whole**, so a trailing `// …` never shields the code
     before it (B14); `cfg_test_ranges()` computes the `#[cfg(test)]` **item** region with brace
     matching over `mask_non_code()` output, and `_live_lines()` drops it (B15);
     `RUNTIME_IMPORT_PATTERNS = {"pcgen_import": r"\bpcgen_import\b"}` is merged into `PATTERNS`
     and deliberately kept out of `IDENTIFIER_PATTERNS` (B16).
  2. **B16 actually reaches the totals, which is the claim worth doubting.** Being listed in
     `PATTERNS` is not the same as being counted: `scan()` iterates `_COMPILED` — built from
     `PATTERNS`, so including `pcgen_import` — and adds every match into `file_hits`, which is
     what increments `res.live_files` / `res.live_hits`. Only `IDENTIFIER_PATTERNS` matches are
     *additionally* split out into `identifier_hits`. So a run-time `pcgen_import::` call site
     raises the closure bar's own numbers, from any of the five roots.
  3. **No live path is carved out.** `EXCLUDED_PREFIXES` is the empty tuple and `LIVE_ROOTS` is
     all five roots **including `apps/desktop`** (which covers both `src-tauri/` and the `src/`
     frontend, via `SOURCE_EXTENSIONS` `.rs .ts .tsx .js .jsx .mjs .cjs`).
     `python3 -m unittest scripts.tests.test_pcgen_residue_gate` → **Ran 39 tests, OK**.
  4. **An independent census, different algorithm, agrees at this HEAD.**
     `AT-35-E6-004_cycle1_independent_residue_census.py` re-reads the live side with a
     character-level Rust scanner that understands string literals, raw strings, char literals and
     block comments, borrowing nothing from the gate but its root list and patterns:
     ```
     INDEPENDENT live_files=0 live_hits=0
     over_skip_hits_hidden_by_gate=0
     lines_gate_counts_that_independent_calls_test=0
     gate_skipped_lines=90434 independent_skipped_lines=90434 files_with_boundary_mismatch=0
     ```
  5. **The wiring fails on a real re-entry — three probes, three different failure shapes.** Each
     was planted, the gate run, and the probe removed; the tree was `git status --porcelain`-clean
     before and after.

     | probe | where | gate output | exit |
     |---|---|---|---|
     | a run-time converter call, `codex::pcgen_import::cache_gen::probe_marker()` | `apps/desktop/src-tauri/src/at35_e6_004_c2_probe.rs` (B16's root) | `pattern pcgen_import files=1 hits=1` / `root apps/desktop files=1 hits=1` / `live_files=1 live_hits=1 verdict=FAIL` | 1 |
     | an ingest token in a string literal, `"BONUS:COMBAT\|TOHIT\|1"` | `src/rules_core/at35_e6_004_c2_probe.rs` | `pattern BONUS: files=1 hits=1` / `root src/rules_core files=1 hits=1` / `live_files=1 live_hits=1 verdict=FAIL` | 1 |
     | **the same token planted inside the window the old B15 bug used to blank** — line 1900 of `apps/desktop/src-tauri/src/update/transaction.rs` | shipping code between two `#[cfg(test)]` modules | `pattern BONUS: files=1 hits=1` / `root apps/desktop files=1 hits=1` / `live_files=1 live_hits=1 verdict=FAIL` | 1 |
     | all three removed | — | `live_files=0 live_hits=0 verdict=PASS` | 0 |

     The third probe is the one that matters. Cycle 1 fixed a line-level brace counter that let
     `b"not-json-{garbage"` run a `#[cfg(test)]` skip on past its module and blank 618 lines of
     shipping code (1869–2486). Re-derived at this HEAD,
     `cfg_test_ranges()` on that file returns `[(1062,1444), (1447,1499), (1502,1868),
     (2487,2885)]` over `total_lines=2885` — the third region ends at 1868, not 2885 — and a token
     planted at 1900 is **caught**. The fix is still in force and is proved by execution, not by
     reading the diff that made it.

- **Closure mode is the stage's mode.** `scripts/verify.sh:1480` —
  `if [[ "${PCGEN_RESIDUE_GATE_CLOSURE:-1}" == 1 ]]`. The default is `1`, so the
  `pcgen-residue-gate` stage runs `--check --closure`; the ratchet remains reachable as
  `PCGEN_RESIDUE_GATE_CLOSURE=0` as a diagnostic aid, never the shipping posture. Independently
  confirmed by the isolated wrap-up worker's full-gate run, whose stage table records
  `27 | pcgen-residue-gate | PASS | live_files=0 live_hits=0 verdict=PASS`
  (`EPIC-6_wrapup_gate_report.md`).

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
  | `new_since_cycle1` | — | **0** |

  The parity document at HEAD is **byte-identical** to `AT-35-E6-004_cycle1_sheet-parity-after.json`
  (`head == cycle1 doc: True`), so the three commits that landed after cycle 1 — including B17's
  shipped-data cleanup — moved no rendered number. The one disagreement new *since the epic's
  start* is `half_elf_fighter_l1 target:Pool:favored_class ours=1 oracle=2`, named and attributed
  in `AT-35-E6-003-SWEEP_cycle13_receipt.md`, not this cycle's and not cycle 1's. Artifact:
  `AT-35-E6-004_cycle2_sheet-parity-after.json`.

- **The tool side is intact** (`decisions.md §11` — KEPT for Starfinder):
  ```
  cargo build --locked --bin sheet_rule_convert --bin gen_book_cache   -> TOOLSIDE_EXIT=0
  cargo build --locked --release --bin sheet_rule_parity               -> PARITY_BUILD_EXIT=0 (2m01s)
  python3 scripts/oracle_harness/run.py --help                         -> ORACLE_HELP_EXIT=0
  git diff --stat c3500e7984..HEAD -- scripts/pcgen-oracle-pin.env     -> (empty; pin unchanged)
  git diff --stat c3500e7984..HEAD -- src/pcgen_import scripts/oracle_harness src/oracle_validation
                                                                       -> 64 files changed, 38465 insertions(+), 236 deletions(-)
  ```
  **Zero net deletions of function bodies**, proved by name rather than by diffstat — a diffstat
  cannot tell a move from a removal. Every `fn`/`def` name defined under `src/pcgen_import/**`,
  `scripts/oracle_harness/**` and `src/oracle_validation/**` at `c3500e7984` was re-derived at this
  HEAD:
  ```
  tool_side_files start=39 head=88
  fn/def definitions start=488 head=1453
  names_present_at_start_and_absent_at_HEAD=0
  ```
  The definition counts are lower than cycle 1's (`536` / `1652`) because this is a **different
  scanner**, written for this cycle: it counts *distinct* names via a single-line
  `pub/async/unsafe/extern fn <name>` + `def <name>` regex, where cycle 1's counted occurrences.
  The two are not comparable and are not being compared — the load-bearing figure is
  `names_present_at_start_and_absent_at_HEAD`, which is **0** under both instruments, and using a
  second instrument to reach it is the point.

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
    `docs/work-inventory.json` is byte-identical to `7fadc67843`'s.
  - **reachability:** none moved. The live side was already at zero when this cycle opened.
  - **instrument-correction:** none. This cycle changed no instrument — that is what makes it an
    audit. The `gate_skipped_lines` figure moved 90,433 → 90,434 between cycle 1 and now, which is
    one line of `#[cfg(test)]` code added by an intervening commit, not a boundary moving:
    the independent census reports the identical 90,434 and `files_with_boundary_mismatch=0`.
- **Refused tokens:** none. `token_coverage.py --check` → `refused=142 refused_non_done=0`; every
  refused token sits on a unit already `DONE` under the sheet rule (it renders as words). Zero
  units were in this cycle's scope, so no `deferral` event is owed (`decisions.md §2`).
- **Discoveries:** none. No token type, kind, or remaining-step category surfaced that
  `token-coverage.json` and the atlas did not already predict, and no defect was found in the
  gate — which, for an audit cycle, is the result being reported, not the absence of one. No
  `correction` event is owed; the one `verification` event emitted is
  `1789392460984-at-35-e6-004-fc318f`.
- **Figures + their re-derive commands:**

  | figure | value | command | denominator |
  |---|---|---|---|
  | live PCGen surface, closure mode | `live_files=0 live_hits=0 verdict=PASS` | `python3 scripts/pcgen_residue_gate.py --check --closure` | the 5 live roots, `.rs .ts .tsx .js .jsx .mjs .cjs`, comments (B14) and `#[cfg(test)]` regions (B15) excluded, shipped installer data (B17) included |
  | live PCGen surface, ratchet mode | `live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736 verdict=PASS` | `python3 scripts/pcgen_residue_gate.py --check` | same population, measured against `scripts/pcgen-residue-baseline.env` |
  | independent census | `live_files=0 live_hits=0`, `files_with_boundary_mismatch=0` | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-004_cycle1_independent_residue_census.py` | same population, different algorithm |
  | lines both censuses skip as `#[cfg(test)]` | gate `90434`, independent `90434` | the same census run's last line | the live files carrying a `#[cfg(test)]` item |
  | `#[cfg(test)]` regions in `update/transaction.rs` | `[(1062,1444),(1447,1499),(1502,1868),(2487,2885)]` | `python3 -c "import sys;sys.path.insert(0,'scripts');import pcgen_residue_gate as g;print(g.cfg_test_ranges(open('apps/desktop/src-tauri/src/update/transaction.rs').read().splitlines()))"` | `total_lines=2885` in that file |
  | gate unit tests | Ran **39** tests, OK | `python3 -m unittest scripts.tests.test_pcgen_residue_gate` | the gate's own suite |
  | `pcgen` references in the desktop crate | **167**, all comment or `#[cfg(test)]` | `grep -rniE 'pcgen' apps/desktop/src-tauri/src apps/desktop/src --include=*.rs --include=*.ts --include=*.tsx \| wc -l` | the desktop crate + frontend; that none is executable is the gate's `root apps/desktop files=0 hits=0`, not this grep |
  | tool-side function bodies lost | **0** (488 → 1,453 distinct names, 39 → 88 files) | the `fn`/`def` name census quoted above, `git ls-tree` at `c3500e7984` vs `HEAD` | `src/pcgen_import` + `scripts/oracle_harness` + `src/oracle_validation` |
  | oracle parity at HEAD | 156/154/2 lines, 382/376/6 chassis, 29 characters | `sheet_rule_parity --roster artifacts/epic-2-sheet-rule/oracle-parity/roster --output <ours>` then `python3 scripts/oracle_harness/sheet_parity.py compare --ours <ours> --exports artifacts/epic-2-sheet-rule/oracle-parity/exports --output <out>` | the 29-character fixture roster, pin `7f818006e3` |
  | parity disagreements cleared or created since the epic start | `gone_since_start=0`, `new_since_cycle1=0` | `python3` set-difference over the three parity JSONs (start / cycle 1 after / this cycle) | the 505 rendered lines across the 29-character roster |
  | ingest tokens shipped in `data/sheet_rules/` | **0** | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | every converted rule file |
  | converter `--check` | `CONVERT_CHECK_EXIT=0` | `cargo run --locked --bin sheet_rule_convert -- --check` | `data/sheet_rules/` |
  | denominator gate | `files_checked=159 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` | the SD-35 bundle's markdown only — narrower than the full gate's 336, which globs the whole repo |
  | figure provenance | `figures_examined=486 violations=0` | `python3 scripts/denominator_gate.py --check-provenance '<same two globs>'` | same 159 files; cycle 1's two unsourced rows are gone |

- **Build scope verified:** **no Rust source changed this cycle** (`rust_lines_changed=0`), so the
  compiled surface is `7fadc67843`'s — re-proved rather than assumed, on a `CARGO_TARGET_DIR`
  scoped to this cycle. `cargo build --locked --bin sheet_rule_convert --bin gen_book_cache` →
  `TOOLSIDE_EXIT=0`; `cargo build --locked --release --bin sheet_rule_parity` →
  `PARITY_BUILD_EXIT=0`; `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`, **419 test
  executables** linked; `cargo test --locked --lib -j 6` → `LIB_EXIT=0`, `test result: ok. 3390 passed; 0 failed; 16 ignored` (45.60 s).
  `cargo test --no-fail-fast` is **not** re-run: its trigger is "`src/` or the classifier changed"
  and neither did. `cargo clippy` is not run: its input this cycle is four markdown/JSON files and
  no Rust target. The desktop crate and frontend are not re-run here (`apps/` untouched); their
  result at this tree is the wrap-up worker's full-gate run. Gates run at HEAD:
  `completion_atlas.py --check` → `missing_clearing_mechanisms=0 stale_derived_at=False
  citation_failures=0`; `token_coverage.py --check` → `verdict=PASS`;
  `shape_engine_boundary.py --check` → `magnitude_bearing=26396 not_held_by_engine=0
  citation_ok=True`; `missing_engine_tables.py --check` → `population=0 kinds=0
  citation_failures=0`; `denominator_gate.py --check` → `files_checked=159 violations=0`;
  `scripts/verify.sh --only pi-sweep` → `RESULT: PASS`.
- **Sweep population:** N/A — no corpus record changed, so `corpus_literal_sweep` is not re-run.
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
  without needing a flag.
