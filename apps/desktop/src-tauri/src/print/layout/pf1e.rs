//! The Pathfinder 1e layout file (operator ruling R2; SD-40 `decisions.md §4`): which blocks the
//! sheet prints, in which order, and where on the 12-column page grid. Page 1 (`technical-design.md
//! §4`, Letter first) opens with the portrait box beside the Character Name and Campaign fields (no
//! title above them, `decisions.md §23.3`), the Speed under those fields and the Armor Class beside
//! them; then the square block (Hit Points, Initiative, Encumbrance, Conditions and their notes,
//! E2.9) beside the Abilities; the Skills in one column (name + total, E2.11) beside the Weapons, with the Attack
//! and the Saving Throws under them, the Languages under the portrait (E2.13) (the Combat block split along
//! the screen's boxes, `decisions.md §8`). Page 2 starts with the Identity block, then the armor; gear, feats, features,
//! spells, custom records, pets and actions follow; then the option blocks, each on pages of its
//! own: Spellbook, Show the math, Bio and campaign notes (R4). The blocks
//! themselves (`print/blocks/`) read what the Character Sheet reads; nothing here computes a
//! number (NR-2).

use crate::print::blocks::{self, SheetSources};
use crate::print::layout::Placed;
use crate::print::model::{SheetBlock, SheetDocument};

/// The sheet's blocks, in print order, each under its vocabulary id (`golden/block_vocabulary.json`).
const SHEET: &[Placed] = &[
    Placed::at("portrait", blocks::portrait, 0, 3),
    // The Skills panel's Languages line (E2.13), one line under the portrait box: the top band's
    // tallest cell is the name column beside it, so the line costs page 1 no height. Every lower
    // cell is full on Letter (the skills column reaches page 1's foot; the Weapons grow per weapon).
    Placed::under("languages", blocks::languages, 0, 3),
    Placed::at("nameCampaign", blocks::name_campaign, 3, 4),
    Placed::under("speed", blocks::speed, 3, 4),
    Placed::at("armorClass", blocks::armor_class, 7, 5),
    Placed::at("squareBlock", blocks::square_block, 0, 7),
    Placed::at("abilities", blocks::abilities, 7, 5),
    // One column of name + total (E2.11) needs 5 columns; the Weapons take the other 7.
    Placed::at("skills", blocks::skills, 0, 5),
    Placed::at("weapons", blocks::weapons, 5, 7),
    Placed::under("attack", blocks::attack, 5, 7),
    Placed::under("saves", blocks::saves, 5, 7),
    // Page 2 opens with the Identity block until the Bio pages take it (E3.4).
    Placed::full("characterDetails", blocks::character_details),
    Placed::full("defense", blocks::armor),
    Placed::full("gear", blocks::inventory),
    Placed::full("feats", blocks::feats),
    Placed::full("classFeatures", blocks::class_features),
    Placed::full("spells", blocks::spells),
    Placed::full("custom", blocks::custom),
    Placed::full("pets", blocks::pets),
    Placed::full("actions", blocks::actions),
];

/// The block page 2 starts with: page 1 holds the portrait, the Character Name and Campaign fields
/// and the step-1 blocks only (`technical-design.md §4`, `decisions.md §23.3`), so no later
/// block's heading is left at page 1's foot.
const PAGE_TWO_STARTS_WITH: &str = "characterDetails";

/// The sheet's blocks, in print order.
pub fn layout(sources: &SheetSources) -> SheetDocument {
    let mut sheet: Vec<SheetBlock> = SHEET
        .iter()
        .map(|placed| {
            let block = placed.block(sources);
            // Page 1 is the step-1 sheet (operator ruling R5); the rest starts page 2.
            if placed.id == PAGE_TWO_STARTS_WITH {
                block.on_new_page()
            } else {
                block
            }
        })
        .collect();
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
