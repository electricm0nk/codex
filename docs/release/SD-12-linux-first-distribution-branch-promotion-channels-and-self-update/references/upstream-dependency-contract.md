# Upstream Dependency Contract — SD-12

## Purpose
This contract records what the SD-12 source STC may rely on from accepted upstream strategic, requirements, doctrine, and live-repo surfaces, and what those surfaces do **not** authorize.

## Upstream contract table
| Upstream surface | SD-12 may rely on | SD-12 must not infer |
|---|---|---|
| `programs/codex/plans/spec-domains/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update.md` | the strategic problem statement, in-scope/out-of-scope boundary, required source-STC path, and the requirement to keep Linux first-class while separating operator branches from tester channels | implementation authority, exact repo/CI write scope, or exact updater/package technology |
| `programs/codex/plans/spec-domains/SD-11-test-user-workbench-and-github-feedback-intake.md` and `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md` | the tester-facing workbench boundary, GitHub-backed feedback posture, and the rule that user-facing channel/support language already exists upstream | authority to rewrite SD-11 issue-flow or tester-workbench scope, or proof that updater mechanics already exist |
| `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md` | the current mapping of live `develop -> main` operator truth plus a reserved future `beta` label to tester-facing channel language, plus Linux/macOS/Windows support-tier vocabulary | final release automation, package formats, or automatic-update eligibility |
| `programs/codex/doctrine/program-doctrine-and-scope-charter.md` and `quality-gate-policy.md` | evidence-first, no-counterfeit-completion, and source-STC-before-implementation doctrine | permission to skip exact handoff fields, exact verification commands, or exact trust thresholds |
| `programs/codex/doctrine/documentation-control-plane.md` | the artifact-lifecycle and authority-surface rules that make SD-12 a program requirements packet rather than repo folklore | permission to bypass the same-domain execution-story step or to hide upstream implications |
| `repos/codex/README.md` | the truthful current-state statement that Linux onboarding/build is verified while release packaging remains unfinished | proof of productized distribution, packaging, or updater readiness |
| `repos/codex/.github/workflows/allow-only-develop-into-main.yml` | the existence of branch-governance truth for `main`, including the rule that only `develop` may flow into `main` | a complete release-control plane, `uat` automation, or any artifact-publication flow |
| `repos/codex/apps/desktop/package.json` and `src-tauri/tauri.conf.json` | current desktop package identity, build toolchain, and the fact that bundle outputs are still inactive | proof that packaging or updater behavior already exists |
| `repos/codex/apps/desktop/src/App.tsx` and `src/sd11/**` | the current tester-facing build/channel/support/update language and anti-counterfeit-update posture | permission to treat UI copy as release authority or to skip a manifest/publication contract |

## Downstream obligations
Any later SD-12 execution handoff must:
- name the exact repo or CI write scope
- preserve live `develop -> main` operator truth while keeping tester-facing channel language separate and treating any future `beta` candidate stage as reserved until backed
- preserve Linux-first, macOS-second-class, and Windows-third-class promises exactly
- publish explicit manifest/checksum/provenance materials before claiming automatic update
- preserve rollback/withdrawal visibility instead of substituting silent deletion or generic failure text
- keep SD-11 as the tester-facing UX authority while feeding it governed release truth

## What this packet still does not authorize
- no repo or CI changes by itself
- no updater library selection by implication
- no package-format selection by implication
- no macOS or Windows parity claim by implication
- no public-release or marketplace posture by implication
