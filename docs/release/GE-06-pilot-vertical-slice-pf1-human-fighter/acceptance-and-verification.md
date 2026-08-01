---
title: GE-06 Acceptance and Verification
stc_id: STC-CODEX-GE-06
artifact_type: acceptance-and-verification
status: draft
scope: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter
source_stc: ./README.md
source_artifacts:
  - ./README.md
  - ./technical-requirements.md
  - ./technical-design.md
  - ./references/upstream-dependency-contract.md
  - ./artifacts/pilot-charter-alignment.md
  - ./artifacts/pilot-character-fixture-requirements.md
  - ./artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md
  - ./artifacts/ge06-e2-f1-execution-readiness-closure-2026-06-21.md
  - ./artifacts/ge06-e2-f1a-execution-handoff-2026-06-21.md
  - ./artifacts/ge06-e2-f1a-merge-receipt-2026-06-21.md
  - ./artifacts/ge06-e2-f2a-execution-readiness-closure-2026-06-21.md
  - ./artifacts/ge06-e2-f2a-execution-handoff-2026-06-21.md
  - ./artifacts/ge06-e2-f2a-merge-receipt-2026-06-21.md
  - ./artifacts/ge06-e2-f2b-execution-readiness-closure-2026-06-21.md
  - ./artifacts/ge06-e2-f2b-execution-handoff-2026-06-21.md
  - ./artifacts/ge06-e2-f2b-merge-receipt-2026-06-21.md
  - ./artifacts/ge06-e2-f2c-execution-readiness-closure-2026-06-21.md
  - ./artifacts/ge06-e2-f2c-execution-handoff-2026-06-21.md
  - ./artifacts/ge06-e2-f2c-merge-receipt-2026-06-21.md
  - ./artifacts/ge06-e2-f2d-execution-readiness-closure-2026-06-21.md
  - ./artifacts/ge06-e2-f2d-execution-handoff-2026-06-21.md
  - ./artifacts/ge06-e2-f2d-merge-receipt-2026-06-21.md
  - ./artifacts/ge06-e2-f3-execution-readiness-closure-2026-06-21.md
  - ./artifacts/ge06-e2-f3-execution-handoff-2026-06-21.md
  - ./artifacts/ge06-e2-f3-merge-receipt-2026-06-21.md
  - ./artifacts/ge06-post-f2d-handoff-rack-2026-06-21.md
  - ./artifacts/ge06-post-f3-handoff-rack-2026-06-21.md
  - ./artifacts/ge06-e3-f3-viability-evidence-bundle-2026-06-22.md
  - ./artifacts/ge06-post-e3-f3-evidence-rack-2026-06-22.md
  - ./artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md
  - ./artifacts/ge06-post-e5-f1-decision-rack-2026-06-22.md
  - ./artifacts/ge06-e5-f2-narrow-vs-expand-decision-2026-06-22.md
  - ./artifacts/ge06-post-e5-f2-decision-rack-2026-06-22.md
  - ./artifacts/ge06-e5-f3-upstream-delta-review-2026-06-22.md
  - ./artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md
  - ./artifacts/ge06-e4-f1-execution-handoff-2026-06-22.md
  - ./artifacts/ge06-e4-f2-prebuild-readiness-closure-2026-06-22.md
  - ./artifacts/ge06-e4-f2-prebuild-handoff-2026-06-22.md
  - ./artifacts/ge06-e4-f3-prebuild-readiness-closure-2026-06-22.md
  - ./artifacts/ge06-e4-f3-prebuild-handoff-2026-06-22.md
  - ./execution-handoff.md
  - ./artifacts/required-token-family-list-requirements.md
  - ./artifacts/required-canonical-object-list-requirements.md
  - ./artifacts/pilot-stack-viability-decision-criteria.md
---

# GE-06 Acceptance and Verification

## Objective
Define the checks that prove the GE-06 source STC is complete enough for planning and later bounded implementation-handoff derivation, without pretending that the pilot vertical slice has already been built or proven viable.

## Acceptance posture
GE-06 acceptance at this stage is documentary and falsifiable. It proves the integrated proof contract exists. It does not prove the integrated code path already runs.

## AT-06-001 — Source STC bundle exists and is internally linked
**Given** the GE-06 spec domain requires a source STC at `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/`  
**When** the source STC is reviewed  
**Then** the rich bundle exists and links upward to the spec domain, roadmap, pilot charter, doctrine surfaces, and upstream GE-01 through GE-05 planning inputs.

