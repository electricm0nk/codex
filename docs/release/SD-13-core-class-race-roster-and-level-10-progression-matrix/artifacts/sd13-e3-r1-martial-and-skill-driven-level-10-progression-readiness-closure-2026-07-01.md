# SD13-E3-R1 Readiness Closure — Martial and skill-driven level-10 progression

## Card outcome
- evidence_class: `documentary-artifact`
- readiness_verdict: `codex-ready-for-handoff-authoring`
- route truth: this card closes as a documentary readiness artifact only; it does not authorize repo edits, a PR, or a merge surface by itself
- next board move if accepted: continue into the same-domain successor `SD13-E3-R2 FLOW: Martial and skill-driven level-10 progression handoff artifact`

## Live repo truth grounded on 2026-07-01
- The accepted upstream basis for this closure is `origin/develop` at `25765e8c2cb4ed50bd936183b24a2f2189977bc0`, the merge commit for PR #41 (`SD13-E2` Human race-semantics slice).
- The shared checkout at `/home/ubuntu/workspace/repos/codex` is not currently that basis. During this closure it reported branch `feat/sd13-e6-f11-support-state-debt-presentation` at `122de6a60609d9452de53c6d3ad406aeb81c2a82`, plus untracked `apps/desktop/src-tauri/gen/` content.
- The shared checkout also differs from accepted `origin/develop` on the exact SD13-E3-relevant surfaces: `src/rules_core/pilot_compute.rs`, `src/rules_core/support_state_matrix.rs`, `tests/ge06_failure_classifier.rs`, `tests/ge06_pilot_headless_receipt.rs`, `tests/ge06_pilot_view_model.rs`, and `tests/sd13_support_state_matrix.rs`.
- Because of that drift, this closure was re-grounded in a clean detached worktree at `/tmp/codex-sd13-e3-r1-origin-develop` created directly from `origin/develop`. Later handoff authoring must do the same thing in principle: either sync the main repo checkout to accepted `develop` truth first, or explicitly author from a clean worktree based on `origin/develop`.
- In the accepted `origin/develop` code, `src/rules_core/pilot_compute.rs` still hard-gates the executable class seam on `has_fighter_level_1(...)`. The current live computed class surface is therefore still one bounded `race:human` + `class:fighter:1` deterministic pilot, not a general martial roster.
- In the same accepted code, the repo now also preserves the bounded SD13-E2 Human race seam explicitly via `race.human.ability_bonus_target`, `race.human.bonus_feat_grant`, and the non-claim-blocking `race.human.bounded_semantics` diagnostic. Any later SD13-E3 uplift must preserve that accepted Human interaction truth rather than accidentally rolling it back.
- `src/rules_core/support_state_matrix.rs` on accepted `origin/develop` still classifies:
  - `class.fighter.level_1_pilot` as `Partial` / `Computed`
  - `class.fighter.levels_2_10` as `Blocked` / `Computed`
  - `class.rogue.bounded_progression` as `Blocked` / `Computed`
  - `class.barbarian.bounded_progression` and `class.monk.bounded_progression` as `Unverified` / `Observed`
  - `class.paladin.hybrid_chassis_and_spell_burden` and `class.ranger.hybrid_chassis_and_spell_burden` as `Unverified` / `Observed`
- `tests/ge06_pilot_total_saves.rs` still proves the current save surface is truthful only for Fighter level 1, explicitly claim-blocking both Rogue level 1 and Fighter level 2.
- `tests/ge06_pilot_combat_baseline.rs` still proves the current combat/defense surface is truthful only for the deterministic Fighter level-1 baseline, explicitly claim-blocking Fighter level 2 and unsupported loadout changes.
- `tests/ge06_pilot_selected_skill_modifiers.rs` now matters directly to SD13-E3. It proves only the deterministic Fighter level-1 Climb / Intimidate / Swim skill slice and explicitly claim-blocks non-Fighter chassis widening. That means the first honest SD13-E3 lane must treat skill pressure as a bounded regression seam, not as a broad skill engine.
- `tests/ge06_pilot_headless_receipt.rs`, `tests/ge06_failure_classifier.rs`, and `tests/ge06_pilot_view_model.rs` prove that the bounded Fighter/Human seam is propagated through receipt, failure ownership, and view-model layers. Later E3 work may not widen class progression in `pilot_compute.rs` while silently breaking these downstream truth surfaces.

