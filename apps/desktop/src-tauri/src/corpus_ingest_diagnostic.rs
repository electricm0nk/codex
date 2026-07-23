//! Corpus ingest diagnostic (Criterion 5.1, Epic 5 sketch scope).
//!
//! Reports the REAL ingested state of every populated `rules_tables` book —
//! `RuleSetId::{Crb,Apg,Acg,Bestiary1}` — by counting the actual data
//! structures compiled into this binary (`ClassId::ALL`, `SPELL_LIST`,
//! `equipment_tables()`, ...). This is the same "book-partitioned table
//! store" `docs/architecture/rules-data-tables.md` describes; nothing here
//! is fixture/hand-guessed data — every count is read from the real
//! landed tables, so a future book that ships zero content, or a book that
//! loses a class roster to a regression, changes this diagnostic's output
//! without anyone touching this file.
//!
//! **Sketch-only boundary (per `cycles/5_1.md`):** exactly four fields —
//! `book_id`, `status`, `last_ingested_at`, `content_kind_counts`. SD-26
//! fans out the full status table + flags + ETA once the JSON ingest cache
//! lands; anything beyond these four fields belongs there, not here.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};

use codex::rules_core::rules_tables::acg::{self, AcgClassId};
use codex::rules_core::rules_tables::apg::{self, ApgClassId};
use codex::rules_core::rules_tables::beastiary1::MonsterId;
use codex::rules_core::rules_tables::crb::{
    class_tables::ClassId, equipment_tables as crb_equipment_tables, feats as crb_feats,
    race_tables::RaceId, spell_list as crb_spell_list,
};

/// One book's real ingested-corpus status. Field set is deliberately
/// bounded to the sketch scope named above.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookIngestStatus {
    /// Lowercase book identifier, matching the `rules_tables` directory
    /// name (`"crb"`, `"apg"`, `"acg"`, `"beastiary1"`).
    pub book_id: String,
    /// `"populated"` when this book's real content-kind counts sum to more
    /// than zero, `"empty"` otherwise — computed, never hand-set per book.
    pub status: String,
    /// RFC 3339 timestamp of the most recent git commit that touched this
    /// book's `rules_tables` directory, or `None` when git history isn't
    /// reachable (e.g. a packaged build with no `.git` checkout alongside
    /// it — the same graceful-degradation shape `build.rs`'s
    /// `git_short_sha` already uses for `CODEX_GIT_SHA`).
    pub last_ingested_at: Option<String>,
    /// Real per-kind record counts (e.g. `"classes"`, `"spells"`), read
    /// directly from the book's own landed tables.
    pub content_kind_counts: BTreeMap<String, u32>,
}

/// Every Bestiary 1 monster the real `MonsterId` enum declares today.
///
/// This used to be a hand-maintained duplicate list (`beastiary1::mod.rs`
/// had no public `ALL`/count constant, unlike
/// `ClassId::ALL`/`ApgClassId::ALL`/`AcgClassId::ALL`). SD-26 Epic 3
/// Criterion 3.4 (`decisions.md §11.6`) added the real
/// `MonsterId::ALL` constant so this diagnostic and the JSON-cache
/// generator (`codex::rules_core::cache_gen::beastiary1`) both read the
/// same single source of truth instead of each maintaining their own
/// copy of this list a second/third time.
const ALL_BESTIARY1_MONSTERS: &[MonsterId] = MonsterId::ALL;

fn crb_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("classes".to_string(), ClassId::ALL.len() as u32);
    counts.insert("races".to_string(), RaceId::ALL.len() as u32);
    counts.insert(
        "feats".to_string(),
        crb_feats::feat_tables().len() as u32,
    );
    counts.insert(
        "spells".to_string(),
        crb_spell_list::SPELL_LIST.len() as u32,
    );
    counts.insert(
        "equipment".to_string(),
        crb_equipment_tables::equipment_tables().len() as u32,
    );
    counts
}

fn apg_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("classes".to_string(), ApgClassId::ALL.len() as u32);
    counts.insert("spells".to_string(), apg::spell_list::SPELL_LIST.len() as u32);
    counts.insert(
        "equipment".to_string(),
        apg::equipment_tables::EQUIPMENT_TABLE.len() as u32,
    );
    counts
}

fn acg_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("classes".to_string(), AcgClassId::ALL.len() as u32);
    counts.insert("spells".to_string(), acg::spell_list::SPELL_LIST.len() as u32);
    counts.insert(
        "equipment".to_string(),
        acg::equipment_tables::equipment_tables().len() as u32,
    );
    counts
}

fn beastiary1_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("monsters".to_string(), ALL_BESTIARY1_MONSTERS.len() as u32);
    counts
}

