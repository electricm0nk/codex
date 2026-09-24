//! GE-06 pilot deterministic rules-core computation surface.
//!
//! Computes and explains the bounded set of outputs accumulated across the GE-06
//! pilot slices for the accepted PF1 Human Fighter level-1 deterministic input:
//!
//! - ability modifiers (`floor(score/2) - 5`)
//! - Fighter level-1 base attack bonus
//! - Fighter level-1 base saves
//! - baseline melee attack bonus for the deterministic Longsword loadout
//! - baseline armor class for the deterministic Chain Shirt / Dodge / no-shield posture
//! - total Fortitude / Reflex / Will saves (base save + relevant ability modifier)
//!
//! Each computed value carries a machine-checkable explanation record. This is
//! intentionally not a full rules engine: it does not compute feat-, item-, or
//! condition-based save modifiers, weapon damage, active Power Attack math,
//! initiative, skill modifiers, armor-check penalties, equipment effects beyond
//! the deterministic baseline, feat prerequisites, or any oracle parity. Support
//! is the bounded deterministic Human Fighter posture widened across the SD13-E3
//! milestone tranche from level 1 to levels 2 and 3 only: the level-2 bonus-feat
//! progression seam and the level-3 armor-training seam are surfaced explicitly,
//! but nothing here grounds level-4+ Fighter burden, a general feat-effect engine,
//! spellcasting, multiclassing, or non-Fighter positive support. The SD13-E3-F6 slice
//! additionally recognizes the deterministic Human Paladin level-1 and Human Ranger
//! level-1 hybrid chassis as direct runtime evidence, but keeps both explicitly
//! claim-blocked on their still-missing non-spell class-feature burden and later spell
//! burden; it grounds no hybrid class-feature or spell math. The SD13-E4-F7 slice
//! also recognizes the deterministic Human Sorcerer level-1 spell-bearing identity as
//! a direct runtime evidence, but keeps it explicitly claim-blocked on its
//! bloodline and spontaneous known-spell / slot posture burdens; it grounds no
//! bloodline power and no spell math. The SD13-E4-R3 slice further recognizes the
//! deterministic Human Wizard level-1 prepared arcane spell-bearing identity as
//! direct runtime evidence, but keeps it explicitly claim-blocked on its school
//! specialization burden and prepared spellbook / spells-prepared / spell-slot
//! posture burden; it grounds no spellbook content, no spells prepared, no spell
//! slots, no spell save DCs, no bonus spells, no school-opposition bookkeeping, and
//! no specialty school bonus. A later SD13-E4 Wizard decomposition slice splits that
//! school specialization burden in two: Scribe Scroll, the free specialization-
//! independent bonus feat every 1st-level Wizard is granted, is grounded for real,
//! while the specialization CHOICE burden (chosen school, opposed schools, specialty
//! school bonus) stays its own named claim-blocking diagnostic; the prepared
//! spellbook / spells-prepared / spell-slot posture burden is untouched. The SD13-E5
//! Wizard specialization slice then grounds the flat surface of that choice for real:
//! the canonical Evocation specialization with Necromancy and Transmutation opposed
//! is recognized, and the specialist bonus slot is grounded as a flat count (one
//! 1st-level Evocation-only slot at level 1, no cantrip-level slot, no slot
//! contents), narrowing the claim-blocker to the school powers (intense spells,
//! force missile) and the opposed-school two-slot preparation cost. The SD13-E3
//! Fighter milestone tranche has since
//! widened further still, to level 8: the level-8 bonus-feat progression seam is
//! surfaced explicitly, mirroring the level-2/4/6 bonus-feat seams, and grounds no
//! level-9+ Fighter burden. The SD13-E3 Rogue pillar-grounding slice widens the
//! deterministic Human Rogue level-1 chassis to ground base-attack progression
//! (3/4 BAB), base-save progression (good Reflex, poor Fortitude, poor Will), and
//! the sneak attack damage-die count (1, i.e. 1d6); the SD13-E5 Rogue slice grounds
//! the fourth named pillar, Trapfinding (the flat max(rogue level / 2, 1) bonus on
//! Perception checks to locate traps and on Disable Device checks, plus the
//! magic-trap-disarm statement), so no named Rogue pillar burden remains
//! claim-blocked, and `defense.total_save.*` is still never computed for
//! it. The SD13-E3 Barbarian level-1 martial chassis slice is widened further here:
//! base-attack progression, base-save progression, and the fast-movement +10 ft.
//! speed value are now grounded as standalone explanation records (mirroring the
//! Fighter formula shape). The SD13-E5 Barbarian slice then resolves the
//! formerly-named illiteracy burden as vacuous (the PF1 Core Rulebook Barbarian is
//! not illiterate; illiteracy is a D&D 3.5e trait that never existed in PF1) and
//! grounds Rage's flat numeric surface — rage rounds per day (4 + Constitution
//! modifier) and the flat rage constants — values only, leaving the rage-state
//! execution engine explicitly claim-blocked as the honest remaining Barbarian
//! burden. A later SD13-E3 slice widens the deterministic Human Monk
//! level-1 chassis to ground base-attack, base-save, and AC Bonus (Wisdom-to-AC),
//! while keeping unarmed strike / Flurry of Blows and the level-1 bonus feat grant
//! explicitly claim-blocked. The SD13-E3 Ranger decomposition further splits the F6
//! Ranger non-spell class-feature burden into three named pillars: favored enemy and
//! combat style stay explicitly claim-blocked by their own named diagnostics, and
//! Track (the Survival-check bonus to follow tracks, ½ ranger level minimum 1) is
//! grounded for real as a bounded numeric value; it grounds no combat-style math and
//! no ranger spell posture. The SD13-E5 Ranger Favored Enemy slice then grounds the
//! favored-enemy flat surface for real — recognition of the chosen favored-enemy type
//! (from `choice:ranger_favored_enemy`), the flat +2 bonus on Bluff, Knowledge,
//! Perception, Sense Motive, and Survival checks against the favored enemy, and the
//! flat +2 bonus on weapon attack and damage rolls against the favored enemy (PF1
//! includes attack rolls, unlike D&D 3.5) — retiring the favored-enemy claim-blocking
//! diagnostic while grounding no target-type matching or conditional-application
//! engine. A later SD13-E5 Ranger Combat Style slice corrects a mistaken framing in
//! the combat-style diagnostic (it previously claimed the archery-vs-two-weapon-combat
//! style choice was a level-1 decision separate from a level-2 bonus-feat grant; PF1
//! Core Rulebook actually grants the style choice and its first bonus feat TOGETHER at
//! 2nd level) and retires the claim-blocking diagnostic in favor of a grounded
//! level-gate absence record (value 0), mirroring the Paladin mercy level-gate idiom —
//! no bonus-feat mechanical value is fabricated. The SD13-E4 Druid Wild Empathy
//! slice further splits the Druid nature-bond/wild-empathy class-feature blocker
//! into two named diagnostics: nature bond (the animal-companion-vs-domain choice
//! and nature sense) stays explicitly claim-blocked, and Wild Empathy (PF1 Core
//! Rulebook: 1d20 + druid level + Charisma modifier, used like a Diplomacy check to
//! improve an animal's attitude) is grounded for real as the flat druid-level +
//! Cha-modifier value; it grounds no nature-bond power execution and no
//! Diplomacy-check/d20-roll resolution. The SD13-E5 Druid Nature Sense /
//! nature-bond-choice slice grounds Nature Sense for real (PF1 Core Rulebook: a
//! flat, level-independent +2 bonus on Knowledge (nature) and Survival checks,
//! kept as a standalone record not wired into any skill total), recognizes the
//! deterministic `choice:druid_nature_bond -> bond:animal_companion` selection as
//! a +0 recognition record, and narrows the retired combined nature-bond blocker
//! to the chosen bond's execution (companion stat block, companion advancement,
//! link / share spells), which stays claim-blocked. The SD13-E4 Sorcerer decomposition
//! slice further splits the F7 combined bloodline burden into two named diagnostics
//! and grounds one for real: Eschew Materials, the universal, bloodline-independent
//! bonus feat every 1st-level Sorcerer receives; it grounds no bloodline power,
//! bloodline arcana, or spell math. The SD13-E5 Sorcerer bloodline-choice slice then
//! recognizes the canonical deterministic bloodline choice-slot selection
//! (`choice:sorcerer_bloodline -> bloodline:arcane`) as chosen input — recognition
//! only, since the Arcane bloodline's level-1 power is Arcane Bond (a familiar or a
//! bonded object), an execution engine rather than a flat number — and narrows the
//! former bloodline-power blocker to the Arcane Bond / bloodline progression burden;
//! that burden and the spontaneous spell-posture burden stay explicitly
//! claim-blocked. Unsupported input yields
//! claim-blocking diagnostics and withheld explanations rather than fabricated values.

