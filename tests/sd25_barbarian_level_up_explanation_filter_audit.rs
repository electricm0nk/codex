//! SD-25 Epic 7 (per-class residue audit, criterion 7.5): Barbarian
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
//! This audit covers Barbarian (`src/rules_core/level_up/barbarian.rs`,
//! suggested criterion 7.5). Two things are checked, exhaustively across
//! every supported Barbarian level (1-20), not sampled:
//!
//! 1. **No unknown explanation-id family exists for Barbarian in
//!    `pilot_compute.rs` today.** Every explanation id
//!    `compute_pilot_base_chassis` grounds for a synthetic single-class
//!    Human Barbarian input, at every level 1-20, is prefixed either
//!    `class_chassis.barbarian.` or `class_feature.barbarian.` — the
//!    exact two prefixes `barbarian.rs`'s own
//!    `append_class_feature_grants` filter
//!    (`is_barbarian_class_feature_id`) admits. Unlike Wizard, Barbarian
//!    has no `class_spell.barbarian.*` family in `pilot_compute.rs` —
//!    Barbarian is a non-caster in PF1 Core Rulebook — so this is a
//!    genuine negative finding, not a skipped check: if a future
//!    `pilot_compute.rs` change ever grounds a new Barbarian explanation
//!    family under a THIRD prefix (e.g. `class_talent.barbarian.*`)
//!    without widening `barbarian.rs`'s filter to match, this assertion
//!    catches it immediately (this is the exact shape the Wizard bug
//!    took: a new prefix landing after the module was authored, never
//!    added to its filter).
//! 2. **Every real (non-"correctly absent", non-class-table-covered,
//!    non-resource-pool) Barbarian explanation that newly becomes
//!    granted between two levels surfaces as a real grant in
//!    `compute_barbarian_level_up_grants`'s output**, swept across all
//!    19 level-up transitions (1->2 through 19->20), cross-checking the
//!    raw `pilot_compute` explanation set against the composed
//!    `LevelUpPlan` one level pair at a time. This is the literal "would
//!    this catch the Wizard-shaped bug" test: temporarily narrowing
//!    `barbarian.rs`'s filter to admit only `class_chassis.barbarian.`
//!    (dropping `class_feature.barbarian.`, exactly the Wizard bug's
//!    shape applied to Barbarian's second prefix) was confirmed live to
//!    make this test fail — Uncanny Dodge, Trap Sense, Improved Uncanny
//!    Dodge, Damage Reduction, and Indomitable Will all vanish from the
//!    plan — before being reverted. See this cycle's receipt
//!    (`artifacts/epic_7/barbarian-residue-audit-cycle_receipt.md`) for
//!    the full RED transcript.

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState, SelectedChoice,
};
use codex::rules_core::level_up::barbarian::compute_barbarian_level_up_grants;
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;

const BARBARIAN_RAGE_POWER_CHOICES: &[(u8, &str, &str)] = &[
    (2, "choice:barbarian_rage_power", "rage_power:beast_totem"),
    (4, "choice:barbarian_rage_power_2", "rage_power:knockback"),
    (6, "choice:barbarian_rage_power_3", "rage_power:no_escape"),
    (8, "choice:barbarian_rage_power_4", "rage_power:guarded_stance"),
    (10, "choice:barbarian_rage_power_5", "rage_power:terrifying_howl"),
    (12, "choice:barbarian_rage_power_6", "rage_power:superstition"),
    (14, "choice:barbarian_rage_power_7", "rage_power:witch_hunter"),
    (16, "choice:barbarian_rage_power_8", "rage_power:eater_of_magic"),
    (18, "choice:barbarian_rage_power_9", "rage_power:mighty_swing"),
    (20, "choice:barbarian_rage_power_10", "rage_power:invulnerability"),
];

/// A deterministic Human Barbarian (Constitution 16, matching
/// `sd20_levelup_barbarian.rs`'s already-established posture so rage
/// rounds per day stays positive at every level 1-20: 4 + 3 + 2 *
/// (level - 1) is never non-positive) with every numbered rage-power
/// choice-slot pre-populated, at the given class level.
fn human_barbarian_input(level: u8) -> CharacterInput {
    let selected_choices = BARBARIAN_RAGE_POWER_CHOICES
        .iter()
        .map(|(_, choice_set_id, selection_id)| SelectedChoice {
            choice_set_id: choice_set_id.to_string(),
            selection_id: selection_id.to_string(),
        })
        .collect();

    CharacterInput {
        case_id: Some("sd25_barbarian_residue_audit".to_string()),
        source_package_id: "sd25_barbarian_residue_audit".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "race:human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:barbarian".to_string(),
                level,
            }],
            ability_scores: AbilityScores {
                strength: 16,
                dexterity: 14,
                constitution: 16,
                intelligence: 8,
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
    "class_chassis.barbarian.base_attack_bonus",
    "class_chassis.barbarian.base_save.fortitude",
    "class_chassis.barbarian.base_save.reflex",
    "class_chassis.barbarian.base_save.will",
];

