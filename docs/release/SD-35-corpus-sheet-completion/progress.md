---
canonical: true
owner: god-emporer
bundle_id: SD-35
status: planning-ready — not launched (launch-readiness audit passed 2026-09-08 00:10Z)
date: 2026-09-07
---

# SD-35 Progress

Live cycle-by-cycle record. Cycles **prepend** their entry (newest first) and update
`kanban.md` in the same commit, via `workflow-instruction.md §5`'s retry protocol.

Every cycle entry carries, verbatim from its receipt: the `cycle_scope_gate.py --min 500` line,
the `--receipt` rows (closed / relabeled / rust_lines_changed / ratio / builds_recorded /
pcgen_live_files), and the refused-token remainder. **An entry without the scope-gate line is a
process defect** recorded by the epic wrap-up.

## Open blockers

*(none — an entry here pauses the bundle and is a request for an operator ruling;
`decisions.md §6`)*

## Status matrix

| Epic | Criteria | Complete | In progress | Not started |
|---|---:|---:|---:|---:|
| 1 — Tax cut | 6 | 3 | 0 | 3 |
| 2 — Sheet rule | 5 | 0 | 0 | 5 |
| 3 — Place and surface | 4 | 0 | 0 | 4 |
| 4 — Resolve and verify | 3 | 0 | 0 | 3 |
| 5 — Residues | 5 | 0 | 0 | 5 |
| 6 — PCGen exit | 4 | 0 | 0 | 4 |
| 7 — Closure | 3 | 0 | 0 | 3 |
| **Total** | **30** | **3** | **0** | **27** |

Corpus at the `tranche/15` cut (2026-09-07, `4c6c57eb9f`, identical to authoring at `5f6b18f4e3`):
`DONE=26123 of 49438`; non-DONE 23,315 of 49,438. Live-side PCGen residue at authoring: 78 files by coarse grep
(`content-unit-inventory.md §6`); the exact baseline is AT-35-E1-005's first run. Both
re-measured at the cut by the launch-readiness audit.

## Cycle log

### 2026-09-07 — AT-35-E1-005 cycle 1 — the PCGen residue gate exists, baseline recorded — `complete`

