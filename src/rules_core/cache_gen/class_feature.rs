//! `class_feature` JSON cache generator (SD-31 `epic-5-chassis-sweep` F1,
//! `SD31-E5-F1-001`).
//!
//! Writes `data/corpus/<book>/class_feature/<class-slug>/<feature-slug>.json`
//! for every `class_feature` unit `v06_work_inventory` already enumerates
//! from a book's PRIMARY `*_abilities_class.lst` file (see
//! [`BOOK_PRIMARY_FILES`] below for the exact scope this cycle covers, and
//! why).
//!
//! ## Why this generator is NOT a `decisions.md §11.3` Rust-table dump
//!
//! Every prior `cache_gen::*` module (`acg`, `apg`, `beastiary1`,
//! `ultimate_equipment`) dumps an already-completed, hand-built
//! `rules_tables::<book>` Rust module that carries every record's real
//! field values -- `§11.3`'s "dump from the completed Rust module, do not
//! re-parse raw LST from scratch" applies to exactly that shape. **No such
//! module exists for `class_feature`.** `grep -rl
//! 'class_feature\|ClassFeature' src/rules_core/rules_tables/` (re-run this
//! cycle) finds only scattered class-CHASSIS mechanism code (Fighter Weapon
//! Training bonuses in `crb/weapon_tables.rs`, four Pathfinder Unchained
//! per-class feature files) -- never a per-record data table naming every
//! class feature's key/description/citation the way
//! `ultimate_equipment::equipment_tables` does for equipment. There is
//! nothing to dump.
//!
//! `decisions.md §11.3`'s own text anticipates this exact case: a generic
//! LST-token-to-JSON path is "well-suited to bulk extraction of
//! well-formed corpus tokens... the shape of [building] a cache from
//! scratch." That is what this generator does, and ONLY that: for each
//! unit's already-known `(book, source_file, source_line, key, name)`
//! citation -- established by `v06_work_inventory`'s own enumeration,
//! never re-derived here -- it reads the real corpus row and TRANSCRIBES
//! its tab-delimited fields into `data.raw_tokens`, the same pure
//! byte-for-byte transcription `enrich_equipment_raw_tokens.rs` and
//! `enrich_spell_raw_tokens.rs` already perform for their kinds. No field
//! value is invented, computed, or interpreted -- every token is copied
//! verbatim from the cited line, and [`corpus_literal_sweep`] independently
//! re-derives the same closure from the same citation to confirm the copy
//! byte for byte.
//!
//! ## Scope: every `*abilities_class*.lst` file per book (`BOOK_PRIMARY_FILES`)
//!
//! `v06_work_inventory::enumerate_book` walks a book's ENTIRE directory
//! tree recursively, so some books' `class_feature` population spans not
//! only their own primary `<abbrev>_abilities_class.lst` but also nested
//! `support/*_abilities_class_*.lst` and `_pfs/*.lst` cross-book variant
//! files (e.g. `ultimate_combat/support/uc_abilities_class_um.lst`). An
//! earlier version of this generator scoped ONLY the primary file per
//! book and left every nested variant `no_record` -- 3,333 of the 5,604
//! `no_record` `class_feature` units (decisions.md §20) lived exactly
//! there, all matching the same `*abilities_class*.lst` naming convention
//! the primary files use. This cycle widens scope: [`units_from_inventory_json`]
//! now accepts any `class_feature` unit of a [`BOOK_PRIMARY_FILES`] book
//! whose `source_file` matches that convention (case-sensitive substring
//! `"abilities_class"`), not only the book's own listed primary file, and
//! [`generate`] resolves each unit's real file path with
//! [`resolve_book_file`] (a recursive basename search under the book's
//! directory, the same shape `wiring_class::resolve_corpus_file` already
//! uses for `.lst` line reads) instead of assuming the primary file's flat
//! `<book-dir>/<file>` join. Every record this generator writes cites the
//! REAL relative path it read from (`<book-dir>/<file>` for a primary file,
//! `<book-dir>/support/<file>` for a nested one) -- see the module's
//! `ultimate_psionics` section above for the one confirmed downstream
//! consequence of a nested citation (a stale `--json-out` book-attribution
//! bug, `OPEN-ISSUES.md` row 22, that blocks `literal-verified` stamping
//! but not `shape_ledger.py`'s join).
//! `pathfinder_unchained` WAS excluded entirely because it already carries
//! 64 hand-curated `class_feature` records from earlier mechanism-wiring
//! cycles (`barbarian_unchained_class/`, `monk_unchained_class/`, ...), a
//! different schema (`data.class_key`/`base_class_key`, not this
//! generator's `data.class`) written through a different code path. That
//! exclusion left 536 OTHER `pathfinder_unchained` `class_feature` units
//! `no_record` -- units the hand-curation never touched at all, not units
//! it disagrees with. This cycle brings the book back into scope and
//! protects those 64 records with [`foreign_citations`] (checked per unit,
//! module doc comment above it) instead of excluding the whole book: a
//! citation a foreign record already covers is skipped, never
//! double-written or overwritten; every other citation is ingested exactly
//! like any other book's.
//!
//! `ultimate_psionics` WAS excluded (decisions.md §20 wave): the module
//! doc comment used to say `corpus_literal_sweep::book_dir_of`
//! hard-required a 5-segment `source.path`, breaking Dreamscarred Press's
//! 4-segment `pathfinder/dreamscarred_press/<book>/<file>` layout. That was
//! true when written; it no longer is. `book_dir_of` (`src/bin/
//! corpus_literal_sweep.rs`) already special-cases a 4-segment
//! `dreamscarred_press` path (`014f210b9`, landed before this exclusion's
//! own commit), so the finding this exclusion cited is stale. Re-verified
//! this cycle: `up_abilities_class.lst` lives at
//! `pathfinder/dreamscarred_press/ultimate_psionics/up_abilities_class.lst`
//! (4 segments, the branch `book_dir_of` now handles), and adding it to
//! [`BOOK_PRIMARY_FILES`] below closes 1,573 of the 5,604 `no_record`
//! `class_feature` units decisions.md §20 named. **Nested support-file
//! scope note (this cycle):** unlike `corpus_literal_sweep::book_dir_of`
//! (whose `>= 5 segments -> first 4` rule tolerates arbitrary extra
//! nesting), `corpus_literal_sweep`'s SEPARATE `--json-out` writer derives
//! a verified triple's `"book"` field from `source_path.parent().file_name()`
//! (`OPEN-ISSUES.md` SD-31 row 22, `src/bin/corpus_literal_sweep.rs:267-276`,
//! a file this cycle does not edit) -- so a nested-support-file record's
//! `--json-out` "book" comes out as `"support"`, not the real book, and
//! `v06_work_inventory`'s `apply_done_rung_stamps` join on
//! `(book, file, line)` will not match it for `literal-verified` stamping.
//! This does NOT block `shape_ledger.py`'s join (it keys off the OUTPUT
//! JSON's own directory + `source.path`'s basename + `source.line`, never
//! off `--json-out`'s book field) -- so a nested-support-file record's
//! shape is genuinely measured, closing its `no_record` gap, even though
//! its `literal-verified` rung is blocked on the same pre-existing row-22
//! defect every other nested-file kind already carries. Named, not hidden.
//!
//! ## PI screening -- both SD-30 invocation contracts, on NAME and
//! DESCRIPTION (`decisions.md §52.3` / `§53.5`)
//!
//! `cache_gen::ultimate_equipment`'s confirmed hole
//! (`OPEN-ISSUES.md` row 38): it computes `DeclaredProductIdentity.name`
//! but only ever threads `.description` into the screen, silently
//! dropping the name half. This generator reads BOTH halves
//! ([`declared_pi_at`]) and, per `pi_screening.rs`'s own doc comment ("a
//! name cannot be redacted... the only way not to publish it is not to
//! publish the row"), **a record whose row declares `NAMEISPI:YES` is not
//! written at all** -- the safer default absent an operator ruling on a
//! per-book override (`docs/governance/ogl-pi-blacklist.md` §3), counted
//! in [`GenerationReport::name_pi_skipped`] rather than silently dropped.
//!
//! **Wave-4 correction (`SD31-W4-INTEGRATE-001`, `OPEN-ISSUES.md` row 48):**
//! the wave-3 hole this section describes fixing is `§53.5` (the declared
//! `NAMEISPI:`/`DESCISPI:` reader) ONLY. This generator's own first landed
//! version carried the identical hole one level over: it ran `§52.3`'s
//! bounded blacklist term scan (`pi_screening::classify_field`) on
//! `description` but never on `name` -- so a name containing a blacklisted
//! term with NO `NAMEISPI:YES` declaration on its row shipped unredacted.
//! 14 shipped records were exposed this way (2 with no PI marking on the
//! record at all); fixed by running `classify_field("name", ...)` on the
//! same union basis `equipment_gap.rs` already established as the correct
//! pattern (`declared.name || name_license == PiRedacted` => whole-record
//! skip, `name_pi_skipped` incremented) -- see the module's own doc comment
//! there for why a name has no field-level redaction path to fall back to.
//! `description` still runs the union screen
//! (`pi_screening::classify_optional_field_declared`) exactly as every
//! other generator does. Real, non-hypothetical stakes: this cycle's own
//! re-derivation found `adventurers_guide/ag_abilities_class.lst` alone
//! carries 49 `NAMEISPI:YES` and 269 `DESCISPI:YES` declarations --
//! `grep -oE '(NAMEISPI|DESCISPI):[A-Za-z]+' .../adventurers_guide/ag_abilities_class.lst | sort | uniq -c`.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Serialize;
use serde_json::Value;

use crate::rules_core::cache_gen::WiringClassIndex;
use crate::rules_core::codex_neutral_name::{neutral_key, neutral_name};
use crate::rules_core::corpus_literal_sweep::tab_tokens;
use crate::rules_core::pi_screening::{self, DeclaredProductIdentity, PI_BLACKLIST_TERMS};

/// `(book id, corpus-relative directory, primary `_abilities_class.lst`
/// basename)` for every one of the 23 in-scope `class_feature` books.
/// `ultimate_psionics` and `pathfinder_unchained` were both excluded
/// through the prior cycle -- the first on a `book_dir_of` finding that has
/// since gone stale, the second on a hand-curation conflict now handled per
/// unit by [`foreign_citations`] instead of by leaving the whole book out
/// (module doc comment's `ultimate_psionics` and `pathfinder_unchained`
/// sections). Both are back in scope as of this cycle. The third column names
/// each book's own PRIMARY file only -- [`units_from_inventory_json`] and
/// [`generate`] also pick up that book's nested `support/*abilities_class*.lst`
/// variants (module doc comment's "Scope" section), so this column is a
/// directory anchor and a primary-file fallback, not the full file list.
/// Re-derived this cycle directly against the pinned oracle checkout, one
/// book at a time: `find "$PCGEN_CORPUS_ROOT/pathfinder" -iname '<book>'
/// -type d` then `ls` that directory for its own `*_abilities_class*.lst`.
pub const BOOK_PRIMARY_FILES: &[(&str, &str, &str)] = &[
    ("advanced_class_guide", "pathfinder/paizo/roleplaying_game/advanced_class_guide", "acg_abilities_class.lst"),
    ("advanced_players_guide", "pathfinder/paizo/roleplaying_game/advanced_players_guide", "apg_abilities_class.lst"),
    ("ultimate_combat", "pathfinder/paizo/roleplaying_game/ultimate_combat", "uc_abilities_class.lst"),
    ("ultimate_magic", "pathfinder/paizo/roleplaying_game/ultimate_magic", "um_abilities_class.lst"),
    ("occult_adventures", "pathfinder/paizo/roleplaying_game/occult_adventures", "oa_abilities_class.lst"),
    ("core_rulebook", "pathfinder/paizo/roleplaying_game/core_rulebook", "cr_abilities_class.lst"),
    ("ultimate_wilderness", "pathfinder/paizo/roleplaying_game/ultimate_wilderness", "uw_abilities_class.lst"),
    ("ultimate_intrigue", "pathfinder/paizo/roleplaying_game/ultimate_intrigue", "ui_abilities_class.lst"),
    ("adventurers_guide", "pathfinder/paizo/roleplaying_game/adventurers_guide", "ag_abilities_class.lst"),
    ("advanced_race_guide", "pathfinder/paizo/roleplaying_game/advanced_race_guide", "arg_abilities_class.lst"),
    ("horror_adventures", "pathfinder/paizo/roleplaying_game/horror_adventures", "ha_abilities_class.lst"),
    ("inner_sea_combat", "pathfinder/paizo/campaign_setting/inner_sea_combat", "isc_abilities_class.lst"),
    ("inner_sea_magic", "pathfinder/paizo/campaign_setting/inner_sea_magic", "ism_abilities_class.lst"),
    ("book_of_the_damned_volume_2", "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_2", "botd2_abilities_classes.lst"),
    ("inner_sea_world_guide", "pathfinder/paizo/campaign_setting/inner_sea_world_guide", "iswg_abilities_class.lst"),
    ("inner_sea_intrigue", "pathfinder/paizo/campaign_setting/inner_sea_intrigue", "isi_abilities_class.lst"),
    ("monster_codex", "pathfinder/paizo/roleplaying_game/monster_codex", "mc_abilities_class.lst"),
    ("bestiary_6", "pathfinder/paizo/roleplaying_game/bestiary_6", "b6_abilities_class.lst"),
    ("inner_sea_taverns", "pathfinder/paizo/campaign_setting/inner_sea_taverns", "istav_abilities_class.lst"),
    ("book_of_the_damned_volume_1", "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_1", "botd1_abilities_class.lst"),
    ("bestiary_4", "pathfinder/paizo/roleplaying_game/bestiary_4", "b4_abilities_class.lst"),
    ("ultimate_psionics", "pathfinder/dreamscarred_press/ultimate_psionics", "up_abilities_class.lst"),
    ("pathfinder_unchained", "pathfinder/paizo/roleplaying_game/pathfinder_unchained", "pu_abilities_class.lst"),
];

/// Substring every `class_feature` book's own `*abilities_class*.lst`
/// primary and support files share, per this module's corpus sampling
/// (module doc comment "Scope" section) -- 56 distinct filenames, 100% of
/// them containing this substring, re-derived this cycle from every
/// `no_record` `class_feature` unit's `source_file`
/// (`python3 -c "..."` over `docs/work-inventory.json`, see the cycle
/// receipt for the exact command). Used by [`units_from_inventory_json`] to
/// widen scope from "the book's one listed primary file" to "every file of
/// this shape the book has", without guessing at a filename this module has
/// never seen.
const ABILITIES_CLASS_FILE_SUBSTRING: &str = "abilities_class";

/// `(book, source_file)` pairs that carry genuine `class_feature`-shaped
/// rows OUTSIDE the `*abilities_class*.lst` naming convention -- found this
/// cycle (`t9-onboarding`) as the true cause of `class_feature`'s last 25
/// `no_record` units (25 = 15 `advanced_players_guide` + 10
/// `advanced_class_guide`; `decisions.md §20`/`§17a`), NOT a stale record
/// or an already-covered coordinate (verified: no corpus record of any
/// kind exists at these 25 `(book, source_file, source_line)` triples
/// before this cycle's write). Both books also have their own primary
/// `*_abilities_class.lst` file already in scope via
/// [`ABILITIES_CLASS_FILE_SUBSTRING`] above; `acg_abilities_race.lst` /
/// `apg_abilities_race.lst` are a SECOND file per book that PCGen's own
/// authors mixed favored-class-bonus rows into (Skald/Inquisitor/Oracle
/// bonus-spell-known progressions, one Warpriest favored-class blessing
/// counter) alongside the file's otherwise-race-ability content -- direct
/// read of both files at the cited lines confirms real class-feature
/// tokens (`CATEGORY:Special Ability`, a `TYPE:Bonus*`/`TYPE:<Class> Class
/// Feature...` facet, `BONUS:SPELLKNOWN`/`BONUS:VAR` or a `DEFINE:`
/// counter), not a race trait. `v06_work_inventory.rs`'s census already
/// typed these correctly as `kind: class_feature`
/// (`units_from_inventory_json`'s `kind` filter above trusts that, not
/// re-derived here) -- the ONLY gap was this generator's file-scope list
/// never including the second file. An explicit pair list, not a broadened
/// substring match, so no other book's `*_abilities_race.lst` (genuinely
/// race content) is swept in without the same per-file verification.
const EXTRA_CLASS_FEATURE_SOURCE_FILES: &[(&str, &str)] = &[
    ("advanced_class_guide", "acg_abilities_race.lst"),
    ("advanced_players_guide", "apg_abilities_race.lst"),
];

/// One `class_feature` unit as `v06_work_inventory`'s own enumeration
/// already established it -- this generator never re-derives `key`/`name`/
/// the citation, only reads the line they already cite.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassFeatureSourceUnit {
    pub book: String,
    pub source_file: String,
    pub source_line: u32,
    pub key: String,
    pub name: String,
    /// The unit's `type_facet` field (PCGen's dot-delimited `TYPE:` token,
    /// e.g. `"Barbarian Class Feature.Skald Class Feature.Rage Power..."`),
    /// when `v06_work_inventory.rs`'s own enumeration recorded one. Read
    /// only by [`type_facet_dispatched_owner`] / [`type_facet_corpus_owner`]
    /// (SD-32 card 11, T2a/T12 combined cycle) as a second and fourth
    /// fallback in [`generate`]'s class-derivation chain -- never affects
    /// which line is read or where the record is written.
    pub type_facet: Option<String>,
}

/// Parses `units_from_inventory_json`'s input: every `kind == "class_feature"`
/// entry of a `docs/work-inventory.json`-shaped document, restricted to
/// [`BOOK_PRIMARY_FILES`]' books and each book's own primary file (the
/// module doc comment's scope note). Never touches the filesystem; pure
/// parsing of already-computed fields.
pub fn units_from_inventory_json(json_text: &str) -> Result<Vec<ClassFeatureSourceUnit>, String> {
    let doc: Value = serde_json::from_str(json_text).map_err(|e| format!("invalid inventory JSON: {e}"))?;
    let known_books: BTreeSet<&str> = BOOK_PRIMARY_FILES.iter().map(|(book, _, _)| *book).collect();
    let units = doc
        .get("units")
        .and_then(Value::as_array)
        .ok_or_else(|| "inventory JSON has no top-level `units` array".to_string())?;
    let mut out = Vec::new();
    for unit in units {
        if unit.get("kind").and_then(Value::as_str) != Some("class_feature") {
            continue;
        }
        let Some(book) = unit.get("book").and_then(Value::as_str) else { continue };
        if !known_books.contains(book) {
            continue;
        }
        let Some(source_file) = unit.get("source_file").and_then(Value::as_str) else { continue };
        // Widened scope (this cycle): any `*abilities_class*.lst` file of
        // this book, not only its own listed primary file -- module doc
        // comment's "Scope" section. `generate` resolves the real path
        // (primary or nested `support/`) via [`resolve_book_file`].
        // Further widened (t9-onboarding cycle): the small, explicit
        // [`EXTRA_CLASS_FEATURE_SOURCE_FILES`] allowlist admits the two
        // known `*_abilities_race.lst` files that also carry genuine
        // class-feature rows -- see that constant's doc comment.
        let in_scope = source_file.contains(ABILITIES_CLASS_FILE_SUBSTRING)
            || EXTRA_CLASS_FEATURE_SOURCE_FILES.contains(&(book, source_file));
        if !in_scope {
            continue;
        }
        let Some(source_line) = unit.get("source_line").and_then(Value::as_u64) else { continue };
        let Some(key) = unit.get("corpus_key").and_then(Value::as_str) else { continue };
        let Some(name) = unit.get("name").and_then(Value::as_str) else { continue };
        let type_facet = unit.get("type_facet").and_then(Value::as_str).map(str::to_string);
        out.push(ClassFeatureSourceUnit {
            book: book.to_string(),
            source_file: source_file.to_string(),
            source_line: source_line as u32,
            key: key.to_string(),
            name: name.to_string(),
            type_facet,
        });
    }
    Ok(out)
}

