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
use codex::rules_core::equipment_effects::{compute_equipment_effects, skill_set_rows, worn_item_stated_bonuses};
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
        compute_encumbrance(&[selection("Backpack", state)], &corpus, 16, SizeCategory::Medium, &[], &[]).total_carried_weight_lbs
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

// --- A worn item's stated numbers reach the totals. The sheet's own read of each is the same filter
// in `apps/desktop/src-tauri/src/character_hub.rs`. ---

fn worn(items: &[&str]) -> Vec<EquipmentSelection> {
    items.iter().map(|item| selection(item, ActiveState::EquippedActive)).collect()
}

/// Two of one type take the higher, a row that loses reads zero, and the rows sum to the total.
#[test]
fn worn_item_applies_the_higher_of_one_armor_class_type() {
    let corpus = core_rulebook();
    let effects = compute_equipment_effects(&worn(&["Ring of Protection +1", "Ring of Protection +2"]), &corpus);
    assert_eq!(effects.armor_class_delta, 2);
    assert_eq!(effects.per_item[0].armor_class_bonus, Some(0));
    assert_eq!(effects.per_item[1].armor_class_bonus, Some(2));

    let two_types = compute_equipment_effects(&worn(&["Ring of Protection +1", "Amulet of Natural Armor +1"]), &corpus);
    assert_eq!(two_types.armor_class_delta, 2, "two types add");
    assert_eq!(two_types.armor_class_not_against_touch(), 1, "the amulet's");

    // An armor's own `+N` belongs to its armor bonus: +2 Chain Shirt is 6 against Bracers +5.
    let mut shirt = selection(SHIRT, ActiveState::EquippedActive);
    shirt.applied_modifiers.push("Special Ability ~ +2 ~ Armor".to_owned());
    let over_bracers = compute_equipment_effects(&[shirt, selection("Bracers of Armor +5", ActiveState::EquippedActive)], &corpus);
    assert_eq!(over_bracers.armor_class_delta, 6);
    assert_eq!(over_bracers.armor_class_not_against_touch(), 6);
    assert_eq!(over_bracers.per_item[1].armor_class_bonus, Some(0), "the bracers cease");
}

/// A record stating no number reads the record it is made from; its own stated number stands.
#[test]
fn worn_item_applies_a_derived_record_reads_what_it_does_not_state() {
    let corpus = core_rulebook();
    let celestial = compute_equipment_effects(&worn(&["Celestial Armor"]), &corpus);
    assert_eq!(celestial.armor_class_delta, 6, "Chainmail's");
    assert_eq!(celestial.armor_class_not_against_touch(), 6);

    let carried = [selection("Mithral Shirt", ActiveState::SelectedInactive), selection("Flame Tongue", ActiveState::SelectedInactive)];
    let load = compute_encumbrance(&carried, &corpus, 10, SizeCategory::Medium, &[], &[]);
    assert_eq!(load.total_carried_weight_lbs, 24.0, "Mithral Shirt's own 20, Flame Tongue reads Longsword's 4");
    assert!(load.unresolved_item_ids.is_empty(), "{:?}", load.unresolved_item_ids);
}

/// Saves and skills and scores, each by its settled type; carried, none.
#[test]
fn worn_item_applies_only_what_is_worn_and_stacks_each_target() {
    let corpus = core_rulebook();
    let both = worn_item_stated_bonuses(&worn(&["Cloak of Resistance +1", "Cloak of Resistance +3", "Ioun Stone (Pale green Prism)"]), &corpus);
    assert_eq!(both.save("fortitude"), 4, "the higher resistance bonus, and the competence bonus");
    assert_eq!(both.save("reflex"), 4);
    assert_eq!(both.save("will"), 4);

    let input = fighter();
    let score = input.chosen.ability_scores.strength;
    let belt = holding(input.clone(), "Belt of Giant Strength +2", ActiveState::EquippedActive);
    let stated = worn_item_stated_bonuses(&belt.chosen.equipment_selections, &corpus);
    let raised = stated.raised(&belt);
    assert_eq!(raised.chosen.ability_scores.strength, score + 2);
    assert_eq!(belt.chosen.ability_scores.strength, score, "the character itself does not change");

    let boots = worn_item_stated_bonuses(&worn(&["Boots of Elvenkind"]), &corpus);
    assert_eq!(boots.counted("skill:acrobatics", "Competence"), Some(5));

    let stowed = holding(fighter(), "Cloak of Resistance +1", ActiveState::SelectedInactive);
    let none = worn_item_stated_bonuses(&stowed.chosen.equipment_selections, &corpus);
    assert_eq!(none.save("will"), 0, "carried adds 0");
}

/// Attack, initiative and speed, each by its settled type; carried, none.
#[test]
fn worn_item_applies_attack_initiative_and_speed_by_type() {
    let corpus = core_rulebook();
    let stones = worn_item_stated_bonuses(&worn(&["Ioun Stone (Pale green Prism)", "Stone of Good Luck (Luckstone)", "Boots of Striding And Springing"]), &corpus);
    assert_eq!(stones.counted("attack", "Competence"), Some(1));
    assert_eq!(stones.counted("initiative", "Luck"), Some(1));
    assert_eq!(stones.counted("speed", "Enhancement"), Some(10));

    let stowed = [selection("Ioun Stone (Pale green Prism)", ActiveState::SelectedInactive), selection("Boots of Striding And Springing", ActiveState::SelectedInactive)];
    let none = worn_item_stated_bonuses(&stowed, &corpus);
    assert_eq!(none.rows(), &[], "carried adds 0");
}

