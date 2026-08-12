//! SD-22 Epic 5 acceptance test — Bestiary 1 monster-block subset 04
//! (criteria 14-17 re-verified against a fourth subset: `mod.rs`
//! registration holds for the new monsters, per-monster resolution
//! works, and the cross-book resolution invariant holds).
//!
//! **CR 2 continuation, alphabetically after subset 03's "Cave Fisher"**
//! (not a roster correction — `corpus-source-inventory.md` §3.1 had no
//! illustrative sample row for subset 4 to correct, only a placeholder
//! `...` row): before writing any GREEN code, this cycle enumerated
//! every real, non-`#`-commented, non-`.MOD`/`.COPY=` CR:2 monster
//! stat-block row in `b1_races.lst` directly, excluding parenthetical
//! sub-variant names (e.g. "Ant (Giant)", "Cat (Cheetah)", "Demon
//! (Dretch)") the same way subsets 01-03 all already established. There
//! are 34 real `CR:2` rows total in the file; excluding parentheticals
//! leaves 19 clean species names, of which subset 03 used the first
//! five alphabetically (Bat Swarm, Boar, Boggard, Bugbear, Cave Fisher).
//! This cycle lands the next five alphabetically: **Choker** (line 70),
//! **Crocodile** (line 83), **Dark Creeper** (line 89), **Iron Cobra**
//! (line 249), **Morlock** (line 297). See
//! `docs/release/SD-22/artifacts/beastiary1/subset_04_cycle_receipt.md`
//! for the full grounding and the new `corpus-source-inventory.md` §3.1
//! row this cycle adds for subset 4.

use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::beastiary1::{MonsterId, monster_key_resolve, monster_resolve};

#[test]
fn choker_resolves_via_ruleset_bestiary1() {
    let monster = monster_resolve(MonsterId::Choker, RuleSetId::Bestiary1)
        .expect("Choker should resolve via RuleSetId::Bestiary1");
    assert_eq!(monster.name, "Choker");
    assert_eq!(monster.challenge_rating, 2.0);
    assert_eq!(monster.speed_ft, 20, "walk speed, not the climb speed");
    assert_eq!(monster.race_type, "Aberration");
    assert_eq!(monster.race_subtype, None);
    assert_eq!(monster.source_page, "p.45");
    // The Tentacle is named by an `ABILITY:Internal` cross-reference
    // rather than a `NATURALATTACKS:` token, so its dice are grounded
    // from published values rather than transcribed.
    assert!(monster.natural_attacks.iter().any(|a| a.name == "Tentacle" && a.damage_dice == "1d4"));
}

#[test]
fn all_five_subset_04_monsters_resolve_via_ruleset_bestiary1() {
    for (id, expected_name, expected_cr) in [
        (MonsterId::Choker, "Choker", 2.0),
        (MonsterId::Crocodile, "Crocodile", 2.0),
        (MonsterId::DarkCreeper, "Dark Creeper", 2.0),
        (MonsterId::IronCobra, "Iron Cobra", 2.0),
        (MonsterId::Morlock, "Morlock", 2.0),
    ] {
        let monster = monster_resolve(id, RuleSetId::Bestiary1)
            .unwrap_or_else(|| panic!("{expected_name} should resolve via RuleSetId::Bestiary1"));
        assert_eq!(monster.name, expected_name);
        assert_eq!(monster.challenge_rating, expected_cr);
    }
}

/// Subsets 01, 02, and 03's fifteen monsters must still resolve
/// unchanged — landing subset 04 must not regress any prior subset
/// (sibling-preservation).
#[test]
fn subset_01_02_03_monsters_still_resolve_unchanged() {
    for (id, expected_name) in [
        (MonsterId::Ghoul, "Ghoul"),
        (MonsterId::Gnoll, "Gnoll"),
        (MonsterId::GoblinDog, "Goblin Dog"),
        (MonsterId::Lizardfolk, "Lizardfolk"),
        (MonsterId::Wolf, "Wolf"),
        (MonsterId::Darkmantle, "Darkmantle"),
        (MonsterId::Horse, "Horse"),
        (MonsterId::Hyena, "Hyena"),
        (MonsterId::Octopus, "Octopus"),
        (MonsterId::SpiderSwarm, "Spider Swarm"),
        (MonsterId::BatSwarm, "Bat Swarm"),
        (MonsterId::Boar, "Boar"),
        (MonsterId::Boggard, "Boggard"),
        (MonsterId::Bugbear, "Bugbear"),
        (MonsterId::CaveFisher, "Cave Fisher"),
    ] {
        let monster = monster_resolve(id, RuleSetId::Bestiary1)
            .unwrap_or_else(|| panic!("{expected_name} should still resolve via RuleSetId::Bestiary1"));
        assert_eq!(monster.name, expected_name);
    }
}

