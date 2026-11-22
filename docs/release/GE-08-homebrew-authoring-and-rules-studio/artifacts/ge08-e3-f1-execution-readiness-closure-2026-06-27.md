---
title: GE08-E3-F1 Execution Readiness Closure
artifact_type: execution-readiness-closure
stc_id: STC-CODEX-GE-08
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE08-E3-F1 — Headless validation and diagnostics for first proof package
workflow_route: readiness-closure
readiness: codex-ready
handoff_created: true
created_handoff:
  - ./ge08-e3-f1-execution-handoff-2026-06-27.md
review_date: 2026-06-27
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE08-E3-F1 Execution Readiness Closure

## Verdict
GE08-E3-F1 is now grounded sufficiently to mint the next narrow Claude-routed coding handoff, and that paired live artifact now exists.

The active E3-F1 code-authorizing artifact created from this readiness closure is:

```text
programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e3-f1-execution-handoff-2026-06-27.md
```

This readiness closure is not code authority. It records why the separate E3-F1 handoff now truthfully carries `code_authority: true` while the root `execution-handoff.md` remains only a route surface.

## Core problem
GE08-E2-F1 proved the deterministic authored package bundle can be created, saved, loaded, diffed, and export-gated. But without a bounded validation/diagnostics lane, later preview claims could counterfeit success by treating malformed, widened, deferred, or provenance-broken authored content as though it were trustworthy proof input.

The next honest move is therefore not more lifecycle restatement. It is a refusal-first validation/diagnostics slice that keeps machine-readable diagnostics, claim-blocking posture, and provenance/source references explicit before preview or explanation is allowed.

## Selected bounded slice
```text
GE08-E3-F1 — Headless validation and diagnostics for first proof package
```

This slice should do only four things:
1. validate the deterministic authored package bundle structurally and semantically against the first-proof contract
2. preserve machine-readable diagnostic classes, source refs, and provenance refs inside the GE-08 authored package flow
3. block preview/explanation/export claims whenever claim-bearing diagnostics remain or the package state is not preview-eligible
4. prove one happy path plus required negative cases without widening into UI, plugins, or broad formula-language work

## Required source evidence recovered
| Gate | Evidence |
|---|---|
| GE08-E2-F1 acceptance is explicit | Todd accepted downstream advance from review gate `t_7c6b7b8f` in this session. |
| Upstream package-lifecycle substrate is now durable | `ge08-e2-f1-branch-ready-receipt-2026-06-27.md` captures a clean branch/commit at `ge08-e2-f1-package-lifecycle@c6c18d2c085dc22cd456977442aa47abc6c267cf`. |
| Validation contract is concrete | `validation-and-preview-workflow-requirements.md` names the state model, refusal rules, diagnostic classes, and required negative cases. |
| Package contract is already bounded | `package-file-lifecycle-requirements.md` fixes the deterministic bundle shape, package-state fields, and existing GE08-E2 verification commands. |
| First-proof object is fixed | `ge08-e1-minimum-proof-object-selection-2026-06-22.md` fixes the Guard Stance proof object and Human bonus feat substitution boundary. |
| Exact launch worktree now exists | `/home/ubuntu/workspace/repos/codex-ge08-e3-f1` is a clean worktree on `ge08-e3-f1-validation-and-diagnostics`, stacked on `c6c18d2c085dc22cd456977442aa47abc6c267cf`. |
| Claude launch substrate exists | Claude Code CLI is installed at `/home/ubuntu/.local/bin/claude` and authenticated under Todd's Claude Max account. |

## Grounded implementation posture
Because the accepted E2 substrate is not merged into `origin/develop`, the truthful E3 base is a **stacked branch** rather than fresh `origin/develop`.

The live launch worktree is:

```text
/home/ubuntu/workspace/repos/codex-ge08-e3-f1
branch: ge08-e3-f1-validation-and-diagnostics
stacked base commit: c6c18d2c085dc22cd456977442aa47abc6c267cf
upstream substrate branch: ge08-e2-f1-package-lifecycle
```

This closure therefore authorizes a stacked Claude launch packet, not a develop-based one.

## Exact write surface justified by the evidence
The paired handoff confines implementation to:

```text
src/homebrew_authoring/mod.rs
src/homebrew_authoring/package_manifest.rs
src/homebrew_authoring/package_store.rs
tests/ge08_package_file_lifecycle.rs
tests/ge08_validation_and_diagnostics.rs
tests/fixtures/ge08/guard-stance-package-invalid-missing-effect/**
tests/fixtures/ge08/guard-stance-package-invalid-widened-preview/**
```

That scope is sufficient because the package-lifecycle foothold now exists on the accepted E2 branch capture and the design docs already name the required diagnostic classes and negative cases.

## Verification contract for the coding harness
The paired execution handoff requires at minimum:

```bash
export PATH=/home/ubuntu/.cargo/bin:$PATH
cargo test ge08_package_file_lifecycle -- --nocapture
cargo test ge08_validation_and_diagnostics -- --nocapture
```

The first command protects the accepted E2 substrate. The second proves the new refusal-first validation lane.

## Handoff minting decision
The analysis is complete. The blocker is gone.

GE08-E3-F1 is now a truthful, bounded, launch-ready coding slice. The route surface should advance to `awaiting-todd-launch`, paired with a stage-specific execution handoff and a board-visible Todd launch gate.
