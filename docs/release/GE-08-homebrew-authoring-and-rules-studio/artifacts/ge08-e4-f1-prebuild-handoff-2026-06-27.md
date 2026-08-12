---
title: GE08-E4-F1 Prebuild Handoff Draft — Headless Preview and Explanation Bridge for First Proof Package
handoff_id: HANDOFF-CODEX-GE-08-E4-F1-PREBUILD-2026-06-27
stc_id: STC-CODEX-GE-08
handoff_kind: execution-handoff-draft
work_type: implementation-ready
workflow_route: coding
readiness: blocked
status: prebuilt-draft
owner: Todd Hintzmann
scope: program
canonical: false
canonical_path: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/execution-handoff.md
source_stc: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/README.md
readiness_closure: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e4-f1-prebuild-readiness-closure-2026-06-27.md
selected_slice: GE08-E4-F1 — Headless preview and explanation bridge for first proof package
run_in: Claude Code or equivalent frontier coding harness, but only after GE08-E3-F1 evidence and documentary promotion
code_authority: false
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  observed_origin_develop: cc4e1a55caad07af83768a036d4b0f5fffbf99c9
  recommended_branch: ge08-e4-f1-preview-and-explanation-bridge
future_live_artifacts:
  - artifacts/ge08-e4-f1-execution-readiness-closure-YYYY-MM-DD.md
  - artifacts/ge08-e4-f1-execution-handoff-YYYY-MM-DD.md
  - artifacts/ge08-e4-f1-claude-launch-receipt-YYYY-MM-DD.md
allowed_write_scope:
  - src/lib.rs
  - src/homebrew_authoring/mod.rs
  - src/homebrew_authoring/preview_bridge.rs
  - tests/ge08_preview_bridge.rs
  - tests/fixtures/ge08/guard-stance-package/**
forbidden_write_scope:
  - /home/ubuntu/workspace/repos/pcgen
  - src/rules_core/**
  - src/oracle_validation/**
  - src/pcgen_import/**
  - programs/codex/**
  - Cargo.toml
  - Cargo.lock
  - AGENTS.md
  - CLAUDE.md
---

# GE08-E4-F1 Prebuild Handoff Draft — Headless Preview and Explanation Bridge for First Proof Package

## Status
This is a prebuilt draft only.

Do not hand this to Claude Code yet. It carries `code_authority: false` until a later documentary pass promotes it after GE08-E3-F1 evidence and a fresh `origin/develop` audit.

## Objective
Once GE08-E3-F1 is proven, create the smallest headless bridge that consumes the validated deterministic authored package bundle plus the fixed GE08-E1 proof binding and emits a bounded preview/explanation result envelope for the first proof package.

## Required reads before any future promotion
1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
3. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e4-f1-prebuild-readiness-closure-2026-06-27.md`
4. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/technical-design.md`
5. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/validation-and-preview-workflow-requirements.md`
6. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e1-minimum-proof-object-selection-2026-06-22.md`
7. `/home/ubuntu/workspace/repos/codex/src/homebrew_authoring/mod.rs`
8. `/home/ubuntu/workspace/repos/codex/src/homebrew_authoring/package_manifest.rs`
9. `/home/ubuntu/workspace/repos/codex/src/homebrew_authoring/package_store.rs`
10. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md`

## Promotion gate
A future documentary run may mint the live E4-F1 execution handoff only if all are true:
1. GE08-E3-F1 has branch-ready or stronger evidence.
2. The live repo still needs only the candidate bridge surface below after resetting to `origin/develop`.
3. The candidate write scope remains the smallest truthful bridge lane.
4. No upstream doctrine changed the headless envelope or explanation obligations.

If any item fails, throw this draft away and derive a fresh stage-specific handoff.

## Candidate write scope after promotion
```text
src/lib.rs
src/homebrew_authoring/mod.rs
src/homebrew_authoring/preview_bridge.rs
tests/ge08_preview_bridge.rs
tests/fixtures/ge08/guard-stance-package/**
```

## Draft implementation behavior
When promoted, the live handoff should require the coding harness to:
1. create a failing focused bridge test first
2. surface `homebrew_authoring` from `src/lib.rs` only as needed for the bounded bridge entrypoint
3. emit a result envelope carrying `success`, `blocked`, and `unsupported`
4. preserve diagnostics, provenance/source refs, explanation refs, and oracle-dimension status in that envelope
5. prove the Human bonus feat substitution path into the bounded armor-class preview output without widening into UI or plugin work

## Candidate verification commands
The future live handoff should, at minimum, require:
```bash
export PATH=/home/ubuntu/.cargo/bin:$PATH
cargo test ge08_validation_and_diagnostics -- --nocapture
cargo test ge08_preview_bridge -- --nocapture
```

If a broader repo test becomes necessary, state why explicitly rather than widening silently.

## Non-goals
The future live handoff must not authorize:
- GE-07 UI/editor work
- plugin runtime or extension-system work
- broad formula-language work
- old-system/PCGen execution
- Cargo dependency changes unless the packet proves incomplete

## Why this draft exists
Todd asked for future GE-08 Claude lanes to be visible and explicit before the next launch boundary arrives.

This draft satisfies that request without counterfeit activation: it captures the bounded bridge surface, required reads, and verification posture while leaving the real code-authorizing moment in the future.
