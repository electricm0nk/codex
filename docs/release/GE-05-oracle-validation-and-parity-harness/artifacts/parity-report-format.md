---
title: GE-05 Parity Report Format
stc_id: STC-CODEX-GE-05
artifact_type: generated-documentary-output
status: draft
scope: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts
source_stc: ../README.md
related:
  - ../technical-requirements.md
  - ./golden-case-fixture-format.md
  - ./known-gap-policy.md
---

# GE-05 Parity Report Format

## Purpose
Define the report shape that records old-vs-new comparison evidence for a scoped Codex compatibility claim.

A parity report is the thing that prevents vague “parity works” language. It must show exactly what was compared, what matched, what differed, what was blocked, and what evidence supports the claim.

## Required report sections

### 1. Report metadata
- report ID
- generated-at timestamp or run ID
- case ID and case version
- harness version or implementation commit when available
- claim tier target and achieved claim state

### 2. Evidence sources
- legacy PCGen route and raw output reference
- Codex route and raw output reference
- normalization rule reference
- fixture reference
- commands/tests/receipts when available

### 3. Summary
- total dimensions compared
- pass count
- fail count
- blocked count
- known-gap count
- intentionally divergent count
- highest claim tier justified by the report

### 4. Dimension results
Each dimension result MUST include:

| Field | Required meaning |
|---|---|
| `dimension_id` | Stable ID for the compared output. |
| `description` | Human-readable compared behavior. |
| `old_source` | PCGen output value or reference. |
| `new_source` | Codex output value or reference. |
| `normalization` | Rule applied or `none`. |
| `old_normalized` | Normalized old value or reference. |
| `new_normalized` | Normalized new value or reference. |
| `status` | `pass`, `fail`, `blocked`, `known-gap`, or `intentionally-divergent`. |
| `diagnostics` | Relevant importer/rules/oracle/normalization diagnostics. |
| `diff` | Required when status is `fail`. |
| `known_gap_ref` | Required when status is `known-gap` or `blocked` for known reasons. |
| `decision_ref` | Required when status is `intentionally-divergent`. |

### 5. Actionable diffs
Failure records MUST identify:

- compared dimension
- old-system value/reference
- new-system value/reference
- normalized values when applicable
- delta classification
- likely owner when known
- relevant diagnostics
- next investigation target

### 6. Known gaps and blockers
The report MUST include all blocked or non-comparable dimensions. Omission is failure.

### 7. Claim statement
The report MUST end with a scoped claim statement in the accepted shape:

```text
For <source package / character path / token family>, new-system behavior matches legacy PCGen for <specific outputs>, proven by <comparison artifact>.
```

If the report cannot justify that claim, it MUST state the blocked claim instead.

## Status vocabulary

| Status | Meaning |
|---|---|
| `pass` | Old and new matched under declared normalization. |
| `fail` | Old and new differed; actionable diff required. |
| `blocked` | Required old or new evidence is unavailable. |
| `known-gap` | Non-comparable, unsupported, unresolved, or deferred behavior is recorded. |
| `intentionally-divergent` | PCGen behavior is known but not preserved by Codex, backed by decision record. |

## Report skeleton

```yaml
report_id: ge05-report-example
case_id: pf1-crb-human-fighter-level1
case_version: 0
claim_target: oracle-checked
claim_state: insufficient-evidence
sources:
  legacy_pcgen:
    route: unresolved
    raw_output_ref: null
  codex:
    route: unresolved
    raw_output_ref: null
summary:
  compared: 0
  pass: 0
  fail: 0
  blocked: 0
  known_gap: 0
  intentionally_divergent: 0
dimensions: []
claim_statement: blocked until old and new output evidence exists
```

## Completion rule
The report format is complete when a future harness can fail a report for missing evidence, missing diffs, missing known-gap records, or overbroad claim language.
