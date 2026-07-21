//! SD-24 Epic 1, Criterion 1.1/1.2 — repo-wide identifier-discipline
//! regression guard.
//!
//! Prior bundles (SD-16 through SD-23) each ran their own one-shot
//! `identifier-discipline` grep by hand during a closure cycle (see e.g.
//! `docs/release/SD-22/artifacts/epic_1_2/prelaunch_and_identifier_audit_cycle_receipt.md`)
//! but never landed the check as a standing `cargo test` — so a leak
//! introduced *between* bundles had no automated tripwire. This test closes
//! that gap by running a corrected identifier-discipline audit against the
//! three shipping source trees (`apps/desktop/`, `apps/desktop/src-tauri/`,
//! `src/`) on every `cargo test`.
//!
//! ## DISCOVERED: the canonical audit pattern under-detects
//!
//! `epic-breakdown.md` Criterion 1.1 and `loop-instruction.md §2.3` step 4
//! both name this exact command as the audit:
//!
//! ```text
//! grep -nE '\b(sd(16|19|22|23|24)_|SD(16|19|22|23|24)_|Sd(16|19|22|23|24)|t_[0-9a-f]{8,})\b'
//! ```
//!
//! The trailing `\b` is a bug: `\b` only matches between a `\w` and a
//! non-`\w` character, and `_` *is* a `\w` character. So `sd(16|19|...)_`
//! only matches when the underscore is the literal last character of the
//! line/token (e.g. a bare `sd24_`) — it never matches a real identifier
//! like `sd19_class_catalog`, because the `_` is immediately followed by
//! another word character. Live proof against this repo's pre-remediation
//! state (SD-24 Epic 1 cycle 1, 2026-07-21): the literal criterion command
//! returned **0 hits** against `apps/desktop/`, `apps/desktop/src-tauri/`,
//! `src/`, while the repo in fact shipped six production Rust modules named
//! `sd16_browser_handoff`, `sd19_class_catalog`, `sd19_corpus`,
//! `sd19_equipment_catalog`, `sd19_race_catalog`, `sd19_spell_catalog`, a
//! `resources/sd19_corpus_fixtures/` bundled Tauri resource directory, and
//! thirteen `SD19_*_TEST` constants in `support_state_matrix.rs` — all
//! genuine bundle-tag identifier leaks the criterion exists to catch. This
//! cycle remediated all of them (see the cycle receipt) and this test uses
//! the *corrected* pattern (same alternation, trailing `\b` dropped) so a
//! future regression is actually caught. The canonical pattern in
//! `epic-breakdown.md`/`loop-instruction.md` themselves is left alone here
//! (governance-doc authorship is out of this cycle's granted scope) but the
//! flaw is recorded in `progress.md`'s `## DISCOVERED` block for governance
//! to harden.
//!
//! `tests/` itself (this file included) is intentionally out of the
//! audited path set — the repo's own test-fixture naming convention
//! (`tests/sd13_*.rs`, `tests/sd19_*.rs`, `tests/ge06_*.rs`, this file's own
//! `sd24_` prefix) legitimately carries bundle-tag prefixes by design.
//! Production source is additionally allowed to *cite* a real `tests/...`
//! fixture or spec path by name in a doc comment/string (e.g. `"grounding_ref:
//! tests/sd19_school_abjuration.rs"`) without that citation counting as a
//! leak — matching the exemption SD-22's own identifier-audit receipt
//! established for "rustdoc-comment citations of test files using the test
//! file's own name". Any hit whose matched line contains the substring
//! `tests/` is treated as such a citation and excluded from the failure set;
//! every other hit is a real identifier that must be renamed.

use std::process::Command;

/// Corrected identifier-discipline audit pattern: same bundle-tag
/// alternation as `epic-breakdown.md` Criterion 1.1's own command, but
/// without the trailing `\b` that silently defeats it (see module doc
/// comment `## DISCOVERED` note above).
const AUDIT_PATTERN: &str =
    r"\b(sd(16|19|22|23|24)_|SD(16|19|22|23|24)_|Sd(16|19|22|23|24)|t_[0-9a-f]{8,})";

#[test]
fn no_bundle_tag_identifier_leaks_in_shipping_source() {
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
         source (apps/desktop/, apps/desktop/src-tauri/, src/) -- rename \
         these identifiers per governance/identifier-discipline.md (skill:\n\
         `identifier-discipline`) before Criterion 1.1 can pass:\n{}",
        real_leaks.join("\n")
    );
}
