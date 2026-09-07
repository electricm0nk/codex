//! SD-20 Epic 2-7 -> boundary-contract wiring project, Cycle 1
//! (`contract:skill_wiring`, per `adaptive-squishing-mccarthy.md`).
//!
//! Before this cycle, `PilotReceipt` composed only
//! `PilotBaseChassisComputation` (whose `selected_skill_modifiers` field
//! is `pilot_compute.rs::compute_selected_skill_modifiers`'s narrow,
//! single-posture Climb/Intimidate/Swim rank-1-only check) and never
//! called Epic 4's real `skill_allocation::allocate_skill_ranks`. This
//! file proves two things about the fix:
//!
//! (a) **Parity**: `to_pilot_receipt(...).skills` is exactly what a
//!     direct call to `allocate_skill_ranks(&input)` produces for the
//!     same input -- the wiring composes the engine's real output, it
//!     does not reinterpret or duplicate it.
//! (b) **A real improvement, not just a refactor**: a character with 2
//!     ranks in Climb deviates from the OLD chassis check's exact
//!     rank-1-only posture (`unmet_selected_skill_posture_conditions`),
//!     which used to trip claim-blocking `skill.selected_modifier.unsupported`
//!     and render `sheet.skill.climb` as `Blocked` -- a real,
//!     computable value hidden behind a diagnostic that has nothing to
//!     do with whether the value is actually knowable.
//!     `allocate_skill_ranks`'s own diagnostics are all `claim_blocking:
//!     false` (see `skill_allocation.rs`'s `SkillTotals::diagnostics` doc
//!     comment, confirmed by reading the module before writing this
//!     test), so under the new wiring this same character's
//!     `sheet.skill.climb` cell renders the real, correct `Number`
//!     instead.

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState, SkillAllocation,
};
use codex::rules_core::contract::{
    printed_sheet_cell_map, to_pilot_receipt, PrintedSheetCellValue,
};
use codex::rules_core::pilot_compute_corpus::compute_pilot_with_corpus;
use codex::rules_core::skill_allocation::allocate_skill_ranks;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

fn empty_corpus(name: &str) -> SourcePackageContent<'static> {
    let source_ref = SourceRef {
        lst_file: name.to_string(),
        line: 1,
    };
    SourcePackageContent::empty(name.to_string(), source_ref)
}

/// A level-1 Fighter with exactly the rank-1 Climb/Intimidate/Swim
/// posture `pilot_compute.rs::compute_selected_skill_modifiers` requires
/// -- deliberately reused so this test's parity assertion (a) is not
/// trivially true only because the chassis check also happens to be
/// unsupported for some unrelated reason.
fn fighter_with_rank_1_skills() -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_contract_skill_wiring".to_string()),
        source_package_id: "sd20_contract_skill_wiring".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:fighter".to_string(),
                level: 1,
            }],
            ability_scores: AbilityScores {
                strength: 16,
                dexterity: 14,
                constitution: 14,
                intelligence: 10,
                wisdom: 12,
                charisma: 8,
            },
            selected_feats: Vec::new(),
            skill_allocations: vec![
                SkillAllocation {
                    skill_id: "skill:climb".to_string(),
                    ranks: 1,
                },
                SkillAllocation {
                    skill_id: "skill:intimidate".to_string(),
                    ranks: 1,
                },
                SkillAllocation {
                    skill_id: "skill:swim".to_string(),
                    ranks: 1,
                },
            ],
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

#[test]
fn pilot_receipt_skills_field_matches_a_direct_allocate_skill_ranks_call() {
    let input = fighter_with_rank_1_skills();
    let corpus = empty_corpus("sd20_contract_skill_wiring_parity");

    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);

    let direct = allocate_skill_ranks(&input);

    assert_eq!(
        receipt.skills, direct,
        "PilotReceipt.skills must be exactly what a direct allocate_skill_ranks(&input) call \
         produces for the same input -- the wiring must compose the engine's real output, not \
         reinterpret or duplicate it"
    );

    // Sanity: this fixture's posture really does resolve real (non-empty)
    // totals for the three skills it allocates, so the parity assertion
    // above is not vacuously true over an empty map.
    assert!(direct.totals.contains_key("skill:climb"));
    assert!(direct.totals.contains_key("skill:intimidate"));
    assert!(direct.totals.contains_key("skill:swim"));
}

