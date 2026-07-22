//! Epic 2 Criterion 2.5 — verification cycle for the pilot case.
//!
//! Wires the full Oracle-Harness Comparator pipeline this epic built across
//! Criteria 2.1-2.4 — `pcgen_runner` (real PCGen engine run + normalize) ->
//! `comparator` (real dimension-by-dimension compare) -> `parity_report`
//! (real Markdown report render + write) — end to end against the real PF1
//! Core Rulebook Human Fighter level 1 pilot case fixtures:
//! `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
//! (the Codex side, via the real `rules_core::pilot_compute` chassis) and
//! `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt`
//! (the pilot case identity/claim-status record).
//!
//! Per `epic-breakdown.md` Criterion 2.5, the golden fixture's
//! `current_claim_status` upgrades from `not_yet_grounded` to
//! `oracle_checked` ONLY if this real end-to-end run confirms genuine parity.
//! It does not: this repo (and the checked-out PCGen repo) hold no real
//! PCGen-native `.pcg` character file for this exact pilot build anywhere —
//! see SD-25's `pcgen_runner_smoke.rs` and this epic's own
//! `sd26_pcgen_runner.rs` `## DISCOVERED` history, which already established
//! that the only real, non-synthetic `.pcg` fixture available to exercise
//! the real PCGen Gradle engine is the bundled substitute
//! `code/testsuite/PCGfiles/pf_Paladin.pcg` — a materially different
//! character build (different race, class, and level) than the pilot's own
//! Human Fighter level 1. Running the real PCGen engine against that
//! substitute and comparing its real output against the pilot's real
//! computed Codex values is not a genuine same-character parity check; a
//! "match" would be coincidental and a "mismatch" would be expected but
//! uninformative about the pilot case's actual parity.
//!
//! This cycle therefore proves the full pipeline is genuinely wired
//! end-to-end (real PCGen engine invocation, real normalization/carry, real
//! comparator, real parity-report render + write to
//! `artifacts/oracle_validation/parity_report_pf1-crb-human-fighter-level1.md`)
//! without fabricating a parity verdict, and asserts — concretely, via the
//! `character.identity` dimension disagreeing — that this run cannot ground
//! genuine oracle-checked parity for the pilot case. The golden fixture's
//! `current_claim_status` therefore correctly remains `not_yet_grounded`,
//! both in the loaded, typed representation and in the fixture file on disk.
//!
//! ## DISCOVERED
//! No real PCGen-native `.pcg` character file exists anywhere in either repo
//! for the exact pilot build (PF1 Core Rulebook Human Fighter level 1). This
//! blocks a genuine oracle-checked upgrade for `pf1-crb-human-fighter-level1`
//! until either (a) a real `.pcg` matching the pilot's exact deterministic
//! input is hand-authored in the PCGen checkout (production data-authoring
//! work in a different repo, outside this cycle's `src/oracle_validation/`
//! file-touch grant), or (b) another legitimate, same-character oracle
//! source is identified. Forwarded as a real, structural blocker — not
//! self-healable inline.

use codex::oracle_validation::comparator::compare;
use codex::oracle_validation::golden_fixture::{load_golden_case_fixture, ClaimTier};
use codex::oracle_validation::normalization::default_normalization_rules;
use codex::oracle_validation::parity_report::{
    default_parity_report_dir, write_parity_report,
};
use codex::oracle_validation::pcgen_runner::{run_pcgen_character, PcgenRunOptions};
use codex::oracle_validation::selected_parity_dimensions::SelectedParityDimensions;
use codex::rules_core::character_input::load_character_input_fixture;
use codex::rules_core::pilot_compute::build_pilot_headless_receipt;

use std::path::PathBuf;

const DETERMINISTIC_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");
const GOLDEN_FIXTURE_TEXT: &str =
    include_str!("fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt");
const GOLDEN_FIXTURE_REPO_PATH: &str =
    "tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt";

const PILOT_CASE_ID: &str = "pf1-crb-human-fighter-level1";
const PILOT_SOURCE_PACKAGE_ID: &str = "pf1.core_rulebook";
const PILOT_LEGACY_ROUTE: &str =
    "headless Gradle run batch export via code/testsuite/base-xml.ftl";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn pcgen_repo_dir() -> PathBuf {
    std::env::var("PCGEN_REPO_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/home/ubuntu/workspace/repos/pcgen"))
}

/// Real, non-synthetic PCGen `.pcg` fixture bundled with PCGen's own test
/// suite. Same substitute `sd26_pcgen_runner.rs`'s real end-to-end test
/// already uses, since no real `.pcg` exists anywhere for the pilot case's
/// own Human Fighter level 1 build.
fn substitute_pcg_fixture() -> PathBuf {
    pcgen_repo_dir().join("code/testsuite/PCGfiles/pf_Paladin.pcg")
}

fn load_golden_fixture_or_panic() -> codex::oracle_validation::golden_fixture::GoldenCaseFixture {
    let result = load_golden_case_fixture(GOLDEN_FIXTURE_TEXT);
    assert!(
        result.diagnostics.is_empty(),
        "golden fixture should load cleanly: {:?}",
        result.diagnostics
    );
    result
        .fixture
        .expect("valid golden fixture should produce a GoldenCaseFixture record")
}

