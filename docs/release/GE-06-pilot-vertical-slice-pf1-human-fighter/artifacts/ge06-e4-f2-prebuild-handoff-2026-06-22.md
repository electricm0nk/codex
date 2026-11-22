---
title: GE06-E4-F2 Prebuild Handoff Draft — Explanation and Diagnostic Inspection Surface
handoff_id: HANDOFF-CODEX-GE-06-E4-F2-PREBUILD-2026-06-22
stc_id: STC-CODEX-GE-06
handoff_kind: execution-handoff-draft
work_type: implementation-ready
workflow_route: coding
readiness: blocked
status: prebuilt-draft
owner: Todd Hintzmann
scope: program
canonical: false
canonical_path: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/execution-handoff.md
source_stc: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md
readiness_closure: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f2-prebuild-readiness-closure-2026-06-22.md
selected_slice: GE06-E4-F2 — Explanation and diagnostic inspection surface
run_in: Claude Code or equivalent frontier coding harness, but only after post-E4-F1 promotion
code_authority: false
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  recommended_branch: ge06-e4-f2-inspection-surface
future_live_artifacts:
  - artifacts/ge06-e4-f2-execution-readiness-closure-YYYY-MM-DD.md
  - artifacts/ge06-e4-f2-execution-handoff-YYYY-MM-DD.md
  - artifacts/ge06-e4-f2-merge-receipt-YYYY-MM-DD.md
allowed_write_scope:
  - apps/desktop/src/main.tsx
  - apps/desktop/src/App.tsx
  - apps/desktop/src-tauri/src/main.rs
forbidden_write_scope:
  - /home/ubuntu/workspace/repos/pcgen
  - src/rules_core/**
  - src/oracle_validation/**
  - src/pcgen_import/**
  - tests/**
  - Cargo.toml
  - Cargo.lock
  - AGENTS.md
  - CLAUDE.md
  - programs/codex/**
---

# GE06-E4-F2 Prebuild Handoff Draft — Explanation and Diagnostic Inspection Surface

## Status
This is a prebuilt draft only.

Do not hand this to Claude Code yet. It carries `code_authority: false` until a later documentary pass promotes it after GE06-E4-F1 merge evidence exists and the live repo still supports the same narrow implementation lane.

## Objective
Once GE06-E4-F1 is merged, create the smallest additive inspection surface that lets a user inspect why a surfaced value exists, why a choice is unavailable, and which diagnostics still apply, all over real GE-06 pilot data.

The future surface should preserve:
- the real pilot snapshot / blocked posture from GE06-E4-F1
- one explanation-affordance path for a derived value
- one inspectable invalid or unavailable choice reason path
- visible diagnostics and warning/problem state
- the existing evidence ceiling and known-gap language

## Required reads before any future promotion
1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
3. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f2-prebuild-readiness-closure-2026-06-22.md`
4. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-merge-receipt-YYYY-MM-DD.md`
5. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md`
6. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-handoff-2026-06-22.md`
7. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_view_model.rs`
8. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
9. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_failure.rs`
10. `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/technical-requirements.md`
11. `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/technical-design.md`
12. `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ui-command-boundary-requirements.md`
13. `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/component-surface-inventory.md`
14. `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/pilot-ux-flow-requirements.md`
15. `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e1-shell-scaffold-receipt-2026-06-22.md`
16. `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e1-runtime-boundary-adr-input-2026-06-22.md`
17. `programs/codex/doctrine/quality-gate-policy.md`

## Post-merge promotion gate
A future documentary run may mint the live GE06-E4-F2 execution handoff only if all are true:

1. GE06-E4-F1 is merged and receipted.
2. The merged repo still exposes a stable view-model / snapshot surface materially compatible with the E4-F1 handoff.
3. The additive `apps/desktop/` subtree either already exists from an earlier scaffold lane or can be grounded in the same bounded lane without widening beyond inspection proof.
4. The candidate write scope below still remains the smallest truthful implementation for explanation/diagnostic inspection.
5. No new upstream doctrine changed the explanation/diagnostic visibility obligations or the evidence ceiling.

If any item fails, throw this draft away and derive a fresh stage-specific handoff.

## Candidate write scope after promotion
```text
apps/desktop/src/main.tsx
apps/desktop/src/App.tsx
apps/desktop/src-tauri/src/main.rs
```

The goal of that future code slice should be to stay inside the additive desktop shell lane and off the rules-core implementation surface.

## Draft implementation behavior
When promoted, the live handoff should require the coding harness to:
1. render the real GE06-E4-F1 pilot snapshot or explicit blocked posture through an explicit UI/core boundary
2. expose one inspectable derived-value explanation path using upstream explanation references rather than frontend-owned math
3. expose one inspectable invalid or unavailable choice reason path when that condition exists in the real upstream payload
4. keep diagnostics visible in the same bounded interaction flow rather than burying them as implementation noise
5. avoid rules-library breadth, export breadth, parity promotion, or shell-polish detours

## Verification posture to ground later
The future live handoff should not invent a UI test stack that the repo has not grounded yet.

Instead, the later promotion pass should require exact receipts proving at minimum:
- the additive shell boots or produces a bounded startup receipt
- the shell requests real GE06-E4-F1-derived data through an explicit boundary
- explanation inspection is backed by upstream payloads
- invalid/unavailable-choice inspection is backed by upstream payloads or blocked-state truth
- diagnostics remain visible when present
- no hardcoded pilot numbers are used as proof of behavior

## Non-goals
The future live handoff must not authorize:
- edits to `src/rules_core/**`, `src/oracle_validation/**`, or `src/pcgen_import/**`
- rules-library or source-package browsing work
- export summary work
- parity comparator or claim-tier promotion work
- frontend-owned semantic logic
- diagnostic suppression for cleanliness
- broad GE-07 shell architecture work beyond this bounded inspection proof

## Why this draft exists
GE06-E4-F2 is a real downstream packet, but today it depends on a merged GE06-E4-F1 contract that does not exist yet.

This draft preserves the bounded objective, future artifact identities, and candidate implementation lane without counterfeiting activation. The real code-authorizing moment remains after GE06-E4-F1 merge evidence, where it belongs.
