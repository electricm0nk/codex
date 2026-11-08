//! The Cyberpunk RED layout file (operator ruling R2, R15): the boxes of the Cyberpunk RED sheet
//! (`cyberpunkSheetModel.ts`), in the order its layout registry stacks them
//! (`cyberpunkSheetLayout.ts`: the left column -- STATs, Skills, Role Ability, Reputation,
//! Improvement Points, Lifepath -- then the right -- Hit Points / Humanity / MOVE / LUCK, Wound
//! track, Critical Injuries, Armor, Weapons, Range DV, Cyberware, Cyberdeck, Cash / Ammunition /
//! Rent, Other totals), then the printed lines no box places (`cpr_sheet_print.rs`'s rule lines)
//! as "Rules and features"; then the option blocks. Every number is one `cpr.` explanation row's
//! value; dice, names and words are the engine's printed lines (`sheet_lines`), dice literal.
//! Nothing here computes a number (NR-2).

use super::{
    block, cells_row, did_not_compute, group_row, grouped, humanise, joined_row, notes, other_totals, rules_and_features, with_options, Cell,
    Placement,
};
use crate::print::blocks::SheetSources;
use crate::print::model::{SheetBlock, SheetDocument, SheetRow};

/// The ten STATs in book order: the entered value's row id, the label, and the row of the current
/// value where the engine adjusts it (armour on REF, DEX and MOVE; cyberware BODY; EMP after loss).
const STATS: [(&str, &str, Option<&str>); 10] = [
    ("int", "INT", None),
    ("ref", "REF", Some("cpr.armor.ref")),
    ("dex", "DEX", Some("cpr.armor.dex")),
    ("tech", "TECH", None),
    ("cool", "COOL", None),
    ("will", "WILL", None),
    ("luck", "LUCK", None),
    ("move", "MOVE", Some("cpr.armor.move")),
    ("body", "BODY", Some("cpr.cyberware.body")),
    ("emp", "EMP", Some("cpr.derived.emp_current")),
];

const VITALS: [(&str, &[&str]); 6] = [
    ("Hit Points", &["cpr.derived.hit_points"]),
    ("Seriously Wounded", &["cpr.derived.seriously_wounded"]),
    ("Death Save", &["cpr.derived.death_save"]),
    ("Humanity", &["cpr.derived.humanity_current", "cpr.derived.humanity_max"]),
    ("MOVE", &["cpr.derived.move"]),
    ("LUCK", &["cpr.derived.luck_pool"]),
];

const WOUND_PREFIX: &str = "cpr.wound.";
const WOUND_SINGLES: [(&str, &str); 2] = [("cpr.wound.death_save", "Death Save"), ("cpr.wound.base_death_save_penalty", "Base Death Save Penalty")];

const ARMOUR: [(&str, Option<&str>, &str); 4] = [
    ("Head", Some("cpr.armor.sp_head"), "cpr.armor.penalty_head"),
    ("Body", Some("cpr.armor.sp_body"), "cpr.armor.penalty_body"),
    ("Shield (HP)", Some("cpr.armor.shield_hp"), "cpr.armor.shield_penalty"),
    ("Armor Penalty (REF, DEX, MOVE)", None, "cpr.armor.penalty"),
];

const WEAPON_FIELDS: [(&str, &str); 6] =
    [("DMG", "damage"), ("ROF", "rof"), ("Hands", "hands"), ("Concealable", "concealable"), ("Magazine", "magazine"), ("Skill Base", "skill_base")];
const RANGE_BAND_PREFIX: &str = "cpr.range_band.";

const CYBERWARE_TOTALS: [(&str, &str); 5] = [
    ("Humanity Loss", "cpr.cyberware.humanity_loss"),
    ("Maximum Humanity", "cpr.cyberware.humanity_cap"),
    ("Head SP (cyberware)", "cpr.cyberware.sp_head"),
    ("Body SP (cyberware)", "cpr.cyberware.sp_body"),
    ("Initiative bonus", "cpr.cyberware.initiative_bonus"),
];

const NET_ACTIONS: &str = "cpr.netrunning.net_actions";
const DECK_LINE: &str = "cpr.netrunning.deck";
const FACEDOWN_ROLL: &str = "cpr.reputation.facedown_roll";
const SKILL_PREFIX: &str = "cpr.skill.";
const SKILL_LEVEL_PREFIX: &str = "cpr.skill_level.";

