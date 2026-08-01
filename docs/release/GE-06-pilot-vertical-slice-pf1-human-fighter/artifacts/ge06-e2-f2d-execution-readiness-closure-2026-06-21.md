---
title: GE06-E2-F2d Execution Readiness Closure
artifact_type: execution-readiness-closure
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE06-E2-F2d — Selected deterministic skill modifiers and Chain Shirt armor-check effects
workflow_route: readiness-closure
readiness: codex-ready
handoff_created: false
created_handoff: []
review_date: 2026-06-21
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE06-E2-F2d Execution Readiness Closure

## Verdict
GE-06 may advance to the next narrow code-producing handoff.

There is **not yet** an active F2d code-authorizing artifact. The next required coding artifact is a fresh stage-specific handoff, not a mutation of any earlier brief:

```text
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2d-execution-handoff-2026-06-21.md
```

This readiness closure itself is not code authority. It records why the separate F2d handoff may later carry `code_authority: true` while the root `execution-handoff.md` remains only a route surface.

## Core problem
GE06-E2-F2c closed deterministic total saving throws, but the accepted pilot contract still names unresolved first-slice proof in the skill/equipment-effect surface. GE-06 still cannot compute the selected deterministic skill modifiers for Climb, Intimidate, and Swim while truthfully applying the already-grounded Chain Shirt armor-check effect only where it belongs.

## Selected bounded slice

```text
GE06-E2-F2d — Selected deterministic skill modifiers and Chain Shirt armor-check effects
```

This slice is deliberately narrow. It computes only:

- selected deterministic skill totals for Climb, Intimidate, and Swim
- the bounded contributors for each selected total:
  - chosen rank allocation
  - key ability modifier
  - class-skill bonus when the grounded Fighter class-skill posture plus at least one rank are present
  - Chain Shirt armor-check penalty for Climb and Swim only
- explanation records naming those exact contributors
- explicit claim-blocking diagnostics when the chosen skill allocation, Fighter class posture, or deterministic equipment posture is absent or widened beyond this slice

This slice does **not** compute the general Pathfinder skill system. It does not authorize cross-skill coverage, feat-based skill modifiers, item bonuses beyond the deterministic Chain Shirt armor-check penalty already grounded here, encumbrance, temporary effects, size adjustments, movement-mode bonuses, parity, or UI work.

## Required source evidence recovered

| Gate | Evidence |
|---|---|
| Prior merged foothold | GE06-E2-F2c merged into `origin/develop` at `1b44c07`. |
| Target repo exists | `/home/ubuntu/workspace/repos/codex`. |
| Branch policy | Start from current `origin/develop`, target PR to `develop`. |
| Baseline tests | `"$HOME/.cargo/bin/cargo" test --quiet` passes on a detached worktree at `origin/develop` commit `1b44c07`. |
| Existing compute surface | `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`. |
| Existing chosen-input structures | `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs` defines `skill_allocations`, `equipment_selections`, and `ActiveState`. |
| Existing merged proof tests | `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_base_computation.rs`, `tests/ge06_pilot_combat_baseline.rs`, and `tests/ge06_pilot_total_saves.rs`. |
| Input fixture | `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt` lines 18-24 preserve Climb 1, Intimidate 1, Swim 1, Chain Shirt active, Longsword active, shield absent, and Power Attack selected-inactive. |
| Deterministic input contract | `artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md`. |
| Fighter skill-rank source | `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst:141` shows Fighter base skill points `2`; the accepted deterministic contract already fixes Human Skilled `+1` and favored-class bonus to hit point rather than skill rank. |
| Fighter class-skill grounding | `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_class.lst:2835` lists Fighter class skills including Climb, Intimidate, and Swim. |
| Selected skill definitions | `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_skills.lst:10`, `:42`, and `:102` ground Climb, Intimidate, and Swim, including `KEYSTAT` and armor-check applicability. |
| Chain Shirt grounding | `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_arms_armor.lst:40` grounds `ACCHECK:-2`. |
| GE-04 computation requirements | `../GE-04-rules-engine-and-explainability-core/technical-requirements.md` TR-04-010 and TR-04-011. |

## Grounded expected values

Using the accepted deterministic input fixture and keeping the slice strictly bounded to Climb, Intimidate, and Swim:

### Climb

```text
rank 1 + STR modifier +3 + class-skill bonus +3 + Chain Shirt armor-check penalty -2 = 5
```

### Intimidate

```text
rank 1 + CHA modifier -1 + class-skill bonus +3 = 3
```

### Swim

