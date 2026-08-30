//! SD-20 Epic 4 (skill-rank allocation engine): max-rank-cap handling —
//! fourth and final work-unit. Closes Epic 4.
//!
//! Per `SD-20-rules-engine-completeness-scope-draft.md` §1.4 and
//! `epic-breakdown.md` criterion 9: "Per-level skill rank totals respect
//! PF1's max-rank cap: class skills max at character level + 3,
//! cross-class skills max at (character level + 1) / 2 rounded up. Cap
//! violations produce diagnostics, not fabricated totals."
//!
//! **What is genuinely new here (checked against the two already-landed
//! work-units before writing this test):**
//!
//! - The class-skill cap (`character level + 3`) was never enforced by
//!   any prior cycle — the class-skill-handling cycle and the
//!   cross-class-penalty cycle both pass a class skill's raw allocated
//!   `ranks` straight through with no ceiling at all.
//! - The cross-class-penalty cycle *did* already silently cap a
//!   cross-class skill's effective ranks at `ceil((character level + 1)
//!   / 2)` (`ranks.min(cross_class_cap)`), but it produced no diagnostic
//!   when that cap actually clipped an over-allocation — it just quietly
//!   returned the legal number. Criterion 9 requires a diagnostic on cap
//!   violation, for *either* category, not a silent clip. This work-unit
//!   adds that diagnostic (for both the newly-enforced class-skill cap
//!   and the already-enforced cross-class cap) without changing the
//!   legal, non-fabricated totals either cap already produces.

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState, SkillAllocation,
};
use codex::rules_core::skill_allocation::{allocate_skill_ranks, SkillTotal};

fn base_ability_scores() -> AbilityScores {
    AbilityScores {
        strength: 14,     // modifier +2
        dexterity: 15,    // modifier +2
        constitution: 13, // modifier +1
        intelligence: 10, // modifier +0
        wisdom: 10,       // modifier +0
        charisma: 8,      // modifier -1
    }
}

fn input_for(
    class_levels: Vec<CharacterClassLevel>,
    skill_allocations: Vec<SkillAllocation>,
) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_skill_allocation_max_rank_cap".to_string()),
        source_package_id: "sd20_skill_allocation_max_rank_cap".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "human".to_string(),
            class_levels,
            ability_scores: base_ability_scores(),
            selected_feats: Vec::new(),
            skill_allocations,
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

fn fighter_class_levels(level: u8) -> Vec<CharacterClassLevel> {
    vec![CharacterClassLevel {
        class_id: "class:fighter".to_string(),
        level,
    }]
}

#[test]
fn a_class_skill_over_allocated_past_level_plus_3_is_capped_with_a_diagnostic() {
    // Level 1 fighter: class-skill cap = character level + 3 = 4. Climb is
    // a grounded Fighter class skill. Allocating 6 ranks exceeds the cap.
    let input = input_for(
        fighter_class_levels(1),
        vec![SkillAllocation {
            skill_id: "skill:climb".to_string(),
            ranks: 6,
        }],
    );

    let totals = allocate_skill_ranks(&input);

    assert_eq!(
        totals.totals.get("skill:climb"),
        Some(&SkillTotal {
            ranks: 4, // capped at character level (1) + 3, not the raw 6
            ability_modifier: 2,
            class_skill_bonus: 3,
            misc_modifier: 0,
            total_modifier: 9,
        }),
        "a class skill's effective ranks must be capped at character level \
         + 3 -- the real, legal total, never the raw over-allocated number"
    );
    assert_eq!(
        totals.diagnostics.len(),
        1,
        "an over-allocated class skill must produce exactly one cap \
         violation diagnostic, not a silently fabricated total"
    );
    let diagnostic = &totals.diagnostics[0];
    assert_eq!(diagnostic.id, "skill_allocation.class_skill_max_rank_exceeded");
    assert!(
        !diagnostic.claim_blocking,
        "a cap violation still yields a real, legal capped total -- it is \
         a diagnostic notice, not a claim-blocking failure"
    );
}

