//! `print_golden` (SD-39 E8.5, gate G-10, operator ruling R6): the golden suite. One character per
//! system -- Pathfinder's Elowen (Wizard 5), Starfinder's SF-Mystic-5, Cyberpunk RED's
//! cpr-netrunner-decked -- prints through the real print path on both papers with each of the four
//! option sets: none, Spellbook, Show the math, and all three. 3 x 2 x 4 = 24 PDFs, one test each.
//!
//! Each test asserts, on the PDF itself:
//! - **Typst diagnostics 0** (a sheet that overflows or misses a font warns; NR-4);
//! - **every page the paper's size** (MediaBox);
//! - **page count** equal to the golden fixture's (`golden/print_golden.json`);
//! - **headings in layout order**: the layout file's headings equal the fixture's list, and each
//!   is found in the PDF's text after the one before it;
//! - **value presence**: every label, value and note the layout file put in a block is in the PDF's
//!   text between that block's heading and the next one, so nothing is clipped or dropped;
//! - **determinism**: a second render of the same request is byte-equal (NR-3).
//!
//! It does not compare pixels (DEF-4): a block that prints its words in the wrong place on the page
//! passes, as long as the words keep their block and their order of headings.
//!
//! Run (from `apps/desktop/src-tauri`):
//!
//! ```text
//! cargo test --locked -j 8 print_golden -- --test-threads=8
//! ```
//!
//! With `PRINT_GOLDEN_OUT` set (a directory outside the repo), each test keeps its PDF there as
//! `<case>.pdf`.

use std::path::{Path, PathBuf};

use codex::rules_core::cpr::store::CprCharacterStore;
use codex::saved_character::local_store::SavedCharacterStore;
use codex::saved_character::{SavedCharacterEnvelope, SavedCharacterRevisionKind, CURRENT_SAVED_CHARACTER_SCHEMA_VERSION};

use crate::character_hub::seed_default_characters_at;
use crate::print::model::SheetDocument;
use crate::print::{render_sheet_pdf_at_root, sheet_document_at_root, CampaignNote, Paper, PrintRequest};
use crate::sf_adapter::STARFINDER_RULE_SYSTEM_ID;

/// Per case (`<system>-<paper>-<options>`): the page count and the headings in print order.
const GOLDEN: &str = include_str!("golden/print_golden.json");

/// Elowen Brightwater, the second Pathfinder starter seed (Wizard 5): the PF character with spells.
const ELOWEN_ID: &str = "00000000-0000-0000-0000-000000000002";
const SF_SEED: &str = "SF-Mystic-5";
const CPR_FIXTURE: &str = "cpr-netrunner-decked";

#[derive(Clone, Copy, Debug)]
enum System {
    Pf1e,
    Sf1e,
    Cpr,
}

#[derive(Clone, Copy, Debug)]
enum Options {
    None,
    Spellbook,
    Math,
    All,
}

impl Options {
    fn name(self) -> &'static str {
        match self {
            Options::None => "none",
            Options::Spellbook => "spellbook",
            Options::Math => "math",
            Options::All => "all",
        }
    }
}

fn case_name(system: System, paper: Paper, options: Options) -> String {
    let system = match system {
        System::Pf1e => "pf1e",
        System::Sf1e => "sf1e",
        System::Cpr => "cpr",
    };
    let paper = match paper {
        Paper::A4 => "a4",
        Paper::Letter => "letter",
    };
    format!("{system}-{paper}-{}", options.name())
}

fn request(paper: Paper, options: Options) -> PrintRequest {
    let (spellbook, math, bio) = match options {
        Options::None => (false, false, false),
        Options::Spellbook => (true, false, false),
        Options::Math => (false, true, false),
        Options::All => (true, true, true),
    };
    let campaigns = if bio {
        vec![CampaignNote { name: "The Golden Road".to_owned(), description: "Three sheets, two papers, four option sets.".to_owned() }]
    } else {
        Vec::new()
    };
    PrintRequest { paper, spellbook, math, bio, campaigns }
}

