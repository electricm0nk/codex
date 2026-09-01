//! SD13-E3/E5 Monk level-1 martial chassis baseline proof.
//!
//! Proves the SD13-E3 monk slice (mirroring the Barbarian level-1 martial-baseline
//! pattern) plus the SD13-E5 unarmed-strike/flurry grounding slice: the live
//! rules-core surface ingests a deterministic Human `class:monk:1` input, leaves
//! direct computed evidence that acknowledges the bounded level-1 martial chassis
//! identity rather than treating it as an undocumented packet placeholder, and now
//! grounds four pillar burdens directly (base-attack progression, base-save
//! progression, AC Bonus / Wisdom-to-AC, and the unarmed-strike damage die plus
//! the Flurry of Blows flat attack surface), while staying explicitly
//! claim-blocked on the one still-missing burden (the level-1 bonus feat grant).
//! It also pins the matrix reclassification of the monk row from `Unverified` /
//! `Observed` to `Partial` / `Computed`.
//!
//! It is intentionally not a martial class engine. It grounds the Monk 3/4-BAB
//! progression (`classlevel * 3 / 4`), the all-three-good base-save progression
//! (`classlevel / 2 + 2` for Fortitude, Reflex, and Will), the flat level-1
//! Wisdom-to-AC value (Wisdom modifier, if positive, added to AC), the Medium-monk
//! level-1 unarmed strike damage die size (1d6 — die size only, no damage roll or
//! damage total), and the level-1 Flurry of Blows flat surface (two attacks, each
//! at monk level - 2 = -1 before ability modifiers), but it grounds no
//! attack-resolution or damage-roll engine, no monk-weapon flurry, no level-4+
//! unarmed damage die progression, no level-1 bonus feat grant from the
//! restricted Monk feat list, no ki pool, no level-4+ AC Bonus dodge-bonus
//! progression, no "unarmored and unencumbered" runtime state-check engine, and
//! no level-2+ martial progression. It also preserves the accepted
//! Fighter 1-3 truth, the Rogue blocked negative control, the Barbarian
//! partial/computed truth, the Paladin/Ranger blocked hybrid negative controls, and
//! the Human race/interaction truth.

use codex::rules_core::pilot_compute::{
    ComputationDiagnostic,
    HeadlessReceiptStatus,
    PilotBaseChassisComputation,
    build_pilot_headless_receipt,
    compute_pilot_base_chassis,
};
use codex::rules_core::pilot_failure::PrimaryOwner;
use codex::rules_core::pilot_view_model::PilotViewModel;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const MONK_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_monk_level1_sd13_deterministic_input.txt");

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

fn has_diagnostic(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.diagnostics.iter().any(|d| d.id == id)
}

// ----- Direct runtime evidence: the martial chassis identity is acknowledged -----

