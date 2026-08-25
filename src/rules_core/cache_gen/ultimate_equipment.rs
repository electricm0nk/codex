//! Ultimate Equipment (UE) JSON cache generator (SD-31 `epic-6-ingest-lanes`
//! F5/F6, `SD31-E6-F5-001`).
//!
//! Writes `data/corpus/ultimate_equipment/equipment/*.json` by DUMPING the
//! current, already-completed state of
//! `rules_core::rules_tables::ultimate_equipment::equipment_tables`
//! (`equipment_tables()` + `equipmod_tables()`) -- per `decisions.md
//! §11.3`, this generator never re-parses raw PCGen LST to derive a
//! field's *value*; every value written here is read straight from the
//! compiled Rust module, exactly as `equipment_resolver::equipment_
//! catalog_rows()` already reads it via the same public accessors (that
//! table already drives the desktop equipment catalog and
//! `docs/work-inventory.json`'s `equipment`/`equipment_modifier` `book:
//! "ultimate_equipment"` rows -- this generator closes the *other* half:
//! the on-disk `data/corpus/` cache the `equipment`-effect wiring probe
//! and `corpus_literal_sweep` both need but this book has never had).
//!
//! Mirrors `cache_gen::acg`'s shape closely (own local Shape B types, own
//! citation helpers -- no shared types file, per `decisions.md §11.3` and
//! `loop-instruction.md`'s disjoint-file-touch convention) with two
//! differences UE's own corpus shape requires:
//!
//! 1. **Four category files**, not one: `ue_equip_general.lst` /
//!    `ue_equip_arms_armor.lst` / `ue_equip_magic_items.lst` (General /
//!    ArmsArmor / MagicItems) plus `ue_equipmods.lst` (Equipmods) --
//!    matching APG's per-category-file split, not ACG's single merged file.
//! 2. **`.COPY=` variant citations.** `equipment_tables.rs`'s own doc
//!    comment: 92 of the 1,424 declared equipment rows are `.COPY=` rows
//!    naming a genuinely distinct new item (a masterwork/size variant).
//!    Those resolve via [`find_copy_variant`] (a line whose first column
//!    ends `.COPY=<record_name>`), tried after the direct first-column/
//!    `KEY:` lookups fail -- the same fallback `cache_gen::apg::
//!    resolve_citation` already established.
//!
//! `equipmod_tables()`'s `Equipmods`-category rows are written into the
//! *same* `equipment/` directory as `equipment_tables()`'s rows (matching
//! `cache_gen::acg`'s precedent): `docs/work-inventory.json`'s
//! `equipment_modifier` kind is derived from `data.category ==
//! "Equipmods"`, not from a separate top-level directory
//! (`v06_work_inventory.rs`'s `Kind::EquipmentModifier` classification).
//!
//! ## PI screening -- both SD-30 invocation contracts (`decisions.md
//! §52.3`/`§53.5`, cited by `SD-31 kanban.md`'s cross-gate note)
//!
//! Every record's `description` runs through `pi_screening::
//! classify_optional_field_declared`, which is the union of both
//! contracts in one call: the row's own `NAMEISPI:`/`DESCISPI:` corpus
//! declaration (read off the resolved citation line by
//! [`declared_pi_at`], §53.5's declared-PI reader) takes precedence when
//! present, and the shared 55-term blacklist (`pi_screening::
//! PI_BLACKLIST_TERMS`, §52.3's blacklist sweep) still runs over
//! everything else regardless -- "the two screens are a union, never a
//! substitution" (`pi_screening.rs`'s own doc comment).

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Serialize;

use crate::rules_core::cache_gen::WiringClassIndex;
use crate::rules_core::cache_gen::equipment_gap::{RenameInfo, resolve_name_or_rename};
use crate::rules_core::pi_screening::{self, DeclaredProductIdentity};
use crate::rules_core::rules_tables::ultimate_equipment as ue;
use crate::rules_core::rules_tables::ultimate_equipment::equipment_tables::EquipmentCategory;

/// `wiring_class`'s corpus-wide book id for Ultimate Equipment.
const WIRING_CLASS_BOOK_ID: &str = "ultimate_equipment";

// ---------------------------------------------------------------------
// Shape B schema (decisions.md §7, corrected §11.1/§11.2) -- mirrors
// cache_gen::acg's own local, self-contained definition.
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
    LstToken {
        path: String,
        sha256: String,
        line: u32,
        record_key: String,
    },
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
    /// SD-32 T9 onboarding (card 11) group E: `decisions.md §24b`-3, ported
    /// from `cache_gen::equipment_gap`'s identical field (this file
    /// predates `§24`'s neutral-rename mechanism and previously dropped a
    /// `NAMEISPI:YES` row outright -- see `resolve_name_or_rename`'s call
    /// site in `generate_equipment` below for the worked "Otyugh Hide"
    /// example). Defaults to `false` via `#[serde(default)]` on read, so
    /// this is additive to every already-shipped record's shape.
    #[serde(default)]
    pub codex_generated_name: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rename: Option<RenameInfo>,
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
// Corpus-access helpers (citation lookup only, never value derivation)
// ---------------------------------------------------------------------

const UE_DIR: &str = "pathfinder/paizo/roleplaying_game/ultimate_equipment";

fn book_dir(corpus_root: &Path) -> PathBuf {
    corpus_root.join(UE_DIR)
}

