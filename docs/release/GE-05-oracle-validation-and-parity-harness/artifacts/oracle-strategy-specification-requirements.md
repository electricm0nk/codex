---
title: GE-05 Oracle Strategy Specification Requirements
stc_id: STC-CODEX-GE-05
artifact_type: generated-documentary-output
status: draft
scope: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts
source_stc: ../README.md
related:
  - ../technical-requirements.md
  - ../references/upstream-dependency-contract.md
  - ../../GE-01-legacy-corpus-and-conversion-matrix/references/oracle-surface-inventory.md
---

# GE-05 Oracle Strategy Specification Requirements

## Purpose
Define the requirements for using PCGen as a bounded oracle for Codex behavior claims.

This artifact defines the evidence standard. It does not select a final PCGen command or prove that any route already works.

## Strategy principles
1. Prefer headless, repeatable oracle routes over GUI driving.
2. Treat PCGen as behavior evidence, not runtime architecture.
3. Keep static source truth separate from runtime behavior evidence.
4. Require exact scope for every compatibility claim.
5. Preserve raw output, normalized output, diagnostics, known gaps, and report evidence.
6. Do not preserve undesirable PCGen behavior without a decision record.

## Oracle surface classes

| Class | Use | Acceptance posture |
|---|---|---|
| Static corpus source | Establish loaded files, source declarations, token presence, and candidate semantics. | Supports hypotheses and source-truth references, not runtime parity. |
| PCGen documentation | Interpret token semantics when exact documentation is cited. | Medium trust; requires exact sections and may still differ from runtime. |
| PCGen code path | Explain loader or runtime mechanics when code is inspected. | Useful for diagnosis; not automatically a desired Codex design. |
| PCGen runtime output | Compare old-system behavior against Codex output. | Required for `Oracle-checked` claims. |
| PCGen export/stat block | Compare limited product-style outputs. | Practical when available; full export parity remains out of scope. |
| GUI-derived output | Last-resort behavior evidence. | Requires explicit risk acceptance and repeatability notes. |

## Minimum oracle-run record
Every future oracle run SHOULD produce a record with:

- case ID
- PCGen repo/build identity when available
- command, task, script, export path, or GUI route used
- working directory and relevant environment
- source package/campaign loaded
- character input or save-file identity
- raw output artifact path or captured output reference
- warnings, errors, and exit status
- limitations and non-comparable fields

## Claim standard
A behavior may be called `Oracle-checked` only when:

- the exact output dimension is named
- old-system evidence is captured or referenced
- new-system evidence is captured or referenced
- normalization, if any, is explicit
- the comparison result is recorded
- diffs or known gaps are recorded for failures or non-comparable outputs

## Prohibited shortcuts
The following do not satisfy oracle parity:

- static PCC/LST source inspection by itself
- a GE-04 rules-engine test by itself
- a UI screenshot
- a manually typed expected value without source evidence
- a report that omits failed or non-comparable dimensions
- a broad statement such as “Pathfinder works” or “PCGen parity works”

## Discovery completion rule
The oracle strategy is implementation-ready for its first slice only after a bounded discovery pass identifies the selected PCGen output route or records a hard blocker with evidence.