/// Rage rounds per day is a resource-pool delta (`ResourcePoolDelta`),
/// not a discrete `Grant` — `barbarian.rs`'s own `append_class_feature_grants`
/// deliberately routes it to `plan.resource_pool_change` instead of
/// `plan.automatic_features`, so this audit's check 2 must exclude it
/// from the "must surface as a grant" sweep the same way `barbarian.rs`
/// itself does.
const RAGE_ROUNDS_PER_DAY_EXPLANATION_ID: &str = "class_chassis.barbarian.rage_rounds_per_day";

/// Check 1: no explanation family for Barbarian exists in
/// `pilot_compute.rs` today outside the two prefixes `barbarian.rs`'s
/// filter admits. Swept across all 20 supported levels, not sampled.
#[test]
fn every_barbarian_explanation_id_from_pilot_compute_is_covered_by_barbarians_own_two_admitted_prefixes()
{
    for level in 1..=20u8 {
        let character = human_barbarian_input(level);
        let explanations = compute_pilot_base_chassis(&character).explanations;

        let barbarian_explanations: Vec<_> = explanations
            .iter()
            .filter(|explanation| explanation.id.contains("barbarian"))
            .collect();
        assert!(
            !barbarian_explanations.is_empty(),
            "expected at least one barbarian explanation at level {level}"
        );

        for explanation in barbarian_explanations {
            let admitted = explanation.id.starts_with("class_chassis.barbarian.")
                || explanation.id.starts_with("class_feature.barbarian.");
            assert!(
                admitted,
                "level {level}: found a barbarian explanation id ({:?}) outside the two \
                 prefixes `barbarian.rs`'s filter admits (`class_chassis.barbarian.` / \
                 `class_feature.barbarian.`) — this is exactly the Wizard bug's shape (a new \
                 explanation family landing after the module's filter was authored, never \
                 added to it); `barbarian.rs` needs a corresponding filter widening",
                explanation.id
            );
        }
    }
}

/// Check 2: every real (non-absent, non-class-table-covered,
/// non-resource-pool) Barbarian explanation that newly becomes granted
/// between `from_level` and `to_level` shows up as a real grant in
/// `compute_barbarian_level_up_grants`. Swept across every one of the 19
/// level-up transitions.
#[test]
fn every_newly_granted_barbarian_explanation_surfaces_as_a_real_levelup_grant_at_every_transition()
{
    for to_level in 2..=20u8 {
        let from_level = to_level - 1;
        let from_character = human_barbarian_input(from_level);
        let to_character = human_barbarian_input(to_level);

        let from_explanations = compute_pilot_base_chassis(&from_character).explanations;
        let to_explanations = compute_pilot_base_chassis(&to_character).explanations;
        let plan = compute_barbarian_level_up_grants(&to_character, from_level, to_level);

        for to_explanation in &to_explanations {
            let is_barbarian_class_feature_id = to_explanation
                .id
                .starts_with("class_chassis.barbarian.")
                || to_explanation.id.starts_with("class_feature.barbarian.");
            if !is_barbarian_class_feature_id {
                continue;
            }
            if CLASS_TABLE_COVERED_EXPLANATION_IDS.contains(&to_explanation.id.as_str()) {
                // Already asserted for real by `sd20_levelup_barbarian.rs`
                // via `append_class_table_grants` — out of this audit's
                // scope.
                continue;
            }
            if to_explanation.id == RAGE_ROUNDS_PER_DAY_EXPLANATION_ID {
                // Resource pool delta, not a Grant — asserted separately
                // below.
                continue;
            }

            let from_match = from_explanations
                .iter()
                .find(|explanation| explanation.id == to_explanation.id);
            let from_granted =
                from_match.map(|explanation| !is_absent_marker(&explanation.detail));
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
                 changed/newly-granted Barbarian fact per pilot_compute.rs) to surface as a \
                 grant in compute_barbarian_level_up_grants's plan, but it did not — got \
                 automatic features: {:#?}",
                to_explanation.id, plan.automatic_features
            );
        }

        // Rage rounds per day must surface as a resource pool change
        // whenever it actually changes value between the two levels —
        // the same fact, checked via the correct field rather than
        // skipped outright.
        let from_rage_rounds = from_explanations
            .iter()
            .find(|explanation| explanation.id == RAGE_ROUNDS_PER_DAY_EXPLANATION_ID)
            .map(|explanation| explanation.value);
        let to_rage_rounds = to_explanations
            .iter()
            .find(|explanation| explanation.id == RAGE_ROUNDS_PER_DAY_EXPLANATION_ID)
            .map(|explanation| explanation.value);
        if let (Some(from_value), Some(to_value)) = (from_rage_rounds, to_rage_rounds) {
            if from_value != to_value {
                let surfaced_pool = plan
                    .resource_pool_change
                    .pools
                    .iter()
                    .any(|pool| pool.pool_id == "rage_rounds_per_day" && pool.to_value == to_value);
                assert!(
                    surfaced_pool,
                    "level {from_level} -> {to_level}: expected rage_rounds_per_day \
                     ({from_value} -> {to_value}) to surface as a resource pool change, but it \
                     did not — got pools: {:#?}",
                    plan.resource_pool_change.pools
                );
            }
        }
    }
}
