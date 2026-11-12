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

/// The 19 blocks every Pathfinder sheet prints, in the layout file's order: the Combat block split
/// along the screen's boxes (SD-40 `decisions.md §8`), page 1 laid out on the grid (E2.3) under the
/// portrait and the Character Name field, the Hit Points and Initiative one square block (E2.9),
/// the Identity block opening page 2 (E2.5).
const BASE_HEADINGS: [&str; 19] = [
    "Portrait",
    "Character Name",
    "Speed",
    "Armor Class",
    "Hit Points",
    "Abilities",
    "Skills",
    "Weapons",
    "Attack",
    "Saving Throws",
    "Identity",
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
        assert_eq!(headings, expected, "{tag}: the 22 blocks (Spells twice: its list, then the Spellbook), in the layout file's order");
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
    let modifiers = &loaded.snapshot.as_ref().expect("Elowen computes").ability_modifiers;
    let text = pdf_text(&render(&root, &request(false, false, false)));
    let expected = [
        // Skills: the load response's total, the key ability's modifier, the ranks.
        format!("Spellcraft {} Int {} 5", signed(explained("skill.total.spellcraft")), signed(modifiers.intelligence)),
        format!("Climb* {} Str {} 1", signed(explained("skill.total.climb")), signed(modifiers.strength)),
        // Weapons (E2.10): the attack total, the engine's damage total (the dice literal), the critical.
        format!(
            "Longsword {} 1d8{} 19-20/x2",
            signed(explained("combat.weapon_attack_bonus.longsword")),
            signed(explained("combat.weapon_damage_total.longsword"))
        ),
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

/// A Skills line in the PDF's text (SD-40 E2.2): the name (its armor-check mark after it), then the
/// total, the key ability and its modifier, and the ranks.
fn assert_skill_line(tag: &str, skills: &str, name: &str, total: &str, ranks: u32) {
    let at = skills
        .match_indices(name)
        .map(|(at, _)| at)
        .find(|at| *at == 0 || skills[..*at].ends_with(' '))
        .unwrap_or_else(|| panic!("{tag}: the Skills block prints `{name}`: {skills}"));
    let after: Vec<&str> = skills[at + name.len()..].trim_start_matches('*').split_whitespace().take(4).collect();
    let ranks = ranks.to_string();
    assert_eq!((after.first().copied(), after.get(3).copied()), (Some(total), Some(ranks.as_str())), "{tag}: `{name}` prints its total {total} and its {ranks} ranks: {after:?}");
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
            assert_skill_line(tag, skills, &super::skills::skill_name(slug), &signed(total), u32::from(allocation.ranks));
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
            assert_skill_line(tag, skills, &super::skills::skill_name(slug), &signed(row.value), ranks);
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
    let identity = &text[text.find("Identity").expect("Identity")..];
    let identity = &identity[..identity.find("Armor").expect("the Armor block follows the Identity block")];
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

/// E8.8: the Class Features panel lists every `class_feature.*` / `class_chassis.*` row the engine
/// computed (`buildClassFeatureSurface`), each with its class, its label and its value, so the basic
/// sheet prints every one. The label is the panel's own rule: `golden/class_feature_labels.json`
/// holds every seed's panel rows, which `classFeatureLabelParity.test.ts` holds the panel to and this
/// test holds the print and the seeds' own load to.
#[test]
fn print_pf_blocks_every_class_features_panel_row_prints_its_label_and_value() {
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("../golden/class_feature_labels.json")).expect("the label fixture is JSON");
    for (tag, id) in [("aldric", ALDRIC_ID), ("elowen", ELOWEN_ID), ("tomas", TOMAS_ID)] {
        let root = seeded(&format!("class-features-{tag}"), id);
        let loaded = load_saved_character_at_root(&root).expect("the seed loads");
        let seed = &fixture[tag];
        assert_eq!(seed["characterId"].as_str(), Some(id), "{tag}: the fixture names the seed");
        let is_class_record = |row: &&crate::character_hub::ExplanationDto| {
            row.id.starts_with("class_feature.") || row.id.starts_with("class_chassis.")
        };
        let listed: Vec<(String, i64)> = loaded
            .explanations
            .iter()
            .filter(is_class_record)
            .filter(|row| !row.id.ends_with(".unsupported"))
            .map(|row| (row.id.clone(), i64::from(row.value)))
            .collect();
        let expected = seed["features"].as_array().expect("features is a list");
        let fixture_rows: Vec<(String, i64)> = expected
            .iter()
            .map(|row| (row["id"].as_str().expect("id").to_owned(), row["value"].as_i64().expect("value")))
            .collect();
        assert_eq!(listed, fixture_rows, "{tag}: the fixture holds every row the panel lists, in the engine's order");
        let unsupported: Vec<&str> = loaded
            .explanations
            .iter()
            .filter(is_class_record)
            .filter(|row| row.id.ends_with(".unsupported"))
            .map(|row| row.id.as_str())
            .collect();
        let fixture_unsupported: Vec<&str> =
            seed["notComputed"].as_array().expect("notComputed is a list").iter().map(|id| id.as_str().expect("an id")).collect();
        assert_eq!(unsupported, fixture_unsupported, "{tag}: the fixture holds every row the panel files as not computed");

        let document = sheet_document_at_root(&root, &request(false, false, false)).expect("the document builds");
        let block = document.blocks.iter().find(|block| block.heading == "Class Features").expect("Class Features");
        let text = pdf_text(&render(&root, &request(false, false, false)));
        // Up to the Custom heading: a feature's own label may hold the word "Spells".
        let start = text.find("Class Features").expect("Class Features");
        let features = &text[start..start + text[start..].find("Custom").expect("Custom")];
        for row in expected {
            let class = row["classLabel"].as_str().unwrap_or("Chassis");
            let label = format!("{class}: {}", row["label"].as_str().expect("label"));
            let value = row["value"].as_i64().expect("value").to_string();
            let printed = block
                .rows
                .iter()
                .find(|printed| printed.label == label)
                .unwrap_or_else(|| panic!("{tag}: the Class Features block holds `{label}`: {:?}", block.rows));
            assert_eq!(printed.values, vec![value.clone()], "{tag}: `{label}` prints its value");
            let line = format!("{label} {value}");
            assert!(features.contains(&line), "{tag}: the PDF's Class Features prints `{line}`: {features}");
        }
    }
}

/// E8.8: the Defense tab shows `Damage Reduction: <n>/—` whenever the snapshot holds one, so the
/// basic sheet prints it then, and not otherwise. No seed holds one (the engine grounds it for a
/// Barbarian), so the snapshot is given one here.
#[test]
fn print_pf_blocks_damage_reduction_prints_as_the_defense_tab_shows_it() {
    let root = seeded("damage-reduction", ALDRIC_ID);
    let mut loaded = load_saved_character_at_root(&root).expect("Aldric loads");
    let basic = request(false, false, false);
    let typeset_text = |loaded: &crate::character_hub::LoadSavedCharacterResponse| {
        let sources = super::SheetSources {
            loaded,
            held: list_held_equipment_at_root(&root).expect("held copies list"),
            custom: load_character_custom_at_root(&root).unwrap_or_default(),
            bio: crate::character_hub::load_character_bio_at_root(&root).expect("bio loads"),
            money: Some(crate::character_hub::load_character_money_at_root(&root).expect("money loads")),
            portrait: None,
            request: &basic,
        };
        let document = crate::print::layout::pf1e::layout(&sources);
        let typeset = crate::print::typst::typeset(&document).expect("the sheet typesets");
        assert!(typeset.warnings.is_empty(), "Typst reports no diagnostics: {:?}", typeset.warnings);
        pdf_text(&typeset.pdf)
    };
    assert_eq!(loaded.snapshot.as_ref().and_then(|snapshot| snapshot.damage_reduction), None, "the seed holds no Damage Reduction");
    let none = typeset_text(&loaded);
    assert!(!none.contains("Damage Reduction"), "no Damage Reduction prints when the screen shows none: {none}");

    loaded.snapshot.as_mut().expect("Aldric computes a snapshot").damage_reduction = Some(3);
    let text = typeset_text(&loaded);
    let hit_points = &text[text.find("Hit Points").expect("Hit Points")..text.find("Abilities").expect("Abilities")];
    assert!(hit_points.contains("Damage Reduction 3/—"), "the square block's Hit Points print `Damage Reduction 3/—`: {hit_points}");
}

/// The screen's and the paper's one vocabulary (SD-40 `decisions.md §5`): every block the
/// Pathfinder sheet prints, option pages too, carries an id the shared fixture lists, once.
/// `sheetLayout.test.ts` holds the fixture's UI-sourced ids to the screen's registry and tabs.
#[test]
fn print_pf_blocks_every_pf_block_id_is_in_the_shared_vocabulary() {
    let vocabulary: serde_json::Value =
        serde_json::from_str(include_str!("../golden/block_vocabulary.json")).expect("the vocabulary fixture is JSON");
    let vocabulary = vocabulary.as_object().expect("the vocabulary is an object of ids");
    for (tag, id) in [("vocabulary-aldric", ALDRIC_ID), ("vocabulary-elowen", ELOWEN_ID)] {
        let root = seeded(tag, id);
        let document = sheet_document_at_root(&root, &request(true, true, true)).expect("the document builds");
        let ids: Vec<&str> = document.blocks.iter().map(|block| block.id).collect();
        for block in &document.blocks {
            assert!(vocabulary.contains_key(block.id), "{tag}: block {:?} prints as id {:?}, which block_vocabulary.json lists", block.heading, block.id);
        }
        let mut unique = ids.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), ids.len(), "{tag}: each block id prints once: {ids:?}");
        assert!(!ids.contains(&"identity") && ids.contains(&"characterDetails"), "{tag}: the identity block is the screen's characterDetails: {ids:?}");
    }
}

