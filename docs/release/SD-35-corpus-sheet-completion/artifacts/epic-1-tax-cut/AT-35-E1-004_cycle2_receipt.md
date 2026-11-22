# Cycle AT-35-E1-004-cycle2 — Epic 1 — Tax cut / AT-35-E1-004

- **Commit SHA:** `dd48f73d0f` (the script + test change; this receipt and the board rows land in the follow-up docs commit, `workflow-instruction.md §6` steps 5-7)
- **Scope gate:** `SCOPE_GATE: EXEMPT (gate-retargeting cycle — closes zero units by design, decisions.md §2)`
- **Files touched:** `scripts/denominator_gate.py`, `scripts/tests/test_denominator_gate.py`,
  `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-1-tax-cut/AT-35-E1-004_cycle2_receipt.md`,
  `docs/release/SD-35-corpus-sheet-completion/progress.md`,
  `docs/release/SD-35-corpus-sheet-completion/kanban.md`,
  `docs/retro/events/sd31-transcribe.jsonl` (this cycle's `correction` event, folded),
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
  (`derived_at` stamp refreshed by this cycle's own `completion_atlas.py --check`, folded —
  precedent `c15e64bc3e`)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** *(verbatim, `epic-breakdown.md` "### AT-35-E1-004 — the ratio row, and
  the gates point at this package")*

  > `workflow-instruction.md §7`'s receipt carries the `rust_lines_changed / units_closed` and
  > `pcgen_live_files` rows, produced by `cycle_scope_gate.py --receipt`. `scripts/denominator_gate.py`
  > (`BUNDLE_DIR` at :101 and `DEFAULT_GLOBS` at :121 still point at **SD-33** — never advanced to
  > SD-34) and the `figure-provenance` stage default to SD-33 **and** SD-34 **and** this package;
  > nothing already scanned stops being scanned.
  >
  > **Evidence:** `scripts/verify.sh --only denominator-gate` default run lists every SD-35 `.md`
  > in `files_checked`, `violations=0`; `--only figure-provenance` exits 0 across the package.

- **Receipt rows (mechanical):**
  ```
  since=942c8d3ae5db5d447efd12400b2db3b8644e8be1 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=260
  ```
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`
  — identical to the previous receipt's; this cycle touched no live path.
- **Oracle parity:** N/A — no `Number` mapping added, no live path touched.
- **Movement, four buckets:**
  - closure (into DONE, by id-set): **none** — a gate-scope cycle, zero units by design.
  - relabel (bucket to bucket): none.
  - reachability: none.
  - instrument-correction: **one, and it is this cycle's whole content.** Cycle 1's default-glob
    widening scanned 39 of the 40 `.md` files in SD-35's package; `references/README.md` (in
    neither the package root nor `artifacts/`) was never read by a default run of either stage.
    Cycle 1's own coverage test could not catch it: `_real_sd35_md()` built its "every SD-35 `.md`"
    expected set by re-running `*.md` + `artifacts/**/*.md`, the same two globs it then asserted
    `DEFAULT_GLOBS` covered — a circular assertion that cannot fail for a file the globs miss.
- **Refused tokens:** none
- **Discoveries:** one, instrument-shaped — a self-referential coverage test. Emitted as a
  `correction` retro event (`1788881408334-sd31-transcribe-957b44`). Not a token type and not an
  atlas category, so no atlas re-derivation is owed; the atlas ran clean (`citation_failures=0`).
- **Figures + their re-derive commands:**
  | Figure | Denominator | Command |
  |---|---|---|
  | SD-35 `.md` files on disk: **40** before this receipt, **41** after | all `.md` under `docs/release/SD-35-corpus-sheet-completion/` | `find docs/release/SD-35-corpus-sheet-completion -name '*.md' \| wc -l` |
  | covered by cycle 1's globs: **39 of 40** | same 40 | `python3 -c "import sys;sys.path.insert(0,'scripts');import glob,os;d='docs/release/SD-35-corpus-sheet-completion';print(len(set(glob.glob(d+'/*.md'))\|set(glob.glob(d+'/artifacts/**/*.md',recursive=True))))"` |
  | covered after this cycle: **41 of 41** (every file the walk finds) | all `.md` under the package | `python3 -m unittest discover -s scripts/tests -p test_denominator_gate.py` (`test_default_run_includes_every_sd35_md_file`) |
  | `denominator-gate` `files_checked` **225 → 227** (+1 `references/README.md`, the widening's whole point; +1 this receipt itself) | every path in `DEFAULT_GLOBS` (SD-33 + SD-34 + SD-35) | `scripts/verify.sh --only denominator-gate` |
  | `figure-provenance` `files_checked` **155 → 157** (same two files), `figures_examined=173` | every path in `PROVENANCE_DEFAULT_GLOBS` (SD-34 + SD-35) | `scripts/verify.sh --only figure-provenance` |
  | gate unit tests: **55 of 55** pass (54 before, +1 anti-circularity test) | `scripts/tests/test_denominator_gate.py` | `python3 -m unittest discover -s scripts/tests -p test_denominator_gate.py` |
  | lib tests **3217 passed, 0 failed**, 14 ignored | the workspace `--lib` target set | `cargo test --locked --lib -j 6` |
  | `token_coverage` `non_done=1404` of `refused=1810` | SD-35's token-coverage population | `python3 scripts/token_coverage.py --check` |
- **Build scope verified:** `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`;
  `cargo test --locked --lib -j 6` → `test result: ok. 3217 passed; 0 failed; 14 ignored`,
  `LIB_EXIT=0`; `cargo run --locked --bin sheet_rule_convert -- --check` → `CONVERT_CHECK_EXIT=0`.
  `cargo test --locked --no-fail-fast -j 6` **not run** — `workflow-instruction.md §6` step 3
  requires it "when `src/` or the classifier changed", and this cycle changed neither (the diff is
  two Python files under `scripts/`). `corpus_literal_sweep` **not run** — no corpus record
  changed. `cargo clippy` **not run** — no Rust target touched. Desktop crate and frontend: epic
  cadence, `apps/` untouched. Run at SHA `942c8d3ae5` (tree = HEAD plus this cycle's two script
  edits).
- **Sweep population:** N/A — no corpus records changed.
- **Oracle pin:** N/A — no figure came from the pinned corpus.
- **Status:** complete
- **Notes:** The fix is additive, as the criterion's own invariant demands — cycle 1's two SD-35
  entries stay in both lists and a third, `SHEET_COMPLETION_BUNDLE_DIR/**/*.md`, subsumes them and
  picks up any future subdirectory without another widening cycle. `expand_paths` deduplicates, so
  the overlap costs nothing. `test_nothing_already_scanned_stops_being_scanned` still pins the
  frozen pre-widening lists; SD-33 stays out of the provenance default for the measured reason
  already recorded there.
- **Next-cycle scope:** criterion at zero.

## Full gate run, this cycle

```
scripts/verify.sh --only denominator-gate       PASS  (files_checked=227 violations=0)   RESULT: PASS
scripts/verify.sh --only figure-provenance      PASS  (files_checked=157 figures_examined=173 violations=0)   RESULT: PASS
python3 -m unittest .../test_denominator_gate.py    Ran 55 tests   OK
cargo test --locked --no-run -j 6               NO_RUN_EXIT=0
cargo test --locked --lib -j 6                  3217 passed; 0 failed; 14 ignored   LIB_EXIT=0
cargo run --locked --bin sheet_rule_convert -- --check   CONVERT_CHECK_EXIT=0
grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l    0
python3 scripts/pcgen_residue_gate.py --check   live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS
python3 scripts/completion_atlas.py --check     done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0   EXIT=0
python3 scripts/token_coverage.py --check       non_done=1404 tokened=1399 token_less=5 refused=1810 refused_non_done=659 token_types=231 shapes=81 verdict=PASS
python3 scripts/shape_engine_boundary.py --check    magnitude_bearing=26396 not_held_by_engine=363 citation_ok=True   EXIT=0
python3 scripts/missing_engine_tables.py --check    citation_failures=0   EXIT=0
python3 scripts/denominator_gate.py --check '<SD-35 root *.md>' '<SD-35 artifacts/**/*.md>'    files_checked=40 violations=0
scripts/verify.sh --only pi-sweep               PASS  (11 hits over src/rules_core/rules_tables, 11 baseline rows)   RESULT: PASS
```

## RED → GREEN

**RED** (before the widening, with the anti-circularity fix to `_real_sd35_md()` in place):

```
FAIL: test_default_run_includes_every_sd35_md_file
  SD-35 .md file(s) not covered by the widened default:
    {'.../SD-35-corpus-sheet-completion/references/README.md'}
FAIL: test_provenance_default_includes_every_sd35_md_file
  SD-35 .md file(s) not covered by the provenance default:
    {'.../SD-35-corpus-sheet-completion/references/README.md'}
Ran 55 tests — FAILED (failures=2)
```

**GREEN** (after adding the recursive entry to both lists):

```
Ran 55 tests in 1.909s
OK
```

A third test, `test_expected_set_is_a_filesystem_walk_not_the_globs_under_test`, pins the
anti-circularity fix itself: it asserts the filesystem walk finds at least one `.md` that neither
cycle-1 glob matches, so the two coverage assertions can never quietly go vacuous again.
