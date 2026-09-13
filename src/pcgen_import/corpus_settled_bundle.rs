//! The **converter half** of the settled-record bundle: it reads a book's
//! `data/corpus/<book>/<kind>/**/*.json` records and produces the
//! `_settled/<kind>.json` bundle the live side reads with serde.
//!
//! SD-35 `AT-35-E6-003-RULED` cycle 15, `decisions.md` §11 ("rule conversion
//! happens at ingest into our own schema") and §19 (ruling B16).
//!
//! # Why this is on the converter side, and the reading is not repeated
//!
//! Every value in a bundle is produced by **the exact call the live loader made
//! at run time before this cycle**, unchanged:
//! [`crate::pcgen_import::corpus_equipment_json::corpus_equipment_source_record`]
//! for the equipment kind and
//! [`crate::pcgen_import::corpus_race_json::corpus_race_source_record`] /
//! [`…::corpus_race_trait_source_record`](crate::pcgen_import::corpus_race_json::corpus_race_trait_source_record)
//! for the race kinds. Nothing is re-derived here, no value is re-parsed here,
//! and no new reading rule is spelled here. What changed is **when** the call
//! happens — once, at authoring time — and therefore **who** has to name the
//! converter to make it: this module, on the converter's own side of the
//! boundary, instead of `rules_core::corpus_loader` on the live side.
//!
//! # Determinism
//!
//! `records` is a `BTreeMap` keyed by each record's path relative to its kind
//! directory, so bundle bytes are key-sorted and identical from any checkout.
//! `serde_json`'s object type is a `BTreeMap` in this build (no
//! `preserve_order`), so nested object key order is sorted too. That is what
//! makes `--check` a byte comparison rather than a semantic one.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::pcgen_import::corpus_equipment_json::corpus_equipment_source_record;
use crate::pcgen_import::corpus_race_json::{
    corpus_race_source_record, corpus_race_trait_source_record,
};
use crate::rules_core::settled_corpus::{
    bundle_key, settled_bundle_path, SettledBundle, SettledEquipmentEntry, EQUIPMENT_KIND,
    RACE_KIND, RACE_TRAIT_KIND,
};
use crate::rules_core::source_content::SourceContentPayload;

/// One bundle this tool would write: its path and its exact bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleArtifact {
    pub path: PathBuf,
    pub text: String,
    /// How many records the bundle carries.
    pub records: usize,
    /// Corpus files under the kind directory that produced no settled record —
    /// reported, never hidden. These are the same files the live loader
    /// reported as malformed-record diagnostics before this cycle.
    pub skipped: Vec<String>,
}

/// Every corpus record file under `dir`, by the traversal rule
/// `rules_core::corpus_loader::find_json_files` and
/// `rules_core::race_resolver::find_json_files` both state: recurse, skip
/// `_parity/`, skip `LICENSE.json`, take `*.json`, sorted.
///
/// Restated here rather than borrowed because those two are private to their
/// live modules; the rule is asserted against the real corpus by
/// `the_generator_walks_exactly_the_files_the_live_loader_walks` below.
fn find_json_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&current) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();
            if path.is_dir() {
                if file_name == "_parity" {
                    continue;
                }
                stack.push(path);
            } else if file_name == "LICENSE.json" {
                continue;
            } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

fn read_data_object(path: &Path) -> Option<serde_json::Value> {
    let text = std::fs::read_to_string(path).ok()?;
    let value: serde_json::Value = serde_json::from_str(&text).ok()?;
    value.get("data").cloned()
}

fn render(bundle_kind: &str, records: BTreeMap<String, serde_json::Value>) -> String {
    let bundle = SettledBundle::new(bundle_kind, records);
    let mut text = serde_json::to_string_pretty(&bundle).expect("a settled bundle serializes");
    text.push('\n');
    text
}

/// Build every settled bundle one book's corpus root states, in a stable order.
/// A kind directory the book does not have contributes nothing.
pub fn bundles_for_book(book_dir: &Path) -> Vec<BundleArtifact> {
    let mut out = Vec::new();
    if let Some(artifact) = equipment_bundle(book_dir) {
        out.push(artifact);
    }
    if let Some(artifact) = race_bundle(book_dir) {
        out.push(artifact);
    }
    if let Some(artifact) = race_trait_bundle(book_dir) {
        out.push(artifact);
    }
    out
}

