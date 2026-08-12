# SD13-E1-R1 Execution Readiness Closure — Matrix schema and seeded current-state rows

## Card outcome
- evidence_class: `documentary-artifact`
- readiness_verdict: `codex-ready-for-handoff-authoring`
- route truth: this card closes as a documentary readiness artifact only; it does not create repo code, a PR, or a merge surface
- next board move if accepted: continue into the already-routed successor `SD13-E1-R2 FLOW: Matrix schema and seeded current-state handoff artifact`

## Live repo truth grounded on 2026-06-30
- `git rev-parse --abbrev-ref HEAD` reports `sd11-f10-update-action-surface`.
- `git rev-parse HEAD` reports `a0a90dcdf75cd0cc09d4b71e9fb7d3b440aaf293`.
- `git rev-parse origin/develop` reports `c2cea5c6baeb3ca34077b85331214c4b42a4809c`.
- `/home/ubuntu/workspace/repos/codex/README.md` still states that Codex is a developer proof harness plus bounded desktop workbench surface, not a general character builder or broad Pathfinder product.
- `/home/ubuntu/workspace/repos/codex/src/rules_core/mod.rs` currently exports only `character_input`, `pilot_compute`, `pilot_failure`, and `pilot_view_model`, which means the first honest machine-usable SD-13 foothold belongs inside `rules_core` rather than inside UI, release, or oracle-reporting surfaces.
- `/home/ubuntu/workspace/repos/codex/Cargo.toml` has an empty `[dependencies]` section. There is no existing root-crate JSON/YAML/serde data-carrier layer to extend. The smallest truthful repo-facing implementation path is therefore a typed Rust module, not a new external schema/parser subsystem.
- `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs` and `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt` still prove the only accepted direct roster input is `race:human` plus `class:fighter:1`.
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs` explicitly claim-blocks `class:rogue:1` and `class:fighter:2` under the current bounded compute path.
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs` also explicitly claim-blocks `class:fighter:2` for combat/defense surfaces.
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_view_model.rs` proves the current repo can project bounded GE-06 truth into a machine-usable carrier without inventing faux-success snapshots, which is the closest existing architectural seam for an SD-13 matrix carrier.
- `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md` and `.../unsupported-partial-lossy-and-unverified-semantics-ledger.md` already define the authoritative program-level schema obligations and seeded current-state rows, but the repo does not yet contain a corresponding machine-usable control-plane surface.

## Actual verification run during this closure
The following commands were run successfully on the live repo during this card:

```bash
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test --test ge06_pilot_input_contract --test ge06_pilot_total_saves --test ge06_pilot_combat_baseline --test ge06_pilot_view_model
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test
```

Observed result:
- all four focused GE-06 proof files passed
- full `cargo test` passed
- the full suite emitted existing GE-08 `dead_code` warnings in test builds, but no failures

This matters because the next handoff can name exact regression commands that are real today instead of inventing a broader roster proof surface.

## Exact required reads for the next handoff artifact
The stage-specific handoff produced by `SD13-E1-R2` should require reading exactly these surfaces:
- `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
- `/home/ubuntu/workspace/repos/codex/AGENTS.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/references/upstream-dependency-contract.md`
- `/home/ubuntu/workspace/repos/codex/README.md`
- `/home/ubuntu/workspace/repos/codex/Cargo.toml`
- `/home/ubuntu/workspace/repos/codex/src/rules_core/mod.rs`
- `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
- `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
- `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_view_model.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_view_model.rs`
- `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`

## Exact candidate repo paths and allowed write scope
The first honest repo-facing SD-13 lane is narrower than `src/rules_core/**`. The next handoff should authorize only this candidate write surface unless a later readiness pass widens it explicitly.

### Primary candidate write paths
1. `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs`
   - new typed machine-usable carrier for the SD-13 support-state matrix and seeded current-state rows
   - owns enums/structs/constants/functions for the separate support-state axis, evidence-tier axis, subject type, and seeded rows
   - must remain documentary/control-plane truth only; no rules computation, no parser, no file I/O, no UI projection

2. `/home/ubuntu/workspace/repos/codex/src/rules_core/mod.rs`
   - export the new module only
   - no broader `rules_core` reshaping

3. `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs`
   - one focused proof file for roster identity, row-shape invariants, and seeded current-state truth
   - should assert separate support-state and evidence-tier axes rather than flattening them into one label

### Read-only grounding seams for this lane
These files should be read for truth preservation but are not part of the first expected write scope:
- `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
- `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
- `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_view_model.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_view_model.rs`
- `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`

