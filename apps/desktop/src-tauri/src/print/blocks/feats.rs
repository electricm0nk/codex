//! Feats, Class Features and Actions. The words are the converted rules' own ("Rules and
//! features", `sheet_lines`); the numbers are the engine's (`feats.budget.*` rows, a sheet line's
//! printed value). An override tag prints on the feat it let in (E6.2).

use super::core::{class_name, rule_label};
use super::{derivation, words_of, SheetSources};
use crate::character_hub::SheetLineDto;
use crate::character_ledger::AdmittedKind;
use crate::print::model::{Look, SheetBlock, SheetRow};

const BUDGET_PREFIX: &str = "feats.budget.";

/// `feat:power_attack` reads `Power Attack` (the converted record's label); a name saved bare
/// (`Improved Two-Weapon Fighting`) reads as itself.
pub(super) fn feat_label(feat: &str) -> String {
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
    // The budget as one line (E3.1): `<pool> <spent> of <granted>` per pool, in the engine's order;
    // Show the math keeps every row behind it.
    let budget: Vec<_> = loaded.explanations.iter().filter(|row| row.id.starts_with(BUDGET_PREFIX)).collect();
    let mut pools: Vec<&str> = Vec::new();
    for row in &budget {
        let rest = &row.id[BUDGET_PREFIX.len()..];
        let pool = rest.rsplit_once('.').map_or(rest, |(pool, _)| pool);
        if !pools.contains(&pool) {
            pools.push(pool);
        }
    }
    if !pools.is_empty() {
        let field = |pool: &str, field: &str| {
            budget.iter().find(|row| row.id == format!("{BUDGET_PREFIX}{pool}.{field}")).map_or_else(|| "?".to_owned(), |row| row.value.to_string())
        };
        let line = pools.iter().map(|pool| format!("{} {} of {}", pool.replace('_', " "), field(pool, "spent"), field(pool, "granted"))).collect::<Vec<_>>().join(" · ");
        rows.insert(0, SheetRow::new("Feat slots", [line]).of_kind("budget").derived(budget.iter().map(|row| derivation(row)).collect()));
    }
    SheetBlock::new("Feats", rows).in_look(Look::Feats)
}

/// The record families the Class Features panel lists (`classFeaturesModel.ts`), and the suffix of
/// a row it files under "not computed" instead.
const CLASS_RECORD_PREFIXES: [&str; 2] = ["class_feature.", "class_chassis."];
const UNSUPPORTED_SUFFIX: &str = ".unsupported";

/// `sneak_attack` reads `Sneak Attack`; `base_save.will` reads `Base Save Will`: the panel's
/// `humanise`, ported (every `.`, `_` or space splits a word; each word's first letter raised).
fn humanise(segments: &[&str]) -> String {
    segments
        .join(" ")
        .split(|c: char| c == '.' || c == '_' || c.is_whitespace())
        .filter(|word| !word.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            chars.next().map(|first| first.to_uppercase().chain(chars).collect::<String>()).unwrap_or_default()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// A class record's owning class token and its label: the panel's `splitId`, ported. The class is
/// looked for at the first segment after the prefix and one further in (a book's namespace,
/// `class_feature.pu.<class>.…`); a leading `corpus_record` family segment is not part of the label.
fn split_class_record<'a>(id: &'a str, held: &[(String, String)]) -> (Option<&'a str>, String) {
    let after: Vec<&str> = id.split('.').skip(1).collect();
    for index in 0..=1 {
        if index + 1 >= after.len() {
            break;
        }
        if held.iter().any(|(token, _)| token == after[index]) {
            let mut rest = &after[index + 1..];
            if rest.len() > 1 && rest[0] == "corpus_record" {
                rest = &rest[1..];
            }
            return (Some(after[index]), humanise(rest));
        }
    }
    (None, humanise(&after))
}

/// Each held class's id and level, from the summary's `class:fighter:3,class:wizard:1`.
fn held_classes<'a>(sources: &'a SheetSources) -> impl Iterator<Item = (&'a str, &'a str)> {
    sources
        .loaded
        .summary
        .class_summary
        .split(',')
        .map(str::trim)
        .filter(|segment| !segment.is_empty())
        .map(|segment| segment.rsplit_once(':').unwrap_or((segment, "")))
}

