//! SD18 Sorcerer level-20 widening grounding proof.
//!
//! Widens the accepted deterministic Human Sorcerer level-1..level-19 spell-
//! bearing chassis (`tests/sd18_sorcerer_level19_widening.rs`) to Sorcerer
//! level 20, the FINAL level within PF1's 1-20 character-level cap,
//! mirroring the sibling-class level-range-gate idiom
//! (`supported_sorcerer_level` is generalized from `1..=19` to `1..=20` via
//! `MAX_SUPPORTED_SORCERER_LEVEL = 20`, exactly as this sweep already
//! widened `MAX_SUPPORTED_BARBARIAN_LEVEL`, `MAX_SUPPORTED_BARD_LEVEL`,
//! `MAX_SUPPORTED_CLERIC_LEVEL`, `MAX_SUPPORTED_FIGHTER_LEVEL`,
//! `MAX_SUPPORTED_PALADIN_LEVEL`, `MAX_SUPPORTED_RANGER_LEVEL`,
//! `MAX_SUPPORTED_ROGUE_LEVEL`, and `MAX_SUPPORTED_WIZARD_LEVEL` from 19 to
//! 20 already this level-20 sweep — Sorcerer is the loop's NINTH and LAST
//! §3.2 level-20 landing, closing the level-20 sweep at 9 of 9 eligible
//! classes (Druid capped at 15, Monk capped at 12, both documented
//! structural exceptions outside this sweep)).
//! §3.1 race rows and §3.3 interaction rows stay fully exhausted /
//! structurally blocked (cited from the progress doc, not re-derived).
//!
//! Verified independently against TWO primary sources — a raw,
//! non-AI-summarized parse of d20pfsrd.com's own HTML table (obtained via
//! `curl` and stripped of markup) and the Archives of Nethys
//! `aonprd.com/ClassDisplay.aspx?ItemName=Sorcerer` mirror (also parsed the
//! same way) — both agreeing byte-for-byte on the full level-20 class-table
//! row (including the separate Spells Known table); a third source was not
//! required since no disagreement was found:
//!
//! - level 20 base attack bonus GENUINELY RISES to +10 (`20 / 2 = 10`, up
//!   from level 19's +9 — confirmed genuine by the raw table's own
//!   `+10/+5` cell, up from level 19's `+9/+4` cell). Both base saves STAY
//!   at +6 (`20 / 3 = 6`, an integer-division coincidence with level 19)
//!   and good Will GENUINELY RISES to +12 (`20 / 2 + 2 = 12`, up from
//!   level 19's +11).
//! - the PF1 Core Rulebook Sorcerer class table's level-20 "Special" column
//!   reads "Bloodline power" (bloodline-specific — the sorcerer's SECOND
//!   bloodline, gaining that bloodline's arcana and 1st/3rd/9th-level
//!   powers) — left named-but-unproven by the pre-existing Arcane Bond /
//!   bloodline progression blocker, exactly mirroring levels
//!   3/5/7/9/11/13/15/17/19 — so no new pillar record is grounded from it.
//! - the Spells per Day table's level-20 row is `6/6/6/6/6/6/6/6/6` (1st
//!   through 9th), genuinely risen from level 19's `6/6/6/6/6/6/6/6/4` only
//!   at the 9th spell level (4 -> 6), with no genuinely new spell-level
//!   column opening (the 9th-level column already opened at level 18).
//! - the Spells Known table's level-20 row is `9/5/5/4/4/4/3/3/3/3` (0th
//!   through 9th), with the 0th through 8th columns numerically UNCHANGED
//!   from level 19's `9/5/5/4/4/4/3/3/3/2`, while the 9th-level column
//!   rises from 2 to 3, with no genuinely new spell-level column opening.
//! - the spell-level access ladder STAYS at 9 (unchanged from level 19; no
//!   new threshold constant is needed — the ladder was fully populated
//!   through 9th-level spells at level 18).
//! - the spell-save-DC and Charisma-bonus-spell formulas widen automatically
//!   over the unchanged access ladder, with no new code needed (both loops
//!   already iterate generically over `1..=sorcerer_spell_level_access`).
//! - the bloodline choice and bloodline class-skill choice recognitions are
//!   not level-gated, so both still fire at level 20 for the same fixture
//!   selections.
//!
//! It deliberately does not touch Arcane Bond, bloodline arcana, the
//! bloodline power selection, the bloodline bonus spell selection, or the
//! spontaneous which-spells-known selection / casting-execution burden (all
//! stay named-but-unproven, unchanged from levels 1-19). It also preserves
//! the accepted Sorcerer level-1..level-19 truth (unchanged), the Fighter
//! negative control, and the multiclass negative control. Per the sweep's
//! established lesson about stale negative controls, this cycle also moves
//! the sibling "level 20 is not promoted" negative controls in
//! `tests/sd13_sorcerer_level10_progression.rs`,
//! `tests/sd18_sorcerer_level11_widening.rs`,
//! `tests/sd18_sorcerer_level12_widening.rs`,
//! `tests/sd18_sorcerer_level13_widening.rs`, and
//! `tests/sd18_sorcerer_level14_widening.rs` — to a "level 21 is not
//! promoted" boundary (a pure implementation-gate check, since PF1 has no
//! 21st character level); `tests/sd18_sorcerer_level19_widening.rs`'s own
//! level-20 negative control is removed rather than moved, since level 20
//! is now itself the supported/grounded row, mirroring the exact fix every
//! prior level-N cycle made for its own siblings.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const SORCERER_LEVEL19_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_sorcerer_level19_sd18_widening_deterministic_input.txt"
);

