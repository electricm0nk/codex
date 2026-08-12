//! Real corpus loading for the desktop app — closes the gap
//! `corpus_fixtures.rs` documents as out of scope.
//!
//! Built 2026-07-30
//! (`docs/release/v0.6/book-agnostic-backend-gaps-scoping.md` finding 4):
//! every live character build previously used `corpus_fixtures::
//! corpus_fixture_bundle()` — a hardcoded ~4-record fixture, regardless of
//! book. This loads real equipment data from `data/corpus/<book>/equipment/`
//! for all 6 already-ingested books via `codex::rules_core::corpus_loader`
//! (built and proven the same day — see that module's own doc comment).
//!
//! **Equipment only, for now.** Spell/feat/class content-kinds still route
//! through `corpus_fixtures.rs`'s own bundled fixtures (2 real spells) —
//! `corpus_loader` doesn't cover those content-kinds yet. This module's
//! `full_corpus_bundle()` merges the two: real equipment for all 6 books,
//! plus the existing fixture spells, into one `SourcePackageContent`. An
//! honest partial improvement, not a false completeness claim.
//!
//! **Path resolution, not yet packaged-app-ready.** Uses `codex_repo_root()`
//! (via `resolve_package_path`, same mechanism `corpus_fixtures.rs` already
//! uses for its own dev-mode fallback) to find `data/corpus/` — works
//! correctly for a source checkout (dev builds, this repo's own tests) and
//! for any deployment that sets `CODEX_REPO_ROOT` to a location carrying
//! `data/corpus/`. Bundling `data/corpus/` into a packaged Tauri installer
//! (via `tauri.conf.json`'s `resources` list, the way `corpus_fixtures/` is
//! bundled today) is real, separate follow-on work — not done here, and not
//! silently assumed to work for a shipped binary.

use std::path::PathBuf;
use std::sync::OnceLock;

use codex::rules_core::corpus_loader::{load_equipment_corpus, BookCorpusRoot};
use codex::rules_core::source_content::{SourceContentKind, SourcePackageContent};

use crate::corpus_fixtures::corpus_fixture_bundle;
use crate::authoring_workbench::codex_repo_root;

const BOOKS: &[&str] = &[
    "core_rulebook",
    "advanced_players_guide",
    "advanced_class_guide",
    "beastiary",
    "advanced_race_guide",
    "pathfinder_unchained",
];

fn corpus_root_dir() -> Result<PathBuf, String> {
    codex_repo_root().map(|root| root.join("data/corpus"))
}

fn build_full_corpus_bundle() -> SourcePackageContent<'static> {
    let corpus_root = corpus_root_dir().expect("data/corpus root must resolve");
    let book_dirs: Vec<PathBuf> = BOOKS.iter().map(|book| corpus_root.join(book)).collect();
    let roots: Vec<BookCorpusRoot<'_>> = BOOKS
        .iter()
        .zip(book_dirs.iter())
        .map(|(book_id, dir)| BookCorpusRoot { book_id, dir: dir.as_path() })
        .collect();
    let mut package: SourcePackageContent<'static> = load_equipment_corpus(&roots);

    // Merge in the existing bundled fixture's real spell records (2) --
    // spell/feat/class content-kinds don't have a real loader yet.
    for record in corpus_fixture_bundle().records_by_kind(SourceContentKind::Spell) {
        package.push(record);
    }

    package
}

/// The real corpus bundle: all 6 books' equipment (loaded from the real
/// on-disk `data/corpus/`) plus the existing bundled fixture's 2 spells.
/// Built once, cached for the process lifetime — mirrors
/// `corpus_fixtures::corpus_fixture_bundle()`'s own caching shape.
pub fn full_corpus_bundle() -> &'static SourcePackageContent<'static> {
    static BUNDLE: OnceLock<SourcePackageContent<'static>> = OnceLock::new();
    BUNDLE.get_or_init(build_full_corpus_bundle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_corpus_bundle_carries_real_equipment_from_multiple_books_and_the_fixture_spells() {
        let corpus = full_corpus_bundle();
        let equipment_count = corpus.records_by_kind(SourceContentKind::Equipment).len();
        assert!(
            equipment_count > 2000,
            "expected thousands of real equipment records across 6 books, got {equipment_count}"
        );
        assert_eq!(
            corpus.records_by_kind(SourceContentKind::Spell).len(),
            2,
            "spell content-kind still routes through the bundled fixture for now"
        );
    }

    #[test]
    fn a_real_item_from_a_non_crb_book_resolves_with_real_mechanical_data() {
        use codex::rules_core::equipment_resolver::equipment_id_resolve;
        use codex::rules_core::rules_tables::RuleSetId;

        let corpus = full_corpus_bundle();
        let (record, _) = equipment_id_resolve("Dogslicer", RuleSetId::Crb, corpus)
            .expect("ARG's real Dogslicer must resolve through the real desktop corpus bundle");
        let wt = record.tokens.iter().find(|t| t.key == "WT").expect("real WT: token");
        assert_eq!(wt.value, "1");
    }
}

