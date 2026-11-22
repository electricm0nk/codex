//! SD-37 E4a.3: no table citation names a PCGen list file.
//!
//! Table rows cite where they came from by a source-file stem and a line. SD-36's D6 ruling
//! parked the literal list-file extension on those citations until the tables became a data
//! package. This suite is the control that keeps it burned down: the literal must not appear in
//! whatever remains of the compiled tables or in any file of the shipped data package.
//!
//! What the scan does not cover: it matches the extension literal only. A stem such as a book
//! prefix plus a content word is still a citation of a source file; the stems are kept on
//! purpose (they are the join key the generators check each row against).

use std::path::{Path, PathBuf};

const EXTENSION: &str = ".lst";

fn files_under(root: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(root) else { return };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            files_under(&path, out);
        } else {
            out.push(path);
        }
    }
}

/// Lines containing the extension literal, summed over every file under `root`.
fn citation_lines(root: &Path) -> usize {
    let mut files = Vec::new();
    files_under(root, &mut files);
    files
        .iter()
        .map(|f| {
            let bytes = std::fs::read(f).expect("read file");
            String::from_utf8_lossy(&bytes).lines().filter(|l| l.contains(EXTENSION)).count()
        })
        .sum()
}

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn the_scan_counts_a_planted_citation() {
    let dir = std::env::temp_dir().join(format!("codex-e4a3-plant-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("nested")).unwrap();
    std::fs::write(dir.join("nested/a.json"), "{\n  \"source_file\": \"b1_races.lst\"\n}\n").unwrap();
    std::fs::write(dir.join("b.rs"), "source_file: \"b1_races\",\n").unwrap();
    assert_eq!(citation_lines(&dir), 1);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_compiled_tables_hold_no_citation_literal() {
    let dir = repo().join("src/rules_core/rules_tables");
    if !dir.exists() {
        return;
    }
    assert_eq!(citation_lines(&dir), 0, "compiled tables still cite the list-file extension");
}

#[test]
fn the_shipped_data_package_holds_no_citation_literal() {
    let package = repo().join("data/rules_tables");
    assert!(package.is_dir(), "the data package must exist");
    // The package is shipped because tauri.conf.json bundles it; check that the bundle entry
    // still points at this directory so the scan covers what ships.
    let conf = std::fs::read_to_string(repo().join("apps/desktop/src-tauri/tauri.conf.json")).unwrap();
    assert!(conf.contains("../../../data/rules_tables/"), "the package must stay bundled");
    assert_eq!(citation_lines(&package), 0, "the data package still cites the list-file extension");

    // The package's schema ships beside it and is generated from the same row types, whose doc
    // comments become its descriptions.
    let schema = repo().join("schemas/rules/rules_tables.schema.json");
    let text = std::fs::read_to_string(&schema).expect("read the package schema");
    assert_eq!(
        text.lines().filter(|l| l.contains(EXTENSION)).count(),
        0,
        "the package schema still cites the list-file extension"
    );
}
