//! SD18 Sorcerer level-11 widening grounding proof.
//!
//! Widens the accepted deterministic Human Sorcerer level-1..level-10 spell-
//! bearing chassis (`tests/sd13_sorcerer_level10_progression.rs`, the SD13
//! tranche's declared ceiling) to Sorcerer level 11 — the tenth SD-18 §3.2
//! class-row widening, mirroring the sibling-class level-range-gate idiom
//! (`supported_sorcerer_level` is generalized from `1..=10` to `1..=11` via
//! `MAX_SUPPORTED_SORCERER_LEVEL = 11`, exactly as prior cycles widened
//! `MAX_SUPPORTED_BARBARIAN_LEVEL`, `MAX_SUPPORTED_BARD_LEVEL`,
//! `MAX_SUPPORTED_CLERIC_LEVEL`, `MAX_SUPPORTED_DRUID_LEVEL`,
//! `MAX_SUPPORTED_FIGHTER_LEVEL`, `MAX_SUPPORTED_MONK_LEVEL`,
//! `MAX_SUPPORTED_PALADIN_LEVEL`, `MAX_SUPPORTED_ROGUE_LEVEL`, and
//! `MAX_SUPPORTED_WIZARD_LEVEL`, all from 10 to 11). §3.1 race rows and §3.3
//! interaction rows were re-checked live this cycle per priority order and
//! remain, respectively, fully exhausted and non-advanceable (no class row
//! yet branches its compute path on a specific non-Human race identity), so
//! this cycle picks the next fully-untouched §3.2 class row: Ranger and
//! Sorcerer were the two remaining candidates, and Sorcerer's own level-11
//! "Special" column entry ("Bloodline spell") is confirmed bloodline-
//! specific and therefore already-precedented as ungroundable (mirroring the
//! level-3/5/7/9 "Bloodline power"/"Bloodline spell" entries this seam has
//! already left unproven three times running), so this cycle is a pure
//! widen-the-flat-pillars cycle with no new-subsystem risk — the cleanest
//! fit of the two remaining candidates.
//!
//! Both PF1 CRB primary sources (d20pfsrd and the Archives of Nethys
//! aonprd.com mirror) were read directly before writing any code or test,
//! and both agree byte-for-byte:
//!
//! - level 11 base attack bonus stays +5 (`11 / 2 = 5`, unchanged from level
//!   10, an integer-division coincidence, not a sign the 1/2-BAB formula
//!   stopped scaling) and base saves stay +3 Fortitude (poor, `11 / 3 = 3`),
//!   +3 Reflex (poor, `11 / 3 = 3`), and +7 Will (good, `11 / 2 + 2 = 7`) —
//!   all four values numerically IDENTICAL to level 10, confirmed by the
//!   same formulas already grounded at levels 1-10, not re-derived.
//! - the PF1 Core Rulebook Sorcerer class table's level-11 "Special" column
//!   reads only "Bloodline spell" (verified independently against both
//!   primary sources, checked rather than assumed away) — the sorcerer's
//!   fifth bloodline spell grant, bloodline-specific and not flat/
//!   identity-shaped, so this cycle grounds no new pillar for level 11
//!   either, mirroring exactly how the level-3/5/7/9 bloodline power/spell
//!   entries were left unproven.
//! - the Spells per Day table's level-11 row is `6/6/6/6/4` (1st/2nd/3rd/
//!   4th/5th), genuinely risen from level 10's `6/6/6/5/3` at the 4th
//!   (5 -> 6) and 5th (3 -> 4) spell levels, with the 6th-level column
//!   staying "—" (6th-level spells first arrive at level 12, not level 11 —
//!   checked, not assumed) — verified identically on both primary sources.
//! - the Spells Known table's level-11 row is `9/5/5/4/3/2` (0th/1st/2nd/
//!   3rd/4th/5th), genuinely risen from level 10's `9/5/4/3/2/1` at the 2nd
//!   (4 -> 5), 3rd (3 -> 4), 4th (2 -> 3), and 5th (1 -> 2) spell levels,
//!   with the 0th and 1st columns unchanged (9, 5) and no new 6th-level
//!   column appearing — verified identically on both primary sources.
//! - the spell-save-DC and Charisma-bonus-spell formulas are unchanged
//!   (live arithmetic over the already-grounded access ladder and the
//!   fixture's Charisma modifier), widening automatically to the level-11
//!   access ladder (which itself stays at 5, unchanged from level 10, since
//!   the 6th-level threshold has not yet been reached).
//! - the bloodline choice and bloodline class-skill choice recognitions are
//!   not level-gated, so both still fire at level 11 for the same fixture
//!   selections.
//!
//! It deliberately does not touch Arcane Bond, bloodline arcana, the
//! bloodline powers/spells/feats, or the spontaneous which-spells-known
//! selection / casting-execution burden (all stay named-but-unproven,
//! unchanged from levels 1-10), and it does not ground Sorcerer level 12+.
//! It also preserves the accepted Sorcerer level-1..level-10 truth
//! (unchanged), the Fighter negative control, and the multiclass negative
//! control.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const SORCERER_LEVEL10_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_sorcerer_level10_sd13_deterministic_input.txt"
);

const SORCERER_LEVEL11_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_sorcerer_level11_sd18_widening_deterministic_input.txt"
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

// ----- Base attack bonus at level 11 stays numerically unchanged -----