/// The number in `id` = `<prefix><n><suffix>`.
fn numbered(id: &str, prefix: &str, suffix: &str) -> Option<u32> {
    id.strip_prefix(prefix)?.strip_suffix(suffix)?.parse().ok()
}

/// The ids matching `<prefix><n><suffix>`, in `n`'s order.
fn numbered_ids<'a>(ids: impl Iterator<Item = &'a str>, prefix: &str, suffix: &str) -> Vec<&'a str> {
    let mut found: Vec<(u32, &str)> = ids.filter_map(|id| numbered(id, prefix, suffix).map(|n| (n, id))).collect();
    found.sort_by_key(|(n, _)| *n);
    found.into_iter().map(|(_, id)| id).collect()
}

/// `Conceal/Reveal Object` reads `conceal_reveal_object` (the engine's `slug`): lower case, every
/// run of other characters one `_`.
fn skill_slug(name: &str) -> String {
    let mut out = String::new();
    for c in name.to_lowercase().chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c);
        } else if !out.ends_with('_') {
            out.push('_');
        }
    }
    out.trim_matches('_').to_owned()
}

/// A cell's text when it holds one: a weapon's or a piece's printed name.
fn name_of(cell: Cell, fallback: String) -> String {
    match cell {
        Cell::Row(text, _) | Cell::Printed(text) => text,
        Cell::Missing => fallback,
    }
}

/// The skill catalog's (name, category, linked STAT), in catalog order; `None` when it cannot be
/// read (the skills then print ungrouped, labelled from their ids, as on the screen).
fn skill_catalog() -> Option<Vec<(String, String, String)>> {
    let text = |value: &serde_json::Value| value.as_str().map_or_else(|| value.to_string(), str::to_owned);
    let catalog = crate::cpr_catalog::list_cyberpunk_catalog("skill".to_owned()).ok()?;
    Some(catalog.records.iter().map(|record| (text(&record["name"]), text(&record["category"]), text(&record["stat"]))).collect())
}

/// The Skills box: one row per `cpr.skill.<skill>[.<specialty>]` row with its level twin, under its
/// category with its linked STAT; a skill the catalog does not hold is left for Other totals.
fn skills(placed: &mut Placement, ids: &[&str]) -> Vec<Option<SheetRow>> {
    let bases: Vec<&str> = ids.iter().copied().filter(|id| id.starts_with(SKILL_PREFIX)).collect();
    let split = |id: &str| -> (String, String, Option<String>) {
        let rest = id[SKILL_PREFIX.len()..].to_owned();
        match rest.split_once('.') {
            Some((skill, specialty)) => (rest.clone(), skill.to_owned(), Some(specialty.to_owned())),
            None => (rest.clone(), rest, None),
        }
    };
    let named = |name: &str, specialty: &Option<String>| match specialty {
        Some(specialty) => format!("{name} ({})", humanise(specialty)),
        None => name.to_owned(),
    };
    let row_of = |placed: &mut Placement, id: &str, label: String| {
        let (rest, _, _) = split(id);
        let level = placed.cell(&format!("{SKILL_LEVEL_PREFIX}{rest}"), false);
        let base = placed.cell(id, false);
        cells_row(label, vec![level, base])
    };
    let Some(catalog) = skill_catalog() else {
        return bases
            .into_iter()
            .map(|id| {
                let (_, skill, specialty) = split(id);
                row_of(placed, id, named(&humanise(&skill), &specialty))
            })
            .collect();
    };
    let mut categories: Vec<&str> = Vec::new();
    for (_, category, _) in &catalog {
        if !categories.contains(&category.as_str()) {
            categories.push(category);
        }
    }
    let groups = categories
        .into_iter()
        .map(|category| {
            let rows = bases
                .iter()
                .filter_map(|id| {
                    let (_, skill, specialty) = split(id);
                    let (name, _, stat) = catalog.iter().find(|(name, record_category, _)| skill_slug(name) == skill && record_category == category)?;
                    Some(row_of(placed, id, format!("{} ({stat})", named(name, &specialty))))
                })
                .collect();
            (category.to_owned(), rows)
        })
        .collect();
    grouped(groups)
}

/// Cyberpunk RED has no spells; the Spellbook option says so on its page.
fn spellbook() -> SheetBlock {
    SheetBlock::new("Spellbook", vec![SheetRow::new("Cyberpunk RED has no spells.", Vec::<String>::new())]).on_new_page()
}

