//! PF1 CRB class tables — one row per class per level.
//!
//! Base attack bonus and base-save cells are derived from the same
//! full/three-quarter/half-BAB and good/poor-save formulas already
//! implemented (and primary-source-verified by SD-18's test suite) in
//! `pilot_compute.rs`'s per-class chassis functions — not re-derived from
//! memory. Coverage is bounded to each class's `MAX_SUPPORTED_<CLASS>_LEVEL`
//! ceiling from `pilot_compute.rs`, since a row this table carries beyond
//! what the chassis seam actually supports would be an unverifiable claim.
//!
//! Named per-level features and exact spell-per-day cells are
//! deliberately out of scope for this bootstrap: slot math is a named
//! SD-19 non-goal (`scope-draft.md` §1.1 "What this slice does NOT do"),
//! and hand-transcribing exhaustive per-level feature text without a
//! verifiable in-repo source would be exactly the fabricated-data risk
//! `AGENTS.md` rules out.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClassId {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Wizard,
}

impl ClassId {
    pub const ALL: &'static [ClassId] = &[
        ClassId::Barbarian,
        ClassId::Bard,
        ClassId::Cleric,
        ClassId::Druid,
        ClassId::Fighter,
        ClassId::Monk,
        ClassId::Paladin,
        ClassId::Ranger,
        ClassId::Rogue,
        ClassId::Sorcerer,
        ClassId::Wizard,
    ];
}

/// `pub(crate)` (rather than private) so
/// `pilot_compute::untabled_base_class_chassis` (SD-32 Epic 3's
/// 20-real-base-classes-without-tables cycle) can reuse this exact
/// classification and the two formula functions below rather than
/// re-declaring a second, independently-maintained copy of either.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BabProgression {
    Full,
    ThreeQuarter,
    Half,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GoodSaves {
    fortitude: bool,
    reflex: bool,
    will: bool,
}

struct ClassMeta {
    class_id: ClassId,
    max_supported_level: u8,
    bab: BabProgression,
    good_saves: GoodSaves,
    /// Hit die size (SD-13/v0.6 alpha swarm durability calc), from the same
    /// `cr_classes.lst` `HD:` token every other field in this table already
    /// cites -- e.g. `CLASS:Fighter HD:10` at line 139 (the same line this
    /// table's own save-formula doc comments already cite for Fighter).
    ///
    /// **One row is deliberately NOT a transcription of its `HD:` token:**
    /// Monk is `8` here against the corpus's `HD:10`, per the operator's
    /// 2026-07-29 ruling (risks item 91). See the comment block on the Monk
    /// row below before changing it -- it is a documented corpus-defect
    /// override, not an uncorrected drift.
    hit_die: u8,
}

/// Mirrors `MAX_SUPPORTED_<CLASS>_LEVEL` in `pilot_compute.rs` as of the
/// SD-18 level-20 capstone-widening sweep (2026-07-16).
const CLASS_META: &[ClassMeta] = &[
    ClassMeta { class_id: ClassId::Barbarian, max_supported_level: 20, bab: BabProgression::Full, good_saves: GoodSaves { fortitude: true, reflex: false, will: false }, hit_die: 12 },
    ClassMeta { class_id: ClassId::Bard, max_supported_level: 20, bab: BabProgression::ThreeQuarter, good_saves: GoodSaves { fortitude: false, reflex: true, will: true }, hit_die: 8 },
    ClassMeta { class_id: ClassId::Cleric, max_supported_level: 20, bab: BabProgression::ThreeQuarter, good_saves: GoodSaves { fortitude: true, reflex: false, will: true }, hit_die: 8 },
    // Druid widened 15 -> 20 (v0.6, 2026-07-29), the last CRB class still
    // short of the cap. `CLASS:Druid` carries `MAXLEVEL:20` and its BAB and
    // save formulas are byte-for-byte identical to `CLASS:Cleric`'s
    // (`cr_classes.lst` lines 93 and 55), which already ran to 20 here.
    ClassMeta { class_id: ClassId::Druid, max_supported_level: 20, bab: BabProgression::ThreeQuarter, good_saves: GoodSaves { fortitude: true, reflex: false, will: true }, hit_die: 8 },
    ClassMeta { class_id: ClassId::Fighter, max_supported_level: 20, bab: BabProgression::Full, good_saves: GoodSaves { fortitude: true, reflex: false, will: false }, hit_die: 10 },
    // MONK HIT DIE: DELIBERATE, OPERATOR-RULED OVERRIDE OF A CORPUS DEFECT.
    // DO NOT "correct" this 8 back to 10 to match the corpus.
    //
    // The PCGen corpus says 10: `cr_classes.lst:147` reads
    // `CLASS:Monk  HD:10  ... SOURCEPAGE:p.56`. Every other `hit_die` in
    // this table is a faithful transcription of its own `HD:` token. This
    // one is not, and that is intentional.
    //
    // The published Pathfinder 1e Core Rulebook, at the very page the
    // corpus record itself cites (p.56), gives the Monk a d8. The corpus
    // token contradicts its own SOURCEPAGE -- an internal contradiction of
    // exactly the kind risks item 50 (Swashbuckler's `SwashbucklerDeedQualifyLVL`)
    // and item 69 (Skald's Raging Song `-2` base-offset) established as the
    // bar for deviating from a literal corpus token. The operator (Todd
    // Hintzmann) ruled directly on 2026-07-29 that the Monk's hit die is d8.
    //
    // This is recorded as risks item 91 in
    // `docs/release/v0.6/risks-and-open-questions.md`. The consequence is
    // real and visible: a Monk 20 at CON +2 is 143 HP here, versus 164 under
    // the corpus's d10. Pinned at levels 1/10/20 by
    // `tests/v06_durability.rs`'s
    // `monk_max_hp_follows_the_published_d8_not_the_corpus_d10_at_levels_1_10_and_20`,
    // which also asserts the d10 values are NOT produced.
    //
    // Note the corpus is NOT edited to match: `$HOME/workspace/repos/pcgen`
    // is this project's independent parity oracle (`src/oracle_validation/`),
    // and making the oracle agree with us by construction would destroy the
    // independence that makes parity testing meaningful -- the precise blind
    // spot that hid the -4 nonproficiency error in risks item 89.
    ClassMeta { class_id: ClassId::Monk, max_supported_level: 20, bab: BabProgression::ThreeQuarter, good_saves: GoodSaves { fortitude: true, reflex: true, will: true }, hit_die: 8 },
    ClassMeta { class_id: ClassId::Paladin, max_supported_level: 20, bab: BabProgression::Full, good_saves: GoodSaves { fortitude: true, reflex: false, will: true }, hit_die: 10 },
    ClassMeta { class_id: ClassId::Ranger, max_supported_level: 20, bab: BabProgression::Full, good_saves: GoodSaves { fortitude: true, reflex: true, will: false }, hit_die: 10 },
    ClassMeta { class_id: ClassId::Rogue, max_supported_level: 20, bab: BabProgression::ThreeQuarter, good_saves: GoodSaves { fortitude: false, reflex: true, will: false }, hit_die: 8 },
    ClassMeta { class_id: ClassId::Sorcerer, max_supported_level: 20, bab: BabProgression::Half, good_saves: GoodSaves { fortitude: false, reflex: false, will: true }, hit_die: 6 },
    ClassMeta { class_id: ClassId::Wizard, max_supported_level: 20, bab: BabProgression::Half, good_saves: GoodSaves { fortitude: false, reflex: false, will: true }, hit_die: 6 },
];

