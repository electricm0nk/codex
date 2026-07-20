//! SD-22 Epic 5 acceptance test — Bestiary 1 monster-block subset 05
//! (criteria 14-17 re-verified against a fifth subset: `mod.rs`
//! registration holds for the new monsters, per-monster resolution
//! works, and the cross-book resolution invariant holds).
//!
//! **CR 2 continuation, alphabetically after subset 04's "Morlock"**
//! (not a roster correction — `corpus-source-inventory.md` §3.1 had no
//! illustrative sample row for subset 5 to correct, only a placeholder
//! `...` row): before writing any GREEN code, this cycle independently
//! re-enumerated every real, non-`#`-commented, non-`.MOD`/`.COPY=`
//! CR:2 monster stat-block row in `b1_races.lst` directly, excluding
//! parenthetical sub-variant names (e.g. "Ant (Giant)", "Cat (Cheetah)",
//! "Demon (Dretch)") the same way subsets 01-04 all already established.
//! There are 34 real `CR:2` rows total in the file; excluding
//! parentheticals leaves 19 clean species names, of which subsets 03+04
//! used the first ten alphabetically (Bat Swarm, Boar, Boggard, Bugbear,
//! Cave Fisher, Choker, Crocodile, Dark Creeper, Iron Cobra, Morlock).
//! This cycle lands the next five alphabetically: **Rat Swarm** (line
//! 334), **Sahuagin** (line 345), **Shark** (line 360), **Shocker
//! Lizard** (line 362), **Skum** (line 366). See
//! `docs/release/SD-22/artifacts/beastiary1/subset_05_cycle_receipt.md`
//! for the full grounding and the new `corpus-source-inventory.md` §3.1
//! row this cycle adds for subset 5.

use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::beastiary1::{MonsterId, monster_key_resolve, monster_resolve};

#[test]
fn rat_swarm_resolves_via_ruleset_bestiary1() {
    let monster = monster_resolve(MonsterId::RatSwarm, RuleSetId::Bestiary1)
        .expect("Rat Swarm should resolve via RuleSetId::Bestiary1");
    assert_eq!(monster.name, "Rat Swarm");
    assert_eq!(monster.challenge_rating, 2.0);
    assert_eq!(monster.size, "T");
    assert_eq!(monster.speed_ft, 15, "walk speed, not the climb/swim speed");
    assert_eq!(monster.race_type, "Animal");
    assert_eq!(monster.race_subtype.as_deref(), Some("Swarm"));
    assert_eq!(monster.source_page, "p.232");
    assert!(monster.natural_attacks.iter().any(|a| a.name == "Swarm" && a.damage_dice == "1d6"));
}

#[test]
fn all_five_subset_05_monsters_resolve_via_ruleset_bestiary1() {
    for (id, expected_name, expected_cr) in [
        (MonsterId::RatSwarm, "Rat Swarm", 2.0),
        (MonsterId::Sahuagin, "Sahuagin", 2.0),
        (MonsterId::Shark, "Shark", 2.0),
        (MonsterId::ShockerLizard, "Shocker Lizard", 2.0),
        (MonsterId::Skum, "Skum", 2.0),
    ] {
        let monster = monster_resolve(id, RuleSetId::Bestiary1)
            .unwrap_or_else(|| panic!("{expected_name} should resolve via RuleSetId::Bestiary1"));
        assert_eq!(monster.name, expected_name);
        assert_eq!(monster.challenge_rating, expected_cr);
    }
}

/// Subsets 01-04's twenty monsters must still resolve unchanged — landing
/// subset 05 must not regress any prior subset (sibling-preservation).
#[test]
fn subset_01_02_03_04_monsters_still_resolve_unchanged() {
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
    ] {
        let monster = monster_resolve(id, RuleSetId::Bestiary1)
            .unwrap_or_else(|| panic!("{expected_name} should still resolve via RuleSetId::Bestiary1"));
        assert_eq!(monster.name, expected_name);
    }
}

