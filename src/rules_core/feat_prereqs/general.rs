//! SD-20 Epic 3, first feat category (`scope-draft.md` §1.3 work-unit
//! order): CRB `General` feats.
//!
//! Reads `rules_tables::crb::feats::feat_tables()` directly (SD-19's
//! foundation-slice catalog, landed at `04c3d08`) — never hand-rolled,
//! per this cycle's own brief.
//!
//! **Bounded posture, and why.** `FeatTableEntry` (the landed catalog)
//! carries `key` / `category` / `name` / `description` per record only —
//! confirmed by reading `feats.rs` and `feat_data/general.rs` directly.
//! No `PREREQ:` / `PREABILITY:` / `PRELEVEL:` token is transcribed into
//! the table store, even though several real General feats have one on
//! the raw corpus (`core_rulebook/cr_feats.lst`, confirmed directly):
//! `Greater Spell Focus` carries `PREABILITY:1,CATEGORY=FEAT,Spell Focus`,
//! `Improved Great Fortitude` carries
//! `PREABILITY:1,CATEGORY=FEAT,Great Fortitude`, and `Leadership` carries
//! `PRELEVEL:MIN=7`. This epic's file-touch partition forbids editing
//! `rules_tables/crb/feats.rs` or `feat_data/` (read-only for Epic 3),
//! and this cycle's brief directs reading feat records via the `feats.rs`
//! catalog, not hand-rolling data — so a real per-feat prerequisite chain
//! cannot be evaluated from data this engine can query without either
//! violating the read-only boundary or fabricating a value the catalog
//! doesn't carry (the same "no counterfeit completion" discipline the
//! prior blocked cycle's log invoked).
//!
//! So this cycle's evaluation is honestly bounded to what the catalog
//! *does* prove: whether the requested feat id is a real `General`-
//! category record. A feat id absent from the `General` slice of
//! `feat_tables()` fails eligibility ("not found"); a feat id present has
//! no evaluable failing prerequisite recorded. This is not a claim that
//! every General feat has *no* real prerequisite — several do, as the
//! examples above show — it is an honest statement of what this engine
//! can currently verify from the landed table store. Widening this to a
//! real per-feat prerequisite chain requires the SD-19 table store to
//! carry `PREREQ:`-family tokens per record, which is a foundation-slice
//! change outside this cycle's scope (mirrors the original catalog
//! blocker's own "what would unblock it" framing).

use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::rules_tables::crb::feats::{feat_tables, FeatCategory};

/// Result of a bounded, catalog-membership-only prerequisite check for a
/// `General`-category feat lookup. See this module's doc comment for the
/// scope this is bounded to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneralFeatPrerequisiteEvaluation {
    pub is_eligible: bool,
    pub failing_prerequisites: Vec<String>,
}

/// Evaluates whether `feat_id` is eligible under a `General`-category
/// lookup. Eligible iff a `FeatCategory::General` record in the landed
/// catalog matches `feat_id` by `key` or `name` (`FeatTableEntry.key`
/// falls back to `name` when the corpus record carries no `KEY:` token,
/// per `feats.rs`'s own doc comment — matching on either field mirrors
/// that fallback rather than re-deriving it).
pub fn evaluate_general_feat_prerequisites(feat_id: &str) -> GeneralFeatPrerequisiteEvaluation {
    let found = feat_tables()
        .iter()
        .any(|entry| entry.category == FeatCategory::General && matches_feat_id(entry, feat_id));

    if found {
        GeneralFeatPrerequisiteEvaluation {
            is_eligible: true,
            failing_prerequisites: Vec::new(),
        }
    } else {
        GeneralFeatPrerequisiteEvaluation {
            is_eligible: false,
            failing_prerequisites: vec![format!(
                "'{feat_id}' is not a recognized CRB General feat in the catalog \
                 (rules_tables::crb::feats::feat_tables(), FeatCategory::General)"
            )],
        }
    }
}

/// One resolved `General`-category feat's effect: its catalog id,
/// description text, and `TableCellRef` provenance, all read directly
/// off the matched `FeatTableEntry` — never hand-rolled. See this
/// module's doc comment for why this stays bounded to description text
/// rather than a numeric derived-stat delta.
#[derive(Debug, Clone, PartialEq)]
pub struct GeneralFeatEffect {
    pub feat_id: String,
    pub description: String,
    pub table_cell: TableCellRef,
}

/// Resolves `feat_id` against the `General` slice of the canonical CRB
/// feat catalog. Returns `None` when `feat_id` is not a real
/// `General`-category record in the table store — SD-19 owns the table
/// store; this function reads it, it never fabricates an entry for a key
/// the table store doesn't have (mirrors
/// `spellbook::abjuration::resolve_abjuration_spell_effect`'s own
/// `None`-on-miss discipline).
pub fn resolve_general_feat_effect(feat_id: &str) -> Option<GeneralFeatEffect> {
    let entry = feat_tables()
        .iter()
        .find(|entry| entry.category == FeatCategory::General && matches_feat_id(entry, feat_id))?;

    Some(GeneralFeatEffect {
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
    fn resolves_a_real_general_feat() {
        let effect = resolve_general_feat_effect("Toughness")
            .expect("Toughness is a real General feat in the landed catalog");
        assert_eq!(effect.feat_id, "Toughness");
        assert_eq!(effect.description, "You have enhanced physical stamina.");
        assert_eq!(effect.table_cell.table, "feats");
        assert_eq!(effect.table_cell.row_key, "Toughness");
    }

    #[test]
    fn rejects_a_feat_from_a_different_category() {
        // Power Attack is real, but filed under FeatCategory::Combat.
        assert!(resolve_general_feat_effect("Power Attack").is_none());
        let evaluation = evaluate_general_feat_prerequisites("Power Attack");
        assert!(!evaluation.is_eligible);
        assert!(!evaluation.failing_prerequisites.is_empty());
    }

    #[test]
    fn rejects_an_unknown_feat_id() {
        assert!(resolve_general_feat_effect("Definitely Not A Feat").is_none());
        let evaluation = evaluate_general_feat_prerequisites("Definitely Not A Feat");
        assert!(!evaluation.is_eligible);
        assert!(!evaluation.failing_prerequisites.is_empty());
    }
}
