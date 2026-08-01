---
title: GE06-E2-F1 Execution Readiness Closure
artifact_type: execution-readiness-closure
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE06-E2-F1a — Deterministic pilot input contract fixture load gate
workflow_route: readiness-closure
readiness: codex-ready
handoff_created: true
created_handoff: ../execution-handoff.md
review_date: 2026-06-21
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE06-E2-F1 Execution Readiness Closure

## Verdict
GE-06 may now advance to one narrow code-producing handoff.

The authorized handoff is:

```text
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/execution-handoff.md
```

This closure itself is not code authority. It records why the separate handoff may carry `code_authority: true`.

## Core problem
GE-06 needs a headless implementation foothold that consumes the final deterministic pilot input contract without pretending to compute Pathfinder outputs, run oracle parity, or build UI.

## Selected bounded slice

```text
GE06-E2-F1a — Deterministic pilot input contract fixture load gate
```

This is a deliberately narrower implementation slice derived from GE06-E2-F1 and GE06-E2-F3:

- GE06-E2-F1 asks for an integrated source/load contract.
- GE06-E2-F3 asks for one bounded command or test path that can produce integrated headless evidence or a blocker receipt.
- The final deterministic input contract says the first implementation artifact may target headless fixture/input representation and validation before computation or parity.

The selected first code slice is therefore: **represent and validate the accepted GE-06 deterministic pilot input contract as headless character input.**

## Required source evidence recovered

| Gate | Evidence |
|---|---|
| Source STC exists | `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md` |
| Final deterministic input contract exists | `artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md` |
| Target repo exists | `/home/ubuntu/workspace/repos/codex` |
| Repo instruction surface exists | `/home/ubuntu/workspace/repos/codex/AGENTS.md` |
| Current branch | `develop` |
| Local head | `a2c7e88` |
| `origin/develop` | `a2c7e88` |
| GE-05 merge present | `acf6ad4` is contained in `origin/develop` as established during GE-06 final documentary pass |
| Toolchain available | `cargo` at `/home/ubuntu/.cargo/bin/cargo` under login shell |
| Baseline tests | `cargo test --quiet` passes on `develop` |
| Existing character input surface | `src/rules_core/character_input.rs` and `tests/character_input_record.rs` |
| Existing GE-04 fixture | `tests/fixtures/rules_core/pf1_human_fighter_level1_minimal_character_input.txt` |
| Existing GE-05 fixture schema | `src/oracle_validation/golden_fixture.rs` |

## Gate table

| Gate | Status | Resolution |
|---|---|---|
| Bounded implementation slice selected | pass | GE06-E2-F1a is limited to loading/validating the deterministic pilot input contract. |
| Target repo/workdir exists | pass | `/home/ubuntu/workspace/repos/codex`. |
| Branch policy explicit | pass | Start from clean, current `origin/develop`; create a feature branch targeting `develop`. |
| Allowed write scope explicit | pass | Limited to rules-core character input tests/fixture and minimal supporting implementation; no UI, oracle comparator, importer expansion, or PCGen writes. |
| Runtime instruction surface exists | pass | Repo `AGENTS.md` exists and requires strict TDD. |
| Toolchain grounded | pass | `cargo test --quiet` runs and passes on current `develop`. |
| Verification commands known | pass | Specific test command plus full `cargo test --quiet`. |
| Unresolved input choices remain | pass | None for first pilot input; final numeric runtime values remain out of scope. |
| Non-goals explicit | pass | Computation, parity, UI, old-system command execution, and broad Pathfinder support excluded. |
| Harness route explicit | pass | `execution-handoff.md` is for Claude Code / frontier coding harness, not Hermes. |

## Pre-existing repo residue

The target repo currently reports these untracked files before this handoff:

```text
?? AGENTS.md
?? CLAUDE.md
?? Cargo.lock
```

Treatment:

- `AGENTS.md` and `CLAUDE.md` are instruction surfaces already present in the repo directory; the coding harness may read them.
- The handoff does not authorize adding, deleting, or modifying those files.
- `Cargo.lock` is pre-existing residue; this handoff does not authorize deleting it or treating it as evidence of new work.
- Scope audit must compare individual changed files, not merely directory-level status noise.

## Authorized implementation objective

Create a headless test/fixture path that proves Codex can represent and validate the accepted GE-06 deterministic pilot input contract as character input.

The implementation must prove the following input facts are represented, not computed:

- case identity / source package identity
- Human Fighter level 1
- final ability scores: STR 16, DEX 14, CON 14, INT 10, WIS 12, CHA 8
- selected feats: Power Attack, Dodge, Weapon Focus (Longsword)
- selected choices needed to explain those feat slots and Human ability bonus
- skill ranks: Climb 1, Intimidate 1, Swim 1
- equipment: Chain Shirt active/worn, Longsword active/primary, no shield
- active state: Power Attack selected but inactive for baseline outputs
- provenance references tying the fixture back to the GE-06 final deterministic input contract and source evidence

## Authorized write scope for the derived handoff

The derived handoff may authorize writes only to these repo paths:

```text
tests/ge06_pilot_input_contract.rs
tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt
src/rules_core/character_input.rs
```

Conditional write, only if Rust module exposure requires it:

```text
src/rules_core/mod.rs
```

No other writes are authorized.

## Required TDD posture

The coding harness must:

1. create or update the failing GE-06 test first
2. run the specific test and capture the RED failure
3. implement the smallest loader/model change needed to pass
4. run the specific test and capture GREEN
5. run full `cargo test --quiet`
6. run a file-granular scope audit

## Explicit non-goals

The derived handoff must not authorize:

- computing BAB, saves, armor class, attack bonus, skill modifiers, or armor-check effects
- creating a rules engine evaluator
- creating an importer expansion
- creating an oracle comparator
- creating a normalization engine
- creating a parity report writer
- running PCGen
- modifying `/home/ubuntu/workspace/repos/pcgen`
- modifying GE-05 historical fixtures to hide provisional assumptions
- UI, Tauri, React, desktop shell, screenshots, or product-visible claims
- broad Pathfinder support
- final numeric expected values beyond the input values named in the deterministic contract

## Claim tier after this slice

If the handoff succeeds, GE-06 may claim:

```text
selected pilot input contract: represented
headless input load gate: tested
computed Pathfinder outputs: not yet
oracle-checked parity: not yet
product-visible UI: not yet
```

## Why lesser approaches fail

The obvious but wrong move is to tell the coding harness to “build the headless integrated path.” That would collapse fixture representation, source import, rules computation, oracle comparison, and receipt generation into one counterfeit sprint.

The decisive move is smaller: first make the deterministic input contract executable as a headless fixture. Once that is real, computation can stand on something stable.

## Completion rule

This closure is complete when `execution-handoff.md` exists, carries `code_authority: true`, names the narrow GE06-E2-F1a objective, enforces strict TDD, lists exact allowed repo paths, excludes computation/parity/UI, and gives runnable verification commands.