/// Each look names one Typst template (SD-40 `decisions.md §4`): `typst.rs`'s `LOOKS` holds one
/// entry per `Look` variant by its JSON name, `resources/print/looks/` holds that file and no other,
/// `blocks.typ` dispatches to it, and a block in that look typesets with no diagnostic.
#[test]
fn print_pf_blocks_every_look_has_its_template() {
    use crate::print::model::{Look, SheetBlock, SheetDocument, SheetRow};
    use crate::print::typst::LOOKS;
    let dispatcher = include_str!("../../../resources/print/blocks.typ");
    let looks_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/print/looks");
    let mut on_disk: Vec<String> = std::fs::read_dir(&looks_dir)
        .expect("resources/print/looks is a directory")
        .map(|entry| entry.expect("a directory entry").file_name().to_string_lossy().into_owned())
        .collect();
    on_disk.sort();
    let mut named: Vec<String> = Look::ALL.iter().map(|look| format!("{}.typ", look.name())).collect();
    named.sort();
    assert_eq!(on_disk, named, "one template file per look, and no file without a look");
    assert_eq!(LOOKS.len(), Look::ALL.len(), "typst.rs holds one LOOKS entry per look");
    for look in Look::ALL {
        let name = look.name();
        assert_eq!(serde_json::to_value(look).expect("a look serialises"), serde_json::json!(name), "{name}: the JSON name is the template's");
        let template = LOOKS.iter().find(|(entry, _)| *entry == name).map(|(_, text)| *text).unwrap_or_else(|| panic!("{name}: LOOKS holds it"));
        assert_eq!(template, std::fs::read_to_string(looks_dir.join(format!("{name}.typ"))).expect("the template reads"), "{name}: LOOKS holds the file");
        assert!(dispatcher.contains(&format!("\"/looks/{name}.typ\"")), "{name}: blocks.typ imports its template");
        let mut block = SheetBlock::new("Look Probe", vec![SheetRow::new("Probe label", ["probe value".to_owned()])]);
        block.look = look;
        let document = SheetDocument { title: "Look probe".to_owned(), paper: "a4", blocks: vec![block] };
        let typeset = crate::print::typst::typeset(&document).unwrap_or_else(|err| panic!("{name}: typesets: {err}"));
        assert!(typeset.warnings.is_empty(), "{name}: Typst reports no diagnostics: {:?}", typeset.warnings);
        let text = pdf_text(&typeset.pdf);
        assert!(text.contains("Look Probe") && text.contains("Probe label") && text.contains("probe value"), "{name}: prints the block: {text}");
    }
}

/// The page grid (SD-40 `decisions.md §4`): the Pathfinder layout places the Combat block beside
/// the Abilities on page 1's 12 columns, and a block placed past the grid's last column is refused
/// by name before Typst sees it, never clipped or wrapped in silence.
#[test]
fn print_pf_blocks_a_place_past_the_grid_is_refused() {
    use crate::print::model::{Place, GRID_COLUMNS};
    let root = seeded("place-past-grid", ALDRIC_ID);
    let mut document = sheet_document_at_root(&root, &request(false, false, false)).expect("the document builds");
    assert_eq!(document.columns(), GRID_COLUMNS, "the Pathfinder sheet prints on the page grid");
    let place = |id: &str| document.blocks.iter().find(|block| block.id == id).unwrap_or_else(|| panic!("{id} prints")).place;
    assert_eq!(place("squareBlock"), Place::at(0, 7), "the square block takes the row's left seven columns (E2.9)");
    assert_eq!(place("abilities"), Place::at(7, 5), "the Abilities sit beside it, on the right");
    assert_eq!(place("attack"), Place::under(7, 5), "the Attack is stacked under the Weapons");
    let square = document.blocks.iter().position(|block| block.id == "squareBlock").expect("the square block");
    assert_eq!(document.blocks[square + 1].id, "abilities", "one row: the square block, then the Abilities beside it");
    crate::print::typst::typeset(&document).expect("the layout's own places typeset");

    document.blocks[square + 1].place = Place::at(8, 5);
    let refused = crate::print::typst::typeset(&document).expect_err("a block past column 12 is refused");
    assert!(refused.contains("print.layout") && refused.contains("abilities") && refused.contains("past the 12-column grid"), "the refusal names the block and the grid: {refused}");
}

/// The page grid's other refusals (`SheetDocument::check_places`, SD-40 `decisions.md §21`): a block
/// that starts past its row's next free column (a gap Typst's auto-placement would fill out of
/// order), a row left open before a page break or the sheet's end, and a block of no width.
#[test]
fn print_pf_blocks_a_row_left_open_or_out_of_order_is_refused() {
    use crate::print::model::{Place, SheetBlock, SheetDocument, SheetRow};
    let block = |id: &'static str, x: u8, span: u8| {
        SheetBlock::new(id, vec![SheetRow::new("Probe", ["1".to_owned()])]).with_id(id).at(Place::at(x, span))
    };
    let sheet = |blocks: Vec<SheetBlock>| SheetDocument { title: "Grid probe".to_owned(), paper: "a4", blocks };

    sheet(vec![block("left", 0, 6), block("right", 6, 6), block("under", 0, 12), block("next", 0, 4).on_new_page(), block("rest", 4, 8)])
        .check_places()
        .expect("full rows, a page break after a full row: printable");

    let gap = sheet(vec![block("left", 0, 4), block("right", 6, 6)]).check_places().expect_err("a gap before `right`");
    assert!(gap.contains("`right`") && gap.contains("starts at column 6") && gap.contains("next free column is 4"), "{gap}");

    let open_end = sheet(vec![block("under", 0, 12), block("left", 0, 6)]).check_places().expect_err("the last row is half full");
    assert!(open_end.contains("the sheet's end") && open_end.contains("ends at column 6"), "{open_end}");

    let open_break = sheet(vec![block("left", 0, 6), block("next", 0, 12).on_new_page()]).check_places().expect_err("a page break mid-row");
    assert!(open_break.contains("`next`'s page break") && open_break.contains("ends at column 6"), "{open_break}");

    let empty = sheet(vec![block("empty", 0, 0)]).check_places().expect_err("a block of no width");
    assert!(empty.contains("`empty`") && empty.contains("past the 12-column grid"), "{empty}");

    // A block stacked `under` another shares that block's cell (SD-40 E2.3): the same columns.
    let under = |id: &'static str, x: u8, span: u8| block(id, x, span).at(Place::under(x, span));
    sheet(vec![block("tall", 0, 7), block("top", 7, 5), under("middle", 7, 5), under("bottom", 7, 5), block("next", 0, 12)])
        .check_places()
        .expect("a column of stacked blocks beside a tall one: printable");
    let astray = sheet(vec![block("left", 0, 6), under("astray", 6, 6), block("right", 6, 6)]).check_places().expect_err("stacked in another cell");
    assert!(astray.contains("`astray`") && astray.contains("stacked under `left`") && astray.contains("not in its cell"), "{astray}");
    let alone = sheet(vec![under("alone", 0, 12)]).check_places().expect_err("stacked under nothing");
    assert!(alone.contains("`alone`") && alone.contains("stacked under no block"), "{alone}");
    let broken = sheet(vec![block("top", 0, 12), under("broken", 0, 12).on_new_page()]).check_places().expect_err("a stacked block on a new page");
    assert!(broken.contains("`broken`") && broken.contains("not in its cell"), "{broken}");
}

