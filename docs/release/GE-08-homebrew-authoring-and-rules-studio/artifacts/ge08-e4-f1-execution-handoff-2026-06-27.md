---
title: GE08-E4-F1 Execution Handoff — Headless Preview and Explanation Bridge for First Proof Package
handoff_id: HANDOFF-CODEX-GE-08-E4-F1-CODING-2026-06-27
stc_id: STC-CODEX-GE-08
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: awaiting-todd-launch
owner: Todd Hintzmann
scope: program
canonical: false
canonical_path: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/execution-handoff.md
source_stc: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/README.md
readiness_closure: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e4-f1-execution-readiness-closure-2026-06-27.md
selected_slice: GE08-E4-F1 — Headless preview and explanation bridge for first proof package
run_in: Claude Code or equivalent frontier coding harness
code_authority: true
created_at: 2026-06-27
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex-ge08-e4-f1
  workdir: /home/ubuntu/workspace/repos/codex-ge08-e4-f1
  branch_base: ge08-e3-f1-validation-and-diagnostics
  expected_base_sha_at_creation: 6de72fcd2ba2bacc79bc4ae4f0a8bf163c875606
  recommended_branch: ge08-e4-f1-preview-and-explanation-bridge
  pr_target: develop
allowed_write_scope:
  - src/lib.rs
  - src/homebrew_authoring/mod.rs
  - src/homebrew_authoring/preview_bridge.rs
  - tests/ge08_preview_bridge.rs
  - tests/fixtures/ge08/guard-stance-package/**
forbidden_write_scope:
  - /home/ubuntu/workspace/repos/pcgen/**
  - programs/codex/**
  - src/oracle_validation/**
  - src/pcgen_import/**
  - src/rules_core/**
  - tests/ge08_validation_and_diagnostics.rs
  - Cargo.toml
  - Cargo.lock
  - AGENTS.md
  - CLAUDE.md
---

# GE08-E4-F1 Execution Handoff — Headless Preview and Explanation Bridge for First Proof Package

## Status
This is the live stage-specific code-authorizing brief for GE08-E4-F1.

It carries `code_authority: true` for GE08-E4-F1 only and is currently `awaiting-todd-launch`.

## Run in
Claude Code or an equivalent frontier coding harness.

Do not run this in Hermes as a documentary card. Hermes produced this handoff; the coding harness implements it.

## Core problem
GE08-E3-F1 proved validation honesty, but not the bounded bridge that turns a validated proof package into a preview/explanation envelope. Until Codex can consume the validated authored bundle, preserve diagnostics and provenance/source context, and distinguish `success`, `blocked`, and `unsupported` outcomes, the GE08 route still lacks the first truthful bridge into product-visible authoring behavior.

The next honest move is narrower: add the headless preview/explanation bridge only, prove the Human bonus feat substitution path into the bounded armor-class preview output, and avoid widening into GE-07 UI, plugin runtime, or general rules-authoring work.

## Objective
Create the smallest bridge layer that makes the first proof package previewable and explainable without counterfeit success.

The result must prove:
1. a valid validated guard-stance package plus the fixed proof binding can produce the bounded armor-class preview output
2. blocked authored content returns a blocked envelope with diagnostics and references instead of counterfeit success
3. unsupported authored content remains explicit rather than silently widened
4. diagnostics, provenance/source refs, explanation refs, and oracle-dimension status survive in the result envelope
5. the bridge remains headless and bounded, not a disguised GE-07/UI/editor packet

## Branch / worktree policy
Do **not** reset this lane to `origin/develop`. GE08-E3-F1 is branch-ready but not merged, so the truthful base is a stacked launch worktree.

Launch from the already-provisioned clean worktree:

```text
/home/ubuntu/workspace/repos/codex-ge08-e4-f1
branch: ge08-e4-f1-preview-and-explanation-bridge
base branch: ge08-e3-f1-validation-and-diagnostics
base commit: 6de72fcd2ba2bacc79bc4ae4f0a8bf163c875606
```

If you need to recreate the worktree, the equivalent setup is:

```bash
git fetch origin --prune
git worktree add /home/ubuntu/workspace/repos/codex-ge08-e4-f1 -b ge08-e4-f1-preview-and-explanation-bridge 6de72fcd2ba2bacc79bc4ae4f0a8bf163c875606
```

Record the actual launch worktree, branch, and base SHA in the final receipt.

## Required reads before coding
Read these first, in order:
1. `/home/ubuntu/workspace/repos/codex-ge08-e4-f1/AGENTS.md`
2. `/home/ubuntu/workspace/repos/codex-ge08-e4-f1/CLAUDE.md`
3. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e4-f1-execution-readiness-closure-2026-06-27.md`
4. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/execution-handoff.md`
5. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e3-f1-branch-ready-receipt-2026-06-27.md`
6. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/validation-and-preview-workflow-requirements.md`
7. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e1-minimum-proof-object-selection-2026-06-22.md`
8. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/technical-design.md`
9. `/home/ubuntu/workspace/repos/codex-ge08-e4-f1/src/lib.rs`
10. `/home/ubuntu/workspace/repos/codex-ge08-e4-f1/src/homebrew_authoring/mod.rs`
11. `/home/ubuntu/workspace/repos/codex-ge08-e4-f1/src/homebrew_authoring/package_manifest.rs`
12. `/home/ubuntu/workspace/repos/codex-ge08-e4-f1/src/homebrew_authoring/package_store.rs`
13. `/home/ubuntu/workspace/repos/codex-ge08-e4-f1/tests/ge08_validation_and_diagnostics.rs`

## Required RED -> GREEN execution pattern
TDD is mandatory here.

### RED first
Before changing production code, add a focused failing test in `tests/ge08_preview_bridge.rs` that proves the current stack still lacks the bounded bridge or mishandles one of the required blocked/unsupported envelopes.

At minimum, the failing assertions must cover:
- a valid validated package plus the fixed proof binding can reach the bounded armor-class preview path only after the bridge exists
- blocked authored content remains blocked with diagnostics and references preserved
- unsupported authored content is explicit rather than silently widened

Capture the RED result in the final report.

### GREEN second
After the intended RED failure, make the smallest change necessary inside the allowed write scope to satisfy the bridge contract.

## Allowed write scope
You may write only:

```text
src/lib.rs
src/homebrew_authoring/mod.rs
src/homebrew_authoring/preview_bridge.rs
tests/ge08_preview_bridge.rs
tests/fixtures/ge08/guard-stance-package/**
```

If you need any other file, stop and report the blocker.

## Forbidden widening
This handoff does **not** authorize:
- GE-07 UI/editor work
- plugin runtime or extension-system work
- broad formula-language or generalized rules-authoring work
- old-system/PCGen execution
- edits under `src/oracle_validation/**`, `src/pcgen_import/**`, or `src/rules_core/**`
- Cargo dependency changes unless a blocker proves the packet incomplete
- any write under `/home/ubuntu/workspace/repos/pcgen`

## Verification commands
Run at minimum:

```bash
export PATH=/home/ubuntu/.cargo/bin:$PATH
cargo test ge08_package_file_lifecycle -- --nocapture
cargo test ge08_validation_and_diagnostics -- --nocapture
cargo test ge08_preview_bridge -- --nocapture
```

If a broader repo test becomes necessary, say exactly why.

## Merge authority boundary
This handoff does **not** authorize the coding harness to merge anything.

- Do not merge the branch or PR.
- Do not land code directly onto `develop` or `main`.
- Stop at verified branch or PR-ready state and hand control back to Todd.

## Final report requirements
The final execution receipt must include:
- exact handoff path
- exact worktree path used
- actual branch name
- actual base SHA
- files changed
- RED failure summary
- final verification commands and results
- whether a PR was created or the lane stopped at branch-ready

Without that receipt, this lane must not be described as Claude-executed.
