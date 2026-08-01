---
title: GE07-E1 Execution Readiness Closure
artifact_type: execution-readiness-closure
stc_id: STC-CODEX-GE-07
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE07-E1 — Shell scaffold and runtime boundary spike
workflow_route: coding-readiness
readiness: codex-ready
handoff_created: true
review_date: 2026-06-22
owner: Todd Hintzmann
scope: program
code_authority: false
created_from_breakage_repair: true
paired_handoff: ./ge07-e1-execution-handoff-2026-06-22.md
---

# GE07-E1 Execution Readiness Closure

## Verdict
GE07-E1 is now ready for a bounded code-authorizing handoff.

The previous GE-07 board pass correctly discovered that the repo has no desktop scaffold, but it incorrectly allowed the GE07-E1 card to complete without producing the execution handoff that would create that scaffold. This closure repairs that process failure.

## Core problem
GE-07 cannot advance through E2/E3/E4/E5 because every downstream lane depends on a fact that is currently false:

```text
origin/develop has no apps/desktop/ subtree and no Tauri shell scaffold.
```

The decisive next move is therefore not another documentary GE-07 story. It is the first code-producing GE07-E1 scaffold spike.

## Selected bounded slice
```text
GE07-E1 — Shell scaffold and runtime boundary spike
```

## Why this slice is code-ready now
| Gate | Status | Evidence |
|---|---|---|
| Source STC exists | pass | `../README.md` exists and is planning-ready. |
| GE-06 viability posture exists | pass | GE-06 has an explicit domain-confidence decision at `../../GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md`; early UI work remains non-production spike-only. |
| Repo/workdir grounded | pass | `/home/ubuntu/workspace/repos/codex` exists. |
| Branch base grounded | pass | `origin/develop = 7bc89e8c1edf8f1d1a6d490a0ad28ac72fc6f104` observed on 2026-06-22. |
| Current repo has no shell | pass | `git ls-tree -r --name-only origin/develop` contains no `apps/desktop/` or `src-tauri/` entries. |
| Toolchain available | pass | Prior E1 receipt observed Rust, Cargo, Node, and npm in the runtime. |
| Exact write surface known | pass | all writes are confined under `apps/desktop/**`. |
| Non-goals clear | pass | no product UI, no broad character builder, no rules semantics, no packaging/signing claims. |
| Verification posture known | pass | scaffold must preserve existing Rust core tests and provide at least a bounded startup/build/test receipt for the desktop subtree. |

## Target runtime
```text
repo: /home/ubuntu/workspace/repos/codex
workdir: /home/ubuntu/workspace/repos/codex
branch_base: origin/develop
expected_base_sha_at_creation: 7bc89e8c1edf8f1d1a6d490a0ad28ac72fc6f104
recommended_branch: ge07-e1-desktop-shell-scaffold
pr_target: develop
```

## Exact allowed write scope
The paired execution handoff may authorize writes only under:

```text
apps/desktop/**
```

That includes, at minimum, the scaffold path family named by the E1 documentary receipt:

```text
apps/desktop/package.json
apps/desktop/src/main.tsx
apps/desktop/src/App.tsx
apps/desktop/src-tauri/Cargo.toml
apps/desktop/src-tauri/tauri.conf.json
apps/desktop/src-tauri/src/main.rs
```

Package-manager or Rust lockfiles are allowed only if they are created under `apps/desktop/**`.

## Forbidden write scope
The handoff must forbid changes to:

```text
Cargo.toml
Cargo.lock
src/**
tests/**
AGENTS.md
CLAUDE.md
programs/codex/**
/home/ubuntu/workspace/repos/pcgen/**
```

The coding harness must not patch governance documents, root Rust code, or existing tests during this scaffold spike.

## Implementation boundary
The E1 implementation may:
- create an additive desktop shell subtree under `apps/desktop/`
- create the smallest Tauri/TypeScript/React shell scaffold consistent with the current GE-07 architecture posture
- include a read-only boundary stub or command shape that clearly forwards/represents current GE-06 headless truth without implementing rules semantics locally
- include tests or static checks within `apps/desktop/**`
- record startup/build limitations truthfully in the final report

The E1 implementation must not:
- modify the Rust headless core
- implement the GE06-E4-F1 rules-core view-model bridge
- create product-visible character-builder breadth
- hide diagnostics or explanation obligations
- claim packaging/signing/release readiness
- merge anything into `develop`

## Verification requirement
The paired handoff must require the coding harness to run, at minimum:

```bash
git fetch origin --prune
git switch develop
git reset --hard origin/develop
git switch -c ge07-e1-desktop-shell-scaffold
"$HOME/.cargo/bin/cargo" test --quiet
```

Then, inside `apps/desktop/`, the harness must run the strongest available scaffold verification after creating the app. The preferred commands are:

```bash
npm install
npm run typecheck
npm run build
```

If Tauri platform dependencies prevent a full desktop build in the available environment, the harness must still run the TypeScript/static build checks that are available and record the exact blocker instead of claiming a successful desktop build.

## Completion rule for this closure
This closure is complete because it converts the previously discovered stop condition into the missing stage-specific execution handoff. Downstream GE07-E2/E3/E4/E5 must still wait for their own prerequisites, but GE07-E1 no longer lacks a launchable coding artifact.
