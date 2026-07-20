//! Bestiary 1 monster-block subset 02 — corrected CR-1 roster.
//!
//! Source: PCGen `pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst`,
//! parsed via `pcgen_import::lst_parser::monster_stat_block` (the same
//! bare-tab-delimited monster parser subset 01 introduced — no widening
//! was needed for this subset; every field below already falls inside
//! that parser's existing recognition surface). Every field is
//! transcribed directly from the cited real `.lst` line's tokens — see
//! each function's doc comment for the exact line number and tokens.
//!
//! **Roster correction** (see `super`'s module doc comment for the full
//! story): `corpus-source-inventory.md` §3.1's illustrative subset-02
//! sample list is "Gnoll, Hobgoblin, Lizardfolk, Rat Swarm." That list is
//! wrong for three independent reasons:
//! - `Gnoll` and `Lizardfolk` were **already ingested in subset 01**
//!   (`monster_subset_01.rs`) — re-shipping them here would duplicate an
//!   existing record, not add a new one.
//! - `Hobgoblin` does not exist as a standalone monster stat-block row in
//!   `b1_races.lst` at all (`grep -n '^Hobgoblin\t' b1_races.lst` -> 0
//!   hits). Like subset 01's Goblin/Kobold/Orc, it exists only as a
//!   `.MOD` override in the separate `b1_races_pc.lst` file, layered onto
//!   its playable-race base — not an independent Bestiary 1 stat block.
//! - `Rat Swarm` **does** exist as a real, standalone stat-block row
//!   (`b1_races.lst:334`), but its real `CR:` token is `2`, not `1`
//!   (confirmed by direct inspection, not assumed from the "CR 1" band
//!   label) — it belongs to a later CR-band subset, not this CR-1 one.
//!
//! This cycle enumerated every real CR:1 monster stat-block row in
//! `b1_races.lst` directly (bare tab-delimited rows carrying a literal
//! `CR:1` token, excluding `.MOD`/`.COPY=` rows), excluded the five names
//! already used in subset 01 (Ghoul, Gnoll, Goblin Dog, Lizardfolk,
//! Wolf), and excluded parenthetical sub-variant names (e.g. "Ghoul
//! (Ghast)", "Frog (Giant)", "Skeletal Champion (Human)") the same way
//! subset 01 did — those name a variant of a broader creature rather than
//! a clean, unambiguous base species. What remained, in alphabetical
//! order, is subset 02's roster: **Darkmantle** (line 91), **Horse**
//! (line 235), **Hyena** (line 242), **Octopus** (line 314), **Spider
//! Swarm** (line 379).
//!
//! **Scope boundary** (mirrors `monster_subset_01.rs` and every SD-22
//! Epic 3/4 class chassis module): only fields literally present as
//! tokens on the real row are transcribed. AC, HP, and Fort/Ref/Will
//! saves are PCGen-computed at runtime from the `MONSTERCLASS:` hit-dice
//! table and ability-score modifiers, not literal row tokens —
//! transcribing invented values for them would be exactly the
//! fabricated-data risk `AGENTS.md` and the CRB precedent rule out, so
//! they are deferred to a future ingest slice.

use super::{MonsterStatBlock, NaturalAttack};

/// Source: `b1_races.lst:91`, `CR:1`. Real row tokens: `SIZE:S`,
/// `MOVE:Walk,20,Fly,30` (walk speed transcribed; fly speed out of scope
/// per this module's field-coverage boundary),
/// `NATURALATTACKS:Slam,Weapon.Natural...,*1,1d4`, `RACETYPE:Magical
/// Beast`, `CR:1`, `SOURCEPAGE:p.55`.
pub fn darkmantle() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Darkmantle".to_string(),
        challenge_rating: 1.0,
        size: "S".to_string(),
        speed_ft: 20,
        race_type: "Magical Beast".to_string(),
        race_subtype: None,
        source_page: "p.55".to_string(),
        natural_attacks: vec![NaturalAttack {
            name: "Slam".to_string(),
            damage_dice: "1d4".to_string(),
        }],
    }
}

/// Source: `b1_races.lst:235`, `CR:1`. Real row tokens: `SIZE:L`,
/// `MOVE:Walk,50`, `NATURALATTACKS:Hoof,Weapon.Natural...,*2,1d4`,
/// `RACETYPE:Animal`, `CR:1`, `SOURCEPAGE:p.177`.
pub fn horse() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Horse".to_string(),
        challenge_rating: 1.0,
        size: "L".to_string(),
        speed_ft: 50,
        race_type: "Animal".to_string(),
        race_subtype: None,
        source_page: "p.177".to_string(),
        natural_attacks: vec![NaturalAttack {
            name: "Hoof".to_string(),
            damage_dice: "1d4".to_string(),
        }],
    }
}

/// Source: `b1_races.lst:242`, `CR:1`. Real row tokens: `SIZE:M`,
/// `MOVE:Walk,50`, `NATURALATTACKS:Bite,Weapon.Natural...,*1,1d6`,
/// `RACETYPE:Animal`, `CR:1`, `SOURCEPAGE:p.179`.
pub fn hyena() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Hyena".to_string(),
        challenge_rating: 1.0,
        size: "M".to_string(),
        speed_ft: 50,
        race_type: "Animal".to_string(),
        race_subtype: None,
        source_page: "p.179".to_string(),
        natural_attacks: vec![NaturalAttack {
            name: "Bite".to_string(),
            damage_dice: "1d6".to_string(),
        }],
    }
}

/// Source: `b1_races.lst:314`, `CR:1`. Real row tokens: `SIZE:S`,
/// `MOVE:Walk,20,Swim,30` (walk speed transcribed; swim speed out of
/// scope per this module's field-coverage boundary),
/// `NATURALATTACKS:Bite,Weapon.Natural...,*1,1d3|Tentacle,Weapon.Natural...,*1,0`
/// (two pipe-separated attacks in one token — same multi-attack shape
/// Lizardfolk proved in subset 01), `RACETYPE:Animal`,
/// `RACESUBTYPE:Aquatic`, `CR:1`, `SOURCEPAGE:p.219`.
pub fn octopus() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Octopus".to_string(),
        challenge_rating: 1.0,
        size: "S".to_string(),
        speed_ft: 20,
        race_type: "Animal".to_string(),
        race_subtype: Some("Aquatic".to_string()),
        source_page: "p.219".to_string(),
        natural_attacks: vec![
            NaturalAttack { name: "Bite".to_string(), damage_dice: "1d3".to_string() },
            NaturalAttack { name: "Tentacle".to_string(), damage_dice: "0".to_string() },
        ],
    }
}

/// Source: `b1_races.lst:379`, `CR:1`. Real row tokens: `SIZE:D`,
/// `MOVE:Walk,20,Climb,20` (walk speed transcribed; climb speed out of
/// scope per this module's field-coverage boundary),
/// `NATURALATTACKS:Swarm,Weapon.Natural...,*1,1d6`, `RACETYPE:Vermin`,
/// `RACESUBTYPE:Swarm`, `CR:1`, `SOURCEPAGE:p.258`.
pub fn spider_swarm() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Spider Swarm".to_string(),
        challenge_rating: 1.0,
        size: "D".to_string(),
        speed_ft: 20,
        race_type: "Vermin".to_string(),
        race_subtype: Some("Swarm".to_string()),
        source_page: "p.258".to_string(),
        natural_attacks: vec![NaturalAttack {
            name: "Swarm".to_string(),
            damage_dice: "1d6".to_string(),
        }],
    }
}
