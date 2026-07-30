//! v0.6 alpha swarm: carrying-capacity / encumbrance catalogue adoption.
//!
//! `src/rules_core/encumbrance.rs` (commit `d475097`) grounds the alpha
//! bar's "carry capacity" and "encumbrance" calculations, previously a
//! complete production gap (QA's original wave-1 gap-list survey found
//! every prior "encumbrance"/"carrying capacity" hit in `src/rules_core/`
//! was a doc-comment disclaimer or corpus flavor text, never computation).
//! The module carries its own inline `#[cfg(test)] mod tests` (backend's
//! stopgap since `tests/**` is QA's owned surface for this swarm). This
//! file is QA's independent catalogue adoption: real CRB corpus records
//! (transcribed from this repo's own `data/corpus/core_rulebook/equipment/`
//! shape), independently authored assertions, and PCGen cross-check values
//! pulled from the real PCGen checkout during QA's original formula-spec
//! pass (`/home/ubuntu/workspace/repos/pcgen/system/gameModes/Pathfinder/load.lst`),
//! not from `encumbrance.rs`'s own inline test module.

use codex::rules_core::character_input::{ActiveState, EquipmentSelection};
use codex::rules_core::encumbrance::{
    compute_encumbrance, carrying_capacity_thresholds, CarryingCapacityThresholds, EncumbranceLevel,
};
use codex::rules_core::rules_tables::crb::race_tables::{race_size_for_race_id, RaceId};
use codex::rules_core::size::SizeCategory;
use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

/// Independently transcribed real CRB records (Chain Shirt, Longsword,
/// Backpack) -- different items than `encumbrance.rs`'s own inline
/// fixture (Leather Armor, Buckler, Longsword), so this catalogue entry
/// does not merely re-run the implementer's own sample set.
const FIXTURE_TEXT: &str = "\
Chain Shirt\tKEY:Chain Shirt (Base)\tTYPE:Armor.Light\tCOST:100\tWT:25\tACCHECK:-2\tMAXDEX:4\tSPELLFAILURE:20\tBONUS:COMBAT|AC|4|TYPE=Armor|PREVAREQ:DisableArmorBonus,0
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8
Backpack\tKEY:Backpack\tTYPE:General\tCOST:2\tWT:2
";

fn corpus_from(text: &str) -> SourcePackageContent<'static> {
    let result = parse_equipment_entries("cr_equip_arms_armor.lst", text);
    assert!(result.diagnostics.is_empty(), "fixture text must parse cleanly: {:?}", result.diagnostics);
    let source_ref = SourceRef { lst_file: "cr_equip_arms_armor.lst".to_string(), line: 1 };
    let mut corpus = SourcePackageContent::empty("core_rulebook", source_ref);
    for record in result.entries {
        let record: &'static EquipmentRecord = Box::leak(Box::new(record));
        corpus.push(convert_equipment_record(record));
    }
    corpus
}

fn selection(item_id: &str, state: ActiveState) -> EquipmentSelection {
    EquipmentSelection {
        item_id: item_id.to_owned(),
        equipped_or_active: state == ActiveState::EquippedActive,
        active_state: state,
        applied_modifiers: Vec::new(),
    }
}

/// Cross-checks `carrying_capacity_thresholds` against the real PCGen
/// engine's own `load.lst` data (not the Archives of Nethys citation
/// `encumbrance.rs` itself was transcribed from) -- an independent second
/// source for the same table, pulled during QA's original formula-spec
/// pass. `load.lst`'s `LOAD:<Strength>|<value>` entries are the "Heavy"
/// tier (1x multiplier per `load.lst`'s own `ENCUMBRANCE:Heavy|1`); Light
/// is 1/3 and Medium is 2/3 of that same value, per `load.lst`'s
/// `ENCUMBRANCE:Light|1/3` and `ENCUMBRANCE:Medium|2/3` entries.
#[test]
fn carrying_capacity_thresholds_match_the_real_pcgen_load_lst_table() {
    // LOAD:6|60 in load.lst.
    assert_eq!(
        carrying_capacity_thresholds(6, SizeCategory::Medium),
        CarryingCapacityThresholds { light_max_lbs: 20.0, medium_max_lbs: 40.0, heavy_max_lbs: 60.0 }
    );
    // LOAD:16|230 in load.lst.
    assert_eq!(
        carrying_capacity_thresholds(16, SizeCategory::Medium),
        CarryingCapacityThresholds { light_max_lbs: 76.0, medium_max_lbs: 153.0, heavy_max_lbs: 230.0 }
    );
    // LOAD:25|800 in load.lst.
    assert_eq!(
        carrying_capacity_thresholds(25, SizeCategory::Medium),
        CarryingCapacityThresholds { light_max_lbs: 266.0, medium_max_lbs: 533.0, heavy_max_lbs: 800.0 }
    );
}