#[test]
fn skill_cells_are_never_blocked_by_the_old_chassis_single_posture_diagnostic() {
    let mut input = fighter_with_rank_1_skills();
    // 2 ranks in Climb: the OLD chassis check
    // (`compute_selected_skill_modifiers`'s `unmet_selected_skill_posture_conditions`)
    // requires *exactly* rank 1 in Climb/Intimidate/Swim and nothing else
    // -- 2 ranks trips its claim-blocking `skill.selected_modifier.unsupported`
    // diagnostic. Before this cycle, that diagnostic gated
    // `sheet.skill.climb`/`.intimidate`/`.swim` to `Blocked`. This is the
    // exact "old check only handled rank-1" deviation the task brief
    // names.
    input.chosen.skill_allocations = vec![
        SkillAllocation {
            skill_id: "skill:climb".to_string(),
            ranks: 2,
        },
        SkillAllocation {
            skill_id: "skill:intimidate".to_string(),
            ranks: 1,
        },
        SkillAllocation {
            skill_id: "skill:swim".to_string(),
            ranks: 1,
        },
    ];

    let corpus = empty_corpus("sd20_contract_skill_wiring_deviation");
    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);

    // Confirm the OLD chassis-level diagnostic really does still fire for
    // this posture (pilot_compute.rs is untouched trunk) -- this is what
    // proves the fix is a real improvement, not a test that happens to
    // avoid the old gate entirely.
    assert!(
        receipt
            .diagnostics
            .iter()
            .any(|d| d.id == "skill.selected_modifier.unsupported" && d.claim_blocking),
        "2 ranks in Climb must still trip the old chassis check's \
         skill.selected_modifier.unsupported diagnostic (unrelated trunk behavior, unchanged \
         by this cycle): {:?}",
        receipt.diagnostics
    );
    // And the chassis's own (now-unused-for-cells) field is still zeroed
    // by that diagnostic, confirming this is genuinely the old gate firing.
    assert_eq!(receipt.chassis.selected_skill_modifiers.climb, 0);

    // Epic 4's allocate_skill_ranks has no notion of that posture at all
    // and produces every diagnostic as claim_blocking: false -- confirmed
    // by reading skill_allocation.rs before writing this test.
    let direct = allocate_skill_ranks(&input);
    assert!(
        direct.diagnostics.iter().all(|d| !d.claim_blocking),
        "every allocate_skill_ranks diagnostic must be claim_blocking: false: {:?}",
        direct.diagnostics
    );
    let expected_climb_total = direct
        .totals
        .get("skill:climb")
        .expect("climb must resolve: it's a recognized, allocated, class skill")
        .total_modifier;
    // Concrete expected value, derived from the module's own cited,
    // documented formula (ranks + STR modifier + class-skill trained
    // bonus): 2 ranks + (+3 STR mod for a 16) + 3 trained bonus = 8.
    assert_eq!(
        expected_climb_total, 8,
        "2 ranks class-skill Climb at STR 16 should total 2 + 3 (STR mod) + 3 (trained) = 8"
    );

    let cells = printed_sheet_cell_map(&receipt);
    let climb_cell = cells
        .iter()
        .find(|cell| cell.cell_id == "sheet.skill.climb")
        .expect("sheet.skill.climb cell must exist");
    let intimidate_cell = cells
        .iter()
        .find(|cell| cell.cell_id == "sheet.skill.intimidate")
        .expect("sheet.skill.intimidate cell must exist");
    let swim_cell = cells
        .iter()
        .find(|cell| cell.cell_id == "sheet.skill.swim")
        .expect("sheet.skill.swim cell must exist");

    // The load-bearing assertion: despite the old diagnostic firing
    // claim_blocking: true above, sheet.skill.climb now renders the real
    // Number Epic 4 computed -- never Blocked. This is the concrete proof
    // the fix is a real improvement, not just a refactor.
    assert_eq!(
        climb_cell.value,
        PrintedSheetCellValue::Number(expected_climb_total as i16),
        "sheet.skill.climb must render the real allocate_skill_ranks total, not Blocked, even \
         though the old chassis single-posture diagnostic still fires for this input"
    );
    assert_ne!(
        climb_cell.value,
        PrintedSheetCellValue::Blocked,
        "sheet.skill.climb must never be Blocked by the old chassis single-posture diagnostic"
    );
    assert!(
        !matches!(intimidate_cell.value, PrintedSheetCellValue::Blocked),
        "sheet.skill.intimidate must also not be blocked by the old chassis diagnostic"
    );
    assert!(
        !matches!(swim_cell.value, PrintedSheetCellValue::Blocked),
        "sheet.skill.swim must also not be blocked by the old chassis diagnostic"
    );
}

