# SD13-E2-R1 Core Race Semantic Readiness Closure — 2026-06-30

## Card outcome
- evidence_class: `documentary-artifact`
- readiness_verdict: `codex-ready-for-handoff-authoring`
- route truth: this card closes as a documentary readiness artifact only; it does not create repo code, a branch, a PR, or a merge surface
- next board move if accepted: continue into the already-named successor `SD13-E2-R2 FLOW: Core race semantic coverage handoff artifact`

## Live repo truth grounded on 2026-06-30
- `git branch --show-current` reports `feat/sd13-e1-f1-rules-core-support-state-matrix`.
- `git rev-parse HEAD` reports `3827378a5bfe6dda22ad18695140d7f4fa723a5f`.
- `git rev-parse origin/develop` reports `c2cea5c6baeb3ca34077b85331214c4b42a4809c`.
- `gh pr view 35 --json number,title,headRefName,baseRefName,state,url,mergeStateStatus,commits` reports an open clean PR on `feat/sd13-e1-f1-rules-core-support-state-matrix` targeting `develop`: `https://github.com/electricm0nk/codex/pull/35`.
- This matters because the earlier SD13-E1 readiness artifacts still recorded the older local branch `sd11-f10-update-action-surface` at `a0a90dcdf75cd0cc09d4b71e9fb7d3b440aaf293`. A truthful SD13-E2 handoff must not copy those stale branch facts forward.
- `/home/ubuntu/workspace/repos/codex/README.md` still states that Codex is a developer proof harness plus bounded desktop workbench surface, not a general character builder or broad Pathfinder product.
- `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs` now exists and seeds the SD-13 current-truth matrix: Human race remains `Partial` / `Computed`, the Human interaction seam remains `Partial` / `Computed`, Fighter level 1 remains `Partial` / `Computed`, Fighter levels 2-10 and Rogue remain `Blocked` / `Computed`, and every non-Human core race remains `Unverified` / `Observed`.
- `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs` proves that only five seeded rows rise above `Observed`, that there are no `Supported` rows, and that the matrix still treats race rows, class rows, and interaction rows as separate units of truth.
- `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs` already carries `race_id`, `selected_choices`, `selected_feats`, `skill_allocations`, and `equipment_selections`. That is enough structural input surface to describe the currently grounded Human seam and to host later bounded race-linked input evidence without inventing a broader parser or schema subsystem first.
- `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs` is still the only live rules-core seam that converts bounded chosen input into computed evidence, explanations, and claim-blocking diagnostics. It remains Fighter-level-1-centric and does not yet provide bounded race-semantic evidence beyond the current Human deterministic seam.
- `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_view_model.rs` still projects the existing headless receipt without adding a separate race-semantic projection layer.
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs` proves the accepted deterministic fixture still names only `race:human` plus `class:fighter:1`, with Human bonus-feat and Human ability-bonus selections preserved as explicit chosen-input truth.
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs` and `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs` still claim-block Rogue level 1 and Fighter level 2. Those blockers remain class-progression truth, not reasons to stall the narrower race-semantic handoff lane.
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_selected_skill_modifiers.rs`, `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_headless_receipt.rs`, `/home/ubuntu/workspace/repos/codex/tests/ge06_failure_classifier.rs`, and `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_view_model.rs` prove that any change to `pilot_compute.rs` can propagate into computed skills, headless receipt status, blocker ownership, and view-model projection. A truthful E2 handoff therefore cannot pretend that `pilot_compute.rs` is isolated from those regression surfaces.
- `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`, `.../unsupported-partial-lossy-and-unverified-semantics-ledger.md`, `technical-requirements.md`, `technical-design.md`, and `coverage-evidence-and-fixture-plan.md` already define the exact race-semantic obligation: advance race truth separately from class truth, require explicit race-semantics fixture families, and preserve named interaction rows only where the combination materially changes support truth.

## Actual verification run during this closure
The following commands were run successfully on the live repo during this card:

```bash
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test --test sd13_support_state_matrix --test ge06_pilot_input_contract --test ge06_pilot_total_saves --test ge06_pilot_combat_baseline --test ge06_pilot_view_model
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test --test ge06_pilot_selected_skill_modifiers --test ge06_pilot_headless_receipt --test ge06_failure_classifier
```

Observed result:
- `sd13_support_state_matrix`: 18 passed, 0 failed
- `ge06_pilot_input_contract`: 2 passed, 0 failed
- `ge06_pilot_total_saves`: 3 passed, 0 failed
- `ge06_pilot_combat_baseline`: 4 passed, 0 failed
- `ge06_pilot_view_model`: 2 passed, 0 failed
- `ge06_pilot_selected_skill_modifiers`: 5 passed, 0 failed
- `ge06_pilot_headless_receipt`: 2 passed, 0 failed
- `ge06_failure_classifier`: 5 passed, 0 failed
- aggregate for the focused closure run: 41 passed, 0 failed

This matters because the next handoff can name a real regression floor for any `pilot_compute.rs` or `support_state_matrix.rs` change instead of bluffing broader race coverage.

## Exact required reads for the later handoff artifact
The stage-specific handoff produced by `SD13-E2-R2` should require reading exactly these surfaces, in order:
1. `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
2. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
3. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e2-r1-core-race-semantic-readiness-closure-2026-06-30.md`
4. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md`
5. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md`
6. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-requirements.md`
7. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-design.md`
8. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`
9. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md`
10. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/coverage-evidence-and-fixture-plan.md`
11. `/home/ubuntu/workspace/repos/codex/README.md`
12. `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs`
13. `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
14. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
15. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_view_model.rs`
16. `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs`
17. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`
18. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs`
19. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`
20. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_selected_skill_modifiers.rs`
21. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_headless_receipt.rs`
22. `/home/ubuntu/workspace/repos/codex/tests/ge06_failure_classifier.rs`
23. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_view_model.rs`
24. `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`

