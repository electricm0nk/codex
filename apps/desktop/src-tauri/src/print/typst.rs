//! `SheetDocument` → PDF bytes through Typst used as a library (SD-39 `decisions.md §4`).
//!
//! The Typst world here is closed: its only files are the shared frame and blocks templates and
//! the block looks (`resources/print/*.typ`, `resources/print/looks/*.typ`, compiled into the
//! binary), the sheet's own JSON and the generated main
//! file; its only fonts are the bundled OFL Liberation Sans faces (`resources/fonts/`, compiled
//! in). It never searches the system's fonts, never reads the disk and never asks for the date,
//! so one sheet prints the same bytes on every machine (NR-3).

use std::path::PathBuf;
use std::sync::LazyLock;

use typst::diag::{FileError, FileResult, SourceDiagnostic};
use typst::foundations::{Bytes, Datetime, Duration};
use typst::syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};
use typst_layout::PagedDocument;
use typst_pdf::PdfOptions;

use serde::Serialize;

use crate::print::model::SheetDocument;

/// The shared page frame and the look dispatcher (operator ruling R2; SD-40 `decisions.md §4`).
const FRAME_TYP: &str = include_str!("../../resources/print/frame.typ");
const BLOCKS_TYP: &str = include_str!("../../resources/print/blocks.typ");

/// The block looks (SD-40 `decisions.md §4`): one Typst template per `Look`, by the name the
/// sheet's JSON gives it, served at `/looks/<name>.typ`; `blocks.typ` dispatches on that name.
pub(crate) const LOOKS: &[(&str, &str)] = &[
    ("table", include_str!("../../resources/print/looks/table.typ")),
    ("abilities", include_str!("../../resources/print/looks/abilities.typ")),
    ("hit-points", include_str!("../../resources/print/looks/hit-points.typ")),
    ("initiative", include_str!("../../resources/print/looks/initiative.typ")),
    ("armor-class", include_str!("../../resources/print/looks/armor-class.typ")),
    ("attack", include_str!("../../resources/print/looks/attack.typ")),
    ("saves", include_str!("../../resources/print/looks/saves.typ")),
    ("speed", include_str!("../../resources/print/looks/speed.typ")),
    ("skills", include_str!("../../resources/print/looks/skills.typ")),
];

/// The bundled fonts, and nothing else: Liberation Sans 2.1.5, SIL OFL 1.1 (`resources/fonts/OFL.txt`).
const FONT_FILES: [&[u8]; 4] = [
    include_bytes!("../../resources/fonts/LiberationSans-Regular.ttf"),
    include_bytes!("../../resources/fonts/LiberationSans-Bold.ttf"),
    include_bytes!("../../resources/fonts/LiberationSans-Italic.ttf"),
    include_bytes!("../../resources/fonts/LiberationSans-BoldItalic.ttf"),
];

/// The generated entry point: the frame, given the sheet.
const MAIN_TYP: &str = "#import \"/frame.typ\": sheet\n#sheet(json(\"/sheet.json\"))\n";

static FONTS: LazyLock<Vec<Font>> =
    LazyLock::new(|| FONT_FILES.iter().flat_map(|data| Font::iter(Bytes::new(*data))).collect());

static LIBRARY: LazyLock<LazyHash<Library>> = LazyLock::new(|| LazyHash::new(Library::default()));

static BOOK: LazyLock<LazyHash<FontBook>> = LazyLock::new(|| LazyHash::new(FontBook::from_fonts(FONTS.iter())));

/// The font families the print world can typeset with, sorted and deduplicated.
#[cfg(test)]
pub fn font_families() -> Vec<String> {
    let mut families: Vec<String> = FONTS.iter().map(|font| font.info().family.clone()).collect();
    families.sort();
    families.dedup();
    families
}

fn file_id(path: &str) -> FileId {
    FileId::new(RootedPath::new(
        VirtualRoot::Project,
        VirtualPath::new(path).expect("a print template path is a valid virtual path"),
    ))
}

