# SD-15 Test-Program Operations, Regression, and Triage Technical Requirements

## Purpose
This document defines the normative requirements for tester-program operations in Codex: how incoming issues are classified, what regression evidence is mandatory, what install/use and clean-machine proof must exist, how external test cycles are planned and reported, and how tranche-closure truth is reconciled across operator-facing surfaces.

## Current-state grounding
- SD-11 already defines the tester-facing workbench boundary, structured GitHub intake posture, and evidence-capture vocabulary.
- SD-12 already defines distribution/update/rollback truth, channel/support semantics, and the refusal to treat shell folklore as update doctrine.
- SD-13 already defines the bounded roster/progression support-state problem and requires partial or unsupported semantics to remain visible.
- SD-14 already defines saved-state, migration, and upgrade-safe continuity truth for local character artifacts.
- `repos/codex/README.md` and `programs/codex/README.md` already expose current-state claims that will become counterfeit if tranche-closure evidence is not reconciled explicitly.
- the execution status ledger already exposes route truth and active-artifact truth, but it is not by itself a regression, install/use, or external-test reporting surface.

## Requirement families

### 1. Boundary and scope requirements
- SD-15 MUST define tester-program operations as evidence work rather than a generic issue bucket.
- SD-15 MUST remain bounded to tester-program operations for tranche-2 closure.
- SD-15 MUST NOT imply public-release support operations, broad telemetry/SRE scope, or customer-service infrastructure.
- SD-15 MUST preserve the distinction between:
  - SD-11 tester-facing workbench and issue-flow UX
  - SD-12 distribution/update/rollback truth
  - SD-13 bounded support-state and progression truth
  - SD-14 persistence/migration truth
  - SD-15 operator triage, regression evidence, validation, and closure-truth reconciliation

### 2. Issue intake and triage taxonomy requirements
The SD-15 packet MUST define a bounded issue taxonomy that distinguishes at minimum:
1. UI or presentation defects
2. rules-engine defects
3. content or data defects
4. unsupported semantics / known unsupported paths
5. packaging or distribution defects
6. install/use defects
7. persistence, migration, or saved-state continuity defects
8. status/documentation drift

For each class, the packet MUST define:
- the meaning of the class
- the minimum evidence needed to classify a report there
- what adjacent authority surface supplies the governing truth for the class
- what outcomes should remain visibly `unsupported`, `blocked`, `partial`, or `not-yet-verified` instead of being mislabeled as ordinary bugs

GitHub remains the intake destination, but SD-15 MUST define the downstream operator classification contract after intake.

### 3. Regression evidence and provenance requirements
Every regression claim or defect state under SD-15 MUST be reconstructable from explicit evidence rather than memory.

At minimum, the packet MUST require explicit treatment of:
- tester-visible build label or version
- tester-facing channel and support label
- operator branch/provenance handle when available
- commit or build identity when available
- platform and package/install context
- active bounded workflow or mission under test
- relevant SD-13 support-state context when breadth/progression is involved
- relevant SD-14 persistence/migration context when saved-state behavior is involved
- observed behavior
- expected behavior
- reproduction steps or reproduction impossibility note
- diagnostics, explanation, or status evidence when present
- attachment/redaction posture for screenshots, logs, save files, or release metadata

The packet MUST define what evidence is auto-captured, what is user-supplied, and what is operator-added during triage.

### 4. Install/use matrix requirements
The packet MUST define a tranche-2 install/use matrix with exact path-level authority under `artifacts/tranche-2-install-and-use-matrix.md`.

The matrix MUST record at minimum:
- platform and support tier
- build/channel/support identity under test
- install prerequisites and acquisition path
- bounded workflow entry point(s)
- expected visible proof for successful install/use
- required evidence capture when a step fails
- allowed status vocabulary such as `not-run`, `pass`, `blocked`, `unsupported`, and `out-of-scope`

The matrix MUST connect install/use claims to adjacent authorities rather than replacing them.

