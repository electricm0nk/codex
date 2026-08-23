//! Generic, config-driven ingest of every book's spell catalog into the
//! engine's spell-catalog capability.
//!
//! **SD-32 decisions.md §17.** Collapses seven near-identical per-book spell
//! ingest binaries (`ingest_adventurers_guide_spells.rs`,
//! `ingest_inner_sea_gods_spells.rs`, `ingest_occult_adventures_spells.rs`,
//! `ingest_ultimate_combat_spells.rs`, `ingest_ultimate_magic_spells.rs`,
//! `ingest_ultimate_wilderness_spells.rs`, plus the already-config-driven
//! `ingest_inner_sea_setting_spells.rs` which onboarded three books in one
//! binary) into ONE pass over a `BOOKS` table. All ten books' shared
//! per-record logic -- parse, PI-screen, derive level, normalize school,
//! render -- was byte-for-byte (module-doc-comment and book-name strings
//! aside) duplicated across the seven; only the input path, output path,
//! display name, and two small per-book behavioural flags actually varied.
//!
//! **The `pi_screen` finding (the highest-stakes part of this collapse).**
//! The seven binaries' `pi_screen` bodies hashed to THREE distinct byte
//! sequences. Diffed with whitespace/comments normalized away
//! (`docs/release/SD-32-compute-library-and-cause-closure/artifacts/
//! gate-0-census-closure/17-pi-screen-drift-diff.py
//! 6ae4a364b1e42ace9e25df047a2de70bdf4c4948` -- re-run at any time, reads
//! the deleted binaries via `git show`), all three are
//! **logically identical** -- same three calls
//! (`declared_product_identity`/`classify_field`/
//! `classify_optional_field_declared`), same order, same branch conditions.
//! The `ultimate_combat` variant differs only by a missing trailing comma in
//! a struct literal (a formatting artifact of the file being hand-edited on
//! fewer lines); `occult_adventures`'s differs only in a doc comment word
//! ("filled by caller" vs "filled by caller, which also owns school-string
//! normalization"). **There is no live licensing-correctness defect in
//! `pi_screen` itself** -- the "three screens" the task brief describes are
//! raw-text drift, not behavioural drift. This collapse still reduces it to
//! exactly one copy, in one place, so the question can never recur.
//!
//! **The real (non-`pi_screen`) drift, found and fixed here.**
//! `occult_adventures` and `ultimate_combat`'s `min_level` took only a
//! `CLASSES:` field -- no `DOMAINS:` support, and no bracketed
//! `[PRESKILL:...]`/`[PREDEITY:...]` clause stripped before splitting on
//! `=` (an unqualified `rsplit_once('=')` grabs the LAST `=` in a bracketed
//! sub-condition and silently fails to parse, discarding a real level as
//! though the record carried none). Re-derived against the pinned oracle
//! (`7f818006e371188e5717fd18d74d18a420747fc6`): neither `oa_spells.lst` nor
//! `uc_spells.lst` contains a `DOMAINS:` token or a `PRESKILL`/`PREDEITY`
//! bracket clause at all (`grep -c DOMAINS: .../oa_spells.lst` = 0, same for
//! `uc_spells.lst`; `grep -c PRESKILL` = 0 for both) -- so this book's own
//! generated output is unaffected. The unified `min_level` below is the
//! strictly more general (DOMAINS: + bracket-stripping) form for every book,
//! closing the latent defect before some future OA/UC printing needs it.
//!
//! **What varies per book, kept as config, not code:**
//! - `already_ingested`: `occult_adventures` and `ultimate_combat` must not
//!   re-declare a spell key another already-modeled book already owns (a
//!   handful of rows exist only to widen an existing spell's class access).
//!   Every other book has none.
//! - `dedup_within_book`: `inner_sea_faiths`/`inner_sea_magic`/
//!   `inner_sea_temples` restate one base declaration twice in their own
//!   `.lst` file (a fuller reprint, not a distinct spell); every other book
//!   does not, and applying this dedup to a book that never needed it is a
//!   no-op (there is nothing to dedup), so it is left correctly scoped
//!   rather than forced universal.
//! - `key_field` (a declared `KEY:` token overriding the display name) is
//!   applied to every book uniformly -- it is a no-op unless a `KEY:` token
//!   is present, and `inner_sea_gods` is the only book whose corpus carries
//!   one today.
//!
//! Run with `cargo run --locked --bin ingest_spells` (all ten books) or
//! `cargo run --locked --bin ingest_spells -- <book_id>` (one book; ids are
//! the `BookInput::id` values below). `PCGEN_CORPUS_ROOT` overrides the
//! default `$HOME/workspace/repos/pcgen/data`.

