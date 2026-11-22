---
stc_id: STC-CODEX-GE-06
stc_kind: source-requirements
template_version: 2
work_type: planning-only
workflow_route: planning
readiness: planning-ready
status: active
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch: future GE-06 implementation handoffs should branch from current develop unless an explicit dependency branch is named; origin/develop re-verified 2026-06-26 is now at `cc4e1a5` after merged GE06-E4-F1 via `a11f7a4` / `1840cd9`, the root route surface is now retired to `no-active-handoff`, the GE06-E3-F3 evidence bundle plus the E5 documentary chain remain review truth, `artifacts/ge06-e4-f1-merge-receipt-2026-06-26.md` preserves the last bounded GE-06 coding lane, and the next mandatory proof burden still routes through GE-05 parity ownership
  write_scope: source STC itself grants none; any future GE-06 implementation handoff must declare exact repo paths, allowed writes, and whether it is headless-only or UI-facing
review_state: draft
last_reviewed_at: 2026-06-26
parent_scopes:
  - programs/codex
source_artifacts:
  - programs/codex/plans/spec-domains/GE-06-pilot-vertical-slice-pf1-human-fighter.md
  - programs/codex/plans/roadmaps/codex-spec-domain-roadmap-2026-06-18.md
  - programs/codex/plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md
  - programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/README.md
  - programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-token-taxonomy.csv
  - programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv
  - programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/README.md
  - programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/canonical-model-specification.md
  - programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/README.md
  - programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/technical-requirements.md
  - programs/codex/requirements/GE-04-rules-engine-and-explainability-core/README.md
  - programs/codex/requirements/GE-04-rules-engine-and-explainability-core/technical-requirements.md
  - programs/codex/requirements/GE-04-rules-engine-and-explainability-core/artifacts/pilot-golden-computation-fixture-requirements.md
  - programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/README.md
  - programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/technical-requirements.md
  - programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/references/upstream-dependency-contract.md
  - programs/codex/plans/spec-domains/GE-07-desktop-shell-and-modern-ux.md
  - programs/codex/doctrine/program-doctrine-and-scope-charter.md
  - programs/codex/doctrine/documentation-control-plane.md
  - programs/codex/doctrine/quality-gate-policy.md
related_artifacts:
  - programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/execution-handoff.md
  - programs/codex/requirements/GE-04-rules-engine-and-explainability-core/execution-handoff.md
  - programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/research-handoff.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-handoff-readiness-closure-2026-06-20.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f1-execution-readiness-closure-2026-06-21.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f1a-execution-handoff-2026-06-21.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f1a-merge-receipt-2026-06-21.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2a-execution-readiness-closure-2026-06-21.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2a-execution-handoff-2026-06-21.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2a-merge-receipt-2026-06-21.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2b-execution-readiness-closure-2026-06-21.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2b-execution-handoff-2026-06-21.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2b-merge-receipt-2026-06-21.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2c-execution-readiness-closure-2026-06-21.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2c-execution-handoff-2026-06-21.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2c-merge-receipt-2026-06-21.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2d-execution-readiness-closure-2026-06-21.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2d-execution-handoff-2026-06-21.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f1-prebuild-readiness-closure-2026-06-21.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f1-prebuild-handoff-2026-06-21.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f2-prebuild-readiness-closure-2026-06-21.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f2-prebuild-handoff-2026-06-21.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f1-execution-readiness-closure-2026-06-22.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f1-execution-handoff-2026-06-22.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f2-execution-readiness-closure-2026-06-22.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f2-execution-handoff-2026-06-22.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/research-handoff.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f3-viability-evidence-bundle-2026-06-22.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-post-e3-f3-evidence-rack-2026-06-22.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-post-e5-f1-decision-rack-2026-06-22.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e5-f2-narrow-vs-expand-decision-2026-06-22.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-post-e5-f2-decision-rack-2026-06-22.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e5-f3-upstream-delta-review-2026-06-22.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-handoff-2026-06-22.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-merge-receipt-2026-06-26.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f2-prebuild-readiness-closure-2026-06-22.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f2-prebuild-handoff-2026-06-22.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f3-prebuild-readiness-closure-2026-06-22.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f3-prebuild-handoff-2026-06-22.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/execution-handoff.md
  - programs/codex/requirements/README.md
upstream_targets:
  - programs/codex/README.md
  - programs/codex/requirements/README.md
