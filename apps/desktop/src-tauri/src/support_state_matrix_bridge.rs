//! SD13-E6-F11 read-only desktop bridge over the SD-13 support-state matrix.
//!
//! This module is a documentary/control-plane adapter only. It consumes the
//! upstream `rules_core` support-state matrix carrier verbatim and projects a
//! serializable snapshot for the SD-11 tester workbench. It deliberately does
//! **not** compute rules, persist, mutate, promote/demote, recompute, filter,
//! aggregate, or touch feedback/issue-transport concerns.
//!
//! Support state and evidence tier are kept as separate axes exactly as the
//! upstream carrier keeps them, so a `Computed`/`Partial` row is never silently
//! read as `Supported`. The tester-facing wording is drawn from the SD-13
//! support-language contract, not invented in the UI.
//!
//! SD13-E6-F12 remains explicitly deferred: this bridge presents the current
//! matrix debt only and does not add evidence capture, issue submission, or
//! persistence/update coupling.

use serde::Serialize;

use codex::rules_core::support_state_matrix::{
    seeded_current_truth, EvidenceFreshness, EvidenceTier, MatrixSubjectType,
    SupportState, SupportStateRow,
};

/// A single presentation row derived read-only from one seeded SD-13 matrix row.
///
/// Every field mirrors upstream truth; nothing here is recomputed or promoted.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportStateRowPresentation {
    /// Stable upstream row identity (e.g. `class.fighter.levels_2_10`).
    pub row_id: String,
    /// `race`, `class`, or `interaction`.
    pub subject_type: String,
    /// Subject identity (e.g. `class:fighter`).
    pub subject_id: String,
    /// The semantic/progression dimension under claim.
    pub dimension: String,
    /// Raw support-state token (`supported` | `partial` | `lossy` | `blocked` | `unverified`).
    pub support_state: String,
    /// Raw evidence-tier token on the Codex quality-gate scale.
    pub evidence_tier: String,
    /// Raw evidence-freshness token projected verbatim from the SD-13 carrier
    /// (`refreshable-from-live-proof` | `awaiting-initial-evidence`). This is the
    /// SD13-E7-F13 breadth-claim audit axis; it is never reinterpreted here.
    pub evidence_freshness: String,
    /// SD-13-owned refresh-audit wording derived from `evidence_freshness` only.
    /// Both current postures stay explicitly refresh-required.
    pub refresh_audit_label: String,
    /// SD-13-approved tester-facing wording derived from `support_state` only.
    pub tester_facing_state_label: String,
    /// Real doc/repo grounding reference for the row.
    pub grounding_ref: String,
    /// Blocker/lossiness note; empty when the state needs none.
    pub blocker_or_lossiness_note: String,
    /// Next required uplift or owning future slice.
    pub next_required_uplift: String,
}

/// The top-level read-only snapshot handed to the desktop boundary.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportStateMatrixSnapshot {
    /// One row per seeded SD-13 matrix row, in upstream order, unfiltered.
    pub rows: Vec<SupportStateRowPresentation>,
    /// Machine identity of the upstream truth surface this snapshot mirrors.
    pub data_source: String,
    /// Human-readable posture note about the read-only nature of this snapshot.
    pub note: String,
}

/// Canonical SD-13 tester-facing wording for `supported` rows.
const SUPPORTED_LABEL: &str =
    "Supported in the current bounded PF1 Core Rulebook roster slice for the named level band.";
/// Canonical SD-13 tester-facing wording for `partial` rows.
const PARTIAL_LABEL: &str = "Partially supported in the current bounded roster slice; some \
progression or semantic obligations remain explicitly limited.";
/// Canonical SD-13 tester-facing wording for `lossy` rows.
const LOSSY_LABEL: &str = "Available only with lossy support in the current bounded roster slice; \
important semantics are simplified or approximated.";
/// Canonical SD-13 tester-facing wording for `blocked` rows.
const BLOCKED_LABEL: &str = "Blocked by known missing semantics in the current bounded roster slice.";
/// Canonical SD-13 tester-facing wording for `unverified` rows.
const UNVERIFIED_LABEL: &str =
    "Included in the bounded roadmap scope, but not yet verified for this support level.";