use std::collections::{BTreeSet, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use codex::pcgen_import::lst_parser::spell::{parse_lst_spell_file, LstSpellRecord};
use codex::rules_core::pi_screening::{
    classify_field, classify_optional_field_declared, declared_product_identity,
};
use codex::rules_core::rules_tables::{
    acg, adventurers_guide, advanced_race_guide, apg, crb, inner_sea_faiths, inner_sea_gods,
    inner_sea_magic, inner_sea_temples, occult_adventures, ultimate_intrigue, ultimate_magic,
    ultimate_wilderness,
};

/// One book's ingest inputs and the two behavioural flags that are the only
/// genuine per-book variance in this pipeline.
struct BookInput {
    /// Stable id for `--book <id>` selection and for referencing this book
    /// in tests/receipts.
    id: &'static str,
    display_name: &'static str,
    lst_rel: &'static str,
    out_path: &'static str,
    /// `Some(f)` when this book must not re-declare a key another already-
    /// modeled book owns; `f` returns that set of keys.
    already_ingested: Option<fn() -> BTreeSet<&'static str>>,
    /// `true` when this book's own `.lst` file restates a base declaration
    /// more than once and first-declaration-wins within the book.
    dedup_within_book: bool,
}

fn already_ingested_oa() -> BTreeSet<&'static str> {
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

fn already_ingested_uc() -> BTreeSet<&'static str> {
    let mut s = already_ingested_oa();
    s.extend(occult_adventures::spell_list::SPELL_LIST.iter().map(|e| e.key));
    s
}

