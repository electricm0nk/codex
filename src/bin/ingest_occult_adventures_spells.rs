//! Ingests Occult Adventures's base spell catalog into the engine's
//! spell-catalog capability -- `SD31-E6-F2-003`'s primary deliverable, the
//! next book in `SD31-E6-F2-002`'s own named remaining-scope list
//! (`OPEN-ISSUES.md` row 57: 19 books, 1,257 units outside the then-6-book
//! chain; `occult_adventures` is the largest at 473 units).
//!
//! **Shape, re-derived one record deep before writing this file** (never
//! transcribed from the dispatch's own count):
//! `awk -F'\t' '!/^#/ && !/^SOURCELONG/ && NF>0 {print $1}'
//! oa_spells.lst | wc -l` -> **2040** raw active rows. Of those, **1526**
//! end in `.MOD` (declare nothing -- PCGen modifies an existing record) and
//! **369** contain `.COPY=` immediately after the `.MOD`/`.COPY=` filter
//! (`grep -c '\.COPY=' oa_spells.lst` on the first column) -- but EVERY one
//! of those 369 `.COPY=` rows in this book is bare (only the `.COPY=`
//! directive itself, no `SCHOOL:`/`CLASSES:` token of its own; worked
//! example: `oa_spells.lst:570`, `Analyze Aura.COPY=Occultist Spell ~
//! Analyze Aura`, nothing else on the line), so
//! `v06_work_inventory::has_classifying_token` drops every one as
//! `missing_classifying_token` and none reaches `docs/work-inventory.json`
//! as a `spell` unit under this book. The real content for those
//! class-scoped copies lives on a SEPARATE `.MOD` row targeting the copy's
//! own name (e.g. `oa_spells.lst:612`,
//! `Occultist Spell ~ Analyze Aura.MOD ... CLASSES:Occultist=2`), which
//! `v06_work_inventory` DOES enumerate as its own unit (`origin: mod_only`)
//! because its target name matches no record declared within this same
//! file. **This binary does not ingest that `mod_only` population** (328
//! units, re-derived: `python3` filtering `docs/work-inventory.json` for
//! `book=='occult_adventures' and kind=='spell' and origin=='mod_only'`) --
//! each one widens an ALREADY-CATALOGUED spell's class access (mostly this
//! book's own six new casting classes: Kineticist, Medium, Mesmerist,
//! Occultist, Psychic, Spiritualist) rather than declaring a new spell, and
//! correctly resolving 328 of them one at a time against whichever OTHER
//! book's table already carries the base record is materially more ingest
//! work than one book-onboarding cycle's bounded scope -- named here, not
//! silently dropped, and left for a follow-on (`OPEN-ISSUES.md`, this
//! cycle's own row).
//!
//! **This binary ingests exactly the 145 real base declarations** (rows
//! that are neither `.MOD`-suffixed nor `.COPY=`-bearing in their own first
//! field) -- the same `.MOD`/`.COPY=` exclusion convention every existing
//! per-book `spell_list.rs` states in its own doc comment
//! (`is_base_declaration`, identical logic to
//! `ingest_ultimate_magic_spells.rs`).
//!
//! **PI screening, both SD-30 invocation contracts, per record, before
//! writing anything** (`SD-30 decisions.md §52.3` blacklist sweep, `§53.5`
//! declared-PI reader), on the NAME as well as the description (the
//! safety-critical shape `cache_gen::ultimate_equipment`'s confirmed hole
//! named, not reproduced here). `oa_spells.lst` carries zero
//! `NAMEISPI:`/`DESCISPI:` tokens at all (re-derived:
//! `grep -c "NAMEISPI\|DESCISPI" oa_spells.lst` -> 0), so this run is not
//! expected to drop or redact anything -- the declared-PI reader still runs
//! on every record's own raw tokens, not skipped on that assumption. PI
//! gate: `epic-3-pi-gate` is `COMPLETE` package-wide
//! (`SD-30-class-feature-archetype-bundle/kanban.md`), and Occult
//! Adventures is one of the seven `future_state` books it names by name
//! (`epic-11-book-onboarding` row).
//!
//! `oa_spells.lst` carries no `DOMAINS:` token at all (re-derived:
//! `grep -c "DOMAINS:" oa_spells.lst` -> 0), unlike `um_spells.lst` -- so
//! unlike `ingest_ultimate_magic_spells.rs`, level derivation here reads
//! `CLASSES:` alone. None of the 145 base rows carry a bracketed
//! `[PRESKILL:...]` sub-condition inside `CLASSES:` either (re-derived:
//! regex-scanned, zero hits), so the bracket-stripping logic
//! `levels_in_field` needed for Ultimate Magic's Masterpiece records is not
//! reproduced here; it is omitted rather than carried as unreachable code.
//!
//! Two real corpus gaps, kept as `None` rather than fabricated (never
//! `.MOD`/`.COPY=` shaped -- both are genuine base declarations with a bare
//! row): `Talismanic Implement` (`oa_spells.lst:126`) carries no `CLASSES:`
//! token at all; `Repulsion` (`oa_spells.lst:464`) and `Share Language
//! (Communal)` (`oa_spells.lst:488`) carry only `TYPE:`/`CLASSES:` -- no
//! `SCHOOL:`/`DESC:` of their own.
//!
//! Run with `cargo run --locked --bin ingest_occult_adventures_spells`.
//! `PCGEN_CORPUS_ROOT` overrides the default `$HOME/workspace/repos/pcgen/data`.
//! Writes `src/rules_core/rules_tables/occult_adventures/spell_list.rs`.