expected_output_artifacts:
  - path: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/pilot-charter-alignment.md
    completion_rule: Maps GE-06 integrated-slice obligations to the pilot charter, records whether charter edits or ADR triggers are required, and refuses silent scope expansion.
  - path: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/pilot-character-fixture-requirements.md
    completion_rule: Defines the PF1 Human Fighter level 1 fixture boundary and points to the accepted deterministic input contract for closed choices, required output categories, explanation duties, and inherited GE-04/GE-05 proof obligations without fabricating final parity values.
  - path: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md
    completion_rule: Closes the Human ability bonus, feat-slot, skill-rank, equipment, active-state, and export-summary input choices needed before deriving a bounded headless implementation handoff.
  - path: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/required-token-family-list-requirements.md
    completion_rule: Enumerates the GE-01 token families the vertical slice depends on and classifies which are hard integration gates versus supporting-but-still-visible scope.
  - path: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/required-canonical-object-list-requirements.md
    completion_rule: Enumerates the minimum GE-02 canonical model homes, supporting records, and runtime-boundary obligations the integrated slice must consume or validate.
  - path: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/acceptance-and-verification.md
    completion_rule: Defines the end-to-end acceptance criteria proving that import, compute, explanation, selected oracle comparison, diagnostic visibility, failure classification, and minimal UI truth are all bounded and falsifiable.
  - path: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/pilot-stack-viability-decision-criteria.md
    completion_rule: Defines pass/fail stack-viability criteria, fatal-flaw signals, narrowing triggers, and required evidence thresholds tied to the Codex quality-gate tiers.
supersedes: []
superseded_by: []
tags:
  - codex
  - ge-06
  - pilot
  - vertical-slice
  - pf1
  - fighter
  - viability
  - explainability
  - oracle
---

# GE-06 — Pilot Vertical Slice: PF1 Human Fighter Level 1

## Objective
Define the authoritative integrated proof contract for one Pathfinder 1e Core Rulebook Human Fighter level 1 path: import grounded legacy content, load canonical content, compute the character, surface explanations and diagnostics, compare selected outputs against legacy PCGen, project the same real outputs into a minimal UI slice, and decide whether the stack survives.

## Deliverable Type
`planning-only`

## Workflow Route
`planning`

## Readiness
`planning-ready`

Why this readiness is accurate:
- the strategic GE-06 spec domain exists and explicitly frames this work as a boundary object rather than an implementation prompt
- GE-01 is accepted and grounds the pilot token families, corpus boundary, conversion posture, and unsupported-token visibility that the integrated slice depends on
- GE-02 is accepted and grounds the canonical model homes that the integrated slice must consume rather than reinvent
- GE-03, GE-04, and GE-05 source STCs exist and define the importer, computation/explainability, and oracle-comparison boundaries that GE-06 must integrate
- the pilot charter grounds the first case identity plus initial ability-score and feat seed values, and `artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md` closes the remaining skill, equipment, Human ability bonus, active-state, and additional feat/choice input debt for the first bounded pilot case
- the GE-07 spec domain and accepted GE-07 source STC now ground the minimal UI consumer boundary, but they still do not authorize broad UI implementation from GE-06; the first E4 lane remains bounded as a non-production spike and is now grounded through a stage-specific rules-core view-model handoff rather than shell work
- this bundle now includes the GE-06 control documents plus the concrete same-epic documentary outputs the spec domain requires
- the local Codex repo is grounded, GE06-E2-F1a, GE06-E2-F2a, GE06-E2-F2b, GE06-E2-F2c, GE06-E2-F2d, GE06-E2-F3, GE06-E3-F2, and GE06-E3-F1 have all completed as bounded develop-first slices, `artifacts/ge06-e3-f3-viability-evidence-bundle-2026-06-22.md` now materializes the E3 fan-in evidence family, `artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md` fixes the current downstream posture at `computed-but-not-oracle-checked`, `artifacts/ge06-e5-f2-narrow-vs-expand-decision-2026-06-22.md` records the decisive branch as narrow-the-pilot, the root route surface now truthfully preserves GE06-E4-F1 as merged historical authority through `artifacts/ge06-e4-f1-merge-receipt-2026-06-26.md` at `no-active-handoff`, and `artifacts/ge06-e5-f3-upstream-delta-review-2026-06-22.md` now records the resulting propagation/no-change verdict explicitly

