---
title: GE06-E2-F3 Execution Readiness Closure
artifact_type: execution-readiness-closure
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE06-E2-F3 — End-to-end command and receipt path
workflow_route: readiness-closure
readiness: codex-ready
handoff_created: true
created_handoff:
  - ./ge06-e2-f3-execution-handoff-2026-06-21.md
review_date: 2026-06-21
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE06-E2-F3 Execution Readiness Closure

## Verdict
GE-06 was grounded sufficiently to mint the next narrow code-producing handoff, and that paired artifact now exists.

The active E2-F3 code-authorizing artifact created from this readiness closure is:

```text
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f3-execution-handoff-2026-06-21.md
```

This readiness closure itself is not code authority. It records why the separate E2-F3 handoff carries `code_authority: true` while the root `execution-handoff.md` remains a route surface.

## Core problem
GE06-E2-F2d closed the deterministic selected-skill foothold, but GE-06 still lacks one integrated headless evidence path that a later parity consumer or minimal UI consumer can rely on. The repo can already load the accepted pilot fixture and compute bounded outputs, yet it still cannot emit one bounded receipt surface that says, in a machine-checkable way, either:

- here is the current integrated GE-06 headless evidence, or
- here is the explicit blocker receipt and the diagnostics that prevented the claim.

## Selected bounded slice

```text
GE06-E2-F3 — End-to-end command and receipt path
```

This slice is deliberately narrow. It proves only one headless integrated receipt path for the accepted deterministic pilot. It should:

- consume the existing deterministic fixture through the existing character-input loader
- call the existing GE-06 pilot compute surface
- emit one bounded structured receipt surface that preserves:
  - case identity
  - source package identity
  - currently supported computed outputs
  - explanation references/details already proven by prior slices
  - claim-blocking diagnostics when the path is unsupported or blocked
  - a simple status that distinguishes evidence from blocker
- be executable from one focused test path

This slice does **not** authorize:

- a production CLI
- new dependencies such as `clap`, `serde`, or `serde_json`
- broad report-writer architecture
- oracle comparison
- normalization logic
- parity claims
- UI/view-model work
- importer expansion
- new fixture semantics

## Required source evidence recovered

| Gate | Evidence |
|---|---|
| Prior merged foothold | GE06-E2-F2d merged into `origin/develop` at `2deb11b`. |
| Target repo exists | `/home/ubuntu/workspace/repos/codex`. |
| Branch policy | Start from current `origin/develop`, target PR to `develop`. |
| Baseline tests | `"$HOME/.cargo/bin/cargo" test --quiet` passes on a detached worktree at `origin/develop` commit `2deb11b`. |
| Existing load surface | `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs` exposes `load_character_input_fixture`. |
| Existing compute surface | `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs` exposes `compute_pilot_base_chassis` and now includes the F2a/F2b/F2c/F2d bounded outputs plus explanations and diagnostics. |
| Existing pilot proof tests | `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`, `tests/ge06_pilot_base_computation.rs`, `tests/ge06_pilot_combat_baseline.rs`, `tests/ge06_pilot_total_saves.rs`, and `tests/ge06_pilot_selected_skill_modifiers.rs`. |
| Input fixture | `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`. |
| No current command surface | `src/` contains library modules only; there is no existing `main.rs`, `bin/`, or command crate surface to extend truthfully in this slice. |
| No current serialization/CLI dependency | `/home/ubuntu/workspace/repos/codex/Cargo.toml` has no dependencies declared. |
| GE-06 headless-first doctrine | `technical-requirements.md` TR-06-007, TR-06-008, and TR-06-009 require one end-to-end proof path with explanation and diagnostic visibility before UI claims. |
| GE-04 headless/explainability doctrine | GE-04 remains a headless pipeline that must emit explanations and diagnostics for tested behavior. |
| GE-05 downstream need | GE-05 design expects new-system output capture to be available as a headless evidence surface for later comparison work. |

## Grounded implementation posture
Because the repo currently has:

- a stable loader,
- a stable deterministic compute surface,
- stable proof tests,
- no existing CLI/bin surface,
- and no serialization dependency,

this slice should use the **smallest truthful implementation**:

1. extend the existing `pilot_compute.rs` surface with one bounded headless receipt/result shape
2. add one focused integration test that exercises the receipt path in both evidence and blocker modes
3. keep the entire slice headless and library-first

A new command-line interface would be counterfeit expansion here. The honest "command or test path" for E2-F3 is the focused test path.

## Expected bounded receipt surface
The derived handoff should require one structured receipt/result shape that can preserve, at minimum:

