---
title: GE-08 Execution Route Surface
artifact_type: execution-route-surface
stc_id: STC-CODEX-GE-08
source_stc: ./README.md
workflow_route: coding
readiness: codex-ready
status: no-active-handoff
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/execution-handoff.md
code_authority: false
active_handoff: []
active_readiness_closure: []
last_completed_handoff:
  - artifacts/ge08-e4-f1-branch-ready-receipt-2026-06-27.md
historical_prebuild_artifacts:
  - artifacts/ge08-e4-f1-prebuild-readiness-closure-2026-06-27.md
  - artifacts/ge08-e4-f1-prebuild-handoff-2026-06-27.md
next_candidate_slice: GE08-E5-R2 — Desktop authoring boundary and native verification closure is the next honest GE-08 route surface
next_handoff_required: true
intended_executor: Claude Code
execution_origin_rule: Without a durable successful Claude completion receipt, future GE-08 implementation claims must be classified as Claude-launched mixed-custody reconciled, Hermes-executed, or unverified — never casually as pure Claude execution.
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: develop
  accepted_substrate_commit: 9c883dc1ee3e8a7de35ecea9cf84bd6c9611cb1f
  observed_origin_develop: cc4e1a55caad07af83768a036d4b0f5fffbf99c9
---

# GE-08 Execution Route Surface

## Current state
There is **no active GE-08 code-authorizing handoff** right now.

GE08-E4-F1 has already been launched, implemented to branch-ready state, verified, pushed, and opened for review. The durable evidence set is now:

```text
programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e4-f1-branch-ready-receipt-2026-06-27.md
programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e4-f1-pr-created-receipt-2026-06-27.md
```

## Last completed coding packet
```text
GE08-E4-F1 — Headless preview and explanation bridge for first proof package
branch: ge08-e4-f1-preview-and-explanation-bridge
commit: 9c883dc1ee3e8a7de35ecea9cf84bd6c9611cb1f
pull request: https://github.com/electricm0nk/codex/pull/20
```

Truth posture:
- GE08-E2-F1 remains historically Hermes-executed, not Claude-executed
- GE08-E3-F1 has durable branch-ready evidence at `ge08-e3-f1-validation-and-diagnostics@6de72fcd2ba2bacc79bc4ae4f0a8bf163c875606`
- GE08-E4-F1 was launched in Claude and then reconciled by Hermes because the surviving Claude result ended with `error_max_turns` instead of a clean final success receipt
- the resulting GE08-E4-F1 repo truth is durable on origin at `ge08-e4-f1-preview-and-explanation-bridge@9c883dc1ee3e8a7de35ecea9cf84bd6c9611cb1f`
- PR `#20` is open and mergeable, but Todd still performs merges manually

## Latest route decision
GE08-E5-R1 is now closed with a blocked readiness verdict at:

```text
programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e5-r1-execution-readiness-closure-2026-06-27.md
```

That closure disproves the stale “missing GE07 scaffold” blocker and identifies the real remaining boundary: the desktop shell still lacks a truthful GE08 authoring command contract, and this host cannot yet verify Tauri-side boundary changes because native desktop checks fail before the seam can be exercised.

## Next honest route
The next route surface is therefore not a code-authorizing GE08-E5-F1 handoff yet. It is:

```text
GE08-E5-R2 — Desktop authoring boundary and native verification closure
```

That route must:
- ground the first truthful GE08 desktop request/response contract over the existing headless authoring substrate
- repair or explicitly designate the native verification environment needed to validate the Tauri-side boundary
- only then decide whether a bounded non-production GE08-E5-F1 code handoff is honest

## Human gates
The automation boundary remains explicit:
- Todd launched the E4 Claude packet manually
- Hermes verified, pushed, and opened PR `#20`
- Todd still owns the merge decision for PR `#20`
- no later GE08 code-authorizing handoff may be minted from E5 until the R2 closure resolves the shell-boundary and native-verification gap truthfully

## Rule for future stages
Do not retarget this file into a stage brief.

Any later GE-08 code-authorizing work must be minted as a new stage-specific handoff artifact under `artifacts/`, paired with its own readiness closure and later branch-ready/merge receipt.
