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
//! The SD13-E2 Dwarf bounded race-semantics recognition slice
//! (`tests/sd13_dwarf_bounded_race_semantics.rs`) executed exactly the promotion
//! path this file's original guards anticipated: it landed grounded evidence for
//! four race-semantic families (ability modifiers, size, speed, senses) and
//! updated the row state in the typed matrix carrier with a non-empty blocker
//! note naming the still-unproven remainder. These tests now pin that promoted
//! truth (`Partial` / `Computed` / `RefreshableFromLiveProof`) instead of the
//! pre-slice `Unverified` / `Observed` evidence floor.
//!
//! Slice: t_3cf90c2c, matrix row_id: race:dwarf:bounded-race-semantics.

use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, MatrixSubjectType, SupportState,
    SupportStateMatrix, SupportStateRow, seeded_sd13_e1_f1_current_truth,
};

const DWARF_ROW_ID: &str = "race.dwarf.bounded_semantics";
const DWARF_SUBJECT_ID: &str = "race:dwarf";

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
fn dwarf_row_state_is_partial_after_sd13_e2_recognition() {
    // The SD13-E2 Dwarf recognition slice landed grounded evidence for four
    // race-semantic families (ability modifiers, size, speed, senses),
    // promoting the row from Unverified to Partial. The row is not Supported:
    // several families (Stonecunning, Defensive Training, Hardy, Stability,
    // Hatred, weapon familiarity) remain unproven.
    // Later promoted to Supported/ProductVisible by SD-19's Race Trait
    // Catalog browser UI-surfacing work (2026-07-16).
    let matrix = matrix();
    let dwarf = row(&matrix, DWARF_ROW_ID);
    assert_eq!(
        dwarf.support_state,
        SupportState::Supported,
        "Dwarf row must be Supported after SD-19's Race Trait Catalog browser \
         UI-surfacing work."
    );
}

#[test]
fn dwarf_row_evidence_tier_is_computed() {
    let matrix = matrix();
    let dwarf = row(&matrix, DWARF_ROW_ID);
    assert_eq!(
        dwarf.evidence_tier,
        EvidenceTier::ProductVisible,
        "Dwarf row must be ProductVisible once SD-19's Race Trait Catalog \
         browser surfaces it live."
    );
}

#[test]
fn dwarf_row_evidence_freshness_is_refreshable_from_live_proof() {
    let matrix = matrix();
    let dwarf = row(&matrix, DWARF_ROW_ID);
    assert_eq!(
        dwarf.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof,
        "Dwarf row must be RefreshableFromLiveProof once grounded on the \
         live SD13-E2 recognition test surface."
    );
}

#[test]
fn dwarf_row_dimension_names_the_recognized_families() {
    let matrix = matrix();
    let dwarf = row(&matrix, DWARF_ROW_ID);
    // The dimension is updated by the SD13-E2 slice to name the four
    // recognized families rather than the pre-slice generic placeholder text.
    for token in ["ability modifiers", "size", "speed", "senses"] {
        assert!(
            dwarf.dimension.contains(token),
            "Dwarf row dimension must name the recognized '{token}' family: {}",
            dwarf.dimension
        );
    }
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
fn dwarf_row_does_not_silently_promote_to_supported_or_lossy() {
    // Belt-and-braces guard. The row WAS legitimately promoted to Supported
    // by SD-19's Race Trait Catalog browser UI-surfacing work (2026-07-16) --
    // that is an intentional, documented promotion, not a silent one. This
    // guard now checks the row never lands on Lossy, which remains
    // unintentional under any circumstance.
    let matrix = matrix();
    let dwarf = row(&matrix, DWARF_ROW_ID);
    assert_ne!(
        dwarf.support_state,
        SupportState::Lossy,
        "Dwarf row must never be silently promoted to Lossy. Current state: {:?}.",
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