## Closure State
GE-06 is a planning-ready source STC for the integrated pilot vertical-slice boundary. As of 2026-06-22, the final deterministic pilot input choices are closed by `artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md`; GE06-E2-F1a, GE06-E2-F2a, GE06-E2-F2b, GE06-E2-F2c, GE06-E2-F2d, GE06-E2-F3, GE06-E3-F2, and GE06-E3-F1 are all merged; `artifacts/ge06-e2-f3-merge-receipt-2026-06-21.md`, `artifacts/ge06-e3-f2-merge-receipt-2026-06-22.md`, and `artifacts/ge06-e3-f1-merge-receipt-2026-06-22.md` record the bounded headless, failure-routing, and selected-dimension merges at `6977c86`, `7bc89e8`, and `b2f2154`; `artifacts/ge06-e3-f3-viability-evidence-bundle-2026-06-22.md` materializes the E3 evidence family with explicit per-dimension `Computed` evidence and `OracleGap` blockers; `artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md` records that the strongest truthful current posture is still `computed-but-not-oracle-checked`; `artifacts/ge06-e5-f2-narrow-vs-expand-decision-2026-06-22.md` records that the correct branch is to narrow the pilot through GE-05 parity ownership rather than expand requirements or stop for architectural failure; `artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md`, `artifacts/ge06-e4-f1-execution-handoff-2026-06-22.md`, and `artifacts/ge06-e4-f1-merge-receipt-2026-06-26.md` now preserve the optional rules-core view-model spike as a completed merged slice rather than an active GE-06 coding lane; `artifacts/ge06-e4-f2-prebuild-readiness-closure-2026-06-22.md` plus `artifacts/ge06-e4-f2-prebuild-handoff-2026-06-22.md` now preserve the next downstream explanation/diagnostic inspection packet as explicit prebuild-only truth; `artifacts/ge06-e4-f3-prebuild-readiness-closure-2026-06-22.md` plus `artifacts/ge06-e4-f3-prebuild-handoff-2026-06-22.md` now preserve the export-summary boundary packet as explicit prebuild-only truth; and `artifacts/ge06-e5-f3-upstream-delta-review-2026-06-22.md` now records the resulting upstream delta/no-change verdict while leaving the pilot charter and GE-07 source STC unchanged. GE-06 still does **not** authorize implementation code by itself, settle the exact minimal product-visible UI implementation, invent final parity verdicts, or claim product-visible pilot success.

## Authority and Scope
- owner: `Todd Hintzmann`
- scope: `program`
- canonical: `true`
- canonical path: `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md`
- parent scopes:
  - `programs/codex`

This STC governs the Codex integrated pilot-proof planning surface. GE-01 owns grounded legacy-source discovery and token-family taxonomy. GE-02 owns canonical model homes. GE-03 owns import/provenance bridge truth. GE-04 owns computation and explainability truth. GE-05 owns parity-comparison truth. GE-07 owns the desktop shell and broader UI architecture once its own source STC exists. GE-06 owns only the integrated proof contract that binds those layers into one falsifiable pilot path.

## Target Runtime
- repo: `/home/ubuntu/workspace/repos/codex`
- workdir: `/home/ubuntu/workspace/repos/codex`
- branch/worktree: `future GE-06 implementation handoffs should branch from current develop unless an explicit dependency branch is named; origin/develop observed 2026-06-26 now contains merged GE06-E4-F1 via a11f7a4 / 1840cd9 at cc4e1a5, the route surface is retired to no-active-handoff, the E3 fan-in bundle remains documentary review input, and any later GE-06 coding work must start from a fresh successor handoff rather than a relaunch of the consumed E4-F1 or E3 packets`
- allowed write scope: `none during source STC generation; future GE-06 implementation handoffs must declare exact repo paths and whether they are headless-only or UI-facing`

The target repo is grounded only as the future implementation surface. This package is a requirements authority surface under `programs/codex/requirements/`, not a repo-local implementation brief.

