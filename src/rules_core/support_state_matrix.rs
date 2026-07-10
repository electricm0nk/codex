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
//!   blocked; further SD13-E5 slices widen the level-range gate to level 2
//!   (grounding Evasion as a flat identity record) and to level 3 (grounding
//!   Trap Sense as a flat-magnitude record); the live GE-06 negative control
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
//!   compute seam (mirroring the Barbarian pattern), and the SD13-E5 slices ground
//!   five named pillar burdens (base attack, base save, AC Bonus, unarmed strike /
//!   Flurry of Blows, and the level-1 bonus feat choice-slot selection); the
//!   recognized bonus feat's own mechanics (an execution engine, not a flat
//!   number) remain unproven,
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

/// SD13-E3/E5 dedicated proof surface for the bounded Human Rogue
/// level-1/level-2/level-3/level-4 chassis baseline (mirroring the
/// Barbarian/Monk pattern): direct computed chassis-recognition evidence with
/// all four named pillar burdens (base attack, base save, sneak attack die
/// count, trapfinding) grounded as standalone records at level 1, widened to
/// level 2 by a later SD13-E5 slice (the level-range gate plus the Evasion
/// identity/recognition record), widened to level 3 by a further SD13-E5
/// slice (the level-range gate plus the Trap Sense flat-magnitude record),
/// widened to level 4 by a further SD13-E5 slice (the level-range gate plus
/// the Uncanny Dodge identity/recognition record), widened through level 8 by
/// further SD13-E5 slices, and widened again to level 9 by a still further
/// SD13-E5 slice (sneak attack genuinely rises to 5d6 and Trap Sense to +3,
/// both via the pre-existing formulas; poor Fortitude/Will both genuinely
/// rise to +3 while base attack and good Reflex stay put, integer-division
/// coincidences; level 9 is not a rogue-talent level, so no new pillar is
/// added), citing all nine proof
/// files as one combined literal.
const SD13_ROGUE_LEVEL1_TEST: &str = "tests/sd13_rogue_level1_chassis_baseline.rs + \
    tests/sd13_rogue_level2_progression.rs + \
    tests/sd13_rogue_level3_progression.rs + \
    tests/sd13_rogue_level4_progression.rs + \
    tests/sd13_rogue_level5_progression.rs + \
    tests/sd13_rogue_level6_progression.rs + \
    tests/sd13_rogue_level7_progression.rs + \
    tests/sd13_rogue_level8_progression.rs + \
    tests/sd13_rogue_level9_progression.rs";

/// SD13-E5 dedicated proof surface for the bounded Fighter level-9/level-10
/// milestones (Weapon Training 2 attack-roll seam, second weapon-training group
/// seam, and level-10 bonus-feat seam). This is the most specific/current proof
/// for the levels-2-10 row's grounding_ref. (The Fighter Bravery flat-magnitude
/// milestone — tests/sd13_fighter_bravery.rs — layers on top of this same
/// levels-1-10 fixture set without moving the row's grounding_ref, since the
/// already-landed sd13_fighter_level9_level10_progression.rs test asserts this
/// exact grounding_ref string.)
const SD13_FIGHTER_LEVEL9_LEVEL10_TEST: &str = "tests/sd13_fighter_level9_level10_progression.rs";

/// The combined grounding reference for the Fighter level-1 pilot row, citing
/// the SD13-E3-F5 mandatory-milestone classification proof (which level-1
/// mandatory milestones are proven versus unproven for the level-10 progression
/// matrix), the SD13-E5 level-1 hit-point baseline proof (level-1 hit points
/// = maximized d10 hit die 10 + Constitution modifier as a standalone grounded
/// explanation record), and the SD13-E5 favored-class bonus choice recognition
/// proof (which of the PF1 Core Rulebook Favored Class rule's two options, +1
/// hp or +1 skill rank, was selected, as a standalone flat-magnitude record)
/// as one literal (paladin-row idiom). Each `.contains()` consumer reads its
/// respective substring from this combined grounding reference.
const SD13_FIGHTER_LEVEL1_ROW_GROUNDING_REF: &str =
    "tests/sd13_fighter_level1_mandatory_milestone_classification.rs + \
     tests/sd13_fighter_level1_hit_point_baseline.rs + \
     tests/sd13_fighter_favored_class_bonus_choice.rs";

/// The combined grounding reference for the Paladin hybrid baseline row, citing
/// F6 (chassis identity), the per-burden decomposition test, the SD13-E5
/// effective-caster-level gate test, and the SD13-E5 level-2 lay on hands /
/// divine grace grounding test as one literal. Each `.contains()` consumer
/// reads its respective substring from this combined grounding reference.
const SD13_PALADIN_ROW_GROUNDING_REF: &str = "tests/sd13_hybrid_level1_chassis_baseline.rs +      tests/sd13_paladin_level1_chassis_and_spell_burden_separation.rs + \
    tests/sd13_paladin_partial_caster_effective_caster_level.rs + \
    tests/sd13_paladin_level2_lay_on_hands_divine_grace.rs + \
    tests/sd13_paladin_base_attack_and_saves.rs + \
    tests/sd13_paladin_level3_mercy.rs + \
    tests/sd13_paladin_level4_progression.rs + \
    tests/sd13_paladin_level5_progression.rs + \
    tests/sd13_paladin_level6_progression.rs + \
    tests/sd13_paladin_level7_progression.rs + \
    tests/sd13_paladin_level8_progression.rs";

/// The combined grounding reference for the Ranger hybrid baseline row, citing
/// F6 (chassis identity), the Ranger-only per-pillar decomposition + Track /
/// Favored-Enemy-flat-surface grounding test, and the SD13-E5 base-attack/
/// base-save progression test as one literal, mirroring
/// [`SD13_PALADIN_ROW_GROUNDING_REF`]. Each `.contains()` consumer reads its
/// respective substring from this combined grounding reference.
const SD13_RANGER_ROW_GROUNDING_REF: &str = "tests/sd13_hybrid_level1_chassis_baseline.rs + \
    tests/sd13_ranger_level1_chassis_and_class_feature_separation.rs + \
    tests/sd13_ranger_base_attack_and_saves.rs + \
    tests/sd13_ranger_level2_progression.rs + \
    tests/sd13_ranger_level3_progression.rs + \
    tests/sd13_ranger_favored_terrain_choice.rs + \
    tests/sd13_ranger_level4_progression.rs + \
    tests/sd13_ranger_level5_progression.rs + \
    tests/sd13_ranger_level6_progression.rs + \
    tests/sd13_ranger_level7_progression.rs + \
    tests/sd13_ranger_level8_progression.rs + \
    tests/sd13_ranger_level9_progression.rs";

/// SD13-E4-F7 / SD13-E4 / SD13-E5 dedicated proof surface for the bounded Human
/// Sorcerer level-1/level-2/level-3 spell baseline: direct computed recognition of the
/// spontaneous arcane spell-bearing identity, plus the SD13-E4 decomposition slice's
/// grounded Eschew Materials bonus-feat grant, the SD13-E5 bloodline-choice and
/// bloodline-class-skill-choice recognition slices, and the base-attack/base-save
/// progression pillar, widened to level 2 by a later SD13-E5 slice (the level-range
/// gate plus every named pillar formula extended to level 2 via the same formula,
/// with no new class feature gained at 2nd level per the PF1 Core Rulebook Sorcerer
/// class table's blank level-2 "Special" column), and widened again to level 3 by a
/// further SD13-E5 slice (every named pillar formula extended to level 3 via the same
/// formula; the level-3 "Special" column reads "Bloodline power, bloodline spell", but
/// both entries are bloodline-specific and not flat/identity-shaped, so no new pillar
/// is added), widened again through levels 4-7 by further SD13-E5 slices, and widened
/// again to level 8 by a still further SD13-E5 slice (base attack genuinely rises to
/// +4 while good Will genuinely rises to +6; poor Fortitude/Reflex both stay +2,
/// integer-division coincidences; the level-8 "Special" column is blank per both
/// primary sources, so no new pillar is added — the first 4th-level spell slots arrive
/// at 8th but belong to the still-unproven spontaneous spell burden), while the Arcane
/// Bond / bloodline progression burden and the spontaneous known-spell / slot posture
/// burden stay explicitly blocked, widened again to level 9 by a still further
/// SD13-E5 slice (poor Fortitude/Reflex both genuinely rise to +3 while base attack
/// and good Will stay put, integer-division coincidences; the level-9 "Special"
/// column's "Bloodline power, bloodline spell" entries were checked against both
/// primary sources and confirmed bloodline-specific, not flat, so both stay
/// named-but-unproven per the level-3/5/7 precedent), citing the proof files as one
/// combined literal, mirroring [`SD13_CLERIC_LEVEL1_TEST`] / [`SD13_DRUID_LEVEL1_TEST`].
const SD13_SORCERER_LEVEL1_TEST: &str = "tests/sd13_sorcerer_level1_spell_baseline.rs + \
    tests/sd13_sorcerer_bloodline_class_skill_choice.rs + \
    tests/sd13_sorcerer_level2_progression.rs + \
    tests/sd13_sorcerer_level3_progression.rs + \
    tests/sd13_sorcerer_level4_progression.rs + \
    tests/sd13_sorcerer_level5_progression.rs + \
    tests/sd13_sorcerer_level6_progression.rs + \
    tests/sd13_sorcerer_level7_progression.rs + \
    tests/sd13_sorcerer_level8_progression.rs + \
    tests/sd13_sorcerer_level9_progression.rs";

/// SD13-E3/E5 dedicated proof surface for the bounded Human Barbarian level-1/
/// level-2/level-3/level-4 martial chassis baseline: direct computed
/// chassis-recognition evidence, plus grounded base-attack, base-save,
/// fast-movement, and flat Rage pillar values (rage rounds per day and the rage
/// constants, values only) and the vacuous illiteracy-burden rules correction at
/// level 1, widened to level 2 by a later SD13-E5 slice (the level-range gate plus
/// the extended formulas, plus the Uncanny Dodge identity/recognition record),
/// widened to level 3 by a further SD13-E5 slice (the level-range gate plus the
/// extended formulas, plus the Trap Sense flat-magnitude record), widened to level 4
/// by a still further SD13-E5 slice (the level-range gate plus the extended
/// formulas; Uncanny Dodge and Trap Sense both stay granted, not re-derived; the
/// level-4 "Special" entry is confirmed to be another Rage Power grant, not a new
/// class feature), that stays explicitly blocked only on the remaining named
/// rage-state execution burden, the Rage Power choice-list feature, and weapon
/// familiarity.
const SD13_BARBARIAN_LEVEL1_TEST: &str = "tests/sd13_barbarian_level1_chassis_baseline.rs + \
    tests/sd13_barbarian_level2_progression.rs + \
    tests/sd13_barbarian_level3_progression.rs + \
    tests/sd13_barbarian_level4_progression.rs + \
    tests/sd13_barbarian_level5_progression.rs + \
    tests/sd13_barbarian_level6_progression.rs + \
    tests/sd13_barbarian_level7_progression.rs + \
    tests/sd13_barbarian_level8_progression.rs + \
    tests/sd13_barbarian_level9_progression.rs";

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

/// SD13-E4-F7/SD13-E4/SD13-E5 dedicated proof surface for the bounded Human Bard
/// level-1/level-2/level-3/level-4 spell baseline: direct computed recognition of
/// the spontaneous arcane spell-bearing identity, the grounded Bardic Knowledge and
/// flat Bardic Performance pillars (rounds per day, inspire courage magnitude), the
/// flat Fascinate DC / affected-creature-count formulas, (at level 2) the flat
/// Well-Versed magnitude, and (at level 3) the flat Inspire Competence magnitude,
/// with the bardic performance-execution burden (including Countersong,
/// Distraction, and Versatile Performance) and the spontaneous known-spell / slot
/// posture burden still blocked, widened to level 2 by a later SD13-E5 slice (the
/// level-range gate plus every named pillar formula extended to level 2 via the
/// same formula), widened again to level 3 by a further SD13-E5 slice (the
/// level-range gate plus every named pillar formula extended to level 3 via the
/// same formula, with Inspire Competence newly grounded as a flat +2 magnitude;
/// unlike Wizard/Cleric's level-3 spell-slot-count doubling, Bard has no grounded
/// spell-slot-count pillar for the doubling precedent to apply to), widened again
/// to level 4 by a further SD13-E5 slice (the level-range gate plus every named
/// pillar formula extended to level 4 via the same formula; verified against both
/// primary sources that the PF1 Core Rulebook Bard class table's level-4 Special
/// column is blank, so no new pillar is introduced), widened again to level 5 by
/// a further SD13-E5 slice (the level-range gate plus every named pillar formula
/// extended to level 5 via the same formula, with the Inspire Courage flat
/// magnitude genuinely increasing to +2 and Lore Master newly grounded as a
/// bounded grant-only flat 1/day take-20 usage-count record, verified
/// independently against both primary sources rather than trusted from an
/// earlier cycle's phrasing at face value), widened again to level 6 by a
/// still further SD13-E5 slice (the level-range gate plus every named pillar
/// formula extended to level 6 via the same formula, with Bardic Knowledge
/// genuinely rising to 3 and the Fascinate DC genuinely rising to 15; Inspire
/// Courage and the Fascinate affected-creature count both stay numerically
/// unchanged from level 5; the level-6 "Special" column's two entries,
/// Suggestion and the additional Versatile Performance substitution, were
/// both checked against a primary source and confirmed NOT flat, so neither
/// is grounded), widened to level 7 by a still further SD13-E5 slice (base
/// attack genuinely rises to 5, base saves stay numerically unchanged,
/// Bardic Knowledge stays 3, the Fascinate DC stays 15, the Fascinate
/// affected-creature count genuinely rises to 3, and Inspire Competence
/// genuinely rises from +2 to +3 per the PF1 Core Rulebook's own "+1 for
/// every four levels beyond 3rd" rule text), widened again to level 8 by a
/// still further SD13-E5 slice (base attack genuinely rises to 6, base
/// Fortitude stays numerically unchanged while Reflex/Will both genuinely
/// rise to 6, Bardic Knowledge genuinely rises to 4, the Fascinate DC
/// genuinely rises to 16, the Fascinate affected-creature count stays
/// numerically unchanged at 3, Inspire Courage and Inspire Competence both
/// stay at their existing tiers, and the level-8 "Special" column's Dirge
/// of Doom entry was checked against two primary sources and confirmed NOT
/// flat, so it is deliberately left named-but-unproven), widened again to
/// level 9 by a still further SD13-E5 slice (the performance rounds pool
/// genuinely rises to 22 and poor Fortitude genuinely rises to +3 while every
/// other pillar stays put, integer-division coincidences and pre-10th/11th
/// tier gates each checked rather than assumed; the level-9 "Special"
/// column's Inspire Greatness entry was checked against both primary sources
/// and confirmed NOT flat — bonus Hit Dice, temporary hit points, and
/// competence bonuses behind the ungrounded performance-state engine — so it
/// is deliberately left named-but-unproven), citing all nine
/// proof files as one combined literal, mirroring [`SD13_CLERIC_LEVEL1_TEST`].
const SD13_BARD_LEVEL1_TEST: &str = "tests/sd13_bard_level1_spell_baseline.rs + \
    tests/sd13_bard_level2_progression.rs + tests/sd13_bard_level3_progression.rs + \
    tests/sd13_bard_level4_progression.rs + tests/sd13_bard_level5_progression.rs + \
    tests/sd13_bard_level6_progression.rs + tests/sd13_bard_level7_progression.rs + \
    tests/sd13_bard_level8_progression.rs + tests/sd13_bard_level9_progression.rs";

/// SD13-E4-R3 dedicated proof surface for the bounded Human Wizard level-1/level-3
/// prepared arcane spell baseline: direct computed recognition of the prepared
/// arcane spell-bearing identity, plus a later SD13-E4 decomposition slice grounding
/// the Scribe Scroll bonus feat grant and the SD13-E5 slices grounding the school
/// specialization choice, flat specialist-bonus-slot count, and base attack/save
/// progression for real, widened to level 2 by a further SD13-E5 slice (the
/// level-range gate plus every named pillar formula extended to level 2 via the same
/// formula, with no new class feature gained at 2nd level per the PF1 Core Rulebook
/// Wizard class table's blank level-2 "Special" column), then widened again to level
/// 3 by a further SD13-E5 slice (the level-range gate extended to 1..=3; the
/// specialist bonus slot flat count changes for real at level 3, from 1 to 2, since
/// a level-3 wizard casts 2nd-level spells for the first time; the level-3 "Special"
/// column is also blank, so no new pillar is added), then widened again to level 4
/// by a further SD13-E5 slice (the level-range gate extended to 1..=4; the
/// specialist bonus slot flat count is checked rather than assumed to double again
/// and correctly stays at 2, since 3rd-level wizard spells do not become available
/// until wizard level 5; Intense Spells' bonus-damage magnitude changes for real at
/// level 4, from 1 to 2, via the pre-existing half-wizard-level-minimum-1 formula;
/// the level-4 "Special" column is also blank, so no new pillar is added), then
/// widened again to level 5 by a further SD13-E5 slice (the level-range gate
/// extended to 1..=5; the specialist bonus slot flat count changes for real at level
/// 5, from 2 to 3, since a level-5 wizard casts 3rd-level spells for the first time;
/// Intense Spells' bonus-damage magnitude stays 2, an integer-division coincidence;
/// the level-5 "Special" column reads "Bonus feat", a genuinely new class feature
/// checked and confirmed not flat — a choice among an open-ended metamagic/item
/// creation feat set or Spell Mastery — so it is deliberately left
/// named-but-unproven, mirroring the Monk High Jump precedent), widened again to
/// level 6 by a still further SD13-E5 slice (base attack bonus and all three base
/// saves genuinely rise; the specialist bonus slot flat count is checked rather
/// than assumed to rise again and correctly stays at 3, since 4th-level wizard
/// spells do not become available until level 7; Intense Spells' bonus-damage
/// magnitude genuinely rises to 3; the level-6 "Special" column is blank, so no
/// new pillar is added), widened again to level 7 by a still further SD13-E5
/// slice (base attack bonus and all three base saves stay numerically unchanged,
/// an integer-division coincidence; the specialist bonus slot flat count
/// genuinely rises to 4, since a level-7 wizard casts 4th-level spells for the
/// first time; Intense Spells' bonus-damage magnitude stays at 3, another
/// integer-division coincidence; the level-7 "Special" column is blank, so no
/// new pillar is added), widened again to level 8 by a still further SD13-E5
/// slice (base attack genuinely rises to +4 and good Will genuinely rises to
/// +6 while poor Fortitude/Reflex stay +2, integer-division coincidences; the
/// specialist bonus slot flat count is checked rather than assumed to rise
/// again and correctly stays at 4, since 5th-level wizard spells do not become
/// available until level 9; Intense Spells' bonus-damage magnitude genuinely
/// rises to 4; the level-8 "Special" column is blank — the Wizard's bonus
/// feats land at levels 5/10/15/20 — so no new pillar is added), that stays
/// explicitly blocked on the school-powers / opposed-school-cost burden and
/// the prepared spellbook / spell-slot posture burden, citing all eight proof
/// files as one combined literal, mirroring [`SD13_SORCERER_LEVEL1_TEST`].
const SD13_WIZARD_LEVEL1_TEST: &str = "tests/sd13_wizard_level1_prepared_spell_baseline.rs + \
    tests/sd13_wizard_level2_progression.rs + tests/sd13_wizard_level3_progression.rs + \
    tests/sd13_wizard_level4_progression.rs + tests/sd13_wizard_level5_progression.rs + \
    tests/sd13_wizard_level6_progression.rs + tests/sd13_wizard_level7_progression.rs + \
    tests/sd13_wizard_level8_progression.rs";

/// SD13-E4/E5 dedicated proof surface for the bounded Human Cleric level-1/level-2/
/// level-3 prepared divine spell baseline: direct computed recognition of the
/// prepared divine spell-bearing identity, with Channel Energy, the domain choice
/// seam, and the flat domain spell slot count grounded for real, that stays
/// explicitly blocked on the domain powers burden and the prepared divine spell
/// posture burden, widened to level 2 by a later SD13-E5 slice (the level-range gate
/// plus every named pillar formula extended to level 2 via the same formula), widened
/// again to level 3 by a further SD13-E5 slice (Channel Energy's die count and the
/// domain spell slot count both change for real at level 3), citing all three proof
/// files as one combined literal, mirroring [`SD13_MONK_LEVEL1_TEST`] /
/// [`SD13_ROGUE_LEVEL1_TEST`] / [`SD13_WIZARD_LEVEL1_TEST`].
const SD13_CLERIC_LEVEL1_TEST: &str = "tests/sd13_cleric_level1_spell_baseline.rs + \
    tests/sd13_cleric_level2_progression.rs + tests/sd13_cleric_level3_progression.rs + \
    tests/sd13_cleric_level4_progression.rs + tests/sd13_cleric_level5_progression.rs + \
    tests/sd13_cleric_level6_progression.rs + tests/sd13_cleric_level7_progression.rs + \
    tests/sd13_cleric_level8_progression.rs + tests/sd13_cleric_level9_progression.rs";

