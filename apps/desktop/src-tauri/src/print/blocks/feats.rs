//! Feats, Class Features and Actions. The words are the converted rules' own ("Rules and
//! features", `sheet_lines`); the numbers are the engine's (`feats.budget.*` rows, a sheet line's
//! printed value). An override tag prints on the feat it let in (E6.2).

use super::core::rule_label;
use super::{derivation, words_of, SheetSources};
use crate::character_hub::SheetLineDto;
use crate::character_ledger::AdmittedKind;
use crate::print::model::{SheetBlock, SheetRow};

const BUDGET_PREFIX: &str = "feats.budget.";

/// `feat:power_attack` reads `Power Attack` (the converted record's label); a name saved bare
/// (`Improved Two-Weapon Fighting`) reads as itself.
fn feat_label(feat: &str) -> String {
    match feat.strip_prefix("feat:") {
        Some(slug) => rule_label("feat", slug).unwrap_or_else(|| words_of(slug)),
        None => feat.to_owned(),
    }
}

/// A sheet line's printed value and its other numbers: `+2`, `1d8`, `6/day`.
fn line_values(line: &SheetLineDto) -> Vec<String> {
    let mut values: Vec<String> = Vec::new();
    if !line.value.is_empty() {
        values.push(line.value.clone());
    }
    if !line.also.is_empty() {
        values.push(line.also.join(", "));
    }
    values
}

/// A sheet line as a row: its label and values, its condition and words under it. A number line
/// is a computed value; its derivation is the rule itself, which no `explanations` row records.
pub(crate) fn line_row(line: &SheetLineDto) -> SheetRow {
    let row = SheetRow::new(line.label.clone(), line_values(line))
        .with_notes(line.condition.iter().map(|condition| format!("Condition: {condition}")))
        .with_notes(line.prose.split('\n').map(str::to_owned));
    if line.form == "number" {
        row.derived(Vec::new())
    } else {
        row
    }
}

pub fn feats(sources: &SheetSources) -> SheetBlock {
    let loaded = sources.loaded;
    let mut rows: Vec<SheetRow> = loaded
        .selected_feats
        .iter()
        .map(|feat| {
            let label = feat_label(feat);
            let targets = loaded
                .chosen_feat_targets
                .iter()
                .find(|chosen| &chosen.feat_id == feat && !chosen.targets.is_empty())
                .map(|chosen| format!(" ({})", chosen.targets.join(", ")))
                .unwrap_or_default();
            let words = loaded
                .sheet_lines
                .iter()
                .find(|line| line.kind == "feat" && line.label == label)
                .map(|line| line.prose.split('\n').map(str::to_owned).collect::<Vec<_>>())
                .unwrap_or_default();
            SheetRow::new(format!("{label}{targets}"), Vec::<String>::new())
                .with_notes(sources.override_tags(AdmittedKind::Feat, feat))
                .with_notes(words)
        })
        .collect();
    if rows.is_empty() {
        rows.push(SheetRow::new("No feat is held.", Vec::<String>::new()));
    }
    for row in loaded.explanations.iter().filter(|row| row.id.starts_with(BUDGET_PREFIX)) {
        let rest = &row.id[BUDGET_PREFIX.len()..];
        let (pool, field) = rest.rsplit_once('.').unwrap_or((rest, ""));
        rows.push(
            SheetRow::new(format!("Feat slots: {} {field}", pool.replace('_', " ")), [row.value.to_string()])
                .derived(vec![derivation(row)]),
        );
    }
    SheetBlock::new("Feats", rows)
}

pub fn class_features(sources: &SheetSources) -> SheetBlock {
    let mut rows: Vec<SheetRow> = sources
        .loaded
        .sheet_lines
        .iter()
        .filter(|line| line.kind == "class" || line.kind == "class_feature")
        .map(line_row)
        .collect();
    // The levels an override let in: no tab lists a level, so the Custom tab and this block do.
    for row in sources.loaded.admitted_under_override.iter().filter(|row| row.kind == AdmittedKind::ClassLevel) {
        rows.push(SheetRow::new(format!("A level of {}", row.id), Vec::<String>::new()).with_notes([super::override_tag(row.override_flag)]));
    }
    if rows.is_empty() {
        let reason = sources.loaded.sheet_rules_unavailable_reason.clone().unwrap_or_else(|| "No class feature is held.".to_owned());
        rows.push(SheetRow::new(reason, Vec::<String>::new()));
    }
    SheetBlock::new("Class Features", rows)
}

/// What the character's race lets it do, and the race's rule lines: the Actions tab's Racial
/// Traits section and its race lines.
pub fn actions(sources: &SheetSources) -> SheetBlock {
    let racial = &sources.loaded.resolved_racial_traits;
    let mut rows: Vec<SheetRow> = racial
        .applied_traits
        .iter()
        .map(|applied| SheetRow::new(applied.name.clone(), Vec::<String>::new()).with_notes([applied.description.clone()]))
        .collect();
    rows.extend(
        sources
            .loaded
            .sheet_lines
            .iter()
            .filter(|line| line.kind == "race" || line.kind == "race_trait")
            .map(line_row),
    );
    if rows.is_empty() {
        rows.push(SheetRow::new("No racial trait is held.", Vec::<String>::new()));
    }
    SheetBlock::new("Actions", rows)
}