/// Real sha256 of `path`'s current on-disk content (mirrors
/// `cache_gen::acg::sha256_file`).
pub fn sha256_file(path: &Path) -> std::io::Result<String> {
    let output = Command::new("sha256sum").arg(path).output()?;
    if !output.status.success() {
        return Err(std::io::Error::other(format!("sha256sum failed for {}", path.display())));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    Ok(text.split_whitespace().next().unwrap_or_default().to_string())
}

fn equipment_category_file(category: EquipmentCategory) -> &'static str {
    match category {
        EquipmentCategory::General => "ue_equip_general.lst",
        EquipmentCategory::ArmsArmor => "ue_equip_arms_armor.lst",
        EquipmentCategory::MagicItems => "ue_equip_magic_items.lst",
        EquipmentCategory::Equipmods => "ue_equipmods.lst",
    }
}

/// Finds `record_name` as an exact match on a line's first tab-delimited
/// column in `lst_path`.
fn find_exact_first_column(lst_path: &Path, record_name: &str) -> std::io::Result<Option<u32>> {
    let content = std::fs::read_to_string(lst_path)?;
    for (idx, line) in content.lines().enumerate() {
        let first_col = line.split('\t').next().unwrap_or("");
        if first_col == record_name {
            return Ok(Some((idx + 1) as u32));
        }
    }
    Ok(None)
}

/// Finds a line carrying the exact tab-delimited field `KEY:<record_key>`
/// in `lst_path` -- required for `ue_equipmods.lst`'s `~`-qualified keys
/// (e.g. `Material ~ Bone`), the same disambiguation
/// `cache_gen::acg::find_by_key_field` established for ACG's own
/// equipmods.
fn find_by_key_field(lst_path: &Path, record_key: &str) -> std::io::Result<Option<u32>> {
    let content = std::fs::read_to_string(lst_path)?;
    let needle = format!("KEY:{record_key}");
    for (idx, line) in content.lines().enumerate() {
        if line.split('\t').any(|field| field == needle) {
            return Ok(Some((idx + 1) as u32));
        }
    }
    Ok(None)
}

/// Finds a `.COPY=<record_name>` variant line's first column in
/// `lst_path` (see module doc comment's 92-row `.COPY=` note).
fn find_copy_variant(lst_path: &Path, record_name: &str) -> std::io::Result<Option<u32>> {
    let content = std::fs::read_to_string(lst_path)?;
    let needle = format!(".COPY={record_name}");
    for (idx, line) in content.lines().enumerate() {
        let first_col = line.split('\t').next().unwrap_or("");
        if first_col.ends_with(&needle) {
            return Ok(Some((idx + 1) as u32));
        }
    }
    Ok(None)
}

/// Reads [`DeclaredProductIdentity`] off the real corpus line at
/// `lst_path:line` (1-indexed) -- `§53.5`'s declared-PI reader, applied
/// per-record against the same citation this generator already resolved
/// for `source`. `line == 0` (an unresolved citation) reads as no
/// declaration, matching every other generator's honest-gap handling.
fn declared_pi_at(lst_path: &Path, line: u32) -> std::io::Result<DeclaredProductIdentity> {
    if line == 0 {
        return Ok(DeclaredProductIdentity::default());
    }
    let content = std::fs::read_to_string(lst_path)?;
    let Some(row) = content.lines().nth((line - 1) as usize) else {
        return Ok(DeclaredProductIdentity::default());
    };
    let tokens: Vec<(&str, &str)> = row
        .split('\t')
        .filter_map(|field| field.split_once(':'))
        .collect();
    Ok(pi_screening::declared_product_identity(tokens))
}

/// Resolves a real citation for `entry`, trying (in order): the tab-
/// delimited `KEY:<entry.key>` field, the exact first-column match
/// (`entry.key == entry.name`, the common case), then a
/// `.COPY=<entry.key>` variant line. Returns `(line, category_file)`;
/// `line == 0` means unresolved (pushed onto `unresolved` by the caller).
///
/// **`KEY:` is tried first for every category, not only `Equipmods`.**
/// `Equipmods` rows need it for `find_by_key_field`'s own documented
/// reason (a repeated display name across distinct `KEY:` targets), but
/// re-deriving the 36 initially-unresolved General/ArmsArmor/MagicItems
/// `.COPY=` rows (`Artisan's Tools (Masterwork)`, `Arrow (Hushing)`,
/// `Dagger (Bloodthirst)`, ...) against the real corpus found a second,
/// distinct reason the same lookup order applies everywhere: each of
/// those rows carries an explicit `KEY:<entry.key>` token on its
/// `.COPY=` line that OVERRIDES the `.COPY=<display name>` suffix (e.g.
/// `Artisan's Tools.COPY=Artisan's Tools, Masterwork␉KEY:Artisan's Tools
/// (Masterwork)` -- the corpus's own real identity is the `KEY:` value,
/// not the comma-form display name the `.COPY=` suffix carries). A
/// first-column/`.COPY=`-suffix-only lookup order silently missed all 36;
/// `KEY:`-first finds every one of them the same way it already found
/// `Equipmods`.
/// `true` when a record's identity is Product Identity -- the union of
/// the row's own `NAMEISPI:YES` corpus declaration (`declared_name`) and
/// the shared blacklist term scan against `name` (strong, word-bounded,
/// case-folded, OCR-normalized, concatenated-identifier: the SAME scan
/// `cache_gen::{acg,apg,beastiary1,equipment_gap}` already union into
/// their own `name`/`key` screen, `t9-onboarding-pi-final-leaks-and-
/// generators` cycle). Factored out so this exact predicate is directly
/// unit-testable without needing a real corpus row from the compiled,
/// non-injectable `equipment_tables()` static table.
///
/// t9-onboarding-pi-last-leak-and-generators cycle: before this fix, this
/// generator's ONLY name-PI signal was `declared_name` -- there was no
/// blacklist term scan of `name`/`key` at all, so a future
/// `PI_BLACKLIST_TERMS` amendment could make an existing curated table
/// entry newly PI with nothing here ever re-screening it (the seventh
/// instance of "screens some shipped fields, not all" in this bundle).
fn name_or_key_is_pi(declared_name: bool, name: &str) -> bool {
    declared_name || pi_screening::blacklist_term_hit_including_concatenated(name).is_some()
}

