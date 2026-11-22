---
title: GE-02 Provenance and Source-Map Specification
stc_id: STC-CODEX-GE-02
artifact_type: generated-documentary-output
status: accepted
scope: programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts
source_stc: ../README.md
---

# GE-02 Provenance and Source-Map Specification

## Purpose
Prescribe the provenance and source-map outputs required for GE-02 so every imported canonical object, field, effect, formula, prerequisite, choice, and diagnostic can be traced back to legacy evidence.

## Required source-map fields
A source-map record MUST contain, when available:

| Field | Required meaning |
|---|---|
| `source_package_id` | Codex package identity, e.g. `pf1.crb`. |
| `source_system` | Legacy/source system identity, e.g. `pcgen`. |
| `pcc_path` | PCC root or include path that admitted the source. |
| `include_chain` | PCC include/dependency chain where known. |
| `lst_path` | LST source file path. |
| `entry_name` | Legacy object/entry name. |
| `source_span` | Line, token span, or structured location. |
| `legacy_construct` | Legacy token family or construct class. |
| `canonical_target_id` | Stable Codex target ID. |
| `canonical_target_field` | Field/effect/formula/prerequisite/choice touched. |
| `support_disposition` | exact, partial, deferred, unsupported, intentionally ignored, or equivalent accepted vocabulary. |
| `lossiness_class` | none-expected, medium-risk, high-risk, unknown-risk, or equivalent accepted vocabulary. |
| `diagnostic_refs` | Linked diagnostics for unresolved or lossy behavior. |
| `oracle_surface_refs` | Candidate or trusted oracle surface references where applicable. |

## Diagnostic requirements
Diagnostics MUST be first-class records. They MUST NOT be only prose comments.

A diagnostic MUST contain:
- stable diagnostic ID
- severity
- source-map reference
- canonical target reference when known
- reason
- downstream owner or recovery path
- whether behavior is blocking for import, engine execution, validation, or parity claims

## Source-span downgrade policy
If a future implementation cannot capture token-level spans immediately, it MUST record the downgrade explicitly:
- exact line span is preferred
- file + entry name is acceptable only if line/span capture is not yet available
- file-only lineage is allowed only for package/include surfaces or explicit early-stage diagnostics
- unknown lineage is a blocker for parity claims

## Oracle linkage rule
GE-02 may cite oracle surfaces as source-truth or future comparison candidates. It MUST NOT claim behavioral parity unless a later GE-05 comparison artifact records executable evidence.
