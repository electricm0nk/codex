---
canonical: true
owner: operator
bundle_id: SD-36
date: 2026-09-15
---

# SD-36 Technical Requirements

Pre-execution prerequisites and normative requirements.

---

## Pre-execution checklist

- [ ] SD-35 merged to develop (PR #390 merged at 50572eebad).
- [ ] Hotfix PR (fix/pcgen-pinned-tree-ci-guard) merged to develop and CI green.
- [ ] `origin/develop` is at the hotfix commit (CI run green).
- [ ] `origin/tranche/16` branch exists or will be cut from `origin/develop`.
- [ ] Box has ≥ 300G free disk (for builds and test artifacts).
- [ ] At most 1 active `git worktree` on the shared checkout (this tree is the only writer).
- [ ] `git status --porcelain` in main tree is clean (no stray commits or uncommitted work).

---

## Build and test infrastructure

### Cargo and compilation

- **Workspace:** Root `Cargo.toml` defines workspace at planning time; after Epic A, `[workspace] members = ["crates/*"]` added.
- **Lockfiles:** Two lockfiles (root + `apps/desktop/src-tauri/Cargo.lock`). Both refreshed at Epic A completion.
- **Manifest limits:** No circular dependencies. Type-identity trap: if `codex` dev-depends on `codex-ingest`, the trap triggers on dual compilation. Build gate enforces this.
- **Parallel builds:** Max 2 cargo lanes on this box per policy (`~/.cargo/config.toml` caps `jobs = 6`). One lane per epic pass.
- **Cold-start build time:** ~2 min 45 sec (warm cache ~20 sec). Allocate 30+ min for full `verify.sh` run (Epic B: 40 stages, +2 after A, reorder for C2 test changes).

### Verification stages

- **Per-cycle:** `cargo test --locked --no-run`, scoped suites, fast Python gates.
- **Per-epic:** Full `bash scripts/verify.sh` run with current stage list.
- **Baselines:** Updated in `scripts/verify-baselines.env` per epic (B, A, C-end); recorded in `release-notes.md`.

---

## Code-level constraints

### Epic B (Freeze + retire)

- **Deletion discipline:** 8 binaries + modules, 9 test files, 3 scripts → deleted, not stubbed.
- **Desktop UI:** Components removed from DOM tree, wiring deleted, no "unavailable" fallback UI.
- **Documentation:** Retired tools documented as "Retired in SD-36" with date; not marked "pending."
- **Tests:** All `#[test]` deletions are explicit (grep confirms the count drops).

### Epic A (Crate wall)

- **Crate structure:** `crates/codex-ingest/src/lib.rs` exports public modules; no re-export of test-only items.
- **Desktop dep:** `codex-ingest` listed ONLY under `[dev-dependencies]`, never `[dependencies]`.
- **Path rewrites:** `CARGO_MANIFEST_DIR` → `repo_root()` with zero remaining in crate (git grep enforces).
- **Test eviction:** 36 `#[cfg(test)]` modules in `src/rules_core` are routed to moved tests or fixture-ized; codex NEVER dev-depends on codex-ingest.

### Epic C1 (Source refactor)

- **Module split:** Moves only, no logic changes. All 62 call sites keep working via `pub use x::*;` in mod.rs.
- **Self-reading test:** `include_str!("mod.rs")` updated to point to the correct file.
- **Clippy:** New `-- -D warnings` flag in `clippy_one_crate()` function.

### Epic C2 (Test rewrite)

- **Table structure:** `const ROWS: &[Row]` per test family; `paste` macro emits `#[test]` per row.
- **Entry count:** `cargo test --locked -- --list` diff must show byte-identical or only expected additions/removals.
- **Oracle tests:** 65 `#[ignore]` oracle tests kept; ran once per pass with real PCGen corpus.

---

## Verification gates and commands

| Gate | Command | When | Pass criteria |
|---|---|---|---|
| Compile | `cargo test --locked --no-run -j 6` at root and desktop | Per epic | exit 0 |
| Residue (PCGen) | `python3 scripts/pcgen_residue_gate.py --check --closure` | Per epic | exit 0; `lst_file` 0/0, `codex_ingest` 0/0 |
| Frozen status | `scripts/site/check_frozen_status.py --check` | Epic B | exit 0; 49,450 / 49,450 |
| Wall | `cargo tree`, `cargo metadata`, manifest check, residue gate | Epic A | 0 codex_ingest in build-deps; wall-stage green |
| Full verify | `bash scripts/verify.sh` | Per epic | all stages green; every FAILED attributed to a suite |
| Test list | `cargo test --locked -- --list` | Per epic (C2 especially) | diff artifact; byte-identical or documented change |
| Disk | `df -h /` | Epic D | ≥ 850G free |

---

## Deferred from SD-36 (forward scope)

- **Semantic dedups in pilot_compute** (fable review PC5-3/PC6-1/PC2-3) — design work, not in this bundle.
- **SD-34 fable-review P1 correctness rows** (C2.5) — separate correctness bundle.
- **rules_tables→data package** (181,797 lines) — Starfinder bundle.
- **pf1e_dashboard_producer.py extraction** — if not complete during Epic B, recorded as Epic B7 deferral in retro.

---

