//! Codegen for `rules_tables::feat_gap_tables` — the corpus `feat` records
//! that live in an **already-compiled** book whose hand-authored per-book feat
//! table does not hold them.
//!
//! # Why this binary exists
//!
//! `docs/work-inventory.json` classifies a `feat` unit as `not-ingested`
//! (`evidence: "feat_key_absent_from_catalog"`) when the book has a compiled
//! rule set but `feats_all::all_feat_tables()` holds no record matching the
//! corpus record's `KEY:` or its display name. Those are real gaps inside
//! started books — not un-started books — and closing them needs no new
//! `RuleSetId`, no new corpus cache and no new player surface: the desktop
//! feat catalog already renders every record `all_feat_tables()` yields.
//!
//! This is the feat sibling of `gen_equipment_gap_tables`, and it follows
//! that binary's contract exactly: the same record predicate as
//! `v06_work_inventory::enumerate_file`, an already-held filter derived from
//! the hand-authored tables themselves (never a hand-maintained exclusion
//! list), and a PI screen run over the generated text **before** it is
//! written.
//!
//! # What it deliberately does not do
//!
//! It never invents a value. `description` is the record's own `DESC:` (joined
//! with `BENEFIT:` when both are present, exactly as
//! `ultimate_wilderness::feat_tables`' own projection does for the same corpus
//! shape) and is `None` when the record carries neither. `prerequisites` is
//! every top-level `PRE`-family token verbatim, and `None` — never `Some(&[])`
//! — when the record carries none.
//!
//! `category` is the corpus `TYPE:` token's **first dot-segment, verbatim**,
//! not a per-book `FeatCategory` enum variant. That is a deliberate
//! difference from the hand-authored tables and it is the honest one: a gap
//! row has no per-book table to take a variant name from, and mapping the
//! corpus facet onto some book's enum would invent a classification the
//! corpus never made. `feats_all::FeatCatalogRecord::category` is already a
//! `&'static str` for precisely this reason (see its doc comment).
//!
//! Run it with a local PCGen corpus checkout:
//!
//! ```text
//! PCGEN_CORPUS_ROOT="$HOME/workspace/repos/pcgen/data" \
//!   cargo run --locked --bin gen_feat_gap_tables
//! ```

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use codex::rules_core::pi_table_sweep::screen_generated_table;
use codex::rules_core::rules_tables::feats_all::hand_authored_feat_tables;
use codex::rules_core::rules_tables::RuleSetId;

/// Where the generated table lands, relative to the crate root.
const OUTPUT_RELATIVE_PATH: &str = "src/rules_core/rules_tables/feat_gap_tables.rs";

/// One book's gap-lane inputs: the `RuleSetId` the joined catalog files its
/// records under, the `RuleSetId` variant name to emit in generated source,
/// and each `.lst` path relative to the corpus root.
struct BookInput {
    rule_set: RuleSetId,
    variant: &'static str,
    slug: &'static str,
    files: &'static [&'static str],
}

