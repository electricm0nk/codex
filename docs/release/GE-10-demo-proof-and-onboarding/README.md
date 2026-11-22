---
stc_id: STC-CODEX-GE-10
stc_kind: source-requirements
template_version: 2
work_type: planning-only
workflow_route: planning
readiness: planning-ready
status: active
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/GE-10-demo-proof-and-onboarding/README.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch: current local repo state observed 2026-06-28 is branch `develop` aligned with `origin/develop`; repo HEAD observed during live verification is commit `43314de`
  write_scope: documentary-only updates inside this source STC bundle and `repos/codex/README.md`; no implementation-code authority
review_state: draft
last_reviewed_at: 2026-06-28
parent_scopes:
  - programs/codex
source_artifacts:
  - programs/codex/plans/spec-domains/GE-10-demo-proof-and-onboarding.md
  - programs/codex/plans/spec-domains/README.md
  - programs/codex/plans/roadmaps/codex-spec-domain-roadmap-2026-06-18.md
  - programs/codex/plans/roadmaps/codex-execution-status-ledger-2026-06-21.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md
  - programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/README.md
  - programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/README.md
  - repos/codex/README.md
  - repos/codex/AGENTS.md
  - repos/codex/apps/desktop/package.json
  - repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt
  - repos/codex/tests/fixtures/ge08/guard-stance-package/manifest.yaml
related_artifacts:
  - programs/codex/README.md
  - programs/codex/requirements/README.md
  - programs/codex/plans/spec-domains/README.md
upstream_targets:
  - repos/codex/README.md
  - programs/codex/requirements/README.md
  - programs/codex/plans/spec-domains/README.md
expected_output_artifacts:
  - path: programs/codex/requirements/GE-10-demo-proof-and-onboarding/artifacts/vanilla-machine-demo-runbook.md
    completion_rule: Gives a truthful, step-ordered demo path that starts with prerequisite installation and the first desktop-shell build on the currently verified platform.
  - path: programs/codex/requirements/GE-10-demo-proof-and-onboarding/artifacts/demo-step-cards.md
    completion_rule: Defines one governed card body per demo step using explicit Description, DoD, Prereqs, Outputs, Inputs, and Next step sections.
  - path: programs/codex/requirements/GE-10-demo-proof-and-onboarding/artifacts/current-project-state-summary.md
    completion_rule: States what is live and verified today, what remains bounded or unfinished, and which proof surfaces back each claim.
  - path: programs/codex/requirements/GE-10-demo-proof-and-onboarding/artifacts/developer-onboarding-checklist.md
    completion_rule: Gives incoming developers the minimum reads, commands, and behavioral rules required to onboard without scope confusion.
  - path: repos/codex/README.md
    completion_rule: Exposes current state, verified getting-started instructions, demo entrypoints, and known limitations at the repo root.
supersedes: []
superseded_by: []
tags:
  - codex
  - ge-10
  - demo
  - onboarding
  - vanilla-machine
  - proof-surfaces
  - readme
---

# GE-10 — Demo Proof and Developer Onboarding

## Objective

Define the canonical developer-facing path that turns the current Codex proof surfaces into a repeatable vanilla-machine demo and onboarding experience without overstating product maturity.

## Deliverable Type

`planning-only`

## Workflow Route

`planning`

## Readiness

`planning-ready`

Why this readiness is accurate:
- the strategic GE-10 spec domain exists and clearly limits this lane to documentary/onboarding authority rather than new runtime capability
- the live repo state has already been verified enough to ground truthful getting-started, build, test, and demo instructions
- this bundle includes both the control documents and the same-epic output artifacts needed to make the onboarding lane concrete rather than recursive
- the repo-root README has been updated in the same pass so the primary onboarding surface is no longer missing

## Closure State

GE-10 is generated as a planning-ready source STC in a documentary-only posture as of 2026-06-28. It codifies the current answer to “what is real now and how do I run it on a clean machine?” while explicitly refusing to convert bounded proof surfaces into broad end-user product claims.

## Authority and Scope

- owner: `Todd Hintzmann`
- scope: `program`
- canonical: `true`
- canonical path: `programs/codex/requirements/GE-10-demo-proof-and-onboarding/README.md`
- parent scopes:
  - `programs/codex`

This STC governs the documentary legibility layer around Codex: the repo-root getting-started surface, the current-state summary, the canonical demo runbook, the governed per-step card set, and the new-developer onboarding checklist. It does not authorize implementation code, cross-platform release claims, or new product capability.

## Target Runtime

- repo: `/home/ubuntu/workspace/repos/codex`
- workdir: `/home/ubuntu/workspace/repos/codex`
- branch/worktree: `current local repo state observed 2026-06-28 is branch develop aligned with origin/develop; repo HEAD observed during live verification is commit 43314de`
- allowed write scope: `documentary-only updates inside this source STC bundle and repos/codex/README.md; no implementation code authority`

This bundle is an authority surface under `programs/codex/requirements/`, not a repo-local coding brief.

## Document Map

- `technical-requirements.md` — normative requirements for vanilla-machine prerequisites, front-loaded build closure, README contract, current-state truth, demo flow, step cards, and onboarding guidance
- `technical-design.md` — design response describing how README, runbook, current-state summary, onboarding checklist, and card catalog fit together without duplication drift
- `acceptance-and-verification.md` — observable checks proving the GE-10 documentary outputs align with the live repo verification surfaces
- `risks-and-open-questions.md` — preserves unresolved questions about cross-platform proof, GUI assumptions, drift control, and future automation
- `epic-breakdown.md` — downstream decomposition for demo-step flow cards, README upkeep, and possible future automation
- `artifacts/vanilla-machine-demo-runbook.md` — canonical walkthrough for preparing the current verified platform, building the desktop shell, and then progressing through the current proof surfaces
- `artifacts/demo-step-cards.md` — governed per-step card set for running the demo in bounded stages
- `artifacts/current-project-state-summary.md` — explicit current-state assessment distinguishing verified capability from unfinished product work
- `artifacts/developer-onboarding-checklist.md` — newcomer intake checklist for reading, commands, and behavioral rules

