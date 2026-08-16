//! Ingests Ultimate Magic's spell catalog into the engine's spell-catalog
//! capability -- this cycle's primary deliverable (`SD31-E6-F2-002`).
//!
//! **The finding this closes.** `spell_resolver::spell_catalog_rows()`
//! chains exactly five books (CRB/APG/ACG/ARG/UI); every OTHER spell-bearing
//! book's units are structurally `not-ingested` no matter how much ingest
//! work runs against them, because `classify()`'s `Kind::Spell` arm never
//! consults anything but that one compiled table. Ultimate Magic alone
//! carries 291 such units. This binary builds the missing sixth-book slice
//! -- `rules_tables::ultimate_magic::spell_list::SPELL_LIST` -- from the
//! real, already-PI-screened `um_spells.lst` corpus (this book's
//! `data/corpus/ultimate_magic/LICENSE.json` already covers it at book
//! level, `classified_by_cycle: SD29-E7-F2-010`; `epic-3-pi-gate` is
//! `COMPLETE` package-wide per `kanban.md` "Cross-SD gate discipline").
//!
//! **Reuses the tested general-purpose LST spell parser**
//! (`pcgen_import::lst_parser::spell::parse_lst_spell_file`), not a
//! reimplementation -- that parser's own doc comment names `um_spells.lst`
//! explicitly as one of its supported shapes. This binary adds exactly the
//! two things that parser does not do: `.MOD`/`.COPY=` exclusion (the same
//! convention every existing per-book `spell_list.rs` follows) and
//! `CLASSES:`/`DOMAINS:` level derivation (the `rules_tables::acg::spell_list`
//! precedent: "Minimum spell level across the real record's `CLASSES:`/
//! `DOMAINS:` tag").
//!
//! **PI screening, both SD-30 invocation contracts, per record, before
//! writing anything** (`SD-30 decisions.md §52.3` blacklist sweep,
//! `§53.5` declared-PI reader) -- and unlike `cache_gen::ultimate_equipment`
//! (the confirmed hole named in this cycle's own dispatch), the **NAME** is
//! screened by both contracts, not only the description. A name cannot be
//! redacted (per `pi_screening::DeclaredProductIdentity`'s own doc comment:
//! "the only way not to publish it is not to publish the row"), so a
//! name-PI hit drops the record entirely rather than shipping it redacted.
//! A description-PI hit redacts the description field only.
//!
//! `um_spells.lst` carries zero `NAMEISPI:`/`DESCISPI:` tokens at all
//! (re-derived: `grep -c "NAMEISPI\|DESCISPI" um_spells.lst` -> 0), so this
//! run is not expected to drop or redact anything -- the declared-PI reader
//! still runs on every record's own raw tokens, not skipped on that
//! assumption.
//!
//! Run with `cargo run --locked --bin ingest_ultimate_magic_spells`.
//! `PCGEN_CORPUS_ROOT` overrides the default `$HOME/workspace/repos/pcgen/data`.
//! Writes `src/rules_core/rules_tables/ultimate_magic/spell_list.rs`.

use std::env;
use std::fs;
use std::path::PathBuf;

use codex::pcgen_import::lst_parser::spell::{parse_lst_spell_file, LstSpellRecord};
use codex::rules_core::pi_screening::{
    classify_field, classify_optional_field_declared, declared_product_identity,
};

const UM_SPELLS_REL: &str =
    "pathfinder/paizo/roleplaying_game/ultimate_magic/um_spells.lst";
