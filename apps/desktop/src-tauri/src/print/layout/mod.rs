//! The per-system layout files (operator ruling R2): which blocks a system's sheet prints, in which
//! order, with which labels. E8.1 holds Pathfinder's; E8.3 adds Starfinder's and Cyberpunk RED's.

pub mod cpr;
pub mod pf1e;
pub mod sf1e;

/// A value the sheet shows with its sign, as the screen's tiles write it: `+3`, `0` stays `+0`, `-1`.
/// Formatting only; the number is the engine's.
pub(crate) fn signed(value: i16) -> String {
    if value >= 0 {
        format!("+{value}")
    } else {
        value.to_string()
    }
}

/// What a cell prints when the engine holds no value for it: a dash, never a guessed number.
pub(crate) const NO_VALUE: &str = "—";

/// One entry of a layout (SD-40 `decisions.md §4`): a block's vocabulary id, its builder, and its
/// place on the page grid. A layout is an ordered `&[Placed]`; a print style is one such list.
pub(crate) struct Placed {
    pub id: &'static str,
    pub build: fn(&SheetSources) -> SheetBlock,
    pub place: Place,
}

impl Placed {
    /// A block across the whole page.
    pub const fn full(id: &'static str, build: fn(&SheetSources) -> SheetBlock) -> Self {
        Self { id, build, place: Place::FULL }
    }

    /// A block from column `x`, `span` columns wide.
    pub const fn at(id: &'static str, build: fn(&SheetSources) -> SheetBlock, x: u8, span: u8) -> Self {
        Self { id, build, place: Place { x, span } }
    }

    /// The block, built over `sources`, under its id at its place (its look the builder's).
    pub fn block(&self, sources: &SheetSources) -> SheetBlock {
        (self.build)(sources).with_id(self.id).at(self.place)
    }
}

// The Starfinder and Cyberpunk RED sheets (E8.3) lay out the engine's explanation rows and printed
// lines the way their screens do (`starfinderSheetModel.ts`, `cyberpunkSheetModel.ts`): every
// number is one row's value, a row no block names prints under "Other totals" and a line no block
// places under "Rules and Features", so nothing the engine sent is dropped.

use std::collections::BTreeSet;

use crate::character_hub::{DiagnosticDto, ExplanationDto, LoadSavedCharacterResponse, SheetLineDto};
use crate::print::blocks::{self, derivation, line_row, SheetSources};
use crate::print::model::{Derivation, Place, SheetBlock, SheetRow};

/// One cell: an engine row's value (as the screen formats it), an engine printed line's text
/// (dice, a name, words), or nothing.
pub(crate) enum Cell<'a> {
    Row(String, &'a ExplanationDto),
    Printed(String),
    Missing,
}

impl Cell<'_> {
    fn text(&self) -> String {
        match self {
            Cell::Row(text, _) | Cell::Printed(text) => text.clone(),
            Cell::Missing => NO_VALUE.to_owned(),
        }
    }

    fn is_missing(&self) -> bool {
        matches!(self, Cell::Missing)
    }
}

/// The rows a printed figure stands on: `Some` when any cell is an engine row (Show the math
/// prints them), `None` for printed words and names.
fn derivation_of(cells: &[Cell]) -> Option<Vec<Derivation>> {
    let rows: Vec<Derivation> = cells
        .iter()
        .filter_map(|cell| match cell {
            Cell::Row(_, row) => Some(derivation(row)),
            _ => None,
        })
        .collect();
    (!rows.is_empty()).then_some(rows)
}

/// What a layout has taken from the load response: each engine row is printed once.
pub(crate) struct Placement<'a> {
    pub loaded: &'a LoadSavedCharacterResponse,
    /// The printed lines in play (a Cyberpunk RED build that did not compute lays out none).
    pub lines: Vec<&'a SheetLineDto>,
    placed_rows: BTreeSet<&'a str>,
    placed_lines: BTreeSet<&'a str>,
}

impl<'a> Placement<'a> {
    pub fn new(loaded: &'a LoadSavedCharacterResponse, lines_in_play: bool) -> Self {
        let lines = if lines_in_play { loaded.sheet_lines.iter().collect() } else { Vec::new() };
        Self { loaded, lines, placed_rows: BTreeSet::new(), placed_lines: BTreeSet::new() }
    }

