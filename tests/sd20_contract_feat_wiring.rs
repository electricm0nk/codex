//! SD-20 Epic 2-7 -> boundary-contract wiring project, Cycle 3
//! (`contract:feat_wiring`, per `adaptive-squishing-mccarthy.md`).
//!
//! Before this cycle, `PilotReceipt` never called Epic 3's real
//! `feat_prereqs::{evaluate_feat_prerequisites, compute_feat_effects}` at
//! all (`tests/sd20_tabletop_readiness_integration.rs`'s Finding 1) --
//! feat prerequisites/effects were reachable only via a direct call to the
//! engine, never through the boundary contract. This file proves two
//! things about the fix:
//!
//! (a) **Parity**: `to_pilot_receipt(...).feats` matches exactly what a
//!     direct call to `evaluate_feat_prerequisites`/`compute_feat_effects`
//!     produces for the same resolved `FeatKey`s -- the wiring composes
//!     the engine's real output, it does not reinterpret or duplicate it.
//!     Exercised with the Fighter tabletop fixture's real feats (Dodge,
//!     Weapon Focus, Power Attack -- all real CRB `FeatCategory::Combat`
//!     catalog records per `feat_data/combat.rs`), so the parity
//!     assertion is not vacuously true over an empty `Vec`.
//! (b) **Honest omission**: a character whose `selected_feats` contains an
//!     unrecognized/garbage feat-id string alongside real feat names
//!     produces NO `ResolvedFeat` for the garbage string -- it is silently
//!     skipped, not fabricated into some default category -- while still
//!     correctly resolving the character's other real feats. This mirrors
//!     the human_fighter_level_1_tabletop.json fixture's own
//!     `selected_feats` shape, which already carries both namespaced ids
//!     (`feat:dodge`, unrecognized by `feat_tables()`) and real catalog
//!     names (`Dodge`) side by side.
//!
//! Per `adaptive-squishing-mccarthy.md`'s "Feat resolution" design
//! decision: `CharacterInput.chosen.selected_feats: Vec<String>` has no
//! category field, so the wiring resolves each entry against
//! `rules_tables::crb::feats::feat_tables()` by matching
//! `entry.key == feat_id || entry.name == feat_id`. An unmatched id is
//! honestly skipped, not fabricated -- same discipline the feat catalog's
//! own generator already uses (see `feats.rs`'s module doc comment on the
//! 12 deliberately-excluded corpus records).
//!
//! Per the same design decision's "Not every epic output becomes a sheet
//! cell" clause: this cycle lands NO new `printed_sheet_cell_map` cells.
//! `PrerequisiteEvaluation`/`FeatEffects` don't reduce to a single sheet
//! number -- `receipt.feats` is reachable directly, not flattened into
//! cells.

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState,
};
use codex::rules_core::contract::{printed_sheet_cell_map, to_pilot_receipt};
use codex::rules_core::feat_prereqs::{
    compute_feat_effects, evaluate_feat_prerequisites, FeatKey,
};
use codex::rules_core::pilot_compute_corpus::compute_pilot_with_corpus;
use codex::rules_core::rules_tables::crb::feats::{feat_tables, FeatCategory};
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

fn empty_corpus(name: &str) -> SourcePackageContent<'static> {
    let source_ref = SourceRef {
        lst_file: name.to_string(),
        line: 1,
    };
    SourcePackageContent::empty(name.to_string(), source_ref)
}

/// A level-1 Fighter with the tabletop fixture's real feat set (Dodge,
/// Weapon Focus, Power Attack -- all real CRB `FeatCategory::Combat`
/// catalog records) PLUS a garbage/unrecognized feat-id string, mirroring
/// `human_fighter_level_1_tabletop.json`'s own `selected_feats` shape
/// (which mixes namespaced ids like `feat:dodge`, unrecognized by
/// `feat_tables()`, alongside real catalog names like `Dodge`).
fn fighter_with_real_and_garbage_feats() -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_contract_feat_wiring_fighter".to_string()),
        source_package_id: "sd20_contract_feat_wiring_fighter".to_string(),
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
            selected_feats: vec![
                "Dodge".to_string(),
                "Weapon Focus".to_string(),
                "Power Attack".to_string(),
                "feat:not_a_real_feat".to_string(),
            ],
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

