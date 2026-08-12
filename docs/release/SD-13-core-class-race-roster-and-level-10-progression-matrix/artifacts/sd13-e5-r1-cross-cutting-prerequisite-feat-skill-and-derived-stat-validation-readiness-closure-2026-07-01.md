# SD13-E5-R1 Readiness Closure — Cross-cutting prerequisite, feat, skill, and derived-stat validation

## Card outcome
- evidence_class: `documentary-artifact`
- readiness_verdict: `codex-ready-for-handoff-authoring`
- route truth: the first code-authorizing SD13-E5 move should start with `SD13-F9` on the accepted deterministic Human Fighter levels 1-3 validation seam, not by flattening still-blocked hybrid or spell-bearing class-family burdens into counterfeit cross-cutting support
- next board move if accepted: continue into the same-domain successor `SD13-E5-R2 FLOW: Cross-cutting prerequisite, feat, skill, and derived-stat validation handoff artifact`

## Live repo truth grounded on 2026-07-01
- Fresh grounding was taken from current `origin/develop` after `git fetch origin --prune`; it resolves to `454a92ed67578124d88232b130a832de6ed571df`, the merged post-PR-45 truth named by this card.
- The shared checkout at `/home/ubuntu/workspace/repos/codex` is not the authoritative launch substrate for later repo-facing work:
  - `HEAD` = `122de6a60609d9452de53c6d3ad406aeb81c2a82`
  - branch = `feat/sd13-e6-f11-support-state-debt-presentation`
  - upstream tracking is gone
  - untracked `apps/desktop/src-tauri/gen/` content is present
  Later repo-facing SD13-E5 work must therefore launch from fresh `origin/develop` truth or an isolated worktree rather than treating the shared checkout as accepted state.
- A clean detached verification worktree was created at `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict` and verified at `454a92ed67578124d88232b130a832de6ed571df` with no local drift.
- In accepted `src/rules_core/support_state_matrix.rs`, the current class-family posture is now sharp enough to separate the first truthful E5 route from the still-blocked family burdens E5 must not counterfeit away:
  - `class.fighter.level_1_pilot` = `Partial` / `Computed`
  - `class.fighter.levels_2_10` = `Partial` / `Computed`, grounded by `tests/sd13_fighter_level2_level3_progression.rs`, with explicit blocker text that levels 2 and 3 only are proven and that a general feat-effect / prerequisite engine still does not exist
  - `class.rogue.bounded_progression` = `Blocked` / `Computed`
  - `class.paladin.hybrid_chassis_and_spell_burden` = `Blocked` / `Computed`, with the unresolved non-spell class-feature burden kept distinct from the later spell burden
  - `class.ranger.hybrid_chassis_and_spell_burden` = `Blocked` / `Computed`, with the same two-burden separation
  - `class.sorcerer.progression_and_spell_burden` = `Blocked` / `Computed`, with the bloodline burden kept distinct from the spontaneous known-spell / slot posture burden
  - `class.bard.progression_and_spell_burden`, `class.cleric.progression_and_spell_burden`, `class.druid.progression_and_spell_burden`, and `class.wizard.progression_and_spell_burden` remain `Unverified` / `Observed`
- The accepted interaction posture matters directly to the E5 routing decision. `interaction.human_bonus_feat_ability_bonus.pilot_pressure` is already `Partial` / `Computed`, which means the repo now carries one explicitly named race/class choice-pressure seam instead of only generic roster prose.
- In accepted `tests/ge06_pilot_input_contract.rs`, the deterministic Human Fighter input contract already exposes the concrete cross-cutting choice surfaces E5 needs instead of leaving them folkloric:
  - selected feats include `feat:power_attack`, `feat:dodge`, and `feat:weapon_focus`
  - selected choices preserve `choice:level_1_character_feat`, `choice:human_bonus_feat`, `choice:fighter_bonus_feat`, and `choice:human_ability_bonus`
  - chosen skill allocations preserve `skill:climb`, `skill:intimidate`, and `skill:swim`
