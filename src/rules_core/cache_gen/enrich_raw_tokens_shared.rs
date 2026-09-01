//! Shared engine behind the `enrich_<kind>_raw_tokens` bin family
//! (`enrich_companion_raw_tokens.rs`, `enrich_monster_raw_tokens.rs`,
//! `enrich_monster_ability_raw_tokens.rs`, `enrich_spell_raw_tokens.rs`).
//! `enrich_class_raw_tokens.rs` (uses `cache_gen::WiringClassIndex`) and
//! `enrich_equipment_raw_tokens.rs` (uses a typed copy-base parser) are
//! genuinely different tools and do NOT use this module — R8-04's own
//! `verify_note` confirms only these four share one shape.
//!
//! This module holds the byte-identical parts of that shape: the file walk,
//! the `book_dir_of`/`.MOD`-index resolution, the token-closure decomposition,
//! and the PI-screen-then-write sequence. What genuinely differs between the
//! four bins (whether `<system>/<publisher>/<book>/<file>` -- no `<line>`
//! tier -- is a valid citation shape for `dreamscarred_press`; which
//! `data.*` fields seed a record's closure identities; the per-field PI
//! screen; whether a `NAMEISPI:YES` hit deletes the shipped file outright or
//! merely skips writing `raw_tokens`; whether a redacted field also stamps
//! `license`/`pi_field`/`pi_marker` at the record root) is captured in
//! [`EnrichConfig`], one instance per bin, so each bin's own behavior is
//! reproduced exactly rather than averaged across the family.

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::rules_core::corpus_literal_sweep::token_closure;
use crate::rules_core::pi_screening::declared_product_identity;
use crate::rules_core::shape_b_v1::{License, REDACTED_PI_MARKER};
use crate::rules_core::wiring_class::build_mod_index;
use serde_json::{Value, json};

/// `PCGEN_CORPUS_ROOT` when set, else `$HOME/workspace/repos/pcgen/data` --
/// byte-identical across all four bins.
pub fn pcgen_data_root() -> PathBuf {
    if let Ok(v) = env::var("PCGEN_CORPUS_ROOT") {
        return PathBuf::from(v);
    }
    let home = env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
    PathBuf::from(home).join("workspace/repos/pcgen/data")
}

