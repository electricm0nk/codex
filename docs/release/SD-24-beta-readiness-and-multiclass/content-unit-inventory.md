# SD-24 — Content Unit Inventory

> **Per-content-unit N-tuple.** Mirrors SD-22's `corpus-source-inventory.md` pattern. The column "Content shape" is illustrative only — production code reads the actual LST data per Epic 6, the actual class-feature tables per Epic 4, etc.

This document enumerates the per-content-unit shape that SD-24 cycles will encounter. Each row is the routing record for one content unit; the cycle's RED → GREEN phase reads the row, finds the actual content on disk, and renders the production code against the canonical source.

Per the operator-pinned doctrine-of-record at `~/workspace/governance/cross-canonical-doctrine.md`, this file is the row-routing table; the canonical content lives in the PCGen LST corpus at `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/` (per Epic 6's equipment coverage audit) or the canonical class-feature sources per Epic 4. The shape column here is not authoritative.

## 1. Equipment / Armor (Epic 6)

### 1.1 Per-item routing

| Content unit | Rust module path | Test fixture path | Cycle artifact path | Source canonical |
|---|---|---|---|---|
| `masterwork_backpack` | `src/rules_core/rules_tables/equipment/masterwork_backpack.rs` | `tests/sd24_equipment_masterwork_backpack.rs` | `artifacts/epic_6/masterwork_backpack_cycle_receipt.md` | PCGen LST `equipment_masterwork.lst` or `pathfinder_paizo/equipment.lst` |
| `full_plate_armor` | `src/rules_core/rules_tables/armor/full_plate.rs` | `tests/sd24_armor_full_plate.rs` | `artifacts/epic_6/full_plate_cycle_receipt.md` | PCGen LST `armor_medium.lst` and `armor_heavy.lst` |
| ... (n items) | per-item `*.rs` | per-item `*.rs` test | per-item cycle receipt | per-item PCGen LST source |

### 1.2 Required fields per item

| Field | Type | Required for beta tester scope |
|---|---|---|
| `name` | string | yes |
| `cost_gp` | u32 (or 0 for items without a cost) | yes (where applicable) |
| `weight_lbs` | f32 (or 0 for items without weight) | yes (where applicable) |
| `full_description` | string | yes |
| `category` | enum (weapon / armor / shield / wondrous / consumable / etc.) | yes |
| ... (mechanical fields) | various | per Epic 6's audit findings |

## 2. Spells (Epic 6, criterion 6.5)

### 2.1 Per-spell routing

| Content unit | Rust module path | Test fixture path | Cycle artifact path | Source canonical |
|---|---|---|---|---|
| `fireball` | `src/rules_core/rules_tables/spells/fireball.rs` | `tests/sd24_spell_fireball.rs` | `artifacts/epic_6/spell_fireball_cycle_receipt.md` | PCGen LST `spells-fire.lst` and `spell_list.lst` |
| ... | per-spell `*.rs` | per-spell test | per-spell cycle receipt | per-spell PCGen LST source |

### 2.2 Required fields per spell

| Field | Type | Required |
|---|---|---|
| `name` | string | yes |
| `school` | enum | yes |
| `level_<class>` | u8 (per class) | yes (where applicable) |
| `casting_time` | string | yes |
| `range` | string | yes |
| `components` | string (V/S/M/DF) | yes |
| `duration` | string | yes |
| `save` | enum (Fortitude / Reflex / Will / None) + DC | yes (where applicable) |
| `spell_resistance` | bool | yes |
| `description_full` | string (full text per SRD/PRD) | yes |

## 3. Classes — Per-class feature coverage (Epic 4)

### 3.1 Per-class routing

For each class, the row tracks `class_features_wired` vs `class_features_expected`. The audit cycle reads the row, computes the count from source, and emits a coverage-matrix entry.

| Class | Book | Canonical feature source | Cycle artifact |
|---|---|---|---|
| Fighter | CRB | `crb/classes/fighter.lst` + Pathfinder SRD core | `artifacts/epic_4/class_fighter_coverage.md` |
| Wizard | CRB | `crb/classes/wizard.lst` | `artifacts/epic_4/class_wizard_coverage.md` |
| Cleric | CRB | `crb/classes/cleric.lst` | `artifacts/epic_4/class_cleric_coverage.md` |
| ... (CRB: 11 classes) | CRB | per-class | per-class |
| Alchemist | APG | `apg/classes/alchemist.lst` | `artifacts/epic_4/apg_alchemist_coverage.md` |
| Cavalier | APG | `apg/classes/cavalier.lst` | `artifacts/epic_4/apg_cavalier_coverage.md` |
| ... (APG: 6 classes) | APG | per-class | per-class |
| Arcanist | ACG | `acg/classes/arcanist.lst` | `artifacts/epic_4/acg_arcanist_coverage.md` |
| Bloodrager | ACG | `acg/classes/bloodrager.lst` | `artifacts/epic_4/acg_bloodrager_coverage.md` |
| ... (ACG: 10 classes) | ACG | per-class | per-class |

### 3.2 Required fields per class coverage row

| Field | Description |
|---|---|
| `class_name` | per PCGen canonical |
| `book` | CRB / APG / ACG |
| `feature_table_path` | path to the canonical `.lst` source record |
| `feature_table_sha` | git SHA of the canonical source at audit time |
| `class_features_expected` | count of feature rows in the canonical source |
| `class_features_wired` | count of feature rows wired in `src/rules_core/rules_tables/<book>/class_<name>.rs` (or for CRB, split between `src/rules_core/rules_tables/crb/class_tables.rs` shared table + `src/rules_core/level_up/<class>.rs` class-specific) — (corrected 2026-07-22 per SD-25 criterion 7.P: CRB uses split structure with shared `class_tables.rs` + per-class `level_up/<class>.rs`, not single per-class files like APG/ACG) |
| `gap_features` | list of feature names from `expected` not present in `wired` |
| `gap_priority` | P0 / P1 / P2 / P3 (Epic 4 priority enumeration) | — (corrected 2026-07-22 per SD-25 criterion 7.P: APG uses 6 separate per-class files `apg/class_alchemist.rs`...`apg/class_witch.rs` + shared `apg/mod.rs`, opposite from CRB's shared table pattern; ACG uses one file per class `acg/class_arcanist.rs`...`acg/class_warpriest.rs` + `acg/mod.rs`) |

## 4. Tauri commands (Epic 7)

### 4.1 Per-command routing

| Tauri command | Rust module path | Test fixture path | Cycle artifact path |
|---|---|---|---|
| `appendToCharacter` | `apps/desktop/src-tauri/src/characterHub/appendToCharacter.rs` | inline test via `#[cfg(test)] mod tests` (no standalone file) — (corrected 2026-07-22 per SD-25 criterion 7.P: Tauri command tests are inline, not standalone `tests/` files) | `artifacts/epic_7/appendToCharacter_cycle_receipt.md` |
| `recomputeCharacter` | `apps/desktop/src-tauri/src/characterHub/recomputeCharacter.rs` | inline test via `#[cfg(test)] mod tests` (no standalone file) | `artifacts/epic_7/recomputeCharacter_cycle_receipt.md` |
| `reSaveCharacter` | `apps/desktop/src-tauri/src/characterHub/reSaveCharacter.rs` | inline test via `#[cfg(test)] mod tests` (no standalone file) | `artifacts/epic_7/reSaveCharacter_cycle_receipt.md` |

### 4.2 Required fields per Tauri command

| Field | Type | Required |
|---|---|---|
| `command_name` | PascalCase per identifier-discipline | yes |
| `invoke_string` | snake_case at the IPC layer | yes |
| `input_struct` | typed struct | yes |
| `output_struct` | typed struct (success + failure-with-reason variants) | yes |
| `cancellation` | bool | yes |
| `parallel_safe` | bool | yes |

## 5. Multiclass dispatch (Epic 5)

### 5.1 Per-class-pair routing

| Class pair | Test fixture path | Cycle artifact path | Scope |
|---|---|---|---|
| Fighter (1-10) | `tests/sd24_multiclass_fighter_lv10.rs` | `artifacts/epic_5/fighter_lv10_cycle_receipt.md` | in scope (Criterion 5.1, 5.2) |
| Wizard (1-10) | `tests/sd24_multiclass_wizard_lv10.rs` | `artifacts/epic_5/wizard_lv10_cycle_receipt.md` | in scope (Criterion 5.1, 5.2) |
| Fighter+Wizard (split class at lv5, lv10) | `tests/sd24_multiclass_fighter_wizard_split.rs` | `artifacts/epic_5/fighter_wizard_split_cycle_receipt.md` | in scope (Criterion 5.1, 5.2) |
| APG classes | n/a | `artifacts/epic_5/apg-acg-multiclass-deferred.md` | out of scope (deferred) |
| ACG classes | n/a | `artifacts/epic_5/apg-acg-multiclass-deferred.md` | out of scope (deferred) |

## 6. Cross-reference

- `../SD-22/corpus-source-inventory.md` — predecessor inventory pattern
- `./epic-breakdown.md` — per-cycle stories
- `./scope-draft.md §4` — files in this folder
- `./decisions.md §4` — multiclass scope (Fighter+Wizard only)
- `./decisions.md §5` — equipment corpus strict 100% field coverage
- `../../governance/spec-domain-lifecycle.md` — bundle vs. cycle routing
