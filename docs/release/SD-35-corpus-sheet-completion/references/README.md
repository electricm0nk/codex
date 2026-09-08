---
canonical: true
owner: god-emporer
bundle_id: SD-35
date: 2026-09-07
---

# SD-35 References

## Doctrine of record

| Document | Why it binds SD-35 |
|---|---|
| `../../../governance/blocker-closure-doctrine.md` | A blocker on the DoD is cleared or escalated, never deferred. `## Open blockers` pauses the bundle. Gates `decisions.md §6`, `workflow-instruction.md §8` and `§11` step 1. |
| `../../../governance/deferral-revisit-doctrine.md` | The sibling rule for a *planned capability deferral*. The test — was this scope in the DoD at launch? |
| `../../../governance/no-stub-mvp-doctrine.md` | No stubs, inline mocks, or `"Would ..."` strings in shipping code. Enforced by `workflow-instruction.md §6` step 2's second grep. |
| `../../../doctrine-external/identifier-discipline.md` | No bundle-tag leaks in shipping identifiers. Enforced by the first grep. |
| `../../../governance/workflow-instruction-template.md` | The template `workflow-instruction.md` is authored from. |
| `../../template/template.md` | The chassis template `README.md` is authored from; `§6` owns the closure pipeline's artifact half. |

## The operator's rulings this package rests on

| Ruling | Where recorded |
|---|---|
| **The sheet rule** — DONE is a final number, a dice expression, or the rule's words; do the math; no dice engine; no per-unit proof machinery (2026-09-07) | `../decisions.md §1`, quoted verbatim |
| **The batch floor** — one mechanism corpus-wide, 500 units minimum, as a script (2026-09-07) | `../decisions.md §2` |
| **No PCGen in live code** — converter and test oracle only; the 78 existing live readers come out in SD-35 (2026-09-07) | `../decisions.md §11`, quoted verbatim |
| **Keep the converter and the oracle** — Starfinder starts right after PF1e; the tool side is the asset, not litter (2026-09-07) | `../decisions.md §11`, "what is kept", quoted verbatim |
| **SD-34's unrun closure is folded into SD-35 Epic 1** (2026-09-07, audit) | `../decisions.md §12` |
| **`box_ledger.py` retired as a gate; the atlas is the partition** (2026-09-07, audit) | `../decisions.md §13` |
| **Fable on every lane until dry, then Opus; orchestrator on Opus** (2026-09-07) | `../decisions.md §14` |
| **Schema v2 from the token-mapping synthesis; rulings R1–R3 pending** (2026-09-08) | `../decisions.md §15`, `../artifacts/epic-2-sheet-rule/token-mapping/{SYNTHESIS,blockers}.md` |
| **Cost scales with token vocabulary, not unit count** (2026-08-31) | `../../SD-34-book-completion/fable-review.md §1.b`, register row C1.6 |
| **Text-only features are complete** (SD-31 Decision 7, 2026-08-16) — the precedent the sheet rule generalizes | `../../SD-31-corpus-closure-grind/decisions.md` Decision 7 (line 505) |
| **The interpreter ban is dead** (SD-31 Decision 20, 2026-08-21) — only `derived_evaluator_fixture_check` binds | `../../SD-31-corpus-closure-grind/decisions.md` Decision 20 |
| **Bucket X needs the per-character choice filter** (2026-08-28) | `../../SD-34-book-completion/decisions.md §17` |
| **Bucket U is DONE when nothing a player reads is missing** (2026-08-28) | `../../SD-34-book-completion/decisions.md §17` |

## Predecessor bundle

`../../SD-34-book-completion/` — the direct predecessor. SD-35 inherits and does not rebuild its
Completion Atlas (`scripts/completion_atlas.py`), its eight engine tables, its citation
instruments, the shape engines and oracle harness it inherited from SD-33, and — as method — its
fable review.

Its most load-bearing findings for SD-35:

- **The atlas partitions every unit fail-closed** — `population=49438 buckets=10 unclassified=0`.
- **A shape engine computes a number; it does not complete a record** (`decisions.md §2a`). Under
  the sheet rule, "completing" is rendering the line.
- **The live side reads PCGen today** — 78 files by coarse grep (`../content-unit-inventory.md §6`);
  SD-31 Decision 20 allowed it and nothing ruled it back out until `../decisions.md §11`.
- **The `grounded` bar was simulation-shaped** — wave 51's `racial_sla` module and Monk refusal
  (`artifacts/bucket-d-mining/wave51_core_rulebook_ultimate_campaign_cycle_receipt.md §3, §5`).
- **Batching works** — waves 49–51 closed 129 / 343 / 217 units with one build each after
  waves 43–48 closed 2–45 per build (`progress.md`, the wave receipts).
- **Line-number citation pins drift** — two of three were stale at HEAD before wave 51
  (`wave51 … receipt.md §6`).
