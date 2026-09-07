//! SD13-E5 Cleric domain powers grounding proof (Good: Touch of Good, Healing: Rebuke
//! Death), narrowing the `class_feature.cleric.domain_powers.unsupported` blocker.
//!
//! Independently verified against the PF1 Core Rulebook Good Domain and Healing Domain
//! rule text (d20pfsrd, cross-checked by a second web search) before writing any code,
//! per the loop's standing practice of checking primary sources rather than trusting an
//! existing blocker-message claim or memory:
//!
//! - Good domain granted power Touch of Good (Sp): standard action, touch a creature to
//!   grant a sacred bonus on attack rolls, skill checks, ability checks, and saving
//!   throws equal to half the cleric's level (minimum 1) for 1 round; usable 3 + Wisdom
//!   modifier times per day. Both the bonus magnitude and the uses-per-day count are
//!   flat, non-dice formulas, so BOTH ground for real this slice.
//! - Healing domain granted power Rebuke Death (Sp): standard action, touch a living
//!   creature below 0 hit points to heal it 1d4 points of damage plus 1 for every two
//!   cleric levels; usable 3 + Wisdom modifier times per day. The heal amount is NOT a
//!   flat number (a 1d4 dice roll, plus a hit-point-state gating check on the target),
//!   so this slice deliberately grounds ONLY the flat uses-per-day count and leaves the
//!   heal amount itself named-but-unproven — the existing blocker message's claim that
//!   this pre-existing "3 + Wisdom modifier" uses-per-day rate is shared by both powers
//!   turned out to be correct on independent verification, but the heal-amount contents
//!   were never claimed as flat and are not grounded here.
//!
//! No touch-attack resolution, no healing-application engine, no hit-point-state gating
//! engine, and no per-use consumption tracking is implemented for either power. Domain
//! spell-list contents and the prepared divine spell posture burden remain untouched and
//! out of scope for this slice.

use codex::rules_core::pilot_compute::{
    ComputationDiagnostic,
    PilotBaseChassisComputation,
    compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{SupportState, seeded_current_truth};
mod common;
use common::{load, explanation, has_explanation};

const CLERIC_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_cleric_level1_sd13_deterministic_input.txt");

const TOUCH_OF_GOOD_BONUS_ID: &str = "class_feature.domain.good_touch_of_good_bonus";
const TOUCH_OF_GOOD_USES_PER_DAY_ID: &str =
    "class_feature.domain.good_touch_of_good_uses_per_day";
const REBUKE_DEATH_USES_PER_DAY_ID: &str =
    "class_chassis.cleric.domain_power_healing_rebuke_death_uses_per_day";

fn claim_blocking<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationDiagnostic {
    let diag = computation
        .diagnostics
        .iter()
        .find(|d| d.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected diagnostic id '{id}', got {:?}",
                computation.diagnostics
            )
        });
    assert!(
        diag.claim_blocking,
        "diagnostic '{id}' must be claim-blocking: {diag:?}"
    );
    diag
}

// ----- Grounded for real: Touch of Good (Good domain) magnitude and uses per day -----

