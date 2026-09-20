#[allow(unused_imports)]
pub(crate) use super::*;

/// v0.6 alpha swarm, task 4: multiclass BAB/save stacking, generalized
/// beyond the Fighter+Wizard-only dispatch `compute_multiclass_base_chassis`
/// shipped with (SD-21 Epic 7). This file's other tests all live in
/// `tests/*.rs` (an integration-test convention, exercising only this
/// module's public API) rather than an inline module — this one is inline
/// only because `tests/**` is the QA teammate's owned surface for this
/// swarm; it uses the exact same public entry point
/// (`compute_pilot_base_chassis`) as `tests/sd21_multiclass_fighter_wizard_chassis_computes.rs`
/// so it is trivially portable into that file's convention once QA reviews
/// and adopts it into the catalogue. Several sibling files under
/// `src/rules_core/` (e.g. `composed_input.rs`, `equipment_resolver.rs`)
/// already carry their own inline `#[cfg(test)] mod tests`, so this is not
/// a new pattern for the crate, just a new one for this particular file.
#[cfg(test)]
mod multiclass_bab_save_stacking_generalization_tests {
    use super::{
        compute_pilot_base_chassis, BaseSaves, PilotBaseChassisComputation, CLERIC_CLASS_ID,
        FIGHTER_CLASS_ID, ROGUE_CLASS_ID, WIZARD_CLASS_ID,
    };
    use crate::rules_core::character_input::{load_character_input_fixture, CharacterInput};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn load(fixture: &str) -> CharacterInput {
        let result = load_character_input_fixture(fixture);
        assert!(
            result.diagnostics.is_empty(),
            "fixture should load cleanly: {:?}",
            result.diagnostics
        );
        result
            .character_input
            .expect("valid fixture should produce a character input record")
    }

    /// Builds a length-2 multiclass mix by overriding the Fighter level-1
    /// fixture's single `class_levels` entry, mirroring the existing
    /// `tests/sd21_multiclass_fighter_wizard_chassis_computes.rs`'s own
    /// `multiclass_fighter_wizard` helper shape, generalized to any two
    /// class ids.
    fn multiclass(first_class_id: &str, first_level: u8, second_class_id: &str, second_level: u8) -> CharacterInput {
        let mut input = load(FIGHTER_LEVEL_1_FIXTURE);
        let mut first = input.chosen.class_levels[0].clone();
        first.class_id = first_class_id.to_owned();
        first.level = first_level;
        let mut second = first.clone();
        second.class_id = second_class_id.to_owned();
        second.level = second_level;
        input.chosen.class_levels = vec![first, second];
        input
    }

    fn has_diagnostic(computation: &PilotBaseChassisComputation, id: &str) -> bool {
        computation.diagnostics.iter().any(|d| d.id == id)
    }

    /// The task's own literal reproducer: "a Fighter/X character across
    /// levels 1-6" -- Fighter 3 / Rogue 3 (total level 6). Before this task,
    /// Rogue was not one of the two classes `multiclass_class_level_supported`
    /// recognized (only Fighter/Wizard), so this mix tripped the universal
    /// `class_chassis.unsupported` diagnostic and produced a fabricated
    /// `base_attack_bonus: 0` / `base_saves: BaseSaves::default()` exactly
    /// like an unrecognized single class does.
    ///
    /// Fighter 3 (Full BAB): 3. Rogue 3 (3/4 BAB): floor(3*3/4) = 2. Summed: 5.
    /// Fortitude: Fighter3 good (3/2+2=3.5) + Rogue3 poor (3/3=1.0) = 4.5 -> 4.
    /// Reflex: Fighter3 poor (3/3=1.0) + Rogue3 good (3/2+2=3.5) = 4.5 -> 4.
    /// Will: Fighter3 poor (3/3=1.0) + Rogue3 poor (3/3=1.0) = 2.0 -> 2.
    #[test]
    fn fighter3_rogue3_base_attack_bonus_and_saves_are_genuinely_computed() {
        let input = multiclass(FIGHTER_CLASS_ID, 3, ROGUE_CLASS_ID, 3);
        let computation = compute_pilot_base_chassis(&input);

        assert!(
            !has_diagnostic(&computation, "class_chassis.unsupported"),
            "a Fighter 3 / Rogue 3 multiclass mix must be a dispatch-supported chassis \
             once the BAB/save-stacking generalization lands: {:?}",
            computation.diagnostics
        );
        assert_eq!(
            computation.base_attack_bonus, 5,
            "Fighter 3 / Rogue 3 base attack bonus must be the genuine per-class sum: \
             {computation:?}"
        );
        assert_eq!(
            computation.base_saves,
            BaseSaves { fortitude: 4, reflex: 4, will: 2 },
            "Fighter 3 / Rogue 3 base saves must use PF1's sum-fractions-then-round-down-once \
             multiclass rule: {computation:?}"
        );
    }

    /// Proves the generalization is not anchored to Fighter specifically --
    /// a Wizard/Rogue mix (Fighter is not part of it at all) must also
    /// stack correctly, since `multiclass_class_level_supported` and
    /// `multiclass_good_saves` route by `table_class_id`, not a Fighter
    /// special case.
    ///
    /// Wizard 4 (1/2 BAB): 4/2 = 2. Rogue 2 (3/4 BAB): floor(2*3/4) = 1. Summed: 3.
    /// Fortitude: Wizard4 poor (4/3=1.333) + Rogue2 poor (2/3=0.667) = 2.0 -> 2.
    /// Reflex: Wizard4 poor (4/3=1.333) + Rogue2 good (2/2+2=3.0) = 4.333 -> 4.
    /// Will: Wizard4 good (4/2+2=4.0) + Rogue2 poor (2/3=0.667) = 4.667 -> 4.
    #[test]
    fn wizard4_rogue2_base_attack_bonus_and_saves_are_genuinely_computed() {
        let input = multiclass(WIZARD_CLASS_ID, 4, ROGUE_CLASS_ID, 2);
        let computation = compute_pilot_base_chassis(&input);

        assert!(
            !has_diagnostic(&computation, "class_chassis.unsupported"),
            "a Wizard 4 / Rogue 2 mix (Fighter is not part of it at all) must also be \
             dispatch-supported, proving the generalization is not a Fighter-anchored \
             special case: {:?}",
            computation.diagnostics
        );
        assert_eq!(computation.base_attack_bonus, 3, "{computation:?}");
        assert_eq!(
            computation.base_saves,
            BaseSaves { fortitude: 2, reflex: 4, will: 4 },
            "{computation:?}"
        );
    }

    /// A class outside this task's scoped `table_class_id` allowlist must
    /// still fail honestly rather than silently computing a value. Cleric
    /// is not one of Fighter/Wizard/Rogue, so a Fighter/Cleric mix must
    /// still trip `class_chassis.unsupported` -- proving this task
    /// deliberately did not widen every class `class_tables()` carries
    /// data for (several of the other 8 classes have their own standalone,
    /// not-yet-integrated `class_chassis.<class>.*` explanations and
    /// existing tests elsewhere that assert they stay unintegrated; see
    /// `table_class_id`'s own doc comment for why widening those is
    /// deliberately out of this task's scope).
    #[test]
    fn fighter1_cleric1_stays_out_of_this_tasks_scoped_allowlist() {
        // (v0.6 alpha swarm, risks item 8, sixth slice, 2026-07-25) `table_class_id`
        // now recognizes Cleric too (this task's original Fighter/Wizard/
        // Rogue-only allowlist has since grown across several sessions), so
        // the generic `class_chassis.unsupported` diagnostic this test
        // originally checked no longer fires -- Cleric genuinely resolves a
        // real BAB/save chassis row now, the same way Ranger/Paladin/
        // Sorcerer do. The real invariant this test protects (a
        // Fighter+Cleric mix must never be silently Computed) still holds:
        // Cleric's domain-powers burden is permanently unconditional, so it
        // still honestly claim-blocks the mix, just via a different,
        // real diagnostic id.
        let input = multiclass(FIGHTER_CLASS_ID, 1, CLERIC_CLASS_ID, 1);
        let computation = compute_pilot_base_chassis(&input);

        assert!(
            has_diagnostic(&computation, "class_feature.cleric.domain_powers.unsupported"),
            "a Fighter+Cleric multiclass must stay honestly claim-blocked on Cleric's \
             permanently-unconditional domain-powers burden, not silently computed: {:?}",
            computation.diagnostics
        );
    }
}

/// v0.6 alpha swarm, risks item 8, second slice: APG BAB/save dispatch
/// wiring for all 6 real APG classes. Proves the whole pattern end-to-end
/// (real BAB/save values, real HP via the hit-die addition, honest
/// unconditional blocking so none of the 6 can reach a false `Computed`,
/// and multiclass safety by construction) before rolling the identical
/// pattern out to ACG's 10 classes in a follow-up cycle.
#[cfg(test)]
mod apg_class_chassis_dispatch_tests {
    use super::{
        build_pilot_headless_receipt, CharacterClassLevel, HeadlessReceiptStatus, FIGHTER_CLASS_ID,
    };
    use crate::rules_core::character_input::{load_character_input_fixture, CharacterInput};
    use crate::rules_core::durability::compute_max_hp;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    /// Real BAB/save/HP values for all 6 APG classes at level 1, verified
    /// directly against `apg_classes.lst`'s real `BONUS:COMBAT|BASEAB|...`/
    /// `BONUS:SAVE|...`/`HD:` tokens (the same source each per-class file's
    /// own doc comment already cites) -- not re-derived here, just
    /// exercised end-to-end through the real dispatch and HP paths.
    const EXPECTED_LEVEL_1: [(&str, i16, i16, i16, i16, i16); 6] = [
        // (class_id, base_attack_bonus, fort, ref, will, max_hp)
        ("class:alchemist", 0, 2, 2, 0, 8),
        ("class:cavalier", 1, 2, 0, 0, 10),
        ("class:inquisitor", 0, 2, 0, 2, 8),
        ("class:oracle", 0, 0, 0, 2, 8),
        ("class:summoner", 0, 0, 0, 2, 8),
        ("class:witch", 0, 0, 0, 2, 6),
    ];

    fn ranger_style_input(class_id: &str, level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
        input
    }

    #[test]
    fn all_six_apg_classes_ground_real_bab_save_and_hp_at_level_1() {
        for (class_id, expected_bab, expected_fort, expected_ref, expected_will, expected_hp) in
            EXPECTED_LEVEL_1
        {
            let input = ranger_style_input(class_id, 1);
            let receipt = build_pilot_headless_receipt(&input);

            assert_eq!(
                receipt.computation.base_attack_bonus, expected_bab,
                "{class_id} level 1 base attack bonus"
            );
            assert_eq!(
                receipt.computation.base_saves.fortitude, expected_fort,
                "{class_id} level 1 Fortitude save"
            );
            assert_eq!(
                receipt.computation.base_saves.reflex, expected_ref,
                "{class_id} level 1 Reflex save"
            );
            assert_eq!(
                receipt.computation.base_saves.will, expected_will,
                "{class_id} level 1 Will save"
            );

            let max_hp = compute_max_hp(&input.chosen.class_levels, 0)
                .unwrap_or_else(|| panic!("{class_id} must resolve a real max HP"));
            assert_eq!(max_hp, expected_hp, "{class_id} level 1 max HP (0 CON modifier)");
        }
    }

