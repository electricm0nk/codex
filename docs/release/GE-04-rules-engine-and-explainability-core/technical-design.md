---
title: GE-04 Technical Design
stc_id: STC-CODEX-GE-04
artifact_type: technical-design
status: draft
scope: programs/codex/requirements/GE-04-rules-engine-and-explainability-core
source_stc: ./README.md
---

# GE-04 Technical Design

This file is the architecture/design response to the GE-04 requirements. It describes the intended subsystem boundaries without becoming implementation code authority.

## Design posture
The Codex rules core should be a headless pipeline that consumes validated canonical content and a deterministic character input fixture, emits derived values, and attaches explanations and diagnostics to every tested behavior.

The engine is not the canonical source model. It is not the importer. It is not the oracle harness. It is the deterministic evaluation surface that makes later parity and UI work possible.

## Context and constraints
Inherited constraints:

- GE-02 owns source-package authority, model homes, diagnostics, provenance, expression criteria, and compiled-IR boundary posture.
- GE-03 owns importer bridge outputs and unresolved-token visibility.
- GE-05 owns oracle comparison and parity evidence.
- GE-06 owns the integrated pilot vertical slice.
- GE-00 requires headless core first and treats explainability as product behavior.

Design implication: GE-04 must define engine shape in terms of accepted canonical concepts, but must not choose final schemas, evaluator technology, repo layout, or parity claims.

## High-level pipeline
The intended rules-core pipeline is:

```text
canonical source package(s)
  -> package validation state
  -> compiled/evaluation-ready representation
  -> character input normalization
  -> candidate rule/effect collection
  -> prerequisite/formula/choice evaluation
  -> derived state calculation
  -> diagnostics
  -> explanation graph
  -> headless test/CLI outputs
```

Each stage must preserve evidence needed by later stages. A stage may emit diagnostics and stop or downgrade claims when required inputs are invalid, unsupported, or unresolved.

## Component boundaries

### Package input boundary
Accepts validated canonical source-package content or an explicitly diagnosed package state.

Responsibilities:
- identify package and dependency graph
- expose semantic objects, effects, prerequisites, formulas, choice sets, selectors, diagnostics, and source maps
- preserve GE-02 source authority; do not make compiled runtime IR the only source of truth

### Character input boundary
Accepts user/fixture selections for the pilot character.

Responsibilities:
- normalize selections into stable IDs or structured references
- distinguish chosen state from derived state
- record selection provenance for explanations
- reject or diagnose invalid input rather than coercing silently

### Evaluation context
The evaluation context is the runtime surface that holds current known values, candidate effects, dependency information, diagnostics, and explanation nodes.

Responsibilities:
- provide read-only access to canonical content and character input
- expose typed values or equivalent validation to formulas/prerequisites
- record dependencies discovered during formula/prerequisite evaluation
- prevent arbitrary host side effects

### Effect collection and activation
Effect collection gathers all potential contributions from canonical objects and selected character state.

Responsibilities:
- collect effects from race, class, feats, skills, equipment, selected choices, and package defaults
- separate candidate effects from active effects
- route activation conditions through prerequisite/formula evaluation
- emit diagnostics for unresolved, unsupported, circular, or invalid effects

### Formula evaluator
The formula evaluator computes symbolic numeric/string/boolean values required by derived stats and prerequisites.

Responsibilities:
- deterministic evaluation
- dependency capture
- typed/validated outputs
- diagnostic emission for invalid or unsupported expressions
- no I/O or mutable global state

Final evaluator selection is deferred.

### Prerequisite evaluator
The prerequisite evaluator computes eligibility and produces expected-versus-actual explanations.

Responsibilities:
- evaluate prerequisites against the evaluation context
- emit pass/fail/unknown outcomes
- record observed values and expected thresholds/conditions
- distinguish invalid character state from unsupported content semantics

### Choice availability engine
The choice availability engine computes selectable options and unavailable-choice explanations.

Responsibilities:
- enumerate candidate options
- apply selectors and prerequisites
- account for already-selected/repeatability constraints where relevant
- explain why each unavailable tested option is blocked
- surface unsupported selector or prerequisite semantics as diagnostics

### Derived value calculator
The derived value calculator produces pilot output values.

Responsibilities:
- compute values from baseline inputs and active effects
- preserve contribution breakdowns
- reject unstable/circular computations
- expose structured output for tests, CLI, GE-05 comparison, and GE-07 presentation consumers

### Diagnostic emitter
The diagnostic emitter normalizes engine issues into stable diagnostic records.

Responsibilities:
- classify source content, character input, unsupported import semantics, invalid expressions, cycles, provenance gaps, and engine defects separately
- link diagnostics to objects, expressions, effects, prerequisites, choices, source maps, and explanation graph nodes where available
- preserve severity and claim-blocking posture

### Explanation graph builder
The explanation graph builder records how outputs were produced.

Responsibilities:
- create nodes for inputs, source objects, effects, formulas, prerequisites, choices, derived values, diagnostics, and provenance
- create edges showing contribution, dependency, grant/modifier relationships, failed checks, selected choices, and source lineage
- make explanations useful for both human inspection and automated tests

### Headless entry point
The first implementation surface should expose rules behavior without UI.

Responsibilities:
- run deterministic fixtures
- emit derived values, explanations, and diagnostics
- fail when expectations are not met
- serve as the evidence source for GE-04 rules correctness and as input to later GE-05 comparison

## Data flow obligations
For every pilot output under test, a future implementation must be able to answer:

1. What character input and canonical objects contributed?
2. Which effects activated?
3. Which formulas or prerequisites were evaluated?
4. What values were observed?
5. Which diagnostics affected the result or claim level?
6. Which source/provenance records support imported behavior?
7. Why is this output eligible for computation, and what blocks parity or product-visible claims?

## Design non-goals
This design does not choose final Rust module layout, final expression evaluator, final serialized schemas, importer logic, PCGen oracle comparison, UI presentation, full Pathfinder stacking, spells, archetypes, multiclassing, or plugin behavior.

## Design review triggers
Reopen this design if GE-02 changes model boundaries, GE-03 implementation produces incompatible canonical content/diagnostics, GE-05 requires comparison outputs not represented here, GE-06 fixes pilot selections that change fixture requirements, or an ADR selects evaluator/stacking/circular-dependency policy.
