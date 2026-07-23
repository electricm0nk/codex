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
//!
//! ## Follow-up cycle (this file's current state)
//! The first 2.5 cycle found no real PCGen-native `.pcg` character file
//! anywhere for the exact pilot build, and ran the pipeline against the
//! bundled substitute `code/testsuite/PCGfiles/pf_Paladin.pcg` instead —
//! proving the pipeline was wired end to end without fabricating parity (see
//! this file's git history and
//! `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/artifacts/epic_2/pilot_case_oracle_checked-cycle_receipt.md`).
//!
//! A real, near-complete precursor `.pcg` was subsequently located at
//! `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg`
//! (outside this repo) — proven to load in the real PCGen engine by GE-05's
//! own runtime-output receipt. This follow-up cycle completed that file (added
//! the Dodge and Weapon Focus (Longsword) feats, Climb/Intimidate/Swim rank-1
//! skill allocations, and the Chain Shirt + Longsword equipped/active
//! loadout with no shield — the exact GE-06 deterministic input contract) and
//! renamed its `CHARACTERNAME` to the pilot's own `case_id` so the
//! `character.identity` dimension carries a genuine same-character signal
//! rather than an arbitrary display name. Full details:
//! `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/artifacts/epic_2/pilot_case_oracle_checked-followup-cycle_receipt.md`.
//!
//! Running the real pipeline against this completed, genuinely same-character
//! `.pcg` produces a real, informative result: 7 of 9 selected parity
//! dimensions agree (`character.identity`,
//! `combat.baseline_melee_attack_bonus`, `defense.baseline_armor_class`, all
//! three `defense.total_save.*` dimensions, and
//! `skill.selected_modifier.intimidate`), but
//! `skill.selected_modifier.climb` and `skill.selected_modifier.swim`
//! genuinely disagree (PCGen: 6, Codex: 5). This is a real, structural
//! finding, not a normalization artifact: Codex's
//! `pilot_compute::compute_ability_modifiers` derives `AbilityModifiers`
//! directly from the chosen ability *scores* and never actually folds in the
//! chosen Human `+2 Strength` racial ability bonus (`choice:human_ability_bonus`)
//! before computing the Strength modifier, even though
//! `explain_human_pilot_race_seam` emits an explanation record that narrates
//! the bonus. So `ability_modifiers.strength` is `+3` (from the raw score 16)
//! rather than the correct `+4` (from the effective, bonus-applied score 18).
//! `combat.baseline_melee_attack_bonus` still coincidentally matches PCGen
//! (Codex: BAB +1 + STR +3 + Weapon Focus +1 = 5; PCGen: BAB +1 + STR +4 + 0
//! generic-melee Weapon Focus contribution = 5) — the two systems reach the
//! same total via different, non-equivalent arithmetic, which is a real
//! observation worth flagging even though it does not itself fail this
//! dimension's exact-value comparison. `skill.selected_modifier.climb`/`swim`
//! have no such compensating term, so the missing `+1` from the unapplied
//! racial Strength bonus surfaces directly as a real value mismatch.
//!
//! Because two real dimension mismatches remain, `current_claim_status`
//! correctly stays `not_yet_grounded` — this cycle does NOT force the
//! upgrade. The blocker is forwarded as a real, structural, self-healable
//! (in a future rules_core cycle, not in-scope here) Codex bug: apply chosen
//! racial ability-score bonuses before deriving `AbilityModifiers`, not just
//! after, in a narrative-only explanation record.

use codex::oracle_validation::comparator::{compare, MismatchReason};
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

