//! Option-pool `class_feature` reference catalog -- Tauri command adapter
//! over `codex::rules_core::class_feature_pool_catalog` (SD31-W22-POOLMEMBER-001).
//!
//! # Why this module exists
//!
//! `class_feature_descriptions.rs` serves a class's OWN feature descriptions
//! (Sneak Attack, Trapfinding, ...), joined to the engine's own real
//! `ExplanationDto.id`. An option-pool member (a Rogue Talent, a Sorcerer
//! Bloodline power, ...) is not GRANTED by the class directly -- the class
//! grants a pick from a pool, and no `ExplanationDto.id` names the specific
//! record until a character has actually chosen it. `class_feature_
//! descriptions.rs`'s own doc comment says so explicitly: "a pool-member
//! record with no `~`-split owner has no class to join against and is
//! correctly absent from this catalog".
//!
//! This module is the missing browsable surface for that population --
//! modelled directly on `race_trait_picker.rs`'s own ARG-alternate-trait
//! menu (`list_alternate_racial_traits`), which already established the
//! precedent this program's own Decision 7 relies on: a menu of every real
//! option, described from the corpus, shown regardless of whether the
//! player has selected it yet, satisfies condition 3's "renders on the
//! character sheet" bar (`SD31-D7-PROSE-001`/`002`, +146 `race_trait` units).
//! `Kind::ClassFeature`'s own doc comment in `v06_work_inventory.rs` names
//! the missing precondition directly: "no generic class_feature catalog
//! exists anywhere in this engine, unlike feat/spell/equipment"
//! (`decisions.md §42`). This module, and the shared lib catalog it wraps,
//! is that catalog -- for the registered pools
//! (`class_feature_pool_catalog::REGISTERED_POOL_GROUPS`: Rogue Talent,
//! wave 22; Rage Power, `SD31-W23-POOLMEMBER-002`) proved end to end so
//! far.
//!
//! # No new corpus data
//!
//! Every record served here already lives in the committed `data/corpus/`
//! cache; this module (and its shared lib counterpart) adds a new READING of
//! existing, already-PI-screened data, not a new ingest.
//!
//! # PI screening
//!
//! Discharged upstream by `cache_gen::class_feature::generate` before a
//! record is ever written to `data/corpus/` (SD-30 `§52.3`/`§53.5`). This
//! module re-runs no PI check of its own -- same trust boundary every other
//! `data/corpus/`-reading module in this crate holds.

use std::sync::OnceLock;

use serde::Serialize;

use codex::rules_core::class_feature_pool_catalog::{PoolCatalogEntry, load_pool_catalog};

use crate::authoring_workbench::codex_repo_root;

/// One option-pool member, ready for a browsable reference list.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ClassFeaturePoolOptionDto {
    pub book: String,
    /// The registered pool group this record belongs to (e.g. `"Rogue
    /// Talent"`).
    pub pool_group: String,
    /// The corpus `KEY:` token verbatim.
    pub key: String,
    pub name: String,
    /// Rendered, leak-checked prose -- see the shared lib catalog's own doc
    /// comment for the render-and-refuse gate.
    pub description: String,
}

impl From<PoolCatalogEntry> for ClassFeaturePoolOptionDto {
    fn from(entry: PoolCatalogEntry) -> Self {
        Self {
            book: entry.book,
            pool_group: entry.pool_group,
            key: entry.key,
            name: entry.name,
            description: entry.description,
        }
    }
}

/// Built once, cached for the process lifetime -- mirrors
/// `class_feature_descriptions::class_feature_descriptions()`'s own caching
/// shape.
fn pool_options() -> &'static Vec<ClassFeaturePoolOptionDto> {
    static TABLE: OnceLock<Vec<ClassFeaturePoolOptionDto>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let repo_root = codex_repo_root()
            .expect("codex repo root must resolve for class_feature pool catalog loading");
        load_pool_catalog(&repo_root).into_iter().map(ClassFeaturePoolOptionDto::from).collect()
    })
}

#[tauri::command]
pub fn list_class_feature_pool_options() -> Vec<ClassFeaturePoolOptionDto> {
    pool_options().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The real corpus loads real Rogue Talent options -- proven against
    /// the live checkout, not a fixture.
    #[test]
    fn list_class_feature_pool_options_returns_real_rogue_talents() {
        let options = list_class_feature_pool_options();
        assert!(options.len() >= 10, "expected at least 10 real Rogue Talent options, got {}", options.len());
        let ledge_walker = options
            .iter()
            .find(|o| o.book == "core_rulebook" && o.key == "Rogue Talent ~ Ledge Walker")
            .expect("core_rulebook's Ledge Walker must be in the catalog");
        assert_eq!(ledge_walker.pool_group, "Rogue Talent");
        assert!(!ledge_walker.description.is_empty());
    }

    /// The known unresolvable-argument record must never appear -- proves
    /// the render-and-refuse gate crosses the IPC boundary intact.
    #[test]
    fn bleeding_attack_never_reaches_the_command_output() {
        let options = list_class_feature_pool_options();
        assert!(!options.iter().any(|o| o.key == "Rogue Talent ~ Bleeding Attack"));
    }

    #[test]
    fn list_class_feature_pool_options_returns_the_cached_table() {
        let a = list_class_feature_pool_options();
        let b = list_class_feature_pool_options();
        assert_eq!(a.len(), b.len());
        assert!(!a.is_empty());
    }
}
