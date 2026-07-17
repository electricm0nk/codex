//! SD-20 Level Up grant model — Epic 7 (`scope-draft.md` §1.7,
//! `technical-design.md` §2.6).
//!
//! Integrates after Epics 2-6 close (per the loop instruction's
//! dependency graph: "Epic 7 (Level Up grants) integrates after epics
//! 2-6 close" — Epics 1-6 all closed by `062919d`). This is Epic 7's
//! first cycle: the parent module (`compute_level_up_grants`, the
//! `LevelUpPlan` shape and its sub-types) plus the first per-class file
//! (`src/rules_core/level_up/barbarian.rs`), per Step 2's stated
//! per-class order (barbarian, then bard, ..., then wizard — 11 core
//! classes total, one per cycle).
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

use crate::rules_core::character_input::CharacterInput;
use crate::rules_core::pilot_compute_corpus::TableCellRef;

/// The core classes this cycle's dispatch recognizes so far. Widens by
/// one per future cycle (druid, fighter, ..., wizard), per Step 2's
/// stated order.
const BARBARIAN_CLASS_ID: &str = "class:barbarian";
const BARD_CLASS_ID: &str = "class:bard";
const CLERIC_CLASS_ID: &str = "class:cleric";

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
/// mirroring `pilot_compute.rs`'s own `supported_barbarian_level`
/// single-class gate). Unrecognized classes (every core class except
/// Barbarian, as of this cycle) return an honestly-empty `LevelUpPlan`
/// rather than a fabricated one.
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
        _ => LevelUpPlan::default(),
    }
}