use super::character_input::{
    AbilityScores, ActiveState, AcquisitionMode, CharacterClassLevel, CharacterInput,
    SkillAllocation,
};
use super::description_completion::{feat_description_completion, ZeroMagnitudeResolution};
use super::feat_prereqs::metamagic::{evaluate_metamagic_feat_prerequisites, resolve_metamagic_feat_effect};
use super::rules_tables::acg::{self, AcgClassId};
use super::rules_tables::acg::shaman_spell_list;
use super::rules_tables::acg::hunter_spell_list;
use super::rules_tables::advanced_race_guide;
use super::rules_tables::apg::{self, ApgClassId};
use super::rules_tables::class_spell_levels;
use super::rules_tables::apg::alchemist_spell_list;
use super::rules_tables::apg::inquisitor_spell_list;
use super::rules_tables::apg::witch_spell_list;
use super::rules_tables::pathfinder_unchained::class_chassis::{self as pu_class_chassis, PuClassId};
use super::rules_tables::pathfinder_unchained::{
    barbarian_features, monk_features, rogue_features, summoner_features,
};
use super::rules_tables::ultimate_combat::{self as uc, UcClassId};
use crate::rules_core::archetype_resolver;
use crate::rules_core::durability::FamiliarSpecies;
use crate::rules_core::feat_identity;
// SD-35 `AT-35-E6-003`: this file used to `use crate::pcgen_import::pcgen_desc::{…}` here and
// render a record's `DESC:` tokens at run time. It reads the converted `data/sheet_rules/`
// prose instead — `pilot_compute::resolved_prose`, declared above.
use crate::rules_core::pilot_compute::resolved_prose::{resolved_description, DisplayValues};
use crate::rules_core::race_resolver::race_size_for_race_token;
use crate::rules_core::size::SizeCategory;
use super::rules_tables::crb::class_tables::{ClassId, class_tables, good_saves_for};
use super::rules_tables::crb::paladin_spell_list;
use super::rules_tables::crb::bard_spell_list;
use super::rules_tables::crb::cleric_spell_list;
use super::rules_tables::crb::druid_spell_list;
use super::rules_tables::crb::ranger_spell_list;
use super::rules_tables::crb::sorcerer_spell_list;
use super::rules_tables::crb::spell_list::{Pf1SchoolId, SPELL_LIST};
use super::rules_tables::crb::weapon_tables;
use super::rules_tables::RuleSetId;

