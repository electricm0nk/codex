---
canonical: true
owner: operator
bundle_id: SD-36
date: 2026-09-15
---

# SD-36 Content-Unit Inventory

**NOTE:** SD-36 is a code consolidation bundle, not a content-unit (corpus records) bundle. The "units" here are code artifacts (files, lines, test suites, binaries) and crate boundaries, not character-sheet items. The baseline figures are measured in file counts, line counts, and test-suite counts.

---

## Epic-wise unit breakdown

### Epic B — Freeze status, retire producers

**Scope:** Delete all machinery that feeds the PF1e status dashboard.

| Item | Type | File count | Line count |
|---|---|---|---|
| v06_work_inventory.rs | binary/module | 1 | 33,091 |
| support_state_matrix.rs | module | 1 | 7,475 |
| support_state_matrix_bridge.rs (desktop) | Rust component | 1 | ~1,500 |
| reach_gate.rs (desktop) | Rust component | 1 | 8,846 |
| fixture_verified_oracle_probe.rs | binary | 1 | ~1,200 |
| Test files (sd13_support_state_matrix, v06_work_inventory, race-bounded, desktop bridge, atlas, site vocab) | test files | 9 | ~2,500 |
| Desktop UI components (SupportDebtPanel, BreadthClaimAuditPanel) | TypeScript/TSX | 2+ files | ~1,500 |
| Scripts (publish-site-dashboard.sh, shape_engine_boundary.py, completion_atlas.py) | scripts | 3 | ~1,500 |
| Documentation | markdown | 3 | ~500 |
| **Epic B total** | | **19+ files** | **~55,827 lines** |

### Epic A — PCGen wall: crate codex-ingest

**Scope:** Move 47 tool binaries, ~82 tool test suites, and supporting modules into `crates/codex-ingest`. Create crate boundary and wall.

| Item | Type | File count | Line count |
|---|---|---|---|
| Tool binaries (moved from `src/bin/`) | binaries | 47 | ~15,000 |
| Tool test suites (moved from `tests/`) | test files | ~82 | ~18,000 |
| pcgen_import module and submodules | module tree | ~50 | ~40,000 |
| oracle_validation module | module | ~10 | ~8,000 |
| bar_check bin (moved/new) | binary | 1 | ~500 |
| Crate scaffold (Cargo.toml, lib.rs) | manifest | 2 | ~200 |
| Test-module evictions from src/rules_core | test modules | 36 | 161 |
| SourceRef.lst_file → source_path rename | field rename | 57 test files | ~200 changes |
| Desktop dev-dep rewrite (9 imports) | rewrite | 9 files | ~50 changes |
| **Epic A total** | | **~290 files touched** | **~82,000 lines moved/changed** |

### Epic C — Bloat cuts

**Scope:** C1 (source) + C2 (tests).

#### Pass C1 — Source refactor

| Item | Type | File count | Line count |
|---|---|---|---|
| pilot_compute/mod.rs (split into submodules) | module split | 37 files (1 parent + 36 submodules) | 88,828 → 4,000 (mod.rs) + 84,000 distributed |
| src/support/paths.rs (new consolidated) | module | 1 | ~150 |
| Path helper deletions (8 sites) | deletions | 8 | ~150 |
| Clippy lock amendment | verify.sh change | 1 | ~5 |
| **Pass C1 total** | | **~37 files** | **88,828 lines refactored** |

#### Pass C2 — Test rewrite

| Item | Type | File count | Line count |
|---|---|---|---|
| tests/sd18_widening/ table-driven rewrite | test rewrite | ~50 files (rosters + rows.rs + per-row) | ~50,000 → ~15,000 |
| tests/sd13_progression/ table-driven rewrite | test rewrite | ~50 files (rosters + rows.rs + per-row) | ~25,000 → ~11,000 |
| tests/common/ path helpers (deleted) | deletions | 59 | ~150 |
| Branch promotion test move | file move | 1 | ~50 |
| #[ignore] audit (oracle tests + evidence) | audit | 65+ tests kept, 2 deleted | ~100 |
| **Pass C2 total** | | **~165 files** | **~75,000 → ~26,000 lines** |

### Epic D — Closure

**Scope:** Retrospective, architecture-docs refresh, release notes, graphify, PR.

| Item | Type | Files | Content |
|---|---|---|---|
| Retrospective (SD-36) | markdown | 1 | ~1,000 lines |
| Architecture-docs refresh | markdown | 6+ | ~500 lines (updates across overview, rules-engine, desktop-app, status, testing, conventions) |
| Release notes | markdown | 1 | ~200 lines (baseline table + commentary) |
| Receipts | YAML | updates to existing file | ~100 lines (graphify, architecture truth-up, PR metadata) |

---

## Baseline summary (from plan verified table, §1 of acceptance-and-verification.md)

Re-derive at the end of each epic pass:

| Baseline | Value (SD-35) | Command | Expected change (SD-36) |
|---|---|---|---|
| src lines | 465,469 | `find src -name '*.rs' -exec cat {} + \| wc -l` | down ~40k (B deletions, C1 split doesn't change totals) |
| tests lines | 182,070 | `find tests -name '*.rs' -exec cat {} + \| wc -l` | down ~49k (C2 rewrite, B deletions) |
| test entries | 8,723 | `cargo test --locked -- --list \| wc -l` | unchanged or ±5 (additions for crate-ingest) |
| root binaries | 419 | `scripts/verify-baselines.env` line | down to 412 (B: -7, A: -47) |
| root full tests | 8,926 | `scripts/verify-baselines.env` line | TBD (re-derive at epic-end gate) |
| ingest binaries (new) | 0 | (did not exist) | 47 (A moves these) |
| ingest tests (new) | 0 | (did not exist) | ~82 (A moves these) |

---