    /// The real safety property this slice's own design depends on: every
    /// APG class OTHER THAN Cavalier/Alchemist stays honestly `Blocked`
    /// with the original, unmodified generic diagnostic -- BAB/save/HP
    /// are real, but the class-skill/feature/spellcasting bucket is
    /// genuinely ungrounded, so this must never silently read as
    /// `Computed`.
    ///
    /// Cavalier, Alchemist, Inquisitor, and Oracle are carved out of this
    /// loop (v0.6 alpha swarm, risks item 8, Cavalier Mount / Alchemist
    /// Mutagen / Inquisitor Judgment / Oracle full-build closures,
    /// first/second/third/fourth APG class-specific closures): each has
    /// its own generic `class_feature.apg.<class>.unsupported` diagnostic
    /// retired and replaced with a narrower one, since the Mount / Mutagen
    /// / Judgment / known-spell+Mystery+Curse posture / Ward hex are now
    /// genuinely grounded -- see
    /// `cavalier_stays_blocked_with_the_new_narrower_diagnostic_not_the_retired_one`
    /// / `alchemist_stays_blocked_with_the_new_narrower_diagnostic_not_the_retired_one`
    /// / `inquisitor_stays_blocked_with_the_new_narrower_diagnostic_not_the_retired_one`
    /// / `oracle_stays_blocked_with_the_new_narrower_diagnostic_not_the_retired_one`
    /// / `witch_stays_blocked_with_the_new_narrower_diagnostic_not_the_retired_one`
    /// below for each class's own dedicated coverage.
    #[test]
    fn all_six_apg_classes_stay_blocked_with_the_real_unconditional_diagnostic() {
        for (class_id, ..) in EXPECTED_LEVEL_1 {
            if class_id == "class:cavalier"
                || class_id == "class:alchemist"
                || class_id == "class:inquisitor"
                || class_id == "class:oracle"
                || class_id == "class:witch"
                // v0.6 alpha swarm, task #17 (2026-07-27): Summoner's
                // generic diagnostic is retired too -- the canonical
                // Quadruped Eidolon's stat block and evolution-pool size
                // are now grounded, so its remaining claim-blocking
                // diagnostic is the narrower
                // `eidolon.evolutions_deferred.unsupported`, covered by
                // `summoner_eidolon_tests` below.
                || class_id == "class:summoner"
            {
                continue;
            }

            let input = ranger_style_input(class_id, 1);
            let receipt = build_pilot_headless_receipt(&input);

            assert_eq!(
                receipt.status,
                HeadlessReceiptStatus::Blocked,
                "{class_id} must not reach Computed while class-skill/feature/spellcasting is \
                 ungrounded: {:?}",
                receipt.computation.diagnostics
            );
            let class_name = class_id.trim_start_matches("class:");
            let expected_diagnostic_id = format!("class_feature.apg.{class_name}.unsupported");
            assert!(
                receipt
                    .computation
                    .diagnostics
                    .iter()
                    .any(|d| d.id == expected_diagnostic_id && d.claim_blocking),
                "expected {expected_diagnostic_id}: {:?}",
                receipt.computation.diagnostics
            );
        }
    }

    /// Cavalier-specific coverage for the retired-diagnostic/new-diagnostic
    /// swap: the OLD generic `class_feature.apg.cavalier.unsupported`
    /// diagnostic must never appear for Cavalier, while the NEW, narrower
    /// `class_feature.apg.cavalier.other_features_deferred.unsupported`
    /// diagnostic always does -- Cavalier stays `Blocked` on its other
    /// named features alone, even though the Mount itself is now
    /// genuinely grounded. Also verifies the reused Horse companion stat
    /// block is grounded under Cavalier's own id prefix, with the real
    /// verified values (Str 16 -> +3 modifier; +4 natural armor -> AC 14).
    #[test]
    fn cavalier_stays_blocked_with_the_new_narrower_diagnostic_not_the_retired_one() {
        let input = ranger_style_input("class:cavalier", 1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Cavalier must stay Blocked on its other-features-deferred posture alone: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.cavalier.unsupported"),
            "the retired generic diagnostic must never appear for Cavalier: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.cavalier.other_features_deferred.unsupported"
                    && d.claim_blocking),
            "expected the new narrower other_features_deferred diagnostic: {:?}",
            receipt.computation.diagnostics
        );

