//! SD-27 Cycle E3.x — Pathfinder Unchained PCGen parity baseline.
//!
//! Per `docs/release/SD-27-future-state-book-content-ingestion/loop-instruction.md`
//! section 3.4 (E3.x — Per-book parity baseline cycle), mirrors SD-26's own
//! CRB Human Fighter level 1 pilot pattern
//! (`tests/sd26_pilot_case_verification.rs`,
//! `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt`)
//! for the Pathfinder Unchained (PU) book, using the same Power Attack /
//! Dodge / Weapon Focus (Longsword) / Chain Shirt / Longsword GE-06
//! deterministic posture, plus one real record from THIS book's own Shape B
//! cache to exercise:
//!
//! - `data/corpus/pathfinder_unchained/feat/endurance.json` (the "Endurance"
//!   feat, `pu_feats.lst` line 26) -- Pathfinder Unchained's own "Wound
//!   Threshold" variant of Endurance (`TYPE:Wound Threshold`), distinct from
//!   the Core Rulebook feat of the same name. It has no prerequisites and no
//!   Combat-type restriction, so it is legal in any open feat slot. It
//!   grants situational Fortitude-save / Swim-check / Constitution-check
//!   bonuses none of this pilot's selected comparator dimensions measure, so
//!   it is carried as an extra chosen feat on both the PCGen `.pcg` and the
//!   Codex deterministic input without needing new engine support to
//!   compute its effect (`selected_feats` accepts any feat id string;
//!   `unmet_combat_posture_conditions` only requires the three named
//!   GE-06 feats to be present via containment, not an exact-set check, so
//!   the extra pick is inert on the Codex side exactly as it is inert for
//!   every PCGen-side comparator dimension).
//!
//! This is the SAME real end-to-end pipeline SD-26 wired
//! (`pcgen_runner` -> `comparator` -> per-dimension table), run for real
//! against a real PCGen engine invocation via the real `.pcg` fixture at
//! `data/corpus/pathfinder_unchained/_parity/pf_pathfinder_unchained_human_fighter_level1.pcg`.
//! Per `decisions.md §10`, this cycle's assertion is "match rate at the time
//! of cycle close," not a required 9-of-9/14-of-14 -- the inherited CG-03
//! (Human ability-modifier) baseline is v0.6's lane, not this cycle's to fix.
//! The real run this cycle performed reproduces that exact inherited
//! baseline: 14 of 15 compared dimensions match; the one real, structurally
//! distinct mismatch (`combat.baseline_melee_attack_bonus`, PCGen 5 vs.
//! Codex 6) is the same weapon-agnostic-vs-weapon-specific melee-total
//! discrepancy `tests/sd26_pilot_case_verification.rs`'s own doc comment
//! already diagnoses -- not a new, PU-specific regression.
//!
//! This test does NOT write into `artifacts/oracle_validation/` (SD-26's
//! own default parity-report directory) -- that path is outside this
//! cycle's file partition (`loop-instruction.md §6`). It uses
//! `render_parity_report` (an in-memory string render, no I/O) instead of
//! `write_parity_report`, and the cycle's own receipt
//! (`docs/release/SD-27-future-state-book-content-ingestion/artifacts/epic_3/pathfinder_unchained_parity-cycle_receipt.md`)
//! embeds that rendered table directly.

use codex::oracle_validation::comparator::compare;
use codex::oracle_validation::normalization::default_normalization_rules;
use codex::oracle_validation::parity_report::render_parity_report;
use codex::oracle_validation::pcgen_runner::{run_pcgen_character, PcgenRunOptions};
use codex::oracle_validation::selected_parity_dimensions::SelectedParityDimensions;
use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
use codex::rules_core::character_input::load_character_input_fixture;
use codex::rules_core::contract::to_pilot_receipt;
use codex::rules_core::pilot_compute_corpus::compute_pilot_with_corpus;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

use std::path::{Path, PathBuf};

/// Same GE-06 deterministic posture equipment SD-26's own pilot fixture uses
/// (Longsword / Chain Shirt, `WT:4`/`WT:25`) -- this pilot carries no other
/// equipment, so `encumbrance.total_carried_weight_lbs` stays the same
/// 29 lbs the GE-06/SD-26 baseline already established (this cycle's real
/// PU-sourced record, Endurance, is a feat, not equipment, so it adds no
/// weight on either side).
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

