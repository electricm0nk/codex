---
title: SD13-E4-R3 Execution Handoff — Wizard level-1 prepared arcane spell-burden baseline blockers
handoff_id: HANDOFF-CODEX-SD13-E4-R3-WIZARD-LEVEL1-PREPARED-SPELL-BASELINE-2026-07-06
stc_id: STC-CODEX-SD-13
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: ready-for-claude-launch
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e4-r3-wizard-level1-prepared-spell-baseline-execution-handoff-2026-07-06.md
source_stc: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md
source_epic_breakdown: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md
source_readiness_closure: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e4-r1-spellcasting-and-hybrid-level-10-progression-readiness-closure-2026-07-01.md
selected_slice: SD13-E4 next code slice — Wizard level-1 prepared arcane spell-burden baseline blockers, mirroring the Sorcerer level-1 spontaneous baseline pattern with school specialization + spellbook/spells-prepared/spell-slots burden named explicitly
run_in: Claude Code only
code_authority: true
authority_dependencies:
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-requirements.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-design.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/acceptance-and-verification.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/level-10-progression-validation-contract.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e4-r1-spellcasting-and-hybrid-level-10-progression-readiness-closure-2026-07-01.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e4-r2-spellcasting-and-hybrid-level-10-progression-execution-handoff-2026-07-01.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e3-r5-paladin-and-ranger-hybrid-chassis-baseline-execution-handoff-2026-07-01.md
  - src/rules_core/pilot_compute.rs (Sorcerer seam: explain_sorcerer_level1_spell_baseline)
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  worktree: /home/ubuntu/worktrees/sd13-class-wizard-burden
  branch: feat/sd13-class-wizard-prepared-spell-burden
  branch_base: origin/develop
  expected_base_sha_at_creation: c78287ce76d3cce10fe814806558976fcfd70543
  compare_base_ref: origin/develop
  pr_target: develop
allowed_write_scope:
  - src/rules_core/pilot_compute.rs
  - tests/sd13_wizard_level1_prepared_spell_baseline.rs
  - tests/fixtures/rules_core/pf1_human_wizard_level1_sd13_deterministic_input.txt
