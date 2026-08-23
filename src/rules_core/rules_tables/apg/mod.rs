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

//!
//! SD-29 Epic 7 round 9 (`SD29-E7-F2-010`) added `companion_data` -- this
//! book's `companion` family. **The module is `apg`; the corpus book is
//! `advanced_players_guide`**, the same split `decisions.md §54.3` recorded for
//! Bestiary 1 and round 8 for Core Rulebook; `transcribe_companion_tables`'s
//! `MODULE_DIR` already carried the mapping, added by round 8 in anticipation
//! of exactly this round.
//!
//! 17 of the book's 220 `companion` corpus rows ship (9 creature rows and 8
//! ability rows), which is exactly the `reachable remainder`
//! `python3 scripts/classify_companion_rows.py advanced_players_guide` prints.
//! No new `RuleSetId`: `RuleSetId::Apg` predates this lane.
//!
//! **4 -> 17 by `SD31-CE-COMPANION-001` (2026-08-18), and only 8 of the 13 are
//! new rows.** `decisions.md §9` retired the `core_essentials` book id, and
//! `ce_races_familiar_apg.lst` -- which declares `SOURCELONG:Advanced Player's
//! Guide` in its own header -- brought this book 8 familiar creature rows that
//! had been served out of a `core_essentials` table while their corpus records
//! sat, unreachable, in this book's own `data/corpus/` directory. Adding those
//! 8 owners then gave FIVE previously-orphan `apg_abilities_companion.lst`
//! rows an owner for the first time, which is the other half of the move: an
//! ability row's shippability is a fact about whether a shipped creature
//! reaches it, so importing owners can un-orphan rows nothing else touched.
//!
//! **This is the lane's most lopsided book: 208 of its 212 rows do not ship,
//! and they are ONE finding.** They are the summoner's evolution pool --
//! `Evolution ~ …` and `Temp Evolution ~ …` -- which hangs off the eidolon
//! CLASS feature this chassis does not model rather than off the `Eidolon`
//! creature row. Ultimate Magic and Advanced Race Guide, both landed the same
//! round, carry the continuation of the same block; the three books' 361
//! orphans between them are one missing record type seen three times, not 361
//! per-row accidents. Carried per `decisions.md §50`, named row by row in
//! `companion_data`'s module doc, and keeping their honest `not-ingested`
//! status in `docs/work-inventory.json`.

pub mod alchemist_spell_list;
pub mod antipaladin_features;
pub mod archetype_tables;
mod companion_data;

pub use super::companion_chassis::{CompanionAbilityRecord, CompanionRecord};

/// Every companion creature this book defines, in corpus row order.
pub const fn companions_static() -> &'static [CompanionRecord] {
    companion_data::COMPANIONS
}

/// Every companion ability record this book defines, in corpus row order.
pub const fn companion_abilities_static() -> &'static [CompanionAbilityRecord] {
    companion_data::COMPANION_ABILITIES
}

/// Every companion creature this book defines, in corpus row order.
pub fn companions() -> &'static [CompanionRecord] {
    companions_static()
}

/// Every companion ability record this book defines, in corpus row order.
pub fn companion_abilities() -> &'static [CompanionAbilityRecord] {
    companion_abilities_static()
}