fn resolve_line(corpus_root: &Path, entry: &ue::equipment_tables::EquipmentTableEntry) -> (u32, &'static str) {
    let category_file = equipment_category_file(entry.category);
    let path = book_dir(corpus_root).join(category_file);

    if let Ok(Some(line)) = find_by_key_field(&path, entry.key) {
        return (line, category_file);
    }
    if let Ok(Some(line)) = find_exact_first_column(&path, entry.key) {
        return (line, category_file);
    }
    if let Ok(Some(line)) = find_copy_variant(&path, entry.key) {
        return (line, category_file);
    }
    (0, category_file)
}

// ---------------------------------------------------------------------
// Generation report
// ---------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct GenerationReport {
    pub equipment_written: usize,
    pub equipment_modifier_written: usize,
    /// Record keys whose real LST citation could not be resolved (should
    /// be empty for a clean generation run against the real corpus).
    pub unresolved_citations: Vec<String>,
    /// Records whose real corpus row declares `NAMEISPI:YES`
    /// (`file:line record_key`). `decisions.md §24`: these are no longer
    /// dropped -- they are WRITTEN under a Codex-generated neutral name
    /// (`codex_neutral_name::neutral_name`/`neutral_key`, via
    /// `cache_gen::equipment_gap::resolve_name_or_rename`), coordinate-only.
    /// Field name kept for compatibility with existing callers; it now
    /// counts RENAMES, not silent drops -- every one of these units is
    /// still counted in `equipment_written`/`equipment_modifier_written`
    /// above. Reported, never silent: a row that vanishes without a line
    /// here is indistinguishable from a citation bug.
    pub name_pi_dropped: Vec<String>,
    /// `(kind, book, source_file, source_line, codex_name, reason)`
    /// divergence entries for every unit renamed this run --
    /// `decisions.md §24b`-4: coordinate + reason, never the original
    /// string. Mirrors `cache_gen::equipment_gap::GenerationReport`'s field
    /// of the same name.
    pub name_pi_renamed_records: Vec<serde_json::Value>,
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

/// SD-32 Epic 5 protective sweep -- see `cache_gen::acg::write_json`'s
/// identical doc comment; same shape, same fix. `generate_equipment`'s own
/// stale-key sweep (see its doc comment) is what still makes a genuinely
/// dropped record's file disappear -- this guard only protects a record
/// that is STILL VALID this run from being needlessly re-derived in a
/// narrower, pre-enrichment shape.
fn write_json<T: Serialize>(out_dir: &Path, slug: &str, record: &CacheRecord<T>) -> std::io::Result<()> {
    std::fs::create_dir_all(out_dir)?;
    let path = out_dir.join(format!("{slug}.json"));
    if path.exists() {
        return Ok(());
    }
    let json = serde_json::to_string_pretty(record)
        .expect("CacheRecord<T> is a plain-data shape; serialization cannot fail");
    std::fs::write(path, json)
}

/// Removes every JSON file directly or recursively under `dir` whose
/// `data.key` is NOT in `current_keys` -- the same "genuinely stale, not
/// merely about to be rewritten" rule `gen_book_cache.rs`'s `gen_monster_book`
/// established (`SD31-E6-F9-005`). Replaces `generate_equipment`'s former
/// unconditional `remove_dir_all` of the whole directory: that fix kept a
/// dropped record's file from lingering (OPEN-ISSUES row 38) but did so by
/// erasing every STILL-VALID record too, discarding whatever
/// `enrich_equipment_raw_tokens.rs` had separately written into them. This
/// keeps the "dropped record does not linger" property while leaving every
/// still-valid record's file (and any enrichment already on it) untouched.
///
/// **`owns_citation` -- the cross-generator self-erasure guard
/// (`decisions.md §1a`/incident found live 2026-08-23).** `data.key` alone
/// is NOT a safe ownership test: two different generators can share one
/// output directory (`cache_gen::spell_lane_dump` and
/// `cache_gen::spell_mod_access` both write `data/corpus/<book>/spell/`,
/// and a `.MOD` row genuinely reuses its base spell's own key/name -- see
/// `spell_lane_dump`'s call site). A key-only check therefore reads a
/// sibling generator's still-valid record as "not mine, not current" and
/// deletes it. This caller-supplied predicate answers "did MY OWN parse of
/// MY OWN source file ever produce a citation at this exact
/// `(source.path, source.line)`?" -- a coordinate a sibling generator's
/// rows structurally cannot share, because they cite a *different* LST
/// row even when the file and the key both collide. A record whose
/// `source.path`/`source.line` cannot be read from its JSON is treated as
/// **not owned** (never deleted) -- ownership must be proven, not assumed,
/// the same `§1a` "empty case fails closed" discipline every gate in this
/// bundle already follows.
pub fn remove_stale_owned_files(
    dir: &Path,
    current_keys: &std::collections::HashSet<String>,
    owns_citation: &dyn Fn(&str, u32) -> bool,
) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            remove_stale_owned_files(&path, current_keys, owns_citation);
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let Some(parsed) = std::fs::read_to_string(&path)
            .ok()
            .and_then(|raw| serde_json::from_str::<serde_json::Value>(&raw).ok())
        else {
            continue;
        };
        let Some(key) = parsed.get("data").and_then(|d| d.get("key")).and_then(|k| k.as_str()) else {
            continue;
        };
        if current_keys.contains(key) {
            continue;
        }
        let source_path = parsed.get("source").and_then(|s| s.get("path")).and_then(|p| p.as_str());
        let source_line =
            parsed.get("source").and_then(|s| s.get("line")).and_then(|l| l.as_u64()).map(|l| l as u32);
        let owned = match (source_path, source_line) {
            (Some(p), Some(l)) => owns_citation(p, l),
            _ => false,
        };
        if owned {
            let _ = std::fs::remove_file(&path);
        }
    }
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

