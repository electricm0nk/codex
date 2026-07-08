//! SD13-E1-F1 support-state matrix carrier.
//!
//! This module is the first machine-usable SD-13 control-plane surface. It carries
//! the bounded support-state matrix and its seeded current-truth rows so later
//! breadth claims update typed truth instead of improvising folklore.
//!
//! It is documentary/control-plane truth only. It deliberately does **not** compute
//! character mechanics, parse external files, project UI, serialize/persist data,
//! or promote rows. Support state and evidence tier are kept as separate axes so a
//! `Computed` row is never silently read as `Supported`.
//!
//! The seed encodes only truth already grounded by the SD-13 packet and the live
//! GE-06 repo evidence:
//! - the Human pilot race seam and the Fighter level-1 pilot chassis are `Partial`
//!   / `Computed` (proven, but with named missing semantics),
//! - the Fighter levels-2-10 row is `Partial` / `Computed`: the SD13-E3 tranche now
//!   proves Fighter levels 2 through 8 (base attack/save progression, the level-2/
//!   4/6/8 bonus-feat seams, the level-3 Armor Training 1 seam, the level-5 Weapon
//!   Training 1 attack-roll seam, and the level-7 Armor Training 2 seam), while
//!   levels 9-10 remain out of proof,
//! - the Rogue row is `Partial` / `Computed`: the SD13-E3 slice proves the
//!   deterministic Human Rogue level-1 chassis identity is recognized on the
//!   compute seam, and a later SD13-E3 pillar-grounding slice grounds the
//!   base-attack, base-save, and sneak-attack (die count only) pillars; only
//!   trapfinding remains blocked; the live GE-06 negative control
//!   (`tests/ge06_pilot_total_saves.rs::unsupported_chassis_blocks_total_saves`)
//!   keeps claim-blocking it unmodified since `defense.total_save.*` is still
//!   never computed for Rogue,
//! - the Paladin hybrid row is `Blocked` / `Computed`: the SD13-E3-F6 slice proves the
//!   deterministic Human Paladin level-1 hybrid chassis is recognized on the compute
//!   seam, but it stays blocked on the named non-spell class-feature burden and the
//!   later spell burden,
//! - the Ranger hybrid row is `Partial` / `Computed`: the SD13-E3-F6 slice proves the
//!   deterministic Human Ranger level-1 hybrid chassis is recognized on the compute
//!   seam, and the SD13-E3 Ranger decomposition slice grounds Track for real (the
//!   Survival-check bonus to follow tracks, ½ ranger level minimum 1), but the
//!   favored-enemy and combat-style pillar burdens remain named and unproven, and the
//!   later ranger spell burden (slots, source, spells known/prepared) stays deferred
//!   to SD13-E4,
//! - the Sorcerer row is `Blocked` / `Computed`: the SD13-E4-F7 slice proves the
//!   deterministic Human Sorcerer level-1 spontaneous arcane spell-bearing identity is
//!   recognized on the compute seam, but it stays blocked on the bloodline burden and the
//!   spontaneous known-spell / slot posture burden, and fabricates no spell math,
//! - the Bard row is `Blocked` / `Computed`: the SD13-E4-F7 slice proves the
//!   deterministic Human Bard level-1 spontaneous arcane spell-bearing identity is
//!   recognized on the compute seam, but it stays blocked on the bardic knowledge +
//!   bardic music chassis-class-feature burden and the spontaneous known-spell / slot
//!   posture burden, and fabricates no Bardic-class-feature math and no spell math,
//! - the Wizard row is `Blocked` / `Computed`: the SD13-E4-R3 slice proves the
//!   deterministic Human Wizard level-1 prepared arcane spell-bearing identity is
//!   recognized on the compute seam (merge receipt executed 2026-07-07), but it stays
//!   blocked on the school specialization burden and the prepared spellbook /
//!   spell-slot posture burden, and fabricates no spell math,
//! - the Cleric row is `Blocked` / `Computed`: the SD13-E4 slice proves the
//!   deterministic Human Cleric level-1 prepared divine spell-bearing identity is
//!   recognized on the compute seam, but it stays blocked on the domain / channel
//!   energy burden and the prepared divine spell posture burden, and fabricates no
//!   spell math,
//! - the Druid row is `Blocked` / `Computed`: the SD13-E4 slice proves the
//!   deterministic Human Druid level-1 prepared divine spell-bearing identity is
//!   recognized on the compute seam, but it stays blocked on the nature bond / wild
//!   empathy burden and the prepared divine spell posture burden, and fabricates no
//!   spell math,
//! - the Barbarian row is `Partial` / `Computed`: the SD13-E3 slice proves the
//!   deterministic Human Barbarian level-1 martial chassis identity is recognized
//!   on the compute seam and now grounds three of the four named martial pillar
//!   burdens as standalone explanation records (base attack, base save, fast
//!   movement), none wired into the integrated pilot surface; only the illiteracy
//!   trait burden remains unproven,
//! - the Monk row is `Partial` / `Computed`: the SD13-E3 slice proves the
//!   deterministic Human Monk level-1 martial chassis identity is recognized on the
//!   compute seam (mirroring the Barbarian pattern), and now grounds three named
//!   pillar burdens (base attack, base save, AC Bonus); two named pillar burdens
//!   (unarmed strike / Flurry of Blows, and the level-1 bonus feat grant) remain
//!   unproven,
//! - the Dwarf row is `Partial` / `Computed`: the SD13-E2 slice proves four grounded
//!   Dwarf racial trait dimensions (ability modifiers, size, speed, senses) are
//!   recognized on the compute seam, but the remaining Dwarf family surface
//!   (Stonecunning, Defensive Training, Hardy, Stability, Hatred, weapon
//!   familiarity) stays unproven,
//! - the Elf row is `Partial` / `Computed`: the SD13-E2 slice proves four grounded
//!   Elf racial trait dimensions (ability modifiers, size, speed, senses) are
//!   recognized on the compute seam, but the remaining Elf family surface (Elven
//!   Immunities, Keen Senses, weapon familiarity, bonus languages) stays unproven,
//! - the Gnome row is `Partial` / `Computed`: the SD13-E2 slice proves four
//!   grounded Gnome racial trait dimensions (ability modifiers, size, speed,
//!   senses) are recognized on the compute seam, but the remaining Gnome family
//!   surface (Defensive Training, Illusion Resistance, Hatred, Keen Senses, Gnome
//!   Magic, weapon familiarity) stays unproven,
//! - the Half-Elf row is `Partial` / `Computed`: the SD13-E2 slice proves four
//!   grounded Half-Elf racial trait dimensions (a player-chosen ability-bonus
//!   target, size, speed, senses) are recognized on the compute seam, but the
//!   remaining Half-Elf family surface (Elven Immunities, Adaptability, Keen
//!   Senses, Multitalented) stays unproven,
//! - the Half-Orc row is `Partial` / `Computed`: the SD13-E2 slice proves four
//!   grounded Half-Orc racial trait dimensions (a player-chosen ability-bonus
//!   target, size, speed, senses) are recognized on the compute seam, but the
//!   remaining Half-Orc family surface (Intimidating, Orc Ferocity, weapon
//!   familiarity) stays unproven,
//! - the Halfling row is `Partial` / `Computed`: the SD13-E2 slice proves four
//!   grounded Halfling racial trait dimensions (ability modifiers, size, speed,
//!   senses) are recognized on the compute seam, but the remaining Halfling
//!   family surface (Fearless, Halfling Luck, Keen Senses, Sure-Footed, weapon
//!   familiarity) stays unproven — with this slice, every core race row
//!   carries runtime evidence,
//! - the Human bonus-feat / ability-bonus interaction seam is `Partial` / `Computed`,
//! - every other core race and core class, and the broader non-Human interaction row
//!   remain `Unverified` / `Observed` (named by SD-13 scope only, no runtime evidence
//!   yet).

