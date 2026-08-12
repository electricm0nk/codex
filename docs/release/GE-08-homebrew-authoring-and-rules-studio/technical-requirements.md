---
title: GE-08 Technical Requirements
stc_id: STC-CODEX-GE-08
artifact_type: technical-requirements
status: draft
scope: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio
source_stc: ./README.md
source_artifacts:
  - ../../plans/spec-domains/GE-08-homebrew-authoring-and-rules-studio.md
  - ../GE-02-canonical-rules-model-and-content-packages/README.md
  - ../GE-02-canonical-rules-model-and-content-packages/technical-requirements.md
  - ../GE-04-rules-engine-and-explainability-core/README.md
  - ../GE-04-rules-engine-and-explainability-core/technical-requirements.md
  - ../GE-06-pilot-vertical-slice-pf1-human-fighter/README.md
  - ../../plans/spec-domains/GE-07-desktop-shell-and-modern-ux.md
---

# GE-08 Technical Requirements

## Objective
Define the normative requirements for safe, structured, inspectable homebrew authoring and package editing required to replace routine PCGen LST authoring for the first Codex proof path.

## Normative language
- **MUST** means required for GE-08 closure or for any downstream implementation that claims to satisfy GE-08.
- **SHOULD** means expected unless a later decision surface records a justified deviation.
- **MUST NOT** means prohibited for this STC.

## TR-08-001 — Authoring posture
Codex MUST treat GE-08 as the ordinary-homebrew authoring boundary between structured content editing and exceptional extension mechanisms.

The ordinary authoring path MUST be:
- data-first rather than arbitrary-code-first
- reviewable and diffable
- validation-backed
- provenance-bearing
- explainability-compatible with downstream GE-04 outputs
- portable enough for package import/export without hidden local state dependency

Codex MUST NOT treat GE-08 as:
- a license to clone PCGen LST syntax into a prettier shell
- a plugin-first rules-customization strategy
- a final editor-UX architecture decision
- proof that the pilot substrate is already fully proven

## TR-08-002 — Upstream dependency contract
GE-08 MUST consume rather than redefine upstream authority.

At minimum:
- GE-02 MUST remain authoritative for canonical package/model homes, stable IDs, provenance, diagnostics classes, and authoring-versus-compiled-IR boundaries
- GE-04 MUST remain authoritative for compute/explanation/diagnostic semantics and the preview outputs authoring is expected to surface
- GE-06 MUST remain authoritative for the current pilot-evidence ceiling and the first integrated proof path
- GE-07 MUST be treated as the planning-ready upstream authority for future editor/workbench surface expectations, command-boundary posture, and diagnostics/explanation visibility, but MUST NOT be treated as code authority for final UI/editor implementation decisions

## TR-08-003 — Minimum proof object
GE-08 MUST define the minimum homebrew object that proves authoring value before broad studio scope is authorized.

The first proof object MUST:
- be representable as canonical authored content rather than LST text
- exercise at least one structured rule change with downstream compute implications
- remain narrow enough to validate, preview, explain, and diff without broad ecosystem policy work
- be usable as a bounded acceptance case without requiring plugin execution

The exact first proof object is fixed by `artifacts/ge08-e1-minimum-proof-object-selection-2026-06-22.md` as a package-local feat-like authored object selected into the already-closed GE-06 Human bonus feat slot, carrying one bounded armor-class contribution and no broad formula/choice-set authoring burden.

Later GE-08 slices MUST preserve that bounded shape unless an accepted higher-order review explicitly changes it.

## TR-08-004 — Package authoring and manifest requirements
Codex MUST define authored package requirements capable of representing custom rules content without cloning PCC/LST mechanics.

The authored package model MUST support:
- a deterministic directory-backed source bundle rather than an opaque local-only store
- a package `schema_version` distinct from the authored `package_version`
- package identity and version/revision posture
- stable object identifiers aligned with GE-02
- dependency/include semantics where needed
- authored-content provenance and edit history fields appropriate to local package work
- validation status and diagnostic summaries
- explicit boundary between human-authored content and compiled runtime IR/cache

For the first bounded proof package, the logical source sections MUST include:
- `manifest`
- `objects/feats`
- `rules/effects`
- `rules/prerequisites` when prerequisite parity is claimed
- `metadata/provenance`
- `metadata/diagnostics`

GE-08 MAY leave exact stable-ID string syntax open, but it MUST NOT leave the first-proof package layout or lifecycle ambiguous after `artifacts/package-file-lifecycle-requirements.md` is accepted.

## TR-08-005 — Structured editing requirements
Codex MUST define authoring in terms of structured edit surfaces rather than free-form token editing.

The ordinary edit surface MUST support, at minimum:
- creating or editing a package manifest
- creating or editing a bounded authored object with stable ID and object kind
- adding or editing effect, prerequisite, formula, or choice records as structured fields
- linking authored records to provenance and diagnostics surfaces
- preserving unresolved or unsupported semantics as explicit debt rather than silently discarding them

GE-08 MUST NOT assume that a raw text editor alone satisfies structured authoring.

## TR-08-006 — Safe expression-authoring constraints
Codex MUST define a constrained expression-authoring posture for formulas and prerequisites.

The ordinary expression path MUST:
- preserve structured predicate or formula meaning strongly enough for validation and later evaluation
- inherit GE-02 expression-language decision criteria rather than bypassing them
- reject or diagnose unsupported, unsafe, or ambiguous constructs explicitly
- allow escalation to documented debt or plugin-exception review instead of silently widening the language