pub mod inquisitor_spell_list;
pub mod witch_spell_list;
pub mod class_alchemist;
pub mod class_cavalier;
pub mod class_inquisitor;
pub mod class_oracle;
pub mod class_summoner;
pub mod class_witch;
pub mod equipment_data;
pub mod equipment_tables;
pub mod feat_data;
pub mod feats;
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
    /// hexes fold into its single Hex slot. Inquisitor counts 6 (Judgment
    /// slot, Stern Gaze slot, Monster Lore, Cunning Initiative, Track, and
    /// Bane -- the middle three added 2026-07-26 task #18, Bane added task
    /// #47, 2026-07-28 -- see below). Judgment stays ONE slot even
    /// though it now grounds all 9 of the real judgment types (Justice/
    /// Protection/Purity/Smiting, widened task #47 to add Destruction/
    /// Healing/Piercing/Resiliency/Resistance too) -- per this field's own
    /// "counts slots, not each slot's selectable sub-options" convention
    /// (see `named_features_expected`'s doc comment, the same reasoning
    /// that keeps Witch's ~20 individual hexes from inflating its own
    /// count past the single Hex slot). Stern Gaze, Monster Lore, Cunning
    /// Initiative, Track, and Bane are each genuinely separate, top-level
    /// `KEY:Inquisitor ~ ...` records, so each adds its own slot rather
    /// than folding into Judgment's. **Monster Lore/Cunning Initiative/
    /// Track correction (task #18, 2026-07-26)**: these were originally
    /// excluded under an over-strict "needs a live consumer" bar -- this
    /// codebase's own established precedent (Bard's Bardic Knowledge,
    /// Slayer's Track/Trapfinding, Barbarian's Damage Reduction all
    /// already ground a standalone flat fact with zero live consumer)
    /// shows a consumer was never actually required, only a genuinely
    /// verified magnitude; all three now ground the same way, and task
    /// #47's Judgment widening (Destruction/Healing/Piercing/Resiliency/
    /// Resistance) and Bane both apply the identical corrected bar.
    /// Inquisitor's own known-spell posture (a real, independently-
    /// verified 219-spell spontaneous list, `rules_tables::apg::
    /// inquisitor_spell_list`, built fresh since the real corpus record
    /// carries no `SPELLLIST:` token to reuse) does NOT add a seventh
    /// slot, the same "shares the general spellcasting mechanism, not
    /// independently implemented" convention Oracle's own known-spell
    /// posture already established below. Alchemist
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
    /// The full current roster, all six APG classes, none at 0:
    /// **Witch 2**, **Alchemist 3**, **Oracle 5**, **Inquisitor 6**,
    /// **Cavalier 6**, **Summoner 6**.
    ///
    /// Summoner rose from 1 to 6 when Slice A landed (`d7eec49f`): the
    /// Eidolon slot plus Bond Senses, Maker's Call, Merge Forms, Twin
    /// Eidolon, and Summon Monster (whose duration, uses/day and
    /// accessible spell level are three facets of one slot, per the
    /// Cleric-Channel-Energy convention). The bump did not ship with that
    /// commit; it is corrected here. Worth noting *why* it went unnoticed:
    /// Summoner sits in the coverage audit's zero-canary **skip** list
    /// while having no row in that test's explicit expected-count table,
    /// so nothing asserted its value at all. A row was added alongside
    /// this correction -- the stale number was the symptom, the missing
    /// assertion was the cause.
    ///
    /// Witch's 2 is the Hex slot -- all 27 hexes fold into it, being
    /// mutually-exclusive picks of one chooser rather than independent
    /// features -- plus Familiar, a genuinely separate feature landing a
    /// real magnitude on the computed max-HP total. That familiar bump
    /// was owed from the familiar closure (`8e47479a`) and landed for
    /// Shaman first, so for a while the two classes counted one shared
    /// implementation differently; corrected 2026-07-27.
    ///
    /// This paragraph previously read "Cavalier, Alchemist, and Witch
    /// each count 1 ... every other APG class remains at 0", describing
    /// the SD-22 Epic 3 state where the ingest was scoped to the
    /// BAB/save chassis only. Every clause of that is now false --
    /// Alchemist and Cavalier had already grown past 1, and Oracle and
    /// Inquisitor past 0, before Witch's own correction touched it.
    /// Corrected 2026-07-27 alongside the Witch bump, since a count
    /// change has to sweep the prose derived from the old counts too --
    /// no test asserts a doc comment, so this is the one place these
    /// numbers can rot silently.
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
        // Witch counts 2: the Hex slot (all 27 hexes fold into it as
        // mutually-exclusive picks of one chooser) + Familiar; it has no
        // Eidolon. Summoner counts 6: its Eidolon slot plus Slice A's five
        // (`d7eec49f`) -- Bond Senses, Maker's Call, Merge Forms, Twin
        // Eidolon, and Summon Monster, whose three facets (duration,
        // uses/day, accessible spell level) count as ONE slot per the
        // Cleric-Channel-Energy "one record, several parameters"
        // convention that already governs Alchemist's Bomb.
        //
        // Summoner rose from 6 to 7 (Summoner Eidolon evolution
        // canonical-narrowing closure, 2026-07-29): the Improved Natural
        // Armor evolution purchase is its own distinct corpus record
        // (`KEY:Evolution ~ Improved Natural Armor`) with genuinely
        // separate logic -- its own cost drawn from the pool, its own
        // `PREVARLTEQ` prerequisite evaluated, and its own magnitude
        // landed on the Eidolon's natural-armor total. That is a
        // different mechanism from the Eidolon stat-block slot it sits
        // beside, not another parameter of it. The other 103 evolution
        // records do NOT add slots -- they are the same chooser's
        // unbuilt sub-options, per this field's own "counts slots, not
        // each slot's selectable sub-options" convention.
        ApgClassId::Summoner => 7,
        ApgClassId::Witch => 2,
        ApgClassId::Alchemist => 3,
        ApgClassId::Oracle => 5,
        // Inquisitor rose from 5 to 6 (task #47, 2026-07-28): Bane is a
        // genuinely separate, top-level `KEY:Inquisitor ~ Bane` record, so
        // it earns its own slot (Judgment slot + Stern Gaze slot +
        // Monster Lore + Cunning Initiative + Track + Bane). Widening
        // Judgment's own sub-types from 4/9 to 9/9 real judgment types
        // (Destruction/Healing/Piercing/Resiliency/Resistance joined
        // Justice/Protection/Purity/Smiting) does NOT add a slot, per this
        // field's own "counts slots, not each slot's selectable
        // sub-options" convention -- the same reasoning that keeps
        // Witch's ~20 hexes and Summoner's Summon Monster facets from
        // inflating their own counts. Inquisitor's own known-spell
        // posture (a real, independently-verified 219-spell spontaneous
        // list) also does NOT add a slot, the same "shares the general
        // spellcasting mechanism, not independently implemented"
        // convention already applied to Oracle's own known-spell posture.
        ApgClassId::Inquisitor => 6,
        ApgClassId::Cavalier => 6,
        // Exhaustive over all six ApgClassId variants now that every
        // APG class has at least one wired named feature -- the former
        // `_ => 0` wildcard is gone, so adding a class forces a
        // deliberate count rather than silently defaulting to 0.
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
