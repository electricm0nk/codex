//! SD-22 Epic 4 acceptance test — ACG shared equipment table (criterion
//! 13: "Per-cycle tests for ACG spell and equipment resolution assert
//! representative samples resolve to the expected table cell"). Mirrors
//! `tests/sd22_apg_equipment_resolves.rs`'s shape exactly.
//!
//! Source grounding (per `decisions.md §5`, corrected 2026-07-19): a
//! bootstrap/representative sample of real, verbatim `COST:` records
//! from the single `acg_equip.lst` file (unlike APG, which splits
//! equipment across three files, ACG carries general goods, weapons,
//! armor, and magic items together in one file) — one item per
//! category, mirroring `rules_tables::crb::equipment_tables`'s own "one
//! representative item per category" bootstrap philosophy. Not
//! exhaustive coverage.

use std::fs;
use std::path::PathBuf;

use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::acg::equipment_tables::{EquipmentCategory, equipment_resolve};

#[test]
fn marlinspike_resolves_via_ruleset_acg() {
    let entry = equipment_resolve("Marlinspike", RuleSetId::Acg)
        .expect("Marlinspike should resolve via RuleSetId::Acg");
    assert_eq!(entry.category, EquipmentCategory::General);
    assert_eq!(entry.cost_gp, Some(0.8), "acg_equip.lst:179 COST:0.8");
}

#[test]
fn headsmans_blade_resolves_via_ruleset_acg() {
    let entry = equipment_resolve("Headsman's Blade", RuleSetId::Acg)
        .expect("Headsman's Blade should resolve via RuleSetId::Acg");
    assert_eq!(entry.category, EquipmentCategory::ArmsArmor);
    assert_eq!(entry.cost_gp, Some(50.0), "acg_equip.lst:262 COST:50");
}

#[test]
fn ring_of_eloquence_resolves_via_ruleset_acg() {
    let entry = equipment_resolve("Ring of Eloquence", RuleSetId::Acg)
        .expect("Ring of Eloquence should resolve via RuleSetId::Acg");
    assert_eq!(entry.category, EquipmentCategory::MagicItems);
    assert_eq!(entry.cost_gp, Some(3500.0), "acg_equip.lst:271 COST:3500");
}

#[test]
fn unknown_equipment_key_resolves_to_none() {
    assert_eq!(
        equipment_resolve("Not A Real ACG Item", RuleSetId::Acg),
        None,
        "a key absent from the bootstrap sample must not resolve"
    );
}

/// Cross-book resolution invariant (`corpus-source-inventory.md` §2.3):
/// ACG equipment resolves via `RuleSetId::Acg` but must return `None`
/// for `RuleSetId::Crb` or `RuleSetId::Apg`.
#[test]
fn marlinspike_returns_none_for_ruleset_crb() {
    assert_eq!(
        equipment_resolve("Marlinspike", RuleSetId::Crb),
        None,
        "ACG-only equipment must not resolve under RuleSetId::Crb"
    );
}

#[test]
fn marlinspike_returns_none_for_ruleset_apg() {
    assert_eq!(
        equipment_resolve("Marlinspike", RuleSetId::Apg),
        None,
        "ACG-only equipment must not resolve under RuleSetId::Apg"
    );
}

// Real-corpus-gated grounding test, opt-in via `PCGEN_CORPUS_ROOT` per the
// established pattern in tests/sd22_apg_equipment_resolves.rs.
#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data"]
fn hand_transcribed_equipment_sample_matches_the_real_lst_lines() {
    let corpus_root = std::env::var("PCGEN_CORPUS_ROOT")
        .expect("set PCGEN_CORPUS_ROOT to the pcgen data/ directory to run this grounding test");
    let path = PathBuf::from(corpus_root)
        .join("pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_equip.lst");
    let contents = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));

    let expectations: &[(&str, &str)] = &[
        ("Marlinspike", "COST:0.8"),
        ("Headsman's Blade", "COST:50"),
        ("Ring of Eloquence", "COST:3500"),
    ];

    for (name, expected_cost_token) in expectations {
        let line = contents
            .lines()
            .find(|line| line.starts_with(name))
            .unwrap_or_else(|| panic!("expected a real line starting with {name} in acg_equip.lst"));
        assert!(
            line.contains(expected_cost_token),
            "{name}'s real corpus line should contain {expected_cost_token}, got: {line}"
        );
    }
}