/// Current support state for a single matrix row. Kept independent from
/// [`EvidenceTier`]: how strong the evidence is does not by itself decide whether
/// the bounded claim is supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportState {
    /// Proven at the required evidence floor with no known missing semantics inside
    /// the bounded claim.
    Supported,
    /// Some required semantics are proven, but one or more named required semantics
    /// remain incomplete and visible.
    Partial,
    /// The path works only by discarding or approximating named semantics.
    Lossy,
    /// Known missing semantics or explicit claim-blocking diagnostics prevent the
    /// claim.
    Blocked,
    /// No direct evidence yet exists for the named dimension.
    Unverified,
}

/// Highest evidence tier achieved for a row, on the Codex quality-gate scale. This
/// is a separate axis from [`SupportState`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceTier {
    Observed,
    Parsed,
    Converted,
    Computed,
    OracleChecked,
    ProductVisible,
}

/// The subject a matrix row classifies. Limited to `Race`, `Class`, and
/// `Interaction` for this slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatrixSubjectType {
    Race,
    Class,
    Interaction,
}

/// Bounded evidence-freshness posture for a single row (SD13-E7-F13).
///
/// This is the audit axis for the first breadth-claim / evidence-refresh slice.
/// It is deliberately kept independent from both [`SupportState`] and
/// [`EvidenceTier`]: it records whether a row's breadth claim can currently be
/// trusted as *refreshed against its grounding evidence*, not how supported the
/// claim is or how strong the evidence tier is.
///
/// This first slice records no calendar timestamp or SLA policy. It carries only
/// the conservative, honest distinction the seeded truth can actually prove, and
/// **no variant asserts a row is currently fresh**. Every variant is explicitly
/// refresh-required until a later slice records real refresh checkpoints, so the
/// downstream audit surface can only ever conclude "refresh-required" from this
/// seed — never "all fresh" and never a bare "all stale forever" divorced from
/// per-row grounding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceFreshness {
    /// The row is anchored to a live, re-runnable proof surface (an executable
    /// GE-06 test or the deterministic compute seam). Its evidence *could* be
    /// refreshed by re-running the cited proof, but this slice records no
    /// completed refresh checkpoint, so the breadth claim still requires an
    /// explicit refresh audit before it may be trusted as current.
    RefreshableFromLiveProof,
    /// The row rests only on bounded SD-13 roster-scope naming with no runtime
    /// evidence yet. There is nothing to refresh from: it is awaiting its first
    /// grounding evidence, not merely a stale re-audit.
    AwaitingInitialEvidence,
}

impl EvidenceFreshness {
    /// Whether this posture asserts the row is currently fresh / refresh-confirmed.
    ///
    /// Both variants introduced by the first slice are refresh-required, so this is
    /// always `false` today. It exists so downstream audit derivation reads a real
    /// property instead of a hard-coded constant, and so a later slice that adds a
    /// genuine refreshed-checkpoint variant only has to flip it here.
    pub fn is_refresh_confirmed(self) -> bool {
        match self {
            EvidenceFreshness::RefreshableFromLiveProof => false,
            EvidenceFreshness::AwaitingInitialEvidence => false,
        }
    }
}

/// A single typed support-state matrix row.
///
/// String fields are `&'static str` because the seed is a fixed, deterministic,
/// in-source carrier; this slice adds no parsing, deserialization, or runtime row
/// construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SupportStateRow {
    /// Stable identifier for the row (e.g. `class.fighter.level_1_pilot`).
    pub row_id: &'static str,
    pub subject_type: MatrixSubjectType,
    /// Subject identity (e.g. `race:human`, `class:fighter`).
    pub subject_id: &'static str,
    /// The semantic or progression dimension this row classifies.
    pub dimension: &'static str,
    pub support_state: SupportState,
    pub evidence_tier: EvidenceTier,
    /// Bounded evidence-freshness / breadth-claim audit posture (SD13-E7-F13).
    /// Carrier-owned truth; downstream layers project it, never invent it.
    pub evidence_freshness: EvidenceFreshness,
    /// Real doc or repo evidence grounding the row. Never chat prose or invented
    /// receipts.
    pub grounding_ref: &'static str,
    /// Non-empty for `Blocked` (and, in later seeds, `Lossy`) rows; explains why the
    /// row is not `Supported`. Empty when the state needs no blocker/lossiness note.
    pub blocker_or_lossiness_note: &'static str,
    /// The next required uplift or the owning future slice.
    pub next_required_uplift: &'static str,
}

/// The typed SD-13 support-state matrix: an ordered carrier of rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportStateMatrix {
    pub rows: Vec<SupportStateRow>,
}

impl SupportStateMatrix {
    /// Narrow lookup helper: return the row with the given `row_id`, if present.
    pub fn row(&self, row_id: &str) -> Option<&SupportStateRow> {
        self.rows.iter().find(|r| r.row_id == row_id)
    }
}

// Grounding references. Repo-relative paths into real, present surfaces.

/// SD-13 roster/matrix authority that names the roster member and its current
/// posture. Cited by `Observed`-tier rows that have no runtime evidence yet.
const SD13_ROSTER_MATRIX_DOC: &str = "programs/codex/requirements/\
SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/\
core-roster-and-support-state-matrix.md";

/// The combined grounding reference for the Human race row, citing both the live
/// compute seam (named Human ability-bonus and bonus-feat explanation records plus
/// the SD13-E6-F3a trait-bundle recognition records) and the trait-bundle proof
/// surface that pins the four classified dimensions. Both `.contains()` consumers
/// read their respective substring from this combined literal (paladin-row idiom).
const SD13_HUMAN_ROW_GROUNDING_REF: &str =
    "src/rules_core/pilot_compute.rs +      tests/sd13_human_full_trait_bundle.rs";

