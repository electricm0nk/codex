# Tranche-2 External Test-Cycle Plan

## Purpose
Define the launch contract for bounded tranche-2 external evidence work so testers exercise named builds, named platforms, and named missions with a receipt-grade evidence burden instead of vague "try it and report back" instructions.

## Boundary of this surface
- This document is a launch-plan surface, not evidence that any tester has already been recruited, contacted, or run.
- This plan may authorize only bounded external evidence work for tranche-2; it does not authorize public release, open-ended beta operations, or closure claims.
- No row in this plan may be treated as launchable until the row-specific readiness gates below are satisfied with named build, platform, and evidence-capture truth.
- External-test evidence supplements but does not replace the SD-15 install/use matrix, clean-machine validation boundary, regression receipt schema, or status-reconciliation surfaces.
- Unsupported, partial, blocked, and not-yet-verified states must remain visible throughout the cycle; the plan must not turn known bounds into counterfeit product support.

## Plan status header
Populate all fields before an actual launch decision. If a field is not yet grounded, preserve the gap explicitly.

- plan state: `bounded-attempt-recorded`
- cycle identifier: `SD15-EXT-2026-07-01-001`
- plan date: `2026-07-01`
- operator owner: `god-emporer`
- launch decision state: `blocked-before-launch`
- target build/channel matrix revision: `LNX-A / alpha-v0.0.0-c2cea5c6`
- target platform/support-tier matrix revision: `Linux / first-class`
- target mission bundle revision: `M1-M6 / bounded GE08 Guard Stance proof package`
- evidence packet revision: `SD15-RR-20260701-001 + tranche-2-clean-machine-validation-report.md`
- adjacent authority references:
  - `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md`
  - `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md`
  - `programs/codex/plans/spec-domains/SD-13-core-class-race-roster-and-level-10-progression-matrix.md`
  - `programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/README.md`
  - `programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/triage-class-dictionary.md`
  - `programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/regression-receipt-schema.md`
  - `programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/tranche-2-install-and-use-matrix.md`
  - `programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/tranche-2-clean-machine-validation-report.md`

## Launch posture
- External testing is evidence work. The operator is asking testers to validate a bounded tranche claim, not to roam for general feedback.
- The minimum truthful external launch unit is a specific matrix row plus a specific mission bundle plus a specific evidence packet.
- A row may move to `ready-for-row-launch` only when the governed build handle, the platform/support warning, the bounded mission text, and the evidence path are all fixed.
- A cycle may remain globally `not-ready` even if this plan is complete; plan readiness is not the same thing as build/channel/platform readiness.

## Current bounded attempt status
- bounded attempt state: `pre-launch blocked`
- launchable row truth at capture time: `LNX-A` is the only row grounded enough to launch; `LNX-S`, `MAC-A`, `MAC-S`, `WIN-A`, and `WIN-S` remain not-ready per the install/use matrix.
- evidence basis now available:
  - `artifacts/tranche-2-clean-machine-validation-report.md`
  - `artifacts/sd15-rr-2026-07-01-linux-alpha-clean-machine-ge08-fallback.md`
- decisive pre-launch fact: the governed Linux alpha path is now grounded through clean-machine acquisition/install/launch evidence, and Todd Hintzmann is the intended `LNX-A` external Linux tester, but the external cycle is intentionally deferred until tranche-2 coding is fully complete; no row-specific operator packet issuance event should be recorded before that launch condition is satisfied.
- result for this bounded attempt: the cycle is recorded as `not launched` with launch decision state `blocked-before-launch`; external participation must remain zero until tranche-2 coding is fully complete and the `LNX-A` operator packet is actually issued to Todd Hintzmann.

