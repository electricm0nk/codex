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
//!   the sheet and printed.
//! * **Custom equipment** may also state typed `bonuses` and a body `slot`. Worn, it applies
//!   ([`WornItemBonuses`]) and holds its slot against stock items too; carried, it adds nothing. An
//!   ability bonus goes in at the score on the copy of the input the sheet computes from, never the
//!   saved one, so what the score drives follows from the engine. Its other numbers move the totals
//!   the engine returned, by the same stacking rule as the `Var` fold.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::Path;

use crate::character_hub::{
    worn_stock_slots, CreateCharacterResponse, DiagnosticDto, ExplanationDto, LoadSavedCharacterResponse, ResolvedEquipmentEffectDto,
    WeaponFeatEffectDto,
};
use codex::rules_core::body_slots::{slot_conflict, BodySlot};
use codex::rules_core::character_input::{AbilityScores, ActiveState, CharacterInput};
use codex::rules_core::pilot_view_model::PilotSnapshot;
use codex::rules_core::game_system::GameSystem;
use codex::rules_core::sheet_rule::stacking_types;
use codex::rules_core::skill_allocation::SHEET_SKILL_IDS;
use codex::saved_character::local_store::SavedCharacterStore;

const CUSTOM_FILE_NAME: &str = "custom.json";
const MAX_ENTRIES: usize = 200;
const MAX_NAME: usize = 120;
const MAX_DESCRIPTION: usize = 10_000;
const MAX_STATS: usize = 40;
const MAX_STAT_TEXT: usize = 400;
const MAX_GRANT_VALUE: i32 = 12;
const ABILITIES: [&str; 6] = ["strength", "dexterity", "constitution", "intelligence", "wisdom", "charisma"];
const SAVES: [&str; 3] = ["fortitude", "reflex", "will"];
/// The sheet's three skill rows that name no subtype, so they are not in `SHEET_SKILL_IDS`.
const SHEET_FAMILY_SKILLS: [&str; 3] = ["craft", "perform", "profession"];
const FLAT_TARGETS: [&str; 8] = ["attack", "damage", "ac", "speed", "initiative", "hit_points", "cmb", "cmd"];
const MAX_BONUS_VALUE: i32 = 100;
const UNTYPED: &str = "Untyped";
/// The engine's words for a bonus's type (`SheetRule` `BonusType.name`), and `Untyped`.
const BONUS_TYPES: [&str; 18] = [
    UNTYPED, "Alchemical", "Armor", "Circumstance", "Competence", "Deflection", "Dodge", "Enhancement", "Insight", "Luck",
    "Morale", "NaturalArmor", "Profane", "Racial", "Resistance", "Sacred", "Shield", "Size",
];
/// What a touch attack ignores. An enhancement bonus to AC is one on armor or a shield.
const NOT_AGAINST_TOUCH: [&str; 4] = ["Armor", "Shield", "NaturalArmor", "Enhancement"];
/// The prefix naming a custom item to `set_equipment_active_state`, as `list_held_equipment` lists it.
pub(crate) const CUSTOM_ITEM_PREFIX: &str = "custom:";

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

/// One number a custom item states. `target` is a name in [`bonus_target_is_known`]; `bonus_type` is
/// one of [`BONUS_TYPES`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomBonusDto {
    pub target: String,
    pub bonus_type: String,
    pub value: i32,
}

/// A custom feat, item, spell or device: a raw object the GM gives stats and a description. Custom
/// equipment may also state `bonuses` and a `slot`, and is `worn` or carried; a record with none of
/// the three is saved as the text record it always was.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomRecordDto {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub stats: Vec<CustomStatDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bonuses: Vec<CustomBonusDto>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slot: Option<BodySlot>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub worn: bool,
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
    /// Brief 2 E2: the conditions the character is under (`"Shaken"`), shown on the sheet's
    /// Conditions box (`Normal` when empty). `#[serde(default)]` so a `custom.json` saved before
    /// this field existed loads with none.
    #[serde(default)]
    pub conditions: Vec<String>,
}

