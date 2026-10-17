//! Cyberpunk RED corpus manifest gate: the Rust twin of `scripts/cpr_manifest_check.py`.
//!
//! An independent second implementation of the same checks (decisions.md §11, R12): every kind
//! named by `data/cyberpunk-red/corpus/MANIFEST.json` exists as `<kind>.json`, is a JSON array of
//! objects, holds exactly `expected` records (never `null` under the all-kinds check), and every
//! record carries a `source_page` of the form `pNNN` inside `p001`..`p459`; no corpus file is
//! unnamed; and the manifest's kinds and `expected` sum equal the kind table of the content-unit
//! inventory §3. The plant tests below fail it the same ten ways the Python `--selftest` does.
//!
//! Does not cover: field values (the page-image sample is the oracle for those), nested sub-table
//! counts, or whether a `source_page` is the right page.

use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const MAX_PAGE: u32 = 459;
const NON_KIND_FILES: [&str; 2] = ["MANIFEST.json", "LICENSE.json"];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

struct Entry {
    kind: String,
    file: String,
    expected: Option<u64>,
}

fn load_manifest(path: &Path) -> Result<Vec<Entry>, Vec<String>> {
    let text = fs::read_to_string(path)
        .map_err(|e| vec![format!("MANIFEST {}: cannot read ({e})", path.display())])?;
    let value: Value = serde_json::from_str(&text)
        .map_err(|e| vec![format!("MANIFEST {}: cannot read ({e})", path.display())])?;
    let kinds = match value.get("kinds").and_then(Value::as_array) {
        Some(k) if !k.is_empty() => k,
        _ => {
            return Err(vec![format!(
                "MANIFEST {}: no `kinds` array",
                path.display()
            )]);
        }
    };
    let mut errs = Vec::new();
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for e in kinds {
        let Some(kind) = e
            .get("kind")
            .and_then(Value::as_str)
            .filter(|k| !k.is_empty())
        else {
            errs.push(format!("MANIFEST entry without a kind: {e}"));
            continue;
        };
        if !seen.insert(kind.to_string()) {
            errs.push(format!("MANIFEST {kind}: duplicate entry"));
        }
        let file = e.get("file").and_then(Value::as_str).unwrap_or("");
        if file != format!("{kind}.json") {
            errs.push(format!(
                "MANIFEST {kind}: file must be {kind}.json, got {file:?}"
            ));
        }
        let expected = match e.get("expected") {
            None | Some(Value::Null) => None,
            Some(v) => match v.as_u64() {
                Some(n) => Some(n),
                None => {
                    errs.push(format!(
                        "MANIFEST {kind}: expected must be a non-negative int or null"
                    ));
                    None
                }
            },
        };
        if !e.get("print_only").is_some_and(Value::is_boolean) {
            errs.push(format!("MANIFEST {kind}: print_only must be a bool"));
        }
        out.push(Entry {
            kind: kind.to_string(),
            file: file.to_string(),
            expected,
        });
    }
    if errs.is_empty() { Ok(out) } else { Err(errs) }
}

