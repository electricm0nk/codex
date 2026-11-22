---
title: GE-05 Epic Breakdown
stc_id: STC-CODEX-GE-05
artifact_type: epic-breakdown
status: draft
scope: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness
source_stc: ./README.md
source_artifacts:
  - ./README.md
  - ./technical-requirements.md
  - ./technical-design.md
---

# GE-05 Epic Breakdown

## Purpose
Decompose GE-05 into implementation-facing epics and feature seeds that can later become bounded execution-readiness closures or handoffs.

This file is not a code-authorizing brief. It names the terrain so later implementation can be narrow.

## Epic GE05-E1 — PCGen command and oracle discovery
**Objective:** Discover and document the lowest-friction PCGen route that can produce usable pilot behavior evidence for old-vs-new comparison.

**Derived from:** TR-05-003, TR-05-004, TR-05-016.

### Feature seeds
#### GE05-E1-F1 — Candidate oracle route inventory
Acceptance:
- candidate PCGen CLI, validation, export, scripting, test, and GUI routes are identified or explicitly ruled out for the pilot
- each candidate records command/path evidence, trust tier, repeatability, and limitations
- static source surfaces remain classified as source truth, not runtime parity evidence

#### GE05-E1-F2 — First reproducible old-system output route
Acceptance:
- one selected route can produce or attempt to produce old-system output for the Human Fighter case
- failure output is captured if the route is blocked
- route evidence is sufficient to derive the next fixture or blocker

#### GE05-E1-F3 — Oracle-route decision record trigger
Acceptance:
- if GUI driving or another brittle route is required, a decision record is required before treating it as accepted harness design

## Epic GE05-E2 — Golden fixture schema and initial expected-output contract
**Objective:** Define and implement the fixture contract that binds PCGen output, Codex output, compared dimensions, normalization, and known gaps for the first pilot case.

**Derived from:** TR-05-005, TR-05-014, `artifacts/golden-case-fixture-format.md`, `artifacts/initial-human-fighter-l1-expected-output-source-requirements.md`.

### Feature seeds
#### GE05-E2-F1 — Golden-case fixture schema
Acceptance:
- fixture schema includes case ID, source package, character inputs, old/new output references, compared dimensions, normalization declarations, diagnostics, known-gap links, and claim-tier target
- fixture schema can represent blocked or not-yet-grounded outputs without pretending they passed

#### GE05-E2-F2 — PF1 Human Fighter level 1 fixture instance
Acceptance:
- first fixture is scoped to `pf1-crb-human-fighter-level1`
- inherited character inputs are grounded from the pilot charter
- final expected values are either linked to actual captured output or explicitly marked unresolved

#### GE05-E2-F3 — Fixture retention and licensing rule
Acceptance:
- fixture output retention is classified as direct, reduced, hashed/reference-only, or generated on demand
- legal/licensing uncertainty is recorded before substantial PCGen-derived output is committed

## Epic GE05-E3 — Output capture and normalization
**Objective:** Define and implement old/new output capture and normalization rules that make comparable fields explicit without hiding disagreement.

**Derived from:** TR-05-006, TR-05-007, TR-05-008, TR-05-009.

### Feature seeds
#### GE05-E3-F1 — New-system output contract adapter
Acceptance:
- Codex output exposes selected derived values, choices/prerequisites, diagnostics, provenance/source-map references, and explanation references needed by GE-05
- missing GE-03/GE-04 fields become diagnostics or known gaps

#### GE05-E3-F2 — PCGen output capture adapter
Acceptance:
- old-system output is captured with route metadata, raw artifact reference, warnings/errors, and comparable field extraction
- captured output distinguishes runtime behavior evidence from static source evidence

#### GE05-E3-F3 — Normalization rule set
Acceptance:
- every normalization rule names raw field, normalized field, transformation, reason, lossiness, and fields that must not be normalized away
- ambiguous normalization blocks the affected dimension or creates a known gap

## Epic GE05-E4 — Comparator, diff reporter, and parity report writer
**Objective:** Compare normalized old/new output, produce actionable diffs, and write evidence reports suitable for compatibility claim review.

**Derived from:** TR-05-010, TR-05-011, TR-05-013, `artifacts/parity-report-format.md`.

### Feature seeds
#### GE05-E4-F1 — Dimension comparator
Acceptance:
- comparator supports pass/fail/blocked/known-gap/intentionally-divergent status per dimension
- raw and normalized values or references remain traceable

#### GE05-E4-F2 — Actionable diff reporter
Acceptance:
- failure records include compared dimension, old reference, new reference, normalized values, delta classification, likely owner when known, diagnostic/known-gap links, and next investigation target

