//! SD-27 — the `decisions.md §28` standing-guard pin for the Pathfinder
//! Unchained class wiring (2026-07-31).
//!
//! §28 lifted §8's ban on editing `src/rules_core/pilot_compute.rs` and
//! attached one condition to it, verbatim: *"Every change to it lands with
//! a test pinning the before/after per affected race or class, so drift is
//! a caught failure rather than a silent recomputation."* This file is that
//! test for the class-wiring change.
//!
//! # What it pins, and how the numbers were obtained
//!
//! Every row of [`CHASSIS_PIN`] is a literal, not a value derived from the
//! same tables the engine reads — a pin that recomputes its own expectation
//! pins nothing. The 81 rows for the 27 pre-existing classes were captured
//! by running the real compute pipeline in a **clean `git worktree` at the
//! branch's HEAD commit (`9220a929`)**, before any of this change existed,
//! and were then compared byte-for-byte against the same sweep in the
//! working tree afterwards. They were identical, which is the "the 27
//! existing classes compute byte-identically" claim, made checkable. The 12
//! rows for the four Unchained classes were captured the same way, after.
//!
//! Deliberately narrow: this pins the class chassis (base attack bonus,
//! the three base saves) and max hit points, which is the exact blast
//! radius of adding a class. It does NOT pin Armor Class, CMB or CMD —
//! those are a different in-flight change's lane (§28's other open defect,
//! size modifiers), and a pin that fails for somebody else's correct work
//! is noise, not a guard.

use codex::rules_core::character_input::{
    CharacterClassLevel, CharacterInput, load_character_input_fixture,
};
use codex::rules_core::durability::compute_max_hp;
use codex::rules_core::pilot_compute::build_pilot_headless_receipt;

/// The same shared deterministic fixture `src/bin/v06_class_state_dump.rs`
/// sweeps, so this pin and the `class-dump` verify stage describe the same
/// posture rather than two different ones.
const FIXTURE: &str =
    "tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt";

