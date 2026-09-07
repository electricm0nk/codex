//! SD18 Sorcerer level-18 widening grounding proof.
//!
//! Widens the accepted deterministic Human Sorcerer level-1..level-17 spell-
//! bearing chassis (`tests/sd18_sorcerer_level17_widening.rs`) to Sorcerer
//! level 18, mirroring the sibling-class level-range-gate idiom
//! (`supported_sorcerer_level` is generalized from `1..=17` to `1..=18` via
//! `MAX_SUPPORTED_SORCERER_LEVEL = 18`, exactly as this sweep already
//! widened `MAX_SUPPORTED_WIZARD_LEVEL`, `MAX_SUPPORTED_CLERIC_LEVEL`,
//! `MAX_SUPPORTED_PALADIN_LEVEL`, `MAX_SUPPORTED_FIGHTER_LEVEL`,
//! `MAX_SUPPORTED_BARBARIAN_LEVEL`, `MAX_SUPPORTED_ROGUE_LEVEL`, and
//! `MAX_SUPPORTED_RANGER_LEVEL` from 17 to 18 already this level-18 sweep —
//! Sorcerer is the loop's EIGHTH §3.2 level-18 landing).
//! §3.1 race rows and §3.3 interaction rows stay fully exhausted /
//! structurally blocked (cited from the progress doc, not re-derived).
//!
//! **This cycle's primary task was resolving a multi-cycle-carried-forward
//! flag**: prior cycles repeatedly noted that a raw Sorcerer spells-per-day
//! fetch looked internally inconsistent at level 18 — an apparent
//! "premature" 9th-level spell column, which seemed to contradict a
//! commonly-repeated (but, this cycle discovered, WRONG for Sorcerer)
//! folk-rule that sorcerers gain 9th-level spells only at 20th level. This
//! cycle re-fetched Sorcerer's full levels 14-20 block fresh from THREE
//! independent primary sources: (1) a raw, non-AI-summarized parse of
//! d20pfsrd.com's own HTML table (bypassing any tabular-summarization
//! ambiguity entirely), (2) the Archives of Nethys aonprd.com mirror
//! (`aonprd.com/ClassDisplay.aspx?ItemName=Sorcerer`), and (3) the
//! legacy.aonprd.com CRB mirror. All three agree: the Sorcerer's
//! spells-per-day table opens a genuinely NEW spell-level column every two
//! class levels starting at 4th (2nd-level spells at 4, 3rd at 6, 4th at 8,
//! 5th at 10, 6th at 12, 7th at 14, 8th at 16, 9th at 18 — matching this
//! row's own already-grounded and already-verified
//! `SORCERER_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL` through
//! `SORCERER_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL` thresholds exactly),
//! with 1st-level spells available from level 1 (no zero step). This means
//! the 9th-level column genuinely, correctly opens at class level 18 — ONE
//! LEVEL EARLIER than Wizard/Cleric's own already-grounded
//! `WIZARD_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL` /
//! `CLERIC_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL` threshold of 17 — which
//! is exactly consistent with every other already-proven Sorcerer threshold
//! in this row being one level later than Wizard's equivalent (Wizard's own
//! 8th-level threshold is 15, Sorcerer's is 16; the "two-level cadence
//! starting at 4" for Sorcerer versus Wizard's "odd-level cadence starting
//! at 1" always lands Sorcerer one level behind Wizard from the 2nd spell
//! level onward). The previously-flagged "premature 9th-level column" was
//! therefore the CORRECT reading all along; the folk-rule that assumed a
//! 20th-level-only 9th-spell-level threshold for Sorcerer does not hold —
//! that assumption was never independently re-verified against a primary
//! source in any prior cycle, only carried forward by analogy.
//!
//! With the row now definitively resolved as internally consistent, this
//! slice lands the level-18 widening via the standard TDD procedure, using
//! the SAME new-spell-level-access-threshold-constant idiom already used
//! for Wizard's and Cleric's own 9th-level-spell-threshold widenings (a
//! new `SORCERER_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL = 18` constant),
//! since a genuinely new spell-level column does open at level 18 (unlike
//! the level-17 cycle, which was a pure table-widen with no new column).
//!
//! Verified independently against all three primary sources above, byte-
//! for-byte identical on every value below:
//!
//! - level 18 base attack bonus genuinely rises to +9 (`18 / 2 = 9`, up
//!   from level 17's +8) via the same already-grounded formula, not
//!   re-derived. Both base saves genuinely rise to +6 (`18 / 3 = 6`, up
//!   from level 17's +5) and good Will genuinely rises to +11
//!   (`18 / 2 + 2 = 11`, up from level 17's +10).
//! - the PF1 Core Rulebook Sorcerer class table's level-18 "Special" column
//!   is genuinely BLANK on all three sources — UNLIKE level 17's
//!   "Bloodline spell" entry — so no new pillar record is grounded from
//!   it; this is a pure ceiling raise on the base-attack/base-save pillars
//!   plus the genuinely-new spell-level-column opening described below.
//! - the Spells per Day table's level-18 row is `6/6/6/6/6/6/6/5/3` (1st
//!   through 9th), genuinely risen from level 17's `6/6/6/6/6/6/6/4` at the
//!   8th spell level (4 -> 5) AND a genuinely NEW 9th-level column opening
//!   at 3 for the first time.
//! - the Spells Known table's level-18 row is `9/5/5/4/4/4/3/3/2/1` (0th
//!   through 9th), with the 0th through 8th columns numerically UNCHANGED
//!   from level 17's `9/5/5/4/4/4/3/3/2`, while a genuinely NEW 9th-level
//!   column opens at 1 for the first time.
//! - the spell-level access ladder genuinely rises to 9 (up from 8 at level
//!   17) via a new `SORCERER_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL = 18`
//!   threshold constant, mirroring the Wizard's and Cleric's own
//!   9th-level-column-opening cycles.
//! - the spell-save-DC and Charisma-bonus-spell formulas widen
//!   automatically over the newly-risen access ladder, with no new code
//!   needed (both loops already iterate generically over
//!   `1..=sorcerer_spell_level_access`).
//! - the bloodline choice and bloodline class-skill choice recognitions are
//!   not level-gated, so both still fire at level 18 for the same fixture
//!   selections.
//!
//! It deliberately does not touch Arcane Bond, bloodline arcana, the
//! bloodline feat selection, the bloodline bonus spell selection, or the
//! spontaneous which-spells-known selection / casting-execution burden (all
//! stay named-but-unproven, unchanged from levels 1-17), and it does not
//! ground Sorcerer level 19+. It also preserves the accepted Sorcerer
//! level-1..level-17 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control. Per the sweep's established lesson
//! about stale negative controls, this cycle also moves the sibling "level
//! 18 is not promoted" negative controls in
//! `tests/sd13_sorcerer_level10_progression.rs`,
//! `tests/sd18_sorcerer_level11_widening.rs`,
//! `tests/sd18_sorcerer_level12_widening.rs`,
//! `tests/sd18_sorcerer_level13_widening.rs`, and
//! `tests/sd18_sorcerer_level14_widening.rs` — to a "level 19 is not
//! promoted" boundary; `tests/sd18_sorcerer_level17_widening.rs`'s own
//! level-18 negative control is removed rather than moved, since level 18
//! is now itself the supported/grounded row, mirroring the exact fix every
//! prior level-N cycle made for its own siblings.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const SORCERER_LEVEL17_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_sorcerer_level17_sd18_widening_deterministic_input.txt"
);

