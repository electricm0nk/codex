---
title: SD-18 — Technical Design
status: draft (operator review required)
date: 2026-07-12
companion_to: /home/ubuntu/workspace/SD-18-core-rules-breadth-scope-draft.md
---

# SD-18 — Technical Design

Concrete pointers for the loop and the pre-loop slice. Every technical reference names a path, a count, and the public interface the slice or loop will touch.

## Corpus root and PCC include graph

**Entry PCC**: `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc`

**Direct includes** (line scan of `core_rulebook.pcc`):
- `PCC:@/homebrew/conversion_support/conversion_support.pcc` (cross-source support)
- `PCC:@/pathfinder/paizo/roleplaying_game/core_essentials/_core_essentials.pcc` (49 lines of PCC: directives approximately; this is the umbrella for the standard subset)
- 7 core-race sub-PCCs (one per core race — see §3.1 race breakdown)

**Total LST reach at the Core Rulebook subtree:**
- `core_rulebook/` direct: **35 LST files** (named `cr_*.lst`, including `cr_spells.lst`, `cr_classes.lst`, `cr_races.lst`, `cr_equip_*.lst`, etc.)
- `core_essentials/` direct: **40 LST files** (named `ce_*.lst`, plus the structured `races/<race>/` subdirectories)
- `core_essentials/races/<race>/` subdirectories: race-specific LSTs (each race has 6–10 LST files)
- `homebrew/conversion_support/` (from the `conversion_support.pcc` include): token-translation support

