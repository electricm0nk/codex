//! SD-20 Level Up grant model — Epic 7 (`scope-draft.md` §1.7,
//! `technical-design.md` §2.6). **Epic 7 is now CLOSED**: all 11 core
//! classes (barbarian, bard, cleric, druid, fighter, monk, paladin,
//! ranger, rogue, sorcerer, wizard) have a landed `LevelUpPlan`, one per
//! cycle, per Step 2's stated per-class order.
//!
//! Integrated after Epics 2-6 closed (per the loop instruction's
//! dependency graph: "Epic 7 (Level Up grants) integrates after epics
//! 2-6 close" — Epics 1-6 all closed by `062919d`). The parent module
//! (`compute_level_up_grants`, the `LevelUpPlan` shape and its
//! sub-types) landed on Epic 7's first cycle
//! (`src/rules_core/level_up/barbarian.rs`); every subsequent cycle
//! added one more per-class file, ending with
//! `src/rules_core/level_up/wizard.rs` (this cycle).
//!
//! Adapts `technical-design.md` §2.6's illustrative
//! `compute_level_up_grants(character, from_level, to_level) ->
//! LevelUpPlan` seam per §2.0's retirement of the illustrative
//! `rules_tables: &RulesTables` parameter: no such parameter appears
//! here. Per-class files read the specific `rules_tables::crb::<table>`
//! item they need directly (this cycle:
//! `rules_tables::crb::class_tables::class_tables()`), the same way
//! `spellbook/<school>.rs` reads `SPELL_LIST` and
//! `equipment_effects/<category>.rs` reads `equipment_tables`.
//!
//! `LevelUpPlan`'s fields are adapted from the doctrine doc's
//! illustrative shape to compose with data that is *actually* landed in
//! this repo rather than re-deriving it (the same "adapt, don't
//! re-derive" precedent Epic 1's `contract.rs` set for `PilotReceipt`):
//! `automatic_features` composes with two already-grounded sources —
//! `rules_tables::crb::class_tables::class_tables()` (SD-19's BAB/save
//! progression table) for the class-generic pillars, and
//! `pilot_compute::compute_pilot_base_chassis`'s own per-class
//! `explanations` (SD13/SD18's grounded per-level class-feature
//! recognition/magnitude records — e.g. Barbarian's Uncanny Dodge, Trap
//! Sense, Damage Reduction, the Rage constants) for the class-specific
//! pillars. Neither source is re-derived; both are read read-only,
//! exactly as `contract.rs` reads `PilotBaseChassisComputation` and
//! `CorpusDerivedSection` read-only. `pick_from_lists` stays genuinely
//! empty wherever the underlying candidate catalog does not exist yet in
//! this repo (e.g. Barbarian's open-ended Rage Power list has no catalog
//! anywhere in `rules_tables::crb` — the identical "no catalog to
//! enumerate" boundary Epic 3's original feat-catalog blocker hit, here
//! scoped to a single class's own choice-list feature rather than an
//! entire epic, so it is a documented boundary on this cycle's
//! `pick_from_lists`, not a blocker on the whole `LevelUpPlan`).

pub mod barbarian;
pub mod bard;
pub mod cleric;
pub mod druid;
pub mod fighter;
pub mod monk;
pub mod paladin;
pub mod ranger;
pub mod rogue;
pub mod sorcerer;
pub mod wizard;

use crate::rules_core::character_input::CharacterInput;
use crate::rules_core::pilot_compute_corpus::TableCellRef;

/// The core classes this dispatch recognizes — all 11 (barbarian, bard,
/// cleric, druid, fighter, monk, paladin, ranger, rogue, sorcerer,
/// wizard). Epic 7 closed with Wizard's landing (this cycle) — every
/// core class in Step 2's stated order now has a real dispatch arm.
const BARBARIAN_CLASS_ID: &str = "class:barbarian";
const BARD_CLASS_ID: &str = "class:bard";
const CLERIC_CLASS_ID: &str = "class:cleric";
const DRUID_CLASS_ID: &str = "class:druid";
const FIGHTER_CLASS_ID: &str = "class:fighter";
const MONK_CLASS_ID: &str = "class:monk";
const PALADIN_CLASS_ID: &str = "class:paladin";
const RANGER_CLASS_ID: &str = "class:ranger";
const ROGUE_CLASS_ID: &str = "class:rogue";
const SORCERER_CLASS_ID: &str = "class:sorcerer";
const WIZARD_CLASS_ID: &str = "class:wizard";

