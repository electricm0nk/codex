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
//! - the Fighter levels-2-10 row is `Partial` / `Computed`: the SD13-E3/SD13-E5
//!   tranches now prove Fighter levels 2 through 10 (base attack/save progression,
//!   the level-2/4/6/8/10 bonus-feat seams, the level-3 Armor Training 1 seam, the
//!   level-5 Weapon Training 1 attack-roll seam, the level-7 Armor Training 2 seam,
//!   and the level-9 Weapon Training 2 attack-roll seam), while the Weapon Training
//!   damage-roll half and Bravery remain out of proof,
//! - the Rogue row is `Partial` / `Computed`: the SD13-E3 slice proves the
//!   deterministic Human Rogue level-1 chassis identity is recognized on the
//!   compute seam, a later SD13-E3 pillar-grounding slice grounds the
//!   base-attack, base-save, and sneak-attack (die count only) pillars, and
//!   the SD13-E5 slice grounds the fourth named pillar, Trapfinding (the flat
//!   numeric Perception-to-locate-traps / Disable Device bonus plus the
//!   magic-trap-disarm statement), so no named Rogue pillar burden remains
//!   blocked; the live GE-06 negative control
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
//! - the Sorcerer row is `Partial` / `Computed`: the SD13-E4-F7 slice proves the
//!   deterministic Human Sorcerer level-1 spontaneous arcane spell-bearing identity is
//!   recognized on the compute seam, AND the SD13-E4 Sorcerer decomposition slice grounds
//!   Eschew Materials (the universal, bloodline-independent 1st-level bonus feat) for
//!   real, but the bloodline-power burden and the spontaneous known-spell / slot posture
//!   burden remain named and unproven, and no spell math is fabricated,
//! - the Bard row is `Partial` / `Computed`: the SD13-E4-F7 slice proves the
//!   deterministic Human Bard level-1 spontaneous arcane spell-bearing identity is
//!   recognized on the compute seam, the SD13-E4 Bard decomposition slice grounds
//!   Bardic Knowledge for real (the Knowledge-check competence bonus, max(bard level /
//!   2, 1)), and the SD13-E5 slice grounds the flat Bardic Performance surface (the
//!   4 + CHA-modifier rounds-per-day budget and the flat +1 Inspire Courage level-1
//!   magnitude), but the bardic performance-execution burden (start/maintain action
//!   economy, round tracking/consumption, countersong / distraction / fascinate) and
//!   the spontaneous known-spell / slot posture burden remain unproven, and no
//!   performance-execution math and no spell math is fabricated,
//! - the Wizard row is `Partial` / `Computed`: the SD13-E4-R3 slice proves the
//!   deterministic Human Wizard level-1 prepared arcane spell-bearing identity is
//!   recognized on the compute seam (merge receipt executed 2026-07-07), a later
//!   SD13-E4 decomposition slice grounds Scribe Scroll (the free, specialization-
//!   independent bonus feat every 1st-level Wizard is granted) for real, promoting
//!   the row from Blocked to Partial (mirroring the Ranger Track promotion), and the
//!   SD13-E5 specialization slice grounds the school specialization choice (canonical
//!   Evocation specialist, Necromancy and Transmutation opposed) and the flat
//!   specialist-bonus-slot count for real; the row stays blocked on the
//!   school-powers / opposed-school-cost burden and the prepared spellbook /
//!   spell-slot posture burden, and fabricates no spell math,
//! - the Cleric row is `Partial` / `Computed`: the SD13-E4 slice proves the
//!   deterministic Human Cleric level-1 prepared divine spell-bearing identity is
//!   recognized on the compute seam, a later SD13-E4 Cleric Channel Energy slice
//!   grounds Channel Energy for real (die count and uses per day), and the SD13-E5
//!   Cleric domain slice grounds the domain choice seam and the flat domain spell
//!   slot count, but the domain powers burden and the prepared divine spell posture
//!   burden remain named and unproven, and fabricates no domain power math and no
//!   spell math,
//! - the Druid row is `Blocked` / `Computed`: the SD13-E4 slice proves the
//!   deterministic Human Druid level-1 prepared divine spell-bearing identity is
//!   recognized on the compute seam, but it stays blocked on the nature bond / wild
//!   empathy burden and the prepared divine spell posture burden, and fabricates no
//!   spell math,
//! - the Barbarian row is `Partial` / `Computed`: the SD13-E3 slice proves the
//!   deterministic Human Barbarian level-1 martial chassis identity is recognized
//!   on the compute seam and grounds base attack, base save, and fast movement as
//!   standalone explanation records; the SD13-E5 slice resolves the formerly-named
//!   illiteracy burden as vacuous (the PF1 Core Rulebook Barbarian is not
//!   illiterate — illiteracy is a D&D 3.5e trait that never existed in PF1) and
//!   grounds Rage's flat numeric surface (rage rounds per day = 4 + Constitution
//!   modifier, plus the flat rage constants), values only, none wired into the
//!   integrated pilot surface; the rage-state execution engine remains the named
//!   unproven burden,
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