### Explicit scope boundary
If the later handoff discovers that the first machine-usable matrix slice cannot be implemented inside the three primary candidate write paths above, the handoff must stop and route back through a new readiness closure before touching broader surfaces such as:
- `/home/ubuntu/workspace/repos/codex/src/lib.rs`
- `/home/ubuntu/workspace/repos/codex/src/oracle_validation/**`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/**`
- `/home/ubuntu/workspace/repos/codex/tests/fixtures/**`
- any existing GE-06 test file bodies
- any dependency manifest changes beyond the current root-crate no-dependency posture

## Exact schema and seeding obligations the later handoff must preserve
The next handoff should convert the SD-13 packet into one bounded typed carrier with, at minimum, these machine-usable elements:
- a support-state enum or equivalent vocabulary containing exactly `supported`, `partial`, `lossy`, `blocked`, and `unverified`
- an evidence-tier enum or equivalent vocabulary containing exactly `Observed`, `Parsed`, `Converted`, `Computed`, `Oracle-checked`, and `Product-visible`
- a row subject type covering at least `race`, `class`, and `interaction`
- row fields for subject identity, semantic/progression dimension, support state, highest evidence tier, grounding reference, blocker/lossiness note when not supported, and next required uplift/owning slice

The initial seeded repo rows should be constrained to truth already grounded by the packet and live repo, including at least:
- Human race row as `partial` / `Computed`
- Fighter level-1 class row as `partial` / `Computed`
- Fighter levels 2-10 row as `blocked` / `Computed`
- Rogue bounded class row as `blocked` / `Computed`
- Human bonus-feat / ability-bonus interaction seam as `partial` / `Computed`
- remaining core-race and core-class roster rows as `unverified` / `Observed` unless the handoff can cite stronger existing repo evidence without inventing new support

The later handoff must not silently upgrade any row beyond what the currently cited GE-06 evidence actually proves.

## Exact non-goals for the first repo-facing SD-13 lane
The next handoff must state these non-goals plainly:
- no broad roster implementation work for non-Human races or non-Fighter classes
- no Fighter level-2 or level-10 rules computation work
- no spellcasting burden implementation
- no multiclassing, archetype, prestige-class, or non-core expansion
- no changes to `pilot_compute.rs`, `character_input.rs`, or `pilot_view_model.rs` unless a later readiness pass explicitly widens scope
- no UI/workbench/status/reporting work under SD-11 surfaces
- no distribution/update/channel work under SD-12 surfaces
- no persistence/lifecycle work under SD-14 surfaces
- no new external schema format, parser, serializer, or root-crate dependency addition merely to carry the matrix
- no edits to existing GE-06 fixtures or regression tests as a shortcut to make the matrix look better
- no claim that the repo now supports the full core roster merely because seeded rows exist

## Exact verification surfaces for the next handoff
The next handoff should name these repo-grounded verification commands explicitly:

```bash
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test --test sd13_support_state_matrix
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test --test ge06_pilot_input_contract --test ge06_pilot_total_saves --test ge06_pilot_combat_baseline --test ge06_pilot_view_model
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test
```

Verification interpretation:
- the new focused `sd13_support_state_matrix` test is the acceptance gate for the new machine-usable carrier itself
- the four focused GE-06 tests are mandatory regression protection because the seeded rows cite those exact existing truths and blockers
- full `cargo test` is a smoke/regression sweep only; it does not upgrade SD-13 breadth claims by itself

## Readiness verdict
This lane is ready for handoff authoring now.

Why it is ready:
- the authoritative SD-13 packet already defines the exact matrix and ledger semantics the repo carrier must preserve
- the live repo exposes a narrow existing `rules_core` seam that can host a typed control-plane surface without inventing UI, importer, or release scope
- the current repo truth is sharp enough to seed the first honest rows without pretending broader race/class support
- the real verification commands are already known and were run successfully during this closure
- the root crate has no existing serializer/dependency surface, which makes a small typed Rust module the smallest truthful implementation move

Why it is not yet a direct code-authorizing outcome by itself:
- this card does not author the Claude-routed execution handoff
- it does not yet freeze the exact handoff prose, branch instructions, or receipt requirements for the later CODE lane
- it does not justify widening beyond the new `rules_core` carrier plus one focused test file

## Successor truth
The earned successor remains:
- `SD13-E1-R2 FLOW: Matrix schema and seeded current-state handoff artifact`

That successor should produce the stage-specific handoff artifact that converts the scope above into a code-authorizing brief for the later Claude-only CODE lane without widening into broader roster, UI, release, or persistence work.
