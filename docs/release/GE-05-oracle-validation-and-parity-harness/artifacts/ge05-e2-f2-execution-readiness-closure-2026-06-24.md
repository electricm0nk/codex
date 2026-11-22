---
title: GE05-E2-F2 Execution Readiness Closure
artifact_type: execution-readiness-closure
stc_id: STC-CODEX-GE-05
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE05-E2-F2 — PF1 Human Fighter level 1 governed fixture instance
workflow_route: readiness-closure
readiness: codex-ready
handoff_created: true
created_handoff:
  - ./ge05-e2-f2-execution-handoff-2026-06-24.md
review_date: 2026-06-24
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE05-E2-F2 Execution Readiness Closure

## Verdict
GE-05 is now grounded sufficiently to mint the next narrow code-producing handoff for the first governed PF1 Human Fighter level 1 fixture instance, and that paired artifact now exists.

The active E2-F2 code-authorizing artifact created from this readiness closure is:

```text
programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e2-f2-execution-handoff-2026-06-24.md
```

This readiness closure is not code authority. It records why the separate E2-F2 handoff now truthfully carries `code_authority: true` while the root `execution-handoff.md` remains only a route surface.

## Core problem
GE05-E2-F1 merged the schema, loader, and one repo-local sample fixture, but the sample still records provisional pilot-input assumptions rather than the now-accepted deterministic pilot contract. The result is a truthful seed, not yet the first governed case instance.

The smallest honest next move is not comparator work, not normalization, and not report writing. It is one bounded fixture-instance slice that upgrades the existing seed fixture to governed-input truth, preserves the already-captured GE05-E1-F2 oracle evidence exactly, and keeps any still-missing Codex/parity outputs explicitly unresolved or blocked instead of fabricating a pass.

## Selected bounded slice

```text
GE05-E2-F2 — PF1 Human Fighter level 1 governed fixture instance
```

This slice should do only four things:

1. reuse the existing repo-local golden fixture as seed evidence rather than inventing a second first-case representation
2. ground inherited character inputs from the accepted deterministic pilot contract
3. preserve the existing GE05-E1-F2 legacy oracle route, reduced-facts reference, retention posture, and SHA-256 unchanged
4. keep every output/evidence field below `OracleChecked` unless a stable already-grounded repo-local reference exists; otherwise record explicit unresolved or blocked truth plus known-gap posture

This slice does not authorize GE05-E3 output-capture implementation, normalization rules, comparator logic, report writing, GE-06 code changes, PCGen repo writes, UI work, or release work.

## Required source evidence recovered

| Gate | Evidence |
|---|---|
| Prior GE-05 coding foothold is merged | `artifacts/ge05-e2-f1-merge-receipt-2026-06-21.md` proves the schema/loader/sample-fixture slice merged on `develop` and remains an ancestor of current `origin/develop`. |
| GE05-E2-F2 acceptance is explicit | `epic-breakdown.md` requires the first fixture to stay scoped to `pf1-crb-human-fighter-level1`, ground inherited character inputs from the pilot charter, and either link final expected values to captured output or mark them unresolved. |
| Deterministic pilot input closure now exists | `../GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md` closes the Human ability bonus, feat slots, skill ranks, equipment loadout, and active-state choices that were provisional in the F1 fixture. |
| Selected parity-dimension evidence now exists as merged repo truth | `../GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f1-merge-receipt-2026-06-22.md`, `src/oracle_validation/selected_parity_dimensions.rs`, and `tests/ge06_selected_parity_dimensions.rs` prove the repo now carries machine-checkable `Computed` evidence for the selected pilot dimensions without claiming oracle parity. |
| Current repo state is safe to branch from fresh `origin/develop` | Live verification on 2026-06-24 found a clean worktree in `/home/ubuntu/workspace/repos/codex`, current local branch `ge06-e3-f2-classifier-impl`, and `origin/develop` at `7bc89e8c1edf8f1d1a6d490a0ad28ac72fc6f104`; the checked-out branch is behind `origin/develop`, so E2-F2 must branch fresh from `origin/develop`. |
| Baseline proof commands already pass before the slice starts | `"$HOME/.cargo/bin/cargo" test --test golden_case_fixture_schema --quiet` and full `"$HOME/.cargo/bin/cargo" test --quiet` both pass against the current repo. |
| Repo conduct surface demands an exact handoff | `/home/ubuntu/workspace/repos/codex/AGENTS.md` requires exact objective, exact write scope, exact required reads, and explicit verification commands before implementation begins. |

## Grounded implementation posture
Because the repo now has:

- one merged golden-fixture schema and loader
- one repo-local seed fixture already tied to real GE05-E1-F2 oracle evidence
- one accepted deterministic pilot contract closing the previously provisional inputs
- one merged selected-parity carrier proving current new-system `Computed` evidence exists elsewhere in the repo
- and a clean branch point on `origin/develop`

...the smallest truthful implementation is:

1. update the existing fixture file to represent the governed first-case identity and closed pilot-input posture
2. tighten `tests/golden_case_fixture_schema.rs` first in RED/GREEN fashion so the governed fixture requirements are machine-checked
3. make only the minimal `golden_fixture.rs` changes needed to support that governed representation if the existing loader proves too narrow

## Exact write surface justified by the evidence
The handoff created from this closure confines implementation to:

```text
src/oracle_validation/golden_fixture.rs
tests/golden_case_fixture_schema.rs
tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt
```

That scope is sufficient because the public module exposure already exists and the repo already has both the deterministic input fixture and the GE-06 selected-parity precedent available as read-only upstream truth.

## Branch / runtime rule
The live checkout is not the branch this slice should continue from. The coding harness must branch fresh from current `origin/develop`, not from the checked-out `ge06-e3-f2-classifier-impl` branch.

## Verification contract for the coding harness
The paired execution handoff requires at minimum:

```bash
"$HOME/.cargo/bin/cargo" test --test golden_case_fixture_schema --quiet
"$HOME/.cargo/bin/cargo" test --test ge06_selected_parity_dimensions --quiet
"$HOME/.cargo/bin/cargo" test --quiet
```

The targeted GE-05 test proves the governed fixture lane directly. The GE-06 selected-dimension test and the full suite prove the slice did not damage the upstream computed-evidence foothold it depends on.

## Handoff minting decision
The analysis is complete. The blocker is gone.

GE05-E2-F2 is now a truthful, bounded, launch-ready coding slice. The route surface should advance from `no-active-handoff` to `awaiting-kanban-card-dispatch`, paired with a stage-specific execution handoff and a board-visible CODE card that triggers Claude Code / frontier-harness execution automatically.