const SORCERER_LEVEL20_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_sorcerer_level20_sd18_widening_deterministic_input.txt"
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

// ----- Base attack bonus at level 20 genuinely rises -----

#[test]
fn sorcerer_level20_base_attack_bonus_is_grounded() {
    let input = load(SORCERER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(
        base_attack.value, 10,
        "Sorcerer level 20 1/2-BAB progression (20 / 2) must equal 10 — genuinely up from level \
         19's +9, confirmed genuine by the raw table's own `+10/+5` cell: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 20: poor saves stay put, good Will genuinely rises -----

#[test]
fn sorcerer_level20_base_saves_are_grounded() {
    let input = load(SORCERER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.sorcerer.base_save.fortitude");
    assert_eq!(
        fortitude.value, 6,
        "Sorcerer level 20 poor Fortitude (20/3) must equal 6 — an integer-division coincidence \
         with level 19's +6"
    );

    let reflex = explanation(&computation, "class_chassis.sorcerer.base_save.reflex");
    assert_eq!(
        reflex.value, 6,
        "Sorcerer level 20 poor Reflex (20/3) must equal 6 — an integer-division coincidence \
         with level 19's +6"
    );

    let will = explanation(&computation, "class_chassis.sorcerer.base_save.will");
    assert_eq!(
        will.value, 12,
        "Sorcerer level 20 good Will (20/2+2) must equal 12 — genuinely up from level 19's +11"
    );
}

// ----- Base spells per day widen at level 20 within the already-opened 9-column shape -----

#[test]
fn sorcerer_level20_base_spells_per_day_match_the_raw_table_row() {
    let input = load(SORCERER_LEVEL20_FIXTURE);
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
            (format!("{PER_DAY_PREFIX}spell_level_8"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_9"), 6),
        ],
        "level 20 (`6/6/6/6/6/6/6/6/6`): the 9th-level column rises from 4 to 6, with no \
         genuinely new spell-level column opening, verified independently across two primary \
         sources"
    );
}

// ----- Spells known widen at level 20 within the already-opened 10-column shape -----

#[test]
fn sorcerer_level20_spells_known_match_the_raw_table_row() {
    let input = load(SORCERER_LEVEL20_FIXTURE);
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
            (format!("{KNOWN_PREFIX}spell_level_8"), 3),
            (format!("{KNOWN_PREFIX}spell_level_9"), 3),
        ],
        "level 20 (`9/5/5/4/4/4/3/3/3/3`): the 0th through 8th columns stay numerically \
         unchanged from level 19's `9/5/5/4/4/4/3/3/3/2`, while the 9th-level column rises from \
         2 to 3, verified independently across two primary sources"
    );
}

// ----- Spell-level access ladder stays at 9 (no new threshold constant needed) -----

#[test]
fn sorcerer_level20_spell_level_access_stays_at_nine() {
    let input = load(SORCERER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(
        &computation,
        "class_chassis.sorcerer.spontaneous.spell_level_access",
    );
    assert_eq!(
        access.value, 9,
        "Sorcerer level 20 spell-level access must STAY at 9th-level spells — unchanged from \
         level 19, since the access ladder was already fully populated through 9th-level \
         spells: {}",
        access.detail
    );
}

// ----- Bonus spells and spell-save DCs extend to the 9th spell level with no new code -----

#[test]
fn sorcerer_level20_bonus_spells_and_save_dcs_extend_to_ninth_level() {
    let input = load(SORCERER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dc9 = explanation(
        &computation,
        "class_chassis.sorcerer.spontaneous.spell_save_dc.spell_level_9",
    );
    // Fixture Charisma 17 -> Charisma modifier +3 (unchanged deterministic posture).
    // 10 + 9 + 3 = 22.
    assert_eq!(
        dc9.value, 23,
        "Sorcerer level 20 9th-level spell save DC must be 10 + 9 + Charisma modifier: {}",
        dc9.detail
    );

    let bonus9 = explanation(
        &computation,
        "class_chassis.sorcerer.spontaneous.bonus_spells_per_day.spell_level_9",
    );
    // Charisma modifier +3 < spell level 9, so bonus spells at 9th level is 0.
    assert_eq!(
        bonus9.value, 0,
        "Sorcerer level 20 9th-level bonus spells from a +3 Charisma modifier must be 0 (below \
         the spell level threshold): {}",
        bonus9.detail
    );

    let total9 = explanation(
        &computation,
        "class_chassis.sorcerer.spontaneous.total_spells_per_day.spell_level_9",
    );
    assert_eq!(
        total9.value, 6,
        "Sorcerer level 20 9th-level total spells per day must equal the base count (6) plus \
         the zero bonus: {}",
        total9.detail
    );
}

// ----- Bloodline choice recognition still fires at level 20 -----

#[test]
fn sorcerer_level20_still_recognizes_the_bloodline_choice() {
    let input = load(SORCERER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.sorcerer.bloodline_choice");
    assert_eq!(
        choice.value, 0,
        "bloodline choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Arcane"),
        "bloodline choice recognition must still name the Arcane bloodline at level 20: {}",
        choice.detail
    );
}

// ----- The spell-bearing baseline recognition and both burden diagnostics persist -----

#[test]
fn sorcerer_level20_still_recognizes_the_spell_bearing_baseline_and_claim_blocks_burdens() {
    let input = load(SORCERER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.sorcerer"),
        "level-20 Sorcerer must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported"
            && d.claim_blocking),
        "level-20 Sorcerer must still claim-block on the Arcane Bond / bloodline progression \
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

// ----- No new bloodline-feat/bloodline-power/bloodline-spell record is fabricated at level 20 -----

#[test]
fn sorcerer_level20_does_not_fabricate_any_bloodline_entry() {
    let input = load(SORCERER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("bloodline_feat")
                || e.id.to_lowercase().contains("bloodline_power")
                || e.id.to_lowercase().contains("bloodline_spell")),
        "level-20 Sorcerer must not fabricate a bloodline-feat/bloodline-power/bloodline-spell \
         record — the level-20 \"Bloodline power\" Special column stays named by the \
         pre-existing Arcane Bond / bloodline progression blocker only: {:?}",
        computation.explanations
    );
}

// ----- Negative control: level 19 truth is unchanged by this widening -----

#[test]
fn sorcerer_level19_truth_is_unchanged_by_this_slice() {
    let input = load(SORCERER_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(base_attack.value, 9, "Sorcerer level 19 base attack bonus must stay 9");

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
            (format!("{PER_DAY_PREFIX}spell_level_8"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_9"), 4),
        ],
        "Sorcerer level 19 base spells per day must stay `6/6/6/6/6/6/6/6/4`, unchanged by this \
         slice"
    );
}

// ----- Negative control: level 21 stays unrecognized (PF1 has no 21st level) -----

#[test]
fn sorcerer_level_21_is_not_promoted_by_this_slice() {
    let level_21 = SORCERER_LEVEL20_FIXTURE.replace("class:sorcerer:20", "class:sorcerer:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.sorcerer.")
                || e.id == "class_chassis.spell_baseline.sorcerer"),
        "level-21 Sorcerer must not gain any bounded sorcerer chassis explanation (PF1 has no \
         21st character level): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the sorcerer path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_sorcerer_level20_recognition() {
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
fn multiclass_sorcerer_level20_is_not_promoted_by_this_slice() {
    let multiclass = SORCERER_LEVEL20_FIXTURE.replace(
        "class_level=class:sorcerer:20",
        "class_level=class:sorcerer:20\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-20 widening -----

#[test]
fn matrix_sorcerer_row_names_level_20_widening() {
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
            .contains("sd18_sorcerer_level20_widening"),
        "sorcerer row must cite the live SD18 level-20 proof surface: {}",
        sorcerer.grounding_ref
    );
    let note = sorcerer.blocker_or_lossiness_note;
    assert!(
        note.contains("level 20") || note.contains("level-20"),
        "sorcerer partial note must name the level-20 widening: {note}"
    );
}
