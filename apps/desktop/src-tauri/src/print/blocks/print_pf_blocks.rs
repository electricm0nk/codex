//! `print_pf_blocks` (E8.2, `decisions.md §4`, operator rulings R4, R17, R21): the Pathfinder sheet
//! prints all 15 blocks through the real print path, and the three option boxes (Spellbook, Show
//! the math, Bio and campaign notes) each add what they name.
//!
//! Run (from `apps/desktop/src-tauri`):
//!
//! ```text
//! cargo test --locked -j 8 print_pf_blocks -- --test-threads=8
//! ```

use crate::character_custom::{load_character_custom_at_root, save_character_custom_at_root};
use crate::character_hub::{
    add_feat_selection_enforcing_prerequisites_at_root, list_held_equipment_at_root, load_saved_character_at_root,
    seed_default_characters_at, set_equipment_active_state_at_root, ActiveStateDto,
};
use crate::print::{render_sheet_pdf_at_root, sheet_document_at_root, CampaignNote, Paper, PrintRequest};
use codex::rules_core::character_input::ActiveState;
use std::path::{Path, PathBuf};

const ALDRIC_ID: &str = "00000000-0000-0000-0000-000000000001";
const ELOWEN_ID: &str = "00000000-0000-0000-0000-000000000002";
const TOMAS_ID: &str = "00000000-0000-0000-0000-000000000003";
const AT: &str = "2026-11-05T00:00:00Z";

/// The 13 blocks every Pathfinder sheet prints, in the layout file's order.
const BASE_HEADINGS: [&str; 13] = [
    "Identity",
    "Combat",
    "Abilities",
    "Skills",
    "Weapons",
    "Armor",
    "Inventory",
    "Feats",
    "Class Features",
    "Spells",
    "Custom",
    "Pets",
    "Actions",
];
const SPELLBOOK: &str = "Spellbook";
const MATH: &str = "Show the Math";
const BIO: &str = "Bio and Campaign Notes";
const NO_DERIVATION: &str = "(no derivation recorded)";

/// A fresh app-data directory holding the starter seeds; returns the named seed's character root.
fn seeded(tag: &str, character_id: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("codex-print-pf-blocks-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp app-data dir is creatable");
    seed_default_characters_at(&dir, "0.0.0-test").expect("starter seeding succeeds");
    let root = dir.join("characters").join(character_id);
    assert!(root.is_dir(), "the seed's saved root exists at {}", root.display());
    root
}

fn request(spellbook: bool, math: bool, bio: bool) -> PrintRequest {
    PrintRequest { paper: Paper::A4, spellbook, math, bio, campaigns: Vec::new() }
}

