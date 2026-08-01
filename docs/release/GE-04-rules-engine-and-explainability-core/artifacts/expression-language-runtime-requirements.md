---
title: GE-04 Expression-Language Runtime Requirements
stc_id: STC-CODEX-GE-04
artifact_type: generated-documentary-output
status: draft
scope: programs/codex/requirements/GE-04-rules-engine-and-explainability-core/artifacts
source_stc: ../README.md
source_inputs:
  - ../../GE-02-canonical-rules-model-and-content-packages/artifacts/expression-language-decision-criteria.md
---

# GE-04 Expression-Language Runtime Requirements

## Purpose
Extend GE-02 expression-language decision criteria into GE-04 runtime requirements for formula and prerequisite evaluation without choosing a final evaluator.

## Required runtime qualities
Any future GE-04 evaluator MUST support deterministic evaluation, sandboxed execution with no filesystem/network/process/clock/random/mutable host side effects unless explicitly modeled, typed values or equivalent validation, structured expression representation, dependency discovery, provenance linkage, diagnostics for parse/type/reference/unsupported/unsafe failures, and deferred expression records when conversion is not safe.

## Formula result expectations
Formula evaluation SHOULD return a structured result resembling:

```yaml
expression_id: <stable id>
source_ref: <canonical/provenance ref>
status: pass | fail | unknown | diagnostic
value: <typed value or null>
dependencies: []
diagnostics: []
explanation_node_ref: <node id>
```

This is conceptual, not final schema authority.

## Prerequisite result expectations
Prerequisite evaluation SHOULD return a structured result resembling:

```yaml
prerequisite_id: <stable id>
subject_ref: <character/object/input ref>
status: satisfied | not_satisfied | unknown | diagnostic
expected: <condition description or structured expression>
actual: <observed value/state or null>
dependencies: []
diagnostics: []
explanation_node_ref: <node id>
```

Failed prerequisites MUST expose expected-versus-actual information when available.

## Prohibited evaluator behavior
A future evaluator MUST NOT execute arbitrary user/plugin code with host side effects, silently coerce invalid expressions into zero/false/default values, evaluate formulas without dependencies, return a naked prerequisite boolean when expected/actual detail exists, hide unsupported legacy fragments in generic failures, or claim PCGen parity without GE-05 evidence.

## Candidate-selection boundary
This artifact does not choose CEL, Rhai, a custom DSL, embedded Rust, or any other evaluator. Selection requires a later spike or decision record that tests pilot formula/prerequisite pressure and documents tradeoffs.

## Minimum pilot expression pressures
The pilot evaluator requirements should expect pressure from ability-score modifiers, class progression formulas, save and attack-related formulas, skill-related values, feat/proficiency prerequisites, equipment-affected derived values, and choice-set filters/selectors.
