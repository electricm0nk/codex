//! SD-20 Epic 4 (skill-rank allocation engine): untrained-use handling —
//! cycle 3.
//!
//! Per `SD-20-rules-engine-completeness-scope-draft.md` §1.4 and the loop
//! instruction's Step 2 ("one skill-class category per cycle — class-skill
//! handling, then cross-class-penalty handling, then untrained-use
//! handling, then max-rank-cap handling"), this is Epic 4's third
//! work-unit.
//!
//! **The exact PF1 rule (confirmed, not guessed).** Most PF1 skills may be
//! used with zero ranks invested, at the character's raw ability-modifier
//! value (no ranks, no trained bonus). A bounded set of skills are
//! "Trained Only" per the Core Rulebook's skill summary table (e.g.
//! Disable Device, Handle Animal, all Knowledge subtypes, Linguistics,
//! Profession, Sleight of Hand, Spellcraft, Use Magic Device) — a
//! character with zero ranks invested in one of those skills cannot
//! attempt the check at all.
//!
//! Widened, cited posture for this cycle: `skill:disable_device`
//! (Dexterity-keyed) is added to the module's bounded, cited skill
//! universe and marked trained-only, per `cr_skills.lst:36`
//! (`Disable Device ... KEYSTAT:DEX ... USEUNTRAINED:NO`). This is the
//! same corpus this module's earlier cycles already cite for the other
//! recognized skills' key-ability mappings (`cr_skills.lst:10/42/102`
//! confirmed by the class-skill-handling cycle; `cr_skills.lst:35` for
//! Diplomacy confirmed by the cross-class-penalty cycle). No skill outside
//! this bounded, cited universe is ever claimed trained-only or given a
//! fabricated ability modifier.

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
        case_id: Some("sd20_skill_allocation_untrained".to_string()),
        source_package_id: "sd20_skill_allocation_untrained".to_string(),
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
fn a_trained_only_skill_with_zero_ranks_cannot_be_used_at_all() {
    let input = input_for(
        fighter_class_levels(1),
        vec![SkillAllocation {
            skill_id: "skill:disable_device".to_string(),
            ranks: 0,
        }],
    );

    let totals = allocate_skill_ranks(&input);

    assert!(
        !totals.totals.contains_key("skill:disable_device"),
        "a trained-only skill with zero ranks invested must not appear in \
         totals at all -- it cannot be attempted untrained, so no total \
         modifier (fabricated or otherwise) may be reported for it"
    );
    assert!(
        !totals.untrained_use.contains_key("skill:disable_device"),
        "a trained-only skill must never be recorded as usable untrained"
    );
}

#[test]
fn a_trained_only_skill_with_ranks_invested_is_usable_normally() {
    // Character level 4: cross-class cap = ceil((4 + 1) / 2) = 3, so 2
    // ranks fits under the cap untouched. Disable Device is not a member
    // of Fighter's grounded class-skill set, so this also exercises the
    // (already-landed) cross-class path alongside the trained-only rule.
    let input = input_for(
        fighter_class_levels(4),
        vec![SkillAllocation {
            skill_id: "skill:disable_device".to_string(),
            ranks: 2,
        }],
    );

    let totals = allocate_skill_ranks(&input);

    assert_eq!(
        totals.totals.get("skill:disable_device"),
        Some(&SkillTotal {
            ranks: 2,
            ability_modifier: 2, // dexterity 15 -> +2
            class_skill_bonus: 0,
            misc_modifier: 0,
            total_modifier: 4,
        }),
        "a trained-only skill with at least 1 rank invested is usable and \
         computed normally"
    );
    assert!(
        !totals.untrained_use.contains_key("skill:disable_device"),
        "untrained_use only records skills actually used untrained (zero \
         ranks); a ranked skill must not appear there"
    );
}

#[test]
fn a_skill_usable_untrained_with_zero_ranks_reports_its_raw_ability_modifier() {
    let input = input_for(
        fighter_class_levels(1),
        vec![SkillAllocation {
            skill_id: "skill:climb".to_string(),
            ranks: 0,
        }],
    );

    let totals = allocate_skill_ranks(&input);

    assert_eq!(
        totals.totals.get("skill:climb"),
        Some(&SkillTotal {
            ranks: 0,
            ability_modifier: 2, // strength 14 -> +2
            class_skill_bonus: 0,
            misc_modifier: 0,
            total_modifier: 2,
        }),
        "a skill that is not trained-only remains usable with zero ranks, \
         at the raw ability modifier"
    );
    assert_eq!(
        totals.untrained_use.get("skill:climb"),
        Some(&2),
        "untrained_use must record the raw ability-modifier value for a \
         skill actually used untrained"
    );
}

#[test]
fn a_ranked_skill_is_not_recorded_in_untrained_use() {
    let input = input_for(
        fighter_class_levels(1),
        vec![SkillAllocation {
            skill_id: "skill:climb".to_string(),
            ranks: 2,
        }],
    );

    let totals = allocate_skill_ranks(&input);

    assert!(totals.totals.contains_key("skill:climb"));
    assert!(
        !totals.untrained_use.contains_key("skill:climb"),
        "untrained_use must only record skills that were actually used \
         untrained (zero ranks), never a ranked skill"
    );
}

#[test]
fn a_skill_outside_the_bounded_universe_is_never_fabricated_in_either_map() {
    // `AT-34-E3-003` widened the ability-key match to all 35 real PF1 skills
    // (`skill_allocation.rs` line ~609 grounds "skill:perception" now), so a skill id
    // outside the real PF1 universe entirely is the right negative example here.
    let input = input_for(
        fighter_class_levels(1),
        vec![SkillAllocation {
            skill_id: "skill:not_a_real_pf1_skill".to_string(),
            ranks: 0,
        }],
    );

    let totals = allocate_skill_ranks(&input);

    assert!(!totals.totals.contains_key("skill:not_a_real_pf1_skill"));
    assert!(!totals.untrained_use.contains_key("skill:not_a_real_pf1_skill"));
}