/// Every `kind == "class"` unit's `name`, keyed by its lowercase form
/// mapped to the corpus's own natural-case spelling (`"vigilante"` ->
/// `"Vigilante"`) -- the SAME population `v06_work_inventory.rs`'s
/// `corpus_class_names` fact builds (its own doc comment: "Every class
/// name the corpus declares anywhere, so a class feature of an
/// un-ingested class ... is reported as a real `not-ingested` gap rather
/// than as an unclassifiable mystery"), read here from the same
/// already-committed `docs/work-inventory.json` rather than re-walking
/// raw PCGen `*_class.lst` files a second time (SD-32 card 11, T2a/T12
/// combined cycle -- see [`generate`]'s own doc comment for why this
/// population matters to `data.class`, not just to the census).
pub fn corpus_class_names_from_inventory_json(json_text: &str) -> Result<BTreeMap<String, String>, String> {
    let doc: Value = serde_json::from_str(json_text).map_err(|e| format!("invalid inventory JSON: {e}"))?;
    let units = doc
        .get("units")
        .and_then(Value::as_array)
        .ok_or_else(|| "inventory JSON has no top-level `units` array".to_string())?;
    let mut out = BTreeMap::new();
    for unit in units {
        if unit.get("kind").and_then(Value::as_str) != Some("class") {
            continue;
        }
        if let Some(name) = unit.get("name").and_then(Value::as_str) {
            out.insert(name.to_lowercase(), name.to_string());
        }
    }
    Ok(out)
}

