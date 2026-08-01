---
title: GE-06 Upstream Dependency Contract
stc_id: STC-CODEX-GE-06
artifact_type: reference
status: draft
scope: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/references
source_stc: ../README.md
source_artifacts:
  - ../../GE-01-legacy-corpus-and-conversion-matrix/README.md
  - ../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-token-taxonomy.csv
  - ../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv
  - ../../GE-02-canonical-rules-model-and-content-packages/README.md
  - ../../GE-02-canonical-rules-model-and-content-packages/artifacts/canonical-model-specification.md
  - ../../GE-03-pcgen-import-pipeline-and-provenance/README.md
  - ../../GE-03-pcgen-import-pipeline-and-provenance/technical-requirements.md
  - ../../GE-04-rules-engine-and-explainability-core/README.md
  - ../../GE-04-rules-engine-and-explainability-core/technical-requirements.md
  - ../../GE-04-rules-engine-and-explainability-core/artifacts/pilot-golden-computation-fixture-requirements.md
  - ../../GE-05-oracle-validation-and-parity-harness/README.md
  - ../../GE-05-oracle-validation-and-parity-harness/technical-requirements.md
  - ../../GE-05-oracle-validation-and-parity-harness/references/upstream-dependency-contract.md
  - ../artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md
  - ../../../plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md
  - ../../../plans/spec-domains/GE-07-desktop-shell-and-modern-ux.md
  - ../../GE-07-desktop-shell-and-modern-ux/README.md
  - ../../../doctrine/quality-gate-policy.md
---

# GE-06 Upstream Dependency Contract

## Purpose
Record exactly what GE-06 may rely on from upstream planning surfaces, and what those surfaces do not authorize.

## Dependency map
| Upstream surface | Permitted GE-06 use | Does not authorize |
|---|---|---|
| GE-01 source STC | Grounded pilot corpus boundary, token-family taxonomy, conversion posture, and unsupported-token visibility. | Claiming imported runtime behavior, semantic conversion success, or parity from source discovery alone. |
| GE-01 pilot token taxonomy and conversion matrix | Exact pilot-critical token families, grounded object-class examples, and risk posture for import/model/engine work. | Treating “critical token family exists” as proof that it already converts, computes, or compares successfully. |
| GE-02 source STC and canonical-model specification | Canonical model homes, pilot minimum object set, diagnostic/provenance records, and runtime-boundary doctrine. | Inventing new semantic model homes or claiming final runtime behavior from model design alone. |
| GE-03 source STC and technical requirements | Importer, provenance, conversion-report, and unsupported-token diagnostic obligations that any integrated path must preserve. | Assuming actual importer outputs already exist for the pilot. |
| GE-04 source STC and technical requirements | Character-input, computation, explanation, diagnostic, and headless-entry obligations that the integrated slice must preserve. | Final expected values, production engine behavior, or parity evidence. |
| GE-04 pilot golden computation fixture requirements | Candidate output categories, explanation assertions, and warning that GE-06 must finalize the integrated pilot path. | Final Human Fighter selections or final computed values. |
| GE-05 source STC, technical requirements, and upstream dependency contract | Selected comparison posture, normalization boundaries, known-gap doctrine, and compatibility claim tiers relevant to the pilot. | Final old-system command route, final comparison outputs, or broad parity claims. |
| GE-06 final deterministic pilot input contract | Closed Human ability, feat-slot, skill-rank, equipment, active-state, and export-summary choices for the first bounded pilot case. | Final computed values, final old-system comparison outputs, broad UI work, or broad parity claims. |
| PF1 Human Fighter pilot charter | Pilot case identity, initial acceptance target, non-expansion rule, and downstream ownership split. | Broad Pathfinder scope, final integrated success, or a substitute for execution readiness. |
| GE-07 spec domain | UI consumer boundary and warning that broad UI work must stay downstream of proven behavior. | Final shell architecture or a UI implementation brief. |
| GE-07 source STC | Minimal shell boundary, component surfaces, command-boundary obligations, and the explicit rule that UI work stays non-authorizing until viability or a bounded spike posture grounds a later handoff. | Broad product UI authority, final repo write scope, verified runtime toolchain, or a ready-to-run GE06-E4-F1 handoff. |
| Quality gate policy | Evidence classes for documentation, import fidelity, rules correctness, oracle parity, UI truth, and compatibility claim tiers. | Specific commands, repo paths, or a code-authorizing handoff. |

## GE-06 obligations imposed by upstream inputs
GE-06 must:
- keep the pilot narrow and charter-aligned
- preserve GE-01 token-family visibility and GE-02 semantic model boundaries
- preserve GE-03 provenance/diagnostic visibility through the integrated path
- preserve GE-04 explanation and headless-computation truth before UI claims
- preserve GE-05 claim-tier and known-gap doctrine for selected parity dimensions
- treat GE-07 as the owner of broader UI architecture rather than absorbing it into GE-06
- classify integrated failures by primary owner instead of hiding them under “integration”

## What remains unresolved
The upstream inputs and final GE-06 deterministic input contract do not yet prove:
- the first runnable integrated headless command
- the final old-system command/export route for selected parity dimensions
- final computed values for the selected pilot outputs
- final oracle-checked comparison results for selected parity dimensions
- the final minimal UI implementation shape
- the exact branch/worktree/write scope for implementation
- whether the first product-visible UI lane should wait for a GE-06 viability verdict or proceed earlier only as a bounded non-production spike with explicitly grounded repo paths, toolchains, and verification receipts

## Propagation rule
If GE-06 discovery changes pilot boundary assumptions, required token families, canonical object needs, parity dimensions, or UI-minimum doctrine, the discovering handoff must update this STC and, when necessary, propagate deltas to:
- `../GE-01-legacy-corpus-and-conversion-matrix/` for newly discovered source or token-family implications
- `../GE-02-canonical-rules-model-and-content-packages/` when canonical object homes or runtime-boundary needs change
- `../GE-03-pcgen-import-pipeline-and-provenance/` when importer or provenance obligations change
- `../GE-04-rules-engine-and-explainability-core/` when computation, explanation, or fixture obligations change
- `../GE-05-oracle-validation-and-parity-harness/` when parity dimensions, known-gap doctrine, or comparison evidence expectations change
- `../../../plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md` when pilot scope itself changes
- `../../../doctrine/decisions/` when the change is architectural or scope-bearing