/// (block id, heading, look, row label, the row's cells, the engine row behind its number)
type SplitLine<'a> = (&'a str, &'a str, &'a str, &'a str, Vec<String>, Option<&'a str>);

/// The Combat block split along the screen's boxes (SD-40 `decisions.md §8`, E2.1): `hitPoints`,
/// `armorClass`, `initiative`, `saves`, `attack` and `speed` each print in a look of their own, and
/// together they print the same 13 numbers the one Combat block printed, each with the same engine
/// row behind it (Show the math). The write-in cells print empty: no number is invented for them.
#[test]
fn print_pf_blocks_the_split_combat_blocks_print_the_same_values() {
    use crate::print::model::Derivation;
    for (tag, id) in [("split-aldric", ALDRIC_ID), ("split-elowen", ELOWEN_ID), ("split-tomas", TOMAS_ID)] {
        let root = seeded(tag, id);
        let loaded = load_saved_character_at_root(&root).expect("the seed loads");
        let snapshot = loaded.snapshot.as_ref().expect("the seed computes");
        let row = |id: &str| loaded.explanations.iter().find(|row| row.id == id).unwrap_or_else(|| panic!("{tag}: the {id} row"));
        let signed = |n: i16| if n >= 0 { format!("+{n}") } else { n.to_string() };
        let behind = |id: &str| vec![Derivation { source: format!("{id} = {}", row(id).value), detail: row(id).detail.clone() }];
        let document = sheet_document_at_root(&root, &request(false, false, false)).expect("the document builds");
        assert!(document.blocks.iter().all(|block| block.id != "combatStrip" && block.heading != "Combat"), "{tag}: no Combat block is left");
        let block = |id: &str| document.blocks.iter().find(|block| block.id == id).unwrap_or_else(|| panic!("{tag}: the {id} block prints"));

        let w = String::new;
        let expected: Vec<SplitLine> = vec![
            ("squareBlock", "Hit Points", "square-block", "Max", vec![row("hit_points.max").value.to_string()], Some("hit_points.max")),
            ("squareBlock", "Hit Points", "square-block", "Current", vec![w()], None),
            ("squareBlock", "Hit Points", "square-block", "Nonlethal", vec![w()], None),
            ("squareBlock", "Hit Points", "square-block", "Initiative", vec![signed(row("initiative.total").value)], Some("initiative.total")),
            ("armorClass", "Armor Class", "armor-class", "Armor Class", vec![snapshot.baseline_armor_class.to_string()], Some("defense.baseline_armor_class")),
            ("armorClass", "Armor Class", "armor-class", "Touch", vec![row("defense.touch_armor_class").value.to_string()], Some("defense.touch_armor_class")),
            ("armorClass", "Armor Class", "armor-class", "Flat-Footed", vec![row("defense.flat_footed_armor_class").value.to_string()], Some("defense.flat_footed_armor_class")),
            ("armorClass", "Armor Class", "armor-class", "Armor", vec![w()], None),
            ("armorClass", "Armor Class", "armor-class", "Shield", vec![w()], None),
            ("armorClass", "Armor Class", "armor-class", "Dex", vec![w()], None),
            ("armorClass", "Armor Class", "armor-class", "Size", vec![w()], None),
            ("armorClass", "Armor Class", "armor-class", "Natural", vec![w()], None),
            ("armorClass", "Armor Class", "armor-class", "Deflection", vec![w()], None),
            ("armorClass", "Armor Class", "armor-class", "Misc", vec![w()], None),
            ("saves", "Saving Throws", "saves", "Fortitude", vec![signed(snapshot.total_saves.fortitude)], Some("defense.total_save.fortitude")),
            ("saves", "Saving Throws", "saves", "Reflex", vec![signed(snapshot.total_saves.reflex)], Some("defense.total_save.reflex")),
            ("saves", "Saving Throws", "saves", "Will", vec![signed(snapshot.total_saves.will)], Some("defense.total_save.will")),
            ("attack", "Attack", "attack", "Base Attack Bonus", vec![signed(snapshot.base_attack_bonus)], Some("class_chassis.base_attack_bonus")),
            ("attack", "Attack", "attack", "Melee", vec![signed(snapshot.baseline_melee_attack_bonus)], Some("combat.baseline_melee_attack_bonus")),
            ("attack", "Attack", "attack", "CMB", vec![signed(row("combat.combat_maneuver_bonus").value)], Some("combat.combat_maneuver_bonus")),
            ("attack", "Attack", "attack", "CMD", vec![row("defense.combat_maneuver_defense").value.to_string()], Some("defense.combat_maneuver_defense")),
            ("speed", "Speed", "speed", "Land", vec![format!("{} ft.", row("speed.land").value)], Some("speed.land")),
        ];
        let mut numbers = 0;
        for (id, heading, look, label, cells, source) in &expected {
            let printed = block(id);
            assert_eq!((printed.heading.as_str(), printed.look.name()), (*heading, *look), "{tag}: {id}'s heading and look");
            let line = printed.rows.iter().find(|line| line.label == *label).unwrap_or_else(|| panic!("{tag}: {id} prints `{label}`"));
            // The square block pads every line to its widest (Encumbrance's seven cells).
            let values: Vec<String> = line.values.iter().take(cells.len()).cloned().collect();
            assert!(line.values[cells.len()..].iter().all(String::is_empty), "{tag}: {id} `{label}` pads with empty cells only");
            assert_eq!(&values, cells, "{tag}: {id} `{label}` prints the engine's value, write-in cells empty");
            assert_eq!(line.derivation, source.map(behind), "{tag}: {id} `{label}` has the same engine row behind it");
            numbers += usize::from(source.is_some());
        }
        assert_eq!(numbers, 13, "{tag}: the 13 numbers the Combat block printed");
        for id in ["armorClass", "saves", "attack", "speed"] {
            let rows = expected.iter().filter(|line| line.0 == id).count();
            assert_eq!(block(id).rows.len(), rows, "{tag}: {id} prints exactly its rows");
        }
        // The square block also prints Encumbrance and Conditions (E2.9); no Damage Reduction on a seed.
        assert_eq!(block("squareBlock").rows.len(), 4 + 2, "{tag}: the square block's rows");

        let text = pdf_text(&render(&root, &request(false, false, false)));
        assert_in_order(&text, &["Speed", "Armor Class", "Hit Points", "Initiative", "Skills", "Attack", "Saving Throws"]);
        for line in [
            format!("Max {}", row("hit_points.max").value),
            format!("Touch {}", row("defense.touch_armor_class").value),
            format!("CMD {}", row("defense.combat_maneuver_defense").value),
            format!("Land {} ft.", row("speed.land").value),
        ] {
            assert!(text.contains(&line), "{tag}: the PDF prints `{line}`: {text}");
        }
    }
}

/// The Abilities look (SD-40 E2.1; E2.10, `decisions.md §23.1` item 3b): six lines, each the score
/// and the modifier the engine holds, white space beside them (no temp boxes).
#[test]
fn print_pf_blocks_abilities_print_six_boxes_with_write_in_cells() {
    let root = seeded("abilities-boxes", ALDRIC_ID);
    let loaded = load_saved_character_at_root(&root).expect("Aldric loads");
    let scores = &loaded.ability_scores;
    let modifiers = &loaded.snapshot.as_ref().expect("Aldric computes").ability_modifiers;
    let signed = |n: i16| if n >= 0 { format!("+{n}") } else { n.to_string() };
    let document = sheet_document_at_root(&root, &request(false, false, false)).expect("the document builds");
    let abilities = document.blocks.iter().find(|block| block.id == "abilities").expect("the Abilities block");
    assert_eq!(abilities.look.name(), "abilities", "the Abilities print in their own look");
    let expected = [
        ("Strength", scores.strength, modifiers.strength),
        ("Dexterity", scores.dexterity, modifiers.dexterity),
        ("Constitution", scores.constitution, modifiers.constitution),
        ("Intelligence", scores.intelligence, modifiers.intelligence),
        ("Wisdom", scores.wisdom, modifiers.wisdom),
        ("Charisma", scores.charisma, modifiers.charisma),
    ];
    assert_eq!(abilities.rows.len(), 6, "six ability boxes");
    for ((label, score, modifier), row) in expected.iter().zip(&abilities.rows) {
        assert_eq!(row.label, *label);
        assert_eq!(row.values, vec![score.to_string(), signed(*modifier)], "{label}: score and modifier only");
        assert!(row.derivation.is_some(), "{label}: the modifier keeps its engine row");
    }
    let text = pdf_text(&render(&root, &request(false, false, false)));
    assert!(text.contains(&format!("Strength {} {}", scores.strength, signed(modifiers.strength))), "the PDF prints Strength's score and modifier: {text}");
    for caption in ["Temp score", "Temp modifier"] {
        assert!(!text.contains(caption), "no temp column is captioned `{caption}`: {text}");
    }
}

