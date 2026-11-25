//! Writes `data.member_types` on every ability-shaped corpus record the pinned oracle types into an
//! ability category through a tag the record does not carry (SD-43 E3.12; the rule is
//! `pcgen_import::member_typing`'s module doc). Additive and idempotent: a record gains, keeps or
//! loses only that one field; every other byte of every file is untouched. Each tag is PI-screened
//! (`screen_member_tags`, SD-43 E3.22) before it is written.
//!
//! Run: `source <(bash scripts/fetch-pcgen-oracle.sh --check --quiet)` (exports `PCGEN_CORPUS_ROOT`
//! and `PCGEN_REPO_DIR`), then `cargo run --locked -p codex-ingest --bin enrich_member_typing`.
//! `--check` writes nothing and exits 1 when a record would change.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use codex_ingest::pcgen_import::member_typing::{record_identity, screen_member_tags, with_member_types, MemberTyping, ABILITY_KINDS};
use serde_json::Value;

fn json_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            json_files(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
            out.push(path);
        }
    }
}

fn env_dir(name: &str) -> PathBuf {
    let dir = PathBuf::from(std::env::var(name).unwrap_or_else(|_| panic!("{name} unset: source <(bash scripts/fetch-pcgen-oracle.sh --check --quiet)")));
    assert!(dir.is_dir(), "{name}={} is not a directory", dir.display());
    dir
}

fn main() {
    let check = std::env::args().any(|a| a == "--check");
    let oracle_data = env_dir("PCGEN_CORPUS_ROOT");
    let game_mode = env_dir("PCGEN_REPO_DIR").join("system/gameModes/Pathfinder");
    let corpus = codex_ingest::repo_root().join("data/corpus");

    let mut records = Vec::new();
    let mut roots = BTreeSet::new();
    let mut books: Vec<PathBuf> = fs::read_dir(&corpus).expect("data/corpus").flatten().map(|e| e.path()).filter(|p| p.is_dir()).collect();
    books.sort();
    for book in &books {
        let mut all = Vec::new();
        json_files(book, &mut all);
        all.sort();
        for path in all {
            let kind = path.strip_prefix(book).ok().and_then(|p| p.components().next()).and_then(|c| c.as_os_str().to_str()).unwrap_or_default().to_string();
            let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
            let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
            if let Some(src) = doc.get("source").and_then(|s| s.get("path")).and_then(Value::as_str) {
                // The book root: the file's directory, at most four segments deep (as E0.1's census).
                let dir: Vec<&str> = src.split('/').collect();
                let dir = &dir[..dir.len().saturating_sub(1)];
                if !dir.is_empty() {
                    roots.insert(dir[..dir.len().min(4)].join("/"));
                }
            }
            if ABILITY_KINDS.contains(&kind.as_str()) {
                records.push((path, kind, text, doc));
            }
        }
    }
    let mut category_files: Vec<PathBuf> = fs::read_dir(&game_mode).expect("game mode dir").flatten().map(|e| e.path()).filter(|p| p.extension().and_then(|e| e.to_str()) == Some("lst")).collect();
    category_files.sort();
    let typing = MemberTyping::from_oracle(&oracle_data, &roots, &category_files).expect("read the oracle");

    let (mut written, mut tags_added, mut drift) = (0usize, 0usize, Vec::new());
    for (path, kind, text, doc) in &records {
        let Some((category, key, carried)) = record_identity(doc, kind) else { continue };
        let tags = screen_member_tags(typing.missing(&category, &key, &carried));
        let new = with_member_types(text, &tags).unwrap_or_else(|| panic!("{path:?}: no two-space `data` object to write into"));
        if !tags.is_empty() {
            written += 1;
            tags_added += tags.len();
        }
        if &new != text {
            drift.push(path.strip_prefix(codex_ingest::repo_root()).unwrap_or(path).display().to_string());
            if !check {
                fs::write(path, new).unwrap_or_else(|e| panic!("write {path:?}: {e}"));
            }
        }
    }
    println!(
        "member typing: {} ability records scanned over {} book roots; {written} carry member_types ({tags_added} tags); {} file(s) {}; {} .MOD TYPE rows name no base row in scope",
        records.len(),
        roots.len(),
        drift.len(),
        if check { "would change" } else { "changed" },
        typing.unresolved_mod_rows,
    );
    if check && !drift.is_empty() {
        for d in drift.iter().take(20) {
            eprintln!("  drift: {d}");
        }
        std::process::exit(1);
    }
}
