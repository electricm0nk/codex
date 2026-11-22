# Epic 4 — Class Coverage Audit: Wizard (CRB, criterion 4.1)

## DISCOVERED — plan-vs-reality path correction

Same correction as `class_fighter_coverage.md`: `content-unit-inventory.md`
§3.1's `src/rules_core/rules_tables/<book>/class_<name>.rs` path does not
exist. Wizard's real per-class wiring is
`src/rules_core/rules_tables/crb/class_tables.rs` (generic BAB/save,
shared with all 11 CRB classes) + `src/rules_core/level_up/wizard.rs`
(the class-specific `LevelUpPlan` composer,
`compute_wizard_level_up_grants`). This audit is scored against those two
real modules.

## Canonical feature source

PF1 Core Rulebook Wizard class table + class features, cross-checked
against `pilot_compute.rs`'s own already-primary-source-cited grounding
(`explain_wizard_level1_prepared_spell_baseline` for the base chassis and
class-specific pillars; `ground_wizard_prepared_spellbook` /
`unmet_wizard_spellbook_conditions` for the prepared-spellbook posture).

## Feature inventory (`class_features_expected`) vs. wiring (`class_features_wired`)

| # | Pillar | `pilot_compute.rs` id(s) | Wired into `LevelUpPlan` before this cycle? | After this cycle? |
|---|---|---|---|---|
| 1 | Base attack bonus / Fort / Ref / Will base saves (half BAB, good Will only) | `class_tables()` columns | Yes | Yes (unchanged) |
| 2 | Wizard level-1 prepared arcane spell-bearing recognition | `class_chassis.spell_baseline.wizard` | Yes | Yes (unchanged) |
| 3 | Scribe Scroll (universal 1st-level bonus feat) | `class_chassis.wizard.scribe_scroll` | Yes | Yes (unchanged) |
| 4 | Arcane school specialization choice recognition | `class_chassis.wizard.specialization_choice` | Yes | Yes (unchanged) |
| 5 | Specialist bonus spell slot flat-count ladder | `class_chassis.wizard.specialist_bonus_slot` | Yes | Yes (unchanged) |
| 6 | Intense Spells bonus damage (Evocation) | `class_chassis.wizard.intense_bonus_damage` | Yes | Yes (unchanged) |
| 7 | Force Missile uses-per-day pool (Evocation) | `class_chassis.wizard.force_missile_uses_per_day` | Yes | Yes (unchanged) |
| 8 | Spellbook contents (recorded spells) | `class_spell.wizard.spellbook_contents` | **No — GAP** | **Yes (fixed this cycle)** |
| 9 | Daily preparation selection | `class_spell.wizard.daily_preparation` | **No — GAP** | **Yes (fixed this cycle)** |
| 10 | Base spells per day, per spell level 0-3 | `class_spell.wizard.base_spells_per_day.spell_level_{0,1,2,3}` | **No — GAP** | **Yes (fixed this cycle)** |
| 11 | Intelligence bonus spells per day, per spell level 0-3 | `class_spell.wizard.intelligence_bonus_spells_per_day.spell_level_{0,1,2,3}` | **No — GAP** | **Yes (fixed this cycle)** |
| 12 | Total spells per day, per spell level 0-3 | `class_spell.wizard.total_spells_per_day.spell_level_{0,1,2,3}` | **No — GAP** | **Yes (fixed this cycle)** |
| 13 | Arcane Bond (bonded object/familiar) | none — not grounded anywhere in `pilot_compute.rs` | No | No (correctly not fabricated — `pilot_compute.rs`'s own boundary, not a `level_up/wizard.rs` wiring gap) |
| 14 | Evocation school-power execution (spell-damage application, Force Missile casting) | `class_feature.wizard.school_powers_and_opposed_school_cost.unsupported` (claim-blocking diagnostic while unmet) | No | No (correctly not fabricated — `pilot_compute.rs`'s own boundary; the flat *magnitudes* in rows 6-7 above are wired, only the *execution* is not grounded anywhere to wire) |
| 15 | Spellbook/spell-slot posture beyond wizard level 3, or without the canonical Evocation specialization | `class_spell.wizard.prepared_spellbook.unsupported` (claim-blocking diagnostic while unmet) | No | No (correctly not fabricated — `pilot_compute.rs`'s own `WIZARD_SPELLBOOK_SUPPORTED_MAX_LEVEL = 3` boundary) |
| 16 | Spell save DCs | not grounded anywhere in `pilot_compute.rs` | No | No (not a wiring gap — nothing to wire) |
| 17 | Bonus-feat choice list (metamagic / item creation / Spell Mastery at levels 5+/10+) | no candidate catalog in `rules_tables::crb` | No | No — `pick_from_lists` stays empty, documented boundary (mirrors Fighter's own bonus-feat-candidate-list note) |

**Root cause of the gap (rows 8-12):** `src/rules_core/level_up/wizard.rs`'s
`append_class_feature_grants` filtered `to_explanations` to ids equal to
`WIZARD_RECOGNITION_ID` or prefixed `WIZARD_EXPLANATION_PREFIX =
"class_chassis.wizard."`. The `class_spell.wizard.*` explanation family
(landed later, by SD-21 E6b.2's `ground_wizard_prepared_spellbook`) never
matched that prefix, so every real spellbook/spells-per-day fact was
silently dropped from the `LevelUpPlan` even once a wizard's prepared-
spellbook posture was fully met (canonical Evocation specialization,
levels 1-3, a consistent recorded/prepared spellbook within the per-level
slot budget). The module's own doc comment (pre-cycle) still claimed this
posture was "still named by `pilot_compute.rs`'s own live
`class_spell.wizard.prepared_spellbook.unsupported` diagnostic" — true
when `level_up/wizard.rs` first landed (SD-20 Epic 7), stale once SD-21
added the real grounding.

`class_features_expected` (wireable pillars only, excluding rows
13/14/15/16/17 which have no source to wire from) = **12** (rows 1-12,
with row 1 counted once as the shared generic-table pillar).
`class_features_wired` before this cycle = **7** (rows 1-7).
`class_features_wired` after this cycle = **12 of 12 (100%)**.

## RED → GREEN evidence

- **RED:** `tests/sd24_wizard_level_up_spell_coverage.rs` —
  `leveling_up_into_a_supported_spellbook_posture_grants_the_real_spell_facts`
  drives `compute_wizard_level_up_grants` for a Human Wizard (canonical
  Evocation specialization) with a populated spellbook, from level 2
  (posture unmet — one recorded/prepared spell targets 2nd level, not yet
  accessible) to level 3 (posture met). Failed pre-fix: `plan.automatic_features`
  contained only the two generic `class_tables()` save-column grants plus
  the pre-existing `specialist_bonus_slot` chassis grant — no
  `class_spell.wizard.*`-sourced grant at all.
- **GREEN (fix):** added `WIZARD_SPELL_EXPLANATION_PREFIX =
  "class_spell.wizard."` to `src/rules_core/level_up/wizard.rs`'s
  `is_wizard_pillar` filter (and to `friendly_name`'s prefix-trim chain).
  Test passes: `spellbook_contents`, `daily_preparation`, and the
  newly-accessible `total_spells_per_day.spell_level_2` grant all appear.
- **Regression check:** `tests/sd21_wizard_prepared_spellbook.rs` (the
  base-chassis-level test suite this fix composes with) — 5 passed, 0
  failed, unchanged.

## Audit verdict

**1 genuine gap found and remediated.** Wizard's `LevelUpPlan` composer
was dropping every real spellbook/spells-per-day fact `pilot_compute.rs`
grounds once the bounded prepared-spellbook posture (levels 1-3, canonical
Evocation specialization) is met. Fixed in
`src/rules_core/level_up/wizard.rs` this cycle.

## Remediation plan (criterion 4.4 input)

| Missing feature | Fix | Cycle | Status |
|---|---|---|---|
| `class_spell.wizard.*` (5 explanation families) dropped from `LevelUpPlan` | Add `class_spell.wizard.` to the `is_wizard_pillar` filter | this cycle (`4.1`) | **DONE** |

## Gap priority

P1 (silent data loss on a real, already-grounded compute path — a wizard
who levels up loses visibility into her own newly-accessible spell slots
in the Level-Up UI surface, even though the underlying math was already
correct) — resolved same-cycle, no `## Open blockers` entry needed.

## Boundary notes (not gaps, not remediated — `pilot_compute.rs`'s own scope)

Arcane Bond, Evocation school-power execution, spellbook support beyond
level 3 or outside the canonical specialization, spell save DCs, and the
bonus-feat-choice candidate list (rows 13-17 above) all remain ungrounded
*in `pilot_compute.rs` itself* — there is nothing there for
`level_up/wizard.rs` to compose with. Widening any of them is a
`pilot_compute.rs`-level design decision outside this audit's granted
file scope (`class_tables.rs`, `level_up/fighter.rs`, `level_up/wizard.rs`
only).
