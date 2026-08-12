//! SD-27 `decisions.md §28`, defect 1: **races had no size modifier to Armor
//! Class**, so a Small character's sheet showed a Medium character's
//! arithmetic. Reproduced on screen as a Goblin Fighter 1 reading AC 18 where
//! PF1's Small value at those same stats is 19.
//!
//! # What this file is for
//!
//! §28's standing guard: *"Every change to `pilot_compute.rs` lands with a test
//! pinning the before/after per affected race or class, so drift is a caught
//! failure rather than a silent recomputation."*
//!
//! So this pins **all 18 in-scope races by name**, not just the 5 Small ones.
//! The 13 Medium races are the load-bearing half of the guard: their numbers
//! must be byte-identical to what they were before the size modifier existed,
//! and a regression that quietly re-based every character is precisely what a
//! Small-races-only test would wave through.
//!
//! Every race is driven through the *same* fixture with only `race_id`
//! swapped, so creature size is the single independent variable and any
//! difference between two races here is attributable to size and nothing else.
//!
//! # The pinned "before" values are measured, not remembered
//!
//! Before this cycle, `defense.baseline_armor_class` was
//! `10 + Chain Shirt 4 + DEX 2 + Dodge 1 = 17` for **every one of the 18
//! races**, Small and Medium alike -- that identity is what proved the modifier
//! was missing rather than merely wrong. `BASELINE_ARMOR_CLASS_BEFORE_SIZE`
//! records it, and `every_medium_race_is_byte_identical_to_the_pre_size_baseline`
//! asserts the 13 Medium races still equal it exactly.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::race_resolver::race_size_for_race_token;
use codex::rules_core::size::SizeCategory;

const DETERMINISTIC_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

/// `10 (base) + 4 (Chain Shirt) + 2 (DEX, within MAXDEX 4) + 1 (Dodge)`.
///
/// This was the value produced for *every* race before the size modifier
/// landed, and it stays the correct value for every Medium race after it.
const BASELINE_ARMOR_CLASS_BEFORE_SIZE: i16 = 17;

/// The 18 races SD-27 ingested and made creatable, each with the creature size
/// `race_resolver::RACE_SIZES` resolves it to.
///
/// Deliberately spelled out here rather than imported from the engine: this is
/// the *expectation*, and a test whose expectation is read from the code under
/// test cannot catch that code changing. `the_expected_race_sizes_still_match_the_engines_own_table`
/// then cross-checks the two, so a real corpus correction fails loudly here
/// with the race named instead of silently rewriting what "correct" means.
const RACES: &[(&str, SizeCategory)] = &[
    // Core Rulebook's 7.
    ("Dwarf", SizeCategory::Medium),
    ("Elf", SizeCategory::Medium),
    ("Gnome", SizeCategory::Small),
    ("Half-Elf", SizeCategory::Medium),
    ("Half-Orc", SizeCategory::Medium),
    ("Halfling", SizeCategory::Small),
    ("Human", SizeCategory::Medium),
    // Bestiary 1's 11.
    ("Aasimar", SizeCategory::Medium),
    ("Drow", SizeCategory::Medium),
    ("Duergar", SizeCategory::Medium),
    ("Goblin", SizeCategory::Small),
    ("Hobgoblin", SizeCategory::Medium),
    ("Kobold", SizeCategory::Small),
    ("Merfolk", SizeCategory::Medium),
    ("Orc", SizeCategory::Medium),
    ("Svirfneblin", SizeCategory::Small),
    ("Tengu", SizeCategory::Medium),
    ("Tiefling", SizeCategory::Medium),
];

/// The deterministic Fighter 1 fixture with only `race_id` changed.
fn input_for_race(race: &str) -> CharacterInput {
    let slug = race.to_lowercase();
    let text =
        DETERMINISTIC_FIXTURE.replace("race_id=race:human", &format!("race_id=race:{slug}"));
    assert!(
        text.contains(&format!("race_id=race:{slug}")),
        "test setup should have swapped the race to {race}"
    );
    let loaded = load_character_input_fixture(&text);
    assert!(
        loaded.diagnostics.is_empty(),
        "{race} fixture should load cleanly: {:?}",
        loaded.diagnostics
    );
    loaded
        .character_input
        .expect("valid fixture should produce a character input record")
}

fn armor_class_for_race(race: &str) -> i16 {
    let computation = compute_pilot_base_chassis(&input_for_race(race));
    let explanation = computation
        .explanations
        .iter()
        .find(|e| e.id == "defense.baseline_armor_class")
        .unwrap_or_else(|| panic!("{race} must produce a defense.baseline_armor_class explanation"));
    assert_eq!(
        explanation.value, computation.baseline_armor_class,
        "{race}'s explanation and its computed field must not disagree"
    );
    computation.baseline_armor_class
}

/// Guards the expectation table above against the engine's own authoritative
/// size resolution. If a corpus correction changes a race's size, this fails
/// and names it, rather than the sheet silently changing underneath.
#[test]
fn the_expected_race_sizes_still_match_the_engines_own_table() {
    for (race, expected_size) in RACES {
        assert_eq!(
            race_size_for_race_token(race),
            Some(*expected_size),
            "{race}'s size in race_resolver::RACE_SIZES no longer matches this test's expectation"
        );
    }
    let small: Vec<&str> = RACES
        .iter()
        .filter(|(_, size)| *size == SizeCategory::Small)
        .map(|(race, _)| *race)
        .collect();
    assert_eq!(
        small,
        vec!["Gnome", "Halfling", "Goblin", "Kobold", "Svirfneblin"],
        "exactly these 5 of the 18 in-scope races are Small"
    );
}

