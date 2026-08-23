//! Equipment/equipment-modifier "gap" JSON cache generator (SD-31
//! `epic-6-ingest-lanes` F5/F6, `SD31-E6-F5-002`).
//!
//! Writes `data/corpus/<book>/equipment/*.json` (or
//! `data/corpus/<book>/equipment/equipmods/*.json` for `Equipmods`-
//! category rows, matching `cache_gen::ultimate_equipment`'s own nested
//! layout) by DUMPING the current, already-completed state of
//! `rules_core::rules_tables::equipment_gap_tables::equipment_gap_rows()`
//! -- per `decisions.md §11.3`, this generator never re-parses raw PCGen
//! LST to derive a field's *value*; every value written here is read
//! straight from the compiled Rust module.
//!
//! `equipment_gap_tables` itself is a SEPARATE, already-shipped lever
//! from any per-book hand-authored table: it is the corpus-wide
//! `not-ingested` residue for 8 already-compiled books' `equipment`/
//! `equipment_modifier` kinds -- generated once by `gen_equipment_gap_tables`
//! against the real PCGen oracle and checked in as plain Rust data, but
//! (before this cycle) never dumped to `data/corpus/`, so neither
//! `corpus_literal_sweep` nor the equipment-effect wiring probe could ever
//! see it. Same shape as `OPEN-ISSUES.md` row 11/row 12's finding for
//! Ultimate Equipment, applied to 704 records across 8 OTHER books.
//!
//! **Now includes the `"UE"` (Ultimate Equipment) row slice's own gap
//! residue** (`decisions.md §20`; previously excluded on the assumption
//! `cache_gen::ultimate_equipment` fully owned that book -- true only for
//! the keys its own hand-authored tables list, not for `.lst` content
//! absent from them). `held` (from `hand_authored_equipment_rows()`) makes
//! this additive-only: no record `cache_gen::ultimate_equipment` already
//! wrote is touched or duplicated.
//!
//! ## Citation resolution
//!
//! Unlike `cache_gen::ultimate_equipment` (which knows its book's 4
//! category-file names statically), `equipment_gap_tables::EquipmentGapRow`
//! carries no source file at all -- it only carries the compiled value and
//! the book it came from. [`find_citation`] recovers a real, checkable
//! `(file, line)` citation by searching the book's own `.lst` files: first
//! every file directly under the book directory (flat, sorted for
//! determinism), then -- only if nothing flat matches -- every `.lst` file
//! in the book's subdirectories. The search tries, in order, a `KEY:<key>`
//! tab-delimited field match (needed for the `~`-qualified equipmod keys,
//! e.g. `"Special Ability ~ Allying ~ Weapon"`), then an exact match on the
//! row's first tab-delimited column against `key`, then (only when
//! `key != name`) the same first-column match against `name`.
//!
//! **A citation resolved only inside a subdirectory is written honestly
//! (real path, real line) but will not reach `literal-verified` today** --
//! `corpus_literal_sweep`'s own `--json-out` book-attribution derives
//! `book` from the citation's immediate parent directory name
//! (`OPEN-ISSUES.md` row 22, a pre-existing, out-of-territory defect this
//! module does not fix). None of the 8 books this generator actually
//! resolved a citation for needed the subdirectory fallback (see this
//! cycle's receipt), so the defect is named for completeness, not hit.
//!
//! ## PI screening -- BOTH contracts, NAME and DESCRIPTION
//!
//! `cache_gen::ultimate_equipment` computes a row's [`DeclaredProductIdentity`]
//! (via [`declared_pi_at`]) but only ever passes `declared.description` into
//! `pi_screening::classify_optional_field_declared` -- `entry.name` is
//! written raw, with NO screen at all, the confirmed defect named in this
//! wave's dispatch preamble. This module screens `name` too:
//!
//! * `name` is a REQUIRED field (a record's identity), so it cannot be
//!   redacted to a marker the way an optional `description` can --
//!   [`DeclaredProductIdentity`]'s own doc comment: "the only way not to
//!   publish it is not to publish the row." When `declared.name` is `true`
//!   OR the shared blacklist term scan
//!   (`pi_screening::classify_field("name", entry.name)`) flags the name,
//!   this generator does not write the record at all -- it is counted in
//!   [`GenerationReport::name_pi_excluded`], never silently dropped.
//! * `description` keeps `cache_gen::ultimate_equipment`'s existing,
//!   correct redact-to-marker behaviour via
//!   `pi_screening::classify_optional_field_declared`.
//!
//! Zero rows hit either name screen against the real corpus at generation
//! time (this cycle's receipt records the exact count: 0); the code path
//! is exercised by a unit test with a synthetic PI-declared name instead,
//! because the real 704-row population happens not to need it.

// `SD31-E6-F5-003`: `book_routing`, `find_citation`, `disabled_identity_
// column`, `declared_pi_at`, `slugify`, and `write_json` below are
// `pub(crate)` so `cache_gen::hand_authored_equipment` (a sibling module,
// same file territory) can reuse this module's already-verified citation
// resolution, PI reading, and no-clobber write discipline instead of
// duplicating it for a second, book-list-shaped generator. Pure visibility
// widening -- zero behavior change to this module's own `generate()`.

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Serialize;

use crate::rules_core::cache_gen::WiringClassIndex;
use crate::rules_core::pi_screening::{self, DeclaredProductIdentity};
use crate::rules_core::rules_tables::equipment_gap_tables;

