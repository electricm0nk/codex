//! Bestiary 1 monster-block subset 06 — band-exhaustion cleanup subset,
//! closing out the last two unused CR-1 monsters and the last four
//! unused CR-2 monsters in one landing so subset 07 can start CR 3
//! cleanly.
//!
//! Source: PCGen `pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst`,
//! parsed via `pcgen_import::lst_parser::monster_stat_block` (the same
//! bare-tab-delimited monster parser subset 01 introduced — no widening
//! was needed for this subset either). Every field below is transcribed
//! directly from the cited real `.lst` line's tokens — see each
//! function's doc comment for the exact line number and tokens.
//!
//! **Roster derivation (band-exhaustion cleanup, not a straight CR-band
//! continuation):** before writing any GREEN code, this cycle
//! independently re-enumerated every real, non-`#`-commented,
//! non-`.MOD`/`.COPY=` CR:1 and CR:2 monster stat-block row in
//! `b1_races.lst` directly against the live corpus file (not from any
//! prior cycle's summary):
//!
//! - CR:1 — 27 rows carry a `CR:1` token; excluding parenthetical
//!   sub-variant names (the same exclusion rule every prior subset has
//!   used), subsets 01+02 already used every clean CR:1 name except
//!   **Squid** (line 380) and **Troglodyte** (line 390) — 2 remaining,
//!   not enough alone for a five-monster subset.
//! - CR:2 — 34 rows carry a `CR:2` token; excluding parentheticals
//!   leaves 19 clean species names; subsets 03+04+05 used the first
//!   fifteen alphabetically, leaving **Vargouille** (line 401),
//!   **Wolverine** (line 416), **Worg** (line 418), and **Yellow Musk
//!   Creeper** (line 430) — 4 remaining, also not enough alone.
//!
//! Rather than ship an undersized four-monster subset now and strand the
//! two leftover CR-1 monsters for an even-smaller subset later, this
//! cycle combines both remainders into one six-monster subset that fully
//! exhausts CR 1 and CR 2. The combined pool sorts alphabetically as
//! Squid, Troglodyte, Vargouille, Wolverine, Worg, Yellow Musk Creeper —
//! which happens to already be CR-ascending too (both CR-1 leftovers
//! sort before all four CR-2 leftovers), so no further band-interleaving
//! decision was needed.
//!
//! **Scope boundary** (mirrors `monster_subset_01.rs` through
//! `monster_subset_05.rs` and every SD-22 Epic 3/4 class chassis module):
//! only fields literally present as tokens on the real row are
//! transcribed. AC, HP, and Fort/Ref/Will saves are PCGen-computed at
//! runtime from the `MONSTERCLASS:` hit-dice table and ability-score
//! modifiers, not literal row tokens — transcribing invented values for
//! them would be exactly the fabricated-data risk `AGENTS.md` and the
//! CRB precedent rule out, so they are deferred to a future ingest
//! slice.
//!
//! **Shapes this subset exercises (both already have precedent, not new
//! parser surface):** Squid's real row (`b1_races.lst:380`) carries
//! `MOVE:Swim,60,Jet,240` with no `Walk` pair — same "no Walk token"
//! shape subset 05's Shark already proved; transcribed here as
//! `speed_ft: 0`, the literal fact of the token's absence. Vargouille's
//! row (`b1_races.lst:401`) carries `MOVE:Fly,30`, also no `Walk` pair —
//! same shape, also `speed_ft: 0`. Troglodyte's row (`b1_races.lst:390`)
//! carries two separate `NATURALATTACKS:` tab fields, each
//! pipe-separated, accumulating into four entries — same shape subset
//! 05's Sahuagin/Skum already proved. Vargouille, Wolverine, and Worg's
//! rows carry no `NATURALATTACKS:` token at all — same "fights via an
//! `ABILITY:Internal` cross-reference or manufactured weapons instead"
//! shape subset 04's Choker/Crocodile/Dark Creeper already proved.

use super::{MonsterStatBlock, NaturalAttack};

/// Source: `b1_races.lst:380`, `CR:1`. Real row tokens: `SIZE:M`,
/// `MOVE:Swim,60,Jet,240` (no `Walk` pair on this row; transcribed as
/// `speed_ft: 0`), `NATURALATTACKS:Bite,Weapon.Natural...,*1,1d3|Tentacles,
/// Weapon.Natural...,*1,1d4` (pipe-separated, two entries),
/// `RACETYPE:Animal`, `RACESUBTYPE:Aquatic`, `CR:1`, `SOURCEPAGE:p.259`.
pub fn squid() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Squid".to_string(),
        challenge_rating: 1.0,
        size: "M".to_string(),
        speed_ft: 0,
        race_type: "Animal".to_string(),
        race_subtype: Some("Aquatic".to_string()),
        source_page: "p.259".to_string(),
        natural_attacks: vec![
            NaturalAttack { name: "Bite".to_string(), damage_dice: "1d3".to_string() },
            NaturalAttack { name: "Tentacles".to_string(), damage_dice: "1d4".to_string() },
        ],
    }
}