/// The Skills look (SD-40 E2.2): the 35 Skills-panel rows (`skill.total.<skill>`) in two narrow
/// columns, each the class-skill box (marked where the load response states the class skill's +3,
/// else empty for the player, SD-n), the name with the armor-check-penalty mark the converted skill
/// record holds (`ACHECK`), the total, the key ability and its modifier (the number the Abilities
/// block prints), the ranks and an empty misc write-in. The block spans 7 of the 12 columns with the
/// Weapons beside it; on Letter it prints whole on one page, beside the Weapons.
#[test]
fn print_pf_blocks_skills_print_every_panel_row_in_two_columns() {
    use crate::print::model::Place;
    // The Core Rulebook's key ability per Skills-panel row (Table 4-1), held apart from the engine's sets.
    let key = |slug: &str| -> &str {
        if slug.starts_with("knowledge_") {
            return "Int";
        }
        match slug {
            "climb" | "swim" => "Str",
            "acrobatics" | "disable_device" | "escape_artist" | "fly" | "ride" | "sleight_of_hand" | "stealth" => "Dex",
            "appraise" | "craft" | "linguistics" | "spellcraft" => "Int",
            "heal" | "perception" | "profession" | "sense_motive" | "survival" => "Wis",
            "bluff" | "diplomacy" | "disguise" | "handle_animal" | "intimidate" | "perform" | "use_magic_device" => "Cha",
            other => panic!("`{other}` is no Skills-panel row"),
        }
    };
    // Table 4-1's armor check penalty column: exactly these nine.
    let armor_check = ["acrobatics", "climb", "disable_device", "escape_artist", "fly", "ride", "sleight_of_hand", "stealth", "swim"];
    let signed = |n: i16| if n >= 0 { format!("+{n}") } else { n.to_string() };
    for (tag, id) in [("skills-look-aldric", ALDRIC_ID), ("skills-look-elowen", ELOWEN_ID), ("skills-look-tomas", TOMAS_ID)] {
        let root = seeded(tag, id);
        let loaded = load_saved_character_at_root(&root).expect("the seed loads");
        let modifiers = &loaded.snapshot.as_ref().expect("the seed computes").ability_modifiers;
        let modifier = |abbr: &str| match abbr {
            "Str" => modifiers.strength,
            "Dex" => modifiers.dexterity,
            "Int" => modifiers.intelligence,
            "Wis" => modifiers.wisdom,
            _ => modifiers.charisma,
        };
        let document = sheet_document_at_root(&root, &request(false, false, false)).expect("the document builds");
        let skills = document.blocks.iter().find(|block| block.id == "skills").expect("the Skills block");
        assert_eq!(skills.look.name(), "skills", "{tag}: the Skills print in their own look");
        assert_eq!(skills.place, Place::at(0, 7), "{tag}: the Skills span 7 of the 12 columns");
        let weapons = document.blocks.iter().find(|block| block.id == "weapons").expect("the Weapons block");
        assert_eq!(weapons.place, Place::at(7, 5), "{tag}: the Weapons print beside the Skills");

        let totals: Vec<_> = loaded.explanations.iter().filter(|row| row.id.starts_with("skill.total.")).collect();
        assert_eq!(totals.len(), 35, "{tag}: one total per Skills panel row");
        let panel_rows: Vec<_> = skills.rows.iter().filter(|row| row.derivation.is_some()).collect();
        assert_eq!(panel_rows.len(), 35, "{tag}: the block prints the 35 panel rows: {:?}", skills.rows);
        let mut marked = 0;
        let mut lines = Vec::new();
        for total in &totals {
            let slug = total.id.strip_prefix("skill.total.").expect("a skill total");
            let name = super::skills::skill_name(slug);
            let row = panel_rows.iter().find(|row| row.label == name).unwrap_or_else(|| panic!("{tag}: the `{name}` row"));
            let ranks: u32 = loaded
                .skill_allocations
                .iter()
                .filter(|allocation| allocation.skill_id.strip_prefix("skill:") == Some(slug))
                .map(|allocation| u32::from(allocation.ranks))
                .sum();
            let class_skill = total.detail.contains("class skill +3");
            marked += usize::from(class_skill);
            let abbr = key(slug);
            let ability = format!("{abbr} {}", signed(modifier(abbr)));
            let acp = armor_check.contains(&slug);
            let mark = |on: bool, text: &str| if on { text.to_owned() } else { String::new() };
            assert_eq!(
                row.values,
                vec![mark(class_skill, "X"), signed(total.value), ability.clone(), ranks.to_string(), String::new(), mark(acp, "*")],
                "{tag}: `{name}` prints its class-skill mark, total, ability, ranks, the misc write-in and its armor-check mark"
            );
            assert_eq!(row.derivation.as_deref(), Some(&[crate::print::model::Derivation { source: format!("{} = {}", total.id, total.value), detail: total.detail.clone() }][..]));
            lines.push(format!("{}{}{} {} {} {}", mark(class_skill, "X "), name, mark(acp, "*"), signed(total.value), ability, ranks));
        }
        let ranked_class_skills = loaded.explanations.iter().filter(|row| row.id.starts_with("skill.total.") && row.detail.contains("class skill +3")).count();
        assert_eq!(marked, ranked_class_skills, "{tag}: every class skill the load response states is marked");
        assert!(marked >= 3, "{tag}: the seed has at least three ranked class skills stated ({marked})");

        // On Letter the Skills print whole on one page, the Weapons beside them: every panel line
        // is on the page that holds the block's first line.
        let letter = PrintRequest { paper: Paper::Letter, ..request(false, false, false) };
        let pdf = render(&root, &letter);
        let parsed = lopdf::Document::load_mem(&pdf).expect("the PDF parses");
        let pages: Vec<String> = (1..=parsed.get_pages().len() as u32)
            .map(|page| parsed.extract_text(&[page]).expect("the page's text").split_whitespace().collect::<Vec<_>>().join(" "))
            .collect();
        let first = super::skills::skill_name(totals.first().expect("a total").id.strip_prefix("skill.total.").expect("a skill total"));
        let page = pages.iter().find(|text| text.contains(&first)).unwrap_or_else(|| panic!("{tag}: `{first}` prints on a Letter page"));
        assert!(page.contains("Weapons"), "{tag}: the Weapons print on the Skills' Letter page: {page}");
        let text = pdf_text(&pdf);
        let block = &text[text.find("Skills").expect("Skills")..text.find("Weapons").expect("Weapons")];
        for line in &lines {
            assert!(block.contains(line.as_str()), "{tag}: the Skills block prints `{line}`: {block}");
            let name = line.trim_start_matches("X ").split(['*', '+', '-']).next().unwrap_or_default().trim().to_owned();
            assert!(page.contains(&name), "{tag}: `{name}` prints on the Skills' Letter page: {page}");
        }
        for caption in ["Tot.", "Abil.", "Rk.", "Misc", "key ability modifier", "ranks", "class skill", "armor check penalty"] {
            assert!(block.contains(caption), "{tag}: the columns are captioned `{caption}`: {block}");
        }
    }
}