/// `{kind: leading integer of the Expected cell, or None}` from the inventory's §3 kind table.
fn cui_table(path: &Path) -> Result<BTreeMap<String, Option<u64>>, String> {
    let text =
        fs::read_to_string(path).map_err(|e| format!("SUM cannot read {}: {e}", path.display()))?;
    let mut in_section = false;
    let mut found = false;
    let mut out = BTreeMap::new();
    for line in text.lines() {
        if line.starts_with("## §3 ") {
            in_section = true;
            found = true;
            continue;
        }
        if line.starts_with("## §4") {
            in_section = false;
        }
        if !in_section || !line.starts_with('|') {
            continue;
        }
        // Split on `|` not preceded by a backslash (`\|` is a literal pipe inside a cell).
        let body = line.trim().trim_matches('|');
        let mut cells = Vec::new();
        let mut cur = String::new();
        let mut prev = '\0';
        for ch in body.chars() {
            if ch == '|' && prev != '\\' {
                cells.push(cur.trim().to_string());
                cur.clear();
            } else {
                cur.push(ch);
            }
            prev = ch;
        }
        cells.push(cur.trim().to_string());
        if cells.len() < 2
            || cells[0] == "Kind"
            || cells[0].chars().all(|c| matches!(c, '-' | ':' | ' '))
        {
            continue;
        }
        let lead: String = cells[1]
            .trim_start()
            .chars()
            .take_while(char::is_ascii_digit)
            .collect();
        let lead = lead.parse::<u64>().ok();
        let mut rest = cells[0].as_str();
        while let Some(open) = rest.find('`') {
            let after = &rest[open + 1..];
            let Some(close) = after.find('`') else { break };
            let name = &after[..close];
            if !name.is_empty()
                && name
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
            {
                out.insert(name.to_string(), lead);
            }
            rest = &after[close + 1..];
        }
    }
    if found {
        Ok(out)
    } else {
        Err("SUM CUI section 3 not found".to_string())
    }
}

fn check_sum(manifest: &[Entry], cui: &Path) -> Vec<String> {
    let table = match cui_table(cui) {
        Ok(t) => t,
        Err(e) => return vec![e],
    };
    let mk: BTreeMap<&str, Option<u64>> = manifest
        .iter()
        .map(|e| (e.kind.as_str(), e.expected))
        .collect();
    let mut errs = Vec::new();
    for k in table.keys().filter(|k| !mk.contains_key(k.as_str())) {
        errs.push(format!(
            "SUM {k}: in CUI section 3, absent from the manifest"
        ));
    }
    for k in mk.keys().filter(|k| !table.contains_key(**k)) {
        errs.push(format!(
            "SUM {k}: in the manifest, absent from CUI section 3"
        ));
    }
    for (k, v) in &mk {
        if let Some(t) = table.get(*k)
            && v != t
        {
            errs.push(format!(
                "SUM {k}: manifest expected {v:?}, CUI section 3 says {t:?}"
            ));
        }
    }
    let ms: u64 = mk.values().flatten().sum();
    let ts: u64 = table.values().flatten().sum();
    if ms != ts {
        errs.push(format!("SUM total: manifest {ms}, CUI section 3 {ts}"));
    }
    errs
}

fn valid_page(v: Option<&Value>) -> bool {
    let Some(s) = v.and_then(Value::as_str) else {
        return false;
    };
    let b = s.as_bytes();
    if b.len() != 4 || b[0] != b'p' || !b[1..].iter().all(u8::is_ascii_digit) {
        return false;
    }
    let n: u32 = s[1..].parse().unwrap_or(0);
    (1..=MAX_PAGE).contains(&n)
}

fn check_kind(entry: &Entry, corpus: &Path, require_count: bool) -> Vec<String> {
    let k = &entry.kind;
    let mut errs = Vec::new();
    if entry.expected.is_none() && require_count {
        errs.push(format!("NULL_COUNT {k}: expected is null"));
    }
    let path = corpus.join(&entry.file);
    let Ok(text) = fs::read_to_string(&path) else {
        errs.push(format!("MISSING {k}: {}", entry.file));
        return errs;
    };
    let recs = match serde_json::from_str::<Value>(&text) {
        Ok(Value::Array(a)) if a.iter().all(Value::is_object) => a,
        Ok(_) => {
            errs.push(format!("BAD_FILE {k}: not a JSON array of objects"));
            return errs;
        }
        Err(e) => {
            errs.push(format!("BAD_FILE {k}: not JSON ({e})"));
            return errs;
        }
    };
    if let Some(exp) = entry.expected
        && recs.len() as u64 != exp
    {
        errs.push(format!("COUNT {k}: {} records, expected {exp}", recs.len()));
    }
    let bad: Vec<usize> = recs
        .iter()
        .enumerate()
        .filter(|(_, r)| !valid_page(r.get("source_page")))
        .map(|(i, _)| i)
        .collect();
    for i in bad.iter().take(3) {
        errs.push(format!(
            "BAD_SOURCE {k}[{i}]: source_page {:?}",
            recs[*i].get("source_page")
        ));
    }
    if bad.len() > 3 {
        errs.push(format!("BAD_SOURCE {k}: {} records in all", bad.len()));
    }
    errs
}

