//! SD18 Sorcerer level-13 widening grounding proof.
//!
//! Widens the accepted deterministic Human Sorcerer level-1..level-12 spell-
//! bearing chassis (`tests/sd18_sorcerer_level12_widening.rs`) to Sorcerer
//! level 13, mirroring the sibling-class level-range-gate idiom
//! (`supported_sorcerer_level` is generalized from `1..=12` to `1..=13` via
//! `MAX_SUPPORTED_SORCERER_LEVEL = 13`, exactly as prior cycles widened
//! `MAX_SUPPORTED_BARBARIAN_LEVEL`, `MAX_SUPPORTED_BARD_LEVEL`,
//! `MAX_SUPPORTED_CLERIC_LEVEL`, `MAX_SUPPORTED_DRUID_LEVEL`,
//! `MAX_SUPPORTED_FIGHTER_LEVEL`, `MAX_SUPPORTED_PALADIN_LEVEL`,
//! `MAX_SUPPORTED_ROGUE_LEVEL`, and `MAX_SUPPORTED_RANGER_LEVEL`, all from 12
//! to 13). §3.1 race rows and §3.3 interaction rows stay fully exhausted /
//! structurally blocked (cited from the progress doc, not re-derived). This
//! cycle re-verified live which §3.2 classes still sat at the level-12
//! ceiling: Sorcerer and Wizard both did. Per the brief's explicit
//! alphabetical tie-break, this cycle picks **Sorcerer**.
//!
//! Three PF1 CRB primary sources (d20pfsrd, the Archives of Nethys
//! aonprd.com mirror, and legacy.aonprd.com) were read directly before
//! writing any code or test, and all three fetches agree byte-for-byte:
//!
//! - level 13 base attack bonus stays numerically at +6 (`13 / 2 = 6`, an
//!   integer-division coincidence with level 12, not a sign the formula
//!   stopped scaling) and all three base saves stay numerically unchanged
//!   too: +4 Fortitude (poor, `13 / 3 = 4`), +4 Reflex (poor, `13 / 3 = 4`),
//!   and +8 Will (good, `13 / 2 + 2 = 8`) — all four values via the same
//!   formulas already grounded at levels 1-12, not re-derived.
//! - the PF1 Core Rulebook Sorcerer class table's level-13 "Special" column
//!   reads "Bloodline feat, bloodline spell" (verified independently
//!   against all three sources, checked rather than assumed away) — the
//!   sorcerer's second bloodline feat grant (bloodline feats are first
//!   granted at 7th level and every six levels thereafter: 7, 13, 19,
//!   confirmed by this codebase's own pre-existing level-7 doc trail on
//!   `explain_sorcerer_level1_spell_baseline`) and a further bloodline
//!   spell grant. Exactly like the level-3/5/7/11 "Bloodline power"/
//!   "Bloodline spell" entries, both level-13 grants are bloodline-specific
//!   (varying per bloodline, e.g. the Arcane bloodline's own 13th-level
//!   bloodline spell is true seeing) and neither is flat/identity-shaped,
//!   so this slice grounds no new pillar from the Special column either —
//!   both entries stay named by the existing Arcane Bond / bloodline
//!   progression blocker's "bonus spells/feats at 3rd+ level" language,
//!   unchanged.
//! - the Spells per Day table's level-13 row is `6/6/6/6/6/4` (1st/2nd/3rd/
//!   4th/5th/6th), genuinely risen from level 12's `6/6/6/6/5/3` at the 5th
//!   (5 -> 6) and 6th (3 -> 4) spell levels, with no genuinely new spell
//!   level column (the 7th-level column does not open until level 14) —
//!   verified identically on all three fetches.
//! - the Spells Known table's level-13 row is `9/5/5/4/4/3/2` (0th/1st/2nd/
//!   3rd/4th/5th/6th), genuinely risen from level 12's `9/5/5/4/3/2/1` at
//!   the 4th (3 -> 4), 5th (2 -> 3), and 6th (1 -> 2) spell levels, with the
//!   0th through 3rd columns numerically unchanged — verified identically
//!   on all three fetches.
//! - the spell-level access ladder stays at 6 (unchanged from level 12; the
//!   7th-level threshold is not reached until level 14).
//! - the spell-save-DC and Charisma-bonus-spell formulas are unchanged
//!   (live arithmetic over the already-grounded access ladder and the
//!   fixture's Charisma modifier).
//! - the bloodline choice and bloodline class-skill choice recognitions are
//!   not level-gated, so both still fire at level 13 for the same fixture
//!   selections.
//!
//! It deliberately does not touch Arcane Bond, bloodline arcana, the
//! bloodline feat selection, the bloodline bonus spell selection, or the
//! spontaneous which-spells-known selection / casting-execution burden (all
//! stay named-but-unproven, unchanged from levels 1-12), and it does not
//! ground Sorcerer level 14+. It also preserves the accepted Sorcerer
//! level-1..level-12 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control. Per the brief's lesson about stale
//! negative controls, this cycle also moves the sibling "level 13 is not
//! promoted" negative controls in `tests/sd13_sorcerer_level10_progression.rs`
//! and `tests/sd18_sorcerer_level12_widening.rs` to a "level 14 is not
//! promoted" boundary in the same commit.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const SORCERER_LEVEL12_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_sorcerer_level12_sd18_widening_deterministic_input.txt"
);

