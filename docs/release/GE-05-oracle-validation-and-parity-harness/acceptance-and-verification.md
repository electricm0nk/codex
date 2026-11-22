---
title: GE-05 Acceptance and Verification
stc_id: STC-CODEX-GE-05
artifact_type: acceptance-and-verification
status: draft
scope: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness
source_stc: ./README.md
source_artifacts:
  - ./README.md
  - ./technical-requirements.md
  - ./technical-design.md
  - ./artifacts/oracle-strategy-specification-requirements.md
  - ./artifacts/golden-case-fixture-format.md
  - ./artifacts/parity-report-format.md
  - ./artifacts/initial-human-fighter-l1-expected-output-source-requirements.md
  - ./artifacts/known-gap-policy.md
---

# GE-05 Acceptance and Verification

## Objective
Define the checks that prove the GE-05 source STC is complete enough for planning and later bounded implementation handoff derivation, without pretending that PCGen parity has already been demonstrated.

## Acceptance posture
GE-05 acceptance at this stage is documentary and falsifiable. It proves the evidence standard exists. It does not prove the evidence has already been generated.

## AT-05-001 — Source STC bundle exists and is internally linked
**Given** the GE-05 spec domain requires a source STC at `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/`  
**When** the source STC is reviewed  
**Then** the required control bundle exists and links upward to the spec domain, roadmap, quality-gate policy, and upstream GE-01/GE-03/GE-04 inputs.

Evidence:
- `README.md`
- `technical-requirements.md`
- `technical-design.md`
- `acceptance-and-verification.md`
- `risks-and-open-questions.md`
- `epic-breakdown.md`
- `references/upstream-dependency-contract.md`

## AT-05-002 — Required GE-05 documentary outputs exist
**Given** the GE-05 spec domain names concrete required outputs  
**When** the STC package is inspected  
**Then** each output class is materialized as an exact artifact path with a completion rule and documentary specification.

Evidence:
- `artifacts/oracle-strategy-specification-requirements.md`
- `artifacts/golden-case-fixture-format.md`
- `artifacts/parity-report-format.md`
- `artifacts/initial-human-fighter-l1-expected-output-source-requirements.md`
- `artifacts/known-gap-policy.md`

## AT-05-003 — Reproducible old-vs-new comparison path is specified
**Given** the GE-05 exit gate requires a documented reproducible comparison path  
**When** the requirements and artifact specifications are reviewed  
**Then** the STC defines the required components of a future old-vs-new comparison path: fixture, PCGen output capture, Codex output capture, normalization, comparison, report, and known-gap routing.

Evidence:
- `technical-requirements.md` TR-05-004 through TR-05-010
- `technical-design.md` conceptual pipeline
- `artifacts/golden-case-fixture-format.md`
- `artifacts/parity-report-format.md`

## AT-05-004 — Parity failures require actionable diffs
**Given** a future comparison detects old-vs-new disagreement  
**When** the parity report is generated  
**Then** the report must record compared dimension, old value or reference, new value or reference, normalized values when applicable, delta classification, likely owner when known, diagnostics/known gaps, and next investigation target.

Evidence:
- `technical-requirements.md` TR-05-011
- `artifacts/parity-report-format.md`

## AT-05-005 — Known gaps are mandatory for non-comparable output
**Given** a future output cannot be compared  
**When** a comparison report is generated  
**Then** the output must be recorded as a blocked comparison, known gap, intentional divergence, or decision-record dependency; it must not be silently omitted.

Evidence:
- `technical-requirements.md` TR-05-012
- `artifacts/known-gap-policy.md`
- `risks-and-open-questions.md`

## AT-05-006 — Oracle evidence boundaries prevent counterfeit parity claims
**Given** the quality-gate policy only permits `Oracle-checked` claims with comparison evidence  
**When** the GE-05 source STC is reviewed  
**Then** it must explicitly prohibit parity claims from static PCGen source files, GE-04 computed values alone, UI screenshots, or plausible-looking new-system behavior.

Evidence:
- `README.md` Blockers / Forbidden Assumptions
- `technical-requirements.md` TR-05-003 and TR-05-013
- `artifacts/oracle-strategy-specification-requirements.md`

