---
title: GE08-E3-F1 Execution Handoff — Headless Validation and Diagnostics for First Proof Package
handoff_id: HANDOFF-CODEX-GE-08-E3-F1-CODING-2026-06-27
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
readiness_closure: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e3-f1-execution-readiness-closure-2026-06-27.md
selected_slice: GE08-E3-F1 — Headless validation and diagnostics for first proof package
run_in: Claude Code or equivalent frontier coding harness
code_authority: true
created_at: 2026-06-27
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex-ge08-e3-f1
  workdir: /home/ubuntu/workspace/repos/codex-ge08-e3-f1
  branch_base: ge08-e2-f1-package-lifecycle
  expected_base_sha_at_creation: c6c18d2c085dc22cd456977442aa47abc6c267cf
  recommended_branch: ge08-e3-f1-validation-and-diagnostics
  pr_target: develop
allowed_write_scope:
  - src/homebrew_authoring/mod.rs
  - src/homebrew_authoring/package_manifest.rs
  - src/homebrew_authoring/package_store.rs
  - tests/ge08_package_file_lifecycle.rs
  - tests/ge08_validation_and_diagnostics.rs
  - tests/fixtures/ge08/guard-stance-package-invalid-missing-effect/**
  - tests/fixtures/ge08/guard-stance-package-invalid-widened-preview/**
forbidden_write_scope:
  - /home/ubuntu/workspace/repos/pcgen/**
  - programs/codex/**
  - src/lib.rs
  - src/oracle_validation/**
  - src/pcgen_import/**
  - src/rules_core/**
  - tests/ge08_preview_bridge.rs
  - Cargo.toml
  - Cargo.lock
  - AGENTS.md
  - CLAUDE.md
---

# GE08-E3-F1 Execution Handoff — Headless Validation and Diagnostics for First Proof Package

## Status
This is the live stage-specific code-authorizing brief for GE08-E3-F1.

It carries `code_authority: true` for GE08-E3-F1 only and is currently `awaiting-todd-launch`.

## Run in
Claude Code or an equivalent frontier coding harness.

Do not run this in Hermes as a documentary card. Hermes produced this handoff; the coding harness implements it.

## Core problem
GE08-E2-F1 proved deterministic authored package lifecycle, but not honesty under malformed or widened authored content. Until Codex can validate authored packages, preserve machine-readable diagnostics, and refuse preview/explanation/export claims when blocking diagnostics remain, any later bridge lane would be building on counterfeit trust.

The next honest move is narrower: add the validation/diagnostics substrate only, keep the claim-blocking posture explicit, and avoid widening into preview bridge, UI, plugin runtime, or general authoring language work.

## Objective
Create the smallest validation/diagnostics layer that makes the first proof package honest.

The result must prove:
1. a valid guard-stance package remains preview-eligible only after validation
2. missing required record/reference structure emits machine-readable diagnostics and blocks preview/explanation/export claims
3. widened or unsupported semantics are refused rather than silently accepted
4. package-state-aware refusal (`draft`, `invalid`, `deferred`) remains explicit and machine-checkable
5. provenance/source refs survive through the validation path instead of being discarded

## Branch / worktree policy
Do **not** reset this lane to `origin/develop`. The accepted GE08-E2-F1 substrate is not merged yet, so the truthful base is a stacked launch worktree.

Launch from the already-provisioned clean worktree:

```text
/home/ubuntu/workspace/repos/codex-ge08-e3-f1
branch: ge08-e3-f1-validation-and-diagnostics
base branch: ge08-e2-f1-package-lifecycle
base commit: c6c18d2c085dc22cd456977442aa47abc6c267cf
```

If you need to recreate the worktree, the equivalent setup is:

```bash
git fetch origin --prune
git worktree add /home/ubuntu/workspace/repos/codex-ge08-e3-f1 -b ge08-e3-f1-validation-and-diagnostics c6c18d2c085dc22cd456977442aa47abc6c267cf
```

Record the actual launch worktree, branch, and base SHA in the final receipt.

## Required reads before coding
Read these first, in order:
1. `/home/ubuntu/workspace/repos/codex-ge08-e3-f1/AGENTS.md`
2. `/home/ubuntu/workspace/repos/codex-ge08-e3-f1/CLAUDE.md`
3. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e3-f1-execution-readiness-closure-2026-06-27.md`
4. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/execution-handoff.md`
5. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e2-f1-branch-ready-receipt-2026-06-27.md`
6. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/validation-and-preview-workflow-requirements.md`
7. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/package-file-lifecycle-requirements.md`
8. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e1-minimum-proof-object-selection-2026-06-22.md`
9. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/technical-design.md`
10. `/home/ubuntu/workspace/repos/codex-ge08-e3-f1/src/homebrew_authoring/mod.rs`
11. `/home/ubuntu/workspace/repos/codex-ge08-e3-f1/src/homebrew_authoring/package_manifest.rs`
12. `/home/ubuntu/workspace/repos/codex-ge08-e3-f1/src/homebrew_authoring/package_store.rs`
13. `/home/ubuntu/workspace/repos/codex-ge08-e3-f1/tests/ge08_package_file_lifecycle.rs`

## Required RED -> GREEN execution pattern
TDD is mandatory here.

### RED first
Before changing production code, add a focused failing test in `tests/ge08_validation_and_diagnostics.rs` that proves the current stack still accepts or mishandles one of the required invalid cases.

At minimum, the failing assertions must cover:
- missing required record/reference structure blocks preview eligibility and emits machine-readable diagnostics
- widened/unsupported authored semantics are refused rather than accepted
- a valid guard-stance package still validates cleanly without degrading the accepted E2 lifecycle path

Capture the RED result in the final report.

### GREEN second
After the intended RED failure, make the smallest change necessary inside the allowed write scope to satisfy the validation contract.

## Allowed write scope
You may write only:

```text
src/homebrew_authoring/mod.rs
src/homebrew_authoring/package_manifest.rs
src/homebrew_authoring/package_store.rs
tests/ge08_package_file_lifecycle.rs
tests/ge08_validation_and_diagnostics.rs
tests/fixtures/ge08/guard-stance-package-invalid-missing-effect/**
tests/fixtures/ge08/guard-stance-package-invalid-widened-preview/**
```

If you need any other file, stop and report the blocker.

## Forbidden widening
This handoff does **not** authorize:
- `src/lib.rs`
- preview/explanation bridge implementation
- GE-07 UI/editor work
- plugin runtime or extension-system work
- broad formula-language or generalized rules-authoring work
- Cargo dependency changes unless a blocker proves the packet incomplete
- any write under `/home/ubuntu/workspace/repos/pcgen`

## Verification commands
Run at minimum:

```bash
export PATH=/home/ubuntu/.cargo/bin:$PATH
cargo test ge08_package_file_lifecycle -- --nocapture
cargo test ge08_validation_and_diagnostics -- --nocapture
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