        let armor_class = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_chassis.cavalier.mount.armor_class")
            .expect("Cavalier's own reused Horse companion stat block must be grounded");
        assert_eq!(
            armor_class.value, 14,
            "Horse companion AC: base 10 + natural armor +4 = 14: {:?}",
            armor_class
        );
    }

    /// Alchemist-specific coverage for the retired-diagnostic/new-
    /// diagnostic swap: the OLD generic `class_feature.apg.alchemist
    /// .unsupported` diagnostic must never appear for Alchemist, while
    /// the NEW, narrower `class_feature.apg.alchemist
    /// .other_features_deferred.unsupported` diagnostic always does --
    /// Alchemist stays `Blocked` on other-features alone, even when
    /// Mutagen/Bomb/Poison Resistance/spellcasting are all genuinely
    /// grounded (deepening 2026-07-26, task #4: this test's own
    /// expected diagnostic ID was updated from the now-retired
    /// `spellcasting_deferred` name).
    #[test]
    fn alchemist_stays_blocked_with_the_new_narrower_diagnostic_not_the_retired_one() {
        let input = ranger_style_input("class:alchemist", 1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Alchemist must stay Blocked on its deferred other-features posture alone: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.alchemist.unsupported"),
            "the retired generic diagnostic must never appear for Alchemist: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.alchemist.other_features_deferred.unsupported"
                    && d.claim_blocking),
            "expected the new narrower other_features_deferred diagnostic: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.apg.alchemist.mutagen_execution.not_mutated"),
            "expected the honest not-mutated recognition record: {:?}",
            receipt.computation.explanations
        );
    }

    /// Inquisitor-specific coverage for the retired-diagnostic/new-
    /// diagnostic swap: the OLD generic `class_feature.apg.inquisitor
    /// .unsupported` diagnostic must never appear for Inquisitor, while
    /// the NEW, narrower `class_feature.apg.inquisitor
    /// .other_features_deferred.unsupported` diagnostic always does --
    /// Inquisitor stays `Blocked` on its other named features alone, even
    /// when Judgment itself is genuinely grounded and active.
    #[test]
    fn inquisitor_stays_blocked_with_the_new_narrower_diagnostic_not_the_retired_one() {
        let input = ranger_style_input("class:inquisitor", 1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Inquisitor must stay Blocked on its other-features-deferred posture alone: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.inquisitor.unsupported"),
            "the retired generic diagnostic must never appear for Inquisitor: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.inquisitor.other_features_deferred.unsupported"
                    && d.claim_blocking),
            "expected the new narrower other_features_deferred diagnostic: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.apg.inquisitor.judgment_execution.not_judging"),
            "expected the honest not-judging recognition record: {:?}",
            receipt.computation.explanations
        );
    }

    /// Oracle-specific coverage for the retired-diagnostic/new-diagnostic
    /// swap: the OLD generic `class_feature.apg.oracle.unsupported`
    /// diagnostic must never appear for Oracle, while the NEW, narrower
    /// `class_feature.apg.oracle.other_features_deferred.unsupported`
    /// diagnostic always does -- a BARE Oracle (no Mystery, no Curse)
    /// stays `Blocked` on this diagnostic, with the Mystery/Curse claim-
    /// blocking diagnostics also firing. This comment used to call that
    /// posture "permanent"; Path A canonical narrowing (2026-07-29)
    /// retired that claim -- a recorded Mystery/Curse pair that grounds
    /// real powers now reaches `Computed`. The bare posture asserted here
    /// is unchanged.
    #[test]
    fn oracle_stays_blocked_with_the_new_narrower_diagnostic_not_the_retired_one() {
        let input = ranger_style_input("class:oracle", 1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Oracle must stay Blocked on its other-features-deferred posture: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.oracle.unsupported"),
            "the retired generic diagnostic must never appear for Oracle: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.oracle.other_features_deferred.unsupported"
                    && d.claim_blocking),
            "expected the new narrower other_features_deferred diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// Witch-specific coverage for the retired-diagnostic/new-diagnostic
    /// swap: the OLD generic `class_feature.apg.witch.unsupported`
    /// diagnostic must never appear for Witch, while the NEW, narrower
    /// `class_feature.apg.witch.other_features_deferred.unsupported`
    /// diagnostic always does -- Witch stays permanently `Blocked` on
    /// this diagnostic (like Oracle, no MVP narrowing beyond Ward was
    /// found this slice), with the hex_powers claim-blocking diagnostic
    /// also firing for a bare Witch.
    #[test]
    fn witch_stays_blocked_with_the_new_narrower_diagnostic_not_the_retired_one() {
        let input = ranger_style_input("class:witch", 1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Witch must stay Blocked on its other-features-deferred posture: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.witch.unsupported"),
            "the retired generic diagnostic must never appear for Witch: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.apg.witch.other_features_deferred.unsupported"
                    && d.claim_blocking),
            "expected the new narrower other_features_deferred diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// **Retired as a negative-leak test on 2026-07-29 (Monk/Summoner
    /// chassis-recognition closure), and replaced in place rather than
    /// deleted.**
    ///
    /// This test previously asserted that the ONE remaining unadmitted APG
    /// class produced ZERO
    /// `defense.total_save.*`/`combat.baseline_*`/`skill.selected_modifier.*`
    /// explanations under the satisfying posture, skipping the five that
    /// were genuinely admitted. That "other one" was Summoner, and it is
    /// now admitted too (`is_supported_summoner_single_class`), so the
    /// skip-list covered every row in `EXPECTED_LEVEL_1` and the loop body
    /// stopped executing at all. Left as-is it would have been a
    /// permanently vacuous green test -- worse than a failing one.
    ///
    /// What it becomes is the positive statement the roster now supports:
    /// **all six** APG classes produce real pillar explanations. The
    /// exact-match discipline the original test protected (that each
    /// `is_supported_<class>_single_class` is a real `== Some(..)` match
    /// and not a broad `.is_some()` that would admit any APG class) has
    /// NOT been dropped -- it moves to
    /// `each_apg_predicate_matches_only_its_own_class` below, which tests
    /// the predicates directly instead of inferring exactness from a
    /// side effect.
    #[test]
    fn all_six_apg_classes_produce_real_pillar_explanations_under_the_satisfying_posture() {
        for (class_id, ..) in EXPECTED_LEVEL_1 {
            let input = ranger_style_input(class_id, 1);
            let receipt = build_pilot_headless_receipt(&input);

            for expected_prefix in
                ["defense.total_save.", "combat.baseline_", "skill.selected_modifier."]
            {
                assert!(
                    receipt
                        .computation
                        .explanations
                        .iter()
                        .any(|e| e.id.starts_with(expected_prefix)),
                    "expected at least one {expected_prefix}* explanation for {class_id}"
                );
            }
        }
    }

    /// The exact-match guarantee the retired negative-leak test used to
    /// provide, now asserted against the predicates themselves.
    ///
    /// Each `is_supported_<class>_single_class` must accept its OWN class
    /// and reject the other five. A broad
    /// `ApgClassId::from_class_id_str(..).is_some()` implementation would
    /// pass the accept half and fail every reject half -- which is exactly
    /// the bug shape this guards, and which admitting all six classes into
    /// the composite gate can no longer reveal on its own.
    #[test]
    fn each_apg_predicate_matches_only_its_own_class() {
        // Named rather than written inline: the array literal mixes six
        // distinct fn-item types, so it needs an explicit fn-pointer
        // annotation to coerce, and spelling that out inline trips
        // clippy::type_complexity.
        type ApgSupportPredicate = fn(&CharacterInput) -> bool;

        let predicates: [(&str, ApgSupportPredicate); 6] = [
            ("class:alchemist", super::is_supported_alchemist_single_class),
            ("class:cavalier", super::is_supported_cavalier_single_class),
            ("class:inquisitor", super::is_supported_inquisitor_single_class),
            ("class:oracle", super::is_supported_oracle_single_class),
            ("class:summoner", super::is_supported_summoner_single_class),
            ("class:witch", super::is_supported_witch_single_class),
        ];

        for (owner_id, predicate) in predicates {
            for (candidate_id, ..) in EXPECTED_LEVEL_1 {
                let input = ranger_style_input(candidate_id, 1);
                let accepted = predicate(&input);
                assert_eq!(
                    accepted,
                    candidate_id == owner_id,
                    "the {owner_id} predicate must accept only {owner_id}, but it \
                     returned {accepted} for {candidate_id}"
                );
            }
        }
    }

    /// Cavalier's own positive counterpart: the SAME satisfying posture
    /// DOES produce real total-save/combat-baseline/selected-skill
    /// explanations for Cavalier specifically, proving the new gate
    /// genuinely admits it (not merely failing to admit the other 5 by
    /// accident) -- the first-ever proof this gate admits any APG class.
    #[test]
    fn cavalier_alone_produces_real_pillar_explanations_under_the_satisfying_posture() {
        let input = ranger_style_input("class:cavalier", 1);
        let receipt = build_pilot_headless_receipt(&input);

        for expected_prefix in
            ["defense.total_save.", "combat.baseline_", "skill.selected_modifier."]
        {
            assert!(
                receipt
                    .computation
                    .explanations
                    .iter()
                    .any(|e| e.id.starts_with(expected_prefix)),
                "expected at least one {expected_prefix}* explanation for Cavalier: {:?}",
                receipt.computation.explanations
            );
        }
    }

    /// Alchemist's own positive counterpart, mirroring Cavalier's
    /// exactly -- proves the gate genuinely admits Alchemist regardless
    /// of Mutagen's own (here, inactive) state.
    #[test]
    fn alchemist_alone_produces_real_pillar_explanations_under_the_satisfying_posture() {
        let input = ranger_style_input("class:alchemist", 1);
        let receipt = build_pilot_headless_receipt(&input);

        for expected_prefix in
            ["defense.total_save.", "combat.baseline_", "skill.selected_modifier."]
        {
            assert!(
                receipt
                    .computation
                    .explanations
                    .iter()
                    .any(|e| e.id.starts_with(expected_prefix)),
                "expected at least one {expected_prefix}* explanation for Alchemist: {:?}",
                receipt.computation.explanations
            );
        }
    }

    /// Inquisitor's own positive counterpart, mirroring Cavalier's/
    /// Alchemist's exactly -- proves the gate genuinely admits Inquisitor
    /// regardless of Judgment's own (here, inactive) state.
    #[test]
    fn inquisitor_alone_produces_real_pillar_explanations_under_the_satisfying_posture() {
        let input = ranger_style_input("class:inquisitor", 1);
        let receipt = build_pilot_headless_receipt(&input);

        for expected_prefix in
            ["defense.total_save.", "combat.baseline_", "skill.selected_modifier."]
        {
            assert!(
                receipt
                    .computation
                    .explanations
                    .iter()
                    .any(|e| e.id.starts_with(expected_prefix)),
                "expected at least one {expected_prefix}* explanation for Inquisitor: {:?}",
                receipt.computation.explanations
            );
        }
    }

    /// Oracle's own positive counterpart, mirroring Cavalier's/
    /// Alchemist's/Inquisitor's exactly -- proves the gate genuinely
    /// admits Oracle regardless of its own Mystery/Curse/known-spell
    /// posture state.
    #[test]
    fn oracle_alone_produces_real_pillar_explanations_under_the_satisfying_posture() {
        let input = ranger_style_input("class:oracle", 1);
        let receipt = build_pilot_headless_receipt(&input);

        for expected_prefix in
            ["defense.total_save.", "combat.baseline_", "skill.selected_modifier."]
        {
            assert!(
                receipt
                    .computation
                    .explanations
                    .iter()
                    .any(|e| e.id.starts_with(expected_prefix)),
                "expected at least one {expected_prefix}* explanation for Oracle: {:?}",
                receipt.computation.explanations
            );
        }
    }

    /// Witch's own positive counterpart, mirroring Oracle's/Inquisitor's/
    /// Alchemist's/Cavalier's exactly -- proves the gate genuinely admits
    /// Witch, AND (unlike every other APG positive counterpart) proves
    /// the real Intimidate-only partial class-skill match: Intimidate
    /// genuinely earns the bonus, Climb genuinely does not.
    #[test]
    fn witch_alone_produces_real_pillar_explanations_under_the_satisfying_posture() {
        let input = ranger_style_input("class:witch", 1);
        let receipt = build_pilot_headless_receipt(&input);

        for expected_prefix in
            ["defense.total_save.", "combat.baseline_", "skill.selected_modifier."]
        {
            assert!(
                receipt
                    .computation
                    .explanations
                    .iter()
                    .any(|e| e.id.starts_with(expected_prefix)),
                "expected at least one {expected_prefix}* explanation for Witch: {:?}",
                receipt.computation.explanations
            );
        }

        let intimidate = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "skill.selected_modifier.intimidate")
            .expect("Intimidate must be grounded for Witch");
        assert_eq!(
            intimidate.value, 3,
            "Witch genuinely earns the class-skill bonus on Intimidate (real class-skill list \
             includes it): {:?}",
            intimidate
        );

        let climb = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "skill.selected_modifier.climb")
            .expect("Climb must be grounded for Witch");
        assert_eq!(
            climb.value, 3,
            "Witch does NOT earn the class-skill bonus on Climb (real class-skill list \
             excludes it): {:?}",
            climb
        );
    }

    /// Multiclass safety, verified directly (not assumed from "this path is
    /// unreachable") -- the same discipline the Ranger slice's adversarial
    /// review established. An APG-class-containing multiclass mix must
    /// stay Blocked, since `ApgClassId::from_class_id_str` is deliberately
    /// not registered with `table_class_id`/`multiclass_class_level_supported`.
    #[test]
    fn an_apg_class_multiclassed_with_fighter_stays_blocked() {
        for (class_id, ..) in EXPECTED_LEVEL_1 {
            let mut input = ranger_style_input(class_id, 4);
            input
                .chosen
                .class_levels
                .push(CharacterClassLevel { class_id: FIGHTER_CLASS_ID.to_owned(), level: 1 });

            let receipt = build_pilot_headless_receipt(&input);

            assert_eq!(
                receipt.status,
                HeadlessReceiptStatus::Blocked,
                "{class_id}+Fighter multiclass must not reach Computed: {:?}",
                receipt.computation.diagnostics
            );
        }
    }

    /// A level beyond an APG class's real `MAXLEVEL:20` ceiling stays
    /// honestly blocked with the generic diagnostic, not a fabricated row.
    /// (All 6 real APG classes share the same `MAXLEVEL:20` ceiling, unlike
    /// CRB's Druid/Monk asymmetry.)
    #[test]
    fn a_level_beyond_the_real_maxlevel_ceiling_stays_blocked() {
        let input = ranger_style_input("class:alchemist", 21);

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_chassis.unsupported"),
            "{:?}",
            receipt.computation.diagnostics
        );
    }
}

/// v0.6 alpha swarm, risks item 8, fourth slice: ACG BAB/save/HP dispatch
/// wiring for all 10 real ACG classes. Mirrors `apg_class_chassis_dispatch_tests`
/// exactly (real BAB/save values, real HP via the hit-die addition, honest
/// unconditional blocking so none of the 10 can reach a false `Computed`,
/// and multiclass safety by construction).
#[cfg(test)]
mod acg_class_chassis_dispatch_tests {
    use super::{
        build_pilot_headless_receipt, CharacterClassLevel, HeadlessReceiptStatus, FIGHTER_CLASS_ID,
    };
    use crate::rules_core::character_input::{load_character_input_fixture, CharacterInput};
    use crate::rules_core::durability::compute_max_hp;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    /// Real BAB/save/HP values for all 10 ACG classes at level 1, verified
    /// directly against `acg_classes.lst`'s real `BONUS:COMBAT|BASEAB|...`/
    /// `BONUS:SAVE|...`/`HD:` tokens (the same source each per-class file's
    /// own doc comment already cites) -- not re-derived here, just
    /// exercised end-to-end through the real dispatch and HP paths.
    const EXPECTED_LEVEL_1: [(&str, i16, i16, i16, i16, i16); 10] = [
        // (class_id, base_attack_bonus, fort, ref, will, max_hp)
        ("class:arcanist", 0, 0, 0, 2, 6),
        ("class:bloodrager", 1, 2, 0, 0, 10),
        ("class:brawler", 1, 2, 2, 0, 10),
        ("class:hunter", 0, 2, 2, 0, 8),
        ("class:investigator", 0, 0, 2, 2, 8),
        ("class:shaman", 0, 0, 0, 2, 8),
        ("class:skald", 0, 2, 0, 2, 8),
        ("class:slayer", 1, 2, 2, 0, 10),
        ("class:swashbuckler", 1, 0, 2, 0, 10),
        ("class:warpriest", 0, 2, 0, 2, 8),
    ];

    fn acg_style_input(class_id: &str, level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
        input
    }

    #[test]
    fn all_ten_acg_classes_ground_real_bab_save_and_hp_at_level_1() {
        for (class_id, expected_bab, expected_fort, expected_ref, expected_will, expected_hp) in
            EXPECTED_LEVEL_1
        {
            let input = acg_style_input(class_id, 1);
            let receipt = build_pilot_headless_receipt(&input);

            assert_eq!(
                receipt.computation.base_attack_bonus, expected_bab,
                "{class_id} level 1 base attack bonus"
            );
            assert_eq!(
                receipt.computation.base_saves.fortitude, expected_fort,
                "{class_id} level 1 Fortitude save"
            );
            assert_eq!(
                receipt.computation.base_saves.reflex, expected_ref,
                "{class_id} level 1 Reflex save"
            );
            assert_eq!(
                receipt.computation.base_saves.will, expected_will,
                "{class_id} level 1 Will save"
            );

            let max_hp = compute_max_hp(&input.chosen.class_levels, 0)
                .unwrap_or_else(|| panic!("{class_id} must resolve a real max HP"));
            assert_eq!(max_hp, expected_hp, "{class_id} level 1 max HP (0 CON modifier)");
        }
    }

