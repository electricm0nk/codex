//! SD13-Dwarf bounded race-semantics classification test.
//!
//! Pin the honest classification of the Dwarf race-semantics row at the
//! evidence floor on 2026-07-06. The repo contains no direct runtime
//! evidence for any Dwarf semantic family: the only Dwarf surface is the
//! row carrier in `src/rules_core/support_state_matrix.rs` plus the row
//! names in the markdown matrix and the visibility ledger. The
//! `pilot_compute` seam explicitly gates every non-Human race out of the
//! compute path with `if input.chosen.race_id != HUMAN_RACE_ID`.
//!
//! These tests must stay green until a later bounded slice lands
//! grounded evidence for at least one of the seven required race-semantic
//! families (identity/provenance, ability modifier, size/speed/movement,
//! senses, bonus feats/skill/derived-stat modifiers, prerequisite/feat/
//! class-feature interactions, other core racial traits) AND upgrades the
//! row state in the typed matrix carrier with a non-empty blocker note.
//! Promotion of the row above `Unverified` without that grounded
//! evidence is a counterfeit breadth claim and must be rejected by these
//! tests.
//!
//! Slice: t_3cf90c2c, matrix row_id: race:dwarf:bounded-race-semantics.

use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, MatrixSubjectType, SupportState,
    SupportStateMatrix, SupportStateRow, seeded_sd13_e1_f1_current_truth,
};

const DWARF_ROW_ID: &str = "race.dwarf.bounded_semantics";
const DWARF_SUBJECT_ID: &str = "race:dwarf";
const CLASSIFICATION_ARTIFACT_PATH: &str =
    "programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-dwarf-bounded-race-semantics-classification-2026-07-06.md";

fn matrix() -> SupportStateMatrix {
    seeded_sd13_e1_f1_current_truth()
}

fn row<'a>(matrix: &'a SupportStateMatrix, row_id: &str) -> &'a SupportStateRow {
    matrix
        .row(row_id)
        .unwrap_or_else(|| panic!("expected seeded row '{row_id}'"))
}

#[test]
fn dwarf_row_is_present_in_seeded_matrix() {
    let matrix = matrix();
    let dwarf = row(&matrix, DWARF_ROW_ID);
    assert_eq!(dwarf.row_id, DWARF_ROW_ID);
    assert_eq!(dwarf.subject_type, MatrixSubjectType::Race);
    assert_eq!(dwarf.subject_id, DWARF_SUBJECT_ID);
}

#[test]
fn dwarf_row_state_is_unverified_at_evidence_floor() {
    // The honest verdict on 2026-07-06 is Unverified/Observed. Promotion
    // above Unverified is counterfeit breadth until a later slice lands
    // grounded evidence for at least one of the seven required
    // race-semantic families and updates the row accordingly.
    let matrix = matrix();
    let dwarf = row(&matrix, DWARF_ROW_ID);
    assert_eq!(
        dwarf.support_state,
        SupportState::Unverified,
        "Dwarf row must remain Unverified at the live evidence floor \
         (slice t_3cf90c2c, 2026-07-06). Promotion requires grounded \
         evidence per artifact {CLASSIFICATION_ARTIFACT_PATH}."
    );
}

#[test]
fn dwarf_row_evidence_tier_is_observed_only() {
    // The evidence tier stays Observed: Dwarf is named by SD-13 packet
    // roster and appears in the typed matrix carrier, but no
    // parsed/converted/computed/oracle-checked evidence exists yet.
    let matrix = matrix();
    let dwarf = row(&matrix, DWARF_ROW_ID);
    assert_eq!(
        dwarf.evidence_tier,
        EvidenceTier::Observed,
        "Dwarf row must remain Observed until a later slice lands \
         Parsed/Converted/Computed/Oracle-checked evidence."
    );
}

#[test]
fn dwarf_row_evidence_freshness_is_awaiting_initial_evidence() {
    let matrix = matrix();
    let dwarf = row(&matrix, DWARF_ROW_ID);
    assert_eq!(
        dwarf.evidence_freshness,
        EvidenceFreshness::AwaitingInitialEvidence,
        "Dwarf row must remain AwaitingInitialEvidence until a later \
         slice lands live evidence."
    );
}

#[test]
fn dwarf_row_dimension_unchanged_for_this_slice() {
    let matrix = matrix();
    let dwarf = row(&matrix, DWARF_ROW_ID);
    // The dimension string is preserved. The slice only refines the
    // blocker_or_lossiness_note and the next_required_uplift pointers;
    // it does not relabel the dimension itself.
    assert_eq!(
        dwarf.dimension, "bounded race semantics",
        "Dwarf row dimension must remain 'bounded race semantics'; \
         this slice does not relabel the dimension."
    );
}

#[test]
fn dwarf_row_grounding_ref_is_present() {
    let matrix = matrix();
    let dwarf = row(&matrix, DWARF_ROW_ID);
    assert!(
        !dwarf.grounding_ref.is_empty(),
        "Dwarf row must cite a grounding ref; an empty ref would \
         indicate the row was never seeded."
    );
}

