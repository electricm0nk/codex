//! ACG (Advanced Class Guide) book-level module. SD-22 Epic 4
//! content-source ingest — sibling directory to `rules_tables::apg` and
//! `rules_tables::crb` per `SD-19-corpus-aware-compute-seam/decisions.md`
//! §9. Arcanist is the first class ingested (`decisions.md §5`'s
//! corrected real-LST-corpus sourcing, corrected 2026-07-19).
//!
//! **Roster correction (this cycle, mirrors the APG Gunslinger/Magus
//! correction in `apg/mod.rs`):** `corpus-source-inventory.md §2.1`'s
//! row 1, "Alchemist (ACG-side)", names a class with **no real
//! `CLASS:Alchemist` record anywhere in `acg_classes.lst`** — confirmed
//! by direct grep of the real corpus (`grep -c "^CLASS:Alchemist"
//! acg_classes.lst` → 0). Alchemist is APG-only content; ACG never
//! republishes a distinct Alchemist chassis. The real, complete 10-class
//! `CLASS:` roster in `acg_classes.lst` is: Arcanist, Bloodrager,
//! Brawler, Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler,
//! Warpriest (plus an internal `Ex-Warpriest` `VISIBLE:NO` variant,
//! correctly excluded from the player-facing roster). `decisions.md
//! §3`'s stated ACG ordering also omits `Slayer` (which does have a
//! real record) while wrongly including "Alchemist" — the same
//! roster-defect shape as the resolved Gunslinger/Magus blocker. See
//! `docs/release/SD-22/progress.md`'s `## Open blockers` for the full
//! record. This module's roster started at Arcanist, the first class
//! with a real record, and — with Warpriest's ingest cycle — has now
//! grown to the full, corrected 10-class list: Arcanist, Bloodrager,
//! Brawler, Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler,
//! Warpriest. Epic 4's class-roster criteria (10-12) are complete for
//! all ten classes. Criterion 13 (shared ACG spell/equipment tables,
//! mirroring APG's criterion 9) lands this cycle as `spell_list.rs` and
//! `equipment_tables.rs` — Epic 4 (ACG) is now fully complete.

pub mod class_arcanist;
pub mod class_bloodrager;
pub mod class_brawler;
pub mod class_hunter;
pub mod class_investigator;
pub mod class_shaman;
pub mod class_skald;
pub mod class_slayer;
pub mod class_swashbuckler;
pub mod class_warpriest;
pub mod equipment_data;
pub mod equipment_tables;
pub mod spell_list;

use crate::rules_core::rules_tables::RuleSetId;

/// One ACG class's chassis-table row: level, BAB, and the three saves.
/// Shared shape across every per-class module in this directory so
/// `class_chassis_resolve` can return a single type regardless of
/// which class was queried. Mirrors `rules_tables::apg::ClassTableRow`
/// (kept book-local rather than shared, per that module's own
/// established shape).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassTableRow {
    pub level: u8,
    pub base_attack_bonus: i16,
    pub fort_save: i16,
    pub ref_save: i16,
    pub will_save: i16,
}

/// Identifies which ACG class a chassis-table query targets. Arcanist,
/// Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Slayer,
/// Swashbuckler, and Warpriest are all ten real ACG classes — the full,
/// corrected roster (see this module's doc comment for the roster
/// correction). Criterion 13 (shared ACG spell/equipment tables) remains
/// open as Epic 4's last piece.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AcgClassId {
    Arcanist,
    Bloodrager,
    Brawler,
    Hunter,
    Investigator,
    Shaman,
    Skald,
    Slayer,
    Swashbuckler,
    Warpriest,
}

