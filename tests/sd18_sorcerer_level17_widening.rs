//! SD18 Sorcerer level-17 widening grounding proof.
//!
//! Widens the accepted deterministic Human Sorcerer level-1..level-16 spell-
//! bearing chassis (`tests/sd18_sorcerer_level16_widening.rs`) to Sorcerer
//! level 17, mirroring the sibling-class level-range-gate idiom
//! (`supported_sorcerer_level` is generalized from `1..=16` to `1..=17` via
//! `MAX_SUPPORTED_SORCERER_LEVEL = 17`, exactly as this sweep already
//! widened `MAX_SUPPORTED_RANGER_LEVEL`, `MAX_SUPPORTED_BARD_LEVEL`,
//! `MAX_SUPPORTED_ROGUE_LEVEL`, `MAX_SUPPORTED_FIGHTER_LEVEL`,
//! `MAX_SUPPORTED_WIZARD_LEVEL`, `MAX_SUPPORTED_CLERIC_LEVEL`,
//! `MAX_SUPPORTED_PALADIN_LEVEL`, and `MAX_SUPPORTED_BARBARIAN_LEVEL`, all
//! from 16 to 17 already this level-17 sweep).
//! §3.1 race rows and §3.3 interaction rows stay fully exhausted /
//! structurally blocked (cited from the progress doc, not re-derived).
//!
//! Two PF1 CRB primary sources (d20pfsrd and the Archives of Nethys
//! aonprd.com mirror) were read directly before writing any code or test,
//! fetching the full levels-15-through-19 class-table block (including the
//! separate Spells Known table) in one pass to rule out level-misattribution,
//! and agreed byte-for-byte on every value below:
//!
//! - level 17 base attack bonus STAYS at +8 (`17 / 2 = 8`, an
//!   integer-division coincidence with level 16) via the same
//!   already-grounded formula, not re-derived. Both poor saves STAY at +5
//!   (`17 / 3 = 5`) and good Will STAYS at +10 (`17 / 2 + 2 = 10`), all
//!   integer-division coincidences with level 16.
//! - the PF1 Core Rulebook Sorcerer class table's level-17 "Special" column
//!   reads "Bloodline spell" (bloodline-specific, left named-but-unproven by
//!   the pre-existing Arcane Bond / bloodline progression blocker, exactly
//!   mirroring levels 3/5/7/9/11/13/15) — UNLIKE level 16's genuinely blank
//!   Special column. No new pillar record is grounded from it, mirroring the
//!   exact treatment of every prior odd-level bloodline entry in this row's
//!   own history.
//! - the Spells per Day table's level-17 row is `6/6/6/6/6/6/6/4` (1st
//!   through 8th), genuinely risen from level 16's `6/6/6/6/6/6/5/3` at the
//!   7th spell level (5 -> 6) AND the 8th-level column (3 -> 4), with no
//!   genuinely new spell-level column opening (the 8th-level column already
//!   opened at level 16).
//! - the Spells Known table's level-17 row is `9/5/5/4/4/4/3/3/2` (0th
//!   through 8th), with the 0th-6th columns numerically UNCHANGED from level
//!   16's `9/5/5/4/4/4/3/2/1`, while the 7th-level column rises from 2 to 3
//!   AND the 8th-level column rises from 1 to 2 — no genuinely new
//!   spell-level column opens.
//! - the spell-level access ladder STAYS at 8 (unchanged from level 16; no
//!   new threshold constant is needed).
//! - the spell-save-DC and Charisma-bonus-spell formulas widen automatically
//!   over the unchanged access ladder, with no new code needed (both loops
//!   already iterate generically over `1..=sorcerer_spell_level_access`).
//! - the bloodline choice and bloodline class-skill choice recognitions are
//!   not level-gated, so both still fire at level 17 for the same fixture
//!   selections.
//!
//! It deliberately does not touch Arcane Bond, bloodline arcana, the
//! bloodline feat selection, the bloodline bonus spell selection, or the
//! spontaneous which-spells-known selection / casting-execution burden (all
//! stay named-but-unproven, unchanged from levels 1-16), and it does not
//! ground Sorcerer level 18+. It also preserves the accepted Sorcerer
//! level-1..level-16 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control. Per the sweep's established lesson
//! about stale negative controls, this cycle also moves the sibling "level
//! 17 is not promoted" negative controls in
//! `tests/sd13_sorcerer_level10_progression.rs`,
//! `tests/sd18_sorcerer_level11_widening.rs`,
//! `tests/sd18_sorcerer_level12_widening.rs`,
//! `tests/sd18_sorcerer_level13_widening.rs`, and
//! `tests/sd18_sorcerer_level14_widening.rs` — to a "level 18 is not
//! promoted" boundary; `tests/sd18_sorcerer_level15_widening.rs`'s and
//! `tests/sd18_sorcerer_level16_widening.rs`'s own level-17 negative controls
//! are removed rather than moved, since level 17 is now itself the
//! supported/grounded row, mirroring the exact fix every prior level-N cycle
//! made for its own siblings.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const SORCERER_LEVEL16_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_sorcerer_level16_sd18_widening_deterministic_input.txt"
);

