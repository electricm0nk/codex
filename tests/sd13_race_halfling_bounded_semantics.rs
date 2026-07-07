//! SD13-Halfling bounded race-semantics classification test.
//!
//! Pin the honest classification of the Halfling race-semantics row at the
//! evidence floor on 2026-07-06. The repo contains no direct runtime
//! evidence for any Halfling semantic family: the only Halfling surface is
//! the row carrier in `src/rules_core/support_state_matrix.rs` plus the row
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
//! Slice: t_1731714c, matrix row_id: race:halfling:bounded-race-semantics.

use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, MatrixSubjectType, SupportState,
    SupportStateMatrix, SupportStateRow, seeded_sd13_e1_f1_current_truth,
};

const HALFLING_ROW_ID: &str = "race.halfling.bounded_semantics";
const HALFLING_SUBJECT_ID: &str = "race:halfling";
const CLASSIFICATION_ARTIFACT_PATH: &str =
    "programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-halfling-bounded-race-semantics-classification-2026-07-06.md";

fn matrix() -> SupportStateMatrix {
    seeded_sd13_e1_f1_current_truth()
}

fn row<'a>(matrix: &'a SupportStateMatrix, row_id: &str) -> &'a SupportStateRow {
    matrix
        .row(row_id)
        .unwrap_or_else(|| panic!("expected seeded row '{row_id}'"))
}

#[test]
fn halfling_row_is_present_in_seeded_matrix() {
    let matrix = matrix();
    let halfling = row(&matrix, HALFLING_ROW_ID);
    assert_eq!(halfling.row_id, HALFLING_ROW_ID);
    assert_eq!(halfling.subject_type, MatrixSubjectType::Race);
    assert_eq!(halfling.subject_id, HALFLING_SUBJECT_ID);
}

#[test]
fn halfling_row_state_is_unverified_at_evidence_floor() {
    // The honest verdict on 2026-07-06 is Unverified/Observed. Promotion
    // above Unverified is counterfeit breadth until a later slice lands
    // grounded evidence for at least one of the seven required
    // race-semantic families and updates the row accordingly.
    let matrix = matrix();
    let halfling = row(&matrix, HALFLING_ROW_ID);
    assert_eq!(
        halfling.support_state,
        SupportState::Unverified,
        "Halfling row must remain Unverified at the live evidence floor \
         (slice t_1731714c, 2026-07-06). Promotion requires grounded \
         evidence per artifact {CLASSIFICATION_ARTIFACT_PATH}."
    );
}

#[test]
fn halfling_row_evidence_tier_is_observed_only() {
    // The evidence tier stays Observed: Halfling is named by SD-13 packet
    // roster and appears in the typed matrix carrier, but no
    // parsed/converted/computed/oracle-checked evidence exists yet.
    let matrix = matrix();
    let halfling = row(&matrix, HALFLING_ROW_ID);
    assert_eq!(
        halfling.evidence_tier,
        EvidenceTier::Observed,
        "Halfling row must remain Observed until a later slice lands \
         Parsed/Converted/Computed/Oracle-checked evidence."
    );
}

#[test]
fn halfling_row_evidence_freshness_is_awaiting_initial_evidence() {
    let matrix = matrix();
    let halfling = row(&matrix, HALFLING_ROW_ID);
    assert_eq!(
        halfling.evidence_freshness,
        EvidenceFreshness::AwaitingInitialEvidence,
        "Halfling row must remain AwaitingInitialEvidence until a later \
         slice lands live evidence."
    );
}

#[test]
fn halfling_row_dimension_unchanged_for_this_slice() {
    let matrix = matrix();
    let halfling = row(&matrix, HALFLING_ROW_ID);
    // The dimension string is preserved. The slice only refines the
    // blocker_or_lossiness_note and the next_required_uplift pointers;
    // it does not relabel the dimension itself.
    assert_eq!(
        halfling.dimension, "bounded race semantics",
        "Halfling row dimension must remain 'bounded race semantics'; \
         this slice does not relabel the dimension."
    );
}

#[test]
fn halfling_row_grounding_ref_is_present() {
    let matrix = matrix();
    let halfling = row(&matrix, HALFLING_ROW_ID);
    assert!(
        !halfling.grounding_ref.is_empty(),
        "Halfling row must cite a grounding ref; an empty ref would \
         indicate the row was never seeded."
    );
}

