//! Codegen for `rules_tables::equipment_gap_tables` — the corpus equipment
//! and equipment-modifier records that live in an **already-compiled** book
//! whose hand-authored per-book table does not hold them.
//!
//! # Why this binary exists
//!
//! `docs/work-inventory.json` classifies an `equipment`/`equipment_modifier`
//! unit as `not-ingested` when the book has a compiled rule set but
//! `equipment_resolver::equipment_catalog_rows()` holds no row matching the
//! record's `KEY:` (or, failing that, its display name). Those are real gaps
//! inside started books — not un-started books — and closing them needs no
//! new `RuleSetId`, no new corpus cache, and no new player surface: the
//! existing equipment catalog already renders every row the resolver chains.
//!
//! # What it does, and what it deliberately does not do
//!
//! It re-parses each named `.lst` with the **same record predicate**
//! `v06_work_inventory`'s own enumerator applies (skip comment rows, skip
//! ALL-CAPS directive rows, skip `CATEGORY=Internal|`/`CATEGORY:Internal`
//! bookkeeping rows, skip `.MOD` overlays, take a `.COPY=` row's variant name,
//! identity is `KEY:` when present else the display name) and emits only those
//! records the hand-authored tables do not already hold. It never invents a
//! value: `cost_gp`/`weight_lbs` are `None` whenever the corpus token is
//! absent or carries a PCGen formula this table does not evaluate, exactly as
//! every per-book table in `rules_tables/` already documents.
//!
//! Every generated table is screened through
//! `pi_table_sweep::screen_generated_table` **before** it is written, per the
//! provenance gate this bundle landed ahead of the content lanes; a hit is a
//! hard stop, never a filtered-out row.
//!
//! Run it with a local PCGen corpus checkout:
//!
//! ```text
//! PCGEN_CORPUS_ROOT=~/workspace/repos/pcgen/data \
//!   cargo run --locked --bin gen_equipment_gap_tables
//! ```

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use codex::rules_core::equipment_resolver::{hand_authored_equipment_rows, EQUIPMENT_BOOK_ACG, EQUIPMENT_BOOK_APG, EQUIPMENT_BOOK_ARG, EQUIPMENT_BOOK_CRB, EQUIPMENT_BOOK_UC, EQUIPMENT_BOOK_UE, EQUIPMENT_BOOK_UI, EQUIPMENT_BOOK_UPSI, EQUIPMENT_BOOK_UW};
use codex::rules_core::pi_table_sweep::screen_generated_table;

/// Where the generated table lands, relative to the crate root.
const OUTPUT_RELATIVE_PATH: &str = "src/rules_core/rules_tables/equipment_gap_tables.rs";

/// One book's gap-lane inputs: the `EQUIPMENT_BOOK_*` code the resolver files
/// its rows under, the `docs/work-inventory.json` book slug the classifier
/// keys on, and each `.lst` path relative to the corpus root.
struct BookInput {
    code: &'static str,
    slug: &'static str,
    files: &'static [&'static str],
}

