//! SD-22 Epic 4 acceptance test — ACG shared spell list (criterion 13:
//! "Per-cycle tests for ACG spell and equipment resolution assert
//! representative samples resolve to the expected table cell").
//! Mirrors `tests/sd22_apg_spell_list_resolves.rs`'s shape exactly.
//!
//! Source grounding (per `decisions.md §5`, corrected 2026-07-19): the
//! real `acg_spells.lst` "New Spells" block (lines 6-143) carries real,
//! active (non-`.MOD`, non-commented) spell records naming only two of
//! ACG's ten classes as a caster in a `CLASSES:` token — Bloodrager and
//! Shaman. Arcanist, Hunter, Investigator (only a `.MOD` cross-reference
//! onto APG's own `Bomber's Eye`, not a full definition), Skald, and
//! Warpriest have zero real `CLASSES:<name>` hits anywhere in
//! `acg_spells.lst` or the wider `advanced_class_guide/` tree (confirmed
//! by direct grep, not assumed) — those classes draw from other books'
//! spell lists (e.g. Arcanist from the arcane Sorcerer/Wizard list) with
//! no ACG-specific `.lst` record of their own. This is a bootstrap/
//! representative sample, mirroring `rules_tables::crb::equipment_tables`'s
//! own "one representative item per category" precedent — not exhaustive
//! coverage. See `rules_tables::acg::spell_list`'s doc comment for the
//! full citation.

use std::fs;
use std::path::PathBuf;

use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::acg::spell_list::{Pf1SchoolId, spell_resolve};

#[test]
fn blade_lash_resolves_via_ruleset_acg() {
    let entry = spell_resolve("Blade Lash", RuleSetId::Acg)
        .expect("Blade Lash (Bloodrager=1) should resolve via RuleSetId::Acg");
    assert_eq!(entry.school, Pf1SchoolId::Transmutation);
    assert_eq!(entry.level, 1, "acg_spells.lst:27 CLASSES:Bloodrager,Magus=1");
}

#[test]
fn air_geyser_resolves_via_ruleset_acg() {
    let entry = spell_resolve("Air Geyser", RuleSetId::Acg)
        .expect("Air Geyser (Bloodrager=3) should resolve via RuleSetId::Acg");
    assert_eq!(entry.school, Pf1SchoolId::Evocation);
    assert_eq!(
        entry.level, 3,
        "acg_spells.lst:14 CLASSES:Bloodrager,...=3|Shaman=4 (this bootstrap entry cites the Bloodrager=3 level)"
    );
}

#[test]
fn beastspeak_resolves_via_ruleset_acg() {
    let entry = spell_resolve("Beastspeak", RuleSetId::Acg)
        .expect("Beastspeak (Shaman=2) should resolve via RuleSetId::Acg");
    assert_eq!(entry.school, Pf1SchoolId::Divination);
    assert_eq!(entry.level, 2, "acg_spells.lst:25 CLASSES:Druid,Shaman,Witch=2");
}

#[test]
fn anti_incorporeal_shell_resolves_via_ruleset_acg() {
    let entry = spell_resolve("Anti-Incorporeal Shell", RuleSetId::Acg)
        .expect("Anti-Incorporeal Shell (Shaman=4) should resolve via RuleSetId::Acg");
    assert_eq!(entry.school, Pf1SchoolId::Abjuration);
    assert_eq!(entry.level, 4, "acg_spells.lst:21 CLASSES:Cleric,Shaman,Witch=4");
}

#[test]
fn unknown_spell_key_resolves_to_none() {
    assert_eq!(
        spell_resolve("Not A Real ACG Spell", RuleSetId::Acg),
        None,
        "a key absent from the bootstrap sample must not resolve"
    );
}

/// Cross-book resolution invariant (`corpus-source-inventory.md` §2.3):
/// an ACG spell resolves via `RuleSetId::Acg` but must return `None` for
/// `RuleSetId::Crb` or `RuleSetId::Apg` — an ACG-only spell is never a
/// valid answer for a CRB or APG query.
#[test]
fn blade_lash_returns_none_for_ruleset_crb() {
    assert_eq!(
        spell_resolve("Blade Lash", RuleSetId::Crb),
        None,
        "ACG-only spell must not resolve under RuleSetId::Crb"
    );
}

#[test]
fn blade_lash_returns_none_for_ruleset_apg() {
    assert_eq!(
        spell_resolve("Blade Lash", RuleSetId::Apg),
        None,
        "ACG-only spell must not resolve under RuleSetId::Apg"
    );
}

// Real-corpus-gated grounding test, opt-in via `PCGEN_CORPUS_ROOT` per the
// established pattern in tests/sd22_apg_spell_list_resolves.rs.
#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data"]
fn hand_transcribed_spell_sample_matches_the_real_lst_lines() {
    let corpus_root = std::env::var("PCGEN_CORPUS_ROOT")
        .expect("set PCGEN_CORPUS_ROOT to the pcgen data/ directory to run this grounding test");
    let mut path = PathBuf::from(corpus_root);
    path.push("pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_spells.lst");
    let contents = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));

    let expectations: &[(&str, &str)] = &[
        ("Blade Lash", "CLASSES:Bloodrager,Magus=1"),
        ("Air Geyser", "CLASSES:Bloodrager,Druid,Magus,Sorcerer,Witch,Wizard=3"),
        ("Beastspeak", "CLASSES:Druid,Shaman,Witch=2"),
        ("Anti-Incorporeal Shell", "CLASSES:Cleric,Shaman,Witch=4"),
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
