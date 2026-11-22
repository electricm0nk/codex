---
title: GE08-E3-F1 Prebuild Handoff Draft — Headless Validation and Diagnostics for First Proof Package
handoff_id: HANDOFF-CODEX-GE-08-E3-F1-PREBUILD-2026-06-27
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
readiness_closure: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e3-f1-prebuild-readiness-closure-2026-06-27.md
selected_slice: GE08-E3-F1 — Headless validation and diagnostics for first proof package
run_in: Claude Code or equivalent frontier coding harness, but only after GE08-E2-F1 gate acceptance and documentary promotion
code_authority: false
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  observed_origin_develop: cc4e1a55caad07af83768a036d4b0f5fffbf99c9
  recommended_branch: ge08-e3-f1-validation-and-diagnostics
future_live_artifacts:
  - artifacts/ge08-e3-f1-execution-readiness-closure-YYYY-MM-DD.md
  - artifacts/ge08-e3-f1-execution-handoff-YYYY-MM-DD.md
  - artifacts/ge08-e3-f1-claude-launch-receipt-YYYY-MM-DD.md
allowed_write_scope:
  - src/homebrew_authoring/mod.rs
  - src/homebrew_authoring/package_manifest.rs
  - src/homebrew_authoring/package_store.rs
  - tests/ge08_package_file_lifecycle.rs
  - tests/ge08_validation_and_diagnostics.rs
  - tests/fixtures/ge08/guard-stance-package-invalid-missing-effect/**
  - tests/fixtures/ge08/guard-stance-package-invalid-widened-preview/**
forbidden_write_scope:
  - /home/ubuntu/workspace/repos/pcgen
  - src/rules_core/**
  - src/oracle_validation/**
  - src/pcgen_import/**
  - src/lib.rs
  - programs/codex/**
  - Cargo.toml
  - Cargo.lock
  - AGENTS.md
  - CLAUDE.md
---

# GE08-E3-F1 Prebuild Handoff Draft — Headless Validation and Diagnostics for First Proof Package

## Status
This is a prebuilt draft only.

Do not hand this to Claude Code yet. It carries `code_authority: false` until a later documentary pass promotes it after GE08-E2-F1 gate acceptance and a fresh `origin/develop` audit.

## Objective
Once GE08-E2-F1 is accepted, create the smallest validation/diagnostics layer that makes the first proof package honest: malformed or widened authored content must emit machine-readable diagnostics and block preview/explanation/export claims instead of slipping through as though it were valid.

## Required reads before any future promotion
1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
3. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e3-f1-prebuild-readiness-closure-2026-06-27.md`
4. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/validation-and-preview-workflow-requirements.md`
5. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/package-file-lifecycle-requirements.md`
6. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e1-minimum-proof-object-selection-2026-06-22.md`
7. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/technical-design.md`
8. `/home/ubuntu/workspace/repos/codex/src/homebrew_authoring/mod.rs`
9. `/home/ubuntu/workspace/repos/codex/src/homebrew_authoring/package_manifest.rs`
10. `/home/ubuntu/workspace/repos/codex/src/homebrew_authoring/package_store.rs`
11. `/home/ubuntu/workspace/repos/codex/tests/ge08_package_file_lifecycle.rs`

## Promotion gate
A future documentary run may mint the live E3-F1 execution handoff only if all are true:
1. GE08-E2-F1 review gate `t_7c6b7b8f` explicitly accepts or advances the package-lifecycle substrate.
2. The live repo still presents the bounded `src/homebrew_authoring/**` surface expected here after resetting to `origin/develop`.
3. The candidate write scope below remains the smallest truthful validation lane.
4. No upstream doctrine changed the first-proof diagnostic contract or package-state posture.

If any item fails, throw this draft away and derive a fresh stage-specific handoff.

## Candidate write scope after promotion
```text
src/homebrew_authoring/mod.rs
src/homebrew_authoring/package_manifest.rs
src/homebrew_authoring/package_store.rs
tests/ge08_package_file_lifecycle.rs
tests/ge08_validation_and_diagnostics.rs
tests/fixtures/ge08/guard-stance-package-invalid-missing-effect/**
tests/fixtures/ge08/guard-stance-package-invalid-widened-preview/**
```

## Draft implementation behavior
When promoted, the live handoff should require the coding harness to:
1. write or extend a failing focused validation/diagnostics test first
2. preserve the deterministic package source bundle as the only authored-source authority
3. emit machine-readable diagnostics for missing required files, unresolved references, invalid package state, and widened/unsupported semantics
4. refuse preview/explanation/export claims whenever blocking diagnostics or non-preview-eligible package states remain
5. avoid preview-bridge rendering, GE-07 UI work, plugin runtime, or broad formula-language work

## Candidate verification commands
The future live handoff should, at minimum, require:
```bash
export PATH=/home/ubuntu/.cargo/bin:$PATH
cargo test ge08_package_file_lifecycle -- --nocapture
cargo test ge08_validation_and_diagnostics -- --nocapture
```

If a broader repo test becomes necessary, state why explicitly rather than widening silently.

## Non-goals
The future live handoff must not authorize:
- edits to `src/lib.rs`
- preview/explanation bridge implementation beyond claim-blocking posture
- old-system/PCGen execution
- UI/editor work
- plugin runtime or extension-system work
- Cargo dependency changes unless the packet proves incomplete

## Why this draft exists
Todd asked for GE-08 to stop collapsing intended-Claude lanes into Hermes-owned code cards.

This draft satisfies that requirement without counterfeit activation: it captures the bounded write scope, required reads, and verification posture while leaving the real code-authorizing moment in the future, where it belongs.
