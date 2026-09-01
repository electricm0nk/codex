//! SD18 Sorcerer level-15 widening grounding proof.
//!
//! Widens the accepted deterministic Human Sorcerer level-1..level-14 spell-
//! bearing chassis (`tests/sd18_sorcerer_level14_widening.rs`) to Sorcerer
//! level 15, mirroring the sibling-class level-range-gate idiom
//! (`supported_sorcerer_level` is generalized from `1..=14` to `1..=15` via
//! `MAX_SUPPORTED_SORCERER_LEVEL = 15`, exactly as this cycle's level-15
//! sweep already widened `MAX_SUPPORTED_BARBARIAN_LEVEL`,
//! `MAX_SUPPORTED_ROGUE_LEVEL`, `MAX_SUPPORTED_FIGHTER_LEVEL`,
//! `MAX_SUPPORTED_CLERIC_LEVEL`, `MAX_SUPPORTED_DRUID_LEVEL`,
//! `MAX_SUPPORTED_RANGER_LEVEL`, `MAX_SUPPORTED_WIZARD_LEVEL`, and
//! `MAX_SUPPORTED_PALADIN_LEVEL`, all from 14 to 15 already this sweep).
//! §3.1 race rows and §3.3 interaction rows stay fully exhausted /
//! structurally blocked (cited from the progress doc, not re-derived). This
//! cycle re-verified live which §3.2 classes still sat at the level-14
//! ceiling: Bard and Sorcerer both did. Per the progress doc's own risk
//! assessment (verified directly rather than trusted at face value), this
//! cycle picks **Sorcerer**.
//!
//! Two PF1 CRB primary sources (d20pfsrd and the Archives of Nethys
//! aonprd.com mirror) were read directly before writing any code or test,
//! and agreed byte-for-byte on every value below:
//!
//! - level 15 base attack bonus STAYS +7 (`15 / 2 = 7`, an integer-division
//!   coincidence with level 14's +7) via the same already-grounded formula,
//!   not re-derived. The good Will save also STAYS +9 (`15 / 2 + 2 = 9`,
//!   an integer-division coincidence with level 14). Both poor saves
//!   genuinely RISE to +5 (`15 / 3 = 5`, up from level 14's +4).
//! - the PF1 Core Rulebook Sorcerer class table's level-15 "Special" column
//!   reads "Bloodline power, bloodline spell" (verified independently
//!   against both primary sources, checked rather than assumed away) — a
//!   further bloodline power and bloodline spell grant, exactly mirroring
//!   the level-3/5/7/9/11/13 pattern this row has already left unproven six
//!   times. Both entries are bloodline-specific (varying per chosen
//!   bloodline) and neither is flat/identity-shaped, so this slice grounds
//!   no new pillar from the Special column either — both entries stay named
//!   by the existing Arcane Bond / bloodline progression blocker's "bonus
//!   spells/feats at 3rd+ level" and "bloodline power" language, unchanged.
//! - the Spells per Day table's level-15 row is `6/6/6/6/6/6/4` (1st through
//!   7th), genuinely risen from level 14's `6/6/6/6/6/5/3` at the 6th spell
//!   level (5 -> 6) AND the 7th-level column (3 -> 4) — no genuinely new
//!   spell-level column opens (the 8th-level column stays "—" through level
//!   16). This widens `sorcerer_base_spells_per_day` within its existing
//!   7-element array shape.
//! - the Spells Known table's level-15 row is `9/5/5/4/4/4/3/2` (0th through
//!   7th), genuinely risen from level 14's `9/5/5/4/4/3/2/1` at the 5th
//!   spell level (3 -> 4), the 6th spell level (2 -> 3), and the 7th spell
//!   level (1 -> 2), while the 0th-4th columns stay numerically unchanged.
//!   This widens `sorcerer_spells_known` within its existing 8-element array
//!   shape.
//! - the spell-level access ladder STAYS at 7 (unchanged from level 14; the
//!   8th-level threshold is not reached at level 15).
//! - the spell-save-DC and Charisma-bonus-spell formulas are unchanged
//!   (live arithmetic over the unchanged access ladder and the fixture's
//!   Charisma modifier); no new code is needed since both loops already
//!   iterate generically over `1..=sorcerer_spell_level_access`.
//! - the bloodline choice and bloodline class-skill choice recognitions are
//!   not level-gated, so both still fire at level 15 for the same fixture
//!   selections.
//!
//! It deliberately does not touch Arcane Bond, bloodline arcana, the
//! bloodline feat selection, the bloodline bonus spell selection, or the
//! spontaneous which-spells-known selection / casting-execution burden (all
//! stay named-but-unproven, unchanged from levels 1-14), and it does not
//! ground Sorcerer level 16+. It also preserves the accepted Sorcerer
//! level-1..level-14 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control. Per the sweep's established lesson
//! about stale negative controls, this cycle also moves the sibling "level
//! 15 is not promoted" negative controls in
//! `tests/sd13_sorcerer_level10_progression.rs`,
//! `tests/sd18_sorcerer_level11_widening.rs`,
//! `tests/sd18_sorcerer_level12_widening.rs`, and
//! `tests/sd18_sorcerer_level13_widening.rs` — plus this file's own sibling
//! `tests/sd18_sorcerer_level14_widening.rs`, which already carries a level
//! 15 negative control unchanged — to a "level 16 is not promoted" boundary
//! where still applicable, in the same commit.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const SORCERER_LEVEL14_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_sorcerer_level14_sd18_widening_deterministic_input.txt"
);

