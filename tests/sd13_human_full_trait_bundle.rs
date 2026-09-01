//! SD13-E6-F3a Human racial trait bundle proof.
//!
//! Proves the second truthful SD13-F3a slice: the live rules-core surface
//! classifies the remaining PF1 Standard Human racial trait burden (size,
//! speed, senses, extra skill ranks) as four explicit `ComputationExplanation`
//! records on the deterministic pilot seam. Each dimension is named on the
//! runtime path rather than left as an incidental side-effect or a folklore
//! claim, and three of the four carry the grounded PF1 source value as a
//! recognition record. The senses dimension is classified explicitly as a
//! bounded no-effect record because PF1 Standard Human grants no special
//! senses.
//!
//! It is intentionally not a Human racial trait engine. It grounds no
//! numeric contribution to attack rolls, AC, skill checks, ability checks,
//! base speed, or any other chassis output. It also grounds no alternate
//! Human racial trait, no variant Human, no half-Human heritage, no other
//! race, and no PF1 alternate ruleset. It preserves the accepted Human
//! Fighter 1-3 truth, the Human ability-bonus and bonus-feat race seams,
//! the hybrid Paladin/Ranger baseline, and the Sorcerer spell baseline
//! recognition records.

use codex::rules_core::pilot_compute::{
    HeadlessReceiptStatus,
    build_pilot_headless_receipt,
    compute_pilot_base_chassis,
};
mod common;
use common::{load, explanation, has_explanation};

const FIGHTER_LEVEL_1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");
const FIGHTER_LEVEL_2_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level2_sd13_deterministic_input.txt");
const FIGHTER_LEVEL_3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level3_sd13_deterministic_input.txt");
const PALADIN_LEVEL_1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt");
const RANGER_LEVEL_1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt");
const SORCERER_LEVEL_1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt");

// ----- The four Human trait bundle records exist on every Human pilot posture -----

const ALL_HUMAN_FIXTURES: &[&str] = &[
    FIGHTER_LEVEL_1_FIXTURE,
    FIGHTER_LEVEL_2_FIXTURE,
    FIGHTER_LEVEL_3_FIXTURE,
    PALADIN_LEVEL_1_FIXTURE,
    RANGER_LEVEL_1_FIXTURE,
    SORCERER_LEVEL_1_FIXTURE,
];

const BUNDLE_IDS: &[&str] = &[
    "race.human.trait_bundle.size",
    "race.human.trait_bundle.speed",
    "race.human.trait_bundle.senses",
    "race.human.trait_bundle.extra_skill_ranks",
];

#[test]
fn every_human_posture_surfaces_all_four_trait_bundle_records() {
    for fixture in ALL_HUMAN_FIXTURES {
        let input = load(fixture);
        let computation = compute_pilot_base_chassis(&input);

        for id in BUNDLE_IDS {
            assert!(
                has_explanation(&computation, id),
                "fixture {fixture} (race:human) must surface Human trait bundle record '{id}', \
                 got explanations {:?}",
                computation.explanations
            );
        }
    }
}

// ----- size: grounded PF1 Standard Human size category, no fabricated value -----

