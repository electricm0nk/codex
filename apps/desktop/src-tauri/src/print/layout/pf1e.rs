//! The Pathfinder 1e layout file (operator ruling R2; SD-40 `decisions.md §4`): which blocks the
//! sheet prints, in which order, and where on the 12-column page grid. Page 1 (`technical-design.md
//! §4`, Letter first) is the operator's round-2 grid (`decisions.md §24.3.1`, E2.18): the portrait box,
//! two rows tall with the Languages under it, beside the Character Name and Campaign fields (one
//! row, no title above them), then the Armor Class boxes (no heading) and the Speed (Land, Swim, Fly)
//! beside it; then a left column of the square block (Hit Points, Initiative, Encumbrance,
//! Conditions, E2.9), the Weapons, the Attack, the Saving Throws and the Conditional Bonuses box
//! (E2.20), and a right column of one width
//! holding the Abilities over the Skills (one column, name + total, E2.11). Page 2 starts with the
//! Identity block, then the armor; gear, feats, features, spells, custom records, pets and actions
//! follow; then the option blocks, each on pages of its
//! own: Spellbook, Show the math, Bio and campaign notes (R4). The blocks
//! themselves (`print/blocks/`) read what the Character Sheet reads; nothing here computes a
//! number (NR-2).

use crate::print::blocks::{self, SheetSources};
use crate::print::layout::Placed;
use crate::print::model::{SheetBlock, SheetDocument};

/// The sheet's blocks, in print order, each under its vocabulary id (`golden/block_vocabulary.json`).
const SHEET: &[Placed] = &[
    // Row 1 and row 2 (`decisions.md §24.3.1`): the portrait's cell is two rows tall, so the name
    // row and the AC row sit beside it; the Languages line is stacked in its cell under the box.
    Placed::at("portrait", blocks::portrait, 0, 3).spanning_rows(2),
    Placed::under("languages", blocks::languages, 0, 3),
    Placed::at("nameCampaign", blocks::name_campaign, 3, 9),
    Placed::at("armorClass", blocks::armor_class, 3, 6),
    Placed::at("speed", blocks::speed, 9, 3),
    // The body: the left column (0–6) and the right column (7–11, one width for the Abilities and
    // the one-column Skills, which need 5 of 12, E2.11). The 7 / 5 split is §24.3.1's default.
    Placed::at("squareBlock", blocks::square_block, 0, 7),
    Placed::under("weapons", blocks::weapons, 0, 7),
    Placed::under("attack", blocks::attack, 0, 7),
    Placed::under("saves", blocks::saves, 0, 7),
    Placed::under("conditionalBonuses", blocks::conditional_bonuses, 0, 7),
    Placed::under("resistances", blocks::resistances, 0, 7),
    Placed::at("abilities", blocks::abilities, 7, 5),
    Placed::under("skills", blocks::skills, 7, 5),
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
