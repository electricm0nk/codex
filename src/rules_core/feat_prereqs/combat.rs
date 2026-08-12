//! SD-20 Epic 3, second feat category (`scope-draft.md` §1.3 work-unit
//! order): CRB `Combat` feats.
//!
//! Reads `rules_tables::crb::feats::feat_tables()` directly (SD-19's
//! foundation-slice catalog, landed at `04c3d08`) — never hand-rolled,
//! per this cycle's own brief. Mirrors `feat_prereqs/general.rs` exactly,
//! one category over.
//!
//! **Bounded posture, and why.** Identical to `general.rs`'s reasoning:
//! `FeatTableEntry` (the landed catalog) carries `key` / `category` /
//! `name` / `description` per record only — confirmed by reading
//! `feats.rs` and `feat_data/combat.rs` directly. No `PREREQ:` /
//! `PREABILITY:` / `PRELEVEL:` / `PREFEAT:` token is transcribed into the
//! table store, even though several real Combat feats have one on the raw
//! corpus (`core_rulebook/cr_feats.lst`): e.g. `Improved Critical` and
//! `Weapon Focus`-chain feats gate on BAB and prior feats in the published
//! CRB, none of which the landed catalog carries. This epic's file-touch
//! partition forbids editing `rules_tables/crb/feats.rs` or `feat_data/`
//! (read-only for Epic 3), and this cycle's brief directs reading feat
//! records via the `feats.rs` catalog, not hand-rolling data — so a real
//! per-feat prerequisite chain cannot be evaluated from data this engine
//! can query without either violating the read-only boundary or
//! fabricating a value the catalog doesn't carry.
//!
//! So this cycle's evaluation is honestly bounded to what the catalog
//! *does* prove: whether the requested feat id is a real `Combat`-category
//! record. A feat id absent from the `Combat` slice of `feat_tables()`
//! fails eligibility ("not found"); a feat id present has no evaluable
//! failing prerequisite recorded. Widening this to a real per-feat
//! prerequisite chain (BAB gates, feat-chain gates) requires the SD-19
//! table store to carry `PREREQ:`-family tokens per record, which is a
//! foundation-slice change outside this cycle's scope.

use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::rules_tables::crb::feats::{feat_tables, FeatCategory};

/// Result of a bounded, catalog-membership-only prerequisite check for a
/// `Combat`-category feat lookup. See this module's doc comment for the
/// scope this is bounded to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CombatFeatPrerequisiteEvaluation {
    pub is_eligible: bool,
    pub failing_prerequisites: Vec<String>,
}

/// Evaluates whether `feat_id` is eligible under a `Combat`-category
/// lookup. Eligible iff a `FeatCategory::Combat` record in the landed
/// catalog matches `feat_id` by `key` or `name` (`FeatTableEntry.key`
/// falls back to `name` when the corpus record carries no `KEY:` token,
/// per `feats.rs`'s own doc comment — matching on either field mirrors
/// that fallback rather than re-deriving it).
pub fn evaluate_combat_feat_prerequisites(feat_id: &str) -> CombatFeatPrerequisiteEvaluation {
    let found = feat_tables()
        .iter()
        .any(|entry| entry.category == FeatCategory::Combat && matches_feat_id(entry, feat_id));

    if found {
        CombatFeatPrerequisiteEvaluation {
            is_eligible: true,
            failing_prerequisites: Vec::new(),
        }
    } else {
        CombatFeatPrerequisiteEvaluation {
            is_eligible: false,
            failing_prerequisites: vec![format!(
                "'{feat_id}' is not a recognized CRB Combat feat in the catalog \
                 (rules_tables::crb::feats::feat_tables(), FeatCategory::Combat)"
            )],
        }
    }
}

/// One resolved `Combat`-category feat's effect: its catalog id,
/// description text, and `TableCellRef` provenance, all read directly
/// off the matched `FeatTableEntry` — never hand-rolled. See this
/// module's doc comment for why this stays bounded to description text
/// rather than a numeric derived-stat delta.
#[derive(Debug, Clone, PartialEq)]
pub struct CombatFeatEffect {
    pub feat_id: String,
    pub description: String,
    pub table_cell: TableCellRef,
}

/// Resolves `feat_id` against the `Combat` slice of the canonical CRB
/// feat catalog. Returns `None` when `feat_id` is not a real
/// `Combat`-category record in the table store — SD-19 owns the table
/// store; this function reads it, it never fabricates an entry for a key
/// the table store doesn't have (mirrors
/// `feat_prereqs::general::resolve_general_feat_effect`'s own `None`-on-miss
/// discipline).
pub fn resolve_combat_feat_effect(feat_id: &str) -> Option<CombatFeatEffect> {
    let entry = feat_tables()
        .iter()
        .find(|entry| entry.category == FeatCategory::Combat && matches_feat_id(entry, feat_id))?;

    Some(CombatFeatEffect {
        feat_id: entry.key.to_string(),
        description: entry.description.unwrap_or_default().to_string(),
        table_cell: TableCellRef {
            rule_set: RuleSetId::Crb,
            table: "feats".to_string(),
            row_key: entry.key.to_string(),
            column_key: String::new(),
        },
    })
}

fn matches_feat_id(
    entry: &crate::rules_core::rules_tables::crb::feats::FeatTableEntry,
    feat_id: &str,
) -> bool {
    entry.key == feat_id || entry.name == feat_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_a_real_combat_feat() {
        let effect = resolve_combat_feat_effect("Power Attack")
            .expect("Power Attack is a real Combat feat in the landed catalog");
        assert_eq!(effect.feat_id, "Power Attack");
        assert_eq!(
            effect.description,
            "You can make exceptionally deadly melee attacks by sacrificing accuracy for strength."
        );
        assert_eq!(effect.table_cell.table, "feats");
        assert_eq!(effect.table_cell.row_key, "Power Attack");
    }

    #[test]
    fn rejects_a_feat_from_a_different_category() {
        // Toughness is real, but filed under FeatCategory::General.
        assert!(resolve_combat_feat_effect("Toughness").is_none());
        let evaluation = evaluate_combat_feat_prerequisites("Toughness");
        assert!(!evaluation.is_eligible);
        assert!(!evaluation.failing_prerequisites.is_empty());
    }

    #[test]
    fn rejects_an_unknown_feat_id() {
        assert!(resolve_combat_feat_effect("Definitely Not A Feat").is_none());
        let evaluation = evaluate_combat_feat_prerequisites("Definitely Not A Feat");
        assert!(!evaluation.is_eligible);
        assert!(!evaluation.failing_prerequisites.is_empty());
    }
}