/// Repo root, derived from the crate's own compile-time manifest
/// directory (`apps/desktop/src-tauri`) rather than the process's current
/// working directory, which Tauri does not guarantee.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

/// RFC 3339 commit date of the most recent commit touching
/// `repo_relative_dir`, or `None` when git isn't reachable (no `.git`
/// checkout, `git` not on `PATH`, or the path has no history — e.g. a
/// packaged production build, which ships the compiled binary only).
fn last_commit_iso_date(repo_relative_dir: &str) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root())
        .args(["log", "-1", "--format=%cI", "--", repo_relative_dir])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let text = String::from_utf8(output.stdout).ok()?;
    let trimmed = text.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn book_status(book_id: &str, repo_relative_dir: &str, counts: BTreeMap<String, u32>) -> BookIngestStatus {
    let total: u32 = counts.values().sum();
    BookIngestStatus {
        book_id: book_id.to_string(),
        status: if total > 0 { "populated" } else { "empty" }.to_string(),
        last_ingested_at: last_commit_iso_date(repo_relative_dir),
        content_kind_counts: counts,
    }
}

/// Build the full diagnostic response. A thin, testable wrapper behind the
/// Tauri command below, mirroring `class_catalog::build_class_catalog`'s
/// command/pure-fn split.
pub fn build_corpus_ingest_diagnostic() -> Vec<BookIngestStatus> {
    vec![
        book_status("crb", "src/rules_core/rules_tables/crb", crb_counts()),
        book_status("apg", "src/rules_core/rules_tables/apg", apg_counts()),
        book_status("acg", "src/rules_core/rules_tables/acg", acg_counts()),
        book_status(
            "beastiary1",
            "src/rules_core/rules_tables/beastiary1",
            beastiary1_counts(),
        ),
    ]
}

#[tauri::command]
pub fn corpus_ingest_diagnostic() -> Vec<BookIngestStatus> {
    build_corpus_ingest_diagnostic()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_exactly_the_four_populated_books() {
        let response = build_corpus_ingest_diagnostic();
        let book_ids: Vec<&str> = response.iter().map(|b| b.book_id.as_str()).collect();
        assert_eq!(book_ids, vec!["crb", "apg", "acg", "beastiary1"]);
    }

    #[test]
    fn every_book_is_populated_with_real_nonzero_counts() {
        for book in build_corpus_ingest_diagnostic() {
            assert_eq!(book.status, "populated", "book {} should be populated", book.book_id);
            assert!(
                !book.content_kind_counts.is_empty(),
                "book {} must report at least one content kind",
                book.book_id
            );
            for (kind, count) in &book.content_kind_counts {
                assert!(
                    *count > 0,
                    "book {} kind {} must have a real nonzero count",
                    book.book_id,
                    kind
                );
            }
        }
    }

    #[test]
    fn crb_counts_match_the_real_underlying_tables() {
        let response = build_corpus_ingest_diagnostic();
        let crb = response.iter().find(|b| b.book_id == "crb").expect("crb present");
        assert_eq!(crb.content_kind_counts["classes"], ClassId::ALL.len() as u32);
        assert_eq!(crb.content_kind_counts["races"], RaceId::ALL.len() as u32);
        assert_eq!(
            crb.content_kind_counts["feats"],
            crb_feats::feat_tables().len() as u32
        );
        assert_eq!(
            crb.content_kind_counts["spells"],
            crb_spell_list::SPELL_LIST.len() as u32
        );
        assert_eq!(
            crb.content_kind_counts["equipment"],
            crb_equipment_tables::equipment_tables().len() as u32
        );
    }

    #[test]
    fn beastiary1_monster_count_matches_the_documented_real_total() {
        // docs/architecture/rules-data-tables.md: "41 monsters total as of
        // this verification" (mod.rs's own subset roster doc comments).
        let response = build_corpus_ingest_diagnostic();
        let bestiary = response
            .iter()
            .find(|b| b.book_id == "beastiary1")
            .expect("beastiary1 present");
        assert_eq!(bestiary.content_kind_counts["monsters"], 41);
    }

    #[test]
    fn last_ingested_at_is_a_real_git_derived_timestamp_when_available() {
        // This test runs inside the real repo checkout, so git history for
        // every book directory must be reachable — assert the diagnostic
        // actually queried it rather than silently defaulting to None.
        for book in build_corpus_ingest_diagnostic() {
            let timestamp = book
                .last_ingested_at
                .as_ref()
                .unwrap_or_else(|| panic!("expected a real git-derived timestamp for {}", book.book_id));
            assert!(
                timestamp.contains('T'),
                "timestamp {timestamp} for {} should be RFC 3339-shaped",
                book.book_id
            );
        }
    }
}