#[test]
fn carrying_capacity_thresholds_clamps_a_below_minimum_strength_score() {
    // PF1 does not define carrying capacity below Strength 1; encumbrance.rs
    // defensively floors to the Strength-1 row rather than panicking or
    // extrapolating downward. Str 0 (e.g. a not-yet-fully-loaded fixture, or
    // a hypothetical drained-to-zero score) must not crash or produce a
    // negative/nonsensical capacity.
    assert_eq!(carrying_capacity_thresholds(0, SizeCategory::Medium), carrying_capacity_thresholds(1, SizeCategory::Medium));
}

#[test]
fn carrying_capacity_thresholds_extrapolate_two_tiers_beyond_strength_29() {
    // Str 40 = Str 20's row (same ones digit) multiplied by 4^2 = 16, per
    // load.lst's LOADMULT:4 applied twice (two full +10 steps above the
    // Str-29 table ceiling). encumbrance.rs's own inline test only checks
    // one tier (Str 30); this checks the recursive step holds at two tiers.
    let base = carrying_capacity_thresholds(20, SizeCategory::Medium);
    let extrapolated = carrying_capacity_thresholds(40, SizeCategory::Medium);
    assert_eq!(
        extrapolated,
        CarryingCapacityThresholds {
            light_max_lbs: base.light_max_lbs * 16.0,
            medium_max_lbs: base.medium_max_lbs * 16.0,
            heavy_max_lbs: base.heavy_max_lbs * 16.0,
        }
    );
}

/// The full `LOAD:<Strength>|<value>` column of the real PCGen Pathfinder
/// game mode's `load.lst`, transcribed verbatim (Strength 1-29):
/// `/home/ubuntu/workspace/repos/pcgen/system/gameModes/Pathfinder/load.lst`
/// lines 10-38. This is the *heavy* tier (`ENCUMBRANCE:Heavy|1`, i.e. a 1x
/// multiplier on the load score); light is `1/3` and medium is `2/3` of the
/// same value per that file's `ENCUMBRANCE:Light|1/3` / `Medium|2/3`.
const PCGEN_LOAD_LST_HEAVY_BY_STRENGTH: [i64; 29] = [
    10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 115, 130, 150, 175, 200, 230, 260, 300, 350, 400, 460,
    520, 600, 700, 800, 920, 1040, 1200, 1400,
];

/// Exhaustive cross-check of *every* Strength row against the real PCGen
/// `load.lst` table, rather than the three-row spot check
/// (`carrying_capacity_thresholds_match_the_real_pcgen_load_lst_table`,
/// Str 6/16/25) that preceded it.
///
/// This matters: the spot check sampled only rows that happened to be
/// correct, so a real off-by-one in the Str 15 medium threshold (134,
/// where `load.lst` derives 133) sat undetected in
/// `encumbrance.rs`'s hand-transcribed table. A row-complete assertion
/// closes the whole class of transcription error, not just the one
/// instance.
///
/// The light/medium values are *derived* from `load.lst`'s own heavy
/// column and multipliers rather than separately transcribed, so this test
/// depends on exactly one hand-copied column instead of three. PF1 load
/// tiers truncate toward zero (integer pounds), matching PCGen's own
/// `BigDecimal` load-score arithmetic.
#[test]
fn carrying_capacity_thresholds_match_every_row_of_the_real_pcgen_load_lst_table() {
    for (index, heavy) in PCGEN_LOAD_LST_HEAVY_BY_STRENGTH.iter().enumerate() {
        let strength_score = (index + 1) as i16;
        let expected = CarryingCapacityThresholds {
            light_max_lbs: (heavy / 3) as f64,
            medium_max_lbs: (heavy * 2 / 3) as f64,
            heavy_max_lbs: *heavy as f64,
        };
        assert_eq!(
            carrying_capacity_thresholds(strength_score, SizeCategory::Medium),
            expected,
            "Strength {strength_score} must match load.lst's LOAD:{strength_score}|{heavy} row \
             (light = 1/3, medium = 2/3 of the heavy tier)"
        );
    }
}

