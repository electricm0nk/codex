# Tranche-2 Project-Status Truth Reconciliation Checklist

## Purpose
Define the exact status claims that must reconcile across repo, workspace, and operator-ledger surfaces before tranche-2 closure can be stated honestly.

This surface is an evidence-population checklist, not a status-surface update lane. It records what the three public/operator surfaces say today, what the current tranche-2 evidence actually proves, which drift class applies, and what downstream slice class must perform any later surface update.

## Surfaces in scope
- `repos/codex/README.md`
- `programs/codex/README.md`
- `programs/codex/plans/roadmaps/codex-execution-status-ledger-2026-06-21.md`

## Reconciliation states
- `aligned` — all relevant surfaces say materially the same thing and cite compatible evidence
- `pending-update` — evidence exists but one or more surfaces have not been updated yet
- `contradicted` — surfaces make materially incompatible claims
- `insufficient-evidence` — evidence is not yet strong enough to update any surface confidently

## Required claim families
| Claim family | Repo README | Program README | Execution ledger | Evidence required before update |
|---|---|---|---|---|
| current product posture | current truthful product state | current program/tranche posture | active route/state posture | latest accepted validation and adjacent planning truth |
| install/use reality | whether testers can install and use the bounded surface | whether the tranche is operationally usable | whether the route can be considered closure-capable | install/use matrix and clean-machine report |
| build/channel/support truth | visible platform/channel/support posture | workspace-level tranche posture | operator route and active artifact posture | SD-12 build/channel truth plus validation evidence |
| known bounds and blockers | repo-facing limitations | workspace-facing blockers | route blockers or incomplete next steps | triage distribution plus validation/external-test evidence |
| external validation posture | what external evidence exists or is still absent | tranche-level external-test posture | route-state effect of the cycle | external-test plan/report |

## Downstream update authority by surface
| Surface | What this surface is allowed to express after evidence review | Downstream slice class that should update it |
|---|---|---|
| `repos/codex/README.md` | repo-facing onboarding/current-state truth for testers and implementers | repo-facing documentation sync slice released after an SD-15 gate verdict |
| `programs/codex/README.md` | workspace-facing program/tranche posture and control-plane summary | program/workspace status sync slice released after an SD-15 gate verdict |
| `codex-execution-status-ledger-2026-06-21.md` | operator route/state truth plus any tranche-impacting status consequence that belongs on the ledger | operator-status / ledger-sync slice released after an SD-15 gate verdict |

## Populated reconciliation matrix
| Claim family | Repo README statement | Program README statement | Execution ledger statement | Evidence basis | Reconciliation state | Required action |
|---|---|---|---|---|---|---|
| current product posture | `repos/codex/README.md` says Codex is a `developer proof harness plus a buildable desktop workbench surface` and now also states that tranche-2 is not closure-ready because the governed Linux alpha path still carries the GE-08 package-root defect and no external cycle has launched. | `programs/codex/README.md` now records the same blocked tranche-2 tester posture on the workspace authority surface. | The ledger now preserves the same operator truth in the `SD-15 tranche-2 closure addendum`: blocked verdict, GE-08 package-root defect, and external-cycle non-launch with zero participants. | Repo current-state section; install/use matrix current-truth anchors; clean-machine report final verdict; external-test-cycle report tranche-closure posture. Current evidence supports a bounded tester/workbench surface, but not tranche-2 closure or broad usability. | `aligned` | Keep the three surfaces synchronized on the blocked tranche-2 posture until a superseding evidence set changes the verdict. |
| install/use reality | Repo README now says the Linux desktop path is launchable on the currently verified path, but the governed Linux alpha artifact reaches only the bounded fallback workbench and the preferred GE-08 Guard Stance path remains blocked on `package root does not exist`. | Program README now states that clean-machine proof exists for governed acquisition/install/launch and fallback entry, but that the preferred GE-08 path is still blocked. | The ledger now records the same install/use limit as a tranche-2 blocker rather than omitting it. | `artifacts/tranche-2-install-and-use-matrix.md` marks Linux alpha as `pass-with-known-bounds`; `artifacts/tranche-2-clean-machine-validation-report.md` records final verdict `failed`; linked receipt `artifacts/sd15-rr-2026-07-01-linux-alpha-clean-machine-ge08-fallback.md` shows the governed artifact installs and launches but the preferred GE08 workbench fails on a missing package-root path; external-test report preserves that as prerequisite-only evidence. | `aligned` | Preserve the blocked install/use posture until a superseding governed artifact proves the preferred GE-08 path or the public claim is narrowed further by evidence. |
| build/channel/support truth | Repo README says the walkthrough is grounded for the currently verified Linux desktop path, warns other platforms should not be assumed, and says only the Linux onboarding/build path was verified; it does not surface governed `alpha`/`stable` channel truth, reserved `beta`, or the current platform-tier matrix. | Program README does not currently state tester channel, platform-tier, or support-posture truth. | The ledger discusses route state and later release-governance planning, but it does not currently function as the channel/platform support matrix for tranche-2 tester truth. | `artifacts/tranche-2-install-and-use-matrix.md` anchors SD-12 truth: `develop -> alpha`, `main -> stable`, `beta` reserved/unavailable, Linux first-class, macOS second-class, Windows third-class; Linux stable/macOS/Windows rows remain blocked because concrete artifact or launch proof is absent. | `pending-update` | A later gate-backed sync slice should import the governed SD-12 channel/platform/support posture into whichever status surfaces need it, without upgrading blocked macOS/Windows/stable paths into implied readiness. |
| known bounds and blockers | Repo README now exposes the governed-artifact GE08 package-root failure and the unfulfilled external-test gate alongside the pre-existing bounded/demo limitations. | Program README now summarizes the active tranche-2 blockers and closure consequence. | The ledger now records the SD-15 clean-machine failure, the external-cycle non-launch, and the fact that these remain closure blockers even though route truth is clearer. | Clean-machine report final verdict `failed` with primary class `content or data defect`; linked clean-machine receipt for the GE08 package-root failure; external-test-cycle report verdict `blocked-reproduction`, actual participants `0`, tranche-closure posture `not closure-ready`; install/use matrix keeps non-Linux and stable paths visibly blocked or out-of-scope. | `aligned` | Preserve visible blocker-state truth on all three surfaces until the evidence changes. |
| external validation posture | Repo README now states that bounded external cycle `SD15-EXT-2026-07-01-001` did not launch and recorded `0` actual external participants. | Program README now states that external tranche-2 evidence is still absent and still blocking closure. | The ledger now records the external-test-cycle non-launch and the resulting closure consequence for tranche-2. | `artifacts/tranche-2-external-test-cycle-report.md` adjudicates bounded attempt `SD15-EXT-2026-07-01-001` as `not launched / pre-launch blocked`, with `0` actual external participants and tranche-closure posture `not closure-ready`; the report explicitly refuses to reinterpret the clean-machine prerequisite receipt as external-tester evidence. | `aligned` | Preserve the explicit absence of launched external-test evidence unless higher-order authority later waives that proof burden. |