/// The Weapons look (SD-40 E2.3; E2.10 totals only): one table, a line per weapon in hand, its cells
/// the attack total (`combat.weapon_attack_bonus.<weapon>`), the damage total
/// (`combat.weapon_damage_total.<weapon>`, E2.14), the critical and the range; the engine's four
/// per-weapon rows behind that line, never lines of their own; the feat notes under the line; then
/// two blank lines and an ammunition write-in for the pencil.
#[test]
fn print_pf_blocks_weapons_print_as_one_table() {
    use crate::print::model::Derivation;
    let signed = |n: i16| if n >= 0 { format!("+{n}") } else { n.to_string() };
    for (tag, id) in [("weapons-aldric", ALDRIC_ID), ("weapons-elowen", ELOWEN_ID), ("weapons-tomas", TOMAS_ID)] {
        let root = seeded(tag, id);
        let loaded = load_saved_character_at_root(&root).expect("the seed loads");
        let document = sheet_document_at_root(&root, &request(false, false, false)).expect("the document builds");
        let weapons = document.blocks.iter().find(|block| block.id == "weapons").expect("the Weapons block");
        assert_eq!(weapons.look.name(), "weapons", "{tag}: the Weapons print in their own look");
        assert!(!loaded.weapon_damage.is_empty(), "{tag}: the seed holds a weapon in hand");
        // The four rows behind a weapon's line: attack, damage total (SD-40 E2.14), damage bonus, threat.
        let per_weapon: Vec<_> = loaded.explanations.iter().filter(|row| row.id.starts_with("combat.weapon_")).collect();
        assert_eq!(per_weapon.len(), 4 * loaded.weapon_damage.len(), "{tag}: four engine rows per weapon in hand");
        for weapon in &loaded.weapon_damage {
            // The engine keys a weapon's rows by its record's name, letters and digits only.
            let key = weapon.weapon_record_key.as_deref().expect("the seed's weapon resolves");
            let key = key.strip_prefix("item:").unwrap_or(key);
            let slug: String = key.strip_suffix(" (Base)").unwrap_or(key).chars().filter(char::is_ascii_alphanumeric).map(|c| c.to_ascii_lowercase()).collect();
            let row = |prefix: &str| per_weapon.iter().find(|row| row.id == format!("{prefix}{slug}")).unwrap_or_else(|| panic!("{tag}: {prefix}{slug}"));
            let (attack, total, damage, threat) =
                (row("combat.weapon_attack_bonus."), row("combat.weapon_damage_total."), row("combat.weapon_damage_bonus."), row("combat.weapon_threat_range_low."));
            let dice = weapon.base_dice.as_ref().map(|dice| format!("{}d{}", dice.count, dice.die_size)).expect("the seed's weapon has dice");
            let [low, high] = weapon.critical_threat_range.expect("the seed's weapon has a threat range");
            let critical = format!("{}/x{}", if low == high { low.to_string() } else { format!("{low}-{high}") }, weapon.critical_multiplier.expect("a multiplier"));
            let line = weapons
                .rows
                .iter()
                .find(|line| line.values.first() == Some(&signed(attack.value)) && line.values.get(1).is_some_and(|damage| damage.starts_with(&dice)))
                .unwrap_or_else(|| panic!("{tag}: a line for {key}: {:?}", weapons.rows));
            assert_eq!(
                line.values,
                vec![
                    signed(attack.value),
                    if total.value == 0 { dice.clone() } else { format!("{dice}{}", signed(total.value)) },
                    critical.clone(),
                    weapon.range_increment.map_or_else(|| "—".to_owned(), |feet| format!("{feet} ft.")),
                ],
                "{tag}: {key}'s line: attack total, damage total, critical, range"
            );
            let behind: Vec<Derivation> =
                [attack, total, damage, threat].iter().map(|row| Derivation { source: format!("{} = {}", row.id, row.value), detail: row.detail.clone() }).collect();
            assert_eq!(line.derivation.as_deref(), Some(&behind[..]), "{tag}: {key}'s line keeps the four engine rows behind it");
        }
        for line in &weapons.rows {
            assert!(
                !["Attack bonus (", "Damage bonus (", "Threat range from ("].iter().any(|prefix| line.label.starts_with(prefix)),
                "{tag}: a weapon's engine row is a cell of its line, not a line `{}`",
                line.label
            );
        }
        let text = pdf_text(&render(&root, &request(false, false, false)));
        let block = &text[text.find("Weapons").expect("Weapons")..];
        for caption in ["Weapon", "Attack", "Damage", "Critical", "Range", "Ammunition"] {
            assert!(block.contains(caption), "{tag}: the Weapons look prints `{caption}`: {block}");
        }
    }
}

/// Page 1 is whole (operator ruling R5, SD-40 `technical-design.md §4`): the PDF's text before
/// page 1's footer holds the character's details and every step-1 block, each found by its heading
/// and its first printed words; and the text after that footer starts with the block page 2 opens
/// with (`characterDetails`, E2.5; `defense` before it), so no page-1 block ran over onto page 2 (E2.MC: an anchor alone passes a
/// block whose tail spilled).
fn assert_page_one_whole(tag: &str, document: &crate::print::model::SheetDocument, pdf: &[u8]) {
    use crate::print::print_golden::{block_anchor, page_one, PAGE_ONE_BLOCKS};
    let unspaced = |text: &str| text.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    let text = pdf_text(pdf);
    let pages = page_count(pdf);
    let one = page_one(&text, &document.title, pages).unwrap_or_else(|| panic!("{tag}: page 1's footer prints: {text}"));
    let first = unspaced(one);
    for block_id in PAGE_ONE_BLOCKS {
        let block = document.blocks.iter().find(|block| block.id == block_id).unwrap_or_else(|| panic!("{tag}: the {block_id} block"));
        assert!(first.contains(&block_anchor(block)), "{tag}: page 1 holds {:?}; page 1 reads: {first}", block.heading);
    }
    let (_, after) = text[one.len()..].split_once(&format!("page 1 of {pages}")).unwrap_or_else(|| panic!("{tag}: page 1's footer"));
    let second = document.blocks.iter().find(|block| block.id == "characterDetails").expect("the Identity block");
    assert!(
        unspaced(after).starts_with(&block_anchor(second)),
        "{tag}: page 2 starts with {:?}, nothing of page 1 ran over; page 2 reads: {}",
        second.heading,
        &after[..after.len().min(400)]
    );
}

/// Page 1 (operator ruling R5, SD-40 `technical-design.md §4`): on Letter and on A4, the first
/// page holds the character's details and every step-1 block (abilities, hit points, initiative,
/// armor class, attack, saves, speed, skills, weapons), whole (`assert_page_one_whole`); with three
/// weapons in hand too.
#[test]
fn print_pf_blocks_page_one_holds_the_step_one_blocks_on_both_papers() {
    for (tag, id) in [("page-one-aldric", ALDRIC_ID), ("page-one-elowen", ELOWEN_ID), ("page-one-tomas", TOMAS_ID)] {
        let root = seeded(tag, id);
        for paper in [Paper::Letter, Paper::A4] {
            let printed = PrintRequest { paper, ..request(false, false, false) };
            let document = sheet_document_at_root(&root, &printed).expect("the document builds");
            assert_page_one_whole(&format!("{tag} {paper:?}"), &document, &render(&root, &printed));
        }
    }
    // A character with three weapons in hand (the seed's longsword line three times over: the
    // seeds hold one each) still prints the ten blocks on page 1, on Letter and on A4.
    let root = seeded("page-one-three-weapons", ALDRIC_ID);
    for paper in [Paper::Letter, Paper::A4] {
        let mut document = sheet_document_at_root(&root, &PrintRequest { paper, ..request(false, false, false) }).expect("the document builds");
        let weapons = document.blocks.iter_mut().find(|block| block.id == "weapons").expect("the Weapons block");
        let line = weapons.rows[1].clone();
        weapons.rows.splice(2..2, [line.clone(), line]);
        let typeset = crate::print::typst::typeset(&document).expect("three weapons typeset");
        assert!(typeset.warnings.is_empty(), "three weapons {paper:?}: Typst reports no diagnostics: {:?}", typeset.warnings);
        assert_page_one_whole(&format!("three weapons {paper:?}"), &document, &typeset.pdf);
    }
}