/// A fresh directory holding the system's character, saved the way the app saves it; returns its root.
fn saved_character(system: System, case: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("codex-print-golden-{}-{case}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("the temp directory is creatable");
    match system {
        System::Pf1e => {
            seed_default_characters_at(&dir, "0.0.0-test").expect("starter seeding succeeds");
            dir.join("characters").join(ELOWEN_ID)
        }
        System::Sf1e => {
            let (seed, _, input) = crate::sf_adapter::tests::seeds()
                .into_iter()
                .find(|(seed, _, _)| *seed == SF_SEED)
                .expect("the Mystic seed");
            let root = dir.join(seed);
            let envelope = SavedCharacterEnvelope {
                character_id: seed.to_owned(),
                revision_id: format!("{seed}.rev.1"),
                revision_kind: SavedCharacterRevisionKind::Authoritative,
                saved_at: "2026-11-05T00:00:00Z".to_owned(),
                schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
                app_or_runtime_version: "codex-dev".to_owned(),
                content_or_rules_provenance: STARFINDER_RULE_SYSTEM_ID.to_owned(),
                game_system: STARFINDER_RULE_SYSTEM_ID.to_owned(),
                latest_authoritative_revision_ref: format!("{seed}.rev.1"),
                display_label: format!("{seed} printed"),
                character_input: input,
            };
            SavedCharacterStore::save(&envelope, &root).expect("the seed saves");
            root
        }
        System::Cpr => {
            let (character, choices) = crate::cpr_adapter::tests::sheet_fixture_characters()
                .into_iter()
                .find(|(character, _)| character.character_id == CPR_FIXTURE)
                .expect("the decked Netrunner fixture");
            let root = dir.join(CPR_FIXTURE);
            CprCharacterStore::save(&character, &root).expect("the character saves");
            CprCharacterStore::save_choices(&choices, &root).expect("its choices save");
            if let Some(lifepath) = crate::cpr_adapter::tests::sheet_fixture_lifepath(CPR_FIXTURE) {
                CprCharacterStore::save_lifepath(&lifepath, &root).expect("its Lifepath saves");
            }
            root
        }
    }
}

/// Folds every run of whitespace to one space, as the PDF's text is folded.
fn folded(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn pdf_text(bytes: &[u8]) -> String {
    folded(&pdf_extract::extract_text_from_mem(bytes).expect("the PDF's text extracts"))
}

/// Every page's `/MediaBox` width and height, in PDF points.
fn page_sizes(bytes: &[u8]) -> Vec<(f32, f32)> {
    let document = lopdf::Document::load_mem(bytes).expect("the PDF parses");
    document
        .get_pages()
        .values()
        .map(|page_id| {
            let page = document.get_dictionary(*page_id).expect("a page is a dictionary");
            let numbers: Vec<f32> = page
                .get(b"MediaBox")
                .expect("a page has a MediaBox")
                .as_array()
                .expect("MediaBox is an array")
                .iter()
                .map(|n| n.as_float().or_else(|_| n.as_i64().map(|i| i as f32)).expect("a MediaBox entry is a number"))
                .collect();
            (numbers[2] - numbers[0], numbers[3] - numbers[1])
        })
        .collect()
}

fn render(root: &Path, request: &PrintRequest, case: &str) -> Vec<u8> {
    let rendered = render_sheet_pdf_at_root(root, request).unwrap_or_else(|err| panic!("{case}: renders: {err}"));
    assert!(rendered.warnings.is_empty(), "{case}: Typst reports no diagnostics: {:?}", rendered.warnings);
    rendered.pdf
}

/// The fixture's entry for `case`: (page count, headings). A case with no entry fails, naming the
/// entry this render would record, so a reviewer reads the change before it is pasted in.
fn golden(case: &str, actual_pages: usize, actual_headings: &[String]) -> (usize, Vec<String>) {
    let all: serde_json::Value = serde_json::from_str(GOLDEN).expect("the golden fixture is JSON");
    let entry = all.get(case).unwrap_or_else(|| {
        let actual = serde_json::json!({ case: { "pages": actual_pages, "headings": actual_headings } });
        panic!("{case}: no golden entry in golden/print_golden.json; this render reads {actual}")
    });
    let pages = entry["pages"].as_u64().unwrap_or_else(|| panic!("{case}: golden pages is a number")) as usize;
    let headings = entry["headings"]
        .as_array()
        .unwrap_or_else(|| panic!("{case}: golden headings is a list"))
        .iter()
        .map(|heading| heading.as_str().expect("a golden heading is a string").to_owned())
        .collect();
    (pages, headings)
}

/// `text` with no whitespace at all: a note the frame breaks over lines, or whose paragraphs the
/// PDF's text runs together, still compares with the words the layout file gave it.
fn unspaced(text: &str) -> String {
    text.chars().filter(|c| !c.is_whitespace()).collect()
}

/// The PDF's text without its page footers (`<title> · page <n> of <pages>`), which the frame
/// prints on every page and so land inside any block that crosses a page.
fn without_footers(text: &str, title: &str, pages: usize) -> String {
    let mut text = text.to_owned();
    for page in 1..=pages {
        text = text.replace(&format!("{} · page {page} of {pages}", folded(title)), " ");
    }
    text
}

/// Every label, value and note of each block must be in the PDF's text between the block's heading
/// and the next block's heading; the headings must come in the layout file's order. A heading is
/// found together with its block's first printed words, so a heading's word inside an earlier
/// block's text ("Abjuration Wizard Spells" before the Spells block) is not taken for it.
fn assert_blocks_print_in_order(document: &SheetDocument, text: &str, case: &str) {
    let text = unspaced(text);
    let pieces = |block: &crate::print::model::SheetBlock| -> Vec<String> {
        block
            .rows
            .iter()
            .flat_map(|row| std::iter::once(&row.label).chain(row.values.iter()).chain(row.notes.iter()))
            .map(|piece| unspaced(piece))
            .filter(|piece| !piece.is_empty())
            .collect()
    };
    let mut starts = Vec::with_capacity(document.blocks.len());
    let mut from = 0;
    for block in &document.blocks {
        let anchor = format!("{}{}", unspaced(&block.heading), pieces(block).first().cloned().unwrap_or_default());
        let at = text[from..]
            .find(&anchor)
            .unwrap_or_else(|| panic!("{case}: heading {:?} and its first words print after byte {from} of the PDF's text", block.heading));
        starts.push(from + at);
        from += at + unspaced(&block.heading).len();
    }
    for (index, block) in document.blocks.iter().enumerate() {
        let end = starts.get(index + 1).copied().unwrap_or(text.len());
        let segment = &text[starts[index]..end];
        for piece in pieces(block) {
            assert!(segment.contains(&piece), "{case}: block {:?} prints {piece:?}; its text: {segment}", block.heading);
        }
    }
}

fn golden_case(system: System, paper: Paper, options: Options) {
    let case = case_name(system, paper, options);
    let root = saved_character(system, &case);
    let request = request(paper, options);

    let document = sheet_document_at_root(&root, &request).unwrap_or_else(|err| panic!("{case}: lays out: {err}"));
    let pdf = render(&root, &request, &case);
    if let Some(dir) = std::env::var_os("PRINT_GOLDEN_OUT") {
        std::fs::create_dir_all(&dir).expect("makes the golden-out directory");
        std::fs::write(PathBuf::from(dir).join(format!("{case}.pdf")), &pdf).expect("writes the PDF");
    }
    assert!(pdf.starts_with(b"%PDF-") && pdf.len() > 1024, "{case}: a PDF over 1 KiB");

    let again = render(&root, &request, &case);
    assert!(again == pdf, "{case}: two renders are byte-equal ({} and {} bytes)", pdf.len(), again.len());

    let expected_size = match paper {
        Paper::A4 => (595.28_f32, 841.89_f32),
        Paper::Letter => (612.0, 792.0),
    };
    let sizes = page_sizes(&pdf);
    for (width, height) in &sizes {
        assert!(
            (width - expected_size.0).abs() < 0.5 && (height - expected_size.1).abs() < 0.5,
            "{case}: page {width}x{height}, want {expected_size:?}"
        );
    }

    let headings: Vec<String> = document.blocks.iter().map(|block| block.heading.clone()).collect();
    let (golden_pages, golden_headings) = golden(&case, sizes.len(), &headings);
    assert_eq!(headings, golden_headings, "{case}: the headings, in print order");
    assert_eq!(sizes.len(), golden_pages, "{case}: the page count");

    let text = without_footers(&pdf_text(&pdf), &document.title, sizes.len());
    assert!(text.contains(&folded(&document.title)), "{case}: the title {:?} prints", document.title);
    assert_blocks_print_in_order(&document, &text, &case);
    if request.bio {
        assert!(text.contains("The Golden Road") && text.contains("Three sheets, two papers, four option sets."), "{case}: the campaign prints");
    }
}

macro_rules! golden {
    ($($name:ident: $system:ident $paper:ident $options:ident;)*) => {
        $(
            #[test]
            fn $name() {
                golden_case(System::$system, Paper::$paper, Options::$options);
            }
        )*
    };
}

golden! {
    pf1e_a4_none: Pf1e A4 None;
    pf1e_a4_spellbook: Pf1e A4 Spellbook;
    pf1e_a4_math: Pf1e A4 Math;
    pf1e_a4_all: Pf1e A4 All;
    pf1e_letter_none: Pf1e Letter None;
    pf1e_letter_spellbook: Pf1e Letter Spellbook;
    pf1e_letter_math: Pf1e Letter Math;
    pf1e_letter_all: Pf1e Letter All;
    sf1e_a4_none: Sf1e A4 None;
    sf1e_a4_spellbook: Sf1e A4 Spellbook;
    sf1e_a4_math: Sf1e A4 Math;
    sf1e_a4_all: Sf1e A4 All;
    sf1e_letter_none: Sf1e Letter None;
    sf1e_letter_spellbook: Sf1e Letter Spellbook;
    sf1e_letter_math: Sf1e Letter Math;
    sf1e_letter_all: Sf1e Letter All;
    cpr_a4_none: Cpr A4 None;
    cpr_a4_spellbook: Cpr A4 Spellbook;
    cpr_a4_math: Cpr A4 Math;
    cpr_a4_all: Cpr A4 All;
    cpr_letter_none: Cpr Letter None;
    cpr_letter_spellbook: Cpr Letter Spellbook;
    cpr_letter_math: Cpr Letter Math;
    cpr_letter_all: Cpr Letter All;
}