Evidence:
- `README.md`
- `technical-requirements.md`
- `technical-design.md`
- `acceptance-and-verification.md`
- `risks-and-open-questions.md`
- `epic-breakdown.md`
- `references/upstream-dependency-contract.md`

## AT-06-002 — Required GE-06 documentary outputs exist
**Given** the GE-06 spec domain names concrete required outputs  
**When** the package is inspected  
**Then** each output class is materialized as an exact artifact path with a completion rule or dedicated requirements content.

Evidence:
- `artifacts/pilot-charter-alignment.md`
- `artifacts/pilot-character-fixture-requirements.md`
- `artifacts/required-token-family-list-requirements.md`
- `artifacts/required-canonical-object-list-requirements.md`
- `artifacts/pilot-stack-viability-decision-criteria.md`
- `acceptance-and-verification.md`
- `artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md`

## AT-06-003 — Upstream dependency truth is preserved
**Given** GE-06 integrates multiple upstream epics  
**When** the requirements and dependency contract are reviewed  
**Then** the package explicitly states what GE-01, GE-02, GE-03, GE-04, GE-05, and GE-07 own, and what GE-06 may rely on without claiming that runtime outputs already exist.

Evidence:
- `README.md` Authority and Scope, Required Reads, and Next Stage Rule
- `technical-requirements.md` TR-06-002 and TR-06-003
- `references/upstream-dependency-contract.md`

## AT-06-004 — Pilot fixture is grounded but not fabricated
**Given** the pilot charter defines the first case identity and initial acceptance target  
**When** the GE-06 pilot fixture artifact is reviewed  
**Then** grounded selections are preserved, first-pilot input selections are closed by the deterministic input contract, and final computed/parity values are not invented.

Evidence:
- `artifacts/pilot-character-fixture-requirements.md`
- `artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md`
- `../GE-04-rules-engine-and-explainability-core/artifacts/pilot-golden-computation-fixture-requirements.md`
- `../../plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md`

## AT-06-005 — Required token families are explicit
**Given** the integrated slice cannot be evaluated honestly without knowing which legacy surfaces matter  
**When** the GE-06 token-family artifact is reviewed  
**Then** the hard-gate and supporting token families are enumerated from grounded GE-01 inputs rather than summarized as vague import coverage.

Evidence:
- `artifacts/required-token-family-list-requirements.md`
- `../GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-token-taxonomy.csv`
- `../GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv`

## AT-06-006 — Required canonical objects are explicit
**Given** the integrated slice must consume canonical content rather than direct LST syntax  
**When** the canonical-object artifact is reviewed  
**Then** the minimum GE-02 model homes and support records required for the pilot are enumerated explicitly.

Evidence:
- `artifacts/required-canonical-object-list-requirements.md`
- `../GE-02-canonical-rules-model-and-content-packages/artifacts/canonical-model-specification.md`

## AT-06-007 — End-to-end proof path is specified and headless-first
**Given** the roadmap and quality-gate policy require headless domain truth before UI truth  
**When** the GE-06 technical requirements and design are reviewed  
**Then** the integrated proof path names import, compute, explanation, parity, UI, and viability stages, and the UI portion is explicitly downstream of headless proof.

Evidence:
- `technical-requirements.md` TR-06-007 through TR-06-011
- `technical-design.md`
- `../../doctrine/quality-gate-policy.md`

## AT-06-008 — Failure taxonomy and viability criteria are explicit
**Given** the integrated slice can fail in multiple layers  
**When** the package is reviewed  
**Then** failures are required to resolve to model flaw, importer flaw, engine flaw, oracle gap, or UI gap, and the viability artifact defines fatal-flaw and narrowing triggers.

Evidence:
- `technical-requirements.md` TR-06-012 and TR-06-013
- `artifacts/pilot-stack-viability-decision-criteria.md`
- `risks-and-open-questions.md`

## AT-06-009 — UI truth contract refuses mock-state success
**Given** GE-06 includes a minimal UI slice  
**When** the package is reviewed  
**Then** the UI is explicitly required to consume real domain outputs, keep diagnostics visible, and avoid owning rules semantics.

Evidence:
- `technical-requirements.md` TR-06-011
- `technical-design.md`
- `../../plans/spec-domains/GE-07-desktop-shell-and-modern-ux.md`

