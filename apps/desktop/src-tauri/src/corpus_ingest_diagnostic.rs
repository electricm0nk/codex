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
//! # SD-27 (2026-07-31): the panel could not tell you which book a race came from
//!
//! `races` was a Core Rulebook row only, read from `RaceId::ALL`, because that
//! is the only race roster with a `rules_tables` representation. Bestiary 1
//! showed `monsters: 41` and no race count whatsoever — while 18 races were
//! creatable in the app and **11 of them were Bestiary 1's**. Read literally,
//! the panel said Bestiary 1 shipped no playable race.
//!
//! `races` is now reported per book, derived from the same on-disk race corpus
//! the race catalog and the character-creation roster read, so the panel and
//! the creation screen cannot disagree about what exists. See
//! [`race_counts_by_diagnostic_book`] for why that one row is corpus-derived
//! rather than table-derived, and what it does when the corpus is unreachable.
//!
//! That does widen the caption's "landed in `rules_tables`" boundary by
//! exactly one content kind, deliberately and only where the alternative was a
//! false statement: a `rules_tables`-only race count cannot be honest, because
//! for eleven of the eighteen races there is no `rules_tables` row to count.
//!
//! ## What this panel is still not counting, stated rather than left implicit
//!
//! ARG's **156 alternate/default racial-trait records are not a row here**,
//! and that is a boundary, not an oversight. (Distinct from the `races` row
//! added above: ARG contributes 0 *races*, which is reported, and 156 racial
//! *traits*, which are not.) They have no `rules_tables`
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
//! **That open finding is closed.** It read: of those 156 records, 3 reach no
//! player surface at all — `Feral ~ Languages` and
//! `Scion of Humanity ~ Languages` (`TraitRole::Unclassified`: no gate the
//! resolver can read) and `Saltbeard ~ Dwarf ~ Greed`
//! (`TraitRole::FlagGranted`), all three dropped by
//! `race_trait_picker::build_menu`, which filters to `Default | Alternate`.
//! All three are now `TraitRole::FlagGranted` and reach the player through
//! `resolve_race_alternate_selection`'s applied rows, which
//! AlternateTraitPicker.tsx renders — the menu is still `Default | Alternate`,
//! and it was never the only surface. See `reach_gate::race_traits_reach` and
//! `tests/sd27_ability_automatic_granted_race_traits.rs`.
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
use codex::rules_core::rules_tables::ultimate_campaign as uca;
use codex::rules_core::rules_tables::ultimate_equipment as ue;
use codex::rules_core::rules_tables::ultimate_combat as uc;
use codex::rules_core::rules_tables::ultimate_magic as um;
use codex::rules_core::rules_tables::ultimate_psionics as upsi;
use codex::rules_core::rules_tables::ultimate_wilderness as uw;
use codex::rules_core::rules_tables::ultimate_intrigue as ui;

use crate::race_catalog::{book_code, build_race_catalog, RACE_CORPUS_BOOKS};

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

/// Real creatable-race counts, per book, derived from the same on-disk race
/// corpus the race catalog and character creation both read.
///
/// # Why this is not a `rules_tables` count like every other row here
///
/// It cannot be. Only the Core Rulebook's seven races have a `rules_tables`
/// representation (`RaceId::ALL`); Bestiary 1's eleven were ingested straight
/// to `data/corpus/beastiary/race/` and are read at runtime by
/// `race_resolver::load_race_corpus`, which is the path `race_catalog.rs` and
/// `character_hub`'s creation roster use to put them on screen. So this panel
/// reported CRB `races: 7` and, for Bestiary 1, `monsters: 41` and no race
/// count at all — while 18 races were creatable in the app and 11 of them were
/// Bestiary 1's. A tester reading the panel would conclude Bestiary 1 shipped
/// no playable race.
///
/// Reading the live corpus rather than `RaceId::ALL` also makes the two agree
/// by construction rather than by coincidence;
/// `crb_race_count_agrees_with_the_compiled_race_id_table` pins that.
///
/// Every book in `RACE_CORPUS_BOOKS` gets an entry here, including a genuine
/// zero for ARG (`decisions.md §25.2` — all 37 rows in `arg_races.lst` are
/// `.MOD` reprints of chassis owned by other books). `book_status` is what
/// decides not to *render* a zero, so this map stays a straight measurement
/// and the presentation rule lives in one place.
///
/// Returns an empty map when the corpus cannot be loaded at all (a packaged
/// build with no `data/corpus/` alongside it). Callers then omit the `races`
/// row rather than report a fabricated zero — the same graceful-degradation
/// shape `last_ingested_at` already uses for an unreachable git history.
fn race_counts_by_diagnostic_book() -> BTreeMap<String, u32> {
    let mut counts: BTreeMap<String, u32> = BTreeMap::new();
    let catalog = build_race_catalog();
    if catalog.entries.is_empty() {
        // No on-disk corpus (a packaged build). The Core Rulebook's seven
        // races are additionally compiled into this binary as `RaceId::ALL`,
        // so that one row is still answerable and is answered; the books whose
        // races exist only as corpus JSON honestly report nothing rather than
        // a fabricated zero.
        counts.insert("crb".to_string(), RaceId::ALL.len() as u32);
        return counts;
    }
    for book in RACE_CORPUS_BOOKS {
        counts.insert(diagnostic_book_id(&book_code(book)), 0);
    }
    let mut seen: BTreeMap<String, std::collections::BTreeSet<String>> = BTreeMap::new();
    for entry in &catalog.entries {
        seen.entry(diagnostic_book_id(&entry.book))
            .or_default()
            .insert(entry.race_id.clone());
    }
    for (book_id, races) in seen {
        counts.insert(book_id, races.len() as u32);
    }
    counts
}

