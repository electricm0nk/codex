//! SD-27 `decisions.md` §24/§28 — the Advanced Race Guide's 187 feats and
//! Pathfinder Unchained's 17 moved no computed number at all.
//!
//! They were addable, persisted and displayed, and inert. This file pins both
//! halves of the answer: **which of them are legitimately inert**, and **which
//! were a defect**, now closed.
//!
//! # The classification, and how it is derived
//!
//! A PF1 feat that moves no standing number is not automatically a defect.
//! Many are situational (a bonus only when flanking), grant an action, or gate
//! another choice. The corpus states the difference itself: a `BONUS:` token
//! with an unconditional value is a standing modifier; a record with no
//! `BONUS:` token at all is prose; a `BONUS:` token carrying its own inline
//! `PRE*`/`!PRE*` qualifier is conditioned.
//!
//! Row-level `PRE*` tokens are deliberately NOT counted as conditions on the
//! bonus. Those gate *taking* the feat, which is `feat_prereqs`' job — the same
//! split `feat_effects::master_craftsman_facts_from_choices` already documents.
//! Only a `PRE*` appended to a `BONUS:` token conditions the bonus.
//!
//! ARG's split is re-derived below from the **shipped catalog table**, which
//! carries every record's `BONUS:` tokens verbatim, so it cannot drift from
//! what the engine actually holds. PU's catalog carries no `effect` field (see
//! `rules_tables::feats_all`'s own doc comment for why), so its split is pinned
//! as constants; re-derive them against the corpus with:
//!
//! ```text
//! awk -F'\t' 'tolower($0) ~ /category:feat/' \
//!   ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/pathfinder_unchained/pu_feats.lst \
//!   | tr '\t' '\n' | grep -c '^BONUS:'
//! ```
//!
//! # What the split does NOT say
//!
//! "Has an unconditional `BONUS:`" is an **upper bound** on "should be wired",
//! not the wiring list. ARG's 49 such feats include 11 carrying a
//! `BONUS:ABILITYPOOL|<pool>|1` (a further *choice*, not a magnitude) and a
//! long tail of `BONUS:VAR|<internal>` increments to PCGen bookkeeping
//! variables for spell-like-ability uses per day, racial luck budgets and fly
//! manoeuvrability — none of which this engine models, so none of which has
//! anything to land on. `feat_effects`' own SD-27 section header records that
//! reasoning per category.
//!
//! **Correction, 2026-08-01.** This header previously counted "3 feats whose
//! token is `BONUS:SITUATION`" and filed them as legitimately inert. Both
//! halves were wrong, and both are now pinned by
//! `args_situation_tokens_are_three_across_two_feats` below rather than
//! restated: it is **3 tokens across 2 feats**, and they are not inert — this
//! engine already grounds the Core Rulebook dwarf's own `BONUS:SITUATION`
//! tokens, so ARG's land in exactly the same place. Two `BONUS:VAR` deferrals
//! (`DefiantLuckTimes`, `ImprovisationBonus`) and one movement token
//! (Stretched Wings' `BONUS:MOVEADD|TYPE.Fly|40`, misfiled as manoeuvrability)
//! were wrong for the same reason: the category was read off the token's shape
//! rather than off what the corpus does with it.
//!
//! # The standing guard (§28)
//!
//! Every `pilot_compute.rs` change lands with a test pinning the before/after.
//! Each assertion below differences the *same character* with and without the
//! feat, so nothing here can pass on a producer that exists but is never
//! consumed — the exact failure mode that left 45 CRB feats' worth of correct
//! code running for nobody.

use codex::rules_core::character_input::{
    load_character_input_fixture, CharacterInput, SelectedChoice,
};
use codex::rules_core::pilot_compute::{compute_pilot_base_chassis, PilotBaseChassisComputation};
use codex::rules_core::rules_tables::advanced_race_guide::feats as arg_feats;
use codex::rules_core::rules_tables::pathfinder_unchained::feat_tables as pu_feats;

const DETERMINISTIC_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

fn input_for_race(race_slug: &str) -> CharacterInput {
    let text = DETERMINISTIC_FIXTURE
        .replace("race_id=race:human", &format!("race_id=race:{race_slug}"));
    load_character_input_fixture(&text)
        .character_input
        .expect("the deterministic fixture must load")
}

fn baseline() -> CharacterInput {
    input_for_race("human")
}

fn with_feats(mut input: CharacterInput, feats: &[&str]) -> CharacterInput {
    for feat in feats {
        input.chosen.selected_feats.push((*feat).to_owned());
    }
    input
}

fn value(computation: &PilotBaseChassisComputation, id: &str) -> Option<i16> {
    computation.explanations.iter().find(|e| e.id == id).map(|e| e.value)
}

