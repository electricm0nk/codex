//! The Pathfinder 1e layout file (operator ruling R2; SD-40 `decisions.md §4`): which blocks the
//! sheet prints, in which order, and where on the 12-column page grid. Page 1 (`technical-design.md
//! §4`, Letter first) is the operator's round-2 grid (`decisions.md §24.3.1`, E2.18): the portrait box,
//! two rows tall with the Languages under it, beside the Character Name and Campaign fields (one
//! row, no title above them), then the Armor Class boxes (no heading) and the Speed (Land, Swim, Fly)
//! beside it; then a left column of the square block (Hit Points, Initiative, Encumbrance,
//! Conditions, E2.9), the Weapons, the Attack, the Saving Throws and the Conditional Bonuses box
//! (E2.20), and a right column of one width
//! holding the Abilities over the Skills (one column, name + total, E2.11). Then the sections
//! (`decisions.md §24.3.7`, E2.23), each from a new page under its title: Abilities and Feats,
//! Inventory (Defense, Gear, Money, E3.2), Spellbook (when asked), Bio (the details, then the Biography and campaign notes when asked, then
//! ruled lines, E3.4), Math (when asked: the breakdowns page 1 leaves out, then the derivation lines, E3.5). The blocks
//! themselves (`print/blocks/`) read what the Character Sheet reads; nothing here computes a
//! number (NR-2).

use crate::print::blocks::{self, SheetSources};
use crate::print::layout::Placed;
use crate::print::model::{Look, SheetBlock, SheetDocument};

/// Primary (page 1): its blocks, in print order, each under its vocabulary id (`golden/block_vocabulary.json`).
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
];

/// The sections after Primary (page 1), in the operator's order (`decisions.md §24.3.7`, E2.23): each
/// opens a page under its title (vocabulary id, words) and ends with a page break, so it prints
/// alone. Spellbook and Math print when asked; Bio's notes join its details when asked (E3.4).
const ABILITIES_AND_FEATS: &[Placed] = &[
    Placed::full("feats", blocks::feats),
    Placed::full("classFeatures", blocks::class_features),
    Placed::full("actions", blocks::actions),
    Placed::full("spells", blocks::spells),
    Placed::full("custom", blocks::custom),
    Placed::full("pets", blocks::pets),
];
const INVENTORY: &[Placed] = &[
    Placed::full("defense", blocks::armor),
    Placed::full("gear", blocks::inventory),
    Placed::full("money", blocks::money),
];
/// The Bio page (E3.4): the details, the Biography and campaign notes when asked, ruled lines.
const BIO: &[Placed] = &[Placed::full("characterDetails", blocks::character_details)];

/// A section: its title block opens a page, its blocks follow it with no break of their own (the
/// option blocks' builders break a page for Starfinder's and Cyberpunk RED's sheets).
fn section(sheet: &mut Vec<SheetBlock>, id: &'static str, title: &str, blocks: Vec<SheetBlock>) {
    sheet.push(SheetBlock::new(title, Vec::new()).in_look(Look::SectionTitle).with_id(id).on_new_page());
    sheet.extend(blocks.into_iter().map(|block| SheetBlock { new_page: false, ..block }));
}

/// The sheet's blocks, in print order: Primary, then each section on pages of its own.
pub fn layout(sources: &SheetSources) -> SheetDocument {
    let placed = |list: &[Placed]| list.iter().map(|placed| placed.block(sources)).collect::<Vec<_>>();
    let request = sources.request;
    let mut sheet = placed(SHEET);
    section(&mut sheet, "abilitiesAndFeatsTitle", "Abilities and Feats", placed(ABILITIES_AND_FEATS));
    section(&mut sheet, "inventoryTitle", "Inventory", placed(INVENTORY));
    if request.spellbook {
        section(&mut sheet, "spellbookTitle", "Spellbook", vec![blocks::spellbook(sources).with_id("spellbook")]);
    }
    section(&mut sheet, "bioTitle", "Bio", placed(BIO));
    if request.math {
        // The breakdowns (E3.5) are built from the sources, not from page 1's printed blocks; the
        // derivation lines then follow, in two columns (the `math` look).
        let mut math = blocks::breakdowns(sources);
        math.push(blocks::math(&sheet).in_look(Look::Math).with_id("math"));
        section(&mut sheet, "mathTitle", "Math", math);
    }
    SheetDocument { title: sources.loaded.summary.display_label.clone(), paper: request.paper.typst_name(), blocks: sheet }
}