GE-08 MUST NOT treat arbitrary scripting as the default answer to authoring complexity.

## TR-08-007 — Validation and diagnostics requirements
Authored content MUST be validated before it is treated as safe, loadable, or previewable.

Validation MUST cover:
- manifest structure and required fields
- stable-ID uniqueness and reference resolution
- object-kind and field-shape validity
- effect/prerequisite/formula/choice-set structural validity
- provenance/source-map completeness appropriate to authored content
- unsupported, lossy, deferred, or intentionally ignored behavior reporting

Diagnostics MUST:
- remain machine-readable enough for tooling and human-readable enough for review
- preserve severity and claim-blocking posture
- distinguish authoring mistakes from engine/runtime defects where possible
- stay visible during preview rather than being buried

## TR-08-008 — Compile-preview workflow requirements
Codex MUST define an authoring-to-preview workflow that makes rule changes inspectable before broad product claims are made.

The workflow MUST include:
- authoring/edit phase
- validation phase with blocking versus non-blocking diagnostics
- compile or prepare-for-preview phase consistent with GE-02 compiled-IR boundaries
- headless preview or bounded pilot preview phase
- explanation-preview phase tied to downstream GE-04 explanation surfaces

Preview MUST NOT be satisfied by mock values or disconnected demo state.

## TR-08-009 — Explanation-preview requirements
A user authoring a rule MUST be able to inspect why the rule has the effect it does, within the bounded proof surface.

At minimum, GE-08 MUST require:
- visibility of authored-object identity and provenance
- visibility of whether the authored rule was accepted, downgraded, or blocked
- visibility of the derived output or blocked-path explanation that the authored rule influences
- visibility of relevant diagnostics when the rule cannot be previewed or explained

GE-08 MAY defer the exact UI presentation of explanations to GE-07, but MUST NOT defer the requirement that explanation truth remain available.

## TR-08-010 — Provenance, diffability, and portability requirements
Authored content MUST remain reviewable and portable.

Codex MUST preserve:
- stable IDs suitable for future re-edits and imports
- authored source lineage sufficient for debugging and review
- deterministic file organization or equivalent packaging structure
- diff-friendly serialization posture
- explicit import/export surfaces and portability constraints

The first-proof package serialization MUST additionally:
- normalize authored source deterministically across save/reload cycles
- keep stable file paths keyed by section plus stable ID or accepted equivalent rule
- exclude compiled/runtime cache material from the portable source bundle
- avoid machine-local-only noise such as absolute paths or save-time-only timestamp churn in authored source files

GE-08 MUST NOT allow authored content to become hostage to opaque local-only state.

## TR-08-011 — Package import/export lifecycle requirements
Codex MUST define the lifecycle for creating, editing, saving, importing, exporting, and reloading authored packages.

The lifecycle MUST state:
- what constitutes a valid local authored package
- what data is required before import/export is allowed
- how validation outcomes and diagnostics travel with the package
- how unresolved or unsupported semantics are signaled on exchange
- what is portable now versus explicitly deferred

For the first proof package, the lifecycle MUST also state that:
- package creation may produce a saveable `draft` shell, but only a validated package is preview-eligible or export-eligible
- save operations persist authored source even when invalid, while updating structured diagnostics and package validation state honestly
- load/reload revalidates authored source and MUST NOT treat compiled/runtime artifacts as source authority
- export is refused when required manifest fields, stable-ID integrity, reference integrity, provenance obligations, or claim-blocking validation conditions fail
- import preserves package-local stable IDs unless an explicit fork/new-package operation is chosen

## TR-08-012 — Plugin exception boundary
Plugins MAY exist later, but GE-08 MUST define them as exceptional rather than ordinary.

The ordinary homebrew path MUST be able to satisfy the first bounded proof cases without plugins.

A plugin exception path, if later allowed, MUST:
- be explicitly named as an exception
- state what authoring need could not be expressed safely in the ordinary path
- preserve diagnostics and provenance expectations
- avoid becoming a silent escape hatch for missing model/editor discipline

GE-08 MUST NOT authorize a plugin ABI, plugin sandbox, or arbitrary extension runtime from this STC alone.

## TR-08-013 — Acceptance-case requirements
GE-08 MUST name initial bounded homebrew acceptance cases strong enough to prove authoring value.

Those cases MUST include, at minimum:
- one simple authored rule that changes a bounded numeric or categorical outcome
- one validation-negative case that proves diagnostics are actionable
- one preview/explanation case proving that authored changes can be inspected
- one lifecycle case covering save/diff/import/export posture

The cases MAY remain documentary at this stage, but they MUST be concrete enough that a later bounded implementation handoff can test them without guessing.

## TR-08-014 — GE-07 boundary requirements
GE-08 MUST describe future product/editor surfaces without allowing those surfaces to own rules truth.

Because GE-07 currently exists only as a planning-ready source STC:
- editor flow, command boundary, shell layout, and final interaction model MUST remain non-code authority only until a later GE-07 readiness closure or execution handoff grounds them implementation-honestly
- GE-08 MAY define what surfaces must eventually exist, but MUST NOT grant final UI architecture authority
- any future code-authorizing handoff for product-visible authoring MUST either cite an accepted GE-07 authority surface or declare itself a bounded non-production spike

## TR-08-015 — Non-goal enforcement
GE-08 MUST keep the early authoring pass narrow.

This STC MUST NOT be used to justify:
- public package distribution or trust policy
- full visual-programming environments
- broad system-agnostic authoring abstractions
- ecosystem/plugin-marketplace design
- implementation breadth beyond the first bounded homebrew proof surface
