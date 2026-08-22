---
canonical: true
owner: god-emporer
status: planning-ready (chassis completed 2026-08-22)
date: 2026-08-22
---

# SD-32 Artifacts — Index

Per-cycle evidence lives here. Each cycle writes one receipt file under the appropriate gate
subdirectory. The schema lives in `loop-instruction.md §7`.

## Subdirectories

| Directory | Purpose | Owner |
|---|---|---|
| `HANDOFF.md` | The SD-31 → SD-32 session handoff (captured 2026-08-22). Read first. | operator (do not modify) |
| `UNMERGED-BRANCHES.md` | Ten branches at the `tranche/11 → tranche/12` boundary, with recommended disposition order | operator (do not modify until disposition is decided) |
| `gate-0-census-closure/` | Gate 0 cycle receipts + `excluded-directories.md` (per AT-32-G0-001) | loop (created on first Gate 0 cycle) |
| `gate-1-shape-closure/` | Gate 1 cycle receipts + `ledger.json` + `families.md` (proof width per family) | loop |
| `gate-2-engines/` | Gate 2 cycle receipts + per-engine `*.corpus-wide.json` outputs + `*.fixtures.json` | loop |
| `gate-3-closure-invariant/` | Gate 3 cycle receipts + standing-gate receipt + named-corpus-SHA proof | loop |
| `epic-5-protective-sweep/` | Epic 5 cycle receipts (self-erasure check across all Rust generators) | loop (created on first Epic 5 cycle) |

## Reading order for a fresh session

1. `HANDOFF.md` — five operator-pattern footguns, two theses refuted, what is immediately
   actionable. This is the load-bearing context the SD-31 session had that isn't written
   anywhere else.
2. `UNMERGED-BRANCHES.md` — what work exists at the boundary that hasn't been claimed yet.
3. The most-recent gate's `*_cycle_receipt.md` files — what's been done, what's still open.
4. `progress.md` (in the bundle root) — the time-ordered audit trail.

## What is intentionally NOT in `artifacts/`

- The corpus-side artifacts (per-book fixture files, per-engine expected-value corpora) live at
  `$PCGEN_CORPUS_ROOT`, not here. `artifacts/` is per-cycle evidence, not corpus data.
- The standing tools (`scripts/coverage_ledger.py`, `scripts/derived_evaluator_fixture_check.py`,
  the new `scripts/census_independent.py`, `scripts/shape_ledger.py`, and
  `scripts/shape_coverage_standing_gate.py` that this bundle builds) live under `scripts/`, not
  here. Tools and their receipts are separate surfaces.
- Cross-SD carry-forward items (`forward-scope-register.md`'s C1.8, C1.9) point at SD-31's own
  artifacts where relevant. The SD-31 work is not duplicated here.

## Self-heal rule for `artifacts/`

A cycle that needs to add a new artifact type creates the subdirectory at write-time with a
frontmatter-only `README.md` inside it (a single section: "What lives here" + the schema), and
appends the row to the table above. Stale subdirectories (no cycles for 5+ waves, gate declared
closed) get a `STATUS: ARCHIVED` marker in their README and remain on disk for audit; nothing
is deleted at gate close.
