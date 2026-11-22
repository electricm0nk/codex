//! Progression (SD-40 E3.7, `decisions.md §24.3.7`): the Show Progression drawer's content level by
//! level, read from the rows the drawer reads (`list_progression_ledger_at_root`) and worded as the
//! drawer words them (`characterProgression.ts`'s `describeProgressionLevel`). A count the ledger
//! states prints as it stands; nothing is added up here (NR-2), so the ranks print per skill
//! without the drawer's spent total.

use super::core::class_name;
use super::skills::skill_name;
use super::words_of;
use crate::character_ledger::{ProgressionChoicesDto, ProgressionLedgerDto};
use crate::print::model::{SheetBlock, SheetRow};

/// One row per character level (kind `level`): `Level <n>`, the class and class level, then the
/// drawer's lines; then the rows placed at no level (kind `apart`), when there are any.
pub fn progression(ledger: &ProgressionLedgerDto) -> SheetBlock {
    let mut rows: Vec<SheetRow> = ledger
        .levels
        .iter()
        .map(|level| {
            let mut lines = Vec::new();
            if level.assigned_on_upgrade {
                lines.push("Assigned on upgrade".to_owned());
            }
            if let Some(result) = level.hit_die_result {
                lines.push(format!("Hit die result: {result}"));
            }
            if !level.class_features.is_empty() {
                let features: Vec<String> =
                    level.class_features.iter().map(|row| if row.raised { format!("{} {}", row.label, row.value) } else { row.label.clone() }).collect();
                lines.push(format!("Class features: {}", features.join(", ")));
            }
            lines.extend(choice_lines(&level.taken));
            if level.taken.ability_increases.is_empty() && level.ability_increase_due {
                lines.push("Ability increase: none recorded".to_owned());
            }
            let unspent: Vec<String> = [(level.unspent.feats, "feat"), (level.unspent.spells, "spell"), (level.unspent.ranks, "skill rank")]
                .into_iter()
                .filter(|(count, _)| *count > 0)
                .map(|(count, word)| format!("{count} {word}{}", if count == 1 { "" } else { "s" }))
                .collect();
            if !unspent.is_empty() {
                lines.push(format!("Unspent: {}", unspent.join(", ")));
            }
            SheetRow::new(format!("Level {}", level.character_level), [format!("{} {}", class_name(&level.class_id), level.class_level)])
                .with_notes(lines)
                .of_kind("level")
        })
        .collect();
    let apart = choice_lines(&ledger.without_level);
    if !apart.is_empty() {
        rows.push(SheetRow::new("Not placed at a level", Vec::<String>::new()).with_notes(apart).of_kind("apart"));
    }
    SheetBlock::new("", rows)
}

/// The feats (name, and target in brackets), the spells (the catalog key the Spells tab prints), the
/// ranks per skill and the ability increases a set of ledger rows took.
fn choice_lines(choices: &ProgressionChoicesDto) -> Vec<String> {
    let mut lines = Vec::new();
    if !choices.feats.is_empty() {
        let feats: Vec<String> =
            choices.feats.iter().map(|feat| feat.target.as_ref().map_or_else(|| feat.name.clone(), |target| format!("{} ({target})", feat.name))).collect();
        lines.push(format!("Feats: {}", feats.join(", ")));
    }
    if !choices.spells.is_empty() {
        lines.push(format!("Spells learned: {}", choices.spells.iter().map(|spell| spell.id.as_str()).collect::<Vec<_>>().join(", ")));
    }
    if !choices.ranks.is_empty() {
        let ranks: Vec<String> = choices.ranks.iter().map(|row| format!("{} {}", skill_name(row.skill_id.rsplit(':').next().unwrap_or(&row.skill_id)), row.ranks)).collect();
        lines.push(format!("Skill ranks: {}", ranks.join(", ")));
    }
    if !choices.ability_increases.is_empty() {
        let increases: Vec<String> = choices.ability_increases.iter().map(|row| format!("{} +{}", words_of(&row.ability), row.points)).collect();
        lines.push(format!("Ability increase: {}", increases.join(", ")));
    }
    lines
}
