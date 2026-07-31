//! Corpus ingest diagnostic (Criterion 5.1, Epic 5 sketch scope).
//!
//! Reports the REAL ingested state of every book landed in `rules_tables` —
//! `RuleSetId::{Crb,Apg,Acg,Bestiary1,Arg,Pu}` — by counting the actual data
//! structures compiled into this binary (`ClassId::ALL`, `SPELL_LIST`,
//! `equipment_tables()`, ...). This is the same "book-partitioned table
//! store" `docs/architecture/rules-data-tables.md` describes; nothing here
//! is fixture/hand-guessed data — every count is read from the real
//! landed tables, so a future book that ships zero content, or a book that
//! loses a class roster to a regression, changes this diagnostic's output
//! without anyone touching this file.
//!
//! # SD-27 (2026-07-31): the panel was claiming two ingested books did not exist
//!
//! This module reported four books — `crb`, `apg`, `acg`, `beastiary1` —
//! long after `advanced_race_guide` and `pathfinder_unchained` landed as
//! real `rules_tables` books with real records (ARG: 200 equipment, 187
//! feats, 92 spells, 156 racial traits; PU: 4 classes, 64 class features, 42
//! equipment modifiers, 17 feats). Because the panel's own caption reads
//! "every rule book landed in `rules_tables`", a tester reading that screen
//! would correctly conclude, from a truthful-sounding caption, that two
//! ingested books were missing.
//!
//! Both are now reported, and
//! `tests::every_book_landed_in_rules_tables_is_reported` derives the
//! expected book set by **reading `src/rules_core/rules_tables/`**, so book
//! seven fails this module's own test suite until it is added here. That
//! guard, not the one-time correction, is the fix.
//!
//! ## What this panel is still not counting, stated rather than left implicit
//!
//! ARG's **156 alternate/default racial-trait records are not a row here**,
//! and that is a boundary, not an oversight. They have no `rules_tables`
//! module: they were ingested straight to
//! `data/corpus/advanced_race_guide/race_trait/` and are read at runtime by
//! `race_resolver::load_race_corpus` (the path `race_catalog.rs` and
//! `race_trait_picker.rs` use to put them on screen). This panel's own
//! caption — "every rule book landed in `rules_tables`" — is what bounds it,
//! and folding a corpus-JSON-only content kind into a `rules_tables` count
//! would make that caption false in the other direction.
//!
//! They are accounted for elsewhere, so nothing is hidden: their compliance
//! count is in `data/corpus/advanced_race_guide/LICENSE.json` (635 records,
//! guarded by `tests/sd27_book_license_record_counts.rs`), and every one of
//! ARG's 153 *alternate* traits reaches the player through
//! `list_alternate_racial_traits`.
//!
//! **Open finding, found while widening this file and not repaired here:** of
//! those 156 records, 3 reach no player surface at all —
//! `Feral ~ Languages` and `Scion of Humanity ~ Languages`
//! (`TraitRole::Unclassified`: no gate the resolver can read) and
//! `Saltbeard ~ Dwarf ~ Greed` (`TraitRole::FlagGranted`). All three are
//! dropped by `race_trait_picker::build_menu`, which filters to
//! `Default | Alternate`. Repairing that belongs to the picker, not to this
//! diagnostic.
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
use codex::rules_core::rules_tables::advanced_race_guide as arg;
use codex::rules_core::rules_tables::apg::{self, ApgClassId};
use codex::rules_core::rules_tables::beastiary1::MonsterId;
use codex::rules_core::rules_tables::crb::{
    class_tables::ClassId, equipment_tables as crb_equipment_tables, feats as crb_feats,
    race_tables::RaceId, spell_list as crb_spell_list,
};
use codex::rules_core::rules_tables::pathfinder_unchained as pu;

/// One book's real ingested-corpus status. Field set is deliberately
/// bounded to the sketch scope named above.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookIngestStatus {
    /// Lowercase book identifier, matching the `rules_tables` directory
    /// name (`"crb"`, `"apg"`, `"acg"`, `"beastiary1"`,
    /// `"advanced_race_guide"`, `"pathfinder_unchained"`). That the two sets
    /// are identical is asserted, not assumed — see
    /// `tests::every_book_landed_in_rules_tables_is_reported`.
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
    counts.insert("feats".to_string(), apg::feats::feat_tables().len() as u32);
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
    counts.insert("feats".to_string(), acg::feats::feat_tables().len() as u32);
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

/// Every Pathfinder Unchained class feature the four real feature tables
/// declare.
///
/// Summed rather than read from one constant because PU has no aggregate: the
/// records live in four per-class modules, two exposing `Feature::ALL` and two
/// a `features()` accessor. Summing the four live tables is what makes this a
/// derived count — a class whose feature table is emptied by a regression
/// changes this number without anyone editing it.
fn pu_class_feature_count() -> u32 {
    (pu::barbarian_features::features().len()
        + pu::monk_features::features().len()
        + pu::rogue_features::UnchainedRogueFeature::ALL.len()
        + pu::summoner_features::UnchainedSummonerFeature::ALL.len()) as u32
}

