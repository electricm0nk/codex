//! Bestiary 1 monster-block subset 05 — continued CR-2 breadth,
//! alphabetically after subset 04's "Morlock".
//!
//! Source: PCGen `pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst`,
//! parsed via `pcgen_import::lst_parser::monster_stat_block` (the same
//! bare-tab-delimited monster parser subset 01 introduced — no widening
//! was needed for this subset either). Every field below is transcribed
//! directly from the cited real `.lst` line's tokens — see each
//! function's doc comment for the exact line number and tokens.
//!
//! **Roster derivation (not a roster correction — subset 5 had no
//! illustrative sample row in `corpus-source-inventory.md` §3.1 to
//! correct, only a placeholder `...` row):** before writing any GREEN
//! code, this cycle independently re-enumerated every real,
//! non-`#`-commented, non-`.MOD`/`.COPY=` CR:2 monster stat-block row in
//! `b1_races.lst` directly (34 total rows carry a `CR:2` token).
//! Excluding parenthetical sub-variant names (e.g. "Ant (Giant)", "Cat
//! (Cheetah)", "Demon (Dretch)") — the same exclusion rule subsets 01-04
//! all already established — leaves 19 clean CR:2 species names.
//! Subsets 03+04 used the first ten alphabetically (Bat Swarm, Boar,
//! Boggard, Bugbear, Cave Fisher, Choker, Crocodile, Dark Creeper, Iron
//! Cobra, Morlock). This cycle lands the next five alphabetically:
//! **Rat Swarm** (line 334), **Sahuagin** (line 345), **Shark** (line
//! 360), **Shocker Lizard** (line 362), **Skum** (line 366).
//!
//! **Scope boundary** (mirrors `monster_subset_01.rs` through
//! `monster_subset_04.rs` and every SD-22 Epic 3/4 class chassis
//! module): only fields literally present as tokens on the real row are
//! transcribed. AC, HP, and Fort/Ref/Will saves are PCGen-computed at
//! runtime from the `MONSTERCLASS:` hit-dice table and ability-score
//! modifiers, not literal row tokens — transcribing invented values for
//! them would be exactly the fabricated-data risk `AGENTS.md` and the
//! CRB precedent rule out, so they are deferred to a future ingest
//! slice.
//!
//! **New shape this subset introduces:** Shark's real row (`b1_races.lst:360`)
//! carries `MOVE:Swim,60` with no `Walk,<N>` pair at all — every prior
//! subset's monsters had a Walk pair. The parser's `parse_walk_speed`
//! returns `None` for a `MOVE:` value with no `Walk` component
//! (confirmed directly: `sed -n '360p' b1_races.lst` shows only
//! `MOVE:Swim,60`, no `Walk`). This is transcribed here as `speed_ft: 0`
//! — not an invented value, but the literal fact the real row records no
//! land-movement token, which matches the real, published Shark stat
//! block's "Speed 0 ft., swim 60 ft." Sahuagin and Skum's rows each
//! carry *two* separate `NATURALATTACKS:` tab fields (one plain, one
//! pipe-separated), which the parser accumulates into one combined
//! `natural_attacks` list — a variant of the "multiple tokens on one
//! row" shape, not previously exercised with two full `NATURALATTACKS:`
//! fields at once.

use super::{MonsterStatBlock, NaturalAttack};

/// Source: `b1_races.lst:334`, `CR:2`. Real row tokens: `SIZE:T`,
/// `MOVE:Walk,15,Climb,15,Swim,15` (walk speed transcribed; climb/swim
/// speeds out of scope per this module's field-coverage boundary),
/// `NATURALATTACKS:Swarm,Weapon.Natural...,*1,1d6`, `RACETYPE:Animal`,
/// `RACESUBTYPE:Swarm`, `CR:2`, `SOURCEPAGE:p.232`.
pub fn rat_swarm() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Rat Swarm".to_string(),
        challenge_rating: 2.0,
        size: "T".to_string(),
        speed_ft: 15,
        race_type: "Animal".to_string(),
        race_subtype: Some("Swarm".to_string()),
        source_page: "p.232".to_string(),
        natural_attacks: vec![NaturalAttack {
            name: "Swarm".to_string(),
            damage_dice: "1d6".to_string(),
        }],
    }
}