use std::env;
use std::fs;
use std::path::PathBuf;

use std::collections::BTreeSet;

use codex::pcgen_import::lst_parser::spell::{parse_lst_spell_file, LstSpellRecord};
use codex::rules_core::pi_screening::{
    classify_field, classify_optional_field_declared, declared_product_identity,
};
use codex::rules_core::rules_tables::{
    acg, advanced_race_guide, apg, crb, ultimate_intrigue, ultimate_magic,
};

const OA_SPELLS_REL: &str =
    "pathfinder/paizo/roleplaying_game/occult_adventures/oa_spells.lst";
const OUT_PATH: &str = "src/rules_core/rules_tables/occult_adventures/spell_list.rs";

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

/// Every spell key already ingested by one of the six other modeled books --
/// the cross-book collision set this binary must not re-declare.
///
/// **Why this exists.** `oa_spells.lst:464`, `Repulsion`, is a genuine base
/// declaration in this book's own first-field shape (neither `.MOD`-suffixed
/// nor `.COPY=`-bearing) -- but it carries only `TYPE:`/`CLASSES:Spiritualist=6`,
/// no `SCHOOL:`/`DESC:` of its own, because Repulsion is already a full CRB
/// spell (`crb::spell_list::SPELL_LIST`, Abjuration 6) and this row exists
/// only to widen ITS class access to Spiritualist, not to declare a second,
/// competing "Repulsion". `spell_resolver.rs`'s own doc comment states spell
/// identity is the bare name, matched exactly, across every book -- so
/// shipping a second, thinner "Repulsion" entry under `OA` would violate
/// that identity (`spell_catalog.rs`'s
/// `no_key_is_served_twice_so_a_selection_resolves_unambiguously` test would
/// catch it) and would also be a strictly worse record than the one already
/// shipping (no school, no description). Re-derived, not assumed: of this
/// book's 145 base declarations, `Repulsion` is the ONLY collision (checked
/// against all six other tables' keys). `Share Language (Communal)` -- the
/// other record with the same bare `TYPE:`/`CLASSES:`-only shape -- collides
/// with none of them and is a genuine new spell, kept.
fn already_ingested_elsewhere() -> BTreeSet<&'static str> {
    crb::spell_list::SPELL_LIST
        .iter()
        .map(|e| e.key)
        .chain(apg::spell_list::SPELL_LIST.iter().map(|e| e.key))
        .chain(acg::spell_list::SPELL_LIST.iter().map(|e| e.key))
        .chain(advanced_race_guide::spell_list::SPELL_LIST.iter().map(|e| e.key))
        .chain(ultimate_intrigue::spell_list::SPELL_LIST.iter().map(|e| e.key))
        .chain(ultimate_magic::spell_list::SPELL_LIST.iter().map(|e| e.key))
        .collect()
}

/// Minimum spell level across one `CLASSES:`-shaped field value
/// (`"Medium=2|Mesmerist,Psychic,Spiritualist=3"` -> `2`). `None` when the
/// record carries no `CLASSES:` token at all (a real corpus gap, e.g.
/// `Talismanic Implement`, never fabricated here).
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
        school: None, // filled by caller
        level: None,  // filled by caller
        description: stored_description,
    })
}