/// GE-06 deterministic pilot input contract proof (race/class identity, Human
/// bonus-feat and ability-bonus selections).
const GE06_INPUT_CONTRACT_TEST: &str = "tests/ge06_pilot_input_contract.rs";

/// SD13-E3 dedicated proof surface for the bounded Human Rogue level-1 chassis
/// baseline (mirroring the Barbarian/Monk pattern): direct computed
/// chassis-recognition evidence that stays explicitly blocked on the four named
/// pillar burdens (base attack, base save, sneak attack, trapfinding).
const SD13_ROGUE_LEVEL1_TEST: &str = "tests/sd13_rogue_level1_chassis_baseline.rs";

/// SD13-E3 dedicated proof surface for the bounded Fighter level-8 milestone
/// (level-8 bonus-feat seam). This is the most specific/current proof for the
/// levels-2-10 row's grounding_ref.
const SD13_FIGHTER_LEVEL8_TEST: &str = "tests/sd13_fighter_level8_progression.rs";

/// SD13-E3-F5 dedicated proof surface for the bounded Fighter level-1 mandatory
/// milestone classification: enumerates which level-1 mandatory milestones the
/// deterministic pilot surface has proven (computed) and which remain unproven
/// for the level-10 progression matrix.
const SD13_FIGHTER_LEVEL1_MILESTONE_TEST: &str =
    "tests/sd13_fighter_level1_mandatory_milestone_classification.rs";

/// The combined grounding reference for the Paladin hybrid baseline row, citing
/// both F6 (chassis identity) and the per-burden decomposition test as one
/// literal. Both .contains() consumers (F6 test and this slice's test) read
/// their respective substring from this combined grounding reference.
const SD13_PALADIN_ROW_GROUNDING_REF: &str = "tests/sd13_hybrid_level1_chassis_baseline.rs +      tests/sd13_paladin_level1_chassis_and_spell_burden_separation.rs";

/// The combined grounding reference for the Ranger hybrid baseline row, citing
/// both F6 (chassis identity) and the Ranger-only per-pillar decomposition +
/// Track-grounding test as one literal, mirroring
/// [`SD13_PALADIN_ROW_GROUNDING_REF`]. Both .contains() consumers (the F6 test
/// and this slice's test) read their respective substring from this combined
/// grounding reference.
const SD13_RANGER_ROW_GROUNDING_REF: &str = "tests/sd13_hybrid_level1_chassis_baseline.rs + \
    tests/sd13_ranger_level1_chassis_and_class_feature_separation.rs";

/// SD13-E4-F7 dedicated proof surface for the bounded Human Sorcerer level-1 spell
/// baseline: direct computed recognition of the spontaneous arcane spell-bearing identity
/// that stays explicitly blocked on the bloodline burden and the spontaneous
/// known-spell / slot posture burden.
const SD13_SORCERER_LEVEL1_TEST: &str = "tests/sd13_sorcerer_level1_spell_baseline.rs";

/// SD13-E3 dedicated proof surface for the bounded Human Barbarian level-1 martial
/// chassis baseline: direct computed chassis-recognition evidence, plus grounded
/// base-attack, base-save, and fast-movement pillar values, that stays explicitly
/// blocked only on the remaining named illiteracy trait burden.
const SD13_BARBARIAN_LEVEL1_TEST: &str = "tests/sd13_barbarian_level1_chassis_baseline.rs";

/// SD13-E2 dedicated proof surface for the bounded Gnome race-semantics
/// recognition: direct computed recognition of four grounded PF1 Core Rulebook
/// Gnome racial trait dimensions (ability modifiers, size, speed, senses) that
/// stays explicitly honest about the remaining unproven Gnome family surface.
const SD13_GNOME_LEVEL1_TEST: &str = "tests/sd13_gnome_race_semantics_recognition.rs";

/// SD13-E2 dedicated proof surface for the bounded Half-Elf race-semantics
/// recognition: direct computed recognition of four grounded PF1 Core Rulebook
/// Half-Elf racial trait dimensions (chosen ability-bonus target, size, speed,
/// senses) that stays explicitly honest about the remaining unproven Half-Elf
/// family surface.
const SD13_HALF_ELF_LEVEL1_TEST: &str = "tests/sd13_half_elf_race_semantics_recognition.rs";

/// SD13-E2 dedicated proof surface for the bounded Half-Orc race-semantics
/// recognition: direct computed recognition of four grounded PF1 Core Rulebook
/// Half-Orc racial trait dimensions (chosen ability-bonus target, size, speed,
/// senses) that stays explicitly honest about the remaining unproven Half-Orc
/// family surface.
const SD13_HALF_ORC_LEVEL1_TEST: &str = "tests/sd13_half_orc_race_semantics_recognition.rs";

/// SD13-E2 dedicated proof surface for the bounded Halfling race-semantics
/// recognition: direct computed recognition of four grounded PF1 Core Rulebook
/// Halfling racial trait dimensions (ability modifiers, size, speed, senses)
/// that stays explicitly honest about the remaining unproven Halfling family
/// surface.
const SD13_HALFLING_LEVEL1_TEST: &str = "tests/sd13_halfling_race_semantics_recognition.rs";

/// SD13-E4-F7 dedicated proof surface for the bounded Human Bard level-1 spell
/// baseline: direct computed recognition of the spontaneous arcane spell-bearing identity
/// that stays explicitly blocked on the bardic knowledge + bardic music chassis-class-feature
/// burden and the spontaneous known-spell / slot posture burden.
const SD13_BARD_LEVEL1_TEST: &str = "tests/sd13_bard_level1_spell_baseline.rs";

/// SD13-E4-R3 dedicated proof surface for the bounded Human Wizard level-1 prepared
/// arcane spell baseline: direct computed recognition of the prepared arcane
/// spell-bearing identity that stays explicitly blocked on the school specialization
/// burden and the prepared spellbook / spell-slot posture burden.
const SD13_WIZARD_LEVEL1_TEST: &str = "tests/sd13_wizard_level1_prepared_spell_baseline.rs";

/// SD13-E4 dedicated proof surface for the bounded Human Cleric level-1 prepared
/// divine spell baseline: direct computed recognition of the prepared divine
/// spell-bearing identity that stays explicitly blocked on the domain / channel
/// energy burden and the prepared divine spell posture burden.
const SD13_CLERIC_LEVEL1_TEST: &str = "tests/sd13_cleric_level1_spell_baseline.rs";

/// SD13-E4 dedicated proof surface for the bounded Human Druid level-1 prepared
/// divine spell baseline: direct computed recognition of the prepared divine
/// spell-bearing identity that stays explicitly blocked on the nature bond / wild
/// empathy burden and the prepared divine spell posture burden.
const SD13_DRUID_LEVEL1_TEST: &str = "tests/sd13_druid_level1_spell_baseline.rs";

