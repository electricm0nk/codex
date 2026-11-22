//! The live side's reader for a book's **settled-record bundle**,
//! `data/corpus/<book>/_settled/<kind>.json`.
//!
//! SD-35 `AT-35-E6-003-RULED` cycle 15, under `decisions.md` §11 ("no PCGen in
//! live code"), `decisions.md` §19 (operator ruling B16 — naming `pcgen_import`
//! in shipping code under a live root is a hit) and `technical-design.md` §0
//! (the boundary is **by path**).
//!
//! # What this closes
//!
//! Cycles 13 and 14 settled the equipment and race kinds: the live side stopped
//! holding ingest token arrays and started holding
//! [`CorpusEquipmentRecord`](crate::rules_core::equipment_record::CorpusEquipmentRecord),
//! [`CorpusRaceRecord`](crate::rules_core::race_record::CorpusRaceRecord) and
//! [`CorpusRaceTraitRecord`](crate::rules_core::race_record::CorpusRaceTraitRecord).
//! What did **not** move was *who performs the reading*: the live loader still
//! called `pcgen_import::corpus_equipment_json` / `::corpus_race_json` at run
//! time to turn one corpus record's `data` object into a settled record. Both
//! cycles booked that honestly as the remaining `corpus_json_boundary` group and
//! named its clearing condition in the same words: *"it clears when
//! `data/corpus/` carries the settled fields itself."*
//!
//! This module is the live half of that. The conversion happens once, at
//! authoring time, in `src/bin/gen_settled_corpus.rs`; the result is written
//! beside the book's corpus as ordinary data; and the live loader reads it with
//! serde and names nothing.
//!
//! # Why a per-book bundle and not a field on every record
//!
//! Two reasons, both measured rather than preferred:
//!
//! 1. `data/corpus/**` is 227 MB of **ingested** records carrying license and
//!    `pi_*` redaction stamps. Rewriting 8,762 of them in place to add a
//!    settled block is exactly the shape that has destroyed those stamps before
//!    (`docs/retro/` — "generated artifacts mutated post-hoc"). A bundle beside
//!    the records adds files and mutates none.
//! 2. The bundle is keyed by each record's path **relative to its kind
//!    directory**, so the loader keeps its existing file walk, its existing
//!    per-file diagnostics and its existing `source`/`pi_field` reads. Only the
//!    question *"what settled record does this file stand for?"* changes its
//!    answerer, from the converter to serde.
//!
//! # Staleness
//!
//! A bundle that disagrees with the corpus is a **loud** failure, not a silent
//! one: `gen_settled_corpus --check` regenerates every bundle in memory and
//! compares bytes, the same contract `sheet_rule_convert --check` and
//! `gen_desktop_fixture_corpus --check` carry, and a record present on disk with
//! no bundle entry lands as a named loader diagnostic.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::rules_core::equipment_record::CorpusEquipmentRecord;
use crate::rules_core::race_record::{CorpusRaceRecord, CorpusRaceTraitRecord};
use crate::rules_core::source_content::SourceRef;

/// The subdirectory of one book's corpus root that holds its settled bundles.
///
/// It sits **beside** the kind directories (`equipment/`, `race/`,
/// `race_trait/`), not inside one, so every existing corpus walk — which starts
/// at a kind directory — cannot see it and needed no new skip rule.
pub const SETTLED_DIR: &str = "_settled";

/// The three kinds that currently ship a settled bundle.
pub const EQUIPMENT_KIND: &str = "equipment";
pub const RACE_KIND: &str = "race";
pub const RACE_TRAIT_KIND: &str = "race_trait";

/// `<book_dir>/_settled/<kind>.json`.
pub fn settled_bundle_path(book_dir: &Path, kind: &str) -> PathBuf {
    book_dir.join(SETTLED_DIR).join(format!("{kind}.json"))
}

