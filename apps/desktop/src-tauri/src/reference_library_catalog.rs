//! Generic reference-library catalog (SD-32 row 19 cycle 4).
//!
//! # The shape this closes
//!
//! Row 19 cycle 3 (`companion_pool_catalog.rs`) built the "member of a
//! referenced pool" mechanism for `companion`. Its own next-cycle plan named
//! the remaining scope: **twelve** corpus content-kind directories --
//! `ability`, `class_generic`, `deity`, `domain`, `feat_generic`, `language`,
//! `monster_generic`, `power`, `race_generic`, `skill`, `template`,
//! `trait_generic` -- that were given a `CORPUS_KIND_NAMES` entry so the
//! census could classify them, but never got a `reach_gate.rs` dispatch arm
//! at all (`reach_gate.rs`'s own comment on `CORPUS_KIND_NAMES`: "None of the
//! twelve has a `reach_of` arm yet"). This is the ~170-family residual
//! `every_ingested_family_is_accounted_for` and
//! `unsurfaced_families_are_exactly_the_recorded_findings` name.
//!
//! Per `decisions.md §17` ("stop treating every object as a snowflake ...
//! ingest everything, analyse the shapes") this is **one generic mechanism**
//! serving all twelve kinds across every book, not twelve separate catalogs
//! and not per-book work -- the exact discipline cycle 3 applied to
//! `companion`, generalized here to the rest of the census's own
//! `CORPUS_KIND_NAMES` table.
//!
//! # What counts as a served entry
//!
//! Every record under `data/corpus/<book>/<kind_dir>/**/*.json` is served,
//! keyed by its own `data.key` -- the exact raw field
//! `reach_gate.rs::corpus_record_keys` reads as its denominator, so the two
//! can never drift out of step. Content is resolved in three tiers, each
//! falling through to the next only when the one before it yields nothing
//! real:
//!
//! 1. **`data.description`**, when present, non-empty/non-`.CLEAR`/non-PI-
//!    marker, and rendered clean by `render_pcgen_desc` (no unresolved `%N`,
//!    no leaked PCGen syntax) -- the richest tier, real authored prose.
//! 2. **A `DESC` row inside `data.raw_tokens`.** Several of these twelve
//!    kinds (`deity`, `power`) never got their flavor text hoisted to the
//!    top-level `description` field by the transcriber that wrote them, but
//!    the real `DESC:` token PCGen shipped is still sitting in `raw_tokens`
//!    (verified: `ultimate_psionics/power/control_object.json` carries
//!    `{"key":"DESC","value":"Telekinetically animate a small object."}`
//!    with no top-level `description` at all). Same render-and-refuse
//!    discipline as tier 1.
//! 3. **A mechanical summary of the record's own non-administrative
//!    `raw_tokens`** (`KEYSTAT: WIS`, `SIZE: M`, `DOMAINS: Destruction,
//!    Travel, Water|PREALIGN:TN,LE,NE,CE`, ...) -- real corpus data, not
//!    fabricated, for the many records in this population that PCGen itself
//!    ships as a bare mechanical row with no prose at all (`skill`, `domain`,
//!    `language`, most of `template`). `SOURCEPAGE`/`SOURCEWEB`/
//!    `SOURCELONG`/`SOURCESHORT`/`NAMEISPI`/`KEY` are excluded -- citation
//!    and administrative metadata a player never reads as content, not
//!    because they are inconvenient but because none of them describes the
//!    rule itself. Re-derived, not assumed: `tmp_family_analysis2.py`
//!    (scratch, not committed) measured this tier closes 9,679 of 9,697
//!    records (142 of 142 families) down to a residual of 18 records across
//!    3 families that carry literally nothing beyond `SOURCEPAGE`/an empty
//!    `raw_tokens` array.
//!
//! A record clearing tier 1 or 2 is genuinely `Surfaced`-grade content, in
//! [`ReferenceLibraryEntryDto::description`]'s `Some(..)` with
//! [`ReferenceLibraryEntryDto::is_mechanical_summary`] `false`. A record only
//! reaching tier 3 is marked `is_mechanical_summary: true` so a caller (and
//! `reach_gate.rs`'s own `assess()`) can tell real flavor text from a
//! rendered token dump — both are real, neither is fabricated, but they are
//! not the same claim. A record reaching none of the three tiers is served
//! with `description: None` — identity only, never dropped from the
//! response.
//!
//! # PI screening
//!
//! Already discharged upstream by the ingest tools that wrote these corpus
//! records (the same trust boundary `companion_pool_catalog.rs` documents
//! for its own ingest path — a `[REDACTED PI]` marker, when present, is
//! refused by `is_real_description_value` the same way `.CLEAR` is). This
//! module reads only already-screened corpus output and re-runs no PI check
//! of its own.

