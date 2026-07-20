//! Bestiary 1 monster-block subset 01 — corrected CR-1 roster.
//!
//! Source: PCGen `pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst`,
//! parsed via `pcgen_import::lst_parser::monster_stat_block` (SD-22 Epic 5's
//! new bare-tab-delimited monster parser). Every field below is
//! transcribed directly from the cited real `.lst` line's tokens — see
//! each function's doc comment for the exact line number and tokens.
//!
//! **Roster correction** (see `super`'s module doc comment for the full
//! story): `corpus-source-inventory.md` §3.1's illustrative subset-01
//! sample list is "Goblin, Kobold, Orc, Skeleton, Zombie." None of those
//! five names is a real, standalone CR-1 monster stat-block row in the
//! real corpus file:
//! - `Goblin`, `Kobold`, `Orc` exist in Bestiary 1's data only as
//!   `Goblin.MOD` / `Kobold.MOD` / `Orc.MOD` overrides in the separate
//!   `b1_races_pc.lst` file, layered onto their *playable-race* base
//!   defined under `core_essentials/races/<race>/` — there is no
//!   independent Bestiary 1 monster stat block for them in this corpus.
//! - `Skeleton (Human)` (`b1_races.lst:364`) is CR 1/3, not CR 1.
//! - `Zombie (Human)` (`b1_races.lst:436`) is CR 1/2, not CR 1.
//! - The bare `Skeleton` / `Zombie` rows (`b1_races.lst:439-440`) carry
//!   no `CR:` token at all — they are template-application shims, not
//!   monster stat blocks.
//!
//! This cycle verified the real file directly and found five real CR-1
//! monster stat-block rows with complete, unambiguous, directly
//! transcribable tokens: Ghoul, Gnoll, Goblin Dog, Lizardfolk, Wolf.
//! Subset 01 ships this corrected roster, alphabetically ordered per
//! `corpus-source-inventory.md` §3's own default subset ordering rule.
//!
//! **Scope boundary** (mirrors `rules_tables::crb::class_tables` and
//! every SD-22 Epic 3/4 class chassis module): only fields literally
//! present as tokens on the real row are transcribed. AC, HP, and
//! Fort/Ref/Will saves are PCGen-computed at runtime from the
//! `MONSTERCLASS:` hit-dice table and ability-score modifiers, not
//! literal row tokens — transcribing invented values for them would be
//! exactly the fabricated-data risk `AGENTS.md` and the CRB precedent
//! rule out, so they are deferred to a future ingest slice.

use super::{MonsterStatBlock, NaturalAttack};

/// Source: `b1_races.lst:200`, `CR:1`. Real row tokens:
/// `SIZE:M`, `MOVE:Walk,30`, `NATURALATTACKS:Claw,...,*2,1d6`,
/// `NATURALATTACKS:Bite,...,*1,1d6`, `RACETYPE:Undead`, `CR:1`,
/// `SOURCEPAGE:p.146`.
pub fn ghoul() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Ghoul".to_string(),
        challenge_rating: 1.0,
        size: "M".to_string(),
        speed_ft: 30,
        race_type: "Undead".to_string(),
        race_subtype: None,
        source_page: "p.146".to_string(),
        natural_attacks: vec![
            NaturalAttack { name: "Claw".to_string(), damage_dice: "1d6".to_string() },
            NaturalAttack { name: "Bite".to_string(), damage_dice: "1d6".to_string() },
        ],
    }
}

/// Source: `b1_races.lst:212`, `CR:1`. Real row tokens: `SIZE:M`,
/// `MOVE:Walk,30`, `RACETYPE:Humanoid`, `RACESUBTYPE:Gnoll`, `CR:1`,
/// `SOURCEPAGE:p.155`. The real row carries no `NATURALATTACKS:` token
/// (Gnoll fights with a manufactured Longspear per `AUTO:WEAPONPROF`,
/// not a natural weapon).
pub fn gnoll() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Gnoll".to_string(),
        challenge_rating: 1.0,
        size: "M".to_string(),
        speed_ft: 30,
        race_type: "Humanoid".to_string(),
        race_subtype: Some("Gnoll".to_string()),
        source_page: "p.155".to_string(),
        natural_attacks: vec![],
    }
}

/// Source: `b1_races.lst:213`, `CR:1`. Real row tokens: `SIZE:M`,
/// `MOVE:Walk,50`, `NATURALATTACKS:Bite,...,*1,1d6`, `RACETYPE:Animal`,
/// `CR:1`, `SOURCEPAGE:p.157`.
pub fn goblin_dog() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Goblin Dog".to_string(),
        challenge_rating: 1.0,
        size: "M".to_string(),
        speed_ft: 50,
        race_type: "Animal".to_string(),
        race_subtype: None,
        source_page: "p.157".to_string(),
        natural_attacks: vec![NaturalAttack {
            name: "Bite".to_string(),
            damage_dice: "1d6".to_string(),
        }],
    }
}

/// Source: `b1_races.lst:276`, `CR:1`. Real row tokens: `SIZE:M`,
/// `MOVE:Walk,30,Swim,15` (walk speed transcribed; swim speed out of
/// scope per this module's field-coverage boundary),
/// `NATURALATTACKS:Claw,...,*1,1d4|Bite,...,*1,1d4`,
/// `RACETYPE:Humanoid`, `RACESUBTYPE:Reptilian`, `CR:1`,
/// `SOURCEPAGE:p.195`.
pub fn lizardfolk() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Lizardfolk".to_string(),
        challenge_rating: 1.0,
        size: "M".to_string(),
        speed_ft: 30,
        race_type: "Humanoid".to_string(),
        race_subtype: Some("Reptilian".to_string()),
        source_page: "p.195".to_string(),
        natural_attacks: vec![
            NaturalAttack { name: "Claw".to_string(), damage_dice: "1d4".to_string() },
            NaturalAttack { name: "Bite".to_string(), damage_dice: "1d4".to_string() },
        ],
    }
}

/// Source: `b1_races.lst:414`, `CR:1`. Real row tokens: `SIZE:M`,
/// `MOVE:Walk,50`, `RACETYPE:Animal`, `CR:1`, `SOURCEPAGE:p.278`. The
/// real row carries no `NATURALATTACKS:` token — its Bite attack is
/// granted via `ABILITY:Internal|AUTOMATIC|Bite`, a cross-reference into
/// a shared ability definition this cycle's parser does not resolve
/// (out of scope; see this module's field-coverage boundary note).
pub fn wolf() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Wolf".to_string(),
        challenge_rating: 1.0,
        size: "M".to_string(),
        speed_ft: 50,
        race_type: "Animal".to_string(),
        race_subtype: None,
        source_page: "p.278".to_string(),
        natural_attacks: vec![],
    }
}
