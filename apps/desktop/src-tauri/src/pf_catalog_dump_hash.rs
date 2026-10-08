//! PF catalog dump harness (SD-37 E4a.4): the "catalog outputs byte-identical" half of the
//! `rules_tables` removal gate, beside the seed render hash (`pf_seed_render_hash.rs`).
//!
//! Serialises the response of every zero-argument Pathfinder catalog / picker Tauri command --
//! the exact value the desktop receives over IPC -- with `serde_json::to_string_pretty`, and
//! prints one sha256 per command plus the Bestiary 1 monster count read off the monster catalog.
//!
//! Run (from `apps/desktop/src-tauri`):
//!
//! ```text
//! PF_CATALOG_DUMP_OUT=<dir> cargo test --locked -j 8 --bins pf_catalog_dump_hash -- --test-threads=8 --nocapture
//! ```
//!
//! With `PF_CATALOG_DUMP_OUT` set it writes `<dir>/<command>.json` and `<dir>/sha256.txt` (the
//! `sha256sum` format). The tables and corpus come from the tree this binary was compiled in, so
//! a cross-tree comparison builds it once per tree.
//!
//! Not dumped: commands that take arguments (`list_feats`, `list_spells`, `list_equipment`,
//! `list_reference_library_catalog`, `list_starfinder_catalog`, the class-spell-level and
//! creation commands), the Starfinder commands (no Starfinder file reads the rules tables), and
//! `corpus_ingest_diagnostic` (a build-tree report, not catalog content).

use sha2::{Digest, Sha256};
use std::path::PathBuf;

fn pretty<T: serde::Serialize>(name: &str, value: &T) -> String {
    serde_json::to_string_pretty(value).unwrap_or_else(|err| panic!("{name}: serialises: {err}"))
}

/// Every zero-argument PF catalog/picker command, by name, rendered to pretty JSON.
fn dump() -> Vec<(&'static str, String)> {
    vec![
        (
            "list_class_catalog",
            pretty(
                "list_class_catalog",
                &crate::class_catalog::list_class_catalog(),
            ),
        ),
        (
            "list_class_feature_descriptions",
            pretty(
                "list_class_feature_descriptions",
                &crate::class_feature_descriptions::list_class_feature_descriptions(),
            ),
        ),
        (
            "list_class_feature_feat_bridge_descriptions",
            pretty(
                "list_class_feature_feat_bridge_descriptions",
                &crate::class_feature_feat_bridge::list_class_feature_feat_bridge_descriptions(),
            ),
        ),
        (
            "list_class_feature_pool_options",
            pretty(
                "list_class_feature_pool_options",
                &crate::class_feature_pool_picker::list_class_feature_pool_options(),
            ),
        ),
        (
            "list_equipment_catalog",
            pretty(
                "list_equipment_catalog",
                &crate::equipment_catalog::list_equipment_catalog(),
            ),
        ),
        (
            "list_race_catalog",
            pretty(
                "list_race_catalog",
                &crate::race_catalog::list_race_catalog(),
            ),
        ),
        (
            "list_companion_catalog",
            pretty(
                "list_companion_catalog",
                &crate::companion_catalog::list_companion_catalog(),
            ),
        ),
        (
            "list_intelligent_item_catalog",
            pretty(
                "list_intelligent_item_catalog",
                &crate::intelligent_item_catalog::list_intelligent_item_catalog(),
            ),
        ),
        (
            "list_feat_catalog",
            pretty(
                "list_feat_catalog",
                &crate::feat_catalog::list_feat_catalog(),
            ),
        ),
        (
            "list_weapon_targets",
            pretty(
                "list_weapon_targets",
                &crate::feat_catalog::list_weapon_targets(),
            ),
        ),
        (
            "list_spell_catalog",
            pretty(
                "list_spell_catalog",
                &crate::spell_catalog::list_spell_catalog(),
            ),
        ),
        (
            "list_alternate_racial_traits",
            pretty(
                "list_alternate_racial_traits",
                &crate::race_trait_picker::list_alternate_racial_traits(),
            ),
        ),
        (
            "list_monster_catalog",
            pretty(
                "list_monster_catalog",
                &crate::monster_catalog::list_monster_catalog(),
            ),
        ),
        (
            "list_race_creation_roster",
            pretty(
                "list_race_creation_roster",
                &crate::character_hub::list_race_creation_roster(),
            ),
        ),
        (
            "list_available_character_traits",
            pretty(
                "list_available_character_traits",
                &crate::trait_picker::list_available_character_traits(),
            ),
        ),
    ]
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

#[test]
fn pf_catalog_dump_hash_renders_every_catalog_deterministically() {
    let first = dump();
    let second = dump();
    for ((name, json), (_, again)) in first.iter().zip(&second) {
        assert!(
            json == again,
            "{name}: two dumps of the same catalog differ"
        );
        assert!(json.len() > 2, "{name}: the catalog is empty");
    }

    // Bestiary 1's monsters: every catalog entry whose key is a `beastiary1:monster:` key.
    let monsters = crate::monster_catalog::list_monster_catalog();
    // Two predicates: the hand-modelled 46 (`beastiary1:monster:` keys) and every row the
    // catalog serves under Bestiary 1's book code (those 46 plus the chassis half).
    let bestiary1 = monsters
        .entries
        .iter()
        .filter(|entry| entry.key.starts_with("beastiary1:monster:"))
        .count();
    let book_b1 = monsters
        .entries
        .iter()
        .filter(|entry| entry.book == "B1")
        .count();
    assert!(
        bestiary1 > 0,
        "Bestiary 1 contributes monsters to the catalog"
    );
    println!(
        "pf-catalog-dump bestiary1_monsters={bestiary1} book_b1_entries={book_b1} monster_entries={}",
        monsters.entries.len()
    );

    let out = std::env::var_os("PF_CATALOG_DUMP_OUT").map(PathBuf::from);
    if let Some(out) = &out {
        std::fs::create_dir_all(out).expect("PF_CATALOG_DUMP_OUT is creatable");
    }
    let mut manifest = String::new();
    for (name, json) in &first {
        let hash = sha256_hex(json.as_bytes());
        println!("pf-catalog-dump {name} sha256={hash} bytes={}", json.len());
        manifest.push_str(&format!("{hash}  {name}.json\n"));
        if let Some(out) = &out {
            std::fs::write(out.join(format!("{name}.json")), json)
                .expect("catalog JSON is writable");
        }
    }
    if let Some(out) = &out {
        std::fs::write(out.join("sha256.txt"), manifest).expect("sha256.txt is writable");
        std::fs::write(
            out.join("bestiary1_monsters.txt"),
            format!(
                "beastiary1_keys={bestiary1} book_b1={book_b1} all={}\n",
                monsters.entries.len()
            ),
        )
        .expect("count is writable");
    }
}
