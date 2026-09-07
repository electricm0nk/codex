//! v0.6 alpha swarm: real end-to-end PCGen parity proof for the Wizard pilot
//! case, mirroring `sd26_pilot_case_verification.rs`'s Fighter proof.
//!
//! Backend built and PCGen-verified (BUILD SUCCESSFUL twice, real headless
//! Gradle pipeline) a genuine, same-character `.pcg` fixture for the exact
//! `compose_character_input` Wizard-1 fixed loadout (risks-and-open-
//! questions.md item 11, previously scoped-not-attempted): Human, STR16(+2
//! human bonus)/DEX14/CON14/INT10/WIS12/CHA8, Power Attack/Dodge/Weapon
//! Focus(Longsword), Climb/Intimidate/Swim rank 1 (correctly marked
//! `CLASSSKILL:N` -- confirmed by this swarm's own class-skill-modifier fix
//! that these are not real Wizard class skills), Longsword+Chain Shirt, an
//! Evoker subclass with `PROHIBITED:Necromancy,Transmutation`, and the
//! seeded "Light" cantrip as both Known and Prepared. QA (this file) wired
//! the Codex-side `CharacterInput` fixture
//! (`pf1_human_wizard_level1_ge06_deterministic_input.txt`) to match it
//! field-for-field and ran it through the real Oracle-Harness Comparator
//! pipeline -- `pcgen_runner` (real PCGen engine run + normalize) ->
//! `comparator` (real dimension-by-dimension compare) -> `parity_report`
//! (real Markdown report render + write) -- the same wiring
//! `sd26_pilot_case_verification.rs` proved for Fighter.
//!
//! No golden-case-fixture claim-tier ceremony here (that machinery is
//! SD-26-Criterion-2.5-specific bookkeeping, not a general requirement) --
//! this file proves the pipeline and records the real, independently-run
//! comparison result, whatever it turns out to be, rather than assuming
//! backend's own manual normalizer-script tally.

use codex::oracle_validation::comparator::{compare, MismatchReason};
use codex::oracle_validation::selected_parity_dimensions::SelectedParityDimensions;
use codex::oracle_validation::parity_report::{default_parity_report_dir, write_parity_report};
use codex::oracle_validation::normalization::default_normalization_rules;
use codex::oracle_validation::pcgen_runner::{run_pcgen_character, PcgenRunOptions};
use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
use codex::rules_core::character_input::load_character_input_fixture;
use codex::rules_core::contract::to_pilot_receipt;
use codex::rules_core::pilot_compute::HeadlessReceiptStatus;
use codex::rules_core::pilot_compute_corpus::compute_pilot_with_corpus;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

use std::path::{Path, PathBuf};

/// Same equipment shape as `sd26_pilot_case_verification.rs`'s
/// `FIGHTER_GEAR_FIXTURE_TEXT` (identical Longsword/Chain Shirt records --
/// the Wizard pilot case uses the exact same gear) -- duplicated rather
/// than shared, matching that file's own stated reasoning (no existing
/// cross-file coupling for a small, stable fixture).
const WIZARD_GEAR_FIXTURE_TEXT: &str = "\
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Resizable.Melee.Martial.OneHanded.Slashing.Sword.BladeHeavy.Weapon Group Blades Heavy\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\tWIELD:OneHanded\tSIZE:M
Chain Shirt\tKEY:Chain Shirt (Base)\tTYPE:Armor.Light\tCOST:100\tWT:25\tACCHECK:-2\tMAXDEX:4\tSPELLFAILURE:20\tBONUS:COMBAT|AC|4|TYPE=Armor|PREVAREQ:DisableArmorBonus,0
Masterwork (Weapon)\tKEY:Special Quality ~ Masterwork ~ Weapon\tTYPE:MasterworkQuality.Weapon\tCOST:0\tBONUS:WEAPON|TOHIT|1|TYPE=Enhancement
";

fn corpus_with_wizard_gear() -> SourcePackageContent<'static> {
    let result = parse_equipment_entries("cr_equip_arms_armor.lst", WIZARD_GEAR_FIXTURE_TEXT);
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

const DETERMINISTIC_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_wizard_level1_ge06_deterministic_input.txt"
);

