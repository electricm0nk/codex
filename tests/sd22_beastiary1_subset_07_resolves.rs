//! SD-22 Epic 5 acceptance test — Bestiary 1 monster-block subset 07
//! (criteria 14-17 re-verified against a seventh subset: `mod.rs`
//! registration holds for the new monsters, per-monster resolution
//! works, and the cross-book resolution invariant holds).
//!
//! **CR-band move for subset 07** (documented in full in
//! `docs/release/SD-22/artifacts/beastiary1/subset_07_cycle_receipt.md`):
//! CR 1 and CR 2 are both fully exhausted after subset 06 (no unused,
//! non-parenthetical, standalone monster names remain in either band —
//! see `monster_subset_06.rs`'s header), so subset 07 moves to CR 3.
//! Before writing any GREEN code, this cycle independently re-enumerated
//! every real, non-`#`-commented, non-`.MOD`/`.COPY=` CR:3 monster
//! stat-block row in `b1_races.lst` directly against the live corpus
//! file: 44 rows carry a `CR:3` token; excluding parenthetical
//! sub-variant names (e.g. `Ant (Drone)`, `Dragon (Black)`, `Elemental
//! (Air/Medium)`) and `.MOD`-suffixed override rows (e.g. `Iron Cobra
//! (Adamantine Cobra).MOD`) — the same exclusion rule every prior subset
//! has used — leaves 20 clean, standalone CR:3 species names. This
//! subset ships the first five alphabetically: **Ankheg** (line 18),
//! **Assassin Vine** (line 29), **Centaur** (line 60), **Cockatrice**
//! (line 73), **Derro** (line 104).

use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::beastiary1::{MonsterId, monster_key_resolve, monster_resolve};

#[test]
fn ankheg_resolves_via_ruleset_bestiary1() {
    let monster = monster_resolve(MonsterId::Ankheg, RuleSetId::Bestiary1)
        .expect("Ankheg should resolve via RuleSetId::Bestiary1");
    assert_eq!(monster.name, "Ankheg");
    assert_eq!(monster.challenge_rating, 3.0);
    assert_eq!(monster.size, "L");
    assert_eq!(monster.speed_ft, 30, "MOVE:Walk,30,Burrow,20 -- walk speed, not burrow");
    assert_eq!(monster.race_type, "Magical Beast");
    assert_eq!(monster.race_subtype, None);
    assert_eq!(monster.source_page, "p.15");
    // No `NATURALATTACKS:` token on the real row -- the Bite is named by
    // `ABILITY:Internal|AUTOMATIC|Bite` and its `2d6` grounded from
    // published values (`natural_attack_provenance`).
    assert!(monster.natural_attacks.iter().any(|a| a.name == "Bite" && a.damage_dice == "2d6"));
}

#[test]
fn all_five_subset_07_monsters_resolve_via_ruleset_bestiary1() {
    for (id, expected_name, expected_cr) in [
        (MonsterId::Ankheg, "Ankheg", 3.0),
        (MonsterId::AssassinVine, "Assassin Vine", 3.0),
        (MonsterId::Centaur, "Centaur", 3.0),
        (MonsterId::Cockatrice, "Cockatrice", 3.0),
        (MonsterId::Derro, "Derro", 3.0),
    ] {
        let monster = monster_resolve(id, RuleSetId::Bestiary1)
            .unwrap_or_else(|| panic!("{expected_name} should resolve via RuleSetId::Bestiary1"));
        assert_eq!(monster.name, expected_name);
        assert_eq!(monster.challenge_rating, expected_cr);
    }
}

/// Subsets 01-06's thirty-one monsters must still resolve unchanged —
/// landing subset 07 must not regress any prior subset
/// (sibling-preservation).
#[test]
fn subset_01_through_06_monsters_still_resolve_unchanged() {
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
    ] {
        let monster = monster_resolve(id, RuleSetId::Bestiary1)
            .unwrap_or_else(|| panic!("{expected_name} should still resolve via RuleSetId::Bestiary1"));
        assert_eq!(monster.name, expected_name);
    }
}

/// Cross-book resolution invariant (`corpus-source-inventory.md` §3.2,
/// adapted to subset 07's roster): a Bestiary 1 monster resolves via
/// `RuleSetId::Bestiary1` but must return `None` for `RuleSetId::Apg`,
/// `RuleSetId::Acg`, and `RuleSetId::Crb` — monsters aren't spell-list
/// items.
#[test]
fn derro_returns_none_for_ruleset_apg_acg_crb() {
    assert_eq!(monster_resolve(MonsterId::Derro, RuleSetId::Apg), None);
    assert_eq!(monster_resolve(MonsterId::Derro, RuleSetId::Acg), None);
    assert_eq!(monster_resolve(MonsterId::Derro, RuleSetId::Crb), None);
}

/// Derro is the one subset-07 monster carrying a `RACESUBTYPE:` token
/// (`RACESUBTYPE:Derro`); the other four carry none — real-data variety,
/// not fabricated.
#[test]
fn derro_carries_racesubtype_others_do_not() {
    let derro = monster_resolve(MonsterId::Derro, RuleSetId::Bestiary1)
        .expect("Derro should resolve via RuleSetId::Bestiary1");
    assert_eq!(derro.race_subtype.as_deref(), Some("Derro"));
    assert_eq!(derro.size, "S");
    assert_eq!(derro.speed_ft, 20);
    assert_eq!(derro.source_page, "p.70");

    let assassin_vine = monster_resolve(MonsterId::AssassinVine, RuleSetId::Bestiary1)
        .expect("Assassin Vine should resolve via RuleSetId::Bestiary1");
    assert_eq!(assassin_vine.race_subtype, None);
    assert_eq!(assassin_vine.race_type, "Plant");
    assert_eq!(assassin_vine.speed_ft, 5);
    assert_eq!(assassin_vine.source_page, "p.22");

    let centaur = monster_resolve(MonsterId::Centaur, RuleSetId::Bestiary1)
        .expect("Centaur should resolve via RuleSetId::Bestiary1");
    assert_eq!(centaur.race_subtype, None);
    assert_eq!(centaur.race_type, "Monstrous Humanoid");
    assert_eq!(centaur.speed_ft, 50);
    assert_eq!(centaur.source_page, "p.42");

    let cockatrice = monster_resolve(MonsterId::Cockatrice, RuleSetId::Bestiary1)
        .expect("Cockatrice should resolve via RuleSetId::Bestiary1");
    assert_eq!(cockatrice.race_subtype, None);
    assert_eq!(cockatrice.race_type, "Magical Beast");
    assert_eq!(cockatrice.size, "S");
    assert_eq!(cockatrice.speed_ft, 20, "MOVE:Walk,20,Fly,60 -- walk speed, not fly");
    assert_eq!(cockatrice.source_page, "p.48");
}

/// Key-based resolution, mirroring `corpus-source-inventory.md` §3.2's
/// `beastiary1:monster:<name>` key shape.
#[test]
fn derro_resolves_by_key_via_ruleset_bestiary1_only() {
    let monster = monster_key_resolve("beastiary1:monster:derro", RuleSetId::Bestiary1)
        .expect("beastiary1:monster:derro should resolve via RuleSetId::Bestiary1");
    assert_eq!(monster.name, "Derro");
    assert_eq!(monster.race_type, "Humanoid");
    assert_eq!(monster.race_subtype.as_deref(), Some("Derro"));

    assert_eq!(monster_key_resolve("beastiary1:monster:derro", RuleSetId::Apg), None);
    assert_eq!(monster_key_resolve("beastiary1:monster:unknown", RuleSetId::Bestiary1), None);
}
