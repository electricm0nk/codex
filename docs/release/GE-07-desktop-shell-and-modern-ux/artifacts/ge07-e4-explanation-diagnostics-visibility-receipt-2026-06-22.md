---
title: GE07-E4 Explanation and Diagnostics Visibility Receipt
artifact_type: verification-receipt
stc_id: STC-CODEX-GE-07
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE07-E4 — Explanation and diagnostics surfaces
workflow_route: planning
readiness: planning-ready
review_date: 2026-06-22
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE07-E4 Explanation and Diagnostics Visibility Receipt

## Objective
Prove which explanation and diagnostics payloads are already visible in the live Codex headless/import surfaces before any shell slice claims product progress.

## Verification commands run
In `/home/ubuntu/workspace/repos/codex`:
- `git rev-parse --short origin/develop`
- `git branch --show-current`
- `git rev-parse --short HEAD`
- `git merge-base --is-ancestor HEAD origin/develop && echo HEAD_IS_ANCESTOR_OF_ORIGIN_DEVELOP`
- `git ls-tree -r --name-only origin/develop | grep -E '^(apps/desktop/|src-tauri/)' || true`
- `git ls-tree -r --name-only origin/develop | grep -E '^(src/rules_core/pilot_view_model.rs|tests/ge06_pilot_view_model.rs)' || true`
- `"$HOME/.cargo/bin/cargo" test --test ge06_pilot_headless_receipt --quiet`
- `"$HOME/.cargo/bin/cargo" test --test ge06_failure_classifier --quiet`
- `"$HOME/.cargo/bin/cargo" test --test ge06_pilot_combat_baseline --quiet`
- `"$HOME/.cargo/bin/cargo" test --test ge06_pilot_input_contract --quiet`
- `"$HOME/.cargo/bin/cargo" test --test character_input_record --quiet`
- `"$HOME/.cargo/bin/cargo" test --test pcc_entry_parse --quiet`

In the temporary probe workspace `/home/ubuntu/.hermes/kanban/boards/codex/workspaces/t_71b6d8d0/explanation_probe`:
- `"$HOME/.cargo/bin/cargo" run --quiet`

## Observed verification results
- `origin/develop = 7bc89e8`
- checked-out branch during the probe was `ge06-e3-f2-classifier-impl` at `cc45f2c`
- `git merge-base --is-ancestor HEAD origin/develop` succeeded, so the probe ran against code already contained by the current `origin/develop` lineage
- `git ls-tree` showed no `apps/desktop/` or `src-tauri/` entries on `origin/develop`
- `git ls-tree` showed no `src/rules_core/pilot_view_model.rs` or `tests/ge06_pilot_view_model.rs` on `origin/develop`
- the six targeted test files all passed: 2 + 5 + 4 + 2 + 3 + 2 tests, respectively

## Computed receipt explanation payloads observed live
The probe built the deterministic headless receipt and observed:
- `case_id = pf1-crb-human-fighter-level1`
- `source_package_id = pf1.core_rulebook`
- `status = Computed`
- `primary_owner = OracleGap`
- `explanation_count = 18`
- `claim_blocking_diagnostics = 0`

Representative explanation details observed verbatim from the live payloads:
1. `ability_modifier.strength`
   - `strength ability modifier from chosen score 16: floor(16 / 2) - 5 = 3`
2. `combat.baseline_melee_attack_bonus`
   - `Baseline melee attack bonus for the Longsword: Fighter base attack bonus (+1) + Strength modifier (+3) + Weapon Focus (Longsword) (+1); Power Attack is selected but inactive (+0) = 5`
3. `defense.baseline_armor_class`
   - `Baseline armor class: base 10 + Chain Shirt armor bonus (+4) + Dexterity contribution (+2, DEX modifier +2 within MAXDEX:4) + Dodge (+1); shield is absent (+0) = 17`
4. `defense.total_save.fortitude`
   - `Total Fortitude save: Fighter base Fortitude save (+2) + Constitution modifier (+2) = 4`