// ---------------------------------------------------------------------
// Shape B schema -- own local types, per `decisions.md §11.3`'s
// disjoint-file-touch convention every `cache_gen::*` module already
// follows (no shared struct file).
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Population {
    InScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Completeness {
    ChassisOnly,
    Full,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Source {
    LstToken { path: String, sha256: String, line: u32, record_key: String },
}

#[derive(Debug, Clone, Serialize)]
pub struct RawToken {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ClassFeatureData {
    pub key: String,
    pub name: String,
    /// The owning class/talent-pool name, split off `key`'s ` ~ ` separator
    /// -- a pure parse of the record's own already-established key, the
    /// same split `class_feature_owner`/`Kind::ClassFeature`'s classify arm
    /// already perform; never a new value.
    pub class: Option<String>,
    /// SD-32 card 11, decision 23a's genuinely-multi-owner shape (`"Domain
    /// Power"`, 172 records): the set of classes whose own domain-access
    /// mechanism the oracle proves grants this specific power, resolved by
    /// [`domain_power_owning_classes`] reading the corpus itself -- never
    /// hand-authored. `None` when this record's `key` is not a `"Domain
    /// Power ~ <X>"` key at all, or when no upstream grant chain was found
    /// for it (an honest gap, not a guess). Deliberately a SEPARATE field
    /// from `class` (not a collapse into it): `class` keeps its existing
    /// single-owner meaning for every other resolution tier, and this field
    /// is the only place a record legitimately says "more than one class
    /// grants me" -- `CATEGORY_LABEL_ALIASES`' single-label-to-single-class
    /// shape does not fit here and must not be forced (`decisions.md §1a`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classes: Option<Vec<String>>,
    pub description: Option<String>,
    pub raw_tokens: Vec<RawToken>,
}

/// `decisions.md §24b`-4: divergence recorded as coordinate + reason only,
/// never the original PI string. Mirrors `ability`'s own `data.rename`
/// shape (`scripts/ingest_ability.py`) so a renamed record looks the same
/// regardless of which generator produced it.
#[derive(Debug, Clone, Serialize)]
pub struct RenameInfo {
    pub reason: String,
    pub coordinate: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CacheRecord {
    pub population: Population,
    pub completeness: Completeness,
    pub ingested_at: String,
    pub data: ClassFeatureData,
    pub source: Source,
    pub wiring_class: String,
    pub wiring_class_signals: Vec<String>,
    pub license: crate::rules_core::shape_b_v1::License,
    pub pi_field: Option<String>,
    pub pi_marker: Option<String>,
    /// `decisions.md §24b`-3: "a field marks it as carrying a
    /// Codex-generated name, so no reader or player mistakes it for the
    /// printed name."
    pub codex_generated_name: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename: Option<RenameInfo>,
}

#[derive(Debug, Default)]
pub struct GenerationReport {
    pub written: usize,
    /// Records whose row declares `NAMEISPI:YES` (or hits the name
    /// blacklist) -- `decisions.md §24` renames and ships these under a
    /// Codex-generated neutral name (`codex_neutral_name::neutral_name`)
    /// rather than skipping them. This counter (name kept for the existing
    /// `gen_cache_class_feature` printed message) now counts RENAMES, not
    /// silent drops -- every one of these units is still `written` above.
    pub name_pi_skipped: usize,
    /// `(kind, book, source_file, source_line, codex_name, reason)`
    /// divergence entries for every unit renamed this run --
    /// `decisions.md §24b`-4: coordinate + reason, never the original
    /// string. Consumed by `gen_cache_class_feature`'s report writer.
    pub name_pi_renamed_records: Vec<serde_json::Value>,
    /// `(book, source_file, source_line)` citations that did not resolve
    /// to a real corpus line -- should be empty against the real corpus,
    /// since every citation here was already validated by
    /// `v06_work_inventory`'s own enumeration.
    pub unresolved_citations: Vec<String>,
    pub books_written: BTreeSet<String>,
    /// Units skipped because [`foreign_citations`] found a hand-authored
    /// (non-generic) record already citing that exact line -- protects
    /// `pathfinder_unchained`'s 64 hand-curated records now that the book
    /// is back in scope (this cycle).
    pub foreign_citation_skipped: usize,
}

#[derive(Debug)]
pub enum GenerationError {
    Io(std::io::Error),
    CorpusUnreachable(PathBuf),
}

impl From<std::io::Error> for GenerationError {
    fn from(e: std::io::Error) -> Self {
        GenerationError::Io(e)
    }
}

/// Maximum directory depth [`resolve_book_file`] descends below a book's
/// own directory -- mirrors `wiring_class::MAX_NESTED_LST_DEPTH`; this
/// module's real corpus never nests an `abilities_class` file more than
/// one level deep (`support/`), but the extra headroom costs nothing.
const MAX_BOOK_FILE_DEPTH: usize = 3;

/// Finds `file`'s real path under `book_dir`: first the flat
/// `book_dir/file` join every primary file uses, then (this cycle's
/// widened scope) a recursive basename search for a nested variant like
/// `book_dir/support/file`. Returns `None` if `file` is not present under
/// `book_dir` at all, or if more than one file shares that basename (never
/// guesses between two real matches -- the same refusal
/// `wiring_class::resolve_corpus_file` makes for the identical shape,
/// which [`generate`]'s own `lines.line(...)` call already goes through).
fn resolve_book_file(book_dir: &Path, file: &str) -> Option<PathBuf> {
    let direct = book_dir.join(file);
    if direct.is_file() {
        return Some(direct);
    }
    let mut matches: Vec<PathBuf> = Vec::new();
    let mut stack: Vec<(PathBuf, usize)> = vec![(book_dir.to_path_buf(), 0)];
    while let Some((dir, depth)) = stack.pop() {
        if depth > MAX_BOOK_FILE_DEPTH {
            continue;
        }
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push((path, depth + 1));
            } else if path.file_name().and_then(|n| n.to_str()) == Some(file) {
                matches.push(path);
            }
        }
    }
    match matches.len() {
        1 => matches.pop(),
        _ => None,
    }
}

pub fn sha256_file(path: &Path) -> std::io::Result<String> {
    let output = Command::new("sha256sum").arg(path).output()?;
    if !output.status.success() {
        return Err(std::io::Error::other(format!("sha256sum failed for {}", path.display())));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    Ok(text.split_whitespace().next().unwrap_or_default().to_string())
}

fn slugify(name: &str, used: &mut BTreeSet<String>) -> String {
    let mut slug: String = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    while slug.contains("__") {
        slug = slug.replace("__", "_");
    }
    let slug = slug.trim_matches('_').to_string();
    let slug = if slug.is_empty() { "unnamed".to_string() } else { slug };
    if !used.contains(&slug) {
        used.insert(slug.clone());
        return slug;
    }
    let mut n = 2;
    loop {
        let candidate = format!("{slug}-{n}");
        if !used.contains(&candidate) {
            used.insert(candidate.clone());
            return candidate;
        }
        n += 1;
    }
}

/// Reads [`DeclaredProductIdentity`] off the real corpus line at
/// `lst_path:line` (1-indexed), matching `cache_gen::ultimate_equipment`'s
/// `declared_pi_at` -- reproduced locally rather than shared, per this
/// package's disjoint-file-touch convention for `cache_gen::*` modules.
fn declared_pi_at(lst_path: &Path, line: u32) -> std::io::Result<DeclaredProductIdentity> {
    if line == 0 {
        return Ok(DeclaredProductIdentity::default());
    }
    let content = std::fs::read_to_string(lst_path)?;
    let Some(row) = content.lines().nth((line - 1) as usize) else {
        return Ok(DeclaredProductIdentity::default());
    };
    let tokens: Vec<(&str, &str)> = row.split('\t').filter_map(|field| field.split_once(':')).collect();
    Ok(pi_screening::declared_product_identity(tokens))
}

/// One raw `.lst` row's own tab-delimited tokens as `{key, value}` pairs
/// (field 0, the record's identity column, is never a token -- matches
/// `corpus_literal_sweep::tab_tokens`'s own `skip(1)`). Pure transcription:
/// every pair is copied verbatim from the row, nothing computed.
fn row_tokens(row: &str) -> Vec<RawToken> {
    tab_tokens(row)
        .into_iter()
        .filter_map(|field| field.split_once(':'))
        .map(|(k, v)| RawToken { key: k.to_string(), value: v.to_string() })
        .collect()
}

fn desc_value(tokens: &[RawToken]) -> Option<String> {
    tokens.iter().find(|t| t.key == "DESC").map(|t| t.value.clone())
}

/// One resolved fact from `cache_gen::class_feature_grants`' own output
/// (`data/class_feature_grants/<book>/<class-slug>.json`) -- read here as
/// plain data, never through that module's own types, per this package's
/// disjoint-file-touch convention (module doc comment). Only the two
/// fields this generator needs are extracted; every other field on the
/// real record (`level`, `level_explicit`, `gate`, `corpus_record_exists`,
/// `source`) is ignored.
#[derive(Debug, Clone, serde::Deserialize)]
struct GrantFactClassOnly {
    key: String,
    class: String,
}

/// Builds the `key -> true granting class` map for one book from every
/// `data/class_feature_grants/<book>/*.json` file, so [`generate`] can
/// correct `ClassFeatureData.class` instead of deriving it from the key's
/// own text (`OPEN-ISSUES.md` row 334's closing note; wave 22's
/// `class_feature_grants.rs` module doc comment, "A related, pre-existing,
/// OUT-OF-SCOPE defect this cycle found and did NOT fix").
///
/// A missing `grants_root/<book>` directory (5 of the 21
/// [`BOOK_PRIMARY_FILES`] books have no grant data yet) returns an empty
/// map -- every record in that book falls back to the old key-prefix
/// split, unchanged.
///
/// **Ambiguity is refused, not guessed.** If two different grant facts in
/// the same book claim the SAME `key` under DIFFERENT classes, that key is
/// left OUT of the map entirely (falls back to the key-prefix split) rather
/// than picking one arbitrarily -- verified against the real corpus this
/// cycle to occur zero times today (`python3` cross-check, see cycle
/// receipt), but a future grant-data regen could introduce one, and a
/// silent pick would be exactly the "correctness proof narrower than the
/// real data" shape this program's own history (waves 20/21) keeps
/// punishing.
fn true_class_by_key(grants_root: &Path, book: &str) -> BTreeMap<String, String> {
    let book_dir = grants_root.join(book);
    let mut resolved: BTreeMap<String, String> = BTreeMap::new();
    let mut ambiguous: BTreeSet<String> = BTreeSet::new();
    let Ok(entries) = std::fs::read_dir(&book_dir) else {
        return resolved;
    };
    let mut files: Vec<PathBuf> = entries.filter_map(|e| e.ok()).map(|e| e.path()).collect();
    files.sort();
    for file in files {
        if file.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&file) else { continue };
        let Ok(facts) = serde_json::from_str::<Vec<GrantFactClassOnly>>(&text) else { continue };
        for fact in facts {
            if ambiguous.contains(&fact.key) {
                continue;
            }
            match resolved.get(&fact.key) {
                None => {
                    resolved.insert(fact.key, fact.class);
                }
                Some(existing) if *existing == fact.class => {}
                Some(_) => {
                    resolved.remove(&fact.key);
                    ambiguous.insert(fact.key);
                }
            }
        }
    }
    resolved
}

// ---------------------------------------------------------------------
// SD-32 card 11 (`epic-2-cause-closure`, T2a/T12 combined cycle):
// two further class-derivation fallbacks, tried after `true_class_by_key`
// (grant facts) and before the raw key-prefix guess.
//
// `true_class_by_key` alone leaves `data.class` at the naive key-prefix
// split for any key with no grant fact -- which is exactly wrong whenever
// that prefix is a category-label OPTION POOL ("Rage Power", "Domain",
// "Mystery", ...) rather than a class's own name: T2a's own name for this
// shape ("`data.class` read from the wrong place"). `MEASURE-TWICE.md`
// (`docs/release/SD-31-corpus-closure-grind/artifacts/`) measured 8,243
// such records; this cycle's own re-derivation (cycle receipt) found the
// corpus-wide non-dispatched population had already shrunk to 5,678 by the
// time this cycle ran (the wave-22/23 `true_class_by_key` fix landing in
// between), and that within that 5,678, two ALREADY-PROVEN mechanisms
// `v06_work_inventory.rs`'s own `Kind::ClassFeature` classify arm already
// trusts for the SAME question (which real class does this option-pool
// record belong to?) resolve the true class name for a further ~2,964 of
// them without guessing:
//
// 1. [`pool_catalog_owner`] -- the registered option-pool table
//    (`CLASS_FEATURE_POOLS` in `v06_work_inventory.rs`), reproduced here
//    per this package's disjoint-file-touch convention (module doc
//    comment). Resolves a category label like "Rage Power" to its real
//    DISPATCHED owner ("Barbarian") -- true T2a plumbing, closed at the
//    cause.
// 2. [`type_facet_dispatched_owner`] -- the corpus's own `TYPE:` token
//    frequently spells the owning class out literally as a
//    `"<Class> Class Feature(s)"` taxonomy segment even when the group
//    prefix does not name it; also reproduced from
//    `v06_work_inventory.rs`'s `class_feature_type_facet_owner_candidates`.
//
// A THIRD and FOURTH tier below extend both mechanisms to match against
// the full **corpus-declared** class roster ([`corpus_class_names_from_
// inventory_json`]), not only the 34 dispatched ones -- this is what
// closes the T2a/T12 OVERLAP correctly: a record whose true class is
// "Vigilante" (corpus-declared, engine-undispatched) now ships
// `data.class: "Vigilante"` instead of the category label "Vigilante
// Talent", which is honest either way this record's true class is later
// modelled or not, and stops it from being counted as an unexplained
// mystery. See [`generate`]'s own resolution-order comment for how all
// four combine with `true_class_by_key` and the raw fallback.
// ---------------------------------------------------------------------

/// Mirrors `v06_work_inventory.rs::CLASS_FEATURE_POOLS` exactly (registered
/// group word, real owning class, Title-Case as `data.class` itself is
/// written) -- reproduced locally per this package's disjoint-file-touch
/// convention. Cross-checked for drift by
/// `dispatched_class_title_names_len_matches_the_real_34_class_roster`.
const POOL_TO_DISPATCHED_CLASS: &[(&str, &str)] = &[
    ("Rage Power", "Barbarian"),
    ("Unchained Rage Power", "Unchained Barbarian"),
    ("Discovery", "Alchemist"),
    ("Grand Discovery", "Alchemist"),
    ("Rogue Talent", "Rogue"),
    ("Advanced Talents", "Rogue"),
    ("Hex", "Witch"),
    ("Revelation", "Oracle"),
    ("Mercy", "Paladin"),
    ("Investigator Talent", "Investigator"),
    ("Slayer Talent", "Slayer"),
    ("Judgment", "Inquisitor"),
    ("Inquisition", "Inquisitor"),
    ("Blessing", "Warpriest"),
    ("Evolution", "Summoner"),
    ("Bloodline", "Sorcerer"),
    ("Bloodrager Bloodline", "Bloodrager"),
    ("Domain", "Cleric"),
    ("Order", "Cavalier"),
    ("Mystery", "Oracle"),
    ("Curse", "Oracle"),
    ("Spirit", "Shaman"),
    ("Animal Focus", "Hunter"),
    ("Favored Enemy", "Ranger"),
    ("Favored Terrain", "Ranger"),
    ("Versatile Performance", "Bard"),
    ("Arcane School", "Wizard"),
    ("Focused Arcane School", "Wizard"),
];

/// Mirrors `v06_work_inventory.rs::CLASS_FEATURE_POOL_FALSE_SUFFIX_MATCHES`
/// -- groups whose textual shape satisfies the suffix rule below but whose
/// own corpus row proves they belong elsewhere (that file's own doc
/// comment on each row is the citation; reproduced verbatim here).
const POOL_FALSE_SUFFIX_MATCHES: &[&str] = &[
    "Heretical Revelation",
    "Shifter's Blessing",
    "Spider's Blessing",
    "Zevgavizeb's Blessing",
    "Totem Spirit",
    "Inspired Discovery",
    "Mutation Warrior Discovery",
    "Merciful Healer Mercy",
    "Take Inquisition",
];

/// Mirrors `v06_work_inventory.rs::CLASS_FEATURE_POOL_SLOT_QUALIFIERS`.
const POOL_SLOT_QUALIFIERS: &[&str] = &["wandering", "secondary", "major", "grand", "advanced", "unchained"];

/// The 34-class dispatched roster's Title-Case display spellings, used only
/// for [`pool_catalog_owner`]'s cross-class-collision guard and
/// [`type_facet_dispatched_owner`]'s match set. Count cross-checked against
/// the real enums by `dispatched_class_title_names_len_matches_the_real_34_class_roster`
/// rather than trusted as a hand-typed list on its own.
const DISPATCHED_CLASS_TITLE_NAMES: &[&str] = &[
    "Barbarian", "Bard", "Cleric", "Druid", "Fighter", "Monk", "Paladin", "Ranger", "Rogue", "Sorcerer", "Wizard",
    "Arcanist", "Bloodrager", "Brawler", "Hunter", "Investigator", "Shaman", "Skald", "Slayer", "Swashbuckler",
    "Warpriest", "Alchemist", "Cavalier", "Inquisitor", "Oracle", "Summoner", "Witch", "Gunslinger", "Ninja",
    "Samurai", "Unchained Barbarian", "Unchained Monk", "Unchained Rogue", "Unchained Summoner",
];

/// Mirrors `v06_work_inventory.rs::class_feature_pool_group_matches` exactly
/// (same three guards, same doc-comment reasoning), operating on Title-Case
/// text instead of the engine's own lowercase/underscored class ids.
fn pool_group_matches(registered: &str, owner: &str, group: &str) -> bool {
    if group == registered {
        return true;
    }
    let Some(prefix) = group.strip_suffix(&format!(" {registered}")) else {
        return false;
    };
    if prefix.is_empty() {
        return true;
    }
    if POOL_FALSE_SUFFIX_MATCHES.contains(&group) {
        return false;
    }
    prefix.split_whitespace().all(|token| {
        let normalized = token.trim_end_matches("'s").trim_end_matches('\'').to_ascii_lowercase();
        if POOL_SLOT_QUALIFIERS.contains(&normalized.as_str()) {
            return false;
        }
        let is_another_dispatched_class = DISPATCHED_CLASS_TITLE_NAMES
            .iter()
            .any(|c| c.eq_ignore_ascii_case(&normalized) && !c.eq_ignore_ascii_case(owner));
        !is_another_dispatched_class
    })
}

/// Resolves `group` (the key's own `" ~ "`-split prefix) against
/// [`POOL_TO_DISPATCHED_CLASS`], returning the real DISPATCHED owner's
/// Title-Case name when it matches -- `None` otherwise. Pure lookup, no
/// filesystem access.
fn pool_catalog_owner(group: &str) -> Option<&'static str> {
    POOL_TO_DISPATCHED_CLASS
        .iter()
        .find(|(registered, owner)| pool_group_matches(registered, owner, group))
        .map(|(_, owner)| *owner)
}

/// Mirrors `v06_work_inventory.rs::class_feature_type_facet_owner_candidates`:
/// every `"<Name> Class Feature(s)"` taxonomy segment a dot-delimited
/// `type_facet` string carries, in order.
fn type_facet_owner_candidates(type_facet: Option<&str>) -> Vec<String> {
    const MARKERS: [&str; 2] = [" Class Features", " Class Feature"];
    let Some(type_facet) = type_facet else {
        return Vec::new();
    };
    type_facet
        .split('.')
        .filter_map(|segment| {
            let segment = segment.trim();
            MARKERS.iter().find_map(|marker| segment.strip_suffix(marker))
        })
        .map(|name| name.trim().to_string())
        .filter(|name| !name.is_empty())
        .collect()
}

/// Resolves `type_facet`'s own class-name candidates against the 34
/// dispatched classes, returning the first that matches (exact or
/// whole-word prefix/suffix, matching `v06_work_inventory.rs::class_feature_
/// owner`'s own comparison shape).
fn type_facet_dispatched_owner(type_facet: Option<&str>) -> Option<String> {
    type_facet_owner_candidates(type_facet).into_iter().find_map(|candidate| {
        DISPATCHED_CLASS_TITLE_NAMES
            .iter()
            .find(|class| owner_text_matches(&candidate, class))
            .map(|class| class.to_string())
    })
}

/// Resolves `group` against the full corpus-declared class roster
/// (`corpus_class_names`, lowercase key -> natural-case value) -- the T2a/
/// T12 overlap fix: a record whose true owner is a real, corpus-declared
/// class the engine does not model (e.g. "Vigilante") gets that class's own
/// name instead of a category label, honestly, without claiming the class
/// is modelled.
fn corpus_class_owner(group: &str, corpus_class_names: &BTreeMap<String, String>) -> Option<String> {
    let group_lower = group.to_lowercase();
    corpus_class_names
        .iter()
        .filter(|(lower, _)| owner_text_matches(&group_lower, lower))
        .max_by_key(|(lower, _)| lower.len())
        .map(|(_, natural)| natural.clone())
}

/// Same as [`corpus_class_owner`] but tried against `type_facet`'s own
/// candidate names instead of the key's group prefix.
fn type_facet_corpus_owner(
    type_facet: Option<&str>,
    corpus_class_names: &BTreeMap<String, String>,
) -> Option<String> {
    type_facet_owner_candidates(type_facet)
        .into_iter()
        .find_map(|candidate| corpus_class_owner(&candidate, corpus_class_names))
}

/// Whole-word exact/prefix/suffix match, case-insensitive -- the same
/// comparison shape `v06_work_inventory.rs::class_feature_owner` uses
/// (`group == class`, `group.starts_with("{class} ")`,
/// `group.ends_with(" {class}")`), reproduced here for Title-Case text.
fn owner_text_matches(text: &str, class: &str) -> bool {
    let text_lower = text.to_lowercase();
    let class_lower = class.to_lowercase();
    text_lower == class_lower
        || text_lower.starts_with(&format!("{class_lower} "))
        || text_lower.ends_with(&format!(" {class_lower}"))
}

// ---------------------------------------------------------------------
// SD-32 card 11, T2a-residual cycle: a fifth resolution tier, ALIAS text
// that names neither a dispatched class ([`pool_catalog_owner`],
// [`type_facet_dispatched_owner`]) nor a `TYPE:`/`type_facet` string that
// literally contains the owning class's own name
// ([`corpus_class_owner`], [`type_facet_corpus_owner`]) -- e.g. "Ki Power"
// names no class at all in its own text, but every one of its 80 corpus
// records carries `PRE: 1,Monk=4`. `POOL_TO_DISPATCHED_CLASS` cannot
// widen to cover this: it is a suffix/prefix TEXT matcher, and there is no
// text relationship between "Ki Power" and "Monk" to match against.
//
// Each entry below was verified the same way `CLASS_FEATURE_POOLS`' own 27
// entries were: reading every one of the label's corpus records' `TYPE:`,
// `PRE*:`, `BONUS`, and `ABILITY` tokens (not a sample) and confirming
// they name exactly one class, with no cross-book or cross-class
// collision (`artifacts/gate-3-closure-invariant/card11-t2a-residual-
// alias-verification.md` carries the per-label evidence and re-derive
// commands). Unlike [`POOL_TO_DISPATCHED_CLASS`], the target may be a
// real corpus-declared class the engine does not yet dispatch
// (`"Kineticist"`, `"Occultist"`) -- this closes T2a-residual for those
// records without claiming T12's engine gap is also closed.
//
// A label is deliberately ABSENT from this table when the verification
// found it is NOT single-owner (`"Domain Power"`: `DomainLawLVL`-shaped
// `PRE:` tokens and its own DESC text are shared across every class with
// domain access -- Cleric, Inquisitor's Inquisition, Warpriest's
// Blessing-domain hybrid, Paladin's Sacred Servant archetype -- with no
// per-record token that says which one; forcing it to one class the way
// `"Rage Power" -> "Barbarian"` works would be the exact anti-gaming
// failure `decisions.md §1a` rules out, a relabelled shape, not a closed
// one) or NOT class-owned at all (`"Demonic Obedience"`: every `PRE:`
// token names a demon lord, never a class -- a deity-obedience feat line,
// structurally outside any class chassis). Both are reported open, with
// this reasoning, in the cycle receipt rather than silently mapped to
// shrink the count.
// ---------------------------------------------------------------------

/// Verified label -> real owning class, for category labels whose own
/// text names no class at all (see the section comment above). Applied
/// only via exact match on `group` (the key's `" ~ "`-split prefix) --
/// no suffix/prefix fuzzing, unlike [`POOL_TO_DISPATCHED_CLASS`], because
/// there is no shared text to fuzz.
const CATEGORY_LABEL_ALIASES: &[(&str, &str)] = &[
    ("Ki Power", "Monk"),
    ("Master of Many Styles", "Monk"),
    ("Maneuver Master", "Monk"),
    ("Wildcat", "Monk"),
    ("Pack Lord", "Druid"),
    ("Adaptation", "Ranger"),
    ("Favored Enemy Bonus", "Ranger"),
    ("Favored Terrain Bonus", "Ranger"),
    ("Terrain Mastery", "Ranger"),
    ("Terrain Dominance", "Ranger"),
    ("Infiltrator", "Ranger"),
    ("Hunter's Tricks", "Ranger"),
    ("Beastmaster", "Ranger"),
    ("Beastmaster Follower", "Ranger"),
    ("Packmaster", "Hunter"),
    ("Packmaster Follower", "Hunter"),
    ("Wildblooded", "Sorcerer"),
    ("Refined Education", "Rogue"),
    ("Blessings", "Warpriest"),
    ("Wild Talent", "Kineticist"),
    ("Implement School Focus Power", "Occultist"),
];

/// Resolves `group` against [`CATEGORY_LABEL_ALIASES`] by exact match,
/// then maps the verified target class name through `corpus_class_names`
/// (so the returned string is always the corpus's own natural-case
/// spelling, dispatched or not) -- `None` if the target class is not
/// itself corpus-declared (defensive; every entry above was verified
/// against a real corpus-declared class at construction time) or the
/// group has no alias entry at all.
fn category_label_alias_owner(group: &str, corpus_class_names: &BTreeMap<String, String>) -> Option<String> {
    let (_, target) = CATEGORY_LABEL_ALIASES.iter().find(|(label, _)| *label == group)?;
    corpus_class_names.get(&target.to_lowercase()).cloned()
}

// ---------------------------------------------------------------------
// SD-32 card 11, decision 23a: `"Domain Power"` is the one label
// `CATEGORY_LABEL_ALIASES` cannot close, because it is genuinely
// multi-owner -- the operator ruled option (a), extend the generator's
// INPUTS (`decisions.md §23a`), rather than declaring "shared across
// domain-access classes" an acceptable disposition (rejected: it closes
// the counter without learning which class grants what).
//
// The link the prior cycle's `TYPE:`/`PRE*:`-token search could not find
// lives one hop upstream. Every `"Domain Power ~ <X>"` ability is granted
// to a character by a class-namespaced chooser record shaped `"<Prefix>
// Domain ~ <domain>"` (`CATEGORY:Internal`) via an
// `ABILITY:...|AUTOMATIC|Domain Power ~ <X>|...` token on that chooser
// record -- the prefix names which class's domain-access mechanism the
// grant runs through:
//
//   - `Core Domain ~ <domain>` -- the base PCGen `DOMAIN` facet. Verified
//     directly against the class `.lst` files: `CLASS:Cleric` sets
//     `BONUS:DOMAIN|NUMBER|ClericDomainCount` and
//     `BONUS:VAR|ClericDomainCount|2` (`cr_classes.lst`); `CLASS:Paladin`
//     also carries `BONUS:DOMAIN|NUMBER|PaladinDomainCount`, but
//     `PaladinDomainCount` DEFINEs to 0 and is raised only by the Sacred
//     Servant archetype ability (`apg_abilities_class.lst`,
//     `KEY:Sacred Servant ~ Spells`, `BONUS:VAR|PaladinDomainCount|1
//     |TYPE=Base`) -- so `"Core Domain ~"` owns both classes, the second
//     conditioned on that archetype.
//   - `Druid Domain ~ <domain>` / `Inquisitor Domain ~ <domain>` -- each
//     class's own separate domain-access mechanism (Inquisitor's
//     Inquisition-adjacent grant is the one that shows up across the
//     seven SD-32 in-scope books; verified 98 of 172 in-scope records
//     also carry an `"Inquisitor Domain ~"` grant alongside `"Core Domain
//     ~"`, via [`domain_power_owning_classes`]'s own re-derive).
//
// Some (mostly newer, subdomain) books skip the class-namespaced wrapper
// entirely and grant a power straight from the bare domain/subdomain-
// named record (e.g. `bestiary_6/b6_domains.lst`'s `"Dragon Subdomain"`
// grants `"Domain Power ~ Venomous Stare"` directly, no `"Core Domain ~
// Dragon Subdomain"` intermediate). That bare record is reachable through
// the SAME base `DOMAIN` facet every `"Core Domain ~"` grant runs
// through, so it resolves the same owners as an explicit `"Core Domain
// ~"` grant -- not a guess, the identical mechanism minus one redundant
// hop. The one exclusion: a `.MOD` line whose own key names a foreign
// category (`CATEGORY=FEAT|...`) is a cross-reference into an unrelated
// feat record, not a domain grant point, and is skipped.
//
// This is a corpus-wide read, not a hand-authored table (`decisions.md
// §17`): [`scan_domain_power_owners`] walks every `.lst` file under
// `corpus_root` once per [`generate`] call and returns the full set of
// classes each `"Domain Power ~ <X>"` target resolves to, built fresh
// from the oracle every regen. `scripts/derive_domain_power_classes.py`
// carries the identical logic as an independently re-runnable oracle;
// this cycle's receipt records the cross-check between the two.
// ---------------------------------------------------------------------

/// The prefix->owning-classes table for an explicit `"<Prefix> Domain ~
/// <domain>"` chooser record (see the section comment above). This is
/// NOT a per-record table -- three entries total, one per class-access
/// mechanism the oracle itself distinguishes -- unlike
/// [`CATEGORY_LABEL_ALIASES`], which is keyed per LABEL because no
/// generic rule connects a label's text to its class at all.
const DOMAIN_POWER_PREFIX_CLASSES: &[(&str, &[&str])] =
    &[("Core", &["Cleric", "Paladin"]), ("Druid", &["Druid"]), ("Inquisitor", &["Inquisitor"])];

/// The same owners a `"Core Domain ~"` grant resolves to, applied when a
/// domain/subdomain record grants a `"Domain Power ~ <X>"` power directly
/// with no class-namespaced wrapper at all (see section comment above).
const DOMAIN_POWER_BARE_GRANT_CLASSES: &[&str] = &["Cleric", "Paladin"];

/// A record's effective key for this scan: PCGen lets the first
/// tab-delimited field be a display name distinct from the record's real
/// `KEY:` token (`"Chaos<TAB>...<TAB>KEY:Inquisitor Domain ~
/// Chaos<TAB>..."`) -- the explicit `KEY:` token wins when present, since
/// [`DOMAIN_POWER_PREFIX_CLASSES`]' prefixes are matched against the KEY
/// namespace, not the display name.
fn effective_lst_key(line: &str) -> &str {
    for field in line.split('\t') {
        if let Some(key) = field.strip_prefix("KEY:") {
            return key.trim();
        }
    }
    line.split('\t').next().unwrap_or(line).trim()
}

/// Walks every `.lst` file under `corpus_root` once, and for every line
/// that grants one or more `"Domain Power ~ <X>"` targets via an
/// `AUTOMATIC` `ABILITY:` token, resolves the owning class(es) per the
/// section comment above. Returns `{domain power key suffix (the text
/// after "Domain Power ~ ") -> owning classes}`. Cheap enough to run once
/// per [`generate`] call (~2,900 `.lst` files, single pass, no
/// allocation-heavy parsing) -- this is a generic corpus-wide scan, not
/// per-record work (`decisions.md §17`).
fn scan_domain_power_owners(corpus_root: &Path) -> BTreeMap<String, BTreeSet<String>> {
    let mut owners: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    if !corpus_root.is_dir() {
        return owners;
    }
    let mut stack = vec![corpus_root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            let is_lst = path.extension().and_then(|e| e.to_str()).map(|e| e.eq_ignore_ascii_case("lst"));
            if is_lst != Some(true) {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            for line in text.lines() {
                if line.is_empty() || line.starts_with('#') || !line.contains("Domain Power ~ ") {
                    continue;
                }
                let effective_key = effective_lst_key(line);
                let classes: &[&str] = if let Some((_, classes)) =
                    DOMAIN_POWER_PREFIX_CLASSES.iter().find(|(prefix, _)| {
                        effective_key.starts_with(*prefix) && effective_key[prefix.len()..].starts_with(" Domain ~ ")
                    }) {
                    classes
                } else if !effective_key.contains("CATEGORY=") {
                    DOMAIN_POWER_BARE_GRANT_CLASSES
                } else {
                    continue;
                };
                for segment in line.split("Domain Power ~ ").skip(1) {
                    let end = segment.find(['|', '\t']).unwrap_or(segment.len());
                    let target = segment[..end].trim();
                    if target.is_empty() {
                        continue;
                    }
                    let entry = owners.entry(target.to_string()).or_default();
                    entry.extend(classes.iter().map(|c| c.to_string()));
                }
            }
        }
    }
    owners
}

/// Resolves `key` (a full `"Domain Power ~ <X>"` record key) against
/// `owners` (from [`scan_domain_power_owners`]) -- `None` when `key` is
/// not in the `"Domain Power ~ "` namespace at all, or when the scan
/// found no upstream grant chain for it (an honest gap, never guessed).
fn domain_power_owning_classes(key: &str, owners: &BTreeMap<String, BTreeSet<String>>) -> Option<Vec<String>> {
    let suffix = key.strip_prefix("Domain Power ~ ")?;
    let set = owners.get(suffix)?;
    if set.is_empty() {
        return None;
    }
    Some(set.iter().cloned().collect())
}

/// Redacts the `DESC` token in `raw_tokens` in place whenever `description`
/// classified as PI-redacted (declared `DESCISPI:Yes` OR blacklist-detected
/// via [`pi_screening::classify_field`]) -- otherwise `data.raw_tokens`
/// re-exposes the full Product-Identity prose verbatim even while
/// `data.description` correctly carries `[redacted PI]`. Never touches any
/// other token. No-op when `license` is not [`License::PiRedacted`].
fn redact_desc_token_if_pi(tokens: &mut [RawToken], license: crate::rules_core::shape_b_v1::License) {
    if license != crate::rules_core::shape_b_v1::License::PiRedacted {
        return;
    }
    for t in tokens.iter_mut() {
        if t.key == "DESC" {
            t.value = crate::rules_core::shape_b_v1::REDACTED_PI_MARKER.to_string();
        }
    }
}

/// SD-32 card 11, T9-round-4-followup: a blacklisted term concatenated
/// PascalCase-style into a NON-`DESC` token's value (`AldoriDefensiveParryLVL`,
/// `CalistrianHunter ~ ...`) ships un-redacted whenever the record's own NAME
/// and DESCRIPTION are both clean -- `redact_desc_token_if_pi` above only
/// ever screens the `DESC` token, and [`scrub_name_pi_tokens`] below only
/// ever runs on the `name_is_pi` branch. This scrubs EVERY raw token value
/// against [`pi_screening::blacklist_term_hit_including_concatenated`]
/// (the Rust port of `scripts/pi_scrub.py`'s function of the same name),
/// unconditionally -- the generic ingest path's equivalent of
/// `scrub_name_pi_tokens`'s check 1+4, run on every record rather than only
/// name-PI ones. Skips a value that is already the redaction marker (a
/// `redact_desc_token_if_pi` DESC token, most commonly) so it is not
/// re-scanned against itself. Never mutates a token this cycle did not
/// redact; returns whether anything changed.
fn redact_concatenated_blacklist_tokens(tokens: &mut [RawToken]) -> bool {
    let mut any_redacted = false;
    for t in tokens.iter_mut() {
        if t.value.is_empty() || t.value == crate::rules_core::shape_b_v1::REDACTED_PI_MARKER {
            continue;
        }
        if pi_screening::blacklist_term_hit_including_concatenated(&t.value).is_some() {
            t.value = crate::rules_core::shape_b_v1::REDACTED_PI_MARKER.to_string();
            any_redacted = true;
        }
    }
    any_redacted
}

/// `decisions.md §24b`-2: "The PI original appears nowhere that ships."
/// A record whose NAME is PI can carry that same name again inside another
/// token's VALUE (a `KEY:` token restating the row's own key verbatim is
/// the concrete shape `scripts/ingest_ability.py::scrub_name_pi_tokens`
/// found live). Scrubs any token VALUE that either hits the shared
/// blacklist term list, or contains the record's OWN original `name`/`key`
/// (or a `~`-delimited segment of `key`) as a case-insensitive substring.
/// `name`/`key` are used ONLY to build the redaction needle set -- never
/// written into the returned tokens, and the caller never stores the
/// original `name`/`key` on a renamed record's `data.name`/`data.key`.
/// Never mutates the input; returns `(scrubbed_tokens, any_redacted)`.
// `decisions.md §24b`-2, this cycle's own live finding: the shared
// `pi_screening::PI_BLACKLIST_TERMS` (57 terms) is stale against
// `scripts/sd32_t9_pi_review_feat_equipment.py`'s actively-amended 60-term
// SD-32 T9 list -- `ingest_ability.py`'s own module doc comment already
// names this exact gap ("rather than forking a fourth copy of the stale
// 57-term substring scan"). Found live: `adventurers_guide/
// ag_abilities_class.lst:889`'s `ABILITY:...TYPE=MagaambyaSpellAccess` and
// `:1086`'s `KEY:...Aldori...` both survived the shared list unredacted.
// Widening the SHARED constant was tried and reverted this cycle: it makes
// `tests/pi_table_sweep.rs`'s corpus-wide gate newly fail against
// `feat_gap_tables.rs`'s own already-shipped, out-of-this-cycle's-scope
// "Aldori"/"Magaambya" prose (a pre-existing, unrelated leak this cycle
// does not own fixing). This LOCAL supplement is scoped to exactly the
// records THIS function screens -- the renamed `class_feature` units --
// without touching the shared list's broader blast radius.
const RENAME_SCRUB_SUPPLEMENTAL_TERMS: &[&str] = &["Aldori", "Magaambya", "Magaambyan"];

fn scrub_name_pi_tokens(tokens: &[RawToken], name: &str, key: &str) -> (Vec<RawToken>, bool) {
    let mut needles: Vec<String> = Vec::new();
    for s in [name, key] {
        let s = s.trim();
        if !s.is_empty() {
            needles.push(s.to_lowercase());
        }
    }
    for segment in key.split('~') {
        let segment = segment.trim();
        if !segment.is_empty() {
            needles.push(segment.to_lowercase());
        }
    }

    let mut any_redacted = false;
    let scrubbed = tokens
        .iter()
        .map(|t| {
            let value = &t.value;
            let blacklist_hit = !value.is_empty()
                && (PI_BLACKLIST_TERMS.iter().any(|term| value.contains(term))
                    || RENAME_SCRUB_SUPPLEMENTAL_TERMS.iter().any(|term| value.contains(term)));
            let lower_value = value.to_lowercase();
            // `word_bounded_contains`, not a bare `.contains()` -- found live,
            // this cycle (t9-onboarding-pi-final-leaks-and-generators): a bare
            // substring match against a `~`-delimited KEY segment
            // over-redacted a record's own CLEAN `BONUS`/`VAR` formula tokens
            // whenever the formula's variable-identifier name happened to
            // CONTAIN a generic, non-PI segment as a coincidental substring
            // (a live `inner_sea_magic:ism_abilities_class.lst` record's KEY
            // -- a `<demonym-of-index-23> Pilgrim Domain ~ Chaos`-shaped
            // string, see this cycle's receipt for the coordinate -- splits
            // to the segment "chaos", which then matched inside the CLEAN
            // token value `VAR|DomainChaosLVL|2` -- "domainchaoslvl" contains
            // "chaos" with no separator on either side). The universal rule
            // this violates: "a BONUS:/DEFINE: value is a game rule, not
            // Product Identity -- never redact one" (this bundle already
            // restored 63 formulas destroyed by exactly this shape once).
            // Word-boundary matching preserves the check's real intent
            // (catching a token that RESTATES the record's own original
            // identity, verbatim or as a whole delimited segment) while
            // refusing a coincidental mid-identifier substring match --
            // mirrors `pi_screening::normalized_term_hit`'s own reasoning
            // for the SAME shape of false positive against the blacklist
            // scan itself.
            let identity_hit =
                !value.is_empty() && needles.iter().any(|n| pi_screening::word_bounded_contains(&lower_value, n));
            if blacklist_hit || identity_hit {
                any_redacted = true;
                RawToken { key: t.key.clone(), value: crate::rules_core::shape_b_v1::REDACTED_PI_MARKER.to_string() }
            } else {
                t.clone()
            }
        })
        .collect();
    (scrubbed, any_redacted)
}

/// Every `(source file basename, source line)` a FOREIGN (not this
/// generator's own) `class_feature` record already cites under
/// `class_feature_dir`, found by walking the directory and checking each
/// JSON's `data.class_key` field -- present ONLY on the hand-authored
/// `pathfinder_unchained` dump this module's doc comment describes (64
/// records, `barbarian_unchained_class/` and its three siblings, landed by
/// an earlier mechanism-wiring cycle through a different code path); this
/// generator's own output always uses `data.class`, never `data.class_key`.
/// [`generate`] skips any incoming unit whose citation appears here, so
/// widening scope to `pathfinder_unchained` (this cycle) cannot duplicate
/// or shadow those 64 records at a different computed path -- the
/// module doc comment's "must not overwrite" constraint, now enforced by
/// citation rather than by leaving the whole book out of scope. A citation
/// this generator wrote itself is never in this set (no `class_key` field),
/// so idempotent re-runs still refresh every record they already own --
/// unchanged from every prior cycle's regen behaviour.
fn foreign_citations(class_feature_dir: &Path) -> BTreeSet<(String, u32)> {
    let mut found = BTreeSet::new();
    if !class_feature_dir.is_dir() {
        return found;
    }
    let mut stack = vec![class_feature_dir.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            let Ok(v) = serde_json::from_str::<Value>(&text) else { continue };
            let is_foreign = v.get("data").and_then(|d| d.get("class_key")).is_some();
            if !is_foreign {
                continue;
            }
            let Some(src_path) = v.get("source").and_then(|s| s.get("path")).and_then(Value::as_str) else {
                continue;
            };
            let Some(line) = v.get("source").and_then(|s| s.get("line")).and_then(Value::as_u64) else {
                continue;
            };
            let basename = Path::new(src_path).file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
            found.insert((basename, line as u32));
        }
    }
    found
}

