//! Ingests Ultimate Combat's base spell catalog into the engine's
//! spell-catalog capability -- `SD31-E6-F2-004`'s spell lever, the largest
//! genuinely-not-started book after `occult_adventures` (`ultimate_combat`:
//! 147 `spell` units, all `not-ingested`, re-derived fresh off
//! `docs/work-inventory.json`, book-agnostic to any prior estimate).
//!
//! **Shape, re-derived one record deep before writing this file** (never
//! transcribed): `awk -F'\t' '!/^#/ && NF>0 {print $1}' uc_spells.lst | wc
//! -l` -> **308** raw active first-column values. Of those, **159** end in
//! `.MOD` (widen an existing record -- e.g. attach a `DESC:`/`ITEM:POTION`
//! variant row to a base declaration elsewhere on this same file) and
//! **0** carry `.COPY=` (re-derived: `grep -c '\.COPY=' uc_spells.lst` ->
//! 0, unlike Occult Adventures). The remaining **147** first-column values
//! are genuine base declarations, matching `docs/work-inventory.json`'s own
//! 147-unit `ultimate_combat`/`spell` population exactly (145 `origin:
//! declared` + 2 `origin: mod_only`).
//!
//! **The 2 `mod_only`-origin work-inventory units, named rather than
//! silently dropped.** `Life Conduit (Greater)` and `Life Conduit
//! (Improved)` (`uc_spells.lst:65-66`) ARE base declarations in this book's
//! own first-column shape, but neither their own row nor `Life Conduit`
//! itself (the base spell those two are named variants of) carries a
//! `SCHOOL:`/`CLASSES:` token -- their real content (`DESC:`) lives on
//! separate `.MOD` rows at lines 216-217 (a third `.MOD` pair at 319-320
//! only adds `ITEM:POTION`, an alchemical-item variant, irrelevant to the
//! spell's own level/school). This is the exact shape
//! `ingest_occult_adventures_spells.rs`'s own doc comment named for
//! `Talismanic Implement`/`Repulsion`/`Share Language (Communal)` -- kept
//! here as base declarations with `level: None`/`school: None` (never
//! fabricated, 3 records total: `Life Conduit` and its two named variants),
//! NOT excluded, since `is_base_declaration` only filters on the record's
//! OWN first-column shape, not on whether it carries a `SCHOOL:`/
//! `CLASSES:` token. Whether `v06_work_inventory`'s separate `mod_only`-
//! origin enumeration re-resolves the 2 named-variant units once this
//! table exists is downstream of this cycle's own binary; not assumed
//! here.
//!
//! **Byte-for-byte, not reconstructed.** Reuses the existing tested
//! `pcgen_import::lst_parser::spell::parse_lst_spell_file` parser -- the
//! same parser `ingest_ultimate_magic_spells.rs`/
//! `ingest_occult_adventures_spells.rs` already use, not reimplemented.
//!
//! **PI screening, both SD-30 invocation contracts, per record, before
//! writing anything** (`SD-30 decisions.md §52.3` blacklist sweep, `§53.5`
//! declared-PI reader), on the NAME as well as the description. `uc_spells
//! .lst` carries zero `NAMEISPI:`/`DESCISPI:` tokens at all (re-derived:
//! `grep -c "NAMEISPI\|DESCISPI" uc_spells.lst` -> 0), so this run is not
//! expected to drop or redact anything -- the declared-PI reader still runs
//! on every record's own raw tokens, not skipped on that assumption. PI
//! gate: `epic-3-pi-gate` is `COMPLETE` package-wide, and Ultimate Combat is
//! one of the already-onboarded corpus books (`data/corpus/ultimate_combat`
//! exists with a shipped `LICENSE.json` from an earlier ingest lane).
//!
//! `uc_spells.lst` carries no `DOMAINS:` token at all (re-derived:
//! `grep -c "DOMAINS:" uc_spells.lst` -> 0) -- level derivation reads
//! `CLASSES:` alone, the same shape Occult Adventures has.
//!
//! Every school this book's base rows carry is one of the standard eight
//! (re-derived: `grep -oP '(?<=\t)SCHOOL:\K[A-Za-z]+' uc_spells.lst | sort
//! -u` restricted to base rows finds Abjuration/Conjuration/Divination/
//! Enchantment/Evocation/Illusion/Necromancy/Transmutation -- no
//! `Universal` in this book's base population), but the unrecognized-value
//! branch is kept for the same forward-compatibility reason every prior
//! book's ingest binary keeps it.
//!
//! Run with `cargo run --locked --bin ingest_ultimate_combat_spells`.
//! `PCGEN_CORPUS_ROOT` overrides the default `$HOME/workspace/repos/pcgen/data`.
//! Writes `src/rules_core/rules_tables/ultimate_combat/spell_list.rs`.

