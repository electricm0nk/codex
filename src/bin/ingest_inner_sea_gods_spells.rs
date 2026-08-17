//! Ingests Inner Sea Gods' spell catalog into the engine's spell-catalog
//! capability (`SD31-E6-F10-001`).
//!
//! **The finding this closes.** `spell_resolver::spell_catalog_rows()`
//! chains eight books (CRB/APG/ACG/ARG/UI/UM/OA/UC) before this cycle;
//! every OTHER spell-bearing book's units are structurally `not-ingested`
//! (or, for a book with no compiled `RuleSetId` at all, `not-started`) no
//! matter how much ingest work runs against them, because `classify()`'s
//! `Kind::Spell` arm never consults anything but that one compiled table.
//! `inner_sea_gods` has a compiled `RuleSetId::Isg` already (SD-29's monster
//! lane) and a real, dedicated `isg_spells.lst` — 96 of the kind's units are
//! `not-ingested` against this book today (re-derived,
//! `docs/work-inventory.json`). This binary builds the missing ninth-book
//! slice — `rules_tables::inner_sea_gods::spell_list::SPELL_LIST` — from the
//! real, oracle-pinned `isg_spells.lst` corpus.
//!
//! **Reuses the tested general-purpose LST spell parser**
//! (`pcgen_import::lst_parser::spell::parse_lst_spell_file`), not a
//! reimplementation. This binary adds exactly what
//! `ingest_ultimate_magic_spells.rs` (the direct precedent this file
//! mirrors line-for-line where the shape is identical) already established:
//! `.MOD` exclusion (the same convention every existing per-book
//! `spell_list.rs` follows — `isg_spells.lst` has 0 `.COPY=` rows, 4 `.MOD`
//! rows, re-derived: `grep -c '\.COPY=\|\.MOD' isg_spells.lst`) and
//! `CLASSES:`/`DOMAINS:` level derivation (`isg_spells.lst` carries 0
//! `DOMAINS:` tokens — re-derived, `grep -c DOMAINS: isg_spells.lst` → 0 —
//! so `domains_field` is kept only for parity with the shared shape and
//! never fires here; left in rather than dropped so a future re-run against
//! a corrected/expanded corpus is not silently blind to one).
//!
//! **PI screening, both SD-30 invocation contracts, per record, before
//! writing anything** (`SD-30 decisions.md §52.3` blacklist sweep, `§53.5`
//! declared-PI reader) — the **NAME** is screened by both contracts, not
//! only the description, per this program's own safety-critical mandate.
//! A name cannot be redacted (`pi_screening::DeclaredProductIdentity`'s own
//! doc comment: "the only way not to publish it is not to publish the
//! row"), so a name-PI hit drops the record entirely; a description-PI hit
//! redacts the description field only.
//!
//! `isg_spells.lst` carries zero `NAMEISPI:`/`DESCISPI:` tokens at all
//! (re-derived: `grep -c "NAMEISPI\|DESCISPI" isg_spells.lst` → 0), matching
//! this book's own `LICENSE.json` note ("zero rows of any of the four .lst
//! files declare NAMEISPI:YES") — this run is not expected to drop or
//! redact anything, but the declared-PI reader still runs on every record's
//! own raw tokens, not skipped on that assumption. Per-book PI-gate
//! citation: `data/corpus/inner_sea_gods/LICENSE.json`,
//! `classified_by_cycle: SD29-E5-F2-010` — `epic-3-pi-gate` is `COMPLETE`
//! package-wide (`SD-31 kanban.md` "Cross-SD gate discipline").
//!
//! Run with `cargo run --locked --bin ingest_inner_sea_gods_spells`.
//! `PCGEN_CORPUS_ROOT` overrides the default `$HOME/workspace/repos/pcgen/data`.
//! Writes `src/rules_core/rules_tables/inner_sea_gods/spell_list.rs`.

use std::env;
use std::fs;
use std::path::PathBuf;

use codex::pcgen_import::lst_parser::spell::{parse_lst_spell_file, LstSpellRecord};
use codex::rules_core::pi_screening::{
    classify_field, classify_optional_field_declared, declared_product_identity,
};

const ISG_SPELLS_REL: &str =
    "pathfinder/paizo/campaign_setting/inner_sea_gods/isg_spells.lst";
