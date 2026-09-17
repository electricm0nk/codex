---
canonical: true
owner: operator
bundle_id: SD-36
date: 2026-09-15
---

# SD-36 Technical Design

Architectural rationale for the three major moves: dashboard freeze, crate wall, and bloat cuts.

---

## §1 — Dashboard freeze and producer retirement

### Problem

The PF1e status dashboard and all supporting machinery (work inventory bins, support state matrix, reach gate, desktop bridge) exist only to provide one feedback surface: "How complete is the content for each book?" That question was answered at **100% complete** (49,450 of 49,450 units) by the end of SD-35.

The supporting machinery has grown:
- `v06_work_inventory.rs` — 33,091 lines
- `support_state_matrix.rs` — 7,475 lines
- `reach_gate.rs` (desktop) — 8,846 lines
- Desktop bridge code — ~1,500 lines
- Cron jobs and dashboard renderer — ~2,000 lines
- Test overhead — ~2,500 lines
- **Total: 55,827 lines to maintain for a static answer.**

### Solution

Freeze the status page at 100% complete. The page (`site/status.html`, `site/status-data.json`) is regenerated once per bundle closure and never updated in live work. This:
1. Deletes the 55,827 lines of supporting code.
2. Removes the per-cycle "denominator" question from the engine.
3. Simplifies the public interface.
4. Keeps the frozen status for historical reference and user communication.

### Enforcement

New gate `scripts/site/check_frozen_status.py` asserts:
- Denominator is exactly 49,450 units (every unit in the DONE statuses from `docs/work-inventory.json`).
- `overall.pct == 100.0`.
- All partial and not_started counts are 0.
- `generated_at` timestamp is constant.

The gate runs per cycle and after closure before publication.

---

## §2 — PCGen crate wall

### Problem

PCGen machinery (converter, oracle, tool binaries, tool tests) exists only for content ingest. It is never used by the desktop app or the rule engine at runtime. However:
1. It is scattered across `src/pcgen_import/`, `src/oracle_validation/`, tool bins in `src/bin/`, and tool tests in `tests/`.
2. The desktop app accidentally links it via shared `src/` structure (no explicit boundary).
3. Gate blind spots make it hard to enforce "nothing of PCGen in live code."

### Solution

Move all PCGen machinery into dedicated `crates/codex-ingest`. Structure:

```
crates/codex-ingest/
├── Cargo.toml           (no dep on desktop app; depends on codex)
├── src/lib.rs           (exports pcgen_import, oracle_validation, bar_check)
├── src/pcgen_import/    (moved from src/)
├── src/oracle_validation/
└── tests/               (moved from tests/; ~82 suites)

Workspace root:
├── Cargo.toml           (adds members = ["crates/*"])
└── apps/desktop/src-tauri/Cargo.toml  (codex-ingest ONLY in [dev-dependencies])
```

### Enforcement

**Build gate (crate-wall stage):**
1. `cargo tree --locked -e normal,build` in desktop shows 0 lines starting `codex-ingest ` (build-deps only).
2. `cargo metadata --no-deps` shows `codex-ingest` depends on `codex`; never the reverse.
3. Desktop manifest awk: `codex-ingest` appears only under `[dev-dependencies]`.
4. `python3 scripts/pcgen_residue_gate.py --check` exits 0.

If any check fails, the build stops. The gate cannot be bypassed.

**Type-identity trap mitigation:** If `codex` dev-depends on `codex-ingest` (forbidden), Rust compiles `codex` twice: once for the lib and once for the bin/test, with different `CARGO_MANIFEST_DIR` values. The ingest module sees inconsistent symbols. The `cargo metadata` check detects this.

---

## §3 — Bloat cuts: source refactor and test rewrite

### Problem

Two files have grown to maintenance hazard size:
- `src/rules_core/pilot_compute/mod.rs` — 88,828 lines (one file)
- Tests with table-driven logic — ~75,000 lines (2 families, 2,219 entries)

Both are structurally sound but hinder readability and IDE performance.

### Solution

#### Pass C1: Split `pilot_compute/mod.rs`

Move logic into ~36 submodules by class/system:
- `class_*.rs` — ~12 class-specific modules, ~3–6k lines each
- `race_seams.rs`, `combat.rs`, `feat_pillars.rs`, `skills_and_saves.rs`, `spellcasting.rs`, `pool_groups.rs`, `companion.rs`, `untabled_base_class_features.rs`, `prestige_class_features.rs`, `class_dispatch.rs`, `class_unchained.rs`
- `mod.rs` → ~4,000 lines (re-exports via `pub use x::*;` to keep all 62 call sites working unchanged)

No logic changes, only moves. Largest submodule ≈ 6,600 lines.

#### Pass C2: Table-driven test rewrite

Consolidate test families (`tests/sd18_widening/`, `tests/sd13_progression/`) into:
- Roster file + per-row module files (structure unchanged for `--list` identity).
- New `rows.rs` with `const ROWS: &[Row]` per family.
- Macro (`paste!`) emitting one `#[test]` per row.
- Bespoke tests (~770) remain verbatim.
- Result: ~75,000 lines → ~26,000 lines (entry count unchanged: 2,219 / 8,723 total).

### Enforcement

1. **C1 proof:** `cargo test --locked -- --list` byte-identical (modules split but re-exported; all paths work).
2. **C2 proof:** `cargo test --locked -- --list` diff artifact showing same entries (table-driven rewrite proves test structure preserved).
3. Both passes: Full `bash scripts/verify.sh` green.

---

## §4 — Path helper consolidation

### Problem

Eight locations each define `repo_root()`, `corpus_root()`, and related path fns independently:
- `src/bin/` (~3 copies)
- `src/rules_core/` (~2 copies)
- `crates/codex-ingest/` (added in A; if defined locally, would be 9 copies)
- `apps/desktop/` (~3 copies)

Each has subtly different error handling or assumptions.

### Solution

New `src/support/paths.rs` with canonical definitions + unit tests:
```rust
pub fn repo_root() -> PathBuf { /* single canonical impl */ }
pub fn corpus_root() -> PathBuf { /* single canonical impl */ }
pub fn corpus_root_if_set() -> Option<PathBuf>
pub fn pcgen_corpus_root() -> PathBuf
pub fn find_json_files(dir: &Path) -> Vec<PathBuf>
```

All eight locations updated to `use codex::support::paths::*;` instead of local definitions.

### Enforcement

Gate: `git grep -w 'fn repo_root' | wc -l` exits with count = 1 (only in `src/support/paths.rs`). Same for `corpus_root` and `find_json_files`.

---

## §5 — Clippy enforcement

### Problem

The build does not enforce `deny(warnings)` globally. Lint warnings accumulate; no systematic pressure to fix them.

### Solution

Add `-- -D warnings` to `clippy_one_crate()` function in `scripts/verify.sh`. This makes clippy failures (warnings) exit non-zero, so CI catches them. Run at Epic C1 end; baseline set to 0 warnings.

### Enforcement

Gate: `bash scripts/verify.sh` runs clippy stages (one per crate after C1); any warning = non-zero exit. Baseline `BASELINE_CLIPPY_WARNINGS_*` set to 0.

---

