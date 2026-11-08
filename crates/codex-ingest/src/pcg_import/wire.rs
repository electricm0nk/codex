//! The JSON `pcg_import` writes. `displayLabel` and `characterInput` are the file `import_character`
//! reads, field for field; serde ignores the rest there. `bio`, `money` and `custom` are the shapes of
//! the character's own `bio.json`, `money.json` and `custom.json`, and `report` is shown to the player.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportFile {
    pub display_label: String,
    pub character_input: CharacterInputWire,
    pub bio: BioWire,
    pub money: MoneyWire,
    pub custom: CustomWire,
    pub report: Report,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterInputWire {
    pub source_package_id: String,
    pub chosen: ChosenWire,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChosenWire {
    pub race_id: String,
    pub class_levels: Vec<ClassLevelWire>,
    pub ability_scores: AbilitiesWire,
    pub selected_feats: Vec<String>,
    pub skill_allocations: Vec<SkillWire>,
    pub equipment_selections: Vec<EquipmentWire>,
    pub selected_choices: Vec<ChoiceWire>,
    pub selected_traits: Vec<String>,
    pub spells_selected: Vec<SpellWire>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassLevelWire {
    pub class_id: String,
    pub level: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilitiesWire {
    pub strength: i16,
    pub dexterity: i16,
    pub constitution: i16,
    pub intelligence: i16,
    pub wisdom: i16,
    pub charisma: i16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillWire {
    pub skill_id: String,
    pub ranks: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentWire {
    pub item_id: String,
    /// `EquippedActive` (worn or wielded), `SelectedInactive` (carried), or `Absent`.
    pub active_state: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applied_modifiers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChoiceWire {
    pub choice_set_id: String,
    pub selection_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpellWire {
    pub spell_id: String,
    pub source_class_id: String,
    /// `Known`, `Prepared` or `Granted`.
    pub acquisition_mode: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BioWire {
    pub player_name: String,
    pub alignment: String,
    pub deity: String,
    pub sex: String,
    pub age: String,
    pub height: String,
    pub weight: String,
    pub hair: String,
    pub eyes: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoneyWire {
    pub total_copper: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomWire {
    pub feats: Vec<CustomRecord>,
    pub equipment: Vec<CustomRecord>,
    pub spells: Vec<CustomRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomRecord {
    pub id: String,
    pub name: String,
    pub description: String,
    pub stats: Vec<CustomStat>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomStat {
    pub label: String,
    pub value: String,
}

/// What became of one line of the file, or of one row the importer had to add.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Outcome {
    /// Read into a character field; `to` names it.
    Mapped,
    /// Codex holds no such thing, so it is a custom record, listed and printed but not computed from.
    Custom,
    /// Codex works this out from class and race, so the file's own number is not read.
    Derived,
    /// How PCGen was set up. It says nothing about the character.
    Setting,
    /// In no line of the file: a row the rules engine needs for this character to compute.
    Added,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportLine {
    pub key: String,
    pub from: String,
    pub outcome: Outcome,
    pub to: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub source_file: String,
    pub game_mode: String,
    /// Keys the mapping table has no row for. Each such line is a custom record.
    pub unknown_keys: Vec<String>,
    /// Classes with no Codex record. An import of nothing but these does not compute.
    pub unmapped_classes: Vec<String>,
    /// The file's race, when Codex has no record of it. Such an import does not compute either.
    pub unmapped_race: Option<String>,
    pub warnings: Vec<String>,
    pub lines: Vec<ReportLine>,
}