/// Maps a race-catalog wire book code (`"CRB"`, `"B1"`, `"ARG"`) onto this
/// panel's own book identifier. An unrecognized code passes through verbatim
/// rather than being silently re-attributed, so a newly ingested book shows up
/// as itself and trips
/// `tests::every_book_landed_in_rules_tables_is_reported` instead of quietly
/// landing in the wrong row.
fn diagnostic_book_id(race_catalog_book_code: &str) -> String {
    match race_catalog_book_code {
        "CRB" => "crb".to_string(),
        "B1" => "beastiary1".to_string(),
        "ARG" => "advanced_race_guide".to_string(),
        other => other.to_string(),
    }
}

fn crb_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("classes".to_string(), ClassId::ALL.len() as u32);
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

/// Ultimate Campaign: SD-28 Epic 13 (`epic-13-calibration`) cost
/// calibration book. 23 real corpus records, all `feats` -- see
/// `ultimate_campaign::feat_tables`'s own doc comment for the catalog and
/// its 3 `deferred-with-reason` records (still counted here: they are real
/// ingested rows, not stubs -- see that module's own doc comment).
fn ultimate_campaign_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("feats".to_string(), uca::feat_tables::feat_tables().len() as u32);
    counts
}

/// Ultimate Intrigue: SD-28 Epic 24 (`epic-24-ui-complete`) from-scratch
/// book ingest, slices 1-2. 104 feats, 101 spells, 98 equipment (91 + 7
/// equipmods) -- see `ultimate_intrigue::{feat_tables,spell_list,
/// equipment_tables}`'s own doc comments for each catalog. Remaining
/// record families (`class_feature`, races, etc.) are not yet ingested
/// and are honestly absent from this map rather than reported as a
/// fabricated zero. (Caught while wiring UE: this map was never updated
/// when slice 2 landed spell/equipment -- fixed here.)
fn ultimate_intrigue_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("feats".to_string(), ui::feat_tables::feat_tables().len() as u32);
    counts.insert("spells".to_string(), ui::spell_list::SPELL_LIST.len() as u32);
    counts.insert(
        "equipment".to_string(),
        (ui::equipment_tables::equipment_tables().len() + ui::equipment_tables::equipmod_tables().len()) as u32,
    );
    counts
}

/// Ultimate Equipment: SD-28 Epic 25 (`epic-25-ue-complete`) from-scratch
/// book ingest, first slice. 1,369 equipment + 180 equipment-modifier
/// records -- see `ultimate_equipment::equipment_tables`'s own doc
/// comment for the catalog and its collision-exclusion ruling.
fn ultimate_equipment_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert(
        "equipment".to_string(),
        (ue::equipment_tables::equipment_tables().len() + ue::equipment_tables::equipmod_tables().len()) as u32,
    );
    counts
}

/// Ultimate Wilderness: SD-28 Epic 26 (`epic-26-uw-complete`) from-scratch
/// book ingest, first slice. 135 feat records -- see
/// `ultimate_wilderness::feat_tables`'s own doc comment for the catalog.
fn ultimate_wilderness_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("feats".to_string(), uw::feat_tables::feat_tables().len() as u32);
    counts
}

