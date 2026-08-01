---
title: GE-05 Upstream Dependency Contract
stc_id: STC-CODEX-GE-05
artifact_type: reference
status: draft
scope: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/references
source_stc: ../README.md
source_artifacts:
  - ../../GE-01-legacy-corpus-and-conversion-matrix/README.md
  - ../../GE-01-legacy-corpus-and-conversion-matrix/references/oracle-surface-inventory.md
  - ../../GE-03-pcgen-import-pipeline-and-provenance/README.md
  - ../../GE-03-pcgen-import-pipeline-and-provenance/technical-requirements.md
  - ../../GE-04-rules-engine-and-explainability-core/README.md
  - ../../GE-04-rules-engine-and-explainability-core/technical-requirements.md
  - ../../GE-04-rules-engine-and-explainability-core/artifacts/pilot-golden-computation-fixture-requirements.md
  - ../../../plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md
  - ../../../doctrine/quality-gate-policy.md
---

# GE-05 Upstream Dependency Contract

## Purpose
Record exactly what GE-05 may rely on from upstream planning surfaces, and what those surfaces do not authorize.

## Dependency map

| Upstream surface | Permitted GE-05 use | Does not authorize |
|---|---|---|
| GE-01 source STC | Legacy corpus and oracle-surface discovery posture. | Runtime oracle output, final PCGen command, or parity claims. |
| GE-01 oracle surface inventory | Grounded static/source surfaces and explicit warning that PCGen runtime character-generation output is not grounded. | Treating static PCC/LST files as behavioral parity evidence. |
| GE-03 source STC and technical requirements | Import/provenance, conversion-report, unsupported-token diagnostic, fixture, and source-map obligations that new-system output must preserve. | Assuming importer implementation or canonical converted content already exists. |
| GE-04 source STC and technical requirements | Deterministic computation, explanation graph, diagnostics, headless entry, and pilot fixture requirements for Codex output. | Assuming rules-engine implementation, final expected values, or PCGen parity already exists. |
| GE-04 pilot golden computation fixture requirements | Candidate output categories and warning that exact values belong to later fixture/oracle work. | Final Human Fighter expected values or comparison evidence. |
| PF1 Human Fighter pilot charter | First case identity and initial acceptance target dimensions. | Broad Pathfinder support, final GE-06 integration success, or parity evidence. |
| Quality gate policy | Claim-tier and oracle parity gate doctrine. | Specific implementation commands or report artifacts. |

## GE-05 obligations imposed by upstream inputs
GE-05 must:

- compare only scoped behavior with explicit old/new evidence
- preserve GE-03 provenance and diagnostic visibility in comparison inputs
- preserve GE-04 explanation and diagnostic visibility in comparison inputs
- record unsupported, lossy, blocked, non-comparable, or intentionally divergent behavior instead of omitting it
- keep PCGen as oracle substrate, not architecture
- refuse to promote `Computed` claims to `Oracle-checked` without reproducible comparison evidence

## What remains unresolved
The upstream inputs do not yet prove:

- final PCGen runtime/export/validation command
- final old-system output for the Human Fighter pilot
- final Codex new-system output for the Human Fighter pilot
- final output normalization rules
- final parity report storage path
- exact branch/worktree/write scope for implementation
- exact verification commands for a future GE-05 code handoff
- legal/fixture-retention policy for PCGen-derived output

## Propagation rule
If GE-05 discovery changes assumptions about oracle surfaces, required outputs, known gaps, or comparison claim tiers, the discovering handoff must update this STC and, when necessary, propagate deltas to:

- `../GE-01-legacy-corpus-and-conversion-matrix/references/oracle-surface-inventory.md` for new oracle-surface facts
- `../GE-03-pcgen-import-pipeline-and-provenance/` when importer output requirements or diagnostics must change
- `../GE-04-rules-engine-and-explainability-core/` when engine output/explanation requirements must change
- `../../../doctrine/decisions/` when Codex intentionally diverges from observed PCGen behavior
- GE-06 planning when the pilot viability boundary changes
