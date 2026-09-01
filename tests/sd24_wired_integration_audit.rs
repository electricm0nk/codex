//! SD-24 Epic 3, Criteria 3.1-3.4 — repo-wide wired-integration-discipline
//! regression guard (full four-check audit, standing as `cargo test`).
//!
//! `epic-breakdown.md`'s Epic 3 purpose statement and
//! `acceptance-and-verification.md` CG-02 both require the wired-integration
//! audit to cover *the entire codebase* (not just this bundle's diff against
//! `develop` — that narrower diff-scoped grep is `loop-instruction.md §2.3`
//! step 4's per-cycle gate, already run and clean by every prior SD-24
//! cycle). This test is the codebase-wide sweep: it runs all four checks
//! from `docs/governance/no-stub-mvp-doctrine.md` §"Per-cycle audit" /
//! `wired-integration-discipline` against the three shipping source trees
//! (`apps/desktop/`, `apps/desktop/src-tauri/`, `src/`) and fails on any
//! *new* violation, the same durable-tripwire shape as
//! `tests/sd24_identifier_discipline_audit.rs`.
//!
//! ## Audit result (this cycle, 2026-07-21)
//!
//! - Check 2 (no-op `onClick` handlers): 0 hits, repo-wide.
//! - Check 3 (mock leaks: `mockResolvedValue`/`mockReturnValue(`/`vi.mock(`/
//!   `__mocks__` outside test files): 0 hits, repo-wide.
//! - Check 4 (`"Would ..."` return strings): 0 hits, repo-wide.
//! - Check 1 (`STUB`/`MOCK`/`not yet implemented`/`todo`/`fixme`/`hack`,
//!   case-sensitive per the canonical pattern): 0 hits, repo-wide.
//! - Check 1's remaining alternative, `placeholder`, is the one noisy term
//!   in the canonical pattern: it also matches the legitimate HTML/JSX
//!   input-placeholder-text affordance and ordinary engineering prose about
//!   "a placeholder value" (a sentinel/default), neither of which is the
//!   forbidden pattern the doctrine targets (an unfinished-work marker
//!   shipping as if it were real behavior). Manual per-hit review this cycle
//!   (`docs/release/SD-24-beta-readiness-and-multiclass/artifacts/epic_3/wired-integration-audit.md`)
//!   classified every repo-wide `placeholder` hit into exactly three
//!   buckets, encoded below as the test's three exclusion filters.
//!
//! ## DISCOVERED / deferred finding (genuine, not excluded away)
//!
//! One real forbidden-pattern instance survives every filter:
//! `apps/desktop/src-tauri/src/main.rs`'s `load_pilot_shell_snapshot` Tauri
//! command (and its frontend counterpart
//! `apps/desktop/src/boundary/loadPilotShellSnapshot.ts`) unconditionally
//! returns hardcoded fixture data (`case_id: "ge07-e1-scaffold-placeholder"`)
//! regardless of any real character/pilot state — fixture-only data crossing
//! into a production Tauri command path (doctrine forbidden-pattern #5).
//! This is legacy scaffolding (the `ge07` tag predates the `SD-<N>` bundle
//! convention entirely) consumed only by the SD-11 internal tester
//! workbench, which already labels it honestly to the viewer ("Frontend
//! fallback must never masquerade as product truth" — `main.rs`'s own doc
//! comment; `loadSd11TesterWorkbenchSurface.ts` strips the placeholder's
//! marker lines before display rather than presenting them as real
//! diagnostics). Real remediation requires a design decision this cycle's
//! granted scope does not cover (what a "headless-core-backed" pilot shell
//! snapshot even means, with what input contract) — recorded as a Wired
//! Integration Cleanup candidate in
//! `docs/release/SD-24-beta-readiness-and-multiclass/risks-and-open-questions.md`
//! §5 Deferrals rather than silently excluded. The three marker substrings
//! below (`scaffold-placeholder`, `loadPilotShellSnapshot`,
//! `"Future slices should replace this placeholder"`) are the *only*
//! forbidden-pattern hits this test tolerates; anything else must be a new,
//! real issue and fails the build.
//!
//! ## Exclusion added 2026-07-22 (registry entry 0002, SD-25 criteria 3.3/3.4)
//!
//! `docs/governance/wired-integration-stubs-registry.md` entry 0002 is an
//! operator-approved permanent exception for `StubAdapter`'s literal
//! `"Would render for system {system_id}; not yet implemented"` message
//! (SD-25 criterion 3.3), widened by criterion 3.4 to the three Tauri
//! command files that dispatch unknown rule-system ids to it and assert its
//! exact message in their own tests. `no_zero_tolerance_forbidden_tokens_in_shipping_source`
//! and `no_would_strings_in_shipping_source` had not been updated when 3.3/
//! 3.4 landed, so this standing whole-repo tripwire failed on the very hits
//! the per-cycle scoped dual-audit had already accepted. Both tests now
//! carry a named exclusion, `is_registry_0002_stub_adapter_exception`,
//! scoped to exactly the file set registry entry 0002 documents:
//! `apps/desktop/src-tauri/src/stub_adapter.rs` (whole file, matched by
//! path) and the specific doc-comment/test-assertion lines in
//! `apps/desktop/src-tauri/src/characterHub/{appendToCharacter,recomputeCharacter,reSaveCharacter}.rs`
//! (matched by path *and* the literal `"Would render for system ...; not
//! yet implemented"` content, or the doc comment's distinctive phrase) —
//! any other file or any different forbidden-pattern hit still fails.