## AT-06-010 — Source-STC generation did not authorize rolling code work
**Given** this run is source-STC generation  
**When** the package is inspected  
**Then** the source-STC pass itself does not silently authorize code, and any later coding route must arrive through a bounded stage-specific readiness closure and handoff rather than a rolling mutable brief.

Evidence:
- `README.md` Next Stage Rule
- `artifacts/ge06-e2-f1a-execution-handoff-2026-06-21.md`
- `artifacts/ge06-e2-f2a-execution-handoff-2026-06-21.md`

Supersession:
- This criterion described the initial source-STC generation pass. As of 2026-06-22, bounded coding gates have produced route-explicit handoffs and merge receipts through GE06-E3-F2, and the root `execution-handoff.md` remains a non-authorizing route surface that now points at documentary review truth plus any live bounded handoff pair rather than serving as code authority itself.

## AT-06-011 — Current execution route state is explicit after E4-F1 merge reconciliation
**Given** the deterministic pilot input contract is closed, GE06-E3-F2 and GE06-E3-F1 are merged, the E3 evidence bundle exists, the downstream viability and branch decisions have been written, and GE06-E4-F1 is now preserved by a readiness closure, historical handoff, and merge receipt  
**When** the E3 merge receipts, the E5-F1 decision artifact, the E5-F2 decision artifact, the E5-F3 review artifact, the E4-F1 readiness/handoff artifacts, the E4-F1 merge receipt, and the root execution route surface are inspected  
**Then** the package preserves that there is no active GE-06 code-authorizing handoff, records the merged E3 upstream evidence pair explicitly, preserves merged GE06-E4-F1 as historical authority, preserves GE06-E5-F1 and GE06-E5-F2 as completed documentary decisions, and keeps GE06-E5-F3 plus later packets routed truthfully instead of by vibes.

Evidence:
- `artifacts/ge06-e3-f2-execution-handoff-2026-06-22.md`
- `artifacts/ge06-e3-f2-merge-receipt-2026-06-22.md`
- `artifacts/ge06-e3-f1-execution-handoff-2026-06-22.md`
- `artifacts/ge06-e3-f1-merge-receipt-2026-06-22.md`
- `artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md`
- `artifacts/ge06-e5-f2-narrow-vs-expand-decision-2026-06-22.md`
- `artifacts/ge06-e5-f3-upstream-delta-review-2026-06-22.md`
- `artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md`
- `artifacts/ge06-e4-f1-execution-handoff-2026-06-22.md`
- `artifacts/ge06-e4-f1-merge-receipt-2026-06-26.md`
- `artifacts/ge06-post-e5-f2-decision-rack-2026-06-22.md`
- `execution-handoff.md`

## AT-06-012 — E3 fan-in evidence, downstream viability posture, and merged E4 historical lane are explicit and non-counterfeit
**Given** GE06-E3-F1 and GE06-E3-F2 are merged, the E3 bundle exists, the downstream viability and branch decisions must be grounded rather than narrated, and E4-F1 is now preserved as a merged bounded spike rather than an implied future launch  
**When** the E3 bundle, the E5-F1 decision artifact, the E5-F2 decision artifact, the post-E5-F2 rack, the E4 launch posture, the E4 readiness/handoff pair, the E4 merge receipt, and the root execution route surface are inspected  
**Then** the package records the exact selected pilot dimensions, their current `Computed` evidence, the explicit `OracleGap` blocker to stronger comparison claims, the blocked-path `EngineFlaw` example, the fact that GE06-E5-F1 fixes the current posture at `computed-but-not-oracle-checked`, the fact that GE06-E5-F2 chooses to narrow through GE-05 parity ownership, the fact that GE06-E4-F1 is preserved as a bounded rules-core view-model lane rather than shell work, and the fact that GE06-E5-F3 now exists as an explicit upstream delta/no-change review rather than a pending narration gap.

Evidence:
- `artifacts/ge06-e3-f3-viability-evidence-bundle-2026-06-22.md`
- `artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md`
- `artifacts/ge06-e5-f2-narrow-vs-expand-decision-2026-06-22.md`
- `artifacts/ge06-post-e5-f2-decision-rack-2026-06-22.md`
- `artifacts/ge06-e5-f3-upstream-delta-review-2026-06-22.md`
- `artifacts/ge06-e4-f1-launch-posture-2026-06-22.md`
- `artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md`
- `artifacts/ge06-e4-f1-execution-handoff-2026-06-22.md`
- `artifacts/ge06-e4-f1-merge-receipt-2026-06-26.md`
- `execution-handoff.md`

