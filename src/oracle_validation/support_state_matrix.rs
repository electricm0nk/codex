//! SD13-E1-F1 support-state matrix: typed, machine-usable current-truth surface.
//!
//! This module is the first SD-13 control-plane slice. It establishes a typed
//! carrier for the SD-13 support-state matrix and seeds the exact current
//! truthful rows already grounded by the SD-13 packet and the existing GE-06
//! repo evidence. It keeps **support state** and **evidence tier** as separate
//! axes, limits rows to the three SD-13 subject types (`Race`, `Class`,
//! `Interaction`), and refuses to silently promote any row to `Supported`.
//!
//! Deliberate non-goals (deferred to later SD-13 slices): no claim-composition
//! logic, no promotion rules, no mutable update orchestration, no UI/reporting
//! layer, no serialization/persistence framework, and no 7x11 combination
//! matrix. This slice stops at typed schema plus seeded current truth.

/// Whether a named dimension is proven, partially proven, lossy, blocked, or
/// unverified. Kept independent from [`EvidenceTier`]: a `Computed` row is not
/// necessarily `Supported`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportState {
    /// Proven at the required evidence floor with no known missing semantics
    /// inside the bounded claim. The initial seed contains none of these.
    Supported,
    /// Some required semantics are proven, but named required semantics remain
    /// incomplete and visible.
    Partial,
    /// The path works only by discarding or approximating named semantics.
    Lossy,
    /// Known missing semantics, explicit claim-blocking diagnostics, or
    /// contradictory behavior prevent the claim.
    Blocked,
    /// No direct evidence yet exists for the named dimension.
    Unverified,
}

/// Highest Codex quality-gate evidence tier achieved for a row. Tracked
/// independently from [`SupportState`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceTier {
    Observed,
    Parsed,
    Converted,
    Computed,
    OracleChecked,
    ProductVisible,
}

/// The three SD-13 matrix subject types. No cross-cutting row type is added in
/// this slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatrixSubjectType {
    Race,
    Class,
    Interaction,
}

/// One support-state matrix row. Carries a stable id, the subject it describes,
/// the bounded dimension, the two independent axes (support state and evidence
/// tier), a real grounding reference, an optional blocker/lossiness note, and
/// the next required uplift / owning slice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportStateRow {
    /// Stable, machine-usable row id (e.g. `race.human.pilot_semantics`).
    pub row_id: String,
    pub subject_type: MatrixSubjectType,
    /// Subject identity (e.g. `race:human`, `class:fighter`).
    pub subject_id: String,
    /// The bounded semantic or progression dimension this row describes.
    pub dimension: String,
    pub support_state: SupportState,
    pub evidence_tier: EvidenceTier,
    /// Real doc or repo evidence reference, not chat prose or invented receipts.
    pub grounding_ref: String,
    /// Required for every `Blocked` (and any future `Lossy`) row.
    pub blocker_or_lossiness_note: Option<String>,
    /// Next required uplift or owning slice.
    pub next_required_uplift: String,
}

/// A bounded carrier of SD-13 support-state rows. This slice exposes only the
/// seeded current truth and a narrow lookup helper; it adds no mutation,
/// promotion, or persistence behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportStateMatrix {
    pub rows: Vec<SupportStateRow>,
}

impl SupportStateMatrix {
    /// Narrow lookup: returns the first row with the given id, or `None`.
    pub fn row(&self, row_id: &str) -> Option<&SupportStateRow> {
        self.rows.iter().find(|row| row.row_id == row_id)
    }
}

/// Grounding reference for rows whose only current authority is the SD-13
/// roster/support-state artifact (the `Unverified` + `Observed` breadth rows).
const SD13_ROSTER_AUTHORITY: &str =
    "programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md";

/// Convenience builder for an `Unverified` + `Observed` row whose only current
/// authority is the SD-13 roster artifact.
fn unverified_row(
    row_id: &str,
    subject_type: MatrixSubjectType,
    subject_id: &str,
    dimension: &str,
    next_required_uplift: &str,
) -> SupportStateRow {
    SupportStateRow {
        row_id: row_id.to_owned(),
        subject_type,
        subject_id: subject_id.to_owned(),
        dimension: dimension.to_owned(),
        support_state: SupportState::Unverified,
        evidence_tier: EvidenceTier::Observed,
        grounding_ref: SD13_ROSTER_AUTHORITY.to_owned(),
        blocker_or_lossiness_note: None,
        next_required_uplift: next_required_uplift.to_owned(),
    }
}