// SD31-E4-F1-005: per-class modules split out of this file, a pure code-move
// (unchanged behaviour -- see the split's own commit message and receipt).
// Each submodule opens with `use super::*;`, so it sees this module's own
// private items exactly as it did before the move (child modules see a
// parent's private items in Rust). The blanket `use` below re-imports each
// submodule's `pub(super)` items back into this module's own namespace, so
// every pre-existing call site here -- and this file's own inline unit test
// modules, several thousand lines below, that call these functions via
// `super::<name>` -- keeps resolving unqualified, name for name, with no
// call site edited.
// `pub(crate)`, not private: `derived_evaluator_fixture_check.rs`'s own `class_feature_
// description_entries` bar check (SD-31 wave 26) drives this module's REAL `resolve_pcgen_var_
// chain`/`class_level_variable_name` directly, rather than a second, duplicated implementation --
// both live inside the `codex` crate, so `pub(crate)` is enough; no `tests/`-crate-visible `pub`
// is needed or added.
pub mod class_feature_grant_consumer;
mod class_slayer;
mod class_ultimate_combat;
pub mod prestige_class_entry_gate;
pub mod untabled_base_class_chassis;
pub mod untabled_base_class_feature_roster;
// SD-35 AT-35-E6-001 (`decisions.md` §11): the PCGen formula interpreter, its reproduction
// harness, its corpus-wide scan, the `BonusObj`-shape bonus-stack reader and the race-trait
// formula binding are CONVERTER code. They were on the live side; they now live under
// `src/pcgen_import/` (`crate::pcgen_import::formula_interpreter`, `::formula_reproduction_harness`,
// `::formula_interpreter_corpus_wide`, the bonus-stack reader, `::race_trait_formula_binding`).
// They are KEPT, not deleted -- they are reused for Starfinder (`decisions.md` §11, what is kept).
/// SD-32 T12 `epic-10-reference-library-residual-reach` row 20 cycles 5-7 — the generic
/// companion base-ability-score table, generalizing `ground_wolf_companion_stat_block`/
/// `ground_horse_companion_stat_block` (below) into a table-driven function. `pub(crate)`,
/// mirroring `class_feature_grant_consumer`'s own visibility: `apps/desktop/src-tauri` calls
/// this module only indirectly, through this file's own `ground_selected_companion_or_default`
/// (cycle 7's new dispatch point, reading the new `COMPANION_SPECIES_CHOICE_ID` choice set) —
/// `ground_companion_stat_block` itself stays `pub(crate)`, never crossing the crate boundary
/// directly. See this module's own doc comment for the cycle 7 wiring addendum.
pub(crate) mod companion_base_stat_table;
/// SD-32 T12 `epic-10-reference-library-residual-reach` row 20 cycle 5 — see its own module doc
/// comment. `pub(crate)`: `resolve` is called from `compute_class_chassis` below (same crate);
/// no `apps/desktop/src-tauri` caller exists yet.
/// SD-35 `AT-35-E6-001`: the ONE live reader of the converted class chassis
/// (`data/sheet_rules/<book>/class/<slug>.json`). `pub` because the desktop
/// crate's `class_catalog_generic` reads the same records for the
/// reference-library browser rather than mirroring the derivation a second
/// time.
pub mod class_chassis_sheet_rules;
/// SD-36 Epic F §3.4: a class's weapon proficiency read from the converted package -- the
/// source for every class without a `CLASS_WEAPON_PROFICIENCIES` row. `pub` so the
/// codex-ingest oracle pin (`class_weapon_proficiency_via_converter.rs`) can call it.
pub mod class_proficiency_sheet_rules;
pub mod class_skill_sheet_rules;
mod generic_class_chassis;
mod multiclass_fold;
/// SD-34 `AT-34-E3-001` (`decisions.md §14`, mechanism `class_absent_from_
/// ClassId_ALL_and_book_class_id_enums`) -- see its own module doc comment.
/// `pub`: `modelled_class_books()` in `src/bin/v06_work_inventory.rs` (a
/// separate compilation unit) calls `covered_classes()` directly, the same
/// visibility shape `untabled_base_class_chassis` already uses.
pub mod crb_untabled_class_chassis;
/// SD-34 `AT-34-E3-001`: `pub` so `v06_work_inventory.rs` can call
/// `domain_power::domain_power_probe_catalog()` directly, the same
/// visibility shape `crb_untabled_class_chassis` above already uses.
pub mod domain_power;
// SD-35 `AT-35-E6-003` (`decisions.md` §11): a converted rule's own words rendered with this
// character's numbers, read from `data/sheet_rules/`. It replaces the verbatim `DESC:` token
// constants and the run-time PCGen renderer this file used to hold.
pub mod resolved_prose;
pub use class_slayer::*;
use class_ultimate_combat::compute_uc_class_chassis;
use domain_power::*;

