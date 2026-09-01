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

    // Re-derived 2026-09-01, same widening and same live value as
    // `multiclass_characters_get_the_union_of_grounded_class_skill_postures` above
    // (`AT-34-E3-003` / `c5c4a1b788` -- `full_fighter_class_skills()`'s real 62-skill list).
    assert_eq!(
        totals.class_skills,
        vec![
            "skill:climb".to_string(),
            "skill:craft_alchemy".to_string(),
            "skill:craft_armor".to_string(),
            "skill:craft_baskets".to_string(),
            "skill:craft_blacksmithing".to_string(),
            "skill:craft_books".to_string(),
            "skill:craft_bows".to_string(),
            "skill:craft_calligraphy".to_string(),
            "skill:craft_carpentry".to_string(),
            "skill:craft_cloth".to_string(),
            "skill:craft_clothing".to_string(),
            "skill:craft_gemcutting".to_string(),
            "skill:craft_glass".to_string(),
            "skill:craft_jewelry".to_string(),
            "skill:craft_leather".to_string(),
            "skill:craft_locks".to_string(),
            "skill:craft_paintings".to_string(),
            "skill:craft_pottery".to_string(),
            "skill:craft_sculptures".to_string(),
            "skill:craft_ships".to_string(),
            "skill:craft_shoes".to_string(),
            "skill:craft_stonemasonry".to_string(),
            "skill:craft_traps".to_string(),
            "skill:craft_weapons".to_string(),
            "skill:handle_animal".to_string(),
            "skill:intimidate".to_string(),
            "skill:knowledge_dungeoneering".to_string(),
            "skill:knowledge_engineering".to_string(),
            "skill:profession_architect".to_string(),
            "skill:profession_baker".to_string(),
            "skill:profession_barrister".to_string(),
            "skill:profession_brewer".to_string(),
            "skill:profession_butcher".to_string(),
            "skill:profession_clerk".to_string(),
            "skill:profession_cook".to_string(),
            "skill:profession_courtesan".to_string(),
            "skill:profession_driver".to_string(),
            "skill:profession_engineer".to_string(),
            "skill:profession_farmer".to_string(),
            "skill:profession_fisherman".to_string(),
            "skill:profession_gambler".to_string(),
            "skill:profession_gardener".to_string(),
            "skill:profession_herbalist".to_string(),
            "skill:profession_innkeeper".to_string(),
            "skill:profession_librarian".to_string(),
            "skill:profession_merchant".to_string(),
            "skill:profession_midwife".to_string(),
            "skill:profession_miller".to_string(),
            "skill:profession_miner".to_string(),
            "skill:profession_porter".to_string(),
            "skill:profession_sailor".to_string(),
            "skill:profession_scribe".to_string(),
            "skill:profession_shepherd".to_string(),
            "skill:profession_soldier".to_string(),
            "skill:profession_soothsayer".to_string(),
            "skill:profession_stable_master".to_string(),
            "skill:profession_tanner".to_string(),
            "skill:profession_trapper".to_string(),
            "skill:profession_woodcutter".to_string(),
            "skill:ride".to_string(),
            "skill:survival".to_string(),
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

    // Re-derived 2026-09-01 (`AT-34-E3-003` / `c5c4a1b788`, 2026-08-2x, already landed at
    // this cycle's HEAD -- `skill_allocation.rs` unchanged since the fable-review's own
    // capture of this exact live value): Fighter's class-skill posture widened from the old
    // 3-skill grounded placeholder to `full_fighter_class_skills()`'s real, corpus-cited
    // full list (`decisions.md §2a`), expanded via `expand_raw_class_skill_list` -- which is
    // why Wizard (still ungrounded here) contributes nothing further but Fighter alone now
    // produces 62 skills, not 3. This IS the union this test's own name promises; the old
    // 3-skill pin predates the widening it never re-derived against.
    assert_eq!(
        totals.class_skills,
        vec![
            "skill:climb".to_string(),
            "skill:craft_alchemy".to_string(),
            "skill:craft_armor".to_string(),
            "skill:craft_baskets".to_string(),
            "skill:craft_blacksmithing".to_string(),
            "skill:craft_books".to_string(),
            "skill:craft_bows".to_string(),
            "skill:craft_calligraphy".to_string(),
            "skill:craft_carpentry".to_string(),
            "skill:craft_cloth".to_string(),
            "skill:craft_clothing".to_string(),
            "skill:craft_gemcutting".to_string(),
            "skill:craft_glass".to_string(),
            "skill:craft_jewelry".to_string(),
            "skill:craft_leather".to_string(),
            "skill:craft_locks".to_string(),
            "skill:craft_paintings".to_string(),
            "skill:craft_pottery".to_string(),
            "skill:craft_sculptures".to_string(),
            "skill:craft_ships".to_string(),
            "skill:craft_shoes".to_string(),
            "skill:craft_stonemasonry".to_string(),
            "skill:craft_traps".to_string(),
            "skill:craft_weapons".to_string(),
            "skill:handle_animal".to_string(),
            "skill:intimidate".to_string(),
            "skill:knowledge_dungeoneering".to_string(),
            "skill:knowledge_engineering".to_string(),
            "skill:profession_architect".to_string(),
            "skill:profession_baker".to_string(),
            "skill:profession_barrister".to_string(),
            "skill:profession_brewer".to_string(),
            "skill:profession_butcher".to_string(),
            "skill:profession_clerk".to_string(),
            "skill:profession_cook".to_string(),
            "skill:profession_courtesan".to_string(),
            "skill:profession_driver".to_string(),
            "skill:profession_engineer".to_string(),
            "skill:profession_farmer".to_string(),
            "skill:profession_fisherman".to_string(),
            "skill:profession_gambler".to_string(),
            "skill:profession_gardener".to_string(),
            "skill:profession_herbalist".to_string(),
            "skill:profession_innkeeper".to_string(),
            "skill:profession_librarian".to_string(),
            "skill:profession_merchant".to_string(),
            "skill:profession_midwife".to_string(),
            "skill:profession_miller".to_string(),
            "skill:profession_miner".to_string(),
            "skill:profession_porter".to_string(),
            "skill:profession_sailor".to_string(),
            "skill:profession_scribe".to_string(),
            "skill:profession_shepherd".to_string(),
            "skill:profession_soldier".to_string(),
            "skill:profession_soothsayer".to_string(),
            "skill:profession_stable_master".to_string(),
            "skill:profession_tanner".to_string(),
            "skill:profession_trapper".to_string(),
            "skill:profession_woodcutter".to_string(),
            "skill:ride".to_string(),
            "skill:survival".to_string(),
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
            // `AT-34-E3-003` widened `skill_ability_key`'s match to all 35 real PF1 Core
            // Rulebook skill categories (`decisions.md §2a`), so "skill:perception" is now
            // genuinely grounded (KEYSTAT Wisdom, `skill_allocation.rs` line ~609) and no
            // longer serves this test's purpose. A skill id outside the real PF1 universe
            // entirely stays the right negative example.
            SkillAllocation {
                skill_id: "skill:not_a_real_pf1_skill".to_string(),
                ranks: 1,
            },
        ],
    );

    let totals = allocate_skill_ranks(&input);

    assert!(totals.totals.contains_key("skill:climb"));
    assert!(
        !totals.totals.contains_key("skill:not_a_real_pf1_skill"),
        "an allocated skill outside the bounded, cited skill universe must \
         not appear in totals with a fabricated ability modifier"
    );
}