const SORCERER_LEVEL13_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_sorcerer_level13_sd18_widening_deterministic_input.txt"
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

// ----- Base attack bonus at level 13 stays numerically unchanged (integer-division coincidence) -----

#[test]
fn sorcerer_level13_base_attack_bonus_is_grounded() {
    let input = load(SORCERER_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(
        base_attack.value, 6,
        "Sorcerer level 13 1/2-BAB progression (13 / 2) must equal 6 — numerically unchanged \
         from level 12, an integer-division coincidence, not a sign the formula stopped \
         scaling: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 13 stay numerically unchanged -----

#[test]
fn sorcerer_level13_base_saves_are_grounded() {
    let input = load(SORCERER_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.sorcerer.base_save.fortitude");
    assert_eq!(
        fortitude.value, 4,
        "Sorcerer level 13 poor Fortitude (13/3) must equal 4 — unchanged from level 12"
    );

    let reflex = explanation(&computation, "class_chassis.sorcerer.base_save.reflex");
    assert_eq!(
        reflex.value, 4,
        "Sorcerer level 13 poor Reflex (13/3) must equal 4 — unchanged from level 12"
    );

    let will = explanation(&computation, "class_chassis.sorcerer.base_save.will");
    assert_eq!(
        will.value, 8,
        "Sorcerer level 13 good Will (13/2+2) must equal 8 — unchanged from level 12"
    );
}

// ----- Base spells per day widen at level 13, with no new spell-level column -----

#[test]
fn sorcerer_level13_base_spells_per_day_match_the_raw_table_row() {
    let input = load(SORCERER_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_5"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_6"), 4),
        ],
        "level 13 (`6/6/6/6/6/4`): the 5th-level column rises from 5 to 6 and the 6th-level \
         column rises from 3 to 4 — no genuinely new spell-level column (the 7th-level column \
         does not open until level 14)"
    );
}

// ----- Spells known widen at level 13, with no new spell-level column -----

#[test]
fn sorcerer_level13_spells_known_match_the_raw_table_row() {
    let input = load(SORCERER_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, KNOWN_PREFIX),
        vec![
            (format!("{KNOWN_PREFIX}spell_level_0"), 9),
            (format!("{KNOWN_PREFIX}spell_level_1"), 5),
            (format!("{KNOWN_PREFIX}spell_level_2"), 5),
            (format!("{KNOWN_PREFIX}spell_level_3"), 4),
            (format!("{KNOWN_PREFIX}spell_level_4"), 4),
            (format!("{KNOWN_PREFIX}spell_level_5"), 3),
            (format!("{KNOWN_PREFIX}spell_level_6"), 2),
        ],
        "level 13 (`9/5/5/4/4/3/2`): the 4th, 5th, and 6th-level columns each rise by one from \
         level 12's `9/5/5/4/3/2/1`, while the 0th through 3rd columns stay numerically \
         unchanged"
    );
}