- **Scope gate:** `SCOPE_GATE: EXEMPT (gate-building cycle — this cycle CREATES pcgen_residue_gate.py; closes zero units by design, decisions.md §2)`. `scripts/cycle_scope_gate.py` was absent in the cycle's tree at `53296d80f0` (AT-35-E1-001 landed on `origin/tranche/15` while this cycle ran; the receipt rows below were re-derived with it after the rebase).
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=1 pcgen_live_files=260` — see the receipt for the mechanical `cycle_scope_gate.py --receipt` line run after the rebase, and the hand commands it agrees with.
- **PCGen residue (first recording, `53296d80f0`):** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` (`python3 scripts/pcgen_residue_gate.py --check`). `identifier_files=68 identifier_hits=514` of those 260 files are the five-identifier readers the authoring-time "78 files" figure was counting (that grep scanned `apps/desktop/src-tauri/src` only and included `cache_gen/`); the rest is token-syntax literals, 8,078 `PRE[A-Z]+:` + 2,403 `BONUS:` hits of 12,736, almost all in generated `src/rules_core/rules_tables/**`. Correction event `1788831658230-at-35-e1-005-0d124e`.
- **RED→GREEN on the real tree:** planted `src/rules_core/zz_planted_residue_probe.rs` (`r.raw_tokens.len()`) → `live_files=261 live_hits=12737 baseline_files=260 baseline_hits=12736 verdict=FAIL_INCREASED` exit 1; removed → `verdict=PASS` exit 0. `--check --closure` at the baseline → `live_files=260 live_hits=12736 verdict=FAIL` exit 1. `--rebaseline` at the baseline → `rebaseline=REFUSED ... verdict=FAIL_NOT_REDUCED` exit 1.
- **verify.sh:** stage `pcgen-residue-gate` in both sets after `figure-provenance` — `scripts/verify.sh --list` → 42 stages (36 quick) with AT-35-E1-001's `cycle-scope-gate-selftest` landed first; `--only pcgen-residue-gate` → `PASS`; `PCGEN_RESIDUE_GATE_CLOSURE=1 ... --only pcgen-residue-gate` → `FAIL` (closure mode, for AT-35-E6-004). `scripts/tests/test_pcgen_residue_gate.py` → `Ran 15 tests OK` (RED first: `ModuleNotFoundError`).
- **Build:** `cargo test --locked --no-run -j 6` exit 0; `--lib` 3186 passed / 0 failed (= floor); `--no-fail-fast`, desktop, frontend, clippy not run — no Rust touched (`git diff --stat 53296d80f0 -- '*.rs'` empty). Fast gates green: atlas, shape-engine-boundary, missing-engine-tables, pi-sweep; `data/sheet_rules/` grep → 0 (directory not yet created). **`denominator_gate.py --check` over `*.md` + `artifacts/**/*.md` is RED on inherited prose:** `files_checked=22 violations=11`, all 11 in `artifacts/epic-2-sheet-rule/token-mapping/` (committed `a232e27b03`, pre-cycle, outside Epic 1's touch set); this cycle's own three files → `violations=0`. Incident event emitted (the same finding AT-35-E1-001 and AT-35-E1-006 recorded); owner: the token-mapping synthesis / AT-35-E1-004.
- **Refused tokens:** none. Receipt: `artifacts/epic-1-tax-cut/AT-35-E1-005_cycle1_receipt.md` (names the code and docs SHAs); transcript: `artifacts/epic-1-tax-cut/pcgen-residue-first-run.txt`.

### Cycle — AT-35-E1-006 cycle 1 — SD-34's unrun closure folded: retrospective written and cited, 17 open rows and 29 open deferrals dispositioned — complete (2026-09-08)

**Status: complete.** Docs only; zero units moved by design. Work commit `9cc73dca76`, receipt
commit `800c363e42`; receipt `artifacts/epic-1-tax-cut/AT-35-E1-006_cycle1_receipt.md`.

- **Scope gate:** `SCOPE_GATE: EXEMPT (docs-only fold of SD-34's closure epilogue — closes zero units by design, decisions.md §2 and §12)` — `cycle_scope_gate.py` was absent at cycle start (`53296d80f0`) and arrived via AT-35-E1-001 on the pre-push rebase.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=unavailable` (`cycle_scope_gate.py --receipt --since 53296d80f0 --target-dir /tmp/cargo-sd35-AT-35-E1-006`; `pcgen_residue_gate.py` absent — AT-35-E1-005 builds it; coarse-grep stand-in 78 files, `content-unit-inventory.md §6`).
- **Refused tokens:** none.
- **Evidence:** `test -f docs/retro/sd34-book-completion-retrospective.md` → present; `grep -c sd34-book-completion-retrospective` → 1 in each `references/README.md`; row map 1,590 of 1,590 (core_rulebook 1,529 of 6,701 + ultimate_campaign 61 of 265, `completion_atlas.py --book <book> --check`) sum-checked; SD-34 `progress.md` `status: closed-by-fold`; 29 of 29 deferrals dispositioned (8 resolved with a SHA, 1 superseded by register C2.5, 20 mapped) — `retro.py summary --since 2026-08-27` now reads `deferrals.open=20`, all SD-35-owned.
- **Gates:** `completion_atlas.py --check` 0; `shape_engine_boundary.py --check` 0; `missing_engine_tables.py --check` 0; `verify.sh --only pi-sweep` PASS; **`denominator_gate.py --check` red — 11 violations in 4 pre-existing `artifacts/epic-2-sheet-rule/token-mapping/*.md` files this cycle did not touch** (incident `1788831974625-at-35-e1-006-ae4135`; owner: the token-mapping synthesis / AT-35-E2-001).
- **Carried one-liner** (`SD-34 forward-scope-register.md` C1.8, `358a71516f`): `monk_ki_pool` "size"-suffix — unit is bucket C at the cut, owned by AT-35-E3-003; not applied here (docs only).
- **Operator attention:** the two fable-review P1s (R11-01, R14-02) are unfixed and outside SD-35 scope per `forward-scope-register.md` C2.5 — dispositioned `superseded` (register), not resolved.

### 2026-09-07 — AT-35-E1-001 cycle 1 — `batch-floor-gate` — complete

- **Scope gate:** `SCOPE_GATE: EXEMPT (gate-building cycle — this cycle CREATES cycle_scope_gate.py; it closes zero units by design, decisions.md §2)`
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=unavailable` (`pcgen_residue_gate.py` not yet in the tree — AT-35-E1-005)
- **Refused tokens:** none
- **Landed:** `scripts/cycle_scope_gate.py` (floor + `--receipt`), `scripts/tests/test_cycle_scope_gate.py` (51 cases), `verify.sh` stage `cycle-scope-gate-selftest` (stage count 40 → 41; `scripts/verify.sh --list`). Live at HEAD: `--bucket B --kind class_feature` → `scoped=7866 remaining_non_done=23315 verdict=PASS`; `--bucket A --kind companion` → `scoped=28 ... FAIL_UNDER_FLOOR` exit 1 (`python3 scripts/cycle_scope_gate.py --min 500 ...`).
- **Found, not fixed (outside file-touch set):** `denominator_gate.py --check` on the package is `violations=11` of `files_checked=21`, all in 4 pre-launch `artifacts/epic-2-sheet-rule/token-mapping/*.md` files — retro incident `denominator-gate-red-on-package-prose`; owner AT-35-E1-004.
- **Receipt:** `artifacts/epic-1-tax-cut/AT-35-E1-001_cycle1_receipt.md` — code at `1d821cdc8d`.