#[test]
fn sorcerer_level11_base_attack_bonus_is_grounded_and_unchanged() {
    let input = load(SORCERER_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(
        base_attack.value, 5,
        "Sorcerer level 11 1/2-BAB progression (11 / 2) must equal 5 — numerically unchanged \
         from level 10, an integer-division coincidence: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 11 stay numerically unchanged -----

#[test]
fn sorcerer_level11_base_saves_are_grounded_and_unchanged() {
    let input = load(SORCERER_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.sorcerer.base_save.fortitude");
    assert_eq!(
        fortitude.value, 3,
        "Sorcerer level 11 poor Fortitude (11/3) must equal 3 — unchanged from level 10"
    );

    let reflex = explanation(&computation, "class_chassis.sorcerer.base_save.reflex");
    assert_eq!(reflex.value, 3, "Sorcerer level 11 poor Reflex (11/3) must equal 3");

    let will = explanation(&computation, "class_chassis.sorcerer.base_save.will");
    assert_eq!(
        will.value, 7,
        "Sorcerer level 11 good Will (11/2+2) must equal 7 — unchanged from level 10"
    );
}

// ----- Base spells per day widen at level 11 -----

#[test]
fn sorcerer_level11_base_spells_per_day_match_the_raw_table_row() {
    let input = load(SORCERER_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_5"), 4),
        ],
        "level 11 (`6/6/6/6/4`): the 4th-level column rises from 5 to 6 and the 5th-level \
         column rises from 3 to 4; the 6th-level column stays inaccessible (no record)"
    );
}

// ----- Spells known widen at level 11 -----

#[test]
fn sorcerer_level11_spells_known_match_the_raw_table_row() {
    let input = load(SORCERER_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, KNOWN_PREFIX),
        vec![
            (format!("{KNOWN_PREFIX}spell_level_0"), 9),
            (format!("{KNOWN_PREFIX}spell_level_1"), 5),
            (format!("{KNOWN_PREFIX}spell_level_2"), 5),
            (format!("{KNOWN_PREFIX}spell_level_3"), 4),
            (format!("{KNOWN_PREFIX}spell_level_4"), 3),
            (format!("{KNOWN_PREFIX}spell_level_5"), 2),
        ],
        "level 11 (`9/5/5/4/3/2`): the 2nd/3rd/4th/5th-level columns each rise by one over \
         level 10's `9/5/4/3/2/1`; the 0th and 1st columns stay unchanged and no new 6th-level \
         column appears"
    );
}

// ----- Bloodline choice recognition still fires at level 11 -----

#[test]
fn sorcerer_level11_still_recognizes_the_bloodline_choice() {
    let input = load(SORCERER_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.sorcerer.bloodline_choice");
    assert_eq!(
        choice.value, 0,
        "bloodline choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Arcane"),
        "bloodline choice recognition must still name the Arcane bloodline at level 11: {}",
        choice.detail
    );
}

// ----- The spell-bearing baseline recognition and both burden diagnostics persist -----

#[test]
fn sorcerer_level11_still_recognizes_the_spell_bearing_baseline_and_claim_blocks_burdens() {
    let input = load(SORCERER_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.sorcerer"),
        "level-11 Sorcerer must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported"
            && d.claim_blocking),
        "level-11 Sorcerer must still claim-block on the Arcane Bond / bloodline progression \
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

// ----- No bloodline-spell record is fabricated at level 11 -----

#[test]
fn sorcerer_level11_does_not_fabricate_the_eleventh_level_bloodline_spell_entry() {
    let input = load(SORCERER_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("bloodline_power")
                || e.id.to_lowercase().contains("bloodline_spell")),
        "level-11 Sorcerer must not fabricate a bloodline-spell record — the entry is \
         bloodline-specific and left unproven, mirroring level 3/5/7/9: {:?}",
        computation.explanations
    );
}

// ----- Negative control: level 10 truth is unchanged by this widening -----

#[test]
fn sorcerer_level10_truth_is_unchanged_by_this_slice() {
    let input = load(SORCERER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(base_attack.value, 5, "Sorcerer level 10 base attack bonus must stay 5");

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 5),
            (format!("{PER_DAY_PREFIX}spell_level_5"), 3),
        ],
        "Sorcerer level 10 base spells per day must stay `6/6/6/5/3`"
    );
}

// ----- Negative control: level 21 stays unrecognized by this cycle -----
//
// SD18 widened Sorcerer support to level 20, PF1's level cap
// (tests/sd18_sorcerer_level20_widening.rs), so this boundary moved to 21
// (which does not exist in PF1), mirroring the exact same boundary move
// every other Barbarian/Bard/Cleric/Fighter/Paladin/Rogue/Ranger/Wizard
// level-20 widening cycle made to its own sibling level-11 widening test.

#[test]
fn sorcerer_level_21_is_not_promoted_by_this_slice() {
    let level_21 = SORCERER_LEVEL11_FIXTURE.replace("class:sorcerer:11", "class:sorcerer:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.sorcerer.")
                || e.id == "class_chassis.spell_baseline.sorcerer"),
        "level-21 Sorcerer must not gain any bounded sorcerer chassis explanation: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the sorcerer path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_sorcerer_level11_recognition() {
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
fn multiclass_sorcerer_level11_is_not_promoted_by_this_slice() {
    let multiclass = SORCERER_LEVEL11_FIXTURE.replace(
        "class_level=class:sorcerer:11",
        "class_level=class:sorcerer:11\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-11 widening -----

#[test]
fn matrix_sorcerer_row_names_level_11_widening() {
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
            .contains("sd18_sorcerer_level11_widening"),
        "sorcerer row must cite the live SD18 level-11 proof surface: {}",
        sorcerer.grounding_ref
    );
    let note = sorcerer.blocker_or_lossiness_note;
    assert!(
        note.contains("level 11") || note.contains("level-11"),
        "sorcerer partial note must name the level-11 widening: {note}"
    );
}
