//! SD18 Sorcerer level-12 widening grounding proof.
//!
//! Widens the accepted deterministic Human Sorcerer level-1..level-11 spell-
//! bearing chassis (`tests/sd18_sorcerer_level11_widening.rs`) to Sorcerer
//! level 12, mirroring the sibling-class level-range-gate idiom
//! (`supported_sorcerer_level` is generalized from `1..=11` to `1..=12` via
//! `MAX_SUPPORTED_SORCERER_LEVEL = 12`, exactly as prior cycles widened
//! `MAX_SUPPORTED_BARBARIAN_LEVEL`, `MAX_SUPPORTED_BARD_LEVEL`,
//! `MAX_SUPPORTED_CLERIC_LEVEL`, `MAX_SUPPORTED_DRUID_LEVEL`,
//! `MAX_SUPPORTED_FIGHTER_LEVEL`, `MAX_SUPPORTED_MONK_LEVEL`,
//! `MAX_SUPPORTED_PALADIN_LEVEL`, `MAX_SUPPORTED_ROGUE_LEVEL`, and
//! `MAX_SUPPORTED_RANGER_LEVEL`, all from 11 to 12). §3.1 race rows and §3.3
//! interaction rows stay fully exhausted / structurally blocked (cited from
//! the progress doc, not re-derived). This cycle re-verified live which
//! §3.2 classes still sat at the level-11 ceiling: Sorcerer and Wizard both
//! did. Wizard carries a live claim-blocker naming "spellbook content" as
//! deliberately out of scope (`class_spell.wizard.prepared_spellbook.unsupported`),
//! so this cycle picks **Sorcerer** instead: it is a spontaneous caster, and
//! its own claim-blocker (`class_spell.sorcerer.spontaneous.unsupported`,
//! spontaneous casting execution and which-spells-known selection) is a
//! different shape from Wizard's prepared-spellbook blocker. This cycle's
//! widening stays strictly within the already-grounded arithmetic pillars
//! (base attack bonus, base saves, the spell-level access ladder, the base
//! spells-per-day table, base spell-save-DC arithmetic, the base
//! spells-known table, Charisma bonus spell slots, and the integrated
//! totals) — it does not touch actual spell selection, preparation, or
//! casting, which stays claim-blocked exactly as before.
//!
//! Both PF1 CRB primary sources (d20pfsrd and the Archives of Nethys
//! aonprd.com / legacy.aonprd.com mirror) were read directly before writing
//! any code or test, and all three fetches agree byte-for-byte:
//!
//! - level 12 base attack bonus genuinely rises to +6 (`12 / 2 = 6`, up from
//!   +5 at level 11) and all three base saves genuinely rise too: +4
//!   Fortitude (poor, `12 / 3 = 4`), +4 Reflex (poor, `12 / 3 = 4`), and +8
//!   Will (good, `12 / 2 + 2 = 8`) — all four values via the same formulas
//!   already grounded at levels 1-11, not re-derived.
//! - the PF1 Core Rulebook Sorcerer class table's level-12 "Special" column
//!   is genuinely BLANK (verified independently against both primary
//!   sources, checked rather than assumed away) — like levels 2, 4, 6, 8,
//!   and 10, and UNLIKE the level-3/5/7/9/11 "Bloodline power"/"Bloodline
//!   spell" rows — so no new class feature is gained at 12th level and no
//!   new pillar is grounded from the Special column.
//! - the Spells per Day table's level-12 row is `6/6/6/6/5/3` (1st/2nd/3rd/
//!   4th/5th/6th), genuinely risen from level 11's `6/6/6/6/4` at the 5th
//!   (4 -> 5) spell level, with a genuinely NEW 6th-level column appearing
//!   for the first time (3) — verified identically on all three fetches.
//! - the Spells Known table's level-12 row is `9/5/5/4/3/2/1` (0th/1st/2nd/
//!   3rd/4th/5th/6th), genuinely risen only by the appearance of a NEW
//!   6th-level column (1), with the 0th through 5th columns numerically
//!   unchanged from level 11's `9/5/5/4/3/2` — verified identically on all
//!   three fetches.
//! - the spell-save-DC and Charisma-bonus-spell formulas are unchanged
//!   (live arithmetic over the already-grounded access ladder and the
//!   fixture's Charisma modifier), widening automatically to the level-12
//!   access ladder (which itself genuinely rises to 6, since the 6th-level
//!   threshold is reached for the first time at level 12).
//! - the bloodline choice and bloodline class-skill choice recognitions are
//!   not level-gated, so both still fire at level 12 for the same fixture
//!   selections.
//!
//! It deliberately does not touch Arcane Bond, bloodline arcana, the
//! bloodline powers/spells/feats, or the spontaneous which-spells-known
//! selection / casting-execution burden (all stay named-but-unproven,
//! unchanged from levels 1-11), and it does not ground Sorcerer level 13+.
//! It also preserves the accepted Sorcerer level-1..level-11 truth
//! (unchanged), the Fighter negative control, and the multiclass negative
//! control. Per the brief's lesson about stale negative controls, this
//! cycle also moves the sibling "level 12 is not promoted" negative
//! controls in `tests/sd13_sorcerer_level10_progression.rs` and
//! `tests/sd18_sorcerer_level11_widening.rs` to a "level 13 is not
//! promoted" boundary in the same commit.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const SORCERER_LEVEL11_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_sorcerer_level11_sd18_widening_deterministic_input.txt"
);

const SORCERER_LEVEL12_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_sorcerer_level12_sd18_widening_deterministic_input.txt"
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

// ----- Base attack bonus at level 12 genuinely rises -----

