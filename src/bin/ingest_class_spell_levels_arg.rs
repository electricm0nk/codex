//! SD-27 — ingests **per-class spell levels** from the PCGen spell corpus.
//!
//! **The gap this closes.** A spell record's `level` field (in every book's
//! `spell_list.rs` and in `data/corpus/<book>/spell/*.json`) is the MINIMUM
//! level across every class in that record's `CLASSES:` token — not the level
//! for any one class. The per-class answer lives only in the raw `CLASSES:`
//! token, e.g. `Tsunami ... CLASSES:Druid,Sorcerer,Wizard=9`.
//!
//! For the Core Rulebook, APG and ACG that token was already ingested, into the
//! twelve hand-generated `<class>_spell_list.rs` tables (`crb::wizard_spell_list`
//! and siblings). **The Advanced Race Guide was ingested later and never got the
//! same treatment**, so all 92 ARG spells had no per-class level anywhere in the
//! repo. This binary emits that missing table:
//! `rules_tables::advanced_race_guide::class_spell_levels`.
//!
//! **It also verifies itself against the three books that already have tables.**
//! `--verify` re-parses `cr_spells.lst`, `apg_spells.lst` and `acg_spells.lst`
//! with the same parser and diffs the result against the shipped
//! `<class>_spell_list.rs` tables. A parser that reproduces 12 independently
//! generated tables exactly is a parser whose ARG output can be trusted; a
//! divergence is printed, not swallowed.
//!
//! Run with `cargo run --bin ingest_class_spell_levels_arg -- [--verify] [--emit]`.
//! `PCGEN_CORPUS_ROOT` overrides the default `$HOME/workspace/repos/pcgen/data`.
//! `--emit` rewrites
//! `src/rules_core/rules_tables/advanced_race_guide/class_spell_levels.rs`;
//! without it the binary only reports.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const CRB_SPELLS: &str = "pathfinder/paizo/roleplaying_game/core_rulebook/cr_spells.lst";
const APG_SPELLS: &str =
    "pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_spells.lst";
const ACG_SPELLS: &str =
    "pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_spells.lst";
const ARG_SPELLS: &str = "pathfinder/paizo/roleplaying_game/advanced_race_guide/arg_spells.lst";

/// One base spell record's per-class levels, as stated by its `CLASSES:` token.
#[derive(Debug, Clone)]
struct SpellClasses {
    /// The record's `KEY:` token when it carries one, else its display name —
    /// the same identity every `spell_list.rs` uses, so the two join.
    key: String,
    /// `(class name verbatim from the corpus, spell level)`, sorted by name.
    /// Empty when the record carries no `CLASSES:` token at all.
    classes: Vec<(String, u8)>,
    /// True when the record carries no `CLASSES:` token — a real corpus gap,
    /// never filled in here.
    has_classes_token: bool,
}

fn pcgen_data_root() -> PathBuf {
    if let Ok(v) = std::env::var("PCGEN_CORPUS_ROOT") {
        return PathBuf::from(v);
    }
    let home =
        std::env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
    PathBuf::from(home).join("workspace/repos/pcgen/data")
}

