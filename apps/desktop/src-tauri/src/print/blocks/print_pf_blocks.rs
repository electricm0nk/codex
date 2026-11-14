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
    add_feat_selection_enforcing_prerequisites_at_root, load_character_bio_at_root, save_character_bio_at_root, CharacterBioDto, list_held_equipment_at_root, load_saved_character_at_root,
    seed_default_characters_at, set_equipment_active_state_at_root, ActiveStateDto,
};
use crate::print::{render_sheet_pdf_at_root, sheet_document_at_root, CampaignNote, Paper, PrintRequest};
use codex::rules_core::character_input::ActiveState;
use std::path::{Path, PathBuf};

const ALDRIC_ID: &str = "00000000-0000-0000-0000-000000000001";
const ELOWEN_ID: &str = "00000000-0000-0000-0000-000000000002";
const TOMAS_ID: &str = "00000000-0000-0000-0000-000000000003";
const AT: &str = "2026-11-05T00:00:00Z";

/// The 25 blocks every Pathfinder sheet prints, in the layout file's order: the Combat block split
/// along the screen's boxes (SD-40 `decisions.md §8`), page 1 laid out on the round-2 grid (E2.18,
/// `decisions.md §24.3.1`: the AC boxes with no heading, then Speed; the left column, then the
/// Abilities over the Skills), the Hit Points and Initiative one square block (E2.9), the Languages
/// under the portrait (E2.13), the Conditional Bonuses box under the Saving Throws (E2.20), the
/// Resistances block under it (E2.22); then the sections, each under its title (E2.23,
/// `decisions.md §24.3.7`): Abilities and Feats, Inventory (Armor and Shields, Gear, Money, E3.2), Bio
/// (the Identity block).
const BASE_HEADINGS: [&str; 26] = [
    "Portrait",
    "Languages",
    "Character Name",
    "",
    "Speed",
    "Hit Points",
    "Weapons",
    "Attack",
    "Saving Throws",
    "Conditional Bonuses",
    "Resistances",
    "Abilities",
    "Skills",
    "Abilities and Feats",
    "Feats",
    "Class Features",
    "Actions",
    "Spells",
    "Custom",
    "Pets",
    "Inventory",
    "Armor and Shields",
    "Gear",
    "Money",
    "Bio",
    "Identity",
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
        // Every option: the Spellbook section before Bio, the Bio notes in Bio, the Math section last.
        let mut expected: Vec<&str> = BASE_HEADINGS[..BASE_HEADINGS.len() - 2].to_vec();
        expected.extend(["Spellbook", SPELLBOOK, "Bio", "Identity", BIO, "Math", MATH]);
        assert_eq!(headings, expected, "{tag}: the 33 blocks and titles, in the layout file's order");
        let full = render(&root, &all);
        // The PDF's text reads the portrait's two-row cell after the name row (E2.18, `print_order`).
        let printed: Vec<&str> = crate::print::print_golden::print_order(&document).iter().map(|block| block.heading.as_str()).collect();
        assert_in_order(&pdf_text(&full), &printed);
        // For a reader's own look (`pdftotext`, a viewer): the full sheet, when asked for.
        if let Ok(out) = std::env::var("PRINT_PF_BLOCKS_OUT") {
            std::fs::create_dir_all(&out).expect("the output dir is creatable");
            std::fs::write(Path::new(&out).join(format!("{tag}-full.pdf")), &full).expect("the PDF writes");
        }

        let none = sheet_document_at_root(&root, &request(false, false, false)).expect("the document builds");
        let headings: Vec<&str> = none.blocks.iter().map(|block| block.heading.as_str()).collect();
        assert_eq!(headings, BASE_HEADINGS.to_vec(), "{tag}: basic prints no option block");
        let text = pdf_text(&render(&root, &request(false, false, false)));
        let printed: Vec<&str> = crate::print::print_golden::print_order(&none).iter().map(|block| block.heading.as_str()).collect();
        assert_in_order(&text, &printed);
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
        let bio_pdf = render(&root, &request(false, false, true));
        let bio = page_count(&bio_pdf);
        let full = page_count(&render(&root, &request(true, true, true)));
        eprintln!("{tag}: pages basic {basic}, spellbook {spellbook}, math {math}, bio {bio}, full {full}");
        assert!(spellbook > basic, "{tag}: Spellbook on adds a page ({basic} -> {spellbook})");
        assert!(math > basic, "{tag}: Show the math on adds a page ({basic} -> {math})");
        // The Bio section prints on every sheet (E2.23); its notes join it, on its pages, when asked.
        assert!(bio >= basic && pdf_text(&bio_pdf).contains(BIO), "{tag}: Bio and campaign notes on prints them in the Bio section ({basic} -> {bio})");
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
        // Skills (E2.11): the name and the load response's total, nothing else.
        format!("Spellcraft {} Stealth", signed(explained("skill.total.spellcraft"))),
        format!("Climb {} Craft", signed(explained("skill.total.climb"))),
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
        // Feats: the engine's budget rows, one line `<pool> <spent> of <granted>` (E3.1).
        format!("Feat slots: level {} of {}", explained("feats.budget.level.spent"), explained("feats.budget.level.granted")),
        // Spells (E3.3): level 3's line, its save DC then its spells per day.
        format!(
            "Level 3 {} {}",
            explained("class_chassis.wizard.spell_save_dc.spell_level_3"),
            explained("class_spell.wizard.total_spells_per_day.spell_level_3")
        ),
        "Fireball".to_owned(),
        // Actions: the race's own traits.
        "Skilled".to_owned(),
    ];
    for line in expected {
        assert!(text.contains(&line), "the PDF prints `{line}`: {text}");
    }
}

/// A Skills line in the PDF's text (SD-40 E2.2; E2.11): the name, then the total.
fn assert_skill_line(tag: &str, skills: &str, name: &str, total: &str) {
    let at = skills
        .match_indices(name)
        .map(|(at, _)| at)
        .find(|at| *at == 0 || skills[..*at].ends_with(' '))
        .unwrap_or_else(|| panic!("{tag}: the Skills block prints `{name}`: {skills}"));
    // E2.11: the name, then its total, and nothing else on the line (no mark, ability or ranks).
    let after = skills[at + name.len()..].split_whitespace().next();
    assert_eq!(after, Some(total), "{tag}: `{name}` prints its total {total} right after its name: {}", &skills[at..]);
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
        // E2.18: the Skills close page 1's right column; the Identity opens page 2.
        let skills = &text[text.find("Skills").expect("Skills")..text.find("Identity").expect("Identity")];
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
            assert_skill_line(tag, skills, &super::skills::skill_name(slug), &signed(total));
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
        let skills = &text[text.find("Skills").expect("Skills")..text.find("Identity").expect("Identity")];
        let totals: Vec<_> = loaded.explanations.iter().filter(|row| row.id.starts_with("skill.total.")).collect();
        assert_eq!(totals.len(), 35, "{tag}: one total per Skills panel row");
        for row in totals {
            let slug = row.id.strip_prefix("skill.total.").expect("a skill total");
            assert_skill_line(tag, skills, &super::skills::skill_name(slug), &signed(row.value));
        }
    }
}

/// The Gear tab's money tiles (PP, GP, SP, CP, `load_character_money`; the Money block's, E3.2) and the
/// Details panel's Size and Vision (the race roster's, `deriveRaceTraits`) print on the basic sheet as
/// the screen shows them.
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
    // The Identity block is the Bio section's, the sheet's last on the basic sheet (E2.23).
    let identity = &text[text.find("Bio Identity").expect("the Bio section opens with the Identity block")..];
    assert!(identity.contains(&format!("Size {}", race.size)), "Size {}: {identity}", race.size);
    assert!(identity.contains(&format!("Vision {}", race.vision)), "Vision {}: {identity}", race.vision);
    // Each coin's label is over its value in its box; the text extraction runs the two together.
    let coins: String = text_between(&text, "Money PP", "Bio Identity").split_whitespace().collect();
    for (label, value) in [("PP", money.platinum), ("GP", money.gold), ("SP", money.silver), ("CP", money.copper)] {
        assert!(coins.contains(&format!("{label}{value}")), "{label} {value}: {coins}");
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
        // E3.3: the stat-block lines (`Casting time: ...`) print as the card's fields; the rest of the
        // rule's words print whole as its text.
        let is_stat = |l: &str| l.split_once(": ").is_some_and(|(label, _)| ["School", "Subschool", "Descriptor", "Casting time", "Components", "Range", "Target or area", "Duration", "Saving throw", "Spell resistance"].contains(&label));
        let words = bare(&line.prose.lines().filter(|l| !is_stat(l)).collect::<Vec<_>>().join(" "));
        assert!(words.len() > 200, "{name}: the rules hold its full text ({} chars)", words.len());
        assert!(bare(spellbook).contains(&words), "the Spellbook prints {name}'s whole text: {spellbook}");
        for stat in line.prose.lines().filter(|l| is_stat(l) && !l.starts_with("School") && !l.starts_with("Descriptor")) {
            assert!(bare(spellbook).contains(&bare(stat)), "the Spellbook prints {name}'s `{stat}`: {spellbook}");
        }
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

/// The Gear block's text: in the Inventory section (its title, then the Armor and Shields block), up
/// to the Money block after it (E3.2).
fn gear_text(text: &str) -> &str {
    let section = &text[text.find("Inventory Armor and Shields").expect("the Inventory section opens with the Armor and Shields block")..];
    text_between(section, "Gear Item", "Money PP")
}

#[test]
fn print_pf_blocks_a_left_behind_item_is_owned_with_no_weight() {
    let root = seeded("left-behind-aldric", ALDRIC_ID);
    set_equipment_active_state_at_root(&root, "item:chain_shirt", ActiveState::Absent, AT).expect("left behind");
    let held = list_held_equipment_at_root(&root).expect("held");
    assert!(held.iter().any(|copy| copy.item_id == "item:chain_shirt" && copy.active_state == ActiveStateDto::Absent));
    let text = pdf_text(&render(&root, &request(false, false, false)));
    let inventory = gear_text(&text);
    // A section's title and its first line are two table rows, which the text extraction runs together.
    let bare: String = inventory.split_whitespace().collect();
    assert!(bare.contains("LeftbehindChainShirtnoweight"), "listed as owned, left behind, with no weight: {inventory}");
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
            // E3.1: the block's class lines carry two cells, so a feature row's value is its first.
            assert_eq!(printed.values.first(), Some(&value), "{tag}: `{label}` prints its value");
            assert!(printed.values[1..].iter().all(String::is_empty), "{tag}: `{label}` prints one value");
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
    assert!(loaded.resistances.damage_reduction.is_empty(), "nor does its resistances field");

    // A barbarian's class DR, as the load response carries it (E2.21): the snapshot's and the
    // resistances field's. It prints in the Resistances block (E2.22, `decisions.md §24.3.6`), not
    // under Hit Points.
    loaded.snapshot.as_mut().expect("Aldric computes a snapshot").damage_reduction = Some(3);
    loaded.resistances.add_damage_reduction(3, "—", "class_feature.barbarian.damage_reduction");
    let text = typeset_text(&loaded);
    let hit_points = &text[text.find("Hit Points").expect("Hit Points")..text.find("Weapons").expect("Weapons")];
    assert!(!hit_points.contains("Damage Reduction"), "the Hit Points print no Damage Reduction: {hit_points}");
    let resistances = &text[text.find("Resistances").expect("the Resistances block prints")..];
    assert!(resistances.contains("Damage Reduction 3/—"), "the Resistances block prints `Damage Reduction 3/—`: {resistances}");
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
    assert_eq!(place("attack"), Place::under(0, 7), "the Attack is stacked in the left column, under the Weapons (E2.18)");
    let abilities = document.blocks.iter().position(|block| block.id == "abilities").expect("the Abilities");
    assert_eq!(document.blocks[abilities - 1].id, "resistances", "one row: the left column's cell, then the Abilities beside it");
    crate::print::typst::typeset(&document).expect("the layout's own places typeset");

    document.blocks[abilities].place = Place::at(8, 5);
    document.blocks[abilities + 1].place = Place::under(8, 5);
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

    // A cell two rows tall (E2.18's portrait): the next row's blocks start beside it, never under it,
    // and the sheet does not break or end while it still holds its rows.
    let tall = |id: &'static str, x: u8, span: u8| block(id, x, span).at(Place::at(x, span).spanning_rows(2));
    sheet(vec![tall("portrait", 0, 3), block("name", 3, 9), block("ac", 3, 6), block("speed", 9, 3), block("body", 0, 12)])
        .check_places()
        .expect("a two-row cell beside two rows: printable");
    let beneath = sheet(vec![tall("portrait", 0, 3), block("name", 3, 9), block("under-it", 0, 12)]).check_places().expect_err("a block in the tall cell's columns");
    assert!(beneath.contains("`under-it`") && beneath.contains("starts at column 0") && beneath.contains("next free column is 3"), "{beneath}");
    let held = sheet(vec![tall("portrait", 0, 3), block("name", 3, 9), block("next", 0, 12).on_new_page()]).check_places().expect_err("a page break while the tall cell holds a row");
    assert!(held.contains("`next`'s page break"), "{held}");
    let ended = sheet(vec![tall("portrait", 0, 3), block("name", 3, 9)]).check_places().expect_err("the sheet ends while the tall cell holds a row");
    assert!(ended.contains("the sheet's end"), "{ended}");
}

/// (block id, heading, look, row label, the row's cells, the engine row behind its number)
type SplitLine<'a> = (&'a str, &'a str, &'a str, &'a str, Vec<String>, Option<&'a str>);

