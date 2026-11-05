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

/// One block of the sheet (Identity, Combat, Abilities ...), printed under its heading.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SheetBlock {
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
        Self { heading: heading.into(), value_columns, new_page: false, rows }
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
