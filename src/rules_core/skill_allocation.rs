//! SD-20 Epic 4 — skill-rank allocation engine. Scope draft §1.4.
//!
//! First work-unit (per the loop instruction's Step 2 per-epic order):
//! **class-skill handling**. `allocate_skill_ranks` determines, for each
//! of the character's user-allocated skill ranks
//! (`CharacterInput.chosen.skill_allocations` — the existing, already-wired
//! type; see below), whether that skill is a class skill for the
//! character's class(es), and computes each allocated skill's total
//! modifier. Later Epic-4 cycles add cross-class-penalty handling,
//! untrained-use handling, and max-rank-cap handling on top of this
//! module (`SkillTotals.cross_class_penalty_applied` and `.untrained_use`
//! are present at the type level, per `technical-design.md` §2.3, but
//! stay at their bounded defaults — `false` / empty — until those later
//! work-units land; this cycle does not claim to enforce them).
//!
//! ## Deviation from `technical-design.md` §2.3's illustrative seam
//!
//! `technical-design.md` §2.3 illustrates
//! `allocate_skill_ranks(input, allocation: &SkillAllocation, rules_tables:
//! &RulesTables) -> SkillTotals`. Two of those three parameters don't
//! survive contact with the real repo, mirroring the doc-note precedent
//! `pilot_compute_corpus.rs` already set ("`PilotReceipt` in the doctrine
//! doc's illustrative code does not exist in this repo"):
//!
//! - `allocation: &SkillAllocation` (a `BTreeMap<SkillId, u8>` wrapper) is
//!   redundant with the already-landed
//!   `CharacterInput.chosen.skill_allocations: Vec<character_input::SkillAllocation>`
//!   (`skill_id: String, ranks: u8` per record) — the character's own
//!   ranks-per-skill choices are already on the wire type. Introducing a
//!   second, differently-shaped carrier for the same information would
//!   duplicate the single wire contract every SD-20 Epic-1 cycle to date
//!   has kept singular (`classify_character_input(&CharacterInput)`,
//!   `to_pilot_receipt(&CorpusPilotReceipt)` both take the composed input
//!   type directly, no echo parameter).
//! - `rules_tables: &RulesTables` has no defined type anywhere in this
//!   repo as of this cycle (confirmed by grep: zero hits outside
//!   `technical-design.md`'s own illustrative code). It is referenced
//!   identically across Epic 2/3/4/5's seam signatures, so it is shared
//!   infrastructure no single epic's file-touch partition (own parent
//!   module only) can safely invent unilaterally without risking a
//!   shape collision with a concurrent sibling stream — this exact
//!   reasoning is independently recorded by the Epic-3 sibling cycle's
//!   blocked-cycle log entry (`~/workspace/SD-20-rules-engine-completeness-progress.md`,
//!   cycle-2026-07-17T1920).
//!
//! ## Class-skill data source
//!
//! `src/rules_core/rules_tables/crb/` carries no class-skill-list table
//! today — only `class_tables.rs`'s per-class-per-level base-attack-bonus
//! and base-save rows (`ClassTableRow`), confirmed by grep across the
//! whole `rules_tables` tree before writing this cycle's RED test.
//! Extending that table store is out of SD-20's authority (SD-19 owns the
//! table store's shape; see the loop instruction's hard stops, "SD-20
//! cannot extend it autonomously") and `rules_tables/crb/` sits outside
//! this epic's file-touch partition regardless.
//!
//! Unlike Epic 3's feat catalog (verified absent *everywhere* in the
//! repo, informal or otherwise — see the Epic-3 blocked-cycle log entry
//! above), a class-skill fact already exists, grounded and shipped, in
//! `pilot_compute.rs`'s deterministic bounded posture: the comment block
//! above `compute_selected_skill_modifiers` cites `cr_abilities_class.lst:2835`
//! for "Fighter class skills include Climb, Intimidate, Swim" (also
//! `cr_skills.lst:10/42/102` for each skill's own `TYPE=ClassSkill`
//! entry). This module reuses those same three already-cited skill
//! identities — not a duplicate derivation, the same underlying evidence
//! — scoped identically (Fighter only, these three skills only). It does
//! not import `pilot_compute.rs`'s private constants (they're not `pub`);
//! it composes with the chassis's own already-computed, public
//! `AbilityModifiers` (via `compute_pilot_base_chassis`) for the ability
//! side of the math, rather than re-deriving ability modifiers.
//!
//! No skill or class outside this bounded, cited posture is ever claimed
//! as a class skill, and no skill outside the bounded ability-key mapping
//! below is ever given a fabricated `ability_modifier` — such allocations
//! are simply absent from `SkillTotals.totals` rather than populated with
//! an invented number. Widening this posture (more classes, more skills)
//! is future Epic-4 cycle territory, gated on either a genuine SD-19
//! table-store extension or the operator authorizing one.
//!
//! ## PF1 core rule reused as-is
//!
//! A skill that is a class skill for the character and has at least 1
//! rank invested gets a flat +3 trained bonus. This is a system-wide PF1
//! rule (identical for every class and every skill), not per-class table
//! data — the same status this codebase already grants the ability-score
//! modifier formula (`floor(score / 2) - 5`, computed with no corpus
//! citation anywhere in `pilot_compute.rs`).

use std::collections::BTreeMap;

use crate::rules_core::character_input::CharacterInput;
use crate::rules_core::pilot_compute::{compute_pilot_base_chassis, AbilityModifiers};