/// SD13-E4/E5 dedicated proof surface for the bounded Human Druid level-1/level-2/
/// level-3 prepared divine spell baseline: direct computed recognition of the
/// prepared divine spell-bearing identity, with base attack bonus, base save
/// progression, Wild Empathy, Nature Sense, the nature-bond choice recognition, (at
/// level 2) Woodland Stride, and (at level 3) Trackless Step grounded for real, that
/// stays explicitly blocked on the animal companion execution burden and the
/// prepared divine spell posture burden, widened to level 2 by a later SD13-E5 slice
/// (the level-range gate plus every named pillar formula extended to level 2 via the
/// same formula), to level 3 by a further SD13-E5 slice (the level-range gate plus
/// every named pillar formula extended to level 3 via the same formula, plus the
/// Trackless Step identity/recognition record), to level 4 by a still further
/// SD13-E5 slice (the level-range gate plus every named pillar formula extended to
/// level 4 via the same formula, plus the Resist Nature's Lure flat-magnitude
/// identity record), to level 5 by a still further SD13-E5 slice (the level-range
/// gate plus every named pillar formula extended to level 5 via the same formula;
/// the class table's level-5 "Special" column is genuinely blank, so no new pillar
/// is added), and to level 6 by a still further SD13-E5 slice (the level-range gate
/// plus every named pillar formula extended to level 6 via the same formula; the
/// class table's level-6 "Wild shape (2/day)" Special-column entry was checked and
/// confirmed not genuinely separable from Wild Shape's own non-flat form-list
/// expansion, so no new pillar is added either), widened again to level 8 by a
/// still further SD13-E5 slice (base attack genuinely rises to +6 and both good
/// saves genuinely rise to +6 while poor Reflex stays +2, an integer-division
/// coincidence; Wild Empathy genuinely rises to 9; the level-8 "Special" column's
/// "Wild shape (3/day)" entry was checked against both primary sources and
/// confirmed to be the same non-separable frequency-plus-form-expansion bundle as
/// at level 6, so it stays entirely named-but-unproven and no new pillar is
/// added), citing all eight proof files as one
/// combined literal, mirroring [`SD13_CLERIC_LEVEL1_TEST`] / [`SD13_BARD_LEVEL1_TEST`].
const SD13_DRUID_LEVEL1_TEST: &str = "tests/sd13_druid_level1_spell_baseline.rs + \
    tests/sd13_druid_level2_progression.rs + tests/sd13_druid_level3_progression.rs + \
    tests/sd13_druid_level4_progression.rs + tests/sd13_druid_level5_progression.rs + \
    tests/sd13_druid_level6_progression.rs + tests/sd13_druid_level7_progression.rs + \
    tests/sd13_druid_level8_progression.rs";