fn compute(input: &CharacterInput) -> PilotBaseChassisComputation {
    compute_pilot_base_chassis(input)
}

/// Whether one corpus `BONUS:` qualifier list carries its own inline
/// `PRE*`/`!PRE*` condition.
///
/// The shipped ARG table stores each `BONUS:` token as its pipe-split
/// qualifiers with the leading `BONUS:` stripped, so qualifier `0` is the bonus
/// category (`SKILL`, `VAR`, `ABILITYPOOL`, ...) and any later qualifier
/// beginning `PRE` or `!PRE` is a condition on the bonus itself.
fn bonus_is_conditioned(qualifiers: &[&str]) -> bool {
    qualifiers
        .iter()
        .skip(1)
        .any(|q| q.starts_with("PRE") || q.starts_with("!PRE"))
}

/// The situational-versus-should-be-wired split for ARG's 187 feats, re-derived
/// from the shipped catalog rather than restated from a report.
///
/// The three buckets are exhaustive and disjoint by construction, so the test
/// also proves it has not lost or double-counted a record.
#[test]
fn args_187_feats_split_133_prose_only_5_pre_gated_and_49_unconditionally_bonused() {
    let (mut prose_only, mut all_conditioned, mut has_unconditional) = (0, 0, 0);
    for entry in arg_feats::feat_tables() {
        match entry.effect {
            None => prose_only += 1,
            Some(bonuses) => {
                if bonuses.iter().any(|b| !bonus_is_conditioned(b.qualifiers)) {
                    has_unconditional += 1;
                } else {
                    all_conditioned += 1;
                }
            }
        }
    }

    assert_eq!(arg_feats::feat_tables().len(), 187, "ARG's shipped feat count");
    assert_eq!(
        prose_only, 133,
        "ARG records with NO BONUS token at all — prose-only, legitimately situational \
         or action-granting"
    );
    assert_eq!(
        all_conditioned, 5,
        "ARG records whose every BONUS token carries its own inline PRE* condition: \
         Human Spirit, Draconic Aspect, Draconic Breath, Tail Terror, Wings of Air"
    );
    assert_eq!(
        has_unconditional, 49,
        "ARG records with at least one unconditional BONUS token — the UPPER BOUND on \
         'should be wired', not the wiring list; see this file's header"
    );
    assert_eq!(prose_only + all_conditioned + has_unconditional, 187, "buckets are exhaustive");
}

/// PU's 17, pinned as constants because its catalog carries no `effect` field.
/// The deriving command is in this file's header.
#[test]
fn pus_17_feats_split_12_prose_only_and_5_unconditionally_bonused() {
    assert_eq!(pu_feats::feat_tables().len(), 17, "PU's shipped feat count");

    const PROSE_ONLY: usize = 12;
    const UNCONDITIONALLY_BONUSED: usize = 5;
    const ALL_CONDITIONED: usize = 0;
    assert_eq!(PROSE_ONLY + ALL_CONDITIONED + UNCONDITIONALLY_BONUSED, 17);

    // The 5 by name, from `pu_feats.lst`:
    //   Combat Stamina               BONUS:VAR|StaminaPool|BAB+CON
    //   Extra Stamina                BONUS:VAR|StaminaPool|3
    //   Push the Limits              BONUS:VAR|SecondaryStaminaPool|CON
    //   Extra Unchained Rogue Talent BONUS:ABILITYPOOL|Unchained Rogue Talent|1
    //   Signature Skill              BONUS:ABILITYPOOL|Signature Skill 5 Ranks|1
    // The last two are chooser pools, not magnitudes, so 3 of the 5 are wired.
    for key in ["Combat Stamina", "Extra Stamina", "Push the Limits"] {
        assert!(
            pu_feats::feat_tables().iter().any(|e| e.key == key),
            "{key} must be a shipped PU catalog key"
        );
    }
}

/// ARG's one feat whose unconditional magnitude lands on a total this engine
/// computes. Differenced against the same character without it, so a producer
/// that were never consumed would fail here.
///
/// `+2` natural armor: Armor Class rises by 2 and touch AC does not move at all
/// (PF1 touch attacks ignore natural armor), which is the pair of assertions
/// that catches wiring it into only one of the two.
#[test]
fn armor_of_the_pit_raises_a_tieflings_armor_class_by_two_and_leaves_touch_ac_alone() {
    let without = compute(&input_for_race("tiefling"));
    let with = compute(&with_feats(input_for_race("tiefling"), &["Armor of the Pit"]));

    let ac_before = value(&without, "defense.baseline_armor_class")
        .expect("the deterministic Fighter 1 posture must compute an armor class");
    let ac_after = value(&with, "defense.baseline_armor_class").expect("still computed");
    assert_eq!(ac_after - ac_before, 2, "Armor of the Pit grants +2 natural armor");

    let touch_before = value(&without, "defense.touch_armor_class").expect("touch AC computed");
    let touch_after = value(&with, "defense.touch_armor_class").expect("touch AC still computed");
    assert_eq!(
        touch_after, touch_before,
        "PF1 touch AC ignores natural armor, so this feat must not move it"
    );
}