## Document Map
- `technical-requirements.md` — normative requirements for the integrated pilot fixture, token-family boundary, canonical-object boundary, end-to-end proof path, explanation/diagnostic visibility, oracle-comparison boundary, UI truth contract, and viability decision
- `technical-design.md` — architecture/design response describing the integrated proof path, subsystem ownership split, payload boundaries, and narrow-first execution posture separately from normative requirements
- `acceptance-and-verification.md` — observable checks proving the GE-06 source STC and required output artifacts define a falsifiable vertical-slice contract without counterfeit implementation claims
- `risks-and-open-questions.md` — records that fixture-selection debt is closed and preserves remaining runtime-proof, parity-dimension, post-view-model UI-minimum, next-slice grounding, and fatal-flaw classification questions
- `epic-breakdown.md` — downstream implementation-facing epics and feature seeds for later bounded readiness closure and handoff derivation
- `research-handoff.md` — historical/superseded non-code discovery handoff for GE06-E1-F1 grounded character-selection closure
- `artifacts/ge06-e1-f1-handoff-readiness-closure-2026-06-20.md` — historical handoff-readiness closure that established GE06-E1-F1 as a non-code research handoff before final deterministic input closure
- `artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md` — accepted documentary closure that fixes the first pilot's Human ability target, feat slots, skill ranks, equipment, active states, and headless export-summary boundary
- `artifacts/ge06-e2-f1-execution-readiness-closure-2026-06-21.md` — readiness closure that authorized the first narrow code-producing GE-06 slice: deterministic pilot input contract fixture load gate
- `artifacts/ge06-e2-f1a-execution-handoff-2026-06-21.md` — archived copy of the completed F1a code-authorizing handoff
- `artifacts/ge06-e2-f1a-merge-receipt-2026-06-21.md` — verified merge receipt for F1a at Codex `develop` commit `9f3cb93`
- `artifacts/ge06-e2-f2a-execution-readiness-closure-2026-06-21.md` — readiness closure authorizing the next narrow code-producing GE-06 slice: base ability modifiers and Fighter class chassis computation
- `artifacts/ge06-e2-f2a-execution-handoff-2026-06-21.md` — stable stage-specific F2a code-authorizing handoff preserved after merge; no later slice may overwrite it
- `artifacts/ge06-e2-f2a-merge-receipt-2026-06-21.md` — verified merge receipt for F2a at Codex `origin/develop` commit `760c9b0`
- `artifacts/ge06-e2-f2b-execution-readiness-closure-2026-06-21.md` — readiness closure authorizing the next narrow code-producing GE-06 slice: baseline melee attack bonus and armor class under the deterministic loadout
- `artifacts/ge06-e2-f2b-execution-handoff-2026-06-21.md` — stable stage-specific F2b code-authorizing handoff preserved after merge; no later slice may overwrite it
- `artifacts/ge06-e2-f2b-merge-receipt-2026-06-21.md` — verified merge receipt for F2b at Codex `origin/develop` commit `75c26ce`
- `artifacts/ge06-e2-f2c-execution-readiness-closure-2026-06-21.md` — codex-ready readiness closure for total Fortitude/Reflex/Will saves under deterministic ability scores, with mandatory `pilot_compute.rs` prose synchronization folded into the same narrow slice
- `artifacts/ge06-e2-f2c-execution-handoff-2026-06-21.md` — stable stage-specific F2c code-authorizing handoff preserved after merge; no later slice may overwrite it
- `artifacts/ge06-e2-f2c-merge-receipt-2026-06-21.md` — verified merge receipt for F2c at Codex `origin/develop` commit `1b44c07`
- `artifacts/ge06-e2-f2d-execution-readiness-closure-2026-06-21.md` — codex-ready readiness closure that grounded the F2d selected-skill handoff
- `artifacts/ge06-e2-f2d-execution-handoff-2026-06-21.md` — stable stage-specific F2d code-authorizing handoff; it preserves the no-merge boundary and must not be mutated into later slices
- `artifacts/ge06-e2-f2d-merge-receipt-2026-06-21.md` — verified merge receipt for F2d at Codex `origin/develop` commit `2deb11b`
- `artifacts/ge06-e2-f3-execution-readiness-closure-2026-06-21.md` — historical readiness closure that grounded the first integrated headless receipt-path packet over the already-merged deterministic pilot outputs
- `artifacts/ge06-e2-f3-execution-handoff-2026-06-21.md` — stable stage-specific E2-F3 code-authorizing handoff preserved after merge; no later slice may overwrite it
- `artifacts/ge06-e2-f3-merge-receipt-2026-06-21.md` — verified merge receipt for F3 at Codex `origin/develop` commit `6977c86`
- `artifacts/ge06-post-f2d-handoff-rack-2026-06-21.md` — historical queue/rack surface that prepared E2-F3 and prebuilt the first downstream E3 drafts before the integrated receipt path merged
- `artifacts/ge06-post-f3-handoff-rack-2026-06-21.md` — historical queue/rack surface after the merged E2-F3 receipt path and the 2026-06-22 E3 promotion; it captured the temporary `awaiting-todd-launch` posture for GE06-E3-F1 and GE06-E3-F2 before both lanes merged
- `artifacts/ge06-post-e3-fan-in-handoff-rack-2026-06-22.md` — historical queue/rack surface after both merged E3 upstream lanes; it retired the consumed live handoffs, classified GE06-E3-F3 as `ready-to-derive`, and preserved downstream E4/E5 posture before the E3 bundle itself existed
- `artifacts/ge06-e3-f3-viability-evidence-bundle-2026-06-22.md` — explicit E3 fan-in evidence bundle that records the nine mandatory selected pilot dimensions at a `Computed` claim-tier floor, names `OracleGap` as the current supported-path blocker, and prepares the downstream viability / domain-confidence review input surface
- `artifacts/ge06-post-e3-f3-evidence-rack-2026-06-22.md` — historical queue/rack surface after the E3 evidence bundle existed but before the downstream viability / domain-confidence decision was written
- `artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md` — documentary decision artifact that records the current GE-06 posture as `computed-but-not-oracle-checked`, refuses counterfeit `pilot-viable` language, and points the next mandatory proof burden toward the oracle gap
- `artifacts/ge06-post-e5-f1-decision-rack-2026-06-22.md` — historical queue/rack surface after the E5-F1 decision existed but before the branch decision was made; it preserved GE06-E4-F1 in bounded pre-viability spike posture and rotated GE06-E5-F2 to ready
- `artifacts/ge06-e5-f2-narrow-vs-expand-decision-2026-06-22.md` — documentary branch decision artifact that consumes GE06-E5-F1, chooses narrow-the-pilot, routes the next mandatory proof burden to GE-05 parity ownership, and refuses unjustified upstream expansion or architectural-stop language
- `artifacts/ge06-post-e5-f2-decision-rack-2026-06-22.md` — historical queue/rack surface after the branch decision existed but before the upstream delta/no-change review was written; it records the then-live GE06-E4-F1 `awaiting-todd-launch` posture, GE06-E5-F2 as complete, and GE06-E5-F3 as the then-next documentary move
- `artifacts/ge06-e5-f3-upstream-delta-review-2026-06-22.md` — documentary review artifact that propagates the narrow-through-GE-05 decision into higher-order GE-05 / GE-06 / GE-09 posture surfaces, explicitly declares no immediate charter or GE-07 expansion, and preserves the then-current GE06-E4-F1 bounded launch posture as historical context
- `artifacts/ge06-e4-f1-launch-posture-2026-06-22.md` — historical pre-readiness posture that cleared the stale “spec domain only” blocker before the live E4-F1 pair existed
- `artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md` — readiness closure that grounded the bounded rules-core view-model contract from real outputs before the lane was launched
- `artifacts/ge06-e4-f1-execution-handoff-2026-06-22.md` — historical stage-specific E4-F1 code-authorizing handoff that was later consumed and merged; it remains bounded to the rules-core view-model lane and must not be widened into shell work
- `artifacts/ge06-e4-f1-merge-receipt-2026-06-26.md` — verified merge receipt for GE06-E4-F1, preserving PR #19 / `a11f7a4` as the authoritative completion surface for the rules-core view-model lane
- `artifacts/ge06-e4-f2-prebuild-readiness-closure-2026-06-22.md` — non-authorizing downstream readiness closure that prebuilds the explanation/diagnostic inspection packet while keeping it blocked on the future merged E4-F1 contract
- `artifacts/ge06-e4-f2-prebuild-handoff-2026-06-22.md` — prebuilt draft for the future E4-F2 inspection lane; it preserves the bounded downstream objective and candidate shell-facing write scope without minting code authority yet
- `artifacts/ge06-e4-f3-prebuild-readiness-closure-2026-06-22.md` — non-authorizing downstream readiness closure that prebuilds the export-summary boundary packet while keeping it blocked on the future merged E4-F1 contract
- `artifacts/ge06-e4-f3-prebuild-handoff-2026-06-22.md` — prebuilt draft for the future E4-F3 summary lane; it preserves the bounded downstream objective and candidate rules-core write scope without minting code authority yet
- `artifacts/ge06-e3-f1-prebuild-readiness-closure-2026-06-21.md` — historical non-authorizing prebuild closure for the selected parity-dimension adapter packet; superseded as the live launch gate by the 2026-06-22 readiness closure/handoff pair
- `artifacts/ge06-e3-f1-prebuild-handoff-2026-06-21.md` — historical non-authorizing prebuild handoff draft that captured the bounded oracle-validation write scope before live promotion
- `artifacts/ge06-e3-f2-prebuild-readiness-closure-2026-06-21.md` — historical non-authorizing prebuild closure for the failure-classifier packet; superseded as the live launch gate by the 2026-06-22 readiness closure/handoff pair
- `artifacts/ge06-e3-f2-prebuild-handoff-2026-06-21.md` — historical non-authorizing prebuild handoff draft that captured the bounded rules-core write scope before live promotion
- `artifacts/ge06-e3-f1-execution-readiness-closure-2026-06-22.md` — historical readiness closure that re-read merged E2-F3 truth and grounded the selected parity-dimension adapter handoff before merge
- `artifacts/ge06-e3-f1-execution-handoff-2026-06-22.md` — stable stage-specific E3-F1 code-authorizing handoff preserved after merge; no later slice may overwrite it
- `artifacts/ge06-e3-f1-merge-receipt-2026-06-22.md` — verified merge receipt for E3-F1 at Codex `origin/develop` merge commit `b2f2154`
- `artifacts/ge06-e3-f2-execution-readiness-closure-2026-06-22.md` — historical readiness closure that re-read merged E2-F3 truth and grounded the failure-classifier handoff before merge
- `artifacts/ge06-e3-f2-execution-handoff-2026-06-22.md` — stable stage-specific E3-F2 code-authorizing handoff preserved after merge; no later slice may overwrite it
- `artifacts/ge06-e3-f2-merge-receipt-2026-06-22.md` — verified merge receipt for E3-F2 at Codex `origin/develop` merge commit `7bc89e8`
- `execution-handoff.md` — canonical GE-06 execution route surface; it now truthfully sits at `no-active-handoff`, preserves merged GE06-E4-F1 through `artifacts/ge06-e4-f1-merge-receipt-2026-06-26.md`, preserves the E3 evidence bundle plus the full E5 documentary chain as review truth, and points at historical or future stage-specific handoffs without becoming code authority itself
- `references/upstream-dependency-contract.md` — compact contract mapping GE-01 through GE-05, the pilot charter, GE-07, and doctrine surfaces into GE-06 obligations
- `artifacts/pilot-charter-alignment.md` — concrete charter-alignment and scope-delta rules for GE-06
- `artifacts/pilot-character-fixture-requirements.md` — concrete integrated pilot-character fixture requirements
- `artifacts/required-token-family-list-requirements.md` — concrete token-family gate list for the pilot slice
- `artifacts/required-canonical-object-list-requirements.md` — concrete canonical-object gate list for the pilot slice
- `artifacts/pilot-stack-viability-decision-criteria.md` — concrete pass/fail decision criteria for declaring the stack viable or blocked for the pilot

