//! The Abilities sub-tabs' commands (SD-43 E3.2, `technical-design.md` §3.2): list a sub-tab's
//! options with `qualified` and the unmet reasons, add a pick, remove one. The budget and the
//! prerequisites are the engine's (`codex::rules_core::ability_budget`); this file loads the saved
//! character, asks the engine, and saves through the same recompute-before-persist gate every add
//! path uses. Feats keep their own commands.
//!
//! **Racial Abilities** (SD-43 E3.6) are the race's alternate racial traits, offered and refused by the
//! race picker's own resolution (`race_trait_picker::build_alternate_racial_traits` /
//! `build_race_selection`, the functions behind `list_alternate_racial_traits` /
//! `resolve_race_alternate_selection`) and saved where creation saves them (`selected_choices`,
//! `choice:race_alternate_trait`). Two alternates that replace the same racial trait are refused. They
//! have no count budget, so the sub-tab serves no budget rows; a replaced trait that grants a feat
//! leaves the Feats sub-tab's racial pool (`feat_budget` reads the applied racial traits).
//!
//! **Archetype** (SD-43 E3.7) is an `ability_budget` sub-tab: the engine lists each held class's
//! archetypes and refuses one replacing a class feature a held archetype already replaces; a pick is
//! saved as a `SelectedChoice` under `ARCHETYPE_CHOICE_ID`.
//!
//! **Favored Class** (SD-43 E3.8, `decisions.md §19`) is an `ability_budget` sub-tab shown as a section of
//! Class Features: the favored class, then a bonus per level of it. A PCGen import's favored class bonus
//! is already Custom grants (`pcg_import/map.rs` `add_favored_class_bonus`, ids `favored-class-…`); those
//! spend the bonus pool here ([`imported_favored_bonuses`]) so the same levels are not paid twice.
//!
//! **Background** and **Misc** (SD-43 E3.9) are `ability_budget` sub-tabs over PCGen's ability categories shown
//! there; a pick is saved as an `ability_selections` row like a Class Features pick, at the character's level.

use std::path::Path;

use codex::rules_core::ability_budget::{
    ability_budget, ability_options, favored_class_budget, favored_next_level, favored_pick_of, AbilityOption, SubTab, FAVORED_BONUS_POOL,
    FAVORED_CLASS_CHOICE_ID, FAVORED_CLASS_POOL,
};
use codex::rules_core::ability_prereqs;
use codex::rules_core::archetype_resolver::ARCHETYPE_CHOICE_ID;
use codex::rules_core::character_input::AbilitySelection;
use codex::rules_core::game_system::GameSystem;
use codex::rules_core::character_input::{CharacterInput, SelectedChoice};
use codex::rules_core::pilot_compute::{compute_pilot_base_chassis, race_alternate_trait_selection_id, RACE_ALTERNATE_TRAIT_CHOICE_ID};
use codex::saved_character::local_store::SavedCharacterStore;
use serde::{Deserialize, Serialize};

use crate::character_hub::{
    apply_add_trait_selection, map_explanations_dto, mutate_saved_character_at_root, read_alternate_trait_keys,
    remove_trait_selection_at_root, resolve_alternate_trait_choices, resolve_character_root, sheet_rule_package_for,
    CreateCharacterResponse, ExplanationDto,
};
use crate::race_trait_picker::{build_alternate_racial_traits, build_race_selection, RaceSelectionRequest};

/// The Racial Abilities sub-tab's id; its picks are the race picker's, not an `ability_budget` sub-tab's.
const RACIAL_ABILITIES: &str = "racial_abilities";
/// The pool an alternate racial trait is listed under.
const ALTERNATE_POOL: &str = "alternate_racial_trait";

/// This character's race's alternate racial traits, each refused while a held alternate already replaces
/// a trait it replaces (the `PREMULT` self-exclusion guard the picker reads) or while it replaces nothing
/// in the loaded books; and the held keys.
fn racial_options(input: &CharacterInput) -> Result<(Vec<AbilityOptionDto>, Vec<String>), String> {
    let held = read_alternate_trait_keys(input);
    let resolution =
        build_race_selection(&RaceSelectionRequest { race_key: input.chosen.race_id.clone(), selected_alternate_keys: held.clone() });
    if !resolution.errors.is_empty() {
        return Err(resolution.errors.join("; "));
    }
    let menu = build_alternate_racial_traits();
    let race = menu
        .races
        .iter()
        .find(|race| race.race_key == resolution.race_key)
        .ok_or_else(|| format!("the race picker lists no race {:?}", resolution.race_key))?;
    let options = race
        .alternates
        .iter()
        .map(|alternate| {
            let replaced = |flag: &str| {
                let names: Vec<&str> = alternate.replaces.iter().filter(|r| r.flag == flag).map(|r| r.name.as_str()).collect();
                if names.is_empty() { flag.to_owned() } else { names.join(", ") }
            };
            let mut unmet: Vec<String> = resolution
                .blocked_alternates
                .iter()
                .chain(&resolution.conflicting_selections)
                .filter(|blocked| blocked.key == alternate.key)
                .map(|blocked| {
                    format!("replaces {}, which {} already replaces; a racial trait is replaced once", replaced(&blocked.flag), blocked.blocked_by_name)
                })
                .collect();
            unmet.extend(alternate.unmatched_flags.iter().map(|flag| {
                format!("its {flag} replaces and grants nothing in the loaded books, so the trait it names would stay")
            }));
            let replaces: Vec<&str> = alternate.replaces.iter().map(|r| r.name.as_str()).collect();
            AbilityOptionDto {
                id: alternate.key.clone(),
                label: alternate.name.clone(),
                pool: ALTERNATE_POOL.to_owned(),
                category: (!replaces.is_empty()).then(|| format!("Replaces {}", replaces.join(", "))),
                qualified: unmet.is_empty(),
                unmet,
                description: Some(alternate.description.clone()).filter(|d| !d.trim().is_empty()),
            }
        })
        .collect();
    Ok((options, held))
}