/// `technical-design.md` §2.6's `LevelUpPlan`, adapted per §2.0 (no
/// `rules_tables: &RulesTables` parameter on the seam that produces it)
/// and per this cycle's "compose, don't fabricate" scope: every field
/// this cycle actually populates cites its source via `TableCellRef`
/// (`Grant.source_table`, `ResourcePoolDelta.source_table`); fields this
/// cycle cannot honestly ground for a given input (e.g. `pick_from_lists`
/// when no candidate catalog exists) stay empty rather than fabricated.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LevelUpPlan {
    /// Grants that fire automatically at `to_level` — no player choice
    /// needed (e.g. "base attack bonus rises to +2", "Uncanny Dodge").
    pub automatic_features: Vec<Grant>,
    /// Open-ended choice-list grants (e.g. "pick 1 feat from this
    /// list"). Empty when the underlying candidate catalog is not yet
    /// grounded anywhere in `rules_tables::crb` for this class/level —
    /// see this module's own doc comment.
    pub pick_from_lists: Vec<PickList>,
    /// Named resource pools whose size changed between `from_level` and
    /// `to_level` (e.g. Barbarian's rage rounds per day).
    pub resource_pool_change: ResourcePoolChange,
    /// New prerequisites unlocked at `to_level` (e.g. a feat that now
    /// becomes selectable because its BAB prerequisite is met). Empty
    /// this cycle — no per-class file yet composes with Epic 3's
    /// `feat_prereqs` output.
    pub prerequisites_added: Vec<Prerequisite>,
    /// True when `to_level` crosses this class's PF1 Core Rulebook
    /// capstone (level 20 for every core class).
    pub capstone_threshold: bool,
}

/// A single automatic or chosen grant, with its provenance.
#[derive(Debug, Clone, PartialEq)]
pub struct Grant {
    pub name: String,
    /// Provenance — which table cell (or grounded chassis explanation
    /// record) this grant's value comes from. Never omitted: every
    /// `Grant` this module produces cites a real, already-landed source.
    pub source_table: TableCellRef,
    pub effects: Vec<GrantEffect>,
}

/// One numeric or narrative effect a `Grant` carries.
#[derive(Debug, Clone, PartialEq)]
pub struct GrantEffect {
    pub description: String,
    pub value: i16,
}