/// Generates the `class_feature` cache for exactly the units passed in
/// (already scoped to [`BOOK_PRIMARY_FILES`] by
/// [`units_from_inventory_json`], or an equivalent caller-built list).
/// `corpus_root` is a PCGen `data/` checkout; `grants_root` is
/// `data/class_feature_grants` (wave 22's trustworthy per-record grant-fact
/// tree -- see [`true_class_by_key`]); `out_dir` is `data/corpus` (one call
/// covers every book the unit list names).
pub fn generate(
    corpus_root: &Path,
    grants_root: &Path,
    out_dir: &Path,
    ingested_at: &str,
    units: &[ClassFeatureSourceUnit],
    corpus_class_names: &BTreeMap<String, String>,
) -> Result<GenerationReport, GenerationError> {
    let mut report = GenerationReport::default();
    let dir_by_book: BTreeMap<&str, &str> =
        BOOK_PRIMARY_FILES.iter().map(|(book, dir, _)| (*book, *dir)).collect();

    let mut units_by_book: BTreeMap<&str, Vec<&ClassFeatureSourceUnit>> = BTreeMap::new();
    for unit in units {
        units_by_book.entry(unit.book.as_str()).or_default().push(unit);
    }

    // SD-32 card 11 decision 23a: only scan the oracle for the domain-power
    // grant chain (`scan_domain_power_owners`'s single ~2,900-file walk)
    // when this run actually includes `"Domain Power ~ "`-keyed units --
    // every other `generate()` call (including every other test in this
    // module) pays nothing for it.
    let domain_power_owners = if units.iter().any(|u| u.key.starts_with("Domain Power ~ ")) {
        scan_domain_power_owners(corpus_root)
    } else {
        BTreeMap::new()
    };

    for (book, book_units) in units_by_book {
        let Some(&rel_dir) = dir_by_book.get(book) else { continue };
        let book_dir = corpus_root.join(rel_dir);
        if !book_dir.is_dir() {
            return Err(GenerationError::CorpusUnreachable(book_dir));
        }
        let wiring_index = WiringClassIndex::build(book, &book_dir);
        let mut lines = wiring_index.lines();
        let mut sha_by_file: HashMap<String, String> = HashMap::new();
        let mut used: BTreeSet<String> = BTreeSet::new();
        let class_feature_dir = out_dir.join(book).join("class_feature");
        let true_class = true_class_by_key(grants_root, book);
        let foreign = foreign_citations(&class_feature_dir);

        for unit in book_units {
            if foreign.contains(&(unit.source_file.clone(), unit.source_line)) {
                report.foreign_citation_skipped += 1;
                continue;
            }
            let Some(_raw_row) = lines.line(book, &unit.source_file, unit.source_line as usize) else {
                report.unresolved_citations.push(format!("{book}:{}:{}", unit.source_file, unit.source_line));
                continue;
            };
            let Some(file_path) = resolve_book_file(&book_dir, &unit.source_file) else {
                report.unresolved_citations.push(format!("{book}:{}:{}", unit.source_file, unit.source_line));
                continue;
            };
            let sha256 = match sha_by_file.get(&unit.source_file) {
                Some(s) => s.clone(),
                None => {
                    let s = sha256_file(&file_path)?;
                    sha_by_file.insert(unit.source_file.clone(), s.clone());
                    s
                }
            };
            let declared = declared_pi_at(&file_path, unit.source_line).unwrap_or_default();
            let (name_license, _, _, _) = pi_screening::classify_field("name", &unit.name);
            // `decisions.md §24`: a name-PI row is no longer skipped whole
            // -- it ingests under a Codex-generated neutral name (below,
            // after `tokens`/description are built) rather than never being
            // written at all (module doc comment's pre-§24 "not written"
            // text is superseded here).
            let name_is_pi = declared.name || name_license == crate::rules_core::shape_b_v1::License::PiRedacted;
            // Row 21 fix (`decisions.md`): `raw_row` alone is only this
            // unit's OWN base corpus row -- a real `.MOD`-appended
            // `BONUS:VAR` line targeting the SAME base name lives on a
            // SEPARATE row and, read this way, was silently dropped (all 8
            // `bloodline_tracker.json` records corpus-wide carried 1-2
            // tokens and ZERO `BONUS:VAR` tokens before this fix). Building
            // `tokens` from `wiring_index`'s own closure -- the identical
            // resolution `wiring_class_for` already uses to CLASSIFY this
            // record -- unions every `.MOD` row targeting this unit's
            // `name`/`key` (plus a `.COPY=` base row, when applicable) into
            // `raw_tokens`, so the mechanics the wiring-class read already
            // "sees" are the same ones shipped. `closure_rows`'s first
            // entry is always this same base row, so a unit with no `.MOD`
            // rows targeting it (the common case) sees byte-identical
            // `tokens` to before this fix.
            let closure_rows = wiring_index.closure_rows(
                &mut lines,
                &unit.source_file,
                unit.source_line,
                &unit.name,
                &unit.key,
            );
            let mut tokens: Vec<RawToken> =
                closure_rows.iter().filter_map(|r| r.as_deref()).flat_map(row_tokens).collect();
            let description = desc_value(&tokens);
            let (mut license, mut pi_field, mut pi_marker, stored_desc) = pi_screening::classify_optional_field_declared(
                "description",
                description.as_deref(),
                declared.description,
            );
            // SD-32 card 11, T9-onboarding-class-feature-pi-and-rescreen:
            // `classify_optional_field_declared` screens `description`
            // through `classify_field`'s BARE-SUBSTRING scan against the
            // literal `PI_BLACKLIST_TERMS` list -- it never applies the
            // word-bounded, OCR-normalized fold `pi_screening::
            // blacklist_term_hit_including_concatenated`/`normalized_term_hit`
            // apply. `redact_concatenated_blacklist_tokens` below already
            // scrubs `raw_tokens`' own `DESC` entry with the STRONG scan, so
            // an OCR-glitched term (e.g. the pinned oracle's own "Gorurn"
            // for "Gorum", `inner_sea_combat:isc_abilities_class.lst:256` --
            // found live, corpus-wide re-derivation this cycle) could ship
            // `data.description` raw while `data.raw_tokens`' DESC entry
            // was correctly redacted: the SAME text, screened by two
            // differently-strong scans, disagreeing. Re-screens
            // `stored_desc` with the STRONG scan and forces the marker if
            // the weaker scan above missed it -- never weakens an existing
            // redaction, only strengthens a miss.
            let stored_desc = match &stored_desc {
                Some(v) if v != crate::rules_core::shape_b_v1::REDACTED_PI_MARKER => {
                    if pi_screening::blacklist_term_hit_including_concatenated(v).is_some() {
                        license = crate::rules_core::shape_b_v1::License::PiRedacted;
                        pi_marker = Some(crate::rules_core::shape_b_v1::PI_MARKER_REDACTED.to_string());
                        if !pi_field.as_deref().is_some_and(|f| f.split(',').any(|p| p == "description")) {
                            pi_field = Some(match pi_field.take() {
                                Some(existing) => format!("{existing},description"),
                                None => "description".to_string(),
                            });
                        }
                        Some(crate::rules_core::shape_b_v1::REDACTED_PI_MARKER.to_string())
                    } else {
                        stored_desc
                    }
                }
                _ => stored_desc,
            };
            // W19-INTEGRATE fix (adversarial review, OPEN-ISSUES.md row 63 follow-up):
            // `description`/`stored_desc` above is correctly PI-screened, but `tokens`
            // (below, shipped verbatim as `data.raw_tokens`) was NOT -- a declared
            // DESCISPI:Yes row had its full Product-Identity prose re-exposed through
            // raw_tokens even while `data.description` carried the redaction marker.
            // Mirror `enrich_equipment_raw_tokens.rs::screen_field_value`'s precedent.
            redact_desc_token_if_pi(&mut tokens, license);
            // SD-32 card 11, T9-round-4-followup: run the concatenated-term
            // scrub over EVERY raw token before the name-PI branch below, so
            // a clean-name/clean-description record that still carries a
            // blacklisted term concatenated into some OTHER token's value
            // (`DEFINE`/`BONUS`/`TYPE`/`KEY`, ...) is caught regardless of
            // which branch this unit takes.
            let concat_redacted = redact_concatenated_blacklist_tokens(&mut tokens);
            if concat_redacted && license != crate::rules_core::shape_b_v1::License::PiRedacted {
                license = crate::rules_core::shape_b_v1::License::PiRedacted;
                pi_marker = Some(crate::rules_core::shape_b_v1::PI_MARKER_REDACTED.to_string());
            }
            if concat_redacted {
                let already_named = pi_field.as_deref().is_some_and(|f| f.split(',').any(|p| p == "raw_tokens"));
                if !already_named {
                    pi_field = Some(match pi_field.take() {
                        Some(existing) => format!("{existing},raw_tokens"),
                        None => "raw_tokens".to_string(),
                    });
                }
            }
            let (wiring_class, wiring_class_signals) = wiring_index.wiring_class_for(
                &mut lines,
                &unit.source_file,
                unit.source_line,
                &unit.name,
                &unit.key,
            );
            let completeness = if stored_desc.is_some() { Completeness::Full } else { Completeness::ChassisOnly };
            // The key's own owner segment (`Sigilus ~ Inscribe Rune` ->
            // `Sigilus`) -- kept ONLY as the directory-naming fallback and
            // for the `class` fallback below, never shipped as the data
            // field's value when the grant data resolves. This is the SAME
            // split the field used to derive `class` from outright, which
            // this cycle found is wrong whenever the key's owner segment is
            // an archetype name rather than the real class.
            let key_owner = unit.key.split_once(" ~ ").map(|(owner, _)| owner.to_string());
            // `true_class` (wave 22's grant-fact ground truth) wins first.
            // SD-32 card 11 (T2a/T12 combined cycle) adds two more
            // resolution tiers BEFORE the raw key-prefix guess -- the pool
            // catalog (real, tested option-pool -> dispatched-class table)
            // and the `type_facet` "<Class> Class Feature" marker, each
            // tried first against the 34 DISPATCHED classes (closes true
            // T2a plumbing) and then against the full corpus-declared
            // roster (closes the T2a/T12 overlap: a genuinely-undispatched
            // class's own name, not a category label). The T2a-residual
            // cycle adds a sixth tier, [`category_label_alias_owner`], for
            // labels whose own text names no class at all (`"Ki Power"` ->
            // `"Monk"` via its `PRE:` token, not via any text match) -- see
            // the section comment above [`CATEGORY_LABEL_ALIASES`] for why
            // it is a verified per-label table, not a text fuzzer, and for
            // which labels were deliberately excluded. The raw key-prefix
            // split is the last-resort fallback for whatever none of the
            // six tiers above resolves. See the module section comment
            // above `POOL_TO_DISPATCHED_CLASS` for the full argument.
            let group = key_owner.as_deref().unwrap_or(&unit.key);
            let class = true_class
                .get(&unit.key)
                .cloned()
                .or_else(|| pool_catalog_owner(group).map(str::to_string))
                .or_else(|| type_facet_dispatched_owner(unit.type_facet.as_deref()))
                .or_else(|| corpus_class_owner(group, corpus_class_names))
                .or_else(|| category_label_alias_owner(group, corpus_class_names))
                .or_else(|| type_facet_corpus_owner(unit.type_facet.as_deref(), corpus_class_names))
                // `decisions.md §24b`-2: this LAST fallback ships the raw
                // key-owner TEXT verbatim -- safe for an ordinary key
                // (`"Fighter ~ Bravery"` -> `"Fighter"`), but for a
                // name-PI row (`declared.name`/blacklist true) the row's
                // OWN key can carry the SAME PI content as an owner
                // segment (a `"<Patron> ~ <Boon Name>"`-shaped
                // Demonic-Obedience boon, `book_of_the_damned_volume_2`,
                // whose "owner" segment is itself the patron's own PI
                // name -- found live this cycle: 7 of 140 renamed
                // `class_feature` units leaked their patron's name into
                // `data.class` and the output directory this way before
                // this guard, never named here per this repo's own PI
                // discipline). Never guess a class from PI-tainted text --
                // an honest `None` gap here, not a guess.
                .or_else(|| if name_is_pi { None } else { key_owner.clone() });

            // SD-32 card 11 decision 23a: `"Domain Power"` is genuinely
            // multi-owner and none of the six tiers above force it to one
            // class -- this is the seventh, separate resolution that
            // records the FULL owning-class set (never collapsed into
            // `class`) by reading the oracle's own domain-grant chain. See
            // the section comment above [`scan_domain_power_owners`].
            let classes = domain_power_owning_classes(&unit.key, &domain_power_owners);

            // SD-32 card 11, T9-onboarding-class-feature-pi-and-rescreen:
            // `key`/`class` were NEVER screened against the blacklist except
            // via the narrow guard at line ~1423, which only suppresses a
            // leak into `class` for a row ALREADY flagged `name_is_pi` by
            // its OWN `name`/declared-PI check above -- a `"<Feature Name>
            // ~ <PI-owner>"`-shaped key (e.g. `"Lunatic's Gift ~ Lamashtu"`,
            // `book_of_the_damned_volume_2`) has a perfectly clean `name`
            // ("Lunatic's Gift") while its OWNER SEGMENT carries the
            // Product Identity, so `name_is_pi` was false and `record_key`
            // below shipped `unit.key` verbatim -- confirmed live,
            // corpus-wide re-derivation (T9-onboarding cycle, `decisions.md
            // §17a`): 43 `class_feature` records, 71 field-level hits, the
            // large majority on `key`/`class`, not `name`/`description`.
            // This is the fourth confirmed instance of "screens one field,
            // not every shipped field" in this generator family (after
            // `raw_tokens` here, `prerequisites` in `feat_gap.rs`, and the
            // `class`-only guard this comment extends). Widening
            // `name_is_pi` to also cover `key`/the FINAL resolved `class`
            // (checked post-computation, so every one of the seven
            // resolution tiers above is covered uniformly, not just the
            // raw key-owner fallback) routes a key-PI or class-PI record
            // through the SAME `§24` neutral-rename path a name-PI record
            // already takes -- the record's own key/class IS its identity
            // exactly as much as `name` is, so the identical treatment
            // applies. Uses `blacklist_term_hit_including_concatenated`
            // (word-bounded, OCR-normalized, PLUS the concatenated-
            // identifier check), the same scan `scrub_name_pi_tokens`/the
            // corpus-wide audit use -- not the weaker bare-substring
            // `classify_field` the `name`-only check above uses, so this
            // catches a concatenated-identifier key shape too.
            let key_is_pi =
                pi_screening::blacklist_term_hit_including_concatenated(&unit.key).is_some();
            let class_is_pi = class
                .as_deref()
                .map(|c| pi_screening::blacklist_term_hit_including_concatenated(c).is_some())
                .unwrap_or(false);
            let name_is_pi = name_is_pi || key_is_pi || class_is_pi;

            // The `key_is_pi`/`class_is_pi` widening above only decides
            // whether the RECORD'S IDENTITY (`name`/`key`) gets routed
            // through the `§24` rename path -- it does NOT, by itself,
            // redact `data.class`'s own stored value. `class` is written
            // verbatim (`class.clone()`, below) regardless of `name_is_pi`,
            // so an ALREADY-name-PI-renamed record could still ship a
            // PI-bearing `class` (found live: a `codex_named_unit_*` file
            // whose `data.name`/`data.key` were correctly Codex-named but
            // whose `data.class` still read `"Aldori Swordlord"` verbatim --
            // the archetype name IS the very content `decisions.md §19a`
            // amendment 3d added the term to protect). `class` is a
            // secondary, derived field (not the record's own identity the
            // way `name`/`key` are) -- like `description`, it is redacted
            // to the marker in place, not renamed.
            //
            // The directory-placement logic below also reads `class` for a
            // renamed record -- found live in this cycle's own dry run:
            // redacting `class` here and reusing the SAME (now-redacted)
            // value for directory placement put every PI-class-bearing
            // renamed record under a literal `redacted_pi/` directory,
            // still shipping the archetype's real name in the FILE PATH
            // even though `data.class` itself was clean. The
            // directory-placement code below is written to fall through to
            // `record_name` whenever `class_is_pi`, so it never reads this
            // redacted value at all -- see that guard's own comment.
            let class = if class_is_pi {
                Some(crate::rules_core::shape_b_v1::REDACTED_PI_MARKER.to_string())
            } else {
                class
            };

            // `decisions.md §24` -- ingest a name-PI unit under a
            // Codex-generated neutral name derived ONLY from
            // (kind, book, source_file, source_line)
            // (`codex_neutral_name::neutral_name`/`neutral_key`; see that
            // module's own doc comment and test for the `§24b`-1 proof this
            // cannot be influenced by the original PI name).
            let (record_name, record_key, record_tokens, codex_generated_name, rename_info): (
                String,
                String,
                Vec<RawToken>,
                bool,
                Option<RenameInfo>,
            ) = if name_is_pi {
                let codex_name = neutral_name("class_feature", book, &unit.source_file, unit.source_line);
                let codex_key = neutral_key("class_feature", book, &unit.source_file, unit.source_line);
                let (scrubbed_tokens, extra_redacted) = scrub_name_pi_tokens(&tokens, &unit.name, &unit.key);
                report.name_pi_skipped += 1;
                report.name_pi_renamed_records.push(serde_json::json!({
                    "kind": "class_feature",
                    "book": book,
                    "source_file": Path::new(&unit.source_file).file_name().and_then(|n| n.to_str()).unwrap_or(&unit.source_file),
                    "source_line": unit.source_line,
                    "codex_name": codex_name,
                    "reason": "name_pi_blocked",
                }));
                // Append to (never overwrite) whatever the description
                // screen above already found -- a record can be BOTH
                // name-PI and desc-PI at once (found live: 91 of the 140
                // renamed units), and `declared_pi_shipping_audit`'s own
                // DESC-PI-SHIPPED check requires "description" to still be
                // list-present in `pi_field` even when "name" is also
                // present.
                let mut redacted_fields: Vec<&str> = Vec::new();
                // `.split(',')` rather than an exact `== Some("description")`
                // equality check: the concatenated-token scrub above can
                // already have set `pi_field` to `"description,raw_tokens"`
                // by this point (a record can be desc-PI AND carry a
                // concatenated blacklist hit in some OTHER token at once),
                // and an exact-equality check would silently drop
                // "description" off this branch's rebuilt list.
                if pi_field.as_deref().is_some_and(|f| f.split(',').any(|p| p == "description")) {
                    redacted_fields.push("description");
                }
                redacted_fields.push("name");
                if extra_redacted || concat_redacted {
                    redacted_fields.push("raw_tokens");
                }
                if class_is_pi {
                    redacted_fields.push("class");
                }
                license = crate::rules_core::shape_b_v1::License::PiRedacted;
                pi_field = Some(redacted_fields.join(","));
                pi_marker = Some(crate::rules_core::shape_b_v1::PI_MARKER_REDACTED.to_string());
                let rename_info = Some(RenameInfo {
                    reason: "name_pi_blocked".to_string(),
                    coordinate: format!(
                        "{book}:{}:{}",
                        Path::new(&unit.source_file).file_name().and_then(|n| n.to_str()).unwrap_or(&unit.source_file),
                        unit.source_line
                    ),
                });
                (codex_name.clone(), codex_key, scrubbed_tokens, true, rename_info)
            } else {
                (unit.name.clone(), unit.key.clone(), tokens, false, None)
            };

            let record = CacheRecord {
                population: Population::InScope,
                completeness,
                ingested_at: ingested_at.to_string(),
                data: ClassFeatureData {
                    key: record_key.clone(),
                    name: record_name.clone(),
                    class: class.clone(),
                    classes,
                    description: stored_desc,
                    raw_tokens: record_tokens,
                },
                codex_generated_name,
                rename: rename_info,
                source: Source::LstToken {
                    // Real relative path to the file this record was read
                    // from -- `{rel_dir}/{file}` for a primary file,
                    // `{rel_dir}/support/{file}` (etc.) for a nested
                    // variant [`resolve_book_file`] found. Falls back to
                    // the flat join only if `strip_prefix` somehow fails
                    // (never observed against `corpus_root`, since
                    // `file_path` was resolved from `book_dir` which is
                    // itself `corpus_root.join(rel_dir)`).
                    path: file_path
                        .strip_prefix(corpus_root)
                        .map(|p| p.to_string_lossy().replace('\\', "/"))
                        .unwrap_or_else(|_| format!("{rel_dir}/{}", unit.source_file)),
                    sha256,
                    line: unit.source_line,
                    record_key: record_key.clone(),
                },
                wiring_class,
                wiring_class_signals,
                license,
                pi_field,
                pi_marker,
            };

            // Directory placement stays keyed on the key's OWN owner segment
            // (never the corrected `class`) so this cycle's fix changes
            // exactly one field's VALUE and nothing about a record's path --
            // per the guarded-regen discipline, the only expected diff
            // against the pre-image is `data.class` (plus `ingested_at`).
            // `decisions.md §24b`-2: a renamed unit's directory/file naming
            // must never fall back to the raw original `key_owner` OR
            // `name` either -- `key_owner` (the key's own text BEFORE
            // " ~ ") can itself BE the PI content (a `"<Patron> ~ <Boon
            // Name>"`-shaped Demonic-Obedience key -- found live this
            // cycle leaking the patron's own name into both the directory
            // and `data.class` before this guard, not named here). For a
            // renamed record,
            // the directory source is the already-PI-screened `class`
            // (`None` when nothing resolved -- honest gap, never a guess)
            // falling back to the already-neutral `record_name`; an
            // ordinary record keeps its unchanged `key_owner`-first
            // behaviour so this fix produces zero diff on anything not
            // itself renamed.
            // SD-32 card 11, T9-onboarding-class-feature-pi-and-rescreen:
            // when `class` ITSELF is the PI content (`class_is_pi`), it is
            // now redacted to the marker in `data.class` (above) -- using
            // that (or the ORIGINAL, unredacted `raw_class`) here would
            // ship the archetype's real name in the FILE PATH instead of
            // the JSON body, the same leak moved one level over. Falls
            // through to the already-neutral `record_name`, exactly the
            // existing `class: None` honest-gap path already does.
            let dir_name_source: &str = if codex_generated_name {
                if class_is_pi { &record_name } else { class.as_deref().unwrap_or(&record_name) }
            } else {
                key_owner.as_deref().unwrap_or(&unit.name)
            };
            let name_fallback_for_slugs: &str = if codex_generated_name { &record_name } else { &unit.name };
            let class_dir_slug = slugify(dir_name_source, &mut BTreeSet::new());
            let feature_slug = {
                let key_for_used = format!("{class_dir_slug}/");
                let mut scoped: BTreeSet<String> = used
                    .iter()
                    .filter_map(|u| u.strip_prefix(&key_for_used).map(str::to_string))
                    .collect();
                let slug = slugify(name_fallback_for_slugs, &mut scoped);
                used.insert(format!("{key_for_used}{slug}"));
                slug
            };
            let out_dir_for_record = class_feature_dir.join(&class_dir_slug);
            std::fs::create_dir_all(&out_dir_for_record)?;
            let path = out_dir_for_record.join(format!("{feature_slug}.json"));
            let json = serde_json::to_string_pretty(&record)
                .expect("CacheRecord is a plain-data shape; serialization cannot fail");
            std::fs::write(path, json)?;
            report.written += 1;
            report.books_written.insert(book.to_string());
        }
    }

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn book_primary_files_covers_the_23_in_scope_books() {
        assert_eq!(BOOK_PRIMARY_FILES.len(), 23);
        // `ultimate_psionics` is back in scope this cycle -- the
        // `book_dir_of` 5-segment-path finding that excluded it went stale
        // once `book_dir_of` gained a 4-segment `dreamscarred_press`
        // branch (`014f210b9`). See module doc comment's `ultimate_psionics`
        // section.
        assert!(BOOK_PRIMARY_FILES.iter().any(|(book, _, _)| *book == "ultimate_psionics"));
        // `pathfinder_unchained` is back in scope this cycle too --
        // [`foreign_citations`] protects its 64 hand-curated records per
        // unit now, so the whole book no longer needs excluding. See
        // module doc comment's `pathfinder_unchained` section.
        assert!(BOOK_PRIMARY_FILES.iter().any(|(book, _, _)| *book == "pathfinder_unchained"));
    }

    #[test]
    fn foreign_citations_finds_only_records_carrying_class_key_never_this_generators_own() {
        let dir = std::env::temp_dir().join(format!(
            "codex_class_feature_foreign_citations_test_{}",
            std::process::id()
        ));
        let sub = dir.join("some_class");
        std::fs::create_dir_all(&sub).unwrap();
        std::fs::write(
            sub.join("foreign.json"),
            r#"{"data":{"class_key":"Summoner ~ Unchained Class","raw_tokens":[]},
                "source":{"path":"pathfinder/paizo/roleplaying_game/pathfinder_unchained/pu_abilities_class.lst","line":736}}"#,
        )
        .unwrap();
        std::fs::write(
            sub.join("own.json"),
            r#"{"data":{"class":"Rogue","raw_tokens":[]},
                "source":{"path":"pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_class.lst","line":1615}}"#,
        )
        .unwrap();

        let found = foreign_citations(&dir);
        assert_eq!(found.len(), 1);
        assert!(found.contains(&("pu_abilities_class.lst".to_string(), 736)));
        assert!(!found.contains(&("cr_abilities_class.lst".to_string(), 1615)));

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn units_from_inventory_json_accepts_any_abilities_class_file_of_a_known_book() {
        let json = r#"{"units":[
            {"kind":"class_feature","book":"core_rulebook","source_file":"cr_abilities_class.lst","source_line":1615,"corpus_key":"Rogue ~ Sneak Attack","name":"Sneak Attack"},
            {"kind":"class_feature","book":"ultimate_combat","source_file":"uc_abilities_class_um.lst","source_line":12,"corpus_key":"A ~ B","name":"B"},
            {"kind":"class_feature","book":"core_rulebook","source_file":"some_other_file.lst","source_line":4,"corpus_key":"X ~ Y","name":"Y"},
            {"kind":"feat","book":"core_rulebook","source_file":"cr_abilities_class.lst","source_line":5,"corpus_key":"Z","name":"Z"},
            {"kind":"class_feature","book":"not_a_book","source_file":"nope.lst","source_line":5,"corpus_key":"Z","name":"Z"}
        ]}"#;
        let units = units_from_inventory_json(json).unwrap();
        assert_eq!(units.len(), 2);
        assert_eq!(units[0].key, "Rogue ~ Sneak Attack");
        assert_eq!(units[0].source_line, 1615);
        // The nested-support-file row (`uc_abilities_class_um.lst`) is
        // accepted too -- widened scope, this cycle.
        assert!(units.iter().any(|u| u.source_file == "uc_abilities_class_um.lst"));
    }

    #[test]
    fn units_from_inventory_json_accepts_the_two_known_abilities_race_files_but_no_other_book() {
        // RED before the `EXTRA_CLASS_FEATURE_SOURCE_FILES` allowlist: all
        // three `abilities_race.lst` rows below were dropped by the bare
        // `abilities_class` substring check, reproducing the real
        // `no_record` shape found this cycle (`decisions.md §20`/`§17a`).
        let json = r#"{"units":[
            {"kind":"class_feature","book":"advanced_class_guide","source_file":"acg_abilities_race.lst","source_line":294,"corpus_key":"Skald Spell Level 0","name":"Skald Spell Level 0"},
            {"kind":"class_feature","book":"advanced_players_guide","source_file":"apg_abilities_race.lst","source_line":284,"corpus_key":"Oracle Spell Level 0","name":"Oracle Spell Level 0"},
            {"kind":"class_feature","book":"core_rulebook","source_file":"cr_abilities_race.lst","source_line":1,"corpus_key":"Not A Real Pair","name":"Not A Real Pair"}
        ]}"#;
        let units = units_from_inventory_json(json).unwrap();
        // Only the two allowlisted (book, file) pairs are admitted -- a
        // THIRD book's own `*_abilities_race.lst` (genuinely race content,
        // never verified for this book) stays excluded, proving this is a
        // precise pair list, not a broadened substring match.
        assert_eq!(units.len(), 2);
        assert!(units.iter().any(|u| u.book == "advanced_class_guide" && u.source_file == "acg_abilities_race.lst"));
        assert!(units.iter().any(|u| u.book == "advanced_players_guide" && u.source_file == "apg_abilities_race.lst"));
        assert!(!units.iter().any(|u| u.book == "core_rulebook"));
    }

    #[test]
    fn resolve_book_file_finds_a_nested_support_file_by_basename() {
        let dir = std::env::temp_dir().join(format!(
            "codex_class_feature_resolve_test_{}",
            std::process::id()
        ));
        let support = dir.join("support");
        std::fs::create_dir_all(&support).unwrap();
        std::fs::write(dir.join("primary_abilities_class.lst"), "primary").unwrap();
        std::fs::write(support.join("nested_abilities_class.lst"), "nested").unwrap();

        let primary = resolve_book_file(&dir, "primary_abilities_class.lst");
        assert_eq!(primary, Some(dir.join("primary_abilities_class.lst")));

        let nested = resolve_book_file(&dir, "nested_abilities_class.lst");
        assert_eq!(nested, Some(support.join("nested_abilities_class.lst")));

        let missing = resolve_book_file(&dir, "does_not_exist.lst");
        assert_eq!(missing, None);

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn row_tokens_skips_the_identity_column_and_splits_on_first_colon() {
        let row = "Sneak Attack\t\tKEY:Rogue ~ Sneak Attack\t\tCATEGORY:Special Ability\tDEFINE:RogueSneakAttackLVL|0";
        let tokens = row_tokens(row);
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].key, "KEY");
        assert_eq!(tokens[0].value, "Rogue ~ Sneak Attack");
        assert_eq!(tokens[2].key, "DEFINE");
        assert_eq!(tokens[2].value, "RogueSneakAttackLVL|0");
    }

    #[test]
    fn desc_value_finds_the_desc_token() {
        let tokens = vec![
            RawToken { key: "CATEGORY".to_string(), value: "Special Ability".to_string() },
            RawToken { key: "DESC".to_string(), value: "You gain a bonus.".to_string() },
        ];
        assert_eq!(desc_value(&tokens).as_deref(), Some("You gain a bonus."));
        assert_eq!(desc_value(&[]), None);
    }

    #[test]
    fn slugify_handles_collisions() {
        let mut used = BTreeSet::new();
        let a = slugify("Sneak Attack", &mut used);
        let b = slugify("Sneak Attack", &mut used);
        assert_eq!(a, "sneak_attack");
        assert_ne!(a, b);
    }

    /// `OPEN-ISSUES.md` row 48: a class-feature name carrying a
    /// blacklisted Product-Identity term must be flagged even with no
    /// `NAMEISPI:YES` declaration on its own row -- the same union basis
    /// `equipment_gap.rs` already established. Both of the two shipped
    /// records that carried NO PI marking at all reproduce this exact
    /// shape: their row does not declare `NAMEISPI:YES`, only the
    /// blacklist term scan catches them.
    #[test]
    fn a_blacklisted_name_is_flagged_even_with_no_nameispi_declaration() {
        let (license, _, _, _) = pi_screening::classify_field("name", "Gorum");
        assert_eq!(license, crate::rules_core::shape_b_v1::License::PiRedacted);
        let (license2, _, _, _) = pi_screening::classify_field("name", "Death (Pharasma)");
        assert_eq!(license2, crate::rules_core::shape_b_v1::License::PiRedacted);
    }

    /// The production call site's actual gating logic, isolated from file
    /// I/O: the union of `declared.name` (row-declared) and the blacklist
    /// term scan (undeclared-but-listed) must both trigger a skip, and a
    /// clean name with neither signal must not.
    #[test]
    fn name_skip_is_the_union_of_declared_and_blacklisted() {
        fn should_skip(declared_name: bool, name: &str) -> bool {
            let (name_license, _, _, _) = pi_screening::classify_field("name", name);
            declared_name || name_license == crate::rules_core::shape_b_v1::License::PiRedacted
        }
        assert!(should_skip(true, "Ordinary Feature"), "row-declared NAMEISPI:YES must skip");
        assert!(should_skip(false, "Gorum"), "blacklisted name with no declaration must still skip");
        assert!(!should_skip(false, "Sneak Attack"), "an ordinary name must not skip");
    }

    /// `decisions.md §24b`-2 unit test: a token VALUE restating the
    /// record's own original name/key is scrubbed, a blacklisted term in an
    /// unrelated token is also scrubbed, and an ordinary token untouched.
    #[test]
    fn scrub_name_pi_tokens_redacts_identity_restatement_and_blacklist_hits_only() {
        // Fictional stand-ins, deliberately not a real Product-Identity
        // name -- this repo's own discipline forbids putting a real PI
        // term in code/tests/comments (`decisions.md §24`).
        let tokens = vec![
            RawToken { key: "KEY".to_string(), value: "Exalted Boon ~ Fictional Patron ~ Ember Lance".to_string() },
            RawToken { key: "CATEGORY".to_string(), value: "Special Ability".to_string() },
            RawToken { key: "SOURCEPAGE".to_string(), value: "p.12".to_string() },
        ];
        let (scrubbed, any_redacted) =
            scrub_name_pi_tokens(&tokens, "Ember Lance", "Exalted Boon ~ Fictional Patron ~ Ember Lance");
        assert!(any_redacted);
        let by_key: BTreeMap<&str, &str> = scrubbed.iter().map(|t| (t.key.as_str(), t.value.as_str())).collect();
        assert_eq!(by_key["KEY"], crate::rules_core::shape_b_v1::REDACTED_PI_MARKER, "own key restatement must scrub");
        assert_eq!(by_key["CATEGORY"], "Special Ability", "an ordinary token must survive untouched");
        assert_eq!(by_key["SOURCEPAGE"], "p.12");
    }

    /// t9-onboarding-pi-final-leaks-and-generators cycle: reproduces the
    /// live `inner_sea_magic:ism_abilities_class.lst` shape (coordinate in
    /// this cycle's receipt) -- a `~`-delimited key segment that is an
    /// ordinary, non-PI word
    /// ("Chaos") must NOT redact a CLEAN `BONUS`/`VAR` formula token whose
    /// variable-identifier name merely happens to contain that word as a
    /// coincidental substring with no separator on either side. Mutation
    /// proof: reproducing the OLD bare-`.contains()` check directly (not by
    /// reverting the fix) shows it WOULD have redacted this token, so this
    /// test would have failed red before the word-boundary fix.
    #[test]
    fn scrub_name_pi_tokens_does_not_over_redact_a_clean_formula_sharing_a_generic_key_segment() {
        let tokens = vec![
            RawToken { key: "KEY".to_string(), value: "Domain Feature ~ Chaos".to_string() },
            RawToken { key: "BONUS".to_string(), value: "VAR|DomainChaosLVL|2".to_string() },
            RawToken { key: "CATEGORY".to_string(), value: "Special Ability".to_string() },
        ];
        let (scrubbed, any_redacted) = scrub_name_pi_tokens(&tokens, "Chaos", "Domain Feature ~ Chaos");
        let by_key: BTreeMap<&str, &str> = scrubbed.iter().map(|t| (t.key.as_str(), t.value.as_str())).collect();
        assert_eq!(by_key["KEY"], crate::rules_core::shape_b_v1::REDACTED_PI_MARKER, "own key restatement must still scrub");
        assert_eq!(
            by_key["BONUS"], "VAR|DomainChaosLVL|2",
            "a clean formula must survive even though its identifier coincidentally contains the generic segment \"chaos\" with no separator"
        );
        assert_eq!(by_key["CATEGORY"], "Special Ability");
        assert!(any_redacted, "the KEY restatement alone must still trigger any_redacted");

        // Mutation proof: the OLD unbounded check, reproduced inline (never
        // by reverting the real fix), DOES flag the BONUS token -- proving
        // this test exercises a real behavioural difference, not a vacuous
        // assertion.
        let old_unbounded_would_flag = "var|domainchaoslvl|2".contains("chaos");
        assert!(old_unbounded_would_flag, "sanity: the pre-fix bare-substring check must reproduce the over-redaction");
    }

    #[test]
    fn scrub_name_pi_tokens_is_a_no_op_when_nothing_restates_the_identity() {
        let tokens =
            vec![RawToken { key: "CATEGORY".to_string(), value: "Special Ability".to_string() }];
        let (scrubbed, any_redacted) = scrub_name_pi_tokens(&tokens, "Bravery", "Fighter ~ Bravery");
        assert!(!any_redacted);
        assert_eq!(scrubbed[0].value, "Special Ability");
    }

    /// End-to-end RED->GREEN precedent for `decisions.md §24`: a
    /// `NAMEISPI:YES` row must now be WRITTEN (not skipped) under a
    /// Codex-generated neutral name, visibly marked, with the coordinate
    /// (never the original name) recorded on `data.rename`, and the
    /// original name/key must appear nowhere in the written file.
    #[test]
    fn generate_renames_a_name_pi_row_instead_of_skipping_it() {
        let tmp = std::env::temp_dir().join(format!("cf-generate-name-pi-rename-{}", std::process::id()));
        let corpus_root = tmp.join("corpus_root");
        let grants_root = tmp.join("grants_root");
        let out_dir = tmp.join("out");
        std::fs::remove_dir_all(&tmp).ok();

        let book_dir = corpus_root.join("pathfinder/paizo/roleplaying_game/adventurers_guide");
        std::fs::create_dir_all(&book_dir).unwrap();
        // A single-segment KEY (no " ~ " owner split) whose own name IS the
        // declared-PI content -- the exact edge this test guards against a
        // directory/slug fallback leaking the original name.
        std::fs::write(
            book_dir.join("ag_abilities_class.lst"),
            "Ordinary Feature\t\tNAMEISPI:YES\tCATEGORY:Special Ability\tDESC:Some mechanical text.\n",
        )
        .unwrap();

        let units = vec![ClassFeatureSourceUnit {
            book: "adventurers_guide".to_string(),
            source_file: "ag_abilities_class.lst".to_string(),
            source_line: 1,
            key: "Ordinary Feature".to_string(),
            name: "Ordinary Feature".to_string(),
            type_facet: None,
        }];

        let report = generate(
            &corpus_root,
            &grants_root,
            &out_dir,
            "2026-08-23T00:00:00Z",
            &units,
            &BTreeMap::new(),
        )
        .expect("generate must succeed against a well-formed fixture");

        assert_eq!(report.written, 1, "a name-PI row must be WRITTEN, not skipped (§24)");
        assert_eq!(report.name_pi_skipped, 1, "the counter still tracks the rename (kept name for compatibility)");
        assert_eq!(report.name_pi_renamed_records.len(), 1);

        // The record must land SOMEWHERE under out_dir/adventurers_guide/class_feature
        // (directory name is coordinate-derived, not the original "Ordinary Feature").
        let cf_dir = out_dir.join("adventurers_guide/class_feature");
        let mut found: Option<String> = None;
        for entry in walkdir(&cf_dir) {
            if entry.extension().and_then(|e| e.to_str()) == Some("json") {
                found = Some(std::fs::read_to_string(&entry).unwrap());
            }
        }
        let written = found.expect("generate must write exactly one json file for the renamed unit");
        let json: Value = serde_json::from_str(&written).unwrap();

        assert_eq!(json["codex_generated_name"].as_bool(), Some(true));
        assert!(
            json["data"]["name"].as_str().unwrap().starts_with("Codex-Named Unit ("),
            "data.name must carry the Codex-generated marker: {written}"
        );
        assert!(
            json["data"]["key"].as_str().unwrap().starts_with("Codex-Named Unit ("),
            "data.key must carry the Codex-generated marker: {written}"
        );
        assert_eq!(
            json["rename"]["coordinate"].as_str(),
            Some("adventurers_guide:ag_abilities_class.lst:1"),
            "rename must record ONLY the coordinate + reason: {written}"
        );
        assert_eq!(json["rename"]["reason"].as_str(), Some("name_pi_blocked"));
        assert!(
            !written.contains("Ordinary Feature"),
            "the original PI name must appear NOWHERE in the written file (§24b-2): {written}"
        );

        std::fs::remove_dir_all(&tmp).ok();
    }

    /// SD-32 card 11, T9-onboarding-class-feature-pi-and-rescreen: a
    /// `"<Feature Name> ~ <PI-owner>"`-shaped key can carry Product Identity
    /// in its OWNER segment while `name` itself is perfectly clean (e.g.
    /// `"Lunatic's Gift ~ Lamashtu"`, `book_of_the_damned_volume_2` --
    /// found live, unredacted, on disk: `name_is_pi` only ever inspected
    /// `unit.name`/the row's `NAMEISPI` declaration, never `unit.key`'s own
    /// text or the resolved `class` value, so `record_key` shipped the raw
    /// key -- deity name included -- verbatim). Proves the widened
    /// `name_is_pi` (key/class blacklist check) routes this shape through
    /// the SAME `§24` neutral-rename path a name-PI row already takes.
    #[test]
    fn generate_renames_a_row_whose_key_owner_segment_carries_pi_even_when_name_is_clean() {
        let tmp = std::env::temp_dir().join(format!("cf-generate-key-owner-pi-{}", std::process::id()));
        let corpus_root = tmp.join("corpus_root");
        let grants_root = tmp.join("grants_root");
        let out_dir = tmp.join("out");
        std::fs::remove_dir_all(&tmp).ok();

        let book_dir = corpus_root.join("pathfinder/paizo/campaign_setting/book_of_the_damned_volume_2");
        std::fs::create_dir_all(&book_dir).unwrap();
        // No NAMEISPI/DESCISPI declaration at all -- the leak is purely
        // that the KEY's owner segment is a blacklisted deity name; `name`
        // is ordinary, undeclared prose.
        std::fs::write(
            book_dir.join("botd2_abilities_classes.lst"),
            "Lunatic's Gift\t\tCATEGORY:Special Ability|Lamashtu\tDESC:Some mechanical text.\n",
        )
        .unwrap();

        let units = vec![ClassFeatureSourceUnit {
            book: "book_of_the_damned_volume_2".to_string(),
            source_file: "botd2_abilities_classes.lst".to_string(),
            source_line: 1,
            key: "Lunatic's Gift ~ Lamashtu".to_string(),
            name: "Lunatic's Gift".to_string(),
            type_facet: None,
        }];

        let report = generate(
            &corpus_root,
            &grants_root,
            &out_dir,
            "2026-08-23T00:00:00Z",
            &units,
            &BTreeMap::new(),
        )
        .expect("generate must succeed against a well-formed fixture");

        assert_eq!(report.written, 1);
        assert_eq!(
            report.name_pi_skipped, 1,
            "a key-owner-segment PI hit must route through the SAME rename counter as a name-PI row"
        );

        let cf_dir = out_dir.join("book_of_the_damned_volume_2/class_feature");
        let mut found: Option<String> = None;
        for entry in walkdir(&cf_dir) {
            if entry.extension().and_then(|e| e.to_str()) == Some("json") {
                found = Some(std::fs::read_to_string(&entry).unwrap());
            }
        }
        let written = found.expect("generate must write exactly one json file for the renamed unit");
        let json: Value = serde_json::from_str(&written).unwrap();

        assert_eq!(json["codex_generated_name"].as_bool(), Some(true));
        assert!(
            json["data"]["key"].as_str().unwrap().starts_with("Codex-Named Unit ("),
            "data.key must carry the Codex-generated marker, not the raw owner-segmented key: {written}"
        );
        assert!(
            !written.contains("Lamashtu"),
            "the deity name must appear NOWHERE in the written file (§24b-2), including data.class: {written}"
        );

        std::fs::remove_dir_all(&tmp).ok();
    }

    /// SD-32 card 11, T9-onboarding-class-feature-pi-and-rescreen: the
    /// widened `name_is_pi` (previous test) only decides whether the
    /// RECORD gets renamed -- it does not, by itself, redact `data.class`'s
    /// own stored value, which is written verbatim regardless. Found live:
    /// an already-renamed `codex_named_unit_*` record whose `data.class`
    /// still read `"Aldori Swordlord"` unredacted, because `class` came
    /// from the `corpus_class_owner` resolution tier (a REAL corpus-
    /// declared class name lookup) which runs BEFORE, and is not gated by,
    /// the `name_is_pi` guard the key-owner FALLBACK tier already had.
    /// Proves `class` itself gets redacted to the marker whenever its
    /// resolved value hits the blacklist, regardless of which of the seven
    /// resolution tiers produced it.
    #[test]
    fn generate_redacts_a_class_field_resolved_from_a_real_corpus_class_name_that_is_itself_pi() {
        let tmp = std::env::temp_dir().join(format!("cf-generate-class-field-pi-{}", std::process::id()));
        let corpus_root = tmp.join("corpus_root");
        let grants_root = tmp.join("grants_root");
        let out_dir = tmp.join("out");
        std::fs::remove_dir_all(&tmp).ok();

        let book_dir = corpus_root.join("pathfinder/paizo/roleplaying_game/adventurers_guide");
        std::fs::create_dir_all(&book_dir).unwrap();
        // `name` is ordinary, undeclared prose -- only the KEY's owner
        // segment (the FIRST `~`-delimited segment, matching the real
        // corpus shape: `"Aldori Swordlord ~ Quick Draw ~ Aldori Dueling
        // Mastery"`, `adventurers_guide:ag_abilities_class.lst:17`) and the
        // corpus-declared class name it resolves to carry the archetype's
        // (blacklisted) name.
        std::fs::write(
            book_dir.join("ag_abilities_class.lst"),
            "Combat Feat\t\tCATEGORY:Special Ability|Aldori Swordlord\tDESC:Some mechanical text.\n",
        )
        .unwrap();

        let units = vec![ClassFeatureSourceUnit {
            book: "adventurers_guide".to_string(),
            source_file: "ag_abilities_class.lst".to_string(),
            source_line: 1,
            key: "Aldori Swordlord ~ Combat Feat".to_string(),
            name: "Combat Feat".to_string(),
            type_facet: None,
        }];
        let mut corpus_class_names = BTreeMap::new();
        corpus_class_names.insert("aldori swordlord".to_string(), "Aldori Swordlord".to_string());

        generate(&corpus_root, &grants_root, &out_dir, "2026-08-23T00:00:00Z", &units, &corpus_class_names)
            .expect("generate must succeed against a well-formed fixture");

        let cf_dir = out_dir.join("adventurers_guide/class_feature");
        let mut found: Option<String> = None;
        for entry in walkdir(&cf_dir) {
            if entry.extension().and_then(|e| e.to_str()) == Some("json") {
                found = Some(std::fs::read_to_string(&entry).unwrap());
            }
        }
        let written = found.expect("generate must write exactly one json file for the record");
        let json: Value = serde_json::from_str(&written).unwrap();

        assert_eq!(
            json["data"]["class"].as_str(),
            Some(crate::rules_core::shape_b_v1::REDACTED_PI_MARKER),
            "data.class must be redacted to the marker, not the real archetype name: {written}"
        );
        assert!(
            !written.contains("Aldori"),
            "the archetype's (blacklisted) name must appear NOWHERE in the written file: {written}"
        );
        // The directory the file was found under (via `walkdir(&cf_dir)`
        // above) must ALSO never carry the archetype's name -- moving the
        // leak from `data.class` into the FILE PATH is the same defect.
        assert!(
            !cf_dir.join("aldori_swordlord").is_dir(),
            "the record must not be written under a directory named after the PI archetype"
        );

        std::fs::remove_dir_all(&tmp).ok();
    }

    /// SD-32 card 11, T9-onboarding-class-feature-pi-and-rescreen: found
    /// live -- `data.description`'s own screen (`classify_field`'s bare
    /// substring scan) missed an OCR-glitched blacklist term
    /// ("Gorurn" for "Gorum") that `raw_tokens`' concatenated-token screen
    /// (the STRONG, OCR-normalized scan) caught, so the SAME text shipped
    /// redacted in `raw_tokens` but raw in `data.description`. Proves the
    /// supplementary strong-scan check closes the gap.
    #[test]
    fn generate_redacts_a_description_carrying_an_ocr_glitched_blacklist_term_the_weak_scan_misses() {
        let tmp = std::env::temp_dir().join(format!("cf-generate-desc-ocr-{}", std::process::id()));
        let corpus_root = tmp.join("corpus_root");
        let grants_root = tmp.join("grants_root");
        let out_dir = tmp.join("out");
        std::fs::remove_dir_all(&tmp).ok();

        let book_dir = corpus_root.join("pathfinder/paizo/campaign_setting/inner_sea_combat");
        std::fs::create_dir_all(&book_dir).unwrap();
        std::fs::write(
            book_dir.join("isc_abilities_class.lst"),
            "Ranger Combat Style\t\tCATEGORY:Special Ability\tDESC:If a ranger selects Gorurn's style, he gains a bonus.\n",
        )
        .unwrap();

        let units = vec![ClassFeatureSourceUnit {
            book: "inner_sea_combat".to_string(),
            source_file: "isc_abilities_class.lst".to_string(),
            source_line: 1,
            key: "Ranger Combat Style".to_string(),
            name: "Ranger Combat Style".to_string(),
            type_facet: None,
        }];

        generate(&corpus_root, &grants_root, &out_dir, "2026-08-23T00:00:00Z", &units, &BTreeMap::new())
            .expect("generate must succeed against a well-formed fixture");

        let cf_dir = out_dir.join("inner_sea_combat/class_feature");
        let mut found: Option<String> = None;
        for entry in walkdir(&cf_dir) {
            if entry.extension().and_then(|e| e.to_str()) == Some("json") {
                found = Some(std::fs::read_to_string(&entry).unwrap());
            }
        }
        let written = found.expect("generate must write exactly one json file for the record");
        let json: Value = serde_json::from_str(&written).unwrap();

        assert_eq!(
            json["data"]["description"].as_str(),
            Some(crate::rules_core::shape_b_v1::REDACTED_PI_MARKER),
            "data.description must be redacted even though the weak bare-substring scan alone would miss \
             the OCR-glitched spelling: {written}"
        );
        assert!(!written.contains("Gorurn"), "{written}");

        std::fs::remove_dir_all(&tmp).ok();
    }

    /// Regression test: `declared_pi_shipping_audit`'s DESC-PI-SHIPPED
    /// check requires `pi_field` to still list `"description"` on a record
    /// that is BOTH name-PI and desc-PI at once -- found live this cycle
    /// (91 of 140 renamed `class_feature` units) when the rename branch
    /// OVERWROTE `pi_field` with `"name"`/`"name,raw_tokens"` instead of
    /// appending to whatever the description screen already set.
    #[test]
    fn generate_keeps_description_in_pi_field_when_both_name_and_desc_are_pi() {
        let tmp = std::env::temp_dir().join(format!("cf-generate-both-pi-{}", std::process::id()));
        let corpus_root = tmp.join("corpus_root");
        let grants_root = tmp.join("grants_root");
        let out_dir = tmp.join("out");
        std::fs::remove_dir_all(&tmp).ok();

        let book_dir = corpus_root.join("pathfinder/paizo/roleplaying_game/adventurers_guide");
        std::fs::create_dir_all(&book_dir).unwrap();
        std::fs::write(
            book_dir.join("ag_abilities_class.lst"),
            "Both PI\t\tNAMEISPI:YES\tDESCISPI:YES\tCATEGORY:Special Ability\tDESC:Secret prose.\n",
        )
        .unwrap();

        let units = vec![ClassFeatureSourceUnit {
            book: "adventurers_guide".to_string(),
            source_file: "ag_abilities_class.lst".to_string(),
            source_line: 1,
            key: "Both PI".to_string(),
            name: "Both PI".to_string(),
            type_facet: None,
        }];

        generate(&corpus_root, &grants_root, &out_dir, "2026-08-23T00:00:00Z", &units, &BTreeMap::new())
            .expect("generate must succeed against a well-formed fixture");

        let cf_dir = out_dir.join("adventurers_guide/class_feature");
        let mut found: Option<String> = None;
        for entry in walkdir(&cf_dir) {
            if entry.extension().and_then(|e| e.to_str()) == Some("json") {
                found = Some(std::fs::read_to_string(&entry).unwrap());
            }
        }
        let written = found.expect("generate must write exactly one json file for the unit");
        let json: Value = serde_json::from_str(&written).unwrap();
        assert_eq!(json["data"]["description"].as_str(), Some(crate::rules_core::shape_b_v1::REDACTED_PI_MARKER));
        let pi_field = json["pi_field"].as_str().unwrap();
        assert!(pi_field.split(',').any(|p| p == "description"), "pi_field must still list \"description\": {pi_field}");
        assert!(pi_field.split(',').any(|p| p == "name"), "pi_field must also list \"name\": {pi_field}");

        std::fs::remove_dir_all(&tmp).ok();
    }

    /// Recursive helper for the test above -- this module has no existing
    /// directory walker exposed for tests to reuse.
    fn walkdir(dir: &Path) -> Vec<PathBuf> {
        let mut out = Vec::new();
        let mut stack = vec![dir.to_path_buf()];
        while let Some(d) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&d) else { continue };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else {
                    out.push(path);
                }
            }
        }
        out
    }

    /// W19-INTEGRATE (adversarial review, `advanced_class_guide` finding on
    /// `ecclesitheurge/domain_mastery.json`): a PI-redacted description must
    /// not survive verbatim inside `raw_tokens`'s own `DESC` entry -- that
    /// was exactly the live leak this test guards. Mutating
    /// `redact_desc_token_if_pi` to a no-op (or dropping its call site) must
    /// turn this red; it is the mutation-proof this cycle's fix is real.
    #[test]
    fn redact_desc_token_if_pi_redacts_only_desc_when_license_says_pi_redacted() {
        let mut tokens = vec![
            RawToken { key: "KEY".to_string(), value: "Ecclesitheurge ~ Domain Mastery".to_string() },
            RawToken { key: "DESC".to_string(), value: "Full Product-Identity prose goes here.".to_string() },
            RawToken { key: "CATEGORY".to_string(), value: "Special Ability".to_string() },
        ];
        redact_desc_token_if_pi(&mut tokens, crate::rules_core::shape_b_v1::License::PiRedacted);
        assert_eq!(
            tokens.iter().find(|t| t.key == "DESC").map(|t| t.value.as_str()),
            Some(crate::rules_core::shape_b_v1::REDACTED_PI_MARKER)
        );
        // Every non-DESC token is untouched.
        assert_eq!(tokens[0].value, "Ecclesitheurge ~ Domain Mastery");
        assert_eq!(tokens[2].value, "Special Ability");
    }

    #[test]
    fn redact_desc_token_if_pi_is_a_no_op_when_license_is_not_pi_redacted() {
        let mut tokens =
            vec![RawToken { key: "DESC".to_string(), value: "Ordinary open-content prose.".to_string() }];
        redact_desc_token_if_pi(&mut tokens, crate::rules_core::shape_b_v1::License::Ogl);
        assert_eq!(tokens[0].value, "Ordinary open-content prose.");
    }

    // ------------------------------------------------------------------
    // `redact_concatenated_blacklist_tokens` -- SD-32 card 11,
    // T9-round-4-followup's own live-found gap: a clean-name/clean-
    // description record whose SOME OTHER raw token concatenates a
    // blacklisted term into an identifier with no separator
    // (`AldoriDefensiveParryLVL`, `CalistrianHunter ~ ...`).
    // ------------------------------------------------------------------

    /// Live shape, `adventurers_guide/class_feature/aldori_defender/
    /// defensive_parry.json`'s own `DEFINE`/`BONUS` tokens, reproduced
    /// exactly. Mutating `redact_concatenated_blacklist_tokens` to a no-op
    /// (or dropping its call site in `generate`) must turn this red -- the
    /// mutation-proof this cycle's fix is real.
    #[test]
    fn redact_concatenated_blacklist_tokens_redacts_a_pascalcase_concatenated_hit() {
        let mut tokens = vec![
            RawToken { key: "KEY".to_string(), value: "Aldori Defender ~ Defensive Parry".to_string() },
            RawToken { key: "DEFINE".to_string(), value: "AldoriDefensiveParryLVL|0".to_string() },
            RawToken {
                key: "BONUS".to_string(),
                value: "VAR|AldoriDefensiveParryLVL|FighterLVL".to_string(),
            },
            RawToken { key: "CATEGORY".to_string(), value: "Special Ability".to_string() },
        ];
        let any_redacted = redact_concatenated_blacklist_tokens(&mut tokens);
        assert!(any_redacted);
        assert_eq!(tokens[0].value, crate::rules_core::shape_b_v1::REDACTED_PI_MARKER, "KEY carries a word-bounded hit too");
        assert_eq!(tokens[1].value, crate::rules_core::shape_b_v1::REDACTED_PI_MARKER);
        assert_eq!(tokens[2].value, crate::rules_core::shape_b_v1::REDACTED_PI_MARKER);
        // The unrelated CATEGORY token is untouched.
        assert_eq!(tokens[3].value, "Special Ability");
    }

    #[test]
    fn redact_concatenated_blacklist_tokens_is_a_no_op_on_clean_tokens() {
        let mut tokens = vec![
            RawToken { key: "KEY".to_string(), value: "Fighter ~ Bravery".to_string() },
            RawToken { key: "DEFINE".to_string(), value: "FighterBraveryLVL|0".to_string() },
        ];
        let any_redacted = redact_concatenated_blacklist_tokens(&mut tokens);
        assert!(!any_redacted);
        assert_eq!(tokens[0].value, "Fighter ~ Bravery");
        assert_eq!(tokens[1].value, "FighterBraveryLVL|0");
    }

    #[test]
    fn redact_concatenated_blacklist_tokens_skips_a_value_already_redacted() {
        let mut tokens = vec![RawToken {
            key: "DESC".to_string(),
            value: crate::rules_core::shape_b_v1::REDACTED_PI_MARKER.to_string(),
        }];
        // The marker string itself must never trip a second (spurious) hit.
        let any_redacted = redact_concatenated_blacklist_tokens(&mut tokens);
        assert!(!any_redacted);
        assert_eq!(tokens[0].value, crate::rules_core::shape_b_v1::REDACTED_PI_MARKER);
    }

    // ------------------------------------------------------------------
    // `true_class_by_key` -- SD-31 wave 23, `OPEN-ISSUES.md` row 334's
    // closing note: `ClassFeatureData.class` must prefer the real
    // granting class from `data/class_feature_grants/`, not the key's
    // own owner-segment text (which is wrong whenever that segment names
    // an archetype rather than the real class).
    // ------------------------------------------------------------------

    fn write_grant_file(dir: &Path, class_slug: &str, facts: &[(&str, &str)]) {
        std::fs::create_dir_all(dir).unwrap();
        let body: Vec<Value> = facts
            .iter()
            .map(|(key, class)| {
                serde_json::json!({
                    "key": key,
                    "class": class,
                    "level": 1,
                    "level_explicit": true,
                    "gate": "preclass",
                    "corpus_record_exists": true,
                    "source": {"kind": "class_feature_grant", "path": "x.lst", "sha256": "abc", "line": 1}
                })
            })
            .collect();
        std::fs::write(dir.join(format!("{class_slug}.json")), serde_json::to_string_pretty(&body).unwrap())
            .unwrap();
    }

    /// The live defect this cycle fixes, reproduced as a unit test: a key
    /// whose OWN text names an archetype (`Sigilus`) must resolve to the
    /// real granting class (`Magus`) once a grant fact states it -- the
    /// exact `sigilus/inscribe_rune.json` shape `OPEN-ISSUES.md` row 334
    /// cites. Mutating `true_class_by_key` to always return an empty map
    /// (i.e. deleting the grant-data lookup) turns this red, since the
    /// assertion requires the CORRECTED class, not the key-prefix guess.
    #[test]
    fn true_class_by_key_resolves_an_archetype_owned_key_to_the_real_class() {
        let tmp = std::env::temp_dir().join(format!("cf-grants-test-{}", std::process::id()));
        let book_dir = tmp.join("adventurers_guide");
        write_grant_file(&book_dir, "magus", &[("Sigilus ~ Inscribe Rune", "Magus")]);
        let map = true_class_by_key(&tmp, "adventurers_guide");
        assert_eq!(map.get("Sigilus ~ Inscribe Rune").map(String::as_str), Some("Magus"));
        std::fs::remove_dir_all(&tmp).ok();
    }

    /// A key with no matching grant fact must be simply ABSENT from the
    /// map (never guessed) so [`generate`]'s own fallback -- the key-prefix
    /// split -- is what runs for it.
    #[test]
    fn true_class_by_key_omits_keys_with_no_grant_fact() {
        let tmp = std::env::temp_dir().join(format!("cf-grants-test-nogrant-{}", std::process::id()));
        let book_dir = tmp.join("adventurers_guide");
        write_grant_file(&book_dir, "magus", &[("Sigilus ~ Inscribe Rune", "Magus")]);
        let map = true_class_by_key(&tmp, "adventurers_guide");
        assert!(!map.contains_key("Some Other Key ~ Feature"));
        std::fs::remove_dir_all(&tmp).ok();
    }

    /// A book with no `data/class_feature_grants/<book>/` directory at all
    /// (5 of the 21 in-scope books have none yet) must return an empty
    /// map, not error -- every record in that book keeps the old
    /// key-prefix-split fallback.
    #[test]
    fn true_class_by_key_returns_empty_map_for_a_book_with_no_grant_directory() {
        let tmp = std::env::temp_dir().join(format!("cf-grants-test-missing-{}", std::process::id()));
        let map = true_class_by_key(&tmp, "book_of_the_damned_volume_1");
        assert!(map.is_empty());
    }

    /// Two grant facts in the SAME book claiming the SAME key under
    /// DIFFERENT classes must be refused (key absent from the map, not an
    /// arbitrary pick) -- the anti-gaming shape this cycle's own doc
    /// comment names explicitly. Mutating the `Some(_) => ...` ambiguity
    /// arm to `resolved.insert(fact.key, fact.class)` (last write wins)
    /// turns this red.
    #[test]
    fn true_class_by_key_refuses_a_key_claimed_by_two_different_classes() {
        let tmp = std::env::temp_dir().join(format!("cf-grants-test-ambiguous-{}", std::process::id()));
        let book_dir = tmp.join("core_rulebook");
        write_grant_file(&book_dir, "fighter", &[("Shared ~ Feature", "Fighter")]);
        write_grant_file(&book_dir, "paladin", &[("Shared ~ Feature", "Paladin")]);
        let map = true_class_by_key(&tmp, "core_rulebook");
        assert!(!map.contains_key("Shared ~ Feature"), "an ambiguous key must not resolve to either class");
        std::fs::remove_dir_all(&tmp).ok();
    }

    /// Two grant facts in the SAME book agreeing on the SAME key/class
    /// (e.g. two different feature entries in the same class's grant file
    /// citing the class consistently) must resolve normally -- agreement
    /// is not ambiguity.
    #[test]
    fn true_class_by_key_resolves_when_multiple_facts_agree() {
        let tmp = std::env::temp_dir().join(format!("cf-grants-test-agree-{}", std::process::id()));
        let book_dir = tmp.join("core_rulebook");
        write_grant_file(
            &book_dir,
            "fighter",
            &[("Fighter ~ Bravery", "Fighter"), ("Weapon Master ~ Bravery", "Fighter")],
        );
        let map = true_class_by_key(&tmp, "core_rulebook");
        assert_eq!(map.get("Weapon Master ~ Bravery").map(String::as_str), Some("Fighter"));
        std::fs::remove_dir_all(&tmp).ok();
    }

    /// End-to-end regression, wave-23 integration review finding: every
    /// `true_class_by_key` test above exercises the helper in isolation --
    /// none of them prove [`generate`] actually WIRES its result into the
    /// shipped `data.class` field. A verbatim revert of `generate`'s one
    /// load-bearing line (`true_class.get(&unit.key).cloned().or_else(||
    /// key_owner.clone())` -> `key_owner.clone()`) left the entire lib
    /// suite green, because nothing called `generate` with real grant data
    /// and checked its OUTPUT. This does: a real `Sigilus ~ Inscribe Rune`
    /// row (the exact `OPEN-ISSUES.md` row 334 example) goes through the
    /// whole pipeline -- corpus row read, wiring-class index, PI screen,
    /// directory placement -- and the written JSON file's `data.class` must
    /// be `"Magus"`, not `"Sigilus"` (the key-prefix guess `generate` used
    /// before this cycle, and what it falls back to for every key with no
    /// grant fact).
    #[test]
    fn generate_writes_the_true_class_not_the_key_prefix_guess() {
        let tmp = std::env::temp_dir().join(format!("cf-generate-e2e-{}", std::process::id()));
        let corpus_root = tmp.join("corpus_root");
        let grants_root = tmp.join("grants_root");
        let out_dir = tmp.join("out");
        std::fs::remove_dir_all(&tmp).ok();

        let book_dir = corpus_root.join("pathfinder/paizo/roleplaying_game/adventurers_guide");
        std::fs::create_dir_all(&book_dir).unwrap();
        std::fs::write(
            book_dir.join("ag_abilities_class.lst"),
            "Inscribe Rune\t\tKEY:Sigilus ~ Inscribe Rune\t\tCATEGORY:Special Ability\tDESC:You inscribe a rune.\n",
        )
        .unwrap();

        write_grant_file(&grants_root.join("adventurers_guide"), "magus", &[("Sigilus ~ Inscribe Rune", "Magus")]);

        let units = vec![ClassFeatureSourceUnit {
            book: "adventurers_guide".to_string(),
            source_file: "ag_abilities_class.lst".to_string(),
            source_line: 1,
            key: "Sigilus ~ Inscribe Rune".to_string(),
            name: "Inscribe Rune".to_string(),
            type_facet: None,
        }];

        let report = generate(
            &corpus_root,
            &grants_root,
            &out_dir,
            "2026-08-20T00:00:00Z",
            &units,
            &BTreeMap::new(),
        )
        .expect("generate must succeed against a well-formed fixture");
        assert_eq!(report.written, 1, "the one fixture unit must be written exactly once");

        // Directory placement stays keyed on the key's own owner segment
        // ("Sigilus") -- see `generate`'s own comment -- only the `class`
        // field's VALUE is corrected.
        let written = std::fs::read_to_string(out_dir.join("adventurers_guide/class_feature/sigilus/inscribe_rune.json"))
            .expect("generate must write to the key-owner-keyed path even when `class` is corrected");
        let json: Value = serde_json::from_str(&written).unwrap();
        assert_eq!(
            json["data"]["class"].as_str(),
            Some("Magus"),
            "generate() must ship the grant-fact-corrected class, not the key-prefix guess (\"Sigilus\"): {written}"
        );

        std::fs::remove_dir_all(&tmp).ok();
    }

    // ------------------------------------------------------------------
    // SD-32 card 11 (`epic-2-cause-closure`, T2a/T12 combined cycle):
    // the pool-catalog / type-facet / corpus-class fallbacks.
    // ------------------------------------------------------------------

    /// Drift guard: [`DISPATCHED_CLASS_TITLE_NAMES`] is a hand-typed const,
    /// not derived from the real enums (cache_gen's own disjoint-file-touch
    /// convention forbids importing `v06_work_inventory.rs`'s `bin`-only
    /// logic) -- this pins its count against the same five real enums
    /// `modelled_class_books()` iterates, so an enum roster change that
    /// forgets to update this list fails loudly here instead of silently
    /// mis-resolving `data.class`.
    #[test]
    fn dispatched_class_title_names_len_matches_the_real_34_class_roster() {
        use crate::rules_core::rules_tables::acg::AcgClassId;
        use crate::rules_core::rules_tables::apg::ApgClassId;
        use crate::rules_core::rules_tables::crb::class_tables::ClassId;
        use crate::rules_core::rules_tables::pathfinder_unchained::class_chassis::PuClassId;
        use crate::rules_core::rules_tables::ultimate_combat::UcClassId;
        let real_total =
            ClassId::ALL.len() + ApgClassId::ALL.len() + AcgClassId::ALL.len() + UcClassId::ALL.len() + PuClassId::ALL.len();
        assert_eq!(DISPATCHED_CLASS_TITLE_NAMES.len(), real_total);
        assert_eq!(real_total, 34);
    }

    #[test]
    fn pool_catalog_owner_resolves_a_registered_category_label_to_its_real_dispatched_class() {
        assert_eq!(pool_catalog_owner("Rage Power"), Some("Barbarian"));
        assert_eq!(pool_catalog_owner("Domain"), Some("Cleric"));
        assert_eq!(pool_catalog_owner("Battle Mystery"), Some("Oracle"));
    }

    #[test]
    fn pool_catalog_owner_refuses_a_cross_class_collision() {
        // "Druid Domain" belongs to Druid's own PREABILITY-gated variant,
        // never Cleric's Domain pool -- the same guard
        // `v06_work_inventory.rs::class_feature_pool_group_matches` enforces.
        assert_eq!(pool_catalog_owner("Druid Domain"), None);
    }

    #[test]
    fn pool_catalog_owner_refuses_a_verified_false_suffix_match() {
        assert_eq!(pool_catalog_owner("Shifter's Blessing"), None);
    }

    #[test]
    fn pool_catalog_owner_returns_none_for_an_unregistered_group() {
        assert_eq!(pool_catalog_owner("Wild Talent"), None);
    }

    #[test]
    fn type_facet_dispatched_owner_reads_the_class_feature_marker() {
        assert_eq!(
            type_facet_dispatched_owner(Some("Barbarian Class Feature.Skald Class Feature.RagePower")),
            Some("Barbarian".to_string())
        );
    }

    #[test]
    fn type_facet_dispatched_owner_returns_none_with_no_marker() {
        assert_eq!(type_facet_dispatched_owner(Some("SpecialQuality.KiPower")), None);
    }

    #[test]
    fn corpus_class_owner_resolves_a_genuinely_undispatched_real_class() {
        let mut corpus_class_names = BTreeMap::new();
        corpus_class_names.insert("vigilante".to_string(), "Vigilante".to_string());
        assert_eq!(
            corpus_class_owner("Vigilante Talent", &corpus_class_names),
            Some("Vigilante".to_string())
        );
    }

    #[test]
    fn corpus_class_owner_returns_none_when_the_group_names_no_corpus_class() {
        let mut corpus_class_names = BTreeMap::new();
        corpus_class_names.insert("vigilante".to_string(), "Vigilante".to_string());
        assert_eq!(corpus_class_owner("Domain Power", &corpus_class_names), None);
    }

    /// `"Ki Power"` names no class in its own text at all (unlike
    /// `"Rage Power"`, whose suffix IS the pool-catalog's own registered
    /// word) -- only [`CATEGORY_LABEL_ALIASES`]' verified table resolves
    /// it, to a corpus-declared class ("Monk") that need not be dispatched
    /// for the lookup itself to succeed.
    #[test]
    fn category_label_alias_owner_resolves_a_verified_text_free_label() {
        let mut corpus_class_names = BTreeMap::new();
        corpus_class_names.insert("monk".to_string(), "Monk".to_string());
        assert_eq!(category_label_alias_owner("Ki Power", &corpus_class_names), Some("Monk".to_string()));
    }

    /// The alias table's target may itself be a genuinely-undispatched
    /// corpus-declared class ("Kineticist") -- proving this tier closes
    /// T2a-residual for such a label without claiming T12's engine gap is
    /// also closed (the class stays unmodelled; only `data.class` becomes
    /// honest).
    #[test]
    fn category_label_alias_owner_resolves_to_an_undispatched_corpus_declared_class() {
        let mut corpus_class_names = BTreeMap::new();
        corpus_class_names.insert("kineticist".to_string(), "Kineticist".to_string());
        assert_eq!(category_label_alias_owner("Wild Talent", &corpus_class_names), Some("Kineticist".to_string()));
    }

    #[test]
    fn category_label_alias_owner_returns_none_for_an_unregistered_group() {
        let corpus_class_names = BTreeMap::new();
        assert_eq!(category_label_alias_owner("Refined Education", &corpus_class_names), None);
    }

    /// `"Domain Power"` is deliberately ABSENT from [`CATEGORY_LABEL_ALIASES`]
    /// -- the section comment above the table names why (multi-owner, no
    /// per-record signal). This test pins the absence so a future edit
    /// cannot silently re-introduce the anti-gaming failure the census
    /// flagged without a reviewer noticing the assertion break.
    #[test]
    fn category_label_alias_owner_refuses_the_known_multi_owner_and_not_class_owned_labels() {
        let corpus_class_names = BTreeMap::new();
        for label in ["Domain Power", "Demonic Obedience"] {
            assert_eq!(
                category_label_alias_owner(label, &corpus_class_names),
                None,
                "{label} must stay unmapped -- see CATEGORY_LABEL_ALIASES's section comment"
            );
        }
    }

    #[test]
    fn corpus_class_names_from_inventory_json_reads_every_class_kind_unit() {
        let json = serde_json::json!({
            "units": [
                {"kind": "class", "name": "Vigilante"},
                {"kind": "class", "name": "Magus"},
                {"kind": "class_feature", "name": "Sneak Attack"},
            ]
        })
        .to_string();
        let names = corpus_class_names_from_inventory_json(&json).unwrap();
        assert_eq!(names.get("vigilante").map(String::as_str), Some("Vigilante"));
        assert_eq!(names.get("magus").map(String::as_str), Some("Magus"));
        assert_eq!(names.len(), 2, "the class_feature unit must not be counted as a class");
    }

    /// End-to-end: a pool-catalog category label ("Rage Power ~ ...") with
    /// NO grant fact must still ship the real dispatched owner
    /// ("Barbarian"), not the raw key-prefix guess ("Rage Power") --
    /// exactly T2a's own shape. Mutating `generate`'s
    /// `.or_else(|| pool_catalog_owner(group)...)` line out turns this red.
    #[test]
    fn generate_writes_the_pool_catalog_owner_for_an_unregistered_grant_key() {
        let tmp = std::env::temp_dir().join(format!("cf-generate-pool-{}", std::process::id()));
        let corpus_root = tmp.join("corpus_root");
        let grants_root = tmp.join("grants_root");
        let out_dir = tmp.join("out");
        std::fs::remove_dir_all(&tmp).ok();

        let book_dir = corpus_root.join("pathfinder/paizo/roleplaying_game/core_rulebook");
        std::fs::create_dir_all(&book_dir).unwrap();
        std::fs::write(
            book_dir.join("cr_abilities_class.lst"),
            "Ferocity\t\tKEY:Rage Power ~ Ferocity\t\tCATEGORY:Special Ability\tDESC:You fight through wounds.\n",
        )
        .unwrap();
        // Deliberately no grant file for `core_rulebook` -- exercises the
        // pool-catalog fallback, not `true_class_by_key`.
        std::fs::create_dir_all(&grants_root).unwrap();

        let units = vec![ClassFeatureSourceUnit {
            book: "core_rulebook".to_string(),
            source_file: "cr_abilities_class.lst".to_string(),
            source_line: 1,
            key: "Rage Power ~ Ferocity".to_string(),
            name: "Ferocity".to_string(),
            type_facet: None,
        }];

        generate(&corpus_root, &grants_root, &out_dir, "2026-08-20T00:00:00Z", &units, &BTreeMap::new())
            .expect("generate must succeed against a well-formed fixture");

        let written = std::fs::read_to_string(out_dir.join("core_rulebook/class_feature/rage_power/ferocity.json"))
            .expect("generate must still write to the key-owner-keyed path");
        let json: Value = serde_json::from_str(&written).unwrap();
        assert_eq!(
            json["data"]["class"].as_str(),
            Some("Barbarian"),
            "generate() must ship the pool-catalog-resolved class, not the raw label \"Rage Power\": {written}"
        );

        std::fs::remove_dir_all(&tmp).ok();
    }

    /// End-to-end: a category label whose true owner is a real,
    /// corpus-declared but UNDISPATCHED class ("Vigilante Talent" ->
    /// "Vigilante") must ship that class's own name, not the category
    /// label -- the T2a/T12 overlap fix. Proves `generate`'s
    /// `corpus_class_names` parameter is actually wired, not merely
    /// accepted.
    #[test]
    fn generate_writes_the_corpus_declared_undispatched_owner_for_the_t2a_t12_overlap() {
        let tmp = std::env::temp_dir().join(format!("cf-generate-corpusclass-{}", std::process::id()));
        let corpus_root = tmp.join("corpus_root");
        let grants_root = tmp.join("grants_root");
        let out_dir = tmp.join("out");
        std::fs::remove_dir_all(&tmp).ok();

        let book_dir = corpus_root.join("pathfinder/paizo/campaign_setting/inner_sea_intrigue");
        std::fs::create_dir_all(&book_dir).unwrap();
        std::fs::write(
            book_dir.join("isi_abilities_class.lst"),
            "Coax Information\t\tKEY:Vigilante Talent ~ Coax Information\t\tCATEGORY:Special Ability\tDESC:You coax secrets.\n",
        )
        .unwrap();
        std::fs::create_dir_all(&grants_root).unwrap();

        let units = vec![ClassFeatureSourceUnit {
            book: "inner_sea_intrigue".to_string(),
            source_file: "isi_abilities_class.lst".to_string(),
            source_line: 1,
            key: "Vigilante Talent ~ Coax Information".to_string(),
            name: "Coax Information".to_string(),
            type_facet: None,
        }];

        let mut corpus_class_names = BTreeMap::new();
        corpus_class_names.insert("vigilante".to_string(), "Vigilante".to_string());

        generate(
            &corpus_root,
            &grants_root,
            &out_dir,
            "2026-08-20T00:00:00Z",
            &units,
            &corpus_class_names,
        )
        .expect("generate must succeed against a well-formed fixture");

        let written =
            std::fs::read_to_string(out_dir.join("inner_sea_intrigue/class_feature/vigilante_talent/coax_information.json"))
                .expect("generate must still write to the key-owner-keyed path");
        let json: Value = serde_json::from_str(&written).unwrap();
        assert_eq!(
            json["data"]["class"].as_str(),
            Some("Vigilante"),
            "generate() must ship the corpus-declared undispatched owner, not the category label \"Vigilante Talent\": {written}"
        );

        std::fs::remove_dir_all(&tmp).ok();
    }

    /// End-to-end: `"Ki Power"` names no class in its own text at all --
    /// neither the pool catalog's suffix match nor a `type_facet`/`TYPE:`
    /// text match can reach "Monk" from it. Only
    /// [`category_label_alias_owner`]'s verified table (SD-32 card 11,
    /// T2a-residual cycle) resolves it. Mutating `generate`'s
    /// `.or_else(|| category_label_alias_owner(...))` line out turns this
    /// red.
    #[test]
    fn generate_writes_the_alias_owner_for_a_text_free_category_label() {
        let tmp = std::env::temp_dir().join(format!("cf-generate-alias-{}", std::process::id()));
        let corpus_root = tmp.join("corpus_root");
        let grants_root = tmp.join("grants_root");
        let out_dir = tmp.join("out");
        std::fs::remove_dir_all(&tmp).ok();

        let book_dir = corpus_root.join("pathfinder/paizo/roleplaying_game/ultimate_magic");
        std::fs::create_dir_all(&book_dir).unwrap();
        std::fs::write(
            book_dir.join("um_abilities_class.lst"),
            "Wholeness of Body\t\tKEY:Ki Power ~ Wholeness of Body\t\tCATEGORY:Special Ability\tDESC:You heal yourself.\n",
        )
        .unwrap();
        std::fs::create_dir_all(&grants_root).unwrap();

        let units = vec![ClassFeatureSourceUnit {
            book: "ultimate_magic".to_string(),
            source_file: "um_abilities_class.lst".to_string(),
            source_line: 1,
            key: "Ki Power ~ Wholeness of Body".to_string(),
            name: "Wholeness of Body".to_string(),
            type_facet: None,
        }];

        let mut corpus_class_names = BTreeMap::new();
        corpus_class_names.insert("monk".to_string(), "Monk".to_string());

        generate(
            &corpus_root,
            &grants_root,
            &out_dir,
            "2026-08-23T00:00:00Z",
            &units,
            &corpus_class_names,
        )
        .expect("generate must succeed against a well-formed fixture");

        let written = std::fs::read_to_string(out_dir.join("ultimate_magic/class_feature/ki_power/wholeness_of_body.json"))
            .expect("generate must still write to the key-owner-keyed path");
        let json: Value = serde_json::from_str(&written).unwrap();
        assert_eq!(
            json["data"]["class"].as_str(),
            Some("Monk"),
            "generate() must ship the verified alias owner, not the category label \"Ki Power\": {written}"
        );

        std::fs::remove_dir_all(&tmp).ok();
    }

    // -------------------------------------------------------------
    // SD-32 card 11, decision 23a: `"Domain Power"`'s multi-owner
    // resolution (`scan_domain_power_owners` / `domain_power_owning_classes`).
    // -------------------------------------------------------------

    #[test]
    fn effective_lst_key_prefers_the_explicit_key_token_over_the_display_name() {
        assert_eq!(
            effective_lst_key("Chaos\t\tKEY:Inquisitor Domain ~ Chaos\t\tCATEGORY:Special Ability"),
            "Inquisitor Domain ~ Chaos"
        );
        // No explicit KEY: token -- falls back to the first field, the
        // same shape [`foreign_citations`]/[`key_owner`] already rely on.
        assert_eq!(
            effective_lst_key("Core Domain ~ Azata Subdomain (Chaos)\tCATEGORY:Internal\tABILITY:..."),
            "Core Domain ~ Azata Subdomain (Chaos)"
        );
    }

    #[test]
    fn scan_domain_power_owners_resolves_core_druid_inquisitor_and_bare_grants() {
        let dir = std::env::temp_dir().join(format!("cf-scan-domain-power-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).unwrap();

        // A "Core Domain ~" wrapper (Cleric + Paladin, per the section
        // comment's verified evidence).
        std::fs::write(
            dir.join("core.lst"),
            "Core Domain ~ Azata Subdomain (Chaos)\tCATEGORY:Internal\t\tABILITY:Special Ability|AUTOMATIC|Domain Power ~ Chaos Blade|PREVARGTEQ:X,8\n",
        )
        .unwrap();
        // An "Inquisitor Domain ~" wrapper for the SAME power -- proves
        // the multi-owner union, not a single winner.
        std::fs::write(
            dir.join("inquisitor.lst"),
            "Chaos\t\tKEY:Inquisitor Domain ~ Chaos\t\tCATEGORY:Special Ability\tABILITY:Special Ability|AUTOMATIC|Domain Power ~ Chaos Blade|PREVARGTEQ:Y,1\n",
        )
        .unwrap();
        // A "Druid Domain ~" wrapper for a DIFFERENT power -- proves the
        // scan does not cross-contaminate targets.
        std::fs::write(
            dir.join("druid.lst"),
            "Alchemy\t\tKEY:Druid Domain ~ Alchemy\t\tCATEGORY:Special Ability\tABILITY:Special Ability|AUTOMATIC|Domain Power ~ Alchemical Simulacrum|PREVARGTEQ:Z,1\n",
        )
        .unwrap();
        // A bare (no class-prefixed wrapper) subdomain grant -- resolves
        // the same Cleric+Paladin owners as an explicit "Core Domain ~".
        std::fs::write(
            dir.join("bare.lst"),
            "Dragon Subdomain\t\tPREMULT:1\tABILITY:Special Ability|AUTOMATIC|Domain Power ~ Venomous Stare|PREVARGTEQ:W,1\n",
        )
        .unwrap();
        // A `.MOD` line naming a foreign feat category -- the real oracle
        // shape (`acg_feats.lst:177`): the FIRST FIELD is the
        // `CATEGORY=...MOD` token itself, no name column at all. Must NOT
        // be read as a domain grant point.
        std::fs::write(
            dir.join("mod.lst"),
            "CATEGORY=FEAT|Believer's Boon.MOD\tDEFINE:DomainAirLVL|0\t\tABILITY:Special Ability|AUTOMATIC|Domain Power ~ Agile Feet|PREVARGTEQ:V,1\n",
        )
        .unwrap();

        let owners = scan_domain_power_owners(&dir);

        assert_eq!(
            owners.get("Chaos Blade").cloned(),
            Some(BTreeSet::from(["Cleric".to_string(), "Inquisitor".to_string(), "Paladin".to_string()])),
            "a power granted by BOTH a Core and an Inquisitor wrapper must union both classes: {owners:?}"
        );
        assert_eq!(
            owners.get("Alchemical Simulacrum").cloned(),
            Some(BTreeSet::from(["Druid".to_string()])),
            "the Druid-only grant must not leak Cleric/Paladin/Inquisitor: {owners:?}"
        );
        assert_eq!(
            owners.get("Venomous Stare").cloned(),
            Some(BTreeSet::from(["Cleric".to_string(), "Paladin".to_string()])),
            "a bare subdomain grant must resolve the same owners as Core Domain ~: {owners:?}"
        );
        assert!(
            owners.get("Agile Feet").is_none(),
            "a .MOD line naming a foreign feat category must not be read as a domain grant point: {owners:?}"
        );

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn domain_power_owning_classes_returns_none_outside_the_namespace_and_when_unmapped() {
        let mut owners = BTreeMap::new();
        owners.insert("Chaos Blade".to_string(), BTreeSet::from(["Cleric".to_string(), "Paladin".to_string()]));

        assert_eq!(
            domain_power_owning_classes("Domain Power ~ Chaos Blade", &owners),
            Some(vec!["Cleric".to_string(), "Paladin".to_string()])
        );
        // Not a "Domain Power ~ " key at all.
        assert_eq!(domain_power_owning_classes("Ki Power ~ Wholeness of Body", &owners), None);
        // In the namespace, but the scan found no grant chain for it.
        assert_eq!(domain_power_owning_classes("Domain Power ~ Nonexistent Power", &owners), None);
    }

    #[test]
    fn generate_writes_the_multi_owner_classes_for_a_domain_power_record_never_collapsing_class() {
        let tmp = std::env::temp_dir().join(format!("cf-generate-domain-power-{}", std::process::id()));
        let corpus_root = tmp.join("corpus_root");
        let grants_root = tmp.join("grants_root");
        let out_dir = tmp.join("out");
        std::fs::remove_dir_all(&tmp).ok();

        // The record itself, cited exactly the way `v06_work_inventory`
        // enumerates it (own KEY: token line, no class name in its own
        // text or TYPE/PRE tokens -- matches the real corpus shape).
        let book_dir = corpus_root.join("pathfinder/paizo/roleplaying_game/core_rulebook");
        std::fs::create_dir_all(&book_dir).unwrap();
        std::fs::write(
            book_dir.join("cr_abilities_class.lst"),
            "Chaos Blade\t\tKEY:Domain Power ~ Chaos Blade\t\tCATEGORY:Special Ability\tDESC:Anarchic weapon.\n",
        )
        .unwrap();
        // The upstream grant chain, in a SEPARATE file (as the real
        // oracle has it: cr_domains.lst grants the "Core Domain ~"
        // wrapper; apg_abilities_class.lst is where the wrapper itself
        // grants the power) -- proves the scan is corpus-wide, not
        // scoped to the unit's own citation file.
        let elsewhere_dir = corpus_root.join("pathfinder/paizo/roleplaying_game/advanced_players_guide");
        std::fs::create_dir_all(&elsewhere_dir).unwrap();
        std::fs::write(
            elsewhere_dir.join("apg_abilities_class.lst"),
            "Core Domain ~ Azata Subdomain (Chaos)\tCATEGORY:Internal\t\tABILITY:Special Ability|AUTOMATIC|Domain Power ~ Chaos Blade|PREVARGTEQ:X,8\n",
        )
        .unwrap();
        std::fs::create_dir_all(&grants_root).unwrap();

        let units = vec![ClassFeatureSourceUnit {
            book: "core_rulebook".to_string(),
            source_file: "cr_abilities_class.lst".to_string(),
            source_line: 1,
            key: "Domain Power ~ Chaos Blade".to_string(),
            name: "Chaos Blade".to_string(),
            type_facet: None,
        }];
        let corpus_class_names = BTreeMap::new();

        generate(
            &corpus_root,
            &grants_root,
            &out_dir,
            "2026-08-23T00:00:00Z",
            &units,
            &corpus_class_names,
        )
        .expect("generate must succeed against a well-formed fixture");

        let written = std::fs::read_to_string(out_dir.join("core_rulebook/class_feature/domain_power/chaos_blade.json"))
            .expect("generate must write the record");
        let json: Value = serde_json::from_str(&written).unwrap();
        assert_eq!(
            json["data"]["classes"].as_array().map(|a| a.iter().map(|v| v.as_str().unwrap()).collect::<Vec<_>>()),
            Some(vec!["Cleric", "Paladin"]),
            "must record BOTH owning classes, never collapsed to one: {written}"
        );
        // `class` (the existing single-owner field) stays exactly what
        // it was before this cycle -- the key-owner fallback "Domain
        // Power" -- because none of the six single-class tiers resolve
        // it, and this cycle deliberately does not force one.
        assert_eq!(
            json["data"]["class"].as_str(),
            Some("Domain Power"),
            "single-owner `class` must not be forced/collapsed by the new multi-owner field: {written}"
        );

        std::fs::remove_dir_all(&tmp).ok();
    }

    /// Row 21 (`decisions.md`): reproduces `core_rulebook`'s real
    /// `"Bloodline Tracker"` shape verbatim -- one base declaration plus
    /// THREE separate `.MOD` rows appending distinct `BONUS:VAR` tokens
    /// (`cr_abilities_class.lst:1704-1707`, cycle 7's own finding). Before
    /// the fix, `generate()` read only the base row's own line and every
    /// `.MOD`-appended token was silently dropped -- all 8 real
    /// `bloodline_tracker.json` records corpus-wide carried 1-2 tokens and
    /// ZERO `BONUS:VAR` tokens. Proves every `.MOD` row's tokens now
    /// survive into `data.raw_tokens`, alongside the base row's own.
    #[test]
    fn generate_unions_every_mod_row_s_tokens_into_raw_tokens_not_just_the_base_row() {
        let tmp = std::env::temp_dir().join(format!("cf-generate-modunion-{}", std::process::id()));
        let corpus_root = tmp.join("corpus_root");
        let grants_root = tmp.join("grants_root");
        let out_dir = tmp.join("out");
        std::fs::remove_dir_all(&tmp).ok();

        let book_dir = corpus_root.join("pathfinder/paizo/roleplaying_game/core_rulebook");
        std::fs::create_dir_all(&book_dir).unwrap();
        std::fs::write(
            book_dir.join("cr_abilities_class.lst"),
            "Bloodline Tracker\tCATEGORY:Internal\tKEY:Bloodline Tracker\tSOURCEPAGE:p.1\tBONUS:VAR|BloodlineLVL|SorcererLVL|TYPE=Base\n\
             CATEGORY=Internal|Bloodline Tracker.MOD\tBONUS:VAR|BloodlineCasterLVL|SorcererLVL|TYPE=Base\n\
             CATEGORY=Internal|Bloodline Tracker.MOD\tBONUS:VAR|BloodlineProgressionLVL|SorcererLVL|TYPE=Base\n",
        )
        .unwrap();
        std::fs::create_dir_all(&grants_root).unwrap();

        let units = vec![ClassFeatureSourceUnit {
            book: "core_rulebook".to_string(),
            source_file: "cr_abilities_class.lst".to_string(),
            source_line: 1,
            key: "Bloodline Tracker".to_string(),
            name: "Bloodline Tracker".to_string(),
            type_facet: None,
        }];

        generate(&corpus_root, &grants_root, &out_dir, "2026-08-23T00:00:00Z", &units, &BTreeMap::new())
            .expect("generate must succeed against a well-formed fixture");

        let written = std::fs::read_to_string(out_dir.join("core_rulebook/class_feature/bloodline_tracker/bloodline_tracker.json"))
            .expect("generate must write the base record's own file");
        let json: Value = serde_json::from_str(&written).unwrap();
        let bonus_vars: Vec<String> = json["data"]["raw_tokens"]
            .as_array()
            .expect("raw_tokens must be an array")
            .iter()
            .filter(|t| t["key"].as_str() == Some("BONUS") && t["value"].as_str().is_some_and(|v| v.starts_with("VAR|")))
            .map(|t| t["value"].as_str().unwrap().to_string())
            .collect();
        assert_eq!(
            bonus_vars,
            vec![
                "VAR|BloodlineLVL|SorcererLVL|TYPE=Base".to_string(),
                "VAR|BloodlineCasterLVL|SorcererLVL|TYPE=Base".to_string(),
                "VAR|BloodlineProgressionLVL|SorcererLVL|TYPE=Base".to_string(),
            ],
            "all three BONUS:VAR tokens (base row's own plus both .MOD rows') must survive into raw_tokens, not just the base row's: {written}"
        );

        std::fs::remove_dir_all(&tmp).ok();
    }

    /// A record with NO `.MOD` row targeting it must see byte-identical
    /// `raw_tokens` to before the closure-based fix -- the common case
    /// (most `class_feature` records have no sibling `.MOD` row at all)
    /// must not regress.
    #[test]
    fn generate_leaves_a_record_with_no_mod_row_unchanged() {
        let tmp = std::env::temp_dir().join(format!("cf-generate-nomod-{}", std::process::id()));
        let corpus_root = tmp.join("corpus_root");
        let grants_root = tmp.join("grants_root");
        let out_dir = tmp.join("out");
        std::fs::remove_dir_all(&tmp).ok();

        let book_dir = corpus_root.join("pathfinder/paizo/roleplaying_game/core_rulebook");
        std::fs::create_dir_all(&book_dir).unwrap();
        std::fs::write(
            book_dir.join("cr_abilities_class.lst"),
            "Ferocity\t\tKEY:Rage Power ~ Ferocity\t\tCATEGORY:Special Ability\tDESC:You fight through wounds.\n",
        )
        .unwrap();
        std::fs::create_dir_all(&grants_root).unwrap();

        let units = vec![ClassFeatureSourceUnit {
            book: "core_rulebook".to_string(),
            source_file: "cr_abilities_class.lst".to_string(),
            source_line: 1,
            key: "Rage Power ~ Ferocity".to_string(),
            name: "Ferocity".to_string(),
            type_facet: None,
        }];

        generate(&corpus_root, &grants_root, &out_dir, "2026-08-23T00:00:00Z", &units, &BTreeMap::new())
            .expect("generate must succeed against a well-formed fixture");

        let written = std::fs::read_to_string(out_dir.join("core_rulebook/class_feature/rage_power/ferocity.json"))
            .expect("generate must still write to the key-owner-keyed path");
        let json: Value = serde_json::from_str(&written).unwrap();
        let tokens = json["data"]["raw_tokens"].as_array().expect("raw_tokens must be an array");
        assert_eq!(tokens.len(), 3, "no .MOD row targets this unit, so raw_tokens must be exactly this base row's own tokens (KEY/CATEGORY/DESC): {written}");
    }

    /// A `.MOD` row targeting this unit's `key` sits in a NESTED `support/`
    /// file under the same book directory (the real `ultimate_combat/
    /// support/uc_abilities_class_ag.lst` shape) AND carries a real
    /// Product-Identity term (`"Aldori"` -- a real Golarion proper noun,
    /// the same blacklist term `redact_concatenated_blacklist_tokens`
    /// already screens every other token for). Proves BOTH: the nested
    /// file's row is found by the closure fix (a `BONUS` token appears at
    /// all, where before this fix none of this row's tokens would), AND
    /// `§15`'s PI discipline still applies to a token this fix newly
    /// surfaces -- it ships REDACTED, never the real PI value, and the
    /// record's `license`/`pi_field` correctly reflect the redaction.
    #[test]
    fn generate_finds_a_mod_row_in_a_nested_support_file_and_still_redacts_its_pi() {
        let tmp = std::env::temp_dir().join(format!("cf-generate-nested-{}", std::process::id()));
        let corpus_root = tmp.join("corpus_root");
        let grants_root = tmp.join("grants_root");
        let out_dir = tmp.join("out");
        std::fs::remove_dir_all(&tmp).ok();

        let book_dir = corpus_root.join("pathfinder/paizo/roleplaying_game/ultimate_combat");
        std::fs::create_dir_all(&book_dir).unwrap();
        std::fs::write(
            book_dir.join("uc_abilities_class.lst"),
            "Bonus Feat\t\tKEY:Master Of Many Styles ~ Bonus Feat\t\tCATEGORY:Special Ability\tDESC:x\n",
        )
        .unwrap();
        let support_dir = book_dir.join("support");
        std::fs::create_dir_all(&support_dir).unwrap();
        std::fs::write(
            support_dir.join("uc_abilities_class_ag.lst"),
            "CATEGORY=Special Ability|Master Of Many Styles ~ Bonus Feat.MOD\tBONUS:VAR|MonkBonusFeat_AldoriStyle|1\n",
        )
        .unwrap();
        std::fs::create_dir_all(&grants_root).unwrap();

        let units = vec![ClassFeatureSourceUnit {
            book: "ultimate_combat".to_string(),
            source_file: "uc_abilities_class.lst".to_string(),
            source_line: 1,
            key: "Master Of Many Styles ~ Bonus Feat".to_string(),
            name: "Bonus Feat".to_string(),
            type_facet: None,
        }];

        generate(&corpus_root, &grants_root, &out_dir, "2026-08-24T00:00:00Z", &units, &BTreeMap::new())
            .expect("generate must succeed");

        let written = std::fs::read_to_string(out_dir.join("ultimate_combat/class_feature/master_of_many_styles/bonus_feat.json"))
            .expect("generate must write the record");
        let json: Value = serde_json::from_str(&written).unwrap();
        let tokens = json["data"]["raw_tokens"].as_array().unwrap();
        let bonus = tokens.iter().find(|t| t["key"].as_str() == Some("BONUS"));
        assert!(bonus.is_some(), "the nested support/ file's .MOD row must be found: {written}");
        assert_eq!(
            bonus.unwrap()["value"].as_str(),
            Some(crate::rules_core::shape_b_v1::REDACTED_PI_MARKER),
            "a real PI term (\"Aldori\") in a newly-surfaced .MOD token must still be redacted, never shipped raw: {written}"
        );
        assert_eq!(json["license"].as_str(), Some("PI-REDACTED"), "license must reflect the redaction: {written}");

        std::fs::remove_dir_all(&tmp).ok();
    }
}