/// Every `SIZEMULT:` row of the real PCGen Pathfinder game mode's
/// `load.lst`, transcribed verbatim
/// (`/home/ubuntu/workspace/repos/pcgen/system/gameModes/Pathfinder/load.lst`
/// lines 1-8), as an exact `(numerator, denominator)` rational rather than
/// a float:
///
/// ```text
/// SIZEMULT:F|0.125   SIZEMULT:D|0.25   SIZEMULT:T|0.5   SIZEMULT:S|0.75
/// SIZEMULT:L|2       SIZEMULT:H|4      SIZEMULT:G|8     SIZEMULT:C|16
/// ```
///
/// Medium has no `SIZEMULT:` row at all: it is the unmultiplied base the
/// `LOAD:` column itself is expressed in (`LOAD:10|100` is exactly PF1's
/// published Medium Strength-10 heavy maximum), so its ratio is 1/1. That
/// absence is the *reason* Medium is 1x, not an assumption about it.
const PCGEN_LOAD_LST_SIZEMULT: &[(SizeCategory, i64, i64)] = &[
    (SizeCategory::Fine, 1, 8),
    (SizeCategory::Diminutive, 1, 4),
    (SizeCategory::Tiny, 1, 2),
    (SizeCategory::Small, 3, 4),
    (SizeCategory::Medium, 1, 1),
    (SizeCategory::Large, 2, 1),
    (SizeCategory::Huge, 4, 1),
    (SizeCategory::Gargantuan, 8, 1),
    (SizeCategory::Colossal, 16, 1),
];

/// Row-complete cross-check of the size multipliers, mirroring the
/// discipline `carrying_capacity_thresholds_match_every_row_of_the_real_pcgen_load_lst_table`
/// established for the `LOAD:` column: assert *every* `SIZEMULT:` row
/// against `load.lst`, not a spot check. The Str-15 medium-threshold error
/// (134 for 133) survived a three-row spot check precisely because the
/// sampled rows happened to be right.
#[test]
fn size_load_capacity_ratios_match_every_sizemult_row_of_the_real_pcgen_load_lst() {
    for (size, numerator, denominator) in PCGEN_LOAD_LST_SIZEMULT {
        assert_eq!(
            size.load_capacity_ratio(),
            (*numerator, *denominator),
            "{size:?} must match load.lst's own SIZEMULT row"
        );
    }
}

/// The seven playable races' sizes, read from the *race* corpus rather
/// than from memory. The authoritative token is `FACT:BaseSize|<code>` on
/// each race's own `<race>_races.lst` record in the PCGen checkout the
/// ingested corpus is built from:
/// `data/pathfinder/paizo/roleplaying_game/core_essentials/races/<race>/<race>_races.lst:6`.
///
/// Note the token is `FACT:BaseSize|M`, **not** `SIZE:MEDIUM` -- the
/// pre-existing `race_tables.rs` `Size` trait rows cite a
/// `cr_races.lst race:human SIZE:MEDIUM` that does not exist in that form
/// (`cr_races.lst` carries only `.MOD` source-page citations; the real base
/// race records live in the `core_essentials/races/` PCC pack, the same
/// place `explain_elf_race_seam` already documents finding Elf's true
/// ability-score row). The *values* those prose rows assert are correct;
/// only their citation was wrong, so this test re-derives sizes from the
/// real token instead of trusting the shipped prose.
#[test]
fn every_playable_race_size_matches_its_corpus_base_size_token() {
    // FACT:BaseSize|M on human_races.lst:6, dwarf_races.lst:6,
    // elf_races.lst:6, halfelf_races.lst:6, halforc_races.lst:6.
    // FACT:BaseSize|S on gnome_races.lst:6, halfling_races.lst:6.
    let expected = [
        ("race:human", RaceId::Human, SizeCategory::Medium),
        ("race:dwarf", RaceId::Dwarf, SizeCategory::Medium),
        ("race:elf", RaceId::Elf, SizeCategory::Medium),
        ("race:gnome", RaceId::Gnome, SizeCategory::Small),
        ("race:half-elf", RaceId::HalfElf, SizeCategory::Medium),
        ("race:half-orc", RaceId::HalfOrc, SizeCategory::Medium),
        ("race:halfling", RaceId::Halfling, SizeCategory::Small),
    ];
    for (race_token, _race_id, size) in expected {
        assert_eq!(
            race_size_for_race_id(race_token),
            Some(size),
            "{race_token} must carry its real corpus FACT:BaseSize size"
        );
    }
    // Every curated playable race must resolve; a silent `None` here would
    // send a real character down the unknown-race fallback path.
    assert_eq!(expected.len(), RaceId::ALL.len(), "every curated race must be covered");
}

