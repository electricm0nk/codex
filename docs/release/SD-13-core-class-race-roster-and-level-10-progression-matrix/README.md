---
stc_id: STC-CODEX-SD-13
stc_kind: source-requirements
template_version: 2
work_type: planning-only
workflow_route: planning
readiness: planning-ready
status: active
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch: current live repo state observed 2026-06-30 is branch `sd11-f10-update-action-surface` at commit `a0a90dcdf75cd0cc09d4b71e9fb7d3b440aaf293`; `origin/develop` observed the same day is commit `c2cea5c6baeb3ca34077b85331214c4b42a4809c`; durable accepted breadth truth is still the GE-06 PF1 Core Rulebook Human Fighter level-1 proof, and live tests explicitly claim-block Rogue level 1 and Fighter level 2 postures rather than proving broader class or level support
  write_scope: documentary-only updates inside this source STC bundle plus control-plane sync in `programs/codex/requirements/README.md`; no repo implementation-code authority
review_state: draft
last_reviewed_at: 2026-06-30
parent_scopes:
  - programs/codex
source_artifacts:
  - programs/codex/plans/spec-domains/SD-13-core-class-race-roster-and-level-10-progression-matrix.md
  - programs/codex/plans/spec-domains/README.md
  - programs/codex/plans/roadmaps/codex-spec-domain-roadmap-2026-06-18.md
  - programs/codex/requirements/README.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md
  - programs/codex/requirements/GE-10-demo-proof-and-onboarding/README.md
  - programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md
  - programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/tester-workbench-surface-specification.md
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/distribution-platform-support-and-channel-matrix.md
  - programs/codex/doctrine/program-doctrine-and-scope-charter.md
  - programs/codex/doctrine/documentation-control-plane.md
  - programs/codex/doctrine/quality-gate-policy.md
  - repos/codex/README.md
  - repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt
  - repos/codex/tests/ge06_pilot_total_saves.rs
  - repos/codex/tests/ge06_pilot_combat_baseline.rs
related_artifacts:
  - programs/codex/README.md
  - programs/codex/plans/roadmaps/codex-execution-status-ledger-2026-06-21.md
upstream_targets:
  - programs/codex/requirements/README.md
  - programs/codex/plans/roadmaps/codex-execution-status-ledger-2026-06-21.md
expected_output_artifacts:
  - path: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md
    completion_rule: Names the exact PF1 Core Rulebook core-race and core-class roster, defines the support-state taxonomy, seeds the current truthful matrix, and records what evidence is still missing before bounded breadth claims are honest.
  - path: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/level-10-progression-validation-contract.md
    completion_rule: Defines the mandatory progression dimensions and class-specific proof burdens that later execution slices must satisfy before any class can claim level-10 support.
  - path: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md
    completion_rule: Defines the visible debt ledger for unsupported, partial, lossy, blocked, and unverified roster semantics, including minimum row fields and seeded current-state entries.
  - path: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/adjacent-lane-boundary-and-breadth-claim-contract.md
    completion_rule: Defines the explicit seam between SD-13 breadth truth and the adjacent SD-11 tester-surface, SD-12 distribution/update, and SD-14 persistence/lifecycle lanes so later work cannot counterfeit scope by implication.
  - path: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/tester-facing-support-language-contract.md
    completion_rule: Defines the only approved tester-facing wording for SD-13 support states so downstream workbench surfaces can render roster truth without inventing softer or stronger labels.
  - path: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/coverage-evidence-and-fixture-plan.md
    completion_rule: Defines the minimum fixture families and evidence-class posture required before support-state rows may be promoted or demoted honestly.
supersedes: []
superseded_by: []
tags:
  - codex
  - sd-13
  - bounded-breadth
  - pf1
  - core-rulebook
  - class-roster
  - race-roster
  - progression
  - level-10
---

# SD-13 — Core class/race roster and level-10 progression matrix

## Objective
Define the first truthful breadth-expansion authority surface for Codex: Pathfinder 1e Core Rulebook core races plus core classes, with explicit support-state classification and level-10 progression proof obligations that grow beyond the Human Fighter level-1 pilot without pretending to general Pathfinder support.

## Deliverable Type
`planning-only`