- In accepted `src/rules_core/pilot_compute.rs`, the live compute seam already contains bounded substrate for the first honest E5 slice:
  - explicit Human choice-seam constants for `choice:human_bonus_feat` and `choice:human_ability_bonus`
  - explicit Fighter choice-seam constants for `choice:fighter_bonus_feat` and `choice:fighter_bonus_feat_2`
  - explicit bounded selected-skill outputs
  - explicit explanation and diagnostic carriers propagated into receipts and view models
  - explicit level-3 armor-training pressure that changes the bounded selected-skill outputs
  - explicit statement that no general feat-effect or prerequisite engine exists yet
- In accepted `tests/sd13_fighter_level2_level3_progression.rs`, the repo already proves the exact deterministic seam that makes `SD13-F9` the first truthful E5 route:
  - level 2 is no longer blanket-blocked for the deterministic Human Fighter path
  - the level-2 bonus-feat progression seam is explicit and bounded, contributing no counterfeit feat-effect value
  - level 3 surfaces the armor-training seam and changes selected skill modifiers honestly
  - receipts and view models continue to propagate the widened bounded truth
- In accepted `tests/ge06_pilot_selected_skill_modifiers.rs`, the selected-skill pressure is already grounded as real computed output with explanation text, not a future aspiration.
- In accepted `tests/ge06_pilot_headless_receipt.rs`, `tests/ge06_failure_classifier.rs`, and `tests/ge06_pilot_view_model.rs`, explanation and diagnostic visibility already propagate through downstream surfaces. That means E5 does not need to invent a new reporting substrate before it can validate cross-cutting pressure honestly.
- The accepted hybrid and Sorcerer baseline tests are still critical, but as boundaries rather than as the first positive E5 lane:
  - `tests/sd13_hybrid_level1_chassis_baseline.rs` proves Paladin/Ranger level-1 chassis recognition while keeping both classes explicitly blocked on unresolved class-feature and spell burdens
  - `tests/sd13_sorcerer_level1_spell_baseline.rs` proves Sorcerer level-1 spell-bearing recognition while keeping both the bloodline burden and spontaneous spell burden explicitly blocked
  - those tests make excellent regression/negative-control surfaces for E5, but they do not justify starting E5 by pretending those class families are already cross-cutting-ready for positive prerequisite closure

## Actual verification run during this closure
All verification below was run successfully against `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict` after grounding that worktree on accepted `origin/develop`.

```bash
cd /home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict && . "$HOME/.cargo/env" && cargo test --test ge06_pilot_input_contract --test ge06_pilot_total_saves --test ge06_pilot_combat_baseline --test ge06_pilot_selected_skill_modifiers --test ge06_pilot_headless_receipt --test ge06_failure_classifier --test ge06_pilot_view_model --test sd13_fighter_level2_level3_progression --test sd13_hybrid_level1_chassis_baseline --test sd13_sorcerer_level1_spell_baseline --test sd13_support_state_matrix
```

Observed result:
- focused regression suite passed with no failures
- `ge06_pilot_input_contract`: 2 passed
- `ge06_pilot_total_saves`: 3 passed
- `ge06_pilot_combat_baseline`: 4 passed
- `ge06_pilot_selected_skill_modifiers`: 5 passed
- `ge06_pilot_headless_receipt`: 3 passed
- `ge06_failure_classifier`: 6 passed
- `ge06_pilot_view_model`: 3 passed
- `sd13_fighter_level2_level3_progression`: 9 passed
- `sd13_hybrid_level1_chassis_baseline`: 10 passed
- `sd13_sorcerer_level1_spell_baseline`: 12 passed
- `sd13_support_state_matrix`: 26 passed
- total passing tests in the focused bundle: 83

This matters because the readiness verdict below is grounded on accepted repo execution truth, not on stale PR prose or on the drifted shared checkout.

## Which cross-cutting seam can truthfully move next
### Ready to move next
1. First code-authorizing SD13-E5 work should target `SD13-F9` prerequisite and invalid-choice blocking, anchored to the deterministic Human Fighter levels 1-3 path.
   - The live Fighter levels-2-10 row already names the missing general feat-effect / prerequisite engine explicitly. That is no longer a theoretical future burden; it is written into the accepted matrix truth on `origin/develop`.
   - The deterministic Human Fighter path is the only accepted class family that now combines positive computed progression evidence, explicit choice seams, explicit skill pressure, and propagated explanation/diagnostic surfaces.
   - That makes it the narrowest truthful E5 route: close the first honest prerequisite / bonus-choice / invalid-choice blocker surface where the repo already has positive, bounded progression truth.
