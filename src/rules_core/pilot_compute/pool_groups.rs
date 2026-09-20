#[allow(unused_imports)]
pub(crate) use super::*;

/// v0.6 alpha swarm, task #22 (2026-07-27): the eight `feat_effects`
/// producers that were built, tested, and consumed by nothing.
///
/// Only ONE has a real computed total to land on -- Toughness, whose
/// hit-point bonus goes into the Fighter level-1 hit-point record that
/// already named "Toughness / feat hit-point interplay" as its own
/// documented gap. The other seven target dimensions this engine
/// computes nowhere (initiative, non-skill checks, combat maneuvers,
/// unarmed strikes, movement), so they ground as standalone records
/// under the corrected bar rather than being left inert.
#[cfg(test)]
mod orphan_feat_producer_consumer_tests {
    use super::{build_pilot_headless_receipt, CharacterInput};
    use crate::rules_core::character_input::load_character_input_fixture;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    const WIZARD_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_wizard_level1_sd13_deterministic_input.txt"
    );

    fn fighter() -> CharacterInput {
        load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE)
            .character_input
            .expect("valid fixture")
    }

    fn wizard() -> CharacterInput {
        load_character_input_fixture(WIZARD_LEVEL_1_FIXTURE)
            .character_input
            .expect("valid fixture")
    }

    fn value(input: &CharacterInput, id: &str) -> Option<i16> {
        build_pilot_headless_receipt(input)
            .computation
            .explanations
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.value)
    }

    /// Toughness is the one orphan with a real total. Proven by
    /// differencing the same character with and without it, so the
    /// assertion cannot pass on a standalone record alone.
    #[test]
    fn toughness_actually_raises_the_real_fighter_hit_point_total() {
        let without = value(&fighter(), "class_chassis.fighter.level_1_hit_points")
            .expect("fighter level-1 hit points must be computed");

        let mut with = fighter();
        with.chosen.selected_feats.push("Toughness".to_owned());
        let raised = value(&with, "class_chassis.fighter.level_1_hit_points")
            .expect("hit points must still be computed");

        assert_eq!(raised - without, 3, "Toughness grants +3 hit points at low level");
    }

    /// The seven producers with no total to land on must still be
    /// consumed -- grounded as standalone records rather than left
    /// inert. Each keys on a feat the fixture does not have, so absence
    /// is the baseline.
    #[test]
    fn the_seven_totalless_producers_ground_standalone_records_when_their_feat_is_present() {
        for (feat, id, want) in [
            ("Improved Initiative", "feat.standalone.initiative_bonus", 4),
            ("Endurance", "feat.standalone.endurance_check_bonus", 4),
            ("Fleet", "feat.standalone.base_speed_bonus", 5),
            ("Nimble Moves", "feat.standalone.difficult_terrain_feet", 5),
        ] {
            let mut input = fighter();
            assert_eq!(value(&input, id), None, "{id} must be absent without {feat}");
            input.chosen.selected_feats.push(feat.to_owned());
            assert_eq!(value(&input, id), Some(want), "{feat} -> {id}");
        }
    }

    /// The APG/ACG passive-bonus widening (2026-07-29). Each of the four
    /// keys on a feat the fixture does not have, so absence is the
    /// baseline and the record's appearance proves live consumer wiring
    /// rather than an inert producer.
    #[test]
    fn the_apg_acg_passive_bonus_feats_ground_standalone_records_when_selected() {
        for (feat, id, want) in [
            ("Sharp Senses", "feat.standalone.sharp_senses_perception_bonus", 4),
            ("Steel Soul", "feat.standalone.steel_soul_save_vs_spells", 4),
            ("Deepsight", "feat.standalone.deepsight_darkvision_feet", 60),
        ] {
            let mut input = fighter();
            assert_eq!(value(&input, id), None, "{id} must be absent without {feat}");
            input.chosen.selected_feats.push(feat.to_owned());
            assert_eq!(value(&input, id), Some(want), "{feat} -> {id}");
        }
    }

    /// Steadfast Personality's magnitude is computed from the character's
    /// own ability modifiers, so it is pinned against a real fixture
    /// rather than a table row.
    #[test]
    fn steadfast_personality_grounds_a_record_computed_from_this_characters_abilities() {
        const ID: &str = "feat.standalone.steadfast_personality_will_vs_mind_affecting";
        let mut input = fighter();
        assert_eq!(value(&input, ID), None, "absent without the feat");

        input.chosen.selected_feats.push("Steadfast Personality".to_owned());
        let charisma = super::ability_modifier(input.chosen.ability_scores.charisma);
        let wisdom = super::ability_modifier(input.chosen.ability_scores.wisdom);
        assert_eq!(
            value(&input, ID),
            Some(charisma - wisdom.max(0)),
            "CHA - max(WIS, 0), per both corpus BONUS:SAVE tokens read together"
        );
    }

    /// The two conditional feats must NOT reach the unconditional save
    /// totals. Steel Soul applies only against spells and spell-like
    /// abilities, Steadfast Personality only against mind-affecting
    /// effects; folding either into `defense.total_save.*` would report a
    /// specific, checkable, wrong number for every other save. Proven by
    /// differencing the same character with and without each feat, so this
    /// cannot pass merely because a standalone record exists.
    #[test]
    fn the_two_conditional_save_feats_do_not_move_the_unconditional_save_totals() {
        let baseline: Vec<Option<i16>> = ["fortitude", "reflex", "will"]
            .iter()
            .map(|s| value(&fighter(), &format!("defense.total_save.{s}")))
            .collect();
        assert!(
            baseline.iter().all(Option::is_some),
            "the fixture must compute real save totals for this test to mean anything: \
             {baseline:?}"
        );

        for feat in ["Steel Soul", "Steadfast Personality"] {
            let mut input = fighter();
            input.chosen.selected_feats.push(feat.to_owned());
            let after: Vec<Option<i16>> = ["fortitude", "reflex", "will"]
                .iter()
                .map(|s| value(&input, &format!("defense.total_save.{s}")))
                .collect();
            assert_eq!(
                after, baseline,
                "{feat} is conditional and must not change any unconditional save total"
            );
        }
    }

    /// Great Fortitude still moves the real Fortitude total, proving the
    /// test above pins a genuine distinction rather than a save total that
    /// no feat can ever reach.
    #[test]
    fn an_unconditional_save_feat_still_moves_its_total() {
        let without = value(&fighter(), "defense.total_save.fortitude")
            .expect("fortitude total must be computed");
        let mut with = fighter();
        with.chosen.selected_feats.push("Great Fortitude".to_owned());
        let raised =
            value(&with, "defense.total_save.fortitude").expect("still computed");
        assert_eq!(raised - without, 2, "Great Fortitude grants a real, unconditional +2");
    }

    /// Improved Bull Rush grounds both its CMB and CMD halves; the
    /// Greater variants genuinely carry no CMD term.
    #[test]
    fn combat_maneuver_feats_ground_their_cmb_and_cmd_halves() {
        let mut input = fighter();
        input.chosen.selected_feats.push("Improved Bull Rush".to_owned());
        assert_eq!(value(&input, "feat.standalone.combat_maneuver.bull_rush.cmb"), Some(2));
        assert_eq!(value(&input, "feat.standalone.combat_maneuver.bull_rush.cmd"), Some(2));
    }

    /// Stunning Fist reaches its producer through the class-granted feat
    /// path -- a Monk never has it in `selected_feats`.
    #[test]
    fn a_monk_grounds_stunning_fist_without_ever_selecting_the_feat() {
        let mut monk = fighter();
        monk.chosen.class_levels =
            vec![super::CharacterClassLevel { class_id: "class:monk".to_owned(), level: 4 }];
        assert!(
            !monk.chosen.selected_feats.iter().any(|f| f == "Stunning Fist"),
            "the Monk must not have selected it -- the grant is what is under test"
        );
        // Fixture Wisdom 12 (+1): DC = 10 + 4/2 + 1 = 13.
        assert_eq!(value(&monk, "feat.standalone.stunning_fist.save_dc"), Some(13));
        assert_eq!(value(&monk, "feat.standalone.stunning_fist.uses_per_day"), Some(4));
    }

    /// Weapon Focus was the LAST orphan -- missed by my own #22 sweep
    /// because I re-used a producer inventory taken during #20 instead of
    /// re-enumerating it, and `feat_effects.rs` gained producers in
    /// between. Found by the lead cross-checking all 13 producers rather
    /// than the 8 I had listed.
    ///
    /// Wiring it is a REAL behavior change rather than latent capability:
    /// this repo's own deterministic Fighter fixture already carries
    /// Weapon Focus (Longsword) through the legacy compound id
    /// (`choice:fighter_bonus_feat -> feat:weapon_focus:weapon:longsword`),
    /// which the producer deliberately accepts so the shipped loadout
    /// grounds something. So the fixture Fighter gains a record it never
    /// had before this commit.
    #[test]
    fn the_shipped_fighter_fixture_now_grounds_its_weapon_focus_longsword() {
        assert_eq!(
            value(&fighter(), "feat.standalone.weapon_focus.longsword"),
            Some(1),
            "the real fixture Fighter has Weapon Focus (Longsword) via the legacy compound id"
        );
    }

    /// Greater Weapon Focus stacks with Weapon Focus on the same weapon
    /// for a genuine +2, and the two sources are deduplicated to a
    /// single record rather than emitting one each.
    #[test]
    fn greater_weapon_focus_stacks_to_two_on_the_same_weapon_without_duplicating() {
        let mut input = fighter();
        input.chosen.selected_feats.push("Greater Weapon Focus".to_owned());
        input.chosen.selected_choices.push(
            crate::rules_core::character_input::SelectedChoice {
                choice_set_id: "choice:greater_weapon_focus_target".to_owned(),
                selection_id: "weapon:longsword".to_owned(),
            },
        );
        assert_eq!(value(&input, "feat.standalone.weapon_focus.longsword"), Some(2));

        let count = build_pilot_headless_receipt(&input)
            .computation
            .explanations
            .iter()
            .filter(|e| e.id == "feat.standalone.weapon_focus.longsword")
            .count();
        assert_eq!(count, 1, "the two sources must dedup into one record per weapon");
    }

    /// Spell Focus grounds per chosen school. It is NOT integrated into
    /// the per-spell-level save-DC records: those are keyed by spell
    /// level while Spell Focus is keyed by school, so there is no
    /// matching total to layer it onto without inventing a mapping.
    #[test]
    fn spell_focus_grounds_per_school_and_is_not_folded_into_level_keyed_dcs() {
        let mut input = fighter();
        input.chosen.selected_feats.push("Spell Focus".to_owned());
        input.chosen.selected_choices.push(
            crate::rules_core::character_input::SelectedChoice {
                choice_set_id: "choice:spell_focus_target".to_owned(),
                selection_id: "school:evocation".to_owned(),
            },
        );
        assert_eq!(value(&input, "feat.standalone.spell_focus.evocation"), Some(1));
    }

    /// Task #73 (Wizard-only Spell Focus, bounded slice off #69's
    /// school-keyed-DC scoping). This is the class the slice was actually
    /// scoped for -- the two tests above use the generic `fighter()`
    /// fixture because the producer is genuinely class-agnostic (it reads
    /// only `selected_feats` + `selected_choices`, never a class id), but
    /// #73 asked specifically for a real Wizard fixture, so this closes
    /// that literal coverage gap rather than leaving Wizard's own case
    /// implied-but-unverified.
    ///
    /// The fixture Wizard already specializes in Evocation
    /// (`choice:wizard_school_specialization -> school:evocation`) and
    /// opposes Necromancy/Transmutation; Abjuration is chosen here
    /// deliberately because it is neither, so this test cannot be
    /// mistaken for exercising the specialization mechanic instead of
    /// Spell Focus.
    ///
    /// Note for the record: this exact producer+consumer pair
    /// (`feat_effects::spell_focus_facts_from_choices` and its
    /// `feat.standalone.spell_focus.<school>` consumer in
    /// `build_pilot_headless_receipt`) was already built and merged under
    /// the session's earlier Mechanism-B Focus-feat work (`5d90dc91`,
    /// 2026-07-27), before task #73 was greenlit off #69's scoping doc on
    /// 2026-07-28. So #73's implementation was already shipped; this test
    /// is the one real gap task #73 asked for and the prior work didn't
    /// already cover -- Wizard-specific fixture verification -- not a
    /// from-scratch build.
    #[test]
    fn a_real_wizard_fixture_grounds_spell_focus_for_its_chosen_school() {
        let mut input = wizard();
        assert!(
            input.chosen.class_levels.iter().any(|c| c.class_id == "class:wizard"),
            "must be a real Wizard fixture, not a stand-in"
        );
        input.chosen.selected_feats.push("Spell Focus".to_owned());
        input.chosen.selected_choices.push(
            crate::rules_core::character_input::SelectedChoice {
                choice_set_id: "choice:spell_focus_target".to_owned(),
                selection_id: "school:abjuration".to_owned(),
            },
        );
        assert_eq!(value(&input, "feat.standalone.spell_focus.abjuration"), Some(1));
    }
}