/// `kinds: None` is the all-kinds check (`--all`); `Some(list)` checks only those kinds.
fn run(manifest: &Path, corpus: &Path, cui: &Path, kinds: Option<&[&str]>) -> Vec<String> {
    let entries = match load_manifest(manifest) {
        Ok(e) => e,
        Err(errs) => return errs,
    };
    let mut errs = check_sum(&entries, cui);
    match kinds {
        None => {
            let listed: BTreeSet<&str> = entries.iter().map(|e| e.file.as_str()).collect();
            let mut files: Vec<String> = fs::read_dir(corpus)
                .map(|rd| {
                    rd.filter_map(|d| d.ok())
                        .map(|d| d.file_name().to_string_lossy().into_owned())
                        .collect()
                })
                .unwrap_or_default();
            files.sort();
            for f in files {
                if f.ends_with(".json")
                    && !NON_KIND_FILES.contains(&f.as_str())
                    && !listed.contains(f.as_str())
                {
                    errs.push(format!("EXTRA {f}: not named by the manifest"));
                }
            }
            for e in &entries {
                errs.extend(check_kind(e, corpus, true));
            }
        }
        Some(sel) => {
            for k in sel {
                match entries.iter().find(|e| e.kind == *k) {
                    Some(e) => errs.extend(check_kind(e, corpus, false)),
                    None => errs.push(format!("MANIFEST {k}: not a manifest kind")),
                }
            }
        }
    }
    errs
}

#[test]
fn every_manifest_kind_is_pinned_and_the_corpus_matches_it() {
    let root = repo_root();
    let errs = run(
        &root.join("data/cyberpunk-red/corpus/MANIFEST.json"),
        &root.join("data/cyberpunk-red/corpus"),
        &root.join("docs/release/SD-38-cyberpunk-red/content-unit-inventory.md"),
        None,
    );
    assert!(
        errs.is_empty(),
        "corpus manifest check failed:\n{}",
        errs.join("\n")
    );
}

#[test]
fn manifest_kind_count_and_expected_sum_are_the_pinned_figures() {
    let root = repo_root();
    let entries = load_manifest(&root.join("data/cyberpunk-red/corpus/MANIFEST.json"))
        .expect("manifest loads");
    let nulls = entries.iter().filter(|e| e.expected.is_none()).count();
    let sum: u64 = entries.iter().filter_map(|e| e.expected).sum();
    assert_eq!(
        (entries.len(), nulls, sum),
        (53, 0, 930),
        "kinds, null counts, expected sum"
    );
}

// ---- plants: the same ten failure shapes as `cpr_manifest_check.py --selftest` ----

struct Scratch {
    dir: PathBuf,
}

