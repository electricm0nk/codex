---
title: GE06-E2-F2c Execution Readiness Closure
artifact_type: execution-readiness-closure
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE06-E2-F2c — Total Fortitude, Reflex, and Will saving throws under deterministic ability scores
workflow_route: readiness-closure
readiness: codex-ready
handoff_created: true
created_handoff:
  - artifacts/ge06-e2-f2c-execution-handoff-2026-06-21.md
review_date: 2026-06-21
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE06-E2-F2c Execution Readiness Closure

## Verdict
GE-06 has now advanced to one new narrow code-producing handoff.

That handoff now exists at:

```text
artifacts/ge06-e2-f2c-execution-handoff-2026-06-21.md
```

This closure itself is still not code authority. It records why that separate stage-specific F2c handoff may carry `code_authority: true`, and it makes one additional requirement explicit: **the F2c coding slice must repair the stale prose in `src/rules_core/pilot_compute.rs` so the file-level comments match the post-F2b behavior already merged into `develop`.**

## Core problem
GE06-E2-F2b proved the first deterministic combat-facing totals: baseline melee attack bonus and baseline armor class. Codex still cannot compute the first deterministic **total** saving throws for the accepted pilot, and the current `pilot_compute.rs` file still contains F2a-era prose that falsely says the surface does not compute armor class or attack bonus.

## Selected bounded slice

```text
GE06-E2-F2c — Total Fortitude, Reflex, and Will saving throws under deterministic ability scores
```

This slice is deliberately narrow. It computes only:

- total Fortitude save for the deterministic pilot
- total Reflex save for the deterministic pilot
- total Will save for the deterministic pilot
- explanation records for each bounded output naming the exact contributors
- explicit claim-blocking diagnostics when an unsupported posture would force dishonest broadening
- mandatory file-level prose synchronization inside `src/rules_core/pilot_compute.rs` so module comments and struct comments truthfully describe the currently supported GE-06 outputs after F2b and F2c

This slice does **not** compute feat-based save modifiers, item-based save modifiers, conditional or situational modifiers, damage, active Power Attack math, initiative, skill modifiers, parity, UI, or broader rules-engine breadth.

## Required source evidence recovered

| Gate | Evidence |
|---|---|
| Prior merged foothold | GE06-E2-F2b merged into `origin/develop` at `75c26ce`. |
| Target repo exists | `/home/ubuntu/workspace/repos/codex`. |
| Branch policy | Start from current `origin/develop`, target PR to `develop`. |
| Baseline tests | `"$HOME/.cargo/bin/cargo" test --quiet` passes on a detached worktree at `origin/develop` commit `75c26ce`. |
| Existing compute surface | `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`. |
| Existing prior-proof tests | `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_base_computation.rs` and `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`. |
| Input fixture | `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`. |
| Base save grounding already merged | `class_chassis.base_save.fortitude`, `class_chassis.base_save.reflex`, and `class_chassis.base_save.will` are already produced by the current compute surface from grounded Fighter level-1 chassis values. |
| Deterministic ability modifiers already merged | Current compute surface already yields CON `+2`, DEX `+2`, and WIS `+1` for the accepted deterministic pilot input. |
| GE-04 computation requirements | `../GE-04-rules-engine-and-explainability-core/technical-requirements.md` TR-04-010 and TR-04-011. |
| Known prose mismatch grounded | `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs:1-8` and `:14` still describe the file as an F2a-only surface and still claim it does not compute armor class or attack bonus, which is false after merged F2b. |

## Grounded expected values

Using the accepted deterministic input fixture:

### Total Fortitude save

```text
Fighter base Fortitude (+2) + Constitution modifier (+2) = 4
```

### Total Reflex save

```text
Fighter base Reflex (+0) + Dexterity modifier (+2) = 2
```

### Total Will save

```text
Fighter base Will (+0) + Wisdom modifier (+1) = 1
```

These values are local computed outputs only. They are not oracle-checked parity.

## Gate table

| Gate | Status | Resolution |
|---|---|---|
| Prior foothold merged | pass | GE06-E2-F2b is on `origin/develop` at `75c26ce`. |
| Bounded implementation slice selected | pass | GE06-E2-F2c is limited to total Fortitude, Reflex, and Will saves under deterministic ability scores only. |
| Target repo/workdir exists | pass | `/home/ubuntu/workspace/repos/codex`. |
| Branch policy explicit | pass | Branch from current `origin/develop`, PR to `develop`. |
| Allowed write scope explicit | pass | Extend `pilot_compute.rs`, including mandatory prose synchronization in that same file, and add one new GE-06 total-saves test only. |
| Runtime instruction surface exists | pass | Repo `AGENTS.md` exists and requires strict TDD. |
| Toolchain grounded | pass | Full test suite passes when invoked as `"$HOME/.cargo/bin/cargo" test --quiet`; current shell PATH does not expose `cargo` by default. |
| Verification commands known | pass | Exact per-test and full-suite commands are named below. |
| Prose repair obligation explicit | pass | The next coding slice must fix stale F2a-only module/struct prose in `pilot_compute.rs`; this is part of truthful scope, not optional polish. |
| Non-goals explicit | pass | Excludes feat/item/conditional save modifiers, damage, active Power Attack math, skills, parity, importer expansion, and UI. |
| Harness route explicit | pass | The future stage-specific execution handoff runs in Claude Code / equivalent frontier coding harness. |

