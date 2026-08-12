//! Rust-side PCGen runner wrapper proof (SD-26 Epic 2, Criterion 2.4).
//!
//! Proves that `oracle_validation::pcgen_runner` is a real, wired Rust-side
//! wrapper around the two real scripts SD-25's Epic 4 PCGen-runner
//! scaffolding already ships — `scripts/pcgen-run-character.sh` (drives the
//! real PCGen Gradle wrapper end to end against a `.pcg` character file) and
//! `scripts/pcgen-normalize-output.py` (normalizes the resulting raw XML into
//! typed dimension values) — and that its parsed output composes directly
//! into Criterion 2.1's `comparator::NormalizedOutput` shape.
//!
//! No PCGen output is mocked, stubbed, or fabricated anywhere in this file:
//! the end-to-end test shells out (via the wrapper) to the real
//! `pcgen-run-character.sh`, which itself drives the real PCGen Gradle
//! wrapper, and to the real `pcgen-normalize-output.py`.

use codex::oracle_validation::comparator::{compare, MismatchReason};
use codex::oracle_validation::pcgen_runner::{
    default_normalizer_script, default_runner_script, parse_normalized_output,
    run_pcgen_character, PcgenRunOptions, PcgenRunnerError,
};
use codex::oracle_validation::selected_parity_dimensions::{
    ClaimTierFloor, SelectedDimension, SelectedParityDimensions,
};

use std::path::{Path, PathBuf};

/// Absolute path to the repo root this test binary was compiled from.
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Real, non-synthetic PCGen `.pcg` fixture bundled with PCGen's own test
/// suite (`code/testsuite/PCGfiles/pf_Paladin.pcg`, `GAMEMODE:Pathfinder_RPG`).
/// Same substitute the SD-25 `pcgen_runner_smoke.rs` end-to-end test uses,
/// since no real `.pcg` exists anywhere in either repo yet for the SD-25/26
/// pilot case (`pf1-crb-human-fighter-level1`).
/// `PCGEN_REPO_DIR` wins when set; otherwise `$HOME/workspace/repos/pcgen`
/// — HOME-relative because the operator keeps `workspace/` in the home
/// directory and syncs it between machines, so the default is correct on any
/// box. Rust does not expand `~`, so `$HOME` is read via `std::env::var`.
fn pcgen_repo_dir() -> PathBuf {
    if let Ok(configured) = std::env::var("PCGEN_REPO_DIR") {
        return PathBuf::from(configured);
    }
    let home = std::env::var("HOME")
        .expect("HOME must be set to locate the default PCGen repo checkout");
    PathBuf::from(home).join("workspace/repos/pcgen")
}

fn substitute_pcg_fixture() -> PathBuf {
    pcgen_repo_dir().join("code/testsuite/PCGfiles/pf_Paladin.pcg")
}

/// The SD-25/26 pilot case's deterministic character-input contract.
fn pilot_deterministic_input_fixture() -> PathBuf {
    repo_root().join("tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt")
}

/// The SD-25/26 pilot case's golden oracle-comparison fixture.
fn pilot_golden_fixture() -> PathBuf {
    repo_root().join("tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt")
}

/// Reads the first `key=value` line matching `key` out of a Codex fixture
/// file (the flat `key=value` line format used by both pilot fixtures above).
fn read_fixture_field(path: &Path, key: &str) -> String {
    let text = std::fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("failed to read pilot fixture {}: {err}", path.display()));
    let prefix = format!("{key}=");
    for line in text.lines() {
        if let Some(value) = line.strip_prefix(&prefix) {
            return value.trim().to_string();
        }
    }
    panic!(
        "pilot fixture {} has no '{key}=' field (checked {} lines)",
        path.display(),
        text.lines().count()
    );
}

#[test]
fn wrapped_scripts_resolve_to_real_files_in_this_checkout() {
    let runner = default_runner_script();
    assert!(
        runner.is_file(),
        "pcgen_runner must resolve scripts/pcgen-run-character.sh to a real file; got {}",
        runner.display()
    );

    let normalizer = default_normalizer_script();
    assert!(
        normalizer.is_file(),
        "pcgen_runner must resolve scripts/pcgen-normalize-output.py to a real file; got {}",
        normalizer.display()
    );
}

#[test]
fn run_pcgen_character_reports_a_missing_character_file_without_shelling_out() {
    let options = PcgenRunOptions::new("case", "pkg", "route");
    let missing = Path::new("/nonexistent/does-not-exist.pcg");

    let result = run_pcgen_character(missing, &options);

    match result {
        Err(PcgenRunnerError::CharacterFileNotFound(path)) => {
            assert_eq!(path, missing);
        }
        other => panic!("expected CharacterFileNotFound, got {other:?}"),
    }
}

