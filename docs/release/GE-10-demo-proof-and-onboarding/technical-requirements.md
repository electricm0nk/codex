---
title: GE-10 Technical Requirements
stc_id: STC-CODEX-GE-10
artifact_type: technical-requirements
status: draft
scope: programs/codex/requirements/GE-10-demo-proof-and-onboarding
source_stc: ./README.md
---

# GE-10 Technical Requirements

## Problem statement

Codex has real proof surfaces, but until this pass it lacked one authoritative onboarding path that told a new developer how to prepare a vanilla machine, build the desktop shell first, verify the bounded headless and desktop proof surfaces, and understand the current state honestly.

## Normative requirements

### GE10-TR-01 — Repo-root truth surface
The repo-root `repos/codex/README.md` must become the first truthful onboarding surface for the implementation repo.

It must state:
- what Codex is
- what the current project state is
- which proof surfaces are verified live
- how to install prerequisites for the currently verified platform
- how to build and verify the desktop shell
- how to run the current bounded demo
- what the known limitations are

### GE10-TR-02 — Front-loaded prerequisite closure
The first canonical demo step must install prerequisites and complete the first desktop-shell build before any later step asks the operator to continue.

The first step must include:
- Linux desktop prerequisite packages for the currently verified platform
- Rust installation or activation
- Node.js/npm availability verification
- `npm ci` in `repos/codex/apps/desktop`
- `npm run typecheck`
- `npm run build`
- `npm run tauri:check`
- `npx tauri build --debug`

Later demo steps must not require the operator to stop for new prerequisite installation if GE10-TR-02 was followed.

### GE10-TR-03 — Verified platform posture
GE-10 must name the currently verified platform explicitly rather than implying universal platform proof.

Current verified posture for this pass:
- Linux desktop
- Ubuntu 24.04-derived environment
- Rust toolchain available through `rustup` / `cargo`
- Node.js and npm available
- Tauri Linux development libraries available

The docs may name broader target platforms only as planned or unverified unless live proof exists.

### GE10-TR-04 — Headless proof-surface coverage
The canonical demo path must include the current headless proof surfaces.

At minimum it must include:
- root `cargo test`
- focused GE-06 proof command
- focused GE-08 proof command
- explanation of what those surfaces do and do not prove

### GE10-TR-05 — Desktop boundary coverage
The canonical demo path must include the current desktop boundary surfaces.

At minimum it must include:
- `npm run typecheck`
- `npm run build`
- `npm run tauri:check`
- `npx tauri build --debug`
- the GUI-launch step on a graphical Linux session
- explicit warning that a headless shell can build but not launch the GTK GUI

### GE10-TR-06 — Current-state narrative
GE-10 must create and maintain a current-state summary that distinguishes:
- verified working proof surfaces
- bounded or partial capability
- unverified or unfinished product scope
- known platform/GUI limitations

The summary must not claim:
- general character-builder readiness
- broad parity completeness
- production release readiness

### GE10-TR-07 — Governed card grammar
GE-10 must define per-step demo cards whose titles and bodies are suitable for the user’s governed board conventions.

Each card title must use the route-aware form:

```text
GE10-EX FLOW: Title
```

Each card body must contain exactly these sections in order:
- `Description`
- `DoD`
- `Prereqs`
- `Outputs`
- `Inputs`
- `Next step`

Each section must be explicit enough that a later operator can run the step without reopening the entire STC.

### GE10-TR-08 — Onboarding checklist
GE-10 must create a new-developer onboarding checklist that tells incoming developers:
- what to read first
- which commands to run first
- which repo and program boundaries matter
- which false assumptions to avoid
- how to tell a proof harness from finished product scope in this repo

### GE10-TR-09 — Live-verification precedence rule
When a narrative document conflicts with live repo verification, the onboarding surfaces must prefer the live repo truth and update the narrative rather than repeating stale text.

### GE10-TR-10 — No counterfeit maturity
No GE-10 artifact may describe Codex as a finished end-user application. The required posture is:

```text
developer proof harness + buildable desktop workbench surface
```

If that posture changes later, a new evidence pass must update GE-10 deliberately.

## Concrete output obligations

GE-10 must produce or update exactly these artifacts:

- `programs/codex/requirements/GE-10-demo-proof-and-onboarding/artifacts/vanilla-machine-demo-runbook.md`
- `programs/codex/requirements/GE-10-demo-proof-and-onboarding/artifacts/demo-step-cards.md`
- `programs/codex/requirements/GE-10-demo-proof-and-onboarding/artifacts/current-project-state-summary.md`
- `programs/codex/requirements/GE-10-demo-proof-and-onboarding/artifacts/developer-onboarding-checklist.md`
- `repos/codex/README.md`

## Non-goals

- introducing new product features
- replacing execution handoffs
- defining release operations
- claiming platform proof beyond what has been verified