impl Scratch {
    fn new(name: &str) -> Self {
        let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("cpr_manifest_{name}"));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("corpus")).unwrap();
        fs::write(
            dir.join("cui.md"),
            "## §3 Kinds\n\n| Kind | Expected records | Figure |\n|---|---|---|\n\
             | `alpha` | 3 | F-1 |\n| `beta` | set by E1.8 | - |\n\n## §4 next\n",
        )
        .unwrap();
        let s = Scratch { dir };
        s.manifest(3, false);
        s
    }
    fn manifest(&self, alpha_expected: u64, extra_delta: bool) {
        let mut kinds = vec![
            serde_json::json!({"kind": "alpha", "file": "alpha.json", "expected": alpha_expected, "figure": "F-1",
                               "print_only": false, "nested": null, "lane": "E1.x"}),
            serde_json::json!({"kind": "beta", "file": "beta.json", "expected": null, "figure": null,
                               "print_only": true, "nested": null, "lane": "E1.8"}),
        ];
        if extra_delta {
            kinds.push(serde_json::json!({"kind": "delta", "file": "delta.json", "expected": 1, "figure": null,
                                          "print_only": false, "nested": null, "lane": "x"}));
        }
        fs::write(
            self.dir.join("MANIFEST.json"),
            serde_json::json!({ "kinds": kinds }).to_string(),
        )
        .unwrap();
    }
    fn alpha(&self, recs: Option<Value>) {
        let p = self.dir.join("corpus/alpha.json");
        let _ = fs::remove_file(&p);
        if let Some(r) = recs {
            fs::write(p, r.to_string()).unwrap();
        }
    }
    fn errs(&self, kinds: Option<&[&str]>) -> Vec<String> {
        run(
            &self.dir.join("MANIFEST.json"),
            &self.dir.join("corpus"),
            &self.dir.join("cui.md"),
            kinds,
        )
    }
}

fn good() -> Value {
    serde_json::json!([{"source_page": "p001"}, {"source_page": "p002"}, {"source_page": "p459"}])
}

fn assert_code(errs: &[String], code: &str) {
    assert!(
        errs.iter().any(|e| e.starts_with(code)),
        "want a {code:?} failure, got {errs:?}"
    );
}

#[test]
fn plant_good_control_passes() {
    let s = Scratch::new("good");
    s.alpha(Some(good()));
    assert_eq!(s.errs(Some(&["alpha"])), Vec::<String>::new());
}

#[test]
fn plant_null_count_and_missing_kind_fail_the_all_check() {
    let s = Scratch::new("null");
    s.alpha(Some(good()));
    let errs = s.errs(None);
    assert_code(&errs, "NULL_COUNT beta");
    assert_code(&errs, "MISSING beta");
}

#[test]
fn plant_short_count_fails() {
    let s = Scratch::new("short");
    s.alpha(Some(
        serde_json::json!([{"source_page": "p001"}, {"source_page": "p002"}]),
    ));
    assert_code(&s.errs(Some(&["alpha"])), "COUNT alpha");
}

#[test]
fn plant_page_460_fails() {
    let s = Scratch::new("p460");
    s.alpha(Some(serde_json::json!([{"source_page": "p001"}, {"source_page": "p002"}, {"source_page": "p460"}])));
    assert_code(&s.errs(Some(&["alpha"])), "BAD_SOURCE alpha");
}

#[test]
fn plant_record_without_source_page_fails() {
    let s = Scratch::new("nosource");
    s.alpha(Some(
        serde_json::json!([{"source_page": "p001"}, {"source_page": "p002"}, {"name": "x"}]),
    ));
    assert_code(&s.errs(Some(&["alpha"])), "BAD_SOURCE alpha");
}

#[test]
fn plant_missing_kind_file_fails() {
    let s = Scratch::new("missing");
    s.alpha(None);
    assert_code(&s.errs(Some(&["alpha"])), "MISSING alpha");
}

#[test]
fn plant_extra_corpus_file_fails() {
    let s = Scratch::new("extra");
    s.alpha(Some(good()));
    fs::write(s.dir.join("corpus/gamma.json"), "[]").unwrap();
    assert_code(&s.errs(None), "EXTRA gamma.json");
}

#[test]
fn plant_manifest_sum_mismatch_fails() {
    let s = Scratch::new("sum");
    s.alpha(Some(good()));
    s.manifest(4, false);
    assert_code(&s.errs(Some(&["alpha"])), "SUM alpha");
}

#[test]
fn plant_manifest_kind_absent_from_the_inventory_fails() {
    let s = Scratch::new("delta");
    s.alpha(Some(good()));
    s.manifest(3, true);
    assert_code(&s.errs(Some(&["alpha"])), "SUM delta");
}