/// Every book that carries at least one `not-ingested` feat unit, with the
/// exact files those units come from. Derived from
/// `docs/work-inventory.json`'s own `source_file` field over the
/// `status == "not-ingested"` feat population — not guessed from a directory
/// glob, so a file with no gap is never re-parsed and cannot introduce a row
/// nobody asked for.
const BOOK_INPUTS: &[BookInput] = &[
    BookInput {
        rule_set: RuleSetId::Crb,
        variant: "Crb",
        slug: "core_rulebook",
        files: &["pathfinder/paizo/roleplaying_game/core_rulebook/cr_feats.lst"],
    },
    BookInput {
        rule_set: RuleSetId::Ce,
        variant: "Ce",
        slug: "core_essentials",
        // **Corrected `SD31-E6-F8-001`** — this was previously filed under
        // `RuleSetId::Crb` on the theory that `core_rulebook.pcc`'s
        // unconditional include of `core_essentials` made CRB the "observed
        // host". That predates `RuleSetId::Ce` existing as its own compiled
        // rule set (added later for companion/familiar content); now that it
        // does, `classify()`'s feat arm resolves a `core_essentials`-directory
        // record's `engine_book` straight from its own `source_book` field
        // ("core_essentials" -> `RuleSetId::Ce`, `own_engine_book`, never the
        // CRB shared-library-host fallback) — so filing these 15 records
        // under Crb left them permanently unreachable through the ONLY path
        // `classify()` actually checks, regardless of which real-world book
        // Decision 9's separate content re-attribution says they belong to
        // (`bestiary`, per that decision's SOURCELONG join — a different
        // question from which RULE SET serves them at chargen).
        files: &["pathfinder/paizo/roleplaying_game/core_essentials/ce_feats.lst"],
    },
    BookInput {
        rule_set: RuleSetId::Arg,
        variant: "Arg",
        slug: "advanced_race_guide",
        files: &["pathfinder/paizo/roleplaying_game/advanced_race_guide/arg_feats.lst"],
    },
    BookInput {
        rule_set: RuleSetId::Uc,
        variant: "Uc",
        slug: "ultimate_combat",
        files: &["pathfinder/paizo/roleplaying_game/ultimate_combat/uc_feats.lst"],
    },
    BookInput {
        rule_set: RuleSetId::Ui,
        variant: "Ui",
        slug: "ultimate_intrigue",
        // `support/ui_feats_oa.lst` is loaded by `ultimate_intrigue.pcc`
        // behind `PRECAMPAIGN:1,Occult Adventures` — the gate is on the pcc
        // LOAD LINE, not inside the `.lst` (a `grep PRECAMPAIGN` over the
        // file itself returns 0). It is included here anyway, and the reason
        // is that the gate governs which CAMPAIGN activates the file, not
        // which BOOK ships it: these three records are Ultimate Intrigue's
        // own, `v06_work_inventory` enumerates them as `ultimate_intrigue`
        // units, and this engine models no campaign activation to gate on.
        // Filing them under `Ui` is the accurate book attribution; excluding
        // them would leave three units `not-ingested` forever with no lane
        // that could ever close them.
        files: &["pathfinder/paizo/roleplaying_game/ultimate_intrigue/support/ui_feats_oa.lst"],
    },
    BookInput {
        rule_set: RuleSetId::Um,
        variant: "Um",
        slug: "ultimate_magic",
        files: &[
            "pathfinder/paizo/roleplaying_game/ultimate_magic/um_feats.lst",
            "pathfinder/paizo/roleplaying_game/ultimate_magic/um_feats_wordsofpower.lst",
        ],
    },
    BookInput {
        rule_set: RuleSetId::Upsi,
        variant: "Upsi",
        slug: "ultimate_psionics",
        files: &["pathfinder/dreamscarred_press/ultimate_psionics/up_feats.lst"],
    },
    BookInput {
        rule_set: RuleSetId::Uw,
        variant: "Uw",
        slug: "ultimate_wilderness",
        files: &["pathfinder/paizo/roleplaying_game/ultimate_wilderness/uw_feats.lst"],
    },
];

/// One parsed corpus feat record, before the already-held filter runs.
struct ParsedRecord {
    key: String,
    name: String,
    category: String,
    description: Option<String>,
    prerequisites: Vec<String>,
}

fn tab_fields(line: &str) -> Vec<&str> {
    line.split('\t').collect()
}

fn token_value<'a>(fields: &[&'a str], token: &str) -> Option<&'a str> {
    fields.iter().find_map(|f| f.trim().strip_prefix(token))
}

fn has_token(fields: &[&str], token: &str) -> bool {
    fields.iter().any(|f| f.trim().starts_with(token))
}

