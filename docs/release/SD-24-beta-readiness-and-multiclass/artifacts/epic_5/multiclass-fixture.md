# Criterion 5.2 — Deterministic test surface: 30 character-advancement cycles

Per `epic-breakdown.md` criterion 5.2 and `technical-design.md §2.2`, this artifact
records the per-cycle input/output for the 30-cycle deterministic walk landed in
`tests/sd24_multiclass_deterministic.rs`.

All 30 cycles are asserted against PF1's canonical formulas, computed
independently in the test file (not copied from `pilot_compute.rs`'s own
internals):

- Fighter: full (1/1) base attack bonus; good Fortitude (`level/2 + 2`
  fractional, floored for a solo class); poor Reflex/Will (`level/3`
  fractional, floored for a solo class).
- Wizard: half (1/2) base attack bonus (`floor(level/2)`); poor
  Fortitude/Reflex (`level/3` fractional, floored for a solo class); good
  Will (`level/2 + 2` fractional, floored for a solo class).
- Multiclass mix (Fighter + Wizard): base attack bonus is the additive sum
  of each class's own independently-computed base attack bonus; each base
  save sums the two classes' own *unrounded* fractional contributions and
  floors only once for the total (SD-21 E7.29's rule).

## Cycles 1-10 — solo Fighter, level 1 → 10

Input: `pf1_human_fighter9_wizard1_sd24_multiclass_lv10_input.txt`'s own posture,
cloned with `class_levels = [class:fighter:<level>]` for `level` 1..10.

| Cycle | Input (class_levels) | Output BAB | Output Fort/Ref/Will | Claim-blocked? |
|---|---|---|---|---|
| 1 | fighter:1 | 1 | 2/0/0 | no |
| 2 | fighter:2 | 2 | 3/0/0 | no |
| 3 | fighter:3 | 3 | 3/1/1 | no |
| 4 | fighter:4 | 4 | 4/1/1 | no |
| 5 | fighter:5 | 5 | 4/1/1 | no |
| 6 | fighter:6 | 6 | 5/2/2 | no |
| 7 | fighter:7 | 7 | 5/2/2 | no |
| 8 | fighter:8 | 8 | 6/2/2 | no |
| 9 | fighter:9 | 9 | 6/3/3 | no |
| 10 | fighter:10 | 10 | 7/3/3 | no |

## Cycles 11-20 — solo Wizard, level 1 → 10

Input: `pf1_human_wizard9_fighter1_sd24_multiclass_lv10_input.txt`'s own posture,
cloned with `class_levels = [class:wizard:<level>]` for `level` 1..10.

| Cycle | Input (class_levels) | Output BAB | Output Fort/Ref/Will | `class_chassis.spell_baseline.wizard` present? |
|---|---|---|---|---|
| 11 | wizard:1 | 0 | 0/0/2 | yes |
| 12 | wizard:2 | 1 | 0/0/3 | yes |
| 13 | wizard:3 | 1 | 1/1/3 | yes |
| 14 | wizard:4 | 2 | 1/1/4 | yes |
| 15 | wizard:5 | 2 | 1/1/4 | yes |
| 16 | wizard:6 | 3 | 2/2/5 | yes |
| 17 | wizard:7 | 3 | 2/2/5 | yes |
| 18 | wizard:8 | 4 | 2/2/6 | yes |
| 19 | wizard:9 | 4 | 3/3/6 | yes |
| 20 | wizard:10 | 5 | 3/3/7 | yes |

## Cycles 21-25 — Fighter-side split-advance, total level 6 → 10

Split point (total level 5, Fighter 4 / Wizard 1) is criterion 5.1's own test
surface (`sd24_multiclass_fighter_wizard_split.rs`) and is not re-counted
here. Input: `pf1_human_fighter9_wizard1_sd24_multiclass_lv10_input.txt`'s own
posture, cloned with `class_levels = [class:fighter:<f>, class:wizard:1]`.

| Cycle | Total level | Input (class_levels) | Output BAB | Output Fort/Ref/Will |
|---|---|---|---|---|
| 21 | 6 | fighter:5, wizard:1 | 5 | 4/2/4 |
| 22 | 7 | fighter:6, wizard:1 | 6 | 5/2/4 |
| 23 | 8 | fighter:7, wizard:1 | 7 | 5/2/4 |
| 24 | 9 | fighter:8, wizard:1 | 8 | 6/3/5 |
| 25 | 10 | fighter:9, wizard:1 | 9 | 6/3/5 |

Every one of these 5 cycles also asserts `class_chassis.spell_baseline.wizard`
stays present — this is the exact surface criterion 5.1's fix
(`wizard_level_in_mix`) protects, now proven across the whole Fighter-side
advancement walk, not just its two endpoints.

## Cycles 26-30 — Wizard-side split-advance, total level 6 → 10

Mirror image. Input: `pf1_human_wizard9_fighter1_sd24_multiclass_lv10_input.txt`'s
own posture, cloned with `class_levels = [class:fighter:1, class:wizard:<w>]`.

| Cycle | Total level | Input (class_levels) | Output BAB | Output Fort/Ref/Will |
|---|---|---|---|---|
| 26 | 6 | fighter:1, wizard:5 | 3 | 4/2/4 |
| 27 | 7 | fighter:1, wizard:6 | 4 | 4/2/5 |
| 28 | 8 | fighter:1, wizard:7 | 4 | 4/2/5 |
| 29 | 9 | fighter:1, wizard:8 | 5 | 4/2/6 |
| 30 | 10 | fighter:1, wizard:9 | 5 | 5/3/6 |

Every one of these 5 cycles also asserts `class_chassis.spell_baseline.wizard`
stays present.

## RED → GREEN evidence

RED demonstrated live by temporarily reverting the three source files this
bundle's Epic 5 owns (`pilot_compute.rs`, `level_up/fighter.rs`,
`level_up/wizard.rs`) to their pre-criterion-5.1-fix state
(`git checkout c3330b6 -- <files>`, the commit immediately preceding the
5.1 fix commit `0068818`), then running
`cargo test --locked --test sd24_multiclass_deterministic`:

- The two solo cycles (1-10, 11-20) passed even with the fix reverted
  (expected — they never touch the multiclass gate).
- The two mix cycles (21-25, 26-30) both FAILED, each panicking on the
  assertion `Wizard {N} / Fighter {M} must keep Wizard's own spell-baseline
  recognition surfaced inside the mix` — the exact defect criterion 5.1
  fixed (`class_chassis.spell_baseline.wizard` silently absent from every
  multiclass mix's explanations).

Restoring the fix (`git checkout HEAD -- <files>`) and re-running:
`4 passed; 0 failed`.

## Cycle artifact

- **Test file:** `tests/sd24_multiclass_deterministic.rs`
- **Cycle receipt:** `./fighter-wizard-multiclass-deterministic-cycle_cycle_receipt.md`