/// E2.MC's population probe (SD-40 `epic-breakdown.md` E2.MC): page 1 stays whole, with Typst
/// diagnostics 0, on Letter and A4, for shapes the seeds do not hold: Aldric with a dagger and a
/// Ring of Protection +1 bought at the Market (standard price) and worn, so a second weapon line
/// and a deflection bonus print; Aldric with every skill rank taken away; and Elowen.
#[test]
fn print_pf_blocks_page_one_stays_whole_with_a_bought_weapon_and_ring_with_no_ranks_and_for_elowen() {
    use crate::character_hub::{adjust_character_money_at_root, purchase_equipment_priced_at_root, PriceMode, PurchaseEquipmentResponse};
    use crate::equipment_catalog::{list_market, MarketCategory, MarketFilter};
    use codex::saved_character::local_store::SavedCharacterStore;

    let bought = seeded("page-one-bought", ALDRIC_ID);
    let before = load_saved_character_at_root(&bought).expect("the seed loads");
    adjust_character_money_at_root(&bought, 1_000_000).expect("funds for the Market");
    for (category, name) in [(MarketCategory::Weapon, "Dagger"), (MarketCategory::MagicItem, "Ring of Protection +1")] {
        // The key the Market's row hands the purchase, as the equipment screen does.
        let shelf = list_market(MarketFilter { category, name_contains: Some(name.to_owned()), book: None }).entries;
        let row = shelf.iter().find(|row| row.entry.name == name && row.entry.cost_gp.is_some()).unwrap_or_else(|| panic!("the Market sells {name}"));
        let response = purchase_equipment_priced_at_root(&bought, &row.entry.key, ActiveState::EquippedActive, AT, PriceMode::Standard).expect("the purchase runs");
        assert!(matches!(response, PurchaseEquipmentResponse::Purchased { .. }), "{name} is bought: {response:?}");
    }
    let after = load_saved_character_at_root(&bought).expect("the bought character loads");
    assert_eq!(after.weapon_damage.len(), before.weapon_damage.len() + 1, "the dagger is a second weapon in hand");
    assert_eq!(
        after.corpus_derived.equipment_effects.armor_class_delta,
        before.corpus_derived.equipment_effects.armor_class_delta + 1,
        "the worn ring adds its +1"
    );

    let unranked = seeded("page-one-no-ranks", ALDRIC_ID);
    let mut envelope = SavedCharacterStore::load(&unranked).expect("the seed's envelope loads");
    assert!(envelope.character_input.chosen.skill_allocations.iter().any(|allocation| allocation.ranks > 0), "the seed holds ranks to take away");
    envelope.character_input.chosen.skill_allocations.clear();
    SavedCharacterStore::save(&envelope, &unranked).expect("the unranked character saves");
    let loaded = load_saved_character_at_root(&unranked).expect("the unranked character loads");
    assert!(loaded.skill_allocations.iter().all(|allocation| allocation.ranks == 0), "no skill holds a rank");

    let elowen = seeded("page-one-population-elowen", ELOWEN_ID);
    for (tag, root) in [("bought dagger and ring", &bought), ("no ranks", &unranked), ("elowen", &elowen)] {
        for paper in [Paper::Letter, Paper::A4] {
            let printed = PrintRequest { paper, ..request(false, false, false) };
            let document = sheet_document_at_root(root, &printed).expect("the document builds");
            let weapons = document.blocks.iter().find(|block| block.id == "weapons").expect("the Weapons block");
            if tag == "bought dagger and ring" {
                assert!(weapons.rows.iter().any(|line| line.label.contains("Dagger")), "{tag}: the dagger prints a weapon line: {:?}", weapons.rows);
            }
            assert_page_one_whole(&format!("{tag} {paper:?}"), &document, &render(root, &printed));
        }
    }
}

/// A 2 x 1 RGB PNG: the smallest real portrait a test can save (`save_character_portrait` writes
/// the PNG bytes it is sent to `portrait.png` in the character's directory).
const TEST_PORTRAIT_PNG: [u8; 70] = [
    0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
    0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x7b, 0x40, 0xe8, 0xdd, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9c, 0x63, 0x68, 0x70,
    0x50, 0x00, 0x22, 0x00, 0x06, 0xe7, 0x01, 0xc1, 0x53, 0x8c, 0x54, 0x83, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60,
    0x82,
];

/// How many image objects the PDF holds (each stream whose `/Subtype` is `/Image`).
fn image_objects(pdf: &[u8]) -> usize {
    let document = lopdf::Document::load_mem(pdf).expect("the PDF parses");
    document
        .objects
        .values()
        .filter(|object| match object {
            lopdf::Object::Stream(stream) => matches!(stream.dict.get(b"Subtype"), Ok(lopdf::Object::Name(name)) if name == b"Image"),
            _ => false,
        })
        .count()
}

/// Page 1's text (everything before its footer) for `root` printed on `paper` with `campaigns`.
fn page_one_text(root: &Path, paper: Paper, campaigns: Vec<CampaignNote>) -> (crate::print::model::SheetDocument, String, Vec<u8>) {
    let printed = PrintRequest { paper, campaigns, ..request(false, false, false) };
    let document = sheet_document_at_root(root, &printed).expect("the document builds");
    let pdf = render(root, &printed);
    let text = pdf_text(&pdf);
    let one = crate::print::print_golden::page_one(&text, &document.title, page_count(&pdf))
        .unwrap_or_else(|| panic!("page 1's footer prints: {text}"))
        .to_owned();
    (document, one, pdf)
}

/// E2.5 (`decisions.md §23.1` items 1, 2): page 1 opens with the portrait box and the Character
/// Name and Campaign fields, with no title above them; the name prints once on page 1 (the footer
/// keeps it, §23.6 Q4); the Identity block has left page 1 and opens page 2. Letter and A4.
#[test]
fn print_pf_blocks_page_one_opens_with_the_portrait_and_name_campaign() {
    let root = seeded("header-no-portrait", ALDRIC_ID);
    for paper in [Paper::Letter, Paper::A4] {
        let (document, one, pdf) = page_one_text(&root, paper, Vec::new());
        let name = document.title.clone();
        assert!(one.starts_with("Portrait"), "{paper:?}: page 1 starts with the portrait box: {one}");
        assert_in_order(&one, &["Portrait", "Character Name", &name, "Campaign"]);
        assert_eq!(one.matches(name.as_str()).count(), 1, "{paper:?}: the name prints once on page 1: {one}");
        assert!(!one.contains("Identity"), "{paper:?}: page 1 holds no Identity block: {one}");
        assert_eq!(image_objects(&pdf), 0, "{paper:?}: no portrait saved, no image printed");
        let ids: Vec<&str> = document.blocks.iter().map(|block| block.id).collect();
        assert_eq!(&ids[..2], ["portrait", "nameCampaign"], "{paper:?}: the top band's two blocks lead the sheet");
        let identity = document.blocks.iter().find(|block| block.id == "characterDetails").expect("the Identity block still prints");
        assert!(identity.new_page, "{paper:?}: the Identity block opens page 2");
    }
}

/// E2.5 (`decisions.md §23.1` item 1): a saved `portrait.png` prints inside page 1's portrait box
/// (one image object in the PDF); an unreadable one never fails the print: the empty box prints
/// and the render's warnings name the file.
#[test]
fn print_pf_blocks_a_saved_portrait_prints_on_page_one() {
    let root = seeded("header-portrait", ALDRIC_ID);
    std::fs::write(root.join("portrait.png"), TEST_PORTRAIT_PNG).expect("the portrait saves");
    for paper in [Paper::Letter, Paper::A4] {
        let (_, one, pdf) = page_one_text(&root, paper, Vec::new());
        assert_eq!(image_objects(&pdf), 1, "{paper:?}: the saved portrait prints as one image");
        assert!(one.starts_with("Portrait"), "{paper:?}: the box keeps its caption: {one}");
        if let Ok(out) = std::env::var("PRINT_PF_BLOCKS_OUT") {
            std::fs::create_dir_all(&out).expect("the output dir is creatable");
            std::fs::write(Path::new(&out).join(format!("aldric-portrait-{paper:?}.pdf")), &pdf).expect("the PDF writes");
        }
    }
    std::fs::write(root.join("portrait.png"), b"not a png").expect("the broken portrait writes");
    let rendered = render_sheet_pdf_at_root(&root, &request(false, false, false)).expect("an unreadable portrait never fails the print");
    assert_eq!(image_objects(&rendered.pdf), 0, "the unreadable portrait prints the empty box");
    assert!(
        rendered.warnings.iter().any(|warning| warning.contains("portrait.png")),
        "the warnings name the unreadable file: {:?}",
        rendered.warnings
    );
}

