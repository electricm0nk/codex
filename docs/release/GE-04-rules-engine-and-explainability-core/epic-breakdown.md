---
title: GE-04 Epic Breakdown
stc_id: STC-CODEX-GE-04
artifact_type: epic-breakdown
status: draft
scope: programs/codex/requirements/GE-04-rules-engine-and-explainability-core
source_stc: ./README.md
---

# GE-04 Epic Breakdown

This file decomposes the Codex GE-04 source STC into bounded downstream epics and feature seeds. These are implementation-facing planning units, not execution prompts.

## Epic GE04-E1 — Character Input and Fixture Boundary
**Objective:** Define and later implement the minimum character input shape required to compute the PF1 Human Fighter level 1 pilot.

**Derived from:** TR-04-003, TR-04-014, and `artifacts/pilot-golden-computation-fixture-requirements.md`.

### Feature seeds

#### GE04-E1-F1 — Character input record shape
**Outcome:** A bounded record shape for package identity, race/class/level, ability scores, feats, skills, equipment, and selected choices.

**Acceptance signals:** chosen state is separated from derived state; fixture inputs can be loaded headlessly; invalid character input produces diagnostics.

**Execution-readiness status:** validated as `codex-ready`. `artifacts/ge04-e1-f1-execution-readiness-closure-2026-06-20.md` records the grounded gate evidence, and `execution-handoff.md` is the derived code-authorizing brief. The coding run must branch from a clean, current `develop` into `ge04-e1-f1-character-input-record-shape`.

#### GE04-E1-F2 — Pilot fixture normalization
**Outcome:** Deterministic normalization of the pilot fixture into stable IDs and structured selections.

**Acceptance signals:** canonical references are resolved or diagnosed; fixture state is reproducible; explanation graph can point to input selections.

## Epic GE04-E2 — Effect Evaluation Core
**Objective:** Define and later implement the bounded effect collection and activation pipeline for pilot canonical content.

**Derived from:** TR-04-004, TR-04-005, TR-04-006, and `artifacts/evaluation-order-definition.md`.

### Feature seeds

#### GE04-E2-F1 — Candidate effect collection
**Outcome:** Candidate effects can be gathered from race, class, feats, skills, equipment, choices, and package defaults.

**Acceptance signals:** effect sources are visible; unsupported effect semantics emit diagnostics; collected effects do not yet imply activation.

#### GE04-E2-F2 — Effect activation and contribution records
**Outcome:** Active effects contribute to derived state with recorded contribution evidence.

**Acceptance signals:** each contribution has a source object/effect identity; inactive or blocked effects explain why; circular or unstable chains are diagnosed.

## Epic GE04-E3 — Formula and Prerequisite Evaluator
**Objective:** Define and later implement deterministic, sandboxed evaluation for pilot formulas and prerequisites.

**Derived from:** TR-04-007, TR-04-008, and `artifacts/expression-language-runtime-requirements.md`.

### Feature seeds

#### GE04-E3-F1 — Formula evaluator skeleton
**Outcome:** Pilot formulas can be represented and evaluated under deterministic, dependency-capturing constraints.

**Acceptance signals:** expressions cannot perform host side effects; dependencies are recorded; invalid or unsupported formulas produce diagnostics.

#### GE04-E3-F2 — Prerequisite expected-versus-actual output
**Outcome:** Prerequisites return pass/fail/unknown results with expected and actual state details.

**Acceptance signals:** unavailable feats/choices explain the blocking condition; unsupported prerequisites do not become silent false/true outcomes; tests can assert prerequisite explanations.

## Epic GE04-E4 — Choice Availability Engine
**Objective:** Define and later implement headless calculation of available and unavailable choices for pilot choice sets.

**Derived from:** TR-04-009 and `artifacts/explanation-graph-schema.md`.

### Feature seeds

#### GE04-E4-F1 — Candidate option enumeration
**Outcome:** Choice sets can enumerate candidate options with stable identities.

**Acceptance signals:** candidate options are separated from selected options; unresolved selectors produce diagnostics; source/provenance can be linked.

#### GE04-E4-F2 — Unavailable-choice explanations
**Outcome:** Unavailable choices produce explanations linked to prerequisites, selectors, diagnostics, or repeatability rules.

**Acceptance signals:** unavailable result has expected-versus-actual detail where applicable; UI consumers can later display the reason without recomputing it; tests can assert blocked-choice evidence.

## Epic GE04-E5 — Derived Stats and Diagnostic Emitter
**Objective:** Define and later implement pilot derived-value calculation and rules diagnostic output.

