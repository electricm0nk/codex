//! SD13-Elf bounded race-semantics classification test.
//!
//! Originally pinned the honest classification of the Elf race-semantics row
//! at the 2026-07-06 evidence floor, when the repo contained no direct runtime
//! evidence for any Elf semantic family.
//!
//! The SD13-E2 Elf bounded race-semantics recognition slice
//! (`tests/sd13_elf_race_semantics_recognition.rs`) executed exactly the
//! promotion path this file's original guards anticipated: it landed grounded
//! evidence for four race-semantic families (ability modifiers, size, speed,
//! senses) and updated the row state in the typed matrix carrier with a
//! non-empty blocker note naming the still-unproven remainder. These tests now
//! pin that promoted truth (`Partial` / `Computed` / `RefreshableFromLiveProof`)
//! instead of the pre-slice `Unverified` / `Observed` evidence floor.
//!
//! Slice: t_37dbab62, matrix row_id: race.elf.bounded_semantics.

use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, MatrixSubjectType, SupportState, SupportStateMatrix,
    SupportStateRow, seeded_sd13_e1_f1_current_truth,
};

const ELF_ROW_ID: &str = "race.elf.bounded_semantics";
const ELF_SUBJECT_ID: &str = "race:elf";

fn matrix() -> SupportStateMatrix {
    seeded_sd13_e1_f1_current_truth()
}

fn row<'a>(matrix: &'a SupportStateMatrix, row_id: &str) -> &'a SupportStateRow {
    matrix
        .row(row_id)
        .unwrap_or_else(|| panic!("expected seeded row '{row_id}'"))
}

#[test]
fn elf_row_is_present_in_seeded_matrix() {
    let matrix = matrix();
    let elf = row(&matrix, ELF_ROW_ID);
    assert_eq!(elf.row_id, ELF_ROW_ID);
    assert_eq!(elf.subject_type, MatrixSubjectType::Race);
    assert_eq!(elf.subject_id, ELF_SUBJECT_ID);
}

#[test]
fn elf_row_state_is_partial_after_sd13_e2_recognition() {
    // The SD13-E2 Elf recognition slice landed grounded evidence for four
    // race-semantic families (ability modifiers, size, speed, senses),
    // promoting the row from Unverified to Partial. The row is not Supported:
    // several families (Elven Immunities, Keen Senses, weapon familiarity,
    // bonus languages) remain unproven.
    let matrix = matrix();
    let elf = row(&matrix, ELF_ROW_ID);
    assert_eq!(
        elf.support_state,
        SupportState::Partial,
        "Elf row must be Partial after the SD13-E2 recognition slice lands \
         grounded evidence for its four named families."
    );
    assert_eq!(
        elf.evidence_tier,
        EvidenceTier::Computed,
        "Elf row must be Computed once the SD13-E2 recognition slice lands \
         direct runtime evidence."
    );
    assert_eq!(
        elf.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof,
        "Elf row must be RefreshableFromLiveProof once grounded on the live \
         SD13-E2 recognition test surface."
    );
}

#[test]
fn elf_row_grounding_cites_the_live_recognition_test_surface() {
    // The SD13-E2 recognition slice upgrades the grounding_ref from the
    // roster authority to the live, re-runnable proof surface.
    let matrix = matrix();
    let elf = row(&matrix, ELF_ROW_ID);
    assert!(
        elf.grounding_ref
            .contains("sd13_elf_race_semantics_recognition"),
        "Elf row must ground to the live SD13-E2 recognition test surface; \
         got '{}'",
        elf.grounding_ref
    );
}

#[test]
fn elf_row_dimension_names_the_recognized_families() {
    // The SD13-E2 slice updates the dimension to name the four recognized
    // families rather than the pre-slice generic placeholder text.
    let matrix = matrix();
    let elf = row(&matrix, ELF_ROW_ID);
    for token in ["ability modifiers", "size", "speed", "senses"] {
        assert!(
            elf.dimension.contains(token),
            "Elf row dimension must name the recognized '{token}' family: {}",
            elf.dimension
        );
    }
}