## Workflow Route
`planning`

## Readiness
`planning-ready`

Why this readiness is accurate:
- the SD-13 strategic spec domain already exists and explicitly frames this lane as a bounded breadth contract rather than a code-authorizing prompt
- the live repo gives enough hard grounding to describe the current ceiling honestly: the deterministic fixture is still `race:human` plus `class:fighter:1`, `tests/ge06_pilot_total_saves.rs` explicitly claim-blocks `class:rogue:1`, and both `tests/ge06_pilot_total_saves.rs` and `tests/ge06_pilot_combat_baseline.rs` explicitly claim-block `class:fighter:2`
- the repo README still states plainly that Codex is not yet a general character builder or broad Pathfinder product, which means this packet can define the next truthful breadth step without inventing current runtime maturity
- SD-11 already fixes the tester-workbench boundary, SD-12 already fixes the distribution/update boundary, and SD-14 already fixes the persistence/lifecycle boundary, so this packet can keep breadth truth separate from those adjacent lanes
- this bundle includes both the control documents and the same-epic documentary artifacts required to keep roster breadth, progression obligations, support-state taxonomy, and visible unsupported-depth debt concrete instead of recursive

## Closure State
SD-13 is generated as a planning-ready source STC on 2026-06-30. It defines the bounded PF1 Core Rulebook core-roster breadth lane: seven core races, eleven core classes, explicit support-state taxonomy, class-family and class-specific level-10 validation burdens, visible unsupported/partial/lossy/unverified debt, and adjacent-lane scope boundaries. It does not authorize repo implementation code, broaden the ruleset beyond the PF1 Core Rulebook, collapse spellcasting classes into Fighter-like proof standards, admit multiclassing, or smuggle tester/distribution/persistence scope into a breadth packet. Current durable repo truth remains the GE-06 Human Fighter level-1 proof surface plus negative evidence that Rogue level 1 and Fighter level 2 are not presently supported by the bounded compute path.

## Authority and Scope
- owner: `Todd Hintzmann`
- scope: `program`
- canonical: `true`
- canonical path: `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md`
- parent scopes:
  - `programs/codex`

This STC governs bounded breadth truth: what roster is inside the first expansion tranche, how support state is classified, what level-10 progression obligations exist, what unsupported or lossy semantics must remain visible, and how later execution slices may claim success without counterfeiting breadth. GE-06 owns the existing Human Fighter proof. SD-11 owns the tester-facing workbench and support wording. SD-12 owns build/update delivery and platform/channel posture. SD-14 owns saved-state continuity. SD-13 owns only the breadth/progression truth surface between them.

## Target Runtime
- repo: `/home/ubuntu/workspace/repos/codex`
- workdir: `/home/ubuntu/workspace/repos/codex`
- branch/worktree: `current live repo state observed 2026-06-30 is branch sd11-f10-update-action-surface at commit a0a90dcdf75cd0cc09d4b71e9fb7d3b440aaf293; origin/develop observed the same day is c2cea5c6baeb3ca34077b85331214c4b42a4809c; accepted breadth truth is still bounded to the GE-06 deterministic Human Fighter level-1 pilot, with no accepted multi-class or level-10 breadth subsystem yet`
- allowed write scope: `none during source STC generation beyond this packet and control-plane sync docs; future SD-13 execution handoffs must declare exact repo paths, exact write scope, exact required reads, and exact verification commands before repo files may change`

This bundle is an authority surface under `programs/codex/requirements/`, not a repo-local implementation brief.

