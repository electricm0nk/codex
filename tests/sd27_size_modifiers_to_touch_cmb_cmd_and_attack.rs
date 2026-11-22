//! SD-27 `decisions.md §28`, defect 1 — the four cells the Armor Class fix
//! left behind.
//!
//! `tests/sd27_size_modifiers_to_armor_class.rs` closed **AC**. Measured on
//! screen for the same Goblin Fighter 1 (STR 14, DEX 18, BAB +1, Chain Shirt,
//! Weapon Focus), four more cells were still computing a Medium creature's
//! arithmetic:
//!
//! ```text
//! cell     on screen   PF1 correct
//! AC       19          19   <- already fixed by the sibling file
//! TOUCH    14          15
//! CMB      +3          +2
//! CMD      17          16
//! MELEE    +4          +5
//! ```
//!
//! Touch AC, CMB and CMD were not merely un-sized: they did not exist in the
//! engine at all. `apps/desktop/src/characterHub/CharacterSheet.tsx` computed
//! all three in React (`10 + dexMod`, `bab + str`, `10 + bab + str + dexMod`),
//! which is why the sheet could show `AC 19` and `TOUCH 14` side by side and
//! contradict itself: `19 − 4 (armor) = 15`, not 14.
//!
//! # What this file pins
//!
//! §28's standing guard: *"Every change to `pilot_compute.rs` lands with a test
//! pinning the before/after per affected race or class, so drift is a caught
//! failure rather than a silent recomputation."*
//!
//! So every assertion below runs over **all 18 in-scope races**, driven through
//! the same fixture with only `race_id` swapped, so creature size is the single
//! independent variable. The 13 Medium races are the load-bearing half: they
//! must be byte-identical to the pre-size arithmetic.
//!
//! # Why Human's numbers differ from the other 12 Medium races
//!
//! Not a size effect, and not new. The shared deterministic fixture carries
//! `choice=choice:human_ability_bonus:ability:strength`, and
//! `pilot_compute::apply_human_ability_bonus` applies the PF1 Standard Human
//! +2 to Strength *before* modifiers are derived — for Human only. So Human
//! computes at STR 18 (+4) and the other 17 races at the fixture's chosen STR
//! 16 (+3). Every cell below that reads Strength therefore has two "before"
//! constants, both spelled out, rather than one constant plus a silent
//! exception.

use codex::rules_core::character_input::{load_character_input_fixture, CharacterInput};
use codex::rules_core::pilot_compute::{compute_pilot_base_chassis, PilotBaseChassisComputation};
use codex::rules_core::size::SizeCategory;

const DETERMINISTIC_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

/// The 18 races SD-27 ingested and made creatable, with the creature size
/// `race_resolver::RACE_SIZES` resolves each to. Restated here rather than
/// imported from the engine for the reason the sibling AC file states: a test
/// whose expectation is read out of the code under test cannot catch that code
/// changing. `tests/sd27_size_modifiers_to_armor_class.rs` cross-checks this
/// same table against the engine's own resolution.
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

/// `BAB +1 + Strength +3 + Weapon Focus +1`, with no size term — the value
/// every non-Human race produced before this cycle.
const MELEE_BEFORE_SIZE: i16 = 5;
/// Human's own, one point higher: its +2 racial Strength makes the modifier +4.
const MELEE_BEFORE_SIZE_HUMAN: i16 = 6;

/// `10 (base) + 2 (DEX, within the Chain Shirt's MAXDEX 4) + 1 (Dodge)` — the
/// Armor Class with the armor bonus removed, which is what touch AC *is*.
/// Strength-independent, so Human shares it.
const TOUCH_BEFORE_SIZE: i16 = 13;

/// `BAB +1 + Strength +3`, with no special size term.
const CMB_BEFORE_SIZE: i16 = 4;
const CMB_BEFORE_SIZE_HUMAN: i16 = 5;

/// `10 + BAB +1 + Strength +3 + Dexterity +2`, with no special size term.
const CMD_BEFORE_SIZE: i16 = 16;
const CMD_BEFORE_SIZE_HUMAN: i16 = 17;

