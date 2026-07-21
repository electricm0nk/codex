//! ACG shared spell list — bootstrap/representative sample.
//!
//! Source: PCGen `acg_spells.lst`'s "New Spells" block (Advanced Class
//! Guide, lines 6-143). Bootstrap coverage: one representative real,
//! active (non-`.MOD`, non-commented) spell record per ACG caster class
//! that has one — mirrors `rules_tables::apg::spell_list`'s own "one
//! representative item per category" bootstrap philosophy (exhaustive
//! per-class spell-list coverage is later loop work). SD-22 Epic 4
//! criterion 13.
//!
//! - `Blade Lash` — `acg_spells.lst:27`, `CLASSES:Bloodrager,Magus=1`, `SCHOOL:Transmutation`.
//! - `Air Geyser` — `acg_spells.lst:14`, `CLASSES:Bloodrager,Druid,Magus,Sorcerer,Witch,Wizard=3|Shaman=4`, `SCHOOL:Evocation` (this bootstrap entry cites the `Bloodrager=3` level).
//! - `Beastspeak` — `acg_spells.lst:25`, `CLASSES:Druid,Shaman,Witch=2`, `SCHOOL:Divination`.
//! - `Anti-Incorporeal Shell` — `acg_spells.lst:21`, `CLASSES:Cleric,Shaman,Witch=4`, `SCHOOL:Abjuration`.
//!
//! **Arcanist, Hunter, Investigator, Skald, and Warpriest have no active,
//! full-definition spell record naming them in `acg_spells.lst`.**
//! Confirmed by direct grep of the whole `advanced_class_guide/` tree
//! (`grep -rn "CLASSES:.*<ClassName>" .` for each of the five names
//! returns zero hits, other than Investigator's single `Bomber's
//! Eye.MOD` line — a `.MOD` cross-reference onto APG's own `Bomber's
//! Eye` record, not a full ACG-side spell definition, so excluded per
//! the same non-`.MOD` rule `apg/spell_list.rs` already establishes).
//! Those five classes draw their spells from other books' existing
//! lists (e.g. Arcanist from the arcane Sorcerer/Wizard list) rather
//! than an ACG-specific spell block. This is a real gap in the source
//! data at this bootstrap's scope, not an omission in this
//! transcription — same posture as `apg/spell_list.rs`'s documented
//! Summoner gap.

use crate::rules_core::rules_tables::RuleSetId;

/// The subset of PF1 spell schools represented in this bootstrap sample.
/// Not the full 9-school enum (see `rules_tables::crb::spell_list::Pf1SchoolId`
/// for that) — widened as future cycles add spells from other schools.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Pf1SchoolId {
    Transmutation,
    Evocation,
    Divination,
    Abjuration,
}

impl Pf1SchoolId {
    /// Maps the corpus's raw `SCHOOL:` string to this bootstrap enum.
    pub fn from_corpus_str(raw: &str) -> Option<Self> {
        match raw {
            "Transmutation" => Some(Pf1SchoolId::Transmutation),
            "Evocation" => Some(Pf1SchoolId::Evocation),
            "Divination" => Some(Pf1SchoolId::Divination),
            "Abjuration" => Some(Pf1SchoolId::Abjuration),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpellListEntry {
    /// The spell's `name` is its identity in `acg_spells.lst`, same as
    /// `rules_tables::apg::spell_list` (no `KEY:` token on spell rows).
    pub key: &'static str,
    pub school: Pf1SchoolId,
    /// Minimum spell level across the real record's `CLASSES:` tag for
    /// the class this entry is cited for.
    pub level: u8,
    pub description: &'static str,
}

pub const SPELL_LIST: &[SpellListEntry] = &[
    SpellListEntry {
        key: "Blade Lash",
        school: Pf1SchoolId::Transmutation,
        level: 1,
        description: "Weapon elongates into a whip; attempt a trip maneuver at +10.",
    },
    SpellListEntry {
        key: "Air Geyser",
        school: Pf1SchoolId::Evocation,
        level: 3,
        description: "Blast of air deals 2d6 bludgeoning damage and knocks target upward.",
    },
    SpellListEntry {
        key: "Beastspeak",
        school: Pf1SchoolId::Divination,
        level: 2,
        description: "Speak normally, including verbal spell components, while in animal form.",
    },
    SpellListEntry {
        key: "Anti-Incorporeal Shell",
        school: Pf1SchoolId::Abjuration,
        level: 4,
        description: "Mobile hemispherical field that incorporeal creatures cannot enter.",
    },
];

/// SD-24 Epic 6 criterion 6.1 — spell field-coverage audit row. Mirrors
/// `rules_tables::crb::spell_list::SpellFieldCoverage`'s shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpellFieldCoverage {
    /// Records currently in `SPELL_LIST`.
    pub total_records: u32,
    /// Real, active (non-`.MOD`, non-comment) record count in
    /// `acg_spells.lst` (145 distinct spell names).
    pub records_expected: u32,
    /// Records with a non-empty `description` -- always equals
    /// `total_records` (non-optional field).
    pub has_description: u32,
    /// Records whose ingested `description` is the full SRD/PRD spell
    /// text rather than a short summary. Always 0 today, same finding as
    /// `crb::spell_list::SpellFieldCoverage::full_text_verified`.
    pub full_text_verified: u32,
}

/// Computes this book's spell field-coverage audit row.
pub fn spell_coverage_report() -> SpellFieldCoverage {
    let total = SPELL_LIST.len() as u32;
    SpellFieldCoverage {
        total_records: total,
        records_expected: 145,
        has_description: total,
        full_text_verified: 0,
    }
}

/// Resolves an ACG spell by name, scoped to `RuleSetId::Acg`. Returns
/// `None` for any other rule set (cross-book invariant, mirrors
/// `acg::class_chassis_resolve`), and `None` when the key isn't in the
/// bootstrap sample above.
pub fn spell_resolve(key: &str, rule_set: RuleSetId) -> Option<&'static SpellListEntry> {
    if rule_set != RuleSetId::Acg {
        return None;
    }
    SPELL_LIST.iter().find(|entry| entry.key == key)
}
