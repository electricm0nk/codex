---
title: GE-05 Known-Gap Policy
stc_id: STC-CODEX-GE-05
artifact_type: generated-documentary-output
status: draft
scope: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts
source_stc: ../README.md
related:
  - ../risks-and-open-questions.md
  - ./parity-report-format.md
---

# GE-05 Known-Gap Policy

## Purpose
Define how GE-05 handles behavior that cannot be compared, should not be preserved, or is not yet implemented.

Known gaps are not excuses. They are controlled truth surfaces that prevent silent false parity.

## Gap classes

| Gap class | Meaning | Claim effect |
|---|---|---|
| `oracle-route-unavailable` | PCGen cannot yet produce usable output for the dimension. | Blocks `Oracle-checked` claim. |
| `codex-output-unavailable` | New-system output does not yet exist. | Blocks comparison. |
| `unsupported-imported-semantics` | GE-03 conversion cannot represent required behavior yet. | Blocks or lowers trust; requires diagnostic/ledger linkage. |
| `rules-engine-debt` | GE-04 cannot compute or explain the value yet. | Blocks comparison until implemented. |
| `normalization-ambiguous` | Old/new values may be comparable but transformation is not accepted. | Blocks pass/fail until normalized honestly. |
| `non-comparable-output` | The systems expose outputs in ways that cannot currently be compared. | Must be reported; may be acceptable only if scoped. |
| `intentionally-divergent` | PCGen behavior is known but Codex chooses not to preserve it. | Requires decision record; not counted as accidental pass. |
| `legal-retention-limited` | Fixture/report cannot store output directly until retention policy is settled. | Requires reduced/reference/generated-on-demand evidence. |
| `out-of-pilot-scope` | Behavior exceeds the first Human Fighter pilot boundary. | Excluded from pilot claim; may route to GE-09 expansion governance. |

## Required ledger fields
A future known-gap ledger SHOULD record:

- gap ID
- case ID
- dimension ID
- gap class
- old-system evidence state
- new-system evidence state
- diagnostic references
- owner or owning GE
- blocking status
- accepted workaround, if any
- decision record reference, if intentional divergence
- review trigger
- last reviewed date

## Reporting rule
Every parity report MUST include known gaps that affect the case. A report that omits non-comparable dimensions to look green is invalid.

## Intentional divergence rule
If Codex should not preserve a PCGen behavior, the decision belongs under:

```text
programs/codex/doctrine/decisions/
```

The parity report may then mark the dimension `intentionally-divergent` only when it links to the decision record.

## Acceptable known gaps for the pilot
A known gap MAY be acceptable for pilot viability when all are true:

- it is visible in the report
- it is outside the minimum claim being made
- it does not invalidate the architecture proof purpose
- it has an owner and review trigger
- it does not permit broad compatibility language

A known gap is NOT acceptable when it hides a core pilot claim such as source load identity, essential derived values, or the comparison route itself.

## Failure behavior
When an output cannot be compared:

1. classify the gap
2. record old and new evidence state
3. link diagnostics or blockers
4. decide whether it blocks the scoped claim
5. route to owner or decision record
6. keep it in the report summary

## Completion rule
The known-gap policy is complete when a future comparator/report writer can make non-comparable behavior explicit and prevent it from being mistaken for a pass.
