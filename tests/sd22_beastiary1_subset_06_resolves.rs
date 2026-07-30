//! SD-22 Epic 5 acceptance test — Bestiary 1 monster-block subset 06
//! (criteria 14-17 re-verified against a sixth subset: `mod.rs`
//! registration holds for the new monsters, per-monster resolution
//! works, and the cross-book resolution invariant holds).
//!
//! **Band-exhaustion cleanup subset, not a straight CR-band continuation**
//! (documented in full in
//! `docs/release/SD-22/artifacts/beastiary1/subset_06_cycle_receipt.md`):
//! before writing any GREEN code, this cycle independently re-enumerated
//! every real, non-`#`-commented, non-`.MOD`/`.COPY=` CR:1 and CR:2
//! monster stat-block row in `b1_races.lst` directly against the live
//! corpus file. Only **two** unused, non-parenthetical CR:1 names remain
//! (Squid, Troglodyte — left over since subsets 01+02 exhausted the rest
//! of CR 1) and only **four** unused, non-parenthetical CR:2 names remain
//! (Vargouille, Wolverine, Worg, Yellow Musk Creeper — the last four of
//! the 19 clean CR:2 species names after subsets 03/04/05 used the first
//! fifteen). Neither remainder alone reaches five monsters. Rather than
//! ship an undersized four-monster subset now and a two-monster subset
//! later (or jump straight to CR 3 and strand the CR-1/CR-2 leftovers
//! indefinitely), this cycle combines both remainders into one
//! six-monster subset that fully exhausts CR 1 and CR 2, so subset 07 can
//! start CR 3 cleanly. The six names, sorted alphabetically across the
//! combined remainder pool (the sort happens to fall out in CR-ascending
//! order too — Squid/Troglodyte at CR 1 both alphabetically precede
//! Vargouille/Wolverine/Worg/Yellow Musk Creeper at CR 2): **Squid**
//! (line 380, CR 1), **Troglodyte** (line 390, CR 1), **Vargouille**
//! (line 401, CR 2), **Wolverine** (line 416, CR 2), **Worg** (line 418,
//! CR 2), **Yellow Musk Creeper** (line 430, CR 2).

use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::beastiary1::{MonsterId, monster_key_resolve, monster_resolve};

#[test]
fn squid_resolves_via_ruleset_bestiary1() {
    let monster = monster_resolve(MonsterId::Squid, RuleSetId::Bestiary1)
        .expect("Squid should resolve via RuleSetId::Bestiary1");
    assert_eq!(monster.name, "Squid");
    assert_eq!(monster.challenge_rating, 1.0);
    assert_eq!(monster.size, "M");
    assert_eq!(monster.speed_ft, 0, "MOVE:Swim,60,Jet,240 has no Walk pair on the real row");
    assert_eq!(monster.race_type, "Animal");
    assert_eq!(monster.race_subtype.as_deref(), Some("Aquatic"));
    assert_eq!(monster.source_page, "p.259");
    assert!(monster.natural_attacks.iter().any(|a| a.name == "Bite" && a.damage_dice == "1d3"));
    assert!(monster.natural_attacks.iter().any(|a| a.name == "Tentacles" && a.damage_dice == "1d4"));
}

#[test]
fn all_six_subset_06_monsters_resolve_via_ruleset_bestiary1() {
    for (id, expected_name, expected_cr) in [
        (MonsterId::Squid, "Squid", 1.0),
        (MonsterId::Troglodyte, "Troglodyte", 1.0),
        (MonsterId::Vargouille, "Vargouille", 2.0),
        (MonsterId::Wolverine, "Wolverine", 2.0),
        (MonsterId::Worg, "Worg", 2.0),
        (MonsterId::YellowMuskCreeper, "Yellow Musk Creeper", 2.0),
    ] {
        let monster = monster_resolve(id, RuleSetId::Bestiary1)
            .unwrap_or_else(|| panic!("{expected_name} should resolve via RuleSetId::Bestiary1"));
        assert_eq!(monster.name, expected_name);
        assert_eq!(monster.challenge_rating, expected_cr);
    }
}

/// Subsets 01-05's twenty-five monsters must still resolve unchanged —
/// landing subset 06 must not regress any prior subset
/// (sibling-preservation).
#[test]
fn subset_01_through_05_monsters_still_resolve_unchanged() {
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
    ] {
        let monster = monster_resolve(id, RuleSetId::Bestiary1)
            .unwrap_or_else(|| panic!("{expected_name} should still resolve via RuleSetId::Bestiary1"));
        assert_eq!(monster.name, expected_name);
    }
}

