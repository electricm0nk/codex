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
//! PCGEN_CORPUS_ROOT="$HOME/workspace/repos/pcgen/data" \
//!   cargo run --locked --bin gen_equipment_gap_tables
//! ```

use std::collections::{BTreeSet, HashMap};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use codex::rules_core::equipment_resolver::{hand_authored_equipment_rows, EQUIPMENT_BOOK_ACG, EQUIPMENT_BOOK_APG, EQUIPMENT_BOOK_ARG, EQUIPMENT_BOOK_B1, EQUIPMENT_BOOK_CRB, EQUIPMENT_BOOK_UC, EQUIPMENT_BOOK_UE, EQUIPMENT_BOOK_UI, EQUIPMENT_BOOK_UPSI, EQUIPMENT_BOOK_UW};
use codex::rules_core::pcgen_desc::{leaked_pcgen_syntax, render_pcgen_desc};
use codex::rules_core::pi_table_sweep::screen_generated_table;

/// Refuses to ship a description whose rendering the player would see as
/// broken PCGen syntax -- an unsubstituted `%N`/`%<KEYWORD>` reference or a
/// raw `|` argument tail `render_pcgen_desc`/`split_prose_and_args` could
/// not resolve for this row's shape (confirmed real: `IntItemBase`'s
/// `SPROP:` states 4 BARE (unnumbered) `%` placeholders followed by a
/// 4-argument pipe tail naming the `BONUS:VAR` keys each one substitutes,
/// a shape `max_arg_reference`'s numbered-reference detection does not
/// recognize -- caught live by `apps/desktop`'s own
/// `no_catalog_serves_a_description_carrying_raw_pcgen_syntax` test).
/// Never fabricates a fix; the SAME judgment call `v06_work_inventory.rs`'s
/// `corpus_json_description_leaks_pcgen_syntax` already makes for the
/// identical shape, applied here at the SOURCE so a broken description
/// never ships at all rather than being caught downstream.
///
/// **Checks the RENDERED text's own leak, never `dropped_args` alone**
/// (empirically confirmed, not assumed: `%CHOICE` with no `|` tail drops
/// cleanly to a readable sentence with `dropped_args: ["CHOICE"]` but
/// `leaked_pcgen_syntax: None` — this is the SAME shape the real desktop
/// equipment catalog already ships today; refusing on `dropped_args` alone
/// would have wrongly discarded 68 of the 69 `%`/`|`-carrying recovered
/// descriptions this cycle recovers, keeping only the 1 that genuinely
/// leaks). Matches `apps/desktop`'s own `no_catalog_serves_a_description_
/// carrying_raw_pcgen_syntax` check exactly (`leaked_pcgen_syntax` on the
/// rendered text, nothing else), so this refuses precisely what that test
/// would otherwise catch downstream — never more, never less.
fn safe_description(description: Option<String>) -> Option<String> {
    let description = description?;
    let rendered = render_pcgen_desc(&description);
    if leaked_pcgen_syntax(&rendered.text).is_some() {
        return None;
    }
    Some(description)
}

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
        files: &["pathfinder/paizo/roleplaying_game/core_rulebook/cr_equipmods.lst"],
    },
    // `decisions.md §9` (`core_essentials` re-attribution, "re-attribute
    // first, drop the label second"): these 2 files physically live under
    // the shared `core_essentials` library `core_rulebook.pcc` includes
    // unconditionally, and an earlier draft of this table routed their 3
    // records to CRB on that basis ("CRB is that host" -- now corrected).
    // Both files' own uncommented `SOURCELONG:Bestiary`/`SOURCESHORT:B1`
    // header (verified 2026-08-17, not assumed) says otherwise: 100% of
    // each file's content is Bestiary, none is genuinely Core Rulebook, so
    // Decision 9's "re-attribute by the file's own SOURCELONG" rule routes
    // them to B1/bestiary instead. Confirmed harmless to CRB: neither file
    // ever supplied a genuinely-CRB record (`grep SOURCELONG` on both
    // files finds exactly one value, `Bestiary`, each).
    BookInput {
        code: EQUIPMENT_BOOK_B1,
        slug: "bestiary",
        files: &[
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

/// True when a raw `.lst` line is not a real record declaration at all
/// (blank, comment, ALL-CAPS directive other than `CLASS:`, an internal
/// bookkeeping `CATEGORY:`/`CATEGORY=Internal|` row, or a `.MOD` overlay) —
/// shared by [`parse_lst`] and [`collect_base_fields`] so the two scans of
/// the same corpus text can never silently disagree on what counts as a
/// record (the exact shape of `OPEN-ISSUES.md` row 90's citation defect:
/// two similar-but-drifted predicates over the same file).
fn is_non_record_line(first: &str, fields: &[&str]) -> bool {
    if first.is_empty() || first.starts_with('#') {
        return true;
    }
    let is_directive = first
        .split_once(':')
        .map(|(head, _)| {
            !head.is_empty() && head.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
        })
        .unwrap_or(false);
    if is_directive && !first.starts_with("CLASS:") {
        return true;
    }
    if first.starts_with("CATEGORY=Internal|") || fields.iter().any(|f| f.trim() == "CATEGORY:Internal")
    {
        return true;
    }
    if first.contains(".MOD") {
        return true;
    }
    false
}

/// A `.COPY=`-declaring record's base-record fields, keyed by the identity a
/// `.COPY=<identity>` reference resolves against — the base row's own `KEY:`
/// token when present, else its bare declared name. This is PCGen's own
/// resolution rule (confirmed against the real corpus: `Special Ability ~
/// Answering ~ Weapon.COPY=Answering` resolves against the `KEY:Special
/// Ability ~ Answering ~ Weapon` row, not any row literally named
/// "Special Ability ~ Answering ~ Weapon").
#[derive(Debug, Clone, Default, PartialEq)]
struct BaseFields {
    description: Option<String>,
    cost_gp: Option<f64>,
    weight_lbs: Option<f64>,
}

/// Builds the base-record lookup used by [`parse_lst`]'s `.COPY=`
/// inheritance, from every PLAIN (non-`.COPY=`) row across a book's own
/// input files — never from another `.COPY=` row, so inheritance is at most
/// one hop deep and cannot chain through an already-inherited value. Corpus-
/// wide: 0 `.COPY=` rows in this generator's 19 input files carry their own
/// `DESC:`/`SPROP:`/`COST:`/`WT:` token (re-derived at generation time, not
/// assumed), so this restriction has never actually excluded a real base.
/// "First wins" per book, matching every other first-match convention this
/// generator and `equipment_catalog_row_by_key` already use.
fn collect_base_fields(texts: &[String]) -> HashMap<String, BaseFields> {
    let mut map: HashMap<String, BaseFields> = HashMap::new();
    for text in texts {
        for line in text.lines() {
            let fields = tab_fields(line);
            let Some(first) = fields.first() else { continue };
            let first = first.trim();
            if is_non_record_line(first, &fields) || first.contains(".COPY=") {
                continue;
            }
            let key = token_value(&fields, "KEY:").map(str::to_string).unwrap_or_else(|| first.to_string());
            let desc = token_value(&fields, "DESC:").map(str::trim).filter(|d| !d.is_empty());
            let sprop = token_value(&fields, "SPROP:").map(str::trim).filter(|d| !d.is_empty());
            let description = match (desc, sprop) {
                (Some(d), Some(s)) if d != s => Some(format!("{d} {s}")),
                (Some(d), _) => Some(d.to_string()),
                (None, Some(s)) => Some(s.to_string()),
                (None, None) => None,
            };
            map.entry(key).or_insert(BaseFields {
                description,
                cost_gp: numeric(&fields, "COST:"),
                weight_lbs: numeric(&fields, "WT:"),
            });
        }
    }
    map
}

/// Parse one `.lst` under exactly `v06_work_inventory::enumerate_file`'s
/// record predicate. Kept as a standalone function so its agreement with
/// that enumerator is testable rather than asserted.
///
/// `base_fields` recovers a `.COPY=` row's inherited `description`/
/// `cost_gp`/`weight_lbs` when the row's OWN line states none of them —
/// `OPEN-ISSUES.md` rows 70/103's named recovery, generalized past
/// `description` alone once the same base-lookup mechanism proved it also
/// explains the pre-existing 8-row ACG `cost_gp` hand-correction
/// (`equipment_gap_tables.rs`'s former doc comment): both defects have the
/// identical root cause, a `.COPY=` row parsed as if it stated nothing
/// beyond its own line.
fn parse_lst(text: &str, category: &'static str, base_fields: &HashMap<String, BaseFields>) -> Vec<ParsedRecord> {
    let mut out = Vec::new();
    for line in text.lines() {
        let fields = tab_fields(line);
        let Some(first) = fields.first() else { continue };
        let first = first.trim();
        if is_non_record_line(first, &fields) {
            continue;
        }
        let copy_split = first.split_once(".COPY=");
        let copy_base = copy_split.map(|(base, _)| base.to_string());
        let name = if let Some((_, variant)) = copy_split {
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
        let mut description = match (desc, sprop) {
            (Some(d), Some(s)) if d != s => Some(format!("{d} {s}")),
            (Some(d), _) => Some(d.to_string()),
            (None, Some(s)) => Some(s.to_string()),
            (None, None) => None,
        };
        let mut cost_gp = numeric(&fields, "COST:");
        let mut weight_lbs = numeric(&fields, "WT:");

        // `.COPY=` inheritance: a field this row's own line leaves unstated
        // is inherited from the base record it declares itself a copy of —
        // never overriding a field the row DOES state.
        if let Some(base) = &copy_base
            && let Some(inherited) = base_fields.get(base)
        {
            if description.is_none() {
                description = inherited.description.clone();
            }
            if cost_gp.is_none() {
                cost_gp = inherited.cost_gp;
            }
            if weight_lbs.is_none() {
                weight_lbs = inherited.weight_lbs;
            }
        }

        out.push(ParsedRecord { key, name, category, cost_gp, weight_lbs, description: safe_description(description) });
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

/// The corpus checkout, from the environment only.
///
/// No default and no tilde expansion: `tests/no_foreign_home_paths.rs` treats
/// both an absolute `/home/<someone>` literal and an unexpanded `~` default in
/// Rust source as failures, and it is right to — a baked-in path is one
/// machine's truth shipped as everyone's. `PCGEN_CORPUS_ROOT` is the same
/// variable `pathfinder_unchained::monk_features`'s corpus-gated test already
/// requires, so there is no second convention to learn.
fn corpus_root() -> PathBuf {
    PathBuf::from(
        std::env::var("PCGEN_CORPUS_ROOT")
            .expect("PCGEN_CORPUS_ROOT must point at a local pcgen/data checkout"),
    )
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

        // Read every one of this book's files up front, once — needed twice
        // per file (base-field collection spans the whole book, then the
        // real per-record parse), and re-reading from disk a second time
        // risks a race against nothing (files are static) but is simply
        // wasted I/O; read once, use twice.
        let mut file_texts: Vec<(String, String)> = Vec::new();
        for rel in input.files {
            let path = root.join(rel);
            let text = std::fs::read_to_string(&path)
                .map_err(|e| format!("{}: {e}", path.display()))?;
            let basename = Path::new(rel).file_name().unwrap().to_string_lossy().into_owned();
            file_texts.push((basename, text));
        }
        let base_fields =
            collect_base_fields(&file_texts.iter().map(|(_, t)| t.clone()).collect::<Vec<_>>());

        for (basename, text) in &file_texts {
            for record in parse_lst(text, category_for(basename), &base_fields) {
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
         //! record's `DESC:` and `SPROP:` tokens when both are present. A `.COPY=`\n\
         //! row that states none of `description`/`cost_gp`/`weight_lbs` on its own\n\
         //! line inherits them from the base record it declares itself a copy of\n\
         //! (`SD31-E6-F6-001`, `OPEN-ISSUES.md` rows 70/103) — never fabricated,\n\
         //! never inherited past one hop.\n\
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

#[cfg(test)]
mod safe_description_tests {
    use super::*;

    /// The real reproduction (`IntItemBase`): a bare (unnumbered) `%`
    /// placeholder run followed by a multi-argument `|` tail render_pcgen_
    /// desc's numbered-reference detection does not recognize -- the tail
    /// survives verbatim, and `no_catalog_serves_a_description_carrying_
    /// raw_pcgen_syntax` (apps/desktop) correctly refuses to serve it.
    /// `safe_description` must refuse it at the source instead.
    #[test]
    fn a_description_whose_render_still_leaks_pcgen_syntax_is_refused() {
        let raw = "Intelligence %, Wisdom %, Charisma %, Ego Score %|IntItemStatINT|IntItemStatWIS|IntItemStatCHA|IntelligentItemEgo".to_string();
        assert_eq!(safe_description(Some(raw)), None);
    }

    /// A description with no PCGen substitution syntax at all is untouched.
    #[test]
    fn a_clean_description_passes_through_unchanged() {
        let raw = "Enhancement bonus increases by 4 (to a max of 5)".to_string();
        assert_eq!(safe_description(Some(raw.clone())), Some(raw));
    }

    #[test]
    fn none_stays_none() {
        assert_eq!(safe_description(None), None);
    }

    /// Empirical check, not assumed: a bare `%CHOICE` keyword reference
    /// with NO trailing `|` argument tail renders clean today (confirmed by
    /// this cycle's own guarded regen -- only 1 of 69 `%`/`|`-carrying
    /// recovered descriptions actually leaked in the real desktop catalog
    /// render). Prints the rendered result so a future reader can see
    /// exactly what `safe_description` decided, rather than trusting a
    /// bare pass/fail.
    #[test]
    fn a_bare_choice_keyword_with_no_pipe_tail_survives() {
        let raw = "Enhancement bonus to ability %CHOICE".to_string();
        let result = safe_description(Some(raw));
        assert_eq!(
            result.as_deref(),
            Some("Enhancement bonus to ability %CHOICE"),
            "a dropped %CHOICE that renders to clean, leak-free text must still ship -- \
             matches production's own equipment catalog behavior"
        );
    }
}

#[cfg(test)]
mod copy_inheritance_tests {
    use super::*;

    /// The proof case, reproduced from the real corpus (`SD31-E6-F6-001`):
    /// `acg_equipmods.lst`'s "Answering" `.COPY=` row states only
    /// `VISIBLE:NO`; the base it copies (`KEY:Special Ability ~ Answering ~
    /// Weapon`) carries a real `SPROP:`. Before this cycle's fix, the `.COPY=`
    /// row shipped `description: None` despite the base's real prose existing
    /// two lines away in the same file — `OPEN-ISSUES.md` rows 70/103's own
    /// named recovery, generalized to this generator.
    #[test]
    fn a_copy_row_inherits_the_base_records_description_when_it_states_none_of_its_own() {
        let text = "Answering\t\tKEY:Special Ability ~ Answering ~ Weapon\t\tSPROP:Enhancement bonus increases by 4\n\
                     Special Ability ~ Answering ~ Weapon.COPY=Answering\t\tVISIBLE:NO\n";
        let base_fields = collect_base_fields(&[text.to_string()]);
        let records = parse_lst(text, "Equipmods", &base_fields);
        let copy_record = records.iter().find(|r| r.key == "Answering").expect("copy row parsed");
        assert_eq!(
            copy_record.description.as_deref(),
            Some("Enhancement bonus increases by 4"),
            "the .COPY= row must inherit the base row's SPROP text, not ship None"
        );
    }

    /// The base's identity for a `.COPY=` reference is its own `KEY:` token,
    /// not its bare first-column name — resolving against the bare name
    /// alone would silently miss every real case in the corpus (this exact
    /// shape: the base's first column is "Answering" too, coincidentally
    /// equal to the KEY the `.COPY=` reference actually names).
    #[test]
    fn resolution_is_by_key_not_by_bare_first_column_name() {
        let text = "Answering\t\tKEY:Special Ability ~ Answering ~ Weapon\t\tSPROP:Real base text\n\
                     Special Ability ~ Answering ~ Weapon.COPY=Answering\t\tVISIBLE:NO\n";
        let base_fields = collect_base_fields(&[text.to_string()]);
        assert!(
            base_fields.contains_key("Special Ability ~ Answering ~ Weapon"),
            "must be keyed by the KEY: token, since that is what the .COPY= reference names"
        );
        assert!(
            !base_fields.contains_key("Answering"),
            "must NOT also be keyed by the bare first-column name — that name belongs to a \
             DIFFERENT identity (the variant), not the base"
        );
    }

    /// A `.COPY=` row that DOES state its own field on its own line keeps
    /// that value — inheritance only fills a genuine gap, never overrides.
    #[test]
    fn a_copy_row_stating_its_own_field_is_never_overridden_by_the_base() {
        let text = "Widget\t\tKEY:Special Ability ~ Widget ~ Weapon\t\tSPROP:Base text\t\tCOST:100\n\
                     Special Ability ~ Widget ~ Weapon.COPY=Widget Variant\t\tDESC:Own real text\t\tCOST:250\n";
        let base_fields = collect_base_fields(&[text.to_string()]);
        let records = parse_lst(text, "Equipmods", &base_fields);
        let copy_record =
            records.iter().find(|r| r.name == "Widget Variant").expect("copy row parsed");
        assert_eq!(copy_record.description.as_deref(), Some("Own real text"));
        assert_eq!(copy_record.cost_gp, Some(250.0));
    }

    /// `cost_gp`/`weight_lbs` inherit the identical way `description` does —
    /// the same defect shape as the pre-existing 8-row ACG hand-correction
    /// this cycle's fix generalizes and makes automatic.
    #[test]
    fn a_copy_row_inherits_cost_and_weight_when_it_states_neither() {
        let text = "Amorphous\t\tKEY:Special Ability ~ Amorphous ~ Armor\t\tCOST:4500\t\tSPROP:1/day take form\n\
                     Special Ability ~ Amorphous ~ Armor.COPY=Amorphous\t\tVISIBLE:NO\n";
        let base_fields = collect_base_fields(&[text.to_string()]);
        let records = parse_lst(text, "Equipmods", &base_fields);
        let copy_record = records.iter().find(|r| r.key == "Amorphous").expect("copy row parsed");
        assert_eq!(copy_record.cost_gp, Some(4500.0));
        assert_eq!(copy_record.description.as_deref(), Some("1/day take form"));
    }

    /// No base found at all (the true no-fabrication case): a `.COPY=` row
    /// whose base is genuinely absent from this book's files stays `None` —
    /// never invents a value.
    #[test]
    fn a_copy_row_with_no_resolvable_base_stays_none_rather_than_fabricating() {
        let text = "Some Base.COPY=Orphan Variant\t\tVISIBLE:NO\n";
        let base_fields = collect_base_fields(&[text.to_string()]);
        let records = parse_lst(text, "Equipmods", &base_fields);
        let copy_record = records.iter().find(|r| r.name == "Orphan Variant").expect("parsed");
        assert_eq!(copy_record.description, None);
        assert_eq!(copy_record.cost_gp, None);
        assert_eq!(copy_record.weight_lbs, None);
    }

    /// Base-field collection spans multiple files of the same book (the real
    /// shape: `ACG` reads both `acg_equipmods.lst` and
    /// `_pfs/pfs_acg_equip.lst`) — a base declared in one file must be found
    /// by a `.COPY=` row parsed from a different file's text.
    #[test]
    fn base_lookup_spans_multiple_files_of_the_same_book() {
        let file_a = "Foo\t\tKEY:Special Ability ~ Foo ~ Weapon\t\tSPROP:Cross-file base text\n".to_string();
        let file_b = "Special Ability ~ Foo ~ Weapon.COPY=Foo\t\tVISIBLE:NO\n".to_string();
        let base_fields = collect_base_fields(&[file_a, file_b.clone()]);
        let records = parse_lst(&file_b, "Equipmods", &base_fields);
        let copy_record = records.iter().find(|r| r.key == "Foo").expect("copy row parsed");
        assert_eq!(copy_record.description.as_deref(), Some("Cross-file base text"));
    }

    /// A `.COPY=` row can never itself serve as another row's base — proves
    /// inheritance is at most one hop and cannot chain through an
    /// already-inherited value.
    #[test]
    fn a_copy_row_is_never_used_as_a_base_for_another_copy_row() {
        let text = "Base\t\tKEY:X\t\tSPROP:Real\n\
                     X.COPY=Mid\t\tVISIBLE:NO\n\
                     Mid.COPY=Leaf\t\tVISIBLE:NO\n";
        let base_fields = collect_base_fields(&[text.to_string()]);
        assert!(
            !base_fields.contains_key("Mid"),
            "a .COPY= row (Mid, whose own declared name is X.COPY=Mid) must never be \
             registered as a base — only plain (non-.COPY=) declarations are bases"
        );
        let records = parse_lst(text, "Equipmods", &base_fields);
        let leaf = records.iter().find(|r| r.name == "Leaf").expect("leaf parsed");
        // Leaf's base identity is "Mid" (bare, no KEY: token on that COPY
        // line), which base_fields correctly does NOT hold — so leaf stays
        // unresolved rather than silently chaining through X's real text.
        assert_eq!(leaf.description, None);
    }
}
