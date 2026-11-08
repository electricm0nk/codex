//! Skills: every skill the Skills panel lists (one `skill.total.<skill>` row each, E8.7), every skill
//! the character put ranks in, and Climb, Intimidate and Swim, which the engine always totals: its
//! ranks and the load response's total, the one value the panel reads too. A ranked skill with no
//! panel row has no total: `—`.

use super::{words_of, SheetSources};
use crate::print::layout::{signed, NO_VALUE};
use crate::print::model::{SheetBlock, SheetRow};

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