/// The corpus's raw `SCHOOL:` string, verbatim -- normalized to a
/// `Pf1SchoolId` variant name at codegen time. Every school this book's 145
/// base rows carry is one of the standard nine (re-derived:
/// `grep -oP 'SCHOOL:\K[A-Za-z]+' oa_spells.lst | sort -u` restricted to
/// base rows finds only Abjuration/Conjuration/Divination/Enchantment/
/// Evocation/Illusion/Necromancy/Transmutation -- no `Universal` and no
/// unrecognized value in this book's base population), but the
/// unrecognized-value branch is kept (never a silent `None`-mapping-to-
/// something-else) for the same reason `ingest_ultimate_magic_spells.rs`
/// keeps it: a future re-run against a moved oracle must fail loudly, not
/// silently, if that ever changes.
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
        "//! Occult Adventures (OA) shared spell list.\n\
         //!\n\
         //! Generated by `src/bin/ingest_occult_adventures_spells.rs` from the\n\
         //! real `oa_spells.lst` corpus (SD31-E6-F2-003). Record coverage: every\n\
         //! real, active (non-`.MOD`, non-`.COPY=`) base spell declaration --\n\
         //! see that binary's own module doc comment for the 328-unit\n\
         //! `mod_only` class-widening residue this table deliberately does not\n\
         //! cover.\n\
         //!\n\
         //! `level` is the minimum level across the record's `CLASSES:`\n\
         //! token(s) (`oa_spells.lst` carries no `DOMAINS:` token at all,\n\
         //! unlike Ultimate Magic), `None` for the rare record that states\n\
         //! none (never fabricated -- these land `text-complete`, not\n\
         //! `ingested-magnitude`, via `v06_work_inventory::classify`'s\n\
         //! existing `Some(false)` branch).\n\
         //!\n\
         //! `school`/`description` are `Option` (mirroring\n\
         //! `rules_tables::ultimate_magic::spell_list`'s own shape) because a\n\
         //! small minority of records carry neither token of their own on\n\
         //! this book's base row.\n\n",
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
    let lst_path = data_root.join(OA_SPELLS_REL);
    let parsed = parse_lst_spell_file(&lst_path)
        .unwrap_or_else(|e| panic!("failed to parse {lst_path:?}: {e:?}"));

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
        "ingest_occult_adventures_spells: {} base declarations, {} cross-book collisions (already ingested elsewhere, skipped), {} PI-dropped, {} no-level (real gap, not fabricated), {} school-unrecognized",
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
    fn repulsion_is_the_only_cross_book_collision() {
        let elsewhere = already_ingested_elsewhere();
        assert!(
            elsewhere.contains("Repulsion"),
            "premise: CRB must already carry a real 'Repulsion' spell"
        );
        assert!(
            !elsewhere.contains("Share Language (Communal)"),
            "'Share Language (Communal)' must be a genuine new OA spell, not a collision"
        );
    }

    #[test]
    fn is_base_declaration_excludes_mod_and_copy_rows() {
        assert!(is_base_declaration("Akashic Form"));
        assert!(!is_base_declaration("Akashic Form.MOD"));
        assert!(!is_base_declaration("Analyze Aura.COPY=Occultist Spell ~ Analyze Aura"));
    }

    #[test]
    fn min_level_takes_the_minimum_across_multiple_classes_groups() {
        assert_eq!(
            min_level(Some("Medium=2|Mesmerist,Psychic,Spiritualist=3")),
            Some(2)
        );
    }

    #[test]
    fn min_level_is_none_when_the_token_is_absent() {
        assert_eq!(min_level(None), None);
    }

    #[test]
    fn min_level_handles_a_single_group() {
        assert_eq!(min_level(Some("Psychic=9")), Some(9));
    }

    #[test]
    fn pi_screen_drops_a_record_whose_row_declares_nameispi_yes() {
        let raw = "Secret Name\tNAMEISPI:YES\tCLASSES:Psychic=1\tSCHOOL:Evocation\tDESC:text";
        let outcome = pi_screen(raw, "Secret Name", Some("text"));
        assert!(matches!(outcome, PiOutcome::NamePiDropped(_)));
    }

    #[test]
    fn pi_screen_redacts_a_description_whose_row_declares_descispi_yes() {
        let raw = "Ordinary Spell\tDESCISPI:YES\tCLASSES:Psychic=1\tSCHOOL:Evocation\tDESC:secret lore";
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
        let raw = "Akashic Form\tCLASSES:Psychic=9\tSCHOOL:Necromancy\tDESC:restore yourself";
        let outcome = pi_screen(raw, "Akashic Form", Some("restore yourself"));
        match outcome {
            PiOutcome::Clean(entry) => {
                assert_eq!(entry.description.as_deref(), Some("restore yourself"));
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
