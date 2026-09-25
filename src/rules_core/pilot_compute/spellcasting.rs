#[allow(unused_imports)]
pub(crate) use super::*;

/// v0.6 alpha swarm: the "later cycle" the four bounded MVP spellcasting
/// ceilings explicitly deferred to.
///
/// `WIZARD_SPELLBOOK_SUPPORTED_MAX_LEVEL`,
/// `ARCANIST_SPELLBOOK_SUPPORTED_MAX_LEVEL`,
/// `WARPRIEST_SPELLBOOK_SUPPORTED_MAX_LEVEL` and
/// `ORACLE_KNOWN_SPELLS_SUPPORTED_MAX_LEVEL` each shipped at 3 because only
/// levels 1-3 of their per-level tables had been transcribed and verified.
/// These tests pin the REAL remaining rows, transcribed literally from the
/// PCGen corpus (see each table function's own doc comment for the exact
/// file and line range), at levels 4, 10 and 20 -- not merely "the ceiling
/// diagnostic no longer fires". A wrong number here is worse than a blocked
/// class, so every row asserted below is the raw corpus row.
///
/// Rows are compared as `Vec` rather than fixed-width arrays so each
/// assertion expresses the intended width (10 columns, spell levels 0-9)
/// independently of the array type it is compared against.
#[cfg(test)]
mod spellcasting_level_cap_widening_tests {
    use super::{
        arcanist_base_spells_per_day, oracle_spells_known_table, warpriest_base_spells_per_day,
        wizard_base_spells_per_day, ARCANIST_SPELLBOOK_SUPPORTED_MAX_LEVEL,
        ORACLE_KNOWN_SPELLS_SUPPORTED_MAX_LEVEL, WARPRIEST_SPELLBOOK_SUPPORTED_MAX_LEVEL,
        WIZARD_SPELLBOOK_SUPPORTED_MAX_LEVEL,
    };

    /// Build one expected row from the corpus's own comma-separated `CAST:`/
    /// `KNOWN:` shape: trailing/absent columns and explicit `0`s both mean an
    /// inaccessible ("--") spell level.
    fn row(cols: &[i16]) -> Vec<Option<i16>> {
        let mut out: Vec<Option<i16>> =
            cols.iter().map(|c| if *c > 0 { Some(*c) } else { None }).collect();
        out.resize(10, None);
        out
    }

    /// All four ceilings must now cover the full 1-20 class-level range.
    #[test]
    fn every_bounded_spellcasting_ceiling_now_covers_all_twenty_levels() {
        assert_eq!(WIZARD_SPELLBOOK_SUPPORTED_MAX_LEVEL, 20);
        assert_eq!(ARCANIST_SPELLBOOK_SUPPORTED_MAX_LEVEL, 20);
        assert_eq!(WARPRIEST_SPELLBOOK_SUPPORTED_MAX_LEVEL, 20);
        assert_eq!(ORACLE_KNOWN_SPELLS_SUPPORTED_MAX_LEVEL, 20);
    }

    /// `cr_classes.lst:306` / `:312` / `:322` (`CLASS:Wizard` level
    /// progression `CAST:` rows).
    #[test]
    fn wizard_spells_per_day_matches_the_raw_corpus_cast_rows() {
        assert_eq!(wizard_base_spells_per_day(4).to_vec(), row(&[4, 3, 2]), "cr_classes.lst:306");
        assert_eq!(
            wizard_base_spells_per_day(10).to_vec(),
            row(&[4, 4, 4, 3, 3, 2]),
            "cr_classes.lst:312"
        );
        assert_eq!(
            wizard_base_spells_per_day(20).to_vec(),
            row(&[4, 4, 4, 4, 4, 4, 4, 4, 4, 4]),
            "cr_classes.lst:322"
        );
        // The already-shipped, independently-verified level 1-3 rows must be
        // preserved byte-for-byte by this widening.
        assert_eq!(wizard_base_spells_per_day(1).to_vec(), row(&[3, 1]), "cr_classes.lst:303");
        assert_eq!(wizard_base_spells_per_day(3).to_vec(), row(&[4, 2, 1]), "cr_classes.lst:305");
    }