/// PF1's published Small carrying-capacity column, transcribed for the
/// Strength scores a real 1st-level Gnome or Halfling actually reaches.
///
/// These are *not* Medium's values scaled in this test -- they are the
/// independently-known Small numbers, so the test would still catch an
/// implementation that applied the multiplier in the wrong place. The
/// order of operations matters and is the whole bug: PCGen computes
/// `loadValue * sizeMult` **first** and truncates once at the end
/// (`LoadFacet.getMaxLoad`: `loadValue.doubleValue() * mult *
/// getLoadMultForSize(id)`; `CharacterDisplay.getLoadToken`:
/// `getMaxLoad(mult).intValue()`). Scaling the already-truncated Medium
/// tier values instead would give Strength 10 a light max of
/// `33 * 0.75 = 24.75 -> 24`, where the real answer is `(100 * 3/4) / 3 =
/// 25`. Strength 11 and 13 are included because their Small values
/// truncate on all three tiers (86.25, 57.5, 28.75 / 112.5, 37.5), which a
/// clean-dividing row like Strength 10 would not catch.
const PF1_SMALL_CAPACITY_BY_STRENGTH: &[(i16, f64, f64, f64)] = &[
    (8, 20.0, 40.0, 60.0),
    (10, 25.0, 50.0, 75.0),
    (11, 28.0, 57.0, 86.0),
    (13, 37.0, 75.0, 112.0),
    (15, 50.0, 100.0, 150.0),
    (18, 75.0, 150.0, 225.0),
];

#[test]
fn small_race_carrying_capacity_matches_pf1s_published_small_column() {
    for (strength_score, light, medium, heavy) in PF1_SMALL_CAPACITY_BY_STRENGTH {
        assert_eq!(
            carrying_capacity_thresholds(*strength_score, SizeCategory::Small),
            CarryingCapacityThresholds {
                light_max_lbs: *light,
                medium_max_lbs: *medium,
                heavy_max_lbs: *heavy,
            },
            "Strength {strength_score} at Small size"
        );
    }
}

/// The bug this task closes, stated as an assertion: a Small character's
/// capacity must be *strictly less* than a Medium character's at the same
/// Strength, and specifically 3/4 of the heavy maximum. Before the fix
/// `carrying_capacity_thresholds` had no size parameter at all and every
/// race got the Medium row, so a Gnome or Halfling was handed 4/3 of its
/// true capacity.
#[test]
fn small_size_carrying_capacity_is_three_quarters_of_medium_not_equal_to_it() {
    for strength_score in 1..=29_i16 {
        let medium = carrying_capacity_thresholds(strength_score, SizeCategory::Medium);
        let small = carrying_capacity_thresholds(strength_score, SizeCategory::Small);

        assert!(
            small.heavy_max_lbs < medium.heavy_max_lbs,
            "Strength {strength_score}: a Small creature must carry strictly less than a Medium one"
        );
        // load.lst SIZEMULT:S|0.75 applied to the heavy tier, truncated to
        // whole pounds exactly once (PCGen's `.intValue()`).
        assert_eq!(
            small.heavy_max_lbs,
            (medium.heavy_max_lbs * 3.0 / 4.0).trunc(),
            "Strength {strength_score}: Small's heavy max is SIZEMULT:S|0.75 of Medium's"
        );
    }
}