## Actual verification run during this closure
All verification below was run successfully against the clean `origin/develop` worktree at `/tmp/codex-sd13-e3-r1-origin-develop`.

```bash
cd /tmp/codex-sd13-e3-r1-origin-develop && . "$HOME/.cargo/env" && cargo test --test ge06_pilot_input_contract --test ge06_pilot_total_saves --test ge06_pilot_combat_baseline --test ge06_pilot_selected_skill_modifiers --test ge06_pilot_headless_receipt --test ge06_failure_classifier --test ge06_pilot_view_model --test sd13_support_state_matrix
cd /tmp/codex-sd13-e3-r1-origin-develop && . "$HOME/.cargo/env" && cargo test
```

Observed result:
- focused regression suite passed:
  - `ge06_pilot_input_contract`: 2 passed
  - `ge06_pilot_total_saves`: 3 passed
  - `ge06_pilot_combat_baseline`: 4 passed
  - `ge06_pilot_selected_skill_modifiers`: 5 passed
  - `ge06_pilot_headless_receipt`: 3 passed
  - `ge06_failure_classifier`: 6 passed
  - `ge06_pilot_view_model`: 3 passed
  - `sd13_support_state_matrix`: 20 passed
- full `cargo test` passed
- the full suite emitted existing GE-08 `dead_code` warnings in test builds, but no failures

This matters because the later handoff can name exact regression commands tied to accepted repo truth instead of reusing stale branch folklore.

## Which martial and skill-driven seams can truthfully move next
### Ready to move next
1. Fighter progression beyond the current level-1 ceiling
   - This is the only class family with a live computed seam already present in `pilot_compute.rs`.
   - The repo already names the exact blocker honestly: Fighter level 2 is explicitly blocked in current tests and the matrix already carries a `Blocked` / `Computed` row for `class.fighter.levels_2_10`.
   - The first honest repo-facing SD13-E3 lane is therefore not “all martials.” It is a bounded Fighter progression uplift that names milestone levels and class-feature burden through level 10 without pretending Barbarian, Monk, Rogue, Paladin, or Ranger are solved.

2. Rogue as an explicit negative-control seam
   - Rogue is already present as live negative evidence (`Blocked` / `Computed`) in both the matrix and the current total-save / selected-skill regression posture.
   - That makes Rogue suitable as a preservation seam in the next handoff: the later lane may use Rogue blockers to prove that Fighter widening did not silently flatten class identity.

### Not ready to move as positive support yet
1. Barbarian positive support
   - No live rage, rage-power, movement, or uncanny-dodge/trap-sense seam exists in the current repo.
   - Current truthful state remains `Unverified` / `Observed`.

2. Monk positive support
   - No live flurry, ki, AC-bonus, maneuver, or bonus-feat progression seam exists in the current repo.
   - Current truthful state remains `Unverified` / `Observed`.

3. Paladin and Ranger positive support
   - They are hybrid martial/spell classes by packet doctrine.
   - SD13-E3 may at most prepare their non-spell chassis boundary later, but this readiness pass does not justify claiming their progression truth now.

4. Any spell-bearing class burden
   - Explicit non-goal for this lane. SD13-E4 owns spell burden closure.

## Exact required reads for the later SD13-E3 handoff artifact
The stage-specific handoff produced by `SD13-E3-R2` should require reading exactly these surfaces, with repo truth taken from accepted `origin/develop` content rather than from a stale feature branch copy:
- `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
- `/home/ubuntu/workspace/repos/codex/AGENTS.md`
- `/home/ubuntu/workspace/repos/codex/README.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-requirements.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-design.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/level-10-progression-validation-contract.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e2-r1-core-race-semantic-readiness-closure-2026-06-30.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e3-r1-martial-and-skill-driven-level-10-progression-readiness-closure-2026-07-01.md`
- `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
- `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_selected_skill_modifiers.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_headless_receipt.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_failure_classifier.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_view_model.rs`
- `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs`
- `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`