    /// `acg_abilities_class.lst:77`, `KEY:Arcanist ~ Spells Prepared`
    /// `BONUS:VAR|ArcanistPreparedLVL_*` formulas evaluated at each level.
    #[test]
    fn arcanist_spells_prepared_matches_the_raw_corpus_bonus_var_formulas() {
        assert_eq!(arcanist_base_spells_per_day(4).to_vec(), row(&[6, 3, 1]));
        assert_eq!(arcanist_base_spells_per_day(10).to_vec(), row(&[9, 5, 4, 3, 2, 1]));
        assert_eq!(arcanist_base_spells_per_day(20).to_vec(), row(&[9, 5, 5, 4, 4, 4, 3, 3, 3, 3]));
        assert_eq!(arcanist_base_spells_per_day(1).to_vec(), row(&[4, 2]));
        assert_eq!(arcanist_base_spells_per_day(3).to_vec(), row(&[5, 3]));
    }

    /// `acg_classes.lst:394` / `:400` / `:410` (`CLASS:Warpriest` level
    /// progression `CAST:` rows). Warpriest is a 6-level caster: spell
    /// levels 7-9 are never accessible, even at class level 20.
    #[test]
    fn warpriest_spells_per_day_matches_the_raw_corpus_cast_rows() {
        assert_eq!(
            warpriest_base_spells_per_day(4).to_vec(),
            row(&[4, 3, 1]),
            "acg_classes.lst:394"
        );
        assert_eq!(
            warpriest_base_spells_per_day(10).to_vec(),
            row(&[5, 5, 4, 3, 1]),
            "acg_classes.lst:400"
        );
        assert_eq!(
            warpriest_base_spells_per_day(20).to_vec(),
            row(&[5, 5, 5, 5, 5, 5, 5]),
            "acg_classes.lst:410"
        );
        assert_eq!(warpriest_base_spells_per_day(1).to_vec(), row(&[3, 1]), "acg_classes.lst:391");
        assert_eq!(warpriest_base_spells_per_day(3).to_vec(), row(&[4, 3]), "acg_classes.lst:393");

        let level20 = warpriest_base_spells_per_day(20);
        for (spell_level, slot) in level20.iter().enumerate().skip(7) {
            assert_eq!(
                *slot, None,
                "Warpriest is a 6-level caster: spell level {spell_level} must stay inaccessible"
            );
        }
    }

    /// `apg_classes.lst:118` / `:124` / `:134` (`CLASS:Oracle` level
    /// progression `KNOWN:` rows).
    #[test]
    fn oracle_spells_known_matches_the_raw_corpus_known_rows() {
        assert_eq!(oracle_spells_known_table(4).to_vec(), row(&[6, 3, 1]), "apg_classes.lst:118");
        assert_eq!(
            oracle_spells_known_table(10).to_vec(),
            row(&[9, 5, 4, 3, 2, 1]),
            "apg_classes.lst:124"
        );
        assert_eq!(
            oracle_spells_known_table(20).to_vec(),
            row(&[9, 5, 5, 4, 4, 4, 3, 3, 3, 3]),
            "apg_classes.lst:134"
        );
        assert_eq!(oracle_spells_known_table(1).to_vec(), row(&[4, 2]), "apg_classes.lst:115");
        assert_eq!(oracle_spells_known_table(3).to_vec(), row(&[5, 3]), "apg_classes.lst:117");
    }

    /// An out-of-range class level stays a real "no such class level"
    /// sentinel for all four tables -- widening the ceiling must not turn an
    /// out-of-range level into a silently-populated row.
    #[test]
    fn out_of_range_levels_still_return_a_fully_empty_row() {
        for level in [0u8, 21u8] {
            assert_eq!(wizard_base_spells_per_day(level).to_vec(), row(&[]), "level {level}");
            assert_eq!(arcanist_base_spells_per_day(level).to_vec(), row(&[]), "level {level}");
            assert_eq!(warpriest_base_spells_per_day(level).to_vec(), row(&[]), "level {level}");
            assert_eq!(oracle_spells_known_table(level).to_vec(), row(&[]), "level {level}");
        }
    }
}

