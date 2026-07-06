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
//!   proves Fighter levels 2 and 3 (base progression, the level-2 bonus-feat seam,
//!   and the level-3 armor-training seam), while levels 4-10 remain out of proof,
//! - Rogue level 1 is `Blocked` / `Computed` because the live GE-06 test explicitly
//!   claim-blocks it, keeping it an explicit negative-control seam,
//! - the Paladin and Ranger hybrid rows are `Blocked` / `Computed`: the SD13-E3-F6
//!   slice proves the deterministic Human Paladin level-1 and Human Ranger level-1
//!   hybrid chassis are recognized on the compute seam, but both stay blocked on the
//!   named non-spell class-feature burden and the later spell burden,
//! - the Sorcerer row is `Blocked` / `Computed`: the SD13-E4-F7 slice proves the
//!   deterministic Human Sorcerer level-1 spontaneous arcane spell-bearing identity is
//!   recognized on the compute seam, but it stays blocked on the bloodline burden and the
//!   spontaneous known-spell / slot posture burden, and fabricates no spell math,
//! - the Human bonus-feat / ability-bonus interaction seam is `Partial` / `Computed`,
//! - every other core race, core class (including Bard and Wizard), and the broader
//!   non-Human interaction row remain `Unverified` / `Observed` (named by SD-13 scope
//!   only, no runtime evidence yet).

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

/// The live compute seam that now makes the grounded Human race pressure explicit by
/// emitting named Human ability-bonus and bonus-feat explanation records.
const PILOT_COMPUTE_MODULE: &str = "src/rules_core/pilot_compute.rs";

/// GE-06 deterministic pilot input contract proof (race/class identity, Human
/// bonus-feat and ability-bonus selections).
const GE06_INPUT_CONTRACT_TEST: &str = "tests/ge06_pilot_input_contract.rs";

/// GE-06 total saving throws proof. Also claim-blocks Rogue level 1 and Fighter
/// level 4.
const GE06_TOTAL_SAVES_TEST: &str = "tests/ge06_pilot_total_saves.rs";

/// GE-06 pilot view-model projection proof over the bounded computed snapshot.
const GE06_VIEW_MODEL_TEST: &str = "tests/ge06_pilot_view_model.rs";

/// SD13-E3 dedicated proof surface for the bounded Fighter levels-2-and-3 milestone
/// tranche (base progression, level-2 bonus-feat seam, level-3 armor-training seam).
const SD13_FIGHTER_LEVEL2_LEVEL3_TEST: &str = "tests/sd13_fighter_level2_level3_progression.rs";

/// SD13-E3-F6 dedicated proof surface for the bounded Paladin and Ranger level-1 hybrid
/// chassis baseline: direct computed chassis-recognition evidence that stays explicitly
/// blocked on the named non-spell class-feature burden and the later spell burden.
const SD13_HYBRID_LEVEL1_TEST: &str = "tests/sd13_hybrid_level1_chassis_baseline.rs";

/// SD13-E4-F7 dedicated proof surface for the bounded Human Sorcerer level-1 spell
/// baseline: direct computed recognition of the spontaneous arcane spell-bearing identity
/// that stays explicitly blocked on the bloodline burden and the spontaneous
/// known-spell / slot posture burden.
const SD13_SORCERER_LEVEL1_TEST: &str = "tests/sd13_sorcerer_level1_spell_baseline.rs";