## Expected Output Artifacts

| Artifact | Completion rule |
|---|---|
| `artifacts/vanilla-machine-demo-runbook.md` | Gives a truthful, step-ordered demo path that starts with prerequisite installation and the first desktop-shell build on the currently verified platform. |
| `artifacts/demo-step-cards.md` | Defines one governed card body per demo step using explicit Description, DoD, Prereqs, Outputs, Inputs, and Next step sections. |
| `artifacts/current-project-state-summary.md` | States what is live and verified today, what remains bounded or unfinished, and which proof surfaces back each claim. |
| `artifacts/developer-onboarding-checklist.md` | Gives incoming developers the minimum reads, commands, and behavioral rules required to onboard without scope confusion. |
| `repos/codex/README.md` | Exposes current state, verified getting-started instructions, demo entrypoints, and known limitations at the repo root. |

## Required Reads

- `../../plans/spec-domains/GE-10-demo-proof-and-onboarding.md` — primary strategic authority for this source STC
- `../../plans/roadmaps/codex-spec-domain-roadmap-2026-06-18.md` — roadmap ordering and scope boundary for the new demo/onboarding lane
- `../../plans/roadmaps/codex-execution-status-ledger-2026-06-21.md` — operator-facing route-state context that GE-10 must not contradict silently
- `../GE-06-pilot-vertical-slice-pf1-human-fighter/README.md` — current pilot proof boundary and evidence ceiling
- `../GE-07-desktop-shell-and-modern-ux/README.md` — desktop-shell boundary GE-10 must present honestly rather than inflate
- `../GE-08-homebrew-authoring-and-rules-studio/README.md` — bounded GE-08 workbench proof surface that the current demo path exposes
- `/home/ubuntu/workspace/repos/codex/README.md` — repo-root surface that new developers see first and that GE-10 must correct
- `/home/ubuntu/workspace/repos/codex/AGENTS.md` — repo-root coding behavior boundary for developers who advance beyond documentary onboarding
- `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json` — authoritative JS/Tauri scripts used in the getting-started and demo paths

## Conditional Reads

- Tauri prerequisites documentation at `https://v2.tauri.app/start/prerequisites/` — mandatory when the verified Linux prerequisite surface changes or a new platform is being documented
- `/home/ubuntu/workspace/repos/codex/Cargo.toml` — only when the core verification commands or crate surfaces change materially
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json` — only when desktop packaging or runtime assumptions are being updated
- future GE-09 release/packaging artifacts — only if the onboarding lane later expands into installer or release instructions

## In Scope

- documentary artifacts under `programs/codex/requirements/GE-10-demo-proof-and-onboarding/`
- repo-root onboarding/current-state updates in `repos/codex/README.md`
- the currently verified Linux desktop prerequisite posture
- front-loaded prerequisite and first-build closure for the current demo path
- headless proof-surface walkthrough for GE-06 and GE-08
- desktop build and GUI-launch walkthrough for the bounded Tauri workbench surface
- new-developer onboarding guidance and explicit current-state narrative

## Out of Scope

- modifying implementation code under `repos/codex/src/` or `repos/codex/apps/desktop/src/`
- modifying `/home/ubuntu/workspace/repos/pcgen`
- broad cross-platform support claims without live proof
- release engineering, installers, auto-updaters, or public packaging operations
- pretending the current desktop shell is already a general character builder or full PCGen replacement
- replacing source STCs with README prose

## Acceptance Summary

The acceptance criteria in `acceptance-and-verification.md` are satisfied for the GE-10 planning-ready source-STC boundary when this bundle, its named same-epic documentary outputs, and the updated repo README exist and remain aligned with live verification commands.

Compact summary:
- the first demo step installs prerequisites and builds the desktop shell up front
- the README now exposes a truthful current-state and getting-started surface
- the demo path distinguishes headless proof, desktop build proof, and GUI-launch proof
- the onboarding lane states plainly that Codex is currently a developer proof harness plus bounded desktop workbench surface, not a finished end-user product

## Allowed Assumptions

- Ubuntu 24.04/Linux desktop is the only platform verified in this pass for the full prerequisite/build/demo path
- `@tauri-apps/cli` remains a local dev dependency in `apps/desktop/package.json`, so `npx tauri ...` is the preferred repo-local invocation path
- headless shells can build the desktop binary but cannot launch the GTK GUI successfully
- live repo verification on 2026-06-28 remains authoritative over older narrative ledgers when contradictions appear

## Blockers / Forbidden Assumptions

- stop if any future GE-10 update claims public-release or end-user-product readiness without new evidence
- do not claim Windows or macOS onboarding is verified until live proof exists on those platforms
- do not let the README silently drift away from the demo runbook or the current-state summary
- do not route GE-10 into implementation code merely because documentation reveals a missing convenience script
- do not hide the GUI-session requirement for launching the desktop shell

## Next Stage Rule

- GE-10 is planning-ready because both the source-STC control bundle and its same-epic documentary output artifacts now exist.
- GE-10 has no `execution-handoff.md`; this source STC does not authorize code by itself.
- The next truthful move is upkeep: keep the README, runbook, and current-state summary aligned with live repo verification, or explicitly mint a later automation lane if documentary maintenance proves insufficient.
