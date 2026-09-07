//! Corpus-wide gate: a shipped `data/corpus/**/*.json` record must never
//! carry a NAME or DESCRIPTION its own real PCGen `.lst` source row
//! declares Product Identity (`NAMEISPI:YES`/`DESCISPI:YES`), and a
//! `LICENSE.json` must never claim the declared-PI reader ran over a
//! writer that does not call it.
//!
//! Built SD31-PI-REPAIR-001 (OPEN-ISSUES rows 38/39). Both defects that
//! motivated this gate were the same shape: a screening contract asserted
//! in prose (a doc comment, a `LICENSE.json` note) and never actually
//! called in code:
//!
//! * `cache_gen::ultimate_equipment.rs` computed `DeclaredProductIdentity
//!   ::name` and never read it, so `Otyugh Hide` (`NAMEISPI:YES`) shipped
//!   its real name unredacted (row 38).
//! * `ingest_races.rs`'s two writers hardcoded `pi_field: None` and never
//!   called `pi_screening::declared_product_identity` at all, while
//!   `data/corpus/bestiary_5/LICENSE.json` claimed they did (row 39).
//!
//! Neither defect was reachable from the corpus's own OGL-hit data (both
//! affected books carried zero `PI_BLACKLIST_TERMS` hits) — only reading
//! the corpus's own `NAMEISPI:`/`DESCISPI:` declaration and cross-checking
//! it against what actually shipped can catch this shape, which is what
//! CHECK A below does. CHECK B closes the second half: a `LICENSE.json`
//! that opts in to a structured, machine-checked claim
//! (`redaction_policy.declared_pi_reader_verified`) is verified against
//! its own named writer source files, rather than trusted as prose.
//!
//! Run via `cargo run --locked --bin declared_pi_shipping_audit`.

use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

use codex::rules_core::pi_screening;
use codex::rules_core::shape_b_v1::REDACTED_PI_MARKER;
use serde_json::Value;

fn pcgen_data_root() -> PathBuf {
    if let Ok(v) = std::env::var("PCGEN_CORPUS_ROOT") {
        return PathBuf::from(v);
    }
    let home = std::env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
    PathBuf::from(home).join("workspace/repos/pcgen/data")
}

fn find_json_files(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if !root.is_dir() {
        return out;
    }
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("json") && path.file_name().and_then(|n| n.to_str()) != Some("LICENSE.json")
            {
                out.push(path);
            }
        }
    }
    out
}

/// A cache of PCGen `.lst` source files, keyed by their corpus-relative
/// path, each read and line-split from disk exactly once. Without this,
/// `declared_at` re-reads and re-splits the SAME source file once per
/// citing record — real corpus files are cited thousands of times each
/// (`acg_abilities_class.lst` alone is cited by 2,687 of the 34k+ shipped
/// records at the current widened population), which turned this stage
/// from a linear scan of ~72MB of unique `.lst` bytes into tens of GB of
/// repeated re-reads and re-splitting, hanging `declared-pi-audit` at
/// 99.9% CPU with no output for minutes. Caching each file's lines once
/// makes the total work proportional to (unique files read once) +
/// (one Vec index per citation) instead of (citations × file size).
type LstFileCache = HashMap<PathBuf, Option<Vec<String>>>;

/// [`pi_screening::declared_product_identity`] read off the real corpus
/// line at `corpus_root/rel_path:line` (1-indexed). `line == 0` or a
/// missing file/line reads as no declaration, matching every generator's
/// own honest-gap handling. `cache` memoizes each source file's lines
/// across every call so a file cited by many records is only ever read
/// and split once — see [`LstFileCache`].
fn declared_at(
    corpus_root: &Path,
    rel_path: &str,
    line: u64,
    cache: &mut LstFileCache,
) -> pi_screening::DeclaredProductIdentity {
    if line == 0 {
        return pi_screening::DeclaredProductIdentity::default();
    }
    let full_path = corpus_root.join(rel_path);
    let lines = cache.entry(full_path.clone()).or_insert_with(|| {
        fs::read_to_string(&full_path)
            .ok()
            .map(|text| text.lines().map(str::to_string).collect())
    });
    let Some(lines) = lines else {
        return pi_screening::DeclaredProductIdentity::default();
    };
    let Some(row) = lines.get((line - 1) as usize) else {
        return pi_screening::DeclaredProductIdentity::default();
    };
    let tokens: Vec<(&str, &str)> = row.split('\t').filter_map(|field| field.split_once(':')).collect();
    pi_screening::declared_product_identity(tokens)
}

#[derive(Debug)]
struct Violation {
    file: String,
    reason: String,
}