## Target tester cohort and selection constraints
### Cohort slices
| Cohort slice | Target count | Platform / support tier | Channel target | Why included | Selection constraints | Launch gate state |
|---|---:|---|---|---|---|---|
| Linux-first tranche anchor | 1 tester for the first cycle (Todd Hintzmann), expandable later if needed | Linux / first-class | `alpha` first, `stable` only when separately grounded | Establish the first truthful external acquisition, launch, and bounded-workbench evidence on the only platform currently grounded as first-class | Todd Hintzmann is the intended first `LNX-A` tester, must use a graphical Linux desktop session, must use only the named governed artifact, and must capture screenshots/logs plus issue-ready evidence through the SD-11 intake path | `ready-on-build-and-clean-machine-basis; deferred until tranche-2 coding completion and subsequent LNX-A operator-packet issuance to Todd` |
| Linux-first confirmation slice | 1-2 testers | Linux / first-class | `stable` | Confirms that a separately grounded stable row does not inherit truth by implication from alpha | Same as the Linux-first tranche anchor, plus the stable artifact and provenance must be separately identified | `pending-stable-artifact-selection` |
| macOS bounded sentinel slice | 1-2 testers | macOS / second-class | `alpha` or `stable` only for rows that leave `blocked` in the install/use matrix | Preserves second-class platform visibility without pretending parity | Must have the exact governed macOS artifact and trust-install instructions, must accept second-class support posture, and must be able to capture trust/install/launch evidence | `blocked-until-macos-row-is-grounded` |
| Windows bounded sentinel slice | 1 tester | Windows / third-class | `alpha` or `stable` only for rows that leave `blocked` in the install/use matrix | Keeps third-class posture explicit without erasing the platform or inflating it | Must have the exact governed Windows artifact path, must accept third-class support posture, and must be willing to stop on unsupported-path warnings rather than treating them as generic defects | `blocked-until-windows-row-is-grounded` |

### Selection rules
- Do not recruit or brief a tester against a platform/channel row that is still `blocked`, `unsupported`, or `out-of-scope` in the install/use matrix.
- Do not use ad hoc feature-branch artifacts, local developer builds, or authoring-machine outputs as official external-test-cycle inputs.
- Prefer testers who can complete the full evidence packet without operator shadowing: screenshot capture, issue payload completion, and attachment/redaction handling.
- Use Linux-first slices as the default opening cohort because SD-12 and the install/use matrix ground Linux as first-class while macOS and Windows remain explicitly bounded.
- Do not include persistence or migration missions unless the specific build brief also names the relevant SD-14 expectations and evidence fields.

## Build / channel / platform launch matrix
| Matrix row | Platform / support tier | Governed channel/support label | Build/provenance expectation | Bounded launch use | Required pre-launch proof | Current launchability rule |
|---|---|---|---|---|---|---|
| LNX-A | Linux / first-class | `alpha` / first-class tester track | Governed GitHub-backed alpha artifact with release metadata, provenance handle, and immutable build identity | Primary tranche-2 external evidence row | Matching install/use matrix row is no longer blocked, named clean-machine receipt exists for the same path, and the artifact handle is fixed in operator instructions | Build/evidence prerequisites are now satisfied; Todd Hintzmann is the intended first tester, but the row remains cycle-blocked until tranche-2 coding is fully complete and the row-specific operator packet is actually issued |
| LNX-S | Linux / first-class | `stable` / first-class tester track | Governed GitHub-backed stable artifact with release metadata, provenance handle, and immutable build identity | Stable-channel confirmation row, never implied by alpha success | Stable row in install/use matrix is grounded, stable clean-machine receipt exists for the same path, and the stable artifact handle is fixed | Remains `not-ready` until stable proof exists independently |
| MAC-A | macOS / second-class | `alpha` / second-class tester track | Governed macOS alpha artifact plus trust/install instructions | Sentinel evidence row only after macOS moves from documentary posture into grounded artifact truth | macOS artifact handle, trust prerequisite instructions, matching install/use row, and matching clean-machine receipt | Remains `not-ready` until macOS row stops being documentary-only |
| MAC-S | macOS / second-class | `stable` / second-class tester track | Governed macOS stable artifact plus trust/install instructions | Stable sentinel evidence row | Same proof burden as MAC-A, but for stable | Remains `not-ready` until separately grounded |
| WIN-A | Windows / third-class | `alpha` / third-class tester track | Governed Windows alpha artifact plus explicit support-bound wording | Third-class evidence row only if the path becomes grounded enough to test truthfully | Windows artifact handle, explicit third-class warning, matching install/use row, and matching clean-machine receipt | Remains `not-ready` until Windows artifact posture is explicit |
| WIN-S | Windows / third-class | `stable` / third-class tester track | Governed Windows stable artifact plus explicit support-bound wording | Third-class stable evidence row | Same proof burden as WIN-A, but for stable | Remains `not-ready` until separately grounded |
| RESERVED-BETA | Any platform | `beta` / reserved candidate label | No governed backing surface exists | None | None | `out-of-scope` until a governed candidate publication surface exists |
| ADHOC-DEV | Any platform | feature-branch or local authoring artifact | Local build or ad hoc artifact not backed by governed tester-channel truth | None for official external testing | None | `forbidden` for tranche-2 external evidence |