// ---------------------------------------------------------------------
// Shape B schema (decisions.md §7) -- own local copy, per
// cache_gen::acg / cache_gen::ultimate_equipment's own established
// no-shared-types-file convention (decisions.md §11.3).
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
pub struct CacheRecord<T: Serialize> {
    pub population: Population,
    pub completeness: Completeness,
    pub ingested_at: String,
    pub data: T,
    pub source: Source,
    pub wiring_class: String,
    pub wiring_class_signals: Vec<String>,
    pub license: crate::rules_core::shape_b_v1::License,
    pub pi_field: Option<String>,
    pub pi_marker: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EquipmentData {
    pub key: String,
    pub category: String,
    pub name: String,
    pub cost_gp: Option<f64>,
    pub weight_lbs: Option<f64>,
    pub description: Option<String>,
}

// ---------------------------------------------------------------------
// Book routing -- `EquipmentGapRow.book` short codes
// (`equipment_resolver::EQUIPMENT_BOOK_*`) to a `wiring_class`/
// `data/corpus/` book id and its real LST directory. `"UE"` is
// deliberately absent: `cache_gen::ultimate_equipment` owns that book.
// ---------------------------------------------------------------------

pub(crate) fn book_routing(short_code: &str) -> Option<(&'static str, &'static str)> {
    match short_code {
        "CRB" => Some(("core_rulebook", "pathfinder/paizo/roleplaying_game/core_rulebook")),
        // `decisions.md §9`: B1's 3 gap rows cite the shared `core_essentials`
        // library directory, not a `bestiary`-named directory -- the corpus
        // carries no such directory. `find_citation` needs the directory the
        // files physically live in; the `book_id` half (`"bestiary"`) is what
        // actually names the shipped `data/corpus/` book, independent of
        // where the source `.lst` lives (`gen_equipment_gap_tables.rs`'s `B1`
        // `BookInput` doc comment names the same file-vs-attribution split).
        "B1" => Some(("bestiary", "pathfinder/paizo/roleplaying_game/core_essentials")),
        "APG" => Some(("advanced_players_guide", "pathfinder/paizo/roleplaying_game/advanced_players_guide")),
        "ACG" => Some(("advanced_class_guide", "pathfinder/paizo/roleplaying_game/advanced_class_guide")),
        "ARG" => Some(("advanced_race_guide", "pathfinder/paizo/roleplaying_game/advanced_race_guide")),
        "UC" => Some(("ultimate_combat", "pathfinder/paizo/roleplaying_game/ultimate_combat")),
        "UI" => Some(("ultimate_intrigue", "pathfinder/paizo/roleplaying_game/ultimate_intrigue")),
        "UM" => Some(("ultimate_magic", "pathfinder/paizo/roleplaying_game/ultimate_magic")),
        "UPSI" => Some(("ultimate_psionics", "pathfinder/dreamscarred_press/ultimate_psionics")),
        "UW" => Some(("ultimate_wilderness", "pathfinder/paizo/roleplaying_game/ultimate_wilderness")),
        // SD31-E6-F10-003: 13 further already-compiled books, same shape as
        // `UW` above -- each carries its own real equipment `.lst` files
        // directly in its own corpus directory (no shared-library `B1`-style
        // host-discovery hazard), confirmed against `v06_work_inventory.rs`'s
        // `COMPILED_RULE_SETS` before routing.
        "ISG" => Some(("inner_sea_gods", "pathfinder/paizo/campaign_setting/inner_sea_gods")),
        "OA" => Some(("occult_adventures", "pathfinder/paizo/roleplaying_game/occult_adventures")),
        "HA" => Some(("horror_adventures", "pathfinder/paizo/roleplaying_game/horror_adventures")),
        "MYTHIC" => Some(("mythic_adventures", "pathfinder/paizo/roleplaying_game/mythic_adventures")),
        "ISC" => Some(("inner_sea_combat", "pathfinder/paizo/campaign_setting/inner_sea_combat")),
        "ISR" => Some(("inner_sea_races", "pathfinder/paizo/campaign_setting/inner_sea_races")),
        "ISWG" => Some(("inner_sea_world_guide", "pathfinder/paizo/campaign_setting/inner_sea_world_guide")),
        "MC" => Some(("monster_codex", "pathfinder/paizo/roleplaying_game/monster_codex")),
        "ISI" => Some(("inner_sea_intrigue", "pathfinder/paizo/campaign_setting/inner_sea_intrigue")),
        "B2" => Some(("bestiary_2", "pathfinder/paizo/roleplaying_game/bestiary_2")),
        "B3" => Some(("bestiary_3", "pathfinder/paizo/roleplaying_game/bestiary_3")),
        "B4" => Some(("bestiary_4", "pathfinder/paizo/roleplaying_game/bestiary_4")),
        "BOTD2" => Some(("book_of_the_damned_volume_2", "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_2")),
        // SD-32 T9 residual (`decisions.md §20`): `gen_equipment_gap_tables.rs`'s
        // `BOOK_INPUTS` already declared `EQUIPMENT_BOOK_ISTEM`/`EQUIPMENT_BOOK_ISM`
        // and generated rows for both books (43 + 6, re-derived against the
        // pinned oracle), but this match had no arm for either code, so
        // `generate()`'s `let Some(..) = book_routing(book) else { continue }`
        // silently dropped every one of those rows before they ever reached
        // `data/corpus/` -- the config table and the cache writer had drifted
        // out of sync. Fixed at the root: both codes now route to their real
        // corpus directories.
        "ISTEM" => Some(("inner_sea_temples", "pathfinder/paizo/campaign_setting/inner_sea_temples")),
        "ISM" => Some(("inner_sea_magic", "pathfinder/paizo/campaign_setting/inner_sea_magic")),
        // SD-32 T9 residual: `adventurers_guide` had no `BOOK_INPUTS` entry at
        // all (115 `not-ingested` equipment units, none captured). Added
        // alongside this routing arm; see `gen_equipment_gap_tables.rs`'s
        // `EQUIPMENT_BOOK_AG` `BookInput`.
        "AG" => Some(("adventurers_guide", "pathfinder/paizo/roleplaying_game/adventurers_guide")),
        // SD-32 T9 residual (`decisions.md §20`): `"UE"` was deliberately
        // absent here on the assumption `cache_gen::ultimate_equipment`
        // "owns" the book -- true for the 1,613 keys its own hand-authored
        // `equipment_tables`/`equipmod_tables` modules hold, but that
        // generator only ever dumps what THOSE tables list. It never reads
        // `equipment_gap_tables::equipment_gap_rows()`'s 64 `"UE"` rows at
        // all, so the drift here matches this file's own `ISTEM`/`ISM`
        // fix's shape exactly: a config table (`gen_equipment_gap_tables.rs`
        // already declares an `EQUIPMENT_BOOK_UE` `BookInput`) computed real
        // rows this match silently dropped before they ever reached
        // `data/corpus/`. `held` (from `hand_authored_equipment_rows()`,
        // which already indexes every one of the 1,613 keys) makes routing
        // UE additive-only: no already-shipped UE record is touched or
        // duplicated, only the residue nobody wrote gets a corpus record.
        "UE" => Some(("ultimate_equipment", "pathfinder/paizo/roleplaying_game/ultimate_equipment")),
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

fn list_lst_files_flat(dir: &Path) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().and_then(|e| e.to_str()) == Some("lst"))
        .collect();
    out.sort();
    out
}

fn list_lst_files_recursive_excluding_flat(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack: Vec<PathBuf> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("lst") {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

/// Finds a line carrying the exact tab-delimited field `KEY:<record_key>`.
fn find_by_key_field(lst_path: &Path, record_key: &str) -> Option<u32> {
    let content = std::fs::read_to_string(lst_path).ok()?;
    let needle = format!("KEY:{record_key}");
    content
        .lines()
        .enumerate()
        .find(|(_, line)| line.split('\t').any(|field| field == needle))
        .map(|(idx, _)| (idx + 1) as u32)
}

/// Finds `record_name` as an exact match on a line's first tab-delimited column.
fn find_exact_first_column(lst_path: &Path, record_name: &str) -> Option<u32> {
    let content = std::fs::read_to_string(lst_path).ok()?;
    content
        .lines()
        .enumerate()
        .find(|(_, line)| line.split('\t').next().unwrap_or("") == record_name)
        .map(|(idx, _)| (idx + 1) as u32)
}

/// Finds a `.COPY=<record_name>` variant line's first column -- the same
/// fallback `cache_gen::ultimate_equipment::find_copy_variant` established
/// (a masterwork/size/special-ability variant declared as a `.COPY=` of a
/// base row rather than its own standalone `KEY:`-bearing declaration; e.g.
/// core_rulebook's `cr_equipmods.lst:665`:
/// `Special Ability ~ +1 ~ Weapon.COPY=PLUS1W`).
fn find_copy_variant(lst_path: &Path, record_name: &str) -> Option<u32> {
    let content = std::fs::read_to_string(lst_path).ok()?;
    let needle = format!(".COPY={record_name}");
    content
        .lines()
        .enumerate()
        .find(|(_, line)| line.split('\t').next().unwrap_or("").ends_with(&needle))
        .map(|(idx, _)| (idx + 1) as u32)
}

/// Tries, per identifier (`key` first, then `name` when `key != name`), in
/// order: `KEY:<id>`, `.COPY=<id>`, first-column `id` -- across every file
/// in `files` -- returning the first hit as `(path relative to `book_dir`,
/// line)`.
///
/// `.COPY=<id>` is tried BEFORE the first-column match for the same `id`
/// (`decisions.md §17`'s "search for the existing path", re-applied to a
/// second drift in this same resolver): a `.lst` block frequently declares
/// a base row whose own DISPLAY name (first column) coincidentally equals
/// the short `KEY:`-less identity a *different* row's `.COPY=<id>` line
/// creates -- e.g. `advanced_class_guide/acg_equipmods.lst`, where line 27
/// is `Answering\tKEY:Special Ability ~ Answering ~ Weapon\t...` (a
/// template row whose own key is the long string, display name
/// "Answering") and line 95 is
/// `Special Ability ~ Answering ~ Weapon.COPY=Answering\t...VISIBLE:NO`
/// (the row that actually creates the playable object keyed `"Answering"`).
/// Trying first-column before `.COPY=` let the template row's coincidental
/// display name win, citing the wrong line while still reporting
/// mechanically-correct data (`.COPY=` inherits the base row's fields) --
/// so the defect surfaced as a silent wrong-citation, not a wrong value.
/// A `.COPY=<id>` target is a stronger identity signal than a bare
/// first-column match: PCGen's own `.COPY=` syntax's SOLE purpose is to
/// mint a new object under that exact key, whereas a first-column string is
/// just that row's display name and can coincide with an unrelated row's
/// key for cosmetic reasons. Promoting it is safe by construction: it can
/// only ever change an outcome when `.COPY=<id>` *and* a first-column `id`
/// both exist in the same file tier, and two real PCGen objects sharing one
/// literal key would itself be a data collision this corpus has never
/// exhibited (confirmed by the full-population regression this fix's own
/// commit re-derives: every previously-resolved citation this reorder could
/// have touched is unchanged, named in the cycle receipt).
fn try_files(files: &[PathBuf], book_dir: &Path, key: &str, name: &str) -> Option<(PathBuf, u32)> {
    let resolve = |id: &str| -> Option<(PathBuf, u32)> {
        for path in files {
            if let Some(line) = find_by_key_field(path, id) {
                return Some((path.strip_prefix(book_dir).ok()?.to_path_buf(), line));
            }
        }
        for path in files {
            if let Some(line) = find_copy_variant(path, id) {
                return Some((path.strip_prefix(book_dir).ok()?.to_path_buf(), line));
            }
        }
        for path in files {
            if let Some(line) = find_exact_first_column(path, id) {
                return Some((path.strip_prefix(book_dir).ok()?.to_path_buf(), line));
            }
        }
        None
    };
    if let Some(hit) = resolve(key) {
        return Some(hit);
    }
    if key != name {
        if let Some(hit) = resolve(name) {
            return Some(hit);
        }
    }
    None
}

/// `true` when `path`'s own file name contains the substring `"equip"` --
/// the naming convention every real equipment/equipment-modifier source
/// file in this corpus follows across all 9 books this module and its
/// sibling `cache_gen::hand_authored_equipment` route citations into
/// (`*_equip_arms_armor.lst`, `*_equip_general.lst`,
/// `*_equip_magic_items.lst`, `*_equipmods.lst`, `up_equipment.lst`) --
/// verified corpus-wide before relying on it: every one of the ~1,900
/// already-correctly-resolved citations in the shipped corpus already
/// lands in a file this predicate accepts (`OPEN-ISSUES.md` row 90's own
/// re-derivation).
///
/// A weapon/armor proficiency file (`*_profs_weapon.lst`) or a class-
/// ability file (`*_abilities_class.lst`) can carry a `KEY:`-tagged row
/// that coincidentally matches an equipment record's own identity string
/// without being the equipment row at all -- `find_citation` uses this
/// predicate to try every strategy against equipment-shaped files FIRST,
/// so a coincidental match there can never beat the real row.
fn is_equipment_shaped_file(path: &Path) -> bool {
    path.file_name().and_then(|n| n.to_str()).is_some_and(|name| name.contains("equip"))
}

/// Resolves `(path relative to `book_dir`, line)` for `key`/`name`.
///
/// Search order (`OPEN-ISSUES.md` row 90 fix): every strategy
/// [`try_files`] tries is run FIRST against only the book directory's
/// equipment-shaped flat `.lst` files ([`is_equipment_shaped_file`]),
/// then against its equipment-shaped nested files, and only if NEITHER
/// tier resolves anything does the search widen to every remaining
/// (non-equipment-shaped) flat file and then every remaining nested file.
/// This is narrower than the pre-fix order (which tried `KEY:` matches
/// across every file, equipment-shaped or not, before ever trying a
/// first-column match anywhere) but strictly more permissive than
/// requiring an equipment-shaped file outright -- a book whose real
/// content genuinely lives outside an "equip"-named file (none observed
/// in this corpus today, per the doc comment above) still resolves via
/// the fallback tier rather than reporting unresolved.
///
/// `book_dir` is the absolute directory; the returned path is relative to
/// it (POSIX-separated) so the caller can build both the citation
/// `source.path` and re-open the file.
pub(crate) fn find_citation(book_dir: &Path, key: &str, name: &str) -> Option<(PathBuf, u32)> {
    let flat = list_lst_files_flat(book_dir);
    let (flat_equip, flat_other): (Vec<PathBuf>, Vec<PathBuf>) =
        flat.into_iter().partition(|p| is_equipment_shaped_file(p));
    if let Some(hit) = try_files(&flat_equip, book_dir, key, name) {
        return Some(hit);
    }
    let nested = list_lst_files_recursive_excluding_flat(book_dir);
    let (nested_equip, nested_other): (Vec<PathBuf>, Vec<PathBuf>) =
        nested.into_iter().partition(|p| is_equipment_shaped_file(p));
    if let Some(hit) = try_files(&nested_equip, book_dir, key, name) {
        return Some(hit);
    }
    if let Some(hit) = try_files(&flat_other, book_dir, key, name) {
        return Some(hit);
    }
    try_files(&nested_other, book_dir, key, name)
}

/// `true` when the real corpus row at `lst_path:line` (1-indexed) opens
/// with a `#` in its identity column -- PCGen's own comment marker for a
/// row the maintainers explicitly disabled, per `Trap::DisabledLine`
/// (`src/pcgen_import/corpus_traps.rs`). A missing/unreadable row is
/// treated as NOT disabled (the pre-existing unresolved-citation path
/// already handles a genuinely absent row; this check only fires on a
/// row that resolved and needs its identity column read).
pub(crate) fn disabled_identity_column(lst_path: &Path, line: u32) -> bool {
    if line == 0 {
        return false;
    }
    let Ok(content) = std::fs::read_to_string(lst_path) else { return false };
    let Some(row) = content.lines().nth((line - 1) as usize) else { return false };
    row.trim_start().starts_with('#')
}

/// Reads [`DeclaredProductIdentity`] off the real corpus line at
/// `lst_path:line` (1-indexed) -- `§53.5`'s declared-PI reader.
pub(crate) fn declared_pi_at(lst_path: &Path, line: u32) -> DeclaredProductIdentity {
    if line == 0 {
        return DeclaredProductIdentity::default();
    }
    let Ok(content) = std::fs::read_to_string(lst_path) else {
        return DeclaredProductIdentity::default();
    };
    let Some(row) = content.lines().nth((line - 1) as usize) else {
        return DeclaredProductIdentity::default();
    };
    let tokens: Vec<(&str, &str)> =
        row.split('\t').filter_map(|field| field.split_once(':')).collect();
    pi_screening::declared_product_identity(tokens)
}

pub(crate) fn slugify(name: &str, used: &mut BTreeSet<String>) -> String {
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

/// Writes `record` to `<out_dir>/<slug>.json` -- UNLESS a file already
/// exists there, in which case it is left untouched and `Ok(false)` is
/// returned. A pre-existing file at this exact path means a DIFFERENT,
/// already-shipped ingest run already claimed this identity slot (this
/// module's own `used`/`slugify` collision-avoidance only tracks slugs
/// used WITHIN one `generate()` call; it cannot see a book's real,
/// already-committed corpus). Overwriting would silently discard
/// already-verified data no `git status` review would ever flag as a
/// deletion -- confirmed the hard way this cycle: `core_rulebook`'s gap
/// row `"Intelligent Item Purpose (Slay All)"` slugifies to the same
/// filename as an already-shipped, richer,
/// `key: "Intelligent Item ~ Purpose / Slay All"` record at a DIFFERENT
/// real citation line (446 vs. this row's 895) -- two real corpus rows,
/// same slug, and the first `write_json` implementation clobbered the
/// better one before this guard existed.
/// Reads `<out_dir>/<slug>.json`'s own `source.line` (`None` if the file
/// is absent, unreadable, or carries no such field -- never fabricated;
/// callers treat `None` the same as "not a same-line rerun", i.e. any
/// existing-but-unparseable file still blocks a write via `write_json`'s
/// own `path.exists()` guard, this function only ever widens what CAN be
/// disambiguated, never what gets skipped).
pub(crate) fn existing_source_line(out_dir: &Path, slug: &str) -> Option<u32> {
    let path = out_dir.join(format!("{slug}.json"));
    let text = std::fs::read_to_string(path).ok()?;
    let value: serde_json::Value = serde_json::from_str(&text).ok()?;
    value.get("source")?.get("line")?.as_u64().map(|n| n as u32)
}

pub(crate) fn write_json<T: Serialize>(out_dir: &Path, slug: &str, record: &CacheRecord<T>) -> std::io::Result<bool> {
    std::fs::create_dir_all(out_dir)?;
    let path = out_dir.join(format!("{slug}.json"));
    if path.exists() {
        return Ok(false);
    }
    let json = serde_json::to_string_pretty(record)
        .expect("CacheRecord<T> is a plain-data shape; serialization cannot fail");
    std::fs::write(path, json)?;
    Ok(true)
}

#[derive(Debug, Default)]
pub struct GenerationReport {
    pub equipment_written: usize,
    pub equipment_modifier_written: usize,
    /// Rows whose real LST citation could not be resolved -- honestly not
    /// written (never fabricated).
    pub unresolved_citations: Vec<String>,
    /// Rows whose `name` carries declared or blacklist-matched Product
    /// Identity -- honestly not written, per the module doc comment's
    /// "cannot redact a required field" rule.
    pub name_pi_excluded: Vec<String>,
    /// Rows whose slugified output path already exists on disk from a
    /// DIFFERENT, already-shipped ingest run -- not written, to guarantee
    /// this generator never clobbers already-verified data (see
    /// `write_json`'s doc comment for the real collision this guards).
    pub skipped_pre_existing: Vec<String>,
    /// Rows whose `key` is a PCGen removal directive (`.FORGET`), not a
    /// declared item -- not written, since it is not real equipment
    /// content (see the `generate()` loop's doc comment for the real
    /// example this guards against).
    pub excluded_non_content_directive: Vec<String>,
    /// Rows that slugified to the SAME path as an already-shipped file but
    /// whose own resolved citation `line` differs from that file's own
    /// `source.line` -- a genuine second real corpus row hiding behind one
    /// occupied slug (`write_json`'s doc comment names the exact incident
    /// this disambiguates), not a rerun of the same row. Written under a
    /// `slugify`-disambiguated (`-2`, `-3`, ...) sibling filename rather
    /// than silently dropped. Disjoint from `skipped_pre_existing`, whose
    /// entries share the SAME line as the file already on disk (a true
    /// idempotent rerun).
    pub disambiguated_collision: Vec<String>,
}

#[derive(Debug)]
pub enum GenerationError {
    CorpusUnreachable(PathBuf),
}

/// Generates the gap JSON cache for every book `equipment_gap_tables`
/// covers, under `out_root` (`data/corpus/`), reading real LST citations
/// from `corpus_root` (a PCGen `data/` checkout).
/// `ingested_at` is stamped at call time by the caller (real wall-clock
/// ISO-8601, never derived).
pub fn generate(
    corpus_root: &Path,
    out_root: &Path,
    ingested_at: &str,
) -> Result<GenerationReport, GenerationError> {
    let mut report = GenerationReport::default();
    let mut used_by_book: HashMap<&'static str, BTreeSet<String>> = HashMap::new();
    let mut sha_cache: HashMap<PathBuf, String> = HashMap::new();
    let mut wiring_indexes: HashMap<&'static str, WiringClassIndex> = HashMap::new();

    for entry in equipment_gap_tables::equipment_gap_rows() {
        // `.FORGET` is a PCGen directive (real example:
        // `advanced_class_guide/_pfs/pfs_acg_equip.lst:6-7`, a Pathfinder
        // Society legality overlay marking "Dust Knuckles"/"False Face" as
        // removed from PFS play) -- not a declared item. The upstream
        // `equipment_gap_tables.rs` (generated, not this cycle's file)
        // carries both as ordinary rows; dumping them as catalog equipment
        // would ship a removal directive as if it were content. Caught by
        // `pi_screening_regeneration_round_trip.rs`'s round-trip test
        // flagging them as "stale" against `cache_gen::acg`'s own real
        // table -- the corpus wins: they are not real equipment, so they
        // are not written, not merely reclassified as someone else's stale
        // leftover.
        if entry.key.ends_with(".FORGET") {
            report.excluded_non_content_directive.push(format!("{}:{}", entry.book, entry.key));
            continue;
        }
        let book = entry.book;
        let Some((book_id, book_rel_dir)) = book_routing(book) else {
            // Any future unmapped code: not this module's territory.
            continue;
        };
        let book_dir = corpus_root.join(book_rel_dir);
        if !book_dir.is_dir() {
            return Err(GenerationError::CorpusUnreachable(book_dir));
        }

        let Some((rel_path, line)) = find_citation(&book_dir, entry.key, entry.name) else {
            report.unresolved_citations.push(format!("{book_id}:{}", entry.key));
            continue;
        };
        let abs_path = book_dir.join(&rel_path);
        let rel_path_str = rel_path.to_string_lossy().replace('\\', "/");

        // `SD31-W4-INTEGRATE-001` (`OPEN-ISSUES.md` row 48/49): a leading
        // `#` in the identity column is PCGen's own comment marker --
        // `Trap::DisabledLine` (`src/pcgen_import/corpus_traps.rs`) already
        // names these rows "suppressed and must not be ingested, but they
        // look live." Same disposition as the `.FORGET` guard above (not
        // real content, excluded rather than shipped), same reason the
        // upstream compiled table doesn't know to skip them: 3 real
        // records were shipping the raw KEY: token (`CRRSVE_BRST_M`,
        // `CRRSVE_BRST_R`, `REACH`) as their player-facing `name` because
        // the row they cite is disabled.
        if disabled_identity_column(&abs_path, line) {
            report.excluded_non_content_directive.push(format!("{book_id}:{} (disabled #-line)", entry.key));
            continue;
        }

        let sha = match sha_cache.get(&abs_path) {
            Some(s) => s.clone(),
            None => {
                let s = sha256_file(&abs_path).unwrap_or_default();
                sha_cache.insert(abs_path.clone(), s.clone());
                s
            }
        };

        let declared = declared_pi_at(&abs_path, line);
        let (name_license, _, _, _) = pi_screening::classify_field("name", entry.name);
        if declared.name || name_license == crate::rules_core::shape_b_v1::License::PiRedacted {
            report.name_pi_excluded.push(format!("{book_id}:{}", entry.key));
            continue;
        }

        let wiring_index = wiring_indexes
            .entry(book_id)
            .or_insert_with(|| WiringClassIndex::build(book_id, &book_dir));
        let mut wiring_lines = wiring_index.lines();
        let (wiring_class, wiring_class_signals) =
            wiring_index.wiring_class_for(&mut wiring_lines, &rel_path_str, line, entry.key, entry.key);

        let (license, pi_field, pi_marker, stored_desc) = pi_screening::classify_optional_field_declared(
            "description",
            entry.description,
            declared.description,
        );

        let completeness =
            if entry.description.is_some() { Completeness::Full } else { Completeness::ChassisOnly };

        let record = CacheRecord {
            population: Population::InScope,
            completeness,
            ingested_at: ingested_at.to_string(),
            data: EquipmentData {
                key: entry.key.to_string(),
                category: entry.category.to_string(),
                name: entry.name.to_string(),
                cost_gp: entry.cost_gp,
                weight_lbs: entry.weight_lbs,
                description: stored_desc,
            },
            source: Source::LstToken {
                path: format!("{book_rel_dir}/{rel_path_str}"),
                sha256: sha,
                line,
                record_key: entry.key.to_string(),
            },
            wiring_class,
            wiring_class_signals,
            license,
            pi_field,
            pi_marker,
        };

        let used = used_by_book.entry(book_id).or_default();
        let mut slug = slugify(entry.key, used);
        let book_out = out_root.join(book_id).join("equipment");
        let (write_dir, is_modifier) = if entry.category == "Equipmods" {
            (book_out.join("equipmods"), true)
        } else {
            (book_out.clone(), false)
        };
        // A genuine second real corpus row can slugify to the same path an
        // already-shipped file occupies (`write_json`'s own doc comment
        // names the incident: `core_rulebook`'s "Intelligent Item Purpose
        // (Slay All)"/"(Slay Creature Type)" `.COPY=`-named rows collide
        // with the already-shipped BASE declaration's own richer record).
        // Disambiguate ONLY when the citation line genuinely differs --
        // when it matches, this is an ordinary idempotent rerun of the
        // SAME row, and must keep skipping exactly as before.
        if let Some(existing_line) = existing_source_line(&write_dir, &slug) {
            if existing_line != line {
                report.disambiguated_collision.push(format!(
                    "{book_id}:{} (line {line}, was slug of the line-{existing_line} record)",
                    entry.key
                ));
                slug = slugify(entry.key, used);
            }
        }
        let wrote = write_json(&write_dir, &slug, &record)
            .map_err(|_| GenerationError::CorpusUnreachable(book_out.clone()))?;
        if !wrote {
            report.skipped_pre_existing.push(format!("{book_id}:{}", entry.key));
            continue;
        }
        if is_modifier {
            report.equipment_modifier_written += 1;
        } else {
            report.equipment_written += 1;
        }
    }

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn book_routing_includes_ue_gap_residue() {
        // `decisions.md §20`: `equipment_gap_tables::equipment_gap_rows()`
        // already computes 64 `"UE"` rows (the hand-authored
        // `rules_tables::ultimate_equipment::equipment_tables` module's own
        // real coverage gap, e.g. "Aklys"/"Belt of Foraging" -- present in
        // the real `.lst` content, absent from the hand-authored table) but
        // `generate()`'s `let Some(..) = book_routing(book) else { continue
        // }` silently dropped every one before it ever reached
        // `data/corpus/`, because this match had no arm for `"UE"` at all
        // (same drift shape this file's own `ISTEM`/`ISM` fix already
        // named). `held` (built from `hand_authored_equipment_rows()`,
        // which already indexes UE's 1,613 hand-authored keys) protects
        // every already-shipped UE record from being touched or
        // duplicated -- routing UE only surfaces the residue nobody else
        // ever wrote.
        assert_eq!(
            book_routing("UE"),
            Some((
                "ultimate_equipment",
                "pathfinder/paizo/roleplaying_game/ultimate_equipment"
            ))
        );
    }

    /// Regression test for the real defect this cycle caught: a `.FORGET`
    /// row (a PCGen removal directive, e.g. ACG's real
    /// `_pfs/pfs_acg_equip.lst:6`: `Dust Knuckles.FORGET`) is not a
    /// declared item. `generate()`'s own filter recognizes it by its `key`
    /// suffix -- this test proves that check directly, independent of any
    /// file I/O, against both a real-shaped hit and a real-shaped miss.
    #[test]
    fn forget_directive_keys_are_recognized_and_ordinary_keys_are_not() {
        assert!("Dust Knuckles.FORGET".ends_with(".FORGET"));
        assert!("False Face.FORGET".ends_with(".FORGET"));
        assert!(!"Amorphous".ends_with(".FORGET"));
        assert!(!"Special Ability ~ Dueling ~ Melee".ends_with(".FORGET"));
    }

    /// `OPEN-ISSUES.md` row 48/49: 3 shipped records were sourced from a
    /// commented-out (`#`-prefixed) identity column and shipped the raw
    /// KEY: token as their player-facing name -- `CRRSVE_BRST_M`,
    /// `CRRSVE_BRST_R`, `REACH`. Regression test against the real,
    /// byte-for-byte disabled rows (`apg_equipmods.lst:13`,
    /// `uc_equipmods.lst:7`) and a real-shaped live row that must NOT be
    /// flagged.
    #[test]
    fn disabled_hash_prefixed_rows_are_recognized_and_live_rows_are_not() {
        let dir = std::env::temp_dir().join(format!("cgeq_disabled_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("book_equipmods.lst");
        std::fs::write(
            &file,
            "#Corrosive Burst\tKEY:CRRSVE_BRST_M\tTYPE:Weapon.Melee\n\
             Widget\tKEY:Special Ability ~ Widget ~ Weapon\tCOST:0\n",
        )
        .unwrap();
        assert!(disabled_identity_column(&file, 1), "a #-prefixed identity column must be flagged");
        assert!(
            !disabled_identity_column(&file, 2),
            "an ordinary live row must not be flagged"
        );
        assert!(!disabled_identity_column(&file, 0), "line 0 (no citation) is not disabled");
        assert!(
            !disabled_identity_column(&dir.join("nonexistent.lst"), 1),
            "an unreadable file is not disabled -- the unresolved-citation path handles absence"
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn book_routing_covers_every_non_ue_gap_book() {
        for code in [
            "CRB", "APG", "ACG", "ARG", "UC", "UI", "UM", "UPSI", "UW", "B1", "ISG", "OA", "HA",
            "MYTHIC", "ISC", "ISR", "ISWG", "MC", "ISI", "B2", "B3", "B4", "BOTD2", "ISTEM",
            "ISM", "AG",
        ] {
            assert!(book_routing(code).is_some(), "missing routing for {code}");
        }
    }

    /// `decisions.md §9`: `B1`'s 3 gap rows physically live under the
    /// shared `core_essentials` directory (there is no `bestiary`-named
    /// directory in the corpus at all), but the shipped book id must still
    /// be `"bestiary"` -- the directory a citation is found IN is not
    /// necessarily the book id the record ships under (`SOURCELONG`
    /// governs attribution, not file placement). Regression test for the
    /// exact defect this cycle fixed: before this cycle `B1` had no
    /// routing at all, so its 3 rows (previously mis-attributed to `"CRB"`,
    /// whose own directory does not contain them) were never dumped to
    /// `data/corpus/` under either book.
    #[test]
    fn book_routing_b1_ships_under_bestiary_but_reads_core_essentials() {
        let (book_id, book_dir) = book_routing("B1").expect("B1 must be routed");
        assert_eq!(book_id, "bestiary");
        assert_eq!(book_dir, "pathfinder/paizo/roleplaying_game/core_essentials");
    }

    #[test]
    fn slugify_dedupes_collisions() {
        let mut used = BTreeSet::new();
        let a = slugify("Cold Iron", &mut used);
        let b = slugify("Cold Iron", &mut used);
        assert_ne!(a, b);
    }

    #[test]
    fn declared_pi_at_line_zero_is_no_declaration() {
        assert!(!declared_pi_at(Path::new("/nonexistent"), 0).any());
    }

    /// Regression test for the real defect this cycle caught by hand
    /// (`OPEN-ISSUES.md`/`progress.md` `SD31-E6-F5-002` receipt):
    /// `write_json` must never overwrite a file that already exists --
    /// a slug collision with an already-shipped, different-key record is
    /// a signal to skip, never to clobber.
    #[test]
    fn write_json_never_overwrites_an_existing_file() {
        let dir = std::env::temp_dir().join(format!("cgeq_test_noclobber_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("widget.json"), "PRE-EXISTING REAL DATA").unwrap();

        let record = CacheRecord {
            population: Population::InScope,
            completeness: Completeness::ChassisOnly,
            ingested_at: "2026-08-16T00:00:00Z".to_string(),
            data: EquipmentData {
                key: "Widget".to_string(),
                category: "General".to_string(),
                name: "Widget".to_string(),
                cost_gp: None,
                weight_lbs: None,
                description: None,
            },
            source: Source::LstToken {
                path: "x".to_string(),
                sha256: "x".to_string(),
                line: 1,
                record_key: "Widget".to_string(),
            },
            wiring_class: "display".to_string(),
            wiring_class_signals: vec![],
            license: crate::rules_core::shape_b_v1::License::Ogl,
            pi_field: None,
            pi_marker: None,
        };
        let wrote = write_json(&dir, "widget", &record).unwrap();
        assert!(!wrote, "write_json must report it did NOT write over an existing file");
        let on_disk = std::fs::read_to_string(dir.join("widget.json")).unwrap();
        assert_eq!(on_disk, "PRE-EXISTING REAL DATA", "the pre-existing file must survive untouched");
        std::fs::remove_dir_all(&dir).ok();
    }

    /// The safety-critical fix this module exists to carry: a row whose
    /// corpus line declares `NAMEISPI:YES` must not be written at all, not
    /// redacted-in-place -- proven directly against the real reader with a
    /// synthetic declared-PI row, since the real 704-row population (this
    /// cycle's receipt) happens to carry zero such rows.
    #[test]
    fn a_nameispi_declared_row_would_be_excluded_not_redacted() {
        let tokens = [("NAMEISPI", "YES")];
        let declared = pi_screening::declared_product_identity(tokens);
        assert!(declared.name);
        // EquipmentData.name is a required String, not Option<String> --
        // there is no field-level redaction path for it by construction,
        // which is exactly why `generate()` treats `declared.name` as a
        // whole-record skip (`report.name_pi_excluded`) rather than
        // reaching for a marker this type has nowhere to put.
    }

    /// The blacklist half of the union: a name containing a blacklisted
    /// term is excluded even with no `NAMEISPI:` declaration at all.
    #[test]
    fn a_blacklisted_name_is_flagged_by_the_term_scan() {
        let (license, _, _, _) = pi_screening::classify_field("name", "Iomedae's Blessed Blade");
        assert_eq!(license, crate::rules_core::shape_b_v1::License::PiRedacted);
    }

    #[test]
    fn find_citation_key_then_first_column_then_name() {
        let dir = std::env::temp_dir().join(format!("cgeq_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("book_equipmods.lst");
        std::fs::write(&file, "Widget\tKEY:Special Ability ~ Widget ~ Weapon\tCOST:0\n").unwrap();
        let found = find_citation(&dir, "Special Ability ~ Widget ~ Weapon", "Widget");
        assert_eq!(found, Some((PathBuf::from("book_equipmods.lst"), 1)));
        std::fs::remove_dir_all(&dir).ok();
    }

    /// The shape `core_rulebook/cr_equipmods.lst:665` actually carries:
    /// `Special Ability ~ +1 ~ Weapon.COPY=PLUS1W` -- `PLUS1W` never
    /// appears as a `KEY:` field or a bare first column anywhere, only as
    /// a `.COPY=` variant suffix on another row's first column.
    #[test]
    fn find_citation_falls_back_to_copy_variant() {
        let dir = std::env::temp_dir().join(format!("cgeq_test_copy_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("cr_equipmods.lst");
        std::fs::write(&file, "Special Ability ~ +1 ~ Weapon.COPY=PLUS1W\t\tVISIBLE:NO\n").unwrap();
        let found = find_citation(&dir, "PLUS1W", "PLUS1W");
        assert_eq!(found, Some((PathBuf::from("cr_equipmods.lst"), 1)));
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn find_citation_returns_none_when_nothing_matches() {
        let dir = std::env::temp_dir().join(format!("cgeq_test_none_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("book_equipmods.lst");
        std::fs::write(&file, "SomethingElse\tCOST:0\n").unwrap();
        assert_eq!(find_citation(&dir, "NoSuchKey", "NoSuchName"), None);
        std::fs::remove_dir_all(&dir).ok();
    }

    /// `OPEN-ISSUES.md` row 90's confirmed defect, reproduced from the real
    /// oracle bytes: `uc_profs_weapon.lst:188` carries a bare weapon-
    /// proficiency listing row whose `KEY:` field happens to equal the
    /// equipment record's own identity string (`"Catapult (Standard)"`),
    /// with no `COST:`/`WT:`/`SPROP:` payload at all -- the real item row,
    /// with the real `COST:800`, lives at `uc_equip_arms_armor.lst:168`
    /// as a first-column (not `KEY:`-field) match. The OLD search order
    /// (`find_by_key_field` across every file, THEN `find_exact_first_
    /// column` across every file) matched the proficiency file's `KEY:`
    /// field before ever trying the real equipment file, giving 39 shipped
    /// records a real, correctly-valued payload under a wrong citation and
    /// wrong `raw_tokens`. The fix must try every strategy against
    /// equipment-shaped files (`is_equipment_shaped_file`) FIRST, only
    /// falling back to non-equipment-shaped files if nothing there
    /// resolves -- proven here by naming the file that must win.
    #[test]
    fn find_citation_prefers_an_equipment_shaped_file_over_a_proficiency_file_with_a_coincidental_key_match() {
        let dir = std::env::temp_dir().join(format!("cgeq_test_row90_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        // Sorted before the equip file (`p` < `u`... no: alphabetically
        // "uc_equip_arms_armor.lst" < "uc_profs_weapon.lst" already, so this
        // reproduces the defect purely via the OLD strategy-major loop
        // order, not file sort order -- the fix must not depend on sort
        // order happening to favor the right file.
        std::fs::write(
            dir.join("uc_profs_weapon.lst"),
            "Catapult\tKEY:Catapult (Standard)\tTYPE:Exotic.Ranged.SiegeEngine.Siege.SiegeWeapon.Bludgeoning\n",
        )
        .unwrap();
        std::fs::write(
            dir.join("uc_equip_arms_armor.lst"),
            "Catapult (Standard)\tPROFICIENCY:WEAPON|Catapult (Standard)\tTYPE:Weapon.Ranged.SiegeWeapon.Exotic.Bludgeoning\tCOST:800\tCRITMULT:x2\tDAMAGE:6d6\tRANGE:200\n",
        )
        .unwrap();
        let found = find_citation(&dir, "Catapult (Standard)", "Catapult (Standard)");
        assert_eq!(found, Some((PathBuf::from("uc_equip_arms_armor.lst"), 1)));
        std::fs::remove_dir_all(&dir).ok();
    }

    /// A book whose real content genuinely has no equipment-shaped file at
    /// all must still resolve via the non-equipment-shaped fallback tier --
    /// the fix narrows the search ORDER, it does not remove the fallback.
    #[test]
    fn find_citation_falls_back_to_a_non_equipment_shaped_file_when_no_equipment_shaped_file_resolves() {
        let dir = std::env::temp_dir().join(format!("cgeq_test_fallback_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("book_profs_weapon.lst"), "Widget\tKEY:Widget\tCOST:5\n").unwrap();
        let found = find_citation(&dir, "Widget", "Widget");
        assert_eq!(found, Some((PathBuf::from("book_profs_weapon.lst"), 1)));
        std::fs::remove_dir_all(&dir).ok();
    }

    /// The real defect traced in `advanced_class_guide/acg_equipmods.lst`
    /// (cycle receipt `artifacts/gate-3-closure-invariant/
    /// t9-onboarding-equipment-ue-gap-routing_cycle-1_cycle_receipt.md`):
    /// a base template row's DISPLAY name (first column) coincidentally
    /// equals the short key a *different* row's `.COPY=<key>` mints, and
    /// the old strategy order (first-column before `.COPY=`) let the
    /// template row win, citing the wrong line. `.COPY=<key>` must win
    /// over the coincidental first-column match on the same identifier.
    /// `existing_source_line` regression: `None` for an absent file, `None`
    /// for a file with no readable `source.line` (never fabricates a line
    /// out of malformed/foreign JSON), and `Some(real line)` for a real
    /// shipped-shaped record.
    #[test]
    fn existing_source_line_reads_a_real_record_and_is_none_otherwise() {
        let dir = std::env::temp_dir().join(format!("cgeq_existing_line_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        assert_eq!(existing_source_line(&dir, "absent"), None);

        std::fs::write(dir.join("not_json.json"), "not json at all").unwrap();
        assert_eq!(existing_source_line(&dir, "not_json"), None);

        std::fs::write(
            dir.join("real.json"),
            r#"{"source":{"kind":"lst_token","path":"x.lst","sha256":"a","line":446,"record_key":"x"}}"#,
        )
        .unwrap();
        assert_eq!(existing_source_line(&dir, "real"), Some(446));
        std::fs::remove_dir_all(&dir).ok();
    }

    /// RED->GREEN regression for the real defect `write_json`'s own doc
    /// comment names: `core_rulebook`'s `.COPY=`-named row "Intelligent
    /// Item Purpose (Slay All)" (citation line 895) slugifies to the exact
    /// same filename as the ALREADY-SHIPPED base declaration "Intelligent
    /// Item ~ Purpose / Slay All" (citation line 446) -- two real corpus
    /// rows, one slug. Before this cycle's `existing_source_line` check,
    /// `generate()`'s loop called `write_json` once, saw `path.exists()`,
    /// and silently dropped the second row forever (never fabricated,
    /// but also never written -- a real `no_record` unit that no config
    /// change alone could close). This test proves the disambiguation
    /// decision in isolation, without a full `generate()` run: a
    /// DIFFERENT line must trigger a second `slugify` call (which the
    /// `used`-set mechanism already disambiguates via `-2`); the SAME line
    /// must not (an ordinary idempotent rerun stays a single file).
    #[test]
    fn a_different_citation_line_at_an_occupied_slug_is_disambiguated_not_dropped() {
        let dir = std::env::temp_dir().join(format!("cgeq_collision_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("intelligent_item_purpose_slay_all.json"),
            r#"{"source":{"kind":"lst_token","path":"x.lst","sha256":"a","line":446,"record_key":"Intelligent Item ~ Purpose / Slay All"}}"#,
        )
        .unwrap();

        // RED (the defect, reproduced): naive re-slugify without checking
        // the existing file's own line would just collide with itself
        // again on the FIRST call, since `used` starts empty for this
        // invocation -- there is nothing in `used` yet to disambiguate
        // against, which is exactly why the fix needs `existing_source_line`
        // rather than relying on `slugify`'s in-run set alone.
        let mut used_naive: BTreeSet<String> = BTreeSet::new();
        let naive_slug = slugify("Intelligent Item Purpose (Slay All)", &mut used_naive);
        assert_eq!(
            naive_slug, "intelligent_item_purpose_slay_all",
            "sanity: the two real records really do collide on one slug"
        );

        // GREEN (the fix): a genuinely different line disambiguates.
        let mut used: BTreeSet<String> = BTreeSet::new();
        let mut slug = slugify("Intelligent Item Purpose (Slay All)", &mut used);
        let new_line: u32 = 895;
        if let Some(existing_line) = existing_source_line(&dir, &slug) {
            assert_eq!(existing_line, 446);
            if existing_line != new_line {
                slug = slugify("Intelligent Item Purpose (Slay All)", &mut used);
            }
        } else {
            panic!("existing_source_line must read the fixture's real line");
        }
        assert_eq!(slug, "intelligent_item_purpose_slay_all-2");

        // Idempotency preserved: a SAME-line rerun must not disambiguate.
        let mut used2: BTreeSet<String> = BTreeSet::new();
        let mut slug2 = slugify("Intelligent Item ~ Purpose / Slay All", &mut used2);
        let rerun_line: u32 = 446;
        if let Some(existing_line) = existing_source_line(&dir, &slug2) {
            if existing_line != rerun_line {
                slug2 = slugify("Intelligent Item ~ Purpose / Slay All", &mut used2);
            }
        }
        assert_eq!(slug2, "intelligent_item_purpose_slay_all");

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn find_citation_prefers_a_copy_variant_over_a_coincidental_first_column_match() {
        let dir = std::env::temp_dir().join(format!("cgeq_test_copy_vs_first_col_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("acg_equipmods.lst"),
            "Answering\tKEY:Special Ability ~ Answering ~ Weapon\tTYPE:Weapon\tPLUS:1\tSOURCEPAGE:p.212\n\
             Special Ability ~ Answering ~ Weapon.COPY=Answering\tVISIBLE:NO\n",
        )
        .unwrap();
        let found = find_citation(&dir, "Answering", "Answering");
        assert_eq!(found, Some((PathBuf::from("acg_equipmods.lst"), 2)));
        std::fs::remove_dir_all(&dir).ok();
    }

    /// Full-population regression for the `try_files` strategy reorder
    /// above (`decisions.md §17`'s "prove it across the full population,
    /// not just the traced case"). Re-resolves EVERY already-shipped
    /// `data/corpus/**/equipment*/**/*.json` record whose `source.kind ==
    /// "lst_token"` (i.e. was originally citation-resolved by this exact
    /// `find_citation`, via `equipment_gap::generate` or
    /// `hand_authored_equipment::generate`) using its own stored
    /// `data.key`/`data.name`, and asserts the citation is UNCHANGED from
    /// what shipped -- proving the reorder is additive for the coincidental-
    /// collision shape and does not silently re-point any other already-
    /// correct citation. Requires a real PCGen oracle checkout
    /// (`PCGEN_CORPUS_ROOT`); `#[ignore]`d by default like this crate's
    /// other oracle-backed audits (`sd24_equipment_coverage_audit`, etc.),
    /// run explicitly:
    /// `PCGEN_CORPUS_ROOT=... cargo test --locked --lib
    /// equipment_gap::tests::find_citation_full_population_regression -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn find_citation_full_population_regression() {
        let corpus_root = PathBuf::from(
            std::env::var("PCGEN_CORPUS_ROOT").expect("PCGEN_CORPUS_ROOT must be set for this audit"),
        );
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let corpus_out = PathBuf::from(manifest_dir).join("data/corpus");

        let mut book_dirs: HashMap<&'static str, PathBuf> = HashMap::new();
        // `book_rel_dir` (e.g. `"pathfinder/paizo/roleplaying_game/ultimate_intrigue"`)
        // is the prefix BOTH `equipment_gap::generate` and
        // `hand_authored_equipment::generate` stamp onto `source.path`
        // (`format!("{book_rel_dir}/{rel_path_str}")`) -- `find_citation`
        // itself returns a path relative to `book_dir` only, so this
        // regression must re-apply the same prefix before comparing, or
        // every already-correct citation reads as a false "mismatch".
        let mut book_rel_dirs: HashMap<&'static str, &'static str> = HashMap::new();
        for code in [
            "CRB", "B1", "APG", "ACG", "ARG", "UC", "UI", "UM", "UPSI", "UW", "ISG", "OA", "HA",
            "MYTHIC", "ISC", "ISR", "ISWG", "MC", "ISI", "B2", "B3", "B4", "BOTD2", "ISTEM", "ISM",
            "AG", "UE",
        ] {
            if let Some((book_id, book_rel_dir)) = book_routing(code) {
                book_dirs.insert(book_id, corpus_root.join(book_rel_dir));
                book_rel_dirs.insert(book_id, book_rel_dir);
            }
        }

        fn find_json_files(dir: &Path, out: &mut Vec<PathBuf>) {
            let Ok(entries) = std::fs::read_dir(dir) else { return };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    find_json_files(&path, out);
                } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
                    out.push(path);
                }
            }
        }

        let mut checked = 0usize;
        let mut mismatches: Vec<String> = Vec::new();

        for entry in std::fs::read_dir(&corpus_out).expect("data/corpus must exist") {
            let entry = entry.expect("readable dir entry");
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let book_id = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
            let equip_dir = path.join("equipment");
            if !equip_dir.is_dir() {
                continue;
            }
            let Some(book_dir) = book_dirs.get(book_id.as_str()) else { continue };
            if !book_dir.is_dir() {
                continue;
            }
            let book_rel_dir = book_rel_dirs.get(book_id.as_str()).copied().unwrap_or("");
            let mut files = Vec::new();
            find_json_files(&equip_dir, &mut files);

            for file in files {
                let Ok(text) = std::fs::read_to_string(&file) else { continue };
                let Ok(json): Result<serde_json::Value, _> = serde_json::from_str(&text) else { continue };
                let Some(source) = json.get("source") else { continue };
                if source.get("kind").and_then(|v| v.as_str()) != Some("lst_token") {
                    continue;
                }
                let Some(data) = json.get("data") else { continue };
                let Some(key) = data.get("key").and_then(|v| v.as_str()) else { continue };
                let Some(name) = data.get("name").and_then(|v| v.as_str()) else { continue };
                let Some(src_path) = source.get("path").and_then(|v| v.as_str()) else { continue };
                let Some(src_line) = source.get("line").and_then(|v| v.as_u64()) else { continue };

                checked += 1;
                match find_citation(book_dir, key, name) {
                    Some((resolved_path, resolved_line)) => {
                        let resolved_rel = resolved_path.to_string_lossy().replace('\\', "/");
                        let resolved_path_str = format!("{book_rel_dir}/{resolved_rel}");
                        if resolved_path_str != src_path || u64::from(resolved_line) != src_line {
                            mismatches.push(format!(
                                "{}: key={key:?} name={name:?} shipped=({src_path}:{src_line}) now=({resolved_path_str}:{resolved_line})",
                                file.display()
                            ));
                        }
                    }
                    None => {
                        mismatches.push(format!(
                            "{}: key={key:?} name={name:?} shipped=({src_path}:{src_line}) now=UNRESOLVED",
                            file.display()
                        ));
                    }
                }
            }
        }

        println!("find_citation_full_population_regression: checked={checked} mismatches={}", mismatches.len());
        for m in &mismatches {
            println!("MISMATCH {m}");
        }
        assert!(checked > 0, "no lst_token-sourced equipment records found -- corpus_root or corpus_out is wrong");
        assert!(
            mismatches.is_empty(),
            "{} citation(s) changed by the strategy reorder: {:#?}",
            mismatches.len(),
            mismatches
        );
    }
}