2. The first E5 slice should stay centered on Human/Fighter deterministic pressure and use hybrid/spell-bearing rows only as regression boundaries.
   - `interaction.human_bonus_feat_ability_bonus.pilot_pressure` is already a named computed interaction row.
   - `tests/ge06_pilot_input_contract.rs` and `tests/sd13_fighter_level2_level3_progression.rs` already expose the concrete choice seams the lane needs: Human bonus feat, Human ability bonus, Fighter bonus feat, Fighter level-2 bonus feat, class-skill pressure, and the derived outputs affected by those seams.
   - Paladin, Ranger, and Sorcerer should remain negative-control/regression surfaces so E5 does not counterfeit their still-blocked class-family burdens into general choice-support closure.
3. The first E5 slice may exercise derived outputs only insofar as they are needed to prove the prerequisite / invalid-choice surface behaves honestly.
   - The route is not “build a general skill engine” or “rewrite all derived-stat math.”
   - It is “prove that bounded choice pressure can block or preserve the already-grounded Human/Fighter outputs without hiding the reason in folklore.”

### Not ready to move as positive support yet
1. Paladin, Ranger, or Sorcerer as the first positive prerequisite/invalid-choice slice
   - Their accepted rows remain `Blocked` / `Computed` for named class-family reasons outside the cross-cutting lane.
   - Starting E5 there would flatten unresolved class-feature or spell burdens into counterfeit cross-cutting readiness.
2. Bard, Cleric, Druid, or Wizard as the first E5 slice
   - They remain `Unverified` / `Observed` in the live matrix.
   - E5 must not pretend broader class-family proof exists when the matrix still says otherwise.
3. A combined `SD13-F9 + SD13-F10` “validate everything” tranche
   - The first truthful move is smaller: prerequisite / invalid-choice blocking on the accepted Human/Fighter deterministic path.
   - A combined tranche would blur route truth and dilute the exact missing burden already named by the matrix.
4. Any route that treats hybrid or spell-bearing baseline evidence as proof that feat/prerequisite legality is broadly solved across the roster
   - That is exactly the counterfeit promotion this readiness closure is meant to prevent.

## Exact required reads for the later SD13-E5 handoff artifact
The stage-specific handoff produced by `SD13-E5-R2` should require reading exactly these surfaces, with repo truth taken from fresh accepted `origin/develop` content rather than from the stale shared checkout:
- `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict/AGENTS.md`
- `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
- `/home/ubuntu/workspace/repos/codex/AGENTS.md`
- `/home/ubuntu/workspace/repos/codex/README.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-requirements.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-design.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/acceptance-and-verification.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/level-10-progression-validation-contract.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e4-r1-spellcasting-and-hybrid-level-10-progression-readiness-closure-2026-07-01.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e5-r1-cross-cutting-prerequisite-feat-skill-and-derived-stat-validation-readiness-closure-2026-07-01.md`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict/src/rules_core/pilot_compute.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict/src/rules_core/support_state_matrix.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict/src/oracle_validation/support_state_matrix.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict/tests/ge06_pilot_input_contract.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict/tests/ge06_pilot_total_saves.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict/tests/ge06_pilot_combat_baseline.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict/tests/ge06_pilot_selected_skill_modifiers.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict/tests/ge06_pilot_headless_receipt.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict/tests/ge06_failure_classifier.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict/tests/ge06_pilot_view_model.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict/tests/sd13_fighter_level2_level3_progression.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict/tests/sd13_hybrid_level1_chassis_baseline.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict/tests/sd13_sorcerer_level1_spell_baseline.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict/tests/sd13_support_state_matrix.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict/tests/fixtures/rules_core/pf1_human_fighter_level2_sd13_deterministic_input.txt`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict/tests/fixtures/rules_core/pf1_human_fighter_level3_sd13_deterministic_input.txt`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict/tests/fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict/tests/fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict/tests/fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt`

