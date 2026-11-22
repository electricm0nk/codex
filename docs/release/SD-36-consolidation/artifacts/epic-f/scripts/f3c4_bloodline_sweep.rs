//! SD-36 F3c4 measurement (kept copy). Run: cp this file to tests/zz_probe_f3c4.rs, then
//! `cargo test --locked -j 8 --test zz_probe_f3c4 -- --nocapture --test-threads=8`; delete it after.
//! Output rows: ROW|record id|label|granted edges|lines held beyond the record at sorcerer 20|
//! unresolved `Sorcerer Bloodline|<label>` refs|levels Computed of 20|claim-blocking ids.
use std::collections::BTreeMap;
use codex::rules_core::class_census::load_sweep_fixture;
use codex::rules_core::class_seeds::input_for;
use codex::rules_core::pilot_compute::build_pilot_headless_receipt;
use codex::rules_core::sheet_rule::{held_set, CharacterFacts, HeldSeed};
use codex::rules_core::sheet_rule_package;

#[test]
fn f3c4_bloodline_sweep() {
    let package = sheet_rule_package::package().as_ref().expect("pkg");
    let fixture = load_sweep_fixture().expect("fixture");
    let unresolved: Vec<String> = serde_json::from_str(&std::fs::read_to_string("data/sheet_rules/_defects/unresolved-references.json").unwrap()).unwrap();
    let mut records: Vec<String> = package.slugs_of_kind("class_feature").filter(|s| s.starts_with("sorcerer_bloodline_") && !s.starts_with("sorcerer_bloodline_feat_")).map(str::to_owned).collect();
    records.sort();
    println!("DENOM|{}", records.len());
    for slug in &records {
        let id = package.find("class_feature", slug).unwrap().clone();
        let rule = package.rule(&id).unwrap();
        let x = slug.strip_prefix("sorcerer_bloodline_").unwrap();
        // held-set: what the record's own granted lines reach at level 20
        let facts = CharacterFacts { level: 20, class_levels: vec![("sorcerer".into(), 20)], ..CharacterFacts::default() };
        let base = held_set(package, &HeldSeed { classes: vec![("sorcerer".into(), 20)], ..HeldSeed::default() }, &facts);
        let held = held_set(package, &HeldSeed { classes: vec![("sorcerer".into(), 20)], rule_ids: vec![id.clone()], ..HeldSeed::default() }, &facts);
        let new: Vec<&String> = held.rules.keys().filter(|k| !base.rules.contains_key(*k) && !k.starts_with(&id)).collect();
        let granted_edges = ["ability","class_feature","feat","spell"].iter().flat_map(|k| package.rules_of_kind(k)).filter(|r| r.granted_by.iter().any(|g| matches!(&g.by, codex::rules_core::sheet_rule::Granter::Rule(x) if x == &id))).count();
        let label = rule.label.clone();
        let pick_unresolved = unresolved.iter().filter(|u| u.ends_with(&format!("Sorcerer Bloodline|{label}"))).count();
        // census-style sweep
        let mut computed = 0; let mut blockers: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        for lvl in 1..=20u8 {
            let mut input = input_for(&fixture, "sorcerer", lvl);
            for c in input.chosen.selected_choices.iter_mut() {
                if c.choice_set_id == "choice:sorcerer_bloodline" { c.selection_id = format!("bloodline:{x}"); }
            }
            if x != "arcane" { input.chosen.selected_choices.retain(|c| c.choice_set_id != "choice:sorcerer_arcane_bond"); }
            let r = build_pilot_headless_receipt(&input);
            if format!("{:?}", r.status) == "Computed" { computed += 1; }
            for d in r.computation.diagnostics.iter().filter(|d| d.claim_blocking) { blockers.entry(d.id.clone()).or_default().push(lvl); }
        }
        let bl: Vec<String> = blockers.iter().map(|(k, v)| format!("{k} ({} of 20)", v.len())).collect();
        println!("ROW|{id}|{label}|{granted_edges}|{}|{pick_unresolved}|{computed}|{}", new.len(), bl.join("; "));
    }
}