use std::process::Command;

/// Run `git grep -nE <pattern>` over the three shipping source trees,
/// excluding test files/directories, returning matched lines (or empty if
/// the pattern found nothing — `git grep` exits 1 in that case, which is
/// the passing state, not a broken invocation).
fn git_grep_shipping_trees(pattern: &str) -> Vec<String> {
    let repo_root = env!("CARGO_MANIFEST_DIR");

    let output = Command::new("git")
        .args([
            "grep",
            "-nE",
            pattern,
            "--",
            "apps/desktop/",
            "apps/desktop/src-tauri/",
            "src/",
        ])
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
        .filter(|line| {
            !(line.contains("__tests__/")
                || line.contains(".test.")
                || line.contains("/tests/")
                || line.contains("_test.rs"))
        })
        .map(str::to_owned)
        .collect()
}

/// Check 1 (zero-tolerance half): `STUB`, `MOCK`, `not yet implemented`,
/// `todo`, `fixme`, `hack` — none of these carry a legitimate non-stub
/// meaning in shipping source, so no exclusion filter is warranted --
/// with one narrow, named exception (SD-24 criterion 6.5): the real PF1
/// SRD/PRD full text of the `Plant Growth` spell ("creatures must hack or
/// force a way through") uses "hack" as an ordinary English verb, not a
/// stub marker. Matched by the record's own distinctive corpus phrase so
/// no *different* future stub can silently ride along under the same
/// exclusion -- any other `hack`/`STUB`/`MOCK`/etc. hit still fails.
#[test]
fn no_zero_tolerance_forbidden_tokens_in_shipping_source() {
    let hits = git_grep_shipping_trees(
        r"\b(STUB|MOCK|not yet implemented|todo|fixme|hack)\b",
    );
    let is_plant_growth_full_spell_text =
        |line: &str| line.contains("creatures must hack or force a way through");
    // Second named exception, same shape (real PF1 corpus text using "hack" as an ordinary
    // English verb, not a stub marker): Tophet (Bestiary 3)'s swallow-whole ability text --
    // "a creature can attempt to hack or smash its way out as normal". Matched by the
    // record's own distinctive phrase, same discipline as the Plant Growth exception above.
    let is_tophet_swallow_whole_full_text =
        |line: &str| line.contains("A creature can attempt to hack or smash its way out as normal");
    // Registry entry 0002 (`docs/governance/wired-integration-stubs-registry.md`):
    // `StubAdapter`'s operator-approved "Would render for system ...; not
    // yet implemented" placeholder message, widened (criterion 3.4) to the
    // three Tauri command files that dispatch to it and assert its exact
    // message in their own tests. Scoped to that named file set only, by
    // path *and* by the distinctive matched content -- a `not yet
    // implemented`/`STUB`/`MOCK`/etc. hit anywhere else still fails.
    let is_registry_0002_stub_adapter_exception = |line: &str| {
        line.starts_with("apps/desktop/src-tauri/src/stub_adapter.rs:")
            || (line.starts_with(
                "apps/desktop/src-tauri/src/characterHub/appendToCharacter.rs:",
            ) && line.contains(
                "which honestly reports \"not yet implemented\" rather than the call",
            ))
            || ((line.starts_with(
                "apps/desktop/src-tauri/src/characterHub/appendToCharacter.rs:",
            ) || line.starts_with(
                "apps/desktop/src-tauri/src/characterHub/reSaveCharacter.rs:",
            ) || line.starts_with(
                "apps/desktop/src-tauri/src/characterHub/recomputeCharacter.rs:",
            )) && line.contains("Would render for system")
                && line.contains("not yet implemented"))
    };
    let hits: Vec<String> = hits
        .into_iter()
        .filter(|line| {
            !is_plant_growth_full_spell_text(line)
                && !is_tophet_swallow_whole_full_text(line)
                && !is_registry_0002_stub_adapter_exception(line)
        })
        .collect();
    assert!(
        hits.is_empty(),
        "wired-integration audit found zero-tolerance forbidden tokens in \
         shipping source (apps/desktop/, apps/desktop/src-tauri/, src/):\n{}",
        hits.join("\n")
    );
}

