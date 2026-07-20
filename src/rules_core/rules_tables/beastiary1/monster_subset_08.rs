//! Bestiary 1 monster-block subset 08 — CR 3, continued (subset 07's
//! first five CR-3 monsters alphabetically; this subset ships the next
//! five).
//!
//! Source: PCGen `pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst`,
//! parsed via `pcgen_import::lst_parser::monster_stat_block` (the same
//! bare-tab-delimited monster parser subset 01 introduced — no widening
//! was needed for this subset either). Every field below is transcribed
//! directly from the cited real `.lst` line's tokens — see each
//! function's doc comment for the exact line number and tokens.
//!
//! **Roster derivation (CR-band continuation):** before writing any
//! GREEN code, this cycle independently re-enumerated every real,
//! non-`#`-commented, non-`.MOD`/`.COPY=` CR:3 monster stat-block row in
//! `b1_races.lst` directly against the live corpus file: 20 clean,
//! standalone CR:3 species names exist total (same count subset 07's
//! cycle found). Subset 07 shipped the first five alphabetically
//! (Ankheg, Assassin Vine, Centaur, Cockatrice, Derro); this subset
//! ships the next five: **Doppelganger** (line 127), **Dryad** (line
//! 141), **Ettercap** (line 175), **Gelatinous Cube** (line 189),
//! **Hell Hound** (line 230). `Hell Hound (Nessian)` (line 231, CR 9) is
//! a parenthetical sub-variant and excluded, same rule every prior
//! subset has used.
//!
//! **Scope boundary** (mirrors `monster_subset_01.rs` through
//! `monster_subset_07.rs` and every SD-22 Epic 3/4 class chassis
//! module): only fields literally present as tokens on the real row are
//! transcribed. AC, HP, and Fort/Ref/Will saves are PCGen-computed at
//! runtime from the `MONSTERCLASS:` hit-dice table and ability-score
//! modifiers, not literal row tokens — transcribing invented values for
//! them would be exactly the fabricated-data risk `AGENTS.md` and the
//! CRB precedent rule out, so they are deferred to a future ingest
//! slice.
//!
//! **Shape this subset exercises (precedent already established, not
//! new parser surface):** Doppelganger and Hell Hound both carry a
//! `RACESUBTYPE:` token (Doppelganger single-value `Shapechanger`; Hell
//! Hound pipe-separated multi-value `Evil|Extraplanar|Fire|Lawful`,
//! transcribed verbatim as the literal token string, same as every
//! other `RACESUBTYPE:` field in this ingest). Ettercap carries two
//! separate `NATURALATTACKS:` tokens on one row (Bite, then Claw) — the
//! same multi-token-per-row shape `monster_stat_block.rs`'s own
//! `parses_pipe_separated_natural_attacks_in_one_token` unit test
//! already proves the parser handles, just split across two tab fields
//! instead of one pipe-separated field. Ettercap's row also carries
//! `MOVE:Walk,30,Climb,30` — walk speed transcribed, climb speed out of
//! scope, same shape subset 07's Cockatrice (`MOVE:Walk,20,Fly,60`)
//! already proved.

use super::{MonsterStatBlock, NaturalAttack};

/// Source: `b1_races.lst:127`, `CR:3`. Real row tokens: `SIZE:M`,
/// `MOVE:Walk,30`, `NATURALATTACKS:Claw,...,*2,1d8` (transcribed as one
/// `Claw`/`1d8` entry), `RACETYPE:Monstrous Humanoid`,
/// `RACESUBTYPE:Shapechanger`, `CR:3`, `SOURCEPAGE:p.89`.
pub fn doppelganger() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Doppelganger".to_string(),
        challenge_rating: 3.0,
        size: "M".to_string(),
        speed_ft: 30,
        race_type: "Monstrous Humanoid".to_string(),
        race_subtype: Some("Shapechanger".to_string()),
        source_page: "p.89".to_string(),
        natural_attacks: vec![NaturalAttack { name: "Claw".to_string(), damage_dice: "1d8".to_string() }],
    }
}