const OUT_PATH: &str = "src/rules_core/rules_tables/ultimate_magic/spell_list.rs";

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
/// **Strips a trailing `[...]` PRESKILL/condition clause before looking for
/// the level's own `=`.** Found the hard way, via `reach_gate.rs`'s own
/// `bare_records_are_exactly_the_recorded_findings` gate: 35 UM `CLASSES:`
/// values carry a bracketed sub-condition with its own embedded `=`
/// (`Bard=3[PRESKILL:1,Perform (String Instruments)=7,Perform (Wind
/// Instruments)=7]`, the 15 `Masterpiece` bard-performance records among
/// them) -- an unqualified `rsplit_once('=')` grabbed the LAST `=` in the
/// whole group (`=7]`, inside the bracket), which fails to parse as a `u8`
/// and silently discarded a REAL level (3, here) as though the record
/// carried none. Every group is truncated to the text before its first `[`
/// (there are none in this corpus with a `|` inside a bracket, confirmed:
/// `grep -oP 'CLASSES:[^\t]*\[[^]]*\]' um_spells.lst | grep '|'` -> empty)
/// before the `=` search runs.
fn levels_in_field(value: &str) -> Vec<u8> {
    value
        .split('|')
        .map(|group| group.split('[').next().unwrap_or(group))
        .filter_map(|group| group.rsplit_once('='))
        .filter_map(|(_, level)| level.trim().parse::<u8>().ok())
        .collect()
}

/// Minimum spell level across the record's `CLASSES:` and `DOMAINS:` tokens
/// combined -- the `rules_tables::acg::spell_list` precedent ("Minimum spell
/// level across the real record's `CLASSES:`/`DOMAINS:` tag"). `None` when
/// neither token yields a parseable level (a genuine corpus gap, e.g.
/// `Restore Eidolon`, which carries neither and is never fabricated a level
/// here -- it lands `text-complete`, not `ingested-magnitude`, via
/// `classify()`'s existing `Some(false)` branch).
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

/// `DOMAINS:` is not one of `pcgen_import::lst_parser::spell`'s known tags
/// (that parser's own `KNOWN_TAGS` list omits it -- confirmed by direct
/// read), so it is read directly off the raw row here, the same
/// first-match-wins convention that parser applies to every other tag.
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
/// description, per this cycle's own safety-critical mandate. A name hit
/// (from either contract) drops the record; a description hit redacts the
/// description field only.
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
/// table has never seen (there are none today; re-derived below) fails
/// loudly rather than silently mapping to `None`.
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
        "//! Ultimate Magic (UM) shared spell list.\n\
         //!\n\
         //! Generated by `src/bin/ingest_ultimate_magic_spells.rs` from the real\n\
         //! `um_spells.lst` corpus (SD31-E6-F2-002). Record coverage: every real,\n\
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
         /// own copy exactly (`rules_tables::advanced_race_guide::spell_list::Pf1SchoolId`).\n\
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
    let lst_path = data_root.join(UM_SPELLS_REL);
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
        "ingest_ultimate_magic_spells: {} base declarations, {} PI-dropped, {} no-level (real gap, not fabricated), {} school-unrecognized",
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
            min_level(Some("Alchemist=4|Druid,Sorcerer,Witch,Wizard=6"), None),
            Some(4)
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

    /// The real corpus row (`um_spells.lst`, "Masterpiece (At the Heart of
    /// It All)"): a `[PRESKILL:...]` clause with its own embedded `=`s must
    /// not shadow the class's own level.
    #[test]
    fn levels_in_field_strips_a_bracketed_preskill_clause_before_finding_the_level() {
        assert_eq!(
            levels_in_field("Bard=3[PRESKILL:1,Perform (String Instruments)=7,Perform (Wind Instruments)=7]"),
            vec![3]
        );
    }

    #[test]
    fn levels_in_field_handles_a_bracketed_clause_among_several_pipe_groups() {
        assert_eq!(levels_in_field("Bard=2[PRESKILL:1,Perform (Dance)=5]|Wizard=4"), vec![2, 4]);
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
        let raw = "Wall of Silver\tCLASSES:Wizard=6\tSCHOOL:Conjuration\tDESC:you conjure a wall of silver";
        let outcome = pi_screen(raw, "Wall of Silver", Some("you conjure a wall of silver"));
        match outcome {
            PiOutcome::Clean(entry) => {
                assert_eq!(entry.description.as_deref(), Some("you conjure a wall of silver"));
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
