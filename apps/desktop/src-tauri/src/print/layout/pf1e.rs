//! The Pathfinder 1e layout file (operator ruling R2; E8.1 holds Identity, Combat and Abilities,
//! E8.2 adds the rest). Every value is read off the `LoadSavedCharacterResponse` the Character
//! Sheet renders -- the snapshot, the engine's `explanations` rows, the saved summary -- and the
//! same field the screen's tile reads (`CharacterSheet.tsx`'s `ac`, `touch`, `flatFooted`, `cmb`,
//! `cmd`, `saves`, `initiativeRow`, `AbilitiesPanel`). Nothing here computes a number (NR-2).

use super::{signed, NO_VALUE};
use crate::character_hub::LoadSavedCharacterResponse;
use crate::print::model::{SheetBlock, SheetDocument, SheetRow};

/// The sheet's blocks, in print order.
pub fn layout(loaded: &LoadSavedCharacterResponse, paper: &'static str) -> SheetDocument {
    SheetDocument {
        title: loaded.summary.display_label.clone(),
        paper,
        blocks: vec![identity(loaded), combat(loaded), abilities(loaded)],
    }
}

fn identity(loaded: &LoadSavedCharacterResponse) -> SheetBlock {
    SheetBlock::new(
        "Identity",
        vec![
            SheetRow::new("Name", [loaded.summary.display_label.clone()]),
            SheetRow::new("Race", [race_label(&loaded.summary.race_id)]),
            SheetRow::new("Class", [class_label(&loaded.summary.class_summary)]),
        ],
    )
}

fn combat(loaded: &LoadSavedCharacterResponse) -> SheetBlock {
    let snapshot = loaded.snapshot.as_ref();
    let row = |id: &str| loaded.explanations.iter().find(|row| row.id == id).map(|row| row.value);
    let plain = |value: Option<i16>| value.map_or_else(|| NO_VALUE.to_owned(), |v| v.to_string());
    let with_sign = |value: Option<i16>| value.map_or_else(|| NO_VALUE.to_owned(), signed);
    SheetBlock::new(
        "Combat",
        vec![
            SheetRow::new("Armor Class", [plain(snapshot.map(|s| s.baseline_armor_class))]),
            SheetRow::new("Touch", [plain(row("defense.touch_armor_class"))]),
            SheetRow::new("Flat-Footed", [plain(row("defense.flat_footed_armor_class"))]),
            SheetRow::new("Base Attack Bonus", [with_sign(snapshot.map(|s| s.base_attack_bonus))]),
            SheetRow::new("Melee", [with_sign(snapshot.map(|s| s.baseline_melee_attack_bonus))]),
            SheetRow::new("CMB", [with_sign(row("combat.combat_maneuver_bonus"))]),
            SheetRow::new("CMD", [plain(row("defense.combat_maneuver_defense"))]),
            SheetRow::new("Initiative", [with_sign(row("initiative.total"))]),
            SheetRow::new("Fortitude", [with_sign(snapshot.map(|s| s.total_saves.fortitude))]),
            SheetRow::new("Reflex", [with_sign(snapshot.map(|s| s.total_saves.reflex))]),
            SheetRow::new("Will", [with_sign(snapshot.map(|s| s.total_saves.will))]),
        ],
    )
}

fn abilities(loaded: &LoadSavedCharacterResponse) -> SheetBlock {
    let scores = &loaded.ability_scores;
    let modifiers = loaded.snapshot.as_ref().map(|s| &s.ability_modifiers);
    let ability = |label: &str, score: i16, modifier: Option<i16>| {
        SheetRow::new(label, [score.to_string(), modifier.map_or_else(|| NO_VALUE.to_owned(), signed)])
    };
    SheetBlock::new(
        "Abilities",
        vec![
            ability("Strength", scores.strength, modifiers.map(|m| m.strength)),
            ability("Dexterity", scores.dexterity, modifiers.map(|m| m.dexterity)),
            ability("Constitution", scores.constitution, modifiers.map(|m| m.constitution)),
            ability("Intelligence", scores.intelligence, modifiers.map(|m| m.intelligence)),
            ability("Wisdom", scores.wisdom, modifiers.map(|m| m.wisdom)),
            ability("Charisma", scores.charisma, modifiers.map(|m| m.charisma)),
        ],
    )
}

/// `race:half-orc` reads `Half-orc`: the Load screen's own label rule (`formatRaceLabel`).
fn race_label(race_id: &str) -> String {
    let bare = race_id.strip_prefix("race:").unwrap_or(race_id);
    let mut chars = bare.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => NO_VALUE.to_owned(),
    }
}

/// `class:fighter:3,class:wizard:1` reads `Fighter 3 / Wizard 1` (the sheet's `formatHeldClasses`),
/// each name the converted class record's own label -- the lookup the class roster's names come
/// from (`class_census::roster_display_name`), without sweeping every class; a class with no
/// record prints its slug, as the roster does.
fn class_label(class_summary: &str) -> String {
    let package = codex::rules_core::corpus_loader::live_sheet_rules();
    let held: Vec<String> = class_summary
        .split(',')
        .map(str::trim)
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let (class_id, level) = segment.rsplit_once(':').unwrap_or((segment, ""));
            let slug = class_id.strip_prefix("class:").unwrap_or(class_id);
            let name = package
                .and_then(|package| package.find("class", slug).and_then(|id| package.rule(id)))
                .map_or_else(|| slug.to_owned(), codex::rules_core::sheet_rule::display_label);
            if level.is_empty() {
                name
            } else {
                format!("{name} {level}")
            }
        })
        .collect();
    if held.is_empty() {
        NO_VALUE.to_owned()
    } else {
        held.join(" / ")
    }
}
