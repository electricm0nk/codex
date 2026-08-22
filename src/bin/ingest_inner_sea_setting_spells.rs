//! Ingests three Inner Sea campaign-setting books' spell catalogs into the
//! engine's spell-catalog capability -- SD-32 Gate 0 book-onboarding
//! precondition (`gate-0-book-onboarding-precondition`, AT-32-G0-003).
//!
//! **The finding this closes.** `spell_resolver::spell_catalog_rows()`
//! chained twelve books before this cycle; `inner_sea_faiths`,
//! `inner_sea_magic` and `inner_sea_temples` had no `RuleSetId` at all, so
//! `v06_work_inventory::classify`'s book-level gate (`engine_book_for` ->
//! `rule_set_for` -> `None`) short-circuited EVERY one of each book's units
//! -- not only their spells but `inner_sea_magic`'s 218 `class_feature`
//! units too (already ingested corpus-wide, `data/corpus/inner_sea_magic/
//! class_feature/`, but unreachable until this gate opens) -- to
//! `not-started`/`no_compiled_rule_set_for_book` before any per-kind arm
//! ever ran. This binary builds each book's missing `rules_tables::
//! <book>::spell_list::SPELL_LIST` from its real corpus `.lst` file, the
//! book's first compiled rule set of any kind (the same shape
//! `RuleSetId::Oa`/`RuleSetId::Mythic`/`RuleSetId::AdventurersGuide`
//! established), one book at a time, three books in one binary because the
//! per-record logic is identical across all three.
//!
//! `inner_sea_taverns`, the fourth SD-32 Gate 0 book, has no `*_spells.lst`
//! file at all -- its first compiled rule set is a `feat` gap-table entry
//! instead (`src/bin/gen_feat_gap_tables.rs`'s `BOOK_INPUTS`), not this
//! binary.
//!
//! **Reuses the tested general-purpose LST spell parser**
//! (`pcgen_import::lst_parser::spell::parse_lst_spell_file`), not a
//! reimplementation, and the identical `.MOD`/`.COPY=` exclusion and
//! `CLASSES:`/`DOMAINS:` level derivation every prior `ingest_<book>_spells`
//! binary (`ingest_adventurers_guide_spells.rs`, ...) already established --
//! copied deliberately rather than re-derived, per this program's own "fix
//! the source, not the symptom" / no second implementation discipline.
//!
//! **PI screening, both SD-30 invocation contracts, per record, before
//! writing anything** (`SD-30 decisions.md §52.3` blacklist sweep, `§53.5`
//! declared-PI reader), matching every prior spell-lane cycle. All three
//! files carry `NAMEISPI:YES`/`DESCISPI:YES` declarations (re-derived at
//! dispatch: `grep -oE '(NAMEISPI|DESCISPI):[A-Za-z]+' <file> | sort | uniq
//! -c`) -- real, non-hypothetical stakes for every one of these three runs.
//!
//! Run with `cargo run --locked --bin ingest_inner_sea_setting_spells`.
//! `PCGEN_CORPUS_ROOT` overrides the default `$HOME/workspace/repos/pcgen/data`.
//! Writes `src/rules_core/rules_tables/{inner_sea_faiths,inner_sea_magic,
//! inner_sea_temples}/spell_list.rs`.

use std::env;
use std::fs;
use std::path::PathBuf;

use codex::pcgen_import::lst_parser::spell::{parse_lst_spell_file, LstSpellRecord};
use codex::rules_core::pi_screening::{
    classify_field, classify_optional_field_declared, declared_product_identity,
};

/// One book's ingest inputs: the corpus `.lst` file (relative to
/// `PCGEN_CORPUS_ROOT`), where the generated module lands, and the book's
/// display name for the module doc comment.
struct BookInput {
    display_name: &'static str,
    lst_rel: &'static str,
    out_path: &'static str,
}

const BOOKS: &[BookInput] = &[
    BookInput {
        display_name: "Inner Sea Faiths",
        lst_rel: "pathfinder/paizo/campaign_setting/inner_sea_faiths/isf_spells.lst",
        out_path: "src/rules_core/rules_tables/inner_sea_faiths/spell_list.rs",
    },
    BookInput {
        display_name: "Inner Sea Magic",
        lst_rel: "pathfinder/paizo/campaign_setting/inner_sea_magic/ism_spells.lst",
        out_path: "src/rules_core/rules_tables/inner_sea_magic/spell_list.rs",
    },
    BookInput {
        display_name: "Inner Sea Temples",
        lst_rel: "pathfinder/paizo/campaign_setting/inner_sea_temples/istem_spells.lst",
        out_path: "src/rules_core/rules_tables/inner_sea_temples/spell_list.rs",
    },
];

