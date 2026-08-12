//! SD-24 Epic 5, Criterion 5.4 — "Multiclass dispatch passes the four-check
//! audit" as a standing regression guard, mirroring
//! `tests/sd24_wired_integration_audit.rs`'s repo-wide shape but narrowed to
//! the multiclass-dispatch production surface itself: the files that
//! Criteria 5.1/5.3 actually touched to grant a Fighter+Wizard multiclass
//! mix real chassis (BAB/save) stacking:
//!
//! - `src/rules_core/pilot_compute.rs`
//! - `src/rules_core/level_up/fighter.rs`
//! - `src/rules_core/level_up/wizard.rs`
//! - `src/rules_core/rules_tables/crb/class_tables.rs`
//!
//! `epic-breakdown.md`'s criterion 5.4 row reads: "Multiclass dispatch passes
//! the four-check audit" with cycle artifact "dual-audit gate output
//! captured." The per-cycle diff-scoped grep in `loop-instruction.md §2.3`
//! step 4 (against `${BASE_BRANCH}...HEAD`, the whole bundle's diff) is
//! already run and recorded clean by this cycle's own artifact receipt; this
//! test is the durable, `cargo test`-standing counterpart scoped specifically
//! to the multiclass dispatch files, so that any future cycle that touches
//! these four files (Epic 5 follow-on, or an unrelated cycle brushing past
//! them) re-proves the four-check audit automatically rather than relying on
//! a human re-running the loop-instruction grep by hand.
//!
//! Checks 2 (`onClick` no-op handlers) has no applicable surface here — the
//! multiclass dispatch files are pure Rust (`.rs`) with no `.tsx`/`.jsx`
//! affordances — so it is asserted as vacuously satisfied rather than
//! omitted, keeping the "four-check" framing explicit and honest about why
//! that check does not fire.
//!
//! ## RED evidence (this cycle, genuine, not injected)
//!
//! This test's `placeholder` check was written first with no exclusion
//! filter at all and run against the real files: it failed immediately,
//! reporting 19 real hits in `src/rules_core/pilot_compute.rs` (see
//! the cycle receipt for the full `cargo test` failure output). Manual
//! review classified all 19 as benign — 18 are the same "undocumented
//! packet placeholder" anti-fabrication idiom already reviewed and bucketed
//! in `tests/sd24_wired_integration_audit.rs` (criterion 3.1's repo-wide
//! audit), and 1 is ordinary `//`-comment prose. The exclusion buckets below
//! encode that review; removing them reproduces the RED failure.

use std::process::Command;

/// The exact multiclass-dispatch production file set (5.1/5.3's own touched
/// files, confirmed via `git log --name-only 0068818^..HEAD -- src/`).
const MULTICLASS_DISPATCH_FILES: [&str; 4] = [
    "src/rules_core/pilot_compute.rs",
    "src/rules_core/level_up/fighter.rs",
    "src/rules_core/level_up/wizard.rs",
    "src/rules_core/rules_tables/crb/class_tables.rs",
];