const PILOT_CASE_ID: &str = "pf1-crb-human-wizard-level1";
const PILOT_SOURCE_PACKAGE_ID: &str = "pf1.core_rulebook";
const PILOT_LEGACY_ROUTE: &str =
    "headless Gradle run batch export via code/testsuite/base-xml.ftl";

/// Repo-relative path to backend's real, PCGen-verified Wizard pilot-case
/// `.pcg` fixture (BUILD SUCCESSFUL against the real headless Gradle
/// pipeline, twice), vendored into the GE-05 build's own artifact folder —
/// the same convention `sd26_pilot_case_verification.rs`'s Fighter fixture
/// now uses.
///
/// This was previously a hardcoded absolute path into another machine's
/// `programs/codex/requirements/` *planning* tree, with no environment
/// override, which made this suite unrunnable anywhere but that one box.
/// Build artifacts belong in the build's own artifact folder rather than
/// behind a reference to an external source — see
/// `tests/ge05_vendored_pcg_fixtures.rs` for the provenance and integrity
/// proof.
const PILOT_PCG_FIXTURE_PATH: &str =
    "docs/release/GE-05-oracle-validation-and-parity-harness/artifacts/\
     pf1-crb-human-wizard-level1-v06-alpha-swarm.pcg";

/// sha256 of the vendored fixture as committed. Pinned so a silently
/// swapped or regenerated `.pcg` fails loudly here instead of quietly
/// shifting the parity numbers this suite publishes as evidence. Kept in
/// sync with `tests/ge05_vendored_pcg_fixtures.rs`.
const PILOT_PCG_FIXTURE_SHA256: &str =
    "e2bcdae8cfccecbf871f7c587d4af577a86b5466747e41b329d0e36ee777330b";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Resolution follows `authoring_workbench::resolve_package_path`'s contract —
/// repo-relative paths anchor at the codex repo root. That helper lives in
/// the separate `codex-desktop` crate and is not reachable from a `codex`
/// root-crate integration test, so this uses the identical anchor its
/// fallback uses (`CARGO_MANIFEST_DIR`), which is also the in-crate pattern
/// `sd27_advanced_race_guide_parity.rs` already established for its own
/// vendored `.pcg` fixture.
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

fn pilot_case_pcg_fixture() -> PathBuf {
    repo_root().join(PILOT_PCG_FIXTURE_PATH)
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

/// The Codex-side fixture reaches `Computed`, not `Blocked` -- proving the
/// Wizard fixture genuinely satisfies all three exactness gates (combat
/// posture, skill posture, and the Wizard spellbook posture) before it is
/// ever compared against PCGen. A `Blocked` fixture would make any
/// downstream comparison meaningless.
#[test]
fn wizard_pilot_case_input_reaches_computed() {
    let input_load = load_character_input_fixture(DETERMINISTIC_FIXTURE);
    assert!(
        input_load.diagnostics.is_empty(),
        "wizard pilot deterministic input fixture should load cleanly: {:?}",
        input_load.diagnostics
    );
    let input = input_load
        .character_input
        .expect("valid deterministic input fixture should produce a CharacterInput record");
    let receipt = codex::rules_core::pilot_compute::build_pilot_headless_receipt(&input);
    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Computed,
        "the real fixed Wizard loadout must reach Computed before any PCGen comparison is \
         meaningful: {:?}",
        receipt.computation.diagnostics
    );
}

