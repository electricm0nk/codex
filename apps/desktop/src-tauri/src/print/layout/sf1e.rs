//! The Starfinder 1e layout file (operator ruling R2, R15): the boxes of the Starfinder sheet
//! (`starfinderSheetModel.ts`, `StarfinderCharacterSheet.tsx`), in its order -- Class, Ability
//! Scores, Stamina / Hit Points / Resolve, Armor Class, Saving Throws, Attack, Credits and Bulk,
//! Weapons, Skills, each casting class's Spells, Other totals -- then the engine's printed lines
//! (`sf_sheet_print.rs`, carried on the load response's `sheet_lines`) as "Rules and features";
//! then the option blocks. Every number is one `sf.*` explanation row's value, signed where the
//! screen signs it; nothing here computes one (NR-2). Starfinder has no CMB, CMD, touch or
//! flat-footed AC, so none is laid out.

use super::{block, cells_row, did_not_compute, humanise, notes, other_totals, rules_and_features, with_options, Placement, NO_VALUE};
use crate::character_hub::SheetLineDto;
use crate::print::blocks::{line_row, SheetSources};
use crate::print::model::{SheetBlock, SheetDocument, SheetRow};

/// The non-blocking diagnostic every computed Starfinder build carries about the Pathfinder-shaped
/// snapshot fields; the sheet does not show it.
const PATHFINDER_FIELDS_DIAGNOSTIC: &str = "sf_adapter.pathfinder_fields_zero";

const ABILITIES: [(&str, &str); 6] = [
    ("strength", "Strength"),
    ("dexterity", "Dexterity"),
    ("constitution", "Constitution"),
    ("intelligence", "Intelligence"),
    ("wisdom", "Wisdom"),
    ("charisma", "Charisma"),
];

/// One single-value row: its row id, its label, and whether it prints with its sign.
type SingleValue = (&'static str, &'static str, bool);

/// The single-value boxes, in sheet order.
const SINGLE_VALUE_SECTIONS: [(&str, &[SingleValue]); 5] = [
    (
        "Stamina, Hit Points and Resolve",
        &[("sf.stamina", "Stamina Points", false), ("sf.hit_points", "Hit Points", false), ("sf.resolve", "Resolve Points", false)],
    ),
    (
        "Armor Class",
        &[("sf.eac", "Energy Armor Class (EAC)", false), ("sf.kac", "Kinetic Armor Class (KAC)", false), ("sf.initiative", "Initiative", true)],
    ),
    ("Saving Throws", &[("sf.fortitude", "Fortitude", true), ("sf.reflex", "Reflex", true), ("sf.will", "Will", true)]),
    (
        "Attack",
        &[("sf.base_attack_bonus", "Base Attack Bonus", true), ("sf.attack.melee", "Melee attack", true), ("sf.attack.ranged", "Ranged attack", true)],
    ),
    (
        "Credits and Bulk",
        &[
            ("sf.credits.starting", "Starting credits", false),
            ("sf.credits.spent", "Credits spent", false),
            ("sf.credits.remaining", "Credits remaining", false),
            ("sf.bulk", "Bulk carried", false),
            ("sf.bulk_limit.unencumbered_max", "Unencumbered up to (bulk)", false),
            ("sf.bulk_limit.overburdened_above", "Overburdened above (bulk)", false),
        ],
    ),
];

const WEAPON_FIELDS: [(&str, &str); 2] = [("attack", "Attack"), ("damage", "Damage bonus")];
const SPELL_FIELDS: [(&str, &str); 3] = [("per_day", "Per day"), ("known", "Known"), ("save_dc", "Save DC")];

/// The printed label of the held record `<book>:<kind>:<slug>` (no `#` sub-line), if sent.
fn record_label(lines: &[&SheetLineDto], kind: &str, slug: &str) -> Option<String> {
    let suffix = format!(":{kind}:{slug}");
    lines.iter().find(|line| line.kind == kind && !line.id.contains('#') && line.id.ends_with(&suffix)).map(|line| line.label.clone())
}

