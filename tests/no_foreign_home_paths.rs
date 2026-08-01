//! Guard: no foreign absolute home directory may be hardcoded under
//! `tests/`, `src/` or `scripts/`.
//!
//! Operator ruling (recorded 2026-08-01):
//!
//! > "If you use ~/workspace you will always be right, no matter which
//! > machine you are working from. I'm putting workspace in the home
//! > directory and keeping it synced with syncthing."
//!
//! Before this guard existed, 49 functional defaults and 21 doc comments
//! named another machine's home directory outright. Three suites
//! (`sd26_pcgen_runner`, `sd27_advanced_race_guide_parity`,
//! `sd27_pathfinder_unchained_parity`) failed on every box but one, and the
//! failures read as environmental rather than as a hardcoded path, which is
//! precisely why they survived so long.
//!
//! ## The `~` trap this guard also closes
//!
//! Rust does **not** expand `~`. A `PathBuf::from` of a tilde-prefixed
//! string literal is a relative directory literally named `~`, which
//! silently does not exist — the same quiet wrongness in a new costume.
//! Rust defaults resolve `$HOME` through [`std::env::var`]; only shell
//! scripts may write `$HOME` textually, and even there a tilde inside
//! quotes does not expand. So this guard rejects tilde-prefixed path
//! literals in Rust source too.
//!
//! ## Scope
//!
//! `tests/`, `src/` and `scripts/` — the surfaces this repo builds and runs
//! from. `docs/` is deliberately out of scope: it carries dated historical
//! records whose value is that they say what was true when they were
//! written.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Assembled at compile time from two fragments so that this file — which is
/// itself inside the scanned tree — does not contain the contiguous literal
/// it searches for. A guard that has to exempt itself is a guard with a hole
/// in it.
const FOREIGN_HOME: &str = concat!("/home/", "ubuntu");

/// The directories under the repo root that this guard scans.
const SCANNED_DIRS: &[&str] = &["tests", "src", "scripts"];

/// Paths (repo-root-relative) exempted from the scan, each with the reason.
///
/// `scripts/verify-baselines.env` is a dated, append-only audit narrative:
/// every entry records what a specific past verification run observed,
/// including `ls -d <foreign home>` reporting "No such file or directory" as
/// the evidence for why five suites failed. Those are findings, not
/// conventions. Rewriting them would falsify the record the file exists to
/// keep, so they stay exactly as they were written.
const ALLOWLIST: &[&str] = &["scripts/verify-baselines.env"];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Recursively collects every regular file under `dir`, skipping build
/// output and VCS metadata.
fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(err) => panic!("guard must be able to read {}: {err}", dir.display()),
    };
    for entry in entries {
        let entry = entry.expect("directory entry must be readable");
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name == "target" || name == ".git" || name == "node_modules" {
            continue;
        }
        let file_type = entry.file_type().expect("file type must be readable");
        if file_type.is_dir() {
            collect_files(&path, out);
        } else if file_type.is_file() {
            out.push(path);
        }
    }
}

/// Every scanned file, as repo-root-relative paths, minus the allowlist.
fn scanned_files() -> Vec<PathBuf> {
    let root = repo_root();
    let allowed: BTreeSet<&str> = ALLOWLIST.iter().copied().collect();
    let mut absolute = Vec::new();
    for dir in SCANNED_DIRS {
        let path = root.join(dir);
        assert!(
            path.is_dir(),
            "scanned directory {} must exist; if it was renamed, update SCANNED_DIRS \
             rather than letting this guard quietly scan nothing",
            path.display()
        );
        collect_files(&path, &mut absolute);
    }
    assert!(
        absolute.len() > 100,
        "guard scanned only {} files across {SCANNED_DIRS:?}, which means the walk broke \
         rather than that the repo shrank",
        absolute.len()
    );
    absolute
        .into_iter()
        .filter_map(|path| {
            let relative = path
                .strip_prefix(&root)
                .expect("scanned path is under the repo root")
                .to_path_buf();
            let key = relative.to_string_lossy().to_string();
            if allowed.contains(key.as_str()) {
                None
            } else {
                Some(relative)
            }
        })
        .collect()
}