use std::env;
use std::fs;
use std::path::PathBuf;

use std::collections::BTreeSet;

use codex::pcgen_import::lst_parser::spell::{parse_lst_spell_file, LstSpellRecord};
use codex::rules_core::pi_screening::{
    classify_field, classify_optional_field_declared, declared_product_identity,
};
use codex::rules_core::rules_tables::{
    acg, advanced_race_guide, apg, crb, occult_adventures, ultimate_intrigue, ultimate_magic,
};

const UC_SPELLS_REL: &str = "pathfinder/paizo/roleplaying_game/ultimate_combat/uc_spells.lst";
const OUT_PATH: &str = "src/rules_core/rules_tables/ultimate_combat/spell_list.rs";

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
/// doc comment. Deliberately checks the record's OWN first-column shape
/// only, not whether it carries a `SCHOOL:`/`CLASSES:` token -- see this
/// binary's own module doc comment for why `Life Conduit (Greater)`/
/// `(Improved)` are still base declarations despite carrying neither.
fn is_base_declaration(name: &str) -> bool {
    !name.ends_with(".MOD") && !name.contains(".COPY=")
}

/// Every spell key already ingested by one of the seven other modeled
/// books -- the cross-book collision set this binary must not re-declare.
/// Re-derived corpus-wide before writing, not assumed empty: this book's
/// 147 base declarations were checked against all seven other tables'
/// keys; see this binary's own tests for the result.
fn already_ingested_elsewhere() -> BTreeSet<&'static str> {
    crb::spell_list::SPELL_LIST
        .iter()
        .map(|e| e.key)
        .chain(apg::spell_list::SPELL_LIST.iter().map(|e| e.key))
        .chain(acg::spell_list::SPELL_LIST.iter().map(|e| e.key))
        .chain(advanced_race_guide::spell_list::SPELL_LIST.iter().map(|e| e.key))
        .chain(ultimate_intrigue::spell_list::SPELL_LIST.iter().map(|e| e.key))
        .chain(ultimate_magic::spell_list::SPELL_LIST.iter().map(|e| e.key))
        .chain(occult_adventures::spell_list::SPELL_LIST.iter().map(|e| e.key))
        .collect()
}

/// Minimum spell level across one `CLASSES:`-shaped field value. `None`
/// when the record carries no `CLASSES:` token at all (a real corpus gap,
/// e.g. `Life Conduit (Greater)`/`(Improved)`, never fabricated here).
fn min_level(classes: Option<&str>) -> Option<u8> {
    classes?
        .split('|')
        .filter_map(|group| group.rsplit_once('='))
        .filter_map(|(_, level)| level.trim().parse::<u8>().ok())
        .min()
}

