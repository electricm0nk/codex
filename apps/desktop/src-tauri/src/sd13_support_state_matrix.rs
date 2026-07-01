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

use serde::Serialize;

use codex::rules_core::support_state_matrix::{
    seeded_sd13_e1_f1_current_truth, EvidenceTier, MatrixSubjectType, SupportState, SupportStateRow,
};

/// A single presentation row derived read-only from one seeded SD-13 matrix row.
///
/// Every field mirrors upstream truth; nothing here is recomputed or promoted.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sd13SupportStateRow {
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
pub struct Sd13SupportStateMatrixSnapshot {
    /// One row per seeded SD-13 matrix row, in upstream order, unfiltered.
    pub rows: Vec<Sd13SupportStateRow>,
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

fn present_row(row: &SupportStateRow) -> Sd13SupportStateRow {
    Sd13SupportStateRow {
        row_id: row.row_id.to_string(),
        subject_type: subject_type_token(row.subject_type).to_string(),
        subject_id: row.subject_id.to_string(),
        dimension: row.dimension.to_string(),
        support_state: support_state_token(row.support_state).to_string(),
        evidence_tier: evidence_tier_token(row.evidence_tier).to_string(),
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
pub fn load_sd13_support_state_matrix_snapshot() -> Sd13SupportStateMatrixSnapshot {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let rows = matrix.rows.iter().map(present_row).collect();

    Sd13SupportStateMatrixSnapshot {
        rows,
        data_source: "rules_core::support_state_matrix::seeded_sd13_e1_f1_current_truth"
            .to_string(),
        note: "Read-only SD-13 support-state and debt truth presented verbatim for the SD-11 \
               tester workbench: no filtering, promotion, demotion, or recomputation."
            .to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot() -> Sd13SupportStateMatrixSnapshot {
        load_sd13_support_state_matrix_snapshot()
    }

    fn row<'a>(
        snapshot: &'a Sd13SupportStateMatrixSnapshot,
        row_id: &str,
    ) -> &'a Sd13SupportStateRow {
        snapshot
            .rows
            .iter()
            .find(|r| r.row_id == row_id)
            .unwrap_or_else(|| panic!("expected presented row '{row_id}'"))
    }

    #[test]
    fn snapshot_preserves_all_twenty_one_rows_unfiltered() {
        let snapshot = snapshot();
        assert_eq!(
            snapshot.rows.len(),
            21,
            "snapshot must preserve exactly 21 rows with no filtering, got {}",
            snapshot.rows.len()
        );
    }

    #[test]
    fn human_pilot_row_remains_partial_and_computed() {
        let snapshot = snapshot();
        let human = row(&snapshot, "race.human.pilot_semantics");
        assert_eq!(human.subject_type, "race");
        assert_eq!(human.subject_id, "race:human");
        assert_eq!(human.support_state, "partial");
        assert_eq!(human.evidence_tier, "computed");
    }

    #[test]
    fn fighter_levels_2_10_row_remains_blocked_and_computed() {
        let snapshot = snapshot();
        let blocked = row(&snapshot, "class.fighter.levels_2_10");
        assert_eq!(blocked.subject_type, "class");
        assert_eq!(blocked.subject_id, "class:fighter");
        assert_eq!(blocked.support_state, "blocked");
        assert_eq!(blocked.evidence_tier, "computed");
    }

    #[test]
    fn every_blocked_row_preserves_a_non_empty_blocker_note() {
        let snapshot = snapshot();
        let blocked: Vec<&Sd13SupportStateRow> = snapshot
            .rows
            .iter()
            .filter(|r| r.support_state == "blocked")
            .collect();
        assert!(
            !blocked.is_empty(),
            "at least one blocked row must exist in the seeded truth"
        );
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
    fn canonical_wording_matches_the_contract_for_partial_blocked_and_unverified() {
        let snapshot = snapshot();

        let partial = row(&snapshot, "race.human.pilot_semantics");
        assert_eq!(
            partial.tester_facing_state_label,
            "Partially supported in the current bounded roster slice; some progression or \
             semantic obligations remain explicitly limited."
        );

        let blocked = row(&snapshot, "class.fighter.levels_2_10");
        assert_eq!(
            blocked.tester_facing_state_label,
            "Blocked by known missing semantics in the current bounded roster slice."
        );

        let unverified = row(&snapshot, "race.dwarf.bounded_semantics");
        assert_eq!(
            unverified.tester_facing_state_label,
            "Included in the bounded roadmap scope, but not yet verified for this support level."
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
        let allowed_subjects = ["race", "class", "interaction"];
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
}
