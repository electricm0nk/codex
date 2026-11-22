---
title: GE-02 to GE-03 Dependency Reconciliation Review Handoff
handoff_id: HANDOFF-CODEX-GE-02-TO-GE-03-REVIEW-2026-06-19
handoff_kind: review-brief
work_type: review-only
workflow_route: review
readiness: review-ready
status: active
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/review-handoff.md
source_stc: programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/README.md
source_readiness: planning-ready
created_at: 2026-06-19
code_authority: false
---

# Review Handoff: GE-03 Dependency Reconciliation After GE-02 Closure

## Objective
Re-audit the GE-03 source STC against the now-created GE-02 source STC and generated GE-02 artifacts, then patch GE-03 so it no longer treats GE-02 as a missing dependency and carries only truthful remaining blockers.

## Work Type
`review-only`

This is documentary review and reconciliation work. It may modify Codex requirements documents, but it does not authorize implementation code.

## Workflow Route
`review`

## Readiness
`review-ready`

Why this handoff is ready:
- GE-02 is `planning-ready` and accepted as a source STC.
- GE-02 generated documentary outputs exist under `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/`.
- GE-03 currently contains grounded stale language saying the GE-02 source STC does not yet exist.
- The downstream task is bounded to documentation reconciliation and readiness review, not code implementation.

## Source STC
- path: `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/README.md`
- source readiness: `planning-ready`
- source review state: `accepted`

## Handoff Readiness Assessment
GE-02 is ready to produce this non-code review handoff.

GE-02 is **not** ready to produce a coding `execution-handoff.md` because:
- GE-02 is `planning-only`, not `implementation-ready`
- GE-02 workflow route is `planning`, not `coding`
- no bounded code implementation slice has been selected
- no code write scope has been granted
- no verification commands for repo-code work are specified

Therefore this handoff deliberately uses `review-handoff.md` and `review-ready`.

## Required Reads
Read these before making any changes:

1. `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/README.md`
   - confirms GE-02 readiness, authority, next-stage rule, and no-code boundary
2. `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/canonical-model-specification.md`
   - canonical model homes and relationships GE-03 must target
3. `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/content-package-layout-specification.md`
   - package/manifest layout assumptions GE-03 importer planning must respect
4. `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/provenance-source-map-specification.md`
   - lineage/source-map fields GE-03 parser/importer planning must preserve
5. `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/content-validation-and-diagnostics-specification.md`
   - diagnostics and validation classes GE-03 must not bypass
6. `programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/README.md`
   - current GE-03 control document and readiness state
7. `programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/technical-requirements.md`
   - current GE-03 normative dependency language
8. `programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/technical-design.md`
   - current GE-03 design dependency language
9. `programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/acceptance-and-verification.md`
   - current GE-03 acceptance criteria, especially GE-02 dependency posture
10. `programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/risks-and-open-questions.md`
    - current stale open questions around absent GE-02

## Conditional Reads
Read only if the trigger appears:

- `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/expression-language-decision-criteria.md`
  - only if GE-03 prerequisite/formula parsing language needs to distinguish parse preservation from evaluator selection
- `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/compiled-ir-boundary-definition.md`
  - only if GE-03 text mentions runtime IR, cache, or engine handoff boundaries
- `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv`
  - only if GE-03 dependency reconciliation needs exact legacy-to-canonical target mappings
- `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/unsupported-token-ledger.csv`
  - only if GE-03 diagnostics or unsupported-token wording needs exact GE-01 unresolveds
- `/home/ubuntu/workspace/repos/codex/AGENTS.md`
  - only if a later session tries to turn this review into repo-code implementation; otherwise do not read repo-local runtime instructions

## Source Universe / Inputs
Allowed input universe:
- Codex program requirements under `programs/codex/requirements/`
- Codex spec domains under `programs/codex/plans/spec-domains/`
- Codex pilot charter under `programs/codex/plans/pilot-slices/`
- Codex research/reference architecture only when a cited requirement needs grounding

Do not inspect or modify implementation code for this handoff.

## Required Output Artifacts
The downstream reviewer must produce or update these exact artifacts:

1. Modify: `programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/README.md`
   - replace stale “GE-02 source STC does not yet exist” language
   - add GE-02 source STC and relevant GE-02 generated artifacts to `source_artifacts` / required reads where appropriate
   - update readiness rationale and allowed assumptions
   - decide whether GE-03 can promote from `drafting` to `planning-ready`

2. Modify: `programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/technical-requirements.md`
   - replace stale “GE-02 source STC does not yet exist” dependency posture
   - cite GE-02 artifact outputs as canonical target-model inputs
   - preserve any real unresolveds that GE-02 still leaves open, especially expression/evaluator finality and implementation-specific schema details

3. Modify: `programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/technical-design.md`
   - update design context from “GE-02 absent” to “GE-02 planning-ready and accepted”
   - route parser/conversion-handler outputs toward GE-02 model homes and artifacts
   - keep code-authorizing boundaries explicit

4. Modify: `programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/acceptance-and-verification.md`
   - replace acceptance criteria that require missing-GE-02 honesty with criteria requiring correct GE-02 artifact usage
   - preserve no-code and no-parity-claim gates

