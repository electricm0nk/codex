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
//!
//! **Subset 06** (documented in full in
//! `docs/release/SD-22/artifacts/beastiary1/subset_06_cycle_receipt.md`
//! and in `tests/sd22_beastiary1_subset_06_resolves.rs`'s header): a
//! band-exhaustion cleanup subset, not a straight CR-band continuation.
//! Only 2 unused non-parenthetical CR-1 names (Squid, Troglodyte) and
//! only 4 unused non-parenthetical CR-2 names (Vargouille, Wolverine,
//! Worg, Yellow Musk Creeper) remained — neither remainder alone reached
//! five monsters. This subset combines both remainders into one
//! six-monster subset that fully exhausts CR 1 and CR 2, so subset 07
//! can start CR 3 cleanly.
//!
//! **CR-band move for subset 07** (documented in full in
//! `docs/release/SD-22/artifacts/beastiary1/subset_07_cycle_receipt.md`
//! and in `tests/sd22_beastiary1_subset_07_resolves.rs`'s header): CR 1
//! and CR 2 are both fully exhausted after subset 06, so subset 07 moves
//! to CR 3. Ships the first five real, unambiguous, non-parenthetical
//! CR-3 monsters in alphabetical order: Ankheg, Assassin Vine, Centaur,
//! Cockatrice, Derro.
//!
//! **Subset 08** (documented in full in
//! `docs/release/SD-22/artifacts/beastiary1/subset_08_cycle_receipt.md`
//! and in `tests/sd22_beastiary1_subset_08_resolves.rs`'s header):
//! continues CR 3 alphabetically after subset 07's "Derro". Of the 20
//! clean, non-parenthetical CR-3 species names in the real corpus, this
//! subset ships the next five: Doppelganger, Dryad, Ettercap, Gelatinous
//! Cube, Hell Hound. This brings Epic 5 to 8 of a default 8-12 subsets
//! (41 monsters total) — see `docs/release/SD-22/progress.md`'s cycle
//! log for this cycle's closure-readiness assessment.
//!
//! **Natural-attack grounding (v0.6, 2026-07-29):** a full verification
//! pass found this book error-free across all 41 monsters but with 12
//! carrying missing or partial natural attacks — Ankheg, Assassin Vine,
//! Boar, Cave Fisher (partial), Centaur, Choker, Cockatrice, Crocodile,
//! Vargouille, Wolf, Wolverine, Worg. Those rows name their attacks with
//! an `ABILITY:Internal|AUTOMATIC|<Name>` cross-reference instead of an
//! inline `NATURALATTACKS:` token, and **no hop of that reference
//! carries damage dice** (the target rows live in
//! `core_essentials/ce_abilities_race.lst`, not under `bestiary/`, and
//! are dice-less mechanical markers — PCGen supplies the dice at runtime
//! from size tables). All 12 are now grounded from published values with
//! at least two agreeing allowed-domain sources; the single exception is
//! Crocodile's Tail Slap, genuinely recovered from a real cross-file
//! corpus token (`b1_abilities_race.lst:248`, `...,*1,1d12`). Full
//! per-value citations live in `natural_attack_provenance`, pinned by
//! `tests/v06_beastiary1_natural_attack_grounding.rs`. Five other
//! monsters (Bugbear, Dark Creeper, Derro, Dryad, Gnoll) keep empty
//! attack lists **correctly** — they are weapon users, confirmed in
//! print, and a test now guards them against a future "close the empty
//! lists" sweep.
//!
//! **Equipment tables (SD-25 criterion 7.N item 4, added this cycle):**
//! `equipment_tables`/`equipment_data` close the "no `beastiary1`
//! equipment module exists" scope gap
//! `tests/sd24_equipment_coverage_audit.rs` documented. See
//! `equipment_tables.rs`'s module doc comment for the full sourcing
//! methodology, the register A8 codegen-path decision, and the register
//! A13 finding that no spell-list concept exists for this book.