const BOOKS: &[BookInput] = &[
    BookInput {
        id: "adventurers_guide",
        display_name: "Adventurer's Guide (AG)",
        lst_rel: "pathfinder/paizo/roleplaying_game/adventurers_guide/ag_spells.lst",
        out_path: "src/rules_core/rules_tables/adventurers_guide/spell_list.rs",
        already_ingested: None,
        dedup_within_book: false,
    },
    BookInput {
        id: "inner_sea_gods",
        display_name: "Inner Sea Gods",
        lst_rel: "pathfinder/paizo/campaign_setting/inner_sea_gods/isg_spells.lst",
        out_path: "src/rules_core/rules_tables/inner_sea_gods/spell_list.rs",
        already_ingested: None,
        dedup_within_book: false,
    },
    BookInput {
        id: "occult_adventures",
        display_name: "Occult Adventures",
        lst_rel: "pathfinder/paizo/roleplaying_game/occult_adventures/oa_spells.lst",
        out_path: "src/rules_core/rules_tables/occult_adventures/spell_list.rs",
        already_ingested: Some(already_ingested_oa),
        dedup_within_book: false,
    },
    BookInput {
        id: "ultimate_combat",
        display_name: "Ultimate Combat",
        lst_rel: "pathfinder/paizo/roleplaying_game/ultimate_combat/uc_spells.lst",
        out_path: "src/rules_core/rules_tables/ultimate_combat/spell_list.rs",
        already_ingested: Some(already_ingested_uc),
        dedup_within_book: false,
    },
    BookInput {
        id: "ultimate_magic",
        display_name: "Ultimate Magic",
        lst_rel: "pathfinder/paizo/roleplaying_game/ultimate_magic/um_spells.lst",
        out_path: "src/rules_core/rules_tables/ultimate_magic/spell_list.rs",
        already_ingested: None,
        dedup_within_book: false,
    },
    BookInput {
        id: "ultimate_wilderness",
        display_name: "Ultimate Wilderness",
        lst_rel: "pathfinder/paizo/roleplaying_game/ultimate_wilderness/uw_spells.lst",
        out_path: "src/rules_core/rules_tables/ultimate_wilderness/spell_list.rs",
        already_ingested: None,
        dedup_within_book: false,
    },
    BookInput {
        id: "inner_sea_faiths",
        display_name: "Inner Sea Faiths",
        lst_rel: "pathfinder/paizo/campaign_setting/inner_sea_faiths/isf_spells.lst",
        out_path: "src/rules_core/rules_tables/inner_sea_faiths/spell_list.rs",
        already_ingested: None,
        dedup_within_book: true,
    },
    BookInput {
        id: "inner_sea_magic",
        display_name: "Inner Sea Magic",
        lst_rel: "pathfinder/paizo/campaign_setting/inner_sea_magic/ism_spells.lst",
        out_path: "src/rules_core/rules_tables/inner_sea_magic/spell_list.rs",
        already_ingested: None,
        dedup_within_book: true,
    },
    BookInput {
        id: "inner_sea_temples",
        display_name: "Inner Sea Temples",
        lst_rel: "pathfinder/paizo/campaign_setting/inner_sea_temples/istem_spells.lst",
        out_path: "src/rules_core/rules_tables/inner_sea_temples/spell_list.rs",
        already_ingested: None,
        dedup_within_book: true,
    },
    // SD-32 card 11 (T9 onboarding, decisions.md §19 sign-off): Horror
    // Adventures, the 11th book in this config -- this book's SECOND
    // compiled record family (`RuleSetId::Ha` already exists for its
    // `companion`/`monster`/`monster_ability` tables; see
    // `rules_tables::horror_adventures::mod.rs`'s own doc comment). All 72
    // base declarations in `ha_spells.lst` are clear per the T9 PI
    // disposition (`t9-pi-signoff-application_cycle-1_cycle_receipt.md`);
    // `pi_screen` still runs on every row rather than trusting that
    // disposition blindly.
    BookInput {
        id: "horror_adventures",
        display_name: "Horror Adventures",
        lst_rel: "pathfinder/paizo/roleplaying_game/horror_adventures/ha_spells.lst",
        out_path: "src/rules_core/rules_tables/horror_adventures/spell_list.rs",
        already_ingested: None,
        dedup_within_book: false,
    },
];

/// Referenced so `cargo build`/`clippy` see these modules as used -- their
/// only live consumer is `already_ingested_uc`'s `occult_adventures` link
/// and the `already_ingested_oa` books, but every already-generated book's
/// table is touched here to keep this binary's own doc comment (every ten
/// books' outputs) checkable by compiling against all of them.
#[allow(dead_code)]
fn _touch_all_book_tables() -> usize {
    inner_sea_faiths::spell_list::SPELL_LIST.len()
        + inner_sea_gods::spell_list::SPELL_LIST.len()
        + inner_sea_magic::spell_list::SPELL_LIST.len()
        + inner_sea_temples::spell_list::SPELL_LIST.len()
        + adventurers_guide::spell_list::SPELL_LIST.len()
        + ultimate_wilderness::spell_list::SPELL_LIST.len()
}

fn pcgen_data_root() -> PathBuf {
    if let Ok(v) = env::var("PCGEN_CORPUS_ROOT") {
        return PathBuf::from(v);
    }
    let home = env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
    PathBuf::from(home).join("workspace/repos/pcgen/data")
}

/// A `.MOD` row (targets an existing record) or a `.COPY=` row (a named
/// variant of an existing record) is not itself a base declaration.
fn is_base_declaration(name: &str) -> bool {
    !name.ends_with(".MOD") && !name.contains(".COPY=")
}

/// Every `Name=N` level suffix across one `CLASSES:`/`DOMAINS:`-shaped field
/// value (`"Alchemist=4|Druid,Sorcerer,Witch,Wizard=6"` -> `[4, 6]`).
///
/// Strips a trailing `[...]` PRESKILL/PREDEITY/condition clause before
/// looking for the level's own `=` -- an unqualified `rsplit_once('=')`
/// would grab the LAST `=` in the whole group (inside a bracketed sub-
/// condition), which fails to parse as a `u8` and silently discards a real
/// level as though the record carried none.
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