/// SD13-E3 dedicated proof surface for the bounded Human Monk level-1 martial
/// chassis baseline (mirroring the Barbarian pattern): direct computed
/// chassis-recognition evidence, now grounding three named martial pillar burdens
/// (base attack, base save, AC Bonus) and staying explicitly blocked on the two
/// remaining named burdens (unarmed strike / Flurry of Blows, level-1 bonus feat
/// grant).
const SD13_MONK_LEVEL1_TEST: &str = "tests/sd13_monk_level1_chassis_baseline.rs";

/// SD13-E2 dedicated proof surface for the bounded Dwarf race-semantics
/// recognition: direct computed recognition of four grounded PF1 Core Rulebook
/// Dwarf racial trait dimensions (ability modifiers, size, speed, senses) that
/// stays explicitly honest about the remaining unproven Dwarf family surface.
const SD13_DWARF_LEVEL1_TEST: &str = "tests/sd13_dwarf_bounded_race_semantics.rs";

/// SD13-E2 dedicated proof surface for the bounded Elf race-semantics
/// recognition: direct computed recognition of four grounded PF1 Core Rulebook
/// Elf racial trait dimensions (ability modifiers, size, speed, senses) that
/// stays explicitly honest about the remaining unproven Elf family surface.
const SD13_ELF_LEVEL1_TEST: &str = "tests/sd13_elf_race_semantics_recognition.rs";