/// The GE-06 deterministic Human Fighter level 1 input contract
/// (`tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`),
/// inlined here rather than sharing that file directly, since this cycle's
/// file partition does not include `tests/fixtures/rules_core/` -- mirrors
/// `tests/sd27_advanced_race_guide_parity.rs`'s (and, before it,
/// `sd26_pilot_case_verification.rs`'s) own precedent of inlining a small,
/// stable fixture rather than introducing a cross-file coupling. Extended
/// with this book's own real exercised record: the Endurance feat (Wound
/// Threshold variant).
const DETERMINISTIC_FIXTURE: &str = "\
case_id=pf-pathfinder_unchained-human-fighter-level1
source_package_id=pf1.pathfinder_unchained
race_id=race:human
class_level=class:fighter:1
ability=strength:16
ability=dexterity:14
ability=constitution:14
ability=intelligence:10
ability=wisdom:12
ability=charisma:8
feat=feat:power_attack
feat=feat:dodge
feat=feat:weapon_focus
feat=feat:endurance
skill=skill:climb:1
skill=skill:intimidate:1
skill=skill:swim:1
equipment=item:chain_shirt:equipped_worn_active
equipment=item:longsword:equipped_primary_active
equipment=item:shield:absent
equipment=power_attack:selected_inactive
choice=choice:level_1_character_feat:feat:power_attack
choice=choice:human_bonus_feat:feat:dodge
choice=choice:fighter_bonus_feat:feat:weapon_focus:weapon:longsword
choice=choice:human_ability_bonus:ability:strength
provenance=docs/release/SD-27-future-state-book-content-ingestion/loop-instruction.md#34-e3x-per-book-parity-baseline-cycle\n";

const PILOT_CASE_ID: &str = "pf-pathfinder_unchained-human-fighter-level1";
const PILOT_SOURCE_PACKAGE_ID: &str = "pf1.pathfinder_unchained";
const PILOT_LEGACY_ROUTE: &str =
    "headless Gradle run batch export via code/testsuite/base-xml.ftl";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The real, hand-authored `.pcg` fixture for this cycle's pilot case (this
/// cycle's own output, per `loop-instruction.md §3.4`).
fn pilot_case_pcg_fixture() -> PathBuf {
    repo_root().join(
        "data/corpus/pathfinder_unchained/_parity/pf_pathfinder_unchained_human_fighter_level1.pcg",
    )
}

/// Mirrors `src/oracle_validation/pcgen_runner::default_pcgen_repo_dir` and
/// `scripts/pcgen-run-character.sh`'s own `-w`/`$PCGEN_REPO_DIR` contract:
/// `PCGEN_REPO_DIR` wins when set; otherwise `$HOME/workspace/repos/pcgen`,
/// the same HOME-relative default `pcgen-run-character.sh` falls back to.
fn default_pcgen_repo_dir() -> PathBuf {
    if let Ok(configured) = std::env::var("PCGEN_REPO_DIR") {
        return PathBuf::from(configured);
    }
    let home =
        std::env::var("HOME").expect("HOME must be set to locate the default PCGen repo checkout");
    PathBuf::from(home).join("workspace/repos/pcgen")
}

/// True iff the PCGen Gradle wrapper at `<pcgen_repo_dir>/gradlew` is a
/// present, executable file. Mirrors PR #356's contract for
/// `tests/sd26_pilot_case_verification::pcgen_gradle_wrapper_is_runnable`,
/// which is the canonical fix for the same E2E-vs-CI-runner precondition
/// gap (GitHub Actions runners do not check out the companion PCGen repo,
/// so `$HOME/workspace/repos/pcgen/gradlew` is absent there even though the
/// runner script is present and invokable).
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