## Expected Output Artifacts
| Artifact | Completion rule |
|---|---|
| `artifacts/pilot-charter-alignment.md` | Maps GE-06 obligations back to the pilot charter, records current no-change versus required-update posture, and names ADR triggers for scope expansion. |
| `artifacts/pilot-character-fixture-requirements.md` | Defines the Human Fighter level 1 fixture boundary, output categories, explanation duties, and parity boundaries without inventing final expected values. |
| `artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md` | Closes the first pilot's deterministic input selections: Human ability target, feat slots, skill ranks, equipment, active states, and headless export-summary boundary. |
| `artifacts/required-token-family-list-requirements.md` | Enumerates the grounded GE-01 token families the integrated slice depends on and classifies them into hard gates versus supporting scope. |
| `artifacts/required-canonical-object-list-requirements.md` | Enumerates the minimum GE-02 canonical model homes, support records, and runtime-boundary obligations required for the pilot slice. |
| `acceptance-and-verification.md` | Carries the end-to-end acceptance criteria proving that import, compute, explanation, selected oracle comparison, diagnostic visibility, failure categorization, and minimal UI truth are all bounded and falsifiable. |
| `artifacts/pilot-stack-viability-decision-criteria.md` | Defines pass/fail stack-viability criteria, fatal-flaw triggers, narrowing triggers, and evidence thresholds aligned to Codex quality-gate tiers. |