/// SD13-E3/E5 dedicated proof surface for the bounded Human Rogue level-1 chassis
/// baseline (mirroring the Barbarian/Monk pattern): direct computed
/// chassis-recognition evidence with all four named pillar burdens (base attack,
/// base save, sneak attack die count, trapfinding) grounded as standalone records.
const SD13_ROGUE_LEVEL1_TEST: &str = "tests/sd13_rogue_level1_chassis_baseline.rs";

/// SD13-E5 dedicated proof surface for the bounded Fighter level-9/level-10
/// milestones (Weapon Training 2 attack-roll seam, second weapon-training group
/// seam, and level-10 bonus-feat seam). This is the most specific/current proof
/// for the levels-2-10 row's grounding_ref.
const SD13_FIGHTER_LEVEL9_LEVEL10_TEST: &str = "tests/sd13_fighter_level9_level10_progression.rs";

/// The combined grounding reference for the Fighter level-1 pilot row, citing
/// both the SD13-E3-F5 mandatory-milestone classification proof (which level-1
/// mandatory milestones are proven versus unproven for the level-10 progression
/// matrix) and the SD13-E5 level-1 hit-point baseline proof (level-1 hit points
/// = maximized d10 hit die 10 + Constitution modifier as a standalone grounded
/// explanation record) as one literal (paladin-row idiom). Both `.contains()`
/// consumers read their respective substring from this combined grounding
/// reference.
const SD13_FIGHTER_LEVEL1_ROW_GROUNDING_REF: &str =
    "tests/sd13_fighter_level1_mandatory_milestone_classification.rs + \
     tests/sd13_fighter_level1_hit_point_baseline.rs";

/// The combined grounding reference for the Paladin hybrid baseline row, citing
/// F6 (chassis identity), the per-burden decomposition test, and the SD13-E5
/// effective-caster-level gate test as one literal. Each `.contains()`
/// consumer (the F6 test, the decomposition test, and this slice's test) reads
/// its respective substring from this combined grounding reference.
const SD13_PALADIN_ROW_GROUNDING_REF: &str = "tests/sd13_hybrid_level1_chassis_baseline.rs +      tests/sd13_paladin_level1_chassis_and_spell_burden_separation.rs + tests/sd13_paladin_partial_caster_effective_caster_level.rs";

/// The combined grounding reference for the Ranger hybrid baseline row, citing
/// both F6 (chassis identity) and the Ranger-only per-pillar decomposition +
/// Track / Favored-Enemy-flat-surface grounding test as one literal, mirroring
/// [`SD13_PALADIN_ROW_GROUNDING_REF`]. Both .contains() consumers (the F6 test
/// and this slice's test) read their respective substring from this combined
/// grounding reference.
const SD13_RANGER_ROW_GROUNDING_REF: &str = "tests/sd13_hybrid_level1_chassis_baseline.rs + \
    tests/sd13_ranger_level1_chassis_and_class_feature_separation.rs";

/// SD13-E4-F7 / SD13-E4 dedicated proof surface for the bounded Human Sorcerer level-1
/// spell baseline: direct computed recognition of the spontaneous arcane spell-bearing
/// identity, plus the SD13-E4 decomposition slice's grounded Eschew Materials bonus-feat
/// grant, while the bloodline-power burden and the spontaneous known-spell / slot
/// posture burden stay explicitly blocked.
const SD13_SORCERER_LEVEL1_TEST: &str = "tests/sd13_sorcerer_level1_spell_baseline.rs";

/// SD13-E3/E5 dedicated proof surface for the bounded Human Barbarian level-1
/// martial chassis baseline: direct computed chassis-recognition evidence, plus
/// grounded base-attack, base-save, fast-movement, and flat Rage pillar values
/// (rage rounds per day and the rage constants, values only) and the vacuous
/// illiteracy-burden rules correction, that stays explicitly blocked only on the
/// remaining named rage-state execution burden.
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

