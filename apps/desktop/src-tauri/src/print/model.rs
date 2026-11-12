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
        let width = usize::from(GRID_COLUMNS);
        // A cell `row_span` rows tall holds its columns in the rows under its own: `below[c]` is how
        // many rows under the current one column `c` is still held, `taken[c]` whether the current
        // row may not use it. A row's blocks fill its free columns left to right.
        let mut below = vec![0u8; width];
        let mut taken = vec![false; width];
        let free_from = |taken: &[bool], from: u16| (from..columns).find(|&c| !taken[usize::from(c)]).unwrap_or(columns);
        let mut cursor: u16 = 0;
        let row_open = |cursor: u16, before: &str| {
            Err(format!("print.layout: the row before {before} ends at column {cursor} of the {columns}-column grid; a row is filled before it ends"))
        };
        for (index, block) in self.blocks.iter().enumerate() {
            let (x, span) = (u16::from(block.place.x), u16::from(block.place.span));
            if span == 0 || x + span > columns {
                return Err(format!(
                    "print.layout: block `{}` ({}) is placed at columns {x}..{}, past the {columns}-column grid",
                    block.id,
                    block.heading,
                    x + span
                ));
            }
            if block.place.under {
                // Stacked under the block before it, in that block's cell: the same columns, no break.
                let above = index.checked_sub(1).map(|before| &self.blocks[before]);
                match above {
                    Some(above) if !block.new_page && above.place.x == block.place.x && above.place.span == block.place.span => continue,
                    Some(above) => {
                        return Err(format!(
                            "print.layout: block `{}` ({}) is stacked under `{}` but is not in its cell (columns {x}..{} against {}..{}, or a page break)",
                            block.id,
                            block.heading,
                            above.id,
                            x + span,
                            above.place.x,
                            above.place.x + above.place.span
                        ))
                    }
                    None => return Err(format!("print.layout: block `{}` ({}) is stacked under no block", block.id, block.heading)),
                }
            }
            while cursor == columns {
                for c in 0..width {
                    taken[c] = below[c] > 0;
                    below[c] = below[c].saturating_sub(1);
                }
                cursor = free_from(&taken, 0);
            }
            if block.new_page && (taken.contains(&true) || below.iter().any(|&rows| rows > 0)) {
                return row_open(cursor, &format!("block `{}`'s page break", block.id));
            }
            if x != cursor {
                return Err(format!(
                    "print.layout: block `{}` ({}) starts at column {x}, but its row's next free column is {cursor}",
                    block.id, block.heading
                ));
            }
            for c in x..x + span {
                taken[usize::from(c)] = true;
                below[usize::from(c)] = block.place.row_span.saturating_sub(1);
            }
            cursor = free_from(&taken, x + span);
        }
        if (cursor != columns && taken.contains(&true)) || below.iter().any(|&rows| rows > 0) {
            return row_open(cursor, "the sheet's end");
        }
        Ok(())
    }
}

/// Which Typst template prints a block (SD-40 `decisions.md §4`): each variant names one
/// `resources/print/looks/<name>.typ`, held in `typst.rs`'s `LOOKS` and dispatched by `blocks.typ`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Look {
    /// The generic table every block printed in before SD-40: a heading and labelled rows.
    #[default]
    Table,
    /// The six ability boxes: score, modifier, and the temp score and temp modifier write-ins.
    Abilities,
    /// The square block (SD-40 E2.9): Hit Points (max, the big current-HP area, nonlethal),
    /// Initiative, Encumbrance and Conditions.
    SquareBlock,
    /// Armor Class, Touch and Flat-Footed boxes over a strip of the AC's component cells.
    ArmorClass,
    /// The Base Attack Bonus, Melee, CMB and CMD boxes.
    Attack,
    /// The three saves, each its total, three to a line (E2.20).
    Saves,
    /// The land speed box.
    Speed,
    /// The skills in two narrow columns: the class-skill box, the name with its armor-check mark,
    /// the total, the key ability, the ranks and the misc write-in.
    Skills,
    /// The weapons table: a line per weapon in hand (attack, damage dice, Str, enhancement, damage
    /// bonus, threat, critical, wield), two blank lines and the ammunition write-in.
    Weapons,
    /// The portrait box (SD-40 E2.5): the saved portrait scaled into a frame, or the empty frame,
    /// with its small "Portrait" caption.
    Portrait,
    /// The Character Name and Campaign fields (the screen's IdentityPanel): each label over its
    /// value, a write-in line where there is none.
    NameCampaign,
    /// The languages (SD-40 E2.13): the known list as one comma list, then one blank write-in line.
    Languages,
    /// The Conditional Bonuses box (SD-40 E2.20): an outlined box of fixed height, a ruled line per
    /// conditional, then blank write-in lines to its foot.
    ConditionalBonuses,
    /// The Resistances block (SD-40 E2.22): five energy boxes, Damage Reduction, Spell Resistance and
    /// Immunities, each empty when the character has none.
    Resistances,
}

