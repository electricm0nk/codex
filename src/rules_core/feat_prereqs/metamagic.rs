//! SD-20 Epic 3, fourth and FINAL feat category (`scope-draft.md` §1.3
//! work-unit order): CRB `Metamagic` feats. Landing this category closes
//! Epic 3 — every feat category in `rules_tables::crb::feats::feat_tables()`
//! (General, Combat, ItemCreation, Metamagic) now has a landed per-category
//! evaluation module.
//!
//! Reads `rules_tables::crb::feats::feat_tables()` directly (SD-19's
//! foundation-slice catalog, landed at `04c3d08`) — never hand-rolled,
//! per this cycle's own brief. Mirrors `feat_prereqs/general.rs`,
//! `feat_prereqs/combat.rs`, and `feat_prereqs/item_creation.rs` exactly,
//! one category over.
//!
//! **Bounded posture, and why.** Identical to `general.rs`'s, `combat.rs`'s,
//! and `item_creation.rs`'s reasoning: `FeatTableEntry` (the landed catalog)
//! carries `key` / `category` / `name` / `description` per record only —
//! confirmed by reading `feats.rs` and `feat_data/metamagic.rs` directly. No
//! `PREREQ:` / `PREABILITY:` / `PRELEVEL:` token is transcribed into the
//! table store, even though the published CRB gates several Metamagic feats
//! (e.g. the `Heighten Spell +N` chain) on already having `Heighten Spell`.
//! This epic's file-touch partition forbids editing `rules_tables/crb/feats.rs`
//! or `feat_data/` (read-only for Epic 3), and this cycle's brief directs
//! reading feat records via the `feats.rs` catalog, not hand-rolling data —
//! so a real per-feat prerequisite chain cannot be evaluated from data this
//! engine can query without either violating the read-only boundary or
//! fabricating a value the catalog doesn't carry.
//!
//! So this cycle's evaluation is honestly bounded to what the catalog *does*
//! prove: whether the requested feat id is a real `Metamagic`-category
//! record. A feat id absent from the `Metamagic` slice of `feat_tables()`
//! fails eligibility ("not found"); a feat id present has no evaluable
//! failing prerequisite recorded. Widening this to a real per-feat
//! prerequisite chain requires the SD-19 table store to carry
//! `PREREQ:`-family tokens per record, which is a foundation-slice change
//! outside this cycle's scope.

use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::rules_tables::crb::feats::{feat_tables, FeatCategory};

/// Result of a bounded, catalog-membership-only prerequisite check for a
/// `Metamagic`-category feat lookup. See this module's doc comment for the
/// scope this is bounded to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetamagicFeatPrerequisiteEvaluation {
    pub is_eligible: bool,
    pub failing_prerequisites: Vec<String>,
}

/// Evaluates whether `feat_id` is eligible under a `Metamagic`-category
/// lookup. Eligible iff a `FeatCategory::Metamagic` record in the landed
/// catalog matches `feat_id` by `key` or `name` (`FeatTableEntry.key`
/// falls back to `name` when the corpus record carries no `KEY:` token,
/// per `feats.rs`'s own doc comment — matching on either field mirrors
/// that fallback rather than re-deriving it).
pub fn evaluate_metamagic_feat_prerequisites(feat_id: &str) -> MetamagicFeatPrerequisiteEvaluation {
    let found = feat_tables().iter().any(|entry| {
        entry.category == FeatCategory::Metamagic && matches_feat_id(entry, feat_id)
    });

    if found {
        MetamagicFeatPrerequisiteEvaluation {
            is_eligible: true,
            failing_prerequisites: Vec::new(),
        }
    } else {
        MetamagicFeatPrerequisiteEvaluation {
            is_eligible: false,
            failing_prerequisites: vec![format!(
                "'{feat_id}' is not a recognized CRB Metamagic feat in the catalog \
                 (rules_tables::crb::feats::feat_tables(), FeatCategory::Metamagic)"
            )],
        }
    }
}

/// One resolved `Metamagic`-category feat's effect: its catalog id,
/// description text, and `TableCellRef` provenance, all read directly
/// off the matched `FeatTableEntry` — never hand-rolled. See this
/// module's doc comment for why this stays bounded to description text
/// rather than a numeric derived-stat delta.
#[derive(Debug, Clone, PartialEq)]
pub struct MetamagicFeatEffect {
    pub feat_id: String,
    pub description: String,
    pub table_cell: TableCellRef,
}

/// Resolves `feat_id` against the `Metamagic` slice of the canonical CRB
/// feat catalog. Returns `None` when `feat_id` is not a real
/// `Metamagic`-category record in the table store — SD-19 owns the table
/// store; this function reads it, it never fabricates an entry for a key
/// the table store doesn't have (mirrors
/// `feat_prereqs::general::resolve_general_feat_effect`'s,
/// `feat_prereqs::combat::resolve_combat_feat_effect`'s, and
/// `feat_prereqs::item_creation::resolve_item_creation_feat_effect`'s own
/// `None`-on-miss discipline).
pub fn resolve_metamagic_feat_effect(feat_id: &str) -> Option<MetamagicFeatEffect> {
    let entry = feat_tables().iter().find(|entry| {
        entry.category == FeatCategory::Metamagic && matches_feat_id(entry, feat_id)
    })?;

    Some(MetamagicFeatEffect {
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
    fn resolves_a_real_metamagic_feat() {
        let effect = resolve_metamagic_feat_effect("Empower Spell")
            .expect("Empower Spell is a real Metamagic feat in the landed catalog");
        assert_eq!(effect.feat_id, "Empower Spell");
        assert_eq!(
            effect.description,
            "You can increase the power of your spells, causing them to deal more damage."
        );
        assert_eq!(effect.table_cell.table, "feats");
        assert_eq!(effect.table_cell.row_key, "Empower Spell");
    }

    #[test]
    fn rejects_a_feat_from_a_different_category() {
        // Toughness is real, but filed under FeatCategory::General.
        assert!(resolve_metamagic_feat_effect("Toughness").is_none());
        let evaluation = evaluate_metamagic_feat_prerequisites("Toughness");
        assert!(!evaluation.is_eligible);
        assert!(!evaluation.failing_prerequisites.is_empty());
    }

    #[test]
    fn rejects_an_unknown_feat_id() {
        assert!(resolve_metamagic_feat_effect("Definitely Not A Feat").is_none());
        let evaluation = evaluate_metamagic_feat_prerequisites("Definitely Not A Feat");
        assert!(!evaluation.is_eligible);
        assert!(!evaluation.failing_prerequisites.is_empty());
    }
}