#[test]
fn golden_fixture_starts_this_cycle_at_not_yet_grounded() {
    let fixture = load_golden_fixture_or_panic();
    assert_eq!(fixture.case_id, PILOT_CASE_ID);
    assert_eq!(fixture.current_claim_status, ClaimTier::NotYetGrounded);
}

/// The full pipeline proof this criterion names: pcgen_runner (2.4) ->
/// comparator (2.1) -> parity_report (2.3), run for real against the pilot
/// case's real Codex-computed dimensions and a real PCGen engine invocation.
/// Confirms the pipeline is genuinely wired, then confirms — rather than
/// assumes — that this particular run cannot ground genuine parity for the
/// pilot case, so the golden fixture's `current_claim_status` correctly
/// stays at `not_yet_grounded` rather than being force-upgraded.
#[test]
fn full_pipeline_runs_end_to_end_and_the_pilot_case_stays_not_yet_grounded() {
    // --- Codex side: real, computed selected parity dimensions. ---
    let input_load = load_character_input_fixture(DETERMINISTIC_FIXTURE);
    assert!(
        input_load.diagnostics.is_empty(),
        "pilot deterministic input fixture should load cleanly: {:?}",
        input_load.diagnostics
    );
    let input = input_load
        .character_input
        .expect("valid deterministic input fixture should produce a CharacterInput record");
    let receipt = build_pilot_headless_receipt(&input);
    let codex_dims = SelectedParityDimensions::from_receipt(&receipt);
    assert!(
        !codex_dims.dimensions.is_empty(),
        "expected real computed Codex selected parity dimensions for the pilot case"
    );

    // --- PCGen side: a real end-to-end PCGen engine run via Criterion 2.4's wrapper. ---
    let pcg = substitute_pcg_fixture();
    assert!(
        pcg.is_file(),
        "expected a real bundled PCGen .pcg fixture at {} (checked-out PCGen repo required)",
        pcg.display()
    );

    let options = PcgenRunOptions::new(PILOT_CASE_ID, PILOT_SOURCE_PACKAGE_ID, PILOT_LEGACY_ROUTE);
    let pcgen_output = run_pcgen_character(&pcg, &options)
        .unwrap_or_else(|err| panic!("real PCGen engine run should succeed: {err}"));
    assert!(
        !pcgen_output.dimensions.is_empty(),
        "expected at least one real computed dimension from the genuine PCGen run"
    );
    let normalized = pcgen_output.to_normalized_output();

    // --- Comparator: real compare() between the real PCGen run and the real Codex dims. ---
    let comparison = compare(&normalized, &codex_dims);

    // --- Parity report: real render + write to the real default output path. ---
    let rules = default_normalization_rules();
    let report_path = write_parity_report(
        &default_parity_report_dir(),
        PILOT_CASE_ID,
        &comparison,
        &rules,
    )
    .expect("parity report should write successfully to the real default output directory");
    assert!(
        report_path.is_file(),
        "expected a real parity report file at {}",
        report_path.display()
    );
    let report_text = std::fs::read_to_string(&report_path)
        .unwrap_or_else(|err| panic!("written parity report should be readable: {err}"));
    assert!(
        report_text.contains(PILOT_CASE_ID),
        "parity report should name the pilot case id"
    );

    // --- The honest finding: this run is not a genuine same-character parity check. ---
    // The substitute PCGen run's own character identity necessarily disagrees with the
    // pilot Codex case's identity, since they are different character builds entirely.
    // This is concrete, structural evidence -- not an assumption -- that this run cannot
    // ground genuine oracle-checked parity for the pilot case.
    let identity_mismatch = comparison
        .mismatches
        .iter()
        .find(|m| m.dimension_id == "character.identity");
    assert!(
        identity_mismatch.is_some(),
        "expected the substitute PCGen run's character.identity to disagree with the pilot \
         Codex case's own identity (different character builds), proving this run cannot \
         substitute for a genuine same-character parity check: {:?}",
        comparison
    );

    // --- Because no genuine same-character oracle run exists, do not force the upgrade. ---
    let fixture_after = load_golden_fixture_or_panic();
    assert_eq!(
        fixture_after.current_claim_status,
        ClaimTier::NotYetGrounded,
        "pilot fixture must not be force-upgraded to oracle_checked without a genuine \
         same-character parity run"
    );

    // And the fixture file on disk (not just the in-memory constant) must still say so --
    // proving this cycle did not edit the fixture to fake the upgrade.
    let on_disk_path = repo_root().join(GOLDEN_FIXTURE_REPO_PATH);
    let on_disk_text = std::fs::read_to_string(&on_disk_path).unwrap_or_else(|err| {
        panic!(
            "golden fixture file should be readable at {}: {err}",
            on_disk_path.display()
        )
    });
    assert!(
        on_disk_text.contains("current_claim_status=not_yet_grounded"),
        "golden fixture file on disk must still read current_claim_status=not_yet_grounded \
         (no forced upgrade)"
    );
}
