---
stc_id: STC-CODEX-SD-11
stc_kind: source-requirements
template_version: 2
work_type: planning-only
workflow_route: planning
readiness: planning-ready
status: active
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch: current local repo state observed 2026-06-28 is branch `develop` aligned with `origin/develop`; repo HEAD observed during live verification is commit `aba751e8432743bd201161269c6cd1d146592cce`
  write_scope: documentary-only updates inside this source STC bundle plus control-plane sync in `programs/codex/requirements/README.md` and the SD-11 strategic surface; no repo implementation-code authority
review_state: draft
last_reviewed_at: 2026-06-28
parent_scopes:
  - programs/codex
source_artifacts:
  - programs/codex/plans/spec-domains/SD-11-test-user-workbench-and-github-feedback-intake.md
  - programs/codex/plans/spec-domains/README.md
  - programs/codex/plans/roadmaps/codex-spec-domain-roadmap-2026-06-18.md
  - programs/codex/plans/roadmaps/codex-execution-status-ledger-2026-06-21.md
  - programs/codex/requirements/README.md
  - programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/README.md
  - programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/README.md
  - programs/codex/requirements/GE-10-demo-proof-and-onboarding/README.md
  - programs/codex/doctrine/program-doctrine-and-scope-charter.md
  - programs/codex/doctrine/documentation-control-plane.md
  - programs/codex/doctrine/quality-gate-policy.md
  - repos/codex/README.md
  - repos/codex/apps/desktop/package.json
  - repos/codex/apps/desktop/src/App.tsx
  - repos/codex/apps/desktop/src/boundary/loadPilotShellSnapshot.ts
  - repos/codex/apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts
  - repos/codex/apps/desktop/src-tauri/src/main.rs
related_artifacts:
  - programs/codex/README.md
  - programs/codex/plans/roadmaps/codex-execution-status-ledger-2026-06-21.md
upstream_targets:
  - programs/codex/requirements/README.md
  - programs/codex/plans/roadmaps/codex-execution-status-ledger-2026-06-21.md
expected_output_artifacts:
  - path: programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/tester-workbench-surface-specification.md
    completion_rule: Defines the required tester-facing workbench surfaces, backing data sources, visibility obligations, and explicit non-goals for the first bounded tester workbench.
  - path: programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/github-bug-report-intake-contract.md
    completion_rule: Defines the GitHub bug-report issue contract, required evidence payload, failure handling, and what may be auto-captured versus user-supplied.
  - path: programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/github-enhancement-request-intake-contract.md
    completion_rule: Defines the GitHub enhancement-request issue contract, required goal/friction/capability fields, and evidence expectations.
  - path: programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/tester-feedback-evidence-capture-matrix.md
    completion_rule: Records the exact evidence matrix for tester submissions, including automatic capture, user-supplied fields, attachment handling, and redaction rules.
  - path: programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md
    completion_rule: Defines the live operator truth `develop -> main`, reserves any future `beta` candidate label until it has a governed backing surface, and preserves the Linux-first support-tier rules the updater surface must present honestly.
supersedes: []
superseded_by: []
tags:
  - codex
  - sd-11
  - tester-workbench
  - github-intake
  - diagnostics
  - explainability
  - update-ux
---

# SD-11 — Test-User Workbench, GitHub Feedback Intake, and Bounded Upgrade UX

## Objective
Define the first tester-facing Codex workbench as a bounded desktop surface over real domain truth, with visible diagnostics and explanations, governed GitHub feedback intake, and honest upgrade semantics that do not expose repository topology as product UX.

## Deliverable Type
`planning-only`

## Workflow Route
`planning`

## Readiness
`planning-ready`

Why this readiness is accurate:
- the SD-11 strategic spec domain already exists and clearly states that this lane is a boundary object rather than a code-authorizing implementation brief
- the live repo already contains a real desktop workbench surface under `repos/codex/apps/desktop/`, which grounds the current UI/runtime seam instead of forcing this STC to speculate abstractly
- the current desktop surface is honestly bounded: `App.tsx` exposes a GE-08 authoring workbench over real headless data, while `loadPilotShellSnapshot.ts` and `src-tauri/src/main.rs` preserve the still-placeholder GE-07 pilot-character seam; that gives SD-11 enough current-state evidence to define truthful next requirements
- GE-07, GE-08, and GE-10 already ground adjacent shell, authoring-workbench, and onboarding truths this packet must inherit rather than rewrite
- this bundle includes both the control documents and the same-epic documentary artifacts needed to keep the tester-workbench problem concrete instead of recursive