/// E2.5 (`decisions.md §23.1` item 2): the Campaign field prints the names of the campaigns the
/// request carries, joined by ", "; with none, a blank write-in line under the label, never a dash.
#[test]
fn print_pf_blocks_campaign_prints_names_or_a_blank_line() {
    let root = seeded("header-campaigns", ALDRIC_ID);
    let campaigns = vec![
        CampaignNote { name: "Rise of the Runelords".to_owned(), description: "Sandpoint.".to_owned() },
        CampaignNote { name: "Kingmaker".to_owned(), description: String::new() },
    ];
    let (document, one, _) = page_one_text(&root, Paper::Letter, campaigns);
    assert!(one.contains("Campaign Rise of the Runelords, Kingmaker"), "the campaigns' names, joined: {one}");
    assert!(!one.contains("Sandpoint"), "a campaign's description is not page 1's: {one}");
    let header = document.blocks.iter().find(|block| block.id == "nameCampaign").expect("the nameCampaign block");
    assert_eq!(header.rows.last().map(|row| row.values.clone()), Some(vec!["Rise of the Runelords, Kingmaker".to_owned()]));

    let (document, one, _) = page_one_text(&root, Paper::Letter, Vec::new());
    let header = document.blocks.iter().find(|block| block.id == "nameCampaign").expect("the nameCampaign block");
    assert_eq!(header.rows.last().map(|row| (row.label.clone(), row.values.clone())), Some(("Campaign".to_owned(), vec![String::new()])), "no campaign: a write-in cell");
    let campaign = &one[one.find("Campaign").expect("the Campaign label prints")..];
    assert!(!campaign.starts_with("Campaign —") && !campaign.starts_with("Campaign -"), "no campaign prints no dash: {campaign}");
}

/// The square block's text in the PDF: from its heading to the Abilities block after it.
fn square_text(text: &str) -> &str {
    let from = text.find("Hit Points").expect("the square block prints");
    &text[from..from + text[from..].find("Abilities").expect("the Abilities block follows the square block")]
}

/// E2.9 (`decisions.md §23.1` item 3a, §23.3): one `squareBlock` replaces page 1's Hit Points and
/// Initiative blocks: Hit Points (max, the current and nonlethal write-ins), Initiative (the total
/// only), Encumbrance (Light / Medium / Heavy limits, carried, remaining, gear value, the load's
/// name) and Conditions (the `custom.json` conditions the screen shows, else a write-in). Every
/// number is the load response's field, formatted.
#[test]
fn print_pf_blocks_square_block_prints_hp_initiative_encumbrance_and_conditions() {
    use super::pounds;
    let root = seeded("square-aldric", ALDRIC_ID);
    let loaded = load_saved_character_at_root(&root).expect("Aldric loads");
    let row = |id: &str| loaded.explanations.iter().find(|row| row.id == id).unwrap_or_else(|| panic!("the {id} row")).value;
    let signed = |n: i16| if n >= 0 { format!("+{n}") } else { n.to_string() };
    let load = &loaded.corpus_derived.encumbrance;
    assert_eq!(load.level, "Light", "Aldric carries a light load");
    let encumbrance = vec![
        pounds(load.light_max_lbs),
        pounds(load.medium_max_lbs),
        pounds(load.heavy_max_lbs),
        pounds(load.total_carried_weight_lbs),
        pounds(load.remaining_lbs),
        format!("{} gp", super::trim_number(load.total_carried_cost_gp)),
        "Light Load".to_owned(),
    ];
    let square = |root: &Path| {
        let document = sheet_document_at_root(root, &request(false, false, false)).expect("the document builds");
        for retired in ["hitPoints", "initiative"] {
            assert!(document.blocks.iter().all(|block| block.id != retired), "no `{retired}` block is left on the sheet");
        }
        document.blocks.iter().find(|block| block.id == "squareBlock").expect("the squareBlock prints").clone()
    };
    // A row's cells, without the padding `SheetBlock::new` adds to match the widest row.
    let cells = |block: &crate::print::model::SheetBlock, label: &str| {
        let mut values = block.rows.iter().find(|row| row.label == label).unwrap_or_else(|| panic!("the square block prints `{label}`: {:?}", block.rows)).values.clone();
        while values.last().is_some_and(String::is_empty) {
            values.pop();
        }
        values
    };

    let block = square(&root);
    assert_eq!(block.look.name(), "square-block", "the square block prints in its own look");
    assert_eq!(cells(&block, "Max"), vec![row("hit_points.max").to_string()], "Max is hit_points.max");
    assert_eq!(cells(&block, "Current"), Vec::<String>::new(), "Current is the pencil's");
    assert_eq!(cells(&block, "Nonlethal"), Vec::<String>::new(), "Nonlethal is the pencil's");
    assert_eq!(cells(&block, "Initiative"), vec![signed(row("initiative.total"))], "Initiative is the total only");
    assert_eq!(cells(&block, "Encumbrance"), encumbrance, "each encumbrance cell is the response's field");
    assert_eq!(cells(&block, "Conditions"), Vec::<String>::new(), "no condition saved: a write-in box");
    let text = pdf_text(&render(&root, &request(false, false, false)));
    let printed = square_text(&text);
    for piece in ["Initiative", "Encumbrance", "Conditions"].into_iter().map(str::to_owned).chain(encumbrance.iter().cloned()).chain([row("hit_points.max").to_string()]) {
        assert!(printed.contains(&piece), "the square block prints `{piece}`: {printed}");
    }

    let mut custom = load_character_custom_at_root(&root).unwrap_or_default();
    custom.conditions = vec!["Shaken".to_owned()];
    save_character_custom_at_root(&root, &custom, AT).expect("the condition saves");
    assert_eq!(cells(&square(&root), "Conditions"), vec!["Shaken".to_owned()], "the saved condition fills the box");
    let text = pdf_text(&render(&root, &request(false, false, false)));
    let printed = square_text(&text);
    assert!(printed[printed.find("Conditions").expect("Conditions")..].contains("Shaken"), "Shaken prints in the Conditions box: {printed}");
}

/// E2.9 (`decisions.md §23.1` item 3a): under the square block, the engine's notes (E2.6's
/// `conditionalNotes`), each `label: value condition`: the hit point notes, then the initiative
/// notes, each list followed by two blank lines, then two blank lines for encumbrance notes (no
/// engine source, E2.6). E2.6's initiative fixture (APG trait Veteran of Battle, saved on Aldric)
/// prints its note under Initiative; Aldric as seeded holds none, so only the captions and the
/// blank lines print.
#[test]
fn print_pf_blocks_square_block_prints_the_engine_notes() {
    use codex::saved_character::local_store::SavedCharacterStore;
    let notes_of = |block: &crate::print::model::SheetBlock, label: &str| {
        block.rows.iter().find(|row| row.label == label).unwrap_or_else(|| panic!("`{label}`: {:?}", block.rows)).notes.clone()
    };
    let square = |root: &Path| {
        let document = sheet_document_at_root(root, &request(false, false, false)).expect("the document builds");
        document.blocks.iter().find(|block| block.id == "squareBlock").expect("the squareBlock prints").clone()
    };

    let plain = seeded("square-notes-aldric", ALDRIC_ID);
    let loaded = load_saved_character_at_root(&plain).expect("Aldric loads");
    assert!(loaded.conditional_notes.hit_points.is_empty() && loaded.conditional_notes.initiative.is_empty(), "Aldric holds no HP or initiative note");
    let block = square(&plain);
    assert_eq!((notes_of(&block, "Max"), notes_of(&block, "Initiative")), (Vec::<String>::new(), Vec::new()), "Aldric: no note, blank lines only");
    let text = pdf_text(&render(&plain, &request(false, false, false)));
    assert_in_order(square_text(&text), &["Hit point notes", "Initiative notes", "Encumbrance notes"]);

    let veteran = seeded("square-notes-veteran", ALDRIC_ID);
    let mut envelope = SavedCharacterStore::load(&veteran).expect("the seed's envelope loads");
    envelope.character_input.chosen.selected_traits.push("trait:trait_veteran_of_battle".to_owned());
    SavedCharacterStore::save(&envelope, &veteran).expect("the trait saves");
    let loaded = load_saved_character_at_root(&veteran).expect("the veteran loads");
    let note = loaded
        .conditional_notes
        .initiative
        .iter()
        .find(|note| note.rule_id == "advanced_players_guide:trait:trait_veteran_of_battle")
        .unwrap_or_else(|| panic!("E2.6's initiative fixture lands in `initiative`: {:?}", loaded.conditional_notes));
    let words = format!("{}: {} {}", note.label, note.value, note.condition);
    assert_eq!(notes_of(&square(&veteran), "Initiative"), vec![words.clone()], "the note, in the engine's words");
    let letter = PrintRequest { paper: Paper::Letter, ..request(false, false, false) };
    let pdf = render(&veteran, &letter);
    // For a reader's own look: the veteran's page 1, with Shaken saved too, when asked for.
    if let Ok(out) = std::env::var("PRINT_PF_BLOCKS_OUT") {
        let mut custom = load_character_custom_at_root(&veteran).unwrap_or_default();
        custom.conditions = vec!["Shaken".to_owned(), "Fatigued".to_owned()];
        save_character_custom_at_root(&veteran, &custom, AT).expect("the conditions save");
        std::fs::create_dir_all(&out).expect("the output dir is creatable");
        std::fs::write(Path::new(&out).join("veteran-shaken-letter.pdf"), render(&veteran, &letter)).expect("the PDF writes");
    }
    let text = pdf_text(&pdf);
    let printed = square_text(&text);
    let initiative_notes = &printed[printed.find("Initiative notes").expect("Initiative notes")..printed.find("Encumbrance notes").expect("Encumbrance notes")];
    assert!(initiative_notes.contains(&words), "`{words}` prints under Initiative notes: {printed}");
}