/// `(class name, level, base attack bonus, Fortitude, Reflex, Will, max HP)`.
///
/// Max HP is computed at a Constitution modifier of 0, so the number is the
/// class's hit die contribution alone and a hit-die regression cannot hide
/// behind an ability modifier.
const CHASSIS_PIN: &[(&str, u8, i16, i16, i16, i16, i16)] = &[
    ("barbarian", 1, 1, 2, 0, 0, 12),
    ("barbarian", 10, 10, 7, 3, 3, 75),
    ("barbarian", 20, 20, 12, 6, 6, 145),
    ("bard", 1, 0, 0, 2, 2, 8),
    ("bard", 10, 7, 3, 7, 7, 53),
    ("bard", 20, 15, 6, 12, 12, 103),
    ("cleric", 1, 0, 2, 0, 2, 8),
    ("cleric", 10, 7, 7, 3, 7, 53),
    ("cleric", 20, 15, 12, 6, 12, 103),
    ("druid", 1, 0, 2, 0, 2, 8),
    ("druid", 10, 7, 7, 3, 7, 53),
    ("druid", 20, 15, 12, 6, 12, 103),
    ("fighter", 1, 1, 2, 0, 0, 10),
    ("fighter", 10, 10, 7, 3, 3, 64),
    ("fighter", 20, 20, 12, 6, 6, 124),
    ("monk", 1, 0, 2, 2, 2, 8),
    ("monk", 10, 7, 7, 7, 7, 53),
    ("monk", 20, 15, 12, 12, 12, 103),
    ("paladin", 1, 1, 2, 0, 2, 10),
    ("paladin", 10, 10, 7, 3, 7, 64),
    ("paladin", 20, 20, 12, 6, 12, 124),
    ("ranger", 1, 1, 2, 2, 0, 10),
    ("ranger", 10, 10, 7, 7, 3, 64),
    ("ranger", 20, 20, 12, 12, 6, 124),
    ("rogue", 1, 0, 0, 2, 0, 8),
    ("rogue", 10, 7, 3, 7, 3, 53),
    ("rogue", 20, 15, 6, 12, 6, 103),
    ("sorcerer", 1, 0, 0, 0, 2, 6),
    ("sorcerer", 10, 5, 3, 3, 7, 42),
    ("sorcerer", 20, 10, 6, 6, 12, 82),
    ("wizard", 1, 0, 0, 0, 2, 6),
    ("wizard", 10, 5, 3, 3, 7, 42),
    ("wizard", 20, 10, 6, 6, 12, 82),
    ("alchemist", 1, 0, 2, 2, 0, 8),
    ("alchemist", 10, 7, 7, 7, 3, 53),
    ("alchemist", 20, 15, 12, 12, 6, 103),
    ("cavalier", 1, 1, 2, 0, 0, 10),
    ("cavalier", 10, 10, 7, 3, 3, 64),
    ("cavalier", 20, 20, 12, 6, 6, 124),
    ("inquisitor", 1, 0, 2, 0, 2, 8),
    ("inquisitor", 10, 7, 7, 3, 7, 53),
    ("inquisitor", 20, 15, 12, 6, 12, 103),
    ("oracle", 1, 0, 0, 0, 2, 8),
    ("oracle", 10, 7, 3, 3, 7, 53),
    ("oracle", 20, 15, 6, 6, 12, 103),
    ("summoner", 1, 0, 0, 0, 2, 8),
    ("summoner", 10, 7, 3, 3, 7, 53),
    ("summoner", 20, 15, 6, 6, 12, 103),
    ("witch", 1, 0, 0, 0, 2, 6),
    ("witch", 10, 5, 3, 3, 7, 42),
    ("witch", 20, 10, 6, 6, 12, 82),
    ("arcanist", 1, 0, 0, 0, 2, 6),
    ("arcanist", 10, 5, 3, 3, 7, 42),
    ("arcanist", 20, 10, 6, 6, 12, 82),
    ("bloodrager", 1, 1, 2, 0, 0, 10),
    ("bloodrager", 10, 10, 7, 3, 3, 64),
    ("bloodrager", 20, 20, 12, 6, 6, 124),
    ("brawler", 1, 1, 2, 2, 0, 10),
    ("brawler", 10, 10, 7, 7, 3, 64),
    ("brawler", 20, 20, 12, 12, 6, 124),
    ("hunter", 1, 0, 2, 2, 0, 8),
    ("hunter", 10, 7, 7, 7, 3, 53),
    ("hunter", 20, 15, 12, 12, 6, 103),
    ("investigator", 1, 0, 0, 2, 2, 8),
    ("investigator", 10, 7, 3, 7, 7, 53),
    ("investigator", 20, 15, 6, 12, 12, 103),
    ("shaman", 1, 0, 0, 0, 2, 8),
    ("shaman", 10, 7, 3, 3, 7, 53),
    ("shaman", 20, 15, 6, 6, 12, 103),
    ("skald", 1, 0, 2, 0, 2, 8),
    ("skald", 10, 7, 7, 3, 7, 53),
    ("skald", 20, 15, 12, 6, 12, 103),
    ("slayer", 1, 1, 2, 2, 0, 10),
    ("slayer", 10, 10, 7, 7, 3, 64),
    ("slayer", 20, 20, 12, 12, 6, 124),
    ("swashbuckler", 1, 1, 0, 2, 0, 10),
    ("swashbuckler", 10, 10, 3, 7, 3, 64),
    ("swashbuckler", 20, 20, 6, 12, 6, 124),
    ("warpriest", 1, 0, 2, 0, 2, 8),
    ("warpriest", 10, 7, 7, 3, 7, 53),
    ("warpriest", 20, 15, 12, 6, 12, 103),
    ("unchained_barbarian", 1, 1, 2, 0, 0, 12),
    ("unchained_barbarian", 10, 10, 7, 3, 3, 75),
    ("unchained_barbarian", 20, 20, 12, 6, 6, 145),
    ("unchained_monk", 1, 1, 2, 2, 0, 10),
    ("unchained_monk", 10, 10, 7, 7, 3, 64),
    ("unchained_monk", 20, 20, 12, 12, 6, 124),
    ("unchained_rogue", 1, 0, 0, 2, 0, 8),
    ("unchained_rogue", 10, 7, 3, 7, 3, 53),
    ("unchained_rogue", 20, 15, 6, 12, 6, 103),
    ("unchained_summoner", 1, 0, 0, 0, 2, 8),
    ("unchained_summoner", 10, 7, 3, 3, 7, 53),
    ("unchained_summoner", 20, 15, 6, 6, 12, 103),
];

fn fixture() -> CharacterInput {
    let text = std::fs::read_to_string(FIXTURE).expect("shared deterministic fixture is readable");
    load_character_input_fixture(&text)
        .character_input
        .expect("shared deterministic fixture loads")
}

fn explanation_value(receipt_ids: &[(String, i16)], id: &str) -> i16 {
    receipt_ids
        .iter()
        .find(|(candidate, _)| candidate == id)
        .map(|(_, value)| *value)
        .unwrap_or_else(|| panic!("receipt carries no `{id}` explanation"))
}

