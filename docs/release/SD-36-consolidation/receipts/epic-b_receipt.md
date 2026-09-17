# SD-36 Epic B receipt — freeze the PF1e status snapshot, retire the producers

Written to this fallback path because `docs/release/SD-36-consolidation/` (the
full STC package: README, decisions.md, epic-breakdown.md, etc.) did not
exist yet when this epic executed — per the execution brief's instruction,
this receipt stands alone until that package is authored.

Branch `tranche/16`. Operator rulings executed: D3 (retire the producers),
D5 (public page counts every unit with the inventory's own DONE vocabulary,
freezes at 100%).

## Commits (in order)

1. `82e2439ab2` — feat(sd36,epic-b): D5 freeze the public status page at 100%
2. `0aa1f77223` — feat(sd36,epic-b): retire v06_work_inventory, support_state_matrix + bridge
3. `b8ff0e895b` — feat(sd36,epic-b): retire reach_gate.rs (separate, revertable commit)
4. `94d915609c` — fix(sd36,epic-b): fold the reach_gate retirement's remaining edits
5. (this cycle's final commit — verify.sh doneness/figure-provenance fixes, docs, sidecar, retro, baselines)

## D5: public status page frozen at 100%

`scripts/site/build_public_status.py` rewritten: doneness now comes from
the ledger's own `status` word against the inventory's DONE vocabulary
(`DONE_STATUSES`/`NOT_DONE_STATUSES`), not a wiring-class-derived producer
verdict; denominator = every unit, no standing gate.

Before → after (`python3 -c "import json;print(json.load(open('site/status-data.json'))['overall'])"`):

| field | before | after |
|---|---|---|
| pct | 95.0 | 100.0 |
| done | 35,982 | 49,450 |
| denominator | 37,892 | 49,450 |
| not_started | 0 | 0 |
| partial | 1,910 | 0 |

New `scripts/site/check_frozen_status.py` + test + verify.sh stage
`site-status-frozen-check` (both stage sets) assert this never drifts.

## D3: files deleted (whole files, pre-deletion `wc -l`)

| path | lines | command |
|---|---|---|
| `src/bin/v06_work_inventory.rs` | 33,091 | `wc -l` on the file before `git rm` |
| `src/rules_core/support_state_matrix.rs` | 7,475 | ″ |
| `apps/desktop/src-tauri/src/support_state_matrix_bridge.rs` | 415 | ″ |
| `apps/desktop/src-tauri/src/reach_gate.rs` | 8,846 | ″ (separate commit) |
| `apps/desktop/src/boundary/loadSupportStateMatrix.ts` | 54 | ″ |
| `apps/desktop/src/boundary/loadSupportStateMatrix.test.ts` | 34 | ″ |
| `tests/sd13_support_state_matrix.rs` | 1,258 | ″ |
| `tests/v06_work_inventory.rs` | 1,220 | ″ |
| `tests/sd13_race_dwarf_bounded_semantics.rs` | 246 | ″ |
| `tests/sd13_race_halfling_bounded_semantics.rs` | 247 | ″ |
| `tests/sd13_elf_bounded_race_semantics.rs` | 238 | ″ — **not in the design appendix's delete list**, found during execution: a 5th pure matrix-row file, every one of its 10 `#[test]` fns asserts a matrix row (confirmed by reading each fn body) |
| `scripts/publish-site-dashboard.sh` | 278 | ″ |
| `scripts/tests/test_publish_site_dashboard.sh` | 294 | ″ |
| `scripts/shape_engine_boundary.py` | 229 | ″ |
| `scripts/tests/test_shape_engine_boundary.py` | 260 | ″ |
| `scripts/tests/test_site_dashboard_units_status_vocabulary_current.py` | 104 | ″ |
| `.claude/skills/swarm-status-sync/SKILL.md` | 257 | ″ |
| `docs/architecture/support-state-matrix.md` | 176 | ″ |
| `src/bin/fixture_verified_oracle_probe.rs` | 745 | ″ — confirmed zero callers/tests/scripts via `git grep` before deletion |

**Total whole-file deletion: 55,467 lines** (the appendix's own estimate was
55,827 for a 4-file, not 5-file, matrix-file list — the 238-line addition
plus small count differences in the named files account for the gap).

**NOT deleted despite the design appendix's §3 list naming them:**
`scripts/completion_atlas.py` (787 lines) and
`scripts/tests/test_completion_atlas.py` (545 lines). See "Corrections to
the plan" below.

## Partial edits

- `src/rules_core/mod.rs`: dropped `pub mod support_state_matrix;`.
- **290 test files** (`git grep -l seeded_current_truth -- tests`, minus
  the 5 wholesale-deleted files above which had already been removed from
  the list by the time of the final count): stripped exactly the
  matrix-row-asserting `#[test]` fn(s) + the `support_state_matrix`
  use-import per file via a brace-aware Python script
  (string/comment-sanitizing before brace matching, verified against 3
  hand-checked samples before the bulk run); every other test in each file
  is untouched. 307 + 26 = **333 matrix-row test fns removed** across the
  batch (two script runs, the first halted itself before writing a
  malformed file when it hit the one file needing manual handling first).
- `tests/sd13_race_gnome_bounded_semantics.rs`,
  `tests/sd13_half_orc_bounded_race_semantics.rs`: manual surgical strip
  (not wholesale delete, not the bulk script) — each carries real,
  independent `compute_pilot_base_chassis` tests alongside the matrix-row
  ones; confirmed by reading every `#[test]` fn body, not by the file's
  name pattern (the appendix's own characterization of "every fn asserts a
  matrix row" for these two was wrong).
- `tests/sd19_seam_shapes_correctness.rs`: stripped part (d)
  (`MatrixSubjectType` construct/compare) plus its now-unused
  `EquipmentCategory`/`Pf1SchoolId` imports and the `support_state_matrix`
  use-import — found via `git grep support_state_matrix:: -- tests` *after*
  the `seeded_current_truth` grep returned 0, since this file imports
  `MatrixSubjectType` directly and never calls `seeded_current_truth()`, so
  it never appeared in the appendix's own delete-list-generating grep.
- `tests/sd35_rendered_prose_carries_no_ingest_vocabulary.rs`: removed the
  now-deleted `src/rules_core/support_state_matrix.rs` entry from its
  `SCANNED` file list — this test `.unwrap_or_else(|e| panic!(...))`s on a
  missing file, so it would have failed hard the moment the cited file was
  gone; the appendix listed this file's `support_state_matrix.rs` mention
  among ~300 "prose, historical" hits without checking this one was a
  content-scan path, not prose.
- `apps/desktop/src-tauri/src/main.rs`: dropped `mod support_state_matrix_bridge;`,
  its use-import, the `load_support_state_matrix` Tauri command, its
  `generate_handler!` entry, and (separate commit) `#[cfg(test)] mod reach_gate;`.
- `apps/desktop/src/App.tsx`: removed `SupportDebtPanel`, `BreadthClaimAuditPanel`,
  their two mounts, and the dead-code-only `supportStateTone`/
  `freshnessLabel`/`subjectTypeLabel`/`humanizeSubjectId` helpers +
  `SUBJECT_TYPE_LABELS` const those panels alone used — no "unavailable"
  stub left, per no-stub doctrine.
- `apps/desktop/src/testerWorkbench/loadTesterWorkbenchSurface.ts` +
  `loadTesterWorkbenchSurfaceRuntime.ts`: removed the SupportDebt/
  BreadthClaimAudit presentation types, builders, and the
  `loadSupportStateMatrix` dependency end to end.
- `apps/desktop/src/boundary/levelUpCharacter.test.ts`: fixed a doc-comment
  cross-reference to the deleted `loadSupportStateMatrix.test.ts`.
- `apps/desktop/src-tauri/src/race_catalog.rs`: removed
  `ingested_race_ids_for_book` — its own doc comment named it as existing
  solely to feed `reach_gate`'s races claim; zero other callers; flagged as
  dead code by `cargo test --locked --no-run` after the `reach_gate.rs`
  deletion.
- `tests/sd24_wired_integration_audit.rs`: dropped the
  `is_reach_gate_open_finding_corpus_prose` allow-list closure (its path
  can never match again).
- `scripts/verify.sh`: removed stages `site-dashboard-selftest`,
  `site-dashboard-pin`, `site-dashboard-check`, `shape-engine-boundary-selftest`,
  `shape-engine-boundary`, `reach` (functions, both stage-list entries, case
  dispatch arms); added `site-status-frozen-check` (both stage sets) and
  `doneness-selftest` (both stage sets, see "Corrections to the plan").
  Reworded 3 stale cross-reference comments in surviving stages that named
  a now-removed stage or the retired citation machinery.
- `scripts/missing_engine_tables.py` + its test: same content-anchor-removal
  fix as `completion_atlas.py` (see below); **this also required fixing
  `run_missing_engine_tables()` in verify.sh**, which parsed a
  `citation_failures=` line the script no longer prints — undetected until
  the stage was run directly (would have permanently failed the kept
  `missing-engine-tables` stage otherwise).
- `scripts/denominator_gate.py` + its test: added a reviewed
  `RETIRED_SCRIPT_PATHS` registry so a sealed historical `_cycle_receipt.md`
  citing a since-retired script does not fail `figure-provenance` forever
  (see "Corrections to the plan").
- `pf1e_dashboard_producer.py`'s only two real remaining importers narrowed:
  new `scripts/observer/doneness.py` (~250 lines incl. full historical doc
  comments, verified byte-identical to the producer's own
  `doneness_verdict`/`EXCLUDED_BOOKS` across all 260 (wiring_class, status,
  kind) combinations both modules define) now serves
  `scripts/coverage_ledger.py`; the dead, unused `pf1e_dashboard_producer`
  import in `scripts/shape_ledger.py` removed. `pf1e_dashboard_producer.py`
  itself is untouched (see "Corrections to the plan").
- Docs: `docs/work-inventory.FROZEN.md` (new sidecar), `site/dashboard/README.md`
  (FROZEN section replacing "Refreshing the data"), `.claude/skills/publish-site/SKILL.md`,
  `docs/governance/book-ingestion-playbook.md` (top-of-file retirement
  notice), `docs/architecture/{status,overview,testing,rules-engine,
  rules-data-tables,conventions,README,desktop-app}.md` (dead links,
  current-state table rows, and diagram nodes for the deleted matrix/bridge
  removed; historical narrative left as historical record).
  `docs/governance/license-matrix.md` had 0 live mentions at execution time
  (plan's "2 mentions" was stale); no edit made.

## Corrections to the plan (recorded in `docs/retro/events/sd36-epic-b.jsonl`)

1. **Delete-list undercounted matrix-only test files** (4 named, 5 real;
   2 of the 4 named were actually mixed real+matrix content). See the
   `correction` event, subject "SD-36 dashboard-retirement design appendix".
2. **`completion_atlas.py` deletion would have broken 3 KEPT verify.sh
   stages** (`cycle-scope-gate-selftest`, `token-coverage-selftest`,
   `token-coverage`) — restored the file, stripped only its dead
   content-anchor machinery instead. See the second `correction` event.
3. **Retiring 2 scripts broke `figure-provenance` against 8 sealed SD-34/
   SD-35 receipts** (9 citations) — fixed via a reviewed
   `RETIRED_SCRIPT_PATHS` registry, not by editing the sealed receipts. See
   the `note` event.
4. **`pf1e_dashboard_producer.py` full deletion deferred**, not completed —
   the appendix's own "extract ~150 lines then delete the producer"
   instruction conflicts with the SAME appendix's "keep producer-selftest"
   verify.sh instruction; `scripts/reachability_audit.py` also has a real
   (if narrow) live dependency on the producer beyond `doneness_verdict`
   (`WIRING_CLASS_VALUES`) that the appendix's deferral note did not
   mention. See the `deferral` event for the exact remaining work.
5. **A tool-use mistake, not a plan error**: `git stash` was run once
   mid-session investigating an unrelated question, in violation of a
   standing rule against it on this exact checkout; recovered cleanly via
   `git stash pop` (created with `--keep-index`), confirmed via
   `git status --porcelain` and re-running affected tests. See the
   `incident` event.
6. **A second tool-use mistake**: after the first full `bash scripts/verify.sh`
   pass finished (RESULT: FAIL — see "Verification" below), a background
   wait loop (`until ! pgrep -f "bash scripts/verify.sh"; ...`) matched its
   own shell's command line via `pgrep -f` and never exited, appearing to
   hang for hours after the real work was already done. Caught by a
   teammate, who killed the stray loop. Fixed by polling a captured PID
   with `kill -0 "$PID"` instead of a pattern match, and by bounding every
   subsequent wait to short foreground polls rather than an unbounded
   background wait.

## Verification

- `python3 -m unittest` on every touched/new Python test file: green
  (`test_build_public_status.py` 40/40, `test_check_frozen_status.py` 4/4,
  `test_completion_atlas.py` 42/42, `test_missing_engine_tables.py` 11/11,
  `test_denominator_gate.py` 57/57, `test_doneness.py` 5/5,
  `test_coverage_ledger.py` + `test_shape_ledger.py` 88/88).
- `cargo test --locked --no-run -j 6` at root: compiles clean, 0 errors,
  0 warnings (after the `race_catalog.rs` dead-code fix).
- `cargo test --locked --no-run -j 6` in `apps/desktop/src-tauri`: compiles
  clean, 0 errors, 0 warnings.
- `cargo test --locked -j 6 --test sd24_wired_integration_audit`: 5/5 pass.
- `bash -n scripts/verify.sh`: syntax OK, checked after every edit to the file.
- `git grep -c seeded_current_truth -- tests src apps`: 0 matches.
- `git grep -n support_state_matrix:: -- tests src apps`: 0 matches.
- **First full `bash scripts/verify.sh` pass: RESULT FAIL.** 0 test
  failures anywhere; 2 stages red:
  - `root-full`: `7855 passed across 412 suites, 0 suite(s) never ran` —
    the stale `BASELINE_ROOT_FULL_TESTS=8926` floor, not a regression (see
    "Baseline figures" below).
  - `clippy`: `root:4 desktop:1` warnings — 4 dead `EquipmentCategory`
    imports in `tests/sd19_equipment_{arms_armor,general,equipmods,magic_items}.rs`
    left behind by the matrix-fn strip (the fn removed used
    `MatrixSubjectType::Equipment(EquipmentCategory::...)`; the separate,
    non-`support_state_matrix` `EquipmentCategory` import was not part of
    that strip's target and went unused), and 1 pre-existing (not caused by
    this epic) "empty line after doc comment" in
    `apps/desktop/src-tauri/src/class_spell_levels.rs` (a `//` section
    comment sat between a `///` doc comment and the fn it documented).
    Fixed all 5 at the source; re-verified `cargo clippy --locked --tests
    -j 6` at root and in `apps/desktop/src-tauri`: 0 warnings, both crates.
- Baselines corrected (see below) and **second pass,
  `bash scripts/verify.sh --only root-full --only clippy`: RESULT PASS.**
  `root-full (7855 passed across 412 suites, all 360 tests/*.rs suites
  executed)`; `clippy (root:0 desktop:0 warnings, 0 errors)`.

## Baseline figures (before → after, each with its command)

Appended as a dated block to `scripts/verify-baselines.env` (full
attribution and two independent re-derivations are in that block's own
comment; summarized here):

| baseline | before | after | why |
|---|---|---|---|
| `BASELINE_ROOT_FULL_TESTS` | 8,926 | 7,855 | retired, not regressed: v06_work_inventory.rs's own 644 `#[test]` fns, 333 matrix-row fns stripped from 289 kept files, 732 fns in 5 wholly-deleted test files |
| `BASELINE_ROOT_TEST_BINARIES` | 419 | 412 | −1 bin-unittest binary (v06_work_inventory.rs), −6 `tests/*.rs` binaries |
| `BASELINE_ROOT_LIB_TESTS` | 3,390 | 3,390 (unchanged) | `support_state_matrix.rs` carried 0 `#[test]` fns of its own |
| `BASELINE_DESKTOP_TESTS` | 570 | 592 | pre-existing SD-35 growth, un-rebaselined; flagged, not attributed (out of this epic's scope) |
| `BASELINE_FRONTEND_TEST_FILES` | 101 | 121 | same disposition as `BASELINE_DESKTOP_TESTS` |

`python3 -c "import json;print(json.load(open('site/status-data.json'))['overall'])"`:
`{'done': 49450, 'partial': 0, 'not_started': 0, 'denominator': 49450, 'pct': 100.0, ...}`.
