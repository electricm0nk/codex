# SD-15 Test-Program Operations, Regression, and Triage Epic Breakdown

## Breakdown rule
This file decomposes the SD-15 source STC into implementation-facing and documentary-facing epics without becoming an execution handoff.

## Epic SD15-E1 — Triage taxonomy and intake-to-routing contract
**Objective:** Define and later operationalize the bounded SD-15 issue taxonomy and the exact bridge from SD-11 GitHub intake into operator triage classes.

**Derived from:**
- SD-15 README: Objective / Authority and Scope / Acceptance Summary
- `technical-requirements.md`: Issue intake and triage taxonomy requirements
- `references/upstream-dependency-contract.md`

**Depends on:**
- SD-11 tester-workbench and GitHub-intake authority
- SD-13 support-state truth
- SD-14 persistence-failure truth

### Feature seed SD15-F1 — Triage class dictionary
**Outcome:** The SD-15 taxonomy is explicit enough that operator routes can distinguish UI, rules, content, unsupported, packaging, install/use, persistence, and status-drift classes.

**Acceptance signals:**
- every class has a meaning, evidence threshold, and adjacent-authority reference
- unsupported or partial states remain visibly classifiable

### Feature seed SD15-F2 — Intake-to-triage mapping
**Outcome:** A tester-submitted GitHub issue or feedback payload can be mapped into the SD-15 taxonomy without rewriting SD-11 issue UX.

**Acceptance signals:**
- required intake fields for triage are explicit
- operator-added classification data is separated from tester-provided evidence

## Epic SD15-E2 — Regression evidence and provenance contract
**Objective:** Define and later operationalize the receipt-grade evidence surface that makes regressions and defect states reconstructable.

**Derived from:**
- `technical-requirements.md`: Regression evidence and provenance requirements
- `artifacts/tranche-2-project-status-truth-reconciliation-checklist.md`

**Depends on:**
- SD15-E1
- SD-12 build/channel/platform truth
- SD-13 and SD-14 adjacent truth where relevant

### Feature seed SD15-F3 — Regression receipt schema
**Outcome:** Every material regression claim has a bounded metadata contract for build, platform, channel, workflow, support-state, persistence-state, and evidence attachments.

**Acceptance signals:**
- reproduction and provenance fields are explicit
- attachment and redaction posture is explicit

### Feature seed SD15-F4 — Evidence refresh and verdict rules
**Outcome:** Operators can tell whether a receipt is current, stale, partial, or insufficient for closure use.

**Acceptance signals:**
- receipt freshness and insufficiency states are defined
- “not reproduced” cannot masquerade as “fixed” without the required evidence

## Epic SD15-E3 — Tranche-2 install/use matrix and clean-machine validation
**Objective:** Define and later populate the install/use matrix and clean-machine receipt surfaces for bounded tester builds.

**Derived from:**
- `technical-requirements.md`: Install/use matrix requirements; Clean-machine validation requirements
- `artifacts/tranche-2-install-and-use-matrix.md`
- `artifacts/tranche-2-clean-machine-validation-report.md`

**Depends on:**
- SD-12 distribution/update truth
- SD15-E2

### Feature seed SD15-F5 — Install/use matrix population slice
**Outcome:** The tranche-2 tester surface has a governed per-platform/per-channel install and use matrix rather than ad hoc notes.

**Acceptance signals:**
- matrix rows name acquisition, install, launch, and bounded workflow proof
- status vocabulary is explicit and evidence-bearing

### Feature seed SD15-F6 — Clean-machine validation slice
**Outcome:** A clean-machine receipt/report can prove or block tranche-2 install/use claims outside the authoring environment.

**Acceptance signals:**
- environment identity, build under test, and per-step evidence are captured
- failures route back into the SD-15 taxonomy rather than becoming folklore

## Epic SD15-E4 — External test-cycle planning and result adjudication
**Objective:** Define and later populate the bounded plan/report surfaces for real external testing.

