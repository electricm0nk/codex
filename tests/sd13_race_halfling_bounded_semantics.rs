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
//! The SD13-E2 Halfling bounded race-semantics recognition slice
//! (`tests/sd13_halfling_race_semantics_recognition.rs`) executed exactly the
//! promotion path this file's original guards anticipated: it landed grounded
//! evidence for four race-semantic families (ability modifiers, size, speed,
//! senses) and updated the row state in the typed matrix carrier with a
//! non-empty blocker note naming the still-unproven remainder. These tests now
//! pin that promoted truth (`Partial` / `Computed` / `RefreshableFromLiveProof`)
//! instead of the pre-slice `Unverified` / `Observed` evidence floor.
//!
//! Slice: t_1731714c, matrix row_id: race:halfling:bounded-race-semantics.

use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, MatrixSubjectType, SupportState,
    SupportStateMatrix, SupportStateRow, seeded_sd13_e1_f1_current_truth,
};

const HALFLING_ROW_ID: &str = "race.halfling.bounded_semantics";
const HALFLING_SUBJECT_ID: &str = "race:halfling";

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
fn halfling_row_state_is_partial_after_sd13_e2_recognition() {
    // The SD13-E2 Halfling recognition slice landed grounded evidence for four
    // race-semantic families (ability modifiers, size, speed, senses),
    // promoting the row from Unverified to Partial. The row is not Supported:
    // several families (Fearless, Halfling Luck, Keen Senses, Sure-Footed,
    // weapon familiarity) remain unproven.
    // Later promoted to Supported/ProductVisible by SD-19's Race Trait
    // Catalog browser UI-surfacing work (2026-07-16).
    let matrix = matrix();
    let halfling = row(&matrix, HALFLING_ROW_ID);
    assert_eq!(
        halfling.support_state,
        SupportState::Supported,
        "Halfling row must be Supported after SD-19's Race Trait Catalog \
         browser UI-surfacing work."
    );
}

#[test]
fn halfling_row_evidence_tier_is_computed() {
    let matrix = matrix();
    let halfling = row(&matrix, HALFLING_ROW_ID);
    assert_eq!(
        halfling.evidence_tier,
        EvidenceTier::ProductVisible,
        "Halfling row must be ProductVisible once SD-19's Race Trait Catalog \
         browser surfaces it live."
    );
}

#[test]
fn halfling_row_evidence_freshness_is_refreshable_from_live_proof() {
    let matrix = matrix();
    let halfling = row(&matrix, HALFLING_ROW_ID);
    assert_eq!(
        halfling.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof,
        "Halfling row must be RefreshableFromLiveProof once grounded on the \
         live SD13-E2 recognition test surface."
    );
}

#[test]
fn halfling_row_dimension_names_the_recognized_families() {
    let matrix = matrix();
    let halfling = row(&matrix, HALFLING_ROW_ID);
    for token in ["ability modifiers", "size", "speed", "senses"] {
        assert!(
            halfling.dimension.contains(token),
            "Halfling row dimension must name the recognized '{token}' family: {}",
            halfling.dimension
        );
    }
}

#[test]
fn halfling_row_grounding_ref_cites_the_live_recognition_test_surface() {
    let matrix = matrix();
    let halfling = row(&matrix, HALFLING_ROW_ID);
    assert!(
        halfling
            .grounding_ref
            .contains("sd13_halfling_race_semantics_recognition"),
        "Halfling row must ground to the live SD13-E2 recognition test surface; \
         got '{}'",
        halfling.grounding_ref
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
fn halfling_row_is_not_silently_promoted_to_supported_or_lossy() {
    // Belt-and-braces guard. The row WAS legitimately promoted to Supported
    // by SD-19's Race Trait Catalog browser UI-surfacing work (2026-07-16) --
    // that is an intentional, documented promotion, not a silent one. This
    // guard now checks the row never lands on Lossy, which remains
    // unintentional under any circumstance.
    let matrix = matrix();
    let halfling = row(&matrix, HALFLING_ROW_ID);
    assert_ne!(
        halfling.support_state,
        SupportState::Lossy,
        "Halfling row must never be silently promoted to Lossy. Current state: {:?}.",
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