#### GE05-E4-F3 — Parity report writer
Acceptance:
- report includes case metadata, run/source references, dimensions, results, diffs, diagnostics, known gaps, and claim tier
- report is machine-checkable enough for tests and human-readable enough for review

## Epic GE05-E5 — Known-gap ledger and intentional-divergence routing
**Objective:** Ensure non-comparable, unsupported, undesirable, legally constrained, or out-of-scope behavior is recorded instead of silently disappearing.

**Derived from:** TR-05-012, TR-05-015, `artifacts/known-gap-policy.md`.

### Feature seeds
#### GE05-E5-F1 — Known-gap ledger schema
Acceptance:
- ledger records case ID, dimension, gap type, old/new evidence state, reason, owner, blocking status, review trigger, and linked decision record when applicable

#### GE05-E5-F2 — Intentional-divergence decision workflow
Acceptance:
- behavior that PCGen exhibits but Codex should not preserve requires a decision record under `programs/codex/doctrine/decisions/`
- parity report can link intentional divergence without counting it as accidental pass or silent failure

#### GE05-E5-F3 — Known-gap report integration
Acceptance:
- comparator/report writer includes known-gap entries in summary and dimension-level output
- unknown gaps block claim-tier promotion until resolved or accepted by decision

## Epic GE05-E6 — Headless verification and CI/test integration
**Objective:** Make parity comparison repeatable through bounded commands or tests once the implementation exists.

**Derived from:** TR-05-016.

### Feature seeds
#### GE05-E6-F1 — Old/new comparison command
Acceptance:
- one command or test can produce or validate old output, new output, normalization, comparison, and report generation for the pilot fixture
- blocked old-system route produces a clear failure receipt rather than silent skip

#### GE05-E6-F2 — Regression guard for report completeness
Acceptance:
- tests fail when a comparison report lacks evidence references, diffs for failures, or known-gap records for non-comparable outputs

#### GE05-E6-F3 — GE-06 integration evidence handoff
Acceptance:
- GE-06 can consume GE-05 report artifacts without redefining the oracle-parity standard
- integrated pilot acceptance can distinguish computed, oracle-checked, and product-visible claims

## Recommended sequencing
1. GE05-E1 — discover the PCGen oracle route first; without it, parity evidence cannot exist.
2. GE05-E2 — define fixture schema and first Human Fighter fixture instance using whatever oracle evidence or blockers GE05-E1 discovers.
3. GE05-E3 — build capture/normalization for old and new outputs.
4. GE05-E4 — implement comparator, diff reporter, and parity report writer.
5. GE05-E5 — integrate known-gap and intentional-divergence governance.
6. GE05-E6 — harden as repeatable headless verification and GE-06 consumable evidence.

## First handoff-readiness result
`artifacts/ge05-e1-f1-handoff-readiness-closure-2026-06-20.md` established the first route-correct handoff posture, and the later coding follow-through has now completed its first merged slice:

- `GE05-E1-F1 — Candidate oracle route inventory` was correctly routed as a non-code discovery/research handoff.
- `GE05-E1-F2 — First reproducible old-system output route` then grounded real runtime evidence for the provisional Human Fighter input.
- `GE05-E2-F1 — Golden-case fixture schema` was subsequently grounded, handed off to the coding harness, and is now preserved as historical authority at `artifacts/ge05-e2-f1-execution-handoff-2026-06-20.md`.
- `artifacts/ge05-e2-f1-merge-receipt-2026-06-21.md` verifies that the GE05-E2-F1 slice merged on `develop`, passed detached cargo verification, and no longer holds active code authority.
- `execution-handoff.md` is now the root GE-05 coding route surface with `status: running-under-card-triggered-harness` via Kanban card `t_0cdc64d0`.
- `artifacts/ge05-e2-f2-execution-readiness-closure-2026-06-24.md` and `artifacts/ge05-e2-f2-execution-handoff-2026-06-24.md` now exist as the active GE05-E2-F2 pair.
- The current truthful candidate has become the current truthful launch-ready packet: `GE05-E2-F2 — PF1 Human Fighter level 1 governed fixture instance`.

## Completion gate
GE-05 is implementation-ready only when a bounded slice has:

- exact objective
- exact repo/workdir
- branch/worktree policy
- allowed write scope
- required legacy and Codex reads
- selected implementation slice
- old/new output evidence target or discovery blocker
- verification commands or discovery receipt requirements
- non-goals preventing broad regression-suite drift

`GE05-E2-F1` satisfied this gate for the fixture-schema slice and has now merged. The epic breakdown remains planning authority; live code authority now sits only in `artifacts/ge05-e2-f2-execution-handoff-2026-06-24.md`, while the root route surface remains non-authorizing and `running-under-card-triggered-harness`.