#[test]
fn new_diplomacy_and_disable_device_cells_exist_and_source_from_skill_totals() {
    let mut input = fighter_with_rank_1_skills();
    input.chosen.skill_allocations.push(SkillAllocation {
        skill_id: "skill:diplomacy".to_string(),
        ranks: 1,
    });
    input.chosen.skill_allocations.push(SkillAllocation {
        skill_id: "skill:disable_device".to_string(),
        ranks: 1,
    });

    let corpus = empty_corpus("sd20_contract_skill_wiring_new_cells");
    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);
    let direct = allocate_skill_ranks(&input);

    let cells = printed_sheet_cell_map(&receipt);
    let diplomacy_cell = cells
        .iter()
        .find(|cell| cell.cell_id == "sheet.skill.diplomacy")
        .expect("sheet.skill.diplomacy cell must exist");
    let disable_device_cell = cells
        .iter()
        .find(|cell| cell.cell_id == "sheet.skill.disable_device")
        .expect("sheet.skill.disable_device cell must exist");

    let expected_diplomacy = direct
        .totals
        .get("skill:diplomacy")
        .expect("diplomacy must resolve (cross-class, allocated)")
        .total_modifier;
    let expected_disable_device = direct
        .totals
        .get("skill:disable_device")
        .expect("disable_device must resolve (cross-class + trained-only, but allocated)")
        .total_modifier;

    assert_eq!(
        diplomacy_cell.value,
        PrintedSheetCellValue::Number(expected_diplomacy as i16)
    );
    assert_eq!(
        disable_device_cell.value,
        PrintedSheetCellValue::Number(expected_disable_device as i16)
    );
}

#[test]
fn a_skill_never_allocated_at_all_renders_blocked_not_a_fabricated_zero() {
    // This fixture's base posture never allocates skill:disable_device at
    // all (not even at 0 ranks) -- confirmed by reading
    // allocate_skill_ranks: it only ever produces a totals/untrained_use
    // entry for a skill that appears in input.chosen.skill_allocations at
    // all, so a skill entirely absent from that list has no entry to
    // source a cell value from. The cell map must render Blocked (no
    // data), never a fabricated Number(0).
    let input = fighter_with_rank_1_skills();
    let direct = allocate_skill_ranks(&input);
    assert!(
        !direct.totals.contains_key("skill:disable_device"),
        "this fixture never allocates skill:disable_device; totals must have no entry for it"
    );
    assert!(
        !direct.untrained_use.contains_key("skill:disable_device"),
        "untrained_use must also have no entry: it is populated from the same per-allocation \
         loop as totals, so a never-allocated skill has no fallback there either"
    );

    let corpus = empty_corpus("sd20_contract_skill_wiring_absent");
    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);
    let cells = printed_sheet_cell_map(&receipt);
    let disable_device_cell = cells
        .iter()
        .find(|cell| cell.cell_id == "sheet.skill.disable_device")
        .expect("sheet.skill.disable_device cell must exist");

    assert_eq!(
        disable_device_cell.value,
        PrintedSheetCellValue::Blocked,
        "a skill with no entry at all in skill_allocations must render Blocked (honest absence \
         of data), never a fabricated Number(0)"
    );
}
