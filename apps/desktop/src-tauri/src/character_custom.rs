//! Custom: the GM's grants and house-rule records for one character, saved as `custom.json` beside
//! the character (the same sidecar precedent as `hit_points.json` and `bio.json`).
//!
//! Two kinds of entry, honestly different:
//!
//! * **Grants** change numbers. An ability grant (`ability:wisdom`, +1) is applied to the saved
//!   ability score itself, the same place a racial adjustment already lives, so AC, saves, attacks,
//!   skills and hit points all follow from the engine; no second, display-only "twin" is kept.
//!   Editing or removing a grant applies only the difference. `hit_points` and `skill_points`
//!   grants are read by the sheet (the hit point total and the skill point pool).
//! * **Records** are things the rules corpus does not have: a custom feat, piece of equipment,
//!   spell or magic device, each a name, a description and free-form stat lines. They are listed on
//!   the sheet and printed; the engine does not compute from them.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::Path;

use crate::character_hub::CreateCharacterResponse;
use codex::rules_core::character_input::AbilityScores;
use codex::saved_character::local_store::SavedCharacterStore;

const CUSTOM_FILE_NAME: &str = "custom.json";
const MAX_ENTRIES: usize = 200;
const MAX_NAME: usize = 120;
const MAX_DESCRIPTION: usize = 10_000;
const MAX_STATS: usize = 40;
const MAX_STAT_TEXT: usize = 400;
const MAX_GRANT_VALUE: i32 = 12;
const ABILITIES: [&str; 6] = ["strength", "dexterity", "constitution", "intelligence", "wisdom", "charisma"];

/// One number the GM changed: `target` is `ability:<name>`, `hit_points` or `skill_points`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomGrantDto {
    pub id: String,
    pub label: String,
    pub target: String,
    pub value: i32,
    /// Why (the quest, the god, the house rule), shown with the grant.
    #[serde(default)]
    pub reason: String,
}

/// One line of a custom record's stats ("Damage" / "1d8").
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomStatDto {
    pub label: String,
    pub value: String,
}

/// A custom feat, item, spell or device: a raw object the GM gives stats and a description.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomRecordDto {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub stats: Vec<CustomStatDto>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterCustomDto {
    #[serde(default)]
    pub grants: Vec<CustomGrantDto>,
    #[serde(default)]
    pub feats: Vec<CustomRecordDto>,
    #[serde(default)]
    pub equipment: Vec<CustomRecordDto>,
    #[serde(default)]
    pub spells: Vec<CustomRecordDto>,
    #[serde(default)]
    pub devices: Vec<CustomRecordDto>,
}

fn validate_records(kind: &str, records: &[CustomRecordDto], seen: &mut BTreeSet<String>) -> Result<(), String> {
    if records.len() > MAX_ENTRIES {
        return Err(format!("too many custom {kind} (at most {MAX_ENTRIES})"));
    }
    for record in records {
        if record.id.trim().is_empty() || !seen.insert(record.id.clone()) {
            return Err(format!("a custom {kind} entry has a missing or duplicate id ({:?})", record.id));
        }
        if record.name.trim().is_empty() {
            return Err(format!("a custom {kind} entry needs a name"));
        }
        if record.name.chars().count() > MAX_NAME {
            return Err(format!("the custom {kind} {:?} has a name over {MAX_NAME} characters", record.name));
        }
        if record.description.chars().count() > MAX_DESCRIPTION {
            return Err(format!("the description of {:?} is over {MAX_DESCRIPTION} characters", record.name));
        }
        if record.stats.len() > MAX_STATS {
            return Err(format!("{:?} has more than {MAX_STATS} stat lines", record.name));
        }
        for stat in &record.stats {
            if stat.label.trim().is_empty() || stat.value.trim().is_empty() {
                return Err(format!("{:?} has a stat line with no label or no value", record.name));
            }
            if stat.label.chars().count() > MAX_STAT_TEXT || stat.value.chars().count() > MAX_STAT_TEXT {
                return Err(format!("{:?} has a stat line over {MAX_STAT_TEXT} characters", record.name));
            }
        }
    }
    Ok(())
}