## AT-06-013 — The narrow-vs-expand branch is explicit and points at owning surfaces
**Given** GE06-E5-F1 already fixed the posture at `computed-but-not-oracle-checked` and named `OracleGap` as the primary blocker  
**When** the E5-F2 decision artifact, the post-E5-F2 rack, the GE-06 route surface, and the GE-05 route surface are inspected  
**Then** the package records that the correct branch is to narrow the pilot rather than expand requirements or stop for architectural failure, and the recommendation points at GE-05 parity ownership rather than a vague follow-up instruction.

Evidence:
- `artifacts/ge06-e5-f2-narrow-vs-expand-decision-2026-06-22.md`
- `artifacts/ge06-post-e5-f2-decision-rack-2026-06-22.md`
- `execution-handoff.md`
- `../GE-05-oracle-validation-and-parity-harness/execution-handoff.md`

## AT-06-014 — GE06-E4-F2 is prebuilt without counterfeit activation
**Given** TR-06-009 and TR-06-011 require inspectable explanation and diagnostic truth, GE06-E4-F2 depends on the real merged GE06-E4-F1 contract, and no active GE-06 coding lane remains after the GE06-E4-F1 merge receipt  
**When** the GE06-E4-F2 prebuild closure, the GE06-E4-F2 prebuild handoff draft, the post-E5-F2 rack, and the root execution route surface are inspected  
**Then** the package preserves GE06-E4-F2 as a downstream blocked-but-prebuilt packet, records the future artifact identities and candidate shell-facing write scope explicitly, and refuses to mint code authority until a later post-E4-F1 merge promotion pass re-grounds the live repo truth.

Evidence:
- `artifacts/ge06-e4-f2-prebuild-readiness-closure-2026-06-22.md`
- `artifacts/ge06-e4-f2-prebuild-handoff-2026-06-22.md`
- `artifacts/ge06-post-e5-f2-decision-rack-2026-06-22.md`
- `execution-handoff.md`

## AT-06-015 — GE06-E4-F3 is prebuilt without counterfeit activation
**Given** the pilot charter and deterministic input contract require one exportable summary boundary, GE06-E4-F3 depends on the real merged GE06-E4-F1 contract, and no active GE-06 coding lane remains after the GE06-E4-F1 merge receipt  
**When** the GE06-E4-F3 prebuild closure, the GE06-E4-F3 prebuild handoff draft, the post-E5-F2 rack, and the root execution route surface are inspected  
**Then** the package preserves GE06-E4-F3 as a downstream blocked-but-prebuilt packet, records the future artifact identities and candidate rules-core write scope explicitly, and refuses to mint code authority until a later post-E4-F1 merge promotion pass re-grounds the live repo truth.

Evidence:
- `artifacts/ge06-e4-f3-prebuild-readiness-closure-2026-06-22.md`
- `artifacts/ge06-e4-f3-prebuild-handoff-2026-06-22.md`
- `artifacts/ge06-post-e5-f2-decision-rack-2026-06-22.md`
- `execution-handoff.md`

## Exit gate checklist
- [x] The integrated pilot case is bounded by grounded charter facts.
- [x] Required token families are explicit.
- [x] Required canonical objects are explicit.
- [x] The end-to-end proof path is documented before any UI truth claim.
- [x] Failure taxonomy and stack-viability decision criteria are explicit.
- [x] First-pilot input selections are closed, while runtime facts and final expected values remain evidence-gated rather than invented.
- [x] The E3 fan-in evidence family now exists as an explicit documentary bundle rather than a pending derivation.
- [x] GE06-E4-F2 is preserved as explicit prebuild-only downstream truth rather than an implied future packet.
- [x] GE06-E4-F3 is preserved as explicit prebuild-only downstream truth rather than an implied future packet.

## Verification commands for this documentary package
A package-generation pass may verify the source-STC shape with file-existence and text checks such as:

