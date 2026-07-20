//! SD-22 Epic 5 acceptance test — Bestiary 1 monster-block subset 01
//! (criteria 14-17: `rules_tables/beastiary1/mod.rs` populated with the
//! `RuleSetId::Bestiary1` variant registration, per-monster resolution,
//! and the cross-book resolution invariant).
//!
//! **Roster correction (documented, mirroring the established Epic 3
//! Gunslinger/Magus and Epic 4 "Alchemist (ACG-side)" precedent):**
//! `corpus-source-inventory.md` §3.1's subset-01 sample list ("Goblin,
//! Kobold, Orc, Skeleton, Zombie") does not correspond to real,
//! standalone CR-1 monster stat-block rows in the real corpus file
//! (`pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst`):
//! - Goblin, Kobold, and Orc exist in that book only as `.MOD` overrides
//!   in the separate `b1_races_pc.lst` file, layered onto their
//!   *playable-race* base defined in `core_essentials/races/<race>/`,
//!   not as a Bestiary 1 monster stat block with its own CR/combat data.
//! - Skeleton (Human) and Zombie (Human) are real full stat-block rows,
//!   but their real Challenge Ratings are 1/3 and 1/2 respectively, not
//!   1 (the bare `Skeleton`/`Zombie` rows at `b1_races.lst:439-440` are
//!   template-application shims with no CR of their own).
//!
//! This cycle verified `b1_races.lst` directly (not the illustrative
//! sample-monster prose) and found five real, CR-1, fully-populated
//! monster stat-block rows: Ghoul (line 200), Gnoll (line 212), Goblin
//! Dog (line 213), Lizardfolk (line 276), Wolf (line 414) — see
//! `docs/release/SD-22/artifacts/beastiary1/subset_01_cycle_receipt.md`
//! for the full grounding. Subset 01 ships this corrected five-monster
//! roster, in the file's own alphabetical order (matching
//! `corpus-source-inventory.md` §3's own "alphabetical by monster name
//! within CR band" default ordering rule).

use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::beastiary1::{MonsterId, monster_key_resolve, monster_resolve};

#[test]
fn ghoul_resolves_via_ruleset_bestiary1() {
    let monster = monster_resolve(MonsterId::Ghoul, RuleSetId::Bestiary1)
        .expect("Ghoul should resolve via RuleSetId::Bestiary1");
    assert_eq!(monster.name, "Ghoul");
    assert_eq!(monster.challenge_rating, 1.0);
    assert_eq!(monster.speed_ft, 30);
    assert_eq!(monster.race_type, "Undead");
    assert_eq!(monster.source_page, "p.146");
    assert!(monster.natural_attacks.iter().any(|a| a.name == "Claw" && a.damage_dice == "1d6"));
    assert!(monster.natural_attacks.iter().any(|a| a.name == "Bite" && a.damage_dice == "1d6"));
}

#[test]
fn all_five_subset_01_monsters_resolve_via_ruleset_bestiary1() {
    for (id, expected_name, expected_cr) in [
        (MonsterId::Ghoul, "Ghoul", 1.0),
        (MonsterId::Gnoll, "Gnoll", 1.0),
        (MonsterId::GoblinDog, "Goblin Dog", 1.0),
        (MonsterId::Lizardfolk, "Lizardfolk", 1.0),
        (MonsterId::Wolf, "Wolf", 1.0),
    ] {
        let monster = monster_resolve(id, RuleSetId::Bestiary1)
            .unwrap_or_else(|| panic!("{expected_name} should resolve via RuleSetId::Bestiary1"));
        assert_eq!(monster.name, expected_name);
        assert_eq!(monster.challenge_rating, expected_cr);
    }
}

/// Cross-book resolution invariant (`corpus-source-inventory.md` §3.2,
/// adapted to the corrected roster): a Bestiary 1 monster resolves via
/// `RuleSetId::Bestiary1` but must return `None` for `RuleSetId::Apg` and
/// `RuleSetId::Acg` — monsters aren't spell-list items.
#[test]
fn ghoul_returns_none_for_ruleset_apg_and_acg() {
    assert_eq!(monster_resolve(MonsterId::Ghoul, RuleSetId::Apg), None);
    assert_eq!(monster_resolve(MonsterId::Ghoul, RuleSetId::Acg), None);
    assert_eq!(monster_resolve(MonsterId::Ghoul, RuleSetId::Crb), None);
}

/// Key-based resolution, mirroring `corpus-source-inventory.md` §3.2's
/// `beastiary1:monster:<name>` key shape (adapted to the corrected
/// roster — `ghoul` substitutes for the illustrative `goblin` example,
/// see this file's header note).
#[test]
fn ghoul_resolves_by_key_via_ruleset_bestiary1_only() {
    let monster = monster_key_resolve("beastiary1:monster:ghoul", RuleSetId::Bestiary1)
        .expect("beastiary1:monster:ghoul should resolve via RuleSetId::Bestiary1");
    assert_eq!(monster.name, "Ghoul");

    assert_eq!(monster_key_resolve("beastiary1:monster:ghoul", RuleSetId::Apg), None);
    assert_eq!(monster_key_resolve("beastiary1:monster:unknown", RuleSetId::Bestiary1), None);
}