5. `skill.selected_modifier.climb`
   - `Selected Climb modifier: rank 1 + Strength modifier (+3) + class-skill bonus (+3) + Chain Shirt armor-check penalty (-2) = 5`

## Blocked-route diagnostics and explanation withholding observed live
### Shield-equipped blocked posture
The probe mutated the deterministic fixture so the shield was equipped instead of absent.

Observed results:
- `status = Blocked`
- `primary_owner = EngineFlaw`
- `has_attack_explanation = false`
- `has_ac_explanation = false`
- `retains_strength_explanation = true`
- real claim-blocking diagnostic observed:
  - `combat.baseline_unsupported` — `baseline combat totals are only computed for the exact GE-06 deterministic Longsword/Chain Shirt/Dodge/no-shield posture; unmet conditions: item:shield must be Absent for the deterministic baseline, got Some(EquippedActive)`

Meaning:
- blocked values must not keep a faux explanation surface attached to values the engine refused to compute
- unrelated still-valid upstream truth may remain visible

### Rogue-chassis blocked posture
The probe mutated the deterministic fixture so the class level was Rogue 1 instead of Fighter 1.

Observed results:
- `status = Blocked`
- `primary_owner = EngineFlaw`
- real claim-blocking diagnostics observed:
  1. `class_chassis.unsupported`
  2. `combat.baseline_unsupported`
  3. `defense.total_save.unsupported`
  4. `skill.selected_modifier.unsupported`

Meaning:
- the blocked path already carries a real multi-diagnostic explanation burden
- the shell must not suppress this into one generic toast or one vague failure sentence

## Validation problems observed live
The probe fed an invalid equipment active-state token into the character-input loader.

Observed result:
- `character_input_present = false`
- structured diagnostic observed:
  - class: `InvalidCharacterInput`
  - severity: `Error`
  - `claim_blocking = true`
  - `subject_ref = equipment_selections`
  - message: `invalid character input equipment selection 'item:longsword:equiped_primary_active' has an unsupported state`

Meaning:
- validation problems already have stable class/severity/subject/message fields
- the shell must not replace these with an unstructured "invalid file" abstraction that hides the actionable subject

## Raw importer diagnostics observed live
The probe parsed a malformed PCC snippet containing `PCC:` with no target.

Observed result:
- `include_count = 1`
- `diagnostic_count = 1`
- structured diagnostic observed:
  - kind: `MalformedInclude`
  - line: `3`
  - raw line: `PCC:`
  - message: `PCC include directive has no target`

Meaning:
- importer warnings can already be structured with kind, source position, and verbatim evidence
- this is not yet the same thing as a pilot-shell-ready importer diagnostics projection

## Current visibility burden for a future GE07-E4 shell slice
A truthful explanation/diagnostics surface must therefore do all of the following:
1. show explanation detail from upstream payloads rather than recomputing or paraphrasing it into weaker product prose
2. preserve blocked-route posture (`Blocked`) and the real `primary_owner` when the pilot path fails
3. keep claim-blocking diagnostics visible with their stable ids/messages instead of collapsing them into one generic warning state
4. preserve structured validation-problem fields strongly enough that a user/operator can tell what failed and where
5. preserve structured importer warning fields strongly enough that malformed or unsupported import surfaces do not disappear
6. refuse to render explanations for values the engine withheld under an unsupported posture
7. keep explanation/diagnostic surfaces cross-linked back to the active value, route, or source context
8. remain explicit that invalid-choice reason payloads are not yet grounded in the live rules-core and therefore cannot be faked or silently omitted

## Why this receipt matters
This receipt answers the question GE07-E4 must not improvise:

What explanation and diagnostics truth already exists today, and what exactly must remain visible when a shell eventually renders it?

The answer is now grounded by live tests and a real probe, not by aspiration.

## Completion rule
This receipt is complete because it records executable proof for:
- real explanation detail payloads on the computed deterministic path
- real blocked-route diagnostics plus selective explanation withholding
- real validation-problem diagnostics from the input loader
- real raw importer diagnostics from the PCC parser
- the current absence of any merged shell scaffold or consumer bridge that would justify code authority now
