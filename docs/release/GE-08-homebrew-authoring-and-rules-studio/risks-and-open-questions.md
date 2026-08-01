---
title: GE-08 Risks and Open Questions
stc_id: STC-CODEX-GE-08
artifact_type: risks-and-open-questions
status: draft
scope: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio
source_stc: ./README.md
---

# GE-08 Risks and Open Questions

## Objective
Quarantine unresolved GE-08 questions so the source STC stays honest instead of smearing uncertainty through every requirement.

## Resolved this pass
- GE-08 is being created in a constrained early posture rather than waiting for a perfect future state.
- Plugins are explicitly treated as exceptional rather than ordinary.
- GE-07 now exists as a planning-ready source STC and is recorded as a real upstream authority surface without being mistaken for GE-08 code authority.
- `artifacts/ge08-e1-minimum-proof-object-selection-2026-06-22.md` closes the minimum proof-object selection as a feat-like authored package case bound to one GE-06 pilot variant.
- `artifacts/plugin-exception-boundary.md` now defines the planning-stage threshold for genuine plugin exceptions, the non-qualifying cases that must stay in structured authoring, and the research/ADR route required before any plugin-related implementation work could begin.

## Closed by GE08-E1

### CQ-08-001 — Minimum proof object
The first truthful proof case is no longer open. GE08-E1 selected a package-local feat-like authored object that substitutes for the GE-06 Human bonus feat `Dodge` and contributes one bounded armor-class modifier through existing GE-02 homes and GE-04/GE-06 preview obligations.

## Open questions

### OQ-08-002 — Which expression-authoring affordances are sufficient first?
Open tension:
- overly weak affordances make ordinary homebrew impractical
- overly broad affordances push the system toward arbitrary scripting

Need later evidence on:
- whether simple structured forms cover the first proof cases
- where unsupported semantics should become explicit debt versus plugin exceptions

### OQ-08-003 — How much GE-06 proof is enough before authoring code starts?
The current pilot lane is still bounded. GE-08 should not assume full end-to-end authoring trust before the pilot substrate proves enough preview/explanation reality.

Need later evidence on:
- which pilot outputs are stable enough to receive authored changes
- whether preview can remain headless before any GE-07 product-visible route is implementation-grounded

### OQ-08-004 — How much GE-07 grounding is needed before product-visible authoring/editor work begins?
The GE-07 source STC now exists, but only in planning-ready form.

Therefore unresolved:
- final command/API boundary for editor <-> core
- product-visible authoring flow
- shell/panel/information architecture details for explanation and diagnostics during editing

### OQ-08-005 — What is the exchange boundary for import/export and sharing?
This STC defines package lifecycle requirements but does not settle:
- trust/distribution policy
- compatibility/version negotiation for broad sharing
- public registry behavior
- signed package or provenance-hardening policy

### OQ-08-006 — Which future cases might actually prove the plugin threshold is real?
The planning-stage policy boundary is now defined by:
- `artifacts/plugin-exception-boundary.md`
- `../../doctrine/decisions/ADR-0001-plugin-exception-path.md`

What remains open is not the governance route but the future evidence question:
- whether any later accepted GE-02/GE-08 slice will demonstrate semantics that cannot be modeled safely through structured content or constrained expressions
- whether any future approved scope will require a bounded host-capability bridge outside the ordinary portable package path

Until such evidence exists, no GE-08 plugin runtime work is justified.

## Hard blockers for a future code-authorizing handoff
Stop and block if any future GE-08 execution attempt lacks:
- exact proof object
- exact repo paths and write scope
- explicit dependency baseline against then-current GE-06 and GE-07 truth
- explicit verification commands and expected receipts
- explicit statement of whether the slice is headless-only or product-visible

## Forbidden assumptions
- that unresolved GE-07 implementation details can be improvised later without changing the authoring architecture
- that a broad rules studio must be built before any honest value can be proved
- that ordinary customization should require plugins
- that preview/explanation can be mocked instead of grounded in real downstream outputs
