//! SD-20 Epic 3 cycle 1 — General feats RED test
//! (`SD-20-rules-engine-completeness-scope-draft.md` §1.3,
//! `technical-design.md` §2.2).
//!
//! Per §1.3's per-cycle test description: "a user-selected feat that
//! satisfies all prerequisites produces a non-empty `FeatEffects`; a feat
//! that fails a prerequisite produces a non-empty
//! `PrerequisiteEvaluation.failing_prerequisites`."
//!
//! This cycle's feat catalog is the SD-19 foundation-slice
//! `rules_tables::crb::feats` table store, landed at `04c3d08` after the
//! prior cycle's blocker. That table carries `key`/`category`/`name`/
//! `description` per record only — no `PREREQ:`/`PREABILITY:`/`PRELEVEL:`
//! tokens are transcribed (confirmed by reading `feats.rs` and
//! `feat_data/general.rs` directly). So the "prerequisite" this cycle's
//! engine can honestly evaluate is catalog membership in the requested
//! category, not a real per-feat prerequisite chain — see
//! `src/rules_core/feat_prereqs/general.rs`'s own doc comment for the
//! full reasoning.
//!
//! Two real, verbatim catalog entries exercise both branches:
//! - "Toughness" — a real CRB General feat (`feat_data/general.rs` line
//!   with `key: "Toughness"`) — is eligible, no failing prerequisites,
//!   and resolves a non-empty `FeatEffects`.
//! - "Power Attack" — a real CRB feat, but filed under
//!   `FeatCategory::Combat` in the catalog (`feat_data/combat.rs`), not
//!   `FeatCategory::General` — looked up as a General-category selection
//!   it fails eligibility (not a General feat) and resolves no effect.

use codex::rules_core::feat_prereqs::{compute_feat_effects, evaluate_feat_prerequisites, FeatKey};
use codex::rules_core::rules_tables::crb::feats::FeatCategory;

#[test]
fn toughness_is_eligible_with_no_failing_prerequisites() {
    let feat = FeatKey {
        feat_id: "Toughness".to_string(),
        category: FeatCategory::General,
    };

    let evaluation = evaluate_feat_prerequisites(&feat);

    assert!(
        evaluation.is_eligible,
        "Toughness is a real CRB General feat with no prerequisite tokens in the corpus; \
         it must be eligible"
    );
    assert!(
        evaluation.failing_prerequisites.is_empty(),
        "an eligible feat must carry no failing prerequisites, got {:?}",
        evaluation.failing_prerequisites
    );
}

#[test]
fn toughness_produces_a_nonempty_feat_effects() {
    let feat = FeatKey {
        feat_id: "Toughness".to_string(),
        category: FeatCategory::General,
    };

    let effects = compute_feat_effects(&feat);

    assert_eq!(effects.feat_id, "Toughness");
    let description = effects
        .description
        .as_deref()
        .expect("Toughness carries a real DESC: token in the catalog, so effects.description must be Some");
    assert_eq!(description, "You have enhanced physical stamina.");
    let table_cell = effects
        .table_cell
        .as_ref()
        .expect("a resolved feat's effects must carry TableCellRef provenance");
    assert_eq!(table_cell.table, "feats");
    assert_eq!(table_cell.row_key, "Toughness");
}

#[test]
fn power_attack_looked_up_as_general_fails_eligibility() {
    // Power Attack is a real CRB feat, but the catalog files it under
    // `FeatCategory::Combat` (`feat_data/combat.rs`), not `General`. A
    // General-category lookup for it must not fabricate eligibility.
    let feat = FeatKey {
        feat_id: "Power Attack".to_string(),
        category: FeatCategory::General,
    };

    let evaluation = evaluate_feat_prerequisites(&feat);

    assert!(
        !evaluation.is_eligible,
        "Power Attack is not a General-category feat in the catalog; it must not be eligible \
         under a General lookup"
    );
    assert!(
        !evaluation.failing_prerequisites.is_empty(),
        "an ineligible feat must carry a non-empty failing_prerequisites explaining why"
    );
}

#[test]
fn power_attack_looked_up_as_general_resolves_no_effect() {
    let feat = FeatKey {
        feat_id: "Power Attack".to_string(),
        category: FeatCategory::General,
    };

    let effects = compute_feat_effects(&feat);

    assert_eq!(effects.feat_id, "Power Attack");
    assert!(
        effects.description.is_none(),
        "an unresolvable General-category lookup must not fabricate a description"
    );
    assert!(
        effects.table_cell.is_none(),
        "an unresolvable General-category lookup must not fabricate provenance"
    );
}

#[test]
fn an_unknown_feat_id_fails_eligibility_and_resolves_no_effect() {
    let feat = FeatKey {
        feat_id: "Not A Real Feat".to_string(),
        category: FeatCategory::General,
    };

    let evaluation = evaluate_feat_prerequisites(&feat);
    assert!(!evaluation.is_eligible);
    assert!(!evaluation.failing_prerequisites.is_empty());

    let effects = compute_feat_effects(&feat);
    assert!(effects.description.is_none());
    assert!(effects.table_cell.is_none());
}
