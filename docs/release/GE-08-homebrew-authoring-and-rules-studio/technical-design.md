---
title: GE-08 Technical Design
type: technical-design
stc_id: STC-CODEX-GE-08
status: draft
scope: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio
source_stc: ./README.md
---

# GE-08 Technical Design

## Purpose
This design operationalizes the GE-08 source STC without collapsing it into implementation code. It describes the intended authoring pipeline and subsystem boundaries for safe homebrew authoring in a constrained early posture.

## Design posture
- architecture style: `structured authoring -> refusal-first validation -> headless preview bridge -> GE-04 explanation obligations`
- authoring posture: `data-first ordinary customization, plugin-exception only`
- evidence posture: `no usability or correctness claims without validation, preview, and explanation receipts`
- dependency posture: `consume GE-02 model truth, GE-04 compute/explanation truth, GE-06 pilot truth, and GE-07 presentation truth without collapsing them into one surface`
- presentation posture: `preview/explanation must be complete before any GE-07 surface renders them; UI is a consumer, not an authority`

## Boundary conditions
- GE-08 is not a final editor implementation plan.
- GE-08 is not a plugin ABI design.
- GE-08 must stay useful while GE-07 is only planning-ready and not yet implementation-grounded.
- GE-08 must preserve a path for later UI/editor work without pretending that UI flow is already settled.

## Conceptual pipeline
1. **Authored package creation/editing**
   - user creates or edits a bounded package/object through structured fields
2. **Structural validation**
   - package shape, references, expression structure, and provenance completeness are checked
3. **Preview preparation / compile boundary**
   - authored source is normalized toward the GE-02 compiled-runtime boundary without losing source authority
4. **Bounded preview execution**
   - downstream GE-04/GE-06-compatible preview path produces derived outputs or blocking diagnostics
5. **Explanation/diagnostic review**
   - the user inspects what changed, why it changed, and why anything failed
6. **Lifecycle actions**
   - save, diff, import, export, and later reuse of the package remain explicit and portable

## Subsystem boundaries

### 1. Authoring package surface
Responsibilities:
- package identity/version posture
- object creation/editing
- stable-ID assignment policy inherited from GE-02
- local provenance/edit metadata sufficient for review

Must not own:
- runtime engine semantics
- final desktop-shell UX
- plugin ABI

### 2. Expression authoring boundary
Responsibilities:
- structured representation of formulas/prerequisites
- validation of allowed forms
- downgrade/escalation path for unsupported semantics

Must not own:
- arbitrary code execution
- silent widening of the expression language

### 3. Validation and diagnostics boundary
Responsibilities:
- structural validation
- reference and field checks
- machine-readable diagnostics
- claim-blocking posture before preview/load

Must not own:
- explanation of final runtime outcomes beyond what downstream preview returns

### 4. Preview bridge
Responsibilities:
- connect authored content to the bounded GE08-E1 preview path only
- preserve authored-source authority while preparing GE-04-compatible preview inputs
- carry forward the exact GE08-E1 proof binding: inherited GE-06 pilot case plus Human bonus feat substitution
- emit a headless result envelope containing selected-slot resolution, bounded armor-class output or blocked marker, diagnostics, provenance/source refs, explanation refs, and oracle-dimension status
- expose success/failure state back to the authoring surface without waiting for any GE-07 UI layer

Must not own:
- product UI policy
- final pilot viability claims
- alternate proof cases beyond the fixed GE08-E1 object

### 5. Explanation preview boundary
Responsibilities:
- surface contribution/explanation truth for authored changes
- preserve provenance and diagnostics linkage
- show blocked paths when authoring changes are invalid or unsupported
- preserve the semantic path Human bonus feat slot -> authored feat -> authored effect -> armor-class derived value
- preserve the minimum GE-04 graph obligations for node kinds, edge kinds, and diagnostic visibility needed by the first proof object

Must not own:
- broad UI presentation design beyond conceptual requirements
- a UI-only explanation surrogate that bypasses headless explanation truth

### 6. Lifecycle and exchange boundary
Responsibilities:
- create/save/reload/diff/import/export package behavior
- portability rules
- unresolved/unsupported signaling on exchange

Must not own:
- public registry policy
- trust/distribution model

#### Concrete first-proof package substrate
The first honest package substrate is a deterministic directory-backed YAML bundle aligned to GE-02 logical sections:

```text
<package-root>/
  manifest.yaml
  objects/
    feats/
      <feat-stable-id>.yaml
  rules/
    effects/
      <effect-stable-id>.yaml
    prerequisites/
      <prerequisite-stable-id>.yaml   # optional
  metadata/
    provenance.yaml
    diagnostics.yaml
```

This boundary exists so authored source remains:
- human-reviewable
- stable under diff/reload
- explicit about package dependency and provenance
- separate from any compiled/runtime cache the preview bridge may later derive

#### Lifecycle state model
The lifecycle boundary should treat package state as a small explicit state machine:
- `draft` — saveable authored shell or work-in-progress source; not previewable or exportable
- `valid` — required files, references, provenance, and diagnostics posture all satisfy the first-proof package contract
- `invalid` — package loads only with claim-blocking diagnostics; saveable locally but not exportable
- `deferred` — package remains structurally known but still carries accepted unsupported semantics; documentary only, not proof-claimable