## Exact repo surfaces likely to matter next
The later handoff should authorize one bounded `SD13-F9` slice. It should not authorize a general breadth-validation sprint.

### Primary candidate write paths
1. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
   - this is the live seam that already carries Human choice pressure, Fighter level-2 bonus-feat pressure, selected-skill outputs, explanation records, and claim-blocking diagnostics
   - it is the natural place to add the first truthful prerequisite / invalid-choice validation behavior
2. `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs`
   - this is the authoritative live matrix carrier
   - any uplift to the Human interaction row or to the Fighter levels-2-10 blocker note must remain explicit here rather than living only in tests or prose
3. `/home/ubuntu/workspace/repos/codex/src/oracle_validation/support_state_matrix.rs`
   - if the rules-core carrier changes, the oracle-validation carrier must be kept intentionally aligned rather than drifting again
4. `/home/ubuntu/workspace/repos/codex/tests/sd13_fighter_level2_level3_progression.rs`
   - this is the accepted positive proof surface for the deterministic Human Fighter levels 2-3 tranche and already names the bounded bonus-feat seam
5. `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs`
   - this must pin whatever matrix truth the first E5 slice changes or deliberately preserves
6. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`
   - the concrete feat/choice/skill inputs already live here and must remain visible if the E5 slice depends on them
7. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_selected_skill_modifiers.rs`
   - this is the current proof surface for derived skill outputs and must remain honest if invalid-choice or bonus-choice pressure changes what may be computed
8. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_headless_receipt.rs`
9. `/home/ubuntu/workspace/repos/codex/tests/ge06_failure_classifier.rs`
10. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_view_model.rs`
   - these downstream surfaces matter because E5 is supposed to keep explanations and diagnostics structured and visible, not trapped inside one compute-side branch

### Expected new proof surface that the later handoff must freeze explicitly
11. `/home/ubuntu/workspace/repos/codex/tests/`
   - the first E5 code lane will need one dedicated tranche-specific proof file for the bounded prerequisite / invalid-choice slice
   - this readiness closure does not invent that filename prematurely; `SD13-E5-R2` must freeze it exactly
12. `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/`
   - the first E5 code lane may need a bounded new deterministic fixture only if the chosen invalid-choice path cannot be expressed as a safe mutation of the accepted Fighter fixtures already present
   - if no new fixture is needed, the later handoff must say so explicitly rather than adding one by habit

### Read-only grounding seams the handoff must classify deliberately
13. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs`
14. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`
15. `/home/ubuntu/workspace/repos/codex/tests/sd13_hybrid_level1_chassis_baseline.rs`
16. `/home/ubuntu/workspace/repos/codex/tests/sd13_sorcerer_level1_spell_baseline.rs`
   - these are the regression floor that protects accepted derived-output truth and keeps hybrid/spell-bearing class-family boundaries visible while E5 opens a cross-cutting validation slice

## Explicit non-goals for the later SD13-E5 handoff artifact
The next handoff must state these non-goals plainly:
- no claim that the whole roster now has truthful prerequisite or feat legality support
- no use of Paladin, Ranger, or Sorcerer blocked/computed baseline evidence as proof that their class-family burdens are resolved
- no broad spell-bearing, divine, or hybrid spell-support work under the first E5 slice
- no generic feat-effect engine, no general prerequisite engine across the full ruleset, and no general skill engine
- no level-4+ Fighter uplift, no wider martial breadth closure, and no non-Fighter positive support promotion
- no multiclassing, archetypes, prestige classes, non-core scope expansion, UI/workbench/reporting/distribution/persistence work under SD-11, SD-12, or SD-14 authority
- no silent weakening of accepted Human race seam, Human interaction seam, Fighter level-1 and levels-2-10 partial/computed truth, Rogue blocker truth, hybrid blocked/computed truth, or Sorcerer blocked/computed truth already proven on `origin/develop`
- no silent omission of `src/oracle_validation/support_state_matrix.rs` if the chosen slice changes matrix truth in `rules_core`