impl AcgClassId {
    /// The full, corrected 10-class ACG roster (see this module's doc
    /// comment for why "Alchemist" is excluded and "Slayer" is included),
    /// in ingest order. SD-24 Epic 4 criterion 4.3's per-class audit
    /// iterates this list rather than a hand-typed one.
    pub const ALL: [AcgClassId; 10] = [
        AcgClassId::Arcanist,
        AcgClassId::Bloodrager,
        AcgClassId::Brawler,
        AcgClassId::Hunter,
        AcgClassId::Investigator,
        AcgClassId::Shaman,
        AcgClassId::Skald,
        AcgClassId::Slayer,
        AcgClassId::Swashbuckler,
        AcgClassId::Warpriest,
    ];

    /// Lowercase class name, matching the `class_id` string convention
    /// `pilot_compute.rs`'s `FIGHTER_CLASS_ID`/`WIZARD_CLASS_ID` constants
    /// use (`"class:<name>"`), for building synthetic audit inputs and for
    /// naming this class in coverage-report output. Mirrors
    /// `rules_tables::apg::ApgClassId::name`.
    pub const fn name(self) -> &'static str {
        match self {
            AcgClassId::Arcanist => "arcanist",
            AcgClassId::Bloodrager => "bloodrager",
            AcgClassId::Brawler => "brawler",
            AcgClassId::Hunter => "hunter",
            AcgClassId::Investigator => "investigator",
            AcgClassId::Shaman => "shaman",
            AcgClassId::Skald => "skald",
            AcgClassId::Slayer => "slayer",
            AcgClassId::Swashbuckler => "swashbuckler",
            AcgClassId::Warpriest => "warpriest",
        }
    }

    /// Reverse of `name`: resolves a `"class:<name>"` id string (the
    /// convention `pilot_compute.rs`'s per-class constants use) back to
    /// an `AcgClassId`. Mirrors `rules_tables::apg::ApgClassId::from_class_id_str`.
    pub fn from_class_id_str(class_id_str: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|id| class_id_str == format!("class:{}", id.name()))
    }
}

/// The real per-class hit die (`HD:` token on each class's real
/// `CLASS:` record in `acg_classes.lst`). Mirrors
/// `rules_tables::apg::hit_die_for` exactly.
pub fn hit_die_for(class_id: AcgClassId) -> u8 {
    match class_id {
        AcgClassId::Arcanist => class_arcanist::HIT_DIE,
        AcgClassId::Bloodrager => class_bloodrager::HIT_DIE,
        AcgClassId::Brawler => class_brawler::HIT_DIE,
        AcgClassId::Hunter => class_hunter::HIT_DIE,
        AcgClassId::Investigator => class_investigator::HIT_DIE,
        AcgClassId::Shaman => class_shaman::HIT_DIE,
        AcgClassId::Skald => class_skald::HIT_DIE,
        AcgClassId::Slayer => class_slayer::HIT_DIE,
        AcgClassId::Swashbuckler => class_swashbuckler::HIT_DIE,
        AcgClassId::Warpriest => class_warpriest::HIT_DIE,
    }
}

/// Resolves an ACG class's chassis-table row for `level`, scoped to
/// `RuleSetId::Acg`. Returns `None` for any other rule set — an ACG
/// class chassis is never a valid answer for a `RuleSetId::Crb` or
/// `RuleSetId::Apg` query (cross-book invariant,
/// `corpus-source-inventory.md` §2.3), and `None` when `level` exceeds
/// the class's real `MAXLEVEL` ceiling.
pub fn class_chassis_resolve(
    class_id: AcgClassId,
    level: u8,
    rule_set: RuleSetId,
) -> Option<ClassTableRow> {
    if rule_set != RuleSetId::Acg {
        return None;
    }
    match class_id {
        AcgClassId::Arcanist => class_arcanist::class_table()
            .into_iter()
            .find(|row| row.level == level),
        AcgClassId::Bloodrager => class_bloodrager::class_table()
            .into_iter()
            .find(|row| row.level == level),
        AcgClassId::Brawler => class_brawler::class_table()
            .into_iter()
            .find(|row| row.level == level),
        AcgClassId::Hunter => class_hunter::class_table()
            .into_iter()
            .find(|row| row.level == level),
        AcgClassId::Investigator => class_investigator::class_table()
            .into_iter()
            .find(|row| row.level == level),
        AcgClassId::Shaman => class_shaman::class_table()
            .into_iter()
            .find(|row| row.level == level),
        AcgClassId::Skald => class_skald::class_table()
            .into_iter()
            .find(|row| row.level == level),
        AcgClassId::Slayer => class_slayer::class_table()
            .into_iter()
            .find(|row| row.level == level),
        AcgClassId::Swashbuckler => class_swashbuckler::class_table()
            .into_iter()
            .find(|row| row.level == level),
        AcgClassId::Warpriest => class_warpriest::class_table()
            .into_iter()
            .find(|row| row.level == level),
    }
}