/// Every JSON file under a book's `<kind_subdir>/` directory, walked
/// recursively (a flat `read_dir` silently under-reports a book that nests
/// this kind's records, e.g. `spell`'s `core_rulebook` `level_N/`
/// subdirectories -- byte-identical shape across all four bins, only the
/// subdirectory name differs).
pub fn find_kind_json_files(book_dir: &Path, kind_subdir: &str) -> Vec<PathBuf> {
    let dir = book_dir.join(kind_subdir);
    let mut out = Vec::new();
    let mut stack = vec![dir];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

/// The corpus-relative book directory a `source.path` citation belongs to:
/// its first four path segments (`<system>/<publisher>/<line>/<book>`) --
/// byte-identical to `corpus_literal_sweep`'s own `book_dir_of` for a
/// standard four-tier citation. Used by `enrich_companion_raw_tokens.rs` and
/// `enrich_spell_raw_tokens.rs`, neither of which ships a
/// `dreamscarred_press`-sourced record today.
pub fn book_dir_of_strict(source_path: &str) -> Option<String> {
    let segments: Vec<&str> = source_path.split('/').filter(|s| !s.is_empty()).collect();
    if segments.len() < 5 {
        return None;
    }
    Some(segments[..4].join("/"))
}

/// Same as [`book_dir_of_strict`], but also accepts `dreamscarred_press`'s
/// three-segment `<system>/<publisher>/<book>/<file>` layout (no `<line>`
/// tier) -- `corpus_literal_sweep`'s own `book_dir_of` special-cases this;
/// `enrich_monster_raw_tokens.rs` and `enrich_monster_ability_raw_tokens.rs`
/// both ship real `ultimate_psionics` citations that need this branch (SD-31
/// wave 20).
pub fn book_dir_of_with_dreamscarred_press(source_path: &str) -> Option<String> {
    let segments: Vec<&str> = source_path.split('/').filter(|s| !s.is_empty()).collect();
    if segments.len() >= 5 {
        return Some(segments[..4].join("/"));
    }
    if segments.len() == 4 && segments[1] == "dreamscarred_press" {
        return Some(segments[..3].join("/"));
    }
    None
}

/// One book's `.MOD` rows, keyed by the record name they target -- the same
/// derivation `corpus_literal_sweep`'s own `Sweep::mod_index` performs,
/// duplicated here (not imported: that method is a private impl on a
/// binary-local struct, not part of the library). `book_dir` is the FULL
/// corpus-relative directory (a [`book_dir_of_strict`]/
/// [`book_dir_of_with_dreamscarred_press`] return), never the short book
/// slug.
pub fn mod_index_for_book(data_root: &Path, book_dir: &str) -> BTreeMap<String, Vec<String>> {
    let mut book_paths = BTreeMap::new();
    book_paths.insert(book_dir.to_string(), data_root.join(book_dir));
    build_mod_index(&book_paths).into_iter().map(|((_, name), rows)| (name, rows)).collect()
}

/// Split one closure field (`"COST:150"`, `"DESC:some text: with colons"`)
/// into a `{key, value}` pair on the FIRST colon. Round-trips exactly:
/// `format!("{key}:{value}")` always reconstructs the original field
/// (`corpus_literal_sweep::ShippedToken::joined`), for any field that
/// contains at least one colon -- every PCGen `TAG:VALUE` token does, by
/// construction of the format this closure was built from.
pub fn split_token_field(field: &str) -> Option<(&str, &str)> {
    field.split_once(':')
}

/// PI-screen one closure field's value: blacklist term scan
/// ([`crate::rules_core::pi_screening::classify_field`]) union'd with the
/// row's own `DESCISPI:YES` declaration for `DESC`-keyed fields specifically
/// -- SD-30 `§52.3`/`§53.5`, byte-identical contract shared by
/// `enrich_companion_raw_tokens.rs`, `enrich_monster_raw_tokens.rs`, and
/// `enrich_monster_ability_raw_tokens.rs`. `enrich_spell_raw_tokens.rs` uses
/// its own broader screen (also covers `BENEFIT`/`SPECIAL` fields and treats
/// any non-`Ogl` license as blacklisted, not just `PiRedacted`), kept local
/// to that bin rather than shared here.
pub fn screen_field_value(key: &str, value: &str, declared_description: bool) -> (String, bool) {
    if key.eq_ignore_ascii_case("DESC") && declared_description {
        return (REDACTED_PI_MARKER.to_string(), true);
    }
    let (license, ..) = crate::rules_core::pi_screening::classify_field(key, value);
    if license == License::PiRedacted {
        return (REDACTED_PI_MARKER.to_string(), true);
    }
    (value.to_string(), false)
}

/// What `enrich_one` did with one record -- one shared enum across all four
/// bins; each bin's own `main` only ever observes the subset its
/// [`EnrichConfig`] can actually produce ([`EnrichConfig::remove_file_on_name_pi`]
/// picks between [`Outcome::DroppedPi`] and [`Outcome::NameIsProductIdentity`]).
pub enum Outcome {
    Enriched { redacted_fields: usize },
    /// `remove_file_on_name_pi: true` path: the record's own closure
    /// declares `NAMEISPI:YES`; the shipped file has already been deleted
    /// (`decisions.md §50.3`: a name cannot be redacted, only dropped). The
    /// message names the citation and `record_key` for the caller's own log.
    DroppedPi(String),
    /// `remove_file_on_name_pi: false` path: same declaration, but the file
    /// is left on disk untouched -- `raw_tokens` is simply never written.
    NameIsProductIdentity,
    NoLstCitation,
    AlreadyEnriched,
    CitationMiss(String),
}

/// The per-bin behavior [`enrich_one`] is parametrized on. Every field here
/// is a real, provable difference between at least two of the four bins
/// (see this module's own doc comment) -- nothing here is a stand-in for
/// logic that was actually identical everywhere.
pub struct EnrichConfig {
    /// [`book_dir_of_strict`] or [`book_dir_of_with_dreamscarred_press`].
    pub book_dir_of: fn(&str) -> Option<String>,
    /// The `data.*` keys checked for a closure identity, e.g.
    /// `&["key", "name", "corpus_key"]`. `source.record_key` is ALWAYS
    /// checked in addition to this list (every bin does), so it is not
    /// itself a config field.
    pub identity_fields: &'static [&'static str],
    /// [`screen_field_value`] or a bin-local equivalent of the same
    /// `(key, value, declared_description) -> (stored_value, was_redacted)`
    /// contract.
    pub screen: fn(&str, &str, bool) -> (String, bool),
    /// `true` only for `enrich_monster_raw_tokens.rs`: when any field was
    /// redacted, additionally stamp `license`/`pi_field`/`pi_marker` on the
    /// record root.
    pub mark_redacted_root: bool,
    /// `true` for companion/monster/monster_ability: a `NAMEISPI:YES` hit
    /// deletes the shipped file and returns [`Outcome::DroppedPi`]. `false`
    /// for spell: the file is left alone and [`Outcome::NameIsProductIdentity`]
    /// is returned instead.
    pub remove_file_on_name_pi: bool,
}

