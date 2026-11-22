# GE-07 Epic Breakdown

## Purpose
This breakdown decomposes GE-07 into future bounded work so the desktop-shell program does not become a single vague UI sprint.

## Proposed downstream slices

### GE07-E1 — Shell scaffold and runtime boundary spike
Goal:
- prove the smallest non-production desktop shell scaffold and runtime boundary shape without claiming product truth

Preconditions:
- explicit spike posture or GE-06 viability gate decision
- repo paths and toolchain grounded

Likely outputs:
- `artifacts/ge07-e1-shell-scaffold-receipt-2026-06-22.md`
- `artifacts/ge07-e1-runtime-boundary-adr-input-2026-06-22.md`
- no counterfeit product claim

Current grounded state:
- the documentary E1 receipt/ADR pair now exists and proves the smallest additive shell shape plus the first read-only runtime-boundary answer
- `artifacts/ge07-e1-execution-readiness-closure-2026-06-22.md` now exists and records GE07-E1 as the repaired first code-ready scaffold lane
- `artifacts/ge07-e1-execution-handoff-2026-06-22.md` now exists and has produced verified branch-ready scaffold evidence on `ge07-e1-desktop-shell-scaffold` at `48892249d5573927bf23a7e47a6d7d6a742da664`
- the root GE-07 route surface at `execution-handoff.md` now points at the GE07-E1 pair and is `awaiting-todd-merge`
- no merge receipt exists yet because `origin/develop` does not yet contain that scaffold

### GE07-E2 — UI-to-core command boundary contract
Goal:
- turn the GE-07 command-boundary requirements into a bounded implementation-ready contract over real domain payloads

Preconditions:
- upstream GE-03/GE-04/GE-06 payload expectations grounded
- exact write scope in repo known

Likely outputs:
- stage-specific readiness closure
- code-authorizing handoff for boundary adapter work only

Current grounded state:
- `artifacts/ge07-e2-execution-readiness-closure-2026-06-22.md` now exists and records the exact stop condition for this lane
- the exact prerequisite scaffold path set is known from GE07-E1, but those paths are still absent on `origin/develop`
- the narrow rules-core consumer bridge this lane would otherwise consume is still represented by the awaiting-Todd-launch GE06-E4-F1 handoff rather than merged repo truth
- therefore no GE07-E2 execution handoff exists yet

### GE07-E3 — Pilot character workspace shell
Goal:
- present the real pilot character path with value groups, current selections, and route framing over real domain outputs

Preconditions:
- boundary contract exists
- minimum pilot workspace state defined

Likely outputs:
- narrow shell/workspace handoff
- UI-truth verification receipts using real pilot data

Current grounded state:
- `artifacts/ge07-e3-execution-readiness-closure-2026-06-22.md` now exists and records the exact stop condition for this lane
- `artifacts/ge07-e3-ui-truth-verification-receipt-2026-06-22.md` now proves the minimum pilot workspace truth burden over live deterministic pilot data, including computed and blocked route examples
- the minimum workspace state is now explicit as grouped values plus current selections plus route framing over the real receipt lane
- the shell subtree is still absent on `origin/develop`, and the upstream rules-core view-model bridge remains the awaiting-Todd-launch GE06-E4-F1 handoff rather than merged repo truth
- therefore no GE07-E3 execution handoff exists yet

### GE07-E4 — Explanation and diagnostics surfaces
Goal:
- expose derived-value explanations, invalid-choice reasons, validation problems, and importer diagnostics without hiding them

Preconditions:
- pilot workspace exists
- upstream explanation/diagnostic payloads are grounded

Likely outputs:
- bounded handoff for explanation drawer/panel and diagnostics surface
- receipts proving explanations and warnings remain visible

Current grounded state:
- `artifacts/ge07-e4-execution-readiness-closure-2026-06-22.md` now exists and records the exact stop condition for this lane
- `artifacts/ge07-e4-explanation-diagnostics-visibility-receipt-2026-06-22.md` now proves the current explanation/diagnostics truth burden over live Codex surfaces, including computed explanation detail plus blocked-route, validation, and importer diagnostics
- the live repo still has no shell subtree on `origin/develop`, and the upstream GE06-E4-F1 consumer bridge remains the awaiting-Todd-launch handoff rather than merged repo truth
- the live rules-core still has no grounded invalid-choice/prerequisite-reason payload, so no honest code-authorizing GE07-E4 execution handoff exists yet

### GE07-E5 — Rules library and source-package pilot views
Goal:
- provide bounded browsing/inspection of the pilot rules surfaces and source-package lineage

Preconditions:
- pilot workspace and boundary contract exist
- provenance/source-package payload shape is grounded

Likely outputs:
- narrow library/source inspection handoff
- receipts proving cross-links back into the active character path

Current grounded state:
- `artifacts/ge07-e5-execution-readiness-closure-2026-06-22.md` now exists and records the exact stop condition for this lane
- the raw pilot rule identities are grounded in the deterministic character-input fixture, and current source-package lineage carriers are grounded across the headless receipt, selected parity dimensions, and golden oracle fixture
- the active-character cross-link target is already defined by GE07-E3, so E5's burden is bounded to inspection surfaces rather than detached content browsing
- the shell subtree is still absent on `origin/develop`, the upstream GE06-E4-F1 workspace/view-model bridge remains the awaiting-Todd-launch handoff rather than merged repo truth, and no dedicated inspection projection exists yet
- therefore no GE07-E5 execution handoff exists yet

### GE07-E6 — Cross-platform packaging and ship-readiness spike
Goal:
- discover packaging/signing/runtime blockers for Linux, Windows, and macOS without overstating release readiness

Preconditions:
- at least one real shell slice exists
- packaging questions are prioritized

Likely outputs:
- platform-risk receipts
- `artifacts/ge07-e6-platform-risk-receipt-2026-06-22.md`
- decision inputs for later release-governance work

Current grounded state:
- `artifacts/cross-platform-build-constraint-questions.md` now holds the canonical GE07-E6 question ledger
- `artifacts/ge07-e6-platform-risk-receipt-2026-06-22.md` now grounds the current repo stop condition, host/tooling gaps, and later GE-09 decision inputs for Linux, Windows, and macOS
- no real shell slice exists yet, so no packaging proof or ship-readiness claim is justified

## Decomposition rules
- each future coding slice must have its own stage-specific handoff identity
- no future slice may claim the whole GE-07 epic as its write authority
- explanation/diagnostics visibility must never be split away so aggressively that the shell looks clean only because the truth is hidden
- packaging work should remain spike/documentary first until a real shell exists

## Not allowed
- one broad “build the UI” handoff
- hidden code authority inside research or collection prose
- early branch/worktree/write-scope guesses embedded here without runtime proof
