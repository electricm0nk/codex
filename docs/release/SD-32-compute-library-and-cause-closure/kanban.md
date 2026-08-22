---
canonical: true
owner: god-emporer
status: planning-ready (chassis completed 2026-08-22; claim-priority order)
date: 2026-08-22
---

# SD-32 Kanban (local-file)

This is the **local-file** kanban for SD-32, paired with `progress.md` (no Hermes board; per
`SD-30-class-feature-archetype-bundle/decisions.md` Decision 14a, retired 2026-08-01). Each card
is one row; cycles update the `Status` column in place and append the receipt to `progress.md`.

Cards are ordered by **claim priority**, not by epic number. The pre-Epic-5 work (Epic 5's
protective sweep) fires first because scaling Gate 2 engines over an unchecked generator is the
failure class Gate 2 depends on not existing. After Epic 5, the gate ordering from
`loop-instruction.md §3` applies: G0 → G1 → G2 → G3.

| # | Card ID | Gate / Epic | Title | Status | Cycle | Notes |
|---|---|---|---|---|---|---|
| 1 | `epic-5-protective-sweep` | Epic 5 (pre-G0) | Self-erasure check across all Rust generators | pending | — | Fires before Gate 0. See `artifacts/HANDOFF.md`: 3 of 12 SD-31-checked generators vulnerable; ~30 unchecked. |
| 2 | `gate-0-census-closure` | Gate 0 | Build `scripts/census_independent.py`; diff against inventory; per-kind object-definition rules | pending | — | New walker, mirror of LST reader; AT-32-G0-001/002/003. |
| 3 | `gate-0-book-onboarding-precondition` | Gate 0 / Epic 4 | Onboard the 4 unbuilt books | pending | — | Sequenced behind Gate 0 census walk; AT-32-G0-003 binding. |
| 4 | `gate-1-shape-closure` | Gate 1 | Build `scripts/shape_ledger.py`; close every unit into one of F1..F10 | pending | — | AT-32-G1-001/002/003. Vocabulary extension allowed with measured units. |
| 5 | `gate-2-engines-f1-f9` | Gate 2 | Confirm `formula_interpreter.rs` reaches all 9 in-scope families with fixtures | pending | — | AT-32-G2-001/002/003/004. |
| 6 | `gate-2-engines-f10-binding` | Gate 2 | Generalise `bonus_stack_reader.rs` for F10 binding-layer family | pending | — | Reaches 77.2% of custom identifiers per SD-31 wave 31 measurement. |
| 7 | `gate-2-corpus-wide-runs` | Gate 2 | Per-engine corpus-wide run with fixture check | pending | — | AT-32-G2-004; one cycle per engine. |
| 8 | `gate-3-closure-invariant` | Gate 3 | Build standing gate; wire into `scripts/verify.sh` | pending | — | AT-32-G3-001/002/003. |
| 9 | `epic-1-compute-library` | Epic 1 | Build the library from proven code (F1/F2/F3 of `epic-breakdown.md`) | pending | — | Behind G1+G2 by construction. |
| 10 | `epic-2-cause-closure` | Epic 2 | Close T2a/T2b/T3/T4/T5/T9/T12 by class | pending | — | Behind G1+G2 by construction. |
| 11 | `epic-3-class-reachability` | Epic 3 | 77 prestige-class gating; 18 untabled base classes | pending | — | Behind Gate 0 by construction (chassis-blocked). |
| 12 | `closure-epilogue` | Closure | Architecture-docs refresh, `tranche/12 → develop` PR, release-notes population | pending | — | Fires LAST. |

## Status values

- `pending` — card exists, no cycle has claimed it.
- `in-progress` — a cycle has claimed it; cycle receipt pending.
- `complete` — cycle receipt is in `progress.md` and the gate's AT-32-* criteria are met.
- `returned-to-backlog` — cycle ran, hit a non-self-healable blocker; receipt filed under
  `## Open blockers` in `progress.md`.
- `DISCOVERED-forked` — cycle found work that didn't fit the card; new card opened in the
  discovery row above (or in `forward-scope-register.md` if it doesn't fit SD-32).

## Cross-SD gate discipline

There are no cross-SD gates for SD-32 to cite at launch (no sibling bundle is the precondition;
the dependency on SD-31 is the closure PR being merged to develop, which is verified by
`loop-instruction.md §1.3` rather than a per-cycle PI-screen citation). The PI-gate and
declared-PI reader are not consumed here because SD-32 does not produce records that ship into
the player-facing app in its own right — the engines it builds are surfaced via
`reach_gate.rs` from the existing ingest path that already carries the PI-gate.

## Why no Hermes kanban board

The Hermes kanban board was retired 2026-08-01 (`SD-30-class-feature-archetype-bundle/decisions.md`
Decision 14a); SD-30 and its successors use the local-file pattern. This is documented at
`loop-instruction.md §1.1`, `loop-instruction.md §2`, and `loop-instruction.md §6.8`.
