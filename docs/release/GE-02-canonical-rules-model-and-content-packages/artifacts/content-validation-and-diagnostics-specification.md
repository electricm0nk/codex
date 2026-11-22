---
title: GE-02 Content Validation and Diagnostics Specification
stc_id: STC-CODEX-GE-02
artifact_type: generated-documentary-output
status: accepted
scope: programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts
source_stc: ../README.md
---

# GE-02 Content Validation and Diagnostics Specification

## Purpose
Prescribe validation and diagnostic outputs for canonical content packages before importer, engine, or UI work can trust them.

## Required validation classes
A content package validator MUST eventually check:
- manifest identity, version, game system, and dependency graph
- stable ID uniqueness and namespace validity
- object kind validity and required fields
- reference resolution across objects, effects, prerequisites, formulas, choices, and selectors
- source-map/provenance completeness for imported objects
- formula/prerequisite/choice parseability or explicit deferred posture
- unsupported/lossy/deferred/intentionally ignored behavior visibility
- compiled IR derivability only after source validation passes or emits accepted downgraded diagnostics

## Diagnostic classes
GE-02 requires at least these diagnostic classes:

| Diagnostic class | Meaning |
|---|---|
| `unsupported_construct` | Legacy construct has no accepted canonical model or handler. |
| `deferred_semantics` | Model home exists, but exact semantics are intentionally deferred. |
| `lossy_conversion_risk` | Conversion may lose behavior unless later work resolves the risk. |
| `unresolved_reference` | Canonical target or source reference cannot yet resolve. |
| `invalid_package_shape` | Package manifest/layout violates required structure. |
| `invalid_object_shape` | Object record violates kind-specific requirements. |
| `invalid_expression` | Formula/prerequisite/choice expression cannot parse or validate. |
| `provenance_gap` | Source lineage is too weak for debugging or parity claims. |

## Parity prohibition
No downstream artifact may claim import parity, engine parity, or UI correctness when relevant diagnostics remain unresolved for the claimed scope.
