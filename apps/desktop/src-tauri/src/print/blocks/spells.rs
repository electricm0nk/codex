//! Spells (section 2, SD-40 E3.3: per class the caster level, the save DCs and spells per day the
//! engine computed, the used boxes and the spell names by level) and the Spellbook (section 4: one
//! entry per held spell, its fields and its full text, from the converted rules' spell line, else
//! the spell catalog's school and description).

use std::collections::BTreeMap;

use super::{derivation, words_of, SheetSources};
use crate::character_hub::{AcquisitionModeDto, ExplanationDto};
use crate::character_ledger::AdmittedKind;
use crate::print::layout::NO_VALUE;
use crate::print::model::{Look, SheetBlock, SheetRow};
use codex::rules_core::rules_catalog::class_spell_levels;

/// The source class id a spell takes when an override let it in (E6.1).
const OVERRIDE_SOURCE: &str = "override:spells_outside_class_list";

/// Each spell once, in the order first held, with every way the character holds it.
fn held_spells(sources: &SheetSources) -> Vec<(String, Vec<&'static str>, Vec<String>)> {
    let mut held: Vec<(String, Vec<&'static str>, Vec<String>)> = Vec::new();
    for selection in &sources.loaded.spells_selected {
        let mode = match selection.acquisition_mode {
            AcquisitionModeDto::Known => "Known",
            AcquisitionModeDto::Prepared => "Prepared",
            AcquisitionModeDto::Granted => "Granted",
        };
        let index = match held.iter().position(|(id, _, _)| id == &selection.spell_id) {
            Some(index) => index,
            None => {
                held.push((selection.spell_id.clone(), Vec::new(), Vec::new()));
                held.len() - 1
            }
        };
        let (_, modes, sources_of) = &mut held[index];
        if !modes.contains(&mode) {
            modes.push(mode);
        }
        let class = selection.source_class_id.strip_prefix("class:").unwrap_or(&selection.source_class_id);
        if selection.source_class_id != OVERRIDE_SOURCE && !sources_of.iter().any(|known| known == class) {
            sources_of.push(class.to_owned());
        }
    }
    held
}

/// A spell's level for the class that holds it: that class's own spell list (as the Spells tab
/// reads it, `list_class_spell_levels`), else the record's lowest level across classes (`true`: the
/// class has no spell list ingested, or an override let the spell in).
fn spell_level(class: &str, spell: &str) -> (Option<u8>, bool) {
    if let Some(entries) = class_spell_levels::class_spell_list_entries(class) {
        if let Some((_, level)) = entries.iter().find(|(key, _)| key.eq_ignore_ascii_case(spell)) {
            return (Some(*level), false);
        }
    }
    let lowest = codex::rules_core::spell_resolver::spell_catalog_rows().iter().find(|row| row.key.eq_ignore_ascii_case(spell)).and_then(|row| row.level);
    (lowest, true)
}

/// The class token of a spells-per-day row id, its spell level and whether it is the total (else the
/// class table's base column): `class_spell.wizard.total_spells_per_day.spell_level_3`,
/// `class_spell.acg.investigator.base_extracts_per_day.extract_level_1`,
/// `class_chassis.sorcerer.spontaneous.total_spells_per_day.spell_level_1` (the Spells tab's
/// `PER_DAY_ID`, `spellsPerDayModel.ts`).
fn per_day_id(id: &str) -> Option<(&str, u8, bool)> {
    let rest = id.strip_prefix("class_spell.").or_else(|| id.strip_prefix("class_chassis."))?;
    let parts: Vec<&str> = rest.split('.').collect();
    let [.., before, kind, level] = parts.as_slice() else { return None };
    let level = level.strip_prefix("spell_level_").or_else(|| level.strip_prefix("extract_level_"))?.parse().ok()?;
    let total = match *kind {
        "total_spells_per_day" | "total_extracts_per_day" => true,
        "base_spells_per_day" | "base_extracts_per_day" => false,
        _ => return None,
    };
    let class = if matches!(*before, "spontaneous" | "partial_caster") { parts.get(parts.len().checked_sub(4)?)? } else { before };
    Some((class, level, total))
}

/// One casting class's lines: its caster level, its per-day and save-DC rows by spell level, and
/// the spells it holds (name, level, how held).
#[derive(Default)]
struct Caster<'a> {
    token: String,
    caster_level: Option<&'a ExplanationDto>,
    per_day: BTreeMap<u8, &'a ExplanationDto>,
    dc: BTreeMap<u8, &'a ExplanationDto>,
    spells: Vec<(String, Option<u8>, Vec<&'static str>)>,
    lowest: bool,
}

/// The Spells block (SD-40 E3.3, section 2): per class, its caster level, then per spell level the
/// save DC, the spells per day (the engine's rows), how many spells it knows / has prepared at that
/// level, and the `spells` look's used boxes, one per spell per day; then the spell names by level.
pub fn spells(sources: &SheetSources) -> SheetBlock {
    let mut casters: Vec<Caster> = Vec::new();
    let caster = |casters: &mut Vec<Caster>, token: &str| -> usize {
        casters.iter().position(|caster| caster.token == token).unwrap_or_else(|| {
            casters.push(Caster { token: token.to_owned(), ..Caster::default() });
            casters.len() - 1
        })
    };
    for selection in &sources.loaded.spells_selected {
        let mode = match selection.acquisition_mode {
            AcquisitionModeDto::Known => "Known",
            AcquisitionModeDto::Prepared => "Prepared",
            AcquisitionModeDto::Granted => "Granted",
        };
        let (token, class_id) = if selection.source_class_id == OVERRIDE_SOURCE {
            (OVERRIDE_SOURCE, "")
        } else {
            (selection.source_class_id.rsplit(':').next().unwrap_or(&selection.source_class_id), selection.source_class_id.as_str())
        };
        let index = caster(&mut casters, token);
        let held = &mut casters[index];
        match held.spells.iter_mut().find(|(spell, _, _)| spell == &selection.spell_id) {
            Some((_, _, modes)) if !modes.contains(&mode) => modes.push(mode),
            Some(_) => {}
            None => {
                let (level, lowest) = spell_level(class_id, &selection.spell_id);
                held.lowest |= lowest;
                held.spells.push((selection.spell_id.clone(), level, vec![mode]));
            }
        }
    }
    for row in &sources.loaded.explanations {
        if let Some((token, level, total)) = per_day_id(&row.id) {
            let index = caster(&mut casters, token);
            let slot = casters[index].per_day.entry(level).or_insert(row);
            // A total wins over the class table's base column (the Spells tab's rule).
            if total && !slot.id.contains(".total_") {
                *slot = row;
            }
        }
    }
    // The caster level and the save DCs, for the classes that hold slots or spells.
    for row in &sources.loaded.explanations {
        let Some((token, what)) = row.id.strip_prefix("class_chassis.").and_then(|rest| rest.split_once('.')) else { continue };
        let Some(caster) = casters.iter_mut().find(|caster| caster.token == token) else { continue };
        if what == "caster_level" {
            caster.caster_level = Some(row);
        } else if let Some(level) = what.strip_prefix("spell_save_dc.spell_level_").and_then(|level| level.parse().ok()) {
            caster.dc.insert(level, row);
        }
    }
    let mut rows: Vec<SheetRow> = Vec::new();
    for caster in &casters {
        let name = if caster.token == OVERRIDE_SOURCE { "Outside the class list".to_owned() } else { words_of(&caster.token) };
        let mut class = match caster.caster_level {
            Some(row) => SheetRow::new(name, [row.value.to_string()]).derived(vec![derivation(row)]),
            None => SheetRow::new(name, Vec::<String>::new()),
        };
        if caster.lowest {
            class = class.with_notes(["Levels marked by a spell's lowest level across classes: no spell list for this class.".to_owned()]);
        }
        rows.push(class.of_kind("class"));
        let levels: std::collections::BTreeSet<u8> = caster.per_day.keys().copied().chain(caster.spells.iter().filter_map(|(_, level, _)| *level)).collect();
        for level in levels {
            let count = |mode: &str| caster.spells.iter().filter(|(_, at, modes)| *at == Some(level) && modes.contains(&mode)).count();
            let per_day = caster.per_day.get(&level);
            let dc = caster.dc.get(&level);
            let value = |row: Option<&&ExplanationDto>| row.map_or_else(|| NO_VALUE.to_owned(), |row| row.value.to_string());
            let mut line = SheetRow::new(format!("Level {level}"), [value(dc), value(per_day), format!("{} / {}", count("Known"), count("Prepared"))]).of_kind("level");
            if per_day.is_some_and(|row| !row.id.contains(".total_")) {
                line = line.with_notes(["Spells per day: the class table only, bonus spells not included.".to_owned()]);
            }
            if dc.is_some() || per_day.is_some() {
                line = line.derived(dc.into_iter().chain(per_day).map(|row| derivation(row)).collect());
            }
            rows.push(line);
        }
        let mut spells: Vec<&(String, Option<u8>, Vec<&'static str>)> = caster.spells.iter().collect();
        spells.sort_by_key(|(_, level, _)| level.unwrap_or(u8::MAX));
        for (spell, level, modes) in spells {
            let mut tags = sources.override_tags(AdmittedKind::Spell, spell);
            if caster.token == OVERRIDE_SOURCE && tags.is_empty() {
                tags.push(super::override_tag(crate::character_ledger::OverrideFlag::SpellsOutsideClassList));
            }
            // Known and prepared are the level line's counts; a spell a feature grants says so.
            if modes.contains(&"Granted") {
                tags.push("Granted".to_owned());
            }
            let level = level.map_or_else(|| NO_VALUE.to_owned(), |level| level.to_string());
            rows.push(SheetRow::new(spell.clone(), [level]).of_kind("spell").with_notes(tags));
        }
    }
    if rows.is_empty() {
        rows.push(SheetRow::new("No spell is known.", Vec::<String>::new()));
    }
    SheetBlock::new("Spells", rows).in_look(Look::Spells)
}

/// The fields of a Spellbook entry, in print order: the captions the `spellbook` look prints and,
/// but for School and Level, the spell rule's own stat-block labels (`<label>: <words>` lines of its
/// sheet line).
const CARD_FIELDS: [&str; 9] = ["School", "Level", "Casting time", "Components", "Range", "Target or area", "Duration", "Saving throw", "Spell resistance"];

/// The Spellbook (SD-40 E3.3, section 4; R3: PCGen's 3x5 cards as the picture): one entry per held
/// spell, its name, the `CARD_FIELDS` and its text, from the converted rules' spell line (each
/// stat-block line lifted into its field), else the spell catalog's school and description.
pub fn spellbook(sources: &SheetSources) -> SheetBlock {
    let catalog = codex::rules_core::spell_resolver::spell_catalog_rows();
    let mut rows = Vec::new();
    for (spell, _, classes) in held_spells(sources) {
        let line = sources.loaded.sheet_lines.iter().find(|line| line.kind == "spell" && line.label.eq_ignore_ascii_case(&spell));
        let record = catalog.iter().find(|row| row.key.eq_ignore_ascii_case(&spell));
        let mut fields: Vec<String> = vec![String::new(); CARD_FIELDS.len()];
        let (mut subschool, mut descriptor) = (None, None);
        let mut text: Vec<String> = Vec::new();
        for words in line.map(|line| line.prose.as_str()).unwrap_or_default().lines() {
            let stat = words.split_once(": ").filter(|(label, _)| CARD_FIELDS.contains(label) || matches!(*label, "Subschool" | "Descriptor"));
            // The first line of a label is the spell's: a record's later `School: FireSchool` or
            // `Descriptor: WoodSchool` line is its elemental-school tag, not a fact the card prints.
            match stat {
                Some(("Subschool", value)) => subschool = subschool.or(Some(value)),
                Some(("Descriptor", value)) => descriptor = descriptor.or(Some(value)),
                Some((label, value)) => {
                    let field = &mut fields[CARD_FIELDS.iter().position(|field| *field == label).expect("a card field")];
                    if field.is_empty() {
                        value.clone_into(field);
                    }
                }
                None => text.push(words.to_owned()),
            }
        }
        if fields[0].is_empty() {
            fields[0] = record.and_then(|row| row.school.clone()).unwrap_or_default();
        }
        // `Evocation (Teleportation) [Fire]`, the way the books print a school.
        if let Some(subschool) = subschool {
            fields[0] = format!("{} ({subschool})", fields[0]).trim_start().to_owned();
        }
        if let Some(descriptor) = descriptor {
            fields[0] = format!("{} [{descriptor}]", fields[0]).trim_start().to_owned();
        }
        fields[1] = classes
            .iter()
            .map(|class| format!("{} {}", words_of(class), spell_level(&format!("class:{class}"), &spell).0.map_or_else(|| NO_VALUE.to_owned(), |level| level.to_string())))
            .collect::<Vec<_>>()
            .join(", ");
        for field in &mut fields {
            if field.trim().is_empty() {
                *field = NO_VALUE.to_owned();
            }
        }
        if text.iter().all(|words| words.trim().is_empty()) {
            text = vec![record.and_then(|row| row.description).unwrap_or("The rules data hold no text for this spell.").to_owned()];
        }
        rows.push(SheetRow::new(spell, fields).of_kind("spell").with_notes(text));
    }
    // The captions come last: the block's first words are its first spell's name.
    if rows.is_empty() {
        rows.push(SheetRow::new("No spell is known.", Vec::<String>::new()));
    } else {
        rows.push(SheetRow::new("Spell", CARD_FIELDS.map(str::to_owned)).of_kind("captions"));
    }
    SheetBlock::new("Spellbook", rows).in_look(Look::Spellbook).on_new_page()
}