/// Reads a file as text, returning `None` for binary content (fixtures under
/// `tests/` include real `.pcg` and cache blobs).
fn read_text(path: &Path) -> Option<String> {
    let bytes = std::fs::read(path).ok()?;
    String::from_utf8(bytes).ok()
}

/// Returns `(relative_path, line_number, line)` for every offending line.
fn offenders(needle: &str) -> Vec<(String, usize, String)> {
    let root = repo_root();
    let mut hits = Vec::new();
    for relative in scanned_files() {
        let Some(text) = read_text(&root.join(&relative)) else {
            continue;
        };
        for (index, line) in text.lines().enumerate() {
            if line.contains(needle) {
                hits.push((
                    relative.to_string_lossy().to_string(),
                    index + 1,
                    line.trim().to_string(),
                ));
            }
        }
    }
    hits
}

/// The guard itself. Any reappearance of another machine's home directory
/// under `tests/`, `src/` or `scripts/` fails here, naming every file and
/// line, rather than surfacing later as five suites that "fail
/// environmentally" on every box but one.
#[test]
fn no_foreign_absolute_home_path_under_tests_src_or_scripts() {
    let hits = offenders(FOREIGN_HOME);
    assert!(
        hits.is_empty(),
        "{} hardcoded `{FOREIGN_HOME}` path(s) found. Defaults must be HOME-relative: \
         resolve `std::env::var(\"HOME\")` in Rust (`PathBuf::from(home).join(\"workspace/...\")`, \
         never a tilde-prefixed literal — Rust does not expand `~`) and `$HOME` in \
         shell. Offenders:\n{}",
        hits.len(),
        hits.iter()
            .map(|(file, line, text)| format!("  {file}:{line}: {text}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// A tilde-prefixed path literal, assembled from fragments for the same
/// reason [`FOREIGN_HOME`] is: this file must not trip its own guard.
const TILDE_LITERAL_PREFIX: &str = concat!("\"", "~", "/");

/// The companion trap: `~` is not expanded by Rust, so a tilde-prefixed path
/// literal in Rust source is a directory named `~` that silently does not
/// exist. Shell scripts are exempt — `$HOME` is preferred there, but a
/// leading tilde on an unquoted word does expand in a shell.
#[test]
fn no_rust_source_defaults_a_path_to_an_unexpanded_tilde() {
    let root = repo_root();
    let mut hits = Vec::new();
    for relative in scanned_files() {
        if relative.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let Some(text) = read_text(&root.join(&relative)) else {
            continue;
        };
        for (index, line) in text.lines().enumerate() {
            if line.contains(TILDE_LITERAL_PREFIX) {
                hits.push(format!(
                    "  {}:{}: {}",
                    relative.to_string_lossy(),
                    index + 1,
                    line.trim()
                ));
            }
        }
    }
    assert!(
        hits.is_empty(),
        "{} Rust string literal(s) begin a path with a tilde. Rust does not expand `~`: \
         such a literal is a relative directory named `~` that does not exist, and it \
         fails silently. Resolve `std::env::var(\"HOME\")` instead. Offenders:\n{}",
        hits.len(),
        hits.join("\n")
    );
}

/// Proves the guard can actually see a violation. Without this, a broken
/// walk (wrong root, silent read error, empty file list) would make the
/// guard above pass forever while checking nothing.
#[test]
fn guard_detects_a_literal_that_is_present() {
    // `scripts/verify.sh` is in the scanned tree and unquestionably contains
    // the word "cargo"; if the walk works, the guard finds it.
    let hits = offenders("cargo");
    assert!(
        !hits.is_empty(),
        "guard found no occurrence of a string known to be present in the scanned tree, \
         so the scan is not actually reading files"
    );
}
