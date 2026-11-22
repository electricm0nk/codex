---
title: GE07-E1 Execution Handoff — Desktop Shell Scaffold and Runtime Boundary Spike
handoff_id: HANDOFF-CODEX-GE-07-E1-CODING-2026-06-22
stc_id: STC-CODEX-GE-07
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: awaiting-todd-merge
owner: Todd Hintzmann
scope: program
canonical: false
canonical_path: programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/execution-handoff.md
source_stc: programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/README.md
readiness_closure: programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e1-execution-readiness-closure-2026-06-22.md
selected_slice: GE07-E1 — Shell scaffold and runtime boundary spike
run_in: Claude Code or equivalent frontier coding harness
code_authority: true
created_at: 2026-06-22
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  expected_base_sha_at_creation: 7bc89e8c1edf8f1d1a6d490a0ad28ac72fc6f104
  recommended_branch: ge07-e1-desktop-shell-scaffold
  pr_target: develop
allowed_write_scope:
  - apps/desktop/**
forbidden_write_scope:
  - Cargo.toml
  - Cargo.lock
  - src/**
  - tests/**
  - AGENTS.md
  - CLAUDE.md
  - programs/codex/**
  - /home/ubuntu/workspace/repos/pcgen/**
---

# GE07-E1 Execution Handoff — Desktop Shell Scaffold and Runtime Boundary Spike

## Status
This is the live stage-specific code-authorizing brief for GE07-E1.

It carries `code_authority: true` for **GE07-E1 only** and, after one completed coding pass, is now `awaiting-todd-merge`.

## Run in
Claude Code or an equivalent frontier coding harness.

Do not run this in Hermes as a documentary card. Hermes produced this handoff; the coding harness implements it.

## Core problem
GE-07 downstream work is blocked because the repository has no desktop-shell scaffold at all. Several completed GE-07 documentary closures correctly recorded that stop condition, but they did not create the code-authorizing packet needed to resolve it.

This handoff creates the first real GE-07 implementation foothold: an additive, non-production desktop shell scaffold under `apps/desktop/`.

## Objective
Create the smallest additive desktop shell scaffold and runtime-boundary spike under `apps/desktop/` without touching the existing headless Rust core.

The result must prove:
1. a desktop app subtree exists in the repo
2. the scaffold is visibly separate from the root headless core
3. the shell can build or at least pass the strongest available static/scaffold checks in the current environment
4. the shell has a clear placeholder/boundary for future real GE-06 pilot data consumption
5. no product UI, packaging, release, or rules-semantics claim is made

## Required reads
Read these before editing code:

1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
3. `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e1-execution-readiness-closure-2026-06-22.md`
4. `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e1-shell-scaffold-receipt-2026-06-22.md`
5. `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e1-runtime-boundary-adr-input-2026-06-22.md`
6. `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/pilot-shell-architecture-requirements.md`
7. `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ui-command-boundary-requirements.md`
8. `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/component-surface-inventory.md`
9. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md`
10. `programs/codex/doctrine/quality-gate-policy.md`

Do not read broad documentation trees unless one of the above documents explicitly forces it.

## Branch setup
From `/home/ubuntu/workspace/repos/codex`:

```bash
git fetch origin --prune
git switch develop
git reset --hard origin/develop
git switch -c ge07-e1-desktop-shell-scaffold
```

Expected base at handoff creation:

```text
origin/develop = 7bc89e8c1edf8f1d1a6d490a0ad28ac72fc6f104
```

If `origin/develop` has advanced, use the current fetched `origin/develop` and record the actual base SHA in the final report.

## Baseline repo posture
Observed at handoff creation:

```text
/home/ubuntu/workspace/repos/codex
root files: Cargo.toml, Cargo.lock, src/, tests/, AGENTS.md, CLAUDE.md
origin/develop: 7bc89e8c1edf8f1d1a6d490a0ad28ac72fc6f104
apps/desktop: absent
src-tauri: absent
package.json: absent at repo root
```

The repo is currently a headless Rust library/test surface. Preserve that truth.

## Merge authority boundary
This handoff does not authorize the coding harness to merge anything.

- Do not merge the branch or PR.
- Do not land code directly onto `develop` or `main`.
- Stop at a verified branch state and hand control back to Todd for review/merge.

## Verified post-run repo truth
Observed on 2026-06-24 after `git fetch origin --prune`, branch inspection, and GitHub PR discovery:

```text
repo: /home/ubuntu/workspace/repos/codex
branch: ge07-e1-desktop-shell-scaffold
head: 48892249d5573927bf23a7e47a6d7d6a742da664
origin/develop: 7bc89e8c1edf8f1d1a6d490a0ad28ac72fc6f104
head merged into origin/develop: no
feature branch on origin: present (`origin/ge07-e1-desktop-shell-scaffold`)
open GitHub PR discovered: no
```

## Allowed write scope
You may write only under:

```text
apps/desktop/**
```

This includes files such as:

```text
apps/desktop/package.json
apps/desktop/package-lock.json
apps/desktop/index.html
apps/desktop/tsconfig.json
apps/desktop/vite.config.ts
apps/desktop/src/main.tsx
apps/desktop/src/App.tsx
apps/desktop/src-tauri/Cargo.toml
apps/desktop/src-tauri/Cargo.lock
apps/desktop/src-tauri/tauri.conf.json
apps/desktop/src-tauri/src/main.rs
```

The exact file list may differ if the scaffold tool generates a standard Tauri 2 layout, but all writes must remain inside `apps/desktop/**`.

## Forbidden write scope
Do not modify:

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

If a scaffold tool tries to edit root files, stop and report the blocker.

## Required implementation behavior
Build the smallest scaffold that satisfies GE07-E1 without smuggling downstream scope.

Required:
- create an additive desktop app subtree under `apps/desktop/`
- keep the root Rust crate as the headless domain authority
- use the current GE-07 architecture posture: Tauri 2 plus TypeScript/React unless the toolchain makes that impossible, in which case stop and report the exact blocker
- include a visible runtime-boundary seam for a future read-only pilot shell snapshot
- make clear in code naming or comments that any placeholder state is scaffolding only, not product truth
- expose diagnostics/blocked/computed concepts as future boundary vocabulary if a scaffold placeholder is needed
- add only scaffold-local tests/static checks/config under `apps/desktop/**`

Not required in this slice:
- final UI design
- full pilot workspace rendering
- rules-library browsing
- explanation drawer/panel
- packaging/signing/update workflow
- final command transport ADR
- GE06-E4-F1 rules-core view-model bridge

## Boundary rule
The shell may define a placeholder/boundary shape for future `load_pilot_shell_snapshot` behavior, but it must not implement rules semantics locally.

Allowed boundary placeholder fields:

```text
case_id
source_package_id
receipt_status      # Computed | Blocked | Unknown/Unavailable for scaffold state
summary_values      # absent or clearly marked unavailable until wired to real core data
diagnostics
explanation_refs
```

Do not hardcode the deterministic fighter’s computed values as if the UI already owns them. If any placeholder is shown, label it as a scaffold placeholder and keep the real-data wiring as a future integration point.

## Strict TDD / verification sequence
This is a scaffold spike, so the failure-first proof may be a scaffold-local test/static check rather than a root Rust unit test. Still follow the repo’s discipline: prove the absence/failure first, then make the smallest change green.

### BASELINE
Before creating files, run:

```bash
"$HOME/.cargo/bin/cargo" test --quiet
```

Record the result.

### RED
Create the smallest scaffold-local check first. Acceptable examples:

- a TypeScript typecheck script that initially fails because the shell boundary module/component does not exist
- a scaffold-local test that expects the shell app component or boundary vocabulary to exist
- a Tauri/Rust compile check that initially fails because the command module is absent

Then run the chosen check and confirm it fails for the expected reason.

### GREEN
Implement the minimal scaffold and boundary placeholder needed to pass the scaffold-local check.

### VERIFY
Run the strongest available verification set.

At minimum:

```bash
"$HOME/.cargo/bin/cargo" test --quiet
```

Inside `apps/desktop/`, prefer:

```bash
npm install
npm run typecheck
npm run build
```

If the scaffold uses different standard script names, run the equivalent scripts and report the exact commands.

If a full Tauri desktop build fails because the environment lacks platform libraries, webkit, appindicator, or other host packages, do not hide it. Report:

- what command failed
- the exact missing dependency or error class
- what checks did pass
- whether the committed scaffold remains valid as a non-production code foothold

## Acceptance criteria
The run is successful only if all of these are true:

- `apps/desktop/**` exists on the branch
- all changed files are inside `apps/desktop/**`
- root `cargo test --quiet` still passes, or any failure is unrelated/pre-existing and reported with evidence
- a scaffold-local static/build/test check is added and run
- the shell scaffold remains additive and non-production
- no rules semantics are implemented in the UI
- no GE06-E4-F1 view-model work is duplicated
- no packaging/signing/release readiness is claimed
- final report lists changed files and exact commands/results

## File-scope audit
Before finishing, run:

```bash
git diff --name-only
git ls-files --others --exclude-standard
```

Every path must be under `apps/desktop/`. If not, revert the out-of-scope change or stop and report the violation.

## Final report required
Report:

- branch name and actual base SHA
- files changed
- scaffold tool or manual scaffold approach used
- RED check and observed failure
- GREEN/VERIFY commands and observed results
- any platform dependency blockers
- explicit statement that no merge was performed

## Non-goals
This handoff does not authorize:

- broad “build the UI” work
- final UX polish
- implementation of GE07-E2/E3/E4/E5
- rules-core changes
- oracle/parity work
- product-visible claims
- cross-platform packaging or release claims
- merge to `develop` or `main`
