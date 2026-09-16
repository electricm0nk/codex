---
canonical: true
owner: operator
bundle_id: SD-36
date: 2026-09-15
---

# SD-36 Epic Breakdown

Acceptance criteria per epic, grouped with acceptance commands. Read this before dispatch.

---

## Epic B — Freeze PF1e status page, retire producers (one batch, one pass)

| Criterion | Acceptance command / evidence |
|---|---|
| B1. Frozen-status gate exists and fires | `python3 -m pytest scripts/tests/test_check_frozen_status.py -q` (all green); `scripts/site/check_frozen_status.py --check` exits zero on `site/status-data.json` |
| B2. Public status rewritten per D5 | `python3 scripts/site/build_public_status.py --check` exits zero; `site/status-data.json` shows 49,450 / 49,450; old-rule tests in `test_build_public_status.py` rewritten and green |
| B3. Whole files deleted (8 binaries/modules, 9 tests) | `ls -la src/bin/v06_work_inventory.rs` exits nonzero (file gone); same for `support_state_matrix.rs`, `reach_gate.rs`, desktop bridge, 9 test files |
| B4. Desktop UI cleanup | `grep -r 'SupportDebtPanel\|BreadthClaimAuditPanel\|supportStateTone' apps/desktop/src/` exits nonzero (component code gone); git diff shows the UI components deleted, wiring removed |
| B5. Verify.sh stages updated | `bash scripts/verify.sh --list \| grep 'site-dashboard'` (0 matches); `bash scripts/verify.sh --list \| grep 'site-status-frozen'` (1+ matches in both stage sets) |
| B6. Baseline table updated | `scripts/verify-baselines.env` shows `BASELINE_ROOT_TEST_BINARIES=412` (was 419 before deletions); `BASELINE_ROOT_FULL_TESTS` re-measured and recorded |
| B7. Work inventory frozen | `ls -la docs/work-inventory.FROZEN.md` (file exists); content records SHA, date, rule; `docs/work-inventory.json` unchanged |
| B8. Deferral/extraction decision recorded | `scripts/retro.py deferral` entry if `pf1e_dashboard_producer.py` extraction incomplete; commit message references the receipt |
| B9. One pass green | `cargo test --locked --no-run -j 6` at root and desktop succeeds; `bash scripts/verify.sh` all stages green; all test suites pass |

**Deleted files / line counts (for receipt):**
- `src/bin/v06_work_inventory.rs` — 33,091 lines
- `src/rules_core/support_state_matrix.rs` — 7,475 lines
- `apps/desktop/src-tauri/src/support_state_matrix_bridge.rs` — ~1,500 lines (approx)
- `apps/desktop/src-tauri/src/reach_gate.rs` — 8,846 lines
- 9 test files — ~2,500 lines (total)
- 3+ support scripts and docs — ~3,000 lines (total)

Total: 55,827 lines.

---

## Epic A — PCGen wall: crate `crates/codex-ingest` (one batch, one pass)

| Criterion | Acceptance command / evidence |
|---|---|
| A1. Gate tests green | `python3 -m pytest scripts/tests/test_pcgen_residue_gate.py -q` (all green); patterns detect `lst_file` identifier and `codex_ingest` in test vs. live root |
| A2. Residue gate patterns added | `grep 'lst_file' scripts/pcgen_residue_gate.py` (pattern present); `python3 scripts/pcgen_residue_gate.py --check --closure` exits zero; output shows `lst_file files=0 hits=0` and `codex_ingest files=0 hits=0` |
| A3. Crate scaffolded | `ls -la crates/codex-ingest/{Cargo.toml,src/lib.rs}` (both exist); manifest declares workspace membership; lib exports `pub mod pcgen_import; pub mod oracle_validation;` etc. |
| A4. Machinery moved | `ls -la src/pcgen_import` exits nonzero (moved); same for `src/oracle_validation`; `wc -l crates/codex-ingest/src/pcgen_import/` shows the moved code; 47 tool bins present in crate (git diff `--name-status` shows M mode for moves) |
| A5. Path rewrites complete | `git grep -c 'CARGO_MANIFEST_DIR' -- crates` exits 0 (no remaining); `git grep -c 'crate::pcgen_import' -- crates/codex-ingest` shows uses redirected to `codex_ingest::` within the crate |
| A6. Test-module eviction done | `find src/rules_core -name '*.rs' -exec grep -l 'cfg(test)' {} +` shows 0 lines with `#[cfg(test)] mod` + `crate::pcgen_import` (these are gone; they moved to fixtures or were routed to `crates/codex-ingest/tests/`); `git grep -c 'crate::pcgen_import' -- src/rules_core` exits 0 |
| A7. `SourceRef.lst_file` renamed | `git grep -c 'lst_file' -- src` exits 0 (no raw field name); `git grep -c 'source_path' -- src` shows uses; desktop fixtures diff shows only the key rename (no content drift) |
| A8. Desktop import rewritten | `git grep 'codex_ingest::pcgen_import::pcgen_desc::leaked_pcgen_syntax' -- apps/desktop/src-tauri/tests` shows 9+ lines (9 test imports); `git log -1 --name-status` shows edits to those files |
| A9. Verify.sh stages added | `bash scripts/verify.sh --list \| grep 'ingest\|crate-wall'` shows both stages present; `bash scripts/verify.sh --list \| grep 'clippy' \| tail -1` shows third clippy (crate-ingest); CI workflow shows `cargo test --locked --workspace` (not just `--lib`) |
| A10. Baselines updated | `scripts/verify-baselines.env` shows `BASELINE_INGEST_FULL_TESTS`, `BASELINE_INGEST_TEST_BINARIES`, `BASELINE_CLIPPY_WARNINGS_INGEST=0`; root binaries drop by 47; root + ingest totals ≥ Epic B green count |
| A11. One pass green | `cargo test --locked --workspace --no-run -j 6` at root succeeds; `cargo test --locked --no-run -j 6` in desktop succeeds; `bash scripts/verify.sh` all stages green, `crate-wall` stage green; residue gate 0/0; all test suites pass |