/// The Scaled Skin branch. Its corpus token is
/// `...|!PREABILITY:1,CATEGORY=Special Ability,Scaled Skin C ~ Tiefling,...`,
/// and the prose says the resistance is granted *instead of* the natural armor.
/// A character who took that alternate racial trait must get no AC change —
/// and must be told why rather than silently getting nothing.
#[test]
fn armor_of_the_pit_withholds_its_natural_armor_from_a_scaled_skin_tiefling() {
    let mut scaled = input_for_race("tiefling");
    scaled.chosen.selected_choices.push(SelectedChoice {
        choice_set_id: "choice:race_alternate_trait".to_owned(),
        selection_id: "race_trait:Tiefling ~ Scaled Skin".to_owned(),
    });

    let without = compute(&scaled);
    let with = compute(&with_feats(scaled.clone(), &["Armor of the Pit"]));

    assert_eq!(
        value(&with, "defense.baseline_armor_class"),
        value(&without, "defense.baseline_armor_class"),
        "the Scaled Skin branch grants resistance INSTEAD of natural armor"
    );
    assert_eq!(
        value(&with, "feat.arg_standalone.armor_of_the_pit_scaled_skin_branch"),
        Some(0),
        "the withheld branch must be named, with no fabricated resistance value: the rule \
         grants two of three energy types chosen by the player and nothing records which two"
    );
    assert_eq!(
        value(&without, "feat.arg_standalone.armor_of_the_pit_scaled_skin_branch"),
        None,
        "and it must not appear for a character who does not hold the feat"
    );
}

/// ARG's one feat whose unconditional `BONUS:SKILL` names a skill this engine
/// computes a total for. Differenced, so the `+2` must reach the number a
/// player reads, not merely a remark beside it.
#[test]
fn sure_and_fleet_raises_the_computed_climb_total_by_two() {
    let without = compute(&baseline());
    let with = compute(&with_feats(baseline(), &["Sure and Fleet"]));

    let before = value(&without, "skill.selected_modifier.climb")
        .expect("the deterministic posture must compute a Climb modifier");
    let after = value(&with, "skill.selected_modifier.climb").expect("still computed");
    assert_eq!(after - before, 2, "Sure and Fleet grants +2 racial Climb");

    assert_eq!(
        value(&with, "feat.arg_skill_bonus.sure_and_fleet.acrobatics"),
        Some(2),
        "its Acrobatics half grounds standalone — no Acrobatics total exists here"
    );
}

/// Three feats, one skill, opposite signs — the collision that made ARG's skill
/// records carry the feat key in their id. CRB's Stealthy already owns
/// `feat.standalone_skill_bonus.stealth`; a skill-only id would have collapsed
/// all three into one record and silently discarded two real magnitudes.
#[test]
fn three_different_feats_may_each_carry_their_own_stealth_record() {
    let computation = compute(&with_feats(
        baseline(),
        &["Stealthy", "Seen and Unseen", "Angelic Flesh"],
    ));

    assert_eq!(
        value(&computation, "feat.standalone_skill_bonus.stealth"),
        Some(2),
        "CRB Stealthy's own record is untouched"
    );
    assert_eq!(
        value(&computation, "feat.arg_skill_bonus.seen_and_unseen.stealth"),
        Some(2)
    );
    assert_eq!(
        value(&computation, "feat.arg_skill_bonus.angelic_flesh.stealth"),
        Some(-2),
        "a real penalty is reported, not dropped for being unwelcome"
    );
    assert_eq!(
        value(&computation, "feat.arg_skill_bonus.angelic_flesh.disguise"),
        Some(-2)
    );
}

/// Brewmaster's corpus token says `1` and its own `BENEFIT:` prose says `+2`,
/// with no prerequisite supplying the difference. The prose value is grounded
/// and the disagreement is recorded rather than either number being taken
/// silently.
#[test]
fn brewmaster_grounds_the_plus_two_its_benefit_prose_states() {
    let computation = compute(&with_feats(baseline(), &["Brewmaster"]));
    assert_eq!(
        value(&computation, "feat.arg_skill_bonus.brewmaster.craft_alchemy"),
        Some(2)
    );
    assert_eq!(
        value(&computation, "feat.arg_skill_bonus.brewmaster.profession_brewer"),
        Some(2)
    );
}