/// A line per held class (E3.1): its name and level, then the words of the class's own "Rules and
/// features" line (`Hit die: d10`, `Skill ranks per level: 2`), which it replaces in the list; and the
/// ids of the lines it took.
fn class_lines<'a>(sources: &'a SheetSources) -> (Vec<SheetRow>, Vec<&'a str>) {
    let mut taken = Vec::new();
    let rows = held_classes(sources)
        .map(|(class_id, level)| {
            let name = class_name(class_id);
            let line = sources.loaded.sheet_lines.iter().find(|line| line.kind == "class" && line.form == "words" && line.label == name);
            let words: Vec<String> = line.map(|line| line.prose.split('\n').filter(|said| !said.trim().is_empty()).map(str::to_owned).collect()).unwrap_or_default();
            taken.extend(line.map(|line| line.id.as_str()));
            SheetRow::new(format!("{name} level {level}"), words).of_kind("class")
        })
        .collect();
    (rows, taken)
}

/// Every row the Class Features panel lists, as it lists it: the held class's name (`Chassis` for a
/// record naming none), the label and the engine's value, in the engine's order. The value is the
/// row's own; Show the math prints the row behind it.
fn class_record_rows(sources: &SheetSources) -> Vec<SheetRow> {
    let held: Vec<(String, String)> = held_classes(sources).map(|(class_id, _)| {
        let token = class_id.rsplit(':').next().unwrap_or(class_id);
        (token.to_owned(), class_name(class_id))
    }).collect();
    sources
        .loaded
        .explanations
        .iter()
        .filter(|row| CLASS_RECORD_PREFIXES.iter().any(|prefix| row.id.starts_with(prefix)) && !row.id.ends_with(UNSUPPORTED_SUFFIX))
        .map(|row| {
            let (token, label) = split_class_record(&row.id, &held);
            let class = token
                .and_then(|token| held.iter().find(|(held_token, _)| held_token == token))
                .map_or("Chassis", |(_, name)| name.as_str());
            SheetRow::new(format!("{class}: {label}"), [row.value.to_string()]).derived(vec![derivation(row)])
        })
        .collect()
}

pub fn class_features(sources: &SheetSources) -> SheetBlock {
    let (mut rows, taken) = class_lines(sources);
    rows.extend(class_record_rows(sources));
    rows.extend(
        sources
            .loaded
            .sheet_lines
            .iter()
            .filter(|line| (line.kind == "class" || line.kind == "class_feature") && !taken.contains(&line.id.as_str()))
            .map(line_row),
    );
    // The levels an override let in: no tab lists a level, so the Custom tab and this block do.
    for row in sources.loaded.admitted_under_override.iter().filter(|row| row.kind == AdmittedKind::ClassLevel) {
        rows.push(SheetRow::new(format!("A level of {}", row.id), Vec::<String>::new()).with_notes([super::override_tag(row.override_flag)]));
    }
    if rows.iter().all(|row| row.kind == "class") {
        let reason = sources.loaded.sheet_rules_unavailable_reason.clone().unwrap_or_else(|| "No class feature is held.".to_owned());
        rows.push(SheetRow::new(reason, Vec::<String>::new()));
    }
    SheetBlock::new("Class Features", rows).in_look(Look::ClassFeatures)
}

/// What the character's race lets it do, and the race's rule lines: the Actions tab's Racial
/// Traits section and its race lines.
pub fn actions(sources: &SheetSources) -> SheetBlock {
    let racial = &sources.loaded.resolved_racial_traits;
    let mut rows: Vec<SheetRow> = racial
        .applied_traits
        .iter()
        .map(|applied| SheetRow::new(applied.name.clone(), Vec::<String>::new()).with_notes([applied.description.clone()]).of_kind("Racial trait"))
        .collect();
    rows.extend(
        sources
            .loaded
            .sheet_lines
            .iter()
            .filter(|line| line.kind == "race" || line.kind == "race_trait")
            .map(|line| line_row(line).of_kind(if line.kind == "race" { "Race" } else { "Racial trait" })),
    );
    if rows.is_empty() {
        rows.push(SheetRow::new("No racial trait is held.", Vec::<String>::new()));
    }
    // The table's column captions (E3.1), its first row, as the Weapons table's are.
    rows.insert(0, SheetRow::new("Name", ["Kind".to_owned(), "Value".to_owned()]).of_kind("captions"));
    SheetBlock::new("Actions", rows).in_look(Look::Actions)
}