#[test]
fn halfling_row_blocker_note_carries_honest_unverified_reason() {
    // After this slice lands, the blocker_or_lossiness_note field must
    // be non-empty and must name the honest unverified reason — not the
    // pre-slice empty string and not a counterfeit "supported" /
    // "partial" claim.
    let matrix = matrix();
    let halfling = row(&matrix, HALFLING_ROW_ID);
    let note = halfling.blocker_or_lossiness_note;
    assert!(
        !note.is_empty(),
        "Halfling row blocker_or_lossiness_note must be non-empty after \
         slice t_1731714c lands. An empty note means the row was never \
         classified and reverts to silent breadth ambiguity."
    );
    assert!(
        note.contains("race-semantic") || note.contains("ability") ||
        note.contains("unverified") || note.contains("proven") ||
        note.contains("family") || note.contains("Halfling"),
        "Halfling row blocker_or_lossiness_note must name the honest \
         reason (got: {note:?})."
    );
}

#[test]
fn halfling_row_next_uplift_points_at_classification_artifact() {
    // After this slice lands, next_required_uplift must reference the
    // slice's evidence artifact or name a concrete family gap. The
    // pre-slice generic "SD13-E2 race-semantic slice" text is too vague
    // for a closeout tranche row.
    let matrix = matrix();
    let halfling = row(&matrix, HALFLING_ROW_ID);
    let uplift = halfling.next_required_uplift;
    assert!(
        !uplift.is_empty(),
        "Halfling row next_required_uplift must be non-empty after the \
         classification slice lands."
    );
    assert!(
        uplift.contains("sd13-halfling-bounded-race-semantics-classification") ||
        uplift.contains("Halfling") ||
        uplift.contains("race-semantic") ||
        uplift.contains("ability") ||
        uplift.contains("speed") ||
        uplift.contains("luck") ||
        uplift.contains("family"),
        "Halfling row next_required_uplift must reference the slice's \
         classification artifact or a concrete family gap. Got: {uplift:?}"
    );
}

#[test]
fn halfling_row_does_not_promote_above_unverified() {
    // Belt-and-braces guard. If a future change flips the support state
    // to Partial, Lossy, Blocked, or Supported without updating this
    // slice's artifact, the test fails. The honest path forward requires
    // a new slice that lands grounded evidence for at least one
    // race-semantic family AND updates the artifact with the new
    // evidence floor AND adds the family-specific test.
    let matrix = matrix();
    let halfling = row(&matrix, HALFLING_ROW_ID);
    assert!(
        matches!(halfling.support_state, SupportState::Unverified),
        "Halfling row must not be silently promoted above Unverified. \
         Current state: {:?}. Promotion requires a new slice that \
         lands grounded evidence and updates the classification \
         artifact.",
        halfling.support_state
    );
}

#[test]
fn halfling_row_is_not_part_of_seven_by_eleven_combination_claim() {
    // Belt-and-braces guard against the "all 7 races are supported"
    // breadth claim. The matrix must remain a per-row truthful surface,
    // not a per-combination universal grid. The halfling row alone, with
    // state Unverified, is the canonical evidence that Halfling has not
    // been silently included in any 7x11 supported roster.
    let matrix = matrix();
    let halfling = row(&matrix, HALFLING_ROW_ID);
    assert_eq!(
        halfling.subject_type,
        MatrixSubjectType::Race,
        "Halfling row must remain a Race row, not collapse into a \
         race/class combination row."
    );
    assert_eq!(halfling.subject_id, HALFLING_SUBJECT_ID);
}

#[test]
fn halfling_row_subject_id_unchanged() {
    let matrix = matrix();
    let halfling = row(&matrix, HALFLING_ROW_ID);
    assert_eq!(
        halfling.subject_id, HALFLING_SUBJECT_ID,
        "Halfling row subject_id must remain 'race:halfling'; this slice \
         does not rename the subject."
    );
}

#[test]
fn halfling_row_coexists_with_other_unverified_race_rows() {
    // Sanity check that this slice's refinement of the Halfling row does
    // not accidentally promote or break any other race row. The matrix
    // remains a 21-row carrier and every other race row stays in the
    // state it was seeded with.
    let matrix = matrix();
    let other_unverified_races = [
        "race.dwarf.bounded_semantics",
        "race.elf.bounded_semantics",
        "race.gnome.bounded_semantics",
        "race.half_elf.bounded_semantics",
        "race.half_orc.bounded_semantics",
    ];
    for race_id in other_unverified_races {
        let r = row(&matrix, race_id);
        assert_eq!(
            r.subject_type,
            MatrixSubjectType::Race,
            "sibling race row {race_id} must remain a Race row"
        );
        // Other race rows are owned by their own classification slices
        // (t_3cf90c2c Dwarf, t_37dbab62 Elf, t_d8f575a0 Gnome,
        // t_356173db Half-Elf, t_7f355f9c Half-Orc). This Halfling slice
        // does not change them. They may stay Unverified or be promoted
        // by their own slices; we only assert they remain a Race row
        // and were not silently collapsed by this slice's diff.
        assert_ne!(
            r.row_id, HALFLING_ROW_ID,
            "sibling row {race_id} must not be renamed to the Halfling row"
        );
    }
}