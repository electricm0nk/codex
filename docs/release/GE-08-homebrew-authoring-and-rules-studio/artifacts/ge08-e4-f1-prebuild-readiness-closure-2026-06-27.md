---
title: GE08-E4-F1 Prebuild Readiness Closure
artifact_type: execution-readiness-prebuild
stc_id: STC-CODEX-GE-08
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE08-E4-F1 — Headless preview and explanation bridge for first proof package
workflow_route: readiness-closure
readiness: blocked
status: prebuilt-draft
created_handoff:
  - ./ge08-e4-f1-prebuild-handoff-2026-06-27.md
review_date: 2026-06-27
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE08-E4-F1 Prebuild Readiness Closure

## Verdict
A bounded GE08-E4-F1 packet can be prebuilt now, but it must remain non-authorizing until GE08-E3-F1 has real implementation evidence and the live repo still matches the bridge assumptions captured here.

This artifact exists to make the future Claude launch explicit rather than leaving E4 as a Hermes-owned placeholder.

## Core problem
GE-08 already has a concrete headless bridge contract in the design docs, but the live repo still lacks both the validation/diagnostics lane and the bridge entrypoint that would consume it. The next bridge packet can therefore be described precisely now, but it cannot become live code authority until E3 truth exists.

Without this prebuilt packet, the board is tempted to treat E4 as just another generic implementation story. That is the failure pattern we are stopping.

## Selected bounded slice
```text
GE08-E4-F1 — Headless preview and explanation bridge for first proof package
```

This slice should do only five things once its gate opens:
1. surface `homebrew_authoring` from `src/lib.rs`
2. consume the validated deterministic authored package bundle plus the fixed GE08-E1 proof binding
3. emit a result envelope that distinguishes `success`, `blocked`, and `unsupported`
4. preserve diagnostics, provenance/source refs, explanation refs, and oracle-dimension status in that envelope
5. prove the Human bonus feat substitution path into the bounded armor-class preview output without widening into editor, plugin, or broad rules-authoring work

It must not own GE-07 UI/editor work, plugin runtime, broad formula language, public sharing, or product-visible desktop-shell behavior.

## Grounded source evidence available now
| Gate | Grounded source |
|---|---|
| Bridge contract is concrete | `technical-design.md` fixes the headless preview bridge responsibilities and non-goals. |
| Validation/preview workflow is concrete | `artifacts/validation-and-preview-workflow-requirements.md` names the required result-envelope posture, refusal rules, and negative cases. |
| Upstream proof binding is fixed | `artifacts/ge08-e1-minimum-proof-object-selection-2026-06-22.md` fixes the Human bonus feat substitution case. |
| Live repo still lacks the bridge surface | `/home/ubuntu/workspace/repos/codex/src/lib.rs` does not yet surface `homebrew_authoring`, and no GE08 preview/explanation bridge tests exist. |
| Current shell residue is not the base we want | The repo shell is currently on `ge07-e1-desktop-shell-scaffold`; any future GE08 launch must reset to `origin/develop` first. |

## Launch gates that remain closed
This packet must not be promoted into a code-authorizing handoff until all are true:
1. GE08-E3-F1 has branch-ready or stronger evidence from the real implementation lane.
2. The live repo on `origin/develop` still matches the bounded bridge assumptions captured here.
3. The draft write scope below remains the smallest truthful bridge surface after re-reading E3 outputs.
4. No upstream GE-04/GE-06/GE-08 doctrine changed the result-envelope or explanation obligations.

If any gate fails, re-derive the packet instead of widening silently.

## Candidate implementation posture after gate clear
The smallest likely implementation surface is:

```text
src/lib.rs
src/homebrew_authoring/mod.rs
src/homebrew_authoring/preview_bridge.rs
tests/ge08_preview_bridge.rs
tests/fixtures/ge08/guard-stance-package/**
```

Read-only dependencies for that later run should include:

```text
src/homebrew_authoring/package_manifest.rs
src/homebrew_authoring/package_store.rs
programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/technical-design.md
programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/validation-and-preview-workflow-requirements.md
programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e1-minimum-proof-object-selection-2026-06-22.md
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md
AGENTS.md
CLAUDE.md
```

## Expected bridge obligations after promotion
The future live handoff should require, at minimum:
- one happy-path proof for `pf1-crb-human-fighter-level1-homebrew-feat-proof` surviving load -> validate -> prepare -> preview -> explain
- one blocked-path proof showing malformed or widened authored content returns diagnostics and blocked claims rather than counterfeit success
- an explicit envelope carrying `success`, `blocked`, and `unsupported` states
- preservation of diagnostics, provenance/source refs, explanation refs, and oracle-dimension status in that envelope

## Explicit non-goals
Do not let a future E4-F1 handoff authorize:
- edits outside `src/lib.rs`, bounded `src/homebrew_authoring/**`, bounded GE08 bridge tests, and bounded GE08 fixtures
- GE-07 product-visible/editor work
- plugin runtime or extension-system work
- broad formula-language or general rules-authoring work
- Cargo dependency changes unless a blocker proves the packet incomplete

## Completion rule
This prebuild readiness closure is complete when it leaves no ambiguity about three facts:
- what GE08-E4-F1 is supposed to bridge
- which write surface it is allowed to touch
- exactly why the packet still cannot be launched before GE08-E3-F1 evidence exists
