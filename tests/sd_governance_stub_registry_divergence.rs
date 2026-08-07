//! Registry/artifact divergence guard for the `book_stub` kind.
//!
//! `docs/governance/wired-integration-stubs-registry.md` is the
//! operator-granted, hand-maintained ledger of deliberately-inert stubs.
//! `data/stubs/<book_id>.json` are the corresponding manifest artifacts.
//! Nothing previously tied these two together mechanically: a cycle could
//! add a stub manifest under `data/stubs/` without ever adding the matching
//! `### NNNN — `book_stub`: `<book_id>`` entry to the registry, and nothing
//! would fail. That is exactly what happened between SD-27 (last registry
//! commit) and SD-28/SD-30 (`ultimate_psionics`, then the twelve
//! `campaign_setting` books including `book_of_the_damned_volume_{1,2}` and
//! the ten `inner_sea_*` titles) — 13 manifests shipped with no registry
//! entry, discovered during a 2026-08-07 stub-registry audit.
//!
//! This test is the same shape as `Trap::WiringClassMismatch`: it fails
//! whenever a stored/declared set (the registry) disagrees with what is
//! actually on disk (the artifacts), rather than trusting either side.
//!
//! Note: this asserts set *equality*, not merely "no orphaned artifacts".
//! A registry entry with no backing artifact is caught too (defect in the
//! other direction) — as of 2026-08-07 that direction is clean.

use std::collections::BTreeSet;
use std::path::Path;

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

/// `book_id`s declared in the registry via `### NNNN — `book_stub`:
/// `<book_id>` ...` headers.
fn registered_book_stub_ids() -> BTreeSet<String> {
    let text = std::fs::read_to_string(
        repo_root().join("docs/governance/wired-integration-stubs-registry.md"),
    )
    .expect("wired-integration-stubs-registry.md must exist and be readable");

    let mut ids = BTreeSet::new();
    for line in text.lines() {
        let line = line.trim();
        // Matches lines like: ### 0003 — `book_stub`: `advanced_race_guide` not yet ingested
        let Some(after_kind) = line
            .strip_prefix("### ")
            .and_then(|rest| rest.split_once("`book_stub`:"))
            .map(|(_, rest)| rest.trim())
        else {
            continue;
        };
        let Some(after_open) = after_kind.strip_prefix('`') else {
            continue;
        };
        if let Some((id, _)) = after_open.split_once('`') {
            ids.insert(id.to_string());
        }
    }
    ids
}

/// `book_id`s that have a manifest artifact under `data/stubs/`.
fn artifact_book_stub_ids() -> BTreeSet<String> {
    let dir = repo_root().join("data/stubs");
    std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("data/stubs/ must be readable: {e}"))
        .filter_map(|entry| {
            let entry = entry.expect("readable data/stubs/ dir entry");
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string())
            } else {
                None
            }
        })
        .collect()
}

#[test]
fn registry_book_stub_entries_match_stub_artifacts_exactly() {
    let registered = registered_book_stub_ids();
    let artifacts = artifact_book_stub_ids();

    assert!(
        !registered.is_empty(),
        "sanity: expected at least one registered book_stub entry"
    );
    assert!(
        !artifacts.is_empty(),
        "sanity: expected at least one data/stubs/*.json artifact"
    );

    let unregistered: Vec<_> = artifacts.difference(&registered).cloned().collect();
    let orphaned_entries: Vec<_> = registered.difference(&artifacts).cloned().collect();

    assert!(
        unregistered.is_empty(),
        "data/stubs/ contains manifest(s) with no matching registry entry: {unregistered:?}\n\
         Every stub artifact must have a `### NNNN — `book_stub`: `<id>`` entry in \
         docs/governance/wired-integration-stubs-registry.md — see the no-stub-mvp-doctrine \
         'Per-cycle audit' section. Add the registry entry (operator-granted) or remove the \
         artifact if it should not exist."
    );
    assert!(
        orphaned_entries.is_empty(),
        "registry contains book_stub entry/entries with no matching data/stubs/*.json artifact: \
         {orphaned_entries:?}\nEither the artifact was deleted without updating the registry, or \
         the registry entry is stale."
    );
}