#[test]
fn cleric_level1_grounds_touch_of_good_bonus_and_uses_per_day() {
    let input = load(CLERIC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Good Domain: sacred bonus = half cleric level, minimum 1.
    // At level 1: max(1 / 2, 1) = 1.
    let bonus = explanation(&computation, TOUCH_OF_GOOD_BONUS_ID);
    assert_eq!(
        bonus.value, 1,
        "cleric level 1 Touch of Good must ground a +1 sacred bonus (half level, minimum 1)"
    );
    assert!(
        bonus.detail.contains("Touch of Good") && bonus.detail.contains("sacred"),
        "touch of good bonus explanation must name Touch of Good and the sacred bonus: {}",
        bonus.detail
    );
    assert!(
        bonus.detail.contains("touch-attack"),
        "touch of good bonus explanation must disclaim touch-attack resolution: {}",
        bonus.detail
    );

    // PF1 Core Rulebook Good Domain: usable 3 + Wisdom modifier times per day.
    // Fixture Wisdom is 17 + 2 Human racial (CG-03 fix) -> modifier +4 -> 3 + 4 = 7.
    let uses = explanation(&computation, TOUCH_OF_GOOD_USES_PER_DAY_ID);
    assert_eq!(
        uses.value, 7,
        "cleric level 1 with WIS 17 (+4) must ground 3 + 4 = 7 Touch of Good uses per day"
    );
    assert!(
        uses.detail.contains("Wisdom") && uses.detail.contains("Touch of Good"),
        "touch of good uses-per-day explanation must name Wisdom and Touch of Good: {}",
        uses.detail
    );

    assert_eq!(computation.ability_modifiers.wisdom, 4);
}

// ----- Grounded for real (uses/day only): Rebuke Death (Healing domain) -----

#[test]
fn cleric_level1_grounds_rebuke_death_uses_per_day_but_not_heal_amount() {
    let input = load(CLERIC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Healing Domain: usable 3 + Wisdom modifier times per day.
    // CG-03 fix: Wisdom modifier is now +4 (base 17 + 2 Human racial), not +3.
    let uses = explanation(&computation, REBUKE_DEATH_USES_PER_DAY_ID);
    assert_eq!(
        uses.value, 7,
        "cleric level 1 with WIS 17 (+4) must ground 3 + 4 = 7 Rebuke Death uses per day"
    );
    assert!(
        uses.detail.contains("Wisdom") && uses.detail.contains("Rebuke Death"),
        "rebuke death uses-per-day explanation must name Wisdom and Rebuke Death: {}",
        uses.detail
    );
    assert!(
        uses.detail.contains("1d4") && uses.detail.contains("not a flat number"),
        "rebuke death uses-per-day explanation must disclaim the non-flat heal amount: {}",
        uses.detail
    );

    // The heal amount itself (1d4 + level scaling, gated on target hp) is never
    // fabricated as a flat number under any id. Note: "healing" (the domain name)
    // legitimately appears in the grounded uses-per-day id, so this checks
    // specifically for a heal-amount-shaped id, not any substring of "heal".
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.contains("rebuke_death") && e.id.contains("heal_amount")),
        "no rebuke death heal-amount explanation may exist: {:?}",
        computation.explanations
    );
}

// ----- Grounding the domain powers must not fabricate spell-list contents -----

#[test]
fn cleric_level1_domain_powers_ground_no_spell_list_or_prepared_posture() {
    let input = load(CLERIC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, "class_feature.cleric.domain_spells"),
        "grounding domain powers must not fabricate domain spell-list contents"
    );
    assert!(
        !has_explanation(&computation, "class_spell.cleric.prepared_divine"),
        "grounding domain powers must not fabricate the prepared divine spell posture"
    );

    // (v0.6 alpha swarm, risks item 8, Good domain closure) the old flat
    // "class_feature.cleric.domain_powers.unsupported" diagnostic no longer
    // fires for this Good+Healing fixture -- Good domain's Touch of Good can now genuinely
    // close (self-application only), so the domain-powers burden is split:
    // Rebuke Death's heal amount stays claim-blocking under its own narrowed
    // id (Healing is chosen on this fixture), and the domain spell-list
    // contents gap is a separate, non-blocking note.
    let rebuke_death = claim_blocking(
        &computation,
        "class_feature.cleric.healing_domain.rebuke_death.unsupported",
    );
    assert!(
        rebuke_death.message.contains("Touch of Good") && rebuke_death.message.contains("Rebuke Death"),
        "rebuke death blocker must contrast against the closed Touch of Good burden: {}",
        rebuke_death.message
    );
    assert!(
        rebuke_death.message.contains("heal amount"),
        "rebuke death blocker must name the still-unproven heal amount: {}",
        rebuke_death.message
    );

    let spell_list_note = computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_feature.cleric.domain_spell_list_contents.unmodeled")
        .expect("the domain spell-list-contents note must still fire");
    assert!(
        !spell_list_note.claim_blocking,
        "the domain spell-list-contents note must not block an otherwise-valid Good-domain posture"
    );
    assert!(
        spell_list_note.message.contains("spell-list"),
        "the domain spell-list-contents note must name the unproven spell-list contents: {}",
        spell_list_note.message
    );

    // (v0.6 alpha swarm, risks item 8) Only ONE cleric-specific claim-blocking
    // diagnostic remains here: this fixture has zero prepared spells selected, a
    // genuinely valid prepared-divine posture, so
    // class_spell.cleric.prepared_divine.unsupported correctly no longer fires,
    // and Touch of Good (Good domain) can now genuinely close -- only Rebuke
    // Death (Healing domain) stays claim-blocking.
    let distinct_blocking = computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking && d.id.starts_with("class_") && d.id.contains("cleric"))
        .count();
    assert_eq!(
        distinct_blocking, 1,
        "cleric must leave exactly one class-specific claim-blocking diagnostic (domain powers) \
         on a valid prepared-spell posture: {:?}",
        computation.diagnostics
    );
    let prepared_count = computation
        .explanations
        .iter()
        .find(|e| e.id == "class_spell.cleric.daily_preparation")
        .map(|e| e.value)
        .unwrap_or(-1);
    assert_eq!(
        prepared_count, 0,
        "no spells are fabricated merely because the prepared-divine blocker stopped firing: {:?}",
        computation.diagnostics
    );
}