/// Cross-book resolution invariant (`corpus-source-inventory.md` §3.2,
/// adapted to subset 04's roster): a Bestiary 1 monster resolves via
/// `RuleSetId::Bestiary1` but must return `None` for `RuleSetId::Apg`,
/// `RuleSetId::Acg`, and `RuleSetId::Crb` — monsters aren't spell-list
/// items.
#[test]
fn dark_creeper_returns_none_for_ruleset_apg_acg_crb() {
    assert_eq!(monster_resolve(MonsterId::DarkCreeper, RuleSetId::Apg), None);
    assert_eq!(monster_resolve(MonsterId::DarkCreeper, RuleSetId::Acg), None);
    assert_eq!(monster_resolve(MonsterId::DarkCreeper, RuleSetId::Crb), None);
}

/// Choker, Crocodile, and Dark Creeper all carry no `NATURALATTACKS:`
/// token on their real row — but only **Dark Creeper** is genuinely
/// attack-less. This test originally asserted all three were empty,
/// conflating "the row has no `NATURALATTACKS:` token" with "the monster
/// has no attack". Choker and Crocodile both really fight with natural
/// weapons; their rows merely name them by cross-reference, so their
/// dice are grounded (`natural_attack_provenance`). Iron Cobra and
/// Morlock DO carry real `NATURALATTACKS:` tokens (Morlock's is
/// pipe-separated, two-attack).
#[test]
fn choker_and_crocodile_have_grounded_attacks_while_dark_creeper_is_a_genuine_weapon_user() {
    let choker = monster_resolve(MonsterId::Choker, RuleSetId::Bestiary1)
        .expect("Choker should resolve via RuleSetId::Bestiary1");
    assert!(choker.natural_attacks.iter().any(|a| a.name == "Tentacle" && a.damage_dice == "1d4"));

    let crocodile = monster_resolve(MonsterId::Crocodile, RuleSetId::Bestiary1)
        .expect("Crocodile should resolve via RuleSetId::Bestiary1");
    assert!(crocodile.natural_attacks.iter().any(|a| a.name == "Bite" && a.damage_dice == "1d8"));
    // The Tail Slap is the one attack in this pass recovered from a real
    // cross-file corpus token (`b1_abilities_race.lst:248`), not the web.
    assert!(crocodile.natural_attacks.iter().any(|a| a.name == "Tail Slap" && a.damage_dice == "1d12"));
    assert_eq!(crocodile.size, "L");

    let dark_creeper = monster_resolve(MonsterId::DarkCreeper, RuleSetId::Bestiary1)
        .expect("Dark Creeper should resolve via RuleSetId::Bestiary1");
    assert!(dark_creeper.natural_attacks.is_empty(), "Dark Creeper is a weapon user; its empty list is correct, not a gap");
    assert_eq!(dark_creeper.race_subtype.as_deref(), Some("Dark Folk"));

    let iron_cobra = monster_resolve(MonsterId::IronCobra, RuleSetId::Bestiary1)
        .expect("Iron Cobra should resolve via RuleSetId::Bestiary1");
    assert!(iron_cobra.natural_attacks.iter().any(|a| a.name == "Bite" && a.damage_dice == "1d6"));
    assert_eq!(iron_cobra.race_type, "Construct");

    let morlock = monster_resolve(MonsterId::Morlock, RuleSetId::Bestiary1)
        .expect("Morlock should resolve via RuleSetId::Bestiary1");
    assert_eq!(morlock.natural_attacks.len(), 2, "Morlock's row carries a pipe-separated two-attack token");
    assert!(morlock.natural_attacks.iter().any(|a| a.name == "Bite (Primary)"));
    assert!(morlock.natural_attacks.iter().any(|a| a.name == "Bite (With Weapon Attack)"));
}

/// Key-based resolution, mirroring `corpus-source-inventory.md` §3.2's
/// `beastiary1:monster:<name>` key shape.
#[test]
fn morlock_resolves_by_key_via_ruleset_bestiary1_only() {
    let monster = monster_key_resolve("beastiary1:monster:morlock", RuleSetId::Bestiary1)
        .expect("beastiary1:monster:morlock should resolve via RuleSetId::Bestiary1");
    assert_eq!(monster.name, "Morlock");
    assert_eq!(monster.race_type, "Monstrous Humanoid");

    assert_eq!(monster_key_resolve("beastiary1:monster:morlock", RuleSetId::Apg), None);
    assert_eq!(monster_key_resolve("beastiary1:monster:unknown", RuleSetId::Bestiary1), None);
}
