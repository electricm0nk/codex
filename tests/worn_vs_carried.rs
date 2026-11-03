//! Only `EquippedActive` items feed a number; both `EquippedActive` and `SelectedInactive` weigh
//! (`docs/release/SD-39-ui-print-corrections/decisions.md §9`). One test per engine consumer, each
//! on the real Core Rulebook records. The sheet a player reads is proved over the desktop's own
//! load in `apps/desktop/src-tauri/src/character_hub.rs`, same filter.

use std::path::Path;

use codex::rules_core::character_input::{
    load_character_input_fixture, ActiveState, CharacterInput, EquipmentSelection,
};
use codex::rules_core::contract::to_pilot_receipt;
use codex::rules_core::corpus_loader::{load_equipment_corpus, BookCorpusRoot};
use codex::rules_core::damage_total::resolve_weapon_damage_breakdown;
use codex::rules_core::encumbrance::compute_encumbrance;
use codex::rules_core::equipment_effects::compute_equipment_effects;
use codex::rules_core::pilot_compute_corpus::compute_pilot_with_corpus;
use codex::rules_core::size::SizeCategory;
use codex::rules_core::source_content::SourcePackageContent;

fn core_rulebook() -> SourcePackageContent<'static> {
    let roots = [BookCorpusRoot { book_id: "core_rulebook", dir: Path::new("data/corpus/core_rulebook") }];
    let corpus = load_equipment_corpus(&roots);
    assert!(corpus.len() > 1000, "the real book, got {} records: {:?}", corpus.len(), corpus.diagnostics);
    corpus
}

fn selection(item_id: &str, state: ActiveState) -> EquipmentSelection {
    EquipmentSelection {
        item_id: item_id.to_owned(),
        equipped_or_active: state == ActiveState::EquippedActive,
        active_state: state,
        applied_modifiers: Vec::new(),
    }
}

const SHIRT: &str = "Chain Shirt (Base)";
const SWORD: &str = "Longsword (Base)";

/// The GE-06 Human Fighter 1 as its file says: Chain Shirt and Longsword on, no shield.
fn fighter_as_filed() -> CharacterInput {
    let text = std::fs::read_to_string("tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt")
        .expect("the fixture is tracked");
    load_character_input_fixture(&text).character_input.expect("the fixture parses")
}

/// The same Fighter, each item by the Core Rulebook identity that states its numbers: whole-book,
/// `item:chain_shirt` folds to a record of that name which states nothing (`Chain Shirt Barding`).
fn fighter() -> CharacterInput {
    let mut input = fighter_as_filed();
    for selection in &mut input.chosen.equipment_selections {
        match selection.item_id.as_str() {
            "item:chain_shirt" => selection.item_id = SHIRT.to_owned(),
            "item:longsword" => selection.item_id = SWORD.to_owned(),
            _ => {}
        }
    }
    input
}

fn with_state(mut input: CharacterInput, item_id: &str, state: ActiveState) -> CharacterInput {
    let held = input.chosen.equipment_selections.iter_mut().find(|s| s.item_id == item_id).expect("held");
    held.active_state = state;
    held.equipped_or_active = state == ActiveState::EquippedActive;
    input
}

fn holding(mut input: CharacterInput, item_id: &str, state: ActiveState) -> CharacterInput {
    input.chosen.equipment_selections.push(selection(item_id, state));
    input
}

/// `character_input.rs`: a fixture's state token is the state, and `equipped_or_active` follows it.
#[test]
fn worn_vs_carried_character_input_states_are_what_the_file_says() {
    let input = fighter_as_filed();
    let state_of = |id: &str| {
        let held = input.chosen.equipment_selections.iter().find(|s| s.item_id == id).unwrap();
        (held.active_state, held.equipped_or_active)
    };
    assert_eq!(state_of("item:chain_shirt"), (ActiveState::EquippedActive, true));
    assert_eq!(state_of("item:shield"), (ActiveState::Absent, false));
    assert_eq!(state_of("power_attack"), (ActiveState::SelectedInactive, false));
}

