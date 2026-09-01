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
//! A real, near-complete precursor `.pcg` was subsequently located in the
//! GE-05 artifact tree and is now vendored into this repository at
//! `docs/release/GE-05-oracle-validation-and-parity-harness/artifacts/pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg`
//! — proven to load in the real PCGen engine by GE-05's
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
//! Net result (at the CG-03 fix cycle): 8 of 9 selected parity dimensions
//! genuinely agree (`character.identity`, `defense.baseline_armor_class`,
//! all three `defense.total_save.*` dimensions, and all three
//! `skill.selected_modifier.*` dimensions including the two that cycle
//! fixed), and exactly one genuine mismatch remains
//! (`combat.baseline_melee_attack_bonus`, PCGen: 5, Codex: 6). Because a real
//! mismatch remains, `current_claim_status` correctly stays `not_yet_grounded`
//! — that cycle did NOT force the upgrade to `oracle_checked`.
//!
//! ## Wave-2 dimension expansion (v0.6 alpha swarm)
//! Backend's `b726a36` (comparator field-extraction) extended the PCGen
//! normalizer to also emit `encumbrance.carrying_capacity.{light,medium,
//! heavy}_max_lbs`, `encumbrance.total_carried_weight_lbs`, and
//! `durability.max_hp` — sourced from `encumbrance.rs`/`durability.rs`,
//! both grounded earlier in wave 2. That exposed a stale assertion here:
//! `SelectedParityDimensions::from_receipt` (the headless, corpus-free
//! adapter this test used) never populated those five dimensions, so they
//! surfaced as `MissingFromCodex` mismatches rather than real comparisons.
//! Backend's `2298780` added `from_pilot_receipt` — a new adapter reading
//! the corpus-aware `contract::PilotReceipt` (which already carries the
//! `encumbrance` field wired in `d475097`), rather than touching
//! `from_receipt` at all (zero blast radius on this file's siblings,
//! `ge06_selected_parity_dimensions.rs` / `sd26_comparator.rs`, which still
//! use `from_receipt` unchanged). QA (this cycle) switched this specific
//! test to `from_pilot_receipt`, since real PCGen-comparison coverage for
//! durability/encumbrance is strictly better than a permanent
//! `MissingFromCodex` gap, and PCGen's real export values for this pilot
//! case were independently confirmed to already match Codex's own
//! `encumbrance.rs`/`durability.rs` formulas (light/medium/heavy 100/200/300
//! lbs at Strength 18 post-CG-03, 29 lbs carried, 12 max HP) before this
//! switch — so the expectation below is a genuine 13-of-14 match, not a
//! guess. `combat.baseline_melee_attack_bonus` remains the one real,
//! previously-diagnosed, still-unfixed mismatch.

use codex::oracle_validation::comparator::{compare, MismatchReason};
use codex::oracle_validation::golden_fixture::{load_golden_case_fixture, ClaimTier};
use codex::oracle_validation::normalization::default_normalization_rules;
use codex::oracle_validation::parity_report::{
    default_parity_report_dir, write_parity_report,
};
use codex::oracle_validation::pcgen_runner::{run_pcgen_character, PcgenRunOptions};
use codex::oracle_validation::selected_parity_dimensions::SelectedParityDimensions;
use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
use codex::rules_core::character_input::load_character_input_fixture;
use codex::rules_core::contract::to_pilot_receipt;
use codex::rules_core::pilot_compute_corpus::compute_pilot_with_corpus;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

use std::path::{Path, PathBuf};

/// The exact GE-06 deterministic posture's equipment records (Longsword +
/// Chain Shirt, `WT:4`/`WT:25` — the real weights PCGen's own export
/// reports as 29 lbs total carried). Mirrors
/// `tests/sd20_tabletop_readiness_integration.rs`'s own
/// `FIGHTER_GEAR_FIXTURE_TEXT` (same posture, same fixture shape) rather
/// than sharing it directly, since this file has no dependency on that
/// one today and duplicating a small, stable fixture text is cheaper than
/// introducing a cross-file coupling for it.
const FIGHTER_GEAR_FIXTURE_TEXT: &str = "\
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Resizable.Melee.Martial.OneHanded.Slashing.Sword.BladeHeavy.Weapon Group Blades Heavy\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\tWIELD:OneHanded\tSIZE:M
Chain Shirt\tKEY:Chain Shirt (Base)\tTYPE:Armor.Light\tCOST:100\tWT:25\tACCHECK:-2\tMAXDEX:4\tSPELLFAILURE:20\tBONUS:COMBAT|AC|4|TYPE=Armor|PREVAREQ:DisableArmorBonus,0
Masterwork (Weapon)\tKEY:Special Quality ~ Masterwork ~ Weapon\tTYPE:MasterworkQuality.Weapon\tCOST:0\tBONUS:WEAPON|TOHIT|1|TYPE=Enhancement
";