/// The deterministic Fighter 1 fixture with only `race_id` changed.
fn input_for_race(race: &str) -> CharacterInput {
    let slug = race.to_lowercase();
    let text = DETERMINISTIC_FIXTURE.replace("race_id=race:human", &format!("race_id=race:{slug}"));
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

fn computation_for_race(race: &str) -> PilotBaseChassisComputation {
    compute_pilot_base_chassis(&input_for_race(race))
}

fn explanation_value(computation: &PilotBaseChassisComputation, id: &str, race: &str) -> i16 {
    computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| panic!("{race} must produce a {id} explanation"))
        .value
}

/// Per-race "before" value for a Strength-reading cell — see this file's header
/// for why Human is its own case.
fn expected_before(race: &str, non_human: i16, human: i16) -> i16 {
    if race == "Human" {
        human
    } else {
        non_human
    }
}

/// **The regression half of the guard.** Every Medium race's four cells must be
/// byte-identical to the pre-size arithmetic. Medium is PF1's +0 baseline on
/// both size columns, so introducing size modifiers must not move a single
/// Medium character's sheet by a single point.
#[test]
fn every_medium_race_is_byte_identical_to_the_pre_size_values_on_all_four_cells() {
    for (race, size) in RACES {
        if *size != SizeCategory::Medium {
            continue;
        }
        let computation = computation_for_race(race);

        assert_eq!(
            computation.baseline_melee_attack_bonus,
            expected_before(race, MELEE_BEFORE_SIZE, MELEE_BEFORE_SIZE_HUMAN),
            "{race} is Medium: its melee attack bonus must be unchanged"
        );
        assert_eq!(
            explanation_value(&computation, "defense.touch_armor_class", race),
            TOUCH_BEFORE_SIZE,
            "{race} is Medium: its touch AC must be the Medium arithmetic"
        );
        assert_eq!(
            explanation_value(&computation, "combat.combat_maneuver_bonus", race),
            expected_before(race, CMB_BEFORE_SIZE, CMB_BEFORE_SIZE_HUMAN),
            "{race} is Medium: its CMB must be the Medium arithmetic"
        );
        assert_eq!(
            explanation_value(&computation, "defense.combat_maneuver_defense", race),
            expected_before(race, CMD_BEFORE_SIZE, CMD_BEFORE_SIZE_HUMAN),
            "{race} is Medium: its CMD must be the Medium arithmetic"
        );
    }
}

/// The fix itself, in the exact PF1 directions and magnitudes: Small gains +1
/// on the Armor-Class column (touch AC, and attack rolls, which take the
/// identical modifier) and loses 1 on the special size column (CMB and CMD).
#[test]
fn every_small_race_moves_by_exactly_the_published_size_modifiers() {
    for (race, size) in RACES {
        if *size != SizeCategory::Small {
            continue;
        }
        let computation = computation_for_race(race);

        assert_eq!(
            computation.baseline_melee_attack_bonus,
            MELEE_BEFORE_SIZE + 1,
            "{race} is Small: attack rolls take the same +1 size modifier as AC"
        );
        assert_eq!(
            explanation_value(&computation, "defense.touch_armor_class", race),
            TOUCH_BEFORE_SIZE + 1,
            "{race} is Small: touch AC takes the +1 Table 8-1 size modifier"
        );
        assert_eq!(
            explanation_value(&computation, "combat.combat_maneuver_bonus", race),
            CMB_BEFORE_SIZE - 1,
            "{race} is Small: CMB takes the -1 special size modifier"
        );
        assert_eq!(
            explanation_value(&computation, "defense.combat_maneuver_defense", race),
            CMD_BEFORE_SIZE - 1,
            "{race} is Small: CMD takes the -1 special size modifier"
        );
    }
}

