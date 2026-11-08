//! Identity, Combat and Abilities (E8.1's three blocks, moved here from the layout file by E8.2).
//! Each value is the field the screen's tile reads (`CharacterSheet.tsx`'s `ac`, `touch`,
//! `flatFooted`, `cmb`, `cmd`, `saves`, `initiativeRow`, `AbilitiesPanel`).

use super::SheetSources;
use crate::print::layout::{signed, NO_VALUE};
use crate::print::model::{SheetBlock, SheetRow};

pub fn identity(sources: &SheetSources) -> SheetBlock {
    let summary = &sources.loaded.summary;
    let (size, vision) = size_and_vision(sources);
    SheetBlock::new(
        "Identity",
        vec![
            SheetRow::new("Name", [summary.display_label.clone()]),
            SheetRow::new("Race", [race_label(&summary.race_id)]),
            SheetRow::new("Class", [class_label(&summary.class_summary)]),
            SheetRow::new("Size", [size]),
            SheetRow::new("Vision", [vision]),
        ],
    )
}

/// The Details panel's Size and Vision: the race roster's (`list_race_creation_roster`, what
/// `deriveRaceTraits` reads), `Unknown` for a race it does not hold, and an alternate racial trait's
/// senses row where the engine computed one (Minesight), as the screen shows them.
fn size_and_vision(sources: &SheetSources) -> (String, String) {
    const UNKNOWN: &str = "Unknown";
    let roster = crate::character_hub::list_race_creation_roster();
    let race = roster.races.iter().find(|race| race.race_id == sources.loaded.summary.race_id);
    let size = race.map_or_else(|| UNKNOWN.to_owned(), |race| race.size.clone());
    let vision = match sources.explanation("race.dwarf.alternate_trait.minesight.senses") {
        Some(row) => format!("Darkvision {} ft.", row.value),
        None => race.map_or_else(|| UNKNOWN.to_owned(), |race| race.vision.clone()),
    };
    (size, vision)
}

pub fn combat(sources: &SheetSources) -> SheetBlock {
    let snapshot = sources.loaded.snapshot.as_ref();
    let row = |id: &str| sources.explanation(id).map(|row| row.value);
    let plain = |value: Option<i16>| value.map_or_else(|| NO_VALUE.to_owned(), |v| v.to_string());
    let with_sign = |value: Option<i16>| value.map_or_else(|| NO_VALUE.to_owned(), signed);
    let line = |label: &str, value: String, source: &str| SheetRow::new(label, [value]).derived(sources.derivation_of(source));
    let mut block = SheetBlock::new(
        "Combat",
        vec![
            line("Armor Class", plain(snapshot.map(|s| s.baseline_armor_class)), "defense.baseline_armor_class"),
            line("Touch", plain(row("defense.touch_armor_class")), "defense.touch_armor_class"),
            line("Flat-Footed", plain(row("defense.flat_footed_armor_class")), "defense.flat_footed_armor_class"),
            line("Base Attack Bonus", with_sign(snapshot.map(|s| s.base_attack_bonus)), "class_chassis.base_attack_bonus"),
            line("Melee", with_sign(snapshot.map(|s| s.baseline_melee_attack_bonus)), "combat.baseline_melee_attack_bonus"),
            line("CMB", with_sign(row("combat.combat_maneuver_bonus")), "combat.combat_maneuver_bonus"),
            line("CMD", plain(row("defense.combat_maneuver_defense")), "defense.combat_maneuver_defense"),
            line("Initiative", with_sign(row("initiative.total")), "initiative.total"),
            line("Fortitude", with_sign(snapshot.map(|s| s.total_saves.fortitude)), "defense.total_save.fortitude"),
            line("Reflex", with_sign(snapshot.map(|s| s.total_saves.reflex)), "defense.total_save.reflex"),
            line("Will", with_sign(snapshot.map(|s| s.total_saves.will)), "defense.total_save.will"),
        ],
    );
    // The Defense tab's `Damage Reduction: <n>/—`, printed whenever the screen shows one.
    if let Some(reduction) = snapshot.and_then(|s| s.damage_reduction) {
        block.rows.push(line("Damage Reduction", format!("{reduction}/—"), "class_feature.barbarian.damage_reduction"));
    }
    block
}

pub fn abilities(sources: &SheetSources) -> SheetBlock {
    let scores = &sources.loaded.ability_scores;
    let modifiers = sources.loaded.snapshot.as_ref().map(|s| &s.ability_modifiers);
    let ability = |label: &str, key: &str, score: i16, modifier: Option<i16>| {
        SheetRow::new(label, [score.to_string(), modifier.map_or_else(|| NO_VALUE.to_owned(), signed)])
            .derived(sources.derivation_of(&format!("ability_modifier.{key}")))
    };
    SheetBlock::new(
        "Abilities",
        vec![
            ability("Strength", "strength", scores.strength, modifiers.map(|m| m.strength)),
            ability("Dexterity", "dexterity", scores.dexterity, modifiers.map(|m| m.dexterity)),
            ability("Constitution", "constitution", scores.constitution, modifiers.map(|m| m.constitution)),
            ability("Intelligence", "intelligence", scores.intelligence, modifiers.map(|m| m.intelligence)),
            ability("Wisdom", "wisdom", scores.wisdom, modifiers.map(|m| m.wisdom)),
            ability("Charisma", "charisma", scores.charisma, modifiers.map(|m| m.charisma)),
        ],
    )
}

/// `race:half-orc` reads `Half-orc`: the Load screen's own label rule (`formatRaceLabel`).
pub(crate) fn race_label(race_id: &str) -> String {
    let bare = race_id.strip_prefix("race:").unwrap_or(race_id);
    let mut chars = bare.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => NO_VALUE.to_owned(),
    }
}

/// The converted rule's own label for `kind`/`slug`, or the slug when the package holds none --
/// the lookup the class roster's names come from (`class_census::roster_display_name`).
pub(crate) fn rule_label(kind: &str, slug: &str) -> Option<String> {
    let package = codex::rules_core::corpus_loader::live_sheet_rules()?;
    package.find(kind, slug).and_then(|id| package.rule(id)).map(codex::rules_core::sheet_rule::display_label)
}

/// `class:fighter:3,class:wizard:1` reads `Fighter 3 / Wizard 1` (the sheet's `formatHeldClasses`),
/// each name the converted class record's own label; a class with no record prints its slug, as
/// the roster does.
pub(crate) fn class_label(class_summary: &str) -> String {
    let held: Vec<String> = class_summary
        .split(',')
        .map(str::trim)
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let (class_id, level) = segment.rsplit_once(':').unwrap_or((segment, ""));
            let name = class_name(class_id);
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

/// `class:fighter` (or `fighter`) reads `Fighter`: the converted class record's own label, or the
/// slug for a class with no record, as the roster does.
pub(crate) fn class_name(class_id: &str) -> String {
    let slug = class_id.strip_prefix("class:").unwrap_or(class_id);
    rule_label("class", slug).unwrap_or_else(|| slug.to_owned())
}
