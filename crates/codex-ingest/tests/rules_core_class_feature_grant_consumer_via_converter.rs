// -- split from `tests` in src/rules_core/pilot_compute/class_feature_grant_consumer.rs (pcgen-touching items only) --
mod tests {
    use codex::rules_core::pilot_compute::class_feature_grant_consumer::*;
    use std::collections::BTreeMap;
    use codex::rules_core::pilot_compute::AbilityModifiers;
    use codex::rules_core::record_vars::{self, ConvertedChain};

    /// SD-35 `AT-35-E6-003-RULED` cycle 16 -- every record this module serves renders the SAME
    /// sentence from its settled description that the request-time renderer produced from the
    /// stored one, under six value environments derived from each record's own slots.
    ///
    /// This is the oracle for the removal of the last converter call on the live side. It reads
    /// the real table, not a fixture roster, and it compares dropped arguments as well as text:
    /// a settling that silently resolved something the renderer refused would pass a text-only
    /// comparison and then print a guess on a sheet.
    #[test]
    fn every_served_record_renders_the_same_sentence_from_its_settled_description() {
        let table = class_feature_record_tokens_pre_gate_safe();
        let templates = &record_vars::package().desc_templates;
        let mut compared = 0usize;
        let mut with_a_slot = 0usize;
        let mut failures: Vec<String> = Vec::new();
        for (key, record) in table {
            let Some(template) = templates.get(key) else { continue };
            if record.raw_description.is_empty() {
                continue;
            }
            compared += 1;
            let names = template.args();
            if !names.is_empty() {
                with_a_slot += 1;
            }
            for which in 0..6usize {
                let mut env: BTreeMap<String, i64> = BTreeMap::new();
                for (index, name) in names.iter().enumerate() {
                    let bind = match which {
                        0 => false,
                        1 | 4 => true,
                        2 => index % 2 == 0,
                        3 => index % 2 == 1,
                        _ => !name.contains(['+', '-']),
                    };
                    if bind {
                        let value = match which {
                            4 => -7,
                            5 => 0,
                            _ => i64::try_from(index).unwrap_or(0) + 3,
                        };
                        env.insert(name.clone(), value);
                    }
                }
                let mut values = codex_ingest::pcgen_import::pcgen_desc::PcgenDisplayValues::new();
                for (name, value) in &env {
                    values.set(name, *value);
                }
                let ours = template.render(&env);
                let theirs = codex_ingest::pcgen_import::pcgen_desc::render_pcgen_desc_with_values(
                    &record.raw_description,
                    &values,
                );
                if ours.text != theirs.text || ours.dropped_args != theirs.dropped_args {
                    failures.push(format!(
                        "{key} probe={which}\n  settled : {:?} / {:?}\n  renderer: {:?} / {:?}",
                        ours.text, ours.dropped_args, theirs.text, theirs.dropped_args
                    ));
                }
            }
        }
        assert!(
            compared >= 3_000 && with_a_slot >= 200,
            "the compared population collapsed: compared={compared} with_a_slot={with_a_slot}"
        );
        assert!(
            failures.is_empty(),
            "{} of {compared} served records render a different sentence once settled:\n{}",
            failures.len(),
            failures.iter().take(20).cloned().collect::<Vec<_>>().join("\n")
        );
    }