// ----- Negative controls: absent domain selections do not fabricate the other domain's power -----

#[test]
fn cleric_level1_without_good_domain_selection_does_not_fabricate_touch_of_good() {
    let stripped = CLERIC_FIXTURE.replace("choice=choice:cleric_domain:domain:good\n", "");
    let input = load(&stripped);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, TOUCH_OF_GOOD_BONUS_ID),
        "an absent Good domain selection must not fabricate the Touch of Good bonus"
    );
    assert!(
        !has_explanation(&computation, TOUCH_OF_GOOD_USES_PER_DAY_ID),
        "an absent Good domain selection must not fabricate Touch of Good uses per day"
    );
    // Healing is still selected, so Rebuke Death uses per day still grounds.
    assert!(
        has_explanation(&computation, REBUKE_DEATH_USES_PER_DAY_ID),
        "Rebuke Death uses per day must still ground when only Healing is selected"
    );
}

#[test]
fn cleric_level1_without_healing_domain_selection_does_not_fabricate_rebuke_death() {
    let stripped = CLERIC_FIXTURE.replace("choice=choice:cleric_domain:domain:healing\n", "");
    let input = load(&stripped);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, REBUKE_DEATH_USES_PER_DAY_ID),
        "an absent Healing domain selection must not fabricate Rebuke Death uses per day"
    );
    // Good is still selected, so Touch of Good still grounds.
    assert!(
        has_explanation(&computation, TOUCH_OF_GOOD_BONUS_ID),
        "Touch of Good bonus must still ground when only Good is selected"
    );
    assert!(
        has_explanation(&computation, TOUCH_OF_GOOD_USES_PER_DAY_ID),
        "Touch of Good uses per day must still ground when only Good is selected"
    );
}

// ----- Control plane: the matrix row stays Partial and names the narrowed burden -----

#[test]
fn matrix_cleric_row_names_touch_of_good_grounded_and_rebuke_death_heal_amount_unproven() {
    let matrix = seeded_current_truth();
    let cleric = matrix
        .row("class.cleric.progression_and_spell_burden")
        .expect("cleric row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class Progression
    // Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(cleric.support_state, SupportState::Supported);
    assert!(
        cleric.blocker_or_lossiness_note.contains("Touch of Good")
            && cleric.blocker_or_lossiness_note.contains("Rebuke Death"),
        "cleric blocker note must still name both granted powers: {}",
        cleric.blocker_or_lossiness_note
    );
    assert!(
        cleric.blocker_or_lossiness_note.contains("heal amount"),
        "cleric blocker note must name the still-unproven Rebuke Death heal amount: {}",
        cleric.blocker_or_lossiness_note
    );
}
