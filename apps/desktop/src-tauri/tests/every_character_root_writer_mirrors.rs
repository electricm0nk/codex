//! Every write under a character root reaches the campaign copies (SD-41 `decisions.md §29`).
//!
//! The campaign write-through is one call, `after_write(root)`, that each writer makes after its
//! write. A writer that forgets it leaves every campaign folder silently stale, and no other test
//! would notice. This test reads the writers' source the way `packaged_resources.rs` reads
//! `tauri.conf.json`: it finds every `fs::write`, `fs::rename` or `atomic_write` whose target
//! derives from a character root (`root.join(…)`, `character_root(…)`, `…_at_root(…)`), every
//! `equipment_sets::save(…)` and `save_memberships(…)` call, and fails naming each live function
//! that holds one and does not call `after_write`. A live `SavedCharacterStore::save(` is refused
//! too: the desktop saves through `save_and_mirror`, which keeps the mirror's warnings.

use std::fs;
use std::path::PathBuf;

/// The files that write under a character root.
const WRITER_FILES: &[&str] = &["src/character_hub.rs", "src/character_custom.rs", "src/character_ledger.rs"];

/// The files that hold a live save of the envelope (`git grep 'SavedCharacterStore::save(' -- apps`).
const SAVE_FILES: &[&str] = &[
    "src/character_hub.rs",
    "src/character_custom.rs",
    "src/pf1_adapter.rs",
    "src/sf_adapter.rs",
    "src/sf_choices.rs",
    "src/sf_creation.rs",
    "src/sf_level_up.rs",
    "src/characterHub/reSaveCharacter.rs",
    "src/characterHub/appendToCharacter.rs",
];

/// The files whose live functions save a Cyberpunk RED character through `CprCharacterStore::save*`
/// (`git grep 'CprCharacterStore::save' -- apps/desktop/src-tauri/src`, test modules excluded).
const CPR_SAVE_FILES: &[&str] = &["src/cpr_adapter.rs", "src/cpr_creation.rs", "src/cpr_improve.rs"];

struct Function {
    name: String,
    body: Vec<String>,
}

/// The file's live functions: the text before its test module, less any `#[cfg(test)]` function.
fn live_functions(relative: &str) -> Vec<Function> {
    let text = fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative)).unwrap_or_else(|err| panic!("{relative}: {err}"));
    let lines: Vec<&str> = text.lines().collect();
    let end = (1..lines.len())
        .find(|&i| lines[i - 1].trim() == "#[cfg(test)]" && ["mod ", "pub(crate) mod "].iter().any(|prefix| lines[i].trim_start().starts_with(prefix)))
        .unwrap_or(lines.len());
    let mut functions: Vec<(Function, bool)> = Vec::new();
    for (i, line) in lines[..end].iter().enumerate() {
        if let Some(name) = fn_name(line) {
            let test_only = lines[..i].iter().rev().take_while(|l| l.trim().starts_with("#[") || l.trim().starts_with("///")).any(|l| l.trim() == "#[cfg(test)]" || l.trim() == "#[test]");
            functions.push((Function { name, body: Vec::new() }, test_only));
        } else if let Some((function, _)) = functions.last_mut() {
            function.body.push((*line).to_owned());
        }
    }
    functions.into_iter().filter(|(_, test_only)| !test_only).map(|(function, _)| function).collect()
}

fn fn_name(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    let rest = ["pub(crate) fn ", "pub(super) fn ", "pub fn ", "fn "].iter().find_map(|prefix| trimmed.strip_prefix(prefix))?;
    Some(rest.split(['(', '<']).next()?.to_owned())
}

/// The variable a write call targets (`fs::write(&path, …)` → `path`).
fn first_argument(line: &str, call: &str) -> Option<String> {
    let after = &line[line.find(call)? + call.len()..];
    Some(after.trim_start_matches('&').split(|c: char| !(c.is_alphanumeric() || c == '_')).next()?.to_owned())
}

fn root_derived(text: &str) -> bool {
    text.contains("root.join(") || text.contains("character_root(") || text.contains("_at_root(")
}

/// The live functions in `relative` that write under a character root and never call `after_write`.
fn unmirrored_writers(relative: &str) -> Vec<String> {
    let mut missing = Vec::new();
    for function in live_functions(relative) {
        let writes = function.body.iter().any(|line| {
            if line.contains("equipment_sets::save(") || line.contains("save_memberships(") {
                return true;
            }
            ["fs::write(", "fs::rename(", "atomic_write"].iter().any(|call| {
                line.contains(call)
                    && (root_derived(line)
                        || first_argument(line, call).is_some_and(|target| function.body.iter().any(|l| l.contains(&format!("let {target} = ")) && root_derived(l))))
            })
        });
        if writes && !function.body.iter().any(|line| line.contains("after_write(")) {
            missing.push(format!("{relative}: {}", function.name));
        }
    }
    missing
}

#[test]
fn every_character_root_writer_mirrors() {
    let missing: Vec<String> = WRITER_FILES.iter().flat_map(|file| unmirrored_writers(file)).collect();
    let bare_saves: Vec<String> = SAVE_FILES
        .iter()
        .flat_map(|file| {
            live_functions(file)
                .into_iter()
                .filter(|function| function.body.iter().any(|line| line.contains("SavedCharacterStore::save(")))
                .map(move |function| format!("{file}: {}", function.name))
        })
        .collect();
    let cpr_unmirrored: Vec<String> = CPR_SAVE_FILES
        .iter()
        .flat_map(|file| {
            live_functions(file)
                .into_iter()
                .filter(|function| function.body.iter().any(|line| line.contains("CprCharacterStore::save")) && !function.body.iter().any(|line| line.contains("after_write(")))
                .map(move |function| format!("{file}: {}", function.name))
        })
        .collect();
    let cpr_found: usize = CPR_SAVE_FILES.iter().map(|file| live_functions(file).iter().filter(|f| f.body.iter().any(|l| l.contains("CprCharacterStore::save"))).count()).sum();
    // The scan is only as good as its reach: the writers it knows must still be there to find.
    let found: usize = WRITER_FILES.iter().map(|file| live_functions(file).iter().filter(|f| f.body.iter().any(|l| l.contains("after_write("))).count()).sum();
    assert!(missing.is_empty(), "these functions write under a character root and do not call after_write: {missing:#?}");
    assert!(bare_saves.is_empty(), "these functions save with SavedCharacterStore::save, which drops the campaign warnings; use save_and_mirror: {bare_saves:#?}");
    assert!(cpr_unmirrored.is_empty(), "these functions save a Cyberpunk RED character and do not call after_write: {cpr_unmirrored:#?}");
    assert!(cpr_found >= 5, "only {cpr_found} functions save a Cyberpunk RED character; the scan lost the writers");
    assert!(found >= 6, "only {found} functions call after_write; the scan lost the writers");
}