fn advanced_race_guide_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("feats".to_string(), arg::feats::feat_tables().len() as u32);
    counts.insert("spells".to_string(), arg::spell_list::SPELL_LIST.len() as u32);
    counts.insert(
        "equipment".to_string(),
        arg::equipment_tables::equipment_tables().len() as u32,
    );
    counts
}

fn pathfinder_unchained_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert(
        "classes".to_string(),
        pu::class_chassis::PuClassId::ALL.len() as u32,
    );
    counts.insert("class_features".to_string(), pu_class_feature_count());
    counts.insert("feats".to_string(), pu::feat_tables::feat_tables().len() as u32);
    counts.insert(
        "equipment".to_string(),
        pu::equipment_tables::equipment_tables().len() as u32,
    );
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
        book_status(
            "advanced_race_guide",
            "src/rules_core/rules_tables/advanced_race_guide",
            advanced_race_guide_counts(),
        ),
        book_status(
            "pathfinder_unchained",
            "src/rules_core/rules_tables/pathfinder_unchained",
            pathfinder_unchained_counts(),
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
    use std::collections::BTreeSet;

    /// **The drift guard.** The set of books this diagnostic reports must
    /// equal the set of books actually landed in `rules_tables`, derived by
    /// reading the directory rather than from a list maintained here.
    ///
    /// This is the test that would have caught the defect it was written for:
    /// `advanced_race_guide` and `pathfinder_unchained` landed as real
    /// `rules_tables` books and this diagnostic kept reporting four, so the
    /// panel — whose caption reads "every rule book landed in `rules_tables`"
    /// — told a tester that two ingested books did not exist.
    #[test]
    fn every_book_landed_in_rules_tables_is_reported() {
        let reported: BTreeSet<String> = build_corpus_ingest_diagnostic()
            .into_iter()
            .map(|book| book.book_id)
            .collect();
        let landed = books_on_disk();

        let missing: Vec<&String> = landed.difference(&reported).collect();
        assert!(
            missing.is_empty(),
            "these books are landed in src/rules_core/rules_tables/ and this diagnostic does not \
             report them: {missing:?}. The Corpus Ingest panel states it shows every rule book \
             landed in rules_tables, so an unreported book reads to a tester as an un-ingested \
             book. Add a `book_status(..)` row deriving its counts from its own real tables."
        );

        let phantom: Vec<&String> = reported.difference(&landed).collect();
        assert!(
            phantom.is_empty(),
            "this diagnostic reports books with no src/rules_core/rules_tables/ directory: \
             {phantom:?}"
        );
    }

    /// Every book directory under `src/rules_core/rules_tables/`, which is
    /// where the diagnostic's own `book_id` values come from.
    fn books_on_disk() -> BTreeSet<String> {
        let dir = repo_root().join("src/rules_core/rules_tables");
        std::fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("{} must be readable: {e}", dir.display()))
            .filter_map(|entry| {
                let entry = entry.expect("readable dir entry");
                entry
                    .path()
                    .is_dir()
                    .then(|| entry.file_name().to_string_lossy().into_owned())
            })
            .collect()
    }

    #[test]
    fn reports_every_landed_book_in_a_stable_order() {
        let response = build_corpus_ingest_diagnostic();
        let book_ids: Vec<&str> = response.iter().map(|b| b.book_id.as_str()).collect();
        assert_eq!(
            book_ids,
            vec![
                "crb",
                "apg",
                "acg",
                "beastiary1",
                "advanced_race_guide",
                "pathfinder_unchained"
            ]
        );
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

    /// The APG/ACG feat ingest has to show up here too — this diagnostic
    /// is what reports per-book coverage, and leaving `feats` off the APG
    /// and ACG rows would understate 301 real ingested records.
    #[test]
    fn apg_and_acg_feat_counts_match_the_real_underlying_tables() {
        let response = build_corpus_ingest_diagnostic();
        let apg_book = response.iter().find(|b| b.book_id == "apg").expect("apg present");
        assert_eq!(
            apg_book.content_kind_counts["feats"],
            apg::feats::feat_tables().len() as u32
        );
        assert_eq!(apg_book.content_kind_counts["feats"], 172);

        let acg_book = response.iter().find(|b| b.book_id == "acg").expect("acg present");
        assert_eq!(
            acg_book.content_kind_counts["feats"],
            acg::feats::feat_tables().len() as u32
        );
        assert_eq!(acg_book.content_kind_counts["feats"], 129);
    }

    /// ARG's rows must be the book's own real tables, not a repeat of another
    /// book's. Asserted against the live tables *and* against the literal
    /// counts each module's own doc comment documents, so a silent table
    /// regression fails here even though both sides moved together.
    #[test]
    fn advanced_race_guide_counts_match_the_real_underlying_tables() {
        let response = build_corpus_ingest_diagnostic();
        let arg_book = response
            .iter()
            .find(|b| b.book_id == "advanced_race_guide")
            .expect("advanced_race_guide present");

        assert_eq!(
            arg_book.content_kind_counts["feats"],
            arg::feats::feat_tables().len() as u32
        );
        assert_eq!(arg_book.content_kind_counts["feats"], 187);
        assert_eq!(
            arg_book.content_kind_counts["spells"],
            arg::spell_list::SPELL_LIST.len() as u32
        );
        assert_eq!(arg_book.content_kind_counts["spells"], 92);
        assert_eq!(
            arg_book.content_kind_counts["equipment"],
            arg::equipment_tables::equipment_tables().len() as u32
        );
        assert_eq!(arg_book.content_kind_counts["equipment"], 200);

        // ARG's racial traits are deliberately not a row — they have no
        // `rules_tables` module, which is what this panel counts. Pinned so
        // the boundary is a decision on record rather than something a later
        // reader has to infer from an absence.
        assert!(
            !arg_book.content_kind_counts.contains_key("race_traits"),
            "ARG's 156 racial-trait records are corpus-JSON-only; see the module doc for why \
             they are accounted for in LICENSE.json rather than here"
        );
    }

    /// PU's rows, same rule. The class and class-feature counts are the ones
    /// that matter most: they are what the SD-27 Unchained wiring added, and
    /// they are invisible to `reach_gate`'s source scanner (its records live
    /// inside accessor function bodies, not in column-zero `pub const`
    /// slices), so this diagnostic is the only automated inventory that sees
    /// them at all.
    #[test]
    fn pathfinder_unchained_counts_match_the_real_underlying_tables() {
        let response = build_corpus_ingest_diagnostic();
        let pu_book = response
            .iter()
            .find(|b| b.book_id == "pathfinder_unchained")
            .expect("pathfinder_unchained present");

        assert_eq!(
            pu_book.content_kind_counts["classes"],
            pu::class_chassis::PuClassId::ALL.len() as u32
        );
        assert_eq!(pu_book.content_kind_counts["classes"], 4);
        assert_eq!(
            pu_book.content_kind_counts["class_features"],
            pu_class_feature_count()
        );
        assert_eq!(pu_book.content_kind_counts["class_features"], 64);
        assert_eq!(
            pu_book.content_kind_counts["equipment"],
            pu::equipment_tables::equipment_tables().len() as u32
        );
        assert_eq!(pu_book.content_kind_counts["equipment"], 42);
        assert_eq!(
            pu_book.content_kind_counts["feats"],
            pu::feat_tables::feat_tables().len() as u32
        );
        assert_eq!(pu_book.content_kind_counts["feats"], 17);
    }

    /// The two SD-27 books' reported totals must equal the licensed content
    /// records their own `data/corpus/<book>/LICENSE.json` accounts for.
    ///
    /// The two artifacts are derived from completely different sources — this
    /// diagnostic counts compiled tables, `LICENSE.json` counts files on disk
    /// — so agreement between them is real evidence rather than a tautology,
    /// and a table that silently loses records shows up as a mismatch here.
    ///
    /// ARG's 156 corpus-JSON-only racial traits are the one declared
    /// difference, stated as a number here rather than waved at, so the two
    /// artifacts reconcile exactly.
    #[test]
    fn the_two_sd27_books_totals_reconcile_with_their_license_artifacts() {
        for (book_id, corpus_dir, corpus_only_records) in [
            ("advanced_race_guide", "advanced_race_guide", 156u32),
            ("pathfinder_unchained", "pathfinder_unchained", 0),
        ] {
            let response = build_corpus_ingest_diagnostic();
            let book = response
                .iter()
                .find(|b| b.book_id == book_id)
                .unwrap_or_else(|| panic!("{book_id} present"));
            let reported: u32 = book.content_kind_counts.values().sum();

            let license_path = repo_root()
                .join("data/corpus")
                .join(corpus_dir)
                .join("LICENSE.json");
            let license: serde_json::Value = serde_json::from_str(
                &std::fs::read_to_string(&license_path)
                    .unwrap_or_else(|e| panic!("{} readable: {e}", license_path.display())),
            )
            .expect("LICENSE.json is valid JSON");
            let licensed = license["records_processed"]
                .as_u64()
                .expect("records_processed is an integer") as u32;

            assert_eq!(
                reported + corpus_only_records,
                licensed,
                "{book_id}: this diagnostic reports {reported} records from rules_tables plus \
                 {corpus_only_records} known corpus-only records, but {} accounts for \
                 {licensed}. One of the two is stale.",
                license_path.display()
            );
        }
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

