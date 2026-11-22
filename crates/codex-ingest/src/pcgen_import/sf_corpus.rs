//! The Starfinder 1e corpus: one licence-screened identity record per inventory unit of the
//! converted books (SD-37 E3.4, `docs/release/SD-37-starfinder-1e/decisions.md §4`, `§6`, `§7`).
//!
//! Pathfinder's `data/corpus/` grew over thirty bundles from per-book, per-kind ingest bins.
//! Starfinder's is generated whole, by one pass, from the two inputs that already define it: the
//! SF work inventory (E0.3, `docs/work-inventory.starfinder-1e.json`) and each unit's own row in
//! the pinned oracle (read through the converted books' `.pcc` includes, so an excluded book is
//! never opened). Output: `data/starfinder-1e/corpus/<book>/<kind>/<slug>.json` plus one
//! `LICENSE.json` per book, deterministic (no timestamp), so `--check` can prove it fresh.
//!
//! **What a record carries.** The unit's identity (`unit_id`, `data.key`, `data.name`), the row's
//! own `CATEGORY:`/`TYPE:` (`data.category`, `data.type`, which the converter reads for a
//! Starfinder record), its provenance (`source`: path, line, the `.lst` file's sha256), and the
//! licence screen's verdict. It carries **no** token array and no description: the sheet-rule
//! converter reads the row itself from the pinned tree, so a second copy of the source format
//! would only be residue (`scripts/pcgen_residue_gate.py`).
//!
//! **The licence screen** (E0.2, `pi_screening::classify_field_sf`: the Pathfinder term list plus
//! `SF_PI_TERMS`, and the row's own `NAMEISPI:`/`DESCISPI:` declarations):
//! - a product-identity **name** cannot be redacted and still identify the record, so the record
//!   is renamed to the Codex-generated neutral name of its coordinate
//!   (`codex_neutral_name::neutral_name`, the Pathfinder `§24` rule), file and all;
//! - a product-identity **description** (any `DESC:` field of the row) is stamped
//!   `pi_field: description`, and the converter prints none of it;
//! - either makes the record `PI-REDACTED`; neither leaves it `OGL`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use codex::rules_core::codex_neutral_name::{neutral_key, neutral_name};
use codex::rules_core::game_system::GameSystem;
use codex::rules_core::pi_screening::{classify_field_sf, declared_product_identity};
use codex::rules_core::shape_b_v1::License;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::pcgen_import::sheet_rule::closure::{PinnedTree, RowRef, tokenize_row};
use crate::pcgen_import::sheet_rule::ctx::slug;
use crate::pcgen_import::sheet_rule::inventory_source_rows;
use crate::pcgen_import::system_books::{self, CONVERTED_BOOKS};

/// Repo-relative root of the Starfinder corpus (the `GameSystem` package root).
pub fn corpus_dir(repo: &Path, system: GameSystem) -> PathBuf {
    system.package_roots(repo).corpus
}

/// One generated corpus: repo-root-relative-to-the-corpus path -> bytes, and the counts the
/// `LICENSE.json` files and the run summary state.
#[derive(Debug, Default)]
pub struct CorpusRun {
    pub files: BTreeMap<String, Vec<u8>>,
    pub records: usize,
    pub redacted: usize,
    pub renamed: usize,
    pub description_redacted: usize,
    /// Units whose own row is not in the pinned tree (each refused by id, never written).
    pub missing_rows: Vec<String>,
}

/// The screen's verdict for one row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Screen {
    pub name_pi: bool,
    pub description_pi: bool,
}

/// Screen one row: its name (and `KEY:`) and every `DESC:` field, by the row's own declarations
/// and by `classify_field_sf`.
pub fn screen_row(name: &str, key: &str, tokens: &[(String, String)]) -> Screen {
    let declared = declared_product_identity(tokens.iter().map(|(k, v)| (k.as_str(), v.as_str())));
    let hit = |field: &str, value: &str| classify_field_sf(field, value).0 == License::PiRedacted;
    let name_pi = declared.name || hit("name", name) || hit("name", key);
    let description_pi = declared.description || tokens.iter().any(|(k, v)| k == "DESC" && hit("description", v));
    Screen { name_pi, description_pi }
}

