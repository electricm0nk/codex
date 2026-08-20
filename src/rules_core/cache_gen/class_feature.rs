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
//! ## Scope THIS cycle: primary files only (`BOOK_PRIMARY_FILES`)
//!
//! `v06_work_inventory::enumerate_book` walks a book's ENTIRE directory
//! tree recursively, so some books' `class_feature` population spans not
//! only their own primary `<abbrev>_abilities_class.lst` but also nested
//! `support/*_abilities_class_*.lst` and `_pfs/*.lst` cross-book variant
//! files (e.g. `ultimate_combat/support/uc_abilities_class_um.lst`).
//! `corpus_literal_sweep`'s own `--json-out` book-attribution helper has a
//! CONFIRMED, out-of-this-card's-territory bug (`OPEN-ISSUES.md` row 22,
//! `src/bin/corpus_literal_sweep.rs`, a file this card may not edit) that
//! derives a shipped record's book from `source.path`'s PARENT directory
//! name rather than the real book -- so a record whose real citation lives
//! under a nested nested subdirectory would misattribute if `source.path`
//! encoded that nesting. Every record this generator writes therefore
//! cites a FLAT `<book-dir>/<primary-file>` path (matching every other
//! working generator's shape), and units sourced from a nested support/PFS
//! file are simply not in this cycle's population -- a real, named
//! shortfall (`OPEN-ISSUES.md`, this cycle's own row), not a silent gap.
//! `pathfinder_unchained` is excluded entirely: it already carries 64
//! hand-curated `class_feature` records from earlier mechanism-wiring
//! cycles (`barbarian_unchained_class/`, `monk_unchained_class/`, ...) that
//! this generator must not overwrite.
//!
//! `ultimate_psionics` is ALSO excluded this cycle (found live, not
//! anticipated): `corpus_literal_sweep::book_dir_of` hard-requires a
//! 5-segment `source.path` (`<system>/<publisher>/<line>/<book>/<file>`),
//! but Dreamscarred Press's own corpus layout has no "line" tier --
//! `pathfinder/dreamscarred_press/ultimate_psionics/<file>` is only 4
//! segments -- so every `ultimate_psionics` record this generator wrote
//! failed the sweep with `source.path ... is not
//! <system>/<publisher>/<line>/<book>/<file>-shaped`
//! (`cargo run --locked --bin corpus_literal_sweep`, reproduced this
//! cycle). `book_dir_of` lives in `src/bin/corpus_literal_sweep.rs`, a
//! file this card may not edit -- logged as a named shortfall
//! (`OPEN-ISSUES.md`) rather than shipped with a dirty sweep.
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
use crate::rules_core::corpus_literal_sweep::tab_tokens;
use crate::rules_core::pi_screening::{self, DeclaredProductIdentity};

/// `(book id, corpus-relative directory, primary `_abilities_class.lst`
/// basename)` for every one of the 23 in-scope `class_feature` books
/// EXCEPT `pathfinder_unchained` (already hand-ingested) and
/// `ultimate_psionics` (its non-Paizo path shape breaks
/// `corpus_literal_sweep::book_dir_of`, both excluded on purpose -- see
/// module doc comment). Re-derived this cycle directly against the pinned
/// oracle checkout, one book at a time:
/// `find "$PCGEN_CORPUS_ROOT/pathfinder" -iname '<book>' -type d` then
/// `ls` that directory for its own `*_abilities_class*.lst`.
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
}

/// Parses `units_from_inventory_json`'s input: every `kind == "class_feature"`
/// entry of a `docs/work-inventory.json`-shaped document, restricted to
/// [`BOOK_PRIMARY_FILES`]' books and each book's own primary file (the
/// module doc comment's scope note). Never touches the filesystem; pure
/// parsing of already-computed fields.
pub fn units_from_inventory_json(json_text: &str) -> Result<Vec<ClassFeatureSourceUnit>, String> {
    let doc: Value = serde_json::from_str(json_text).map_err(|e| format!("invalid inventory JSON: {e}"))?;
    let primary_file_by_book: BTreeMap<&str, &str> =
        BOOK_PRIMARY_FILES.iter().map(|(book, _, file)| (*book, *file)).collect();
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
        let Some(&primary_file) = primary_file_by_book.get(book) else { continue };
        let Some(source_file) = unit.get("source_file").and_then(Value::as_str) else { continue };
        if source_file != primary_file {
            continue;
        }
        let Some(source_line) = unit.get("source_line").and_then(Value::as_u64) else { continue };
        let Some(key) = unit.get("corpus_key").and_then(Value::as_str) else { continue };
        let Some(name) = unit.get("name").and_then(Value::as_str) else { continue };
        out.push(ClassFeatureSourceUnit {
            book: book.to_string(),
            source_file: source_file.to_string(),
            source_line: source_line as u32,
            key: key.to_string(),
            name: name.to_string(),
        });
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
    pub description: Option<String>,
    pub raw_tokens: Vec<RawToken>,
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
}

