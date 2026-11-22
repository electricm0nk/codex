---
title: GE-05 Execution Route Surface
artifact_type: execution-route-surface
stc_id: STC-CODEX-GE-05
source_stc: ./README.md
workflow_route: coding
readiness: codex-ready
status: running-under-card-triggered-harness
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/execution-handoff.md
code_authority: false
active_handoff:
  - artifacts/ge05-e2-f2-execution-handoff-2026-06-24.md
active_kanban_card:
  - t_0cdc64d0
active_readiness_closure:
  - artifacts/ge05-e2-f2-execution-readiness-closure-2026-06-24.md
last_completed_handoff:
  - artifacts/ge05-e2-f1-execution-handoff-2026-06-20.md
  - artifacts/ge05-e2-f1-merge-receipt-2026-06-21.md
next_candidate_slice: GE05-E2-F2 — PF1 Human Fighter level 1 governed fixture instance is the active launch-ready coding packet
next_handoff_required: false
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  verified_merge_commit: a2c7e8812b47d7ade228d9fbc3f65442594f2228
  observed_origin_develop: 7bc89e8c1edf8f1d1a6d490a0ad28ac72fc6f104
---

# GE-05 Execution Route Surface

## Current state
There is now **one active GE-05 stage-specific code-authorizing handoff running under a Kanban CODE-card-triggered harness lane**.

The active live pair and board trigger are:

```text
GE05-E2-F2 — PF1 Human Fighter level 1 governed fixture instance
programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e2-f2-execution-readiness-closure-2026-06-24.md
programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e2-f2-execution-handoff-2026-06-24.md
kanban://codex-phase-2/t_0cdc64d0
```

This root file remains a **stable route surface**, not the code-authorizing brief itself. The stage-specific handoff above carries `code_authority: true`.

## Why this route surface exists
The previous GE05-E2-F1 slice merged correctly, but the root surface was still parked at `no-active-handoff` while the next bounded coding move remained only an inferred candidate.

That ambiguity is now closed. The GE05-E2-F2 readiness pass established exact repo truth, exact branch/base truth, exact write scope, exact required reads, and exact verification commands. The result is one real launch-ready coding packet rather than another planning-shaped suggestion.

## Active coding packet
```text
GE05-E2-F2 — PF1 Human Fighter level 1 governed fixture instance
```

Current posture:
- GE05-E2-F1 schema/loader/sample-fixture work is merged and preserved as historical authority
- the deterministic pilot-input contract is accepted under GE-06 and closes the provisional pilot-input choices that F1 could only record as assumptions
- the merged GE06-E3-F1 selected-parity carrier proves current new-system `Computed` evidence exists without claiming oracle parity
- the GE05-E2-F2 execution-readiness closure now exists and is `codex-ready`
- the GE05-E2-F2 stage-specific execution handoff now exists and is `running-under-card-triggered-harness` via `kanban://codex-phase-2/t_0cdc64d0`
- write scope is confined to the golden fixture lane: `src/oracle_validation/golden_fixture.rs`, `tests/golden_case_fixture_schema.rs`, and `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt`
- the governed Kanban CODE card is the launch trigger for Claude Code / the frontier coding harness

## Not active yet
The following GE-05 terrain remains non-authorizing until E2-F2 lands and produces new repo truth:

- GE05-E3 — output capture and normalization lanes
- GE05-E4 — comparator, diff, and parity-report lanes
- GE05-E5 — known-gap ledger and intentional-divergence routing
- GE05-E6 — repeatable headless verification / GE-06 consumable parity evidence

## Automation and human gates
The automation boundary remains explicit:
- the governed Kanban CODE card launches the coding harness automatically
- the card must record a durable Claude execution receipt before claiming Claude/Opus execution
- Todd retains merge/review authority after the branch or PR exists
- this route surface should move to `running-under-card-triggered-harness`, `awaiting-todd-merge`, or `no-active-handoff` only when evidence exists

At the moment, the truthful state is `running-under-card-triggered-harness` through `kanban://codex-phase-2/t_0cdc64d0`.

## Truth-surface warning carried forward
The stale crate-level prose in `src/lib.rs` still exists. It remains a real truth-surface defect, but this E2-F2 handoff does not authorize writing `src/lib.rs`.

Do not open a standalone cleanup lane for that prose. Repair it only when a later bounded slice already needs to touch `src/lib.rs` for real implementation reasons.

## Rule for future stages
Do not retarget this file into a stage brief.

Any later GE-05 code-authorizing work must be minted as a new stage-specific handoff artifact under `artifacts/`, paired with its own readiness closure and later merge receipt.