    /// SD-32 T12 Epic 8 row 18 cycle 10. The shared merge policy both
    /// cross-book tables now use: a target seen in an earlier book is never
    /// overwritten by a later book's own value for the same target name,
    /// but a target the earlier book never defined at all DOES get pulled
    /// in from a later book -- proving the exact defect the old whole-
    /// record `or_insert_with` (first book wins ENTIRELY, even for targets
    /// it never carried) used to have, on both tables, before cycle 8/10.
    #[test]
    fn merge_bonus_var_target_map_pulls_in_new_targets_but_never_overwrites_a_seen_one() {
        let mut first: BTreeMap<String, String> = BTreeMap::new();
        first.insert("SharedTarget".to_string(), "1".to_string());
        first.insert("OnlyFirstBook".to_string(), "1".to_string());
        let mut second: BTreeMap<String, String> = BTreeMap::new();
        second.insert("SharedTarget".to_string(), "2".to_string());
        second.insert("OnlySecondBook".to_string(), "2".to_string());
        let mut into = lowered_chain_for_test(&first);
        let from = lowered_chain_for_test(&second);
        let first_book_value = into.get("SharedTarget").cloned();

        merge_bonus_var_target_map_never_overwriting(&mut into, from);

        assert_eq!(
            into.get("SharedTarget").cloned(),
            first_book_value,
            "a target already bound by an earlier book must never be overwritten by a later one"
        );
        assert!(into.contains_key("OnlyFirstBook"));
        assert!(
            into.contains_key("OnlySecondBook"),
            "a target the earlier book never defined must still merge in from a later book -- \
             this is the exact behaviour the old whole-record `or_insert_with` lacked"
        );
    }

    /// `Assassin ~ Save against Poisons` (`core_rulebook`, real corpus record): a single
    /// `BONUS:VAR|AssassinPoisonSaveBonus|AssassinLVL/2` token, no chain hop needed at all --
    /// the simplest real shape this resolver handles.
    #[test]
    fn resolve_pcgen_var_chain_reproduces_a_single_hop_division_formula() {
        let mut bonus_vars = BTreeMap::new();
        bonus_vars.insert("AssassinPoisonSaveBonus".to_string(), "AssassinLVL/2".to_string());
        for (level, expected) in [(2u8, 1i64), (3, 1), (4, 2), (10, 5), (20, 10)] {
            let vars =
                resolve_pcgen_var_chain(&lowered_chain_for_test(&bonus_vars), "AssassinLVL", level, &AbilityModifiers::default());
            assert_eq!(
                vars.get("AssassinPoisonSaveBonus"),
                Some(&expected),
                "level {level}"
            );
        }
    }

    /// `Rogue ~ Trapfinding` (`core_rulebook`, real corpus record): a TWO-hop chain
    /// (`TrapfindingLVL` -> `RogueLVL`, then `TrapfindingBonus` -> `max(TrapfindingLVL/2,1)`),
    /// the shape wave 25b's own worked example (`Bardic Knowledge`) also uses. Proves the
    /// fixed-point pass genuinely chains through an intermediate variable, not just a bare
    /// single-hop lookup.
    #[test]
    fn resolve_pcgen_var_chain_reproduces_a_two_hop_max_formula() {
        let mut bonus_vars = BTreeMap::new();
        bonus_vars.insert("TrapfindingLVL".to_string(), "RogueLVL".to_string());
        bonus_vars.insert("TrapfindingBonus".to_string(), "max(TrapfindingLVL/2,1)".to_string());
        for (level, expected) in [(1u8, 1i64), (2, 1), (3, 1), (4, 2), (5, 2), (10, 5), (20, 10)] {
            let vars =
                resolve_pcgen_var_chain(&lowered_chain_for_test(&bonus_vars), "RogueLVL", level, &AbilityModifiers::default());
            assert_eq!(vars.get("TrapfindingBonus"), Some(&expected), "level {level}");
        }
    }