fn pcgen_data_root() -> PathBuf {
    if let Ok(v) = env::var("PCGEN_CORPUS_ROOT") {
        return PathBuf::from(v);
    }
    let home = env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
    PathBuf::from(home).join("workspace/repos/pcgen/data")
}

/// A `.MOD` row (targets an existing record) or a `.COPY=` row (a named
/// variant of an existing record) is not itself a base declaration -- the
/// same exclusion every existing per-book `spell_list.rs` states in its own
/// doc comment.
fn is_base_declaration(name: &str) -> bool {
    !name.ends_with(".MOD") && !name.contains(".COPY=")
}

/// Every `Name=N` level suffix across one `CLASSES:`/`DOMAINS:`-shaped field
/// value (`"Alchemist=4|Druid,Sorcerer,Witch,Wizard=6"` -> `[4, 6]`).
///
/// Strips a trailing `[...]` PRESKILL/condition clause before looking for
/// the level's own `=` -- an unqualified `rsplit_once('=')` would grab the
/// LAST `=` in the whole group (inside a bracketed sub-condition), which
/// fails to parse as a `u8` and silently discards a real level as though the
/// record carried none.
fn levels_in_field(value: &str) -> Vec<u8> {
    value
        .split('|')
        .map(|group| group.split('[').next().unwrap_or(group))
        .filter_map(|group| group.rsplit_once('='))
        .filter_map(|(_, level)| level.trim().parse::<u8>().ok())
        .collect()
}

/// Minimum spell level across the record's `CLASSES:` and `DOMAINS:` tokens
/// combined -- the `rules_tables::acg::spell_list` precedent. `None` when
/// neither token yields a parseable level (a genuine corpus gap, never
/// fabricated here -- it lands `text-complete`, not `ingested-magnitude`,
/// via `classify()`'s existing `Some(false)` branch).
fn min_level(classes: Option<&str>, domains: Option<&str>) -> Option<u8> {
    let mut all: Vec<u8> = Vec::new();
    if let Some(c) = classes {
        all.extend(levels_in_field(c));
    }
    if let Some(d) = domains {
        all.extend(levels_in_field(d));
    }
    all.into_iter().min()
}

/// `DOMAINS:` is not one of `pcgen_import::lst_parser::spell`'s known tags,
/// so it is read directly off the raw row here, the same first-match-wins
/// convention that parser applies to every other tag.
fn domains_field(raw_line: &str) -> Option<String> {
    raw_line
        .split('\t')
        .skip(1)
        .find_map(|col| col.trim().strip_prefix("DOMAINS:"))
        .map(str::to_string)
}

/// The row's own `(key, value)` tab tokens, split on the first `:` --
/// exactly what `pi_screening::declared_product_identity` consumes. Built
/// from the raw line directly (not from `LstSpellRecord`, which only
/// surfaces the fields the shared parser names) so `NAMEISPI:`/`DESCISPI:`
/// -- fields no existing typed accessor exposes -- are never silently
/// unreachable by this screen.
fn raw_row_tokens(raw_line: &str) -> Vec<(String, String)> {
    raw_line
        .split('\t')
        .skip(1)
        .filter_map(|col| col.trim().split_once(':'))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SpellEntry {
    key: String,
    school: Option<String>,
    level: Option<u8>,
    description: Option<String>,
}

enum PiOutcome {
    Clean(SpellEntry),
    NamePiDropped(String),
}

/// Screens one record with BOTH SD-30 invocation contracts -- the blacklist
/// sweep (`classify_field`) and the declared-PI reader
/// (`declared_product_identity`) -- against BOTH the name and the
/// description. A name hit (from either contract) drops the record; a
/// description hit redacts the description field only.
fn pi_screen(raw_line: &str, name: &str, description: Option<&str>) -> PiOutcome {
    let declared = declared_product_identity(raw_row_tokens(raw_line));

    let (name_license, ..) = classify_field("name", name);
    let name_blacklisted = name_license != codex::rules_core::shape_b_v1::License::Ogl;
    if declared.name || name_blacklisted {
        return PiOutcome::NamePiDropped(name.to_string());
    }

    let (_, _, _, stored_description) =
        classify_optional_field_declared("description", description, declared.description);

    PiOutcome::Clean(SpellEntry {
        key: name.to_string(),
        school: None, // filled by caller, which also owns school-string normalization
        level: None,  // filled by caller
        description: stored_description,
    })
}

/// The corpus's raw `SCHOOL:` string, verbatim -- normalized to a
/// `Pf1SchoolId` variant name at codegen time so a school this book's own
/// table has never seen fails loudly rather than silently mapping to
/// `None`.
fn school_variant_name(raw: &str) -> Option<&'static str> {
    match raw {
        "Abjuration" => Some("Abjuration"),
        "Conjuration" => Some("Conjuration"),
        "Divination" => Some("Divination"),
        "Enchantment" => Some("Enchantment"),
        "Evocation" => Some("Evocation"),
        "Illusion" => Some("Illusion"),
        "Necromancy" => Some("Necromancy"),
        "Transmutation" => Some("Transmutation"),
        "Universal" => Some("Universal"),
        _ => None,
    }
}