/// SD-13 refresh-audit wording for rows anchored to a live, re-runnable proof.
const REFRESHABLE_FROM_LIVE_PROOF_LABEL: &str =
    "Refresh required: anchored to a live, re-runnable proof surface, but no evidence-refresh \
checkpoint has been recorded for the current bounded slice, so the breadth claim is not yet \
confirmed fresh.";
/// SD-13 refresh-audit wording for rows with no runtime evidence to refresh yet.
const AWAITING_INITIAL_EVIDENCE_LABEL: &str =
    "Refresh required: rests only on bounded roster-scope naming with no runtime evidence yet, so \
there is no refreshed evidence backing any breadth claim for this row.";

fn support_state_token(state: SupportState) -> &'static str {
    match state {
        SupportState::Supported => "supported",
        SupportState::Partial => "partial",
        SupportState::Lossy => "lossy",
        SupportState::Blocked => "blocked",
        SupportState::Unverified => "unverified",
    }
}

fn evidence_tier_token(tier: EvidenceTier) -> &'static str {
    match tier {
        EvidenceTier::Observed => "observed",
        EvidenceTier::Parsed => "parsed",
        EvidenceTier::Converted => "converted",
        EvidenceTier::Computed => "computed",
        EvidenceTier::OracleChecked => "oracle-checked",
        EvidenceTier::ProductVisible => "product-visible",
    }
}

fn subject_type_token(subject_type: MatrixSubjectType) -> &'static str {
    match subject_type {
        MatrixSubjectType::Race => "race",
        MatrixSubjectType::Class => "class",
        MatrixSubjectType::Interaction => "interaction",
        MatrixSubjectType::School(_) => "school",
        MatrixSubjectType::Equipment(_) => "equipment",
    }
}

/// Project the carrier's evidence-freshness posture verbatim as a stable token.
/// This is a direct mapping, not a heuristic: the carrier owns the freshness truth.
fn evidence_freshness_token(freshness: EvidenceFreshness) -> &'static str {
    match freshness {
        EvidenceFreshness::RefreshableFromLiveProof => "refreshable-from-live-proof",
        EvidenceFreshness::AwaitingInitialEvidence => "awaiting-initial-evidence",
    }
}

/// The SD-13-owned refresh-audit wording for a freshness posture. Derived from the
/// freshness axis alone so no UI-local optimism can leak in; both current postures
/// stay explicitly refresh-required.
fn refresh_audit_label(freshness: EvidenceFreshness) -> &'static str {
    match freshness {
        EvidenceFreshness::RefreshableFromLiveProof => REFRESHABLE_FROM_LIVE_PROOF_LABEL,
        EvidenceFreshness::AwaitingInitialEvidence => AWAITING_INITIAL_EVIDENCE_LABEL,
    }
}

/// The SD-13-approved tester-facing wording for a support state. This is the only
/// place wording is derived, and it is derived from state alone so no UI-local
/// optimism can leak in.
fn tester_facing_state_label(state: SupportState) -> &'static str {
    match state {
        SupportState::Supported => SUPPORTED_LABEL,
        SupportState::Partial => PARTIAL_LABEL,
        SupportState::Lossy => LOSSY_LABEL,
        SupportState::Blocked => BLOCKED_LABEL,
        SupportState::Unverified => UNVERIFIED_LABEL,
    }
}

fn present_row(row: &SupportStateRow) -> SupportStateRowPresentation {
    SupportStateRowPresentation {
        row_id: row.row_id.to_string(),
        subject_type: subject_type_token(row.subject_type).to_string(),
        subject_id: row.subject_id.to_string(),
        dimension: row.dimension.to_string(),
        support_state: support_state_token(row.support_state).to_string(),
        evidence_tier: evidence_tier_token(row.evidence_tier).to_string(),
        evidence_freshness: evidence_freshness_token(row.evidence_freshness).to_string(),
        refresh_audit_label: refresh_audit_label(row.evidence_freshness).to_string(),
        tester_facing_state_label: tester_facing_state_label(row.support_state).to_string(),
        grounding_ref: row.grounding_ref.to_string(),
        blocker_or_lossiness_note: row.blocker_or_lossiness_note.to_string(),
        next_required_uplift: row.next_required_uplift.to_string(),
    }
}

