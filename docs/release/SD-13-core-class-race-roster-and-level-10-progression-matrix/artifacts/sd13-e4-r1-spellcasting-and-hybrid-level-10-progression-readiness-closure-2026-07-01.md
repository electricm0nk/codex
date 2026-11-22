# SD13-E4-R1 Readiness Closure — Spellcasting and hybrid level-10 progression

## Card outcome
- evidence_class: `documentary-artifact`
- readiness_verdict: `codex-ready-for-handoff-authoring`
- route truth: this card closes as a documentary readiness artifact only; it does not authorize repo edits, a PR, or a merge surface by itself
- next board move if accepted: continue into the same-domain successor `SD13-E4-R2 FLOW: Spellcasting and hybrid level-10 progression handoff artifact`

## Live repo truth grounded on 2026-07-01
- The accepted upstream basis for this closure is current `origin/develop` at `8e48056c1fc5fc2f1af772a4a90c9e73ce2144c5`, the merged post-PR-44 truth used by the upstream SD13-E4 release gate.
- The fresh evidence worktree at `/home/ubuntu/workspace/worktrees/codex-sd13-e4-verdict` was re-grounded during this closure with `git fetch origin develop`; it now reports `HEAD` = `origin/develop` = `8e48056c1fc5fc2f1af772a4a90c9e73ce2144c5` and `git status --short --branch` reports detached `HEAD` with no local drift.
- The focused regression bundle was re-run successfully from that accepted worktree during this closure. That matters because this readiness artifact is grounded on live accepted repo truth rather than on stale PR prose.
- In accepted `src/rules_core/pilot_compute.rs`, the runtime seam is still deliberately narrow:
  - Fighter progression is the only positive computed chassis family already widened beyond the original level-1 pilot seam.
  - Paladin and Ranger are recognized only at the bounded deterministic Human level-1 hybrid chassis boundary.
  - Their runtime posture remains explicitly claim-blocked on two different burden families: missing non-spell class-feature execution and later spell burden.
  - No Bard, Cleric, Druid, Sorcerer, or Wizard runtime spell posture exists yet.
- In accepted `src/rules_core/support_state_matrix.rs`, the spell-bearing rows are now split truthfully instead of being flattened:
  - `class.paladin.hybrid_chassis_and_spell_burden` = `Blocked` / `Computed`, grounded by `tests/sd13_hybrid_level1_chassis_baseline.rs`, with explicit blocker text separating the unresolved non-spell class-feature burden from the later spell burden.
  - `class.ranger.hybrid_chassis_and_spell_burden` = `Blocked` / `Computed`, grounded the same way, with the same explicit two-burden separation.
  - `class.bard.progression_and_spell_burden`, `class.cleric.progression_and_spell_burden`, `class.druid.progression_and_spell_burden`, `class.sorcerer.progression_and_spell_burden`, and `class.wizard.progression_and_spell_burden` remain `Unverified` / `Observed`.
- The live tests confirm the hybrid baseline is still only a baseline. `tests/sd13_hybrid_level1_chassis_baseline.rs` proves direct runtime acknowledgement for deterministic Human Paladin/Ranger level-1 inputs while simultaneously asserting:
  - no Paladin/Ranger level 2+ uplift
  - no class-feature execution closure
  - no spell-slot / spell-source / known-versus-prepared posture closure
  - no promotion of either row to `Partial` or `Supported`
- The repo currently contains deterministic hybrid fixtures for Paladin and Ranger level 1 only:
  - `tests/fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt`
  - `tests/fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt`
  There are no corresponding deterministic Bard, Cleric, Druid, Sorcerer, or Wizard fixtures in the live rules-core fixture set yet.
- A second matrix carrier still lags the newly accepted hybrid truth: `src/oracle_validation/support_state_matrix.rs` continues to serialize Paladin and Ranger as `Unverified` / `Observed`. That file is therefore a real repo surface the later handoff must read explicitly and either keep deliberately out of scope or align on purpose; it must not be forgotten by folklore.

## Actual verification run during this closure
All verification below was run successfully against `/home/ubuntu/workspace/worktrees/codex-sd13-e4-verdict` after confirming it matches accepted `origin/develop`.

