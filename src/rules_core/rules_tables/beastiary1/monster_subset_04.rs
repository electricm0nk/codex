//! Bestiary 1 monster-block subset 04 — continued CR-2 breadth,
//! alphabetically after subset 03's "Cave Fisher".
//!
//! Source: PCGen `pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst`,
//! parsed via `pcgen_import::lst_parser::monster_stat_block` (the same
//! bare-tab-delimited monster parser subset 01 introduced — no widening
//! was needed for this subset either). Every field below is transcribed
//! directly from the cited real `.lst` line's tokens — see each
//! function's doc comment for the exact line number and tokens.
//!
//! **Roster derivation (not a roster correction — subset 4 had no
//! illustrative sample row in `corpus-source-inventory.md` §3.1 to
//! correct, only a placeholder `...` row):** before writing any GREEN
//! code, this cycle enumerated every real, non-`#`-commented,
//! non-`.MOD`/`.COPY=` CR:2 monster stat-block row in `b1_races.lst`
//! directly (34 total rows carry `CR:2`). Excluding parenthetical
//! sub-variant names (e.g. "Ant (Giant)", "Cat (Cheetah)", "Demon
//! (Dretch)") — the same exclusion rule subsets 01-03 all already
//! established, since those name a variant of a broader creature rather
//! than a clean, unambiguous base species — leaves 19 clean CR:2 species
//! names. Subset 03 used the first five alphabetically (Bat Swarm, Boar,
//! Boggard, Bugbear, Cave Fisher). This cycle lands the next five
//! alphabetically: **Choker** (line 70), **Crocodile** (line 83),
//! **Dark Creeper** (line 89), **Iron Cobra** (line 249), **Morlock**
//! (line 297).
//!
//! **Scope boundary** (mirrors `monster_subset_01.rs`/`monster_subset_02.rs`/
//! `monster_subset_03.rs` and every SD-22 Epic 3/4 class chassis module):
//! only fields literally present as tokens on the real row are
//! transcribed. AC, HP, and Fort/Ref/Will saves are PCGen-computed at
//! runtime from the `MONSTERCLASS:` hit-dice table and ability-score
//! modifiers, not literal row tokens — transcribing invented values for
//! them would be exactly the fabricated-data risk `AGENTS.md` and the
//! CRB precedent rule out, so they are deferred to a future ingest
//! slice.

use super::{MonsterStatBlock, NaturalAttack};

/// Source: `b1_races.lst:70`, `CR:2`. Real row tokens: `SIZE:S`,
/// `MOVE:Walk,20,Climb,10` (walk speed transcribed; climb speed out of
/// scope per this module's field-coverage boundary), `RACETYPE:Aberration`,
/// `CR:2`, `SOURCEPAGE:p.45`. No `RACESUBTYPE:` token on this row. The
/// real row carries no `NATURALATTACKS:` token.
///
/// **Tentacle dice are grounded, not transcribed.** The row names the
/// attack via `ABILITY:Internal|AUTOMATIC|Tentacle`, but resolving that
/// reference (to `core_essentials/ce_abilities_race.lst:260`) yields a
/// mechanical marker with no dice. `1d4` is corroborated by aonprd +
/// d20pfsrd (both "2 tentacles +6 (1d4+3 plus grab)"). The published
/// block has tentacles only — the choker has no claw attack, so none is
/// recorded. Full citation in `super::natural_attack_provenance` —
/// **read it before reverting this to an empty list.**
pub fn choker() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Choker".to_string(),
        challenge_rating: 2.0,
        size: "S".to_string(),
        speed_ft: 20,
        race_type: "Aberration".to_string(),
        race_subtype: None,
        source_page: "p.45".to_string(),
        natural_attacks: vec![NaturalAttack { name: "Tentacle".to_string(), damage_dice: "1d4".to_string() }],
    }
}