/// The deterministic seeded SD-13 current-truth matrix for the E1-F1 slice.
///
/// Returns exactly 21 rows: 7 race, 12 class, and 2 interaction. The content is
/// fixed and grounded; this function performs no computation or promotion.
pub fn seeded_sd13_e1_f1_current_truth() -> SupportStateMatrix {
    SupportStateMatrix {
        rows: vec![
            // ----- Race rows (7) -----
            SupportStateRow {
                row_id: "race.human.pilot_semantics",
                subject_type: MatrixSubjectType::Race,
                subject_id: "race:human",
                dimension: "bounded Human pilot race semantics: the named Human \
                            ability-bonus (Strength) and Human bonus-feat (Dodge) \
                            selections exercised by the GE-06 deterministic proof, plus \
                            the SD13-E6-F3a classified trait bundle (size, speed, senses, \
                            extra skill ranks)",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_HUMAN_ROW_GROUNDING_REF,
                blocker_or_lossiness_note: "the deterministic pilot grounds the named Human \
                    ability-bonus and bonus-feat pressure, and the SD13-E6-F3a trait bundle \
                    (size, speed, senses, extra skill ranks) is classified explicitly as \
                    recognition records that ground no numeric contribution; the remaining \
                    PF1 Standard Human racial trait surface (alternate Human racial traits, \
                    variant Humans, half-Human heritages, and ruleset-level effects outside \
                    the named deterministic pilot) remains unverified",
                next_required_uplift: "classify the remaining PF1 Standard Human racial trait \
                    surface (alternate Human racial traits, variant Humans, half-Human \
                    heritages) explicitly, or ground a first computed Human trait mechanic \
                    from the classified bundle (e.g. extra skill ranks into a bounded \
                    skill-rank engine)",
            },
            SupportStateRow {
                row_id: "race.dwarf.bounded_semantics",
                subject_type: MatrixSubjectType::Race,
                subject_id: "race:dwarf",
                dimension: "bounded Dwarf race semantics: four grounded PF1 Core Rulebook \
                            Dwarf racial trait dimensions (ability modifiers, size, speed, \
                            senses) recognized on the compute seam, with the remaining Dwarf \
                            family surface still unproven",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_DWARF_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E2 leaves direct computed evidence that four \
                    Dwarf race-semantic families are recognized on the compute seam (ability \
                    modifiers: +2 Constitution / -2 Charisma; size: Medium; speed: 20 ft, \
                    never reduced by armor or encumbrance; senses: Darkvision 60 ft), but the \
                    remaining families stay unproven: skill or derived-stat modifiers \
                    (Stonecunning), Defensive Training, Hardy, Stability, Hatred, and weapon \
                    familiarity. PF1 core Dwarves gain no racial bonus feat, so that family is \
                    not applicable rather than unproven. No numeric mechanical contribution is \
                    fabricated for any of the four recognized dimensions.",
                next_required_uplift: "later SD13-E2 slice grounding one or more of the \
                    remaining Dwarf families (Stonecunning, Defensive Training, Hardy, \
                    Stability, Hatred, weapon familiarity) as a real computed contribution",
            },
            SupportStateRow {
                row_id: "race.elf.bounded_semantics",
                subject_type: MatrixSubjectType::Race,
                subject_id: "race:elf",
                dimension: "bounded Elf race semantics: four grounded PF1 Core Rulebook Elf \
                            racial trait dimensions (ability modifiers, size, speed, senses) \
                            recognized on the compute seam, with the remaining Elf family \
                            surface still unproven",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_ELF_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E2 leaves direct computed evidence that four \
                    Elf race-semantic families are recognized on the compute seam (ability \
                    modifiers: +2 Dexterity / -2 Constitution; size: Medium; speed: 30 ft; \
                    senses: low-light vision), but the remaining families stay unproven: Elven \
                    Immunities (sleep immunity, enchantment save bonus), Keen Senses (Perception \
                    bonus), weapon familiarity (longbow, composite longbow, longsword, rapier, \
                    shortbow, composite shortbow), and bonus language grants. PF1 core Elves \
                    gain no racial bonus feat, so that family is not applicable rather than \
                    unproven. No numeric mechanical contribution is fabricated for any of the \
                    four recognized dimensions.",
                next_required_uplift: "later SD13-E2 slice grounding one or more of the \
                    remaining Elf families (Elven Immunities, Keen Senses, weapon familiarity, \
                    bonus languages) as a real computed contribution",
            },
            SupportStateRow {
                row_id: "race.gnome.bounded_semantics",
                subject_type: MatrixSubjectType::Race,
                subject_id: "race:gnome",
                dimension: "bounded Gnome race semantics: four grounded PF1 Core Rulebook \
                            Gnome racial trait dimensions (ability modifiers, size, speed, \
                            senses) recognized on the compute seam, with the remaining Gnome \
                            family surface still unproven",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_GNOME_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E2 leaves direct computed evidence that four \
                    Gnome race-semantic families are recognized on the compute seam (ability \
                    modifiers: +2 Constitution / -2 Strength; size: Small; speed: 20 ft; \
                    senses: low-light vision), but the remaining families stay unproven: \
                    Defensive Training, Illusion Resistance, Hatred, Keen Senses, Gnome Magic, \
                    and weapon familiarity. PF1 core Gnomes gain no racial bonus feat, so that \
                    family is not applicable rather than unproven. No numeric mechanical \
                    contribution is fabricated for any of the four recognized dimensions.",
                next_required_uplift: "later SD13-E2 slice grounding one or more of the \
                    remaining Gnome families (Defensive Training, Illusion Resistance, Hatred, \
                    Keen Senses, Gnome Magic, weapon familiarity) as a real computed \
                    contribution",
            },
            SupportStateRow {
                row_id: "race.half_elf.bounded_semantics",
                subject_type: MatrixSubjectType::Race,
                subject_id: "race:half-elf",
                dimension: "bounded Half-Elf race semantics: four grounded PF1 Core Rulebook \
                            Half-Elf racial trait dimensions (chosen ability-bonus target, \
                            size, speed, senses) recognized on the compute seam, with the \
                            remaining Half-Elf family surface still unproven",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_HALF_ELF_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E2 leaves direct computed evidence that four \
                    Half-Elf race-semantic families are recognized on the compute seam (a \
                    player-chosen +2 ability-bonus target, mirroring the Human ability-bonus \
                    mechanic's shape rather than a fixed pair; size: Medium; speed: 30 ft; \
                    senses: low-light vision), but the remaining families stay unproven: Elven \
                    Immunities (sleep immunity, enchantment save bonus), Adaptability (a bonus \
                    Skill Focus feat), Keen Senses (Perception bonus), and Multitalented \
                    (dual favored classes). No numeric mechanical contribution is fabricated \
                    beyond the already-computed ability modifier for the chosen target.",
                next_required_uplift: "later SD13-E2 slice grounding one or more of the \
                    remaining Half-Elf families (Elven Immunities, Adaptability, Keen Senses, \
                    Multitalented) as a real computed contribution",
            },
            SupportStateRow {
                row_id: "race.half_orc.bounded_semantics",
                subject_type: MatrixSubjectType::Race,
                subject_id: "race:half-orc",
                dimension: "bounded Half-Orc race semantics: four grounded PF1 Core Rulebook \
                            Half-Orc racial trait dimensions (chosen ability-bonus target, \
                            size, speed, senses) recognized on the compute seam, with the \
                            remaining Half-Orc family surface still unproven",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_HALF_ORC_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E2 leaves direct computed evidence that four \
                    Half-Orc race-semantic families are recognized on the compute seam (a \
                    player-chosen +2 ability-bonus target, mirroring the Half-Elf mechanic's \
                    shape; size: Medium; speed: 30 ft; senses: Darkvision 60 ft), but the \
                    remaining families stay unproven: Intimidating (a bonus on Intimidate \
                    checks), Orc Ferocity (fighting on for one more round below 0 hit points), \
                    and weapon familiarity (orc double axe, falchion). No numeric mechanical \
                    contribution is fabricated beyond the already-computed ability modifier \
                    for the chosen target.",
                next_required_uplift: "later SD13-E2 slice grounding one or more of the \
                    remaining Half-Orc families (Intimidating, Orc Ferocity, weapon \
                    familiarity) as a real computed contribution",
            },
            SupportStateRow {
                row_id: "race.halfling.bounded_semantics",
                subject_type: MatrixSubjectType::Race,
                subject_id: "race:halfling",
                dimension: "bounded Halfling race semantics: four grounded PF1 Core Rulebook \
                            Halfling racial trait dimensions (ability modifiers, size, speed, \
                            senses) recognized on the compute seam, with the remaining \
                            Halfling family surface still unproven",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_HALFLING_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E2 leaves direct computed evidence that four \
                    Halfling race-semantic families are recognized on the compute seam \
                    (ability modifiers: +2 Dexterity / -2 Strength; size: Small; speed: 20 ft; \
                    senses: no special senses), but the remaining families stay unproven: \
                    Fearless, Halfling Luck, Keen Senses, Sure-Footed, and weapon familiarity. \
                    PF1 core Halflings gain no racial bonus feat, so that family is not \
                    applicable rather than unproven. No numeric mechanical contribution is \
                    fabricated for any of the four recognized dimensions.",
                next_required_uplift: "later SD13-E2 slice grounding one or more of the \
                    remaining Halfling families (Fearless, Halfling Luck, Keen Senses, \
                    Sure-Footed, weapon familiarity) as a real computed contribution",
            },
            // ----- Class rows (12) -----
            SupportStateRow {
                row_id: "class.fighter.level_1_pilot",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:fighter",
                dimension: "class progression through level 1 deterministic pilot surface",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_FIGHTER_LEVEL1_MILESTONE_TEST,
                blocker_or_lossiness_note: "SD13-E3-F5 widens the Fighter level-1 deterministic \
                    pilot surface into a bounded mandatory-milestone classification: the proven \
                    (computed) level-1 surface covers ability modifier, base attack bonus, base \
                    saving throws (fortitude/reflex/will), total saves with contributors, baseline \
                    melee attack bonus with contributors, defense.baseline_armor_class with \
                    contributors, selected skill modifier (class-skill + chosen rank + ability \
                    modifier + bounded armor-check penalty), pilot view-model projection, the \
                    Human race ability-bonus target and bonus-feat grant seams, prerequisite / \
                    invalid-choice blocking for the canonical Human Fighter feat selection, and \
                    explicit claim-blocking on missing chassis / wrong Fighter level / unsupported \
                    loadout. Remaining unproven Fighter level-1 mandatory milestones for the \
                    level-10 progression matrix are: hit point computation (Fighter d10 HD at \
                    level 1), a general class skill rank allocation engine beyond the named \
                    selected-skill seam, a general feat selection engine beyond the canonical \
                    Human Fighter choice seam, equipment / weapon / armor effects beyond the \
                    bounded baseline AC + BAB seam, a general feat prerequisite engine, and the \
                    level-1 prerequisites of the level-10 progression milestones (level-1 bonus- \
                    feat selection into the repeated bonus-feat cadence, level-1 BAB into the \
                    level-10 BAB climb, level-1 base saves into the level-10 save climb, and the \
                    level-1 armor / weapon selection that seeds armor-training and weapon-training \
                    progression at higher levels)",
                next_required_uplift: "SD13-E3 slice widening the bounded Fighter surface \
                    beyond level 1 toward the level-10 progression milestones (base attack \
                    bonus climb, base save climb, bonus-feat cadence, armor-training ranks, \
                    weapon-training ranks, and ability-score progression), per the bounded \
                    milestones enumerated in the L2-10 row",
            },
            SupportStateRow {
                row_id: "class.fighter.levels_2_10",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:fighter",
                dimension: "class progression across levels 2-10: bounded milestone proof \
                            for levels 2 through 8 only, with levels 9-10 still unproven",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_FIGHTER_LEVEL8_TEST,
                blocker_or_lossiness_note: "SD13-E3 proves Fighter levels 2 through 8: base \
                    attack / base save progression (the classlevel, classlevel/2+2, \
                    classlevel/3 formulas are level-generic), the level-2 bonus-feat, level-4 \
                    bonus-feat, level-6 bonus-feat, and level-8 bonus-feat progression seams, \
                    the level-3 Armor Training 1 seam, the level-5 Weapon Training 1 \
                    attack-roll half (folded into the \
                    baseline melee attack bonus for the canonical Heavy Blades group), and the \
                    level-7 Armor Training 2 seam (raises the Climb/Swim selected-skill totals \
                    by +1 each on the deterministic Chain Shirt) over the deterministic Human \
                    loadout. The Weapon Training damage-roll half stays unproven — no damage \
                    total is computed anywhere in this codebase for any Fighter level, so this \
                    is not a new gap. The generic PF1 level-4 ability-score-increase milestone \
                    needs no separate seam: the chosen ability score is trusted at face value. \
                    Levels 9-10 remain out of proof: PF1 core Fighter has no new class-feature \
                    milestone at level 9 (the bonus-feat cadence's next entry is level 10, and \
                    base attack / base save progression is already the level-generic formula \
                    this row proves), and level 10 needs only the level-10 bonus-feat cadence \
                    entry proven (the ordinary PF1 ability-score-increase milestone is already \
                    trusted at face value, like every other ability adjustment in this codebase, \
                    with no separate seam needed). Any general feat-effect/prerequisite engine \
                    also remains out of proof",
                next_required_uplift: "later SD13-E3 slice widening Fighter beyond level 8 \
                    toward the level-10 milestones: level 9 has no new PF1 Fighter-specific \
                    milestone (base attack/save progression is already level-generic and \
                    auto-covered), and level 10 needs only the level-10 bonus-feat cadence \
                    entry proven (no separate ability-score-increase seam, which is already \
                    trusted at face value)",
            },
            SupportStateRow {
                row_id: "class.rogue.bounded_progression",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:rogue",
                dimension: "bounded Rogue chassis progression: the deterministic Human Rogue \
                            level-1 chassis identity, with base-attack, base-save, and \
                            sneak-attack (die count) now grounded and only trapfinding still \
                            unproven",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_ROGUE_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E3 leaves direct computed evidence that the \
                    deterministic Human Rogue level-1 chassis identity is recognized on the \
                    compute seam. Three named pillar burdens are now grounded: base attack \
                    progression (3/4 BAB, level * 3 / 4), base save progression (good Reflex, \
                    poor Fortitude, poor Will), and sneak attack (die count only, +1d6 at level \
                    1 — damage-roll execution and the flanking / Dexterity-denial \
                    trigger-condition engine remain unproven). Of the four originally named \
                    burdens, only trapfinding remains unproven (Perception / Disable Device \
                    bonus). No mechanical math is fabricated beyond these grounded pillars and \
                    no Rogue level 2+ is proven. \
                    tests/ge06_pilot_total_saves.rs (unsupported_chassis_blocks_total_saves) \
                    still claim-blocks class:rogue:1 unmodified: the new \
                    class_chassis.rogue.base_attack_bonus / base_save.* / sneak_attack \
                    explanations are standalone records, not wired into compute_fighter_chassis, \
                    compute_total_saves, or compute_combat_baseline, so \
                    defense.total_save.* is still never computed for Rogue.",
                next_required_uplift: "later SD13-E3 slice grounding the Rogue trapfinding \
                    burden (Perception / Disable Device bonus)",
            },
            SupportStateRow {
                row_id: "class.barbarian.bounded_progression",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:barbarian",
                dimension: "bounded class progression",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_BARBARIAN_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E3 Barbarian level-1 proof surfaces the bounded \
                    martial chassis-recognition record and now grounds three of the four named \
                    martial pillar burdens as standalone explanation records: base attack \
                    progression (full BAB, classlevel = +1 at level 1), base save progression (the \
                    good Fortitude classlevel/2+2 cadence, +2 at level 1, and the poor \
                    Reflex / poor Will classlevel/3 cadence, +0 at level 1), and fast \
                    movement (the flat +10 ft. land speed extension value while wearing no heavy \
                    armor and carrying no heavy load — no armor/encumbrance-state check engine is \
                    grounded, none exists anywhere in this codebase yet). None of these three are \
                    wired into the integrated base_attack_bonus/base-saves/speed totals, so the \
                    integrated pilot surface still reports a blocked posture. The slice remains \
                    explicitly blocked only on the illiteracy trait. No rage execution, weapon \
                    familiarity, or level-2+ martial progression is claimed",
                next_required_uplift: "ground the illiteracy trait engine, and wire the grounded \
                    base-attack / base-save / fast-movement values into the integrated pilot \
                    surface, later widening into rage execution and level-2+ martial progression",
            },
            SupportStateRow {
                row_id: "class.bard.progression_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:bard",
                dimension: "bounded spell-bearing class progression: the deterministic Human \
                            Bard level-1 spell baseline, with the bardic knowledge + bardic \
                            music chassis-class-feature burden and the spontaneous known-spell \
                            / slot posture burden still blocked",
                support_state: SupportState::Blocked,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_BARD_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E4-F7 leaves direct computed evidence that the \
                    deterministic Human Bard level-1 spontaneous arcane spell-bearing identity is \
                    recognized on the compute seam, but the row stays blocked: the bardic knowledge \
                    competence bonus on Knowledge checks (half Bard level + INT modifier) and the \
                    bardic music performance family (inspire courage and later performances) are not \
                    implemented, and the spontaneous spell burden (spontaneous spells known, spells \
                    per day, bonus spell slots from CHA, spell save DCs, school choice, prepared \
                    posture) is not computed. No Bardic-class-feature math and no spell math is \
                    fabricated and no Bard level 2+ is proven",
                next_required_uplift: "SD13-E4 Bard chassis-class-feature (bardic knowledge, \
                    bardic music) and spontaneous spell-slot slice, then level-2+ progression",
            },
            SupportStateRow {
                row_id: "class.cleric.progression_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:cleric",
                dimension: "bounded spell-bearing class progression: the deterministic Human Cleric \
                            level-1 prepared divine spell baseline, with the domain / channel energy \
                            burden and the prepared divine spell posture burden still blocked",
                support_state: SupportState::Blocked,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_CLERIC_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E4 leaves direct computed evidence that the \
                    deterministic Human Cleric level-1 prepared divine spell-bearing identity is \
                    recognized on the compute seam, but the row stays blocked: the domain and channel \
                    energy burden (two chosen domains, domain spells, domain powers, channel energy) \
                    is not implemented, and the prepared divine spell posture burden (spells prepared \
                    from the full Cleric list, spontaneous cure/inflict conversion, spell slots per \
                    day, bonus spells from a high Wisdom, spell save DCs) is not computed. No spell \
                    math is fabricated and no Cleric level 2+ is proven",
                next_required_uplift: "SD13-E4 Cleric domain / channel energy and prepared divine \
                    spell slice, then level-2+ progression",
            },
            SupportStateRow {
                row_id: "class.druid.progression_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:druid",
                dimension: "bounded spell-bearing class progression: the deterministic Human Druid \
                            level-1 prepared divine spell baseline, with the nature bond / wild \
                            empathy burden and the prepared divine spell posture burden still blocked",
                support_state: SupportState::Blocked,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_DRUID_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E4 leaves direct computed evidence that the \
                    deterministic Human Druid level-1 prepared divine spell-bearing identity is \
                    recognized on the compute seam, but the row stays blocked: the nature bond and \
                    wild empathy burden (nature bond choice between an animal companion and a domain, \
                    nature sense, wild empathy) is not implemented, and the prepared divine spell \
                    posture burden (spells prepared from the full Druid list, spontaneous summon \
                    nature's ally conversion, spell slots per day, bonus spells from a high Wisdom, \
                    spell save DCs) is not computed. No spell math is fabricated and no Druid level \
                    2+ is proven",
                next_required_uplift: "SD13-E4 Druid nature bond / wild empathy and prepared divine \
                    spell slice, then level-2+ progression",
            },
            SupportStateRow {
                row_id: "class.monk.bounded_progression",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:monk",
                dimension: "bounded Monk martial chassis progression: the deterministic Human \
                            Monk level-1 martial chassis identity, with base-attack, base-save, \
                            and AC Bonus now grounded, and unarmed-strike/Flurry-of-Blows and the \
                            level-1 bonus feat grant still unproven",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_MONK_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E3 leaves direct computed evidence that the \
                    deterministic Human Monk level-1 martial chassis identity is recognized on \
                    the compute seam, and now grounds three named pillar burdens: base attack \
                    progression (3/4 BAB), base save progression (good Fortitude, Reflex, and \
                    Will), and AC Bonus (Wisdom-to-AC, the flat level-1 value only). Two named \
                    pillar burdens remain unproven: unarmed strike damage die and Flurry of Blows, \
                    and the level-1 bonus feat grant from the restricted Monk feat list. No \
                    martial math beyond the three grounded pillars is fabricated and no Monk \
                    level 2+ is proven",
                next_required_uplift: "later SD13-E3 slice grounding one or both of the two \
                    remaining named Monk martial pillar burdens (unarmed strike / Flurry of \
                    Blows, and the level-1 bonus feat grant)",
            },
            SupportStateRow {
                row_id: "class.paladin.hybrid_chassis_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:paladin",
                dimension: "bounded hybrid class progression: the deterministic Human \
                            Paladin level-1 chassis baseline, with smite evil's uses-per-day / \
                            attack-bonus / damage-bonus formula grounded and the lay on hands / \
                            divine grace / mercy per-feature burdens plus the partial-caster \
                            spell burden still blocked",
                support_state: SupportState::Blocked,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_PALADIN_ROW_GROUNDING_REF,
                blocker_or_lossiness_note: "SD13-E3/E4 leaves direct computed evidence that the \
                    deterministic Human Paladin level-1 hybrid chassis is recognized on the compute \
                    seam AND that its non-spell class-feature burden is now split into per-feature \
                    blockers (smite evil / lay on hands / divine grace / mercy) instead of the single \
                    combined F6 string. Of those four, smite evil is now grounded for real: uses per \
                    day = 1, attack-roll bonus = Charisma modifier (if positive), damage bonus = \
                    paladin level (PF1 Core Rulebook), computed against the deterministic fixture as \
                    1 / +2 / +1 at level 1; this grounds only that flat numeric formula, not \
                    alignment/evil-subtype target resolution or evil-outsider/dragon/undead damage \
                    doubling. The row stays blocked: lay on hands, divine grace, and mercy remain \
                    unproven per-feature chassis burdens, and the later partial-caster spell burden \
                    (Paladin is a divine partial caster in PF1 Core Rulebook: effective caster level = \
                    paladin level - 2, spell slots first available at level 2) is deferred to SD13-E4. \
                    No Paladin level 2+ is proven. The F6 hybrid baseline, the F6 hybrid blockers, and \
                    the F6 hybrid chassis recognition explanation all remain in place; this slice only \
                    adds per-burden granularity and the one grounded smite-evil formula next to them",
                next_required_uplift: "ground the lay on hands / divine grace / mercy per-feature \
                    chassis burdens, then the SD13-E4 paladin partial-caster spell burden slice, then \
                    paladin level-2+ progression",
            },
            SupportStateRow {
                row_id: "class.ranger.hybrid_chassis_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:ranger",
                dimension: "bounded hybrid class progression: the deterministic Human \
                            Ranger level-1 chassis baseline, with Track grounded for real \
                            and the favored-enemy / combat-style pillar burdens and the \
                            later spell burden still blocked",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_RANGER_ROW_GROUNDING_REF,
                blocker_or_lossiness_note: "SD13-E3-F6 leaves direct computed evidence that the \
                    deterministic Human Ranger level-1 hybrid chassis is recognized on the compute \
                    seam, AND the SD13-E3 Ranger decomposition slice grounds Track (the \
                    skill/tracking pillar: a bonus on Survival checks to follow tracks equal to \
                    max(ranger level / 2, 1), i.e. +1 at level 1) for real. The row is Partial, not \
                    Supported: the favored enemy pillar (favored-enemy type and its Bluff / \
                    Knowledge / Perception / Sense Motive / Survival / weapon-damage bonuses) and the \
                    combat style pillar (the level-1 style choice and its level-2 bonus-feat grant) \
                    remain named and unproven, and the later ranger spell burden (spell slots, spell \
                    source, spells known/prepared) is still deferred to SD13-E4. No Ranger level 2+ \
                    is proven",
                next_required_uplift: "SD13-E3 ranger favored-enemy and combat-style grounding \
                    slice, then SD13-E4 ranger spell burden",
            },
            SupportStateRow {
                row_id: "class.sorcerer.progression_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:sorcerer",
                dimension: "bounded spell-bearing class progression: the deterministic Human \
                            Sorcerer level-1 spell baseline, with the bloodline burden and the \
                            spontaneous known-spell / slot posture burden still blocked",
                support_state: SupportState::Blocked,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_SORCERER_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E4-F7 leaves direct computed evidence that the \
                    deterministic Human Sorcerer level-1 spontaneous arcane spell-bearing identity is \
                    recognized on the compute seam, but the row stays blocked: the bloodline burden \
                    (bloodline selection, level-1 bloodline power, bloodline arcana, bonus \
                    spells/feats/skills) is not implemented, and the spontaneous spell burden \
                    (spontaneous spells known, spell slots per day, bonus spell slots, spell save DCs) \
                    is not computed. No spell math is fabricated and no Sorcerer level 2+ is proven",
                next_required_uplift: "SD13-E4 Sorcerer bloodline and spontaneous spell-slot slice, \
                    then level-2+ progression",
            },
            SupportStateRow {
                row_id: "class.wizard.progression_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:wizard",
                // The SD13-E4-R3 merge receipt executed after the tranche 2.6
                // closeout merged to develop (2026-07-07): the row carries the
                // post-merge posture the slice's proof surface pinned as its
                // merge-receipt obligation.
                dimension: "bounded spell-bearing class progression: the deterministic Human \
                            Wizard level-1 prepared arcane spell baseline, with the school \
                            specialization burden and the prepared spellbook / spell-slot \
                            posture burden still blocked",
                support_state: SupportState::Blocked,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_WIZARD_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E4-R3 leaves direct computed evidence that the \
                    deterministic Human Wizard level-1 prepared arcane spell-bearing identity is \
                    recognized on the compute seam, but the row stays blocked: the school \
                    specialization burden (specialization choice, two opposed schools, specialty \
                    school bonus spell slot) is not implemented, and the prepared spell posture \
                    burden (spellbook content, spells prepared per day, spell slots per day, \
                    bonus slots from high Intelligence, spell save DCs) is not computed. No \
                    spell math is fabricated and no Wizard level 2+ is proven",
                next_required_uplift: "SD13-E4 Wizard school-specialization and prepared \
                    spellbook / spell-slot slice, then level-2+ progression",
            },
            // ----- Interaction rows (2) -----
            SupportStateRow {
                row_id: "interaction.human_bonus_feat_ability_bonus.pilot_pressure",
                subject_type: MatrixSubjectType::Interaction,
                subject_id: "interaction:human-bonus-feat-ability-bonus",
                dimension: "named Human bonus-feat and ability-bonus interaction pressure \
                            on the deterministic pilot path",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: GE06_INPUT_CONTRACT_TEST,
                blocker_or_lossiness_note: "only the named deterministic Human Fighter pilot \
                    seam is grounded: the human_bonus_feat -> feat:dodge and \
                    human_ability_bonus -> ability:strength selections now surfaced as explicit \
                    compute explanations; the general interaction-row model is not",
                next_required_uplift: "generalize the named Human pilot pressure into the \
                    interaction-row model once a second computed interaction pressure exists \
                    (per the non-Human interaction row's warrant condition)",
            },
            SupportStateRow {
                row_id: "interaction.non_human_any_class.progression_pressure",
                subject_type: MatrixSubjectType::Interaction,
                subject_id: "interaction:non-human-any-class-progression",
                dimension: "race/class interaction pressure beyond the pilot",
                support_state: SupportState::Unverified,
                evidence_tier: EvidenceTier::Observed,
                evidence_freshness: EvidenceFreshness::AwaitingInitialEvidence,
                grounding_ref: SD13_ROSTER_MATRIX_DOC,
                // SD13-E2-F15 first-slice verdict (reconciled post the SD13-E2 race
                // sweep and the SD13-E3/E4 class-recognition slices): no named
                // non-Human interaction row is required at this stage. The audit
                // basis is the SD-13 carrier itself, not invented semantics:
                //   * every non-Human race row (race.dwarf / race.elf / race.gnome /
                //     race.half_elf / race.half_orc / race.halfling) is now
                //     Partial/Computed, grounding a bounded ability-modifiers /
                //     size / speed / senses recognition bundle via its own
                //     dedicated race seam in pilot_compute.rs — but every one of
                //     those seams fires identically regardless of the chosen class;
                //     none branches on class identity, so no race x class pressure
                //     is exposed by the race side of the carrier;
                //   * every class row that has Computed evidence (Fighter L1,
                //     Fighter L2-10, Rogue, Barbarian, Monk, Paladin, Ranger,
                //     Sorcerer, Bard, Wizard, Cleric, Druid) is Blocked on a
                //     class-feature or spell burden, or Partial only on the
                //     deterministic Human pilot surface — several of those class
                //     seams (Human Fighter, Barbarian, Monk) are themselves gated
                //     to race:human specifically, so the only race x class pressure
                //     any class row exposes is already the named Human interaction
                //     row's pressure, not a distinct non-Human one;
                //   * the only Human-named interaction row
                //     (interaction.human_bonus_feat_ability_bonus.pilot_pressure)
                //     is already Partial/Computed and is explicitly distinct from
                //     this row, so no collapse is needed;
                // therefore no race x class seam is currently under-captured by the
                // separate race and class rows, and adding a named non-Human
                // interaction row today would invent a pressure the carrier cannot
                // ground. This row itself stays Unverified/Observed while the
                // verdict stands. A named non-Human interaction row becomes
                // warranted only when a class row's compute path is proven to
                // branch on a specific non-Human race identity — a distinct
                // non-Human race x class pressure that the separate race and class
                // rows do not already absorb.
                blocker_or_lossiness_note: "no named non-Human interaction row is required \
                    at this stage: every non-Human race row (race.dwarf, race.elf, race.gnome, \
                    race.half_elf, race.half_orc, race.halfling) now grounds a bounded, \
                    class-independent recognition trait bundle via its own dedicated race seam, \
                    but each seam fires identically regardless of the chosen class — no race \
                    seam branches on class identity; every class row that has Computed evidence \
                    (class.fighter.level_1_pilot, class.fighter.levels_2_10, \
                    class.rogue.bounded_progression, \
                    class.barbarian.bounded_progression, \
                    class.monk.bounded_progression, \
                    class.paladin.hybrid_chassis_and_spell_burden, \
                    class.ranger.hybrid_chassis_and_spell_burden, \
                    class.sorcerer.progression_and_spell_burden, \
                    class.bard.progression_and_spell_burden, \
                    class.wizard.progression_and_spell_burden, \
                    class.cleric.progression_and_spell_burden, \
                    class.druid.progression_and_spell_burden) is Blocked on a class-feature or \
                    spell burden, or Partial only on the Human deterministic pilot surface — \
                    several of these class seams are themselves gated to race:human, so their \
                    pressure is already absorbed by the named Human interaction row rather than \
                    exposing a distinct non-Human pressure; and the named Human interaction row \
                    interaction.human_bonus_feat_ability_bonus.pilot_pressure already covers the \
                    only race/class pressure the deterministic compute surface exposes today; \
                    this row itself remains Unverified/Observed while the verdict stands; a \
                    named non-Human interaction row becomes warranted only when a class row's \
                    compute path is proven to branch on a specific non-Human race identity, a \
                    distinct non-Human race x class pressure the separate rows do not already \
                    absorb",
                next_required_uplift: "add a named non-Human interaction row only when a class \
                    row's compute path is proven to branch on a specific non-Human race \
                    identity — the SD13-E2 race-semantic slices already prove non-Human race \
                    traits are computed at the compute surface, but no class row yet exposes a \
                    distinct non-Human race x class pressure that the separate race and class \
                    rows do not already absorb",
            },
        ],
    }
}