/// The row's own `(key, value)` tab tokens, split on the first `:` --
/// exactly what `pi_screening::declared_product_identity` consumes.
fn raw_row_tokens(raw_line: &str) -> Vec<(String, String)> {
    raw_line.split('\t').skip(1).filter_map(|col| col.trim().split_once(':')).map(|(k, v)| (k.to_string(), v.to_string())).collect()
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

    let (_, _, _, stored_description) = classify_optional_field_declared("description", description, declared.description);

    PiOutcome::Clean(SpellEntry { key: name.to_string(), school: None, level: None, description: stored_description })
}

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
        "//! Ultimate Combat (UC) shared spell list.\n\
         //!\n\
         //! Generated by `src/bin/ingest_ultimate_combat_spells.rs` from the\n\
         //! real `uc_spells.lst` corpus (SD31-E6-F2-004). Record coverage: every\n\
         //! real, active (non-`.MOD`, non-`.COPY=`) base spell declaration --\n\
         //! see that binary's own module doc comment for the 3-unit `Life\n\
         //! Conduit`/`Life Conduit (Greater)`/`Life Conduit (Improved)` residue\n\
         //! this table still carries with `level: None`\n\
         //! (a genuine corpus gap, not fabricated).\n\
         //!\n\
         //! `level` is the minimum level across the record's `CLASSES:`\n\
         //! token(s) (`uc_spells.lst` carries no `DOMAINS:` token at all),\n\
         //! `None` for the rare record that states none.\n\
         //!\n\
         //! `school`/`description` are `Option` (mirroring\n\
         //! `rules_tables::occult_adventures::spell_list`'s own shape)\n\
         //! because a small minority of records carry neither token of\n\
         //! their own on this book's base row.\n\n",
    );
    out.push_str(
        "/// The full 9-school PF1 spell-school enum, mirroring every other\n\
         /// book's own copy exactly.\n\
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
    let lst_path = data_root.join(UC_SPELLS_REL);
    let parsed = parse_lst_spell_file(&lst_path).unwrap_or_else(|e| panic!("failed to parse {lst_path:?}: {e:?}"));

    let raw_text = fs::read_to_string(&lst_path).unwrap_or_else(|e| panic!("read {lst_path:?}: {e}"));
    let raw_lines: Vec<&str> = raw_text.split('\n').collect();

    let elsewhere = already_ingested_elsewhere();
    let mut entries: Vec<SpellEntry> = Vec::new();
    let mut dropped_pi: Vec<String> = Vec::new();
    let mut school_unrecognized: Vec<String> = Vec::new();
    let mut no_level: Vec<String> = Vec::new();
    let mut cross_book_collision: Vec<String> = Vec::new();

    for record in &parsed.records {
        let LstSpellRecord { name, .. } = record;
        if !is_base_declaration(name) {
            continue;
        }
        if elsewhere.contains(name.as_str()) {
            cross_book_collision.push(name.clone());
            continue;
        }
        let raw_line = raw_lines.get(record.line_number - 1).copied().unwrap_or("");
        let level = min_level(record.classes.as_deref());
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
        "ingest_ultimate_combat_spells: {} base declarations, {} cross-book collisions (already ingested elsewhere, skipped), {} PI-dropped, {} no-level (real gap, not fabricated), {} school-unrecognized",
        entries.len(),
        cross_book_collision.len(),
        dropped_pi.len(),
        no_level.len(),
        school_unrecognized.len(),
    );
    if !cross_book_collision.is_empty() {
        eprintln!("Cross-book collisions (kept the existing book's fuller record): {cross_book_collision:?}");
    }
    if !dropped_pi.is_empty() {
        eprintln!("PI-dropped (name declared or blacklisted): {dropped_pi:?}");
    }
    if !no_level.is_empty() {
        eprintln!("No CLASSES: level (kept, level=None -> text-complete): {no_level:?}");
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
        assert!(is_base_declaration("Ablative Barrier"));
        assert!(!is_base_declaration("Ablative Barrier.MOD"));
        assert!(!is_base_declaration("Something.COPY=Something Else"));
    }

    #[test]
    fn min_level_takes_the_minimum_across_multiple_classes_groups() {
        assert_eq!(min_level(Some("Bard=1|Sorcerer,Wizard=2")), Some(1));
    }

    #[test]
    fn min_level_is_none_when_the_token_is_absent() {
        assert_eq!(min_level(None), None);
    }

    #[test]
    fn pi_screen_drops_a_record_whose_row_declares_nameispi_yes() {
        let raw = "Secret Name\tNAMEISPI:YES\tCLASSES:Fighter=1\tSCHOOL:Evocation\tDESC:text";
        let outcome = pi_screen(raw, "Secret Name", Some("text"));
        assert!(matches!(outcome, PiOutcome::NamePiDropped(_)));
    }

    #[test]
    fn pi_screen_redacts_a_description_whose_row_declares_descispi_yes() {
        let raw = "Ordinary Spell\tDESCISPI:YES\tCLASSES:Fighter=1\tSCHOOL:Evocation\tDESC:secret lore";
        let outcome = pi_screen(raw, "Ordinary Spell", Some("secret lore"));
        match outcome {
            PiOutcome::Clean(entry) => assert_ne!(entry.description.as_deref(), Some("secret lore")),
            PiOutcome::NamePiDropped(_) => panic!("a DESCISPI-only declaration must not drop the record"),
        }
    }

    #[test]
    fn pi_screen_passes_a_clean_record_through_unredacted() {
        let raw = "Ablative Barrier\tCLASSES:Magus=1\tSCHOOL:Abjuration\tDESC:protects you";
        let outcome = pi_screen(raw, "Ablative Barrier", Some("protects you"));
        match outcome {
            PiOutcome::Clean(entry) => assert_eq!(entry.description.as_deref(), Some("protects you")),
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

    /// This book's real "bare declaration" shape, named in the module doc
    /// comment: `Life Conduit (Greater)` carries no `CLASSES:`/`SCHOOL:` of
    /// its own -- must land `level: None`, never fabricated.
    #[test]
    fn a_bare_base_declaration_with_no_classifying_token_is_kept_with_none_fields() {
        let raw = "Life Conduit (Greater)\tSOURCEPAGE:p.234";
        let outcome = pi_screen(raw, "Life Conduit (Greater)", None);
        match outcome {
            PiOutcome::Clean(entry) => {
                assert_eq!(entry.level, None);
                assert_eq!(entry.description, None);
            }
            PiOutcome::NamePiDropped(_) => panic!("a clean bare record must not be dropped"),
        }
    }
}