const SORCERER_LEVEL17_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_sorcerer_level17_sd18_widening_deterministic_input.txt"
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

// ----- Base attack bonus at level 17 stays put (integer-division coincidence) -----

#[test]
fn sorcerer_level17_base_attack_bonus_is_grounded() {
    let input = load(SORCERER_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(
        base_attack.value, 8,
        "Sorcerer level 17 1/2-BAB progression (17 / 2) must equal 8 — an integer-division \
         coincidence with level 16's +8: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 17: all three stay put (integer-division coincidences) -----

#[test]
fn sorcerer_level17_base_saves_are_grounded() {
    let input = load(SORCERER_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.sorcerer.base_save.fortitude");
    assert_eq!(
        fortitude.value, 5,
        "Sorcerer level 17 poor Fortitude (17/3) must equal 5 — an integer-division \
         coincidence with level 16's +5"
    );

    let reflex = explanation(&computation, "class_chassis.sorcerer.base_save.reflex");
    assert_eq!(
        reflex.value, 5,
        "Sorcerer level 17 poor Reflex (17/3) must equal 5 — an integer-division coincidence \
         with level 16's +5"
    );

    let will = explanation(&computation, "class_chassis.sorcerer.base_save.will");
    assert_eq!(
        will.value, 10,
        "Sorcerer level 17 good Will (17/2+2) must equal 10 — an integer-division coincidence \
         with level 16's +10"
    );
}

// ----- Base spells per day widen at level 17 within the already-widened 8-column shape -----

#[test]
fn sorcerer_level17_base_spells_per_day_match_the_raw_table_row() {
    let input = load(SORCERER_LEVEL17_FIXTURE);
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
            (format!("{PER_DAY_PREFIX}spell_level_7"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_8"), 4),
        ],
        "level 17 (`6/6/6/6/6/6/6/4`): the 7th-level column rises from 5 to 6 AND the 8th-level \
         column rises from 3 to 4, with no genuinely new spell-level column opening"
    );
}

// ----- Spells known widen at level 17 within the already-widened 9-column shape -----

#[test]
fn sorcerer_level17_spells_known_match_the_raw_table_row() {
    let input = load(SORCERER_LEVEL17_FIXTURE);
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
            (format!("{KNOWN_PREFIX}spell_level_7"), 3),
            (format!("{KNOWN_PREFIX}spell_level_8"), 2),
        ],
        "level 17 (`9/5/5/4/4/4/3/3/2`): the 0th through 6th columns stay numerically unchanged \
         from level 16's `9/5/5/4/4/4/3/2/1`, while the 7th-level column rises from 2 to 3 AND \
         the 8th-level column rises from 1 to 2"
    );
}

// ----- Spell-level access ladder stays at 8 (integer-division coincidence, no new threshold) -----

#[test]
fn sorcerer_level17_spell_level_access_stays_at_eight() {
    let input = load(SORCERER_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(
        &computation,
        "class_chassis.sorcerer.spontaneous.spell_level_access",
    );
    assert_eq!(
        access.value, 8,
        "Sorcerer level 17 spell-level access must stay at 8th-level spells — unchanged from \
         level 16, no new threshold reached: {}",
        access.detail
    );
}

// ----- Bonus spells and spell-save DCs extend to the 8th spell level with no new code -----

#[test]
fn sorcerer_level17_bonus_spells_and_save_dcs_extend_to_eighth_level() {
    let input = load(SORCERER_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dc8 = explanation(
        &computation,
        "class_chassis.sorcerer.spontaneous.spell_save_dc.spell_level_8",
    );
    // Fixture Charisma 17 -> Charisma modifier +3 (unchanged deterministic posture).
    // 10 + 8 + 3 = 21.
    assert_eq!(
        dc8.value, 22,
        "Sorcerer level 17 8th-level spell save DC must be 10 + 8 + Charisma modifier: {}",
        dc8.detail
    );

    let bonus8 = explanation(
        &computation,
        "class_chassis.sorcerer.spontaneous.bonus_spells_per_day.spell_level_8",
    );
    // Charisma modifier +3 < spell level 8, so bonus spells at 8th level is 0.
    assert_eq!(
        bonus8.value, 0,
        "Sorcerer level 17 8th-level bonus spells from a +3 Charisma modifier must be 0 (below \
         the spell level threshold): {}",
        bonus8.detail
    );

    let total8 = explanation(
        &computation,
        "class_chassis.sorcerer.spontaneous.total_spells_per_day.spell_level_8",
    );
    assert_eq!(
        total8.value, 4,
        "Sorcerer level 17 8th-level total spells per day must equal the base count (4) plus \
         the zero bonus: {}",
        total8.detail
    );
}

// ----- Bloodline choice recognition still fires at level 17 -----

#[test]
fn sorcerer_level17_still_recognizes_the_bloodline_choice() {
    let input = load(SORCERER_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.sorcerer.bloodline_choice");
    assert_eq!(
        choice.value, 0,
        "bloodline choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Arcane"),
        "bloodline choice recognition must still name the Arcane bloodline at level 17: {}",
        choice.detail
    );
}

// ----- The spell-bearing baseline recognition and both burden diagnostics persist -----

#[test]
fn sorcerer_level17_still_recognizes_the_spell_bearing_baseline_and_claim_blocks_burdens() {
    let input = load(SORCERER_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.sorcerer"),
        "level-17 Sorcerer must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported"
            && d.claim_blocking),
        "level-17 Sorcerer must still claim-block on the Arcane Bond / bloodline progression \
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

// ----- No new bloodline-feat/bloodline-power/bloodline-spell record is fabricated at level 17 -----

#[test]
fn sorcerer_level17_does_not_fabricate_any_bloodline_entry() {
    let input = load(SORCERER_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("bloodline_feat")
                || e.id.to_lowercase().contains("bloodline_power")
                || e.id.to_lowercase().contains("bloodline_spell")),
        "level-17 Sorcerer must not fabricate a bloodline-feat/bloodline-power/bloodline-spell \
         record — the level-17 Special column names \"Bloodline spell\" but stays \
         bloodline-specific and named-but-unproven, exactly mirroring levels 3/5/7/9/11/13/15: \
         {:?}",
        computation.explanations
    );
}

// ----- Negative control: level 16 truth is unchanged by this widening -----

#[test]
fn sorcerer_level16_truth_is_unchanged_by_this_slice() {
    let input = load(SORCERER_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(base_attack.value, 8, "Sorcerer level 16 base attack bonus must stay 8");

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_5"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_6"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_7"), 5),
            (format!("{PER_DAY_PREFIX}spell_level_8"), 3),
        ],
        "Sorcerer level 16 base spells per day must stay `6/6/6/6/6/6/5/3`, unchanged by this \
         slice"
    );
}

// ----- Negative control: the sorcerer path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_sorcerer_level17_recognition() {
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
fn multiclass_sorcerer_level17_is_not_promoted_by_this_slice() {
    let multiclass = SORCERER_LEVEL17_FIXTURE.replace(
        "class_level=class:sorcerer:17",
        "class_level=class:sorcerer:17\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-17 widening -----

#[test]
fn matrix_sorcerer_row_names_level_17_widening() {
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
            .contains("sd18_sorcerer_level17_widening"),
        "sorcerer row must cite the live SD18 level-17 proof surface: {}",
        sorcerer.grounding_ref
    );
    let note = sorcerer.blocker_or_lossiness_note;
    assert!(
        note.contains("level 17") || note.contains("level-17"),
        "sorcerer partial note must name the level-17 widening: {note}"
    );
}
