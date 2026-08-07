//! SD28-E16 acceptance test — Bestiary 1 monster-block subset 09
//! (mirrors SD-22's `sd22_beastiary1_subset_08_resolves.rs` shape:
//! `mod.rs` registration holds for the new monsters, per-monster
//! resolution works, and the cross-book resolution invariant holds).
//!
//! **Subset 09** (documented in full in
//! `src/rules_core/rules_tables/beastiary1/monster_subset_09.rs`'s own
//! module doc comment): continues CR 3 alphabetically after subset 08's
//! "Hell Hound". Before writing any GREEN code, this cycle independently
//! re-enumerated every real, non-`#`-commented, non-`.MOD` `CR:3` row in
//! `b1_races.lst` directly against the live corpus file: 20 clean,
//! standalone (non-parenthetical) CR:3 species names exist total, same
//! count subsets 07-08 found. Subsets 07-08 shipped the first ten
//! alphabetically; this subset ships the next five: **Lion** (line 272),
//! **Ogre** (line 316), **Pegasus** (line 323), **Rust Monster** (line
//! 341), **Shadow** (line 357).

use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::beastiary1::{MonsterId, NaturalAttack, monster_key_resolve, monster_resolve};

#[test]
fn lion_resolves_via_ruleset_bestiary1() {
    let monster =
        monster_resolve(MonsterId::Lion, RuleSetId::Bestiary1).expect("Lion should resolve via RuleSetId::Bestiary1");
    assert_eq!(monster.name, "Lion");
    assert_eq!(monster.challenge_rating, 3.0);
    assert_eq!(monster.size, "L");
    assert_eq!(monster.speed_ft, 40);
    assert_eq!(monster.race_type, "Animal");
    assert_eq!(monster.race_subtype, None);
    assert_eq!(monster.source_page, "p.193");
    assert_eq!(
        monster.natural_attacks,
        vec![
            NaturalAttack { name: "Bite".to_string(), damage_dice: "1d8".to_string() },
            NaturalAttack { name: "Claw".to_string(), damage_dice: "1d4".to_string() },
        ]
    );
}

#[test]
fn all_five_subset_09_monsters_resolve_via_ruleset_bestiary1() {
    for (id, expected_name, expected_cr) in [
        (MonsterId::Lion, "Lion", 3.0),
        (MonsterId::Ogre, "Ogre", 3.0),
        (MonsterId::Pegasus, "Pegasus", 3.0),
        (MonsterId::RustMonster, "Rust Monster", 3.0),
        (MonsterId::Shadow, "Shadow", 3.0),
    ] {
        let monster = monster_resolve(id, RuleSetId::Bestiary1)
            .unwrap_or_else(|| panic!("{expected_name} should resolve via RuleSetId::Bestiary1"));
        assert_eq!(monster.name, expected_name);
        assert_eq!(monster.challenge_rating, expected_cr);
    }
}

/// Subsets 01-08's forty-one monsters must still resolve unchanged --
/// landing subset 09 must not regress any prior subset
/// (sibling-preservation).
#[test]
fn subset_01_through_08_monsters_still_resolve_unchanged() {
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
        (MonsterId::Choker, "Choker"),
        (MonsterId::Crocodile, "Crocodile"),
        (MonsterId::DarkCreeper, "Dark Creeper"),
        (MonsterId::IronCobra, "Iron Cobra"),
        (MonsterId::Morlock, "Morlock"),
        (MonsterId::RatSwarm, "Rat Swarm"),
        (MonsterId::Sahuagin, "Sahuagin"),
        (MonsterId::Shark, "Shark"),
        (MonsterId::ShockerLizard, "Shocker Lizard"),
        (MonsterId::Skum, "Skum"),
        (MonsterId::Squid, "Squid"),
        (MonsterId::Troglodyte, "Troglodyte"),
        (MonsterId::Vargouille, "Vargouille"),
        (MonsterId::Wolverine, "Wolverine"),
        (MonsterId::Worg, "Worg"),
        (MonsterId::YellowMuskCreeper, "Yellow Musk Creeper"),
        (MonsterId::Ankheg, "Ankheg"),
        (MonsterId::AssassinVine, "Assassin Vine"),
        (MonsterId::Centaur, "Centaur"),
        (MonsterId::Cockatrice, "Cockatrice"),
        (MonsterId::Derro, "Derro"),
        (MonsterId::Doppelganger, "Doppelganger"),
        (MonsterId::Dryad, "Dryad"),
        (MonsterId::Ettercap, "Ettercap"),
        (MonsterId::GelatinousCube, "Gelatinous Cube"),
        (MonsterId::HellHound, "Hell Hound"),
    ] {
        let monster = monster_resolve(id, RuleSetId::Bestiary1)
            .unwrap_or_else(|| panic!("{expected_name} should still resolve via RuleSetId::Bestiary1"));
        assert_eq!(monster.name, expected_name);
    }
}