---

## Epic C — Bloat cuts (two batches, two passes)

### Pass C1 — Source-side refactor

| Criterion | Acceptance command / evidence |
|---|---|
| C1.1. `pilot_compute/mod.rs` split | `wc -l src/rules_core/pilot_compute/mod.rs` shows ≤ 4,000 lines (was 88,828); `ls -la src/rules_core/pilot_compute/class_*.rs` shows ~12 class files exist; largest file `wc -l src/rules_core/pilot_compute/prestige_class_features.rs` shows ≤ 6,600 |
| C1.2. Submodules wired | `git grep -c 'mod class_' -- src/rules_core/pilot_compute/mod.rs` shows ~20+ mod declarations; `git grep -c 'pilot_compute::' -- src` shows call sites unchanged (paths still work) |
| C1.3. Path helpers consolidated | `ls -la src/support/paths.rs` (exists); `wc -l src/support/paths.rs` shows 100-200 lines (includes the 5 fns + tests); `git grep -c 'fn repo_root' -- src` shows 1 hit (only in paths.rs) |
| C1.4. Clippy lock added | `grep -A2 'fn clippy_one_crate' scripts/verify.sh \| grep -- '-- -D warnings'` (present); `bash scripts/verify.sh --list \| grep clippy \| wc -l` shows same count or +1 |
| C1.5. One pass green | `cargo test --locked -- --list \| head -50` (output same as before, modules normalized); `bash scripts/verify.sh` all stages green; all suites pass |

### Pass C2 — Test-side rewrite

| Criterion | Acceptance command / evidence |
|---|---|
| C2.1. Table-driven tests rewritten | `wc -l tests/sd18_widening/rows.rs` shows const ROWS present; `ls -la tests/sd18_widening/` shows per-row files remain, new rows.rs added; `cargo test --locked -- --list \| grep sd18_widening \| wc -l` shows 2,219 entries (unchanged) |
| C2.2. Test count proof | `cargo test --locked -- --list \| grep sd18_widening` saved to diff artifact before/after; byte-identical match (no module path normalisation needed) |
| C2.3. Path helpers re-exported | `git grep -c 'from super::paths' -- tests/` shows 1+ hits; local copy count in tests/ drops from 59 to 0 |
| C2.4. Branch promotion test moved | `ls -la tools/ci/test_branch_promotion_guard.sh` (exists); `ls -la tests/sd16-e5-f1/` (directory exists but file gone); 5 references in `publish-site-to-main.sh`, CI workflows, etc. updated |
| C2.5. Oracle tests kept | `cargo test --locked -- --list \| grep -c '\\[ignore\\]'` shows 65+ (unchanged); oracle tests run once with real PCGen corpus (documented in receipt) |
| C2.6. One pass green | `cargo test --locked -- --list` diff IDENTICAL; `bash scripts/verify.sh` all stages green; all suites pass |

---

## Epic D — Closure

| Criterion | Acceptance command / evidence |
|---|---|
| D1. Architecture docs refreshed | `git diff docs/architecture/` shows updated Last-verified headers and refreshed boundary/rules-engine/desktop-app/status/testing sections for topics touched by B/A/C |
| D2. Retrospective written | `ls -la docs/retro/sd36-retrospective.md` (exists); content includes retro events for each retirement (B7, A6 deferral decision, etc.); referenced in `docs/release/SD-36-consolidation/references/README.md` |
| D3. Release notes updated | `ls -la docs/release/SD-36-consolidation/release-notes.md`; baseline table re-derived (each row with its command from plan); before/after src/tests/binaries counts match verified gate output |
| D4. Graphify run | `ls -la docs/release/SD-36-consolidation/receipts.md` (exists); contains `graphify:update` receipt block with exit code |
| D5. PR open and merged | `gh pr list --state merged --base develop \| grep 'tranche/16'` (1+ rows); CI run for merged PR is green (`gh run list --limit 1 --jq '.[] \| select(.name == "Publish tester release") \| .conclusion'` shows "success") |
| D6. Worktree swept | Closure receipt shows `git worktree list` count and `git branch` in main tree; worktrees removed, branches cleaned (tranche/15 deleted) |

---

