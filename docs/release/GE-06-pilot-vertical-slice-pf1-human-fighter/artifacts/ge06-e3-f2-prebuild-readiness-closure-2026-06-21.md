---
title: GE06-E3-F2 Prebuild Readiness Closure
artifact_type: execution-readiness-prebuild
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE06-E3-F2 — Failure classifier and owner mapping
workflow_route: readiness-closure
readiness: blocked
status: prebuilt-draft
created_handoff:
  - ./ge06-e3-f2-prebuild-handoff-2026-06-21.md
review_date: 2026-06-21
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE06-E3-F2 Prebuild Readiness Closure

## Verdict
A bounded E3-F2 packet can be prebuilt now, but it must remain non-authorizing until E2-F3 merge evidence proves the live diagnostic surface that the classifier will consume.

## Core problem
TR-06-012 requires GE-06 to classify integrated failures into one primary owner: model flaw, importer flaw, engine flaw, oracle gap, or UI gap. The technical design is explicit that Codex must classify by the first broken contract, not the last visible symptom.

After E2-F3, Codex should have one merged integrated headless receipt path with computed-or-blocked status plus claim-blocking diagnostics. E3-F2 is the smallest honest next slice: introduce a narrow classifier surface that turns merged integrated receipt facts into a primary-owner failure classification without pretending parity or UI proof already exists.

## Selected bounded slice

```text
GE06-E3-F2 — Failure classifier and owner mapping
```

This slice should do only three things once promoted:

1. consume the merged E2-F3 headless receipt and its structured diagnostics
2. classify the current failure or blocked posture into one primary GE-06 owner category
3. preserve contributing-owner context when useful without replacing the required primary owner

It should not become a generic program-wide incident framework or a parity report writer.

## Draft primary-owner contract
The prebuilt packet should carry this narrow classification vocabulary forward unchanged unless a post-merge audit proves the repo cannot support it truthfully:
- `ModelFlaw`
- `ImporterFlaw`
- `EngineFlaw`
- `OracleGap`
- `UiGap`

### First-broken-contract guidance
Use the GE-06 technical-design examples as the routing rule:
- canonical representation breaks first -> `ModelFlaw`
- load/provenance/import breaks first -> `ImporterFlaw`
- derived values, explanations, or rules diagnostics break first -> `EngineFlaw`
- computed outputs exist but comparison evidence is absent/non-comparable -> `OracleGap`
- headless outputs exist but product-visible surface hides truth -> `UiGap`

### Explicit anti-pattern
`IntegrationIssue` must not exist as a terminal bucket.

## Grounded source evidence available now
| Gate | Grounded source |
|---|---|
| GE-06 failure-taxonomy requirement | `technical-requirements.md` TR-06-012 names the five required primary-owner classes and forbids “integration issue” as terminal diagnosis. |
| GE-06 design posture | `technical-design.md` defines the first-broken-contract rule and provides concrete examples for all five owner categories. |
| E2-F3 blocked/computed status contract | `artifacts/ge06-e2-f3-execution-readiness-closure-2026-06-21.md` and `artifacts/ge06-e2-f3-execution-handoff-2026-06-21.md` define a merged receipt shape with computed/blocked status and claim-blocking diagnostics. |
| Live rules-core lane today | `/home/ubuntu/workspace/repos/codex/src/rules_core/mod.rs` currently exposes `character_input` and `pilot_compute`; there is no dedicated GE-06 failure-classifier surface yet. |
| Existing blocked-case proof target | `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_headless_receipt.rs` already defines the kind of blocked receipt posture this future slice expects to consume once E2-F3 is merged. |

## Launch gates that remain closed
This packet must not be promoted into a code-authorizing handoff until all of the following are true:

1. `artifacts/ge06-e2-f3-merge-receipt-YYYY-MM-DD.md` exists for the real merged E2-F3 slice.
2. The merged repo still exposes structured blocked/computed receipt facts and claim-blocking diagnostics compatible with this draft classifier boundary.
3. The candidate write scope below remains disjoint from E3-F1 so the pair can launch in parallel without collision.
4. The post-merge documentary pass confirms that no broader cross-epic incident framework is needed for the first narrow GE-06 classifier.

If any gate fails, stop and re-derive instead of widening into a vague reporting subsystem.

## Candidate implementation posture after gate clear
The smallest likely implementation surface is:

```text
src/rules_core/mod.rs
src/rules_core/pilot_failure.rs
tests/ge06_failure_classifier.rs
```

Read-only dependencies for that later run should include:

```text
src/rules_core/pilot_compute.rs
tests/ge06_pilot_headless_receipt.rs
src/oracle_validation/golden_fixture.rs
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/technical-design.md
```

This candidate scope is intentionally separate from the likely E3-F1 oracle-validation adapter surface so the pair can become the first honest parallel launch after E2-F3.

## Explicit non-goals
Do not let a future E3-F2 handoff authorize:
- generic incident-management infrastructure
- parity comparator or parity report writer behavior
- UI implementation or GE-07 scope
- importer or rules-engine rewrites under the label of “classification”
- edits to `src/oracle_validation/**` unless a post-merge audit proves the classifier cannot remain rules-core-local

## Completion rule
This prebuild readiness closure is complete when it leaves no ambiguity about three facts:
- which owner vocabulary GE-06 requires
- which live receipt/diagnostic facts E3-F2 depends on
- exactly why the packet still cannot launch before E2-F3 merge evidence exists