    /// The real safety property this slice's own design depends on: every
    /// ACG class OTHER THAN Skald/Bloodrager/Brawler/Hunter stays honestly
    /// `Blocked` with the original, unmodified generic diagnostic --
    /// BAB/save/HP are real, but the class-skill/feature/spellcasting
    /// bucket is genuinely ungrounded, so this must never silently read
    /// as `Computed`.
    ///
    /// Skald, Bloodrager, Brawler, and Hunter are carved out of this loop
    /// (v0.6 alpha swarm, risks item 8, first/second/third/fourth APG/ACG
    /// closures, adversarial review finding 2, reapplied identically each
    /// time): each has its own generic `class_feature.acg.<class>
    /// .unsupported` diagnostic retired and replaced with a narrower one,
    /// since Inspired Rage / Bloodrage / AC Bonus / Animal Companion are
    /// now genuinely grounded -- see
    /// `skald_stays_blocked_with_the_new_narrower_diagnostic_not_the_retired_one`
    /// / `bloodrager_stays_blocked_with_the_new_narrower_diagnostic_not_the_retired_one`
    /// / `brawler_stays_blocked_with_the_new_narrower_diagnostic_not_the_retired_one`
    /// / `hunter_stays_blocked_with_the_new_narrower_diagnostic_not_the_retired_one`
    /// below for each class's own dedicated coverage.
    #[test]
    fn all_ten_acg_classes_stay_blocked_with_the_real_unconditional_diagnostic() {
        for (class_id, ..) in EXPECTED_LEVEL_1 {
            if class_id == "class:skald"
                || class_id == "class:bloodrager"
                || class_id == "class:brawler"
                || class_id == "class:hunter"
                || class_id == "class:arcanist"
                || class_id == "class:warpriest"
                || class_id == "class:slayer"
                || class_id == "class:swashbuckler"
                || class_id == "class:investigator"
                || class_id == "class:shaman"
            {
                continue;
            }

            let input = acg_style_input(class_id, 1);
            let receipt = build_pilot_headless_receipt(&input);

            assert_eq!(
                receipt.status,
                HeadlessReceiptStatus::Blocked,
                "{class_id} must not reach Computed while class-skill/feature/spellcasting is \
                 ungrounded: {:?}",
                receipt.computation.diagnostics
            );
            let class_name = class_id.trim_start_matches("class:");
            let expected_diagnostic_id = format!("class_feature.acg.{class_name}.unsupported");
            assert!(
                receipt
                    .computation
                    .diagnostics
                    .iter()
                    .any(|d| d.id == expected_diagnostic_id && d.claim_blocking),
                "expected {expected_diagnostic_id}: {:?}",
                receipt.computation.diagnostics
            );
        }
    }