/// SD13-E4-F7/SD13-E4/SD13-E5 dedicated proof surface for the bounded Human Bard level-1
/// spell baseline: direct computed recognition of the spontaneous arcane spell-bearing
/// identity, the grounded Bardic Knowledge and flat Bardic Performance pillars (rounds
/// per day, inspire courage magnitude), and the still-blocked bardic
/// performance-execution burden and spontaneous known-spell / slot posture burden.
const SD13_BARD_LEVEL1_TEST: &str = "tests/sd13_bard_level1_spell_baseline.rs";

/// SD13-E4-R3 dedicated proof surface for the bounded Human Wizard level-1 prepared
/// arcane spell baseline: direct computed recognition of the prepared arcane
/// spell-bearing identity, plus a later SD13-E4 decomposition slice grounding the
/// Scribe Scroll bonus feat grant and the SD13-E5 slice grounding the school
/// specialization choice and flat specialist-bonus-slot count for real, that stays
/// explicitly blocked on the school-powers / opposed-school-cost burden and the
/// prepared spellbook / spell-slot posture burden.
const SD13_WIZARD_LEVEL1_TEST: &str = "tests/sd13_wizard_level1_prepared_spell_baseline.rs";

/// SD13-E4 dedicated proof surface for the bounded Human Cleric level-1 prepared
/// divine spell baseline: direct computed recognition of the prepared divine
/// spell-bearing identity, with Channel Energy, the domain choice seam, and the
/// flat domain spell slot count grounded for real, that stays explicitly blocked
/// on the domain powers burden and the prepared divine spell posture burden.
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
                grounding_ref: SD13_FIGHTER_LEVEL1_ROW_GROUNDING_REF,
                blocker_or_lossiness_note: "SD13-E3-F5 widens the Fighter level-1 deterministic \
                    pilot surface into a bounded mandatory-milestone classification: the proven \
                    (computed) level-1 surface covers ability modifier, base attack bonus, base \
                    saving throws (fortitude/reflex/will), total saves with contributors, baseline \
                    melee attack bonus with contributors, defense.baseline_armor_class with \
                    contributors, selected skill modifier (class-skill + chosen rank + ability \
                    modifier + bounded armor-check penalty), level-1 hit points (SD13-E5: \
                    maximized d10 hit die 10 + Constitution modifier, grounded as a standalone \
                    explanation record wired into no view-model total), pilot view-model \
                    projection, the Human race ability-bonus target and bonus-feat grant seams, \
                    prerequisite / invalid-choice blocking for the canonical Human Fighter feat \
                    selection, and explicit claim-blocking on missing chassis / wrong Fighter \
                    level / unsupported loadout. Remaining unproven Fighter level-1 mandatory \
                    milestones for the level-10 progression matrix are: the favored-class +1 hp / \
                    +1 skill-rank choice (no input surface exists for it), hit points at levels \
                    2+ and Toughness / feat hit-point interplay, a general class skill rank \
                    allocation engine beyond the named selected-skill seam, a general feat \
                    selection engine beyond the canonical Human Fighter choice seam, equipment / \
                    weapon / armor effects beyond the bounded baseline AC + BAB seam, a general \
                    feat prerequisite engine, and the level-1 prerequisites of the level-10 \
                    progression milestones (level-1 bonus-feat selection into the repeated \
                    bonus-feat cadence, level-1 BAB into the level-10 BAB climb, level-1 base \
                    saves into the level-10 save climb, and the level-1 armor / weapon selection \
                    that seeds armor-training and weapon-training progression at higher levels)",
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
                            for levels 2 through 10, with the Weapon Training damage-roll \
                            half and Bravery still unproven",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_FIGHTER_LEVEL9_LEVEL10_TEST,
                blocker_or_lossiness_note: "SD13-E3/SD13-E5 prove Fighter levels 2 through 10: \
                    base attack / base save progression (the classlevel, classlevel/2+2, \
                    classlevel/3 formulas are level-generic), the level-2, level-4, level-6, \
                    level-8, and level-10 bonus-feat progression seams (the level-10 canonical \
                    Greater Weapon Focus selection's prerequisites are honestly met by the \
                    canonical loadout), the level-3 Armor Training 1 seam, the level-5 Weapon \
                    Training 1 attack-roll half, the level-7 Armor Training 2 seam (raises the \
                    Climb/Swim selected-skill totals by +1 each on the deterministic Chain \
                    Shirt), and the level-9 Weapon Training 2 attack-roll half (rank = 1 + \
                    (level - 5) / 4: the first-group Heavy Blades bonus rises to +2, folded \
                    into the baseline melee attack bonus, and the canonical second group, Bows, \
                    is surfaced at +1 as an explanation-only seam covering no equipped weapon) \
                    over the deterministic Human loadout. The Weapon Training damage-roll half \
                    stays unproven — no damage total is computed anywhere in this codebase for \
                    any Fighter level, so this is not a new gap. Bravery stays unproven — the \
                    level-2 Fighter Will-save bonus vs fear (+1 at level 2, +2 at level 6, +3 \
                    at level 10) is absent from this codebase entirely; no Will-vs-fear total \
                    or seam exists. The generic PF1 ability-score-increase milestones need no \
                    separate seam: the chosen ability score is trusted at face value. Any \
                    general feat-effect/prerequisite engine also remains out of proof",
                next_required_uplift: "later SD13 slice grounding the remaining named Fighter \
                    class-feature burdens inside levels 2-10: the Bravery Will-vs-fear seam \
                    (+1 at level 2, +2 at level 6, +3 at level 10) and the Weapon Training \
                    damage-roll half (which first needs any damage total to exist on the \
                    compute surface)",
            },
            SupportStateRow {
                row_id: "class.rogue.bounded_progression",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:rogue",
                dimension: "bounded Rogue chassis progression: the deterministic Human Rogue \
                            level-1 chassis identity, with all four named pillars now grounded \
                            (base-attack, base-save, sneak-attack die count, and trapfinding) \
                            and the check-execution / rogue-talent / integration remainder \
                            still unproven",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_ROGUE_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E3 leaves direct computed evidence that the \
                    deterministic Human Rogue level-1 chassis identity is recognized on the \
                    compute seam, and all four named pillar burdens are now grounded: base \
                    attack progression (3/4 BAB, level * 3 / 4), base save progression (good \
                    Reflex, poor Fortitude, poor Will), sneak attack (die count only, +1d6 at \
                    level 1 — damage-roll execution and the flanking / Dexterity-denial \
                    trigger-condition engine remain unproven), and, per the SD13-E5 slice, \
                    trapfinding (the flat max(rogue level / 2, 1) bonus on Perception checks to \
                    locate traps and on Disable Device checks, +1 at level 1, plus the \
                    magic-trap-disarm statement — a check-execution engine, trap DC resolution, \
                    and a magic-trap disarm engine remain unproven). The row is Partial, not \
                    Supported: no rogue talent (a level-2+ milestone) is proven, no Rogue level \
                    2+ is proven, and no mechanical math is fabricated beyond these grounded \
                    pillars. \
                    tests/ge06_pilot_total_saves.rs (unsupported_chassis_blocks_total_saves) \
                    still claim-blocks class:rogue:1 unmodified: the \
                    class_chassis.rogue.base_attack_bonus / base_save.* / sneak_attack / \
                    trapfinding explanations are standalone records, not wired into \
                    compute_fighter_chassis, compute_total_saves, or compute_combat_baseline, \
                    so defense.total_save.* is still never computed for Rogue.",
                next_required_uplift: "later SD13 slice wiring the grounded Rogue pillar \
                    records into the integrated pilot surface (the generic chassis diagnostics \
                    still claim-block), then rogue talents and level-2+ progression",
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
                    martial chassis-recognition record and grounds base attack \
                    progression (full BAB, classlevel = +1 at level 1), base save progression (the \
                    good Fortitude classlevel/2+2 cadence, +2 at level 1, and the poor \
                    Reflex / poor Will classlevel/3 cadence, +0 at level 1), and fast \
                    movement (the flat +10 ft. land speed extension value while wearing no heavy \
                    armor and carrying no heavy load — no armor/encumbrance-state check engine is \
                    grounded, none exists anywhere in this codebase yet) as standalone explanation \
                    records. The SD13-E5 slice resolves the formerly-named illiteracy burden as \
                    vacuous — a rules correction, not an uplift: the PF1 Core Rulebook Barbarian \
                    is not illiterate; illiteracy is a D&D 3.5e Barbarian trait that never \
                    existed in PF1, documented by the grounded illiteracy_absent record — and \
                    grounds Rage's flat numeric surface: rage rounds per day (4 + Constitution \
                    modifier, 7 on the Con 16 fixture) and the flat rage constants (+4 morale \
                    Strength, +4 morale Constitution, +2 morale Will saves, -2 AC), values only. \
                    None of the grounded records are wired into the integrated \
                    base_attack_bonus/base-saves/speed/ability totals, so the integrated pilot \
                    surface still reports a blocked posture. The row remains explicitly blocked \
                    on the rage execution engine (activation/deactivation, rage-round \
                    consumption, fatigue after rage, temporary stat application). No weapon \
                    familiarity or level-2+ martial progression is claimed",
                next_required_uplift: "ground the Barbarian rage-state execution engine \
                    (activation/deactivation, rage-round consumption, post-rage fatigue, \
                    temporary application of the rage constants), and wire the grounded \
                    base-attack / base-save / fast-movement values into the integrated pilot \
                    surface, later widening into weapon familiarity and level-2+ martial \
                    progression",
            },
            SupportStateRow {
                row_id: "class.bard.progression_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:bard",
                dimension: "bounded spell-bearing class progression: the deterministic Human \
                            Bard level-1 spell baseline, with Bardic Knowledge and the flat \
                            Bardic Performance surface (rounds per day, inspire courage \
                            magnitude) grounded for real and the bardic performance-execution \
                            burden and the spontaneous known-spell / slot posture burden still \
                            blocked",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_BARD_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E4-F7 leaves direct computed evidence that the \
                    deterministic Human Bard level-1 spontaneous arcane spell-bearing identity is \
                    recognized on the compute seam, the SD13-E4 Bard decomposition slice grounds \
                    bardic knowledge (the Knowledge-check competence bonus equal to max(bard \
                    level / 2, 1), i.e. +1 at level 1, computed against the fixture with no \
                    dependency on the Bard's Intelligence modifier or skill ranks) for real, AND \
                    the SD13-E5 slice grounds the flat bardic performance surface: the bardic \
                    performance rounds per day budget (4 + CHA modifier, i.e. 6 against the \
                    fixture's Charisma 15) and the flat inspire courage level-1 magnitude (+1 \
                    competence bonus on attack and weapon damage rolls, +1 morale bonus on \
                    saves against charm and fear effects). The row is Partial, not Supported: \
                    the performance-state engine (start/maintain action economy, round \
                    tracking/consumption of the grounded budget) is not implemented, the other \
                    level-1 performances (countersong, distraction, fascinate) are not \
                    grounded, and the entire spontaneous spell burden (spontaneous spells \
                    known, spells per day, bonus spell slots from CHA, spell save DCs, school \
                    choice, prepared posture) is not computed. No performance-execution math \
                    and no spell math is fabricated and no Bard level 2+ is proven",
                next_required_uplift: "SD13-E5+ Bard performance-execution engine slice \
                    (start/maintain action economy, round tracking, countersong / distraction / \
                    fascinate grounding), then the spontaneous spell-slot burden, then level-2+ \
                    progression",
            },
            SupportStateRow {
                row_id: "class.cleric.progression_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:cleric",
                dimension: "bounded spell-bearing class progression: the deterministic Human Cleric \
                            level-1 prepared divine spell baseline, with Channel Energy, the domain \
                            choice seam, and the flat domain spell slot count grounded for real and \
                            the domain powers burden and the prepared divine spell posture burden \
                            still blocked",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_CLERIC_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E4 leaves direct computed evidence that the \
                    deterministic Human Cleric level-1 prepared divine spell-bearing identity is \
                    recognized on the compute seam, the SD13-E4 Cleric Channel Energy slice grounds \
                    Channel Energy for real (PF1 Core Rulebook: ceil(cleric level / 2) d6, minimum 1d6, \
                    computed against the fixture as 1d6 at level 1; usable 3 + Charisma modifier times \
                    per day, computed against the fixture's Charisma 14 (+2) as 5 uses per day), AND \
                    the SD13-E5 Cleric domain slice grounds the domain choice seam (the two canonical \
                    fixture selections choice:cleric_domain -> domain:good and domain:healing, \
                    surfaced as an explicit choice seam carrying no mechanical value) and the flat \
                    domain spell slot count (PF1 Core Rulebook Domains: one domain spell slot per \
                    level of cleric spells she can cast, 1st and up — exactly one 1st-level domain \
                    slot at level 1; the slot's contents are not grounded). The row is Partial, not \
                    Supported: the domain powers burden (the granted powers of the chosen domains — \
                    Good: Touch of Good; Healing: Rebuke Death, each 3 + Wisdom modifier uses per \
                    day — and the domain spell-list contents) remains named and unproven, and the \
                    prepared divine spell posture burden (spells prepared from the full Cleric list, \
                    spontaneous cure/inflict conversion, spell slots per day, bonus spells from a \
                    high Wisdom, spell save DCs) is still entirely unproven. No domain power math and \
                    no spell math is fabricated and no Cleric level 2+ is proven",
                next_required_uplift: "SD13-E5 Cleric domain powers grounding slice (Touch of Good, \
                    Rebuke Death, domain spell-list contents), then the prepared divine spell \
                    posture burden, then level-2+ progression",
            },
            SupportStateRow {
                row_id: "class.druid.progression_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:druid",
                dimension: "bounded spell-bearing class progression: the deterministic Human Druid \
                            level-1 prepared divine spell baseline, with Wild Empathy, Nature \
                            Sense, and the nature-bond choice recognition now grounded, and the \
                            animal-companion execution burden and the prepared divine spell \
                            posture burden still blocked",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_DRUID_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E4 leaves direct computed evidence that the \
                    deterministic Human Druid level-1 prepared divine spell-bearing identity is \
                    recognized on the compute seam, the SD13-E4 wild empathy grounding slice \
                    grounds wild empathy for real (PF1 Core Rulebook: 1d20 + druid level + Charisma \
                    modifier, used like a Diplomacy check to improve an animal's attitude; only the \
                    flat druid-level + Charisma-modifier bonus is computed against the deterministic \
                    fixture, no d20 roll and no Diplomacy-check execution engine), AND the SD13-E5 \
                    slice grounds nature sense for real (PF1 Core Rulebook: a flat, \
                    level-independent +2 bonus on Knowledge (nature) and Survival checks, kept as \
                    a standalone record not wired into any skill-check total) while recognizing \
                    the deterministic nature bond selection (choice:druid_nature_bond -> \
                    bond:animal_companion, a +0 recognition record with no bond execution). The \
                    row is Partial, not Supported: the animal companion execution burden (the \
                    companion's stat block, its advancement, and its link / share spells \
                    abilities) remains named and unproven, and the prepared divine spell posture \
                    burden (spells prepared from the full Druid list, spontaneous summon nature's \
                    ally conversion, spell slots per day, bonus spells from a high Wisdom, spell \
                    save DCs) is still entirely unproven. No spell math is fabricated and no Druid \
                    level 2+ is proven",
                next_required_uplift: "SD13-E5 Druid animal companion execution slice, or the \
                    prepared divine spell burden slice, then level-2+ progression",
            },
            SupportStateRow {
                row_id: "class.monk.bounded_progression",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:monk",
                dimension: "bounded Monk martial chassis progression: the deterministic Human \
                            Monk level-1 martial chassis identity, with base-attack, base-save, \
                            AC Bonus, the unarmed strike damage die, and the Flurry of Blows \
                            flat attack surface now grounded, and the level-1 bonus feat grant \
                            still unproven",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_MONK_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E3/E5 leaves direct computed evidence that the \
                    deterministic Human Monk level-1 martial chassis identity is recognized on \
                    the compute seam, and now grounds four named pillar burdens: base attack \
                    progression (3/4 BAB), base save progression (good Fortitude, Reflex, and \
                    Will), AC Bonus (Wisdom-to-AC, the flat level-1 value only), and the unarmed \
                    strike / Flurry of Blows flat surface (Medium monk 1d6 unarmed damage — die \
                    size only, no damage roll or total — and the level-1 flurry posture of two \
                    attacks at monk level - 2 = -1 each before ability modifiers). One named \
                    pillar burden remains unproven: the level-1 bonus feat grant from the \
                    restricted Monk feat list. The level-4+ unarmed damage die progression, \
                    flurry with special monk weapons, an attack-resolution engine, wiring into \
                    integrated combat totals, and Monk level 2+ all remain unproven, and no \
                    martial math beyond the grounded flat surfaces is fabricated",
                next_required_uplift: "later SD13-E5 slice grounding the one remaining named \
                    Monk martial pillar burden (the level-1 bonus feat grant from the restricted \
                    Monk feat list), then level-2+ progression",
            },
            SupportStateRow {
                row_id: "class.paladin.hybrid_chassis_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:paladin",
                dimension: "bounded hybrid class progression: the deterministic Human \
                            Paladin level-1 chassis baseline, with smite evil's uses-per-day / \
                            attack-bonus / damage-bonus formula grounded, the lay on hands / \
                            divine grace / mercy burdens grounded as correct PF1 CRB level-gate \
                            absences at level 1, the partial-caster effective-caster-level gate \
                            grounded as a correct zero absence, and the hybrid chassis pair plus \
                            the spells-known/spells-per-day/spell-DC spell burden still named and \
                            unproven",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_PALADIN_ROW_GROUNDING_REF,
                blocker_or_lossiness_note: "SD13-E3/E4/E5 leaves direct computed evidence that the \
                    deterministic Human Paladin level-1 hybrid chassis is recognized on the compute \
                    seam and that all four named non-spell class-feature burdens are grounded. The \
                    smite evil pillar is grounded for real: uses per day = 1, attack-roll bonus = Charisma \
                    modifier (if positive), damage bonus = paladin level (PF1 Core Rulebook), \
                    computed against the deterministic fixture as 1 / +2 / +1 at level 1; this \
                    grounds only that flat numeric formula, not alignment/evil-subtype target \
                    resolution or evil-outsider/dragon/undead damage doubling. Lay on hands, divine \
                    grace, and mercy are grounded as correct level gate absences: lay on hands and \
                    divine grace are 2nd-level paladin features and mercy is a 3rd-level paladin \
                    feature in the PF1 Core Rulebook, so at level 1 each emits a value-0 record \
                    naming its at-grant formula without computing it. SD13-E5 additionally grounds \
                    the partial-caster IDENTITY itself as one more flat level-gate record: \
                    effective caster level = max(paladin level - 3, 0), which correctly grounds to \
                    0 at level 1 (PF1 Core Rulebook: paladin spells begin at paladin level 4). The \
                    row is Partial, not Supported: the F6 hybrid chassis pair (class-feature and \
                    spell) stays claim-blocking as accepted hybrid truth, and the partial-caster \
                    spell burden itself (Paladin is a divine partial caster in PF1 Core Rulebook: \
                    spells begin at paladin level 4, effective caster level = paladin level - 3) \
                    remains named and unproven beyond the grounded caster-level gate arithmetic — \
                    no spell-source lineage, spells known or prepared posture, spells-per-day \
                    progression, bonus spell slots, or spell save DCs are grounded. No Paladin \
                    level 2+ is proven. The F6 hybrid baseline, the F6 hybrid blockers, and the F6 \
                    hybrid chassis recognition explanation all remain in place",
                next_required_uplift: "ground the paladin spells-known/spells-per-day/spell-DC \
                    burden content now that the effective-caster-level gate is grounded (spells \
                    begin at paladin level 4, caster level = paladin level - 3), then paladin \
                    level-2+ progression (lay on hands and divine grace at level 2, mercy at \
                    level 3)",
            },
            SupportStateRow {
                row_id: "class.ranger.hybrid_chassis_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:ranger",
                dimension: "bounded hybrid class progression: the deterministic Human \
                            Ranger level-1 chassis baseline, with Track and the favored-enemy \
                            flat surface grounded for real and the combat-style pillar burden \
                            and the later spell burden still blocked",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_RANGER_ROW_GROUNDING_REF,
                blocker_or_lossiness_note: "SD13-E3-F6 leaves direct computed evidence that the \
                    deterministic Human Ranger level-1 hybrid chassis is recognized on the compute \
                    seam, the SD13-E3 Ranger decomposition slice grounds Track (the skill/tracking \
                    pillar: a bonus on Survival checks to follow tracks equal to \
                    max(ranger level / 2, 1), i.e. +1 at level 1) for real, AND the SD13-E5 slice \
                    grounds the favored enemy flat surface for real: recognition of the chosen \
                    favored-enemy type (choice:ranger_favored_enemy), the flat +2 bonus on Bluff / \
                    Knowledge / Perception / Sense Motive / Survival checks against the favored \
                    enemy, and the flat +2 bonus on weapon attack and damage rolls against the \
                    favored enemy (PF1 includes attack rolls, unlike D&D 3.5). The row is Partial, \
                    not Supported: the favored-enemy conditional-application engine (target-type \
                    matching that would decide whether a specific check or attack is made against \
                    the favored enemy) is not implemented, the combat style pillar (the level-1 \
                    style choice and its level-2 bonus-feat grant) remains named and unproven, and \
                    the later ranger spell burden (spell slots, spell source, spells \
                    known/prepared) is still deferred to SD13-E4. No Ranger level 2+ is proven",
                next_required_uplift: "SD13-E5 ranger combat-style grounding slice and a \
                    favored-enemy conditional-application engine, then SD13-E4 ranger spell burden",
            },
            SupportStateRow {
                row_id: "class.sorcerer.progression_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:sorcerer",
                dimension: "bounded spell-bearing class progression: the deterministic Human \
                            Sorcerer level-1 spell baseline, with Eschew Materials and the \
                            canonical bloodline choice recognition grounded for real and the \
                            Arcane Bond / bloodline progression burden and the spontaneous \
                            known-spell / slot posture burden still blocked",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_SORCERER_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E4-F7 leaves direct computed evidence that the \
                    deterministic Human Sorcerer level-1 spontaneous arcane spell-bearing identity is \
                    recognized on the compute seam, the SD13-E4 Sorcerer decomposition slice \
                    grounds Eschew Materials (the universal, bloodline-independent bonus feat every \
                    1st-level Sorcerer receives: casting a spell with a material component costing \
                    1 gp or less without needing that material component) for real, AND the \
                    SD13-E5 bloodline-choice slice recognizes the canonical deterministic \
                    bloodline selection (choice:sorcerer_bloodline -> bloodline:arcane) as chosen \
                    input — recognition only, since the Arcane bloodline's level-1 power is Arcane \
                    Bond (a familiar or a bonded object), an execution engine rather than a flat \
                    number, so no power value is fabricated. The row is Partial, not Supported: \
                    the Arcane Bond / bloodline progression burden (Arcane Bond execution, the \
                    conditional bloodline arcana, the bloodline class skill grant, and the bonus \
                    spells/feats at 3rd+ level) remains named and unproven, and the spontaneous \
                    spell burden (spontaneous spells known, spell slots per day, bonus spell \
                    slots, spell save DCs) is entirely unproven. No spell math is fabricated and \
                    no Sorcerer level 2+ is proven",
                next_required_uplift: "SD13 Sorcerer Arcane Bond grounding slice (the chosen \
                    bloodline's level-1 power execution), then the spontaneous spell burden, then \
                    level-2+ progression",
            },
            SupportStateRow {
                row_id: "class.wizard.progression_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:wizard",
                // The SD13-E4-R3 merge receipt executed after the tranche 2.6
                // closeout merged to develop (2026-07-07): the row carried the
                // post-merge Blocked/Computed posture the slice's proof surface
                // pinned as its merge-receipt obligation. A further SD13-E4 Wizard
                // decomposition slice then grounds Scribe Scroll for real,
                // promoting the row from Blocked to Partial (mirroring the Ranger
                // Track promotion). The SD13-E5 Wizard specialization slice then
                // grounds the flat surface of the school specialization choice
                // (canonical Evocation specialist, Necromancy and Transmutation
                // opposed) plus the specialist bonus slot count, narrowing the
                // class-feature blocker to the school powers and the opposed-school
                // preparation cost; the prepared spell posture burden remains
                // entirely unproven.
                dimension: "bounded spell-bearing class progression: the deterministic Human \
                            Wizard level-1 prepared arcane spell baseline, with Scribe Scroll, \
                            the school specialization choice, and the specialist-bonus-slot flat \
                            count grounded for real, and the school-powers / opposed-school-cost \
                            burden and the prepared spellbook / spell-slot posture burden still \
                            blocked",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_WIZARD_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E4-R3 leaves direct computed evidence that the \
                    deterministic Human Wizard level-1 prepared arcane spell-bearing identity is \
                    recognized on the compute seam, a later SD13-E4 Wizard decomposition slice \
                    grounds Scribe Scroll (the free, specialization-independent bonus feat every \
                    1st-level Wizard is granted, letting them create scrolls of spells they know) \
                    for real, AND the SD13-E5 specialization slice grounds the school \
                    specialization choice (the canonical Evocation specialist with Necromancy and \
                    Transmutation opposed) as a recognition record plus the specialist bonus slot \
                    as a flat count only (one 1st-level Evocation-only bonus slot at level 1, no \
                    cantrip-level bonus slot, no slot contents). The row is Partial, not \
                    Supported: the school powers and opposed-school preparation-cost burden (the \
                    Evocation intense spells and force missile 3 + Int-mod/day powers, and the \
                    two-prepared-slot cost for opposed-school spells) remains named and unproven, \
                    and the prepared spell posture burden (spellbook content, spells prepared per \
                    day, spell slots per day, bonus slots from high Intelligence, spell save DCs) \
                    is still entirely unproven. No spell math is fabricated and no Wizard level \
                    2+ is proven",
                next_required_uplift: "SD13-E5 Wizard school-powers and opposed-school \
                    preparation-cost grounding slice, then the prepared spellbook / spell-slot \
                    posture slice, then level-2+ progression",
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
