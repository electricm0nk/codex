//! SD-20 Epic 3 foundation slice: CRB feat catalog structural proof.
//!
//! `src/rules_core/rules_tables/crb/` had no feat catalog at all (no
//! `feats.rs`/`feat_data/`), unlike equipment (`equipment_tables.rs` +
//! `equipment_data/`) and spells (`spell_list.rs`), blocking Epic 3
//! (feat prerequisite engine) cycles before they could even start. This
//! test mirrors `sd19_table_store_foundation.rs`'s structural-proof
//! pattern for the new catalog: it asserts the catalog is present,
//! non-empty per category, and internally consistent (no fabricated
//! rows) — it does not implement or exercise prerequisite logic, which
//! is Epic 3's own future job.
//!
//! The second test cross-checks the transcribed catalog against the live
//! PCGen corpus file directly (the same `CORPUS_ROOT`-gated pattern used
//! by `sd19_equipment_arms_armor.rs`'s 310-record check), so corpus drift
//! is caught rather than silently trusted.

use std::path::PathBuf;

use codex::rules_core::rules_tables::crb::feats::{feat_tables, FeatCategory, FeatTableEntry};

#[test]
fn every_feat_category_is_non_empty() {
    for category in FeatCategory::ALL {
        let count = feat_tables().iter().filter(|f| f.category == *category).count();
        assert!(
            count >= 1,
            "feat category {:?} has zero feats in feat_tables()",
            category
        );
    }
}

#[test]
fn feat_tables_has_expected_total_and_per_category_counts() {
    // Per feats.rs's own doc comment: 185 total (General 50, Combat 110,
    // ItemCreation 8, Metamagic 17). If this drifts, either the corpus
    // changed or feat_data/ needs regenerating -- see feats.rs's doc
    // comment for the TYPE:-facet derivation rule.
    let all = feat_tables();
    assert_eq!(all.len(), 185, "expected 185 total transcribed feat records");

    let count_of = |category: FeatCategory| all.iter().filter(|f| f.category == category).count();
    assert_eq!(count_of(FeatCategory::General), 50);
    assert_eq!(count_of(FeatCategory::Combat), 110);
    assert_eq!(count_of(FeatCategory::ItemCreation), 8);
    assert_eq!(count_of(FeatCategory::Metamagic), 17);
}

#[test]
fn key_falls_back_to_name_for_every_entry() {
    // None of the 185 in-scope cr_feats.lst records carry an explicit
    // KEY: token (the one corpus record that does, "Cleave (Granted by
    // Sylvan Scimitar)", has no TYPE: facet and is excluded -- see
    // feats.rs's module doc comment), so key must equal name everywhere
    // in this catalog today, matching EquipmentTableEntry's documented
    // fallback rule.
    for entry in feat_tables() {
        assert_eq!(
            entry.key, entry.name,
            "expected key to fall back to name for '{}' (no corpus KEY: token \
             is transcribed into this catalog today)",
            entry.name
        );
    }
}

#[test]
fn well_known_feats_are_queryable_by_key_in_their_documented_category() {
    let all = feat_tables();
    let find = |key: &str| all.iter().find(|f: &&FeatTableEntry| f.key == key);

    let acrobatic = find("Acrobatic").expect("Acrobatic must be in the catalog");
    assert_eq!(acrobatic.category, FeatCategory::General);
    assert_eq!(
        acrobatic.description,
        Some("You are skilled at leaping, jumping, and flying.")
    );

    let power_attack = find("Power Attack").expect("Power Attack must be in the catalog");
    assert_eq!(power_attack.category, FeatCategory::Combat);

    let craft_wand = find("Craft Wand").expect("Craft Wand must be in the catalog");
    assert_eq!(craft_wand.category, FeatCategory::ItemCreation);

    let empower_spell = find("Empower Spell").expect("Empower Spell must be in the catalog");
    assert_eq!(empower_spell.category, FeatCategory::Metamagic);

    // "Heighten Spell +2" carries no corpus DESC: token -- the catalog
    // honestly records that absence as None rather than fabricating text.
    let heighten_plus_2 = find("Heighten Spell +2").expect("Heighten Spell +2 must be in the catalog");
    assert_eq!(heighten_plus_2.category, FeatCategory::Metamagic);
    assert_eq!(heighten_plus_2.description, None);
}

#[test]
fn duplicate_corpus_records_are_preserved_verbatim_not_deduplicated() {
    // "Combat Expertise" appears twice in cr_feats.lst (two distinct rule
    // variants with different PREMULT/BONUS formulas) -- the catalog
    // preserves both rows rather than collapsing them, mirroring
    // EquipmentTableEntry's own no-dedup discipline.
    let count = feat_tables()
        .iter()
        .filter(|f| f.name == "Combat Expertise")
        .count();
    assert_eq!(count, 2, "expected both real corpus 'Combat Expertise' variants to be present");
}

fn corpus_root() -> Option<PathBuf> {
    match std::env::var("CORPUS_ROOT") {
        Ok(value) => {
            let path = PathBuf::from(value);
            if path.is_dir() { Some(path) } else { None }
        }
        Err(_) => None,
    }
}

fn cr_feats_path(root: &std::path::Path) -> PathBuf {
    root.join("pathfinder/paizo/roleplaying_game/core_rulebook/cr_feats.lst")
}

/// Minimal inline re-derivation of the same `TYPE:`-facet classification
/// rule `feats.rs`'s doc comment describes, applied directly to the raw
/// corpus text -- deliberately not sharing code with the generator, so
/// this test would actually fail if the transcribed data drifted from
/// the live corpus.
fn facet_category(type_value: &str) -> Option<&'static str> {
    let has_facet = |name: &str| type_value.split('.').any(|facet| facet == name);
    if has_facet("Combat") {
        Some("Combat")
    } else if has_facet("General") {
        Some("General")
    } else if has_facet("Metamagic") {
        Some("Metamagic")
    } else if has_facet("ItemCreation") {
        Some("ItemCreation")
    } else {
        None
    }
}

#[test]
fn catalog_matches_live_corpus_type_facet_counts() {
    let Some(root) = corpus_root() else {
        eprintln!(
            "CORPUS_ROOT not set or not a directory; skipping (set \
             CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data to enable)"
        );
        return;
    };
    let cr_feats = cr_feats_path(&root);
    if !cr_feats.is_file() {
        eprintln!("canonical cr_feats.lst not present at {}; skipping", cr_feats.display());
        return;
    }

    let text = std::fs::read_to_string(&cr_feats).expect("cr_feats.lst must be readable");
    let mut corpus_names: Vec<String> = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() || line.starts_with('#') || line.starts_with("###Block") {
            continue;
        }
        let fields: Vec<&str> = line.split('\t').collect();
        let name = fields[0].trim();
        if name.is_empty() || name.starts_with("SOURCELONG") || name.starts_with("CATEGORY=") {
            continue;
        }
        let type_field = fields
            .iter()
            .skip(1)
            .find_map(|f| f.trim().strip_prefix("TYPE:"));
        let Some(type_value) = type_field else {
            continue;
        };
        if facet_category(type_value).is_some() {
            corpus_names.push(name.to_string());
        }
    }

    assert_eq!(
        corpus_names.len(),
        185,
        "live corpus TYPE:-facet-classifiable record count drifted from the \
         185 this catalog was generated from; regenerate feat_data/"
    );

    let catalog = feat_tables();
    for name in &corpus_names {
        assert!(
            catalog.iter().any(|entry| entry.name == name),
            "corpus record '{name}' is classifiable but missing from feat_tables()"
        );
    }
}
