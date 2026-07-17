//! SD-20 Epic 4 — skill-rank allocation engine. Scope draft §1.4.
//!
//! First work-unit (per the loop instruction's Step 2 per-epic order):
//! **class-skill handling**. `allocate_skill_ranks` determines, for each
//! of the character's user-allocated skill ranks
//! (`CharacterInput.chosen.skill_allocations` — the existing, already-wired
//! type; see below), whether that skill is a class skill for the
//! character's class(es), and computes each allocated skill's total
//! modifier. A later Epic-4 cycle adds max-rank-cap handling on top of
//! this module (`SkillTotals.untrained_use` is present at the type level,
//! per `technical-design.md` §2.3, and is populated starting with the
//! third work-unit below; this cycle does not yet enforce a max-rank-cap
//! diagnostic).
//!
//! Second work-unit (this cycle): **cross-class-penalty handling**. See
//! the "PF1 cross-class rule" section below for the exact rule (confirmed
//! against the SRD, not guessed) and what this cycle does and does not
//! cover.
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
//! This cycle widens the bounded ability-key mapping by exactly one
//! skill, `skill:diplomacy` (Charisma-keyed), to make the cross-class path
//! exercisable at all against the grounded Fighter posture: Diplomacy is
//! confirmed *not* a member of Fighter's class-skill list (see the
//! "PF1 cross-class rule" section below for the citation), so it is a
//! genuine, cited cross-class example for a Fighter build — not an
//! invented one.
//!
//! ## PF1 core rule reused as-is
//!
//! A skill that is a class skill for the character and has at least 1
//! rank invested gets a flat +3 trained bonus. This is a system-wide PF1
//! rule (identical for every class and every skill), not per-class table
//! data — the same status this codebase already grants the ability-score
//! modifier formula (`floor(score / 2) - 5`, computed with no corpus
//! citation anywhere in `pilot_compute.rs`).
//!
//! ## PF1 cross-class rule (confirmed, not guessed)
//!
//! Pathfinder 1st Edition removed D&D 3.5's "cross-class skills cost 2
//! skill points per rank" rule (confirmed via a Paizo rules-forum thread
//! discussing precisely this point of common confusion, and independently
//! via the Roll20 PF1 compendium's "Acquiring Skills" page: a skill point
//! always buys exactly 1 rank, regardless of class-skill status). The two
//! real mechanical differences a cross-class skill has in PF1 are:
//!
//! 1. It never gets the class skill's flat +3 trained bonus (already
//!    landed in the class-skill-handling cycle — the existing
//!    `class_skill_bonus` computation already yields `0` for any skill
//!    not in `class_skills`, so no change was needed there).
//! 2. Its maximum investable rank is *half* a class skill's cap, rounded
//!    up: `ceil((character level + 1) / 2)`, versus a class skill's
//!    `character level + 3`, per `scope-draft.md` §1.4's explicit formula.
//!    This module enforces only the cross-class half-cap this cycle — the
//!    class-skill cap (`character level + 3`) and *diagnostic* surfacing
//!    of cap violations for either category are explicitly the later
//!    "max-rank-cap handling" work-unit (see the loop instruction's Step
//!    2 per-epic order). This cycle silently reports the true, legal
//!    effective rank total for a cross-class skill — never the raw
//!    over-allocated number, and never a diagnostic (that's out of this
//!    cycle's scope).
//!
//! `SkillId` "skill:diplomacy"'s cross-class status for Fighter is cited
//! from `cr_abilities_class.lst:2835` ("Fighter Core Class Skills ...
//! CSKILL:Climb|TYPE=Craft|Handle Animal|Intimidate|Knowledge
//! (Dungeoneering)|Knowledge (Engineering)|TYPE=Profession|Ride|Survival|
//! Swim" — no Diplomacy) and its Charisma key from `cr_skills.lst:35`
//! (`Diplomacy ... KEYSTAT:CHA`).
//!
//! Third work-unit (this cycle): **untrained-use handling**.
//!
//! ## PF1 untrained-use rule (confirmed, not guessed)
//!
//! Most PF1 skills may be attempted with zero ranks invested, at the
//! character's raw ability-modifier value (no ranks, no trained bonus —
//! this was already this module's behavior for any recognized skill with
//! `ranks == 0`, prior to this cycle, simply because `class_skill_bonus`
//! is `0` below 1 rank). A bounded set of skills are "Trained Only" per
//! the Core Rulebook's skill summary table (Disable Device, Handle
//! Animal, all Knowledge subtypes, Linguistics, Profession, Sleight of
//! Hand, Spellcraft, Use Magic Device): a character with zero ranks
//! invested in one of those skills cannot attempt the check at all — no
//! total modifier of any kind, fabricated or otherwise, may be reported
//! for it.
//!
//! This cycle widens the bounded ability-key mapping by exactly one
//! skill, `skill:disable_device` (Dexterity-keyed), to make the
//! trained-only path exercisable at all: cited from `cr_skills.lst:36`
//! (`Disable Device ... KEYSTAT:DEX ... USEUNTRAINED:NO`) — the same
//! corpus file this module's earlier cycles already cite for every other
//! recognized skill's key-ability mapping. Disable Device is not a member
//! of Fighter's grounded class-skill list either (see the citation
//! above), so a ranked Disable Device allocation on a Fighter build also
//! exercises the already-landed cross-class path; that is incidental to
//! this cycle, not a new rule.
//!
//! `SkillTotals.untrained_use` (present at the type level since the
//! class-skill-handling cycle, always empty until now) is populated by
//! this cycle: for every recognized, allocated skill whose *final*
//! effective rank count is `0` (which, after the trained-only exclusion
//! above, can only be a skill that is genuinely usable untrained), the
//! map records that skill's raw ability-modifier value — the same number
//! already present in `SkillTotal.total_modifier` for that entry, since
//! ranks and class-skill bonus are both `0` at that point. This is a
//! deliberately narrow, non-fabricating scope: `untrained_use` mirrors
//! only what `totals` already grounds; it does not enumerate every skill
//! in the bounded universe regardless of whether the character allocated
//! it (no such enumeration exists anywhere in this module).

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