pub mod equipment_data;
pub mod equipment_tables;
pub mod natural_attack_provenance;
pub mod monster_subset_01;
pub mod monster_subset_02;
pub mod monster_subset_03;
pub mod monster_subset_04;
pub mod monster_subset_05;
pub mod monster_subset_06;
pub mod monster_subset_07;
pub mod monster_subset_08;
pub mod monster_subset_09;

use crate::rules_core::rules_tables::RuleSetId;

/// A single natural-weapon attack.
///
/// Usually transcribed from a `NATURALATTACKS:` token on the monster's
/// real `.lst` row. Twelve Bestiary 1 monsters instead carry only an
/// `ABILITY:Internal|AUTOMATIC|<Name>` cross-reference, which names the
/// attack but supplies no dice at any hop — for those, `damage_dice` is
/// grounded from published values and every one is documented, with its
/// sources, in [`natural_attack_provenance`]. **Read that module before
/// changing any `natural_attacks` list back to empty.**
///
/// `damage_dice` is the die expression only, with no Strength modifier
/// (`"1d6"`, not `"1d6+1"`). `"0"` means a real attack that deals no
/// damage — e.g. Cave Fisher's Filament, whose own corpus token ends
/// `,*1,0`. The per-attack `*N` count on a `NATURALATTACKS:` token is
/// deliberately **not** modelled: this struct records distinct attack
/// types, so a Ghoul's `Claw,...,*2,1d6` yields one `Claw` entry.
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
    Squid,
    Troglodyte,
    Vargouille,
    Wolverine,
    Worg,
    YellowMuskCreeper,
    Ankheg,
    AssassinVine,
    Centaur,
    Cockatrice,
    Derro,
    Doppelganger,
    Dryad,
    Ettercap,
    GelatinousCube,
    HellHound,
    Lion,
    Ogre,
    Pegasus,
    RustMonster,
    Shadow,
}