## Required Reads
- `../../plans/spec-domains/GE-06-pilot-vertical-slice-pf1-human-fighter.md` — primary strategic authority for this integrated pilot source STC
- `../../plans/roadmaps/codex-spec-domain-roadmap-2026-06-18.md` — Stage D roadmap posture and exit-gate authority for the product slice
- `../../plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md` — canonical pilot boundary, initial acceptance target, and non-expansion rule
- `../GE-01-legacy-corpus-and-conversion-matrix/README.md` — accepted legacy-source control plane and collection closure state
- `../GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-token-taxonomy.csv` — grounded pilot token families that the integrated slice depends on
- `../GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv` — grounded object-class and token-family conversion posture for the pilot
- `../GE-02-canonical-rules-model-and-content-packages/README.md` — accepted canonical-model source STC and generated documentary artifact index
- `../GE-02-canonical-rules-model-and-content-packages/artifacts/canonical-model-specification.md` — canonical model homes and pilot minimum object set inherited by GE-06
- `../GE-03-pcgen-import-pipeline-and-provenance/README.md` — importer bridge posture and planning boundary for pilot import truth
- `../GE-03-pcgen-import-pipeline-and-provenance/technical-requirements.md` — parser, provenance, conversion-report, and unsupported-token obligations inherited by the integrated slice
- `../GE-04-rules-engine-and-explainability-core/README.md` — computation/explainability planning boundary for pilot behavior
- `../GE-04-rules-engine-and-explainability-core/technical-requirements.md` — character-input, computation, explanation, diagnostics, and headless-entry obligations inherited by the integrated slice
- `../GE-04-rules-engine-and-explainability-core/artifacts/pilot-golden-computation-fixture-requirements.md` — GE-04 fixture contract that GE-06 must finalize rather than contradict
- `../GE-05-oracle-validation-and-parity-harness/README.md` — oracle/parity planning boundary for the pilot
- `../GE-05-oracle-validation-and-parity-harness/technical-requirements.md` — parity-report, normalization, known-gap, and comparison obligations inherited by the integrated slice
- `../GE-05-oracle-validation-and-parity-harness/references/upstream-dependency-contract.md` — already-grounded GE-05 view of what upstream surfaces do and do not authorize
- `../../plans/spec-domains/GE-07-desktop-shell-and-modern-ux.md` — spec-domain boundary for the UI consumer side of the pilot
- `../GE-07-desktop-shell-and-modern-ux/README.md` — accepted GE-07 source STC defining the minimal shell, command-boundary, and non-authorizing UI route posture that any later GE-06 UI slice must respect
- `../../doctrine/quality-gate-policy.md` — claim-tier and evidence-gate doctrine for import, compute, parity, and UI truth