## AT-05-007 — Initial Human Fighter case is bounded but not fabricated
**Given** the pilot charter names the PF1 Core Rulebook Human Fighter level 1 target  
**When** the GE-05 expected-output source requirements are reviewed  
**Then** the STC must identify the first case, output categories, provenance/explanation requirements, and evidence requirements without inventing final old-system or new-system expected values.

Evidence:
- `artifacts/initial-human-fighter-l1-expected-output-source-requirements.md`
- `artifacts/golden-case-fixture-format.md`
- `../GE-04-rules-engine-and-explainability-core/artifacts/pilot-golden-computation-fixture-requirements.md`

## AT-05-008 — Source STC generation does not itself create code authority
**Given** this run is source-STC generation  
**When** the package is inspected  
**Then** no `execution-handoff.md` may be treated as active for GE-05 until a later execution-readiness closure grounds exact branch/worktree, write scope, selected implementation slice, required reads, and verification commands.

Current status note: that later closure was created at `artifacts/ge05-e2-f1-execution-readiness-closure-2026-06-20.md`, the resulting GE05-E2-F1 handoff has now merged, and `execution-handoff.md` has been retired back to a root route surface with `status: no-active-handoff`.

Evidence:
- `README.md` Next Stage Rule
- `execution-handoff.md` route-surface state
- `artifacts/ge05-e2-f1-execution-handoff-2026-06-20.md`
- `artifacts/ge05-e2-f1-merge-receipt-2026-06-21.md`

## Exit gate checklist
- [x] The pilot has a documented old-vs-new comparison **requirements path**.
- [x] Parity failures are required to produce actionable diffs.
- [x] Non-comparable outputs are required to become known gaps, blocked comparisons, or decision records.
- [x] Oracle evidence is defined well enough to decide what future evidence would make the stack viable.
- [x] The source STC names exact documentary artifact paths for the spec domain's required outputs.
- [x] Runtime PCGen command, final output values, branch/worktree, write scope, and verification commands remain explicitly unresolved rather than invented.

## Verification commands for this documentary package
A package-generation pass may verify the source STC shape with file-existence and text checks such as:

```bash
python3 - <<'PY'
from pathlib import Path
base = Path('programs/codex/requirements/GE-05-oracle-validation-and-parity-harness')
required = [
    'README.md',
    'technical-requirements.md',
    'technical-design.md',
    'acceptance-and-verification.md',
    'risks-and-open-questions.md',
    'epic-breakdown.md',
    'references/upstream-dependency-contract.md',
    'artifacts/oracle-strategy-specification-requirements.md',
    'artifacts/golden-case-fixture-format.md',
    'artifacts/parity-report-format.md',
    'artifacts/initial-human-fighter-l1-expected-output-source-requirements.md',
    'artifacts/known-gap-policy.md',
]
missing = [p for p in required if not (base / p).is_file()]
assert not missing, missing
for rel in required:
    text = (base / rel).read_text()
    assert text.startswith('---'), rel
    assert 'STC-CODEX-GE-05' in text, rel
assert 'GE-05 Oracle Validation and Parity Harness' in Path('programs/codex/requirements/README.md').read_text()
route_surface = (base / 'execution-handoff.md').read_text()
assert 'artifact_type: execution-route-surface' in route_surface
assert 'status: no-active-handoff' in route_surface
for rel in [
    'artifacts/ge05-e2-f1-execution-handoff-2026-06-20.md',
    'artifacts/ge05-e2-f1-merge-receipt-2026-06-21.md',
]:
    assert (base / rel).is_file(), rel
    assert 'STC-CODEX-GE-05' in (base / rel).read_text(), rel
PY
```

These commands verify the documentary package. They do not verify runtime parity.

## Future implementation verification requirements
A later code-authorizing GE-05 handoff must define runnable verification commands for the selected slice. At minimum, those commands must eventually prove:

- old-system oracle output can be produced or the blocker is recorded
- new-system output can be produced from the same case or the blocker is recorded
- normalization is explicit
- comparator failures produce actionable diffs
- non-comparable outputs become known-gap records
- the parity report records claim tier and evidence references

## Completion rule
GE-05 source-STC planning is complete when the documentary package exists, declares expected output artifacts, preserves all unresolved runtime facts honestly, and refuses to authorize implementation until a bounded execution-readiness closure is created.
