//! Skills: every skill the Skills panel lists (one `skill.total.<skill>` row each, E8.7), printed in
//! the `skills` look: one column, each row the name and the load response's total (the one value
//! the panel reads too) and nothing else (SD-40 E2.11, `decisions.md §23.1` item 3c: page 1 prints
//! totals only; the breakdown is Show the math's, E3.5). A skill the character put ranks in with no
//! panel row has no total: it prints under the column, with its ranks and `—`, as does what a feat
//! states on a skill and no total adds.

use super::{words_of, SheetSources};
use crate::print::layout::{signed, NO_VALUE};
use crate::print::model::{Look, SheetBlock, SheetRow};

/// The engine always totals these three, so they print with no rank too.
const ENGINE_TOTAL_PREFIX: &str = "skill.selected_modifier.";
const TOTAL_PREFIX: &str = "skill.total.";

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
    // What a feat states on a skill and no total adds: printed with its condition (§25, E4.6).
    let bonuses = &loaded.feat_skill_bonuses;
    let not_added: Vec<String> = bonuses
        .situational
        .iter()
        .chain(&bonuses.unknown)
        .map(|line| format!("{} on {}: {}", line.label, skill_name(&line.skill), line.reason))
        .collect();
    if !not_added.is_empty() {
        rows.push(SheetRow::new("Not added to a total:", Vec::<String>::new()).with_notes(not_added));
    }
    SheetBlock::new("Skills", rows).in_look(Look::Skills)
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