/// Saves `key` beside the held alternates once the creation path's validation accepts the new set.
fn add_racial_ability_at_root(root: &Path, key: &str, saved_at: &str) -> Result<CreateCharacterResponse, String> {
    let input = SavedCharacterStore::load(root).map_err(|err| err.message)?.character_input;
    let (options, mut held) = racial_options(&input)?;
    if held.iter().any(|k| k == key) {
        return Err(format!("This character already holds '{key}'."));
    }
    let option = options.iter().find(|o| o.id == key).ok_or_else(|| format!("'{key}' is not an alternate racial trait of this character's race"))?;
    if !option.qualified {
        return Err(format!("'{}' cannot be taken by this character: {}", option.label, option.unmet.join("; ")));
    }
    held.push(key.to_owned());
    let choices: Vec<SelectedChoice> = resolve_alternate_trait_choices(&input.chosen.race_id, &held)
        .map_err(|diagnostics| diagnostics.into_iter().map(|d| d.message).collect::<Vec<_>>().join("; "))?;
    mutate_saved_character_at_root(root, saved_at, |input| {
        input.chosen.selected_choices.retain(|c| c.choice_set_id != RACE_ALTERNATE_TRAIT_CHOICE_ID);
        input.chosen.selected_choices.extend(choices);
    })
}

fn remove_racial_ability_at_root(root: &Path, key: &str, saved_at: &str) -> Result<CreateCharacterResponse, String> {
    let input = SavedCharacterStore::load(root).map_err(|err| err.message)?.character_input;
    if !read_alternate_trait_keys(&input).iter().any(|k| k == key) {
        return Err(format!("This character does not hold '{key}'."));
    }
    let selection = race_alternate_trait_selection_id(key);
    mutate_saved_character_at_root(root, saved_at, |input| {
        input.chosen.selected_choices.retain(|c| !(c.choice_set_id == RACE_ALTERNATE_TRAIT_CHOICE_ID && c.selection_id == selection));
    })
}

/// The id prefix the PCGen import gives its favored class bonus grants (`pcg_import/map.rs`).
const IMPORTED_FAVORED_GRANT_PREFIX: &str = "favored-class-";

/// How many favored class bonuses a PCGen import took as Custom grants: one a hit point or skill rank.
pub(crate) fn imported_favored_bonuses(root: &Path) -> i64 {
    crate::character_custom::load_character_custom_at_root(root)
        .map(|custom| custom.grants.iter().filter(|grant| grant.id.starts_with(IMPORTED_FAVORED_GRANT_PREFIX)).map(|grant| i64::from(grant.value.max(0))).sum())
        .unwrap_or(0)
}

/// A sub-tab's budget; Favored Class with the imported bonuses spent.
fn budget_at_root(root: &Path, package: &codex::rules_core::sheet_rule::SheetRulePackage, input: &CharacterInput, subtab: SubTab) -> codex::rules_core::ability_budget::AbilityBudget {
    match subtab {
        SubTab::FavoredClass => favored_class_budget(package, input, imported_favored_bonuses(root)),
        _ => ability_budget(package, input, subtab),
    }
}

fn subtab_of(id: &str) -> Result<SubTab, String> {
    SubTab::from_id(id).ok_or_else(|| format!("'{id}' is not an Abilities sub-tab with an engine budget"))
}

fn package() -> Result<&'static codex::rules_core::sheet_rule::SheetRulePackage, String> {
    sheet_rule_package_for(GameSystem::Pathfinder1e).as_ref().map_err(Clone::clone)
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAbilityOptionsRequest {
    pub character_id: String,
    pub subtab: String,
}

/// One option as the sub-tab renders it.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AbilityOptionDto {
    pub id: String,
    pub label: String,
    pub pool: String,
    pub category: Option<String>,
    pub qualified: bool,
    pub unmet: Vec<String>,
    /// The record's rule text when the engine carries it.
    pub description: Option<String>,
}

impl From<AbilityOption> for AbilityOptionDto {
    fn from(option: AbilityOption) -> Self {
        AbilityOptionDto {
            id: option.id,
            label: option.label,
            pool: option.pool,
            category: option.category,
            qualified: option.qualified,
            unmet: option.unmet,
            description: option.description,
        }
    }
}

/// A sub-tab's options and its `abilities.budget.<subtab>.*` rows.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityOptionsDto {
    pub budget: Vec<ExplanationDto>,
    pub options: Vec<AbilityOptionDto>,
    /// The option ids the character holds, in saved order (Class Features: `<pool>:<record id>`).
    pub selected: Vec<String>,
}

