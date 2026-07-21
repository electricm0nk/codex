//! APG (Advanced Player's Guide) book-level module. SD-22 Epic 3
//! content-source ingest — sibling directory to `rules_tables::crb` per
//! `SD-19-corpus-aware-compute-seam/decisions.md` §9. Alchemist is the
//! first class ingested, Cavalier the second, Inquisitor the third,
//! Oracle the fourth, Summoner the fifth, Witch the sixth and last
//! (`decisions.md §5`'s corrected real-LST-corpus sourcing, corrected
//! 2026-07-19). Gunslinger and Magus
//! are not real
//! APG content and are permanently excluded from this roster (corrected
//! 2026-07-19, `corpus-source-inventory.md §1`) — the real PCGen corpus
//! has no `CLASS:Gunslinger` or `CLASS:Magus` record anywhere under
//! `advanced_players_guide/`; both live in `ultimate_combat/uc_classes.lst`
//! and `ultimate_magic/um_classes.lst` respectively, books
//! `decisions.md §1` explicitly excludes from SD-22.

pub mod class_alchemist;
pub mod class_cavalier;
pub mod class_inquisitor;
pub mod class_oracle;
pub mod class_summoner;
pub mod class_witch;
pub mod equipment_data;
pub mod equipment_tables;
pub mod spell_list;

use crate::rules_core::rules_tables::RuleSetId;

/// One APG class's chassis-table row: level, BAB, and the three saves.
/// Shared shape across every per-class module in this directory so
/// `class_chassis_resolve` can return a single type regardless of
/// which class was queried.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassTableRow {
    pub level: u8,
    pub base_attack_bonus: i16,
    pub fort_save: i16,
    pub ref_save: i16,
    pub will_save: i16,
}

/// Identifies which APG class a chassis-table query targets. All six
/// real APG classes now have a variant (Gunslinger and Magus are not
/// real APG content in the PCGen corpus, see this module's doc
/// comment).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApgClassId {
    Alchemist,
    Cavalier,
    Inquisitor,
    Oracle,
    Summoner,
    Witch,
}

impl ApgClassId {
    /// All six real APG classes, in the same ingest order as this module's
    /// doc comment (Alchemist, Cavalier, Inquisitor, Oracle, Summoner,
    /// Witch) — mirrors `rules_tables::crb::class_tables::ClassId::ALL`'s
    /// shape so SD-24 Epic 4's per-class audit can iterate a book's whole
    /// roster the same way regardless of book.
    pub const ALL: &'static [ApgClassId] = &[
        ApgClassId::Alchemist,
        ApgClassId::Cavalier,
        ApgClassId::Inquisitor,
        ApgClassId::Oracle,
        ApgClassId::Summoner,
        ApgClassId::Witch,
    ];

    /// Lowercase class name, matching the `class_id` string convention
    /// `pilot_compute.rs`'s `FIGHTER_CLASS_ID`/`WIZARD_CLASS_ID` constants
    /// use (`"class:<name>"`), for building synthetic audit inputs and for
    /// naming this class in coverage-report output.
    pub const fn name(self) -> &'static str {
        match self {
            ApgClassId::Alchemist => "alchemist",
            ApgClassId::Cavalier => "cavalier",
            ApgClassId::Inquisitor => "inquisitor",
            ApgClassId::Oracle => "oracle",
            ApgClassId::Summoner => "summoner",
            ApgClassId::Witch => "witch",
        }
    }
}

/// Resolves an APG class's chassis-table row for `level`, scoped to
/// `RuleSetId::Apg`. Returns `None` for any other rule set — an APG
/// class chassis is never a valid answer for a `RuleSetId::Crb` query
/// (cross-book invariant, `corpus-source-inventory.md` §1.3), and
/// `None` when `level` exceeds the class's real `MAXLEVEL` ceiling.
pub fn class_chassis_resolve(
    class_id: ApgClassId,
    level: u8,
    rule_set: RuleSetId,
) -> Option<ClassTableRow> {
    if rule_set != RuleSetId::Apg {
        return None;
    }
    match class_id {
        ApgClassId::Alchemist => class_alchemist::class_table()
            .into_iter()
            .find(|row| row.level == level),
        ApgClassId::Cavalier => class_cavalier::class_table()
            .into_iter()
            .find(|row| row.level == level),
        ApgClassId::Inquisitor => class_inquisitor::class_table()
            .into_iter()
            .find(|row| row.level == level),
        ApgClassId::Oracle => class_oracle::class_table()
            .into_iter()
            .find(|row| row.level == level),
        ApgClassId::Summoner => class_summoner::class_table()
            .into_iter()
            .find(|row| row.level == level),
        ApgClassId::Witch => class_witch::class_table()
            .into_iter()
            .find(|row| row.level == level),
    }
}