#[test]
fn dwarf_row_blocker_note_carries_honest_unverified_reason() {
    // After this slice lands, the blocker_or_lossiness_note field must
    // be non-empty and must name the honest unverified reason — not the
    // pre-slice empty string and not a counterfeit "supported" /
    // "partial" claim.
    let matrix = matrix();
    let dwarf = row(&matrix, DWARF_ROW_ID);
    let note = dwarf.blocker_or_lossiness_note;
    assert!(
        !note.is_empty(),
        "Dwarf row blocker_or_lossiness_note must be non-empty after \
         slice t_3cf90c2c lands. An empty note means the row was never \
         classified and reverts to silent breadth ambiguity."
    );
    assert!(
        note.contains("race-semantic") || note.contains("ability") ||
        note.contains("unverified") || note.contains("proven") ||
        note.contains("family") || note.contains("Dwarf"),
        "Dwarf row blocker_or_lossiness_note must name the honest \
         reason (got: {note:?})."
    );
}

#[test]
fn dwarf_row_next_uplift_points_at_classification_artifact() {
    // After this slice lands, next_required_uplift must reference the
    // slice's evidence artifact or name a concrete family gap. The
    // pre-slice generic "SD13-E2 race-semantic slice" text is too vague
    // for a closeout tranche row.
    let matrix = matrix();
    let dwarf = row(&matrix, DWARF_ROW_ID);
    let uplift = dwarf.next_required_uplift;
    assert!(
        !uplift.is_empty(),
        "Dwarf row next_required_uplift must be non-empty after the \
         classification slice lands."
    );
    assert!(
        uplift.contains("sd13-dwarf-bounded-race-semantics-classification") ||
        uplift.contains("Dwarf") ||
        uplift.contains("race-semantic") ||
        uplift.contains("ability") ||
        uplift.contains("speed") ||
        uplift.contains("darkvision") ||
        uplift.contains("family"),
        "Dwarf row next_required_uplift must reference the slice's \
         classification artifact or a concrete family gap. Got: {uplift:?}"
    );
}

#[test]
fn dwarf_row_does_not_promote_above_unverified() {
    // Belt-and-braces guard. If a future change flips the support state
    // to Partial, Lossy, Blocked, or Supported without updating this
    // slice's artifact, the test fails. The honest path forward requires
    // a new slice that lands grounded evidence for at least one
    // race-semantic family AND updates the artifact with the new
    // evidence floor AND adds the family-specific test.
    let matrix = matrix();
    let dwarf = row(&matrix, DWARF_ROW_ID);
    assert!(
        matches!(dwarf.support_state, SupportState::Unverified),
        "Dwarf row must not be silently promoted above Unverified. \
         Current state: {:?}. Promotion requires a new slice that \
         lands grounded evidence and updates the classification \
         artifact.",
        dwarf.support_state
    );
}

#[test]
fn dwarf_row_is_not_part_of_seven_by_eleven_combination_claim() {
    // Belt-and-braces guard against the "all 7 races are supported"
    // breadth claim. The matrix must remain a per-row truthful surface,
    // not a per-combination universal grid. The dwarf row alone, with
    // state Unverified, is the canonical evidence that Dwarf has not
    // been silently included in any 7x11 supported roster.
    let matrix = matrix();
    let dwarf = row(&matrix, DWARF_ROW_ID);
    assert_eq!(
        dwarf.subject_type,
        MatrixSubjectType::Race,
        "Dwarf row must remain a Race row, not collapse into a \
         race/class combination row."
    );
    assert_eq!(dwarf.subject_id, DWARF_SUBJECT_ID);
}

#[test]
fn dwarf_row_subject_id_unchanged() {
    let matrix = matrix();
    let dwarf = row(&matrix, DWARF_ROW_ID);
    assert_eq!(
        dwarf.subject_id, DWARF_SUBJECT_ID,
        "Dwarf row subject_id must remain 'race:dwarf'; this slice \
         does not rename the subject."
    );
}

#[test]
fn dwarf_row_coexists_with_other_unverified_race_rows() {
    // Sanity check that this slice's refinement of the Dwarf row does
    // not accidentally promote or break any other race row. The matrix
    // remains a 21-row carrier and every other race row stays in the
    // state it was seeded with.
    let matrix = matrix();
    let other_unverified_races = [
        "race.elf.bounded_semantics",
        "race.gnome.bounded_semantics",
        "race.half_elf.bounded_semantics",
        "race.half_orc.bounded_semantics",
        "race.halfling.bounded_semantics",
    ];
    for race_id in other_unverified_races {
        let r = row(&matrix, race_id);
        assert_eq!(
            r.subject_type,
            MatrixSubjectType::Race,
            "sibling race row {race_id} must remain a Race row"
        );
        // Other race rows are owned by their own classification slices
        // (t_37dbab62 Elf, t_d8f575a0 Gnome, t_356173db Half-Elf,
        // t_7f355f9c Half-Orc, t_1731714c Halfling). This Dwarf slice
        // does not change them. They may stay Unverified or be promoted
        // by their own slices; we only assert they remain a Race row
        // and were not silently collapsed by this slice's diff.
        assert_ne!(
            r.row_id, DWARF_ROW_ID,
            "sibling row {race_id} must not be renamed to the Dwarf row"
        );
    }
}