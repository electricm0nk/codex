//! `print_smoke` (E8.1, `decisions.md §4`): the Aldric seed goes through the real print path --
//! `load_saved_character_at_root`, the PF layout file, the Typst world with only the bundled
//! fonts, `typst-pdf` -- and comes back as a real PDF whose text names him and his Armor Class.
//!
//! Run (from `apps/desktop/src-tauri`):
//!
//! ```text
//! cargo test --locked -j 8 print_smoke -- --test-threads=8
//! ```

use super::{render_sheet_pdf_at_root, sheet_font_families, write_print_copy_at_root, write_sheet_pdf_at_root, Paper, PrintRequest};
use crate::character_hub::{load_saved_character_at_root, seed_default_characters_at};
use std::path::{Path, PathBuf};

/// Aldric Ironhand, the first Pathfinder starter seed (`character_hub.rs` `DEFAULT_CHARACTER_ID`).
const ALDRIC_ID: &str = "00000000-0000-0000-0000-000000000001";

/// A fresh app-data directory holding the starter seeds; returns Aldric's character root.
fn seeded_aldric(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("codex-print-smoke-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp app-data dir is creatable");
    seed_default_characters_at(&dir, "0.0.0-test").expect("starter seeding succeeds");
    let root = dir.join("characters").join(ALDRIC_ID);
    assert!(root.is_dir(), "Aldric's saved root exists at {}", root.display());
    root
}