/// Every book that carries at least one `not-ingested` equipment or
/// equipment-modifier unit, with the exact files those units come from.
/// Derived from `docs/work-inventory.json`'s own `source_file` field over the
/// `status == "not-ingested"` population — not guessed from a directory glob,
/// so a file with no gap is not re-parsed and cannot introduce a row nobody
/// asked for.
const BOOK_INPUTS: &[BookInput] = &[
    BookInput {
        code: EQUIPMENT_BOOK_CRB,
        slug: "core_rulebook",
        files: &[
            "pathfinder/paizo/roleplaying_game/core_rulebook/cr_equipmods.lst",
            // `core_essentials` is the shared library `core_rulebook.pcc`
            // includes unconditionally; its three equipment records are
            // reported `shared_library_record_held_by_no_ingested_host`
            // precisely because no ingested host's table holds them. CRB is
            // that host.
            "pathfinder/paizo/roleplaying_game/core_essentials/ce_equip_arms_armor.lst",
            "pathfinder/paizo/roleplaying_game/core_essentials/ce_equip_general.lst",
        ],
    },
    BookInput {
        code: EQUIPMENT_BOOK_APG,
        slug: "advanced_players_guide",
        files: &["pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_equipmods.lst"],
    },
    BookInput {
        code: EQUIPMENT_BOOK_ACG,
        slug: "advanced_class_guide",
        files: &[
            "pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_equipmods.lst",
            "pathfinder/paizo/roleplaying_game/advanced_class_guide/_pfs/pfs_acg_equip.lst",
        ],
    },
    BookInput {
        code: EQUIPMENT_BOOK_ARG,
        slug: "advanced_race_guide",
        files: &[
            "pathfinder/paizo/roleplaying_game/advanced_race_guide/arg_equipmods.lst",
            "pathfinder/paizo/roleplaying_game/advanced_race_guide/arg_equip_arms_armor.lst",
        ],
    },
    BookInput {
        code: EQUIPMENT_BOOK_UC,
        slug: "ultimate_combat",
        files: &["pathfinder/paizo/roleplaying_game/ultimate_combat/uc_equipmods.lst"],
    },
    BookInput {
        code: EQUIPMENT_BOOK_UI,
        slug: "ultimate_intrigue",
        files: &["pathfinder/paizo/roleplaying_game/ultimate_intrigue/ui_equipmods.lst"],
    },
    BookInput {
        code: EQUIPMENT_BOOK_UE,
        slug: "ultimate_equipment",
        files: &[
            "pathfinder/paizo/roleplaying_game/ultimate_equipment/ue_equip_general.lst",
            "pathfinder/paizo/roleplaying_game/ultimate_equipment/ue_equip_arms_armor.lst",
            "pathfinder/paizo/roleplaying_game/ultimate_equipment/ue_equip_magic_items.lst",
            "pathfinder/paizo/roleplaying_game/ultimate_equipment/ue_equipmods.lst",
        ],
    },
    BookInput {
        code: EQUIPMENT_BOOK_UPSI,
        slug: "ultimate_psionics",
        files: &["pathfinder/dreamscarred_press/ultimate_psionics/up_equipmods.lst"],
    },
    BookInput {
        code: EQUIPMENT_BOOK_UW,
        slug: "ultimate_wilderness",
        files: &[
            "pathfinder/paizo/roleplaying_game/ultimate_wilderness/uw_equip_general.lst",
            "pathfinder/paizo/roleplaying_game/ultimate_wilderness/uw_equip_arms_armor.lst",
            "pathfinder/paizo/roleplaying_game/ultimate_wilderness/uw_equip_magic_items.lst",
            "pathfinder/paizo/roleplaying_game/ultimate_wilderness/uw_equipmods.lst",
        ],
    },
];

/// One parsed corpus record, before the already-held filter runs.
struct ParsedRecord {
    key: String,
    name: String,
    category: &'static str,
    cost_gp: Option<f64>,
    weight_lbs: Option<f64>,
    description: Option<String>,
}

/// The catalog category a `.lst` basename declares. `_equipmods` is tested
/// before `_equip` for the same reason `file_kind` tests it first: every
/// equipmods basename also contains `_equip`.
fn category_for(basename: &str) -> &'static str {
    if basename.contains("_equipmods") {
        "Equipmods"
    } else if basename.contains("_magic_items") || basename.contains("_equip_magic") {
        "MagicItems"
    } else if basename.contains("_arms_armor") || basename.contains("_arm_armor") {
        "ArmsArmor"
    } else {
        "General"
    }
}

fn tab_fields(line: &str) -> Vec<&str> {
    line.split('\t').collect()
}

fn token_value<'a>(fields: &[&'a str], token: &str) -> Option<&'a str> {
    fields.iter().find_map(|f| f.trim().strip_prefix(token))
}

/// A PCGen numeric token, or `None` when the token is absent or carries a
/// formula (`WT*375`, `1+2`, …) this table deliberately does not evaluate.
fn numeric(fields: &[&str], token: &str) -> Option<f64> {
    token_value(fields, token).and_then(|v| v.trim().parse::<f64>().ok())
}

