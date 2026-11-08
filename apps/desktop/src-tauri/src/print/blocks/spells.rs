//! Spells (the list, the save DCs and spells per day the engine computed) and the Spellbook
//! option (operator ruling R4): the full text of every spell the character holds, from the
//! converted rules' spell line, else the spell catalog's description.

use super::{derivation, words_of, SheetSources};
use crate::character_hub::AcquisitionModeDto;
use crate::character_ledger::AdmittedKind;
use crate::print::model::{SheetBlock, SheetRow};

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

/// `class_spell.wizard.total_spells_per_day.spell_level_3` reads `Wizard: spells per day, level 3`.
fn spell_row_label(id: &str) -> Option<String> {
    let shapes: [(&str, &str, &str); 3] = [
        ("class_chassis.", ".spell_save_dc.spell_level_", "spell save DC, level "),
        ("class_spell.", ".total_spells_per_day.spell_level_", "spells per day, level "),
        ("class_chassis.", ".caster_level", "caster level"),
    ];
    for (prefix, middle, words) in shapes {
        let Some(rest) = id.strip_prefix(prefix) else { continue };
        let Some((class, level)) = rest.split_once(middle) else { continue };
        if class.contains('.') {
            continue;
        }
        return Some(format!("{}: {words}{level}", words_of(class)));
    }
    None
}

pub fn spells(sources: &SheetSources) -> SheetBlock {
    let mut rows: Vec<SheetRow> = held_spells(sources)
        .into_iter()
        .map(|(spell, modes, classes)| {
            let mut tags = sources.override_tags(AdmittedKind::Spell, &spell);
            let admitted_by_source = sources
                .loaded
                .spells_selected
                .iter()
                .any(|selection| selection.spell_id == spell && selection.source_class_id == OVERRIDE_SOURCE);
            if admitted_by_source && tags.is_empty() {
                tags.push(super::override_tag(crate::character_ledger::OverrideFlag::SpellsOutsideClassList));
            }
            let class = classes.iter().map(|class| words_of(class)).collect::<Vec<_>>().join(", ");
            SheetRow::new(spell, [class, modes.join(", ")]).with_notes(tags)
        })
        .collect();
    if rows.is_empty() {
        rows.push(SheetRow::new("No spell is known.", Vec::<String>::new()));
    }
    for row in &sources.loaded.explanations {
        if let Some(label) = spell_row_label(&row.id) {
            rows.push(SheetRow::new(label, [row.value.to_string()]).derived(vec![derivation(row)]));
        }
    }
    SheetBlock::new("Spells", rows)
}

/// The Spellbook: each held spell's full text, on pages of its own.
pub fn spellbook(sources: &SheetSources) -> SheetBlock {
    let catalog = codex::rules_core::spell_resolver::spell_catalog_rows();
    let mut rows: Vec<SheetRow> = held_spells(sources)
        .into_iter()
        .map(|(spell, _, _)| {
            let line = sources.loaded.sheet_lines.iter().find(|line| line.kind == "spell" && line.label.eq_ignore_ascii_case(&spell));
            let row = catalog.iter().find(|row| row.key.eq_ignore_ascii_case(&spell));
            let school = row.and_then(|row| row.school.clone()).unwrap_or_default();
            let text = line
                .map(|line| line.prose.clone())
                .filter(|prose| !prose.trim().is_empty())
                .or_else(|| row.and_then(|row| row.description.map(str::to_owned)))
                .unwrap_or_else(|| "The rules data hold no text for this spell.".to_owned());
            SheetRow::new(spell, [school]).with_notes(text.split('\n').map(str::to_owned))
        })
        .collect();
    if rows.is_empty() {
        rows.push(SheetRow::new("No spell is known.", Vec::<String>::new()));
    }
    SheetBlock::new("Spellbook", rows).on_new_page()
}