/// The pin itself. A failure here means either a real rules correction (in
/// which case update the literal *and say why in the commit*) or a silent
/// recomputation, which is the thing §28 is guarding against.
#[test]
fn every_class_chassis_and_hit_point_total_matches_its_pinned_value() {
    let fixture = fixture();
    for (name, level, bab, fort, reflex, will, max_hp) in CHASSIS_PIN {
        let mut input = fixture.clone();
        input.case_id = Some(format!("sd27_pu_class_wiring_pin.{name}.level{level}"));
        input.chosen.class_levels = vec![CharacterClassLevel {
            class_id: format!("class:{name}"),
            level: *level,
        }];

        let receipt = build_pilot_headless_receipt(&input);
        let ids: Vec<(String, i16)> = receipt
            .computation
            .explanations
            .iter()
            .map(|e| (e.id.clone(), e.value))
            .collect();

        assert_eq!(
            explanation_value(&ids, "class_chassis.base_attack_bonus"),
            *bab,
            "{name} level {level} base attack bonus"
        );
        assert_eq!(
            explanation_value(&ids, "class_chassis.base_save.fortitude"),
            *fort,
            "{name} level {level} base Fortitude save"
        );
        assert_eq!(
            explanation_value(&ids, "class_chassis.base_save.reflex"),
            *reflex,
            "{name} level {level} base Reflex save"
        );
        assert_eq!(
            explanation_value(&ids, "class_chassis.base_save.will"),
            *will,
            "{name} level {level} base Will save"
        );
        assert_eq!(
            compute_max_hp(&input.chosen.class_levels, 0),
            Some(*max_hp),
            "{name} level {level} max hit points at Constitution modifier 0"
        );
    }
}

/// The pin must actually cover both halves of every replacement pair, or a
/// collision between an Unchained class and its namesake could slip past it.
#[test]
fn the_pin_covers_all_four_replacement_pairs_on_both_sides() {
    for (unchained, replaced) in [
        ("unchained_barbarian", "barbarian"),
        ("unchained_monk", "monk"),
        ("unchained_rogue", "rogue"),
        ("unchained_summoner", "summoner"),
    ] {
        for name in [unchained, replaced] {
            assert_eq!(
                CHASSIS_PIN.iter().filter(|row| row.0 == name).count(),
                3,
                "{name} must be pinned at levels 1, 10 and 20"
            );
        }
    }
    assert_eq!(CHASSIS_PIN.len(), 93, "27 pre-existing + 4 new classes, 3 levels each");
}

/// The whole point of the pair being a REPLACEMENT rather than an alias:
/// the Unchained Monk's chassis and hit die genuinely differ from the Core
/// Rulebook Monk's, so a future edit that quietly routes one to the other
/// cannot pass. The other three pairs deliberately DO match, because their
/// corpus records override no chassis field — asserted here too, so
/// "identical" is a stated expectation rather than an unexamined
/// coincidence.
#[test]
fn the_unchained_monk_diverges_from_the_crb_monk_while_the_other_three_pairs_match() {
    let row = |name: &str, level: u8| {
        *CHASSIS_PIN
            .iter()
            .find(|row| row.0 == name && row.1 == level)
            .unwrap_or_else(|| panic!("{name} level {level} is pinned"))
    };

    for level in [1u8, 10, 20] {
        let crb = row("monk", level);
        let pu = row("unchained_monk", level);
        assert!(pu.2 > crb.2, "Unchained Monk has FULL base attack bonus at level {level}");
        assert!(pu.5 < crb.5, "Unchained Monk has a POOR Will save at level {level}");
        assert_eq!(pu.3, crb.3, "both are good Fortitude at level {level}");
        assert_eq!(pu.4, crb.4, "both are good Reflex at level {level}");
        assert!(pu.6 > crb.6, "Unchained Monk is d10 against the CRB Monk's d8 at level {level}");

        for (unchained, replaced) in [
            ("unchained_barbarian", "barbarian"),
            ("unchained_rogue", "rogue"),
            ("unchained_summoner", "summoner"),
        ] {
            assert_eq!(
                row(unchained, level),
                (unchained, level, row(replaced, level).2, row(replaced, level).3,
                 row(replaced, level).4, row(replaced, level).5, row(replaced, level).6),
                "{unchained} borrows {replaced}'s chassis unchanged at level {level}"
            );
        }
    }
}
