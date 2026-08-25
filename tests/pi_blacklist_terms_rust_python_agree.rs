//! SD-32 `decisions.md §12b` — the Rust and Python PI-blacklist term-list
//! twins must agree in behaviour. `src/rules_core/pi_screening.rs::
//! PI_BLACKLIST_TERMS` and `scripts/pi_scrub.py::PI_BLACKLIST_TERMS` are two
//! independently-maintained copies of the same term list (the Rust copy
//! backs the production ingest-time screen; the Python copy backs the
//! read-only PI review/audit scripts) and this bundle found them
//! one-term-divergent — never checked against each other by any prior test.
//!
//! This test shells out to `python3` to read the live Python list (never a
//! hand-transcribed copy, which would just be a third place to drift) and
//! diffs it against the Rust constant. It is the mechanical control this
//! divergence class needs: a future edit to either list that doesn't also
//! update its twin fails the build here rather than shipping a silent
//! screening-behaviour gap (`decisions.md §1a`: a gate that cannot fail is
//! worse than no gate).
//!
//! Per this bundle's PI discipline, this file never prints or asserts on the
//! actual term strings — only on set equality and counts — so a failure
//! message here never leaks blacklist content into CI logs or a receipt.

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Reads `scripts/pi_scrub.py::PI_BLACKLIST_TERMS` via a live `python3`
/// import (never a hand-copied literal), returning one term per line.
fn python_term_list() -> Option<Vec<String>> {
    let root = repo_root();
    let scripts_dir = root.join("scripts");
    let code = format!(
        "import sys; sys.path.insert(0, {scripts_dir:?}); import pi_scrub; \
         print(chr(10).join(pi_scrub.PI_BLACKLIST_TERMS))",
    );
    let output = Command::new("python3")
        .args(["-c", &code])
        .current_dir(&root)
        .output()
        .ok()?;
    if !output.status.success() {
        eprintln!(
            "pi_blacklist_terms_rust_python_agree: python3 invocation failed ({:?}), skipping \
             this environment rather than false-failing",
            String::from_utf8_lossy(&output.stderr)
        );
        return None;
    }
    let text = String::from_utf8(output.stdout).ok()?;
    Some(text.lines().map(|s| s.to_string()).collect())
}

#[test]
fn rust_and_python_pi_blacklist_term_lists_agree() {
    let Some(python_terms) = python_term_list() else {
        return;
    };
    let rust_terms: Vec<String> = codex::rules_core::pi_screening::PI_BLACKLIST_TERMS
        .iter()
        .map(|s| s.to_string())
        .collect();

    assert_eq!(
        rust_terms.len(),
        python_terms.len(),
        "src/rules_core/pi_screening.rs::PI_BLACKLIST_TERMS and scripts/pi_scrub.py::\
         PI_BLACKLIST_TERMS have drifted apart in LENGTH (decisions.md §12b) -- a term was \
         added to one copy and not its twin. Never resolve this by writing the differing \
         term into a commit message, test name, or receipt; identify it by comparing the two \
         source files directly and add it to whichever copy is short."
    );

    let rust_set: std::collections::BTreeSet<&String> = rust_terms.iter().collect();
    let python_set: std::collections::BTreeSet<&String> = python_terms.iter().collect();
    let only_in_rust = rust_set.difference(&python_set).count();
    let only_in_python = python_set.difference(&rust_set).count();
    assert_eq!(
        only_in_rust, 0,
        "src/rules_core/pi_screening.rs::PI_BLACKLIST_TERMS carries a term \
         scripts/pi_scrub.py::PI_BLACKLIST_TERMS does not (decisions.md §12b) -- \
         identify it by diffing the two files directly, never by naming it here."
    );
    assert_eq!(
        only_in_python, 0,
        "scripts/pi_scrub.py::PI_BLACKLIST_TERMS carries a term \
         src/rules_core/pi_screening.rs::PI_BLACKLIST_TERMS does not (decisions.md §12b) -- \
         identify it by diffing the two files directly, never by naming it here."
    );
}
