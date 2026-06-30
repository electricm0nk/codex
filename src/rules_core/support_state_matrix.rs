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
//! - Fighter levels 2-10 and Rogue level 1 are `Blocked` / `Computed` because the
//!   live GE-06 tests explicitly claim-block them,
//! - the Human bonus-feat / ability-bonus interaction seam is `Partial` / `Computed`,
//! - every other core race, core class, and the broader non-Human interaction row
//!   remain `Unverified` / `Observed` (named by SD-13 scope only, no runtime
//!   evidence yet).

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

/// The accepted GE-06 deterministic Human Fighter level-1 input fixture.
const GE06_DETERMINISTIC_FIXTURE: &str =
    "tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt";

/// GE-06 deterministic pilot input contract proof (race/class identity, Human
/// bonus-feat and ability-bonus selections).
const GE06_INPUT_CONTRACT_TEST: &str = "tests/ge06_pilot_input_contract.rs";

/// GE-06 total saving throws proof. Also claim-blocks Rogue level 1 and Fighter
/// level 2.
const GE06_TOTAL_SAVES_TEST: &str = "tests/ge06_pilot_total_saves.rs";

/// GE-06 baseline combat values proof. Also claim-blocks Fighter level 2 for
/// combat/defense surfaces.
const GE06_COMBAT_BASELINE_TEST: &str = "tests/ge06_pilot_combat_baseline.rs";

/// GE-06 pilot view-model projection proof over the bounded computed snapshot.
const GE06_VIEW_MODEL_TEST: &str = "tests/ge06_pilot_view_model.rs";

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
                dimension: "bounded pilot race semantics actually exercised by the \
                            GE-06 deterministic proof",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                grounding_ref: GE06_DETERMINISTIC_FIXTURE,
                blocker_or_lossiness_note: "current evidence proves only the bounded \
                    Human seam exercised by the deterministic pilot, not the full \
                    Human race burden",
                next_required_uplift: "classify remaining Human race semantics explicitly",
            },
            SupportStateRow {
                row_id: "race.dwarf.bounded_semantics",
                subject_type: MatrixSubjectType::Race,
                subject_id: "race:dwarf",
                dimension: "bounded race semantics",
                support_state: SupportState::Unverified,
                evidence_tier: EvidenceTier::Observed,
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
                dimension: "class progression through levels 2-10",
                support_state: SupportState::Blocked,
                evidence_tier: EvidenceTier::Computed,
                grounding_ref: GE06_COMBAT_BASELINE_TEST,
                blocker_or_lossiness_note: "GE-06 tests explicitly claim-block \
                    class:fighter:2 (wrong_fighter_level_blocks_combat_totals in \
                    tests/ge06_pilot_combat_baseline.rs and \
                    wrong_fighter_level_blocks_total_saves in tests/ge06_pilot_total_saves.rs), \
                    so levels 2-10 cannot be claimed",
                next_required_uplift: "SD13-E3 martial progression slice",
            },
            SupportStateRow {
                row_id: "class.rogue.bounded_progression",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:rogue",
                dimension: "bounded class progression",
                support_state: SupportState::Blocked,
                evidence_tier: EvidenceTier::Computed,
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
                support_state: SupportState::Unverified,
                evidence_tier: EvidenceTier::Observed,
                grounding_ref: SD13_ROSTER_MATRIX_DOC,
                blocker_or_lossiness_note: "",
                next_required_uplift: "SD13-E3 martial progression slice",
            },
            SupportStateRow {
                row_id: "class.bard.progression_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:bard",
                dimension: "bounded class progression and spell burden",
                support_state: SupportState::Unverified,
                evidence_tier: EvidenceTier::Observed,
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
                grounding_ref: SD13_ROSTER_MATRIX_DOC,
                blocker_or_lossiness_note: "",
                next_required_uplift: "SD13-E3 martial progression slice",
            },
            SupportStateRow {
                row_id: "class.paladin.hybrid_chassis_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:paladin",
                dimension: "bounded class progression and hybrid spell burden",
                support_state: SupportState::Unverified,
                evidence_tier: EvidenceTier::Observed,
                grounding_ref: SD13_ROSTER_MATRIX_DOC,
                blocker_or_lossiness_note: "",
                next_required_uplift: "SD13-E3 then SD13-E4",
            },
            SupportStateRow {
                row_id: "class.ranger.hybrid_chassis_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:ranger",
                dimension: "bounded class progression and hybrid spell burden",
                support_state: SupportState::Unverified,
                evidence_tier: EvidenceTier::Observed,
                grounding_ref: SD13_ROSTER_MATRIX_DOC,
                blocker_or_lossiness_note: "",
                next_required_uplift: "SD13-E3 then SD13-E4",
            },
            SupportStateRow {
                row_id: "class.sorcerer.progression_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:sorcerer",
                dimension: "bounded class progression and spell burden",
                support_state: SupportState::Unverified,
                evidence_tier: EvidenceTier::Observed,
                grounding_ref: SD13_ROSTER_MATRIX_DOC,
                blocker_or_lossiness_note: "",
                next_required_uplift: "SD13-E4 spellcasting slice",
            },
            SupportStateRow {
                row_id: "class.wizard.progression_and_spell_burden",
                subject_type: MatrixSubjectType::Class,
                subject_id: "class:wizard",
                dimension: "bounded class progression and spell burden",
                support_state: SupportState::Unverified,
                evidence_tier: EvidenceTier::Observed,
                grounding_ref: SD13_ROSTER_MATRIX_DOC,
                blocker_or_lossiness_note: "",
                next_required_uplift: "SD13-E4 spellcasting slice",
            },
            // ----- Interaction rows (2) -----
            SupportStateRow {
                row_id: "interaction.human_bonus_feat_ability_bonus.pilot_pressure",
                subject_type: MatrixSubjectType::Interaction,
                subject_id: "interaction:human-bonus-feat-ability-bonus",
                dimension: "race/class interaction pressure on the deterministic pilot path",
                support_state: SupportState::Partial,
                evidence_tier: EvidenceTier::Computed,
                grounding_ref: GE06_INPUT_CONTRACT_TEST,
                blocker_or_lossiness_note: "only the deterministic Human Fighter pilot \
                    seam (human_bonus_feat and human_ability_bonus selections) is grounded, \
                    not the general interaction-row model",
                next_required_uplift: "SD13-E2 / SD13-E3 coupling",
            },
            SupportStateRow {
                row_id: "interaction.non_human_any_class.progression_pressure",
                subject_type: MatrixSubjectType::Interaction,
                subject_id: "interaction:non-human-any-class-progression",
                dimension: "race/class interaction pressure beyond the pilot",
                support_state: SupportState::Unverified,
                evidence_tier: EvidenceTier::Observed,
                grounding_ref: SD13_ROSTER_MATRIX_DOC,
                blocker_or_lossiness_note: "",
                next_required_uplift: "add named interaction rows only where separate \
                    race and class rows are insufficient",
            },
        ],
    }
}