/// The Combat block split along the screen's boxes (SD-40 `decisions.md §8`, E2.1): `hitPoints`,
/// `armorClass`, `initiative`, `saves`, `attack` and `speed` each print in a look of their own, and
/// together they print the same 13 numbers the one Combat block printed, each with the same engine
/// row behind it (Show the math), and the AC's seven parts from the engine's rows (E2.12). The
/// write-in cells print empty: no number is invented for them.
#[test]
fn print_pf_blocks_the_split_combat_blocks_print_the_same_values() {
    use crate::print::model::Derivation;
    for (tag, id) in [("split-aldric", ALDRIC_ID), ("split-elowen", ELOWEN_ID), ("split-tomas", TOMAS_ID)] {
        let root = seeded(tag, id);
        let loaded = load_saved_character_at_root(&root).expect("the seed loads");
        let snapshot = loaded.snapshot.as_ref().expect("the seed computes");
        let row = |id: &str| loaded.explanations.iter().find(|row| row.id == id).unwrap_or_else(|| panic!("{tag}: the {id} row"));
        let signed = |n: i16| if n >= 0 { format!("+{n}") } else { n.to_string() };
        // An AC part (E2.12): signed, `0` where the engine states 0.
        let part = |id: &str| if row(id).value == 0 { "0".to_owned() } else { signed(row(id).value) };
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
            ("armorClass", "", "armor-class", "Armor Class", vec![snapshot.baseline_armor_class.to_string()], Some("defense.baseline_armor_class")),
            ("armorClass", "", "armor-class", "Touch", vec![row("defense.touch_armor_class").value.to_string()], Some("defense.touch_armor_class")),
            ("armorClass", "", "armor-class", "Flat-Footed", vec![row("defense.flat_footed_armor_class").value.to_string()], Some("defense.flat_footed_armor_class")),
            ("armorClass", "", "armor-class", "Armor", vec![part("defense.armor_class.armor")], Some("defense.armor_class.armor")),
            ("armorClass", "", "armor-class", "Shield", vec![part("defense.armor_class.shield")], Some("defense.armor_class.shield")),
            ("armorClass", "", "armor-class", "Dex", vec![part("defense.armor_class.dexterity")], Some("defense.armor_class.dexterity")),
            ("armorClass", "", "armor-class", "Size", vec![part("defense.armor_class.size")], Some("defense.armor_class.size")),
            ("armorClass", "", "armor-class", "Natural", vec![part("defense.armor_class.natural")], Some("defense.armor_class.natural")),
            ("armorClass", "", "armor-class", "Deflection", vec![part("defense.armor_class.deflection")], Some("defense.armor_class.deflection")),
            ("armorClass", "", "armor-class", "Misc", vec![part("defense.armor_class.misc")], Some("defense.armor_class.misc")),
            ("saves", "Saving Throws", "saves", "Fortitude", vec![signed(snapshot.total_saves.fortitude)], Some("defense.total_save.fortitude")),
            ("saves", "Saving Throws", "saves", "Reflex", vec![signed(snapshot.total_saves.reflex)], Some("defense.total_save.reflex")),
            ("saves", "Saving Throws", "saves", "Will", vec![signed(snapshot.total_saves.will)], Some("defense.total_save.will")),
            ("attack", "Attack", "attack", "Base Attack Bonus", vec![signed(snapshot.base_attack_bonus)], Some("class_chassis.base_attack_bonus")),
            ("attack", "Attack", "attack", "Melee", vec![signed(snapshot.baseline_melee_attack_bonus)], Some("combat.baseline_melee_attack_bonus")),
            ("attack", "Attack", "attack", "CMB", vec![signed(row("combat.combat_maneuver_bonus").value)], Some("combat.combat_maneuver_bonus")),
            ("attack", "Attack", "attack", "CMD", vec![row("defense.combat_maneuver_defense").value.to_string()], Some("defense.combat_maneuver_defense")),
            ("speed", "Speed", "speed", "Land", vec![format!("{} ft.", row("speed.land").value)], Some("speed.land")),
            // E2.18: Swim and Fly boxes, empty for the pencil: no seed states either mode.
            ("speed", "Speed", "speed", "Swim", vec![w()], None),
            ("speed", "Speed", "speed", "Fly", vec![w()], None),
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
        assert_eq!(numbers, 13 + 7, "{tag}: the 13 numbers the Combat block printed and the AC's seven parts (E2.12)");
        for id in ["armorClass", "saves", "attack", "speed"] {
            let rows = expected.iter().filter(|line| line.0 == id).count();
            assert_eq!(block(id).rows.len(), rows, "{tag}: {id} prints exactly its rows");
        }
        // The square block also prints Encumbrance and Conditions (E2.9); no Damage Reduction on a seed.
        assert_eq!(block("squareBlock").rows.len(), 4 + 2, "{tag}: the square block's rows");

        let text = pdf_text(&render(&root, &request(false, false, false)));
        assert_in_order(&text, &["Armor Class", "Speed", "Hit Points", "Initiative", "Weapons", "Attack", "Saving Throws", "Abilities", "Skills"]);
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

/// The Skills look (SD-40 E2.2; E2.11 one column): the 35 Skills-panel rows (`skill.total.<skill>`),
/// each the name and the load response's total, with the engine row behind it (Show the math). The
/// block spans 5 of the 12 columns with the Weapons beside it; on Letter it prints whole on one
/// page, beside the Weapons, and prints no breakdown captions.
#[test]
fn print_pf_blocks_skills_print_every_panel_row_in_one_column() {
    use crate::print::model::Place;
    let signed = |n: i16| if n >= 0 { format!("+{n}") } else { n.to_string() };
    for (tag, id) in [("skills-look-aldric", ALDRIC_ID), ("skills-look-elowen", ELOWEN_ID), ("skills-look-tomas", TOMAS_ID)] {
        let root = seeded(tag, id);
        let loaded = load_saved_character_at_root(&root).expect("the seed loads");
        let document = sheet_document_at_root(&root, &request(false, false, false)).expect("the document builds");
        let skills = document.blocks.iter().find(|block| block.id == "skills").expect("the Skills block");
        assert_eq!(skills.look.name(), "skills", "{tag}: the Skills print in their own look");
        assert_eq!(skills.place, Place::under(7, 5), "{tag}: the Skills span 5 of the 12 columns, under the Abilities (E2.18)");
        let weapons = document.blocks.iter().find(|block| block.id == "weapons").expect("the Weapons block");
        assert_eq!(weapons.place, Place::under(0, 7), "{tag}: the Weapons print in the left column, beside the Skills");

        let totals: Vec<_> = loaded.explanations.iter().filter(|row| row.id.starts_with("skill.total.")).collect();
        assert_eq!(totals.len(), 35, "{tag}: one total per Skills panel row");
        let panel_rows: Vec<_> = skills.rows.iter().filter(|row| row.derivation.is_some()).collect();
        assert_eq!(panel_rows.len(), 35, "{tag}: the block prints the 35 panel rows: {:?}", skills.rows);
        let mut lines = Vec::new();
        for total in &totals {
            let name = super::skills::skill_name(total.id.strip_prefix("skill.total.").expect("a skill total"));
            let row = panel_rows.iter().find(|row| row.label == name).unwrap_or_else(|| panic!("{tag}: the `{name}` row"));
            assert_eq!(row.values, vec![signed(total.value)], "{tag}: `{name}` prints its total only");
            assert_eq!(row.derivation.as_deref(), Some(&[crate::print::model::Derivation { source: format!("{} = {}", total.id, total.value), detail: total.detail.clone() }][..]));
            lines.push(format!("{name} {}", signed(total.value)));
        }

        // On Letter the Skills print whole on one page, the Weapons beside them.
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
        let block = &text[text.find("Skills").expect("Skills")..text.find("Abilities and Feats").expect("the next section's title")];
        for line in &lines {
            assert!(block.contains(line.as_str()), "{tag}: the Skills block prints `{line}`: {block}");
            let name = line.rsplit_once(' ').map(|(name, _)| name).unwrap_or_default();
            assert!(page.contains(name), "{tag}: `{name}` prints on the Skills' Letter page: {page}");
        }
        for caption in ["Tot.", "Abil.", "Rk.", "key ability modifier", "class skill", "armor check penalty"] {
            assert!(!block.contains(caption), "{tag}: no breakdown caption `{caption}`: {block}");
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
/// with (the Abilities and Feats section's title, E2.23), so no page-1 block ran over onto page 2 (E2.MC: an anchor alone passes a
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
    let second = document.blocks.iter().find(|block| block.id == "abilitiesAndFeatsTitle").expect("the Abilities and Feats title");
    assert!(
        unspaced(after).starts_with(&block_anchor(second)),
        "{tag}: page 2 starts with {:?}, nothing of page 1 ran over; page 2 reads: {}",
        second.heading,
        &after[..after.len().min(400)]
    );
}

/// Page 1 (operator ruling R5, SD-40 `technical-design.md §4`, `decisions.md §23.3`, E2.4): on
/// Letter and on A4, the first page holds its twelve blocks (portrait, Character Name and Campaign,
/// square block, abilities, armor class, attack, speed, saves, weapons, skills, languages), each
/// whole (`assert_page_one_whole`), for the three seeds; with three weapons in hand too.
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
    // seeds hold one each) still prints the twelve blocks on page 1, on Letter and on A4.
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

/// E2.4 (`decisions.md §23.2` new SD-b, §23.5): page 1's overflow rule. A Dwarf (Aldric with his
/// race saved as `race:dwarf`: Hardy, Bravery and Stability in the Conditional Bonuses box, E2.20)
/// under each stress alone and all at once: six weapon lines, ten more lines in the Conditional
/// Bonuses box (the notes case, `decisions.md §24.4`), a 40-character name, three campaigns.
/// On Letter and A4, Typst diagnostics 0, and:
/// - no page-1 block leaves page 1: each of the twelve `PAGE_ONE_BLOCKS` prints its heading and first
///   words before page 1's footer;
/// - every line prints: each weapon line, conditional bonus, skill line, the name and each campaign
///   is on page 1, or on page 2 after its block's `<heading> (continued)`;
/// - a continuation opens page 2 (nothing else of page 1 ran over), and all four at once on Letter
///   needs one (the rule fires).
#[test]
fn print_pf_blocks_page_one_lists_continue_on_page_two_and_no_block_leaves() {
    use crate::print::model::{SheetDocument, SheetRow};
    use crate::print::print_golden::{block_anchor, page_one, PAGE_ONE_BLOCKS};
    use codex::saved_character::local_store::SavedCharacterStore;
    const NAME: &str = "Aldric Stonehammer of the Western Marche";
    assert_eq!(NAME.chars().count(), 40, "the long name is 40 characters");
    let unspaced = |text: &str| text.chars().filter(|c| !c.is_whitespace()).collect::<String>();

    let root = seeded("overflow-dwarf", ALDRIC_ID);
    let mut envelope = SavedCharacterStore::load(&root).expect("the seed's envelope loads");
    envelope.character_input.chosen.race_id = "race:dwarf".to_owned();
    SavedCharacterStore::save(&envelope, &root).expect("the dwarf saves");
    let three_campaigns: Vec<CampaignNote> = ["Rise of the Runelords", "Curse of the Crimson Throne", "Second Darkness"]
        .into_iter()
        .map(|name| CampaignNote { name: name.to_owned(), description: String::new() })
        .collect();

    let block = |document: &mut SheetDocument, id: &str| -> usize {
        document.blocks.iter().position(|block| block.id == id).unwrap_or_else(|| panic!("the {id} block"))
    };
    let six_weapons = |document: &mut SheetDocument| {
        let at = block(document, "weapons");
        let weapons = &mut document.blocks[at];
        let line = weapons.rows[1].clone();
        let lines = (1..=6).map(|n| SheetRow { label: format!("{} {n}", line.label), ..line.clone() });
        weapons.rows.splice(1..2, lines);
    };
    let ten_conditional = |document: &mut SheetDocument| {
        let at = block(document, "conditionalBonuses");
        let lines = (1..=10).map(|n| SheetRow::new("Skills", [format!("Probe feat {n:02} on Perception: +2 when the stress test says so")]));
        document.blocks[at].rows.extend(lines);
    };
    let long_name = |document: &mut SheetDocument| {
        document.title = NAME.to_owned();
        let at = block(document, "nameCampaign");
        document.blocks[at].rows[0].values[0] = NAME.to_owned();
    };
    type Stress<'a> = &'a dyn Fn(&mut SheetDocument);
    let shapes: [(&str, bool, &[Stress]); 6] = [
        ("dwarf", false, &[]),
        ("six weapons", false, &[&six_weapons]),
        ("ten conditional lines", false, &[&ten_conditional]),
        ("40-character name", false, &[&long_name]),
        ("three campaigns", true, &[]),
        ("all at once", true, &[&six_weapons, &ten_conditional, &long_name]),
    ];
    for (shape, with_campaigns, stresses) in shapes {
        for paper in [Paper::Letter, Paper::A4] {
            let tag = format!("{shape} {paper:?}");
            let campaigns = if with_campaigns { three_campaigns.clone() } else { Vec::new() };
            let mut document = sheet_document_at_root(&root, &PrintRequest { paper, campaigns, ..request(false, false, false) }).expect("the document builds");
            for stress in stresses {
                stress(&mut document);
            }
            let typeset = crate::print::typst::typeset(&document).expect("the stressed sheet typesets");
            assert!(typeset.warnings.is_empty(), "{tag}: Typst reports no diagnostics: {:?}", typeset.warnings);
            if let Ok(out) = std::env::var("PRINT_PF_BLOCKS_OUT") {
                std::fs::create_dir_all(&out).expect("the output dir is creatable");
                std::fs::write(Path::new(&out).join(format!("overflow-{}-{paper:?}.pdf", shape.replace(' ', "-"))), &typeset.pdf).expect("the PDF writes");
            }
            let text = pdf_text(&typeset.pdf);
            let pages = page_count(&typeset.pdf);
            let one = page_one(&text, &document.title, pages).unwrap_or_else(|| panic!("{tag}: page 1's footer prints: {text}"));
            let first = unspaced(one);
            for block_id in PAGE_ONE_BLOCKS {
                let block = document.blocks.iter().find(|block| block.id == block_id).unwrap_or_else(|| panic!("{tag}: the {block_id} block"));
                assert!(first.contains(&block_anchor(block)), "{tag}: no block leaves page 1: page 1 holds {:?}; page 1 reads: {one}", block.heading);
            }
            let (_, after) = text[one.len()..].split_once(&format!("page 1 of {pages}")).unwrap_or_else(|| panic!("{tag}: page 1's footer"));
            let second = unspaced(after.split_once(" page 2 of ").map_or(after, |(page, _)| page));

            // Every line of page 1's lists, by the block that prints it.
            let mut lines: Vec<(&str, String)> = Vec::new();
            for block in &document.blocks {
                let listed: Vec<String> = match block.id {
                    "weapons" => block.rows.iter().skip(1).map(|row| row.label.clone()).collect(),
                    "conditionalBonuses" => block.rows.iter().map(|row| format!("{}: {}", row.label, row.values[0])).collect(),
                    // Each skill line by its name, then each line under the column.
                    "skills" => block.rows.iter().flat_map(|row| if row.values.is_empty() { row.notes.clone() } else { vec![row.label.clone()] }).collect(),
                    "nameCampaign" => block.rows.iter().flat_map(|row| row.values.clone()).filter(|value| !value.is_empty()).collect(),
                    _ => continue,
                };
                lines.extend(listed.into_iter().map(|line| (block.heading.as_str(), line)));
            }
            let mut continued = 0;
            for (heading, line) in &lines {
                let line = unspaced(line);
                if first.contains(&line) {
                    continue;
                }
                let head = unspaced(&format!("{heading} (continued)"));
                let at = second.find(&head).unwrap_or_else(|| panic!("{tag}: `{line}` is not on page 1, so page 2 holds `{heading} (continued)`; page 2 reads: {second}"));
                assert!(second[at..].contains(&line), "{tag}: `{line}` prints under `{heading} (continued)`; page 2 reads: {second}");
                continued += 1;
            }
            if continued == 0 {
                assert!(second.starts_with(&unspaced("Abilities and Feats")), "{tag}: nothing continues, page 2 opens the Abilities and Feats section; page 2 reads: {second}");
            } else {
                let opens = document.blocks.iter().any(|block| second.starts_with(&unspaced(&format!("{} (continued)", block.heading))));
                assert!(opens, "{tag}: page 2 opens with a continuation; page 2 reads: {second}");
            }
            if shape == "all at once" && matches!(paper, Paper::Letter) {
                assert!(continued > 0, "{tag}: page 1 cannot hold every line at once, so a list continues on page 2");
            }
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
/// keeps it, §23.6 Q4); the Identity block has left page 1 (for the Bio section, E2.23). Letter and A4.
#[test]
fn print_pf_blocks_page_one_opens_with_the_portrait_and_name_campaign() {
    let root = seeded("header-no-portrait", ALDRIC_ID);
    for paper in [Paper::Letter, Paper::A4] {
        let (document, one, pdf) = page_one_text(&root, paper, Vec::new());
        let name = document.title.clone();
        // E2.18: the portrait's cell is two rows tall, so the PDF's text reads it after the name row.
        assert!(one.starts_with("Character Name"), "{paper:?}: page 1's text opens with the name row: {one}");
        assert_in_order(&one, &["Character Name", &name, "Campaign", "Portrait", "Languages"]);
        assert_eq!(one.matches(name.as_str()).count(), 1, "{paper:?}: the name prints once on page 1: {one}");
        assert!(!one.contains("Identity"), "{paper:?}: page 1 holds no Identity block: {one}");
        assert_eq!(image_objects(&pdf), 0, "{paper:?}: no portrait saved, no image printed");
        let ids: Vec<&str> = document.blocks.iter().map(|block| block.id).collect();
        assert_eq!(&ids[..3], ["portrait", "languages", "nameCampaign"], "{paper:?}: the top band's blocks lead the sheet (the Languages line under the portrait, E2.13)");
        let next = document.blocks.iter().position(|block| block.new_page).expect("a block opens page 2");
        let identity = document.blocks.iter().position(|block| block.id == "characterDetails").expect("the Identity block still prints");
        assert!(identity > next, "{paper:?}: the Identity block prints after page 1, in the Bio section");
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
        let caption = one.find("Campaign").map(|at| &one[at..]).unwrap_or_default();
        assert!(caption.contains("Portrait"), "{paper:?}: the box keeps its caption (read after the name row, E2.18): {one}");
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

/// E2.9, re-pinned by E2.20 (`decisions.md §24.3.4`): the square block prints no notes, no notes
/// captions and no blank notes lines; the engine's hit point and initiative notes (E2.6's
/// `conditionalNotes`) print in the Conditional Bonuses box, each `<what>: label: value condition`.
/// Aldric as seeded holds none; E2.6's initiative fixture (APG trait Veteran of Battle, saved on
/// Aldric) prints its note in the box under `Initiative`, on Letter page 1.
#[test]
fn print_pf_blocks_square_block_prints_the_engine_notes() {
    use codex::saved_character::local_store::SavedCharacterStore;
    let document_of = |root: &Path| sheet_document_at_root(root, &request(false, false, false)).expect("the document builds");
    let block = |document: &crate::print::model::SheetDocument, id: &str| document.blocks.iter().find(|block| block.id == id).unwrap_or_else(|| panic!("the {id} block")).clone();

    let plain = seeded("square-notes-aldric", ALDRIC_ID);
    let loaded = load_saved_character_at_root(&plain).expect("Aldric loads");
    assert!(loaded.conditional_notes.hit_points.is_empty() && loaded.conditional_notes.initiative.is_empty(), "Aldric holds no HP or initiative note");
    let square = block(&document_of(&plain), "squareBlock");
    assert!(square.rows.iter().all(|row| row.notes.is_empty()), "the square block carries no notes: {:?}", square.rows);
    let text = pdf_text(&render(&plain, &request(false, false, false)));
    for retired in ["Hit point notes", "Initiative notes", "Encumbrance notes"] {
        assert!(!square_text(&text).contains(retired), "no `{retired}` under the square block: {}", square_text(&text));
    }

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
    let document = document_of(&veteran);
    assert!(block(&document, "squareBlock").rows.iter().all(|row| row.notes.is_empty()), "the note leaves the square block");
    let boxed = block(&document, "conditionalBonuses");
    assert!(boxed.rows.iter().any(|row| row.label == "Initiative" && row.values == vec![words.clone()]), "the note, in the engine's words, under Initiative: {:?}", boxed.rows);
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
    let one = crate::print::print_golden::page_one(&text, &document.title, page_count(&pdf)).expect("page 1's footer");
    let boxed = &one[one.find("Conditional Bonuses").expect("the box prints on page 1")..];
    assert!(boxed.contains(&format!("Initiative: {words}")), "`Initiative: {words}` prints in the box: {boxed}");
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

/// E2.10, re-pinned by E2.20 (`decisions.md §24.3.4–5`): the Saving Throws print their three
/// totals and no note; each save's notes print in the Conditional Bonuses box, a note the rules give
/// all three saves once under `Saves`. A Dwarf (Aldric with his race saved as `race:dwarf`): Hardy's
/// words (CRB p.21) print once in the box under `Saves`; Bravery (CRB p.55, Will alone) under `Will`.
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
    let hardy_words = format!("Saves: {}", box_words(hardy));
    let bravery = notes.will.iter().find(|note| note.label == "Bravery").expect("Bravery lands under Will");
    let bravery_words = format!("Will: {}", box_words(bravery));

    let document = sheet_document_at_root(&root, &request(false, false, false)).expect("the document builds");
    let saves = document.blocks.iter().find(|block| block.id == "saves").expect("the saves block");
    assert!(saves.rows.iter().all(|row| row.notes.is_empty()), "the saves carry no notes: {:?}", saves.rows);
    let boxed = document.blocks.iter().find(|block| block.id == "conditionalBonuses").expect("the box");
    let printed: Vec<String> = boxed.rows.iter().map(|row| format!("{}: {}", row.label, row.values[0])).collect();
    assert_eq!(printed.iter().filter(|line| **line == hardy_words).count(), 1, "Hardy prints once: {printed:?}");
    assert!(printed.contains(&bravery_words), "Bravery prints: {printed:?}");
    for paper in [Paper::Letter, Paper::A4] {
        let (_, one, _) = page_one_text(&root, paper, Vec::new());
        let saves_text = &one[one.find("Saving Throws").expect("the saves print")..one.find("Conditional Bonuses").expect("the box prints")];
        assert!(!saves_text.contains("Hardy") && !saves_text.contains("Bravery"), "{paper:?}: no note under the saves: {saves_text}");
        assert_eq!(one.matches(&hardy_words).count(), 1, "{paper:?}: Hardy's words print once: {one}");
        assert!(one.contains(&bravery_words), "{paper:?}: Bravery's words print in the box: {one}");
    }
}

/// Page 1's text lines as the PDF holds them (before the first footer), each line's whitespace
/// folded: a name and a number on one line sit on one baseline. In-process, as `pdftotext -layout`
/// would read them, so the check needs no poppler on the test machine.
fn page_one_lines(pdf: &[u8]) -> Vec<String> {
    let raw = pdf_extract::extract_text_from_mem(pdf).expect("the PDF's text extracts");
    let one = &raw[..raw.find("page 1 of").unwrap_or_else(|| panic!("page 1's footer prints: {raw}"))];
    one.lines().map(|line| line.split_whitespace().collect::<Vec<_>>().join(" ")).filter(|line| !line.is_empty()).collect()
}

/// E2.11 (`decisions.md §23.1` item 3c; §23.3): the Skills print in ONE column, every Skills panel
/// row (`skill.total.<skill>`) on one line holding its name and signed total only: no class-skill
/// box, no `*` mark, no key ability, no ranks, no misc cell, no column captions for them. On Letter
/// and A4, for Aldric and Elowen, no name wraps at the page-1 width and none is shortened (each
/// fits). A Knowledge name shortens to `Know. (<Subject>)` only when the full name would wrap in its
/// cell (the look's `measure`): a probe narrows the column to 3 of the 12 and Knowledge
/// (Dungeoneering) shortens while Knowledge (Local), which fits, does not.
#[test]
fn print_pf_blocks_skills_print_one_line_each_with_name_and_total() {
    use crate::print::model::Place;
    let signed = |n: i16| if n >= 0 { format!("+{n}") } else { n.to_string() };
    const ABILITIES: [&str; 6] = ["Str", "Dex", "Con", "Int", "Wis", "Cha"];
    for (tag, id) in [("skill-lines-aldric", ALDRIC_ID), ("skill-lines-elowen", ELOWEN_ID)] {
        let root = seeded(tag, id);
        let loaded = load_saved_character_at_root(&root).expect("the seed loads");
        let totals: Vec<_> = loaded.explanations.iter().filter(|row| row.id.starts_with("skill.total.")).collect();
        assert_eq!(totals.len(), 35, "{tag}: one total per Skills panel row");
        let names: Vec<String> = totals.iter().map(|row| super::skills::skill_name(row.id.strip_prefix("skill.total.").expect("a skill total"))).collect();
        for paper in [Paper::Letter, Paper::A4] {
            let (document, one, pdf) = page_one_text(&root, paper, Vec::new());
            let skills = document.blocks.iter().find(|block| block.id == "skills").expect("the Skills block");
            for (row, name) in totals.iter().zip(&names) {
                let line = skills.rows.iter().find(|line| &line.label == name).unwrap_or_else(|| panic!("{tag}: the `{name}` row"));
                assert_eq!(line.values, vec![signed(row.value)], "{tag}: `{name}` sends its signed total only");
            }
            let lines = page_one_lines(&pdf);
            for (row, name) in totals.iter().zip(&names) {
                let printed = format!("{name} {}", signed(row.value));
                let (line, at) = lines
                    .iter()
                    .find_map(|line| line.match_indices(&printed).map(|(at, _)| at).find(|at| *at == 0 || line[..*at].ends_with(' ')).map(|at| (line, at)))
                    .unwrap_or_else(|| panic!("{tag} {paper:?}: `{printed}` prints on one line of page 1: {lines:#?}"));
                let after = line[at + printed.len()..].trim_start();
                assert!(!ABILITIES.iter().any(|ability| after.starts_with(&format!("{ability} "))), "{tag} {paper:?}: no key-ability cell after `{printed}`: {line}");
                assert!(!line[..at].ends_with("X "), "{tag} {paper:?}: no class-skill mark before `{name}`: {line}");
                let others: Vec<&String> = names.iter().filter(|other| *other != name && line.contains(&**other)).collect();
                assert!(others.is_empty(), "{tag} {paper:?}: one column: `{name}`'s line holds no other skill {others:?}: {line}");
            }
            for name in &names {
                assert!(!one.contains(&format!("{name}*")), "{tag} {paper:?}: no armor-check mark after `{name}`: {one}");
            }
            for caption in ["Tot.", "Abil.", "Rk.", "key ability modifier", "class skill", "armor check penalty"] {
                assert!(!one.contains(caption), "{tag} {paper:?}: page 1 prints no `{caption}` caption: {one}");
            }
        }
    }

    // The probe: the same Skills in a column 3 of the 12 wide.
    let root = seeded("skill-lines-narrow", ALDRIC_ID);
    let loaded = load_saved_character_at_root(&root).expect("the seed loads");
    let total = |slug: &str| signed(loaded.explanations.iter().find(|row| row.id == format!("skill.total.{slug}")).expect("the total").value);
    let mut document = sheet_document_at_root(&root, &PrintRequest { paper: Paper::Letter, ..request(false, false, false) }).expect("the document builds");
    for block in &mut document.blocks {
        match block.id {
            "squareBlock" => block.place = Place::at(0, 9),
            "weapons" | "attack" | "saves" | "conditionalBonuses" | "resistances" => block.place = Place::under(0, 9),
            "abilities" => block.place = Place::at(9, 3),
            "skills" => block.place = Place::under(9, 3),
            _ => {}
        }
    }
    let typeset = crate::print::typst::typeset(&document).expect("the narrow probe typesets");
    assert!(typeset.warnings.is_empty(), "Typst reports no diagnostics: {:?}", typeset.warnings);
    let lines = page_one_lines(&typeset.pdf);
    let short = format!("Know. (Dungeoneering) {}", total("knowledge_dungeoneering"));
    assert!(lines.iter().any(|line| line.contains(&short)), "the narrow column shortens a Knowledge name that would wrap: `{short}`: {lines:#?}");
    let full = format!("Knowledge (Local) {}", total("knowledge_local"));
    assert!(lines.iter().any(|line| line.contains(&full)), "a Knowledge name that fits is never shortened: `{full}`: {lines:#?}");
}

/// E2.12 (`decisions.md §23.1` item 4, the AC exception): the Armor Class box prints its seven
/// parts (Armor, Shield, Dex, Size, Natural, Deflection, Misc), each the value of E2.7's
/// `defense.armor_class.<part>` row (signed, `0` where the engine states 0) with that row behind
/// it (Show the math). For Aldric and Aldric wearing a Ring of Protection +1 bought at the Market,
/// on Letter and A4, the parts printed on page 1 add up, with 10, to the AC printed beside them;
/// the sum is made here, in the test, never in the print.
#[test]
fn print_pf_blocks_armor_class_prints_every_part_from_the_engine() {
    use crate::character_hub::{adjust_character_money_at_root, purchase_equipment_priced_at_root, PriceMode, PurchaseEquipmentResponse};
    use crate::equipment_catalog::{list_market, MarketCategory, MarketFilter};
    use crate::print::model::Derivation;

    const PARTS: [(&str, &str); 7] = [
        ("Armor", "armor"),
        ("Shield", "shield"),
        ("Dex", "dexterity"),
        ("Size", "size"),
        ("Natural", "natural"),
        ("Deflection", "deflection"),
        ("Misc", "misc"),
    ];
    let aldric = seeded("ac-parts-aldric", ALDRIC_ID);
    let ringed = seeded("ac-parts-ringed", ALDRIC_ID);
    adjust_character_money_at_root(&ringed, 1_000_000).expect("funds for the Market");
    let shelf = list_market(MarketFilter { category: MarketCategory::MagicItem, name_contains: Some("Ring of Protection +1".to_owned()), book: None }).entries;
    let ring = shelf.iter().find(|row| row.entry.name == "Ring of Protection +1" && row.entry.cost_gp.is_some()).expect("the Market sells the ring");
    let bought = purchase_equipment_priced_at_root(&ringed, &ring.entry.key, ActiveState::EquippedActive, AT, PriceMode::Standard).expect("the purchase runs");
    assert!(matches!(bought, PurchaseEquipmentResponse::Purchased { .. }), "the ring is bought: {bought:?}");

    let mut printed_acs = Vec::new();
    for (tag, root) in [("aldric", &aldric), ("aldric + ring", &ringed)] {
        let loaded = load_saved_character_at_root(root).expect("the character loads");
        let row = |id: &str| loaded.explanations.iter().find(|row| row.id == id).unwrap_or_else(|| panic!("{tag}: the {id} row"));
        let shown = |n: i16| if n == 0 { "0".to_owned() } else if n > 0 { format!("+{n}") } else { n.to_string() };
        for paper in [Paper::Letter, Paper::A4] {
            let (document, one, _) = page_one_text(root, paper, Vec::new());
            let block = document.blocks.iter().find(|block| block.id == "armorClass").expect("the Armor Class block");
            let ac = block.rows.iter().find(|line| line.label == "Armor Class").expect("the AC line").values[0].clone();
            let mut sum = 10;
            let mut printed = Vec::new();
            for (label, part) in PARTS {
                let id = format!("defense.armor_class.{part}");
                let line = block.rows.iter().find(|line| line.label == label).unwrap_or_else(|| panic!("{tag}: `{label}` prints"));
                assert_eq!(line.values, vec![shown(row(&id).value)], "{tag} {paper:?}: `{label}` prints the {id} row");
                assert_eq!(
                    line.derivation,
                    Some(vec![Derivation { source: format!("{id} = {}", row(&id).value), detail: row(&id).detail.clone() }]),
                    "{tag} {paper:?}: `{label}` has its row behind it"
                );
                printed.push(format!("{label} {}", line.values[0]));
                sum += line.values[0].parse::<i16>().expect("a printed part is a number");
            }
            let strip = printed.join(" ");
            assert!(one.contains(&strip), "{tag} {paper:?}: page 1 prints `{strip}`: {one}");
            // The AC's box prints just before the strip of its parts (pdf_extract's order).
            let at = one.find(&strip).expect("the strip prints");
            let before: Vec<&str> = one[..at].split_whitespace().rev().take(6).collect();
            assert!(before.contains(&ac.as_str()), "{tag} {paper:?}: the AC {ac} prints in the box above `{strip}`: {before:?}");
            assert_eq!(sum.to_string(), ac, "{tag} {paper:?}: 10 + the printed parts ({strip}) is the printed AC");
            printed_acs.push(sum);
        }
    }
    assert_eq!(printed_acs[2], printed_acs[0] + 1, "the worn ring prints one more AC, in Deflection");
}

/// E2.13 (`decisions.md §23.1` item 6, §23.7 Q1 "Known + one blank line"): page 1's `languages`
/// block (the Skills panel's Languages line) prints the load response's `languages.known` as one
/// comma list with no write-in line (re-pinned by E2.16, `decisions.md §24.3.3`): no bonus-slot
/// count, whatever the character's Intelligence (Aldric the fighter, Elowen the wizard). Letter and A4.
#[test]
fn print_pf_blocks_languages_print_a_comma_list_and_write_in_lines() {
    for (tag, id) in [("languages-aldric", ALDRIC_ID), ("languages-elowen", ELOWEN_ID)] {
        let root = seeded(tag, id);
        let loaded = load_saved_character_at_root(&root).expect("the seed loads");
        let known = loaded.languages.known.join(", ");
        assert!(known.contains("Common"), "{tag}: the engine's known list holds Common: {known}");
        for paper in [Paper::Letter, Paper::A4] {
            let (document, one, _) = page_one_text(&root, paper, Vec::new());
            let block = document.blocks.iter().find(|block| block.id == "languages").unwrap_or_else(|| panic!("{tag} {paper:?}: a languages block prints"));
            assert_eq!(block.heading, "Languages", "{tag} {paper:?}: the heading is the Skills panel's word");
            let values: Vec<&String> = block.rows.iter().flat_map(|row| &row.values).collect();
            assert_eq!(values, [&known], "{tag} {paper:?}: the comma list and no write-in line (E2.16, decisions.md §24.3.3)");
            let at = one.find("Languages").unwrap_or_else(|| panic!("{tag} {paper:?}: page 1 holds `Languages`: {one}"));
            assert!(one[at..].starts_with(&format!("Languages {known}")), "{tag} {paper:?}: the list prints under its heading on page 1: {}", &one[at..]);
            assert!(!one.contains("bonus language"), "{tag} {paper:?}: no bonus-slot count prints: {one}");
        }
    }
}

/// E2.16 (`decisions.md §24.3.3`): Aldric with seven languages stored through the bio save path
/// (E2.27) prints every name on page 1 after `Languages`, whole, on Letter and A4: one comma list,
/// no write-in row, no Typst warning (`render` asserts it).
#[test]
fn print_pf_blocks_seven_languages_print_whole_under_the_portrait() {
    let root = seeded("languages-seven", ALDRIC_ID);
    let seven = crate::print::print_proof::ALDRIC_SEVEN_LANGUAGES;
    let bio = CharacterBioDto { languages: seven.map(str::to_owned).to_vec(), ..load_character_bio_at_root(&root).expect("the bio loads") };
    save_character_bio_at_root(&root, &bio).expect("the seven languages save");
    assert_eq!(load_saved_character_at_root(&root).expect("Aldric loads").languages.known, seven, "the load response reads the stored list");
    for paper in [Paper::Letter, Paper::A4] {
        let (document, one, _) = page_one_text(&root, paper, Vec::new());
        let block = document.blocks.iter().find(|block| block.id == "languages").unwrap_or_else(|| panic!("{paper:?}: a languages block prints"));
        let values: Vec<&String> = block.rows.iter().flat_map(|row| &row.values).collect();
        assert_eq!(values, [&seven.join(", ")], "{paper:?}: one comma list and no write-in row");
        let at = one.find("Languages").unwrap_or_else(|| panic!("{paper:?}: page 1 holds `Languages`: {one}"));
        assert_in_order(&one[at..], &seven);
    }
}

/// E2.18 (`decisions.md §24.3.1`, the operator's round-2 grid): page 1 on Letter and A4, for Aldric
/// and Elowen. Row 1: the portrait (columns 0–2, two rows tall, the Languages under it) beside one
/// Character Name and Campaign block (3–11). Row 2 beside the portrait: the Armor Class boxes with
/// no "Armor Class" heading (3–8), then Speed (9–11). Then the left column (0–6): square block,
/// Weapons, Attack, Saving Throws stacked in one cell; the right column (7–11, one width): Abilities
/// then Skills. The page-1 text reads in that order and says "Armor Class" once (the AC box's label).
#[test]
fn print_pf_blocks_page_one_follows_the_round_two_grid() {
    let place = |x: u8, span: u8, under: bool, rows: u8| serde_json::json!({ "x": x, "span": span, "under": under, "row_span": rows });
    let grid = [
        ("portrait", place(0, 3, false, 2)),
        ("languages", place(0, 3, true, 1)),
        ("nameCampaign", place(3, 9, false, 1)),
        ("armorClass", place(3, 6, false, 1)),
        ("speed", place(9, 3, false, 1)),
        ("squareBlock", place(0, 7, false, 1)),
        ("weapons", place(0, 7, true, 1)),
        ("attack", place(0, 7, true, 1)),
        ("saves", place(0, 7, true, 1)),
        ("conditionalBonuses", place(0, 7, true, 1)),
        ("resistances", place(0, 7, true, 1)),
        ("abilities", place(7, 5, false, 1)),
        ("skills", place(7, 5, true, 1)),
    ];
    for (tag, id) in [("grid-aldric", ALDRIC_ID), ("grid-elowen", ELOWEN_ID)] {
        let root = seeded(tag, id);
        for paper in [Paper::Letter, Paper::A4] {
            let (document, one, _) = page_one_text(&root, paper, Vec::new());
            let placed: Vec<(&str, serde_json::Value)> =
                document.blocks.iter().take(grid.len()).map(|block| (block.id, serde_json::to_value(block.place).expect("a place serialises"))).collect();
            assert_eq!(placed, grid.to_vec(), "{tag} {paper:?}: page 1's blocks, in order, at their places on the round-2 grid");
            let ac = document.blocks.iter().find(|block| block.id == "armorClass").expect("the Armor Class block");
            assert_eq!(ac.heading, "", "{tag} {paper:?}: the AC block has no heading (§24.3.1)");
            assert_eq!(one.matches("Armor Class").count(), 1, "{tag} {paper:?}: `Armor Class` prints once, as the AC box's label: {one}");
            assert_in_order(
                &one,
                &["Character Name", "Campaign", "Portrait", "Languages", "Armor Class", "Touch", "Flat-Footed", "Speed", "Land", "Swim", "Fly", "Hit Points", "Weapons", "Attack", "Saving Throws", "Conditional Bonuses", "Resistances", "Abilities", "Skills"],
            );
        }
    }
}

/// E2.18 (`decisions.md §24.3.2`, §24.6 Q3): the Speed block prints Land, Swim and Fly boxes from
/// `speed.land` / `speed.swim` / `speed.fly` as `<n> ft.`, the fly value followed by
/// `(<maneuverability>)` from `flyManeuverability`; a mode the character lacks prints an empty box
/// (no dash, no 0) with no engine row behind it. Aldric (Human: land only), Aldric saved as an
/// Undine (swim 30) and as a Strix (fly 60, average); Letter and A4.
#[test]
fn print_pf_blocks_speed_prints_land_swim_fly_boxes() {
    use codex::saved_character::local_store::SavedCharacterStore;
    let as_race = |tag: &str, race: &str| {
        let root = seeded(tag, ALDRIC_ID);
        let mut envelope = SavedCharacterStore::load(&root).expect("the seed's envelope loads");
        envelope.character_input.chosen.race_id = race.to_owned();
        SavedCharacterStore::save(&envelope, &root).expect("the race saves");
        root
    };
    let cases = [
        ("aldric", seeded("speed-aldric", ALDRIC_ID), ["30 ft.", "", ""]),
        ("undine", as_race("speed-undine", "race:undine"), ["30 ft.", "30 ft.", ""]),
        ("strix", as_race("speed-strix", "race:strix"), ["30 ft.", "", "60 ft. (average)"]),
    ];
    for (tag, root, expected) in &cases {
        let loaded = load_saved_character_at_root(root).expect("the character loads");
        for paper in [Paper::Letter, Paper::A4] {
            let (document, one, pdf) = page_one_text(root, paper, Vec::new());
            let speed = document.blocks.iter().find(|block| block.id == "speed").expect("the Speed block");
            let lines: Vec<(&str, Vec<String>)> = speed.rows.iter().map(|row| (row.label.as_str(), row.values.clone())).collect();
            let want: Vec<(&str, Vec<String>)> = ["Land", "Swim", "Fly"].into_iter().zip(expected.iter().map(|v| vec![(*v).to_owned()])).collect();
            assert_eq!(lines, want, "{tag} {paper:?}: Land, Swim, Fly boxes");
            for (row, id) in speed.rows.iter().zip(["speed.land", "speed.swim", "speed.fly"]) {
                let held = loaded.explanations.iter().any(|row| row.id == id);
                assert_eq!(row.derivation.is_some(), held, "{tag} {paper:?}: `{}` has the {id} row behind it only when the engine states one", row.label);
                assert!(!row.values.iter().any(|value| value.contains('—') || value == "0"), "{tag} {paper:?}: no dash, no 0: {:?}", row.values);
            }
            for (label, value) in ["Land", "Swim", "Fly"].into_iter().zip(expected) {
                if !value.is_empty() {
                    assert!(one.contains(&format!("{label} {value}")), "{tag} {paper:?}: page 1 prints `{label} {value}`: {one}");
                }
            }
            if let Ok(out) = std::env::var("PRINT_PF_BLOCKS_OUT") {
                std::fs::create_dir_all(&out).expect("the output dir is creatable");
                std::fs::write(Path::new(&out).join(format!("speed-{tag}-{paper:?}.pdf")), &pdf).expect("the PDF writes");
            }
        }
    }
}

/// The `conditional-bonuses` look's own facts about its box (`look_metadata`): the box's lines, the
/// blank ones among them, and its measured height in points.
fn conditional_box(document: &crate::print::model::SheetDocument) -> (u64, u64, f64) {
    let facts = crate::print::typst::look_metadata(document, "conditional-bonuses-box").expect("the sheet typesets");
    assert_eq!(facts.len(), 1, "one Conditional Bonuses box prints: {facts:?}");
    let fact = &facts[0];
    let number = |key: &str| fact[key].as_f64().unwrap_or_else(|| panic!("the box states `{key}`: {fact}"));
    (number("lines") as u64, number("blank") as u64, number("height"))
}

/// The engine's words for `note`, as the box prints them after `<what>: `: `label: value condition`.
fn box_words(note: &codex::rules_core::pilot_compute::conditional_notes::ConditionalNote) -> String {
    let said: Vec<&str> = [note.value.as_str(), note.condition.as_str()].into_iter().filter(|part| !part.is_empty()).collect();
    format!("{}: {}", note.label, said.join(" "))
}

/// E2.20 (`decisions.md §24.3.4–5`, §24.6 Q1): the Conditional Bonuses box, under the Saving Throws
/// in the left column, lists every conditional the engine states, in order HP, Initiative,
/// Fortitude, Reflex, Will (a note on several saves once), AC, CMB, CMD, Skills, each line
/// `<what>: <label>: <value> <condition>`, in an outlined box of 10 lines of which at least 4 are
/// blank. Nothing conditional prints anywhere else on page 1: no `Hit point notes`, `Initiative
/// notes`, `Encumbrance notes`, `Not added to a total`. Letter and A4: Aldric (Bravery), Aldric
/// saved as an Elf (Elven Immunities, on all three saves, once), and a fixture holding a note of
/// every kind (Aldric as a Dwarf with the Veteran of Battle trait: Hardy, Bravery, Stability,
/// Veteran of Battle from the engine; an HP, an AC, a CMB and a skill line added to its load
/// response): more lines than the box holds, so the rest continue on page 2, each once, and the
/// Weapons keep page 1.
#[test]
fn print_pf_blocks_conditional_bonuses_box_lists_every_conditional() {
    use crate::character_custom::CharacterCustomDto;
    use codex::rules_core::pilot_compute::conditional_notes::ConditionalNote;
    use codex::rules_core::pilot_compute::feat_skill_bonus_sheet_rules::FeatSkillNotFolded;
    use codex::saved_character::local_store::SavedCharacterStore;
    const RETIRED: [&str; 4] = ["Hit point notes", "Initiative notes", "Encumbrance notes", "Not added to a total"];
    let as_race = |tag: &str, race: &str, traits: &[&str]| {
        let root = seeded(tag, ALDRIC_ID);
        let mut envelope = SavedCharacterStore::load(&root).expect("the seed's envelope loads");
        envelope.character_input.chosen.race_id = race.to_owned();
        envelope.character_input.chosen.selected_traits.extend(traits.iter().map(|t| (*t).to_owned()));
        SavedCharacterStore::save(&envelope, &root).expect("the race saves");
        root
    };
    let lines_of = |document: &crate::print::model::SheetDocument| -> Vec<String> {
        let block = document.blocks.iter().find(|block| block.id == "conditionalBonuses").unwrap_or_else(|| panic!("the conditionalBonuses block: {:?}", document.blocks.iter().map(|b| b.id).collect::<Vec<_>>()));
        assert_eq!((block.heading.as_str(), block.look.name()), ("Conditional Bonuses", "conditional-bonuses"), "the box's heading and look");
        let at = document.blocks.iter().position(|b| b.id == "conditionalBonuses").expect("placed");
        assert_eq!(document.blocks[at - 1].id, "saves", "the box sits under the Saving Throws");
        assert_eq!((block.place.x, block.place.span, block.place.under), (0, 7, true), "in the left column");
        block.rows.iter().map(|row| format!("{}: {}", row.label, row.values.join(" "))).collect()
    };
    // Each line prints once: in the box on page 1, or (`may_continue`, a box too full for its lines)
    // under `Conditional Bonuses (continued)` at the top of page 2.
    let check_page_one = |tag: &str, document: &crate::print::model::SheetDocument, pdf: &[u8], lines: &[String], may_continue: bool| {
        let text = pdf_text(pdf);
        let pages = page_count(pdf);
        let one = crate::print::print_golden::page_one(&text, &document.title, pages).unwrap_or_else(|| panic!("{tag}: page 1's footer: {text}"));
        let boxed = &one[one.find("Conditional Bonuses").unwrap_or_else(|| panic!("{tag}: the box prints on page 1: {one}"))..];
        let (_, after) = text[one.len()..].split_once(&format!("page 1 of {pages}")).unwrap_or_else(|| panic!("{tag}: page 1's footer"));
        let two = after.split_once(" page 2 of ").map_or(after, |(page, _)| page).trim_start();
        // Matched without whitespace: a line breaks at a word's own hyphen ("spell-like").
        let unspaced = |text: &str| text.chars().filter(|c| !c.is_whitespace()).collect::<String>();
        let continued = unspaced(two.strip_prefix("Conditional Bonuses (continued)").unwrap_or(""));
        assert!(may_continue || continued.is_empty(), "{tag}: every line fits the box: page 2 reads {two}");
        let (one_flat, boxed) = (unspaced(one), unspaced(boxed));
        for line in lines {
            let line = unspaced(line);
            assert_eq!(one_flat.matches(&line).count() + continued.matches(&line).count(), 1, "{tag}: `{line}` prints once: page 1 {one}, page 2 {two}");
            assert!(boxed.contains(&line) || continued.contains(&line), "{tag}: `{line}` prints in the box or its continuation: {boxed}");
        }
        for retired in RETIRED {
            assert!(!one.contains(retired), "{tag}: page 1 prints no `{retired}`: {one}");
        }
        let (lines, blank, _) = conditional_box(document);
        assert_eq!(lines, 10, "{tag}: the box is 10 lines (operator Q1)");
        assert!(blank >= 4, "{tag}: at least 4 blank lines, {blank} printed");
    };

    // Aldric: Bravery, in the box and nowhere else on page 1.
    let aldric = seeded("box-aldric", ALDRIC_ID);
    let loaded = load_saved_character_at_root(&aldric).expect("Aldric loads");
    let bravery = loaded.conditional_notes.will.iter().find(|note| note.label == "Bravery").expect("Aldric holds Bravery");
    let bravery_line = format!("Will: {}", box_words(bravery));
    // Elf: Elven Immunities on every save, printed once.
    let elf = as_race("box-elf", "race:elf", &[]);
    let elf_loaded = load_saved_character_at_root(&elf).expect("the elf loads");
    let immunities = elf_loaded.conditional_notes.will.iter().find(|note| note.label == "Elven Immunities").unwrap_or_else(|| panic!("the elf holds Elven Immunities: {:?}", elf_loaded.conditional_notes));
    assert!(elf_loaded.conditional_notes.fortitude.iter().chain(&elf_loaded.conditional_notes.reflex).filter(|n| n.rule_id == immunities.rule_id).count() == 2, "on all three saves");
    let immunities_line = format!("Saves: {}", box_words(immunities));
    for (tag, root, line) in [("aldric", &aldric, &bravery_line), ("elf", &elf, &immunities_line)] {
        for paper in [Paper::Letter, Paper::A4] {
            let (document, _, pdf) = page_one_text(root, paper, Vec::new());
            let lines = lines_of(&document);
            assert!(lines.contains(line), "{tag} {paper:?}: the box lists `{line}`: {lines:?}");
            check_page_one(&format!("{tag} {paper:?}"), &document, &pdf, &lines, false);
            if let Ok(out) = std::env::var("PRINT_PF_BLOCKS_OUT") {
                std::fs::create_dir_all(&out).expect("the output dir is creatable");
                std::fs::write(Path::new(&out).join(format!("box-{tag}-{paper:?}.pdf")), &pdf).expect("the PDF writes");
            }
        }
    }

    // Every kind: the engine's Dwarf + Veteran of Battle notes, and an HP, AC, CMB and skill line.
    let every = as_race("box-every", "race:dwarf", &["trait:trait_veteran_of_battle"]);
    let mut loaded = load_saved_character_at_root(&every).expect("the dwarf loads");
    let probe = |label: &str, condition: &str| ConditionalNote { label: label.to_owned(), value: "+2".to_owned(), condition: condition.to_owned(), rule_id: format!("probe:{label}") };
    let notes = &mut loaded.conditional_notes;
    assert!(!notes.initiative.is_empty() && !notes.combat_maneuver_defense.is_empty() && !notes.fortitude.is_empty(), "the engine's notes: {notes:?}");
    notes.hit_points.push(probe("Probe Toughness", "when the probe says so"));
    notes.armor_class.push(probe("Probe Dodge", "against the probe"));
    notes.combat_maneuver_bonus.push(probe("Probe Grapple", "to grapple the probe"));
    loaded.feat_skill_bonuses.situational.push(FeatSkillNotFolded {
        skill: "perception".to_owned(),
        rule_id: "probe:alert".to_owned(),
        label: "Probe Alert".to_owned(),
        reason: "+2 when the probe listens".to_owned(),
    });
    let notes = &loaded.conditional_notes;
    let hardy = notes.fortitude.iter().find(|n| n.label == "Hardy").expect("Hardy");
    let mut want = vec![format!("HP: {}", box_words(&notes.hit_points[0]))];
    want.extend(notes.initiative.iter().map(|n| format!("Initiative: {}", box_words(n))));
    want.push(format!("Saves: {}", box_words(hardy)));
    want.extend(notes.will.iter().filter(|n| n.rule_id != hardy.rule_id).map(|n| format!("Will: {}", box_words(n))));
    want.push(format!("AC: {}", box_words(&notes.armor_class[0])));
    want.push(format!("CMB: {}", box_words(&notes.combat_maneuver_bonus[0])));
    want.extend(notes.combat_maneuver_defense.iter().map(|n| format!("CMD: {}", box_words(n))));
    want.push("Skills: Probe Alert on Perception: +2 when the probe listens".to_owned());
    assert!(notes.fortitude.iter().chain(&notes.reflex).all(|n| n.rule_id == hardy.rule_id), "the dwarf's only Fortitude and Reflex note is Hardy: {notes:?}");
    for paper in [Paper::Letter, Paper::A4] {
        let request = PrintRequest { paper, ..request(false, false, false) };
        let sources = super::SheetSources {
            loaded: &loaded,
            held: list_held_equipment_at_root(&every).expect("the copies list"),
            custom: CharacterCustomDto::default(),
            bio: CharacterBioDto::default(),
            money: None,
            portrait: None,
            request: &request,
        };
        let document = crate::print::layout::pf1e::layout(&sources);
        assert_eq!(lines_of(&document), want, "{paper:?}: every kind, under its word, in order");
        let typeset = crate::print::typst::typeset(&document).expect("the sheet typesets");
        assert!(typeset.warnings.is_empty(), "{paper:?}: Typst reports no diagnostics: {:?}", typeset.warnings);
        check_page_one(&format!("every kind {paper:?}"), &document, &typeset.pdf, &want, true);
        if let Ok(out) = std::env::var("PRINT_PF_BLOCKS_OUT") {
            std::fs::create_dir_all(&out).expect("the output dir is creatable");
            std::fs::write(Path::new(&out).join(format!("box-every-kind-{paper:?}.pdf")), &typeset.pdf).expect("the PDF writes");
        }
        let identity = pdf_text(&typeset.pdf);
        assert!(!identity.contains("Weapons (continued)"), "every kind {paper:?}: a full box takes no room from the Weapons: {identity}");
    }
}

/// E2.20 (`decisions.md §24.3.5`, SD-b): 30 notes in the box. On Letter and A4 the box keeps its
/// height (the one-note box's) and its 4 blank lines; every note prints once, on page 1 in the box
/// or at the top of page 2 under `Conditional Bonuses (continued)`; some do continue.
#[test]
fn print_pf_blocks_conditional_bonuses_overflow_continues_on_page_two() {
    use crate::print::model::SheetRow;
    let root = seeded("box-overflow", ALDRIC_ID);
    let unspaced = |text: &str| text.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    for paper in [Paper::Letter, Paper::A4] {
        let tag = format!("thirty {paper:?}");
        let printed = PrintRequest { paper, ..request(false, false, false) };
        let mut document = sheet_document_at_root(&root, &printed).expect("the document builds");
        let at = document.blocks.iter().position(|block| block.id == "conditionalBonuses").expect("the conditionalBonuses block");
        let (_, _, one_note_height) = conditional_box(&document);
        let notes: Vec<String> = (1..=30).map(|n| format!("Probe note {n:02}: +1 when the overflow test says so")).collect();
        document.blocks[at].rows = notes.iter().map(|note| SheetRow::new("Will", [note.clone()])).collect();
        let typeset = crate::print::typst::typeset(&document).expect("thirty notes typeset");
        assert!(typeset.warnings.is_empty(), "{tag}: Typst reports no diagnostics: {:?}", typeset.warnings);
        if let Ok(out) = std::env::var("PRINT_PF_BLOCKS_OUT") {
            std::fs::create_dir_all(&out).expect("the output dir is creatable");
            std::fs::write(Path::new(&out).join(format!("box-thirty-{paper:?}.pdf")), &typeset.pdf).expect("the PDF writes");
        }
        let (lines, blank, height) = conditional_box(&document);
        assert_eq!(lines, 10, "{tag}: the box is 10 lines");
        assert!(blank >= 4, "{tag}: the 4 blank lines stay, {blank} printed");
        assert!((height - one_note_height).abs() < 0.01, "{tag}: the box keeps its height: {height} pt against {one_note_height} pt");
        let text = pdf_text(&typeset.pdf);
        let pages = page_count(&typeset.pdf);
        let one = crate::print::print_golden::page_one(&text, &document.title, pages).unwrap_or_else(|| panic!("{tag}: page 1's footer"));
        let first = unspaced(one);
        let (_, after) = text[one.len()..].split_once(&format!("page 1 of {pages}")).unwrap_or_else(|| panic!("{tag}: page 1's footer"));
        let second = unspaced(after.split_once(" page 2 of ").map_or(after, |(page, _)| page));
        let head = unspaced("Conditional Bonuses (continued)");
        assert!(second.starts_with(&head), "{tag}: page 2 opens with the continuation: {second}");
        let mut continued = 0;
        for note in &notes {
            let line = unspaced(&format!("Will: {note}"));
            let on_one = first.matches(&line).count();
            let on_two = second[head.len()..].matches(&line).count();
            assert_eq!(on_one + on_two, 1, "{tag}: `{note}` prints once, on page 1 ({on_one}) or under the continuation ({on_two})");
            continued += on_two;
        }
        assert!(continued > 0 && continued < notes.len(), "{tag}: some notes fill the box, the rest continue: {continued} of 30 on page 2");
    }
}

/// E2.22 (`decisions.md §24.3.6`): the Resistances block, under the Conditional Bonuses box in the
/// left column, prints the five energy boxes (Acid, Cold, Electricity, Fire, Sonic), Damage
/// Reduction, Spell Resistance and Immunities from the load response's `resistances` (E2.21), each
/// box empty when the character has none; the Hit Points print no Damage Reduction; page 1 stays
/// whole. Letter and A4: Aldric (none), Aldric's root holding an Ifrit (Fire 5), an Undine (Cold
/// 5) and a Human Barbarian 7 (DR 1/—), each built through the creation path's input.
#[test]
fn print_pf_blocks_resistances_print_boxes_and_values() {
    use codex::saved_character::local_store::SavedCharacterStore;
    const LABELS: [&str; 8] = ["Acid", "Cold", "Electricity", "Fire", "Sonic", "Damage Reduction", "Spell Resistance", "Immunities"];
    let unspaced = |text: &str| text.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    // Aldric's saved root holding a character built as `race`, `class`, `level`.
    let built = |tag: &str, race: &str, class: &str, level: u8| {
        let root = seeded(tag, ALDRIC_ID);
        let request: crate::character_hub::CreateCharacterRequest = serde_json::from_value(serde_json::json!({
            "characterId": "char-test", "displayLabel": "Test Character", "raceId": race, "classId": class, "level": level,
            "abilityScores": { "strength": 16, "dexterity": 14, "constitution": 14, "intelligence": 10, "wisdom": 12, "charisma": 8 },
            "abilityBonusTarget": "strength", "savedAt": AT,
        }))
        .expect("a creation request");
        let mut envelope = SavedCharacterStore::load(&root).expect("the seed's envelope loads");
        envelope.character_input = crate::pf1_adapter::compose_character_input(&request);
        SavedCharacterStore::save(&envelope, &root).expect("the character saves");
        root
    };
    type Filled = &'static [(&'static str, &'static str)];
    let cases: [(&str, PathBuf, Filled); 4] = [
        ("aldric", seeded("resist-aldric", ALDRIC_ID), &[]),
        ("ifrit", built("resist-ifrit", "race:ifrit", "class:fighter", 1), &[("Fire", "5")]),
        ("undine", built("resist-undine", "race:undine", "class:fighter", 1), &[("Cold", "5")]),
        ("barbarian", built("resist-barbarian", "race:human", "class:barbarian", 7), &[("Damage Reduction", "1/—")]),
    ];
    for (tag, root, filled) in &cases {
        for paper in [Paper::Letter, Paper::A4] {
            let tag = format!("{tag} {paper:?}");
            let printed = PrintRequest { paper, ..request(false, false, false) };
            let document = sheet_document_at_root(root, &printed).expect("the document builds");
            let at = document.blocks.iter().position(|block| block.id == "resistances").unwrap_or_else(|| panic!("{tag}: the resistances block: {:?}", document.blocks.iter().map(|b| b.id).collect::<Vec<_>>()));
            let block = &document.blocks[at];
            assert_eq!((block.heading.as_str(), block.look.name()), ("Resistances", "resistances"), "{tag}: heading and look");
            assert_eq!(document.blocks[at - 1].id, "conditionalBonuses", "{tag}: under the Conditional Bonuses box");
            assert_eq!((block.place.x, block.place.span, block.place.under), (0, 7, true), "{tag}: in the left column");
            let rows: Vec<(&str, &str)> = block.rows.iter().map(|row| (row.label.as_str(), row.values[0].as_str())).collect();
            let want: Vec<(&str, &str)> = LABELS.iter().map(|label| (*label, filled.iter().find(|(l, _)| l == label).map_or("", |(_, v)| *v))).collect();
            assert_eq!(rows, want, "{tag}: every box, filled from `resistances` or empty");

            let pdf = render(root, &printed);
            let text = pdf_text(&pdf);
            let one = crate::print::print_golden::page_one(&text, &document.title, page_count(&pdf)).unwrap_or_else(|| panic!("{tag}: page 1's footer: {text}"));
            let hit_points = &one[one.find("Hit Points").expect("Hit Points")..one.find("Weapons").expect("Weapons")];
            assert!(!hit_points.contains("Damage Reduction"), "{tag}: no Damage Reduction under Hit Points: {hit_points}");
            let flat = unspaced(one);
            let section = &flat[flat.find("ResistancesAcid").unwrap_or_else(|| panic!("{tag}: the Resistances block prints on page 1: {one}"))..];
            for label in LABELS {
                assert!(section.contains(&unspaced(label)), "{tag}: `{label}` prints: {section}");
            }
            for (label, value) in filled.iter() {
                assert!(section.contains(&unspaced(&format!("{label}{value}"))), "{tag}: `{label} {value}` prints: {section}");
            }
            // The Barbarian's rage conditionals fill the Conditional Bonuses box and continue on page 2
            // (SD-b); every other case leaves nothing of page 1 to page 2.
            if !tag.starts_with("barbarian") {
                assert_page_one_whole(&tag, &document, &pdf);
            }
            if let Ok(out) = std::env::var("PRINT_PF_BLOCKS_OUT") {
                std::fs::create_dir_all(&out).expect("the output dir is creatable");
                std::fs::write(Path::new(&out).join(format!("resist-{}.pdf", tag.replace(' ', "-"))), &pdf).expect("the PDF writes");
            }
        }
    }
}

/// The section titles after Primary, in the operator's order (E2.23, `decisions.md §24.3.7`;
/// Progression's arrives with E3.7).
const SECTION_TITLES: [&str; 5] = ["Abilities and Feats", "Inventory", "Spellbook", "Bio", "Math"];

/// The PDF's text page by page: each page's words before its footer (`<title> · page <n> of <pages>`).
fn pages_text(text: &str, title: &str, pages: usize) -> Vec<String> {
    let title = title.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut rest = text;
    (1..=pages)
        .map(|page| {
            let footer = format!("{title} · page {page} of {pages}");
            let (on_page, after) = rest.split_once(&footer).unwrap_or_else(|| panic!("page {page}'s footer prints: {rest}"));
            rest = after;
            on_page.trim().to_owned()
        })
        .collect()
}

/// E2.23 (`decisions.md §24.3.7`): each section after Primary starts a page under its prominent
/// title and ends with a page break: the titles are each the first text of a page, in order, and
/// each section's blocks are the operator's list, so no page holds blocks of two sections.
/// Aldric with every option, Letter and A4.
#[test]
fn print_pf_blocks_each_section_starts_a_page_under_its_title() {
    let root = seeded("sections", ALDRIC_ID);
    for paper in [Paper::Letter, Paper::A4] {
        let tag = format!("aldric all {paper:?}");
        let printed = PrintRequest { paper, ..request(true, true, true) };
        let document = sheet_document_at_root(&root, &printed).expect("the document builds");
        // The sections as the document holds them: each title, then its blocks up to the next title.
        let mut sections: Vec<(&str, Vec<&str>)> = Vec::new();
        for block in &document.blocks {
            if block.look.name() == "section-title" {
                assert!(block.new_page, "{tag}: the {} title starts a page", block.heading);
                sections.push((block.heading.as_str(), Vec::new()));
            } else if let Some((_, ids)) = sections.last_mut() {
                assert!(!block.new_page, "{tag}: {} breaks no page inside its section", block.heading);
                ids.push(block.id);
            }
        }
        let expected: Vec<(&str, Vec<&str>)> = vec![
            ("Abilities and Feats", vec!["feats", "classFeatures", "actions", "spells", "custom", "pets"]),
            ("Inventory", vec!["defense", "gear", "money"]),
            ("Spellbook", vec!["spellbook"]),
            ("Bio", vec!["characterDetails", "bioNotes"]),
            ("Math", vec!["math"]),
        ];
        assert_eq!(sections, expected, "{tag}: the sections, their titles and their blocks, in the operator's order");

        let pdf = render(&root, &printed);
        let pages = page_count(&pdf);
        let text = pdf_text(&pdf);
        let on_pages = pages_text(&text, &document.title, pages);
        let mut last = 0;
        for title in SECTION_TITLES {
            let at = on_pages
                .iter()
                .position(|page| page.starts_with(title))
                .unwrap_or_else(|| panic!("{tag}: `{title}` is the first text of a page; pages open: {:?}", on_pages.iter().map(|p| p.chars().take(40).collect::<String>()).collect::<Vec<_>>()));
            assert!(at > last, "{tag}: `{title}` opens page {} after the section before it (page {})", at + 1, last + 1);
            last = at;
        }
        if let Ok(out) = std::env::var("PRINT_PF_BLOCKS_OUT") {
            std::fs::create_dir_all(&out).expect("the output dir is creatable");
            std::fs::write(Path::new(&out).join(format!("sections-{paper:?}.pdf")), &pdf).expect("the PDF writes");
        }
    }
}

/// E2.23 (`decisions.md §24.3.7`): Primary (page 1) carries no section title: the document holds a
/// title per section asked for, none of them in page 1's run, and page 1's text prints none.
/// Aldric with no option and with every option, Letter and A4.
#[test]
fn print_pf_blocks_page_one_carries_no_section_title() {
    let root = seeded("page-one-untitled", ALDRIC_ID);
    for paper in [Paper::Letter, Paper::A4] {
        for (options, all) in [("none", false), ("all", true)] {
            let tag = format!("aldric {options} {paper:?}");
            let printed = PrintRequest { paper, ..request(all, all, all) };
            let document = sheet_document_at_root(&root, &printed).expect("the document builds");
            let titles: Vec<&str> = document.blocks.iter().filter(|block| block.look.name() == "section-title").map(|block| block.heading.as_str()).collect();
            let expected: Vec<&str> = if all { SECTION_TITLES.to_vec() } else { vec!["Abilities and Feats", "Inventory", "Bio"] };
            assert_eq!(titles, expected, "{tag}: a title per section printed");
            let page_one_run = document.blocks.iter().skip(1).position(|block| block.new_page).map_or(document.blocks.len(), |at| at + 1);
            assert!(
                document.blocks[..page_one_run].iter().all(|block| block.look.name() != "section-title"),
                "{tag}: page 1's run holds no section title"
            );
            let pdf = render(&root, &printed);
            let text = pdf_text(&pdf);
            let one = &pages_text(&text, &document.title, page_count(&pdf))[0];
            assert!(one.starts_with("Character Name"), "{tag}: page 1 opens with the Character Name field, untitled: {one}");
            assert!(!one.contains("Abilities and Feats"), "{tag}: page 1 prints no section title: {one}");
        }
    }
}

/// A look's one stated fact about itself (`look_metadata` under `label`): one per block printed in it.
fn look_fact(document: &crate::print::model::SheetDocument, label: &str) -> serde_json::Value {
    let facts = crate::print::typst::look_metadata(document, label).expect("the sheet typesets");
    assert_eq!(facts.len(), 1, "one block states `{label}`: {facts:?}");
    facts[0].clone()
}

/// The PDF's text from `from` up to the next `to` after it.
fn text_between<'a>(text: &'a str, from: &str, to: &str) -> &'a str {
    let start = text.find(from).unwrap_or_else(|| panic!("`{from}` prints: {text}"));
    let rest = &text[start..];
    &rest[..rest[from.len()..].find(to).map_or(rest.len(), |at| at + from.len())]
}

/// E3.1 (`decisions.md §23.4`, section 2 "Abilities and Feats"): the Feats block prints in its own
/// look, two columns: each feat's name, its chosen target in brackets and its rule's whole text
/// (the "Rules and features" words the Feats tab shows); the budget rows as one line, `<pool>
/// <spent> of <granted>` per pool, from the engine's `feats.budget.*` rows; then two blank rows.
/// Aldric (Weapon Focus (longsword)) and Elowen, Letter and A4.
#[test]
fn print_pf_blocks_feats_print_name_target_and_text() {
    for (tag, id) in [("aldric", ALDRIC_ID), ("elowen", ELOWEN_ID)] {
        let root = seeded(&format!("feats-look-{tag}"), id);
        let loaded = load_saved_character_at_root(&root).expect("the seed loads");
        let row_value = |id: &str| loaded.explanations.iter().find(|row| row.id == id).map(|row| row.value).unwrap_or_else(|| panic!("{tag}: {id}"));
        let mut pools: Vec<&str> = Vec::new();
        for row in loaded.explanations.iter().filter(|row| row.id.starts_with("feats.budget.")) {
            let pool = row.id["feats.budget.".len()..].rsplit_once('.').map_or("", |(pool, _)| pool);
            if !pools.contains(&pool) {
                pools.push(pool);
            }
        }
        assert!(!pools.is_empty(), "{tag}: the engine states a feat budget");
        let budget: Vec<String> = pools
            .iter()
            .map(|pool| {
                let field = |name: &str| row_value(&format!("feats.budget.{pool}.{name}"));
                format!("{} {} of {}", pool.replace('_', " "), field("spent"), field("granted"))
            })
            .collect();
        let budget = budget.join(" · ");
        for paper in [Paper::Letter, Paper::A4] {
            let printed = PrintRequest { paper, ..request(false, false, false) };
            let document = sheet_document_at_root(&root, &printed).expect("the document builds");
            let block = document.blocks.iter().find(|block| block.id == "feats").expect("the Feats block");
            assert_eq!(block.look.name(), "feats", "{tag} {paper:?}: the Feats block prints in the feats look");
            let slots = block.rows.iter().filter(|row| row.label == "Feat slots").collect::<Vec<_>>();
            assert_eq!(slots.len(), 1, "{tag} {paper:?}: the budget is one line: {:?}", block.rows);
            assert_eq!(slots[0].values.first().map(String::as_str), Some(budget.as_str()), "{tag} {paper:?}: `<pool> <spent> of <granted>` per pool");
            assert!(slots[0].derivation.as_ref().is_some_and(|rows| rows.len() == pools.len() * 3), "{tag} {paper:?}: Show the math keeps every budget row");
            let fact = look_fact(&document, "feats-look");
            assert_eq!((fact["columns"].as_u64(), fact["blank"].as_u64()), (Some(2), Some(2)), "{tag} {paper:?}: two columns, two blank rows: {fact}");

            let text = pdf_text(&render(&root, &printed));
            let feats = text_between(&text, "Feats", "Class Features");
            assert!(feats.contains(&format!("Feat slots: {budget}")), "{tag} {paper:?}: the budget line prints: {feats}");
            for feat in &loaded.selected_feats {
                let label = super::feats::feat_label(feat);
                let target = loaded
                    .chosen_feat_targets
                    .iter()
                    .find(|chosen| &chosen.feat_id == feat && !chosen.targets.is_empty())
                    .map(|chosen| format!(" ({})", chosen.targets.join(", ")))
                    .unwrap_or_default();
                let name = format!("{label}{target}");
                let row = block.rows.iter().find(|row| row.label == name).unwrap_or_else(|| panic!("{tag}: `{name}` is a row: {:?}", block.rows));
                assert!(feats.contains(&name), "{tag} {paper:?}: `{name}` prints: {feats}");
                let words = loaded.sheet_lines.iter().find(|line| line.kind == "feat" && line.label == label).map(|line| line.prose.clone()).unwrap_or_default();
                for line in words.split('\n').filter(|line| !line.trim().is_empty()) {
                    assert!(row.notes.iter().any(|note| note == line), "{tag}: `{name}` holds its rule's words `{line}`");
                    let flat = line.split_whitespace().collect::<Vec<_>>().join(" ");
                    assert!(feats.contains(&flat), "{tag} {paper:?}: `{name}`'s whole text prints `{flat}`: {feats}");
                }
            }
            if tag == "aldric" {
                assert!(feats.contains("Weapon Focus (longsword)"), "{tag} {paper:?}: the target prints in brackets: {feats}");
            }
        }
    }
}

/// E3.1 (`decisions.md §23.4`, section 2): the Class Features block prints in its own look: per held
/// class, its name and level, its hit die and its skill ranks per level (the class's own "Rules and
/// features" words, held here to the engine's `class_hit_die_for` and the converted record's `Skill
/// ranks per level`), then every feature line
/// with its value and condition, in two columns; the Actions block prints the Actions tab's lines as
/// a table under `Name`, `Kind`, `Value`. Aldric (Fighter), Elowen (Wizard), Letter and A4.
#[test]
fn print_pf_blocks_class_features_print_in_two_columns() {
    use codex::rules_core::pilot_compute::class_chassis_sheet_rules::skill_ranks_per_level_from_package;
    for (tag, id, class_id, name) in [("aldric", ALDRIC_ID, "class:fighter", "Fighter"), ("elowen", ELOWEN_ID, "class:wizard", "Wizard")] {
        let root = seeded(&format!("class-look-{tag}"), id);
        let loaded = load_saved_character_at_root(&root).expect("the seed loads");
        let level = loaded
            .summary
            .class_summary
            .split(',')
            .find_map(|segment| segment.trim().strip_prefix(&format!("{class_id}:")).map(str::to_owned))
            .unwrap_or_else(|| panic!("{tag}: holds {class_id}: {}", loaded.summary.class_summary));
        let die = crate::character_hub::class_hit_die_for(class_id).expect("the engine states the hit die");
        let ranks = skill_ranks_per_level_from_package(&class_id["class:".len()..]).expect("the record states skill ranks").0;
        let header = format!("{name} level {level}");
        let facts = [format!("Hit die: d{die}"), format!("Skill ranks per level: {ranks}")];
        for paper in [Paper::Letter, Paper::A4] {
            let printed = PrintRequest { paper, ..request(false, false, false) };
            let document = sheet_document_at_root(&root, &printed).expect("the document builds");
            let block = document.blocks.iter().find(|block| block.id == "classFeatures").expect("the Class Features block");
            assert_eq!(block.look.name(), "class-features", "{tag} {paper:?}: Class Features prints in its own look");
            let first = block.rows.first().expect("a row");
            assert_eq!((first.label.as_str(), &first.values[..2]), (header.as_str(), &facts[..]), "{tag} {paper:?}: the class line opens the block");
            let fact = look_fact(&document, "class-features-look");
            assert_eq!(fact["columns"].as_u64(), Some(2), "{tag} {paper:?}: the feature lines print in two columns: {fact}");
            let actions = document.blocks.iter().find(|block| block.id == "actions").expect("the Actions block");
            assert_eq!(actions.look.name(), "actions", "{tag} {paper:?}: Actions prints in its own look");

            let text = pdf_text(&render(&root, &printed));
            let features = text_between(&text, "Class Features", "Actions");
            assert!(features.contains(&format!("{header} {} {}", facts[0], facts[1])), "{tag} {paper:?}: the class line prints: {features}");
            assert_eq!(features.matches(&facts[0]).count(), 1, "{tag} {paper:?}: the hit die prints once, on the class line: {features}");
            let table = text_between(&text, "Actions", "Spells");
            assert!(table.starts_with("Actions Name Kind Value"), "{tag} {paper:?}: the Actions table's columns: {table}");
            for applied in &loaded.resolved_racial_traits.applied_traits {
                assert!(table.contains(&format!("{} Racial trait", applied.name)), "{tag} {paper:?}: `{}` prints with its kind: {table}", applied.name);
            }
        }
    }
}

/// The Inventory section's blocks (E3.2, `decisions.md §23.1` item 5, §24.3.7): Defense, Gear, Money.
const INVENTORY_BLOCKS: [&str; 3] = ["defense", "gear", "money"];

/// E3.2 (`decisions.md §23.1` item 5 "Inventory: its own page(s), starts on a new page"): the
/// Inventory section opens a page under its title, then the armor and shield table (`defense`, in
/// its own look, a Weight column and one blank row), then `gear`, then `money`; nothing else is in
/// it. Aldric with no option and with every option, Letter and A4.
#[test]
fn print_pf_blocks_inventory_starts_on_a_new_page() {
    let root = seeded("inventory-page", ALDRIC_ID);
    for paper in [Paper::Letter, Paper::A4] {
        for all in [false, true] {
            let tag = format!("aldric all={all} {paper:?}");
            let printed = PrintRequest { paper, ..request(all, all, all) };
            let document = sheet_document_at_root(&root, &printed).expect("the document builds");
            let title = document.blocks.iter().position(|block| block.id == "inventoryTitle").expect("the Inventory title");
            assert!(document.blocks[title].new_page, "{tag}: the Inventory title starts a page");
            let ids: Vec<&str> = document.blocks[title + 1..].iter().take_while(|block| block.look.name() != "section-title").map(|block| block.id).collect();
            assert_eq!(ids, INVENTORY_BLOCKS.to_vec(), "{tag}: the Inventory section holds Defense, Gear, Money");
            let looks: Vec<&str> = document.blocks[title + 1..title + 4].iter().map(|block| block.look.name()).collect();
            assert_eq!(looks, vec!["defense", "gear", "money"], "{tag}: each prints in its own look");
            let defense = &document.blocks[title + 1];
            assert_eq!(defense.rows[0].values.last().map(String::as_str), Some("Weight"), "{tag}: the armor table has a Weight column: {:?}", defense.rows[0]);
            assert_eq!(look_fact(&document, "defense-look")["blank"].as_u64(), Some(1), "{tag}: one blank row");

            let pdf = render(&root, &printed);
            let text = pdf_text(&pdf);
            let pages = pages_text(&text, &document.title, page_count(&pdf));
            let opening = format!("Inventory {}", defense.heading);
            assert!(
                pages.iter().any(|page| page.starts_with(&opening)),
                "{tag}: a page opens `{opening}`; pages open: {:?}",
                pages.iter().map(|p| p.chars().take(40).collect::<String>()).collect::<Vec<_>>()
            );
        }
    }
}

/// E3.2: the Gear block prints every owned copy under one of three sections, Worn (by slot, its slot
/// beside it), Carried, Left behind, each with its weight (a copy left behind weighs nothing), then
/// the carried weight, the load and the three limits; two blank rows per section. Aldric with a
/// Dagger and then a Ring of Protection +1 bought and worn (the ring's slot comes first), his chain
/// shirt left behind and his longsword carried, Letter and A4.
#[test]
fn print_pf_blocks_gear_prints_worn_carried_and_left_behind_sections() {
    use crate::character_hub::{adjust_character_money_at_root, purchase_equipment_priced_at_root, PriceMode, PurchaseEquipmentResponse};
    use crate::equipment_catalog::{list_market, MarketCategory, MarketFilter};
    let root = seeded("gear-sections", ALDRIC_ID);
    adjust_character_money_at_root(&root, 1_000_000).expect("funds for the Market");
    for (category, name) in [(MarketCategory::Weapon, "Dagger"), (MarketCategory::MagicItem, "Ring of Protection +1")] {
        let shelf = list_market(MarketFilter { category, name_contains: Some(name.to_owned()), book: None }).entries;
        let row = shelf.iter().find(|row| row.entry.name == name && row.entry.cost_gp.is_some()).unwrap_or_else(|| panic!("the Market sells {name}"));
        let response = purchase_equipment_priced_at_root(&root, &row.entry.key, ActiveState::EquippedActive, AT, PriceMode::Standard).expect("the purchase runs");
        assert!(matches!(response, PurchaseEquipmentResponse::Purchased { .. }), "{name} is bought: {response:?}");
    }
    set_equipment_active_state_at_root(&root, "item:chain_shirt", ActiveState::Absent, AT).expect("left behind");
    set_equipment_active_state_at_root(&root, "item:longsword", ActiveState::SelectedInactive, AT).expect("carried");
    let held = list_held_equipment_at_root(&root).expect("held");
    let loaded = load_saved_character_at_root(&root).expect("Aldric loads");
    let encumbrance = &loaded.corpus_derived.encumbrance;
    let section_of = |state: ActiveStateDto| match state {
        ActiveStateDto::EquippedActive => "Worn",
        ActiveStateDto::SelectedInactive => "Carried",
        ActiveStateDto::Absent => "Left behind",
    };
    for state in [ActiveStateDto::EquippedActive, ActiveStateDto::SelectedInactive, ActiveStateDto::Absent] {
        assert!(held.iter().any(|copy| copy.active_state == state), "Aldric holds a {} copy: {held:?}", section_of(state));
    }
    for paper in [Paper::Letter, Paper::A4] {
        let tag = format!("aldric {paper:?}");
        let printed = PrintRequest { paper, ..request(false, false, false) };
        let document = sheet_document_at_root(&root, &printed).expect("the document builds");
        let gear = document.blocks.iter().find(|block| block.id == "gear").expect("the Gear block");
        assert_eq!(gear.look.name(), "gear", "{tag}: Gear prints in its own look");
        // The rows section by section: each section's heading row, then its items.
        let mut sections: Vec<(&str, Vec<&crate::print::model::SheetRow>)> = Vec::new();
        for row in &gear.rows {
            match row.kind {
                "section" => sections.push((row.label.as_str(), Vec::new())),
                "item" => sections.last_mut().expect("an item under a section").1.push(row),
                _ => {}
            }
        }
        let titles: Vec<&str> = sections.iter().map(|(title, _)| *title).collect();
        assert_eq!(titles, vec!["Worn", "Carried", "Left behind"], "{tag}: the three sections, in order");
        for (title, rows) in &sections {
            let owned: Vec<&str> = held.iter().filter(|copy| section_of(copy.active_state) == *title).map(|copy| copy.name.as_str()).collect();
            let mut printed: Vec<&str> = rows.iter().map(|row| row.label.as_str()).collect();
            let mut expected = owned.clone();
            printed.sort_unstable();
            expected.sort_unstable();
            assert_eq!(printed, expected, "{tag}: `{title}` lists every copy in that state");
        }
        // Worn by slot: in the body-slot order, each with its slot's words.
        let worn: Vec<&crate::character_hub::HeldEquipmentCopyDto> = {
            let mut worn: Vec<_> = held.iter().filter(|copy| copy.active_state == ActiveStateDto::EquippedActive).collect();
            worn.sort_by_key(|copy| copy.slot);
            worn
        };
        let printed_worn: Vec<&str> = sections[0].1.iter().map(|row| row.label.as_str()).collect();
        assert_eq!(printed_worn, worn.iter().map(|copy| copy.name.as_str()).collect::<Vec<_>>(), "{tag}: Worn is in slot order");
        assert!(worn.len() >= 2 && worn[0].slot < worn[1].slot, "{tag}: two worn copies in two slots: {worn:?}");
        for left in &sections[2].1 {
            assert_eq!(left.values.last().map(String::as_str), Some("no weight"), "{tag}: a copy left behind weighs nothing: {left:?}");
        }
        // A worn or carried copy prints the weight the engine carries it at; none stated: the no-value mark.
        for carried in sections[0].1.iter().chain(&sections[1].1) {
            let copy = held.iter().find(|copy| copy.name == carried.label).expect("a held copy");
            let stated = encumbrance.per_item.iter().find(|item| item.item_id == copy.item_id);
            let expected = stated.map_or_else(|| crate::print::layout::NO_VALUE.to_owned(), |item| super::pounds(item.weight_lbs));
            assert_eq!(carried.values.last(), Some(&expected), "{tag}: `{}` prints its weight", carried.label);
        }
        assert!(sections[0].1.iter().chain(&sections[1].1).any(|row| row.values.last().is_some_and(|weight| weight.ends_with(" lb"))), "{tag}: a weighed copy prints in pounds");
        let fact = look_fact(&document, "gear-look");
        assert_eq!((fact["sections"].as_u64(), fact["blank"].as_u64()), (Some(3), Some(2)), "{tag}: three sections, two blank rows each: {fact}");

        let text = pdf_text(&render(&root, &printed));
        let block = text_between(&text, "Gear Item", "Money PP");
        let bare: String = block.split_whitespace().collect();
        assert_in_order(&bare, &["Worn", "Carried", "Longsword", "Leftbehind", "ChainShirtnoweight"]);
        for (label, value) in [
            ("Carried weight", super::pounds(encumbrance.total_carried_weight_lbs)),
            ("Load", encumbrance.level.clone()),
            ("Light load up to", super::pounds(encumbrance.light_max_lbs)),
            ("Medium load up to", super::pounds(encumbrance.medium_max_lbs)),
            ("Heavy load up to", super::pounds(encumbrance.heavy_max_lbs)),
        ] {
            let pair: String = format!("{label}{value}").split_whitespace().collect();
            assert!(bare.contains(&pair), "{tag}: `{label} {value}` prints: {block}");
        }
    }
}

/// E3.2: Money is a block of its own: four coin boxes, PP, GP, SP and CP, each holding the Gear
/// tab's value (`load_character_money`), then a write-in ledger of four lines. Aldric, Letter and A4.
#[test]
fn print_pf_blocks_money_prints_four_coin_boxes() {
    let root = seeded("money-boxes", ALDRIC_ID);
    crate::character_hub::adjust_character_money_at_root(&root, 1_234_567).expect("money adjusts");
    let money = crate::character_hub::load_character_money_at_root(&root).expect("money loads");
    let coins = [("PP", money.platinum), ("GP", money.gold), ("SP", money.silver), ("CP", money.copper)];
    for paper in [Paper::Letter, Paper::A4] {
        let tag = format!("aldric {paper:?}");
        let printed = PrintRequest { paper, ..request(false, false, false) };
        let document = sheet_document_at_root(&root, &printed).expect("the document builds");
        let block = document.blocks.iter().find(|block| block.id == "money").expect("the Money block");
        assert_eq!(block.look.name(), "money", "{tag}: Money prints in its own look");
        let rows: Vec<(&str, &str)> = block.rows.iter().map(|row| (row.label.as_str(), row.values[0].as_str())).collect();
        let values: Vec<String> = coins.iter().map(|(_, value)| value.to_string()).collect();
        let expected: Vec<(&str, &str)> = coins.iter().zip(&values).map(|((label, _), value)| (*label, value.as_str())).collect();
        assert_eq!(rows, expected, "{tag}: four coin boxes with the Gear tab's values");
        let fact = look_fact(&document, "money-look");
        assert_eq!((fact["boxes"].as_u64(), fact["ledger"].as_u64()), (Some(4), Some(4)), "{tag}: four boxes, a four-line ledger: {fact}");
        let gear = document.blocks.iter().find(|block| block.id == "gear").expect("the Gear block");
        assert!(gear.rows.iter().all(|row| !["PP", "GP", "SP", "CP"].contains(&row.label.as_str())), "{tag}: the coins left the Gear block");

        let text = pdf_text(&render(&root, &printed));
        let printed_money = &text[text.find("Money PP").unwrap_or_else(|| panic!("{tag}: the Money block opens with PP: {text}"))..];
        // Each coin's label is over its value in its box; the text extraction runs the two together.
        let bare: String = printed_money.split_whitespace().collect();
        let boxes: String = coins.iter().map(|(label, value)| format!("{label}{value}")).collect();
        assert!(bare.starts_with(&format!("Money{boxes}Ledger")), "{tag}: the four boxes in order, then the ledger: {printed_money}");
        assert!(printed_money.contains("Ledger"), "{tag}: the ledger prints: {printed_money}");
    }
}

/// E3.3 (`decisions.md §23.4`, §23.7 Q3: section 2's daily spells list, custom and pets): the Spells
/// block prints in its own look, per class: the class and its caster level, then per spell level a
/// line of the save DC, the spells per day, the known / prepared counts and one **used** write-in box
/// per spell per day; then the spell names by level in two columns. Every number is the engine's
/// `explanations` row (`class_chassis.<class>.caster_level`, `.spell_save_dc.spell_level_<n>`,
/// `class_spell.<class>.total_spells_per_day.spell_level_<n>`). Custom and Pets print as compact
/// tables in looks of their own. Elowen (Wizard 5: Light, Fireball) and Aldric (no spell), Letter and A4.
#[test]
fn print_pf_blocks_spells_print_used_boxes_per_level() {
    let root = seeded("spells-look-elowen", ELOWEN_ID);
    let loaded = load_saved_character_at_root(&root).expect("Elowen loads");
    let value_of = |id: String| loaded.explanations.iter().find(|row| row.id == id).map(|row| row.value.to_string());
    // The levels the engine grounds a per-day count for, from its own rows.
    let mut per_day: Vec<(u8, String)> = loaded
        .explanations
        .iter()
        .filter_map(|row| row.id.strip_prefix("class_spell.wizard.total_spells_per_day.spell_level_").map(|level| (level.parse::<u8>().expect("a level"), row.value.to_string())))
        .collect();
    per_day.sort();
    assert!(per_day.iter().any(|(level, _)| *level == 3), "Elowen has third-level slots: {per_day:?}");
    let boxes: u64 = per_day.iter().map(|(_, count)| count.parse::<u64>().expect("a count")).sum();
    for paper in [Paper::Letter, Paper::A4] {
        let tag = format!("elowen {paper:?}");
        let printed = PrintRequest { paper, ..request(false, false, false) };
        let document = sheet_document_at_root(&root, &printed).expect("the document builds");
        let spells = document.blocks.iter().find(|block| block.id == "spells").expect("the Spells block");
        assert_eq!(spells.look.name(), "spells", "{tag}: Spells prints in its own look");
        let class = spells.rows.iter().find(|row| row.kind == "class").unwrap_or_else(|| panic!("{tag}: a class line: {:?}", spells.rows));
        assert_eq!(class.label, "Wizard", "{tag}");
        assert_eq!(class.values.first().cloned(), value_of("class_chassis.wizard.caster_level".to_owned()), "{tag}: the caster level");
        let levels: Vec<&crate::print::model::SheetRow> = spells.rows.iter().filter(|row| row.kind == "level").collect();
        assert_eq!(levels.len(), per_day.len(), "{tag}: one line per level with slots: {levels:?}");
        for ((level, count), row) in per_day.iter().zip(&levels) {
            assert_eq!(row.label, format!("Level {level}"), "{tag}");
            let dc = value_of(format!("class_chassis.wizard.spell_save_dc.spell_level_{level}")).unwrap_or_else(|| crate::print::layout::NO_VALUE.to_owned());
            assert_eq!(row.values[0], dc, "{tag}: level {level}'s save DC");
            assert_eq!(&row.values[1], count, "{tag}: level {level}'s spells per day");
            let held = match level {
                0 => "1 / 1",
                3 => "1 / 1",
                _ => "0 / 0",
            };
            assert_eq!(row.values[2], held, "{tag}: level {level}'s known / prepared (Light at 0, Fireball at 3)");
        }
        let names: Vec<(&str, &str)> = spells.rows.iter().filter(|row| row.kind == "spell").map(|row| (row.label.as_str(), row.values[0].as_str())).collect();
        assert!(spells.rows.iter().filter(|row| row.kind == "spell").all(|row| row.values.len() == 1 || row.values[1..].iter().all(String::is_empty)), "{tag}: a spell line holds its level only: {:?}", spells.rows);
        assert_eq!(names, vec![("Light", "0"), ("Fireball", "3")], "{tag}: the spell names by level");
        let fact = look_fact(&document, "spells-look");
        assert_eq!((fact["boxes"].as_u64(), fact["columns"].as_u64()), (Some(boxes), Some(2)), "{tag}: one used box per spell per day, names in two columns: {fact}");

        let text = pdf_text(&render(&root, &printed));
        let block = text_between(&text, "Spells Wizard", "Custom");
        let bare: String = block.split_whitespace().collect();
        let caster = class.values[0].clone();
        assert!(bare.contains(&format!("Casterlevel{caster}")), "{tag}: the caster level prints: {block}");
        let third = levels.iter().find(|row| row.label == "Level 3").expect("level 3");
        let line: String = format!("Level3{}{}", third.values[0], third.values[1]);
        assert!(bare.contains(&line), "{tag}: level 3's DC and per day print on its line: {block}");
        assert_in_order(&bare, &["Light", "Fireball"]);

        // Custom and Pets (section 2): compact tables in looks of their own.
        for (id, look) in [("custom", "custom"), ("pets", "pets")] {
            let block = document.blocks.iter().find(|block| block.id == id).unwrap_or_else(|| panic!("{tag}: the {id} block"));
            assert_eq!(block.look.name(), look, "{tag}: {id} prints in its own look");
        }
        assert!(text.contains("No animal companion or mount."), "{tag}: Pets says so when there is none");
    }
    // No seed holds a companion: the `pets` look typesets one built as `blocks::pets` builds it.
    {
        use crate::print::model::{Look, SheetBlock, SheetDocument, SheetRow};
        let rows = vec![
            SheetRow::new("Animal companion", ["Wolf".to_owned()]).of_kind("companion").with_notes(["From Druid".to_owned()]),
            SheetRow::new("Bite", ["+3".to_owned()]).of_kind("stat"),
            SheetRow::new("AC", ["+4".to_owned()]).of_kind("stat"),
            SheetRow::new("Notes", Vec::<String>::new()).of_kind("notes").with_notes(["Trip on a bite.".to_owned()]),
        ];
        let document = SheetDocument { title: "Pets probe".to_owned(), paper: "us-letter", blocks: vec![SheetBlock::new("Pets", rows).in_look(Look::Pets)] };
        let typeset = crate::print::typst::typeset(&document).expect("the companion typesets");
        assert!(typeset.warnings.is_empty(), "{:?}", typeset.warnings);
        let bare: String = pdf_text(&typeset.pdf).split_whitespace().collect();
        assert_in_order(&bare, &["AnimalcompanionWolf", "FromDruid", "Bite+3", "AC+4", "Notes", "Triponabite."]);
    }
    // A character with no spell prints the block's one line and no class.
    let aldric = seeded("spells-look-aldric", ALDRIC_ID);
    let document = sheet_document_at_root(&aldric, &request(false, false, false)).expect("the document builds");
    let spells = document.blocks.iter().find(|block| block.id == "spells").expect("the Spells block");
    assert_eq!(spells.look.name(), "spells", "aldric: Spells prints in its own look");
    assert!(spells.rows.iter().all(|row| row.kind != "class"), "aldric: no class line: {:?}", spells.rows);
    assert!(pdf_text(&render(&aldric, &request(false, false, false))).contains("No spell is known."));
}

/// E3.3 (`decisions.md §22` "2: A": the Spellbook prints spells only; R3: PCGen's 3x5 cards as the
/// picture): one entry per held spell, its name, school, level for the class that holds it, casting
/// time, range, duration, saving throw, spell resistance and its text, the entries two to a line.
/// Each field is the spell rule's own words (its stat-block lines), never re-derived. Elowen's
/// Fireball and Light, Letter and A4.
#[test]
fn print_pf_blocks_spellbook_prints_card_entries() {
    let root = seeded("spellbook-cards-elowen", ELOWEN_ID);
    let loaded = load_saved_character_at_root(&root).expect("Elowen loads");
    for paper in [Paper::Letter, Paper::A4] {
        let tag = format!("elowen {paper:?}");
        let printed = PrintRequest { paper, ..request(true, false, false) };
        let document = sheet_document_at_root(&root, &printed).expect("the document builds");
        let book = document.blocks.iter().find(|block| block.id == "spellbook").expect("the Spellbook block");
        assert_eq!(book.look.name(), "spellbook", "{tag}: the Spellbook prints in its own look");
        let captions = book.rows.iter().find(|row| row.kind == "captions").expect("the field captions");
        let expected_captions = ["School", "Level", "Casting time", "Components", "Range", "Target or area", "Duration", "Saving throw", "Spell resistance"];
        assert_eq!(captions.values, expected_captions.map(str::to_owned).to_vec(), "{tag}: the card's fields");
        let field = |row: &crate::print::model::SheetRow, caption: &str| row.values[expected_captions.iter().position(|c| *c == caption).expect("a caption")].clone();
        let entries: Vec<&crate::print::model::SheetRow> = book.rows.iter().filter(|row| row.kind == "spell").collect();
        assert_eq!(entries.iter().map(|row| row.label.as_str()).collect::<Vec<_>>(), vec!["Light", "Fireball"], "{tag}: one entry per held spell");
        let fireball = entries[1];
        // The school with its descriptor, the way the books print it; the record's later
        // `School: FireSchool` line (its elemental-school tag) is not the spell's school.
        assert_eq!(field(fireball, "School"), "Evocation [Fire]", "{tag}: {fireball:?}");
        assert_eq!(field(fireball, "Level"), "Wizard 3", "{tag}");
        assert_eq!(field(fireball, "Casting time"), "1 standard action", "{tag}");
        let line = loaded.sheet_lines.iter().find(|line| line.kind == "spell" && line.label == "Fireball").expect("Fireball's line");
        let range = line.prose.lines().find_map(|l| l.strip_prefix("Range: ")).expect("the rule states a range");
        assert_eq!(field(fireball, "Range"), range, "{tag}: the range is the rule's own words");
        assert_eq!(field(fireball, "Duration"), "Instantaneous", "{tag}");
        assert_eq!(field(fireball, "Saving throw"), "Reflex half", "{tag}");
        assert_eq!(field(fireball, "Spell resistance"), "Yes", "{tag}");
        assert_eq!(field(entries[0], "Level"), "Wizard 0", "{tag}: Light is a cantrip");
        // The text is the rule's words with the stat-block lines lifted out into the fields.
        let text_lines: Vec<&str> = line.prose.lines().filter(|l| !l.starts_with("Casting time: ") && !l.starts_with("Duration: ")).collect();
        assert!(fireball.notes.iter().any(|note| text_lines.contains(&note.as_str())), "{tag}: the rule's text: {fireball:?}");
        assert!(fireball.notes.iter().all(|note| !note.starts_with("Casting time: ")), "{tag}: no stat-block line left in the text: {fireball:?}");
        let fact = look_fact(&document, "spellbook-look");
        assert_eq!((fact["entries"].as_u64(), fact["columns"].as_u64()), (Some(2), Some(2)), "{tag}: two entries, two to a line: {fact}");

        let text = pdf_text(&render(&root, &printed));
        let spellbook = &text[text.find("Spellbook Spellbook").unwrap_or_else(|| panic!("{tag}: the Spellbook section: {text}"))..];
        let bare: String = spellbook.split_whitespace().collect();
        for pair in ["Fireball", "Level:Wizard3", "Castingtime:1standardaction", "Duration:Instantaneous", "Savingthrow:Reflexhalf", "Spellresistance:Yes"] {
            assert!(bare.contains(pair), "{tag}: the Fireball card prints `{pair}`: {spellbook}");
        }
    }
}