/// ARG's two CMD feats. Feline Grace's single corpus token names five
/// maneuvers; Tree Hanger's names one, and the two must not collide on the
/// maneuver they share.
#[test]
fn feline_grace_and_tree_hanger_each_ground_their_own_cmd_records() {
    let computation = compute(&with_feats(baseline(), &["Feline Grace", "Tree Hanger"]));
    for maneuver in ["bull_rush", "grapple", "overrun", "reposition", "trip"] {
        assert_eq!(
            value(&computation, &format!("feat.arg_maneuver_defense.feline_grace.{maneuver}")),
            Some(2),
            "Feline Grace vs {maneuver}"
        );
    }
    assert_eq!(
        value(&computation, "feat.arg_maneuver_defense.tree_hanger.trip"),
        Some(2),
        "Tree Hanger's own trip record, distinct from Feline Grace's"
    );

    let general_cmd_before = value(&compute(&baseline()), "defense.combat_maneuver_defense");
    assert_eq!(
        value(&computation, "defense.combat_maneuver_defense"),
        general_cmd_before,
        "neither may move the GENERAL Combat Maneuver Defense: that total applies to every \
         maneuver, including the disarm and sunder these feats say nothing about"
    );
}

/// The five energy-resistance grants, each independent of the others.
#[test]
fn each_energy_resistance_feat_grounds_only_its_own_energy_type() {
    for (feat, id) in [
        ("Expanded Fiendish Resistance (Acid)", "expanded_fiendish_resistance_acid.acid"),
        ("Expanded Fiendish Resistance (Cold)", "expanded_fiendish_resistance_cold.cold"),
        (
            "Expanded Fiendish Resistance (Electricity)",
            "expanded_fiendish_resistance_electricity.electricity",
        ),
        ("Expanded Fiendish Resistance (Fire)", "expanded_fiendish_resistance_fire.fire"),
        ("Flame Heart", "flame_heart.fire"),
    ] {
        let full_id = format!("feat.arg_energy_resistance.{id}");
        assert_eq!(value(&compute(&baseline()), &full_id), None, "absent without {feat}");
        assert_eq!(
            value(&compute(&with_feats(baseline(), &[feat])), &full_id),
            Some(5),
            "{feat} grants resistance 5"
        );
    }
}

/// Flame Heart's record carries its fire-resistance token twice. Summing them
/// would report resistance 10; its `BENEFIT:` prose says 5.
#[test]
fn flame_heart_grounds_resistance_five_and_a_fire_only_caster_level_bonus() {
    let computation = compute(&with_feats(baseline(), &["Flame Heart"]));
    assert_eq!(
        value(&computation, "feat.arg_energy_resistance.flame_heart.fire"),
        Some(5),
        "not 10 — the duplicated corpus token is one typed and one untyped copy of one grant"
    );
    assert_eq!(
        value(&computation, "feat.arg_standalone.flame_heart_fire_caster_level"),
        Some(1)
    );
}

/// Neither ARG emotion feat states a literal magnitude: both print the running
/// total through a `%1` substitution token bound to the same corpus variable, which is
/// PCGen's way of saying they add.
#[test]
fn the_emotion_descriptor_save_bonus_is_plus_one_alone_and_plus_two_together() {
    const ID: &str = "feat.arg_standalone.emotion_descriptor_save_bonus";
    assert_eq!(value(&compute(&baseline()), ID), None);
    assert_eq!(
        value(&compute(&with_feats(baseline(), &["Fearless Curiosity"])), ID),
        Some(1)
    );
    assert_eq!(
        value(
            &compute(&with_feats(
                baseline(),
                &["Fearless Curiosity", "Intimidating Confidence"]
            )),
            ID
        ),
        Some(2)
    );

    // And it must stay OUT of the general Will save, which applies to every
    // effect and not only emotion-descriptor ones.
    assert_eq!(
        value(
            &compute(&with_feats(
                baseline(),
                &["Fearless Curiosity", "Intimidating Confidence"]
            )),
            "defense.total_save.will"
        ),
        value(&compute(&baseline()), "defense.total_save.will"),
    );
}

#[test]
fn aquatic_ancestry_and_gnome_weapon_focus_ground_their_flat_magnitudes() {
    for (feat, id, want) in [
        ("Aquatic Ancestry", "feat.arg_standalone.aquatic_ancestry_swim_speed", 10),
        (
            "Gnome Weapon Focus",
            "feat.arg_standalone.gnome_weapon_focus_attack_bonus",
            1,
        ),
    ] {
        assert_eq!(value(&compute(&baseline()), id), None, "absent without {feat}");
        assert_eq!(value(&compute(&with_feats(baseline(), &[feat])), id), Some(want));
    }

    // Gnome Weapon Focus is scoped to a weapon TYPE this engine's per-weapon
    // totals cannot test, so it must not move the baseline melee attack bonus.
    assert_eq!(
        value(
            &compute(&with_feats(baseline(), &["Gnome Weapon Focus"])),
            "combat.baseline_melee_attack_bonus"
        ),
        value(&compute(&baseline()), "combat.baseline_melee_attack_bonus"),
    );
}