5. Modify: `programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/risks-and-open-questions.md`
   - remove or close open questions whose only basis was GE-02 absence
   - add narrower remaining open questions for schema finality, expression/evaluator choice, source-span precision, and implementation handoff boundaries if still unresolved

6. Review and modify if necessary: `programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/epic-breakdown.md`
   - ensure downstream GE-03 epics reference GE-02 artifacts rather than a missing dependency

7. Create: `programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/artifacts/ge02-dependency-reconciliation-2026-06-19.md`
   - record the review verdict, files changed, stale dependency phrases removed, remaining real blockers, and readiness decision

## Output Placement Rules
- All GE-03 reconciliation edits must remain under `programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/`.
- The review receipt must live under `programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/artifacts/`.
- Do not write to `/home/ubuntu/workspace/repos/codex`.
- Do not write to `/home/ubuntu/workspace/repos/pcgen`.
- Do not create repo-local implementation docs from this handoff.

## In Scope
- GE-03 source-STC dependency reconciliation
- GE-03 readiness review after GE-02 closure
- stale dependency phrase removal
- linking GE-03 to GE-02 generated documentary artifacts
- preserving real unresolveds that GE-02 does not settle
- creating a GE-03 review receipt artifact

## Out of Scope
- parser implementation
- importer implementation
- token-handler implementation
- source-map writer implementation
- conversion-report CLI implementation
- tests or code in `/home/ubuntu/workspace/repos/codex`
- modifying legacy PCGen code or data
- declaring GE-03 `codex-ready`
- deriving a GE-03 code-authorizing `execution-handoff.md`
- final expression-language selection
- final runtime engine semantics
- parity or conversion-success claims

## Route-Specific Constraints
Review criteria:
- Every statement claiming GE-02 is absent must be removed or replaced.
- Every GE-03 dependency on canonical targets must cite the GE-02 source STC or concrete GE-02 output artifacts.
- GE-03 must remain honest about what GE-02 does not settle: final production schemas, expression/evaluator implementation, runtime engine behavior, exact code write scope, and future verification commands.
- If GE-03 still cannot promote to `planning-ready`, the review receipt must name the exact blocker.
- If GE-03 can promote to `planning-ready`, update its metadata and readiness rationale accordingly.

## Acceptance Criteria
The handoff is complete when:
- GE-03 contains no stale claim that the GE-02 source STC does not exist.
- GE-03 required/source artifacts cite `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/README.md` and the relevant `artifacts/` outputs.
- GE-03 risks/open questions distinguish resolved GE-02-absence blockers from remaining real model or implementation uncertainties.
- GE-03 acceptance criteria now verify correct GE-02 usage, not just honest GE-02 absence.
- A review receipt exists at `programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/artifacts/ge02-dependency-reconciliation-2026-06-19.md`.
- The reviewer records whether GE-03 is now `planning-ready` or still `drafting/blocked`, with evidence.
- No implementation code is written.

## Verification
Run or perform these checks before declaring completion:

1. Confirm required GE-02 source and artifact files exist:
   - `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/README.md`
   - `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/canonical-model-specification.md`
   - `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/content-package-layout-specification.md`
   - `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/provenance-source-map-specification.md`
   - `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/content-validation-and-diagnostics-specification.md`

2. Search GE-03 for stale phrases and confirm none remain unless quoted as historical evidence in the review receipt:
   - `GE-02 source STC does not yet exist`
   - `even though the GE-02 source STC does not yet exist`
   - `missing GE-02 source STC`
   - `GE-02 source STC is still absent`

3. Confirm GE-03 has no code-authorizing handoff:
   - no new `execution-handoff.md` should be created by this review
   - no write authority should be granted to `/home/ubuntu/workspace/repos/codex`

4. Confirm the review receipt exists and includes:
   - files changed
   - stale dependency statements removed/replaced
   - GE-02 artifacts cited
   - readiness verdict
   - remaining blockers or no-blocker statement

## Allowed Assumptions
- GE-02 is planning-ready and accepted for the PF1 Core Rulebook Human Fighter level 1 pilot boundary.
- GE-02 artifacts are documentary authority surfaces, not final production schemas.
- GE-03 remains a planning-only source STC until a later coding handoff exists.
- The local Codex repo at `/home/ubuntu/workspace/repos/codex` is future implementation context only, not write scope for this handoff.

## Blockers / Forbidden Assumptions
Stop and report if:
- a required GE-02 artifact is missing or empty
- GE-03 depends on a canonical-model fact not present in GE-02 artifacts and not safely deferrable
- promoting GE-03 would require inventing final schema, expression-language, runtime engine, branch/worktree, write scope, or verification commands
- the task begins to require implementation code

Forbidden assumptions:
- do not assume GE-02 planning-ready means production schema finality
- do not assume GE-03 can define canonical model homes independently of GE-02
- do not assume parser/importer work can start from this review handoff
- do not claim parser coverage, conversion success, provenance completeness, or parity success
- do not write to the Codex implementation repo or PCGen repo
