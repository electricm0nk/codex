# GE-07 Technical Requirements

## Purpose
This document defines the normative requirements for the Codex desktop shell and modern UX. It specifies what the shell must surface, what data and explanation contracts it must consume, and what it is forbidden to do.

## Problem statement
Codex needs a product-visible desktop shell that lets a real user inspect and operate the pilot character path. Without that shell, the system remains invisible. With an ungrounded shell, the project risks counterfeit progress: beautiful surfaces over unproven or hidden domain behavior.

GE-07 exists to prevent that failure. The shell must make real domain truth legible without inheriting responsibility for rules semantics.

## Current-state facts
- GE-07 now has a planning-ready source STC, GE07-E1 documentary spike artifacts, a repaired GE07-E1 execution-readiness closure, and an active GE07-E1 stage-specific code-authorizing handoff in `awaiting-todd-merge` state after one bounded coding pass; GE07-E2 through GE07-E6 remain documentary/readiness or risk surfaces, not completed implementation.
- GE-06 defines the integrated pilot-proof boundary and now has an explicit viability/domain-confidence decision fixing the current downstream posture at `computed-but-not-oracle-checked` rather than product-visible proof.
- GE-03 and GE-04 define the importer-diagnostics/provenance and explainability/computation obligations the UI must expose.
- The live Codex repo re-observed on 2026-06-24 still lacks `apps/desktop/**` on `origin/develop`, but the feature branch `ge07-e1-desktop-shell-scaffold` at `48892249d5573927bf23a7e47a6d7d6a742da664` now carries the bounded scaffold result.
- The minimum pilot workspace truth burden is now grounded over live deterministic pilot data: grouped value sections, current selections, explanation hooks, and computed-vs-blocked route framing are all explicit documentary truth.
- The raw pilot rules/source-package carriers are now grounded in the live tree: chosen rule/object IDs live in the deterministic character-input fixture, `source_package_id` is preserved through the headless receipt and selected parity-dimension carrier, and the golden oracle fixture expands lineage into system/package/campaign/game-mode fields.
- The live tree still has no dedicated UI-consumer inspection projection for bounded rules browsing or source-package detail; current E5 truth is carrier-level, not yet a shell-ready browse contract.
- The live runtime observed on 2026-06-22 has `cargo`, `rustc`, `node`, and `npm`, which clears the crude toolchain-missing blocker but does not prove Tauri packaging/signing readiness.
- GE07-E6 now has a grounded platform-risk receipt showing that the current Ubuntu 24.04 host lacks Linux Tauri system dependencies, Tauri CLI, Windows cross-build tooling, and Apple signing tooling; packaging questions are now visible, but ship readiness remains unearned.
- GE07-E4 now has a grounded visibility receipt and readiness closure proving that the live tree already carries computed explanation detail plus blocked-route, validation, and importer diagnostic payloads, but still no merged consumer bridge, bounded shell-ready projection, or grounded invalid-choice reason lane.
- The roadmap and reference architecture ground Tauri 2 plus a TypeScript UI as the current preferred direction, but exact framework/binding and packaging decisions remain open.
- The quality-gate policy forbids UI-only proof and requires the UI to front proven domain behavior.

## Normative requirements

### R1. Shell runtime boundary
1. The first GE-07 implementation surface must be a local-first desktop shell intended for Linux, Windows, and macOS.
2. The shell architecture must assume Tauri 2 as the current preferred runtime unless a later accepted ADR explicitly changes that decision.
3. The shell must be designed as a consumer over a headless Codex substrate. It must not collapse UI and domain responsibilities into one undifferentiated layer.
4. The shell must preserve an honest path for future desktop packaging without requiring cloud services, user accounts, or browser-hosted architecture for the pilot.

### R2. UI-to-core command boundary
1. All domain reads and mutations used by the UI must cross an explicit command or API boundary.
2. The UI boundary must return structured payloads for:
   - character snapshot/state
   - derived-value explanations
   - validation/problems
   - importer diagnostics and unsupported-token visibility
   - rules-library browsing
   - source-package inspection
3. The UI must not derive authoritative rules answers locally from copied formula logic, duplicate prerequisite evaluation, or ad hoc value recomputation.
4. The command boundary may remain transport-agnostic at this planning stage, but the future handoff must settle whether it is Tauri commands, an internal RPC layer, a service boundary, or another explicit mechanism.
5. The UI must be able to reference provenance identifiers or source pointers returned by the core rather than synthesizing fake source context.

See also: `artifacts/ui-command-boundary-requirements.md`.

### R3. Pilot data truth
1. The pilot character workspace must render real outputs from the integrated pilot path grounded by GE-06.
2. The shell must not treat mock state, screenshots, or manually fabricated JSON as proof of product behavior.
3. The pilot shell must be able to display imported/computed character state, derived combat values, selected choices, explanation data, validation state, and import diagnostics from real domain payloads.
4. If the shell cannot obtain real domain outputs for a future slice, that slice is a spike or placeholder only and must not be presented as product truth.

