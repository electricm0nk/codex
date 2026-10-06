//! Write or check the `rules_tables` data package (`data/rules_tables/`) — SD-37 E4a.1.
//!
//! The package is rendered from the compiled tables by
//! `codex::rules_core::rules_data_package::render_package`: one JSON file per table, each with
//! its licence/PI stamp.
//!
//! Usage: `rules_tables_package (--write | --check) [--root <package dir>]`
//!
//! * `--write` writes every file and deletes any `*.json` under the root that no table owns.
//! * `--check` writes nothing; it exits 1 if any file is missing, differs, or is not owned.
//!
//! Prints one summary line: `rules_tables_package: tables=<n> rows_bytes=<b> pi_files=<p> verdict=<PASS|FAIL|WROTE>`.

use codex::rules_core::rules_data_package::{package_root, render_package, PackageLicense, LicenceStamp};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn json_files(root: &Path) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "json") {
                out.insert(path.strip_prefix(root).unwrap().to_string_lossy().replace('\\', "/"));
            }
        }
    }
    out
}

fn main() -> ExitCode {
    let mut mode = None;
    let mut root: Option<PathBuf> = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--write" | "--check" => mode = Some(arg),
            "--root" => root = args.next().map(PathBuf::from),
            other => {
                eprintln!("rules_tables_package: unknown argument {other}");
                return ExitCode::from(2);
            }
        }
    }
    let Some(mode) = mode else {
        eprintln!("rules_tables_package: pass --write or --check");
        return ExitCode::from(2);
    };
    let root = root.unwrap_or_else(|| package_root(Path::new(env!("CARGO_MANIFEST_DIR"))));

    let files = render_package();
    let owned: BTreeSet<String> = files.iter().map(|(rel, _)| rel.clone()).collect();
    let bytes: usize = files.iter().map(|(_, text)| text.len()).sum();
    let pi_files = files
        .iter()
        .filter(|(_, text)| {
            let value: serde_json::Value = serde_json::from_str(text).expect("rendered JSON parses");
            let stamp: LicenceStamp = serde_json::from_value(value["licence"].clone()).expect("stamp");
            stamp.license == PackageLicense::Pi
        })
        .count();

    if mode == "--write" {
        for (rel, text) in &files {
            let path = root.join(rel);
            std::fs::create_dir_all(path.parent().unwrap()).expect("create the table's directory");
            std::fs::write(&path, text).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        }
        for stale in json_files(&root).difference(&owned) {
            std::fs::remove_file(root.join(stale)).expect("remove a file no table owns");
            println!("removed {stale}");
        }
        println!("rules_tables_package: tables={} rows_bytes={bytes} pi_files={pi_files} verdict=WROTE", files.len());
        return ExitCode::SUCCESS;
    }

    let mut problems = Vec::new();
    for (rel, text) in &files {
        match std::fs::read_to_string(root.join(rel)) {
            Ok(on_disk) if &on_disk == text => {}
            Ok(_) => problems.push(format!("DIFFERS {rel}")),
            Err(_) => problems.push(format!("MISSING {rel}")),
        }
    }
    for stale in json_files(&root).difference(&owned) {
        problems.push(format!("NOT OWNED {stale}"));
    }
    for p in &problems {
        println!("{p}");
    }
    let verdict = if problems.is_empty() { "PASS" } else { "FAIL" };
    println!("rules_tables_package: tables={} rows_bytes={bytes} pi_files={pi_files} verdict={verdict}", files.len());
    if problems.is_empty() { ExitCode::SUCCESS } else { ExitCode::from(1) }
}
