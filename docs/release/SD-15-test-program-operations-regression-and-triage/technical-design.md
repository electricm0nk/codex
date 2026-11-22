# SD-15 Test-Program Operations, Regression, and Triage Technical Design

## Purpose
This design operationalizes the SD-15 source STC by defining how GitHub intake, operator triage, regression provenance, install/use validation, external test-cycle surfaces, and status-reconciliation surfaces should cooperate without mutating into fake release operations or undocumented backlog folklore.

## Design posture
- architecture style: `documentary-first tester-operations control plane`
- authority posture: `adjacent-authority-preserving`
- evidence posture: `receipt-first`
- closure posture: `reconciliation-before-claim`

## Context and constraints
- SD-11 already gives Codex a tester-facing workbench and GitHub-facing intake contract, but not the downstream operator classification surface.
- SD-12 already gives Codex channel, platform, update, and rollback truth, but not proof that testers can actually install and use those surfaces on a clean machine.
- SD-13 and SD-14 already make support-state and persistence-state distinctions materially important to triage.
- the repo and workspace README surfaces already communicate project status, so tranche closure will become counterfeit if those surfaces drift away from the ledger or from validation evidence.
- the first truthful SD-15 lane must stay bounded to tester operations and tranche-2 closure rather than inviting public-release operations or observability sprawl.

## Proposed system shape
SD-15 defines six cooperating surfaces:
1. a **GitHub intake boundary** that preserves SD-11 issue-form and evidence-capture truth
2. a **triage classification surface** that turns intake into bounded operator classes rather than one undifferentiated queue
3. a **regression evidence surface** that stores the provenance needed to reconstruct failures and fixes
4. an **install/use validation surface** that records exact build, channel, platform, and workflow proof, including clean-machine validation
5. an **external test-cycle surface** that plans and reports bounded external testing rather than implying it
6. a **status-truth reconciliation surface** that determines whether repo, workspace, and operator-ledger claims may be updated and whether tranche closure is honest

## Data flow
1. a tester interacts with the SD-11 workbench or a distributed build and submits a GitHub issue or bounded feedback payload
2. the triage surface classifies the report against the SD-15 taxonomy using the captured evidence plus adjacent SD-12, SD-13, and SD-14 truth
3. the regression evidence surface records the build/platform/channel/support-state/persistence context needed to reproduce or classify the report later
4. install/use and clean-machine surfaces record whether the current tranche can be acquired, installed, launched, and exercised outside the authoring environment
5. the external test-cycle surfaces convert internal proof into bounded outside evidence with explicit stop conditions and result capture
6. the reconciliation surface compares the resulting truth against `repos/codex/README.md`, `programs/codex/README.md`, and the execution status ledger before closure claims are allowed

## Component boundaries

### GitHub intake boundary
- responsibilities:
  - preserve structured bug/enhancement submission posture from SD-11
  - preserve evidence-capture fields and redaction rules
  - provide the operator with enough context to classify the report
- inputs:
  - tester-submitted bug/enhancement payloads
  - SD-11 evidence-capture rules
- outputs:
  - bounded intake records ready for SD-15 classification
- must not own:
  - support-state truth from SD-13
  - persistence truth from SD-14
  - release/update truth from SD-12

### Triage classification surface
- responsibilities:
  - classify reports into bounded SD-15 categories
  - distinguish unsupported or not-yet-verified states from genuine defects
  - route classification decisions with adjacent-authority provenance
- inputs:
  - intake records
  - SD-12, SD-13, and SD-14 authority references
- outputs:
  - issue class, route, and escalation state
- must not own:
  - product support policy beyond tranche-2 scope
  - silent relabeling of unsupported states as bugs for convenience

### Regression evidence surface
- responsibilities:
  - preserve the exact metadata required to reconstruct a claim
  - link observed failure or unsupported state to build/platform/channel/workflow truth
  - keep attachment and redaction posture explicit
- inputs:
  - triaged issues
  - build and support-state context
- outputs:
  - durable regression receipt or evidence bundle
- must not own:
  - final closure verdict by itself
  - repo/workspace status updates without reconciliation

### Install/use validation surface
- responsibilities:
  - define and later receive the tranche-2 install/use matrix
  - define and later receive clean-machine validation receipts
  - connect validation failures back to the triage taxonomy
- inputs:
  - SD-12 distribution truth
  - bounded workflow expectations from adjacent lanes
- outputs:
  - matrix rows and validation reports
- must not own:
  - updater transport doctrine
  - feature-support truth that belongs to SD-13 or SD-14

### External test-cycle surface
- responsibilities:
  - define cohort, missions, evidence burden, stop conditions, and cadence
  - record what external testers actually exercised and what they encountered
  - convert outside evidence into closure-usable truth without overclaiming breadth or maturity
- inputs:
  - install/use readiness
  - bounded mission definitions
  - triage and evidence rules
