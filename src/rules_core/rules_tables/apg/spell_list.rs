//! APG shared spell list — bootstrap/representative sample.
//!
//! Source: PCGen `apg_spells.lst`'s "Main Spell List" block (Advanced
//! Player's Guide). Bootstrap coverage: one representative real, active
//! (non-`.MOD`, non-commented) spell record per APG caster class that
//! has one — mirrors `rules_tables::crb::equipment_tables`'s own "one
//! representative item per category" bootstrap philosophy (exhaustive
//! per-class spell-list coverage is later loop work, same as CRB's
//! equipment tables). SD-22 Epic 3 criterion 9.
//!
//! - `Bomber's Eye` — `apg_spells.lst:44`, `CLASSES:Alchemist=1`, `SCHOOL:Transmutation`.
//! - `Burst Bonds` — `apg_spells.lst:53`, `CLASSES:Inquisitor=1`, `SCHOOL:Evocation`.
//! - `Borrow Fortune` — `apg_spells.lst:277`, `CLASSES:Oracle=3`, `SCHOOL:Evocation`.
//! - `Ill Omen` — `apg_spells.lst:150`, `CLASSES:Witch=1`, `SCHOOL:Enchantment`.
//!
//! **Summoner has no active spell record in the real corpus.** The
//! "Summoner Spells - APG" block (`apg_spells.lst:471` onward) is
//! entirely `#`-commented out in the real corpus (every line prefixed
//! `#Acid Pit.MOD`, `#Ant Haul.MOD`, ...) — confirmed by direct
//! inspection, not assumed. That is a real gap in the source data, not
//! an omission in this transcription; a future cycle should revisit if
//! the upstream PCGen corpus is amended, or a base-spell-list
//! association approach is chosen instead of a dedicated APG record.
//!
//! **Cavalier casts no spells** (see `class_cavalier.rs`'s doc comment:
//! no `SPELLSTAT:` token on the real `CLASS:Cavalier` record), so it has
//! no row here by design.

use crate::rules_core::rules_tables::RuleSetId;

/// The subset of PF1 spell schools represented in this bootstrap sample.
/// Not the full 9-school enum (see `rules_tables::crb::spell_list::Pf1SchoolId`
/// for that) — widened as future cycles add spells from other schools.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Pf1SchoolId {
    Transmutation,
    Evocation,
    Enchantment,
}

impl Pf1SchoolId {
    /// Maps the corpus's raw `SCHOOL:` string to this bootstrap enum.
    pub fn from_corpus_str(raw: &str) -> Option<Self> {
        match raw {
            "Transmutation" => Some(Pf1SchoolId::Transmutation),
            "Evocation" => Some(Pf1SchoolId::Evocation),
            "Enchantment" => Some(Pf1SchoolId::Enchantment),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpellListEntry {
    /// The spell's `name` is its identity in `apg_spells.lst`, same as
    /// `rules_tables::crb::spell_list` (no `KEY:` token on spell rows).
    pub key: &'static str,
    pub school: Pf1SchoolId,
    /// Minimum spell level across the real record's `CLASSES:` tag for
    /// the class this entry is cited for.
    pub level: u8,
    pub description: &'static str,
}

pub const SPELL_LIST: &[SpellListEntry] = &[
    SpellListEntry {
        key: "Bomber's Eye",
        school: Pf1SchoolId::Transmutation,
        level: 1,
        description: "Increases thrown weapon range; +1 attack.",
    },
    SpellListEntry {
        key: "Burst Bonds",
        school: Pf1SchoolId::Evocation,
        level: 1,
        description: "(min(CASTERLEVEL,5))d6 damage to restraints.",
    },
    SpellListEntry {
        key: "Borrow Fortune",
        school: Pf1SchoolId::Evocation,
        level: 3,
        description: "Retry attack or check, but do worse on next two.",
    },
    SpellListEntry {
        key: "Ill Omen",
        school: Pf1SchoolId::Enchantment,
        level: 1,
        description: "Target rolls twice for checks and attacks and uses worst roll.",
    },
];

/// SD-24 Epic 6 criterion 6.1 — spell field-coverage audit row. Mirrors
/// `rules_tables::crb::spell_list::SpellFieldCoverage`'s shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpellFieldCoverage {
    /// Records currently in `SPELL_LIST`.
    pub total_records: u32,
    /// Real, active (non-`.MOD`, non-comment) record count in
    /// `apg_spells.lst` (298 distinct spell names).
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
        records_expected: 298,
        has_description: total,
        full_text_verified: 0,
    }
}

/// Resolves an APG spell by name, scoped to `RuleSetId::Apg`. Returns
/// `None` for any other rule set (cross-book invariant, mirrors
/// `apg::class_chassis_resolve`), and `None` when the key isn't in the
/// bootstrap sample above.
pub fn spell_resolve(key: &str, rule_set: RuleSetId) -> Option<&'static SpellListEntry> {
    if rule_set != RuleSetId::Apg {
        return None;
    }
    SPELL_LIST.iter().find(|entry| entry.key == key)
}