    /// SD-32 T12 Epic 8 row 18 cycle 12 correction: this test's ORIGINAL body used a made-up,
    /// not-in-the-corpus name (`SiblingRecordOwnVariable`) to stand in for "an identifier the
    /// chain can never reach", asserting the whole formula stayed unbound. Reading the pinned
    /// oracle's own `VariableProcessor.java`/`PlayerCharacter.java` (this cycle's own receipt)
    /// proved that assumption wrong for a name genuinely absent from the corpus under every
    /// condition: real PCGen's `getVariable` -> `getVariableValue` -> `processBrokenParser`
    /// chain silently treats such a bare additive term as `0`, not a refusal. Corrected to prove
    /// the REAL safety property this test was reaching for -- a name that DOES exist elsewhere in
    /// the corpus as a `BONUS:VAR` target (here, `AssassinPoisonSaveBonus`, real record
    /// `data/corpus/core_rulebook/class_feature/assassin/save_against_poisons.json`) but is not
    /// reachable from THIS formula's own local `bonus_vars` map still refuses -- it is a REAL,
    /// possibly-conditional PCGen value this resolver cannot see from here, and must never guess.
    #[test]
    fn resolve_pcgen_var_chain_never_binds_an_identifier_bound_elsewhere_in_the_corpus() {
        let mut bonus_vars = BTreeMap::new();
        bonus_vars
            .insert("SomeBonus".to_string(), "10+(SomeLVL/2)+AssassinPoisonSaveBonus".to_string());
        bonus_vars.insert("SomeLVL".to_string(), "RogueLVL".to_string());
        let vars =
            resolve_pcgen_var_chain(&lowered_chain_for_test(&bonus_vars), "RogueLVL", 10, &AbilityModifiers::default());
        assert_eq!(vars.get("SomeLVL"), Some(&10));
        assert!(
            !vars.contains_key("SomeBonus"),
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   BONUS:VAR target
            "a formula referencing an identifier bound elsewhere in the corpus (a sibling record's \
             own real) must never resolve to a guessed number: {vars:?}"
        );
    }

    /// SD-32 T12 Epic 8 row 18 cycle 12: real PCGen's own 0-default for a bare identifier the
    /// corpus never binds ANYWHERE, under ANY condition (this cycle's own receipt traces the
    /// oracle's `VariableProcessor.java`/`PlayerCharacter.java` chain to it). `NeverBoundAnywhere`
    /// is not a real corpus name and appears in no fixture -- standing in for the shape, proven
    /// absent from `every_corpus_bound_bonus_var_target()`/`corpus_define_literal_defaults()` by
    /// construction (a name this test file invents can never appear in either live-corpus table).
    #[test]
    fn resolve_pcgen_var_chain_defaults_a_corpus_wide_unbound_identifier_to_zero() {
        // SD-35 `AT-35-E6-001` cycle 4: the defaultable set is DERIVED AT INGEST -- every name
        // any converted chain references, minus every name the corpus binds somewhere -- and
        // shipped in the artifact, instead of being decided per lookup at request time. So the
        // property must be stated with a name the corpus really leaves unbound rather than an
        // invented one: a name that appears in NO corpus record at all cannot reach this
        // resolver, so it is not a case this resolver has. Re-derive the set with
        // `jq -r '.var_defaults | keys[]' data/converted/record_vars.json`.
        let unbound = "ArcaneStrikeDamageBonus";
        assert!(
            record_vars::package().var_defaults.contains_key(unbound),
            "{unbound} must still be a real name the corpus references and binds nowhere"
        );
        let mut bonus_vars = BTreeMap::new();
        bonus_vars.insert("SomeBonus".to_string(), format!("10+(SomeLVL/2)+{unbound}"));
        bonus_vars.insert("SomeLVL".to_string(), "RogueLVL".to_string());
        let vars = resolve_pcgen_var_chain(
            &lowered_chain_for_test(&bonus_vars),
            "RogueLVL",
            10,
            &AbilityModifiers::default(),
        );
        assert_eq!(vars.get("SomeLVL"), Some(&10));
        assert_eq!(
            vars.get("SomeBonus"),
            Some(&15),
            "10 + (10/2) + 0 = 15 -- a genuinely corpus-wide-unbound identifier is the rule source's own \
             real 0, not a refusal: {vars:?}"
        );
    }