/// v0.6 alpha swarm: the four classes whose last remaining blockers were
/// **spellcasting-shaped** -- Alchemist and Investigator (prepared
/// extracts), Warpriest (prepared spellbook), Bloodrager (its own spell
/// list plus the Bloodline slot its bonus spells hang off).
///
/// Every assertion here runs the SAME canonical seeds
/// `compose_character_input` (`apps/desktop/src-tauri/src/pf1_adapter.rs`)
/// applies at creation time, so a passing test is a claim about what a
/// real freshly created character of that class does in the shipped app,
/// not about a hypothetical hand-built input.
#[cfg(test)]
mod spellcasting_shaped_class_closure_tests {
    use super::{
        build_pilot_headless_receipt, compute_pilot_base_chassis, AcquisitionMode,
        CharacterClassLevel, CharacterInput, HeadlessReceiptStatus, ALCHEMIST_CLASS_ID,
        ALCHEMIST_DISCOVERY_CHOICE_ID,
        ARCANE_BLOODRAGER_BLOODLINE_SELECTION, BLOODRAGER_BLOODLINE_CHOICE_ID,
        BLOODRAGER_CLASS_ID, CANONICAL_EXTRACT_SPELL_ID, DESTRUCTION_BLESSING_SELECTION,
        FERAL_MUTAGEN_DISCOVERY_SELECTION, INVESTIGATOR_CLASS_ID, INVESTIGATOR_TALENT_CHOICE_ID,
        RESILIENCY_TALENT_SELECTION, WARPRIEST_BLESSING_CHOICE_ID, WARPRIEST_CLASS_ID,
        WARPRIEST_STARTER_SPELL_ID,
    };
    use crate::rules_core::character_input::{
        load_character_input_fixture, SelectedChoice, SpellSelection,
    };

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn choice(set: &str, selection: &str) -> SelectedChoice {
        SelectedChoice {
            choice_set_id: set.to_owned(),
            selection_id: selection.to_owned(),
        }
    }

    fn spell(spell_id: &str, class_id: &str, mode: AcquisitionMode) -> SpellSelection {
        SpellSelection {
            spell_id: spell_id.to_owned(),
            source_class_id: class_id.to_owned(),
            acquisition_mode: mode,
        }
    }

    /// The exact canonical seeds `compose_character_input` applies, per class.
    fn canonical_seeds(class_id: &str) -> (Vec<SelectedChoice>, Vec<SpellSelection>) {
        match class_id {
            ALCHEMIST_CLASS_ID => (
                vec![choice(
                    ALCHEMIST_DISCOVERY_CHOICE_ID,
                    FERAL_MUTAGEN_DISCOVERY_SELECTION,
                )],
                vec![
                    spell(CANONICAL_EXTRACT_SPELL_ID, ALCHEMIST_CLASS_ID, AcquisitionMode::Known),
                    spell(
                        CANONICAL_EXTRACT_SPELL_ID,
                        ALCHEMIST_CLASS_ID,
                        AcquisitionMode::Prepared,
                    ),
                ],
            ),
            INVESTIGATOR_CLASS_ID => (
                vec![choice(INVESTIGATOR_TALENT_CHOICE_ID, RESILIENCY_TALENT_SELECTION)],
                vec![
                    spell(
                        CANONICAL_EXTRACT_SPELL_ID,
                        INVESTIGATOR_CLASS_ID,
                        AcquisitionMode::Known,
                    ),
                    spell(
                        CANONICAL_EXTRACT_SPELL_ID,
                        INVESTIGATOR_CLASS_ID,
                        AcquisitionMode::Prepared,
                    ),
                ],
            ),
            WARPRIEST_CLASS_ID => (
                vec![choice(WARPRIEST_BLESSING_CHOICE_ID, DESTRUCTION_BLESSING_SELECTION)],
                vec![
                    spell(WARPRIEST_STARTER_SPELL_ID, WARPRIEST_CLASS_ID, AcquisitionMode::Known),
                    spell(
                        WARPRIEST_STARTER_SPELL_ID,
                        WARPRIEST_CLASS_ID,
                        AcquisitionMode::Prepared,
                    ),
                ],
            ),
            BLOODRAGER_CLASS_ID => (
                vec![choice(
                    BLOODRAGER_BLOODLINE_CHOICE_ID,
                    ARCANE_BLOODRAGER_BLOODLINE_SELECTION,
                )],
                Vec::new(),
            ),
            other => panic!("no canonical seeds defined for {other}"),
        }
    }