**Derived from:** TR-04-010, TR-04-012, and `artifacts/diagnostic-schema.md`.

### Feature seeds

#### GE04-E5-F1 — Pilot derived output categories
**Outcome:** A bounded output surface for pilot ability modifiers, attack/combat values, saves, skills, and selected prerequisite/choice outcomes.

**Acceptance signals:** output categories are explicit; expected values are fixture-backed, not invented; each tested output has contribution records.

#### GE04-E5-F2 — Rules diagnostic record shape
**Outcome:** Rules execution emits machine-readable/human-readable diagnostics with source and claim-blocking posture.

**Acceptance signals:** invalid content, invalid character choices, unsupported semantics, expression errors, cycles, provenance gaps, and engine defects are distinguishable; diagnostics can attach to explanation graph nodes.

## Epic GE04-E6 — Explanation Graph Builder
**Objective:** Define and later implement the graph structure that explains derived values, prerequisites, choices, diagnostics, and provenance.

**Derived from:** TR-04-011 and `artifacts/explanation-graph-schema.md`.

### Feature seeds

#### GE04-E6-F1 — Explanation node and edge vocabulary
**Outcome:** A minimal graph vocabulary for inputs, objects, effects, formulas, prerequisites, choices, values, diagnostics, and provenance.

**Acceptance signals:** graph represents contribution and dependency edges; graph can explain at least one derived value and one blocked choice/prerequisite.

#### GE04-E6-F2 — Explanation trace emission
**Outcome:** Rules execution can emit explanation traces suitable for tests and later UI consumption.

**Acceptance signals:** trace includes source contribution path; trace includes diagnostics where behavior is unsupported or invalid; trace is inspectable without UI.

## Epic GE04-E7 — Rules-Core CLI/Test Entry Point
**Objective:** Define and later implement the first headless execution path for deterministic GE-04 fixtures.

**Derived from:** TR-04-013, TR-04-014, and `artifacts/pilot-golden-computation-fixture-requirements.md`.

### Feature seeds

#### GE04-E7-F1 — Deterministic fixture runner
**Outcome:** A test or CLI route loads a fixture, computes outputs, emits explanations/diagnostics, and fails on unmet expectations.

**Acceptance signals:** no desktop UI required; missing expected values fail the run; missing explanation edges fail the run.

#### GE04-E7-F2 — GE-05 comparison-ready output envelope
**Outcome:** Engine output is structured enough for future GE-05 oracle comparison without claiming parity.

**Acceptance signals:** output dimensions are named; diagnostics and known gaps travel with outputs; parity claim remains blocked until GE-05 evidence exists.

## Recommended sequencing (dependency order, not exclusive scope)
1. GE04-E1 — Character Input and Fixture Boundary
2. GE04-E2 — Effect Evaluation Core
3. GE04-E3 — Formula and Prerequisite Evaluator
4. GE04-E4 — Choice Availability Engine
5. GE04-E5 — Derived Stats and Diagnostic Emitter
6. GE04-E6 — Explanation Graph Builder
7. GE04-E7 — Rules-Core CLI/Test Entry Point

GE-04 is fulfilled by eventually executing all of these downstream epics. This ordering is dependency guidance, not permission to stop after the first slice.

## Handoff boundary
No coding harness should receive this file as an execution prompt.

GE04-E1-F1 has an execution-readiness closure at `artifacts/ge04-e1-f1-execution-readiness-closure-2026-06-20.md` and a derived code-authorizing brief at `execution-handoff.md`. Do not code from `main`, from the GE-03 feature branch, or from this epic-breakdown file itself; code only from the bounded handoff after establishing a clean current `develop`.

Before a derived code-authorizing GE-04 implementation handoff is allowed, the following must be true:
- the specific downstream epic or feature seed covered by that handoff is chosen
- target repo/workdir/branch/write-scope facts are grounded for that exact slice
- required reads are exact and bounded
- verification commands are runnable in the target runtime or the handoff explicitly names a prepared environment
- failing-first test posture is named
- remaining GE-02/GE-03/GE-05/GE-06 dependencies remain referenced instead of invented
- the handoff remains narrower than the spec domain

## Completion gate
- [ ] every GE-04 requirement is routed to at least one downstream epic
- [ ] every epic has a bounded objective
- [ ] generated documentary artifact dependencies remain visible
- [ ] unresolved questions remain in `risks-and-open-questions.md`
- [ ] decomposition remains upstream of execution handoff