/// Ultimate Combat: SD-28 Epic 27 (`epic-27-uc-complete`) from-scratch
/// book ingest, first slice. 263 feat records -- see
/// `ultimate_combat::feat_tables`'s own doc comment for the catalog.
fn ultimate_combat_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("feats".to_string(), uc::feat_tables::feat_tables().len() as u32);
    counts
}

/// Ultimate Magic: SD-28 Epic 28 (`epic-28-um-complete`) from-scratch
/// book ingest, first slice. 144 feat records -- see
/// `ultimate_magic::feat_tables`'s own doc comment for the catalog.
fn ultimate_magic_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("feats".to_string(), um::feat_tables::feat_tables().len() as u32);
    counts
}

/// Ultimate Psionics: SD-28 Epic 29 (`epic-29-upsi-complete`) from-scratch
/// book ingest, first slice, and the last Ultimate book. 221 feat
/// records -- see `ultimate_psionics::feat_tables`'s own doc comment for
/// the catalog and the license-posture check.
fn ultimate_psionics_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("feats".to_string(), upsi::feat_tables::feat_tables().len() as u32);
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

/// The race count is passed in rather than looked up per book so the whole
/// panel reads the corpus once, and so this stays a pure function the tests can
/// drive with a known map.
fn book_status(
    book_id: &str,
    repo_relative_dir: &str,
    mut counts: BTreeMap<String, u32>,
    race_counts: &BTreeMap<String, u32>,
) -> BookIngestStatus {
    // A zero is dropped rather than reported. This panel's own contract —
    // pinned by `every_book_is_populated_with_real_nonzero_counts` — is that a
    // content-kind row means real records; ARG genuinely declares zero races
    // (`decisions.md §25.2`), and a `races: 0` row would be the one row on the
    // screen that means something different from all the others.
    if let Some(races) = race_counts.get(book_id).filter(|races| **races > 0) {
        counts.insert("races".to_string(), *races);
    }
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
    let races = race_counts_by_diagnostic_book();
    vec![
        book_status("crb", "src/rules_core/rules_tables/crb", crb_counts(), &races),
        book_status("apg", "src/rules_core/rules_tables/apg", apg_counts(), &races),
        book_status("acg", "src/rules_core/rules_tables/acg", acg_counts(), &races),
        book_status(
            "beastiary1",
            "src/rules_core/rules_tables/beastiary1",
            beastiary1_counts(),
            &races,
        ),
        book_status(
            "advanced_race_guide",
            "src/rules_core/rules_tables/advanced_race_guide",
            advanced_race_guide_counts(),
            &races,
        ),
        book_status(
            "pathfinder_unchained",
            "src/rules_core/rules_tables/pathfinder_unchained",
            pathfinder_unchained_counts(),
            &races,
        ),
        book_status(
            "ultimate_campaign",
            "src/rules_core/rules_tables/ultimate_campaign",
            ultimate_campaign_counts(),
            &races,
        ),
        book_status(
            "ultimate_intrigue",
            "src/rules_core/rules_tables/ultimate_intrigue",
            ultimate_intrigue_counts(),
            &races,
        ),
        book_status(
            "ultimate_equipment",
            "src/rules_core/rules_tables/ultimate_equipment",
            ultimate_equipment_counts(),
            &races,
        ),
        book_status(
            "ultimate_wilderness",
            "src/rules_core/rules_tables/ultimate_wilderness",
            ultimate_wilderness_counts(),
            &races,
        ),
        book_status(
            "ultimate_combat",
            "src/rules_core/rules_tables/ultimate_combat",
            ultimate_combat_counts(),
            &races,
        ),
        book_status(
            "ultimate_magic",
            "src/rules_core/rules_tables/ultimate_magic",
            ultimate_magic_counts(),
            &races,
        ),
        book_status(
            "ultimate_psionics",
            "src/rules_core/rules_tables/ultimate_psionics",
            ultimate_psionics_counts(),
            &races,
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
                "pathfinder_unchained",
                "ultimate_campaign",
                "ultimate_intrigue",
                "ultimate_equipment",
                "ultimate_wilderness",
                "ultimate_combat",
                "ultimate_magic",
                "ultimate_psionics"
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
        // `races` is now read off the on-disk race corpus rather than off
        // `RaceId::ALL` (see `race_counts_by_diagnostic_book`). Keeping this
        // assertion turns it into the agreement pin between the two sources:
        // the compiled table and the corpus must answer the same for CRB, or
        // one of them has drifted.
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
        // SD28-E16 subset 09 (2026-08-07) raised this from 41 to 46
        // (Lion, Ogre, Pegasus, Rust Monster, Shadow —
        // `beastiary1::mod.rs`'s own subset roster doc comments carry the
        // current count; `docs/architecture/rules-data-tables.md`'s "41
        // monsters" figure predates this subset and is stale, flagged here
        // rather than edited -- that doc is outside this cycle's write
        // scope).
        let response = build_corpus_ingest_diagnostic();
        let bestiary = response
            .iter()
            .find(|b| b.book_id == "beastiary1")
            .expect("beastiary1 present");
        assert_eq!(bestiary.content_kind_counts["monsters"], 46);
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

    /// The defect: the panel showed CRB `races: 7` and, for Bestiary 1,
    /// `monsters: 41` with no race count at all — while 18 races were
    /// creatable and 11 of them were Bestiary 1's.
    #[test]
    fn every_book_that_declares_races_reports_how_many_it_declares() {
        let response = build_corpus_ingest_diagnostic();
        let races = |book_id: &str| -> Option<u32> {
            response
                .iter()
                .find(|book| book.book_id == book_id)
                .and_then(|book| book.content_kind_counts.get("races").copied())
        };

        assert_eq!(races("crb"), Some(7), "the Core Rulebook's seven races");
        assert_eq!(
            races("beastiary1"),
            Some(11),
            "Bestiary 1's eleven races reached the creation screen long before they \
             reached this panel"
        );
        assert_eq!(
            races("advanced_race_guide"),
            None,
            "ARG declares zero races of its own (`decisions.md §25.2`), and this panel's \
             convention is that a content-kind row means real records — so the honest \
             rendering of zero here is no row, not `races: 0`"
        );
    }

    /// The number the panel reports must be the number a player can actually
    /// create, so the two surfaces cannot drift apart.
    #[test]
    fn the_panels_race_total_equals_the_race_catalogs_own_creatable_total() {
        let response = build_corpus_ingest_diagnostic();
        let panel_total: u32 = response
            .iter()
            .filter_map(|book| book.content_kind_counts.get("races").copied())
            .sum();

        let catalog = build_race_catalog();
        let creatable: BTreeSet<&str> =
            catalog.entries.iter().map(|entry| entry.race_id.as_str()).collect();

        assert_eq!(
            panel_total as usize,
            creatable.len(),
            "the panel's per-book race counts must sum to exactly the races the catalog serves"
        );
        assert_eq!(panel_total, 18, "18 in-scope races today: CRB's 7 plus Bestiary 1's 11");
    }

    /// A book with no race content must not grow a misleading `races: 0` row —
    /// only the books the race corpus is actually searched for get one.
    #[test]
    fn books_the_race_corpus_does_not_cover_carry_no_race_row_at_all() {
        let response = build_corpus_ingest_diagnostic();
        for book_id in ["apg", "acg", "pathfinder_unchained"] {
            let book = response
                .iter()
                .find(|book| book.book_id == book_id)
                .unwrap_or_else(|| panic!("{book_id} present"));
            assert!(
                !book.content_kind_counts.contains_key("races"),
                "{book_id} is not a race corpus book; a `races` row here would be an \
                 unmeasured zero dressed as a measurement"
            );
        }
    }

    /// `book_status` is pure over the race map, so the "corpus unreachable"
    /// branch is a real, driven case rather than an untested `if`.
    #[test]
    fn a_book_with_no_entry_in_the_race_map_reports_no_race_row() {
        let status = book_status(
            "beastiary1",
            "src/rules_core/rules_tables/beastiary1",
            beastiary1_counts(),
            &BTreeMap::new(),
        );
        assert!(
            !status.content_kind_counts.contains_key("races"),
            "an unreachable corpus must omit the row, never report a fabricated zero"
        );
        assert_eq!(status.content_kind_counts["monsters"], 46);
    }

    #[test]
    fn the_race_catalog_book_codes_map_onto_this_panels_book_ids() {
        assert_eq!(diagnostic_book_id("CRB"), "crb");
        assert_eq!(diagnostic_book_id("B1"), "beastiary1");
        assert_eq!(diagnostic_book_id("ARG"), "advanced_race_guide");
        assert_eq!(
            diagnostic_book_id("UM"),
            "UM",
            "an unrecognized book passes through verbatim rather than landing in the wrong row"
        );
    }
}