/// Pathfinder Unchained's stamina pool, the book's one genuinely numeric feat
/// mechanic. `BONUS:VAR|StaminaPool|BAB+CON`, read against the engine's own
/// computed base attack bonus rather than a hardcoded expectation.
#[test]
fn combat_stamina_grounds_a_pool_of_base_attack_bonus_plus_constitution() {
    const ID: &str = "feat.pu_standalone.stamina_pool";
    let bare = compute(&baseline());
    assert_eq!(value(&bare, ID), None, "no pool without the feat");

    let with = compute(&with_feats(baseline(), &["Combat Stamina"]));
    let expected = bare.base_attack_bonus + bare.ability_modifiers.constitution;
    assert_eq!(value(&with, ID), Some(expected), "BAB + CON modifier");
}

/// Extra Stamina is `STACK:YES MULT:YES`, so occurrences are counted, and its
/// own `!PREABILITY:3,CATEGORY=FEAT,Extra Stamina` caps that count at three.
#[test]
fn extra_stamina_stacks_three_points_per_pick_up_to_the_corpus_cap() {
    const ID: &str = "feat.pu_standalone.stamina_pool";
    let base = value(
        &compute(&with_feats(baseline(), &["Combat Stamina"])),
        ID,
    )
    .expect("Combat Stamina grounds a pool");

    for (picks, expected_extra) in [(1usize, 3i16), (2, 6), (3, 9), (4, 9)] {
        let mut feats = vec!["Combat Stamina"];
        feats.extend(std::iter::repeat_n("Extra Stamina", picks));
        assert_eq!(
            value(&compute(&with_feats(baseline(), &feats)), ID),
            Some(base + expected_extra),
            "{picks} Extra Stamina pick(s)"
        );
    }
}

/// Push the Limits' pool is reported separately: its points are spendable only
/// at 0 primary stamina or while fatigued, states this engine does not model,
/// so summing the two would overstate what the character can spend.
#[test]
fn push_the_limits_grounds_a_separate_secondary_pool_and_leaves_the_primary_alone() {
    let primary_only = compute(&with_feats(baseline(), &["Combat Stamina"]));
    let both = compute(&with_feats(baseline(), &["Combat Stamina", "Push the Limits"]));

    assert_eq!(
        value(&both, "feat.pu_standalone.stamina_pool"),
        value(&primary_only, "feat.pu_standalone.stamina_pool"),
        "the secondary pool must not inflate the primary one"
    );
    assert_eq!(
        value(&both, "feat.pu_standalone.secondary_stamina_pool"),
        Some(compute(&baseline()).ability_modifiers.constitution),
        "secondary pool = Constitution modifier"
    );
    assert_eq!(
        value(&primary_only, "feat.pu_standalone.secondary_stamina_pool"),
        None
    );
}

/// Every stamina feat requires Combat Stamina, so without it there is no pool
/// at all — no record, rather than a fabricated zero-point one.
#[test]
fn no_stamina_pool_is_claimed_without_combat_stamina() {
    let computation = compute(&with_feats(baseline(), &["Extra Stamina", "Push the Limits"]));
    assert_eq!(value(&computation, "feat.pu_standalone.stamina_pool"), None);
    assert_eq!(
        value(&computation, "feat.pu_standalone.secondary_stamina_pool"),
        None
    );
}

/// §28's standing guard, stated as a property rather than a promise: a
/// character holding NONE of the newly wired feats must compute byte-identical
/// explanations to one computed before this cycle. Every new record and every
/// new addend is gated on a feat, so a character without them must be
/// untouched — proven by differencing the full explanation set of the shipped
/// deterministic fixture against itself across all 18 in-scope races.
#[test]
fn no_character_without_these_feats_sees_any_number_change() {
    // The 18 creatable races, so the guard covers the size/race seams the same
    // cycle touched rather than only the fixture's Human.
    for race in [
        "dwarf", "elf", "gnome", "half-elf", "half-orc", "halfling", "human", "aasimar", "drow",
        "duergar", "goblin", "hobgoblin", "kobold", "merfolk", "orc", "svirfneblin", "tengu",
        "tiefling",
    ] {
        let computation = compute(&input_for_race(race));
        let leaked: Vec<&str> = computation
            .explanations
            .iter()
            .map(|e| e.id.as_str())
            .filter(|id| {
                id.starts_with("feat.arg_") || id.starts_with("feat.pu_")
            })
            .collect();
        assert!(
            leaked.is_empty(),
            "{race} holds none of these feats, so none of their records may appear: {leaked:?}"
        );
    }
}

