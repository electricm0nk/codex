//! SD-26 Epic 1, Criterion 1.1 — repo-wide identifier-discipline regression
//! guard, extended to `scripts/` and `data/`.
//!
//! `tests/sd24_identifier_discipline_audit.rs` (SD-24 Epic 1) landed the
//! first standing `cargo test` for this audit, but scoped it to
//! `apps/desktop/`, `apps/desktop/src-tauri/`, `src/` only. SD-26's own
//! Criterion 1.1 (`epic-breakdown.md` Epic 1) names a wider path set —
//! `apps/desktop/ apps/desktop/src-tauri/ src/ scripts/ data/` — because
//! SD-26 Epics 3 and 4 are about to populate `data/corpus/` and
//! `data/stubs/` heavily with newly-generated JSON, and `scripts/` with
//! new ingest tooling. Neither `scripts/` nor `data/` had any automated
//! bundle-tag-leak tripwire before this cycle: a leak introduced there by
//! a later SD-26 cycle (or any future bundle) would have gone undetected
//! forever, since `sd24_identifier_discipline_audit.rs`'s `git grep` never
//! looks inside those two directories. This test closes that gap.
//!
//! ## RED confirmation performed this cycle
//!
//! Before adding this file, the literal Criterion 1.1 RED command from
//! `epic-breakdown.md` --
//! `git grep -nE '\b(sd(16|19|22|23|24)_|SD(16|19|22|23|24)_|Sd(16|19|22|23|24)|t_[0-9a-f]{8,})\b' apps/desktop/ apps/desktop/src-tauri/ src/ scripts/ data/`
//! -- was run against `tranche/5-4` HEAD and returned **0 hits** (exit code
//! 1), i.e. the criterion's own RED step did not confirm a failure the way
//! Criterion 1.1's prose describes ("expect >=1 hit"). This is not a false
//! pass: `tests/sd24_identifier_discipline_audit.rs`'s `## DISCOVERED` note
//! already recorded that the canonical pattern's trailing `\b` under-detects
//! (it only matches when the underscore is the last character of the
//! token), and that bundle's cycle already remediated every real leak the
//! *corrected* pattern (no trailing `\b`) found in `apps/desktop/`,
//! `apps/desktop/src-tauri/`, `src/`. Re-running the corrected pattern
//! here, now widened to also cover `scripts/` and `data/`, likewise
//! returned 0 real leaks (all matches are `tests/...` path citations in doc
//! comments/string literals, the documented exemption class -- see
//! `docs/doctrine-external/identifier-discipline.md`). So there was no
//! rename to perform this cycle; the actual RED this cycle closes is
//! "`scripts/` and `data/` have zero automated leak protection," proven by
//! temporarily injecting a synthetic bundle-tag identifier into a scratch
//! file under `data/` and confirming this test's assertion fails against
//! it, then removing the scratch file and confirming the test passes
//! clean against the real tree (see cycle receipt for the transcript).
//!
//! `tests/` itself is intentionally out of the audited path set, matching
//! the exemption `sd24_identifier_discipline_audit.rs` established:
//! test-fixture files are allowed to carry bundle-tag prefixes by design
//! (`tests/sd13_*.rs`, `tests/sd19_*.rs`, `tests/sd24_*.rs`, this file's own
//! `sd26_` prefix), and production source citing a real `tests/...` file by
//! name in a doc comment/string literal is not a leak either.

use std::process::Command;

/// Same bundle-tag alternation as `epic-breakdown.md` Criterion 1.1's own
/// command, with the trailing `\b` dropped (see module doc comment) so the
/// pattern actually matches identifiers like `sd19_class_catalog` rather
/// than only a bare `sd24_` with nothing after it.
const AUDIT_PATTERN: &str =
    r"\b(sd(16|19|22|23|24)_|SD(16|19|22|23|24)_|Sd(16|19|22|23|24)|t_[0-9a-f]{8,})";

#[test]
fn no_bundle_tag_identifier_leaks_in_scripts_and_data() {
    let repo_root = env!("CARGO_MANIFEST_DIR");

    let output = Command::new("git")
        .args([
            "grep",
            "-nE",
            AUDIT_PATTERN,
            "--",
            "apps/desktop/",
            "apps/desktop/src-tauri/",
            "src/",
            "scripts/",
            "data/",
        ])
        .current_dir(repo_root)
        .output()
        .expect("git grep must be runnable inside a git checkout");

    // `git grep` exits 1 when it finds zero matching lines -- that is the
    // *passing* state for this audit, not a broken invocation. Any other
    // non-zero exit code means the command itself failed (bad pattern, not
    // a git repo, etc.) and must not be silently treated as "clean".
    let exit_code = output.status.code();
    assert!(
        exit_code == Some(0) || exit_code == Some(1),
        "git grep exited unexpectedly (status={exit_code:?}); stderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let hits = String::from_utf8_lossy(&output.stdout);
    let real_leaks: Vec<&str> = hits
        .lines()
        .filter(|line| !line.contains("tests/"))
        .collect();

    assert!(
        real_leaks.is_empty(),
        "identifier-discipline audit found bundle-tag leaks in shipping \
         source or data (apps/desktop/, apps/desktop/src-tauri/, src/, \
         scripts/, data/) -- rename these identifiers per \
         docs/doctrine-external/identifier-discipline.md (skill: \
         `identifier-discipline`) before Criterion 1.1 can pass:\n{}",
        real_leaks.join("\n")
    );
}