/// SD13-E3 dedicated proof surface for the bounded Human Barbarian level-1 martial
/// chassis baseline: direct computed chassis-recognition evidence that stays explicitly
/// blocked on the four named martial pillar burdens (base attack, base save,
/// fast movement, illiteracy trait).
const SD13_BARBARIAN_LEVEL1_TEST: &str = "tests/sd13_barbarian_level1_chassis_baseline.rs";

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
                            selections exercised by the GE-06 deterministic proof",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: PILOT_COMPUTE_MODULE,
                blocker_or_lossiness_note: "the deterministic pilot grounds only the named \
                    Human ability-bonus and bonus-feat pressure; Human size, speed, senses, \
                    extra skill ranks, and the remaining racial trait burden are still unverified",
                next_required_uplift: "classify the remaining Human racial trait burden \
                    (size, speed, senses, skill ranks) explicitly",
            },
            SupportStateRow {
                row_id: "race.dwarf.bounded_semantics",
                subject_type: MatrixSubjectType::Race,
                subject_id: "race:dwarf",
                dimension: "bounded race semantics",
                support_state: SupportState::Unverified,
                evidence_tier: EvidenceTier::Observed,
                evidence_freshness: EvidenceFreshness::AwaitingInitialEvidence,
                grounding_ref: SD13_ROSTER_MATRIX_DOC,
                blocker_or_lossiness_note: "",
                next_required_uplift: "SD13-E2 race-semantic slice",
            },
            SupportStateRow {
                row_id: "race.elf.bounded_semantics",
                subject_type: MatrixSubjectType::Race,
                subject_id: "race:elf",
                dimension: "bounded race semantics",
                support_state: SupportState::Unverified,
                evidence_tier: EvidenceTier::Observed,
                evidence_freshness: EvidenceFreshness::AwaitingInitialEvidence,
                grounding_ref: SD13_ROSTER_MATRIX_DOC,
                blocker_or_lossiness_note: "",
                next_required_uplift: "SD13-E2 race-semantic slice",
            },
            SupportStateRow {
                row_id: "race.gnome.bounded_semantics",
                subject_type: MatrixSubjectType::Race,
                subject_id: "race:gnome",
                dimension: "bounded race semantics",
                support_state: SupportState::Unverified,
                evidence_tier: EvidenceTier::Observed,
                evidence_freshness: EvidenceFreshness::AwaitingInitialEvidence,
                grounding_ref: SD13_ROSTER_MATRIX_DOC,
                blocker_or_lossiness_note: "",
                next_required_uplift: "SD13-E2 race-semantic slice",
            },
            SupportStateRow {
                row_id: "race.half_elf.bounded_semantics",
                subject_type: MatrixSubjectType::Race,
                subject_id: "race:half-elf",
                dimension: "bounded race semantics",
                support_state: SupportState::Unverified,
                evidence_tier: EvidenceTier::Observed,
                evidence_freshness: EvidenceFreshness::AwaitingInitialEvidence,
                grounding_ref: SD13_ROSTER_MATRIX_DOC,
                blocker_or_lossiness_note: "",
                next_required_uplift: "SD13-E2 race-semantic slice",
            },
            SupportStateRow {
                row_id: "race.half_orc.bounded_semantics",
                subject_type: MatrixSubjectType::Race,
                subject_id: "race:half-orc",
                dimension: "bounded race semantics",
                support_state: SupportState::Unverified,
                evidence_tier: EvidenceTier::Observed,
                evidence_freshness: EvidenceFreshness::AwaitingInitialEvidence,
                grounding_ref: SD13_ROSTER_MATRIX_DOC,
                blocker_or_lossiness_note: "",
                next_required_uplift: "SD13-E2 race-semantic slice",
            },
            SupportStateRow {
                row_id: "race.halfling.bounded_semantics",
                subject_type: MatrixSubjectType::Race,
                subject_id: "race:halfling",
                dimension: "bounded race semantics",
                support_state: SupportState::Unverified,
                evidence_tier: EvidenceTier::Observed,
                evidence_freshness: EvidenceFreshness::AwaitingInitialEvidence,
                grounding_ref: SD13_ROSTER_MATRIX_DOC,
                blocker_or_lossiness_note: "",
                next_required_uplift: "SD13-E2 race-semantic slice",
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
                grounding_ref: GE06_VIEW_MODEL_TEST,
                blocker_or_lossiness_note: "only the bounded Fighter level-1 deterministic \
                    pilot surface is proven; mandatory level-10 milestones remain unclassified",
                next_required_uplift: "widen beyond level 1 and classify mandatory \
                    level-10 milestones",
            },
            SupportStateRow {
                row_id: "class.fighter.levels_2_10",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:fighter",
                dimension: "class progression across levels 2-10: bounded milestone proof \
                            for levels 2 and 3 only, with levels 4-10 still unproven",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_FIGHTER_LEVEL2_LEVEL3_TEST,
                blocker_or_lossiness_note: "SD13-E3 proves only Fighter levels 2 and 3: base \
                    attack / base save progression, the level-2 bonus-feat progression seam, and \
                    the level-3 armor-training seam over the deterministic Human loadout. Levels \
                    4-10 remain out of proof, along with level-4 ability-score progression, the \
                    repeated bonus-feat cadence, weapon training, later armor-training ranks, and \
                    any general feat-effect/prerequisite engine",
                next_required_uplift: "later SD13-E3 slice widening Fighter beyond level 3 toward \
                    the level-10 milestones",
            },
            SupportStateRow {
                row_id: "class.rogue.bounded_progression",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:rogue",
                dimension: "bounded class progression",
                support_state: SupportState::Blocked,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: GE06_TOTAL_SAVES_TEST,
                blocker_or_lossiness_note: "tests/ge06_pilot_total_saves.rs \
                    (unsupported_chassis_blocks_total_saves) explicitly claim-blocks \
                    class:rogue:1 under the current bounded compute path",
                next_required_uplift: "SD13-E3 martial progression slice",
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
                blocker_or_lossiness_note: "SD13-E3 Barbarian level-1 proof surfaces only the \
                    bounded martial chassis-recognition record; the slice is explicitly blocked \
                    on the four still-missing martial pillar burdens: base attack progression \
                    (full BAB and the higher-level BAB cadence), base save progression (good \
                    Fortitude +3 and the poor Reflex / poor Will base-save cadence), fast \
                    movement (+10 ft. land speed extension while wearing no heavy armor), and \
                    the illiteracy trait. No rage execution, weapon familiarity, or level-2+ \
                    martial progression is claimed",
                next_required_uplift: "widen beyond level 1 by grounding base-attack / base-save \
                    progression, fast-movement speed extension, and the illiteracy trait engine, \
                    later widening into rage execution and level-2+ martial progression",
            },
            SupportStateRow {
                row_id: "class.bard.progression_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:bard",
                dimension: "bounded class progression and spell burden",
                support_state: SupportState::Unverified,
                evidence_tier: EvidenceTier::Observed,
                evidence_freshness: EvidenceFreshness::AwaitingInitialEvidence,
                grounding_ref: SD13_ROSTER_MATRIX_DOC,
                blocker_or_lossiness_note: "",
                next_required_uplift: "SD13-E4 spellcasting slice",
            },
            SupportStateRow {
                row_id: "class.cleric.progression_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:cleric",
                dimension: "bounded class progression and spell burden",
                support_state: SupportState::Unverified,
                evidence_tier: EvidenceTier::Observed,
                evidence_freshness: EvidenceFreshness::AwaitingInitialEvidence,
                grounding_ref: SD13_ROSTER_MATRIX_DOC,
                blocker_or_lossiness_note: "",
                next_required_uplift: "SD13-E4 spellcasting slice",
            },
            SupportStateRow {
                row_id: "class.druid.progression_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:druid",
                dimension: "bounded class progression and spell burden",
                support_state: SupportState::Unverified,
                evidence_tier: EvidenceTier::Observed,
                evidence_freshness: EvidenceFreshness::AwaitingInitialEvidence,
                grounding_ref: SD13_ROSTER_MATRIX_DOC,
                blocker_or_lossiness_note: "",
                next_required_uplift: "SD13-E4 spellcasting slice",
            },
            SupportStateRow {
                row_id: "class.monk.bounded_progression",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:monk",
                dimension: "bounded class progression",
                support_state: SupportState::Unverified,
                evidence_tier: EvidenceTier::Observed,
                evidence_freshness: EvidenceFreshness::AwaitingInitialEvidence,
                grounding_ref: SD13_ROSTER_MATRIX_DOC,
                blocker_or_lossiness_note: "",
                next_required_uplift: "SD13-E3 martial progression slice",
            },
            SupportStateRow {
                row_id: "class.paladin.hybrid_chassis_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:paladin",
                dimension: "bounded hybrid class progression: the deterministic Human \
                            Paladin level-1 chassis baseline, with the non-spell \
                            class-feature burden and the later spell burden still blocked",
                support_state: SupportState::Blocked,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_HYBRID_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E3-F6 leaves direct computed evidence that the \
                    deterministic Human Paladin level-1 hybrid chassis is recognized on the compute \
                    seam, but the row stays blocked: the non-spell class-feature burden (smite evil, \
                    lay on hands, divine grace, mercy) is not implemented, and the later paladin spell \
                    burden (spell slots, spell source, spells known/prepared) is deferred to SD13-E4. \
                    No Paladin level 2+ is proven",
                next_required_uplift: "SD13-E3 paladin class-feature slice, then SD13-E4 spell burden",
            },
            SupportStateRow {
                row_id: "class.ranger.hybrid_chassis_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:ranger",
                dimension: "bounded hybrid class progression: the deterministic Human \
                            Ranger level-1 chassis baseline, with the non-spell \
                            class-feature burden and the later spell burden still blocked",
                support_state: SupportState::Blocked,
                evidence_tier: EvidenceTier::Computed,
                evidence_freshness: EvidenceFreshness::RefreshableFromLiveProof,
                grounding_ref: SD13_HYBRID_LEVEL1_TEST,
                blocker_or_lossiness_note: "SD13-E3-F6 leaves direct computed evidence that the \
                    deterministic Human Ranger level-1 hybrid chassis is recognized on the compute \
                    seam, but the row stays blocked: the non-spell class-feature burden (favored enemy, \
                    combat style, skill/tracking) is not implemented, and the later ranger spell burden \
                    (spell slots, spell source, spells known/prepared) is deferred to SD13-E4. No Ranger \
                    level 2+ is proven",
                next_required_uplift: "SD13-E3 ranger class-feature slice, then SD13-E4 spell burden",
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
                dimension: "bounded class progression and spell burden",
                support_state: SupportState::Unverified,
                evidence_tier: EvidenceTier::Observed,
                evidence_freshness: EvidenceFreshness::AwaitingInitialEvidence,
                grounding_ref: SD13_ROSTER_MATRIX_DOC,
                blocker_or_lossiness_note: "",
                next_required_uplift: "SD13-E4 spellcasting slice",
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
                next_required_uplift: "SD13-E2 / SD13-E3 coupling",
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
                blocker_or_lossiness_note: "",
                next_required_uplift: "add named interaction rows only where separate \
                    race and class rows are insufficient",
            },
        ],
    }
}