/// Plain-string skill identity, matching
/// `character_input::SkillAllocation.skill_id`'s existing convention —
/// class/item/spell identities are all `String` throughout this crate;
/// no typed `SkillId` enum exists, and none is introduced here.
pub type SkillId = String;

/// `character_input::CharacterClassLevel.class_id` value that hits the
/// grounded Fighter class-skill posture below. Matches
/// `pilot_compute.rs`'s own `FIGHTER_CLASS_ID` value (verified by
/// reading its source; that constant itself is not `pub`, so it is not
/// imported, only matched by value).
const FIGHTER_CLASS_ID: &str = "class:fighter";

/// The bounded, cited Fighter class-skill posture. See the module doc
/// comment's "Class-skill data source" section for the citation.
const GROUNDED_FIGHTER_CLASS_SKILLS: &[&str] = &["skill:climb", "skill:intimidate", "skill:swim"];

/// PF1 core rule: flat trained bonus for any class skill with at least 1
/// rank invested. A system-wide constant, not per-class/per-book table
/// data — see the module doc comment's closing section.
const TRAINED_CLASS_SKILL_BONUS: i8 = 3;

/// Output of [`allocate_skill_ranks`]. Per `technical-design.md` §2.3.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SkillTotals {
    /// One entry per skill the character actually allocated ranks to
    /// *and* that this module's bounded, cited posture recognizes (see
    /// the module doc comment). Skills outside that bounded posture are
    /// omitted here rather than populated with a fabricated modifier.
    pub totals: BTreeMap<SkillId, SkillTotal>,
    /// The character's full class-skill set (union across all of the
    /// character's classes that this module's bounded posture
    /// recognizes), independent of whether ranks were actually
    /// allocated to each member. Sorted for determinism.
    pub class_skills: Vec<SkillId>,
    /// Not yet enforced by this cycle (a later Epic-4 work-unit per the
    /// loop instruction's Step 2: "cross-class-penalty handling").
    /// Always `false` until that work-unit lands.
    pub cross_class_penalty_applied: bool,
    /// Not yet populated by this cycle (a later Epic-4 work-unit per the
    /// loop instruction's Step 2: "untrained-use handling"). Always
    /// empty until that work-unit lands.
    pub untrained_use: BTreeMap<SkillId, i8>,
}

/// One skill's computed total, per `technical-design.md` §2.3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SkillTotal {
    pub ranks: u8,
    pub ability_modifier: i8,
    pub class_skill_bonus: i8,
    /// Not yet populated by this cycle (armor-check-penalty and similar
    /// contributions are Epic 5's territory; a future integration cycle
    /// composes them in). Always `0` until then.
    pub misc_modifier: i8,
    pub total_modifier: i8,
}

/// The bounded, cited ability-key mapping for the skills this module
/// currently recognizes at all (see the module doc comment). Returns
/// `None` for any skill id outside that bounded universe — callers must
/// not fabricate a modifier in that case.
fn skill_key_ability_modifier(skill_id: &str, ability_modifiers: &AbilityModifiers) -> Option<i16> {
    match skill_id {
        "skill:climb" | "skill:swim" => Some(ability_modifiers.strength),
        "skill:intimidate" => Some(ability_modifiers.charisma),
        _ => None,
    }
}

/// The character's class-skill set: the union, across every class the
/// character has levels in, of that class's grounded class-skill
/// posture. Only Fighter has a grounded posture as of this cycle (see
/// the module doc comment); every other class contributes nothing.
fn class_skill_set(input: &CharacterInput) -> Vec<SkillId> {
    let has_fighter = input
        .chosen
        .class_levels
        .iter()
        .any(|class_level| class_level.class_id == FIGHTER_CLASS_ID);

    let mut class_skills: Vec<SkillId> = if has_fighter {
        GROUNDED_FIGHTER_CLASS_SKILLS
            .iter()
            .map(|skill_id| (*skill_id).to_string())
            .collect()
    } else {
        Vec::new()
    };
    class_skills.sort();
    class_skills
}

/// Computes per-skill rank totals for every skill the character both
/// allocated ranks to and that this module's bounded, cited posture
/// recognizes. See the module doc comment for what's deliberately not
/// yet handled (cross-class penalty, untrained use, max-rank cap,
/// non-Fighter class-skill postures).
pub fn allocate_skill_ranks(input: &CharacterInput) -> SkillTotals {
    let chassis = compute_pilot_base_chassis(input);
    let class_skills = class_skill_set(input);

    let mut totals = BTreeMap::new();
    for allocation in &input.chosen.skill_allocations {
        let Some(ability_mod) =
            skill_key_ability_modifier(&allocation.skill_id, &chassis.ability_modifiers)
        else {
            // Outside the bounded, cited skill universe: no known
            // ability-key mapping. Omit rather than fabricate.
            continue;
        };

        let is_class_skill = class_skills
            .iter()
            .any(|skill_id| skill_id == &allocation.skill_id);
        let class_skill_bonus = if is_class_skill && allocation.ranks >= 1 {
            TRAINED_CLASS_SKILL_BONUS
        } else {
            0
        };

        let ability_modifier = ability_mod as i8;
        let ranks = allocation.ranks;
        let misc_modifier = 0;
        let total_modifier = ranks as i8 + ability_modifier + class_skill_bonus + misc_modifier;

        totals.insert(
            allocation.skill_id.clone(),
            SkillTotal {
                ranks,
                ability_modifier,
                class_skill_bonus,
                misc_modifier,
                total_modifier,
            },
        );
    }

    SkillTotals {
        totals,
        class_skills,
        cross_class_penalty_applied: false,
        untrained_use: BTreeMap::new(),
    }
}