/// How often `token` prints as a word of its own in `text` (split on whitespace).
fn word_count(text: &str, token: &str) -> usize {
    text.split_whitespace().filter(|word| *word == token).count()
}

/// E2.10 (`decisions.md §23.1` items 3b, 4; §23.3): page 1 prints totals only. On Letter and A4,
/// page 1's text holds none of the breakdown captions (`Temp`, `Base`, `Str`, `Enh.`, `Thr.`,
/// `Wield`) beyond what the blocks E2.10 does not own print as their own words (the Attack box's
/// `Base Attack Bonus`; the Skills' key abilities, E2.11's); the Abilities print score + modifier,
/// the saves their totals, and each weapon line reads `<name> <attack> <damage total> <critical>`
/// from the engine rows (`combat.weapon_attack_bonus.<weapon>`, `combat.weapon_damage_total.<weapon>`).
#[test]
fn print_pf_blocks_page_one_prints_totals_only() {
    let signed = |n: i16| if n >= 0 { format!("+{n}") } else { n.to_string() };
    const CAPTIONS: [&str; 8] = ["Temp", "Base", "Str", "Enh.", "Thr.", "Wield", "Dice", "Dmg"];
    for (tag, id) in [("totals-aldric", ALDRIC_ID), ("totals-elowen", ELOWEN_ID)] {
        let root = seeded(tag, id);
        let loaded = load_saved_character_at_root(&root).expect("the seed loads");
        let row = |id: &str| loaded.explanations.iter().find(|row| row.id == id).unwrap_or_else(|| panic!("{tag}: the {id} row"));
        for paper in [Paper::Letter, Paper::A4] {
            let (document, one, _) = page_one_text(&root, paper, Vec::new());
            // Each caption's words in the page-1 blocks E2.10 does not own, as the document holds them.
            let elsewhere = |token: &str| {
                use crate::print::print_golden::PAGE_ONE_BLOCKS;
                document
                    .blocks
                    .iter()
                    .filter(|block| PAGE_ONE_BLOCKS.contains(&block.id) && !["abilities", "saves", "weapons"].contains(&block.id))
                    .flat_map(|block| std::iter::once(&block.heading).chain(block.rows.iter().flat_map(|r| std::iter::once(&r.label).chain(&r.values).chain(&r.notes))))
                    .map(|piece| word_count(piece, token))
                    .sum::<usize>()
            };
            for caption in CAPTIONS {
                assert_eq!(word_count(&one, caption), elsewhere(caption), "{tag} {paper:?}: page 1 prints no `{caption}` caption: {one}");
            }
            let block = |id: &str| document.blocks.iter().find(|block| block.id == id).unwrap_or_else(|| panic!("{tag}: the {id} block")).clone();
            assert!(block("abilities").rows.iter().all(|r| r.values.len() == 2), "{tag}: an ability prints score + modifier: {:?}", block("abilities").rows);
            assert!(block("saves").rows.iter().all(|r| r.values.len() == 1), "{tag}: a save prints its total: {:?}", block("saves").rows);
            for weapon in &loaded.weapon_damage {
                let key = weapon.weapon_record_key.as_deref().expect("the seed's weapon resolves");
                let slug: String = key.strip_suffix(" (Base)").unwrap_or(key).chars().filter(char::is_ascii_alphanumeric).map(|c| c.to_ascii_lowercase()).collect();
                let dice = weapon.base_dice.as_ref().map(|dice| format!("{}d{}", dice.count, dice.die_size)).expect("the seed's weapon has dice");
                let total = row(&format!("combat.weapon_damage_total.{slug}")).value;
                let damage = if total == 0 { dice } else { format!("{dice}{}", signed(total)) };
                let [low, high] = weapon.critical_threat_range.expect("a threat range");
                let critical = format!("{}/x{}", if low == high { low.to_string() } else { format!("{low}-{high}") }, weapon.critical_multiplier.expect("a multiplier"));
                let name = key.strip_suffix(" (Base)").unwrap_or(key);
                let line = format!("{name} {} {damage} {critical}", signed(row(&format!("combat.weapon_attack_bonus.{slug}")).value));
                assert!(one.contains(&line), "{tag} {paper:?}: the weapon line reads `{line}`: {one}");
            }
        }
    }
}

/// E2.10 (`decisions.md §23.1` item 4): under the three save totals, each save's notes from E2.6
/// (`label: value condition`), a note the rules give all three saves printed once, then two blank
/// lines. A Dwarf (Aldric with his race saved as `race:dwarf`): Hardy's words (CRB p.21) print
/// once under the saves; Bravery (CRB p.55, Will alone) prints too.
#[test]
fn print_pf_blocks_save_notes_print_under_the_saves() {
    use codex::saved_character::local_store::SavedCharacterStore;
    let root = seeded("save-notes-dwarf", ALDRIC_ID);
    let mut envelope = SavedCharacterStore::load(&root).expect("the seed's envelope loads");
    envelope.character_input.chosen.race_id = "race:dwarf".to_owned();
    SavedCharacterStore::save(&envelope, &root).expect("the dwarf saves");
    let loaded = load_saved_character_at_root(&root).expect("the dwarf loads");
    let notes = &loaded.conditional_notes;
    let hardy = notes.fortitude.iter().find(|note| note.label == "Hardy").unwrap_or_else(|| panic!("Hardy lands under Fortitude: {notes:?}"));
    for list in [&notes.reflex, &notes.will] {
        assert!(list.iter().any(|note| note.rule_id == hardy.rule_id), "Hardy lands under each save: {notes:?}");
    }
    let words = |note: &codex::rules_core::pilot_compute::conditional_notes::ConditionalNote| {
        [note.value.as_str(), note.condition.as_str()].into_iter().filter(|part| !part.is_empty()).collect::<Vec<_>>().join(" ")
    };
    let hardy_words = format!("{}: {}", hardy.label, words(hardy));
    let bravery = notes.will.iter().find(|note| note.label == "Bravery").expect("Bravery lands under Will");
    let bravery_words = format!("{}: {}", bravery.label, words(bravery));

    let document = sheet_document_at_root(&root, &request(false, false, false)).expect("the document builds");
    let saves = document.blocks.iter().find(|block| block.id == "saves").expect("the saves block");
    let printed: Vec<&String> = saves.rows.iter().flat_map(|row| &row.notes).collect();
    assert_eq!(printed.iter().filter(|note| ***note == hardy_words).count(), 1, "Hardy prints once: {printed:?}");
    assert!(printed.iter().any(|note| **note == bravery_words), "Bravery prints: {printed:?}");
    for paper in [Paper::Letter, Paper::A4] {
        let (_, one, _) = page_one_text(&root, paper, Vec::new());
        let under = &one[one.find("Saving Throws").expect("the saves print")..];
        assert_eq!(under.matches(&hardy_words).count(), 1, "{paper:?}: Hardy's words print once under the saves: {under}");
        assert!(under.contains(&bravery_words), "{paper:?}: Bravery's words print under the saves: {under}");
    }
}