/// The option ids of the picks a sub-tab saves, in saved order.
fn selected_ids(input: &CharacterInput, subtab: SubTab) -> Vec<String> {
    match subtab {
        SubTab::Traits => input.chosen.selected_traits.clone(),
        SubTab::ClassFeatures | SubTab::Background | SubTab::Misc => input
            .chosen
            .ability_selections
            .iter()
            .filter(|pick| pick.subtab == subtab.id())
            .map(|pick| format!("{}:{}", pick.pool, pick.record_key))
            .collect(),
        SubTab::Archetype => input
            .chosen
            .selected_choices
            .iter()
            .filter(|choice| choice.choice_set_id == ARCHETYPE_CHOICE_ID)
            .map(|choice| choice.selection_id.clone())
            .collect(),
        SubTab::FavoredClass => input
            .chosen
            .selected_choices
            .iter()
            .filter(|choice| choice.choice_set_id == FAVORED_CLASS_CHOICE_ID)
            .map(|choice| format!("{FAVORED_CLASS_POOL}:{}", choice.selection_id))
            .chain(
                input
                    .chosen
                    .ability_selections
                    .iter()
                    .filter(|pick| pick.subtab == subtab.id())
                    .map(|pick| format!("{FAVORED_BONUS_POOL}:{}@{}", pick.record_key, pick.level)),
            )
            .collect(),
    }
}

#[tauri::command]
pub fn list_ability_options(app: tauri::AppHandle, request: ListAbilityOptionsRequest) -> Result<AbilityOptionsDto, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    list_ability_options_at_root(&root, &request.subtab)
}

pub(crate) fn list_ability_options_at_root(root: &Path, subtab: &str) -> Result<AbilityOptionsDto, String> {
    if subtab == RACIAL_ABILITIES {
        let input = SavedCharacterStore::load(root).map_err(|err| err.message)?.character_input;
        let (options, selected) = racial_options(&input)?;
        return Ok(AbilityOptionsDto { budget: Vec::new(), options, selected });
    }
    let subtab = subtab_of(subtab)?;
    let package = package()?;
    let input = SavedCharacterStore::load(root).map_err(|err| err.message)?.character_input;
    let base = compute_pilot_base_chassis(&input);
    Ok(AbilityOptionsDto {
        budget: map_explanations_dto(&budget_at_root(root, package, &input, subtab).explanation_rows()),
        options: ability_options(package, &input, &base, subtab).into_iter().map(AbilityOptionDto::from).collect(),
        selected: selected_ids(&input, subtab),
    })
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilitySelectionRequest {
    pub character_id: String,
    pub subtab: String,
    /// An option's `id` from `list_ability_options`.
    pub option_id: String,
    pub saved_at: String,
}

/// Adds the pick when the engine's budget has room and the character qualifies; otherwise `Err`
/// with the engine's reason, leaving the saved character untouched.
#[tauri::command]
pub fn add_ability_selection(app: tauri::AppHandle, request: AbilitySelectionRequest) -> Result<CreateCharacterResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    add_ability_selection_at_root(&root, &request.subtab, &request.option_id, &request.saved_at)
}

pub(crate) fn add_ability_selection_at_root(
    root: &Path,
    subtab: &str,
    option_id: &str,
    saved_at: &str,
) -> Result<CreateCharacterResponse, String> {
    if subtab == RACIAL_ABILITIES {
        return add_racial_ability_at_root(root, option_id, saved_at);
    }
    let subtab = subtab_of(subtab)?;
    let package = package()?;
    let input = SavedCharacterStore::load(root).map_err(|err| err.message)?.character_input;
    let budget = budget_at_root(root, package, &input, subtab);
    if let Some(refusal) = budget.refusal(package, option_id) {
        return Err(refusal);
    }
    let base = compute_pilot_base_chassis(&input);
    let option = ability_options(package, &input, &base, subtab)
        .into_iter()
        .find(|option| option.id == option_id)
        .ok_or_else(|| format!("'{option_id}' is not an option of the {} sub-tab", subtab.id()))?;
    if let Some(refusal) = ability_prereqs::refusal(&option) {
        return Err(refusal);
    }
    match subtab {
        SubTab::Traits => mutate_saved_character_at_root(root, saved_at, |input| apply_add_trait_selection(input, option_id, None)),
        SubTab::ClassFeatures => {
            let (pool, record_key) = option_id.split_once(':').ok_or_else(|| format!("'{option_id}' names no class feature pool"))?;
            let level = budget
                .pool(pool)
                .and_then(|pool| pool.next_pick_level())
                .ok_or_else(|| format!("The {pool} pool has no grant left to charge '{}' to", option.label))?;
            let pick = AbilitySelection { subtab: subtab.id().to_owned(), pool: pool.to_owned(), record_key: record_key.to_owned(), level, target: None };
            mutate_saved_character_at_root(root, saved_at, |input| input.chosen.ability_selections.push(pick))
        }
        SubTab::Background | SubTab::Misc => {
            let (pool, record_key) = option_id.split_once(':').ok_or_else(|| format!("'{option_id}' names no {} pool", subtab.id()))?;
            let level = input.chosen.class_levels.iter().map(|class| class.level).sum::<u8>().max(1);
            let pick = AbilitySelection { subtab: subtab.id().to_owned(), pool: pool.to_owned(), record_key: record_key.to_owned(), level, target: None };
            mutate_saved_character_at_root(root, saved_at, |input| input.chosen.ability_selections.push(pick))
        }
        SubTab::Archetype => {
            let choice = SelectedChoice { choice_set_id: ARCHETYPE_CHOICE_ID.to_owned(), selection_id: option_id.to_owned() };
            mutate_saved_character_at_root(root, saved_at, |input| input.chosen.selected_choices.push(choice))
        }
        SubTab::FavoredClass => match option_id.split_once(':') {
            Some((FAVORED_CLASS_POOL, class)) => {
                let choice = SelectedChoice { choice_set_id: FAVORED_CLASS_CHOICE_ID.to_owned(), selection_id: class.to_owned() };
                mutate_saved_character_at_root(root, saved_at, |input| input.chosen.selected_choices.push(choice))
            }
            Some((_, record_key)) => {
                let level = favored_next_level(&input, imported_favored_bonuses(root))
                    .ok_or_else(|| format!("No level of the favored class is left to charge '{}' to", option.label))?;
                let pick = AbilitySelection { subtab: subtab.id().to_owned(), pool: FAVORED_BONUS_POOL.to_owned(), record_key: record_key.to_owned(), level, target: None };
                mutate_saved_character_at_root(root, saved_at, |input| input.chosen.ability_selections.push(pick))
            }
            None => Err(format!("'{option_id}' names no favored class pool")),
        },
    }
}