```bash
cd /home/ubuntu/workspace/worktrees/codex-sd13-e4-verdict && . "$HOME/.cargo/env" && cargo test --test ge06_pilot_input_contract --test ge06_pilot_total_saves --test ge06_pilot_combat_baseline --test ge06_pilot_selected_skill_modifiers --test ge06_pilot_headless_receipt --test ge06_failure_classifier --test ge06_pilot_view_model --test sd13_support_state_matrix --test sd13_hybrid_level1_chassis_baseline
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
  - `sd13_hybrid_level1_chassis_baseline`: 10 passed
  - `sd13_support_state_matrix`: 26 passed
- no failures occurred in the focused bundle

This closure intentionally stops at the focused accepted proof floor. The later handoff must freeze any new dedicated E4 proof command explicitly rather than pretending the existing bundle already covers the future spell-bearing slice.

## Which spellcasting and hybrid seams can truthfully move next
### Ready to move next
1. First code-authorizing E4 work should target the arcane spellcasting family (`SD13-F7`), not the hybrid spell-burden family (`SD13-F8`).
   - The repo already proves that Paladin and Ranger are not merely generic martials; their hybrid level-1 chassis identity is now visible and their unresolved burden is named in two parts.
   - That same proof also shows why they are not the first honest E4 execution target: their rows are still blocked on unresolved non-spell class-feature burden before any later spell-burden closure could count as truthful class support.
   - By contrast, Bard, Sorcerer, and Wizard all remain untouched `Unverified` / `Observed` spell-bearing rows. They are the cleanest first E4 family because their next move can be framed directly as spell-bearing-class burden work rather than as a rescue of unfinished E3 hybrid feature debt.
2. The first E4 handoff should stay inside one spell-bearing family and force a narrower slice than “all spellcasters.”
   - `artifacts/level-10-progression-validation-contract.md` separates spell-bearing proof burden from martial/hybrid burden.
   - `epic-breakdown.md` already splits `SD13-E4` into `SD13-F7` (Bard/Sorcerer/Wizard) and `SD13-F8` (Cleric/Druid/Paladin/Ranger).
   - The truthful next move is therefore a bounded `F7` handoff, not a combined `F7 + F8` blast radius.
3. The later handoff may decide the first bounded `F7` code slice is smaller than all three arcane classes together, but this readiness closure does not counterfeit that narrower decision yet.
   - Bard carries bardic-performance and support-feature burden beyond generic spell posture.
   - Sorcerer carries bloodline and spontaneous known-spell burden.
   - Wizard carries prepared/spellbook posture plus school-or-bonded-item burden.
   - The later `R2` handoff must freeze which one bounded slice is first and name its exact write scope and proof file. This `R1` closure only establishes that `F7`, not hybrid or divine/hybrid `F8`, is the first truthful execution route.

### Not ready to move as positive support yet
1. Paladin and Ranger spell-burden closure as the first E4 slice
   - Current live repo truth still leaves both rows blocked on unresolved non-spell class-feature burden.
   - Treating the accepted level-1 chassis evidence as spell-burden readiness would flatten exactly the distinction this gate was created to preserve.
2. Cleric and Druid as the first E4 slice
   - They belong to the broader `SD13-F8` divine / branch-heavy burden family.
   - Starting there would combine prepared divine branch pressure with the still-open hybrid burden family instead of taking the cleaner arcane-first route.
3. Any combined “all spell-bearing classes now move together” tranche
   - No live runtime spell posture exists for any full caster.
   - No deterministic full-caster fixtures exist yet.
   - The packet explicitly forbids collapsing spell-bearing proof into roster-name theater.

## Exact required reads for the later SD13-E4 handoff artifact
The stage-specific handoff produced by `SD13-E4-R2` should require reading exactly these surfaces, with repo truth taken from accepted `origin/develop` content rather than from a stale feature branch copy:
- `/home/ubuntu/workspace/worktrees/codex-sd13-e4-verdict/AGENTS.md`
- `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
- `/home/ubuntu/workspace/repos/codex/AGENTS.md`
- `/home/ubuntu/workspace/repos/codex/README.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-requirements.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-design.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/acceptance-and-verification.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/level-10-progression-validation-contract.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e3-r5-paladin-and-ranger-hybrid-chassis-baseline-execution-handoff-2026-07-01.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e4-r1-spellcasting-and-hybrid-level-10-progression-readiness-closure-2026-07-01.md`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e4-verdict/src/rules_core/pilot_compute.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e4-verdict/src/rules_core/support_state_matrix.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e4-verdict/src/oracle_validation/support_state_matrix.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e4-verdict/tests/sd13_support_state_matrix.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e4-verdict/tests/sd13_hybrid_level1_chassis_baseline.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e4-verdict/tests/ge06_pilot_input_contract.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e4-verdict/tests/ge06_pilot_total_saves.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e4-verdict/tests/ge06_pilot_combat_baseline.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e4-verdict/tests/ge06_pilot_selected_skill_modifiers.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e4-verdict/tests/ge06_pilot_headless_receipt.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e4-verdict/tests/ge06_failure_classifier.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e4-verdict/tests/ge06_pilot_view_model.rs`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e4-verdict/tests/fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt`
- `/home/ubuntu/workspace/worktrees/codex-sd13-e4-verdict/tests/fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt`

