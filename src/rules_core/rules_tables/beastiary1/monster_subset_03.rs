//! Bestiary 1 monster-block subset 03 — CR-band move from CR 1 to CR 2.
//!
//! Source: PCGen `pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst`,
//! parsed via `pcgen_import::lst_parser::monster_stat_block` (the same
//! bare-tab-delimited monster parser subset 01 introduced — no widening
//! was needed for this subset either). Every field below is transcribed
//! directly from the cited real `.lst` line's tokens — see each
//! function's doc comment for the exact line number and tokens.
//!
//! **CR-band move (not a roster correction — subset 3 had no
//! illustrative sample row in `corpus-source-inventory.md` §3.1 to
//! correct, only a placeholder `...` row):** before writing any GREEN
//! code, this cycle enumerated every real, non-`#`-commented,
//! non-`.MOD`/`.COPY=` CR:1 monster stat-block row in `b1_races.lst`
//! directly. Excluding parenthetical sub-variant names (e.g. "Ghoul
//! (Ghast)", "Ant (Worker)", "Frog (Giant)") — the same exclusion rule
//! subset 01 and subset 02 both already established, since those name a
//! variant of a broader creature rather than a clean, unambiguous base
//! species — there are only 12 real CR:1 monster names in the whole file:
//! Ghoul, Gnoll, Goblin Dog, Lizardfolk, Wolf (subset 01), Darkmantle,
//! Horse, Hyena, Octopus, Spider Swarm (subset 02), and **Squid** /
//! **Troglodyte** (unused, but only two — not enough for a five-monster
//! subset). CR 1 is exhausted for a clean five-monster subset.
//!
//! Per the loop-instruction's explicit "move to the next CR band if CR 1
//! is exhausted" guidance, this cycle moved to CR 2 and enumerated every
//! real, non-commented, non-parenthetical CR:2 monster stat-block row,
//! picking the first five alphabetically: **Bat Swarm** (line 41),
//! **Boar** (line 49), **Boggard** (line 51), **Bugbear** (line 52),
//! **Cave Fisher** (line 59).
//!
//! **Scope boundary** (mirrors `monster_subset_01.rs`/`monster_subset_02.rs`
//! and every SD-22 Epic 3/4 class chassis module): only fields literally
//! present as tokens on the real row are transcribed. AC, HP, and
//! Fort/Ref/Will saves are PCGen-computed at runtime from the
//! `MONSTERCLASS:` hit-dice table and ability-score modifiers, not
//! literal row tokens — transcribing invented values for them would be
//! exactly the fabricated-data risk `AGENTS.md` and the CRB precedent
//! rule out, so they are deferred to a future ingest slice.

use super::{MonsterStatBlock, NaturalAttack};

/// Source: `b1_races.lst:41`, `CR:2`. Real row tokens: `SIZE:D`,
/// `MOVE:Walk,5,Fly,40` (walk speed transcribed; fly speed out of scope
/// per this module's field-coverage boundary),
/// `NATURALATTACKS:Swarm,Weapon.Natural...,*1,1d6`, `RACETYPE:Animal`,
/// `RACESUBTYPE:Swarm`, `CR:2`, `SOURCEPAGE:p.30`.
pub fn bat_swarm() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Bat Swarm".to_string(),
        challenge_rating: 2.0,
        size: "D".to_string(),
        speed_ft: 5,
        race_type: "Animal".to_string(),
        race_subtype: Some("Swarm".to_string()),
        source_page: "p.30".to_string(),
        natural_attacks: vec![NaturalAttack {
            name: "Swarm".to_string(),
            damage_dice: "1d6".to_string(),
        }],
    }
}

