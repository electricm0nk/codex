//! SD-37 E4a.4a: the six table generators write the `rules_tables` data package
//! (`data/rules_tables/<id>.json`), never compiled source.
//!
//! E4a.4 removed `src/rules_core/rules_tables/`; six generators still wrote `.rs` files into it
//! (five failed at the write, one recreated the directory). These tests pin both halves of the fix:
//!
//! * the writer every Rust generator now calls (`codex_ingest::rules_package_out`) produces the
//!   package's own canonical file for a table id, at the package's own path, refuses a Product
//!   Identity hit before writing anything, and the file loads back as its row type;
//! * no generator source names the removed directory as a write target.
//!
//! The generators' byte-identity against the shipped package is proven by re-running each against
//! the pinned oracle (the E4a.4a receipt), not here: a unit test has no corpus.

use codex::rules_core::rules_catalog::feats_all::FeatCatalogRecord;
use codex::rules_core::rules_data_package::{load_table_from, render_table};
use codex_ingest::rules_package_out::{write_screened_table, write_table, WriteRefused};
use std::path::{Path, PathBuf};

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("codex-e4a4a-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn rows() -> Vec<FeatCatalogRecord> {
    vec![FeatCatalogRecord { key: "Alpha", category: "General", name: "Alpha", description: Some("A plain feat.") }]
}

#[test]
fn a_written_table_is_the_canonical_package_file_at_its_id_path_and_loads_as_its_row_type() {
    let root = scratch("write");
    let id = "feat_gap_tables/TEST_FEAT_GAP_ROWS";
    let path = write_table(&root, id, &rows()).expect("write");
    assert_eq!(path, root.join("feat_gap_tables/TEST_FEAT_GAP_ROWS.json"));
    assert_eq!(std::fs::read_to_string(&path).unwrap(), render_table(id, &rows()));
    let loaded: &'static [FeatCatalogRecord] = load_table_from(&root, id).expect("loads as its row type");
    assert_eq!(loaded, rows().as_slice());
    std::fs::remove_dir_all(&root).unwrap();
}

#[test]
fn a_screened_write_refuses_a_product_identity_term_and_writes_nothing() {
    let root = scratch("refuse");
    let id = "feat_gap_tables/TEST_FEAT_GAP_ROWS";
    let bad = vec![FeatCatalogRecord { key: "Blessed", category: "General", name: "Blessed", description: Some("Granted by Iomedae herself.") }];
    match write_screened_table(&root, id, &bad) {
        Err(WriteRefused::ProductIdentity(hits)) => assert!(!hits.is_empty()),
        other => panic!("expected a PI refusal, got {other:?}"),
    }
    assert!(!root.join("feat_gap_tables/TEST_FEAT_GAP_ROWS.json").exists(), "a refused table must not be written");
    std::fs::remove_dir_all(&root).unwrap();
}

/// The card's acceptance predicate, as a test: no generator under `crates/codex-ingest/src/bin`
/// or `scripts/` (tests excluded) names the removed compiled directory as a path literal.
#[test]
fn no_generator_writes_into_the_removed_compiled_tables_directory() {
    let repo = codex_ingest::repo_root();
    let mut offenders = Vec::new();
    for dir in ["crates/codex-ingest/src/bin", "scripts"] {
        walk(&repo.join(dir), &mut |path| {
            let rel = path.strip_prefix(&repo).unwrap().to_string_lossy().replace('\\', "/");
            if rel.starts_with("scripts/tests/") || !(rel.ends_with(".rs") || rel.ends_with(".py")) {
                return;
            }
            let text = std::fs::read_to_string(path).unwrap_or_default();
            if text.contains("\"src/rules_core/rules_tables/") || text.contains("f\"src/rules_core/rules_tables/") {
                offenders.push(rel);
            }
        });
    }
    offenders.sort();
    assert!(offenders.is_empty(), "generators still writing compiled source into the removed directory: {offenders:?}");
}

fn walk(dir: &Path, visit: &mut dyn FnMut(&Path)) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk(&path, visit);
        } else {
            visit(&path);
        }
    }
}
