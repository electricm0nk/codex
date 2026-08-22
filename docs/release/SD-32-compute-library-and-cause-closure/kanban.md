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
failure class Gate 2 depends on not existing. After the Pre-G0 phase (cards 1-2), the gate ordering
from `workflow-instruction.md §3` applies: G0 (card 3, then card 4) → G1 (card 5) → G2 (cards 6/7,
each followed by its card-8 corpus-wide run) → G3 (card 9) → Epics 1-3 (cards 10-12) → Closure
(card 13). `workflow-instruction.md §2.4` is the dispatch script for exactly this order. **Closure
fires on the Definition of Done (all four gates' AT-32-* criteria met), never on a wave budget.**

| # | Card ID | Gate / Epic | Title | Status | Cycle | Notes |
|---|---|---|---|---|---|---|
| 1 | `epic-5-protective-sweep` | Epic 5 (pre-G0) | Self-erasure check across all 29 Rust generators | complete | 1 | Population re-verified 29. Fixed 7 vulnerable generators: the 2 SD-31 D9 binaries (`gen_book_cache.rs`'s `gen_advanced_race_guide`/`gen_companion_book`/`gen_pathfinder_unchained`, `gen_core_rulebook_cache.rs`) plus 5 newly found in the 17-never-checked bucket (`gen_cache_acg`, `gen_cache_apg`, `gen_cache_beastiary` [SD-31's "safe" verdict corrected], `gen_cache_spell_lane_dump`, `gen_cache_ultimate_equipment`). Live RED→GREEN for the binaries, unit-test RED→GREEN for the `cache_gen::*` modules. Receipt: `artifacts/epic-5-protective-sweep/cycle-1_cycle_receipt.md`. |
| 2 | `boundary-branch-review` | Pre-G0 (housekeeping) | Review and disposition the 3 orphaned-but-real branches from `artifacts/UNMERGED-BRANCHES.md` § "Real work orphaned by an orchestrator decision" | complete | 1 | **Runs in the primary checkout** — nine of the ten branches are local-only (`artifacts/UNMERGED-BRANCHES.md`). `worktree-wf_c1156061-e3f-3` (highest priority — closes a `gen_book_cache` self-erasure, same defect class as Epic 5's sweep), `worktree-wf_c1156061-e3f-5`, `review-merge-test`. Also confirm `worktree-wf_cb84ba1e-439-2`'s closed sweep landed in `todo/sweeps.md`, and merge/discard `worktree-wf_be4660f2-72a-3`. Disposition `site-deploy` / `fix/site-deploy-page-workflow` (merged via site PRs #366-#373? delete; else file a reason) and the nine origin-side branches UNMERGED-BRANCHES lists as unlisted-at-capture. Leave the two GAMED branches and the rescue branch (`sd31/racetrait4-SD31-E6-F4-005`) untouched. |
| 3 | `gate-0-census-closure` | Gate 0 | Build `scripts/census_independent.py`; diff against inventory; per-kind object-definition rules | pending | — | New walker, mirror of LST reader; AT-32-G0-001/002/003. |
| 4 | `gate-0-book-onboarding-precondition` | Gate 0 / Epic 4 | Onboard the 4 unbuilt books | pending | — | Sequenced behind Gate 0 census walk; AT-32-G0-003 binding. |
| 5 | `gate-1-shape-closure` | Gate 1 | Build `scripts/shape_ledger.py`; close every unit into one of F1..F10 | pending | — | AT-32-G1-001/002/003. Vocabulary extension allowed with measured units. |
| 6 | `gate-2-engines-f1-f9` | Gate 2 | Confirm `formula_interpreter.rs` reaches all 9 in-scope families with fixtures | pending | — | AT-32-G2-001/002/003/004. |
| 7 | `gate-2-engines-f10-binding` | Gate 2 | Generalise `bonus_stack_reader.rs` for F10 binding-layer family | pending | — | Reaches 77.2% of custom identifiers per SD-31 wave 31 measurement. |
| 8 | `gate-2-corpus-wide-runs` | Gate 2 | Per-engine corpus-wide run with fixture check | pending | — | AT-32-G2-004; one cycle per engine. |
| 9 | `gate-3-closure-invariant` | Gate 3 | Build standing gate; wire into `scripts/verify.sh` | pending | — | AT-32-G3-001/002/003. |
| 10 | `epic-1-compute-library` | Epic 1 | Build the library from proven code (F1/F2/F3 of `epic-breakdown.md`) | pending | — | Behind G1+G2 by construction. |
| 11 | `epic-2-cause-closure` | Epic 2 | Close the eight measured blocker shapes T2a/T2b/T9/T4/T12/T5/T1/T3 by class | pending | — | Behind G1+G2 by construction. T5 credited via card 4, T3 via card 1 (cite, don't re-close); T8/T7 opportunistic; T10 is census-process (`epic-breakdown.md` Epic 2, AT-32-E2-001). |
| 12 | `epic-3-class-reachability` | Epic 3 | 77 prestige-class gating; 18 untabled base classes | pending | — | Behind Gate 0 by construction (chassis-blocked). |
| 13 | `closure-epilogue` | Closure | Write + cite `docs/retro/sd32-...-retrospective.md`; full worktree/branch sweep; architecture-docs refresh; `tranche/12 → develop` PR; release-notes population | pending | — | Fires LAST. Full sequence: `workflow-instruction.md §13`. |

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
the dependency on SD-31 is its content being merged to develop — PR #374, 2026-08-22, verified by
content in `workflow-instruction.md §1` item 3 rather than by a per-cycle PI-screen citation). The PI-gate and
declared-PI reader are not consumed here because SD-32 does not produce records that ship into
the player-facing app in its own right — the engines it builds are surfaced via
`reach_gate.rs` from the existing ingest path that already carries the PI-gate.

## Why no Hermes kanban board

The Hermes kanban board was retired 2026-08-01 (`SD-30-class-feature-archetype-bundle/decisions.md`
Decision 14a); SD-30 and its successors use the local-file pattern. This is documented at
`workflow-instruction.md §1` item 1, `workflow-instruction.md §2`, and `workflow-instruction.md §6` step 8.