/// Each distinct value of `pick` over the ids, in first-seen order.
fn distinct<'a>(ids: impl Iterator<Item = &'a str>, pick: impl Fn(&'a str) -> Option<&'a str>) -> Vec<&'a str> {
    let mut out: Vec<&str> = Vec::new();
    for value in ids.filter_map(pick) {
        if !out.contains(&value) {
            out.push(value);
        }
    }
    out
}

fn identity(sources: &SheetSources, lines: &[&SheetLineDto]) -> SheetBlock {
    let race = lines.iter().find(|line| line.kind == "race" && !line.id.contains('#')).map_or_else(|| NO_VALUE.to_owned(), |line| line.label.clone());
    SheetBlock::new(
        "Identity",
        vec![
            SheetRow::new("Name", [sources.loaded.summary.display_label.clone()]),
            SheetRow::new("Game system", ["Starfinder".to_owned()]),
            SheetRow::new("Race", [race]),
        ],
    )
}

/// The Spellbook: each spell line's whole text (the converted spell's words, its numbers filled
/// for this caster), on pages of its own.
fn spellbook(lines: &[&SheetLineDto]) -> SheetBlock {
    let mut rows: Vec<SheetRow> = lines
        .iter()
        .filter(|line| line.kind == "spell")
        .map(|line| SheetRow { derivation: None, ..line_row(line) })
        .collect();
    if rows.is_empty() {
        rows.push(SheetRow::new("No spell is known.", Vec::<String>::new()));
    }
    SheetBlock::new("Spellbook", rows).on_new_page()
}

/// The sheet's blocks, in print order.
pub fn layout(sources: &SheetSources) -> SheetDocument {
    let loaded = sources.loaded;
    let mut placed = Placement::new(loaded, true);
    let lines = placed.lines.clone();
    let ids: Vec<&str> = placed.row_ids().collect();
    let with_prefix = |prefix: &str| ids.iter().copied().filter(|id| id.starts_with(prefix)).collect::<Vec<&str>>();

    let mut sheet: Vec<SheetBlock> = vec![identity(sources, &lines)];
    sheet.extend(did_not_compute(&loaded.diagnostics));
    let mut boxes: Vec<Option<SheetBlock>> = Vec::new();

    let classes = with_prefix("sf.class_level.")
        .into_iter()
        .map(|id| {
            let slug = &id["sf.class_level.".len()..];
            let label = record_label(&lines, "class", slug).unwrap_or_else(|| humanise(slug));
            cells_row(label, vec![placed.cell(id, false)])
        })
        .collect();
    boxes.push(block("Class", &["Level"], classes));

    let abilities = ABILITIES
        .iter()
        .map(|(key, label)| {
            let score = placed.cell(&format!("sf.ability_score.{key}"), false);
            let modifier = placed.cell(&format!("sf.ability_modifier.{key}"), true);
            cells_row(*label, vec![score, modifier])
        })
        .collect();
    boxes.push(block("Ability Scores", &["Score", "Modifier"], abilities));

    for (title, rows) in SINGLE_VALUE_SECTIONS {
        let rows = rows.iter().map(|(id, label, with_sign)| cells_row(*label, vec![placed.cell(id, *with_sign)])).collect();
        boxes.push(block(title, &[], rows));
    }

    // `sf.weapon.<equipment slug>.<field>`: one row per carried weapon.
    let weapons = distinct(with_prefix("sf.weapon.").into_iter(), |id| id.split('.').nth(2))
        .into_iter()
        .map(|slug| {
            let label = record_label(&lines, "equipment", slug).unwrap_or_else(|| humanise(slug));
            let cells = WEAPON_FIELDS.iter().map(|(field, _)| placed.cell(&format!("sf.weapon.{slug}.{field}"), true)).collect();
            cells_row(label, cells)
        })
        .collect();
    boxes.push(block("Weapons", &WEAPON_FIELDS.map(|(_, column)| column), weapons));

    let skills = with_prefix("sf.skill.")
        .into_iter()
        .map(|id| cells_row(humanise(&id["sf.skill.".len()..]), vec![placed.cell(id, true)]))
        .collect();
    boxes.push(block("Skills", &[], skills));

    // `sf.spells.<class>.<level>.<field>`: one box per casting class, one row per spell level.
    let spell_ids = with_prefix("sf.spells.");
    for class in distinct(spell_ids.iter().copied(), |id| id.split('.').nth(2)) {
        let levels = distinct(spell_ids.iter().copied().filter(|id| id.split('.').nth(2) == Some(class)), |id| id.split('.').nth(3));
        let rows = levels
            .into_iter()
            .map(|level| {
                let cells = SPELL_FIELDS.iter().map(|(field, _)| placed.cell(&format!("sf.spells.{class}.{level}.{field}"), false)).collect();
                cells_row(format!("Level {level}"), cells)
            })
            .collect();
        let name = record_label(&lines, "class", class).unwrap_or_else(|| humanise(class));
        boxes.push(block(&format!("{name} Spells"), &SPELL_FIELDS.map(|(_, column)| column), rows));
    }

    boxes.push(other_totals(&mut placed));
    sheet.extend(boxes.into_iter().flatten());
    sheet.extend(notes(&loaded.diagnostics, &[PATHFINDER_FIELDS_DIAGNOSTIC]));
    sheet.extend(rules_and_features(&lines, loaded.sheet_rules_unavailable_reason.as_deref()));

    let blocks = with_options(sheet, || spellbook(&lines), sources);
    SheetDocument { title: loaded.summary.display_label.clone(), paper: sources.request.paper.typst_name(), blocks }
}

