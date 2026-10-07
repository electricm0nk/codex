//! Writes `data/equipment_types.json`: PCGen `TYPE:` tags per equipment record identity.
//!
//! The equipment catalog's own four-way category cannot split thousands of items into browsable
//! kinds; PCGen's `TYPE:` taxonomy can. This reads every `*equip*.lst` under the pinned oracle's
//! `pathfinder/` tree (in sorted path order, so a duplicated identity resolves the same way every
//! run) and records tags by identity, using `pcgen_import::equipment_types`.
//!
//! ```text
//! PCGEN_CORPUS_ROOT="$HOME/workspace/repos/pcgen/data" \
//!   cargo run --locked -p codex-ingest --bin gen_equipment_types
//! ```
//!
//! Output is deterministic and carries the oracle pin it was read from. The desktop catalog joins
//! it onto rows by key; a row with no entry shows as uncategorized rather than guessed.

use std::path::{Path, PathBuf};

use codex_ingest::pcgen_import::equipment_types::{merge_equipment_types, parse_equipment_types};

fn corpus_root() -> PathBuf {
    match std::env::var("PCGEN_CORPUS_ROOT") {
        Ok(configured) => PathBuf::from(configured),
        Err(_) => PathBuf::from(std::env::var("HOME").expect("HOME must be set")).join("workspace/repos/pcgen/data"),
    }
}

fn collect_equipment_lsts(dir: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_equipment_lsts(&path, out)?;
        } else if path.extension().is_some_and(|e| e == "lst")
            && path.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.to_ascii_lowercase().contains("equip"))
        {
            out.push(path);
        }
    }
    Ok(())
}

fn pinned_oracle_sha(repo_root: &Path) -> String {
    let pin = std::fs::read_to_string(repo_root.join("scripts/pcgen-oracle-pin.env")).unwrap_or_default();
    pin.lines()
        .find_map(|line| line.strip_prefix("PCGEN_ORACLE_SHA="))
        .map(|rest| rest.split_whitespace().next().unwrap_or("").to_string())
        .unwrap_or_default()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = corpus_root();
    let pathfinder = root.join("pathfinder");
    if !pathfinder.is_dir() {
        return Err(format!("PCGEN_CORPUS_ROOT has no pathfinder/ directory: {}", root.display()).into());
    }
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");

    let mut files = Vec::new();
    collect_equipment_lsts(&pathfinder, &mut files)?;
    files.sort();

    let mut records = Vec::new();
    for file in &files {
        let text = String::from_utf8_lossy(&std::fs::read(file)?).into_owned();
        records.extend(parse_equipment_types(&text));
    }
    let merged = merge_equipment_types(records);

    let document = serde_json::json!({
        "oracle_sha": pinned_oracle_sha(&repo_root),
        "source": "PCGen equipment .lst TYPE: tokens under data/pathfinder",
        "files_read": files.len(),
        "identity_conflicts_first_wins": merged.conflicts,
        "types": merged.types,
    });
    let output = repo_root.join("data/equipment_types.json");
    std::fs::write(&output, serde_json::to_string_pretty(&document)? + "\n")?;
    println!(
        "wrote {}: {} identities from {} files ({} duplicate identities disagreed; first kept)",
        output.display(),
        merged.types.len(),
        files.len(),
        merged.conflicts
    );
    Ok(())
}
