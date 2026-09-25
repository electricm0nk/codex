//! SD-36 Epic F3c4b: a sorcerer's bloodline pick (the CONVERTED `pool_option` rule the
//! `Standard Bloodline` choice offers) reaches its bloodline's lines through the held set.
//!
//! F3c4 (`f3c4-bloodline-sweep.md`) measured 0 of 287 bloodline lines reachable: each line is
//! gated on a variable only the pick row raised, and the pick row was not converted.
//!
//! Hand-worked from the book first (CRB p.75, Draconic bloodline): class skill Perception;
//! bloodline arcana and bonus spells from 1st level; powers Claws (1st), Dragon Resistances (3rd),
//! Breath Weapon (9th), Wings (15th), Power of Wyrms (20th). The oracle states the same levels as
//! `Sorcerer_Draconic_BloodlineProgressionLVL >= 1 / 3 / 9 / 15 / 20` on each power record
//! (`cr_abilities_class.lst:2450-2460`), and the progression variable is `SorcererLVL` through
//! `Bloodline Tracker` (`:1707`).

use codex::rules_core::sheet_rule::{held_set, CharacterFacts, HeldSeed};
use codex::rules_core::sheet_rule_package;

const CHOOSER: &str = "core_rulebook:class_feature:sorcerer_standard_bloodline_selection";
const DRACONIC_PICK: &str = "core_rulebook:pool_option:sorcerer_bloodline_draconic_bloodline";

fn held_lines(level: i64, pick: Option<&str>) -> Vec<String> {
    let package = sheet_rule_package::package().as_ref().expect("the converted package loads");
    let mut facts = CharacterFacts { level, race: Some("human".into()), class_levels: vec![("sorcerer".into(), level)], ..CharacterFacts::default() };
    if let Some(p) = pick {
        facts.choices.insert(CHOOSER.into(), vec![(p.into(), p.into())]);
    }
    let seed = HeldSeed { race: Some("human".into()), classes: vec![("sorcerer".into(), level)], ..HeldSeed::default() };
    let held = held_set(package, &seed, &facts);
    let mut out: Vec<String> = held.rules.keys().filter(|k| !held.removed.contains(*k)).cloned().collect();
    out.sort();
    out
}

fn has(lines: &[String], id: &str) -> bool {
    lines.iter().any(|l| l == id)
}

#[test]
fn a_draconic_pick_reaches_its_lines_at_the_levels_the_book_states() {
    let base = "core_rulebook:class_feature:";
    let record = format!("{base}sorcerer_bloodline_draconic");
    let always = ["core_rulebook:ability:perception", "draconic_bloodline_bloodline_arcana", "draconic_bloodline_bonus_spells", "draconic_bloodline_bloodline_powers"];
    let powers = [(3, "draconic_bloodline_dragon_resistances"), (9, "draconic_bloodline_breath_weapon"), (15, "draconic_bloodline_wings"), (20, "draconic_bloodline_power_of_wyrms")];
    let id = |s: &str| if s.contains(':') { s.to_string() } else { format!("{base}{s}") };
    // No pick: the record and its lines stay off.
    let none = held_lines(20, None);
    assert!(!has(&none, &record), "no bloodline is held without a pick");
    for level in [1i64, 3, 9, 15, 20] {
        let lines = held_lines(level, Some(DRACONIC_PICK));
        assert!(has(&lines, DRACONIC_PICK) && has(&lines, &record), "level {level}: the pick holds the record");
        assert!(has(&lines, "core_rulebook:class_feature:bloodline_tracker"), "level {level}: Standard Bloodline holds Bloodline Tracker");
        for s in always {
            assert!(has(&lines, &id(s)), "level {level}: {s} is held");
        }
        for (at, s) in powers {
            assert_eq!(has(&lines, &id(s)), level >= at, "level {level}: {s} is held from level {at} (CRB p.75)");
        }
        // Claws (1st level, CRB p.75) converts to ONE line -- its claw-size bonus, gated
        // `Sorcerer_Draconic_BloodlinePower1LVL >= 7` (`cr_abilities_class.lst:2444`) -- and a
        // single-line record keeps that line as its principal (SD-36 Epic E CONV-02), so the
        // record is held from sorcerer 7, not 1. A named converter remainder
        // (`f3c4b-receipt.md` §5), asserted here only where the package and the book agree.
        if level >= 7 {
            assert!(has(&lines, &id("draconic_bloodline_claws")), "level {level}: claws is held");
        }
    }
}

#[test]
fn a_race_gated_pick_is_not_held_by_another_race() {
    // Kobold bloodline (ARG, `arg_abilities_class.lst:631`, PRERACE kobold): a human's pick does
    // not hold it.
    let lines = held_lines(20, Some("advanced_race_guide:pool_option:sorcerer_bloodline_kobold_bloodline"));
    assert!(!has(&lines, "advanced_race_guide:class_feature:sorcerer_bloodline_kobold"), "{lines:?}");
}
