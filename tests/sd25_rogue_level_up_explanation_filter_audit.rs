//! SD-25 Epic 7 (per-class residue audit, criterion 7.3): Rogue
//! `class_spell.*`-vs-`class_feature.*` explanation-id-filter audit.
//!
//! SD-24 criterion 4.1 found and fixed a bug in `level_up/wizard.rs`: its
//! `is_wizard_pillar` filter admitted only `class_chassis.wizard.` (plus
//! one named recognition id), so once SD-21 E6b.2 landed a real
//! `class_spell.wizard.*` explanation family in `pilot_compute.rs`
//! (grounded AFTER `wizard.rs` was authored), every one of those real
//! facts was silently dropped from `LevelUpPlan.automatic_features` —
//! the filter was never widened to admit the new prefix. SD-24 never
//! audited the other 9 CRB classes for the same bug shape (see SD-25
//! `progress.md` §`## DISCOVERED`, register A6 carry-forward, criterion
//! 7.1's residue intake).
//!
//! This audit covers Rogue (`src/rules_core/level_up/rogue.rs`,
//! suggested criterion 7.3). Two things are checked, exhaustively across
//! every supported Rogue level (1-20), not sampled:
//!
//! 1. **No unknown explanation-id family exists for Rogue in
//!    `pilot_compute.rs` today.** Every explanation id
//!    `compute_pilot_base_chassis` grounds for a synthetic single-class
//!    Human Rogue input, at every level 1-20, is prefixed either
//!    `class_chassis.rogue.` or `class_feature.rogue.` — the exact two
//!    prefixes `rogue.rs`'s own `append_class_feature_grants` filter
//!    (`is_rogue_class_feature_id`) admits. Unlike Wizard, Rogue has no
//!    `class_spell.rogue.*` family in `pilot_compute.rs` — Rogue is a
//!    non-caster in PF1 Core Rulebook — so this is a genuine negative
//!    finding, not a skipped check: if a future `pilot_compute.rs` change
//!    ever grounds a new Rogue explanation family under a THIRD prefix
//!    (e.g. `class_talent.rogue.*`) without widening `rogue.rs`'s filter
//!    to match, this assertion catches it immediately (this is the exact
//!    shape the Wizard bug took: a new prefix landing after the module
//!    was authored, never added to its filter).
//! 2. **Every real (non-"correctly absent", non-class-table-covered)
//!    Rogue explanation that newly becomes granted between two levels
//!    surfaces as a real grant in `compute_rogue_level_up_grants`'s
//!    output**, swept across all 19 level-up transitions (1->2 through
//!    19->20), cross-checking the raw `pilot_compute` explanation set
//!    against the composed `LevelUpPlan` one level pair at a time. This
//!    is the literal "would this catch the Wizard-shaped bug" test:
//!    temporarily narrowing `rogue.rs`'s filter to admit only
//!    `class_chassis.rogue.` (dropping `class_feature.rogue.`, exactly
//!    the Wizard bug's shape applied to Rogue's second prefix) was
//!    confirmed live to make this test fail — Evasion, Trap Sense,
//!    Uncanny Dodge, Improved Uncanny Dodge, and Master Strike all
//!    vanish from the plan — before being reverted. See this cycle's
//!    receipt (`artifacts/epic_7/rogue-residue-audit-cycle_receipt.md`)
//!    for the full RED transcript.

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState, SelectedChoice,
};
use codex::rules_core::level_up::rogue::compute_rogue_level_up_grants;
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;

const ROGUE_TALENT_CHOICES: &[(u8, &str, &str)] = &[
    (2, "choice:rogue_talent", "talent:finesse_rogue"),
    (4, "choice:rogue_talent_2", "talent:combat_trick"),
    (6, "choice:rogue_talent_3", "talent:fast_stealth"),
    (8, "choice:rogue_talent_4", "talent:trap_spotter"),
    (10, "choice:rogue_talent_5", "talent:resiliency"),
    (12, "choice:rogue_talent_6", "talent:crippling_strike"),
    (14, "choice:rogue_talent_7", "talent:dispelling_attack"),
    (16, "choice:rogue_talent_8", "talent:slow_reactions"),
    (18, "choice:rogue_talent_9", "talent:opportunist"),
    (20, "choice:rogue_talent_10", "talent:another_day"),
];