/// The sheet's blocks, in print order.
pub fn layout(sources: &SheetSources) -> SheetDocument {
    let loaded = sources.loaded;
    // A build that did not compute lays out nothing, its printed lines included.
    let computed = !loaded.diagnostics.iter().any(|d| d.claim_blocking);
    let mut placed = Placement::new(loaded, computed);
    let ids: Vec<&str> = placed.row_ids().collect();
    let line_ids: Vec<&str> = placed.lines.iter().map(|line| line.id.as_str()).collect();

    let stats = STATS
        .iter()
        .map(|(id, label, current)| {
            let entered = placed.cell(&format!("cpr.stat.{id}"), false);
            let now = current.map_or(Cell::Missing, |current| placed.cell(current, false));
            cells_row(*label, vec![entered, now])
        })
        .collect();
    let stats = block("STATs", &["STAT", "Current"], stats);
    let skill_rows = skills(&mut placed, &ids);
    let skills = block("Skills", &["Level", "Base"], skill_rows);
    let vitals = VITALS.iter().map(|(label, ids)| joined_row(*label, ids.iter().map(|id| placed.cell(id, false)).collect())).collect();
    let vitals = block("Hit Points, Humanity, MOVE and LUCK", &[], vitals);

    // The Wound States in the engine's order, then the Death Save and its base penalty.
    let mut wounds: Vec<Option<SheetRow>> = ids
        .iter()
        .copied()
        .filter(|id| id.starts_with(WOUND_PREFIX) && !id.ends_with("_dv") && !WOUND_SINGLES.iter().any(|(single, _)| single == id))
        .map(|id| {
            let below = placed.cell(id, false);
            let dv = placed.cell(&format!("{id}_dv"), false);
            cells_row(humanise(&id[WOUND_PREFIX.len()..]), vec![below, dv])
        })
        .collect();
    wounds.extend(WOUND_SINGLES.iter().map(|(id, label)| cells_row(*label, vec![placed.cell(id, false), Cell::Missing])));
    let wound_track = block("Wound Track", &["Below HP", "Stabilize DV"], wounds);

    let armour = ARMOUR
        .iter()
        .map(|(label, sp, penalty)| {
            let sp = sp.map_or(Cell::Missing, |sp| placed.cell(sp, false));
            cells_row(*label, vec![sp, placed.cell(penalty, false)])
        })
        .collect();
    let armour = block("Armor", &["SP", "Penalty"], armour);

    // The weapons in the engine's order (`cpr.weapon.<n>.`), each by its printed name.
    let mut weapons: Vec<u32> = ids
        .iter()
        .chain(line_ids.iter())
        .filter_map(|id| id.strip_prefix("cpr.weapon.")?.split('.').next()?.parse().ok())
        .collect();
    weapons.sort_unstable();
    weapons.dedup();
    let weapon_name = |placed: &mut Placement, n: u32| name_of(placed.figure(&format!("cpr.weapon.{n}.name")), format!("cpr.weapon.{n}"));
    let weapon_rows = weapons
        .iter()
        .map(|n| {
            let name = weapon_name(&mut placed, *n);
            let cells = WEAPON_FIELDS.iter().map(|(_, field)| placed.figure(&format!("cpr.weapon.{n}.{field}"))).collect();
            cells_row(name, cells)
        })
        .collect();
    let weapon_box = block("Weapons", &WEAPON_FIELDS.map(|(heading, _)| heading), weapon_rows);

    // Range DV: each weapon's single-shot row, then its Autofire row, under the range bands.
    let bands: Vec<&str> = ids.iter().copied().filter(|id| id.starts_with(RANGE_BAND_PREFIX)).collect();
    let mut range_rows: Vec<Option<SheetRow>> = Vec::new();
    for n in &weapons {
        for (field, tag) in [("range_dv", ""), ("autofire_dv", " (Autofire)")] {
            let cells: Vec<Cell> = (0..bands.len()).map(|b| placed.figure(&format!("cpr.weapon.{n}.{field}.{b}"))).collect();
            if cells.iter().any(|cell| !matches!(cell, Cell::Missing)) {
                let name = weapon_name(&mut placed, *n);
                range_rows.push(cells_row(format!("{name}{tag}"), cells));
            }
        }
    }
    let range_dv = if range_rows.iter().any(Option::is_some) {
        let heads: Vec<Cell> = bands.iter().map(|band| placed.cell(band, false)).collect();
        let mut rows = vec![cells_row("Range (m/yds up to)", heads)];
        rows.extend(range_rows);
        block("Range DV", &[], rows)
    } else {
        None
    };

    // The Role Ability: each Role's Rank, then the values its Rank gives, in the engine's order.
    let role_of = |id: &str| -> Option<(String, String)> {
        let (role, field) = id.strip_prefix("cpr.role.")?.split_once('.')?;
        let word = |s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_lowercase() || c == '_');
        (word(role) && word(field)).then(|| (role.to_owned(), field.to_owned()))
    };
    let role_ids: Vec<&str> = ids.iter().copied().filter(|id| role_of(id).is_some()).collect();
    let mut roles: Vec<String> = Vec::new();
    for id in &role_ids {
        let (role, _) = role_of(id).unwrap_or_default();
        if !roles.contains(&role) {
            roles.push(role);
        }
    }
    let role_groups = roles
        .iter()
        .map(|role| {
            let rank = format!("cpr.role.{role}.rank");
            let mut order: Vec<&str> = vec![rank.as_str()];
            order.extend(role_ids.iter().copied().filter(|id| role_of(id).is_some_and(|(r, _)| &r == role) && *id != rank));
            let rows = order
                .into_iter()
                .map(|id| {
                    let field = role_of(id).map(|(_, field)| field).unwrap_or_default();
                    cells_row(humanise(&field), vec![placed.cell(id, false)])
                })
                .collect();
            (humanise(role), rows)
        })
        .collect();
    let role_ability = block("Role Ability", &[], grouped(role_groups));

    // Cyberware: each installed piece by its printed name with its Humanity Loss, the totals, then
    // each option-slot container as used / total.
    let pieces: Vec<&str> = line_ids.iter().copied().filter(|id| numbered(id, "cpr.cyberware.piece.", ".name").is_some()).collect();
    let mut cyberware: Vec<Option<SheetRow>> = pieces
        .iter()
        .map(|id| {
            let name = name_of(placed.figure(id), (*id).to_owned());
            cells_row(name, vec![placed.cell(&id.replace(".name", ".humanity_loss"), false)])
        })
        .collect();
    cyberware.extend(CYBERWARE_TOTALS.iter().map(|(label, id)| joined_row(*label, vec![placed.cell(id, false)])));
    let containers: Vec<&str> =
        ids.iter().filter_map(|id| id.strip_prefix("cpr.cyberware.slots.")?.strip_suffix(".used")).collect();
    for container in containers {
        let label = match container {
            "fashionware" => "Fashionware".to_owned(),
            "internal" => "Internal Body".to_owned(),
            "external" => "External Body".to_owned(),
            "neural_link" => "Neural Link".to_owned(),
            "cyberaudio_suite" => "Cyberaudio Suite".to_owned(),
            other => humanise(other),
        };
        let used = placed.cell(&format!("cpr.cyberware.slots.{container}.used"), false);
        let capacity = placed.cell(&format!("cpr.cyberware.slots.{container}.capacity"), false);
        cyberware.push(joined_row(format!("{label} slots"), vec![used, capacity]));
    }
    let cyberware = block("Cyberware", &[], cyberware);

    // A Netrunner's deck: NET Actions, slots used / available, the deck and what it holds.
    let mut deck: Vec<Option<SheetRow>> = vec![
        joined_row("NET Actions", vec![placed.cell(NET_ACTIONS, false)]),
        joined_row(
            "Deck slots (used / available)",
            vec![placed.cell("cpr.netrunning.deck_slots_used", false), placed.cell("cpr.netrunning.deck_slots_available", false)],
        ),
        joined_row("Cyberdeck", vec![if placed.line(DECK_LINE).is_some() { placed.figure(DECK_LINE) } else { Cell::Missing }]),
    ];
    for id in line_ids.iter().copied() {
        let label = match id.strip_prefix("cpr.netrunning.").and_then(|rest| rest.split_once('.')) {
            Some(("program", n)) if n.parse::<u32>().is_ok() => "Program",
            Some(("hardware", n)) if n.parse::<u32>().is_ok() => "Hardware",
            _ => continue,
        };
        deck.push(joined_row(label, vec![placed.figure(id)]));
    }
    let netrunning = block("Cyberdeck and NET Actions", &[], deck);

    // Reputation: the Level, the Facedown and its roll (the die literal), every recorded event.
    let mut reputation: Vec<Option<SheetRow>> = vec![
        joined_row("Reputation Level", vec![placed.cell("cpr.reputation.level", false)]),
        joined_row("Facedown (COOL + Reputation)", vec![placed.cell("cpr.reputation.facedown", false)]),
        joined_row("Facedown roll", vec![if placed.line(FACEDOWN_ROLL).is_some() { placed.figure(FACEDOWN_ROLL) } else { Cell::Missing }]),
    ];
    for id in numbered_ids(line_ids.iter().copied(), "cpr.reputation.event.", "") {
        let label = placed.line(id).map(|line| line.label.clone()).unwrap_or_default();
        reputation.push(joined_row(label, vec![placed.figure(id)]));
    }
    let reputation = block("Reputation", &[], reputation);
    let improvement = block(
        "Improvement Points",
        &[],
        vec![joined_row("Improvement Points (current / total)", vec![placed.cell("cpr.ip.current", false), placed.cell("cpr.ip.total", false)])],
    );
    let lifepath_rows = numbered_ids(line_ids.iter().copied(), "cpr.lifepath.", "")
        .into_iter()
        .map(|id| {
            let label = placed.line(id).map(|line| line.label.clone()).unwrap_or_default();
            joined_row(label, vec![placed.figure(id)])
        })
        .collect();
    let lifepath = block("Lifepath", &[], lifepath_rows);

    // The Critical Injuries the character has, under their table, each with the engine's words.
    let critical_groups = [("body", "Critical Injuries to the Body"), ("head", "Critical Injuries to the Head")]
        .into_iter()
        .map(|(location, table)| {
            let prefix = format!("cpr.critical.{location}.");
            let rows = numbered_ids(line_ids.iter().copied(), &prefix, "")
                .into_iter()
                .map(|id| {
                    let line = placed.line(id);
                    let words: Vec<String> = line.map(|line| line.prose.split('\n').map(str::to_owned).collect()).unwrap_or_default();
                    let name = name_of(placed.figure(id), id.to_owned());
                    Some(SheetRow::new(name, Vec::<String>::new()).with_notes(words))
                })
                .collect();
            (table.to_owned(), rows)
        })
        .collect();
    let criticals = block("Critical Injuries", &[], grouped(critical_groups));

    // The p121 sheet's Cash, each ammunition stack under its name, and the housing's rent.
    let mut ledger: Vec<Option<SheetRow>> = vec![joined_row("Cash", vec![placed.cell("cpr.cash", false)])];
    let ammo: Vec<Option<SheetRow>> = numbered_ids(ids.iter().copied(), "cpr.ammo.", "")
        .into_iter()
        .map(|id| {
            let name = name_of(placed.figure(&format!("{id}.name")), "Ammunition".to_owned());
            joined_row(name, vec![placed.cell(id, false)])
        })
        .collect();
    if ammo.iter().any(Option::is_some) {
        ledger.push(Some(group_row("Ammunition")));
        ledger.extend(ammo);
    }
    ledger.push(joined_row("Rent", vec![placed.figure("cpr.housing.rent")]));
    let ledger = block("Cash, Ammunition and Rent", &[], ledger);

    let other = other_totals(&mut placed);
    let rules = placed.unplaced_lines();

    let mut sheet: Vec<SheetBlock> = vec![SheetBlock::new(
        "Identity",
        vec![SheetRow::new("Name", [loaded.summary.display_label.clone()]), SheetRow::new("Game system", ["Cyberpunk RED".to_owned()])],
    )];
    sheet.extend(did_not_compute(&loaded.diagnostics));
    sheet.extend(
        [
            stats, skills, role_ability, reputation, improvement, lifepath, vitals, wound_track, criticals, armour, weapon_box, range_dv, cyberware,
            netrunning, ledger, other,
        ]
        .into_iter()
        .flatten(),
    );
    sheet.extend(notes(&loaded.diagnostics, &[]));
    sheet.extend(rules_and_features(&rules, None));

    let blocks = with_options(sheet, spellbook, sources);
    SheetDocument { title: loaded.summary.display_label.clone(), paper: sources.request.paper.typst_name(), blocks }
}

