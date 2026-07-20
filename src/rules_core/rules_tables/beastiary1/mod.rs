//! Bestiary 1 book-level module. SD-22 Epic 5 content-source ingest —
//! sibling directory to `rules_tables::{apg,acg,crb}` per
//! `SD-19-corpus-aware-compute-seam/decisions.md` §9 and
//! `SD-22-content-source-ingest-and-dm-toolkit/decisions.md` §5.
//!
//! **Roster correction for subset 01** (documented in full in
//! `docs/release/SD-22/artifacts/beastiary1/subset_01_cycle_receipt.md`
//! and in `tests/sd22_beastiary1_subset_01_resolves.rs`'s header):
//! `corpus-source-inventory.md` §3.1's illustrative subset-01 sample
//! list ("Goblin, Kobold, Orc, Skeleton, Zombie") does not correspond to
//! real, standalone CR-1 monster stat-block rows in the real corpus file
//! `pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst` — Goblin,
//! Kobold, and Orc are `.MOD` overrides onto their playable-race base
//! (no independent Bestiary 1 stat block), and Skeleton (Human) / Zombie
//! (Human) are CR 1/3 and CR 1/2, not CR 1. Subset 01 ships the real
//! five CR-1 monsters this cycle verified directly against the corpus:
//! Ghoul, Gnoll, Goblin Dog, Lizardfolk, Wolf (alphabetical, per
//! `corpus-source-inventory.md` §3's own default ordering rule).
//!
//! This module also introduces the new bare-tab-delimited monster
//! stat-block parser at
//! `pcgen_import::lst_parser::monster_stat_block`, which closes the gap
//! `race_ability.rs`'s `RACE:`/`ABILITY:`-only parser left for Epic 5's
//! first cycle (`docs/release/SD-22/progress.md`'s "new parsing code
//! required" blocker, resolved this cycle).

pub mod monster_subset_01;

use crate::rules_core::rules_tables::RuleSetId;

/// A single natural-weapon attack, transcribed from a `NATURALATTACKS:`
/// token on the monster's real `.lst` row.
#[derive(Debug, Clone, PartialEq)]
pub struct NaturalAttack {
    pub name: String,
    pub damage_dice: String,
}

/// A Bestiary 1 monster's chassis data, bounded to the fields literally
/// present as tokens on the real bare monster row (see
/// `pcgen_import::lst_parser::monster_stat_block`'s module doc comment
/// for the scope-boundary rationale: AC/HP/saves are PCGen-computed, not
/// literal row tokens, and are deliberately out of scope for this
/// cycle).
#[derive(Debug, Clone, PartialEq)]
pub struct MonsterStatBlock {
    pub name: String,
    pub challenge_rating: f32,
    pub size: String,
    pub speed_ft: u32,
    pub race_type: String,
    pub race_subtype: Option<String>,
    pub source_page: String,
    pub natural_attacks: Vec<NaturalAttack>,
}

/// Identifies which Bestiary 1 monster a chassis query targets. Subset
/// 01's corrected five-monster roster (see this module's doc comment).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MonsterId {
    Ghoul,
    Gnoll,
    GoblinDog,
    Lizardfolk,
    Wolf,
}

/// Resolves a Bestiary 1 monster's chassis data, scoped to
/// `RuleSetId::Bestiary1`. Returns `None` for any other rule set — a
/// Bestiary 1 monster is never a valid answer for an APG/ACG/CRB query
/// (cross-book invariant, `corpus-source-inventory.md` §3.2).
pub fn monster_resolve(monster_id: MonsterId, rule_set: RuleSetId) -> Option<MonsterStatBlock> {
    if rule_set != RuleSetId::Bestiary1 {
        return None;
    }
    Some(match monster_id {
        MonsterId::Ghoul => monster_subset_01::ghoul(),
        MonsterId::Gnoll => monster_subset_01::gnoll(),
        MonsterId::GoblinDog => monster_subset_01::goblin_dog(),
        MonsterId::Lizardfolk => monster_subset_01::lizardfolk(),
        MonsterId::Wolf => monster_subset_01::wolf(),
    })
}

/// Key-based resolution, mirroring `corpus-source-inventory.md` §3.2's
/// `beastiary1:monster:<lowercase-name>` key shape.
pub fn monster_key_resolve(key: &str, rule_set: RuleSetId) -> Option<MonsterStatBlock> {
    let monster_id = match key {
        "beastiary1:monster:ghoul" => MonsterId::Ghoul,
        "beastiary1:monster:gnoll" => MonsterId::Gnoll,
        "beastiary1:monster:goblin_dog" => MonsterId::GoblinDog,
        "beastiary1:monster:lizardfolk" => MonsterId::Lizardfolk,
        "beastiary1:monster:wolf" => MonsterId::Wolf,
        _ => return None,
    };
    monster_resolve(monster_id, rule_set)
}