/// Source: `b1_races.lst:83`, `CR:2`. Real row tokens: `SIZE:L`,
/// `MOVE:Walk,20,Swim,30` (walk speed transcribed; swim speed out of
/// scope per this module's field-coverage boundary), `RACETYPE:Animal`,
/// `CR:2`, `SOURCEPAGE:p.51`. No `RACESUBTYPE:` token on this row, and no
/// `NATURALATTACKS:` token.
///
/// **Mixed provenance — the Tail Slap is a genuine corpus recovery.**
/// This row reaches its attacks through
/// `ABILITY:Internal|AUTOMATIC|Racial Traits ~ Crocodile`, which resolves
/// to `b1_abilities_race.lst:244`. That record's own
/// `ABILITY:Internal|AUTOMATIC|Bite|Crocodile ~ Tail Slap` names both
/// attacks, in that order:
///
/// - **Bite `1d8`** — grounded. The generic `Bite` marker carries no
///   dice; corroborated by aonprd + d20pfsrd ("bite +5 (1d8+4 plus
///   grab)").
/// - **Tail Slap `1d12`** — *transcribed from a real corpus token.*
///   Unlike the generic markers, `Crocodile ~ Tail Slap`
///   (`b1_abilities_race.lst:248`) carries an inline
///   `NATURALATTACKS:Tail Slap,...,*1,1d12`. The published text agrees
///   independently ("tail slap +0 (1d12+2)").
///
/// Full citations in `super::natural_attack_provenance` — **read it
/// before reverting this to an empty list.**
pub fn crocodile() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Crocodile".to_string(),
        challenge_rating: 2.0,
        size: "L".to_string(),
        speed_ft: 20,
        race_type: "Animal".to_string(),
        race_subtype: None,
        source_page: "p.51".to_string(),
        natural_attacks: vec![
            NaturalAttack { name: "Bite".to_string(), damage_dice: "1d8".to_string() },
            NaturalAttack { name: "Tail Slap".to_string(), damage_dice: "1d12".to_string() },
        ],
    }
}

/// Source: `b1_races.lst:89`, `CR:2`. Real row tokens: `SIZE:S`,
/// `MOVE:Walk,30`, `RACETYPE:Humanoid`, `RACESUBTYPE:Dark Folk`, `CR:2`,
/// `SOURCEPAGE:p.53`. The real row carries no `NATURALATTACKS:` token —
/// Dark Creepers fight with weapons and sneak-attack abilities, not a
/// natural weapon.
pub fn dark_creeper() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Dark Creeper".to_string(),
        challenge_rating: 2.0,
        size: "S".to_string(),
        speed_ft: 30,
        race_type: "Humanoid".to_string(),
        race_subtype: Some("Dark Folk".to_string()),
        source_page: "p.53".to_string(),
        natural_attacks: vec![],
    }
}

/// Source: `b1_races.lst:249`, `CR:2`. Real row tokens: `SIZE:S`,
/// `MOVE:Walk,40`, `NATURALATTACKS:Bite,Weapon.Natural.Weapon Group
/// Natural.Melee.Finesseable.Piercing.Slashing.Poison,*1,1d6`,
/// `RACETYPE:Construct`, `CR:2`, `SOURCEPAGE:p.182`. No `RACESUBTYPE:`
/// token on this row.
pub fn iron_cobra() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Iron Cobra".to_string(),
        challenge_rating: 2.0,
        size: "S".to_string(),
        speed_ft: 40,
        race_type: "Construct".to_string(),
        race_subtype: None,
        source_page: "p.182".to_string(),
        natural_attacks: vec![NaturalAttack {
            name: "Bite".to_string(),
            damage_dice: "1d6".to_string(),
        }],
    }
}

/// Source: `b1_races.lst:297`, `CR:2`. Real row tokens: `SIZE:M`,
/// `MOVE:Walk,40,Climb,30` (walk speed transcribed; climb speed out of
/// scope per this module's field-coverage boundary),
/// `NATURALATTACKS:Bite (Primary),Weapon.Natural...,*1,1d4|Bite (With
/// Weapon Attack),Weapon.Natural...,*1,1d4` (pipe-separated two-attack
/// token, same shape subset 02's Octopus already proved), `RACETYPE:
/// Monstrous Humanoid`, `CR:2`, `SOURCEPAGE:p.209`. No `RACESUBTYPE:`
/// token on this row.
pub fn morlock() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Morlock".to_string(),
        challenge_rating: 2.0,
        size: "M".to_string(),
        speed_ft: 40,
        race_type: "Monstrous Humanoid".to_string(),
        race_subtype: None,
        source_page: "p.209".to_string(),
        natural_attacks: vec![
            NaturalAttack {
                name: "Bite (Primary)".to_string(),
                damage_dice: "1d4".to_string(),
            },
            NaturalAttack {
                name: "Bite (With Weapon Attack)".to_string(),
                damage_dice: "1d4".to_string(),
            },
        ],
    }
}
