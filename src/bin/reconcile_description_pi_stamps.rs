//! SD-32 declared-pi-shipping-65-followups: the guarded-path fixer for a
//! shipped `data/corpus/**/*.json` record whose `data.description` is
//! already the redaction marker (`"[redacted PI]"`) but whose `license`/
//! `pi_field` were never stamped to say so — the metadata-labeling gap
//! `declared_pi_shipping_audit` found live in 65
//! `bestiary_4/monster_ability` records (and, per this cycle's own
//! corpus-wide re-derivation, 34 further records across 8 other
//! `(book, kind)` pairs the audit's line-scoped `declared.description`
//! check cannot see, because their redaction was triggered by the
//! blacklist-term scan rather than the corpus row's own `DESCISPI:YES`
//! declaration).
//!
//! **Root cause, fixed separately.** `pi_screening::classify_field` used to
//! treat a value that was ALREADY the marker as ordinary prose (the marker
//! text contains no blacklist term), stamping `Ogl`/`None` over it. That
//! guard now lives in `classify_field` itself, so no NEW record can reopen
//! this gap. This binary is the one-time (idempotent, safely re-runnable)
//! remediation for the 99 records the bug already produced before the
//! guard existed — every one of `gen_book_cache.rs`'s and
//! `cache_gen::{equipment_gap, feat_gap}`'s writers is no-clobber on an
//! existing file, so there is no way to route an already-shipped record
//! back through its normal from-scratch generator path without deleting it
//! first. Deleting and regenerating a record is a bigger, riskier
//! operation than the one this bug actually needs (it never changes
//! `data`, `source`, or `ingested_at` — see the fix in `pi_screening.rs`'s
//! `reconcile_description_pi_stamp`), so this tool reconciles ONLY the
//! three stamp fields (`license`, `pi_field`, `pi_marker`) in place,
//! leaving every other byte of the record untouched — the same
//! read-`Value`-patch-one-field-write-back shape
//! `enrich_monster_ability_raw_tokens.rs` already uses for its own
//! in-place corpus fix.
//!
//! Run via `cargo run --locked --release --bin reconcile_description_pi_stamps`.
//! Walks the WHOLE corpus (book- and kind-agnostic by construction, per
//! `decisions.md §17`'s "generic pass, not per-object work" ruling) —
//! it does not special-case `bestiary_4`/`monster_ability`, so it also
//! closes the same gap wherever else it independently occurred.

use std::fs;
use std::path::{Path, PathBuf};

use codex::rules_core::pi_screening::reconcile_description_pi_stamp;
use codex::rules_core::shape_b_v1::License;
use serde_json::Value;

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
            } else if path.extension().and_then(|e| e.to_str()) == Some("json")
                && path.file_name().and_then(|n| n.to_str()) != Some("LICENSE.json")
            {
                out.push(path);
            }
        }
    }
    out
}

fn license_of(value: Option<&str>) -> License {
    match value {
        Some("PI-REDACTED") => License::PiRedacted,
        Some("PI") => License::Pi,
        _ => License::Ogl,
    }
}

fn license_str(license: License) -> &'static str {
    match license {
        License::Ogl => "OGL",
        License::Pi => "PI",
        License::PiRedacted => "PI-REDACTED",
    }
}

