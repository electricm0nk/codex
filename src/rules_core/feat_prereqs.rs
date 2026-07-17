//! SD-20 feat prerequisite engine — Epic 3 (`scope-draft.md` §1.3,
//! `technical-design.md` §2.2).
//!
//! Third Epic-3 cycle, third work-unit per `scope-draft.md` §1.3's cycle
//! order (general feats, then combat, now `ItemCreation`). The first cycle
//! (`b830769`) landed `FeatCategory::General` after an earlier blocked
//! cycle (`cycle-2026-07-17T1920`) found the SD-19 table store had no feat
//! catalog at all — resolved at `04c3d08`, which landed
//! `rules_tables::crb::feats` (`feat_tables()`, 185 real CRB feat records
//! across four categories — General 50, Combat 110, ItemCreation 8,
//! Metamagic 17). The second cycle (`c15983d`) landed `FeatCategory::Combat`.
//! This cycle lands the third category, `FeatCategory::ItemCreation` — see
//! `feat_prereqs/item_creation.rs`, which mirrors `feat_prereqs/general.rs`
//! and `feat_prereqs/combat.rs` exactly.
//!
//! Reads the feat catalog directly (`rules_tables::crb::feats::feat_tables()`)
//! per `technical-design.md` §2.0's table-store access convention (no
//! `RulesTables` parameter of any kind; a direct, fully-qualified `use`
//! import of the specific table item, called inline) — the same pattern
//! Epic 2 (`spellbook.rs` / `spellbook/abjuration.rs`) and Epic 5
//! (`equipment_effects.rs` / `equipment_effects/arms_armor.rs`) already
//! converged on independently.
//!
//! `technical-design.md` §2.2's illustrative seam signature takes a
//! `feat: &FeatKey` parameter and a `character_history: &CharacterHistory`
//! parameter (a type sketched only in a comment: "feats taken, race,
//! class, ability scores, BAB, skills" — never defined anywhere in this
//! repo). This cycle drops `character_history`/`CharacterHistory`
//! entirely rather than inventing a parallel type or re-deriving a
//! duplicate of the already-landed `CharacterInput`
//! (`character_input.rs`) — the same "adapt illustrative doctrine types
//! to the real codebase shape, don't invent a parallel type" precedent
//! Epic 1's `contract.rs` and Epic 4's `skill_allocation.rs` both already
//! set. It is dropped (not adapted to `&CharacterInput`) because this
//! cycle's bounded General-feats evaluation needs no character state at
//! all — see `feat_prereqs/general.rs`'s doc comment for why. A future
//! category cycle that needs real character context (e.g. a Combat feat
//! gated on BAB, or a Metamagic feat needing known spells) should add a
//! `character: &CharacterInput` parameter back onto these functions at
//! that point, when there is a real field to read from it — not before.
//!
//! `FeatKey` is defined here (not sketched with fields anywhere in
//! `technical-design.md`) as the minimal identity a catalog lookup needs:
//! the feat's catalog id plus its category, reusing the already-landed
//! `rules_tables::crb::feats::FeatCategory` enum rather than re-deriving
//! a duplicate category taxonomy.

pub mod combat;
pub mod general;
pub mod item_creation;

use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::crb::feats::FeatCategory;

/// Identifies one catalog feat: its id (matches `FeatTableEntry.key` /
/// `.name`, and `CharacterInput.chosen.selected_feats` entries) plus the
/// category it is being evaluated under.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatKey {
    pub feat_id: String,
    pub category: FeatCategory,
}

