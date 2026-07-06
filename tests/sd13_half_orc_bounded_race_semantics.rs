//! SD13-E2 Half-Orc bounded race-semantics classification slice.
//!
//! Bounded, honest classification of the `race.half_orc.bounded_semantics` row in
//! the SD-13 support-state matrix. This slice proves:
//!
//! - the seeded Half-Orc row stays `Unverified` / `Observed` /
//!   `AwaitingInitialEvidence` because the live `pilot_compute.rs` surface grounds
//!   race semantics only for `race:human` and emits a non-claim-blocking
//!   `race.semantics.unverified` diagnostic for every other race identity (including
//!   Half-Orc),
//! - the seeded row's `dimension`, `blocker_or_lossiness_note`, and
//!   `next_required_uplift` carry Half-Orc-specific text (the same row shape that
//!   the SD13-E3-F6 precedent gave the Paladin/Ranger hybrid rows), so the named
//!   Half-Orc semantic burden is legible instead of implicit,
//! - the live compute surface is exercised end-to-end with a deterministic
//!   Half-Orc Fighter level-1 fixture and emits the expected
//!   `race.semantics.unverified` diagnostic without fabricating any race trait or
//!   numeric race-derived value.
//!
//! This slice does NOT move Half-Orc to `Supported`/`Partial`/`Lossy` and does NOT
//! implement Half-Orc race semantics in `pilot_compute.rs`. Both moves would
//! require Half-Orc runtime evidence that does not exist on `origin/develop` and
//! would break the existing matrix test floor. The honest classification here is
//! `Unverified` with an explicit Half-Orc-specific blocker note, exactly as the
//! slice's "leave unverified with explicit blocker" branch allows.

use codex::rules_core::character_input::load_character_input_fixture;
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, SupportStateMatrix, SupportStateRow,
    seeded_sd13_e1_f1_current_truth,
};

const HALF_ORC_ROW_ID: &str = "race.half_orc.bounded_semantics";
const HALF_ORC_SUBJECT_ID: &str = "race:half-orc";

fn matrix() -> SupportStateMatrix {
    seeded_sd13_e1_f1_current_truth()
}

fn row<'a>(matrix: &'a SupportStateMatrix, row_id: &str) -> &'a SupportStateRow {
    matrix
        .row(row_id)
        .unwrap_or_else(|| panic!("expected seeded row '{row_id}'"))
}

/// The Half-Orc row must keep its current `Unverified` / `Observed` /
/// `AwaitingInitialEvidence` posture. The existing matrix test floor
/// (`every_non_human_race_row_is_unverified_and_observed`) and
/// `only_pilot_grounded_rows_rise_above_observed` both rely on this; the slice
/// must not silently promote Half-Orc without runtime evidence.
#[test]
fn half_orc_row_stays_unverified_and_observed_awaiting_initial_evidence() {
    let matrix = matrix();
    let half_orc = row(&matrix, HALF_ORC_ROW_ID);

    assert_eq!(half_orc.subject_id, HALF_ORC_SUBJECT_ID);
    assert_eq!(half_orc.support_state, SupportState::Unverified);
    assert_eq!(half_orc.evidence_tier, EvidenceTier::Observed);
    assert_eq!(
        half_orc.evidence_freshness,
        EvidenceFreshness::AwaitingInitialEvidence,
        "Half-Orc has no live proof surface to refresh from yet"
    );
}

/// Half-Orc must keep its bounded-race-semantics classification; the slice must
/// not silently reclassify it as a hybrid-chassis row or a fighter-chassis row.
#[test]
fn half_orc_row_remains_a_race_row_not_a_class_or_interaction_row() {
    use codex::rules_core::support_state_matrix::MatrixSubjectType;

    let matrix = matrix();
    let half_orc = row(&matrix, HALF_ORC_ROW_ID);
    assert_eq!(half_orc.subject_type, MatrixSubjectType::Race);
}

/// The seeded Half-Orc row must now carry a Half-Orc-specific `dimension`. The
/// pre-slice seed used the generic `"bounded race semantics"` text shared by all
/// six non-Human race rows; this slice replaces it with named Half-Orc semantic
/// shape so the row is legible as a Half-Orc row rather than as the generic
/// race roster placeholder.
#[test]
fn half_orc_row_dimension_names_half_orc_specific_semantics() {
    let matrix = matrix();
    let half_orc = row(&matrix, HALF_ORC_ROW_ID);

    assert!(
        half_orc.dimension.contains("half-orc") || half_orc.dimension.contains("Half-Orc"),
        "Half-Orc dimension must name the half-orc subject, got: {}",
        half_orc.dimension
    );
    // Must not retain the pre-slice generic placeholder shared with every other
    // non-Human race row, otherwise the row is indistinguishable from Dwarf/Elf/
    // Gnome/Half-Elf/Halfling.
    assert_ne!(
        half_orc.dimension, "bounded race semantics",
        "Half-Orc dimension must be Half-Orc-specific, not the generic placeholder"
    );
}

