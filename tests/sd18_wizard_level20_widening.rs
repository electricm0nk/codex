//! SD18 Wizard level-20 widening grounding proof.
//!
//! Widens the accepted deterministic Human Wizard level-1..level-19 prepared
//! arcane spell-bearing chassis (`tests/sd18_wizard_level19_widening.rs`) to
//! Wizard level 20, mirroring the sibling-class level-range-gate idiom
//! (`supported_wizard_level` is generalized from `1..=19` to `1..=20` via
//! `MAX_SUPPORTED_WIZARD_LEVEL = 20`). Level 20 is the final level within
//! PF1's 1-20 character-level cap for this class row, and the loop's SECOND
//! §3.2 level-20 landing, after Cleric. §3.1 race rows and §3.3 interaction
//! rows stay fully exhausted / structurally blocked (cited from the
//! progress doc, not re-derived); §3.4/§3.5 stay structurally blocked for
//! the same documented reason.
//!
//! Two independent primary sources (d20pfsrd.com and the Archives of Nethys
//! aonprd.com mirror) were read directly (raw `curl` fetch + a small Python
//! tag-stripper, not AI-summarized, not assumed from training-data memory)
//! before writing any code or test, fetching the full levels-15-through-20
//! block in one pass so the level-20 row's neighbors were visible in
//! context (guards against level-misattribution), and both agree
//! byte-for-byte on every row (no self-contradiction, so a third source was
//! not required):
//!
//! - level 15: "+7/+2 | +5 | +5 | +9 | Bonus feat | 4/4/4/4/4/4/3/2/1"
//! - level 16: "+8/+3 | +5 | +5 | +10 | — | 4/4/4/4/4/4/3/3/2"
//! - level 17: "+8/+3 | +5 | +5 | +10 | — | 4/4/4/4/4/4/4/3/2/1"
//! - level 18: "+9/+4 | +6 | +6 | +11 | — | 4/4/4/4/4/4/4/3/3/2"
//! - level 19: "+9/+4 | +6 | +6 | +11 | — | 4/4/4/4/4/4/4/4/3/3"
//! - level 20: "+10/+5 | +6 | +6 | +12 | Bonus feat | 4/4/4/4/4/4/4/4/4/4"
//!
//! So at level 20: base attack bonus GENUINELY RISES to +10 (`20/2 = 10`, up
//! from +9 at level 19) and good Will GENUINELY RISES to +12 (`20/2+2 = 12`,
//! up from +11 at level 19), while poor Fortitude/Reflex both STAY at +6
//! (`20/3 = 6`, an integer-division coincidence with level 19).
//! - the raw Wizard spells-per-day table's level-20 row is
//!   "4/4/4/4/4/4/4/4/4/4", up from the level-19 row
//!   "4/4/4/4/4/4/4/4/3/3" (the 8th- and 9th-level columns both rise to 4)
//!   — but NO genuinely new spell-level column opens (9th is already the
//!   highest wizard spell level in PF1, first opened at level 17), so the
//!   specialist bonus-slot flat count (one bonus slot of each spell level
//!   she can cast, 1st through 9th) STAYS at 9, unchanged from levels
//!   17-19.
//! - Intense Spells' bonus-damage magnitude GENUINELY RISES to 10
//!   (`max(20/2, 1) = 10`, up from 9 at level 19) via the pre-existing
//!   formula, not re-derived; Force Missile's uses-per-day pool stays the
//!   level-independent 3 + Intelligence modifier (6); Scribe Scroll and the
//!   school specialization choice recognitions are not level-gated and
//!   still fire.
//! - the level-20 "Special" column reads "Bonus feat" on both primary
//!   sources — the SAME genuinely open-ended metamagic/item-creation/
//!   Spell-Mastery choice already left named-but-unproven at levels 5, 10,
//!   and 15 (the class table's own "Bonus Feats" ability text, "At 5th,
//!   10th, 15th, and 20th level, a wizard gains a bonus feat. At each such
//!   opportunity, he can choose a metamagic feat, an item creation feat, or
//!   Spell Mastery," is identical wording on both sources and names no new
//!   mechanic at 20th level), so it stays deliberately named-but-unproven
//!   and no new pillar record is grounded at level 20. A separate,
//!   non-Core-Rulebook "Well-Prepared" alternate capstone appears on both
//!   sources but is explicitly sourced to Pathfinder Player Companion:
//!   Chronicle of Legends (an optional splatbook replacement ability a
//!   player may pick instead of the standard 20th-level ability) — out of
//!   SD18's Core Rulebook scope, not the default row, and not modeled here.
//!
//! It deliberately does not touch the school-power execution burden
//! (Intense Spells' damage application, Force Missile's casting execution),
//! the opposed-school preparation-cost burden, the still-unproven
//! 5th/10th/15th/20th-level "Bonus feat" selection/execution, or the
//! prepared spellbook / spells-prepared / spell-slot posture burden (all
//! stay named-but-unproven, unchanged from levels 1-19). PF1 character
//! levels cap at 20, so this closes the per-level arithmetic-widening
//! frontier for the Wizard row entirely — the row stays Partial, not
//! Supported, because the school-power-execution, opposed-school-cost,
//! bonus-feat-selection, and prepared-spell-posture burdens remain
//! unproven, not because any further level exists to widen into. It also
//! preserves the accepted Wizard level-1..level-19 truth (unchanged), the
//! Fighter negative control, and the multiclass negative control. Per the
//! brief's lesson about stale negative controls, a targeted grep for
//! `class:wizard:20` found FIVE stale sibling files carrying a "level 20
//! stays claim-blocked" negative control one level too shallow:
//! `tests/sd13_wizard_level10_progression.rs`,
//! `tests/sd18_wizard_level11_widening.rs`,
//! `tests/sd18_wizard_level12_widening.rs`,
//! `tests/sd18_wizard_level13_widening.rs`, and
//! `tests/sd18_wizard_level14_widening.rs` — this cycle moves all five
//! sibling "level 20 is not promoted" negative controls to a "level 21 is
//! not promoted" boundary in the same commit, purely as an
//! implementation-gate check (PF1 does not have a 21st character level;
//! this only verifies the code's own range gate does not overshoot the
//! newly raised ceiling), mirroring the Cleric level-20 cycle's own fix
//! exactly. `tests/sd18_wizard_level19_widening.rs`'s own level-20
//! negative-control test is removed rather than moved, since level 20 is
//! now itself the supported row.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const WIZARD_LEVEL19_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_wizard_level19_sd18_widening_deterministic_input.txt"
);

