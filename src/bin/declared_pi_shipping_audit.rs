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

        if declared.name {
            // A key/name cannot be redacted (it is the record's identity
            // on every screen and half of its key) -- its mere presence
            // on disk IS the violation, per `SD-29-corpus-wide-catch-up-
            // lanes/decisions.md §50.3`.
            violations.push(Violation {
                file: file_str.clone(),
                reason: format!("NAME-PI-SHIPPED: cites {rel_path}:{line} (NAMEISPI:YES) but exists on disk"),
            });
        }

        if declared.description {
            let desc = doc.get("data").and_then(|d| d.get("description")).and_then(Value::as_str);
            let license = doc.get("license").and_then(Value::as_str);
            let pi_field = doc.get("pi_field").and_then(Value::as_str);
            let redacted_desc = desc == Some(REDACTED_PI_MARKER);
            let redacted_license = license == Some("PI-REDACTED");
            let redacted_field = pi_field == Some("description");
            if !(redacted_desc && redacted_license && redacted_field) {
                violations.push(Violation {
                    file: file_str,
                    reason: format!(
                        "DESC-PI-SHIPPED: cites {rel_path}:{line} (DESCISPI:YES) but data.description={desc:?} \
                         license={license:?} pi_field={pi_field:?} (expected description=\"[redacted PI]\", \
                         license=\"PI-REDACTED\", pi_field=\"description\")"
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
