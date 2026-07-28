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

pub mod bloodrager_spell_list;
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
pub mod shaman_spell_list;
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
    /// -- see `class_coverage`'s own branches for each.
    ///
    /// The full current roster, all ten ACG classes, none at 0:
    /// **Arcanist 3**, **Bloodrager 2**, **Shaman 2**, **Hunter 3**,
    /// **Skald 8** (task #50), **Warpriest 5**, **Slayer 6**,
    /// **Investigator 8**, **Brawler 9**, **Swashbuckler 12**.
    ///
    /// This paragraph previously ended "Every other ACG class remains at
    /// 0", describing the SD-22 Epic 4 state where the ingest was scoped
    /// to the BAB/save chassis only. That is no longer true of any ACG
    /// class -- the per-class closures ingested
    /// `acg_abilities_class.lst`'s feature blocks for all ten. Corrected
    /// 2026-07-27, the same sweep that caught the identical stale
    /// sentence in `apg/mod.rs`: a count change has to sweep the prose
    /// derived from the old counts, and no test asserts a doc comment,
    /// so this is the one place these numbers can rot silently.
    ///
    /// Arcanist counts 3, raised from 2 by task #56 (Familiar Exploit
    /// closure, 2026-07-28): Arcane Reservoir, Spells Prepared
    /// (`pilot_compute::ground_or_block_arcanist_class_features`/
    /// `ground_arcanist_prepared_spellbook`), AND the Familiar Exploit
    /// (`ground_familiar_master_benefit`, shared class-agnostic
    /// machinery already serving Witch and Shaman -- `KEY:Arcanist
    /// Exploit ~ Familiar` carries the identical `BONUS:VAR|
    /// FamiliarMasterLVL|ArcanistLVL` token, no Arcane Reservoir cost,
    /// no PRE gate) are all real, distinct `KEY:Arcanist ~ ...` /
    /// `KEY:Arcanist Exploit ~ ...` records with genuine backing logic
    /// now. `Cantrips` is a real, distinct KEY record too (verified directly:
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
    /// Brawler counts 9 (AC Bonus, Brawler's Cunning, Brawler's Strike,
    /// Martial Flexibility, Martial Training, Bonus Feats, Brawler's
    /// Flurry, Maneuver Training, Knockout) after the remaining-features
    /// closure (2026-07-27, task #5).
    ///
    /// The six new ones are INDEPENDENT and co-available -- a Brawler
    /// has all of them once each is granted -- so each earns its own
    /// slot under the variant-vs-independent test, the same shape as
    /// Swashbuckler's deeds rather than Alchemist's mutually-exclusive
    /// Mutagen variants.
    ///
    /// Within each, multiple corpus vars fold into one slot as facets of
    /// a single mechanism: Knockout's DC and uses/day, Flurry's extra
    /// attacks and attack penalty, and Martial Training's three
    /// level-equivalence vars. Maneuver Training's canonical Bull Rush
    /// bonus folds into Maneuver Training rather than counting, being a
    /// sub-pick under it.
    ///
    /// Swashbuckler counts 12 (Panache, Charmed Life, Nimble, Finesse,
    /// Weapon Training, Bonus Feats, and all SIX deeds carrying real
    /// magnitudes: Derring-Do, Dodging Panache, Precise Strike, Bleeding
    /// Wound, Deadly Stab, Stunning Stab). Ruled by the lead 2026-07-27.
    ///
    /// **This ruling refines the counting rule itself, and the refinement
    /// matters more than the number.** The corpus KEY prefix is a proxy,
    /// not the actual test. The real question is VARIANT vs INDEPENDENT:
    ///
    /// - Alchemist's Mutagen stat-variants (`Alchemist ~ Mutagen Strength
    ///   (First)` and siblings) are class-namespaced exactly like deeds,
    ///   yet correctly fold into ONE Mutagen slot -- they are mutually
    ///   exclusive answers to a single "which stat do I boost" choice,
    ///   the same shape as Judgment's sub-types.
    /// - Swashbuckler's deeds are the opposite: independent,
    ///   simultaneously-available abilities with genuinely different
    ///   effects (bleed damage / a death effect on a critical / a stun),
    ///   ALL of which a qualified Swashbuckler has at once. That is Stern
    ///   Gaze / Track / Monster Lore's shape, which counts separately.
    ///
    /// Deadly Stab and Stunning Stab sharing a byte-identical DC formula
    /// is coincidental RAW design, NOT evidence of one mechanism. The
    /// parent `KEY:Swashbuckler ~ Deeds` record adds no slot of its own:
    /// it carries no magnitude, only the shared `SwashbucklerDeedsLVL`
    /// tracking variable.
    ///
    /// So a same-prefix record folds when it is a mutually-exclusive
    /// variant, and counts when it is an independent co-available
    /// feature -- check the DESC text, not just the key.
    ///
    /// Bloodrager counts 2 (Bloodrage + Spells), raised from 1 by the
    /// spellcasting-blocker closure (2026-07-27, task #1).
    /// `KEY:Bloodrager ~ Spells` is a real `CATEGORY:Special Ability`
    /// class-feature record whose numeric content -- the spells-per-day
    /// and spells-known tables -- is now genuinely implemented, so it
    /// earns its own slot. This is a real difference from Warpriest,
    /// whose own `KEY:Warpriest ~ ...` list has NO general-spellcasting
    /// record at all (see below); the presence or absence of that record
    /// is what decides it, not whether the class casts.
    ///
    /// Bloodrager's class-skill list does NOT add a third, even though
    /// it too was grounded by that closure and `KEY:Bloodrager ~ Class
    /// Skills` exists: that record is `CATEGORY:Internal`, a PCGen
    /// encoding artifact for how ACG attaches class skills (there is no
    /// `CSKILL:` token on any ACG class line), not a player-facing named
    /// class feature. Warpriest carries the identical
    /// `KEY:Warpriest ~ Class Skills` record and its own count of 5
    /// excludes it, so excluding it here keeps the two consistent. The
    /// scoping doc for task #1 suggested 3 by counting class skills;
    /// this is a deliberate correction to that, on the established
    /// precedent.
    ///
    /// Warpriest counts 5 (Blessings, Sacred Weapon, Fervor, Channel
    /// Energy, Sacred Armor). The original closure landed the first two;
    /// the partial re-scope (deepening 2026-07-26, task #9) added the
    /// other three, each a real, distinct `KEY:Warpriest ~ ...` record
    /// with its own separately-implemented formula -- the same
    /// "structurally independent mechanisms" bar Slayer/Swashbuckler/
    /// Investigator already established. Channel Energy earns its own
    /// count on the strength of its own `WarpriestChannelEnergyDC`
    /// formula, even though it borrows Fervor's dice for its magnitude.
    ///
    /// Two exclusions, both deliberate. Warpriest's own
    /// `KEY:Warpriest ~ ...` list has NO record at all for the general
    /// prepared-spellcasting mechanism (unlike Arcanist's own "Spells
    /// Prepared") -- only `Orisons` names the 0-level-specific sub-rule,
    /// and this closure's own build does not separately implement
    /// Orisons' own distinguishing "known at will, no preparation
    /// needed" content (it grounds level 0 via the exact same unified
    /// per-spell-level table as levels 1-3), so spellcasting does not add
    /// a count, the same "not separately implemented" reasoning that
    /// already excluded Arcanist's own Cantrips. And Destruction
    /// Blessing's own Destructive Attacks and Strength Blessing's own
    /// Strength Surge are tagged `KEY:Destruction Blessing ~ ...` /
    /// `KEY:Strength Blessing ~ ...` in the corpus -- a DIFFERENT class
    /// prefix, not `KEY:Warpriest ~ ...` -- so both are
    /// sub-selectable-list entries under the single `Blessings` feature
    /// slot (the same "floor, not ceiling" sub-list exclusion
    /// `named_features_expected`'s own doc comment already documents for
    /// Investigator's Discoveries), not separate counts.
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
    /// Shaman counts 2 (the Spirit slot + Spirit Animal), raised from 1
    /// by task #12.
    ///
    /// **The other 9 spirits do NOT each earn a slot**, even though all
    /// ten are now genuinely grounded through their own
    /// immediately-available base ability. A shaman selects exactly one
    /// Spirit, so the ten are mutually-exclusive VARIANTS of a single
    /// choice, not independent co-available features -- the same
    /// reasoning that keeps Oracle's ten Mysteries at one "Mystery slot"
    /// and Witch's ~20 hexes at one Hex slot. (Contrast Oracle's curses,
    /// which DO each earn a slot: they are top-level `KEY:Oracle ~ ...`
    /// records with independent formulas, whereas the spirits are
    /// mechanism-namespaced `KEY:Shaman Spirit ~ ...` records.) Grounding
    /// nine more variants widens what the one slot covers; it does not
    /// add slots.
    ///
    /// **Spirit Animal earns the second slot** because it is a genuinely
    /// separate class feature, not a variant of the Spirit choice: its
    /// own `KEY:Shaman ~ Spirit Animal` record grants the shared Standard
    /// Familiar List and lands a real magnitude on the computed max-HP
    /// total. That bump was owed from the familiar closure (`8e47479a`)
    /// and simply never landed.
    ///
    /// **Spellcasting is deliberately NOT counted**, matching the same
    /// "not independently implemented" reasoning that already excludes
    /// Arcanist's Cantrips and Warpriest's Orisons from their own counts.
    ///
    /// Still deferred and uncounted: each Spirit's three higher-tier
    /// abilities (the `ShamanSpiritGreater`/`ShamanSpiritTrue`/
    /// Manifestation tiers), Spirit Magic, and the Hex/Spirit Hex
    /// chooser-list. The match below is exhaustive over `AcgClassId`, not
    /// a wildcard fallback, since every real ACG class has a real answer.
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
    // comment above for why Arcanist counts 3, not 1 (raised to 2 for
    // Spells Prepared, then to 3 by task #56's Familiar Exploit; and why
    // Cantrips does not add a fourth), why Slayer counts a genuine 4 (its four
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
    //
    // Skald's own count stayed at 3, NOT 5, after task #54 grounded
    // Raging Climber's and Raging Swimmer's real self-use magnitude (they
    // now land on the real Climb/Swim totals in pilot_compute.rs): both
    // are tagged `KEY:Rage Power ~ Raging Climber` / `~ Raging Swimmer`
    // in the corpus, a DIFFERENT class-prefix namespace shared with
    // Barbarian, not `KEY:Skald ~ ...`, so they fold into the
    // `KEY:Skald ~ Rage Powers` slot rather than adding their own count
    // -- the exact same "different corpus class-prefix, fold into the
    // slot" precedent Warpriest's own grounded Blessing minor powers
    // (Destructive Attacks/Strength Surge, tagged `KEY:Destruction
    // Blessing ~ ...`/`KEY:Strength Blessing ~ ...`) already established.
    // #54 explicitly left the `KEY:Skald ~ Rage Powers` slot itself (the
    // pool-count formula, `RagePowersLVL/3`, and the ally-granting
    // "shared-list access") genuinely ungrounded -- it only grounds two
    // of the wider 60-record Rage Powers family's own canonical-
    // narrowing representatives' magnitude, per `rage-powers-canonical-
    // narrowing-scoping.md`, not the slot mechanism that grants them.
    //
    // Skald rose again from 3 to 8 with task #50, grounding exactly that
    // still-open slot mechanism plus four more standalone records:
    // Well-Versed (flat +4 save bonus, level-gated at 2nd), Spell
    // Kenning (uses/day pool, self-gating `(1+SkaldLVL)/6`), Lore Master
    // (uses/day pool, a genuine two-argument `min((SkaldLVL-1)/6,3)`),
    // Versatile Performance (a genuinely single-argument
    // `min((SkaldLVL+3)/5)` in the raw corpus -- ground the term itself,
    // not an invented second cap), and Rage Powers (Skald's own
    // pool-SIZE count only, `RagePowersLVL/3` with `RagePowersLVL` set
    // unconditionally from `SkaldLVL`; individual rage-power selection/
    // execution beyond Raging Climber/Swimmer is still a separate,
    // deferred concern) are each their own separate `KEY:Skald ~ ...`
    // record with independently-implemented formula logic. All five
    // level-gate self-consistently with the real corpus's own per-level
    // `ABILITY:...AUTOMATIC` grant rows in `acg_classes.lst`. Rage
    // Powers' own pool-size record is additive with #54's Raging
    // Climber/Swimmer grounding, not a duplicate of it -- the pool-size
    // formula answers "how many rage powers does this skald know," while
    // #54 answers "what do two specific, canonically-narrowed rage
    // powers do once selected"; neither implies the other.
    // Hunter ALSO counts a genuine 3 (Animal Companion, Wild Empathy,
    // Animal Focus -- deepening 2026-07-26, task #2): Wild Empathy is a
    // flat, unconditional check-modifier fact (`CHA+HunterLVL`), and
    // Animal Focus grounds its one canonical Bull option (a real STR
    // enhancement bonus, `+2`/`+4`/`+6` at levels 1/8/15) via the same
    // activation-gated choice-recognition shape Judgment/Mutagen already
    // proved; Hunter's own known-spell posture (still deferred) would not
    // add a fourth even if built, per the same spellcasting-sharing
    // convention. And why Warpriest counts a genuine 5 (Blessings,
    // Sacred Weapon, Fervor, Channel Energy, Sacred Armor -- partial
    // re-scope 2026-07-26, task #9): the last three are each their own
    // `KEY:Warpriest ~ ...` record with its own separately-implemented
    // formula. See this field's own doc comment above for the two
    // deliberate exclusions -- Warpriest's spellcasting (no
    // general-mechanism record exists; only the 0-level-specific
    // `Orisons`, whose distinguishing content is not separately
    // implemented, the same reasoning that excluded Arcanist's Cantrips)
    // and both grounded Blessing minor powers (Destructive Attacks and
    // Strength Surge are tagged with a DIFFERENT class prefix,
    // `KEY:Destruction Blessing ~ ...` / `KEY:Strength Blessing ~ ...`,
    // making them sub-selectable-list entries under the single
    // `Blessings` slot).
    let named_features_wired = match class_id {
        AcgClassId::Shaman => 2,
        // Arcanist rose 2 -> 3 with task #56: the Familiar Exploit
        // (`KEY:Arcanist Exploit ~ Familiar`) dispatches into the same
        // shared, class-agnostic `ground_familiar_master_benefit` that
        // Witch and Shaman already use -- a real, distinct
        // `KEY:Arcanist Exploit ~ ...` record with its own corpus token
        // (`BONUS:VAR|FamiliarMasterLVL|ArcanistLVL`), not a facet of
        // Arcane Reservoir or Spells Prepared, so it earns its own slot
        // on top of that existing 2.
        AcgClassId::Arcanist => 3,
        // Bloodrager rose 2 -> 9 with task #42: the existing Bloodrage and
        // Spells slots plus Fast Movement, Uncanny Dodge, Improved Uncanny
        // Dodge, Blood Sanctuary, Damage Reduction, Greater Bloodrage and
        // Mighty Bloodrage. Uncanny Dodge and Improved Uncanny Dodge count
        // separately -- distinct corpus features with their own grant rows
        // and gates (2 and 5) -- even though the corpus expresses Improved
        // as a second increment to one shared counter. Greater and Mighty
        // likewise: identical tokens, separate records, separate gates.
        //
        // Bloodrager rose 9 -> 10 with task #83, which grounded THREE more
        // corpus records but earns only ONE slot. Indomitable Will counts:
        // its own `KEY:Bloodrager ~ Indomitable Will` record, own level
        // gate (14), and a real separately-computed magnitude (+4 on Will
        // saves vs enchantment while bloodraging). Blood Casting and
        // Eschew Materials deliberately do NOT count, on the same
        // "distinguishing content is not separately implemented" reasoning
        // that already excludes Warpriest's Orisons and Arcanist's
        // Cantrips: both are grounded honestly at +0 as vacuous under this
        // scope -- Blood Casting lifts a casting-while-raging restriction
        // this codebase never imposes (no concentration engine exists),
        // and Eschew Materials is a boolean feat grant against a
        // material-component economy this codebase does not model. They
        // are real, correctly-grounded records; they are not implemented
        // MECHANISMS, and this field counts mechanisms.
        AcgClassId::Bloodrager => 10,
        // Skald rose 3 -> 8 with task #50: Well-Versed, Spell Kenning,
        // Lore Master, Versatile Performance, and Rage Powers' own
        // pool-size count are each their own separate `KEY:Skald ~ ...`
        // record with independently-implemented formula logic, added on
        // top of Inspired Rage/Damage Reduction/Bardic Knowledge's
        // already-landed 3. Versatile Performance and Rage Powers ground
        // pool-SIZE counts only (not the underlying choice/execution),
        // the same "pool size, use not modelled" idiom Swashbuckler's
        // Panache and Warpriest's Blessing uses/day already established.
        AcgClassId::Skald => 8,
        AcgClassId::Hunter => 3,
        AcgClassId::Slayer => 6,
        AcgClassId::Warpriest => 5,
        // Investigator rose 8 -> 9 with task #58: Resiliency, a genuinely
        // separate `KEY:Investigator ~ Rogue Talent ~ Resiliency` record
        // (own formula, `BONUS:VAR|ResiliencyHitPoints|InvestigatorLVL`,
        // own level variable, own choice-gated recognition), drawn from
        // Investigator's own explicit 40-record Rogue Talent whitelist --
        // added on top of the already-landed Trapfinding/Trap Sense/
        // Inspiration/Poison Resistance/Alchemy/spellcasting/Studied
        // Combat/Studied Strike 8.
        AcgClassId::Investigator => 9,
        AcgClassId::Brawler => 9,
        AcgClassId::Swashbuckler => 12,
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