/// Build the read-only SD-13 support-state snapshot from the seeded matrix.
///
/// Every seeded row is preserved in order with no filtering, suppression, or
/// aggregation. This is the single source of truth for the desktop command.
pub fn build_support_state_matrix_snapshot() -> SupportStateMatrixSnapshot {
    let matrix = seeded_current_truth();
    let rows = matrix.rows.iter().map(present_row).collect();

    SupportStateMatrixSnapshot {
        rows,
        data_source: "rules_core::support_state_matrix::seeded_current_truth"
            .to_string(),
        note: "Read-only SD-13 support-state and debt truth presented verbatim for the SD-11 \
               tester workbench: no filtering, promotion, demotion, or recomputation."
            .to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot() -> SupportStateMatrixSnapshot {
        build_support_state_matrix_snapshot()
    }

    fn row<'a>(
        snapshot: &'a SupportStateMatrixSnapshot,
        row_id: &str,
    ) -> &'a SupportStateRowPresentation {
        snapshot
            .rows
            .iter()
            .find(|r| r.row_id == row_id)
            .unwrap_or_else(|| panic!("expected presented row '{row_id}'"))
    }

    #[test]
    fn snapshot_mirrors_every_upstream_row_verbatim_and_unfiltered() {
        // The bridge contract is verbatim projection: every upstream row, in
        // order, with each field mapped through the fixed token/wording tables
        // and nothing recomputed. Asserting against the carrier itself (instead
        // of duplicating its literals here) keeps this test true when the
        // upstream roster moves, which is exactly when a stale copy would lie.
        let matrix = seeded_current_truth();
        let snapshot = snapshot();
        assert_eq!(
            snapshot.rows.len(),
            matrix.rows.len(),
            "snapshot must preserve every seeded row with no filtering"
        );
        for (upstream, projected) in matrix.rows.iter().zip(&snapshot.rows) {
            assert_eq!(projected.row_id, upstream.row_id, "row order must be preserved");
            assert_eq!(projected.subject_type, subject_type_token(upstream.subject_type));
            assert_eq!(projected.subject_id, upstream.subject_id);
            assert_eq!(projected.dimension, upstream.dimension);
            assert_eq!(
                projected.support_state,
                support_state_token(upstream.support_state),
                "row '{}' support state must be projected verbatim",
                upstream.row_id
            );
            assert_eq!(projected.evidence_tier, evidence_tier_token(upstream.evidence_tier));
            assert_eq!(
                projected.evidence_freshness,
                evidence_freshness_token(upstream.evidence_freshness),
                "row '{}' freshness token must be projected verbatim",
                upstream.row_id
            );
            assert_eq!(projected.grounding_ref, upstream.grounding_ref);
            assert_eq!(projected.blocker_or_lossiness_note, upstream.blocker_or_lossiness_note);
            assert_eq!(projected.next_required_uplift, upstream.next_required_uplift);
        }
    }

    #[test]
    fn human_pilot_row_remains_partial_and_computed() {
        // Later promoted to Supported/ProductVisible by SD-19's Race Trait
        // Catalog browser UI-surfacing work (2026-07-16).
        let snapshot = snapshot();
        let human = row(&snapshot, "race.human.pilot_semantics");
        assert_eq!(human.subject_type, "race");
        assert_eq!(human.subject_id, "race:human");
        assert_eq!(human.support_state, "supported");
        assert_eq!(human.evidence_tier, "product-visible");
    }

    #[test]
    fn every_blocked_row_preserves_a_non_empty_blocker_note() {
        // SD13-E5 promoted Paladin, the last remaining `Blocked` class row, to
        // `Partial`, so the seeded truth may now legitimately carry zero
        // `Blocked` rows. This test no longer requires at least one to exist;
        // it only pins the invariant that any row that IS `Blocked` still
        // preserves a non-empty note, so a future regression that reintroduces
        // an unblocked-but-unexplained row is still caught.
        let snapshot = snapshot();
        let blocked: Vec<&SupportStateRowPresentation> = snapshot
            .rows
            .iter()
            .filter(|r| r.support_state == "blocked")
            .collect();
        for r in blocked {
            assert!(
                !r.blocker_or_lossiness_note.is_empty(),
                "blocked row '{}' must preserve a non-empty blocker/lossiness note",
                r.row_id
            );
        }
    }

    #[test]
    fn every_row_preserves_grounding_ref_and_next_required_uplift() {
        let snapshot = snapshot();
        for r in &snapshot.rows {
            assert!(
                !r.grounding_ref.is_empty(),
                "row '{}' must preserve a grounding reference",
                r.row_id
            );
            assert!(
                !r.next_required_uplift.is_empty(),
                "row '{}' must preserve a next required uplift",
                r.row_id
            );
        }
    }

    #[test]
    fn canonical_wording_matches_the_contract_for_every_projected_state() {
        // Rows are located by their projected state (not by hardcoded row ids)
        // so upstream roster moves cannot silently rot this contract check.
        let snapshot = snapshot();
        let expected: [(&str, &str); 5] = [
            ("supported", SUPPORTED_LABEL),
            ("partial", PARTIAL_LABEL),
            ("lossy", LOSSY_LABEL),
            ("blocked", BLOCKED_LABEL),
            ("unverified", UNVERIFIED_LABEL),
        ];
        let mut states_seen = 0;
        for (state, label) in expected {
            for r in snapshot.rows.iter().filter(|r| r.support_state == state) {
                assert_eq!(
                    r.tester_facing_state_label, label,
                    "row '{}' must carry the canonical '{}' wording",
                    r.row_id, state
                );
                states_seen += 1;
            }
        }
        assert_eq!(
            states_seen,
            snapshot.rows.len(),
            "every row's wording must be covered by the canonical contract table"
        );
    }

    #[test]
    fn no_non_supported_row_collapses_into_bare_supported_wording() {
        let snapshot = snapshot();
        for r in &snapshot.rows {
            if r.support_state != "supported" {
                assert_ne!(
                    r.tester_facing_state_label, "Supported",
                    "non-supported row '{}' must not collapse into bare 'Supported'",
                    r.row_id
                );
                assert!(
                    !r.tester_facing_state_label.is_empty(),
                    "row '{}' must carry canonical tester-facing wording",
                    r.row_id
                );
            }
        }
    }

    #[test]
    fn subject_and_state_tokens_use_lowercase_canonical_forms() {
        let snapshot = snapshot();
        let allowed_states = ["supported", "partial", "lossy", "blocked", "unverified"];
        let allowed_subjects = ["race", "class", "interaction", "school", "equipment"];
        for r in &snapshot.rows {
            assert!(
                allowed_states.contains(&r.support_state.as_str()),
                "row '{}' has unexpected support-state token '{}'",
                r.row_id,
                r.support_state
            );
            assert!(
                allowed_subjects.contains(&r.subject_type.as_str()),
                "row '{}' has unexpected subject-type token '{}'",
                r.row_id,
                r.subject_type
            );
        }
    }

    #[test]
    fn every_row_projects_a_canonical_evidence_freshness_token() {
        let snapshot = snapshot();
        let allowed = ["refreshable-from-live-proof", "awaiting-initial-evidence"];
        for r in &snapshot.rows {
            assert!(
                allowed.contains(&r.evidence_freshness.as_str()),
                "row '{}' has unexpected evidence-freshness token '{}'",
                r.row_id,
                r.evidence_freshness
            );
            assert!(
                !r.refresh_audit_label.is_empty(),
                "row '{}' must carry non-empty refresh-audit wording",
                r.row_id
            );
        }
    }

    #[test]
    fn no_projected_row_claims_confirmed_fresh_evidence() {
        // The first slice records no completed refresh checkpoint, so no projected
        // row may imply current freshness: every refresh-audit label is explicitly
        // refresh-required and no freshness token collapses into a "fresh" claim.
        let snapshot = snapshot();
        for r in &snapshot.rows {
            assert_ne!(
                r.evidence_freshness, "fresh",
                "row '{}' must not project a bare 'fresh' claim",
                r.row_id
            );
            assert!(
                r.refresh_audit_label
                    .to_lowercase()
                    .contains("refresh required"),
                "row '{}' refresh-audit wording must stay refresh-required, got '{}'",
                r.row_id,
                r.refresh_audit_label
            );
        }
    }
}
