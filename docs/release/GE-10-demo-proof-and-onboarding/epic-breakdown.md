---
title: GE-10 Epic Breakdown
stc_id: STC-CODEX-GE-10
artifact_type: epic-breakdown
status: draft
scope: programs/codex/requirements/GE-10-demo-proof-and-onboarding
source_stc: ./README.md
---

# GE-10 Epic Breakdown

## Objective

Decompose the demo-proof and onboarding lane into bounded documentary slices without pretending that the source STC is a coding brief.

## Downstream epics / slices

### GE10-E1 — Vanilla machine prerequisites and desktop shell build
Purpose:
- install or verify the currently required prerequisite stack
- complete the first desktop-shell build before the operator enters later proof steps

Primary output:
- first successful local desktop-shell build on the verified platform

Likely route first:
- FLOW card / documentary walkthrough

### GE10-E2 — Headless proof-surface verification
Purpose:
- verify the root Rust proof harness
- demonstrate the bounded GE-06 and GE-08 proof surfaces

Primary output:
- successful headless verification receipts and operator understanding of what they do and do not prove

### GE10-E3 — Desktop boundary verification and artifact build
Purpose:
- verify the frontend and Tauri compile/build surfaces
- keep the built desktop artifact explicit before the GUI-launch step

Primary output:
- successful TypeScript/Tauri verification plus built debug desktop binary

### GE10-E4 — Interactive desktop workbench walkthrough
Purpose:
- launch the current bounded desktop shell on a graphical session
- inspect the GE-08 Guard Stance workbench surface honestly

Primary output:
- operator-visible walkthrough of the bounded workbench surface and its limitations

### GE10-E5 — Current-state and onboarding upkeep
Purpose:
- keep README, runbook, current-state summary, and onboarding checklist aligned with live proof surfaces

Primary output:
- refreshed documentary surfaces after material repo change

### GE10-E6 — Optional future automation lane
Purpose:
- add scripts, smoke checks, or bootstrap helpers only if documentary maintenance proves insufficient

Constraint:
- must be authorized separately; no code work by implication from GE-10 alone

## Routing rule

No item above is code-authorizing by itself.

A future code-producing handoff must:
- identify the exact automation objective
- state exact repo paths and write scope
- preserve the current no-counterfeit-maturity posture
- cite the live verification commands it is meant to simplify
