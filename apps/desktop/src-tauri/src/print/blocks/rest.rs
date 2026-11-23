//! Rest and recovery (SD-43 E2.4, `decisions.md §8`): one source, two surfaces. [`rest_text`] picks
//! the character's rows from the engine's table (`codex::rules_core::rest_text`) or, when the
//! `custom.json` override is set, the GM's words instead (whole-text replace, §30 Q-2.2). The print
//! block after the Bio page and the `load_rest_text` command the Biography tab reads both call it.
//! Every row prints as text; nothing here computes a number.

use std::path::Path;

use codex::rules_core::rest_text::{rest_rows_for, rest_table, RestRow};
use serde::{Deserialize, Serialize};

use super::SheetSources;
use crate::character_custom::{custom_or_empty, CharacterCustomDto};
use crate::character_hub::{resolve_character_root, LoadSavedCharacterResponse};
use crate::print::model::{SheetBlock, SheetRow};
use crate::rule_system_adapter::load_saved_character_via_envelope;

/// What the character's rest and recovery text is: the table's rows, the GM's override, or the
/// sentence naming what the table lacks for this game system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestTextDto {
    pub rows: Vec<RestRow>,
    /// The `custom.json` override; set, it replaces `rows` on both surfaces.
    pub override_text: Option<String>,
    /// No row and no override: what the table lacks, as a sentence.
    pub gap: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestTextRequest {
    pub character_id: String,
}

/// The rest and recovery text for a loaded character and its Custom record.
pub fn rest_text(loaded: &LoadSavedCharacterResponse, custom: &CharacterCustomDto) -> Result<RestTextDto, String> {
    let override_text = custom.rest_text.clone().filter(|text| !text.trim().is_empty());
    if override_text.is_some() {
        return Ok(RestTextDto { rows: Vec::new(), override_text, gap: None });
    }
    let table = rest_table().as_ref().map_err(Clone::clone)?;
    // `class:wizard:5, class:rogue:1` holds `wizard` and `rogue`; `race:elf` is `elf`.
    let classes: Vec<&str> = loaded
        .summary
        .class_summary
        .split(',')
        .map(str::trim)
        .filter_map(|held| held.strip_prefix("class:").map(|rest| rest.rsplit_once(':').map_or(rest, |(slug, _)| slug)))
        .collect();
    let race = loaded.summary.race_id.strip_prefix("race:").unwrap_or(&loaded.summary.race_id);
    let system = loaded.summary.game_system.as_str();
    let rows: Vec<RestRow> = rest_rows_for(table, system, &classes, race, &loaded.selected_alternate_trait_keys).into_iter().cloned().collect();
    let gap = rows.is_empty().then(|| {
        let name = match system {
            "pf1" => "Pathfinder",
            crate::sf_adapter::STARFINDER_RULE_SYSTEM_ID => "Starfinder",
            _ => "Cyberpunk RED",
        };
        format!("The rest and recovery table holds no {name} rule: its rows are cited from the Pathfinder books, and the {name} corpus holds no rest rule record.")
    });
    Ok(RestTextDto { rows, override_text: None, gap })
}

/// [`rest_text`] for the saved character at `root`, its Custom record read as the print reads it
/// (Pathfinder only; the other sheets have no Custom record).
pub fn rest_text_at_root(root: &Path) -> Result<RestTextDto, String> {
    let loaded = load_saved_character_via_envelope(root)?;
    let custom = if loaded.summary.game_system == "pf1" { custom_or_empty(root) } else { CharacterCustomDto::default() };
    rest_text(&loaded, &custom)
}

/// The Biography tab's read (E2.5): the same text the print block prints.
#[tauri::command]
pub fn load_rest_text(app: tauri::AppHandle, request: RestTextRequest) -> Result<RestTextDto, String> {
    rest_text_at_root(&resolve_character_root(&app, &request.character_id)?)
}

/// The Rest and Recovery block: each rule, its words and its book and page; or the GM's words; or
/// the sentence naming what the table lacks, or why it could not be read.
pub fn rest(sources: &SheetSources) -> SheetBlock {
    let rows = match rest_text(sources.loaded, &sources.custom) {
        Ok(RestTextDto { override_text: Some(words), .. }) => {
            vec![SheetRow::new("The GM's rest and recovery rules", Vec::<String>::new()).with_notes(words.lines().map(str::to_owned))]
        }
        Ok(RestTextDto { gap: Some(gap), .. }) => vec![SheetRow::new(gap, Vec::<String>::new())],
        Ok(RestTextDto { rows, .. }) => rows
            .into_iter()
            .map(|row| SheetRow::new(row.rule, Vec::<String>::new()).with_notes([row.text, format!("{}, {}", row.book, row.page)]))
            .collect(),
        Err(reason) => vec![SheetRow::new(reason, Vec::<String>::new())],
    };
    SheetBlock::new("Rest and Recovery", rows)
}
