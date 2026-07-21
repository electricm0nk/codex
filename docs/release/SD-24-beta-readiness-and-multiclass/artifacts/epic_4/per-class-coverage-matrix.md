# Epic 4 — Per-Class Coverage Matrix

Per `epic-breakdown.md` criterion 4.1's cycle artifact
(`./artifacts/epic_4/per-class-coverage-matrix.md`). This file is the
epic-level roll-up; per-class detail lives in
`./artifacts/epic_4/class_<name>_coverage.md`.

## Scope of this cycle

This cycle's granted file-touch scope (per `loop-instruction.md` §2.4's
own partition table, which — unlike `epic-breakdown.md`'s criterion 4.1
header text naming all 11 CRB classes — names only `class_fighter.rs` /
`class_wizard.rs` as Epic 4's owned files) is:

- `src/rules_core/rules_tables/crb/class_tables.rs` (generic BAB/save
  table, shared by all 11 CRB classes — read-only, no defect found)
- `src/rules_core/level_up/fighter.rs`
- `src/rules_core/level_up/wizard.rs`

Fighter and Wizard are also the two classes Epic 5's multiclass work is
scoped to (per operator directive 2026-07-21, `decisions.md` §4), so this
cycle's audit is the one that actually gates Epic 5's dependency
("Epic 5 is gated on Epic 4's coverage-matrix output"). The remaining 9
CRB classes (Cleric, Rogue, Sorcerer, Barbarian, Bard, Druid, Monk,
Paladin, Ranger) each already have a landed `level_up/<class>.rs` module
from SD-20 Epic 7, but auditing them is outside this cycle's granted file
scope; forwarded as a `## DISCOVERED` item for a follow-on cycle if
full-11-class breadth is later required (not gating Epic 5, since Epic 5
is Fighter+Wizard-only).

## Per-class summary

| Class | Book | `class_features_expected` | `class_features_wired` (before) | `class_features_wired` (after) | Gap(s) | Coverage artifact |
|---|---|---|---|---|---|---|
| Fighter | CRB | 10 | 10 | 10 | none | `./class_fighter_coverage.md` |
| Wizard | CRB | 12 | 7 | 12 | `class_spell.wizard.*` (5 explanation families) silently dropped from `LevelUpPlan` — **fixed this cycle** | `./class_wizard_coverage.md` |

## Audit method

For each class, `class_features_expected` counts the named class-feature
pillars `pilot_compute.rs` itself grounds (as `ComputationExplanation`
records, excluding the generic BAB/save columns already covered by
`class_tables()`) for that class. `class_features_wired` counts how many
of those pillars the class's `level_up/<class>.rs` module's explanation
filter actually surfaces into the `LevelUpPlan.automatic_features` list.
A pillar `pilot_compute.rs` does not ground at all for a class (e.g.
Wizard's Arcane Bond) is excluded from `class_features_expected` entirely
— it is a `pilot_compute.rs`-level scope boundary, not a `level_up`
wiring gap, and is recorded as a boundary note in the per-class artifact
rather than a "gap."

## Remediation plan input (criterion 4.4)

| Class | Missing feature | Fix | Priority |
|---|---|---|---|
| Wizard | `class_spell.wizard.*` dropped from `LevelUpPlan` | Add `class_spell.wizard.` prefix to `is_wizard_pillar` filter in `src/rules_core/level_up/wizard.rs` | P1 — **remediated this cycle**, no carry-forward |

## Epic 5 dependency status

Epic 5 (Multiclass, Fighter+Wizard only) may proceed: both classes'
`LevelUpPlan` composers are now free of known wiring gaps within this
audit's scope. This satisfies "Epic 5 is gated on Epic 4's coverage-matrix
output" for the Fighter+Wizard pair Epic 5 actually needs.