/// The combined grounding reference for the Monk martial chassis row, citing the
/// SD13-E3/E5 chassis-baseline test (chassis identity, base attack/save, AC Bonus,
/// unarmed strike die, Flurry of Blows) at level 1, the SD13-E5 bonus-feat
/// choice-recognition test, the SD13-E5 level-2 progression test (level-range
/// gate widening plus Evasion), and the SD13-E5 level-3 progression test
/// (level-range gate widening plus Still Mind) as one literal, mirroring
/// [`SD13_PALADIN_ROW_GROUNDING_REF`] / [`SD13_RANGER_ROW_GROUNDING_REF`]. Both
/// `.contains()` consumers read their respective substring from this combined
/// grounding reference.
const SD13_MONK_LEVEL1_TEST: &str = "tests/sd13_monk_level1_chassis_baseline.rs + \
    tests/sd13_monk_bonus_feat_choice.rs + tests/sd13_monk_level2_progression.rs + \
    tests/sd13_monk_level3_progression.rs + tests/sd13_monk_level4_progression.rs + \
    tests/sd13_monk_level5_progression.rs + tests/sd13_monk_level6_progression.rs + \
    tests/sd13_monk_level7_progression.rs + tests/sd13_monk_level8_progression.rs + \
    tests/sd13_monk_level9_progression.rs";

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
                            half still unproven",
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
                    Shirt), the level-9 Weapon Training 2 attack-roll half (rank = 1 + \
                    (level - 5) / 4: the first-group Heavy Blades bonus rises to +2, folded \
                    into the baseline melee attack bonus, and the canonical second group, Bows, \
                    is surfaced at +1 as an explanation-only seam covering no equipped weapon) \
                    over the deterministic Human loadout, and Bravery (+1 Will save vs fear at \
                    level 2, +2 at level 6, +3 at level 10, rank = 1 + (level - 2) / 4) as a \
                    flat, non-fabricated bonus magnitude record. The Weapon Training \
                    damage-roll half stays unproven — no damage total is computed anywhere in \
                    this codebase for any Fighter level, so this is not a new gap. Bravery's \
                    magnitude is grounded, but the Will-vs-fear total itself stays unproven: no \
                    fear-condition or save-resolution engine exists on this compute surface, so \
                    the Bravery bonus is never folded into the unconditional Will save total. \
                    The generic PF1 ability-score-increase milestones need no separate seam: \
                    the chosen ability score is trusted at face value. Any general \
                    feat-effect/prerequisite engine also remains out of proof",
                next_required_uplift: "later SD13 slice grounding the remaining named Fighter \
                    class-feature burden inside levels 2-10: the Weapon Training damage-roll \
                    half (which first needs any damage total to exist on the compute surface), \
                    or a fear-condition/save-resolution engine to apply the Bravery magnitude \
                    to an actual Will save (a tranche-level subsystem decision, not a slice \
                    decision)",
            },
            SupportStateRow {
                row_id: "class.rogue.bounded_progression",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:rogue",
                dimension: "bounded Rogue chassis progression: the deterministic Human Rogue \
                            level-1/level-2/level-3/level-4/level-5/level-6/level-7/level-8/\
                            level-9 \
                            chassis identity, with all four named pillars grounded across all \
                            nine levels (base-attack, base-save, sneak-attack die count, and \
                            trapfinding), Evasion grounded as a level-2 identity/recognition \
                            record, Trap Sense grounded as a level-3 flat-magnitude record \
                            (genuinely rising to +2 at level 6, unchanged at levels 7-8, and \
                            genuinely rising to +3 at level 9), Uncanny \
                            Dodge grounded as a level-4 identity/recognition record, Improved \
                            Uncanny Dodge grounded as a level-8 identity/recognition record, the \
                            sneak-attack die count genuinely rising to 3d6 at level 5, staying \
                            there at level 6, rising to 4d6 at level 7, staying there at \
                            level 8, and rising to 5d6 at level 9, Trapfinding genuinely rising \
                            to +4 at level 8 (unchanged at level 9), and the \
                            check-execution / rogue-talent / integration remainder still \
                            unproven",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_ROGUE_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E3 leaves direct computed evidence that the \
                    deterministic Human Rogue level-1 chassis identity is recognized on the \
                    compute seam, and all four named pillar burdens are now grounded: base \
                    attack progression (3/4 BAB, level * 3 / 4), base save progression (good \
                    Reflex, poor Fortitude, poor Will), sneak attack (die count only, +1d6 at \
                    levels 1-2 and +2d6 at levels 3-4 — damage-roll execution and the flanking / \
                    Dexterity-denial trigger-condition engine remain unproven), and, per the \
                    SD13-E5 slice, trapfinding (the flat max(rogue level / 2, 1) bonus on \
                    Perception checks to locate traps and on Disable Device checks, +1 at levels \
                    1-3 and +2 at level 4, plus the magic-trap-disarm statement — a \
                    check-execution engine, trap DC resolution, and a magic-trap disarm engine \
                    remain unproven). A later SD13-E5 slice widens the level-1-only gate to \
                    level 2 (mirroring the Fighter/Paladin level-range gate idiom) and grounds \
                    Evasion as a bounded identity/recognition record only (value 0, \
                    non-fabricated): no damage on a successful Reflex save against an effect \
                    that normally allows half damage on a successful save, no benefit on a \
                    failed save — naming the rule text with no saving-throw-resolution or \
                    damage-resolution engine behind it. A further SD13-E5 slice widens the gate \
                    to level 3 (verified independently against d20pfsrd and legacy.aonprd.com) \
                    and grounds Trap Sense as a bounded flat-magnitude record only (rogue level \
                    / 3, floor; +1 at levels 3-4): a bonus on Reflex saves made to avoid traps \
                    and an equal dodge bonus to AC against attacks made by traps, never applied \
                    to any actual Reflex-save total or AC total, since no \
                    saving-throw-resolution or armor-class-resolution engine exists in this \
                    codebase, and no trap-detection or trap-triggering engine exists to decide \
                    when it would apply. A further SD13-E5 slice widens the gate to level 4 \
                    (verified independently against d20pfsrd and legacy.aonprd.com — the Rogue \
                    class table's level-4 \"Special\" column reads \"Rogue talent, uncanny \
                    dodge,\" NOT the same level as Barbarian's own 2nd-level Uncanny Dodge) and \
                    grounds Uncanny Dodge as a bounded identity/recognition record only (value \
                    0, non-fabricated): cannot be caught flat-footed, retains Dexterity bonus to \
                    AC even against an invisible attacker, still loses it if immobilized — \
                    naming the rule text with no flat-footed-state tracking, Armor Class \
                    computation, or invisibility-detection engine behind it. A further SD13-E5 \
                    slice widens the gate to level 5 (verified independently against d20pfsrd \
                    and legacy.aonprd.com: the Rogue class table's level-5 \"Special\" column \
                    reads only \"Sneak attack +3d6,\" no other new class feature) and the \
                    pre-existing sneak-attack die-count formula ((level + 1) / 2) genuinely \
                    produces 3d6 at level 5, up from 2d6 at levels 3-4, via the same formula, \
                    not a new record; Evasion, Trap Sense, and Uncanny Dodge all stay granted at \
                    level 5, not re-derived. A further SD13-E5 slice widens the gate to level 6 \
                    (verified independently against d20pfsrd and legacy.aonprd.com: the Rogue \
                    class table's level-6 \"Special\" column reads \"Rogue talent, trap sense \
                    +2\") and the pre-existing Trap Sense flat-magnitude formula (rogue level / \
                    3, floor) genuinely rises to +2 at level 6, up from +1 at levels 3-5, via \
                    the same formula, not a new record; the pre-existing sneak-attack die-count \
                    formula stays at 3d6, unchanged from level 5; Trapfinding genuinely rises to \
                    +3 (max(rogue level / 2, 1)); Evasion and Uncanny Dodge both stay granted, \
                    not re-derived. The level-6 row's other named entry, a second Rogue Talent \
                    slot, is deliberately left named-but-unproven, mirroring the level-2/level-4 \
                    rogue-talent precedent. A further SD13-E5 slice widens the gate to level 7 \
                    (verified independently against d20pfsrd and legacy.aonprd.com: the Rogue \
                    class table's level-7 \"Special\" column reads only \"Sneak attack +4d6,\" \
                    no other new feature) and the pre-existing sneak-attack die-count formula \
                    ((level + 1) / 2) genuinely rises to 4d6 at level 7, up from 3d6 at level 6, \
                    via the same formula, not a new record; the pre-existing Trap Sense \
                    flat-magnitude formula stays at +2 (unchanged from level 6, the next rise is \
                    at 9th level); Trapfinding stays at +3 (max(rogue level / 2, 1), an \
                    integer-division coincidence with level 6); Evasion and Uncanny Dodge both \
                    stay granted, not re-derived. A further SD13-E5 slice widens the gate to \
                    level 8 (verified independently against d20pfsrd and legacy.aonprd.com: the \
                    Rogue class table's level-8 \"Special\" column reads \"Improved uncanny \
                    dodge, rogue talent\") and grounds Improved Uncanny Dodge as a bounded \
                    identity/recognition record only (value 0, non-fabricated), mirroring \
                    exactly how Barbarian's own Improved Uncanny Dodge was grounded at \
                    barbarian level 5: a rogue of 8th level or higher can no longer be flanked, \
                    denying another rogue the ability to sneak attack her by flanking unless \
                    the attacker has at least four more rogue levels — never applied to any \
                    actual flanking-resolution or attacker-level-comparison engine, neither of \
                    which exists in this codebase; the pre-existing sneak-attack die-count \
                    formula ((level + 1) / 2) stays at 4d6 (unchanged from level 7, since the \
                    die count only rises at odd rogue levels); the pre-existing Trap Sense \
                    flat-magnitude formula stays at +2 (unchanged from level 7, the next rise is \
                    at 9th level); Trapfinding genuinely rises to +4 (max(rogue level / 2, 1), \
                    up from +3 at level 7, via the same formula); Evasion and Uncanny Dodge both \
                    stay granted, not re-derived. The level-8 row's other named entry, a third \
                    Rogue Talent slot, is deliberately left named-but-unproven, mirroring the \
                    level-2/level-4/level-6 rogue-talent precedent — AND a further SD13-E5 \
                    slice widens the level-range gate again (supported_rogue_level, 1..=9), the \
                    first level-9 slice in the tranche, and extends every one of the formulas \
                    above to level 9 via the same formula, without re-derivation, verified \
                    independently against the PF1 Core Rulebook Rogue class table (d20pfsrd and \
                    legacy.aonprd.com): level 9 base attack bonus stays +6 (9 * 3 / 4, an \
                    integer-division coincidence) while poor Fortitude/Will both genuinely rise \
                    to +3 (9 / 3) and good Reflex stays +6 (9 / 2 + 2, another coincidence); \
                    the level-9 \"Special\" column reads \"Sneak attack +5d6, trap sense \
                    +3\" (verified independently against both primary sources, checked rather \
                    than assumed away) — BOTH entries are tier-rises on already-grounded \
                    formula pillars, not new class features: the sneak attack die count \
                    genuinely rises to 5 ((9 + 1) / 2, matching the odd-level rise cadence) and \
                    Trap Sense genuinely rises to +3 (9 / 3); Trapfinding stays +4 \
                    (max(9/2, 1), a coincidence); Evasion, Uncanny Dodge, and Improved Uncanny \
                    Dodge all stay granted, not re-derived; level 9 is NOT a rogue-talent level \
                    (talents land at 2/4/6/8/10...), so no new pillar is grounded and nothing \
                    new is left unproven for the talent tree either. The row is Partial, not \
                    Supported: no rogue talent (a level-2+/4+/6+/8+ choice-list feature, and a \
                    genuinely open-ended talent tree left named but unproven) is proven, no \
                    Rogue level 10+ is proven, and no mechanical math is fabricated beyond these \
                    grounded pillars. tests/ge06_pilot_total_saves.rs \
                    (unsupported_chassis_blocks_total_saves) still claim-blocks class:rogue:1 \
                    unmodified: the class_chassis.rogue.base_attack_bonus / base_save.* / \
                    sneak_attack / trapfinding / class_feature.rogue.evasion / \
                    class_feature.rogue.trap_sense / class_feature.rogue.uncanny_dodge / \
                    class_feature.rogue.improved_uncanny_dodge explanations are standalone \
                    records, not wired into compute_fighter_chassis, compute_total_saves, or \
                    compute_combat_baseline, so defense.total_save.* is still never computed \
                    for Rogue.",
                next_required_uplift: "later SD13 slice wiring the grounded Rogue pillar \
                    records into the integrated pilot surface (the generic chassis diagnostics \
                    still claim-block), then rogue talents and level-10+ progression",
            },
            SupportStateRow {
                row_id: "class.barbarian.bounded_progression",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:barbarian",
                dimension: "bounded Barbarian chassis progression: the deterministic Human \
                    Barbarian level-1/level-2/level-3/level-4/level-5/level-6/level-7/level-8/\
                    level-9 \
                    martial chassis identity, with base-attack, base-save, fast-movement, and \
                    flat Rage pillar values grounded across all nine levels, Uncanny Dodge \
                    grounded as a level-2 identity/recognition record, Trap Sense grounded as a \
                    level-3 flat-magnitude record (unchanged at levels 4-5, rising to +2 at \
                    level 6, unchanged at levels 7-8, and rising to +3 at level 9), Improved \
                    Uncanny Dodge grounded as a \
                    level-5 identity/recognition record, Damage Reduction grounded as a level-7 \
                    flat-magnitude record (unchanged at levels 8-9), and the rage-state execution / \
                    Rage Power choice-list / weapon-familiarity / flanking-resolution / \
                    damage-reduction-application / level-10+ remainder still unproven",
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
                    A later SD13-E5 slice widens the level-1-only gate to level 2 (mirroring the \
                    Fighter/Paladin/Rogue level-range gate idiom): base-attack (classlevel = 2), \
                    base-save (Fortitude +3, Reflex +0, Will +0), and fast movement (unchanged \
                    flat +10 ft.) are extended to level 2 via the same formulas, and rage rounds \
                    per day grows by the PF1 Core Rulebook's own +2-rounds-per-level-after-1st \
                    rule (4 + Constitution modifier + 2 * (level - 1), 9 on the Con 16 fixture at \
                    level 2) — the flat rage constants and the illiteracy-absence record stay \
                    unchanged at level 2. A further SD13-E5 slice grounds Uncanny Dodge, the PF1 \
                    Core Rulebook Barbarian's 2nd-level \"Special\" class table entry (verified \
                    independently against d20pfsrd and legacy.aonprd.com, both naming \"Rage \
                    power, uncanny dodge\" as the level-2 row), as a bounded identity/recognition \
                    record only (value 0): correctly absent below level 2, granted-but-unexecuted \
                    at or above it, mirroring exactly how Rogue's/Monk's own Evasion and Druid's \
                    Woodland Stride were grounded, with no flat-footed-state tracking, no Armor \
                    Class computation, and no invisibility-detection engine implemented. The \
                    level-2 row's other named entry, a Rage Power choice (a genuinely \
                    open-ended choice-list feature, a new-subsystem-shaped burden), is \
                    deliberately left named-but-unproven, mirroring the Monk level-2 bonus feat \
                    grant / Bard Versatile Performance precedent. A still further SD13-E5 slice \
                    widens the gate to level 3 (verified independently against d20pfsrd and \
                    legacy.aonprd.com, both naming \"Trap sense +1\" as the level-3 row): \
                    base-attack (classlevel = 3), base-save (Fortitude +3, Reflex +1, Will +1), \
                    fast movement (unchanged flat +10 ft.), and rage rounds per day (4 + \
                    Constitution modifier + 2 * (level - 1), 11 on the Con 16 fixture at level 3, \
                    i.e. Constitution modifier + 8) are extended to level 3 via the same \
                    formulas, Uncanny Dodge stays granted (not re-derived), and Trap Sense is \
                    newly grounded as a bounded flat-magnitude record only (barbarian level / 3, \
                    floor; +1 at level 3): a bonus on Reflex saves made to avoid traps and an \
                    equal dodge bonus to AC against attacks made by traps, mirroring exactly how \
                    Rogue's own Trap Sense was grounded, never applied to any actual Reflex-save \
                    total or AC total, since no saving-throw-resolution or \
                    armor-class-resolution engine exists in this codebase, and no \
                    trap-detection or trap-triggering engine exists to decide when it would \
                    apply. A still further SD13-E5 slice widens the gate to level 4 (verified \
                    independently against d20pfsrd and legacy.aonprd.com, both naming the \
                    level-4 row as BAB +4, Fort +4, Ref +1, Will +1, Special \"Rage power\"): \
                    base-attack (classlevel = 4), base-save (Fortitude +4, Reflex +1, Will +1), \
                    fast movement (unchanged flat +10 ft.), and rage rounds per day (4 + \
                    Constitution modifier + 2 * (level - 1), 13 on the Con 16 fixture at level 4) \
                    are extended to level 4 via the same formulas, and Uncanny Dodge and Trap \
                    Sense both stay granted, not re-derived (Trap Sense stays at the same +1 \
                    magnitude, since the PF1 Core Rulebook bonus does not rise again until \
                    barbarian level 6). The level-4 row's only named \"Special\" entry is another \
                    Rage Power grant — the same genuinely open-ended choice-list feature already \
                    deliberately left named-but-unproven at level 2, not a new type of class \
                    feature — so no new pillar is grounded at level 4 beyond the arithmetic \
                    extension. A still further SD13-E5 slice widens the gate to level 5 \
                    (verified independently against d20pfsrd and legacy.aonprd.com, both naming \
                    the level-5 row as BAB +5, Fort +4, Ref +1, Will +1, Special \"Improved \
                    uncanny dodge\"): base-attack (classlevel = 5), base-save (Fortitude +4, \
                    Reflex +1, Will +1), fast movement (unchanged flat +10 ft.), and rage rounds \
                    per day (4 + Constitution modifier + 2 * (level - 1), 15 on the Con 16 \
                    fixture at level 5) are extended to level 5 via the same formulas, and \
                    Uncanny Dodge and Trap Sense both stay granted, not re-derived (Trap Sense \
                    stays at the same +1 magnitude, unchanged until barbarian level 6). The \
                    level-5 row's \"Special\" entry, Improved Uncanny Dodge, IS a genuinely new \
                    class feature (a barbarian can no longer be flanked at 5th level and higher, \
                    unless the attacker has at least four more rogue levels than the barbarian \
                    has barbarian levels) and its own grant is flat/identity-shaped exactly like \
                    Uncanny Dodge's own record, so it is newly grounded as a bounded \
                    identity/recognition record only (value 0): the rule's own conditional \
                    flanking-resolution piece (comparing the attacking rogue's own levels \
                    against the barbarian's own levels) is not computed, since no \
                    flanking-resolution, attacker-level-comparison, or sneak-attack-trigger \
                    engine exists anywhere in this codebase. A still further SD13-E5 slice widens \
                    the gate to level 6 (verified independently against d20pfsrd and \
                    legacy.aonprd.com, both naming the level-6 row as BAB +6, Fort +5, Ref +2, \
                    Will +2, Special \"Rage power, trap sense +2\"): base-attack (classlevel = \
                    6), base-save (Fortitude +5, Reflex +2, Will +2), fast movement (unchanged \
                    flat +10 ft.), and rage rounds per day (4 + Constitution modifier + 2 * \
                    (level - 1), 17 on the Con 16 fixture at level 6) are extended to level 6 via \
                    the same formulas, and Uncanny Dodge and Improved Uncanny Dodge both stay \
                    granted, not re-derived. Trap Sense's own flat magnitude GENUINELY RISES at \
                    level 6 (barbarian level / 3, floor: 6 / 3 = 2, up from 1 at levels 3-5) via \
                    the same pre-existing formula, matching the class table's own \"trap sense \
                    +2\" entry exactly. The level-6 row's other named \"Special\" entry is \
                    another Rage Power grant — the same genuinely open-ended choice-list feature \
                    already deliberately left named-but-unproven at levels 2 and 4, not a new \
                    type of class feature — so no new pillar is grounded at level 6 beyond the \
                    arithmetic extension and the Trap Sense magnitude rise. A still further \
                    SD13-E5 slice widens the gate to level 7 (verified independently against \
                    d20pfsrd and legacy.aonprd.com, both naming the level-7 row as BAB +7, \
                    Fort +5, Ref +2, Will +2, Special \"Damage reduction 1/-\"): base-attack \
                    (classlevel = 7), base-save (Fortitude +5, Reflex +2, Will +2), fast \
                    movement (unchanged flat +10 ft.), and rage rounds per day (4 + \
                    Constitution modifier + 2 * (level - 1), 19 on the Con 16 fixture at \
                    level 7) are extended to level 7 via the same formulas, and Uncanny Dodge, \
                    Trap Sense, and Improved Uncanny Dodge all stay granted, not re-derived \
                    (Trap Sense stays at the same +2 magnitude, unchanged until barbarian \
                    level 9). The level-7 row's \"Special\" entry, Damage Reduction 1/- \
                    (verified independently against d20pfsrd and legacy.aonprd.com: \"at 7th \
                    level, a barbarian gains damage reduction. Subtract 1 from the damage the \
                    barbarian takes each time she is dealt damage from a weapon or a natural \
                    attack\"), IS a genuinely new class feature, NOT another Rage Power grant — \
                    both primary sources confirm Rage Powers are granted at 2nd, 4th, 6th, 8th, \
                    and 10th barbarian level, not 7th, so there is no new Rage Power grant to \
                    leave named-but-unproven at this level and no \
                    rage-power-selection-slot-count engine is invented. Damage Reduction's own \
                    flat magnitude (1 point) is flat/identity-shaped exactly like Trap Sense's \
                    own magnitude, so it is newly grounded as a bounded flat-magnitude record \
                    only (value 1 at or above level 7, value 0 below it): the rule's own \
                    application piece (subtracting the value from incoming weapon/natural-attack \
                    damage) is not computed, since no damage-resolution engine or \
                    incoming-damage total exists anywhere in this codebase. A still further \
                    SD13-E5 slice widens the gate to level 8 (verified independently against \
                    d20pfsrd and legacy.aonprd.com, both naming the level-8 row as BAB +8, \
                    Fort +6, Ref +2, Will +2, Special \"Rage power\" only): base-attack \
                    (classlevel = 8), base-save (Fortitude +6, Reflex +2, Will +2), fast \
                    movement (unchanged flat +10 ft.), and rage rounds per day (4 + \
                    Constitution modifier + 2 * (level - 1), 21 on the Con 16 fixture at \
                    level 8) are extended to level 8 via the same formulas, and Uncanny Dodge, \
                    Trap Sense, Improved Uncanny Dodge, and Damage Reduction all stay granted, \
                    not re-derived (Trap Sense stays at the same +2 magnitude, unchanged until \
                    barbarian level 9; Damage Reduction stays at the same 1-point magnitude, \
                    unchanged until barbarian level 10). The level-8 row's \"Special\" entry is \
                    another Rage Power grant — both primary sources confirm Rage Powers are \
                    granted at 2nd, 4th, 6th, 8th, and 10th barbarian level, so this is the same \
                    genuinely open-ended choice-list feature already deliberately left \
                    named-but-unproven at levels 2, 4, and 6, not a new type of class feature — \
                    so no new pillar is grounded at level 8 beyond the arithmetic extension — \
                    AND a further SD13-E5 slice widens the level-range gate again \
                    (supported_barbarian_level, 1..=9) and extends every one of the formulas \
                    above to level 9 via the same formula, without re-derivation, verified \
                    independently against the PF1 Core Rulebook Barbarian class table (d20pfsrd \
                    and legacy.aonprd.com): level 9 base attack bonus genuinely rises to +9 \
                    (full BAB; the table's own \"+9/+4\" iterative notation is not modeled \
                    anywhere in this codebase, only the flat base value) while poor Reflex/Will \
                    both genuinely rise to +3 (9 / 3) and good Fortitude stays +6 (9 / 2 + 2, \
                    an integer-division coincidence); the rage rounds-per-day pool genuinely \
                    rises to 23 (4 + Constitution modifier 3 + 2 per level after 1st) while the \
                    four flat rage-surface magnitudes stay at their standard-rage values (the \
                    next change is Greater Rage at 11th, checked rather than assumed); the \
                    level-9 \"Special\" column reads \"Trap sense +3\" (verified \
                    independently against both primary sources, checked rather than assumed \
                    away) — a tier-rise on the already-grounded Trap Sense formula pillar \
                    (9 / 3 = 3, up from +2 at levels 6-8), not a new class feature; Damage \
                    Reduction stays 1/— (the next DR rise lands at 10th); Fast Movement stays \
                    the flat +10; level 9 is NOT a rage-power level (powers land at \
                    2/4/6/8/10...), so no new pillar is grounded and nothing new is left \
                    unproven for the rage-power list either. \
                    None of the grounded records are wired into the integrated \
                    base_attack_bonus/base-saves/speed/ability/Armor-Class/incoming-damage \
                    totals, so the integrated pilot surface still reports a blocked posture. The \
                    row remains explicitly blocked on the rage execution engine \
                    (activation/deactivation, rage-round consumption, fatigue after rage, \
                    temporary stat application). No weapon familiarity, Rage Power choice-list \
                    grounding, Improved Uncanny Dodge flanking-resolution engine, Damage \
                    Reduction application engine, or level-10+ martial progression is claimed",
                next_required_uplift: "ground the Barbarian rage-state execution engine \
                    (activation/deactivation, rage-round consumption, post-rage fatigue, \
                    temporary application of the rage constants), the Rage Power choice-list \
                    feature (now including the level-2, level-4, level-6, and level-8 grants), \
                    the Improved Uncanny Dodge flanking-resolution/attacker-level-comparison \
                    engine, the Damage Reduction application engine, and wire the grounded \
                    base-attack / base-save / fast-movement / Uncanny Dodge / Trap Sense / \
                    Improved Uncanny Dodge / Damage Reduction values into the integrated pilot \
                    surface, later widening into weapon familiarity and level-9+ martial \
                    progression",
            },
            SupportStateRow {
                row_id: "class.bard.progression_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:bard",
                dimension: "bounded spell-bearing class progression: the deterministic Human \
                            Bard level-1/level-2/level-3/level-4/level-5/level-6/level-7/level-8/\
                            level-9 \
                            spell baseline, with base attack bonus, base save progression, Bardic \
                            Knowledge, the flat Bardic Performance surface (rounds per day, \
                            inspire courage magnitude), the flat Fascinate DC / \
                            affected-creature-count formulas, (at level 2) the flat Well-Versed \
                            magnitude, (at level 3, rising at level 7) the flat Inspire \
                            Competence magnitude, and (at level 5) the flat Lore Master take-20 \
                            uses-per-day magnitude, all grounded for real at every supported \
                            level, and the bardic performance-execution burden (including \
                            Countersong, Distraction, Versatile Performance, and Suggestion) and \
                            the spontaneous known-spell / slot posture burden still blocked",
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
                    an earlier SD13-E5 slice grounds the flat bardic performance surface: the \
                    bardic performance rounds per day budget (4 + CHA modifier, i.e. 6 against \
                    the fixture's Charisma 15 at level 1) and the flat inspire courage level-1 \
                    magnitude (+1 competence bonus on attack and weapon damage rolls, +1 morale \
                    bonus on saves against charm and fear effects), AND a further SD13-E5 slice \
                    grounds the fascinate flat Will-save DC (10 + 1/2 bard level + CHA modifier, \
                    i.e. 12 against the fixture at level 1) and the fascinate flat \
                    affected-creature count (1 at 1st level, plus one more for every three bard \
                    levels beyond 1st, i.e. 1 at level 1) — both verified against the PF1 Core \
                    Rulebook Fascinate rule text, not assumed from memory, AND a further \
                    SD13-E5 slice grounds the foundational base-attack-bonus / base-save \
                    progression pillar that every other class row in this matrix (Fighter, \
                    Barbarian, Monk, Rogue, Paladin, Druid, Cleric) already had and Bard never \
                    had: base attack bonus (3/4 BAB, classlevel * 3 / 4, the same formula shape \
                    as Rogue/Monk/Druid/Cleric) and base save progression (good Reflex, good \
                    Will, poor Fortitude — the same save shape as Rogue, confirmed \
                    independently against the raw PF1 Core Rulebook Bard class table rather \
                    than assumed from Rogue's own pattern), both grounded as standalone \
                    explanation records not wired into compute_total_saves or \
                    compute_combat_baseline, AND a further SD13-E5 slice widens the level-1-only \
                    gate to level 2 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric \
                    level-range gate idiom), extending every formula above to level 2 via the \
                    same formula (level 2 base attack +1, base saves +0/+3/+3 \
                    Fortitude/Reflex/Will, Bardic Knowledge stays 1, Fascinate DC 13 and \
                    affected-creature count 1 on the fixture), widens the Bardic Performance \
                    rounds-per-day budget for real (PF1 Core Rulebook: 2 additional rounds per \
                    day at each level after 1st, i.e. 8 against the fixture's Charisma 15 at \
                    level 2, up from 6 at level 1 — verified this scales, unlike Inspire \
                    Courage's flat magnitude which is confirmed unchanged through level 2, first \
                    increasing only at bard level 5), and grounds Well-Versed, the flat \
                    non-level-scaled +4 bonus on saving throws against bardic performance, \
                    sonic, and language-dependent effects that the PF1 Core Rulebook Bard class \
                    table's level-2 Special column grants alongside Versatile Performance — \
                    Versatile Performance itself is NOT flat (it requires a choice of Perform \
                    type and an actual skill-substitution engine) and is deliberately left \
                    named-but-unproven, mirroring how the Monk level-2 bonus feat grant was \
                    deliberately left unrecognized. A further SD13-E5 slice widens the \
                    level-range gate to level 3 (mirroring the \
                    Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Druid/Sorcerer/Wizard idiom), \
                    extending every formula above to level 3 via the same formula (level 3 base \
                    attack +2, base saves +1/+3/+3 Fortitude/Reflex/Will, Bardic Knowledge stays \
                    1, Bardic Performance rounds per day rises to 10 on the fixture, Inspire \
                    Courage stays +1, Fascinate DC 13 and affected-creature count 1 on the \
                    fixture, Well-Versed stays granted at +4), and grounds Inspire Competence, \
                    the PF1 Core Rulebook Bard class table's level-3 Special column entry \
                    (\"Inspire competence +2\"), as a flat +2 standalone magnitude (a competence \
                    bonus on skill checks with a particular skill) — verified this is genuinely \
                    flat/identity-shaped at the one supported level, mirroring the Fighter \
                    Bravery / Rogue Trap Sense / Barbarian Trap Sense / Monk Still Mind idiom: \
                    never applied to any actual skill-check total, since no \
                    skill-check-resolution engine exists in this codebase. Unlike the Wizard \
                    specialist-bonus-slot or Cleric domain-slot doubling at level 3, Bard has no \
                    grounded spell-slot-count pillar at all to which an analogous doubling could \
                    apply (the Bard spells-per-day table's own 2nd-level spell column does not \
                    turn non-blank until 4th level, verified independently, and the entire \
                    spontaneous spell posture stays named-but-unproven below), so no such \
                    formula was added. A further SD13-E5 slice widens the level-range gate to \
                    level 4 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Druid/ \
                    Sorcerer/Wizard idiom), extending every formula above to level 4 via the \
                    same formula (level 4 base attack +3, base saves +1/+4/+4 \
                    Fortitude/Reflex/Will, Bardic Knowledge rises to 2, Bardic Performance \
                    rounds per day rises to 12 on the fixture, Inspire Courage stays +1, \
                    Fascinate DC 14 and affected-creature count 2 on the fixture, Well-Versed \
                    stays granted at +4, Inspire Competence stays granted at +2) — verified \
                    independently against both primary sources (d20pfsrd and \
                    legacy.aonprd.com) that the PF1 Core Rulebook Bard class table's level-4 \
                    Special column is BLANK (the next new class feature, Lore Master, is not \
                    gained until 5th level), so this widening grounds no new pillar. A further \
                    SD13-E5 slice widens the level-range gate to level 5 (mirroring the \
                    Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Druid/Sorcerer/Wizard/Ranger \
                    idiom), extending every formula above to level 5 via the same formula \
                    (level 5 base attack +3, base saves +1/+4/+4 Fortitude/Reflex/Will — both \
                    saves and the base attack numerically unchanged from level 4, an \
                    integer-division coincidence, not a sign either formula stopped scaling — \
                    Bardic Knowledge stays 2, Bardic Performance rounds per day rises to 14 on \
                    the fixture, Fascinate DC 14 and affected-creature count 2 on the fixture, \
                    both also numerically unchanged from level 4, Well-Versed stays granted at \
                    +4, Inspire Competence stays granted at +2) and grounds two genuinely NEW \
                    values at level 5, both re-verified independently against both primary \
                    sources rather than trusted from an earlier cycle's phrasing at face value: \
                    the Inspire Courage flat magnitude genuinely increases from +1 to +2 exactly \
                    at level 5 (PF1 Core Rulebook: \"At 5th level, and every six bard levels \
                    thereafter, this bonus increases by +1\" — the earlier cycle's \"stays +1 \
                    through level 5\" framing turns out to have been precise, not imprecise: \
                    level 4 stays +1, and level 5 is exactly the level the increase lands on), \
                    and Lore Master, the PF1 Core Rulebook's 5th-level Bard class feature \
                    (\"Inspire courage +2, lore master 1/day\"), is newly grounded as a bounded \
                    grant-only flat 1/day take-20 usage-count record only — the rule's OTHER \
                    capability, an at-will take-10-on-Knowledge-checks-with-ranks toggle, has no \
                    flat magnitude to ground, and neither the take-10 nor the take-20 mechanic \
                    is actually executed against any Knowledge check, since no \
                    skill-check-resolution engine exists anywhere in this codebase. A still \
                    further SD13-E5 slice widens the level-range gate to level 6 (mirroring the \
                    Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Druid/Sorcerer/Wizard/Ranger \
                    idiom), extending every formula above to level 6 via the same formula \
                    (level 6 base attack +4, base saves +2/+5/+5 Fortitude/Reflex/Will, Bardic \
                    Performance rounds per day rises to 16 on the fixture, Well-Versed stays \
                    granted at +4, Inspire Competence stays granted at +2, Lore Master stays \
                    granted at 1/day) and grounds two genuinely NEW values at level 6: Bardic \
                    Knowledge rises to 3 (max(6/2, 1)) and the Fascinate DC rises to 15 \
                    (10 + 6/2 + CHA modifier); the Fascinate affected-creature count stays 2, an \
                    integer-division coincidence with level 5, and Inspire Courage stays +2 (the \
                    next increase does not land until bard level 11). The PF1 Core Rulebook \
                    Bard class table's level-6 \"Special\" column (verified independently \
                    against both primary sources) reads \"Suggestion, Versatile performance\" — \
                    BOTH entries were checked and confirmed NOT flat: Suggestion is a \
                    spell-like ability requiring a fascinated-target prerequisite and the \
                    \"suggestion\" spell's own effect-resolution engine (neither exists in this \
                    codebase), and the 6th-level Versatile Performance grant is merely an \
                    additional instance of the SAME choice-gated skill-substitution engine \
                    already deliberately left named-but-unproven at 2nd level, not a new type of \
                    class feature — so no new pillar record is grounded at level 6. A still \
                    further SD13-E5 slice widens the level-range gate to level 7 (mirroring the \
                    Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Druid/Sorcerer/Wizard/Ranger \
                    idiom), extending every formula above to level 7 via the same formula (level \
                    7 base attack genuinely rises to 5, up from 4 at level 6; base saves \
                    (Fortitude 2, Reflex 5, Will 5) stay numerically unchanged from level 6, an \
                    integer-division coincidence re-verified against the raw class table row \
                    rather than assumed; Bardic Knowledge stays 3; Bardic Performance rounds per \
                    day rises to 18 on the fixture; Inspire Courage stays +2; the Fascinate DC \
                    stays 15, another integer-division coincidence; Well-Versed stays granted at \
                    +4; Lore Master stays granted at 1/day) and grounds two genuinely NEW values \
                    at level 7: the Fascinate affected-creature count rises to 3 \
                    (1 + (7-1)/3, up from 2 at level 6, confirmed by direct arithmetic against \
                    the primary source rule text rather than assumed), and Inspire Competence's \
                    flat magnitude genuinely rises from +2 to +3 (PF1 Core Rulebook: \"This bonus \
                    increases by +1 for every four levels the bard has attained beyond 3rd (+3 \
                    at 7th...)\", i.e. 2 + (level-3)/4, verified independently against both \
                    primary sources, mirroring the Inspire Courage second-tier widening idiom \
                    exactly rather than treating this as a new class feature). The PF1 Core \
                    Rulebook Bard class table's level-7 \"Special\" column (verified \
                    independently against both primary sources) reads only \"Inspire competence \
                    +3\", so no other new pillar record is grounded at level 7. A still further \
                    SD13-E5 slice widens the level-range gate to level 8 (mirroring the \
                    Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Druid/Sorcerer/Wizard/Ranger \
                    idiom), extending every formula above to level 8 via the same formula (level \
                    8 base attack genuinely rises to 6, up from 5 at level 7, the class table's \
                    own iterative-attack notation \"+6/+1\" not modeled anywhere in this \
                    codebase, only the flat base value; base Fortitude stays 2, an \
                    integer-division coincidence with level 7; base Reflex and Will both \
                    genuinely rise to 6, up from 5; Bardic Knowledge genuinely rises to 4, up \
                    from 3; Bardic Performance rounds per day continues scaling; the Fascinate DC \
                    genuinely rises to 16, up from 15; the Fascinate affected-creature count \
                    stays 3, an integer-division coincidence confirmed by direct arithmetic \
                    against the primary source rule text rather than trusted from the formula \
                    alone; Inspire Courage stays +2 and Inspire Competence stays +3, neither's \
                    next tier landing until bard level 11; Well-Versed and Lore Master both stay \
                    granted). The PF1 Core Rulebook Bard class table's level-8 \"Special\" column \
                    (verified independently against both primary sources) reads \"Dirge of doom\" \
                    — a genuinely NEW bardic-performance type, checked and confirmed NOT \
                    flat/identity-shaped: it requires both the same performance-state engine \
                    already left ungrounded and a fear/shaken-condition resolution engine, \
                    neither of which exists in this codebase, so it is deliberately left \
                    named-but-unproven, mirroring the Suggestion / Countersong / Distraction \
                    precedent exactly — no explanation record is fabricated for it — AND a \
                    further SD13-E5 slice widens the level-range gate again \
                    (supported_bard_level, 1..=9) and extends every one of the formulas above \
                    to level 9 via the same formula, without re-derivation, verified \
                    independently against the PF1 Core Rulebook Bard class table (d20pfsrd and \
                    legacy.aonprd.com): level 9 base attack stays +6 (9 * 3 / 4) and good \
                    Reflex/Will both stay +6 (9 / 2 + 2), integer-division coincidences, while \
                    poor Fortitude genuinely rises to +3 (9 / 3); the Bardic Performance \
                    rounds-per-day pool genuinely rises to 22 (4 + Charisma modifier 2 + 2 per \
                    level after 1st); Bardic Knowledge stays 4, the Fascinate DC stays 16 and \
                    its affected-creature count stays 3 (the next rise lands at 10th), Inspire \
                    Courage stays +2 and Inspire Competence stays +3 (both next tiers land at \
                    11th), Well-Versed stays +4, and Lore Master stays 1/day (next rise at \
                    11th) — each checked rather than assumed; UNLIKE the level-8 Dirge of Doom \
                    entry, the level-9 \"Special\" column reads \"Inspire greatness\" \
                    (verified independently against both primary sources, checked rather than \
                    assumed away) — a genuinely NEW bardic-performance type confirmed NOT flat \
                    (it grants 2 bonus Hit Dice with commensurate temporary hit points, a +2 \
                    competence attack bonus, and a +1 competence Fortitude bonus to a willing \
                    ally, requiring the performance-state engine plus \
                    temporary-Hit-Dice/temporary-hit-point mechanics, none of which exist in \
                    this codebase), so it is deliberately left named-but-unproven, mirroring \
                    the Suggestion / Countersong / Distraction / Dirge-of-Doom precedent \
                    exactly — no explanation record or diagnostic is fabricated for it. The row is \
                    Partial, not Supported: the performance-state \
                    engine (start/maintain action economy, round tracking/consumption of the \
                    grounded budget, no application of the grounded fascinate DC, count, \
                    Well-Versed, Inspire Competence, or Lore Master magnitude to any actual \
                    save, skill check, or targeting) is not implemented, the two remaining \
                    level-1 performances (countersong, distraction), Versatile Performance (both \
                    its 2nd-level and 6th-level grants), Suggestion, and Dirge of Doom (the \
                    level-8 \"Special\" column entry) are not grounded at all \
                    — Countersong/Distraction require an opposed Perform-check-vs-effect \
                    substitution resolution, Versatile Performance requires a choice-gated \
                    skill-substitution engine, Suggestion requires a fascinated-target \
                    prerequisite plus the suggestion spell's own effect-resolution engine, and \
                    Dirge of Doom requires both the performance-state engine and a \
                    fear/shaken-condition resolution engine, none a flat number — and the entire \
                    spontaneous spell burden (spontaneous spells known, spells per day, bonus \
                    spell slots from CHA, spell save DCs, school choice, prepared posture) is not \
                    computed. No performance-execution math and no spell math is fabricated and \
                    no Bard level 10+ is proven",
                next_required_uplift: "SD13-E5+ Bard performance-execution engine slice \
                    (start/maintain action economy, round tracking, application of the grounded \
                    Inspire Courage / Fascinate / Well-Versed / Inspire Competence / Lore Master \
                    magnitudes, Countersong / Distraction opposed Perform-check-vs-effect \
                    grounding, Versatile Performance's choice-gated skill-substitution engine \
                    (both grants), Suggestion's fascinated-target-plus-spell-effect resolution, \
                    Dirge of Doom's fear/shaken-condition resolution, Inspire Greatness's \
                    bonus-Hit-Dice/temporary-hit-point mechanics, Lore Master's own \
                    take-10/take-20 skill-check-resolution execution), then the spontaneous \
                    spell-slot burden, then level-10+ progression",
            },
            SupportStateRow {
                row_id: "class.cleric.progression_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:cleric",
                dimension: "bounded spell-bearing class progression: the deterministic Human Cleric \
                            level-1/level-2/level-3/level-4/level-5/level-6/level-7/level-8/level-9 prepared \
                            divine spell baseline, with base attack bonus, base save progression, \
                            Channel Energy (die count increasing to 2d6 at level 3, unchanged at level \
                            4, increasing to 3d6 at level 5, unchanged at level 6, increasing to 4d6 at \
                            level 7, unchanged at level 8), the domain choice seam, the flat domain \
                            spell slot count (increasing to 2 at level 3, unchanged at level 4, \
                            increasing to 3 at level 5, unchanged at level 6, increasing to 4 at level \
                            7, unchanged at level 8), Touch of Good (Good domain, in full, sacred bonus \
                            increasing to 2 at level 4, unchanged at level 5, increasing to 3 at level \
                            6, unchanged at level 7, increasing to 4 at level 8), and Rebuke Death's \
                            uses per day (Healing domain, partial) grounded for real at every supported \
                            level and the Rebuke Death heal amount and the prepared divine spell \
                            posture burden still blocked",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_CLERIC_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E4 leaves direct computed evidence that the \
                    deterministic Human Cleric level-1 prepared divine spell-bearing identity is \
                    recognized on the compute seam, the SD13-E4 Cleric Channel Energy slice grounds \
                    Channel Energy for real (PF1 Core Rulebook: ceil(cleric level / 2) d6, minimum 1d6, \
                    computed against the fixture as 1d6 at level 1; usable 3 + Charisma modifier times \
                    per day, computed against the fixture's Charisma 14 (+2) as 5 uses per day), the \
                    SD13-E5 Cleric domain slice grounds the domain choice seam (the two canonical \
                    fixture selections choice:cleric_domain -> domain:good and domain:healing, \
                    surfaced as an explicit choice seam carrying no mechanical value) and the flat \
                    domain spell slot count (PF1 Core Rulebook Domains: one domain spell slot per \
                    level of cleric spells she can cast, 1st and up — exactly one 1st-level domain \
                    slot at level 1; the slot's contents are not grounded), AND the SD13-E5 Cleric \
                    domain powers slice grounds the Good domain's granted power Touch of Good in full \
                    (PF1 Core Rulebook Good Domain, verified against primary source text: a flat \
                    sacred bonus equal to half cleric level, minimum 1 — 1 at level 1 — and a flat \
                    3 + Wisdom modifier uses-per-day count — 6 against the fixture's Wisdom 17) and \
                    the Healing domain's granted power Rebuke Death's flat uses-per-day count only \
                    (PF1 Core Rulebook Healing Domain, same verified 3 + Wisdom modifier formula — 6 \
                    against the fixture), AND a later SD13-E5 slice grounds the foundational base \
                    attack and base save progression pillar that every other class row in this \
                    matrix already had and Cleric never had: base attack bonus (3/4 BAB, classlevel \
                    * 3 / 4, the same formula shape as Rogue/Monk/Druid) and base save progression \
                    (good Fortitude, good Will, poor Reflex), both verified against the PF1 Core \
                    Rulebook Cleric class table and grounded as standalone explanation records not \
                    wired into compute_total_saves or compute_combat_baseline, AND a later SD13-E5 \
                    slice widens the level-1-only gate to a level-range gate (level 1-2), extending \
                    every one of the above formulas to level 2 via the same formula, not re-derived \
                    (verified independently against the PF1 Core Rulebook Cleric class table via \
                    d20pfsrd and legacy.aonprd.com): level 2 base attack bonus is +1, base \
                    Fortitude/Will are +3, base Reflex is +0; Channel Energy stays 1d6 (it next \
                    increases only at level 3); the domain spell slot count stays exactly 1 (a \
                    level-2 cleric still only casts 1st-level cleric spells — 2nd-level cleric spells \
                    begin at caster level 3); Touch of Good's sacred bonus stays 1 (reached naturally \
                    at level 2 rather than via the level-1 floor); and Cleric gains no new class \
                    feature at 2nd level (the class table's level-2 Special column is blank, unlike \
                    Rogue/Monk's Evasion), so no new pillar was added, only the existing ones widened. \
                    A further SD13-E5 slice widens the gate again to level 3 (verified independently \
                    against the PF1 Core Rulebook Cleric class table and spells-per-day table via \
                    d20pfsrd and legacy.aonprd.com): level 3 base attack bonus is +2, base \
                    Fortitude/Will are +3, base Reflex is +1; Channel Energy's die count genuinely \
                    increases to 2d6 (ceil(3/2) = 2, the class table's level-3 Special column reads \
                    \"Channel energy 2d6\"); the domain spell slot count genuinely increases to 2 (a \
                    level-3 cleric casts 2nd-level cleric spells for the first time, so the count is \
                    one 1st-level plus one 2nd-level domain slot, mirroring the Wizard specialist \
                    bonus slot's own level-3 doubling); Touch of Good's sacred bonus stays 1 (3/2 = 1, \
                    reached naturally, unchanged); and Cleric's level-3 Special column names only the \
                    Channel Energy increase, so no new pillar was added at level 3, only the existing \
                    ones widened (two of them, Channel Energy dice and the domain spell slot count, \
                    to genuinely new values). A further SD13-E5 slice widens the gate again to level 4 \
                    (verified independently against the PF1 Core Rulebook Cleric class table, \
                    spells-per-day table, and the Good Domain granted-power rule text via d20pfsrd and \
                    legacy.aonprd.com): level 4 base attack bonus is +3, base Fortitude/Will are +4, \
                    base Reflex is +1; Channel Energy's die count stays 2d6 (ceil(4/2) = 2, unchanged \
                    from level 3, it next increases only at level 5); the domain spell slot count \
                    stays 2 (a level-4 cleric's 3rd-level spell column is still \"—\" on the raw \
                    spells-per-day table, so 3rd-level cleric spells begin only at level 5); the Good \
                    domain's Touch of Good sacred bonus genuinely increases to 2 (half cleric level, \
                    minimum 1: max(4/2, 1) = 2, up from 1); Rebuke Death's uses per day stays the same \
                    3 + Wisdom modifier formula, unchanged; and Cleric's level-4 Special column is \
                    blank (no new class feature is gained at 4th level), so no new pillar was added at \
                    level 4, only the existing Touch of Good pillar widened to a genuinely new value. \
                    A further SD13-E5 slice widens the gate again to level 5 (verified independently \
                    against the PF1 Core Rulebook Cleric class table and spells-per-day table via \
                    d20pfsrd and legacy.aonprd.com): level 5 base attack bonus is +3, base \
                    Fortitude/Will are +4, base Reflex is +1 (unchanged from level 4, reached \
                    naturally); Channel Energy's die count genuinely increases to 3d6 (ceil(5/2) = 3, \
                    the class table's level-5 Special column reads \"Channel energy 3d6\"); the \
                    domain spell slot count genuinely increases to 3 (a level-5 cleric casts \
                    3rd-level cleric spells for the first time, so the count is one 1st-level plus \
                    one 2nd-level plus one 3rd-level domain slot); the Good domain's Touch of Good \
                    sacred bonus stays 2 (max(5/2, 1) = 2, unchanged from level 4, it next increases \
                    only at level 6); Rebuke Death's uses per day stays the same 3 + Wisdom modifier \
                    formula, unchanged; and Cleric's level-5 Special column names only the Channel \
                    Energy increase, so no new pillar was added at level 5, only the existing Channel \
                    Energy and domain spell slot count pillars widened to genuinely new values. \
                    A further SD13-E5 slice widens the gate again to level 6 (verified independently \
                    against the PF1 Core Rulebook Cleric class table and spells-per-day table via \
                    d20pfsrd and legacy.aonprd.com): level 6 base attack bonus is +4, base \
                    Fortitude/Will are +5, base Reflex is +2; Channel Energy's die count stays 3d6 \
                    (ceil(6/2) = 3, unchanged from level 5, since the die count rises only every odd \
                    cleric level); the domain spell slot count stays 3 (a level-6 cleric's 4th-level \
                    spell column is still \"—\" on the raw spells-per-day table, so 4th-level cleric \
                    spells are not yet available); the Good domain's Touch of Good sacred bonus \
                    genuinely increases to 3 (half cleric level, minimum 1: max(6/2, 1) = 3, up from \
                    2); Rebuke Death's uses per day stays the same 3 + Wisdom modifier formula, \
                    unchanged; and Cleric's level-6 Special column is genuinely blank (no new class \
                    feature is gained at 6th level), so no new pillar was added at level 6 either, \
                    only the existing Touch of Good pillar widened to a genuinely new value. \
                    A further SD13-E5 slice widens the gate again to level 7 (verified independently \
                    against the PF1 Core Rulebook Cleric class table and spells-per-day table via \
                    d20pfsrd and legacy.aonprd.com): level 7 base attack bonus is +5, base \
                    Fortitude/Will are +5 (unchanged from level 6, reached naturally), base Reflex is \
                    +2 (unchanged from level 6, reached naturally); Channel Energy's die count \
                    genuinely increases to 4d6 (ceil(7/2) = 4, the class table's level-7 Special \
                    column reads \"Channel energy 4d6\", confirming level 7 IS one of the odd cleric \
                    levels where the die count rises); the domain spell slot count genuinely \
                    increases to 4 (a level-7 cleric casts 4th-level cleric spells for the first \
                    time, the raw spells-per-day table's level-7 row being the first to show a \
                    non-\"—\" 4th-level spell column, so the count is one 1st-level plus one \
                    2nd-level plus one 3rd-level plus one 4th-level domain slot); the Good domain's \
                    Touch of Good sacred bonus stays 3 (max(7/2, 1) = 3, unchanged from level 6, it \
                    next increases only at level 8); Rebuke Death's uses per day stays the same 3 + \
                    Wisdom modifier formula, unchanged; and Cleric's level-7 Special column names \
                    only the Channel Energy increase, so no new pillar was added at level 7 either, \
                    only the existing Channel Energy and domain spell slot count pillars widened to \
                    genuinely new values. \
                    A further SD13-E5 slice widens the gate again to level 8 (verified independently \
                    against the PF1 Core Rulebook Cleric class table and spells-per-day table via \
                    d20pfsrd and legacy.aonprd.com): level 8 base attack bonus is +6, base \
                    Fortitude/Will are +6, base Reflex is +2 (unchanged from level 7, reached \
                    naturally); Channel Energy's die count stays 4d6 (ceil(8/2) = 4, unchanged from \
                    level 7, since the die count rises only every odd cleric level); the domain spell \
                    slot count stays 4 (a level-8 cleric's 5th-level spell column is still \"—\" on \
                    the raw spells-per-day table, so 5th-level cleric spells are not yet available, \
                    only beginning at level 9); the Good domain's Touch of Good sacred bonus \
                    genuinely increases to 4 (half cleric level, minimum 1: max(8/2, 1) = 4, up from \
                    3); Rebuke Death's uses per day stays the same 3 + Wisdom modifier formula, \
                    unchanged; and Cleric's level-8 Special column is genuinely blank (no new class \
                    feature is gained at 8th level, and the class table's own iterative-attack \
                    notation \"+6/+1\" on the level-8 base-attack column is not modeled anywhere in \
                    this codebase, only the flat base value), so no new pillar was added at level 8 \
                    either, only the existing Touch of Good pillar widened to a genuinely new value. \
                    A further SD13-E5 slice widens the gate again to level 9 (verified \
                    independently against the PF1 Core Rulebook Cleric class table and \
                    spells-per-day table via d20pfsrd and legacy.aonprd.com): level 9 base attack \
                    stays +6 (9 * 3 / 4) and good Fortitude/Will both stay +6 (9 / 2 + 2), \
                    integer-division coincidences, while poor Reflex genuinely rises to +3 \
                    (9 / 3); the level-9 \"Special\" column reads \"Channel energy 5d6\" — a \
                    tier-rise on the already-grounded die-count pillar ((9 + 1) / 2 = 5, up from \
                    4d6 at levels 7-8, the odd-level cadence), not a new class feature, with the \
                    uses-per-day pool staying the level-independent 3 + Charisma modifier; \
                    5th-level cleric spells first appear at 9th (the level-9 spells-per-day row \
                    is \"4/4+1/4+1/3+1/2+1/1+1\", the first non-\"—\" 5th-level column), so \
                    the domain spell slot count genuinely rises to 5 via the same \
                    one-slot-per-castable-spell-level rule; Touch of Good's bonus stays 4 \
                    (9 / 2, an integer-division coincidence) and both domain-power uses-per-day \
                    pools stay level-independent, so no new pillar is added at level 9 either — \
                    only the existing Channel Energy and domain spell slot pillars widened to \
                    genuinely new values. \
                    The row is Partial, not Supported: Rebuke Death's heal amount (1d4 points of \
                    damage plus 1 for every two cleric levels, usable only on a creature below 0 hit \
                    points) is not a flat number and remains named and unproven, the domain \
                    spell-list contents that could fill the grounded domain spell slots remain named \
                    and unproven, the prepared divine spell posture burden (spells prepared from the \
                    full Cleric list, spontaneous cure/inflict conversion, spell slots per day, bonus \
                    spells from a high Wisdom, spell save DCs) is still entirely unproven, and no \
                    Cleric level 10+ is proven. No touch-attack resolution, healing-application \
                    engine, hit-point-state gating check, or per-use consumption tracking is \
                    fabricated",
                next_required_uplift: "SD13-E5+ Cleric domain power grounding: the Rebuke Death \
                    heal-amount piece (requires a dice-roll execution engine and a hit-point-state \
                    gating check, a new-subsystem-shaped burden deliberately not attempted this \
                    slice) and domain spell-list contents, then the prepared divine spell posture \
                    burden, then Cleric level 10+ progression",
            },
            SupportStateRow {
                row_id: "class.druid.progression_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:druid",
                dimension: "bounded spell-bearing class progression: the deterministic Human Druid \
                            level-1/level-2/level-3/level-4/level-5/level-6/level-7/level-8 \
                            prepared divine \
                            spell baseline, with base attack bonus, base save progression, Wild \
                            Empathy, Nature Sense, the nature-bond choice recognition, (at level \
                            2) Woodland Stride, (at level 3) Trackless Step, and (at level 4) \
                            Resist Nature's Lure grounded for real at all eight supported levels, \
                            and the animal-companion execution burden, the Wild Shape execution \
                            burden, and the prepared divine spell posture burden still blocked",
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
                    bond:animal_companion, a +0 recognition record with no bond execution), AND a \
                    later SD13-E5 slice grounds the foundational base attack and base save \
                    progression pillar that every other class row in this matrix already had and \
                    Druid never had: base attack bonus (3/4 BAB, classlevel * 3 / 4, the same \
                    formula shape as Rogue/Monk) and base save progression (good Fortitude, good \
                    Will, poor Reflex), both verified against the PF1 Core Rulebook Druid class \
                    table and grounded as standalone explanation records not wired into \
                    compute_total_saves or compute_combat_baseline, AND a later SD13-E5 slice widens \
                    the level-1-only gate to a level-range gate (level 1-2), extending every one of \
                    the above formulas to level 2 via the same formula, not re-derived (verified \
                    independently against the PF1 Core Rulebook Druid class table via d20pfsrd and \
                    legacy.aonprd.com): level 2 base attack bonus is +1, base Fortitude/Will are +3, \
                    base Reflex is +0; Wild Empathy's modifier is level-generic by construction and \
                    grounds correctly to 3 (2 + Charisma modifier 1) at level 2; Nature Sense stays \
                    the flat +2 bonus and the nature-bond choice recognition is not level-gated, both \
                    confirmed unchanged at level 2 via the same formula, not new records; and the \
                    Druid class table's level-2 \"Special\" column reads \"Woodland stride\" \
                    (verified independently against both primary sources), a new, flat/identity- \
                    shaped class feature grounded as a bounded identity record (value 0, mirroring \
                    exactly how Rogue's/Monk's own Evasion was grounded): a druid may move through \
                    natural undergrowth at normal speed without damage or impediment, with no \
                    terrain-detection engine and no movement-execution engine fabricated. A still \
                    further SD13-E5 slice widens the gate to level 3 (verified independently against \
                    d20pfsrd and legacy.aonprd.com): level 3 base attack bonus is +2, base saves are \
                    +3/+1/+3 (Fortitude/Reflex/Will), extended via the same formulas; Wild Empathy \
                    grounds correctly to 4 (3 + Charisma modifier 1) and Nature Sense stays the flat \
                    +2 bonus, both via the same formula, not new records; Woodland Stride stays \
                    granted, not re-derived; and the Druid class table's level-3 \"Special\" column \
                    reads \"Trackless step\" (verified independently against both primary sources), \
                    a new, flat/identity-shaped class feature grounded as a bounded identity record \
                    (value 0, mirroring exactly how Woodland Stride was grounded): starting at 3rd \
                    level, a druid leaves no trail in natural surroundings and cannot be tracked, \
                    with no tracking-resolution engine and no terrain-detection engine fabricated. \
                    Druid has no currently-grounded spell-slot-count pillar (unlike Wizard's \
                    specialist bonus slot or Cleric's domain slot), so there is no analogous level-3 \
                    doubling to widen. A still further SD13-E5 slice widens the gate to level 4 \
                    (verified independently against d20pfsrd and legacy.aonprd.com): level 4 base \
                    attack bonus is +3, base saves are +4/+1/+4 (Fortitude/Reflex/Will), extended via \
                    the same formulas; Wild Empathy grounds correctly to 5 (4 + Charisma modifier 1) \
                    and Nature Sense stays the flat +2 bonus, both via the same formula, not new \
                    records; Woodland Stride and Trackless Step both stay granted, not re-derived; \
                    and the Druid class table's level-4 \"Special\" column reads \"Resist nature's \
                    lure, wild shape (1/day)\" — TWO distinct entries, both checked independently \
                    rather than assumed. Resist Nature's Lure is a new, flat/identity-shaped class \
                    feature grounded as a bounded flat-magnitude identity record (value 4, mirroring \
                    exactly how Bravery/Divine Grace/Trap Sense were grounded): a druid gains a +4 \
                    bonus on saving throws against the spell-like and supernatural abilities of fey, \
                    a bonus that also applies to spells and effects that target plants, with no \
                    saving-throw resolution engine fabricated. Wild Shape (1/day) was checked and \
                    confirmed NOT flat — a full shapeshifting subsystem (new form, new stat block, \
                    duration tracking) with no execution engine anywhere in this codebase — so it is \
                    deliberately left named-but-unproven, exactly like the animal-companion execution \
                    burden. A still further SD13-E5 slice widens the gate to level 5 (verified \
                    independently against d20pfsrd and legacy.aonprd.com): level 5 base attack bonus \
                    is +3, base saves are +4/+1/+4 (Fortitude/Reflex/Will) — all three numerically \
                    unchanged from level 4 (integer-division coincidences of the same formulas, not a \
                    sign any formula stopped scaling), extended via the same formulas, not re-derived; \
                    Wild Empathy grounds correctly to 6 (5 + Charisma modifier 1) and Nature Sense \
                    stays the flat +2 bonus, both via the same formula, not new records; Woodland \
                    Stride, Trackless Step, and Resist Nature's Lure all stay granted, not \
                    re-derived; and the Druid class table's level-5 \"Special\" column is genuinely \
                    blank (verified independently against both primary sources rather than assumed), \
                    so this slice grounds no new pillar at level 5 — only the existing pillars are \
                    widened. A still further SD13-E5 slice widens the gate to level 6 (verified \
                    independently against d20pfsrd and legacy.aonprd.com): level 6 base attack bonus \
                    is +4, base saves are +5/+2/+5 (Fortitude/Reflex/Will) — all three genuinely new \
                    values, up from +3/+1/+4 at level 5 — extended via the same formulas, not \
                    re-derived; Wild Empathy grounds correctly to 7 (6 + Charisma modifier 1) and \
                    Nature Sense stays the flat +2 bonus, both via the same formula, not new records; \
                    Woodland Stride, Trackless Step, and Resist Nature's Lure all stay granted, not \
                    re-derived; and the Druid class table's level-6 \"Special\" column reads \"Wild \
                    shape (2/day)\", checked independently against both primary sources and confirmed \
                    NOT a genuinely separable flat/identity-shaped element — the \"2/day\" frequency \
                    increase is bundled with a form-list expansion (Large/Tiny animal or Small \
                    elemental forms) and a functioning-level upgrade (beast shape II / elemental body \
                    I), none of which exist in this codebase's engine-free record set, so Wild Shape \
                    (including this level-6 change) stays deliberately named-but-unproven and this \
                    slice grounds no new pillar at level 6 either. A still further SD13-E5 slice \
                    widens the gate to level 7 (verified independently against d20pfsrd and \
                    legacy.aonprd.com): level 7 base attack bonus is +5, a genuinely new value up \
                    from +4 at level 6; base saves are +5/+2/+5 (Fortitude/Reflex/Will), all three \
                    numerically unchanged from level 6 (an integer-division coincidence, \
                    re-verified against the raw class table rather than assumed), extended via the \
                    same formulas, not re-derived; Wild Empathy grounds correctly to 8 (7 + \
                    Charisma modifier 1) and Nature Sense stays the flat +2 bonus, both via the \
                    same formula, not new records; Woodland Stride, Trackless Step, and Resist \
                    Nature's Lure all stay granted, not re-derived; and the Druid class table's \
                    level-7 \"Special\" column is genuinely blank (verified independently against \
                    both primary sources rather than assumed) — Wild Shape's next usage-count \
                    increase (\"Wild shape (3/day)\") does not land until 8th level, so this slice \
                    makes no Wild Shape claim at level 7 either way, and no new pillar is grounded \
                    at level 7 — AND a further SD13-E5 slice widens the level-range gate again \
                    (supported_druid_level, 1..=8) and extends every one of the formulas above to \
                    level 8 via the same formula, without re-derivation, verified independently \
                    against the PF1 Core Rulebook Druid class table (d20pfsrd and \
                    legacy.aonprd.com): level 8 base attack bonus is +6 (genuinely risen from +5; \
                    the class table's own \"+6/+1\" iterative-attack notation is not modeled \
                    anywhere in this codebase, only the flat base value) and base saves are \
                    +6/+2/+6 (Fortitude/Reflex/Will — both good saves genuinely rise from +5, \
                    while poor Reflex stays +2, an integer-division coincidence); Wild Empathy \
                    genuinely rises to 9 (8 + Charisma modifier 1) via the same level-generic \
                    formula; Nature Sense stays the flat +2; Woodland Stride, Trackless Step, and \
                    Resist Nature's Lure all stay granted, not re-derived; UNLIKE the blank \
                    level-7 column, the class table's level-8 \"Special\" column reads \"Wild \
                    shape (3/day)\" (verified independently against both primary sources, checked \
                    rather than assumed away) — but per the level-4/level-6 precedent the rule \
                    text bundles that frequency increase with a form-list expansion \
                    (Huge/Diminutive animal, Medium elemental, Small/Medium plant) and \
                    functioning-level upgrades (beast shape III / elemental body II / plant shape \
                    I), none of which are separable from the \"3/day\" numeral without \
                    misrepresenting the bundled feature as flat, so Wild Shape stays entirely \
                    named-but-unproven and no new pillar is grounded at level 8 either — only the \
                    existing pillars are widened. The row is Partial, not Supported: the animal companion execution \
                    burden (the companion's stat block, its advancement, and its link / share \
                    spells abilities) remains named and unproven, the Wild Shape execution burden \
                    (new form, new stat block, duration tracking, frequency, and the level-6 \
                    form-list expansion) remains named and unproven, and the prepared divine spell \
                    posture burden (spells prepared from the full Druid list, spontaneous summon \
                    nature's ally conversion, spell slots per day, bonus spells from a high Wisdom, \
                    spell save DCs) is still entirely unproven. No spell math is fabricated and no \
                    Druid level 9+ is proven",
                next_required_uplift: "SD13-E5 Druid animal companion execution slice, the Wild \
                    Shape execution slice, or the prepared divine spell burden slice, then Druid \
                    level 8+ progression (out of scope for this slice)",
            },
            SupportStateRow {
                row_id: "class.monk.bounded_progression",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:monk",
                dimension: "bounded Monk martial chassis progression: the deterministic Human \
                            Monk level-1/level-2/level-3/level-4/level-5/level-6/level-7/level-8/\
                            level-9 \
                            martial chassis identity, with base-attack, base-save, AC Bonus, the \
                            unarmed strike damage die (genuinely rising to 1d10 at level 8, \
                            unchanged at level 9), the \
                            Flurry of Blows flat attack surface (attack count genuinely rising to \
                            3 at level 8, with the flat attack modifier genuinely rising to +7 at \
                            level 9), and the level-1 bonus feat choice-slot selection \
                            grounded across all nine levels, Evasion grounded as a level-2 \
                            identity/recognition record, Still Mind grounded as a level-3 \
                            flat-magnitude record, the ki pool's flat size (genuinely rising to 6 \
                            at level 6, unchanged at level 7, rising to 7 at level 8) and Slow \
                            Fall grounded as level-4 records (Slow Fall's own reach magnitude \
                            genuinely rising to 30 ft. at level 6 and 40 ft. at level 8), Purity \
                            of Body grounded as a level-5 grant-only identity record (High Jump \
                            checked and confirmed not flat), Wholeness of Body (the level-7 class \
                            table's new named feature) checked and confirmed not flat and staying \
                            granted-but-unexecuted at level 8, the level-8 \"Special\" column \
                            checked and confirmed to name only the Slow Fall reach rise (not \
                            Improved Uncanny Dodge, which Monk never gains at any level), \
                            Improved Evasion grounded as a level-9 identity/recognition record \
                            (the level-9 \"Special\" column's only entry), and the \
                            recognized bonus feat's own mechanics still unproven",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_MONK_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E3/E5 leaves direct computed evidence that the \
                    deterministic Human Monk level-1/level-2/level-3/level-4/level-5/level-6 \
                    martial chassis identity is recognized on the compute seam, and now grounds ten \
                    named pillar burdens across all five levels: base attack progression (3/4 \
                    BAB), base save progression (good Fortitude, Reflex, and Will), AC Bonus \
                    (Wisdom-to-AC, the flat value at the supported level), the unarmed strike / \
                    Flurry of Blows flat surface (Medium monk 1d6 unarmed damage at levels 1-3, \
                    stepping up to 1d8 at levels 4-5 — die size only, no damage roll or total — \
                    and the flurry posture of two attacks at monk level - 2 before ability \
                    modifiers, i.e. -1 at level 1, +0 at level 2, +1 at level 3, +2 at level 4, \
                    and +3 at level 5, with the attack count staying 2 at all five levels), the \
                    level-1 bonus \
                    feat choice-slot selection (recognized when it names one of the PF1 Core \
                    Rulebook restricted Monk bonus feat list's five feats: Combat Reflexes, \
                    Deflect Arrows, Improved Grapple, Improved Trip, Stunning Fist — a +0 \
                    recognition record with no feat-effect execution, mirroring the Sorcerer \
                    bloodline choice / Cleric domain choice / Druid nature-bond choice \
                    recognition idiom), (SD13-E5) Evasion, a 2nd-level Monk class feature \
                    verified independently against two primary PF1 sources (d20pfsrd and \
                    legacy.aonprd.com), grounded as a bounded identity/recognition record only \
                    (value 0, non-fabricated): no damage on a successful Reflex save against an \
                    effect that normally allows half damage on a successful save, no benefit on a \
                    failed save — naming the rule text with no saving-throw-resolution or \
                    damage-resolution engine behind it, (SD13-E5) Still Mind, a 3rd-level \
                    Monk class feature verified independently against the same two primary \
                    sources, grounded as a bounded flat-magnitude record (a flat +2 bonus on \
                    saving throws against enchantment spells and effects, value 0 as a correct \
                    level-gate absence below level 3), mirroring the Fighter Bravery / Paladin \
                    Divine Grace / Rogue Trap Sense idiom: never applied to any actual save \
                    total, since no saving-throw-resolution engine exists in this codebase, and \
                    (SD13-E5) the ki pool's flat size and Slow Fall, both 4th-level Monk class \
                    features verified independently against the same two primary sources (both \
                    list \"Ki pool (magic), slow fall 20 ft.\" as the Monk 4th-level special \
                    feature entry): the ki pool is grounded as a standalone flat-magnitude \
                    record (1/2 monk level + Wisdom modifier — no stated minimum in either \
                    primary source's rule text — mirroring the Barbarian rage rounds-per-day / \
                    Paladin lay-on-hands-uses-per-day idiom, with no ki-point consumption \
                    tracking, no action-economy engine, and no application of any ki power), and \
                    Slow Fall is grounded as a bounded grant-only identity record (mirroring the \
                    Barbarian/Rogue Uncanny Dodge idiom, with no fall-damage-resolution engine), \
                    and (SD13-E5) Purity of Body, a 5th-level Monk class feature verified \
                    independently against the same two primary sources (both list \"High jump, \
                    purity of body\" as the Monk 5th-level special feature entry): grounded as a \
                    bounded grant-only identity record (immunity to all diseases, including \
                    supernatural and magical diseases; no disease-resolution engine exists in \
                    this codebase). High Jump, the level-5 class table's OTHER \"Special\" column \
                    entry, was checked this cycle and confirmed NOT flat (it requires wiring the \
                    monk's level into an Acrobatics-check total and spending a ki point, neither \
                    of which this codebase implements), so it is deliberately left \
                    named-but-unproven, not fabricated. Fast Movement and Maneuver Training, the \
                    class table's other two 3rd-level \"Special\" column entries, are also \
                    deliberately left named-but-unproven (no speed-total engine and no CMB/CMD \
                    engine exist in this codebase to attach either to). A further SD13-E5 slice \
                    widens the gate to level 6 (verified independently against d20pfsrd and \
                    legacy.aonprd.com: the Monk class table's level-6 \"Special\" column reads \
                    \"Bonus feat, slow fall 30 ft.\"): the pre-existing base-attack, base-save, \
                    and Flurry of Blows formulas produce +4 BAB, +5/+5/+5 saves, and a +4/+4 \
                    flurry (attack count staying 2 — both primary sources' verbatim Flurry of \
                    Blows rule text confirms the third attack is not gained until 8th level, not \
                    6th), the ki pool's pre-existing formula genuinely rises to 6, and Slow \
                    Fall's own reach magnitude genuinely rises from 20 ft to 30 ft via the same \
                    grant-only identity record (value stays 0, non-fabricated; only the \
                    descriptive reach figure is level-accurate) — none of this is a new record, \
                    all of it is the same pre-existing formulas/records extended. The level-6 \
                    \"Special\" column's OTHER entry, \"Bonus feat,\" was checked and confirmed \
                    to be the same open-ended repeat bonus-feat choice-list shape already \
                    deliberately left named-but-unproven at 2nd level (mirroring the Rogue \
                    level-6 \"second Rogue Talent slot\" precedent), not a new automatic class \
                    feature; no new choice-slot and no new diagnostic was added for it. A further \
                    SD13-E5 slice widens the gate to level 7 (verified independently against \
                    d20pfsrd and legacy.aonprd.com: the Monk class table's level-7 \"Special\" \
                    column names Wholeness of Body): the pre-existing base-attack, base-save, and \
                    Flurry of Blows formulas produce +5 BAB, +5/+5/+5 saves, and a +5/+5 flurry \
                    (attack count staying 2 — both primary sources' verbatim Flurry of Blows rule \
                    text confirms the third attack is not gained until 8th level), and the \
                    unarmed strike die, Still Mind, the ki pool's flat size, Slow Fall, and \
                    Purity of Body all stay granted through level 7+ unchanged from level 6 (the \
                    ki pool and Slow Fall's reach values are integer-division/level-gate \
                    coincidences, not new records) — none of this is a new record, all of it is \
                    the same pre-existing formulas/records extended. Wholeness of Body, the \
                    level-7 \"Special\" column's new named feature (\"a monk can heal his own \
                    wounds as a standard action... a number of hit points of damage equal to his \
                    monk level by using 2 points from his ki pool\"), is checked and confirmed \
                    NOT flat: it requires both a ki-point-consumption/action-economy engine and a \
                    healing-resolution engine, neither of which exists in this codebase, so it is \
                    deliberately left named-but-unproven, not fabricated, mirroring the High Jump \
                    precedent. A further SD13-E5 slice widens the gate to level 8 (verified \
                    independently against d20pfsrd and legacy.aonprd.com: the Monk class table's \
                    level-8 \"Special\" column reads \"Slow fall 40 ft.\" only): the pre-existing \
                    base-attack and base-save formulas produce +6 BAB and +6/+6/+6 saves, the \
                    unarmed strike damage die genuinely rises from 1d8 to 1d10 (the 1d10 band \
                    runs levels 8-11), the Flurry of Blows flat attack modifier rises to +6/+6 \
                    and the attack count genuinely rises from 2 to 3 (verified independently \
                    against both primary sources' verbatim Flurry of Blows rule text, \"At 8th \
                    level, the monk can make two additional attacks when he uses flurry of \
                    blows, as if using Improved Two-Weapon Fighting\" — correcting a prior \
                    web-search error that wrongly suggested the third attack lands at level 6 or \
                    7), the ki pool's pre-existing formula genuinely rises to 7, and Slow Fall's \
                    own reach magnitude genuinely rises from 30 ft to 40 ft via the same \
                    grant-only identity record (value stays 0, non-fabricated; only the \
                    descriptive reach figure is level-accurate) — none of this is a new record, \
                    all of it is the same pre-existing formulas/records extended. The level-8 \
                    \"Special\" column was checked and specifically confirmed NOT to name \
                    Improved Uncanny Dodge (a commonly-repeated but WRONG assumption for Monk, \
                    carried over from other classes' 8th-level tables): neither primary source \
                    lists it anywhere on the Monk class table at any level, so no such record is \
                    grounded or fabricated; Wholeness of Body (granted at level 7) stays \
                    granted-but-unexecuted, unchanged — AND a further SD13-E5 slice widens the \
                    level-range gate again (supported_monk_level, 1..=9) and extends every one \
                    of the formulas above to level 9 via the same formula, without \
                    re-derivation, verified independently against the PF1 Core Rulebook Monk \
                    class table (d20pfsrd and legacy.aonprd.com): level 9 base attack stays +6 \
                    (9 * 3 / 4) and all three good saves stay +6 (9 / 2 + 2), integer-division \
                    coincidences; the unarmed die stays 1d10 (the band spans levels 8-11); the \
                    Flurry flat attack modifier genuinely rises to +7 (level - 2) while the \
                    attack count stays 3 (the next count change lands at 15th); the ki pool \
                    stays 7 and Slow Fall's reach stays 40 ft (the next reach increase lands \
                    at 10th); the level-9 \"Special\" column reads \"Improved evasion\" \
                    (verified independently against both primary sources, checked rather than \
                    assumed away) — a genuinely NEW named entry, an upgrade of the 2nd-level \
                    Evasion identity (no damage on a successful Reflex save, and henceforth \
                    only half damage on a failed one), grounded by this slice as a +0 \
                    identity/recognition record only \
                    (class_feature.monk.improved_evasion), mirroring exactly how Evasion itself \
                    and Rogue's Improved Uncanny Dodge were grounded — no \
                    saving-throw-resolution or damage-resolution engine exists in this \
                    codebase, so no damage math is fabricated from the record. One named burden remains unproven: the \
                    recognized bonus feat's own \
                    mechanics (no \
                    attack-resolution, grapple-check, trip-check, or DC/save engine exists for \
                    any of the restricted-list feats). The level-2 and level-6 bonus feat grants \
                    (PF1 grants monks SEPARATE bonus feats at 2nd and 6th level, neither \
                    recognized by this widening), Wholeness of Body's own execution, the \
                    level-10+ unarmed damage die progression, \
                    flurry with special monk weapons, wiring into integrated combat totals, any \
                    ki-power execution engine, High Jump's own Acrobatics/ki-point mechanics, and \
                    Monk level 10+ all remain unproven, and no martial math beyond the grounded \
                    flat surfaces is fabricated",
                next_required_uplift: "later SD13-E5/E6 slice grounding the one remaining named \
                    Monk martial pillar burden (the recognized bonus feat's own mechanics — an \
                    execution engine per feat, not a flat number), then the level-2/level-6 \
                    bonus feat grant recognition, Wholeness of Body's own execution, and Monk \
                    level 10+ progression",
            },
            SupportStateRow {
                row_id: "class.paladin.hybrid_chassis_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:paladin",
                dimension: "bounded hybrid class progression: the deterministic Human \
                            Paladin level-1/level-2/level-3/level-4/level-5/level-6/level-7/\
                            level-8 \
                            chassis baseline, with smite evil's uses-per-day / attack-bonus / \
                            damage-bonus formula grounded at every level (uses-per-day \
                            genuinely increasing to 2/day at level 4, staying 2/day through \
                            level 6, genuinely increasing to 3/day at level 7, and staying \
                            3/day at level 8), lay on \
                            hands and divine grace grounded for real at levels 2-8 (correct PF1 \
                            CRB level-gate absence at level 1, lay on hands genuinely \
                            increasing again at level 6, staying numerically unchanged at \
                            level 7, and genuinely increasing on both axes at level 8), mercy \
                            grounded as a correct PF1 CRB level-gate absence at \
                            levels 1-2 and a granted choice-recognition record at levels 3-8, \
                            channel positive energy grounded as a correct PF1 CRB level-gate \
                            absence at levels 1-3 and a flat die-count magnitude at levels 4-8 \
                            (genuinely increasing from 2d6 to 3d6 at level 5, staying 3d6 at \
                            level 6, genuinely increasing to 4d6 at level 7, and staying 4d6 \
                            at level 8), the \
                            partial-caster effective-caster-level gate grounded as a correct \
                            zero absence at levels 1-3, a genuine value of 1 at level 4, a \
                            genuine value of 2 at level 5, a genuine value of 3 at level 6, a \
                            genuine value of 4 at level 7, and a genuine value of 5 at level \
                            8, and the hybrid chassis pair plus \
                            the spells-known/spells-per-day/spell-DC spell burden still named \
                            and unproven",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_PALADIN_ROW_GROUNDING_REF,
                blocker_or_lossiness_note: "SD13-E3/E4/E5 leaves direct computed evidence that the \
                    deterministic Human Paladin \
                    level-1/level-2/level-3/level-4/level-5/level-6/level-7 hybrid chassis is \
                    recognized on the compute seam and that all four named non-spell \
                    class-feature burdens are grounded across those levels. The foundational \
                    base attack bonus / base save progression pillar is grounded for real at \
                    every supported level as standalone, not-integrated records: full base \
                    attack bonus (classlevel, the same shape as Fighter/Barbarian/Ranger) and \
                    good Fortitude / good Will / poor Reflex base saves (classlevel/2+2 for the \
                    two good saves, classlevel/3 for the poor save) -- NOT the same save shape \
                    as Ranger's good Fortitude/Reflex, poor Will, both verified independently \
                    against the PF1 Core Rulebook Paladin class table, including a fresh read \
                    of the level-7 row (BAB +7/+2, Fort +5, Ref +2, Will +5) confirming all \
                    three base saves stay numerically unchanged from level 6 (an \
                    integer-division coincidence, re-verified rather than assumed). The smite \
                    evil pillar is grounded for real: uses per day = 1 + (paladin level - 1) / \
                    3, attack-roll bonus = Charisma modifier (if positive), damage bonus = \
                    paladin level (PF1 Core Rulebook), computed against the deterministic \
                    fixtures as 1 / +2 / +1 at level 1, 1 / +2 / +2 at level 2, 1 / +2 / +3 at \
                    level 3, GENUINELY 2 / +2 / +4 at level 4 (the PF1 CRB level-4 \"Special\" \
                    column reads \"smite evil 2/day\", verified independently rather than \
                    assumed to stay at 1), unchanged at 2 / +2 / +5 at level 5 and 2 / +2 / +6 \
                    at level 6, and GENUINELY 3 / +2 / +7 at level 7 (the PF1 CRB level-7 \
                    \"Special\" column reads \"Smite evil 3/day\", verified independently \
                    against both d20pfsrd and legacy.aonprd.com rather than assumed to stay at \
                    2; the next increase does not land until level 10); this grounds only that \
                    flat numeric formula, not alignment/evil-subtype target resolution or \
                    evil-outsider/dragon/undead damage doubling. Lay on hands and divine grace \
                    are grounded for real at levels 2-7 (their PF1 CRB level gate): lay on hands \
                    uses per day = 1/2 paladin level + Charisma modifier, with the \
                    heal amount stated as a flat non-fabricated die-count magnitude (1d6 per two \
                    paladin levels, never a rolled value) -- both GENUINELY increase again at \
                    level 6 (uses/day to 5, heal dice to 3d6) and both stay numerically \
                    unchanged at level 7 (an integer-division coincidence, re-verified rather \
                    than assumed); divine grace grants a Charisma-modifier bonus, applied only \
                    if positive, on all saving throws, unchanged. Below that gate, at level 1, \
                    both remain correct level gate absences (value 0). Mercy stays a grounded \
                    level gate absence at levels 1-2: mercy is a 3rd-level paladin feature in \
                    the PF1 Core Rulebook, so it emits a value-0 record naming its at-grant \
                    formula without computing it below the gate. At levels 3-7 (SD13-E5), mercy \
                    is newly GRANTED as a bounded grant-only identity record (verified \
                    independently against legacy.aonprd.com's Core Rulebook Paladin page: \"a \
                    paladin can select one mercy. Each mercy adds an effect to the paladin's \
                    lay on hands ability\"; the first, 3rd-level tier of the mercy list is \
                    Fatigued, Shaken, and Sickened), plus a choice-recognition record naming \
                    whichever mercy was selected on the deterministic \
                    level-3/level-4/level-5/level-6/level-7 fixtures (mercy:shaken) -- \
                    mirroring the Ranger Favored Terrain / Sorcerer bloodline choice-slot idiom; \
                    the selected mercy's own effect (curing the named condition when lay on \
                    hands is used) is not computed, since no lay-on-hands execution engine \
                    exists in this codebase; the grant and choice stay unchanged (not \
                    re-derived) at levels 4-7, even though the PF1 CRB level-6 \"Special\" \
                    column reads \"Mercy\" again (an additional mercy becomes selectable at 6th \
                    level and every three levels thereafter) -- this was checked against a \
                    primary source and confirmed to require a mercy-list-growth mechanism this \
                    codebase has not already grounded (the existing mercy records are a single, \
                    ungated recognition, not a per-level slot count), so it stays deliberately \
                    named-but-unproven rather than fabricated, mirroring the Rogue second-talent \
                    / Barbarian Rage Power / Monk second-bonus-feat precedent; the level-7 \
                    \"Special\" column was independently checked too and reads \"Smite evil \
                    3/day\" only -- level 7 is not one of the repeat-Mercy-grant levels \
                    (3, 6, 9, ...), so nothing new is left unproven for Mercy at level 7. SD13-E5 \
                    additionally grounds the partial-caster IDENTITY itself as one more flat \
                    level-gate record: effective caster level = max(paladin level - 3, 0), which \
                    correctly grounds to 0 at levels 1-3, GENUINELY becomes 1 at level 4 (PF1 \
                    Core Rulebook: paladin spells begin at paladin level 4), GENUINELY becomes 2 \
                    at level 5, GENUINELY becomes 3 at level 6, and GENUINELY becomes 4 at level \
                    7 -- all real value changes, not re-derivations; only the caster-level gate \
                    arithmetic itself is grounded, no spell slots are fabricated. Channel \
                    Positive Energy, the PF1 CRB's OTHER 4th-level paladin class feature \
                    (verified independently against legacy.aonprd.com's Core Rulebook Paladin \
                    page: \"When a paladin reaches 4th level, she gains the supernatural \
                    ability to channel positive energy like a cleric. Using this ability \
                    consumes two uses of her lay on hands ability. A paladin uses her level as \
                    her effective cleric level when channeling positive energy.\"), is a \
                    grounded level-gate absence at levels 1-3 and a flat die-count magnitude \
                    (ceil(paladin level / 2), mirroring Cleric's own Channel Energy dice-count \
                    formula exactly) at levels 4-7: 2d6 at level 4, GENUINELY 3d6 at level 5, \
                    staying 3d6 at level 6 (an integer-division coincidence, re-verified rather \
                    than assumed), and GENUINELY 4d6 at level 7. This grounds only the flat \
                    die-count magnitude and the lay-on-hands-use-cost identity; no \
                    healing/damage-resolution execution, no heal-vs-harm target selection, and \
                    no lay-on-hands-resource-consumption bookkeeping is computed. Divine Bond, \
                    the PF1 CRB's OTHER 5th-level paladin class feature (verified independently \
                    against legacy.aonprd.com's Core Rulebook Paladin page), was checked per \
                    the operator brief's explicit \"verify what the Special column shows\" \
                    instruction and confirmed NOT flat: it requires an \
                    activation/resource-consumption engine (a limited number of uses per day, \
                    for a duration of \"1 minute per paladin level\") plus either an ongoing \
                    weapon-enhancement subsystem or a full mount stat-block/advancement \
                    subsystem (mirroring the still-unproven Ranger Hunter's Bond \"companion\" \
                    form / Druid animal companion), so it is deliberately left \
                    named-but-unproven, mirroring the Monk High Jump / Wizard level-5 bonus \
                    feat precedent exactly -- no explanation or diagnostic record is fabricated \
                    for it, unaffected by the level-7 widening -- AND a further SD13-E5 slice \
                    widens the level-range gate again (supported_paladin_level, 1..=8) and \
                    extends every one of the formulas above to level 8 via the same formula, \
                    without re-derivation, verified independently against the PF1 Core \
                    Rulebook Paladin class table (d20pfsrd and legacy.aonprd.com): level 8 \
                    base attack bonus is +8 (genuinely risen; the table's own \"+8/+3\" \
                    iterative notation is not modeled anywhere in this codebase, only the flat \
                    base value) and base saves are +6/+2/+6 (Fortitude/Reflex/Will -- both \
                    good saves genuinely rise from +5 while poor Reflex stays +2, an \
                    integer-division coincidence); Smite Evil stays 3/day (the next rise \
                    lands at 10th, a threshold stasis checked rather than assumed) with its \
                    damage bonus genuinely rising to 8 (= paladin level); Lay on Hands \
                    genuinely rises on both axes (uses 6, heal dice 4); the effective caster \
                    level genuinely rises to 5 (8 - 3); Channel Positive Energy's die count \
                    stays 4 (the effective-cleric dice rise at odd levels, so the next rise \
                    lands at 9th); level 8 is not a repeat-Mercy-grant level (3/6/9/...), so \
                    the single granted mercy recognition carries over unchanged; UNLIKE the \
                    level-7 \"Smite evil 3/day\" column, the class table's level-8 \
                    \"Special\" column reads \"Aura of resolve\" (verified independently \
                    against both primary sources, checked rather than assumed away) -- a \
                    genuinely NEW class feature, and confirmed NOT flat/identity-shaped: \
                    immunity to charm spells and spell-like abilities plus a +4 morale bonus \
                    against charm effects for allies within 10 feet while the paladin is \
                    conscious needs a condition-immunity engine and an ally-aura/positional \
                    engine, neither of which exists in this codebase, so Aura of Resolve is \
                    deliberately left named-but-unproven, exactly like Aura of Courage and \
                    Divine Health before it, with a dedicated negative test pinning that no \
                    aura record or diagnostic is fabricated. The row is Partial, not \
                    Supported: the F6 hybrid chassis pair (class-feature and spell) stays \
                    claim-blocking as accepted hybrid truth, no Paladin level 9+ is proven, \
                    Divine Bond stays named-but-unproven, the level-6 repeat mercy selection \
                    stays named-but-unproven, and the partial-caster spell burden itself \
                    remains named and unproven beyond the grounded caster-level gate arithmetic \
                    — no spell-source lineage, spells known or prepared posture, \
                    spells-per-day progression, bonus spell slots, or spell save DCs are \
                    grounded. The F6 hybrid baseline, the F6 hybrid blockers, and the F6 hybrid \
                    chassis recognition explanation all remain in place (each gated to the \
                    bounded hybrid baseline level, so they still fire only at level 1)",
                next_required_uplift: "ground the paladin spells-known/spells-per-day/spell-DC \
                    burden content now that the effective-caster-level gate is grounded and \
                    nonzero (spells begin at paladin level 4, caster level = paladin level - 3, \
                    now 5 at level 8), then paladin level-9+ progression",
            },
            SupportStateRow {
                row_id: "class.ranger.hybrid_chassis_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:ranger",
                dimension: "bounded hybrid class progression: the deterministic Human \
                            Ranger level-1/level-2/level-3/level-4/level-5/level-6/level-7/ \
                            level-8/level-9 chassis baseline, with base attack bonus, base save \
                            progression, Track, the favored-enemy flat surface, the \
                            combat-style choice-and-bonus-feat recognition, (level 3) \
                            Endurance and the Favored Terrain choice-and-flat-magnitude \
                            surface, (level 4) the Hunter's Bond choice-and-flat-magnitude \
                            surface, (level 5) the Favored Enemy rule's 5th-level interval \
                            (second favored-enemy selection plus the bonus-increase target \
                            choice), (level 6) the SECOND combat-style bonus feat choice \
                            recognition, (level 7) Woodland Stride (a grant-only identity \
                            record), and (level 8) Swift Tracker (a grant-only identity \
                            record) grounded for real and the later spell burden still \
                            blocked",
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
                    favored enemy (PF1 includes attack rolls, unlike D&D 3.5). A later SD13-E5 \
                    slice grounds the combat style pillar as a correct level-1 ABSENCE (value 0): \
                    PF1 Core Rulebook grants the archery-vs-two-weapon-combat style choice and its \
                    first bonus feat TOGETHER at 2nd level, not split across a level-1 choice and a \
                    level-2 grant as an earlier version of this row's note incorrectly claimed; \
                    there was never a level-1 style choice to ground. A still later SD13-E5 slice \
                    grounds the foundational base attack bonus and base save progression pillar \
                    for the first time (full BAB = classlevel, good Fortitude/good Reflex/poor \
                    Will = classlevel/2+2 and classlevel/3, verified against the PF1 Core \
                    Rulebook Ranger class table, cross-checking level 4/5 BAB to disambiguate \
                    full BAB from 3/4 BAB), grounded as flat standalone explanation records not \
                    wired into the integrated base_attack_bonus field, compute_total_saves, or \
                    compute_combat_baseline. The most recent SD13-E5 slice widens the level-1-only \
                    gate to a level-range gate (1..=2), extending base attack/base save/Track/the \
                    favored-enemy flat surface to level 2 via the same formulas (Track stays 1, \
                    max(2/2, 1); the favored-enemy flat bonus stays +2, PF1 CRB only increases it \
                    at 4th ranger level and beyond), and FINALLY grounds the combat style pillar \
                    for real at the 2nd-level gate it was always named for: recognition of the \
                    chosen combat style (Archery or Two-Weapon Combat, verified against \
                    legacy.aonprd.com's Core Rulebook Ranger page) from \
                    choice:ranger_combat_style, and recognition of the specific bonus feat chosen \
                    from that style's own restricted 2nd-level feat list (Archery: Far Shot, \
                    Point-Blank Shot, Precise Shot, Rapid Shot; Two-Weapon Combat: Double Slice, \
                    Improved Shield Bash, Quick Draw, Two-Weapon Fighting) from \
                    choice:ranger_combat_style_bonus_feat, both bounded identity/recognition \
                    records only (+0 each) — the chosen feat's own mechanical effect (e.g. \
                    Point-Blank Shot's attack/damage bonus within 30 ft.) is not computed. The \
                    most recent SD13-E5 slice widens the level-range gate again to level 3, \
                    extending base attack/base save/Track/the favored-enemy flat surface to level \
                    3 via the same formulas (Track stays 1, max(3/2, 1); the favored-enemy flat \
                    bonus stays +2, PF1 CRB only increases it at 4th ranger level and beyond), and \
                    grounds Endurance, the PF1 CRB's 3rd-level Ranger class feature (verified \
                    independently against d20pfsrd and legacy.aonprd.com), as a bounded grant-only \
                    identity record (value 0, non-fabricated): the ranger gains Endurance as a \
                    bonus feat automatically, with no player choice involved, mirroring the Wizard \
                    Scribe Scroll / Barbarian Uncanny Dodge idiom. A still later SD13-E5 slice \
                    grounds Favored Terrain, the class table's other 3rd-level \"Special\" column \
                    entry, for real: a new choice:ranger_favored_terrain choice-slot recognizes \
                    whichever terrain type was selected as a bounded +0 identity record \
                    (mirroring the Favored Enemy choice-recognition idiom exactly — raw string \
                    interpolation, no restricted-list validation), and the rule's own flat +2 \
                    magnitude on Initiative checks and Knowledge (geography), Perception, Stealth, \
                    and Survival checks made in the chosen terrain (verified independently against \
                    d20pfsrd and legacy.aonprd.com's Table: Ranger Favored Terrains) is grounded \
                    as a standalone, non-applied record — no terrain-detection engine decides \
                    whether the character is actually in the chosen terrain, so no Initiative \
                    total or skill-check total is modified by this record. The level-8th/13th/18th \
                    additional-terrain and bonus-increase progression stays out of scope this \
                    slice. The most recent SD13-E5 slice widens the level-range gate once more to \
                    level 4, extending base attack/base save/Track/the favored-enemy flat surface \
                    to level 4 via the same formulas (Track becomes 2, max(4/2, 1); the \
                    favored-enemy flat bonus stays +2, PF1 CRB only increases it at 5th ranger \
                    level and beyond, verified independently against d20pfsrd and \
                    legacy.aonprd.com), keeps Endurance and Favored Terrain granted unchanged, and \
                    grounds Hunter's Bond, the class table's 4th-level \"Special\" column entry \
                    (verified independently against both primary sources): a restricted \
                    two-option choice recognition (choice:ranger_hunters_bond -> form:bond or \
                    form:companion, mirroring the combat-style choice idiom) as a bounded +0 \
                    record, an unconditional grant-only identity record (mirroring the \
                    Endurance/Favored Terrain idiom), and — only for the \"bond\" form — a flat, \
                    non-applied magnitude equal to half the already-grounded favored-enemy bonus, \
                    grantable via a move action to allies within 30 feet who can see or hear the \
                    ranger against a single target of the appropriate type. No \
                    move-action/action-economy engine, no ally-range-and-perception check, and no \
                    favored-enemy target-type matching is implemented; the \"companion\" form's \
                    own animal-companion stat block/advancement subsystem is deliberately left \
                    named-but-unproven, since it does not exist anywhere in this codebase. The \
                    most recent SD13-E5 slice widens the level-range gate once more to level 5, \
                    extending base attack/base save/Track to level 5 via the same formulas (Track \
                    stays 2, max(5/2, 1); base Fortitude/Reflex/Will all stay 4/4/1, integer-\
                    division coincidences, not signs the formulas stopped scaling), keeps \
                    Endurance, Favored Terrain, combat style, and Hunter's Bond granted unchanged, \
                    and grounds the Favored Enemy rule's own 5th-level interval, the class \
                    table's 5th-level \"Special\" column entry (\"2nd favored enemy\", verified \
                    independently against both primary sources — no other new class feature is \
                    gained at 5th level). The rule text (\"At 5th level and every five levels \
                    thereafter... the ranger may select an additional favored enemy. In addition, \
                    at each such interval, the bonus against any one favored enemy (including the \
                    one just selected, if so desired) increases by 2\") is NOT an automatic bump \
                    to the first favored enemy — it is the ranger's own free choice which ONE \
                    favored enemy is boosted. This slice grounds three things: a second \
                    favored-enemy TYPE selection (choice:ranger_favored_enemy_2, mirroring the \
                    first favored enemy's own open-ended choice-recognition idiom, plus the same \
                    flat +2 base magnitude formula), a restricted two-option choice recognizing \
                    WHICH one favored enemy is the bonus-increase target \
                    (choice:ranger_favored_enemy_bonus_increase_target -> enemy:first or \
                    enemy:second, mirroring the Hunter's Bond/combat-style restricted two-option \
                    idiom), and the resulting +4 magnitude applied only to whichever favored \
                    enemy the target choice actually names — absent an explicit target \
                    selection, both favored enemies correctly stay the flat +2, since nothing is \
                    fabricated about which one the ranger picked. Hunter's Bond's own ally-bonus \
                    magnitude (half the FIRST favored enemy's bonus) naturally recomputes from \
                    the same unchanged formula once that magnitude widens to +4. The most recent \
                    SD13-E5 slice widens the level-range gate once more to level 6, extending \
                    base attack/base save/Track to level 6 via the same formulas (Track \
                    genuinely rises to 3, max(6/2, 1); base Fortitude/Reflex rise to 5 \
                    (6/2+2) and base Will rises to 2 (6/3), all genuinely new values, verified \
                    independently against d20pfsrd and legacy.aonprd.com), keeps Endurance, \
                    Favored Terrain, both favored enemies, and Hunter's Bond granted unchanged, \
                    and grounds the class table's 6th-level \"Special\" column entry (\"Combat \
                    style feat\", verified independently against both primary sources — no other \
                    new class feature is gained at 6th level). The Combat Style Feat rule's own \
                    cadence (\"bonus feats at 2nd, 6th, 10th, 14th, and 18th level\", verified \
                    independently against both primary sources) confirms the ranger's SECOND \
                    combat-style bonus feat lands here, not at 3rd/4th/5th level as an earlier \
                    cycle's check already anticipated. This slice grounds the SECOND combat-style \
                    bonus feat as a restricted-list choice recognition (choice:\
                    ranger_combat_style_bonus_feat_2), gated on the same style already recognized \
                    at 2nd level, validated against each style's own 6th-level feat list only \
                    (Archery: Improved Precise Shot, Manyshot; Two-Weapon Combat: Improved \
                    Two-Weapon Fighting, Two-Weapon Defense) — mirroring the first bonus feat's \
                    own grounding idiom exactly (+0, no mechanical effect computed, no execution \
                    engine exists in this codebase). The most recent SD13-E5 slice widens the \
                    level-range gate once more to level 7, extending base attack/base save/Track \
                    to level 7 via the same formulas (all three stay numerically unchanged from \
                    level 6 — 7/2+2 = 5, 7/3 = 2, max(7/2, 1) = 3 — integer-division \
                    coincidences, re-verified rather than assumed), keeps Endurance, Favored \
                    Terrain, both favored enemies, both combat-style bonus feats, and Hunter's \
                    Bond granted unchanged (neither the Favored Enemy rule's next interval nor \
                    the Combat Style Feat's next bonus feat arrives before 10th level, re-verified \
                    independently against both primary sources), and grounds the class table's \
                    7th-level \"Special\" column entry (\"Woodland stride\", verified \
                    independently against both primary sources — no other new class feature is \
                    gained at 7th level). Woodland Stride (\"a ranger may move through any sort \
                    of undergrowth ... at his normal speed and without taking damage or suffering \
                    any other impairment ... magically manipulated undergrowth ... still affects \
                    him normally\") carries no numeric magnitude of its own, unlike Track or \
                    Favored Terrain — it is grounded as a bounded grant-only identity record \
                    (value 0, non-fabricated), mirroring the Endurance idiom exactly: no \
                    terrain-detection or movement-resolution engine exists anywhere in this \
                    codebase to determine whether the ranger is actually moving through \
                    undergrowth, so only the grant itself is recorded. The most recent SD13-E5 \
                    slice widens the level-range gate once more to level 8, extending base \
                    attack/base save/Track to level 8 via the same formulas (base Fortitude and \
                    Reflex genuinely rise to 6, 8/2+2, up from 5 at level 7; base Will stays 2, \
                    8/3, an integer-division coincidence unchanged from level 7; Track genuinely \
                    rises to 4, max(8/2, 1), up from 3 at level 7 — all verified independently \
                    against d20pfsrd and legacy.aonprd.com), keeps Endurance, Favored Terrain, \
                    both favored enemies, both combat-style bonus feats, Hunter's Bond, and \
                    Woodland Stride granted unchanged (neither the Favored Enemy rule's next \
                    interval nor the Combat Style Feat's next bonus feat arrives before 10th \
                    level, re-verified independently against both primary sources), and grounds \
                    Swift Tracker, one of the class table's TWO 8th-level \"Special\" column \
                    entries (\"Swift tracker, 2nd favored terrain\", verified independently \
                    against both primary sources). Swift Tracker (\"a ranger can move at his \
                    normal speed while using Survival to follow tracks without taking the normal \
                    -5 penalty. He takes only a -10 penalty (instead of the normal -20) when \
                    moving at up to twice normal speed while tracking\") carries no numeric \
                    magnitude of its own and only modifies a tracking-while-moving penalty \
                    resolution that does not exist anywhere in this codebase — it is grounded as \
                    a bounded grant-only identity record (value 0, non-fabricated), mirroring the \
                    Woodland Stride idiom exactly. The level-8 row's OTHER named entry, \"2nd \
                    favored terrain\" (mirroring the already-grounded Favored Enemy 5th-level \
                    idiom: a second terrain-type selection plus a bonus-increase-target choice), \
                    is a real, newly discovered multi-record burden deliberately left \
                    named-but-unproven this slice, not an invented one — AND a further SD13-E5 \
                    slice widens the level-range gate again (supported_ranger_level, 1..=9) and \
                    extends every one of the formulas above to level 9 via the same formula, \
                    without re-derivation, verified independently against the PF1 Core Rulebook \
                    Ranger class table (d20pfsrd and legacy.aonprd.com): level 9 base attack \
                    genuinely rises to +9 (full BAB; the table's own \"+9/+4\" iterative \
                    notation is not modeled anywhere in this codebase, only the flat base \
                    value) and poor Will genuinely rises to +3 (9 / 3), while both good saves \
                    stay +6 (9 / 2 + 2, integer-division coincidences); Track stays 4 \
                    (max(9/2, 1), a coincidence); the favored-enemy skill bonuses, \
                    favored-terrain count, Hunter's Bond ally bonus, Endurance, Woodland \
                    Stride, Swift Tracker, and both combat-style bonus-feat recognitions all \
                    carry over unchanged (the next favored-enemy grant lands at 10th and the \
                    next favored-terrain grant at 13th, both checked rather than assumed); the \
                    level-9 \"Special\" column reads \"Evasion\" (verified independently \
                    against both primary sources, checked rather than assumed away) — a \
                    genuinely NEW class feature carrying the same rule text as Rogue's and \
                    Monk's own Evasion, grounded by this slice as a +0 identity/recognition \
                    record only (class_feature.ranger.evasion), mirroring those precedents \
                    exactly — no saving-throw-resolution or damage-resolution engine exists in \
                    this codebase, so no damage math is fabricated from the record. The row is Partial, not \
                    Supported: the favored-enemy conditional-application engine (target-type \
                    matching that would decide whether a specific check or attack is made \
                    against the favored enemy) is not implemented, neither recognized \
                    combat-style bonus feat's own mechanics are proven, the 2nd favored-terrain \
                    selection and the Favored Terrain level-13th/18th breadth beyond it are \
                    unproven, Hunter's Bond's ally-bonus application and the animal-companion \
                    form are unproven, Woodland Stride's own terrain-movement application and \
                    Swift Tracker's own tracking-penalty application are both unproven, Ranger \
                    level 10+ is not proven, and the later ranger spell burden (spell slots, spell \
                    source, spells known/prepared) is still deferred to SD13-E4",
                next_required_uplift: "the PF1 CRB level-8 2nd favored-terrain selection \
                    (mirroring the already-grounded Favored Enemy 5th-level idiom), Ranger \
                    level-10+ progression, a favored-enemy conditional-application engine, \
                    execution of either recognized combat-style bonus feat's own mechanics, \
                    Hunter's Bond ally-bonus application and the animal-companion stat \
                    block/advancement subsystem, a terrain-detection/movement-resolution engine \
                    for Woodland Stride's own effect, a tracking-penalty-application engine for \
                    Swift Tracker's own effect, then SD13-E4 ranger spell burden",
            },
            SupportStateRow {
                row_id: "class.sorcerer.progression_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:sorcerer",
                dimension: "bounded spell-bearing class progression: the deterministic Human \
                            Sorcerer level-1/level-2/level-3/level-4/level-5/level-6/level-7/\
                            level-8/level-9 \
                            spell baseline, with base attack bonus, base save progression, Eschew \
                            Materials, the canonical bloodline choice recognition, and the \
                            Arcane bloodline's class-skill choice (a player's choice of any one \
                            Knowledge skill) grounded for real and the Arcane Bond / bloodline \
                            progression burden and the spontaneous known-spell / slot posture \
                            burden still blocked",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_SORCERER_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E4-F7 leaves direct computed evidence that the \
                    deterministic Human Sorcerer level-1 spontaneous arcane spell-bearing identity is \
                    recognized on the compute seam, the SD13-E4 Sorcerer decomposition slice \
                    grounds Eschew Materials (the universal, bloodline-independent bonus feat every \
                    1st-level Sorcerer receives: casting a spell with a material component costing \
                    1 gp or less without needing that material component) for real, the SD13-E5 \
                    bloodline-choice slice recognizes the canonical deterministic bloodline \
                    selection (choice:sorcerer_bloodline -> bloodline:arcane) as chosen input — \
                    recognition only, since the Arcane bloodline's level-1 power is Arcane Bond \
                    (a familiar or a bonded object), an execution engine rather than a flat \
                    number, so no power value is fabricated — AND a further SD13-E5 slice \
                    grounds the Arcane bloodline's class-skill grant for real: the PF1 Core \
                    Rulebook text reads \"Class Skill: Knowledge (any one)\" (verified against \
                    d20pfsrd and the legacy Paizo PRD mirror, correcting an earlier imprecise \
                    framing of this grant as a fixed Knowledge [arcana] award), a player's choice \
                    of any one Knowledge skill, recognized here as a +0 recognition record only \
                    (granting a class skill confers no flat modifier by itself in this codebase), \
                    AND a further SD13-E5 slice grounds the foundational base-attack-bonus / \
                    base-save progression pillar that every other class row in this matrix \
                    (Fighter, Barbarian, Monk, Rogue, Paladin, Druid, Cleric, Bard) already had \
                    and Sorcerer never had: base attack bonus (1/2 BAB, classlevel / 2 — UNLIKE \
                    the 3/4 BAB shared by Rogue/Monk/Druid/Cleric/Bard) and base save \
                    progression (good Will only, poor Fortitude, poor Reflex), both verified \
                    against the PF1 Core Rulebook Sorcerer class table (d20pfsrd and the legacy \
                    Paizo PRD mirror), reading the raw level 1-6 table rows directly to \
                    disambiguate the 1/2-vs-3/4 fraction since level 1 alone floors every \
                    fraction to +0, both grounded as standalone explanation records not wired \
                    into compute_total_saves or compute_combat_baseline — AND a further SD13-E5 \
                    slice widens the level-1-only gate (supported_sorcerer_level, 1..=2) and \
                    extends every one of the formulas above to level 2 via the same formula, \
                    without re-derivation, verified independently against the PF1 Core Rulebook \
                    Sorcerer class table (d20pfsrd and legacy.aonprd.com): level 2 base attack \
                    bonus is +1, base saves are +0/+0/+3 (Fortitude/Reflex/Will); the bloodline \
                    choice and bloodline class-skill choice recognitions are not level-gated (a \
                    sorcerer's bloodline does not change by level), so both still fire at level 2 \
                    for the same fixture selections; the Sorcerer class table's level-2 \"Special\" \
                    column is blank (verified independently against both primary sources), so \
                    Sorcerer gains no new class feature at 2nd level (unlike Rogue/Monk/Druid's \
                    Evasion/Woodland Stride, but like Cleric), so no new pillar burden is added \
                    that slice — only the existing pillars are widened — AND a further SD13-E5 \
                    slice widens the level-range gate again (supported_sorcerer_level, 1..=3) and \
                    extends every one of the formulas above to level 3 via the same formula, \
                    without re-derivation, verified independently against the PF1 Core Rulebook \
                    Sorcerer class table (d20pfsrd and legacy.aonprd.com): level 3 base attack \
                    bonus is +1, base saves are +1/+1/+3 (Fortitude/Reflex/Will); the bloodline \
                    choice and bloodline class-skill choice recognitions are not level-gated, so \
                    both still fire at level 3 for the same fixture selections; UNLIKE the blank \
                    level-2 column, the Sorcerer class table's level-3 \"Special\" column reads \
                    \"Bloodline power, bloodline spell\" (verified independently against both \
                    primary sources, checked rather than assumed away) — but both named entries \
                    are bloodline-specific (varying per bloodline, e.g. the Arcane bloodline's own \
                    3rd-level power is Metamagic Adept and its 3rd-level bloodline spell is \
                    Identify) and neither is flat/identity-shaped the way Rogue's Trap Sense or \
                    Monk's Still Mind are, so this slice grounds no new pillar for level 3 either \
                    — both entries stay named by the existing Arcane Bond / bloodline progression \
                    blocker's \"bonus spells/feats at 3rd+ level\" and \"bloodline power\" \
                    language, unchanged — AND a further SD13-E5 slice widens the level-range gate \
                    again (supported_sorcerer_level, 1..=4) and extends every one of the formulas \
                    above to level 4 via the same formula, without re-derivation, verified \
                    independently against the PF1 Core Rulebook Sorcerer class table (d20pfsrd and \
                    legacy.aonprd.com): level 4 base attack bonus is +2, base saves are +1/+1/+4 \
                    (Fortitude/Reflex/Will); the bloodline choice and bloodline class-skill choice \
                    recognitions are not level-gated, so both still fire at level 4 for the same \
                    fixture selections; UNLIKE the level-3 \"Bloodline power, bloodline spell\" \
                    entry, the Sorcerer class table's level-4 \"Special\" column is blank (verified \
                    independently against both primary sources, checked rather than assumed), so \
                    this slice grounds no new pillar for level 4 either — only the existing \
                    pillars are widened — AND a further SD13-E5 slice widens the level-range gate \
                    again (supported_sorcerer_level, 1..=5) and extends every one of the formulas \
                    above to level 5 via the same formula, without re-derivation, verified \
                    independently against the PF1 Core Rulebook Sorcerer class table (d20pfsrd and \
                    legacy.aonprd.com): level 5 base attack bonus is +2, base saves are +1/+1/+4 \
                    (Fortitude/Reflex/Will) — every one of these four values is numerically \
                    unchanged from level 4, an integer-division coincidence, not a sign any \
                    formula stopped scaling; the bloodline choice and bloodline class-skill choice \
                    recognitions are not level-gated, so both still fire at level 5 for the same \
                    fixture selections; UNLIKE the blank level-4 \"Special\" column, the Sorcerer \
                    class table's level-5 \"Special\" column reads \"Bloodline spell\" (verified \
                    independently against both primary sources, checked rather than assumed away) \
                    — the sorcerer's second bloodline spell grant (the Arcane bloodline's own \
                    5th-level bloodline spell is invisibility), but the entry is bloodline-specific \
                    and not flat/identity-shaped, so this slice grounds no new pillar for level 5 \
                    either, mirroring exactly how the level-3 \"Bloodline power, bloodline spell\" \
                    entry was left unproven — only the existing pillars are widened — AND a \
                    further SD13-E5 slice widens the level-range gate again \
                    (supported_sorcerer_level, 1..=6) and extends every one of the formulas above \
                    to level 6 via the same formula, without re-derivation, verified independently \
                    against the PF1 Core Rulebook Sorcerer class table (d20pfsrd and \
                    legacy.aonprd.com): level 6 base attack bonus is +3, base saves are +2/+2/+5 \
                    (Fortitude/Reflex/Will) — every one of these four values is a genuinely NEW \
                    value, up from +2/+1/+1/+4 at level 5; the bloodline choice and bloodline \
                    class-skill choice recognitions are not level-gated, so both still fire at \
                    level 6 for the same fixture selections; UNLIKE the level-5 \"Bloodline \
                    spell\" entry, the Sorcerer class table's level-6 \"Special\" column is \
                    genuinely blank (verified independently against both primary sources, checked \
                    rather than assumed away), so this slice grounds no new pillar for level 6 \
                    either — only the existing pillars are widened — AND a further SD13-E5 slice \
                    widens the level-range gate again (supported_sorcerer_level, 1..=7) and \
                    extends every one of the formulas above to level 7 via the same formula, \
                    without re-derivation, verified independently against the PF1 Core Rulebook \
                    Sorcerer class table (d20pfsrd and legacy.aonprd.com): level 7 base attack \
                    bonus is +3, base saves are +2/+2/+5 (Fortitude/Reflex/Will) — every one of \
                    these four values is numerically unchanged from level 6, an integer-division \
                    coincidence, not a sign any formula stopped scaling; the bloodline choice and \
                    bloodline class-skill choice recognitions are not level-gated, so both still \
                    fire at level 7 for the same fixture selections; UNLIKE the blank level-6 \
                    \"Special\" column, the Sorcerer class table's level-7 \"Special\" column \
                    reads \"Bloodline feat, bloodline spell\" (verified independently against both \
                    primary sources, checked rather than assumed away) — a bloodline feat (chosen \
                    from a list specific to each bloodline, first granted at 7th level and every \
                    six levels thereafter) and the sorcerer's third bloodline spell grant (the \
                    Arcane bloodline's own 7th-level bloodline spell is dispel magic), but both \
                    entries are bloodline-specific and not flat/identity-shaped, so this slice \
                    grounds no new pillar for level 7 either, mirroring exactly how the level-3 \
                    and level-5 bloodline power/spell entries were left unproven — only the \
                    existing pillars are widened — AND a further SD13-E5 slice widens the \
                    level-range gate again (supported_sorcerer_level, 1..=8) and extends every \
                    one of the formulas above to level 8 via the same formula, without \
                    re-derivation, verified independently against the PF1 Core Rulebook Sorcerer \
                    class table (d20pfsrd and legacy.aonprd.com): level 8 base attack bonus is \
                    +4 (genuinely risen from +3 at level 7) and base saves are +2/+2/+6 \
                    (Fortitude/Reflex/Will — good Will genuinely rises from +5, while poor \
                    Fortitude and poor Reflex both stay +2, integer-division coincidences, not a \
                    sign either formula stopped scaling); the bloodline choice and bloodline \
                    class-skill choice recognitions are not level-gated, so both still fire at \
                    level 8 for the same fixture selections; UNLIKE the level-7 \"Bloodline \
                    feat, bloodline spell\" entry, the Sorcerer class table's level-8 \
                    \"Special\" column is blank (verified independently against both primary \
                    sources, checked rather than assumed away) — like levels 2, 4, and 6, no \
                    new class feature is gained at 8th level; the first 4th-level spell slots \
                    arrive at 8th per the class table, but spells per day belong to the \
                    spontaneous spell burden that stays named-but-unproven, so this slice \
                    grounds no new pillar for level 8 either — only the existing pillars are \
                    widened — AND a further SD13-E5 slice widens the level-range gate again \
                    (supported_sorcerer_level, 1..=9) and extends every one of the formulas \
                    above to level 9 via the same formula, without re-derivation, verified \
                    independently against the PF1 Core Rulebook Sorcerer class table (d20pfsrd \
                    and legacy.aonprd.com): level 9 base attack bonus stays +4 (9 / 2) and good \
                    Will stays +6 (9 / 2 + 2), integer-division coincidences, while poor \
                    Fortitude and poor Reflex both genuinely rise to +3 (9 / 3); the bloodline \
                    choice and bloodline class-skill choice recognitions are not level-gated, \
                    so both still fire at level 9 for the same fixture selections; UNLIKE the \
                    blank level-8 column, the Sorcerer class table's level-9 \"Special\" \
                    column reads \"Bloodline power, bloodline spell\" (verified independently \
                    against both primary sources, checked rather than assumed away) — the \
                    second bloodline power (the Arcane bloodline's own 9th-level power is New \
                    Arcana) and the fourth bloodline spell (the Arcane bloodline's own \
                    9th-level bloodline spell is overland flight) — but both entries are \
                    bloodline-specific and not flat/identity-shaped, so this slice grounds no \
                    new pillar for level 9 either, mirroring exactly how the level-3, level-5, \
                    and level-7 bloodline entries were left unproven — only the existing \
                    pillars are widened. The row is \
                    Partial, not Supported: the Arcane Bond / bloodline progression burden (Arcane \
                    Bond execution, the conditional bloodline arcana, the 3rd-level bloodline \
                    power, the 3rd-, 5th-, and 7th-level bloodline spells, the 7th-level bloodline \
                    feat, and further bonus spells/feats at higher levels) remains named and \
                    unproven, and the spontaneous spell burden (spontaneous spells known, spell \
                    slots per day, bonus spell slots, spell save DCs) is entirely unproven. No \
                    spell math is fabricated and no Sorcerer level 10+ is proven",
                next_required_uplift: "SD13 Sorcerer Arcane Bond grounding slice (the chosen \
                    bloodline's level-1 power execution), then the spontaneous spell burden, then \
                    level-10+ progression (widening the now-grounded base attack/base save \
                    formulas)",
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
                // Track promotion). A SD13-E5 Wizard specialization slice then
                // grounds the flat surface of the school specialization choice
                // (canonical Evocation specialist, Necromancy and Transmutation
                // opposed) plus the specialist bonus slot count. A further SD13-E5
                // slice independently verifies (legacy Paizo PRD mirror) and then
                // grounds two of the Evocation school's own 1st-level school
                // powers as flat numeric magnitudes: Intense Spells' bonus-damage
                // magnitude and Force Missile's uses-per-day pool, narrowing the
                // class-feature blocker to the school-power execution machinery
                // and the opposed-school preparation cost; the prepared spell
                // posture burden remains entirely unproven. A further SD13-E5
                // slice grounds the foundational base-attack-bonus / base-save
                // progression pillar (1/2 BAB, the same shape as Sorcerer; good
                // Will only, poor Fortitude, poor Reflex), independently verified
                // against the PF1 Core Rulebook Wizard class table. A further
                // SD13-E5 slice widens the level-1-only gate (`supported_wizard_level`,
                // 1..=2) and extends every one of the formulas above to level 2 via
                // the same formula, without re-derivation, verified independently
                // against the PF1 Core Rulebook Wizard class table (d20pfsrd and
                // legacy.aonprd.com): Wizard gains no new class feature at 2nd level
                // (the class table's level-2 "Special" column is blank), so no new
                // pillar is added, only the existing ones widened. A further SD13-E5
                // slice widens the gate again to 1..=3 (`MAX_SUPPORTED_WIZARD_LEVEL =
                // 3`): the specialist bonus slot flat count changes for real at level
                // 3, from 1 to 2, since a level-3 wizard casts 2nd-level spells for
                // the first time (verified independently against both primary
                // sources' raw spells-per-day table rows); the level-3 "Special"
                // column is also blank, so no new pillar is added. A further SD13-E5
                // slice widens the gate again to 1..=4 (`MAX_SUPPORTED_WIZARD_LEVEL =
                // 4`): the specialist bonus slot flat count is checked rather than
                // assumed to double again and correctly stays at 2 (3rd-level wizard
                // spells do not become available until wizard level 5, verified
                // independently against both primary sources' raw spells-per-day
                // table rows); Intense Spells' bonus-damage magnitude changes for
                // real at level 4, from 1 to 2, via the pre-existing half-wizard-
                // level-minimum-1 formula; the level-4 "Special" column is also
                // blank, so no new pillar is added. A further SD13-E5 slice widens
                // the gate again to 1..=5 (`MAX_SUPPORTED_WIZARD_LEVEL = 5`): the
                // specialist bonus slot flat count changes for real at level 5, from
                // 2 to 3, since a level-5 wizard casts 3rd-level spells for the
                // first time (verified independently against both primary sources'
                // raw spells-per-day table rows); Intense Spells' bonus-damage
                // magnitude stays 2 (max(5/2, 1) = 2, an integer-division
                // coincidence); the level-5 "Special" column reads "Bonus feat" — a
                // genuinely new Wizard class feature, checked and confirmed NOT flat
                // (a choice among an open-ended metamagic/item creation feat set or
                // Spell Mastery, mirroring the Monk High Jump precedent), so it is
                // deliberately left named-but-unproven and no record is fabricated. A
                // further SD13-E5 slice widens the gate again to 1..=6
                // (`MAX_SUPPORTED_WIZARD_LEVEL = 6`): the specialist bonus slot flat
                // count is checked rather than assumed to rise again and correctly
                // stays at 3 (4th-level wizard spells do not become available until
                // wizard level 7, verified independently against both primary
                // sources' raw spells-per-day table rows); Intense Spells'
                // bonus-damage magnitude changes for real at level 6, from 2 to 3,
                // via the pre-existing half-wizard-level-minimum-1 formula; the
                // level-6 "Special" column is genuinely blank, so no new pillar is
                // added. A further SD13-E5 slice widens the gate again to 1..=7
                // (`MAX_SUPPORTED_WIZARD_LEVEL = 7`): base attack bonus and all
                // three base saves stay numerically unchanged from level 6 (an
                // integer-division coincidence, re-verified rather than assumed);
                // the specialist bonus slot flat count is checked rather than
                // assumed to stay put and correctly RISES to 4, since the raw
                // Wizard spells-per-day table's level-7 row is "4/4/3/2/1" — the
                // first non-"—" 4th-level column, so a level-7 specialist now
                // casts 4th-level spells for the first time; Intense Spells'
                // bonus-damage magnitude stays at 3 (max(7/2, 1) = 3, another
                // integer-division coincidence); the level-7 "Special" column is
                // genuinely blank, so no new pillar is added. A further SD13-E5
                // slice widens the gate to level 8: base attack genuinely rises to
                // +4 and good Will genuinely rises to +6 (poor Fortitude/Reflex
                // stay +2, integer-division coincidences); the specialist bonus
                // slot flat count is checked rather than assumed and correctly
                // STAYS at 4, since the raw spells-per-day table's level-8 row is
                // "4/4/3/3/2" with the 5th-level column still "—" (5th-level
                // spells first appear at level 9); Intense Spells' bonus-damage
                // magnitude genuinely rises to 4 (max(8/2, 1) = 4); the level-8
                // "Special" column is genuinely blank, so no new pillar is added.
                dimension: "bounded spell-bearing class progression: the deterministic Human \
                            Wizard level-1/level-8 prepared arcane spell baseline, with base \
                            attack bonus, base save progression, Scribe Scroll, the school \
                            specialization choice, the specialist-bonus-slot flat count (which \
                            becomes 2 at level 3, stays 2 at level 4, becomes 3 at level 5, stays \
                            3 at level 6, becomes 4 at level 7, and stays 4 at level 8), and the \
                            Intense Spells / \
                            Force Missile school-power flat magnitudes (Intense Spells becomes 2 \
                            at level 4, stays 2 at level 5, becomes 3 at level 6, stays 3 at \
                            level 7, and becomes 4 at level 8) grounded for real through level 8, \
                            and the school-power \
                            execution machinery, the opposed-school-cost burden, the level-5 \
                            bonus-feat selection/execution, and the prepared spellbook / \
                            spell-slot posture burden still blocked",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_WIZARD_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E4-R3 leaves direct computed evidence that the \
                    deterministic Human Wizard level-1 prepared arcane spell-bearing identity is \
                    recognized on the compute seam, a later SD13-E4 Wizard decomposition slice \
                    grounds Scribe Scroll (the free, specialization-independent bonus feat every \
                    1st-level Wizard is granted, letting them create scrolls of spells they know) \
                    for real, a SD13-E5 specialization slice grounds the school specialization \
                    choice (the canonical Evocation specialist with Necromancy and Transmutation \
                    opposed) as a recognition record plus the specialist bonus slot as a flat \
                    count only (one 1st-level Evocation-only bonus slot at level 1, no \
                    cantrip-level bonus slot, no slot contents), AND a further SD13-E5 slice \
                    grounds two of the Evocation school's own 1st-level school powers as flat \
                    numeric magnitudes after independent verification against the legacy Paizo \
                    PRD mirror: Intense Spells' bonus-damage magnitude (half wizard level, \
                    minimum 1) and Force Missile's uses-per-day pool (3 + Intelligence modifier), \
                    AND a further SD13-E5 slice grounds the foundational base-attack-bonus / \
                    base-save progression pillar that every other class row in this matrix \
                    (Fighter, Barbarian, Monk, Rogue, Paladin, Druid, Cleric, Bard, Sorcerer) \
                    already had and Wizard never had: base attack bonus (1/2 BAB, classlevel / 2 \
                    — the same shape as Sorcerer, UNLIKE the 3/4 BAB shared by \
                    Rogue/Monk/Druid/Cleric/Bard) and base save progression (good Will only, \
                    poor Fortitude, poor Reflex), both verified against the PF1 Core Rulebook \
                    Wizard class table (d20pfsrd and the legacy Paizo PRD mirror), reading the \
                    raw level 1-6 table rows directly to disambiguate the 1/2-vs-3/4 fraction \
                    since level 1 alone floors every fraction to +0, both grounded as standalone \
                    explanation records not wired into compute_total_saves or \
                    compute_combat_baseline, AND a further SD13-E5 slice widens the level-1-only \
                    gate to a level-range gate (level 1-2) and extends every one of the above \
                    formulas to level 2 via the same formula, without re-derivation, verified \
                    independently against the PF1 Core Rulebook Wizard class table (d20pfsrd and \
                    legacy.aonprd.com): level 2 base attack bonus is +1, base saves are +0/+0/+3 \
                    (Fortitude/Reflex/Will); the specialist bonus slot stays exactly 1 (a level-2 \
                    wizard still only casts 1st-level spells, since 2nd-level wizard spells begin \
                    at caster level 3); Intense Spells' bonus damage stays 1, reached naturally \
                    (max(2/2, 1) = 1) rather than via the level-1 floor; Force Missile's \
                    uses-per-day pool is level-independent and unchanged; Scribe Scroll is \
                    granted once, at 1st level only, and stays recognized as an already-held \
                    grant; the class table's level-2 \"Special\" column is blank, so no new class \
                    feature is gained at 2nd level (unlike Rogue/Monk/Druid's Evasion/Woodland \
                    Stride, but like Cleric/Sorcerer) — this slice widens existing pillars only, \
                    adds no new one. AND a further SD13-E5 slice widens the level-range gate to \
                    level 1-3 and extends the base attack/base save/specialization-choice/Intense- \
                    Spells/Force-Missile/Scribe-Scroll formulas to level 3 via the same formulas, \
                    without re-derivation, verified independently against the PF1 Core Rulebook \
                    Wizard class table (d20pfsrd and legacy.aonprd.com): level 3 base attack bonus \
                    is +1, base saves are +1/+1/+3 (Fortitude/Reflex/Will); the specialist bonus \
                    slot flat count CHANGES for real at level 3, from 1 to 2 (one 1st-level bonus \
                    slot plus one 2nd-level bonus slot), since a level-3 wizard casts 2nd-level \
                    spells for the first time (verified against both primary sources' raw \
                    spells-per-day table rows: level 2 shows \"4/2/—/—\", level 3 shows \"4/2/1/—\"); \
                    Intense Spells' bonus damage stays 1 (max(3/2, 1) = 1); Force Missile's \
                    uses-per-day pool is level-independent and unchanged; Scribe Scroll stays \
                    recognized as an already-held grant; the class table's level-3 \"Special\" \
                    column is also blank, so no new class feature is gained at 3rd level (unlike \
                    Rogue/Monk/Barbarian's own 3rd-level Trap Sense/Still Mind/Trap Sense features) \
                    — this slice widens existing pillars only (one of them, the specialist bonus \
                    slot count, to a new value), adds no new pillar record. AND a further SD13-E5 \
                    slice widens the level-range gate to level 1-4 and extends the base \
                    attack/base save/specialization-choice/specialist-bonus-slot/Intense-Spells/ \
                    Force-Missile/Scribe-Scroll formulas to level 4 via the same formulas, without \
                    re-derivation, verified independently against the PF1 Core Rulebook Wizard \
                    class table (d20pfsrd and legacy.aonprd.com): level 4 base attack bonus is +2, \
                    base saves are +1/+1/+4 (Fortitude/Reflex/Will); the specialist bonus slot flat \
                    count was checked rather than assumed to double again at level 4 (mirroring \
                    the level-3 doubling precedent) and correctly STAYS at 2, since the raw \
                    Wizard spells-per-day table's level-4 row is still \"4/3/2/—/—\" — 3rd-level \
                    wizard spells do not become available until wizard level 5 (level 5 row: \
                    \"4/3/2/1/—\", the first non-\"—\" 3rd-level column); Intense Spells' \
                    bonus-damage magnitude, in contrast, DOES change for real at level 4: \
                    max(4/2, 1) = 2, up from 1 at levels 1-3, via the pre-existing formula, not \
                    re-derived; Force Missile's uses-per-day pool is level-independent and \
                    unchanged; Scribe Scroll stays recognized as an already-held grant. The class \
                    table's level-4 \"Special\" column is also blank (verified independently \
                    against both sources: the Wizard's own next class feature, a bonus feat, is \
                    granted at 5th level, not 4th) — this slice widens existing pillars only (one \
                    of them, Intense Spells, to a genuinely new value), adds no new pillar record. \
                    AND a further SD13-E5 slice widens the level-range gate to level 1-5 and \
                    extends the base attack/base save/specialization-choice/specialist-bonus-slot/ \
                    Intense-Spells/Force-Missile/Scribe-Scroll formulas to level 5 via the same \
                    formulas, without re-derivation, verified independently against the PF1 Core \
                    Rulebook Wizard class table (d20pfsrd and a second independent Archives of \
                    Nethys mirror): level 5 base attack bonus is +2, base saves are +1/+1/+4 \
                    (Fortitude/Reflex/Will) — all four values numerically IDENTICAL to level 4, an \
                    integer-division coincidence, not a sign any formula stopped scaling; the \
                    specialist bonus slot flat count DOES change for real at level 5: the raw \
                    Wizard spells-per-day table's level-5 row is \"4/3/2/1/—\" — 3rd-level wizard \
                    spells become available for the first time at wizard level 5 — so the flat \
                    count becomes 3 (one bonus slot of each spell level 1st through 3rd), up from \
                    2 at levels 3-4; Intense Spells' bonus-damage magnitude, in contrast, STAYS at \
                    2 at level 5 (max(5/2, 1) = 2, another integer-division coincidence); Force \
                    Missile's uses-per-day pool is level-independent and unchanged; Scribe Scroll \
                    stays recognized as an already-held grant. The class table's level-5 \"Special\" \
                    column reads \"Bonus feat\" (verified independently against both sources) — a \
                    genuinely NEW Wizard class feature at 5th level, but checked and confirmed NOT \
                    flat: the feat is chosen from an open-ended set of metamagic feats, item \
                    creation feats, or the single named Spell Mastery feature — a general \
                    feat-selection/feat-prerequisite engine, not a flat magnitude, mirroring the \
                    Monk High Jump precedent exactly, so it is deliberately left \
                    named-but-unproven; no record or diagnostic is fabricated for it. This slice \
                    widens existing pillars only (one of them, the specialist bonus slot count, to \
                    a genuinely new value), adds no new pillar record. AND a further SD13-E5 \
                    slice widens the level-range gate to level 1-6 and extends the base \
                    attack/base save/specialization-choice/specialist-bonus-slot/Intense-Spells/ \
                    Force-Missile/Scribe-Scroll formulas to level 6 via the same formulas, without \
                    re-derivation, verified independently against the PF1 Core Rulebook Wizard \
                    class table (d20pfsrd and legacy.aonprd.com): level 6 base attack bonus is +3, \
                    base saves are +2/+2/+5 (Fortitude/Reflex/Will) — all four values genuinely \
                    NEW, up from +2/+1/+1/+4 at level 5; the specialist bonus slot flat count was \
                    checked rather than assumed to rise again and correctly STAYS at 3, since the \
                    raw Wizard spells-per-day table's level-6 row is \"4/3/3/2/—\" — 4th-level \
                    wizard spells do not become available until wizard level 7 (level 7 row: \
                    \"4/4/3/2/1\", the first non-\"—\" 4th-level column); Intense Spells' \
                    bonus-damage magnitude, in contrast, DOES change for real at level 6: \
                    max(6/2, 1) = 3, up from 2 at level 5, via the pre-existing formula, not \
                    re-derived; Force Missile's uses-per-day pool is level-independent and \
                    unchanged; Scribe Scroll stays recognized as an already-held grant. The class \
                    table's level-6 \"Special\" column is genuinely BLANK (verified independently \
                    against both sources, checked rather than assumed away) — UNLIKE the level-5 \
                    \"Bonus feat\" entry, so no new Wizard class feature is gained at 6th level — \
                    this slice widens existing pillars only (one of them, Intense Spells, to a \
                    genuinely new value), adds no new pillar record. AND a further SD13-E5 slice \
                    widens the level-range gate to level 1-7 and extends the base attack/base \
                    save/specialization-choice/specialist-bonus-slot/Intense-Spells/Force-Missile/ \
                    Scribe-Scroll formulas to level 7 via the same formulas, without re-derivation, \
                    verified independently against the PF1 Core Rulebook Wizard class table \
                    (d20pfsrd and legacy.aonprd.com): level 7 base attack bonus and all three base \
                    saves are numerically UNCHANGED from level 6 (+3 base attack, +2/+2/+5 \
                    Fortitude/Reflex/Will) — an integer-division coincidence, re-verified rather \
                    than assumed; the specialist bonus slot flat count was checked rather than \
                    assumed to stay put and correctly RISES to 4, since the raw Wizard \
                    spells-per-day table's level-7 row is \"4/4/3/2/1\" — the first non-\"—\" \
                    4th-level column, so a level-7 specialist now casts 4th-level spells for the \
                    first time (one bonus slot of each spell level 1st through 4th); Intense \
                    Spells' bonus-damage magnitude, in contrast, STAYS at 3 at level 7 \
                    (max(7/2, 1) = 3, another integer-division coincidence, not a sign the formula \
                    stopped scaling); Force Missile's uses-per-day pool is level-independent and \
                    unchanged; Scribe Scroll stays recognized as an already-held grant. The class \
                    table's level-7 \"Special\" column is genuinely BLANK (verified independently \
                    against both sources, checked rather than assumed away), so no new Wizard \
                    class feature is gained at 7th level — this slice widens existing pillars only \
                    (one of them, the specialist bonus slot, to a genuinely new value), adds no \
                    new pillar record — AND a further SD13-E5 slice widens the level-range gate \
                    again (supported_wizard_level, 1..=8) and extends every one of the formulas \
                    above to level 8 via the same formula, without re-derivation, verified \
                    independently against the PF1 Core Rulebook Wizard class table (d20pfsrd and \
                    legacy.aonprd.com): level 8 base attack bonus is +4 (genuinely risen from +3) \
                    and base saves are +2/+2/+6 (Fortitude/Reflex/Will — good Will genuinely \
                    rises from +5, while poor Fortitude and poor Reflex both stay +2, \
                    integer-division coincidences); the specialist bonus slot flat count was \
                    checked rather than assumed and correctly STAYS at 4, since the raw Wizard \
                    spells-per-day table's level-8 row is \"4/4/3/3/2\" with the 5th-level column \
                    still \"—\" — 5th-level wizard spells first appear at level 9, so the next \
                    slot-count rise lands there, not at 8; Intense Spells' bonus-damage \
                    magnitude, in contrast, GENUINELY RISES to 4 at level 8 (max(8/2, 1) = 4, up \
                    from 3 at levels 6-7, via the same pre-existing formula); Force Missile's \
                    uses-per-day pool is level-independent and unchanged; Scribe Scroll stays \
                    recognized as an already-held grant. The class table's level-8 \"Special\" \
                    column is genuinely BLANK (verified independently against both sources, \
                    checked rather than assumed away — the Wizard's bonus feats land at levels \
                    5/10/15/20), so no new Wizard class feature is gained at 8th level — this \
                    slice widens existing pillars only, adds no new pillar record. The row is \
                    Partial, not Supported: neither school power's \
                    execution machinery is implemented (no evocation spell-damage application for \
                    Intense Spells, no force-missile casting execution / 1d4 damage roll / \
                    automatic-hit targeting for Force Missile), the opposed-school preparation \
                    cost (each opposed-school spell occupies two prepared slots) remains named \
                    and unproven, the level-5 bonus feat's own selection/execution (a general \
                    feat-selection/feat-prerequisite engine) remains named and unproven, and the \
                    prepared spell posture burden (spellbook content, spells prepared per day, \
                    spell slots per day, bonus slots from high Intelligence, spell save DCs) is \
                    still entirely unproven. No spell math is fabricated and no Wizard level 9+ is \
                    proven",
                next_required_uplift: "SD13-E5 Wizard school-power execution and opposed-school \
                    preparation-cost grounding slice, then the prepared spellbook / spell-slot \
                    posture slice, then level-9+ progression (widening the now-grounded base \
                    attack/base save formulas)",
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