**Derived from:**
- `technical-requirements.md`: External test-cycle planning requirements; External test-cycle reporting requirements
- `artifacts/tranche-2-external-test-cycle-plan.md`
- `artifacts/tranche-2-external-test-cycle-report.md`

**Depends on:**
- SD15-E3
- SD15-E2

### Feature seed SD15-F7 — External cycle launch plan
**Outcome:** Operators can launch a bounded external test cycle with explicit cohort, missions, evidence burden, and stop conditions.

**Acceptance signals:**
- tester cohort and build matrix are explicit
- unsupported-path warnings and escalation rules are explicit

### Feature seed SD15-F8 — External cycle result and verdict slice
**Outcome:** External testing produces a durable result surface that can inform closure instead of anecdote.

**Acceptance signals:**
- actual participation and executed missions are recorded
- defect versus unsupported versus blocked outcomes remain distinguishable

## Epic SD15-E5 — Project-status truth reconciliation and tranche-closure verdict
**Objective:** Define and later execute the reconciliation process that decides whether repo/workspace/ledger surfaces may claim tranche closure.

**Derived from:**
- `technical-requirements.md`: Project-status truth reconciliation requirements
- `artifacts/tranche-2-project-status-truth-reconciliation-checklist.md`

**Depends on:**
- SD15-E2
- SD15-E3
- SD15-E4

### Feature seed SD15-F9 — Cross-surface reconciliation checklist
**Outcome:** Repo README, workspace README, and execution ledger claims can be compared against the same evidence basis.

**Acceptance signals:**
- the exact fields to reconcile are explicit
- contradiction and pending-update states are distinct

### Feature seed SD15-F10 — Closure verdict rule
**Outcome:** A later lane can state whether tranche-2 may be called closed, still bounded, or blocked, and why.

**Acceptance signals:**
- closure criteria cite actual evidence surfaces
- lingering contradictions or missing proof remain blocking when the contract says so

## Epic SD15-E6 — Optional later automation and helper surfaces
**Objective:** Only after the documentary evidence model stabilizes, define bounded automation or in-product helper surfaces for triage, regression receipt generation, or status syncing.

**Derived from:**
- SD-15 README: Next Stage Rule
- `risks-and-open-questions.md`: Automation boundaries

**Depends on:**
- SD15-E1 through SD15-E5

### Feature seed SD15-F11 — Regression/status helper boundary
**Outcome:** Any automation proposal is bounded by the already-accepted documentary contracts and cannot invent new authority.

**Acceptance signals:**
- helper outputs point back to the authoritative artifacts
- automation failure cannot counterfeit success or closure

### Feature seed SD15-F12 — Assisted triage or evidence packaging boundary
**Outcome:** Optional helper tooling can reduce operator friction without replacing the authority of the documented taxonomy and evidence fields.

**Acceptance signals:**
- operator override and evidence visibility remain explicit
- helper-generated classifications remain auditable

## Epic SD15-E7 — Released Linux alpha tester-defect repair bundle
**Objective:** Repair the bounded defects discovered by the LNX-A governed Linux alpha tester run without widening into credentialed GitHub submission, tranche closure, or broad product-readiness claims.

**Derived from:**
- `artifacts/tranche-2-lnx-a-testing-instructional-brief-2026-07-02.md`
- tester evidence captured on 2026-07-02 for the released `alpha-v0.0.0-c2cea5c6` Linux artifact
- `artifacts/regression-receipt-schema.md`
- `artifacts/intake-to-triage-mapping.md`
- `artifacts/triage-class-dictionary.md`
- live repo surfaces under `/home/ubuntu/workspace/repos/codex`

**Depends on:**
- SD15-E3 install/use evidence
- SD15-E4 external tester execution evidence
- SD-12 publication workflow truth
- SD-11 tester workbench and feedback-draft truth

### Feature seed SD15-F13 — Packaged GE08 proof-package resource path
**Outcome:** The released Linux desktop artifact can load the governed GE08 Guard Stance proof package without relying on `tests/fixtures/**`, `/home/runner/work/**`, or any source-checkout-only path on tester machines.