## Pre-launch operator packet
Every tester in an actually launched row must receive a row-specific operator packet containing:
1. exact artifact handle or publication URL
2. expected build label/version and tester-facing channel/support label
3. platform/support-tier warning text copied from the relevant matrix row
4. the bounded mission bundle below, with no free-form extras
5. the required evidence checklist and attachment/redaction expectations
6. the GitHub bug/enhancement intake path from SD-11
7. stop-and-escalate instructions naming what should halt the mission immediately

If any of these packet elements is missing, the row stays `not-ready`.

## Bounded mission bundle
### Mission ordering rule
Testers execute missions in order. If a mission stops on a halt condition, later missions do not inherit a fake `not run because probably fine` status.

| Mission ID | Mission | Target workflow | Expected evidence | Primary SD-15 route if it fails | Hard stop condition |
|---|---|---|---|---|---|
| M1 | Acquire governed build | Obtain the exact named artifact from the governed publication path for the assigned matrix row | artifact handle, visible build/download identity, platform/package context, and any acquisition failure evidence | `packaging or distribution defect` or `unsupported semantics or known unsupported paths` if the path is explicitly unavailable | Stop the tester row immediately if the governed artifact cannot be obtained or its identity is ambiguous |
| M2 | Install and first launch | Install or unpack the named artifact, then launch far enough to judge workbench entry truthfully | install transcript or screenshot, launch screenshot, observed versus expected result, logs/diagnostics when present | `install/use defect` or `status or documentation drift` if the published expectations contradict the observed path | Stop later missions if install or first launch cannot be completed truthfully |
| M3 | Confirm visible identity | Verify the app exposes the expected build label, channel/support wording, and visible platform/support posture | screenshot or equivalent visible proof of build/channel/support identity | `status or documentation drift` or `install/use defect` | Stop the row if identity is missing or contradicts the operator packet |
| M4 | Reach bounded tester workbench | Enter the bounded SD-11 tester workbench path defined by the install/use matrix | screenshot or equivalent visible proof of workbench reachability and bounded workflow entry state | `install/use defect` | Stop deeper workflow missions if the workbench cannot be reached |
| M5 | Exercise bounded tranche mission | Load the bounded GE-08 tester-workbench mission identified by current tranche truth: load the Guard Stance proof package and observe package state, preview state, and structured snapshot behavior | screenshots, notes, diagnostics, and observed versus expected behavior tied to the named mission | Route by evidence: `ui or presentation defect`, `rules-engine defect`, `content or data defect`, `unsupported semantics or known unsupported paths`, or `persistence, migration, or saved-state continuity defect` | Stop and escalate when the tester hits a contradiction that prevents truthful continuation or when the path becomes explicitly unsupported |
| M6 | Prove issue-ready evidence capture on any non-pass result | When something blocks, fails, or passes only with known bounds, capture enough evidence that the operator can classify it later without memory | issue payload draft or completed GitHub submission, attachment handles, redaction posture, observed/expected statement, reproduction or impossibility note | `status or documentation drift` if the evidence path itself is unusable; otherwise route by the underlying failure | Stop the row if the tester cannot produce issue-ready evidence for the non-pass state |

### Mission bounds
- Do not ask testers to roam beyond the named bounded mission bundle for the row.
- Do not turn exploratory play, open-ended feature requests, or unsupported roster/persistence experiments into mandatory mission scope.
- If the operator wants persistence, migration, or breadth-specific missions, those must be added as explicit row addenda with the relevant SD-13 or SD-14 authority references before launch.

