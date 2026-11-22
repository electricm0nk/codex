---
title: GE-02 Expression-Language Decision Criteria
stc_id: STC-CODEX-GE-02
artifact_type: generated-documentary-output
status: accepted
scope: programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts
source_stc: ../README.md
---

# GE-02 Expression-Language Decision Criteria

## Purpose
Prescribe the decision criteria for future prerequisite and formula expression technology without prematurely selecting an implementation.

## Required qualities
Any future expression/formula/prerequisite substrate MUST support:
- deterministic evaluation
- sandboxed execution with no arbitrary host side effects
- typed values or equivalent validation for stats, saves, skills, equipment, choices, and effects
- structured AST or equivalent representation, not only free text
- source provenance from legacy token/formula to canonical expression record
- readable diagnostics for parse errors, unsupported constructs, unevaluated formulas, and unmet prerequisites
- dependency discovery so the engine can explain which values or objects an expression used
- deferred/unresolved expression records when conversion is not yet safe

## Candidate evaluation dimensions
A future ADR or spike SHOULD compare candidates on:
- expressiveness required by PF1 pilot formulas and predicates
- safety/sandbox properties
- ease of parsing legacy formula fragments into structured form
- inspectability and explainability
- ability to serialize expressions in source packages
- compatibility with compiled runtime IR
- test fixture ergonomics
- implementation complexity

## Explicitly deferred
This artifact does not choose CEL, Rhai, a custom DSL, embedded Rust/Python, or any other evaluator.

Selection is blocked until a downstream spike or decision surface tests the pilot formula/prerequisite pressures from GE-01.
