# SD12-E2-B1 Promotion Truth Repair Closure — Missing `uat` publication and promotion truth

## Card outcome
- evidence_class: `repaired-by-documentary-revision`
- repair_path: `revised SD-11/SD-12 documentary authority surfaces to match the live repo control plane instead of inventing a nonexistent `uat` stage`
- live operator promotion truth after repair: `develop -> main`
- tester-channel truth after repair:
  - `alpha` is backed by `develop`
  - `stable` is backed by `main`
  - `beta` is reserved/unavailable until a governed candidate promotion surface exists in repo/workflow truth
- SD12-E2-R2 verdict: `unblocked`

## Live repo truth grounded on 2026-06-29
- `git -C /home/ubuntu/workspace/repos/codex branch --list` shows local `develop` and `main`, but no local `uat` branch
- `git -C /home/ubuntu/workspace/repos/codex branch -r` shows `origin/develop` and `origin/main`, but no remote `origin/uat` branch
- `/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml` is still the only live promotion-governance workflow evidence in the repo, and it proves only that pull requests into `main` must come from `develop`
- no repo workflow or branch surface proves a governed `beta`/candidate lane today

## Decisive judgment
The problem was not missing prose. It was documentary doctrine claiming a three-stage promotion model with an intermediate `uat` lane that the live repo does not implement.

I reviewed both honest repair paths:
1. make `uat` real in repo/workflow truth
2. revise the documentary contract so it stops claiming `uat`

The first path had no live evidence, no pre-existing branch surface, and no bounded repo/control-plane implementation brief for creating one. Taking it here would have manufactured a release-control topology from planning desire.

The second path is the truthful one. The documentary contract now matches the real control plane: `develop -> main` is live, `alpha` and `stable` are backed, and `beta` is reserved until a governed candidate promotion surface actually exists.

## Authoritative surfaces after repair
### Live repo/control-plane authority
- `git -C /home/ubuntu/workspace/repos/codex branch --list`
- `git -C /home/ubuntu/workspace/repos/codex branch -r`
- `/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml`

These surfaces are now the authoritative proof for current promotion topology.

### Documentary authority surfaces updated in this repair
- `programs/codex/plans/spec-domains/SD-11-test-user-workbench-and-github-feedback-intake.md`
- `programs/codex/plans/spec-domains/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update.md`
- `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md`
- `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/technical-requirements.md`
- `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/technical-design.md`
- `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/acceptance-and-verification.md`
- `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md`
- `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md`
- `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/technical-requirements.md`
- `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/technical-design.md`
- `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/acceptance-and-verification.md`
- `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/epic-breakdown.md`
- `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/references/upstream-dependency-contract.md`
- `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/distribution-platform-support-and-channel-matrix.md`
- `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/github-artifact-publication-and-promotion-contract.md`

These surfaces are now the authoritative documentary contract for publication/promotion truth.

### Read-only comparison surfaces that remain unchanged in this lane
- `/home/ubuntu/workspace/repos/codex/README.md`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/composeBugReport.test.ts`

These repo implementation surfaces still reflect older three-stage assumptions in places, but they were explicitly out of write scope for this documentary truth-repair lane. They remain read-only comparison evidence, not publication-topology authority.

## What changed in the contract
- removed documentary claims that live operator truth included a three-stage path with intermediate `uat` promotion
- replaced them with the live control-plane truth `develop -> main`
- preserved tester-facing labels `alpha`, `beta`, and `stable`, but reclassified `beta` as reserved/unavailable until a governed candidate promotion surface exists
- updated SD-11 and SD-12 higher-order spec-domain surfaces so the planning/control-plane layer no longer contradicts the requirements bundles
- updated SD-12 epic/publication artifacts so future handoffs cannot silently reintroduce a fake `uat` dependency

## Why SD12-E2-R2 is now unblocked
`SD12-E2-R2 FLOW: GitHub publication and promotion handoff artifact` is now unblocked because the next handoff can be authored against an honest, explicit two-stage live topology:
- `alpha` publication backed by `develop`
- `stable` publication backed by `main`
- no `beta` publication claim unless a future lane first creates and verifies a governed candidate promotion surface

That is enough truth to author a bounded publication handoff without counterfeit repo state.

## Required verification surfaces for downstream use
```bash
git -C /home/ubuntu/workspace/repos/codex branch --list
git -C /home/ubuntu/workspace/repos/codex branch -r
```

```text
/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml
/home/ubuntu/workspace/programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md
/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/distribution-platform-support-and-channel-matrix.md
/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/github-artifact-publication-and-promotion-contract.md
```

## Explicit refusals preserved by this repair
- do not treat documentary desire for a `beta`/`uat` lane as evidence that it exists
- do not treat repo UI/status code as release-topology authority when the repo branches/workflows disprove it
- do not let the later publication handoff claim a candidate lane unless a future bounded lane first makes that control-plane surface real and auditable
