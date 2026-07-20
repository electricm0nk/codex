//! SD-22 Epic 5 acceptance test — Bestiary 1 monster-block subset 03
//! (criteria 14-17 re-verified against a third subset: `mod.rs`
//! registration holds for the new monsters, per-monster resolution
//! works, and the cross-book resolution invariant holds).
//!
//! **CR-band move (not a roster correction of an existing row — subset 3
//! has no illustrative sample row in `corpus-source-inventory.md` §3.1 to
//! correct; only a placeholder `...` row existed there):** before writing
//! any GREEN code, this cycle enumerated every real, non-`#`-commented,
//! non-`.MOD`/`.COPY=` CR:1 monster stat-block row in `b1_races.lst`
//! directly and found only 12 total across the whole file with a clean,
//! unparenthesized species name (i.e. excluding sub-variant names like
//! "Ghoul (Ghast)" / "Frog (Giant)" / "Ant (Worker)", which name a variant
//! of a broader creature rather than a clean, unambiguous base species —
//! same exclusion rule subset 01 and subset 02 both already established).
//! Of those 12, subset 01 used five (Ghoul, Gnoll, Goblin Dog, Lizardfolk,
//! Wolf) and subset 02 used five more (Darkmantle, Horse, Hyena, Octopus,
//! Spider Swarm), leaving only **Squid** and **Troglodyte** — two
//! monsters, not the five a subset needs. CR 1 is exhausted for a clean
//! five-monster subset.
//!
//! Per the loop-instruction's explicit "move to the next CR band if CR 1
//! is exhausted" guidance, this cycle moved to CR 2 and enumerated every
//! real, non-commented, non-parenthetical CR:2 monster stat-block row in
//! the same file, picking the first five alphabetically: **Bat Swarm**
//! (line 41), **Boar** (line 49), **Boggard** (line 51), **Bugbear**
//! (line 52), **Cave Fisher** (line 59). See
//! `docs/release/SD-22/artifacts/beastiary1/subset_03_cycle_receipt.md`
//! for the full grounding and the new `corpus-source-inventory.md` §3.1
//! row this cycle adds for subset 3.

use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::beastiary1::{MonsterId, monster_key_resolve, monster_resolve};

#[test]
fn bat_swarm_resolves_via_ruleset_bestiary1() {
    let monster = monster_resolve(MonsterId::BatSwarm, RuleSetId::Bestiary1)
        .expect("Bat Swarm should resolve via RuleSetId::Bestiary1");
    assert_eq!(monster.name, "Bat Swarm");
    assert_eq!(monster.challenge_rating, 2.0);
    assert_eq!(monster.speed_ft, 5, "walk speed, not the fly speed");
    assert_eq!(monster.race_type, "Animal");
    assert_eq!(monster.race_subtype.as_deref(), Some("Swarm"));
    assert_eq!(monster.source_page, "p.30");
    assert!(monster.natural_attacks.iter().any(|a| a.name == "Swarm" && a.damage_dice == "1d6"));
}

#[test]
fn all_five_subset_03_monsters_resolve_via_ruleset_bestiary1() {
    for (id, expected_name, expected_cr) in [
        (MonsterId::BatSwarm, "Bat Swarm", 2.0),
        (MonsterId::Boar, "Boar", 2.0),
        (MonsterId::Boggard, "Boggard", 2.0),
        (MonsterId::Bugbear, "Bugbear", 2.0),
        (MonsterId::CaveFisher, "Cave Fisher", 2.0),
    ] {
        let monster = monster_resolve(id, RuleSetId::Bestiary1)
            .unwrap_or_else(|| panic!("{expected_name} should resolve via RuleSetId::Bestiary1"));
        assert_eq!(monster.name, expected_name);
        assert_eq!(monster.challenge_rating, expected_cr);
    }
}

/// Subset 01's and subset 02's ten monsters must still resolve unchanged
/// — landing subset 03 must not regress either prior subset
/// (sibling-preservation).
#[test]
fn subset_01_and_02_monsters_still_resolve_unchanged() {
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
    ] {
        let monster = monster_resolve(id, RuleSetId::Bestiary1)
            .unwrap_or_else(|| panic!("{expected_name} should still resolve via RuleSetId::Bestiary1"));
        assert_eq!(monster.name, expected_name);
    }
}

/// Cross-book resolution invariant (`corpus-source-inventory.md` §3.2,
/// adapted to subset 03's roster): a Bestiary 1 monster resolves via
/// `RuleSetId::Bestiary1` but must return `None` for `RuleSetId::Apg`,
/// `RuleSetId::Acg`, and `RuleSetId::Crb` — monsters aren't spell-list
/// items.
#[test]
fn boggard_returns_none_for_ruleset_apg_acg_crb() {
    assert_eq!(monster_resolve(MonsterId::Boggard, RuleSetId::Apg), None);
    assert_eq!(monster_resolve(MonsterId::Boggard, RuleSetId::Acg), None);
    assert_eq!(monster_resolve(MonsterId::Boggard, RuleSetId::Crb), None);
}

/// Boar and Bugbear both carry no `NATURALATTACKS:` token on their real
/// row (Boar's Gore is granted via `ABILITY:Internal|AUTOMATIC|Gore`;
/// Bugbear fights with manufactured weapons per `AUTO:WEAPONPROF`) — same
/// "no natural attack" shape subset 01's Gnoll and Wolf already proved.
#[test]
fn boar_and_bugbear_have_no_natural_attacks() {
    let boar = monster_resolve(MonsterId::Boar, RuleSetId::Bestiary1)
        .expect("Boar should resolve via RuleSetId::Bestiary1");
    assert!(boar.natural_attacks.is_empty());

    let bugbear = monster_resolve(MonsterId::Bugbear, RuleSetId::Bestiary1)
        .expect("Bugbear should resolve via RuleSetId::Bestiary1");
    assert!(bugbear.natural_attacks.is_empty());
    assert_eq!(bugbear.race_subtype.as_deref(), Some("Goblinoid"));
}

/// Key-based resolution, mirroring `corpus-source-inventory.md` §3.2's
/// `beastiary1:monster:<name>` key shape.
#[test]
fn cave_fisher_resolves_by_key_via_ruleset_bestiary1_only() {
    let monster = monster_key_resolve("beastiary1:monster:cave_fisher", RuleSetId::Bestiary1)
        .expect("beastiary1:monster:cave_fisher should resolve via RuleSetId::Bestiary1");
    assert_eq!(monster.name, "Cave Fisher");
    assert!(monster.natural_attacks.iter().any(|a| a.name == "Filament"));

    assert_eq!(monster_key_resolve("beastiary1:monster:cave_fisher", RuleSetId::Apg), None);
    assert_eq!(monster_key_resolve("beastiary1:monster:unknown", RuleSetId::Bestiary1), None);
}