/// The bounded, cited set of skills this module recognizes as "Trained
/// Only" — a character with zero ranks invested in one of these cannot
/// attempt the check at all. See the module doc comment's "PF1
/// untrained-use rule" section for the citation and what this is and is
/// not (only skills in this module's bounded ability-key universe can
/// ever appear here; widening it further is future Epic-4 cycle
/// territory).
const TRAINED_ONLY_SKILLS: &[&str] = &["skill:disable_device"];

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
    /// `true` when at least one of the character's allocated skills was a
    /// recognized cross-class skill (not in `class_skills`), meaning PF1's
    /// cross-class half-cap (`ceil((character level + 1) / 2)`, see the
    /// module doc comment's "PF1 cross-class rule" section) was applied to
    /// that skill's effective `ranks` in `totals`. `false` when no
    /// allocated skill fell into that bounded, recognized cross-class
    /// universe.
    pub cross_class_penalty_applied: bool,
    /// One entry per recognized, allocated skill that was actually used
    /// untrained (i.e. its final effective `ranks` in `totals` is `0`),
    /// mapping to that skill's raw ability-modifier value — see the
    /// module doc comment's "PF1 untrained-use rule" section. A
    /// trained-only skill (see [`TRAINED_ONLY_SKILLS`]) with zero ranks
    /// never appears here, or in `totals`, at all: it cannot be attempted
    /// untrained, so no total of any kind is reported for it.
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
        // Cross-class-only for the grounded Fighter posture (Diplomacy is
        // not in Fighter's class-skill list; see the module doc comment's
        // "PF1 cross-class rule" section for the citation). Charisma-keyed
        // per `cr_skills.lst:35`.
        "skill:diplomacy" => Some(ability_modifiers.charisma),
        // Trained-only (see `TRAINED_ONLY_SKILLS` and the module doc
        // comment's "PF1 untrained-use rule" section). Dexterity-keyed
        // per `cr_skills.lst:36`.
        "skill:disable_device" => Some(ability_modifiers.dexterity),
        _ => None,
    }
}

/// Whether `skill_id` is one of this module's bounded, cited "Trained
/// Only" skills. See [`TRAINED_ONLY_SKILLS`] and the module doc comment's
/// "PF1 untrained-use rule" section.
fn is_trained_only_skill(skill_id: &str) -> bool {
    TRAINED_ONLY_SKILLS.contains(&skill_id)
}