/// Parse one `.lst` under exactly `v06_work_inventory::enumerate_file`'s
/// record predicate. Kept as a standalone function so its agreement with
/// that enumerator is testable rather than asserted.
fn parse_lst(text: &str, category: &'static str) -> Vec<ParsedRecord> {
    let mut out = Vec::new();
    for line in text.lines() {
        let fields = tab_fields(line);
        let Some(first) = fields.first() else { continue };
        let first = first.trim();
        if first.is_empty() || first.starts_with('#') {
            continue;
        }
        let is_directive = first
            .split_once(':')
            .map(|(head, _)| {
                !head.is_empty()
                    && head.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
            })
            .unwrap_or(false);
        if is_directive && !first.starts_with("CLASS:") {
            continue;
        }
        if first.starts_with("CATEGORY=Internal|")
            || fields.iter().any(|f| f.trim() == "CATEGORY:Internal")
        {
            continue;
        }
        if first.contains(".MOD") {
            continue;
        }
        let name = if let Some((_, variant)) = first.split_once(".COPY=") {
            variant.to_string()
        } else if let Some(rest) =
            first.strip_prefix("CATEGORY=").and_then(|r| r.split_once('|')).map(|(_, r)| r)
        {
            rest.to_string()
        } else {
            first.to_string()
        };
        let key = token_value(&fields, "KEY:").map(|k| k.to_string()).unwrap_or_else(|| name.clone());

        // `DESC:` is the record's own prose; `SPROP:` is its special-property
        // line. A record may carry either, both, or neither — joined when
        // both are present, exactly as `ultimate_equipment::equipment_tables`
        // documents for the same corpus shape. Never a fabricated placeholder.
        let desc = token_value(&fields, "DESC:").map(str::trim).filter(|d| !d.is_empty());
        let sprop = token_value(&fields, "SPROP:").map(str::trim).filter(|d| !d.is_empty());
        let description = match (desc, sprop) {
            (Some(d), Some(s)) if d != s => Some(format!("{d} {s}")),
            (Some(d), _) => Some(d.to_string()),
            (None, Some(s)) => Some(s.to_string()),
            (None, None) => None,
        };

        out.push(ParsedRecord {
            key,
            name,
            category,
            cost_gp: numeric(&fields, "COST:"),
            weight_lbs: numeric(&fields, "WT:"),
            description,
        });
    }
    out
}