/// Reconciles one file in place. Returns `true` when a patch was written.
pub fn reconcile_one(path: &Path) -> bool {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    let mut root: Value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("parse {path:?} as JSON: {e}"));

    let description = root.get("data").and_then(|d| d.get("description")).and_then(Value::as_str).map(str::to_string);
    let current_license = license_of(root.get("license").and_then(Value::as_str));
    let current_pi_field = root.get("pi_field").and_then(Value::as_str).map(str::to_string);

    let Some((new_license, new_pi_field, new_pi_marker)) =
        reconcile_description_pi_stamp(description.as_deref(), current_license, current_pi_field.as_deref())
    else {
        return false;
    };

    let obj = root.as_object_mut().unwrap_or_else(|| panic!("{path:?}: top-level JSON is not an object"));
    obj.insert("license".to_string(), Value::String(license_str(new_license).to_string()));
    obj.insert(
        "pi_field".to_string(),
        new_pi_field.map(Value::String).unwrap_or(Value::Null),
    );
    obj.insert(
        "pi_marker".to_string(),
        new_pi_marker.map(Value::String).unwrap_or(Value::Null),
    );

    let new_json = serde_json::to_string_pretty(&root).expect("serialize reconciled record");
    fs::write(path, new_json + "\n").unwrap_or_else(|e| panic!("write {path:?}: {e}"));
    true
}

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let data_corpus_root = PathBuf::from(manifest_dir).join("data/corpus");

    let mut fixed = Vec::new();
    for path in find_json_files(&data_corpus_root) {
        if reconcile_one(&path) {
            fixed.push(path.display().to_string());
        }
    }

    fixed.sort();
    println!("reconcile-description-pi-stamps: {} record(s) patched", fixed.len());
    for f in &fixed {
        println!("  {f}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Scratch {
        root: PathBuf,
    }
    impl Scratch {
        fn new(name: &str) -> Self {
            let root = std::env::temp_dir().join(format!("codex_reconcile_pi_stamps_test_{name}_{}", std::process::id()));
            let _ = fs::remove_dir_all(&root);
            fs::create_dir_all(&root).unwrap();
            Scratch { root }
        }
        fn write(&self, rel: &str, contents: &str) -> PathBuf {
            let path = self.root.join(rel);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(&path, contents).unwrap();
            path
        }
    }
    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    /// The exact 65-record `bestiary_4/monster_ability` shape: marker in
    /// `data.description`, `license: "OGL"`, `pi_field: null`. Every OTHER
    /// field (`data.key`, `source`, `wiring_class`, ...) must survive
    /// byte-for-byte.
    #[test]
    fn reconcile_one_fixes_the_bestiary_4_shape_and_touches_nothing_else() {
        let s = Scratch::new("bestiary4_shape");
        let path = s.write(
            "x.json",
            r#"{"population":"in_scope","completeness":"full","ingested_at":"2026-01-01T00:00:00Z",
                "data":{"key":"bestiary_4:monster_ability:x","name":"X","description":"[redacted PI]"},
                "source":{"kind":"lst_token","path":"some_book/rows.lst","line":1,"record_key":"x"},
                "license":"OGL","pi_field":null,"pi_marker":null,
                "wiring_class":"static","wiring_class_signals":[],"description_source":null}"#,
        );
        assert!(reconcile_one(&path), "must report a patch was written");
        let after: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(after["license"], "PI-REDACTED");
        assert_eq!(after["pi_field"], "description");
        assert_eq!(after["pi_marker"], "redacted");
        // Untouched fields:
        assert_eq!(after["data"]["key"], "bestiary_4:monster_ability:x");
        assert_eq!(after["data"]["description"], "[redacted PI]");
        assert_eq!(after["source"]["line"], 1);
        assert_eq!(after["ingested_at"], "2026-01-01T00:00:00Z");
        assert_eq!(after["wiring_class"], "static");
    }

    /// The 9 `inner_sea_gods/equipment` shape: a `§24` rename already
    /// stamped `pi_field: "name"` -- the fix must UNION `"description"`
    /// in, never drop the existing `"name"` entry.
    #[test]
    fn reconcile_one_unions_description_into_an_existing_name_redaction() {
        let s = Scratch::new("union_shape");
        let path = s.write(
            "x.json",
            r#"{"data":{"key":"codex_named_unit","name":"Codex-Named Unit (x)","description":"[redacted PI]"},
                "source":{"kind":"lst_token","path":"some_book/rows.lst","line":1,"record_key":"x"},
                "license":"PI-REDACTED","pi_field":"name","pi_marker":"redacted",
                "codex_generated_name":true}"#,
        );
        assert!(reconcile_one(&path));
        let after: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(after["pi_field"], "name,description");
        assert_eq!(after["license"], "PI-REDACTED");
        assert_eq!(after["codex_generated_name"], true, "unrelated field must survive");
    }

    /// An already-correct record (post-fix, or a record that was never
    /// broken) is reported as no-patch and its mtime/content-on-disk is
    /// provably untouched (re-reading the file yields byte-identical text).
    #[test]
    fn reconcile_one_is_a_true_no_op_on_an_already_correct_record() {
        let s = Scratch::new("already_correct");
        let before_text = r#"{"data":{"key":"x","name":"X","description":"[redacted PI]"},
                "source":{"kind":"lst_token","path":"some_book/rows.lst","line":1,"record_key":"x"},
                "license":"PI-REDACTED","pi_field":"description","pi_marker":"redacted"}"#;
        let path = s.write("x.json", before_text);
        assert!(!reconcile_one(&path), "an already-correct record must not be reported as patched");
        let after_text = fs::read_to_string(&path).unwrap();
        assert_eq!(after_text, before_text, "file must be byte-identical -- not even rewritten");
    }

    /// Mutation proof (`§1a`): an ordinary, never-redacted record must
    /// never be flagged or touched.
    #[test]
    fn reconcile_one_never_touches_an_ordinary_unredacted_record() {
        let s = Scratch::new("ordinary");
        let before_text = r#"{"data":{"key":"x","name":"X","description":"Deals 1d6 points of fire damage."},
                "source":{"kind":"lst_token","path":"some_book/rows.lst","line":1,"record_key":"x"},
                "license":"OGL","pi_field":null,"pi_marker":null}"#;
        let path = s.write("x.json", before_text);
        assert!(!reconcile_one(&path));
        assert_eq!(fs::read_to_string(&path).unwrap(), before_text);
    }

    #[test]
    fn reconcile_one_never_touches_a_record_with_no_description_field() {
        let s = Scratch::new("no_description");
        let before_text = r#"{"data":{"key":"x","name":"X"},
                "source":{"kind":"lst_token","path":"some_book/rows.lst","line":1,"record_key":"x"},
                "license":"OGL","pi_field":null,"pi_marker":null}"#;
        let path = s.write("x.json", before_text);
        assert!(!reconcile_one(&path));
        assert_eq!(fs::read_to_string(&path).unwrap(), before_text);
    }

    #[test]
    fn find_json_files_skips_license_json() {
        let s = Scratch::new("skip_license");
        s.write("some_book/LICENSE.json", "{}");
        s.write("some_book/monster_ability/x.json", "{}");
        let files = find_json_files(&s.root);
        assert_eq!(files.len(), 1, "{files:?}");
        assert!(files[0].ends_with("some_book/monster_ability/x.json"), "{files:?}");
    }
}
