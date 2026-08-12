# SD-15 Upstream Dependency Contract

## Purpose
This contract records what the upstream strategic, documentary, and status surfaces authorize for SD-15 and what they explicitly do not authorize.

## Upstream surfaces and permitted use

| Upstream surface | What SD-15 may rely on | What it does not authorize |
|---|---|---|
| `programs/codex/plans/spec-domains/SD-15-test-program-operations-regression-and-triage.md` | the strategic objective, scope boundary, minimum operator truths, and same-domain source-STC obligation | repo implementation authority, claims that clean-machine proof already exists, or claims that tranche-2 is already closed |
| `programs/codex/plans/roadmaps/codex-execution-status-ledger-2026-06-21.md` | current active-artifact and route-truth context that status reconciliation must preserve | regression evidence, clean-machine proof, external-test results, or closure truth by itself |
| `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md` | tester-workbench and GitHub issue-flow authority, plus the rule that evidence capture must stay structured | downstream triage authority, regression verdicts, or install/use proof |
| `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/tester-feedback-evidence-capture-matrix.md` | exact intake evidence fields and redaction posture | proof that the downstream operator lane already exists, or permission to invent triage classes without documentary authority |
| `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md` | tester-facing channel/support vocabulary and operator branch mapping | release-transport authority or proof that a specific build was installable on a clean machine |
| `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md` | distribution, build/channel/platform, update, and rollback authority the SD-15 evidence lane must inherit | proof that a distributed build was actually installable or usable on a clean machine |
| `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/rollback-withdrawal-and-downgrade-policy.md` | withdrawn/superseded/blocked/recovery vocabulary the operator lane may reuse | saved-state compatibility truth, bug classification truth, or clean-machine proof |
| `programs/codex/plans/spec-domains/SD-13-core-class-race-roster-and-level-10-progression-matrix.md` | bounded roster/progression support-state truth and the rule that unsupported or partial paths must remain visible | permission to classify unsupported breadth as generic defects for convenience |
| `programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/README.md` | saved-state continuity, migration, and upgrade-safe diagnostic truth | permission to claim update success equals persistence success, or to rewrite saved-state classifications locally |
| `/home/ubuntu/workspace/repos/codex/README.md` | repo-facing current-state, onboarding, and bounded-product truth that closure review must reconcile honestly | tranche-closure proof, operator triage truth, or external-test evidence |
| `/home/ubuntu/workspace/programs/codex/README.md` | workspace-facing current-state and planning/control-plane posture | proof that repo state, external evidence, and operator ledger are already aligned |

## Downstream obligations imposed by this contract
Any later SD-15 handoff must:
- preserve SD-11 ownership of tester-facing issue submission and evidence-capture UX
- preserve SD-12 ownership of build/channel/platform/update/rollback truth
- preserve SD-13 ownership of bounded breadth/support-state truth when classifying unsupported or partial behavior
- preserve SD-14 ownership of saved-state continuity and migration truth when classifying persistence-facing issues
- record explicit evidence when updating repo/workspace/ledger status surfaces so closure claims remain auditable
- distinguish planning artifacts from execution receipts explicitly

## What this packet still does not prove
This packet does not prove:
- that a clean-machine validation run has already been executed
- that a real external tester cohort has already run a cycle
- that the current repo/workspace/ledger surfaces already reconcile
- that a durable regression harness or automation lane already exists
- that every defect class can already be auto-routed without operator judgment

## Propagation rule
If a later implementation or documentary slice discovers a new authoritative regression surface, clean-machine receipt source, or status-reconciliation obligation that changes program-level expectations, patch this contract and the SD-15 README before claiming the new behavior as settled truth.