/// `pilot_compute_corpus.rs`: the receipt's effects hold only what is worn.
#[test]
fn worn_vs_carried_pilot_compute_corpus_counts_only_what_is_worn() {
    let corpus = core_rulebook();
    let effects = |input: &CharacterInput| compute_pilot_with_corpus(input, &corpus).corpus_derived.equipment_effects;

    let on = effects(&fighter());
    assert_eq!(on.armor_class_delta, 4, "Chain Shirt +4");
    assert_eq!(on.max_dex_cap, Some(4));
    assert_eq!(on.armor_check_penalty_total, -2);

    let stowed = effects(&with_state(fighter(), SHIRT, ActiveState::SelectedInactive));
    assert_eq!(stowed.armor_class_delta, 0);
    assert_eq!(stowed.max_dex_cap, None);
    assert_eq!(stowed.armor_check_penalty_total, 0);
    assert!(stowed.per_item.iter().all(|item| item.item_id != SHIRT));

    let ring = "Ring of Protection +1";
    let carried = effects(&holding(fighter(), ring, ActiveState::SelectedInactive));
    assert_eq!(carried.armor_class_delta, 4, "a stowed ring adds 0");
    let worn = effects(&holding(fighter(), ring, ActiveState::EquippedActive));
    assert_eq!(worn.armor_class_delta, 5, "a worn ring adds 1");
}

/// `contract.rs`: the second receipt filters the same way, and weighs the unfiltered list.
#[test]
fn worn_vs_carried_contract_receipt_agrees() {
    let corpus = core_rulebook();
    let receipt = |input: &CharacterInput| {
        to_pilot_receipt(&compute_pilot_with_corpus(input, &corpus), input, &corpus)
    };
    let on = receipt(&fighter());
    let stowed = receipt(&with_state(fighter(), SHIRT, ActiveState::SelectedInactive));
    assert_eq!(on.equipment_effects.armor_class_delta, 4);
    assert_eq!(stowed.equipment_effects.armor_class_delta, 0);
    assert_eq!(stowed.encumbrance.total_carried_weight_lbs, on.encumbrance.total_carried_weight_lbs);

    let carried = receipt(&holding(fighter(), SWORD, ActiveState::SelectedInactive));
    assert_eq!(carried.weapon_damage.len(), on.weapon_damage.len(), "a stowed sword has no damage row");
}

/// `encumbrance.rs`: worn and carried weigh the same, `Absent` not at all.
#[test]
fn worn_vs_carried_encumbrance_weighs_both_states_and_not_absent() {
    let corpus = core_rulebook();
    let weigh = |state: ActiveState| {
        compute_encumbrance(&[selection("Backpack", state)], &corpus, 16, SizeCategory::Medium).total_carried_weight_lbs
    };
    assert_eq!(weigh(ActiveState::EquippedActive), 2.0);
    assert_eq!(weigh(ActiveState::SelectedInactive), 2.0);
    assert_eq!(weigh(ActiveState::Absent), 0.0);
}

/// `damage_total.rs`: a weapon in the pack has no damage row.
#[test]
fn worn_vs_carried_damage_total_has_no_row_for_a_stowed_weapon() {
    let corpus = core_rulebook();
    let rows = |input: &CharacterInput| {
        let selections: Vec<EquipmentSelection> = input
            .chosen
            .equipment_selections
            .iter()
            .filter(|s| s.active_state == ActiveState::EquippedActive)
            .cloned()
            .collect();
        let effects = compute_equipment_effects(&selections, &corpus);
        resolve_weapon_damage_breakdown(input, &corpus, &effects, 3)
    };
    let on = rows(&fighter());
    assert_eq!(on.len(), 1, "the Longsword");
    assert!(rows(&with_state(fighter(), SWORD, ActiveState::SelectedInactive)).is_empty());
}
