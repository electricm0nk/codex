//! The Bio page's details (the screen's Character Details, E3.4), Abilities and the boxes the one Combat block printed
//! until SD-40 split it along the screen's ids (`decisions.md §8`): the square block (Hit Points,
//! Initiative, Encumbrance, Conditions; E2.9), Armor Class, Saving Throws, Attack and Speed. Each value is the field the screen's tile reads
//! (`CharacterSheet.tsx`'s `ac`, `touch`, `flatFooted`, `cmb`, `cmd`, `saves`, `initiativeRow`,
//! `AbilitiesPanel`, the Hit Points and Speed tiles). A write-in cell is an empty string: the
//! player's pencil fills it at the table, never a number made up here (NR-2, `decisions.md §11`).

use super::{pounds, trim_number, SheetSources};
use crate::print::layout::{signed, NO_VALUE};
use crate::print::model::{Look, SheetBlock, SheetRow};
use codex::rules_core::pilot_compute::conditional_notes::ConditionalNote;
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

/// The Bio section's page (`characterDetails`, SD-40 E3.4, `decisions.md §23.1` item 5, §24.9),
/// under the heading Details: the Details panel's fields as a form (`detail` rows: Alignment …
/// Eyes from the bio, Size and Vision as the panel shows them, Race, Class and level, Player; an
/// empty field prints its label over a blank line); then, when the Bio notes are asked for, the
/// Biography tab's text and each campaign's description (`text` rows, a paragraph per line; an
/// empty Biography and no campaign print nothing). The look ends the page with ruled lines.
pub fn character_details(sources: &SheetSources) -> SheetBlock {
    let summary = &sources.loaded.summary;
    let bio = &sources.bio;
    let (size, vision) = size_and_vision(sources);
    let details = [
        ("Alignment", bio.alignment.clone()),
        ("Deity", bio.deity.clone()),
        ("Sex", bio.sex.clone()),
        ("Age", bio.age.clone()),
        ("Height", bio.height.clone()),
        ("Weight", bio.weight.clone()),
        ("Hair", bio.hair.clone()),
        ("Eyes", bio.eyes.clone()),
        ("Size", size),
        ("Vision", vision),
        ("Race", race_label(&summary.race_id)),
        ("Class and level", class_label(&summary.class_summary)),
        ("Player", bio.player_name.clone()),
    ];
    let mut rows: Vec<SheetRow> = details.into_iter().map(|(label, value)| SheetRow::new(label, [value]).of_kind("detail")).collect();
    if sources.request.bio {
        let text = |label: &str, body: &str| SheetRow::new(label, Vec::<String>::new()).with_notes(body.lines().map(str::to_owned)).of_kind("text");
        if !bio.biography.trim().is_empty() {
            rows.push(text("Biography", &bio.biography));
        }
        rows.extend(sources.request.campaigns.iter().map(|campaign| text(&campaign.name, &campaign.description)));
    }
    SheetBlock::new("Details", rows).in_look(Look::Bio)
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
/// (`hit_points.max`, the current and nonlethal write-ins; Damage Reduction prints in the
/// Resistances block, E2.22), Initiative (the total only), Encumbrance (the load
/// response's limits, carried weight, remaining pounds, gear value and the load's name, as the
/// Carried Weight panel shows them) and Conditions (the saved `custom.json` conditions, else the
/// write-in). The hit point and initiative notes print in the Conditional Bonuses box (E2.20).
pub fn square_block(sources: &SheetSources) -> SheetBlock {
    let max = row_value(sources, "hit_points.max").map_or_else(|| NO_VALUE.to_owned(), |v| v.to_string());
    let load = &sources.loaded.corpus_derived.encumbrance;
    // In print order (the page-1 tests find a block by its heading and first words): the current
    // area, then the max beside it.
    let mut rows = vec![
        SheetRow::new("Current", [write_in()]),
        line(sources, "Max", vec![max], "hit_points.max"),
        SheetRow::new("Nonlethal", [write_in()]),
    ];
    rows.extend([
        line(sources, "Initiative", vec![with_sign(row_value(sources, "initiative.total"))], "initiative.total"),
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
fn note_words(note: &ConditionalNote) -> String {
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
    // No "Armor Class" heading over the boxes (`decisions.md §24.3.1`): the AC box's label names it.
    SheetBlock::new("", rows).in_look(Look::ArmorClass)
}

/// The screen's Saving Throws box (`saves`, SD-40 E2.10, `decisions.md §23.1` item 4): each save's
/// total only (the breakdown is Show the math's); the save notes print in the Conditional Bonuses
/// box (E2.20, `decisions.md §24.3.4`).
pub fn saves(sources: &SheetSources) -> SheetBlock {
    let snapshot = sources.loaded.snapshot.as_ref();
    let rows = [
        ("Fortitude", "fortitude", snapshot.map(|s| s.total_saves.fortitude)),
        ("Reflex", "reflex", snapshot.map(|s| s.total_saves.reflex)),
        ("Will", "will", snapshot.map(|s| s.total_saves.will)),
    ]
    .into_iter()
    .map(|(label, key, total)| line(sources, label, vec![with_sign(total)], &format!("defense.total_save.{key}")))
    .collect();
    SheetBlock::new("Saving Throws", rows).in_look(Look::Saves)
}

/// A breakdown table's caption line (Show the math, SD-40 E3.5): the `breakdown` look prints it as
/// the table's header.
pub(super) fn captions(cells: &[&str]) -> SheetRow {
    SheetRow::new("", cells.iter().map(|cell| (*cell).to_owned())).of_kind("captions")
}

/// Show the math's Initiative breakdown (SD-40 E3.5, `decisions.md §23.1` item 4): the cells page 1
/// dropped at E2.9, the engine's total and the Dexterity modifier.
pub(super) fn initiative_breakdown(sources: &SheetSources) -> SheetBlock {
    let dexterity = sources.loaded.snapshot.as_ref().map(|s| s.ability_modifiers.dexterity);
    SheetBlock::new(
        "Initiative Breakdown",
        vec![captions(&["Total", "Dex"]), SheetRow::new("Initiative", [with_sign(row_value(sources, "initiative.total")), with_sign(dexterity)])],
    )
    .in_look(Look::Breakdown)
}

/// Show the math's Saving Throws breakdown (SD-40 E3.5): the cells page 1 dropped at E2.10, each
/// save's total, its class base save and its ability's modifier (Constitution, Dexterity, Wisdom).
pub(super) fn saves_breakdown(sources: &SheetSources) -> SheetBlock {
    let snapshot = sources.loaded.snapshot.as_ref();
    let save = |label: &str, pick: fn(&crate::character_hub::BaseSavesDto) -> i16, ability: fn(&crate::character_hub::AbilityModifiersDto) -> i16| {
        SheetRow::new(
            label,
            [
                with_sign(snapshot.map(|s| pick(&s.total_saves))),
                with_sign(snapshot.map(|s| pick(&s.base_saves))),
                with_sign(snapshot.map(|s| ability(&s.ability_modifiers))),
            ],
        )
    };
    SheetBlock::new(
        "Saving Throw Breakdown",
        vec![
            captions(&["Total", "Base", "Ability"]),
            save("Fortitude", |s| s.fortitude, |m| m.constitution),
            save("Reflex", |s| s.reflex, |m| m.dexterity),
            save("Will", |s| s.will, |m| m.wisdom),
        ],
    )
    .in_look(Look::Breakdown)
}

/// The Conditional Bonuses box (`conditionalBonuses`, print-only, SD-40 E2.20, `decisions.md
/// §24.3.5`): every conditional the engine states, one row each, in order HP, Initiative,
/// Fortitude, Reflex, Will, AC, CMB, CMD (`conditionalNotes`), then Skills (what a feat states on a
/// skill and no total adds, `feat_skill_bonuses`). A row's label is what the bonus is to; its value
/// is the engine's words, `label: value condition`. A note the rules give several saves prints once,
/// under `Saves` when it is on all three, else under the saves that hold it. The look prints the
/// rows in a fixed box with its blank write-in lines.
pub fn conditional_bonuses(sources: &SheetSources) -> SheetBlock {
    let notes = &sources.loaded.conditional_notes;
    let mut rows: Vec<SheetRow> = Vec::new();
    let noted = |rows: &mut Vec<SheetRow>, what: &str, list: &[ConditionalNote]| rows.extend(list.iter().map(|note| SheetRow::new(what, [note_words(note)])));
    noted(&mut rows, "HP", &notes.hit_points);
    noted(&mut rows, "Initiative", &notes.initiative);
    let saves = [("Fortitude", &notes.fortitude), ("Reflex", &notes.reflex), ("Will", &notes.will)];
    let mut printed: Vec<&str> = Vec::new();
    for (_, list) in saves {
        for note in list {
            if printed.contains(&note.rule_id.as_str()) {
                continue;
            }
            printed.push(&note.rule_id);
            let held: Vec<&str> = saves.iter().filter(|(_, list)| list.iter().any(|n| n.rule_id == note.rule_id)).map(|(name, _)| *name).collect();
            let what = if held.len() == saves.len() { "Saves".to_owned() } else { held.join(" / ") };
            rows.push(SheetRow::new(what, [note_words(note)]));
        }
    }
    noted(&mut rows, "AC", &notes.armor_class);
    noted(&mut rows, "CMB", &notes.combat_maneuver_bonus);
    noted(&mut rows, "CMD", &notes.combat_maneuver_defense);
    let skills = &sources.loaded.feat_skill_bonuses;
    rows.extend(
        skills.situational.iter().chain(&skills.unknown).map(|line| SheetRow::new("Skills", [format!("{} on {}: {}", line.label, super::skills::skill_name(&line.skill), line.reason)])),
    );
    SheetBlock::new("Conditional Bonuses", rows).in_look(Look::ConditionalBonuses)
}

/// The Resistances block (`resistances`, print-only, SD-40 E2.22, `decisions.md §24.3.6`): the load
/// response's `resistances` (E2.21) in eight boxes: Acid, Cold, Electricity, Fire, Sonic (the
/// amount), Damage Reduction (each `amount/bypass`, or the record's words), Spell Resistance (each
/// value, or the words) and Immunities (the names as one comma list). A box the character has
/// nothing for is empty, for the pencil. The class DR has the engine's row behind it (Show the math).
pub fn resistances(sources: &SheetSources) -> SheetBlock {
    use codex::rules_core::pilot_compute::resistances::ENERGIES;
    let held = &sources.loaded.resistances;
    let mut rows: Vec<SheetRow> = ENERGIES
        .iter()
        .map(|energy| SheetRow::new(*energy, [held.energy.iter().find(|e| e.energy == *energy).map_or_else(write_in, |e| e.amount.to_string())]))
        .collect();
    let words = |amount: Option<i16>, words: &Option<String>| amount.map(|n| n.to_string()).or_else(|| words.clone()).unwrap_or_default();
    let reduction = held
        .damage_reduction
        .iter()
        .map(|dr| match dr.amount {
            Some(amount) => format!("{amount}/{}", dr.bypass),
            None => words(None, &dr.words),
        })
        .collect::<Vec<_>>()
        .join(", ");
    let derivation: Vec<_> = held.damage_reduction.iter().flat_map(|dr| dr.source.split("; ")).flat_map(|source| sources.derivation_of(source)).collect();
    let reduction = SheetRow::new("Damage Reduction", [reduction]);
    rows.push(if derivation.is_empty() { reduction } else { reduction.derived(derivation) });
    rows.push(SheetRow::new("Spell Resistance", [held.spell_resistance.iter().map(|sr| words(sr.value, &sr.words)).collect::<Vec<_>>().join(", ")]));
    rows.push(SheetRow::new("Immunities", [held.immunities.iter().map(|immunity| immunity.name.as_str()).collect::<Vec<_>>().join(", ")]));
    SheetBlock::new("Resistances", rows).in_look(Look::Resistances)
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

/// The Speed box (`speed`, E2.18, `decisions.md §24.3.2`): Land, Swim and Fly, each `<n> ft.` from the
/// engine's `speed.land` / `speed.swim` / `speed.fly` row, the fly value followed by the load
/// response's `flyManeuverability` words (§24.6 Q3); a mode the engine states no row for is an empty
/// box for the pencil (never a dash, never 0) with no row behind it.
pub fn speed(sources: &SheetSources) -> SheetBlock {
    let mode = |label: &str, id: &str, words: Option<&str>| match row_value(sources, id) {
        Some(feet) => line(sources, label, vec![words.map_or_else(|| format!("{feet} ft."), |words| format!("{feet} ft. ({words})"))], id),
        None => SheetRow::new(label, [write_in()]),
    };
    let fly = sources.loaded.fly_maneuverability.as_deref();
    SheetBlock::new("Speed", vec![mode("Land", "speed.land", None), mode("Swim", "speed.swim", None), mode("Fly", "speed.fly", fly)]).in_look(Look::Speed)
}

/// The Skills panel's Languages line (`languages`, SD-40 E2.13; E2.16, `decisions.md §24.3.3`): the
/// load response's `languages.known` (the stored list, else `Common`) as one comma list, with no
/// write-in line and no bonus-slot count.
pub fn languages(sources: &SheetSources) -> SheetBlock {
    SheetBlock::new("Languages", vec![SheetRow::new("", [sources.loaded.languages.known.join(", ")])]).in_look(Look::Languages)
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