pub(crate) fn validate_custom(custom: &CharacterCustomDto) -> Result<(), String> {
    let mut seen = BTreeSet::new();
    if custom.grants.len() > MAX_ENTRIES {
        return Err(format!("too many grants (at most {MAX_ENTRIES})"));
    }
    for grant in &custom.grants {
        if grant.id.trim().is_empty() || !seen.insert(grant.id.clone()) {
            return Err(format!("a grant has a missing or duplicate id ({:?})", grant.id));
        }
        let target_ok = match grant.target.strip_prefix("ability:") {
            Some(name) => ABILITIES.contains(&name),
            None => grant.target == "hit_points" || grant.target == "skill_points",
        };
        if !target_ok {
            return Err(format!(
                "the grant {:?} has an unknown target {:?} (use ability:<name>, hit_points or skill_points)",
                grant.label, grant.target
            ));
        }
        if grant.value.abs() > MAX_GRANT_VALUE {
            return Err(format!("the grant {:?} has value {}, outside -{MAX_GRANT_VALUE}..={MAX_GRANT_VALUE}", grant.label, grant.value));
        }
        if grant.label.trim().is_empty() || grant.label.chars().count() > MAX_NAME || grant.reason.chars().count() > MAX_DESCRIPTION {
            return Err(format!("the grant {:?} needs a label of at most {MAX_NAME} characters and a shorter reason", grant.label));
        }
    }
    validate_records("feat", &custom.feats, &mut seen)?;
    validate_records("equipment", &custom.equipment, &mut seen)?;
    validate_records("spell", &custom.spells, &mut seen)?;
    validate_records("device", &custom.devices, &mut seen)?;
    Ok(())
}

/// The total ability-score change the grants make, in `ABILITIES` order.
fn ability_totals(custom: &CharacterCustomDto) -> [i32; 6] {
    let mut totals = [0; 6];
    for grant in &custom.grants {
        if let Some(name) = grant.target.strip_prefix("ability:") {
            if let Some(slot) = ABILITIES.iter().position(|ability| *ability == name) {
                totals[slot] += grant.value;
            }
        }
    }
    totals
}

fn score_mut(scores: &mut AbilityScores, slot: usize) -> &mut i16 {
    match slot {
        0 => &mut scores.strength,
        1 => &mut scores.dexterity,
        2 => &mut scores.constitution,
        3 => &mut scores.intelligence,
        4 => &mut scores.wisdom,
        _ => &mut scores.charisma,
    }
}

pub(crate) fn load_character_custom_at_root(root: &Path) -> Result<CharacterCustomDto, String> {
    let path = root.join(CUSTOM_FILE_NAME);
    if !path.exists() {
        return Ok(CharacterCustomDto::default());
    }
    let contents = std::fs::read_to_string(&path).map_err(|err| format!("{}: {err}", path.display()))?;
    serde_json::from_str(&contents).map_err(|err| format!("{}: invalid custom JSON: {err}", path.display()))
}

/// Validates `custom`, applies the change in ability-grant totals to the saved ability scores (a
/// new revision, only when something changed), then writes `custom.json`. Nothing is written when
/// validation fails or the character would no longer compute.
pub(crate) fn save_character_custom_at_root(root: &Path, custom: &CharacterCustomDto, saved_at: &str) -> Result<(), String> {
    validate_custom(custom)?;
    let existing = SavedCharacterStore::load(root).map_err(|err| err.message)?;
    let previous = load_character_custom_at_root(root)?;
    let (before, after) = (ability_totals(&previous), ability_totals(custom));
    let delta: Vec<i32> = (0..6).map(|slot| after[slot] - before[slot]).collect();

    if delta.iter().any(|change| *change != 0) {
        let mut scores = existing.character_input.chosen.ability_scores.clone();
        for (slot, change) in delta.iter().enumerate() {
            let next = i32::from(*score_mut(&mut scores, slot)) + change;
            if !(1..=60).contains(&next) {
                return Err(format!("the grants would make {} {next}, outside 1..=60", ABILITIES[slot]));
            }
        }
        let outcome = crate::pf1_adapter::mutate_saved_character_at_root(root, saved_at, |input| {
            for (slot, change) in delta.iter().enumerate() {
                let score = score_mut(&mut input.chosen.ability_scores, slot);
                *score = (i32::from(*score) + change) as i16;
            }
        })?;
        if let CreateCharacterResponse::Blocked { diagnostics } = outcome {
            return Err(format!(
                "the character no longer computes with these grants: {}",
                diagnostics.into_iter().map(|d| d.message).collect::<Vec<_>>().join("; ")
            ));
        }
    }

    let path = root.join(CUSTOM_FILE_NAME);
    let json = serde_json::to_string_pretty(custom).map_err(|err| format!("failed to serialize custom data: {err}"))?;
    std::fs::write(&path, json).map_err(|err| format!("{}: {err}", path.display()))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterCustomRequest {
    pub character_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveCharacterCustomRequest {
    pub character_id: String,
    pub custom: CharacterCustomDto,
    pub saved_at: String,
}

#[tauri::command]
pub fn load_character_custom(app: tauri::AppHandle, request: CharacterCustomRequest) -> Result<CharacterCustomDto, String> {
    let root = crate::character_hub::resolve_character_root(&app, &request.character_id)?;
    load_character_custom_at_root(&root)
}

#[tauri::command]
pub fn save_character_custom(app: tauri::AppHandle, request: SaveCharacterCustomRequest) -> Result<CharacterCustomDto, String> {
    let root = crate::character_hub::resolve_character_root(&app, &request.character_id)?;
    save_character_custom_at_root(&root, &request.custom, &request.saved_at)?;
    Ok(request.custom)
}