/// SD-24 Epic 4 criterion 4.3 (per-class audit: ACG classes) — per-class
/// wiring coverage row. Mirrors `rules_tables::apg::ApgClassCoverage`
/// exactly (same fields, same "every value is computed from real,
/// already-landed source, never hand-guessed or invented" discipline).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AcgClassCoverage {
    pub class_id: AcgClassId,
    /// Levels 1 through `chassis_rows_expected` this class's `class_table()`
    /// actually returns a base-attack-bonus/save row for. Real ACG classes
    /// all cap at `MAXLEVEL:20` (see each per-class module's own doc
    /// comment, cross-checked against `acg_classes.lst`), so this equals
    /// `chassis_rows_expected` for every class today.
    pub chassis_rows_wired: u8,
    /// The class's real `MAXLEVEL` ceiling (`acg_classes.lst`).
    pub chassis_rows_expected: u8,
    /// Count of distinct named/narrative class-feature records (Arcane
    /// Exploit, Bloodline, Martial Flexibility, Hunter's Trick, Studied
    /// Combat, Spirit, Raging Song, Sneak Attack talents, Panache,
    /// Blessings, ...) this repo has independent wired computation logic
    /// for. Zero for every ACG class except Skald, Bloodrager, Brawler,
    /// Hunter, Arcanist, Warpriest, and Slayer as of the v0.6 alpha
    /// swarm's first through seventh APG/ACG class-specific closures
    /// (risks item 8): Skald's 1st-level Raging Song song type, Inspired
    /// Rage, Bloodrager's Bloodrage, Brawler's AC Bonus, Hunter's Animal
    /// Companion, Warpriest's Blessings + Sacred Weapon, and Slayer's
    /// Sneak Attack + Trap Sense + Trapfinding + Track are now genuinely
    /// wired (`pilot_compute::ground_or_block_skald_inspired_rage`,
    /// `pilot_compute::ground_or_block_bloodrager_bloodrage`,
    /// `pilot_compute::ground_brawler_ac_bonus_and_defer_the_rest`,
    /// `pilot_compute::ground_hunter_animal_companion_and_defer_the_rest`,
    /// `pilot_compute::ground_or_block_warpriest_class_features`,
    /// `pilot_compute::ground_or_block_slayer_class_features`)
    /// -- see `class_coverage`'s own branches for each. Every other ACG
    /// class remains at 0: SD-22 Epic 4 deliberately scoped its ingest to the
    /// BAB/save chassis only (see e.g. `class_arcanist.rs`'s own doc
    /// comment), and no follow-on cycle has since ingested
    /// `acg_abilities_class.lst`'s per-level feature blocks for any other
    /// ACG class.
    ///
    /// Arcanist counts 2, not 1, unlike every prior closure this session:
    /// Arcane Reservoir AND Spells Prepared
    /// (`pilot_compute::ground_or_block_arcanist_class_features`/
    /// `ground_arcanist_prepared_spellbook`) are both real, distinct
    /// `KEY:Arcanist ~ ...` records with genuine backing logic now.
    /// `Cantrips` is a real, distinct KEY record too (verified directly:
    /// "prepare N cantrips, cast like any spell but don't consume a slot,
    /// not expended when cast"), but its own DISTINGUISHING mechanical
    /// content (no-slot-consumption, not-expended-when-cast) is not
    /// separately implemented anywhere -- the cantrip count is grounded
    /// by the exact same generic per-spell-level table loop that grounds
    /// every other spell level (`class_spell.acg.arcanist.
    /// base_spells_per_day.spell_level_0`, same shape as `spell_level_1`/
    /// `spell_level_2`), so it does not get its own count, mirroring
    /// Bloodrager's own precedent: Greater/Mighty/Tireless Bloodrage are
    /// each their own separate `KEY:Bloodrager ~ ...` record too, layered
    /// on one base Bloodrage mechanism, and that closure correctly
    /// stayed at 1 (not 4) since they are facets of one grounded
    /// mechanism, not independently-implemented features. Applying the
    /// same discipline here keeps the "count real distinct KEY records
    /// with genuine, SEPARATELY-implemented backing logic" methodology
    /// consistent across classes rather than letting it drift per-class.
    ///
    /// Warpriest ALSO counts 2 (Blessings, Sacred Weapon), for a related
    /// but distinct reason: Warpriest's own `KEY:Warpriest ~ ...` list
    /// has NO record at all for the general prepared-spellcasting
    /// mechanism (unlike Arcanist's own "Spells Prepared") -- only
    /// `Orisons` names the 0-level-specific sub-rule, and this closure's
    /// own build does not separately implement Orisons' own
    /// distinguishing "known at will, no preparation needed" content (it
    /// grounds level 0 via the exact same unified per-spell-level table
    /// as levels 1-3), so spellcasting does not add a count here either,
    /// the same "not separately implemented" reasoning that already
    /// excluded Arcanist's own Cantrips. Destruction Blessing's own
    /// Destructive Attacks is tagged `KEY:Destruction Blessing ~ ...` in
    /// the corpus -- a DIFFERENT class prefix, not `KEY:Warpriest ~ ...`
    /// -- so it is a sub-selectable-list entry under the single
    /// `Blessings` feature slot (the same "floor, not ceiling" sub-list
    /// exclusion `named_features_expected`'s own doc comment already
    /// documents for Investigator's Discoveries), not a separate count.
    ///
    /// Slayer counts 4 (Sneak Attack, Trap Sense, Trapfinding, Track) --
    /// genuinely different from Arcanist's/Warpriest's own 2, because all
    /// four are real, distinct `KEY:Slayer ~ ...` records, each with its
    /// OWN separately-implemented formula (no shared table or mechanism
    /// links them the way Cantrips/Orisons shared their class's general
    /// spellcasting table) -- an honest 4, not folded down for
    /// consistency's own sake where the underlying mechanisms are
    /// genuinely independent.
    ///
    /// Swashbuckler counts 3 (Panache, Charmed Life, Nimble), the same
    /// "genuinely independent mechanisms" reasoning as Slayer: no shared
    /// table or formula links any pair of the three (Panache is a flat
    /// daily resource pool, Charmed Life is a level-gated, activation-
    /// budgeted save bonus, Nimble is a flat AC dodge bonus) -- three
    /// separate `KEY:Swashbuckler ~ ...` records, each with its own
    /// separately-implemented formula.
    ///
    /// Investigator counts 3 (Trapfinding, Trap Sense, Inspiration
    /// pool-size), the same "genuinely independent mechanisms" reasoning
    /// as Slayer/Swashbuckler: Trapfinding and Trap Sense are separate
    /// `BONUS:VAR` records with different (and, notably, swapped-floor)
    /// formulas, and Inspiration's pool-size fact is its own record too
    /// -- three separate `KEY:Investigator ~ ...` records, each with its
    /// own separately-implemented formula. Investigator's own prepared
    /// extract spellcasting (the Alchemist formula list) is explicitly
    /// deferred to its own follow-on slice, so it does not add a fourth
    /// count here.
    ///
    /// Shaman counts 1 (Life Spirit's own Channel ability), the same
    /// single-slot shape as Skald/Bloodrager/Hunter: the Spirit choice
    /// itself is one slot, and Channel is the only immediately-available
    /// power this closure grounds under it (Life Spirit's own higher-
    /// tier Healer's Touch/Quick Healing/Manifestation abilities, the
    /// other 9 primary spirits, Spirit Magic, Orisons, and fresh
    /// own-list spellcasting all stay deferred). With Shaman's own
    /// closure, all ten real ACG classes now have at least one genuinely
    /// wired named feature -- the match below is exhaustive over
    /// `AcgClassId`, not a wildcard fallback, since every real ACG class
    /// has a real answer now.
    ///
    /// Brawler counts 3 (AC Bonus, Brawler's Cunning, Brawler's Strike),
    /// same "genuinely independent mechanisms" reasoning as Swashbuckler/
    /// Investigator: no shared table or formula links any pair of the
    /// three (AC Bonus is a flat level-driven dodge bonus, Cunning is a
    /// flat unconditional ability-score floor, Strike is a level-gated
    /// DR-bypass progression) -- three separate `KEY:Brawler ~ ...`
    /// records, each with its own separately-implemented formula.
    /// Brawler was the third ACG class closed (AC Bonus only, `== 1`)
    /// and was deepened later (v0.6 alpha swarm, risks item 8, Brawler
    /// deepening, 2026-07-26, directed by the operator to complete
    /// in-progress classes rather than start new ones) to add Cunning
    /// and Strike, moving it out of the single-slot bucket.
    pub named_features_wired: u32,
    /// Count of distinct named class-feature records tagged
    /// `KEY:<Class> ~ ...` for this class in the real PCGen corpus's
    /// `advanced_class_guide/acg_abilities_class.lst` (SD-24 Epic 4 audit
    /// count, PCGen corpus commit
    /// `7f818006e371188e5717fd18d74d18a420747fc6`, 2026-06-17; reproduce
    /// with `grep -oE "KEY:<Class> ~ [^\t]+" acg_abilities_class.lst |
    /// sort -u | wc -l`). Mirrors `ApgClassCoverage::named_features_expected`'s
    /// own "floor, not ceiling" caveat — sub-selectable-list layers (e.g.
    /// individual Discoveries an Investigator can pick) may live under a
    /// separate chooser-list file, not counted here.
    pub named_features_expected: u32,
    /// Whether `pilot_compute.rs`'s live `compute_class_chassis` dispatch
    /// (the function the character-hub pilot flow actually calls)
    /// recognizes this class at all. `true` as of the v0.6 alpha swarm's
    /// ACG BAB/save/HP dispatch wiring cycle (risks item 8, fourth slice,
    /// mirroring the identical APG cycle): `compute_class_chassis` now has
    /// an `AcgClassId::from_class_id_str` branch dispatching to
    /// `compute_acg_class_chassis`, deliberately NOT registered with
    /// `table_class_id` (same reasoning as the APG branch -- see
    /// `compute_apg_class_chassis`'s own doc comment). This does NOT mean
    /// any ACG class reaches `Computed`: only the BAB/save/HP chassis
    /// pillar is real; class-skill lists, named features, and
    /// spellcasting remain unconditionally claim-blocked.
    pub pilot_compute_integrated: bool,
    /// Whether a `level_up::<class>` module (the SD-20 Epic 7 per-level
    /// automatic-feature-grant model CRB's 11 classes all have) exists for
    /// this class. `false` for every ACG class today — `src/rules_core/level_up/`
    /// contains only CRB per-class modules.
    pub level_up_wired: bool,
}