fn generate_equipment(
    corpus_root: &Path,
    out_dir: &Path,
    ingested_at: &str,
    report: &mut GenerationReport,
) -> Result<(), GenerationError> {
    let mut sha_by_file = HashMap::new();
    for file in [
        "ue_equip_general.lst",
        "ue_equip_arms_armor.lst",
        "ue_equip_magic_items.lst",
        "ue_equipmods.lst",
    ] {
        let sha = sha256_file(&book_dir(corpus_root).join(file))?;
        sha_by_file.insert(file, sha);
    }
    let mut used = BTreeSet::new();
    let equipment_dir = out_dir.join("equipment");
    // **SD-32 Epic 5 protective sweep, CORRECTED**: this used to wipe the
    // whole directory unconditionally on every run (OPEN-ISSUES row 38's
    // fix, to make a NAMEISPI:YES-dropped record's stale file actually
    // disappear) -- but that also erased every STILL-VALID record's file,
    // discarding whatever `enrich_equipment_raw_tokens.rs` had separately
    // written into it in a later pass this generator's own `EquipmentData`
    // cannot reconstruct (the exact S6/D9 self-erasure shape). The new
    // rule, matching `gen_book_cache.rs`'s `gen_monster_book`
    // (`SD31-E6-F9-005`): a file is removed ONLY when its key is ABSENT
    // from the set this run just computed (`remove_stale_owned_files`
    // below, after the write loop) -- a real drop still disappears, a
    // still-valid record's file is left completely untouched.
    let mut current_keys: std::collections::HashSet<String> = std::collections::HashSet::new();
    // `Equipmods` records go in a nested `equipment/equipmods/` subdirectory,
    // matching `core_rulebook`'s own already-shipped layout
    // (`data/corpus/core_rulebook/equipment/equipmods/*.json`) rather than
    // `cache_gen::acg`'s flat one. Load-bearing, not cosmetic: UE's raw
    // corpus carries a genuine same-bare-name collision between a General
    // item and an Equipmods record (`Masterwork Tool`,
    // `ue_equip_general.lst:277` vs `ue_equipmods.lst:350` -- neither
    // carries a distinguishing `KEY:` token, so there is no non-fabricated
    // way to make their `record_key`s differ) that trips
    // `corpus_traps::audit_ingested_cache`'s `(book, kind_dir, record_key)`
    // uniqueness check when both live in one flat directory
    // (`tests/v06_corpus_trap_report.rs`'s
    // `no_two_ingested_records_share_a_record_key`, `Severity::Defect`).
    // That check does a two-level, non-recursive `read_dir` walk, so
    // CRB's nested equipmods are already structurally invisible to it --
    // the same shape closes UE's real collision without inventing an
    // identity the corpus does not state. `v06_work_inventory`'s
    // `equipment_modifier` kind classification reads `data.category`, not
    // the directory path (confirmed: CRB's own 676 nested equipmods
    // already classify correctly today), so this is a pure directory-
    // layout change with no effect on doneness classification.
    let equipmods_dir = equipment_dir.join("equipmods");
    let wiring_index = WiringClassIndex::build(WIRING_CLASS_BOOK_ID, &book_dir(corpus_root));
    let mut wiring_lines = wiring_index.lines();

    let all_entries = ue::equipment_tables::equipment_tables()
        .iter()
        .chain(ue::equipment_tables::equipmod_tables());

    for entry in all_entries {
        let (line, category_file) = resolve_line(corpus_root, entry);
        if line == 0 {
            report.unresolved_citations.push(format!("equipment:{}", entry.key));
        }
        let (wiring_class, wiring_class_signals) = wiring_index.wiring_class_for(
            &mut wiring_lines,
            category_file,
            line,
            entry.key,
            entry.key,
        );
        let completeness = if entry.description.is_some() {
            Completeness::Full
        } else {
            Completeness::ChassisOnly
        };
        let declared = declared_pi_at(&book_dir(corpus_root).join(category_file), line)
            .unwrap_or_default();
        // SD-32 T9 onboarding (card 11) group E, `decisions.md §24`: a
        // `NAMEISPI:YES` row is no longer dropped outright -- it ingests
        // under a Codex-generated neutral name derived ONLY from
        // `(kind, book, source_file, source_line)`, the same mechanism
        // `cache_gen::equipment_gap::resolve_name_or_rename` already ships
        // for the compiled-book gap lanes, reused here rather than
        // re-implemented (`decisions.md §24b`-1: no argument path from the
        // PI string to the output). Found by adversarial review
        // (OPEN-ISSUES row 38): this branch previously computed
        // `declared.name` and never read it at all, so `Otyugh Hide`
        // (`ue_equip_arms_armor.lst:66`, `NAMEISPI:YES`) shipped its real
        // name unredacted before that fix made it drop the row outright --
        // this cycle replaces the drop with the real §24 rename this
        // generator predates.
        //
        // t9-onboarding-pi-last-leak-and-generators cycle: this file had
        // NO blacklist term scan of `name`/`key` at all -- only the
        // declared `NAMEISPI:YES` reader above. Identical gap and
        // identical fix to `cache_gen::{acg,apg,beastiary1}`'s own
        // `name_or_key_is_pi` (`t9-onboarding-pi-final-leaks-and-generators`
        // cycle): a future `PI_BLACKLIST_TERMS` amendment (this bundle has
        // amended it at least 4 times, `decisions.md §19`) could make an
        // EXISTING curated table entry newly PI with no code ever
        // re-screening it. Unions the strong, word-bounded,
        // OCR-normalized, concatenated-identifier scan into the same
        // `name_is_pi` predicate the rename branch already reads --
        // `declared.name` alone stays the fast path for every ordinary
        // row (zero behaviour change when no blacklist term is present).
        let name_is_pi = name_or_key_is_pi(declared.name, entry.name);
        let is_modifier = entry.category == EquipmentCategory::Equipmods;
        let kind = if is_modifier { "equipment_modifier" } else { "equipment" };
        let (record_name, record_key, codex_generated_name, rename_info, divergence) =
            resolve_name_or_rename(name_is_pi, kind, "ultimate_equipment", category_file, line, entry.name, entry.key);
        if let Some(entry) = divergence {
            report.name_pi_dropped.push(format!("{category_file}:{line} {}", record_key));
            report.name_pi_renamed_records.push(entry);
        }
        let (mut license, mut pi_field, mut pi_marker, stored_desc) = pi_screening::classify_optional_field_declared(
            "description",
            entry.description,
            declared.description,
        );
        // Same supplementary strong-scan re-screen `cache_gen::equipment_
        // gap`'s own "third defect" fix already established (byte-for-byte
        // shape, this cycle): `classify_optional_field_declared` screens
        // `description` via the weak, bare-substring, case-SENSITIVE scan
        // only -- never weakens an existing redaction, only strengthens a
        // miss the weak scan let through.
        let stored_desc = match &stored_desc {
            Some(v) if v.as_str() != crate::rules_core::shape_b_v1::REDACTED_PI_MARKER => {
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
                    stored_desc.clone()
                }
            }
            _ => stored_desc,
        };
        let source = Source::LstToken {
            path: format!("{UE_DIR}/{category_file}"),
            sha256: sha_by_file.get(category_file).cloned().unwrap_or_default(),
            line,
            record_key: record_key.clone(),
        };
        let record = CacheRecord {
            population: Population::InScope,
            completeness,
            ingested_at: ingested_at.to_string(),
            data: EquipmentData {
                key: record_key.clone(),
                category: format!("{:?}", entry.category),
                name: record_name,
                cost_gp: entry.cost_gp,
                weight_lbs: entry.weight_lbs,
                description: stored_desc,
            },
            source,
            wiring_class,
            wiring_class_signals,
            license,
            pi_field,
            pi_marker,
            codex_generated_name,
            rename: rename_info,
        };
        current_keys.insert(record_key.clone());
        let slug = slugify(&record_key, &mut used);
        if entry.category == EquipmentCategory::Equipmods {
            write_json(&equipmods_dir, &slug, &record)?;
            report.equipment_modifier_written += 1;
        } else {
            write_json(&equipment_dir, &slug, &record)?;
            report.equipment_written += 1;
        }
    }
    if equipment_dir.exists() {
        // Only `cache_gen::ultimate_equipment` itself ever writes under
        // `UE_DIR`'s citation prefix, so a citation-path prefix check is a
        // safe (if coarse) ownership predicate here -- no sibling
        // generator shares this directory today.
        remove_stale_owned_files(&equipment_dir, &current_keys, &|path, _line| {
            path.starts_with(UE_DIR)
        });
    }
    Ok(())
}