/// The character's total level across every class (PF1's "character
/// level" for skill-rank-cap purposes — the sum, not any single class's
/// level).
fn character_level(input: &CharacterInput) -> u16 {
    input
        .chosen
        .class_levels
        .iter()
        .map(|class_level| class_level.level as u16)
        .sum()
}

/// PF1's cross-class maximum rank cap: half a class skill's cap, rounded
/// up — `ceil((character level + 1) / 2)`, per `scope-draft.md` §1.4's
/// explicit formula. See the module doc comment's "PF1 cross-class rule"
/// section for what this is and is not (no diagnostic surfacing here;
/// that's the later max-rank-cap-handling work-unit).
fn cross_class_max_ranks(character_level: u16) -> u8 {
    // ceil((character_level + 1) / 2) == (character_level + 2) / 2 under
    // integer (floor) division.
    ((character_level + 2) / 2) as u8
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
/// recognizes. Applies PF1's cross-class half-cap and trained-only
/// exclusion (see the module doc comment's "PF1 cross-class rule" and
/// "PF1 untrained-use rule" sections). See the module doc comment for
/// what's deliberately not yet handled (the class-skill max-rank cap,
/// cap-violation diagnostics for either category, non-Fighter class-skill
/// postures).
pub fn allocate_skill_ranks(input: &CharacterInput) -> SkillTotals {
    let chassis = compute_pilot_base_chassis(input);
    let class_skills = class_skill_set(input);
    let cross_class_cap = cross_class_max_ranks(character_level(input));
    // The cross-class half-cap is only knowable for a skill when the
    // character has at least one class with a *grounded* class-skill
    // posture (Fighter, this cycle) — only then do we have real PF1
    // evidence that a given skill is cross-class rather than simply
    // unknown. A build with no grounded class-skill posture at all (e.g.
    // an ungrounded "wizard" class id) gets no cross-class treatment,
    // same bounded-caution philosophy `class_skill_set` already follows.
    let has_grounded_class_skill_posture = input
        .chosen
        .class_levels
        .iter()
        .any(|class_level| class_level.class_id == FIGHTER_CLASS_ID);

    let mut totals = BTreeMap::new();
    let mut untrained_use = BTreeMap::new();
    let mut cross_class_penalty_applied = false;
    for allocation in &input.chosen.skill_allocations {
        let Some(ability_mod) =
            skill_key_ability_modifier(&allocation.skill_id, &chassis.ability_modifiers)
        else {
            // Outside the bounded, cited skill universe: no known
            // ability-key mapping. Omit rather than fabricate.
            continue;
        };

        if is_trained_only_skill(&allocation.skill_id) && allocation.ranks == 0 {
            // PF1's untrained-use rule: a trained-only skill with zero
            // ranks invested cannot be attempted at all. See the module
            // doc comment's "PF1 untrained-use rule" section. Omit
            // entirely rather than reporting any total (fabricated or
            // otherwise).
            continue;
        }

        let is_class_skill = class_skills
            .iter()
            .any(|skill_id| skill_id == &allocation.skill_id);

        // Cross-class skills never carry the trained bonus (PF1 rule,
        // already true before this cycle for any non-class-skill) and, for
        // a build whose class-skill posture is actually grounded, are
        // additionally capped at PF1's cross-class half-cap — the ranks
        // actually usable by this character, not the raw allocation. See
        // the module doc comment's "PF1 cross-class rule" section.
        let (ranks, class_skill_bonus) = if is_class_skill {
            let bonus = if allocation.ranks >= 1 {
                TRAINED_CLASS_SKILL_BONUS
            } else {
                0
            };
            (allocation.ranks, bonus)
        } else if has_grounded_class_skill_posture {
            cross_class_penalty_applied = true;
            (allocation.ranks.min(cross_class_cap), 0)
        } else {
            (allocation.ranks, 0)
        };

        let ability_modifier = ability_mod as i8;
        let misc_modifier = 0;
        let total_modifier = ranks as i8 + ability_modifier + class_skill_bonus + misc_modifier;

        if ranks == 0 {
            // Genuinely usable untrained: the trained-only, zero-rank case
            // was already excluded above, so any zero-rank entry reaching
            // this point is a skill PF1 allows to be attempted without
            // ranks. Record its raw ability-modifier value. See the
            // module doc comment's "PF1 untrained-use rule" section.
            untrained_use.insert(allocation.skill_id.clone(), ability_modifier);
        }

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
        cross_class_penalty_applied,
        untrained_use,
    }
}
