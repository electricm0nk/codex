//! Identity (the screen's Character Details), Abilities and the boxes the one Combat block printed
//! until SD-40 split it along the screen's ids (`decisions.md §8`): the square block (Hit Points,
//! Initiative, Encumbrance, Conditions; E2.9), Armor Class, Saving Throws, Attack and Speed. Each value is the field the screen's tile reads
//! (`CharacterSheet.tsx`'s `ac`, `touch`, `flatFooted`, `cmb`, `cmd`, `saves`, `initiativeRow`,
//! `AbilitiesPanel`, the Hit Points and Speed tiles). A write-in cell is an empty string: the
//! player's pencil fills it at the table, never a number made up here (NR-2, `decisions.md §11`).

use super::{pounds, trim_number, SheetSources};
use crate::print::layout::{signed, NO_VALUE};
use crate::print::model::{Look, SheetBlock, SheetRow};
use codex::rules_core::pilot_compute::ArmorClassPart;

/// The portrait box (`portrait`, SD-40 E2.5): the saved portrait when the character has one (the
/// frame scales it to fit), else the empty box; its caption is the heading.
pub fn portrait(sources: &SheetSources) -> SheetBlock {
    SheetBlock::new("Portrait", Vec::new()).in_look(Look::Portrait).with_image(sources.portrait.clone())
}

/// The screen's IdentityPanel (`nameCampaign`, SD-40 E2.5): "Character Name" (the heading) over
/// the name the screen shows, and "Campaign" over the names of the campaigns the request carries,
/// joined by ", "; with none, a write-in cell (a blank line, never a dash).
pub fn name_campaign(sources: &SheetSources) -> SheetBlock {
    let campaigns = sources.request.campaigns.iter().map(|campaign| campaign.name.as_str()).collect::<Vec<_>>().join(", ");
    SheetBlock::new(
        "Character Name",
        vec![SheetRow::new("", [sources.loaded.summary.display_label.clone()]), SheetRow::new("Campaign", [campaigns])],
    )
    .in_look(Look::NameCampaign)
}