```yaml
case_id: pf1-crb-human-fighter-level1
source_package_id: pf1.core_rulebook
status: computed-or-blocked
claim_tier_floor: Computed
outputs:
  ability_modifiers:
    strength: 3
    dexterity: 2
    constitution: 2
    intelligence: 0
    wisdom: 1
    charisma: -1
  base_attack_bonus: 1
  base_saves:
    fortitude: 2
    reflex: 0
    will: 0
  baseline_melee_attack_bonus: 5
  baseline_armor_class: 17
  total_saves:
    fortitude: 4
    reflex: 2
    will: 1
  selected_skill_modifiers:
    climb: 5
    intimidate: 3
    swim: 5
explanations: present
claim_blocking_diagnostics: present when blocked
```

Equivalent field names are acceptable if the result remains:

- structured
- machine-checkable in tests
- reusable by later parity/UI work
- explicit about blocked status rather than fabricating success

These values are local computed outputs only. They are not oracle-checked parity.

## Gate table

| Gate | Status | Resolution |
|---|---|---|
| Prior foothold merged | pass | GE06-E2-F2d is on `origin/develop` at `2deb11b`. |
| Bounded implementation slice selected | pass | E2-F3 is limited to one headless receipt path over the already-grounded deterministic pilot surface. |
| Target repo/workdir exists | pass | `/home/ubuntu/workspace/repos/codex`. |
| Branch policy explicit | pass | Branch from current `origin/develop`, PR to `develop`. |
| Allowed write scope explicit | pass | Extend `pilot_compute.rs` and add one new focused headless-receipt proof test only. |
| Runtime instruction surface exists | pass | Repo `AGENTS.md` exists and requires strict TDD. |
| Toolchain grounded | pass | `"$HOME/.cargo/bin/cargo" test --quiet` passes; current shell PATH does not expose `cargo` by default. |
| Verification commands known | pass | Exact per-test and full-suite commands are named below. |
| No counterfeit CLI expansion required | pass | There is no current CLI/bin surface; the truthful proof path is a focused test over a structured library receipt. |
| Non-goals explicit | pass | Excludes oracle, UI, dependency additions, importer expansion, and broad report architecture. |
| Harness route explicit | pass | E2-F3 has its own stage-specific execution handoff, and the root route surface remains non-authorizing even while pointing at it. |

## Authorized write scope for the derived handoff
The derived handoff may authorize writes only to:

```text
src/rules_core/pilot_compute.rs
tests/ge06_pilot_headless_receipt.rs
```

It may read but must not modify these grounded surfaces:

```text
src/rules_core/character_input.rs
tests/ge06_pilot_input_contract.rs
tests/ge06_pilot_base_computation.rs
tests/ge06_pilot_combat_baseline.rs
tests/ge06_pilot_total_saves.rs
tests/ge06_pilot_selected_skill_modifiers.rs
tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt
Cargo.toml
Cargo.lock
AGENTS.md
CLAUDE.md
```

If a compile break proves another file is required, stop and report the blocker rather than widening silently.

## Required TDD posture
The coding harness must:

1. create the failing `ge06_pilot_headless_receipt.rs` test first
2. run the specific test and capture RED
3. implement the smallest additions inside the existing `pilot_compute.rs` surface needed to pass
4. run the specific test and capture GREEN
5. re-run the prior GE-06 proof tests
6. run full `"$HOME/.cargo/bin/cargo" test --quiet`
7. run a file-granular scope audit

## Explicit non-goals
The derived handoff must not authorize:

- `src/main.rs`, `src/bin/**`, or any CLI package surface
- adding `serde`, `serde_json`, `clap`, `anyhow`, or any other dependency
- rewriting existing proof tests instead of adding the new E2-F3 receipt-path test
- changes to `character_input.rs`
- changes to the deterministic input fixture
- changes to prior F2a/F2b/F2c/F2d proof expectations
- broad report-writer architecture
- oracle comparison or claim `Oracle-checked`
- normalization logic
- PCGen execution or exporter capture
- UI, view-model, desktop shell, or export-sheet work
- generic Pathfinder receipt/report infrastructure beyond the first bounded pilot receipt path

## Claim tier after this slice
If the later E2-F3 handoff succeeds, GE-06 may claim:

```text
selected pilot input contract: represented
ability modifiers: computed with explanations
Fighter base BAB/save chassis: computed with explanations
baseline melee attack bonus: computed with explanation
baseline armor class: computed with explanation
total saves: computed with explanations
selected deterministic skill modifiers: computed with explanations
one integrated headless receipt path for the accepted deterministic pilot: computed and test-proven
oracle parity / UI truth / broad reporting surface: not yet
```

## Completion rule
This readiness closure is complete when the package truthfully records all of the following:

- GE06-E2-F2d is the most recently merged coding slice
- GE06-E2-F3 is now grounded enough for a code-authorizing handoff
- the paired `ge06-e2-f3-execution-handoff-2026-06-21.md` artifact exists and is the active awaiting-launch coding brief
- the root `execution-handoff.md` points at the E2-F3 handoff without becoming code authority itself
- any later E2-F3 implementation run must use a fresh stage-specific handoff with exact repo paths, verification commands, and the non-goals above