## Exact candidate repo surfaces and allowed write scope for the first honest repo-facing SD13-E3 lane
The later handoff should narrow the first code-authorizing slice to “bounded Fighter milestone progression uplift, with Rogue retained as a negative control and Human race-seam truth preserved.” It should not authorize a general martial implementation blast radius.

### Primary candidate write paths
1. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
   - may widen the current Fighter-only level-1 computation seam into a bounded Fighter milestone progression seam
   - must preserve the accepted Human race explanation/diagnostic behavior already merged by SD13-E2
   - must not become a general class engine, general skill engine, or spell system

2. `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs`
   - may update the Fighter and adjacent class rows only to the extent justified by the new bounded evidence
   - must preserve the separation of support state vs evidence tier
   - must not silently promote Barbarian, Monk, Rogue, Paladin, Ranger, or any spell-bearing row without direct new proof

3. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs`
   - may be updated if Fighter progression evidence changes the current level-2 blocker posture or adds named milestone save coverage
   - must preserve explicit blocked behavior where the new slice still does not prove the wider burden

4. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`
   - may be updated if the bounded Fighter progression slice changes which Fighter milestone combat/defense states are honestly computed
   - must preserve explicit refusal for unsupported loadout widening

5. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_selected_skill_modifiers.rs`
   - may be updated only if the bounded Fighter progression slice explicitly claims additional skill-pressure truth
   - must remain a bounded selected-skill proof, not broaden into a general skill system

6. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_headless_receipt.rs`
7. `/home/ubuntu/workspace/repos/codex/tests/ge06_failure_classifier.rs`
8. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_view_model.rs`
   - these three downstream truth surfaces may be updated only as required to keep receipt/classifier/view-model behavior aligned with the widened bounded Fighter seam
   - they are regression-protection surfaces, not a license to broaden UI/product claims

9. `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs`
   - must be updated if the matrix truth changes for the bounded Fighter row(s)
   - must keep the row taxonomy honest when only Fighter moves upward and Rogue/other classes do not

### Conditional write surface allowed only if the handoff names it explicitly
10. `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/`
   - new deterministic Fighter milestone fixture files may be added here if the later slice cannot truthfully prove its milestone levels by bounded mutation of the accepted level-1 fixture alone
   - the existing accepted fixture `pf1_human_fighter_level1_ge06_deterministic_input.txt` must remain read-only unless a separate readiness pass explicitly authorizes altering that already-accepted contract

### Read-only grounding seams for this lane
These files should be read for truth preservation but are not part of the first expected write scope:
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`
- `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
- program-level packet docs listed above

### Explicit stop-and-reroute conditions
If the later handoff concludes that the first honest SD13-E3 code lane cannot be implemented inside the scope above, it must stop and route back through a new readiness pass before touching broader surfaces such as:
- `/home/ubuntu/workspace/repos/codex/src/lib.rs`
- `/home/ubuntu/workspace/repos/codex/src/oracle_validation/**`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/**`
- `/home/ubuntu/workspace/repos/codex/tests/ge08_*`
- dependency manifests or new third-party crates
- non-Fighter class-positive-support surfaces
- existing accepted Human race-semantics assertions beyond preservation-aligned updates

## Exact non-goals for the later SD13-E3 handoff artifact
The next handoff must state these non-goals plainly:
- no claim that “martial and skill-driven classes” are broadly supported after a Fighter-only uplift
- no positive support claim for Barbarian, Monk, Rogue, Paladin, or Ranger unless the slice names and proves their class-specific burden directly
- no spellcasting burden implementation, spell slots, known/prepared posture, domains, schools, bloodlines, or partial-caster spell closure
- no multiclassing, archetypes, prestige classes, or non-core expansion
- no non-Human race coverage uplift beyond preserving the accepted Human interaction seam already merged in SD13-E2
- no general skill engine, feat engine, prerequisite engine, or equipment engine; only bounded Fighter milestone burden explicitly named by the handoff may move
- no UI/workbench/reporting/distribution/persistence work under SD-11, SD-12, or SD-14 authority
- no broad “level-10 support” claim for Codex as a product
- no rollback or accidental weakening of the accepted Human race explanation / diagnostic surfaces or their downstream receipt / classifier / view-model propagation
- no edits to the accepted level-1 deterministic fixture as a shortcut to make new milestone claims easier

## Exact verification commands the later handoff may name
### Preflight grounding commands
These are not success gates by themselves, but the later handoff should require them so the worker does not operate from stale branch truth.

```bash
cd /home/ubuntu/workspace/repos/codex && git fetch origin && git rev-parse --abbrev-ref HEAD && git rev-parse HEAD && git rev-parse origin/develop
cd /home/ubuntu/workspace/repos/codex && git diff --name-only origin/develop -- src/rules_core/pilot_compute.rs src/rules_core/support_state_matrix.rs tests/ge06_pilot_input_contract.rs tests/ge06_pilot_total_saves.rs tests/ge06_pilot_combat_baseline.rs tests/ge06_pilot_selected_skill_modifiers.rs tests/ge06_pilot_headless_receipt.rs tests/ge06_failure_classifier.rs tests/ge06_pilot_view_model.rs tests/sd13_support_state_matrix.rs tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt
```

Interpretation:
- if the target working copy still lags `origin/develop` on the listed files, the worker must sync to accepted `develop` truth or use a clean worktree before claiming SD13-E3 evidence

### Required regression / acceptance commands
```bash
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test --test ge06_pilot_input_contract --test ge06_pilot_total_saves --test ge06_pilot_combat_baseline --test ge06_pilot_selected_skill_modifiers --test ge06_pilot_headless_receipt --test ge06_failure_classifier --test ge06_pilot_view_model --test sd13_support_state_matrix
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test
```

Verification interpretation:
- the focused regression bundle is mandatory because it covers the accepted Human race seam, Fighter chassis/save/combat/skill truth, receipt/classifier/view-model propagation, and matrix truth that the next slice is allowed to touch
- full `cargo test` is a smoke/regression sweep only; it does not upgrade any SD13 support-state claim by itself
- if the later handoff introduces a new dedicated Fighter-milestone proof file or new deterministic milestone fixture, that handoff must add its exact test command explicitly rather than assuming the generic bundle above already covers it

## Readiness verdict
This lane is ready for handoff authoring now.

Why it is ready:
- accepted `origin/develop` truth is sharp enough to identify the first honest E3 move: bounded Fighter progression uplift, not a counterfeit all-martials push
- the repo already contains the exact blocker evidence that defines the next slice honestly (`class:fighter:2` blocked, Rogue blocked, other martial families unverified)
- the packet already defines the level-10 burden table and explicitly separates martial, hybrid, and spell-bearing families
- the Human race-semantics slice is now accepted and machine-visible in the exact downstream surfaces E3 must preserve
- the real regression commands were re-run successfully against a clean accepted `develop` worktree during this closure

Why it is not yet a direct code-authorizing outcome by itself:
- this card does not author the stage-specific `SD13-E3-R2` handoff prose
- it does not yet freeze the exact milestone levels or exact Fighter feature burden the Claude-routed code lane will claim
- it does not justify positive support claims for Barbarian, Monk, Rogue, Paladin, Ranger, or any spell-bearing class
- it does not authorize broad fixture rewriting, UI work, or a generic progression engine

## Successor truth
The earned successor remains:
- `SD13-E3-R2 FLOW: Martial and skill-driven level-10 progression handoff artifact`

That successor should convert this closure into a stage-specific, code-authorizing brief for the later Claude-routed CODE lane while keeping the first executable E3 slice bounded to truthful Fighter progression uplift, Rogue negative-control preservation, and Human race-seam regression protection.