/// The seeded Half-Orc row must now carry a non-empty `blocker_or_lossiness_note`
/// that enumerates the named Half-Orc semantic burden — the same posture pattern
/// the SD13-E3-F6 slice used to give the Paladin/Ranger hybrid rows. Half-Orc is
/// `Unverified` (not `Blocked`), but the row's honest blocker shape is the named
/// enumeration of what is missing, not an empty string.
#[test]
fn half_orc_row_blocker_note_is_non_empty_and_names_missing_burden() {
    let matrix = matrix();
    let half_orc = row(&matrix, HALF_ORC_ROW_ID);

    assert!(
        !half_orc.blocker_or_lossiness_note.is_empty(),
        "Half-Orc row must carry a non-empty blocker_or_lossiness_note enumerating \
         the missing Half-Orc semantic burden (Unverified rows still benefit from \
         an explicit named blocker shape, per the SD13-E3-F6 precedent)"
    );

    let note = half_orc.blocker_or_lossiness_note.to_ascii_lowercase();

    // The note must name at least one Half-Orc-specific semantic family. The exact
    // names below are the canonical Half-Orc PF1 trait burden; the test only
    // requires at least one named burden so the note is Half-Orc-specific.
    let named_burden_tokens = [
        "ability",
        "ferocity",
        "darkvision",
        "weapon familiarity",
        "orc",
        "size",
        "speed",
        "skill",
    ];
    let named_at_least_one = named_burden_tokens
        .iter()
        .any(|token| note.contains(token));
    assert!(
        named_at_least_one,
        "Half-Orc blocker note must name at least one Half-Orc-specific semantic \
         burden (one of: ability / ferocity / darkvision / weapon familiarity / \
         orc / size / speed / skill), got: {}",
        half_orc.blocker_or_lossiness_note
    );
}

/// The seeded Half-Orc row's `next_required_uplift` must name a Half-Orc-specific
/// uplift path. The pre-slice seed used the generic `"SD13-E2 race-semantic slice"`
/// text shared by all six non-Human race rows; this slice replaces it with the
/// Half-Orc-specific uplift.
#[test]
fn half_orc_row_next_uplift_is_half_orc_specific() {
    let matrix = matrix();
    let half_orc = row(&matrix, HALF_ORC_ROW_ID);

    assert_ne!(
        half_orc.next_required_uplift, "SD13-E2 race-semantic slice",
        "Half-Orc next_required_uplift must be Half-Orc-specific, not the generic \
         placeholder shared with every other non-Human race row"
    );
    let uplift = half_orc.next_required_uplift.to_ascii_lowercase();
    assert!(
        uplift.contains("half-orc") || uplift.contains("half orc"),
        "Half-Orc next_required_uplift must name half-orc, got: {}",
        half_orc.next_required_uplift
    );
}

/// Half-Orc has no live proof surface yet, so its `grounding_ref` must remain a
/// roster-only reference. The matrix test floor's freshness invariant
/// (`freshness_tracks_live_proof_grounding_not_optimism`) requires this posture:
/// `AwaitingInitialEvidence` rows may not cite a live proof path.
#[test]
fn half_orc_row_grounding_ref_still_points_to_roster_only() {
    let matrix = matrix();
    let half_orc = row(&matrix, HALF_ORC_ROW_ID);

    assert!(
        !half_orc.grounding_ref.is_empty(),
        "Half-Orc row must cite a grounding reference (the roster doc)"
    );
    assert!(
        half_orc
            .grounding_ref
            .contains("core-roster-and-support-state-matrix"),
        "Half-Orc row has no live proof yet and must remain grounded to the \
         SD-13 roster matrix doc, got: {}",
        half_orc.grounding_ref
    );
}