/// The end-to-end consequence: the same loadout on the same Strength puts
/// a Halfling in a worse load tier than a Human, which is exactly what the
/// pre-fix engine got wrong. Chain Shirt (25) + Longsword (4) = 29 lbs at
/// Strength 10: within Medium's light max (33), but over Small's (25).
///
/// This also pins the downstream consequences the brief calls out -- the
/// max-Dex cap and armor check penalty that follow from the tier -- so the
/// fix is proven to reach the player-visible numbers, not just the
/// threshold struct.
#[test]
fn a_small_race_is_more_encumbered_than_a_medium_race_carrying_the_identical_loadout() {
    let corpus = corpus_from(FIXTURE_TEXT);
    let equipment_selections = vec![
        selection("item:chain_shirt", ActiveState::EquippedActive),
        selection("item:longsword", ActiveState::EquippedActive),
    ];

    let human = compute_encumbrance(&equipment_selections, &corpus, 10, SizeCategory::Medium);
    let halfling = compute_encumbrance(&equipment_selections, &corpus, 10, SizeCategory::Small);

    assert_eq!(human.total_carried_weight_lbs, halfling.total_carried_weight_lbs, "same loadout");

    assert_eq!(human.level, EncumbranceLevel::Light);
    assert_eq!(human.load_max_dex_cap, None);
    assert_eq!(human.load_armor_check_penalty, 0);

    assert_eq!(
        halfling.level,
        EncumbranceLevel::Medium,
        "29 lbs is over a Small Strength-10 character's 25 lb light maximum"
    );
    assert_eq!(halfling.load_max_dex_cap, Some(3));
    assert_eq!(halfling.load_armor_check_penalty, -3);
}

/// PF1's load tiers impose their own max-Dex cap and armor check penalty,
/// independent of what any individual worn item imposes. Grounded in the
/// real PCGen engine's own implementation, not reconstructed from memory:
///
///  - max Dex by load: `PlayerCharacter.java:5362-5368`
///    (`case MEDIUM -> 3; case HEAVY -> 1; case OVERLOAD -> 0;` with Light
///    imposing no cap of its own).
///  - armor check penalty by load: `PlayerCharacter.java:5331`
///    (`(load == Load.MEDIUM) ? -3 : (load == Load.HEAVY) ? -6 : 0`),
///    matching `load.lst`'s own third `ENCUMBRANCE:` field
///    (`Light|1/3||0`, `Medium|2/3||-3`, `Heavy|1||-6`).
#[test]
fn encumbrance_level_carries_the_real_pcgen_load_penalties() {
    assert_eq!(EncumbranceLevel::Light.max_dex_cap(), None, "a light load imposes no max-Dex cap");
    assert_eq!(EncumbranceLevel::Medium.max_dex_cap(), Some(3));
    assert_eq!(EncumbranceLevel::Heavy.max_dex_cap(), Some(1));
    assert_eq!(EncumbranceLevel::OverHeavyCapacity.max_dex_cap(), Some(0));

    assert_eq!(EncumbranceLevel::Light.armor_check_penalty(), 0);
    assert_eq!(EncumbranceLevel::Medium.armor_check_penalty(), -3);
    assert_eq!(EncumbranceLevel::Heavy.armor_check_penalty(), -6);
    assert_eq!(
        EncumbranceLevel::OverHeavyCapacity.armor_check_penalty(),
        -6,
        "carrying beyond the heavy maximum is at least as penalising as a heavy load"
    );
}

/// A real Strength-6 medium-load build must surface the load's own
/// penalties on the computation itself, not merely name the tier -- the
/// player-visible consequence of being encumbered.
#[test]
fn compute_encumbrance_reports_the_load_penalties_for_a_real_medium_load() {
    let corpus = corpus_from(FIXTURE_TEXT);
    // Strength 6: light 20, medium 40, heavy 60 (load.lst LOAD:6|60).
    // Chain Shirt (25) + Longsword (4) = 29 lbs -> Medium.
    let equipment_selections = vec![
        selection("item:chain_shirt", ActiveState::EquippedActive),
        selection("item:longsword", ActiveState::EquippedActive),
    ];

    let computation = compute_encumbrance(&equipment_selections, &corpus, 6, SizeCategory::Medium);

    assert_eq!(computation.level, EncumbranceLevel::Medium);
    assert_eq!(computation.load_max_dex_cap, Some(3));
    assert_eq!(computation.load_armor_check_penalty, -3);
}