## Conditional Reads
- `../GE-03-pcgen-import-pipeline-and-provenance/execution-handoff.md` — only when a later session proposes an implementation slice that depends on actual importer outputs
- `../GE-04-rules-engine-and-explainability-core/execution-handoff.md` — only when a later session proposes a rules-core implementation slice that GE-06 intends to consume
- `../GE-05-oracle-validation-and-parity-harness/research-handoff.md` — only when grounding the current oracle-route discovery state for later integrated comparison work
- `/home/ubuntu/workspace/repos/codex/AGENTS.md` — only if a later session proposes repo-facing implementation work
- legacy PCGen files or runtime surfaces under `/home/ubuntu/workspace/repos/pcgen` — only when grounding unresolved fixture choices or oracle-output boundaries; this source STC does not assert final old-system command paths

## In Scope
- Codex GE-06 source-STC documents under `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/`
- concrete GE-06 same-epic documentary outputs under `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/`
- the integrated proof contract that binds GE-01 through GE-05 into one PF1 Human Fighter level 1 pilot path
- pilot character fixture requirements, including the accepted final deterministic input contract for the first bounded pilot case
- the required token-family list and required canonical-object list for this slice
- end-to-end acceptance criteria, failure taxonomy, and stack-viability decision criteria
- the minimal UI truth contract as a consumer boundary over real domain outputs
- charter-update and ADR trigger rules when the pilot boundary changes

## Out of Scope
- writing importer, engine, parity-harness, UI, or integration code in `/home/ubuntu/workspace/repos/codex`
- modifying `/home/ubuntu/workspace/repos/pcgen`
- broad Pathfinder support, full PCGen parity, or broad export-sheet compatibility
- final Tauri/React shell architecture, cross-platform packaging work, or design-system scope owned by GE-07
- inventing final expected values for parity or adding fixture choices outside the accepted deterministic input contract without a new charter/ADR review
- treating the entire integrated pilot as a single broad code-authorizing handoff
- claiming viability, oracle parity, or product-visible truth from this planning bundle alone

## Acceptance Summary
The acceptance criteria in `acceptance-and-verification.md` are satisfied for the planning-ready GE-06 boundary when this bundle and its named output artifacts exist and remain internally linked.

Compact summary:
- the pilot vertical slice is defined as an integrated proof contract, not as a broad implementation prompt
- the required token families and canonical object homes for the slice are explicit and inherited from grounded upstream artifacts
- the integrated pilot character fixture is bounded by real charter facts and the final deterministic input contract, while runtime expected values remain evidence-gated
- headless import/compute/proof obligations are defined before UI truth claims are allowed
- selected oracle comparison, diagnostics, explanations, and failure categorization are explicit enough to prevent counterfeit completion
- later implementation can be decomposed into bounded slices without inventing branch/worktree, write scope, commands, or final parity values here

## Allowed Assumptions
- the PF1 Core Rulebook Human Fighter level 1 pilot remains the first proof target unless a higher-order decision surface changes it
- the pilot charter's initial ability-score vector and named `power_attack` feat remain the grounded starting point, and the final deterministic input contract is the accepted closure for the first pilot's additional feat/choice, skill, equipment, and active-state inputs
- GE-01 and GE-02 accepted artifact sets are authoritative planning inputs for token-family and canonical-object boundaries
- GE-03, GE-04, and GE-05 continue to own importer, computation/explainability, and parity obligations respectively; GE-06 must consume rather than redefine them
- GE-07 owns broad UI architecture, while GE-06 may only define the minimum UI truth contract needed to prove the slice