/// Absolute path to the completed, genuinely same-character pilot `.pcg`
/// fixture. Lives outside this repo (in the `programs/codex/requirements/`
/// GE-05 artifact tree, alongside its own runtime-evidence receipt), the
/// same way `PcgenRunOptions::pcgen_repo_dir` already points at an
/// out-of-repo PCGen checkout — `pcgen_runner.rs::run_pcgen_character` takes
/// any real absolute `.pcg` path, so no in-repo copy or move is required.
const PILOT_PCG_FIXTURE_PATH: &str = "/home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The real, completed, genuinely same-character PCGen `.pcg` fixture for
/// this pilot case (see this module's doc comment for provenance).
fn pilot_case_pcg_fixture() -> PathBuf {
    PathBuf::from(PILOT_PCG_FIXTURE_PATH)
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
/// case's real Codex-computed dimensions and a real PCGen engine invocation
/// against the completed, genuinely same-character pilot `.pcg`. Confirms
/// the pipeline is genuinely wired, confirms 7 of 9 dimensions now agree
/// (proving this is a real same-character comparison, unlike the prior
/// cycle's substitute-fixture run), and confirms the two real remaining
/// mismatches (Climb/Swim) are a genuine Codex computation gap rather than a
/// fixture or pipeline defect — so the golden fixture's
/// `current_claim_status` correctly stays at `not_yet_grounded` rather than
/// being force-upgraded.
#[test]
fn full_pipeline_runs_end_to_end_and_finds_two_genuine_skill_mismatches() {
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

    // --- PCGen side: a real end-to-end PCGen engine run via Criterion 2.4's wrapper,
    // against the completed, genuinely same-character pilot `.pcg`. ---
    let pcg = pilot_case_pcg_fixture();
    assert!(
        pcg.is_file(),
        "expected the real completed pilot PCGen .pcg fixture at {}",
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

    // --- The real, genuine finding: identity now agrees (proving this is a real
    // same-character run, not the prior cycle's substitute-fixture run), and 7 of 9
    // dimensions match, but two real mismatches remain. ---
    let matched_ids: Vec<&str> = comparison
        .matches
        .iter()
        .map(|m| m.dimension_id.as_str())
        .collect();
    for expected_match in [
        "character.identity",
        "combat.baseline_melee_attack_bonus",
        "defense.baseline_armor_class",
        "defense.total_save.fortitude",
        "defense.total_save.reflex",
        "defense.total_save.will",
        "skill.selected_modifier.intimidate",
    ] {
        assert!(
            matched_ids.contains(&expected_match),
            "expected dimension '{expected_match}' to genuinely match between the real PCGen \
             run and the real Codex computation: {:?}",
            comparison
        );
    }
    assert_eq!(
        comparison.matches.len(),
        7,
        "expected exactly 7 genuinely matching dimensions: {:?}",
        comparison
    );

    // The two genuine mismatches: Climb and Swim, both off by exactly the missing Human
    // +2 Strength racial ability bonus that `pilot_compute::compute_ability_modifiers`
    // never actually folds into `AbilityModifiers.strength` (see this file's module doc
    // comment for the full root-cause analysis).
    for (dimension_id, pcgen_value, codex_value) in [
        ("skill.selected_modifier.climb", 6i16, 5i16),
        ("skill.selected_modifier.swim", 6i16, 5i16),
    ] {
        let mismatch = comparison
            .mismatches
            .iter()
            .find(|m| m.dimension_id == dimension_id)
            .unwrap_or_else(|| {
                panic!("expected a real mismatch for dimension '{dimension_id}': {comparison:?}")
            });
        assert_eq!(mismatch.reason, MismatchReason::ValueMismatch);
        assert_eq!(mismatch.pcgen_value_i16, Some(pcgen_value));
        assert_eq!(mismatch.codex_value_i16, Some(codex_value));
    }
    assert_eq!(
        comparison.mismatches.len(),
        2,
        "expected exactly the two genuine Climb/Swim mismatches and no others: {:?}",
        comparison
    );
    assert!(!comparison.all_matched());

    // --- Because real mismatches remain, do not force the upgrade. ---
    let fixture_after = load_golden_fixture_or_panic();
    assert_eq!(
        fixture_after.current_claim_status,
        ClaimTier::NotYetGrounded,
        "pilot fixture must not be force-upgraded to oracle_checked while real Climb/Swim \
         mismatches remain"
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
