//! The Abilities sub-tabs' commands (SD-43 E3.2, `technical-design.md` §3.2): list a sub-tab's
//! options with `qualified` and the unmet reasons, add a pick, remove one. The budget and the
//! prerequisites are the engine's (`codex::rules_core::ability_budget`); this file loads the saved
//! character, asks the engine, and saves through the same recompute-before-persist gate every add
//! path uses. Feats keep their own commands.

use std::path::Path;

use codex::rules_core::ability_budget::{ability_budget, ability_options, AbilityOption, SubTab};
use codex::rules_core::game_system::GameSystem;
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::saved_character::local_store::SavedCharacterStore;
use serde::{Deserialize, Serialize};

use crate::character_hub::{
    apply_add_trait_selection, map_explanations_dto, mutate_saved_character_at_root, remove_trait_selection_at_root,
    resolve_character_root, sheet_rule_package_for, CreateCharacterResponse, ExplanationDto,
};

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
        }
    }
}

/// A sub-tab's options and its `abilities.budget.<subtab>.*` rows.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityOptionsDto {
    pub budget: Vec<ExplanationDto>,
    pub options: Vec<AbilityOptionDto>,
}

#[tauri::command]
pub fn list_ability_options(app: tauri::AppHandle, request: ListAbilityOptionsRequest) -> Result<AbilityOptionsDto, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    list_ability_options_at_root(&root, &request.subtab)
}

pub(crate) fn list_ability_options_at_root(root: &Path, subtab: &str) -> Result<AbilityOptionsDto, String> {
    let subtab = subtab_of(subtab)?;
    let package = package()?;
    let input = SavedCharacterStore::load(root).map_err(|err| err.message)?.character_input;
    let base = compute_pilot_base_chassis(&input);
    Ok(AbilityOptionsDto {
        budget: map_explanations_dto(&ability_budget(package, &input, subtab).explanation_rows()),
        options: ability_options(package, &input, &base, subtab).into_iter().map(AbilityOptionDto::from).collect(),
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
    let subtab = subtab_of(subtab)?;
    let package = package()?;
    let input = SavedCharacterStore::load(root).map_err(|err| err.message)?.character_input;
    if let Some(refusal) = ability_budget(package, &input, subtab).refusal(package, option_id) {
        return Err(refusal);
    }
    let base = compute_pilot_base_chassis(&input);
    let option = ability_options(package, &input, &base, subtab)
        .into_iter()
        .find(|option| option.id == option_id)
        .ok_or_else(|| format!("'{option_id}' is not an option of the {} sub-tab", subtab.id()))?;
    if !option.qualified {
        return Err(format!("'{}' cannot be taken by this character: {}", option.label, option.unmet.join("; ")));
    }
    match subtab {
        SubTab::Traits => mutate_saved_character_at_root(root, saved_at, |input| apply_add_trait_selection(input, option_id, None)),
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
    match subtab_of(subtab)? {
        SubTab::Traits => remove_trait_selection_at_root(root, option_id, saved_at),
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
        assert!(list_ability_options_at_root(&root, "background").is_err(), "a sub-tab with no engine budget is refused");
        std::fs::remove_dir_all(&root).ok();
    }
}