fn rust_string(value: &str) -> String {
    let mut out = String::from("\"");
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

fn rust_f64(value: Option<f64>) -> String {
    match value {
        Some(v) => format!("Some({v:?})"),
        None => "None".to_string(),
    }
}

fn corpus_root() -> PathBuf {
    let raw = std::env::var("PCGEN_CORPUS_ROOT")
        .unwrap_or_else(|_| "/home/ubuntu/workspace/repos/pcgen/data".to_string());
    PathBuf::from(shellexpand_home(&raw))
}

fn shellexpand_home(raw: &str) -> String {
    match raw.strip_prefix("~/") {
        Some(rest) => match std::env::var("HOME") {
            Ok(home) => format!("{home}/{rest}"),
            Err(_) => raw.to_string(),
        },
        None => raw.to_string(),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = corpus_root();
    if !root.is_dir() {
        return Err(format!(
            "PCGEN_CORPUS_ROOT does not point at a directory: {}",
            root.display()
        )
        .into());
    }

    // What the hand-authored tables already hold, per book code — **row keys
    // only**, then tested against a corpus record's key OR its display name.
    // That asymmetry is not an oversight: it is exactly what
    // `v06_work_inventory`'s `equipment_keys` map does (it inserts
    // `row.key` and nothing else, then asks
    // `s.contains(unit.key) || s.contains(unit.name)`), so this generator's
    // output set is precisely that classifier's `not-ingested` set. A first
    // pass here inserted row NAMES into the set as well and emitted 741 rows
    // against the inventory's 769 — 28 records suppressed because some other
    // record's display name happened to equal theirs. Caught by differencing
    // the two counts, not by a test.
    let mut held: std::collections::BTreeMap<&'static str, BTreeSet<String>> = Default::default();
    for row in hand_authored_equipment_rows() {
        held.entry(row.book).or_default().insert(row.key.to_string());
    }

    let mut body = String::new();
    let mut totals: Vec<(&str, usize)> = Vec::new();

    for input in BOOK_INPUTS {
        let mut rows: Vec<ParsedRecord> = Vec::new();
        // Deduped on the record IDENTITY (`key`) alone, not on `(key, name)`:
        // once a key is in the book's catalog set, every unit carrying it is
        // classified ingested regardless of display name, so a second row
        // would add nothing but a duplicate catalog entry.
        let mut seen: BTreeSet<String> = BTreeSet::new();
        for rel in input.files {
            let path = root.join(rel);
            let text = std::fs::read_to_string(&path)
                .map_err(|e| format!("{}: {e}", path.display()))?;
            let basename = Path::new(rel).file_name().unwrap().to_string_lossy().into_owned();
            for record in parse_lst(&text, category_for(&basename)) {
                let already = held
                    .get(input.code)
                    .map(|s| s.contains(&record.key) || s.contains(&record.name))
                    .unwrap_or(false);
                if already {
                    continue;
                }
                // A key repeated across two of a book's own files is one
                // record for the catalog's purposes; the first wins, matching
                // `equipment_catalog_row_by_key`'s own first-match rule.
                if !seen.insert(record.key.clone()) {
                    continue;
                }
                rows.push(record);
            }
        }

        totals.push((input.slug, rows.len()));
        writeln!(
            body,
            "\n/// {} — {} record(s) the hand-authored `{}` table does not hold.\npub static {}_GAP_ROWS: &[EquipmentGapRow] = &[",
            input.slug,
            rows.len(),
            input.slug,
            input.slug.to_uppercase()
        )?;
        for row in &rows {
            writeln!(
                body,
                "    EquipmentGapRow {{ book: {}, key: {}, name: {}, category: {}, cost_gp: {}, weight_lbs: {}, description: {} }},",
                rust_string(input.code),
                rust_string(&row.key),
                rust_string(&row.name),
                rust_string(row.category),
                rust_f64(row.cost_gp),
                rust_f64(row.weight_lbs),
                match &row.description {
                    Some(d) => format!("Some({})", rust_string(d)),
                    None => "None".to_string(),
                }
            )?;
        }
        writeln!(body, "];")?;
    }

    let total: usize = totals.iter().map(|(_, n)| *n).sum();
    let mut header = String::new();
    writeln!(
        header,
        "//! Corpus equipment and equipment-modifier records that belong to an\n\
         //! ALREADY-COMPILED book whose hand-authored per-book table does not hold\n\
         //! them — the `not-ingested` population of `docs/work-inventory.json`'s\n\
         //! `equipment`/`equipment_modifier` kinds, closed corpus-wide.\n\
         //!\n\
         //! **GENERATED — do not edit by hand.** Regenerate with\n\
         //! `PCGEN_CORPUS_ROOT=<pcgen>/data cargo run --locked --bin gen_equipment_gap_tables`.\n\
         //! The generator applies `v06_work_inventory`'s own record predicate, so a\n\
         //! row here is exactly a row that inventory reported `not-ingested`.\n\
         //!\n\
         //! `cost_gp`/`weight_lbs` are `None` when the corpus record carries no such\n\
         //! token, or carries a PCGen formula this table deliberately does not\n\
         //! evaluate — never a fabricated flat number. `description` joins the\n\
         //! record's `DESC:` and `SPROP:` tokens when both are present.\n\
         //!\n\
         //! Total: {total} rows.\n"
    )?;
    writeln!(
        header,
        "/// One recovered corpus equipment row. Deliberately one flat shape for\n\
         /// every book: unlike the hand-authored per-book tables (each with its own\n\
         /// `EquipmentCategory` enum and field set), these rows exist to be chained\n\
         /// into `equipment_resolver::equipment_catalog_rows()` and rendered by the\n\
         /// desktop equipment catalog, both of which read exactly these fields.\n\
         #[derive(Debug, Clone, Copy, PartialEq)]\n\
         pub struct EquipmentGapRow {{\n\
         \x20   /// One of `equipment_resolver`'s `EQUIPMENT_BOOK_*` codes.\n\
         \x20   pub book: &'static str,\n\
         \x20   /// The record's `KEY:` token when it carries one, else its display name.\n\
         \x20   pub key: &'static str,\n\
         \x20   pub name: &'static str,\n\
         \x20   /// The catalog category, matching the `EquipmentCategory` variant names\n\
         \x20   /// the per-book tables project onto `EquipmentCatalogEntryDto::category`.\n\
         \x20   pub category: &'static str,\n\
         \x20   pub cost_gp: Option<f64>,\n\
         \x20   pub weight_lbs: Option<f64>,\n\
         \x20   pub description: Option<&'static str>,\n\
         }}\n"
    )?;
    writeln!(
        header,
        "/// Every recovered row, in book order. The order is load-bearing the same\n\
         /// way `equipment_catalog_rows()`'s is: first match wins for key lookup.\n\
         pub fn equipment_gap_rows() -> impl Iterator<Item = &'static EquipmentGapRow> {{\n\
         \x20   [{}]\n\
         \x20       .into_iter()\n\
         \x20       .flat_map(|rows| rows.iter())\n\
         }}",
        BOOK_INPUTS
            .iter()
            .map(|b| format!("{}_GAP_ROWS", b.slug.to_uppercase()))
            .collect::<Vec<_>>()
            .join(", ")
    )?;

    let generated = format!("{header}{body}");

    // Provenance gate (`epic-3-provenance`): screen the text BEFORE writing it.
    let hits = screen_generated_table(OUTPUT_RELATIVE_PATH, &generated);
    if !hits.is_empty() {
        eprintln!("PI screening HARD STOP — {} hit(s), nothing written:", hits.len());
        for hit in &hits {
            eprintln!("  {hit:?}");
        }
        std::process::exit(1);
    }

    std::fs::write(Path::new(OUTPUT_RELATIVE_PATH), &generated)?;
    println!("wrote {OUTPUT_RELATIVE_PATH}: {total} rows");
    for (slug, n) in &totals {
        println!("  {slug:28} {n:5}");
    }
    println!("pi-screening: CLEAN (0 hits over the generated text)");
    Ok(())
}