/// `class_id`'s hit die size (e.g. `10` for Fighter's d10), from this
/// table's own ingested `CLASS_META`. Returns `None` for a `class_id` this
/// table does not carry a row for -- mirrors `good_saves_for`'s own
/// `None`-when-absent contract.
pub fn hit_die_for(class_id: ClassId) -> Option<u8> {
    CLASS_META.iter().find(|meta| meta.class_id == class_id).map(|meta| meta.hit_die)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassTableRow {
    pub class_id: ClassId,
    pub level: u8,
    pub base_attack_bonus: i16,
    pub fort_save: i16,
    pub ref_save: i16,
    pub will_save: i16,
}

/// `pub(crate)` -- see `BabProgression`'s own doc comment for why.
pub(crate) fn base_attack_bonus(bab: BabProgression, level: u8) -> i16 {
    let level = level as i16;
    match bab {
        BabProgression::Full => level,
        BabProgression::ThreeQuarter => (level * 3) / 4,
        BabProgression::Half => level / 2,
    }
}

/// `pub(crate)` -- see `BabProgression`'s own doc comment for why.
pub(crate) fn save_bonus(level: u8, good: bool) -> i16 {
    let level = level as i16;
    if good {
        level / 2 + 2
    } else {
        level / 3
    }
}

/// This table's own good/poor Fortitude/Reflex/Will classification for
/// `class_id` (Fortitude, Reflex, Will), the classification `save_bonus`
/// applies to build each `ClassTableRow`'s already-floored per-class save
/// cells. Exposed so callers that need the *un-rounded* fractional pre-floor
/// value per class (SD-21 E7.29's multiclass rule: sum every class's own
/// fractional save contribution, then floor once for the total) can read
/// the classification from this table's own ingested `CLASS_META`, rather
/// than re-declaring it a second time elsewhere in the codebase (SD-24
/// Epic 5 criterion 5.3). Returns `None` for a `class_id` this table does
/// not carry a row for.
pub fn good_saves_for(class_id: ClassId) -> Option<(bool, bool, bool)> {
    CLASS_META.iter().find(|meta| meta.class_id == class_id).map(|meta| {
        (
            meta.good_saves.fortitude,
            meta.good_saves.reflex,
            meta.good_saves.will,
        )
    })
}

/// Builds the CRB class table: one row per class per level, from level 1
/// through that class's `max_supported_level`.
pub fn class_tables() -> Vec<ClassTableRow> {
    let mut rows = Vec::new();
    for meta in CLASS_META {
        for level in 1..=meta.max_supported_level {
            rows.push(ClassTableRow {
                class_id: meta.class_id,
                level,
                base_attack_bonus: base_attack_bonus(meta.bab, level),
                fort_save: save_bonus(level, meta.good_saves.fortitude),
                ref_save: save_bonus(level, meta.good_saves.reflex),
                will_save: save_bonus(level, meta.good_saves.will),
            });
        }
    }
    rows
}