## Exact verification commands the later handoff may name
### Preflight grounding commands
These are not success gates by themselves, but the later handoff should require them so the worker does not operate from stale branch truth.

```bash
cd /home/ubuntu/workspace/repos/codex && git fetch origin --prune && git rev-parse --abbrev-ref HEAD && git rev-parse HEAD && git rev-parse origin/develop
cd /home/ubuntu/workspace/repos/codex && git diff --name-only origin/develop -- src/rules_core/pilot_compute.rs src/rules_core/support_state_matrix.rs src/oracle_validation/support_state_matrix.rs tests/ge06_pilot_input_contract.rs tests/ge06_pilot_total_saves.rs tests/ge06_pilot_combat_baseline.rs tests/ge06_pilot_selected_skill_modifiers.rs tests/ge06_pilot_headless_receipt.rs tests/ge06_failure_classifier.rs tests/ge06_pilot_view_model.rs tests/sd13_fighter_level2_level3_progression.rs tests/sd13_hybrid_level1_chassis_baseline.rs tests/sd13_sorcerer_level1_spell_baseline.rs tests/sd13_support_state_matrix.rs tests/fixtures/rules_core
```

Interpretation:
- if the target working copy still lags `origin/develop` on the listed files, the worker must sync to accepted `develop` truth or launch from a clean worktree before claiming SD13-E5 evidence

### Required regression / acceptance floor inherited from accepted current truth
```bash
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test --test ge06_pilot_input_contract --test ge06_pilot_total_saves --test ge06_pilot_combat_baseline --test ge06_pilot_selected_skill_modifiers --test ge06_pilot_headless_receipt --test ge06_failure_classifier --test ge06_pilot_view_model --test sd13_fighter_level2_level3_progression --test sd13_hybrid_level1_chassis_baseline --test sd13_sorcerer_level1_spell_baseline --test sd13_support_state_matrix
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test
```

Verification interpretation:
- the focused regression floor is mandatory because it protects the accepted Human choice seam, Fighter levels 1-3 truth, selected-skill outputs, derived-output explanation propagation, Rogue blocker truth, and the hybrid/spell-bearing blocked baselines that E5 must not flatten
- full `cargo test` is a smoke/regression sweep only; it does not by itself upgrade any SD13 support-state claim
- the later `SD13-E5-R2` handoff must add one exact new dedicated prerequisite / invalid-choice proof command once it freezes the tranche-specific proof file path

## Readiness verdict
This lane is ready for handoff authoring now.

Why it is ready:
- accepted `origin/develop` truth now exposes one narrow, positive, bounded cross-cutting seam: deterministic Human Fighter levels 1-3 with explicit feat/choice pressure, skill pressure, and propagated derived-output explanations
- the live matrix itself names the missing general feat-effect / prerequisite engine as unresolved burden inside the Fighter levels-2-10 row, so the next move is no longer ambiguous
- the focused accepted regression floor was re-run successfully against a clean accepted worktree during this closure
- the live repo already contains the downstream receipt/classifier/view-model visibility surfaces E5 needs, so the first E5 handoff can stay about bounded validation truth rather than inventing a new reporting lane

Why it is not yet a direct code-authorizing outcome by itself:
- this card does not author the stage-specific `SD13-E5-R2` handoff prose
- it does not yet freeze the exact dedicated proof file path, exact allowed write scope, or fresh worktree launch sequence for the later CODE lane
- it does not justify treating hybrid or spell-bearing blocked/computed baselines as broader prerequisite closure
- it does not authorize a general rules-validation engine or a combined `SD13-F9 + SD13-F10` tranche

## Successor truth
The earned successor is:
- `SD13-E5-R2 FLOW: Cross-cutting prerequisite, feat, skill, and derived-stat validation handoff artifact`

That successor should convert this closure into a stage-specific, code-authorizing brief for the first truthful `SD13-F9` slice: deterministic Human Fighter prerequisite / bonus-choice / invalid-choice validation with bounded skill and derived-output pressure preserved, while hybrid and spell-bearing baselines remain explicit regression boundaries rather than counterfeit support.