---
canonical: true
owner: god-emporer
bundle_id: SD-34
date: 2026-08-26
---

# SD-34 References

## Doctrine of record

| Document | Why it binds SD-34 |
|---|---|
| `../../../governance/blocker-closure-doctrine.md` | A blocker on the DoD is cleared or escalated, never deferred. `## Open blockers` pauses the bundle. Gates `decisions.md §6`, `workflow-instruction.md §8` and `§11` step 1. |
| `../../../governance/deferral-revisit-doctrine.md` | The sibling rule for a *planned capability deferral*: condition, checker, accepted cost. The test separating the two — was this scope in the DoD at launch? |
| `../../../governance/no-stub-mvp-doctrine.md` | No stubs, inline mocks, or `"Would ..."` strings in shipping code. Enforced by `workflow-instruction.md §6` step 2's second grep. |
| `../../../doctrine-external/identifier-discipline.md` | No bundle-tag leaks in shipping identifiers. Enforced by the first grep. |
| `../../../governance/workflow-instruction-template.md` | The template `workflow-instruction.md` is authored from. |
| `../../template/template.md` | The chassis template `README.md` is authored from; `§6` owns the closure pipeline's artifact half. |

## Predecessor bundle

`../../SD-33-computed-value-verification/` — the direct predecessor. SD-34 inherits and does
not rebuild its shape engines (`formula_interpreter`), oracle harness (`scripts/oracle_harness/`),
`box_ledger.py`, denominator gate, or corpus literal sweep. Its `forward-scope-register.md` C2.x and C3.x rows
are carried into this bundle's own register, its three open deferrals and D1.x debt rows are
carried under "Carried forward from SD-33", and its **§E1 ruled-out branches are mirrored in
`../forward-scope-register.md §E1` — not to be re-litigated by AT-34-E6-003's sweep.**

Its most load-bearing findings for SD-34:

- **Ingestion is complete.** All 49,438 units were read from a real source line; 51,505 JSON
  files exist under `data/corpus/`. The `not-ingested` status field is a misnomer for "the engine does not hold
  this record" (`decisions.md §2b`).
- **The shape engines work.** `formula_interpreter` covers F1..F9, recognising 10,626 of 11,652
  formulas and refusing 240 rather than guessing.
- **A shape engine computes a number; it does not complete a record.** 26,396 units carry
  magnitude tokens and 13,119 of those are still not held by the engine (`decisions.md §2a`).
- Its oracle harness is what clears SD-34's bucket `V` (8,330 units corpus-wide).

## Lessons this package encodes

`../decisions.md §12` carries eight lessons — L1–L5 from the session that closed SD-33 and
authored this package, L6–L8 from SD-33's own retrospective §6 — **each with an enforcing
command**. `../workflow-instruction.md §12` carries all 26 standing lessons in table form with
their enforcers.

They share one root, stated in `§12`'s preamble: **a derived artifact was trusted instead of
the source it derives from** — a field's name instead of the code that writes it, an author's
own earlier number instead of the data, a workflow's status instead of the repo, a lane's
account of a failure instead of `git`.

**A lesson without a mechanism is a quote.** SD-31's lessons were captured in SD-32's package
and ignored, because they were prose. That is why every row names a command.

## Retrospectives

| Document | Status |
|---|---|
| `../../../retro/sd34-book-completion-retrospective.md` | **Written at closure by AT-34-E6-002, and cited from THIS file in the same cycle.** Not a follow-up. A retrospective that exists but is never linked from the package it is about is the exact gap an SD-32 chassis review had to fix by hand. |
| `../../../retro/sd33-computed-value-verification-retrospective.md` | SD-33's. The source of `decisions.md §4`, `§5`, `§10`, `§12` L6–L8 (its §6 fold lessons) and `workflow-instruction.md §12` rows 3, 11–16, 23–26. Its §5 corrects the inherited-debt figure to 29 of 599 / 46 of 8,034. |
| `../../../retro/sd32-compute-library-and-cause-closure-retrospective.md` | SD-32's. The source of the denominator-gate and deferral-revisit disciplines SD-34 inherits. |
| `../../../retro/sd31-retrospective.md` | The worked example every bundle's closure retrospective follows in shape. |

## Skills and tooling

| Item | Use |
|---|---|
| `.claude/skills/stc-authoring/SKILL.md` | The skill this package was authored with, and the one an auditing session should load. |
| `scripts/retro.py`, `docs/retro/schema.json` | Retro event emission every cycle; `--verified-by` required on a `correction`. `deferrals.open` is trustworthy post-SD-32 fix. |
| `scripts/verify.sh` | The stage runner. `--only denominator-gate` is live and inherited. |
| `scripts/pcgen-oracle-pin.env` | The pinned oracle SHA. **`~/workspace/repos/pcgen` is forbidden as an oracle path** — a `preflight-oracle` PASS against it fails silently. |

## Architecture

`../../../architecture/` — current-state truth, refreshed at closure per
`../../template/template.md §6` step 2. Every topic SD-34 touches is re-verified there before
the PR opens.