/// SD-36 Epic F0a (`docs/release/SD-36-consolidation/epic-f-class-completion.md`
/// §2): a re-export, not a visibility widening of the `generic_class_chassis`
/// module itself (still crate-private, `mod generic_class_chassis;` above).
/// `rules_core::class_census`, a sibling of `pilot_compute` rather than a
/// child of it, needs `generic_class_chassis::covered_classes()` to build
/// the merged census; this is the one seam that lets it in without widening
/// anything else this module's own private items rely on staying private.
pub(crate) use generic_class_chassis::covered_classes as generic_class_chassis_covered_classes;

// SD-36 Epic C1 (`docs/release/SD-36-consolidation/technical-design.md` §3):
// this file used to hold ~88,800 lines of per-class dispatch, per-class
// tests, and shared combat/race/feat/companion/spellcasting machinery
// directly. It is now split by class/system into the submodules below, each
// starting with `use super::*;` (so it sees this module's own private items
// exactly as before the move -- a child module always sees its parent's
// private items in Rust) and blanket re-exported back here, so every
// pre-existing `pilot_compute::<name>` call site keeps resolving unqualified,
// name for name, with no call site edited. Pure code move; no behaviour
// changed (mirrors the SD31-E4-F1-005 split above, for the same reason).
mod class_alchemist_investigator;
mod class_barbarian;
mod class_bard_skald;
mod class_cleric;
mod class_dispatch;
mod class_druid_shaman;
mod class_fighter;
mod class_hunter_cavalier_swashbuckler;
mod class_inquisitor_warpriest;
mod class_monk;
mod class_occult_and_psionic;
mod class_oracle;
mod class_paladin_ranger;
mod class_rogue;
mod class_shared_core;
mod class_sorcerer_wizard;
mod class_summoner_witch;
mod class_wizard_prepared_spellbook;
mod combat;
mod companion;
mod feat_pillar_and_pool_aggregation;
mod feat_pillars;
mod pool_groups;
mod prestige_class_features;
mod prestige_class_features_campaign;
mod race_seams;
mod skills_and_saves;
mod spellcasting;
mod untabled_base_class_features;
// `pub`, not `pub(crate)`: some of the moved items were bare `pub fn`/`pub
// struct`/... directly in this module before the split (reachable from
// OUTSIDE this crate, e.g. `apps/desktop/src-tauri` and `crates/codex-ingest`
// both call `pilot_compute::` paths directly). A `pub use` re-export of a
// `pub(super)` item is legal (this location has `pub(super)` visibility into
// each child) and widening visibility this way changes no behaviour.
#[allow(unused_imports)]
pub use self::{
    class_alchemist_investigator::*,
    class_barbarian::*,
    class_bard_skald::*,
    class_cleric::*,
    class_dispatch::*,
    class_druid_shaman::*,
    class_fighter::*,
    class_hunter_cavalier_swashbuckler::*,
    class_inquisitor_warpriest::*,
    class_monk::*,
    class_occult_and_psionic::*,
    class_oracle::*,
    class_paladin_ranger::*,
    class_rogue::*,
    class_shared_core::*,
    class_sorcerer_wizard::*,
    class_summoner_witch::*,
    class_wizard_prepared_spellbook::*,
    combat::*,
    companion::*,
    feat_pillar_and_pool_aggregation::*,
    feat_pillars::*,
    pool_groups::*,
    prestige_class_features::*,
    prestige_class_features_campaign::*,
    race_seams::*,
    skills_and_saves::*,
    spellcasting::*,
    untabled_base_class_features::*,
};