/// Source: `b1_races.lst:49`, `CR:2`. Real row tokens: `SIZE:M`,
/// `MOVE:Walk,40`, `RACETYPE:Animal`, `CR:2`, `SOURCEPAGE:p.36`. The real
/// row carries no `NATURALATTACKS:` token.
///
/// **Gore dice are grounded, not transcribed.** The row names the attack
/// via `ABILITY:Internal|AUTOMATIC|Gore`, but resolving that reference
/// (to `core_essentials/ce_abilities_race.lst:250`) yields a mechanical
/// marker with no dice. `1d8` is corroborated by legacy.aonprd +
/// d20pfsrd (both "gore +4 (1d8+4)"). Full citation in
/// `super::natural_attack_provenance` — **read it before reverting this
/// to an empty list.**
pub fn boar() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Boar".to_string(),
        challenge_rating: 2.0,
        size: "M".to_string(),
        speed_ft: 40,
        race_type: "Animal".to_string(),
        race_subtype: None,
        source_page: "p.36".to_string(),
        natural_attacks: vec![NaturalAttack { name: "Gore".to_string(), damage_dice: "1d8".to_string() }],
    }
}

/// Source: `b1_races.lst:51`, `CR:2`. Real row tokens: `SIZE:M`,
/// `MOVE:Walk,20,Swim,30` (walk speed transcribed; swim speed out of
/// scope per this module's field-coverage boundary),
/// `NATURALATTACKS:Tongue,Weapon.Natural...,*1,0`, `RACETYPE:Humanoid`,
/// `RACESUBTYPE:Boggard`, `CR:2`, `SOURCEPAGE:p.37`.
pub fn boggard() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Boggard".to_string(),
        challenge_rating: 2.0,
        size: "M".to_string(),
        speed_ft: 20,
        race_type: "Humanoid".to_string(),
        race_subtype: Some("Boggard".to_string()),
        source_page: "p.37".to_string(),
        natural_attacks: vec![NaturalAttack {
            name: "Tongue".to_string(),
            damage_dice: "0".to_string(),
        }],
    }
}

/// Source: `b1_races.lst:52`, `CR:2`. Real row tokens: `SIZE:M`,
/// `MOVE:Walk,30`, `RACETYPE:Humanoid`, `RACESUBTYPE:Goblinoid`, `CR:2`,
/// `SOURCEPAGE:p.38`. The real row carries no `NATURALATTACKS:` token
/// (Bugbear fights with manufactured Morningstar/Javelin per
/// `AUTO:WEAPONPROF`, not a natural weapon).
pub fn bugbear() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Bugbear".to_string(),
        challenge_rating: 2.0,
        size: "M".to_string(),
        speed_ft: 30,
        race_type: "Humanoid".to_string(),
        race_subtype: Some("Goblinoid".to_string()),
        source_page: "p.38".to_string(),
        natural_attacks: vec![],
    }
}

/// Source: `b1_races.lst:59`, `CR:2`. Real row tokens: `SIZE:M`,
/// `MOVE:Walk,20,Climb,20` (walk speed transcribed; climb speed out of
/// scope per this module's field-coverage boundary),
/// `NATURALATTACKS:Filament,Weapon.Natural...,*1,0`, `RACETYPE:Vermin`,
/// `CR:2`, `SOURCEPAGE:p.41`. No `RACESUBTYPE:` token on this row.
///
/// **This monster is the one partial case: Filament is transcribed,
/// Claw is grounded.** Filament comes straight from the real
/// `NATURALATTACKS:` token above (including its literal `0` damage —
/// a real attack that deals none) and is unchanged. The row separately
/// names a Claw via `ABILITY:Internal|AUTOMATIC|Claw`, whose target
/// (`core_essentials/ce_abilities_race.lst:251`) carries no dice; `1d4`
/// is corroborated by legacy.aonprd + d20pfsrd (both "2 claws +5
/// (1d4+3)"). Both sources also place Filament on the **Ranged** line,
/// independently matching the real token's own `...Ranged...` type
/// flags. Order is load-bearing — the real corpus token comes first.
/// Full citation in `super::natural_attack_provenance`.
pub fn cave_fisher() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Cave Fisher".to_string(),
        challenge_rating: 2.0,
        size: "M".to_string(),
        speed_ft: 20,
        race_type: "Vermin".to_string(),
        race_subtype: None,
        source_page: "p.41".to_string(),
        natural_attacks: vec![
            NaturalAttack { name: "Filament".to_string(), damage_dice: "0".to_string() },
            NaturalAttack { name: "Claw".to_string(), damage_dice: "1d4".to_string() },
        ],
    }
}
