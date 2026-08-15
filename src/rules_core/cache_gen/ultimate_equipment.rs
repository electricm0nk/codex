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

fn write_json<T: Serialize>(out_dir: &Path, slug: &str, record: &CacheRecord<T>) -> std::io::Result<()> {
    std::fs::create_dir_all(out_dir)?;
    let path = out_dir.join(format!("{slug}.json"));
    let json = serde_json::to_string_pretty(record)
        .expect("CacheRecord<T> is a plain-data shape; serialization cannot fail");
    std::fs::write(path, json)
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
        let source = Source::LstToken {
            path: format!("{UE_DIR}/{category_file}"),
            sha256: sha_by_file.get(category_file).cloned().unwrap_or_default(),
            line,
            record_key: entry.key.to_string(),
        };
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
        let (license, pi_field, pi_marker, stored_desc) = pi_screening::classify_optional_field_declared(
            "description",
            entry.description,
            declared.description,
        );
        let record = CacheRecord {
            population: Population::InScope,
            completeness,
            ingested_at: ingested_at.to_string(),
            data: EquipmentData {
                key: entry.key.to_string(),
                category: format!("{:?}", entry.category),
                name: entry.name.to_string(),
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
        };
        let slug = slugify(entry.key, &mut used);
        write_json(&equipment_dir, &slug, &record)?;
        if entry.category == EquipmentCategory::Equipmods {
            report.equipment_modifier_written += 1;
        } else {
            report.equipment_written += 1;
        }
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
}
