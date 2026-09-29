//! SD-36 Epic F3c5 probe: enumerate every `CATEGORY:Internal` natural-attack helper row the pinned
//! tree declares, how `natural_attack::scan` classifies it, and every converted reference to one.
//! Run: copy to crates/codex-ingest/tests/zz_probe_f3c5.rs;
//! `cargo test --locked -j 8 -p codex-ingest --test zz_probe_f3c5 -- --nocapture --test-threads=8`;
//! delete the copy.
use std::collections::{BTreeMap, BTreeSet};

use codex::rules_core::sheet_rule::*;
use codex_ingest::pcgen_import::sheet_rule::closure::{corpus_root, row_identity, tokenize_row, FileFamily, PinnedTree, RowShape};
use codex_ingest::pcgen_import::sheet_rule::natural_attack;
use codex_ingest::pcgen_import::sheet_rule::{build_index, load_population, read_output};

#[test]
fn f3c5_helper_enumeration() {
    let tree = PinnedTree::load(&corpus_root()).expect("pinned tree");
    let repo = codex_ingest::repo_root();
    // Every Internal natural-attack-typed row, by shape, pfs or not, owned or not.
    let mut all = BTreeMap::<String, usize>::new();
    for file in &tree.files {
        if file.family != FileFamily::Ability {
            continue;
        }
        for raw in &file.lines {
            let id = row_identity(raw);
            if id.category != "INTERNAL" {
                continue;
            }
            let (_, toks) = tokenize_row(raw);
            if !toks.iter().any(|(k, v)| k == "TYPE" && v.split('.').any(|t| t.trim().eq_ignore_ascii_case("NaturalAttack"))) {
                continue;
            }
            let shape = match id.shape {
                RowShape::Plain => "plain",
                RowShape::Copy(_) => "copy",
                RowShape::Mod => "mod",
                _ => "other",
            };
            *all.entry(format!("{shape}{}", if file.is_pfs { " (_pfs)" } else { "" })).or_default() += 1;
        }
    }
    println!("internal natural-attack-typed rows in the pinned tree (ability family), by shape: {all:?}");
    let records = load_population(&repo, &tree).expect("population");
    let (index, _closures) = build_index(&tree, records);
    let answered = |cat: &str, key: &str| {
        let pair = (cat.to_string(), key.to_string());
        index.by_cat_key.contains_key(&pair) || index.by_cat_name.contains_key(&pair)
    };
    let owned = |row| index.row_owner.contains_key(&row);
    let scan = natural_attack::scan(&tree, &owned, &answered);
    println!("helpers no unit stands for (plain, outside _pfs): {}", scan.declared);
    let classified: BTreeSet<_> = scan.helpers.values().map(|h| h.row).collect();
    println!("classified natural-attack-only: {} rows, answering {} (Internal, key-or-name) pairs", classified.len(), scan.helpers.len());
    for (k, v) in &scan.defects {
        println!("defect {k}: {} rows", v.len());
        for l in v.iter().take(20) {
            println!("  {l}");
        }
    }
    let mut by_attacks = BTreeMap::<String, usize>::new();
    for h in scan.helpers.values() {
        *by_attacks.entry(h.attacks.join("+")).or_default() += 1;
    }
    println!("pairs by attack: {by_attacks:?}");
    // Converted references (the package on disk).
    let files = read_output(&repo.join("data/sheet_rules"));
    let all_rules: Vec<SheetRule> = files
        .iter()
        .filter(|(rel, _)| !rel.starts_with('_') && rel.ends_with(".json"))
        .flat_map(|(_, b)| serde_json::from_slice::<Vec<SheetRule>>(b).expect("rule file"))
        .collect();
    let mut rules = 0usize;
    let mut effects = BTreeMap::<&str, usize>::new();
    let mut by_kind = BTreeMap::<String, usize>::new();
    let mut attacks = BTreeMap::<String, usize>::new();
    for r in &all_rules {
        let mut any = false;
        for e in &r.grants {
            let (tag, f) = match e {
                Effect::FactGrant(f) => ("FactGrant", f),
                Effect::GatedFactGrant { fact, .. } => ("GatedFactGrant", fact),
                _ => continue,
            };
            if let Fact::NaturalAttack(a) = f {
                *effects.entry(tag).or_default() += 1;
                *attacks.entry(a.clone()).or_default() += 1;
                any = true;
            }
        }
        if any {
            rules += 1;
            *by_kind.entry(r.id.split(':').nth(1).unwrap_or("").to_string()).or_default() += 1;
        }
    }
    println!("package: {rules} rules carry a NaturalAttack fact; effects {effects:?}; rules by kind {by_kind:?}");
    println!("facts by attack: {attacks:?}");
}