const OUT_PATH: &str = "src/rules_core/rules_tables/inner_sea_gods/spell_list.rs";

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
/// Strips a trailing `[...]` PRESKILL/condition clause before looking for
/// the level's own `=` (the `ingest_ultimate_magic_spells.rs` precedent's
/// own fix for the same corpus shape; `isg_spells.lst` carries no bracketed
/// clause today -- re-derived, `grep -c '\[' isg_spells.lst` -- kept for
/// parity and safety against a future corpus update).
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
/// fabricated here).
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

/// The record's real identity when its row declares an explicit `KEY:`
/// token, distinct from column-0's display name -- `pcgen_import::lst_parser::
/// spell::LstSpellRecord` carries no `key` field (no book chained into this
/// catalog before Inner Sea Gods used `KEY:` at all: `grep -c '\tKEY:'
/// um_spells.lst/oa_spells.lst/uc_spells.lst` -> `0 0 0`, `isg_spells.lst`
/// -> `65`), so it is read directly off the raw row here.
///
/// **Found live, not hypothesized:** `v06_corpus_trap_report --audit`'s
/// `key-differs-from-name` check caught 2 real records this ingest first
/// shipped under the wrong identity before this function existed --
/// `"Lighten Object, Mass"` (comma-separated display text) whose row
/// declares `KEY:Lighten Object (Mass)`, and `"Shield of the Dawnflower,
/// Greater"` similarly. Both are archetype-qualified variant spells sharing
/// a base spell's display-name PREFIX but not its `KEY:` -- shipping them
/// under the display text would have let a `corpus_key` lookup silently
/// resolve to the wrong record. Of the book's 65 `KEY:`-bearing rows, only
/// these 2 actually differ from their own display name; the other 63 simply
/// restate it, so this fix moves exactly 2 identities, not 65.
fn key_field(raw_line: &str) -> Option<String> {
    raw_line
        .split('\t')
        .skip(1)
        .find_map(|col| col.trim().strip_prefix("KEY:"))
        .map(str::to_string)
}

/// The row's own `(key, value)` tab tokens, split on the first `:` --
/// exactly what `pi_screening::declared_product_identity` consumes. Built
/// from the raw line directly (not from `LstSpellRecord`, which only
/// surfaces the fields the shared parser names) so `NAMEISPI:`/`DESCISPI:`
/// are never silently unreachable by this screen.
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
/// `None`. Re-derived: `isg_spells.lst` uses only the 9 standard schools
/// (`python3 -c "import re; print(sorted(set(re.findall(r'(?:\t|^)SCHOOL:(\w+)', open('isg_spells.lst').read(), re.MULTILINE))))"`).
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