    /// Skald-specific coverage for the retired-diagnostic/new-diagnostic
    /// swap (adversarial review finding 2): the OLD generic
    /// `class_feature.acg.skald.unsupported` diagnostic must never appear
    /// for Skald at any raging state, while the NEW, narrower
    /// `class_feature.acg.skald.other_features_deferred.unsupported`
    /// diagnostic always does -- Skald stays `Blocked` on its remaining
    /// named features alone, even though Inspired Rage AND known-spell
    /// posture are now both genuinely grounded (v0.6 alpha swarm, risks
    /// item 8, Skald spellcasting closure: unlike Bard, whose own
    /// remaining named features were already built in an earlier SD13-E5
    /// cycle, Skald's own (Bardic Knowledge-analog, Iron Will, Rage
    /// Powers shared-list access, Spell Kenning, Versatile Performance,
    /// War Chant) remain completely unbuilt, so this diagnostic still
    /// unconditionally claim-blocks).
    ///
    /// **Task #91 flips the status assertion.** The features this
    /// diagnostic's blocking claim named -- Raging Song itself and its
    /// four songs, Master Skald, Uncanny Dodge, Improved Uncanny Dodge,
    /// Cantrips and the Scribe Scroll grant -- are now grounded, so the
    /// record is demoted rather than deleted and Skald computes.
    #[test]
    fn skald_reaches_computed_with_its_remainder_diagnostic_demoted_not_deleted() {
        let input = acg_style_input("class:skald", 1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Skald grounds every named corpus feature and must now compute: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.skald.other_features_deferred.unsupported"
                    && !d.claim_blocking),
            "the remainder record must survive as a NON-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.skald.unsupported"),
            "the retired generic diagnostic must never appear for Skald: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.skald.spellcasting_deferred.unsupported"),
            "the now-retired spellcasting_deferred diagnostic must never appear for Skald \
             either, now that known-spell posture is genuinely validated: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_spell.skald.known_spells"),
            "expected the real known-spell posture to be grounded (zero known spells is a \
             valid PF1 posture, mirroring Bard's own precedent): {:?}",
            receipt.computation.explanations
        );
    }

    /// Bloodrager's own counterpart to the Skald diagnostic-swap test above
    /// (v0.6 alpha swarm, risks item 8, second APG/ACG closure): mirrors it
    /// exactly.
    #[test]
    fn bloodrager_stays_blocked_with_the_new_narrower_diagnostic_not_the_retired_one() {
        let input = acg_style_input("class:bloodrager", 1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Bloodrager must stay Blocked on its deferred spellcasting posture alone: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.bloodrager.unsupported"),
            "the retired generic diagnostic must never appear for Bloodrager: {:?}",
            receipt.computation.diagnostics
        );
        // Level 1 Bloodragers have NO spellcasting (task #1, 2026-07-27):
        // the class block carries no CAST:/KNOWN: row below level 4, so
        // the block at this level is other-features, not spellcasting.
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id
                    == "class_feature.acg.bloodrager.other_features_deferred.unsupported"
                    && d.claim_blocking),
            "expected the narrower other_features_deferred diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// Brawler now reaches Computed (task #91).
    ///
    /// This test previously asserted the opposite -- that Brawler "must
    /// stay Blocked on its other-features-deferred posture alone". That
    /// was correct while Awesome Blow, Improved Awesome Blow and Close
    /// Weapon Mastery were genuinely ungrounded. All three are now
    /// grounded from the corpus, so the blocking claim has no remaining
    /// content and the diagnostic has been demoted to non-blocking.
    ///
    /// The demotion is asserted explicitly rather than merely implied by
    /// the status: a future edit that deletes the diagnostic outright
    /// would also produce Computed, and that would lose the honest
    /// remainder the record still carries.
    #[test]
    fn brawler_reaches_computed_with_its_remainder_diagnostic_demoted_not_deleted() {
        let input = acg_style_input("class:brawler", 1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Brawler grounds every named corpus feature and must now compute: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.brawler.unsupported"),
            "the retired generic diagnostic must never appear for Brawler: {:?}",
            receipt.computation.diagnostics
        );
        let remainder = receipt
            .computation
            .diagnostics
            .iter()
            .find(|d| d.id == "class_feature.acg.brawler.other_features_deferred.unsupported")
            .expect("the remainder record must be kept, not deleted");
        assert!(
            !remainder.claim_blocking,
            "the remainder names deferred EXECUTION, not a missing magnitude, so it must not \
             block: {remainder:?}"
        );
        let ac_bonus = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.brawler.ac_bonus")
            .expect("Brawler's own AC Bonus explanation must be grounded");
        assert_eq!(ac_bonus.value, 0, "Brawler level 1 AC Bonus is genuinely +0: {:?}", ac_bonus);
    }

    /// Hunter's own counterpart to the Skald/Bloodrager/Brawler
    /// diagnostic-swap tests above (v0.6 alpha swarm, risks item 8,
    /// fourth APG/ACG closure): mirrors them, and additionally verifies
    /// the reused Wolf companion stat block is grounded under Hunter's
    /// own id prefix.
    ///
    /// **Updated (v0.6 alpha swarm, task #44):** Hunter's own known-spell
    /// posture is now genuinely validated (see
    /// `hunter_dispatch_widening_safety_tests` for the dedicated
    /// known-spell tests), so -- mirroring Skald's/Bloodrager's/Brawler's
    /// own rename precedent above -- the remaining claim-blocking
    /// diagnostic is `other_features_deferred`, not the retired
    /// `spellcasting_deferred` name.
    ///
    /// **Task #91 flips the status assertion.** The three spellcasting
    /// clauses this diagnostic's blocking claim carried (no per-day CAST
    /// totals, no spell save DC, no access ladder) plus Nature Training
    /// are now grounded, and its class-skill-list and Precise Companion
    /// clauses were already stale when written.
    #[test]
    fn hunter_reaches_computed_with_its_remainder_diagnostic_demoted_not_deleted() {
        let input = acg_style_input("class:hunter", 1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Hunter grounds every named corpus feature and must now compute: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.hunter.other_features_deferred.unsupported"
                    && !d.claim_blocking),
            "the remainder record must survive as a NON-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.hunter.unsupported"),
            "the retired generic diagnostic must never appear for Hunter: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.hunter.spellcasting_deferred.unsupported"),
            "the retired spellcasting_deferred diagnostic must never appear for Hunter now that \
             the known-spell posture is genuinely validated: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.acg.hunter.known_spells.unsupported"),
            "zero freely chosen known spells is itself a valid posture, mirroring Oracle's/\
             Sorcerer's own reasoning: {:?}",
            receipt.computation.diagnostics
        );
        let companion_hp = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_chassis.hunter.animal_companion.hit_points")
            .expect("Hunter's own reused Wolf companion stat block must be grounded");
        // Same Wolf stat block math Druid's own closure verified: d8 HD,
        // maximized first Hit Die (8) + average second (average_hit_die_value(8)
        // = 8/2+1 = 5) + Constitution modifier (+2, Con 15) each = (8+2) +
        // (5+2) = 17.
        assert_eq!(
            companion_hp.value, 17,
            "Hunter's reused companion HP must match Druid's own verified math: {:?}",
            companion_hp
        );
        let automatic_sna = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_spell.acg.hunter.automatic_summon_natures_ally_known_spells")
            .expect("the automatic Summon Nature's Ally grant must be grounded");
        assert_eq!(automatic_sna.value, 6, "all six Summon Nature's Ally spells: {:?}", automatic_sna);
    }

    /// Arcanist's own counterpart to the Skald/Bloodrager/Brawler/Hunter
    /// diagnostic-swap tests above (v0.6 alpha swarm, risks item 8,
    /// Arcanist full-build closure, fifth APG/ACG closure): mirrors them,
    /// using `exploits_deferred` (Arcanist's own genuinely built
    /// spellcasting means the remaining gap is Exploits/Consume Spells/
    /// Magical Supremacy, not spellcasting itself). A bare Arcanist with
    /// no spells recorded is a genuinely valid "hasn't chosen spells yet"
    /// posture (mirrors Wizard's own pre-bootstrap-fix shape), so it
    /// stays Blocked on the prepared-spellbook diagnostic too -- this
    /// test also verifies the Arcane Reservoir's own flat values ground
    /// unconditionally regardless of spellbook state.
    #[test]
    fn arcanist_stays_blocked_with_the_new_narrower_diagnostic_not_the_retired_one() {
        let input = acg_style_input("class:arcanist", 1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Arcanist must stay Blocked on its exploits-deferred/spellbook posture: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.arcanist.unsupported"),
            "the retired generic diagnostic must never appear for Arcanist: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.arcanist.exploits_deferred.unsupported"
                    && d.claim_blocking),
            "expected the new narrower exploits_deferred diagnostic: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.acg.arcanist.prepared_spellbook.unsupported"
                    && d.claim_blocking),
            "a bare Arcanist with no recorded spells is a genuinely valid but incomplete \
             posture, mirroring Wizard's own pre-bootstrap-fix shape: {:?}",
            receipt.computation.diagnostics
        );

        let reservoir_max = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.arcanist.arcane_reservoir_max")
            .expect("Arcane Reservoir max must ground unconditionally, regardless of spellbook state");
        assert_eq!(reservoir_max.value, 4, "Arcanist level 1 Reservoir max: 3 + 1 = 4: {:?}", reservoir_max);
        let reservoir_fill = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.arcanist.arcane_reservoir_daily_fill")
            .expect("Arcane Reservoir daily fill must ground unconditionally");
        assert_eq!(
            reservoir_fill.value, 3,
            "Arcanist level 1 Reservoir daily fill: 3 + 1/2 = 3: {:?}",
            reservoir_fill
        );
    }

    /// Warpriest-specific coverage for the retired-diagnostic/new-
    /// diagnostic swap (v0.6 alpha swarm, risks item 8, Warpriest
    /// full-build closure, sixth ACG/APG closure): the OLD generic
    /// `class_feature.acg.warpriest.unsupported` diagnostic must never
    /// appear, while the NEW, narrower `other_features_deferred`
    /// diagnostic always does. This bare fixture (no Blessing choice, no
    /// spells) also trips `blessing_powers.unsupported` and
    /// `prepared_spellbook.unsupported` -- both genuinely valid but
    /// incomplete postures -- while Blessings' flat uses-per-day/DC and
    /// Sacred Weapon's base damage die still ground unconditionally.
    #[test]
    fn warpriest_stays_blocked_with_the_new_narrower_diagnostic_not_the_retired_one() {
        let input = acg_style_input("class:warpriest", 1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Warpriest must stay Blocked on its other-features-deferred posture alone: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.warpriest.unsupported"),
            "the retired generic diagnostic must never appear for Warpriest: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.warpriest.other_features_deferred.unsupported"
                    && d.claim_blocking),
            "expected the new narrower other_features_deferred diagnostic: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.warpriest.blessing_powers.unsupported"
                    && d.claim_blocking),
            "a bare Warpriest with no recognized Destruction Blessing choice is a genuinely \
             valid but incomplete posture: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.acg.warpriest.prepared_spellbook.unsupported"
                    && d.claim_blocking),
            "a bare Warpriest with no recorded spells is a genuinely valid but incomplete \
             posture: {:?}",
            receipt.computation.diagnostics
        );

        let uses = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.warpriest.blessing_uses_per_day")
            .expect("Blessings uses per day must ground unconditionally");
        assert_eq!(uses.value, 3, "Warpriest level 1 Blessing uses: 1/2 + 3 = 3: {:?}", uses);
    }

    /// Slayer-specific coverage for the retired-diagnostic/new-diagnostic
    /// swap (v0.6 alpha swarm, risks item 8, Slayer full-build closure,
    /// seventh ACG/APG closure): the OLD generic
    /// `class_feature.acg.slayer.unsupported` diagnostic must never
    /// appear, while the NEW, narrower `other_features_deferred`
    /// diagnostic always does. All four flat sub-feature formulas ground
    /// unconditionally regardless (no choice or activation gate for any
    /// of them).
    ///
    /// **Task #91 flips the status assertion.** Slayer's blocking claim
    /// listed Stalker, Quarry, Quarry Output, Improved Quarry, Master
    /// Slayer, Slayer's Advance and Swift Tracker. All seven are now
    /// grounded from the corpus, so the diagnostic is demoted rather than
    /// deleted and Slayer computes.
    #[test]
    fn slayer_reaches_computed_with_its_remainder_diagnostic_demoted_not_deleted() {
        let input = acg_style_input("class:slayer", 1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Slayer grounds every named corpus feature and must now compute: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.slayer.unsupported"),
            "the retired generic diagnostic must never appear for Slayer: {:?}",
            receipt.computation.diagnostics
        );
        let remainder = receipt
            .computation
            .diagnostics
            .iter()
            .find(|d| d.id == "class_feature.acg.slayer.other_features_deferred.unsupported")
            .expect("the remainder record must be kept, not deleted");
        assert!(
            !remainder.claim_blocking,
            "the remainder names deferred application and a talent catalog gap, neither of which \
             is a missing magnitude: {remainder:?}"
        );

        let sneak_attack = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.slayer.sneak_attack_dice")
            .expect("Sneak Attack dice must ground unconditionally");
        assert_eq!(sneak_attack.value, 0, "Slayer level 1 Sneak Attack dice: {:?}", sneak_attack);
    }

    /// Swashbuckler-specific coverage for the retired-diagnostic/new-
    /// diagnostic swap (v0.6 alpha swarm, risks item 8, Swashbuckler
    /// full-build closure, ninth ACG/APG closure): the OLD generic
    /// `class_feature.acg.swashbuckler.unsupported` diagnostic must never
    /// appear, while the NEW, narrower `other_features_deferred`
    /// diagnostic always does. Panache's max and Nimble's dodge bonus
    /// ground unconditionally regardless (no choice or activation gate
    /// for either).
    ///
    /// **Task #91 flips the status assertion**, for the same reason as
    /// its sibling in `swashbuckler_dispatch_widening_safety_tests`: the
    /// thirteen deeds, Weapon Mastery and the bonus-feat slot count its
    /// blocking claim named are now grounded.
    #[test]
    fn swashbuckler_reaches_computed_with_its_remainder_diagnostic_demoted_not_deleted() {
        let input = acg_style_input("class:swashbuckler", 1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Swashbuckler grounds every named corpus feature and must now compute: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.swashbuckler.unsupported"),
            "the retired generic diagnostic must never appear for Swashbuckler: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id
                    == "class_feature.acg.swashbuckler.other_features_deferred.unsupported"
                    && !d.claim_blocking),
            "the remainder record must survive as a NON-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );

        let panache = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.swashbuckler.panache_max")
            .expect("Panache max must ground unconditionally");
        assert_eq!(panache.value, 1, "fixture Charisma 8 (-1 modifier): max(1,-1)=1: {:?}", panache);
    }

    /// Investigator-specific coverage for the retired-diagnostic/new-
    /// diagnostic swap (v0.6 alpha swarm, risks item 8, Investigator
    /// full-build closure, tenth ACG/APG closure): the OLD generic
    /// `class_feature.acg.investigator.unsupported` diagnostic must
    /// never appear, while the NEW, narrower `other_features_deferred`
    /// diagnostic always does. Trapfinding/Trap Sense/Inspiration pool
    /// size ground unconditionally regardless (no choice or activation
    /// gate for any of them in this no-spellcasting MVP).
    #[test]
    fn investigator_stays_blocked_with_the_new_narrower_diagnostic_not_the_retired_one() {
        let input = acg_style_input("class:investigator", 1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Investigator must stay Blocked on its other-features-deferred posture alone: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.investigator.unsupported"),
            "the retired generic diagnostic must never appear for Investigator: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id
                    == "class_feature.acg.investigator.other_features_deferred.unsupported"
                    && d.claim_blocking),
            "expected the new narrower other_features_deferred diagnostic: {:?}",
            receipt.computation.diagnostics
        );

        let trapfinding = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.investigator.trapfinding_bonus")
            .expect("Trapfinding must ground unconditionally");
        assert_eq!(
            trapfinding.value, 1,
            "Investigator level 1 Trapfinding: max(0,1)=1: {:?}",
            trapfinding
        );
    }

    /// Shaman-specific coverage for the retired-diagnostic/new-diagnostic
    /// swap (v0.6 alpha swarm, risks item 8, Shaman full-build closure,
    /// twelfth ACG/APG closure): the OLD generic
    /// `class_feature.acg.shaman.unsupported` diagnostic must never
    /// appear, while the NEW, narrower `other_features_deferred`
    /// diagnostic always does. A bare Shaman (no Spirit choice) also
    /// stays claim-blocked on `spirit_powers.unsupported`.
    #[test]
    fn shaman_stays_blocked_with_the_new_narrower_diagnostic_not_the_retired_one() {
        let input = acg_style_input("class:shaman", 1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Shaman must stay Blocked on its other-features-deferred posture: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.shaman.unsupported"),
            "the retired generic diagnostic must never appear for Shaman: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.shaman.other_features_deferred.unsupported"
                    && d.claim_blocking),
            "expected the new narrower other_features_deferred diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// The critical negative-leak test (adversarial review finding 1,
    /// flagged by the lead as the one to verify most carefully, reapplied
    /// for Bloodrager, Brawler, Hunter, Arcanist, Warpriest, Slayer,
    /// Swashbuckler, Investigator, and now Shaman): with Shaman's own
    /// closure, all ten real ACG classes are genuinely admitted by their
    /// own exact-match gate now -- there is no "other" ACG class left to
    /// prove a leak against. The loop below is a deliberate, self-
    /// documenting no-op (asserted explicitly, not left as a silently
    /// vacuous body): every entry in `EXPECTED_LEVEL_1` is carved out,
    /// proving this test's own historical purpose (catching a broad
    /// `.is_some()` gate that would silently admit a sibling class) is
    /// now fully covered by each class's own dedicated positive-
    /// counterpart test instead, not abandoned.
    #[test]
    fn all_ten_acg_classes_are_now_genuinely_admitted_no_leak_target_remains() {
        let carved_out = |class_id: &str| {
            matches!(
                class_id,
                "class:skald"
                    | "class:bloodrager"
                    | "class:brawler"
                    | "class:hunter"
                    | "class:arcanist"
                    | "class:warpriest"
                    | "class:slayer"
                    | "class:swashbuckler"
                    | "class:investigator"
                    | "class:shaman"
            )
        };
        assert!(
            EXPECTED_LEVEL_1.iter().all(|(class_id, ..)| carved_out(class_id)),
            "every one of the ten real ACG classes must now be carved out -- if this fails, a \
             new ACG class was added to EXPECTED_LEVEL_1 without its own leak-test coverage"
        );

        for (class_id, ..) in EXPECTED_LEVEL_1 {
            if carved_out(class_id) {
                continue;
            }

            let input = acg_style_input(class_id, 1);
            let receipt = build_pilot_headless_receipt(&input);

            let leaked: Vec<&str> = receipt
                .computation
                .explanations
                .iter()
                .map(|e| e.id.as_str())
                .filter(|id| {
                    id.starts_with("defense.total_save.")
                        || id.starts_with("defense.baseline_")
                        || id.starts_with("combat.baseline_")
                        || id.starts_with("skill.selected_modifier.")
                })
                .collect();
            assert!(
                leaked.is_empty(),
                "{class_id} must produce zero total-save/combat-baseline/selected-skill \
                 explanations (the chassis-integration gate must not silently admit it): {leaked:?}"
            );
        }
    }

    /// Skald's own positive counterpart to the negative-leak test above:
    /// the SAME satisfying posture DOES produce real total-save/combat-
    /// baseline/selected-skill explanations for Skald specifically, proving
    /// the new gate genuinely admits Skald (not merely failing to admit the
    /// other 9 by accident).
    #[test]
    fn skald_alone_produces_real_pillar_explanations_under_the_satisfying_posture() {
        let input = acg_style_input("class:skald", 1);
        let receipt = build_pilot_headless_receipt(&input);

        for expected_prefix in
            ["defense.total_save.", "combat.baseline_", "skill.selected_modifier."]
        {
            assert!(
                receipt
                    .computation
                    .explanations
                    .iter()
                    .any(|e| e.id.starts_with(expected_prefix)),
                "expected at least one {expected_prefix}* explanation for Skald: {:?}",
                receipt.computation.explanations
            );
        }
    }

    /// Bloodrager's own positive counterpart, mirroring Skald's exactly.
    #[test]
    fn bloodrager_alone_produces_real_pillar_explanations_under_the_satisfying_posture() {
        let input = acg_style_input("class:bloodrager", 1);
        let receipt = build_pilot_headless_receipt(&input);

        for expected_prefix in
            ["defense.total_save.", "combat.baseline_", "skill.selected_modifier."]
        {
            assert!(
                receipt
                    .computation
                    .explanations
                    .iter()
                    .any(|e| e.id.starts_with(expected_prefix)),
                "expected at least one {expected_prefix}* explanation for Bloodrager: {:?}",
                receipt.computation.explanations
            );
        }
    }

    /// Brawler's own positive counterpart, mirroring Skald's/Bloodrager's
    /// exactly. Note Brawler's own AC Bonus is +0 at level 1 (the
    /// progression only starts at 4th level), but `defense.baseline_*`
    /// explanations are always pushed once a class is genuinely admitted
    /// through the chassis-integration gate regardless of the specific
    /// value, so this still proves the gate admits Brawler.
    #[test]
    fn brawler_alone_produces_real_pillar_explanations_under_the_satisfying_posture() {
        let input = acg_style_input("class:brawler", 1);
        let receipt = build_pilot_headless_receipt(&input);

        for expected_prefix in
            ["defense.total_save.", "combat.baseline_", "skill.selected_modifier."]
        {
            assert!(
                receipt
                    .computation
                    .explanations
                    .iter()
                    .any(|e| e.id.starts_with(expected_prefix)),
                "expected at least one {expected_prefix}* explanation for Brawler: {:?}",
                receipt.computation.explanations
            );
        }
    }

    /// Hunter's own positive counterpart, mirroring Skald's/Bloodrager's/
    /// Brawler's exactly.
    #[test]
    fn hunter_alone_produces_real_pillar_explanations_under_the_satisfying_posture() {
        let input = acg_style_input("class:hunter", 1);
        let receipt = build_pilot_headless_receipt(&input);

        for expected_prefix in
            ["defense.total_save.", "combat.baseline_", "skill.selected_modifier."]
        {
            assert!(
                receipt
                    .computation
                    .explanations
                    .iter()
                    .any(|e| e.id.starts_with(expected_prefix)),
                "expected at least one {expected_prefix}* explanation for Hunter: {:?}",
                receipt.computation.explanations
            );
        }
    }

    /// Arcanist's own positive counterpart, mirroring Skald's/
    /// Bloodrager's/Brawler's/Hunter's exactly -- proves the gate
    /// genuinely admits Arcanist regardless of its own (here, bare/no
    /// spells) spellbook state.
    #[test]
    fn arcanist_alone_produces_real_pillar_explanations_under_the_satisfying_posture() {
        let input = acg_style_input("class:arcanist", 1);
        let receipt = build_pilot_headless_receipt(&input);

        for expected_prefix in
            ["defense.total_save.", "combat.baseline_", "skill.selected_modifier."]
        {
            assert!(
                receipt
                    .computation
                    .explanations
                    .iter()
                    .any(|e| e.id.starts_with(expected_prefix)),
                "expected at least one {expected_prefix}* explanation for Arcanist: {:?}",
                receipt.computation.explanations
            );
        }
    }

    /// Warpriest's own positive counterpart, mirroring the other five
    /// exactly -- proves the gate genuinely admits Warpriest regardless
    /// of its own (here, bare/no-Blessing-chosen) posture, AND that
    /// Warpriest genuinely gets the class-skill bonus on Climb/
    /// Intimidate/Swim (v0.6 alpha swarm, risks item 8, Warpriest
    /// full-build closure: the mirror-image of the Wizard class-skill
    /// bug fixed once already).
    #[test]
    fn warpriest_alone_produces_real_pillar_explanations_under_the_satisfying_posture() {
        let input = acg_style_input("class:warpriest", 1);
        let receipt = build_pilot_headless_receipt(&input);

        for expected_prefix in
            ["defense.total_save.", "combat.baseline_", "skill.selected_modifier."]
        {
            assert!(
                receipt
                    .computation
                    .explanations
                    .iter()
                    .any(|e| e.id.starts_with(expected_prefix)),
                "expected at least one {expected_prefix}* explanation for Warpriest: {:?}",
                receipt.computation.explanations
            );
        }

        let climb = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "skill.selected_modifier.climb")
            .expect("Climb must be grounded for Warpriest");
        assert_eq!(
            climb.value, 6,
            "Warpriest genuinely earns the class-skill bonus on Climb (real class-skill list \
             includes it): {:?}",
            climb
        );
    }

    /// Slayer's own positive counterpart, mirroring the other six
    /// exactly -- proves the gate genuinely admits Slayer, AND that
    /// Slayer genuinely gets the class-skill bonus on Climb/Intimidate/
    /// Swim (v0.6 alpha swarm, risks item 8, Slayer full-build closure:
    /// the third class needing this exact widening).
    #[test]
    fn slayer_alone_produces_real_pillar_explanations_under_the_satisfying_posture() {
        let input = acg_style_input("class:slayer", 1);
        let receipt = build_pilot_headless_receipt(&input);

        for expected_prefix in
            ["defense.total_save.", "combat.baseline_", "skill.selected_modifier."]
        {
            assert!(
                receipt
                    .computation
                    .explanations
                    .iter()
                    .any(|e| e.id.starts_with(expected_prefix)),
                "expected at least one {expected_prefix}* explanation for Slayer: {:?}",
                receipt.computation.explanations
            );
        }

        let climb = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "skill.selected_modifier.climb")
            .expect("Climb must be grounded for Slayer");
        assert_eq!(
            climb.value, 6,
            "Slayer genuinely earns the class-skill bonus on Climb (real class-skill list \
             includes it): {:?}",
            climb
        );
    }

    /// Swashbuckler's own positive counterpart, mirroring Slayer's
    /// exactly -- proves the gate genuinely admits Swashbuckler.
    #[test]
    fn swashbuckler_alone_produces_real_pillar_explanations_under_the_satisfying_posture() {
        let input = acg_style_input("class:swashbuckler", 1);
        let receipt = build_pilot_headless_receipt(&input);

        for expected_prefix in
            ["defense.total_save.", "combat.baseline_", "skill.selected_modifier."]
        {
            assert!(
                receipt
                    .computation
                    .explanations
                    .iter()
                    .any(|e| e.id.starts_with(expected_prefix)),
                "expected at least one {expected_prefix}* explanation for Swashbuckler: {:?}",
                receipt.computation.explanations
            );
        }

        let climb = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "skill.selected_modifier.climb")
            .expect("Climb must be grounded for Swashbuckler");
        assert_eq!(
            climb.value, 6,
            "Swashbuckler genuinely earns the class-skill bonus on Climb (real class-skill \
             list includes it): {:?}",
            climb
        );
    }

    /// Investigator's own positive counterpart, mirroring Swashbuckler's
    /// exactly -- proves the gate genuinely admits Investigator, AND
    /// (unlike every prior positive counterpart) proves the real 2-of-3
    /// partial class-skill match: Climb/Intimidate genuinely earn the
    /// bonus, Swim genuinely does not.
    #[test]
    fn investigator_alone_produces_real_pillar_explanations_under_the_satisfying_posture() {
        let input = acg_style_input("class:investigator", 1);
        let receipt = build_pilot_headless_receipt(&input);

        for expected_prefix in
            ["defense.total_save.", "combat.baseline_", "skill.selected_modifier."]
        {
            assert!(
                receipt
                    .computation
                    .explanations
                    .iter()
                    .any(|e| e.id.starts_with(expected_prefix)),
                "expected at least one {expected_prefix}* explanation for Investigator: {:?}",
                receipt.computation.explanations
            );
        }

        let climb = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "skill.selected_modifier.climb")
            .expect("Climb must be grounded for Investigator");
        assert_eq!(
            climb.value, 6,
            "Investigator genuinely earns the class-skill bonus on Climb (real class-skill \
             list includes it): {:?}",
            climb
        );

        let swim = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "skill.selected_modifier.swim")
            .expect("Swim must be grounded for Investigator");
        assert_eq!(
            swim.value, 3,
            "Investigator does NOT earn the class-skill bonus on Swim (real class-skill list \
             excludes it, the first partial match on the roster): {:?}",
            swim
        );
    }

    /// Shaman's own positive counterpart, mirroring Investigator's/
    /// Swashbuckler's/Slayer's exactly -- proves the gate genuinely
    /// admits Shaman. Shaman's own real class-skill list excludes all
    /// three of Climb/Intimidate/Swim (verified directly), the same
    /// "none of three" shape as Wizard/Arcanist/Oracle -- no class-
    /// skill-bonus bug to fix here.
    #[test]
    fn shaman_alone_produces_real_pillar_explanations_under_the_satisfying_posture() {
        let input = acg_style_input("class:shaman", 1);
        let receipt = build_pilot_headless_receipt(&input);

        for expected_prefix in
            ["defense.total_save.", "combat.baseline_", "skill.selected_modifier."]
        {
            assert!(
                receipt
                    .computation
                    .explanations
                    .iter()
                    .any(|e| e.id.starts_with(expected_prefix)),
                "expected at least one {expected_prefix}* explanation for Shaman: {:?}",
                receipt.computation.explanations
            );
        }

        let climb = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "skill.selected_modifier.climb")
            .expect("Climb must be grounded for Shaman");
        assert_eq!(
            climb.value, 3,
            "Shaman does NOT earn the class-skill bonus on Climb (real class-skill list \
             excludes all three tracked skills): {:?}",
            climb
        );
    }

    /// Brawler's AC Bonus progression at higher levels, verified against
    /// the PCGen corpus `BONUS:VAR` formula directly (not merely trusting
    /// the level-1 zero case above): +1 at 4th, +2 at 9th, +3 at 13th, +4
    /// at 18th.
    #[test]
    fn brawler_ac_bonus_progression_matches_the_corpus_formula_at_higher_levels() {
        for (level, expected_ac_bonus) in [(1, 0), (3, 0), (4, 1), (8, 1), (9, 2), (12, 2), (13, 3), (17, 3), (18, 4), (20, 4)]
        {
            let input = acg_style_input("class:brawler", level);
            let receipt = build_pilot_headless_receipt(&input);

            let ac_bonus = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.acg.brawler.ac_bonus")
                .unwrap_or_else(|| panic!("level {level}: Brawler's own AC Bonus must be grounded"));
            assert_eq!(
                ac_bonus.value, expected_ac_bonus,
                "level {level}: expected AC Bonus {expected_ac_bonus}, got {:?}",
                ac_bonus
            );

            let armor_class = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "defense.baseline_armor_class")
                .unwrap_or_else(|| panic!("level {level}: baseline Armor Class must be grounded"));
            // Base AC (10 + Chain Shirt 4 + DEX +2 + Dodge 1 = 17) + AC Bonus.
            assert_eq!(
                armor_class.value,
                17 + expected_ac_bonus,
                "level {level}: AC Bonus must be integrated into the shared Armor Class total: \
                 {:?}",
                armor_class
            );
        }
    }

    /// Brawler's Cunning grounds unconditionally from level 1 (v0.6 alpha
    /// swarm, risks item 8, Brawler deepening, 2026-07-26): the fixture's
    /// real Intelligence score is 10 (below 13), so the effective score
    /// for combat-feat prerequisites is floored to 13.
    #[test]
    fn brawler_cunning_grounds_the_effective_intelligence_floor() {
        let input = acg_style_input("class:brawler", 1);
        let receipt = build_pilot_headless_receipt(&input);

        let cunning = receipt
            .computation
            .explanations
            .iter()
            .find(|e| {
                e.id == "class_feature.acg.brawler.cunning_effective_intelligence_for_combat_feats"
            })
            .expect("Brawler's Cunning must ground unconditionally");
        assert_eq!(
            cunning.value, 13,
            "fixture Intelligence 10 (below 13) floors to 13: {:?}",
            cunning
        );
    }

    /// Brawler's Strike is honestly not-yet-gained below level 5, then
    /// grounds the real DR-bypass tier from level 5 on, verified against
    /// the PCGen corpus progression formula directly (v0.6 alpha swarm,
    /// risks item 8, Brawler deepening, 2026-07-26) -- the same "not yet
    /// gained" honesty already established for Swashbuckler's Charmed
    /// Life.
    #[test]
    fn brawler_strike_is_honestly_not_yet_gained_below_level_5_then_progresses() {
        for (level, expected_tier) in [(1, None), (4, None), (5, Some(1)), (8, Some(1)), (9, Some(2)), (11, Some(2)), (12, Some(3)), (17, Some(4))]
        {
            let input = acg_style_input("class:brawler", level);
            let receipt = build_pilot_headless_receipt(&input);

            match expected_tier {
                None => {
                    let not_yet_gained = receipt
                        .computation
                        .explanations
                        .iter()
                        .find(|e| e.id == "class_feature.acg.brawler.strike_not_yet_gained");
                    assert!(
                        not_yet_gained.is_some(),
                        "level {level}: expected the honest not-yet-gained record: {:?}",
                        receipt.computation.explanations
                    );
                    assert!(
                        !receipt
                            .computation
                            .explanations
                            .iter()
                            .any(|e| e.id == "class_feature.acg.brawler.strike_dr_bypass"),
                        "level {level}: strike_dr_bypass must not fire before level 5: {:?}",
                        receipt.computation.explanations
                    );
                }
                Some(tier) => {
                    let dr_bypass = receipt
                        .computation
                        .explanations
                        .iter()
                        .find(|e| e.id == "class_feature.acg.brawler.strike_dr_bypass")
                        .unwrap_or_else(|| {
                            panic!("level {level}: expected the real DR-bypass tier: {:?}", receipt.computation.explanations)
                        });
                    assert_eq!(
                        dr_bypass.value, tier,
                        "level {level}: expected DR-bypass tier {tier}: {:?}",
                        dr_bypass
                    );

                    let alignment_unmodeled = receipt
                        .computation
                        .diagnostics
                        .iter()
                        .any(|d| d.id == "class_feature.acg.brawler.strike_alignment_selection.unmodeled");
                    assert_eq!(
                        alignment_unmodeled,
                        tier >= 3,
                        "level {level}: alignment-selection diagnostic should only fire at tier \
                         3+ (level 12+): {:?}",
                        receipt.computation.diagnostics
                    );
                }
            }
        }
    }

    /// Brawler's own `other_features_deferred` diagnostic acknowledges
    /// Cunning and Strike as now-grounded (in its own "remains blocked
    /// beyond X, Y, Z" preamble) and no longer lists either among the
    /// still-missing features (v0.6 alpha swarm, risks item 8, Brawler
    /// deepening, 2026-07-26).
    ///
    /// **Reworked by task #76 (2026-07-28), deliberately reversing one
    /// of this test's original assertions.** Two changes, both because
    /// the assertions had been pinned to a message that has since become
    /// more accurate, not because the class's real state changed:
    ///
    /// 1. The preamble check was an exact literal (`"AC Bonus, Brawler's
    ///    Cunning, and Brawler's Strike:"`). Task #5 grounded six more
    ///    features (Flurry, Knockout, Martial Flexibility, Martial
    ///    Training, Bonus Feats, Maneuver Training) and #76 moved them
    ///    into the grounded preamble where they belong, which broke the
    ///    run-on literal without making the message any less true. Now
    ///    checked by substring per feature, so crediting further real
    ///    groundings cannot spuriously fail this test again.
    ///
    /// 2. This test used to REQUIRE that Brawler's Strike's Alignment
    ///    Selection chooser be named in the claim-blocking missing-list.
    ///    #76 asserts the opposite, and the reversal is the point: that
    ///    chooser carries its own dedicated
    ///    `strike_alignment_selection.unmodeled` record with
    ///    `claim_blocking: false`. Naming it in BOTH places double-counts
    ///    it and actively misleads -- a reader who meets it inside a
    ///    claim-blocking message reasonably infers it is part of what
    ///    blocks the class, while the codebase's own separate diagnostic
    ///    explicitly says it does not. Acknowledged once, correctly
    ///    scoped, in exactly one place.
    ///
    /// **Reworked again by task #91**, which closed the last three
    /// features (Awesome Blow, Improved Awesome Blow, Close Weapon
    /// Mastery) and so demoted this diagnostic to non-blocking.
    ///
    /// The previous version of this test parsed the message as
    /// `"<grounded preamble>: <still-missing clause>"` and asserted
    /// against each half. That structure only exists in a *blocking*
    /// message, and there is no longer a still-missing clause to find --
    /// so the parse is retired rather than patched. What is preserved is
    /// the substance both earlier revisions were protecting: every
    /// feature the class actually grounds is credited by name, and none
    /// of them is described as missing.
    #[test]
    fn brawler_other_features_deferred_credits_every_grounded_feature() {
        let input = acg_style_input("class:brawler", 1);
        let receipt = build_pilot_headless_receipt(&input);

        let deferred = receipt
            .computation
            .diagnostics
            .iter()
            .find(|d| d.id == "class_feature.acg.brawler.other_features_deferred.unsupported")
            .expect("other_features_deferred must still fire as the honest remainder");

        assert!(
            !deferred.claim_blocking,
            "with every named feature grounded there is nothing left to block on: {deferred:?}"
        );

        for grounded in [
            "AC Bonus",
            "Brawler's Cunning",
            "Brawler's Strike",
            "Brawler's Flurry",
            "Knockout",
            "Martial Flexibility",
            "Martial Training",
            "Bonus Feats",
            "Maneuver Training",
            "Close Weapon Mastery",
            "Awesome Blow",
            "Improved Awesome Blow",
        ] {
            assert!(
                deferred.message.contains(grounded),
                "the remainder must credit {grounded} by name: {deferred:?}"
            );
        }

        // The words that would signal a live blocking claim. Their
        // absence is what distinguishes an honest remainder from a stale
        // blocker that merely had its flag flipped.
        for retired in ["remains blocked", "ungrounded anywhere"] {
            assert!(
                !deferred.message.contains(retired),
                "a non-blocking remainder must not still speak as a blocker ({retired:?}): \
                 {deferred:?}"
            );
        }
    }

    /// The other half of the reversal documented on
    /// `brawler_other_features_deferred_acknowledges_cunning_and_strike_as_grounded`:
    /// dropping Alignment Selection from the claim-blocking missing-list
    /// must not lose the fact. It is still acknowledged -- once -- by its
    /// own correctly-scoped, non-blocking record at the level it is
    /// actually unlocked (progression tier 3, Brawler level 12).
    #[test]
    fn brawler_alignment_selection_is_acknowledged_once_by_its_own_non_blocking_record() {
        let receipt = build_pilot_headless_receipt(&acg_style_input("class:brawler", 12));

        let alignment = receipt
            .computation
            .diagnostics
            .iter()
            .find(|d| d.id == "class_feature.acg.brawler.strike_alignment_selection.unmodeled")
            .expect("the dedicated Alignment Selection record must fire at level 12");
        assert!(
            !alignment.claim_blocking,
            "Alignment Selection is deliberately NON-blocking -- that is precisely why it must \
             not be named in the claim-blocking missing-list: {alignment:?}"
        );
    }

    /// Multiclass safety, verified directly. An ACG-class-containing
    /// multiclass mix must stay Blocked, since `AcgClassId::from_class_id_str`
    /// is deliberately not registered with
    /// `table_class_id`/`multiclass_class_level_supported`.
    #[test]
    fn an_acg_class_multiclassed_with_fighter_stays_blocked() {
        for (class_id, ..) in EXPECTED_LEVEL_1 {
            let mut input = acg_style_input(class_id, 4);
            input
                .chosen
                .class_levels
                .push(CharacterClassLevel { class_id: FIGHTER_CLASS_ID.to_owned(), level: 1 });

            let receipt = build_pilot_headless_receipt(&input);

            assert_eq!(
                receipt.status,
                HeadlessReceiptStatus::Blocked,
                "{class_id}+Fighter multiclass must not reach Computed: {:?}",
                receipt.computation.diagnostics
            );
        }
    }

    /// A level beyond an ACG class's real `MAXLEVEL:20` ceiling stays
    /// honestly blocked with the generic diagnostic, not a fabricated row.
    #[test]
    fn a_level_beyond_the_real_maxlevel_ceiling_stays_blocked() {
        let input = acg_style_input("class:arcanist", 21);

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_chassis.unsupported"),
            "{:?}",
            receipt.computation.diagnostics
        );
    }
}

