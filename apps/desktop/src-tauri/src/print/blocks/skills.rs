//! Skills: each skill the character put ranks in, its ranks, and the total where the engine holds
//! one (`skill.selected_modifier.<skill>`). The Skills panel adds the rest up in TypeScript
//! (`skillsModel.ts`'s `skillModifier`), so print cannot carry that sum without a second
//! derivation (NR-2): it prints `—` there. Discovered card E8.7 gives every skill total one
//! backend value.

use super::{words_of, SheetSources};
use crate::print::layout::{signed, NO_VALUE};
use crate::print::model::{SheetBlock, SheetRow};

const TOTAL_PREFIX: &str = "skill.selected_modifier.";

pub fn skills(sources: &SheetSources) -> SheetBlock {
    let loaded = sources.loaded;
    let mut slugs: Vec<String> = loaded
        .skill_allocations
        .iter()
        .filter(|allocation| allocation.ranks > 0)
        .map(|allocation| allocation.skill_id.strip_prefix("skill:").unwrap_or(&allocation.skill_id).to_owned())
        .collect();
    for row in &loaded.explanations {
        if let Some(slug) = row.id.strip_prefix(TOTAL_PREFIX) {
            if !slugs.iter().any(|held| held == slug) {
                slugs.push(slug.to_owned());
            }
        }
    }
    let mut lines: Vec<(String, SheetRow)> = slugs
        .iter()
        .map(|slug| {
            let ranks = loaded
                .skill_allocations
                .iter()
                .filter(|allocation| allocation.skill_id.strip_prefix("skill:") == Some(slug.as_str()))
                .map(|allocation| u32::from(allocation.ranks))
                .sum::<u32>();
            let total = sources.explanation(&format!("{TOTAL_PREFIX}{slug}"));
            let name = skill_name(slug);
            let row = SheetRow::new(name.clone(), [ranks.to_string(), total.map_or_else(|| NO_VALUE.to_owned(), |row| signed(row.value))]);
            let row = match total {
                Some(row_of_total) => row.derived(vec![super::derivation(row_of_total)]),
                None => row,
            };
            (name, row)
        })
        .collect();
    lines.sort_by(|a, b| a.0.cmp(&b.0));
    let mut rows = vec![SheetRow::new("Skill", ["Ranks".to_owned(), "Total".to_owned()])];
    if lines.is_empty() {
        rows.push(SheetRow::new("No ranks spent.", Vec::<String>::new()));
    }
    rows.extend(lines.into_iter().map(|(_, row)| row));
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
    SheetBlock::new("Skills", rows)
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