#[test]
fn monk_level1_leaves_direct_chassis_recognition_evidence() {
    let input = load(MONK_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let chassis = explanation(&computation, "class_chassis.monk.bounded_progression");
    assert!(
        chassis.detail.contains("class:monk") && chassis.detail.contains("level 1"),
        "monk chassis recognition must name the class:monk:1 identity: {}",
        chassis.detail
    );
    // Monk level 1 base attack bonus is genuinely 0 (3/4 BAB: 1*3/4 = 0),
    // so this assertion reads the same before and after the chassis
    // widening -- but it now means "the real computed value", not "no
    // value was fabricated".
    assert_eq!(computation.base_attack_bonus, 0, "monk level 1 base attack bonus (3/4 BAB)");

    // Updated 2026-07-29 (v0.6 alpha swarm, Monk/Summoner
    // chassis-recognition closure). This previously asserted the ABSENCE
    // of `class_chassis.base_attack_bonus`, because `table_class_id` did
    // not map `class:monk` and Monk therefore never reached the
    // table-driven chassis path at all. It does now, reading the
    // corpus-backed `class_tables()` Monk row that was always present, so
    // the explanation must be there -- and asserting its absence would now
    // be asserting a bug. The comment on the old assertion called it "a
    // supported Fighter base-attack chassis explanation"; that was
    // misleading even then -- the id is the generic table-driven one every
    // recognized class emits, not a Fighter-specific one.
    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(bab.value, 0, "monk level 1 base attack bonus explanation must carry the real 0");
    assert!(
        bab.detail.contains("class:monk"),
        "the chassis explanation must name the real class it read: {}",
        bab.detail
    );

    // Ability modifiers remain class-independent and still compute (WIS 17 -> +3).
    assert_eq!(computation.ability_modifiers.wisdom, 4);
}

// ----- Grounded: base-attack, base-save, and AC Bonus pillar burdens -----

#[test]
fn monk_level1_grounds_base_attack_bonus() {
    let input = load(MONK_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.monk.base_attack_bonus");
    assert_eq!(
        base_attack.value, 0,
        "monk level 1 base attack bonus is 3/4-BAB: 1 * 3 / 4 = 0"
    );
    assert!(
        base_attack.detail.contains("3/4-BAB"),
        "monk base-attack explanation must name the 3/4-BAB progression: {}",
        base_attack.detail
    );

    assert!(
        !has_diagnostic(
            &computation,
            "class_feature.monk.bounded_progression.base_attack.unsupported"
        ),
        "the old base-attack blocker diagnostic must no longer be emitted: {:?}",
        computation.diagnostics
    );
}

#[test]
fn monk_level1_grounds_base_save_progression() {
    let input = load(MONK_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.monk.base_save.fortitude");
    let reflex = explanation(&computation, "class_chassis.monk.base_save.reflex");
    let will = explanation(&computation, "class_chassis.monk.base_save.will");
    assert_eq!(fortitude.value, 2, "monk level 1 good Fortitude: 1/2+2 = 2");
    assert_eq!(reflex.value, 2, "monk level 1 good Reflex: 1/2+2 = 2");
    assert_eq!(will.value, 2, "monk level 1 good Will: 1/2+2 = 2");
    assert!(
        fortitude.detail.contains("good") && reflex.detail.contains("good") && will.detail.contains("good"),
        "monk base-save explanations must call out that all three saves are good: {:?} {:?} {:?}",
        fortitude.detail,
        reflex.detail,
        will.detail
    );

    assert!(
        !has_diagnostic(
            &computation,
            "class_feature.monk.bounded_progression.base_save.unsupported"
        ),
        "the old base-save blocker diagnostic must no longer be emitted: {:?}",
        computation.diagnostics
    );
}

#[test]
fn monk_level1_grounds_ac_bonus() {
    let input = load(MONK_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // WIS 17 + 2 Human racial (CG-03 fix) -> +4 modifier, so AC Bonus = max(4, 0) = 4.
    let ac_bonus = explanation(&computation, "class_chassis.monk.ac_bonus");
    assert_eq!(
        ac_bonus.value, 4,
        "monk AC Bonus is the positive Wisdom modifier added to AC: max(4, 0) = 4"
    );
    assert!(
        ac_bonus.detail.contains("Wisdom"),
        "monk AC Bonus explanation must name the Wisdom bonus source: {}",
        ac_bonus.detail
    );
}

// ----- Grounded (SD13-E5): unarmed strike damage die + Flurry of Blows flat surface -----

#[test]
fn monk_level1_grounds_unarmed_strike_damage_die() {
    let input = load(MONK_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Medium monk at level 1 deals 1d6 unarmed strike damage. Only the die-size
    // facet (6, i.e. 1d6) is grounded; no damage roll or damage total is computed.
    let die = explanation(&computation, "class_chassis.monk.unarmed_strike_damage_die");
    assert_eq!(
        die.value, 6,
        "monk level 1 Medium unarmed strike damage die is 1d6, so the die-size facet is 6"
    );
    assert!(
        die.detail.contains("1d6"),
        "monk unarmed-strike explanation must name the 1d6 die: {}",
        die.detail
    );
    assert!(
        die.detail.contains("die") && die.detail.contains("no damage roll"),
        "monk unarmed-strike explanation must scope itself to the die size and disclaim \
         damage-roll execution: {}",
        die.detail
    );
    assert!(
        die.detail.contains("lethal") && die.detail.contains("nonlethal"),
        "monk unarmed-strike explanation must record the lethal-or-nonlethal choice as a \
         rule statement: {}",
        die.detail
    );
    assert!(
        die.detail.contains("off-hand"),
        "monk unarmed-strike explanation must record the no-off-hand-penalty rule statement: {}",
        die.detail
    );
    assert!(
        die.detail.contains("1d8") && die.detail.contains("not grounded"),
        "monk unarmed-strike explanation must disclaim the higher-level die progression \
         (1d8 and beyond) as not grounded: {}",
        die.detail
    );
}

#[test]
fn monk_level1_grounds_flurry_of_blows_flat_surface() {
    let input = load(MONK_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 level-1 flurry: the monk uses her monk level in place of her base attack
    // bonus and takes -2 on all attacks, so the flat modifier is 1 - 2 = -1.
    let flurry_bonus = explanation(
        &computation,
        "class_chassis.monk.flurry_of_blows_attack_bonus",
    );
    assert_eq!(
        flurry_bonus.value, -1,
        "monk level 1 flurry flat attack modifier is monk level - 2 = 1 - 2 = -1"
    );
    assert!(
        flurry_bonus.detail.contains("monk level") && flurry_bonus.detail.contains("-2"),
        "monk flurry attack-bonus explanation must name the monk-level-in-place-of-BAB \
         formula and the -2 penalty: {}",
        flurry_bonus.detail
    );
    assert!(
        flurry_bonus.detail.contains("before ability modifiers"),
        "monk flurry attack-bonus explanation must scope the flat modifier as pre-ability: {}",
        flurry_bonus.detail
    );

    // A level-1 flurry grants two attacks. Only the count facet is grounded; no
    // attack-resolution engine exists.
    let flurry_count = explanation(
        &computation,
        "class_chassis.monk.flurry_of_blows_attack_count",
    );
    assert_eq!(
        flurry_count.value, 2,
        "monk level 1 flurry grants two attacks (one additional attack on a full attack)"
    );
    assert!(
        flurry_count.detail.contains("two attacks"),
        "monk flurry attack-count explanation must name the two-attack surface: {}",
        flurry_count.detail
    );
    assert!(
        flurry_count.detail.contains("no attack-resolution"),
        "monk flurry attack-count explanation must disclaim an attack-resolution engine: {}",
        flurry_count.detail
    );
}

#[test]
fn monk_level1_retires_the_unarmed_strike_and_flurry_blocker_but_stays_blocked() {
    let input = load(MONK_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The combined unarmed-strike/flurry blocker is retired: both facets are now
    // grounded as flat surfaces above.
    assert!(
        !has_diagnostic(
            &computation,
            "class_feature.monk.bounded_progression.unarmed_strike_and_flurry.unsupported"
        ),
        "the old unarmed-strike/flurry blocker diagnostic must no longer be emitted: {:?}",
        computation.diagnostics
    );

    // The monk row stays claim-blocked overall: the bonus-feat burden remains named.
    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert!(
        view_model.snapshot.is_none(),
        "blocked monk baseline must not emit a computed snapshot"
    );
}

// ----- Still blocked: narrowed bonus-feat-only diagnostic -----

#[test]
fn monk_level1_stays_blocked_naming_bonus_feat_only() {
    let input = load(MONK_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_diagnostic(
            &computation,
            "class_feature.monk.bounded_progression.ac_bonus_and_bonus_feat.unsupported"
        ),
        "the old combined AC-Bonus/bonus-feat blocker diagnostic must no longer be emitted: {:?}",
        computation.diagnostics
    );

    let bonus_feat = claim_blocking(
        &computation,
        "class_feature.monk.bounded_progression.bonus_feat.unsupported",
    );
    assert!(
        bonus_feat.message.contains("bonus feat"),
        "monk bonus-feat blocker must name the 'bonus feat' burden: {}",
        bonus_feat.message
    );
    assert!(
        !bonus_feat.message.contains("AC Bonus"),
        "the narrowed bonus-feat blocker must not claim AC Bonus is unimplemented: {}",
        bonus_feat.message
    );
}

// ----- The accepted Human race seam is preserved on the monk path -----

#[test]
fn monk_baseline_preserves_human_race_seam() {
    let input = load(MONK_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "race.human.ability_bonus_target"),
        "monk baseline must preserve the Human ability-bonus race seam: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, "race.human.bonus_feat_grant"),
        "monk baseline must preserve the Human bonus-feat race seam: {:?}",
        computation.explanations
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.human.bounded_semantics" && !d.claim_blocking),
        "monk baseline must keep the bounded, non-blocking Human race note: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: the monk path must not leak onto other classes -----

#[test]
fn fighter_barbarian_paladin_ranger_do_not_gain_monk_recognition() {
    let fighter = load(include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    ));
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !has_explanation(&fighter_computation, "class_chassis.monk.bounded_progression"),
        "the Fighter chassis must not surface a monk-baseline recognition record"
    );
    assert!(
        !fighter_computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.monk.")),
        "the Fighter chassis must not surface monk class-feature burden diagnostics: {:?}",
        fighter_computation.diagnostics
    );

    let barbarian = load(include_str!(
        "fixtures/rules_core/pf1_human_barbarian_level1_sd13_deterministic_input.txt"
    ));
    let barbarian_computation = compute_pilot_base_chassis(&barbarian);
    assert!(
        !has_explanation(&barbarian_computation, "class_chassis.monk.bounded_progression"),
        "Barbarian must not surface a monk-baseline recognition record"
    );
    assert!(
        !barbarian_computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.monk.")),
        "Barbarian must not surface monk class-feature burden diagnostics"
    );

    let paladin = load(include_str!(
        "fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt"
    ));
    let paladin_computation = compute_pilot_base_chassis(&paladin);
    assert!(
        !has_explanation(&paladin_computation, "class_chassis.monk.bounded_progression"),
        "Paladin must not surface a monk-baseline recognition record"
    );

    let ranger = load(include_str!(
        "fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt"
    ));
    let ranger_computation = compute_pilot_base_chassis(&ranger);
    assert!(
        !has_explanation(&ranger_computation, "class_chassis.monk.bounded_progression"),
        "Ranger must not surface a monk-baseline recognition record"
    );

    let rogue_fixture = MONK_FIXTURE.replace("class:monk:1", "class:rogue:1");
    let rogue = load(&rogue_fixture);
    let rogue_computation = compute_pilot_base_chassis(&rogue);
    assert!(
        !has_explanation(&rogue_computation, "class_chassis.monk.bounded_progression"),
        "Rogue must not surface a monk-baseline recognition record"
    );
    assert!(
        !rogue_computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.monk.")),
        "Rogue must not surface monk class-feature burden diagnostics"
    );
}

#[test]
fn monk_level_2_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 2 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_monk_level2_progression.rs) widened the level-1-only gate to
    // level 2 (mirroring the Fighter/Paladin/Rogue level-range gate idiom) and
    // grounded Evasion; this negative control is superseded, not violated —
    // pin the new truth here too so this file stays internally consistent.
    let level_2 = MONK_FIXTURE.replace("class:monk:1", "class:monk:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, "class_chassis.monk.bounded_progression"),
        "level-2 Monk is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.monk.")
                && d.id != "class_feature.monk.bounded_progression.bonus_feat.unsupported"),
        "level-2 Monk must not surface any named monk burden diagnostic beyond the still-blocked \
         bonus-feat-mechanics one (Evasion is grounded as an explanation record, not a \
         diagnostic): {:?}",
        computation.diagnostics
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-2 Monk must still be claim-blocked by the generic chassis diagnostics"
    );
}

#[test]
fn multiclass_monk_is_not_promoted_by_this_slice() {
    let multiclass = MONK_FIXTURE.replace(
        "class_level=class:monk:1",
        "class_level=class:monk:1\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, "class_chassis.monk.bounded_progression"),
        "multiclass Monk must not gain the bounded level-1 single-class martial recognition record"
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Monk must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix reclassifies the monk row -----
// Originally Partial/Computed; later promoted to Supported/ProductVisible by
// SD-19's Class Progression Catalog browser UI-surfacing work (2026-07-16).

#[test]
fn matrix_monk_row_is_partial_computed_and_names_remaining_burdens() {
    let matrix = seeded_current_truth();
    let monk = matrix
        .row("class.monk.bounded_progression")
        .expect("monk bounded_progression row must exist");

    assert_eq!(monk.support_state, SupportState::Supported);
    assert_eq!(monk.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        monk.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        monk.grounding_ref
            .contains("sd13_monk_level1_chassis_baseline"),
        "monk row must cite the SD13-E3 monk proof surface: {}",
        monk.grounding_ref
    );
    let note = monk.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "monk partial row must carry a note");
    // Base-attack, base-save, AC Bonus, the unarmed strike damage die, and the
    // flurry flat surface are now grounded; only the level-1 bonus feat grant
    // remains named as still-unproven, and the note must scope what the grounded
    // unarmed/flurry surface deliberately does not prove.
    for token in ["1d6", "Flurry of Blows", "bonus feat"] {
        assert!(
            note.contains(token),
            "monk partial note must name the '{token}' surface: {note}"
        );
    }
    for honesty_token in ["die size only", "attack-resolution", "level 7+"] {
        assert!(
            note.contains(honesty_token),
            "monk partial note must keep the '{honesty_token}' honesty scope: {note}"
        );
    }
    assert!(
        monk.next_required_uplift.contains("bonus feat"),
        "monk next uplift must point at the remaining bonus-feat burden: {}",
        monk.next_required_uplift
    );
}

#[test]
fn matrix_preserves_accepted_truth_and_unchanged_rows() {
    let matrix = seeded_current_truth();

    // Fighter, Druid, Barbarian, and Cleric rows were later promoted to
    // Supported/ProductVisible by SD-19's Class Progression Catalog browser
    // UI-surfacing work (2026-07-16).
    for id in [
        "class.fighter.level_1_pilot",
        "class.fighter.levels_2_10",
        "class.druid.progression_and_spell_burden",
        "class.barbarian.bounded_progression",
        "class.cleric.progression_and_spell_burden",
    ] {
        let row = matrix.row(id).unwrap_or_else(|| panic!("row {id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Supported,
            "row {id} must be Supported after the SD-19 class-row promotion"
        );
        assert_eq!(row.evidence_tier, EvidenceTier::ProductVisible);
    }

    // Paladin was later promoted to Partial/Computed by its own SD13-E5
    // level-gate slice (lay on hands / divine grace / mercy grounded as
    // correct level-1 absences).
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin row must exist");
    assert_eq!(
        paladin.support_state,
        SupportState::Supported,
        "paladin row must be Supported after the SD-19 class-row promotion"
    );

    // Ranger was later promoted to Partial/Computed by its own SD13-E3 Ranger
    // decomposition slice (Track grounded for real).
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger row must exist");
    assert_eq!(
        ranger.support_state,
        SupportState::Supported,
        "ranger row must be Supported after the SD-19 class-row promotion"
    );

    // Rogue was later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-17).
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue row must exist");
    assert_eq!(rogue.support_state, SupportState::Supported);
    assert_eq!(rogue.evidence_tier, EvidenceTier::ProductVisible);

    // Sorcerer was later promoted to Partial/Computed by its own SD13-E4
    // decomposition slice (Eschew Materials grounded for real), then to
    // Supported/ProductVisible by SD-19's Class Progression Catalog browser
    // UI-surfacing work (2026-07-17).
    let sorcerer = matrix
        .row("class.sorcerer.progression_and_spell_burden")
        .unwrap_or_else(|| panic!("row class.sorcerer.progression_and_spell_burden must exist"));
    assert_eq!(
        sorcerer.support_state,
        SupportState::Supported,
        "sorcerer row must be Supported after the SD-19 class-row promotion"
    );
    assert_eq!(sorcerer.evidence_tier, EvidenceTier::ProductVisible);

    // Bard was later promoted to Partial/Computed by its own SD13-E4
    // decomposition slice (Bardic Knowledge grounded for real), then to
    // Supported/ProductVisible by SD-19's Class Progression Catalog browser
    // UI-surfacing work (2026-07-16).
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .unwrap_or_else(|| panic!("row class.bard.progression_and_spell_burden must exist"));
    assert_eq!(
        bard.support_state,
        SupportState::Supported,
        "bard row must be Supported after the SD-19 class-row promotion"
    );
    assert_eq!(bard.evidence_tier, EvidenceTier::ProductVisible);

    // Wizard was later promoted to Partial/Computed by its own SD13-E4 Scribe
    // Scroll decomposition slice, then to Supported/ProductVisible by SD-19's
    // Class Progression Catalog browser UI-surfacing work (2026-07-17).
    let wizard = matrix
        .row("class.wizard.progression_and_spell_burden")
        .expect("wizard row must exist");
    assert_eq!(
        wizard.support_state,
        SupportState::Supported,
        "wizard row must keep its later-accepted Supported posture after the monk slice"
    );

    assert!(
        !matrix
            .rows
            .iter()
            // school.abjuration/illusion.spell_reachability were later promoted to
            // Supported/Product-visible by SD-19's operator-driven UI-surfacing work
            // (2026-07-16) -- excluded here, not an unintended promotion by this slice.
            .any(|r| (r.support_state == SupportState::Supported
                && r.row_id != "school.abjuration.spell_reachability"
                && r.row_id != "school.illusion.spell_reachability"
                && r.row_id != "school.conjuration.spell_reachability"
                && r.row_id != "school.divination.spell_reachability"
                && r.row_id != "school.enchantment.spell_reachability"
                && r.row_id != "school.evocation.spell_reachability"
                && r.row_id != "school.necromancy.spell_reachability"
                && r.row_id != "school.transmutation.spell_reachability"
                && r.row_id != "school.universal.spell_reachability"
                && r.row_id != "equipment.arms_armor.equipment_reachability"
                && r.row_id != "equipment.general.equipment_reachability"
                && r.row_id != "equipment.magic_items.equipment_reachability"
                && r.row_id != "race.human.pilot_semantics"
                && r.row_id != "race.dwarf.bounded_semantics"
                && r.row_id != "race.elf.bounded_semantics"
                && r.row_id != "race.gnome.bounded_semantics"
                && r.row_id != "race.half_elf.bounded_semantics"
                && r.row_id != "race.half_orc.bounded_semantics"
                && r.row_id != "race.halfling.bounded_semantics"
                && r.row_id != "class.fighter.level_1_pilot"
                && r.row_id != "class.fighter.levels_2_10"
                && r.row_id != "class.monk.bounded_progression"
                && r.row_id != "class.druid.progression_and_spell_burden"
                && r.row_id != "class.barbarian.bounded_progression"
                && r.row_id != "class.cleric.progression_and_spell_burden"
                && r.row_id != "class.wizard.progression_and_spell_burden"
                && r.row_id != "class.rogue.bounded_progression"
                && r.row_id != "class.sorcerer.progression_and_spell_burden"
                && r.row_id != "class.bard.progression_and_spell_burden"
                && r.row_id != "class.paladin.hybrid_chassis_and_spell_burden"
                && r.row_id != "class.ranger.hybrid_chassis_and_spell_burden"
                && r.row_id != "interaction.human_bonus_feat_ability_bonus.pilot_pressure"
                && r.row_id != "equipment.equipmods.equipment_reachability")
                || r.support_state == SupportState::Lossy),
        "no row may be silently promoted to Supported or Lossy outside the \
         intentionally-promoted SD-19 rows"
    );
}