    /// The ids of the engine rows, in the engine's order.
    pub fn row_ids(&self) -> impl Iterator<Item = &'a str> {
        self.loaded.explanations.iter().map(|row| row.id.as_str())
    }

    pub fn row(&self, id: &str) -> Option<&'a ExplanationDto> {
        self.loaded.explanations.iter().find(|row| row.id == id)
    }

    pub fn line(&self, id: &str) -> Option<&'a SheetLineDto> {
        self.lines.iter().copied().find(|line| line.id == id)
    }

    /// The engine row `id`, once: its value, signed when the screen signs it.
    pub fn cell(&mut self, id: &str, with_sign: bool) -> Cell<'a> {
        match self.row(id) {
            Some(row) if !self.placed_rows.contains(row.id.as_str()) => {
                self.placed_rows.insert(row.id.as_str());
                let text = if with_sign { signed(row.value) } else { row.value.to_string() };
                Cell::Row(text, row)
            }
            _ => Cell::Missing,
        }
    }

    /// The engine row `id` when the engine computed one, else its printed line: the line's value,
    /// or its words when it has none (dice, a name, "N/A").
    pub fn figure(&mut self, id: &str) -> Cell<'a> {
        let computed = self.cell(id, false);
        if !computed.is_missing() {
            return computed;
        }
        match self.line(id) {
            Some(line) => {
                self.placed_lines.insert(line.id.as_str());
                Cell::Printed(if line.value.is_empty() { line.prose.clone() } else { line.value.clone() })
            }
            None => Cell::Missing,
        }
    }

    /// The engine rows no block named, in the engine's order.
    pub fn unplaced_rows(&self) -> Vec<&'a ExplanationDto> {
        self.loaded.explanations.iter().filter(|row| !self.placed_rows.contains(row.id.as_str())).collect()
    }

    /// The printed lines no block placed, in the engine's order.
    pub fn unplaced_lines(&self) -> Vec<&'a SheetLineDto> {
        self.lines.iter().copied().filter(|line| !self.placed_lines.contains(line.id.as_str())).collect()
    }
}

/// A row of one value per column; `None` when every cell is missing (the screen leaves it out).
pub(crate) fn cells_row(label: impl Into<String>, cells: Vec<Cell>) -> Option<SheetRow> {
    if cells.iter().all(Cell::is_missing) {
        return None;
    }
    let derivation = derivation_of(&cells);
    let row = SheetRow::new(label, cells.iter().map(Cell::text));
    Some(match derivation {
        Some(rows) => row.derived(rows),
        None => row,
    })
}

/// A row whose cells read as one figure, `current / max`, as the screen writes it.
pub(crate) fn joined_row(label: impl Into<String>, cells: Vec<Cell>) -> Option<SheetRow> {
    if cells.iter().all(Cell::is_missing) {
        return None;
    }
    let derivation = derivation_of(&cells);
    let row = SheetRow::new(label, [cells.iter().map(Cell::text).collect::<Vec<_>>().join(" / ")]);
    Some(match derivation {
        Some(rows) => row.derived(rows),
        None => row,
    })
}

/// A heading row for a group of rows (a skill category, a Role); no values.
pub(crate) fn group_row(name: impl Into<String>) -> SheetRow {
    SheetRow::new(name, Vec::<String>::new())
}

/// The rows of each group that hold a value, each group under its heading row; a group with
/// none is left out.
pub(crate) fn grouped(groups: Vec<(String, Vec<Option<SheetRow>>)>) -> Vec<Option<SheetRow>> {
    let mut out: Vec<Option<SheetRow>> = Vec::new();
    for (name, rows) in groups {
        let kept: Vec<SheetRow> = rows.into_iter().flatten().collect();
        if !kept.is_empty() {
            out.push(Some(group_row(name)));
            out.extend(kept.into_iter().map(Some));
        }
    }
    out
}

/// A block of the rows that hold a value, under a row naming the columns when there are two or
/// more (the screen's table head); `None` when no row holds one (the screen shows no box).
pub(crate) fn block(heading: &str, columns: &[&str], rows: Vec<Option<SheetRow>>) -> Option<SheetBlock> {
    let kept: Vec<SheetRow> = rows.into_iter().flatten().collect();
    if kept.is_empty() {
        return None;
    }
    let mut all = Vec::with_capacity(kept.len() + 1);
    if columns.len() >= 2 {
        all.push(SheetRow::new("", columns.iter().map(|column| (*column).to_owned())));
    }
    all.extend(kept);
    Some(SheetBlock::new(heading, all))
}