/// CHECK A: every shipped record's NAME and DESCRIPTION cross-checked
/// against its own cited corpus row's declaration.
fn audit_shipped_records(corpus_root: &Path, data_corpus_root: &Path) -> Vec<Violation> {
    let mut violations = Vec::new();
    let mut lst_cache: LstFileCache = HashMap::new();
    for path in find_json_files(data_corpus_root) {
        let Ok(text) = fs::read_to_string(&path) else { continue };
        let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
        let source = doc.get("source").cloned().unwrap_or(Value::Null);
        if source.get("kind").and_then(Value::as_str) != Some("lst_token") {
            continue;
        }
        let Some(rel_path) = source.get("path").and_then(Value::as_str) else { continue };
        let line = source.get("line").and_then(Value::as_u64).unwrap_or(0);
        let declared = declared_at(corpus_root, rel_path, line, &mut lst_cache);
        let file_str = path.display().to_string();

        // SD-32 `decisions.md §24`: a record whose cited row declares
        // `NAMEISPI:YES` may now ship under a Codex-generated neutral name
        // derived ONLY from `(kind, book, source_file, source_line)`
        // (`scripts/codex_neutral_name.py`) -- the record's own
        // `codex_generated_name: true` field is the visible marker
        // `§24b`-3 requires, and it is what this check trusts to
        // distinguish that shippable case from the pre-`§24` violation
        // (the original PI name shipped verbatim). This does not weaken
        // the check for any record NOT carrying that field: an ordinary
        // record whose cited row declares `NAMEISPI:YES` is still a
        // violation exactly as before.
        let codex_generated_name =
            doc.get("codex_generated_name").and_then(Value::as_bool).unwrap_or(false);

        if declared.name && !codex_generated_name {
            // A key/name cannot be redacted (it is the record's identity
            // on every screen and half of its key) -- its mere presence
            // on disk IS the violation, per `SD-29-corpus-wide-catch-up-
            // lanes/decisions.md §50.3`.
            violations.push(Violation {
                file: file_str.clone(),
                reason: format!("NAME-PI-SHIPPED: cites {rel_path}:{line} (NAMEISPI:YES) but exists on disk"),
            });
        }

        let desc = doc.get("data").and_then(|d| d.get("description")).and_then(Value::as_str);
        let license = doc.get("license").and_then(Value::as_str);
        let pi_field = doc.get("pi_field").and_then(Value::as_str);
        let redacted_desc = desc == Some(REDACTED_PI_MARKER);
        let redacted_license = license == Some("PI-REDACTED");
        // SD-32 `decisions.md §24`'s renamed records stamp `pi_field` as a
        // comma-joined list (e.g. `"description,name,raw_tokens"`) when
        // more than one field was redacted -- `pi_field` naming
        // "description" among possibly others still means the description
        // itself is accounted for, so this checks list membership rather
        // than exact equality. A record with only ever one redacted field
        // (every pre-`§24` generator) keeps matching exactly as before.
        let redacted_field =
            pi_field.map(|f| f.split(',').any(|part| part == "description")).unwrap_or(false);

        if declared.description && !(redacted_desc && redacted_license && redacted_field) {
            violations.push(Violation {
                file: file_str.clone(),
                reason: format!(
                    "DESC-PI-SHIPPED: cites {rel_path}:{line} (DESCISPI:YES) but data.description={desc:?} \
                     license={license:?} pi_field={pi_field:?} (expected description=\"[redacted PI]\", \
                     license=\"PI-REDACTED\", pi_field=\"description\")"
                ),
            });
        }

        // `SD31-W4-INTEGRATE-001` (`OPEN-ISSUES.md` row 48/49): the FIRST
        // version of this check inspected `data.description` only, and
        // only fired when the CORPUS ROW ITSELF declares `DESCISPI:YES`.
        // Neither restriction is safe: `data.description` being correctly
        // redacted does not mean the record is safe -- `data.raw_tokens`
        // can still hold the original `DESC:` token verbatim, since every
        // redaction call site writes the marker into `description` but
        // never touches `raw_tokens`. And a description can be, and
        // routinely is, redacted for a reason `declared_at` cannot see at
        // all: the separate `§52.3` blacklist term scan
        // (`pi_screening::classify_field`), which fires on prose the
        // corpus row never declared PI at all. 413 shipped records were
        // exposed this way (367 declared, 46 blacklist-only) before this
        // extension existed. So this check runs over EVERY record this
        // repo has already marked `license: "PI-REDACTED"`,
        // `pi_field: "description"` -- regardless of which screen
        // triggered the redaction -- not only the `declared.description`
        // subset.
        if redacted_license && redacted_field
            && let Some(raw_tokens) = doc.get("data").and_then(|d| d.get("raw_tokens")).and_then(Value::as_array) {
                for token in raw_tokens {
                    if token.get("key").and_then(Value::as_str) != Some("DESC") {
                        continue;
                    }
                    let value = token.get("value").and_then(Value::as_str);
                    if value != Some(REDACTED_PI_MARKER) {
                        violations.push(Violation {
                            file: file_str.clone(),
                            reason: format!(
                                "DESC-PI-SHIPPED-IN-RAW-TOKENS: record is license=\"PI-REDACTED\" \
                                 pi_field=\"description\" (cites {rel_path}:{line}) but data.raw_tokens \
                                 carries a DESC entry whose value is {value:?}, not the redaction marker \
                                 -- data.description alone being redacted does not close this leak"
                            ),
                        });
                    }
                }
            }
    }
    violations
}