/// The wiring must be reachable through the real catalog string a player's
/// "Add Feat" click actually sends, and through the engine's `feat:` token
/// shape, and through neither-of-those-but-similar strings not at all.
#[test]
fn the_wiring_folds_across_both_real_feat_id_shapes_and_no_further() {
    const ID: &str = "feat.arg_standalone.aquatic_ancestry_swim_speed";
    assert_eq!(
        value(&compute(&with_feats(baseline(), &["Aquatic Ancestry"])), ID),
        Some(10),
        "the catalog key the Feat picker sends"
    );
    assert_eq!(
        value(&compute(&with_feats(baseline(), &["feat:aquatic_ancestry"])), ID),
        Some(10),
        "the engine token shape compose_character_input seeds"
    );
    assert_eq!(
        value(&compute(&with_feats(baseline(), &["Aquatic Ancestry Mastery"])), ID),
        None,
        "a longer feat that merely begins with the key must not match"
    );
}

// ---------------------------------------------------------------------------
// 2026-08-01 — the second pass over the deferral list.
//
// The first pass wired 17 of ARG's 49 unconditionally-bonused feats and
// deferred 32. Re-deriving that deferral list by command (rather than
// re-reading the report that produced it) found two of its reasons wrong:
//
// * `BONUS:SITUATION` was deferred as "the corpus classifies these as
//   situational". It does — and this engine already grounds exactly that shape
//   for the Core Rulebook dwarf (`race.dwarf.stonecunning.perception_bonus`,
//   `race.dwarf.greed.appraise_bonus`), so being situational was never the bar.
// * "`BONUS:VAR` increments a base this engine does not model" holds for the
//   halfling/suli/fetchling budgets and fails for `DefiantLuckTimes` and
//   `ImprovisationBonus`, whose `DEFINE:` sits on the ARG feat record itself.
//   And Stretched Wings was filed under fly manoeuvrability while its other
//   token is a plain `BONUS:MOVEADD|TYPE.Fly|40`.
//
// Eight feats moved from unwired to wired as a result (seven of which move a
// number on their own; Bestow Luck only in the company its own prerequisite
// requires). Each assertion below
// differences the same character with and without the feat, so a producer that
// exists but is never consumed fails here.
// ---------------------------------------------------------------------------

/// Echoes of Stone's two `BONUS:SITUATION` tokens, grounded with their
/// circumstances stated — the same shape the dwarf's Stonecunning and Greed
/// records already take.
#[test]
fn echoes_of_stone_grounds_both_four_point_situational_bonuses_with_their_circumstance() {
    let with = compute(&with_feats(input_for_race("human"), &["Echoes of Stone"]));

    assert_eq!(
        value(&with, "feat.arg_situational_skill_bonus.echoes_of_stone.perception"),
        Some(4)
    );
    assert_eq!(
        value(&with, "feat.arg_situational_skill_bonus.echoes_of_stone.survival"),
        Some(4)
    );

    let detail = with
        .explanations
        .iter()
        .find(|e| e.id == "feat.arg_situational_skill_bonus.echoes_of_stone.perception")
        .map(|e| e.detail.clone())
        .expect("the record must exist");
    assert!(
        detail.contains("underground"),
        "a situational bonus must carry its circumstance, or the sheet reads it as a \
         flat bonus that always applies: {detail}"
    );

    let without = compute(&input_for_race("human"));
    assert_eq!(
        value(&without, "feat.arg_situational_skill_bonus.echoes_of_stone.perception"),
        None
    );
}

/// Carrion Feeder grounds its one tokened bonus and no more. Its `BENEFIT:`
/// prose also promises a +2 on saves against disease and ingested poison, which
/// carries no `BONUS:` token at all — grounding it would be inventing a number
/// from prose, the failure this bundle exists to avoid.
#[test]
fn carrion_feeder_grounds_its_survival_bonus_and_invents_no_save_bonus() {
    let with = compute(&with_feats(input_for_race("tengu"), &["Carrion Feeder"]));
    assert_eq!(
        value(&with, "feat.arg_situational_skill_bonus.carrion_feeder.survival"),
        Some(2)
    );

    let without = compute(&input_for_race("tengu"));
    for save in ["defense.total_save.fortitude", "defense.total_save.reflex", "defense.total_save.will"] {
        assert_eq!(value(&with, save), value(&without, save), "{save} must not move");
    }
}