/// What a chain settles to, off the book's own records: a type, `ALL`, a list, a luck formula and an
/// untyped penalty are numbers; a chain that asks something of the wearer, or a choice, is not.
#[test]
fn worn_item_applies_settles_each_chain_shape() {
    let rows = |book: &'static str, item: &str| -> Vec<(String, Option<String>, i16)> {
        let dir = format!("data/corpus/{book}");
        let roots = [BookCorpusRoot { book_id: book, dir: Path::new(&dir) }];
        let corpus = load_equipment_corpus(&roots);
        let record = codex::rules_core::equipment_resolver::equipment_converted_resolve(item, &corpus).expect(item);
        record.stated_bonuses.iter().map(|b| (b.target.clone(), b.bonus_type.clone(), b.value)).collect()
    };
    let saves = |word: Option<&str>, value: i16| -> Vec<(String, Option<String>, i16)> {
        ["save:fortitude", "save:reflex", "save:will"].iter().map(|t| (t.to_string(), word.map(str::to_owned), value)).collect()
    };
    assert_eq!(rows("occult_adventures", "Lucky Horseshoe"), saves(Some("Luck"), 1), "`1+Global_LuckBonus`");
    let mut robe = vec![("initiative".to_owned(), None, -5), ("attack".to_owned(), None, -2)];
    robe.extend(saves(None, -2));
    robe.push(("skill_set:all".to_owned(), None, -2));
    assert_eq!(rows("ultimate_equipment", "Robe of Vermin"), robe, "an untyped penalty");
    assert_eq!(rows("ultimate_psionics", "Boots of Skating")[0], ("speed".to_owned(), Some("Enhancement".to_owned()), 15), "`MOVEADD|TYPE=Walk`");
    assert_eq!(rows("ultimate_equipment", "Pauldrons of the Jackass")[0], ("speed".to_owned(), None, -10), "`MOVEADD|TYPE.All`");
    assert_eq!(rows("core_rulebook", "Boots of Striding And Springing"), vec![("speed".to_owned(), Some("Enhancement".to_owned()), 10)], "`POSTMOVEADD`; `SITUATION` settles nothing");
    assert_eq!(rows("ultimate_equipment", "Scattershot Bracers"), vec![], "`TOHIT.Ranged` is no number on this sheet");
    assert_eq!(rows("ultimate_psionics", "Tempest's Blade"), vec![], "a formula");
    assert_eq!(rows("core_rulebook", "Cloak of Resistance +3"), saves(Some("Resistance"), 3), "`BONUS:VAR`");
    assert_eq!(
        rows("inner_sea_gods", "Tankard Of The Drunken Hero"),
        vec![("save:fortitude".to_owned(), Some("Resistance".to_owned()), 1), ("save:will".to_owned(), Some("Resistance".to_owned()), 1)],
        "a list",
    );
    assert_eq!(rows("advanced_race_guide", "Halo of Inner Calm"), vec![], "`PRERACE`, `PREALIGN`");
    assert_eq!(rows("ultimate_equipment", "Resplendent Robe of the Thespian"), vec![], "`PRECLASS`");
    assert_eq!(rows("inner_sea_gods", "Rhombocrystal"), vec![], "`PREDEITY`");
}

