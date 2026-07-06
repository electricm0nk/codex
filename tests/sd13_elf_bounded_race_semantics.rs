//! SD13-Elf bounded race-semantics classification test.
//!
//! Pin the honest classification of the Elf race-semantics row at the
//! evidence floor on 2026-07-06. The repo contains no direct runtime
//! evidence for any Elf semantic family: the only Elf surface is the
//! row carrier in `src/rules_core/support_state_matrix.rs` plus the row
//! names in the markdown matrix and the visibility ledger. The
//! `pilot_compute` seam explicitly gates every non-Human race out of
//! the compute path with `if input.chosen.race_id != HUMAN_RACE_ID`.
//!
//! These tests must stay green until a later bounded slice lands
//! grounded evidence for at least one of the seven required
//! race-semantic families (identity/provenance, ability modifier,
//! size/speed/movement, senses, bonus feats/skill/derived-stat
//! modifiers, prerequisite/feat/class-feature interactions, other core
//! racial traits) AND upgrades the row state in the typed matrix
//! carrier with a non-empty blocker note. Promotion of the row above
//! `Unverified` without that grounded evidence is a counterfeit breadth
//! claim and must be rejected by these tests.
//!
//! Slice: t_37dbab62, matrix row_id: race.elf.bounded_semantics.

use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, MatrixSubjectType, SupportState, SupportStateMatrix,
    SupportStateRow, seeded_sd13_e1_f1_current_truth,
};

const ELF_ROW_ID: &str = "race.elf.bounded_semantics";
const ELF_SUBJECT_ID: &str = "race:elf";
const CLASSIFICATION_ARTIFACT_PATH: &str =
    "programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-elf-bounded-race-semantics-classification-2026-07-06.md";

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
fn elf_row_state_is_unverified_at_evidence_floor() {
    // The honest verdict on 2026-07-06 is Unverified/Observed. Promotion
    // above Unverified is counterfeit breadth until a later slice lands
    // grounded evidence for at least one of the seven required
    // race-semantic families and updates the row accordingly.
    let matrix = matrix();
    let elf = row(&matrix, ELF_ROW_ID);
    assert_eq!(
        elf.support_state,
        SupportState::Unverified,
        "Elf row must stay Unverified at the 2026-07-06 evidence floor; \
         promotion is counterfeit breadth until a later slice lands grounded evidence"
    );
    assert_eq!(
        elf.evidence_tier,
        EvidenceTier::Observed,
        "Elf row evidence tier must stay Observed until runtime evidence exists"
    );
    assert_eq!(
        elf.evidence_freshness,
        EvidenceFreshness::AwaitingInitialEvidence,
        "Elf row has no runtime evidence to refresh; must await initial evidence"
    );
}

#[test]
fn elf_row_grounding_stays_on_the_roster_authority_only() {
    // The Elf row has no live compute or test grounding on 2026-07-06, so
    // it must keep grounding only on the SD-13 roster authority. If a
    // later slice grounds Elf on the compute seam or a focused proof
    // surface, that slice must upgrade the grounding_ref explicitly.
    let matrix = matrix();
    let elf = row(&matrix, ELF_ROW_ID);
    assert!(
        elf.grounding_ref.contains("core-roster-and-support-state-matrix"),
        "Elf row must ground to the SD-13 roster authority only (no runtime \
         compute or proof surface exists yet); got '{}'",
        elf.grounding_ref
    );
}

#[test]
fn elf_row_dimension_stays_stable_as_bounded_race_semantics() {
    // The slice keeps the dimension label stable so every race row reads
    // as the same dimension family ("bounded race semantics"). The
    // per-row blocker note and next-uplift carry the bounded scope.
    let matrix = matrix();
    let elf = row(&matrix, ELF_ROW_ID);
    assert_eq!(
        elf.dimension, "bounded race semantics",
        "Elf row dimension must stay 'bounded race semantics' so all race rows \
         share the same dimension label; per-row scope lives in the blocker note"
    );
}

#[test]
fn elf_row_carries_non_empty_blocker_note_naming_required_families() {
    // The slice's actual lift: turn the previously-empty
    // blocker_or_lossiness_note into a non-empty note naming the seven
    // required race-semantic families and their unproven status, so the
    // audit surface shows exactly what would have to be grounded before
    // Elf can move out of Unverified.
    let matrix = matrix();
    let elf = row(&matrix, ELF_ROW_ID);
    assert!(
        !elf.blocker_or_lossiness_note.is_empty(),
        "Elf row blocker/lossiness note must be non-empty after the slice; \
         an empty note would silently hide the missing-semantic-family debt"
    );
    // The blocker note must name enough of the seven required families
    // that the audit surface can show the family-level gap. The slice's
    // note covers all seven.
    let families_or_dwarf_anchors = [
        "ability",          // ability-score modifiers family
        "size",             // size, speed, movement baseline family
        "movement",         // size, speed, movement baseline family
        "vision",           // senses family (low-light vision for Elf)
        "sleep",            // Elf immunity to sleep trait
        "weapon",           // weapon familiarity trait (Elf)
        "trait",            // other core racial traits family
    ];
    for needle in families_or_dwarf_anchors {
        assert!(
            elf.blocker_or_lossiness_note.contains(needle),
            "Elf row blocker/lossiness note must name the '{needle}' family \
             or trait anchor; got '{}'",
            elf.blocker_or_lossiness_note
        );
    }
}

#[test]
fn elf_row_next_uplift_points_at_this_artifact() {
    let matrix = matrix();
    let elf = row(&matrix, ELF_ROW_ID);
    assert!(
        elf.next_required_uplift.contains(CLASSIFICATION_ARTIFACT_PATH),
        "Elf row next_required_uplift must point at the bounded Elf \
         classification artifact at '{CLASSIFICATION_ARTIFACT_PATH}'; got '{}'",
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
fn elf_row_is_not_silently_promoted_to_supported_partial_lossy_or_blocked() {
    // The first-slice contract is honest: even with a non-empty blocker
    // note naming the seven required families, the row does NOT silently
    // move into any stronger posture. Promotion requires a later slice
    // with grounded compute or proof evidence — exactly what this slice
    // does not produce.
    let matrix = matrix();
    let elf = row(&matrix, ELF_ROW_ID);
    assert_ne!(elf.support_state, SupportState::Supported);
    assert_ne!(elf.support_state, SupportState::Partial);
    assert_ne!(elf.support_state, SupportState::Lossy);
    assert_ne!(elf.support_state, SupportState::Blocked);
    assert_eq!(elf.support_state, SupportState::Unverified);
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