const fn named_features_expected(class_id: AcgClassId) -> u32 {
    // Corpus counts per this struct's own doc comment — see there for the
    // reproduction command and the exact PCGen corpus commit audited.
    match class_id {
        AcgClassId::Arcanist => 9,
        AcgClassId::Bloodrager => 19,
        AcgClassId::Brawler => 14,
        AcgClassId::Hunter => 21,
        AcgClassId::Investigator => 95,
        AcgClassId::Shaman => 10,
        AcgClassId::Skald => 20,
        AcgClassId::Slayer => 15,
        AcgClassId::Swashbuckler => 29,
        AcgClassId::Warpriest => 18,
    }
}

/// Computes `class_id`'s SD-24 Epic 4 coverage row.
pub fn class_coverage(class_id: AcgClassId) -> AcgClassCoverage {
    let chassis_rows_wired = match class_id {
        AcgClassId::Arcanist => class_arcanist::class_table().len(),
        AcgClassId::Bloodrager => class_bloodrager::class_table().len(),
        AcgClassId::Brawler => class_brawler::class_table().len(),
        AcgClassId::Hunter => class_hunter::class_table().len(),
        AcgClassId::Investigator => class_investigator::class_table().len(),
        AcgClassId::Shaman => class_shaman::class_table().len(),
        AcgClassId::Skald => class_skald::class_table().len(),
        AcgClassId::Slayer => class_slayer::class_table().len(),
        AcgClassId::Swashbuckler => class_swashbuckler::class_table().len(),
        AcgClassId::Warpriest => class_warpriest::class_table().len(),
    } as u8;
    let chassis_rows_expected = match class_id {
        AcgClassId::Arcanist => class_arcanist::MAX_SUPPORTED_LEVEL,
        AcgClassId::Bloodrager => class_bloodrager::MAX_SUPPORTED_LEVEL,
        AcgClassId::Brawler => class_brawler::MAX_SUPPORTED_LEVEL,
        AcgClassId::Hunter => class_hunter::MAX_SUPPORTED_LEVEL,
        AcgClassId::Investigator => class_investigator::MAX_SUPPORTED_LEVEL,
        AcgClassId::Shaman => class_shaman::MAX_SUPPORTED_LEVEL,
        AcgClassId::Skald => class_skald::MAX_SUPPORTED_LEVEL,
        AcgClassId::Slayer => class_slayer::MAX_SUPPORTED_LEVEL,
        AcgClassId::Swashbuckler => class_swashbuckler::MAX_SUPPORTED_LEVEL,
        AcgClassId::Warpriest => class_warpriest::MAX_SUPPORTED_LEVEL,
    };

    // v0.6 alpha swarm, risks item 8 (first through twelfth APG/ACG
    // class-specific closures): Skald's Inspired Rage, Bloodrager's
    // Bloodrage, Brawler's AC Bonus, Hunter's Animal Companion,
    // Arcanist's Arcane Reservoir + Spells Prepared, Warpriest's
    // Blessings + Sacred Weapon, Slayer's Sneak Attack + Trap Sense +
    // Trapfinding + Track, Swashbuckler's Panache + Charmed Life +
    // Nimble, Investigator's Trapfinding + Trap Sense + Inspiration
    // pool-size, and Shaman's Life Spirit Channel are the real, wired
    // named class features among all ten ACG classes today -- every
    // real ACG class now has at least one, see this field's own doc
    // comment above for why Arcanist counts 2, not 1 (and why Cantrips
    // does not add a third), why Slayer counts a genuine 4 (its four
    // sub-features are structurally independent, not facets of one
    // shared mechanism), why Swashbuckler counts a genuine 3 (same
    // "structurally independent" reasoning as Slayer), why Investigator
    // counts a genuine 5 (deepening 2026-07-26, task #8: Poison
    // Resistance and Alchemy are each their own separate
    // `KEY:Investigator ~ ...` record with genuinely independent
    // grounding logic, added on top of Trapfinding/Trap Sense/
    // Inspiration's own already-landed 3 -- both were originally
    // excluded under the same over-strict "needs a live consumer" bar
    // Skald's/Hunter's own missed wins corrected; Studied Combat/Studied
    // Strike stay deferred as opponent-dependent, ruled consistently
    // with Slayer's own Studied Target pending an opponent-tracking
    // pillar), and why Skald ALSO counts a genuine 3 (Inspired Rage, Damage
    // Reduction, Bardic Knowledge -- deepening 2026-07-26, task #7):
    // Damage Reduction and Bardic Knowledge are each their own separate
    // `KEY:Skald ~ ...` record with genuinely independent grounding
    // logic, the same "structurally independent" bar Slayer/
    // Swashbuckler/Investigator/Brawler already established. Both were
    // initially excluded under an over-strict "needs a live consumer"
    // bar, corrected after finding this codebase's own established
    // precedent (Bard's Bardic Knowledge, Slayer's Track/Trapfinding,
    // Barbarian's Damage Reduction) already tolerates a genuinely
    // verified standalone flat fact with zero live consumer. Skald's
    // known-spell posture does NOT add a fourth, per the same "shares
    // the general spellcasting mechanism, not independently implemented"
    // reasoning that already excluded Oracle's known-spell posture,
    // Arcanist's Cantrips, and Warpriest's Orisons from their own counts.
    // Hunter ALSO counts a genuine 3 (Animal Companion, Wild Empathy,
    // Animal Focus -- deepening 2026-07-26, task #2): Wild Empathy is a
    // flat, unconditional check-modifier fact (`CHA+HunterLVL`), and
    // Animal Focus grounds its one canonical Bull option (a real STR
    // enhancement bonus, `+2`/`+4`/`+6` at levels 1/8/15) via the same
    // activation-gated choice-recognition shape Judgment/Mutagen already
    // proved; Hunter's own known-spell posture (still deferred) would not
    // add a fourth even if built, per the same spellcasting-sharing
    // convention. And why Warpriest counts 2 (Blessings, Sacred Weapon),
    // for a related but distinct reason: Warpriest's own
    // `KEY:Warpriest ~ ...` list has NO record at all for the general
    // prepared-spellcasting mechanic (unlike Arcanist's own "Spells
    // Prepared" record) -- only `Orisons` names the 0-level-specific
    // sub-rule, and this closure's own build does not separately
    // implement Orisons' own distinguishing "known at will, no
    // preparation needed" content (it grounds level 0 via the exact same
    // unified per-spell-level table as levels 1-3), the same "not
    // separately implemented" reasoning that already excluded Cantrips
    // from Arcanist's count. Destruction Blessing's own Destructive
    // Attacks is tagged `KEY:Destruction Blessing ~ ...` in the corpus,
    // a DIFFERENT class prefix, not `KEY:Warpriest ~ ...` -- it is a
    // sub-selectable-list entry under the single `Blessings` feature
    // slot (the same "floor, not ceiling" sub-list exclusion this
    // struct's own `named_features_expected` doc comment already
    // documents for Investigator's Discoveries), so it does not add a
    // separate count either.
    let named_features_wired = match class_id {
        AcgClassId::Bloodrager | AcgClassId::Shaman => 1,
        AcgClassId::Arcanist | AcgClassId::Warpriest => 2,
        AcgClassId::Swashbuckler | AcgClassId::Brawler | AcgClassId::Skald | AcgClassId::Hunter => 3,
        AcgClassId::Slayer => 4,
        AcgClassId::Investigator => 5,
    };

    AcgClassCoverage {
        class_id,
        chassis_rows_wired,
        chassis_rows_expected,
        named_features_wired,
        named_features_expected: named_features_expected(class_id),
        pilot_compute_integrated: true,
        level_up_wired: false,
    }
}

/// The full ACG per-class coverage report (SD-24 Epic 4, criterion 4.3),
/// one row per `AcgClassId::ALL` entry in ingest order.
pub fn coverage_report() -> Vec<AcgClassCoverage> {
    AcgClassId::ALL.iter().map(|&class_id| class_coverage(class_id)).collect()
}