/// Carried *cost* rides along with carried weight: both come from the same
/// `equipment_tables()` entry the weight lookup already resolves, so a
/// loadout's total gp value needs no second resolution pass. Values are the
/// real `COST:` tokens on the fixture's own corpus records (Chain Shirt 100,
/// Longsword 15, Backpack 2 -- verbatim CRB).
#[test]
fn compute_encumbrance_totals_the_real_corpus_cost_of_the_carried_loadout() {
    let corpus = corpus_from(FIXTURE_TEXT);
    let equipment_selections = vec![
        selection("item:chain_shirt", ActiveState::EquippedActive),
        selection("item:longsword", ActiveState::EquippedActive),
        selection("item:backpack", ActiveState::SelectedInactive),
    ];

    let computation = compute_encumbrance(&equipment_selections, &corpus, 14, SizeCategory::Medium);

    assert_eq!(computation.total_carried_cost_gp, 100.0 + 15.0 + 2.0);
    let chain_shirt = computation
        .per_item
        .iter()
        .find(|item| item.item_id == "item:chain_shirt")
        .expect("the equipped Chain Shirt must appear in the per-item breakdown");
    assert_eq!(chain_shirt.weight_lbs, 25.0);
    assert_eq!(chain_shirt.cost_gp, Some(100.0), "the real CRB COST:100 token");
}

#[test]
fn compute_encumbrance_sums_a_different_real_item_set_than_the_inline_fixture() {
    let corpus = corpus_from(FIXTURE_TEXT);
    let equipment_selections = vec![
        selection("item:chain_shirt", ActiveState::EquippedActive),
        selection("item:longsword", ActiveState::EquippedActive),
        selection("item:backpack", ActiveState::SelectedInactive),
    ];

    let computation = compute_encumbrance(&equipment_selections, &corpus, 14, SizeCategory::Medium);

    assert_eq!(computation.total_carried_weight_lbs, 25.0 + 4.0 + 2.0);
    assert!(computation.unresolved_item_ids.is_empty(), "{:?}", computation.unresolved_item_ids);
    // Strength 14: light 58, medium 116, heavy 175 (load.lst LOAD:14|175).
    // 31 lbs total is well within the light threshold.
    assert_eq!(computation.level, EncumbranceLevel::Light);
}

#[test]
fn compute_encumbrance_returns_a_true_zero_for_an_entirely_absent_loadout() {
    let corpus = corpus_from(FIXTURE_TEXT);
    let equipment_selections = vec![
        selection("item:chain_shirt", ActiveState::Absent),
        selection("item:longsword", ActiveState::Absent),
    ];

    let computation = compute_encumbrance(&equipment_selections, &corpus, 10, SizeCategory::Medium);

    assert_eq!(computation.total_carried_weight_lbs, 0.0);
    assert!(computation.per_item.is_empty());
    assert!(computation.unresolved_item_ids.is_empty(), "Absent items must not be flagged unresolved");
    assert_eq!(computation.level, EncumbranceLevel::Light, "0 lbs is always within the light threshold");
}

#[test]
fn compute_encumbrance_reaches_medium_between_light_and_heavy_thresholds() {
    let corpus = corpus_from(FIXTURE_TEXT);
    // Strength 6: light 20, medium 40, heavy 60. Chain Shirt (25) + Longsword
    // (4) = 29 lbs, which is above the light max (20) but at/under the
    // medium max (40) -- the middle tier neither of encumbrance.rs's own
    // inline classification tests (Light, OverHeavyCapacity) exercises.
    let equipment_selections = vec![
        selection("item:chain_shirt", ActiveState::EquippedActive),
        selection("item:longsword", ActiveState::EquippedActive),
    ];

    let computation = compute_encumbrance(&equipment_selections, &corpus, 6, SizeCategory::Medium);

    assert_eq!(computation.total_carried_weight_lbs, 29.0);
    assert_eq!(computation.level, EncumbranceLevel::Medium);
}