/// Source: `b1_races.lst:390`, `CR:1`. Real row tokens: `SIZE:M`,
/// `MOVE:Walk,30` (walk speed transcribed), two `NATURALATTACKS:` tab
/// fields — `NATURALATTACKS:Claw,Weapon.Natural...,*2,1d4|Claw (with
/// weapon attack),Weapon.Natural...,*1,1d4` and
/// `NATURALATTACKS:Bite,Weapon.Natural...,*1,1d4|Bite (with weapon
/// attack),Weapon.Natural...,*1,1d4` — which accumulate into four
/// entries, `RACETYPE:Humanoid`, `RACESUBTYPE:Reptilian`, `CR:1`,
/// `SOURCEPAGE:p.267`.
pub fn troglodyte() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Troglodyte".to_string(),
        challenge_rating: 1.0,
        size: "M".to_string(),
        speed_ft: 30,
        race_type: "Humanoid".to_string(),
        race_subtype: Some("Reptilian".to_string()),
        source_page: "p.267".to_string(),
        natural_attacks: vec![
            NaturalAttack { name: "Claw".to_string(), damage_dice: "1d4".to_string() },
            NaturalAttack { name: "Claw (with weapon attack)".to_string(), damage_dice: "1d4".to_string() },
            NaturalAttack { name: "Bite".to_string(), damage_dice: "1d4".to_string() },
            NaturalAttack { name: "Bite (with weapon attack)".to_string(), damage_dice: "1d4".to_string() },
        ],
    }
}

/// Source: `b1_races.lst:401`, `CR:2`. Real row tokens: `SIZE:S`,
/// `MOVE:Fly,30` (no `Walk` pair on this row; transcribed as
/// `speed_ft: 0`), no `NATURALATTACKS:` token on this row,
/// `RACETYPE:Outsider`, `RACESUBTYPE:Evil|Extraplanar`, `CR:2`,
/// `SOURCEPAGE:p.272`.
///
/// **Bite dice are grounded, not transcribed.** The row names the attack
/// via `ABILITY:Internal|AUTOMATIC|Bite` but no hop carries dice. `1d4`
/// is corroborated by aonprd + d20pfsrd (both "bite +5 (1d4 plus
/// poison)"). The vargouille's **kiss and shriek are deliberately not
/// recorded**: every source lists them only under Special Attacks, with
/// no damage dice anywhere, so giving them dice would be fabrication.
/// Full citation in `super::natural_attack_provenance` — **read it
/// before reverting this to an empty list.**
pub fn vargouille() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Vargouille".to_string(),
        challenge_rating: 2.0,
        size: "S".to_string(),
        speed_ft: 0,
        race_type: "Outsider".to_string(),
        race_subtype: Some("Evil|Extraplanar".to_string()),
        source_page: "p.272".to_string(),
        natural_attacks: vec![NaturalAttack { name: "Bite".to_string(), damage_dice: "1d4".to_string() }],
    }
}

/// Source: `b1_races.lst:416`, `CR:2`. Real row tokens: `SIZE:M`,
/// `MOVE:Walk,30,Burrow,10,Climb,10` (walk speed transcribed;
/// burrow/climb speeds out of scope), no `NATURALATTACKS:` token on this
/// row, `RACETYPE:Animal`, no `RACESUBTYPE:` token, `CR:2`,
/// `SOURCEPAGE:p.279`.
///
/// **Bite and Claw dice are grounded, not transcribed.** The row names
/// both attacks via `ABILITY:Internal|AUTOMATIC|Bite|Claw` but no hop
/// carries dice. Bite `1d4` and Claw `1d6` are corroborated by aonprd +
/// d20pfsrd (both "2 claws +4 (1d6+2), bite +4 (1d4+2)"). List order
/// follows the real row's own `|Bite|Claw` operand order, per this
/// book's "attack order comes from the real row's token order"
/// convention. Full citation in `super::natural_attack_provenance` —
/// **read it before reverting this to an empty list.**
pub fn wolverine() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Wolverine".to_string(),
        challenge_rating: 2.0,
        size: "M".to_string(),
        speed_ft: 30,
        race_type: "Animal".to_string(),
        race_subtype: None,
        source_page: "p.279".to_string(),
        natural_attacks: vec![
            NaturalAttack { name: "Bite".to_string(), damage_dice: "1d4".to_string() },
            NaturalAttack { name: "Claw".to_string(), damage_dice: "1d6".to_string() },
        ],
    }
}

/// Source: `b1_races.lst:418`, `CR:2`. Real row tokens: `SIZE:M`,
/// `MOVE:Walk,50` (walk speed transcribed), no `NATURALATTACKS:` token on
/// this row, `RACETYPE:Magical Beast`, no `RACESUBTYPE:` token, `CR:2`,
/// `SOURCEPAGE:p.280`.
///
/// **Bite dice are grounded, not transcribed.** The row names the attack
/// via `ABILITY:Internal|AUTOMATIC|...|Bite` but no hop carries dice.
/// `1d6` is corroborated by aonprd + d20pfsrd (both "bite +7 (1d6+4 plus
/// trip)"). Full citation in `super::natural_attack_provenance` — **read
/// it before reverting this to an empty list.**
pub fn worg() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Worg".to_string(),
        challenge_rating: 2.0,
        size: "M".to_string(),
        speed_ft: 50,
        race_type: "Magical Beast".to_string(),
        race_subtype: None,
        source_page: "p.280".to_string(),
        natural_attacks: vec![NaturalAttack { name: "Bite".to_string(), damage_dice: "1d6".to_string() }],
    }
}

/// Source: `b1_races.lst:430`, `CR:2`. Real row tokens: `SIZE:M`,
/// `MOVE:Walk,5` (walk speed transcribed), `NATURALATTACKS:Tendril,
/// Weapon.Natural...,*1,1d4` (single entry), `RACETYPE:Plant`, no
/// `RACESUBTYPE:` token, `CR:2`, `SOURCEPAGE:p.285`.
pub fn yellow_musk_creeper() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Yellow Musk Creeper".to_string(),
        challenge_rating: 2.0,
        size: "M".to_string(),
        speed_ft: 5,
        race_type: "Plant".to_string(),
        race_subtype: None,
        source_page: "p.285".to_string(),
        natural_attacks: vec![NaturalAttack { name: "Tendril".to_string(), damage_dice: "1d4".to_string() }],
    }
}