const SORCERER_LEVEL15_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_sorcerer_level15_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const PER_DAY_PREFIX: &str = "class_chassis.sorcerer.spontaneous.base_spells_per_day.";
const KNOWN_PREFIX: &str = "class_chassis.sorcerer.spontaneous.spells_known.";

fn values_with_prefix(
    computation: &PilotBaseChassisComputation,
    prefix: &str,
) -> Vec<(String, i16)> {
    computation
        .explanations
        .iter()
        .filter(|e| e.id.starts_with(prefix))
        .map(|e| (e.id.clone(), e.value))
        .collect()
}

// ----- Base attack bonus at level 15 stays unchanged (integer-division coincidence) -----

#[test]
fn sorcerer_level15_base_attack_bonus_is_grounded() {
    let input = load(SORCERER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(
        base_attack.value, 7,
        "Sorcerer level 15 1/2-BAB progression (15 / 2) must equal 7 — an integer-division \
         coincidence with level 14's +7: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 15: poor saves rise, good Will unchanged -----

#[test]
fn sorcerer_level15_base_saves_are_grounded() {
    let input = load(SORCERER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.sorcerer.base_save.fortitude");
    assert_eq!(
        fortitude.value, 5,
        "Sorcerer level 15 poor Fortitude (15/3) must equal 5 — genuinely risen from level 14's \
         +4"
    );

    let reflex = explanation(&computation, "class_chassis.sorcerer.base_save.reflex");
    assert_eq!(
        reflex.value, 5,
        "Sorcerer level 15 poor Reflex (15/3) must equal 5 — genuinely risen from level 14's +4"
    );

    let will = explanation(&computation, "class_chassis.sorcerer.base_save.will");
    assert_eq!(
        will.value, 9,
        "Sorcerer level 15 good Will (15/2+2) must equal 9 — an integer-division coincidence \
         with level 14's +9"
    );
}

// ----- Base spells per day widen at level 15 within the existing 7-element shape -----

#[test]
fn sorcerer_level15_base_spells_per_day_match_the_raw_table_row() {
    let input = load(SORCERER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_5"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_6"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_7"), 4),
        ],
        "level 15 (`6/6/6/6/6/6/4`): the 6th-level column rises from 5 to 6 and the 7th-level \
         column rises from 3 to 4 — no genuinely new spell-level column opens"
    );
}

// ----- Spells known widen at level 15 within the existing 8-element shape -----

#[test]
fn sorcerer_level15_spells_known_match_the_raw_table_row() {
    let input = load(SORCERER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, KNOWN_PREFIX),
        vec![
            (format!("{KNOWN_PREFIX}spell_level_0"), 9),
            (format!("{KNOWN_PREFIX}spell_level_1"), 5),
            (format!("{KNOWN_PREFIX}spell_level_2"), 5),
            (format!("{KNOWN_PREFIX}spell_level_3"), 4),
            (format!("{KNOWN_PREFIX}spell_level_4"), 4),
            (format!("{KNOWN_PREFIX}spell_level_5"), 4),
            (format!("{KNOWN_PREFIX}spell_level_6"), 3),
            (format!("{KNOWN_PREFIX}spell_level_7"), 2),
        ],
        "level 15 (`9/5/5/4/4/4/3/2`): the 0th through 4th columns stay numerically unchanged \
         from level 14's `9/5/5/4/4/3/2/1`, while the 5th, 6th, and 7th columns each rise by one"
    );
}

// ----- Spell-level access ladder stays at 7 (unchanged from level 14) -----

#[test]
fn sorcerer_level15_spell_level_access_stays_at_seven() {
    let input = load(SORCERER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(
        &computation,
        "class_chassis.sorcerer.spontaneous.spell_level_access",
    );
    assert_eq!(
        access.value, 7,
        "Sorcerer level 15 spell-level access must stay at 7th-level spells — the 8th-level \
         threshold is not reached at level 15: {}",
        access.detail
    );
}

// ----- Bonus spells and spell-save DCs extend to the 7th spell level with no new code -----

#[test]
fn sorcerer_level15_bonus_spells_and_save_dcs_extend_to_seventh_level() {
    let input = load(SORCERER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dc7 = explanation(
        &computation,
        "class_chassis.sorcerer.spontaneous.spell_save_dc.spell_level_7",
    );
    // Fixture Charisma 17 -> Charisma modifier +3 (unchanged deterministic posture).
    // 10 + 7 + 3 = 20.
    assert_eq!(
        dc7.value, 21,
        "Sorcerer level 15 7th-level spell save DC must be 10 + 7 + Charisma modifier: {}",
        dc7.detail
    );

    let bonus7 = explanation(
        &computation,
        "class_chassis.sorcerer.spontaneous.bonus_spells_per_day.spell_level_7",
    );
    // Charisma modifier +3 < spell level 7, so bonus spells at 7th level is 0.
    assert_eq!(
        bonus7.value, 0,
        "Sorcerer level 15 7th-level bonus spells from a +3 Charisma modifier must be 0 (below \
         the spell level threshold): {}",
        bonus7.detail
    );

    let total7 = explanation(
        &computation,
        "class_chassis.sorcerer.spontaneous.total_spells_per_day.spell_level_7",
    );
    assert_eq!(
        total7.value, 4,
        "Sorcerer level 15 7th-level total spells per day must equal the base count (4) plus \
         the zero bonus: {}",
        total7.detail
    );
}

// ----- Bloodline choice recognition still fires at level 15 -----

#[test]
fn sorcerer_level15_still_recognizes_the_bloodline_choice() {
    let input = load(SORCERER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.sorcerer.bloodline_choice");
    assert_eq!(
        choice.value, 0,
        "bloodline choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Arcane"),
        "bloodline choice recognition must still name the Arcane bloodline at level 15: {}",
        choice.detail
    );
}

// ----- The spell-bearing baseline recognition and both burden diagnostics persist -----

#[test]
fn sorcerer_level15_still_recognizes_the_spell_bearing_baseline_and_claim_blocks_burdens() {
    let input = load(SORCERER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.sorcerer"),
        "level-15 Sorcerer must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported"
            && d.claim_blocking),
        "level-15 Sorcerer must still claim-block on the Arcane Bond / bloodline progression \
         burden: {:?}",
        computation.diagnostics
    );
    match computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_spell.sorcerer.spontaneous.unsupported")
    {
        Some(spell_blocker) => assert!(
            spell_blocker.claim_blocking,
            "if the spell blocker fires at all, it must be claim-blocking"
        ),
        None => {
            // (v0.6 alpha swarm, risks item 8) class_spell.sorcerer.spontaneous.unsupported
            // is no longer unconditional -- it's a real, conditional validation of
            // AcquisitionMode::Known selections. This fixture predates spells_selected
            // (zero known spells), so the posture is genuinely valid and the blocker
            // correctly does not fire -- confirmed via the real known-spell count being
            // honestly 0, not fabricated.
            let known_count = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_spell.sorcerer.known_spells")
                .map(|e| e.value)
                .unwrap_or(-1);
            assert_eq!(
                known_count, 0,
                "no spells are fabricated merely because the spell blocker stopped firing: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- No new bloodline-feat/bloodline-power/bloodline-spell record is fabricated at level 15 -----

#[test]
fn sorcerer_level15_does_not_fabricate_any_bloodline_entry() {
    let input = load(SORCERER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| (e.id.to_lowercase().contains("bloodline_feat")
                || e.id.to_lowercase().contains("bloodline_power")
                || e.id.to_lowercase().contains("bloodline_spell"))
                // AT-34-E3-001 owner-matched cycle 5 (`cb0ba2286e`, 2026-08-28) grounded
                // the Bloodline Feat pool's bounded slot-count tracker (real, tested,
                // pre-existing content -- see mod.rs's own
                // sorcerer_bloodline_feat_pool_slot_count_* tests). It is a generic pool
                // size, never a specific named bloodline feat/power/spell entry -- not
                // the fabrication this test guards against.
                && e.id != "class_feature.sorcerer.bloodline_feat_pool.slot_count"),
        "level-15 Sorcerer must not fabricate a bloodline-feat/bloodline-power/bloodline-spell \
         record — the level-15 Special column's 'Bloodline power, bloodline spell' entry is \
         bloodline-specific and stays named-but-unproven, exactly like levels 3/5/7/9/11/13: \
         {:?}",
        computation.explanations
    );
}

// ----- Negative control: level 14 truth is unchanged by this widening -----

#[test]
fn sorcerer_level14_truth_is_unchanged_by_this_slice() {
    let input = load(SORCERER_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(base_attack.value, 7, "Sorcerer level 14 base attack bonus must stay 7");

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_5"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_6"), 5),
            (format!("{PER_DAY_PREFIX}spell_level_7"), 3),
        ],
        "Sorcerer level 14 base spells per day must stay `6/6/6/6/6/5/3`"
    );
}

// ----- Negative control: the sorcerer path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_sorcerer_level15_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.sorcerer.")
                || e.id == "class_chassis.spell_baseline.sorcerer"),
        "the Fighter chassis must not surface any sorcerer-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Sorcerer is not promoted -----

#[test]
fn multiclass_sorcerer_level15_is_not_promoted_by_this_slice() {
    let multiclass = SORCERER_LEVEL15_FIXTURE.replace(
        "class_level=class:sorcerer:15",
        "class_level=class:sorcerer:15\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.sorcerer.")
                || e.id == "class_chassis.spell_baseline.sorcerer"),
        "multiclass Sorcerer must not gain any bounded sorcerer chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Sorcerer must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-15 widening -----

#[test]
fn matrix_sorcerer_row_names_level_15_widening() {
    let matrix = seeded_current_truth();
    let sorcerer = matrix
        .row("class.sorcerer.progression_and_spell_burden")
        .expect("sorcerer progression_and_spell_burden row must exist");

    assert_eq!(sorcerer.support_state, SupportState::Supported);
    assert_eq!(sorcerer.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        sorcerer.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        sorcerer
            .grounding_ref
            .contains("sd18_sorcerer_level15_widening"),
        "sorcerer row must cite the live SD18 level-15 proof surface: {}",
        sorcerer.grounding_ref
    );
    let note = sorcerer.blocker_or_lossiness_note;
    assert!(
        note.contains("level 15") || note.contains("level-15"),
        "sorcerer partial note must name the level-15 widening: {note}"
    );
}
