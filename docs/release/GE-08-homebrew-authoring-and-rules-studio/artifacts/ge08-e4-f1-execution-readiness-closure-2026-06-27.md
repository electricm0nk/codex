---
title: GE08-E4-F1 Execution Readiness Closure
artifact_type: execution-readiness-closure
stc_id: STC-CODEX-GE-08
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE08-E4-F1 — Headless preview and explanation bridge for first proof package
workflow_route: readiness-closure
readiness: codex-ready
handoff_created: true
created_handoff:
  - ./ge08-e4-f1-execution-handoff-2026-06-27.md
review_date: 2026-06-27
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE08-E4-F1 Execution Readiness Closure

## Verdict
GE08-E4-F1 is now grounded sufficiently to mint the next narrow Claude-routed coding handoff, and that live artifact now exists.

The active E4-F1 code-authorizing artifact created from this readiness closure is:

```text
programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e4-f1-execution-handoff-2026-06-27.md
```

This readiness closure is not code authority. It records why the separate E4-F1 handoff now truthfully carries `code_authority: true` while the root `execution-handoff.md` remains only a route surface.

## Core problem
GE08-E3-F1 proved that authored proof packages can be validated, that malformed or widened content is refused with machine-readable claim-blocking diagnostics, and that the first proof package remains preview-eligible only after validation.

The next honest move is therefore no longer another validation pass. It is the bounded headless bridge that consumes the validated package plus the fixed GE08-E1 proof binding and emits a preview/explanation envelope without widening into GE-07 UI, plugin runtime, or broad authoring work.

## Selected bounded slice
```text
GE08-E4-F1 — Headless preview and explanation bridge for first proof package
```

This slice should do only five things:
1. surface `homebrew_authoring` from `src/lib.rs`
2. add the bounded bridge entrypoint in `src/homebrew_authoring/preview_bridge.rs`
3. emit a result envelope distinguishing `success`, `blocked`, and `unsupported`
4. preserve diagnostics, provenance/source refs, explanation refs, and oracle-dimension status in that envelope
5. prove the Human bonus feat substitution path into the bounded armor-class preview output without widening into editor, plugin, or general rules-authoring work

## Required source evidence recovered
| Gate | Evidence |
|---|---|
| GE08-E3-F1 now has implementation evidence | `ge08-e3-f1-branch-ready-receipt-2026-06-27.md` captures a clean stacked branch/commit at `ge08-e3-f1-validation-and-diagnostics@6de72fcd2ba2bacc79bc4ae4f0a8bf163c875606`. |
| Bridge contract is concrete | `technical-design.md` fixes the headless preview bridge responsibilities and non-goals. |
| Validation/preview workflow is concrete | `artifacts/validation-and-preview-workflow-requirements.md` names the result-envelope posture, refusal rules, and negative cases. |
| Upstream proof binding is fixed | `artifacts/ge08-e1-minimum-proof-object-selection-2026-06-22.md` fixes the Human bonus feat substitution case. |
| Exact launch worktree now exists | `/home/ubuntu/workspace/repos/codex-ge08-e4-f1` is a clean worktree on `ge08-e4-f1-preview-and-explanation-bridge`, stacked on `6de72fcd2ba2bacc79bc4ae4f0a8bf163c875606`. |
| Bounded bridge surface still holds | In the clean E4 worktree, `src/lib.rs` does not yet surface `homebrew_authoring`, `src/homebrew_authoring/preview_bridge.rs` does not exist, and no `ge08_preview_bridge` test exists. |

## Grounded implementation posture
The old prebuild assumption about resetting to `origin/develop` is now superseded by real branch truth.

Because GE08-E3-F1 is branch-ready but not merged, the truthful E4 base is another **stacked branch**, not a fresh `origin/develop` lane.

The live launch worktree is:

```text
/home/ubuntu/workspace/repos/codex-ge08-e4-f1
branch: ge08-e4-f1-preview-and-explanation-bridge
stacked base commit: 6de72fcd2ba2bacc79bc4ae4f0a8bf163c875606
upstream substrate branch: ge08-e3-f1-validation-and-diagnostics
```

## Exact write surface justified by the evidence
The paired handoff confines implementation to:

```text
src/lib.rs
src/homebrew_authoring/mod.rs
src/homebrew_authoring/preview_bridge.rs
tests/ge08_preview_bridge.rs
tests/fixtures/ge08/guard-stance-package/**
```

That scope is sufficient because the E3 validation substrate now exists on a durable stacked branch and the design docs already fix the bridge responsibilities and non-goals.

## Verification contract for the coding harness
The paired execution handoff requires at minimum:

```bash
export PATH=/home/ubuntu/.cargo/bin:$PATH
cargo test ge08_package_file_lifecycle -- --nocapture
cargo test ge08_validation_and_diagnostics -- --nocapture
cargo test ge08_preview_bridge -- --nocapture
```

The first two commands protect the accepted E2/E3 substrate. The third proves the new bounded bridge lane.

## Handoff minting decision
The blocking premise has expired.

GE08-E4-F1 is now a truthful, bounded, launch-ready stacked coding slice. The route surface should advance to `awaiting-todd-launch`, paired with a stage-specific execution handoff and a board-visible Todd launch gate.
