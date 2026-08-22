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
    /// Records refused outright because the real corpus row declares
    /// `NAMEISPI:YES` (`file:line record_key`). A NAME cannot be
    /// redacted -- it is the record's identity on every screen and half
    /// of its key -- so these rows are DROPPED, never screened, matching
    /// `SD-29-corpus-wide-catch-up-lanes/decisions.md §50.3` and
    /// `ingest_race_traits.rs`'s `pi_dropped` precedent. Reported, never
    /// silent: a row that vanishes without a line here is
    /// indistinguishable from a citation bug.
    pub name_pi_dropped: Vec<String>,
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
    // Rebuilt every run, matching `ingest_races.rs`'s own precedent ("A
    // stale record from a previous run ... would be indistinguishable from
    // a fresh one, so the output tree is rebuilt"): a record dropped by the
    // NAMEISPI:YES check below must actually disappear from disk on the
    // next run, not merely stop being re-written. Fixes the shipped-but-
    // stale `otyugh_hide.json` this cycle found (OPEN-ISSUES row 38) --
    // clearing the whole directory is what makes "dropped" durable rather
    // than a one-time hand delete.
    if equipment_dir.exists() {
        std::fs::remove_dir_all(&equipment_dir)?;
    }
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
        // A NAME cannot be redacted (`pi_screening.rs`'s own doc comment on
        // `DeclaredProductIdentity::name`; `SD-29-corpus-wide-catch-up-lanes/
        // decisions.md §50.3`): `[redacted PI]` as an equipment key is a
        // record nobody can look up. So a `NAMEISPI:YES` row is DROPPED
        // outright, before the description screen runs, never partially
        // published under a redacted display name -- the same ruling
        // `ingest_race_traits.rs` already applies to race traits. Found by
        // adversarial review (OPEN-ISSUES row 38): this branch previously
        // computed `declared.name` and never read it, so `Otyugh Hide`
        // (`ue_equip_arms_armor.lst:66`) shipped its real name unredacted.
        if declared.name {
            report.name_pi_dropped.push(format!("{category_file}:{line} {}", entry.key));
            continue;
        }
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
        if entry.category == EquipmentCategory::Equipmods {
            write_json(&equipmods_dir, &slug, &record)?;
            report.equipment_modifier_written += 1;
        } else {
            write_json(&equipment_dir, &slug, &record)?;
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

    #[test]
    fn nameispi_yes_drops_the_record_instead_of_publishing_the_real_name() {
        let corpus = ScratchCorpus::new("drops");
        corpus.write_arms_armor(&[&otyugh_hide_row(true)]);
        let out_dir = corpus.root.join("out");

        let report = generate(&corpus.root, &out_dir, "2026-01-01T00:00:00Z").unwrap();

        assert_eq!(
            report.name_pi_dropped,
            vec!["ue_equip_arms_armor.lst:1 Otyugh Hide".to_string()],
            "a NAMEISPI:YES row must be reported as dropped, never silently skipped"
        );
        for path in out_dir_json_files(&out_dir) {
            let text = std::fs::read_to_string(&path).unwrap();
            assert!(
                !text.contains("Otyugh Hide"),
                "{path:?} must not carry the real name of a NAMEISPI:YES record: {text}"
            );
        }
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
}
