//! SD-20 Epic 4 (skill-rank allocation engine): cross-class-penalty
//! handling — cycle 2.
//!
//! Per `SD-20-rules-engine-completeness-scope-draft.md` §1.4 and the loop
//! instruction's Step 2 ("one skill-class category per cycle — class-skill
//! handling, then cross-class-penalty handling, then untrained-use
//! handling, then max-rank-cap handling"), this is Epic 4's second
//! work-unit.
//!
//! **The exact PF1 rule (confirmed, not guessed).** Pathfinder 1st Edition
//! removed D&D 3.5's "cross-class skills cost 2 skill points per rank"
//! rule. In PF1, a skill point always buys exactly 1 rank regardless of
//! class-skill status — the only two mechanical differences between a
//! class skill and a cross-class skill are (a) the flat +3 trained bonus,
//! which only class skills get (already landed in the class-skill-handling
//! cycle, `6c9b4af`), and (b) the maximum rank a character may invest in
//! the skill: a class skill caps at character level + 3, a cross-class
//! skill caps at half that, rounded up — `ceil((character level + 1) / 2)`
//! per `scope-draft.md` §1.4's explicit formula. This cycle implements
//! (b): the cross-class rank cap. General over-allocation *diagnostics*
//! (for either category) are explicitly the later "max-rank-cap handling"
//! work-unit, not this one; this cycle silently reports the true, legal
//! effective rank total for a cross-class skill (never a fabricated one)
//! rather than surfacing a diagnostic.
//!
//! Cross-class skill under test: `skill:diplomacy` (Charisma-keyed per
//! `cr_skills.lst` line 35, `KEYSTAT:CHA`), confirmed *not* a member of
//! Fighter's class-skill list (`cr_abilities_class.lst` line 2835,
//! `Fighter Core Class Skills ... CSKILL:Climb|TYPE=Craft|Handle
//! Animal|Intimidate|Knowledge (Dungeoneering)|Knowledge
//! (Engineering)|TYPE=Profession|Ride|Survival|Swim` — no Diplomacy).

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
        charisma: 14,     // modifier +2
    }
}

fn input_for(
    class_levels: Vec<CharacterClassLevel>,
    skill_allocations: Vec<SkillAllocation>,
) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_skill_allocation_cross_class".to_string()),
        source_package_id: "sd20_skill_allocation_cross_class".to_string(),
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
fn a_cross_class_skill_gets_no_trained_bonus_and_flags_the_penalty_as_applied() {
    // Character level 1: cross-class cap = ceil((1 + 1) / 2) = 1, so 1
    // allocated rank fits under the cap untouched.
    let input = input_for(
        fighter_class_levels(1),
        vec![SkillAllocation {
            skill_id: "skill:diplomacy".to_string(),
            ranks: 1,
        }],
    );

    let totals = allocate_skill_ranks(&input);

    assert!(
        !totals.class_skills.contains(&"skill:diplomacy".to_string()),
        "Diplomacy is not in Fighter's grounded class-skill set"
    );
    assert_eq!(
        totals.totals.get("skill:diplomacy"),
        Some(&SkillTotal {
            ranks: 1,
            ability_modifier: 2, // charisma 14 -> +2
            class_skill_bonus: 0,
            misc_modifier: 0,
            total_modifier: 3,
        }),
        "a cross-class skill never carries the class-skill trained bonus"
    );
    assert!(
        totals.cross_class_penalty_applied,
        "allocating ranks to a recognized cross-class skill must flag that \
         the cross-class penalty was correctly applied"
    );
}

#[test]
fn cross_class_ranks_beyond_the_half_cap_are_capped_not_fabricated() {
    // Character level 4: cross-class cap = ceil((4 + 1) / 2) = 3. The
    // character allocated 4 ranks -- illegal under PF1 chargen -- so the
    // engine must report the true, legal effective total (3), never the
    // raw over-allocated number.
    let input = input_for(
        fighter_class_levels(4),
        vec![SkillAllocation {
            skill_id: "skill:diplomacy".to_string(),
            ranks: 4,
        }],
    );

    let totals = allocate_skill_ranks(&input);

    assert_eq!(
        totals.totals.get("skill:diplomacy"),
        Some(&SkillTotal {
            ranks: 3, // capped at ceil((4 + 1) / 2) = 3, not the raw 4
            ability_modifier: 2,
            class_skill_bonus: 0,
            misc_modifier: 0,
            total_modifier: 5,
        }),
        "a cross-class skill's effective ranks must never exceed PF1's \
         half-cap, even when the raw allocation is higher"
    );
    assert!(totals.cross_class_penalty_applied);
}

#[test]
fn the_cross_class_cap_never_leaks_into_a_class_skills_own_total() {
    // Same build, but also allocates ranks to a class skill (Climb) that
    // would exceed the cross-class cap if (incorrectly) subjected to it.
    // Climb's full ranks must survive untouched.
    let input = input_for(
        fighter_class_levels(4),
        vec![
            SkillAllocation {
                skill_id: "skill:climb".to_string(),
                ranks: 4,
            },
            SkillAllocation {
                skill_id: "skill:diplomacy".to_string(),
                ranks: 4,
            },
        ],
    );

    let totals = allocate_skill_ranks(&input);

    assert_eq!(
        totals.totals.get("skill:climb"),
        Some(&SkillTotal {
            ranks: 4, // class skill: never capped by the cross-class rule
            ability_modifier: 2, // strength 14 -> +2
            class_skill_bonus: 3,
            misc_modifier: 0,
            total_modifier: 9,
        }),
        "the cross-class cap must never reduce a class skill's own ranks"
    );
    assert_eq!(
        totals.totals.get("skill:diplomacy").map(|t| t.ranks),
        Some(3),
        "the cross-class skill in the same build is still capped"
    );
}

#[test]
fn a_build_with_only_class_skill_allocations_never_flags_the_cross_class_penalty() {
    let input = input_for(
        fighter_class_levels(1),
        vec![SkillAllocation {
            skill_id: "skill:climb".to_string(),
            ranks: 1,
        }],
    );

    let totals = allocate_skill_ranks(&input);

    assert!(
        !totals.cross_class_penalty_applied,
        "the flag must not be fabricated when no cross-class skill was \
         actually allocated"
    );
}