/// SD-32 card 11, T9-onboarding-class-feature-pi-and-rescreen (`decisions.md
/// §19`/`§26`): the two coordinates below are CONFIRMED, verified false
/// positives of the OCR-fold canonicalization — the blacklisted place name
/// "Galt" folds `l`→`i` to "gait", which collides with the ordinary English
/// word "gait" ("Steady Gait", "Seadog's Gait", "...his gait more
/// deliberate..."). Confirmed by direct canonicalization
/// (`pi_scrub.canonicalize("Galt") == pi_scrub.canonicalize("gait")`) and by
/// reading each record's real, non-Golarion-referencing prose — named by the
/// `feat`-lane PI-leak receipt (`sd32-pi-leak-screening-path-inner-sea-
/// combat-feat_cycle-1_cycle_receipt.md`) and re-confirmed by this cycle's
/// own corpus-wide re-derivation. This is a coordinate-scoped exemption on
/// CHECK C only — narrower than a term-wide fold change (which is `§26`'s
/// own open, not-yet-closed territory) — so a genuine future leak on ANY
/// OTHER record is still caught. Never widen this list without the SAME
/// direct-canonicalization + real-prose proof these three already have.
const KNOWN_OCR_FOLD_FALSE_POSITIVES: &[&str] = &[
    "advanced_players_guide/class_feature/shifter_s_blessing/form_of_the_cat.json",
    "advanced_race_guide/class_feature/buccaneer/seadog_s_gait.json",
    "horror_adventures/class_feature/dreadnought/steady_gait.json",
];

/// Every string reachable under `value` (dicts/lists walked recursively),
/// paired with a dotted field-path for reporting — mirrors
/// `scripts/sd32_t9_corpus_wide_pi_rescan.py::iter_strings` exactly, so the
/// Rust gate and the Python re-derivation this cycle ran agree on what
/// "every shipped field" means.
fn iter_strings<'a>(value: &'a Value, path: String, out: &mut Vec<(String, &'a str)>) {
    match value {
        Value::String(s) => out.push((path, s.as_str())),
        Value::Object(map) => {
            for (k, v) in map {
                let child_path = if path.is_empty() { k.clone() } else { format!("{path}.{k}") };
                iter_strings(v, child_path, out);
            }
        }
        Value::Array(items) => {
            for (i, v) in items.iter().enumerate() {
                iter_strings(v, format!("{path}[{i}]"), out);
            }
        }
        _ => {}
    }
}

/// CHECK C: every string reachable under EVERY shipped record's `data`
/// object — not only `name`/`description` — re-scanned against the CURRENT
/// 61-term blacklist (`pi_screening::blacklist_term_hit_including_concatenated`,
/// the same word-bounded, OCR-normalized, concatenated-identifier scan
/// `pi_scrub.py`/`scrub_name_pi_tokens` already use), regardless of which
/// generator wrote the record or when.
///
/// **Why this check exists, generically, rather than per-kind:** two
/// independent defects were found live corpus-wide this cycle
/// (`decisions.md §17a` re-derivation): (a) several generators screen only
/// a chosen subset of fields (`name`/`description`) and never screen others
/// they also ship verbatim (`raw_tokens`, `prerequisites`, and — the defect
/// this cycle's own fix in `cache_gen::class_feature.rs` closes — `key`/
/// `class`); (b) EVERY generator's `write_json` is no-clobber, so a record
/// written before a blacklist term existed is NEVER re-screened once that
/// term is added — `ogl-pi-blacklist.md` has been amended at least four
/// times in this bundle. This check does not care which defect produced a
/// leak or which generator owns the record: it re-derives PI-safety from
/// the CURRENT blacklist against the CURRENT shipped bytes, corpus-wide,
/// every time it runs — so a defect of either shape re-opening (a new
/// generator gap, or a future blacklist amendment left unapplied) fails
/// this gate rather than shipping silently. A value already equal to the
/// redaction marker is not a leak (it IS the marker) and is skipped.
fn audit_blacklist_term_hits(data_corpus_root: &Path) -> Vec<Violation> {
    let mut violations = Vec::new();
    for path in find_json_files(data_corpus_root) {
        let Ok(text) = fs::read_to_string(&path) else { continue };
        let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
        let Some(data) = doc.get("data") else { continue };
        let file_str = path.display().to_string();
        let exempt = KNOWN_OCR_FOLD_FALSE_POSITIVES.iter().any(|suffix| file_str.ends_with(suffix));

        let mut strings = Vec::new();
        iter_strings(data, String::new(), &mut strings);
        for (field_path, s) in strings {
            if s.is_empty() || s == REDACTED_PI_MARKER {
                continue;
            }
            if pi_screening::blacklist_term_hit_including_concatenated(s).is_some() {
                if exempt {
                    continue;
                }
                violations.push(Violation {
                    file: file_str.clone(),
                    reason: format!(
                        "BLACKLIST-TERM-SHIPPED: data.{field_path} carries a live (non-redacted) blacklist \
                         term hit — either a generator that never screens this field, or a record written \
                         before this term was added to the blacklist and never re-screened since \
                         (`decisions.md §19`'s amendments)"
                    ),
                });
            }
        }
    }
    violations
}