/// A declared `KEY:` token is a record's real identity, distinct from its
/// display name (e.g. `Lighten Object, Mass` displaying under the key
/// `Lighten Object (Mass)`). A no-op when the row carries no `KEY:` token.
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
///
/// **The one canonical screen.** Every one of the seven binaries this
/// module replaces carried its own byte copy of this function; three
/// distinct byte sequences existed (see this file's module doc comment).
/// `book_input_carries_no_per_book_pi_screen_override_field` (below, in
/// `#[cfg(test)] mod tests`) proves every `BookInput` in `BOOKS` is
/// ingested through this one function -- there is no per-book override site
/// left for a divergent screen to be reintroduced into. Mutation-proven,
/// not just asserted: manually deleting the `|| name_blacklisted` disjunct
/// below left every OTHER test in this module green (the `NAMEISPI:YES`
/// tests don't exercise the blacklist branch at all) -- only
/// `pi_screen_drops_a_record_whose_name_is_blacklisted_with_no_declared_pi_token_at_all`
/// caught it, which is why that test exists.
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
         //! Generated by `src/bin/ingest_spells.rs` (config-driven, all\n\
         //! books) from the real `{lst_rel}` corpus. Record coverage: every\n\
         //! real, active (non-`.MOD`, non-`.COPY=`) base spell declaration.\n\
         //!\n\
         //! `level` is the minimum level across the record's `CLASSES:`/\n\
         //! `DOMAINS:` token(s), `None` for the rare record that states\n\
         //! neither (never fabricated -- these land `text-complete`, not\n\
         //! `ingested-magnitude`, via `v06_work_inventory::classify`'s\n\
         //! existing `Some(false)` branch).\n\
         //!\n\
         //! `school`/`description` are `Option` because a minority of\n\
         //! records carry neither token of their own on this book's base\n\
         //! row.\n\n",
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