#[test]
fn sorcerer_level12_base_attack_bonus_is_grounded_and_rises() {
    let input = load(SORCERER_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(
        base_attack.value, 6,
        "Sorcerer level 12 1/2-BAB progression (12 / 2) must equal 6 — genuinely risen from 5 \
         at level 11: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 12 genuinely rise (all three) -----

#[test]
fn sorcerer_level12_base_saves_are_grounded_and_rise() {
    let input = load(SORCERER_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.sorcerer.base_save.fortitude");
    assert_eq!(
        fortitude.value, 4,
        "Sorcerer level 12 poor Fortitude (12/3) must equal 4 — genuinely risen from 3 at \
         level 11"
    );

    let reflex = explanation(&computation, "class_chassis.sorcerer.base_save.reflex");
    assert_eq!(
        reflex.value, 4,
        "Sorcerer level 12 poor Reflex (12/3) must equal 4 — genuinely risen from 3 at level 11"
    );

    let will = explanation(&computation, "class_chassis.sorcerer.base_save.will");
    assert_eq!(
        will.value, 8,
        "Sorcerer level 12 good Will (12/2+2) must equal 8 — genuinely risen from 7 at level 11"
    );
}

// ----- Base spells per day widen at level 12, with a new 6th-level column -----

#[test]
fn sorcerer_level12_base_spells_per_day_match_the_raw_table_row() {
    let input = load(SORCERER_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

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
        "level 12 (`6/6/6/6/5/3`): the 5th-level column rises from 4 to 5 and a genuinely NEW \
         6th-level column appears (3) for the first time"
    );
}

// ----- Spells known widen at level 12, with a new 6th-level column -----

#[test]
fn sorcerer_level12_spells_known_match_the_raw_table_row() {
    let input = load(SORCERER_LEVEL12_FIXTURE);
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
            (format!("{KNOWN_PREFIX}spell_level_6"), 1),
        ],
        "level 12 (`9/5/5/4/3/2/1`): the 0th through 5th columns stay numerically unchanged \
         from level 11's `9/5/5/4/3/2`, and a genuinely NEW 6th-level column appears (1) for \
         the first time"
    );
}

// ----- Spell-level access ladder rises to 6 at level 12 -----

#[test]
fn sorcerer_level12_spell_level_access_rises_to_six() {
    let input = load(SORCERER_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(
        &computation,
        "class_chassis.sorcerer.spontaneous.spell_level_access",
    );
    assert_eq!(
        access.value, 6,
        "Sorcerer level 12 spell-level access must genuinely rise to 6th-level spells — the \
         sorcerer's two-level cadence (2nd at 4, 3rd at 6, 4th at 8, 5th at 10, 6th at 12): {}",
        access.detail
    );
}

// ----- Bloodline choice recognition still fires at level 12 -----

#[test]
fn sorcerer_level12_still_recognizes_the_bloodline_choice() {
    let input = load(SORCERER_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.sorcerer.bloodline_choice");
    assert_eq!(
        choice.value, 0,
        "bloodline choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Arcane"),
        "bloodline choice recognition must still name the Arcane bloodline at level 12: {}",
        choice.detail
    );
}

// ----- The spell-bearing baseline recognition and both burden diagnostics persist -----

#[test]
fn sorcerer_level12_still_recognizes_the_spell_bearing_baseline_and_claim_blocks_burdens() {
    let input = load(SORCERER_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.sorcerer"),
        "level-12 Sorcerer must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported"
            && d.claim_blocking),
        "level-12 Sorcerer must still claim-block on the Arcane Bond / bloodline progression \
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

// ----- No bloodline-power/spell record is fabricated at level 12 -----

#[test]
fn sorcerer_level12_does_not_fabricate_any_bloodline_entry() {
    let input = load(SORCERER_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("bloodline_power")
                || e.id.to_lowercase().contains("bloodline_spell")),
        "level-12 Sorcerer must not fabricate a bloodline-power/bloodline-spell record — the \
         Special column is genuinely blank at level 12: {:?}",
        computation.explanations
    );
}

// ----- Negative control: level 11 truth is unchanged by this widening -----

#[test]
fn sorcerer_level11_truth_is_unchanged_by_this_slice() {
    let input = load(SORCERER_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(base_attack.value, 5, "Sorcerer level 11 base attack bonus must stay 5");

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_5"), 4),
        ],
        "Sorcerer level 11 base spells per day must stay `6/6/6/6/4`, with no 6th-level column"
    );
}

// ----- Negative control: level 21 stays unrecognized by this cycle -----
//
// SD18 (tests/sd18_sorcerer_level20_widening.rs) widened the bounded tranche
// to level 20, PF1's level cap, so this negative control now sits just above
// the current bound (level 21, which does not exist in PF1) rather than at
// level 20.

#[test]
fn sorcerer_level_21_is_not_promoted_by_this_slice() {
    let level_21 = SORCERER_LEVEL12_FIXTURE.replace("class:sorcerer:12", "class:sorcerer:21");
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
fn fighter_does_not_gain_sorcerer_level12_recognition() {
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
fn multiclass_sorcerer_level12_is_not_promoted_by_this_slice() {
    let multiclass = SORCERER_LEVEL12_FIXTURE.replace(
        "class_level=class:sorcerer:12",
        "class_level=class:sorcerer:12\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-12 widening -----

#[test]
fn matrix_sorcerer_row_names_level_12_widening() {
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
            .contains("sd18_sorcerer_level12_widening"),
        "sorcerer row must cite the live SD18 level-12 proof surface: {}",
        sorcerer.grounding_ref
    );
    let note = sorcerer.blocker_or_lossiness_note;
    assert!(
        note.contains("level 12") || note.contains("level-12"),
        "sorcerer partial note must name the level-12 widening: {note}"
    );
}