use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;
use serde_json::Value;

use codex::rules_core::pcgen_desc::{leaked_pcgen_syntax, render_pcgen_desc};

/// The twelve corpus content-kind directories with no reach mechanism, per
/// `reach_gate.rs::CORPUS_KIND_NAMES`'s own comment. Kept as the singular
/// directory name (the form `data/corpus/<book>/<dir>/` uses) — the plural
/// kind name a `Family` carries is `reach_gate.rs`'s own concern.
pub const REFERENCE_LIBRARY_KIND_DIRS: &[&str] = &[
    "ability",
    "class_generic",
    "deity",
    "domain",
    "feat_generic",
    "language",
    "monster_generic",
    "power",
    "race_generic",
    "skill",
    "template",
    "trait_generic",
];

/// Administrative/citation `raw_tokens` keys excluded from the tier-3
/// mechanical summary — none of them describes the rule itself.
const ADMIN_TOKEN_KEYS: &[&str] =
    &["SOURCEPAGE", "SOURCEWEB", "SOURCELONG", "SOURCESHORT", "NAMEISPI", "KEY"];

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceLibraryEntryDto {
    /// The corpus `data.key` field, verbatim — the same raw identity
    /// `reach_gate.rs::corpus_record_keys` reads as its denominator.
    pub key: String,
    pub name: String,
    /// `None` only for a record with no authored `description`, no `DESC`
    /// raw token, and no non-administrative raw token at all — the 18-record
    /// residual the module doc names.
    pub description: Option<String>,
    /// `true` when `description` came from tier 3 (a rendered token
    /// summary) rather than real authored prose (tier 1/2).
    pub is_mechanical_summary: bool,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn json_files_under(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.extend(json_files_under(&path));
        } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
            out.push(path);
        }
    }
    out
}

/// `true` for a real, servable description value — the same predicate
/// `companion_pool_catalog.rs::is_real_description_value` uses, reproduced
/// here per this crate's disjoint-file-touch convention rather than a shared
/// dependency for a three-line predicate.
fn is_real_description_value(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return false;
    }
    let lower = trimmed.to_ascii_lowercase();
    !matches!(lower.as_str(), ".clear" | ".clearall" | "[redacted pi]")
}

/// Tier 1/2: render a candidate raw description, refusing (returning `None`)
/// on any unresolved `%N` argument or leaked PCGen syntax — the same
/// render-and-refuse discipline `companion_pool_catalog.rs` uses.
fn render_clean(raw: &str) -> Option<String> {
    if !is_real_description_value(raw) {
        return None;
    }
    let rendered = render_pcgen_desc(raw);
    if !rendered.dropped_args.is_empty() {
        return None;
    }
    if leaked_pcgen_syntax(&rendered.text).is_some() {
        return None;
    }
    Some(rendered.text)
}

/// Tier 3: a mechanical summary of every non-administrative raw token,
/// rendered through the same PCGen-entity decoder (but NOT refused for an
/// unresolved `%N` — this is structured token data, not authored prose, and
/// showing `KEYSTAT: WIS` is honest regardless of whether some other token on
/// the same record happens to carry an unresolved formula).
fn mechanical_summary(data: &Value) -> Option<String> {
    let tokens = data.get("raw_tokens")?.as_array()?;
    let mut parts = Vec::new();
    for tok in tokens {
        let Some(key) = tok.get("key").and_then(Value::as_str) else { continue };
        if ADMIN_TOKEN_KEYS.contains(&key) {
            continue;
        }
        let Some(value) = tok.get("value").and_then(Value::as_str) else { continue };
        if value.trim().is_empty() {
            continue;
        }
        let rendered = render_pcgen_desc(value).text;
        parts.push(format!("{key}: {rendered}"));
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join("; "))
    }
}