/// The PDF's text, by the same extraction a reader's copy-paste would see, with every run of
/// whitespace folded to one space so a line break between a label and its value does not matter.
fn pdf_text(bytes: &[u8]) -> String {
    let raw = pdf_extract::extract_text_from_mem(bytes).expect("the PDF's text extracts");
    raw.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Every page's `/MediaBox` width and height, in PDF points.
fn page_sizes(bytes: &[u8]) -> Vec<(f32, f32)> {
    let document = lopdf::Document::load_mem(bytes).expect("the PDF parses");
    document
        .get_pages()
        .values()
        .map(|page_id| {
            let page = document.get_dictionary(*page_id).expect("a page is a dictionary");
            let media_box = page.get(b"MediaBox").expect("a page has a MediaBox");
            let numbers: Vec<f32> = media_box
                .as_array()
                .expect("MediaBox is an array")
                .iter()
                .map(|n| n.as_float().or_else(|_| n.as_i64().map(|i| i as f32)).expect("a MediaBox entry is a number"))
                .collect();
            (numbers[2] - numbers[0], numbers[3] - numbers[1])
        })
        .collect()
}

fn render(root: &Path, paper: Paper) -> Vec<u8> {
    let rendered = render_sheet_pdf_at_root(root, &PrintRequest { paper, ..PrintRequest::default() }).expect("Aldric's sheet renders");
    assert!(
        rendered.warnings.is_empty(),
        "Typst reports no diagnostics for {paper:?}: {:?}",
        rendered.warnings
    );
    rendered.pdf
}

#[test]
fn print_smoke_aldric_is_a_real_pdf_on_a4_and_letter() {
    let root = seeded_aldric("papers");
    for (paper, expected) in [(Paper::A4, (595.28_f32, 841.89_f32)), (Paper::Letter, (612.0, 792.0))] {
        let pdf = render(&root, paper);
        assert!(pdf.len() > 1024, "{paper:?}: a PDF over 1 KiB, got {} bytes", pdf.len());
        assert!(pdf.starts_with(b"%PDF-"), "{paper:?}: the bytes are a PDF");
        let text = pdf_text(&pdf);
        assert!(text.contains("Aldric"), "{paper:?}: the text names Aldric: {text}");
        assert!(text.contains("Armor Class"), "{paper:?}: the text has Armor Class: {text}");
        let sizes = page_sizes(&pdf);
        assert!(!sizes.is_empty(), "{paper:?}: at least one page");
        for (width, height) in sizes {
            assert!(
                (width - expected.0).abs() < 0.5 && (height - expected.1).abs() < 0.5,
                "{paper:?}: page {width} x {height} pt, expected {expected:?}"
            );
        }
    }
}

#[test]
fn print_smoke_two_runs_are_byte_equal() {
    // NR-3: identical input, identical bytes -- bundled fonts, no timestamp, no random id.
    let root = seeded_aldric("determinism");
    for paper in [Paper::A4, Paper::Letter] {
        let first = render(&root, paper);
        let second = render(&root, paper);
        assert!(first == second, "{paper:?}: two renders of one sheet are byte-equal");
    }
    // And a second seeding (another directory, another moment) prints the same sheet.
    let other = seeded_aldric("determinism-other");
    assert!(render(&root, Paper::A4) == render(&other, Paper::A4), "a re-seeded Aldric prints the same bytes");
}

#[test]
fn print_smoke_the_pdf_prints_the_values_the_sheet_holds() {
    // The values come off the same `LoadSavedCharacterResponse` the screen renders; the print path
    // computes none of them (NR-2). Each label is followed by the response's own value.
    let root = seeded_aldric("values");
    let loaded = load_saved_character_at_root(&root).expect("Aldric loads");
    let snapshot = loaded.snapshot.as_ref().expect("Aldric computes");
    let explained = |id: &str| {
        loaded
            .explanations
            .iter()
            .find(|row| row.id == id)
            .unwrap_or_else(|| panic!("Aldric has the {id} row"))
            .value
    };
    let scores = &loaded.ability_scores;
    let signed = |n: i64| if n >= 0 { format!("+{n}") } else { n.to_string() };
    let text = pdf_text(&render(&root, Paper::A4));
    let expected = [
        format!("Name {}", loaded.summary.display_label),
        "Race Human".to_owned(),
        "Class Fighter 3".to_owned(),
        format!("Armor Class {}", snapshot.baseline_armor_class),
        format!("Touch {}", explained("defense.touch_armor_class")),
        format!("Flat-Footed {}", explained("defense.flat_footed_armor_class")),
        format!("Base Attack Bonus {}", signed(i64::from(snapshot.base_attack_bonus))),
        format!("Melee {}", signed(i64::from(snapshot.baseline_melee_attack_bonus))),
        format!("CMB {}", signed(i64::from(explained("combat.combat_maneuver_bonus")))),
        format!("CMD {}", explained("defense.combat_maneuver_defense")),
        format!("Initiative {}", signed(i64::from(explained("initiative.total")))),
        format!("Fortitude {}", signed(i64::from(snapshot.total_saves.fortitude))),
        format!("Reflex {}", signed(i64::from(snapshot.total_saves.reflex))),
        format!("Will {}", signed(i64::from(snapshot.total_saves.will))),
        format!(
            "Strength {} {}",
            scores.strength,
            signed(i64::from(snapshot.ability_modifiers.strength))
        ),
        format!(
            "Charisma {} {}",
            scores.charisma,
            signed(i64::from(snapshot.ability_modifiers.charisma))
        ),
    ];
    for line in expected {
        assert!(text.contains(&line), "the PDF prints `{line}`: {text}");
    }
    // The three blocks, in the PF layout file's order.
    let at = |heading: &str| text.find(heading).unwrap_or_else(|| panic!("heading {heading} printed: {text}"));
    assert!(at("Identity") < at("Combat") && at("Combat") < at("Abilities"), "Identity, Combat, Abilities in order: {text}");
}

#[test]
fn print_smoke_the_typst_world_holds_only_the_bundled_fonts() {
    assert_eq!(sheet_font_families(), vec!["Liberation Sans".to_owned()]);
}

#[test]
fn print_smoke_save_writes_the_rendered_pdf_to_the_chosen_path() {
    let root = seeded_aldric("save");
    let target = std::env::temp_dir().join(format!("codex-print-smoke-{}-aldric.pdf", std::process::id()));
    let _ = std::fs::remove_file(&target);
    let saved = write_sheet_pdf_at_root(&root, &PrintRequest { paper: Paper::Letter, ..PrintRequest::default() }, &target).expect("the PDF saves");
    let on_disk = std::fs::read(&target).expect("the saved file reads back");
    assert_eq!(saved.bytes, on_disk.len(), "the reported size is the file's");
    assert!(saved.warnings.is_empty(), "no Typst warning on Aldric's sheet: {:?}", saved.warnings);
    assert!(on_disk == render(&root, Paper::Letter), "the saved file is the rendered PDF");
    // A path that cannot be written is refused, by name, and writes nothing.
    let refused = write_sheet_pdf_at_root(&root, &PrintRequest { paper: Paper::A4, ..PrintRequest::default() }, &root.join("no-such-dir").join("x.pdf"))
        .expect_err("an unwritable path is refused");
    assert!(refused.contains("no-such-dir"), "the refusal names the path: {refused}");
}


#[test]
fn print_smoke_print_writes_a_temp_copy_named_for_the_character() {
    // Operator ruling R19: Print makes the PDF and opens it in the system viewer, from a temp file.
    let root = seeded_aldric("print");
    let temp = std::env::temp_dir().join(format!("codex-print-smoke-{}-temp", std::process::id()));
    let _ = std::fs::remove_dir_all(&temp);
    let first = write_print_copy_at_root(&root, ALDRIC_ID, &PrintRequest { paper: Paper::A4, ..PrintRequest::default() }, &temp).expect("the print copy writes");
    assert_eq!(PathBuf::from(&first.file_path), temp.join("codex-print").join(format!("{ALDRIC_ID}.pdf")));
    assert!(std::fs::read(&first.file_path).expect("the copy reads back") == render(&root, Paper::A4));
    // A second print replaces the copy rather than piling files up.
    let second = write_print_copy_at_root(&root, ALDRIC_ID, &PrintRequest { paper: Paper::Letter, ..PrintRequest::default() }, &temp).expect("a second print writes");
    assert_eq!(first.file_path, second.file_path);
    assert!(std::fs::read(&second.file_path).expect("the copy reads back") == render(&root, Paper::Letter));
}