/// Source: `b1_races.lst:345`, `CR:2`. Real row tokens: `SIZE:M`,
/// `MOVE:Walk,30,Swim,60` (walk speed transcribed; swim speed out of
/// scope), two `NATURALATTACKS:` tab fields —
/// `NATURALATTACKS:Claws,Weapon.Natural...,*2,1d4` and
/// `NATURALATTACKS:Bite (w/o weapon),Weapon.Natural...,*1,1d4|Bite (with
/// weapon),Weapon.Natural...,*1,1d4` — which accumulate into three
/// entries, `RACETYPE:Monstrous Humanoid`, `RACESUBTYPE:Aquatic`,
/// `CR:2`, `SOURCEPAGE:p.239`.
pub fn sahuagin() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Sahuagin".to_string(),
        challenge_rating: 2.0,
        size: "M".to_string(),
        speed_ft: 30,
        race_type: "Monstrous Humanoid".to_string(),
        race_subtype: Some("Aquatic".to_string()),
        source_page: "p.239".to_string(),
        natural_attacks: vec![
            NaturalAttack {
                name: "Claws".to_string(),
                damage_dice: "1d4".to_string(),
            },
            NaturalAttack {
                name: "Bite (w/o weapon)".to_string(),
                damage_dice: "1d4".to_string(),
            },
            NaturalAttack {
                name: "Bite (with weapon)".to_string(),
                damage_dice: "1d4".to_string(),
            },
        ],
    }
}

/// Source: `b1_races.lst:360`, `CR:2`. Real row tokens: `SIZE:L`,
/// `MOVE:Swim,60` — **no `Walk` pair on this row at all** (see this
/// module's header doc comment); transcribed as `speed_ft: 0`, matching
/// the real Shark's published "Speed 0 ft., swim 60 ft." stat line.
/// `NATURALATTACKS:Bite,Weapon.Natural...,*1,1d8`, `RACETYPE:Animal`,
/// `RACESUBTYPE:Aquatic`, `CR:2`, `SOURCEPAGE:p.247`.
pub fn shark() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Shark".to_string(),
        challenge_rating: 2.0,
        size: "L".to_string(),
        speed_ft: 0,
        race_type: "Animal".to_string(),
        race_subtype: Some("Aquatic".to_string()),
        source_page: "p.247".to_string(),
        natural_attacks: vec![NaturalAttack {
            name: "Bite".to_string(),
            damage_dice: "1d8".to_string(),
        }],
    }
}

/// Source: `b1_races.lst:362`, `CR:2`. Real row tokens: `SIZE:S`,
/// `MOVE:Walk,40,Climb,20,Swim,20` (walk speed transcribed; climb/swim
/// speeds out of scope), `NATURALATTACKS:Bite,Weapon.Natural...,*1,1d4`,
/// `RACETYPE:Magical Beast`, `CR:2`, `SOURCEPAGE:p.248`. No
/// `RACESUBTYPE:` token on this row.
pub fn shocker_lizard() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Shocker Lizard".to_string(),
        challenge_rating: 2.0,
        size: "S".to_string(),
        speed_ft: 40,
        race_type: "Magical Beast".to_string(),
        race_subtype: None,
        source_page: "p.248".to_string(),
        natural_attacks: vec![NaturalAttack {
            name: "Bite".to_string(),
            damage_dice: "1d4".to_string(),
        }],
    }
}

/// Source: `b1_races.lst:366`, `CR:2`. Real row tokens: `SIZE:M`,
/// `MOVE:Walk,20,Swim,40` (walk speed transcribed; swim speed out of
/// scope), two `NATURALATTACKS:` tab fields —
/// `NATURALATTACKS:Bite (w/o weapon),Weapon.Natural...,*1,1d6|Bite
/// (w/weapon),Weapon.Natural...,*1,1d6` and `NATURALATTACKS:Claw (w/o
/// weapon),Weapon.Natural...,*1,1d4|Claw (w/weapon),Weapon.Natural...,*1,1d4`
/// — each pipe-separated, accumulating into four entries,
/// `RACETYPE:Monstrous Humanoid`, `RACESUBTYPE:Aquatic`, `CR:2`,
/// `SOURCEPAGE:p.253`.
pub fn skum() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Skum".to_string(),
        challenge_rating: 2.0,
        size: "M".to_string(),
        speed_ft: 20,
        race_type: "Monstrous Humanoid".to_string(),
        race_subtype: Some("Aquatic".to_string()),
        source_page: "p.253".to_string(),
        natural_attacks: vec![
            NaturalAttack {
                name: "Bite (w/o weapon)".to_string(),
                damage_dice: "1d6".to_string(),
            },
            NaturalAttack {
                name: "Bite (w/weapon)".to_string(),
                damage_dice: "1d6".to_string(),
            },
            NaturalAttack {
                name: "Claw (w/o weapon)".to_string(),
                damage_dice: "1d4".to_string(),
            },
            NaturalAttack {
                name: "Claw (w/weapon)".to_string(),
                damage_dice: "1d4".to_string(),
            },
        ],
    }
}