fn escape_rust_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn render_entry(e: &SpellEntry) -> String {
    let school = match &e.school {
        Some(s) => format!("Some(Pf1SchoolId::{s})"),
        None => "None".to_string(),
    };
    let level = match e.level {
        Some(n) => format!("Some({n})"),
        None => "None".to_string(),
    };
    let description = match &e.description {
        Some(d) => format!("Some(\"{}\")", escape_rust_string(d)),
        None => "None".to_string(),
    };
    format!(
        "    SpellListEntry {{ key: \"{}\", school: {school}, level: {level}, description: {description} }},",
        escape_rust_string(&e.key)
    )
}

fn build_module_source(display_name: &str, lst_rel: &str, entries: &[SpellEntry]) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "//! {display_name} shared spell list.\n\
         //!\n\
         //! Generated by `src/bin/ingest_inner_sea_setting_spells.rs` from the\n\
         //! real `{lst_rel}` corpus (SD-32 Gate 0 book-onboarding precondition,\n\
         //! `gate-0-book-onboarding-precondition`, AT-32-G0-003). Record coverage:\n\
         //! every real, active (non-`.MOD`, non-`.COPY=`) base spell declaration.\n\
         //!\n\
         //! `level` is the minimum level across the record's `CLASSES:`/`DOMAINS:`\n\
         //! token(s) (the `rules_tables::acg::spell_list` precedent), `None` for the\n\
         //! rare record that states neither (never fabricated -- these land\n\
         //! `text-complete`, not `ingested-magnitude`, via\n\
         //! `v06_work_inventory::classify`'s existing `Some(false)` branch).\n\
         //!\n\
         //! `school`/`description` are `Option` (mirroring\n\
         //! `rules_tables::adventurers_guide::spell_list`'s own shape) because a\n\
         //! minority of records carry neither token of their own on this book's\n\
         //! base row.\n\n",
    ));
    out.push_str(
        "/// The full 9-school PF1 spell-school enum, mirroring every other book's\n\
         /// own copy exactly.\n\
         #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]\n\
         pub enum Pf1SchoolId {\n\
         \x20   Abjuration,\n\
         \x20   Conjuration,\n\
         \x20   Divination,\n\
         \x20   Enchantment,\n\
         \x20   Evocation,\n\
         \x20   Illusion,\n\
         \x20   Necromancy,\n\
         \x20   Transmutation,\n\
         \x20   Universal,\n\
         }\n\n\
         impl Pf1SchoolId {\n\
         \x20   pub fn from_corpus_str(raw: &str) -> Option<Self> {\n\
         \x20       match raw {\n\
         \x20           \"Abjuration\" => Some(Pf1SchoolId::Abjuration),\n\
         \x20           \"Conjuration\" => Some(Pf1SchoolId::Conjuration),\n\
         \x20           \"Divination\" => Some(Pf1SchoolId::Divination),\n\
         \x20           \"Enchantment\" => Some(Pf1SchoolId::Enchantment),\n\
         \x20           \"Evocation\" => Some(Pf1SchoolId::Evocation),\n\
         \x20           \"Illusion\" => Some(Pf1SchoolId::Illusion),\n\
         \x20           \"Necromancy\" => Some(Pf1SchoolId::Necromancy),\n\
         \x20           \"Transmutation\" => Some(Pf1SchoolId::Transmutation),\n\
         \x20           \"Universal\" => Some(Pf1SchoolId::Universal),\n\
         \x20           _ => None,\n\
         \x20       }\n\
         \x20   }\n\
         }\n\n",
    );
    out.push_str(
        "#[derive(Debug, Clone, PartialEq, Eq)]\n\
         pub struct SpellListEntry {\n\
         \x20   pub key: &'static str,\n\
         \x20   pub school: Option<Pf1SchoolId>,\n\
         \x20   pub level: Option<u8>,\n\
         \x20   pub description: Option<&'static str>,\n\
         }\n\n",
    );
    out.push_str("pub const SPELL_LIST: &[SpellListEntry] = &[\n");
    for e in entries {
        out.push_str(&render_entry(e));
        out.push('\n');
    }
    out.push_str("];\n");
    out
}