/// Result of checking whether a feat's prerequisites are met. See this
/// module's doc comment: for the landed `General` category, "met" means
/// "found in the catalog under the requested category" — the table store
/// carries no per-feat prerequisite chain data (yet) to evaluate more
/// specifically.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrerequisiteEvaluation {
    pub is_eligible: bool,
    pub failing_prerequisites: Vec<FailedPrerequisite>,
    pub warnings: Vec<PrerequisiteWarning>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailedPrerequisite {
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrerequisiteWarning {
    pub message: String,
}

/// The delta a feat contributes, per `technical-design.md` §2.2. Bounded
/// (like `spellbook::abjuration::AbjurationSpellEffect`) to what the
/// catalog table actually carries: the feat's `DESC:` text plus
/// `TableCellRef` provenance — no numeric derived-stat delta, since the
/// catalog carries no `BONUS:`-token data (unlike `EquipmentRecord`,
/// which exposes raw corpus tokens; `FeatTableEntry` only carries
/// `key`/`category`/`name`/`description`). `None` fields mean "not
/// resolved" (unknown feat id, or a category this engine has not landed
/// yet), never a fabricated value.
#[derive(Debug, Clone, PartialEq)]
pub struct FeatEffects {
    pub feat_id: String,
    pub description: Option<String>,
    pub table_cell: Option<TableCellRef>,
}

/// Dispatches by category to the per-category evaluation function, the
/// same dispatch shape `spellbook::compute_spellbook_coverage` and
/// `equipment_effects::compute_equipment_effects` already use. Only
/// `FeatCategory::General`, `FeatCategory::Combat`, and
/// `FeatCategory::ItemCreation` have a landed per-category module as of
/// this cycle; `FeatCategory::Metamagic` honestly reports "not yet
/// supported by this engine" rather than fabricating an eligibility
/// verdict — a future cycle's `feat_prereqs/metamagic.rs` file extends
/// this match without changing this dispatch's shape (mirrors
/// `spellbook.rs`'s own note on unlanded schools).
pub fn evaluate_feat_prerequisites(feat: &FeatKey) -> PrerequisiteEvaluation {
    match feat.category {
        FeatCategory::General => {
            let result = general::evaluate_general_feat_prerequisites(&feat.feat_id);
            PrerequisiteEvaluation {
                is_eligible: result.is_eligible,
                failing_prerequisites: result
                    .failing_prerequisites
                    .into_iter()
                    .map(|reason| FailedPrerequisite { reason })
                    .collect(),
                warnings: Vec::new(),
            }
        }
        FeatCategory::Combat => {
            let result = combat::evaluate_combat_feat_prerequisites(&feat.feat_id);
            PrerequisiteEvaluation {
                is_eligible: result.is_eligible,
                failing_prerequisites: result
                    .failing_prerequisites
                    .into_iter()
                    .map(|reason| FailedPrerequisite { reason })
                    .collect(),
                warnings: Vec::new(),
            }
        }
        FeatCategory::ItemCreation => {
            let result = item_creation::evaluate_item_creation_feat_prerequisites(&feat.feat_id);
            PrerequisiteEvaluation {
                is_eligible: result.is_eligible,
                failing_prerequisites: result
                    .failing_prerequisites
                    .into_iter()
                    .map(|reason| FailedPrerequisite { reason })
                    .collect(),
                warnings: Vec::new(),
            }
        }
        FeatCategory::Metamagic => PrerequisiteEvaluation {
            is_eligible: false,
            failing_prerequisites: vec![FailedPrerequisite {
                reason: format!(
                    "feat category {:?} is not yet supported by the feat prerequisite engine \
                     (only General, Combat, and ItemCreation feats are landed as of this cycle)",
                    feat.category
                ),
            }],
            warnings: Vec::new(),
        },
    }
}

pub fn compute_feat_effects(feat: &FeatKey) -> FeatEffects {
    match feat.category {
        FeatCategory::General => match general::resolve_general_feat_effect(&feat.feat_id) {
            Some(effect) => FeatEffects {
                feat_id: effect.feat_id,
                description: Some(effect.description),
                table_cell: Some(effect.table_cell),
            },
            None => FeatEffects {
                feat_id: feat.feat_id.clone(),
                description: None,
                table_cell: None,
            },
        },
        FeatCategory::Combat => match combat::resolve_combat_feat_effect(&feat.feat_id) {
            Some(effect) => FeatEffects {
                feat_id: effect.feat_id,
                description: Some(effect.description),
                table_cell: Some(effect.table_cell),
            },
            None => FeatEffects {
                feat_id: feat.feat_id.clone(),
                description: None,
                table_cell: None,
            },
        },
        FeatCategory::ItemCreation => {
            match item_creation::resolve_item_creation_feat_effect(&feat.feat_id) {
                Some(effect) => FeatEffects {
                    feat_id: effect.feat_id,
                    description: Some(effect.description),
                    table_cell: Some(effect.table_cell),
                },
                None => FeatEffects {
                    feat_id: feat.feat_id.clone(),
                    description: None,
                    table_cell: None,
                },
            }
        }
        FeatCategory::Metamagic => FeatEffects {
            feat_id: feat.feat_id.clone(),
            description: None,
            table_cell: None,
        },
    }
}
