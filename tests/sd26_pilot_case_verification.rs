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
//! ## CG-03 ability-modifier fix cycle (this file's current state)
//! A prior cycle found the result above and diagnosed it: Codex's
//! `pilot_compute::compute_ability_modifiers` derived `AbilityModifiers`
//! directly from the chosen ability *scores* and never actually folded in the
//! chosen Human `+2 Strength` racial ability bonus (`choice:human_ability_bonus`)
//! before computing the Strength modifier, even though
//! `explain_human_pilot_race_seam` emitted an explanation record that narrated
//! the bonus without ever applying it. So `ability_modifiers.strength` was
//! `+3` (from the raw score 16) instead of the correct `+4` (from the
//! effective, bonus-applied score 18), which produced two genuine
//! `skill.selected_modifier.{climb,swim}` mismatches (PCGen: 6, Codex: 5).
//!
//! This cycle fixed that bug: `pilot_compute::apply_human_ability_bonus` now
//! applies the Human ability-bonus choice's +2 to the targeted ability's score
//! BEFORE ability modifiers are derived (gated strictly on `race:human` plus a
//! resolved `choice:human_ability_bonus` selection), with a
//! `race.human.ability_bonus_applied` explanation recording the real
//! arithmetic. Re-running the real pipeline against the fix confirms it is
//! correct: `skill.selected_modifier.climb` and `skill.selected_modifier.swim`
//! now genuinely agree with PCGen (both now 6, matching), directly confirming
//! `ability_modifiers.strength` is now the correct `+4`.
//!
//! Fixing the Strength modifier unmasked a SECOND, previously-hidden, genuinely
//! different bug in `combat.baseline_melee_attack_bonus`: before this fix,
//! Codex's total (BAB +1 + STR +3 + Weapon Focus +1 = 5) coincidentally matched
//! PCGen's total (BAB +1 + STR +4 + 0 generic-melee Weapon Focus contribution =
//! 5) via a compensating error the prior cycle's doc comment explicitly
//! flagged as "non-equivalent arithmetic" reaching the same total by luck. Now
//! that Strength is correctly +4, Codex's total is BAB +1 + STR +4 + Weapon
//! Focus +1 = 6, which genuinely diverges from PCGen's 5. This is a real,
//! newly-exposed mismatch, not something this cycle's fix broke: Codex's
//! `combat.baseline_melee_attack_bonus` is explicitly documented (see
//! `compute_combat_baseline` in `pilot_compute.rs`) as "the Longsword" specific
//! attack bonus, which legitimately includes Weapon Focus (Longsword); whether
//! PCGen's compared export field is a genuinely different (weapon-agnostic)
//! quantity, or the oracle-harness normalization maps the wrong PCGen field to
//! this dimension, is undiagnosed and OUT OF SCOPE for the narrowly-authorized
//! Human ability-modifier fix this cycle performed. Per the no-stub-mvp
//! doctrine's "real failure over fake success" principle, this cycle does NOT
//! attempt to paper over that second, not-yet-understood discrepancy — it is
//! forwarded as a new, separate open item (see this repo's SD-26 progress doc,
//! `## Open blockers`, for the follow-up tracking entry).
//!
//! Net result: 8 of 9 selected parity dimensions now genuinely agree
//! (`character.identity`, `defense.baseline_armor_class`, all three
//! `defense.total_save.*` dimensions, and all three
//! `skill.selected_modifier.*` dimensions including the two this cycle fixed),
//! and exactly one genuine mismatch remains
//! (`combat.baseline_melee_attack_bonus`, PCGen: 5, Codex: 6). Because a real
//! mismatch remains, `current_claim_status` correctly stays `not_yet_grounded`
//! — this cycle does NOT force the upgrade to `oracle_checked`.

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
/// against the completed, genuinely same-character pilot `.pcg`. Confirms the
/// pipeline is genuinely wired and, after the CG-03 Human ability-modifier
/// fix, confirms 8 of 9 dimensions now agree (the two prior Climb/Swim
/// mismatches are now fixed for real, directly confirming the ability-modifier
/// fix is correct) while one real, DIFFERENT mismatch
/// (`combat.baseline_melee_attack_bonus`) is newly exposed — a previously
/// masked, structurally distinct discrepancy this cycle does not attempt to
/// fix (see this file's module doc comment) — so the golden fixture's
/// `current_claim_status` correctly stays at `not_yet_grounded` rather than
/// being force-upgraded.
#[test]
fn full_pipeline_runs_end_to_end_and_finds_one_genuine_attack_bonus_mismatch() {
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

    // --- The real, genuine finding after the CG-03 fix: identity still agrees (this
    // remains a real same-character run), and Climb/Swim now GENUINELY match too (proving
    // the Human ability-modifier fix is correct) -- 8 of 9 dimensions match. Only
    // combat.baseline_melee_attack_bonus, previously masked by a compensating error, now
    // genuinely diverges. ---
    let matched_ids: Vec<&str> = comparison
        .matches
        .iter()
        .map(|m| m.dimension_id.as_str())
        .collect();
    for expected_match in [
        "character.identity",
        "defense.baseline_armor_class",
        "defense.total_save.fortitude",
        "defense.total_save.reflex",
        "defense.total_save.will",
        "skill.selected_modifier.climb",
        "skill.selected_modifier.intimidate",
        "skill.selected_modifier.swim",
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
        8,
        "expected exactly 8 genuinely matching dimensions (Climb/Swim now fixed for real): {:?}",
        comparison
    );

    // The one remaining genuine mismatch: combat.baseline_melee_attack_bonus. This is NOT
    // the Strength-modifier bug this cycle fixed (Climb/Swim now confirm that fix is
    // correct) -- it is a second, previously-masked, structurally distinct discrepancy
    // (see this file's module doc comment) that this cycle deliberately does not attempt
    // to fix, since it is undiagnosed and out of the narrowly-authorized CG-03 scope.
    for (dimension_id, pcgen_value, codex_value) in
        [("combat.baseline_melee_attack_bonus", 5i16, 6i16)]
    {
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
        1,
        "expected exactly the one genuine baseline-attack-bonus mismatch and no others: {:?}",
        comparison
    );
    assert!(!comparison.all_matched());

    // --- Because a real mismatch remains, do not force the upgrade. ---
    let fixture_after = load_golden_fixture_or_panic();
    assert_eq!(
        fixture_after.current_claim_status,
        ClaimTier::NotYetGrounded,
        "pilot fixture must not be force-upgraded to oracle_checked while a real \
         combat.baseline_melee_attack_bonus mismatch remains"
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
