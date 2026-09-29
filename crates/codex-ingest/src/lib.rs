//! codex-ingest: the PCGen tool side, walled off from the live character
//! sheet engine (`codex`).
//!
//! Operator ruling D1 (SD-36 Epic A): PCGen is a converter input and a test
//! oracle, kept for Starfinder, but nothing in `codex` -- the code that
//! computes and prints a character sheet -- may read a PCGen token, a PCGen
//! formula string, or a converter type at run time. Splitting the converter
//! (`pcgen_import`) and the oracle harness (`oracle_validation`) into their
//! own crate makes that a build-graph fact instead of a grep result:
//! `codex-desktop` cannot accidentally depend on this crate's non-dev graph,
//! because `cargo tree` would show it. See `scripts/pcgen_residue_gate.py`
//! for the residue count this crate exists to make structurally impossible
//! to violate on the live side, and its `crate-wall` stage in
//! `scripts/verify.sh` for the mechanical proof.
//!
//! Dependency direction is `codex-ingest -> codex`, never the reverse:
//! `codex` must never dev-depend on this crate (a dev-dep cycle would compile
//! `codex` twice, making `crate::rules_core::X` and `codex::rules_core::X`
//! distinct types at the type-check level even though they are the same
//! source -- see the plan's §9 "type identity across the dev-dep cycle").

pub mod bar_check;
pub mod oracle_validation;
pub mod pcgen_import;

/// The repository root, two directories up from this crate's manifest.
///
/// Moved code used to resolve the repo root via
/// `env!("CARGO_MANIFEST_DIR")` when it lived at the workspace root; now that
/// this crate sits at `crates/codex-ingest`, that would resolve to the
/// crate's own directory instead. Every such site was rewritten to call this
/// function.
pub fn repo_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}