/// The targets a custom item's bonus may name (`decisions.md §13`). `ability:<name>` and
/// `hit_points` are the grants' words for the same numbers.
pub(crate) fn bonus_target_is_known(target: &str) -> bool {
    match target.split_once(':') {
        Some(("ability", name)) => ABILITIES.contains(&name),
        Some(("save", name)) => SAVES.contains(&name),
        Some(("skill", name)) => {
            SHEET_FAMILY_SKILLS.contains(&name) || SHEET_SKILL_IDS.iter().any(|id| id.strip_prefix("skill:") == Some(name))
        }
        Some(_) => false,
        None => FLAT_TARGETS.contains(&target),
    }
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
        if kind != "equipment" && (!record.bonuses.is_empty() || record.slot.is_some() || record.worn) {
            return Err(format!("{:?}: only custom equipment can state bonuses, take a slot or be worn", record.name));
        }
        if record.bonuses.len() > MAX_STATS {
            return Err(format!("{:?} has more than {MAX_STATS} bonuses", record.name));
        }
        for bonus in &record.bonuses {
            if !bonus_target_is_known(&bonus.target) {
                return Err(format!("{:?} has a bonus with an unknown target {:?}", record.name, bonus.target));
            }
            if !BONUS_TYPES.contains(&bonus.bonus_type.as_str()) {
                return Err(format!("{:?} has a bonus with an unknown bonus type {:?}", record.name, bonus.bonus_type));
            }
            if bonus.value == 0 || bonus.value.abs() > MAX_BONUS_VALUE {
                return Err(format!("{:?} has a bonus with value {}, which must be from -{MAX_BONUS_VALUE} to {MAX_BONUS_VALUE} and not 0", record.name, bonus.value));
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

/// One bonus of a worn custom item that counts after stacking.
#[derive(Debug, Clone, PartialEq, Eq)]
struct WornBonus {
    item_id: String,
    item_name: String,
    target: String,
    bonus_type: String,
    value: i16,
}

/// What the character's worn custom items add. A carried item is not in it. Same-type bonuses to one
/// target take the higher, except a penalty, an untyped bonus and the types the game mode says stack
/// ([`stacking_types`]), the rule of the sheet engine's `Var` fold.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct WornItemBonuses(Vec<WornBonus>);

impl WornItemBonuses {
    pub(crate) fn of(custom: &CharacterCustomDto) -> Self {
        let stacking = stacking_types(GameSystem::Pathfinder1e);
        let mut counted: Vec<WornBonus> = Vec::new();
        for record in custom.equipment.iter().filter(|record| record.worn) {
            for bonus in &record.bonuses {
                let value = bonus.value.clamp(-MAX_BONUS_VALUE, MAX_BONUS_VALUE) as i16;
                let row = WornBonus {
                    item_id: record.id.clone(),
                    item_name: record.name.clone(),
                    target: bonus.target.clone(),
                    bonus_type: bonus.bonus_type.clone(),
                    value,
                };
                let always_counts = value < 0 || bonus.bonus_type == UNTYPED || stacking.contains(&bonus.bonus_type.as_str());
                let same_type = counted
                    .iter()
                    .position(|held| held.value > 0 && held.target == row.target && held.bonus_type == row.bonus_type);
                match same_type {
                    Some(index) if !always_counts => {
                        if value > counted[index].value {
                            counted[index] = row;
                        }
                    }
                    _ => counted.push(row),
                }
            }
        }
        Self(counted)
    }

    /// The bonuses of the file beside this character. An unreadable `custom.json` adds nothing here,
    /// so the sheet still opens; the Custom tab reads the same file and says what is wrong with it.
    pub(crate) fn at_root(root: &Path) -> Self {
        Self::of(&custom_or_empty(root))
    }

    fn sum(&self, target: &str, counts: impl Fn(&WornBonus) -> bool) -> i16 {
        self.0.iter().filter(|bonus| bonus.target == target && counts(bonus)).map(|bonus| bonus.value).sum()
    }

    fn total(&self, target: &str) -> i16 {
        self.sum(target, |_| true)
    }

    /// `+1 Deflection (Ring of Hats), -1 Untyped (Curse)`, for a row's detail.
    fn sources(&self, target: &str) -> String {
        let words: Vec<String> = self
            .0
            .iter()
            .filter(|bonus| bonus.target == target)
            .map(|bonus| format!("{:+} {} ({})", bonus.value, bonus.bonus_type, bonus.item_name))
            .collect();
        words.join(", ")
    }

    /// Raises the scores on `input`, which must be the copy the sheet computes from, not one that is
    /// saved.
    pub(crate) fn raise_abilities(&self, input: &mut CharacterInput) {
        for (slot, name) in ABILITIES.iter().enumerate() {
            *score_mut(&mut input.chosen.ability_scores, slot) += self.total(&format!("ability:{name}"));
        }
    }

    /// The tiles `recompute_character` and the load both read off the snapshot.
    pub(crate) fn apply_to_snapshot(&self, snapshot: &mut PilotSnapshot) {
        snapshot.combat.baseline_melee_attack_bonus += self.total("attack");
        snapshot.defense.baseline_armor_class += self.total("ac");
        snapshot.defense.total_save.fortitude += self.total("save:fortitude");
        snapshot.defense.total_save.reflex += self.total("save:reflex");
        snapshot.defense.total_save.will += self.total("save:will");
    }

    /// Every other number the sheet reads off the load: the rows behind touch, flat-footed, CMB,
    /// CMD, initiative and each skill, the Weapons tab's damage, `AC by Source`, and a
    /// `custom_item.<target>` row per target saying what moved it. The sheet adds `custom_item.speed`
    /// and `custom_item.hit_points` to the two totals it holds itself.
    pub(crate) fn apply_to_sheet(&self, sheet: &mut LoadSavedCharacterResponse) {
        if self.0.is_empty() {
            return;
        }
        let touch = self.sum("ac", |bonus| !NOT_AGAINST_TOUCH.contains(&bonus.bonus_type.as_str()));
        let flat_footed = self.sum("ac", |bonus| bonus.bonus_type != "Dodge");
        let rows = [
            ("defense.touch_armor_class", "ac", touch),
            ("defense.flat_footed_armor_class", "ac", flat_footed),
            ("combat.combat_maneuver_bonus", "cmb", self.total("cmb")),
            ("defense.combat_maneuver_defense", "cmd", self.total("cmd")),
            ("initiative.total", "initiative", self.total("initiative")),
        ];
        for (id, target, moved) in rows {
            if let Some(row) = sheet.explanations.iter_mut().find(|row| row.id == id) {
                row.value += moved;
                row.detail = format!("{}; worn custom items: {}", row.detail, self.sources(target));
            }
        }

        let mut targets: Vec<&str> = self.0.iter().map(|bonus| bonus.target.as_str()).collect();
        targets.sort_unstable();
        targets.dedup();
        for target in targets {
            let sources = self.sources(target);
            if let Some(skill) = target.strip_prefix("skill:") {
                let id = format!("skill.trait_modifier.{skill}");
                match sheet.explanations.iter_mut().find(|row| row.id == id) {
                    Some(row) => {
                        row.value += self.total(target);
                        row.detail = format!("{}; worn custom items: {sources}", row.detail);
                    }
                    None => sheet.explanations.push(ExplanationDto { id, value: self.total(target), detail: format!("Worn custom items: {sources}") }),
                }
            }
            sheet.explanations.push(ExplanationDto {
                id: format!("custom_item.{}", target.replace(':', ".")),
                value: self.total(target),
                detail: format!("Worn custom items: {sources}"),
            });
        }

        let effects = &mut sheet.corpus_derived.equipment_effects;
        effects.armor_class_delta += self.total("ac");
        for bonus in self.0.iter().filter(|bonus| bonus.target == "ac") {
            let item_id = format!("{CUSTOM_ITEM_PREFIX}{}", bonus.item_id);
            match effects.per_item.iter_mut().find(|row| row.item_id == item_id) {
                Some(row) => row.armor_class_bonus = Some(row.armor_class_bonus.unwrap_or(0) + bonus.value),
                None => effects.per_item.push(ResolvedEquipmentEffectDto {
                    item_id,
                    equipment_record_key: bonus.item_name.clone(),
                    category: "custom".to_owned(),
                    armor_class_bonus: Some(bonus.value),
                    armor_class_enhancement: None,
                    max_dex: None,
                    spell_failure: None,
                    armor_check_penalty: None,
                    spell_resistance_bonus: None,
                }),
            }
        }
        for weapon in &mut sheet.weapon_damage {
            for bonus in self.0.iter().filter(|bonus| bonus.target == "damage") {
                weapon.feat_effects.push(WeaponFeatEffectDto { feat_key: bonus.item_name.clone(), damage_bonus: bonus.value });
            }
        }
    }
}

/// The worn custom items that hold a slot, as `(name, slot)`.
pub(crate) fn worn_custom_slots(custom: &CharacterCustomDto) -> Vec<(String, BodySlot)> {
    custom
        .equipment
        .iter()
        .filter(|record| record.worn)
        .filter_map(|record| record.slot.filter(|slot| *slot != BodySlot::None).map(|slot| (record.name.clone(), slot)))
        .collect()
}

/// The file beside the character, or nothing when it cannot be read; see [`WornItemBonuses::at_root`].
pub(crate) fn custom_or_empty(root: &Path) -> CharacterCustomDto {
    load_character_custom_at_root(root).unwrap_or_default()
}

/// The first worn custom item that does not fit, after what `input` already wears and the worn items
/// before it, in the slot check's words.
fn slot_refusal(input: &CharacterInput, custom: &CharacterCustomDto) -> Result<Option<String>, String> {
    let mine = worn_custom_slots(custom);
    if mine.is_empty() {
        return Ok(None);
    }
    let mut worn = worn_stock_slots(input)?;
    for (name, slot) in mine {
        let refs: Vec<(&str, BodySlot)> = worn.iter().map(|(name, slot)| (name.as_str(), *slot)).collect();
        if let Some(refusal) = slot_conflict(&refs, slot)? {
            return Ok(Some(refusal));
        }
        worn.push((name, slot));
    }
    Ok(None)
}

/// The same bound a grant's score keeps, on the score the sheet computes from.
fn worn_score_refusal(scores: &AbilityScores, custom: &CharacterCustomDto) -> Option<String> {
    let worn = WornItemBonuses::of(custom);
    let mut scores = scores.clone();
    ABILITIES.iter().enumerate().find_map(|(slot, name)| {
        let moved = worn.total(&format!("ability:{name}"));
        let next = i32::from(*score_mut(&mut scores, slot)) + i32::from(moved);
        (moved != 0 && !(1..=60).contains(&next)).then(|| format!("the worn items would make {name} {next}, outside 1..=60"))
    })
}

/// Wears or stows one custom item, the backend half of the Equipment control for a `custom:<id>` row.
/// A refusal is `Blocked`, as a stock item's is, and leaves both files as they were. Past that,
/// `mutate_saved_character_at_root` supplies the recompute gate and the new revision.
pub(crate) fn set_custom_item_state_at_root(root: &Path, id: &str, active_state: ActiveState, saved_at: &str) -> Result<CreateCharacterResponse, String> {
    let mut custom = load_character_custom_at_root(root)?;
    let Some(index) = custom.equipment.iter().position(|record| record.id == id) else {
        return Err(format!("'{CUSTOM_ITEM_PREFIX}{id}' cannot change state: this character does not carry it"));
    };
    custom.equipment[index].worn = active_state == ActiveState::EquippedActive;
    if custom.equipment[index].worn {
        let existing = SavedCharacterStore::load(root).map_err(|err| err.message)?;
        let refusal = match slot_refusal(&existing.character_input, &custom)? {
            Some(refusal) => Some(("equipment.slot_taken", refusal)),
            None => worn_score_refusal(&existing.character_input.chosen.ability_scores, &custom).map(|refusal| ("equipment.score_range", refusal)),
        };
        if let Some((id, message)) = refusal {
            return Ok(CreateCharacterResponse::Blocked {
                diagnostics: vec![DiagnosticDto { id: id.to_owned(), message, claim_blocking: true }],
            });
        }
    }
    let outcome = crate::pf1_adapter::mutate_saved_character_at_root(root, saved_at, |_| {})?;
    if matches!(outcome, CreateCharacterResponse::Saved { .. }) {
        write_custom(root, &custom)?;
    }
    Ok(outcome)
}

fn write_custom(root: &Path, custom: &CharacterCustomDto) -> Result<(), String> {
    let path = root.join(CUSTOM_FILE_NAME);
    let json = serde_json::to_string_pretty(custom).map_err(|err| format!("failed to serialize custom data: {err}"))?;
    std::fs::write(&path, json).map_err(|err| format!("{}: {err}", path.display()))
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

    if let Some(refusal) = slot_refusal(&existing.character_input, custom)? {
        return Err(refusal);
    }
    let mut scores = existing.character_input.chosen.ability_scores.clone();
    for (slot, change) in delta.iter().enumerate() {
        let next = i32::from(*score_mut(&mut scores, slot)) + change;
        if !(1..=60).contains(&next) {
            return Err(format!("the grants would make {} {next}, outside 1..=60", ABILITIES[slot]));
        }
        *score_mut(&mut scores, slot) = next as i16;
    }
    if let Some(refusal) = worn_score_refusal(&scores, custom) {
        return Err(refusal);
    }

    if delta.iter().any(|change| *change != 0) {
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

    write_custom(root, custom)
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
