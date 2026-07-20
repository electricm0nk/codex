//! Bestiary 1 monster-block subset 07 — CR-band move to CR 3 (CR 1 and
//! CR 2 both fully exhausted after subset 06).
//!
//! Source: PCGen `pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst`,
//! parsed via `pcgen_import::lst_parser::monster_stat_block` (the same
//! bare-tab-delimited monster parser subset 01 introduced — no widening
//! was needed for this subset either). Every field below is transcribed
//! directly from the cited real `.lst` line's tokens — see each
//! function's doc comment for the exact line number and tokens.
//!
//! **Roster derivation (CR-band move):** before writing any GREEN code,
//! this cycle independently re-enumerated every real, non-`#`-commented,
//! non-`.MOD`/`.COPY=` CR:3 monster stat-block row in `b1_races.lst`
//! directly against the live corpus file (not from any prior cycle's
//! summary): 44 rows carry a `CR:3` token; excluding parenthetical
//! sub-variant names (e.g. `Ant (Drone)`, `Ape (Dire)`, `Dragon
//! (Black)`, `Elemental (Air/Medium)`) and `.MOD`-suffixed override rows
//! (e.g. `Iron Cobra (Adamantine Cobra).MOD`) — the same exclusion rule
//! every prior subset has used — leaves 20 clean, standalone CR:3
//! species names. This subset ships the first five alphabetically:
//! **Ankheg** (line 18), **Assassin Vine** (line 29), **Centaur** (line
//! 60), **Cockatrice** (line 73), **Derro** (line 104).
//!
//! **Scope boundary** (mirrors `monster_subset_01.rs` through
//! `monster_subset_06.rs` and every SD-22 Epic 3/4 class chassis module):
//! only fields literally present as tokens on the real row are
//! transcribed. AC, HP, and Fort/Ref/Will saves are PCGen-computed at
//! runtime from the `MONSTERCLASS:` hit-dice table and ability-score
//! modifiers, not literal row tokens — transcribing invented values for
//! them would be exactly the fabricated-data risk `AGENTS.md` and the
//! CRB precedent rule out, so they are deferred to a future ingest
//! slice.
//!
//! **Shape this subset exercises (precedent already established, not new
//! parser surface):** all five monsters' real rows carry no
//! `NATURALATTACKS:` token at all — the same "fights via an
//! `ABILITY:Internal` cross-reference (or, for Derro, weapons) instead"
//! shape subset 04's Choker/Crocodile/Dark Creeper and subset 06's
//! Vargouille/Wolverine/Worg already proved; transcribed here as empty
//! lists, not invented attacks. Cockatrice's row carries `MOVE:Walk,20,
//! Fly,60` — walk speed transcribed, fly speed out of scope, same shape
//! subset 01's flying/swimming monsters already proved. Derro is the
//! only monster in this subset carrying a `RACESUBTYPE:` token
//! (`RACESUBTYPE:Derro`).

use super::MonsterStatBlock;

/// Source: `b1_races.lst:18`, `CR:3`. Real row tokens: `SIZE:L`,
/// `MOVE:Walk,30,Burrow,20` (walk speed transcribed; burrow speed out of
/// scope), no `NATURALATTACKS:` token (fights via
/// `ABILITY:Internal|AUTOMATIC|Bite` cross-reference instead —
/// transcribed as an empty list, not an invented attack), `RACETYPE:
/// Magical Beast`, no `RACESUBTYPE:` token, `CR:3`, `SOURCEPAGE:p.15`.
pub fn ankheg() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Ankheg".to_string(),
        challenge_rating: 3.0,
        size: "L".to_string(),
        speed_ft: 30,
        race_type: "Magical Beast".to_string(),
        race_subtype: None,
        source_page: "p.15".to_string(),
        natural_attacks: vec![],
    }
}

/// Source: `b1_races.lst:29`, `CR:3`. Real row tokens: `SIZE:L`,
/// `MOVE:Walk,5`, no `NATURALATTACKS:` token (fights via
/// `ABILITY:Internal|AUTOMATIC|Slam` cross-reference instead —
/// transcribed as an empty list), `RACETYPE:Plant`, no `RACESUBTYPE:`
/// token, `CR:3`, `SOURCEPAGE:p.22`.
pub fn assassin_vine() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Assassin Vine".to_string(),
        challenge_rating: 3.0,
        size: "L".to_string(),
        speed_ft: 5,
        race_type: "Plant".to_string(),
        race_subtype: None,
        source_page: "p.22".to_string(),
        natural_attacks: vec![],
    }
}

/// Source: `b1_races.lst:60`, `CR:3`. Real row tokens: `SIZE:L`,
/// `MOVE:Walk,50`, no `NATURALATTACKS:` token (fights via
/// `ABILITY:Internal|AUTOMATIC|Hoof` cross-reference instead —
/// transcribed as an empty list), `RACETYPE:Monstrous Humanoid`, no
/// `RACESUBTYPE:` token, `CR:3`, `SOURCEPAGE:p.42`.
pub fn centaur() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Centaur".to_string(),
        challenge_rating: 3.0,
        size: "L".to_string(),
        speed_ft: 50,
        race_type: "Monstrous Humanoid".to_string(),
        race_subtype: None,
        source_page: "p.42".to_string(),
        natural_attacks: vec![],
    }
}

/// Source: `b1_races.lst:73`, `CR:3`. Real row tokens: `SIZE:S`,
/// `MOVE:Walk,20,Fly,60` (walk speed transcribed; fly speed out of
/// scope), no `NATURALATTACKS:` token (fights via
/// `ABILITY:Internal|AUTOMATIC|Bite` cross-reference instead —
/// transcribed as an empty list), `RACETYPE:Magical Beast`, no
/// `RACESUBTYPE:` token, `CR:3`, `SOURCEPAGE:p.48`.
pub fn cockatrice() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Cockatrice".to_string(),
        challenge_rating: 3.0,
        size: "S".to_string(),
        speed_ft: 20,
        race_type: "Magical Beast".to_string(),
        race_subtype: None,
        source_page: "p.48".to_string(),
        natural_attacks: vec![],
    }
}

/// Source: `b1_races.lst:104`, `CR:3`. Real row tokens: `SIZE:S`,
/// `MOVE:Walk,20`, no `NATURALATTACKS:` token (fights with weapons —
/// `AUTO:WEAPONPROF|Crossbow (Repeating Light)|...` — instead;
/// transcribed as an empty list), `RACETYPE:Humanoid`,
/// `RACESUBTYPE:Derro` (the only monster in this subset carrying a
/// `RACESUBTYPE:` token), `CR:3`, `SOURCEPAGE:p.70`.
pub fn derro() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Derro".to_string(),
        challenge_rating: 3.0,
        size: "S".to_string(),
        speed_ft: 20,
        race_type: "Humanoid".to_string(),
        race_subtype: Some("Derro".to_string()),
        source_page: "p.70".to_string(),
        natural_attacks: vec![],
    }
}
