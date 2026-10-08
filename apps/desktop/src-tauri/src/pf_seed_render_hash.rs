//! PF seed render-hash harness (SD-37 E1.4; reused unchanged by E4.MC, E6.MC, E4a.4, E7.2).
//!
//! Renders every Pathfinder starter seed through [`load_saved_character_at_root`] -- the body of
//! the `load_saved_character` Tauri command, i.e. the exact `LoadSavedCharacterResponse` the
//! desktop Character Sheet receives over IPC -- and serialises it with
//! `serde_json::to_string_pretty`. The bytes of that JSON are the "rendered sheet" whose sha256
//! the PF byte-identical gates compare across trees.
//!
//! Run (from `apps/desktop/src-tauri`):
//!
//! ```text
//! PF_SEED_RENDER_OUT=<dir> cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8 --nocapture
//! ```
//!
//! With `PF_SEED_RENDER_OUT` set it writes `<dir>/<seed>.json` and `<dir>/sha256.txt` (the
//! `sha256sum` format, so `cd <dir> && sha256sum -c sha256.txt` re-checks it). Without it the
//! test still renders and checks every seed.
//!
//! Each run seeds a fresh temp app-data directory (never the real app-data root). The sheet-rule
//! package and corpus come from the tree this binary was compiled in (the path is baked at
//! compile time), so a cross-tree comparison builds this file once per tree, each tree with its
//! own `CARGO_TARGET_DIR`. For a tree that predates this file: copy it into that tree's
//! `src/` and append the `mod pf_seed_render_hash` declaration found at the bottom of
//! `character_hub.rs`.

use super::{
    characters_root_from_app_data_dir, load_saved_character_at_root, seed_default_characters_at,
    DEFAULT_CHARACTER_ID, SECOND_SEED_CHARACTER_ID,
};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

/// The Pathfinder starter seeds, by output name and saved-character id.
const PF_SEEDS: [(&str, &str); 2] = [
    ("aldric", DEFAULT_CHARACTER_ID),
    ("elowen", SECOND_SEED_CHARACTER_ID),
];

/// A fresh, empty app-data directory under the OS temp dir, unique to this process and `tag`.
fn fresh_app_data_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "codex-pf-seed-render-{}-{tag}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp app-data dir is creatable");
    dir
}

/// Seed `app_data_dir` with the starter characters, then render each PF seed's sheet response
/// as pretty JSON, in [`PF_SEEDS`] order.
fn render_seed_sheets(app_data_dir: &Path) -> Vec<(&'static str, String)> {
    seed_default_characters_at(app_data_dir, "0.0.0-test").expect("starter seeding succeeds");
    let characters = characters_root_from_app_data_dir(app_data_dir);
    PF_SEEDS
        .iter()
        .map(|(name, id)| {
            let loaded = load_saved_character_at_root(&characters.join(id))
                .unwrap_or_else(|err| panic!("{name}: load_saved_character failed: {err}"));
            let json = serde_json::to_string_pretty(&loaded)
                .unwrap_or_else(|err| panic!("{name}: response serialises: {err}"));
            (*name, json)
        })
        .collect()
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

#[test]
fn pf_seed_render_hash_renders_every_seed_deterministically() {
    let first_dir = fresh_app_data_dir("a");
    let second_dir = fresh_app_data_dir("b");
    let first = render_seed_sheets(&first_dir);
    let second = render_seed_sheets(&second_dir);
    let _ = std::fs::remove_dir_all(&first_dir);
    let _ = std::fs::remove_dir_all(&second_dir);

    let names: Vec<&str> = first.iter().map(|(name, _)| *name).collect();
    assert_eq!(names, ["aldric", "elowen"]);

    for ((name, json), (_, again)) in first.iter().zip(&second) {
        // Same seed, two independent app-data roots: the render must not depend on the root
        // path, the process or the clock, or a hash comparison across trees means nothing.
        assert!(json == again, "{name}: two renders of the same seed differ");
        let value: serde_json::Value = serde_json::from_str(json).expect("rendered JSON parses");
        // The sheet-rule package must have loaded: a missing package renders a sheet with no
        // lines and a reason, and two trees that both failed to load it would hash equal.
        assert!(
            value["sheetRulesUnavailableReason"].is_null(),
            "{name}: sheet rules unavailable: {}",
            value["sheetRulesUnavailableReason"]
        );
        let lines = value["sheetLines"].as_array().map_or(0, Vec::len);
        assert!(lines > 0, "{name}: the rendered sheet carries no sheet lines");
    }

    println!("pf-seed-render manifest_dir={}", env!("CARGO_MANIFEST_DIR"));
    let out = std::env::var_os("PF_SEED_RENDER_OUT").map(PathBuf::from);
    if let Some(out) = &out {
        std::fs::create_dir_all(out).expect("PF_SEED_RENDER_OUT is creatable");
    }
    let mut manifest = String::new();
    for (name, json) in &first {
        let hash = sha256_hex(json.as_bytes());
        println!("pf-seed-render {name} sha256={hash} bytes={}", json.len());
        manifest.push_str(&format!("{hash}  {name}.json\n"));
        if let Some(out) = &out {
            std::fs::write(out.join(format!("{name}.json")), json).expect("seed JSON is writable");
        }
    }
    if let Some(out) = &out {
        std::fs::write(out.join("sha256.txt"), manifest).expect("sha256.txt is writable");
    }
}