#[test]
fn human_size_trait_bundle_record_names_medium_category_and_carries_no_value() {
    let input = load(FIGHTER_LEVEL_1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let size = explanation(&computation, "race.human.trait_bundle.size");
    assert!(
        size.detail.contains("Medium"),
        "Human size trait bundle record must name the PF1 Standard Human Medium size category: {}",
        size.detail
    );
    assert!(
        size.detail.contains("SIZE:MEDIUM"),
        "Human size trait bundle record must reference the cr_races.lst source: {}",
        size.detail
    );
    // Recognition record only: contributes no numeric mechanical effect to any output.
    assert_eq!(
        size.value, 0,
        "Human size trait bundle record must carry no fabricated mechanical value (+0)"
    );
}

// ----- speed: grounded 30 ft base land speed, recognition value, no propagation -----

#[test]
fn human_speed_trait_bundle_record_names_30_ft_base_and_carries_recognition_value() {
    let input = load(FIGHTER_LEVEL_1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let speed = explanation(&computation, "race.human.trait_bundle.speed");
    assert!(
        speed.detail.contains("30 ft"),
        "Human speed trait bundle record must name the 30 ft base land speed: {}",
        speed.detail
    );
    assert!(
        speed.detail.contains("GAIT:WALK|30"),
        "Human speed trait bundle record must reference the cr_races.lst GAIT source: {}",
        speed.detail
    );
    // The grounded PF1 base-speed identity (30) is the recognition value;
    // it is identity-only and propagates to no computed output.
    assert_eq!(
        speed.value, 30,
        "Human speed trait bundle record value must carry the grounded 30 ft recognition value"
    );
}

// ----- senses: bounded no-effect classification, no fabricated sense bonus -----

#[test]
fn human_senses_trait_bundle_record_classifies_as_bounded_no_effect() {
    let input = load(FIGHTER_LEVEL_1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let senses = explanation(&computation, "race.human.trait_bundle.senses");
    assert!(
        senses.detail.contains("no special senses"),
        "Human senses trait bundle record must classify Standard Human as having no special senses: {}",
        senses.detail
    );
    // Bounded no-effect classification: the recognition value is +0.
    assert_eq!(
        senses.value, 0,
        "Human senses trait bundle record must carry the bounded no-effect value (+0)"
    );
    // Must explicitly name the sense bonuses Human does NOT have, rather than
    // fabricating one or omitting the dimension entirely.
    assert!(
        senses.detail.contains("darkvision"),
        "Human senses trait bundle record must name darkvision as one of the senses Human does NOT have: {}",
        senses.detail
    );
}

// ----- extra skill ranks: grounded +1/level rule, propagated only as recognition -----

#[test]
fn human_extra_skill_ranks_record_names_both_at_level_1_and_per_level_grants() {
    let input = load(FIGHTER_LEVEL_1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let extra = explanation(&computation, "race.human.trait_bundle.extra_skill_ranks");
    // PF1 Standard Human: 4 extra skill points at 1st level.
    assert!(
        extra.detail.contains("4 extra skill points at 1st level"),
        "Human extra skill ranks record must name the 4 extra-skill-points-at-1st-level grant: {}",
        extra.detail
    );
    // PF1 Standard Human: 1 extra skill rank per additional level.
    assert!(
        extra
            .detail
            .contains("1 extra skill rank per additional level"),
        "Human extra skill ranks record must name the per-additional-level grant: {}",
        extra.detail
    );
    assert!(
        extra.detail.contains("BONUS:SKILL|"),
        "Human extra skill ranks record must reference the cr_races.lst BONUS:SKILL source: {}",
        extra.detail
    );
    // Recognition value carries the per-additional-level +1 identity only;
    // the bounded selected-skill computation remains grounded by the rank-1 posture.
    assert_eq!(
        extra.value, 1,
        "Human extra skill ranks record must carry the grounded +1/level recognition value"
    );
}

// ----- The bounded note no longer claims size/speed/senses/extra skill ranks are unverified -----

#[test]
fn human_bounded_semantics_note_is_updated_to_reflect_classified_trait_bundle() {
    let input = load(FIGHTER_LEVEL_1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bounded = computation
        .diagnostics
        .iter()
        .find(|d| d.id == "race.human.bounded_semantics")
        .unwrap_or_else(|| {
            panic!(
                "expected diagnostic id 'race.human.bounded_semantics', got {:?}",
                computation.diagnostics
            )
        });

    assert!(
        !bounded.claim_blocking,
        "race.human.bounded_semantics must remain non-claim-blocking: {bounded:?}"
    );
    // The note must now reflect the classified trait bundle, not the original
    // "remain unverified" framing.
    assert!(
        bounded.message.contains("classified explicitly"),
        "race.human.bounded_semantics must reference the classified trait bundle: {}",
        bounded.message
    );
    // And it must NOT still carry the pre-F3a framing. These two tokens appear
    // only in the old message ("grounded only for ..."; "...extra skill ranks,
    // and the remaining racial trait burden remain unverified") and never in the
    // new one, whose remaining-surface sentence legitimately ends with the
    // singular "remains unverified".
    for token in ["grounded only for", "racial trait burden remain unverified"] {
        assert!(
            !bounded.message.contains(token),
            "race.human.bounded_semantics must no longer carry the pre-F3a '{token}' framing: {}",
            bounded.message
        );
    }
}

// ----- The trait bundle records are NOT pushed onto non-Human races -----

#[test]
fn non_human_race_does_not_surface_human_trait_bundle_records() {
    // Replace `race:human` with a non-Human race in the canonical Fighter level-1
    // fixture. The bounded selected-skill and combat posture will still fail
    // because the fixture names Human-specific choices, but the trait bundle
    // records must not leak onto the non-Human input.
    let non_human = FIGHTER_LEVEL_1_FIXTURE.replace("race_id=race:human", "race_id=race:dwarf");
    // Strip the Human ability-bonus / bonus-feat choice selections (which
    // would otherwise be invalid for a non-Human race) so the fixture still
    // parses cleanly. The bounded chassis will still claim-block via the
    // non-Human race path; that is acceptable for this negative control.
    let non_human = non_human
        .replace("choice=choice:human_ability_bonus:ability:strength", "")
        .replace("choice=choice:human_bonus_feat:feat:dodge", "");
    let input = load(&non_human);
    let computation = compute_pilot_base_chassis(&input);

    for id in BUNDLE_IDS {
        assert!(
            !has_explanation(&computation, id),
            "non-Human race input must not surface Human trait bundle record '{id}', \
             got explanations {:?}",
            computation.explanations
        );
    }
}

// ----- The trait bundle records propagate onto the Fighter level-2/3 milestones too -----

#[test]
fn fighter_level_2_and_level_3_surfaces_all_four_trait_bundle_records() {
    for fixture in [FIGHTER_LEVEL_2_FIXTURE, FIGHTER_LEVEL_3_FIXTURE] {
        let input = load(fixture);
        let computation = compute_pilot_base_chassis(&input);

        for id in BUNDLE_IDS {
            assert!(
                has_explanation(&computation, id),
                "Fighter level-2/3 (race:human) must surface trait bundle record '{id}', \
                 got explanations {:?}",
                computation.explanations
            );
        }
    }
}

// ----- The Human pilot receipts stay computed (no claim-blocking introduced) -----

#[test]
fn human_fighter_level_1_pilot_receipt_stays_computed_after_classifying_trait_bundle() {
    let input = load(FIGHTER_LEVEL_1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);
    let receipt = build_pilot_headless_receipt(&input);

    // The bounded trait bundle is non-blocking by design; the integrated
    // deterministic pilot must stay computed when the named Human Fighter
    // level-1 posture holds.
    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "Human Fighter level-1 posture must stay free of claim-blocking diagnostics after \
         the trait bundle is classified; got diagnostics {:?}",
        computation.diagnostics
    );
    assert_eq!(receipt.status, HeadlessReceiptStatus::Computed);
}