/// Parses one `CLASSES:` token body into `(class, level)` pairs.
///
/// Grammar, read off the four corpus files rather than assumed:
/// pipe-separated groups, each `Name[,Name...]=<level>`; a level may carry a
/// trailing `[PREVAREQ:...]` gate (the APG Hero Points and Unchained Summoner
/// rules), which is stripped — `crb::wizard_spell_list` already rules those
/// records in, and dropping them is exactly the silent `int(level)` failure
/// that module's doc comment warns about. `.CLEARALL` (only ever on `.MOD`
/// rows) yields no pairs.
fn parse_classes_token(body: &str) -> Vec<(String, u8)> {
    let mut out: Vec<(String, u8)> = Vec::new();
    for group in body.split('|') {
        let group = group.trim();
        if group.is_empty() || group.starts_with(".CLEARALL") {
            continue;
        }
        let Some((names, level_raw)) = group.rsplit_once('=') else {
            continue;
        };
        let level_token = level_raw.split('[').next().unwrap_or("").trim();
        let Ok(level) = level_token.parse::<u8>() else {
            continue;
        };
        for name in names.split(',') {
            let name = name.split('[').next().unwrap_or("").trim();
            if name.is_empty() {
                continue;
            }
            out.push((name.to_owned(), level));
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Every spell record in one `.lst` file, with `.MOD` grafts folded in.
///
/// Base records mirror each `spell_list.rs`'s own exclusion discipline:
/// comments, blanks, the `SOURCELONG:` header row and `.COPY=` rows are not
/// base records. Where a name repeats (APG's one genuine duplicate,
/// `Resounding Blow`), the LAST occurrence wins — the same errata-reprint
/// rule `apg::spell_list` documents.
///
/// **`.MOD` rows are folded in, not skipped**, and that is load-bearing.
/// PCGen grants a class access to an *existing* spell by re-opening the
/// record: `Cure Light Wounds.MOD ... CLASSES:Witch=1`. Those grafts are the
/// bulk of Witch's, Inquisitor's, Alchemist's, Bloodrager's and Shaman's
/// real spell lists — skipping them reproduced only 114 of Witch's 326 rows.
/// A `.MOD` naming a base record this file does not define contributes a
/// row under that base record's key anyway (the graft is a statement about
/// the spell, not about the book that states it).
fn parse_spell_file(path: &Path) -> Vec<SpellClasses> {
    let text = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read the spell corpus {path:?}: {e}"));
    let mut by_key: BTreeMap<String, SpellClasses> = BTreeMap::new();
    let mut order: Vec<String> = Vec::new();
    let mut grafts: Vec<(String, Vec<(String, u8)>)> = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim_end();
        if trimmed.trim().is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let mut fields = trimmed.split('\t').filter(|f| !f.is_empty());
        let Some(name) = fields.next() else { continue };
        let name = name.trim();
        if name.is_empty() || name.starts_with("SOURCELONG:") || name.contains(".COPY=") {
            continue;
        }

        let mut key = name.to_owned();
        let mut classes_body: Option<String> = None;
        for field in trimmed.split('\t') {
            let field = field.trim();
            if let Some(rest) = field.strip_prefix("KEY:") {
                key = rest.to_owned();
            } else if let Some(rest) = field.strip_prefix("CLASSES:") {
                classes_body = Some(rest.to_owned());
            }
        }

        if let Some(base) = key.strip_suffix(".MOD") {
            if let Some(body) = classes_body {
                grafts.push((base.to_owned(), parse_classes_token(&body)));
            }
            continue;
        }

        let record = SpellClasses {
            key: key.clone(),
            classes: classes_body.as_deref().map(parse_classes_token).unwrap_or_default(),
            has_classes_token: classes_body.is_some(),
        };
        if by_key.insert(key.clone(), record).is_none() {
            order.push(key);
        }
    }

    for (base_key, added) in grafts {
        let record = by_key.entry(base_key.clone()).or_insert_with(|| {
            order.push(base_key.clone());
            SpellClasses {
                key: base_key.clone(),
                classes: Vec::new(),
                has_classes_token: false,
            }
        });
        if !added.is_empty() {
            record.has_classes_token = true;
        }
        record.classes.extend(added);
        record.classes.sort();
        record.classes.dedup();
    }

    order
        .into_iter()
        .map(|key| by_key.remove(&key).expect("key was inserted above"))
        .collect()
}

/// Groups parsed records into `class name -> sorted (spell key, level)`.
fn group_by_class(records: &[SpellClasses]) -> BTreeMap<String, Vec<(String, u8)>> {
    let mut out: BTreeMap<String, Vec<(String, u8)>> = BTreeMap::new();
    for record in records {
        for (class_name, level) in &record.classes {
            out.entry(class_name.clone())
                .or_default()
                .push((record.key.clone(), *level));
        }
    }
    for entries in out.values_mut() {
        entries.sort();
        entries.dedup();
    }
    out
}

/// The hub `class:<id>` for a corpus class name. `None` for a corpus class
/// this repo has no class id for (`Adept`, an NPC class), which is reported
/// rather than mapped to something plausible.
fn class_id_for(corpus_name: &str) -> Option<&'static str> {
    Some(match corpus_name {
        "Alchemist" => "class:alchemist",
        "Antipaladin" => "class:antipaladin",
        "Bard" => "class:bard",
        "Bloodrager" => "class:bloodrager",
        "Cleric" => "class:cleric",
        "Druid" => "class:druid",
        "Inquisitor" => "class:inquisitor",
        "Investigator" => "class:investigator",
        "Magus" => "class:magus",
        "Oracle" => "class:oracle",
        "Paladin" => "class:paladin",
        "Ranger" => "class:ranger",
        "Shaman" => "class:shaman",
        "Sorcerer" => "class:sorcerer",
        "Summoner" => "class:summoner",
        "Witch" => "class:witch",
        "Wizard" => "class:wizard",
        _ => return None,
    })
}

fn rust_str(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

fn emit_module(by_class: &BTreeMap<String, Vec<(String, u8)>>, record_count: usize) -> String {
    let mapped: BTreeMap<&'static str, &Vec<(String, u8)>> = by_class
        .iter()
        .filter_map(|(name, entries)| class_id_for(name).map(|id| (id, entries)))
        .collect();
    let total: usize = mapped.values().map(|e| e.len()).sum();

    let mut out = String::new();
    out.push_str(&format!(
        "//! Advanced Race Guide per-class spell levels — the `CLASSES:` token of\n\
         //! every real ARG spell record, split out per class.\n\
         //!\n\
         //! **Generated. Do not hand-edit.** Regenerate with\n\
         //! `cargo run --bin ingest_class_spell_levels_arg -- --emit`.\n\
         //!\n\
         //! **Why this table exists.** `spell_list::SPELL_LIST`'s `level` field is\n\
         //! the MINIMUM level across every class in the record's `CLASSES:` token,\n\
         //! not the level for any one class — the same defect\n\
         //! `crb::wizard_spell_list` was built to remove for the Core Rulebook.\n\
         //! CRB, APG and ACG each had their per-class levels ingested into the\n\
         //! twelve `<class>_spell_list.rs` tables; ARG landed later and never did,\n\
         //! so until this module no ARG spell had a per-class level anywhere, and\n\
         //! `pilot_compute`'s prepared-spell level gate silently accepted every one\n\
         //! of them at every caster level.\n\
         //!\n\
         //! Source: `{ARG_SPELLS}`, {record_count} spell records (base rows plus\n\
         //! the file's own `.MOD` class grafts folded into the record they name;\n\
         //! `.COPY=` variant rows excluded, matching `spell_list.rs`), yielding\n\
         //! {total} (class, spell, level) rows across {classes} classes. The\n\
         //! generator that produced this file reproduces all twelve shipped\n\
         //! CRB/APG/ACG per-class tables exactly from the same corpus token\n\
         //! (`--verify`), which is what makes its ARG output trustworthy.\n\
         //!\n\
         //! Corpus class names with no hub class id are deliberately absent rather\n\
         //! than mapped to a plausible neighbour.\n\n",
        classes = mapped.len(),
    ));
    out.push_str(
        "/// `(class id, &[(spell key, that class's spell level)])`, classes and\n\
         /// spell keys both sorted. Chained by `rules_tables::class_spell_levels`.\n\
         ///\n\
         /// The `allow` is for `src/bin/sd27_gen_book_cache.rs`, which still\n\
         /// `#[path]`-includes this book's `mod.rs` a second time into its own\n\
         /// crate. That include is stale — the module has since been registered\n\
         /// in `rules_tables/mod.rs` — and nothing in that binary consumes this\n\
         /// table, so the duplicate compilation reports it dead.\n\
         #[allow(dead_code)]\n\
         pub const ARG_CLASS_SPELL_LEVELS: &[(&str, &[(&str, u8)])] = &[\n",
    );
    for (class_id, entries) in &mapped {
        out.push_str(&format!("    ({}, &[\n", rust_str(class_id)));
        for (key, level) in entries.iter() {
            out.push_str(&format!("        ({}, {level}),\n", rust_str(key)));
        }
        out.push_str("    ]),\n");
    }
    out.push_str("];\n");
    out
}

/// One `--verify` row: `(label, corpus class name, the shipped table)`.
type ShippedTableCheck = (&'static str, &'static str, &'static [(&'static str, u8)]);

