//! SD-22 Epic 5 acceptance test — Bestiary 1 monster-block subset 08
//! (criteria 14-17 re-verified against an eighth subset: `mod.rs`
//! registration holds for the new monsters, per-monster resolution
//! works, and the cross-book resolution invariant holds).
//!
//! **Subset 08** (documented in full in
//! `docs/release/SD-22/artifacts/beastiary1/subset_08_cycle_receipt.md`):
//! continues CR 3 alphabetically after subset 07's "Derro". Before
//! writing any GREEN code, this cycle independently re-enumerated every
//! real, non-`#`-commented, non-`.MOD`/`.COPY=` CR:3 monster stat-block
//! row in `b1_races.lst` directly against the live corpus file: 20
//! clean, standalone (non-parenthetical) CR:3 species names exist total.
//! Subset 07 shipped the first five alphabetically (Ankheg, Assassin
//! Vine, Centaur, Cockatrice, Derro); this subset ships the next five:
//! **Doppelganger** (line 127), **Dryad** (line 141), **Ettercap** (line
//! 175), **Gelatinous Cube** (line 189), **Hell Hound** (line 230).
//! `Hell Hound (Nessian)` (line 231) is a parenthetical sub-variant and
//! excluded, same rule every prior subset has used.

use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::beastiary1::{MonsterId, monster_key_resolve, monster_resolve};

#[test]
fn doppelganger_resolves_via_ruleset_bestiary1() {
    let monster = monster_resolve(MonsterId::Doppelganger, RuleSetId::Bestiary1)
        .expect("Doppelganger should resolve via RuleSetId::Bestiary1");
    assert_eq!(monster.name, "Doppelganger");
    assert_eq!(monster.challenge_rating, 3.0);
    assert_eq!(monster.size, "M");
    assert_eq!(monster.speed_ft, 30);
    assert_eq!(monster.race_type, "Monstrous Humanoid");
    assert_eq!(monster.race_subtype.as_deref(), Some("Shapechanger"));
    assert_eq!(monster.source_page, "p.89");
    assert_eq!(
        monster.natural_attacks,
        vec![codex::rules_core::rules_tables::beastiary1::NaturalAttack {
            name: "Claw".to_string(),
            damage_dice: "1d8".to_string(),
        }]
    );
}

#[test]
fn all_five_subset_08_monsters_resolve_via_ruleset_bestiary1() {
    for (id, expected_name, expected_cr) in [
        (MonsterId::Doppelganger, "Doppelganger", 3.0),
        (MonsterId::Dryad, "Dryad", 3.0),
        (MonsterId::Ettercap, "Ettercap", 3.0),
        (MonsterId::GelatinousCube, "Gelatinous Cube", 3.0),
        (MonsterId::HellHound, "Hell Hound", 3.0),
    ] {
        let monster = monster_resolve(id, RuleSetId::Bestiary1)
            .unwrap_or_else(|| panic!("{expected_name} should resolve via RuleSetId::Bestiary1"));
        assert_eq!(monster.name, expected_name);
        assert_eq!(monster.challenge_rating, expected_cr);
    }
}

/// Subsets 01-07's thirty-six monsters must still resolve unchanged —
/// landing subset 08 must not regress any prior subset
/// (sibling-preservation).
#[test]
fn subset_01_through_07_monsters_still_resolve_unchanged() {
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
    ] {
        let monster = monster_resolve(id, RuleSetId::Bestiary1)
            .unwrap_or_else(|| panic!("{expected_name} should still resolve via RuleSetId::Bestiary1"));
        assert_eq!(monster.name, expected_name);
    }
}

/// Cross-book resolution invariant (`corpus-source-inventory.md` §3.2,
/// adapted to subset 08's roster): a Bestiary 1 monster resolves via
/// `RuleSetId::Bestiary1` but must return `None` for `RuleSetId::Apg`,
/// `RuleSetId::Acg`, and `RuleSetId::Crb` — monsters aren't spell-list
/// items.
#[test]
fn hell_hound_returns_none_for_ruleset_apg_acg_crb() {
    assert_eq!(monster_resolve(MonsterId::HellHound, RuleSetId::Apg), None);
    assert_eq!(monster_resolve(MonsterId::HellHound, RuleSetId::Acg), None);
    assert_eq!(monster_resolve(MonsterId::HellHound, RuleSetId::Crb), None);
}