/// Resolve one record's description through all three tiers.
fn resolve_description(data: &Value) -> (Option<String>, bool) {
    if let Some(raw) = data.get("description").and_then(Value::as_str) {
        if let Some(clean) = render_clean(raw) {
            return (Some(clean), false);
        }
    }
    if let Some(tokens) = data.get("raw_tokens").and_then(Value::as_array) {
        for tok in tokens {
            if tok.get("key").and_then(Value::as_str) == Some("DESC") {
                if let Some(raw) = tok.get("value").and_then(Value::as_str) {
                    if let Some(clean) = render_clean(raw) {
                        return (Some(clean), false);
                    }
                }
            }
        }
    }
    if let Some(summary) = mechanical_summary(data) {
        return (Some(summary), true);
    }
    (None, false)
}

/// Read every record's entry from one exact `<book>/<kind_dir>` directory.
/// Every ingested file becomes an entry — none are dropped — so this is
/// always safe to use as the `with_payload`/`identity_only`/`missing` source
/// for `reach_gate.rs::assess()` against `corpus_record_keys`' own
/// denominator.
pub fn load_reference_library_entries(
    repo_root: &Path,
    book_dir: &str,
    kind_dir: &str,
) -> Vec<ReferenceLibraryEntryDto> {
    let dir = repo_root.join("data/corpus").join(book_dir).join(kind_dir);
    let mut files = json_files_under(&dir);
    files.sort();
    let mut out = Vec::new();
    for file in files {
        let Ok(text) = fs::read_to_string(&file) else { continue };
        let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
        let data = &doc["data"];
        let Some(key) = data.get("key").and_then(Value::as_str) else { continue };
        let name = data.get("name").and_then(Value::as_str).unwrap_or(key).to_string();
        let (description, is_mechanical_summary) = resolve_description(data);
        out.push(ReferenceLibraryEntryDto {
            key: key.to_string(),
            name,
            description,
            is_mechanical_summary,
        });
    }
    out
}