- outputs:
  - test-cycle plan and report artifacts
- must not own:
  - public support or release marketing posture
  - free-form narrative summaries without evidence structure

### Status-truth reconciliation surface
- responsibilities:
  - compare repo/workspace/ledger claims against actual validation and external-test evidence
  - classify drift as acceptable pending update, blocking contradiction, or unresolved uncertainty
  - define the tranche-closure verdict rule
- inputs:
  - validation and external-test artifacts
  - `repos/codex/README.md`
  - `programs/codex/README.md`
  - execution status ledger
- outputs:
  - explicit reconciliation checklist and closure verdict
- must not own:
  - implementation truth beyond what the evidence surfaces prove
  - silent closure claims based on one preferred surface

## Data and schema notes
Minimum logical records for later execution lanes:
- intake record with tester-provided and auto-captured evidence
- triage classification record with adjacent-authority references
- regression evidence receipt with build/platform/channel/support/persistence context
- install/use matrix row with exact step status and evidence handle
- clean-machine validation receipt with environment identity and verdict
- external test-cycle plan and external test-cycle report
- status reconciliation row keyed by surface and claim family

Minimum status vocabularies that must remain explicit:
- issue state: `defect`, `unsupported`, `partial`, `not-yet-verified`, `status-drift`, `blocked`
- validation state: `not-run`, `pass`, `pass-with-known-bounds`, `blocked`, `failed`
- reconciliation state: `aligned`, `pending-update`, `contradicted`, `insufficient-evidence`

## External dependencies and references
- `../SD-11-test-user-workbench-and-github-feedback-intake/README.md` — tester-facing workbench and issue-flow authority
- `../SD-11-test-user-workbench-and-github-feedback-intake/artifacts/tester-feedback-evidence-capture-matrix.md` — evidence field matrix
- `../SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md` — tester channel and platform-support vocabulary
- `../SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md` — distribution/update/rollback authority
- `../SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/rollback-withdrawal-and-downgrade-policy.md` — recovery and withdrawal vocabulary
- `../../plans/spec-domains/SD-13-core-class-race-roster-and-level-10-progression-matrix.md` — breadth/progression support-state truth
- `../SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/README.md` — saved-state and migration truth
- `/home/ubuntu/workspace/repos/codex/README.md` — repo-facing current-state truth
- `/home/ubuntu/workspace/programs/codex/README.md` — workspace-facing current-state truth
- `../../plans/roadmaps/codex-execution-status-ledger-2026-06-21.md` — operator route/status truth

## Design decisions already fixed
- GitHub remains the intake destination for tester bug and enhancement submissions in this tranche.
- install/use and clean-machine proof are mandatory evidence surfaces, not optional ceremony.
- external testing must be represented by named plan and result artifacts before closure claims.
- repo/workspace/ledger status surfaces must reconcile before tranche closure claims are truthful.
- unsupported or partially supported paths must remain classifiable rather than being flattened into generic defects.

## Deferred design decisions
- the exact GitHub issue-template or form implementation path
- the exact clean-machine environment technology (fresh VM, containerized GUI path, physical spare machine, or equivalent)
- the exact regression-harness automation boundary, if any
- the exact communications/tooling surface for external tester coordination
- the exact cadence and ownership model for recurring reconciliation checks after tranche-2 closure

## Failure modes and observability
- reports arrive with too little evidence to distinguish install/use, breadth, persistence, or UI defects
- authoring-machine success is mistaken for clean-machine proof
- external testers exercise a build or workflow different from the stated plan and the mismatch is not recorded
- unsupported paths are mislabeled as defects, creating counterfeit regression counts
- repo/workspace/ledger surfaces drift apart and one is treated as “truth by preference” rather than by evidence
- a build is called closure-ready even though no named clean-machine or external-test receipts exist

Required observable signals:
- every issue class carries the adjacent-authority reference that justified the classification
- every regression receipt carries build/platform/channel/workflow identity
- every clean-machine run carries environment identity and per-step evidence
- every external cycle records what was and was not exercised
- every closure verdict states whether repo/workspace/ledger surfaces aligned or drifted

## Verification implications
`acceptance-and-verification.md` must prove that this packet defines explicit surfaces for issue classification, regression evidence, install/use proof, external testing, and status reconciliation. It must also prove the packet does not counterfeit already-executed clean-machine or external-test success while the current program truth remains documentary. Later handoffs must prove actual receipts, actual updated surfaces, and actual reconciliation outcomes rather than quoting this packet as though planning text were evidence.

## Change constraints
- do not let later handoffs bury support-state logic inside issue labels without citing SD-13 or SD-14 authority
- do not let later handoffs update repo/workspace/ledger status surfaces independently without the reconciliation contract
- do not let later handoffs treat the external-test plan as proof that external testing happened
- do not let later handoffs expand this lane into public release or support operations under the cover of “triage”
