//! Bestiary 1 monster-block subset 09 — CR 3, continued (subsets 07-08
//! shipped the first ten of 20 CR:3 monsters alphabetically; this subset
//! ships the next five).
//!
//! Source: PCGen `pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst`,
//! parsed via `pcgen_import::lst_parser::monster_stat_block` (the same
//! bare-tab-delimited monster parser subset 01 introduced — no widening
//! was needed for this subset either). Every field below is transcribed
//! directly from the cited real `.lst` line's tokens.
//!
//! **Roster derivation (CR-band continuation), re-derived directly
//! against the live corpus rather than transcribed from subset 08's own
//! doc comment (`loop-instruction.md` step 1b):** `grep`-equivalent scan
//! of every real, non-`#`-commented, non-`.MOD` `CR:3` row in
//! `b1_races.lst`, excluding parenthetical sub-variants, confirms **20**
//! clean CR:3 species names, matching subsets 07/08's own count.
//! Subsets 07-08 shipped the first ten alphabetically (Ankheg, Assassin
//! Vine, Centaur, Cockatrice, Derro, Doppelganger, Dryad, Ettercap,
//! Gelatinous Cube, Hell Hound); this subset ships the next five:
//! **Lion** (line 272), **Ogre** (line 316), **Pegasus** (line 323),
//! **Rust Monster** (line 341), **Shadow** (line 357). The remaining
//! five (Unicorn, Violet Fungus, Wasp Swarm, Wight, Yeth Hound) are not
//! part of this subset.
//!
//! **Scope boundary** (mirrors `monster_subset_01.rs` through
//! `monster_subset_08.rs`): only fields literally present as tokens on
//! the real row are transcribed. AC, HP, and Fort/Ref/Will saves are
//! PCGen-computed at runtime from the `MONSTERCLASS:` hit-dice table and
//! ability-score modifiers, not literal row tokens, and are deferred to
//! a future ingest slice, same as every prior subset.
//!
//! **New shape this subset exercises: a monster with no `MOVE:Walk,...`
//! token at all.** Every prior subset's monsters carry a `Walk` speed
//! (transcribed) alongside any other movement mode (Fly/Climb/Swim,
//! dropped per subset 08's Ettercap/Cockatrice precedent). Shadow's row
//! carries only `MOVE:Fly,40` — no `Walk` component. `speed_ft`'s
//! established meaning across every prior subset is specifically "walk
//! speed, transcribed"; fabricating a walk speed from the Fly number
//! would invent a fact the row does not state, so Shadow's `speed_ft` is
//! `0` (no walk movement) rather than a guessed value. The real `Fly,40`
//! token is out of scope for the same reason Ettercap's `Climb` and
//! Cockatrice's `Fly` were: this `MonsterStatBlock` shape has no field
//! for a non-walk movement mode yet. Shadow is Bestiary 1's **fourth**
//! `speed_ft: 0` record overall (`monster_catalog.rs`'s land-speed-zero
//! test, updated alongside this subset, names all four with their
//! evidence).

use super::{MonsterStatBlock, NaturalAttack};

/// Source: `b1_races.lst:272`, `CR:3`. Real row tokens: `SIZE:L`,
/// `MOVE:Walk,40`, two `NATURALATTACKS:` tokens (`Bite,...,*1,1d8` and
/// `Claw,...,*2,1d4`, each transcribed as one name/damage-dice entry per
/// subset 08's Ettercap precedent for multi-token rows), `RACETYPE:Animal`,
/// no `RACESUBTYPE:`, `CR:3`, `SOURCEPAGE:p.193`.
pub fn lion() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Lion".to_string(),
        challenge_rating: 3.0,
        size: "L".to_string(),
        speed_ft: 40,
        race_type: "Animal".to_string(),
        race_subtype: None,
        source_page: "p.193".to_string(),
        natural_attacks: vec![
            NaturalAttack { name: "Bite".to_string(), damage_dice: "1d8".to_string() },
            NaturalAttack { name: "Claw".to_string(), damage_dice: "1d4".to_string() },
        ],
    }
}