fn build_module_source(entries: &[SpellEntry]) -> String {
    let mut out = String::new();
    out.push_str(
        "//! Inner Sea Gods (ISG) shared spell list.\n\
         //!\n\
         //! Generated by `src/bin/ingest_inner_sea_gods_spells.rs` from the real\n\
         //! `isg_spells.lst` corpus (SD31-E6-F10-001). Record coverage: every real,\n\
         //! active (non-`.MOD`, non-`.COPY=`) base spell declaration.\n\
         //!\n\
         //! `level` is the minimum level across the record's `CLASSES:`/`DOMAINS:`\n\
         //! token(s) (the `rules_tables::acg::spell_list` precedent), `None` for the\n\
         //! rare record that states neither (never fabricated -- these land\n\
         //! `text-complete`, not `ingested-magnitude`, via\n\
         //! `v06_work_inventory::classify`'s existing `Some(false)` branch).\n\
         //!\n\
         //! `school`/`description` are `Option` (mirroring `rules_tables::apg::spell_list`'s\n\
         //! own shape) because a small minority of records carry neither token of\n\
         //! their own on this book's base row.\n\n",
    );
    out.push_str(
        "/// The full 9-school PF1 spell-school enum, mirroring every other book's\n\
         /// own copy exactly (`rules_tables::ultimate_magic::spell_list::Pf1SchoolId`).\n\
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

fn main() {
    let data_root = pcgen_data_root();
    let lst_path = data_root.join(ISG_SPELLS_REL);
    let parsed = parse_lst_spell_file(&lst_path)
        .unwrap_or_else(|e| panic!("failed to parse {lst_path:?}: {e:?}"));

    let raw_text = fs::read_to_string(&lst_path).unwrap_or_else(|e| panic!("read {lst_path:?}: {e}"));
    let raw_lines: Vec<&str> = raw_text.split('\n').collect();

    let mut entries: Vec<SpellEntry> = Vec::new();
    let mut dropped_pi: Vec<String> = Vec::new();
    let mut school_unrecognized: Vec<String> = Vec::new();
    let mut no_level: Vec<String> = Vec::new();

    for record in &parsed.records {
        let LstSpellRecord { name, .. } = record;
        if !is_base_declaration(name) {
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
                // A declared `KEY:` token is this record's real identity,
                // not its display name -- see `key_field`'s own doc
                // comment for the 2 real records this reorders.
                if let Some(real_key) = key_field(raw_line) {
                    entry.key = real_key;
                }
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
        "ingest_inner_sea_gods_spells: {} base declarations, {} PI-dropped, {} no-level (real gap, not fabricated), {} school-unrecognized",
        entries.len(),
        dropped_pi.len(),
        no_level.len(),
        school_unrecognized.len(),
    );
    if !dropped_pi.is_empty() {
        eprintln!("PI-dropped (name declared or blacklisted): {dropped_pi:?}");
    }
    if !no_level.is_empty() {
        eprintln!("No CLASSES:/DOMAINS: level (kept, level=None -> text-complete): {no_level:?}");
    }
    if !school_unrecognized.is_empty() {
        eprintln!("Unrecognized SCHOOL: string (kept, school=None): {school_unrecognized:?}");
    }

    let source = build_module_source(&entries);
    fs::write(OUT_PATH, source).unwrap_or_else(|e| panic!("write {OUT_PATH}: {e}"));
    eprintln!("wrote {OUT_PATH} ({} entries)", entries.len());
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
            min_level(Some("Cleric=1|Inquisitor,Paladin=2"), None),
            Some(1)
        );
    }

    #[test]
    fn min_level_reads_domains_when_classes_is_absent() {
        assert_eq!(min_level(None, Some("Aquatic=7")), Some(7));
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
    fn domains_field_reads_the_raw_row_directly() {
        let raw = "Animal Shapes (Aquatic Creatures Only)\t\tTYPE:Divine\t\tDOMAINS:Aquatic=7\t\tSCHOOL:Transmutation";
        assert_eq!(domains_field(raw), Some("Aquatic=7".to_string()));
    }

    #[test]
    fn domains_field_is_none_when_absent() {
        let raw = "Fireball\tCLASSES:Wizard=3\tSCHOOL:Evocation";
        assert_eq!(domains_field(raw), None);
    }

    /// The real corpus row (`isg_spells.lst:41`): a declared `KEY:` that
    /// differs from the display name is this record's true identity.
    #[test]
    fn key_field_reads_a_declared_key_distinct_from_the_display_name() {
        let raw = "Lighten Object, Mass\tKEY:Lighten Object (Mass)\tCLASSES:Wizard=5\tSCHOOL:Transmutation";
        assert_eq!(key_field(raw), Some("Lighten Object (Mass)".to_string()));
    }

    #[test]
    fn key_field_is_none_when_no_key_token_is_declared() {
        let raw = "Fireball\tCLASSES:Wizard=3\tSCHOOL:Evocation";
        assert_eq!(key_field(raw), None);
    }

    #[test]
    fn pi_screen_drops_a_record_whose_row_declares_nameispi_yes() {
        let raw = "Secret Name\tNAMEISPI:YES\tCLASSES:Wizard=1\tSCHOOL:Evocation\tDESC:text";
        let outcome = pi_screen(raw, "Secret Name", Some("text"));
        assert!(matches!(outcome, PiOutcome::NamePiDropped(_)));
    }

    #[test]
    fn pi_screen_redacts_a_description_whose_row_declares_descispi_yes() {
        let raw = "Ordinary Spell\tDESCISPI:YES\tCLASSES:Wizard=1\tSCHOOL:Evocation\tDESC:secret lore";
        let outcome = pi_screen(raw, "Ordinary Spell", Some("secret lore"));
        match outcome {
            PiOutcome::Clean(entry) => {
                assert_eq!(entry.key, "Ordinary Spell");
                assert_ne!(entry.description.as_deref(), Some("secret lore"));
            }
            PiOutcome::NamePiDropped(_) => panic!("a DESCISPI-only declaration must not drop the record"),
        }
    }

    #[test]
    fn pi_screen_passes_a_clean_record_through_unredacted() {
        let raw = "Spawn Calling\tCLASSES:Cleric=9\tSCHOOL:Conjuration\tDESC:you conjure spawn";
        let outcome = pi_screen(raw, "Spawn Calling", Some("you conjure spawn"));
        match outcome {
            PiOutcome::Clean(entry) => {
                assert_eq!(entry.description.as_deref(), Some("you conjure spawn"));
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
}