#[test]
fn elf_row_carries_non_empty_blocker_note_naming_recognized_and_remaining_families() {
    // The SD13-E2 slice's actual lift: the blocker_or_lossiness_note now names
    // both the four recognized families (with their grounded values) and the
    // remaining unproven families, so the audit surface shows exactly what
    // still needs grounding.
    let matrix = matrix();
    let elf = row(&matrix, ELF_ROW_ID);
    assert!(
        !elf.blocker_or_lossiness_note.is_empty(),
        "Elf row blocker/lossiness note must be non-empty after the slice; \
         an empty note would silently hide the missing-semantic-family debt"
    );
    let anchors = [
        "ability",          // recognized: ability modifiers
        "size",             // recognized: size
        "speed",            // recognized: speed
        "senses",           // recognized: senses
        "Elven Immunities", // remaining: sleep immunity, enchantment saves
        "Keen Senses",      // remaining: Perception bonus
        "weapon",           // remaining: weapon familiarity
        "bonus language",   // remaining: bonus languages
    ];
    for needle in anchors {
        assert!(
            elf.blocker_or_lossiness_note.contains(needle),
            "Elf row blocker/lossiness note must name the '{needle}' anchor; \
             got '{}'",
            elf.blocker_or_lossiness_note
        );
    }
    // The note must not carry the pre-PR-#95 claim that pilot_compute gates
    // every non-Human race out via a bare Human-id check: the race seam is now
    // the `explain_race_seam` dispatcher with a dedicated Half-Elf arm.
    assert!(
        !elf.blocker_or_lossiness_note
            .contains("gates every non-Human race out of the compute path"),
        "Elf row note must not describe the retired pre-dispatcher race gate; got '{}'",
        elf.blocker_or_lossiness_note
    );
}

#[test]
fn elf_row_next_uplift_names_a_concrete_remaining_family() {
    let matrix = matrix();
    let elf = row(&matrix, ELF_ROW_ID);
    assert!(
        !elf.next_required_uplift.is_empty(),
        "Elf row next_required_uplift must be non-empty after the slice."
    );
    assert!(
        elf.next_required_uplift.contains("Elf"),
        "Elf row next_required_uplift must reference a concrete remaining \
         Elf family. Got: {}",
        elf.next_required_uplift
    );
}

#[test]
fn elf_row_subject_type_and_id_remain_intact() {
    let matrix = matrix();
    let elf = row(&matrix, ELF_ROW_ID);
    assert_eq!(elf.subject_type, MatrixSubjectType::Race);
    assert_eq!(elf.subject_id, "race:elf");
}

#[test]
fn elf_row_is_not_silently_promoted_to_supported_or_lossy() {
    // Belt-and-braces guard, updated for the SD13-E2 promotion to Partial. If a
    // future change flips the support state to Supported or Lossy without
    // grounding the remaining Elf families (Elven Immunities, Keen Senses,
    // weapon familiarity, bonus languages) as real computed contributions, the
    // test fails.
    let matrix = matrix();
    let elf = row(&matrix, ELF_ROW_ID);
    assert!(
        !matches!(
            elf.support_state,
            SupportState::Supported | SupportState::Lossy
        ),
        "Elf row must not be silently promoted to Supported or Lossy without \
         grounding the remaining unproven families. Current state: {:?}.",
        elf.support_state
    );
}

#[test]
fn elf_row_does_not_collude_with_human_race_seam() {
    // The Human race-semantics slice is the only race the compute seam
    // grounds today. The Elf slice must never claim or piggy-back on
    // the Human seam: a separate subject_id, a separate row_id, and
    // the Human row's own support_state must remain untouched.
    let matrix = matrix();
    let elf = row(&matrix, ELF_ROW_ID);
    let human = row(&matrix, "race.human.pilot_semantics");
    assert_ne!(elf.row_id, human.row_id);
    assert_ne!(elf.subject_id, human.subject_id);
    assert_eq!(
        human.support_state,
        SupportState::Partial,
        "the accepted Human race-semantics slice must stay Partial/Computed \
         after the Elf slice; this slice must not roll back the accepted Human seam"
    );
}

#[test]
fn slice_does_not_change_any_non_elf_row() {
    // The first-slice contract is honest and bounded: only the Elf row's
    // blocker/lossiness note and next_required_uplift move; every other
    // seeded row stays byte-identical. This guards against accidental
    // cross-race widening during the slice.
    let matrix = matrix();
    for other in matrix.rows.iter().filter(|r| r.row_id != ELF_ROW_ID) {
        assert_ne!(
            other.subject_id, "race:elf",
            "non-Elf row '{}' must not silently inherit race:elf subject identity",
            other.row_id
        );
        assert!(
            matches!(
                other.subject_type,
                MatrixSubjectType::Race | MatrixSubjectType::Class | MatrixSubjectType::Interaction
            ),
            "row '{}' has unexpected subject type {:?}",
            other.row_id,
            other.subject_type
        );
    }
}