## Closure State
SD-11 is generated as a planning-ready source STC on 2026-06-28. It defines the first tester-facing workbench as a bounded layer over real Codex domain outputs, names the structured GitHub feedback contracts, and records the operator-versus-user upgrade semantics that later implementation must preserve. It does **not** authorize implementation code, broad character-builder claims, public-release packaging, or direct repository/branch UX for testers. The live repo truth remains that `repos/codex/apps/desktop/` is currently a bounded GE-08 authoring workbench plus an unwired GE-07 pilot snapshot seam, not the completed SD-11 tester workbench.

## Authority and Scope
- owner: `Todd Hintzmann`
- scope: `program`
- canonical: `true`
- canonical path: `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md`
- parent scopes:
  - `programs/codex`

This STC governs the first tester-facing Codex workbench lane: the workbench surface boundary, diagnostics/explanation visibility, GitHub bug/enhancement intake contracts, evidence packaging rules, and the honest update/channel/support posture. GE-07 owns desktop-shell planning truth. GE-08 owns the current authoring-workbench proof surface. GE-10 owns onboarding/current-state legibility. SD-11 owns the tester-facing product boundary that must consume those upstream truths without inflating them into unsupported product claims.

## Target Runtime
- repo: `/home/ubuntu/workspace/repos/codex`
- workdir: `/home/ubuntu/workspace/repos/codex`
- branch/worktree: `current local repo state observed 2026-06-28 is branch develop aligned with origin/develop; repo HEAD observed during live verification is commit aba751e8432743bd201161269c6cd1d146592cce`
- allowed write scope: `none during source STC generation beyond this packet and control-plane sync docs; future SD-11 execution handoffs must declare exact repo paths, exact write scope, and exact verification commands before code may be changed`

This bundle is an authority surface under `programs/codex/requirements/`, not a repo-local implementation brief.

## Document Map
- `technical-requirements.md` — normative requirements for the tester workbench, diagnostics/explanation visibility, GitHub issue intake flows, evidence payloads, and update/channel semantics
- `technical-design.md` — architecture/design response describing how the tester workbench should sit over the current desktop shell and headless boundaries without becoming rules authority
- `acceptance-and-verification.md` — observable checks proving the SD-11 packet and same-epic documentary outputs remain concrete and reviewable
- `risks-and-open-questions.md` — unresolved GitHub-auth, offline queueing, bounded-character-flow, and updater-safety questions isolated from the main contract
- `epic-breakdown.md` — downstream epic/feature decomposition for same-domain execution-story minting
- `references/upstream-dependency-contract.md` — compact contract naming what GE-07, GE-08, GE-10, and the live repo surface do and do not authorize for SD-11
- `artifacts/tester-workbench-surface-specification.md` — concrete surface inventory for the first tester workbench
- `artifacts/github-bug-report-intake-contract.md` — concrete bug-report issue schema and failure-handling contract
- `artifacts/github-enhancement-request-intake-contract.md` — concrete enhancement-request issue schema and evidence contract
- `artifacts/tester-feedback-evidence-capture-matrix.md` — exact auto-capture, user-supplied, optional, and redacted evidence matrix
- `artifacts/update-channel-and-promotion-mapping.md` — explicit operator-branch, tester-channel, and platform-support mapping

## Expected Output Artifacts
| Artifact | Completion rule |
|---|---|
| `artifacts/tester-workbench-surface-specification.md` | Defines the required tester-facing workbench surfaces, backing data sources, visibility obligations, and explicit non-goals for the first bounded tester workbench. |
| `artifacts/github-bug-report-intake-contract.md` | Defines the GitHub bug-report issue contract, required evidence payload, failure handling, and what may be auto-captured versus user-supplied. |
| `artifacts/github-enhancement-request-intake-contract.md` | Defines the GitHub enhancement-request issue contract, required goal/friction/capability fields, and evidence expectations. |
| `artifacts/tester-feedback-evidence-capture-matrix.md` | Records the exact evidence matrix for tester submissions, including automatic capture, user-supplied fields, attachment handling, and redaction rules. |
| `artifacts/update-channel-and-promotion-mapping.md` | Defines the live operator truth `develop -> main`, reserves any future `beta` candidate label until it has a governed backing surface, and preserves the Linux-first support-tier rules the updater surface must present honestly. |

## Required Reads
- `../../plans/spec-domains/SD-11-test-user-workbench-and-github-feedback-intake.md` — primary strategic authority for this source STC
- `../../plans/roadmaps/codex-spec-domain-roadmap-2026-06-18.md` — roadmap ordering and tranche boundary for SD-11
- `../../plans/roadmaps/codex-execution-status-ledger-2026-06-21.md` — route-state context this packet must not contradict silently
- `../GE-07-desktop-shell-and-modern-ux/README.md` — desktop-shell and UI-boundary truth inherited by SD-11
- `../GE-08-homebrew-authoring-and-rules-studio/README.md` — current bounded desktop workbench proof surface and anti-counterfeit-success posture
- `../GE-10-demo-proof-and-onboarding/README.md` — current-state and Linux-first demo truth the tester lane must preserve honestly
- `/home/ubuntu/workspace/repos/codex/README.md` — live repo onboarding/current-state surface
- `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json` — current desktop toolchain and local command surface
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx` — current tester-visible desktop workbench shape
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadPilotShellSnapshot.ts` — current placeholder pilot-snapshot seam and anti-fake-truth rule
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts` — current real Tauri-command workbench boundary
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs` — current runtime command boundary and existing payload shape
- `../../doctrine/program-doctrine-and-scope-charter.md` — product-scope and headless-core-first doctrine
- `../../doctrine/documentation-control-plane.md` — control-plane and authority-surface doctrine
- `../../doctrine/quality-gate-policy.md` — evidence and truth-surface doctrine