/// The equipment bundle for one book, or `None` when the book has no
/// `equipment/` directory.
pub fn equipment_bundle(book_dir: &Path) -> Option<BundleArtifact> {
    let kind_dir = book_dir.join(EQUIPMENT_KIND);
    if !kind_dir.is_dir() {
        return None;
    }
    let mut records = BTreeMap::new();
    let mut skipped = Vec::new();
    for path in find_json_files(&kind_dir) {
        let Some(key) = bundle_key(&kind_dir, &path) else { continue };
        let Some(data) = read_data_object(&path) else {
            skipped.push(key);
            continue;
        };
        // THE call the live loader made at run time before this cycle, byte
        // for byte. The envelope it returns carries both halves the live side
        // needs: the provenance anchor and the settled record.
        let Some(envelope) = corpus_equipment_source_record(&data) else {
            skipped.push(key);
            continue;
        };
        let SourceContentPayload::Equipment(record) = envelope.payload else {
            skipped.push(key);
            continue;
        };
        let entry = SettledEquipmentEntry { source_ref: envelope.source_ref, record: record.clone() };
        records.insert(key, serde_json::to_value(&entry).expect("a settled entry serializes"));
    }
    let count = records.len();
    Some(BundleArtifact {
        path: settled_bundle_path(book_dir, EQUIPMENT_KIND),
        text: render(EQUIPMENT_KIND, records),
        records: count,
        skipped,
    })
}

/// The race-chassis bundle for one book, or `None` when the book has no
/// `race/` directory.
pub fn race_bundle(book_dir: &Path) -> Option<BundleArtifact> {
    let kind_dir = book_dir.join(RACE_KIND);
    if !kind_dir.is_dir() {
        return None;
    }
    let mut records = BTreeMap::new();
    let mut skipped = Vec::new();
    for path in find_json_files(&kind_dir) {
        let Some(key) = bundle_key(&kind_dir, &path) else { continue };
        let Some(data) = read_data_object(&path) else {
            skipped.push(key);
            continue;
        };
        match corpus_race_source_record(&data) {
            Some(record) => {
                records.insert(key, serde_json::to_value(&record).expect("a race record serializes"));
            }
            None => skipped.push(key),
        }
    }
    let count = records.len();
    Some(BundleArtifact {
        path: settled_bundle_path(book_dir, RACE_KIND),
        text: render(RACE_KIND, records),
        records: count,
        skipped,
    })
}

/// The racial-trait bundle for one book, or `None` when the book has no
/// `race_trait/` directory.
pub fn race_trait_bundle(book_dir: &Path) -> Option<BundleArtifact> {
    let kind_dir = book_dir.join(RACE_TRAIT_KIND);
    if !kind_dir.is_dir() {
        return None;
    }
    let mut records = BTreeMap::new();
    let mut skipped = Vec::new();
    for path in find_json_files(&kind_dir) {
        let Some(key) = bundle_key(&kind_dir, &path) else { continue };
        let Some(data) = read_data_object(&path) else {
            skipped.push(key);
            continue;
        };
        match corpus_race_trait_source_record(&data) {
            Some(record) => {
                records.insert(key, serde_json::to_value(&record).expect("a trait record serializes"));
            }
            None => skipped.push(key),
        }
    }
    let count = records.len();
    Some(BundleArtifact {
        path: settled_bundle_path(book_dir, RACE_TRAIT_KIND),
        text: render(RACE_TRAIT_KIND, records),
        records: count,
        skipped,
    })
}

/// Write every bundle a book states. Returns the artifacts written.
pub fn write_bundles_for_book(book_dir: &Path) -> std::io::Result<Vec<BundleArtifact>> {
    let artifacts = bundles_for_book(book_dir);
    for artifact in &artifacts {
        if let Some(parent) = artifact.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&artifact.path, &artifact.text)?;
    }
    Ok(artifacts)
}