/// Doppelganger and Hell Hound both carry a `RACESUBTYPE:` token
/// (Doppelganger single-value, Hell Hound pipe-separated multi-value);
/// Dryad, Ettercap, and Gelatinous Cube carry none — real-data variety,
/// not fabricated.
#[test]
fn racesubtype_and_natural_attack_variety_across_subset_08() {
    let doppelganger = monster_resolve(MonsterId::Doppelganger, RuleSetId::Bestiary1)
        .expect("Doppelganger should resolve via RuleSetId::Bestiary1");
    assert_eq!(doppelganger.race_subtype.as_deref(), Some("Shapechanger"));

    let dryad = monster_resolve(MonsterId::Dryad, RuleSetId::Bestiary1)
        .expect("Dryad should resolve via RuleSetId::Bestiary1");
    assert_eq!(dryad.race_subtype, None);
    assert_eq!(dryad.race_type, "Fey");
    assert_eq!(dryad.speed_ft, 30);
    assert_eq!(dryad.source_page, "p.116");
    assert!(dryad.natural_attacks.is_empty(), "no NATURALATTACKS: token on the real row");

    let ettercap = monster_resolve(MonsterId::Ettercap, RuleSetId::Bestiary1)
        .expect("Ettercap should resolve via RuleSetId::Bestiary1");
    assert_eq!(ettercap.race_subtype, None);
    assert_eq!(ettercap.race_type, "Aberration");
    assert_eq!(ettercap.speed_ft, 30, "MOVE:Walk,30,Climb,30 -- walk speed, not climb");
    assert_eq!(ettercap.source_page, "p.129");
    assert_eq!(
        ettercap.natural_attacks,
        vec![
            codex::rules_core::rules_tables::beastiary1::NaturalAttack {
                name: "Bite".to_string(),
                damage_dice: "1d6".to_string(),
            },
            codex::rules_core::rules_tables::beastiary1::NaturalAttack {
                name: "Claw".to_string(),
                damage_dice: "1d4".to_string(),
            },
        ]
    );

    let gelatinous_cube = monster_resolve(MonsterId::GelatinousCube, RuleSetId::Bestiary1)
        .expect("Gelatinous Cube should resolve via RuleSetId::Bestiary1");
    assert_eq!(gelatinous_cube.race_subtype, None);
    assert_eq!(gelatinous_cube.race_type, "Ooze");
    assert_eq!(gelatinous_cube.size, "L");
    assert_eq!(gelatinous_cube.speed_ft, 15);
    assert_eq!(gelatinous_cube.source_page, "p.138");
    assert_eq!(
        gelatinous_cube.natural_attacks,
        vec![codex::rules_core::rules_tables::beastiary1::NaturalAttack {
            name: "Slam".to_string(),
            damage_dice: "1d6".to_string(),
        }]
    );

    let hell_hound = monster_resolve(MonsterId::HellHound, RuleSetId::Bestiary1)
        .expect("Hell Hound should resolve via RuleSetId::Bestiary1");
    assert_eq!(hell_hound.race_subtype.as_deref(), Some("Evil|Extraplanar|Fire|Lawful"));
    assert_eq!(hell_hound.race_type, "Outsider");
    assert_eq!(hell_hound.speed_ft, 40);
    assert_eq!(hell_hound.source_page, "p.173");
    assert_eq!(
        hell_hound.natural_attacks,
        vec![codex::rules_core::rules_tables::beastiary1::NaturalAttack {
            name: "Bite".to_string(),
            damage_dice: "1d8".to_string(),
        }]
    );
}

/// Key-based resolution, mirroring `corpus-source-inventory.md` §3.2's
/// `beastiary1:monster:<name>` key shape.
#[test]
fn hell_hound_resolves_by_key_via_ruleset_bestiary1_only() {
    let monster = monster_key_resolve("beastiary1:monster:hell_hound", RuleSetId::Bestiary1)
        .expect("beastiary1:monster:hell_hound should resolve via RuleSetId::Bestiary1");
    assert_eq!(monster.name, "Hell Hound");
    assert_eq!(monster.race_type, "Outsider");

    assert_eq!(monster_key_resolve("beastiary1:monster:hell_hound", RuleSetId::Apg), None);
    assert_eq!(monster_key_resolve("beastiary1:monster:unknown", RuleSetId::Bestiary1), None);
}