```text
rank 1 + STR modifier +3 + class-skill bonus +3 + Chain Shirt armor-check penalty -2 = 5
```

Expected bounded outputs:

```yaml
climb: 5
intimidate: 3
swim: 5
```

Bounded contributor rules for this slice:

- class-skill bonus is present only because Fighter class skills include Climb, Intimidate, and Swim and the deterministic fixture allocates one rank to each
- Chain Shirt armor-check penalty applies only to skills grounded as `ACHECK:YES`; in this slice that means Climb and Swim, not Intimidate
- the slice may refuse unsupported postures rather than guess at broader skill-engine semantics

These values are local computed outputs only. They are not oracle-checked parity.

## Gate table

| Gate | Status | Resolution |
|---|---|---|
| Prior foothold merged | pass | GE06-E2-F2c is on `origin/develop` at `1b44c07`. |
| Bounded implementation slice selected | pass | GE06-E2-F2d is limited to Climb, Intimidate, and Swim totals plus the deterministic Chain Shirt armor-check effect. |
| Target repo/workdir exists | pass | `/home/ubuntu/workspace/repos/codex`. |
| Branch policy explicit | pass | Branch from current `origin/develop`, PR to `develop`. |
| Allowed write scope explicit | pass | Extend `pilot_compute.rs` and add one new selected-skill proof test only. |
| Runtime instruction surface exists | pass | Repo `AGENTS.md` exists and requires strict TDD. |
| Toolchain grounded | pass | Full test suite passes when invoked as `"$HOME/.cargo/bin/cargo" test --quiet`; current shell PATH does not expose `cargo` by default. |
| Verification commands known | pass | Exact per-test and full-suite commands are named below. |
| Non-goals explicit | pass | Excludes broad skill-engine behavior, broad equipment effects, parity, importer expansion, and UI. |
| Harness route explicit | pass | F2d must get its own stage-specific execution handoff; the root route surface remains non-authorizing. |

## Authorized write scope for the derived handoff

The derived handoff may authorize writes only to:

```text
src/rules_core/pilot_compute.rs
tests/ge06_pilot_selected_skill_modifiers.rs
```

It may read but must not modify the prior proof files:

```text
tests/ge06_pilot_base_computation.rs
tests/ge06_pilot_combat_baseline.rs
tests/ge06_pilot_total_saves.rs
tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt
src/rules_core/character_input.rs
```

If a compile break proves another file is required, stop and report the blocker rather than widening silently.

## Required TDD posture

The coding harness must:

1. create the failing GE-06 selected-skill test first
2. run the specific test and capture RED
3. implement the smallest additions inside the existing `pilot_compute.rs` surface needed to pass
4. run the specific test and capture GREEN
5. re-run the prior F2a, F2b, and F2c proof tests
6. run full `"$HOME/.cargo/bin/cargo" test --quiet`
7. run a file-granular scope audit

## Explicit non-goals

The derived handoff must not authorize:

- Acrobatics, Stealth, Ride, or any skill outside Climb / Intimidate / Swim
- favored-class skill-rank changes
- broad class-skill resolution for arbitrary classes
- feat-based skill modifiers
- racial skill modifiers
- encumbrance or inventory breadth
- speed-dependent Swim/Climb rules
- armor-check effects from armor other than the deterministic Chain Shirt posture
- shield penalties or broader equipment-effect propagation
- attack, damage, initiative, AC, or save changes beyond preserving already merged behavior
- imported source-package conversion
- oracle comparison or claim `Oracle-checked`
- normalization engine
- parity report writer
- PCGen execution
- UI, view-model, desktop shell, or export-sheet work

## Claim tier after this slice

If the later F2d handoff succeeds, GE-06 may claim:

```text
selected pilot input contract: represented
ability modifiers: computed with explanations
Fighter base BAB/save chassis: computed with explanations
baseline melee attack bonus: computed with explanation
baseline armor class: computed with explanation
total saves: computed with explanations
selected deterministic skill modifiers for Climb / Intimidate / Swim: computed with explanations
broader skill system / broader equipment effects / parity / UI: not yet
```

## Completion rule
This readiness closure is complete when the package truthfully records all of the following:

- GE06-E2-F2c is the most recently merged coding slice
- GE06-E2-F2d is now grounded enough for a code-authorizing handoff
- there is still **no active F2d stage-specific coding artifact yet**
- the root `execution-handoff.md` points to this readiness state without becoming code authority itself
- any later F2d implementation run must use a fresh stage-specific handoff with exact repo paths, verification commands, and the non-goals above