fn corpus_with_fighter_gear() -> SourcePackageContent<'static> {
    let result = parse_equipment_entries("cr_equip_arms_armor.lst", FIGHTER_GEAR_FIXTURE_TEXT);
    assert!(
        result.diagnostics.is_empty(),
        "fixture text must parse cleanly: {:?}",
        result.diagnostics
    );
    let source_ref = SourceRef { lst_file: "cr_equip_arms_armor.lst".to_string(), line: 1 };
    let mut corpus = SourcePackageContent::empty("core_rulebook", source_ref);
    for record in result.entries {
        let record: &'static EquipmentRecord = Box::leak(Box::new(record));
        corpus.push(convert_equipment_record(record));
    }
    corpus
}

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

/// Repo-relative path to the completed, genuinely same-character pilot
/// `.pcg` fixture, vendored into the GE-05 build's own artifact folder
/// alongside its runtime-evidence receipts.
///
/// This was previously a hardcoded absolute path into another machine's
/// `programs/codex/requirements/` *planning* tree, with no environment
/// override, which made this suite unrunnable anywhere but that one box.
/// Build artifacts belong in the build's own artifact folder rather than
/// behind a reference to an external source — see
/// `tests/ge05_vendored_pcg_fixtures.rs` for the provenance and integrity
/// proof, and `docs/release/GE-05-oracle-validation-and-parity-harness/artifacts/README.md`
/// for the recorded provenance.
const PILOT_PCG_FIXTURE_PATH: &str =
    "docs/release/GE-05-oracle-validation-and-parity-harness/artifacts/\
     pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg";

/// sha256 of the vendored fixture as committed. Pinned so a silently
/// swapped or regenerated `.pcg` fails loudly here instead of quietly
/// shifting the parity numbers this suite publishes as evidence. Kept in
/// sync with `tests/ge05_vendored_pcg_fixtures.rs`.
const PILOT_PCG_FIXTURE_SHA256: &str =
    "d0c6b2a2e9c190d0be97c20caf247b96108299340331d044547d9a57bdb64f4f";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The real, completed, genuinely same-character PCGen `.pcg` fixture for
/// this pilot case (see this module's doc comment for provenance).
///
/// Resolution follows `authoring_workbench::resolve_package_path`'s contract —
/// repo-relative paths anchor at the codex repo root. That helper lives in
/// the separate `codex-desktop` crate and is not reachable from a `codex`
/// root-crate integration test, so this uses the identical anchor its
/// fallback uses (`CARGO_MANIFEST_DIR`), which is also the in-crate pattern
/// `sd27_advanced_race_guide_parity.rs` already established for its own
/// vendored `.pcg` fixture.
fn pilot_case_pcg_fixture() -> PathBuf {
    repo_root().join(PILOT_PCG_FIXTURE_PATH)
}

fn default_pcgen_repo_dir() -> PathBuf {
    if let Ok(configured) = std::env::var("PCGEN_REPO_DIR") {
        return PathBuf::from(configured);
    }
    let home = std::env::var("HOME")
        .expect("HOME must be set to locate the default PCGen repo checkout");
    PathBuf::from(home).join("workspace/repos/pcgen")
}

fn pcgen_gradle_wrapper_is_runnable(pcgen_repo_dir: &Path) -> bool {
    let gradlew = pcgen_repo_dir.join("gradlew");
    if !gradlew.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::metadata(&gradlew)
            .map(|meta| (meta.permissions().mode() & 0o111) != 0)
            .unwrap_or(false)
    }
    #[cfg(not(unix))]
    {
        true
    }
}

/// Fail loudly on a swapped fixture rather than reporting parity numbers
/// computed from content nobody verified.
fn assert_pilot_pcg_fixture_is_pinned(path: &std::path::Path) {
    let actual = codex::rules_core::cache_gen::apg::sha256_file(path)
        .unwrap_or_else(|err| panic!("cannot hash {}: {err}", path.display()));
    assert_eq!(
        actual,
        PILOT_PCG_FIXTURE_SHA256,
        "{} was swapped, regenerated, or truncated -- re-verify the fixture and re-record the \
         digest before this suite's parity results can be trusted",
        path.display()
    );
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

#[test]
fn gradle_wrapper_runnable_check_requires_executable_gradlew() {
    let temp = std::env::temp_dir().join(format!(
        "sd26-pilot-case-verification-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos()
    ));
    std::fs::create_dir_all(&temp).expect("temp directory should be creatable");
    assert!(
        !pcgen_gradle_wrapper_is_runnable(&temp),
        "missing gradlew must be treated as not runnable"
    );

    let gradlew = temp.join("gradlew");
    std::fs::write(&gradlew, "#!/usr/bin/env bash\n").expect("gradlew placeholder should write");

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&gradlew, std::fs::Permissions::from_mode(0o644))
            .expect("permissions should update");
        assert!(
            !pcgen_gradle_wrapper_is_runnable(&temp),
            "non-executable gradlew must be treated as not runnable"
        );

        std::fs::set_permissions(&gradlew, std::fs::Permissions::from_mode(0o755))
            .expect("permissions should update");
    }

    assert!(
        pcgen_gradle_wrapper_is_runnable(&temp),
        "present executable gradlew should be treated as runnable"
    );
    std::fs::remove_dir_all(&temp).expect("temp directory should be removable");
}

