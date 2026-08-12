//! SD-20 Epic 3 cycle 2 — Combat feats RED test
//! (`SD-20-rules-engine-completeness-scope-draft.md` §1.3,
//! `technical-design.md` §2.2).
//!
//! Second Epic-3 work-unit per `scope-draft.md` §1.3's cycle order ("one
//! feat category per cycle... general feats" first, `Combat` next). Mirrors
//! `tests/sd20_feat_general.rs` exactly, one category over: this cycle's
//! feat catalog slice is `rules_tables::crb::feats::feat_tables()`'s
//! `FeatCategory::Combat` records (110 total, landed at `04c3d08`), and the
//! same bounded catalog-membership-only prerequisite posture applies (no
//! `PREREQ:`/`PREABILITY:`/`PRELEVEL:` tokens are transcribed into the
//! table store for Combat feats either) — see
//! `src/rules_core/feat_prereqs/combat.rs`'s own doc comment for the full
//! reasoning, which is identical to `feat_prereqs/general.rs`'s.
//!
//! Two real, verbatim catalog entries exercise both branches:
//! - "Power Attack" — a real CRB Combat feat (`feat_data/combat.rs`, `key:
//!   "Power Attack"`) — is eligible, no failing prerequisites, and
//!   resolves a non-empty `FeatEffects`.
//! - "Toughness" — a real CRB feat, but filed under `FeatCategory::General`
//!   in the catalog (`feat_data/general.rs`), not `FeatCategory::Combat` —
//!   looked up as a Combat-category selection it fails eligibility (not a
//!   Combat feat) and resolves no effect.

use codex::rules_core::feat_prereqs::{compute_feat_effects, evaluate_feat_prerequisites, FeatKey};
use codex::rules_core::rules_tables::crb::feats::FeatCategory;

#[test]
fn power_attack_is_eligible_with_no_failing_prerequisites() {
    let feat = FeatKey {
        feat_id: "Power Attack".to_string(),
        category: FeatCategory::Combat,
    };

    let evaluation = evaluate_feat_prerequisites(&feat);

    assert!(
        evaluation.is_eligible,
        "Power Attack is a real CRB Combat feat carried in the catalog; it must be eligible"
    );
    assert!(
        evaluation.failing_prerequisites.is_empty(),
        "an eligible feat must carry no failing prerequisites, got {:?}",
        evaluation.failing_prerequisites
    );
}

#[test]
fn power_attack_produces_a_nonempty_feat_effects() {
    let feat = FeatKey {
        feat_id: "Power Attack".to_string(),
        category: FeatCategory::Combat,
    };

    let effects = compute_feat_effects(&feat);

    assert_eq!(effects.feat_id, "Power Attack");
    let description = effects.description.as_deref().expect(
        "Power Attack carries a real DESC: token in the catalog, so effects.description must be Some",
    );
    assert_eq!(
        description,
        "You can make exceptionally deadly melee attacks by sacrificing accuracy for strength."
    );
    let table_cell = effects
        .table_cell
        .as_ref()
        .expect("a resolved feat's effects must carry TableCellRef provenance");
    assert_eq!(table_cell.table, "feats");
    assert_eq!(table_cell.row_key, "Power Attack");
}

#[test]
fn toughness_looked_up_as_combat_fails_eligibility() {
    // Toughness is real, but filed under FeatCategory::General.
    let feat = FeatKey {
        feat_id: "Toughness".to_string(),
        category: FeatCategory::Combat,
    };

    let evaluation = evaluate_feat_prerequisites(&feat);

    assert!(
        !evaluation.is_eligible,
        "Toughness is not a Combat-category feat in the catalog; it must not be eligible under \
         a Combat lookup"
    );
    assert!(
        !evaluation.failing_prerequisites.is_empty(),
        "an ineligible feat must carry a non-empty failing_prerequisites explaining why"
    );
}

#[test]
fn toughness_looked_up_as_combat_resolves_no_effect() {
    let feat = FeatKey {
        feat_id: "Toughness".to_string(),
        category: FeatCategory::Combat,
    };

    let effects = compute_feat_effects(&feat);

    assert_eq!(effects.feat_id, "Toughness");
    assert!(
        effects.description.is_none(),
        "an unresolvable Combat-category lookup must not fabricate a description"
    );
    assert!(
        effects.table_cell.is_none(),
        "an unresolvable Combat-category lookup must not fabricate provenance"
    );
}

#[test]
fn an_unknown_feat_id_fails_eligibility_and_resolves_no_effect() {
    let feat = FeatKey {
        feat_id: "Not A Real Feat".to_string(),
        category: FeatCategory::Combat,
    };

    let evaluation = evaluate_feat_prerequisites(&feat);
    assert!(!evaluation.is_eligible);
    assert!(!evaluation.failing_prerequisites.is_empty());

    let effects = compute_feat_effects(&feat);
    assert!(effects.description.is_none());
    assert!(effects.table_cell.is_none());
}