const WIZARD_LEVEL20_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_wizard_level20_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus and good Will genuinely rise at level 20; poor Fort/Reflex stay put -----

#[test]
fn wizard_level20_base_attack_and_good_will_rise() {
    let input = load(WIZARD_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 10,
        "Wizard level 20 1/2-BAB progression (20 / 2) must GENUINELY RISE to 10, up from 9 at \
         level 19: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.wizard.base_save.fortitude");
    assert_eq!(
        fortitude.value, 6,
        "Wizard level 20 poor Fortitude (20/3) must STAY at 6, an integer-division coincidence \
         with level 19"
    );

    let reflex = explanation(&computation, "class_chassis.wizard.base_save.reflex");
    assert_eq!(
        reflex.value, 6,
        "Wizard level 20 poor Reflex (20/3) must STAY at 6, an integer-division coincidence \
         with level 19"
    );

    let will = explanation(&computation, "class_chassis.wizard.base_save.will");
    assert_eq!(
        will.value, 12,
        "Wizard level 20 good Will (20/2+2) must GENUINELY RISE to 12, up from 11 at level 19"
    );
}

// ----- The specialist bonus slot count stays flat at level 20 (no new column opens) -----

#[test]
fn wizard_level20_specialist_bonus_slot_stays_flat_at_nine() {
    let input = load(WIZARD_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(
        slot.value, 9,
        "Wizard level 20 specialist bonus slot count must STAY at 9 — the raw spells-per-day \
         table's level-20 row \"4/4/4/4/4/4/4/4/4/4\" does not open any spell-level column \
         beyond the 9th (already the highest wizard spell level in PF1, first opened at level \
         17): {}",
        slot.detail
    );
}

// ----- Intense Spells' bonus damage genuinely rises at level 20 -----

#[test]
fn wizard_level20_intense_spells_bonus_damage_rises() {
    let input = load(WIZARD_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let intense = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(
        intense.value, 10,
        "Intense Spells' bonus-damage magnitude (max(20/2, 1)) must GENUINELY RISE to 10 at \
         level 20, up from 9 at level 19: {}",
        intense.detail
    );
}

// ----- Force Missile, Scribe Scroll, and the specialization choice carry over unchanged -----

#[test]
fn wizard_level20_grants_carry_over_unchanged() {
    let input = load(WIZARD_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let force_missile = explanation(
        &computation,
        "class_chassis.wizard.force_missile_uses_per_day",
    );
    assert_eq!(
        force_missile.value, 7,
        "Force Missile's uses per day (3 + Intelligence modifier 4) must stay 7 at level 20"
    );

    let scribe_scroll = explanation(&computation, "class_chassis.wizard.scribe_scroll");
    assert_eq!(scribe_scroll.value, 0, "Scribe Scroll must still carry no mechanical value");

    let specialization = explanation(&computation, "class_chassis.wizard.specialization_choice");
    assert_eq!(
        specialization.value, 0,
        "the school specialization choice seam must still carry no mechanical value"
    );
}

// ----- The spell-bearing baseline recognition and both burden diagnostics persist -----

#[test]
fn wizard_level20_still_recognizes_the_spell_bearing_baseline_and_claim_blocks_burdens() {
    let input = load(WIZARD_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.wizard"),
        "level-20 Wizard must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported"
            && d.claim_blocking),
        "level-20 Wizard must still claim-block on the school-power execution and \
         opposed-school preparation-cost burden: {:?}",
        computation.diagnostics
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_spell.wizard.prepared_spellbook.unsupported"
                && d.claim_blocking),
        "level-20 Wizard must still claim-block on the prepared spellbook posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 19 truth is unchanged by this slice -----

#[test]
fn wizard_level19_truth_is_unchanged_by_this_slice() {
    let input = load(WIZARD_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(base_attack.value, 9, "Wizard level 19 base attack bonus must stay 9");

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(slot.value, 9, "Wizard level 19 specialist bonus slot count must stay 9");

    let intense = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(intense.value, 9, "Wizard level 19 Intense Spells bonus damage must stay 9");
}

// ----- Negative control: level 21 stays unrecognized by this slice (implementation-gate -----
// ----- check only; PF1 has no 21st character level) -----

#[test]
fn wizard_level_21_is_not_promoted_by_this_slice() {
    let level_21 = WIZARD_LEVEL20_FIXTURE.replace("class:wizard:20", "class:wizard:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| (e.id.starts_with("class_chassis.wizard.")
                || e.id.starts_with("class_feature.wizard.")
                || e.id == "class_chassis.spell_baseline.wizard")
                // SD-34 decisions.md section 18 (`bfe90f020a`, 2026-08-29) widened the
                // anti-fabrication gate BY CONSTRUCTION for Wizard: class_feature_grant_
                // consumer now emits a real, citation-backed class_feature.wizard.
                // corpus_record.* id for any grant fact with a renderable corpus record,
                // at any Wizard level -- that commit widened the sd13_* acceptance tests
                // it named but never reached these later sd18_* widening siblings. Same
                // carve-out, same reasoning, applied here.
                && !e.id.starts_with("class_feature.wizard.corpus_record.")
                // AT-34-E3-001 cycle 6 (`49d72f5e03`, 2026-08-28) grounded Wizard Weapon
                // and Armor Proficiency unconditionally (real PF1 content, any level) --
                // pre-existing, already-tested, not promotion by this slice.
                && e.id != "class_feature.wizard.weapon_and_armor_proficiency"),
        "level-21 Wizard must not gain any bounded wizard explanation: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the wizard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_wizard_level20_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.wizard.")
                || e.id.starts_with("class_feature.wizard.")),
        "the Fighter chassis must not surface any wizard-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Wizard is not promoted -----

// SD-24 Epic 5 (criterion 5.1) correction: this control used to pair Wizard
// with Fighter as its "definitely still unsupported" second class. Fighter+
// Wizard is now a genuinely supported multiclass mix (SD-24 widened both
// pilot_compute.rs's explain_wizard_level1_prepared_spell_baseline and
// level_up::wizard's own entry gate), so this control now pairs Wizard
// with Rogue instead -- mirroring the Fighter-side negative controls
// (e.g. sd18_fighter_level20_widening.rs), which already used Rogue for
// the identical reason.

#[test]
fn multiclass_wizard_level20_is_not_promoted_by_this_slice() {
    let multiclass = WIZARD_LEVEL20_FIXTURE.replace(
        "class_level=class:wizard:20",
        "class_level=class:wizard:20\nclass_level=class:rogue:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
// (v0.6 swarm update) The v0.6 alpha swarm's multiclass BAB/save-stacking
    // generalization (task 4) widened the Wizard+Rogue multiclass mix into a
    // genuinely supported combination (Rogue now joins Fighter as a class
    // `is_supported_multiclass_mix` recognizes), so `wizard_level_in_mix`
    // (which already fires Wizard's own standalone `class_chassis.wizard.*`
    // explanations once ANY supported second class joins the mix, per the
    // pre-existing SD-24 Epic 5 Fighter+Wizard precedent) now also fires them
    // for a Wizard+Rogue mix. This negative control is superseded, not
    // violated: it now asserts the new, correct truth.
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.wizard.")
                || e.id.starts_with("class_feature.wizard.")),
        "multiclass Wizard now genuinely gains its bounded wizard explanations, mirroring the \
         pre-existing Fighter+Wizard precedent: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Wizard/Rogue still stays claim-blocked in this slice (by the deterministic \
         combat-baseline/skill-posture/spellbook-posture gates, not class-chassis recognition)"
    );
}

// ----- Control plane: the matrix note names the level-20 widening -----

#[test]
fn matrix_wizard_row_names_level_20_widening() {
    let matrix = seeded_current_truth();
    let wizard = matrix
        .row("class.wizard.progression_and_spell_burden")
        .expect("wizard progression_and_spell_burden row must exist");

    assert_eq!(wizard.support_state, SupportState::Supported); // Later promoted to Supported/ProductVisible by SD-19's Class Progression Catalog browser UI-surfacing work (2026-07-17).
    assert_eq!(wizard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(wizard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        wizard.grounding_ref.contains("sd18_wizard_level20_widening"),
        "wizard row must cite the live SD18 level-20 widening proof surface: {}",
        wizard.grounding_ref
    );
    let note = wizard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 20") || note.contains("level-20"),
        "wizard partial note must name the level-20 widening: {note}"
    );
}
