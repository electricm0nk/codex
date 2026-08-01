---
title: GE-08 Rules Studio Surface Definition
stc_id: STC-CODEX-GE-08
artifact_type: generated-artifact
status: draft
scope: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts
source_stc: ../README.md
related:
  - ../technical-requirements.md
  - ../technical-design.md
  - ./homebrew-authoring-surface-specification.md
  - ./validation-and-preview-workflow-requirements.md
  - ../../GE-07-desktop-shell-and-modern-ux/README.md
  - ../../GE-07-desktop-shell-and-modern-ux/artifacts/component-surface-inventory.md
  - ../../GE-07-desktop-shell-and-modern-ux/artifacts/ui-command-boundary-requirements.md
---

# Rules Studio Surface Definition

## Purpose
Define the first truthful product-visible authoring/workbench surface for GE-08 without allowing the editor to become the source of rules semantics, preview truth, or desktop-shell authority.

## Authority surfaces that govern this artifact first
Before any product-visible GE08-E5 implementation is proposed, the slice MUST cite both of these authority surfaces together:

1. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/README.md`
   - owns the authoring problem, bounded proof object, package/edit surface obligations, validation/preview/explanation posture, and plugin-exception boundary
2. `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/README.md`
   - owns the presentation-layer shell/workspace boundary, UI-consumer rule, and the prohibition on UI-owned rules semantics

The following supporting surfaces remain mandatory inputs, but they do not transfer authority to the workbench itself:
- `../technical-requirements.md` and `../technical-design.md` for GE-08 authoring, preview, explanation, and lifecycle obligations
- `./validation-and-preview-workflow-requirements.md` for the refusal-first package-state, preview-state, and blocked-claim rules
- `../../GE-07-desktop-shell-and-modern-ux/artifacts/component-surface-inventory.md` for the shell-side surface taxonomy
- `../../GE-07-desktop-shell-and-modern-ux/artifacts/ui-command-boundary-requirements.md` for the structured UI-to-core boundary and explicit UI prohibitions

## Current route truth
This artifact is documentary only.

It does not authorize:
- repo code changes by itself
- final desktop-shell architecture
- final command transport
- final interaction design
- any rule that lets GE-08 own engine semantics, preview truth, explanation truth, or provenance truth

GE-07 is a real upstream authority surface, but it is still not sufficient by itself to let GE-08 claim product-visible implementation authority. A later code-authorizing handoff for GE08-E5 must either:
- cite a later GE-07 readiness closure or stage-specific execution handoff that grounds the UI route honestly, or
- declare a bounded non-production spike with explicit repo paths, write scope, and verification commands

## Core rule
The first truthful rules-studio workbench is a presentation and orchestration surface over authored source, validation, preview, explanation, provenance, and lifecycle actions.

It is not a second rules engine.
It is not an explanation surrogate.
It is not a substitute for GE-07 shell doctrine.
It is not permission to bury blocked truth behind pleasant UI.

## Product-visible slice objective
The first product-visible GE08-E5 slice should let a user do one honest thing end to end:
- open the bounded first-proof homebrew package
- inspect or edit the package manifest and the fixed feat-like authored object
- validate the package
- run the bounded preview when the package is eligible
- inspect explanation, diagnostics, and provenance for that preview
- save or diff the package honestly

That objective is narrower than “build the rules studio.”
It is the smallest workbench surface that proves GE-08 can be product-visible without becoming semantically dishonest.

## Required workbench surfaces

| Surface | Primary job | Required backing truth | Must not own |
|---|---|---|---|
| Workbench frame / navigation | Keep orientation between authoring, diagnostics, preview, explanation, and provenance surfaces | GE-07 shell-frame discipline plus current package context and current claim state | generic product-shell sprawl or UI-owned route semantics |
| Package/library navigator | Select the bounded package and its contained authored records | real package identity, record inventory, and package state from GE-08 authored source | ad hoc package discovery rules or fake package state |
| Package manifest surface | View/edit package identity, dependency, version, and proof metadata | authored `manifest.yaml` truth and GE-02 package boundary rules | hidden local-only manifest fields or implicit dependency repair |
| Authored object editor | View/edit the bounded feat-like object and its linked effect/prerequisite records through structured fields | authored source records, stable IDs, object kinds, and reference links | free-form semantics that bypass structured authoring rules |
| Validation / problems panel | Show current diagnostics, blocked claims, and package-state posture | machine-readable validation payloads from GE-08 validation rules | prettifying failures by hiding, flattening, or downgrading them |
| Preview trigger and status surface | Make preview eligibility and preview outcome visible | real package state plus headless preview result envelope | previewing invalid/deferred content as though it were valid |
| Explanation surface | Show how the authored package changed the bounded downstream result or why it was blocked | GE-04-compatible explanation refs plus blocked-path visibility | frontend-authored explanation logic or silent explanation gaps |
| Provenance / source lineage surface | Show where the authored object and previewed effect came from | authored-source refs, provenance refs, and stable IDs | prose-only summaries that sever source linkage |
| Diff / review surface | Let the user inspect package changes before claiming progress | deterministic authored-source files and diagnostics changes | invisible mutation or opaque local-only state |
| Save / import / export affordances | Persist the package honestly and enforce lifecycle gates | GE-08 lifecycle state machine and export eligibility rules | exporting invalid/deferred packages or treating save as proof |

## First-slice interaction contract
The first product-visible slice MUST support the following bounded interaction loop:
1. Select the first-proof package and active authored object.
2. Show the current package state (`draft`, `valid`, `invalid`, or `deferred`).
3. Let the user edit only the bounded manifest/object/effect/prerequisite surfaces required by the first-proof case.
4. Run validation and surface machine-readable diagnostics with explicit blocked claims.
5. Permit preview only when the package is `valid` and no claim-blocking diagnostics remain.
6. Return the headless preview result envelope with explicit `success`, `blocked`, or `unsupported` status.
7. Let the user inspect explanation refs, provenance refs, and blocked-path reasons without losing package context.
8. Save drafts honestly; refuse export when the lifecycle rules say export is blocked.

## Cross-surface truth rules
The workbench MUST preserve these rules across every visible surface:
- the active package context must remain visible while moving into diagnostics, explanation, or provenance detail
- blocked-path results must stay visible; the UI must not go blank simply because preview failed
- diagnostics must preserve severity, category, and claim-blocking posture
- preview success must never be implied by button availability alone; the visible status must reflect the real package and preview states
- rules-library, source-package, and explanation views must cross-link back to the active authored change rather than becoming disconnected browsers
- no surface may rely on hand-written summary text as the only carrier for semantics that GE-02/GE-04/GE-08 already require as structured payloads

## What the first slice must expose from upstream truth
The workbench is only honest if it can consume and render these payload families without reinterpretation:
- package identity and package state
- authored record identities, object kinds, and stable references
- validation diagnostics with severity, class, subject reference, and blocked-claim posture
- preview result status and selected-slot resolution for the fixed Human bonus feat substitution
- bounded preview outputs, including armor-class result or blocked marker
- explanation refs aligned to GE-04 obligations
- provenance/source refs for authored package contribution
- lifecycle gate state for save, diff, import, export, and deferred semantics

## Explicit non-ownership rules
The GE08-E5 workbench MUST NOT:
- recompute authoritative rules values locally
- run prerequisite logic as a second semantic engine
- invent or rewrite explanation lineage in the frontend
- convert unsupported semantics into silent omissions
- treat UI-only convenience metadata as rules authority
- replace machine-readable diagnostics with prose-only banners
- define final shell layout, final command transport, or final UX doctrine independently of GE-07
- widen the first slice into general formula editing, broad studio breadth, registry policy, or plugin-runtime work

## Relation to GE-07 shell surfaces
GE08-E5 is not a parallel UI program. It is a GE-08-bounded consumer of GE-07 presentation doctrine.

The minimum mapping is:
- GE-07 shell frame / navigation -> hosts the authoring workbench without giving it global product authority
- GE-07 validation / problems surface -> carries GE-08 authoring diagnostics and blocked claims
- GE-07 explanation surface -> carries GE-08 preview/explanation refs without inventing new semantics
- GE-07 source package view -> carries GE-08 authored-source and provenance inspection
- GE-07 UI command boundary -> carries the structured requests/responses the workbench needs from the headless authoring/preview substrate

If a future GE08-E5 proposal cannot point to those GE-07 surfaces and explain how it consumes them, it is not ready.

## Minimum honest first-proof scope
The first product-visible workbench slice remains bounded to the GE08-E1 proof case:
- one authored `SourcePackage`
- one authored feat-like object
- one authored effect
- optional one authored prerequisite
- one bounded Human bonus feat substitution in the inherited GE-06 pilot case
- one bounded armor-class contribution path

It does not authorize:
- broad content-library editing
- arbitrary formula composition
- multiple package orchestration
- plugin-default extension flows
- public sharing/distribution flows
- cross-system breadth beyond the fixed proof case

## Preconditions for any later code-authorizing GE08-E5 handoff
A later implementation handoff for the product-visible workbench must, at minimum:
- cite this artifact plus the GE-08 README and GE-07 README together
- name the exact GE-07 surface or later readiness closure/execution handoff that grounds the UI route
- name the exact repo paths and write scope
- state whether the slice is non-production spike, internal pilot surface, or production-facing implementation
- consume the headless preview/explanation contract already defined by GE-08 rather than redefining it locally
- preserve the refusal-first package-state and preview-state rules from `validation-and-preview-workflow-requirements.md`
- state explicit verification commands and at least one success-path plus one blocked-path proof

## Non-goals for this artifact
This artifact does not decide:
- the final window/chrome/layout design
- the final widget/component library
- the final transport mechanism between shell and core
- the exact serialized UI payload schema names
- package registry/distribution UX
- plugin-exception review UX beyond showing the blocked/deferred truth honestly

## Completion rule
This artifact is complete when future GE08-E5 readiness or handoff work no longer has to guess:
- which authority surfaces control the product-visible workbench
- which surfaces the first editor/workbench must expose
- which payload families the UI must consume from headless truth
- which semantic responsibilities the workbench is forbidden to own
- how narrow the first product-visible slice must remain to stay honest