/// Source: `b1_races.lst:141`, `CR:3`. Real row tokens: `SIZE:M`,
/// `MOVE:Walk,30`, no `NATURALATTACKS:` token (fights via
/// `ABILITY:Special Ability` cross-references and innate spells
/// instead — transcribed as an empty list), `RACETYPE:Fey`, no
/// `RACESUBTYPE:` token, `CR:3`, `SOURCEPAGE:p.116`.
pub fn dryad() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Dryad".to_string(),
        challenge_rating: 3.0,
        size: "M".to_string(),
        speed_ft: 30,
        race_type: "Fey".to_string(),
        race_subtype: None,
        source_page: "p.116".to_string(),
        natural_attacks: vec![],
    }
}

/// Source: `b1_races.lst:175`, `CR:3`. Real row tokens: `SIZE:M`,
/// `MOVE:Walk,30,Climb,30` (walk speed transcribed; climb speed out of
/// scope), two `NATURALATTACKS:` tokens on the row —
/// `Bite,...,*1,1d6` and `Claw,...,*2,1d4` (transcribed as two
/// entries, in the row's own order), `RACETYPE:Aberration`, no
/// `RACESUBTYPE:` token, `CR:3`, `SOURCEPAGE:p.129`.
pub fn ettercap() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Ettercap".to_string(),
        challenge_rating: 3.0,
        size: "M".to_string(),
        speed_ft: 30,
        race_type: "Aberration".to_string(),
        race_subtype: None,
        source_page: "p.129".to_string(),
        natural_attacks: vec![
            NaturalAttack { name: "Bite".to_string(), damage_dice: "1d6".to_string() },
            NaturalAttack { name: "Claw".to_string(), damage_dice: "1d4".to_string() },
        ],
    }
}

/// Source: `b1_races.lst:189`, `CR:3`. Real row tokens: `SIZE:L`,
/// `MOVE:Walk,15`, `NATURALATTACKS:Slam,...,*1,1d6` (transcribed as one
/// `Slam`/`1d6` entry), `RACETYPE:Ooze`, no `RACESUBTYPE:` token,
/// `CR:3`, `SOURCEPAGE:p.138`.
pub fn gelatinous_cube() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Gelatinous Cube".to_string(),
        challenge_rating: 3.0,
        size: "L".to_string(),
        speed_ft: 15,
        race_type: "Ooze".to_string(),
        race_subtype: None,
        source_page: "p.138".to_string(),
        natural_attacks: vec![NaturalAttack { name: "Slam".to_string(), damage_dice: "1d6".to_string() }],
    }
}

/// Source: `b1_races.lst:230`, `CR:3`. Real row tokens: `SIZE:M`,
/// `MOVE:Walk,40`, `NATURALATTACKS:Bite,...,*1,1d8` (transcribed as one
/// `Bite`/`1d8` entry), `RACETYPE:Outsider`,
/// `RACESUBTYPE:Evil|Extraplanar|Fire|Lawful` (transcribed verbatim as
/// the literal pipe-separated token string), `CR:3`,
/// `SOURCEPAGE:p.173`. `Hell Hound (Nessian)` (line 231, CR 9) is a
/// distinct, parenthetical, higher-CR sub-variant and out of scope for
/// this row.
pub fn hell_hound() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Hell Hound".to_string(),
        challenge_rating: 3.0,
        size: "M".to_string(),
        speed_ft: 40,
        race_type: "Outsider".to_string(),
        race_subtype: Some("Evil|Extraplanar|Fire|Lawful".to_string()),
        source_page: "p.173".to_string(),
        natural_attacks: vec![NaturalAttack { name: "Bite".to_string(), damage_dice: "1d8".to_string() }],
    }
}