/// Run `git grep -nE <pattern>` over exactly the multiclass dispatch file
/// set, returning matched lines (or empty if the pattern found nothing —
/// `git grep` exits 1 in that case, which is the passing state).
fn git_grep_multiclass_dispatch_files(pattern: &str) -> Vec<String> {
    let repo_root = env!("CARGO_MANIFEST_DIR");

    let mut args = vec!["grep".to_string(), "-nE".to_string(), pattern.to_string(), "--".to_string()];
    args.extend(MULTICLASS_DISPATCH_FILES.iter().map(|s| s.to_string()));

    let output = Command::new("git")
        .args(&args)
        .current_dir(repo_root)
        .output()
        .expect("git grep must be runnable inside a git checkout");

    let exit_code = output.status.code();
    assert!(
        exit_code == Some(0) || exit_code == Some(1),
        "git grep exited unexpectedly (status={exit_code:?}); stderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::to_owned)
        .collect()
}

/// Check 1 (zero-tolerance half): `STUB`, `MOCK`, `not yet implemented`,
/// `todo`, `fixme`, `hack` — none of these carry a legitimate non-stub
/// meaning in shipping source, so no exclusion filter is warranted.
#[test]
fn multiclass_dispatch_files_carry_no_zero_tolerance_forbidden_tokens() {
    let hits = git_grep_multiclass_dispatch_files(
        r"\b(STUB|MOCK|not yet implemented|todo|fixme|hack)\b",
    );
    assert!(
        hits.is_empty(),
        "wired-integration four-check audit (check 1, zero-tolerance) found \
         forbidden tokens in the multiclass dispatch files ({:?}):\n{}",
        MULTICLASS_DISPATCH_FILES,
        hits.join("\n")
    );
}

/// Check 1 (`placeholder` half): noisy term. RED evidence for this cycle —
/// this test's first run (before any exclusion filter existed) genuinely
/// failed, surfacing 19 real `placeholder` hits in
/// `src/rules_core/pilot_compute.rs`. Manual review of every hit found:
///
/// - 18 hits are `ComputationExplanation.detail` / doc-comment strings using
///   the phrase "undocumented packet placeholder" (or "packet placeholder")
///   as an anti-fabrication assurance — i.e. asserting the *opposite* of the
///   forbidden pattern: each says explicitly that the record "grounds no ...
///   math" and "carries no fabricated mechanical value" rather than
///   inventing one. This is the same idiom already reviewed and bucketed as
///   "Bucket D" in `tests/sd24_wired_integration_audit.rs` (criterion 3.1's
///   repo-wide audit) — not a new finding, the same pre-existing pattern
///   re-surfacing under this narrower file-scoped grep.
/// - 1 hit is ordinary engineering prose in a `//` comment ("a 'correctly
///   absent' placeholder is unnecessary busywork...") describing a design
///   choice, not an unfinished-work marker.
///
/// Zero hits are unexplained. Both exclusion buckets are matched by
/// distinctive literal substrings so a genuinely new stub-shaped
/// `placeholder` hit (one that does NOT contain "packet placeholder" and is
/// NOT a `//`/`///`/`*` comment line) still fails this test.
#[test]
fn multiclass_dispatch_files_placeholder_hits_are_anti_fabrication_or_reviewed_prose() {
    let hits = git_grep_multiclass_dispatch_files(r"\bplaceholder\b");

    // Bucket D (matches `sd24_wired_integration_audit.rs`'s naming): runtime-
    // visible anti-fabrication assurance text using "packet placeholder" to
    // mean "we did NOT fabricate a value here," the inverse of a stub.
    let is_anti_fabrication_explanation_text = |line: &str| line.contains("packet placeholder");

    // Bucket C: ordinary engineering prose in a comment, not an unfinished-
    // work marker. The zero-tolerance test above already independently
    // catches literal STUB/MOCK/not yet implemented/todo/fixme/hack with no
    // such filter, so gating this bucket on "is a comment line" is safe.
    let is_reviewed_comment_prose = |line: &str| {
        let content = line.splitn(3, ':').nth(2).unwrap_or(line);
        let trimmed = content.trim_start();
        trimmed.starts_with("//") || trimmed.starts_with("*") || trimmed.starts_with("/**")
    };

    let unexplained: Vec<&String> = hits
        .iter()
        .filter(|line| {
            !is_anti_fabrication_explanation_text(line) && !is_reviewed_comment_prose(line)
        })
        .collect();

    assert!(
        unexplained.is_empty(),
        "wired-integration four-check audit (check 1, placeholder) found \
         `placeholder` hits in the multiclass dispatch files ({:?}) that are \
         neither the reviewed anti-fabrication idiom nor reviewed comment \
         prose -- these look like new, unreviewed stub markers and must be \
         triaged before this test can pass:\n{}",
        MULTICLASS_DISPATCH_FILES,
        unexplained
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// Check 2: no-op `onClick` handlers. Vacuously satisfied — the multiclass
/// dispatch file set contains no `.tsx`/`.jsx` files, so there is no
/// user-facing affordance surface for this check to examine. Asserted
/// explicitly (rather than omitted) so the "four-check audit" framing stays
/// honest about why this check always passes here.
#[test]
fn multiclass_dispatch_files_have_no_onclick_surface() {
    let has_frontend_file = MULTICLASS_DISPATCH_FILES
        .iter()
        .any(|f| f.ends_with(".tsx") || f.ends_with(".jsx"));
    assert!(
        !has_frontend_file,
        "multiclass dispatch file set unexpectedly includes a frontend file; \
         check 2 (no-op onClick handlers) must actually run against it, not \
         be treated as vacuous"
    );
}

/// Check 3: no mock-library leaks (`mockResolvedValue`, `mockReturnValue(`,
/// `vi.mock(`, `__mocks__`) in the multiclass dispatch surface.
#[test]
fn multiclass_dispatch_files_carry_no_mock_leaks() {
    let hits = git_grep_multiclass_dispatch_files(
        r"mockResolvedValue|mockReturnValue\(|vi\.mock\(|__mocks__",
    );
    assert!(
        hits.is_empty(),
        "wired-integration four-check audit (check 3) found mock-library \
         leaks in the multiclass dispatch files ({:?}):\n{}",
        MULTICLASS_DISPATCH_FILES,
        hits.join("\n")
    );
}

/// Check 4: no `"Would ..."` stub-return strings in the multiclass dispatch
/// surface.
#[test]
fn multiclass_dispatch_files_carry_no_would_strings() {
    let hits = git_grep_multiclass_dispatch_files(r#""Would [^"]*""#);
    assert!(
        hits.is_empty(),
        "wired-integration four-check audit (check 4) found \"Would ...\" \
         stub-return strings in the multiclass dispatch files ({:?}):\n{}",
        MULTICLASS_DISPATCH_FILES,
        hits.join("\n")
    );
}
