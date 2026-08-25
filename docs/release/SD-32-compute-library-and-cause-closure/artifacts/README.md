---
canonical: true
owner: god-emporer
status: planning-ready (chassis completed 2026-08-22)
date: 2026-08-22
---

# SD-32 Artifacts — Index

Per-cycle evidence lives here. Each cycle writes one receipt file under the appropriate gate
subdirectory. The schema lives in `workflow-instruction.md §7`.

## Subdirectories

| Directory | Purpose | Owner |
|---|---|---|
| `HANDOFF.md` | The SD-31 → SD-32 session handoff (captured 2026-08-22). Read first. | operator (do not modify) |
| `UNMERGED-BRANCHES.md` | Ten branches at the `tranche/11 → tranche/12` boundary, with recommended disposition order | operator (do not modify until disposition is decided) |
| `corpus/` | **The repo-local PCGen oracle slot.** `corpus/operator-supplied/pcgen/` holds the oracle at the pin (git-ignored; only the two READMEs ship). The only oracle location this bundle references — see `corpus/README.md`. | operator (populated via `scripts/fetch-pcgen-oracle.sh --dest`) |
| `gate-0-census-closure/` | Gate 0 cycle receipts + `excluded-directories.md` + `diff.json` (per AT-32-G0-001) | loop (dir created at launch, 2026-08-22) |
| `gate-1-shape-closure/` | Gate 1 cycle receipts + `ledger.json` + `families.md` (proof width per family) | loop (dir created at launch) |
| `gate-2-engines/` | Gate 2 cycle receipts + per-engine `*.fixtures.json`, `*.corpus-wide.json` outputs, and `*.expected.json` expected-value files (transcribed from oracle bytes the engine does not read) | loop (dir created at launch) |
| `gate-3-closure-invariant/` | Gate 3 cycle receipts + standing-gate receipt + named-corpus-SHA proof | loop (dir created at launch) |
| `epic-5-protective-sweep/` | Epic 5 cycle receipts (self-erasure check across all 29 Rust generators) | loop (dir created at launch) |

## Reading order for a fresh session

1. `HANDOFF.md` — five operator-pattern footguns, two theses refuted, what is immediately
   actionable. This is the load-bearing context the SD-31 session had that isn't written
   anywhere else.
2. `UNMERGED-BRANCHES.md` — what work exists at the boundary that hasn't been claimed yet.
3. The most-recent gate's `*_cycle_receipt.md` files — what's been done, what's still open.
4. `progress.md` (in the bundle root) — the time-ordered audit trail.

## What is intentionally NOT in `artifacts/`

- Nothing PCGen-related lives outside the repo. The oracle input is in `corpus/operator-supplied/pcgen`
  (git-ignored slot); bundle-produced fixture and expected-value files are in the per-gate
  directories above. `$PCGEN_CORPUS_ROOT` is always the slot (`workflow-instruction.md §2.1`), never
  `~/workspace/repos/pcgen` (operator directive 2026-08-22).
- The standing tools (`scripts/coverage_ledger.py`; the Rust fixture-check gate
  `src/bin/derived_evaluator_fixture_check.rs` with its library module
  `src/rules_core/derived_evaluator_fixture_check.rs` and fixture
  `tests/fixtures/rules_core/derived-evaluator-fixtures.json`; and the new
  `scripts/census_independent.py`, `scripts/shape_ledger.py`, `scripts/shape_coverage_standing_gate.py`
  that this bundle builds) live under `scripts/` / `src/` / `tests/`, not here. Tools and their
  receipts are separate surfaces.
- Cross-SD carry-forward items (`forward-scope-register.md`'s C1.8, C1.9) point at SD-31's own
  artifacts where relevant. The SD-31 work is not duplicated here.

## Self-heal rule for `artifacts/`

A cycle that needs to add a new artifact type creates the subdirectory at write-time with a
frontmatter-only `README.md` inside it (a single section: "What lives here" + the schema), and
appends the row to the table above. Stale subdirectories (no cycles for 5+ waves, gate declared
closed) get a `STATUS: ARCHIVED` marker in their README and remain on disk for audit; nothing
is deleted at gate close.
