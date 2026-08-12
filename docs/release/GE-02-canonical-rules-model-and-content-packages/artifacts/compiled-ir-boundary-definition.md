---
title: GE-02 Compiled IR Boundary Definition
stc_id: STC-CODEX-GE-02
artifact_type: generated-documentary-output
status: accepted
scope: programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts
source_stc: ../README.md
---

# GE-02 Compiled IR Boundary Definition

## Purpose
Define the boundary between canonical authored/imported source content and compiled runtime IR/cache material.

## Source package authority
The source package is the reviewable authority surface for canonical content.

It owns:
- package manifest and dependency graph
- stable IDs
- semantic objects
- effects, prerequisites, formulas, choices, selectors
- provenance/source maps
- diagnostics and validation state
- human-reviewable source representation

## Compiled IR role
Compiled runtime IR is derived material.

It may own:
- normalized references
- precomputed dependency graphs
- evaluation-ready expression representations
- validation summaries
- optimized rule-loading structures
- runtime cache metadata

It MUST NOT own:
- the only copy of authored content
- untraceable rule behavior
- silent corrections to invalid source content
- final doctrine about object homes or expression semantics

## Required traceability
Every compiled IR item that affects character computation MUST trace back to:
- source package
- canonical object or rule record
- validation outcome
- source-map/provenance record when imported
- diagnostic record when unresolved, lossy, partial, unsupported, or deferred

## Cache invalidation posture
Exact cache invalidation implementation is deferred, but the model MUST preserve enough package identity, version, dependency graph, and validation-state metadata to make invalidation deterministic later.