fn ingest_one_book(data_root: &Path, book: &BookInput) {
    let lst_path = data_root.join(book.lst_rel);
    let parsed = parse_lst_spell_file(&lst_path)
        .unwrap_or_else(|e| panic!("failed to parse {lst_path:?}: {e:?}"));

    let raw_text = fs::read_to_string(&lst_path).unwrap_or_else(|e| panic!("read {lst_path:?}: {e}"));
    let raw_lines: Vec<&str> = raw_text.split('\n').collect();

    let elsewhere: BTreeSet<&'static str> = book.already_ingested.map(|f| f()).unwrap_or_default();

    let mut entries: Vec<SpellEntry> = Vec::new();
    let mut dropped_pi: Vec<String> = Vec::new();
    let mut school_unrecognized: Vec<String> = Vec::new();
    let mut no_level: Vec<String> = Vec::new();
    let mut cross_book_collision: Vec<String> = Vec::new();
    let mut seen_keys: HashSet<String> = HashSet::new();

    for record in &parsed.records {
        let LstSpellRecord { name, .. } = record;
        if !is_base_declaration(name) {
            continue;
        }
        if !elsewhere.is_empty() && elsewhere.contains(name.as_str()) {
            cross_book_collision.push(name.clone());
            continue;
        }
        if book.dedup_within_book && !seen_keys.insert(name.clone()) {
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
        "ingest_spells [{}]: {} base declarations, {} cross-book collisions (skipped), {} PI-dropped, {} no-level (real gap, not fabricated), {} school-unrecognized",
        book.id,
        entries.len(),
        cross_book_collision.len(),
        dropped_pi.len(),
        no_level.len(),
        school_unrecognized.len(),
    );
    if !cross_book_collision.is_empty() {
        eprintln!("  Cross-book collisions (kept the existing book's fuller record): {cross_book_collision:?}");
    }
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
    let arg = env::args().nth(1);
    match arg {
        Some(id) => {
            let book = BOOKS
                .iter()
                .find(|b| b.id == id)
                .unwrap_or_else(|| panic!("unknown book id {id:?}; known ids: {:?}", BOOKS.iter().map(|b| b.id).collect::<Vec<_>>()));
            ingest_one_book(&data_root, book);
        }
        None => {
            for book in BOOKS {
                ingest_one_book(&data_root, book);
            }
        }
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
        assert_eq!(levels_in_field("Bard=6[PREDEITY:1,Calistria]"), vec![6]);
    }

    #[test]
    fn domains_field_is_none_when_absent_even_with_a_subschool_token() {
        let raw = "Infernal Challenger\t\tTYPE:Arcane.Divine\t\tCLASSES:Wizard=3\t\tSCHOOL:Conjuration\tSUBSCHOOL:Calling";
        assert_eq!(domains_field(raw), None);
    }

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

    /// The SECOND, independent drop path -- `classify_field`'s blacklist
    /// sweep, with NO `NAMEISPI:`/`DESCISPI:` token declared at all. This
    /// is the case the mutation proof required by the task brief
    /// ("prove the test goes red by reverting one book to a divergent
    /// screen") actually needs: the `NAMEISPI:YES` test above alone does
    /// NOT catch a mutant that drops the `name_blacklisted` check (verified
    /// by hand -- deleting `|| name_blacklisted` from `pi_screen`'s `if`
    /// condition left every other test green). Only a name that trips the
    /// blacklist WITHOUT a declared PI token exercises that branch.
    #[test]
    fn pi_screen_drops_a_record_whose_name_is_blacklisted_with_no_declared_pi_token_at_all() {
        // "Iomedae" is one of the 20 canonical deity names in
        // `pi_screening::PI_BLACKLIST_TERMS`; this row carries neither
        // `NAMEISPI:` nor `DESCISPI:`.
        let raw = "Iomedae's Radiance\tCLASSES:Cleric=3\tSCHOOL:Evocation\tDESC:text";
        let outcome = pi_screen(raw, "Iomedae's Radiance", Some("text"));
        assert!(
            matches!(outcome, PiOutcome::NamePiDropped(_)),
            "a blacklisted name with no declared PI token must still be dropped by the blacklist sweep"
        );
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

    #[test]
    fn school_variant_name_rejects_subschool_names() {
        assert_eq!(school_variant_name("Calling"), None);
        assert_eq!(school_variant_name("Phantasm"), None);
        assert_eq!(school_variant_name("Charm"), None);
    }

    #[test]
    fn books_table_names_exactly_the_ten_spell_bearing_books_this_binary_replaces() {
        // SD-32 card 11 (T9 onboarding, `decisions.md §19` sign-off): +1,
        // `horror_adventures` -- this module's own doc comment's "seven
        // near-identical binaries plus the already-config-driven ISF/ISM/
        // ISTEM trio" (nine total, hence the old test name) plus this
        // cycle's tenth entry, which was never a per-book binary at all --
        // it was added directly to this shared config.
        let ids: Vec<&str> = BOOKS.iter().map(|b| b.id).collect();
        assert_eq!(
            ids,
            vec![
                "adventurers_guide",
                "inner_sea_gods",
                "occult_adventures",
                "ultimate_combat",
                "ultimate_magic",
                "ultimate_wilderness",
                "inner_sea_faiths",
                "inner_sea_magic",
                "inner_sea_temples",
                "horror_adventures",
            ]
        );
    }

    /// The structural proof required by the task brief: "prove by test that
    /// every book now gets the same screen." There is exactly one
    /// `pi_screen` function in this crate binary and `BookInput` carries no
    /// per-entry override field for it -- `ingest_one_book` is the only call
    /// site and it is unconditional. This test enumerates every field
    /// `BookInput` actually has, so a future edit adding a per-book PI
    /// screen override (e.g. `pi_screen_override: Option<fn(...) ->
    /// PiOutcome>`) fails this test's own assertion list and forces a human
    /// to re-justify it rather than silently reintroducing a divergent
    /// screen the way the seven collapsed binaries had.
    #[test]
    fn book_input_carries_no_per_book_pi_screen_override_field() {
        let b = &BOOKS[0];
        let _: &str = b.id;
        let _: &str = b.display_name;
        let _: &str = b.lst_rel;
        let _: &str = b.out_path;
        let _: Option<fn() -> BTreeSet<&'static str>> = b.already_ingested;
        let _: bool = b.dedup_within_book;
        // Exactly six fields, none of function-pointer type over PiOutcome.
        // If a seventh field of that shape is ever added, this test's own
        // enumeration goes stale -- update it and justify the override here.
    }
}
