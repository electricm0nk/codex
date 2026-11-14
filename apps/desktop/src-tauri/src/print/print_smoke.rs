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
    // Bio ticked: the details (Race, Class and level) print on its page (E4.1: Bio is its own box).
    let rendered = render_sheet_pdf_at_root(root, &PrintRequest { paper, bio: true, ..PrintRequest::default() }).expect("Aldric's sheet renders");
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
        // E3.4: the Bio page's details form labels the class "Class and level".
        "Class and level Fighter 3".to_owned(),
        // E2.18: no "Armor Class" heading over the boxes; the AC box's own label names it.
        format!("Armor Class {}", snapshot.baseline_armor_class),
        format!("Touch {}", explained("defense.touch_armor_class")),
        format!("Flat-Footed {}", explained("defense.flat_footed_armor_class")),
        format!("Attack Base Attack Bonus {}", signed(i64::from(snapshot.base_attack_bonus))),
        format!("Melee {}", signed(i64::from(snapshot.baseline_melee_attack_bonus))),
        format!("CMB {}", signed(i64::from(explained("combat.combat_maneuver_bonus")))),
        format!("CMD {}", explained("defense.combat_maneuver_defense")),
        format!("Initiative {}", signed(i64::from(explained("initiative.total")))),
        format!("Saving Throws Fortitude {}", signed(i64::from(snapshot.total_saves.fortitude))),
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
    // The blocks, in the PF layout file's order: the Combat block split along the screen's boxes
    // (SD-40 `decisions.md §8`), page 1 laid out on the grid (E2.3): each row's cells left to right.
    let at = |heading: &str| text.find(heading).unwrap_or_else(|| panic!("heading {heading} printed: {text}"));
    // E2.18: the portrait's two-row cell reads after the name row in the PDF's text.
    assert!(at("Character Name") < at("Portrait") && at("Portrait") < at("Hit Points") && at("Hit Points") < at("Abilities"), "Character Name, Portrait, Hit Points, Abilities in order: {text}");
    assert!(at("Speed") < at("Details"), "the Bio section's details print after page 1's Speed (E3.4): {text}");
    // E2.18 (`decisions.md §24.3.1`): the AC row beside the portrait; the left column (Weapons,
    // Attack, Saving Throws), then the right column's Abilities.
    assert!(at("Armor Class") < at("Speed") && at("Speed") < at("Weapons") && at("Weapons") < at("Attack") && at("Attack") < at("Saving Throws") && at("Saving Throws") < at("Abilities"), "Armor Class, Speed, Weapons, Attack, Saving Throws, Abilities in order: {text}");
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

/// The load response's `hit_points.max` and `speed.land` rows (E8.6): the one value the sheet's
/// Hit Points and Speed tiles read, and the one the Hit Points and Speed blocks print.
fn hit_points_and_speed(root: &Path) -> (i16, i16) {
    let loaded = load_saved_character_at_root(root).expect("the character loads");
    let row = |id: &str| {
        loaded.explanations.iter().find(|row| row.id == id).unwrap_or_else(|| panic!("the load response has the {id} row")).value
    };
    (row("hit_points.max"), row("speed.land"))
}

#[test]
fn print_smoke_hit_points_and_speed_print_the_load_responses_one_value_for_aldric() {
    let root = seeded_aldric("hp-speed");
    let (hit_points, speed) = hit_points_and_speed(&root);
    // Aldric is a Human (30 ft., CRB p.27) Fighter 3 (d10) with Constitution 14 (+2) and no saved
    // rolls: 10 + 2, then the d10's average 6 + 2 twice (CRB p.30).
    assert_eq!((hit_points, speed), (28, 30), "Aldric's maximum hit points and land speed");
    let text = pdf_text(&render(&root, Paper::A4));
    for line in [format!("Max {hit_points}"), format!("Speed Land {speed} ft.")] {
        assert!(text.contains(&line), "the PDF prints `{line}` under its own heading: {text}");
    }
}