## Required evidence burden
### Minimum evidence every tester submission must carry
| Evidence field | Why it is required | Authority anchor |
|---|---|---|
| tester-visible build label/version | proves what build was actually exercised | `regression-receipt-schema.md` |
| tester-facing channel/support label | preserves the product-facing support claim seen by the tester | `regression-receipt-schema.md`; SD-11/SD-12 channel language |
| platform/OS and package/install context | distinguishes platform-specific installation and launch failures | `regression-receipt-schema.md`; `tranche-2-install-and-use-matrix.md` |
| bounded workflow / mission under test | proves which mission the report answers | `regression-receipt-schema.md`; this plan |
| observed behavior and expected behavior | supports later classification without memory or paraphrase drift | `regression-receipt-schema.md` |
| reproduction steps or impossibility note | distinguishes reproducible failure from blocked proof path | `regression-receipt-schema.md` |
| screenshot, log, or visible status evidence when present | prevents unsupported narrative-only triage | `regression-receipt-schema.md`; SD-11 evidence posture |
| attachment handles and redaction posture | preserves what exists, what was withheld, and why | `regression-receipt-schema.md`; SD-11 evidence posture |
| adjacent support-state or persistence context when relevant | prevents SD-13 or SD-14 issues from being flattened into generic bugs | `triage-class-dictionary.md` |

### Evidence partition rule
- tester-supplied: what the tester observed, expected, attempted, and attached
- auto-captured: build/channel/workflow/platform fields when the product or intake path can capture them
- operator-added: primary SD-15 class, outcome state, adjacent-authority references, sufficiency note, and next required surface

### Evidence conversion rule
Any non-pass external-test result must be convertible into either:
- an SD-15 regression receipt with the schema fields above, or
- a row-level note that explicitly preserves why the result remained `unsupported`, `partial`, `blocked`, or `not-yet-verified`

## Unsupported-path and support-state warnings
These warnings must be visible in the operator packet before a tester starts.

- Linux is first-class; macOS is second-class but real; Windows is explicitly third-class. Do not rewrite those tiers into fake parity.
- `beta` is reserved and unavailable. No tester may be briefed as a `beta` tester until a governed candidate backing surface exists.
- Local developer builds, feature-branch artifacts, and authoring-machine outputs do not count as official tranche-2 external-test inputs.
- SD-13 breadth/progression bounds remain real. Unsupported or partially supported class/race/level semantics must remain visible instead of being reported as ordinary product regressions by default.
- SD-14 persistence and migration bounds remain real. Save/load/reopen/revise/update-survival claims require the relevant persistence context and should not be inferred from ordinary launch success.
- A platform row that remains `blocked` in the install/use matrix is not secretly launchable just because a tester is willing to try it.
- A missing clean-machine receipt for the same path is a launch refusal, not a paperwork gap.

## Stop conditions and escalation rules
### Row-level stop conditions
| Condition | Immediate action | Primary SD-15 route |
|---|---|---|
| Governed artifact cannot be acquired or the provenance is ambiguous | Halt the tester row; do not continue with install or workflow missions | `packaging or distribution defect` |
| The visible build/channel/support identity contradicts the operator packet or adjacent truth surfaces | Halt the tester row and escalate immediately | `status or documentation drift` |
| The tester hits an unsupported-path warning that was missing, unclear, or contradicted by durable surfaces | Halt the affected mission and escalate the documentation/support contradiction | `unsupported semantics or known unsupported paths` or `status or documentation drift` |
| Install, launch, or workbench entry fails before the bounded mission can start | Halt deeper missions for that tester until the failure is classified | `install/use defect` |
| A non-pass result cannot be converted into issue-ready evidence | Halt the row because the cycle has ceased to produce closure-usable truth | `status or documentation drift` |