/// Generate the corpus of `system`'s converted books.
pub fn generate(repo: &Path, tree: &PinnedTree) -> Result<CorpusRun, String> {
    let system = tree.system;
    if CONVERTED_BOOKS.books(system).is_empty() {
        return Err(format!("no converted book is registered for game system {system} (system_books::CONVERTED_BOOKS)"));
    }
    let units = inventory_source_rows(repo, tree)?;
    let mut run = CorpusRun::default();
    let mut file_sha: BTreeMap<String, String> = BTreeMap::new();
    let mut per_book: BTreeMap<String, (usize, usize, usize)> = BTreeMap::new();
    for u in &units {
        let Some(file) = tree.file_index(&u.rel_path).filter(|_| u.line > 0) else {
            run.missing_rows.push(u.id.clone());
            continue;
        };
        let row = tree.row_text(RowRef { file, line: u.line });
        let (head, tokens) = tokenize_row(row);
        let token = |k: &str| tokens.iter().find(|(t, _)| t == k).map(|(_, v)| v.trim().to_string()).filter(|v| !v.is_empty());
        // The row's own KEY: else the name field (a `.COPY=` row's own name is the part before
        // `.COPY=`; `CATEGORY=x|` and `CLASS:` prefixes are not part of the key).
        let own_name = head.split(".COPY=").next().unwrap_or(&head).trim_start_matches("CLASS:").to_string();
        let key = token("KEY").unwrap_or_else(|| own_name.clone());
        let screen = screen_row(&u.name, &key, &tokens);
        let sha = file_sha
            .entry(u.rel_path.clone())
            .or_insert_with(|| {
                let bytes = std::fs::read(tree.root.join(&u.rel_path)).unwrap_or_default();
                format!("{:x}", Sha256::digest(&bytes))
            })
            .clone();
        let line = u32::try_from(u.line).map_err(|_| format!("{}: line {} out of range", u.id, u.line))?;
        let (name, key, file_slug) = if screen.name_pi {
            let n = neutral_name(&u.kind, &u.book, &u.source_file, line);
            let k = neutral_key(&u.kind, &u.book, &u.source_file, line);
            let s = slug(&k);
            (n, k, s)
        } else {
            (u.name.clone(), key, u.id.splitn(3, ':').nth(2).unwrap_or(&u.id).to_string())
        };
        let mut pi_fields: Vec<&str> = Vec::new();
        if screen.description_pi {
            pi_fields.push("description");
        }
        if screen.name_pi {
            pi_fields.push("name");
        }
        let license = if pi_fields.is_empty() { "OGL" } else { "PI-REDACTED" };
        let mut data = serde_json::Map::new();
        data.insert("key".into(), json!(key));
        data.insert("name".into(), json!(name));
        if let Some(c) = token("CATEGORY") {
            data.insert("category".into(), json!(c));
        }
        if let Some(t) = token("TYPE") {
            data.insert("type".into(), json!(t));
        }
        let mut record = serde_json::Map::new();
        record.insert("unit_id".into(), json!(u.id));
        record.insert("population".into(), json!("in_scope"));
        record.insert("data".into(), Value::Object(data));
        record.insert(
            "source".into(),
            json!({ "kind": "lst_token", "path": u.rel_path, "line": u.line, "sha256": sha, "record_key": key }),
        );
        record.insert("license".into(), json!(license));
        record.insert("pi_field".into(), if pi_fields.is_empty() { Value::Null } else { json!(pi_fields.join(",")) });
        record.insert("pi_marker".into(), if pi_fields.is_empty() { Value::Null } else { json!("redacted") });
        if screen.name_pi {
            record.insert("codex_generated_name".into(), json!(true));
            record.insert(
                "rename".into(),
                json!({ "coordinate": format!("{}:{}:{}", u.book, u.source_file, u.line), "reason": "name_pi_blocked" }),
            );
        }
        let rel = format!("{}/{}/{}.json", u.book, u.kind, file_slug);
        let mut text = serde_json::to_string_pretty(&Value::Object(record)).expect("a JSON object serialises");
        text.push('\n');
        if run.files.insert(rel.clone(), text.into_bytes()).is_some() {
            return Err(format!("two units write {rel}"));
        }
        run.records += 1;
        let b = per_book.entry(u.book.clone()).or_default();
        b.0 += 1;
        if license != "OGL" {
            run.redacted += 1;
            b.1 += 1;
        }
        if screen.name_pi {
            run.renamed += 1;
            b.2 += 1;
        }
        if screen.description_pi {
            run.description_redacted += 1;
        }
    }
    if !run.missing_rows.is_empty() {
        return Err(format!(
            "{} unit(s) have no source row in the pinned tree, first: {:?}",
            run.missing_rows.len(),
            &run.missing_rows[..run.missing_rows.len().min(5)]
        ));
    }
    for book in CONVERTED_BOOKS.books(system) {
        let id = system_books::book_id(book);
        let (processed, redacted, renamed) = per_book.get(id).copied().unwrap_or_default();
        let license = json!({
            "book": id,
            "book_pcc": book.pcc,
            "license_declaration": {
                "open_game_content": "OGL 1.0a; the book's own OGL Section 15 notice (docs/governance/license-matrix.md, Starfinder rows)",
                "product_identity_source": "Paizo's Starfinder Product Identity declaration (docs/governance/ogl-pi-blacklist.md §7)"
            },
            "operator_sign_off": {
                "note": "Set true only after an operator has reviewed this book's classification (license-matrix.md: operator_sign_off false).",
                "signed_off": false
            },
            "records_processed": processed,
            "records_redacted": redacted,
            "records_renamed": renamed,
            "redaction_policy": {
                "screen": "codex::rules_core::pi_screening::classify_field_sf on name, KEY and every DESC field, plus the row's NAMEISPI/DESCISPI declarations",
                "name_pi": "renamed to codex_neutral_name::neutral_name of the row coordinate (a name cannot be redacted and still identify the record)",
                "description_pi": "pi_field description; the sheet-rule converter prints none of it",
                "schema_preserving": true
            },
            "generated_by": "cargo run --locked -p codex-ingest --bin sf_corpus -- --write",
            "screening_method_note": "A bounded, case-sensitive substring scan against the Pathfinder blacklist and the Starfinder term set (E0.2), plus the corpus's own declared-PI tokens. It is not an exhaustive human legal review and does not prove the absence of PI beyond what that scan can see."
        });
        let mut text = serde_json::to_string_pretty(&license).expect("a JSON object serialises");
        text.push('\n');
        run.files.insert(format!("{id}/LICENSE.json"), text.into_bytes());
    }
    Ok(run)
}