/// Reads one shipped corpus JSON record at `path`, and -- if it carries an
/// `lst_token` citation but no `raw_tokens` yet -- writes the full token
/// closure the citation resolves to, PI-screened per `config`. Byte-for-byte
/// shared control flow across all four `enrich_<kind>_raw_tokens` bins;
/// `config` supplies the one axis each bin genuinely differs on.
pub fn enrich_one(
    path: &Path,
    data_root: &Path,
    mod_index_cache: &mut BTreeMap<String, BTreeMap<String, Vec<String>>>,
    config: &EnrichConfig,
) -> Outcome {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    let mut root: Value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("parse {path:?} as JSON: {e}"));

    {
        let data = root.get("data").unwrap_or_else(|| panic!("{path:?}: no top-level \"data\" object"));
        if data.get("raw_tokens").is_some() {
            return Outcome::AlreadyEnriched;
        }
    }

    let source = root["source"].clone();
    if source.get("kind").and_then(Value::as_str) != Some("lst_token") {
        return Outcome::NoLstCitation;
    }
    let lst_rel_path = source["path"].as_str().expect("lst_token source must carry a path").to_string();
    let line = source["line"].as_u64().expect("lst_token source must carry a line") as usize;
    let Some(book_dir) = (config.book_dir_of)(&lst_rel_path) else {
        return Outcome::CitationMiss(format!(
            "{lst_rel_path} is not <system>/<publisher>/<line>/<book>/<file>-shaped"
        ));
    };
    let mod_index = mod_index_cache
        .entry(book_dir.clone())
        .or_insert_with(|| mod_index_for_book(data_root, &book_dir));

    let lst_full_path = data_root.join(&lst_rel_path);
    let Ok(lst_text) = fs::read_to_string(&lst_full_path) else {
        return Outcome::CitationMiss(format!("cited LST file not found: {lst_full_path:?}"));
    };
    let lines: Vec<&str> = lst_text.split('\n').collect();
    if line == 0 || line > lines.len() {
        return Outcome::CitationMiss(format!(
            "{lst_rel_path} has {} lines, record claims line {line}",
            lines.len()
        ));
    }
    let base_row = lines[line - 1];

    let data_obj_ref = root.get("data").and_then(Value::as_object).expect("checked above");
    let mut identities: BTreeSet<String> = BTreeSet::new();
    for field in config.identity_fields {
        if let Some(name) = data_obj_ref.get(*field).and_then(Value::as_str) {
            identities.insert(name.to_string());
        }
    }
    if let Some(record_key) = source.get("record_key").and_then(Value::as_str) {
        identities.insert(record_key.to_string());
    }

    let closure = token_closure(base_row, &identities, mod_index, None);
    if closure.is_empty() {
        return Outcome::CitationMiss(format!(
            "{lst_rel_path}:{line}: base row carries no tab-separated fields at all -- \
             a genuinely malformed citation, not a missing token set"
        ));
    }

    let mut pairs: Vec<(&str, &str)> = Vec::with_capacity(closure.len());
    for field in &closure {
        let Some(pair) = split_token_field(field) else {
            return Outcome::CitationMiss(format!(
                "{lst_rel_path}:{line}: closure field {field:?} carries no ':' -- cannot be \
                 decomposed into a {{key,value}} pair that round-trips"
            ));
        };
        pairs.push(pair);
    }

    // `declared_product_identity` reads the WHOLE closure (base row + every
    // `.MOD` row targeting this record's own identities within the same
    // book), never just the base row alone -- SD-30 `§52.3`/`§53.5`.
    let declared = declared_product_identity(pairs.iter().copied());
    if declared.name {
        if config.remove_file_on_name_pi {
            fs::remove_file(path).unwrap_or_else(|e| panic!("remove {path:?}: {e}"));
            return Outcome::DroppedPi(format!(
                "{lst_rel_path}:{line} (record_key={:?}) declares NAMEISPI:YES in its own closure -- \
                 a name cannot be redacted, dropped per decisions.md §50.3",
                source.get("record_key").and_then(Value::as_str).unwrap_or("?")
            ));
        }
        return Outcome::NameIsProductIdentity;
    }

    let mut raw_tokens: Vec<Value> = Vec::with_capacity(pairs.len());
    let mut redacted_fields = 0usize;
    for (key, value) in &pairs {
        let (stored, redacted) = (config.screen)(key, value, declared.description);
        if redacted {
            redacted_fields += 1;
        }
        raw_tokens.push(json!({ "key": key, "value": stored }));
    }

    let data_obj = root.get_mut("data").and_then(Value::as_object_mut).expect("checked above");
    data_obj.insert("raw_tokens".to_string(), Value::Array(raw_tokens));
    if config.mark_redacted_root && redacted_fields > 0 {
        let root_obj = root.as_object_mut().expect("record root is an object");
        root_obj.insert("license".to_string(), json!("PI-REDACTED"));
        root_obj.insert("pi_field".to_string(), json!("raw_tokens"));
        root_obj.insert("pi_marker".to_string(), json!("redacted"));
    }

    let new_json = serde_json::to_string_pretty(&root).expect("serialize enriched record");
    fs::write(path, new_json + "\n").unwrap_or_else(|e| panic!("write {path:?}: {e}"));
    Outcome::Enriched { redacted_fields }
}