**Acceptance signals:**
- packaged builds have a shipped proof-package path or equivalent governed runtime resource
- default tester workbench load no longer falls back merely because the repo fixture path is absent
- fallback behavior remains truthful for genuine package-load failures

### Feature seed SD15-F14 — Release asset/checksum filename truth repair
**Outcome:** GitHub release assets, release notes, and `checksums.sha256` agree on the exact final filenames for `.deb` and `.AppImage` artifacts.

**Acceptance signals:**
- checksum manifest entries name the exact uploaded asset names
- SHA-256 integrity behavior remains unchanged
- publication workflow no longer emits spaced filenames in the checksum manifest while uploading dot-separated assets

### Feature seed SD15-F15 — Manual feedback-draft transport posture hardening
**Outcome:** The released app clearly preserves manual issue/enhancement drafts when no GitHub submission transport is configured, without adding hardcoded credentials or claiming submission.

**Acceptance signals:**
- no-transport bug and enhancement paths remain `draft-preserved` with no issue handle
- UI copy does not imply a live GitHub submission transport exists in this build
- no token, PAT, OAuth credential, or GitHub poster is introduced by this repair bundle

## Epic SD15-E8 — Feedback/update output context-sanitization boundary
**Objective:** Prevent internal memory/system-context blocks from entering user-visible update evidence, preserved drafts, or manual filing payloads while preserving correct local-build update truth.

**Derived from:**
- Todd's 2026-07-02 update-check tester evidence showing a correct `NO-OFFICIAL-RELEASE-FOR-THIS-BUILD` verdict followed by an unexpected `<memory-context>` block
- `artifacts/regression-receipt-schema.md`
- `artifacts/intake-to-triage-mapping.md`
- SD-11 feedback composer and no-transport draft surfaces in `/home/ubuntu/workspace/repos/codex`

**Depends on:**
- SD15-E7 merged tester-defect repair bundle
- SD-11 feedback draft and update-check surfaces
- SD-12 local/non-governed build update truth

### Feature seed SD15-F16 — Internal-context sanitizer for reportable output
**Outcome:** Copyable/reportable tester output strips `<memory-context>` and recalled-memory system-note blocks while preserving ordinary tester evidence around the removed block.

**Acceptance signals:**
- bug and enhancement draft outputs cannot contain `<memory-context>` or recalled-memory system-note text
- removed internal context is replaced with an explicit removal marker
- normal tester text before and after the stripped block remains intact

### Feature seed SD15-F17 — Update-check no-official-release truth preservation
**Outcome:** Defensive sanitization does not mutate the correct local-build update result into a false official release/update claim.

**Acceptance signals:**
- local/non-governed builds still report `no-official-release-for-this-build`
- update-check reportable evidence can be copied or referenced without leaking internal context

## Initial sequencing
1. SD15-E1 — Triage taxonomy and intake-to-routing contract
2. SD15-E2 — Regression evidence and provenance contract
3. SD15-E3 — Tranche-2 install/use matrix and clean-machine validation
4. SD15-E4 — External test-cycle planning and result adjudication
5. SD15-E5 — Project-status truth reconciliation and tranche-closure verdict
6. SD15-E6 — Optional later automation and helper surfaces
7. SD15-E7 — Released Linux alpha tester-defect repair bundle
8. SD15-E8 — Feedback/update output context-sanitization boundary

## Handoff boundary
No coding harness or documentary worker should act directly from this file. Each later slice must receive a dedicated handoff that names:
- exact target surface and destination paths
- exact allowed write scope
- exact required reads
- exact verification commands or proof obligations
- exact non-goals
- exact adjacent-authority boundaries that the slice must preserve

Any derived handoff file must also receive its own artifact card on the board.

## Completion gate
- [ ] every requirement is routed to at least one epic
- [ ] every epic has a bounded objective
- [ ] no epic silently changes program doctrine
- [ ] unresolved decisions remain in `risks-and-open-questions.md`, not hidden here