/// The full pipeline proof: pcgen_runner -> comparator -> parity_report, run
/// for real against the Wizard pilot case's real Codex-computed dimensions
/// and a real PCGen engine invocation against backend's verified `.pcg`.
#[test]
fn full_pipeline_runs_end_to_end_for_the_wizard_pilot_case() {
    // --- CI/runtime precondition: the real PCGen Gradle wrapper is not
    // available on every host. PR #356 established the same skip-guard for
    // SD-26's pilot parity test; this is the v0.6 Wizard pilot case's mirror.
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
            "[skip] v06_wizard_pilot_case_verification: real PCGen Gradle wrapper not found/executable at {} \
             (set $PCGEN_REPO_DIR to a checked-out PCGen repo to run this end-to-end; \
             GitHub Actions runners do not check out the companion PCGen repo)",
            pcgen_repo_dir.join("gradlew").display()
        );
        return;
    }

    let _pcg = pilot_case_pcg_fixture();
    // --- Codex side: real, computed selected parity dimensions, via the
    // corpus-aware PilotReceipt (from_pilot_receipt) so durability/encumbrance
    // are genuinely compared against PCGen rather than left MissingFromCodex. ---
    let input_load = load_character_input_fixture(DETERMINISTIC_FIXTURE);
    assert!(
        input_load.diagnostics.is_empty(),
        "wizard pilot deterministic input fixture should load cleanly: {:?}",
        input_load.diagnostics
    );
    let input = input_load
        .character_input
        .expect("valid deterministic input fixture should produce a CharacterInput record");
    let corpus = corpus_with_wizard_gear();
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
        "expected real computed Codex selected parity dimensions for the wizard pilot case"
    );

    // --- PCGen side: a real end-to-end PCGen engine run against backend's
    // verified `.pcg`. ---
    let pcg = pilot_case_pcg_fixture();
    assert!(
        pcg.is_file(),
        "expected the real PCGen-verified wizard pilot .pcg fixture at {}",
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
        "parity report should name the wizard pilot case id"
    );

    // --- The real, genuine finding, run once and read off, not guessed:
    // 13 of 14 dimensions match. The Wizard fixture's Climb/Intimidate/Swim
    // are NOT real Wizard class skills (this swarm's own class-skill-
    // modifier fix, 93a0636d), so they correctly compare at the un-bonused
    // value (3/0/3, not the Fighter case's class-skill-boosted 6/3/6) --
    // both PCGen and Codex agree on that un-bonused value, independently
    // confirming the fix from the PCGen side, not just Codex's own tests. ---
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
    // The real matched values, not just their presence -- independently
    // confirming the numbers, not just that a comparison happened.
    for (dimension_id, expected_value) in [
        ("defense.baseline_armor_class", 17i16),
        ("defense.total_save.fortitude", 2i16),
        ("defense.total_save.reflex", 2i16),
        ("defense.total_save.will", 3i16),
        ("skill.selected_modifier.climb", 3i16),
        ("skill.selected_modifier.intimidate", 0i16),
        ("skill.selected_modifier.swim", 3i16),
        ("durability.max_hp", 8i16),
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
        "expected exactly 14 genuinely matching dimensions: {:?}",
        comparison
    );

    // The one remaining genuine mismatch: combat.baseline_melee_attack_bonus.
    //
    // **This oracle shares a blind spot with the thing it validates, and
    // that is why the mismatch must NOT be read as "Codex is wrong".**
    // Updated 2026-07-29 (risks item #89, tasks #80+#86): Codex moved from
    // 5 to 1 when the nonproficiency penalty was finally applied. PCGen's
    // compared export field stayed at 4. The gap is now -3 rather than
    // +1, and it is the sum of TWO independent defects in that export
    // field, not one:
    //
    //  1. It omits Weapon Focus (Longsword)'s +1, which the character
    //     genuinely has (this is the long-documented half, the same shape
    //     as the Fighter pilot case's discrepancy).
    //  2. It omits PF1's own -4 nonproficiency penalty
    //     (`WEAPONNONPROFPENALTY` in
    //     `system/gameModes/Pathfinder/miscinfo.lst:193`), even though a
    //     Wizard has no Longsword proficiency at all -- the Wizard's real
    //     grant (`cr_abilities_class.lst`, `KEY:Weapon and Armor
    //     Proficiency ~ Wizard`) is exactly `AUTO:WEAPONPROF|Club|Dagger|
    //     Crossbow (Heavy)|Crossbow (Light)|Quarterstaff`.
    //
    // PCGen's 4 is therefore just BAB(0) + STR(+4) with both the feat and
    // the penalty dropped. Before this fix the two omissions partly
    // cancelled (Codex 5 vs PCGen 4 looked like a tidy +1 Weapon Focus
    // delta), which is exactly how the second defect stayed hidden: the
    // oracle and the engine agreed while BOTH were wrong. Correcting Codex
    // uncovered it rather than caused it.
    //
    // The expected values below are updated deliberately and with that
    // reasoning recorded, rather than leaving a correct engine showing red
    // against a defective export.
    let mismatch = comparison
        .mismatches
        .iter()
        .find(|m| m.dimension_id == "combat.baseline_melee_attack_bonus")
        .unwrap_or_else(|| panic!("expected the known combat.baseline_melee_attack_bonus mismatch: {comparison:?}"));
    assert_eq!(mismatch.reason, MismatchReason::ValueMismatch);
    assert_eq!(mismatch.pcgen_value_i16, Some(4));
    assert_eq!(mismatch.codex_value_i16, Some(1));
    assert_eq!(
        comparison.mismatches.len(),
        1,
        "expected exactly the one known baseline-attack-bonus mismatch and no others: {:?}",
        comparison
    );
    assert!(!comparison.all_matched());
}

