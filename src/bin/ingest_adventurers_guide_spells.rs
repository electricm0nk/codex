//! Ingests Adventurer's Guide's spell catalog into the engine's
//! spell-catalog capability -- SD-31 wave-29's `lane5-book-onboard` lane.
//!
//! **The finding this closes.** `spell_resolver::spell_catalog_rows()`
//! chained eleven books before this cycle (CRB/APG/ACG/ARG/UI/UM/OA/UC/
//! ISG/UW/B6); `adventurers_guide` had no `RuleSetId` at all, so
//! `v06_work_inventory::classify`'s book-level gate (`engine_book_for` ->
//! `rule_set_for` -> `None`) short-circuited EVERY one of this book's
//! units -- not only its spells but its 699 `class_feature` units too
//! (`docs/release/SD-31-corpus-closure-grind/artifacts/THE-BOX.md` §2.1's
//! G4 finding) -- to `not-started`/`no_compiled_rule_set_for_book` before
//! any per-kind arm ever ran. This binary builds the missing twelfth-book
//! slice -- `rules_tables::adventurers_guide::spell_list::SPELL_LIST` --
//! from the real `ag_spells.lst` corpus, the book's first compiled rule
//! set of any kind (the same shape `RuleSetId::Oa`/`RuleSetId::Mythic`
//! established).
//!
//! **Reuses the tested general-purpose LST spell parser**
//! (`pcgen_import::lst_parser::spell::parse_lst_spell_file`), not a
//! reimplementation, and the identical `.MOD`/`.COPY=` exclusion and
//! `CLASSES:`/`DOMAINS:` level derivation every prior `ingest_<book>_spells`
//! binary (`ingest_ultimate_wilderness_spells.rs`, ...) already
//! established -- copied deliberately rather than re-derived, per this
//! program's own "fix the source, not the symptom" / no second
//! implementation discipline.
//!
//! `ag_spells.lst`: 59 lines, 2 comment/header rows, 57 non-comment rows, 8
//! `.MOD`/`.COPY=` delta rows, 49 base declarations -- re-derived at
//! dispatch, SD-31 wave-29.
//!
//! **PI screening, both SD-30 invocation contracts, per record, before
//! writing anything** (`SD-30 decisions.md §52.3` blacklist sweep, `§53.5`
//! declared-PI reader), matching every prior spell-lane cycle.
//! `ag_spells.lst` carries 4 `NAMEISPI:YES` and 14 `DESCISPI:YES`
//! declarations (`grep -oE '(NAMEISPI|DESCISPI):[A-Za-z]+'
//! .../adventurers_guide/ag_spells.lst | sort | uniq -c`, re-derived at
//! dispatch) -- real, non-hypothetical stakes for this run, unlike UW's
//! zero-declaration corpus.
//!
//! Run with `cargo run --locked --bin ingest_adventurers_guide_spells`.
//! `PCGEN_CORPUS_ROOT` overrides the default `$HOME/workspace/repos/pcgen/data`.
//! Writes `src/rules_core/rules_tables/adventurers_guide/spell_list.rs`.

use std::env;
use std::fs;
use std::path::PathBuf;

use codex::pcgen_import::lst_parser::spell::{parse_lst_spell_file, LstSpellRecord};
use codex::rules_core::pi_screening::{
    classify_field, classify_optional_field_declared, declared_product_identity,
};

const AG_SPELLS_REL: &str =
    "pathfinder/paizo/roleplaying_game/adventurers_guide/ag_spells.lst";
const OUT_PATH: &str = "src/rules_core/rules_tables/adventurers_guide/spell_list.rs";

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
/// the level's own `=` -- the same `ingest_ultimate_magic_spells.rs` fix,
/// carried forward: an unqualified `rsplit_once('=')` would grab the LAST
/// `=` in the whole group (inside a bracketed sub-condition), which fails
/// to parse as a `u8` and silently discards a real level as though the
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

