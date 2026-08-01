# SD-11 Test-User Workbench Epic Breakdown

## Breakdown rule
This file decomposes the SD-11 source STC into implementation-facing epics and feature seeds without becoming an execution handoff.

## Epic SD11-E1 — Tester workbench frame over real bounded data
**Objective:** Define and later implement the first tester-facing workbench frame over a real bounded Codex snapshot, with explicit support-tier and bounded-scope messaging.

**Derived from:**
- SD-11 README: Objective / In Scope / Acceptance Summary
- `technical-requirements.md`: Tester workbench shell
- `technical-design.md`: Workbench frame, snapshot/view-model adapter

**Depends on:**
- current GE-07 shell truth
- current GE-08 real workbench boundary
- future decision on the exact bounded tester workflow identity

### Feature seed SD11-F1 — Current build/channel/support header
**Outcome:** The tester workbench visibly states current build/channel/support-tier truth and bounded-scope messaging.

**Acceptance signals:**
- current build/channel/support state is displayed in-product
- Linux/macOS/Windows support asymmetry remains visible

**Notes:**
- must not expose raw branch names as the default tester-facing wording

### Feature seed SD11-F2 — Real bounded workflow surface
**Outcome:** The workbench loads a real bounded workflow snapshot instead of mock/demo-only state.

**Acceptance signals:**
- real data source is identified and visible
- placeholder/fallback states remain explicitly labeled

**Notes:**
- later handoff must name the exact command boundary and exact repo write scope

## Epic SD11-E2 — Diagnostics and explanation visibility
**Objective:** Preserve enough explanation, blocked-claim, provenance, and diagnostic context that testers can understand failures and file actionable issues.

**Derived from:**
- `technical-requirements.md`: Diagnostics and explanation visibility
- `technical-design.md`: Diagnostics/explanation surface

**Depends on:**
- SD11-E1
- live or later bounded payloads carrying diagnostic/explanation context

### Feature seed SD11-F3 — Diagnostic and blocked-claim presentation
**Outcome:** The workbench surfaces diagnostic severity/class and blocked claims clearly enough for user comprehension and issue capture.

**Acceptance signals:**
- blocked claims are visible when present
- testers can distinguish validation/state issues from unsupported-scope messaging

**Notes:**
- must preserve triage value rather than flatten to generic prose

### Feature seed SD11-F4 — Explanation/provenance context linking
**Outcome:** The workbench can point a tester from a visible result/problem to the relevant explanation or provenance context.

**Acceptance signals:**
- explanation or provenance references are visible when present
- issue payloads can include that context

**Notes:**
- UI does not become rules authority

## Epic SD11-E3 — GitHub bug-report intake
**Objective:** Implement a governed bug-report flow that submits or preserves a structured GitHub issue payload with actionable evidence.

**Derived from:**
- `artifacts/github-bug-report-intake-contract.md`
- `artifacts/tester-feedback-evidence-capture-matrix.md`

**Depends on:**
- SD11-E1
- SD11-E2
- a resolved GitHub auth/transport posture

### Feature seed SD11-F5 — Bug-report composer
**Outcome:** A tester can open a bug-report flow preloaded with the required structured evidence fields.

**Acceptance signals:**
- required evidence fields are prefilled or explicitly requested
- observed vs expected behavior remains separate

**Notes:**
- no free-form-only bug report flow

### Feature seed SD11-F6 — Bug-report submit/fallback path
**Outcome:** The app either submits the GitHub issue or preserves a draft/copyable payload when submission fails.

**Acceptance signals:**
- testers are never told a report succeeded when it did not
- fallback path preserves the governed payload structure

**Notes:**
- later handoff must name exact failure behavior and verification steps

## Epic SD11-E4 — GitHub enhancement-request intake
**Objective:** Implement a governed enhancement-request flow that captures blocked goals, friction, requested capability, and evidence cleanly.

**Derived from:**
- `artifacts/github-enhancement-request-intake-contract.md`
- `artifacts/tester-feedback-evidence-capture-matrix.md`

**Depends on:**
- SD11-E1
- a resolved GitHub auth/transport posture

### Feature seed SD11-F7 — Enhancement composer
**Outcome:** A tester can submit a structured request tied to a real blocked workflow or missing capability.

**Acceptance signals:**
- goal/friction/requested-capability fields are distinct
- affected surface and evidence can be attached

**Notes:**
- avoid vague idea-dump behavior

## Epic SD11-E5 — Evidence capture and redaction contract
**Objective:** Encode the exact evidence matrix, auto-capture rules, attachment rules, and redaction boundaries the feedback flows must honor.

**Derived from:**
- `artifacts/tester-feedback-evidence-capture-matrix.md`
- `technical-requirements.md`: Diagnostics and explanation visibility; GitHub flows

**Depends on:**
- SD11-E1
- SD11-E2

### Feature seed SD11-F8 — Evidence matrix implementation surface
**Outcome:** The app can distinguish auto-captured, user-supplied, optional, and redacted evidence fields consistently across both issue flows.

**Acceptance signals:**
- no required field is left ambiguous
- attachment/redaction rules are enforced consistently

**Notes:**
- later handoff must name exact local file/log/screenshot handling

## Epic SD11-E6 — Update/status and promotion mapping surface
**Objective:** Surface honest build/channel/support state and later update actions without exposing raw repository topology as the product.

**Derived from:**
- `artifacts/update-channel-and-promotion-mapping.md`
- `technical-requirements.md`: Update/channel and support posture

**Depends on:**
- SD11-E1
- future SD-12 authority once accepted for packaging/updater specifics

### Feature seed SD11-F9 — Current build/channel/support display
**Outcome:** The workbench shows current build/channel/support truth visibly enough for tester understanding and issue payload inclusion.

**Acceptance signals:**
- current build/channel/support fields are visible
- issue payloads can include them automatically

**Notes:**
- operator branch truth may be stored but not surfaced raw by default

### Feature seed SD11-F10 — Check/update action surface
**Outcome:** The workbench defines and later implements a check/update surface with honest failure behavior and platform-specific constraints.

**Acceptance signals:**
- update checks do not pretend unavailable platforms are supported equally
- failure states remain visible and attributable

**Notes:**
- exact updater transport belongs to a later bounded handoff and may depend on SD-12

## Initial sequencing
1. SD11-E1 — Tester workbench frame over real bounded data
2. SD11-E2 — Diagnostics and explanation visibility
3. SD11-E5 — Evidence capture and redaction contract
4. SD11-E3 — GitHub bug-report intake
5. SD11-E4 — GitHub enhancement-request intake
6. SD11-E6 — Update/status and promotion mapping surface

## Handoff boundary
No coding harness should act directly from this file. Each later execution slice must receive a dedicated execution handoff that names:
- exact repo paths
- exact allowed write scope
- exact required reads
- exact verification commands
- exact non-goals
- exact failure/rollback behavior when transport or update actions are involved

Any derived handoff file must also receive its own artifact card on the board.

## Completion gate
- [ ] every requirement is routed to at least one epic
- [ ] every epic has a bounded objective
- [ ] no epic silently changes program doctrine
- [ ] unresolved decisions remain in `risks-and-open-questions.md`, not hidden here