fn ingest_one_book(data_root: &std::path::Path, book: &BookInput) {
    let lst_path = data_root.join(book.lst_rel);
    let parsed = parse_lst_spell_file(&lst_path)
        .unwrap_or_else(|e| panic!("failed to parse {lst_path:?}: {e:?}"));

    let raw_text = fs::read_to_string(&lst_path).unwrap_or_else(|e| panic!("read {lst_path:?}: {e}"));
    let raw_lines: Vec<&str> = raw_text.split('\n').collect();

    let mut entries: Vec<SpellEntry> = Vec::new();
    let mut dropped_pi: Vec<String> = Vec::new();
    let mut school_unrecognized: Vec<String> = Vec::new();
    let mut no_level: Vec<String> = Vec::new();
    // First-declaration-wins within a book, the same policy
    // `spell_resolver::spell_catalog_rows()` applies across books (SD-31
    // wave-24 integration cycle) -- `isf_spells.lst` genuinely restates
    // "Curse of Disgust (Besmaran)" as a second, non-`.MOD` base
    // declaration later in the same file (a fuller reprint of the same
    // spell, not a distinct one), which `is_base_declaration` alone would
    // not catch. Deduping here, at the source, means the emitted table
    // never asserts a key count the cross-book dedup would silently
    // shrink downstream.
    let mut seen_keys: std::collections::HashSet<String> = std::collections::HashSet::new();

    for record in &parsed.records {
        let LstSpellRecord { name, .. } = record;
        if !is_base_declaration(name) {
            continue;
        }
        if !seen_keys.insert(name.clone()) {
            continue;
        }
        let raw_line = raw_lines.get(record.line_number - 1).copied().unwrap_or("");
        let domains = domains_field(raw_line);
        let level = min_level(record.classes.as_deref(), domains.as_deref());
        if level.is_none() {
            no_level.push(name.clone());
        }

        match pi_screen(raw_line, name, record.description.as_deref()) {
            PiOutcome::NamePiDropped(n) => dropped_pi.push(n),
            PiOutcome::Clean(mut entry) => {
                entry.level = level;
                entry.school = match &record.school {
                    Some(raw_school) => match school_variant_name(raw_school) {
                        Some(v) => Some(v.to_string()),
                        None => {
                            school_unrecognized.push(format!("{name}: {raw_school:?}"));
                            None
                        }
                    },
                    None => None,
                };
                entries.push(entry);
            }
        }
    }

    eprintln!(
        "ingest_inner_sea_setting_spells [{}]: {} base declarations, {} PI-dropped, {} no-level (real gap, not fabricated), {} school-unrecognized",
        book.display_name,
        entries.len(),
        dropped_pi.len(),
        no_level.len(),
        school_unrecognized.len(),
    );
    if !dropped_pi.is_empty() {
        eprintln!("  PI-dropped (name declared or blacklisted): {dropped_pi:?}");
    }
    if !no_level.is_empty() {
        eprintln!("  No CLASSES:/DOMAINS: level (kept, level=None -> text-complete): {no_level:?}");
    }
    if !school_unrecognized.is_empty() {
        eprintln!("  Unrecognized SCHOOL: string (kept, school=None): {school_unrecognized:?}");
    }

    let source = build_module_source(book.display_name, book.lst_rel, &entries);
    fs::write(book.out_path, source).unwrap_or_else(|e| panic!("write {}: {e}", book.out_path));
    eprintln!("  wrote {} ({} entries)", book.out_path, entries.len());
}

