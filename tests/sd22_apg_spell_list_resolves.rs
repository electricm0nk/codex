//! SD-22 Epic 3 acceptance test — APG shared spell list (criterion 9:
//! "Per-cycle tests for APG spell and equipment resolution assert
//! representative samples resolve to the expected table cell").
//!
//! Source grounding (per `decisions.md §5`, corrected 2026-07-19): the
//! real `apg_spells.lst` "Main Spell List" block carries real, active
//! (non-`.MOD`, non-commented) spell records for four of APG's five
//! caster classes — Alchemist, Inquisitor, Oracle, Witch (Summoner's own
//! "Summoner Spells - APG" block at `apg_spells.lst:471+` is entirely
//! `#`-commented out in the real corpus; see
//! `rules_tables::apg::spell_list`'s doc comment for the full citation).
//! This is a bootstrap/representative sample, mirroring
//! `rules_tables::crb::equipment_tables`'s own "one representative item
//! per category" precedent — not exhaustive coverage.

use std::fs;
use std::path::PathBuf;

use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::apg::spell_list::{Pf1SchoolId, spell_resolve};

#[test]
fn bombers_eye_resolves_via_ruleset_apg() {
    let entry = spell_resolve("Bomber's Eye", RuleSetId::Apg)
        .expect("Bomber's Eye (Alchemist=1) should resolve via RuleSetId::Apg");
    assert_eq!(entry.school, Some(Pf1SchoolId::Transmutation));
    assert_eq!(entry.level, Some(1), "apg_spells.lst:44 CLASSES:Alchemist=1");
}

#[test]
fn burst_bonds_resolves_via_ruleset_apg() {
    let entry = spell_resolve("Burst Bonds", RuleSetId::Apg)
        .expect("Burst Bonds (Inquisitor=1) should resolve via RuleSetId::Apg");
    assert_eq!(entry.school, Some(Pf1SchoolId::Evocation));
    assert_eq!(entry.level, Some(1), "apg_spells.lst:53 CLASSES:Inquisitor=1");
}

#[test]
fn borrow_fortune_resolves_via_ruleset_apg() {
    let entry = spell_resolve("Borrow Fortune", RuleSetId::Apg)
        .expect("Borrow Fortune (Oracle=3) should resolve via RuleSetId::Apg");
    assert_eq!(entry.school, Some(Pf1SchoolId::Evocation));
    assert_eq!(entry.level, Some(3), "apg_spells.lst:277 CLASSES:Oracle=3");
}

#[test]
fn ill_omen_resolves_via_ruleset_apg() {
    let entry = spell_resolve("Ill Omen", RuleSetId::Apg)
        .expect("Ill Omen (Witch=1) should resolve via RuleSetId::Apg");
    assert_eq!(entry.school, Some(Pf1SchoolId::Enchantment));
    assert_eq!(entry.level, Some(1), "apg_spells.lst:150 CLASSES:Witch=1");
}

#[test]
fn unknown_spell_key_resolves_to_none() {
    assert_eq!(
        spell_resolve("Not A Real APG Spell", RuleSetId::Apg),
        None,
        "a key absent from the bootstrap sample must not resolve"
    );
}

/// Cross-book resolution invariant (`corpus-source-inventory.md` §1.3):
/// an APG spell resolves via `RuleSetId::Apg` but must return `None` for
/// `RuleSetId::Crb` — an APG-only spell is never a valid answer for a
/// CRB query.
#[test]
fn bombers_eye_returns_none_for_ruleset_crb() {
    assert_eq!(
        spell_resolve("Bomber's Eye", RuleSetId::Crb),
        None,
        "APG-only spell must not resolve under RuleSetId::Crb"
    );
}

// Real-corpus-gated grounding test, opt-in via `PCGEN_CORPUS_ROOT` per the
// established pattern in tests/sd22_apg_class_alchemist_resolves.rs.
#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=$HOME/workspace/repos/pcgen/data"]
fn hand_transcribed_spell_sample_matches_the_real_lst_lines() {
    let corpus_root = std::env::var("PCGEN_CORPUS_ROOT")
        .expect("set PCGEN_CORPUS_ROOT to the pcgen data/ directory to run this grounding test");
    let mut path = PathBuf::from(corpus_root);
    path.push("pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_spells.lst");
    let contents = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));

    let expectations: &[(&str, &str)] = &[
        ("Bomber's Eye", "CLASSES:Alchemist=1"),
        ("Burst Bonds", "CLASSES:Inquisitor=1"),
        ("Borrow Fortune", "CLASSES:Oracle=3"),
        ("Ill Omen", "CLASSES:Witch=1"),
    ];

    for (name, expected_classes_token) in expectations {
        let line = contents
            .lines()
            .find(|line| line.starts_with(name) && !line.contains(".MOD"))
            .unwrap_or_else(|| panic!("expected a real, active spell line starting with {name}"));
        assert!(
            line.contains(expected_classes_token),
            "{name}'s real corpus line should contain {expected_classes_token}, got: {line}"
        );
    }
}