/// SD-24 Epic 4 criterion 4.2 (per-class audit: APG classes) — per-class
/// wiring coverage row. Every field is computed from real, already-landed
/// source (this module's own `class_table()` outputs, or a documented
/// corpus count below) — never a hand-guessed or invented number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApgClassCoverage {
    pub class_id: ApgClassId,
    /// Levels 1 through `chassis_rows_expected` this class's `class_table()`
    /// actually returns a base-attack-bonus/save row for. Real APG classes
    /// all cap at `MAXLEVEL:20` (see each per-class module's own doc
    /// comment, cross-checked against `apg_classes.lst`), so this equals
    /// `chassis_rows_expected` for every class today.
    pub chassis_rows_wired: u8,
    /// The class's real `MAXLEVEL` ceiling (`apg_classes.lst`).
    pub chassis_rows_expected: u8,
    /// Count of distinct named/narrative class-feature records (Bombs,
    /// Discoveries, Mutagen, Hex, Judgment, Mystery Revelation, Eidolon,
    /// Challenge, ...) this repo has independent wired computation logic
    /// for, analogous to `level_up/fighter.rs`'s `explain_fighter_class_features`
    /// wiring for the CRB Fighter. Zero for every APG class today: SD-22
    /// Epic 3 deliberately scoped its ingest to the BAB/save chassis only
    /// (see e.g. `class_alchemist.rs`'s own doc comment), and no follow-on
    /// cycle has since ingested `apg_abilities_class.lst`'s per-level
    /// feature blocks for any APG class.
    pub named_features_wired: u32,
    /// Count of distinct named class-feature records tagged
    /// `KEY:<Class> ~ ...` for this class in the real PCGen corpus's
    /// `advanced_players_guide/apg_abilities_class.lst` (SD-24 Epic 4
    /// audit count, PCGen corpus commit `7f818006e371188e5717fd18d74d18a420747fc6`,
    /// 2026-06-17; reproduce with
    /// `grep -oE "KEY:<Class> ~ [^\t]+" apg_abilities_class.lst | sort -u | wc -l`).
    /// This counts mechanical class-feature *slots* (Bomb, Mutagen, Hex,
    /// Judgment, ...), not each slot's own selectable sub-options (e.g. the
    /// ~20 individual hexes a Witch can pick from live under a separate,
    /// not-yet-audited `CATEGORY:Special Ability` chooser list in
    /// `apg_abilities.lst`, not this file) — so this number is a floor on
    /// the real per-class feature surface, not a ceiling.
    pub named_features_expected: u32,
    /// Whether `pilot_compute.rs`'s live `compute_class_chassis` dispatch
    /// (the function the character-hub pilot flow actually calls)
    /// recognizes this class at all. `false` for every APG class today —
    /// confirmed both by inspection (`compute_class_chassis` only matches
    /// `FIGHTER_CLASS_ID`/`WIZARD_CLASS_ID`) and empirically by
    /// `tests/sd24_apg_class_coverage_audit.rs`'s
    /// `apg_classes_trip_the_honest_class_chassis_unsupported_diagnostic`
    /// test, which drives a real `CharacterInput` for each APG class
    /// through `compute_pilot_base_chassis` and confirms the claim-blocking
    /// `class_chassis.unsupported` diagnostic fires rather than any
    /// fabricated chassis numbers.
    pub pilot_compute_integrated: bool,
    /// Whether a `level_up::<class>` module (the SD-20 Epic 7 per-level
    /// automatic-feature-grant model CRB's 11 classes all have) exists for
    /// this class. `false` for every APG class today — `src/rules_core/level_up/`
    /// contains only CRB per-class modules.
    pub level_up_wired: bool,
}

const fn named_features_expected(class_id: ApgClassId) -> u32 {
    // Corpus counts per this struct's own doc comment — see there for the
    // reproduction command and the exact PCGen corpus commit audited.
    match class_id {
        ApgClassId::Alchemist => 24,
        ApgClassId::Cavalier => 16,
        ApgClassId::Inquisitor => 19,
        ApgClassId::Oracle => 19,
        ApgClassId::Summoner => 17,
        ApgClassId::Witch => 7,
    }
}

/// Computes `class_id`'s SD-24 Epic 4 coverage row.
pub fn class_coverage(class_id: ApgClassId) -> ApgClassCoverage {
    let chassis_rows_wired = match class_id {
        ApgClassId::Alchemist => class_alchemist::class_table().len(),
        ApgClassId::Cavalier => class_cavalier::class_table().len(),
        ApgClassId::Inquisitor => class_inquisitor::class_table().len(),
        ApgClassId::Oracle => class_oracle::class_table().len(),
        ApgClassId::Summoner => class_summoner::class_table().len(),
        ApgClassId::Witch => class_witch::class_table().len(),
    } as u8;
    let chassis_rows_expected = match class_id {
        ApgClassId::Alchemist => class_alchemist::MAX_SUPPORTED_LEVEL,
        ApgClassId::Cavalier => class_cavalier::MAX_SUPPORTED_LEVEL,
        ApgClassId::Inquisitor => class_inquisitor::MAX_SUPPORTED_LEVEL,
        ApgClassId::Oracle => class_oracle::MAX_SUPPORTED_LEVEL,
        ApgClassId::Summoner => class_summoner::MAX_SUPPORTED_LEVEL,
        ApgClassId::Witch => class_witch::MAX_SUPPORTED_LEVEL,
    };

    ApgClassCoverage {
        class_id,
        chassis_rows_wired,
        chassis_rows_expected,
        named_features_wired: 0,
        named_features_expected: named_features_expected(class_id),
        pilot_compute_integrated: false,
        level_up_wired: false,
    }
}

/// The full APG per-class coverage report (SD-24 Epic 4, criterion 4.2),
/// one row per `ApgClassId::ALL` entry in ingest order.
pub fn coverage_report() -> Vec<ApgClassCoverage> {
    ApgClassId::ALL.iter().map(|&class_id| class_coverage(class_id)).collect()
}