```bash
python3 - <<'PY'
from pathlib import Path
base = Path('programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter')
required = [
    'README.md',
    'technical-requirements.md',
    'technical-design.md',
    'acceptance-and-verification.md',
    'risks-and-open-questions.md',
    'epic-breakdown.md',
    'references/upstream-dependency-contract.md',
    'artifacts/pilot-charter-alignment.md',
    'artifacts/pilot-character-fixture-requirements.md',
    'artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md',
    'artifacts/ge06-e2-f1-execution-readiness-closure-2026-06-21.md',
    'artifacts/ge06-e2-f1a-execution-handoff-2026-06-21.md',
    'artifacts/ge06-e2-f1a-merge-receipt-2026-06-21.md',
    'artifacts/ge06-e2-f2a-execution-readiness-closure-2026-06-21.md',
    'artifacts/ge06-e2-f2a-execution-handoff-2026-06-21.md',
    'artifacts/ge06-e2-f2a-merge-receipt-2026-06-21.md',
    'artifacts/ge06-e2-f2b-execution-readiness-closure-2026-06-21.md',
    'artifacts/ge06-e2-f2b-execution-handoff-2026-06-21.md',
    'artifacts/ge06-e2-f2b-merge-receipt-2026-06-21.md',
    'artifacts/ge06-e2-f2c-execution-readiness-closure-2026-06-21.md',
    'artifacts/ge06-e2-f2c-execution-handoff-2026-06-21.md',
    'artifacts/ge06-e2-f2c-merge-receipt-2026-06-21.md',
    'artifacts/ge06-e2-f2d-execution-readiness-closure-2026-06-21.md',
    'artifacts/ge06-e2-f2d-execution-handoff-2026-06-21.md',
    'artifacts/ge06-e2-f2d-merge-receipt-2026-06-21.md',
    'artifacts/ge06-e2-f3-execution-readiness-closure-2026-06-21.md',
    'artifacts/ge06-e2-f3-execution-handoff-2026-06-21.md',
    'artifacts/ge06-e2-f3-merge-receipt-2026-06-21.md',
    'artifacts/ge06-post-f2d-handoff-rack-2026-06-21.md',
    'artifacts/ge06-post-f3-handoff-rack-2026-06-21.md',
    'artifacts/ge06-e3-f3-viability-evidence-bundle-2026-06-22.md',
    'artifacts/ge06-post-e3-f3-evidence-rack-2026-06-22.md',
    'artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md',
    'artifacts/ge06-e5-f2-narrow-vs-expand-decision-2026-06-22.md',
    'artifacts/ge06-post-e5-f2-decision-rack-2026-06-22.md',
    'artifacts/ge06-e5-f3-upstream-delta-review-2026-06-22.md',
    'artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md',
    'artifacts/ge06-e4-f1-execution-handoff-2026-06-22.md',
    'artifacts/ge06-e4-f2-prebuild-readiness-closure-2026-06-22.md',
    'artifacts/ge06-e4-f2-prebuild-handoff-2026-06-22.md',
    'artifacts/ge06-e4-f3-prebuild-readiness-closure-2026-06-22.md',
    'artifacts/ge06-e4-f3-prebuild-handoff-2026-06-22.md',
    'execution-handoff.md',
    'artifacts/required-token-family-list-requirements.md',
    'artifacts/required-canonical-object-list-requirements.md',
    'artifacts/pilot-stack-viability-decision-criteria.md',
]
missing = [p for p in required if not (base / p).is_file()]
assert not missing, missing
for rel in required:
    text = (base / rel).read_text()
    assert text.startswith('---'), rel
    assert 'STC-CODEX-GE-06' in text, rel
root_index = Path('programs/codex/requirements/README.md').read_text()
assert 'GE-06 Pilot Vertical Slice: PF1 Human Fighter' in root_index
PY
```

These commands verify the documentary package. They do not verify integrated runtime behavior.

## Future implementation verification requirements
A later code-authorizing GE-06 handoff must define runnable verification commands for the selected slice. At minimum, those commands must eventually prove:
- the pilot case can be loaded or blocked with explicit diagnostics
- the pilot case can be computed headlessly with explanations
- selected old-vs-new comparison evidence exists or is blocked explicitly
- the UI consumes real outputs and preserves diagnostics/explanations when UI scope is in play
- failures are categorized with a primary owner
- the viability report records which selected outputs reached `Converted`, `Computed`, `Oracle-checked`, and `Product-visible`

## Completion rule
GE-06 source-STC planning is complete when the documentary package exists, the deterministic pilot input contract is closed, unresolved runtime facts remain honest, required output artifacts are named explicitly, the E5 branch decision explicitly chooses narrow/expand/stop from grounded evidence, and any implementation beyond completed merged slices is routed only through a bounded stage-specific code-authorizing handoff rather than implied by the source STC itself.