### R4. Explanation visibility
1. The pilot shell must present explanation affordances for derived values such as AC, BAB, saves, skill modifiers, and similar surfaced character numbers.
2. Explanation views must be consumer-side renderings of upstream explanation payloads, not UI-owned explanation logic.
3. Explanation surfaces must show source contribution context, including modifiers, prerequisites, or provenance references when those are available upstream.
4. Unavailable or invalid choices must be inspectable with a visible reason path rather than hidden or silently disabled.

### R5. Diagnostics visibility
1. Import diagnostics, unsupported-token warnings, and validation problems must remain visible in the shell rather than being buried behind developer-only surfaces.
2. The product may distinguish ordinary-user and developer-oriented detail levels, but it must not hide the fact that a warning, unsupported semantic, or validation failure exists.
3. Problems and diagnostics surfaces must preserve their link to upstream importer/validation outputs and known-gap language rather than replacing them with vague UI prose.

### R6. Required user-facing surfaces
The source STC requires the first GE-07 shell to define and later implement bounded surfaces for:
1. pilot character workspace
2. explanation drawer or panel
3. validation/problems panel
4. import diagnostics view
5. rules library pilot view
6. source package view
7. navigation shell framing those surfaces coherently

The exact scope and duties of each surface are defined in:
- `artifacts/pilot-shell-architecture-requirements.md`
- `artifacts/ui-information-architecture-requirements.md`
- `artifacts/component-surface-inventory.md`

### R7. Local state boundary
1. UI-local state is allowed only for presentation concerns such as route, panel expansion, selection focus, sorting, filtering, draft form state, or cache/in-flight markers.
2. Authoritative rules state, explanations, validation results, and provenance must remain owned by the core domain boundary.
3. If the shell later introduces optimistic interactions or edits, those must still reconcile against authoritative core responses rather than becoming a second rules engine in the UI.
4. Any local storage or offline cache introduced by later implementation work must preserve the distinction between rebuildable cache and source-of-truth data.

### R8. Information architecture requirements
1. The shell must let a user move from the pilot character overview into explanations, diagnostics, rules data, and source-package inspection without losing context.
2. The shell must make the relationship between visible values, their explanations, and their source lineage discoverable.
3. The shell must not present rules-library or source-package surfaces as a detached browser unrelated to the active character path; the pilot needs clear cross-links back into the live build.
4. The IA must support both “what is my value?” and “why is it this value?” workflows.

See: `artifacts/ui-information-architecture-requirements.md` and `artifacts/pilot-ux-flow-requirements.md`.

### R9. Packaging and OS constraints
1. GE-07 must preserve a plausible packaging path for Linux, Windows, and macOS.
2. Cross-platform constraints such as signing, updater posture, file paths, SQLite bundling, webview/runtime assumptions, and OS integration risks must be explicitly recorded before code-authorizing shell work.
3. Platform-risk discovery may begin as bounded documentary or spike work, but unresolved risks must remain visible.
4. No GE-07 artifact may claim ship readiness until a real shell slice exists and each claimed platform has a platform-specific build/signing receipt grounded against live tooling and runner truth.

See: `artifacts/cross-platform-build-constraint-questions.md` and `artifacts/ge07-e6-platform-risk-receipt-2026-06-22.md`.

### R10. Documentary outputs required by this source STC
The GE-07 spec domain obligated the source STC to produce concrete documentary outputs. Those outputs now live at:
- `artifacts/pilot-shell-architecture-requirements.md`
- `artifacts/ui-information-architecture-requirements.md`
- `artifacts/pilot-ux-flow-requirements.md`
- `artifacts/component-surface-inventory.md`
- `artifacts/ui-command-boundary-requirements.md`
- `artifacts/cross-platform-build-constraint-questions.md`

Future sessions must treat those artifacts as part of the GE-07 authoritative bundle, not as optional side notes.

## Subsystem ownership notes
- GE-03 owns importer diagnostics, unsupported-token visibility, and provenance/source-map truth.
- GE-04 owns computed values, prerequisite evaluation, explanation payloads, and invalid-choice truth.
- GE-05 owns parity-report and known-gap comparison truth when such data is exposed in the UI.
- GE-06 owns the integrated pilot-case contract that the first shell must consume.
- GE-07 owns only the shell, UX surface requirements, and command-boundary expectations over those upstream truths.

## Non-goals
- final public-launch design system
- comprehensive Pathfinder browsing beyond the pilot needs
- cloud synchronization, accounts, or collaboration
- mobile or web deployment
- UI-owned rules evaluation, formula execution, or prerequisite solving
- broad packaging automation claims before real runtime proof exists