/// Cross-book resolution invariant (`corpus-source-inventory.md` §3.2,
/// adapted to subset 09's roster): a Bestiary 1 monster resolves via
/// `RuleSetId::Bestiary1` but must return `None` for `RuleSetId::Apg`,
/// `RuleSetId::Acg`, and `RuleSetId::Crb`.
#[test]
fn shadow_returns_none_for_ruleset_apg_acg_crb() {
    assert_eq!(monster_resolve(MonsterId::Shadow, RuleSetId::Apg), None);
    assert_eq!(monster_resolve(MonsterId::Shadow, RuleSetId::Acg), None);
    assert_eq!(monster_resolve(MonsterId::Shadow, RuleSetId::Crb), None);
}

/// Ogre carries no `NATURALATTACKS:` token (fights with weapons);
/// Rust Monster's `Antennae` attack carries a real `0` damage-dice token
/// (its rust effect is a special ability, not weapon damage); Shadow
/// carries no `Walk` component in its `MOVE:` token at all -- this
/// subset's new shape -- so `speed_ft` is `0` rather than a value
/// guessed from its `Fly` speed.
#[test]
fn ogre_rust_monster_and_shadow_carry_this_subsets_real_data_variety() {
    let ogre =
        monster_resolve(MonsterId::Ogre, RuleSetId::Bestiary1).expect("Ogre should resolve via RuleSetId::Bestiary1");
    assert_eq!(ogre.race_subtype.as_deref(), Some("Giant"));
    assert_eq!(ogre.race_type, "Humanoid");
    assert_eq!(ogre.speed_ft, 40);
    assert_eq!(ogre.source_page, "p.220");
    assert!(ogre.natural_attacks.is_empty(), "no NATURALATTACKS: token on the real row");

    let pegasus = monster_resolve(MonsterId::Pegasus, RuleSetId::Bestiary1)
        .expect("Pegasus should resolve via RuleSetId::Bestiary1");
    assert_eq!(pegasus.race_subtype, None);
    assert_eq!(pegasus.race_type, "Magical Beast");
    assert_eq!(pegasus.speed_ft, 60, "MOVE:Walk,60,Fly,120 -- walk speed, not fly");
    assert_eq!(pegasus.source_page, "p.225");
    assert_eq!(
        pegasus.natural_attacks,
        vec![
            NaturalAttack { name: "Bite".to_string(), damage_dice: "1d3".to_string() },
            NaturalAttack { name: "Hoof".to_string(), damage_dice: "1d6".to_string() },
        ]
    );

    let rust_monster = monster_resolve(MonsterId::RustMonster, RuleSetId::Bestiary1)
        .expect("Rust Monster should resolve via RuleSetId::Bestiary1");
    assert_eq!(rust_monster.race_subtype, None);
    assert_eq!(rust_monster.race_type, "Aberration");
    assert_eq!(rust_monster.speed_ft, 40, "MOVE:Walk,40,Climb,10 -- walk speed, not climb");
    assert_eq!(rust_monster.source_page, "p.238");
    assert_eq!(
        rust_monster.natural_attacks,
        vec![
            NaturalAttack { name: "Bite".to_string(), damage_dice: "1d3".to_string() },
            NaturalAttack { name: "Antennae".to_string(), damage_dice: "0".to_string() },
        ]
    );

    let shadow = monster_resolve(MonsterId::Shadow, RuleSetId::Bestiary1)
        .expect("Shadow should resolve via RuleSetId::Bestiary1");
    assert_eq!(shadow.race_subtype.as_deref(), Some("Incorporeal"));
    assert_eq!(shadow.race_type, "Undead");
    assert_eq!(shadow.speed_ft, 0, "MOVE:Fly,40 only -- no Walk token, so no walk speed is transcribed");
    assert_eq!(shadow.source_page, "p.245");
    assert_eq!(
        shadow.natural_attacks,
        vec![NaturalAttack { name: "Incorporeal Touch".to_string(), damage_dice: "1d6".to_string() }]
    );
}

/// Key-based resolution, mirroring `corpus-source-inventory.md` §3.2's
/// `beastiary1:monster:<name>` key shape.
#[test]
fn shadow_resolves_by_key_via_ruleset_bestiary1_only() {
    let monster = monster_key_resolve("beastiary1:monster:shadow", RuleSetId::Bestiary1)
        .expect("beastiary1:monster:shadow should resolve via RuleSetId::Bestiary1");
    assert_eq!(monster.name, "Shadow");
    assert_eq!(monster.race_type, "Undead");

    assert_eq!(monster_key_resolve("beastiary1:monster:shadow", RuleSetId::Apg), None);
    assert_eq!(monster_key_resolve("beastiary1:monster:unknown", RuleSetId::Bestiary1), None);
}
