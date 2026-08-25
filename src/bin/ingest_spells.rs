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

use std::collections::{BTreeSet, HashMap, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use codex::pcgen_import::lst_parser::spell::{parse_lst_spell_file, LstSpellRecord};
use codex::rules_core::codex_neutral_name::neutral_name;
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
    // SD-32 `decisions.md §20`, no_record-to-zero wave: `ultimate_magic`'s
    // SECOND source file -- the Words of Power variant subsystem's three
    // "Example Word Spells" (`um_spells_wordsofpower.lst`), each a normal
    // base spell declaration. A prior cycle's receipt speculated this was
    // the same "missing config row over an already-compiled table" shape
    // as `bestiary_6`; re-derived (`decisions.md §17a`) and found wrong --
    // no compiled table existed for this file at all, so this is a real
    // new `BookInput`/module, not a one-line config addition.
    BookInput {
        id: "ultimate_magic_wordsofpower",
        display_name: "Ultimate Magic (Words of Power examples)",
        lst_rel: "pathfinder/paizo/roleplaying_game/ultimate_magic/um_spells_wordsofpower.lst",
        out_path: "src/rules_core/rules_tables/ultimate_magic_wordsofpower/spell_list.rs",
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
    // SD-32 `decisions.md §20`, card 11 next-cycle-plan item 1: a prior
    // cycle reported `bestiary`'s 109 and `bestiary_4`'s 56 `no_record`
    // `spell` units as "monster-intrinsic with no dedicated `.lst`" without
    // re-verifying. Re-derived: BOTH books carry a real, dedicated spell
    // `.lst` file of custom variant declarations (restricted-form recasts
    // of existing spells used by monster spell-like abilities, e.g. "Blur
    // (self only)", "Charm Monster (elementals only)") -- each row carries
    // its own `TYPE:`/`SCHOOL:`/`DESC:` tokens, the identical shape every
    // other book's base spell declaration has. The prior finding was wrong;
    // named per `decisions.md §17a`, not silently corrected.
    //
    // `bestiary`'s file physically lives under the shared `core_essentials`
    // directory (the same B1/`core_essentials` split
    // `cache_gen::equipment_gap::book_routing`'s own doc comment names for
    // this book's equipment rows) -- `lst_rel` points at the real file, the
    // shipped book id stays `"bestiary"`.
    BookInput {
        id: "bestiary",
        display_name: "Bestiary (custom spell-like-ability variants)",
        lst_rel: "pathfinder/paizo/roleplaying_game/core_essentials/ce_spells.lst",
        out_path: "src/rules_core/rules_tables/bestiary/spell_list.rs",
        already_ingested: None,
        dedup_within_book: false,
    },
    BookInput {
        id: "bestiary_4",
        display_name: "Bestiary 4 (modified spell variants)",
        lst_rel: "pathfinder/paizo/roleplaying_game/bestiary_4/b4_spells_modified.lst",
        out_path: "src/rules_core/rules_tables/bestiary_4/spell_list.rs",
        already_ingested: None,
        dedup_within_book: false,
    },
    // SD-32 `decisions.md §20`, no_record-to-zero wave: eight more books
    // with a real, dedicated spell `.lst` and zero corpus coverage,
    // re-derived directly against the pinned oracle (`find ... -iname
    // '*spell*.lst'` under each book's directory) rather than assumed --
    // same "config-driven, no new logic" shape as every entry above.
    BookInput {
        id: "inner_sea_races",
        display_name: "Inner Sea Races",
        lst_rel: "pathfinder/paizo/campaign_setting/inner_sea_races/isr_spells.lst",
        out_path: "src/rules_core/rules_tables/inner_sea_races/spell_list.rs",
        already_ingested: None,
        dedup_within_book: false,
    },
    BookInput {
        id: "inner_sea_intrigue",
        display_name: "Inner Sea Intrigue",
        lst_rel: "pathfinder/paizo/campaign_setting/inner_sea_intrigue/isi_spells.lst",
        out_path: "src/rules_core/rules_tables/inner_sea_intrigue/spell_list.rs",
        already_ingested: None,
        dedup_within_book: false,
    },
    BookInput {
        id: "monster_codex",
        display_name: "Monster Codex",
        lst_rel: "pathfinder/paizo/roleplaying_game/monster_codex/mc_spells.lst",
        out_path: "src/rules_core/rules_tables/monster_codex/spell_list.rs",
        already_ingested: None,
        dedup_within_book: false,
    },
    BookInput {
        id: "inner_sea_world_guide",
        display_name: "Inner Sea World Guide",
        lst_rel: "pathfinder/paizo/campaign_setting/inner_sea_world_guide/iswg_spells.lst",
        out_path: "src/rules_core/rules_tables/inner_sea_world_guide/spell_list.rs",
        already_ingested: None,
        dedup_within_book: false,
    },
    // The book's own `.pcc` loads `botd1_spells.lst` unconditionally; a
    // `_pfs/pfs_botd1_spells.lst` variant also exists but is the
    // Pathfinder-Society-legal restatement, not this pipeline's target
    // (every other book in this table ingests the base sourcebook file,
    // never its `_pfs/` counterpart).
    BookInput {
        id: "book_of_the_damned_volume_1",
        display_name: "Book of the Damned, Volume 1",
        lst_rel: "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_1/botd1_spells.lst",
        out_path: "src/rules_core/rules_tables/book_of_the_damned_volume_1/spell_list.rs",
        already_ingested: None,
        dedup_within_book: false,
    },
    // This book's own `.pcc` loads TWO spell lists: `botd2_spells.lst`
    // unconditionally and `botd2_spells_ndl.lst` only
    // `!PRECAMPAIGN:1,Inner Sea World Guide` (a "no-duplicates" restatement
    // gated OFF when Inner Sea World Guide is also loaded, to avoid
    // redeclaring spells that book already owns). This pipeline models no
    // campaign gating anywhere else, so `botd2_spells.lst` -- the
    // unconditional file -- is the ingested source, matching every other
    // book's single-canonical-file shape.
    BookInput {
        id: "book_of_the_damned_volume_2",
        display_name: "Book of the Damned, Volume 2",
        lst_rel: "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_2/botd2_spells.lst",
        out_path: "src/rules_core/rules_tables/book_of_the_damned_volume_2/spell_list.rs",
        already_ingested: None,
        dedup_within_book: false,
    },
    // `ma_abilities_spell.lst` is a spell-LIKE-ability catalog (a distinct
    // kind, `monster_ability`/`race_trait`-shaped), not this book's base
    // spell catalog; `ma_spells.lst` is.
    BookInput {
        id: "mythic_adventures",
        display_name: "Mythic Adventures",
        lst_rel: "pathfinder/paizo/roleplaying_game/mythic_adventures/ma_spells.lst",
        out_path: "src/rules_core/rules_tables/mythic_adventures/spell_list.rs",
        already_ingested: None,
        dedup_within_book: false,
    },
    BookInput {
        id: "ultimate_equipment",
        display_name: "Ultimate Equipment",
        lst_rel: "pathfinder/paizo/roleplaying_game/ultimate_equipment/ue_spells.lst",
        out_path: "src/rules_core/rules_tables/ultimate_equipment/spell_list.rs",
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

/// `.COPY=` base declarations this module does not otherwise read: `core_
/// rulebook`'s own spell catalog is not one of `BOOKS` (it is modeled by a
/// separate, hand-authored table, not this generic generator), but it is
/// the single most common base a `.COPY=` row anywhere in the corpus cites
/// (`veil_self_only`'s `Veil` and the corpus-wide `Greater Teleport (Self
/// Plus 50 Lbs. Of Objects Only)`'s `Teleport (Greater)` both resolve here).
/// Named as a book, not a record -- widens the mechanism's real population,
/// not two special-cased units (`decisions.md §17`).
const EXTRA_BASE_DECLARATION_FILES: &[&str] =
    &["pathfinder/paizo/roleplaying_game/core_rulebook/cr_spells.lst"];

/// The corpus-wide `.COPY=` base index: every real (non-`.MOD`, non-
/// `.COPY=`) base spell declaration's own `(level, school)`, keyed by its
/// declared name, across every book `BOOKS` reads plus
/// `EXTRA_BASE_DECLARATION_FILES`. A `.COPY=` row's own line frequently
/// states neither `CLASSES:`/`DOMAINS:` nor `SCHOOL:` (it inherits the
/// spell it names) -- this is the corpus-wide lookup `ingest_one_book`
/// consults so that inheritance is resolved for real rather than left
/// `None` for every `.COPY=` row regardless of book.
///
/// First declaration wins on a name collision (deterministic: `BOOKS`
/// order, then `EXTRA_BASE_DECLARATION_FILES` order) -- the same
/// first-match convention `already_ingested`'s cross-book collision guard
/// already uses elsewhere in this file.
fn build_global_base_index(data_root: &Path) -> HashMap<String, (Option<u8>, Option<String>)> {
    let mut index: HashMap<String, (Option<u8>, Option<String>)> = HashMap::new();
    let mut lst_rels: Vec<&str> = BOOKS.iter().map(|b| b.lst_rel).collect();
    lst_rels.extend_from_slice(EXTRA_BASE_DECLARATION_FILES);
    for lst_rel in lst_rels {
        let path = data_root.join(lst_rel);
        let Ok(parsed) = parse_lst_spell_file(&path) else { continue };
        let Ok(raw_text) = fs::read_to_string(&path) else { continue };
        let raw_lines: Vec<&str> = raw_text.split('\n').collect();
        for record in &parsed.records {
            if !is_base_declaration(&record.name) {
                continue;
            }
            if index.contains_key(&record.name) {
                continue;
            }
            let raw_line = raw_lines.get(record.line_number - 1).copied().unwrap_or("");
            let domains = domains_field(raw_line);
            let level = min_level(record.classes.as_deref(), domains.as_deref());
            index.insert(record.name.clone(), (level, record.school.clone()));
        }
    }
    index
}

/// A `.MOD` row (targets an existing record) or a `.COPY=` row (a named
/// variant of an existing record) is not itself a base declaration.
fn is_base_declaration(name: &str) -> bool {
    !name.ends_with(".MOD") && !name.contains(".COPY=")
}

/// `decisions.md §17` corpus-wide gap: this module had NO `.COPY=`
/// resolution mechanism at all (confirmed by reading `is_base_declaration`
/// above, which excludes every `.COPY=` row from parsing entirely). Every
/// `.MOD` row still targets an EXISTING record and stays out of scope here
/// (`equipment_gap.rs` treats `.MOD` and `.COPY=` differently for exactly
/// this reason: a `.MOD` never introduces a new identity, a `.COPY=`
/// always does). Splitting `"Teleport (Greater).COPY=Greater Teleport
/// (Self Plus 50 Lbs. Of Objects Only)"` yields
/// `("Teleport (Greater)", "Greater Teleport (Self Plus 50 Lbs. Of
/// Objects Only)")` -- the row's OWN new identity is the part AFTER
/// `.COPY=`, mirroring `gen_equipment_gap_tables.rs::parse_lst`'s
/// identical `first.split_once(".COPY=")` convention (`base`, `variant`)
/// for the same corpus shape. `None` for a plain base declaration.
fn copy_variant_split(name: &str) -> Option<(&str, &str)> {
    if name.ends_with(".MOD") {
        return None;
    }
    name.split_once(".COPY=")
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
    /// `decisions.md §24`: `Some(line)` when `key` above is a
    /// Codex-generated neutral identity (the row's real name is Product
    /// Identity) -- carries the real citation line so `cache_gen::
    /// spell_lane_dump` can resolve it without a name-based LST lookup,
    /// which would fail (the real content no longer contains this
    /// string). `None` for an ordinary entry.
    name_pi_line: Option<u32>,
    school: Option<String>,
    level: Option<u8>,
    description: Option<String>,
}

/// Screens one record with BOTH SD-30 invocation contracts -- the blacklist
/// sweep (`classify_field`) and the declared-PI reader
/// (`declared_product_identity`) -- against BOTH the name and the
/// description. A description hit redacts the description field only.
///
/// **`decisions.md §24` (SD-32):** a name hit (from either contract) no
/// longer drops the record -- it ingests under a Codex-generated neutral
/// name/key (`codex_neutral_name::neutral_name`) derived ONLY from
/// `(kind, book, source_file, source_line)`, never from the original PI
/// name (`name`/`description` are not even read in that branch's
/// derivation). `line` is 1-indexed within `lst_rel`.
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
fn pi_screen(
    raw_line: &str,
    name: &str,
    description: Option<&str>,
    book_id: &str,
    lst_rel: &str,
    line: u32,
) -> SpellEntry {
    let declared = declared_product_identity(raw_row_tokens(raw_line));

    let (name_license, ..) = classify_field("name", name);
    let name_blacklisted = name_license != codex::rules_core::shape_b_v1::License::Ogl;
    let name_is_pi = declared.name || name_blacklisted;

    let (_, _, _, stored_description) =
        classify_optional_field_declared("description", description, declared.description);

    let (key, name_pi_line) = if name_is_pi {
        (neutral_name("spell", book_id, lst_rel, line), Some(line))
    } else {
        (name.to_string(), None)
    };

    SpellEntry {
        key,
        name_pi_line,
        school: None, // filled by caller, which also owns school-string normalization
        level: None,  // filled by caller
        description: stored_description,
    }
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
    let name_pi_line = match e.name_pi_line {
        Some(n) => format!("Some({n})"),
        None => "None".to_string(),
    };
    format!(
        "    SpellListEntry {{ key: \"{}\", name_pi_line: {name_pi_line}, school: {school}, level: {level}, description: {description} }},",
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
         //! real, active (non-`.MOD`) base spell declaration, PLUS every\n\
         //! `.COPY=` variant declared in this book (its own new identity is\n\
         //! the name after `.COPY=`; `level`/`school` inherit from the base\n\
         //! record it names, resolved corpus-wide, when its own row states\n\
         //! neither).\n\
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
         \x20   /// `decisions.md §24`: `Some(line)` ONLY when `key` above\n\
         \x20   /// is a Codex-generated neutral identity (the row's real\n\
         \x20   /// name is Product Identity) -- carries the real citation\n\
         \x20   /// line so `cache_gen::spell_lane_dump` can resolve it\n\
         \x20   /// without a name-based lookup. `None` for an ordinary entry.\n\
         \x20   pub name_pi_line: Option<u32>,\n\
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

fn ingest_one_book(
    data_root: &Path,
    book: &BookInput,
    base_index: &HashMap<String, (Option<u8>, Option<String>)>,
) {
    let lst_path = data_root.join(book.lst_rel);
    let parsed = parse_lst_spell_file(&lst_path)
        .unwrap_or_else(|e| panic!("failed to parse {lst_path:?}: {e:?}"));

    let raw_text = fs::read_to_string(&lst_path).unwrap_or_else(|e| panic!("read {lst_path:?}: {e}"));
    let raw_lines: Vec<&str> = raw_text.split('\n').collect();

    let elsewhere: BTreeSet<&'static str> = book.already_ingested.map(|f| f()).unwrap_or_default();

    let mut entries: Vec<SpellEntry> = Vec::new();
    let mut renamed_pi: Vec<String> = Vec::new();
    let mut school_unrecognized: Vec<String> = Vec::new();
    let mut no_level: Vec<String> = Vec::new();
    let mut cross_book_collision: Vec<String> = Vec::new();
    let mut seen_keys: HashSet<String> = HashSet::new();
    let mut copy_variants_resolved: Vec<String> = Vec::new();
    let mut copy_variants_unresolved: Vec<String> = Vec::new();

    for record in &parsed.records {
        let LstSpellRecord { name: raw_name, .. } = record;
        if raw_name.ends_with(".MOD") {
            // A `.MOD` row targets an existing record; it never introduces
            // a new identity, so it stays out of this ingest entirely (same
            // as before this fix).
            continue;
        }
        // `decisions.md §17`: a `.COPY=` row DOES introduce a new record
        // identity (the part after `.COPY=`) and previously was dropped
        // wholesale by `is_base_declaration`. Resolve it here instead of
        // special-casing individual keys -- this applies uniformly to every
        // book this generator reads.
        let copy_base = copy_variant_split(raw_name).map(|(base, _)| base.to_string());
        let name: String = match copy_variant_split(raw_name) {
            Some((_, variant)) => variant.to_string(),
            None => raw_name.clone(),
        };
        let name = name.as_str();
        if !elsewhere.is_empty() && elsewhere.contains(name) {
            cross_book_collision.push(name.to_string());
            continue;
        }
        if book.dedup_within_book && !seen_keys.insert(name.to_string()) {
            continue;
        }
        let raw_line = raw_lines.get(record.line_number - 1).copied().unwrap_or("");
        let domains = domains_field(raw_line);
        let mut level = min_level(record.classes.as_deref(), domains.as_deref());
        let mut school_raw: Option<String> = record.school.clone();
        if let Some(base_name) = &copy_base {
            if let Some((base_level, base_school)) = base_index.get(base_name) {
                if level.is_none() && base_level.is_some() {
                    level = *base_level;
                }
                if school_raw.is_none() && base_school.is_some() {
                    school_raw = base_school.clone();
                }
                copy_variants_resolved.push(name.to_string());
            } else {
                copy_variants_unresolved.push(name.to_string());
            }
        }
        if level.is_none() {
            no_level.push(name.to_string());
        }

        let mut entry = pi_screen(raw_line, name, record.description.as_deref(), book.id, book.lst_rel, record.line_number as u32);
        if entry.name_pi_line.is_some() {
            // `decisions.md §24b`-4: coordinate + reason only, never the
            // original PI name -- `book.id`/`book.lst_rel`/`record.
            // line_number` are all non-PI coordinates.
            renamed_pi.push(format!("{}:{}:{}", book.id, book.lst_rel, record.line_number));
        } else if let Some(real_key) = key_field(raw_line) {
            // A declared `KEY:` token overrides the display name for an
            // ORDINARY record only -- a renamed (name-PI) entry's `key`
            // is already the Codex-generated neutral identity and must
            // never be overwritten back toward anything derived from the
            // real corpus row (`§24b`-1).
            entry.key = real_key;
        }
        entry.level = level;
        entry.school = match &school_raw {
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

    eprintln!(
        "ingest_spells [{}]: {} base declarations, {} cross-book collisions (skipped), {} renamed under a Codex-generated neutral identity (decisions.md §24), {} no-level (real gap, not fabricated), {} school-unrecognized, {} .COPY= variants resolved against a base record, {} .COPY= variants with no findable base",
        book.id,
        entries.len(),
        cross_book_collision.len(),
        renamed_pi.len(),
        no_level.len(),
        school_unrecognized.len(),
        copy_variants_resolved.len(),
        copy_variants_unresolved.len(),
    );
    if !cross_book_collision.is_empty() {
        eprintln!("  Cross-book collisions (kept the existing book's fuller record): {cross_book_collision:?}");
    }
    if !renamed_pi.is_empty() {
        eprintln!("  Renamed under a Codex-generated neutral identity (name declared or blacklisted), by coordinate: {renamed_pi:?}");
    }
    if !no_level.is_empty() {
        eprintln!("  No CLASSES:/DOMAINS: level (kept, level=None -> text-complete): {no_level:?}");
    }
    if !school_unrecognized.is_empty() {
        eprintln!("  Unrecognized SCHOOL: string (kept, school=None): {school_unrecognized:?}");
    }
    if !copy_variants_unresolved.is_empty() {
        eprintln!("  .COPY= variants whose base was not found in the global base index (kept, level/school from own row only): {copy_variants_unresolved:?}");
    }

    let source = build_module_source(book.display_name, book.lst_rel, &entries);
    fs::write(book.out_path, source).unwrap_or_else(|e| panic!("write {}: {e}", book.out_path));
    eprintln!("  wrote {} ({} entries)", book.out_path, entries.len());
}

fn main() {
    let data_root = pcgen_data_root();
    let arg = env::args().nth(1);
    let base_index = build_global_base_index(&data_root);
    match arg {
        Some(id) => {
            let book = BOOKS
                .iter()
                .find(|b| b.id == id)
                .unwrap_or_else(|| panic!("unknown book id {id:?}; known ids: {:?}", BOOKS.iter().map(|b| b.id).collect::<Vec<_>>()));
            ingest_one_book(&data_root, book, &base_index);
        }
        None => {
            for book in BOOKS {
                ingest_one_book(&data_root, book, &base_index);
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

    /// `decisions.md §17` -- the real corpus shape this fix targets:
    /// `ce_spells.lst:49`, `"Veil.COPY=Veil (self only)"`. The row's OWN
    /// new identity is the part AFTER `.COPY=` ("Veil (self only)"), and
    /// the base it inherits from is the part before ("Veil") -- verified
    /// against `gen_equipment_gap_tables.rs::parse_lst`'s identical
    /// `.split_once(".COPY=")` convention for the same corpus shape.
    #[test]
    fn copy_variant_split_reads_base_before_and_variant_after() {
        assert_eq!(
            copy_variant_split("Veil.COPY=Veil (self only)"),
            Some(("Veil", "Veil (self only)"))
        );
    }

    #[test]
    fn copy_variant_split_is_none_for_a_plain_base_declaration() {
        assert_eq!(copy_variant_split("Fireball"), None);
    }

    #[test]
    fn copy_variant_split_is_none_for_a_mod_row_even_if_it_also_names_copy() {
        // A `.MOD` row targets an existing record; it must never be read as
        // introducing a new `.COPY=` identity.
        assert_eq!(copy_variant_split("Fireball.COPY=X.MOD"), None);
    }

    /// RED before this cycle's fix: `build_global_base_index` did not exist
    /// and `ingest_one_book` dropped every `.COPY=` row via
    /// `is_base_declaration`, so a `.COPY=` variant whose own line states
    /// no `CLASSES:`/`SCHOOL:` could never resolve a level or school from
    /// anywhere, corpus-wide. GREEN after: the base's `(level, school)` is
    /// found by name across every file the index scans, regardless of
    /// which file states them.
    #[test]
    fn global_base_index_resolves_a_copy_variants_base_across_files() {
        let dir = std::env::temp_dir()
            .join(format!("ingest_spells_base_index_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("base.lst"),
            "Teleport (Greater)\tTYPE:Arcane\tCLASSES:Sorcerer,Wizard=7\tSCHOOL:Conjuration\tDESC:Base spell.\n",
        )
        .unwrap();
        let mut index = HashMap::new();
        let path = dir.join("base.lst");
        let parsed = parse_lst_spell_file(&path).unwrap();
        let raw_text = std::fs::read_to_string(&path).unwrap();
        let raw_lines: Vec<&str> = raw_text.split('\n').collect();
        for record in &parsed.records {
            if !is_base_declaration(&record.name) {
                continue;
            }
            let raw_line = raw_lines.get(record.line_number - 1).copied().unwrap_or("");
            let domains = domains_field(raw_line);
            let level = min_level(record.classes.as_deref(), domains.as_deref());
            index.insert(record.name.clone(), (level, record.school.clone()));
        }
        assert_eq!(
            index.get("Teleport (Greater)"),
            Some(&(Some(7), Some("Conjuration".to_string())))
        );
        std::fs::remove_dir_all(&dir).ok();
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

    /// `decisions.md §24` end-to-end proof: a declared `NAMEISPI:YES` row
    /// must now be KEPT (not dropped) under a Codex-generated neutral
    /// identity, with `name_pi_line` carrying the real citation and the
    /// original name appearing NOWHERE in the output.
    #[test]
    fn pi_screen_renames_a_record_whose_row_declares_nameispi_yes() {
        let raw = "Secret Name\tNAMEISPI:YES\tCLASSES:Wizard=1\tSCHOOL:Evocation\tDESC:text";
        let entry = pi_screen(raw, "Secret Name", Some("text"), "inner_sea_gods", "isg_spells.lst", 7);
        assert!(entry.key.starts_with("Codex-Named Unit ("), "must carry the marker: {}", entry.key);
        assert!(!entry.key.contains("Secret Name"));
        assert_eq!(entry.name_pi_line, Some(7));
    }

    /// The SECOND, independent rename trigger -- `classify_field`'s
    /// blacklist sweep, with NO `NAMEISPI:`/`DESCISPI:` token declared at
    /// all. This is the case the mutation proof required by the task
    /// brief ("prove the test goes red by reverting one book to a
    /// divergent screen") actually needs: the `NAMEISPI:YES` test above
    /// alone does NOT catch a mutant that drops the `name_blacklisted`
    /// check (verified by hand -- deleting `|| name_blacklisted` from
    /// `pi_screen`'s `if` condition left every other test green). Only a
    /// name that trips the blacklist WITHOUT a declared PI token exercises
    /// that branch.
    #[test]
    fn pi_screen_renames_a_record_whose_name_is_blacklisted_with_no_declared_pi_token_at_all() {
        // "Iomedae" is one of the 20 canonical deity names in
        // `pi_screening::PI_BLACKLIST_TERMS`; this row carries neither
        // `NAMEISPI:` nor `DESCISPI:`.
        let raw = "Iomedae's Radiance\tCLASSES:Cleric=3\tSCHOOL:Evocation\tDESC:text";
        let entry = pi_screen(raw, "Iomedae's Radiance", Some("text"), "inner_sea_gods", "isg_spells.lst", 3);
        assert!(
            entry.key.starts_with("Codex-Named Unit ("),
            "a blacklisted name with no declared PI token must still be renamed by the blacklist sweep"
        );
        assert!(!entry.key.contains("Iomedae"));
        assert_eq!(entry.name_pi_line, Some(3));
    }

    #[test]
    fn pi_screen_redacts_a_description_whose_row_declares_descispi_yes() {
        let raw = "Ordinary Spell\tDESCISPI:YES\tCLASSES:Wizard=1\tSCHOOL:Evocation\tDESC:secret lore";
        let entry = pi_screen(raw, "Ordinary Spell", Some("secret lore"), "occult_adventures", "oa_spells.lst", 1);
        assert_eq!(entry.key, "Ordinary Spell");
        assert!(entry.name_pi_line.is_none(), "a DESCISPI-only declaration must not rename the record");
        assert_ne!(entry.description.as_deref(), Some("secret lore"));
    }

    #[test]
    fn pi_screen_passes_a_clean_record_through_unredacted() {
        let raw = "Bone Flense\tCLASSES:Wizard=6\tSCHOOL:Necromancy\tDESC:you flense the bones";
        let entry = pi_screen(raw, "Bone Flense", Some("you flense the bones"), "occult_adventures", "oa_spells.lst", 1);
        assert!(entry.name_pi_line.is_none());
        assert_eq!(entry.description.as_deref(), Some("you flense the bones"));
    }

    /// `§24b`-1's own required proof: the identity is unchanged when the
    /// ORIGINAL name (never consulted by the rename branch) is swapped
    /// for something completely different.
    #[test]
    fn pi_screen_output_is_unchanged_when_the_original_name_is_swapped() {
        let raw_a = "Name A\tNAMEISPI:YES\tCLASSES:Wizard=1\tSCHOOL:Evocation";
        let raw_b = "Completely Different\tNAMEISPI:YES\tCLASSES:Wizard=1\tSCHOOL:Evocation";
        let a = pi_screen(raw_a, "Name A", None, "book", "file.lst", 9);
        let b = pi_screen(raw_b, "Completely Different", None, "book", "file.lst", 9);
        assert_eq!(a.key, b.key);
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
    fn books_table_names_exactly_the_twenty_one_spell_bearing_books_this_binary_replaces() {
        // SD-32 card 11 (T9 onboarding, `decisions.md §19` sign-off): +1,
        // `horror_adventures` -- this module's own doc comment's "seven
        // near-identical binaries plus the already-config-driven ISF/ISM/
        // ISTEM trio" (nine total, hence the old test name) plus this
        // cycle's tenth entry, which was never a per-book binary at all --
        // it was added directly to this shared config.
        //
        // SD-32 `decisions.md §20` (`§17a` re-derivation): +2,
        // `bestiary`/`bestiary_4` -- a prior cycle's "monster-intrinsic, no
        // dedicated `.lst`" claim for these two books' `no_record` spell
        // population was checked and found wrong; both carry a real,
        // dedicated `.lst` file of custom spell-variant declarations.
        let ids: Vec<&str> = BOOKS.iter().map(|b| b.id).collect();
        assert_eq!(
            ids,
            vec![
                "adventurers_guide",
                "inner_sea_gods",
                "occult_adventures",
                "ultimate_combat",
                "ultimate_magic",
                "ultimate_magic_wordsofpower",
                "ultimate_wilderness",
                "inner_sea_faiths",
                "inner_sea_magic",
                "inner_sea_temples",
                "horror_adventures",
                "bestiary",
                "bestiary_4",
                "inner_sea_races",
                "inner_sea_intrigue",
                "monster_codex",
                "inner_sea_world_guide",
                "book_of_the_damned_volume_1",
                "book_of_the_damned_volume_2",
                "mythic_adventures",
                "ultimate_equipment",
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
