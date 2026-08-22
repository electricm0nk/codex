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
| 3 | `gate-0-census-closure` | Gate 0 | Build `scripts/census_independent.py`; diff against inventory; per-kind object-definition rules | complete | 2 | New walker, mirror of LST reader; AT-32-G0-001/002 met (unexplained=0; 186 book dirs; ten-kind + kind-unenumerable counts). AT-32-G0-003 is card 4's own criterion, not this card's — Gate 0 overall stays open until card 4 lands. Receipt: `artifacts/gate-0-census-closure/001_cycle_receipt.md`. |
| 4 | `gate-0-book-onboarding-precondition` | Gate 0 / Epic 4 | Onboard the 4 unbuilt books | complete | 3 | Sequenced behind Gate 0 census walk; AT-32-G0-003 binding. All four books (inner_sea_faiths/magic/taverns/temples) land their first compiled `RuleSetId` — three via a new spell family, taverns via the feat gap-row lane. Gate 0 (AT-32-G0-001/002/003) now closed. Receipt: `artifacts/gate-0-census-closure/002_cycle_receipt.md`. |
| 5 | `gate-1-shape-closure` | Gate 1 | Build `scripts/shape_ledger.py`; close every unit into one of F1..F10 | complete | 1 | AT-32-G1-001/002/003 met (`unclassified_count`=0 over 24,914 not-done units; fails closed on `/dev/null`; every family states a proof width). Honest extension: F0 no-formula-content (20,113), F8 residual (41). Found + logged (not fixed, out of card scope) a doc mismatch: `epic-breakdown.md` Epic 1 has no F1..F10 count table — its F1/F2/F3 rows are work items, real counts live only in SD-31's `MEASURE-TWICE.md` §3. Gate 1 closed; Gate 2 (cards 6-8) unblocked. Receipt: `artifacts/gate-1-shape-closure/001_cycle_receipt.md`. |
| 6 | `gate-2-engines-f1-f9` | Gate 2 | Confirm `formula_interpreter.rs` reaches all 9 in-scope families with fixtures | complete | 1 | AT-32-G2-001/002/003 met for F1..F9 (F1,F2,F3,F4,F5,F6,F7,F8,F9 — the shape_ledger.py in-scope set; F10 is card 7's own scope). One real corpus-derived, oracle-provenance-verified fixture per family (`tests/fixtures/rules_core/formula-interpreter-family-fixtures.json`) run through the production `PcgenFormulaEvaluator` via a new test file (`tests/formula_interpreter_family_fixture_check.rs`, 5 tests: evaluator-vs-fixture, mutation proof, extraction-consistency, corpus-provenance, shape_ledger-classifier-agreement). No engine source change needed — the interpreter already reached all nine shapes; this cycle proved it with committed fixtures. AT-32-G2-003 entry appended to `acceptance-and-verification.md` (population 4,798 units across F1..F9, proof width, re-derive command). AT-32-G2-004 (corpus-wide run) explicitly NOT claimed — card 8's own criterion. Receipt: `artifacts/gate-2-engines/001_cycle_receipt.md`. |
| 7 | `gate-2-engines-f10-binding` | Gate 2 | Generalise `bonus_stack_reader.rs` for F10 binding-layer family | complete | 1 | AT-32-G2-001 met (engine named: generalised `bonus_stack_reader.rs`). Added `extract_define_base`/`ProducerChain`/`resolve_producer_chain_corpus_wide`/`evaluate_producer_chain` — data-driven, multi-record producer-chain resolution (DEFINE base + BONUS:VAR addends found across ANY records, not one caller-preselected record), reaching the 77.2%/893-of-1,156 F10 figure. Proven against real corpus bytes (`AlchemistBombLVL` spans two records); mutation-proven the multi-record scan is load-bearing (6 vs 8); no regression to the wave-26 API (18/18 module, 830/830 `pilot_compute` suite). AT-32-G2-002 (fixture-check CLI) and AT-32-G2-004 (corpus-wide run) remain open, explicitly not claimed — card 8's job. Receipt: `artifacts/gate-2-engines/007_cycle_receipt.md`. |
| 8 | `gate-2-corpus-wide-runs` | Gate 2 | Per-engine corpus-wide run with fixture check | in-progress | 1 (F10) | **F10 leg complete, F1-F9 leg still pending (behind card 6) — this row stays `in-progress` until both engines' corpus-wide cycles land**, per this card's own "one cycle per engine" scope. AT-32-G2-004 met for the F10 (`bonus_stack_reader`) engine: new `--bin bonus_stack_reader` CLI (`--corpus-wide`, `--fixture-check`); real run over 26,932 corpus records found 4,736 distinct target variables (3,519 resolved, 1,217 refused); fixture-checked against a hand-transcribed `expected.json` (3 real variables, all matched; mutation-proved the check itself fails correctly). Discovered and logged (not a new card) a real third `AlchemistBombLVL` producer beyond card 7's own two. The F1-F9 (`formula_interpreter`) engine's own corpus-wide cycle is separate and still pending — this row's `complete` covers the F10 chain only; Gate 2 overall stays open until both land. Receipt: `artifacts/gate-2-engines/008_cycle_receipt.md`. |
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