## Authorized write scope for the derived handoff

The derived handoff may authorize writes only to:

```text
src/rules_core/pilot_compute.rs
tests/ge06_pilot_total_saves.rs
```

Inside `src/rules_core/pilot_compute.rs`, the derived handoff must treat these as **one scope**:

1. add the smallest truthful total-save computation support
2. add the matching explanation/diagnostic support
3. update stale module/struct/file prose so the comments no longer falsely describe the file as F2a-only or claim that armor class / attack bonus are unsupported

It may read but must not modify the prior proof files:

```text
tests/ge06_pilot_base_computation.rs
tests/ge06_pilot_combat_baseline.rs
tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt
src/rules_core/character_input.rs
```

If a compile break proves another file is required, stop and report the blocker rather than widening silently.

## Required TDD posture

The coding harness must:

1. create the failing GE-06 total-saves test first
2. run the specific test and capture RED
3. implement the smallest additions inside the existing `pilot_compute.rs` surface needed to pass
4. repair the stale module/struct prose in that same file before declaring GREEN
5. run the specific test and capture GREEN
6. re-run the prior F2a proof test
7. re-run the prior F2b combat-baseline proof test
8. run full `"$HOME/.cargo/bin/cargo" test --quiet`
9. run a file-granular scope audit

## Expected implementation shape

Preferred outcome:

- extend `PilotBaseChassisComputation` with one bounded total-save surface rather than creating a broad new engine layer
- preserve the existing headless GE-06 pilot compute entry point
- keep total-save support limited to the accepted deterministic pilot and grounded base-save-plus-ability-modifier arithmetic
- update file prose to reflect the actual post-F2b/post-F2c supported outputs and remaining non-goals

Suggested shape:

```rust
total_saves: BaseSaves
```

Equivalent naming is allowed only if it is smaller, clearer, and machine-checkable in tests.

## Explanation requirements

The computation result must include explanation records sufficient for tests to assert why each new bounded total exists.

At minimum, add machine-checkable explanation ids or equivalent fields for:

```text
defense.total_save.fortitude
defense.total_save.reflex
defense.total_save.will
```

The explanation detail for each total save must mention:

- the grounded Fighter base save value
- the relevant ability modifier added to that base save
- the final total

Tests must assert contributors by ids and/or detail content, not by vague prose only.

## Diagnostic requirements

The result must preserve explicit diagnostics for unsupported inputs that would make this narrow total-save slice dishonest.

Minimum diagnostic behavior:

- if the deterministic Fighter level-1 chassis is absent, do not silently compute total saves
- do not broaden into feat-, item-, or condition-based save modifiers
- when total saves are unsupported, do not emit total-save explanation records that pretend those totals were grounded

You may preserve already-supported F2a and F2b outputs and explanations where appropriate.

## Explicit non-goals

The derived handoff must not authorize:

- feat-based save modifiers
- cloak/ring/item-based save modifiers
- temporary or situational bonuses
- condition-based modifiers
- damage rolls
- active Power Attack penalties or bonuses
- initiative
- skill modifiers
- armor-check penalties
- encumbrance or inventory breadth
- imported source-package conversion
- oracle comparison or claim `Oracle-checked`
- normalization engine
- parity report writer
- PCGen execution
- UI, view-model, desktop shell, or export-sheet work

## Claim tier after this slice

If the handoff succeeds, GE-06 may claim:

```text
selected pilot input contract: represented
ability modifiers: computed with explanations
Fighter base BAB/save chassis: computed with explanations
baseline melee attack bonus: computed with explanation
baseline armor class: computed with explanation
total Fortitude / Reflex / Will saves: computed with explanation
feat/item/conditional save modifiers, damage, parity, UI: not yet
```

## Why lesser approaches fail

The lesser models would treat the prose mismatch as cleanup for later. That is how documentary drift becomes operational drift.

The correct move is narrower and harsher: when F2c edits `pilot_compute.rs`, it must leave the file more truthful than it found it. A coding slice that adds save totals while preserving false file-level claims about what the surface computes is counterfeit legibility.

## Completion rule

This closure is complete because `artifacts/ge06-e2-f2c-execution-handoff-2026-06-21.md` now exists, carries `code_authority: true`, names GE06-E2-F2c, enforces strict TDD, lists exact allowed repo paths, includes the mandatory `pilot_compute.rs` prose-sync repair, excludes broader save/parity/UI scope, and gives runnable verification commands using the truthful cargo-path posture.