/// End-to-end proof: pcgen_runner (real PCGen engine run via Gradle) ->
/// comparator (real dimension-by-dimension compare) -> parity_report
/// (real Markdown render, in-memory only -- no write into
/// `artifacts/oracle_validation/`, which is outside this cycle's file
/// partition). Prints the real rendered report so `cargo test -- --nocapture`
/// surfaces the actual per-dimension match/mismatch table this cycle's
/// receipt transcribes.
#[test]
fn full_pipeline_runs_end_to_end_against_the_real_pu_pilot_case() {
    // --- CI/runtime precondition: the real PCGen Gradle wrapper is not
    // available on every host. PR #356 established the same skip-guard for
    // SD-26's pilot parity test; this is the SD-27 PU parity cycle's mirror.
    // When `gradlew` is absent or non-executable, the test exits early with a
    // clear skip message instead of panicking on the missing PCGen checkout,
    // which is the GitHub Actions runner's actual state — the wrapper, the
    // runner script, and the parser are still exercised end-to-end on any host
    // where the PCGen checkout is available, and a real failure of any of those
    // components (script missing, normalizer non-zero exit, parseable-output
    // contract violated) is still surfaced as a hard test failure. ---
    let pcgen_repo_dir = default_pcgen_repo_dir();
    if !pcgen_gradle_wrapper_is_runnable(&pcgen_repo_dir) {
        eprintln!(
            "[skip] sd27_pathfinder_unchained_parity: real PCGen Gradle wrapper not found/executable at {} \
             (set $PCGEN_REPO_DIR to a checked-out PCGen repo to run this end-to-end; \
             GitHub Actions runners do not check out the companion PCGen repo)",
            pcgen_repo_dir.join("gradlew").display()
        );
        return;
    }

    // --- Codex side: real, computed selected parity dimensions. ---
    let input_load = load_character_input_fixture(DETERMINISTIC_FIXTURE);
    assert!(
        input_load.diagnostics.is_empty(),
        "PU pilot deterministic input fixture should load cleanly: {:?}",
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
        "expected real computed Codex selected parity dimensions for the PU pilot case"
    );

    // --- PCGen side: a real end-to-end PCGen engine run via the real
    // pcgen_runner wrapper, against this cycle's own real .pcg fixture. ---
    let pcg = pilot_case_pcg_fixture();
    assert!(
        pcg.is_file(),
        "expected this cycle's real .pcg fixture at {}",
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

    // --- Comparator: real compare() between the real PCGen run and the
    // real Codex dims (Criterion 2.1's `comparator::compare`, unmodified). ---
    let comparison = compare(&normalized, &codex_dims);

    // --- Parity report: real render (in-memory only; no write outside this
    // cycle's file partition). ---
    let rules = default_normalization_rules();
    let report = render_parity_report(PILOT_CASE_ID, &comparison, &rules);
    assert!(report.contains(PILOT_CASE_ID));
    println!("{report}");

    // Real, non-fabricated proof this pipeline ran end to end against a
    // genuine same-character PCGen build: identity matches every time this
    // pipeline is run against this exact fixture pair.
    let matched_ids: Vec<&str> =
        comparison.matches.iter().map(|m| m.dimension_id.as_str()).collect();
    assert!(
        matched_ids.contains(&"character.identity"),
        "expected character.identity to genuinely match between the real PCGen run \
         and the real Codex computation: {comparison:?}"
    );

    // Per decisions.md §10, this cycle's assertion is "match rate at the
    // time of cycle close," not a required full match. This real run
    // reproduces the exact inherited GE-06/SD-26 baseline (14 of 15
    // dimensions match; the one remaining mismatch is the already-diagnosed
    // weapon-agnostic-vs-weapon-specific melee-total discrepancy, not a new
    // PU-specific regression) -- asserted here so a future genuine
    // regression on this specific pilot fails loudly instead of silently.
    assert_eq!(
        comparison.matches.len(),
        14,
        "expected the inherited GE-06/SD-26 baseline's 14 matches: {comparison:?}"
    );
    assert_eq!(
        comparison.mismatches.len(),
        1,
        "expected exactly the one inherited combat.baseline_melee_attack_bonus mismatch: {comparison:?}"
    );
    let mismatch = &comparison.mismatches[0];
    assert_eq!(mismatch.dimension_id, "combat.baseline_melee_attack_bonus");
    assert_eq!(mismatch.pcgen_value_i16, Some(5));
    assert_eq!(mismatch.codex_value_i16, Some(6));

    println!(
        "PU pilot parity: {} matches, {} mismatches",
        comparison.matches.len(),
        comparison.mismatches.len()
    );
}