#[cfg(test)]
mod print_cpr {
    //! `print_cpr` (E8.3): every Cyberpunk RED fixture character prints through the real print
    //! path -- the envelope router's load, this layout file, the shared Typst frame -- on A4 and
    //! on Letter, and the sheet holds every engine row and every printed line the sheet shows.
    //!
    //! Run (from `apps/desktop/src-tauri`):
    //!
    //! ```text
    //! cargo test --locked -j 8 print_cpr -- --test-threads=8
    //! ```

    use std::collections::BTreeSet;
    use std::path::{Path, PathBuf};

    use codex::rules_core::cpr::store::CprCharacterStore;

    use crate::character_hub::LoadSavedCharacterResponse;
    use crate::print::model::SheetDocument;
    use crate::print::{render_sheet_pdf_at_root, sheet_document_at_root, Paper, PrintRequest};
    use crate::rule_system_adapter::load_saved_character_via_envelope;

    const OPTION_HEADINGS: [&str; 3] = ["Spellbook", "Show the Math", "Bio and Campaign Notes"];

    /// The five sheet fixture characters (`cpr_adapter::tests::sheet_fixture_characters`), each
    /// saved with its choices and Lifepath in a fresh directory; returns (id, character root).
    fn saved_fixtures(tag: &str) -> Vec<(String, PathBuf)> {
        let dir = std::env::temp_dir().join(format!("codex-print-cpr-{}-{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        crate::cpr_adapter::tests::sheet_fixture_characters()
            .into_iter()
            .map(|(character, choices)| {
                let id = character.character_id.clone();
                let root = dir.join(&id);
                CprCharacterStore::save(&character, &root).expect("the character saves");
                CprCharacterStore::save_choices(&choices, &root).expect("its choices save");
                if let Some(lifepath) = crate::cpr_adapter::tests::sheet_fixture_lifepath(&id) {
                    CprCharacterStore::save_lifepath(&lifepath, &root).expect("its Lifepath saves");
                }
                (id, root)
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
        load_saved_character_via_envelope(root).expect("the fixture loads")
    }

    /// The criterion: each of the five fixture characters renders on both papers, a real PDF with
    /// no Typst diagnostic, every page the paper's size, the character's name in the text.
    #[test]
    fn print_cpr_every_fixture_renders_on_a4_and_letter() {
        let fixtures = saved_fixtures("papers");
        assert_eq!(fixtures.len(), 5, "the five Cyberpunk RED sheet fixtures");
        for (id, root) in &fixtures {
            let name = loaded(root).summary.display_label;
            for (paper, expected) in [(Paper::A4, (595.28_f32, 841.89_f32)), (Paper::Letter, (612.0, 792.0))] {
                let pdf = render(root, &request(paper, false, false, false));
                assert!(pdf.starts_with(b"%PDF-") && pdf.len() > 1024, "{id} {paper:?}: a PDF over 1 KiB");
                // With `PRINT_SHEET_OUT` set (a directory outside the repo), each PDF is kept there.
                if let Some(dir) = std::env::var_os("PRINT_SHEET_OUT") {
                    std::fs::create_dir_all(&dir).expect("makes the sheets-out directory");
                    std::fs::write(PathBuf::from(dir).join(format!("{id}-{paper:?}.pdf")), &pdf).expect("writes the sheet");
                }
                let text = pdf_text(&pdf);
                assert!(text.contains(&name), "{id} {paper:?}: the name {name:?} prints: {text}");
                assert!(text.contains("Cyberpunk RED"), "{id} {paper:?}: the system prints");
                for (width, height) in page_sizes(&pdf) {
                    assert!(
                        (width - expected.0).abs() < 0.5 && (height - expected.1).abs() < 0.5,
                        "{id} {paper:?}: page {width}x{height}, want {expected:?}"
                    );
                }
            }
        }
    }

    /// Nothing the screen shows is dropped: every `cpr.` row the load sends is cited by exactly
    /// one printed number, every printed line's text is on the sheet, and the headings print in
    /// the layout file's order.
    #[test]
    fn print_cpr_prints_every_engine_row_once_and_every_printed_line() {
        for (id, root) in saved_fixtures("complete") {
            let loaded = loaded(&root);
            assert!(!loaded.explanations.is_empty() && !loaded.sheet_lines.is_empty(), "{id}: the load has rows and lines");
            let document = sheet_document_at_root(&root, &request(Paper::A4, false, false, false)).expect("the sheet lays out");
            let cited = cited_rows(&document);
            let sent: BTreeSet<String> = loaded.explanations.iter().map(|row| row.id.clone()).collect();
            let cited_set: BTreeSet<String> = cited.iter().cloned().collect();
            let missing: Vec<_> = sent.difference(&cited_set).collect();
            assert!(missing.is_empty(), "{id}: engine rows not printed: {missing:?}");
            assert_eq!(cited.len(), cited_set.len(), "{id}: each engine row printed once");
            let text = document_text(&document);
            for line in &loaded.sheet_lines {
                let first_words = line.prose.split('\n').find(|words| !words.trim().is_empty()).unwrap_or_default();
                let shown = if line.value.is_empty() { first_words } else { line.value.as_str() };
                assert!(text.contains(shown), "{id}: printed line {} ({shown:?}) is on the sheet", line.id);
            }
            let heads = headings(&document);
            assert_eq!(heads.first().copied(), Some("Identity"), "{id}: {heads:?}");
            assert!(heads.iter().all(|h| !OPTION_HEADINGS.contains(h)), "{id}: basic prints no option block: {heads:?}");
            let pdf = pdf_text(&render(&root, &request(Paper::A4, false, false, false)));
            let mut from = 0;
            for heading in heads {
                let at = pdf[from..].find(heading).unwrap_or_else(|| panic!("{id}: heading {heading:?} after byte {from}"));
                from += at + heading.len();
            }
        }
    }

    /// The printed figures are the engine's: the kitted Solo's Role Ability (Combat Awareness)
    /// with its Rank row, its Hit Points row, and its weapons by their printed names.
    #[test]
    fn print_cpr_figures_are_the_engine_rows_and_lines() {
        let fixtures = saved_fixtures("values");
        let (_, root) = fixtures.iter().find(|(id, _)| id == "cpr-solo-kitted").expect("the kitted Solo");
        let loaded = loaded(root);
        let row = |id: &str| loaded.explanations.iter().find(|row| row.id == id).unwrap_or_else(|| panic!("row {id}")).value;
        let text = pdf_text(&render(root, &request(Paper::A4, false, false, false)));
        assert!(text.contains(&format!("Hit Points {}", row("cpr.derived.hit_points"))), "{text}");
        assert!(text.contains(&format!("Rank {}", row("cpr.role.solo.rank"))), "{text}");
        assert!(text.contains("Combat Awareness"), "{text}");
        let weapon_names: Vec<&str> = loaded
            .sheet_lines
            .iter()
            .filter(|line| line.id.starts_with("cpr.weapon.") && line.id.ends_with(".name"))
            .map(|line| line.value.as_str())
            .collect();
        assert!(!weapon_names.is_empty(), "the kitted Solo carries a weapon");
        for name in weapon_names {
            assert!(text.contains(name), "weapon {name}: {text}");
        }
    }

    /// The three option boxes on a Cyberpunk RED sheet: each adds its block, last, on a page of
    /// its own; Show the math cites the engine rows.
    #[test]
    fn print_cpr_each_option_adds_its_block() {
        let fixtures = saved_fixtures("options");
        let (_, root) = fixtures.iter().find(|(id, _)| id == "cpr-netrunner-decked").expect("the Netrunner");
        let pages = |request: &PrintRequest| page_sizes(&render(root, request)).len();
        let basic = pages(&request(Paper::A4, false, false, false));
        for (index, option) in OPTION_HEADINGS.iter().enumerate() {
            let one = request(Paper::A4, index == 0, index == 1, index == 2);
            let document = sheet_document_at_root(root, &one).expect("lays out");
            assert_eq!(headings(&document).last().copied(), Some(*option), "{option} prints last");
            assert!(pages(&one) > basic, "{option} adds a page");
        }
        let full = request(Paper::Letter, true, true, true);
        let document = sheet_document_at_root(root, &full).expect("lays out");
        let heads = headings(&document);
        assert_eq!(&heads[heads.len() - 3..], &OPTION_HEADINGS, "{heads:?}");
        let text = pdf_text(&render(root, &full));
        assert!(text.contains("cpr.derived.hit_points ="), "the math cites the Hit Points row: {text}");
        assert!(text.contains("No campaign lists this character."), "{text}");
    }
}
