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

use codex::rules_core::rules_tables::crb::feats::{feat_tables, FeatCategory, FeatEffectBonus, FeatTableEntry};

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

#[test]
fn great_fortitude_carries_its_real_save_bonus_token() {
    // `KEY:Great Fortitude` in cr_feats.lst carries a single verbatim
    // `BONUS:SAVE|Fortitude|2` token -- this asserts the effect field
    // captures that real numeric mechanical delta, not just prose.
    let all = feat_tables();
    let great_fortitude = all
        .iter()
        .find(|f| f.key == "Great Fortitude")
        .expect("Great Fortitude must be in the catalog");
    assert_eq!(
        great_fortitude.effect,
        Some(&[FeatEffectBonus {
            qualifiers: &["SAVE", "Fortitude", "2"]
        }] as &[FeatEffectBonus])
    );
}

#[test]
fn power_attack_carries_all_four_real_bonus_var_tokens() {
    // `KEY:Power Attack` (the SD-20 Epic 6 damage:feat_effect blocker's
    // own motivating example) carries four `BONUS:VAR|...` tokens in
    // cr_feats.lst, none of which are flat integer literals -- the
    // damage-bearing one
    // (`PowerAttackDamageModifier|PowerAttackDamageBase*floor(PowerAttackModifier)`)
    // is a formula over `BAB`, proving why `effect` cannot honestly be a
    // single resolved `i16`.
    let all = feat_tables();
    let power_attack = all
        .iter()
        .find(|f| f.key == "Power Attack")
        .expect("Power Attack must be in the catalog");
    let effect = power_attack.effect.expect("Power Attack must carry BONUS: tokens");
    assert_eq!(effect.len(), 4);
    assert_eq!(effect[0].qualifiers, &["VAR", "PowerAttackModifier", "(BAB/4)+1"]);
    assert_eq!(effect[1].qualifiers, &["VAR", "PowerAttackDamageBase", "2"]);
    assert_eq!(
        effect[2].qualifiers,
        &["VAR", "PowerAttackDamageModifier", "PowerAttackDamageBase*floor(PowerAttackModifier)"]
    );
    assert_eq!(
        effect[3].qualifiers,
        &["VAR", "MonkFlurryPowerAttackModifier", "BAB+(FlurryLVL-MonkBAB)"]
    );
}

#[test]
fn item_creation_feats_never_carry_a_bonus_token() {
    // Crafting feats' real mechanical effect is a prose crafting rule
    // (gp cost / time), never a `BONUS:` token in the real corpus --
    // this is an honest absence, not missing data.
    for entry in feat_tables().iter().filter(|f| f.category == FeatCategory::ItemCreation) {
        assert_eq!(
            entry.effect, None,
            "expected no BONUS: effect for ItemCreation feat '{}'",
            entry.name
        );
    }
}

#[test]
fn feat_effect_counts_match_the_live_corpus_bonus_token_census() {
    // 81 of the 185 catalogued records carry at least one real `BONUS:`
    // token, broken down per category as below -- if this drifts, either
    // the corpus changed or feat_data/ needs regenerating.
    let all = feat_tables();
    let with_effect = |category: FeatCategory| {
        all.iter()
            .filter(|f| f.category == category && f.effect.is_some())
            .count()
    };
    assert_eq!(with_effect(FeatCategory::General), 30);
    assert_eq!(with_effect(FeatCategory::Combat), 42);
    assert_eq!(with_effect(FeatCategory::Metamagic), 9);
    assert_eq!(with_effect(FeatCategory::ItemCreation), 0);
    assert_eq!(
        all.iter().filter(|f| f.effect.is_some()).count(),
        81,
        "expected 81 total feat records with at least one real BONUS: token"
    );
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

/// Whether a classifiable record's raw tab-delimited fields carry at
/// least one `BONUS:` token -- deliberately not sharing code with the
/// generator, so this test would fail if `effect` drifted from the live
/// corpus (same discipline `catalog_matches_live_corpus_type_facet_counts`
/// already applies to `key`/`category`/`name`).
fn record_has_bonus_token(fields: &[&str]) -> bool {
    fields.iter().skip(1).any(|f| f.trim().starts_with("BONUS:"))
}

#[test]
fn catalog_effect_presence_matches_live_corpus_bonus_tokens() {
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
    // (category, name, whether the record itself has a BONUS: token), in
    // file order per classifiable record. `feat_tables()` concatenates
    // one per-category table per category (see feats.rs's own
    // `feat_tables()`), each internally in file order -- not one single
    // whole-file order -- so this groups by category to match.
    let mut corpus_records: Vec<(&'static str, String, bool)> = Vec::new();
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
        if let Some(category) = facet_category(type_value) {
            corpus_records.push((category, name.to_string(), record_has_bonus_token(&fields)));
        }
    }

    let corpus_with_bonus = corpus_records.iter().filter(|(_, _, has_bonus)| *has_bonus).count();
    let catalog = feat_tables();
    let catalog_with_effect = catalog.iter().filter(|f| f.effect.is_some()).count();
    assert_eq!(
        catalog_with_effect, corpus_with_bonus,
        "catalog's effect-populated record count drifted from the live corpus's \
         BONUS:-token-carrying record count; regenerate feat_data/"
    );

    // Positional cross-check, per category (file order within each
    // category, matching feat_tables()'s own per-category concatenation
    // order): every corpus record's real BONUS:-presence agrees with the
    // catalog entry at the same position within its category.
    for category_name in ["General", "Combat", "ItemCreation", "Metamagic"] {
        let category = match category_name {
            "General" => FeatCategory::General,
            "Combat" => FeatCategory::Combat,
            "ItemCreation" => FeatCategory::ItemCreation,
            "Metamagic" => FeatCategory::Metamagic,
            _ => unreachable!(),
        };
        let corpus_in_category: Vec<&(&'static str, String, bool)> =
            corpus_records.iter().filter(|(cat, _, _)| *cat == category_name).collect();
        let catalog_in_category: Vec<&FeatTableEntry> =
            catalog.iter().filter(|f| f.category == category).collect();
        assert_eq!(
            catalog_in_category.len(),
            corpus_in_category.len(),
            "category {category_name} record count drifted"
        );
        for (entry, (_, name, has_bonus)) in catalog_in_category.iter().zip(corpus_in_category.iter()) {
            assert_eq!(&entry.name, name, "catalog/corpus order drifted within category {category_name}");
            assert_eq!(
                entry.effect.is_some(),
                *has_bonus,
                "'{name}' effect presence disagrees with live corpus BONUS: token presence"
            );
        }
    }
}