## Document Map
- `technical-requirements.md` — normative requirements for exact roster identity, support-state taxonomy, level-10 progression obligations, visibility rules for unsupported/partial/lossy/unverified semantics, and adjacent-lane boundaries
- `technical-design.md` — architecture/design response describing how breadth truth should be represented, how evidence tiers should combine with support states, and how later execution can avoid fake combinatorial coverage theater
- `acceptance-and-verification.md` — observable checks proving the SD-13 packet and same-epic documentary artifacts remain concrete and honest
- `risks-and-open-questions.md` — unresolved spell breadth, interaction-row, prerequisite-pressure, tester-language, and future audit questions isolated from the main contract
- `epic-breakdown.md` — downstream epic and feature decomposition for same-domain execution-story minting
- `references/upstream-dependency-contract.md` — compact contract naming what GE-06, GE-10, SD-11, SD-12, and the live repo do and do not authorize for SD-13
- `artifacts/core-roster-and-support-state-matrix.md` — exact roster list, support-state taxonomy, seeded current-truth matrix, and breadth-claim gating contract
- `artifacts/level-10-progression-validation-contract.md` — concrete level-10 proof obligations, class-family splits, and class-specific semantic burden table
- `artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md` — visible debt ledger contract and seeded current-state rows
- `artifacts/adjacent-lane-boundary-and-breadth-claim-contract.md` — explicit seam map between SD-13 and the adjacent tester/distribution/persistence lanes
- `artifacts/tester-facing-support-language-contract.md` — the only approved tester-facing language for SD-13 support states so downstream surfaces do not invent counterfeit confidence
- `artifacts/coverage-evidence-and-fixture-plan.md` — the minimum fixture families and evidence-class posture required before any support-state change is honest

## Expected Output Artifacts
| Artifact | Completion rule |
|---|---|
| `artifacts/core-roster-and-support-state-matrix.md` | Names the exact PF1 Core Rulebook core-race and core-class roster, defines the support-state taxonomy, seeds the current truthful matrix, and records what evidence is still missing before bounded breadth claims are honest. |
| `artifacts/level-10-progression-validation-contract.md` | Defines the mandatory progression dimensions and class-specific proof burdens that later execution slices must satisfy before any class can claim level-10 support. |
| `artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md` | Defines the visible debt ledger for unsupported, partial, lossy, blocked, and unverified roster semantics, including minimum row fields and seeded current-state entries. |
| `artifacts/adjacent-lane-boundary-and-breadth-claim-contract.md` | Defines the explicit seam between SD-13 breadth truth and the adjacent SD-11 tester-surface, SD-12 distribution/update, and SD-14 persistence/lifecycle lanes so later work cannot counterfeit scope by implication. |
| `artifacts/tester-facing-support-language-contract.md` | Defines the only approved tester-facing wording for SD-13 support states so downstream workbench surfaces can render roster truth without inventing softer or stronger labels. |
| `artifacts/coverage-evidence-and-fixture-plan.md` | Defines the minimum fixture families and evidence-class posture required before support-state rows may be promoted or demoted honestly. |

## Required Reads
- `../../plans/spec-domains/SD-13-core-class-race-roster-and-level-10-progression-matrix.md` — primary strategic authority for this source STC
- `../../plans/roadmaps/codex-spec-domain-roadmap-2026-06-18.md` — roadmap ordering and stage boundary for the breadth lane
- `../GE-06-pilot-vertical-slice-pf1-human-fighter/README.md` — current accepted character-path proof ceiling and anti-counterfeit-breadth truth
- `../GE-10-demo-proof-and-onboarding/README.md` — current developer-proof current-state posture that must not be inflated into product breadth claims
- `../SD-11-test-user-workbench-and-github-feedback-intake/README.md` — adjacent tester-surface authority this packet must support without replacing
- `../SD-11-test-user-workbench-and-github-feedback-intake/artifacts/tester-workbench-surface-specification.md` — tester-workbench visibility obligations this packet must eventually feed without becoming UI authority
- `../SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md` — adjacent distribution/update authority this packet must not counterfeit
- `../SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/distribution-platform-support-and-channel-matrix.md` — platform/channel posture that later breadth claims must not silently override
- `../../doctrine/program-doctrine-and-scope-charter.md` — local-first scope, anti-sprawl, and vertical-slice-before-breadth doctrine
- `../../doctrine/documentation-control-plane.md` — control-plane and authority-surface doctrine
- `../../doctrine/quality-gate-policy.md` — evidence tiers and anti-counterfeit-completion doctrine
- `/home/ubuntu/workspace/repos/codex/README.md` — live repo current-state truth declaring the project is not yet a general character builder
- `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt` — the current bounded roster/progression proof input showing the only accepted race/class path
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs` — live negative evidence that Rogue level 1 and Fighter level 2 are not presently supported by the bounded compute path
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs` — live negative evidence that Fighter level 2 still blocks combat/defense claim surfaces