/// The screen's Character Details (`characterDetails`, SD-40 `decisions.md §5`), printed under
/// the heading Identity: name, race, class, size, vision.
pub fn character_details(sources: &SheetSources) -> SheetBlock {
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

/// A cell the player fills in at the table: printed as an empty ruled box.
fn write_in() -> String {
    String::new()
}

/// `value` with its sign, or the dash for a value the engine does not hold.
fn with_sign(value: Option<i16>) -> String {
    value.map_or_else(|| NO_VALUE.to_owned(), signed)
}

/// `value` as it is, or the dash.
fn plain(value: Option<i16>) -> String {
    value.map_or_else(|| NO_VALUE.to_owned(), |v| v.to_string())
}

/// The engine's `explanations` row `id`'s value.
fn row_value(sources: &SheetSources, id: &str) -> Option<i16> {
    sources.explanation(id).map(|row| row.value)
}

/// One line printing `cells`, with the engine row `source` behind its number (Show the math).
fn line(sources: &SheetSources, label: &str, cells: Vec<String>, source: &str) -> SheetRow {
    SheetRow::new(label, cells).derived(sources.derivation_of(source))
}

/// The screen's square group (`squareBlock`, SD-40 E2.9, `decisions.md §23.1` item 3a): Hit Points
/// (`hit_points.max`, the current and nonlethal write-ins, the Defense tab's `Damage Reduction:
/// <n>/—` whenever the screen shows one), Initiative (the total only), Encumbrance (the load
/// response's limits, carried weight, remaining pounds, gear value and the load's name, as the
/// Carried Weight panel shows them) and Conditions (the saved `custom.json` conditions, else the
/// write-in). The engine's hit point and initiative notes (E2.6) ride on the `Max` and `Initiative`
/// lines, each `label: value condition`; the look prints them, then the blank lines.
pub fn square_block(sources: &SheetSources) -> SheetBlock {
    let max = row_value(sources, "hit_points.max").map_or_else(|| NO_VALUE.to_owned(), |v| v.to_string());
    let notes = &sources.loaded.conditional_notes;
    let load = &sources.loaded.corpus_derived.encumbrance;
    // In print order (the page-1 tests find a block by its heading and first words): the current
    // area, then the max beside it.
    let mut rows = vec![
        SheetRow::new("Current", [write_in()]),
        line(sources, "Max", vec![max], "hit_points.max").with_notes(notes.hit_points.iter().map(note_words)),
        SheetRow::new("Nonlethal", [write_in()]),
    ];
    if let Some(reduction) = sources.loaded.snapshot.as_ref().and_then(|s| s.damage_reduction) {
        rows.push(line(sources, "Damage Reduction", vec![format!("{reduction}/—")], "class_feature.barbarian.damage_reduction"));
    }
    rows.extend([
        line(sources, "Initiative", vec![with_sign(row_value(sources, "initiative.total"))], "initiative.total").with_notes(notes.initiative.iter().map(note_words)),
        SheetRow::new("Conditions", [sources.custom.conditions.join(", ")]),
        SheetRow::new(
            "Encumbrance",
            [
                pounds(load.light_max_lbs),
                pounds(load.medium_max_lbs),
                pounds(load.heavy_max_lbs),
                pounds(load.total_carried_weight_lbs),
                pounds(load.remaining_lbs),
                format!("{} gp", trim_number(load.total_carried_cost_gp)),
                load_name(&load.level),
            ],
        )
        .derived(Vec::new()),
    ]);
    SheetBlock::new("Hit Points", rows).in_look(Look::SquareBlock)
}

/// A conditional note as the sheet prints it: `label: value condition`, the engine's words.
fn note_words(note: &codex::rules_core::pilot_compute::conditional_notes::ConditionalNote) -> String {
    let said: Vec<&str> = [note.value.as_str(), note.condition.as_str()].into_iter().filter(|part| !part.is_empty()).collect();
    format!("{}: {}", note.label, said.join(" "))
}

/// The load's name as the Carried Weight panel shows it (`encumbranceTabModel.ts`'s `LEVEL_LABELS`).
fn load_name(level: &str) -> String {
    match level {
        "Light" => "Light Load".to_owned(),
        "Medium" => "Medium Load".to_owned(),
        "Heavy" => "Heavy Load".to_owned(),
        "OverHeavyCapacity" => "Over Capacity".to_owned(),
        other => other.to_owned(),
    }
}

/// The screen's Armor Class box (`armorClass`): AC, Touch and Flat-Footed, then the AC's seven
/// parts (SD-40 E2.12, `decisions.md §23.1` item 4), each the engine's `defense.armor_class.<part>`
/// row (E2.7) with that row behind it: signed, `0` where the engine states 0, blank where it states
/// no part (never worked out here, NR-2).
pub fn armor_class(sources: &SheetSources) -> SheetBlock {
    let snapshot = sources.loaded.snapshot.as_ref();
    let mut rows = vec![
        line(sources, "Armor Class", vec![plain(snapshot.map(|s| s.baseline_armor_class))], "defense.baseline_armor_class"),
        line(sources, "Touch", vec![plain(row_value(sources, "defense.touch_armor_class"))], "defense.touch_armor_class"),
        line(sources, "Flat-Footed", vec![plain(row_value(sources, "defense.flat_footed_armor_class"))], "defense.flat_footed_armor_class"),
    ];
    let labels = ["Armor", "Shield", "Dex", "Size", "Natural", "Deflection", "Misc"];
    rows.extend(labels.into_iter().zip(ArmorClassPart::ALL).map(|(label, part)| {
        let value = row_value(sources, part.id()).map_or_else(write_in, |v| if v == 0 { v.to_string() } else { signed(v) });
        line(sources, label, vec![value], part.id())
    }));
    SheetBlock::new("Armor Class", rows).in_look(Look::ArmorClass)
}

/// The screen's Saving Throws box (`saves`, SD-40 E2.10, `decisions.md §23.1` item 4): each save's
/// total only (the breakdown is Show the math's), then the save's notes from the engine (E2.6), each
/// `label: value condition`; a note the rules give more than one save (a Dwarf's Hardy) rides on the
/// first and prints once. The look adds the two blank lines.
pub fn saves(sources: &SheetSources) -> SheetBlock {
    let snapshot = sources.loaded.snapshot.as_ref();
    let notes = &sources.loaded.conditional_notes;
    let mut printed: Vec<&str> = Vec::new();
    let mut rows = Vec::new();
    for (label, key, total, list) in [
        ("Fortitude", "fortitude", snapshot.map(|s| s.total_saves.fortitude), &notes.fortitude),
        ("Reflex", "reflex", snapshot.map(|s| s.total_saves.reflex), &notes.reflex),
        ("Will", "will", snapshot.map(|s| s.total_saves.will), &notes.will),
    ] {
        let mut own = Vec::new();
        for note in list {
            if !printed.contains(&note.rule_id.as_str()) {
                printed.push(&note.rule_id);
                own.push(note_words(note));
            }
        }
        rows.push(line(sources, label, vec![with_sign(total)], &format!("defense.total_save.{key}")).with_notes(own));
    }
    SheetBlock::new("Saving Throws", rows).in_look(Look::Saves)
}

/// The screen's Attack box (`attack`): Base Attack Bonus, Melee, CMB and CMD.
pub fn attack(sources: &SheetSources) -> SheetBlock {
    let snapshot = sources.loaded.snapshot.as_ref();
    SheetBlock::new(
        "Attack",
        vec![
            line(sources, "Base Attack Bonus", vec![with_sign(snapshot.map(|s| s.base_attack_bonus))], "class_chassis.base_attack_bonus"),
            line(sources, "Melee", vec![with_sign(snapshot.map(|s| s.baseline_melee_attack_bonus))], "combat.baseline_melee_attack_bonus"),
            line(sources, "CMB", vec![with_sign(row_value(sources, "combat.combat_maneuver_bonus"))], "combat.combat_maneuver_bonus"),
            line(sources, "CMD", vec![plain(row_value(sources, "defense.combat_maneuver_defense"))], "defense.combat_maneuver_defense"),
        ],
    )
    .in_look(Look::Attack)
}

/// The screen's Speed box (`speed`): the load response's `speed.land`, the one value its Land tile shows.
pub fn speed(sources: &SheetSources) -> SheetBlock {
    let land = row_value(sources, "speed.land").map_or_else(|| NO_VALUE.to_owned(), |v| format!("{v} ft."));
    SheetBlock::new("Speed", vec![line(sources, "Land", vec![land], "speed.land")]).in_look(Look::Speed)
}

/// The Skills panel's Languages line (`languages`, SD-40 E2.13, `decisions.md §23.7` Q1): the load
/// response's `languages.known` as one comma list, then exactly one write-in line for a language
/// learned at the table; no bonus-slot count prints.
pub fn languages(sources: &SheetSources) -> SheetBlock {
    SheetBlock::new(
        "Languages",
        vec![SheetRow::new("", [sources.loaded.languages.known.join(", ")]), SheetRow::new("", [write_in()])],
    )
    .in_look(Look::Languages)
}

/// The screen's Abilities panel (SD-40 E2.10, `decisions.md §23.1` item 3b): each score and the
/// engine's modifier only; the look leaves white space beside them for the pencil.
pub fn abilities(sources: &SheetSources) -> SheetBlock {
    let scores = &sources.loaded.ability_scores;
    let modifiers = sources.loaded.snapshot.as_ref().map(|s| &s.ability_modifiers);
    let ability = |label: &str, key: &str, score: i16, modifier: Option<i16>| {
        SheetRow::new(label, [score.to_string(), with_sign(modifier)])
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
    .in_look(Look::Abilities)
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