/// End-to-end runtime proof: loading a deterministic Half-Orc Fighter level-1
/// fixture and running it through `compute_pilot_base_chassis` must produce the
/// `race.semantics.unverified` non-claim-blocking diagnostic and must NOT
/// fabricate any Half-Orc race trait, numeric value, or explanation. This is the
/// only live runtime evidence that Half-Orc is honestly unverified today; the
/// row's `Unverified` classification is grounded by this behavior.
#[test]
fn half_orc_fighter_pilot_emits_race_semantics_unverified_diagnostic() {
    const HALF_ORC_FIGHTER_FIXTURE: &str = include_str!(
        "fixtures/rules_core/pf1_half_orc_fighter_level1_sd13_deterministic_input.txt"
    );

    let loaded = load_character_input_fixture(HALF_ORC_FIGHTER_FIXTURE);
    let input = loaded.character_input.unwrap_or_else(|| {
        panic!(
            "Half-Orc Fighter fixture must load cleanly, got diagnostics: {:?}",
            loaded.diagnostics
        )
    });
    assert_eq!(
        input.chosen.race_id, HALF_ORC_SUBJECT_ID,
        "fixture must carry the half-orc race identity"
    );

    let computation = compute_pilot_base_chassis(&input);

    // The race semantics must be marked unverified exactly once for Half-Orc and
    // must NOT be marked claim-blocking: the Half-Orc loadout still produces a
    // computed pilot receipt because the rest of the deterministic path (Fighter
    // level 1, ability modifiers, base saves, total saves, deterministic combat
    // baseline) is grounded; only the race semantics are unverified.
    let race_unverified_diagnostics: Vec<_> = computation
        .diagnostics
        .iter()
        .filter(|d| d.id == "race.semantics.unverified")
        .collect();
    assert_eq!(
        race_unverified_diagnostics.len(),
        1,
        "expected exactly one race.semantics.unverified diagnostic for Half-Orc, \
         got {} (all diagnostics: {:?})",
        race_unverified_diagnostics.len(),
        computation.diagnostics
    );
    let diag = race_unverified_diagnostics[0];
    assert!(
        !diag.claim_blocking,
        "race.semantics.unverified must remain non-claim-blocking so the deterministic \
         Half-Orc Fighter pilot still reports a Computed receipt; got claim_blocking=true"
    );
    assert!(
        diag.message.contains("race:half-orc"),
        "diagnostic message must name the unverified half-orc identity, got: {}",
        diag.message
    );
    assert!(
        diag.message.contains("race:human"),
        "diagnostic message must name the only grounded race identity (race:human), \
         got: {}",
        diag.message
    );
}

/// The Half-Orc Fighter pilot compute path must NOT emit a `race.human.*`
/// explanation record: Half-Orc has no Human ability-bonus or Human bonus-feat
/// pressure, and the compute surface must not fabricate one. This pins the
/// honest non-fabrication boundary for the slice.
#[test]
fn half_orc_fighter_pilot_does_not_emit_human_race_explanations() {
    const HALF_ORC_FIGHTER_FIXTURE: &str = include_str!(
        "fixtures/rules_core/pf1_half_orc_fighter_level1_sd13_deterministic_input.txt"
    );

    let loaded = load_character_input_fixture(HALF_ORC_FIGHTER_FIXTURE);
    let input = loaded.character_input.expect("fixture must load cleanly");

    let computation = compute_pilot_base_chassis(&input);

    for explanation in &computation.explanations {
        assert!(
            !explanation.id.starts_with("race.human."),
            "Half-Orc pilot must not emit Human race explanations; got id={} \
             (Half-Orc has no Human ability-bonus or bonus-feat pressure)",
            explanation.id
        );
    }
}

/// Half-Orc Fighter level-1 produces a `Computed` pilot receipt: the bounded
/// non-race seam (Fighter chassis, ability modifiers, saves, deterministic
/// combat baseline) is still grounded. The race-semantics gap is a
/// non-claim-blocking diagnostic only; the receipt's status remains Computed.
#[test]
fn half_orc_fighter_pilot_receipt_remains_computed_with_only_race_gap() {
    use codex::rules_core::pilot_compute::{build_pilot_headless_receipt, HeadlessReceiptStatus};

    const HALF_ORC_FIGHTER_FIXTURE: &str = include_str!(
        "fixtures/rules_core/pf1_half_orc_fighter_level1_sd13_deterministic_input.txt"
    );

    let loaded = load_character_input_fixture(HALF_ORC_FIGHTER_FIXTURE);
    let input = loaded.character_input.expect("fixture must load cleanly");

    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Computed,
        "Half-Orc Fighter level-1 deterministic pilot must remain Computed; \
         the race.semantics.unverified diagnostic is intentionally non-claim-blocking"
    );

    // Belt-and-suspenders: no diagnostic on the Half-Orc path may be
    // claim-blocking, otherwise the slice has fabricated a Half-Orc blocking
    // posture that is not part of the honest Unverified-with-explicit-blocker
    // shape this slice is meant to prove.
    for diag in &receipt.computation.diagnostics {
        assert!(
            !diag.claim_blocking,
            "no Half-Orc path diagnostic may be claim-blocking in this slice; \
             the row is Unverified, not Blocked. Offender: id={} message={}",
            diag.id, diag.message
        );
    }
}