    fn seeded(class_id: &str, level: u8) -> CharacterInput {
        let mut input = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE)
            .character_input
            .expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
        let (choices, spells) = canonical_seeds(class_id);
        input.chosen.selected_choices.extend(choices);
        input.chosen.spells_selected.extend(spells);
        input
    }

    fn bare(class_id: &str, level: u8) -> CharacterInput {
        let mut input = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE)
            .character_input
            .expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
        input
    }

    fn blocking_ids(input: &CharacterInput) -> Vec<String> {
        build_pilot_headless_receipt(input)
            .computation
            .diagnostics
            .into_iter()
            .filter(|d| d.claim_blocking)
            .map(|d| d.id)
            .collect()
    }

    fn explanation_value(input: &CharacterInput, id: &str) -> Option<i16> {
        build_pilot_headless_receipt(input)
            .computation
            .explanations
            .into_iter()
            .find(|e| e.id == id)
            .map(|e| e.value)
    }

    /// The deliverable, stated once for all four: with the canonical
    /// creation seeds in place, every level 1-20 reaches `Computed`.
    #[test]
    fn all_four_spellcasting_shaped_classes_reach_computed_at_every_level() {
        for class_id in [
            ALCHEMIST_CLASS_ID,
            INVESTIGATOR_CLASS_ID,
            WARPRIEST_CLASS_ID,
            BLOODRAGER_CLASS_ID,
        ] {
            for level in 1..=20u8 {
                let input = seeded(class_id, level);
                assert_eq!(
                    build_pilot_headless_receipt(&input).status,
                    HeadlessReceiptStatus::Computed,
                    "{class_id} level {level} must reach Computed with its canonical creation \
                     seeds: {:?}",
                    blocking_ids(&input)
                );
            }
        }
    }

    /// The blockers are genuinely gated on the seeds, not deleted: with
    /// no canonical seeds at all, every one of the four stays `Blocked`.
    #[test]
    fn every_one_of_the_four_stays_blocked_without_its_canonical_seeds() {
        for class_id in [
            ALCHEMIST_CLASS_ID,
            INVESTIGATOR_CLASS_ID,
            WARPRIEST_CLASS_ID,
            BLOODRAGER_CLASS_ID,
        ] {
            for level in 1..=20u8 {
                let input = bare(class_id, level);
                assert_eq!(
                    build_pilot_headless_receipt(&input).status,
                    HeadlessReceiptStatus::Blocked,
                    "{class_id} level {level} must stay Blocked with no seeds"
                );
            }
        }
    }

    /// Alchemist's and Investigator's extract postures are one shared
    /// mechanism (`alchemist_spell_list`), so an extract seed for one
    /// must never satisfy the other's blocker.
    #[test]
    fn the_two_extract_classes_do_not_cross_satisfy_each_others_formula_books() {
        let mut alchemist = bare(ALCHEMIST_CLASS_ID, 5);
        alchemist.chosen.spells_selected.push(spell(
            CANONICAL_EXTRACT_SPELL_ID,
            INVESTIGATOR_CLASS_ID,
            AcquisitionMode::Known,
        ));
        assert!(
            blocking_ids(&alchemist)
                .contains(&"class_spell.apg.alchemist.prepared_extracts.unsupported".to_owned()),
            "an investigator-sourced extract must not fill an alchemist's formula book"
        );
    }

    /// The prepared-extract grounding is real, not a bypass: it reports
    /// the actual level-1 slot budget off the shipped table.
    #[test]
    fn the_extract_classes_ground_their_real_slot_budgets() {
        assert_eq!(
            explanation_value(
                &seeded(ALCHEMIST_CLASS_ID, 1),
                "class_spell.apg.alchemist.total_extracts_per_day.extract_level_1"
            ),
            Some(1),
            "alchemist level 1: base 1 + Intelligence bonus 0 (fixture Int 10) = 1"
        );
        assert_eq!(
            explanation_value(
                &seeded(INVESTIGATOR_CLASS_ID, 1),
                "class_spell.acg.investigator.total_extracts_per_day.extract_level_1"
            ),
            Some(1),
            "investigator level 1: base 1 + Intelligence bonus 0 = 1"
        );
    }

    /// Warpriest's Channel Energy uses per day: the corpus record says
    /// using it "expends two uses of his fervor ability", so the real
    /// daily count is the already-grounded Fervor pool halved. Fixture
    /// Wisdom 12 (+1): level 4 Fervor = 4/2 + 1 = 3, so 1 channel.
    #[test]
    fn warpriest_channel_energy_uses_are_the_fervor_pool_halved() {
        assert_eq!(
            explanation_value(
                &seeded(WARPRIEST_CLASS_ID, 4),
                "class_feature.acg.warpriest.channel_energy_uses_per_day"
            ),
            Some(1),
            "level 4: Fervor 3 uses / 2 uses per channel = 1 channel"
        );
        assert_eq!(
            explanation_value(
                &seeded(WARPRIEST_CLASS_ID, 20),
                "class_feature.acg.warpriest.channel_energy_uses_per_day"
            ),
            Some(5),
            "level 20: Fervor 11 uses / 2 = 5 channels"
        );
    }

    /// Below its own level-4 gate Channel Energy is correctly absent, not
    /// silently zero-with-no-record.
    #[test]
    fn warpriest_channel_energy_uses_are_absent_below_level_4() {
        assert_eq!(
            explanation_value(
                &seeded(WARPRIEST_CLASS_ID, 3),
                "class_feature.acg.warpriest.channel_energy_uses_per_day"
            ),
            Some(0),
            "correctly absent below the level-4 gate"
        );
    }

    /// Alchemist's canonical Discovery grounds its real corpus
    /// magnitudes, and only from its own level-2 grant gate.
    #[test]
    fn alchemist_feral_mutagen_discovery_grounds_from_its_own_grant_level() {
        assert_eq!(
            explanation_value(
                &seeded(ALCHEMIST_CLASS_ID, 1),
                "class_feature.apg.alchemist.discovery.feral_mutagen_bite_damage_die"
            ),
            None,
            "no Discovery is granted at alchemist level 1 (pool = level/2)"
        );
        assert_eq!(
            explanation_value(
                &seeded(ALCHEMIST_CLASS_ID, 2),
                "class_feature.apg.alchemist.discovery.feral_mutagen_bite_damage_die"
            ),
            Some(8),
            "Feral Mutagen bite: 1d8 for a Medium alchemist"
        );
        assert_eq!(
            explanation_value(
                &seeded(ALCHEMIST_CLASS_ID, 2),
                "class_feature.apg.alchemist.discovery.feral_mutagen_claw_damage_die"
            ),
            Some(6),
            "Feral Mutagen claws: 1d6 for a Medium alchemist"
        );
    }

    /// SD-32 T12 Epic 8 (`epic-2-cause-closure` row 18, pool-shaped class
    /// features): the GENERIC pool-choice magnitude resolver, proven
    /// end-to-end through `compute_pilot_base_chassis` (not a unit-call-
    /// only proof) on a real Discovery this codebase never hand-modelled
    /// by name -- `Discovery ~ Spontaneous Healing`
    /// (`ultimate_magic/class_feature/discovery/spontaneous_healing.json`,
    /// `BONUS:VAR|SpontaneousHealingAmount|floor(AlchemistDiscoveryLVL/2)*5`).
    /// `AlchemistDiscoveryLVL` is never bound by the member record itself
    /// -- it is only defined by the pool's own HEADER record
    /// (`Alchemist ~ Discovery`'s `BONUS:VAR|AlchemistDiscoveryLVL|
    /// AlchemistLVL`), so this also proves the header-chain merge is
    /// load-bearing, not decorative.
    #[test]
    fn spontaneous_healing_discovery_resolves_generically_through_the_pool_header_chain() {
        let mut level4 = bare(ALCHEMIST_CLASS_ID, 4);
        level4.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ALCHEMIST_DISCOVERY_CHOICE_ID.to_owned(),
            selection_id: "discovery:spontaneous_healing".to_owned(),
        });
        assert_eq!(
            explanation_value(
                &level4,
                "class_feature.apg.alchemist.discovery.generic.spontaneous_healing.\
                 spontaneoushealingamount"
            ),
            Some(10),
            "level 4: floor(AlchemistDiscoveryLVL/2)*5 = floor(4/2)*5 = 10"
        );

        let mut level10 = bare(ALCHEMIST_CLASS_ID, 10);
        level10.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ALCHEMIST_DISCOVERY_CHOICE_ID.to_owned(),
            selection_id: "discovery:spontaneous_healing".to_owned(),
        });
        assert_eq!(
            explanation_value(
                &level10,
                "class_feature.apg.alchemist.discovery.generic.spontaneous_healing.\
                 spontaneoushealingamount"
            ),
            Some(25),
            "level 10: floor(AlchemistDiscoveryLVL/2)*5 = floor(10/2)*5 = 25"
        );
    }

    /// A real selection recorded BELOW the pool's own grant gate must
    /// never ground a magnitude -- mirrors every hand-picked `ground_*`
    /// function's own level check (`alchemist_feral_mutagen_discovery_
    /// grounds_from_its_own_grant_level`, above).
    #[test]
    fn the_generic_resolver_stays_silent_below_the_pools_own_grant_level() {
        let mut level1 = bare(ALCHEMIST_CLASS_ID, 1);
        level1.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ALCHEMIST_DISCOVERY_CHOICE_ID.to_owned(),
            selection_id: "discovery:spontaneous_healing".to_owned(),
        });
        assert_eq!(
            explanation_value(
                &level1,
                "class_feature.apg.alchemist.discovery.generic.spontaneous_healing.\
                 spontaneoushealingamount"
            ),
            None,
            "AlchemistDiscoveryLVL/2 grants no Discovery yet at level 1"
        );
    }

    /// MUTATION-PROOF guard: an invented Discovery selection id must never
    /// resolve to a real corpus key or ground a real magnitude, even
    /// though it is recorded under the real, live `choice:
    /// alchemist_discovery` choice set id -- mirrors the Rage Power
    /// `an_invented_selection_id_never_grounds_the_superstition_bonus`
    /// guard's own contract for this new, generic resolver.
    #[test]
    fn an_invented_discovery_selection_never_grounds_a_generic_magnitude() {
        let mut level10 = bare(ALCHEMIST_CLASS_ID, 10);
        level10.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ALCHEMIST_DISCOVERY_CHOICE_ID.to_owned(),
            selection_id: "discovery:made_up_discovery".to_owned(),
        });
        let computation = compute_pilot_base_chassis(&level10);
        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id.starts_with("class_feature.apg.alchemist.discovery.generic.")),
            "an invented selection id must never ground a real magnitude: {:?}",
            computation.explanations
        );
    }

    /// SD-32 T12 Epic 8 row 18 cycle 20 correction (`§17a`): this test originally pinned the
    /// pre-cycle-20 single-value resolver's "more than one terminal -> ground nothing"
    /// refusal. `Discovery ~ True Mutagen`'s own `MutagenTierLVL` and `MutagenACBonus` are not
    /// ambiguous candidates for one magnitude -- they are two genuinely independent,
    /// unconditional corpus quantities (neither's formula references the other). Cycle 20's
    /// `resolve_pool_member_all_magnitudes` now correctly resolves BOTH, each at its own real,
    /// independently-derived value, restating the same never-guess safety property for the real
    /// capability rather than a blanket refusal (`decisions.md §1a`/`§27b`).
    #[test]
    fn true_mutagen_discovery_resolves_both_independent_terminals_not_a_guess() {
        let mut level10 = bare(ALCHEMIST_CLASS_ID, 10);
        level10.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ALCHEMIST_DISCOVERY_CHOICE_ID.to_owned(),
            selection_id: "discovery:true_mutagen".to_owned(),
        });
        let computation = compute_pilot_base_chassis(&level10);
        let mut true_mutagen_values: Vec<(String, i16)> = computation
            .explanations
            .iter()
            .filter(|e| e.id.starts_with("class_feature.apg.alchemist.discovery.generic.true_mutagen."))
            .map(|e| (e.id.clone(), e.value))
            .collect();
        true_mutagen_values.sort();
        assert_eq!(
            true_mutagen_values,
            vec![
                (
                    "class_feature.apg.alchemist.discovery.generic.true_mutagen.mutagenacbonus"
                        .to_string(),
                    2
                ),
                (
                    "class_feature.apg.alchemist.discovery.generic.true_mutagen.mutagentierlvl"
                        .to_string(),
                    1
                ),
            ],
            "both of True Mutagen's independent terminals must resolve at their real, \
             independently-derived values (MutagenACBonus=2, MutagenTierLVL=1 at level 10), \
             neither guessed nor dropped: {:?}",
            computation.explanations
        );
    }

    /// A record whose real corpus `BONUS:VAR` rows are PRE-gated
    /// (`Discovery ~ Force Bomb`:
    /// `VAR|ForceBombDieSize|3|PREVAREQ:...,1` vs
    /// `VAR|ForceBombDieSize|4|PREVAREQ:...,0`, two rows for the SAME
    /// target) refuses rather than silently keeping whichever row a naive
    /// last-write-wins parse would pick -- proves
    /// `class_feature_record_tokens_pre_gate_safe`'s own safety gate is
    /// load-bearing at this generic consumer's real call site, not merely
    /// present in the library function.
    #[test]
    fn force_bomb_discovery_refuses_a_pre_gated_multi_row_target() {
        let mut level10 = bare(ALCHEMIST_CLASS_ID, 10);
        level10.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ALCHEMIST_DISCOVERY_CHOICE_ID.to_owned(),
            selection_id: "discovery:force_bomb".to_owned(),
        });
        let computation = compute_pilot_base_chassis(&level10);
        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id.starts_with("class_feature.apg.alchemist.discovery.generic.force_bomb.")),
            "a PRE-gated multi-row target must refuse, not silently keep the last row: {:?}",
            computation.explanations
        );
    }

    /// Reachability proof (per the dispatch's own "reachability is proven
    /// through a headless pilot receipt" precedent): the real headless
    /// receipt entry point reaches `Computed` for a Human Alchemist with a
    /// genuine, generically-resolved Discovery selection, and the real
    /// receipt's own explanations carry the grounded record.
    #[test]
    fn the_headless_pilot_receipt_carries_the_generic_discovery_magnitude() {
        let mut level4 = seeded(ALCHEMIST_CLASS_ID, 4);
        level4.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ALCHEMIST_DISCOVERY_CHOICE_ID.to_owned(),
            selection_id: "discovery:spontaneous_healing".to_owned(),
        });
        let receipt = build_pilot_headless_receipt(&level4);
        assert!(
            receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id
                    == "class_feature.apg.alchemist.discovery.generic.spontaneous_healing.\
                        spontaneoushealingamount"
                    && e.value == 10),
            "the generic Discovery magnitude must reach the real headless receipt's own \
             explanations: {:?}",
            receipt.computation.explanations
        );
    }

    /// Bloodrager's canonical Bloodline grounds Disruptive Bloodrage's
    /// real +2 defensive-casting DC increase from its own level-1 gate,
    /// and the higher rungs only at their own corpus gates.
    #[test]
    fn bloodrager_arcane_bloodline_grounds_its_real_power_ladder_gates() {
        assert_eq!(
            explanation_value(
                &seeded(BLOODRAGER_CLASS_ID, 1),
                "class_feature.acg.bloodrager.bloodline.arcane.disruptive_bloodrage_dc_increase"
            ),
            Some(2),
            "Disruptive Bloodrage is a level-1 bloodline power: +2 defensive-casting DC"
        );
        assert_eq!(
            explanation_value(
                &seeded(BLOODRAGER_CLASS_ID, 11),
                "class_feature.acg.bloodrager.bloodline.arcane.casters_scourge_extra_attacks"
            ),
            None,
            "Caster's Scourge is gated at bloodline progression level 12"
        );
        assert_eq!(
            explanation_value(
                &seeded(BLOODRAGER_CLASS_ID, 12),
                "class_feature.acg.bloodrager.bloodline.arcane.casters_scourge_extra_attacks"
            ),
            Some(2),
            "max(1, DEX modifier): fixture Dexterity 14 => +2"
        );
    }

    /// An unrecognized bloodline selection must NOT clear the blocker --
    /// only the one canonically grounded bloodline does.
    #[test]
    fn an_unrecognized_bloodrager_bloodline_keeps_the_blocker() {
        let mut input = bare(BLOODRAGER_CLASS_ID, 5);
        input
            .chosen
            .selected_choices
            .push(choice(BLOODRAGER_BLOODLINE_CHOICE_ID, "bloodline:draconic"));
        assert_eq!(
            build_pilot_headless_receipt(&input).status,
            HeadlessReceiptStatus::Blocked,
            "only the canonically grounded Arcane bloodline clears the Bloodline blocker"
        );
    }
}