## Exact candidate repo surfaces and allowed write scope for the later handoff
The first honest repo-facing SD13-E2 lane is not a broad “all races” sprint. The next handoff should keep default write authority inside the existing `rules_core` truth surfaces and the focused regression tests they already drive.

### Primary candidate write paths
1. `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs`
   - authoritative surface for promoting or preserving SD-13 race rows and named interaction rows
   - may update row states, grounding refs, blocker notes, and next-uplift notes only in ways justified by new bounded evidence
   - must remain documentary/control-plane truth only; no parser, serialization, UI, or product-claim logic

2. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
   - the only live repo seam that can turn race-linked chosen-input truth into computed evidence, explanations, or claim-blocking diagnostics
   - may be touched only for bounded race-semantic evidence generation tied directly to the later handoff’s named roster slice
   - must not widen class progression, spell burden, multiclassing, or general rules-engine scope

3. `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs`
   - mandatory matrix-truth gate for any change that alters race-row or interaction-row posture
   - must keep support state and evidence tier separate and must keep class truth separate from race truth

4. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`
   - regression gate for chosen-input truth, especially `race_id` and the currently grounded Human interaction selections

5. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_selected_skill_modifiers.rs`
   - regression gate if bounded race semantics alter skill-facing outputs or diagnostics through `pilot_compute.rs`

6. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_headless_receipt.rs`
   - regression gate if bounded race semantics alter receipt status or explanation carriage

7. `/home/ubuntu/workspace/repos/codex/tests/ge06_failure_classifier.rs`
   - regression gate if new race-semantic blockers or partial paths change which owner should be reported

8. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_view_model.rs`
   - regression gate if new race-semantic evidence is surfaced through the current view-model projection path

### Read-only grounding seams for the first E2 code lane
These files must be treated as grounding truth unless a later readiness pass explicitly opens them for write authority:
- `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
- `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_view_model.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`
- `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`

### Explicit write-scope rule
The later handoff should authorize writes only to the eight primary candidate paths above by default.

If the later handoff concludes that truthful race-semantic progress requires opening any of the read-only grounding seams or inventing a new fixture/test family, it must say so explicitly in the handoff artifact and justify why the narrower eight-path surface is insufficient. It may not widen scope silently.

## Exact verification commands for the later handoff
The next handoff should name these commands explicitly as the minimum regression floor:

```bash
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test --test sd13_support_state_matrix
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test --test ge06_pilot_input_contract --test ge06_pilot_total_saves --test ge06_pilot_combat_baseline --test ge06_pilot_selected_skill_modifiers --test ge06_pilot_headless_receipt --test ge06_failure_classifier --test ge06_pilot_view_model
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test
```

Verification interpretation:
- `sd13_support_state_matrix` is the acceptance gate for race-row and interaction-row truth updates
- the six focused GE-06 tests are mandatory regression protection because `pilot_compute.rs` changes can alter input-shape truth, computed outputs, receipt status, blocker ownership, and view-model projection
- full `cargo test` is a smoke/regression sweep only; it does not by itself upgrade any SD-13 breadth claim

## Exact non-goals for the later handoff
The next handoff must state these non-goals plainly:
- no claim that the repo now supports the full seven-race by eleven-class roster
- no claim that any non-Human core race is already `supported`
- no claim that class progression beyond the existing Fighter level-1 proof is solved
- no spellcasting burden work
- no multiclassing, prestige-class, archetype, alternate-racial-trait, or non-core expansion
- no tester-surface, GitHub issue-routing, or support-language work under SD-11
- no distribution/update/channel work under SD-12
- no persistence/lifecycle work under SD-14
- no rewrite of `README.md`, `AGENTS.md`, `CLAUDE.md`, or governance documents as a substitute for repo evidence
- no fake 77-combination race/class completion theater
- no silent widening into `character_input.rs`, `pilot_view_model.rs`, new fixture families, or unrelated GE-06 regression bodies without explicit justification in the handoff

## Readiness verdict
This lane is ready for handoff authoring now.

Why it is ready:
- the live repo now contains the seeded SD-13 matrix carrier and its proving test surface, so the handoff does not have to invent the race-truth control plane from nothing
- the repo already exposes one narrow compute seam (`pilot_compute.rs`) that can host bounded race-semantic evidence without forcing a product-wide architecture jump
- the current program artifacts already define the race-semantic requirement families, the interaction-row doctrine, and the fixture-family expectations for a truthful E2 slice
- the real regression floor is known and was exercised successfully during this closure
- the class-progression blockers for Rogue and Fighter level 2 do not prevent authoring a narrower race-semantic handoff, as long as that handoff does not counterfeit class breadth or full interaction closure

Why it is not yet a direct code-authorizing outcome by itself:
- this card does not freeze the exact handoff prose for the later CODE lane
- it does not decide whether the first E2 code slice stays inside the default eight-path write surface or needs an explicitly widened fixture/input surface
- it does not itself create Claude execution, a pushed branch, a PR, or a merge receipt

## Successor truth
The earned successor remains:
- `SD13-E2-R2 FLOW: Core race semantic coverage handoff artifact`

That successor should convert the bounded truth above into a stage-specific handoff artifact for the first repo-facing E2 code lane, anchored on the current accepted SD13-E1 matrix commit/PR truth rather than the stale pre-E1 branch snapshot.