//! Production code must not read `env!("CARGO_MANIFEST_DIR")` directly.
//!
//! That value is the build machine's checkout path, baked in at compile time. A packaged app does
//! not have it, so any production read joined onto it fails on a user's machine while passing in
//! every test (tests run from the checkout). Reads go through `support::paths::repo_root()`, which
//! a packaged app redirects with `set_data_root`; `tests/`-side proof that the redirect is
//! sufficient lives in `apps/desktop/src-tauri/tests/packaged_resources.rs`.
//!
//! Scope of this check: it reads `src/` text and flags a `CARGO_MANIFEST_DIR` use that appears
//! BEFORE the file's first `#[cfg(test)]`. It does not see a baked path built another way
//! (`option_env!`, a literal absolute path), and a production read placed after a file's first
//! `#[cfg(test)]` module is not scanned. `src/bin/*` are developer tools run from a checkout and
//! are excluded.

use std::fs;
use std::path::{Path, PathBuf};

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|n| n == "bin") {
                continue;
            }
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

#[test]
fn production_code_reads_data_through_repo_root_not_the_baked_manifest_dir() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let paths_rs = src.join("support").join("paths.rs");
    let mut files = Vec::new();
    rust_files(&src, &mut files);

    let mut offenders = Vec::new();
    for file in files.into_iter().filter(|f| *f != paths_rs) {
        let text = fs::read_to_string(&file).unwrap();
        for (index, line) in text.lines().enumerate() {
            if line.contains("#[cfg(test)]") {
                break;
            }
            let code = line.trim_start();
            if code.starts_with("//") {
                continue;
            }
            if code.contains("CARGO_MANIFEST_DIR") {
                offenders.push(format!("{}:{}: {}", file.strip_prefix(&src).unwrap().display(), index + 1, code));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "production code reads the baked CARGO_MANIFEST_DIR; use support::paths::repo_root():\n  {}",
        offenders.join("\n  ")
    );
}
