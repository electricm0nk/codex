//! SD-22 Epic 3 acceptance test — APG shared equipment table (criterion
//! 9: "Per-cycle tests for APG spell and equipment resolution assert
//! representative samples resolve to the expected table cell").
//!
//! Source grounding (per `decisions.md §5`, corrected 2026-07-19): a
//! bootstrap/representative sample of real, verbatim `COST:`/`WT:`
//! records from `apg_equip_general.lst`, `apg_equip_arms_armor.lst`, and
//! `apg_equip_magic_items.lst` — one item per file, mirroring
//! `rules_tables::crb::equipment_tables`'s own "one representative item
//! per category" bootstrap philosophy. Not exhaustive coverage.
//!
//! Note: Alchemist bombs are a `Su` class feature computed by formula
//! (bomb count/damage scale with class level), not a purchasable
//! equipment-table row — no `Bomb` record exists in any
//! `apg_equip_*.lst` file. `rules_tables::apg::equipment_tables`'s doc
//! comment records this so a future reader doesn't go looking for one.

use std::fs;
use std::path::PathBuf;

use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::apg::equipment_tables::{EquipmentCategory, equipment_resolve};

#[test]
fn iron_spike_resolves_via_ruleset_apg() {
    let entry = equipment_resolve("Iron Spike", RuleSetId::Apg)
        .expect("Iron Spike should resolve via RuleSetId::Apg");
    assert_eq!(entry.category, EquipmentCategory::General);
    assert_eq!(entry.cost_gp, Some(0.05), "apg_equip_general.lst COST:0.05");
}

#[test]
fn blunt_arrow_resolves_via_ruleset_apg() {
    let entry = equipment_resolve("Arrow (Blunt)", RuleSetId::Apg)
        .expect("Arrow (Blunt) should resolve via RuleSetId::Apg");
    assert_eq!(entry.category, EquipmentCategory::ArmsArmor);
    assert_eq!(entry.cost_gp, Some(0.1), "apg_equip_arms_armor.lst COST:0.1");
}

#[test]
fn knucklebone_of_fickle_fortune_resolves_via_ruleset_apg() {
    let entry = equipment_resolve("Knucklebone of Fickle Fortune", RuleSetId::Apg)
        .expect("Knucklebone of Fickle Fortune should resolve via RuleSetId::Apg");
    assert_eq!(entry.category, EquipmentCategory::MagicItems);
    assert_eq!(entry.cost_gp, Some(0.0), "apg_equip_magic_items.lst COST:0 (artifact, priceless)");
}

#[test]
fn unknown_equipment_key_resolves_to_none() {
    assert_eq!(
        equipment_resolve("Not A Real APG Item", RuleSetId::Apg),
        None,
        "a key absent from the bootstrap sample must not resolve"
    );
}

/// Cross-book resolution invariant (`corpus-source-inventory.md` §1.3):
/// APG equipment resolves via `RuleSetId::Apg` but must return `None`
/// for `RuleSetId::Crb`.
#[test]
fn iron_spike_returns_none_for_ruleset_crb() {
    assert_eq!(
        equipment_resolve("Iron Spike", RuleSetId::Crb),
        None,
        "APG-only equipment must not resolve under RuleSetId::Crb"
    );
}

// Real-corpus-gated grounding test, opt-in via `PCGEN_CORPUS_ROOT` per the
// established pattern in tests/sd22_apg_class_alchemist_resolves.rs.
#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data"]
fn hand_transcribed_equipment_sample_matches_the_real_lst_lines() {
    let corpus_root = std::env::var("PCGEN_CORPUS_ROOT")
        .expect("set PCGEN_CORPUS_ROOT to the pcgen data/ directory to run this grounding test");
    let base = PathBuf::from(corpus_root).join("pathfinder/paizo/roleplaying_game/advanced_players_guide");

    let expectations: &[(&str, &str, &str)] = &[
        ("apg_equip_general.lst", "Iron Spike", "COST:0.05"),
        ("apg_equip_arms_armor.lst", "Arrow (Blunt)", "COST:0.1"),
        ("apg_equip_magic_items.lst", "Knucklebone of Fickle Fortune", "COST:0"),
    ];

    for (file, name, expected_cost_token) in expectations {
        let path = base.join(file);
        let contents = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
        let line = contents
            .lines()
            .find(|line| line.starts_with(name))
            .unwrap_or_else(|| panic!("expected a real line starting with {name} in {file}"));
        assert!(
            line.contains(expected_cost_token),
            "{name}'s real corpus line in {file} should contain {expected_cost_token}, got: {line}"
        );
    }
}
