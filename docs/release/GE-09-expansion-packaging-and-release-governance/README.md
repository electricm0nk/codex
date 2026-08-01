---
stc_id: STC-CODEX-GE-09
stc_kind: source-requirements
template_version: 2
work_type: planning-only
workflow_route: planning
readiness: planning-ready
status: active
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/GE-09-expansion-packaging-and-release-governance/README.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch: future GE-09 implementation work must branch from the then-current develop or another explicitly named dependency branch; this source STC grants no branch authority today
  write_scope: none during source STC generation; any future GE-09 implementation handoff must declare exact repo or operations surfaces explicitly
review_state: draft
last_reviewed_at: 2026-06-21
parent_scopes:
  - programs/codex
source_artifacts:
  - programs/codex/plans/spec-domains/GE-09-expansion-packaging-and-release-governance.md
  - programs/codex/plans/roadmaps/codex-spec-domain-roadmap-2026-06-18.md
  - programs/codex/plans/roadmaps/codex-execution-status-ledger-2026-06-21.md
  - programs/codex/doctrine/program-doctrine-and-scope-charter.md
  - programs/codex/doctrine/quality-gate-policy.md
  - programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/README.md
  - programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv
  - programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/unsupported-token-ledger.csv
  - programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/README.md
  - programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/known-gap-policy.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-post-e5-f2-decision-rack-2026-06-22.md
  - programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/README.md
  - programs/codex/plans/spec-domains/GE-08-homebrew-authoring-and-rules-studio.md
related_artifacts:
  - programs/codex/requirements/README.md
  - programs/codex/doctrine/decisions/README.md
upstream_targets:
  - programs/codex/README.md
  - programs/codex/requirements/README.md
expected_output_artifacts:
  - path: programs/codex/requirements/GE-09-expansion-packaging-and-release-governance/artifacts/expansion-scope-selection-policy.md
    completion_rule: Defines how new token families, source books, and package domains are selected by coverage impact, risk, evidence tier, and known-gap posture rather than by enthusiasm or book order.
  - path: programs/codex/requirements/GE-09-expansion-packaging-and-release-governance/artifacts/release-milestone-model.md
    completion_rule: Defines provisional release and distribution milestone classes and the minimum evidence gates each class must satisfy before stronger compatibility or release language is allowed.
  - path: programs/codex/requirements/GE-09-expansion-packaging-and-release-governance/artifacts/coverage-dashboard-requirements.md
    completion_rule: Defines the required fields, classifications, and review triggers for a governed coverage dashboard spanning token families, packages, evidence tiers, and known gaps.
  - path: programs/codex/requirements/GE-09-expansion-packaging-and-release-governance/artifacts/package-compatibility-and-migration-policy.md
    completion_rule: Defines package-versioning, compatibility-language, migration, and downgrade rules without overpromising behavior the pilot has not yet proven.
  - path: programs/codex/requirements/GE-09-expansion-packaging-and-release-governance/artifacts/known-gap-and-regression-governance.md
    completion_rule: Defines how release and expansion decisions consume GE-05 known-gap truth, regression findings, blockers, and accepted divergence records.
  - path: programs/codex/requirements/GE-09-expansion-packaging-and-release-governance/artifacts/contribution-intake-policy.md
    completion_rule: Defines when external or internal package contributions may enter the system, what validation and provenance gates apply, and why contribution intake stays gated until authoring posture is grounded.
supersedes: []
superseded_by: []
tags:
  - codex
  - ge-09
  - expansion
  - packaging
  - release-governance
  - compatibility
  - coverage
  - known-gaps
---

# GE-09 — Expansion, Packaging, and Release Governance

## Objective
Define the authoritative planning surface for how Codex grows beyond the pilot by token family, content package, compatibility claim, and release milestone without collapsing back into enthusiasm, vague support language, or counterfeit completion.

## Deliverable Type
`planning-only`

## Workflow Route
`planning`

## Readiness
`planning-ready`

