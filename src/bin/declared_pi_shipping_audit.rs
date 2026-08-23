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

use std::collections::BTreeSet;
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

/// [`pi_screening::declared_product_identity`] read off the real corpus
/// line at `corpus_root/rel_path:line` (1-indexed). `line == 0` or a
/// missing file/line reads as no declaration, matching every generator's
/// own honest-gap handling.
fn declared_at(corpus_root: &Path, rel_path: &str, line: u64) -> pi_screening::DeclaredProductIdentity {
    if line == 0 {
        return pi_screening::DeclaredProductIdentity::default();
    }
    let Ok(text) = fs::read_to_string(corpus_root.join(rel_path)) else {
        return pi_screening::DeclaredProductIdentity::default();
    };
    let Some(row) = text.lines().nth((line - 1) as usize) else {
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
    for path in find_json_files(data_corpus_root) {
        let Ok(text) = fs::read_to_string(&path) else { continue };
        let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
        let source = doc.get("source").cloned().unwrap_or(Value::Null);
        if source.get("kind").and_then(Value::as_str) != Some("lst_token") {
            continue;
        }
        let Some(rel_path) = source.get("path").and_then(Value::as_str) else { continue };
        let line = source.get("line").and_then(Value::as_u64).unwrap_or(0);
        let declared = declared_at(corpus_root, rel_path, line);
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
        if redacted_license && redacted_field {
            if let Some(raw_tokens) = doc.get("data").and_then(|d| d.get("raw_tokens")).and_then(Value::as_array) {
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
}