fn main() {
    let data_root = pcgen_data_root();
    for book in BOOKS {
        ingest_one_book(&data_root, book);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_base_declaration_excludes_mod_and_copy_rows() {
        assert!(is_base_declaration("Fireball"));
        assert!(!is_base_declaration("Fireball.MOD"));
        assert!(!is_base_declaration("Fireball.COPY=Fireball (Greater)"));
    }

    #[test]
    fn min_level_takes_the_minimum_across_multiple_classes_groups() {
        assert_eq!(
            min_level(Some("Alchemist=4|Druid,Sorcerer,Witch,Wizard=6"), None),
            Some(4)
        );
    }

    #[test]
    fn min_level_reads_domains_when_classes_is_absent() {
        assert_eq!(min_level(None, Some("Scalykind=5")), Some(5));
    }

    #[test]
    fn min_level_combines_classes_and_domains_taking_the_true_minimum() {
        assert_eq!(min_level(Some("Wizard=9"), Some("Aquatic=3")), Some(3));
    }

    #[test]
    fn min_level_is_none_when_neither_token_is_present() {
        assert_eq!(min_level(None, None), None);
    }

    #[test]
    fn levels_in_field_parses_every_pipe_separated_group() {
        assert_eq!(levels_in_field("A=1|B,C=2|D=3"), vec![1, 2, 3]);
    }

    #[test]
    fn levels_in_field_strips_a_bracketed_preskill_clause_before_finding_the_level() {
        assert_eq!(
            levels_in_field("Bard=3[PRESKILL:1,Perform (String Instruments)=7,Perform (Wind Instruments)=7]"),
            vec![3]
        );
    }

    #[test]
    fn levels_in_field_strips_a_bracketed_predeity_clause_before_finding_the_level() {
        // Real `istem_spells.lst` row shape: `Bard=6[PREDEITY:1,Calistria]`.
        assert_eq!(levels_in_field("Bard=6[PREDEITY:1,Calistria]"), vec![6]);
    }

    #[test]
    fn pi_screen_drops_a_record_whose_row_declares_nameispi_yes() {
        let raw = "Geb's Hammer\tNAMEISPI:YES\tCLASSES:Sorcerer,Witch,Wizard=4\tSCHOOL:Necromancy\tDESC:text";
        let outcome = pi_screen(raw, "Geb's Hammer", Some("text"));
        assert!(matches!(outcome, PiOutcome::NamePiDropped(_)));
    }

    #[test]
    fn pi_screen_redacts_a_description_whose_row_declares_descispi_yes() {
        let raw = "Aroden's Spellbane\tDESCISPI:YES\tCLASSES:Sorcerer,Wizard=9\tSCHOOL:Abjuration\tDESC:secret lore";
        let outcome = pi_screen(raw, "Aroden's Spellbane", Some("secret lore"));
        match outcome {
            PiOutcome::Clean(entry) => {
                assert_eq!(entry.key, "Aroden's Spellbane");
                assert_ne!(entry.description.as_deref(), Some("secret lore"));
            }
            PiOutcome::NamePiDropped(_) => panic!("a DESCISPI-only declaration must not drop the record"),
        }
    }

    #[test]
    fn pi_screen_passes_a_clean_record_through_unredacted() {
        let raw = "Bladed Dash\tCLASSES:Bard,Magus=2\tSCHOOL:Transmutation\tDESC:you dash";
        let outcome = pi_screen(raw, "Bladed Dash", Some("you dash"));
        match outcome {
            PiOutcome::Clean(entry) => {
                assert_eq!(entry.description.as_deref(), Some("you dash"));
            }
            PiOutcome::NamePiDropped(_) => panic!("a clean record must not be dropped"),
        }
    }

    #[test]
    fn school_variant_name_recognizes_all_nine_schools() {
        for (raw, expected) in [
            ("Abjuration", "Abjuration"),
            ("Conjuration", "Conjuration"),
            ("Divination", "Divination"),
            ("Enchantment", "Enchantment"),
            ("Evocation", "Evocation"),
            ("Illusion", "Illusion"),
            ("Necromancy", "Necromancy"),
            ("Transmutation", "Transmutation"),
            ("Universal", "Universal"),
        ] {
            assert_eq!(school_variant_name(raw), Some(expected));
        }
        assert_eq!(school_variant_name("NotASchool"), None);
    }

    #[test]
    fn domains_field_is_none_when_absent() {
        let raw = "Bereave\tTYPE:Arcane.Divine.Psychic\tCLASSES:Bard,Mesmerist=3\tSCHOOL:Enchantment";
        assert_eq!(domains_field(raw), None);
    }

    #[test]
    fn books_list_names_exactly_the_three_spell_bearing_gate_0_books() {
        let names: Vec<&str> = BOOKS.iter().map(|b| b.display_name).collect();
        assert_eq!(names, vec!["Inner Sea Faiths", "Inner Sea Magic", "Inner Sea Temples"]);
    }
}