// ----- Spell-level access ladder stays at 6 at level 13 (unchanged from level 12) -----

#[test]
fn sorcerer_level13_spell_level_access_stays_at_six() {
    let input = load(SORCERER_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(
        &computation,
        "class_chassis.sorcerer.spontaneous.spell_level_access",
    );
    assert_eq!(
        access.value, 6,
        "Sorcerer level 13 spell-level access must stay at 6th-level spells — the 7th-level \
         threshold is not reached until level 14: {}",
        access.detail
    );
}

// ----- Bloodline choice recognition still fires at level 13 -----

#[test]
fn sorcerer_level13_still_recognizes_the_bloodline_choice() {
    let input = load(SORCERER_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.sorcerer.bloodline_choice");
    assert_eq!(
        choice.value, 0,
        "bloodline choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Arcane"),
        "bloodline choice recognition must still name the Arcane bloodline at level 13: {}",
        choice.detail
    );
}

// ----- The spell-bearing baseline recognition and both burden diagnostics persist -----

#[test]
fn sorcerer_level13_still_recognizes_the_spell_bearing_baseline_and_claim_blocks_burdens() {
    let input = load(SORCERER_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.sorcerer"),
        "level-13 Sorcerer must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported"
            && d.claim_blocking),
        "level-13 Sorcerer must still claim-block on the Arcane Bond / bloodline progression \
         burden (this is where the level-13 Bloodline feat / bloodline spell grant stays \
         named): {:?}",
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

// ----- No bloodline-feat/bloodline-spell record is fabricated at level 13 -----

#[test]
fn sorcerer_level13_does_not_fabricate_any_bloodline_entry() {
    let input = load(SORCERER_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("bloodline_feat")
                || e.id.to_lowercase().contains("bloodline_power")
                || e.id.to_lowercase().contains("bloodline_spell")),
        "level-13 Sorcerer must not fabricate a bloodline-feat/bloodline-spell record — the \
         Special column's \"Bloodline feat, bloodline spell\" grants are bloodline-specific and \
         stay named by the existing blocker, not grounded here: {:?}",
        computation.explanations
    );
}

// ----- Negative control: level 12 truth is unchanged by this widening -----

#[test]
fn sorcerer_level12_truth_is_unchanged_by_this_slice() {
    let input = load(SORCERER_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(base_attack.value, 6, "Sorcerer level 12 base attack bonus must stay 6");

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_5"), 5),
            (format!("{PER_DAY_PREFIX}spell_level_6"), 3),
        ],
        "Sorcerer level 12 base spells per day must stay `6/6/6/6/5/3`"
    );
}

// ----- Negative control: level 21 stays unrecognized by this cycle -----
//
// SD18 widened Sorcerer support to level 20, PF1's level cap
// (tests/sd18_sorcerer_level20_widening.rs), so this boundary moved to 21
// (which does not exist in PF1), mirroring the exact same boundary move made
// to every other sorcerer sibling test file this cycle.

#[test]
fn sorcerer_level_21_is_not_promoted_by_this_slice() {
    let level_21 = SORCERER_LEVEL13_FIXTURE.replace("class:sorcerer:13", "class:sorcerer:21");
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
fn fighter_does_not_gain_sorcerer_level13_recognition() {
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
fn multiclass_sorcerer_level13_is_not_promoted_by_this_slice() {
    let multiclass = SORCERER_LEVEL13_FIXTURE.replace(
        "class_level=class:sorcerer:13",
        "class_level=class:sorcerer:13\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-13 widening -----

#[test]
fn matrix_sorcerer_row_names_level_13_widening() {
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
            .contains("sd18_sorcerer_level13_widening"),
        "sorcerer row must cite the live SD18 level-13 proof surface: {}",
        sorcerer.grounding_ref
    );
    let note = sorcerer.blocker_or_lossiness_note;
    assert!(
        note.contains("level 13") || note.contains("level-13"),
        "sorcerer partial note must name the level-13 widening: {note}"
    );
}