/// Removes a held pick; `Err` when the character does not hold it.
#[tauri::command]
pub fn remove_ability_selection(app: tauri::AppHandle, request: AbilitySelectionRequest) -> Result<CreateCharacterResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    remove_ability_selection_at_root(&root, &request.subtab, &request.option_id, &request.saved_at)
}

pub(crate) fn remove_ability_selection_at_root(
    root: &Path,
    subtab: &str,
    option_id: &str,
    saved_at: &str,
) -> Result<CreateCharacterResponse, String> {
    if subtab == RACIAL_ABILITIES {
        return remove_racial_ability_at_root(root, option_id, saved_at);
    }
    match subtab_of(subtab)? {
        SubTab::Traits => remove_trait_selection_at_root(root, option_id, saved_at),
        subtab @ (SubTab::ClassFeatures | SubTab::Background | SubTab::Misc) => {
            let input = SavedCharacterStore::load(root).map_err(|err| err.message)?.character_input;
            let held = |pick: &AbilitySelection| pick.subtab == subtab.id() && format!("{}:{}", pick.pool, pick.record_key) == option_id;
            let at = input
                .chosen
                .ability_selections
                .iter()
                .rposition(held)
                .ok_or_else(|| format!("This character does not hold '{option_id}'."))?;
            mutate_saved_character_at_root(root, saved_at, |input| {
                input.chosen.ability_selections.remove(at);
            })
        }
        SubTab::Archetype => {
            let input = SavedCharacterStore::load(root).map_err(|err| err.message)?.character_input;
            let held = |c: &SelectedChoice| c.choice_set_id == ARCHETYPE_CHOICE_ID && c.selection_id == option_id;
            if !input.chosen.selected_choices.iter().any(held) {
                return Err(format!("This character does not hold '{option_id}'."));
            }
            mutate_saved_character_at_root(root, saved_at, |input| input.chosen.selected_choices.retain(|c| !held(c)))
        }
        // Removing the favored class removes its bonuses: they were its levels'.
        subtab @ SubTab::FavoredClass => {
            let input = SavedCharacterStore::load(root).map_err(|err| err.message)?.character_input;
            if !selected_ids(&input, subtab).iter().any(|id| id == option_id) {
                return Err(format!("This character does not hold '{option_id}'."));
            }
            let pick = favored_pick_of(option_id);
            mutate_saved_character_at_root(root, saved_at, |input| match pick {
                Some((record, level)) => input.chosen.ability_selections.retain(|p| !(p.subtab == subtab.id() && p.record_key == record && p.level == level)),
                None => {
                    input.chosen.selected_choices.retain(|c| c.choice_set_id != FAVORED_CLASS_CHOICE_ID);
                    input.chosen.ability_selections.retain(|p| p.subtab != subtab.id());
                }
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAVED_AT: &str = "2026-11-23T00:00:00Z";

    /// Aldric Ironhand (Human Fighter 3, no traits), copied from the old-save fixture.
    fn aldric_root(label: &str) -> std::path::PathBuf {
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/sd41-old-save");
        let root = std::env::temp_dir().join(format!("codex-abilities-{label}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("temp root");
        for entry in std::fs::read_dir(&fixture).expect("fixture") {
            let entry = entry.expect("fixture file");
            std::fs::copy(entry.path(), root.join(entry.file_name())).expect("copy");
        }
        root
    }

    fn held_traits(root: &Path) -> Vec<String> {
        SavedCharacterStore::load(root).expect("reload").character_input.chosen.selected_traits
    }

    #[test]
    fn abilities_traits_add_until_the_pool_is_full_then_refuse_and_remove() {
        let root = aldric_root("traits");
        let listed = list_ability_options_at_root(&root, "traits").expect("options");
        let remaining = |rows: &[ExplanationDto]| rows.iter().find(|r| r.id == "abilities.budget.traits.traits.remaining").map(|r| r.value);
        assert_eq!(remaining(&listed.budget), Some(2));
        assert!(listed.options.iter().any(|o| o.id == "trait:trait_acrobat" && o.qualified));

        add_ability_selection_at_root(&root, "traits", "trait:trait_acrobat", SAVED_AT).expect("first trait");
        let refusal = add_ability_selection_at_root(&root, "traits", "trait:trait_ambitious", SAVED_AT).expect_err("a second Social trait");
        assert!(refusal.contains("Social"), "{refusal}");
        add_ability_selection_at_root(&root, "traits", "trait:trait_axe_to_grind", SAVED_AT).expect("a Combat trait");
        let refusal = add_ability_selection_at_root(&root, "traits", "trait:trait_air_touched", SAVED_AT).expect_err("a third trait");
        assert!(refusal.contains("traits pool"), "{refusal}");
        assert_eq!(held_traits(&root), ["trait:trait_acrobat", "trait:trait_axe_to_grind"]);
        assert_eq!(remaining(&list_ability_options_at_root(&root, "traits").expect("options").budget), Some(0));

        remove_ability_selection_at_root(&root, "traits", "trait:trait_acrobat", SAVED_AT).expect("remove");
        assert_eq!(held_traits(&root), ["trait:trait_axe_to_grind"]);
        assert!(list_ability_options_at_root(&root, "no_such_subtab").is_err(), "a sub-tab with no engine budget is refused");
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn abilities_class_features_a_rogue_4_takes_two_talents_charged_to_levels_2_and_4_then_is_refused() {
        let root = aldric_root("class-features");
        mutate_saved_character_at_root(&root, SAVED_AT, |input| {
            input.chosen.class_levels =
                vec![codex::rules_core::character_input::CharacterClassLevel { class_id: "class:rogue".to_owned(), level: 4 }];
        })
        .expect("make Aldric a Rogue 4");
        let listed = list_ability_options_at_root(&root, "class_features").expect("options");
        let row = |rows: &[ExplanationDto], field: &str| {
            rows.iter().find(|r| r.id == format!("abilities.budget.class_features.rogue_talent.{field}")).map(|r| r.value)
        };
        assert_eq!(row(&listed.budget, "granted"), Some(2));
        let talents: Vec<&AbilityOptionDto> = listed.options.iter().filter(|o| o.pool == "rogue_talent" && o.qualified).collect();
        assert!(talents.len() >= 2, "{} qualified rogue talents", talents.len());
        let (first, second) = (talents[0].id.clone(), talents[1].id.clone());

        add_ability_selection_at_root(&root, "class_features", &first, SAVED_AT).expect("first talent");
        add_ability_selection_at_root(&root, "class_features", &second, SAVED_AT).expect("second talent");
        let picks = SavedCharacterStore::load(&root).expect("reload").character_input.chosen.ability_selections;
        let levels: Vec<(String, u8)> = picks.iter().map(|p| (format!("{}:{}", p.pool, p.record_key), p.level)).collect();
        assert_eq!(levels, [(first.clone(), 2), (second.clone(), 4)]);
        let third = talents.iter().map(|o| o.id.clone()).find(|id| *id != first && *id != second).expect("a third talent");
        let refusal = add_ability_selection_at_root(&root, "class_features", &third, SAVED_AT).expect_err("no third talent at Rogue 4");
        assert!(refusal.contains("rogue_talent pool"), "{refusal}");
        let listed = list_ability_options_at_root(&root, "class_features").expect("options");
        assert_eq!(row(&listed.budget, "remaining"), Some(0));
        assert_eq!(listed.selected, [first.clone(), second.clone()]);

        remove_ability_selection_at_root(&root, "class_features", &first, SAVED_AT).expect("remove");
        assert_eq!(list_ability_options_at_root(&root, "class_features").expect("options").selected, [second]);
        std::fs::remove_dir_all(&root).ok();
    }

    /// Background (E0.1: 22 categories, 162 records): every character takes one region of origin (Inner Sea World
    /// Guide); a second is refused naming the pool; the pick saves, lists as selected, and removes.
    #[test]
    fn abilities_background_one_region_of_origin_then_refused_and_misc_lists_the_gm_awards() {
        let root = aldric_root("background");
        let listed = list_ability_options_at_root(&root, "background").expect("background options");
        let granted = listed.budget.iter().find(|r| r.id == "abilities.budget.background.region_of_origin.granted").map(|r| r.value);
        assert_eq!(granted, Some(1));
        let regions: Vec<&AbilityOptionDto> = listed.options.iter().filter(|o| o.pool == "region_of_origin" && o.qualified).collect();
        assert!(regions.len() >= 2, "{} regions", regions.len());
        let (first, second) = (regions[0].id.clone(), regions[1].id.clone());
        add_ability_selection_at_root(&root, "background", &first, SAVED_AT).expect("a region");
        let refusal = add_ability_selection_at_root(&root, "background", &second, SAVED_AT).expect_err("a second region");
        assert!(refusal.contains("region_of_origin pool"), "{refusal}");
        let listed = list_ability_options_at_root(&root, "background").expect("background options");
        assert_eq!(listed.selected, std::slice::from_ref(&first));
        let pick = SavedCharacterStore::load(&root).expect("reload").character_input.chosen.ability_selections;
        assert_eq!((pick[0].subtab.as_str(), pick[0].level), ("background", 3), "saved at Aldric's level");
        remove_ability_selection_at_root(&root, "background", &first, SAVED_AT).expect("remove");
        assert!(list_ability_options_at_root(&root, "background").expect("options").selected.is_empty());

        let misc = list_ability_options_at_root(&root, "misc").expect("misc options");
        let award = misc.options.iter().find(|o| o.pool == "gm_awards" && o.qualified).expect("a GM award");
        add_ability_selection_at_root(&root, "misc", &award.id, SAVED_AT).expect("a GM award (an editable pool)");
        assert_eq!(list_ability_options_at_root(&root, "misc").expect("misc").selected, std::slice::from_ref(&award.id));
        std::fs::remove_dir_all(&root).ok();
    }

    /// The feat budget's racial pool, as the Feats sub-tab reads it: `feats.budget.racial.granted`, or 0 when no row.
    fn racial_feat_pool(root: &Path) -> i64 {
        let input = SavedCharacterStore::load(root).expect("reload").character_input;
        let base = compute_pilot_base_chassis(&input);
        let budget = crate::character_hub::feat_budget_for(&input, &base).expect("feat budget");
        budget.pools().iter().find(|pool| pool.id == "racial").map_or(0, |pool| pool.granted)
    }

    /// Piety (Horror Adventures p.41): "This racial trait replaces the bonus feat racial trait." So does
    /// Rationalize; a human replaces the bonus feat once.
    #[test]
    fn abilities_racial_abilities_a_human_alternate_replaces_the_bonus_feat_and_a_second_replacing_it_is_refused() {
        let root = aldric_root("racial-abilities");
        assert_eq!(racial_feat_pool(&root), 1, "a human's bonus feat before any alternate");
        let listed = list_ability_options_at_root(&root, "racial_abilities").expect("options");
        for key in ["Human ~ Piety", "Human ~ Rationalize"] {
            assert!(listed.options.iter().any(|o| o.id == key && o.qualified), "{key} offered and qualified");
        }

        add_ability_selection_at_root(&root, "racial_abilities", "Human ~ Piety", SAVED_AT).expect("Piety");
        let input = SavedCharacterStore::load(&root).expect("reload").character_input;
        assert_eq!(crate::character_hub::read_alternate_trait_keys(&input), ["Human ~ Piety"]);
        assert_eq!(racial_feat_pool(&root), 0, "the feat budget's racial pool follows the replaced trait");
        let listed = list_ability_options_at_root(&root, "racial_abilities").expect("options");
        assert_eq!(listed.selected, ["Human ~ Piety"]);
        let rationalize = listed.options.iter().find(|o| o.id == "Human ~ Rationalize").expect("Rationalize still listed");
        assert!(!rationalize.qualified && rationalize.unmet.iter().any(|r| r.contains("Piety") && r.contains("Bonus Feat")), "{:?}", rationalize.unmet);

        let refusal = add_ability_selection_at_root(&root, "racial_abilities", "Human ~ Rationalize", SAVED_AT).expect_err("a second bonus-feat alternate");
        assert!(refusal.contains("Piety") && refusal.contains("Bonus Feat"), "{refusal}");
        let input = SavedCharacterStore::load(&root).expect("reload").character_input;
        assert_eq!(crate::character_hub::read_alternate_trait_keys(&input), ["Human ~ Piety"], "the refusal saved nothing");

        remove_ability_selection_at_root(&root, "racial_abilities", "Human ~ Piety", SAVED_AT).expect("remove Piety");
        assert_eq!(racial_feat_pool(&root), 1, "the bonus feat comes back");
        std::fs::remove_dir_all(&root).ok();
    }

    /// Acrobat and Rake (Advanced Player's Guide) both replace trapfinding; a class feature is replaced once.
    /// Pirate (Ultimate Combat) replaces the 2nd-level rogue talent, so the Class Features rogue talent pool shrinks.
    #[test]
    fn abilities_archetype_a_rogue_with_two_archetypes_replacing_trapfinding_is_refused_naming_the_slot() {
        let root = aldric_root("archetype");
        mutate_saved_character_at_root(&root, SAVED_AT, |input| {
            input.chosen.class_levels =
                vec![codex::rules_core::character_input::CharacterClassLevel { class_id: "class:rogue".to_owned(), level: 4 }];
        })
        .expect("make Aldric a Rogue 4");
        let talents = |root: &Path| {
            let listed = list_ability_options_at_root(root, "class_features").expect("class features");
            listed.budget.iter().find(|r| r.id == "abilities.budget.class_features.rogue_talent.granted").map_or(0, |r| r.value)
        };
        assert_eq!(talents(&root), 2, "a Rogue 4's rogue talents before any archetype");

        let listed = list_ability_options_at_root(&root, "archetype").expect("archetype options");
        for key in ["archetype:Rogue Archetype ~ Acrobat", "archetype:Rogue Archetype ~ Rake", "archetype:Rogue Archetype ~ Pirate"] {
            assert!(listed.options.iter().any(|o| o.id == key && o.qualified), "{key} offered and qualified");
        }
        assert!(listed.options.iter().all(|o| o.pool == "rogue"), "only the held class's archetypes are offered");
        // Escapologist (Occult Adventures) names no replaced feature; its own converted gate excludes it beside Acrobat.
        const ESCAPOLOGIST: &str = "occult_adventures:class_feature:rogue_archetype_escapologist";
        assert!(listed.options.iter().any(|o| o.id == ESCAPOLOGIST && o.qualified), "Escapologist offered and qualified");
        let cat_burglar = listed.options.iter().find(|o| o.id == "archetype:Rogue Archetype ~ Cat Burglar").expect("Cat Burglar listed");
        assert!(!cat_burglar.qualified && cat_burglar.unmet.iter().any(|r| r.contains("Catfolk")), "a human is no catfolk: {:?}", cat_burglar.unmet);

        add_ability_selection_at_root(&root, "archetype", "archetype:Rogue Archetype ~ Acrobat", SAVED_AT).expect("Acrobat");
        let input = SavedCharacterStore::load(&root).expect("reload").character_input;
        assert!(codex::rules_core::archetype_resolver::archetype_claims_slot(&input, "Rogue", "RogueTrapfinding"), "saved as choice:archetype");
        let listed = list_ability_options_at_root(&root, "archetype").expect("archetype options");
        assert_eq!(listed.selected, ["archetype:Rogue Archetype ~ Acrobat"]);
        let escapologist = listed.options.iter().find(|o| o.id == ESCAPOLOGIST).expect("Escapologist still listed");
        assert!(!escapologist.qualified && escapologist.unmet.iter().any(|r| r.contains("Acrobat")), "{:?}", escapologist.unmet);
        let rake = listed.options.iter().find(|o| o.id == "archetype:Rogue Archetype ~ Rake").expect("Rake still listed");
        assert!(!rake.qualified && rake.unmet.iter().any(|r| r.contains("Trapfinding") && r.contains("Acrobat")), "{:?}", rake.unmet);

        let refusal = add_ability_selection_at_root(&root, "archetype", "archetype:Rogue Archetype ~ Rake", SAVED_AT).expect_err("a second trapfinding archetype");
        assert!(refusal.contains("Trapfinding") && refusal.contains("Acrobat"), "{refusal}");
        let input = SavedCharacterStore::load(&root).expect("reload").character_input;
        let held: Vec<&str> = input.chosen.selected_choices.iter().filter(|c| c.choice_set_id == "choice:archetype").map(|c| c.selection_id.as_str()).collect();
        assert_eq!(held, ["archetype:Rogue Archetype ~ Acrobat"], "the refusal saved nothing");

        add_ability_selection_at_root(&root, "archetype", "archetype:Rogue Archetype ~ Pirate", SAVED_AT)
            .expect_err("Pirate also replaces trapfinding");
        remove_ability_selection_at_root(&root, "archetype", "archetype:Rogue Archetype ~ Acrobat", SAVED_AT).expect("remove Acrobat");
        add_ability_selection_at_root(&root, "archetype", "archetype:Rogue Archetype ~ Pirate", SAVED_AT).expect("Pirate");
        assert_eq!(talents(&root), 1, "Pirate replaces the 2nd-level rogue talent: the pool follows");
        std::fs::remove_dir_all(&root).ok();
    }

    /// Major Magic (Core Rulebook): "A rogue must have the minor magic rogue talent ... before selecting this talent."
    #[test]
    fn abilities_class_features_a_talent_whose_prerequisite_is_another_talent_is_refused_until_that_talent_is_held() {
        let root = aldric_root("class-feature-prereqs");
        mutate_saved_character_at_root(&root, SAVED_AT, |input| {
            input.chosen.class_levels =
                vec![codex::rules_core::character_input::CharacterClassLevel { class_id: "class:rogue".to_owned(), level: 4 }];
        })
        .expect("make Aldric a Rogue 4");
        let listed = list_ability_options_at_root(&root, "class_features").expect("options");
        let id_of = |label: &str| {
            listed.options.iter().find(|o| o.pool == "rogue_talent" && o.label == label).map(|o| o.id.clone()).unwrap_or_else(|| panic!("no {label}"))
        };
        let (minor, major) = (id_of("Minor Magic"), id_of("Major Magic"));
        let refusal = add_ability_selection_at_root(&root, "class_features", &major, SAVED_AT).expect_err("Major Magic before Minor Magic");
        assert!(refusal.contains("Minor Magic"), "{refusal}");

        add_ability_selection_at_root(&root, "class_features", &minor, SAVED_AT).expect("Minor Magic");
        let listed = list_ability_options_at_root(&root, "class_features").expect("options");
        let major = listed.options.iter().find(|o| o.id == major).expect("Major Magic still offered");
        assert!(!major.unmet.iter().any(|reason| reason.contains("Minor Magic")), "{:?}", major.unmet);
        std::fs::remove_dir_all(&root).ok();
    }

    /// The sheet row `id`'s value as the Character Sheet loads it.
    fn sheet_row(root: &Path, id: &str) -> Option<i16> {
        let loaded = crate::character_hub::load_saved_character_at_root(root).expect("the sheet loads");
        loaded.explanations.iter().find(|row| row.id == id).map(|row| row.value)
    }

    const FIGHTER: &str = "favored_class:class:fighter";
    const BONUS_HIT_POINT: &str = "favored_class_bonus:core_rulebook:class_feature:favored_class_bonus_hit_point";
    const BONUS_SKILL_RANK: &str = "favored_class_bonus:core_rulebook:class_feature:favored_class_bonus_skill_rank";

    /// Core Rulebook p.31, Favored Class: "Whenever a character gains a level in his favored class, he
    /// receives either + 1 hit point or + 1 skill rank." Aldric is a Fighter 3: favored Fighter, a hit
    /// point at each of levels 1-3 is +3 on the sheet's maximum hit points; a fourth is refused.
    #[test]
    fn abilities_favored_class_a_fighter_3_takes_a_hit_point_at_levels_1_to_3_and_the_sheet_adds_3() {
        let root = aldric_root("favored-hp");
        let before = sheet_row(&root, "hit_points.max").expect("Aldric's hit points");
        let listed = list_ability_options_at_root(&root, "favored_class").expect("favored class options");
        assert!(listed.options.iter().any(|o| o.id == FIGHTER && o.qualified), "{:?}", listed.options);
        let hit_point = listed.options.iter().find(|o| o.id == BONUS_HIT_POINT).expect("the +1 hit point option");
        assert!(!hit_point.qualified && hit_point.unmet.iter().any(|r| r.contains("favored class")), "{:?}", hit_point.unmet);

        add_ability_selection_at_root(&root, "favored_class", FIGHTER, SAVED_AT).expect("favored Fighter");
        for _ in 0..3 {
            add_ability_selection_at_root(&root, "favored_class", BONUS_HIT_POINT, SAVED_AT).expect("a hit point");
        }
        let refusal = add_ability_selection_at_root(&root, "favored_class", BONUS_HIT_POINT, SAVED_AT).expect_err("a fourth at Fighter 3");
        assert!(refusal.contains("favored_class_bonus pool"), "{refusal}");
        let picks = SavedCharacterStore::load(&root).expect("reload").character_input.chosen.ability_selections;
        let levels: Vec<u8> = picks.iter().filter(|p| p.subtab == "favored_class").map(|p| p.level).collect();
        assert_eq!(levels, [1, 2, 3], "one bonus charged to each Fighter level");
        assert_eq!(sheet_row(&root, "hit_points.max"), Some(before + 3));
        assert_eq!(sheet_row(&root, "favored_class.hit_points"), Some(3));
        std::fs::remove_dir_all(&root).ok();
    }

    /// The same rule's other option: a skill rank at level 2 is +1 to the skill point pool.
    #[test]
    fn abilities_favored_class_a_skill_rank_at_level_2_adds_1_to_the_skill_pool() {
        let root = aldric_root("favored-skill");
        let before = sheet_row(&root, "hit_points.max").expect("Aldric's hit points");
        assert_eq!(sheet_row(&root, "favored_class.skill_ranks"), Some(0), "no favored class: no extra rank");
        add_ability_selection_at_root(&root, "favored_class", FIGHTER, SAVED_AT).expect("favored Fighter");
        add_ability_selection_at_root(&root, "favored_class", BONUS_HIT_POINT, SAVED_AT).expect("level 1: a hit point");
        add_ability_selection_at_root(&root, "favored_class", BONUS_SKILL_RANK, SAVED_AT).expect("level 2: a skill rank");
        let picks = SavedCharacterStore::load(&root).expect("reload").character_input.chosen.ability_selections;
        let taken: Vec<(String, u8)> = picks.iter().filter(|p| p.subtab == "favored_class").map(|p| (p.record_key.clone(), p.level)).collect();
        assert_eq!(
            taken,
            [
                ("core_rulebook:class_feature:favored_class_bonus_hit_point".to_owned(), 1),
                ("core_rulebook:class_feature:favored_class_bonus_skill_rank".to_owned(), 2)
            ]
        );
        assert_eq!(sheet_row(&root, "favored_class.skill_ranks"), Some(1), "the skill pool's +1");
        assert_eq!(sheet_row(&root, "hit_points.max"), Some(before + 1));
        let listed = list_ability_options_at_root(&root, "favored_class").expect("options");
        assert_eq!(listed.selected.len(), 3, "the class and two bonuses: {:?}", listed.selected);
        let skill_pick = listed.selected.iter().find(|id| id.starts_with(BONUS_SKILL_RANK)).expect("the skill pick").clone();
        remove_ability_selection_at_root(&root, "favored_class", &skill_pick, SAVED_AT).expect("remove the skill rank");
        assert_eq!(sheet_row(&root, "favored_class.skill_ranks"), Some(0));
        std::fs::remove_dir_all(&root).ok();
    }

    /// A PCGen import carries its favored class bonus as Custom grants (`pcg_import/map.rs`
    /// `add_favored_class_bonus`): three hit points are +3 once, and they spend the favored class pool,
    /// so the sheet cannot take the same three levels' bonuses again.
    #[test]
    fn abilities_favored_class_an_imported_bonus_is_not_counted_twice() {
        let dir = std::env::temp_dir().join(format!("codex-abilities-favored-import-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("temp dir");
        let pcg = dir.join("fighter.pcg");
        let mut text = String::from("PCGVERSION:2.0\nCHARACTERNAME:Favored\nGAMEMODE:Pathfinder_RPG\nRACE:Human\nCLASS:Fighter|LEVEL:3\n");
        text.push_str("STAT:STR|SCORE:16\nSTAT:DEX|SCORE:12\nSTAT:CON|SCORE:14\nSTAT:INT|SCORE:10\nSTAT:WIS|SCORE:10\nSTAT:CHA|SCORE:8\n");
        text.push_str("ABILITY:Favored Class Bonus|TYPE:NORMAL|CATEGORY:Special Ability|KEY:Favored Class Bonus ~ Hit Point|APPLIEDTO:,,|TYPE:FavoredClassBonus\n");
        std::fs::write(&pcg, text).expect("write the file");
        let out = dir.join("out.json");
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
        codex_ingest::pcg_import::run(&pcg, &out, &repo).expect("the converter reads the file");
        let root = dir.join("character");
        let json = std::fs::read_to_string(&out).expect("the converter's JSON");
        crate::character_hub::import_pcg_json(&json, &root, "favored-import", SAVED_AT, "0.0.0-test").expect("the import runs");
        let custom = crate::character_custom::load_character_custom_at_root(&root).expect("custom.json");
        let imported: Vec<(&str, i32)> = custom.grants.iter().map(|g| (g.target.as_str(), g.value)).collect();
        assert_eq!(imported, [("hit_points", 3)], "the import's own grant");
        let with_grant = sheet_row(&root, "hit_points.max").expect("hit points");

        add_ability_selection_at_root(&root, "favored_class", FIGHTER, SAVED_AT).expect("favored Fighter");
        let listed = list_ability_options_at_root(&root, "favored_class").expect("options");
        let row = |field: &str| listed.budget.iter().find(|r| r.id == format!("abilities.budget.favored_class.favored_class_bonus.{field}")).map(|r| r.value);
        assert_eq!((row("granted"), row("spent"), row("remaining")), (Some(3), Some(3), Some(0)), "{:?}", listed.budget);
        let refusal = add_ability_selection_at_root(&root, "favored_class", BONUS_HIT_POINT, SAVED_AT).expect_err("the import already took levels 1-3");
        assert!(refusal.contains("favored_class_bonus pool"), "{refusal}");
        assert_eq!(sheet_row(&root, "hit_points.max"), Some(with_grant), "the imported +3 counted once");
        std::fs::remove_dir_all(&dir).ok();
    }
}