/// Resolve `feat_id` against the real catalog exactly the way the wiring
/// brief specifies (`entry.key == feat_id || entry.name == feat_id`), for
/// use in this test's own independent parity computation.
fn resolve_via_catalog(feat_id: &str) -> Option<FeatCategory> {
    feat_tables()
        .iter()
        .find(|entry| entry.key == feat_id || entry.name == feat_id)
        .map(|entry| entry.category)
}

#[test]
fn pilot_receipt_feats_field_matches_direct_evaluate_and_compute_calls_for_real_feats() {
    let input = fighter_with_real_and_garbage_feats();
    let corpus = empty_corpus("sd20_contract_feat_wiring_fighter");

    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);

    // Sanity: exactly the 3 real feats resolved, matching the fixture's
    // real Fighter feat set -- so the parity assertion below is not
    // vacuously true over an empty Vec.
    assert_eq!(
        receipt.feats.len(),
        3,
        "expected exactly 3 resolved feats (Dodge, Weapon Focus, Power Attack); got {:?}",
        receipt
            .feats
            .iter()
            .map(|f| f.feat_id.clone())
            .collect::<Vec<_>>()
    );

    for expected_feat_id in ["Dodge", "Weapon Focus", "Power Attack"] {
        let category = resolve_via_catalog(expected_feat_id)
            .unwrap_or_else(|| panic!("{expected_feat_id} must be a real catalog feat"));
        let key = FeatKey {
            feat_id: expected_feat_id.to_string(),
            category,
        };
        let direct_evaluation = evaluate_feat_prerequisites(&key);
        let direct_effects = compute_feat_effects(&key);

        let resolved = receipt
            .feats
            .iter()
            .find(|f| f.feat_id == expected_feat_id)
            .unwrap_or_else(|| panic!("{expected_feat_id} must be present in receipt.feats"));

        assert_eq!(
            resolved.prerequisites, direct_evaluation,
            "receipt.feats[{expected_feat_id}].prerequisites must match a direct \
             evaluate_feat_prerequisites(&key) call exactly"
        );
        assert_eq!(
            resolved.effects, direct_effects,
            "receipt.feats[{expected_feat_id}].effects must match a direct \
             compute_feat_effects(&key) call exactly"
        );
    }

    // Real CRB Combat-category feats resolve eligible with real effects --
    // proves the parity check above isn't vacuous over an all-ineligible
    // set either.
    let dodge = receipt
        .feats
        .iter()
        .find(|f| f.feat_id == "Dodge")
        .unwrap();
    assert!(dodge.prerequisites.is_eligible);
    assert_eq!(
        dodge.effects.description.as_deref(),
        Some("Your training and reflexes allow you to react swiftly to avoid an opponent's attack.")
    );
}

#[test]
fn unrecognized_feat_id_is_honestly_skipped_not_fabricated() {
    let input = fighter_with_real_and_garbage_feats();
    let corpus = empty_corpus("sd20_contract_feat_wiring_fighter_garbage");

    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);

    assert!(
        !receipt
            .feats
            .iter()
            .any(|f| f.feat_id == "feat:not_a_real_feat"),
        "an unrecognized selected-feat string must produce NO ResolvedFeat entry at all -- \
         not a fabricated one under some default category"
    );

    // The character's other 3 real feats still resolve correctly despite
    // the garbage entry sitting alongside them in selected_feats.
    assert_eq!(receipt.feats.len(), 3);
    for expected_feat_id in ["Dodge", "Weapon Focus", "Power Attack"] {
        assert!(receipt.feats.iter().any(|f| f.feat_id == expected_feat_id));
    }
}

#[test]
fn feat_wiring_adds_no_new_printed_sheet_cells() {
    // Per adaptive-squishing-mccarthy.md's Cycle 3 scope: PrerequisiteEvaluation
    // / FeatEffects don't reduce to a single sheet number, so this cycle
    // lands zero new cells -- receipt.feats stays reachable directly.
    let input = fighter_with_real_and_garbage_feats();
    let corpus = empty_corpus("sd20_contract_feat_wiring_no_cells");

    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);
    let cells = printed_sheet_cell_map(&receipt);

    assert!(
        !cells.iter().any(|cell| cell.cell_id.starts_with("sheet.feat")),
        "Cycle 3 lands no sheet.feat* cells: {:?}",
        cells
            .iter()
            .filter(|cell| cell.cell_id.starts_with("sheet.feat"))
            .collect::<Vec<_>>()
    );
}