// ---------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------

/// Generates the full Ultimate Equipment JSON cache under `out_dir`
/// (`data/corpus/ultimate_equipment/`), reading real LST citations from
/// `corpus_root` (a PCGen `data/` checkout). `ingested_at` is stamped at
/// call time by the caller (real wall-clock ISO-8601, never derived --
/// `decisions.md §11.1`).
pub fn generate(
    corpus_root: &Path,
    out_dir: &Path,
    ingested_at: &str,
) -> Result<GenerationReport, GenerationError> {
    if !book_dir(corpus_root).is_dir() {
        return Err(GenerationError::CorpusUnreachable(book_dir(corpus_root)));
    }
    let mut report = GenerationReport::default();
    generate_equipment(corpus_root, out_dir, ingested_at, &mut report)?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::codex_neutral_name::neutral_key;

    #[test]
    fn slugify_handles_parens_and_collisions() {
        let mut used = BTreeSet::new();
        assert_eq!(slugify("Alchemist's Kit", &mut used), "alchemist_s_kit");
        let mut used2 = BTreeSet::new();
        let a = slugify("Gecko (Riding)", &mut used2);
        let b = slugify("Gecko (Riding)", &mut used2);
        assert_ne!(a, b);
    }

    #[test]
    fn declared_pi_at_line_zero_is_no_declaration() {
        let declared = declared_pi_at(Path::new("/nonexistent"), 0).unwrap();
        assert!(!declared.any());
    }

    #[test]
    fn equipment_category_file_covers_every_category() {
        for cat in [
            EquipmentCategory::General,
            EquipmentCategory::ArmsArmor,
            EquipmentCategory::MagicItems,
            EquipmentCategory::Equipmods,
        ] {
            assert!(!equipment_category_file(cat).is_empty());
        }
    }

    // --- OPEN-ISSUES row 38: NAMEISPI:YES must drop the row, not ship it ---

    /// A scratch corpus root carrying real `equipment_tables()` keys under
    /// synthetic `.lst` rows, so `generate()` can run end to end without
    /// touching `$PCGEN_CORPUS_ROOT`. Same pattern as
    /// `wiring_class.rs::ScratchBook`.
    struct ScratchCorpus {
        root: PathBuf,
    }

    impl ScratchCorpus {
        fn new(name: &str) -> Self {
            let root = std::env::temp_dir()
                .join(format!("codex_ue_pi_test_{name}_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&root);
            let ue_dir = root.join(UE_DIR);
            std::fs::create_dir_all(&ue_dir).unwrap();
            // Every category file must exist (generate_equipment sha256s
            // all four up front) even when this test only populates one.
            for file in ["ue_equip_general.lst", "ue_equip_arms_armor.lst", "ue_equip_magic_items.lst", "ue_equipmods.lst"]
            {
                std::fs::write(ue_dir.join(file), "").unwrap();
            }
            ScratchCorpus { root }
        }

        /// Overwrites `ue_equip_arms_armor.lst` with `rows`, one real
        /// tab-delimited PCGen line per row (first column = record key).
        fn write_arms_armor(&self, rows: &[&str]) {
            let path = self.root.join(UE_DIR).join("ue_equip_arms_armor.lst");
            std::fs::write(path, rows.join("\n")).unwrap();
        }
    }

    impl Drop for ScratchCorpus {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    fn out_dir_json_files(out_dir: &Path) -> Vec<PathBuf> {
        let mut files = Vec::new();
        let equipment_dir = out_dir.join("equipment");
        if !equipment_dir.is_dir() {
            return files;
        }
        let mut stack = vec![equipment_dir];
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(&dir).unwrap().flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
                    files.push(path);
                }
            }
        }
        files
    }

    /// `equipment_tables()`'s real `Otyugh Hide` row (key = `ue_equip_
    /// arms_armor.lst:66` in the real corpus): the one entry this
    /// generator knows how to resolve without a full 1,369-row fixture,
    /// because the scratch corpus only needs to satisfy citation lookup
    /// for the ONE key under test -- every other table entry is simply
    /// left unresolved (`unresolved_citations`, not asserted empty here).
    fn otyugh_hide_row(nameispi: bool) -> String {
        if nameispi {
            "Otyugh Hide\tNAMEISPI:YES\tPROFICIENCY:ARMOR|Hide\tCOST:1415\tWT:25".to_string()
        } else {
            "Otyugh Hide\tPROFICIENCY:ARMOR|Hide\tCOST:1415\tWT:25".to_string()
        }
    }

    /// SD-32 T9 onboarding (card 11) group E: `decisions.md §24` replaced
    /// this generator's original "drop the row outright" disposition with a
    /// Codex-generated neutral rename, the same mechanism
    /// `cache_gen::equipment_gap` already ships (`resolve_name_or_rename`,
    /// reused here rather than re-implemented). The record now SHIPS --
    /// under a coordinate-derived neutral name -- rather than vanishing;
    /// only the real "Otyugh Hide" string must never appear anywhere in the
    /// output, which this test still asserts.
    #[test]
    fn nameispi_yes_renames_the_record_instead_of_dropping_it() {
        let corpus = ScratchCorpus::new("renames");
        corpus.write_arms_armor(&[&otyugh_hide_row(true)]);
        let out_dir = corpus.root.join("out");

        let report = generate(&corpus.root, &out_dir, "2026-01-01T00:00:00Z").unwrap();

        let neutral_key = neutral_key("equipment", "ultimate_equipment", "ue_equip_arms_armor.lst", 1);
        assert_eq!(
            report.name_pi_dropped,
            vec![format!("ue_equip_arms_armor.lst:1 {neutral_key}")],
            "a NAMEISPI:YES row must be reported as renamed, by coordinate, never the original string"
        );
        assert_eq!(report.name_pi_renamed_records.len(), 1);
        let mut found_renamed_record = false;
        for path in out_dir_json_files(&out_dir) {
            let text = std::fs::read_to_string(&path).unwrap();
            assert!(
                !text.contains("Otyugh Hide"),
                "{path:?} must not carry the real name of a NAMEISPI:YES record: {text}"
            );
            if text.contains(&neutral_key) {
                found_renamed_record = true;
                assert!(text.contains("\"codex_generated_name\": true"));
            }
        }
        assert!(found_renamed_record, "the renamed record must still be written to disk");
    }

    /// t9-onboarding-pi-last-leak-and-generators cycle: this file's
    /// `description` screen (`pi_screening::classify_optional_field_
    /// declared`, via `classify_field`) is the SAME weak, case-SENSITIVE,
    /// bare-substring scan `cache_gen::equipment_gap` was fixed for --
    /// proves the disagreement the supplementary strong re-screen closes,
    /// mirroring `equipment_gap`'s own identical regression test
    /// byte-for-byte (referencing the term by index, `§24b`-2).
    #[test]
    fn the_weak_description_scan_misses_a_lowercase_term_the_strong_scan_catches() {
        let term = pi_screening::PI_BLACKLIST_TERMS[9];
        let lowercase_variant = term.to_lowercase();
        let text = format!("carvings of {lowercase_variant} in one or both aspects");
        let (weak_license, ..) = pi_screening::classify_field("description", &text);
        assert_eq!(
            weak_license,
            crate::rules_core::shape_b_v1::License::Ogl,
            "sanity: the weak scan must miss the lowercase form"
        );
        assert!(
            pi_screening::blacklist_term_hit_including_concatenated(&text).is_some(),
            "the strong scan this generator's own supplementary re-screen now uses must catch it"
        );
    }

    // --- t9-onboarding-pi-last-leak-and-generators: `name`/`key` blacklist
    // scan, independent of `NAMEISPI:YES` ---

    #[test]
    fn name_or_key_is_pi_is_true_when_declared() {
        assert!(name_or_key_is_pi(true, "Perfectly Ordinary Widget"));
    }

    #[test]
    fn name_or_key_is_pi_is_false_for_an_ordinary_undeclared_name() {
        assert!(!name_or_key_is_pi(false, "Perfectly Ordinary Widget"));
    }

    /// The gap this cycle closes: a name carrying a blacklisted term with
    /// NO `NAMEISPI:` declaration at all must still be flagged -- mirrors
    /// `cache_gen::equipment_gap`'s `a_blacklisted_name_is_flagged_by_the_
    /// term_scan`, referencing the term by index (`§24b`-2: never write a
    /// blacklist term literally into a test).
    #[test]
    fn name_or_key_is_pi_is_true_for_an_undeclared_blacklisted_term() {
        let term = pi_screening::PI_BLACKLIST_TERMS[9];
        let name = format!("{term}'s Blessed Blade");
        assert!(name_or_key_is_pi(false, &name));
    }

    #[test]
    fn without_the_declaration_the_same_row_ships_normally() {
        let corpus = ScratchCorpus::new("ships");
        corpus.write_arms_armor(&[&otyugh_hide_row(false)]);
        let out_dir = corpus.root.join("out");

        let report = generate(&corpus.root, &out_dir, "2026-01-01T00:00:00Z").unwrap();

        assert!(report.name_pi_dropped.is_empty());
        let files = out_dir_json_files(&out_dir);
        let hit = files.iter().find(|p| {
            std::fs::read_to_string(p).unwrap().contains("\"name\": \"Otyugh Hide\"")
        });
        assert!(hit.is_some(), "an undeclared row must still publish its real name");
    }

    #[test]
    fn a_dropped_record_does_not_linger_from_a_prior_run() {
        // Run 1: the row is clean and ships.
        let corpus = ScratchCorpus::new("linger");
        corpus.write_arms_armor(&[&otyugh_hide_row(false)]);
        let out_dir = corpus.root.join("out");
        generate(&corpus.root, &out_dir, "2026-01-01T00:00:00Z").unwrap();
        assert!(out_dir_json_files(&out_dir).iter().any(|p| {
            std::fs::read_to_string(p).unwrap().contains("Otyugh Hide")
        }));

        // Run 2: the corpus is corrected to declare NAMEISPI:YES (e.g. an
        // oracle bump). The stale file from run 1 must not survive.
        corpus.write_arms_armor(&[&otyugh_hide_row(true)]);
        generate(&corpus.root, &out_dir, "2026-01-01T00:00:00Z").unwrap();
        for path in out_dir_json_files(&out_dir) {
            let text = std::fs::read_to_string(&path).unwrap();
            assert!(!text.contains("Otyugh Hide"), "{path:?} is a stale pre-drop file: {text}");
        }
    }

    /// SD-32 Epic 5 protective sweep (`epic-breakdown.md` Epic 5, T3
    /// residual): `generate_equipment` wiped `equipment_dir` wholesale on
    /// every run (the "OPEN-ISSUES row 38" fix above), then rewrote every
    /// entry from scratch -- the S6/D9 self-erasure shape `gen_book_cache.rs`'s
    /// `gen_monster_book` was already fixed for (`SD31-E6-F9-005`), never
    /// extended here. `enrich_equipment_raw_tokens.rs` writes a `raw_tokens`
    /// field onto this generator's own output AFTER it runs (1,368 of the
    /// 1,548 on-disk `ultimate_equipment` equipment records carry it today,
    /// `grep -l raw_tokens data/corpus/ultimate_equipment/equipment/*.json`);
    /// a bare re-run would silently strip every one of them. Proves a
    /// second `generate()` call leaves an already-enriched, still-valid
    /// record's file completely alone.
    #[test]
    fn a_second_run_does_not_erase_a_later_enrichment_pass_on_a_still_valid_record() {
        let corpus = ScratchCorpus::new("no_self_erasure");
        corpus.write_arms_armor(&[&otyugh_hide_row(false)]);
        let out_dir = corpus.root.join("out");

        generate(&corpus.root, &out_dir, "2026-01-01T00:00:00Z").unwrap();
        let written = out_dir_json_files(&out_dir)
            .into_iter()
            .find(|p| std::fs::read_to_string(p).unwrap().contains("Otyugh Hide"))
            .expect("run 1 must write the Otyugh Hide record");

        // Simulate `enrich_equipment_raw_tokens.rs` running after generation
        // and adding a field this generator's own `EquipmentData` cannot
        // reconstruct.
        let enriched = std::fs::read_to_string(&written).unwrap().replace(
            "\"description\": null",
            "\"description\": null,\n  \"raw_tokens\": [\"ENRICHED-MARKER\"]",
        );
        assert!(enriched.contains("ENRICHED-MARKER"), "the fixture edit must actually take");
        std::fs::write(&written, &enriched).unwrap();

        // Run 2: the same row, unchanged, still resolves -- the record is
        // still valid, not dropped.
        generate(&corpus.root, &out_dir, "2026-01-02T00:00:00Z").unwrap();
        let after = std::fs::read_to_string(&written).unwrap();
        assert!(
            after.contains("ENRICHED-MARKER"),
            "a second run must not erase a later enrichment pass on a still-valid record: {after}"
        );
    }

    // --- Cross-generator self-erasure guard (2026-08-23 incident) ---
    //
    // `cache_gen::spell_lane_dump` and `cache_gen::spell_mod_access` both
    // write `data/corpus/<book>/spell/` from the SAME literal `.lst` file,
    // and a `.MOD` row genuinely reuses its base spell's own key/name. A
    // `remove_stale_owned_files` predicate that only checks `data.key` (as
    // this function did before this fix) therefore deletes a sibling
    // generator's still-valid records the moment its keys are absent from
    // the caller's own `current_keys` set -- confirmed live: an unscoped
    // run deleted 1,580 real `spell_mod_access` `.MOD` records this way.
    // These tests exercise `remove_stale_owned_files` directly (the
    // reusable guard both generators call), independent of either
    // generator's own book-parsing pipeline.

    /// Writes a minimal on-disk record whose `data.key`/`source.path`/
    /// `source.line` match what either generator's real `CacheRecord`
    /// shape serializes -- deliberately NOT importing `spell_lane_dump` or
    /// `spell_mod_access`'s own record types, so this test proves the
    /// guard's *on-disk JSON contract*, the same contract a third future
    /// sibling generator would also have to satisfy.
    fn write_stub_record(dir: &Path, slug: &str, key: &str, source_path: &str, source_line: u32) {
        std::fs::create_dir_all(dir).unwrap();
        let json = serde_json::json!({
            "data": {"key": key},
            "source": {"path": source_path, "line": source_line},
        });
        std::fs::write(dir.join(format!("{slug}.json")), json.to_string()).unwrap();
    }

    /// The exact incident shape: a sibling generator's record shares this
    /// generator's key ("ablative barrier" is both the base spell name
    /// AND the `.MOD` row's stripped key) and its source file
    /// (`oa_spells.lst`), but NOT its source line -- the base declaration
    /// lives on one line, the `.MOD` row widening class access on another.
    /// A citation-aware ownership predicate must leave the sibling's file
    /// alone even though `current_keys` (this run's own live spell set)
    /// does not contain the key, because that key belongs to a DIFFERENT
    /// record at a DIFFERENT line the caller never wrote.
    #[test]
    fn a_sibling_generators_record_sharing_the_same_key_and_file_survives() {
        let root = std::env::temp_dir()
            .join(format!("codex_cross_gen_guard_survives_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let dir = root.join("spell");
        // The sibling (`spell_mod_access`-shaped) record: same key, same
        // file, a DIFFERENT line (the `.MOD` row's own line, 4021, never a
        // base declaration line).
        write_stub_record(
            &dir,
            "ablative_barrier_mod",
            "ablative barrier",
            "pathfinder/paizo/roleplaying_game/occult_adventures/oa_spells.lst",
            4021,
        );

        // This run's own `current_keys` does NOT contain "ablative
        // barrier" (simulating the key being absent from the compiled
        // table this particular run, or simply not this record's owner).
        let current_keys: std::collections::HashSet<String> = std::collections::HashSet::new();
        // This run's own citation index: it only ever cites line 1618 for
        // this file (the base declaration), never 4021.
        let owned_lines: std::collections::HashSet<u32> = [1618u32].into_iter().collect();
        let owned_path = "pathfinder/paizo/roleplaying_game/occult_adventures/oa_spells.lst";
        remove_stale_owned_files(&dir, &current_keys, &|path, line| {
            path == owned_path && owned_lines.contains(&line)
        });

        assert!(
            dir.join("ablative_barrier_mod.json").exists(),
            "a sibling generator's record must survive even when its key is absent from this \
             run's current_keys, because its citation line was never this generator's own"
        );
        std::fs::remove_dir_all(&root).ok();
    }

    /// The mirror case: THIS generator's own record, genuinely stale (its
    /// key is absent from `current_keys` AND its citation line IS one this
    /// run's own parse produced), must still be removed -- the guard must
    /// not become so conservative it stops cleaning up real drops.
    #[test]
    fn this_generators_own_stale_record_is_still_removed() {
        let root =
            std::env::temp_dir().join(format!("codex_cross_gen_guard_removes_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let dir = root.join("spell");
        let owned_path = "pathfinder/paizo/roleplaying_game/occult_adventures/oa_spells.lst";
        write_stub_record(&dir, "now_pi_blocked", "now pi blocked", owned_path, 1618);

        let current_keys: std::collections::HashSet<String> = std::collections::HashSet::new();
        let owned_lines: std::collections::HashSet<u32> = [1618u32].into_iter().collect();
        remove_stale_owned_files(&dir, &current_keys, &|path, line| {
            path == owned_path && owned_lines.contains(&line)
        });

        assert!(
            !dir.join("now_pi_blocked.json").exists(),
            "a genuinely stale record this generator itself owns (matching citation, absent key) \
             must still be removed"
        );
        std::fs::remove_dir_all(&root).ok();
    }

    /// Mutation proof for the two tests above: widening the ownership
    /// predicate back to "any file in this directory" (the pre-fix shape,
    /// `|_path, _line| true`) must turn
    /// `a_sibling_generators_record_sharing_the_same_key_and_file_survives`
    /// red. This test pins that the SURVIVAL assertion is actually load-
    /// bearing rather than vacuously true, by re-running the identical
    /// scenario with the unscoped predicate and asserting the sibling
    /// record is (wrongly) deleted -- proving the guard, not the test
    /// setup, is what protects the sibling above.
    #[test]
    fn an_unscoped_key_only_predicate_reproduces_the_incident() {
        let root =
            std::env::temp_dir().join(format!("codex_cross_gen_guard_mutation_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let dir = root.join("spell");
        write_stub_record(
            &dir,
            "ablative_barrier_mod",
            "ablative barrier",
            "pathfinder/paizo/roleplaying_game/occult_adventures/oa_spells.lst",
            4021,
        );
        let current_keys: std::collections::HashSet<String> = std::collections::HashSet::new();
        // The pre-fix shape: any file physically in the directory is
        // treated as owned, regardless of citation.
        remove_stale_owned_files(&dir, &current_keys, &|_path, _line| true);

        assert!(
            !dir.join("ablative_barrier_mod.json").exists(),
            "sanity check: the unscoped predicate must reproduce the incident (deletion) so the \
             citation-aware guard's protection is proven non-vacuous"
        );
        std::fs::remove_dir_all(&root).ok();
    }
}