#[test]
fn print_smoke_hit_points_and_speed_carry_a_gm_grant_and_a_worn_speed_item() {
    use crate::character_custom::{save_character_custom_at_root, CharacterCustomDto, CustomBonusDto, CustomGrantDto, CustomRecordDto};
    use codex::rules_core::body_slots::BodySlot;
    use codex::rules_core::character_input::ActiveState;

    let root = seeded_aldric("hp-speed-custom");
    let (base_hit_points, base_speed) = hit_points_and_speed(&root);
    let bonus = |target: &str, value: i32| CustomBonusDto { target: target.to_owned(), bonus_type: "Enhancement".to_owned(), value };
    let custom = CharacterCustomDto {
        grants: vec![CustomGrantDto {
            id: "g1".to_owned(),
            label: "Blessing of the Hearth".to_owned(),
            target: "hit_points".to_owned(),
            value: 5,
            reason: "the GM's quest reward".to_owned(),
        }],
        equipment: vec![CustomRecordDto {
            id: "boots".to_owned(),
            name: "Boots of the Long Road".to_owned(),
            slot: Some(BodySlot::Feet),
            bonuses: vec![bonus("speed", 10), bonus("hit_points", 3)],
            ..Default::default()
        }],
        ..Default::default()
    };
    save_character_custom_at_root(&root, &custom, "2026-11-05T00:00:00Z").expect("custom.json saves");
    // Carried: the grant counts, the boots do not.
    assert_eq!(hit_points_and_speed(&root), (base_hit_points + 5, base_speed), "a grant moves hit points; carried boots move nothing");
    let worn = crate::character_hub::set_equipment_active_state_at_root(&root, "custom:boots", ActiveState::EquippedActive, "2026-11-05T00:00:01Z")
        .expect("the boots go on");
    assert!(matches!(worn, crate::character_hub::CreateCharacterResponse::Saved { .. }), "{worn:?}");
    let (hit_points, speed) = hit_points_and_speed(&root);
    assert_eq!((hit_points, speed), (base_hit_points + 5 + 3, base_speed + 10), "worn boots add their hit points and speed");
    let text = pdf_text(&render(&root, Paper::A4));
    for line in [format!("Max {hit_points}"), format!("Speed Land {speed} ft.")] {
        assert!(text.contains(&line), "the PDF prints `{line}`: {text}");
    }
}

/// Every Pathfinder starter seed the app writes (`seed_default_characters_at`; counted, not listed)
/// prints through the page grid on both papers with each option set, and Typst reports no
/// diagnostic for any of them (SD-40 E1.MC population probe; NR-4).
#[test]
fn print_smoke_every_pf_seed_renders_through_the_grid_with_no_diagnostics() {
    let dir = std::env::temp_dir().join(format!("codex-print-smoke-{}-every-seed", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp app-data dir is creatable");
    seed_default_characters_at(&dir, "0.0.0-test").expect("starter seeding succeeds");
    let mut roots: Vec<PathBuf> = std::fs::read_dir(dir.join("characters"))
        .expect("the seeds' characters directory")
        .map(|entry| entry.expect("a directory entry").path())
        .filter(|root| {
            crate::rule_system_adapter::load_saved_character_via_envelope(root).is_ok_and(|loaded| loaded.summary.game_system == "pf1")
        })
        .collect();
    roots.sort();
    assert_eq!(roots.len(), 3, "the Pathfinder starter seeds: Aldric, Elowen, Tomas: {roots:?}");
    let mut rendered = 0;
    for root in &roots {
        for paper in [Paper::A4, Paper::Letter] {
            for (spellbook, math, bio) in [(false, false, false), (true, false, false), (false, true, false), (true, true, true)] {
                let request = PrintRequest { paper, spellbook, math, bio, ..PrintRequest::default() };
                let case = format!("{} {paper:?} spellbook={spellbook} math={math} bio={bio}", root.display());
                let document = super::sheet_document_at_root(root, &request).unwrap_or_else(|err| panic!("{case}: builds: {err}"));
                assert_eq!(document.columns(), crate::print::model::GRID_COLUMNS, "{case}: prints on the page grid");
                let sheet = super::typst::typeset(&document).unwrap_or_else(|err| panic!("{case}: typesets: {err}"));
                assert!(sheet.warnings.is_empty(), "{case}: Typst reports no diagnostics: {:?}", sheet.warnings);
                assert!(sheet.pdf.starts_with(b"%PDF-"), "{case}: the bytes are a PDF");
                rendered += 1;
            }
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(rendered, 24, "3 seeds x 2 papers x 4 option sets");
}