/// Diffs this binary's parse of one book against a shipped per-class table.
fn verify_against(
    label: &str,
    parsed: &BTreeMap<String, Vec<(String, u8)>>,
    corpus_class: &str,
    shipped: &[(&str, u8)],
) -> bool {
    let mine: BTreeSet<(String, u8)> = parsed
        .get(corpus_class)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .collect();
    let theirs: BTreeSet<(String, u8)> =
        shipped.iter().map(|(k, l)| ((*k).to_owned(), *l)).collect();
    let only_mine: Vec<_> = mine.difference(&theirs).collect();
    let only_theirs: Vec<_> = theirs.difference(&mine).collect();
    if only_mine.is_empty() && only_theirs.is_empty() {
        println!("  {label:<28} {:>4} rows  MATCH", mine.len());
        true
    } else {
        println!(
            "  {label:<28} {:>4} parsed vs {:>4} shipped  DIVERGES\n    only in parse:  {:?}\n    only in table:  {:?}",
            mine.len(),
            theirs.len(),
            only_mine,
            only_theirs
        );
        false
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let do_verify = args.iter().any(|a| a == "--verify");
    let do_emit = args.iter().any(|a| a == "--emit");

    let root = pcgen_data_root();
    let arg_records = parse_spell_file(&root.join(ARG_SPELLS));
    let arg_by_class = group_by_class(&arg_records);

    println!("arg_spells.lst: {} base records", arg_records.len());
    let no_classes: Vec<&str> = arg_records
        .iter()
        .filter(|r| !r.has_classes_token)
        .map(|r| r.key.as_str())
        .collect();
    println!(
        "  records with no CLASSES: token at all: {} {:?}",
        no_classes.len(),
        no_classes
    );
    for (class_name, entries) in &arg_by_class {
        println!(
            "  {class_name:<14} {:>3} spells{}",
            entries.len(),
            if class_id_for(class_name).is_none() {
                "   (NO HUB CLASS ID — not emitted)"
            } else {
                ""
            }
        );
    }

    // Which SHIPPED catalog records the corpus states no class for at all.
    // Scanned across all four files together, because a record defined in
    // one book can be granted to a class by a `.MOD` row in another.
    {
        use codex::rules_core::rules_tables::{acg, advanced_race_guide, apg, crb};

        let mut mapped: BTreeSet<String> = BTreeSet::new();
        for rel in [CRB_SPELLS, APG_SPELLS, ACG_SPELLS, ARG_SPELLS] {
            for record in parse_spell_file(&root.join(rel)) {
                if !record.classes.is_empty() {
                    mapped.insert(record.key);
                }
            }
        }

        println!("\nshipped catalog records the corpus states NO class for, per book:");
        let books: Vec<(&str, Vec<&str>)> = vec![
            ("CRB", crb::spell_list::SPELL_LIST.iter().map(|e| e.key).collect()),
            ("APG", apg::spell_list::SPELL_LIST.iter().map(|e| e.key).collect()),
            ("ACG", acg::spell_list::SPELL_LIST.iter().map(|e| e.key).collect()),
            (
                "ARG",
                advanced_race_guide::spell_list::SPELL_LIST.iter().map(|e| e.key).collect(),
            ),
        ];
        for (label, keys) in books {
            let unmapped: Vec<&str> =
                keys.iter().copied().filter(|key| !mapped.contains(*key)).collect();
            println!(
                "  {label} {:>4} served, {:>3} unmapped{}",
                keys.len(),
                unmapped.len(),
                if unmapped.len() <= 20 { format!(" {unmapped:?}") } else { String::new() }
            );
        }
    }

    if do_verify {
        use codex::rules_core::rules_tables::{acg, apg, crb};

        let mut three_books: Vec<SpellClasses> = Vec::new();
        for rel in [CRB_SPELLS, APG_SPELLS, ACG_SPELLS] {
            three_books.extend(parse_spell_file(&root.join(rel)));
        }
        let parsed = group_by_class(&three_books);
        println!(
            "\nverify: cr+apg+acg parse ({} base records) vs the shipped per-class tables",
            three_books.len()
        );
        let checks: Vec<ShippedTableCheck> = vec![
            ("crb::wizard", "Wizard", crb::wizard_spell_list::WIZARD_SPELL_LIST),
            ("crb::sorcerer", "Sorcerer", crb::sorcerer_spell_list::SORCERER_SPELL_LIST),
            ("crb::bard", "Bard", crb::bard_spell_list::BARD_SPELL_LIST),
            ("crb::cleric", "Cleric", crb::cleric_spell_list::CLERIC_SPELL_LIST),
            ("crb::druid", "Druid", crb::druid_spell_list::DRUID_SPELL_LIST),
            ("crb::paladin", "Paladin", crb::paladin_spell_list::PALADIN_SPELL_LIST),
            ("crb::ranger", "Ranger", crb::ranger_spell_list::RANGER_SPELL_LIST),
            ("apg::alchemist", "Alchemist", apg::alchemist_spell_list::ALCHEMIST_SPELL_LIST),
            ("apg::inquisitor", "Inquisitor", apg::inquisitor_spell_list::INQUISITOR_SPELL_LIST),
            ("apg::witch", "Witch", apg::witch_spell_list::WITCH_SPELL_LIST),
            ("acg::bloodrager", "Bloodrager", acg::bloodrager_spell_list::BLOODRAGER_SPELL_LIST),
            ("acg::shaman", "Shaman", acg::shaman_spell_list::SHAMAN_SPELL_LIST),
        ];
        let mut all_match = true;
        for (label, corpus_class, shipped) in checks {
            all_match &= verify_against(label, &parsed, corpus_class, shipped);
        }
        println!(
            "\nverify result: {}",
            if all_match { "all shipped tables reproduced" } else { "DIVERGENCES ABOVE" }
        );
    }

    if do_emit {
        let module = emit_module(&arg_by_class, arg_records.len());
        let out = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src/rules_core/rules_tables/advanced_race_guide/class_spell_levels.rs");
        fs::write(&out, module).expect("failed to write the generated module");
        println!("\nwrote {out:?}");
    }
}
