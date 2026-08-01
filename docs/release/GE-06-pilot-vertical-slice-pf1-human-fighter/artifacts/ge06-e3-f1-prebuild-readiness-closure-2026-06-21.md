---
title: GE06-E3-F1 Prebuild Readiness Closure
artifact_type: execution-readiness-prebuild
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE06-E3-F1 — Selected parity-dimension adapter
workflow_route: readiness-closure
readiness: blocked
status: prebuilt-draft
created_handoff:
  - ./ge06-e3-f1-prebuild-handoff-2026-06-21.md
review_date: 2026-06-21
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE06-E3-F1 Prebuild Readiness Closure

## Verdict
A bounded E3-F1 packet can be prebuilt now, but it must remain non-authorizing until E2-F3 lands on `origin/develop` and the merged receipt shape is re-read from the live repo.

This artifact exists to make the launch gate explicit, not to pretend the gate is already open.

## Core problem
TR-06-010 requires GE-06 to define which selected outputs are mandatory oracle-comparison targets for pilot viability, which may remain known gaps, and what evidence is required before any dimension can be called `Oracle-checked`.

After E2-F3, Codex should finally have one integrated headless receipt path over the accepted deterministic pilot. E3-F1 is the smallest honest next slice: adapt that merged new-system receipt into a selected parity-dimension surface that GE-05 comparison work can consume without inventing PCGen evidence, normalization passes, or broad parity claims.

## Selected bounded slice

```text
GE06-E3-F1 — Selected parity-dimension adapter
```

This slice should do only three things once its gate opens:

1. consume the merged E2-F3 headless receipt surface
2. project a narrow, machine-checkable selected-dimension surface for the GE-06 pilot
3. preserve claim-tier honesty by keeping every dimension below `Oracle-checked` until GE-05 comparison evidence exists

It should not run PCGen, compare old vs new, normalize values, or write a broad parity report.

## Draft selected-dimension contract
The prebuilt packet should carry this candidate dimension boundary forward for post-E2-F3 revalidation.

### Mandatory comparison targets for pilot viability
- `character.identity`
- `combat.baseline_melee_attack_bonus`
- `defense.baseline_armor_class`
- `defense.total_save.fortitude`
- `defense.total_save.reflex`
- `defense.total_save.will`
- `skill.selected_modifier.climb`
- `skill.selected_modifier.intimidate`
- `skill.selected_modifier.swim`

### Known-gap-permitted candidates
These may remain explicit known gaps without invalidating the whole pilot, provided they are reported honestly:
- loaded content summaries/counts when old/new routes are not yet comparable
- feat/prerequisite availability dimensions beyond the already-grounded deterministic receipt path
- one exportable summary/stat-block dimension if the old-system route remains unresolved

### Out of current scope
- broad Pathfinder comparison coverage beyond the PF1 Core Rulebook Human Fighter level 1 pilot
- generic parity framework design
- UI-visible parity claims
- claim promotion above `Computed`

## Grounded source evidence available now
| Gate | Grounded source |
|---|---|
| GE-06 parity requirement | `technical-requirements.md` TR-06-010 requires selected parity dimensions, known-gap posture, and evidence-gated `Oracle-checked` claims. |
| GE-06 failure/claim posture | `technical-design.md` defines a parity payload with selected comparison dimensions, old/new source refs, normalization or known-gap status, comparison result, and claim tier. |
| GE-05 fixture carrier | `/home/ubuntu/workspace/repos/codex/src/oracle_validation/golden_fixture.rs` already represents comparison dimensions, Codex output state, known-gap refs, and claim tiers without claiming parity passed. |
| GE-05 evidence doctrine | `../GE-05-oracle-validation-and-parity-harness/artifacts/parity-report-format.md` and `artifacts/known-gap-policy.md` define status vocabulary and known-gap handling. |
| E2-F3 target evidence shape | `artifacts/ge06-e2-f3-execution-readiness-closure-2026-06-21.md` and `artifacts/ge06-e2-f3-execution-handoff-2026-06-21.md` define the merged receipt shape this slice expects to consume. |
| Live repo boundary today | `src/oracle_validation/` exists with `golden_fixture.rs` and `mod.rs`; no comparator/report-writer module exists yet. |

## Launch gates that remain closed
This packet must not be promoted into a code-authorizing handoff until all of the following are true:

1. `artifacts/ge06-e2-f3-merge-receipt-YYYY-MM-DD.md` exists for the real merged E2-F3 slice.
2. The live repo at merged `origin/develop` still exposes a stable headless receipt surface materially compatible with the E2-F3 handoff contract.
3. The draft write scope below still remains disjoint from E3-F2 so the pair can launch in parallel without collision.
4. A post-merge documentary pass confirms that the mandatory dimension list above is still the narrowest truthful viability set.

If any gate fails, re-derive the packet instead of widening silently.

## Candidate implementation posture after gate clear
The smallest likely implementation surface is:

```text
src/oracle_validation/mod.rs
src/oracle_validation/selected_parity_dimensions.rs
tests/ge06_selected_parity_dimensions.rs
```

Read-only dependencies for that later run should include:

```text
src/rules_core/pilot_compute.rs
tests/ge06_pilot_headless_receipt.rs
src/oracle_validation/golden_fixture.rs
tests/golden_case_fixture_schema.rs
programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/parity-report-format.md
programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/known-gap-policy.md
```

This candidate scope is intentionally separate from the likely E3-F2 classifier surface so the pair can become the first honest parallel launch after E2-F3.

## Explicit non-goals
Do not let a future E3-F1 handoff authorize:
- PCGen command execution or raw oracle capture
- normalization engine behavior
- pass/fail comparator logic
- parity report writer implementation
- `Oracle-checked` promotion
- UI work or GE-07 scope
- edits to `src/rules_core/pilot_compute.rs` unless the post-merge audit proves E2-F3 landed differently than specified

## Completion rule
This prebuild readiness closure is complete when it leaves no ambiguity about three facts:
- what E3-F1 is supposed to adapt
- which selected dimensions it is allowed to carry forward
- exactly why the packet still cannot be launched before E2-F3 merge evidence exists