/// The two size columns run in **opposite** directions, and that is the whole
/// point of the "special" size modifier. A copy-paste that gave CMB/CMD the
/// Armor Class column's sign would leave both tests above passing if their
/// constants were edited together; it cannot survive this one.
#[test]
fn small_sits_above_medium_on_defense_and_below_it_on_maneuvers() {
    // Compared against a non-Human Medium race so Strength is identical on
    // both sides and size is genuinely the only difference.
    let medium = computation_for_race("Dwarf");
    let small = computation_for_race("Goblin");

    assert_eq!(
        small.baseline_melee_attack_bonus - medium.baseline_melee_attack_bonus,
        1,
        "Small must sit exactly +1 above Medium on attack rolls"
    );
    assert_eq!(
        explanation_value(&small, "defense.touch_armor_class", "Goblin")
            - explanation_value(&medium, "defense.touch_armor_class", "Dwarf"),
        1,
        "Small must sit exactly +1 above Medium on touch AC"
    );
    assert_eq!(
        explanation_value(&small, "combat.combat_maneuver_bonus", "Goblin")
            - explanation_value(&medium, "combat.combat_maneuver_bonus", "Dwarf"),
        -1,
        "Small must sit exactly 1 BELOW Medium on CMB"
    );
    assert_eq!(
        explanation_value(&small, "defense.combat_maneuver_defense", "Goblin")
            - explanation_value(&medium, "defense.combat_maneuver_defense", "Dwarf"),
        -1,
        "Small must sit exactly 1 BELOW Medium on CMD"
    );
}

/// The internal contradiction the sheet was showing, stated as an invariant:
/// touch AC *is* the Armor Class with the armor bonus removed, so the two must
/// never disagree. `AC 19` next to `TOUCH 14` (armor bonus 4) is exactly what
/// this catches.
#[test]
fn touch_armor_class_is_the_armor_class_with_the_armor_bonus_removed_for_every_race() {
    const CHAIN_SHIRT_ARMOR_BONUS: i16 = 4;
    for (race, _) in RACES {
        let computation = computation_for_race(race);
        assert_eq!(
            explanation_value(&computation, "defense.touch_armor_class", race),
            computation.baseline_armor_class - CHAIN_SHIRT_ARMOR_BONUS,
            "{race}: touch AC and Armor Class must not contradict each other -- touch AC is the \
             same Armor Class with the armor bonus removed, not an independently computed number"
        );
    }
}

/// Every new cell must be *explained*, not just summed. An unexplained point on
/// a player's sheet is indistinguishable from a bug, and this repo's
/// explanations are a shipped surface the desktop sheet reads verbatim.
#[test]
fn each_new_cell_cites_the_size_modifier_and_the_resolved_size() {
    let goblin = computation_for_race("Goblin");
    for id in [
        "defense.touch_armor_class",
        "combat.combat_maneuver_bonus",
        "defense.combat_maneuver_defense",
        "combat.baseline_melee_attack_bonus",
    ] {
        let detail = goblin
            .explanations
            .iter()
            .find(|e| e.id == id)
            .unwrap_or_else(|| panic!("Goblin must produce a {id} explanation"))
            .detail
            .clone();
        assert!(
            detail.contains("size modifier"),
            "{id} must name the size modifier: {detail}"
        );
        assert!(
            detail.contains("Small"),
            "{id} must name the resolved creature size: {detail}"
        );
    }

    // A Medium character's details still name the term at +0. Hiding a zero
    // term makes "no size modifier applied" and "applied, and it was zero"
    // indistinguishable -- the exact ambiguity that let this defect survive
    // since Gnome and Halfling shipped.
    let human = computation_for_race("Human");
    for id in [
        "defense.touch_armor_class",
        "combat.combat_maneuver_bonus",
        "defense.combat_maneuver_defense",
        "combat.baseline_melee_attack_bonus",
    ] {
        let detail = human
            .explanations
            .iter()
            .find(|e| e.id == id)
            .unwrap_or_else(|| panic!("Human must produce a {id} explanation"))
            .detail
            .clone();
        assert!(
            detail.contains("size modifier") && detail.contains("Medium"),
            "{id} must still name the size term and the size for a Medium character: {detail}"
        );
    }
}

/// No in-scope race may become claim-blocked by these three new cells: the
/// engine knows every one of these 18 races' size perfectly well.
#[test]
fn no_in_scope_race_becomes_claim_blocked_by_the_new_cells() {
    for (race, _) in RACES {
        let computation = computation_for_race(race);
        let blocking: Vec<&str> = computation
            .diagnostics
            .iter()
            .filter(|d| d.claim_blocking)
            .map(|d| d.id.as_str())
            .collect();
        assert!(blocking.is_empty(), "{race} must not be claim-blocked: {blocking:?}");
    }
}