struct SheetWorld {
    main: Source,
    /// The frame, the dispatcher and every look.
    templates: Vec<Source>,
    sheet_json: Bytes,
}

impl SheetWorld {
    fn new(sheet_json: Vec<u8>) -> Self {
        let mut templates = vec![
            Source::new(file_id("/frame.typ"), FRAME_TYP.to_owned()),
            Source::new(file_id("/blocks.typ"), BLOCKS_TYP.to_owned()),
        ];
        templates.extend(LOOKS.iter().map(|(name, text)| Source::new(file_id(&format!("/looks/{name}.typ")), (*text).to_owned())));
        Self { main: Source::new(file_id("/main.typ"), MAIN_TYP.to_owned()), templates, sheet_json: Bytes::new(sheet_json) }
    }
}

impl World for SheetWorld {
    fn library(&self) -> &LazyHash<Library> {
        &LIBRARY
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &BOOK
    }

    fn main(&self) -> FileId {
        self.main.id()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        std::iter::once(&self.main)
            .chain(self.templates.iter())
            .find(|source| source.id() == id)
            .cloned()
            .ok_or_else(|| not_found(id))
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        if id == file_id("/sheet.json") {
            return Ok(self.sheet_json.clone());
        }
        self.source(id).map(|source| Bytes::from_string(source.text().to_owned()))
    }

    fn font(&self, index: usize) -> Option<Font> {
        FONTS.get(index).cloned()
    }

    fn today(&self, _offset: Option<Duration>) -> Option<Datetime> {
        // No date reaches the sheet: two prints of one character are the same bytes (NR-3).
        None
    }
}

fn not_found(id: FileId) -> FileError {
    FileError::NotFound(PathBuf::from(id.vpath().get_with_slash()))
}

/// A typeset sheet: the PDF, and every warning Typst raised while making it.
#[derive(Debug)]
pub struct TypesetSheet {
    pub pdf: Vec<u8>,
    pub warnings: Vec<String>,
}

fn describe(diagnostics: &[SourceDiagnostic]) -> Vec<String> {
    diagnostics
        .iter()
        .map(|diagnostic| {
            let hints = diagnostic.hints.iter().map(|hint| format!(" (hint: {})", hint.v)).collect::<String>();
            format!("{:?}: {}{hints}", diagnostic.severity, diagnostic.message)
        })
        .collect()
}

/// What the frame is given: the sheet, and the columns it lays its blocks on.
#[derive(Serialize)]
struct SheetJson<'a> {
    #[serde(flatten)]
    document: &'a SheetDocument,
    columns: u8,
}

/// Typesets `document` through the shared frame into PDF bytes. A layout the page grid cannot
/// print as listed is refused by name (`SheetDocument::check_places`); another error is Typst's own words.
pub fn typeset(document: &SheetDocument) -> Result<TypesetSheet, String> {
    document.check_places()?;
    if let Some(block) = document.blocks.iter().find(|block| !LOOKS.iter().any(|(name, _)| *name == block.look.name())) {
        return Err(format!("print.look: block `{}` ({}) names the look `{}`, which has no template", block.id, block.heading, block.look.name()));
    }
    let sheet = SheetJson { document, columns: document.columns() };
    let sheet_json = serde_json::to_vec(&sheet).map_err(|err| format!("the sheet does not serialise: {err}"))?;
    let world = SheetWorld::new(sheet_json);
    let warned = typst::compile::<PagedDocument>(&world);
    let warnings = describe(&warned.warnings);
    let paged = warned
        .output
        .map_err(|errors| format!("the sheet did not typeset: {}", describe(&errors).join("; ")))?;
    let options = PdfOptions { ident: typst::foundations::Smart::Custom(document.title.clone()), ..PdfOptions::default() };
    let pdf = typst_pdf::pdf(&paged, &options)
        .map_err(|errors| format!("the sheet did not export to PDF: {}", describe(&errors).join("; ")))?;
    Ok(TypesetSheet { pdf, warnings })
}
