//! SD-36 F3c4b measurement (kept copy). Run: cp this file to tests/zz_probe_f3c4b.rs, then
//! `cargo test --locked -j 8 --test zz_probe_f3c4b -- --nocapture --test-threads=8`; delete it after.
//! Output rows: ROW|record id|label|granted edges|pick option|lines held via the pick at sorcerer 20
//! (of the record's granted edges)|lines held with the record alone (F3c4's probe)|levels Computed of 20|
//! claim-blocking ids.
use std::collections::BTreeMap;
use codex::rules_core::class_census::load_sweep_fixture;
use codex::rules_core::class_seeds::input_for;
use codex::rules_core::pilot_compute::build_pilot_headless_receipt;
use codex::rules_core::sheet_rule::{held_set, CharacterFacts, Granter, HeldSeed};
use codex::rules_core::sheet_rule_package;

const CHOOSER: &str = "core_rulebook:class_feature:sorcerer_standard_bloodline_selection";

#[test]
fn f3c4b_bloodline_sweep() {
    let package = sheet_rule_package::package().as_ref().expect("pkg");
    let fixture = load_sweep_fixture().expect("fixture");
    let mut records: Vec<String> = package.slugs_of_kind("class_feature").filter(|s| s.starts_with("sorcerer_bloodline_") && !s.starts_with("sorcerer_bloodline_feat_")).map(str::to_owned).collect();
    records.sort();
    println!("DENOM|{}", records.len());
    let (mut total_edges, mut total_pick, mut total_alone, mut total_late, mut total_checked) = (0, 0, 0, 0, 0);
    for slug in &records {
        let id = package.find("class_feature", slug).unwrap().clone();
        let rule = package.rule(&id).unwrap();
        let x = slug.strip_prefix("sorcerer_bloodline_").unwrap();
        let lines: Vec<String> = ["ability", "class_feature", "feat", "spell"].iter().flat_map(|k| package.rules_of_kind(k)).filter(|r| r.granted_by.iter().any(|g| matches!(&g.by, Granter::Rule(x) if x == &id))).map(|r| r.id.clone()).collect();
        // The pick option (a sorcerer_bloodline pool option granted by the chooser) that grants the record.
        let option: Option<String> = rule.granted_by.iter().filter_map(|g| match &g.by { Granter::Rule(o) => Some(o.clone()), _ => None }).find(|o| {
            package.rule(o).is_some_and(|r| r.pool == "sorcerer_bloodline" && r.granted_by.iter().any(|g| matches!(&g.by, Granter::Choice(c) if c == CHOOSER)))
        });
        // The census fixture's race is human; a pick row gated on another race (`PRERACE`,
        // converted as `Holds template:is<race>`) is probed with that race.
        let race: String = option.as_ref().and_then(|o| package.rule(o)).and_then(|r| {
            let s = format!("{:?}", r.applies);
            s.split("template:is").nth(1).map(|t| t.split('"').next().unwrap_or("human").to_string())
        }).unwrap_or_else(|| "human".into());
        let facts = |pick: Option<&String>| {
            let mut f = CharacterFacts { level: 20, race: Some(race.clone()), class_levels: vec![("sorcerer".into(), 20)], ..CharacterFacts::default() };
            if let Some(o) = pick { f.choices.insert(CHOOSER.into(), vec![(o.clone(), o.clone())]); }
            f
        };
        let seed = HeldSeed { race: Some(race.clone()), classes: vec![("sorcerer".into(), 20)], ..HeldSeed::default() };
        let via_pick = option.as_ref().map(|o| held_set(package, &seed, &facts(Some(o))));
        let alone = held_set(package, &HeldSeed { race: Some(race.clone()), classes: vec![("sorcerer".into(), 20)], rule_ids: vec![id.clone()], ..HeldSeed::default() }, &facts(None));
        let held_pick = via_pick.as_ref().map(|h| lines.iter().filter(|l| h.rules.contains_key(*l) && !h.removed.contains(*l)).count()).unwrap_or(0);
        let record_held = via_pick.as_ref().is_some_and(|h| h.rules.contains_key(&id));
        let held_alone = lines.iter().filter(|l| alone.rules.contains_key(*l)).count();
        total_edges += lines.len(); total_pick += held_pick; total_alone += held_alone;
        // Level-correctness: a line whose gate states `<progression var> >= N` (the level the
        // book grants it) must be held at sorcerer N via the pick.
        let mut late: Vec<String> = Vec::new();
        let mut level_checked = 0usize;
        if let Some(o) = option.as_ref() {
            for l in &lines {
                let Some(r) = package.rule(l) else { continue };
                let s = serde_json::to_value(&r.applies).unwrap();
                // The level the line's own gate states: `<... Progression LVL var> >= N`.
                let mut at: Option<i64> = None;
                fn walk(v: &serde_json::Value, pkg: &codex::rules_core::sheet_rule::SheetRulePackage, at: &mut Option<i64>) {
                    if let Some(c) = v.get("Compare") {
                        if let Some(var) = c.get("lhs").and_then(|l| l.get("Var")).and_then(|x| x.as_str())
                            && pkg.vars.get(var).is_some_and(|t| t.label.ends_with("Progression LVL"))
                            && c.get("op").and_then(|x| x.as_str()) == Some("Gte")
                        {
                            *at = c.get("rhs").and_then(|r| r.get("Const")).and_then(|x| x.as_i64());
                        }
                    }
                    match v { serde_json::Value::Object(m) => m.values().for_each(|x| walk(x, pkg, at)), serde_json::Value::Array(a) => a.iter().for_each(|x| walk(x, pkg, at)), _ => {} }
                }
                walk(&s, package, &mut at);
                let lvl = at.unwrap_or(1).max(1);
                level_checked += 1;
                let mut f = CharacterFacts { level: lvl, race: Some(race.clone()), class_levels: vec![("sorcerer".into(), lvl)], ..CharacterFacts::default() };
                f.choices.insert(CHOOSER.into(), vec![(o.clone(), o.clone())]);
                let h = held_set(package, &HeldSeed { race: Some(race.clone()), classes: vec![("sorcerer".into(), lvl)], ..HeldSeed::default() }, &f);
                if !h.rules.contains_key(l) { late.push(format!("{l}@{lvl}")); }
            }
        }
        total_checked += level_checked;
        total_late += late.len();
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
        let unheld: Vec<&String> = via_pick.as_ref().map(|h| lines.iter().filter(|l| !h.rules.contains_key(*l)).collect()).unwrap_or_default();
        println!("ROW|{id}|{race}|{}|{}|{}|{held_pick}|{record_held}|{held_alone}|{computed}|{}|UNHELD {:?}|NOT HELD AT ITS OWN LEVEL {:?}", rule.label, lines.len(), option.as_deref().unwrap_or("-"), bl.join("; "), unheld, late);
    }
    println!("TOTAL|edges {total_edges}|held via pick {total_pick}|held record alone {total_alone}|level-checked {total_checked}|not held at the level its own gate states (1 when none) {total_late}");
}