#[test]
fn a_class_skill_at_exactly_the_cap_produces_no_diagnostic() {
    // Level 1 fighter: cap = 4. Allocating exactly 4 ranks must not be
    // treated as a violation.
    let input = input_for(
        fighter_class_levels(1),
        vec![SkillAllocation {
            skill_id: "skill:climb".to_string(),
            ranks: 4,
        }],
    );

    let totals = allocate_skill_ranks(&input);

    assert_eq!(
        totals.totals.get("skill:climb"),
        Some(&SkillTotal {
            ranks: 4,
            ability_modifier: 2,
            class_skill_bonus: 3,
            misc_modifier: 0,
            total_modifier: 9,
        })
    );
    assert!(
        totals.diagnostics.is_empty(),
        "an allocation exactly at the cap is legal and must not produce a \
         cap violation diagnostic"
    );
}

#[test]
fn a_cross_class_skill_over_allocated_past_its_half_cap_is_capped_with_a_diagnostic() {
    // Level 1 fighter: cross-class cap = ceil((1 + 1) / 2) = 1. Diplomacy
    // is a recognized, cross-class (not Fighter class-skill) skill.
    // Allocating 3 ranks exceeds the cap. The cross-class-penalty cycle
    // already silently clipped this to 1; this work-unit adds the
    // diagnostic that clip was missing.
    let input = input_for(
        fighter_class_levels(1),
        vec![SkillAllocation {
            skill_id: "skill:diplomacy".to_string(),
            ranks: 3,
        }],
    );

    let totals = allocate_skill_ranks(&input);

    assert_eq!(
        totals.totals.get("skill:diplomacy"),
        Some(&SkillTotal {
            ranks: 1, // capped at ceil((1 + 1) / 2) = 1, not the raw 3
            ability_modifier: -1,
            class_skill_bonus: 0,
            misc_modifier: 0,
            total_modifier: 0,
        }),
        "the cross-class half-cap's legal, capped total is unchanged by \
         this work-unit -- only the diagnostic is new"
    );
    assert_eq!(
        totals.diagnostics.len(),
        1,
        "an over-allocated cross-class skill must now produce exactly one \
         cap violation diagnostic"
    );
    let diagnostic = &totals.diagnostics[0];
    assert_eq!(diagnostic.id, "skill_allocation.cross_class_max_rank_exceeded");
    assert!(!diagnostic.claim_blocking);
}

#[test]
fn a_cross_class_skill_at_exactly_its_half_cap_produces_no_diagnostic() {
    let input = input_for(
        fighter_class_levels(1),
        vec![SkillAllocation {
            skill_id: "skill:diplomacy".to_string(),
            ranks: 1, // == cross-class cap at level 1
        }],
    );

    let totals = allocate_skill_ranks(&input);

    assert_eq!(
        totals.totals.get("skill:diplomacy"),
        Some(&SkillTotal {
            ranks: 1,
            ability_modifier: -1,
            class_skill_bonus: 0,
            misc_modifier: 0,
            total_modifier: 0,
        })
    );
    assert!(
        totals.diagnostics.is_empty(),
        "an allocation exactly at the cross-class cap is legal and must \
         not produce a cap violation diagnostic"
    );
}

#[test]
fn a_skill_outside_the_bounded_universe_never_produces_a_diagnostic() {
    let input = input_for(
        fighter_class_levels(1),
        vec![SkillAllocation {
            skill_id: "skill:perception".to_string(),
            ranks: 99,
        }],
    );

    let totals = allocate_skill_ranks(&input);

    assert!(!totals.totals.contains_key("skill:perception"));
    assert!(
        totals.diagnostics.is_empty(),
        "a skill outside this module's bounded, cited universe must never \
         be evaluated for a cap violation -- it is simply omitted"
    );
}