/// v0.6 alpha swarm (armored-Wizard ASF%/ACP task): confirms that PF1's two
/// unproficiency-adjacent armor mechanics -- arcane spell failure chance and
/// armor check penalty -- are genuinely, class-agnostically grounded for the
/// exact Wizard-in-a-Chain-Shirt scenario this task was dispatched to fix,
/// not just for the Fighter case existing coverage already proved.
///
/// **Finding, not a new mechanism**: `equipment_effects::compute_equipment_effects`
/// (wired into `to_pilot_receipt` unconditionally, see `contract.rs`'s own
/// `equipment_effects` field doc comment) already sums every equipped item's
/// real `SPELLFAILURE:`/`ACCHECK:` corpus tokens into
/// `EquipmentEffects.spell_failure_chance`/`armor_check_penalty_total`,
/// completely independent of the wearer's class or armor proficiency --
/// correct per RAW, since a Chain Shirt's 20% arcane spell failure chance
/// and -2 armor check penalty are properties of the armor itself, not of who
/// wears it (`tests/sd20_contract_equipment_wiring.rs` already proved this
/// for a Fighter; this is the same real, live receipt field, now proven for
/// the class that actually cares about ASF%). This closes the "an armored
/// Wizard's spellcasting is unaffected" concern this task was dispatched
/// with: it already IS affected, on the real output contract, with the
/// exact corpus-verified magnitudes
/// (`cr_equip_arms_armor.lst:40 Chain Shirt -> ACCHECK:-2, SPELLFAILURE:20`).
/// Armor PROFICIENCY itself (whether a class avoids some further RAW
/// nonproficiency penalty) remains genuinely unmodeled and out of scope --
/// see this task's own swarm-status entry for why building it fresh would
/// be speculative, unrequested scope creep rather than grounding an
/// existing gap.
#[test]
fn wizard_in_a_chain_shirt_gets_the_real_spell_failure_and_check_penalty_on_the_receipt() {
    let input_load = load_character_input_fixture(DETERMINISTIC_FIXTURE);
    assert!(
        input_load.diagnostics.is_empty(),
        "wizard pilot deterministic input fixture should load cleanly: {:?}",
        input_load.diagnostics
    );
    let input = input_load
        .character_input
        .expect("valid deterministic input fixture should produce a CharacterInput record");
    let corpus = corpus_with_wizard_gear();
    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let pilot_receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);

    assert_eq!(
        pilot_receipt.equipment_effects.spell_failure_chance,
        Some(20.0),
        "Chain Shirt's real SPELLFAILURE:20 corpus token must reach the receipt for a Wizard, \
         exactly as it already does for a Fighter: {:?}",
        pilot_receipt.equipment_effects
    );
    assert_eq!(
        pilot_receipt.equipment_effects.armor_check_penalty_total,
        -2,
        "Chain Shirt's real ACCHECK:-2 corpus token must reach the receipt for a Wizard: {:?}",
        pilot_receipt.equipment_effects
    );

    // The same -2 Chain Shirt armor-check penalty is also independently
    // applied inside `pilot_compute.rs`'s own deterministic Climb/Swim
    // computation (a second, narrower consumer, separate from the generic
    // receipt-level aggregate above) -- both must agree it is real and
    // nonzero for this exact posture.
    let climb = pilot_receipt
        .chassis
        .explanations
        .iter()
        .find(|e| e.id == "skill.selected_modifier.climb")
        .expect("Climb explanation must be present for the Wizard GE-06 posture");
    assert!(
        climb.detail.contains("Chain Shirt armor-check penalty"),
        "Climb's own explanation should cite the Chain Shirt armor-check penalty: {}",
        climb.detail
    );
}
