//! Show the math (operator ruling R4, `decisions.md §19`): for every computed number the sheet
//! printed, the engine's own rows behind it, verbatim; `(no derivation recorded)` where the
//! engine recorded none. It prints the existing explanations, never a new derivation.

use crate::print::model::{SheetBlock, SheetRow};

pub const NO_DERIVATION: &str = "(no derivation recorded)";

/// One entry per printed number in `printed`, in print order, under its block's name.
pub fn math(printed: &[SheetBlock]) -> SheetBlock {
    let mut rows: Vec<SheetRow> = Vec::new();
    for block in printed {
        // A block printed with no heading (the AC boxes, SD-40 E2.18) goes by its first row's label.
        let name = if block.heading.is_empty() { block.rows.first().map_or("", |row| row.label.as_str()) } else { block.heading.as_str() };
        for row in &block.rows {
            let Some(derivation) = &row.derivation else { continue };
            let value = row.values.iter().filter(|value| !value.is_empty()).cloned().collect::<Vec<_>>().join(" ");
            let notes: Vec<String> = if derivation.is_empty() {
                vec![NO_DERIVATION.to_owned()]
            } else {
                derivation.iter().map(|step| format!("{}: {}", step.source, step.detail)).collect()
            };
            rows.push(SheetRow::new(format!("{} · {}", name, row.label), [value]).with_notes(notes));
        }
    }
    if rows.is_empty() {
        rows.push(SheetRow::new("The sheet prints no computed number.", Vec::<String>::new()));
    }
    SheetBlock::new("Show the Math", rows).on_new_page()
}