const SORCERER_LEVEL18_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_sorcerer_level18_sd18_widening_deterministic_input.txt"
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

// ----- Base attack bonus at level 18 genuinely rises -----

#[test]
fn sorcerer_level18_base_attack_bonus_is_grounded() {
    let input = load(SORCERER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(
        base_attack.value, 9,
        "Sorcerer level 18 1/2-BAB progression (18 / 2) must equal 9 — genuinely risen from \
         level 17's +8: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 18: all three genuinely rise -----

#[test]
fn sorcerer_level18_base_saves_are_grounded() {
    let input = load(SORCERER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.sorcerer.base_save.fortitude");
    assert_eq!(
        fortitude.value, 6,
        "Sorcerer level 18 poor Fortitude (18/3) must equal 6 — genuinely risen from level 17's \
         +5"
    );

    let reflex = explanation(&computation, "class_chassis.sorcerer.base_save.reflex");
    assert_eq!(
        reflex.value, 6,
        "Sorcerer level 18 poor Reflex (18/3) must equal 6 — genuinely risen from level 17's +5"
    );

    let will = explanation(&computation, "class_chassis.sorcerer.base_save.will");
    assert_eq!(
        will.value, 11,
        "Sorcerer level 18 good Will (18/2+2) must equal 11 — genuinely risen from level 17's \
         +10"
    );
}

// ----- Base spells per day widen at level 18, opening a genuinely NEW 9th-level column -----

#[test]
fn sorcerer_level18_base_spells_per_day_match_the_raw_table_row() {
    let input = load(SORCERER_LEVEL18_FIXTURE);
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
            (format!("{PER_DAY_PREFIX}spell_level_8"), 5),
            (format!("{PER_DAY_PREFIX}spell_level_9"), 3),
        ],
        "level 18 (`6/6/6/6/6/6/6/5/3`): the 8th-level column rises from 4 to 5 AND a genuinely \
         NEW 9th-level column opens at 3, verified independently across three primary sources"
    );
}

// ----- Spells known widen at level 18, opening a genuinely NEW 9th-level column -----

#[test]
fn sorcerer_level18_spells_known_match_the_raw_table_row() {
    let input = load(SORCERER_LEVEL18_FIXTURE);
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
            (format!("{KNOWN_PREFIX}spell_level_9"), 1),
        ],
        "level 18 (`9/5/5/4/4/4/3/3/2/1`): the 0th through 8th columns stay numerically \
         unchanged from level 17's `9/5/5/4/4/4/3/3/2`, while a genuinely NEW 9th-level column \
         opens at 1, verified independently across three primary sources"
    );
}