#[derive(Debug, Default)]
pub struct GenerationReport {
    pub written: usize,
    /// Records skipped because their row declares `NAMEISPI:YES` (module
    /// doc comment's PI-screening section) -- never written, counted
    /// instead of silently dropped.
    pub name_pi_skipped: usize,
    /// `(book, source_file, source_line)` citations that did not resolve
    /// to a real corpus line -- should be empty against the real corpus,
    /// since every citation here was already validated by
    /// `v06_work_inventory`'s own enumeration.
    pub unresolved_citations: Vec<String>,
    pub books_written: BTreeSet<String>,
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
) -> Result<GenerationReport, GenerationError> {
    let mut report = GenerationReport::default();
    let dir_by_book: BTreeMap<&str, &str> =
        BOOK_PRIMARY_FILES.iter().map(|(book, dir, _)| (*book, *dir)).collect();

    let mut units_by_book: BTreeMap<&str, Vec<&ClassFeatureSourceUnit>> = BTreeMap::new();
    for unit in units {
        units_by_book.entry(unit.book.as_str()).or_default().push(unit);
    }

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

        for unit in book_units {
            let Some(raw_row) = lines.line(book, &unit.source_file, unit.source_line as usize) else {
                report.unresolved_citations.push(format!("{book}:{}:{}", unit.source_file, unit.source_line));
                continue;
            };
            let file_path = book_dir.join(&unit.source_file);
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
            if declared.name || name_license == crate::rules_core::shape_b_v1::License::PiRedacted {
                report.name_pi_skipped += 1;
                continue;
            }
            let mut tokens = row_tokens(&raw_row);
            let description = desc_value(&tokens);
            let (license, pi_field, pi_marker, stored_desc) = pi_screening::classify_optional_field_declared(
                "description",
                description.as_deref(),
                declared.description,
            );
            // W19-INTEGRATE fix (adversarial review, OPEN-ISSUES.md row 63 follow-up):
            // `description`/`stored_desc` above is correctly PI-screened, but `tokens`
            // (below, shipped verbatim as `data.raw_tokens`) was NOT -- a declared
            // DESCISPI:Yes row had its full Product-Identity prose re-exposed through
            // raw_tokens even while `data.description` carried the redaction marker.
            // Mirror `enrich_equipment_raw_tokens.rs::screen_field_value`'s precedent.
            redact_desc_token_if_pi(&mut tokens, license);
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
            // `true_class` (wave 22's grant-fact ground truth, this cycle's
            // own fix -- `OPEN-ISSUES.md` row 334's closing note) wins when
            // it resolves this key; the key-prefix split is only a fallback
            // for the population the grant data does not yet cover. See
            // module doc comment / `true_class_by_key`.
            let class = true_class.get(&unit.key).cloned().or_else(|| key_owner.clone());

            let record = CacheRecord {
                population: Population::InScope,
                completeness,
                ingested_at: ingested_at.to_string(),
                data: ClassFeatureData {
                    key: unit.key.clone(),
                    name: unit.name.clone(),
                    class: class.clone(),
                    description: stored_desc,
                    raw_tokens: tokens,
                },
                source: Source::LstToken {
                    path: format!("{rel_dir}/{}", unit.source_file),
                    sha256,
                    line: unit.source_line,
                    record_key: unit.key.clone(),
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
            let class_dir_slug = slugify(key_owner.as_deref().unwrap_or(&unit.name), &mut BTreeSet::new());
            let feature_slug = {
                let key_for_used = format!("{class_dir_slug}/");
                let mut scoped: BTreeSet<String> = used
                    .iter()
                    .filter_map(|u| u.strip_prefix(&key_for_used).map(str::to_string))
                    .collect();
                let slug = slugify(&unit.name, &mut scoped);
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
    fn book_primary_files_covers_the_21_in_scope_books() {
        assert_eq!(BOOK_PRIMARY_FILES.len(), 21);
        assert!(!BOOK_PRIMARY_FILES.iter().any(|(book, _, _)| *book == "pathfinder_unchained"));
        // `ultimate_psionics` is excluded this cycle -- see module doc
        // comment's `book_dir_of` 5-segment-path finding.
        assert!(!BOOK_PRIMARY_FILES.iter().any(|(book, _, _)| *book == "ultimate_psionics"));
    }

    #[test]
    fn units_from_inventory_json_filters_to_class_feature_primary_file_rows() {
        let json = r#"{"units":[
            {"kind":"class_feature","book":"core_rulebook","source_file":"cr_abilities_class.lst","source_line":1615,"corpus_key":"Rogue ~ Sneak Attack","name":"Sneak Attack"},
            {"kind":"class_feature","book":"core_rulebook","source_file":"some_other_file.lst","source_line":4,"corpus_key":"X ~ Y","name":"Y"},
            {"kind":"feat","book":"core_rulebook","source_file":"cr_abilities_class.lst","source_line":5,"corpus_key":"Z","name":"Z"},
            {"kind":"class_feature","book":"not_a_book","source_file":"nope.lst","source_line":5,"corpus_key":"Z","name":"Z"}
        ]}"#;
        let units = units_from_inventory_json(json).unwrap();
        assert_eq!(units.len(), 1);
        assert_eq!(units[0].key, "Rogue ~ Sneak Attack");
        assert_eq!(units[0].source_line, 1615);
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
        assert!(map.get("Some Other Key ~ Feature").is_none());
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
        assert!(map.get("Shared ~ Feature").is_none(), "an ambiguous key must not resolve to either class");
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
}
