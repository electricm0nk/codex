
/// SD-36 F3c4 measurement (kept copy: artifacts/epic-f/scripts/f3c4_pool_pass_scan.rs). Appended
/// temporarily to `src/rules_core/pilot_compute/pool_groups.rs` and run with
/// `cargo test --locked -j 8 --lib -- --test-threads=8 --nocapture f3c4_pool_pass_scan`, then removed.
/// For every pool the SD-32 generic pool-group pass prints, every selection that resolves to a
/// real corpus group, and every level 1..=20: the member values the pass resolves (its answer
/// before the F3c4 yield), whether the selection links to a held pick option (the pass then
/// yields), and -- for linked selections -- whether the held set holds each printed member's
/// converted rule at that level. Rows: POOL|...; LINE|...
#[cfg(test)]
mod f3c4_pool_pass_scan {
    use super::*;
    use crate::rules_core::character_input::{load_character_input_fixture, SelectedChoice};
    use crate::rules_core::sheet_rule::{held_set, slug, CharacterFacts, HeldSeed};
    use std::collections::{BTreeMap, BTreeSet};

    const FIXTURE: &str = include_str!("../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

    #[test]
    fn f3c4_pool_pass_scan() {
        let package = crate::rules_core::sheet_rule_package::package().as_ref().expect("package");
        let pools: [(&str, &str, &str, &str, &str); 6] = [
            (SORCERER_CLASS_ID, "Sorcerer", "Bloodline", "bloodline:", SORCERER_BLOODLINE_CHOICE_ID),
            (BLOODRAGER_CLASS_ID, "Bloodrager", "Bloodline", "bloodline:", BLOODRAGER_BLOODLINE_CHOICE_ID),
            (CLERIC_CLASS_ID, "Cleric", "Domain", "domain:", CLERIC_DOMAIN_CHOICE_ID),
            (SHAMAN_CLASS_ID, "Shaman", "Spirit", "spirit:", SHAMAN_SPIRIT_CHOICE_ID),
            (WARPRIEST_CLASS_ID, "Warpriest", "Blessing", "blessing:", WARPRIEST_BLESSING_CHOICE_ID),
            (CAVALIER_CLASS_ID, "Cavalier", "Order", "order:", CAVALIER_ORDER_CHOICE_ID),
        ];
        let groups: BTreeSet<String> = class_feature_grant_consumer::class_feature_record_tokens_pre_gate_safe()
            .keys()
            .filter_map(|k| k.split_once(" ~ ").map(|(g, _)| g.to_owned()))
            .collect();
        for (class_id, class, reg, ns, choice) in pools {
            let class_slug = class_id.strip_prefix("class:").unwrap();
            let mut selections: BTreeMap<String, String> = BTreeMap::new();
            for g in &groups {
                let suffix = format!(" {reg}");
                for cand in [class_feature_id_slug(g), class_feature_id_slug(g.strip_suffix(&suffix).unwrap_or(g))] {
                    if let Some(real) = real_pool_group_for_selection_slug(class, reg, &cand) {
                        selections.entry(real).or_insert(cand);
                    }
                }
            }
            let (mut n_sel, mut n_linked, mut before, mut after, mut shape, mut held_ok, mut no_rule, mut unlinked_lines) = (0, 0, 0, 0, 0, 0, 0, 0);
            for (group, sel_slug) in &selections {
                n_sel += 1;
                let mut linked_sel = false;
                for level in 1..=20u8 {
                    let mut input = load_character_input_fixture(FIXTURE).character_input.unwrap();
                    input.chosen.class_levels = vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
                    input.chosen.selected_choices.push(SelectedChoice { choice_set_id: choice.to_owned(), selection_id: format!("{ns}{sel_slug}") });
                    let mods = compute_pilot_base_chassis(&input).ability_modifiers;
                    let lines = generic_pool_group_member_lines(level, &mods, class, reg, sel_slug);
                    before += lines.len();
                    let linked = crate::rules_core::sheet_rule_package::linked_picks(&input)
                        .into_iter()
                        .any(|l| l.choice_set_id == choice && l.member == *sel_slug && l.option_held);
                    if !linked {
                        after += lines.len();
                        unlinked_lines += lines.len();
                        continue;
                    }
                    linked_sel = true;
                    let seed = HeldSeed { race: Some("human".into()), classes: vec![(class_slug.into(), i64::from(level))], ..HeldSeed::default() };
                    let mut facts = CharacterFacts { level: i64::from(level), race: Some("human".into()), class_levels: vec![(class_slug.into(), i64::from(level))], ..CharacterFacts::default() };
                    facts.choices.insert(choice.to_owned(), vec![(sel_slug.clone(), sel_slug.clone())]);
                    let facts = facts.with_linked_picks(package, &seed);
                    let held = held_set(package, &seed, &facts);
                    for l in &lines {
                        let ids = package.find_all("class_feature", &slug(&l.key));
                        let verdict = if ids.is_empty() {
                            no_rule += 1;
                            "no-converted-member-rule"
                        } else if ids.iter().any(|id| held.holds_itself(id)) {
                            held_ok += 1;
                            "held"
                        } else {
                            shape += 1;
                            "NOT-HELD"
                        };
                        println!("LINE|{class}|{group}|{level}|{}|{}|{}|{verdict}", l.key, l.target, l.value);
                    }
                }
                if linked_sel {
                    n_linked += 1;
                }
            }
            println!(
                "POOL|{class} {reg}|selections {n_sel}|linked+held {n_linked}|lines before {before}|lines after {after}|linked lines: held {held_ok}, NOT-HELD {shape}, no converted member rule {no_rule}|unlinked lines still printed {unlinked_lines}"
            );
        }
    }
}