/// **The regression this guard exists to catch.** Every Medium race must equal
/// the pre-size-modifier baseline exactly. Adding a size modifier must not move
/// a single Medium character's Armor Class by a single point.
#[test]
fn every_medium_race_is_byte_identical_to_the_pre_size_baseline() {
    for (race, size) in RACES {
        if *size != SizeCategory::Medium {
            continue;
        }
        assert_eq!(
            armor_class_for_race(race),
            BASELINE_ARMOR_CLASS_BEFORE_SIZE,
            "{race} is Medium: its Armor Class must be unchanged from before the size \
             modifier existed (Medium is PF1's +0 baseline)"
        );
    }
}

/// The fix itself: each of the 5 Small races gains exactly the +1 size modifier
/// and nothing else.
#[test]
fn every_small_race_gains_exactly_one_point_of_armor_class() {
    for (race, size) in RACES {
        if *size != SizeCategory::Small {
            continue;
        }
        assert_eq!(
            armor_class_for_race(race),
            BASELINE_ARMOR_CLASS_BEFORE_SIZE + 1,
            "{race} is Small: PF1 Table 8-1 gives it a +1 size modifier to Armor Class"
        );
    }
}

/// The two halves stated as one relationship, so a change that moved *both*
/// groups by the same amount -- which would leave each test above passing on
/// its own if their constants were also edited -- still fails here.
#[test]
fn small_races_sit_exactly_one_point_above_medium_races_and_no_group_varies_internally() {
    let medium: Vec<(&str, i16)> = RACES
        .iter()
        .filter(|(_, size)| *size == SizeCategory::Medium)
        .map(|(race, _)| (*race, armor_class_for_race(race)))
        .collect();
    let small: Vec<(&str, i16)> = RACES
        .iter()
        .filter(|(_, size)| *size == SizeCategory::Small)
        .map(|(race, _)| (*race, armor_class_for_race(race)))
        .collect();

    let medium_value = medium[0].1;
    for (race, value) in &medium {
        assert_eq!(
            *value, medium_value,
            "{race} is Medium and must match every other Medium race; size is the only \
             variable in this fixture, so two Medium races differing is a real defect"
        );
    }
    let small_value = small[0].1;
    for (race, value) in &small {
        assert_eq!(
            *value, small_value,
            "{race} is Small and must match every other Small race"
        );
    }
    assert_eq!(
        small_value - medium_value,
        1,
        "Small must sit exactly +1 above Medium on Armor Class (PF1 Table 8-1)"
    );
}

/// The size modifier must be *explained*, not just summed in. An unexplained
/// point of Armor Class is indistinguishable from a bug to the person reading
/// the sheet, and this repo's explanations are a shipped surface.
#[test]
fn the_armor_class_explanation_cites_the_size_modifier_and_the_resolved_size() {
    let goblin = compute_pilot_base_chassis(&input_for_race("Goblin"));
    let detail = goblin
        .explanations
        .iter()
        .find(|e| e.id == "defense.baseline_armor_class")
        .expect("Goblin must produce a defense.baseline_armor_class explanation")
        .detail
        .clone();
    assert!(
        detail.contains("size modifier"),
        "a Small character's AC detail must name the size modifier: {detail}"
    );
    assert!(
        detail.contains("Small"),
        "a Small character's AC detail must name the resolved creature size: {detail}"
    );
    assert!(
        detail.contains("+1"),
        "a Small character's AC detail must show the +1 magnitude: {detail}"
    );

    // A Medium character's detail still names the term, at +0. Hiding a zero
    // term would make "no size modifier applied" and "size modifier applied and
    // it was zero" indistinguishable -- which is the exact ambiguity that let
    // this defect survive since Gnome and Halfling shipped.
    let human = compute_pilot_base_chassis(&input_for_race("Human"));
    let human_detail = human
        .explanations
        .iter()
        .find(|e| e.id == "defense.baseline_armor_class")
        .expect("Human must produce a defense.baseline_armor_class explanation")
        .detail
        .clone();
    assert!(
        human_detail.contains("size modifier") && human_detail.contains("Medium"),
        "a Medium character's AC detail must still name the size term and the size: {human_detail}"
    );
}

/// The supported deterministic posture must stay unblocked for every race --
/// the size modifier must not introduce a claim-blocking diagnostic on a race
/// whose size the engine knows perfectly well.
#[test]
fn no_in_scope_race_becomes_claim_blocked_by_the_size_modifier() {
    for (race, _) in RACES {
        let computation = compute_pilot_base_chassis(&input_for_race(race));
        let blocking: Vec<&str> = computation
            .diagnostics
            .iter()
            .filter(|d| d.claim_blocking)
            .map(|d| d.id.as_str())
            .collect();
        assert!(
            blocking.is_empty(),
            "{race} must not be claim-blocked: {blocking:?}"
        );
    }
}