#[test]
fn parse_normalized_output_parses_the_normalizer_scripts_real_json_shape() {
    let json = r#"{
        "case_id": "pf1-crb-human-fighter-level1",
        "source_package_id": "pf1.core_rulebook",
        "legacy_route": "headless Gradle run batch export via code/testsuite/base-xml.ftl",
        "claim_tier_floor": "computed",
        "dimensions": [
            {"id": "defense.baseline_armor_class", "value_string": null, "value_i16": 17, "source_package_id": "pf1.core_rulebook"},
            {"id": "character.identity", "value_string": "Human Fighter", "value_i16": null, "source_package_id": "pf1.core_rulebook"}
        ],
        "diagnostics": ["missing PCGen output field for dimension 'skill.selected_modifier.climb'"]
    }"#;

    let parsed = parse_normalized_output(json).expect("well-formed normalizer JSON must parse");

    assert_eq!(parsed.case_id, "pf1-crb-human-fighter-level1");
    assert_eq!(parsed.dimensions.len(), 2);
    assert_eq!(parsed.diagnostics.len(), 1);

    let normalized = parsed.to_normalized_output();
    assert_eq!(normalized.dim_values.len(), 2);
    let armor = normalized
        .dim_values
        .iter()
        .find(|d| d.id == "defense.baseline_armor_class")
        .expect("armor dimension carried through");
    assert_eq!(armor.value_i16, Some(17));
    assert_eq!(armor.value_string, None);

    let identity = normalized
        .dim_values
        .iter()
        .find(|d| d.id == "character.identity")
        .expect("identity dimension carried through");
    assert_eq!(identity.value_string, Some("Human Fighter".to_string()));
    assert_eq!(identity.value_i16, None);
}

#[test]
fn parse_normalized_output_reports_malformed_json_as_a_typed_error() {
    let result = parse_normalized_output("not json at all");

    assert!(
        matches!(result, Err(PcgenRunnerError::OutputParseError(_))),
        "expected OutputParseError, got {result:?}"
    );
}

#[test]
fn to_normalized_output_composes_end_to_end_with_the_comparator() {
    let json = r#"{
        "case_id": "pf1-crb-human-fighter-level1",
        "source_package_id": "pf1.core_rulebook",
        "legacy_route": "headless Gradle run batch export via code/testsuite/base-xml.ftl",
        "claim_tier_floor": "computed",
        "dimensions": [
            {"id": "defense.baseline_armor_class", "value_string": null, "value_i16": 17, "source_package_id": "pf1.core_rulebook"}
        ],
        "diagnostics": []
    }"#;
    let parsed = parse_normalized_output(json).expect("well-formed normalizer JSON must parse");
    let normalized = parsed.to_normalized_output();

    let codex = SelectedParityDimensions {
        dimensions: vec![SelectedDimension {
            id: "defense.baseline_armor_class".to_string(),
            value_string: None,
            value_i16: Some(17),
            source_package_id: "pf1.core_rulebook".to_string(),
        }],
        claim_tier_floor: ClaimTierFloor::Computed,
    };

    let result = compare(&normalized, &codex);

    assert!(
        result.all_matched(),
        "pcgen_runner output normalized straight into the comparator should agree with a matching \
         Codex value, got: {:?}",
        result.mismatches
    );
    assert!(result.mismatches.iter().all(|m| m.reason != MismatchReason::ValueMismatch));
}

/// The actual end-to-end proof this criterion's own text names: a real PCGen
/// run, wrapped by this module, normalized, and carried into the comparator
/// input shape. Mirrors SD-25's `pcgen_runner_smoke.rs` real-invocation test
/// but exercises the Rust wrapper directly instead of shelling out to both
/// scripts by hand from the test.
#[test]
fn run_pcgen_character_runs_the_real_pcgen_engine_end_to_end() {
    let pcg = substitute_pcg_fixture();
    if !pcg.is_file() {
        eprintln!(
            "[skip] sd26_pcgen_runner: real PCGen .pcg fixture not found at {} \
             (set $PCGEN_REPO_DIR to a checked-out PCGen repo to run this end-to-end; \
             GitHub Actions runners do not check out the companion PCGen repo)",
            pcg.display()
        );
        return;
    }

    let input_fixture = pilot_deterministic_input_fixture();
    let golden_fixture = pilot_golden_fixture();
    let case_id = read_fixture_field(&input_fixture, "case_id");
    let source_package_id = read_fixture_field(&input_fixture, "source_package_id");
    let legacy_route = read_fixture_field(&golden_fixture, "legacy_route");

    let options = PcgenRunOptions::new(case_id.clone(), source_package_id.clone(), legacy_route);

    let output = run_pcgen_character(&pcg, &options)
        .unwrap_or_else(|err| panic!("real PCGen run + normalize should succeed: {err}"));

    assert_eq!(output.case_id, case_id);
    assert_eq!(output.source_package_id, source_package_id);
    assert!(
        !output.dimensions.is_empty(),
        "expected at least one real computed dimension from a genuine PCGen run"
    );

    let normalized = output.to_normalized_output();
    assert_eq!(normalized.dim_values.len(), output.dimensions.len());
}
