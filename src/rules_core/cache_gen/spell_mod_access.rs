//! Spell class-access `.MOD` row JSON cache generator
//! (SD-32 `decisions.md §20`, card 11 next-cycle-plan item 1).
//!
//! **The gap this closes.** `scripts/shape_ledger.py`'s join is
//! `(book, source_basename, source_line)`. A PCGen spell `.lst` file
//! carries two shapes of record: a base declaration (already dumped by
//! `cache_gen::spell_lane_dump` / the per-book `ingest_*_spells.rs`
//! binaries, via `spell_list::SPELL_LIST`) and a `.MOD` row that widens an
//! EXISTING spell's class access without declaring a new spell
//! (`Occultist Spell ~ Accelerate Poison.MOD ... CLASSES:Occultist=2`).
//! `docs/work-inventory.json`'s own enumerator (`v06_work_inventory.rs`'s
//! `Kind::Spell` predicate, `has_token(SCHOOL:) || has_token(CLASSES:)`)
//! counts each such `.MOD` row as its OWN unit — a real, citable corpus
//! line, distinct from the base declaration it widens — but no generator
//! had ever dumped these rows to `data/corpus/`, so every one reported
//! `no_record`. This module is that dump.
//!
//! **Never re-parses raw PCGen LST to derive a field's *value* that some
//! compiled Rust module already holds** — the general
//! `decisions.md §11.3` rule does not apply here in the usual sense
//! because THERE IS no compiled Rust table for these rows (they were never
//! ingested anywhere; that is the whole gap). This module reads the row's
//! own tab-delimited fields ONCE and transcribes them literally into
//! `raw_tokens` — it does not interpret, compute, or invent a value beyond
//! what the row states.
//!
//! **Scope: rows whose first field ends in `.MOD` and which carry a
//! `CLASSES:` token**, across four books whose `no_record` `spell`
//! population's `origin` field is `mod_only`
//! (`epic-2-feat-spell-no-record-closure_cycle-1_cycle_receipt.md`,
//! re-derived fresh this cycle): `occult_adventures` (328),
//! `ultimate_magic` (19), `advanced_players_guide` (16 rows scanned; some
//! already counted `matched` elsewhere). A fifth unit
//! (`book_of_the_damned_volume_1`, `pfs_botd1_spells.lst:13`,
//! `Greater Teleport (Self Plus 50 Lbs. Of Objects Only).MOD`) carries
//! `TYPE:`/`PRETYPE:` tokens but no `CLASSES:`/`SCHOOL:` token of its own —
//! a different shape (its `spell`-kind classification must trace back to
//! its MOD target rather than its own fields) — and is deliberately left
//! out of this pass's scope rather than force-matched by a looser filter.
//!
//! ## PI screening
//!
//! Same union every sibling `cache_gen` generator applies
//! (`decisions.md §15`/`§19`): a row whose own `NAMEISPI:YES`/`DESCISPI:YES`
//! declares Product Identity, or whose free-text hits the blacklist term
//! list, is screened. A name-PI hit drops the whole record (a name cannot
//! be redacted); a description-PI hit redacts only that field.
//!
//! Run with `cargo run --locked --bin gen_cache_spell_mod_access`.
//! `PCGEN_CORPUS_ROOT` overrides the default
//! `$HOME/workspace/repos/pcgen/data`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::rules_core::cache_gen::WiringClassIndex;
use crate::rules_core::pi_screening::{self, declared_product_identity};
use crate::rules_core::shape_b_v1::License;

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
pub struct SpellModAccessData {
    /// The row's own first-field text, `.MOD` suffix stripped — this
    /// program's identity for the row (matches
    /// `docs/work-inventory.json`'s own `name` field for the same unit).
    pub key: String,
    /// The base spell's real display name, from `OUTPUTNAME:` when the row
    /// carries one, else identical to `key`.
    pub name: String,
    pub description: Option<String>,
    /// Every tab-delimited `TOKEN:value` pair on the row (excluding the
    /// first identity field and the bare `.MOD` marker), transcribed
    /// verbatim — this is a literal dump, not an interpretation.
    pub raw_tokens: Vec<RawToken>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CacheRecord {
    pub population: Population,
    pub completeness: Completeness,
    pub ingested_at: String,
    pub data: SpellModAccessData,
    pub source: Source,
    pub wiring_class: String,
    pub wiring_class_signals: Vec<String>,
    pub license: License,
    pub pi_field: Option<String>,
    pub pi_marker: Option<String>,
}

pub(crate) struct BookSpec {
    pub book_id: &'static str,
    pub dir: &'static str,
    pub file: &'static str,
}

/// Mirrors `cache_gen::feat_gap::BOOK_SPECS`'s directory convention
/// exactly (each `dir` is the full corpus-root-relative book directory,
/// each `file` a single `.lst` relative to `dir`).
pub(crate) const BOOK_SPECS: &[BookSpec] = &[
    BookSpec {
        book_id: "occult_adventures",
        dir: "pathfinder/paizo/roleplaying_game/occult_adventures",
        file: "oa_spells.lst",
    },
    BookSpec {
        book_id: "ultimate_magic",
        dir: "pathfinder/paizo/roleplaying_game/ultimate_magic",
        file: "um_spells.lst",
    },
    BookSpec {
        book_id: "advanced_players_guide",
        dir: "pathfinder/paizo/roleplaying_game/advanced_players_guide",
        file: "apg_spells.lst",
    },
];

/// Hoisted to `cache_gen` (R14-04).
use super::sha256_file;

/// One tab-delimited field's `(TOKEN, value)` split, or `None` for a field
/// with no `:` (never occurs on a well-formed PCGen row's non-identity
/// fields, but handled rather than assumed).
fn split_token(field: &str) -> Option<(String, String)> {
    let field = field.trim();
    if field.is_empty() {
        return None;
    }
    field.split_once(':').map(|(k, v)| (k.to_string(), v.to_string()))
}

/// `(key, name, description, raw_tokens, declared_pi)`, `parse_mod_row`'s own return shape.
type ParsedModRow = (String, String, Option<String>, Vec<RawToken>, pi_screening::DeclaredProductIdentity);

/// Parses one `.lst` line already known to be a `.MOD` row carrying a
/// `CLASSES:` token: `(key, name, description, raw_tokens, declared_pi)`.
fn parse_mod_row(line: &str) -> Option<ParsedModRow> {
    let fields: Vec<&str> = line.split('\t').collect();
    let first = fields.first()?.trim();
    if first.is_empty() || !first.ends_with(".MOD") {
        return None;
    }
    let key = first.trim_end_matches(".MOD").trim().to_string();
    if key.is_empty() {
        return None;
    }
    let mut has_classes = false;
    let mut output_name: Option<String> = None;
    let mut description: Option<String> = None;
    let mut raw_tokens = Vec::new();
    let mut declared_tokens: Vec<(String, String)> = Vec::new();
    for field in fields.iter().skip(1) {
        let Some((tok, val)) = split_token(field) else { continue };
        if tok == "CLASSES" {
            has_classes = true;
        }
        if tok == "OUTPUTNAME" {
            output_name = Some(val.clone());
        }
        if tok == "DESC" && val != ".CLEAR" {
            description = Some(val.clone());
        }
        if tok == "NAMEISPI" || tok == "DESCISPI" {
            declared_tokens.push((tok.clone(), val.clone()));
        }
        raw_tokens.push(RawToken { key: tok, value: val });
    }
    if !has_classes {
        return None;
    }
    let declared = declared_product_identity(declared_tokens);
    let name = output_name.unwrap_or_else(|| key.clone());
    Some((key, name, description, raw_tokens, declared))
}

/// Hoisted to `cache_gen` (R14-04) as `slugify_dedup`, imported back
/// under this file's original local name.
use super::slugify_dedup as slugify;

/// Writes `record` to `<out_dir>/<slug>.json`, UNLESS a file already exists
/// there (mirrors every sibling `cache_gen` generator's no-clobber write
/// discipline). Hoisted to `cache_gen` (R14-04) as `write_json_bool`
/// (generalized over any `Serialize` record, not just this file's
/// concrete `CacheRecord`), imported back under this file's original
/// local name.
use super::write_json_bool as write_json;

#[derive(Debug, Default)]
pub struct GenerationReport {
    pub records_written: usize,
    /// Rows whose `key` carries declared or blacklist-matched Product
    /// Identity — honestly not written.
    pub name_pi_excluded: Vec<String>,
    /// Rows whose slugified output path already exists on disk from a
    /// different, already-shipped ingest run — not written.
    pub skipped_pre_existing: Vec<String>,
}

#[derive(Debug)]
pub enum GenerationError {
    CorpusUnreachable(PathBuf),
}

pub fn generate(
    corpus_root: &Path,
    out_root: &Path,
    ingested_at: &str,
) -> Result<GenerationReport, GenerationError> {
    let mut report = GenerationReport::default();

    for spec in BOOK_SPECS {
        let book_dir = corpus_root.join(spec.dir);
        if !book_dir.is_dir() {
            return Err(GenerationError::CorpusUnreachable(book_dir));
        }
        let file_path = book_dir.join(spec.file);
        let text = std::fs::read_to_string(&file_path)
            .map_err(|_| GenerationError::CorpusUnreachable(file_path.clone()))?;
        let sha = sha256_file(&file_path).unwrap_or_default();

        let wiring_index = WiringClassIndex::build(spec.book_id, &book_dir);
        let mut wiring_lines = wiring_index.lines();

        let mut used: BTreeSet<String> = BTreeSet::new();
        for (idx, line) in text.lines().enumerate() {
            let line_no = (idx + 1) as u32;
            let Some((key, name, description, raw_tokens, declared)) = parse_mod_row(line) else {
                continue;
            };

            let (name_license, ..) = pi_screening::classify_field("name", &name);
            if declared.name || name_license == License::PiRedacted {
                report.name_pi_excluded.push(format!("{}:{}", spec.book_id, key));
                continue;
            }

            let (license, pi_field, pi_marker, stored_desc) = pi_screening::classify_optional_field_declared(
                "description",
                description.as_deref(),
                declared.description,
            );

            let completeness = if stored_desc.is_some() { Completeness::Full } else { Completeness::ChassisOnly };

            let (wiring_class, wiring_class_signals) =
                wiring_index.wiring_class_for(&mut wiring_lines, spec.file, line_no, &key, &key);

            let record = CacheRecord {
                population: Population::InScope,
                completeness,
                ingested_at: ingested_at.to_string(),
                data: SpellModAccessData { key: key.clone(), name, description: stored_desc, raw_tokens },
                source: Source::LstToken {
                    path: format!("{}/{}", spec.dir, spec.file),
                    sha256: sha.clone(),
                    line: line_no,
                    record_key: key.clone(),
                },
                wiring_class,
                wiring_class_signals,
                license,
                pi_field,
                pi_marker,
            };

            let slug = slugify(&key, &mut used);
            let write_dir = out_root.join(spec.book_id).join("spell");
            let wrote = write_json(&write_dir, &slug, &record)
                .map_err(|_| GenerationError::CorpusUnreachable(write_dir.clone()))?;
            if !wrote {
                report.skipped_pre_existing.push(format!("{}:{}", spec.book_id, key));
                continue;
            }
            report.records_written += 1;
        }
    }

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    struct Scratch {
        corpus_root: PathBuf,
        out_root: PathBuf,
    }

    impl Scratch {
        fn new(name: &str) -> Self {
            let base =
                std::env::temp_dir().join(format!("codex_spell_mod_access_{name}_{}", std::process::id()));
            let _ = fs::remove_dir_all(&base);
            let corpus_root = base.join("corpus");
            let out_root = base.join("out");
            fs::create_dir_all(&corpus_root).unwrap();
            fs::create_dir_all(&out_root).unwrap();
            Scratch { corpus_root, out_root }
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(self.corpus_root.parent().unwrap());
        }
    }

    /// Writes `contents` for `spec`'s file, and an empty file for every
    /// OTHER `BOOK_SPECS` entry so `generate`'s per-book directory-existence
    /// check does not fail on books this test does not care about.
    fn write_book_file(scratch: &Scratch, spec: &BookSpec, contents: &str) {
        for other in BOOK_SPECS {
            let dir = scratch.corpus_root.join(other.dir);
            fs::create_dir_all(&dir).unwrap();
            let body = if other.book_id == spec.book_id { contents } else { "" };
            fs::write(dir.join(other.file), body).unwrap();
        }
    }

    #[test]
    fn parse_mod_row_rejects_a_non_mod_line() {
        assert!(parse_mod_row("Accelerate Poison\tSCHOOL:Transmutation\tCLASSES:Wizard=2").is_none());
    }

    #[test]
    fn parse_mod_row_rejects_a_mod_row_with_no_classes_token() {
        assert!(parse_mod_row("Greater Teleport (X).MOD\tTYPE:PFSNotLegal\t!PRETYPE:1,PFSNotLegal").is_none());
    }

    #[test]
    fn parse_mod_row_accepts_the_real_occultist_shape() {
        let line = "Occultist Spell ~ Accelerate Poison.MOD\tOUTPUTNAME:Accelerate Poison\tCLASSES:.CLEARALL\tCLASSES:Occultist=2\tPREFACT:1,ABILITIES,OccultistSchool_Transmutation=True";
        let (key, name, desc, raw, declared) = parse_mod_row(line).expect("must parse");
        assert_eq!(key, "Occultist Spell ~ Accelerate Poison");
        assert_eq!(name, "Accelerate Poison");
        assert_eq!(desc, None);
        assert!(raw.iter().any(|t| t.key == "PREFACT"));
        assert!(!declared.any());
    }

    #[test]
    fn parse_mod_row_reads_a_declared_name_pi_token() {
        let line = "Fake Spell.MOD\tOUTPUTNAME:Fake Spell\tCLASSES:Wizard=1\tNAMEISPI:YES";
        let (.., declared) = parse_mod_row(line).expect("must parse");
        assert!(declared.name);
    }

    /// RED -> GREEN mutation proof: dropping the `has_classes` guard would
    /// wrongly accept the `PFSNotLegal` row above, which the module's own
    /// doc comment names as deliberately out of scope.
    #[test]
    fn generation_against_a_scratch_corpus_writes_one_record_and_skips_the_no_classes_row() {
        let scratch = Scratch::new("basic");
        let spec = &BOOK_SPECS[0];
        write_book_file(
            &scratch,
            spec,
            "Occultist Spell ~ Accelerate Poison.MOD\tOUTPUTNAME:Accelerate Poison\tCLASSES:.CLEARALL\tCLASSES:Occultist=2\n\
             Greater Teleport (X).MOD\tTYPE:PFSNotLegal\t!PRETYPE:1,PFSNotLegal\n",
        );
        let report = generate(&scratch.corpus_root, &scratch.out_root, "2026-08-23T00:00:00Z").expect("generate");
        assert_eq!(report.records_written, 1, "only the CLASSES:-bearing MOD row is written");
        let written = scratch
            .out_root
            .join(spec.book_id)
            .join("spell")
            .join("occultist_spell_accelerate_poison.json");
        assert!(written.exists());
    }

    #[test]
    fn a_second_run_does_not_clobber_the_first() {
        let scratch = Scratch::new("no_clobber");
        let spec = &BOOK_SPECS[0];
        write_book_file(
            &scratch,
            spec,
            "Occultist Spell ~ Accelerate Poison.MOD\tOUTPUTNAME:Accelerate Poison\tCLASSES:Occultist=2\n",
        );
        let r1 = generate(&scratch.corpus_root, &scratch.out_root, "2026-08-23T00:00:00Z").expect("generate 1");
        assert_eq!(r1.records_written, 1);
        let r2 = generate(&scratch.corpus_root, &scratch.out_root, "2026-08-23T00:00:00Z").expect("generate 2");
        assert_eq!(r2.records_written, 0);
        assert_eq!(r2.skipped_pre_existing.len(), 1);
    }

    #[test]
    fn generation_against_the_real_pinned_corpus_writes_records() {
        let corpus_root = match std::env::var("PCGEN_CORPUS_ROOT") {
            Ok(v) => PathBuf::from(v),
            Err(_) => {
                eprintln!("skipping live-corpus test: PCGEN_CORPUS_ROOT not set");
                return;
            }
        };
        if !corpus_root.is_dir() {
            eprintln!("skipping live-corpus test: {corpus_root:?} does not exist");
            return;
        }
        let scratch = Scratch::new("live");
        let report = generate(&corpus_root, &scratch.out_root, "2026-08-23T00:00:00Z").expect("generate");
        assert!(report.records_written > 300, "expected >300 records, got {}", report.records_written);
    }
}