/// Every file under `dir` as `(relative path, bytes)`.
pub fn read_dir_files(dir: &Path) -> BTreeMap<String, Vec<u8>> {
    let mut out = BTreeMap::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else { continue };
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else {
                let rel = p.strip_prefix(dir).unwrap_or(&p).to_string_lossy().replace('\\', "/");
                out.insert(rel, std::fs::read(&p).unwrap_or_default());
            }
        }
    }
    out
}

/// The on-disk corpus against a fresh generation: every difference, named.
pub fn check(dir: &Path, run: &CorpusRun) -> Result<(), Vec<String>> {
    let disk = read_dir_files(dir);
    let mut problems = Vec::new();
    for (rel, bytes) in &run.files {
        match disk.get(rel) {
            None => problems.push(format!("missing on disk: {rel}")),
            Some(d) if d != bytes => problems.push(format!("stale on disk: {rel}")),
            _ => {}
        }
    }
    for rel in disk.keys() {
        if !run.files.contains_key(rel) {
            problems.push(format!("unexpected file on disk: {rel}"));
        }
    }
    if problems.is_empty() { Ok(()) } else { Err(problems) }
}

/// Replace `dir` with exactly the generated files.
pub fn write(dir: &Path, run: &CorpusRun) -> std::io::Result<()> {
    if dir.exists() {
        std::fs::remove_dir_all(dir)?;
    }
    for (rel, bytes) in &run.files {
        let p = dir.join(rel);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(p, bytes)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toks(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    /// A Starfinder proper noun in the name renames the record; in a `DESC:` it stamps the
    /// description; a row's own `NAMEISPI:YES` renames it whatever the term scan says; a
    /// mechanic-only row stays OGL. The Pathfinder screen alone misses every SF term here.
    #[test]
    fn the_screen_reads_names_descriptions_and_declarations() {
        assert_eq!(
            screen_row("Eoxian Bone Blade", "Eoxian Bone Blade", &toks(&[("DESC", "A blade.")])),
            Screen { name_pi: true, description_pi: false }
        );
        assert_eq!(
            screen_row("Bone Blade", "Bone Blade", &toks(&[("DESC", "Forged on Eox.")])),
            Screen { name_pi: false, description_pi: true }
        );
        assert_eq!(
            screen_row("Plain Name", "Plain Name", &toks(&[("NAMEISPI", "YES")])),
            Screen { name_pi: true, description_pi: false }
        );
        assert_eq!(
            screen_row("Second Skin", "Second Skin", &toks(&[("DESC", "Light armor."), ("DESCISPI", "NO")])),
            Screen { name_pi: false, description_pi: false }
        );
        assert_eq!(
            codex::rules_core::pi_screening::classify_field("name", "Eoxian Bone Blade").0,
            License::Ogl,
            "the Pathfinder screen alone does not know Starfinder's terms"
        );
    }
}