**Per-race LST footprint** (count of non-comment content lines across that race's `_abilities_race.lst`, `_races.lst`, `_templates.lst`, etc.):

| Race | LST files | Content lines |
|---|---|---|
| Dwarf | 10 | 131 |
| Gnome | 9 | 131 |
| Elf | 10 | 119 |
| Halfling | 9 | 120 |
| Half-Orc | 9 | 105 |
| Half-Elf | 6 | 103 |
| Human | 7 | 92 |

**Per-corpus-area LST summary:**

| Corpus area | Path | LST files | Notes |
|---|---|---|---|
| Core Rulebook direct | `core_rulebook/cr_*.lst` | 35 | Includes `cr_classes.lst`, `cr_spells.lst`, `cr_races.lst`, `cr_equip_*.lst`, `cr_companionmods.lst`, `cr_deities.lst`, `cr_domains.lst`, `cr_kits.lst`, `cr_languages.lst`, `cr_skills.lst`, `cr_templates.lst`, `cr_feats.lst`, etc. |
| Core Essentials direct | `core_essentials/ce_*.lst` | 40 | Curated subset of CR; pulled in by CR's PCC include |
| 7 core races | `core_essentials/races/{dwarf,elf,gnome,half_elf,half_orc,halfling,human}/` | 60 total (6-10 per race) | Per-race abilities, races, skills, templates, languages, biosettings |
| Supplemental races | `core_essentials/races/{aasimar,android,aquatic_elf,catfolk,changeling,dhampir,drow,duergar,fetchling,...}` | 200+ files | Out of SD-18 scope; reference only |

**`cr_classes.lst` content shape** (CLASS: prefix counts):

The CR `cr_classes.lst` carries **87 `CLASS:` entries** representing 27 distinct `CLASS:<Name>` IDs (including base classes plus templates and special types). The 11 core classes are: Barbarian, Bard, Cleric, Druid, Fighter, Monk, Paladin, Ranger, Rogue, Sorcerer, Wizard. Other CLASS: IDs are templates (Ex-Barbarian, Ex-Paladin, Shadowdancer, Dragon, Mystic, etc.) or non-core base classes. The SD-18 cycle work touches `parse_class_entries` (`/home/ubuntu/workspace/repos/codex/src/pcgen_import/lst_parser/class.rs:425` final module line — file is 425 lines).

**`cr_races.lst` content shape**: uses `<RACE>.MOD` keyed entries (one per race), 7 core race MOD entries at lines 5-12. Each race's full abilities/traits come from the per-race subdirectories under `core_essentials/races/<race>/`.

## Parser surface — what the loop touches

**Module: `src/pcgen_import/`** (in `/home/ubuntu/workspace/repos/codex/src/pcgen_import/`)

Files:

| File | Lines | Purpose |
|---|---|---|
| `pcc.rs` | 118 | PCC entry-file parser (Slice A's deliverable, on disk) |
| `include_resolver.rs` | 405 | `PCC:` include-graph resolver (Slice A's deliverable, on disk) |
| `lst_parser/mod.rs` | 32 | Module facade + public re-exports |
| `lst_parser/class.rs` | 425 | Martial classes (SD-17 B-1) |
| `lst_parser/spellcasting_class.rs` | 815 | Spellcasting classes (SD-17 B-2) |
| `lst_parser/race_ability.rs` | 590 | Races + race-ability objects (SD-17 B-3) |
| `lst_parser/spell.rs` | 488 | Spells (SD-17 B-4) |
| `lst_parser/equipment.rs` | 781 | Equipment + equipment modifiers (SD-17 B-5) |
| `lst_parser/metadata.rs` | 194 | Metadata object kinds (SD-17 B-6) |
| `ir_converter.rs` | 907 | LST-to-canonical-IR converter (Slice C, on disk) |

**Public API of `lst_parser`** (per `mod.rs`):

```rust
pub use class::{ClassEntry, ClassFeatureBlock, ClassLevelLine, ClassParseResult, ClassToken,
                MARTIAL_CLASS_NAMES, parse_class_entries, parse_class_file};
pub use equipment::{BonusToken, EquipmentDiagnostic, EquipmentDiagnosticKind,
                    EquipmentParseResult, EquipmentRecord, EquipmentRecordKind, EquipmentToken,
                    parse_equipment_entries, parse_equipment_file};
pub use metadata::{LstDiagnostic, LstDiagnosticKind, LstMetadataDocument, LstRecord, MetadataKind,
                   parse_lst_metadata, parse_lst_metadata_text};
pub use race_ability::{AbilityDeclaration, AbilityKind, AbilityParsedFields, LstEntryFile,
                       RaceDeclaration, parse_lst_entry};
// spellcasting_class, spell re-exports in their submodule (full surface at
// `pcgen_import::lst_parser::spellcasting_class::*` and `...::spell::*`)
```

The 7 parsers are the loop's reading surface. **Each entry returned references zero-copy into source lines** (per the matured SD-13 parity work and the converter's `ParsedLstRecord` design). The loop reads parsed entries via these public functions; it does NOT need to read raw `.lst` files.

**Corpus-side canonical IR surface (SD17-E planned, not yet on disk):** `src/rules_core/source_content.rs` will house `SourcePackageContent`, `SourceContentRecord`, `SourceRef`, `SourceContentKind`, `SourceContentPayload`, `SourceContentDiagnostic`, `SourceContentLoadResult`. The pre-loop §1.1 slice depends on SD17-E landing on `tranche/2-7` first.

## Rules-core surface — what the slice composes against

**Module: `src/rules_core/`** (in `/home/ubuntu/workspace/repos/codex/src/rules_core/`)

| File | Lines | Public entry points |
|---|---|---|
| `mod.rs` | 7 | `pub mod character_input; pub mod pilot_compute; pub mod pilot_failure; pub mod pilot_view_model; pub mod support_state_matrix;` |
| `character_input.rs` | 408 | `pub fn load_character_input_fixture(input: &str) -> CharacterInputLoadResult` (line ~125) |
| `pilot_compute.rs` | ~12,000 (11699 LOC at last check; per slice work adds seam functions) | `pub fn build_pilot_headless_receipt(input: &CharacterInput) -> PilotHeadlessReceipt` (line 2168); `pub fn compute_pilot_base_chassis(input: &CharacterInput) -> PilotBaseChassisComputation` (line 2186) |
| `pilot_failure.rs` | 67 | diagnostic types |
| `pilot_view_model.rs` | 95 | view-model types |
| `support_state_matrix.rs` | 3688 | `pub fn seeded_sd13_e1_f1_current_truth() -> SupportStateMatrix` (line 727) |

**Character input shape** (`CharacterInput` and `ChosenCharacterState`, from `character_input.rs`):

```rust
pub struct CharacterInput {
    pub case_id: Option<String>,
    pub source_package_id: String,
    pub chosen: ChosenCharacterState,
    pub selection_provenance: Vec<SelectionProvenance>,
}

pub struct ChosenCharacterState {
    pub race_id: String,
    pub class_levels: Vec<CharacterClassLevel>,   // <class_id, level>
    pub ability_scores: AbilityScores,             // STR/DEX/CON/INT/WIS/CHA all i16
    pub selected_feats: Vec<String>,
    pub skill_allocations: Vec<SkillAllocation>,   // <skill_id, ranks>
    pub equipment_selections: Vec<EquipmentSelection>,
    pub selected_choices: Vec<SelectedChoice>,
}
```

**Already-implemented race seam functions in `pilot_compute.rs`** (these are the §3.1 race cycle work's expansion targets):

| Race seam function | File:line | Body size (LOC) |
|---|---|---|
| `explain_human_pilot_race_seam` | `pilot_compute.rs:2403` | ~110 |
| `explain_dwarf_race_seam` | `pilot_compute.rs:2529` | ~125 (sample shown earlier: ability mods, size, speed, senses recognized) |
| `explain_elf_race_seam` | `pilot_compute.rs:2654` | ~115 |
| `explain_gnome_race_seam` | `pilot_compute.rs:2771` | ~110 |
| `explain_half_elf_race_seam` | `pilot_compute.rs:2884` | ~120 |
| `explain_half_orc_race_seam` | `pilot_compute.rs:3005` | ~120 |
| `explain_halfling_race_seam` | `pilot_compute.rs:3126` | ~110 |

The race cycles extend these seams (per the SD-13 pattern — each cycle grounds one additional named family like Stonecunning, Hardy, Stability, etc.). They do NOT introduce new race functions for the 7 core races; the seam functions already exist.

**Already-implemented class seam functions** (the §3.2 class cycle work's expansion targets):

| Class seam function | File:line |
|---|---|
| `explain_barbarian_level1_chassis` | `pilot_compute.rs:6509` |
| `explain_monk_level1_chassis` | `pilot_compute.rs:7079` |
| `explain_rogue_level1_chassis` | `pilot_compute.rs:7988` |
| `explain_paladin_level1_chassis_and_spell_burden_separation` | `pilot_compute.rs:4222` |
| `explain_ranger_level1_chassis_and_class_feature_separation` | `pilot_compute.rs:4987` |
| `explain_sorcerer_level1_spell_baseline` | `pilot_compute.rs:8487` |
| `explain_wizard_level1_prepared_spell_baseline` | `pilot_compute.rs:9247` |
| `explain_cleric_level1_spell_baseline` | `pilot_compute.rs:9656` |
| `explain_druid_level1_spell_baseline` | `pilot_compute.rs:10135` |
| `explain_bard_level1_spell_baseline` | `pilot_compute.rs` (called from line 2293) |

Fighter is integrated via the base chassis path (no standalone `explain_fighter_*` function — it shares the generic `supported_fighter_level` at line 3361). The class cycles add higher-level functions (level 2-10 progression, level 11-20 progression, additional pillar functions).

## §3.1 Race cycles — concrete shape

For each race (7 cycles, one per race, three races per cycle's worth of work in pattern):

**In-cycle work:**
1. Read the per-race LST files from `core_essentials/races/<race>/` (using the parser surface above).
2. Identify the next ungrounded `<Race>.<Family>` dimension from the SD-13 matrix row's `blocker_or_lossiness_note` for `race.<race>.bounded_semantics`.
3. Add a new seam function or extend the existing race seam (e.g., `explain_dwarf_race_seam`) to ground one named family as a real computed contribution.
4. Add a fixture file `tests/fixtures/rules_core/pf1_<race>_fighter_level<1-or-10-or-20>_sd18_<family>.txt`.
5. Add a test file `tests/sd18_<race>_<family>.rs`.
6. Update `support_state_matrix.rs::seeded_sd13_e1_f1_current_truth()` to bump `race.<race>.bounded_semantics` from `Partial/Computed` to either `Partial/Computed` (widened) or eventually `Supported/Computed`.
7. Verify with `cargo test --locked --test sd18_<race>_<family>` (green) and `cargo clippy --locked --tests -- -D warnings` (clean).

**Naming convention:**
- Feature branch: `loop/tranche3-cycle-<cycle-id>-<race>-<family>`
- Test file: `tests/sd18_<race>_<family>.rs`
- Fixture: `tests/fixtures/rules_core/pf1_<race>_fighter_level<N>_sd18_<family>.txt`

## §3.2 Class cycles — concrete shape

For each class (11 cycles, one per class):

**In-cycle work**:
1. Read `cr_classes.lst` (87 CLASS: entries, 27 distinct IDs) and extract the targeted class's record via `parse_class_file` (`src/pcgen_import/lst_parser/class.rs:425`).
2. Identify the next ungrounded class burden from the SD-13 row's `blocker_or_lossiness_note` (e.g., for Wizard: "school-powers / opposed-school-cost burden and prepared spellbook / spell-slot posture burden").
3. Add or extend the class seam function (e.g., `explain_wizard_level1_prepared_spell_baseline` at `pilot_compute.rs:9247`) to ground one level (or one level band) of progress.
4. Fixture: `tests/fixtures/rules_core/pf1_human_<class>_level<N>_sd18_<burden>.txt`.
5. Test file: `tests/sd18_<class>_<burden>.rs`.

**Naming convention:**
- Feature branch: `loop/tranche3-cycle-<cycle-id>-<class>-<level>-<burden>`
- Test file: `tests/sd18_<class>_<burden>.rs`
- Fixture: `tests/fixtures/rules_core/pf1_human_<class>_level<N>_sd18_<burden>.txt`

## §3.3 Interaction cycles

Two cards, both card-routed or loop-routed. The first interaction card is the **Human bonus feat / ability-bonus seam** (Partial/Computed → product-visible). The second is **non-Human race × class progression beyond pilot** (Unverified/Observed → at-least-Partial/Computed for one chosen exemplar).

For each:
- Cycle scope: one tier promotion with explicit grounding artifact.
- Naming: `loop/tranche3-cycle-<cycle-id>-interaction-<interaction-name>`.

## §3.4 Spell school cycles

**Corpus**: `core_rulebook/cr_spells.lst` carries **~652 spell records** (count derived from `SCHOOL:` tags: 652 SCHOOL: line matches across the file).

**Strict-school partition** (PF1's nine schools, with derived counts from the corpus's sub-school keywords):

The LST's `SCHOOL:` tags use sub-school granularity (Transmutation → Polymorph/Calling; Illusion → Figment/Glamer/Shadow/Phantasm/Pattern). The strict-school partition derives from sub-school keywords:

| PF1 strict school | Derived from sub-school keywords |
|---|---|
| Abjuration | direct `Abjuration` + protective school branches |
| Conjuration | `Summoning`, `Creation`, `Calling`, `Teleportation`, `Healing` |
| Divination | `Divination`, `Scrying` |
| Enchantment | `Compulsion`, `Charm` |
| Evocation | direct `Evocation` |
| Illusion | `Figment`, `Glamer`, `Shadow`, `Phantasm`, `Pattern` |
| Necromancy | direct `Necromancy` |
| Transmutation | direct `Transmutation` + `Polymorph` |
| Universal | direct `Universal` |

**Per cycle, per school:**
1. Read `cr_spells.lst` (target ~70-100 spells per school depending on distribution).
2. Use `parse_spells` (planned parser public surface at `src/pcgen_import/lst_parser/spell.rs:488`) to extract the school's spells.
3. Verify each spell's provenance carries through to `SourceContentRecord` (per SD17-E planned shape).
4. Verify spell reachability via `CharacterInput::selected_choices` (a casting class's `class_levels` + chosen caster class).
5. Test file: `tests/sd18_spell_school_<school_name>.rs` (one per school).
6. End-user-visible character: L10 casting class demonstrating the school.

**Naming**: `loop/tranche3-cycle-<cycle-id>-spell-school-<school_name>`.

## §3.5 Equipment category cycles

**Corpus**: 4 core-rulebook equipment LST files.

| File | Path | Content shape |
|---|---|---|
| `cr_equip_arms_armor.lst` | `core_rulebook/cr_equip_arms_armor.lst` | weapons and armor entries (using PCGen object-kind tags: ARMOR, WEAPON, etc., recognized via the SD-17 B-5 parser) |
| `cr_equip_general.lst` | `core_rulebook/cr_equip_general.lst` | general adventuring gear (poisons, mounts, vehicles, etc.) |
| `cr_equip_magic_items.lst` | `core_rulebook/cr_equip_magic_items.lst` | magic items (scrolls ~634, wands ~351, potions ~87, rings ~59, belts ~22, etc.) |
| `cr_equipmods.lst` | `core_rulebook/cr_equipmods.lst` | equipment modifiers (BONUS: chains applied to equipment) |

**Per cycle, per category:**
1. Read the category's LST file via `parse_equipment_file` (`src/pcgen_import/lst_parser/equipment.rs:781`).
2. Verify EQUIP/EQUIPMOD entries survive the consumer-side composition with `ActiveState::EquippedActive`.
3. Verify derived-stat impact (AC, attack bonus, save bonus, damage) reflects the equipped item through the compute path.
4. Test file: `tests/sd18_equipment_category_<category>.rs`.
5. End-user-visible character: a Fighter L5 with armor + magic weapon + wondrous item equipped.

**Naming**: `loop/tranche3-cycle-<cycle-id>-equipment-category-<category>`.

## Per-iteration branch lifecycle (concrete command sequence)

```
# Step 1-2: read state
cat /home/ubuntu/workspace/SD-18-core-rules-breadth-scope-draft.md
ls /home/ubuntu/workspace/programs/codex/requirements/SD-18-core-rules-breadth/  # progress doc lives parallel to scope

# Step 3: fetch and branch
cd /home/ubuntu/workspace/repos/codex
git fetch origin tranche/3
git checkout -b loop/tranche3-cycle-<cycle-id>-<criterion> origin/tranche/3

# Step 4-7: TDD cycle
cargo test --locked --test sd18_<criterion> 2>&1 | tail -40   # RED
# <implement>
cargo test --locked 2>&1 | tail -20                              # GREEN
cargo clippy --locked --tests -- -D warnings 2>&1 | tail -20   # CLEAN

# Step 8: commit (operator identity)
git add <specific files per the lane partition>
git -c user.name='Todd Hintzmann' -c user.email='todd@hintzmann.net' \
  commit -m "feat(sd18): <criterion> (<row transition>)"

# Step 9: push
git push -u origin loop/tranche3-cycle-<cycle-id>-<criterion>

# Step 10: auto-merge to tranche/3
git checkout tranche/3
git pull origin tranche/3
git merge --no-ff loop/tranche3-cycle-<cycle-id>-<criterion> -m "merge: sd18 <criterion>"
git push origin tranche/3

# Step 11: cleanup
git branch -d loop/tranche3-cycle-<cycle-id>-<criterion>
git push origin --delete loop/tranche3-cycle-<cycle-id>-<criterion>

# Step 12: kanban card mint (post-mortem record)
hermes kanban --board codex-tranche-3 create \
  "SD18 <criterion> (loop/tranche3-cycle-<cycle-id>)" \
  --assignee operator \
  --body "<card schema fields per scope doc §4.3>" \
  --initial-status done \
  --json

# Step 13: progress doc update
# Edit /home/ubuntu/workspace/SD-18-core-rules-breadth-progress.md in place.
```

## Kanban card body schema (concrete)

```
epic: SD-18
criterion_section: <scope doc section reference, e.g. "§3.1 Race rows: Dwarf">
row_or_kind: <e.g. "race:dwarf" or "school:abjuration" or "category:arms_armor" or "interaction:human-bonus-feat-seam">
evidence_tier_before: <previous SD-13 row state>
evidence_tier_after: <new SD-13 row state after this merge>
feature_branch: <branch name>
merge_receipt_sha: <merge commit SHA on tranche/3>
cycle_id: <ISO-8601 timestamp>
cargo_test_summary: <test summary string>
clippy_signal: clean | dirty
cycle_timing_seconds: <N>
self_heals_applied: <list, empty if none>
next_required_uplift: <loop's recommendation>
ui_surface: <operator-provided surface name, empty if none>
```

## Cross-reference

- `decisions.md` (11-item decision record)
- `epic-breakdown.md` (34 criteria → execution lanes)
- `risks-and-open-questions.md` (self-healable vs non-self-healable; 4 bundle-level risks)
- `references/sd13-loop-model-excerpt.md` (matured SD-13 inheritance)
- `acceptance-and-verification.md` (closure gates)
- `/home/ubuntu/workspace/SD-18-core-rules-breadth-scope-draft.md` (canonical handoff doc)