## Blockers / Forbidden Assumptions
- stop if a future handoff treats GE-06 as permission for a broad cross-epic implementation sprint without an exact slice, repo paths, branch/worktree, write scope, and verification commands
- do not fabricate final expected values, broaden fixture selections beyond the accepted deterministic input contract, or invent final old-system command routes
- do not allow mock UI state, screenshots, or hardcoded examples to satisfy the GE-06 UI truth gate
- do not treat the GE-07 spec domain as a substitute for a GE-07 source STC when broad UI implementation decisions arise
- do not suppress failures by collapsing them into vague “integration issue” language; every failure must be routed as model flaw, importer flaw, engine flaw, oracle gap, or UI gap
- do not expand the pilot scope without updating the charter and, when the change is architectural or scope-bearing, recording an ADR

## Next Stage Rule
- GE-06 is planning-ready as a source requirements construct because its control bundle and required same-epic documentary outputs now exist.
- GE06-E2-F1a, GE06-E2-F2a, GE06-E2-F2b, GE06-E2-F2c, GE06-E2-F2d, and GE06-E2-F3 are complete, merged, and preserved as stage-specific historical handoff artifacts plus merge receipts.
- `artifacts/ge06-e2-f3-merge-receipt-2026-06-21.md` records the merged F3 state at `origin/develop` commit `6977c86`.
- `execution-handoff.md` is a route surface only. It does **not** authorize code by itself; it now sits at `no-active-handoff`, preserves the E3 bundle plus the full E5 documentary chain as review truth, and preserves merged GE06-E4-F1 through its readiness closure, historical handoff, and merge receipt.
- `artifacts/ge06-post-e5-f2-decision-rack-2026-06-22.md` remains the historical queue surface immediately before the propagation review; it records GE06-E5-F2 as complete documentary branch decision, classifies GE06-E4-F1 as `awaiting-todd-launch`, and classifies GE06-E5-F3 as the then-next documentary governance move.
- `artifacts/ge06-e4-f1-launch-posture-2026-06-22.md` remains the historical pre-readiness posture that cleared the stale “GE-07 is still only a spec domain” blocker before the live E4-F1 pair existed.
- `artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md`, `artifacts/ge06-e4-f1-execution-handoff-2026-06-22.md`, and `artifacts/ge06-e4-f1-merge-receipt-2026-06-26.md` preserve the completed E4 coding lane as bounded historical authority for the rules-core view-model contract from real outputs.
- `artifacts/ge06-e5-f2-narrow-vs-expand-decision-2026-06-22.md` records that the correct branch is to narrow the pilot through GE-05 parity ownership rather than expand requirements or stop for architectural failure.
- `artifacts/ge06-e5-f3-upstream-delta-review-2026-06-22.md` records that the resulting propagation required targeted GE-05 / GE-06 / GE-09 posture updates, while the pilot charter and GE-07 source STC truthfully stayed unchanged.
- `artifacts/ge06-e2-f3-execution-readiness-closure-2026-06-21.md` and `artifacts/ge06-e2-f3-execution-handoff-2026-06-21.md` remain preserved as the historical readiness/handoff pair that grounded the merged first integrated headless receipt-path packet.
- `artifacts/ge06-e3-f1-prebuild-readiness-closure-2026-06-21.md`, `artifacts/ge06-e3-f1-prebuild-handoff-2026-06-21.md`, `artifacts/ge06-e3-f2-prebuild-readiness-closure-2026-06-21.md`, and `artifacts/ge06-e3-f2-prebuild-handoff-2026-06-21.md` remain preserved as historical prebuild drafts.
- `artifacts/ge06-e3-f1-execution-readiness-closure-2026-06-22.md`, `artifacts/ge06-e3-f1-execution-handoff-2026-06-22.md`, `artifacts/ge06-e3-f2-execution-readiness-closure-2026-06-22.md`, and `artifacts/ge06-e3-f2-execution-handoff-2026-06-22.md` are now preserved as the historical merged E3 upstream pair, and `artifacts/ge06-e3-f1-merge-receipt-2026-06-22.md` plus `artifacts/ge06-e3-f2-merge-receipt-2026-06-22.md` record their verified merge truth.
- Any later GE-06 code-authorizing handoff must remain narrower than this spec domain, live in its own stage-specific artifact, and name exact repo paths, required reads, upstream evidence surfaces, branch/worktree policy, allowed write scope, verification commands, expected receipts, and non-goals before implementation begins.
- Any product-visible UI handoff should either cite a GE-07 source STC or explicitly declare itself a bounded non-production spike with evidence outputs and no counterfeit product claim.
- GE06-E4-F1 is now preserved as merged historical authority through its readiness closure, historical handoff, and merge receipt; it remains bounded to the rules-core view-model lane and must not be mistaken for a product-visible shell implementation.
- The next mandatory proof burden no longer sits in a vague GE-06 continuation lane; it now points at GE-05 parity ownership, beginning from the GE-05 execution route surface and its grounded next candidate `GE05-E2-F2 — PF1 Human Fighter level 1 governed fixture instance`.