/// Source: `b1_races.lst:316`, `CR:3`. Real row tokens: `SIZE:L`,
/// `MOVE:Walk,40`, no `NATURALATTACKS:` token (fights with weapons --
/// `AUTO:WEAPONPROF:Greatclub|Javelin` -- rather than natural attacks,
/// same shape subset 08's Dryad already proved), `RACETYPE:Humanoid`,
/// `RACESUBTYPE:Giant`, `CR:3`, `SOURCEPAGE:p.220`.
pub fn ogre() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Ogre".to_string(),
        challenge_rating: 3.0,
        size: "L".to_string(),
        speed_ft: 40,
        race_type: "Humanoid".to_string(),
        race_subtype: Some("Giant".to_string()),
        source_page: "p.220".to_string(),
        natural_attacks: vec![],
    }
}

/// Source: `b1_races.lst:323`, `CR:3`. Real row tokens: `SIZE:L`,
/// `MOVE:Walk,60,Fly,120` (Walk transcribed; Fly out of scope, same rule
/// subset 08's Cockatrice already proved), one pipe-separated
/// `NATURALATTACKS:` token (`Bite,...,*1,1d3|Hoof,...,*2,1d6`,
/// transcribed as two entries per the parser's own pipe-separated-token
/// precedent), `RACETYPE:Magical Beast`, no `RACESUBTYPE:`, `CR:3`,
/// `SOURCEPAGE:p.225`.
pub fn pegasus() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Pegasus".to_string(),
        challenge_rating: 3.0,
        size: "L".to_string(),
        speed_ft: 60,
        race_type: "Magical Beast".to_string(),
        race_subtype: None,
        source_page: "p.225".to_string(),
        natural_attacks: vec![
            NaturalAttack { name: "Bite".to_string(), damage_dice: "1d3".to_string() },
            NaturalAttack { name: "Hoof".to_string(), damage_dice: "1d6".to_string() },
        ],
    }
}

/// Source: `b1_races.lst:341`, `CR:3`. Real row tokens: `SIZE:M`,
/// `MOVE:Walk,40,Climb,10` (Walk transcribed; Climb out of scope, same
/// rule subset 08's Ettercap already proved), one pipe-separated
/// `NATURALATTACKS:` token (`Bite,...,*1,1d3|Antennae,...,*1,0` --
/// Antennae's real damage-dice token is literally `0`, its rust effect
/// being a special ability rather than weapon damage, transcribed
/// verbatim rather than omitted or guessed), `RACETYPE:Aberration`, no
/// `RACESUBTYPE:`, `CR:3`, `SOURCEPAGE:p.238`.
pub fn rust_monster() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Rust Monster".to_string(),
        challenge_rating: 3.0,
        size: "M".to_string(),
        speed_ft: 40,
        race_type: "Aberration".to_string(),
        race_subtype: None,
        source_page: "p.238".to_string(),
        natural_attacks: vec![
            NaturalAttack { name: "Bite".to_string(), damage_dice: "1d3".to_string() },
            NaturalAttack { name: "Antennae".to_string(), damage_dice: "0".to_string() },
        ],
    }
}

/// Source: `b1_races.lst:357`, `CR:3`. Real row tokens: `SIZE:M`,
/// `MOVE:Fly,40` -- no `Walk` component at all, this subset's new shape
/// (see module doc comment); `speed_ft` is `0` rather than a guessed
/// value, and the real `Fly,40` token is out of scope for the same
/// reason every other non-walk movement mode has been out of scope since
/// subset 07. One `NATURALATTACKS:` token
/// (`Incorporeal Touch,...,*1,1d6`), `RACETYPE:Undead`,
/// `RACESUBTYPE:Incorporeal`, `CR:3`, `SOURCEPAGE:p.245`.
pub fn shadow() -> MonsterStatBlock {
    MonsterStatBlock {
        name: "Shadow".to_string(),
        challenge_rating: 3.0,
        size: "M".to_string(),
        speed_ft: 0,
        race_type: "Undead".to_string(),
        race_subtype: Some("Incorporeal".to_string()),
        source_page: "p.245".to_string(),
        natural_attacks: vec![NaturalAttack { name: "Incorporeal Touch".to_string(), damage_dice: "1d6".to_string() }],
    }
}