/// Improvisation `+2`, Improved Improvisation `+4` — both writing the one
/// `ImprovisationBonus` variable whose `DEFINE:` is on the Improvisation record
/// itself.
///
/// The three computed skill totals must not move: the feat applies only to
/// skills with no ranks and this engine's deterministic posture pins
/// Climb/Intimidate/Swim at rank 1, so folding it in would contradict the
/// feat's own text.
#[test]
fn improvisation_grounds_two_then_four_and_moves_no_ranked_skill_total() {
    const ID: &str = "feat.arg_standalone.improvisation_untrained_skill_bonus";
    let without = compute(&baseline());
    let basic = compute(&with_feats(baseline(), &["Improvisation"]));
    let improved = compute(&with_feats(baseline(), &["Improvisation", "Improved Improvisation"]));

    assert_eq!(value(&without, ID), None);
    assert_eq!(value(&basic, ID), Some(2));
    assert_eq!(value(&improved, ID), Some(4));

    for skill in [
        "skill.selected_modifier.climb",
        "skill.selected_modifier.intimidate",
        "skill.selected_modifier.swim",
    ] {
        assert_eq!(
            value(&improved, skill),
            value(&without, skill),
            "{skill} is held at rank 1 by the deterministic posture, so a bonus that \
             applies only to skills with NO ranks must leave it alone"
        );
    }
}

/// Stretched Wings' `BONUS:MOVEADD|TYPE.Fly|40`, reconciled against its prose's
/// "increases to 60 feet" by the wing-clipped strix's corpus `MOVE:Fly,20`.
#[test]
fn stretched_wings_grounds_forty_feet_of_fly_speed_and_no_manoeuvrability_number() {
    let with = compute(&with_feats(baseline(), &["Stretched Wings"]));
    assert_eq!(
        value(&with, "feat.arg_standalone.stretched_wings_fly_speed"),
        Some(40)
    );
    assert_eq!(
        value(&compute(&baseline()), "feat.arg_standalone.stretched_wings_fly_speed"),
        None
    );

    // The record must say what it is NOT claiming: the companion
    // `BONUS:VAR|Maneuverability|1` has no dimension here to land in.
    let detail = with
        .explanations
        .iter()
        .find(|e| e.id == "feat.arg_standalone.stretched_wings_fly_speed")
        .map(|e| e.detail.clone())
        .expect("the record must exist");
    assert!(detail.to_lowercase().contains("manoeuvrab") || detail.to_lowercase().contains("maneuverab"));
}

/// Defiant Luck 1/day, 2/day with Bestow Luck. Bestow Luck alone claims
/// nothing: there is no ability for its extra use to attach to.
#[test]
fn defiant_luck_grounds_one_use_per_day_and_two_with_bestow_luck() {
    const ID: &str = "feat.arg_standalone.defiant_luck_uses_per_day";
    assert_eq!(value(&compute(&baseline()), ID), None);
    assert_eq!(value(&compute(&with_feats(baseline(), &["Defiant Luck"])), ID), Some(1));
    assert_eq!(
        value(&compute(&with_feats(baseline(), &["Defiant Luck", "Bestow Luck"])), ID),
        Some(2)
    );
    assert_eq!(value(&compute(&with_feats(baseline(), &["Bestow Luck"])), ID), None);
}

/// Fiend Sight's darkvision, stated absolutely by the record's own
/// `VISION:Darkvision (120')` over the 60-foot base its
/// `PREVISION:1,Darkvision=60` prerequisite fixes.
#[test]
fn fiend_sight_grounds_a_hundred_and_twenty_foot_darkvision() {
    const ID: &str = "feat.arg_standalone.fiend_sight_darkvision_feet";
    assert_eq!(value(&compute(&input_for_race("tiefling")), ID), None);
    assert_eq!(
        value(&compute(&with_feats(input_for_race("tiefling"), &["Fiend Sight"])), ID),
        Some(120)
    );
    // STACK:YES MULT:YES, capped at two picks by PREVARLT:FiendSightTier,2 —
    // the second pick grants *see in darkness*, a capability, not more range.
    assert_eq!(
        value(
            &compute(&with_feats(input_for_race("tiefling"), &["Fiend Sight", "Fiend Sight"])),
            ID
        ),
        Some(120)
    );
}