fn build_module_source(entries: &[SpellEntry]) -> String {
    let mut out = String::new();
    out.push_str(
        "//! Adventurer's Guide (AG) shared spell list.\n\
         //!\n\
         //! Generated by `src/bin/ingest_adventurers_guide_spells.rs` from the\n\
         //! real `ag_spells.lst` corpus (SD-31 wave-29, `lane5-book-onboard`\n\
         //! lane). Record coverage: every real, active (non-`.MOD`, non-`.COPY=`)\n\
         //! base spell declaration.\n\
         //!\n\
         //! `level` is the minimum level across the record's `CLASSES:`/`DOMAINS:`\n\
         //! token(s) (the `rules_tables::acg::spell_list` precedent), `None` for the\n\
         //! rare record that states neither (never fabricated -- these land\n\
         //! `text-complete`, not `ingested-magnitude`, via\n\
         //! `v06_work_inventory::classify`'s existing `Some(false)` branch).\n\
         //!\n\
         //! `school`/`description` are `Option` (mirroring\n\
         //! `rules_tables::ultimate_wilderness::spell_list`'s own shape) because a\n\
         //! small minority of records carry neither token of their own on this\n\
         //! book's base row.\n\n",
    );
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

fn main() {
    let data_root = pcgen_data_root();
    let lst_path = data_root.join(AG_SPELLS_REL);
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
        "ingest_adventurers_guide_spells: {} base declarations, {} PI-dropped, {} no-level (real gap, not fabricated), {} school-unrecognized",
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

    /// The real corpus row: `Infernal Challenger`, `SUBSCHOOL:Calling` on a
    /// `SCHOOL:Conjuration` base -- proves the subschool token does not get
    /// confused for the school token by this binary's own parsing (a risk
    /// this book's corpus specifically exercises, unlike UW's).
    #[test]
    fn domains_field_is_none_when_absent_even_with_a_subschool_token() {
        let raw = "Infernal Challenger\t\tTYPE:Arcane.Divine\t\tCLASSES:Wizard=3\t\tSCHOOL:Conjuration\tSUBSCHOOL:Calling";
        assert_eq!(domains_field(raw), None);
    }

    #[test]
    fn pi_screen_drops_a_record_whose_row_declares_nameispi_yes() {
        let raw = "Tieldlara's Feint\tNAMEISPI:YES\tCLASSES:Bard=2\tSCHOOL:Enchantment\tDESC:text";
        let outcome = pi_screen(raw, "Tieldlara's Feint", Some("text"));
        assert!(matches!(outcome, PiOutcome::NamePiDropped(_)));
    }

    #[test]
    fn pi_screen_redacts_a_description_whose_row_declares_descispi_yes() {
        let raw = "Infernal Challenger\tDESCISPI:YES\tCLASSES:Wizard=3\tSCHOOL:Conjuration\tDESC:secret lore";
        let outcome = pi_screen(raw, "Infernal Challenger", Some("secret lore"));
        match outcome {
            PiOutcome::Clean(entry) => {
                assert_eq!(entry.key, "Infernal Challenger");
                assert_ne!(entry.description.as_deref(), Some("secret lore"));
            }
            PiOutcome::NamePiDropped(_) => panic!("a DESCISPI-only declaration must not drop the record"),
        }
    }

    #[test]
    fn pi_screen_passes_a_clean_record_through_unredacted() {
        let raw = "Bone Flense\tCLASSES:Wizard=6\tSCHOOL:Necromancy\tDESC:you flense the bones";
        let outcome = pi_screen(raw, "Bone Flense", Some("you flense the bones"));
        match outcome {
            PiOutcome::Clean(entry) => {
                assert_eq!(entry.description.as_deref(), Some("you flense the bones"));
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

    /// `SUBSCHOOL:Calling`/`SUBSCHOOL:Phantasm`/etc. are NOT valid
    /// `SCHOOL:` values -- this book's corpus specifically carries them
    /// (unlike UW's), so this pins that a mis-scoped grep for `SCHOOL:`
    /// could not silently mistake a subschool token for the school itself.
    #[test]
    fn school_variant_name_rejects_subschool_names() {
        assert_eq!(school_variant_name("Calling"), None);
        assert_eq!(school_variant_name("Phantasm"), None);
        assert_eq!(school_variant_name("Charm"), None);
    }
}