## Conditional Reads
- `/home/ubuntu/workspace/repos/codex/AGENTS.md` — only when a later SD-11 execution handoff is being prepared for repo-facing work
- `../GE-07-desktop-shell-and-modern-ux/execution-handoff.md` — only when a later SD-11 slice depends on an accepted GE-07 code lane rather than planning truth alone
- `../GE-06-pilot-vertical-slice-pf1-human-fighter/README.md` — only when the bounded tester workflow is being tied directly to pilot character data rather than the current GE-08 workbench proof
- GitHub Issues REST/API documentation — only when a later handoff fixes the exact issue-submission transport instead of keeping it as a documentary contract
- Tauri updater/channel documentation — only when a later handoff fixes the exact update transport, manifest shape, or platform-specific installer path

## In Scope
- Codex SD-11 source-STC documents under `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/`
- same-epic documentary outputs under `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/`
- the first bounded tester-facing desktop workbench surface over real Codex data
- visibility requirements for diagnostics, explanation context, unsupported semantics, and support-tier messaging
- GitHub bug-report and enhancement-request intake contracts
- evidence-capture, attachment, and redaction requirements for tester submissions
- operator-to-tester mapping for update channels and platform support promises
- downstream epic decomposition for later same-domain story generation

## Out of Scope
- writing implementation code in `/home/ubuntu/workspace/repos/codex`
- modifying `/home/ubuntu/workspace/repos/pcgen`
- claiming Codex is already a general character builder or broad Pathfinder product
- public-release operations, app-store posture, or release-authority workflow beyond the first tester-update truth surface
- cloud sync, accounts, mobile surfaces, or non-GitHub feedback backends
- exposing raw branch names as the primary tester-facing UX
- authoring broad roster coverage or persistence semantics that belong to later SD-13/SD-14 lanes

## Acceptance Summary
The acceptance criteria in `acceptance-and-verification.md` are satisfied for the SD-11 planning-ready boundary when this bundle and its named same-epic documentary artifacts exist, remain internally coherent, and define the tester workbench as a surface over real Codex truth rather than a mock façade.

Compact summary:
- the first tester workbench is bounded, desktop-first, Linux-first, and visibly honest about unsupported scope
- bug reports and enhancement requests are defined as structured GitHub issue flows with explicit evidence contracts
- diagnostics, explanation context, build/channel metadata, and support-tier truth remain visible instead of being hidden for polish
- update semantics distinguish operator branch truth from tester-facing channel/support language

## Allowed Assumptions
- Linux remains the first-class verified tester platform for this tranche; macOS is second-class and Windows remains explicitly third-class
- the live operator promotion path is `develop -> main`; `beta` remains reserved until a governed candidate promotion surface exists, even if the tester-facing UI later prefers channel labels over branch names
- the current live desktop app is the GE-08 authoring workbench proof surface and may be treated as a grounding reference, but not as evidence that the SD-11 tester workbench already exists
- GitHub remains the authoritative destination for bug and enhancement intake in this tranche unless a higher-order doctrine surface changes that rule

## Blockers / Forbidden Assumptions
- stop if a later handoff treats this source STC as code-write authority without exact repo paths, exact write scope, and exact verification commands
- do not assume the current GE-08 authoring workbench equals the completed SD-11 tester workbench
- do not assume GitHub-auth posture, offline queueing behavior, or attachment transport without explicit later grounding
- do not expose raw branch names as the primary tester-facing update UX merely because the operator model is branch-based
- do not flatten Linux/macOS/Windows into fake platform symmetry

## Next Stage Rule
- SD-11 is planning-ready because both the source-STC control bundle and its same-epic documentary output artifacts now exist.
- SD-11 has no `execution-handoff.md`; this source STC does not authorize code by itself.
- The next truthful move is to mint bounded same-domain execution stories from `epic-breakdown.md`, then derive stage-specific handoff artifacts only for the slices Todd explicitly releases.
- Later tranche domains SD-12 through SD-15 remain blocked until Todd reviews the SD-11 workflow and decides the lane is smooth enough to advance.
