//! `SheetDocument` (SD-39 E8.1, `technical-design.md §1`): the system-agnostic shape a layout file
//! fills and the shared Typst frame prints. It holds text only: every number in it was already
//! computed by the engine and is carried here as the string the sheet shows (NR-2).

use serde::Serialize;

/// The paper a sheet is printed on (operator ruling R3: A4 by default, US Letter as the option).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Paper {
    #[default]
    A4,
    Letter,
}

impl Paper {
    /// Typst's own name for the paper, as `page(paper: ..)` takes it.
    pub fn typst_name(self) -> &'static str {
        match self {
            Paper::A4 => "a4",
            Paper::Letter => "us-letter",
        }
    }
}

/// One whole printed sheet: a title, the paper, and the layout file's blocks in order.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SheetDocument {
    pub title: String,
    pub paper: &'static str,
    pub blocks: Vec<SheetBlock>,
}

/// The page grid's width in columns (SD-40 `decisions.md §4`): a block's `place` is on it.
pub const GRID_COLUMNS: u8 = 12;

impl SheetDocument {
    /// How many columns the sheet lays its blocks on: 1 (each block across the page, one under
    /// the other: the flow every sheet printed in before SD-40, and Starfinder's and Cyberpunk
    /// RED's still) when every block spans the whole page, else the 12-column page grid.
    pub fn columns(&self) -> u8 {
        if self.blocks.iter().all(|block| block.place == Place::FULL) {
            1
        } else {
            GRID_COLUMNS
        }
    }

    /// Refuses a layout the page grid cannot print as listed: a block past the grid's last column,
    /// or one that leaves a gap a later block would fill out of order. Each block starts where the
    /// one before it in its row ended (a full row starts the next at column 0), and a page break
    /// or the sheet's end comes after a full row.
    pub fn check_places(&self) -> Result<(), String> {
        let columns = u16::from(GRID_COLUMNS);
        let mut cursor: u16 = 0;
        let row_open = |cursor: u16, before: &str| {
            Err(format!("print.layout: the row before {before} ends at column {cursor} of the {columns}-column grid; a row is filled before it ends"))
        };
        for block in &self.blocks {
            let (x, span) = (u16::from(block.place.x), u16::from(block.place.span));
            if span == 0 || x + span > columns {
                return Err(format!(
                    "print.layout: block `{}` ({}) is placed at columns {x}..{}, past the {columns}-column grid",
                    block.id,
                    block.heading,
                    x + span
                ));
            }
            if cursor == columns {
                cursor = 0;
            }
            if block.new_page && cursor != 0 {
                return row_open(cursor, &format!("block `{}`'s page break", block.id));
            }
            if x != cursor {
                return Err(format!(
                    "print.layout: block `{}` ({}) starts at column {x}, but its row's next free column is {cursor}",
                    block.id, block.heading
                ));
            }
            cursor = x + span;
        }
        if cursor != 0 && cursor != columns {
            return row_open(cursor, "the sheet's end");
        }
        Ok(())
    }
}

/// Which Typst template prints a block (SD-40 `decisions.md §4`): each variant names one
/// `resources/print/looks/<name>.typ`, held in `typst.rs`'s `LOOKS` and dispatched by `blocks.typ`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Look {
    /// The generic table every block printed in before SD-40: a heading and labelled rows.
    #[default]
    Table,
}

impl Look {
    /// Every look, each with its template.
    #[cfg(test)]
    pub const ALL: [Look; 1] = [Look::Table];

    /// The template's name, as the sheet's JSON and `LOOKS` give it.
    pub fn name(self) -> &'static str {
        match self {
            Look::Table => "table",
        }
    }
}

/// Where a block sits on the page grid: its first column `x` (from 0) and how many columns it spans.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Place {
    pub x: u8,
    pub span: u8,
}

impl Place {
    /// Across the whole page.
    pub const FULL: Place = Place { x: 0, span: GRID_COLUMNS };
}

impl Default for Place {
    fn default() -> Self {
        Place::FULL
    }
}

/// One block of the sheet (Identity, Combat, Abilities ...), printed under its heading.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SheetBlock {
    /// The block's name in the screen's and the paper's one vocabulary (`golden/block_vocabulary.json`,
    /// SD-40 `decisions.md §5`), set by the layout file that places it. Empty on a block no layout
    /// names (the Starfinder and Cyberpunk RED sheets keep their own blocks, R2).
    pub id: &'static str,
    /// The template the block prints in.
    pub look: Look,
    /// Its cell on the page grid; the whole width unless the layout file says otherwise.
    pub place: Place,
    pub heading: String,
    /// How many value cells the widest row has: the table's column count after the label.
    pub value_columns: usize,
    /// The block starts a page of its own: the option blocks (Spellbook, Show the math, Bio and
    /// campaign notes) print after the sheet, never wedged into it.
    pub new_page: bool,
    pub rows: Vec<SheetRow>,
}

impl SheetBlock {
    /// A block; every row is padded to the widest row's cell count, so the table keeps its columns.
    pub fn new(heading: impl Into<String>, mut rows: Vec<SheetRow>) -> Self {
        let value_columns = rows.iter().map(|row| row.values.len()).max().unwrap_or(0).max(1);
        for row in &mut rows {
            row.values.resize(value_columns, String::new());
        }
        Self { id: "", look: Look::default(), place: Place::default(), heading: heading.into(), value_columns, new_page: false, rows }
    }

    /// The same block, under its vocabulary id.
    pub fn with_id(self, id: &'static str) -> Self {
        Self { id, ..self }
    }

    /// The same block, at `place` on the page grid.
    pub fn at(self, place: Place) -> Self {
        Self { place, ..self }
    }

    /// The same block, starting on a new page.
    pub fn on_new_page(self) -> Self {
        Self { new_page: true, ..self }
    }
}

/// One labelled line of a block. `values` are the cells after the label, as the sheet shows them
/// (an ability prints its score and its modifier). `notes` print under the line, the width of the
/// block: a rule's words, a condition, an override tag.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SheetRow {
    pub label: String,
    pub values: Vec<String>,
    pub notes: Vec<String>,
    /// For a line that prints a computed number: the engine's rows behind it, which "Show the math"
    /// prints (empty: the engine recorded none). `None` for a line of words or of the player's own
    /// choices (ranks, scores), which no derivation is behind. Not sent to Typst.
    #[serde(skip)]
    pub derivation: Option<Vec<Derivation>>,
}

impl SheetRow {
    pub fn new(label: impl Into<String>, values: impl IntoIterator<Item = String>) -> Self {
        Self { label: label.into(), values: values.into_iter().collect(), notes: Vec::new(), derivation: None }
    }

    /// The same line, with `notes` printed under it.
    pub fn with_notes(mut self, notes: impl IntoIterator<Item = String>) -> Self {
        self.notes.extend(notes.into_iter().filter(|note| !note.trim().is_empty()));
        self
    }

    /// The same line, marked as a computed number with these rows behind it.
    pub fn derived(self, derivation: Vec<Derivation>) -> Self {
        Self { derivation: Some(derivation), ..self }
    }
}

/// One engine record behind a printed number: where it is from (an `explanations` row id and its
/// value) and the engine's own words, verbatim.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Derivation {
    pub source: String,
    pub detail: String,
}