/// One corpus record's path relative to its kind directory, `/`-joined, which
/// is the key every bundle is keyed by.
///
/// `/`-joined rather than platform-joined on purpose: the bundle is committed
/// data read on every platform the desktop ships to, so its keys must not
/// depend on the separator of the machine that wrote it.
pub fn bundle_key(kind_dir: &Path, record_path: &Path) -> Option<String> {
    let rel = record_path.strip_prefix(kind_dir).ok()?;
    let mut parts = Vec::new();
    for part in rel.components() {
        parts.push(part.as_os_str().to_string_lossy().into_owned());
    }
    Some(parts.join("/"))
}

/// The repo-relative corpus root every book directory lives under.
pub const CORPUS_ROOT: &str = "data/corpus";

/// Every book directory under a corpus root, sorted. Used by the authoring-time
/// generator and by the whole-corpus parity proofs, so both cover exactly the
/// books that exist rather than a hand-maintained list.
pub fn corpus_book_dirs(corpus_root: &Path) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(corpus_root) else { return Vec::new() };
    let mut dirs: Vec<PathBuf> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .filter(|path| {
            !path
                .file_name()
                .is_some_and(|name| name.to_string_lossy().starts_with('_'))
        })
        .collect();
    dirs.sort();
    dirs
}

/// One book's settled records for one kind.
///
/// `records` is a `BTreeMap`, so the serialized bytes are key-sorted and
/// byte-stable across checkouts — the same determinism rule
/// `gen_desktop_fixture_corpus` states for its own output.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettledBundle<T> {
    /// The corpus kind this bundle settles (`"equipment"`, `"race"`,
    /// `"race_trait"`), carried so a misfiled bundle is a named error rather
    /// than a silently wrong load.
    pub kind: String,
    /// Record path relative to the kind directory → that record's settled form.
    pub records: BTreeMap<String, T>,
}

impl<T> SettledBundle<T> {
    pub fn new(kind: impl Into<String>, records: BTreeMap<String, T>) -> Self {
        Self { kind: kind.into(), records }
    }
}

/// One settled equipment record plus the provenance anchor the canonical
/// envelope carries.
///
/// The `source_ref` is stored rather than recomputed because it is what the
/// converter's own `convert_equipment_record` put on the envelope, read off the
/// rebuilt ingest row. Recomputing it live would be the live side deriving a
/// converter value again — the precise thing this cycle removes — so it is
/// settled at authoring time with everything else.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettledEquipmentEntry {
    pub source_ref: SourceRef,
    pub record: CorpusEquipmentRecord,
}

/// Why a settled bundle could not be read. Every variant is reported as a
/// loader diagnostic; none of them panics and none of them silently yields an
/// empty book.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettledBundleError {
    /// No `_settled/<kind>.json` beside this book's corpus.
    Missing,
    /// The file exists but could not be read.
    Unreadable(String),
    /// The file exists but is not a `SettledBundle<T>`.
    Malformed(String),
    /// The file is a bundle for a different kind.
    WrongKind { found: String },
}

impl std::fmt::Display for SettledBundleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing => write!(
                f,
                "no settled-record bundle; run `cargo run --locked --bin gen_settled_corpus`"
            ),
            Self::Unreadable(err) => write!(f, "settled-record bundle could not be read: {err}"),
            Self::Malformed(err) => write!(f, "settled-record bundle is malformed: {err}"),
            Self::WrongKind { found } => {
                write!(f, "settled-record bundle declares kind {found:?}")
            }
        }
    }
}

/// Read one book's settled bundle for one kind. Pure serde: this function names
/// no converter module, reads no token and re-derives no value.
pub fn read_settled_bundle<T: serde::de::DeserializeOwned>(
    book_dir: &Path,
    kind: &str,
) -> Result<SettledBundle<T>, SettledBundleError> {
    let path = settled_bundle_path(book_dir, kind);
    if !path.is_file() {
        return Err(SettledBundleError::Missing);
    }
    let text = std::fs::read_to_string(&path)
        .map_err(|err| SettledBundleError::Unreadable(err.to_string()))?;
    let bundle: SettledBundle<T> = serde_json::from_str(&text)
        .map_err(|err| SettledBundleError::Malformed(err.to_string()))?;
    if bundle.kind != kind {
        return Err(SettledBundleError::WrongKind { found: bundle.kind });
    }
    Ok(bundle)
}