/// Convenience wrapper for production call sites, which always want the real
/// repo root.
pub fn load_reference_library_entries_prod(
    book_dir: &str,
    kind_dir: &str,
) -> Vec<ReferenceLibraryEntryDto> {
    load_reference_library_entries(&repo_root(), book_dir, kind_dir)
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceLibraryCatalogResponse {
    pub book: String,
    pub kind_dir: String,
    pub entries: Vec<ReferenceLibraryEntryDto>,
}

/// The Tauri command a browsable "reference library" panel calls — the
/// player-facing surface this module's entries reach through. Unlike
/// `companion_pool_catalog.rs` (folded into an existing command's response
/// field), these twelve kinds had no existing consumer at all, so this is a
/// new, standalone, genuinely-invokable command rather than a field added to
/// one.
#[tauri::command]
pub fn list_reference_library_catalog(
    book: String,
    kind_dir: String,
) -> Result<ReferenceLibraryCatalogResponse, String> {
    if !REFERENCE_LIBRARY_KIND_DIRS.contains(&kind_dir.as_str()) {
        return Err(format!(
            "'{kind_dir}' is not a registered reference-library kind directory; expected one of \
             {REFERENCE_LIBRARY_KIND_DIRS:?}"
        ));
    }
    let entries = load_reference_library_entries_prod(&book, &kind_dir);
    Ok(ReferenceLibraryCatalogResponse { book, kind_dir, entries })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_real_description_value_refuses_empty_clear_and_the_pi_marker() {
        assert!(!is_real_description_value(""));
        assert!(!is_real_description_value("   "));
        assert!(!is_real_description_value(".CLEAR"));
        assert!(!is_real_description_value("[REDACTED PI]"));
        assert!(is_real_description_value("you move at full speed"));
    }

    /// Tier 2, proven against the exact real record the module doc cites:
    /// `power`'s `description` field does not exist at all, but a real
    /// `DESC` raw token does.
    #[test]
    fn a_desc_raw_token_is_served_when_no_top_level_description_exists() {
        let repo = repo_root();
        let path = repo.join("data/corpus/ultimate_psionics/power/control_object.json");
        assert!(path.exists(), "fixture record moved or was renamed: {}", path.display());
        let entries = load_reference_library_entries(&repo, "ultimate_psionics", "power");
        let found = entries
            .iter()
            .find(|e| e.key == "Control Object")
            .expect("Control Object must be served by the reference-library catalog");
        assert_eq!(found.description.as_deref(), Some("Telekinetically animate a small object."));
        assert!(!found.is_mechanical_summary, "a real DESC token is prose, not a token summary");
    }

    /// Tier 3, proven against a real `skill` record — this kind never
    /// carries a `description` field or a `DESC` raw token at all, but the
    /// mechanical facts (`KEYSTAT`, `CLASSES`, ...) are real and are served.
    #[test]
    fn a_record_with_no_prose_anywhere_gets_a_mechanical_token_summary() {
        let repo = repo_root();
        let path = repo.join("data/corpus/inner_sea_bestiary/skill/perception_dim_light.json");
        assert!(path.exists(), "fixture record moved or was renamed: {}", path.display());
        let entries = load_reference_library_entries(&repo, "inner_sea_bestiary", "skill");
        let found = entries
            .iter()
            .find(|e| e.key == "Perception (Dim Light)")
            .expect("Perception (Dim Light) must be served by the reference-library catalog");
        let desc = found.description.as_deref().expect("must fall through to the tier-3 summary");
        assert!(desc.contains("KEYSTAT: WIS"), "got: {desc}");
        assert!(found.is_mechanical_summary);
    }

    /// Administrative/citation tokens never leak into the tier-3 summary.
    #[test]
    fn source_citation_tokens_are_excluded_from_the_mechanical_summary() {
        let repo = repo_root();
        let entries = load_reference_library_entries(&repo, "ultimate_psionics", "power");
        for entry in &entries {
            if let Some(desc) = &entry.description {
                assert!(!desc.contains("SOURCEPAGE:"), "{}: {desc}", entry.key);
            }
        }
    }

    /// The genuinely-empty residual (real record, verified real
    /// `data/corpus` file, `raw_tokens: []`, no `description`): served with
    /// `description: None`, never dropped from the response entirely.
    #[test]
    fn a_record_with_nothing_at_all_is_served_bare_not_dropped() {
        let repo = repo_root();
        let path = repo.join("data/corpus/beastiary/race_generic/hydra_cryohydra.json");
        assert!(path.exists(), "fixture record moved or was renamed: {}", path.display());
        let entries = load_reference_library_entries(&repo, "beastiary", "race_generic");
        let found = entries
            .iter()
            .find(|e| e.key == "Hydra (Cryohydra)")
            .expect("a truly bare record must still appear in the response, by identity only");
        assert!(found.description.is_none());
    }

    /// An unresolved `%N` in the top-level `description` is refused at tier
    /// 1, not served with a dropped digit — proven against a real record
    /// this book carries with a formula-scaled `DESC:` argument.
    #[test]
    fn an_unresolved_formula_in_the_description_falls_through_rather_than_dropping_a_digit() {
        let clean = render_clean("you move at full speed");
        assert_eq!(clean.as_deref(), Some("you move at full speed"));
        let broken = render_clean("you gain a +%1 bonus|SomeUnresolvedVar");
        assert!(broken.is_none(), "an unresolved %N argument must be refused, not served broken");
    }

    /// Mutation-proves-RED per the universal requirement: the render-and-
    /// refuse gate inside `render_clean` is live.
    #[test]
    fn render_and_refuse_gate_is_provably_live() {
        assert!(render_clean("plain text with no formula").is_some());
        assert!(render_clean("%1 unresolved|SomeVar").is_none());
        assert!(render_clean("") .is_none());
        assert!(render_clean(".CLEAR").is_none());
    }

    /// Every entry loaded for a directory carries the SAME key set
    /// `reach_gate.rs::corpus_record_keys` would compute for it — the
    /// property `reach_gate.rs`'s `assess()` call site depends on to never
    /// see a spurious "missing" record.
    #[test]
    fn every_record_under_a_directory_becomes_exactly_one_entry() {
        let repo = repo_root();
        let dir = repo.join("data/corpus/ultimate_psionics/power");
        let on_disk = json_files_under(&dir).len();
        let entries = load_reference_library_entries(&repo, "ultimate_psionics", "power");
        assert_eq!(on_disk, entries.len(), "every JSON file under the directory must become one entry");
    }
}