/// CHECK B: a `LICENSE.json` that opts in to
/// `redaction_policy.declared_pi_reader_verified: true` must name writer
/// source files that actually call the declared-PI reader — a structured,
/// machine-checked replacement for the free-text claim OPEN-ISSUES row 39
/// found unenforced.
fn audit_license_claims(repo_root: &Path, data_corpus_root: &Path) -> Vec<Violation> {
    let mut violations = Vec::new();
    let mut stack = vec![data_corpus_root.to_path_buf()];
    let mut license_files = Vec::new();
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.file_name().and_then(|n| n.to_str()) == Some("LICENSE.json") {
                license_files.push(path);
            }
        }
    }
    license_files.sort();

    for path in license_files {
        let Ok(text) = fs::read_to_string(&path) else { continue };
        let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
        let verified = doc
            .get("redaction_policy")
            .and_then(|r| r.get("declared_pi_reader_verified"))
            .and_then(Value::as_bool)
            .unwrap_or(false);
        if !verified {
            continue;
        }
        let writers: Vec<String> = doc
            .get("redaction_policy")
            .and_then(|r| r.get("declared_pi_reader_writers"))
            .and_then(Value::as_array)
            .map(|a| a.iter().filter_map(|v| v.as_str().map(str::to_string)).collect())
            .unwrap_or_default();
        if writers.is_empty() {
            violations.push(Violation {
                file: path.display().to_string(),
                reason: "LICENSE-CLAIM-UNVERIFIED: declared_pi_reader_verified=true but \
                         declared_pi_reader_writers is empty -- nothing to check the claim against"
                    .to_string(),
            });
            continue;
        }
        for writer in &writers {
            let writer_path = repo_root.join(writer);
            let ok = fs::read_to_string(&writer_path)
                .map(|src| src.contains("declared_product_identity"))
                .unwrap_or(false);
            if !ok {
                violations.push(Violation {
                    file: path.display().to_string(),
                    reason: format!(
                        "LICENSE-CLAIM-UNVERIFIED: declared_pi_reader_verified=true names {writer} \
                         as a writer, but that file does not call `declared_product_identity` \
                         (checked {writer_path:?})"
                    ),
                });
            }
        }
    }
    violations
}