#[cfg(test)]
mod print_sf {
    //! `print_sf` (E8.3): every Starfinder seed prints through the real print path -- the envelope
    //! router's load, this layout file, the shared Typst frame -- on A4 and on Letter, and the
    //! sheet holds every engine row and every printed line the Starfinder sheet shows.
    //!
    //! Run (from `apps/desktop/src-tauri`):
    //!
    //! ```text
    //! cargo test --locked -j 8 print_sf -- --test-threads=8
    //! ```

    use std::collections::BTreeSet;
    use std::path::{Path, PathBuf};

    use codex::saved_character::local_store::SavedCharacterStore;
    use codex::saved_character::{SavedCharacterEnvelope, SavedCharacterRevisionKind, CURRENT_SAVED_CHARACTER_SCHEMA_VERSION};

    use crate::character_hub::LoadSavedCharacterResponse;
    use crate::print::model::SheetDocument;
    use crate::print::{render_sheet_pdf_at_root, sheet_document_at_root, CampaignNote, Paper, PrintRequest};
    use crate::rule_system_adapter::load_saved_character_via_envelope;
    use crate::sf_adapter::STARFINDER_RULE_SYSTEM_ID;

    const OPTION_HEADINGS: [&str; 3] = ["Spellbook", "Show the Math", "Bio and Campaign Notes"];