/// "This character did not compute": the diagnostics that stop the build, as the screen lists them.
pub(crate) fn did_not_compute(diagnostics: &[DiagnosticDto]) -> Option<SheetBlock> {
    let rows: Vec<Option<SheetRow>> =
        diagnostics.iter().filter(|d| d.claim_blocking).map(|d| Some(SheetRow::new(d.message.clone(), Vec::<String>::new()))).collect();
    block("This Character Did Not Compute", &[], rows)
}

/// The other diagnostics the screen shows under the boxes (`skip`: ids the screen does not show).
pub(crate) fn notes(diagnostics: &[DiagnosticDto], skip: &[&str]) -> Option<SheetBlock> {
    let rows: Vec<Option<SheetRow>> = diagnostics
        .iter()
        .filter(|d| !d.claim_blocking && !skip.contains(&d.id.as_str()))
        .map(|d| Some(SheetRow::new(d.message.clone(), Vec::<String>::new())))
        .collect();
    block("Notes", &[], rows)
}

/// "Other totals": every engine row no block named, under its id (the screens' rule).
pub(crate) fn other_totals(placed: &mut Placement) -> Option<SheetBlock> {
    let ids: Vec<&str> = placed.unplaced_rows().into_iter().map(|row| row.id.as_str()).collect();
    let rows = ids.into_iter().map(|id| cells_row(id, vec![placed.cell(id, false)])).collect();
    block("Other Totals", &[], rows)
}

/// The kind headings of the "Rules and features" section (`CharacterSheet.tsx`'s
/// `SHEET_LINE_KIND_LABELS`); another kind reads as its words.
fn kind_label(kind: &str) -> String {
    let named = match kind {
        "ability" => "Abilities",
        "class" => "Classes",
        "class_feature" => "Class features",
        "companion" => "Companions",
        "deity" => "Deities",
        "domain" => "Domains",
        "equipment" => "Equipment",
        "equipment_modifier" => "Equipment modifiers",
        "feat" => "Feats",
        "language" => "Languages",
        "monster" => "Monsters",
        "monster_ability" => "Monster abilities",
        "power" => "Powers",
        "race" => "Races",
        "race_trait" => "Racial traits",
        "skill" => "Skills",
        "spell" => "Spells",
        "template" => "Templates",
        "trait" => "Traits",
        _ => return blocks::words_of(kind),
    };
    named.to_owned()
}

/// "Rules and features": the printed lines, one block per run of one kind, in the engine's order
/// (`groupSheetLinesByKind`); each line its label, its value and its words. When the rules could
/// not be read, the reason, as the screen says it.
pub(crate) fn rules_and_features(lines: &[&SheetLineDto], unavailable: Option<&str>) -> Vec<SheetBlock> {
    if let Some(reason) = unavailable {
        return vec![SheetBlock::new("Rules and Features", vec![SheetRow::new(reason, Vec::<String>::new())])];
    }
    let mut groups: Vec<(&str, Vec<SheetRow>)> = Vec::new();
    for line in lines {
        match groups.last_mut() {
            Some((kind, rows)) if *kind == line.kind => rows.push(line_row(line)),
            _ => groups.push((line.kind.as_str(), vec![line_row(line)])),
        }
    }
    groups.into_iter().map(|(kind, rows)| SheetBlock::new(format!("Rules and Features: {}", kind_label(kind)), rows)).collect()
}

/// The option blocks after the sheet, each on pages of its own (operator ruling R4): the
/// system's Spellbook, Show the math over what the sheet printed, Bio and campaign notes.
pub(crate) fn with_options(mut sheet: Vec<SheetBlock>, spellbook: impl FnOnce() -> SheetBlock, sources: &SheetSources) -> Vec<SheetBlock> {
    let request = sources.request;
    if request.spellbook {
        sheet.push(spellbook());
    }
    if request.math {
        let math = blocks::math(&sheet);
        sheet.push(math);
    }
    if request.bio {
        sheet.push(blocks::bio_notes(sources));
    }
    sheet
}

/// `life_science` reads `Life Science` (the SF and CPR screens' `humanise`).
pub(crate) fn humanise(slug: &str) -> String {
    slug.split('_')
        .filter(|word| !word.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
