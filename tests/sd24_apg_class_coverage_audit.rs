//! SD-24 Epic 4 criterion 4.2 — Per-class audit: APG classes (Alchemist,
//! Cavalier, Inquisitor, Oracle, Summoner, Witch).
//!
//! This is the standing regression test behind
//! `docs/release/SD-24-beta-readiness-and-multiclass/artifacts/epic_4/per-class-coverage-matrix.md`'s
//! APG section and the six per-class
//! `docs/release/SD-24-beta-readiness-and-multiclass/artifacts/epic_4/apg_<class>_coverage.md`
//! receipts: it makes the audit's numeric claims (chassis rows wired,
//! named-feature count, live pilot-compute integration) executable and
//! checked on every run, rather than a one-off hand count that could
//! silently drift from the code.
//!
//! RED -> GREEN evidence (recorded in this cycle's receipt): before
//! `rules_tables::apg::mod::{ApgClassId::ALL, ApgClassCoverage, class_coverage,
//! coverage_report}` existed, this file did not compile (RED — no such
//! items). Adding that small, real (non-fabricated: every field is computed
//! from `class_table()` output, a `MAX_SUPPORTED_LEVEL` const, or a
//! documented corpus count) coverage-report API turned it GREEN.

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState,
};
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::rules_tables::apg::{ApgClassId, class_coverage, coverage_report};

/// Every real APG class's chassis table is fully wired for its own
/// `MAXLEVEL:20` ceiling (SD-22 Epic 3's already-landed, LST-verified
/// BAB/save cut) — the audit's first, structural finding.
#[test]
fn all_six_apg_classes_have_full_chassis_row_coverage() {
    let report = coverage_report();
    assert_eq!(report.len(), 6, "all six real APG classes should have a coverage row");
    for row in report {
        assert_eq!(
            row.chassis_rows_wired, row.chassis_rows_expected,
            "{:?}: chassis rows wired ({}) should equal chassis rows expected ({}) -- \
             SD-22 Epic 3 already ground the full BAB/save chassis for every APG class",
            row.class_id, row.chassis_rows_wired, row.chassis_rows_expected
        );
        assert_eq!(
            row.chassis_rows_expected, 20,
            "{:?}: every real APG class's MAXLEVEL is 20 per apg_classes.lst",
            row.class_id
        );
    }
}

/// The audit's second finding: zero named/narrative class features (Bombs,
/// Discoveries, Hexes, Judgments, Mystery Revelations, Eidolon, Challenge,
/// ...) are wired for any APG class yet -- this is a real gap, not a
/// placeholder value, and this test is a canary: if a future cycle starts
/// wiring named features, it must consciously update this assertion (and
/// the coverage-matrix artifact) rather than silently leaving the audit
/// stale.
#[test]
fn zero_named_class_features_are_wired_for_any_apg_class_yet() {
    for row in coverage_report() {
        assert_eq!(
            row.named_features_wired, 0,
            "{:?}: named_features_wired should be 0 (documented SD-24 Epic 4 finding); \
             if this now fails, a real feature landed -- update this canary and the \
             per-class coverage artifact together",
            row.class_id
        );
        assert!(
            row.named_features_expected > 0,
            "{:?}: named_features_expected should be a real positive corpus count, not 0",
            row.class_id
        );
    }
}

/// The audit's third finding, proven empirically rather than by inspection
/// alone: `pilot_compute::compute_pilot_base_chassis` -- the function the
/// live character-hub pilot flow actually calls -- does not recognize any
/// APG class. Driving a real, minimal `CharacterInput` for each of the six
/// classes through it must produce the honest, already-established
/// claim-blocking `class_chassis.unsupported` diagnostic (the same
/// diagnostic every non-Fighter/Wizard CRB class also trips today, per
/// `tests/sd20_contract_pilot_receipt.rs` and `tests/sd21_wizard_chassis_computes.rs`),
/// never a fabricated base_attack_bonus/base_saves value. This is the
/// "no-stub" doctrine's own worked example: an honest gap, not silent
/// fabrication.
#[test]
fn apg_classes_trip_the_honest_class_chassis_unsupported_diagnostic() {
    for class_id in ApgClassId::ALL {
        let input = minimal_input_for(*class_id);
        let computation = compute_pilot_base_chassis(&input);

        assert_eq!(
            computation.base_attack_bonus, 0,
            "{:?}: unsupported chassis must report zero, never a fabricated number",
            class_id
        );

        let unsupported = computation
            .diagnostics
            .iter()
            .find(|d| d.id == "class_chassis.unsupported")
            .unwrap_or_else(|| {
                panic!(
                    "{:?}: expected a class_chassis.unsupported diagnostic -- if this now \
                     fails, pilot_compute.rs has started recognizing this APG class and the \
                     coverage row's pilot_compute_integrated field (currently false) is stale",
                    class_id
                )
            });
        assert!(
            unsupported.claim_blocking,
            "{:?}: class_chassis.unsupported must remain claim_blocking: true",
            class_id
        );

        let row = class_coverage(*class_id);
        assert!(
            !row.pilot_compute_integrated,
            "{:?}: coverage row's pilot_compute_integrated should be false today",
            class_id
        );
    }
}

fn minimal_input_for(class_id: ApgClassId) -> CharacterInput {
    CharacterInput {
        case_id: Some(format!("sd24_apg_class_coverage_audit.{}", class_id.name())),
        source_package_id: "pf1.advanced_players_guide".to_string(),
        chosen: ChosenCharacterState {
            race_id: "race:human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: format!("class:{}", class_id.name()),
                level: 1,
            }],
            ability_scores: AbilityScores {
                strength: 10,
                dexterity: 10,
                constitution: 10,
                intelligence: 10,
                wisdom: 10,
                charisma: 10,
            },
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            spells_selected: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}
