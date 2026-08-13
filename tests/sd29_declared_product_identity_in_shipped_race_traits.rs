//! Pipeline A's Product-Identity gate against **PCGen's own per-record
//! declaration**, not only against this program's term blacklist.
//!
//! # Why this suite exists
//!
//! `pi_screening::PI_BLACKLIST_TERMS` is a bounded heuristic
//! (`docs/governance/ogl-pi-blacklist.md` says so in its own header): a list of
//! 55 deity/place/NPC names *this program* assembled. PCGen's corpus states the
//! same fact directly, per record, in two tokens the ingest path had never
//! read:
//!
//! * `NAMEISPI:YES` — the record's **name** is Product Identity;
//! * `DESCISPI:YES` — the record's **description** is Product Identity.
//!
//! The two disagree, and the disagreement is one-directional and shipping.
//! Derived over the tree this suite guards, at the commit that added it:
//!
//! ```text
//! shipped race_trait records carrying DESCISPI:YES : 26
//!   of those, redacted by the term blacklist       : 18
//!   of those, shipped verbatim to a player         :  8
//! shipped race_trait records carrying NAMEISPI:YES :  1  (`Elf ~ Sovyrian-Born`)
//! ```
//!
//! The blacklist caught 18 of the 26 **by coincidence** — those descriptions
//! happen to contain a Golarion place name that is on the list. The other 8 say
//! nothing the list knows and were published anyway.
//!
//! # Why a declared-PI name is dropped rather than redacted
//!
//! A description can be replaced with `[redacted PI]` and the record still
//! works: its key, flags, bonuses and page cite are unaffected. A **name**
//! cannot. It is the record's identity on every screen and half of its key, so
//! the only way not to publish it is not to publish the row. This is the same
//! ruling the monster lane reached independently for Inner Sea World Guide's
//! five `NAMEISPI:YES` monster rows (`decisions.md §50`), applied to the kind
//! that lane reported it against.
//!
//! Reclassifying a declared-PI row as shippable is
//! `docs/governance/ogl-pi-blacklist.md` §3's per-book override — an operator
//! decision, not a lane's.

use std::fs;
use std::path::{Path, PathBuf};

use codex::rules_core::shape_b_v1::{PI_MARKER_REDACTED, REDACTED_PI_MARKER};

fn corpus_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/corpus")
}

/// Every shipped `race_trait` record, as `(path, parsed json)`.
fn shipped_race_trait_records() -> Vec<(PathBuf, serde_json::Value)> {
    let mut out = Vec::new();
    let root = corpus_root();
    let books = fs::read_dir(&root).unwrap_or_else(|e| panic!("failed to read {root:?}: {e}"));
    for book in books.flatten() {
        let kind_dir = book.path().join("race_trait");
        if !kind_dir.is_dir() {
            continue;
        }
        collect_json(&kind_dir, &mut out);
    }
    assert!(!out.is_empty(), "no shipped race_trait records found under {root:?}");
    out
}

fn collect_json(dir: &Path, out: &mut Vec<(PathBuf, serde_json::Value)>) {
    for entry in fs::read_dir(dir).unwrap_or_else(|e| panic!("failed to read {dir:?}: {e}")).flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_json(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "json") {
            let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {path:?}: {e}"));
            let json: serde_json::Value =
                serde_json::from_str(&text).unwrap_or_else(|e| panic!("{path:?} is not valid JSON: {e}"));
            out.push((path, json));
        }
    }
}

/// `true` when the record's own `raw_tokens` carry `<token>:YES`.
fn declares(record: &serde_json::Value, token: &str) -> bool {
    record["data"]["raw_tokens"]
        .as_array()
        .map(|tokens| {
            tokens.iter().any(|t| {
                t["key"].as_str().is_some_and(|k| k.eq_ignore_ascii_case(token))
                    && t["value"].as_str().is_some_and(|v| v.trim().eq_ignore_ascii_case("YES"))
            })
        })
        .unwrap_or(false)
}

#[test]
fn no_shipped_race_trait_record_publishes_a_name_the_corpus_declares_product_identity() {
    let offenders: Vec<String> = shipped_race_trait_records()
        .into_iter()
        .filter(|(_, record)| declares(record, "NAMEISPI"))
        .map(|(path, record)| {
            format!("{} ({})", record["data"]["key"].as_str().unwrap_or("<no key>"), path.display())
        })
        .collect();
    assert!(
        offenders.is_empty(),
        "a record NAME cannot be redacted, so a row whose name PCGen declares Product Identity must \
         not ship at all (`decisions.md §50`, `ogl-pi-blacklist.md` §3); still shipped: {offenders:#?}"
    );
}

#[test]
fn every_race_trait_description_the_corpus_declares_product_identity_ships_redacted() {
    let mut declared = 0usize;
    let mut leaked: Vec<String> = Vec::new();
    for (path, record) in shipped_race_trait_records() {
        if !declares(&record, "DESCISPI") {
            continue;
        }
        declared += 1;
        let marker = record["pi_marker"].as_str();
        let description = record["data"]["description"].as_str();
        if marker != Some(PI_MARKER_REDACTED) || description != Some(REDACTED_PI_MARKER) {
            leaked.push(format!(
                "{} ({}) — pi_marker {marker:?}, description {:?}",
                record["data"]["key"].as_str().unwrap_or("<no key>"),
                path.display(),
                description.unwrap_or("<none>"),
            ));
        }
    }
    assert!(
        declared > 0,
        "this suite asserts nothing if no shipped record declares DESCISPI:YES — 26 did when it was \
         written, and a run finding zero means the scan, not the corpus, changed"
    );
    assert!(
        leaked.is_empty(),
        "{declared} shipped race_trait records declare DESCISPI:YES; these publish the declared text \
         anyway because no blacklist term happened to appear in it: {leaked:#?}"
    );
}