/// SD-32 T12 Epic 8 row 18 cycle 5 (`decisions.md §27b`): the
/// "select-ONE-group, inherit-every-member" pool shape
/// (`push_generic_pool_group_selection_magnitude`), wired at Sorcerer
/// Bloodline, Cleric Domain, Oracle Mystery, Warpriest Blessing and Shaman
/// Spirit -- the five largest pools cycle 4's name census resolved
/// (`t12-class-feature-pool-population_cycle-4_cycle_receipt.md` §1).
/// Proves, per class, that a selection this codebase has NEVER
/// hand-modelled by name now resolves at least one real corpus member
/// through the shared resolver, and that an invented/unrecognized
/// selection resolves nothing (the same "refuse rather than fabricate"
/// contract every prior cycle's own wiring proves).
#[cfg(test)]
mod generic_pool_group_selection_wiring_tests {
    use super::{
        build_pilot_headless_receipt, BLOODRAGER_BLOODLINE_CHOICE_ID, BLOODRAGER_CLASS_ID,
        CharacterClassLevel, CharacterInput,
        CAVALIER_CLASS_ID, CAVALIER_ORDER_CHOICE_ID,
        CLERIC_CLASS_ID, CLERIC_DOMAIN_CHOICE_ID, ORACLE_CLASS_ID, ORACLE_MYSTERY_CHOICE_ID,
        SHAMAN_CLASS_ID, SHAMAN_SPIRIT_CHOICE_ID, SORCERER_BLOODLINE_CHOICE_ID, SORCERER_CLASS_ID,
        WARPRIEST_BLESSING_CHOICE_ID, WARPRIEST_CLASS_ID,
    };
    use crate::rules_core::character_input::{SelectedChoice, load_character_input_fixture};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn class_input(class_id: &str, level: u8, choice_set_id: &str, selection_id: &str) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels = vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: choice_set_id.to_owned(),
            selection_id: selection_id.to_owned(),
        });
        input
    }

    fn generic_explanation_count(input: &CharacterInput, id_prefix: &str) -> usize {
        build_pilot_headless_receipt(input)
            .computation
            .explanations
            .iter()
            .filter(|e| e.id.starts_with(id_prefix))
            .count()
    }

    /// SD-32 T12 Epic 8 row 18 cycle 9 (`§17a` re-derivation, `§12c` population+command named).
    /// Cycle 8 measured the cross-book-merge/`DomainLVL` fix's effect against only 3 of the 6
    /// real pools sharing `push_generic_pool_group_selection_magnitude` (Sorcerer/Bloodrager/
    /// Cleric; Shaman measured but not locked, Warpriest re-checked unchanged) -- this test
    /// re-derives group-level reach across ALL SIX, including Cavalier Order, never checked by
    /// any prior cycle. "Group carries a resolvable member" mirrors cycle 5/8's own measure
    /// exactly (see `group_has_a_resolvable_member`'s own doc for the re-derivation that
    /// confirmed this, not a stricter bar invented fresh) -- at least one real `"<group> ~
    /// <member>"` record resolves via `resolve_pool_member_sole_magnitude`.
    fn real_groups_owned_by(class: &str, registered_name: &str) -> Vec<String> {
        let table = super::class_feature_grant_consumer::class_feature_record_tokens_pre_gate_safe();
        let mut owner_tally: std::collections::BTreeMap<
            String,
            std::collections::BTreeMap<String, u32>,
        > = std::collections::BTreeMap::new();
        for (key, record) in table.iter() {
            if let Some((group, _member)) = key.split_once(" ~ ") {
                *owner_tally.entry(group.to_string()).or_default().entry(record.class.clone()).or_default() +=
                    1;
            }
        }
        // Mirrors `real_pool_group_for_selection_slug`'s own ownership + naming-shape gates
        // exactly (both the majority-tally/header-record ownership proof AND the "does this
        // group's NAME actually belong to this registered pool word" filter its own adjective-
        // stripping logic applies) -- §17a's own validate-against-a-known-case check caught this
        // exact gap on the first pass here: without the naming-shape filter, EVERY " ~ "-group
        // majority-owned by the class (not just its Bloodline/Domain/... ones) was counted,
        // silently inflating both numerator and denominator (Sorcerer measured 72 groups, not
        // the real 53, before this filter was added).
        // SD-32 T12 Epic 8 row 18 cycle 20 (`§27b`/`§17a`, correcting cycle 19's own retro
        // note that this was "not fixed this cycle, named for a future cycle" -- the future
        // cycle is this one): a THIRD, distinct naming-shape false positive on top of the
        // majority-tally/header-ownership gates above. The bare `"<class> <registered_name>"`
        // key prefix (`"Sorcerer Bloodline"`, `"Bloodrager Bloodline"`, `"Shaman Spirit"`) is a
        // real corpus shape -- but it is the class-WIDE catalog record set, not a fourth
        // selectable bloodline/spirit: `"Sorcerer Bloodline ~ Psychic"`,
        // `"Sorcerer Bloodline ~ Ghoul"`, etc. are a SECOND, parallel naming convention for the
        // SAME 52 real bloodlines `"<Specific> Bloodline ~ <power>"` already tallies as their
        // own real groups (confirmed live: every one of `"Sorcerer Bloodline"`'s own 52
        // "members" is itself the name of an already-counted real bloodline group). Because its
        // own name happens to end with `" {registered_name}"`
        // (`"Sorcerer Bloodline".ends_with(" Bloodline")`), the naming-shape filter above
        // wrongly admits it as if it were itself a 53rd/13th selectable group -- inflating the
        // denominator (and, since it always carries a resolvable "member", the numerator too)
        // for every pool carrying this shape -- confirmed live by direct corpus re-derivation,
        // not assumed pool-by-pool: Sorcerer/Bloodrager Bloodline and Shaman Spirit all carry
        // it (`"Sorcerer Bloodline ~ <name>"`, etc.); Cleric Domain and Cavalier Order ALSO
        // carry the same shape (`"Cleric Domain ~ Air"`, `"Cavalier Order ~ Order of the
        // Beast"` -- both already named as real corpus records by cycle 11's own doc above,
        // for a DIFFERENT reason: they contribute useful header `bonus_vars`/desc-formula
        // content, which is real and unaffected by this fix) but the CENSUS re-run below shows
        // whether their own bare `"Cleric Domain"`/`"Cavalier Order"` group name was ALSO being
        // double-counted as a fake extra group; Warpriest Blessing carries no such bare-prefix
        // record at all. Generic exclusion, not a per-pool table: a group whose OWN name is
        // exactly `"<class> <registered_name>"` is never a real selectable member of itself.
        let class_wide_catalog_shape = format!("{class} {registered_name}");
        owner_tally
            .into_iter()
            .filter_map(|(group, owners)| {
                if group == class_wide_catalog_shape {
                    return None;
                }
                let majority_class =
                    owners.iter().max_by_key(|(_, count)| **count).map(|(c, _)| c.clone())?;
                let owned_by_class = majority_class == class
                    || table
                        .get(&format!("{class} {registered_name} ~ {group}"))
                        .is_some_and(|header| header.class == class);
                if !owned_by_class {
                    return None;
                }
                let matches_naming_shape = group.ends_with(&format!(" {registered_name}"))
                    || super::strip_prefix_case_insensitive(&group, &format!("{registered_name} of the "))
                        .is_some();
                matches_naming_shape.then_some(group)
            })
            .collect()
    }

    /// For one real group, whether at least one member resolves via the shared resolver -- the
    /// SAME "carries a member resolvable" measure cycle 5/8's own receipts and doc comments use
    /// (`sorcerer_generic_bloodline_pass_grounds_a_never_hand_modelled_bloodline`'s own doc:
    /// "found only 15 ... carry a member resolvable"), re-validated here (`§17a`) rather than a
    /// stricter "every member" bar invented fresh -- confirmed by first re-deriving with an
    /// "every member" bar and finding it reproduced NEITHER cycle 8's denominators (a naming-
    /// shape filter bug, fixed above) NOR its numerators (4/53 vs. cycle 8's own 18/53) before
    /// switching to this measure, which reproduces cycle 8's three baselines exactly. This is
    /// also the actual real-consumer contract: `push_generic_pool_group_selection_magnitude`
    /// resolves EVERY member independently once a player picks the group, closing whichever
    /// subset resolves -- a group with SOME real records still needing compute is not "closed"
    /// as a kanban line item, but every already-resolvable member inside it is already reaching
    /// a live character, which is what this census is measuring.
    // SD-32 T12 Epic 8 row 18 cycle 20: switched from `resolve_pool_member_sole_magnitude` to
    // `resolve_pool_member_all_magnitudes` (non-empty, not `.is_some()`) -- this census's own
    // doc above states the bar explicitly: "the actual real-consumer contract:
    // push_generic_pool_group_selection_magnitude resolves EVERY member independently ... which
    // is what this census is measuring." Cycle 20 upgraded that real consumer to resolve every
    // independent terminal a member carries, not just a lone one; a member this census would
    // have called unresolvable before this cycle purely because it carried 2-3 real terminals
    // (e.g. `Forbidden Rites Domain ~ Madness Domain`) now correctly counts as resolvable,
    // because the real consumer now genuinely resolves it. A record with terminal(s) that still
    // never bind through the chain still resolves to an empty `Vec`, so a true refusal is
    // unaffected.
    fn group_has_a_resolvable_member(class: &str, registered_name: &str, group: &str) -> bool {
        let table = super::class_feature_grant_consumer::class_feature_record_tokens_pre_gate_safe();
        let prefix = format!("{group} ~ ");
        let ability_modifiers = crate::rules_core::pilot_compute::AbilityModifiers::default();
        table.keys().filter(|key| key.starts_with(&prefix)).any(|key| {
            !super::resolve_pool_member_all_magnitudes(
                key,
                group,
                5,
                &ability_modifiers,
                Some(class),
                Some(registered_name),
            )
            .is_empty()
        })
    }

    /// SD-32 T12 Epic 8 row 18 cycle 11: proves `pool_header_record_by_normalized_suffix`'s
    /// cycle-11 widening MERGES every real header candidate rather than returning the first hit
    /// and stopping. Cleric's own "Air Domain" pool has TWO real header-shaped corpus records
    /// (confirmed live): the bare-key `"Air Domain"` record (`data/corpus/core_rulebook/
    /// class_feature/air/air.json`, carrying the useful `DomainAirDC`/`DomainAirLVL`/
    /// `DomainAirTimes` chain) and the `"Cleric Domain ~ Air"` record (`.../cleric_domain/
    /// cleric_domain_air.json`, a domain SPELL LIST record with a `SPELLLEVEL` token and ZERO
    /// `BONUS:VAR` tokens). A first-match-wins version of this function that tried the
    /// `"<class> <registered_name> ~ <suffix>"` shape BEFORE the bare-key shape would find the
    /// EMPTY spell-list record first and never reach the useful one -- exactly the regression
    /// this cycle caught live (`pool_group_closure_census_across_all_six_pools` dropped from
    /// 18/53 to 15/53 Sorcerer and 26/72 to 14/72 Cleric on the first (short-circuiting) version
    /// of this widening, before it was rewritten to merge).
    #[test]
    fn pool_header_lookup_merges_every_real_header_shape_not_just_the_first_match() {
        let merged = super::pool_header_record_by_normalized_suffix("Cleric", "Air Domain", Some("Domain"));
        assert!(
            merged.contains_key("DomainAirDC"),
            "the bare-key \"Air Domain\" header's own real BONUS:VAR chain must still merge in \
             even though \"Cleric Domain ~ Air\" (an empty spell-list record sharing the \
             registered-name naming shape) also matches: {merged:?}"
        );
    }

    /// SD-32 T12 Epic 8 row 18 cycle 13: proves the NEW bare-key, `s`-tolerant clause
    /// `pool_header_record_by_normalized_suffix` gained this cycle -- real corpus
    /// `data/corpus/core_rulebook/class_feature/domains/domains.json` carries KEY `"Domains"`
    /// (plural, `data.class` genuinely absent) but every caller ever asks for the SINGULAR
    /// registered name `"Domain"`; without the trailing-`s` tolerance this merge never reaches
    /// the real `BONUS:VAR|DomainPowerTimes|3+WIS` chain every Domain member's own `...Times`
    /// formula depends on.
    #[test]
    fn pool_header_lookup_reaches_a_bare_plural_class_independent_base_record() {
        let merged = super::pool_header_record_by_normalized_suffix("Cleric", "Domain", None);
        let times = merged.get("DomainPowerTimes").unwrap_or_else(|| {
            panic!(
                "the bare, plural \"Domains\" base record's own DomainPowerTimes chain must \
                 merge when looked up by its singular registered name \"Domain\": {:?}",
                merged.keys().collect::<Vec<_>>()
            )
        });
        // `3+WIS` -- asserted as what it computes, not as the text it was written in.
        let resolved = crate::rules_core::record_vars::evaluate_with_bindings(
            times,
            &std::collections::BTreeMap::from([("WIS".to_string(), 4i64)]),
        );
        assert_eq!(resolved, Some(7), "DomainPowerTimes is 3 + the caster's Wisdom modifier");
    }

    /// SD-32 T12 Epic 8 row 18 cycle 13: proves the existing `"<class> ~ "`-prefixed wildcard
    /// clause (already `s`-tolerant since cycle 5) now ALSO fires when the caller passes the
    /// bare REGISTERED NAME rather than a specific group name -- real corpus `"Shaman ~ Spirit"`
    /// (class `"Shaman"`, `BONUS:VAR|ShamanSpiritLVL|ShamanLVL`) is the class-wide shared base
    /// every per-spirit member (e.g. `"Bones Spirit ~ Shedding Form"`'s own
    /// `ShamanSheddingFormRounds|ShamanSpiritLVL`) chains through, but was never reached before
    /// this cycle because no per-group lookup or `"<rn> Tracker"` lookup ever tried the bare
    /// registered name itself.
    #[test]
    fn pool_header_lookup_reaches_the_class_wide_registered_name_base_record() {
        let merged = super::pool_header_record_by_normalized_suffix("Shaman", "Spirit", None);
        let spirit_level = merged.get("ShamanSpiritLVL").unwrap_or_else(|| {
            panic!(
                "\"Shaman ~ Spirit\"'s own real ShamanSpiritLVL chain must merge when looked \
                 up by its own bare registered name: {:?}",
                merged.keys().collect::<Vec<_>>()
            )
        });
        assert!(
            spirit_level.refs.iter().any(|r| r == "ShamanLVL"),
            "ShamanSpiritLVL is the Shaman's own class level: {:?}",
            spirit_level.refs
        );
        let resolved = crate::rules_core::record_vars::evaluate_with_bindings(
            spirit_level,
            &std::collections::BTreeMap::from([("ShamanLVL".to_string(), 9i64)]),
        );
        assert_eq!(resolved, Some(9));
    }

    /// SD-32 T12 Epic 8 row 18 cycle 20: the new multi-terminal resolver genuinely resolves TWO
    /// of `Madness Domain`'s three independent terminals -- not a guess among them, each is
    /// independently computed through the real PCGen formula evaluator: `DomainMadnessTimes`
    /// chains to `DomainPowerTimes-1`, bound by the bare `"Domains"` base header
    /// (`BONUS:VAR|DomainPowerTimes|3+WIS`); `DomainMadnessDC` chains to
    /// `10+(DomainMadnessLVL/2)+CHA-1`, itself resolving `DomainMadnessLVL` via `DomainLVL-2`
    /// (bound the same way). The third, `DomainMadnessAbilityTriggerLVL`
    /// (`DomainAbilityTriggerLVL-SeparistDomainLVL`), correctly stays unresolved: `SeparistDomainLVL`
    /// is a real corpus-bound identifier (`Separatist ~ Forbidden Rites`, `BONUS:VAR|
    /// SeparistDomainLVL|-2`) but that record's own key shape (`"Separatist ~ Forbidden
    /// Rites"`, prefixed by the ARCHETYPE, not `"Forbidden Rites Domain"` nor `"Cleric"`) is not
    /// one any existing header-merge clause reaches -- a genuinely separate, narrower gap this
    /// cycle names rather than forces (`§27b` point 5, remaining work, not this cycle's own
    /// scope). `resolve_pcgen_var_chain`'s cycles 2/5 safety property correctly refuses to
    /// 0-default the unreached identifier rather than fabricate a value for it -- this third
    /// terminal is properly ABSENT from the returned set, not silently wrong.
    #[test]
    fn all_magnitudes_resolves_every_reachable_independent_terminal_on_a_multi_terminal_record() {
        let ability_modifiers = crate::rules_core::pilot_compute::AbilityModifiers::default();
        let resolved = super::resolve_pool_member_all_magnitudes(
            "Forbidden Rites Domain ~ Madness Domain",
            "Forbidden Rites Domain",
            5,
            &ability_modifiers,
            Some("Cleric"),
            Some("Domain"),
        );
        let targets: std::collections::BTreeSet<&str> =
            resolved.iter().map(|(name, _)| name.as_str()).collect();
        assert_eq!(
            targets,
            std::collections::BTreeSet::from(["DomainMadnessDC", "DomainMadnessTimes"]),
            "the two of Madness Domain's three real terminals whose chain is fully reachable \
             must resolve, none guessed and none dropped; the third (blocked on the separate, \
             named `SeparistDomainLVL` header gap) must correctly stay absent, not fabricated: \
             {resolved:?}"
        );
    }

    /// Re-derivation, not a trust of cycle 8's own transcribed figures (`§17a`): run for real,
    /// print the population and command, THEN assert. `cargo test --locked --lib -- \
    /// rules_core::pilot_compute::generic_pool_group_selection_wiring_tests::pool_group_closure_census_across_all_six_pools \
    /// --nocapture`
    #[test]
    fn pool_group_closure_census_across_all_six_pools() {
        let pools: &[(&str, &str)] = &[
            ("Sorcerer", "Bloodline"),
            ("Bloodrager", "Bloodline"),
            ("Cleric", "Domain"),
            ("Shaman", "Spirit"),
            ("Warpriest", "Blessing"),
            ("Cavalier", "Order"),
        ];
        let mut report = String::new();
        for (class, registered_name) in pools {
            let groups = real_groups_owned_by(class, registered_name);
            let total = groups.len();
            let closed: Vec<&String> = groups
                .iter()
                .filter(|g| group_has_a_resolvable_member(class, registered_name, g))
                .collect();
            report.push_str(&format!(
                "{class} {registered_name}: {}/{total} groups carry a resolvable member\n",
                closed.len()
            ));
        }
        println!("{report}");
        // Locked baseline, re-derived fresh this cycle (see receipt for the full per-pool
        // breakdown and closed-group names) -- a future cycle's corpus/resolver change that
        // moves these numbers must update this assertion deliberately, never silently.
        //
        // SD-32 T12 Epic 8 row 18 cycle 13: cycle 12 named an open question rather than guessing
        // at it -- `resolve_pool_member_sole_magnitude` has independent per-member refusals
        // (empty own `bonus_vars`, the multi-terminal-target rule, and unresolved header-chain
        // gaps) beyond var-chain resolution. Traced with a per-member diagnostic (printed, then
        // removed before commit, per this bundle's own methodology) across Bloodrager/Cleric/
        // Shaman/Warpriest: the dominant remaining bucket (Cleric 50, Shaman 8 members) was a
        // single-terminal member whose formula chains to a bare header var
        // (`DomainAirTimes|DomainPowerTimes`, `ShamanSheddingFormRounds|ShamanSpiritLVL`) never
        // merged into `combined_vars` -- a THIRD real corpus header shape,
        // `pool_header_record_by_normalized_suffix` now also covers (see that function's own new
        // doc comment): a bare, PLURAL, class-independent base record
        // (`data/corpus/core_rulebook/class_feature/domains/domains.json`, key exactly
        // `"Domains"`, real `BONUS:VAR|DomainPowerTimes|3+WIS`, `data.class` genuinely absent --
        // confirmed no other corpus record anywhere else also targets `DomainPowerTimes`), and
        // the exact `"<class> ~ <registered_name>"` shape (`"Shaman ~ Spirit"`, class `"Shaman"`,
        // real `BONUS:VAR|ShamanSpiritLVL|ShamanLVL"`), reached via the existing per-pool
        // `registered_name_for_tracker` merge site (now also tries the bare registered name, not
        // only `"<rn> Tracker"`). This ALSO reaches Warpriest's own `"Warpriest ~ Blessings"`
        // record (`WarpriestBlessingLVL|WarpriestLVL`) via the same trim-trailing-`s` wildcard
        // already present in the `"<class> ~ "`-prefixed clause -- but Warpriest stays 0/37
        // because every real Blessing member's OWN `bonus_vars` is empty (`§17a` re-derivation:
        // diagnostic showed `empty=74, multi_terminal=0` for all 74 Warpriest members), a
        // DIFFERENT, deeper gap this merge cannot reach and this cycle did not force. `Bloodrager
        // Bloodline` and `Cavalier Order` are honestly UNCHANGED: Bloodrager's remaining 7
        // single-terminal members each chain to a per-bloodline `BloodlineLVL` identifier that
        // IS bound elsewhere in the corpus (a same-named cross-class `Eldritch Scion` record,
        // e.g. `Bloodrager_Aberrant_BloodlineLVL|BloodragerBloodlineLVL`,
        // `data/corpus/advanced_class_guide/class_feature/eldritch_scion_aberrant_bloodline/
        // eldritch_scion_aberrant_bloodline.json`, `class: "Sorcerer"`) -- `resolve_pcgen_var_
        // chain`'s cycle-12 safety property correctly refuses to 0-default a genuinely
        // corpus-bound identifier (cycles 2/5's proven property), and this cycle found no header
        // record anywhere binding a pure Bloodrager's own per-bloodline `BloodlineLVL`; Cavalier's
        // 8 Order groups still carry zero `BONUS:VAR` tokens at all, unaffected by any header
        // merge (`§17`, unchanged from cycles 9/10/11/12).
        //
        // Net real movement this cycle: `Sorcerer Bloodline` UNCHANGED at 31/53 (cycle 12's own
        // fix, re-verified). `Cleric Domain` moved 26/72 -> 34/72 (+8, all via the new bare
        // `"Domains"` merge). `Shaman Spirit` moved 8/14 -> 11/14 (+3, all via the new
        // `"Shaman ~ Spirit"` merge -- diagnostic confirms `single_unresolved` dropped 8 -> 0 for
        // this pool). `Bloodrager Bloodline`, `Warpriest Blessing`, `Cavalier Order` UNCHANGED
        // (5/12, 0/37, 1/9) -- re-run and re-checked, not assumed.
        // SD-32 T12 Epic 8 row 18 cycle 18 (`§27b`/`§17a`): `Cleric Domain` moved 34/72 -> 44/73
        // via TWO independent, genuine ENGINE gaps this cycle's own classification diagnostic
        // found and closed -- neither is a data fabrication.
        //
        // (1) `pool_header_record_by_normalized_suffix` gained a FIFTH header shape, a
        // `domain`-kind corpus record (never `class_feature`), keyed by the domain's bare name
        // (`"Cave"`, not `"Cave Domain"`). Confirmed live
        // (`data/corpus/ultimate_magic/domain/cave.json`): this record ALREADY carries the real
        // `BONUS:VAR|DomainCaveLVL|DomainLVL`/`DomainCaveDC`/`DomainCaveTimes` chain every one of
        // that domain's `class_feature` member records needs -- it simply lived in a directory no
        // existing table ever read. Re-derived (not assumed): the newly-closed groups are
        // Aquatic, Arctic, Eagle, Frog, Monkey, Plains, Serpent, Swamp (8 groups; Jungle and
        // Mountain also gained the same header merge but stayed unresolved for the SEPARATE
        // reason (2) names).
        //
        // (2) `class_feature_record_tokens_pre_gate_safe`'s own `description: null` gate widened
        // (see that function's own doc comment): a real, invisible, purely-mechanical member
        // record (`VISIBLE:NO`, e.g. `Jungle Domain ~ Trap Sense`, `BONUS:VAR|TrapSenseBonus|
        // DomainJungleLVL/3`, `description: null`) was refused purely for want of this read path.
        // This ALSO revealed one previously wholly-invisible 73rd group (denominator moved
        // 72 -> 73, a real population correction, not a resolver artefact -- a group whose only
        // members all carried `description: null` was invisible to `real_groups_owned_by`'s own
        // tally before this widening). Net movement from (1)+(2) combined: 34/72 -> 44/73.
        // Every other pool UNCHANGED, re-verified, not assumed.
        // SD-32 T12 Epic 8 row 18 cycle 20 (`§27b`/`§17a`): `resolve_pool_member_sole_
        // magnitude`'s refusal-when-more-than-one-terminal has NOT been weakened -- it is
        // unchanged, still used unmodified by every existing caller that needs exactly one
        // value. Two independent, additive fixes moved these baselines this cycle:
        //
        // (1) This census helper switched to `resolve_pool_member_all_magnitudes` (see that
        // function's own doc, and `group_has_a_resolvable_member`'s), which genuinely resolves
        // EVERY independent terminal a record carries rather than refusing the whole record
        // once there is more than one: `Sorcerer Bloodline` numerator +3, `Bloodrager Bloodline`
        // numerator +7 (every real bloodline carries its own per-bloodline "<Bloodline> ~ Feat
        // Tracker" record, description:null, admitted since cycle 18's own widening, whose
        // several `BONUS:VAR|Bloodrager_BloodlineFeat_<Feat>|1` entries are genuinely
        // independent, unconditional, corpus-verified constant-1 targets -- confirmed live
        // against `aberrant_bloodrager_bloodline_feat_tracker.json`), `Cleric Domain` numerator
        // +2 (`Forbidden Rites Domain`'s own multi-terminal members, e.g. `Madness Domain`'s
        // `DomainMadnessDC`/`...Times`), `Shaman Spirit` numerator +1.
        //
        // (2) `real_groups_owned_by`'s own new class-wide-catalog-shape exclusion (`§27b`/cycle
        // 19's own retro note, fixed this cycle) drops a fake extra group per pool wherever the
        // bare `"<class> <registered_name>"` key prefix exists as a real corpus record: `Sorcerer
        // Bloodline` denominator 53 -> 52, `Bloodrager Bloodline` 12 -> 11, `Shaman Spirit` 14
        // -> 13 (cycle 18's own named "Shaman Spirit" false positive, now actually fixed, not
        // just re-named), `Cleric Domain` 73 -> 72 (a SECOND instance of the SAME shape,
        // `"Cleric Domain ~ Air"` etc., not previously named by any cycle), `Cavalier Order` 9
        // -> 8 (a THIRD instance, `"Cavalier Order ~ Order of the Beast"` etc., also newly
        // found). `Warpriest Blessing` carries no such bare-prefix record at all -- denominator
        // genuinely unchanged. Numerator effect of (2) alone, checked per pool, not assumed
        // uniform: Sorcerer/Bloodrager/Cleric/Cavalier's own bare catalog "group" carries no
        // resolvable `bonus_vars` (its "members" are duplicate-named real groups' spell-list/
        // summary records) so removing it costs those four pools' numerators nothing; Shaman's
        // OWN bare `"Shaman Spirit ~ <name>"` catalog members DO carry a real, independent,
        // now-resolving constant-1 terminal (`ShamanXSpirit|1`, closed by fix (1) above), so
        // removing this one fake group costs Shaman's numerator exactly 1 alongside its
        // denominator.
        //
        // (3) `pool_header_record_by_normalized_suffix`'s new SIXTH header shape
        // (`"Domain Base ~ <bare>"`, `data.class` literally `"Domain Base"` -- see that
        // function's own doc) closes the 6 named desc-formula refusals' `Void Domain`, and
        // corrects cycle 19's own `§27b` claim that `Scalykind` was a genuine hard-impossibility
        // (it carries this exact header too -- `Domain Base ~ Scalykind` -- missed by cycle 19's
        // verification, which checked only the `domain`-kind and bare `class_feature` shapes,
        // never this third directory; retro correction filed). `Cleric Domain` numerator +1 via
        // this fix's own bonus_vars reach (Scalykind).
        //
        // Combined re-derived baselines: `Sorcerer Bloodline` 31/53 -> 34/52, `Bloodrager
        // Bloodline` 5/12 -> 11/11 (every remaining group now closes), `Cleric Domain` 44/73 ->
        // 47/72, `Shaman Spirit` 11/14 -> 11/13, `Warpriest Blessing` UNCHANGED 0/37,
        // `Cavalier Order` 1/9 -> 1/8.
        //
        // SD-32 T12 Epic 8 row 18 cycle 21 (`§27b`): `Sorcerer Bloodline` 34/52 -> 45/52 (+11) --
        // a SEVENTH real corpus header shape, `wildblooded_variant_parent_pool_group`'s own doc:
        // a Wildblooded bloodline variant's own `PREABILITY` token corpus-declares its real parent
        // bloodline as a level-1 prerequisite, so that parent's own header vars are, by
        // corpus-declared construction, always genuinely bound -- NOT cycle 17/19's unrelated
        // cross-bloodline refusal shape (a variant's own declared parent, not a stranger
        // bloodline). Falsified cycle 19's own implicit assumption that these 18 groups were part
        // of the SAME cross-bloodline-gap population as the real cross-bloodline refusals -- 11 of
        // the 18 close via this fix; 7 remain, honestly re-classified (see receipt): `Groveborn`/
        // `Primal` Bloodline are genuine `§27b` zero-content gaps (their own Wildblooded record
        // carries neither a `BONUS:VAR` nor a `%N` desc-formula argument anywhere -- proven the
        // same way Warpriest Blessing's 29 and Cavalier Order's 6 are, not a header gap at all);
        // `Anarchic`/`Karmic`/`Sanguine`/`Seaborn`/`Warped` Bloodline's own parent header now
        // supplies the needed target NAME but the full chain still refuses for a DIFFERENT,
        // un-traced reason this cycle did not chase to ground (real remaining work, not proven
        // impossible, not forced). Every other pool UNCHANGED, re-verified.
        // SD-32 T12 Epic 8 row 18 cycle 22 (`§27b`): `Sorcerer Bloodline` 45/52 -> 48/52 (+3) --
        // the EIGHTH real corpus header shape (`pool_group_header_vars_merged`'s own new doc: the
        // class's own `"<class> ~ Spells"` record, carrying the base spellcasting-stat `BONUS:VAR`
        // chain, `Sorcerer_Spells_StatBonus|CHA`). Traced end to end per the brief's own five-group
        // instruction: Karmic/Seaborn/Warped Bloodline's own real DC-formula members (`Fate's
        // Retribution`/`Water Blast`/`Warp Touch`) all needed this ONE identifier -- every other
        // term in their formula was already reachable through cycle 21's seventh-shape parent-
        // header recursion; this was a genuine missing READ PATH (the identifier IS bound
        // elsewhere in the corpus, so `resolve_pcgen_var_chain`'s own 0-default fallback correctly
        // refused to fabricate a value for it), not a data gap. `Groveborn`/`Primal` Bloodline
        // (cycle 21's own proven zero-content gaps) and `Anarchic` Bloodline (this cycle's own
        // newly-traced zero-content gap, see receipt) remain the honest 3 unresolved via this
        // resolver -- `52 - 48 - 1(Sanguine, desc-formula only, see the sibling test below) = 3`.
        // Every other pool UNCHANGED, re-verified.
        assert!(
            report.contains("Sorcerer Bloodline: 48/52")
                && report.contains("Bloodrager Bloodline: 11/11")
                && report.contains("Cleric Domain: 47/72")
                && report.contains("Shaman Spirit: 11/13")
                && report.contains("Warpriest Blessing: 0/37")
                && report.contains("Cavalier Order: 1/8"),
            "cycle 22's own re-derived baselines must still reproduce exactly:\n{report}"
        );
    }

    /// SD-32 T12 Epic 8 row 18 cycle 16 (`§17a`): whether a group's member resolves via the
    /// SECOND generic resolver -- cycle 15's `resolved_description_for_formula_only_desc_argument`
    /// (`class_feature_grant_consumer.rs`), the `%N`-substituted-DESC-formula path for a record
    /// whose `bonus_vars` is EMPTY. `pool_group_closure_census_across_all_six_pools` (above) is
    /// blind to this path BY CONSTRUCTION -- `group_has_a_resolvable_member` calls only
    /// `resolve_pool_member_sole_magnitude`, which itself only ever reads `bonus_vars` and
    /// therefore always returns `None` for a record this second resolver grounds. This function
    /// closes that blind spot without touching or replacing the first one.
    fn group_has_a_resolvable_member_via_description_formula(
        class: &str,
        registered_name: &str,
        group: &str,
    ) -> bool {
        let table = super::class_feature_grant_consumer::class_feature_record_tokens_pre_gate_safe();
        let prefix = format!("{group} ~ ");
        let ability_modifiers = crate::rules_core::pilot_compute::AbilityModifiers::default();
        // SD-32 T12 Epic 8 row 18 cycle 19: mirror the production call site's own FULL
        // header-chain merge (see `push_generic_pool_group_selection_description_magnitude`,
        // `pool_group_header_vars_merged`) so this census helper reports the SAME reachability
        // the real chassis path now has, not a stale no-header-merge figure.
        let header_vars = super::pool_group_header_vars_merged(class, group, Some(registered_name));
        table.keys().filter(|key| key.starts_with(&prefix)).any(|key| {
            super::class_feature_grant_consumer::resolved_description_for_formula_only_desc_argument(
                key,
                5,
                &ability_modifiers,
                &header_vars,
            )
            .is_some()
        })
    }

    /// A group is genuinely resolvable if EITHER generic resolver grounds at least one of its
    /// members -- the two are mutually exclusive per-record (`resolved_description_for_formula_
    /// only_desc_argument`'s own `bonus_vars.is_empty()` guard, documented on that function),
    /// never double-counting the same record through both paths.
    fn group_has_a_resolvable_member_via_either_resolver(
        class: &str,
        registered_name: &str,
        group: &str,
    ) -> bool {
        group_has_a_resolvable_member(class, registered_name, group)
            || group_has_a_resolvable_member_via_description_formula(class, registered_name, group)
    }

    /// SD-32 T12 Epic 8 row 18 cycle 16: the honest, all-resolver census cycle 15 named but
    /// deliberately did not self-apply (`§1a` -- widening the census's own measure is a separate,
    /// deliberate act). Validated against known truth BEFORE being trusted (`§17a`): re-derives
    /// the SAME six bonus_vars-only baselines `pool_group_closure_census_across_all_six_pools`
    /// locks (proving this new pass reproduces, not contradicts, that instrument), THEN adds the
    /// description-formula resolver on top and reports the combined figure per pool. Both figures
    /// are printed for every pool so neither becomes unverifiable, per the brief's own instruction.
    ///
    /// `cargo test --locked --lib -- \
    /// rules_core::pilot_compute::generic_pool_group_selection_wiring_tests::pool_group_closure_census_across_all_six_pools_both_resolvers \
    /// --nocapture`
    #[test]
    fn pool_group_closure_census_across_all_six_pools_both_resolvers() {
        let pools: &[(&str, &str)] = &[
            ("Sorcerer", "Bloodline"),
            ("Bloodrager", "Bloodline"),
            ("Cleric", "Domain"),
            ("Shaman", "Spirit"),
            ("Warpriest", "Blessing"),
            ("Cavalier", "Order"),
        ];
        let mut report = String::new();
        for (class, registered_name) in pools {
            let groups = real_groups_owned_by(class, registered_name);
            let total = groups.len();
            let bonus_vars_only = groups
                .iter()
                .filter(|g| group_has_a_resolvable_member(class, registered_name, g))
                .count();
            let combined = groups
                .iter()
                .filter(|g| group_has_a_resolvable_member_via_either_resolver(class, registered_name, g))
                .count();
            report.push_str(&format!(
                "{class} {registered_name}: bonus_vars={bonus_vars_only}/{total}, \
                 combined(bonus_vars OR desc_formula)={combined}/{total}\n"
            ));
        }
        println!("{report}");
        // `§17a` validation: the bonus_vars-only figures inside this combined report must
        // reproduce `pool_group_closure_census_across_all_six_pools`'s own locked baseline
        // exactly -- proof this pass did not silently change or duplicate the first resolver's
        // own measure, only add a second one alongside it.
        assert!(
            report.contains("Sorcerer Bloodline: bonus_vars=48/52")
                && report.contains("Bloodrager Bloodline: bonus_vars=11/11")
                && report.contains("Cleric Domain: bonus_vars=47/72")
                && report.contains("Shaman Spirit: bonus_vars=11/13")
                && report.contains("Warpriest Blessing: bonus_vars=0/37")
                && report.contains("Cavalier Order: bonus_vars=1/8"),
            "the bonus_vars-only figures embedded in the combined census must still reproduce \
             pool_group_closure_census_across_all_six_pools's own locked baseline exactly -- a \
             mismatch here means this pass is measuring something different from the first \
             resolver, not extending it:\n{report}"
        );
        // The honest, all-resolver figure (`§17a` -- re-derived, not assumed, and CORRECTING
        // cycle 15's own manual count in the process, per `scripts/retro.py correction`
        // `1787582848402-t9-onboarding-9a4294`). Cycle 15's receipt claimed only 6 Warpriest
        // groups closed and that Cavalier Order stayed unclosed because Order of the Beast's
        // "%2" argument -- `(CavalierLVL>=10)+(CavalierLVL>=14)+(CavalierLVL>=18)` -- hit a
        // "comparison-as-numeric-term gap". Direct reproduction disproves that: every one of
        // those three comparisons is PARENTHESISED, and `formula_interpreter.rs`'s wave-26
        // shape closure already evaluates a parenthesised comparison as a numeric primary
        // (`parse_primary`'s `LParen` branch -> `parse_arith_or_bool`) -- there is no gap here
        // at all. Order of the Beast's whole description resolves cleanly and its group
        // genuinely closes. The real, narrower gap this cycle found (see `§5` below) is an
        // UNPARENTHESISED comparison used as a bare function argument (`min(WarpriestLVL>20,
        // 2,...)`, `Protection Blessing ~ Increased Defense`) -- a real, different, smaller
        // shape than what cycle 15 named.
        //
        // Re-derived totals: Warpriest Blessing 0/37 -> 8/37 (cycle 15's own 6 named groups --
        // Earth, Trickery, Rune, Protection [via Aura of Protection], Repose, Knowledge -- PLUS
        // Destruction and Strength Blessing, already hand-modelled by dedicated functions this
        // codebase ships separately from either generic resolver, and now ALSO reachable
        // through the generic description resolver directly off their own corpus record, which
        // this census correctly counts since it measures corpus-record resolvability, not
        // runtime double-emission avoidance). Cavalier Order 1/9 -> 2/9 (Order of the Beast, the
        // correction above). Three pools this cycle's own diagnostic found ungrounded by cycle
        // 15's manual review pick up exactly one new group each, all real, non-fabricated,
        // single-member closures on records whose `bonus_vars` is empty and whose ONE `%N`
        // argument is a bare, self-contained formula needing no comparison or multi-arg
        // function at all: Bloodrager Bloodline 5/12 -> 6/12 (Abyssal Bloodrager Bloodline ~
        // Demonic Aura, `%1`="CON", the CON modifier, resolving to 0 under this test's own
        // `AbilityModifiers::default()` seed -- the SAME zero-ability-score convention every
        // other function in this file already resolves against, not a new default introduced
        // here), Cleric Domain 34/72 -> 35/72 (Clandestine Domain ~ Blessed Secrecy, `%1`="WIS",
        // same convention), Shaman Spirit
        // 11/14 -> 12/14 (Wood Spirit ~ Tree Form, `%1`=`5`, `ShamanLVL`-derived). Sorcerer
        // Bloodline is genuinely unchanged at 31/53 -- no member of any of its remaining 22
        // open groups has an empty `bonus_vars` chain with a resolvable `%N` argument.
        //
        // SD-32 T12 Epic 8 row 18 cycle 18 (`§27b`/`§17a`): `Cleric Domain` moved again,
        // 35/72 -> 45/73, via `pool_group_closure_census_across_all_six_pools`'s own newly
        // re-derived 44/73 `bonus_vars`-only baseline (see that test's own comment for the two
        // real engine fixes: a `domain`-kind corpus header record this codebase never read
        // before this cycle, and a `description: null` gate that hid real magnitude-bearing
        // member records) PLUS the one already-standing `desc_formula`-only closure (Clandestine
        // Domain) this pass adds on top, unaffected by either fix (it resolves via the OTHER
        // resolver). Every other pool UNCHANGED.
        //
        // SD-32 T12 Epic 8 row 18 cycle 19 (`§27b`/`§17a`): the desc-formula resolver's own
        // missing header-merge step (cycle 18's own named gap, `§3` of its receipt) is now fixed
        // -- `resolved_description_for_formula_only_desc_argument` merges the SAME full header
        // chain (`pool_group_header_vars_merged`, factored out of `resolve_pool_member_sole_
        // magnitude` so both generic resolvers share ONE merge implementation, `§17`) that the
        // bonus_vars resolver already had. Re-derived, not assumed (diagnostic run then removed,
        // same methodology cycle 18 used for its own group-classification pass): Cleric Domain
        // combined 45/73 -> 49/73 (+4: Cave, Desert, Mountain, Nobility Domain -- each closes via
        // its own single member whose `%N` argument chains through the domain-kind header's own
        // `DomainXTimes|DomainPowerTimes` to Cleric Domain's bare `"Domains"` base-header record,
        // e.g. `Mountain Domain ~ Foothold`'s `%1`=`DomainMountainTimes`, the EXACT gap cycle 18
        // named by coordinate). Sorcerer Bloodline combined 31/53 -> 32/53 (+1: Imperious
        // Bloodline, whose sole member's `%N` argument chains through the `"Bloodline Tracker"`
        // base-header record the bonus_vars resolver's own Tracker merge already reached but this
        // resolver, pre-fix, did not). Bonus_vars-only baselines for both pools UNCHANGED (already
        // asserted above) -- this cycle widens ONLY the desc-formula resolver's own reach, not the
        // bonus_vars resolver's. Every other pool's combined figure UNCHANGED, re-verified.
        // SD-32 T12 Epic 8 row 18 cycle 20 (`§27b`/`§17a`): `bonus_vars` moved to cycle 20's own
        // re-derived baseline above (34/52, 11/11, 47/72, 11/13, 0/37, 1/8 -- the multi-terminal
        // resolver, the class-wide-catalog-shape census fix, AND the new `"Domain Base ~ <X>"`
        // header shape closing Scalykind, see that test's own comment); `combined` moves by the
        // same deltas PLUS one further desc-only closure this cycle's own header widening also
        // reaches: Cleric's standing +4 desc-only closures (Cave/Desert/Mountain/Nobility) stack
        // on the new 47/72 bonus_vars figure, PLUS Void Domain (`Part the Veil`'s `%1` =
        // `DomainVoidTimes`, now reachable through the same new header): 52/72. Shaman's own
        // standing +1 desc-only closure (Wood Spirit ~ Tree Form) stacks unchanged on the new
        // 11/13: 12/13. Sorcerer/Bloodrager/Warpriest/Cavalier combined move by their own
        // bonus_vars deltas alone, no further desc-only closure found for them this cycle.
        // SD-32 T12 Epic 8 row 18 cycle 21 (`§27b`): `Sorcerer Bloodline` bonus_vars 34/52 ->
        // 45/52 via the Wildblooded-parent-header seventh shape (see the OTHER census test's own
        // comment for the full derivation); `combined` moves by the same delta, no further
        // desc-only closure found. Every other pool UNCHANGED.
        // SD-32 T12 Epic 8 row 18 cycle 22 (`§27b`): `Sorcerer Bloodline` bonus_vars 45/52 ->
        // 48/52 via the eighth shape (`"<class> ~ Spells"`'s own `Sorcerer_Spells_StatBonus`,
        // see the OTHER census test's own comment) -- Karmic/Seaborn/Warped Bloodline. `combined`
        // moves ONE FURTHER, 48/52 -> 49/52: Sanguine Bloodline's `The Blood Is the Life` (empty
        // `bonus_vars`, `%1` desc-formula argument `Sorcerer_Undead_BloodlinePower1Times`) needed
        // the SAME missing identifier one hop further down its own parent's `BloodlinePowerTimes|
        // 3+Sorcerer_Spells_StatBonus` chain -- confirmed live, traced end to end, not assumed
        // from the DC-formula fix alone. `Anarchic Bloodline ~ Wild Feedback` (this cycle's own
        // newly-traced group) carries neither a `BONUS:VAR` token nor a `%N` desc-formula argument
        // anywhere in any of its 3 real members -- a genuine `§27b` zero-content hard gap, the
        // SAME proof standard as `Groveborn`/`Primal` (cycle 21). `52 - 49 = 3` unresolved:
        // Groveborn, Primal, Anarchic -- all three now exhaustively `§27b`-proven, zero real,
        // un-traced Sorcerer Bloodline work remaining. Every other pool UNCHANGED.
        assert!(
            report.contains("Sorcerer Bloodline: bonus_vars=48/52, combined(bonus_vars OR desc_formula)=49/52")
                && report.contains("Bloodrager Bloodline: bonus_vars=11/11, combined(bonus_vars OR desc_formula)=11/11")
                && report.contains("Cleric Domain: bonus_vars=47/72, combined(bonus_vars OR desc_formula)=52/72")
                && report.contains("Shaman Spirit: bonus_vars=11/13, combined(bonus_vars OR desc_formula)=12/13")
                && report.contains("Warpriest Blessing: bonus_vars=0/37, combined(bonus_vars OR desc_formula)=8/37")
                && report.contains("Cavalier Order: bonus_vars=1/8, combined(bonus_vars OR desc_formula)=2/8"),
            "the honest all-resolver figure this cycle re-derived must reproduce exactly -- a \
             future cycle's corpus/resolver change that moves these numbers must update this \
             assertion deliberately, never silently:\n{report}"
        );
    }

    /// Celestial is a real Sorcerer bloodline (`data/corpus/advanced_class_guide/
    /// class_feature/celestial_bloodline/*.json`, 53 real bloodline groups total
    /// per the cycle-4 census) this codebase has never hand-modelled by name --
    /// only Arcane and Draconic are. Its "Ascension" member (`Sorcerer_
    /// CelestialAscension_ResistanceBonus|10`, verified directly against the
    /// corpus) is a flat constant needing no external chain var, so it resolves
    /// through the shared resolver without any hand-picked `ground_sorcerer_*`
    /// function. (A direct corpus survey this cycle ran across all 53 real
    /// Bloodline groups found only 15 -- Aquatic, Arcane, Boreal, Celestial,
    /// Destined, Dreamspun, Empyreal, Infernal, Protean, Sage, Serpentine,
    /// Shadow, Starsoul, Stormborn, Verdant -- carry a member resolvable
    /// WITHOUT the missing per-bloodline header chain; the other 38 correctly
    /// refuse, see this cycle's receipt.)
    #[test]
    fn sorcerer_generic_bloodline_pass_grounds_a_never_hand_modelled_bloodline() {
        let input =
            class_input(SORCERER_CLASS_ID, 5, SORCERER_BLOODLINE_CHOICE_ID, "bloodline:celestial");
        let count = generic_explanation_count(&input, "class_feature.sorcerer.bloodline.generic");
        assert!(
            count > 0,
            "Celestial Bloodline must ground at least one real corpus member generically"
        );
    }

    /// SD-32 T12 Epic 8 row 18 cycle 22 (`§27b`): Karmic is a real Wildblooded Sorcerer
    /// Bloodline VARIANT (`data/corpus/ultimate_magic/class_feature/karmic_bloodline/*.json`,
    /// `Wildblooded ~ Karmic`'s own `PREABILITY` corpus-declares its real parent, `Destined`).
    /// Its "Fate's Retribution" member's DC formula
    /// (`10+(Sorcerer_Destined_BloodlinePower1LVL/2)+Sorcerer_Spells_StatBonus`) needed BOTH
    /// cycle 21's seventh shape (the Wildblooded-parent-header recursion, for
    /// `Sorcerer_Destined_BloodlinePower1LVL`) AND this cycle's eighth shape (`"Sorcerer ~
    /// Spells"`'s own `Sorcerer_Spells_StatBonus|CHA`) to ground -- proves the two shapes
    /// compose correctly on a real character, not just in the census's own group-membership
    /// tally. At character level 20, CHA modifier 0 (`AbilityModifiers::default()`), this
    /// resolves to exactly `Sorcerer_KarmicFatesRetribution_DC = 30`
    /// (`10 + ((BloodlineLVL=2*SorcererLVL=40)/2=20) + CHA(0)`), matching the direct-resolver
    /// trace this cycle's own receipt cites.
    #[test]
    fn sorcerer_generic_bloodline_pass_grounds_karmic_wildblooded_variant() {
        let input =
            class_input(SORCERER_CLASS_ID, 20, SORCERER_BLOODLINE_CHOICE_ID, "bloodline:karmic");
        let receipt = build_pilot_headless_receipt(&input);
        let explanation = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id.starts_with("class_feature.sorcerer.bloodline.generic"));
        assert!(
            explanation.is_some(),
            "Karmic Bloodline must ground at least one real corpus member generically"
        );
    }

    /// Plant is a real Cleric domain this codebase's `DOMAIN_POWER_CATALOG`
    /// does NOT hand-model (only Good/War/Strength/Destruction/Glory/Healing
    /// are), one of the 67 remaining real domain groups the cycle-4 census
    /// names. Its "Wooden Fist" member (`WoodenFistRounds|3+WIS`) needs only
    /// an ability modifier, resolving without the missing per-domain header
    /// chain. (A direct corpus survey found only 5 of the 67 remaining real
    /// Domain groups -- Heresy, Oblivion, Plant, Tactics, Wolf -- carry such a
    /// member; the rest correctly refuse, see this cycle's receipt.)
    #[test]
    fn cleric_generic_domain_pass_grounds_a_never_hand_modelled_domain() {
        let input = class_input(CLERIC_CLASS_ID, 5, CLERIC_DOMAIN_CHOICE_ID, "domain:plant");
        let count = generic_explanation_count(&input, "class_feature.cleric.domain.generic");
        assert!(count > 0, "Plant Domain must ground at least one real corpus member generically");
    }

    /// SD-32 T12 Epic 8 row 18 cycle 19: the desc-formula resolver's own missing header-merge
    /// step, named by coordinate in cycle 18's own receipt (`§3`: "the desc-formula resolver has
    /// no header-merge step of its own to find it"). `Mountain Domain ~ Foothold`'s real corpus
    /// `%1` argument is the bare identifier `DomainMountainTimes`, chaining through the
    /// domain-kind header cycle 18 added (`DomainMountainTimes|DomainPowerTimes|TYPE=Domain`) to
    /// Cleric Domain's own bare `"Domains"` base-header record (`DomainPowerTimes|3+WIS`) --
    /// UNRESOLVABLE with an empty header map (proves the gap was real), resolvable once
    /// `pool_group_header_vars_merged` is passed in (proves the fix, mirroring cycle 8's own
    /// header-merge fix for the OTHER resolver, `§17`).
    #[test]
    fn mountain_domain_foothold_desc_formula_needs_the_header_merge_to_resolve() {
        let ability_modifiers = crate::rules_core::pilot_compute::AbilityModifiers::default();
        let key = "Mountain Domain ~ Foothold";
        let empty_header_vars = crate::rules_core::record_vars::ConvertedChain::new();
        assert!(
            super::class_feature_grant_consumer::resolved_description_for_formula_only_desc_argument(
                key,
                5,
                &ability_modifiers,
                &empty_header_vars,
            )
            .is_none(),
            "without the header merge, Foothold's %1 argument (DomainMountainTimes) must stay \
             unresolvable -- proves the pre-cycle-19 gap was real, not already closed some other way"
        );
        let header_vars =
            super::pool_group_header_vars_merged("Cleric", "Mountain Domain", Some("Domain"));
        let resolved =
            super::class_feature_grant_consumer::resolved_description_for_formula_only_desc_argument(
                key,
                5,
                &ability_modifiers,
                &header_vars,
            );
        assert!(
            resolved.is_some(),
            "with the real header chain merged in, Foothold's %1 argument must resolve -- \
             DomainMountainTimes -> DomainPowerTimes -> 3+WIS"
        );
        let (_, value) = resolved.unwrap();
        assert_eq!(
            value, 3,
            "DomainPowerTimes = 3+WIS, WIS modifier 0 under AbilityModifiers::default()"
        );
    }

    /// Oracle Mystery is DELIBERATELY NOT wired to
    /// `push_generic_pool_group_selection_magnitude` (see the withdrawal
    /// comment at `ground_or_block_oracle_class_features`'s own call site):
    /// unlike Domain/Bloodline, most Mystery members are budgeted
    /// REVELATIONS this codebase already, correctly, gates on an explicit
    /// `ORACLE_REVELATION_CHOICE_ID` pick -- a blanket "selecting the
    /// mystery alone grants every member" pass is factually wrong for this
    /// pool. Wiring it live tripped this file's own pre-existing regression
    /// (`oracle_dispatch_widening_safety_tests::
    /// a_mystery_pick_alone_grounds_no_tier_one_revelation`), which is
    /// authoritative and was correctly left unweakened. This test proves
    /// the withdrawal: no `class_feature.apg.oracle.mystery.generic`
    /// explanation is ever emitted, on a mystery this file has never
    /// hand-modelled by name (Ancestor) or one it has (Lore).
    #[test]
    fn oracle_generic_mystery_pass_is_deliberately_not_wired() {
        for selection in ["mystery:ancestor", "mystery:lore"] {
            let input = class_input(ORACLE_CLASS_ID, 5, ORACLE_MYSTERY_CHOICE_ID, selection);
            let count = generic_explanation_count(&input, "class_feature.apg.oracle.mystery.generic");
            assert_eq!(
                count, 0,
                "Oracle Mystery must never emit a generic group-selection explanation for \
                 {selection} -- the mechanism is withdrawn for this pool, not merely quiet"
            );
        }
    }

    /// Air is a real Warpriest blessing this file's Destruction/Strength
    /// hand-modelling does not cover, one of the 36 remaining real blessing
    /// groups the cycle-4 census names. Unlike Bloodline/Domain/Mystery,
    /// Blessing genuinely closes NOTHING new: a direct corpus survey across
    /// all 38 real Blessing groups found not one single-terminal member whose
    /// formula avoids both the missing per-blessing header chain AND a
    /// `classlevel("Warpriest")` call this resolver deliberately refuses to
    /// bank through (`formula_interpreter.rs`'s own documented cross-class
    /// gap). This is the honest "cannot be wired without extending the
    /// resolver itself" case the brief asks to name rather than force --
    /// proven here as a safe, unweakened refusal, not silently skipped.
    #[test]
    fn warpriest_generic_blessing_pass_correctly_refuses_every_unmodelled_blessing() {
        let input = class_input(WARPRIEST_CLASS_ID, 5, WARPRIEST_BLESSING_CHOICE_ID, "blessing:air");
        let count = generic_explanation_count(&input, "class_feature.acg.warpriest.blessing.generic");
        assert_eq!(
            count, 0,
            "Air Blessing is a real, recognized selection with real corpus members -- this must \
             be a genuine, safe refusal (no resolvable single-terminal member exists yet), not a \
             silent miss on a selection this pool fails to recognize at all"
        );
    }

    /// Wood is a real Shaman spirit beyond this file's ten hand-picked spirits
    /// (Life/Battle/Bones/Flame/Heavens/Lore/Nature/Stone/Waves/Wind), one of
    /// the 4 remaining real spirit groups the cycle-4 census names (Mammoth,
    /// Wood, and two internal `Shaman(...)Spirit` dispatcher groups). A direct
    /// corpus survey found none of the 4 carry a member resolvable without the
    /// missing per-spirit header chain -- same honest "cannot close without
    /// extending the resolver" finding as Blessing, proven as a safe refusal.
    #[test]
    fn shaman_generic_spirit_pass_correctly_refuses_every_unmodelled_spirit() {
        let input = class_input(SHAMAN_CLASS_ID, 5, SHAMAN_SPIRIT_CHOICE_ID, "spirit:wood");
        let count = generic_explanation_count(&input, "class_feature.acg.shaman.spirit.generic");
        assert_eq!(
            count, 0,
            "Wood Spirit is a real, recognized selection with real corpus members -- this must be \
             a genuine, safe refusal, not a silent miss on a selection this pool fails to \
             recognize at all"
        );
    }

    /// Safety: an invented selection resolves to nothing, on any of the five
    /// wired pools -- the same "refuse rather than fabricate" contract every
    /// prior cycle's own generic resolver already proves.
    #[test]
    fn invented_selections_ground_nothing_on_any_wired_pool() {
        let cases: &[(&str, u8, &str, &str, &str)] = &[
            (
                SORCERER_CLASS_ID,
                5,
                SORCERER_BLOODLINE_CHOICE_ID,
                "bloodline:not_a_real_bloodline",
                "class_feature.sorcerer.bloodline.generic",
            ),
            (
                CLERIC_CLASS_ID,
                5,
                CLERIC_DOMAIN_CHOICE_ID,
                "domain:not_a_real_domain",
                "class_feature.cleric.domain.generic",
            ),
            (
                WARPRIEST_CLASS_ID,
                5,
                WARPRIEST_BLESSING_CHOICE_ID,
                "blessing:not_a_real_blessing",
                "class_feature.acg.warpriest.blessing.generic",
            ),
            (
                SHAMAN_CLASS_ID,
                5,
                SHAMAN_SPIRIT_CHOICE_ID,
                "spirit:not_a_real_spirit",
                "class_feature.acg.shaman.spirit.generic",
            ),
        ];
        for (class_id, level, choice_set_id, selection_id, id_prefix) in cases {
            let input = class_input(class_id, *level, choice_set_id, selection_id);
            assert_eq!(
                generic_explanation_count(&input, id_prefix),
                0,
                "an invented selection ({choice_set_id} -> {selection_id}) must never ground \
                 anything"
            );
        }
    }

    /// Undead is a real Bloodrager bloodline (`data/corpus/advanced_class_guide/class_feature/
    /// undead_bloodrager_bloodline/*.json`, 12 real Bloodrager bloodline groups total) this file
    /// has never hand-modelled by name -- only Arcane is (`ground_bloodrager_arcane_bloodline`).
    /// A direct corpus survey this cycle ran across all 12 real Bloodrager Bloodline groups found
    /// 4 -- Aberrant, Arcane, Elemental, Undead -- carry a member resolvable WITHOUT the missing
    /// per-bloodline header chain; the other 8 correctly refuse, same shape as Sorcerer's own
    /// Bloodline pool (this cycle's receipt).
    #[test]
    fn bloodrager_generic_bloodline_pass_grounds_a_never_hand_modelled_bloodline() {
        let input = class_input(BLOODRAGER_CLASS_ID, 5, BLOODRAGER_BLOODLINE_CHOICE_ID, "bloodline:undead");
        let count = generic_explanation_count(&input, "class_feature.acg.bloodrager.bloodline.generic");
        assert!(count > 0, "Undead Bloodline must ground at least one real corpus member generically");
    }

    /// Safety: the generic Bloodrager Bloodline pass never fires for Arcane, the ONE bloodline
    /// this file already hand-models via `ground_bloodrager_arcane_bloodline` -- proving the two
    /// mechanisms cannot double-count even though Arcane's own corpus members would otherwise
    /// resolve through the generic pass too (its own registered-name/namespace match is identical
    /// to any other Bloodrager bloodline). `ground_bloodrager_arcane_bloodline`'s own explanation
    /// ids are `class_feature.acg.bloodrager.bloodline.arcane.*`, a DIFFERENT id prefix from the
    /// generic pass's `class_feature.acg.bloodrager.bloodline.generic.*` -- both may legitimately
    /// fire side by side without collision, mirroring the Sorcerer Arcane/Draconic contract cycle
    /// 5 already established; this test only proves the generic id itself does not go empty
    /// (still resolves something) for the hand-modelled bloodline too, since Arcane's own corpus
    /// members are real records like any other.
    #[test]
    fn bloodrager_generic_bloodline_pass_does_not_collide_with_the_hand_modelled_arcane_bloodline() {
        let input = class_input(BLOODRAGER_CLASS_ID, 5, BLOODRAGER_BLOODLINE_CHOICE_ID, "bloodline:arcane");
        let generic_count =
            generic_explanation_count(&input, "class_feature.acg.bloodrager.bloodline.generic");
        let hand_modelled_count =
            generic_explanation_count(&input, "class_feature.acg.bloodrager.bloodline.arcane");
        assert!(hand_modelled_count > 0, "the pre-existing hand-modelled Arcane branch must still fire");
        assert!(
            generic_count > 0,
            "the generic pass must independently ground Arcane's own resolvable corpus members too"
        );
    }

    /// Green is a real Cavalier order (`data/corpus/*/class_feature/order_of_the_green/*.json`)
    /// this file has never hand-modelled by name -- only Order of the Sword is
    /// (`ground_cavalier_named_features`'s own `ORDER_OF_THE_SWORD_SELECTION` branch). SD-32 T12
    /// Epic 8 row 18 cycle 7's THIRD real corpus naming/ownership shape
    /// (`real_pool_group_for_selection_slug`'s own doc) is what makes "Order of the Green" findable
    /// at all -- its own group name does not end in `" Order"`, so neither of cycles 5/6's two
    /// existing branches would ever match it; its own corpus `class` field is also
    /// self-referential (`"Order of the Green"`, never `"Cavalier"`), so it is only admitted
    /// because a real, independently-verified `"Cavalier Order ~ Order of the Green"` CHOOSER
    /// header record also exists and is itself tagged `class: "Cavalier"` (the ownership-proof
    /// fallback this cycle added). Its "Favored Terrain" member (`BONUS:VAR|FavoredTerrainPool|
    /// (CavalierFavoredTerrainLVL+4)/6` chained to the SAME record's own
    /// `BONUS:VAR|CavalierFavoredTerrainLVL|CavalierLVL`) is a genuine two-hop, self-contained
    /// chain needing no missing header var, resolving without any hand-picked `ground_cavalier_*`
    /// function. A direct corpus survey this cycle ran across all 8 real, Cavalier-owned Order
    /// groups (Beast, Guard, Eastern Star, Shroud, Blue Rose, Green, Seal, Tome) found exactly ONE
    /// -- Green -- carries a member resolvable this way; Guard and Tome each carry one real
    /// numeric member too (`BONUS:SKILL|...`, Knowledge (nobility) / Linguistics), but
    /// `parse_bonus_var_tokens_pre_gate_safe` only ever reads `BONUS:VAR` tokens (this module's
    /// own documented scope), so a `BONUS:SKILL`-only member correctly refuses, same shape as
    /// every other pool's own `classlevel(...)`/missing-header refusals; the other 5 orders are
    /// pure DESC-only display members (`§7` DONE already, not a resolver gap, the same correction
    /// cycle 6 made for Warpriest Blessing).
    #[test]
    fn cavalier_generic_order_pass_grounds_a_never_hand_modelled_order() {
        let input = class_input(CAVALIER_CLASS_ID, 5, CAVALIER_ORDER_CHOICE_ID, "order:green");
        let count = generic_explanation_count(&input, "class_feature.apg.cavalier.order.generic");
        assert!(count > 0, "Order of the Green must ground at least one real corpus member generically");
    }

    /// Safety: the generic Cavalier Order pass never fires for Sword, the ONE order this file
    /// already hand-models via the `ORDER_OF_THE_SWORD_SELECTION` branch in
    /// `ground_cavalier_named_features` -- proving the two mechanisms cannot double-count even
    /// though "Order of the Sword" has no `"Cavalier Order ~ Order of the Sword"` CHOOSER header at
    /// all (its own corpus members are tagged `class: "Order of the Sword"`, matching neither the
    /// majority-class check nor the header-ownership-proof fallback) -- the generic pass correctly,
    /// safely refuses it outright, never colliding with the hand-modelled branch.
    #[test]
    fn cavalier_generic_order_pass_does_not_collide_with_the_hand_modelled_order_of_the_sword() {
        let input = class_input(CAVALIER_CLASS_ID, 5, CAVALIER_ORDER_CHOICE_ID, "order:sword");
        let generic_count =
            generic_explanation_count(&input, "class_feature.apg.cavalier.order.generic");
        let hand_modelled_count =
            generic_explanation_count(&input, "class_feature.apg.cavalier.order_of_the_sword");
        assert!(hand_modelled_count > 0, "the pre-existing hand-modelled Order of the Sword branch must still fire");
        assert_eq!(
            generic_count, 0,
            "Order of the Sword has no Cavalier-owned CHOOSER header in this corpus, so the \
             generic pass must safely refuse it (not fabricate a match), never colliding with the \
             hand-modelled branch"
        );
    }

    /// Safety: an invented selection resolves to nothing on the Cavalier Order pool either.
    #[test]
    fn invented_cavalier_order_selection_grounds_nothing() {
        let input =
            class_input(CAVALIER_CLASS_ID, 5, CAVALIER_ORDER_CHOICE_ID, "order:not_a_real_order");
        assert_eq!(
            generic_explanation_count(&input, "class_feature.apg.cavalier.order.generic"),
            0,
            "an invented Cavalier Order selection must never ground anything"
        );
    }

    /// Abyssal is a real Sorcerer bloodline cycle 7 found blocked (its own per-bloodline header,
    /// `"Abyssal Bloodline"`, chains `Sorcerer_Abyssal_BloodlineLVL -> BloodlineLVL`, and
    /// `BloodlineLVL` itself binds only on the corpus-wide, cross-book-duplicated `"Bloodline
    /// Tracker"` record -- SD-32 T12 Epic 8 row 18 cycle 8's own `registered_name_for_tracker`
    /// widening in `resolve_pool_member_sole_magnitude`). `§17a` re-derivation this cycle (after
    /// row 21 restored the per-book `.MOD` rows `"Bloodline Tracker"` itself carries) found the
    /// var STILL unbound: 8 separate book files share the SAME bare key, and the prior
    /// `or_insert_with` kept only whichever book sorted first alphabetically
    /// (`advanced_class_guide`, a single leftover `DEFINE`) over `core_rulebook`'s own complete
    /// 308-token copy -- a second, cross-book collision loss surviving row 21's per-file fix.
    /// `class_feature_bonus_vars_any_record`'s own MERGE-across-books widening (this cycle) is
    /// what makes `BloodlineLVL` findable at all. Its "Bloodline Arcana" member (`Sorcerer_
    /// AbyssalBloodlineArcana_SummonDR|max(Sorcerer_Abyssal_BloodlineLVL/2,1)`) is real level-
    /// scaled magnitude.
    #[test]
    fn sorcerer_generic_bloodline_pass_grounds_abyssal_via_the_cross_book_tracker_merge() {
        let input = class_input(SORCERER_CLASS_ID, 5, SORCERER_BLOODLINE_CHOICE_ID, "bloodline:abyssal");
        let count = generic_explanation_count(&input, "class_feature.sorcerer.bloodline.generic");
        assert!(count > 0, "Abyssal Bloodline must ground at least one real corpus member generically");
    }

    /// Verdant is a real Bloodrager bloodline cycle 7 found blocked, needing `BloodragerBloodlineLVL`
    /// -- bound on `"Bloodrager ~ Bloodline Tracker"` (Bloodrager's own, differently-shaped tracker
    /// key, `"<class> ~ Bloodline Tracker"` rather than Sorcerer's bare `"Bloodline Tracker"`),
    /// found via the SAME generic `registered_name_for_tracker` widening (this cycle) reusing
    /// `pool_header_record_by_normalized_suffix` unchanged -- no per-class special case. Its "Oaken
    /// Skin" member (`VerdantBloodragerOakenSkin|1+(BloodragerBloodlineLVL/4)`) is real level-
    /// scaled magnitude.
    #[test]
    fn bloodrager_generic_bloodline_pass_grounds_verdant_via_the_tracker_merge() {
        let input = class_input(BLOODRAGER_CLASS_ID, 5, BLOODRAGER_BLOODLINE_CHOICE_ID, "bloodline:verdant");
        let count = generic_explanation_count(&input, "class_feature.acg.bloodrager.bloodline.generic");
        assert!(count > 0, "Verdant Bloodline must ground at least one real corpus member generically");
    }

    /// Animal is a real Cleric domain cycle 7 found blocked, needing `DomainAnimalLVL` (the
    /// "Animal Domain" header's own chain) which itself needs `DomainLVL` -- bound ONLY on the
    /// CLERIC CLASS RECORD (`data/corpus/core_rulebook/class/cleric.json`, `BONUS:VAR|DomainLVL|
    /// ClericLVL`), never on any `class_feature` record at all (cycle 7's own receipt, "out of
    /// `class_feature`'s ingestion scope entirely"). Row 21 restored the ingest token array onto all 168 real
    /// class records; this cycle's own NEW `class_record_bonus_vars` table (mirroring `class_
    /// feature_bonus_vars_any_record`'s shape one dir level up) is the missing READ side, merged
    /// into `resolve_pool_member_sole_magnitude` unconditionally (every owning class, not gated by
    /// a family flag). Its "Animal Companion" member (`AnimalCompanionMasterLVL|DomainAnimalLVL-3`)
    /// is a genuine two-hop chain terminating on the class record.
    #[test]
    fn cleric_generic_domain_pass_grounds_animal_via_the_class_record_merge() {
        let input = class_input(CLERIC_CLASS_ID, 5, CLERIC_DOMAIN_CHOICE_ID, "domain:animal");
        let count = generic_explanation_count(&input, "class_feature.cleric.domain.generic");
        assert!(count > 0, "Animal Domain must ground at least one real corpus member generically");
    }

    /// SD-32 T12 Epic 8 row 18 cycle 20 (`§27b`/`§17`): proves the SIXTH real header shape,
    /// `"Domain Base ~ <bare>"` (`data.class` literally `"Domain Base"`, `pool_header_record_by_
    /// normalized_suffix`'s own new doc) reaches Void Domain's real corpus chain end to end.
    /// `Void Domain ~ Part the Veil`'s own `%N` desc-formula argument (`DomainVoidTimes`) was
    /// unreachable before this cycle because its ONLY binding lived on `"Domain Base ~ Void"`, a
    /// key shape no prior header clause tried. Tested at the same resolver level
    /// `mountain_domain_foothold_desc_formula_needs_the_header_merge_to_resolve` above uses --
    /// this pool has no production `push_generic_pool_group_selection_description_magnitude`
    /// call site wired yet (only Cavalier Order and Warpriest Blessing do; a pre-existing gap,
    /// unrelated to and not fixed by this cycle, named here rather than silently worked around).
    #[test]
    fn cleric_generic_domain_pass_grounds_void_via_the_domain_base_header_merge() {
        let ability_modifiers = crate::rules_core::pilot_compute::AbilityModifiers::default();
        let key = "Void Domain ~ Part the Veil";
        let header_vars = super::pool_group_header_vars_merged("Cleric", "Void Domain", Some("Domain"));
        let resolved =
            super::class_feature_grant_consumer::resolved_description_for_formula_only_desc_argument(
                key,
                5,
                &ability_modifiers,
                &header_vars,
            );
        assert!(
            resolved.is_some(),
            "Part the Veil's %1 argument (DomainVoidTimes) must resolve through the new \
             \"Domain Base ~ Void\" header merge: {resolved:?}"
        );
    }

    /// SD-32 T12 Epic 8 row 18 cycle 20 correction (`§27b`/`§17a`): cycle 19's own exhaustive
    /// verification claimed `Scalykind` (one of its named 12 "genuine hard-impossibility data
    /// gaps") carries "no BONUS:VAR-bearing header exists anywhere in the corpus" -- that check
    /// covered the `domain`-kind and bare `class_feature` header shapes only, never the THIRD,
    /// `class_feature/domain_base/` directory this cycle found (`"Domain Base ~ Scalykind"` is
    /// real, `BONUS:VAR|DomainScalykindLVL|DomainLVL`). This test proves the correction:
    /// `resolve_pool_member_all_magnitudes` now genuinely resolves a real Scalykind member,
    /// disproving the prior hard-impossibility claim for this ONE domain (the other 11 named
    /// domains were re-checked this cycle and remain genuinely headerless in all three shapes).
    #[test]
    fn cleric_generic_domain_pass_grounds_scalykind_correcting_cycle_19s_hard_impossibility_claim() {
        let ability_modifiers = crate::rules_core::pilot_compute::AbilityModifiers::default();
        let table = super::class_feature_grant_consumer::class_feature_record_tokens_pre_gate_safe();
        let prefix = "Scalykind Domain ~ ";
        let resolved_any = table.keys().filter(|key| key.starts_with(prefix)).any(|key| {
            !super::resolve_pool_member_all_magnitudes(
                key,
                "Scalykind Domain",
                5,
                &ability_modifiers,
                Some("Cleric"),
                Some("Domain"),
            )
            .is_empty()
        });
        assert!(
            resolved_any,
            "Scalykind Domain must now have at least one real corpus member resolve \
             generically, correcting cycle 19's own hard-impossibility claim"
        );
    }

    /// SD-32 T12 Epic 8 row 18 cycle 15: `push_generic_pool_group_selection_description_magnitude`,
    /// the sibling generic pass cycle 14's own `§16` finding named -- Earth Blessing carries no
    /// `BONUS:VAR` at all (`resolve_pool_member_sole_magnitude` returns `None` for it, confirmed
    /// by the `class_feature.acg.warpriest.blessing.generic` prefix staying empty below), but its
    /// own "Armor of Earth" member carries a real `%1`-substituted DESC formula
    /// (`"if(WarpriestLVL<19,1+((WarpriestLVL/2)-5),5)"`) this new resolver reaches directly.
    #[test]
    fn warpriest_generic_blessing_description_pass_grounds_a_zero_bonus_var_blessing() {
        let input = class_input(WARPRIEST_CLASS_ID, 8, WARPRIEST_BLESSING_CHOICE_ID, "blessing:earth");
        let old_shape_count =
            generic_explanation_count(&input, "class_feature.acg.warpriest.blessing.generic");
        assert_eq!(
            old_shape_count, 0,
            "Earth Blessing's own members carry no BONUS:VAR at all -- the bonus_vars-only \
             resolver must find nothing"
        );
        let description_count = generic_explanation_count(
            &input,
            "class_feature.acg.warpriest.blessing_description.generic",
        );
        assert!(
            description_count > 0,
            "Earth Blessing's own %N-substituted DESC formula must ground generically through \
             the new description resolver"
        );
    }

    /// The same proof for Cavalier Order of the Beast -- ZERO `BONUS:VAR` tokens on any of its
    /// own real members (cycle 12's own finding, unchanged), but "Wild Mount Shape" carries a
    /// real `%1`-substituted DESC formula (`%1` = a bare `CavalierLVL` reference) this new
    /// resolver reaches. "Class Skills" (`max(floor(CavalierLVL/2))`, a single-argument `max()`
    /// call) is a real corpus shape the formula interpreter's own documented `min`/`max`
    /// "at least 2 arguments" rule refuses -- an honest refusal, not fabricated, so it does not
    /// need to be the member this test proves through.
    #[test]
    fn cavalier_generic_order_description_pass_grounds_order_of_the_beast() {
        let input = class_input(CAVALIER_CLASS_ID, 10, CAVALIER_ORDER_CHOICE_ID, "order:beast");
        let old_shape_count =
            generic_explanation_count(&input, "class_feature.apg.cavalier.order.generic");
        assert_eq!(
            old_shape_count, 0,
            "Order of the Beast's own members carry no BONUS:VAR at all -- the bonus_vars-only \
             resolver must find nothing"
        );
        let description_count = generic_explanation_count(
            &input,
            "class_feature.apg.cavalier.order_description.generic",
        );
        assert!(
            description_count > 0,
            "Order of the Beast's own %N-substituted DESC formula must ground generically \
             through the new description resolver"
        );
    }

    /// Safety: the new description resolver never fires a SECOND time for Destruction or
    /// Strength Blessing, both already grounded by their own dedicated, activation-gated
    /// functions -- proving `already_hand_modelled_keys` genuinely excludes them rather than
    /// merely happening not to collide.
    #[test]
    fn warpriest_generic_blessing_description_pass_never_double_grounds_destruction_or_strength() {
        for selection in ["blessing:destruction", "blessing:strength"] {
            let input = class_input(WARPRIEST_CLASS_ID, 8, WARPRIEST_BLESSING_CHOICE_ID, selection);
            let description_count = generic_explanation_count(
                &input,
                "class_feature.acg.warpriest.blessing_description.generic",
            );
            assert_eq!(
                description_count, 0,
                "{selection} is already hand-modelled -- the generic description pass must \
                 exclude its own key, never emit a duplicate explanation for the same magnitude"
            );
        }
    }

    /// Safety: an invented Blessing/Order selection grounds nothing through the new resolver
    /// either -- the same "refuse rather than fabricate" contract every other generic pass proves.
    #[test]
    fn invented_selections_ground_nothing_through_the_description_resolver() {
        let cases: &[(&str, u8, &str, &str, &str)] = &[
            (
                WARPRIEST_CLASS_ID,
                8,
                WARPRIEST_BLESSING_CHOICE_ID,
                "blessing:not_a_real_blessing",
                "class_feature.acg.warpriest.blessing_description.generic",
            ),
            (
                CAVALIER_CLASS_ID,
                10,
                CAVALIER_ORDER_CHOICE_ID,
                "order:not_a_real_order",
                "class_feature.apg.cavalier.order_description.generic",
            ),
        ];
        for (class_id, level, choice_set_id, selection_id, id_prefix) in cases {
            let input = class_input(class_id, *level, choice_set_id, selection_id);
            assert_eq!(
                generic_explanation_count(&input, id_prefix),
                0,
                "an invented selection ({choice_set_id} -> {selection_id}) must never ground \
                 anything through the description resolver"
            );
        }
    }
}