/// The full pipeline proof this criterion names: pcgen_runner (2.4) ->
/// comparator (2.1) -> parity_report (2.3), run for real against the pilot
/// case's real Codex-computed dimensions and a real PCGen engine invocation
/// against the completed, genuinely same-character pilot `.pcg`. Confirms the
/// pipeline is genuinely wired and, per this file's module doc comment,
/// confirms 13 of 14 dimensions now agree (the CG-03 Climb/Swim fixes, plus
/// wave-2's five new durability/encumbrance dimensions once genuinely
/// compared via `from_pilot_receipt` instead of left as `MissingFromCodex`)
/// while one real, unfixed mismatch (`combat.baseline_melee_attack_bonus`)
/// remains — so the golden fixture's `current_claim_status` correctly stays
/// at `not_yet_grounded` rather than being force-upgraded.
#[test]
fn full_pipeline_runs_end_to_end_and_finds_one_genuine_attack_bonus_mismatch() {
    let pcgen_repo_dir = default_pcgen_repo_dir();
    if !pcgen_gradle_wrapper_is_runnable(&pcgen_repo_dir) {
        eprintln!(
            "[skip] sd26_pilot_case_verification: real PCGen Gradle wrapper not found/executable at {} \
             (set $PCGEN_REPO_DIR to a checked-out PCGen repo to run this end-to-end; \
             GitHub Actions runners do not check out the companion PCGen repo)",
            pcgen_repo_dir.join("gradlew").display()
        );
        return;
    }

    // --- Codex side: real, computed selected parity dimensions, via the
    // corpus-aware PilotReceipt (from_pilot_receipt) so durability/encumbrance
    // are genuinely compared against PCGen rather than left MissingFromCodex. ---
    let input_load = load_character_input_fixture(DETERMINISTIC_FIXTURE);
    assert!(
        input_load.diagnostics.is_empty(),
        "pilot deterministic input fixture should load cleanly: {:?}",
        input_load.diagnostics
    );
    let input = input_load
        .character_input
        .expect("valid deterministic input fixture should produce a CharacterInput record");
    let corpus = corpus_with_fighter_gear();
    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let pilot_receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);
    let codex_dims = SelectedParityDimensions::from_pilot_receipt(
        &pilot_receipt,
        &input.chosen.class_levels,
        input.case_id.as_deref(),
        &input.source_package_id,
    );
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
    assert_pilot_pcg_fixture_is_pinned(&pcg);

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

    // --- The real, genuine finding: identity still agrees (this remains a real
    // same-character run), Climb/Swim genuinely match (the CG-03 fix), and the five
    // wave-2 durability/encumbrance dimensions now genuinely match too (PCGen's real
    // export values already agreed with Codex's own formulas -- see this file's module
    // doc comment) -- 13 of 14 dimensions match. Only combat.baseline_melee_attack_bonus,
    // previously masked by a compensating error, genuinely diverges. ---
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
        "durability.max_hp",
        "encumbrance.carrying_capacity.light_max_lbs",
        "encumbrance.carrying_capacity.medium_max_lbs",
        "encumbrance.carrying_capacity.heavy_max_lbs",
        "encumbrance.total_carried_weight_lbs",
    ] {
        assert!(
            matched_ids.contains(&expected_match),
            "expected dimension '{expected_match}' to genuinely match between the real PCGen \
             run and the real Codex computation: {:?}",
            comparison
        );
    }
    // The five wave-2 dimensions' real matched values, not just their presence --
    // independently confirming the numbers, not just that a comparison happened.
    for (dimension_id, expected_value) in [
        ("durability.max_hp", 12i16),
        ("encumbrance.carrying_capacity.light_max_lbs", 100i16),
        ("encumbrance.carrying_capacity.medium_max_lbs", 200i16),
        ("encumbrance.carrying_capacity.heavy_max_lbs", 300i16),
        ("encumbrance.total_carried_weight_lbs", 29i16),
    ] {
        let matched = comparison
            .matches
            .iter()
            .find(|m| m.dimension_id == dimension_id)
            .unwrap_or_else(|| panic!("expected a real match for dimension '{dimension_id}': {comparison:?}"));
        assert_eq!(matched.pcgen_value_i16, Some(expected_value), "{dimension_id}: {matched:?}");
        assert_eq!(matched.codex_value_i16, Some(expected_value), "{dimension_id}: {matched:?}");
    }
    assert_eq!(
        comparison.matches.len(),
        14,
        "expected exactly 14 genuinely matching dimensions (8 original + 5 wave-2 \
         durability/encumbrance dimensions + combat.base_attack_bonus): {:?}",
        comparison
    );

    // The one remaining genuine mismatch: combat.baseline_melee_attack_bonus. This is NOT
    // the Strength-modifier bug this cycle fixed (Climb/Swim now confirm that fix is
    // correct) -- it is a second, previously-masked, structurally distinct discrepancy
    // (see this file's module doc comment) that this cycle deliberately does not attempt
    // to fix, since it is undiagnosed and out of the narrowly-authorized CG-03 scope.
    {
        let (dimension_id, pcgen_value, codex_value) = ("combat.baseline_melee_attack_bonus", 5i16, 6i16);
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
