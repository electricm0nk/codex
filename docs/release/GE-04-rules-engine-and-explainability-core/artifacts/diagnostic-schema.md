---
title: GE-04 Diagnostic Schema
stc_id: STC-CODEX-GE-04
artifact_type: generated-documentary-output
status: draft
scope: programs/codex/requirements/GE-04-rules-engine-and-explainability-core/artifacts
source_stc: ../README.md
source_inputs:
  - ../../GE-02-canonical-rules-model-and-content-packages/artifacts/content-validation-and-diagnostics-specification.md
---

# GE-04 Diagnostic Schema

## Purpose
Define the rules-engine diagnostic schema and taxonomy required before engine behavior can be trusted, tested, compared, or displayed.

## Diagnostic classes

| Diagnostic class | Meaning | Claim impact |
|---|---|---|
| `invalid_content` | Canonical source package or object violates model/validation expectations. | Blocks computation claim for affected scope. |
| `invalid_character_input` | Character fixture or user selection is invalid or references unavailable content. | Blocks affected character computation. |
| `unsupported_imported_semantics` | Imported legacy behavior remains unsupported, lossy, deferred, or intentionally ignored. | Blocks support/parity claims for affected behavior. |
| `unresolved_reference` | Object, effect, formula, prerequisite, selector, or choice reference cannot resolve. | Blocks affected value/choice. |
| `invalid_expression` | Formula or prerequisite expression cannot parse, type-check, or evaluate safely. | Blocks affected formula/prerequisite/value. |
| `circular_dependency` | Evaluation dependency cycle prevents stable computation. | Blocks affected computation until cycle policy is defined. |
| `unstable_evaluation` | Evaluation result changes or cannot settle under the chosen order/strategy. | Blocks affected computation. |
| `provenance_gap` | Source lineage is insufficient for debugging, explanation, or parity. | Blocks explanation/parity claims as applicable. |
| `engine_defect` | Implementation failure, panic, impossible internal state, or invariant violation. | Blocks all affected claims and requires code correction. |
| `known_gap` | Accepted gap recorded for current scope, typically not claiming support. | Must travel with output and prevent overstated claims. |

## Required diagnostic fields
A future diagnostic record MUST contain enough information to support both tests and human debugging.

Minimum conceptual fields:

```yaml
id: <stable within run or deterministic where possible>
class: <diagnostic class>
severity: info | warning | error | blocker
message: <human-readable summary>
subject_ref: <object/input/effect/formula/prerequisite/choice/value ref>
source_ref: <canonical/provenance/source-map ref when available>
claim_blocking: true | false
blocked_claims: []
expected: <optional expected condition>
actual: <optional observed state>
recovery_owner: <GE/source or subsystem>
recovery_hint: <optional>
related_diagnostics: []
```

This is conceptual, not final schema authority.

## Severity posture
- `info` — useful context, does not affect claims.
- `warning` — behavior is questionable or incomplete but not blocking for the specific claim.
- `error` — affected behavior is invalid or unavailable.
- `blocker` — broader execution or claim must stop.

Severity must not be used to hide unsupported semantics. If a diagnostic affects the claimed output, it must carry claim-blocking posture.

## Diagnostic provenance rule
When behavior originates from imported content, diagnostics SHOULD link to provenance/source-map evidence. If that evidence is unavailable and the claim needs source lineage, emit `provenance_gap`.

## Engine defect rule
`engine_defect` diagnostics are not content diagnostics. They indicate the engine failed its own invariants. A future implementation must not downgrade engine defects into content warnings.

## Known-gap rule
Known gaps may exist, but they must travel with outputs. A known gap is not success; it is scoped honesty.