/// Check 1 (`placeholder` half): noisy term, filtered per the three
/// documented buckets above, with the one genuine finding tolerated by name
/// (and only by name — anything else fails).
#[test]
fn placeholder_findings_are_ui_text_prose_or_the_one_documented_deferral() {
    let hits = git_grep_shipping_trees(r"\bplaceholder\b");

    // Bucket A: real UI-affordance text (HTML/JSX `placeholder="…"` /
    // `placeholder={…}` prop, its TS type declaration, or the CSS
    // `::placeholder` pseudo-class) — a legitimate product feature, not a
    // stub marker.
    let is_ui_placeholder_text =
        |line: &str| line.contains("placeholder=") || line.contains("placeholder:") || line.contains("::placeholder");

    // Bucket B: the one genuine, documented, deferred finding (see module
    // doc comment). Matched by its distinctive literal markers so any
    // *different* future stub cannot silently ride along under the same
    // exclusion.
    let is_documented_deferred_finding = |line: &str| {
        line.contains("scaffold-placeholder")
            || line.contains("loadPilotShellSnapshot")
            || line.contains("Future slices should replace this placeholder")
            || line.contains("placeholder/fallback")
    };

    // Bucket D: `pilot_compute.rs`'s `ComputationExplanation.detail` strings
    // (runtime-visible rules-engine audit-trail text, not doc comments) that
    // use "packet placeholder" as an anti-fabrication assurance -- the
    // opposite of the forbidden pattern: each says explicitly that the
    // computation "grounds no ... math" and "carries no fabricated
    // mechanical value" rather than inventing one. Reviewed this cycle;
    // every repo-wide hit of this phrase was manually read in context (see
    // the artifact cited in the module doc comment).
    let is_anti_fabrication_explanation_text = |line: &str| line.contains("packet placeholder");

    // Bucket C: ordinary engineering prose (doc comments / line comments)
    // using "placeholder" to mean a sentinel/default *value* in a data
    // model, not an unfinished-work marker. Every hit in this bucket was
    // manually reviewed this cycle (see the artifact cited in the module
    // doc comment) and confirmed non-stub. Comment-only lines are safe to
    // exempt on the keyword alone because the zero-tolerance test above
    // already catches literal `STUB`/`MOCK`/`not yet implemented`/`todo`/
    // `fixme`/`hack` with no such filter.
    let is_reviewed_comment_prose = |line: &str| {
        // `git grep -n` output is `path:lineno:content`; strip the first two
        // fields to inspect just the source line's own text.
        let content = line.splitn(3, ':').nth(2).unwrap_or(line);
        let trimmed = content.trim_start();
        trimmed.starts_with("//") || trimmed.starts_with("*") || trimmed.starts_with("/**")
    };

    // Bucket E (SD-27 race ingestion): `src/bin/ingest_races.rs`'s handling of
    // `SOURCEPAGE:p.xx` — the literal string PCGen leaves on the race chassis
    // rows where a real page citation belongs (`decisions.md` §25). Like
    // bucket D this is the *opposite* of the forbidden pattern: the ingest
    // detects that upstream placeholder and refuses to store it as a citation,
    // and these three lines are its tally `println!` and two assertion
    // messages proving it never leaks. Scoped by path *and* by the
    // distinctive `p.xx` literal, so an ordinary "placeholder" stub marker in
    // this same file still fails.
    //
    // Widened 2026-07-31 to the Pathfinder Unchained feature tables, which
    // carry the identical guard against the identical upstream token:
    // `rogue_features.rs` and `summoner_features.rs` each assert
    // `assert_ne!(page, "p.xx", ...)` over their ingested records, i.e. they
    // FAIL if PCGen's own placeholder is ever stored as a real page
    // citation. This test was red on the branch before that widening,
    // reported against those two lines, and the red was a false positive of
    // exactly bucket E's already-reviewed shape rather than a new stub. The
    // `p.xx` literal requirement is unchanged, so an ordinary "placeholder"
    // stub marker in either of those files still fails.
    //
    // Widened again 2026-08-12 to `race_trait_picker.rs`, which carries the
    // identical guard for the identical upstream token on a third family:
    // Core Essentials states `SOURCEPAGE:p.xx` on all 40 Tiefling heritage rows
    // and `xx` on all 24 Aasimar ones, `ingest_race_traits::is_placeholder_source_page`
    // drops them at ingest, and the picker's pin proves the panel therefore
    // shows no page rather than a fake one. That is bucket E's shape exactly —
    // an assertion message ABOUT the upstream placeholder, which is the
    // opposite of a stub marker. The `p.xx` literal requirement is unchanged,
    // so an ordinary "placeholder" stub marker in that file still fails.
    let is_pcgen_pxx_source_page_token = |line: &str| {
        let in_scoped_path = line.starts_with("src/bin/ingest_races.rs:")
            || line.starts_with("src/rules_core/rules_tables/pathfinder_unchained/")
            || line.starts_with("apps/desktop/src-tauri/src/race_trait_picker.rs:");
        in_scoped_path && line.contains("p.xx")
    };

    // Bucket F (AT-34-E3-001 fallout, gate-remediation cycle, 2026-09-01): five string-
    // literal (not `//`-comment) hits, all reviewed this cycle and confirmed non-stub --
    // opposite-of-a-stub anti-fabrication assertion text and real corpus-shape description,
    // the same discipline as buckets D/E, matched by each hit's own distinctive phrase so no
    // *different* future stub can silently ride along:
    // - `ingest_race_traits.rs`: a test assertion naming the real, corpus-confirmed "Human
    //   Ethnicity placeholder row" shape (the `human_ethnicity_{none,unknown}.json` records).
    // - `v06_work_inventory.rs`: a diagnostic panic message naming the real
    //   "vacuous-placeholder" atlas rung (`decisions.md §2`'s bucket-U sub-cause), not an
    //   unfinished-work marker.
    // - `class_feature_pool_catalog.rs` (4 hits): PCGen's own CHOOSE-menu "no selection"
    //   placeholder rows' data description, plus the anti-fabrication assertion guarding
    //   their structural shape -- both describe upstream PCGen placeholder data, not a stub
    //   in this codebase.
    // - `race_resolver.rs`: an anti-fabrication assertion ("must carry real corpus prose,
    //   not a fabricated placeholder") -- the opposite of the forbidden pattern, same as
    //   bucket D.
    let is_reviewed_placeholder_shape_text = |line: &str| {
        line.contains("Human Ethnicity placeholder row is not dropped")
            || line.contains("expected the vacuous-placeholder rung")
            || line.contains("PCGen's own CHOOSE-menu \"no selection\" placeholder row")
            || line.contains("carries a token beyond the placeholder's structural KEY/CATEGORY/TYPE")
            || line.contains("must carry real corpus prose, not a fabricated placeholder")
    };

    let unexplained: Vec<&String> = hits
        .iter()
        .filter(|line| {
            !is_ui_placeholder_text(line)
                && !is_documented_deferred_finding(line)
                && !is_reviewed_comment_prose(line)
                && !is_anti_fabrication_explanation_text(line)
                && !is_pcgen_pxx_source_page_token(line)
                && !is_reviewed_placeholder_shape_text(line)
        })
        .collect();

    assert!(
        unexplained.is_empty(),
        "wired-integration audit found `placeholder` hits in shipping source \
         that are neither UI-affordance text, the one documented deferred \
         finding (GE-07 pilot-shell-snapshot scaffold, see \
         risks-and-open-questions.md §5 Deferrals), nor reviewed comment \
         prose -- these look like new, unreviewed stub markers and must be \
         triaged before this test can pass:\n{}",
        unexplained
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// Check 2: no empty/no-op `onClick` handlers on user-facing affordances.
#[test]
fn no_noop_click_handlers_in_shipping_tsx() {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let output = Command::new("git")
        .args([
            "grep",
            "-nE",
            r"onClick=\{\s*\(\)\s*=>\s*\{\s*\}\s*\}|onClick=\{undefined",
            "--",
            "apps/desktop/*.tsx",
            "apps/desktop/**/*.tsx",
            "apps/desktop/**/*.jsx",
        ])
        .current_dir(repo_root)
        .output()
        .expect("git grep must be runnable inside a git checkout");
    let exit_code = output.status.code();
    assert!(
        exit_code == Some(0) || exit_code == Some(1),
        "git grep exited unexpectedly (status={exit_code:?}); stderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let hits = String::from_utf8_lossy(&output.stdout);
    let real_hits: Vec<&str> = hits
        .lines()
        .filter(|line| !(line.contains(".test.") || line.contains("__tests__/")))
        .collect();
    assert!(
        real_hits.is_empty(),
        "wired-integration audit found no-op onClick handlers in shipping tsx/jsx:\n{}",
        real_hits.join("\n")
    );
}

/// Check 3: no mock-library leaks (`mockResolvedValue`, `mockReturnValue(`,
/// `vi.mock(`, `__mocks__`) outside dedicated test files.
#[test]
fn no_mock_leaks_in_shipping_source() {
    let hits = git_grep_shipping_trees(r"mockResolvedValue|mockReturnValue\(|vi\.mock\(|__mocks__");
    assert!(
        hits.is_empty(),
        "wired-integration audit found mock-library leaks in shipping source:\n{}",
        hits.join("\n")
    );
}

/// Check 4: no `"Would …"` stub-return strings.
#[test]
fn no_would_strings_in_shipping_source() {
    let hits = git_grep_shipping_trees(r#""Would [^"]*""#);
    // Registry entry 0002 (`docs/governance/wired-integration-stubs-registry.md`):
    // same operator-approved `StubAdapter` exception as the zero-tolerance
    // check above, scoped identically by path -- whole `stub_adapter.rs`,
    // plus only the three named command files' literal `"Would render for
    // system ...; not yet implemented"` test-assertion lines. A `"Would
    // ..."` hit in any other file still fails.
    let is_registry_0002_stub_adapter_exception = |line: &str| {
        line.starts_with("apps/desktop/src-tauri/src/stub_adapter.rs:")
            || ((line.starts_with(
                "apps/desktop/src-tauri/src/characterHub/appendToCharacter.rs:",
            ) || line.starts_with(
                "apps/desktop/src-tauri/src/characterHub/reSaveCharacter.rs:",
            ) || line.starts_with(
                "apps/desktop/src-tauri/src/characterHub/recomputeCharacter.rs:",
            )) && line.contains("Would render for system")
                && line.contains("not yet implemented"))
    };
    let hits: Vec<String> = hits
        .into_iter()
        .filter(|line| !is_registry_0002_stub_adapter_exception(line))
        .collect();
    assert!(
        hits.is_empty(),
        "wired-integration audit found \"Would ...\" stub-return strings in shipping source:\n{}",
        hits.join("\n")
    );
}