fn main() {
    let corpus_root = pcgen_data_root();
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let data_corpus_root = repo_root.join("data/corpus");

    let mut violations = audit_shipped_records(&corpus_root, &data_corpus_root);
    violations.extend(audit_license_claims(&repo_root, &data_corpus_root));
    violations.extend(audit_blacklist_term_hits(&data_corpus_root));

    if violations.is_empty() {
        println!("declared-pi-audit: CLEAN — no shipped record contradicts its own corpus row's PI declaration");
        return;
    }

    let files: BTreeSet<&str> = violations.iter().map(|v| v.file.as_str()).collect();
    println!("declared-pi-audit: FAIL — {} violation(s) across {} file(s)", violations.len(), files.len());
    for v in &violations {
        println!("  {}: {}", v.file, v.reason);
    }
    std::process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Scratch {
        root: PathBuf,
    }

    impl Scratch {
        fn new(name: &str) -> Self {
            let root = std::env::temp_dir().join(format!("codex_declared_pi_audit_test_{name}_{}", std::process::id()));
            let _ = fs::remove_dir_all(&root);
            fs::create_dir_all(&root).unwrap();
            Scratch { root }
        }
        fn write(&self, rel: &str, contents: &str) {
            let path = self.root.join(rel);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, contents).unwrap();
        }
    }
    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    fn record_json(name: &str, description: &str, license: &str, pi_field: &str, source_path: &str, line: u32) -> String {
        format!(
            r#"{{"data":{{"key":"{name}","name":"{name}","description":"{description}"}},
                "source":{{"kind":"lst_token","path":"{source_path}","line":{line},"record_key":"{name}"}},
                "license":"{license}","pi_field":"{pi_field}"}}"#
        )
    }

    // --- CHECK A mutation proof: NAME-PI-SHIPPED -----------------------

    #[test]
    fn a_shipped_record_whose_corpus_row_declares_nameispi_is_a_violation() {
        let s = Scratch::new("name_pi");
        s.write("pcgen/some_book/rows.lst", "Otyugh Hide\tNAMEISPI:YES\tCOST:1415\n");
        s.write(
            "corpus/some_book/equipment/otyugh_hide.json",
            &record_json("Otyugh Hide", "null", "OGL", "null", "some_book/rows.lst", 1),
        );
        let violations = audit_shipped_records(&s.root.join("pcgen"), &s.root.join("corpus"));
        assert_eq!(violations.len(), 1, "the mutation must be caught: {violations:?}");
        assert!(violations[0].reason.contains("NAME-PI-SHIPPED"));
    }

    // --- SD-32 decisions.md §24: codex_generated_name exception ---------

    fn renamed_record_json(codex_name: &str, source_path: &str, line: u32) -> String {
        format!(
            r#"{{"data":{{"key":"{codex_name}","name":"{codex_name}","description":null}},
                "source":{{"kind":"lst_token","path":"{source_path}","line":{line},"record_key":"{codex_name}"}},
                "license":"OGL","pi_field":null,"codex_generated_name":true}}"#
        )
    }

    #[test]
    fn a_record_marked_codex_generated_name_is_not_a_name_pi_violation() {
        // decisions.md §24: a record whose cited row declares NAMEISPI:YES
        // may ship under a Codex-generated neutral name -- the
        // `codex_generated_name: true` field is what distinguishes this
        // from the pre-§24 violation this check otherwise still catches.
        let s = Scratch::new("codex_generated_name_exempt");
        s.write("pcgen/some_book/rows.lst", "Otyugh Hide\tNAMEISPI:YES\tCOST:1415\n");
        s.write(
            "corpus/some_book/ability/codex_named_unit.json",
            &renamed_record_json("Codex-Named Unit (ability_some_book_rows_lst_1)", "some_book/rows.lst", 1),
        );
        let violations = audit_shipped_records(&s.root.join("pcgen"), &s.root.join("corpus"));
        assert!(violations.is_empty(), "a properly-marked renamed record must not be flagged: {violations:?}");
    }

    #[test]
    fn a_record_shipping_the_original_name_without_the_marker_is_still_a_violation() {
        // The exception must be narrow: a record citing a NAMEISPI:YES row
        // that does NOT carry `codex_generated_name: true` is still caught
        // exactly as before -- this is the guard against the exception
        // silently swallowing the pre-§24 defect shape.
        let s = Scratch::new("codex_generated_name_not_exempt_without_marker");
        s.write("pcgen/some_book/rows.lst", "Otyugh Hide\tNAMEISPI:YES\tCOST:1415\n");
        s.write(
            "corpus/some_book/equipment/otyugh_hide.json",
            &record_json("Otyugh Hide", "null", "OGL", "null", "some_book/rows.lst", 1),
        );
        let violations = audit_shipped_records(&s.root.join("pcgen"), &s.root.join("corpus"));
        assert_eq!(violations.len(), 1, "must still be caught without the marker: {violations:?}");
        assert!(violations[0].reason.contains("NAME-PI-SHIPPED"));
    }

    #[test]
    fn a_multi_field_pi_field_list_including_description_still_clears_the_desc_check() {
        // decisions.md §24's renamed records stamp a comma-joined
        // `pi_field` (e.g. "description,name,raw_tokens") when more than
        // one field was redacted. The description check must still
        // recognise "description" as a member of that list.
        let s = Scratch::new("multi_field_pi_field");
        s.write("pcgen/some_book/rows.lst", "Kodar Trait\tDESCISPI:YES\tDESC:Named after the Kodar Mountains.\n");
        s.write(
            "corpus/some_book/ability/codex_named_unit.json",
            &record_json(
                "Codex-Named Unit (x)",
                "[redacted PI]",
                "PI-REDACTED",
                "description,name,raw_tokens",
                "some_book/rows.lst",
                1,
            ),
        );
        let violations = audit_shipped_records(&s.root.join("pcgen"), &s.root.join("corpus"));
        assert!(violations.is_empty(), "{violations:?}");
    }

    #[test]
    fn dropping_the_record_clears_the_violation() {
        // Same fixture as above, minus the JSON file -- the fix's actual
        // shape (drop the row, don't ship it under any name).
        let s = Scratch::new("name_pi_dropped");
        s.write("pcgen/some_book/rows.lst", "Otyugh Hide\tNAMEISPI:YES\tCOST:1415\n");
        let violations = audit_shipped_records(&s.root.join("pcgen"), &s.root.join("corpus"));
        assert!(violations.is_empty());
    }

    // --- CHECK A mutation proof: DESC-PI-SHIPPED ------------------------

    #[test]
    fn a_shipped_description_whose_corpus_row_declares_descispi_but_ships_unredacted_is_a_violation() {
        let s = Scratch::new("desc_pi");
        s.write("pcgen/some_book/rows.lst", "Kodar Trait\tDESCISPI:YES\tDESC:Named after the Kodar Mountains.\n");
        s.write(
            "corpus/some_book/race_trait/kodar.json",
            &record_json(
                "Kodar Trait",
                "Named after the Kodar Mountains.",
                "OGL",
                "null",
                "some_book/rows.lst",
                1,
            ),
        );
        let violations = audit_shipped_records(&s.root.join("pcgen"), &s.root.join("corpus"));
        assert_eq!(violations.len(), 1, "the mutation must be caught: {violations:?}");
        assert!(violations[0].reason.contains("DESC-PI-SHIPPED"));
    }

    #[test]
    fn a_correctly_redacted_description_is_not_a_violation() {
        let s = Scratch::new("desc_pi_clean");
        s.write("pcgen/some_book/rows.lst", "Kodar Trait\tDESCISPI:YES\tDESC:Named after the Kodar Mountains.\n");
        s.write(
            "corpus/some_book/race_trait/kodar.json",
            &record_json("Kodar Trait", "[redacted PI]", "PI-REDACTED", "description", "some_book/rows.lst", 1),
        );
        let violations = audit_shipped_records(&s.root.join("pcgen"), &s.root.join("corpus"));
        assert!(violations.is_empty(), "{violations:?}");
    }

    // --- CHECK A mutation proof: DESC-PI-SHIPPED-IN-RAW-TOKENS ----------

    fn record_json_with_raw_tokens(
        name: &str,
        description: &str,
        license: &str,
        pi_field: &str,
        source_path: &str,
        line: u32,
        raw_desc_value: &str,
    ) -> String {
        format!(
            r#"{{"data":{{"key":"{name}","name":"{name}","description":"{description}",
                "raw_tokens":[{{"key":"DESC","value":"{raw_desc_value}"}}]}},
                "source":{{"kind":"lst_token","path":"{source_path}","line":{line},"record_key":"{name}"}},
                "license":"{license}","pi_field":"{pi_field}"}}"#
        )
    }

    /// The exact hole this extension closes: `data.description` is
    /// correctly redacted, but `data.raw_tokens` still carries the
    /// original declared-PI prose verbatim.
    #[test]
    fn a_correctly_redacted_description_with_a_leaking_raw_token_is_a_violation() {
        let s = Scratch::new("desc_pi_raw_leak_declared");
        s.write("pcgen/some_book/rows.lst", "Kodar Trait\tDESCISPI:YES\tDESC:Named after the Kodar Mountains.\n");
        s.write(
            "corpus/some_book/race_trait/kodar.json",
            &record_json_with_raw_tokens(
                "Kodar Trait",
                "[redacted PI]",
                "PI-REDACTED",
                "description",
                "some_book/rows.lst",
                1,
                "Named after the Kodar Mountains.",
            ),
        );
        let violations = audit_shipped_records(&s.root.join("pcgen"), &s.root.join("corpus"));
        assert_eq!(violations.len(), 1, "the raw_tokens leak must be caught: {violations:?}");
        assert!(violations[0].reason.contains("DESC-PI-SHIPPED-IN-RAW-TOKENS"));
    }

    /// The same leak shape, but triggered by the `§52.3` blacklist term
    /// scan rather than a `DESCISPI:YES` declaration -- `declared_at`
    /// cannot see this trigger at all, so the check must run over every
    /// `license: "PI-REDACTED"` record, not only ones `declared.description`
    /// flags.
    #[test]
    fn a_blacklist_only_redaction_with_a_leaking_raw_token_is_also_caught() {
        let s = Scratch::new("desc_pi_raw_leak_blacklist");
        // No DESCISPI:YES declaration on this row at all.
        s.write("pcgen/some_book/rows.lst", "Jarn's Ward\tDESC:Named for the sage Jarn.\n");
        s.write(
            "corpus/some_book/spell/jarns_ward.json",
            &record_json_with_raw_tokens(
                "Jarn's Ward",
                "[redacted PI]",
                "PI-REDACTED",
                "description",
                "some_book/rows.lst",
                1,
                "Named for the sage Jarn.",
            ),
        );
        let violations = audit_shipped_records(&s.root.join("pcgen"), &s.root.join("corpus"));
        assert_eq!(violations.len(), 1, "{violations:?}");
        assert!(violations[0].reason.contains("DESC-PI-SHIPPED-IN-RAW-TOKENS"));
    }

    /// A correctly redacted raw_tokens DESC entry (the post-fix shape) is
    /// not a violation.
    #[test]
    fn a_raw_tokens_desc_entry_that_is_also_redacted_is_not_a_violation() {
        let s = Scratch::new("desc_pi_raw_clean");
        s.write("pcgen/some_book/rows.lst", "Kodar Trait\tDESCISPI:YES\tDESC:Named after the Kodar Mountains.\n");
        s.write(
            "corpus/some_book/race_trait/kodar.json",
            &record_json_with_raw_tokens(
                "Kodar Trait",
                "[redacted PI]",
                "PI-REDACTED",
                "description",
                "some_book/rows.lst",
                1,
                "[redacted PI]",
            ),
        );
        let violations = audit_shipped_records(&s.root.join("pcgen"), &s.root.join("corpus"));
        assert!(violations.is_empty(), "{violations:?}");
    }

    #[test]
    fn an_undeclared_record_is_never_flagged() {
        let s = Scratch::new("undeclared");
        s.write("pcgen/some_book/rows.lst", "Plain Item\tCOST:5\n");
        s.write(
            "corpus/some_book/equipment/plain_item.json",
            &record_json("Plain Item", "null", "OGL", "null", "some_book/rows.lst", 1),
        );
        let violations = audit_shipped_records(&s.root.join("pcgen"), &s.root.join("corpus"));
        assert!(violations.is_empty());
    }

    // --- CHECK B mutation proof: LICENSE-CLAIM-UNVERIFIED ---------------

    #[test]
    fn a_license_claim_naming_a_writer_that_does_not_call_the_reader_is_a_violation() {
        let s = Scratch::new("license_unwired");
        s.write(
            "corpus/some_book/LICENSE.json",
            r#"{"redaction_policy":{"declared_pi_reader_verified":true,"declared_pi_reader_writers":["src/bin/fake_writer.rs"]}}"#,
        );
        s.write("src/bin/fake_writer.rs", "fn main() { /* pi_field: None, never screened */ }");
        let violations = audit_license_claims(&s.root, &s.root.join("corpus"));
        assert_eq!(violations.len(), 1, "{violations:?}");
        assert!(violations[0].reason.contains("LICENSE-CLAIM-UNVERIFIED"));
    }

    #[test]
    fn a_license_claim_naming_a_writer_that_does_call_the_reader_is_clean() {
        let s = Scratch::new("license_wired");
        s.write(
            "corpus/some_book/LICENSE.json",
            r#"{"redaction_policy":{"declared_pi_reader_verified":true,"declared_pi_reader_writers":["src/bin/real_writer.rs"]}}"#,
        );
        s.write(
            "src/bin/real_writer.rs",
            "fn main() { let d = pi_screening::declared_product_identity(tokens); }",
        );
        let violations = audit_license_claims(&s.root, &s.root.join("corpus"));
        assert!(violations.is_empty(), "{violations:?}");
    }

    #[test]
    fn an_unopted_in_license_json_is_never_flagged() {
        let s = Scratch::new("license_no_claim");
        s.write(
            "corpus/some_book/LICENSE.json",
            r#"{"screening_method_note":"free text that mentions the declared-PI reader but opts into nothing structured"}"#,
        );
        let violations = audit_license_claims(&s.root, &s.root.join("corpus"));
        assert!(violations.is_empty(), "an un-opted-in LICENSE.json must not be flagged: {violations:?}");
    }

    // --- CHECK C mutation proof: BLACKLIST-TERM-SHIPPED ------------------
    // SD-32 card 11, T9-onboarding-class-feature-pi-and-rescreen: "make the
    // gap impossible to reopen" -- a record predating a blacklist term (or
    // written by a generator that never screened its field at all) fails
    // this gate regardless of which of the two defect shapes produced it.

    #[test]
    fn a_key_field_carrying_a_live_blacklist_term_is_a_violation() {
        // The exact shape this cycle found live: `data.key` (not `name` or
        // `description`) carries the term, unredacted, unmarked.
        let s = Scratch::new("blacklist_key_leak");
        s.write(
            "corpus/some_book/class_feature/x/x.json",
            r#"{"data":{"key":"Lunatic's Gift ~ Lamashtu","name":"Lunatic's Gift","description":null},
                "source":{"kind":"lst_token","path":"some_book/rows.lst","line":1,"record_key":"x"},
                "license":"OGL","pi_field":null}"#,
        );
        let violations = audit_blacklist_term_hits(&s.root.join("corpus"));
        assert_eq!(violations.len(), 1, "{violations:?}");
        assert!(violations[0].reason.contains("BLACKLIST-TERM-SHIPPED"));
        assert!(violations[0].reason.contains("data.key"), "{violations:?}");
    }

    #[test]
    fn a_prerequisites_or_raw_tokens_field_carrying_a_live_blacklist_term_is_a_violation() {
        // The two ALREADY-FIXED generator gaps this bundle named
        // (`feat_gap.rs`'s `prerequisites`, `class_feature.rs`'s
        // `raw_tokens`) -- proves this generic, field-name-agnostic check
        // would have caught either shape, not just the specific field this
        // cycle's own fix targets.
        let s = Scratch::new("blacklist_nested_leak");
        s.write(
            "corpus/some_book/feat/x.json",
            r#"{"data":{"key":"x","name":"x","description":null,
                "prerequisites":["PREFEAT:Devotee of Iomedae"]},
                "source":{"kind":"lst_token","path":"some_book/rows.lst","line":1,"record_key":"x"},
                "license":"OGL","pi_field":null}"#,
        );
        let violations = audit_blacklist_term_hits(&s.root.join("corpus"));
        assert_eq!(violations.len(), 1, "{violations:?}");
        assert!(violations[0].reason.contains("data.prerequisites[0]"), "{violations:?}");
    }

    #[test]
    fn a_redaction_marker_value_is_never_flagged() {
        let s = Scratch::new("blacklist_marker_clean");
        s.write(
            "corpus/some_book/class_feature/x/x.json",
            r#"{"data":{"key":"[redacted PI]","name":"x","description":"[redacted PI]"},
                "source":{"kind":"lst_token","path":"some_book/rows.lst","line":1,"record_key":"x"},
                "license":"PI-REDACTED","pi_field":"key,description"}"#,
        );
        let violations = audit_blacklist_term_hits(&s.root.join("corpus"));
        assert!(violations.is_empty(), "the marker itself must never be flagged as a leak: {violations:?}");
    }

    #[test]
    fn a_known_ocr_fold_false_positive_coordinate_is_exempted_but_only_that_exact_file() {
        // decisions.md §26: "Galt" folds to "gait" and collides with the
        // ordinary word -- the three NAMED coordinates are exempt, but an
        // otherwise-identical leak at a DIFFERENT path must still be caught
        // (the exemption is coordinate-scoped, never term-wide).
        let s = Scratch::new("gait_exempt_scoped");
        s.write(
            "corpus/horror_adventures/class_feature/dreadnought/steady_gait.json",
            r#"{"data":{"key":"Steady Gait","name":"Steady Gait","description":"His gait grows steadier."},
                "source":{"kind":"lst_token","path":"horror_adventures/rows.lst","line":1,"record_key":"x"},
                "license":"OGL","pi_field":null}"#,
        );
        s.write(
            "corpus/some_other_book/class_feature/y/y.json",
            r#"{"data":{"key":"y","name":"y","description":"A steady gait, unlike Galt's own turmoil."},
                "source":{"kind":"lst_token","path":"some_other_book/rows.lst","line":1,"record_key":"y"},
                "license":"OGL","pi_field":null}"#,
        );
        let violations = audit_blacklist_term_hits(&s.root.join("corpus"));
        assert_eq!(violations.len(), 1, "only the NON-exempt coordinate must be flagged: {violations:?}");
        assert!(violations[0].file.ends_with("some_other_book/class_feature/y/y.json"), "{violations:?}");
    }

    #[test]
    fn a_record_written_before_the_term_existed_is_still_caught_by_a_fresh_scan() {
        // Simulates defect (b): a record on disk today, never touched
        // since it was first written, whose content now matches the
        // CURRENT (grown) blacklist -- this check has no concept of
        // "when was this file written", so it catches it on every run,
        // regardless of generator or write timestamp.
        let s = Scratch::new("blacklist_predates_term");
        s.write(
            "corpus/adventurers_guide/class_feature/aldori_swordlord/x.json",
            r#"{"data":{"key":"Combat Feat ~ Aldori Swordlord","name":"Combat Feat","description":null},
                "source":{"kind":"lst_token","path":"adventurers_guide/rows.lst","line":1,"record_key":"x"},
                "license":"OGL","pi_field":null,"ingested_at":"2020-01-01T00:00:00Z"}"#,
        );
        let violations = audit_blacklist_term_hits(&s.root.join("corpus"));
        assert_eq!(violations.len(), 1, "{violations:?}");
    }

    /// t9-onboarding-pi-final-leaks-and-generators cycle: requirement 3
    /// ("a test that fails when a generator writes a field it does not
    /// screen"). CHECK C is deliberately generator-agnostic and
    /// field-name-agnostic (its own doc comment above), so it is the
    /// closest enforceable equivalent -- it does not know or care that
    /// `cache_gen::{acg,apg,beastiary1}` never called a `name`-screening
    /// function; it only re-derives PI-safety from the CURRENT shipped
    /// bytes. This test proves that generality against the EXACT shape
    /// this cycle's own three generators write for an `equipment` record
    /// (`key`+`name`+`category`+`cost_gp`+`weight`+`description`, no
    /// `raw_tokens`) -- if any of the three (or a fourth, not-yet-found
    /// generator of the same shape) ever ships a `name` carrying a live
    /// blacklist term with no marker, this exact fixture shape fails.
    ///
    /// **What this does NOT cover** (`AGENTS.md` non-negotiable rule 7):
    /// CHECK C only fires on records that reach `data/corpus/**` on disk --
    /// it is a shipping-time gate, not a compile-time or generation-time
    /// one. A generator that never runs at all (dead code) or a field this
    /// walker's `iter_strings` cannot reach (a non-string leaf, e.g. a
    /// numeric or boolean field a future PI leak somehow encoded into)
    /// would not be caught by this specific test or by CHECK C itself.
    #[test]
    fn an_equipment_name_field_carrying_a_live_blacklist_term_with_no_marker_is_a_violation() {
        // Mirrors `cache_gen::acg`/`apg`/`beastiary1`'s real `EquipmentData`
        // shape exactly -- `key`+`name`+`category`+`cost_gp`+`weight`+
        // `description`, no `raw_tokens` -- proving CHECK C catches THIS
        // shape specifically, not just the `class_feature`/`feat` shapes
        // the other CHECK C tests already cover.
        let s = Scratch::new("equipment_name_leak");
        let term = pi_screening::PI_BLACKLIST_TERMS[0];
        s.write(
            "corpus/some_book/equipment/x.json",
            &format!(
                r#"{{"data":{{"key":"x","name":"{term}'s Holy Symbol","category":"General","cost_gp":50.0,"weight":1.0,"description":null}},
                "source":{{"kind":"lst_token","path":"some_book/rows.lst","line":1,"record_key":"x"}},
                "license":"OGL","pi_field":null,"codex_generated_name":false}}"#
            ),
        );
        let violations = audit_blacklist_term_hits(&s.root.join("corpus"));
        assert_eq!(violations.len(), 1, "{violations:?}");
        assert!(violations[0].reason.contains("data.name"), "{violations:?}");
    }

    /// Mutation proof (`§1a`: a gate that cannot fail is worse than no
    /// gate): a correctly-redacted equipment `name` (the marker in place,
    /// as `cache_gen::{acg,apg,beastiary1}`'s new `§24` rename path now
    /// produces) must NOT trip the same check -- proving the previous test
    /// fails for the PRESENCE of a live term, not merely for the SHAPE of
    /// an equipment record.
    #[test]
    fn a_properly_redacted_equipment_name_is_never_flagged() {
        let s = Scratch::new("equipment_name_clean");
        s.write(
            "corpus/some_book/equipment/x.json",
            r#"{"data":{"key":"Codex-Named Unit (equipment_some_book_rows_lst_1)","name":"Codex-Named Unit (equipment_some_book_rows_lst_1)","category":"General","cost_gp":50.0,"weight":1.0,"description":null},
                "source":{"kind":"lst_token","path":"some_book/rows.lst","line":1,"record_key":"x"},
                "license":"PI-REDACTED","pi_field":"name","codex_generated_name":true}"#,
        );
        let violations = audit_blacklist_term_hits(&s.root.join("corpus"));
        assert!(violations.is_empty(), "{violations:?}");
    }
}
