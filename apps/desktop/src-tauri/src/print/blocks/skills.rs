//! Skills: every skill the Skills panel lists (one `skill.total.<skill>` row each, E8.7), printed in
//! the `skills` look: one column, each row the name and the load response's total (the one value
//! the panel reads too) and nothing else (SD-40 E2.11, `decisions.md §23.1` item 3c: page 1 prints
//! totals only; the breakdown is Show the math's, E3.5). A skill the character put ranks in with no
//! panel row has no total: it prints under the column, with its ranks and `—` (what a feat states on
//! a skill and no total adds prints in the Conditional Bonuses box, E2.20).

use super::{words_of, SheetSources};
use crate::print::layout::{signed, NO_VALUE};
use crate::print::model::{Look, SheetBlock, SheetRow};
use codex::rules_core::equipment_effects::skill_set_rows;
use codex::rules_core::sheet_rule_package;

/// The engine always totals these three, so they print with no rank too.
const ENGINE_TOTAL_PREFIX: &str = "skill.selected_modifier.";
const TOTAL_PREFIX: &str = "skill.total.";
/// The words a `skill.total.<skill>` row states when its total holds a class skill's +3.
const CLASS_SKILL_STATED: &str = "class skill +3";
/// The converted skill record's tag for a skill the armor check penalty applies to (`ACHECK`).
const ARMOR_CHECK_TAG: &str = "ACHECK";

pub fn skills(sources: &SheetSources) -> SheetBlock {
    let loaded = sources.loaded;
    let mut slugs: Vec<String> = loaded
        .skill_allocations
        .iter()
        .filter(|allocation| allocation.ranks > 0)
        .map(|allocation| allocation.skill_id.strip_prefix("skill:").unwrap_or(&allocation.skill_id).to_owned())
        .collect();
    for row in &loaded.explanations {
        // Every Skills panel row has its total (E8.7), ranked or not, and the panel lists them all.
        if let Some(slug) = row.id.strip_prefix(ENGINE_TOTAL_PREFIX).or_else(|| row.id.strip_prefix(TOTAL_PREFIX)) {
            if !slugs.iter().any(|held| held == slug) {
                slugs.push(slug.to_owned());
            }
        }
    }
    let mut lines: Vec<(String, SheetRow)> = Vec::new();
    let mut no_total: Vec<(String, String)> = Vec::new();
    for slug in &slugs {
        let ranks = loaded
            .skill_allocations
            .iter()
            .filter(|allocation| allocation.skill_id.strip_prefix("skill:") == Some(slug.as_str()))
            .map(|allocation| u32::from(allocation.ranks))
            .sum::<u32>();
        let name = skill_name(slug);
        let Some(total) = sources.explanation(&format!("{TOTAL_PREFIX}{slug}")) else {
            no_total.push((name.clone(), format!("{name}: {ranks} ranks, total {NO_VALUE}")));
            continue;
        };
        let row = SheetRow::new(name.clone(), [signed(total.value)]).derived(vec![super::derivation(total)]);
        lines.push((name, row));
    }
    lines.sort_by(|a, b| a.0.cmp(&b.0));
    no_total.sort();
    let mut rows: Vec<SheetRow> = lines.into_iter().map(|(_, row)| row).collect();
    if rows.is_empty() && no_total.is_empty() {
        rows.push(SheetRow::new("No ranks spent.", Vec::<String>::new()));
    }
    if !no_total.is_empty() {
        rows.push(SheetRow::new("Ranks with no Skills panel total:", Vec::<String>::new()).with_notes(no_total.into_iter().map(|(_, line)| line)));
    }
    // What a feat states on a skill and no total adds prints in the Conditional Bonuses box (E2.20).
    SheetBlock::new("Skills", rows).in_look(Look::Skills)
}