/// Cross-book resolution invariant (`corpus-source-inventory.md` §3.2,
/// adapted to subset 06's roster): a Bestiary 1 monster resolves via
/// `RuleSetId::Bestiary1` but must return `None` for `RuleSetId::Apg`,
/// `RuleSetId::Acg`, and `RuleSetId::Crb` — monsters aren't spell-list
/// items.
#[test]
fn yellow_musk_creeper_returns_none_for_ruleset_apg_acg_crb() {
    assert_eq!(monster_resolve(MonsterId::YellowMuskCreeper, RuleSetId::Apg), None);
    assert_eq!(monster_resolve(MonsterId::YellowMuskCreeper, RuleSetId::Acg), None);
    assert_eq!(monster_resolve(MonsterId::YellowMuskCreeper, RuleSetId::Crb), None);
}

/// Troglodyte's real row carries two separate `NATURALATTACKS:` tab
/// fields, each pipe-separated, accumulating into four entries — the
/// same multi-token-accumulation shape subset 05's Sahuagin/Skum already
/// proved. Vargouille, Wolverine, and Worg's real rows carry no
/// `NATURALATTACKS:` token at all — they name their attacks via
/// `ABILITY:Internal` cross-references, so their dice are grounded from
/// published values rather than transcribed
/// (`rules_tables::beastiary1::natural_attack_provenance`). This test
/// previously asserted all three were attack-less, which mistook a
/// missing corpus token for a missing attack.
#[test]
fn troglodyte_accumulates_four_natural_attacks_and_vargouille_wolverine_worg_are_grounded() {
    let troglodyte = monster_resolve(MonsterId::Troglodyte, RuleSetId::Bestiary1)
        .expect("Troglodyte should resolve via RuleSetId::Bestiary1");
    assert_eq!(
        troglodyte.natural_attacks.len(),
        4,
        "two NATURALATTACKS: tab fields, each a pipe-separated pair, accumulate to 4 entries"
    );
    assert!(troglodyte.natural_attacks.iter().any(|a| a.name == "Claw" && a.damage_dice == "1d4"));
    assert!(
        troglodyte
            .natural_attacks
            .iter()
            .any(|a| a.name == "Claw (with weapon attack)" && a.damage_dice == "1d4")
    );
    assert!(troglodyte.natural_attacks.iter().any(|a| a.name == "Bite" && a.damage_dice == "1d4"));
    assert!(
        troglodyte
            .natural_attacks
            .iter()
            .any(|a| a.name == "Bite (with weapon attack)" && a.damage_dice == "1d4")
    );

    let vargouille = monster_resolve(MonsterId::Vargouille, RuleSetId::Bestiary1)
        .expect("Vargouille should resolve via RuleSetId::Bestiary1");
    // No `NATURALATTACKS:` token on the real row -- the Bite is named by
    // `ABILITY:Internal|AUTOMATIC|Bite` and its dice grounded. The kiss
    // and shriek stay out: every source lists them under Special Attacks
    // with no damage dice at all.
    assert!(vargouille.natural_attacks.iter().any(|a| a.name == "Bite" && a.damage_dice == "1d4"));
    assert!(!vargouille.natural_attacks.iter().any(|a| a.name == "Kiss" || a.name == "Shriek"));
    assert_eq!(vargouille.speed_ft, 0, "MOVE:Fly,30 has no Walk pair on the real row");

    let wolverine = monster_resolve(MonsterId::Wolverine, RuleSetId::Bestiary1)
        .expect("Wolverine should resolve via RuleSetId::Bestiary1");
    // Row names both attacks via `ABILITY:Internal|AUTOMATIC|Bite|Claw`;
    // list order follows that operand order.
    assert!(wolverine.natural_attacks.iter().any(|a| a.name == "Bite" && a.damage_dice == "1d4"));
    assert!(wolverine.natural_attacks.iter().any(|a| a.name == "Claw" && a.damage_dice == "1d6"));
    assert_eq!(wolverine.speed_ft, 30, "walk speed, not the burrow/climb speed");

    let worg = monster_resolve(MonsterId::Worg, RuleSetId::Bestiary1)
        .expect("Worg should resolve via RuleSetId::Bestiary1");
    assert!(worg.natural_attacks.iter().any(|a| a.name == "Bite" && a.damage_dice == "1d6"));
    assert_eq!(worg.speed_ft, 50);
}

/// Key-based resolution, mirroring `corpus-source-inventory.md` §3.2's
/// `beastiary1:monster:<name>` key shape.
#[test]
fn yellow_musk_creeper_resolves_by_key_via_ruleset_bestiary1_only() {
    let monster = monster_key_resolve("beastiary1:monster:yellow_musk_creeper", RuleSetId::Bestiary1)
        .expect("beastiary1:monster:yellow_musk_creeper should resolve via RuleSetId::Bestiary1");
    assert_eq!(monster.name, "Yellow Musk Creeper");
    assert_eq!(monster.race_type, "Plant");
    assert_eq!(monster.race_subtype, None);

    assert_eq!(
        monster_key_resolve("beastiary1:monster:yellow_musk_creeper", RuleSetId::Apg),
        None
    );
    assert_eq!(monster_key_resolve("beastiary1:monster:unknown", RuleSetId::Bestiary1), None);
}