Why this readiness is accurate:
- Todd explicitly commissioned GE-09 as documentary-first planning before any implementation or release authority exists, and that remains the truthful route.
- the GE-09 spec domain already names the objective, boundary, and required documentation shape for this work.
- GE-06 now has an explicit verdict (`computed-but-not-oracle-checked`) plus propagated routing truth that broad expansion is not yet the next honest move.
- GE-08 now has a planning-ready source STC, which grounds authoring/contribution dependency posture without granting contributor-workflow or implementation authority.
- GE-00 doctrine, the quality-gate policy, GE-01 conversion-control surfaces, GE-05 known-gap governance, and the execution-status ledger provide enough grounded evidence to rank candidate bands and define review cadence without fabricating actual release proof.
- the local Codex repo is grounded as the future implementation surface, but this STC grants no code, release, deployment, package-publication, or operations authority.

## Closure State
GE-09 remains a planning-ready source STC for expansion, packaging, and release governance. It is deliberately documentary-first. It now consumes the explicit GE-06 verdict and propagation truth plus the GE-08 planning-ready source STC, and it turns those inputs into ranked candidate bands and review cadence rather than pretending they authorize broad expansion, contributor workflow, or release operations.

## Authority and Scope
- owner: `Todd Hintzmann`
- scope: `program`
- canonical: `true`
- canonical path: `programs/codex/requirements/GE-09-expansion-packaging-and-release-governance/README.md`
- parent scopes:
  - `programs/codex`

This STC governs the Codex expansion-and-release planning surface. GE-01 remains the migration control plane for coverage truth. GE-05 remains the authority for known-gap and parity-report posture. GE-06 remains the authority for the current pilot claim ceiling. GE-08 now exists as the planning-ready authoring/homebrew authority surface, though it still grants no contributor-workflow or implementation authority by itself. GE-09 owns the policy layer that decides how those truths may later become package, release, and expansion claims.

## Target Runtime
- repo: `/home/ubuntu/workspace/repos/codex`
- workdir: `/home/ubuntu/workspace/repos/codex`
- branch/worktree: `future GE-09 implementation work must branch from the then-current develop or another explicitly named dependency branch; this source STC does not authorize a branch now`
- allowed write scope: `none during source STC generation; any future GE-09 implementation or operations handoff must declare exact repo or operational write surfaces explicitly`

The target repo is grounded only as the eventual implementation surface. This package is a planning authority surface under `programs/codex/requirements/`, not a repo-local implementation or release brief.

## Document Map
- `technical-requirements.md` — normative requirements for expansion selection, release gates, compatibility language, coverage dashboards, regression/known-gap posture, contribution intake, and cross-platform packaging milestones.
- `technical-design.md` — architecture/design response describing how evidence flows from GE-01/GE-05/GE-06/GE-08 into governed expansion and release decisions without counterfeit authority.
- `acceptance-and-verification.md` — observable checks proving the GE-09 source STC and same-epic documentary outputs exist and remain evidence-first.
- `risks-and-open-questions.md` — records unresolved release-authority, licensing, packaging, compatibility, contribution, and public-distribution questions.
- `epic-breakdown.md` — downstream planning and implementation slices that may emerge once pilot proof and authoring posture are grounded.
- `references/provisional-dependency-posture.md` — compact summary of what GE-06, GE-08, and the current GE-01 evidence basis do and do not currently authorize for GE-09.
- `artifacts/expansion-scope-selection-policy.md` — concrete policy for the current hold/go gate, ranked candidate bands, and rerank cadence.
- `artifacts/release-milestone-model.md` — concrete provisional release/distribution milestone model.
- `artifacts/coverage-dashboard-requirements.md` — concrete dashboard field and review-trigger requirements.
- `artifacts/package-compatibility-and-migration-policy.md` — concrete compatibility-language and versioning/migration rules.
- `artifacts/known-gap-and-regression-governance.md` — concrete rules for how known gaps and regressions affect release and expansion claims.
- `artifacts/contribution-intake-policy.md` — concrete gates for package contribution intake and review posture.

