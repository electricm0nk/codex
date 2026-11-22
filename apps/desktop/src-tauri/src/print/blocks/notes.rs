//! The Notes section's block (SD-40 E4.1, `decisions.md §24.9`): the Notes tab's text (`bio.json`'s
//! `notes`, E3.6), a paragraph per line. The `notes` look ends the page with ruled lines, so an empty
//! text prints a page to write on. Pathfinder only.

use super::SheetSources;
use crate::print::model::{Look, SheetBlock, SheetRow};

pub fn notes(sources: &SheetSources) -> SheetBlock {
    let text = SheetRow::new("", Vec::<String>::new()).with_notes(sources.bio.notes.lines().map(str::to_owned));
    SheetBlock::new("", vec![text]).in_look(Look::Notes)
}
