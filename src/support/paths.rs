//! Shared filesystem-path helpers.
//!
//! SD-36 Epic C1 (`docs/release/SD-36-consolidation/technical-design.md` §3,
//! `epic-breakdown.md` C1.3): before this module existed, `repo_root`
//! was defined byte-identically in six places under `src/rules_core/`
//! (`class_feature_pool_catalog.rs`, `pilot_compute/class_chassis_sheet_rules.rs`,
//! `pilot_compute/class_feature_grant_consumer.rs`, `record_vars.rs`,
//! `rules_tables/simple_kind_tables.rs`, `skinwalker_change_shape.rs`), and
//! `find_json_files` (as a function) was defined identically in three more
//! (`corpus_loader.rs`, `race_resolver.rs`, `trait_pool.rs`). This is a pure
//! move: no logic changed, every call site now reads from here instead of
//! its own local copy.
//!
//! `corpus_root`, `corpus_root_if_set` and `pcgen_corpus_root` round out
//! `technical-design.md` §4's named set of five canonical path fns.
//! `corpus_root` has real in-scope callers (its own doc comment). The other
//! two do not -- their only duplicates (`CORPUS_ROOT`-gated test bodies, the
//! PCGen oracle reader) live in `crates/codex-ingest/`, outside C1's write
//! scope -- so, like `corpus_subdir` below, they are `#[cfg(test)]`: an
//! unused `pub(crate)` fn is dead code the `-D warnings` clippy gate (§5)
//! would fail on, worse than not adding the fn at all.

use std::fs;
use std::path::{Path, PathBuf};

/// The repo root, from Cargo's own build-time env var. Every corpus/data
/// path a live consumer or a test builds is joined onto this.
pub(crate) fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// `data/corpus`, joined onto [`repo_root`] -- the corpus this build ships,
/// unconditionally (no env-var indirection; see [`corpus_root_if_set`] for
/// that). Replaces three sites that used to spell out
/// `repo_root().join("data/corpus")` themselves (`class_feature_pool_catalog.rs`,
/// twice in `pilot_compute/class_feature_grant_consumer.rs`).
pub(crate) fn corpus_root() -> PathBuf {
    repo_root().join("data/corpus")
}

/// `$CORPUS_ROOT`, if set AND a real directory -- `None` otherwise, never
/// falling back to a computed default the way [`corpus_root`] does. The
/// escape hatch a corpus-gated test uses to point at an alternate checkout
/// (the pattern `crates/codex-ingest`'s `CORPUS_ROOT`-gated tests already use).
#[cfg(test)]
pub(crate) fn corpus_root_if_set() -> Option<PathBuf> {
    let path = PathBuf::from(std::env::var("CORPUS_ROOT").ok()?);
    path.is_dir().then_some(path)
}

/// The pinned PCGen oracle checkout: `$PCGEN_CORPUS_ROOT`, or
/// `$HOME/workspace/repos/pcgen/data` if unset. The RAW `.lst` oracle
/// `crates/codex-ingest` diffs its converted output against -- never this
/// crate's own `data/corpus/` ([`corpus_root`]). Mirrors the `corpus_root`
/// function `codex-ingest/src/pcgen_import/sheet_rule/closure.rs` defines.
#[cfg(test)]
pub(crate) fn pcgen_corpus_root() -> PathBuf {
    match std::env::var("PCGEN_CORPUS_ROOT") {
        Ok(configured) => PathBuf::from(configured),
        Err(_) => {
            let home = std::env::var("HOME").expect("HOME must be set to locate the pinned corpus checkout");
            PathBuf::from(home).join("workspace/repos/pcgen/data")
        }
    }
}

/// `data/corpus/<relative>`, joined onto [`repo_root`]. Test modules for
/// individual ingested rule tables (the Unchained Barbarian and Unchained
/// Monk feature sets, for example) each used to spell out
/// `PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/corpus/...")` with
/// only the trailing path differing; this is that pattern with the one
/// varying piece taken as a parameter. `#[cfg(test)]`-only because every
/// current caller is a test module (`decisions.md` §18/B15: a `#[cfg(test)]`
/// region is not live code); a live caller would just as well use it.
#[cfg(test)]
pub(crate) fn corpus_subdir(relative: &str) -> PathBuf {
    repo_root().join("data/corpus").join(relative)
}