/// A deterministic Human Rogue with every numbered talent choice-slot
/// pre-populated (so every talent-choice explanation is grounded at every
/// level, not left named-but-unselected), at the given class level.
fn human_rogue_input(level: u8) -> CharacterInput {
    let selected_choices = ROGUE_TALENT_CHOICES
        .iter()
        .map(|(_, choice_set_id, selection_id)| SelectedChoice {
            choice_set_id: choice_set_id.to_string(),
            selection_id: selection_id.to_string(),
        })
        .collect();

    CharacterInput {
        case_id: Some("sd25_rogue_residue_audit".to_string()),
        source_package_id: "sd25_rogue_residue_audit".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "race:human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:rogue".to_string(),
                level,
            }],
            ability_scores: AbilityScores {
                strength: 12,
                dexterity: 16,
                constitution: 12,
                intelligence: 13,
                wisdom: 10,
                charisma: 8,
            },
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices,
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

fn is_absent_marker(detail: &str) -> bool {
    detail.contains("correctly absent")
}

const CLASS_TABLE_COVERED_EXPLANATION_IDS: [&str; 4] = [
    "class_chassis.rogue.base_attack_bonus",
    "class_chassis.rogue.base_save.fortitude",
    "class_chassis.rogue.base_save.reflex",
    "class_chassis.rogue.base_save.will",
];

/// Check 1: no explanation family for Rogue exists in `pilot_compute.rs`
/// today outside the two prefixes `rogue.rs`'s filter admits. Swept
/// across all 20 supported levels, not sampled.
#[test]
fn every_rogue_explanation_id_from_pilot_compute_is_covered_by_rogues_own_two_admitted_prefixes() {
    for level in 1..=20u8 {
        let character = human_rogue_input(level);
        let explanations = compute_pilot_base_chassis(&character).explanations;

        let rogue_explanations: Vec<_> = explanations
            .iter()
            .filter(|explanation| explanation.id.contains("rogue"))
            .collect();
        assert!(
            !rogue_explanations.is_empty(),
            "expected at least one rogue explanation at level {level}"
        );

        for explanation in rogue_explanations {
            let admitted = explanation.id.starts_with("class_chassis.rogue.")
                || explanation.id.starts_with("class_feature.rogue.");
            assert!(
                admitted,
                "level {level}: found a rogue explanation id ({:?}) outside the two prefixes \
                 `rogue.rs`'s filter admits (`class_chassis.rogue.` / `class_feature.rogue.`) — \
                 this is exactly the Wizard bug's shape (a new explanation family landing \
                 after the module's filter was authored, never added to it); `rogue.rs` needs \
                 a corresponding filter widening",
                explanation.id
            );
        }
    }
}

/// Check 2: every real (non-absent, non-class-table-covered) Rogue
/// explanation that newly becomes granted between `from_level` and
/// `to_level` shows up as a real grant in `compute_rogue_level_up_grants`.
/// Swept across every one of the 19 level-up transitions.
#[test]
fn every_newly_granted_rogue_explanation_surfaces_as_a_real_levelup_grant_at_every_transition() {
    for to_level in 2..=20u8 {
        let from_level = to_level - 1;
        let from_character = human_rogue_input(from_level);
        let to_character = human_rogue_input(to_level);

        let from_explanations = compute_pilot_base_chassis(&from_character).explanations;
        let to_explanations = compute_pilot_base_chassis(&to_character).explanations;
        let plan = compute_rogue_level_up_grants(&to_character, from_level, to_level);

        for to_explanation in &to_explanations {
            let is_rogue_class_feature_id = to_explanation.id.starts_with("class_chassis.rogue.")
                || to_explanation.id.starts_with("class_feature.rogue.");
            if !is_rogue_class_feature_id {
                continue;
            }
            if CLASS_TABLE_COVERED_EXPLANATION_IDS.contains(&to_explanation.id.as_str()) {
                // Already asserted for real by `sd20_levelup_rogue.rs` via
                // `append_class_table_grants` — out of this audit's scope.
                continue;
            }

            let from_match = from_explanations
                .iter()
                .find(|explanation| explanation.id == to_explanation.id);
            let from_granted = from_match.map(|explanation| !is_absent_marker(&explanation.detail));
            let to_granted = !is_absent_marker(&to_explanation.detail);
            let value_changed =
                from_match.map(|explanation| explanation.value) != Some(to_explanation.value);
            let newly_granted = from_granted != Some(true) && to_granted;

            if !value_changed && !newly_granted {
                continue;
            }

            let surfaced = plan
                .automatic_features
                .iter()
                .any(|grant| grant.source_table.column_key == to_explanation.id);
            assert!(
                surfaced,
                "level {from_level} -> {to_level}: expected explanation {:?} (a real, newly-\
                 changed/newly-granted Rogue fact per pilot_compute.rs) to surface as a grant \
                 in compute_rogue_level_up_grants's plan, but it did not — got automatic \
                 features: {:#?}",
                to_explanation.id, plan.automatic_features
            );
        }
    }
}