impl Look {
    /// Every look, each with its template.
    #[cfg(test)]
    pub const ALL: [Look; 14] = [
        Look::Table,
        Look::Abilities,
        Look::SquareBlock,
        Look::ArmorClass,
        Look::Attack,
        Look::Saves,
        Look::Speed,
        Look::Skills,
        Look::Weapons,
        Look::Portrait,
        Look::NameCampaign,
        Look::Languages,
        Look::ConditionalBonuses,
        Look::Resistances,
    ];

    /// The template's name, as the sheet's JSON and `LOOKS` give it.
    pub fn name(self) -> &'static str {
        match self {
            Look::Table => "table",
            Look::Abilities => "abilities",
            Look::SquareBlock => "square-block",
            Look::ArmorClass => "armor-class",
            Look::Attack => "attack",
            Look::Saves => "saves",
            Look::Speed => "speed",
            Look::Skills => "skills",
            Look::Weapons => "weapons",
            Look::Portrait => "portrait",
            Look::NameCampaign => "name-campaign",
            Look::Languages => "languages",
            Look::ConditionalBonuses => "conditional-bonuses",
            Look::Resistances => "resistances",
        }
    }
}

/// Where a block sits on the page grid: its first column `x` (from 0) and how many columns it spans;
/// `under`: in the cell of the block before it, stacked under that block (the same columns), so a
/// column of short blocks sits beside a tall one; `row_span`: how many grid rows its cell is tall
/// (1; page 1's portrait takes 2, beside the name row and the AC row, SD-40 E2.18).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Place {
    pub x: u8,
    pub span: u8,
    pub under: bool,
    pub row_span: u8,
}

impl Place {
    /// Across the whole page.
    pub const FULL: Place = Place::at(0, GRID_COLUMNS);

    /// From column `x`, `span` columns wide, in a cell of its own.
    pub const fn at(x: u8, span: u8) -> Self {
        Place { x, span, under: false, row_span: 1 }
    }

    /// From column `x`, `span` columns wide, stacked under the block before it in that block's cell.
    pub const fn under(x: u8, span: u8) -> Self {
        Place { x, span, under: true, row_span: 1 }
    }

    /// The same place, its cell `rows` grid rows tall.
    pub const fn spanning_rows(self, rows: u8) -> Self {
        Place { row_span: rows, ..self }
    }
}

impl Default for Place {
    fn default() -> Self {
        Place::FULL
    }
}

/// One block of the sheet (Identity, Abilities, Hit Points ...), printed under its heading.
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
    /// An image the block prints (the portrait box's saved portrait), when it has one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<SheetImage>,
}

/// An image file a block prints: the Typst world serves `bytes` at `path`; `source` is the file
/// it was read from, named in the render's warnings when it cannot be printed.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SheetImage {
    pub path: String,
    #[serde(skip)]
    pub source: String,
    #[serde(skip)]
    pub bytes: Vec<u8>,
}

impl SheetBlock {
    /// A block; every row is padded to the widest row's cell count, so the table keeps its columns.
    pub fn new(heading: impl Into<String>, mut rows: Vec<SheetRow>) -> Self {
        let value_columns = rows.iter().map(|row| row.values.len()).max().unwrap_or(0).max(1);
        for row in &mut rows {
            row.values.resize(value_columns, String::new());
        }
        Self { id: "", look: Look::default(), place: Place::default(), heading: heading.into(), value_columns, new_page: false, rows, image: None }
    }

    /// The same block, under its vocabulary id.
    pub fn with_id(self, id: &'static str) -> Self {
        Self { id, ..self }
    }

    /// The same block, printed in `look`.
    pub fn in_look(self, look: Look) -> Self {
        Self { look, ..self }
    }

    /// The same block, at `place` on the page grid.
    pub fn at(self, place: Place) -> Self {
        Self { place, ..self }
    }

    /// The same block, printing `image`.
    pub fn with_image(self, image: Option<SheetImage>) -> Self {
        Self { image, ..self }
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