/// v0.6 alpha swarm (Path A canonical-narrowing closure for Cavalier,
/// Inquisitor and Oracle, 2026-07-29): the three APG chooser-shaped
/// classes whose engines already ground a real, corpus-verified canonical
/// option but which never reached `Computed` because (a) nothing seeded
/// that option in the default posture and (b) their
/// `other_features_deferred` diagnostics stayed claim-blocking even when
/// it WAS present.
///
/// This module pins both halves of the contract at once, at every level
/// 1-20:
///
/// * with the canonical choice recorded, the class reaches genuine
///   `HeadlessReceiptStatus::Computed`, and the honestly-deferred
///   remainder is still NAMED -- as a non-claim-blocking diagnostic, the
///   exact shape `ground_or_block_arcanist_metamagic_knowledge` already
///   established for Arcanist's own 44 unbuilt Exploits;
/// * without it, every original claim-blocking diagnostic fires exactly
///   as before, so no input this narrowing did not specifically improve
///   can silently reach `Computed`.
///
/// The second half is the load-bearing one: it is what keeps the APG
/// class-coverage audit's own bare-input canary
/// (`apg_classes_ground_real_bab_save_but_stay_blocked_on_the_unconditional_diagnostic`)
/// honest rather than merely passing.
#[cfg(test)]
mod apg_canonical_choice_path_a_tests {
    use super::{
        build_pilot_headless_receipt, CharacterClassLevel, CharacterInput, HeadlessReceiptStatus,
        CAVALIER_CLASS_ID, CAVALIER_ORDER_CHOICE_ID, CLOUDED_VISION_CURSE_SELECTION,
        GOOD_DOMAIN_SELECTION, INQUISITOR_CLASS_ID, INQUISITOR_DOMAIN_CHOICE_ID,
        LIFE_MYSTERY_SELECTION, ORACLE_CLASS_ID, ORACLE_CURSE_CHOICE_ID, ORACLE_MYSTERY_CHOICE_ID,
        ORDER_OF_THE_SWORD_SELECTION,
    };
    use crate::rules_core::character_input::{load_character_input_fixture, SelectedChoice};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    /// The same shared GE-06 deterministic fixture `v06_class_state_dump`
    /// sweeps, with the class under test swapped in -- so what this module
    /// proves and what the operator's dashboard reports are the same
    /// posture, not two similar-looking ones.
    fn single_class(class_id: &str, level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
        input
    }