/// Recursively collect every `*.json` file under `dir`, skipping `_parity/`
/// directories and `LICENSE.json` files.
///
/// Sorted, because the ORDER records are pushed into a `SourcePackageContent`
/// is significant -- a resolver reading that package decides a key collision
/// by position. Unsorted, that order is `read_dir` order, which is the
/// filesystem's, which is stable for one directory on one machine and NOT
/// stable across two checkouts of the same corpus. That non-determinism is
/// the reason this walk sorts its output rather than returning it in
/// traversal order.
pub(crate) fn find_json_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        let Ok(entries) = fs::read_dir(&current) else { continue };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corpus_subdir_joins_onto_data_corpus() {
        let expected = repo_root().join("data/corpus").join("pathfinder_unchained");
        assert_eq!(corpus_subdir("pathfinder_unchained"), expected);
    }

    #[test]
    fn root_helper_points_at_a_real_checkout() {
        let root = repo_root();
        assert!(root.join("Cargo.toml").is_file(), "{root:?} must be the repo root");
        assert!(root.join("data/corpus").is_dir(), "{root:?} must have data/corpus");
    }

    #[test]
    fn corpus_root_is_repo_root_join_data_corpus() {
        let root = corpus_root();
        assert_eq!(root, repo_root().join("data/corpus"));
        assert!(root.is_dir(), "{root:?} must be a real, shipped directory");
    }

    /// Both branches in one test, mutating the process-global `CORPUS_ROOT`:
    /// this crate's binary has no other test touching it (`codex-ingest`'s
    /// `CORPUS_ROOT`-gated tests are a separate binary), so nothing races.
    #[test]
    fn corpus_root_if_set_reads_corpus_root_env_var() {
        let original = std::env::var("CORPUS_ROOT").ok();
        // SAFETY: single-threaded with respect to this var -- see doc comment above.
        unsafe { std::env::remove_var("CORPUS_ROOT") };
        assert_eq!(corpus_root_if_set(), None, "unset must resolve to None");
        unsafe { std::env::set_var("CORPUS_ROOT", "/definitely/not/a/real/directory/for/this/test") };
        assert_eq!(corpus_root_if_set(), None, "a non-directory path must resolve to None");
        let real_dir = repo_root();
        unsafe { std::env::set_var("CORPUS_ROOT", &real_dir) };
        assert_eq!(corpus_root_if_set(), Some(real_dir), "a real directory must resolve to Some");
        match original {
            Some(value) => unsafe { std::env::set_var("CORPUS_ROOT", value) },
            None => unsafe { std::env::remove_var("CORPUS_ROOT") },
        }
    }

    /// Same one-test-both-branches shape as the sibling test above, and for
    /// the same reason: `PCGEN_CORPUS_ROOT` is process-global.
    #[test]
    fn pcgen_corpus_root_reads_env_override_else_defaults_under_home() {
        let original = std::env::var("PCGEN_CORPUS_ROOT").ok();
        let home = std::env::var("HOME").expect("HOME must be set for this test to be meaningful");
        // SAFETY: single-threaded with respect to this var -- see the sibling test above.
        unsafe { std::env::remove_var("PCGEN_CORPUS_ROOT") };
        assert_eq!(pcgen_corpus_root(), PathBuf::from(&home).join("workspace/repos/pcgen/data"));
        unsafe { std::env::set_var("PCGEN_CORPUS_ROOT", "/tmp/a-pinned-pcgen-checkout") };
        assert_eq!(pcgen_corpus_root(), PathBuf::from("/tmp/a-pinned-pcgen-checkout"));
        match original {
            Some(value) => unsafe { std::env::set_var("PCGEN_CORPUS_ROOT", value) },
            None => unsafe { std::env::remove_var("PCGEN_CORPUS_ROOT") },
        }
    }

    #[test]
    fn find_json_files_skips_parity_and_license_and_sorts() {
        let dir = repo_root().join("data/corpus");
        let found = find_json_files(&dir);
        assert!(!found.is_empty(), "expected real corpus JSON under {dir:?}");
        assert!(
            found.iter().all(|p| p.extension().and_then(|e| e.to_str()) == Some("json")),
            "every entry must be a .json file"
        );
        assert!(
            found.iter().all(|p| p.file_name().and_then(|n| n.to_str()) != Some("LICENSE.json")),
            "LICENSE.json must never appear"
        );
        assert!(
            found.iter().all(|p| !p.components().any(|c| c.as_os_str() == "_parity")),
            "no path may pass through a _parity directory"
        );
        let mut sorted = found.clone();
        sorted.sort();
        assert_eq!(found, sorted, "result must already be sorted");
    }
}