## Expected Output Artifacts
| Artifact | Completion rule |
|---|---|
| `artifacts/expansion-scope-selection-policy.md` | Defines how new token families, source books, and package domains are selected by coverage impact, risk, evidence tier, and known-gap posture rather than by enthusiasm or book order. |
| `artifacts/release-milestone-model.md` | Defines provisional release and distribution milestone classes and the minimum evidence gates each class must satisfy before stronger compatibility or release language is allowed. |
| `artifacts/coverage-dashboard-requirements.md` | Defines the required fields, classifications, and review triggers for a governed coverage dashboard spanning token families, packages, evidence tiers, and known gaps. |
| `artifacts/package-compatibility-and-migration-policy.md` | Defines package-versioning, compatibility-language, migration, and downgrade rules without overpromising behavior the pilot has not yet proven. |
| `artifacts/known-gap-and-regression-governance.md` | Defines how release and expansion decisions consume GE-05 known-gap truth, regression findings, blockers, and accepted divergence records. |
| `artifacts/contribution-intake-policy.md` | Defines when external or internal package contributions may enter the system, what validation and provenance gates apply, and why contribution intake stays gated until authoring posture is grounded. |

## Required Reads
- `../../plans/spec-domains/GE-09-expansion-packaging-and-release-governance.md` — primary strategic authority for this STC.
- `../../plans/roadmaps/codex-spec-domain-roadmap-2026-06-18.md` — stage ordering and Stage E intent.
- `../../plans/roadmaps/codex-execution-status-ledger-2026-06-21.md` — live route-state authority proving GE-09 is still documentary and that GE-06/GE-08 remain upstream truths rather than implied facts.
- `../../doctrine/program-doctrine-and-scope-charter.md` — program scope boundaries and drift-rejection rules.
- `../../doctrine/quality-gate-policy.md` — evidence classes, compatibility claim tiers, and explicit expansion gate.
- `../GE-01-legacy-corpus-and-conversion-matrix/README.md` — migration-control posture and scope-boundary doctrine for expansion planning.
- `../GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv` — grounded token-family and conversion-coverage truth used to prioritize expansion.
- `../GE-01-legacy-corpus-and-conversion-matrix/artifacts/unsupported-token-ledger.csv` — grounded unsupported/deferred debt that caps how aggressively GE-09 may broaden scope.
- `../GE-05-oracle-validation-and-parity-harness/README.md` — parity and report posture inherited by release governance.
- `../GE-05-oracle-validation-and-parity-harness/artifacts/known-gap-policy.md` — required policy for visible known gaps and non-comparable behavior.
- `../GE-06-pilot-vertical-slice-pf1-human-fighter/README.md` — current integrated pilot boundary and non-authorizing status.
- `../GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md` — explicit GE-06 verdict that caps current expansion claims at `computed-but-not-oracle-checked`.
- `../GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-post-e5-f2-decision-rack-2026-06-22.md` — propagated routing truth showing that the next mandatory proof burden is GE-05 parity, not broad expansion.
- `../GE-08-homebrew-authoring-and-rules-studio/README.md` — current planning-ready authoring boundary that GE-09 may consume without treating it as contributor-workflow authority.
- `../../plans/spec-domains/GE-08-homebrew-authoring-and-rules-studio.md` — strategic boundary for future authoring/homebrew policy that GE-09 must not inflate.
- `references/provisional-dependency-posture.md` — compact dependency and non-authorization summary for the current pass.

## Conditional Reads
- future doctrine records under `../../doctrine/decisions/` — when release authority, package signing, public distribution, or intentional divergence policy becomes explicit.
- `/home/ubuntu/workspace/repos/codex/AGENTS.md` — only if a later session proposes repo-facing implementation or packaging automation.
- future GE-05 parity reports or later GE-06 propagated-posture artifacts — use them to rerank candidate bands and replace the current gate posture where justified.
- future GE-08 readiness closures or contribution-path decisions — read them before tightening package-authoring, contribution, or compatibility policy.

