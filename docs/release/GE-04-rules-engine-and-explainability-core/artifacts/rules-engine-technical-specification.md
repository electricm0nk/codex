---
title: GE-04 Rules Engine Technical Specification
stc_id: STC-CODEX-GE-04
artifact_type: generated-documentary-output
status: draft
scope: programs/codex/requirements/GE-04-rules-engine-and-explainability-core/artifacts
source_stc: ../README.md
---

# GE-04 Rules Engine Technical Specification

## Purpose
Define the concrete documentary rules-engine specification requirements GE-04 requires before implementation can claim computed behavior.

This is a GE-04 output artifact. It is not merely a requirement for the STC to inspect itself.

## Engine identity
The Codex rules engine is a headless deterministic computation and explanation subsystem.

It consumes validated canonical source-package content, imported provenance and diagnostics where applicable, and deterministic character input/fixture data.

It emits derived values, choice availability outcomes, prerequisite outcomes, diagnostics, explanation graph output, and structured output suitable for tests, CLI, future GE-05 comparison, and future GE-07 UI display.

## Minimum engine boundaries

| Boundary | Required role | Not owned here |
|---|---|---|
| Source-package boundary | Read canonical content, validation state, source maps, diagnostics, and compiled/evaluation-ready material. | Final source-package schema, importer behavior, or authored-content authority. |
| Character input boundary | Read fixture/user selections and normalize them into stable references. | Full character-builder UX. |
| Evaluation context | Provide current values, candidate effects, diagnostics, dependencies, and source references to evaluators. | Arbitrary scripting state or UI state. |
| Effect evaluator | Collect candidate effects, activate valid effects, and record contribution evidence. | Full Pathfinder-wide stacking doctrine unless required by pilot. |
| Formula/prerequisite evaluator | Evaluate formulas and predicates deterministically with diagnostics and dependency capture. | Final expression-language selection. |
| Choice engine | Calculate available/unavailable options with explanations. | UI presentation or broad homebrew authoring workflow. |
| Derived-value output | Emit pilot values and contribution breakdowns. | PCGen parity judgment. |
| Diagnostic emitter | Produce structured rules diagnostics. | Importer-only diagnostics unless they affect execution claims. |
| Explanation graph | Represent why values and availability outcomes occurred. | Final UI rendering. |

## Required output envelope
A future implementation output envelope SHOULD be able to represent:

```yaml
run_id: <stable or generated id>
source_packages: []
character_input_ref: <fixture or character id>
claim_level: computed
values: []
choices: []
prerequisites: []
diagnostics: []
explanation_graph_ref: <inline or artifact reference>
provenance_refs: []
known_gaps: []
```

This is a conceptual envelope, not final schema authority.

## Claim-level boundaries
GE-04 can support the `Computed` compatibility tier only when deterministic execution, expected outputs, explanations, and diagnostics exist.

GE-04 does not by itself support `Oracle-checked`, `Product-visible`, or broad Pathfinder support claims.

## Minimum proof obligations for implementation handoff
A later GE-04 execution handoff must require proof that a deterministic fixture can run without UI, derived values are emitted, at least one derived value has a source-contribution explanation, at least one failed prerequisite or unavailable choice has expected-versus-actual explanation, unresolved/unsupported content is diagnostic-visible, and tests fail when expected values/explanation edges/diagnostics are missing.

## What this artifact does not decide
Final Rust layout, final serialized schema syntax, final expression evaluator, exact GE-06 character fixture values, exact GE-05 oracle command, and final UI affordances remain undecided.