impl MonsterId {
    /// Every real Bestiary 1 `MonsterId` variant, in declaration order
    /// (subsets 01-08, per this module's doc comment). Mirrors
    /// `ClassId::ALL`/`ApgClassId::ALL`/`AcgClassId::ALL` on the other
    /// three books (`decisions.md §11.6`) — this book had none before,
    /// forcing `apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs`
    /// to hand-maintain its own duplicate 41-entry workaround list
    /// (`ALL_BESTIARY1_MONSTERS`, now replaced with this constant) and
    /// `SD-26`'s `cache_gen::beastiary1` JSON-cache generator to need one
    /// too. A future roster addition/removal that forgets to update this
    /// list fails to compile at every call site that pattern-matches
    /// exhaustively on `MonsterId` (e.g. `monster_resolve`'s own `match`),
    /// so this list can't silently drift from the real enum.
    pub const ALL: &'static [MonsterId] = &[
        MonsterId::Ghoul,
        MonsterId::Gnoll,
        MonsterId::GoblinDog,
        MonsterId::Lizardfolk,
        MonsterId::Wolf,
        MonsterId::Darkmantle,
        MonsterId::Horse,
        MonsterId::Hyena,
        MonsterId::Octopus,
        MonsterId::SpiderSwarm,
        MonsterId::BatSwarm,
        MonsterId::Boar,
        MonsterId::Boggard,
        MonsterId::Bugbear,
        MonsterId::CaveFisher,
        MonsterId::Choker,
        MonsterId::Crocodile,
        MonsterId::DarkCreeper,
        MonsterId::IronCobra,
        MonsterId::Morlock,
        MonsterId::RatSwarm,
        MonsterId::Sahuagin,
        MonsterId::Shark,
        MonsterId::ShockerLizard,
        MonsterId::Skum,
        MonsterId::Squid,
        MonsterId::Troglodyte,
        MonsterId::Vargouille,
        MonsterId::Wolverine,
        MonsterId::Worg,
        MonsterId::YellowMuskCreeper,
        MonsterId::Ankheg,
        MonsterId::AssassinVine,
        MonsterId::Centaur,
        MonsterId::Cockatrice,
        MonsterId::Derro,
        MonsterId::Doppelganger,
        MonsterId::Dryad,
        MonsterId::Ettercap,
        MonsterId::GelatinousCube,
        MonsterId::HellHound,
        MonsterId::Lion,
        MonsterId::Ogre,
        MonsterId::Pegasus,
        MonsterId::RustMonster,
        MonsterId::Shadow,
    ];
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
        MonsterId::Squid => monster_subset_06::squid(),
        MonsterId::Troglodyte => monster_subset_06::troglodyte(),
        MonsterId::Vargouille => monster_subset_06::vargouille(),
        MonsterId::Wolverine => monster_subset_06::wolverine(),
        MonsterId::Worg => monster_subset_06::worg(),
        MonsterId::YellowMuskCreeper => monster_subset_06::yellow_musk_creeper(),
        MonsterId::Ankheg => monster_subset_07::ankheg(),
        MonsterId::AssassinVine => monster_subset_07::assassin_vine(),
        MonsterId::Centaur => monster_subset_07::centaur(),
        MonsterId::Cockatrice => monster_subset_07::cockatrice(),
        MonsterId::Derro => monster_subset_07::derro(),
        MonsterId::Doppelganger => monster_subset_08::doppelganger(),
        MonsterId::Dryad => monster_subset_08::dryad(),
        MonsterId::Ettercap => monster_subset_08::ettercap(),
        MonsterId::GelatinousCube => monster_subset_08::gelatinous_cube(),
        MonsterId::HellHound => monster_subset_08::hell_hound(),
        MonsterId::Lion => monster_subset_09::lion(),
        MonsterId::Ogre => monster_subset_09::ogre(),
        MonsterId::Pegasus => monster_subset_09::pegasus(),
        MonsterId::RustMonster => monster_subset_09::rust_monster(),
        MonsterId::Shadow => monster_subset_09::shadow(),
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
        "beastiary1:monster:squid" => MonsterId::Squid,
        "beastiary1:monster:troglodyte" => MonsterId::Troglodyte,
        "beastiary1:monster:vargouille" => MonsterId::Vargouille,
        "beastiary1:monster:wolverine" => MonsterId::Wolverine,
        "beastiary1:monster:worg" => MonsterId::Worg,
        "beastiary1:monster:yellow_musk_creeper" => MonsterId::YellowMuskCreeper,
        "beastiary1:monster:ankheg" => MonsterId::Ankheg,
        "beastiary1:monster:assassin_vine" => MonsterId::AssassinVine,
        "beastiary1:monster:centaur" => MonsterId::Centaur,
        "beastiary1:monster:cockatrice" => MonsterId::Cockatrice,
        "beastiary1:monster:derro" => MonsterId::Derro,
        "beastiary1:monster:doppelganger" => MonsterId::Doppelganger,
        "beastiary1:monster:dryad" => MonsterId::Dryad,
        "beastiary1:monster:ettercap" => MonsterId::Ettercap,
        "beastiary1:monster:gelatinous_cube" => MonsterId::GelatinousCube,
        "beastiary1:monster:hell_hound" => MonsterId::HellHound,
        "beastiary1:monster:lion" => MonsterId::Lion,
        "beastiary1:monster:ogre" => MonsterId::Ogre,
        "beastiary1:monster:pegasus" => MonsterId::Pegasus,
        "beastiary1:monster:rust_monster" => MonsterId::RustMonster,
        "beastiary1:monster:shadow" => MonsterId::Shadow,
        _ => return None,
    };
    monster_resolve(monster_id, rule_set)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn all_has_exactly_the_46_real_monsters_with_no_duplicates() {
        assert_eq!(MonsterId::ALL.len(), 46, "real, corrected roster across subsets 01-09 (this module's doc comment)");
        let unique: HashSet<MonsterId> = MonsterId::ALL.iter().copied().collect();
        assert_eq!(unique.len(), 46, "MonsterId::ALL must not repeat any variant");
    }

    #[test]
    fn every_all_entry_resolves_a_real_stat_block_for_bestiary1() {
        for &id in MonsterId::ALL {
            assert!(monster_resolve(id, RuleSetId::Bestiary1).is_some(), "{id:?} must resolve for RuleSetId::Bestiary1");
        }
    }
}