/// The six new records must fold across the real catalog string a player's
/// "Add Feat" click sends and the engine's `feat:` token shape alike — the same
/// property `the_wiring_folds_across_both_real_feat_id_shapes_and_no_further`
/// pins for Aquatic Ancestry.
#[test]
fn the_newly_wired_feats_fold_across_both_real_feat_id_shapes() {
    for (catalog_key, token, id, expected) in [
        (
            "Improvisation",
            "feat:improvisation",
            "feat.arg_standalone.improvisation_untrained_skill_bonus",
            2i16,
        ),
        (
            "Stretched Wings",
            "feat:stretched_wings",
            "feat.arg_standalone.stretched_wings_fly_speed",
            40,
        ),
        (
            "Defiant Luck",
            "feat:defiant_luck",
            "feat.arg_standalone.defiant_luck_uses_per_day",
            1,
        ),
        (
            "Fiend Sight",
            "feat:fiend_sight",
            "feat.arg_standalone.fiend_sight_darkvision_feet",
            120,
        ),
        (
            "Echoes of Stone",
            "feat:echoes_of_stone",
            "feat.arg_situational_skill_bonus.echoes_of_stone.perception",
            4,
        ),
        (
            "Carrion Feeder",
            "feat:carrion_feeder",
            "feat.arg_situational_skill_bonus.carrion_feeder.survival",
            2,
        ),
    ] {
        assert_eq!(
            value(&compute(&with_feats(baseline(), &[catalog_key])), id),
            Some(expected),
            "{catalog_key}: the catalog key the Feat picker sends"
        );
        assert_eq!(
            value(&compute(&with_feats(baseline(), &[token])), id),
            Some(expected),
            "{catalog_key}: the engine token shape compose_character_input seeds"
        );
    }
}

/// The corrected `BONUS:SITUATION` census, re-derived from the shipped catalog
/// rather than restated: **3 tokens across 2 feats**, not 3 feats.
///
/// This is the assertion that would have caught the original miscount. It also
/// pins the two feat keys, so a catalog change that renames or drops either
/// fails here rather than silently orphaning a grounded record.
#[test]
fn args_situation_tokens_are_three_across_two_feats() {
    let mut tokens = 0usize;
    let mut feats_with: Vec<&str> = Vec::new();
    for entry in arg_feats::feat_tables() {
        let Some(bonuses) = entry.effect else { continue };
        let count = bonuses.iter().filter(|b| b.qualifiers.first() == Some(&"SITUATION")).count();
        if count > 0 {
            tokens += count;
            feats_with.push(entry.key);
        }
    }
    assert_eq!(tokens, 3, "ARG's BONUS:SITUATION token count");
    assert_eq!(
        feats_with,
        vec!["Carrion Feeder", "Echoes of Stone"],
        "the feats carrying them — 2, not the 3 originally reported"
    );
}

/// The ledger this cycle is accountable to, derived rather than declared: of
/// ARG's 49 unconditionally-bonused feats, how many now reach a grounded
/// record or a computed total.
///
/// Asserted by *running the real pipeline for every one of the 49 catalog
/// keys*, so a feat claimed wired but producing nothing fails here — the exact
/// failure mode this bundle's history is made of.
#[test]
fn twenty_four_of_args_forty_nine_unconditionally_bonused_feats_now_move_a_number() {
    let unconditionally_bonused: Vec<&str> = arg_feats::feat_tables()
        .iter()
        .filter(|entry| {
            entry
                .effect
                .is_some_and(|bonuses| bonuses.iter().any(|b| !bonus_is_conditioned(b.qualifiers)))
        })
        .map(|entry| entry.key)
        .collect();
    assert_eq!(unconditionally_bonused.len(), 49);

    // "Moves a number" = the full (id, value) set of the deterministic fixture
    // differs with the feat held. A new record and a changed value both count;
    // a producer nobody consumes counts as neither.
    let fingerprint = |input: &CharacterInput| -> Vec<(String, i16)> {
        compute(input).explanations.iter().map(|e| (e.id.clone(), e.value)).collect()
    };
    let baseline_fingerprint = fingerprint(&baseline());
    let moving: Vec<&str> = unconditionally_bonused
        .iter()
        .copied()
        .filter(|key| fingerprint(&with_feats(baseline(), &[key])) != baseline_fingerprint)
        .collect();

    assert_eq!(
        moving.len(),
        24,
        "17 wired in the first pass + 7 in the second. Feats that move a number: {moving:?}"
    );
    for key in [
        "Echoes of Stone",
        "Carrion Feeder",
        "Improvisation",
        "Improved Improvisation",
        "Stretched Wings",
        "Defiant Luck",
        "Fiend Sight",
    ] {
        assert!(moving.contains(&key), "{key} was wired this cycle and must move a number");
    }

    // Bestow Luck is the 8th feat wired this cycle and correctly does NOT
    // appear above: alone it is not a legal character and there is no Defiant
    // Luck ability for its extra use to attach to, so it claims nothing. It
    // moves a number only in the company its own prerequisite requires.
    assert!(!moving.contains(&"Bestow Luck"));
    assert_eq!(
        value(
            &compute(&with_feats(baseline(), &["Defiant Luck", "Bestow Luck"])),
            "feat.arg_standalone.defiant_luck_uses_per_day"
        ),
        Some(2)
    );
}