## Exact repo surfaces likely to matter next
The later handoff should narrow the first code-authorizing slice to one bounded `SD13-F7` spell-bearing tranche. It should not authorize a general spell-engine or hybrid cleanup sprint.

### Primary candidate write paths
1. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
   - the only live rules-core seam currently capable of surfacing direct runtime evidence or explicit claim-blocking diagnostics for a new spell-bearing slice
   - must not become a generic spell engine or a covert Paladin/Ranger feature-completion lane
2. `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs`
   - the authoritative live matrix carrier that would need any row-state uplift grounded by the new spell-bearing proof
   - must preserve support-state versus evidence-tier separation and keep blocked/partial/unverified posture visible
3. `/home/ubuntu/workspace/repos/codex/src/oracle_validation/support_state_matrix.rs`
   - currently stale relative to the accepted hybrid truth and therefore a deliberate decision surface for the next handoff
   - if left read-only, the handoff must say so explicitly; if aligned, the handoff must authorize it explicitly
4. `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs`
   - must pin the exact final matrix posture after any bounded E4 uplift
5. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_headless_receipt.rs`
6. `/home/ubuntu/workspace/repos/codex/tests/ge06_failure_classifier.rs`
7. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_view_model.rs`
   - these downstream surfaces matter because any new blocked/computed spell-bearing posture must propagate honestly through receipt, classifier, and view-model layers instead of remaining an isolated compute-side secret

### Expected new proof surfaces that the handoff must freeze explicitly
8. `/home/ubuntu/workspace/repos/codex/tests/`
   - the first E4 code lane will need a dedicated new spell-bearing proof file with an exact path frozen by `SD13-E4-R2`; this readiness closure does not invent that filename prematurely
9. `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/`
   - the first E4 code lane will likely need new deterministic full-caster fixtures because none exist yet for Bard, Cleric, Druid, Sorcerer, or Wizard in the live accepted fixture set

### Read-only grounding seams the handoff must classify deliberately
10. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`
11. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs`
12. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`
13. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_selected_skill_modifiers.rs`
14. `/home/ubuntu/workspace/repos/codex/tests/sd13_hybrid_level1_chassis_baseline.rs`
   - these are the regression floor that preserves accepted Human/Fighter/hybrid baseline truth while E4 opens a spell-bearing family

## Explicit non-goals for the later SD13-E4 handoff artifact
The next handoff must state these non-goals plainly:
- no claim that all spell-bearing or hybrid core classes are broadly supported after the first E4 slice
- no use of Paladin/Ranger level-1 hybrid chassis evidence as proof that their spell burden is closed
- no attempt to complete Paladin or Ranger non-spell class-feature burden inside the first E4 spell-bearing slice
- no combined `SD13-F7` + `SD13-F8` tranche
- no positive support claim for Cleric, Druid, Paladin, or Ranger in the first E4 code-authorizing handoff unless the handoff names and proves their class-specific burden directly
- no general spell engine, no generic slot resolver, no UI option theater, and no matrix-wide fake promotion driven only by class names
- no multiclassing, archetypes, prestige classes, non-core scope expansion, or UI/workbench/reporting/distribution/persistence work under SD-11, SD-12, or SD-14 authority
- no silent weakening of the accepted Fighter 1-3, Rogue blocked, Human race, Human interaction, or hybrid blocked/computed posture already proven on `origin/develop`
- no silent omission of `src/oracle_validation/support_state_matrix.rs` if the chosen slice would otherwise leave the repo carrying contradictory matrix truth in two places