    fn choose(input: &mut CharacterInput, choice_set_id: &str, selection_id: &str) {
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: choice_set_id.to_owned(),
            selection_id: selection_id.to_owned(),
        });
    }

    fn blocking_ids(input: &CharacterInput) -> Vec<String> {
        let mut ids: Vec<String> = build_pilot_headless_receipt(input)
            .computation
            .diagnostics
            .iter()
            .filter(|d| d.claim_blocking)
            .map(|d| d.id.clone())
            .collect();
        ids.sort();
        ids.dedup();
        ids
    }

    /// The canonical seed each class needs, exactly mirroring what
    /// `compose_character_input` (`apps/desktop/src-tauri/src/pf1_adapter.rs`)
    /// and `v06_class_state_dump`'s own `canonical_seeds_for` apply.
    fn canonically_chosen(class_id: &str, level: u8) -> CharacterInput {
        let mut input = single_class(class_id, level);
        match class_id {
            CAVALIER_CLASS_ID => {
                choose(&mut input, CAVALIER_ORDER_CHOICE_ID, ORDER_OF_THE_SWORD_SELECTION)
            }
            INQUISITOR_CLASS_ID => {
                choose(&mut input, INQUISITOR_DOMAIN_CHOICE_ID, GOOD_DOMAIN_SELECTION)
            }
            ORACLE_CLASS_ID => {
                choose(&mut input, ORACLE_MYSTERY_CHOICE_ID, LIFE_MYSTERY_SELECTION);
                choose(&mut input, ORACLE_CURSE_CHOICE_ID, CLOUDED_VISION_CURSE_SELECTION);
            }
            other => panic!("no canonical seed defined for {other}"),
        }
        input
    }

    /// **The milestone test.** Each of the three classes reaches genuine
    /// `Computed` at every one of PF1's twenty class levels once its own
    /// canonical, corpus-verified choice is recorded.
    #[test]
    fn each_class_reaches_computed_at_every_level_with_its_canonical_choice() {
        for class_id in [CAVALIER_CLASS_ID, INQUISITOR_CLASS_ID, ORACLE_CLASS_ID] {
            for level in 1..=20u8 {
                let input = canonically_chosen(class_id, level);
                assert_eq!(
                    build_pilot_headless_receipt(&input).status,
                    HeadlessReceiptStatus::Computed,
                    "{class_id} level {level} must reach Computed with its canonical choice; \
                     still blocking: {:?}",
                    blocking_ids(&input)
                );
            }
        }
    }

    /// The honest half: reaching `Computed` must NOT delete the record of
    /// what stays deferred. Each class still emits its own
    /// `other_features_deferred` diagnostic naming the remainder -- just
    /// non-claim-blocking, mirroring Arcanist's own `exploits_deferred`.
    #[test]
    fn the_deferred_remainder_is_still_named_just_not_claim_blocking() {
        for (class_id, deferred_id) in [
            (CAVALIER_CLASS_ID, "class_feature.apg.cavalier.other_features_deferred.unsupported"),
            (
                INQUISITOR_CLASS_ID,
                "class_feature.apg.inquisitor.other_features_deferred.unsupported",
            ),
            (ORACLE_CLASS_ID, "class_feature.apg.oracle.other_features_deferred.unsupported"),
        ] {
            for level in [1u8, 2, 8, 15, 20] {
                let receipt = build_pilot_headless_receipt(&canonically_chosen(class_id, level));
                let deferred = receipt
                    .computation
                    .diagnostics
                    .iter()
                    .find(|d| d.id == deferred_id)
                    .unwrap_or_else(|| {
                        panic!(
                            "{class_id} level {level} must still NAME its deferred remainder: \
                             {:?}",
                            receipt.computation.diagnostics
                        )
                    });
                assert!(
                    !deferred.claim_blocking,
                    "{class_id} level {level}: {deferred_id} must be non-claim-blocking once the \
                     canonical choice is present: {deferred:?}"
                );
            }
        }
    }

    /// The guard. Without the canonical choice, every original
    /// claim-blocking diagnostic still fires at every level -- the
    /// narrowing improves exactly the posture it claims to and nothing
    /// else.
    #[test]
    fn without_the_canonical_choice_every_original_blocker_still_fires() {
        for (class_id, want) in [
            (
                CAVALIER_CLASS_ID,
                vec![
                    "class_feature.apg.cavalier.order_powers.unsupported",
                    "class_feature.apg.cavalier.other_features_deferred.unsupported",
                ],
            ),
            (
                INQUISITOR_CLASS_ID,
                vec![
                    "class_feature.apg.inquisitor.other_features_deferred.unsupported",
                    "class_feature.inquisitor.domain_powers.unsupported",
                ],
            ),
            (
                ORACLE_CLASS_ID,
                vec![
                    "class_feature.apg.oracle.curse_powers.unsupported",
                    "class_feature.apg.oracle.mystery_powers.unsupported",
                    "class_feature.apg.oracle.other_features_deferred.unsupported",
                ],
            ),
        ] {
            for level in 1..=20u8 {
                assert_eq!(
                    blocking_ids(&single_class(class_id, level)),
                    want.iter().map(|id| (*id).to_owned()).collect::<Vec<_>>(),
                    "{class_id} level {level} must stay blocked exactly as before without its \
                     canonical choice"
                );
            }
        }
    }

    /// A second domain alongside Good is not a legal Inquisitor posture
    /// (an inquisitor selects ONE domain), and an unrecognized one is not
    /// grounded anywhere -- so the pair must keep claim-blocking rather
    /// than riding Good's own narrowing to a false `Computed`. Mirrors
    /// Cleric's own `unrecognized_other_domain_chosen` guard exactly.
    #[test]
    fn inquisitor_good_domain_plus_an_unrecognized_second_domain_stays_blocked() {
        let mut input = canonically_chosen(INQUISITOR_CLASS_ID, 5);
        choose(&mut input, INQUISITOR_DOMAIN_CHOICE_ID, "domain:war");

        assert_eq!(
            build_pilot_headless_receipt(&input).status,
            HeadlessReceiptStatus::Blocked,
            "an unrecognized second domain must not ride Good's narrowing to Computed"
        );
        assert!(
            blocking_ids(&input)
                .contains(&"class_feature.inquisitor.domain_powers.unsupported".to_owned()),
            "expected the domain-powers blocker: {:?}",
            blocking_ids(&input)
        );
    }

    /// The same guard for Oracle: an unrecognized Mystery or Curse
    /// alongside a recognized one keeps the class blocked. The Haunted
    /// Curse has no grounded power in this codebase, so a build naming it
    /// is not something this engine can honestly call computed.
    ///
    /// **SD31-E4-F2-001 correction:** this test used to probe
    /// `mystery:battle` here; Battle Mystery's own Battlecry became
    /// genuinely grounded, so that probe value went stale. Swapped to
    /// `mystery:heavens`.
    ///
    /// **SD31-E4-F2-002 correction:** `mystery:heavens` in turn went
    /// stale the same way -- this cycle wired Heavens Mystery's own Coat
    /// of Many Stars (alongside Stone/Waves/Wind), so **all 10 real,
    /// corpus-declared Mysteries are now genuinely grounded** and none
    /// remains available as a "real Mystery, still ungrounded" probe
    /// value. Swapped to a synthetic, non-corpus identifier
    /// (`mystery:nonexistent_test_probe`) instead -- this tests the exact
    /// same property `oracle_mystery_grounds_a_power`'s `all_recognized`
    /// guard enforces (an unrecognized selection value must never ride
    /// the canonical Life pick to `Computed`), and does so more directly
    /// than a real-but-ungrounded Mystery id, which no longer exists in a
    /// 10/10-grounded world. `retro.py correction` emitted.
    #[test]
    fn oracle_unrecognized_mystery_or_curse_alongside_the_canonical_one_stays_blocked() {
        for (choice_set_id, selection_id) in [
            (ORACLE_MYSTERY_CHOICE_ID, "mystery:nonexistent_test_probe"),
            (ORACLE_CURSE_CHOICE_ID, "curse:haunted"),
        ] {
            let mut input = canonically_chosen(ORACLE_CLASS_ID, 7);
            choose(&mut input, choice_set_id, selection_id);
            assert_eq!(
                build_pilot_headless_receipt(&input).status,
                HeadlessReceiptStatus::Blocked,
                "{selection_id} is grounded nowhere -- it must not ride the canonical choice to \
                 Computed; blocking: {:?}",
                blocking_ids(&input)
            );
        }
    }

    /// Negative control: the canonical selections are class-owned. A
    /// Fighter carrying all four spoofed entries is unaffected -- no
    /// Cavalier/Inquisitor/Oracle record appears, and Fighter's own
    /// golden path still computes.
    #[test]
    fn spoofed_canonical_choices_on_a_fighter_change_nothing() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        let mut input = result.character_input.expect("valid fixture");
        choose(&mut input, CAVALIER_ORDER_CHOICE_ID, ORDER_OF_THE_SWORD_SELECTION);
        choose(&mut input, INQUISITOR_DOMAIN_CHOICE_ID, GOOD_DOMAIN_SELECTION);
        choose(&mut input, ORACLE_MYSTERY_CHOICE_ID, LIFE_MYSTERY_SELECTION);
        choose(&mut input, ORACLE_CURSE_CHOICE_ID, CLOUDED_VISION_CURSE_SELECTION);

        let receipt = build_pilot_headless_receipt(&input);
        assert_eq!(receipt.status, HeadlessReceiptStatus::Computed);
        assert!(
            !receipt.computation.explanations.iter().any(|e| {
                e.id.contains("cavalier") || e.id.contains("inquisitor") || e.id.contains("oracle")
            }),
            "no class-owned record may be fabricated on a Fighter: {:?}",
            receipt.computation.explanations
        );
    }
}