/// Cross-book resolution invariant (`corpus-source-inventory.md` §3.2,
/// adapted to subset 05's roster): a Bestiary 1 monster resolves via
/// `RuleSetId::Bestiary1` but must return `None` for `RuleSetId::Apg`,
/// `RuleSetId::Acg`, and `RuleSetId::Crb` — monsters aren't spell-list
/// items.
#[test]
fn skum_returns_none_for_ruleset_apg_acg_crb() {
    assert_eq!(monster_resolve(MonsterId::Skum, RuleSetId::Apg), None);
    assert_eq!(monster_resolve(MonsterId::Skum, RuleSetId::Acg), None);
    assert_eq!(monster_resolve(MonsterId::Skum, RuleSetId::Crb), None);
}

/// Shark's real row carries no `Walk` pair in its `MOVE:` token (only
/// `Swim,60`) — a genuinely different shape from every prior subset's
/// monsters, all of which had a Walk pair. Sahuagin and Skum both carry
/// multiple `NATURALATTACKS:` tokens (one per tab field) that accumulate
/// into a single multi-entry attack list.
#[test]
fn shark_has_no_walk_token_sahuagin_and_skum_accumulate_multiple_natural_attack_tokens() {
    let shark = monster_resolve(MonsterId::Shark, RuleSetId::Bestiary1)
        .expect("Shark should resolve via RuleSetId::Bestiary1");
    assert_eq!(shark.speed_ft, 0, "no Walk pair on the real row -- aquatic-only movement");
    assert!(shark.natural_attacks.iter().any(|a| a.name == "Bite" && a.damage_dice == "1d8"));

    let sahuagin = monster_resolve(MonsterId::Sahuagin, RuleSetId::Bestiary1)
        .expect("Sahuagin should resolve via RuleSetId::Bestiary1");
    assert_eq!(
        sahuagin.natural_attacks.len(),
        3,
        "two NATURALATTACKS: tab fields (Claws, and a pipe-separated Bite pair) accumulate to 3 entries"
    );
    assert!(sahuagin.natural_attacks.iter().any(|a| a.name == "Claws" && a.damage_dice == "1d4"));
    assert!(sahuagin.natural_attacks.iter().any(|a| a.name == "Bite (w/o weapon)" && a.damage_dice == "1d4"));
    assert!(sahuagin.natural_attacks.iter().any(|a| a.name == "Bite (with weapon)" && a.damage_dice == "1d4"));

    let skum = monster_resolve(MonsterId::Skum, RuleSetId::Bestiary1)
        .expect("Skum should resolve via RuleSetId::Bestiary1");
    assert_eq!(
        skum.natural_attacks.len(),
        4,
        "two NATURALATTACKS: tab fields, each a pipe-separated pair, accumulate to 4 entries"
    );
    assert_eq!(skum.race_type, "Monstrous Humanoid");
    assert_eq!(skum.race_subtype.as_deref(), Some("Aquatic"));
}

/// Key-based resolution, mirroring `corpus-source-inventory.md` §3.2's
/// `beastiary1:monster:<name>` key shape.
#[test]
fn shocker_lizard_resolves_by_key_via_ruleset_bestiary1_only() {
    let monster = monster_key_resolve("beastiary1:monster:shocker_lizard", RuleSetId::Bestiary1)
        .expect("beastiary1:monster:shocker_lizard should resolve via RuleSetId::Bestiary1");
    assert_eq!(monster.name, "Shocker Lizard");
    assert_eq!(monster.race_type, "Magical Beast");
    assert_eq!(monster.race_subtype, None);

    assert_eq!(monster_key_resolve("beastiary1:monster:shocker_lizard", RuleSetId::Apg), None);
    assert_eq!(monster_key_resolve("beastiary1:monster:unknown", RuleSetId::Bestiary1), None);
}