### 5. Clean-machine validation requirements
The packet MUST define a named clean-machine validation receipt surface under `artifacts/tranche-2-clean-machine-validation-report.md`.

The clean-machine contract MUST define:
- what counts as a clean machine or clean environment
- the exact environment identity fields to record
- the build under test and how it was acquired
- the install/use steps executed
- the evidence captured at each step
- the triage class used when a step fails
- the final verdict vocabulary (`pass`, `pass-with-known-bounds`, `blocked`, `failed`, or equivalent)

The packet MUST explicitly refuse authoring-machine success as a substitute for clean-machine proof.

### 6. External test-cycle planning requirements
The packet MUST define a named external-test-cycle plan surface under `artifacts/tranche-2-external-test-cycle-plan.md`.

The plan MUST define at minimum:
- target tester cohort and selection constraints
- build/channel/platform matrix for the cycle
- the bounded missions or workflows testers are asked to exercise
- the evidence they must capture
- the support-state or unsupported-path warnings they must see up front
- stop conditions and escalation rules
- operator triage cadence during the cycle
- what conditions make the cycle not ready to launch

External testing MUST remain bounded evidence work, not a vague future aspiration.

### 7. External test-cycle reporting requirements
The packet MUST define a named external-test-cycle report surface under `artifacts/tranche-2-external-test-cycle-report.md`.

The report MUST record at minimum:
- which plan it answers
- which build/channel/platform combinations were actually exercised
- what testers completed, skipped, or could not run
- which failures were genuine defects versus unsupported or out-of-scope paths
- the resulting triage distribution by class
- what tranche-closure implications follow
- what remains blocked before the next cycle or closure review

The report MUST preserve negative evidence and partial evidence; silence is not evidence.

### 8. Project-status truth reconciliation requirements
The packet MUST define a named reconciliation surface under `artifacts/tranche-2-project-status-truth-reconciliation-checklist.md`.

Reconciliation MUST cover, at minimum:
- `repos/codex/README.md`
- `programs/codex/README.md`
- `programs/codex/plans/roadmaps/codex-execution-status-ledger-2026-06-21.md`

The reconciliation contract MUST define:
- the exact status fields that must align across those surfaces
- what drift classes are allowed temporarily versus what drift blocks closure
- what evidence input is required before a surface may be updated
- who or what class of downstream slice should update each surface
- the explicit tranche-closure verdict rule when one surface lags or contradicts the others

### 9. Adjacent authority coupling requirements
- SD-15 MUST reuse SD-11 issue-flow vocabulary and evidence-capture posture without allowing SD-11 to become the triage authority.
- SD-15 MUST reuse SD-12 build/channel/platform/update/rollback truth rather than inventing parallel release semantics.
- SD-15 MUST preserve SD-13 support-state truth when classifying breadth/progression reports.
- SD-15 MUST preserve SD-14 persistence, migration, and saved-state failure truth when classifying continuity reports.
- SD-15 MUST make cross-authority provenance visible enough that a report can be routed without rewriting adjacent doctrine.

### 10. Verification and proof obligations for later execution lanes
A future SD-15 execution or documentary handoff derived from this packet MUST name exact commands and inputs, but it may not weaken these proof classes:
- triage taxonomy proof that every major issue/report class is distinguishable
- regression evidence proof that a defect can be reconstructed from durable fields
- install/use proof tied to exact build/platform/channel identity
- clean-machine validation proof tied to a named environment and receipt
- external-test-cycle plan/report proof tied to actual tester activity and explicit stop conditions
- status-reconciliation proof that repo/workspace/ledger surfaces agree before tranche closure is claimed

### 11. Non-goals
This packet does not authorize:
- broad implementation automation in the repo
- public support or release management
- replacing adjacent SD-11 through SD-14 planning surfaces
- claiming clean-machine or external-test success from planned documents alone
- collapsing unsupported behavior and genuine defects into the same reporting class