fn pdf_text(bytes: &[u8]) -> String {
    let raw = pdf_extract::extract_text_from_mem(bytes).expect("the PDF's text extracts");
    raw.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn page_count(bytes: &[u8]) -> usize {
    lopdf::Document::load_mem(bytes).expect("the PDF parses").get_pages().len()
}

/// Renders and insists Typst raised no diagnostic: a sheet that overflows or misnames a field says so.
fn render(root: &Path, request: &PrintRequest) -> Vec<u8> {
    let rendered = render_sheet_pdf_at_root(root, request).expect("the sheet renders");
    assert!(rendered.warnings.is_empty(), "Typst reports no diagnostics: {:?}", rendered.warnings);
    rendered.pdf
}

/// Each heading is found after the one before it in the extracted text.
fn assert_in_order(text: &str, headings: &[&str]) {
    let mut from = 0;
    for heading in headings {
        let at = text[from..].find(heading).unwrap_or_else(|| panic!("`{heading}` printed after byte {from}: {text}"));
        from += at + heading.len();
    }
}

#[test]
fn print_pf_blocks_every_block_heading_prints_in_layout_order() {
    for (tag, id) in [("order-aldric", ALDRIC_ID), ("order-elowen", ELOWEN_ID)] {
        let root = seeded(tag, id);
        let all = request(true, true, true);
        let document = sheet_document_at_root(&root, &all).expect("the document builds");
        let headings: Vec<&str> = document.blocks.iter().map(|block| block.heading.as_str()).collect();
        let mut expected: Vec<&str> = BASE_HEADINGS.to_vec();
        expected.extend([SPELLBOOK, MATH, BIO]);
        assert_eq!(headings, expected, "{tag}: the 15 block types (Spells twice: its list, then the Spellbook), in the layout file's order");
        let full = render(&root, &all);
        assert_in_order(&pdf_text(&full), &expected);
        // For a reader's own look (`pdftotext`, a viewer): the full sheet, when asked for.
        if let Ok(out) = std::env::var("PRINT_PF_BLOCKS_OUT") {
            std::fs::create_dir_all(&out).expect("the output dir is creatable");
            std::fs::write(Path::new(&out).join(format!("{tag}-full.pdf")), &full).expect("the PDF writes");
        }

        let none = sheet_document_at_root(&root, &request(false, false, false)).expect("the document builds");
        let headings: Vec<&str> = none.blocks.iter().map(|block| block.heading.as_str()).collect();
        assert_eq!(headings, BASE_HEADINGS.to_vec(), "{tag}: basic prints no option block");
        let text = pdf_text(&render(&root, &request(false, false, false)));
        assert_in_order(&text, &BASE_HEADINGS);
        for option in [SPELLBOOK, MATH, BIO, NO_DERIVATION] {
            assert!(!text.contains(option), "{tag}: basic prints no `{option}`: {text}");
        }
    }
}

#[test]
fn print_pf_blocks_each_option_adds_pages() {
    for (tag, id) in [("pages-aldric", ALDRIC_ID), ("pages-elowen", ELOWEN_ID)] {
        let root = seeded(tag, id);
        let basic = page_count(&render(&root, &request(false, false, false)));
        let spellbook = page_count(&render(&root, &request(true, false, false)));
        let math = page_count(&render(&root, &request(false, true, false)));
        let bio = page_count(&render(&root, &request(false, false, true)));
        let full = page_count(&render(&root, &request(true, true, true)));
        eprintln!("{tag}: pages basic {basic}, spellbook {spellbook}, math {math}, bio {bio}, full {full}");
        assert!(spellbook > basic, "{tag}: Spellbook on adds a page ({basic} -> {spellbook})");
        assert!(math > basic, "{tag}: Show the math on adds a page ({basic} -> {math})");
        assert!(bio > basic, "{tag}: Bio and campaign notes on adds a page ({basic} -> {bio})");
        assert!(full > spellbook && full > math && full > bio, "{tag}: full is longer than any one option ({full})");
    }
}

#[test]
fn print_pf_blocks_the_blocks_print_the_values_the_response_holds() {
    let root = seeded("values-elowen", ELOWEN_ID);
    let loaded = load_saved_character_at_root(&root).expect("Elowen loads");
    let explained = |id: &str| loaded.explanations.iter().find(|row| row.id == id).unwrap_or_else(|| panic!("{id}")).value;
    let signed = |n: i16| if n >= 0 { format!("+{n}") } else { n.to_string() };
    let text = pdf_text(&render(&root, &request(false, false, false)));
    let expected = [
        // Skills: ranks, and the load response's total.
        format!("Spellcraft 5 {}", signed(explained("skill.total.spellcraft"))),
        format!("Climb 1 {}", signed(explained("skill.total.climb"))),
        // Weapons: the dice stay literal; each facet its own cell.
        "Longsword 1d8".to_owned(),
        "19-20/x2".to_owned(),
        format!("Attack bonus (longsword) {}", signed(explained("combat.weapon_attack_bonus.longsword"))),
        // Armor: the worn armor's own numbers.
        "Chain Shirt +4".to_owned(),
        // Inventory: the encumbrance the engine computed.
        "Load Medium".to_owned(),
        // Feats: the engine's budget rows.
        format!("Feat slots: level granted {}", explained("feats.budget.level.granted")),
        // Spells: the save DC and spells per day rows.
        format!("Wizard: spell save DC, level 3 {}", explained("class_chassis.wizard.spell_save_dc.spell_level_3")),
        format!("Wizard: spells per day, level 3 {}", explained("class_spell.wizard.total_spells_per_day.spell_level_3")),
        "Fireball".to_owned(),
        // Actions: the race's own traits.
        "Skilled".to_owned(),
    ];
    for line in expected {
        assert!(text.contains(&line), "the PDF prints `{line}`: {text}");
    }
}

/// E8.7: every skill a seed holds ranks in prints its ranks and the load response's own total
/// (`skill.total.<skill>`), the value the Skills panel reads. A few totals are also held to the book
/// (CRB Chapter 4: ranks + the key ability's modifier + 3 for a class skill with a rank).
#[test]
fn print_pf_blocks_every_ranked_skill_prints_the_load_responses_total() {
    let signed = |n: i16| if n >= 0 { format!("+{n}") } else { n.to_string() };
    let book = [
        // Aldric, Human Fighter 3: Str +4, Dex +1, Int +2, Wis +1; Ride, Survival and Knowledge
        // (Dungeoneering) are Fighter class skills.
        (ALDRIC_ID, "ride", 3 + 1 + 3),
        (ALDRIC_ID, "survival", 3 + 1 + 3),
        (ALDRIC_ID, "knowledge_dungeoneering", 3 + 2 + 3),
        // Elowen, Human Wizard 5: Int +4, Dex +2; Spellcraft, Fly and Knowledge (Arcana) are Wizard
        // class skills.
        (ELOWEN_ID, "spellcraft", 5 + 4 + 3),
        (ELOWEN_ID, "fly", 5 + 2 + 3),
        (ELOWEN_ID, "knowledge_arcana", 5 + 4 + 3),
    ];
    for (tag, id) in [("skills-aldric", ALDRIC_ID), ("skills-elowen", ELOWEN_ID)] {
        let root = seeded(tag, id);
        let loaded = load_saved_character_at_root(&root).expect("the seed loads");
        let text = pdf_text(&render(&root, &request(false, false, false)));
        let skills = &text[text.find("Skills").expect("Skills")..text.find("Weapons").expect("Weapons")];
        let ranked: Vec<_> = loaded.skill_allocations.iter().filter(|allocation| allocation.ranks > 0).collect();
        assert!(ranked.len() >= 7, "{tag}: the seed holds ranks in at least 7 skills: {ranked:?}");
        for allocation in ranked {
            let slug = allocation.skill_id.strip_prefix("skill:").expect("a skill id");
            let total = loaded
                .explanations
                .iter()
                .find(|row| row.id == format!("skill.total.{slug}"))
                .unwrap_or_else(|| panic!("{tag}: the load response has the skill.total.{slug} row"))
                .value;
            let line = format!("{} {} {}", super::skills::skill_name(slug), allocation.ranks, signed(total));
            assert!(skills.contains(&line), "{tag}: the Skills block prints `{line}`: {skills}");
        }
        for (_, slug, value) in book.iter().filter(|(seed, _, _)| *seed == id) {
            let row = loaded.explanations.iter().find(|row| row.id == format!("skill.total.{slug}")).expect("the row");
            assert_eq!(row.value, *value, "{tag}: {slug} is the book's total: {}", row.detail);
        }
    }
}

/// The Skills panel lists every skill with its total, ranked or not (`skill.total.<skill>`, one row
/// per panel row since E8.7), so the basic sheet does too: an unranked Perception is still a number
/// the player reads off the sheet.
#[test]
fn print_pf_blocks_every_skills_panel_total_prints_ranked_or_not() {
    let signed = |n: i16| if n >= 0 { format!("+{n}") } else { n.to_string() };
    for (tag, id) in [("all-skills-aldric", ALDRIC_ID), ("all-skills-elowen", ELOWEN_ID), ("all-skills-tomas", TOMAS_ID)] {
        let root = seeded(tag, id);
        let loaded = load_saved_character_at_root(&root).expect("the seed loads");
        let text = pdf_text(&render(&root, &request(false, false, false)));
        let skills = &text[text.find("Skills").expect("Skills")..text.find("Weapons").expect("Weapons")];
        let totals: Vec<_> = loaded.explanations.iter().filter(|row| row.id.starts_with("skill.total.")).collect();
        assert_eq!(totals.len(), 35, "{tag}: one total per Skills panel row");
        for row in totals {
            let slug = row.id.strip_prefix("skill.total.").expect("a skill total");
            let ranks: u32 = loaded
                .skill_allocations
                .iter()
                .filter(|allocation| allocation.skill_id.strip_prefix("skill:") == Some(slug))
                .map(|allocation| u32::from(allocation.ranks))
                .sum();
            let line = format!("{} {} {}", super::skills::skill_name(slug), ranks, signed(row.value));
            assert!(skills.contains(&line), "{tag}: the Skills block prints `{line}`: {skills}");
        }
    }
}

/// The Gear tab's money tiles (PP, GP, SP, CP, `load_character_money`) and the Details panel's Size
/// and Vision (the race roster's, `deriveRaceTraits`) print on the basic sheet as the screen shows them.
#[test]
fn print_pf_blocks_money_size_and_vision_print_as_the_sheet_shows_them() {
    let root = seeded("money-size", ALDRIC_ID);
    crate::character_hub::adjust_character_money_at_root(&root, 1_234_567).expect("money adjusts");
    let money = crate::character_hub::load_character_money_at_root(&root).expect("money loads");
    assert!(money.platinum > 0 && money.gold > 0 && money.silver > 0 && money.copper > 0, "{money:?}");
    let loaded = load_saved_character_at_root(&root).expect("the seed loads");
    let roster = crate::character_hub::list_race_creation_roster();
    let race = roster.races.iter().find(|race| race.race_id == loaded.summary.race_id).expect("the seed's race is on the roster");
    let text = pdf_text(&render(&root, &request(false, false, false)));
    let identity = &text[text.find("Identity").expect("Identity")..text.find("Combat").expect("Combat")];
    assert!(identity.contains(&format!("Size {}", race.size)), "Size {}: {identity}", race.size);
    assert!(identity.contains(&format!("Vision {}", race.vision)), "Vision {}: {identity}", race.vision);
    let inventory = &text[text.find("Inventory").expect("Inventory")..text.find("Feats").expect("Feats")];
    for (label, value) in [("PP", money.platinum), ("GP", money.gold), ("SP", money.silver), ("CP", money.copper)] {
        assert!(inventory.contains(&format!("{label} {value}")), "{label} {value}: {inventory}");
    }
}

#[test]
fn print_pf_blocks_show_the_math_prints_every_derivation_and_names_the_rest() {
    for (tag, id) in [("math-aldric", ALDRIC_ID), ("math-elowen", ELOWEN_ID)] {
        let root = seeded(tag, id);
        let loaded = load_saved_character_at_root(&root).expect("the seed loads");
        let document = sheet_document_at_root(&root, &request(false, true, false)).expect("the document builds");
        let math = document.blocks.iter().find(|block| block.heading == MATH).expect("the math block");
        let printed = document.blocks.iter().filter(|block| block.heading != MATH).flat_map(|block| &block.rows);
        let derived: Vec<_> = printed.filter(|row| row.derivation.is_some()).collect();
        assert_eq!(math.rows.len(), derived.len(), "{tag}: one math entry per printed value that has one");
        let missing = derived.iter().filter(|row| row.derivation.as_ref().is_some_and(Vec::is_empty)).count();
        eprintln!("{tag}: {} printed values, {missing} `{NO_DERIVATION}`", derived.len());
        let text = pdf_text(&render(&root, &request(false, true, false)));
        let occurrences = text.matches(NO_DERIVATION).count();
        assert_eq!(occurrences, missing, "{tag}: the PDF prints the marker once per value with no row");
        // Every explanations row the printed values name prints its detail, verbatim.
        for id in ["defense.baseline_armor_class", "initiative.total", "defense.total_save.will", "ability_modifier.strength"] {
            let row = loaded.explanations.iter().find(|row| row.id == id).unwrap_or_else(|| panic!("{tag}: {id}"));
            let detail: String = row.detail.split_whitespace().take(12).collect::<Vec<_>>().join(" ");
            assert!(text.contains(id), "{tag}: the math names `{id}`: {text}");
            assert!(text.contains(&detail), "{tag}: the math prints `{id}`'s detail `{detail}`");
        }
    }
}

#[test]
fn print_pf_blocks_spellbook_prints_each_known_spells_full_text() {
    let root = seeded("spellbook-elowen", ELOWEN_ID);
    let loaded = load_saved_character_at_root(&root).expect("Elowen loads");
    let text = pdf_text(&render(&root, &request(true, false, false)));
    let spellbook = &text[text.find(SPELLBOOK).expect("the Spellbook prints")..];
    for name in ["Fireball", "Light"] {
        let line = loaded.sheet_lines.iter().find(|line| line.kind == "spell" && line.label == name).expect("the spell's line");
        // Compared with every space removed: the text's own line breaks print as separate lines,
        // which the extraction joins with no space ("level.Casting time").
        let bare = |text: &str| text.split_whitespace().collect::<String>();
        let words = bare(&line.prose);
        assert!(words.len() > 200, "{name}: the rules hold its full text ({} chars)", words.len());
        assert!(bare(spellbook).contains(&words), "the Spellbook prints {name}'s whole text: {spellbook}");
    }
    // A character with no spell says so rather than printing an empty page.
    let aldric = seeded("spellbook-aldric", ALDRIC_ID);
    let text = pdf_text(&render(&aldric, &request(true, false, false)));
    assert!(text.contains("No spell is known."), "{text}");
}

#[test]
fn print_pf_blocks_bio_prints_the_bio_and_the_campaigns_description() {
    let root = seeded("bio-aldric", ALDRIC_ID);
    std::fs::write(
        root.join("bio.json"),
        r#"{"playerName":"Todd","alignment":"Lawful Good","deity":"Iomedae","eyes":"Grey"}"#,
    )
    .expect("the bio writes");
    let mut with_campaign = request(false, false, true);
    with_campaign.campaigns =
        vec![CampaignNote { name: "Rise of the Runelords".to_owned(), description: "Sandpoint, autumn 4707.".to_owned() }];
    let text = pdf_text(&render(&root, &with_campaign));
    for line in ["Player Todd", "Alignment Lawful Good", "Deity Iomedae", "Eyes Grey", "Rise of the Runelords", "Sandpoint, autumn 4707."] {
        assert!(text.contains(line), "the bio page prints `{line}`: {text}");
    }
    let text = pdf_text(&render(&root, &request(false, false, true)));
    assert!(text.contains("No campaign lists this character."), "{text}");
}

#[test]
fn print_pf_blocks_an_override_tag_prints_on_what_it_let_in() {
    let root = seeded("override-aldric", ALDRIC_ID);
    let mut custom = load_character_custom_at_root(&root).expect("custom loads");
    custom.overrides.feats_without_prerequisites = true;
    custom.overrides.ignore_feat_budget = true;
    save_character_custom_at_root(&root, &custom, AT).expect("the flags save");
    add_feat_selection_enforcing_prerequisites_at_root(&root, "Improved Two-Weapon Fighting", None, AT)
        .expect("the override admits the feat");
    let loaded = load_saved_character_at_root(&root).expect("Aldric loads");
    assert!(!loaded.admitted_under_override.is_empty(), "the feat came in under an override");
    let text = pdf_text(&render(&root, &request(false, false, false)));
    assert!(text.contains("Override: Feats without prerequisites"), "the tag prints on the feat: {text}");
}

#[test]
fn print_pf_blocks_a_left_behind_item_is_owned_with_no_weight() {
    let root = seeded("left-behind-aldric", ALDRIC_ID);
    set_equipment_active_state_at_root(&root, "item:chain_shirt", ActiveState::Absent, AT).expect("left behind");
    let held = list_held_equipment_at_root(&root).expect("held");
    assert!(held.iter().any(|copy| copy.item_id == "item:chain_shirt" && copy.active_state == ActiveStateDto::Absent));
    let text = pdf_text(&render(&root, &request(false, false, false)));
    let inventory = &text[text.find("Inventory").expect("Inventory")..text.find("Feats").expect("Feats")];
    assert!(inventory.contains("Chain Shirt Left behind"), "listed as owned: {inventory}");
    assert!(inventory.contains("Left behind no weight"), "with no weight: {inventory}");
    // The loadout's `item:shield` row and the Power Attack toggle are not things the character owns.
    assert!(!inventory.contains("power_attack") && !inventory.contains("shield"), "{inventory}");
}