/// [`read_settled_bundle`] for the equipment kind.
pub fn read_equipment_bundle(
    book_dir: &Path,
) -> Result<SettledBundle<SettledEquipmentEntry>, SettledBundleError> {
    read_settled_bundle(book_dir, EQUIPMENT_KIND)
}

/// [`read_settled_bundle`] for the race-chassis kind.
pub fn read_race_bundle(
    book_dir: &Path,
) -> Result<SettledBundle<CorpusRaceRecord>, SettledBundleError> {
    read_settled_bundle(book_dir, RACE_KIND)
}

/// [`read_settled_bundle`] for the racial-trait kind.
pub fn read_race_trait_bundle(
    book_dir: &Path,
) -> Result<SettledBundle<CorpusRaceTraitRecord>, SettledBundleError> {
    read_settled_bundle(book_dir, RACE_TRAIT_KIND)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_bundle_path_sits_beside_the_kind_directories_not_inside_one() {
        let path = settled_bundle_path(Path::new("data/corpus/core_rulebook"), EQUIPMENT_KIND);
        assert_eq!(
            path,
            Path::new("data/corpus/core_rulebook/_settled/equipment.json"),
            "a bundle inside `equipment/` would be walked as a corpus record"
        );
    }

    #[test]
    fn a_bundle_key_is_the_record_path_relative_to_its_kind_dir_slash_joined() {
        let key = bundle_key(
            Path::new("data/corpus/core_rulebook/equipment"),
            Path::new("data/corpus/core_rulebook/equipment/arms_armor/arrow_slaying.json"),
        );
        assert_eq!(key.as_deref(), Some("arms_armor/arrow_slaying.json"));
    }

    #[test]
    fn a_path_outside_the_kind_dir_has_no_bundle_key() {
        assert_eq!(
            bundle_key(
                Path::new("data/corpus/core_rulebook/equipment"),
                Path::new("data/corpus/beastiary/equipment/x.json")
            ),
            None
        );
    }

    #[test]
    fn a_missing_bundle_is_a_named_error_not_an_empty_bundle() {
        let err = read_equipment_bundle(Path::new("data/corpus/nonexistent_book"))
            .expect_err("a book with no bundle must not read as an empty one");
        assert_eq!(err, SettledBundleError::Missing);
    }

    /// The real, on-disk bundle for a real book round-trips and carries the
    /// record the loader's own oldest test resolves.
    #[test]
    fn a_real_on_disk_bundle_round_trips_through_serde() {
        let bundle = read_equipment_bundle(Path::new("data/corpus/advanced_race_guide"))
            .expect("ARG's settled equipment bundle must be on disk");
        assert_eq!(bundle.kind, EQUIPMENT_KIND);
        assert!(!bundle.records.is_empty(), "ARG states real equipment records");
        let dogslicer = bundle
            .records
            .values()
            .find(|entry| entry.record.identity == "Dogslicer")
            .expect("ARG's Dogslicer must be in its own book's bundle");
        assert_eq!(dogslicer.record.weight_lbs, Some(1.0));
        assert_eq!(dogslicer.record.cost_gp, Some(8.0));
    }

    /// A bundle read with the wrong kind is refused rather than half-loaded.
    #[test]
    fn a_bundle_read_as_the_wrong_kind_is_refused() {
        let err = read_settled_bundle::<SettledEquipmentEntry>(
            Path::new("data/corpus/advanced_race_guide"),
            "race",
        );
        // `race.json` deserializes as neither an equipment entry map nor the
        // equipment kind; either refusal is correct, a silent empty load is not.
        assert!(err.is_err(), "reading the race bundle as equipment must fail");
    }
}
