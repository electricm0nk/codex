---
title: SD-20 — Boundary Contract (Epic 1)
status: in progress (cycle 1 of Epic 1 landed 2026-07-16; CharacterInput permutations only)
mirrors: /home/ubuntu/workspace/programs/codex/requirements/SD-20-rules-engine-completeness/technical-design.md §1
---

# SD-20 boundary contract

This is the boundary-contract artifact named by
`SD-20-rules-engine-completeness-scope-draft.md` §1.1 and
`technical-design.md` §1.1. It is the engine-side API surface every other
SD-20 epic produces into: the `CharacterInput` shapes the engine accepts,
the `PilotReceipt` shape it returns, and the printed-sheet cell map the
GUI renders from.

Per `technical-design.md` §1.3: no subsystem engine (epics 2–5), no Level
Up grant (epic 7), and no integration closure (epic 8) may introduce a new
`CharacterInput` field or a new `PilotReceipt` field without first
extending this contract and adding the parity test fixture at
`tests/fixtures/wire/sd20/<criterion>.json`.

This artifact lands progressively across Epic 1's cycles (per the loop
instruction's Step 2: `CharacterInput` types first, then `PilotReceipt`
types, then the printed-sheet cell map, then the first parity fixture for
the boundary contract itself). Sections below are filled in as their
owning cycle lands; unfilled sections say so explicitly rather than being
silently absent.

## 1. Inputs — what the engine accepts

Landed (cycle 1, commit pending at time of writing — see
`~/workspace/SD-20-rules-engine-completeness-progress.md` for the
commit SHA).

The engine's existing, SD-19-shaped `CharacterInput` type
(`src/rules_core/character_input.rs`) is the boundary contract's
`CharacterInput` type. This cycle does not introduce a new, parallel
struct — `class_levels: Vec<CharacterClassLevel>` already carries
brand-new, mid-build, and multiclass character state without a shape
change; what the contract adds is a *classification* over that existing
shape, since "which of the three canonical permutations is this input"
was previously only prose.

`src/rules_core/contract.rs` (NEW module) adds:

- `CharacterInputPermutation` — an enum with three variants:
  - `BrandNew` — zero or one class level at level ≤ 1, and no player
    choices recorded yet (no feats, skill ranks, equipment selections,
    spell selections, or other selected choices).
  - `MidBuild` — a single class, but with at least one player choice
    recorded, or a class level above 1.
  - `Multiclass` — more than one entry in `class_levels` (any subset of
    the 11 core classes at any class-level distribution). Multiclass
    takes precedence over mid-build: a multiclass character with feats
    selected still classifies as `Multiclass`.
- `classify_character_input(input: &CharacterInput) -> CharacterInputPermutation`
  — the classification function.

RED/GREEN test: `tests/sd20_contract_character_input.rs` (7 cases, one
per permutation-boundary condition described above).

## 2. Outputs — what the engine returns (`PilotReceipt`)

Not yet landed. A future Epic-1 cycle lands this section alongside the
`PilotReceipt` type in `src/rules_core/contract.rs`, per
`technical-design.md` §1.1 "Outputs": per-derived-stat fields,
per-source-record fields with `TableCellRef` provenance, and diagnostic
fields (`claim_blocking: true` diagnostics preserved). Note the existing
compute surfaces already in this repo — `PilotBaseChassisComputation`
(`src/rules_core/pilot_compute.rs`) and `CorpusPilotReceipt`
(`src/rules_core/pilot_compute_corpus.rs`) — are the load-bearing
precedents this cycle's `PilotReceipt` composes with or wraps, not
shapes it duplicates from scratch.

## 3. Cells — what the GUI prints (printed-sheet cell map)

Not yet landed. A future Epic-1 cycle lands this section: a row-by-row
map of the printed PF1 character sheet, each cell pointing at exactly
one `PilotReceipt` field (once §2 lands). Per `technical-design.md` §1.1,
a cell whose source field is claim-blocked renders "blocked — see
diagnostics" rather than a fabricated value.

## Cross-reference

- `~/workspace/SD-20-rules-engine-completeness-scope-draft.md` §1.1 — Epic 1 acceptance criterion.
- `~/workspace/programs/codex/requirements/SD-20-rules-engine-completeness/technical-design.md` §1 — contract shape, parity test format, cross-cutting authority surface.
- `~/workspace/SD-20-rules-engine-completeness-progress.md` — cycle log (commit SHAs, evidence-tier transitions).
