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
//!
//! **Roster correction for subset 02** (documented in full in
//! `docs/release/SD-22/artifacts/beastiary1/subset_02_cycle_receipt.md`
//! and in `tests/sd22_beastiary1_subset_02_resolves.rs`'s header):
//! `corpus-source-inventory.md` §3.1's illustrative subset-02 sample list
//! ("Gnoll, Hobgoblin, Lizardfolk, Rat Swarm") is wrong three ways —
//! Gnoll and Lizardfolk were already ingested in subset 01; Hobgoblin has
//! no standalone stat-block row in the real corpus at all (a `.MOD`-only
//! override, same shape as subset 01's Goblin/Kobold/Orc); Rat Swarm does
//! have a real standalone row (`b1_races.lst:334`) but its real CR is 2,
//! not 1. Subset 02 ships the real, unused, unambiguous CR-1 monsters
//! this cycle verified directly: Darkmantle, Horse, Hyena, Octopus,
//! Spider Swarm (alphabetical, excluding parenthetical sub-variant names
//! the same way subset 01 did).
//!
//! **CR-band move for subset 03** (documented in full in
//! `docs/release/SD-22/artifacts/beastiary1/subset_03_cycle_receipt.md`
//! and in `tests/sd22_beastiary1_subset_03_resolves.rs`'s header): CR 1
//! is exhausted after subsets 01+02 (only Squid and Troglodyte remain
//! unused among real, non-parenthetical CR:1 monster names — not enough
//! for a five-monster subset), so subset 03 moves to CR 2. Ships the
//! first five real, unambiguous, non-parenthetical CR-2 monsters in
//! alphabetical order: Bat Swarm, Boar, Boggard, Bugbear, Cave Fisher.
//!
//! **Subset 04** (documented in full in
//! `docs/release/SD-22/artifacts/beastiary1/subset_04_cycle_receipt.md`
//! and in `tests/sd22_beastiary1_subset_04_resolves.rs`'s header):
//! continues CR 2 alphabetically after subset 03's "Cave Fisher". Ships
//! the next five real, unambiguous, non-parenthetical CR-2 monsters:
//! Choker, Crocodile, Dark Creeper, Iron Cobra, Morlock.
//!
//! **Subset 05** (documented in full in
//! `docs/release/SD-22/artifacts/beastiary1/subset_05_cycle_receipt.md`
//! and in `tests/sd22_beastiary1_subset_05_resolves.rs`'s header):
//! continues CR 2 alphabetically after subset 04's "Morlock". Ships the
//! next five real, unambiguous, non-parenthetical CR-2 monsters: Rat
//! Swarm, Sahuagin, Shark, Shocker Lizard, Skum.

pub mod monster_subset_01;
pub mod monster_subset_02;
pub mod monster_subset_03;
pub mod monster_subset_04;
pub mod monster_subset_05;

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
/// 01's and subset 02's corrected rosters (see this module's doc
/// comment).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MonsterId {
    Ghoul,
    Gnoll,
    GoblinDog,
    Lizardfolk,
    Wolf,
    Darkmantle,
    Horse,
    Hyena,
    Octopus,
    SpiderSwarm,
    BatSwarm,
    Boar,
    Boggard,
    Bugbear,
    CaveFisher,
    Choker,
    Crocodile,
    DarkCreeper,
    IronCobra,
    Morlock,
    RatSwarm,
    Sahuagin,
    Shark,
    ShockerLizard,
    Skum,
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
        MonsterId::Darkmantle => monster_subset_02::darkmantle(),
        MonsterId::Horse => monster_subset_02::horse(),
        MonsterId::Hyena => monster_subset_02::hyena(),
        MonsterId::Octopus => monster_subset_02::octopus(),
        MonsterId::SpiderSwarm => monster_subset_02::spider_swarm(),
        MonsterId::BatSwarm => monster_subset_03::bat_swarm(),
        MonsterId::Boar => monster_subset_03::boar(),
        MonsterId::Boggard => monster_subset_03::boggard(),
        MonsterId::Bugbear => monster_subset_03::bugbear(),
        MonsterId::CaveFisher => monster_subset_03::cave_fisher(),
        MonsterId::Choker => monster_subset_04::choker(),
        MonsterId::Crocodile => monster_subset_04::crocodile(),
        MonsterId::DarkCreeper => monster_subset_04::dark_creeper(),
        MonsterId::IronCobra => monster_subset_04::iron_cobra(),
        MonsterId::Morlock => monster_subset_04::morlock(),
        MonsterId::RatSwarm => monster_subset_05::rat_swarm(),
        MonsterId::Sahuagin => monster_subset_05::sahuagin(),
        MonsterId::Shark => monster_subset_05::shark(),
        MonsterId::ShockerLizard => monster_subset_05::shocker_lizard(),
        MonsterId::Skum => monster_subset_05::skum(),
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
        "beastiary1:monster:darkmantle" => MonsterId::Darkmantle,
        "beastiary1:monster:horse" => MonsterId::Horse,
        "beastiary1:monster:hyena" => MonsterId::Hyena,
        "beastiary1:monster:octopus" => MonsterId::Octopus,
        "beastiary1:monster:spider_swarm" => MonsterId::SpiderSwarm,
        "beastiary1:monster:bat_swarm" => MonsterId::BatSwarm,
        "beastiary1:monster:boar" => MonsterId::Boar,
        "beastiary1:monster:boggard" => MonsterId::Boggard,
        "beastiary1:monster:bugbear" => MonsterId::Bugbear,
        "beastiary1:monster:cave_fisher" => MonsterId::CaveFisher,
        "beastiary1:monster:choker" => MonsterId::Choker,
        "beastiary1:monster:crocodile" => MonsterId::Crocodile,
        "beastiary1:monster:dark_creeper" => MonsterId::DarkCreeper,
        "beastiary1:monster:iron_cobra" => MonsterId::IronCobra,
        "beastiary1:monster:morlock" => MonsterId::Morlock,
        "beastiary1:monster:rat_swarm" => MonsterId::RatSwarm,
        "beastiary1:monster:sahuagin" => MonsterId::Sahuagin,
        "beastiary1:monster:shark" => MonsterId::Shark,
        "beastiary1:monster:shocker_lizard" => MonsterId::ShockerLizard,
        "beastiary1:monster:skum" => MonsterId::Skum,
        _ => return None,
    };
    monster_resolve(monster_id, rule_set)
}