- **The token vocabulary is 189 types, 28 covering 90%** of 181,274 instances
  (`artifacts/fable-review/TOKEN-MODEL.md`).
- **Two named engine tables remain** — `power` 421, `companion` 28
  (`artifacts/epic-1-atlas/missing-engine-tables.json`).

Its `forward-scope-register.md` C1.4 and C1.6 are SD-34's own statement of what SD-35 is; its
C2.x/C3.x rows and §E1 ruled-out branches are mirrored in `../forward-scope-register.md`.

## The fable review

`../../SD-34-book-completion/fable-review.md` (2026-08-31 – 2026-09-01) and its evidence in
`../../SD-34-book-completion/artifacts/fable-review/`:

- `B-SYNTH.md` — the backlog verdict and the five-engine proposal SD-35 supersedes with the
  single ingest-time converter (its engines 1, 3, 5 survive as AT-35-E5-001, the `sheet-complete` rung, and
  AT-35-E4-002).
- `TOKEN-MODEL.md` / `.json` — the vocabulary measurement; the method.
- `findings-all.json` — 126 findings, 56 confirmed; R10 is AT-35-E1-003; the eight P1s are
  `../forward-scope-register.md` C2.5.

## Lessons this package encodes

`../decisions.md §9` carries twelve lessons from SD-34's run, each with an enforcing command.
`../workflow-instruction.md §12` adds rows 27–34 to SD-34's 26. They share one root, stated in
`§1`: **the finish line shapes the work.** A simulation-shaped bar built simulation; a
sheet-shaped bar builds sheet lines. And `§11`'s corollary: **a "for now" with no counter is
forever.**

## Retrospectives

| Document | Status |
|---|---|
| `../../../retro/sd35-corpus-sheet-completion-retrospective.md` | **Written at closure by AT-35-E7-002, and cited from THIS file in the same cycle.** Must report units-per-cycle distribution, lines-per-unit distribution, build time before/after, and the `Words` share per kind. |
| `../../../retro/sd34-book-completion-retrospective.md` | SD-34's. **Does not exist at the cut** — SD-34 merged without running its epilogue (`../decisions.md §12`). **Written by AT-35-E1-006 and cited from THIS file in that cycle**, alongside SD-34's own `references/README.md`; its "changes for the next bundle" are folded into `../decisions.md §9` in the same cycle, before Epic 2 dispatches. |
| `../../../retro/sd33-computed-value-verification-retrospective.md` | SD-33's. The measure-before-population-run lesson (N7). |
| `../../../retro/sd32-compute-library-and-cause-closure-retrospective.md` | SD-32's. The denominator-gate and deferral-revisit disciplines; the generic-pass-not-per-object-lanes proof. |
| `../../../retro/sd31-retrospective.md` | The worked example every bundle's closure retrospective follows in shape. |
| `../../../retro/kind-lane-refactor-proposal.md` | Still a proposal. Its finding — book was never a real partition boundary; kind lanes share fewer chokepoint files — is why SD-35's cycles are mechanism-wide, not book-wide. |

## Skills and tooling

| Item | Use |
|---|---|
| `.claude/skills/stc-authoring/SKILL.md` | The skill this package was authored with, and the one an auditing session should load. |
| `scripts/retro.py`, `docs/retro/schema.json` | Retro event emission every cycle; `--verified-by` required on a `correction`. |
| `scripts/verify.sh` | The stage runner. 40 stages at authoring; 46 after Epics 1–2. |
| `scripts/completion_atlas.py` | The partition. `--check` every cycle. |
| `scripts/cycle_scope_gate.py` | **Built by AT-35-E1-001.** The batch floor and the mechanical receipt rows. |
| `scripts/pcgen_residue_gate.py` | **Built by AT-35-E1-005.** The live-side PCGen count; monotonic; zero at closure. |
| `src/bin/sheet_rule_convert.rs` | **Built by AT-35-E2-001.** The only place the sheet rule reads PCGen. |
| `scripts/oracle_harness/` | The test oracle — PCGen's totals against ours. The operator's permitted use of PCGen. |
| `scripts/token_coverage.py` | **Built by AT-35-E2-004.** The per-token remainder ledger. |
| `scripts/pcgen-oracle-pin.env` | The pinned oracle SHA. **`~/workspace/repos/pcgen` is forbidden as an oracle path.** |

## Architecture

`../../../architecture/` — current-state truth, refreshed at closure per
`../../template/template.md §6` step 2. `rules-data-tables.md` lines 447–463 (`wiring_class`)
describe the token closure the converter reads; `rules-engine.md`, `desktop-app.md`,
`corpus-ingest.md`, `status.md`, and `testing.md` are re-verified before the PR opens, and the
converter/live boundary (`../technical-design.md §0`) is written into `overview.md`.