## Current closure implication from the populated matrix
Tranche-2 is not closure-ready on current evidence.

The decisive blockers are:
- the governed Linux alpha artifact still fails the preferred GE-08 Guard Stance path on `package root does not exist`, so the current bounded install/use evidence does not support closure.
- the bounded external cycle did not launch and produced zero external-participant evidence, so external-test proof remains absent.
- repo, program, and ledger surfaces are now synchronized to that blocked state, so the remaining blocker is evidence-real rather than a documentation omission.

## Tranche-2 closure gate verdict
- verdict: `blocked`
- closure claim allowed now: `no`
- repo README status update allowed now: `yes — completed in this gate to surface the blocked tranche-2 truth`
- program README status update allowed now: `yes — completed in this gate to surface the blocked tranche-2 truth`
- execution-ledger status update allowed now: `yes — completed in this gate to surface the blocked tranche-2 truth`

### Why tranche-2 remains blocked after this gate completed
- `repos/codex/README.md`, `programs/codex/README.md`, and `programs/codex/plans/roadmaps/codex-execution-status-ledger-2026-06-21.md` now surface the same blocked-status truth instead of omitting it.
- `artifacts/tranche-2-clean-machine-validation-report.md` and linked receipt `artifacts/sd15-rr-2026-07-01-linux-alpha-clean-machine-ge08-fallback.md` still show the governed Linux alpha artifact failing the preferred GE-08 Guard Stance path on `package root does not exist`.
- `artifacts/tranche-2-external-test-cycle-report.md` still adjudicates bounded attempt `SD15-EXT-2026-07-01-001` as `not launched / pre-launch blocked`, with `0` actual external participants and tranche-closure posture `not closure-ready`.
- The closure block is therefore preserved for evidence reasons, not because the status surfaces remain unsynchronized.

### What must change before a later closure gate may pass
- A superseding evidence set must eliminate the install/use contradiction or narrow the public claim so repo-facing expected behavior no longer outruns the governed artifact.
- A launched external cycle with real tester evidence must exist, or higher-order authority must explicitly waive that proof burden.
- After the evidence burden is satisfied, downstream README/ledger sync slices may align repo, program, and operator surfaces without hiding unsupported or third-class platform truth.

## Closure rule
Tranche-2 closure may be claimed only when:
- install/use and clean-machine evidence exist for the relevant bounded surface
- external-test results exist or an explicit higher-order authority says why they are not required yet
- repo/workspace/ledger claims are either `aligned` or `pending-update` with a named downstream update slice and no material contradiction
- any remaining unsupported or third-class-platform truths remain visible rather than hidden inside optimistic prose

## Blocking drift examples
- repo README claims usable tester installs while no clean-machine receipt exists
- repo README claims the bounded GE08 workbench loads successfully while the governed artifact fails on the named clean-machine path
- workspace README implies broader tranche readiness than SD-12/SD-15 evidence allows
- execution ledger implies tranche closure or route completion while external test-cycle evidence is missing, non-launched, or contradictory

## Explicit refusals
- do not treat one surface as automatically authoritative over the others without evidence
- do not update all three surfaces by copy-paste if the underlying evidence differs
- do not call `pending-update` a success state when the missing update would materially change operator decisions
- do not call tranche-2 closed from route truth, authoring-machine proof, or planned documents alone
