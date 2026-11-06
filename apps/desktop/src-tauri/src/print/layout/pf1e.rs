//! The Pathfinder 1e layout file (operator ruling R2): which blocks the sheet prints, in which
//! order. Page 1 holds identity, combat, abilities and skills; gear, feats, features, spells,
//! custom records, pets and actions follow; then the option blocks, each on pages of its own:
//! Spellbook, Show the math, Bio and campaign notes (R4). The blocks themselves (`print/blocks/`)
//! read what the Character Sheet reads; nothing here computes a number (NR-2).

use crate::print::blocks::{self, SheetSources};
use crate::print::layout::NO_VALUE;
use crate::print::model::{SheetBlock, SheetDocument, SheetRow};

/// The sheet's blocks, in print order.
pub fn layout(sources: &SheetSources) -> SheetDocument {
    let mut sheet = vec![
        blocks::identity(sources),
        combat(sources),
        blocks::abilities(sources),
        blocks::skills(sources),
        blocks::weapons(sources),
        blocks::armor(sources),
        blocks::inventory(sources),
        blocks::feats(sources),
        blocks::class_features(sources),
        blocks::spells(sources),
        blocks::custom(sources),
        blocks::pets(sources),
        blocks::actions(sources),
    ];
    let request = sources.request;
    if request.spellbook {
        sheet.push(blocks::spellbook(sources));
    }
    if request.math {
        let math = blocks::math(&sheet);
        sheet.push(math);
    }
    if request.bio {
        sheet.push(blocks::bio_notes(sources));
    }
    SheetDocument { title: sources.loaded.summary.display_label.clone(), paper: request.paper.typst_name(), blocks: sheet }
}

/// The Combat block, with the load response's `hit_points.max` first and its `speed.land` last: the
/// one value each that the sheet's Hit Points and Speed tiles show. No row prints a dash.
fn combat(sources: &SheetSources) -> SheetBlock {
    let mut block = blocks::combat(sources);
    let line = |label: &str, id: &str, unit: &str| {
        let value = sources.explanation(id).map_or_else(|| NO_VALUE.to_owned(), |row| format!("{}{unit}", row.value));
        SheetRow::new(label, [value]).derived(sources.derivation_of(id))
    };
    block.rows.insert(0, line("Hit Points", "hit_points.max", ""));
    block.rows.push(line("Speed", "speed.land", " ft."));
    block
}