    /// SD-32 T12 Epic 8 row 18 cycle 12: the concrete, real corpus case this cycle's fix targets
    /// -- `data/corpus/advanced_class_guide/class_feature/bloodrager/bloodrager_bloodline_
    /// tracker.json` carries `DEFINE:BloodragerBloodlinePower1LVLBonus|0` and no corpus
    /// `BONUS:VAR` row ever targets that same name, so this resolves through the DEFINE-literal
    /// path (`corpus_define_literal_defaults`), not the bare-0-fallback path -- both land on the
    /// same real number here, but this proves the more precise mechanism actually fires.
    #[test]
    fn resolve_pcgen_var_chain_binds_a_real_corpus_define_zero_baseline() {
        let mut bonus_vars = BTreeMap::new();
        bonus_vars.insert(
            "Bloodrager_Draconic_BloodlinePower1LVL".to_string(),
            "Bloodrager_Draconic_BloodlineLVL+BloodragerBloodlinePower1LVLBonus".to_string(),
        );
        bonus_vars
            .insert("Bloodrager_Draconic_BloodlineLVL".to_string(), "BloodragerLVL".to_string());
        let vars =
            resolve_pcgen_var_chain(&lowered_chain_for_test(&bonus_vars), "BloodragerLVL", 7, &AbilityModifiers::default());
        assert_eq!(
            vars.get("Bloodrager_Draconic_BloodlinePower1LVL"),
            Some(&7),
            "Bloodrager_Draconic_BloodlineLVL (7) + BloodragerBloodlinePower1LVLBonus (real \
             corpus DEFINE, 0) = 7: {vars:?}"
        );
    }

    /// SD-31 wave 27: an ability-modifier-dependent formula (the EXACT real shape `Rogue ~
    /// Master Strike`'s corpus row carries, `10+(MasterStrikeLVL/2)+INT`) now resolves once the
    /// character's real `AbilityModifiers` are seeded -- this is the widening the prior test's
    /// old body (before this wave) proved deliberately did NOT happen.
    #[test]
    fn resolve_pcgen_var_chain_now_binds_a_real_ability_modifier() {
        let mut bonus_vars = BTreeMap::new();
        bonus_vars.insert("MasterStrikeDC".to_string(), "10+(MasterStrikeLVL/2)+INT".to_string());
        bonus_vars.insert("MasterStrikeLVL".to_string(), "RogueLVL".to_string());
        let ability_modifiers = AbilityModifiers { intelligence: 3, ..AbilityModifiers::default() };
        let vars = resolve_pcgen_var_chain(&lowered_chain_for_test(&bonus_vars), "RogueLVL", 20, &ability_modifiers);
        assert_eq!(vars.get("MasterStrikeLVL"), Some(&20));
        assert_eq!(
            vars.get("MasterStrikeDC"),
            Some(&23),
            "10 + (20/2) + 3 = 23, real production INT modifier now seeded: {vars:?}"
        );
        // A DIFFERENT ability abbreviation the formula does not reference is seeded too (all six
        // always are, see `ability_modifier_seed_vars`) but changing it must not move this
        // formula's own result.
        let ability_modifiers_wis_only =
            AbilityModifiers { intelligence: 3, wisdom: 99, ..AbilityModifiers::default() };
        let vars2 = resolve_pcgen_var_chain(&lowered_chain_for_test(&bonus_vars), "RogueLVL", 20, &ability_modifiers_wis_only);
        assert_eq!(vars2.get("MasterStrikeDC"), Some(&23), "an unreferenced WIS seed must not leak in");
    }

    /// Lower a `name -> source formula` map the way the ingest-time converter does, so a test
    /// that states its case in the source form still exercises the REAL conversion and the REAL
    /// fold (SD-35 `AT-35-E6-001` cycle 4). A formula the converter refuses is absent from the
    /// result, exactly as it is absent from the shipped artifact.
    pub(crate) fn lowered_chain_for_test(formulas: &BTreeMap<String, String>) -> ConvertedChain {
        let mut out = ConvertedChain::new();
        for (name, formula) in formulas {
            let Ok(lowered) = codex_ingest::pcgen_import::class_feature_vars::lower_formula(formula)
            else {
                continue;
            };
            out.insert(
                name.clone(),
                codex::rules_core::record_vars::ConvertedVar {
                    expr: lowered.expr,
                    refs: lowered.refs.into_iter().collect(),
                },
            );
        }
        out
    }


}