## In Scope
- Codex GE-09 source-STC documents under `programs/codex/requirements/GE-09-expansion-packaging-and-release-governance/`
- concrete GE-09 same-epic documentary outputs under `programs/codex/requirements/GE-09-expansion-packaging-and-release-governance/artifacts/`
- evidence-first rules for selecting new token families, source books, and package domains
- provisional release/distribution milestone classes tied to documented evidence gates
- compatibility-language, versioning, migration, and downgrade rules for packages and claims
- coverage dashboard field requirements and review-trigger rules
- known-gap and regression governance for future releases and expansion claims
- contribution-intake gates for authored packages once authoring posture is grounded
- explicit dependency posture showing what GE-06 and GE-08 still block

## Out of Scope
- writing implementation, packaging, CI, release, or deployment code in `/home/ubuntu/workspace/repos/codex`
- modifying `/home/ubuntu/workspace/repos/pcgen`
- declaring the pilot viable before GE-06 produces an explicit verdict
- inventing final authoring workflow, contributor operations, or package-editing authority beyond what the current GE-08 planning-ready STC actually grounds
- public package registry, marketplace, billing, cloud services, or broad community governance
- final package-signing, trust-network, or release-authority mechanics absent an accepted decision surface
- claiming compatibility beyond the exact evidence tier reached for the exact package or scope named

## Acceptance Summary
The acceptance criteria in `acceptance-and-verification.md` are satisfied for this GE-09 planning boundary when the control bundle exists, the required same-epic documentary outputs exist, and each output is anchored to evidence rather than optimism.

Compact summary:
- GE-09 now has a canonical source STC and no longer lives only as spec-domain prose.
- the STC defines explicit artifact outputs for expansion selection, release milestones, coverage dashboards, package compatibility, known-gap/regression governance, and contribution intake.
- the STC preserves the decisive current truths: GE-06 is explicitly `computed-but-not-oracle-checked`, and GE-08 is real but still planning-only.
- the STC forbids counterfeit compatibility or release claims and makes expansion follow evidence, not enthusiasm.

## Allowed Assumptions
- the PF1 Core Rulebook Human Fighter level 1 pilot remains the first proof boundary unless a higher-order decision surface changes it.
- the current quality-gate policy and compatibility claim tiers remain authoritative until superseded by an accepted doctrine update.
- GE-01 conversion-matrix truth, GE-01 unsupported-token-ledger posture, and GE-05 known-gap posture are the current grounded evidence sources for discussing expansion priority.
- GE-06 now explicitly caps current claim posture at `computed-but-not-oracle-checked`; broader expansion still waits on stronger parity evidence.
- GE-08 may now be consumed as a planning-ready authoring boundary, but it still does not authorize final authoring workflow or contribution operations.

## Blockers / Forbidden Assumptions
- stop if a future handoff treats this STC as release authority, package-publication authority, or code-write authority.
- do not claim the pilot is viable, release-worthy, or broadly compatible while GE-06 remains `computed-but-not-oracle-checked`.
- do not invent package format finality, public distribution promises, or contribution mechanics merely because GE-08 now exists as a planning-ready source STC.
- do not let book order, nostalgia, or anecdotal demand override conversion-matrix coverage, known-gap posture, and risk-driven expansion selection.
- do not create marketplace, registry, package-signing, or public trust-network docs from this STC alone.

## Next Stage Rule
- GE-09 is planning-ready because its control bundle and required same-epic documentary outputs now exist and the current GE-06 / GE-08 posture is explicitly consumed.
- The next truthful GE-09 moves remain documentary review/planning slices that rerank or refine policy after parity, authoring-readiness, or doctrine changes; this is still not a coding handoff.
- Any later GE-09 code-authorizing or operations-authorizing handoff must be narrower than this spec domain and must name exact runtime surfaces, operators, verification receipts, and non-goals.
- Until stronger parity, authority, and operational truths land, GE-09 remains the governance shell that preserves the rules for future expansion rather than authorizing the expansion itself.
