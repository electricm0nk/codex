---
title: GE08-E3-F1 Prebuild Readiness Closure
artifact_type: execution-readiness-prebuild
stc_id: STC-CODEX-GE-08
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE08-E3-F1 — Headless validation and diagnostics for first proof package
workflow_route: readiness-closure
readiness: blocked
status: prebuilt-draft
created_handoff:
  - ./ge08-e3-f1-prebuild-handoff-2026-06-27.md
review_date: 2026-06-27
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE08-E3-F1 Prebuild Readiness Closure

## Verdict
A bounded GE08-E3-F1 packet can be prebuilt now, but it must remain non-authorizing until the GE08-E2-F1 review gate explicitly accepts the branch-ready package-lifecycle substrate as the upstream truth for the next slice.

This artifact exists to make that launch boundary explicit rather than letting the board drift back into a Hermes-owned implementation card.

## Core problem
GE08-E2-F1 proved the deterministic authored package bundle can be created, saved, loaded, diffed, and export-gated. That is necessary but insufficient. Without a bounded validation/diagnostics lane, later preview claims can counterfeit success by loading malformed, widened, deferred, or provenance-broken authored content as though it were trustworthy proof input.

The next honest GE-08 slice is therefore not more lifecycle restatement. It is a refusal-first validation and diagnostics lane that keeps machine-readable diagnostics, claim-blocking posture, and source/provenance references explicit before preview or explanation is allowed.

## Selected bounded slice
```text
GE08-E3-F1 — Headless validation and diagnostics for first proof package
```

This slice should do only four things once its gate opens:
1. validate the deterministic authored package bundle structurally and semantically against the first-proof contract
2. preserve machine-readable diagnostic classes, source refs, and provenance refs inside the GE-08 authored package flow
3. block preview/explanation/export claims whenever claim-bearing diagnostics remain or the package state is not preview-eligible
4. prove at least one happy path plus required negative cases without widening into UI, plugins, or broad formula-language work

It must not own preview rendering, final explanation presentation, GE-07 editor work, plugin runtime, or public package registry policy.

## Grounded source evidence available now
| Gate | Grounded source |
|---|---|
| Upstream deterministic package substrate exists | `/home/ubuntu/workspace/repos/codex/src/homebrew_authoring/mod.rs`, `package_manifest.rs`, and `package_store.rs` exist from GE08-E2-F1. |
| First-proof object is fixed | `artifacts/ge08-e1-minimum-proof-object-selection-2026-06-22.md` fixes the Guard Stance proof object and Human bonus feat substitution boundary. |
| Validation workflow is concrete | `artifacts/validation-and-preview-workflow-requirements.md` names the state model, refusal rules, required diagnostic classes, and required negative cases. |
| Package contract is already bounded | `artifacts/package-file-lifecycle-requirements.md` fixes the deterministic bundle shape, package-state fields, and existing GE08-E2 verification commands. |
| Live repo boundary is still narrow | `/home/ubuntu/workspace/repos/codex/tests` currently contains only `ge08_package_file_lifecycle.rs`; the validation/preview tests remain absent and therefore still need an honest next slice. |
| Claude launch substrate exists | Claude Code CLI is installed at `/home/ubuntu/.local/bin/claude` and authenticated under Todd's Claude Max account. |

## Launch gates that remain closed
This packet must not be promoted into a code-authorizing handoff until all are true:
1. `t_7c6b7b8f` records an explicit accept/advance verdict for GE08-E2-F1.
2. The live repo is re-read from `origin/develop` rather than from the currently checked-out GE07 branch residue.
3. The draft write scope below remains the smallest truthful implementation surface after that re-read.
4. The required diagnostic classes and negative cases in `validation-and-preview-workflow-requirements.md` still match the first-proof contract without broader preview/UI obligations.

If any gate fails, re-derive the packet instead of widening silently.

## Candidate implementation posture after gate clear
The smallest likely implementation surface is:

```text
src/homebrew_authoring/mod.rs
src/homebrew_authoring/package_manifest.rs
src/homebrew_authoring/package_store.rs
tests/ge08_package_file_lifecycle.rs
tests/ge08_validation_and_diagnostics.rs
tests/fixtures/ge08/guard-stance-package-invalid-missing-effect/
tests/fixtures/ge08/guard-stance-package-invalid-widened-preview/
```

Read-only dependencies for that later run should include:

```text
src/homebrew_authoring/
programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/validation-and-preview-workflow-requirements.md
programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/package-file-lifecycle-requirements.md
programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e1-minimum-proof-object-selection-2026-06-22.md
programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/technical-design.md
AGENTS.md
CLAUDE.md
```

## Expected validation obligations after promotion
The future live handoff should require, at minimum:
- one focused happy-path test proving a valid guard-stance package remains preview-eligible only after validation
- one invalid-package test proving missing required record/reference structure emits machine-readable diagnostics and blocks preview
- one widened/unsupported-package test proving the first-proof contract refuses broadened semantics instead of silently accepting them
- explicit diagnostic coverage for at least the required classes named in `validation-and-preview-workflow-requirements.md`
- preservation of package-state-aware preview/export refusal (`draft`, `invalid`, `deferred` cannot counterfeit success)

## Explicit non-goals
Do not let a future E3-F1 handoff authorize:
- edits outside `src/homebrew_authoring/**`, bounded GE08 tests, and bounded GE08 fixtures
- preview bridge/result-envelope implementation beyond what is needed to block claims honestly
- GE-07 UI/editor work
- plugin runtime or extension-system work
- Cargo dependency changes unless a blocker proves the documentary packet is incomplete
- public registry/distribution behavior

## Completion rule
This prebuild readiness closure is complete when it leaves no ambiguity about three facts:
- what GE08-E3-F1 is supposed to validate
- which write surface it is allowed to touch
- exactly why the packet still cannot be launched before GE08-E2-F1 gate acceptance
