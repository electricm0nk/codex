---
title: GE-07 Execution Route Surface
artifact_type: execution-route-surface
stc_id: STC-CODEX-GE-07
source_stc: ./README.md
workflow_route: coding
readiness: codex-ready
status: awaiting-todd-merge
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/execution-handoff.md
code_authority: false
active_handoff:
  - artifacts/ge07-e1-execution-handoff-2026-06-22.md
active_readiness_closure:
  - artifacts/ge07-e1-execution-readiness-closure-2026-06-22.md
last_completed_handoff: []
next_candidate_slice: GE07-E1 — Desktop shell scaffold and runtime boundary spike is the active branch-ready coding packet awaiting Todd merge
next_handoff_required: false
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  observed_origin_develop: 7bc89e8c1edf8f1d1a6d490a0ad28ac72fc6f104
---

# GE-07 Execution Route Surface

## Current state
There is now **one active GE-07 stage-specific code-authorizing handoff with verified branch-ready implementation evidence awaiting Todd review/merge**.

The active live pair is:

```text
GE07-E1 — Desktop shell scaffold and runtime boundary spike
programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e1-execution-readiness-closure-2026-06-22.md
programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e1-execution-handoff-2026-06-22.md
```

Verified repo truth as of 2026-06-24:

```text
branch: ge07-e1-desktop-shell-scaffold
head: 48892249d5573927bf23a7e47a6d7d6a742da664
origin/develop: 7bc89e8c1edf8f1d1a6d490a0ad28ac72fc6f104
head merged into origin/develop: no
remote feature branch present: yes (`origin/ge07-e1-desktop-shell-scaffold`)
open GitHub PR discovered: no
```

This root file remains a **stable route surface**, not the code-authorizing brief itself. The stage-specific handoff above carries `code_authority: true`.

## Why this route surface exists
The GE-07 board process previously completed E1-E6 documentary cards while every downstream lane still pointed at the same missing fact: no desktop scaffold exists on `origin/develop`.

This route surface repairs that operator-legibility failure by making the next executable artifact explicit.

## Active coding packet
```text
GE07-E1 — Desktop shell scaffold and runtime boundary spike
```

Current posture:
- source STC exists and is planning-ready
- E1 documentary receipt and ADR input exist
- E1 execution-readiness closure now exists and is `codex-ready`
- E1 stage-specific execution handoff has been exercised once and now truthfully sits at `awaiting-todd-merge`
- verified repo truth shows `apps/desktop/**` exists only on `ge07-e1-desktop-shell-scaffold` at `48892249d5573927bf23a7e47a6d7d6a742da664`, not yet on `origin/develop`
- write scope is confined to `apps/desktop/**`
- no open GitHub PR was detected from this branch; the next operator move is Todd review/merge or explicit rework

## Not active yet
The following GE-07 lanes remain non-authorizing until their prerequisites become real repo truth:

- `GE07-E2` — waits for the scaffold and, likely, the GE06-E4-F1 consumer bridge
- `GE07-E3` — waits for the scaffold plus a merged UI-consumable pilot snapshot bridge
- `GE07-E4` — waits for scaffold, projection, and invalid-choice/diagnostic ownership boundaries
- `GE07-E5` — waits for scaffold, workspace bridge, and inspection projection
- `GE07-E6` — waits for at least one real shell slice before packaging proof can mean anything

## Human gates
The automation boundary remains explicit:
- Todd already launched the coding harness manually for this slice.
- Todd performs merges manually.
- This route surface should move to `no-active-handoff` only when repo evidence proves merge, or back through a new active handoff state only if Todd requests rework or a later bounded slice is grounded.

At the moment, the truthful state is `awaiting-todd-merge`.

## Rule for future stages
Do not retarget this file into a stage brief.

Any later GE-07 code-authorizing work must be created as a new stage-specific handoff under `artifacts/`, paired with its own readiness closure and later merge receipt.
