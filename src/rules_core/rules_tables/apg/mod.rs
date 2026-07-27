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

pub mod alchemist_spell_list;
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

    /// The inverse of `name`, matching a real `CharacterClassLevel.class_id`
    /// string (`"class:alchemist"`, etc.) back to its `ApgClassId` (v0.6
    /// alpha swarm, risks item 8) -- the lookup `pilot_compute.rs`'s
    /// dispatch needs to recognize an APG class by its real chosen-input id.
    /// Returns `None` for any string that isn't one of the 6 real APG
    /// class ids (including any real CRB/ACG class id, by construction --
    /// no cross-book name collision exists today).
    pub fn from_class_id_str(class_id_str: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|id| class_id_str == format!("class:{}", id.name()))
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

/// `class_id`'s real hit-die size (v0.6 alpha swarm, risks item 8), each
/// per-class module's own `HIT_DIE` constant -- verified directly against
/// its real `apg_classes.lst` `HD:` token, same as
/// `rules_tables::crb::class_tables::hit_die_for`'s precedent. Unlike
/// `class_chassis_resolve`, this is not `RuleSetId`-gated -- hit-die size
/// has no per-book collision risk the way a class *name* could.
pub fn hit_die_for(class_id: ApgClassId) -> u8 {
    match class_id {
        ApgClassId::Alchemist => class_alchemist::HIT_DIE,
        ApgClassId::Cavalier => class_cavalier::HIT_DIE,
        ApgClassId::Inquisitor => class_inquisitor::HIT_DIE,
        ApgClassId::Oracle => class_oracle::HIT_DIE,
        ApgClassId::Summoner => class_summoner::HIT_DIE,
        ApgClassId::Witch => class_witch::HIT_DIE,
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
    /// wiring for the CRB Fighter. Zero for every APG class except
    /// Cavalier, Alchemist, Inquisitor, Oracle, and Witch as of the v0.6
    /// alpha swarm's Cavalier Mount / Alchemist Mutagen / Inquisitor
    /// Judgment / Oracle / Witch full-build closures (risks item 8):
    /// Cavalier's 1st-level Mount, Alchemist's Mutagen, Inquisitor's
    /// Justice judgment, Oracle's Mystery (Life/Healing Hands) and Curse
    /// (Clouded Vision), and Witch's Ward hex are now genuinely wired
    /// (`pilot_compute::ground_cavalier_mount_and_defer_the_rest`,
    /// `pilot_compute::ground_or_block_alchemist_mutagen`,
    /// `pilot_compute::ground_or_block_inquisitor_judgment`,
    /// `pilot_compute::ground_or_block_oracle_mystery`/
    /// `ground_or_block_oracle_curse`,
    /// `pilot_compute::ground_or_block_witch_class_features`) -- see
    /// `class_coverage`'s own branches for each. Oracle counts 5
    /// (Mystery slot + Clouded Vision + Lame + Wasting + Deaf). It was 2
    /// (Mystery slot + Curse slot) until the curse deepening
    /// (2026-07-26, task #10) grounded three more curses.
    ///
    /// Each curse earns its own slot under the same corpus-prefix test
    /// that governs every other class here: `KEY:Oracle ~ Lame`,
    /// `KEY:Oracle ~ Wasting`, and `KEY:Oracle ~ Deaf` are top-level
    /// `KEY:Oracle ~ ...` records with genuinely independent formulas (a
    /// race-speed-dependent movement reduction, a Charisma-skill penalty
    /// with an Intimidate cancellation, and a curse-level-tiered
    /// initiative/Perception pair) -- the same reasoning that gave
    /// Inquisitor's Stern Gaze/Monster Lore/Cunning Initiative/Track
    /// their own slots, and Warpriest's Fervor/Channel Energy/Sacred
    /// Armor theirs.
    ///
    /// Mystery REVELATIONS do not add slots, for the mirror-image
    /// reason: they are keyed `KEY:Life Mystery ~ ...` /
    /// `KEY:Lore Mystery ~ ...` in the corpus -- a DIFFERENT prefix, not
    /// `KEY:Oracle ~ ...` -- making them sub-selectable-list entries
    /// under the single Mystery slot, exactly as Warpriest's
    /// `KEY:Destruction Blessing ~ ...` / `KEY:Strength Blessing ~ ...`
    /// minor powers fold into its single Blessings slot and Witch's ~20
    /// hexes fold into its single Hex slot. Inquisitor counts 5 (Judgment slot + Stern Gaze slot +
    /// Monster Lore + Cunning Initiative + Track, the latter three added
    /// 2026-07-26, task #18 -- see below). Judgment stays ONE slot even
    /// though it now grounds 4 of its 8 selectable sub-types (Justice/
    /// Protection/Purity/Smiting) -- per this field's own "counts slots,
    /// not each slot's selectable sub-options" convention (see
    /// `named_features_expected`'s doc comment, the same reasoning that
    /// keeps Witch's ~20 individual hexes from inflating its own count
    /// past the single Hex slot). Stern Gaze, Monster Lore, Cunning
    /// Initiative, and Track are each genuinely separate, top-level
    /// `KEY:Inquisitor ~ ...` records, so each adds its own slot rather
    /// than folding into Judgment's. **Monster Lore/Cunning Initiative/
    /// Track correction (task #18, 2026-07-26)**: these were originally
    /// excluded under an over-strict "needs a live consumer" bar -- this
    /// codebase's own established precedent (Bard's Bardic Knowledge,
    /// Slayer's Track/Trapfinding, Barbarian's Damage Reduction all
    /// already ground a standalone flat fact with zero live consumer)
    /// shows a consumer was never actually required, only a genuinely
    /// verified magnitude; all three now ground the same way. Alchemist
    /// counts 3 (Mutagen slot + Bomb slot + Poison Resistance slot,
    /// deepening 2026-07-26, task #4): Bomb (`KEY:Alchemist ~ Bomb`, one
    /// record covering damage dice/bonus, save DC, and uses-per-day as
    /// three numeric facets of the SAME feature, not separate slots --
    /// the same "one record, several parameters" shape Cleric's Channel
    /// Energy and Warpriest's Blessings-DC-plus-uses already established)
    /// and Poison Resistance (`KEY:Alchemist ~ Poison Resistance`, its
    /// own separate record, identical tiers to Investigator's own,
    /// re-derived independently rather than assumed) are both genuinely
    /// wired now. Alchemist's own prepared-extract spellcasting (reusing
    /// the shared `alchemist_spell_list` module Investigator's own
    /// closure built) does NOT add a fourth slot, per the same
    /// spellcasting-sharing convention below. Oracle's
    /// known-spell posture and Orisons are NOT counted separately here,
    /// the same "shares the general spellcasting mechanism, not
    /// independently implemented" reasoning that already excluded
    /// Arcanist's Cantrips and Warpriest's Orisons from their own counts
    /// (see `docs/release/v0.6/oracle-apg-full-build-scoping.md`).
    /// Cavalier, Alchemist, and Witch each count 1 (a single slot alone).
    /// Every other APG class remains at 0: SD-22 Epic 3 deliberately
    /// scoped its ingest to the BAB/save chassis only (see e.g.
    /// `class_alchemist.rs`'s own doc comment), and no follow-on cycle has
    /// since ingested `apg_abilities_class.lst`'s per-level feature blocks
    /// for any other APG class.
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
    /// recognizes this class at all.
    ///
    /// **`true` as of v0.6 alpha swarm, risks item 8 (2026-07-24)**:
    /// `compute_class_chassis` now recognizes all 6 real APG classes via
    /// `ApgClassId::from_class_id_str` + `compute_apg_class_chassis`,
    /// grounding real BAB/save (and, via `durability::compute_max_hp`, real
    /// HP) for each. This was deliberately left unwired since SD-22; the
    /// class-skill/feature/spellcasting bucket is still genuinely
    /// ungrounded, so a real, unconditional `class_feature.apg.<class>.unsupported`
    /// diagnostic keeps every APG class honestly `Blocked` overall (see
    /// `compute_apg_class_chassis`'s own doc comment in `pilot_compute.rs`
    /// for why this replaces the old generic `class_chassis.unsupported`
    /// diagnostic this field's previous doc comment cited).
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

    // v0.6 alpha swarm, risks item 8 (Cavalier Mount / Alchemist Mutagen /
    // Inquisitor Judgment / Oracle full-build closures): see this field's
    // own doc comment above for the exact per-class counting reasoning.
    let named_features_wired = match class_id {
        ApgClassId::Cavalier | ApgClassId::Witch => 1,
        ApgClassId::Alchemist => 3,
        ApgClassId::Oracle | ApgClassId::Inquisitor => 5,
        _ => 0,
    };

    ApgClassCoverage {
        class_id,
        chassis_rows_wired,
        chassis_rows_expected,
        named_features_wired,
        named_features_expected: named_features_expected(class_id),
        // v0.6 alpha swarm, risks item 8: real as of `compute_apg_class_chassis`
        // (`pilot_compute.rs`) -- see this field's own doc comment.
        pilot_compute_integrated: true,
        level_up_wired: false,
    }
}

/// The full APG per-class coverage report (SD-24 Epic 4, criterion 4.2),
/// one row per `ApgClassId::ALL` entry in ingest order.
pub fn coverage_report() -> Vec<ApgClassCoverage> {
    ApgClassId::ALL.iter().map(|&class_id| class_coverage(class_id)).collect()
}