/// Every bundle whose on-disk bytes differ from what this tool would write now.
pub fn drifted_bundles_for_book(book_dir: &Path) -> Vec<PathBuf> {
    bundles_for_book(book_dir)
        .into_iter()
        .filter(|artifact| std::fs::read_to_string(&artifact.path).ok().as_deref() != Some(artifact.text.as_str()))
        .map(|artifact| artifact.path)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::settled_corpus::{read_equipment_bundle, read_race_bundle, read_race_trait_bundle};

    const CRB: &str = "data/corpus/core_rulebook";

    /// The generator's own traversal rule is the live loaders' rule: the real
    /// corpus contains both a `_parity/` directory and `LICENSE.json` files,
    /// and neither reaches a bundle key.
    #[test]
    fn the_generator_walks_exactly_the_files_the_live_loader_walks() {
        let kind_dir = Path::new(CRB).join(EQUIPMENT_KIND);
        let files = find_json_files(&kind_dir);
        assert!(!files.is_empty(), "CRB states real equipment records");
        assert!(
            files.iter().all(|p| p.file_name().is_some_and(|n| n != "LICENSE.json")),
            "LICENSE.json is not a corpus record"
        );
        assert!(
            files.iter().all(|p| !p.components().any(|c| c.as_os_str() == "_parity")),
            "_parity/ is not corpus record data"
        );
        let mut sorted = files.clone();
        sorted.sort();
        assert_eq!(files, sorted, "record order is a property of the corpus, not of readdir");
    }

    /// **The parity proof this cycle rests on**, over the whole live corpus:
    /// every settled equipment record the on-disk bundle carries is
    /// field-for-field the record the run-time call produced, and the
    /// provenance anchor with it. A reading widened, narrowed or reordered by
    /// the move fails here, on the real corpus, not on a fixture.
    #[test]
    fn every_on_disk_equipment_bundle_matches_the_run_time_call_it_replaced() {
        let mut books = 0usize;
        let mut compared = 0usize;
        for book_dir in crate::rules_core::settled_corpus::corpus_book_dirs(std::path::Path::new(crate::rules_core::settled_corpus::CORPUS_ROOT)) {
            let Some(fresh) = equipment_bundle(&book_dir) else { continue };
            let on_disk = read_equipment_bundle(&book_dir)
                .unwrap_or_else(|err| panic!("{}: {err}", book_dir.display()));
            let fresh: SettledBundle<SettledEquipmentEntry> =
                serde_json::from_str(&fresh.text).expect("the freshly built bundle round-trips");
            assert_eq!(
                fresh.records.len(),
                on_disk.records.len(),
                "{}: bundle population drifted from the corpus",
                book_dir.display()
            );
            for (key, entry) in &fresh.records {
                let stored = on_disk
                    .records
                    .get(key)
                    .unwrap_or_else(|| panic!("{}: {key} missing from the on-disk bundle", book_dir.display()));
                assert_eq!(entry, stored, "{}: {key} disagrees with the run-time call", book_dir.display());
                compared += 1;
            }
            books += 1;
        }
        assert!(books >= 20, "every book with an equipment/ dir must be covered, got {books}");
        assert!(compared >= 5_000, "the equipment population is thousands of records, got {compared}");
    }

    /// The same whole-corpus proof for the two race kinds.
    #[test]
    fn every_on_disk_race_bundle_matches_the_run_time_call_it_replaced() {
        let mut chassis = 0usize;
        let mut traits = 0usize;
        for book_dir in crate::rules_core::settled_corpus::corpus_book_dirs(std::path::Path::new(crate::rules_core::settled_corpus::CORPUS_ROOT)) {
            if let Some(fresh) = race_bundle(&book_dir) {
                let on_disk = read_race_bundle(&book_dir)
                    .unwrap_or_else(|err| panic!("{}: {err}", book_dir.display()));
                let fresh: SettledBundle<crate::rules_core::race_record::CorpusRaceRecord> =
                    serde_json::from_str(&fresh.text).expect("round-trips");
                assert_eq!(fresh.records, on_disk.records, "{}", book_dir.display());
                chassis += fresh.records.len();
            }
            if let Some(fresh) = race_trait_bundle(&book_dir) {
                let on_disk = read_race_trait_bundle(&book_dir)
                    .unwrap_or_else(|err| panic!("{}: {err}", book_dir.display()));
                let fresh: SettledBundle<crate::rules_core::race_record::CorpusRaceTraitRecord> =
                    serde_json::from_str(&fresh.text).expect("round-trips");
                assert_eq!(fresh.records, on_disk.records, "{}", book_dir.display());
                traits += fresh.records.len();
            }
        }
        assert!(chassis >= 20, "the live resolver loads real race chassis records, got {chassis}");
        assert!(traits >= 500, "the live resolver loads hundreds of racial traits, got {traits}");
    }

    /// `--check`'s own claim: nothing on disk has drifted from what this tool
    /// would write now.
    #[test]
    fn no_on_disk_bundle_has_drifted_from_the_corpus() {
        let mut drifted = Vec::new();
        for book_dir in crate::rules_core::settled_corpus::corpus_book_dirs(std::path::Path::new(crate::rules_core::settled_corpus::CORPUS_ROOT)) {
            drifted.extend(drifted_bundles_for_book(&book_dir));
        }
        assert!(drifted.is_empty(), "drifted bundles: {drifted:?}");
    }
}