/// The deterministic SD13-E1-F1 current-truth seed.
///
/// Exactly 21 rows: 7 race, 12 class, 2 interaction, in roster order. Only the
/// Human pilot race row, Fighter level-1 row, Fighter levels-2-10 row, Rogue
/// row, and Human interaction row rise above `Observed`; none is `Supported`.
/// Grounding references cite real SD-13 docs and existing GE-06 repo evidence.
pub fn seeded_sd13_e1_f1_current_truth() -> SupportStateMatrix {
    let rows = vec![
        // --- Race rows (7) ---
        SupportStateRow {
            row_id: "race.human.pilot_semantics".to_owned(),
            subject_type: MatrixSubjectType::Race,
            subject_id: "race:human".to_owned(),
            dimension: "bounded pilot race semantics actually exercised by the GE-06 deterministic proof"
                .to_owned(),
            support_state: SupportState::Partial,
            evidence_tier: EvidenceTier::Computed,
            grounding_ref:
                "tests/ge06_pilot_input_contract.rs; tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
                    .to_owned(),
            blocker_or_lossiness_note: None,
            next_required_uplift: "classify remaining Human race semantics explicitly".to_owned(),
        },
        unverified_row(
            "race.dwarf.bounded_semantics",
            MatrixSubjectType::Race,
            "race:dwarf",
            "bounded race semantics",
            "SD13-E2 race-semantic slice",
        ),
        unverified_row(
            "race.elf.bounded_semantics",
            MatrixSubjectType::Race,
            "race:elf",
            "bounded race semantics",
            "SD13-E2 race-semantic slice",
        ),
        unverified_row(
            "race.gnome.bounded_semantics",
            MatrixSubjectType::Race,
            "race:gnome",
            "bounded race semantics",
            "SD13-E2 race-semantic slice",
        ),
        unverified_row(
            "race.half_elf.bounded_semantics",
            MatrixSubjectType::Race,
            "race:half-elf",
            "bounded race semantics",
            "SD13-E2 race-semantic slice",
        ),
        unverified_row(
            "race.half_orc.bounded_semantics",
            MatrixSubjectType::Race,
            "race:half-orc",
            "bounded race semantics",
            "SD13-E2 race-semantic slice",
        ),
        unverified_row(
            "race.halfling.bounded_semantics",
            MatrixSubjectType::Race,
            "race:halfling",
            "bounded race semantics",
            "SD13-E2 race-semantic slice",
        ),
        // --- Class rows (12) ---
        SupportStateRow {
            row_id: "class.fighter.level_1_pilot".to_owned(),
            subject_type: MatrixSubjectType::Class,
            subject_id: "class:fighter".to_owned(),
            dimension: "class progression through level 1 deterministic pilot surface".to_owned(),
            support_state: SupportState::Partial,
            evidence_tier: EvidenceTier::Computed,
            grounding_ref:
                "tests/ge06_pilot_total_saves.rs; tests/ge06_pilot_combat_baseline.rs; tests/ge06_selected_parity_dimensions.rs; tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
                    .to_owned(),
            blocker_or_lossiness_note: None,
            next_required_uplift:
                "widen beyond level 1 and classify mandatory level-10 milestones".to_owned(),
        },
        SupportStateRow {
            row_id: "class.fighter.levels_2_10".to_owned(),
            subject_type: MatrixSubjectType::Class,
            subject_id: "class:fighter".to_owned(),
            dimension: "class progression through levels 2-10".to_owned(),
            support_state: SupportState::Blocked,
            evidence_tier: EvidenceTier::Computed,
            grounding_ref:
                "tests/ge06_pilot_total_saves.rs::wrong_fighter_level_blocks_total_saves (claim-blocks class:fighter:2)"
                    .to_owned(),
            blocker_or_lossiness_note: Some(
                "GE-06 tests explicitly claim-block Fighter level 2: a Fighter above level 1 produces a claim-blocking diagnostic and withholds total-save explanations rather than computing post-level-1 progression."
                    .to_owned(),
            ),
            next_required_uplift: "SD13-E3 martial progression slice".to_owned(),
        },
        SupportStateRow {
            row_id: "class.rogue.bounded_progression".to_owned(),
            subject_type: MatrixSubjectType::Class,
            subject_id: "class:rogue".to_owned(),
            dimension: "bounded class progression".to_owned(),
            support_state: SupportState::Blocked,
            evidence_tier: EvidenceTier::Computed,
            grounding_ref:
                "tests/ge06_pilot_total_saves.rs::unsupported_chassis_blocks_total_saves (claim-blocks class:rogue:1)"
                    .to_owned(),
            blocker_or_lossiness_note: Some(
                "GE-06 total-save test explicitly claim-blocks Rogue level 1: the non-Fighter chassis raises a claim-blocking diagnostic and withholds total-save explanations instead of fabricating Rogue progression."
                    .to_owned(),
            ),
            next_required_uplift: "SD13-E3 martial progression slice".to_owned(),
        },
        unverified_row(
            "class.barbarian.bounded_progression",
            MatrixSubjectType::Class,
            "class:barbarian",
            "bounded class progression",
            "SD13-E3 martial progression slice",
        ),
        unverified_row(
            "class.bard.progression_and_spell_burden",
            MatrixSubjectType::Class,
            "class:bard",
            "bounded class progression and spell burden",
            "SD13-E4 spellcasting slice",
        ),
        unverified_row(
            "class.cleric.progression_and_spell_burden",
            MatrixSubjectType::Class,
            "class:cleric",
            "bounded class progression and spell burden",
            "SD13-E4 spellcasting slice",
        ),
        unverified_row(
            "class.druid.progression_and_spell_burden",
            MatrixSubjectType::Class,
            "class:druid",
            "bounded class progression and spell burden",
            "SD13-E4 spellcasting slice",
        ),
        unverified_row(
            "class.monk.bounded_progression",
            MatrixSubjectType::Class,
            "class:monk",
            "bounded class progression",
            "SD13-E3 martial progression slice",
        ),
        unverified_row(
            "class.paladin.hybrid_chassis_and_spell_burden",
            MatrixSubjectType::Class,
            "class:paladin",
            "bounded class progression and hybrid spell burden",
            "SD13-E3 then SD13-E4",
        ),
        unverified_row(
            "class.ranger.hybrid_chassis_and_spell_burden",
            MatrixSubjectType::Class,
            "class:ranger",
            "bounded class progression and hybrid spell burden",
            "SD13-E3 then SD13-E4",
        ),
        unverified_row(
            "class.sorcerer.progression_and_spell_burden",
            MatrixSubjectType::Class,
            "class:sorcerer",
            "bounded class progression and spell burden",
            "SD13-E4 spellcasting slice",
        ),
        unverified_row(
            "class.wizard.progression_and_spell_burden",
            MatrixSubjectType::Class,
            "class:wizard",
            "bounded class progression and spell burden",
            "SD13-E4 spellcasting slice",
        ),
        // --- Interaction rows (2) ---
        SupportStateRow {
            row_id: "interaction.human_bonus_feat_ability_bonus.pilot_pressure".to_owned(),
            subject_type: MatrixSubjectType::Interaction,
            subject_id: "interaction:human-bonus-feat-ability-bonus".to_owned(),
            dimension: "race/class interaction pressure on the deterministic pilot path".to_owned(),
            support_state: SupportState::Partial,
            evidence_tier: EvidenceTier::Computed,
            grounding_ref:
                "tests/ge06_pilot_input_contract.rs; tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt (carries human_bonus_feat and human_ability_bonus selections)"
                    .to_owned(),
            blocker_or_lossiness_note: None,
            next_required_uplift: "SD13-E2 / SD13-E3 coupling".to_owned(),
        },
        SupportStateRow {
            row_id: "interaction.non_human_any_class.progression_pressure".to_owned(),
            subject_type: MatrixSubjectType::Interaction,
            subject_id: "interaction:non-human-any-class-progression".to_owned(),
            dimension: "race/class interaction pressure beyond the pilot".to_owned(),
            support_state: SupportState::Unverified,
            evidence_tier: EvidenceTier::Observed,
            grounding_ref: SD13_ROSTER_AUTHORITY.to_owned(),
            blocker_or_lossiness_note: None,
            next_required_uplift:
                "add named interaction rows only where separate race and class rows are insufficient"
                    .to_owned(),
        },
    ];

    SupportStateMatrix { rows }
}
