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
    pub rows: Vec<SheetRow>,
}

impl SheetBlock {
    pub fn new(heading: impl Into<String>, rows: Vec<SheetRow>) -> Self {
        let value_columns = rows.iter().map(|row| row.values.len()).max().unwrap_or(0).max(1);
        Self { heading: heading.into(), value_columns, rows }
    }
}

/// One labelled line of a block. `values` are the cells after the label, as the sheet shows them
/// (an ability prints its score and its modifier).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SheetRow {
    pub label: String,
    pub values: Vec<String>,
}

impl SheetRow {
    pub fn new(label: impl Into<String>, values: impl IntoIterator<Item = String>) -> Self {
        Self { label: label.into(), values: values.into_iter().collect() }
    }
}