fn book(book_id: &'static str) -> SourcePackageContent<'static> {
    let dir: &'static str = Box::leak(format!("data/corpus/{book_id}").into_boxed_str());
    let roots = [BookCorpusRoot { book_id, dir: Path::new(dir) }];
    load_equipment_corpus(&roots)
}

/// A skill set's rows are the sets the book's own skill records state: `TYPE:Dexterity.ACHECK.Base`.
/// A family's subtypes are its one row, and a skill's `(Untrained)` record is no row.
#[test]
fn worn_item_applies_every_skill_each_set_is_the_books_own_type() {
    let mut stated: Vec<(String, Vec<String>)> = Vec::new();
    for entry in std::fs::read_dir("data/corpus/core_rulebook/skill").expect("the book's skills") {
        let text = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        let record: serde_json::Value = serde_json::from_str(&text).unwrap();
        let data = &record["data"];
        let key = data["key"].as_str().unwrap_or_default();
        let types = data["raw_tokens"].as_array().into_iter().flatten().find(|token| token["key"] == "TYPE").and_then(|token| token["value"].as_str());
        let Some(types) = types.filter(|_| !key.contains("Untrained")) else { continue };
        let words: Vec<String> = key.split(|c: char| !c.is_ascii_alphanumeric()).filter(|word| !word.is_empty()).map(str::to_ascii_lowercase).collect();
        let row = if ["craft", "perform", "profession"].contains(&words[0].as_str()) { format!("skill:{}", words[0]) } else { format!("skill:{}", words.join("_")) };
        stated.push((row, types.split('.').map(str::to_ascii_lowercase).collect()));
    }
    assert!(stated.len() > 90, "the whole book, got {}", stated.len());
    for set in ["base", "knowledge", "perform", "strength", "dexterity", "intelligence", "wisdom", "charisma"] {
        let mut expected: Vec<String> = stated.iter().filter(|(_, types)| types.iter().any(|word| word == set)).map(|(row, _)| row.clone()).collect();
        expected.sort();
        expected.dedup();
        let mut rows = skill_set_rows(set);
        rows.sort();
        assert_eq!(rows, expected, "{set}");
    }
    assert_eq!(skill_set_rows("all").len(), 35, "the panel's 32 and its 3 families");
    assert_eq!(skill_set_rows("constitution"), Vec::<String>::new(), "no skill keys off it");
}

/// What each kind of chain settles to, off the book's own records.
#[test]
fn worn_item_applies_every_skill_settles_each_chain_shape() {
    let rows = |book_id: &'static str, item: &str| -> Vec<(String, Option<String>, i16)> {
        let corpus = book(book_id);
        let record = codex::rules_core::equipment_resolver::equipment_converted_resolve(item, &corpus).expect(item);
        record.stated_bonuses.iter().map(|b| (b.target.clone(), b.bonus_type.clone(), b.value)).collect()
    };
    let has = |book_id: &'static str, item: &str, target: &str, word: Option<&str>, value: i16| {
        let held = rows(book_id, item);
        assert!(held.contains(&(target.to_owned(), word.map(str::to_owned), value)), "{item}: {held:?}");
    };
    has("core_rulebook", "Ioun Stone (Pale green Prism)", "skill_set:base", Some("Competence"), 1);
    has("ultimate_equipment", "Robe of Vermin", "skill_set:all", None, -2);
    has("ultimate_equipment", "Bracers of the Glib Entertainer", "skill_set:perform", Some("Competence"), 5);
    has("ultimate_equipment", "Staff of Accompaniment", "skill_set:perform", Some("Competence"), 2);
    has("ultimate_equipment", "Mask of a Thousand Tomes", "skill_set:knowledge", Some("Competence"), 10);
    has("ultimate_equipment", "Headband of Stupidity", "skill_set:knowledge", None, -2);
    has("inner_sea_gods", "Blade Of Three Fancies", "skill_set:perform", Some("Sacred"), 4);
    for set in ["strength", "dexterity", "intelligence", "wisdom", "charisma"] {
        has("core_rulebook", "Stone of Good Luck (Luckstone)", &format!("skill_set:{set}"), Some("Luck"), 1);
    }
    has("advanced_players_guide", "Boneless Leather", "cmb", None, 5);
    has("ultimate_equipment", "Juggernaut's Pauldrons", "cmd", None, 4);
    has("ultimate_equipment", "Unlucky Figurine", "cmd", None, -2);
    has("ultimate_equipment", "Unlucky Figurine", "attack", None, -2);
    assert_eq!(rows("ultimate_equipment", "Stone of Weight (Loadstone)"), vec![("speed_percent".to_owned(), Some("Percent of speed".to_owned()), 50)]);
}

/// A set's bonus is on each of its rows, and folds there by type; carried, none.
#[test]
fn worn_item_applies_every_skill_a_set_stacks_on_each_row() {
    let core = book("core_rulebook");
    let prism = worn_item_stated_bonuses(&worn(&["Ioun Stone (Pale green Prism)"]), &core);
    for row in skill_set_rows("all") {
        assert_eq!(prism.counted(&row, "Competence"), Some(1), "{row}");
    }
    let stowed = [selection("Ioun Stone (Pale green Prism)", ActiveState::SelectedInactive)];
    assert_eq!(worn_item_stated_bonuses(&stowed, &core).rows(), &[], "carried adds 0");

    let ue = book("ultimate_equipment");
    let figurine = worn(&["Unlucky Figurine"]);
    let effects = compute_equipment_effects(&figurine, &ue);
    assert_eq!(effects.armor_class_delta, -2, "`COMBAT|ATTACK,AC`: the Armor Class half");
    assert_eq!(effects.max_dex_cap, None, "it is no armor");
    assert_eq!(effects.armor_class_not_against_touch(), 0, "a touch attack does not ignore it");
    assert_eq!(effects.armor_class_lost_flat_footed(), 0, "nor does being flat-footed");
    let stated = worn_item_stated_bonuses(&figurine, &ue);
    let on = |target: &str| stated.rows().iter().filter(|row| row.target == target).map(|row| row.value).sum::<i16>();
    assert_eq!((on("skill:bluff"), on("skill:perform"), on("skill:knowledge_arcana")), (-2, -2, -2));
    assert_eq!(on("ac"), -2);

    let two = worn_item_stated_bonuses(&worn(&["Stone of Weight (Loadstone)", "Stone of Weight (Loadstone)"]), &ue);
    assert_eq!(two.rows().len(), 1, "one multiplier counts: {:?}", two.rows());
}