## Conditional Reads
- `/home/ubuntu/workspace/repos/codex/AGENTS.md` — only when a later SD-13 execution handoff is being prepared for repo-facing work
- repo-local rules-core files under `/home/ubuntu/workspace/repos/codex/src/rules_core/` — only when a later execution slice names the exact compute seam it will change
- later SD-14 persistence artifacts — only when a future lane must prove how expanded breadth changes saved-state compatibility
- Pathfinder source/reference extracts or PCGen comparison surfaces — only when a later execution slice grounds exact class/race semantic evidence or oracle comparison for a named roster path
- GitHub issue or telemetry surfaces — only when a future audit/feedback slice wires breadth claims into tester issue routing or evidence refresh

## In Scope
- Codex SD-13 source-STC documents under `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/`
- same-epic documentary outputs under `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/`
- the exact PF1 Core Rulebook core-race roster and core-class roster for this tranche
- support-state taxonomy and evidence posture for bounded breadth claims
- level-10 single-class progression obligations for the named core classes
- visible unsupported/partial/lossy/unverified semantics and debt recording
- the seam between roster breadth truth and adjacent tester/distribution/persistence lanes
- downstream epic decomposition for later same-domain execution-story minting

## Out of Scope
- writing implementation code in `/home/ubuntu/workspace/repos/codex`
- broad Pathfinder expansion beyond the PF1 Core Rulebook boundary
- multiclassing, prestige classes, archetypes, alternate racial traits, or non-core books
- claiming support beyond level 10 in this tranche
- replacing SD-11 tester-workbench authority, SD-12 release/update authority, or SD-14 persistence authority
- treating parsed content, selectable UI options, or package names as proof of supported roster depth
- collapsing spellcasting classes into the same proof burden as Fighter-like chassis

## Acceptance Summary
The acceptance criteria in `acceptance-and-verification.md` are satisfied for the SD-13 planning-ready boundary when this bundle and its named same-epic documentary artifacts exist, remain internally coherent, and define bounded breadth truth concretely enough that later execution-story minting cannot counterfeit support by merely showing more options.

Compact summary:
- the exact PF1 Core Rulebook core-race and core-class roster is named explicitly
- support-state taxonomy and evidence posture keep partial, lossy, blocked, and unverified semantics visible
- level-10 support is defined as a proof burden over progression dimensions rather than a UI-selection affordance
- spellcasting and non-spellcasting classes remain visibly different in proof burden
- adjacent SD-11, SD-12, and SD-14 boundaries stay explicit

## Allowed Assumptions
- PF1 Core Rulebook remains the sole bounded breadth surface for this tranche
- the breadth lane remains single-class only through level 10 unless a later authority surface explicitly expands that rule
- current accepted repo truth is still the deterministic Human Fighter level-1 proof path and negative evidence for broader chassis/level support, not a broad roster implementation
- SD-11 remains the adjacent authority for tester-facing workbench wording and issue surfaces, and SD-12 remains the adjacent authority for update/distribution posture

## Blockers / Forbidden Assumptions
- stop if a later handoff treats this source STC as repo-write authority without exact repo paths, exact write scope, and exact verification commands
- do not assume a parsed class, a dropdown entry, or a screenshot means the class is supported
- do not assume Human Fighter level 1 proof implies another race, another class, or any Fighter level above 1
- do not count spellcasting classes as supported unless spell lists, slots/known/prepared posture, class features, and related progression semantics are all classified honestly
- do not smuggle multiclassing into this lane because a single-class matrix feels incomplete
- do not hide unsupported, lossy, partial, or unverified semantics in chat memory, UI optimism, or private operator knowledge

## Next Stage Rule
- SD-13 is planning-ready because both the source-STC control bundle and its required same-epic documentary output artifacts now exist.
- SD-13 has no `execution-handoff.md`; this source STC does not authorize code by itself.
- The next truthful move is the already-required workflow card `SD-13 FLOW: Mint bounded execution stories from the SD-13 epic breakdown`, then stage-specific handoff artifacts only for the slices Todd explicitly releases.
- The first execution slice should begin from taxonomy/matrix truth and current-state seeding, not from a fake “implement all classes and races” breadth sprint.