forbidden_write_scope:
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md (matrix file is read-only for hand-edits; row state update flows through the merge receipt only)
  - src/rules_core/support_state_matrix.rs (in-scope carrier is NOT modified by this slice; matrix row transition is recorded by the merge receipt)
  - src/lib.rs
  - src/rules_core/character_input.rs
  - src/rules_core/pilot_view_model.rs
  - src/rules_core/pilot_failure.rs
  - src/rules_core/mod.rs
  - Cargo.toml
  - Cargo.lock
  - AGENTS.md
  - CLAUDE.md
  - apps/desktop/**
  - tests/sd13_sorcerer_level1_spell_baseline.rs
  - tests/sd13_hybrid_level1_chassis_baseline.rs
  - tests/sd13_fighter_*.rs
  - tests/sd13_support_state_matrix.rs
  - tests/fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt
  - tests/fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt
  - tests/fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt
  - tests/fixtures/rules_core/pf1_human_fighter_*.txt
  - tests/ge06_*.rs
  - tests/ge08_*.rs
completion_class: pr-created
reviewed_at: 2026-07-06
---

# SD13-E4-R3 Execution Handoff — Wizard level-1 prepared arcane spell-burden baseline blockers

## Status
This is the stage-specific code-authorizing brief for the second honest repo-facing SD13-E4 lane: the Wizard level-1 prepared arcane spell-burden baseline.

It grants code authority for one bounded slice only:
- add deterministic Human Wizard level-1 runtime recognition as a blocked/computed prepared arcane spell-bearing baseline
- name Wizard's class-specific burden explicitly via two distinct claim-blocking diagnostics: the school specialization burden (specialization choice, two opposed schools, specialty school bonus) and the prepared spell burden (spellbook, spells prepared per day, spell slots per day, bonus spell slots from a high ability score, spell save DCs)
- preserve the accepted Human/Fighter/hybrid/Sorcerer truth already on `origin/develop`
- emit no fabricated spell math (no slot totals, no prepared totals, no school-opposition mechanics)
- the in-source support-state matrix carrier is NOT modified by this slice; the Wizard row transition (Unverified/Observed → Blocked/Computed) is recorded by the merge receipt only

Board-visible verdict:
- this handoff is ready for a governed Claude Code lane now
- later implementation truth exists only if the CODE lane leaves a durable `claude-execution-receipt`
- the bounded slice is one matrix row of the 21-row SD-13 closeout tranche

## Run in
Claude Code only.

Do not substitute Hermes file editing or another coding harness as the primary implementation path. If Claude Code cannot be launched truthfully, block the downstream CODE lane instead of coding through Hermes.

## Core problem
The SD-13 matrix names 12 class rows. After the Sorcerer level-1 spell-burden baseline slice, Wizard still reads `Unverified` / `Observed` — the bounded row has not yet had any runtime evidence. Wizard is the canonical PF1 *prepared* arcane full caster: its class identity differs from Sorcerer in two ways that the matrix must surface explicitly:

1. **Prepared posture**: a Wizard records spells in a spellbook and chooses a smaller subset to prepare each day, rather than the Sorcerer's spontaneous known-spell posture. The bounded slice must name the spellbook/spells-prepared/spell-slots burden explicitly.
2. **School specialization**: a Wizard chooses one school at level 1, two opposed schools are locked, and the specialty school grants a bonus spell slot / spell known at later levels. The bounded slice must name the school specialization / opposed-school / specialty school bonus burden explicitly.

The bounded slice mirrors the Sorcerer pattern:
- recognize `class:wizard:1` as a prepared arcane spell-bearing identity (a `+0` recognition record only)
- emit two distinct claim-blocking diagnostics naming the school specialization burden and the prepared spell posture burden
- preserve the accepted Human race seam
- never fabricate a slot total, a prepared total, a DC, a school opposition, or a specialty school bonus

No level-2+ Wizard is proven. No multiclass Wizard is proven. No non-Human Wizard is proven. No general spell engine is built.

## Slice discipline — read order

Before any code change, read these in order:

1. `./CLAUDE.md` and `./AGENTS.md` (the repo's own conduct surface)
2. `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md`
3. The acceptance-and-verification.md and epic-breakdown.md under the SD-13 directory
4. `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md` (matrix is read-only)
5. `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e4-r2-spellcasting-and-hybrid-level-10-progression-execution-handoff-2026-07-01.md` (the Sorcerer pattern that this slice mirrors)
6. `src/rules_core/pilot_compute.rs` — focus on `explain_sorcerer_level1_spell_baseline`, `is_single_class_sorcerer_level1`, and the surrounding `const` constants
7. `tests/sd13_sorcerer_level1_spell_baseline.rs` — the test pattern this slice mirrors
8. `tests/fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt` — the fixture pattern this slice mirrors

Do not load additional source STCs, epic breakdowns, or other SD-13 handoffs unless explicitly required by the bounded slice.

## Required deliverables

### A. Fixture
Add `tests/fixtures/rules_core/pf1_human_wizard_level1_sd13_deterministic_input.txt`:
- deterministic PF1 Human Wizard level-1 identity as chosen input only (no derived or computed values)
- preserves the Human race-choice seam (`choice:human_bonus_feat`, `choice:human_ability_bonus`)
- names selections only; no spellbook spell list, no spells-prepared list, no spell slots, no DCs, no school opposition, no specialty school bonus
- includes the school specialization selection (one school) via the canonical choice id `choice:wizard_specialization` so the burden name is anchored to a real input selection
- sets `provenance` to this handoff's path so the slice's source artifact is auditable

### B. Compute-seam seam function
Add to `src/rules_core/pilot_compute.rs`:
- a `WIZARD_CLASS_ID` constant `"class:wizard"` and a `WIZARD_BASELINE_LEVEL: u8 = 1`
- an `is_single_class_wizard_level1(input)` predicate mirroring the Sorcerer one (returns true only for a single-class Wizard at level 1)
- an `explain_wizard_level1_prepared_spell_baseline(input, explanations, diagnostics)` function mirroring `explain_sorcerer_level1_spell_baseline`:
  - returns immediately if the input is not a single-class Wizard at level 1, or if `input.chosen.race_id != HUMAN_RACE_ID`
  - pushes one `ComputationExplanation` with id `"class_chassis.spell_baseline.wizard"` and value `0` whose `detail` names the `class:wizard:1` identity as a prepared arcane spell-bearing class and explicitly states it grounds no spell math (no spellbook, no spells prepared, no spell slots, no DCs, no bonus spells, no school opposition, no specialty school bonus) and carries no fabricated mechanical value (+0)
  - pushes two distinct claim-blocking diagnostics:
    1. `class_feature.wizard.specialization.unsupported` — names the school specialization burden: the chosen school, two opposed schools, specialty school bonus spell slots / spells known, and the school's opposed schools are not implemented
    2. `class_spell.wizard.prepared.unsupported` — names the prepared spell posture burden: spellbook content, spells prepared per day, spell slots per day, bonus spell slots from a high Intelligence, and spell save DCs are out of scope and not fabricated
  - uses the existing `HUMAN_RACE_ID` constant; do not introduce new race constants
- wire the new function into `compute_pilot_base_chassis` immediately after `explain_sorcerer_level1_spell_baseline` so the Wizard seam runs alongside the Sorcerer and hybrid seams without reordering them
- update the module-level doc comment to add a new paragraph documenting the Wizard seam alongside the existing Sorcerer / hybrid paragraphs

### C. Test surface
Add `tests/sd13_wizard_level1_prepared_spell_baseline.rs` mirroring `sd13_sorcerer_level1_spell_baseline.rs`:

Tests required (one per behavior, no compound assertions):
1. `wizard_level1_leaves_direct_prepared_spell_baseline_recognition_evidence` — the `class_chassis.spell_baseline.wizard` explanation exists, its detail names `class:wizard`, `level 1`, and "prepared" or "spell", and the explanation's value is `0`; the Fighter-shaped chassis explanations are not surfaced; ability modifiers remain class-independent
2. `wizard_level1_fabricates_no_spell_math` — no explanation beyond the recognition record may contain "spell" in its id, the recognition value is `0`, and the `base_attack_bonus` remains `0` (no fabricated BAB)
3. `wizard_level1_stays_blocked_on_school_specialization_burden` — `class_feature.wizard.specialization.unsupported` exists as a claim-blocking diagnostic whose message names the school specialization burden (must include "school", "opposed", "specialty")
4. `wizard_level1_stays_blocked_on_prepared_spell_posture_burden` — `class_spell.wizard.prepared.unsupported` exists as a claim-blocking diagnostic whose message names the spellbook / spells prepared / spell slots burden (must include "spellbook", "prepared", and "spell slot"); the two diagnostics are distinct; the count of class-specific claim-blocking diagnostics whose id contains "wizard" is exactly 2
5. `wizard_level1_integrated_posture_is_blocked_not_counterfeit_success` — `build_pilot_headless_receipt` status is `Blocked`, view model status is `Blocked`, primary owner is `EngineFlaw`, snapshot is `None`
6. `spell_baseline_preserves_human_race_seam` — same Human race seam preservation test as the Sorcerer slice
7. `fighter_and_rogue_do_not_gain_wizard_recognition` — same negative-control pattern as the Sorcerer slice: a Fighter chassis must not surface a wizard recognition record or wizard burden diagnostics, and a Rogue must stay blocked and not surface wizard diagnostics
8. `wizard_level_2_is_not_promoted_by_this_slice` — level-2 Wizard must not gain the level-1 prepared-spell-baseline recognition record and must stay claim-blocked
9. `matrix_wizard_row_transition_unverified_to_blocked_is_documented_for_merge_receipt` — verifies that the in-source carrier row `class.wizard.progression_and_spell_burden` is currently `Unverified`/`Observed` (because the carrier is NOT modified by this slice), and asserts the explicit merge-receipt contract: after the merge receipt updates the canonical matrix file, the row will move to `Blocked`/`Computed` with a blocker note that names both the school specialization burden and the prepared spell posture burden. This test makes the merge-receipt obligation visible in the test surface.
10. `matrix_keeps_bard_observed_and_preserves_hybrid_and_sorcerer_blocked_truth` — Bard stays `Unverified`/`Observed`; Paladin, Ranger, and Sorcerer rows stay `Blocked`/`Computed` (their carrier states are not regressed by this slice)
11. `matrix_does_not_promote_any_row_to_supported_or_lossy` — same global guard as the Sorcerer slice

Use the exact same helper functions (`explanation`, `claim_blocking`, `has_explanation`, `load`) as the Sorcerer test so the surface style is identical.

### D. Documentation surface
Update the module-level doc comment of `src/rules_core/pilot_compute.rs` to add a paragraph documenting the Wizard seam — mirror the existing Sorcerer paragraph structure (one paragraph, four sentences: name the slice, name the class identity, state what it grounds, state what it deliberately does not ground).

Do not modify any other doc comment, README, AGENTS.md, or CLAUDE.md.

### E. Out of scope (do not do these)
- do not modify `src/rules_core/support_state_matrix.rs` — the in-source carrier is NOT in allowed_write_scope; the matrix row transition is recorded by the merge receipt only
- do not modify `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md` — the matrix file is read-only for hand-edits
- do not add a second matrix carrier to `src/oracle_validation/` (no such file exists on origin/develop and adding one would widen scope)
- do not introduce a spell engine, a spell database, a DC formula, a school-opposition table, a specialty school bonus table, or any spell math beyond the `+0` recognition record
- do not promote Wizard to `Supported` or `Lossy` anywhere
- do not regress Fighter, Paladin, Ranger, or Sorcerer rows
- do not introduce a non-Human Wizard or a multiclass Wizard
- do not touch any other slice's test or fixture file

## Verification commands (must all pass)

Run from `/home/ubuntu/worktrees/sd13-class-wizard-burden`:

```bash
# Compile-check the compute seam and the new test
cargo build --locked --tests --quiet

# RED stage: write the test file first and run it; expect FAIL with the
# "no wizard recognition record yet" panic or equivalent signal that the
# bounded slice is not yet implemented.
cargo test --locked --test sd13_wizard_level1_prepared_spell_baseline 2>&1 | tail -40

# GREEN stage: implement the seam; re-run; expect all 11 tests PASS.
cargo test --locked --test sd13_wizard_level1_prepared_spell_baseline 2>&1 | tail -40

# Negative controls: existing slices must stay green.
cargo test --locked --test sd13_sorcerer_level1_spell_baseline 2>&1 | tail -10
cargo test --locked --test sd13_hybrid_level1_chassis_baseline 2>&1 | tail -10
cargo test --locked --test sd13_fighter_level2_level3_progression 2>&1 | tail -10
cargo test --locked --test sd13_support_state_matrix 2>&1 | tail -10

# Full crate: nothing regresses.
cargo test --locked --quiet 2>&1 | tail -20

# Lint: clippy clean.
cargo clippy --locked --tests -- -D warnings 2>&1 | tail -20

# Scope check: only the allowed files are touched.
git diff --stat origin/develop..HEAD
git diff --name-only origin/develop..HEAD
```

The slice is GREEN when all of the above pass.

## Diff stat expectation

After implementation, `git diff --stat origin/develop..HEAD` must show changes in only:
- `src/rules_core/pilot_compute.rs` (compute seam seam function + module doc paragraph)
- `tests/sd13_wizard_level1_prepared_spell_baseline.rs` (new test file)
- `tests/fixtures/rules_core/pf1_human_wizard_level1_sd13_deterministic_input.txt` (new fixture)
- (the handoff doc itself, `programs/codex/.../sd13-e4-r3-...md`, lives in the worktree but is not a runtime file — keep it under `programs/codex/.../artifacts/` so it is a documentary artifact, not a runtime surface)

If `git diff --name-only origin/develop..HEAD` shows any other file, the slice has widened scope — fix it before declaring done.

## Receipt obligations

After GREEN, before push:

1. Push the feature branch:
   ```bash
   git push -u origin feat/sd13-class-wizard-prepared-spell-burden
   ```
2. Open a PR to `develop`:
   ```bash
   GH_TOKEN=$(cat /home/ubuntu/.config/gh/.claude_gh_token) \
     gh pr create --repo electricm0nk/codex \
       --base develop --head feat/sd13-class-wizard-prepared-spell-burden \
       --title "SD13-E4: Wizard level-1 prepared spell-burden baseline (Blocked/Computed)" \
       --body-file /tmp/wizard_pr_body.md
   ```
3. The PR body must include a "Matrix row transition (merge-receipt obligation)" section that documents the matrix file row transition from `Unverified`/`Observed` to `Blocked`/`Computed`, the blocker note that names both burdens, the next required uplift, and the grounding ref pointing at `tests/sd13_wizard_level1_prepared_spell_baseline.rs`. This is the canonical merge-receipt surface that updates `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`.

## Card-bound authoritative constraint reminder

This slice is bounded by `kanban task t_e6949cfc`. The card's verification gate is:
- the named test file passes deterministically
- the merge-receipt references this card id and updates the matrix row in the receipt body (NOT in the matrix file)

The Claude-CLI execution receipt obligation (per `kanban-worker` skill and class-level rule) is also binding: a `claude-execution-receipt` comment must be posted to the kanban task via `kanban_comment` with the task id, repo, worktree, branch, base SHA, invocation mode, claude session handle, model, verification commands run, verification results, commit SHAs, and PR URL — before `kanban_complete` is called.