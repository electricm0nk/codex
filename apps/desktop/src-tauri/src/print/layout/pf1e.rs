//! The Pathfinder 1e layout file (operator ruling R2; SD-40 `decisions.md §4`): which blocks the
//! sheet prints, in which order, and where on the 12-column page grid. Page 1 holds identity, the
//! Combat block beside the Abilities, and skills; gear, feats, features, spells, custom records,
//! pets and actions follow; then the option blocks, each on pages of its own: Spellbook, Show the
//! math, Bio and campaign notes (R4). The blocks themselves (`print/blocks/`) read what the
//! Character Sheet reads; nothing here computes a number (NR-2).

use crate::print::blocks::{self, SheetSources};
use crate::print::layout::{Placed, NO_VALUE};
use crate::print::model::{SheetBlock, SheetDocument, SheetRow};

/// The sheet's blocks, in print order, each under its vocabulary id (`golden/block_vocabulary.json`).
const SHEET: &[Placed] = &[
    Placed::full("characterDetails", blocks::character_details),
    Placed::at("combatStrip", combat, 0, 6),
    Placed::at("abilities", blocks::abilities, 6, 6),
    Placed::full("skills", blocks::skills),
    Placed::full("weapons", blocks::weapons),
    Placed::full("defense", blocks::armor),
    Placed::full("gear", blocks::inventory),
    Placed::full("feats", blocks::feats),
    Placed::full("classFeatures", blocks::class_features),
    Placed::full("spells", blocks::spells),
    Placed::full("custom", blocks::custom),
    Placed::full("pets", blocks::pets),
    Placed::full("actions", blocks::actions),
];

/// The sheet's blocks, in print order.
pub fn layout(sources: &SheetSources) -> SheetDocument {
    let mut sheet: Vec<SheetBlock> = SHEET.iter().map(|placed| placed.block(sources)).collect();
    let request = sources.request;
    if request.spellbook {
        sheet.push(blocks::spellbook(sources).with_id("spellbook"));
    }
    if request.math {
        let math = blocks::math(&sheet).with_id("math");
        sheet.push(math);
    }
    if request.bio {
        sheet.push(blocks::bio_notes(sources).with_id("bioNotes"));
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