## Exact verification commands the later handoff may name
### Preflight grounding commands
These are not success gates by themselves, but the later handoff should require them so the worker does not operate from stale branch truth.

```bash
cd /home/ubuntu/workspace/repos/codex && git fetch origin --prune && git rev-parse --abbrev-ref HEAD && git rev-parse HEAD && git rev-parse origin/develop
cd /home/ubuntu/workspace/repos/codex && git diff --name-only origin/develop -- src/rules_core/pilot_compute.rs src/rules_core/support_state_matrix.rs src/oracle_validation/support_state_matrix.rs tests/sd13_support_state_matrix.rs tests/sd13_hybrid_level1_chassis_baseline.rs tests/ge06_pilot_input_contract.rs tests/ge06_pilot_total_saves.rs tests/ge06_pilot_combat_baseline.rs tests/ge06_pilot_selected_skill_modifiers.rs tests/ge06_pilot_headless_receipt.rs tests/ge06_failure_classifier.rs tests/ge06_pilot_view_model.rs tests/fixtures/rules_core
```

Interpretation:
- if the target working copy still lags `origin/develop` on the listed files, the worker must sync to accepted `develop` truth or launch from a clean worktree before claiming SD13-E4 evidence

### Required regression / acceptance floor inherited from accepted current truth
```bash
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test --test ge06_pilot_input_contract --test ge06_pilot_total_saves --test ge06_pilot_combat_baseline --test ge06_pilot_selected_skill_modifiers --test ge06_pilot_headless_receipt --test ge06_failure_classifier --test ge06_pilot_view_model --test sd13_support_state_matrix --test sd13_hybrid_level1_chassis_baseline
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test
```

Verification interpretation:
- the focused regression floor is mandatory because it protects the accepted Human race seam, Fighter 1-3 truth, Rogue blocker truth, downstream receipt/classifier/view-model propagation, and the hybrid blocked/computed rows that E4 must not flatten
- full `cargo test` is a smoke/regression sweep only; it does not upgrade any SD13 support-state claim by itself
- the later `SD13-E4-R2` handoff must add one exact new dedicated spell-bearing proof command once it freezes the first bounded `F7` proof file path

## Readiness verdict
This lane is ready for handoff authoring now.

Why it is ready:
- accepted `origin/develop` truth is now sharp enough to separate hybrid baseline truth from later spell burden without ambiguity
- the real focused regression floor was re-run successfully against a clean accepted worktree during this closure
- the live repo now shows the architectural split the later handoff must preserve: hybrid Paladin/Ranger rows are explicitly blocked/computed for named reasons, while full-caster rows remain pure `Unverified` / `Observed`
- `epic-breakdown.md` and `artifacts/level-10-progression-validation-contract.md` already split arcane spell-bearing work from divine/hybrid burden, making `SD13-F7` the first truthful E4 execution family

Why it is not yet a direct code-authorizing outcome by itself:
- this card does not author the stage-specific `SD13-E4-R2` handoff prose
- it does not yet freeze the first exact bounded `F7` spell-bearing slice, proof file path, or allowed write scope
- it does not justify treating Paladin/Ranger hybrid baseline evidence as spell-support closure
- it does not authorize a broad spell-system rewrite or a combined all-casters tranche

## Successor truth
The earned successor is:
- `SD13-E4-R2 FLOW: Spellcasting and hybrid level-10 progression handoff artifact`

That successor should convert this closure into a stage-specific, code-authorizing brief for the first truthful `SD13-F7` spell-bearing tranche while keeping hybrid Paladin/Ranger spell burden and the broader divine/hybrid `SD13-F8` family visibly out of the first slice unless their distinct burden can be named and bounded exactly.