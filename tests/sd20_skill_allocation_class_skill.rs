//! SD-20 Epic 4 (skill-rank allocation engine): class-skill handling —
//! cycle 1.
//!
//! Per `SD-20-rules-engine-completeness-scope-draft.md` §1.4 and the loop
//! instruction's Step 2 ("one skill-class category per cycle — class-skill
//! handling, then cross-class-penalty handling, then untrained-use
//! handling, then max-rank-cap handling"), this is Epic 4's first
//! work-unit: determining which of a character's user-allocated skill
//! ranks land on a class skill, and computing each such skill's total
//! modifier.
//!
//! This cycle's class-skill determination is grounded in the *only*
//! class-skill fact already established anywhere in this codebase with
//! real corpus-line evidence: `pilot_compute.rs`'s deterministic bounded
//! posture, which cites `cr_abilities_class.lst:2835` for "Fighter class
//! skills include Climb, Intimidate, Swim" (see the comment block above
//! `compute_selected_skill_modifiers` in `src/rules_core/pilot_compute.rs`).
//! `src/rules_core/rules_tables/crb/` carries no class-skill-list table
//! today (only `class_tables.rs`'s per-class-per-level base-attack-bonus
//! and base-save rows) — confirmed by grep across the whole `rules_tables`
//! tree before writing this test. Extending that table store is out of
//! SD-20's authority (SD-19 owns it). This module therefore reuses the
//! same three already-cited, already-shipped skill identities rather than
//! inventing new class-skill data — no skill or class outside that bounded
//! posture is ever claimed as a class skill.

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState, SkillAllocation,
};
use codex::rules_core::skill_allocation::{allocate_skill_ranks, SkillTotal};

fn base_ability_scores() -> AbilityScores {
    AbilityScores {
        strength: 14,     // modifier +2
        dexterity: 12,    // modifier +1
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
        case_id: Some("sd20_skill_allocation_class_skill".to_string()),
        source_package_id: "sd20_skill_allocation_class_skill".to_string(),
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

fn fighter_class_levels() -> Vec<CharacterClassLevel> {
    vec![CharacterClassLevel {
        class_id: "class:fighter".to_string(),
        level: 1,
    }]
}

#[test]
fn fighter_class_skills_get_the_flat_plus_three_trained_bonus_when_ranked() {
    let input = input_for(
        fighter_class_levels(),
        vec![
            SkillAllocation {
                skill_id: "skill:climb".to_string(),
                ranks: 2,
            },
            SkillAllocation {
                skill_id: "skill:intimidate".to_string(),
                ranks: 1,
            },
            // Zero ranks invested: still a class skill, but PF1's flat
            // trained bonus only applies once at least 1 rank is
            // invested, so this must NOT carry the +3.
            SkillAllocation {
                skill_id: "skill:swim".to_string(),
                ranks: 0,
            },
        ],
    );

    let totals = allocate_skill_ranks(&input);

    assert_eq!(
        totals.class_skills,
        vec![
            "skill:climb".to_string(),
            "skill:intimidate".to_string(),
            "skill:swim".to_string(),
        ],
        "Fighter's grounded class-skill set is recognized regardless of \
         whether ranks were actually allocated to each member"
    );

    assert_eq!(
        totals.totals.get("skill:climb"),
        Some(&SkillTotal {
            ranks: 2,
            ability_modifier: 2, // strength 14 -> +2
            class_skill_bonus: 3,
            misc_modifier: 0,
            total_modifier: 7,
        })
    );
    assert_eq!(
        totals.totals.get("skill:intimidate"),
        Some(&SkillTotal {
            ranks: 1,
            ability_modifier: -1, // charisma 8 -> -1
            class_skill_bonus: 3,
            misc_modifier: 0,
            total_modifier: 3,
        })
    );
    assert_eq!(
        totals.totals.get("skill:swim"),
        Some(&SkillTotal {
            ranks: 0,
            ability_modifier: 2, // strength 14 -> +2
            class_skill_bonus: 0, // 0 ranks invested: no trained bonus yet
            misc_modifier: 0,
            total_modifier: 2,
        }),
        "a class skill with zero ranks invested must not carry the +3 \
         trained bonus"
    );
}

#[test]
fn a_class_without_a_grounded_class_skill_posture_gets_no_fabricated_bonus() {
    let input = input_for(
        vec![CharacterClassLevel {
            class_id: "wizard".to_string(),
            level: 1,
        }],
        vec![SkillAllocation {
            skill_id: "skill:climb".to_string(),
            ranks: 2,
        }],
    );

    let totals = allocate_skill_ranks(&input);

    assert!(
        totals.class_skills.is_empty(),
        "no class-skill posture is grounded for any class besides the \
         bounded Fighter case, so nothing may be claimed as a class skill"
    );
    assert_eq!(
        totals.totals.get("skill:climb"),
        Some(&SkillTotal {
            ranks: 2,
            ability_modifier: 2,
            class_skill_bonus: 0,
            misc_modifier: 0,
            total_modifier: 4,
        }),
        "the same skill (Climb) computed for a non-Fighter build must not \
         carry the class-skill bonus"
    );
}

#[test]
fn multiclass_characters_get_the_union_of_grounded_class_skill_postures() {
    let mut class_levels = vec![CharacterClassLevel {
        class_id: "wizard".to_string(),
        level: 1,
    }];
    class_levels.extend(fighter_class_levels());

    let input = input_for(
        class_levels,
        vec![SkillAllocation {
            skill_id: "skill:swim".to_string(),
            ranks: 3,
        }],
    );

    let totals = allocate_skill_ranks(&input);

    assert_eq!(
        totals.class_skills,
        vec![
            "skill:climb".to_string(),
            "skill:intimidate".to_string(),
            "skill:swim".to_string(),
        ],
        "a multiclass character carrying the grounded Fighter class level \
         still gets the Fighter class-skill set, alongside any other \
         class's (ungrounded here, so contributing nothing further)"
    );
    assert_eq!(
        totals.totals.get("skill:swim"),
        Some(&SkillTotal {
            ranks: 3,
            ability_modifier: 2,
            class_skill_bonus: 3,
            misc_modifier: 0,
            total_modifier: 8,
        })
    );
}

#[test]
fn skills_outside_the_bounded_grounded_universe_are_never_fabricated() {
    let input = input_for(
        fighter_class_levels(),
        vec![
            SkillAllocation {
                skill_id: "skill:climb".to_string(),
                ranks: 1,
            },
            // "skill:perception" has no grounded ability-key mapping
            // anywhere in this codebase yet (no cited corpus evidence);
            // the engine must not invent one.
            SkillAllocation {
                skill_id: "skill:perception".to_string(),
                ranks: 1,
            },
        ],
    );

    let totals = allocate_skill_ranks(&input);

    assert!(totals.totals.contains_key("skill:climb"));
    assert!(
        !totals.totals.contains_key("skill:perception"),
        "an allocated skill outside the bounded, cited skill universe must \
         not appear in totals with a fabricated ability modifier"
    );
}