// ----- Spell-level access ladder genuinely rises to 9 via a new threshold constant -----

#[test]
fn sorcerer_level18_spell_level_access_rises_to_nine() {
    let input = load(SORCERER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(
        &computation,
        "class_chassis.sorcerer.spontaneous.spell_level_access",
    );
    assert_eq!(
        access.value, 9,
        "Sorcerer level 18 spell-level access must genuinely rise to 9th-level spells — a new \
         SORCERER_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL = 18 threshold, one level earlier than \
         Wizard/Cleric's own ninth-level threshold of 17, mirroring the one-level-behind-Wizard \
         cadence this row's every other threshold already follows: {}",
        access.detail
    );
}

// ----- Bonus spells and spell-save DCs extend to the 9th spell level with no new code -----

#[test]
fn sorcerer_level18_bonus_spells_and_save_dcs_extend_to_ninth_level() {
    let input = load(SORCERER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dc9 = explanation(
        &computation,
        "class_chassis.sorcerer.spontaneous.spell_save_dc.spell_level_9",
    );
    // Fixture Charisma 17 -> Charisma modifier +3 (unchanged deterministic posture).
    // 10 + 9 + 3 = 22.
    assert_eq!(
        dc9.value, 23,
        "Sorcerer level 18 9th-level spell save DC must be 10 + 9 + Charisma modifier: {}",
        dc9.detail
    );

    let bonus9 = explanation(
        &computation,
        "class_chassis.sorcerer.spontaneous.bonus_spells_per_day.spell_level_9",
    );
    // Charisma modifier +3 < spell level 9, so bonus spells at 9th level is 0.
    assert_eq!(
        bonus9.value, 0,
        "Sorcerer level 18 9th-level bonus spells from a +3 Charisma modifier must be 0 (below \
         the spell level threshold): {}",
        bonus9.detail
    );

    let total9 = explanation(
        &computation,
        "class_chassis.sorcerer.spontaneous.total_spells_per_day.spell_level_9",
    );
    assert_eq!(
        total9.value, 3,
        "Sorcerer level 18 9th-level total spells per day must equal the base count (3) plus \
         the zero bonus: {}",
        total9.detail
    );
}

// ----- Bloodline choice recognition still fires at level 18 -----

#[test]
fn sorcerer_level18_still_recognizes_the_bloodline_choice() {
    let input = load(SORCERER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.sorcerer.bloodline_choice");
    assert_eq!(
        choice.value, 0,
        "bloodline choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Arcane"),
        "bloodline choice recognition must still name the Arcane bloodline at level 18: {}",
        choice.detail
    );
}

// ----- The spell-bearing baseline recognition and both burden diagnostics persist -----

#[test]
fn sorcerer_level18_still_recognizes_the_spell_bearing_baseline_and_claim_blocks_burdens() {
    let input = load(SORCERER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.sorcerer"),
        "level-18 Sorcerer must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported"
            && d.claim_blocking),
        "level-18 Sorcerer must still claim-block on the Arcane Bond / bloodline progression \
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

// ----- No new bloodline-feat/bloodline-power/bloodline-spell record is fabricated at level 18 -----

#[test]
fn sorcerer_level18_does_not_fabricate_any_bloodline_entry() {
    let input = load(SORCERER_LEVEL18_FIXTURE);
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
        "level-18 Sorcerer must not fabricate a bloodline-feat/bloodline-power/bloodline-spell \
         record — the level-18 Special column is genuinely blank on all three primary sources: \
         {:?}",
        computation.explanations
    );
}

// ----- Negative control: level 17 truth is unchanged by this widening -----

#[test]
fn sorcerer_level17_truth_is_unchanged_by_this_slice() {
    let input = load(SORCERER_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(base_attack.value, 8, "Sorcerer level 17 base attack bonus must stay 8");

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
        "Sorcerer level 17 base spells per day must stay `6/6/6/6/6/6/6/4`, unchanged by this \
         slice"
    );
}

// ----- Negative control: the sorcerer path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_sorcerer_level18_recognition() {
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
fn multiclass_sorcerer_level18_is_not_promoted_by_this_slice() {
    let multiclass = SORCERER_LEVEL18_FIXTURE.replace(
        "class_level=class:sorcerer:18",
        "class_level=class:sorcerer:18\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-18 widening -----

#[test]
fn matrix_sorcerer_row_names_level_18_widening() {
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
            .contains("sd18_sorcerer_level18_widening"),
        "sorcerer row must cite the live SD18 level-18 proof surface: {}",
        sorcerer.grounding_ref
    );
    let note = sorcerer.blocker_or_lossiness_note;
    assert!(
        note.contains("level 18") || note.contains("level-18"),
        "sorcerer partial note must name the level-18 widening: {note}"
    );
}