### Cycle-level stop conditions
| Condition | Cycle action |
|---|---|
| No active matrix row still satisfies governed artifact identity + clean-machine proof + evidence-path availability | Set cycle state to `halted` or `not-ready`; do not keep testers running on stale assumptions |
| The operator cannot meet the triage cadence below during the active evidence window | Pause new tester assignments until cadence coverage is restored |
| Repeated reports show the same missing warning, wrong channel/support label, or contradictory status claim | Pause the affected row and repair the truth surface before more external evidence is collected |
| The cycle starts depending on ad hoc artifacts or undocumented mission drift | Halt the cycle and reissue a governed operator packet |

### Escalation timing
- immediate escalation: packaging/distribution blockers, provenance ambiguity, status/documentation drift, or missing/contradictory unsupported-path warnings
- same-day triage escalation: install/use defects that block workbench entry or repeat across more than one tester on the same row
- next scheduled triage sweep: bounded in-workflow defects with complete evidence and no platform/channel contradiction
- documentation review escalation: repeated reports that are actually unsupported or partial paths but were not warned clearly enough up front

## Operator triage cadence
| Cadence checkpoint | Minimum action |
|---|---|
| Launch-opening sweep | Verify the active row still has the same governed artifact handle, the same packet revision, and the same clean-machine basis before any new tester begins |
| Mid-window sweep | Review incoming reports for misrouted class, missing evidence, or repeated unsupported-path confusion before more testers are added |
| End-of-day sweep during active cycle days | Reconcile every new report into receipt-ready status: `defect`, `unsupported`, `partial`, `not-yet-verified`, `blocked`, or `status-drift` |
| Immediate ad hoc sweep | Required whenever a packaging/provenance/status contradiction or missing-warning event arrives |

### Cadence policy
- Minimum cadence during active external-test days: opening sweep, one mid-window sweep, and one end-of-day sweep.
- Do not batch packaging/provenance contradictions until the end of the day.
- Repeated unsupported-path reports must be reviewed to decide whether the problem is true product behavior, documentation drift, or insufficient pre-launch warning text.
- Triage output must preserve adjacent-authority provenance so SD-13 and SD-14 bounds are not lost inside generic bug language.

## Explicit not-ready-to-launch states
A row or cycle remains `not-ready` when any of the following is true:

| Not-ready state | Why launch is refused |
|---|---|
| `pending-governed-artifact` | The exact tester-facing build/publication handle is not fixed |
| `pending-clean-machine-receipt` | No matching clean-machine receipt exists for the same path being offered to testers |
| `pending-matrix-grounding` | The corresponding install/use matrix row is still `blocked`, `unsupported`, or `out-of-scope` |
| `pending-mission-freeze` | The operator packet still asks for vague exploration instead of named missions |
| `pending-evidence-path` | Testers do not yet have a usable screenshot/log/issue-capture route |
| `pending-warning-surface` | Support-tier or unsupported-path warnings are not visible enough up front |
| `pending-triage-coverage` | No operator cadence exists to classify results during the active cycle window |
| `pending-adjacent-context` | The row requires SD-13 or SD-14 context that has not been attached to the packet |

## Section-6 requirement crosswalk
| Technical requirement from `technical-requirements.md` section 6 | Where this plan satisfies it |
|---|---|
| target tester cohort and selection constraints | `Target tester cohort and selection constraints` |
| build/channel/platform matrix for the cycle | `Build / channel / platform launch matrix` |
| bounded missions or workflows testers are asked to exercise | `Bounded mission bundle` |
| the evidence they must capture | `Required evidence burden` and mission evidence columns |
| support-state or unsupported-path warnings they must see up front | `Unsupported-path and support-state warnings` |
| stop conditions and escalation rules | `Stop conditions and escalation rules` |
| operator triage cadence during the cycle | `Operator triage cadence` |
| what conditions make the cycle not ready to launch | `Explicit not-ready-to-launch states` |

## Explicit refusals
- do not treat this plan as evidence that any external tester already ran
- do not soften `not-ready` states into optimism because the plan surface itself is complete
- do not use "get feedback" as a substitute for the named mission bundle and evidence packet
- do not allow clean-machine absence, artifact ambiguity, or support-warning gaps to survive as informal operator caveats
- do not let external evidence erase the difference between `defect`, `unsupported`, `partial`, `blocked`, `not-yet-verified`, and `status-drift`
- do not permit tranche-closure claims from this plan alone