    /// The four SD-37 seeds (`sf_adapter::tests::seeds`), each saved as a Starfinder character in
    /// a fresh directory; returns (seed id, character root).
    fn saved_seeds(tag: &str) -> Vec<(&'static str, PathBuf)> {
        let dir = std::env::temp_dir().join(format!("codex-print-sf-{}-{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        crate::sf_adapter::tests::seeds()
            .into_iter()
            .map(|(seed, _, input)| {
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
                (seed, root)
            })
            .collect()
    }

    fn request(paper: Paper, spellbook: bool, math: bool, bio: bool) -> PrintRequest {
        PrintRequest { paper, spellbook, math, bio, campaigns: Vec::new() }
    }

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

    fn render(root: &Path, request: &PrintRequest) -> Vec<u8> {
        let rendered = render_sheet_pdf_at_root(root, request).unwrap_or_else(|err| panic!("{}: renders: {err}", root.display()));
        assert!(rendered.warnings.is_empty(), "{}: Typst reports no diagnostics: {:?}", root.display(), rendered.warnings);
        rendered.pdf
    }

    fn headings(document: &SheetDocument) -> Vec<&str> {
        document.blocks.iter().map(|block| block.heading.as_str()).collect()
    }

    /// Every label, value and note the document prints, one string.
    fn document_text(document: &SheetDocument) -> String {
        let mut text = String::new();
        for block in &document.blocks {
            text.push_str(&block.heading);
            text.push('\n');
            for row in &block.rows {
                text.push_str(&row.label);
                text.push('\n');
                for value in row.values.iter().chain(row.notes.iter()) {
                    text.push_str(value);
                    text.push('\n');
                }
            }
        }
        text
    }

    /// The engine rows a printed number cites, by id (`Derivation.source` is `<id> = <value>`).
    fn cited_rows(document: &SheetDocument) -> Vec<String> {
        document
            .blocks
            .iter()
            .flat_map(|block| block.rows.iter())
            .flat_map(|row| row.derivation.iter().flatten())
            .map(|step| step.source.split(" = ").next().unwrap_or_default().to_owned())
            .collect()
    }

    fn loaded(root: &Path) -> LoadSavedCharacterResponse {
        load_saved_character_via_envelope(root).expect("the seed loads")
    }

    /// The criterion: each of the four seeds renders on both papers, a real PDF with no Typst
    /// diagnostic, every page the paper's size, the character's name and its first block's
    /// heading in the text.
    #[test]
    fn print_sf_every_seed_renders_on_a4_and_letter() {
        let seeds = saved_seeds("papers");
        assert_eq!(seeds.len(), 4, "the four Starfinder seeds");
        for (seed, root) in &seeds {
            for (paper, expected) in [(Paper::A4, (595.28_f32, 841.89_f32)), (Paper::Letter, (612.0, 792.0))] {
                let pdf = render(root, &request(paper, false, false, false));
                assert!(pdf.starts_with(b"%PDF-") && pdf.len() > 1024, "{seed} {paper:?}: a PDF over 1 KiB");
                // With `PRINT_SHEET_OUT` set (a directory outside the repo), each PDF is kept there.
                if let Some(dir) = std::env::var_os("PRINT_SHEET_OUT") {
                    std::fs::create_dir_all(&dir).expect("makes the sheets-out directory");
                    std::fs::write(PathBuf::from(dir).join(format!("{seed}-{paper:?}.pdf")), &pdf).expect("writes the sheet");
                }
                let text = pdf_text(&pdf);
                assert!(text.contains(&format!("{seed} printed")), "{seed} {paper:?}: the name prints: {text}");
                assert!(text.contains("Starfinder"), "{seed} {paper:?}: the system prints");
                for (width, height) in page_sizes(&pdf) {
                    assert!(
                        (width - expected.0).abs() < 0.5 && (height - expected.1).abs() < 0.5,
                        "{seed} {paper:?}: page {width}x{height}, want {expected:?}"
                    );
                }
            }
        }
    }

    /// Nothing the screen shows is dropped: every `sf.*` row the load sends is cited by exactly
    /// one printed number, every printed line's label is on the sheet, and the headings print in
    /// the layout file's order.
    #[test]
    fn print_sf_prints_every_engine_row_once_and_every_printed_line() {
        for (seed, root) in saved_seeds("complete") {
            let loaded = loaded(&root);
            assert!(!loaded.explanations.is_empty() && !loaded.sheet_lines.is_empty(), "{seed}: the load has rows and lines");
            let document = sheet_document_at_root(&root, &request(Paper::A4, false, false, false)).expect("the sheet lays out");
            let cited = cited_rows(&document);
            let sent: BTreeSet<String> = loaded.explanations.iter().map(|row| row.id.clone()).collect();
            let cited_set: BTreeSet<String> = cited.iter().cloned().collect();
            let missing: Vec<_> = sent.difference(&cited_set).collect();
            assert!(missing.is_empty(), "{seed}: engine rows not printed: {missing:?}");
            assert_eq!(cited.len(), cited_set.len(), "{seed}: each engine row printed once");
            let text = document_text(&document);
            for line in &loaded.sheet_lines {
                assert!(text.contains(&line.label), "{seed}: printed line {} ({}) is on the sheet", line.id, line.label);
            }
            let heads = headings(&document);
            assert_eq!(heads.first().copied(), Some("Identity"), "{seed}: {heads:?}");
            assert!(heads.iter().all(|h| !OPTION_HEADINGS.contains(h)), "{seed}: basic prints no option block: {heads:?}");
            let pdf = pdf_text(&render(&root, &request(Paper::A4, false, false, false)));
            let mut from = 0;
            for heading in heads {
                let at = pdf[from..].find(heading).unwrap_or_else(|| panic!("{seed}: heading {heading:?} after byte {from}"));
                from += at + heading.len();
            }
        }
    }

    /// The printed numbers are the engine's rows, formatted as the screen formats them: the
    /// Soldier's Stamina (24, `sf_adapter`'s SRD figure), its saves with their sign.
    #[test]
    fn print_sf_numbers_are_the_engine_rows() {
        let seeds = saved_seeds("values");
        let (_, root) = seeds.iter().find(|(seed, _)| *seed == "SF-Soldier-3").expect("the Soldier seed");
        let loaded = loaded(root);
        let row = |id: &str| loaded.explanations.iter().find(|row| row.id == id).unwrap_or_else(|| panic!("row {id}")).value;
        assert_eq!(row("sf.stamina"), 24);
        let text = pdf_text(&render(root, &request(Paper::A4, false, false, false)));
        assert!(text.contains("Stamina Points 24"), "{text}");
        let fortitude = row("sf.fortitude");
        let signed = if fortitude >= 0 { format!("+{fortitude}") } else { fortitude.to_string() };
        assert!(text.contains(&format!("Fortitude {signed}")), "Fortitude {signed}: {text}");
        assert!(text.contains(&format!("Hit Points {}", row("sf.hit_points"))), "{text}");
    }

    /// The three option boxes: Spellbook prints each known spell's whole line, Show the math
    /// each printed number's engine row, Bio the bio and the campaigns sent; each starts a page.
    #[test]
    fn print_sf_each_option_adds_its_block() {
        let seeds = saved_seeds("options");
        let (_, root) = seeds.iter().find(|(seed, _)| *seed == "SF-Mystic-5").expect("the Mystic seed");
        let pages = |request: &PrintRequest| page_sizes(&render(root, request)).len();
        let basic = pages(&request(Paper::A4, false, false, false));
        for (index, option) in OPTION_HEADINGS.iter().enumerate() {
            let one = request(Paper::A4, index == 0, index == 1, index == 2);
            let document = sheet_document_at_root(root, &one).expect("lays out");
            assert_eq!(headings(&document).last().copied(), Some(*option), "{option} prints last");
            assert!(pages(&one) > basic, "{option} adds a page");
        }
        let mut full = request(Paper::Letter, true, true, true);
        full.campaigns = vec![CampaignNote { name: "Drift Run".to_owned(), description: "Absalom Station to the Vast.".to_owned() }];
        let document = sheet_document_at_root(root, &full).expect("lays out");
        let heads = headings(&document);
        assert_eq!(&heads[heads.len() - 3..], &OPTION_HEADINGS, "{heads:?}");
        let spellbook = document.blocks.iter().find(|block| block.heading == "Spellbook").expect("Spellbook");
        let loaded = loaded(root);
        let spells: Vec<_> = loaded.sheet_lines.iter().filter(|line| line.kind == "spell").collect();
        assert!(!spells.is_empty(), "the Mystic knows spells");
        assert_eq!(spellbook.rows.len(), spells.len(), "one Spellbook entry per spell line");
        for line in spells {
            let entry = spellbook.rows.iter().find(|row| row.label == line.label).unwrap_or_else(|| panic!("{} in the Spellbook", line.label));
            let first = line.prose.split('\n').find(|words| !words.trim().is_empty()).expect("a spell line has words");
            assert!(entry.notes.iter().any(|note| note == first), "{}: its words print", line.label);
        }
        let text = pdf_text(&render(root, &full));
        assert!(text.contains("Show the Math"), "{text}");
        assert!(text.contains("sf.stamina ="), "the math cites the Stamina row: {text}");
        assert!(text.contains("Drift Run") && text.contains("Absalom Station to the Vast."), "{text}");
        assert!(text.contains("No bio is recorded."), "{text}");
    }
}
