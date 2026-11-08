//! The Pathfinder 1e layout file (operator ruling R2; SD-40 `decisions.md §4`): which blocks the
//! sheet prints, in which order, and where on the 12-column page grid. Page 1 holds identity, the
//! Abilities beside the Hit Points, the Armor Class beside the Initiative, the Saving Throws beside
//! the Attack and the Speed (the Combat block split along the screen's boxes, `decisions.md §8`),
//! and the Skills in two narrow columns beside the Weapons (7 and 5 of the 12 columns); gear,
//! feats, features, spells, custom records, pets and actions follow; then the option blocks, each
//! on pages of its own: Spellbook, Show the math, Bio and campaign notes (R4). The blocks
//! themselves (`print/blocks/`) read what the Character Sheet reads; nothing here computes a
//! number (NR-2).

use crate::print::blocks::{self, SheetSources};
use crate::print::layout::Placed;
use crate::print::model::{SheetBlock, SheetDocument};

/// The sheet's blocks, in print order, each under its vocabulary id (`golden/block_vocabulary.json`).
const SHEET: &[Placed] = &[
    Placed::full("characterDetails", blocks::character_details),
    Placed::at("abilities", blocks::abilities, 0, 6),
    Placed::at("hitPoints", blocks::hit_points, 6, 6),
    Placed::at("armorClass", blocks::armor_class, 0, 8),
    Placed::at("initiative", blocks::initiative, 8, 4),
    Placed::at("saves", blocks::saves, 0, 6),
    Placed::at("attack", blocks::attack, 6, 4),
    Placed::at("speed", blocks::speed, 10, 2),
    Placed::at("skills", blocks::skills, 0, 7),
    Placed::at("weapons", blocks::weapons, 7, 5),
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