/// Parse one feat `.lst` under exactly `v06_work_inventory::enumerate_file`'s
/// record predicate for `Kind::Feat`, including its
/// `has_classifying_token(Kind::Feat, ..) == has_token("TYPE:")` requirement.
/// Kept as a standalone function so its agreement with that enumerator is
/// testable rather than asserted.
fn parse_lst(text: &str) -> Vec<ParsedRecord> {
    let mut out = Vec::new();
    for line in text.lines() {
        let fields = tab_fields(line);
        let Some(first) = fields.first() else { continue };
        let first = first.trim();
        if first.is_empty() || first.starts_with('#') {
            continue;
        }
        // An ALL-CAPS `TOKEN:` first field is file metadata, not a record.
        // (`CLASS:` is the enumerator's one exception and cannot occur in a
        // feat file.)
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
        // `.MOD` rows are overlays onto an existing record, never a new one.
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
        // `has_classifying_token(Kind::Feat, ..)`: a feat row without a
        // `TYPE:` token is a sub-choice helper, not an independent record.
        if !has_token(&fields, "TYPE:") {
            continue;
        }
        let key = token_value(&fields, "KEY:")
            .map(|k| k.to_string())
            .unwrap_or_else(|| name.clone());

        let category = token_value(&fields, "TYPE:")
            .and_then(|t| t.split('.').next())
            .map(str::trim)
            .filter(|c| !c.is_empty())
            .unwrap_or("General")
            .to_string();

        let desc = token_value(&fields, "DESC:").map(str::trim).filter(|d| !d.is_empty());
        let benefit = token_value(&fields, "BENEFIT:").map(str::trim).filter(|d| !d.is_empty());
        let description = match (desc, benefit) {
            (Some(d), Some(b)) if d != b => Some(format!("{d} {b}")),
            (Some(d), _) => Some(d.to_string()),
            (None, Some(b)) => Some(b.to_string()),
            (None, None) => None,
        };

        // Every top-level `PRE`-family token, verbatim and in source order,
        // including the negated `!PRE...` form — the same set
        // `feats_all::FeatCatalogRecord::prerequisites` documents.
        let prerequisites: Vec<String> = fields
            .iter()
            .map(|f| f.trim())
            .filter(|f| f.starts_with("PRE") || f.starts_with("!PRE"))
            .map(|f| f.to_string())
            .collect();

        out.push(ParsedRecord { key, name, category, description, prerequisites });
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

/// The corpus checkout, from the environment only.
///
/// No default and no tilde expansion: `tests/no_foreign_home_paths.rs` treats
/// both an absolute `/home/<someone>` literal and an unexpanded `~` default in
/// Rust source as failures, and it is right to — a baked-in path is one
/// machine's truth shipped as everyone's. Same variable
/// `gen_equipment_gap_tables` and `pathfinder_unchained::monk_features` use.
fn corpus_root() -> PathBuf {
    PathBuf::from(
        std::env::var("PCGEN_CORPUS_ROOT")
            .expect("PCGEN_CORPUS_ROOT must point at a local pcgen/data checkout"),
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = corpus_root();
    if !root.is_dir() {
        return Err(
            format!("PCGEN_CORPUS_ROOT does not point at a directory: {}", root.display()).into()
        );
    }

    // What the hand-authored tables already hold, per rule set — both the
    // record KEY and its display NAME, because `v06_work_inventory`'s own
    // check is `set.contains(unit.key) || set.contains(unit.name)` against a
    // set built from `entry.key` alone, then tested against both. Mirroring
    // that asymmetry exactly is what makes this generator's output set
    // precisely the classifier's `not-ingested` set; the equipment lane
    // recorded a 28-row divergence from getting it wrong in the other
    // direction, so it is checked here by count, not by assertion.
    //
    // Keyed on the `RuleSetId`'s `Debug` name rather than the value itself:
    // `RuleSetId` is deliberately not `Ord`, and inventing an ordering for it
    // here just to key a map would be a change to a shared type made for one
    // binary's convenience.
    let mut held: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for book in hand_authored_feat_tables() {
        let set = held.entry(format!("{:?}", book.rule_set)).or_default();
        for entry in book.entries {
            set.insert(entry.key.to_string());
        }
    }

    let mut body = String::new();
    let mut totals: Vec<(&str, usize)> = Vec::new();

    for input in BOOK_INPUTS {
        let mut rows: Vec<ParsedRecord> = Vec::new();
        // Deduped on the record IDENTITY (`key`) alone: once a key is in the
        // book's catalog set every unit carrying it classifies as ingested
        // regardless of display name, so a second row would add nothing but a
        // duplicate catalog entry.
        let mut seen: BTreeSet<String> = BTreeSet::new();
        for rel in input.files {
            let path = root.join(rel);
            let text =
                std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            for record in parse_lst(&text) {
                let already = held
                    .get(&format!("{:?}", input.rule_set))
                    .map(|s| s.contains(&record.key) || s.contains(&record.name))
                    .unwrap_or(false);
                if already {
                    continue;
                }
                if !seen.insert(record.key.clone()) {
                    continue;
                }
                rows.push(record);
            }
        }

        totals.push((input.slug, rows.len()));
        writeln!(
            body,
            "\n/// {} — {} record(s) the hand-authored `{}` feat table does not hold.\npub static {}_FEAT_GAP_ROWS: &[FeatCatalogRecord] = &[",
            input.slug,
            rows.len(),
            input.slug,
            input.slug.to_uppercase()
        )?;
        for row in &rows {
            let prerequisites = if row.prerequisites.is_empty() {
                "None".to_string()
            } else {
                format!(
                    "Some(&[{}])",
                    row.prerequisites.iter().map(|p| rust_string(p)).collect::<Vec<_>>().join(", ")
                )
            };
            writeln!(
                body,
                "    FeatCatalogRecord {{ key: {}, category: {}, name: {}, description: {}, prerequisites: {} }},",
                rust_string(&row.key),
                rust_string(&row.category),
                rust_string(&row.name),
                match &row.description {
                    Some(d) => format!("Some({})", rust_string(d)),
                    None => "None".to_string(),
                },
                prerequisites
            )?;
        }
        writeln!(body, "];")?;
    }

    let total: usize = totals.iter().map(|(_, n)| *n).sum();
    let mut header = String::new();
    writeln!(
        header,
        "//! Corpus `feat` records that belong to an ALREADY-COMPILED book whose\n\
         //! hand-authored per-book feat table does not hold them — the\n\
         //! `not-ingested` population of `docs/work-inventory.json`'s `feat` kind,\n\
         //! closed corpus-wide.\n\
         //!\n\
         //! **GENERATED — do not edit by hand.** Regenerate with\n\
         //! `PCGEN_CORPUS_ROOT=<pcgen>/data cargo run --locked --bin gen_feat_gap_tables`.\n\
         //! The generator applies `v06_work_inventory`'s own record predicate for\n\
         //! `Kind::Feat`, so a row here is exactly a row that inventory reported\n\
         //! `not-ingested`.\n\
         //!\n\
         //! `description` is the record's `DESC:` joined with its `BENEFIT:` when\n\
         //! both are present, and `None` when it carries neither — never a\n\
         //! fabricated placeholder. `prerequisites` is every top-level `PRE`-family\n\
         //! token verbatim, and `None` (never `Some(&[])`) when there are none.\n\
         //! `category` is the corpus `TYPE:` token's first dot-segment verbatim,\n\
         //! NOT a per-book `FeatCategory` variant name: a gap row has no per-book\n\
         //! table to take a variant from, and mapping the corpus facet onto some\n\
         //! book's enum would invent a classification the corpus never made.\n\
         //!\n\
         //! Total: {total} rows.\n"
    )?;
    writeln!(header, "use super::feats_all::FeatCatalogRecord;\nuse super::RuleSetId;\n")?;
    writeln!(
        header,
        "/// The gap rows for one rule set, or an empty slice when that book has\n\
         /// none. Chained AFTER the book's hand-authored records by\n\
         /// `feats_all::all_feat_tables`, so a first-match key lookup keeps\n\
         /// resolving to the hand-authored record.\n\
         pub fn feat_gap_rows_for(rule_set: RuleSetId) -> &'static [FeatCatalogRecord] {{\n\
         \x20   match rule_set {{\n\
         {}\
         \x20       _ => &[],\n\
         \x20   }}\n\
         }}",
        BOOK_INPUTS
            .iter()
            .map(|b| format!(
                "        RuleSetId::{} => {}_FEAT_GAP_ROWS,\n",
                b.variant,
                b.slug.to_uppercase()
            ))
            .collect::<String>()
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