/// An open-ended "pick N from this list" grant.
#[derive(Debug, Clone, PartialEq)]
pub struct PickList {
    pub category: PickCategory,
    pub count: u8,
    pub candidates: Vec<PickCandidate>,
    /// Free-text description of any filter narrowing the candidate list
    /// (e.g. "must satisfy BAB +4"). `None` when unfiltered.
    pub filter: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PickCategory {
    Feat,
    Spell,
    RagePower,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PickCandidate {
    pub id: String,
    pub name: String,
}

/// Named resource-pool size changes between `from_level` and `to_level`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ResourcePoolChange {
    pub pools: Vec<ResourcePoolDelta>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResourcePoolDelta {
    pub pool_id: String,
    pub from_value: i16,
    pub to_value: i16,
    pub source_table: TableCellRef,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Prerequisite {
    pub description: String,
}

/// The Level Up grant model's dispatch seam (`technical-design.md`
/// §2.6, adapted per §2.0). Dispatches on the character's sole class
/// (multiclass layering per `risks-and-open-questions.md` Open Q2 is a
/// future cycle's scope — this cycle bounds to single-class inputs,
/// mirroring `pilot_compute.rs`'s own `supported_barbarian_level` /
/// `supported_bard_level` / `supported_druid_level` single-class gate).
/// Every one of the 11 PF1 Core Rulebook core classes is now recognized
/// (Epic 7 closed with this cycle's Wizard landing). Any class id
/// outside that set of 11 (e.g. a non-core base/hybrid class, or a
/// typo'd id) returns an honestly-empty `LevelUpPlan` rather than a
/// fabricated one — this is a genuine, permanent boundary, not a
/// placeholder for a future cycle.
pub fn compute_level_up_grants(
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) -> LevelUpPlan {
    match character.chosen.class_levels.as_slice() {
        [class_level] if class_level.class_id == BARBARIAN_CLASS_ID => {
            barbarian::compute_barbarian_level_up_grants(character, from_level, to_level)
        }
        [class_level] if class_level.class_id == BARD_CLASS_ID => {
            bard::compute_bard_level_up_grants(character, from_level, to_level)
        }
        [class_level] if class_level.class_id == CLERIC_CLASS_ID => {
            cleric::compute_cleric_level_up_grants(character, from_level, to_level)
        }
        [class_level] if class_level.class_id == DRUID_CLASS_ID => {
            druid::compute_druid_level_up_grants(character, from_level, to_level)
        }
        [class_level] if class_level.class_id == FIGHTER_CLASS_ID => {
            fighter::compute_fighter_level_up_grants(character, from_level, to_level)
        }
        [class_level] if class_level.class_id == MONK_CLASS_ID => {
            monk::compute_monk_level_up_grants(character, from_level, to_level)
        }
        [class_level] if class_level.class_id == PALADIN_CLASS_ID => {
            paladin::compute_paladin_level_up_grants(character, from_level, to_level)
        }
        [class_level] if class_level.class_id == RANGER_CLASS_ID => {
            ranger::compute_ranger_level_up_grants(character, from_level, to_level)
        }
        [class_level] if class_level.class_id == ROGUE_CLASS_ID => {
            rogue::compute_rogue_level_up_grants(character, from_level, to_level)
        }
        [class_level] if class_level.class_id == SORCERER_CLASS_ID => {
            sorcerer::compute_sorcerer_level_up_grants(character, from_level, to_level)
        }
        [class_level] if class_level.class_id == WIZARD_CLASS_ID => {
            wizard::compute_wizard_level_up_grants(character, from_level, to_level)
        }
        _ => LevelUpPlan::default(),
    }
}

/// Dedicated multiclass entry point (SD-25 Epic 3 Criterion 3.2; carry-forward
/// register A2). `compute_level_up_grants` above dispatches on the *whole*
/// `class_levels` roster shape (`[class_level]`) — a single-element-slice
/// pattern that can never match a 2+-class mix, so a Fighter+Wizard
/// character fell through to the honestly-empty `_ => LevelUpPlan::default()`
/// arm even though `fighter::compute_fighter_level_up_grants` and
/// `wizard::compute_wizard_level_up_grants` already support that exact mix
/// internally (`is_fighter_or_supported_fighter_wizard_mix` /
/// `is_wizard_or_supported_fighter_wizard_mix`, SD-24 Epic 5 criterion 5.1)
/// once actually invoked. The gap was purely in this top-level dispatcher's
/// own match shape, not in the per-class functions themselves.
///
/// This entry point dispatches on the caller's explicit `leveling_class_id`
/// — the specific class actually gaining a level — rather than requiring the
/// whole roster to be single-class, so it routes correctly whether
/// `character` carries one class or a supported multiclass mix. The
/// existing 3-argument `compute_level_up_grants` above is left unchanged
/// (its ~50 existing call sites across `tests/*.rs` and `contract.rs` are
/// outside this criterion's file-touch grant) — this is the "add a
/// dedicated multiclass entry point" option register A2 itself offers as an
/// alternative to widening the existing signature.
pub fn compute_level_up_grants_for_class(
    character: &CharacterInput,
    leveling_class_id: &str,
    from_level: u8,
    to_level: u8,
) -> LevelUpPlan {
    match leveling_class_id {
        BARBARIAN_CLASS_ID => barbarian::compute_barbarian_level_up_grants(character, from_level, to_level),
        BARD_CLASS_ID => bard::compute_bard_level_up_grants(character, from_level, to_level),
        CLERIC_CLASS_ID => cleric::compute_cleric_level_up_grants(character, from_level, to_level),
        DRUID_CLASS_ID => druid::compute_druid_level_up_grants(character, from_level, to_level),
        FIGHTER_CLASS_ID => fighter::compute_fighter_level_up_grants(character, from_level, to_level),
        MONK_CLASS_ID => monk::compute_monk_level_up_grants(character, from_level, to_level),
        PALADIN_CLASS_ID => paladin::compute_paladin_level_up_grants(character, from_level, to_level),
        RANGER_CLASS_ID => ranger::compute_ranger_level_up_grants(character, from_level, to_level),
        ROGUE_CLASS_ID => rogue::compute_rogue_level_up_grants(character, from_level, to_level),
        SORCERER_CLASS_ID => sorcerer::compute_sorcerer_level_up_grants(character, from_level, to_level),
        WIZARD_CLASS_ID => wizard::compute_wizard_level_up_grants(character, from_level, to_level),
        // A genuine, permanent boundary (mirrors `compute_level_up_grants`'s
        // own `_` arm) -- an unrecognized class id, not a multiclass-mix
        // artifact.
        _ => LevelUpPlan::default(),
    }
}

#[cfg(test)]
mod multiclass_dispatch_tests {
    use super::*;
    use crate::rules_core::character_input::{
        AbilityScores, CharacterClassLevel, ChosenCharacterState,
    };

    const HUMAN_RACE_ID: &str = "race:human";

    fn human_fighter_wizard_mix(fighter_level: u8, wizard_level: u8) -> CharacterInput {
        CharacterInput {
            case_id: Some("fighter_wizard_mix_level_up".to_owned()),
            source_package_id: "fighter_wizard_mix_level_up".to_owned(),
            chosen: ChosenCharacterState {
                race_id: HUMAN_RACE_ID.to_owned(),
                class_levels: vec![
                    CharacterClassLevel {
                        class_id: FIGHTER_CLASS_ID.to_owned(),
                        level: fighter_level,
                    },
                    CharacterClassLevel {
                        class_id: WIZARD_CLASS_ID.to_owned(),
                        level: wizard_level,
                    },
                ],
                ability_scores: AbilityScores {
                    strength: 16,
                    dexterity: 14,
                    constitution: 14,
                    intelligence: 14,
                    wisdom: 12,
                    charisma: 8,
                },
                selected_feats: Vec::new(),
                skill_allocations: Vec::new(),
                equipment_selections: Vec::new(),
                selected_choices: Vec::new(),
                selected_traits: Vec::new(),
                spells_selected: Vec::new(),
                class_ability_activations: Vec::new(),
            },
            selection_provenance: Vec::new(),
        }
    }

    /// RED -> GREEN proof for carry-forward register A2: leveling the
    /// Fighter side of a Fighter+Wizard mix through the dedicated
    /// multiclass entry point must produce real grants, not the top-level
    /// dispatcher's multiclass-gap empty default. Before this cycle, no
    /// entry point could express this call at all (RED: the top-level
    /// `compute_level_up_grants(&character, 1, 2)` for this same fixture
    /// returns `LevelUpPlan::default()` because the 2-element
    /// `class_levels` slice never matches any `[class_level]` arm).
    #[test]
    fn compute_level_up_grants_for_class_does_not_return_an_empty_plan_for_fighter_side_of_a_fighter_wizard_mix() {
        let character = human_fighter_wizard_mix(1, 1);

        // The pre-existing top-level dispatcher still cannot express this
        // (documents the RED this criterion's carry-forward note names).
        let via_old_dispatcher = compute_level_up_grants(&character, 1, 2);
        assert_eq!(
            via_old_dispatcher,
            LevelUpPlan::default(),
            "documents the multiclass-gap this criterion's dedicated entry point fixes"
        );

        let plan = compute_level_up_grants_for_class(&character, FIGHTER_CLASS_ID, 1, 2);

        assert!(
            !plan.automatic_features.is_empty(),
            "leveling Fighter within a Fighter+Wizard mix must produce real \
             grants via the dedicated multiclass entry point: {plan:?}"
        );
    }

    /// The Wizard side of the identical mix must dispatch correctly too.
    #[test]
    fn compute_level_up_grants_for_class_does_not_return_an_empty_plan_for_wizard_side_of_a_fighter_wizard_mix() {
        let character = human_fighter_wizard_mix(1, 1);

        let plan = compute_level_up_grants_for_class(&character, WIZARD_CLASS_ID, 1, 2);

        assert!(
            !plan.automatic_features.is_empty(),
            "leveling Wizard within the same Fighter+Wizard mix must also \
             produce real grants: {plan:?}"
        );
    }

    /// A genuinely unrecognized class id (not a multiclass-mix artifact)
    /// still returns the honest empty default -- the fix narrows the gap to
    /// exactly the multiclass-shape problem, it does not fabricate grants
    /// for classes this crate has no per-class file for.
    #[test]
    fn compute_level_up_grants_for_class_still_returns_an_honest_empty_plan_for_an_unrecognized_class() {
        let mut character = human_fighter_wizard_mix(1, 1);
        character.chosen.class_levels = vec![CharacterClassLevel {
            class_id: "class:oracle".to_owned(),
            level: 1,
        }];

        let plan = compute_level_up_grants_for_class(&character, "class:oracle", 1, 2);

        assert_eq!(plan, LevelUpPlan::default());
    }
}
