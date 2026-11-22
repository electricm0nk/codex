# GE-07 Technical Design

## Design intent
The GE-07 shell should feel like a modern rules workbench without becoming the new semantic center of the system. The design response is therefore a narrow composition rule:

headless Codex domain surfaces -> explicit command boundary -> desktop shell orchestration -> focused product surfaces

## Reference architecture response
The upstream reference architecture already grounds this stack posture:
- Tauri 2 desktop shell
- TypeScript UI
- Rust-backed core domain and import surfaces
- local-first storage/runtime posture

GE-07 adopts that as the current design response, but preserves open decisions where the architecture is not yet proven.

## Primary design rule
The shell may compose, route, filter, and present. It may not authoritatively compute rules truth.

That means:
- the shell owns navigation, layout, selection focus, user interaction flow, and presentation state
- the shell may request read models tailored for UX needs
- the shell may not reimplement formulas, prerequisite logic, explanation derivation, or import-diagnostic synthesis

## Proposed composition model
### 1. Shell frame
A future implementation should provide a persistent shell frame that anchors:
- primary navigation
- current pilot character context
- diagnostics/problem visibility
- access to rules-library and source-package inspection surfaces

### 2. Character workspace
The main workspace should be centered on the pilot character path rather than a generic dashboard. The first proof is whether a user can inspect and understand the Human Fighter path, not whether the shell can host arbitrary widgets.

### 3. Detail sidecars
Explanation, diagnostics, and provenance details should appear as inspectable sidecars, drawers, tabs, or adjacent panes rather than forcing the user to abandon the current context.

### 4. Bounded rules browsing
Rules-library and source-package inspection surfaces should be available, but bounded to the pilot package/domain during the first slice.

## Data-flow posture
1. shell requests a character or rules payload through the explicit boundary
2. core returns structured domain/read models plus diagnostics/explanation/provenance payloads
3. shell renders those payloads and stores only presentation-local state
4. user inspection actions request deeper detail rather than causing local semantic recomputation

## Command-boundary shape
At this stage the transport is intentionally undecided, but the design requires the boundary to support command/read-model shapes roughly like:
- load pilot case / list available pilot cases
- get current character snapshot
- explain value
- list available choices and invalid reasons
- fetch validation/problems
- fetch import diagnostics / unsupported-token notices
- browse rules-library entities
- inspect source-package metadata/provenance

Those are product obligations, not implementation signatures.

## State ownership model
### UI-owned state
- selected route/view
- open/closed panel state
- focused value / selected rule item
- sort/filter preferences
- temporary draft input and in-flight interaction state

### Core-owned state
- authoritative character state
- derived values
- explanation data
- validation results
- importer diagnostics
- provenance/source lineage
- parity/known-gap results when exposed

## Storage posture
The shell may rely on future local SQLite/cache surfaces identified by the reference architecture, but GE-07 does not yet authorize storage-schema implementation. At the design level the rule is simple:
- user-editable source-of-truth data must remain distinct from rebuildable caches
- the shell must not quietly turn caches into hidden semantic authority

## Surface design posture
The first shell should optimize for legibility, not maximal feature count.

That means the design response favors:
- one coherent pilot workspace
- clear explanations for values and invalid choices
- visible diagnostics and warnings
- rules/library/source inspection tied back to the active character path
- restrained navigation breadth

It explicitly rejects:
- giant menu breadth before pilot truth exists
- a component zoo detached from real domain payloads
- a fake demo shell that only proves visual polish

## Packaging posture
Cross-platform packaging remains a required design concern, but not a solved implementation fact. The design response is to keep packaging risks visible as a tracked question set rather than bury them under UI enthusiasm.

## Design review triggers
Reopen this design if any of the following changes:
- GE-06 pilot truth changes the minimum user journey or domain payload shape
- an ADR changes Tauri, frontend framework, or component-system posture
- GE-03/GE-04 change the diagnostic or explanation contracts exposed to the shell
- cross-platform packaging research reveals a platform-specific blocker that changes scope
- a future handoff proposes local UI semantics that violate the shell/non-semantic boundary