`manifest.yaml` holds the normalized package state summary and `metadata/diagnostics.yaml` holds the detailed machine-readable reasons.

## First-proof bridge payloads
The first proof bridge is a headless contract, not a UI storyboard.

### Required input payload
The bridge consumes:
- the deterministic authored package bundle (`manifest.yaml`, authored object/rule files, `metadata/provenance.yaml`, `metadata/diagnostics.yaml`)
- the fixed GE08-E1 proof binding for case `pf1-crb-human-fighter-level1-homebrew-feat-proof`
- the inherited GE-06 base case identity `pf1-crb-human-fighter-level1`

The bridge does not consume:
- product-visible editor state as rules authority
- shell/window/workbench metadata from GE-07
- broad feature selections outside the fixed Human bonus feat substitution

### Required output payload
The bridge returns a headless envelope containing at minimum:
- package identity and package state
- preview status (`success`, `blocked`, or `unsupported`)
- exact Human bonus feat slot resolution (`remove dodge`, `add homebrew_guard_stance`)
- bounded armor-class preview output or an explicit blocked marker
- diagnostics with claim-blocking posture
- provenance/source refs
- explanation refs aligned to GE-04 obligations
- oracle-dimension status for the previewed dimension set

### Headless-first discipline
Any future GE-07 editor/workbench surface must consume this bridge contract rather than redefine it. If a UI cannot render blocked-path explanation or claim-blocking diagnostics, the UI is incomplete; the bridge is not allowed to simplify the truth to accommodate it.

### Future GE07-backed workbench contract
The later GE08-E5 product-visible workbench is a consumer/orchestrator over authored source, validation, preview, explanation, provenance, and lifecycle actions.

That future surface must therefore:
- consume the package state, diagnostics, preview-status, explanation-ref, and provenance-ref families already defined by GE-08 rather than inventing UI-local semantic substitutes
- inherit GE-07 shell and command-boundary posture for navigation, cross-linking, and structured payload transport
- keep the active authored package/object context visible while the user moves into diagnostics, preview, explanation, provenance, diff, or lifecycle detail
- preserve blocked-path truth instead of blanking or softening failures for product cleanliness

That future surface must not:
- recompute rules or prerequisite semantics locally
- treat UI-only metadata as rules authority
- outrun GE-07 by claiming final shell/layout/transport authority from GE-08 alone
- widen the first slice beyond the fixed GE08-E1 proof object and bounded armor-class contribution path

### 7. Plugin exception boundary
Responsibilities:
- define what ordinary customization must achieve without plugins
- define when an exception review is warranted
- preserve evidence and diagnostic posture when exceptions are discussed

Must not own:
- normal authoring flow
- a broad extension ecosystem plan

## Early-posture routing rule
Because GE-07 now exists only as a planning-ready source STC and GE-06 evidence is still bounded, GE-08 should currently drive toward one of two future routes only:
- a narrow non-UI implementation slice proving a minimum authoring object through headless validation/preview, or
- a later product-visible authoring/editor slice only after GE-07 is grounded through a separate readiness closure or execution-handoff chain

## Bounded next implementation route
This design remains documentary-only, but the bridge contract is now concrete enough that a later execution handoff can be specific instead of speculative.

The narrowest honest first implementation lane after this artifact is a headless preview-and-explanation bridge slice, not another package-lifecycle restatement.

That later handoff should, at minimum:
- add a new `homebrew_authoring` module surfaced from `/home/ubuntu/workspace/repos/codex/src/lib.rs`
- define a headless bridge entrypoint under `/home/ubuntu/workspace/repos/codex/src/homebrew_authoring/` that consumes the deterministic authored package bundle plus the fixed GE08-E1 proof binding
- emit a result envelope that distinguishes `success`, `blocked`, and `unsupported` preview states while preserving diagnostics, provenance/source refs, and explanation refs
- prove the Human bonus feat substitution path into the bounded armor-class preview output without widening into broader editor or plugin behavior
- add bounded GE-08 tests plus a fixture package under `/home/ubuntu/workspace/repos/codex/tests/fixtures/ge08/`

Verification for that later lane should be anchored by explicit headless bridge tests, minimally:
- one happy-path test proving the authored feat survives load -> validate -> prepare -> preview -> explain for `pf1-crb-human-fighter-level1-homebrew-feat-proof`
- one blocked-path test proving malformed or widened authored content returns machine-readable diagnostics and blocked claims instead of counterfeit preview success

Non-goals for that future lane:
- no GE-07 product/editor work
- no plugin runtime
- no broad formula language surface
- no public registry/distribution work
- no widening beyond the fixed first proof object and its bounded armor-class path

## Design non-goals
This design does not choose final desktop-shell structure, command transport, serialized syntax, plugin runtime, public sharing model, or final interaction ergonomics.

## Design review triggers
Reopen this design if:
- GE-07 source STC evolves materially and changes authoring/editor surface assumptions
- GE-06 proves or disproves key preview/explanation expectations
- GE-02 evolves the canonical authored-package boundary
- GE-04 changes required diagnostic or explanation payloads
- a bounded spike demonstrates that the minimum proof object needs to be narrower or broader than assumed here