/// Show the math's Skills breakdown (SD-40 E3.5, `decisions.md §23.1` item 4): a line per Skills
/// panel row with a total, by name, its cells the ones page 1 dropped at E2.11: the class-skill mark
/// (`X` where the total states a class skill's +3), the total, the key ability and its modifier (the
/// number the Abilities block prints), the ranks and the armor-check mark (`*`).
pub(super) fn skills_breakdown(sources: &SheetSources) -> SheetBlock {
    let loaded = sources.loaded;
    let abilities = loaded.snapshot.as_ref().map(|snapshot| &snapshot.ability_modifiers);
    let mark = |on: bool, text: &str| if on { text.to_owned() } else { String::new() };
    let mut lines: Vec<(String, SheetRow)> = loaded
        .explanations
        .iter()
        .filter_map(|total| total.id.strip_prefix(TOTAL_PREFIX).map(|slug| (slug, total)))
        .map(|(slug, total)| {
            let ranks: u32 = loaded
                .skill_allocations
                .iter()
                .filter(|allocation| allocation.skill_id.strip_prefix("skill:") == Some(slug))
                .map(|allocation| u32::from(allocation.ranks))
                .sum();
            let ability = match key_ability(slug) {
                Some((abbreviation, field)) => {
                    let modifier = abilities.map(|modifiers| match field {
                        "strength" => modifiers.strength,
                        "dexterity" => modifiers.dexterity,
                        "intelligence" => modifiers.intelligence,
                        "wisdom" => modifiers.wisdom,
                        _ => modifiers.charisma,
                    });
                    format!("{abbreviation} {}", modifier.map_or_else(|| NO_VALUE.to_owned(), signed))
                }
                None => NO_VALUE.to_owned(),
            };
            let name = skill_name(slug);
            let row = SheetRow::new(
                name.clone(),
                [
                    mark(total.detail.contains(CLASS_SKILL_STATED), "X"),
                    signed(total.value),
                    ability,
                    ranks.to_string(),
                    mark(armor_check_applies(slug), "*"),
                ],
            );
            (name, row)
        })
        .collect();
    lines.sort_by(|a, b| a.0.cmp(&b.0));
    let mut rows = vec![super::core::captions(&["Class", "Total", "Ability", "Ranks", "Check"])];
    rows.extend(lines.into_iter().map(|(_, row)| row));
    SheetBlock::new("Skill Breakdown", rows).in_look(Look::Breakdown)
}

/// The key ability of a Skills panel row, by the engine's own key-ability sets (the ones the
/// `skill.total` rows add): its abbreviation and its `ability_modifiers` field.
fn key_ability(slug: &str) -> Option<(&'static str, &'static str)> {
    let id = format!("skill:{slug}");
    [("Str", "strength"), ("Dex", "dexterity"), ("Int", "intelligence"), ("Wis", "wisdom"), ("Cha", "charisma")]
        .into_iter()
        .find(|(_, field)| skill_set_rows(field).contains(&id))
}

/// Whether the converted skill record carries the armor check penalty tag. A row with no converted
/// record (Craft, Perform and Profession are one row each, whole) prints no mark.
fn armor_check_applies(slug: &str) -> bool {
    let Ok(package) = sheet_rule_package::package() else { return false };
    package
        .find("skill", slug)
        .and_then(|id| package.rule(id))
        .is_some_and(|rule| rule.tags.iter().any(|tag| tag == ARMOR_CHECK_TAG))
}

/// `knowledge_arcana` reads `Knowledge (Arcana)`, `sleight_of_hand` reads `Sleight of Hand`: the
/// Skills panel's names (`skillsModel.ts`'s `SKILLS`).
pub(crate) fn skill_name(slug: &str) -> String {
    for family in ["knowledge", "craft", "perform", "profession"] {
        if let Some(subtype) = slug.strip_prefix(family).and_then(|rest| rest.strip_prefix('_')) {
            return format!("{} ({})", words_of(family), title_words(subtype));
        }
    }
    title_words(slug)
}

fn title_words(slug: &str) -> String {
    slug.split(['_', ' '])
        .filter(|word| !word.is_empty())
        .enumerate()
        .map(|(index, word)| if index > 0 && word == "of" { word.to_owned() } else { words_of(word) })
        .collect::<Vec<_>>()
        .join(" ")
}
