//! SD-22 Epic 5 acceptance test — Bestiary 1 monster-block subset 02
//! (criteria 14-17 re-verified against a second subset: `mod.rs`
//! registration holds for the new monsters, per-monster resolution
//! works, and the cross-book resolution invariant holds).
//!
//! **Roster correction (same defect shape as subset 01, and as the
//! already-resolved Epic 3 Gunslinger/Magus and Epic 4 "Alchemist
//! (ACG-side)" roster mismatches):** `corpus-source-inventory.md` §3.1's
//! subset-02 sample list is "Gnoll, Hobgoblin, Lizardfolk, Rat Swarm."
//! Before transcribing anything, this cycle verified each name directly
//! against the real corpus file
//! (`pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst`) and found:
//! - **Gnoll** and **Lizardfolk** were already ingested in subset 01
//!   (`monster_subset_01.rs`) — re-shipping them in subset 02 would be a
//!   duplicate record, not a new one.
//! - **Hobgoblin** has no standalone monster stat-block row in this file
//!   at all (`grep -n '^Hobgoblin\t' b1_races.lst` -> 0 hits); like
//!   Goblin/Kobold/Orc in subset 01's correction, Hobgoblin only appears
//!   as a `.MOD` override in `b1_races_pc.lst`, layered onto its
//!   playable-race base — not an independent Bestiary 1 stat block.
//! - **Rat Swarm** does exist as a real, standalone stat-block row
//!   (`b1_races.lst:334`), but its real `CR:` token is `2`, not `1` —
//!   confirmed by direct inspection, not assumed from the "CR 1" band
//!   label. It belongs to a later CR-band subset, not this CR-1 one.
//!
//! This cycle enumerated every real CR:1 monster stat-block row in
//! `b1_races.lst` directly (every bare tab-delimited row carrying a
//! `CR:1` token, excluding `.MOD`/`.COPY=` rows) and, excluding the five
//! names already used in subset 01 (Ghoul, Gnoll, Goblin Dog, Lizardfolk,
//! Wolf) and excluding parenthetical sub-variant names (e.g. "Ghoul
//! (Ghast)", "Frog (Giant)" — a distinct disambiguation-needing shape
//! subset 01 also avoided), found the next five real, unambiguous,
//! directly-transcribable CR-1 stat-block rows in alphabetical order:
//! **Darkmantle** (line 91), **Horse** (line 235), **Hyena** (line 242),
//! **Octopus** (line 314), **Spider Swarm** (line 379). See
//! `docs/release/SD-22/artifacts/beastiary1/subset_02_cycle_receipt.md`
//! for the full grounding and the corrected `corpus-source-inventory.md`
//! §3.1 row.

use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::beastiary1::{MonsterId, monster_key_resolve, monster_resolve};

#[test]
fn darkmantle_resolves_via_ruleset_bestiary1() {
    let monster = monster_resolve(MonsterId::Darkmantle, RuleSetId::Bestiary1)
        .expect("Darkmantle should resolve via RuleSetId::Bestiary1");
    assert_eq!(monster.name, "Darkmantle");
    assert_eq!(monster.challenge_rating, 1.0);
    assert_eq!(monster.speed_ft, 20);
    assert_eq!(monster.race_type, "Magical Beast");
    assert_eq!(monster.source_page, "p.55");
    assert!(monster.natural_attacks.iter().any(|a| a.name == "Slam" && a.damage_dice == "1d4"));
}

#[test]
fn all_five_subset_02_monsters_resolve_via_ruleset_bestiary1() {
    for (id, expected_name, expected_cr) in [
        (MonsterId::Darkmantle, "Darkmantle", 1.0),
        (MonsterId::Horse, "Horse", 1.0),
        (MonsterId::Hyena, "Hyena", 1.0),
        (MonsterId::Octopus, "Octopus", 1.0),
        (MonsterId::SpiderSwarm, "Spider Swarm", 1.0),
    ] {
        let monster = monster_resolve(id, RuleSetId::Bestiary1)
            .unwrap_or_else(|| panic!("{expected_name} should resolve via RuleSetId::Bestiary1"));
        assert_eq!(monster.name, expected_name);
        assert_eq!(monster.challenge_rating, expected_cr);
    }
}

/// Subset 01's five monsters must still resolve unchanged — landing
/// subset 02 must not regress subset 01 (sibling-preservation).
#[test]
fn subset_01_monsters_still_resolve_unchanged() {
    for (id, expected_name) in [
        (MonsterId::Ghoul, "Ghoul"),
        (MonsterId::Gnoll, "Gnoll"),
        (MonsterId::GoblinDog, "Goblin Dog"),
        (MonsterId::Lizardfolk, "Lizardfolk"),
        (MonsterId::Wolf, "Wolf"),
    ] {
        let monster = monster_resolve(id, RuleSetId::Bestiary1)
            .unwrap_or_else(|| panic!("{expected_name} should still resolve via RuleSetId::Bestiary1"));
        assert_eq!(monster.name, expected_name);
    }
}

/// Cross-book resolution invariant (`corpus-source-inventory.md` §3.2,
/// adapted to subset 02's roster): a Bestiary 1 monster resolves via
/// `RuleSetId::Bestiary1` but must return `None` for `RuleSetId::Apg`,
/// `RuleSetId::Acg`, and `RuleSetId::Crb` — monsters aren't spell-list
/// items.
#[test]
fn octopus_returns_none_for_ruleset_apg_acg_crb() {
    assert_eq!(monster_resolve(MonsterId::Octopus, RuleSetId::Apg), None);
    assert_eq!(monster_resolve(MonsterId::Octopus, RuleSetId::Acg), None);
    assert_eq!(monster_resolve(MonsterId::Octopus, RuleSetId::Crb), None);
}

/// Octopus carries two pipe-separated `NATURALATTACKS:` entries in one
/// token (`Bite,...,*1,1d3|Tentacle,...,*1,0`) — confirms the multi-attack
/// parsing path already proven by Lizardfolk in subset 01 also holds here.
#[test]
fn octopus_has_both_natural_attacks() {
    let monster = monster_resolve(MonsterId::Octopus, RuleSetId::Bestiary1)
        .expect("Octopus should resolve via RuleSetId::Bestiary1");
    assert!(monster.natural_attacks.iter().any(|a| a.name == "Bite" && a.damage_dice == "1d3"));
    assert!(monster.natural_attacks.iter().any(|a| a.name == "Tentacle"));
}

/// Key-based resolution, mirroring `corpus-source-inventory.md` §3.2's
/// `beastiary1:monster:<name>` key shape.
#[test]
fn spider_swarm_resolves_by_key_via_ruleset_bestiary1_only() {
    let monster = monster_key_resolve("beastiary1:monster:spider_swarm", RuleSetId::Bestiary1)
        .expect("beastiary1:monster:spider_swarm should resolve via RuleSetId::Bestiary1");
    assert_eq!(monster.name, "Spider Swarm");

    assert_eq!(monster_key_resolve("beastiary1:monster:spider_swarm", RuleSetId::Apg), None);
    assert_eq!(monster_key_resolve("beastiary1:monster:unknown", RuleSetId::Bestiary1), None);
}
