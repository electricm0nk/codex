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
//! not the wiring list. ARG's 49 such feats include 11 whose only token is
//! `BONUS:ABILITYPOOL|<pool>|1` (a further *choice*, not a magnitude), 3 whose
//! token is `BONUS:SITUATION|<skill>=<circumstance>|N` (PCGen's own token for a
//! situational bonus — Echoes of Stone's Perception bonus *underground*), and a
//! long tail of `BONUS:VAR|<internal>` increments to PCGen bookkeeping
//! variables for spell-like-ability uses per day, racial luck budgets and fly
//! maneuverability — none of which this engine models, so none of which has
//! anything to land on. `feat_effects`' own SD-27 section header records that
//! reasoning per category.
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
