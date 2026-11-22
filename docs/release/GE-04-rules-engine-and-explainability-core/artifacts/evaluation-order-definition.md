---
title: GE-04 Evaluation Order Definition
stc_id: STC-CODEX-GE-04
artifact_type: generated-documentary-output
status: draft
scope: programs/codex/requirements/GE-04-rules-engine-and-explainability-core/artifacts
source_stc: ../README.md
---

# GE-04 Evaluation Order Definition

## Purpose
Define the planning-level evaluation order required for the first Codex rules-engine pilot without pretending that every Pathfinder-wide ordering, stacking, or circular dependency issue is solved.

## Pilot evaluation order

| Phase | Name | Required behavior | Output |
|---:|---|---|---|
| 0 | Package admission | Load named source package(s), dependency graph, validation state, source maps, and diagnostics. | admitted package context or blocking diagnostics |
| 1 | Character input normalization | Normalize race/class/level/ability/feat/skill/equipment/choice inputs into stable references. | normalized character input or invalid-input diagnostics |
| 2 | Baseline state initialization | Create base ability values, level, empty/known defaults, and package-specified baseline rules relevant to pilot. | baseline evaluation context |
| 3 | Candidate object and effect collection | Gather candidate effects, prerequisites, formulas, choice sets, selectors, equipment effects, and diagnostics from relevant canonical objects. | candidate rule/effect set |
| 4 | Static grant activation | Activate always-on grants that have no unresolved prerequisite or conditional gate. | active grants with contribution records |
| 5 | Formula and prerequisite evaluation | Evaluate formulas and prerequisites needed to decide activation, eligibility, and selected derived values. | pass/fail/unknown outcomes, dependencies, diagnostics |
| 6 | Choice availability calculation | Enumerate choices and filter by prerequisites/selectors/repeatability/known state. | available/unavailable options with explanations |
| 7 | Conditional/equipment effect activation | Apply selected equipment or conditional effects required by the pilot fixture. | active conditional contributions or diagnostics |
| 8 | Derived value calculation | Compute pilot derived values from baseline, active effects, formulas, and selected state. | structured derived values with contribution records |
| 9 | Diagnostic consolidation | Normalize content, character, expression, dependency, provenance, and engine diagnostics. | diagnostic set with claim-blocking posture |
| 10 | Explanation graph emission | Emit graph nodes and edges linking inputs, objects, effects, formulas, prerequisites, choices, values, diagnostics, and provenance. | explanation graph output |
| 11 | Headless output envelope | Emit test/CLI-readable result envelope for GE-04 evidence and future GE-05 comparison. | values, explanations, diagnostics, known gaps |

## Dependency handling rule
The engine MUST record dependencies discovered during formula and prerequisite evaluation. If a derived value depends on another value not yet known, the implementation must schedule evaluation deterministically, perform an explicitly bounded fixed-point/dependency resolution strategy, or emit a diagnostic that the dependency is unresolved or circular.

The implementation MUST NOT produce final values by relying on hidden incidental evaluation order.

## Circular dependency posture
Circular or unstable dependencies are not solved by this planning artifact. A later implementation handoff must either exclude such cases from its slice or define explicit cycle-detection diagnostics and tests.

## Stacking and modifier posture
For the pilot, stacking/modifier behavior should be scoped to the minimum interactions required by the selected character fixture. Broad Pathfinder-wide stacking doctrine is deferred unless the pilot cannot compute honestly without it.

## Explanation timing rule
Explanations must be emitted from the same computation evidence used to produce values. A later UI or report must not reconstruct explanations after the fact from unrelated prose.

## Review triggers
Reopen this artifact when GE-06 fixes the exact fixture, a future evaluator proves this order insufficient, a cycle/stacking issue appears, or GE-05 comparison requires additional output ordering or normalization.
