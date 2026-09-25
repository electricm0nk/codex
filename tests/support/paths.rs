//! Shared filesystem-path helpers for integration tests.
//!
//! SD-36 Epic C2 (`docs/release/SD-36-consolidation/epic-breakdown.md` C2.3,
//! `technical-design.md` §4): before this module existed, `repo_root()` was
//! defined byte-identically in 17 `tests/*.rs` files, `pcgen_data_root()` in
//! 6 more, `corpus_root()` (or its `CORPUS_ROOT`-env-gated `Option` variant)
//! in 5, and `fixture_root()` in 4 (with two incompatible signatures) — 32
//! local copies across 26 files. This mirrors `src/support/paths.rs` (the
//! Epic C1 canonical set for `src/`), adapted for `tests/`: integration
//! tests compile as their own crate and cannot reach `codex`'s
//! `pub(crate)` items, so this is a parallel definition, not a re-export,
//! brought into each file with `#[path = "support/paths.rs"] mod paths;`.
//!
//! This is a pure move: no logic changed from any of the copies it
//! replaces. Every caller now reads from here instead of its own local
//! copy.
//!
//! `#![allow(dead_code)]`, mirroring `tests/common/mod.rs`: this file is
//! included via `#[path]` separately into each consuming test binary, and
//! no single binary calls all five functions, so each binary's own copy of
//! this module would otherwise warn (and, under the `-D warnings` clippy
//! lock, fail) on whichever functions it does not happen to use.
#![allow(dead_code)]

use std::path::PathBuf;

/// The repo root, from Cargo's own build-time env var. Every corpus/data/
/// fixture path a test builds is joined onto this.
pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// `data/corpus`, joined onto [`repo_root`] — the corpus this build ships,
/// unconditionally. Replaces four identical `fn corpus_root() -> PathBuf`
/// copies (`sd27_license_stripping_shape_v1.rs`, `sd29_...`, `sd30_...`,
/// `sd31_lst_provenance_repair_is_durable.rs`).
pub fn corpus_root() -> PathBuf {
    repo_root().join("data/corpus")
}

/// `$CORPUS_ROOT`, if set AND a real directory — `None` otherwise, never
/// falling back to a computed default the way [`corpus_root`] does.
/// Replaces `sd19_feat_catalog.rs`'s identical `fn corpus_root() ->
/// Option<PathBuf>` (imported there under that original name via `use
/// paths::corpus_root_if_set as corpus_root;` so its call sites are
/// unchanged).
pub fn corpus_root_if_set() -> Option<PathBuf> {
    let path = PathBuf::from(std::env::var("CORPUS_ROOT").ok()?);
    path.is_dir().then_some(path)
}

/// The pinned PCGen oracle checkout's data directory: `$PCGEN_CORPUS_ROOT`,
/// or `$HOME/workspace/repos/pcgen/data` if unset. Replaces six identical
/// copies across the `derived_evaluator_fixture_check_*` guarantee files.
pub fn pcgen_data_root() -> Option<PathBuf> {
    if let Ok(root) = std::env::var("PCGEN_CORPUS_ROOT") {
        return Some(PathBuf::from(root));
    }
    let home = std::env::var("HOME").ok()?;
    Some(PathBuf::from(home).join("workspace").join("repos").join("pcgen").join("data"))
}

/// `tests/fixtures/<relative>`, joined onto [`repo_root`]. The four
/// pre-existing `fixture_root()` copies each hard-coded their own file's
/// fixed subpath (two zero-argument, two one-argument with a different
/// format-string shape); this is that pattern generalized to take the
/// varying suffix as a parameter.
pub fn fixture_root(relative: &str) -> PathBuf {
    repo_root().join("tests/fixtures").join(relative)
}
