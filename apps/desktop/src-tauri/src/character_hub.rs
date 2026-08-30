//! Character Hub — Tauri command adapter over the real rules-core compute
//! engine and saved-character persistence.
//!
//! Bridges the desktop shell and the headless `codex` crate: composes a
//! `CharacterInput` from the caller's race/class/level choice with a fixed
//! feat/skill/equipment loadout (none of the compute seam's class-specific
//! diagnostics read those selections, so widening them would not change
//! which combinations compute), computes it via the real engine, and
//! persists successful builds through `SavedCharacterStore`. A `Blocked`
//! result is never written to disk — this module never saves an unproven
//! build. Only single-class Fighter at any of levels 1-3 reaches `Computed`
//! today; every other class/level combination returns real claim-blocking
//! diagnostics from the engine, verbatim.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use base64::Engine as _;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use uuid::Uuid;

use codex::rules_core::character_input::{
    AbilityScores, AcquisitionMode, ActiveState, CharacterClassLevel, CharacterInput,
    ChosenCharacterState, EquipmentSelection, SelectedChoice, SkillAllocation, SpellSelection,
};
use codex::rules_core::damage_total::{resolve_weapon_damage_breakdown, WeaponDamageBreakdown};
use codex::rules_core::durability::{classify_durability, compute_max_hp, DurabilityStatus};
use codex::rules_core::feat_effects;
use codex::rules_core::level_up::{compute_level_up_grants_for_class, LevelUpPlan};
use codex::rules_core::money;
use codex::rules_core::pilot_compute::{
    ability_modifier, apply_human_ability_bonus, build_pilot_headless_receipt,
    race_alternate_trait_selection_id, ComputationExplanation, HeadlessReceiptStatus,
    RACE_ALTERNATE_TRAIT_CHOICE_ID, RACE_ALTERNATE_TRAIT_SELECTION_PREFIX,
};
use codex::rules_core::pilot_compute_corpus::{
    compute_pilot_with_corpus, CorpusDerivedSection, ResolvedEquipment,
};
use codex::rules_core::pilot_view_model::{PilotSnapshot, PilotSpellbookViewModel};

use crate::corpus_fixtures::corpus_fixture_bundle;
use codex::saved_character::local_store::SavedCharacterStore;
use codex::saved_character::{
    SavedCharacterEnvelope, SavedCharacterRevisionKind, SavedCharacterSummary,
    CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
};

// `pub(crate)` (rather than private) so `pf1_adapter.rs`'s
// `compose_character_input` (moved there SD-25 Criterion 3.2) can read the
// same two constants this module's own `create_character`/`clone_character`/
// `seed_default_character_if_needed`/`import_character_from_json` still use.
pub(crate) const HUMAN_RACE_ID: &str = "race:human";
pub(crate) const SOURCE_PACKAGE_ID: &str = "pf1.core_rulebook";
const GAME_SYSTEM_ID: &str = "pf1";
const CHARACTERS_ROOT_DIR_NAME: &str = "characters";

// ----- DTOs (manual re-projection: the `codex` crate has zero dependencies,
// so its types are not `Serialize`/`Deserialize`) -----

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityScoresDto {
    pub strength: i16,
    pub dexterity: i16,
    pub constitution: i16,
    pub intelligence: i16,
    pub wisdom: i16,
    pub charisma: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityModifiersDto {
    pub strength: i16,
    pub dexterity: i16,
    pub constitution: i16,
    pub intelligence: i16,
    pub wisdom: i16,
    pub charisma: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseSavesDto {
    pub fortitude: i16,
    pub reflex: i16,
    pub will: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectedSkillModifiersDto {
    pub climb: i16,
    pub intimidate: i16,
    pub swim: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PilotSnapshotDto {
    pub ability_modifiers: AbilityModifiersDto,
    pub base_attack_bonus: i16,
    pub base_saves: BaseSavesDto,
    pub baseline_melee_attack_bonus: i16,
    pub baseline_armor_class: i16,
    pub total_saves: BaseSavesDto,
    pub selected_skill_modifiers: SelectedSkillModifiersDto,
    /// v0.6 alpha swarm (risks-and-open-questions.md item 6): the flat DR
    /// magnitude from a grounded class-feature DR explanation (currently
    /// only Barbarian's), or absent when no such record exists or its
    /// magnitude is the level-gate absence value of 0. See
    /// `PilotDefenseViewModel::damage_reduction`'s own doc comment
    /// (`pilot_view_model.rs`) for the full derivation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_reduction: Option<i16>,
    /// v0.6 alpha swarm: the character's animal companion or mount, or
    /// absent when this build grounds none.
    ///
    /// The stat block was fully computed in the engine across all twenty
    /// master levels the whole time (`pilot_compute.rs`'s
    /// `ground_wolf_companion_stat_block` /
    /// `ground_horse_companion_stat_block`, for Druid, Hunter and the
    /// Cavalier's Mount) but had no field to travel in, so the desktop
    /// sheet's Pets tab rendered a "coming soon" placeholder over data
    /// that already existed -- the same defect
    /// `EquipmentEffectsDto::per_item` had.
    ///
    /// Same `skip_serializing_if` discipline as `damage_reduction`: a
    /// companion-less class omits the key entirely rather than sending a
    /// literal `null` the frontend's `!== undefined` checks would wave
    /// through into an empty stat block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companion: Option<AnimalCompanionDto>,
    /// epic-31-spell-wiring: the character's real spellbook coverage
    /// (spell save DCs, slots total/used), from
    /// `spellbook::compute_spellbook_coverage`. Absent, not zeroed, for a
    /// non-caster or a build with no spell yet resolved against the
    /// corpus -- same discipline as `damage_reduction`/`companion` above.
    /// See `PilotSpellbookViewModel`'s own doc comment for the twin
    /// problem this closes and Decision 37 for why it carries no slot
    /// totals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spellbook: Option<PilotSpellbookDto>,
}

/// Wire form of `pilot_view_model::PilotSpellSaveDc`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpellSaveDcDto {
    pub class_id: String,
    pub dc: u8,
}

/// Wire form of `pilot_view_model::PilotSpellbookViewModel`. Deliberately
/// has no `slots_total`/`slots_used` fields -- see
/// `PilotSpellbookViewModel`'s doc comment and `decisions.md` Decision 37
/// (epic-31-spell-wiring gap closure, 2026-08-07).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PilotSpellbookDto {
    pub spell_save_dc: Vec<SpellSaveDcDto>,
}

/// Wire form of `pilot_view_model::PilotCompanionStat` -- one grounded
/// companion statistic, with the engine's own label, value and derivation
/// prose carried verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanionStatDto {
    pub label: String,
    pub value: i16,
    pub detail: String,
}

/// Wire form of `pilot_view_model::PilotCompanionViewModel`.
///
/// A wholly separate creature: none of these values are applied to the
/// character's own integrated totals, and the sheet must not mix them in.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimalCompanionDto {
    pub owner_class_label: String,
    pub role_label: String,
    pub species: String,
    pub summary_detail: String,
    /// Only statistics the engine actually emitted -- never zero-filled.
    pub stats: Vec<CompanionStatDto>,
    pub notes: Vec<String>,
    /// The engine's non-blocking `advancement_absent` note: the honest
    /// list of companion columns deliberately left ungrounded because
    /// nothing in this codebase consumes them. It travels here rather
    /// than in `diagnostics` because `load_saved_character` returns an
    /// empty diagnostics list on the `Computed` path -- so this is the
    /// player's only route to it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advancement_note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolCoverageDto {
    /// e.g. "Abjuration" — the `Pf1SchoolId` variant name, verbatim.
    pub school: String,
    /// Corpus spell identities resolved for this school, sorted.
    pub spells: Vec<String>,
    /// Whether the resolved spell(s) also ground through the foundation
    /// slice's bootstrap table cell (`TableCellRef`), not just the corpus.
    pub grounded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedEquipmentDto {
    /// The `CharacterInput.equipment_selections[].item_id` verbatim.
    pub item_id: String,
    pub equipment_record_name: String,
    pub equipment_record_key: String,
    /// Whether this item also grounds through the foundation slice's
    /// bootstrap table cell (`TableCellRef`), not just the corpus.
    pub grounded: bool,
    /// v0.6 alpha swarm items 1+27 sub-task 6: this selection's own
    /// resolved `applied_modifiers` (e.g. a resolved "+1 Enhancement to
    /// Weapon" attached to this Longsword) -- reuses this same DTO shape
    /// rather than a new type, since a resolved modifier is structurally
    /// just another resolved equipment record. Empty for a selection with
    /// no attached modifiers, or whose modifiers all failed to resolve
    /// (those surface via `CorpusDerivedDto.unresolvedEquipmentItemIds`
    /// instead, same list a top-level unresolvable selection already
    /// uses).
    pub applied_modifiers: Vec<ResolvedEquipmentDto>,
}

/// v0.6 alpha swarm item 1, shape (c) (`item-1-architecture-wall-design.md`):
/// the real, corpus-resolved aggregate equipment-effect totals for the
/// character's currently `EquippedActive` items. Explicitly NOT claim-gated
/// -- unlike `PilotSnapshotDto.baselineArmorClass` (the deterministic-posture
/// value the `Computed`/`Blocked` gate itself is built on), these numbers
/// reflect whatever real gear is actually equipped, corpus-resolved,
/// regardless of whether the build reaches `Computed`.
///
/// `attack_bonus_delta` (the bounded single-weapon slice) is `null` on the
/// wire whenever zero or two-or-more weapons are `EquippedActive` --
/// `CharacterInput`'s schema has no field recording which weapon a modifier
/// item attaches to, so with more than one weapon equipped, which one an
/// enhancement modifies is genuinely ambiguous (see
/// `EquipmentEffects.attack_bonus_delta`'s own doc comment,
/// `equipment_effects.rs`). With exactly one weapon equipped it is a real,
/// unambiguous value, including a real `0` when no enhancement applies --
/// the frontend must treat `null` as "not shown" (ambiguous), not as zero.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentEffectsDto {
    /// Per-equipped-item contributions behind the aggregate totals below --
    /// the "AC breakdown by source" data. Empty when nothing is equipped.
    pub per_item: Vec<ResolvedEquipmentEffectDto>,
    pub armor_class_delta: i16,
    pub armor_check_penalty_total: i16,
    /// v0.6 alpha swarm (QA finding, 2026-07-24): without `skip_serializing_if`,
    /// a Rust `None` here serialized as `"maxDexCap":null` -- key present,
    /// literal `null`, not an omitted key -- so the frontend's `!== undefined`
    /// hide-checks (`null !== undefined` is `true`) never fired, rendering
    /// garbled `"+null"`/`"null%"` strings instead of hiding the field. Mirrors
    /// the precedent `PilotSnapshotDto.damage_reduction` already set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_dex_cap: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spell_failure_chance: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_bonus_delta: Option<i16>,
    /// The HIGHEST `per_item[].spellResistanceBonus` among everything
    /// equipped (`equipment_effects::EquipmentEffects.spell_resistance_
    /// total`'s own doc comment: PF1's real rule, multiple SR sources
    /// take the highest value, they do not stack). Same `skip_
    /// serializing_if` discipline as `maxDexCap` above, for the same
    /// reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spell_resistance_total: Option<i16>,
}

/// One carried item's real corpus weight and price, for the Gear tab's
/// per-item breakdown (`encumbrance::CarriedItem`). `costGp` is omitted on
/// the wire when the corpus genuinely carries no price for the record --
/// never serialized as a fabricated `0`. Same `skip_serializing_if`
/// discipline as `EquipmentEffectsDto::max_dex_cap`, for the same reason:
/// a literal `null` defeats the frontend's `!== undefined` checks.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarriedItemDto {
    pub item_id: String,
    pub weight_lbs: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_gp: Option<f64>,
}

/// The character's real carried weight against PF1's Strength-derived
/// carrying-capacity thresholds, plus the load tier's own penalties
/// (`encumbrance::EncumbranceComputation`).
///
/// Before this DTO, `compute_encumbrance` ran for real on every desktop
/// compute and every number it produced was discarded at the IPC boundary
/// -- the engine knew exactly what the character was carrying and the
/// player could not see any of it.
///
/// `level` is the `EncumbranceLevel` variant name (`"Light"`, `"Medium"`,
/// `"Heavy"`, `"OverHeavyCapacity"`), matching the `format!("{:?}", ...)`
/// convention `SchoolCoverageDto.school` already uses on this boundary.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncumbranceDto {
    pub total_carried_weight_lbs: f64,
    /// Total gp value of carried items that carry a real corpus price; a
    /// floor, not necessarily the full value (see `CarriedItemDto.costGp`).
    pub total_carried_cost_gp: f64,
    pub light_max_lbs: f64,
    pub medium_max_lbs: f64,
    pub heavy_max_lbs: f64,
    pub level: String,
    /// The max-Dex cap imposed by the *load tier alone*, absent under a
    /// light load. A consumer showing an effective cap must take the lower
    /// of this and `EquipmentEffectsDto.max_dex_cap` -- they do not sum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_max_dex_cap: Option<i16>,
    /// The armor check penalty imposed by the *load tier alone*; a real `0`
    /// under a light load. Does not sum with worn armor's own penalty --
    /// PF1 takes the more punishing of the two.
    pub load_armor_check_penalty: i16,
    pub per_item: Vec<CarriedItemDto>,
    /// Carried selections whose weight could not be resolved against the
    /// corpus -- so a `0.0` total reads as "genuinely weightless" rather
    /// than "we could not tell".
    pub unresolved_item_ids: Vec<String>,
}

/// One equipped item's own contribution to the defensive totals
/// (`equipment_effects::ResolvedEquipmentEffect`) -- the data behind an
/// "AC breakdown by source" view.
///
/// This was computed for real by `compute_equipment_effects` long before
/// it was exposed: `EquipmentEffects.per_item` has always been populated,
/// but `EquipmentEffectsDto` carried only the aggregates, so the
/// per-source detail stopped at the IPC boundary. Every `Option` here is a
/// genuine corpus absence (a longsword has no armor bonus), omitted on the
/// wire rather than zero-filled.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedEquipmentEffectDto {
    pub item_id: String,
    pub equipment_record_key: String,
    /// The `EquipmentCategory` variant name (`"ArmsArmor"`, `"General"`,
    /// `"MagicItems"`, `"Equipmods"`).
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub armor_class_bonus: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_dex: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spell_failure: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub armor_check_penalty: Option<i16>,
    /// This item's own armor-slot "Spell Resistance" special-ability
    /// contribution (`equipment_effects::ResolvedEquipmentEffect.spell_
    /// resistance_bonus`'s own doc comment). `None` for every item that
    /// carries no literal-integer `SR:` token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spell_resistance_bonus: Option<i16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CorpusDerivedDto {
    pub school_coverage: Vec<SchoolCoverageDto>,
    pub equipped_items: Vec<ResolvedEquipmentDto>,
    pub equipment_effects: EquipmentEffectsDto,
    pub encumbrance: EncumbranceDto,
    /// v0.6 alpha swarm (QA finding, 2026-07-24): every `spellId`/`itemId`
    /// the caller selected that did NOT resolve against this build's
    /// corpus -- e.g. a real, disk-persisted selection outside the
    /// desktop app's deliberately tiny bundled demo corpus
    /// (`corpus_fixtures.rs`, ~4 records total). Before this field, such a
    /// selection simply vanished from `schoolCoverage`/`equippedItems`
    /// with no signal at all, indistinguishable from "nothing selected"
    /// even though the underlying data was never lost. The frontend
    /// should render these as an honest "not shown -- outside demo
    /// corpus" indicator, not silence.
    pub unresolved_spell_ids: Vec<String>,
    pub unresolved_equipment_item_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticDto {
    pub id: String,
    pub message: String,
    pub claim_blocking: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCharacterRequest {
    pub character_id: String,
    pub display_label: String,
    pub race_id: String,
    pub class_id: String,
    pub level: u8,
    pub ability_scores: AbilityScoresDto,
    pub ability_bonus_target: String,
    pub saved_at: String,
    /// The ARG alternate racial traits the player chose for this race, as
    /// corpus record keys (`"Dwarf ~ Saltbeard"`) — exactly the keys
    /// `race_trait_picker`'s `listAlternateRacialTraits` /
    /// `resolveRaceAlternateSelection` commands emit and take, so the picker
    /// round-trips its own identifiers unchanged.
    ///
    /// SD-27: before this field, ARG's 153 alternate racial traits were
    /// browse-only — the picker resolved a swap live and correctly, and then
    /// had nowhere to send it, because `CreateCharacterRequest` had no field
    /// for a selection and `ChosenCharacterState` therefore never received
    /// one. `#[serde(default)]` so a caller that sends no selection (and every
    /// pre-existing saved payload) keeps working unchanged.
    ///
    /// Validated at creation against the real on-disk corpus by
    /// [`resolve_alternate_trait_choices`] — an unknown key, a key belonging to
    /// another race, or a pair that violates ARG's own `PREMULT` mutual-
    /// exclusion guard blocks the save rather than being silently dropped.
    #[serde(default)]
    pub selected_alternate_trait_keys: Vec<String>,
    /// A companion-bearing class's (Druid/Hunter/Cavalier) real
    /// character-creation-time choice of companion/mount species, as a
    /// `companion_base_stat_table.rs` slug (`"gulper_plant"`,
    /// `"allosaurus"`, ...). `#[serde(default)]` so an omitted field (every
    /// pre-existing saved payload, and every non-companion-bearing class)
    /// keeps working unchanged and falls back to that class's own prior
    /// fixed default species (Wolf for Druid/Hunter, Horse for Cavalier) --
    /// row 20 cycle 6's own named wiring gap: `ground_companion_stat_
    /// block`'s verified per-species table had no character-creation-time
    /// dispatch point until this field and `compose_character_input`'s
    /// own read of it. An unrecognized slug (a typo, or a species this
    /// engine has not yet hand-authored a verified base-stat row for)
    /// is never fabricated -- `ground_selected_companion_or_default`
    /// (`pilot_compute/mod.rs`) falls back to the same class default
    /// rather than guessing.
    #[serde(default)]
    pub companion_species: Option<String>,
    /// **AT-34-E4-002**: the character's chosen trait/drawback selections,
    /// as `trait_effects`' own wire ids (`"trait:trait_acrobat"`) --
    /// exactly what `list_available_character_traits`'s response carries
    /// as each option's `id`, so the picker round-trips its own
    /// identifiers unchanged (the same shape
    /// `selected_alternate_trait_keys` already established). Passed
    /// through verbatim into `ChosenCharacterState.selected_traits`, the
    /// same "trusted, unvalidated wire list" precedent `selected_feats`
    /// already follows for creation-time selections -- an id this crate
    /// does not recognize is simply inert everywhere it is read
    /// (`trait_effects::skill_bonuses_from_traits`'s own "omit rather than
    /// fabricate" discipline), never a blocked save. `#[serde(default)]`
    /// so an omitted field (every pre-existing saved payload, and every
    /// caller that sends none) keeps working unchanged.
    #[serde(default)]
    pub selected_traits: Vec<String>,
    /// **AT-34-E4-002 (second slice)**: the player's resolved choice for
    /// each *fixed-choice* `%LIST` trait named in `selected_traits`
    /// (`trait_effects::SKILL_CHOICE_TRAIT_BONUSES`) -- one
    /// `SelectedChoiceDto { choice_set_id, selection_id }` per such trait,
    /// with `choice_set_id` exactly `list_available_character_traits`'s
    /// own `choiceSetId` for that option and `selection_id` one of its
    /// `skillOptions`. Appended to `chosen.selected_choices` verbatim --
    /// the same generic `SelectedChoice` channel `LevelUpCharacterRequest
    /// ::additional_choices` already uses, not a new mechanism. A flat
    /// trait needs no entry here (`choice_set_id` is `None` for it).
    /// `#[serde(default)]` so every pre-existing caller (every flat-only
    /// trait selection, and every payload predating this field) keeps
    /// working unchanged. An entry whose `choice_set_id`/`selection_id`
    /// pair is not a real, corpus-declared option for that trait is
    /// simply inert (`skill_choice_bonuses_from_traits`'s own "omit
    /// rather than fabricate" discipline), never a blocked save.
    #[serde(default)]
    pub trait_skill_choices: Vec<SelectedChoiceDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterSummaryDto {
    pub character_id: String,
    pub display_label: String,
    pub game_system: String,
    pub schema_version: u16,
    pub saved_at: String,
    pub race_id: String,
    pub class_summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSavedCharactersResponse {
    pub characters: Vec<CharacterSummaryDto>,
    pub unreadable_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadSavedCharacterRequest {
    pub character_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadSavedCharacterResponse {
    pub summary: CharacterSummaryDto,
    pub snapshot: Option<PilotSnapshotDto>,
    pub diagnostics: Vec<DiagnosticDto>,
    pub corpus_derived: CorpusDerivedDto,
    /// The character's full persisted `chosen.selected_feats`, verbatim —
    /// not just feats added in the current session. Backlog item 8
    /// (`risks-and-open-questions.md`): the Feat picker previously had no
    /// way to render a loaded character's existing feat list.
    pub selected_feats: Vec<String>,
    /// The character's full persisted `chosen.spells_selected`, verbatim —
    /// not just spells added in the current session. Backlog item 9a
    /// (`risks-and-open-questions.md`): same shape of gap as item 8
    /// (`selected_feats`) — frontend had no way to detect a Wizard's
    /// current spell count/contents without this, which was the root of
    /// their "always route through record_and_prepare_spell_selection"
    /// workaround since they couldn't tell whether a spell add was truly
    /// "the first spell." Reuses `SpellSelectionImportDto` (already a
    /// general-purpose round-trip wire shape for export/import, not
    /// import-only despite the name) rather than inventing a near-duplicate
    /// type.
    pub spells_selected: Vec<SpellSelectionImportDto>,
    /// The resolved target(s) for every chooser feat the character holds.
    ///
    /// `selected_feats` alone cannot answer "Weapon Focus in *what*", and a
    /// repeatable feat taken twice appears there as two identical strings.
    /// Without this, the sheet renders both picks the same way — complete
    /// looking, and wrong.
    ///
    /// One entry per chooser feat, not per pick: nothing in the data model
    /// pairs pick N with target N, so the pairing is not invented here. A
    /// feat held with no target recorded is present with an empty `targets`,
    /// because "held but untargeted" is a state the sheet must show rather
    /// than hide.
    pub chosen_feat_targets: Vec<ChosenFeatTargetsDto>,
    /// Every `ComputationExplanation` the engine emitted for this build,
    /// verbatim — id, computed value, and the engine's own corpus-cited
    /// derivation text.
    ///
    /// Before this field the engine's 636+ distinct explanation records
    /// (`class_chassis.rogue.sneak_attack`, `class_chassis.cleric.
    /// channel_energy_dice`, `class_feature.fighter.bravery`, ...) were
    /// computed, tested and cited on every load and then dropped right
    /// here at the IPC boundary — the sheet had no field for them to
    /// travel in. The same defect had already been one-off-patched four
    /// times (Feats tab, Spells tab, AC-by-source, Pets tab); this is the
    /// structural channel those patches each worked around.
    ///
    /// **`detail` is carried verbatim and must be rendered verbatim.** It
    /// is the engine's corpus-cited derivation; paraphrasing or
    /// regenerating it in the frontend would create a second, unverified
    /// source of rules prose — exactly the hand-authored-rules-data debt
    /// `docs/governance/no-stub-mvp-doctrine.md` forbids.
    ///
    /// Populated on both the `Computed` and the `Blocked` path: a blocked
    /// build's explanations are still real records for the facets that
    /// did ground, and hiding them would flatten `Blocked` into "nothing
    /// computed."
    pub explanations: Vec<ExplanationDto>,
    /// One entry per `EquippedActive` item the engine identifies as a
    /// weapon (`damage_total::resolve_weapon_damage_breakdown`), carrying
    /// that weapon's corpus-cited damage facets.
    ///
    /// **No summed damage total, deliberately.** Each facet stays its own
    /// field, exactly as `contract.rs`'s `PilotReceipt::weapon_damage`
    /// boundary note requires: no summed "damage roll total" formula
    /// exists anywhere in this codebase, and the wield multiplier that
    /// would be needed to build one honestly is unknown. The sheet
    /// renders the breakdown as separate columns; it does not add them up.
    pub weapon_damage: Vec<WeaponDamageDto>,
    /// The ARG alternate racial traits this character actually holds, read back
    /// out of its persisted `chosen.selected_choices` (SD-27).
    ///
    /// Same shape of gap, and same fix, as `selected_feats` and
    /// `spells_selected`: without this the frontend has no way to know a loaded
    /// character's racial-trait choices, so the picker would reopen empty and
    /// the sheet would show a Dwarf with darkvision 90 and no reason why.
    pub selected_alternate_trait_keys: Vec<String>,
    /// **AT-34-E4-002**: the character's full persisted
    /// `chosen.selected_traits`, verbatim — not just traits added this
    /// session. Same shape of gap, and same fix, as `selected_feats`:
    /// without it the trait picker would reopen with no selections
    /// checked, and a mutation refresh that omitted it would silently
    /// clear a saved character's trait choices on the next round trip.
    pub selected_traits: Vec<String>,
    /// **This character's racial traits, resolved and rendered for *it*.**
    ///
    /// [`selected_alternate_trait_keys`](Self::selected_alternate_trait_keys)
    /// carries the choice; this carries what the choice *says*. Without it the
    /// sheet could name a trait and never state what it does — which is exactly
    /// what it did: one name-only card per chosen key, with the prose, the
    /// magnitudes, and the standard trait each one replaced all absent.
    ///
    /// Produced by [`resolve_racial_traits_for_character`], which calls
    /// `race_trait_picker::build_race_selection_for_feats` — the **same**
    /// renderer the Race Traits picker screen consumes, against the same
    /// corpus. `decisions.md §29.1`'s rule is one renderer with several
    /// consumers; this field is another consumer, not another renderer.
    ///
    /// `appliedTraits[].description`, `renderedTraitDescriptions[].text` and
    /// `displayValueFeats` are all rendered against **this character's own
    /// persisted `selected_feats`**, so a Halfling holding ARG's `Fortunate
    /// One` reads "4 times per day" where the book prints three. Render them
    /// verbatim: they are corpus prose with the engine's numbers resolved into
    /// it, not text to paraphrase.
    pub resolved_racial_traits: crate::race_trait_picker::RaceSelectionResponse,
}

/// Wire form of `pilot_compute::ComputationExplanation`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationDto {
    /// Stable engine id, e.g. `class_chassis.rogue.sneak_attack`.
    pub id: String,
    /// The computed magnitude this record explains.
    pub value: i16,
    /// The engine's own corpus-cited derivation text, verbatim.
    pub detail: String,
}

/// Projects the engine's explanation records onto the wire, verbatim.
pub(crate) fn map_explanations_dto(
    explanations: &[ComputationExplanation],
) -> Vec<ExplanationDto> {
    explanations
        .iter()
        .map(|explanation| ExplanationDto {
            id: explanation.id.clone(),
            value: explanation.value,
            detail: explanation.detail.clone(),
        })
        .collect()
}

/// Wire form of `damage_total::DiceExpression`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiceExpressionDto {
    pub count: u8,
    pub die_size: u8,
}

/// Wire form of `damage_total::DamageRollFeatEffect`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponFeatEffectDto {
    pub feat_key: String,
    pub damage_bonus: i16,
}

/// Wire form of `damage_total::WeaponDamageBreakdown`.
///
/// Every facet is `Option`: `None` means the engine found no corpus token
/// for it on this weapon, which is honest absence, not zero. The frontend
/// must render absence as absence — a `None` critical multiplier is not
/// "x0" and not "x2".
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponDamageDto {
    /// The `EquipmentSelection.item_id` this breakdown belongs to.
    pub weapon_item_id: String,
    /// The resolved corpus record key (e.g. `Longsword (Base)`), from
    /// whichever facet resolved it. `None` only if nothing resolved.
    pub weapon_record_key: Option<String>,
    pub base_dice: Option<DiceExpressionDto>,
    /// The Strength modifier's contribution to this weapon's damage.
    /// Bounded to `WeaponHandSlot::Primary` — `EquipmentSelection` has no
    /// hand-slot field, so a genuine two-weapon-fighting off-hand weapon
    /// is currently shown at its primary-hand fraction (see
    /// `resolve_weapon_damage_breakdown`'s own doc comment).
    pub str_damage_modifier: Option<i16>,
    /// `OneHanded` / `TwoHanded` / `Light`, verbatim from the corpus
    /// `WIELD:` token.
    pub wield_category: Option<String>,
    pub enhancement_attack_bonus: Option<i16>,
    pub enhancement_damage_bonus: Option<i16>,
    /// Inclusive `[low, high]` natural-roll threat bounds, e.g. `[19, 20]`
    /// for a longsword.
    pub critical_threat_range: Option<[u8; 2]>,
    pub critical_multiplier: Option<u8>,
    /// Constant damage bonuses from the character's feats. Gathered once
    /// per character, not per weapon — this bounded slice does not model
    /// per-weapon feat targeting.
    pub feat_effects: Vec<WeaponFeatEffectDto>,
}

/// Projects the engine's per-weapon damage breakdown onto the wire.
///
/// Sums nothing: each facet crosses as its own field. See
/// `LoadSavedCharacterResponse::weapon_damage`'s doc comment.
pub(crate) fn map_weapon_damage_dto(
    breakdowns: &[WeaponDamageBreakdown],
) -> Vec<WeaponDamageDto> {
    breakdowns
        .iter()
        .map(|breakdown| WeaponDamageDto {
            weapon_item_id: breakdown.weapon_item_id.clone(),
            weapon_record_key: breakdown
                .base_dice
                .as_ref()
                .map(|dice| dice.weapon_record_key.clone())
                .or_else(|| {
                    breakdown
                        .str_modifier
                        .as_ref()
                        .map(|str_mod| str_mod.weapon_record_key.clone())
                }),
            base_dice: breakdown.base_dice.as_ref().map(|dice| DiceExpressionDto {
                count: dice.base_dice.count,
                die_size: dice.base_dice.die_size,
            }),
            str_damage_modifier: breakdown
                .str_modifier
                .as_ref()
                .map(|str_mod| str_mod.str_damage_modifier),
            wield_category: breakdown
                .str_modifier
                .as_ref()
                .map(|str_mod| format!("{:?}", str_mod.wield_category)),
            enhancement_attack_bonus: breakdown
                .weapon_enhancement
                .as_ref()
                .map(|enhancement| enhancement.attack_bonus),
            enhancement_damage_bonus: breakdown
                .weapon_enhancement
                .as_ref()
                .map(|enhancement| enhancement.damage_bonus),
            critical_threat_range: breakdown
                .critical_threat_range
                .as_ref()
                .map(|range| [range.critical_threat_range.0, range.critical_threat_range.1]),
            critical_multiplier: breakdown
                .critical_multiplier
                .as_ref()
                .map(|multiplier| multiplier.critical_multiplier),
            feat_effects: breakdown
                .feat_effects
                .iter()
                .map(|effect| WeaponFeatEffectDto {
                    feat_key: effect.feat_key.clone(),
                    damage_bonus: effect.damage_bonus,
                })
                .collect(),
        })
        .collect()
}

/// Wire form of `feat_effects::ChosenFeatTargets`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChosenFeatTargetsDto {
    pub feat_id: String,
    /// `"Weapon"`, `"Skill"` or `"SpellSchool"` — the same vocabulary
    /// `FeatCatalogEntryDto::chooser_target_kind` uses, so a caller can match
    /// a held feat against its catalog entry without a second mapping.
    pub target_kind: String,
    pub targets: Vec<String>,
}

/// Projects the engine's resolved chooser targets onto the wire.
///
/// `pub(crate)` so `pf1_adapter`'s twin construction site calls this rather
/// than reimplementing it — the same sharing the `map_spells_selected_dto`
/// precedent established for exactly this pair of copy-paste call sites.
pub(crate) fn map_chosen_feat_targets_dto(
    character_input: &codex::rules_core::character_input::CharacterInput,
) -> Vec<ChosenFeatTargetsDto> {
    codex::rules_core::feat_effects::chosen_feat_targets(
        &character_input.chosen.selected_feats,
        &character_input.chosen.selected_choices,
    )
    .into_iter()
    .map(|resolved| ChosenFeatTargetsDto {
        feat_id: resolved.feat_id,
        target_kind: format!("{:?}", resolved.target_kind),
        targets: resolved.targets,
    })
    .collect()
}

/// The `kind` tag stays PascalCase (`Saved` / `Blocked`) — no container-level
/// `rename_all` — matching the `BaselineArmorClass` precedent so the TS
/// boundary can match on those exact strings. v0.6 alpha swarm (real
/// render-staleness root cause, frontend-found): a bare
/// `#[serde(rename_all = "camelCase")]` on this enum would ALSO camelCase
/// (lowercase-first) the `"Saved"`/`"Blocked"` tag values themselves,
/// breaking every `outcome.kind === 'Saved'` check across the frontend —
/// that's exactly why it was never added despite `corpus_derived` staying
/// snake_case on the wire ever since. The real fix is a per-field rename
/// on just the one field that actually has an underscore (every other
/// field here already happens to have none), not an enum-wide attribute.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum CreateCharacterResponse {
    Saved {
        // Boxed solely to close the `clippy::large_enum_variant` gap against the
        // `Blocked` variant (SD-21 Epic 4 closure, criterion E4.24) — `Box<T>`
        // serializes identically to `T` via serde, so the wire shape to the TS
        // boundary is unchanged.
        summary: Box<CharacterSummaryDto>,
        snapshot: PilotSnapshotDto,
        #[serde(rename = "corpusDerived")]
        corpus_derived: CorpusDerivedDto,
    },
    Blocked {
        diagnostics: Vec<DiagnosticDto>,
    },
}

// ----- Pure functions (unit-testable, no AppHandle / filesystem) -----
//
// `compose_character_input` / `apply_level_up` / `mutate_saved_character_at_root`
// / `level_up_character_at_root` / `apply_add_equipment_selection` /
// `add_equipment_selection_at_root` / `apply_add_spell_selection` /
// `add_spell_selection_at_root` moved to `pf1_adapter.rs` (SD-25 Epic 3
// "Hub of Hubs" refactor, Criterion 3.2 — the `Pf1Adapter` extraction).
// Re-exported below (where an external file's existing import path needs
// it) and brought into this module's own scope via `use` (for this
// module's `#[tauri::command]` wrappers and its own `mod tests`, which
// still call them unqualified via `use super::*`).
pub(crate) use crate::pf1_adapter::{
    add_equipment_selection_at_root, add_feat_selection_at_root, add_spell_selection_at_root,
    apply_attach_equipment_modifier, compose_character_input, level_up_character_at_root,
    mutate_saved_character_at_root, record_and_prepare_spell_selection_at_root,
    resolve_unified_pilot_snapshot, set_skill_allocations_at_root,
};
// `apply_level_up` / `apply_add_equipment_selection` / `apply_add_spell_selection`
// / `apply_add_feat_selection` / `apply_set_skill_allocations` are only
// referenced directly by this module's own `#[cfg(test)] mod tests` (the
// non-test `#[tauri::command]` wrappers only ever call the `_at_root`
// variants re-exported above) — `#[cfg(test)]` on the import itself avoids
// an `unused_imports` warning on non-test builds while keeping
// `use super::*` resolving them inside `mod tests` unchanged.
#[cfg(test)]
pub(crate) use crate::pf1_adapter::{
    apply_add_equipment_selection, apply_add_feat_selection, apply_add_feat_selection_with_target,
    apply_add_spell_selection, apply_level_up, apply_record_and_prepare_spell_selection,
    apply_set_skill_allocations, resolve_feat_target_choice,
};

/// Join the OS app-data directory with the characters-root subdirectory.
/// Split out as a pure function so it's unit-testable without a real
/// `AppHandle`.
fn characters_root_from_app_data_dir(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join(CHARACTERS_ROOT_DIR_NAME)
}

fn map_ability_modifiers_dto(
    modifiers: codex::rules_core::pilot_compute::AbilityModifiers,
) -> AbilityModifiersDto {
    AbilityModifiersDto {
        strength: modifiers.strength,
        dexterity: modifiers.dexterity,
        constitution: modifiers.constitution,
        intelligence: modifiers.intelligence,
        wisdom: modifiers.wisdom,
        charisma: modifiers.charisma,
    }
}

fn map_base_saves_dto(saves: codex::rules_core::pilot_compute::BaseSaves) -> BaseSavesDto {
    BaseSavesDto {
        fortitude: saves.fortitude,
        reflex: saves.reflex,
        will: saves.will,
    }
}

fn map_selected_skill_modifiers_dto(
    modifiers: codex::rules_core::pilot_compute::SelectedSkillModifiers,
) -> SelectedSkillModifiersDto {
    SelectedSkillModifiersDto {
        climb: modifiers.climb,
        intimidate: modifiers.intimidate,
        swim: modifiers.swim,
    }
}

// `pub(crate)` (rather than private): `pf1_adapter.rs`'s
// `mutate_saved_character_at_root` (moved there SD-25 Criterion 3.2) builds
// the same `Saved` response shape this module's own commands do, and reuses
// this mapping rather than re-deriving it.
pub(crate) fn map_snapshot_dto(snapshot: &PilotSnapshot) -> PilotSnapshotDto {
    PilotSnapshotDto {
        ability_modifiers: map_ability_modifiers_dto(snapshot.ability_modifiers),
        base_attack_bonus: snapshot.base_attack_bonus,
        base_saves: map_base_saves_dto(snapshot.base_saves),
        baseline_melee_attack_bonus: snapshot.combat.baseline_melee_attack_bonus,
        baseline_armor_class: snapshot.defense.baseline_armor_class,
        total_saves: map_base_saves_dto(snapshot.defense.total_save),
        selected_skill_modifiers: map_selected_skill_modifiers_dto(
            snapshot.skill.selected_modifier,
        ),
        damage_reduction: snapshot.defense.damage_reduction,
        companion: snapshot.companion.as_ref().map(map_animal_companion_dto),
        spellbook: snapshot.spellbook.as_ref().map(map_pilot_spellbook_dto),
    }
}

fn map_pilot_spellbook_dto(spellbook: &PilotSpellbookViewModel) -> PilotSpellbookDto {
    PilotSpellbookDto {
        spell_save_dc: spellbook
            .spell_save_dc
            .iter()
            .map(|entry| SpellSaveDcDto { class_id: entry.class_id.clone(), dc: entry.dc })
            .collect(),
    }
}

fn map_animal_companion_dto(
    companion: &codex::rules_core::pilot_view_model::PilotCompanionViewModel,
) -> AnimalCompanionDto {
    AnimalCompanionDto {
        owner_class_label: companion.owner_class_label.clone(),
        role_label: companion.role_label.clone(),
        species: companion.species.clone(),
        summary_detail: companion.summary_detail.clone(),
        stats: companion
            .stats
            .iter()
            .map(|stat| CompanionStatDto {
                label: stat.label.clone(),
                value: stat.value,
                detail: stat.detail.clone(),
            })
            .collect(),
        notes: companion.notes.clone(),
        advancement_note: companion.advancement_note.clone(),
    }
}

/// Maps one `ResolvedEquipment` to its DTO, recursing into
/// `applied_modifiers` (v0.6 alpha swarm sub-task 6) -- a resolved
/// modifier is structurally identical to a resolved top-level selection,
/// so this one function handles both without a near-duplicate.
/// `pub(crate)` — same reason as `map_spells_selected_dto`:
/// `rule_system_adapter.rs`'s `TestPf1Delegate` test double reuses this
/// rather than hand-rolling its own mirror a second time.
pub(crate) fn map_resolved_equipment_dto(item: &ResolvedEquipment) -> ResolvedEquipmentDto {
    ResolvedEquipmentDto {
        item_id: item.item_id.clone(),
        equipment_record_name: item.equipment_record_name.clone(),
        equipment_record_key: item.equipment_record_key.clone(),
        grounded: item.table_cell.is_some(),
        applied_modifiers: item.applied_modifiers.iter().map(map_resolved_equipment_dto).collect(),
    }
}

// `pub(crate)` — same reason as `map_snapshot_dto` above.
pub(crate) fn map_corpus_derived_dto(section: &CorpusDerivedSection) -> CorpusDerivedDto {
    CorpusDerivedDto {
        school_coverage: section
            .school_coverage
            .values()
            .map(|coverage| SchoolCoverageDto {
                school: format!("{:?}", coverage.school),
                spells: coverage.spells.clone(),
                grounded: coverage.table_cell.is_some(),
            })
            .collect(),
        equipped_items: section
            .equipped_items
            .iter()
            .map(map_resolved_equipment_dto)
            .collect(),
        equipment_effects: EquipmentEffectsDto {
            per_item: section
                .equipment_effects
                .per_item
                .iter()
                .map(|effect| ResolvedEquipmentEffectDto {
                    item_id: effect.item_id.clone(),
                    equipment_record_key: effect.equipment_record_key.clone(),
                    category: format!("{:?}", effect.category),
                    armor_class_bonus: effect.armor_class_bonus,
                    max_dex: effect.max_dex,
                    spell_failure: effect.spell_failure,
                    armor_check_penalty: effect.armor_check_penalty,
                    spell_resistance_bonus: effect.spell_resistance_bonus,
                })
                .collect(),
            armor_class_delta: section.equipment_effects.armor_class_delta,
            armor_check_penalty_total: section.equipment_effects.armor_check_penalty_total,
            max_dex_cap: section.equipment_effects.max_dex_cap,
            spell_failure_chance: section.equipment_effects.spell_failure_chance,
            attack_bonus_delta: section.equipment_effects.attack_bonus_delta,
            spell_resistance_total: section.equipment_effects.spell_resistance_total,
        },
        encumbrance: map_encumbrance_dto(&section.encumbrance),
        unresolved_spell_ids: section.unresolved_spell_ids.clone(),
        unresolved_equipment_item_ids: section.unresolved_equipment_item_ids.clone(),
    }
}

/// Flattens `EncumbranceComputation`'s nested `thresholds` into the flat
/// wire shape the frontend consumes, and stringifies the load tier the same
/// way `SchoolCoverageDto.school` already stringifies its enum.
pub(crate) fn map_encumbrance_dto(
    encumbrance: &codex::rules_core::encumbrance::EncumbranceComputation,
) -> EncumbranceDto {
    EncumbranceDto {
        total_carried_weight_lbs: encumbrance.total_carried_weight_lbs,
        total_carried_cost_gp: encumbrance.total_carried_cost_gp,
        light_max_lbs: encumbrance.thresholds.light_max_lbs,
        medium_max_lbs: encumbrance.thresholds.medium_max_lbs,
        heavy_max_lbs: encumbrance.thresholds.heavy_max_lbs,
        level: format!("{:?}", encumbrance.level),
        load_max_dex_cap: encumbrance.load_max_dex_cap,
        load_armor_check_penalty: encumbrance.load_armor_check_penalty,
        per_item: encumbrance
            .per_item
            .iter()
            .map(|item| CarriedItemDto {
                item_id: item.item_id.clone(),
                weight_lbs: item.weight_lbs,
                cost_gp: item.cost_gp,
            })
            .collect(),
        unresolved_item_ids: encumbrance.unresolved_item_ids.clone(),
    }
}

// `pub(crate)` — same reason as `map_snapshot_dto` above.
pub(crate) fn map_diagnostics_dto(
    diagnostics: &[codex::rules_core::pilot_compute::ComputationDiagnostic],
) -> Vec<DiagnosticDto> {
    diagnostics
        .iter()
        .map(|diagnostic| DiagnosticDto {
            id: diagnostic.id.clone(),
            message: diagnostic.message.clone(),
            claim_blocking: diagnostic.claim_blocking,
        })
        .collect()
}

// `pub(crate)` — same reason as `map_snapshot_dto` above. Reuses
// `SpellSelectionImportDto` (already a general-purpose export/import
// round-trip wire shape, not import-only despite the name) rather than
// inventing a near-duplicate type for `LoadSavedCharacterResponse`.
pub(crate) fn map_spells_selected_dto(spells: &[SpellSelection]) -> Vec<SpellSelectionImportDto> {
    spells
        .iter()
        .map(|spell| SpellSelectionImportDto {
            spell_id: spell.spell_id.clone(),
            source_class_id: spell.source_class_id.clone(),
            acquisition_mode: spell.acquisition_mode.into(),
        })
        .collect()
}

// `pub(crate)` — `pf1_adapter.rs`'s `impl RuleSystemAdapter for Pf1Adapter`
// (criterion 3.1's trait, landed after this cycle's own dispatch began;
// wired here rather than left as a `## DISCOVERED` follow-up) reuses this
// mapping for `list_saved_characters` rather than re-deriving it.
pub(crate) fn map_summary_dto(summary: &SavedCharacterSummary) -> CharacterSummaryDto {
    CharacterSummaryDto {
        character_id: summary.character_id.clone(),
        display_label: summary.display_label.clone(),
        game_system: summary.game_system.clone(),
        schema_version: summary.schema_version,
        saved_at: summary.saved_at.clone(),
        race_id: summary.race_id.clone(),
        class_summary: summary.class_summary.clone(),
    }
}

// `pub(crate)` — same reason as `map_snapshot_dto` above.
pub(crate) fn summarize_envelope(envelope: &SavedCharacterEnvelope) -> CharacterSummaryDto {
    let class_summary = envelope
        .character_input
        .chosen
        .class_levels
        .iter()
        .map(|class_level| format!("{}:{}", class_level.class_id, class_level.level))
        .collect::<Vec<_>>()
        .join(",");

    CharacterSummaryDto {
        character_id: envelope.character_id.clone(),
        display_label: envelope.display_label.clone(),
        game_system: envelope.game_system.clone(),
        schema_version: envelope.schema_version,
        saved_at: envelope.saved_at.clone(),
        race_id: envelope.character_input.chosen.race_id.clone(),
        class_summary,
    }
}

// ----- `mutate_saved_character` operation table -----

/// The character-mutation surface this module documents. Every operation
/// shares the same `load -> mutate -> recompute -> re-save -> return
/// envelope` semantics: load the saved envelope via
/// `SavedCharacterStore::load`, apply one bounded mutation to its
/// `CharacterInput`, recompute via `compute_pilot_with_corpus`, re-save via
/// `SavedCharacterStore::save`, and return the updated envelope (as a
/// `CreateCharacterResponse::Saved`/`Blocked`, matching every other
/// character-hub command's response shape).
///
/// This table documents the full six-operation surface. As of this cycle
/// all six rows are wired to callable `#[tauri::command]`s
/// (`level_up_character`, `add_equipment_selection`, `add_spell_selection`,
/// `set_skill_allocations`, `add_feat_selection`,
/// `record_and_prepare_spell_selection`).
/// Per the Wired Integration doctrine (`docs/governance/no-stub-mvp-doctrine.md`),
/// the `wired` flag below is descriptive metadata this table's own
/// dispatch-shape test asserts against, not a runtime dispatcher a caller
/// can reach.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SavedCharacterMutationOp {
    LevelUpCharacter,
    AddEquipmentSelection,
    AddSpellSelection,
    SetSkillAllocations,
    AddFeatSelection,
    RecordAndPrepareSpellSelection,
}

/// One row of the `mutate_saved_character` operation table.
#[derive(Debug, Clone, Copy)]
pub struct SavedCharacterMutationOpDescriptor {
    pub op: SavedCharacterMutationOp,
    /// The operation name, matching its `#[tauri::command]` function name
    /// once wired.
    pub name: &'static str,
    /// The mutation this operation applies to the loaded `CharacterInput`,
    /// on top of the load -> mutate -> recompute -> re-save ->
    /// return-envelope semantics every row shares.
    pub description: &'static str,
    /// Whether this operation is reachable through a real, callable Tauri
    /// command in this build (registered in `main.rs`'s
    /// `invoke_handler!`). `false` means the row is documented here but
    /// intentionally not yet registered.
    pub wired: bool,
}

pub const SAVED_CHARACTER_MUTATION_OPERATIONS: [SavedCharacterMutationOpDescriptor; 6] = [
    SavedCharacterMutationOpDescriptor {
        op: SavedCharacterMutationOp::LevelUpCharacter,
        name: "level_up_character",
        description: "Increments the requested class's level (or adds it \
            at level 1 if the character has none yet), then recomputes and \
            re-saves.",
        wired: true,
    },
    SavedCharacterMutationOpDescriptor {
        op: SavedCharacterMutationOp::AddEquipmentSelection,
        name: "add_equipment_selection",
        description: "Appends an entry to chosen.equipment_selections, then \
            recomputes and re-saves.",
        wired: true,
    },
    SavedCharacterMutationOpDescriptor {
        op: SavedCharacterMutationOp::AddSpellSelection,
        name: "add_spell_selection",
        description: "Appends an entry to chosen.spells_selected, then \
            recomputes and re-saves.",
        wired: true,
    },
    SavedCharacterMutationOpDescriptor {
        op: SavedCharacterMutationOp::SetSkillAllocations,
        name: "set_skill_allocations",
        description: "Replaces chosen.skill_allocations wholesale with the \
            caller's full allocation set, then recomputes and re-saves.",
        wired: true,
    },
    SavedCharacterMutationOpDescriptor {
        op: SavedCharacterMutationOp::AddFeatSelection,
        name: "add_feat_selection",
        description: "Appends an entry to chosen.selected_feats, then \
            recomputes and re-saves. SD-27: refuses first when the \
            character does not meet the feat's real corpus prerequisites, \
            naming the unmet ones -- see \
            add_feat_selection_enforcing_prerequisites_at_root.",
        wired: true,
    },
    SavedCharacterMutationOpDescriptor {
        op: SavedCharacterMutationOp::RecordAndPrepareSpellSelection,
        name: "record_and_prepare_spell_selection",
        description: "Appends BOTH a Known and a Prepared entry for the \
            same spell to chosen.spells_selected in one atomic mutation, \
            then recomputes and re-saves -- breaks the Wizard spellbook \
            bootstrap deadlock a single-mode add_spell_selection call \
            cannot cross alone.",
        wired: true,
    },
];

// ----- Tauri commands -----

/// `create_character`'s real implementation, split from the `#[tauri::command]`
/// wrapper below so it is unit-testable against a real `SavedCharacterStore`
/// fixture without an `AppHandle` -- mirrors every other command's own
/// `_at_root` split (`level_up_character_at_root`, `purchase_equipment_at_root`,
/// `recompute_character_at_root`, ...). `app_version` is passed explicitly
/// (rather than an `AppHandle`) since it is the only piece of this function's
/// original body that ever needed one.
///
/// v0.6 alpha swarm item 7 (risks-and-open-questions.md): once the build
/// reaches `Computed` and saves, this also initializes the character's
/// starting money balance via `money::starting_wealth_gp`, for any class
/// that function recognizes -- today that means every character that gets
/// this far at all, since `starting_wealth_gp` covers all 11 CRB classes
/// and only Fighter/Wizard/Rogue currently reach `Computed` in the first
/// place (a class outside that set never reaches this line, having already
/// returned `Blocked` above). An unrecognized class id (`None`) leaves
/// `money.json` uninitialized -- the existing "no file yet" convention
/// already means a 0 balance, so this never fabricates a value for a class
/// this table doesn't cover.
/// Turns a creation request's chosen alternate racial traits into the
/// `SelectedChoice` entries the engine reads, or into blocking diagnostics.
///
/// **Validation is the real picker's, not a second opinion.** Everything here
/// comes from one `race_trait_picker::build_race_selection` call — the same
/// command the picker screen drives, which itself calls `RaceCorpus::resolve`,
/// the single implementation of `decisions.md §26`'s protocol. This function
/// decides nothing about which traits are legal together; it only refuses to
/// persist a selection the resolver already reported a problem with.
///
/// Four ways a selection is refused, each with the resolver's own finding
/// carried verbatim rather than paraphrased:
///
/// * the race is not in the loaded corpus (`errors`),
/// * a key matches no alternate for that race — a typo, or a trait belonging to
///   a different race (`unmatchedSelections`),
/// * two chosen alternates violate each other's ARG `PREMULT` self-exclusion
///   guard (`conflictingSelections`),
/// * a chosen alternate's replace-flag suppresses and grants nothing in the
///   loaded books (`inertFlags`) — the standard trait it claims to replace
///   would silently stay, so the character would quietly get both. That is the
///   9-Aasimar-alternate upstream gap `race_trait_picker` documents; refusing
///   the save is the honest answer until the `globalvar` files are ingested.
///
/// Refusing rather than dropping matters: a silently-ignored selection is
/// exactly the failure `unresolved_spell_ids` / `unresolved_equipment_item_ids`
/// were both added to stop.
fn resolve_alternate_trait_choices(
    race_id: &str,
    selected_alternate_trait_keys: &[String],
) -> Result<Vec<SelectedChoice>, Vec<DiagnosticDto>> {
    if selected_alternate_trait_keys.is_empty() {
        return Ok(Vec::new());
    }

    let resolution = crate::race_trait_picker::build_race_selection(&crate::race_trait_picker::RaceSelectionRequest {
        race_key: race_id.to_owned(),
        selected_alternate_keys: selected_alternate_trait_keys.to_vec(),
    });

    let mut diagnostics: Vec<DiagnosticDto> = Vec::new();
    for error in &resolution.errors {
        diagnostics.push(DiagnosticDto {
            id: "race.alternate_trait.unresolved_race".to_owned(),
            message: error.clone(),
            claim_blocking: true,
        });
    }
    for key in &resolution.unmatched_selections {
        diagnostics.push(DiagnosticDto {
            id: "race.alternate_trait.unmatched_selection".to_owned(),
            message: format!(
                "alternate racial trait {key:?} matches no ARG alternate for race {race_id:?}; \
                 nothing was replaced, so the character was not saved"
            ),
            claim_blocking: true,
        });
    }
    for conflict in &resolution.conflicting_selections {
        diagnostics.push(DiagnosticDto {
            id: "race.alternate_trait.mutually_exclusive".to_owned(),
            message: format!(
                "{} and {} cannot both be taken: ARG's own PREMULT self-exclusion guard on {} \
                 names {}, which {} sets (arg_abilities_race.lst)",
                conflict.name, conflict.blocked_by_name, conflict.name, conflict.flag,
                conflict.blocked_by_name
            ),
            claim_blocking: true,
        });
    }
    for flag in &resolution.inert_flags {
        diagnostics.push(DiagnosticDto {
            id: "race.alternate_trait.inert_flag".to_owned(),
            message: format!(
                "the chosen alternate racial trait fires replace-flag {flag}, which suppresses \
                 and grants nothing in the loaded books, so the standard trait it claims to \
                 replace would still apply and the character would hold both; refused rather \
                 than saved with a swap that did not happen"
            ),
            claim_blocking: true,
        });
    }

    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }

    Ok(selected_alternate_trait_keys
        .iter()
        .map(|key| SelectedChoice {
            choice_set_id: RACE_ALTERNATE_TRAIT_CHOICE_ID.to_owned(),
            selection_id: race_alternate_trait_selection_id(key),
        })
        .collect())
}

/// Every alternate racial trait key a saved character holds, read back out of
/// its persisted `selected_choices` in selection order.
///
/// The read half of the round-trip: `load_saved_character` needs it so the
/// sheet and the picker can show a loaded character's real choices instead of
/// re-deriving them (the gap `selected_feats` and `spells_selected` were each
/// added to close).
/// One saved character's racial traits, resolved against its own choices and
/// **rendered against its own feats**.
///
/// The sibling of [`resolve_alternate_trait_choices`], and deliberately a
/// different call. That one validates whether a *selection* is legal, which is
/// a race-and-selection question a feat cannot change, so it passes no feats
/// and must keep passing none — `build_race_selection`'s own doc records that
/// contract. This one answers a different question: what does this character's
/// sheet *say*. A trait's stated magnitude does move with the feats it holds
/// (`Halfling ~ Adaptable Luck` reads "three times per day" in the book and
/// "4 times per day" for a halfling with ARG's `Fortunate One`), so this call
/// hands over `chosen.selected_feats` verbatim.
///
/// **No rendering happens here.** `race_trait_picker::render_trait_description`
/// is the single renderer (`decisions.md §29.1`); this function only routes a
/// saved character's real race id, real selections and real feats into it. The
/// race id crosses as-is — `RaceCorpus::resolve_key` is prefix- and
/// case-tolerant, so `race:half-elf` reaches the same record as `Half-Elf`
/// without this module inventing a mapping.
///
/// An unknown race yields a response carrying `errors`, never a silently empty
/// trait list: a sheet showing no racial traits at all must be able to say why.
pub(crate) fn resolve_racial_traits_for_character(
    input: &CharacterInput,
) -> crate::race_trait_picker::RaceSelectionResponse {
    crate::race_trait_picker::build_race_selection_for_feats(
        &crate::race_trait_picker::RaceSelectionRequest {
            race_key: input.chosen.race_id.clone(),
            selected_alternate_keys: read_alternate_trait_keys(input),
        },
        &input.chosen.selected_feats,
    )
}

pub(crate) fn read_alternate_trait_keys(input: &CharacterInput) -> Vec<String> {
    input
        .chosen
        .selected_choices
        .iter()
        .filter(|choice| choice.choice_set_id == RACE_ALTERNATE_TRAIT_CHOICE_ID)
        .map(|choice| {
            choice
                .selection_id
                .strip_prefix(RACE_ALTERNATE_TRAIT_SELECTION_PREFIX)
                .unwrap_or(choice.selection_id.as_str())
                .to_owned()
        })
        .collect()
}

pub(crate) fn create_character_at_root(
    root: &Path,
    request: &CreateCharacterRequest,
    app_version: String,
) -> Result<CreateCharacterResponse, String> {
    let mut character_input = compose_character_input(request);
    match resolve_alternate_trait_choices(&request.race_id, &request.selected_alternate_trait_keys) {
        Ok(choices) => character_input.chosen.selected_choices.extend(choices),
        Err(diagnostics) => return Ok(CreateCharacterResponse::Blocked { diagnostics }),
    }
    let character_input = character_input;

    let (snapshot, corpus_receipt) =
        match resolve_unified_pilot_snapshot(&character_input, corpus_fixture_bundle()) {
            Ok(result) => result,
            Err(diagnostics) => {
                return Ok(CreateCharacterResponse::Blocked {
                    diagnostics: map_diagnostics_dto(&diagnostics),
                });
            }
        };

    let envelope = SavedCharacterEnvelope {
        character_id: request.character_id.clone(),
        revision_id: format!("{}.rev.1", request.character_id),
        revision_kind: SavedCharacterRevisionKind::Authoritative,
        saved_at: request.saved_at.clone(),
        schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
        app_or_runtime_version: app_version,
        content_or_rules_provenance: SOURCE_PACKAGE_ID.to_owned(),
        game_system: GAME_SYSTEM_ID.to_owned(),
        latest_authoritative_revision_ref: format!("{}.rev.1", request.character_id),
        display_label: request.display_label.clone(),
        character_input,
    };

    SavedCharacterStore::save(&envelope, root).map_err(|err| err.message)?;

    if let Some(starting_wealth_gp) = money::starting_wealth_gp(&request.class_id) {
        let starting_copper = money::gp_to_copper(f64::from(starting_wealth_gp));
        adjust_character_money_at_root(root, starting_copper as i64)?;
    }

    Ok(CreateCharacterResponse::Saved {
        summary: Box::new(summarize_envelope(&envelope)),
        snapshot: map_snapshot_dto(&snapshot),
        corpus_derived: map_corpus_derived_dto(&corpus_receipt.corpus_derived),
    })
}

#[tauri::command]
pub fn create_character(
    app: tauri::AppHandle,
    request: CreateCharacterRequest,
) -> Result<CreateCharacterResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    create_character_at_root(&root, &request, app.package_info().version.to_string())
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloneCharacterRequest {
    pub character_id: String,
    pub new_character_id: String,
    pub new_display_label: String,
    pub saved_at: String,
}

/// Duplicates a saved character's full `CharacterInput` (race, classes,
/// ability scores, feats, skills, equipment, spells) under a new id and
/// display label, recomputing and saving it exactly like `create_character`.
/// Never persists an unproven build — a source that no longer computes
/// (e.g. after an engine change) returns `Blocked` rather than saving a copy.
#[tauri::command]
pub fn clone_character(
    app: tauri::AppHandle,
    request: CloneCharacterRequest,
) -> Result<CreateCharacterResponse, String> {
    let source_root = resolve_character_root(&app, &request.character_id)?;
    let source_envelope = SavedCharacterStore::load(&source_root).map_err(|err| err.message)?;

    let mut character_input = source_envelope.character_input.clone();
    character_input.case_id = Some(request.new_character_id.clone());

    let (snapshot, corpus_receipt) =
        match resolve_unified_pilot_snapshot(&character_input, corpus_fixture_bundle()) {
            Ok(result) => result,
            Err(diagnostics) => {
                return Ok(CreateCharacterResponse::Blocked {
                    diagnostics: map_diagnostics_dto(&diagnostics),
                });
            }
        };

    let envelope = SavedCharacterEnvelope {
        character_id: request.new_character_id.clone(),
        revision_id: format!("{}.rev.1", request.new_character_id),
        revision_kind: SavedCharacterRevisionKind::Authoritative,
        saved_at: request.saved_at.clone(),
        schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
        app_or_runtime_version: app.package_info().version.to_string(),
        content_or_rules_provenance: SOURCE_PACKAGE_ID.to_owned(),
        game_system: GAME_SYSTEM_ID.to_owned(),
        latest_authoritative_revision_ref: format!("{}.rev.1", request.new_character_id),
        display_label: request.new_display_label.clone(),
        character_input,
    };

    let new_root = resolve_character_root(&app, &request.new_character_id)?;
    SavedCharacterStore::save(&envelope, &new_root).map_err(|err| err.message)?;

    // Best-effort: carry the portrait over too, if the source has one.
    let source_portrait = source_root.join(PORTRAIT_FILE_NAME);
    if source_portrait.exists() {
        let _ = std::fs::copy(&source_portrait, new_root.join(PORTRAIT_FILE_NAME));
    }

    Ok(CreateCharacterResponse::Saved {
        summary: Box::new(summarize_envelope(&envelope)),
        snapshot: map_snapshot_dto(&snapshot),
        corpus_derived: map_corpus_derived_dto(&corpus_receipt.corpus_derived),
    })
}

const DEFAULT_CHARACTER_SEED_MARKER: &str = ".default_character_seeded";
const DEFAULT_CHARACTER_ID: &str = "00000000-0000-0000-0000-000000000001";
const DEFAULT_CHARACTER_SAVED_AT: &str = "2026-01-01T00:00:00.000Z";

/// Seeds a starter character ("Aldric Ironhand": Human Fighter 3) into a
/// fresh install so there's something to open immediately instead of an
/// empty character list.
///
/// Aldric is a single-class Fighter, not the Fighter 3 / Wizard 1 multiclass
/// build shown in the browser-preview sample data (`previewData.ts`) — the
/// real compute engine only reaches `Computed` for a single-class Fighter
/// today (`compute_fighter_chassis` in `src/rules_core/pilot_compute.rs`
/// gates base attack bonus / base saves on that alone; verified directly,
/// not assumed — a single-class Wizard build was tried and still comes back
/// `Blocked`). Ability scores are chosen to reproduce the same ability
/// modifiers as the preview's Aldric (+3/+1/+2/+2/+1/-1).
///
/// Gated on a marker file, not on whether the characters directory is
/// currently empty — so deleting the starter character does not bring it
/// back on next launch. Reuses `compose_character_input`/`create_character`'s
/// own invariant: only saves if the build actually computes, never writes an
/// unproven build.
pub fn seed_default_character_if_needed(app: &tauri::AppHandle) -> Result<(), String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|err| format!("could not resolve app data directory: {err}"))?;
    let marker_path = app_data_dir.join(DEFAULT_CHARACTER_SEED_MARKER);
    if marker_path.exists() {
        return Ok(());
    }

    let request = CreateCharacterRequest {
        character_id: DEFAULT_CHARACTER_ID.to_owned(),
        display_label: "Aldric Ironhand".to_owned(),
        race_id: HUMAN_RACE_ID.to_owned(),
        class_id: "class:fighter".to_owned(),
        level: 3,
        ability_scores: AbilityScoresDto {
            strength: 17,
            dexterity: 13,
            constitution: 14,
            intelligence: 14,
            wisdom: 12,
            charisma: 8,
        },
        ability_bonus_target: "strength".to_owned(),
        saved_at: DEFAULT_CHARACTER_SAVED_AT.to_owned(),
        // The starter character takes no alternate racial trait: it is a plain
        // Human Fighter, and seeding a swap nobody chose would be exactly the
        // fabricated-default this file's other seeds are each argued down to.
        selected_alternate_trait_keys: Vec::new(),
        companion_species: None,
        // The starter character takes no traits either: same "no
        // fabricated default" reasoning as the alternate-trait comment
        // immediately above.
        selected_traits: Vec::new(),
        trait_skill_choices: Vec::new(),
    };

    let character_input = compose_character_input(&request);
    let receipt = build_pilot_headless_receipt(&character_input);
    if receipt.status != HeadlessReceiptStatus::Computed {
        return Err("default starter character build did not compute; not seeding".to_owned());
    }

    let envelope = SavedCharacterEnvelope {
        character_id: request.character_id.clone(),
        revision_id: format!("{}.rev.1", request.character_id),
        revision_kind: SavedCharacterRevisionKind::Authoritative,
        saved_at: request.saved_at.clone(),
        schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
        app_or_runtime_version: app.package_info().version.to_string(),
        content_or_rules_provenance: SOURCE_PACKAGE_ID.to_owned(),
        game_system: GAME_SYSTEM_ID.to_owned(),
        latest_authoritative_revision_ref: format!("{}.rev.1", request.character_id),
        display_label: request.display_label.clone(),
        character_input,
    };

    let root = characters_root_from_app_data_dir(&app_data_dir).join(&request.character_id);
    SavedCharacterStore::save(&envelope, &root).map_err(|err| err.message)?;

    std::fs::create_dir_all(&app_data_dir).map_err(|err| format!("{}: {err}", app_data_dir.display()))?;
    std::fs::write(&marker_path, "seeded\n").map_err(|err| format!("{}: {err}", marker_path.display()))?;

    Ok(())
}

#[tauri::command]
pub fn list_saved_characters(app: tauri::AppHandle) -> Result<ListSavedCharactersResponse, String> {
    let characters_root = resolve_characters_root(&app)?;
    let listing = SavedCharacterStore::list_all(&characters_root).map_err(|err| err.message)?;

    Ok(ListSavedCharactersResponse {
        characters: listing.characters.iter().map(map_summary_dto).collect(),
        unreadable_count: listing.unreadable_entries.len(),
    })
}

#[tauri::command]
pub fn load_saved_character(
    app: tauri::AppHandle,
    request: LoadSavedCharacterRequest,
) -> Result<LoadSavedCharacterResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    load_saved_character_at_root(&root)
}

/// The real body of `load_saved_character`, split out from the
/// `AppHandle`-taking command so it is directly testable against a
/// temp-dir character root — the same `*_at_root` convention
/// `level_up_character_at_root` / `set_skill_allocations_at_root` already
/// established in this module.
pub(crate) fn load_saved_character_at_root(
    root: &Path,
) -> Result<LoadSavedCharacterResponse, String> {
    let envelope = SavedCharacterStore::load(root).map_err(|err| err.message)?;

    let (snapshot, diagnostics, corpus_receipt) =
        match resolve_unified_pilot_snapshot(&envelope.character_input, corpus_fixture_bundle()) {
            Ok((snapshot, corpus_receipt)) => (Some(snapshot), Vec::new(), corpus_receipt),
            Err(diagnostics) => (
                None,
                diagnostics,
                compute_pilot_with_corpus(&envelope.character_input, corpus_fixture_bundle()),
            ),
        };

    // Both already computed above — `corpus_receipt.base` IS the
    // `compute_pilot_base_chassis` output (explanations included), and
    // `corpus_receipt.corpus_derived.equipment_effects` IS the resolved
    // `EquippedActive` effect set. Reused rather than recomputed, matching
    // `contract::to_pilot_receipt`'s own reasoning for the identical pair.
    let explanations = map_explanations_dto(&corpus_receipt.base.explanations);
    let weapon_damage = map_weapon_damage_dto(&resolve_weapon_damage_breakdown(
        &envelope.character_input,
        corpus_fixture_bundle(),
        &corpus_receipt.corpus_derived.equipment_effects,
        corpus_receipt.base.ability_modifiers.strength,
    ));

    Ok(LoadSavedCharacterResponse {
        summary: summarize_envelope(&envelope),
        snapshot: snapshot.as_ref().map(map_snapshot_dto),
        diagnostics: map_diagnostics_dto(&diagnostics),
        corpus_derived: map_corpus_derived_dto(&corpus_receipt.corpus_derived),
        selected_feats: envelope.character_input.chosen.selected_feats.clone(),
        spells_selected: map_spells_selected_dto(&envelope.character_input.chosen.spells_selected),
        chosen_feat_targets: map_chosen_feat_targets_dto(&envelope.character_input),
        explanations,
        weapon_damage,
        selected_alternate_trait_keys: read_alternate_trait_keys(&envelope.character_input),
        selected_traits: envelope.character_input.chosen.selected_traits.clone(),
        resolved_racial_traits: resolve_racial_traits_for_character(&envelope.character_input),
    })
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelUpCharacterRequest {
    pub character_id: String,
    pub class_id: String,
    /// Additional player choices this level-up records (a hit-die roll or
    /// "take average" record, feat picks at feat-gaining levels, ...),
    /// appended to `chosen.selected_choices` verbatim. Empty by default —
    /// callers that only want the bare level increment (the pre-v0.6
    /// behavior) omit this field entirely.
    #[serde(default)]
    pub additional_choices: Vec<SelectedChoiceDto>,
    /// When present, replaces `chosen.skill_allocations` wholesale with
    /// this level-up's skill-point spend (same semantics as
    /// `set_skill_allocations`). `None`/omitted leaves the character's
    /// existing skill allocations untouched.
    #[serde(default)]
    pub skill_allocations: Option<Vec<SkillAllocationDto>>,
    pub saved_at: String,
}

/// Loads the saved character, increments/adds the requested class's level,
/// records any additional level-up choices (hit-die roll, feat picks) and
/// an optional skill-allocation update, recomputes via the real engine, and
/// re-saves — see `level_up_character_at_root` for the full semantics.
#[tauri::command]
pub fn level_up_character(
    app: tauri::AppHandle,
    request: LevelUpCharacterRequest,
) -> Result<CreateCharacterResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    let additional_choices = request
        .additional_choices
        .into_iter()
        .map(|choice| SelectedChoice {
            choice_set_id: choice.choice_set_id,
            selection_id: choice.selection_id,
        })
        .collect();
    let skill_allocations = request.skill_allocations.map(|allocations| {
        allocations
            .into_iter()
            .map(|skill| SkillAllocation {
                skill_id: skill.skill_id,
                ranks: skill.ranks,
            })
            .collect()
    });
    level_up_character_at_root(
        &root,
        &request.class_id,
        additional_choices,
        skill_allocations,
        &request.saved_at,
    )
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewLevelUpRequest {
    pub character_id: String,
    /// The class the next character level would be taken in — either one
    /// the character already holds (levels up by one) or a brand-new class
    /// (starts at class level 1).
    pub class_id: String,
}

/// What taking the next level in `class_id` grants, straight from Epic 7's
/// real per-class level-up engine
/// (`level_up::compute_level_up_grants_for_class`).
///
/// This exists because the frontend used to answer the same question from
/// a hand-authored `CLASS_FEATURES` table in `characterProgression.ts` —
/// bare labels (`'Bravery +1'`, `'Bonus combat feat'`) with no magnitudes
/// and no provenance, duplicating and drifting from the engine's own
/// grounded class tables. That table is deleted; this command replaces it.
///
/// An empty `automatic_features` is a real answer, not a failure: the
/// per-class level-up modules are individually gated (Fighter's, for
/// instance, is bounded to Human Fighter inputs) and
/// `compute_level_up_grants_for_class` returns an honestly-empty
/// `LevelUpPlan` for any class id outside the eleven PF1 Core classes it
/// grounds. The dialog renders that absence as absence rather than
/// inventing a placeholder line.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewLevelUpResponse {
    /// The class's own level before this transition (0 for a class the
    /// character does not yet hold).
    pub from_level: u8,
    /// The class's own level after this transition.
    pub to_level: u8,
    /// The character's total level after this transition.
    pub character_level: u8,
    /// Grants that fire automatically at `to_level` — no player choice
    /// needed.
    pub automatic_features: Vec<LevelUpGrantDto>,
    /// Open-ended "pick N from this list" grants.
    pub pick_from_lists: Vec<LevelUpPickListDto>,
    /// Named resource pools whose size changes across this transition.
    pub resource_pool_changes: Vec<LevelUpResourcePoolDeltaDto>,
    /// True when `to_level` crosses this class's PF1 capstone.
    pub capstone_threshold: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelUpGrantDto {
    pub name: String,
    /// The engine's own effect descriptions, verbatim — same discipline as
    /// `ExplanationDto::detail`.
    pub effects: Vec<LevelUpGrantEffectDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelUpGrantEffectDto {
    pub description: String,
    pub value: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelUpPickListDto {
    /// `"Feat"`, `"Spell"` or `"RagePower"`.
    pub category: String,
    pub count: u8,
    pub candidates: Vec<LevelUpPickCandidateDto>,
    pub filter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelUpPickCandidateDto {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelUpResourcePoolDeltaDto {
    pub pool_id: String,
    pub from_value: i16,
    pub to_value: i16,
}

/// Previews what the next level in `class_id` grants, without persisting
/// anything. Read-only twin of `level_up_character`.
#[tauri::command]
pub fn preview_level_up(
    app: tauri::AppHandle,
    request: PreviewLevelUpRequest,
) -> Result<PreviewLevelUpResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    preview_level_up_at_root(&root, &request.class_id)
}

/// The real body of `preview_level_up` — see `load_saved_character_at_root`
/// for why the `AppHandle` is split off.
pub(crate) fn preview_level_up_at_root(
    root: &Path,
    class_id: &str,
) -> Result<PreviewLevelUpResponse, String> {
    let envelope = SavedCharacterStore::load(root).map_err(|err| err.message)?;
    let class_levels = &envelope.character_input.chosen.class_levels;

    let from_level = class_levels
        .iter()
        .find(|held| held.class_id == class_id)
        .map(|held| held.level)
        .unwrap_or(0);
    let to_level = from_level.saturating_add(1);
    let character_level = class_levels
        .iter()
        .map(|held| held.level)
        .fold(0u8, |sum, level| sum.saturating_add(level))
        .saturating_add(1);

    // `compute_level_up_grants_for_class`, not the top-level
    // `compute_level_up_grants`: the latter dispatches on the character's
    // *sole* class and returns an empty plan for any multiclass build, so
    // it would silently blank the preview for exactly the characters the
    // sheet already supports (see `pf1_adapter.rs`'s register-A2 note).
    let plan: LevelUpPlan = compute_level_up_grants_for_class(
        &envelope.character_input,
        class_id,
        from_level,
        to_level,
    );

    Ok(PreviewLevelUpResponse {
        from_level,
        to_level,
        character_level,
        automatic_features: plan
            .automatic_features
            .iter()
            .map(|grant| LevelUpGrantDto {
                name: grant.name.clone(),
                effects: grant
                    .effects
                    .iter()
                    .map(|effect| LevelUpGrantEffectDto {
                        description: effect.description.clone(),
                        value: effect.value,
                    })
                    .collect(),
            })
            .collect(),
        pick_from_lists: plan
            .pick_from_lists
            .iter()
            .map(|list| LevelUpPickListDto {
                category: format!("{:?}", list.category),
                count: list.count,
                candidates: list
                    .candidates
                    .iter()
                    .map(|candidate| LevelUpPickCandidateDto {
                        id: candidate.id.clone(),
                        name: candidate.name.clone(),
                    })
                    .collect(),
                filter: list.filter.clone(),
            })
            .collect(),
        resource_pool_changes: plan
            .resource_pool_change
            .pools
            .iter()
            .map(|pool| LevelUpResourcePoolDeltaDto {
                pool_id: pool.pool_id.clone(),
                from_value: pool.from_value,
                to_value: pool.to_value,
            })
            .collect(),
        capstone_threshold: plan.capstone_threshold,
    })
}

/// The wire-level projection of `ActiveState` for the `add_equipment_selection`
/// request. A separate DTO (rather than deriving `Deserialize` on
/// `ActiveState` itself) because the `codex` crate has zero dependencies —
/// see this module's own top-of-file note.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ActiveStateDto {
    EquippedActive,
    Absent,
    SelectedInactive,
}

impl From<ActiveStateDto> for ActiveState {
    fn from(dto: ActiveStateDto) -> Self {
        match dto {
            ActiveStateDto::EquippedActive => ActiveState::EquippedActive,
            ActiveStateDto::Absent => ActiveState::Absent,
            ActiveStateDto::SelectedInactive => ActiveState::SelectedInactive,
        }
    }
}

/// Reverse of the above — needed by `export_character`'s
/// `CharacterInput -> CharacterInputDto` projection (Criterion 24's
/// round-trip fix).
impl From<ActiveState> for ActiveStateDto {
    fn from(state: ActiveState) -> Self {
        match state {
            ActiveState::EquippedActive => ActiveStateDto::EquippedActive,
            ActiveState::Absent => ActiveStateDto::Absent,
            ActiveState::SelectedInactive => ActiveStateDto::SelectedInactive,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddEquipmentSelectionRequest {
    pub character_id: String,
    pub item_id: String,
    pub active_state: ActiveStateDto,
    pub saved_at: String,
}

/// Loads the saved character, appends the requested equipment selection,
/// recomputes via the real engine, and re-saves — see
/// `add_equipment_selection_at_root` for the full semantics.
#[tauri::command]
pub fn add_equipment_selection(
    app: tauri::AppHandle,
    request: AddEquipmentSelectionRequest,
) -> Result<CreateCharacterResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    add_equipment_selection_at_root(
        &root,
        &request.item_id,
        request.active_state.into(),
        &request.saved_at,
    )
}

/// v0.6 alpha swarm (risks-and-open-questions.md item 9): the outcome of an
/// atomic equipment purchase. A THIRD case beyond `CreateCharacterResponse`'s
/// `Saved`/`Blocked` doesn't exist here on purpose -- `Blocked` already
/// carries `diagnostics: Vec<DiagnosticDto>`, so an unaffordable purchase or
/// an item with no known cost is represented as a `Blocked` response with
/// one hand-authored diagnostic, the same wire shape frontend already
/// handles for every other mutation command, rather than a new response
/// shape to integrate.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind")]
pub enum PurchaseEquipmentResponse {
    Purchased {
        summary: Box<CharacterSummaryDto>,
        snapshot: PilotSnapshotDto,
        // Same reasoning as `CreateCharacterResponse::Saved`'s own
        // `corpus_derived` field: a per-field rename, not an enum-wide
        // `rename_all`, which would also lowercase the `"Purchased"`/
        // `"Blocked"` tag values themselves.
        #[serde(rename = "corpusDerived")]
        corpus_derived: CorpusDerivedDto,
        money: CharacterMoneyDto,
    },
    Blocked {
        diagnostics: Vec<DiagnosticDto>,
    },
}

/// `purchase_equipment`'s real implementation: real correctness fix for
/// risks-and-open-questions.md item 9 ("Money panel not coupled to
/// equipment purchases... deliberately not built as two non-atomic
/// mutations... risking a partial-apply correctness bug").
///
/// Sequencing (the actual transaction shape, chosen deliberately): resolve
/// the item's real `cost_gp` (headless, no corpus needed --
/// `equipment_cost_gp_headless_resolve`'s own doc comment explains why) and
/// pre-check affordability against the CURRENT balance BEFORE mutating
/// anything. Only once both checks pass does this call
/// `add_equipment_selection_at_root` (the existing equipment mutation,
/// unchanged); only if THAT reaches `Computed` and saves does this deduct
/// the cost via `adjust_character_money_at_root`. This ordering means the
/// only case where equipment is added without a successful matching charge
/// is a true I/O failure on the money-file write immediately after an
/// already-verified-affordable, already-persisted equipment save -- an
/// honestly narrow residual window (same disk, same moment, would likely
/// also have broken the equipment save itself), not the two-independent-
/// frontend-calls-with-no-pre-check gap this fix actually closes. A full
/// two-phase-commit / journaled rollback across the two separate files
/// (`character_input.txt`, `money.json`) would be real engineering but is
/// not proportionate to this codebase's current maturity level or this
/// swarm's bar -- noted here rather than silently assumed away.
///
/// An item with no known `cost_gp` (a `(Base)` template record or a
/// formula-priced equipment modifier) is treated the same as
/// insufficient funds: `Blocked`, nothing mutated, never a free item.
pub(crate) fn purchase_equipment_at_root(
    root: &Path,
    item_id: &str,
    active_state: ActiveState,
    saved_at: &str,
) -> Result<PurchaseEquipmentResponse, String> {
    // SD-27: the Add Weapon / Add Armor picker hands this command a catalog
    // `key` straight off `build_equipment_catalog()`, so the by-key lookup
    // is tried first -- it names the exact row the user picked, including
    // for the one cross-book identity collision (`"Wooden"`) where the
    // free-form resolver deliberately keeps CRB's answer. The free-form
    // resolver remains the fallback for ids that are not catalog keys at
    // all, notably the legacy `"item:longsword"` fixture namespace that
    // seeded characters still carry.
    let Some(cost_gp) =
        codex::rules_core::equipment_resolver::equipment_catalog_row_by_key(item_id)
            .map_or_else(
                || codex::rules_core::equipment_resolver::equipment_cost_gp_headless_resolve(item_id),
                |row| row.cost_gp,
            )
    else {
        return Ok(PurchaseEquipmentResponse::Blocked {
            diagnostics: vec![DiagnosticDto {
                id: "money.equipment_purchase.unknown_cost".to_owned(),
                message: format!(
                    "'{item_id}' has no known gold-piece cost in the equipment catalog (a \
                     template/base record with no independent price, or a formula-priced \
                     equipment modifier), so affordability cannot be verified. The purchase \
                     was not applied and no funds were charged."
                ),
                claim_blocking: true,
            }],
        });
    };

    let cost_copper = money::gp_to_copper(cost_gp);
    let balance_copper = load_character_money_at_root(root)?.total_copper;
    if balance_copper < cost_copper {
        return Ok(PurchaseEquipmentResponse::Blocked {
            diagnostics: vec![DiagnosticDto {
                id: "money.equipment_purchase.insufficient_funds".to_owned(),
                message: format!(
                    "'{item_id}' costs {cost_copper} cp but the character's balance is only \
                     {balance_copper} cp. The purchase was not applied and no funds were \
                     charged."
                ),
                claim_blocking: true,
            }],
        });
    }

    match add_equipment_selection_at_root(root, item_id, active_state, saved_at)? {
        CreateCharacterResponse::Blocked { diagnostics } => {
            Ok(PurchaseEquipmentResponse::Blocked { diagnostics })
        }
        CreateCharacterResponse::Saved { summary, snapshot, corpus_derived } => {
            let cost_signed = i64::try_from(cost_copper)
                .map_err(|_| "purchase cost overflows a signed 64-bit total".to_owned())?;
            let money = adjust_character_money_at_root(root, -cost_signed)?;
            Ok(PurchaseEquipmentResponse::Purchased { summary, snapshot, corpus_derived, money })
        }
    }
}

/// Same wire shape as `PurchaseEquipmentResponse` (`Attached`/`Blocked`,
/// same field set) -- deliberately not reused verbatim so the two
/// commands' response `kind` tags stay distinct on the wire
/// (`"Attached"` vs `"Purchased"`), matching frontend's own
/// per-command-outcome convention.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind")]
pub enum AttachEquipmentModifierResponse {
    Attached {
        summary: Box<CharacterSummaryDto>,
        snapshot: PilotSnapshotDto,
        #[serde(rename = "corpusDerived")]
        corpus_derived: CorpusDerivedDto,
        money: CharacterMoneyDto,
    },
    Blocked {
        diagnostics: Vec<DiagnosticDto>,
    },
}

/// `attach_equipment_modifier`'s real implementation (v0.6 alpha swarm
/// items 1+27 sub-task 6, frontend-proposed shape). Mirrors
/// `purchase_equipment_at_root`'s atomic resolve-cost -> check-affordability
/// -> mutate -> charge sequencing, with two deliberate differences:
///
/// 1. **Validates `modifier_item_id` against the real equipment catalog
///    first**, before any cost or target-selection check -- a typo'd or
///    fabricated modifier id must never silently attach.
///
///    **SD-27 dead-affordance fix.** This check read
///    `crb::equipment_tables()` alone while the Attach Modifier picker that
///    feeds it is served by `build_equipment_catalog()`, which has spanned
///    all six ingested books since the catalog widening. The picker offered
///    763 `Equipmods` rows and this command accepted only CRB's 658, so
///    **105 rows (ACG 48, ARG 15, PU 42) were offered and then refused**
///    with "is not a recognized equipment catalog item" -- the exact dead
///    affordance `docs/governance/no-stub-mvp-doctrine.md` forbids.
///    Recognition now runs against
///    `equipment_resolver::equipment_catalog_row_by_key`, the same six-book
///    row set the picker is built from.
///
///    **Recognition and price come from one lookup, deliberately.** 20 of
///    those 105 refused rows carry a real, non-zero flat price (ACG's
///    4,500 gp `Amorphous`, ARG's 500 gp `Material ~ Whipwood`, ...).
///    Widening recognition while leaving the CRB-only cost path in place
///    would have attached those for free -- silent mispricing, strictly
///    worse than an honest refusal. So the single `equipment_catalog_row_by_key`
///    call below supplies **both** answers from the same row, which is
///    also what makes them impossible to drift apart. It is not
///    interchangeable with `equipment_cost_gp_headless_resolve` here:
///    the picker hands this command a catalog `key`, and for the one
///    genuine cross-book identity collision (`"Wooden"` is APG's `KEY:`
///    at 20 gp and a CRB row's `name` at 1 gp) only the by-key lookup
///    answers with the row the user actually picked.
/// 2. **An unknown `cost_gp` is treated as free to attach, not blocked** --
///    a deliberate deviation from `purchase_equipment_at_root`'s own
///    "unknown cost = blocked, same as unaffordable" rule. Checked against
///    the real static table before choosing this: the actual magical
///    weapon/armor enhancement records (`"Special Ability ~ +1 ~
///    Weapon"` through `~ +10 ~`) all resolve `cost_gp: None` (real PF1
///    enhancement pricing is a bonus-squared formula, not a flat catalog
///    price) -- mirroring `purchase_equipment`'s block-on-unknown-cost
///    behavior here would block exactly the headline use case this
///    command exists for. Only a modifier with a real, known `cost_gp`
///    (e.g. Masterwork, `Some(0.0)` in the current table -- itself a
///    known pre-existing pricing gap, not introduced here) is ever
///    actually charged.
///
/// The target `item_id` must already exist in `equipment_selections` --
/// checked via a read before any charge, so a not-found target is a
/// `Blocked` response with zero side effects, never a charge with nothing
/// to attach to.
pub(crate) fn attach_equipment_modifier_at_root(
    root: &Path,
    item_id: &str,
    modifier_item_id: &str,
    saved_at: &str,
) -> Result<AttachEquipmentModifierResponse, String> {
    let Some(modifier_row) =
        codex::rules_core::equipment_resolver::equipment_catalog_row_by_key(modifier_item_id)
    else {
        return Ok(AttachEquipmentModifierResponse::Blocked {
            diagnostics: vec![DiagnosticDto {
                id: "equipment.attach_modifier.unknown_item".to_owned(),
                message: format!(
                    "'{modifier_item_id}' is not a recognized equipment catalog item. Nothing \
                     was attached and no funds were charged."
                ),
                claim_blocking: true,
            }],
        });
    };

    let envelope = codex::saved_character::local_store::SavedCharacterStore::load(root)
        .map_err(|err| err.message)?;
    let target_exists = envelope
        .character_input
        .chosen
        .equipment_selections
        .iter()
        .any(|selection| selection.item_id == item_id);
    if !target_exists {
        return Ok(AttachEquipmentModifierResponse::Blocked {
            diagnostics: vec![DiagnosticDto {
                id: "equipment.attach_modifier.target_not_found".to_owned(),
                message: format!(
                    "'{item_id}' is not an equipped selection on this character. Nothing was \
                     attached and no funds were charged."
                ),
                claim_blocking: true,
            }],
        });
    }

    // Same row that satisfied recognition above -- see this function's doc
    // comment for why price must not come from a second, independent lookup.
    let cost_copper = match modifier_row.cost_gp {
        Some(cost_gp) => money::gp_to_copper(cost_gp),
        None => 0,
    };

    if cost_copper > 0 {
        let balance_copper = load_character_money_at_root(root)?.total_copper;
        if balance_copper < cost_copper {
            return Ok(AttachEquipmentModifierResponse::Blocked {
                diagnostics: vec![DiagnosticDto {
                    id: "money.equipment_attach_modifier.insufficient_funds".to_owned(),
                    message: format!(
                        "'{modifier_item_id}' costs {cost_copper} cp but the character's \
                         balance is only {balance_copper} cp. Nothing was attached and no \
                         funds were charged."
                    ),
                    claim_blocking: true,
                }],
            });
        }
    }

    match mutate_saved_character_at_root(root, saved_at, |character_input| {
        apply_attach_equipment_modifier(character_input, item_id, modifier_item_id);
    })? {
        CreateCharacterResponse::Blocked { diagnostics } => {
            Ok(AttachEquipmentModifierResponse::Blocked { diagnostics })
        }
        CreateCharacterResponse::Saved { summary, snapshot, corpus_derived } => {
            let money = if cost_copper > 0 {
                let cost_signed = i64::try_from(cost_copper)
                    .map_err(|_| "attach cost overflows a signed 64-bit total".to_owned())?;
                adjust_character_money_at_root(root, -cost_signed)?
            } else {
                load_character_money_at_root(root)?
            };
            Ok(AttachEquipmentModifierResponse::Attached { summary, snapshot, corpus_derived, money })
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachEquipmentModifierRequest {
    pub character_id: String,
    pub item_id: String,
    pub modifier_item_id: String,
    pub saved_at: String,
}

/// See `attach_equipment_modifier_at_root` for the full transaction-shape
/// reasoning (free-attach on unknown cost, target/modifier validation
/// before any charge).
#[tauri::command]
pub fn attach_equipment_modifier(
    app: tauri::AppHandle,
    request: AttachEquipmentModifierRequest,
) -> Result<AttachEquipmentModifierResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    attach_equipment_modifier_at_root(
        &root,
        &request.item_id,
        &request.modifier_item_id,
        &request.saved_at,
    )
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseEquipmentRequest {
    pub character_id: String,
    pub item_id: String,
    pub active_state: ActiveStateDto,
    pub saved_at: String,
}

/// Atomically resolves `item_id`'s real catalog cost, verifies the
/// character can afford it, appends the equipment selection, and deducts
/// the cost from the persisted money balance — or applies none of it and
/// returns a `Blocked` diagnostic. See `purchase_equipment_at_root` for the
/// full transaction-shape reasoning.
#[tauri::command]
pub fn purchase_equipment(
    app: tauri::AppHandle,
    request: PurchaseEquipmentRequest,
) -> Result<PurchaseEquipmentResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    purchase_equipment_at_root(
        &root,
        &request.item_id,
        request.active_state.into(),
        &request.saved_at,
    )
}

/// The wire-level projection of `AcquisitionMode` for the
/// `add_spell_selection` request. A separate DTO for the same reason as
/// `ActiveStateDto` above.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AcquisitionModeDto {
    Known,
    Prepared,
    Granted,
}

impl From<AcquisitionModeDto> for AcquisitionMode {
    fn from(dto: AcquisitionModeDto) -> Self {
        match dto {
            AcquisitionModeDto::Known => AcquisitionMode::Known,
            AcquisitionModeDto::Prepared => AcquisitionMode::Prepared,
            AcquisitionModeDto::Granted => AcquisitionMode::Granted,
        }
    }
}

/// Reverse of the above — needed by `export_character`'s
/// `CharacterInput -> CharacterInputDto` projection (Criterion 24's
/// round-trip fix).
impl From<AcquisitionMode> for AcquisitionModeDto {
    fn from(mode: AcquisitionMode) -> Self {
        match mode {
            AcquisitionMode::Known => AcquisitionModeDto::Known,
            AcquisitionMode::Prepared => AcquisitionModeDto::Prepared,
            AcquisitionMode::Granted => AcquisitionModeDto::Granted,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddSpellSelectionRequest {
    pub character_id: String,
    pub spell_id: String,
    pub source_class_id: String,
    pub acquisition_mode: AcquisitionModeDto,
    pub saved_at: String,
}

/// Loads the saved character, appends the requested spell selection,
/// recomputes via the real engine, and re-saves — see
/// `add_spell_selection_at_root` for the full semantics.
#[tauri::command]
pub fn add_spell_selection(
    app: tauri::AppHandle,
    request: AddSpellSelectionRequest,
) -> Result<CreateCharacterResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    add_spell_selection_at_root(
        &root,
        &request.spell_id,
        &request.source_class_id,
        request.acquisition_mode.into(),
        &request.saved_at,
    )
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordAndPrepareSpellSelectionRequest {
    pub character_id: String,
    pub spell_id: String,
    pub source_class_id: String,
    pub saved_at: String,
}

/// Appends BOTH a `Known` and a `Prepared` entry for the same spell in one
/// atomic mutation — breaks the Wizard spellbook bootstrap deadlock
/// `add_spell_selection` alone cannot: `unmet_wizard_spellbook_conditions`
/// requires a non-empty recorded set AND a non-empty prepared set
/// simultaneously, but `add_spell_selection` only ever appends one spell in
/// one mode per call, and `mutate_saved_character_at_root` discards any
/// call that doesn't independently reach `Computed` — so a `Known`-only
/// call is Blocked (nothing prepared yet) and never persists, and a
/// `Prepared`-only call is *also* Blocked (the prepared spell isn't in the
/// still-empty recorded set) and never persists either. See
/// `apply_record_and_prepare_spell_selection`'s own doc comment for the
/// full analysis.
///
/// Use this once, for the character's first spell. After that, the plain
/// `add_spell_selection` (either mode) works normally for every subsequent
/// spell — this command is not a general replacement for it.
#[tauri::command]
pub fn record_and_prepare_spell_selection(
    app: tauri::AppHandle,
    request: RecordAndPrepareSpellSelectionRequest,
) -> Result<CreateCharacterResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    record_and_prepare_spell_selection_at_root(
        &root,
        &request.spell_id,
        &request.source_class_id,
        &request.saved_at,
    )
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddFeatSelectionRequest {
    pub character_id: String,
    pub feat_id: String,
    /// The target a chooser feat names -- a weapon, skill or school, without
    /// its prefix (the prefix comes from the feat's own contract, so callers
    /// never assemble selection ids). `None` for feats that take no target,
    /// and also legitimate for a chooser feat whose target is not chosen yet.
    ///
    /// Defaults to `None` so a caller that predates chooser targets keeps
    /// working unchanged.
    #[serde(default)]
    pub target: Option<String>,
    pub saved_at: String,
}

/// Loads the saved character, appends the requested feat selection,
/// recomputes via the real engine, and re-saves — see
/// `add_feat_selection_at_root` for the full semantics.
///
/// **SD-27: this now refuses a feat whose real corpus prerequisites the
/// character does not meet.** See
/// `add_feat_selection_enforcing_prerequisites_at_root`.
#[tauri::command]
pub fn add_feat_selection(
    app: tauri::AppHandle,
    request: AddFeatSelectionRequest,
) -> Result<CreateCharacterResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    add_feat_selection_enforcing_prerequisites_at_root(
        &root,
        &request.feat_id,
        request.target.as_deref(),
        &request.saved_at,
    )
}

// ---------------------------------------------------------------------------
// SD-27: feat prerequisite enforcement
// ---------------------------------------------------------------------------
//
// There was no feat prerequisite enforcement anywhere in the product: a
// Fighter 1 with a +1 base attack bonus could take Improved Two-Weapon
// Fighting (BAB +6, Dex 17, Two-Weapon Fighting), and all 690 offered feats
// were accepted by every character. The engine side is
// `codex::rules_core::feat_prereqs`; these two seams are how it reaches a
// player.
//
// Both seams exist deliberately, and neither replaces the other:
//
//  * `list_feats_for_character` is the *honest UI*: it returns all 690
//    records with a verdict on each, so the picker greys the unavailable
//    ones and shows the reason. Removing them from the list instead would
//    hide the rules from the player.
//  * `add_feat_selection_enforcing_prerequisites_at_root` is the *guard*:
//    the mutation itself refuses. Without it the check would be advisory —
//    any caller that skipped the picker (a level-up grant, a replayed
//    command, a future importer) could still write an illegal character to
//    disk.

/// Builds the prerequisite fact snapshot for the character saved at `root`.
///
/// The base attack bonus comes from the engine's own computed chassis
/// (`corpus_receipt.base.base_attack_bonus`) rather than being re-derived
/// here, so a Fighter, an Unchained Rogue and an ACG Warpriest all get the
/// number their own class chassis produced. `base` is used rather than the
/// unified snapshot because it is computed even for a build whose
/// deterministic posture is blocked — a character whose sheet cannot fully
/// compute must still get truthful feat prerequisites rather than a
/// silently-zero BAB.
pub(crate) fn character_prereq_facts_at_root(
    root: &Path,
) -> Result<
    (
        codex::saved_character::SavedCharacterEnvelope,
        codex::rules_core::feat_prereqs::pre_tokens::CharacterPrereqFacts,
    ),
    String,
> {
    let envelope = SavedCharacterStore::load(root).map_err(|err| err.message)?;
    let receipt = compute_pilot_with_corpus(&envelope.character_input, corpus_fixture_bundle());
    let facts = codex::rules_core::feat_prereqs::character_prereq_facts(
        &envelope.character_input,
        receipt.base.base_attack_bonus,
    );
    Ok((envelope, facts))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListFeatsForCharacterRequest {
    pub character_id: String,
    /// The same optional narrowing `list_feats` takes. Omitted/`null`
    /// fields match everything.
    #[serde(default)]
    pub filter: crate::feat_catalog::FeatCatalogFilter,
}

/// The feat catalog with each record's real prerequisite verdict for this
/// character attached.
///
/// Serves **all 690 records**, ineligible ones included and marked, because
/// the requirement is that an unavailable feat is *visibly* unavailable
/// with its reason — not that it disappears. A picker that silently dropped
/// the rows would tell a player nothing about why their build cannot take
/// Improved Two-Weapon Fighting.
#[tauri::command]
pub fn list_feats_for_character(
    app: tauri::AppHandle,
    request: ListFeatsForCharacterRequest,
) -> Result<crate::feat_catalog::FeatCatalogResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    list_feats_for_character_at_root(&root, &request.filter)
}

/// The real body of `list_feats_for_character`, split from the
/// `AppHandle`-taking command so it is directly testable against a temp-dir
/// character root — this module's established `*_at_root` convention.
pub(crate) fn list_feats_for_character_at_root(
    root: &Path,
    filter: &crate::feat_catalog::FeatCatalogFilter,
) -> Result<crate::feat_catalog::FeatCatalogResponse, String> {
    let (_, facts) = character_prereq_facts_at_root(root)?;
    Ok(crate::feat_catalog::filter_feat_catalog_with_eligibility(filter, &facts))
}

/// `add_feat_selection_at_root`, refusing first when the character does not
/// meet the feat's real corpus prerequisites.
///
/// Three deliberate properties:
///
/// * **Only a definitively unmet prerequisite refuses.** A clause the
///   engine cannot evaluate is reported by the picker and does not block
///   here either, so an unmodelled `PRE` kind can never lock a player out
///   of a feat they are entitled to.
/// * **A feat id with no catalog record is passed through untouched.**
///   `chosen.selected_feats` legitimately holds ids this catalog does not
///   carry (engine-seeded tokens for content outside the five ingested
///   books). Refusing those would break existing characters over a lookup
///   miss rather than over a rule.
/// * **The error names the reason.** A refusal a player cannot act on is
///   as bad as no refusal at all.
pub(crate) fn add_feat_selection_enforcing_prerequisites_at_root(
    root: &Path,
    feat_id: &str,
    target: Option<&str>,
    saved_at: &str,
) -> Result<CreateCharacterResponse, String> {
    let (_, facts) = character_prereq_facts_at_root(root)?;
    if let Some(report) =
        codex::rules_core::feat_prereqs::evaluate_feat_key_prerequisites(feat_id, &facts)
    {
        if let Some(reason) = report.unavailable_reason() {
            return Err(format!(
                "'{}' cannot be taken by this character: {reason}",
                report.feat_key
            ));
        }
    }
    add_feat_selection_at_root(root, feat_id, target, saved_at)
}

// ---------------------------------------------------------------------------
// Selection removal (the inverse of the three add paths above)
// ---------------------------------------------------------------------------
//
// Every selection command in this module was append-only: a character sheet
// could take a feat, learn a spell or buy a weapon and then had no way to
// undo any of it short of hand-editing the saved JSON. The three functions
// below are the missing inverses, each built on the SAME
// `mutate_saved_character_at_root` load -> mutate -> recompute -> re-save
// spine its add-path twin uses, so a removal that leaves the build unable to
// compute is discarded exactly like an add that does — the on-disk character
// is never left holding a stale computed value.
//
// Deliberately NOT built here: post-creation alternate racial trait removal.
// Alternate racial traits have no post-creation *add* path either
// (`selected_alternate_trait_keys` is a `create_character` field, resolved
// once by `resolve_alternate_trait_choices`); adding only the remove half
// would give the sheet a one-way door in the other direction. See this
// module's `read_alternate_trait_keys`.

/// Removes one held copy of `feat_id` from `chosen.selected_feats`, plus the
/// chooser target that copy recorded.
///
/// Returns `false` without mutating when the character does not hold the
/// feat — the caller turns that into an honest error rather than a
/// `Saved` response for a removal that removed nothing.
///
/// Three details that are not arbitrary:
///
/// * **Matching is by [`feat_identity::same`], not string equality.**
///   `selected_feats` really carries both shapes (`"Toughness"` from the
///   picker, `"feat:toughness"` from creation seeding), and the same
///   function `add_feat_selection_enforcing_prerequisites_at_root` matches
///   with must be the one that unmatches, or a feat becomes unremovable
///   purely because of which path added it.
/// * **One copy, not all copies.** `selected_feats` is an append-only list
///   that legitimately holds a chooser feat more than once (Weapon Focus in
///   two weapons). Removing every copy on a request to remove one would
///   silently discard a pick the player did not name.
/// * **The last copy takes its orphaned targets with it.** Each chooser
///   feat owns a distinct `choice_set_id`
///   ([`feat_effects::CHOOSER_FEAT_CONTRACTS`]), so once no copy of the
///   feat remains, every choice under that set belongs to nothing. Leaving
///   them behind would show a target for a feat the character no longer
///   has. When `target` names a specific one, only that choice goes and any
///   remaining copies keep theirs.
pub(crate) fn apply_remove_feat_selection(
    character_input: &mut CharacterInput,
    feat_id: &str,
    target: Option<&str>,
) -> bool {
    use codex::rules_core::feat_identity;

    let Some(index) = character_input
        .chosen
        .selected_feats
        .iter()
        .position(|held| feat_identity::same(held, feat_id))
    else {
        return false;
    };
    character_input.chosen.selected_feats.remove(index);

    let Some(contract) = feat_effects::chooser_contract_for_feat(feat_id) else {
        // Not a chooser feat: it never recorded a target, so there is
        // nothing else to take with it.
        return true;
    };

    let still_held = character_input
        .chosen
        .selected_feats
        .iter()
        .any(|held| feat_identity::same(held, feat_id));

    if let Some(named) = target {
        let selection_id = format!("{}{}", contract.selection_prefix, named.trim());
        if let Some(choice_index) = character_input
            .chosen
            .selected_choices
            .iter()
            .position(|choice| {
                choice.choice_set_id == contract.choice_set_id
                    && choice.selection_id.eq_ignore_ascii_case(&selection_id)
            })
        {
            character_input.chosen.selected_choices.remove(choice_index);
        }
    }

    if !still_held {
        character_input
            .chosen
            .selected_choices
            .retain(|choice| choice.choice_set_id != contract.choice_set_id);
    }

    true
}

/// The feat whose own prerequisites this removal would break, and why.
///
/// Reuses `rules_core::feat_prereqs` rather than restating any rule: it
/// evaluates every OTHER feat the character holds twice — once against the
/// facts as they stand, once against the facts as they would stand after
/// the removal — and reports the first feat that was eligible before and is
/// definitively ineligible after.
///
/// The before/after comparison is the whole point. Evaluating only the
/// "after" state would refuse removals for feats that were already failing
/// their prerequisites before anyone touched anything (a character seeded
/// with engine tokens outside the five ingested books, or one built before
/// `add_feat_selection` enforced prerequisites at all), which would tell the
/// player that removing feat A broke feat B when A had nothing to do with
/// it. Only a regression caused *by this removal* refuses.
///
/// A feat with no catalog record is skipped in both passes, matching
/// `add_feat_selection_enforcing_prerequisites_at_root`'s own decision that
/// a lookup miss is not a rules verdict.
pub(crate) fn feat_removal_dependency_refusal(
    before: &CharacterInput,
    after: &CharacterInput,
) -> Option<String> {
    use codex::rules_core::feat_prereqs::{
        character_prereq_facts, evaluate_feat_key_prerequisites,
    };

    let facts_before = character_prereq_facts(
        before,
        compute_pilot_with_corpus(before, corpus_fixture_bundle())
            .base
            .base_attack_bonus,
    );
    let facts_after = character_prereq_facts(
        after,
        compute_pilot_with_corpus(after, corpus_fixture_bundle())
            .base
            .base_attack_bonus,
    );

    for dependent in &after.chosen.selected_feats {
        let Some(report_after) = evaluate_feat_key_prerequisites(dependent, &facts_after) else {
            continue;
        };
        if report_after.is_eligible {
            continue;
        }
        let was_eligible = evaluate_feat_key_prerequisites(dependent, &facts_before)
            .map(|report| report.is_eligible)
            .unwrap_or(false);
        if !was_eligible {
            // Already failing before this removal — not this removal's doing.
            continue;
        }
        let reason = report_after
            .unavailable_reason()
            .unwrap_or_else(|| "its prerequisites would no longer be met".to_owned());
        return Some(format!(
            "'{}' still depends on it: {reason}",
            report_after.feat_key
        ));
    }

    None
}

/// `remove_feat_selection`'s real implementation.
///
/// Order matters and is the mirror image of
/// `add_feat_selection_enforcing_prerequisites_at_root`: the refusals happen
/// *before* `mutate_saved_character_at_root` is entered, because that
/// function takes an infallible closure — a rejected removal must surface as
/// an `Err` here rather than as a silently-unchanged `Saved` envelope that
/// tells the player the feat is gone when it is not.
///
/// Two refusals, both naming their reason:
///
/// * the character does not hold the feat at all, and
/// * removing it would break another held feat's prerequisites
///   (`feat_removal_dependency_refusal`).
///
/// Past those, `mutate_saved_character_at_root` supplies the same
/// recompute-before-persist gate every add path has, so a removal that left
/// the build unable to compute returns `Blocked` with the engine's own
/// diagnostics and leaves the saved character untouched.
pub(crate) fn remove_feat_selection_at_root(
    root: &Path,
    feat_id: &str,
    target: Option<&str>,
    saved_at: &str,
) -> Result<CreateCharacterResponse, String> {
    let envelope = SavedCharacterStore::load(root).map_err(|err| err.message)?;
    let before = envelope.character_input.clone();

    let mut after = before.clone();
    if !apply_remove_feat_selection(&mut after, feat_id, target) {
        return Err(format!(
            "'{feat_id}' cannot be removed: this character does not hold it"
        ));
    }

    if let Some(dependency) = feat_removal_dependency_refusal(&before, &after) {
        return Err(format!("'{feat_id}' cannot be removed: {dependency}"));
    }

    mutate_saved_character_at_root(root, saved_at, |character_input| {
        apply_remove_feat_selection(character_input, feat_id, target);
    })
}

/// Removes every `chosen.spells_selected` entry for `spell_id` under
/// `source_class_id`, in every acquisition mode.
///
/// Returns `false` without mutating when no entry matched.
///
/// **Every mode, not one.** `record_and_prepare_spell_selection` writes a
/// `Known` and a `Prepared` entry for the same spell in one atomic
/// mutation (see its doc comment), so removing only the `Known` half would
/// leave the character with a spell prepared that they no longer know —
/// a state the engine's own spellbook conditions treat as broken. "Forget
/// this spell" is the honest inverse of the pair.
pub(crate) fn apply_remove_spell_selection(
    character_input: &mut CharacterInput,
    spell_id: &str,
    source_class_id: &str,
) -> bool {
    let before = character_input.chosen.spells_selected.len();
    character_input.chosen.spells_selected.retain(|selection| {
        !(selection.spell_id.eq_ignore_ascii_case(spell_id)
            && selection.source_class_id.eq_ignore_ascii_case(source_class_id))
    });
    character_input.chosen.spells_selected.len() != before
}

/// `remove_spell_selection`'s real implementation — the same
/// load -> mutate -> recompute -> re-save spine
/// `add_spell_selection_at_root` uses.
///
/// A prepared caster whose last spell this would remove gets a `Blocked`
/// response carrying the engine's own spellbook diagnostics, and the saved
/// character is left exactly as it was. That is `mutate_saved_character_at_root`'s
/// standing invariant, not a special case here: a Wizard with an empty
/// spellbook does not compute, so it does not persist.
pub(crate) fn remove_spell_selection_at_root(
    root: &Path,
    spell_id: &str,
    source_class_id: &str,
    saved_at: &str,
) -> Result<CreateCharacterResponse, String> {
    let envelope = SavedCharacterStore::load(root).map_err(|err| err.message)?;
    let mut probe = envelope.character_input.clone();
    if !apply_remove_spell_selection(&mut probe, spell_id, source_class_id) {
        return Err(format!(
            "'{spell_id}' cannot be removed: this character has no {spell_id} recorded for \
             {source_class_id}"
        ));
    }

    mutate_saved_character_at_root(root, saved_at, |character_input| {
        apply_remove_spell_selection(character_input, spell_id, source_class_id);
    })
}

/// Removes one `chosen.equipment_selections` entry for `item_id`, together
/// with the equipmods applied to that entry.
///
/// Returns `false` without mutating when the character holds no such item.
///
/// **One entry, not all.** A character can legitimately carry two of the
/// same item (two daggers, one of them with a `+1` equipmod attached);
/// removing both on a request to remove one would discard a purchase the
/// player did not name. The applied modifiers go with the entry they live
/// on because that is where PCGen's `CUSTOMIZATION:EQMOD=` convention puts
/// them — they have no independent selection of their own (see
/// `apply_attach_equipment_modifier`).
///
/// **No refund.** Removal does not touch the persisted money balance.
/// Selling equipment back is a real PF1 rule with a real rate (half price
/// for most items) that nothing in this codebase models; crediting full
/// price here would invent a rule, and crediting half would invent the
/// half-price table's coverage. Dropping an item you already paid for is
/// the truthful subset. The UI states this rather than leaving the player
/// to discover it.
pub(crate) fn apply_remove_equipment_selection(
    character_input: &mut CharacterInput,
    item_id: &str,
) -> bool {
    let Some(index) = character_input
        .chosen
        .equipment_selections
        .iter()
        .position(|selection| selection.item_id.eq_ignore_ascii_case(item_id))
    else {
        return false;
    };
    character_input.chosen.equipment_selections.remove(index);
    true
}

/// `remove_equipment_selection`'s real implementation — the same
/// load -> mutate -> recompute -> re-save spine
/// `add_equipment_selection_at_root` uses.
pub(crate) fn remove_equipment_selection_at_root(
    root: &Path,
    item_id: &str,
    saved_at: &str,
) -> Result<CreateCharacterResponse, String> {
    let envelope = SavedCharacterStore::load(root).map_err(|err| err.message)?;
    let mut probe = envelope.character_input.clone();
    if !apply_remove_equipment_selection(&mut probe, item_id) {
        return Err(format!(
            "'{item_id}' cannot be removed: this character does not carry it"
        ));
    }

    mutate_saved_character_at_root(root, saved_at, |character_input| {
        apply_remove_equipment_selection(character_input, item_id);
    })
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveFeatSelectionRequest {
    pub character_id: String,
    pub feat_id: String,
    /// Which recorded target this removal takes with the feat, without its
    /// prefix — the same un-prefixed shape `AddFeatSelectionRequest.target`
    /// takes, so a caller that added Weapon Focus (Longsword) removes it by
    /// naming `"Longsword"` again.
    ///
    /// `None` removes one held copy and, if it was the last, every target
    /// that copy's chooser set still held.
    #[serde(default)]
    pub target: Option<String>,
    pub saved_at: String,
}

/// Loads the saved character, removes one held copy of the requested feat,
/// recomputes via the real engine, and re-saves — the inverse of
/// `add_feat_selection`. See `remove_feat_selection_at_root` for the
/// refusals and `feat_removal_dependency_refusal` for the prerequisite-chain
/// guard.
#[tauri::command]
pub fn remove_feat_selection(
    app: tauri::AppHandle,
    request: RemoveFeatSelectionRequest,
) -> Result<CreateCharacterResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    remove_feat_selection_at_root(
        &root,
        &request.feat_id,
        request.target.as_deref(),
        &request.saved_at,
    )
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveSpellSelectionRequest {
    pub character_id: String,
    pub spell_id: String,
    pub source_class_id: String,
    pub saved_at: String,
}

/// Loads the saved character, forgets the requested spell in every
/// acquisition mode, recomputes via the real engine, and re-saves — the
/// inverse of `add_spell_selection` / `record_and_prepare_spell_selection`.
/// See `remove_spell_selection_at_root`.
#[tauri::command]
pub fn remove_spell_selection(
    app: tauri::AppHandle,
    request: RemoveSpellSelectionRequest,
) -> Result<CreateCharacterResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    remove_spell_selection_at_root(
        &root,
        &request.spell_id,
        &request.source_class_id,
        &request.saved_at,
    )
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveEquipmentSelectionRequest {
    pub character_id: String,
    pub item_id: String,
    pub saved_at: String,
}

/// Loads the saved character, drops one carried copy of the requested item
/// (with its applied equipmods), recomputes via the real engine, and
/// re-saves — the inverse of `add_equipment_selection` / `purchase_equipment`.
/// Does **not** refund the purchase; see `apply_remove_equipment_selection`.
#[tauri::command]
pub fn remove_equipment_selection(
    app: tauri::AppHandle,
    request: RemoveEquipmentSelectionRequest,
) -> Result<CreateCharacterResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    remove_equipment_selection_at_root(&root, &request.item_id, &request.saved_at)
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSkillAllocationsRequest {
    pub character_id: String,
    /// The caller's complete skill-allocation set (not a delta) — reuses
    /// the `SkillAllocationDto` shape already established for
    /// import/export so the wire contract stays consistent across this
    /// module's DTOs.
    pub skill_allocations: Vec<SkillAllocationDto>,
    pub saved_at: String,
}

/// Loads the saved character, replaces its skill allocations wholesale,
/// recomputes via the real engine, and re-saves — see
/// `set_skill_allocations_at_root` for the full semantics. Replaces rather
/// than appends because `SkillAllocationDialog.onAccept` always sends its
/// complete draft allocation, not an incremental change.
#[tauri::command]
pub fn set_skill_allocations(
    app: tauri::AppHandle,
    request: SetSkillAllocationsRequest,
) -> Result<CreateCharacterResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    let skill_allocations = request
        .skill_allocations
        .into_iter()
        .map(|skill| SkillAllocation {
            skill_id: skill.skill_id,
            ranks: skill.ranks,
        })
        .collect();
    set_skill_allocations_at_root(&root, skill_allocations, &request.saved_at)
}

fn resolve_characters_root(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|err| format!("could not resolve app data directory: {err}"))?;
    Ok(characters_root_from_app_data_dir(&app_data_dir))
}

/// `pub(crate)` (rather than private) so the `characterHub` submodule's
/// commands (e.g. `appendToCharacter` — SD-24 Epic 7, Criterion 7.1) can
/// resolve the same on-disk character root this module's own commands use,
/// without re-deriving the app-data-dir resolution a second time.
pub(crate) fn resolve_character_root(
    app: &tauri::AppHandle,
    character_id: &str,
) -> Result<PathBuf, String> {
    Ok(resolve_characters_root(app)?.join(character_id))
}

const PORTRAIT_FILE_NAME: &str = "portrait.png";
// The frontend crops/resizes to a small fixed square before ever sending
// bytes here; this is a defensive backstop against a buggy or malicious
// caller, not the primary size control.
const MAX_PORTRAIT_BYTES: usize = 3 * 1024 * 1024;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveCharacterPortraitRequest {
    pub character_id: String,
    /// PNG image bytes, base64-encoded (no `data:` URL prefix).
    pub image_base64: String,
}

/// Persists a character portrait as `portrait.png` alongside the character's
/// existing envelope/input files. Requires the character to already be
/// saved — a portrait is never the first write to a character directory.
#[tauri::command]
pub fn save_character_portrait(
    app: tauri::AppHandle,
    request: SaveCharacterPortraitRequest,
) -> Result<(), String> {
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(request.image_base64.as_bytes())
        .map_err(|err| format!("invalid base64 image data: {err}"))?;
    if bytes.len() > MAX_PORTRAIT_BYTES {
        return Err(format!(
            "portrait image is {} bytes, over the {MAX_PORTRAIT_BYTES} byte limit",
            bytes.len()
        ));
    }

    let root = resolve_character_root(&app, &request.character_id)?;
    if !root.exists() {
        return Err(format!(
            "no saved character found for id {}",
            request.character_id
        ));
    }

    let path = root.join(PORTRAIT_FILE_NAME);
    std::fs::write(&path, &bytes).map_err(|err| format!("{}: {err}", path.display()))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportCharacterJsonRequest {
    /// Destination path chosen by the frontend via the dialog plugin's
    /// `save()` picker — this command never resolves its own path.
    pub file_path: String,
    pub contents: String,
}

/// Writes arbitrary JSON text (the character export payload) to a path the
/// user picked themselves. Unlike the other character-hub commands, the
/// destination is outside the app's own data directory, so it's taken as-is
/// rather than resolved from a character id.
#[tauri::command]
pub fn export_character_json(request: ExportCharacterJsonRequest) -> Result<(), String> {
    let path = PathBuf::from(&request.file_path);
    std::fs::write(&path, request.contents.as_bytes()).map_err(|err| format!("{}: {err}", path.display()))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadCharacterPortraitRequest {
    pub character_id: String,
}

/// Returns the character's portrait as a `data:image/png;base64,...` URL, or
/// `None` if no portrait has been uploaded for this character.
#[tauri::command]
pub fn load_character_portrait(
    app: tauri::AppHandle,
    request: LoadCharacterPortraitRequest,
) -> Result<Option<String>, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    let path = root.join(PORTRAIT_FILE_NAME);
    if !path.exists() {
        return Ok(None);
    }

    let bytes = std::fs::read(&path).map_err(|err| format!("{}: {err}", path.display()))?;
    let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
    Ok(Some(format!("data:image/png;base64,{encoded}")))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCharacterPortraitRequest {
    pub character_id: String,
}

#[tauri::command]
pub fn delete_character_portrait(
    app: tauri::AppHandle,
    request: DeleteCharacterPortraitRequest,
) -> Result<(), String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    let path = root.join(PORTRAIT_FILE_NAME);
    if path.exists() {
        std::fs::remove_file(&path).map_err(|err| format!("{}: {err}", path.display()))?;
    }
    Ok(())
}

// ----- `update_character_bio` / `load_character_bio` (v0.6 alpha swarm) -----
//
// Bio fields (alignment/deity/sex/age/height/weight/hair/eyes) are pure
// display flavor text -- no rules-engine calculation reads any of them.
// Rather than adding a field to `ChosenCharacterState` (constructed as a
// struct literal at ~70 call sites across this crate's own test suite,
// mostly qa-owned `tests/**` -- a schema change there would break all of
// them for data the compute engine never touches), bio is persisted as its
// own sidecar file (`bio.json`) alongside the character's existing
// envelope/input files, mirroring `save_character_portrait`/
// `load_character_portrait`/`delete_character_portrait`'s own established
// sidecar-file precedent (`portrait.png`) exactly -- same directory, same
// "requires the character to already be saved" invariant, same shape.

const BIO_FILE_NAME: &str = "bio.json";

/// One character's bio/flavor fields. Every field defaults to an empty
/// string (`#[serde(default)]`) so a bio file written before a future field
/// is added, or a character with no bio file at all, still deserializes
/// (via `Default`) rather than failing to load.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterBioDto {
    #[serde(default)]
    pub alignment: String,
    #[serde(default)]
    pub deity: String,
    #[serde(default)]
    pub sex: String,
    #[serde(default)]
    pub age: String,
    #[serde(default)]
    pub height: String,
    #[serde(default)]
    pub weight: String,
    #[serde(default)]
    pub hair: String,
    #[serde(default)]
    pub eyes: String,
}

/// Writes `bio` as `bio.json` in the character's root directory. Requires
/// the character to already be saved -- mirrors
/// `save_character_portrait`'s own "a portrait is never the first write to
/// a character directory" invariant. Checks via
/// `SavedCharacterStore::load` (not merely `root.exists()` -- the
/// characters-root directory itself may already exist without a saved
/// envelope in it, e.g. under this function's own test fixtures, whose
/// `tempdir` helper always creates the directory) so a bio is never the
/// first write to a character directory either. Split from the
/// `#[tauri::command]` wrapper below so it is unit-testable without an
/// `AppHandle`.
fn save_character_bio_at_root(root: &Path, bio: &CharacterBioDto) -> Result<(), String> {
    SavedCharacterStore::load(root).map_err(|err| err.message)?;
    let path = root.join(BIO_FILE_NAME);
    let json = serde_json::to_string_pretty(bio)
        .map_err(|err| format!("failed to serialize character bio: {err}"))?;
    std::fs::write(&path, json).map_err(|err| format!("{}: {err}", path.display()))
}

/// Reads `bio.json` from the character's root directory, or an all-empty
/// `CharacterBioDto::default()` when no bio has ever been saved for this
/// character -- never an error for the common "no bio yet" case, matching
/// `load_character_portrait`'s own `Ok(None)`-for-absent shape (bio's
/// equivalent "absent" value is the default-empty DTO, since every field is
/// already optional-shaped as an empty string rather than an `Option`).
fn load_character_bio_at_root(root: &Path) -> Result<CharacterBioDto, String> {
    let path = root.join(BIO_FILE_NAME);
    if !path.exists() {
        return Ok(CharacterBioDto::default());
    }
    let contents = std::fs::read_to_string(&path).map_err(|err| format!("{}: {err}", path.display()))?;
    serde_json::from_str(&contents).map_err(|err| format!("{}: invalid bio JSON: {err}", path.display()))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCharacterBioRequest {
    pub character_id: String,
    pub bio: CharacterBioDto,
}

/// Persists the caller's full bio field set as `bio.json` alongside the
/// character's existing saved files. Requires the character to already be
/// saved. Always the character's *complete* bio (not a delta) -- the
/// frontend's bio editor already holds every field's current value, so it
/// always sends the full set on save.
#[tauri::command]
pub fn update_character_bio(
    app: tauri::AppHandle,
    request: UpdateCharacterBioRequest,
) -> Result<(), String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    save_character_bio_at_root(&root, &request.bio)
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadCharacterBioRequest {
    pub character_id: String,
}

/// Reads the character's persisted bio, or an all-empty `CharacterBioDto`
/// when none has been saved yet (including when the character itself does
/// not exist -- resolving a nonexistent root still yields a real path, and
/// reading a bio file that isn't there is the same "nothing saved yet"
/// case either way, so this command does not separately error on a missing
/// character the way the mutation commands do).
#[tauri::command]
pub fn load_character_bio(
    app: tauri::AppHandle,
    request: LoadCharacterBioRequest,
) -> Result<CharacterBioDto, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    load_character_bio_at_root(&root)
}

// ----- `load_character_money` / `adjust_character_money` (v0.6 alpha swarm) -----
//
// Persisted the same sidecar-file way as bio (a `money.json` file, not a
// `ChosenCharacterState` field) -- the alpha bar's "money conversion" calc
// is the denomination-conversion math itself (`codex::rules_core::money`),
// which needs a canonical balance to convert, not a rules-engine-visible
// character-build field the way skill/feat/equipment selections are. Only
// the canonical `total_copper` is ever persisted; the pp/gp/sp/cp
// breakdown in `CharacterMoneyDto` is always derived fresh from it via
// `money::copper_to_denominations`, never stored redundantly (matching
// `money.rs`'s own "never two numbers that could drift apart" doc
// comment).

const MONEY_FILE_NAME: &str = "money.json";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StoredMoney {
    #[serde(default)]
    total_copper: u64,
}

/// The wire response for both money commands: the canonical
/// `total_copper` balance plus its derived platinum/gold/silver/copper
/// breakdown, so the frontend never re-implements the conversion math.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterMoneyDto {
    pub total_copper: u64,
    pub platinum: u64,
    pub gold: u64,
    pub silver: u64,
    pub copper: u64,
}

fn money_dto_from_total(total_copper: u64) -> CharacterMoneyDto {
    let denominations = money::copper_to_denominations(total_copper);
    CharacterMoneyDto {
        total_copper,
        platinum: denominations.platinum,
        gold: denominations.gold,
        silver: denominations.silver,
        copper: denominations.copper,
    }
}

/// Reads the character's persisted money balance, or a zero balance when
/// no `money.json` has ever been saved for this character -- mirrors
/// `load_character_bio_at_root`'s own "no error for the common absent
/// case" shape.
fn load_character_money_at_root(root: &Path) -> Result<CharacterMoneyDto, String> {
    let path = root.join(MONEY_FILE_NAME);
    if !path.exists() {
        return Ok(money_dto_from_total(0));
    }
    let contents = std::fs::read_to_string(&path).map_err(|err| format!("{}: {err}", path.display()))?;
    let stored: StoredMoney = serde_json::from_str(&contents)
        .map_err(|err| format!("{}: invalid money JSON: {err}", path.display()))?;
    Ok(money_dto_from_total(stored.total_copper))
}

/// Applies `delta_copper` (positive to add funds, negative to spend) to
/// the character's persisted balance and returns the new total's
/// denomination breakdown. Requires the character to already be saved
/// (checked via `SavedCharacterStore::load`, same as `save_character_bio_at_root`).
/// Fails honestly with an insufficient-funds error rather than silently
/// allowing a negative balance -- PF1 characters cannot carry negative
/// money.
fn adjust_character_money_at_root(root: &Path, delta_copper: i64) -> Result<CharacterMoneyDto, String> {
    SavedCharacterStore::load(root).map_err(|err| err.message)?;

    let current_total_copper = load_character_money_at_root(root)?.total_copper;
    let new_total = i64::try_from(current_total_copper)
        .map_err(|_| "current balance overflows a signed 64-bit total".to_owned())?
        + delta_copper;
    if new_total < 0 {
        return Err(format!(
            "insufficient funds: balance is {current_total_copper} cp, requested change is \
             {delta_copper} cp"
        ));
    }
    let new_total_copper = new_total as u64;

    let path = root.join(MONEY_FILE_NAME);
    let json = serde_json::to_string_pretty(&StoredMoney { total_copper: new_total_copper })
        .map_err(|err| format!("failed to serialize character money: {err}"))?;
    std::fs::write(&path, json).map_err(|err| format!("{}: {err}", path.display()))?;

    Ok(money_dto_from_total(new_total_copper))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadCharacterMoneyRequest {
    pub character_id: String,
}

#[tauri::command]
pub fn load_character_money(
    app: tauri::AppHandle,
    request: LoadCharacterMoneyRequest,
) -> Result<CharacterMoneyDto, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    load_character_money_at_root(&root)
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustCharacterMoneyRequest {
    pub character_id: String,
    /// Positive to add funds (e.g. selling an item, starting gold),
    /// negative to spend (e.g. buying equipment at its `cost_gp`, converted
    /// via `money::gp_to_copper`).
    pub delta_copper: i64,
}

#[tauri::command]
pub fn adjust_character_money(
    app: tauri::AppHandle,
    request: AdjustCharacterMoneyRequest,
) -> Result<CharacterMoneyDto, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    adjust_character_money_at_root(&root, request.delta_copper)
}

// ----- `load_character_durability` / `adjust_character_hp` (v0.6 alpha swarm) -----
//
// Max HP is a real, derived-from-the-build value (`durability::compute_max_hp`,
// scoped to single-class Fighter/Wizard/Rogue -- see that module's own doc
// comment for why multiclass is honestly out of scope rather than guessed
// at). Current HP / nonlethal damage are live-tracking values persisted as
// a `hp.json` sidecar (same pattern as bio/money): `current_hp` defaults to
// the computed `max_hp` the first time a character is loaded (no file on
// disk yet), then only ever changes via `adjust_character_hp`.

const HP_FILE_NAME: &str = "hp.json";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StoredHp {
    /// `None` (the field absent, or no file at all) means "never
    /// initialized yet" -- distinct from `Some(0)`, a real character at 0
    /// HP. Defaulted to the computed `max_hp` on first load, not before.
    current_hp: Option<i16>,
    #[serde(default)]
    nonlethal_damage: i16,
}

fn durability_status_label(status: DurabilityStatus) -> &'static str {
    match status {
        DurabilityStatus::Normal => "Normal",
        DurabilityStatus::Staggered => "Staggered",
        DurabilityStatus::Disabled => "Disabled",
        DurabilityStatus::Unconscious => "Unconscious",
        DurabilityStatus::Dying => "Dying",
        DurabilityStatus::Dead => "Dead",
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterDurabilityDto {
    pub max_hp: i16,
    pub current_hp: i16,
    pub nonlethal_damage: i16,
    /// One of `"Normal"` / `"Staggered"` / `"Disabled"` / `"Unconscious"` /
    /// `"Dying"` / `"Dead"` -- see `durability::classify_durability`'s own
    /// doc comment for the exact threshold rules.
    pub status: String,
}

/// Computes `max_hp` from the saved character's real build (class levels +
/// effective, racial-bonus-aware Constitution score, same
/// `apply_human_ability_bonus` reuse `encumbrance`'s wiring in
/// `contract.rs` already established) and reads persisted current-HP/
/// nonlethal-damage from `hp.json`, defaulting `current_hp` to the freshly
/// computed `max_hp` when no file exists yet. Fails honestly (rather than
/// fabricating a value) when the build is multiclass or an unsupported
/// class -- `compute_max_hp` returns `None` for exactly those cases.
fn load_character_durability_at_root(root: &Path) -> Result<CharacterDurabilityDto, String> {
    let envelope = SavedCharacterStore::load(root).map_err(|err| err.message)?;

    let mut discarded_explanations = Vec::new();
    let effective_scores = apply_human_ability_bonus(&envelope.character_input, &mut discarded_explanations);
    let constitution_score = effective_scores.constitution;
    let constitution_modifier = ability_modifier(constitution_score);

    let base_max_hp =
        compute_max_hp(&envelope.character_input.chosen.class_levels, constitution_modifier)
            .ok_or_else(|| {
                "durability is only computed for a single-class Fighter, Wizard, or Rogue build \
                 today; this character's class levels are not one of those"
                    .to_owned()
            })?;
    // v0.6 alpha swarm item 17 (feat-effects engine): a grounded feat's real
    // hit-point bonus (currently just Toughness's flat +3) is added on top
    // of the class/Constitution-derived base, not folded into
    // compute_max_hp itself -- feat effects are a per-character add-on, not
    // part of the class hit-die table durability.rs owns.
    // v0.6 alpha swarm task #11 Tier 0 (2026-07-27): a bonded Toad
    // familiar's real +3 maximum hit points layers on here too, the same
    // per-character add-on shape as the feat bonus above -- deliberately
    // not folded into compute_max_hp, which owns only the class hit-die
    // table.
    let max_hp = base_max_hp
        + feat_effects::hp_bonus_from_feats(&envelope.character_input.chosen.selected_feats)
        + codex::rules_core::pilot_compute::character_familiar_hp_bonus(
            &envelope.character_input,
        );

    let path = root.join(HP_FILE_NAME);
    let stored: StoredHp = if path.exists() {
        let contents = std::fs::read_to_string(&path).map_err(|err| format!("{}: {err}", path.display()))?;
        serde_json::from_str(&contents)
            .map_err(|err| format!("{}: invalid hp JSON: {err}", path.display()))?
    } else {
        StoredHp::default()
    };
    let current_hp = stored.current_hp.unwrap_or(max_hp);
    let status = classify_durability(current_hp, stored.nonlethal_damage, constitution_score);

    Ok(CharacterDurabilityDto {
        max_hp,
        current_hp,
        nonlethal_damage: stored.nonlethal_damage,
        status: durability_status_label(status).to_owned(),
    })
}

/// Applies `delta_hp` (positive to heal, negative to take lethal damage)
/// and/or `delta_nonlethal` (positive to take nonlethal damage, negative to
/// recover from it) to the character's persisted HP state, clamping
/// `current_hp` at the computed `max_hp` ceiling (healing cannot exceed
/// max) and `nonlethal_damage` at a floor of 0 (cannot recover past no
/// nonlethal damage). `current_hp` is allowed to go negative (dying/dead is
/// a real, trackable state, not an error) but `adjust_character_hp` still
/// requires the character to already be saved and be a durability-
/// supported build, same as `load_character_durability_at_root`.
fn adjust_character_hp_at_root(
    root: &Path,
    delta_hp: i16,
    delta_nonlethal: i16,
) -> Result<CharacterDurabilityDto, String> {
    let current = load_character_durability_at_root(root)?;

    let new_current_hp = (current.current_hp + delta_hp).min(current.max_hp);
    let new_nonlethal_damage = (current.nonlethal_damage + delta_nonlethal).max(0);

    let path = root.join(HP_FILE_NAME);
    let json = serde_json::to_string_pretty(&StoredHp {
        current_hp: Some(new_current_hp),
        nonlethal_damage: new_nonlethal_damage,
    })
    .map_err(|err| format!("failed to serialize character hp: {err}"))?;
    std::fs::write(&path, json).map_err(|err| format!("{}: {err}", path.display()))?;

    load_character_durability_at_root(root)
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadCharacterDurabilityRequest {
    pub character_id: String,
}

#[tauri::command]
pub fn load_character_durability(
    app: tauri::AppHandle,
    request: LoadCharacterDurabilityRequest,
) -> Result<CharacterDurabilityDto, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    load_character_durability_at_root(&root)
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustCharacterHpRequest {
    pub character_id: String,
    /// Positive to heal, negative to take lethal damage.
    #[serde(default)]
    pub delta_hp: i16,
    /// Positive to take nonlethal damage, negative to recover from it.
    #[serde(default)]
    pub delta_nonlethal: i16,
}

#[tauri::command]
pub fn adjust_character_hp(
    app: tauri::AppHandle,
    request: AdjustCharacterHpRequest,
) -> Result<CharacterDurabilityDto, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    adjust_character_hp_at_root(&root, request.delta_hp, request.delta_nonlethal)
}

// ----- `delete_character` (Storage Tier Minimal Fix, Criterion 22) -----

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCharacterRequest {
    pub character_id: String,
}

/// The wire response for `delete_character`. Unlike every other
/// character-hub command, a failure is carried inside the payload
/// (`ok: false`, `error: Some(..)`) rather than raised as a rejected Tauri
/// IPC call — this matches the criterion's literal `{ ok, error? }` return
/// contract, so the Load Character screen can show an inline status message
/// without a try/catch.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCharacterResponse {
    pub ok: bool,
    pub error: Option<String>,
}

/// Removes the saved character's on-disk directory tree at `root`.
///
/// Deleting a character that has no saved directory (already deleted, or
/// never existed) is a deliberate `ok: true` — see
/// `SavedCharacterStore::delete`'s own doc comment for the idempotent-delete
/// rationale (mirrors this module's existing `delete_character_portrait`
/// idiom of not erroring when there is nothing to remove). Any other
/// failure (I/O error, permissions) is surfaced honestly as `ok: false`
/// with a real message, never silently swallowed.
///
/// Split from the `#[tauri::command]` wrapper below so it is unit-testable
/// against a real `SavedCharacterStore` fixture without an `AppHandle` —
/// mirrors every other `_at_root` function in this module.
fn delete_character_at_root(root: &Path) -> DeleteCharacterResponse {
    match SavedCharacterStore::delete(root) {
        Ok(()) => DeleteCharacterResponse { ok: true, error: None },
        Err(err) => DeleteCharacterResponse {
            ok: false,
            error: Some(err.message),
        },
    }
}

/// Resolves the character's root directory and deletes it — see
/// `delete_character_at_root` for the full semantics.
#[tauri::command]
pub fn delete_character(
    app: tauri::AppHandle,
    request: DeleteCharacterRequest,
) -> DeleteCharacterResponse {
    let root = match resolve_character_root(&app, &request.character_id) {
        Ok(root) => root,
        Err(err) => return DeleteCharacterResponse {
            ok: false,
            error: Some(err),
        },
    };
    delete_character_at_root(&root)
}

// ----- `import_character` / `export_character` (Storage Tier Minimal Fix,
// Criteria 23-24) -----
//
// Manual re-projection of `CharacterInput`'s full shape into DTOs, for the
// same reason as this module's top-of-file note: the `codex` crate has zero
// dependencies, so `CharacterInput` itself is neither `Serialize` nor
// `Deserialize`. These DTOs derive both: `import_character` deserializes an
// `ImportedCharacterFileDto` from the picked file's JSON, and
// `export_character` serializes one back out — the exact same wire shape in
// both directions, so a file this module exports is always a file it can
// import (Criterion 24's export/import round-trip fix). Field names and wire
// values (e.g. `ActiveStateDto`'s `"EquippedActive"`/`"Absent"`/
// `"SelectedInactive"`, `AcquisitionModeDto`'s `"Known"`/`"Prepared"`/
// `"Granted"`) reuse the exact DTOs already established for
// `add_equipment_selection`/`add_spell_selection`, so the import/export JSON
// shape is consistent with the rest of this module's wire contract rather
// than a parallel invention.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterClassLevelDto {
    pub class_id: String,
    pub level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillAllocationDto {
    pub skill_id: String,
    pub ranks: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentSelectionImportDto {
    pub item_id: String,
    pub active_state: ActiveStateDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectedChoiceDto {
    pub choice_set_id: String,
    pub selection_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpellSelectionImportDto {
    pub spell_id: String,
    pub source_class_id: String,
    pub acquisition_mode: AcquisitionModeDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChosenCharacterStateDto {
    pub race_id: String,
    pub class_levels: Vec<CharacterClassLevelDto>,
    pub ability_scores: AbilityScoresDto,
    #[serde(default)]
    pub selected_feats: Vec<String>,
    #[serde(default)]
    pub skill_allocations: Vec<SkillAllocationDto>,
    #[serde(default)]
    pub equipment_selections: Vec<EquipmentSelectionImportDto>,
    #[serde(default)]
    pub selected_choices: Vec<SelectedChoiceDto>,
    /// **AT-34-E4-002**: `trait_effects` wire ids
    /// (`"trait:trait_acrobat"`). `#[serde(default)]` so an import file
    /// exported before this field existed keeps importing unchanged.
    #[serde(default)]
    pub selected_traits: Vec<String>,
    #[serde(default)]
    pub spells_selected: Vec<SpellSelectionImportDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterInputDto {
    pub source_package_id: String,
    pub chosen: ChosenCharacterStateDto,
}

/// The on-disk/exported shape `import_character` expects (and
/// `export_character` produces): a `displayLabel` + `characterInput` (the
/// fields needed to rebuild a real `CharacterInput`) — the same two fields a
/// full `SavedCharacterEnvelope` export carries. Every other envelope field
/// (`characterId`, `revisionId`, `savedAt`, ...) is intentionally NOT part of
/// this DTO: importing always mints a fresh identity/revision/timestamp
/// rather than trusting the source file's own claimed identity (mirrors
/// `clone_character`'s own "never blindly trust source identity" stance).
/// `serde` ignores any extra fields the source JSON carries on import, so a
/// real full-envelope export file is tolerated rather than rejected for
/// having "extra" fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportedCharacterFileDto {
    pub display_label: String,
    pub character_input: CharacterInputDto,
}

fn character_input_from_dto(dto: CharacterInputDto, fresh_character_id: &str) -> CharacterInput {
    CharacterInput {
        case_id: Some(fresh_character_id.to_owned()),
        source_package_id: dto.source_package_id,
        chosen: ChosenCharacterState {
            race_id: dto.chosen.race_id,
            class_levels: dto
                .chosen
                .class_levels
                .into_iter()
                .map(|class_level| CharacterClassLevel {
                    class_id: class_level.class_id,
                    level: class_level.level,
                })
                .collect(),
            ability_scores: AbilityScores {
                strength: dto.chosen.ability_scores.strength,
                dexterity: dto.chosen.ability_scores.dexterity,
                constitution: dto.chosen.ability_scores.constitution,
                intelligence: dto.chosen.ability_scores.intelligence,
                wisdom: dto.chosen.ability_scores.wisdom,
                charisma: dto.chosen.ability_scores.charisma,
            },
            selected_feats: dto.chosen.selected_feats,
            skill_allocations: dto
                .chosen
                .skill_allocations
                .into_iter()
                .map(|skill| SkillAllocation {
                    skill_id: skill.skill_id,
                    ranks: skill.ranks,
                })
                .collect(),
            equipment_selections: dto
                .chosen
                .equipment_selections
                .into_iter()
                .map(|equipment| EquipmentSelection {
                    item_id: equipment.item_id,
                    equipped_or_active: matches!(
                        equipment.active_state,
                        ActiveStateDto::EquippedActive
                    ),
                    active_state: equipment.active_state.into(),
                    applied_modifiers: Vec::new(),
                })
                .collect(),
            selected_choices: dto
                .chosen
                .selected_choices
                .into_iter()
                .map(|choice| SelectedChoice {
                    choice_set_id: choice.choice_set_id,
                    selection_id: choice.selection_id,
                })
                .collect(),
            selected_traits: dto.chosen.selected_traits,
            spells_selected: dto
                .chosen
                .spells_selected
                .into_iter()
                .map(|spell| SpellSelection {
                    spell_id: spell.spell_id,
                    source_class_id: spell.source_class_id,
                    acquisition_mode: spell.acquisition_mode.into(),
                })
                .collect(),
            // v0.6 alpha swarm, risks item 8 (combat-time activation state):
            // no DTO field exists for this yet -- out of scope for this
            // backend-only slice, mirroring how `applied_modifiers` just
            // above is also not yet carried through the import DTO.
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

/// Reverse of `character_input_from_dto` — projects a real `CharacterInput`
/// into the wire DTO `export_character` serializes. `case_id` and
/// `selection_provenance` are intentionally NOT carried into the DTO: they
/// have no field in `CharacterInputDto` (mirroring `character_input_from_dto`
/// always minting its own `case_id` and discarding `selection_provenance` on
/// import), so a round trip through export -> import always ends with a
/// fresh identity, never the source character's own.
fn character_input_to_dto(input: &CharacterInput) -> CharacterInputDto {
    CharacterInputDto {
        source_package_id: input.source_package_id.clone(),
        chosen: ChosenCharacterStateDto {
            race_id: input.chosen.race_id.clone(),
            class_levels: input
                .chosen
                .class_levels
                .iter()
                .map(|class_level| CharacterClassLevelDto {
                    class_id: class_level.class_id.clone(),
                    level: class_level.level,
                })
                .collect(),
            ability_scores: AbilityScoresDto {
                strength: input.chosen.ability_scores.strength,
                dexterity: input.chosen.ability_scores.dexterity,
                constitution: input.chosen.ability_scores.constitution,
                intelligence: input.chosen.ability_scores.intelligence,
                wisdom: input.chosen.ability_scores.wisdom,
                charisma: input.chosen.ability_scores.charisma,
            },
            selected_feats: input.chosen.selected_feats.clone(),
            skill_allocations: input
                .chosen
                .skill_allocations
                .iter()
                .map(|skill| SkillAllocationDto {
                    skill_id: skill.skill_id.clone(),
                    ranks: skill.ranks,
                })
                .collect(),
            equipment_selections: input
                .chosen
                .equipment_selections
                .iter()
                .map(|equipment| EquipmentSelectionImportDto {
                    item_id: equipment.item_id.clone(),
                    active_state: equipment.active_state.into(),
                })
                .collect(),
            selected_choices: input
                .chosen
                .selected_choices
                .iter()
                .map(|choice| SelectedChoiceDto {
                    choice_set_id: choice.choice_set_id.clone(),
                    selection_id: choice.selection_id.clone(),
                })
                .collect(),
            selected_traits: input.chosen.selected_traits.clone(),
            spells_selected: input
                .chosen
                .spells_selected
                .iter()
                .map(|spell| SpellSelectionImportDto {
                    spell_id: spell.spell_id.clone(),
                    source_class_id: spell.source_class_id.clone(),
                    acquisition_mode: spell.acquisition_mode.into(),
                })
                .collect(),
        },
    }
}

/// Parses `contents` as an `ImportedCharacterFileDto`, converts it to a real
/// `CharacterInput` under `fresh_character_id`, recomputes via the real
/// engine, and — mirroring `create_character`/`clone_character`/
/// `level_up_character_at_root`'s own "never persist an unproven build"
/// invariant — only saves and returns `Saved` if the import reaches
/// `Computed`; a structurally valid import the engine cannot compute
/// returns `Blocked` with real diagnostics rather than silently writing a
/// broken build. Malformed or schema-invalid JSON (parse failure) is a
/// distinct failure mode from `Blocked` and is surfaced as a real `Err`,
/// never papered over as an empty/successful import.
///
/// Split from the `#[tauri::command]` wrapper below so it is unit-testable
/// against a real `SavedCharacterStore` fixture without an `AppHandle` or a
/// real file on disk — mirrors every other `_at_root`/`_from_*` function in
/// this module.
fn import_character_from_json(
    contents: &str,
    root: &Path,
    fresh_character_id: &str,
    saved_at: &str,
    app_version: &str,
) -> Result<CreateCharacterResponse, String> {
    let parsed: ImportedCharacterFileDto = serde_json::from_str(contents)
        .map_err(|err| format!("invalid character import JSON: {err}"))?;

    let character_input = character_input_from_dto(parsed.character_input, fresh_character_id);

    let (snapshot, corpus_receipt) =
        match resolve_unified_pilot_snapshot(&character_input, corpus_fixture_bundle()) {
            Ok(result) => result,
            Err(diagnostics) => {
                return Ok(CreateCharacterResponse::Blocked {
                    diagnostics: map_diagnostics_dto(&diagnostics),
                });
            }
        };

    let envelope = SavedCharacterEnvelope {
        character_id: fresh_character_id.to_owned(),
        revision_id: format!("{fresh_character_id}.rev.1"),
        revision_kind: SavedCharacterRevisionKind::Authoritative,
        saved_at: saved_at.to_owned(),
        schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
        app_or_runtime_version: app_version.to_owned(),
        content_or_rules_provenance: SOURCE_PACKAGE_ID.to_owned(),
        game_system: GAME_SYSTEM_ID.to_owned(),
        latest_authoritative_revision_ref: format!("{fresh_character_id}.rev.1"),
        display_label: parsed.display_label,
        character_input,
    };

    SavedCharacterStore::save(&envelope, root).map_err(|err| err.message)?;

    Ok(CreateCharacterResponse::Saved {
        summary: Box::new(summarize_envelope(&envelope)),
        snapshot: map_snapshot_dto(&snapshot),
        corpus_derived: map_corpus_derived_dto(&corpus_receipt.corpus_derived),
    })
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportCharacterRequest {
    /// Path to a JSON file shaped like `ImportedCharacterFileDto` (a real
    /// `displayLabel` + `characterInput`; extra full-envelope fields are
    /// tolerated and ignored). This command reads the file itself — the
    /// caller (the file-open dialog) only needs to supply the path, mirroring
    /// how `export_character_json`'s `file_path` names a destination it
    /// writes to directly.
    pub file_path: String,
    pub saved_at: String,
}

/// Reads `file_path`, mints a fresh `character_id` (a v4 UUID, matching the
/// shape the frontend's own `crypto.randomUUID()` ids already use for
/// `create_character`), and imports it — see `import_character_from_json`
/// for the full parse -> recompute -> save semantics.
#[tauri::command]
pub fn import_character(
    app: tauri::AppHandle,
    request: ImportCharacterRequest,
) -> Result<CreateCharacterResponse, String> {
    let contents = std::fs::read_to_string(&request.file_path)
        .map_err(|err| format!("{}: {err}", request.file_path))?;

    let fresh_character_id = Uuid::new_v4().to_string();
    let root = resolve_character_root(&app, &fresh_character_id)?;
    let app_version = app.package_info().version.to_string();

    import_character_from_json(
        &contents,
        &root,
        &fresh_character_id,
        &request.saved_at,
        &app_version,
    )
}

// ----- `export_character` (Storage Tier Minimal Fix, Criterion 24) -----

/// Loads the saved character at `root` and serializes it into the exact
/// `ImportedCharacterFileDto` shape `import_character_from_json` parses —
/// this is what makes Export -> Import a real round trip rather than the
/// old lossy `{summary, detail}` export payload (see this module's own
/// history: the Load Character screen used to build its export payload from
/// computed summary/snapshot data, which carries no raw `CharacterInput` and
/// so could never be re-imported).
///
/// Split from the `#[tauri::command]` wrapper below so it is unit-testable
/// against a real `SavedCharacterStore` fixture without an `AppHandle` —
/// mirrors every other `_at_root`/`_from_*`/`_to_*` function in this module.
fn export_character_to_json(root: &Path) -> Result<String, String> {
    let envelope = SavedCharacterStore::load(root).map_err(|err| err.message)?;

    let dto = ImportedCharacterFileDto {
        display_label: envelope.display_label,
        character_input: character_input_to_dto(&envelope.character_input),
    };

    serde_json::to_string_pretty(&dto).map_err(|err| format!("failed to serialize character for export: {err}"))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportCharacterRequest {
    pub character_id: String,
    /// Destination path chosen by the frontend via the dialog plugin's
    /// `save()` picker, mirroring `export_character_json`'s own `file_path`
    /// (this command writes directly, same as that one).
    pub file_path: String,
}

/// Resolves the character's root directory, serializes its real saved build
/// into the `ImportedCharacterFileDto` shape, and writes it to `file_path` —
/// see `export_character_to_json` for the full semantics. Unlike
/// `export_character_json` (which writes whatever `contents` string the
/// caller already built), this command builds the export payload itself
/// from the real on-disk envelope, so what it writes is always importable.
#[tauri::command]
pub fn export_character(app: tauri::AppHandle, request: ExportCharacterRequest) -> Result<(), String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    let contents = export_character_to_json(&root)?;

    let path = PathBuf::from(&request.file_path);
    std::fs::write(&path, contents.as_bytes()).map_err(|err| format!("{}: {err}", path.display()))
}

// ---------------------------------------------------------------------------
// Race-creation roster
// ---------------------------------------------------------------------------

/// The per-race chassis character creation needs, read out of the real race
/// corpus rather than hand-maintained beside it.
///
/// # Why this exists
///
/// `RACE_OPTIONS` in `characterHubModel.ts` was a hand-written table of the
/// 7 Core Rulebook races. The corpus carries **18** (CRB's 7 + Bestiary 1's
/// 11), every one of them with a complete creation chassis — asserted field
/// by field, per race, by `raceCreationCoverage.test.ts` against the same
/// on-disk records this reads. The 11 Bestiary 1 races were ingested,
/// resolvable, and browsable in the Race Trait Catalog, but no player could
/// make one.
///
/// A hand-maintained mirror of corpus facts is also how the identical table
/// one layer down (`rules_tables::crb::race_tables`) silently drifted from
/// the corpus on four races' ability modifiers: `BONUS:STAT|CON,WIS|2`
/// states two ability grants in one token and a transcription read only up
/// to the comma. Deriving removes the class of defect rather than re-checking
/// for it.
///
/// # What is derived, and why that is not formula interpretation
///
/// `decisions.md §24` forbids a general `BONUS:`/`DEFINE:`/`PREREQ:` formula
/// interpreter and requires each feature to be a hand-modelled,
/// corpus-verified pure function with a test. Every field below is exactly
/// that: `codex::rules_core::race_creation`'s `fixed_ability_adjustments`
/// reads the ability codes and magnitude off a `BONUS:STAT` chain's own
/// qualifiers, its `vision_reading` reads a `VISION:` token's own declared
/// range, size and speed come from
/// [`ResolvedRace`]'s already-modelled chassis-then-trait-override rule.
/// Nothing is summed across traits, no PCGen variable is resolved, and no
/// `PREREQ:` is evaluated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaceCreationChassisDto {
    /// The `race:<slug>` token the rest of the engine identifies a race by —
    /// what `CreateCharacterRequest::race_id` carries, what
    /// `compose_character_input` threads into `CharacterInput`, and what
    /// `race_resolver::race_size_for_race_token` resolves for carrying
    /// capacity. Derived as the corpus race key lowercased, which reproduces
    /// the 7 previously hardcoded ids exactly (`Half-Elf` -> `race:half-elf`).
    pub race_id: String,
    /// The corpus race key verbatim, e.g. `Half-Elf`, `Svirfneblin`.
    pub label: String,
    /// The short book code, from `race_catalog::book_code` so this cannot
    /// drift from the codes the Race Trait Catalog labels the same race with.
    pub book: String,
    /// `"Small"` or `"Medium"` — [`ResolvedRace::size`], i.e. the race's
    /// `~ Size` trait's `TEMPLATE:SIZE_<code>` over the chassis'
    /// `FACT:BaseSize`. Never the chassis token alone: Aasimar and Tiefling
    /// carry `FACT:BaseSize|S` and are Medium creatures.
    pub size: String,
    /// The race's senses as the Character Sheet prints them, e.g.
    /// `Darkvision 60 ft.`, `Low-light vision`, `Darkvision 120 ft.,
    /// Low-light vision`, or `Normal` for a race that declares no `VISION:`
    /// token at all.
    pub vision: String,
    /// Base land speed in feet — [`ResolvedRace::walk_speed_ft`]. Not the
    /// chassis row's `MOVE:Walk` alone: Goblin's and Hobgoblin's chassis rows
    /// say `MOVE:Walk,0` and their `~ Speed` traits override it to 30.
    pub base_speed_ft: i32,
    /// Fixed racial ability modifiers, keyed by the same ability names
    /// `AbilityScoresDto` uses. Only non-zero entries appear. The frontend
    /// bakes these into the submitted scores (see
    /// `applyRacialAbilityAdjustments`), so a wrong value here is a wrong
    /// character.
    pub ability_adjustments: BTreeMap<String, i16>,
    /// Points the player distributes freely — PF1's "+2 to one ability
    /// score" races. `0` for a race with no such pool.
    pub floating_bonus_points: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaceCreationRosterResponse {
    /// Every race with a complete creation chassis, ordered CRB first then
    /// Bestiary 1, alphabetically within each book.
    pub races: Vec<RaceCreationChassisDto>,
    /// Corpus files that could not be read, plus one entry naming each race
    /// that had to be withheld and the field it was missing. Empty in a
    /// healthy checkout. A race is **dropped rather than defaulted**: an
    /// offered race with a guessed size would compute a wrong carrying
    /// capacity and say nothing about it.
    pub diagnostics: Vec<String>,
}

/// Builds one race's creation chassis, or the reason it cannot be offered.
///
/// The predicate itself lives in the headless rules crate
/// (`codex::rules_core::race_creation`) so that `src/bin/v06_work_inventory.rs`
/// -- which cannot depend on this crate -- can OBSERVE the same function
/// rather than re-implement it. This wrapper is only the wire-DTO mapping;
/// every refusal reason, every `BONUS:STAT` reading and the `VISION:`
/// rendering are that module's, unchanged by the move (SD-31
/// `OPEN-ISSUES.md` rows 170/207/226).
fn race_creation_chassis(
    race: &codex::rules_core::race_resolver::ResolvedRace,
) -> Result<RaceCreationChassisDto, String> {
    let chassis = codex::rules_core::race_creation::race_creation_chassis(race)?;
    Ok(RaceCreationChassisDto {
        race_id: format!("race:{}", chassis.race_key.to_lowercase()),
        label: chassis.race_key,
        book: crate::race_catalog::book_code(&chassis.book_id),
        size: format!("{:?}", chassis.size),
        vision: chassis.vision,
        base_speed_ft: chassis.base_speed_ft,
        ability_adjustments: chassis.ability_adjustments,
        floating_bonus_points: chassis.floating_bonus_points,
    })
}

/// Builds the full creation roster from the real on-disk race corpus.
///
/// A race whose chassis cannot be read completely is **withheld and named in
/// `diagnostics`**, so one gap costs that race and not the other 17.
pub fn build_race_creation_roster() -> RaceCreationRosterResponse {
    let corpus = match crate::race_catalog::race_corpus() {
        Ok(corpus) => corpus,
        Err(message) => {
            return RaceCreationRosterResponse { races: Vec::new(), diagnostics: vec![message.clone()] }
        }
    };

    let mut diagnostics: Vec<String> = corpus
        .diagnostics()
        .iter()
        .map(|diagnostic| format!("{}: {}", diagnostic.path, diagnostic.message))
        .collect();

    let mut races: Vec<RaceCreationChassisDto> = Vec::new();
    for race_key in corpus.race_keys() {
        let Some(resolved) = corpus.resolve(race_key, &[]) else {
            diagnostics.push(format!("{race_key}: could not be resolved against the loaded race corpus"));
            continue;
        };
        match race_creation_chassis(&resolved) {
            Ok(chassis) => races.push(chassis),
            Err(reason) => diagnostics.push(format!("withheld from character creation — {reason}")),
        }
    }

    // The 7 Core Rulebook races first (the roster creation already offered,
    // in the order it offered them), then Bestiary 1's, alphabetically within
    // each book. A book this list does not name sorts last rather than being
    // dropped.
    let book_rank = |book: &str| crate::race_catalog::RACE_CATALOG_BOOKS.iter().position(|b| *b == book).unwrap_or(usize::MAX);
    races.sort_by(|a, b| book_rank(&a.book).cmp(&book_rank(&b.book)).then_with(|| a.label.cmp(&b.label)));

    RaceCreationRosterResponse { races, diagnostics }
}

/// Serves the corpus-derived race roster the creation form builds its race
/// picker from.
#[tauri::command]
pub fn list_race_creation_roster() -> RaceCreationRosterResponse {
    build_race_creation_roster()
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex::rules_core::pilot_compute::HeadlessReceiptStatus;
    use std::collections::BTreeSet;

    // ----- Race-creation roster (the 7 -> 18 widening) -----

    fn roster_race(race_id: &str) -> RaceCreationChassisDto {
        build_race_creation_roster()
            .races
            .into_iter()
            .find(|race| race.race_id == race_id)
            .unwrap_or_else(|| panic!("{race_id} must be offered by the creation roster"))
    }

    /// The widening itself. The roster is built from the corpus, so its size
    /// is a derived fact — 18 records on disk, 18 creatable races. Every id
    /// is asserted, not just the count, so a race silently swapping for
    /// another cannot pass.
    #[test]
    fn creation_roster_offers_every_ingested_race_not_just_the_core_seven() {
        let roster = build_race_creation_roster();
        assert!(
            roster.diagnostics.is_empty(),
            "a healthy checkout serves every race with no diagnostics: {:?}",
            roster.diagnostics
        );

        let ids: Vec<&str> = roster.races.iter().map(|race| race.race_id.as_str()).collect();
        assert_eq!(
            ids,
            vec![
                // The 7 the form offered before this widening.
                "race:dwarf",
                "race:elf",
                "race:gnome",
                "race:half-elf",
                "race:half-orc",
                "race:halfling",
                "race:human",
                // The 11 Bestiary 1 races that reached creation nowhere.
                "race:aasimar",
                "race:drow",
                "race:duergar",
                "race:goblin",
                "race:hobgoblin",
                "race:kobold",
                "race:merfolk",
                "race:orc",
                "race:svirfneblin",
                "race:tengu",
                "race:tiefling",
                // Bestiary 2's 7: the original 6 (SD-31 Epic 1-F2,
                // 2026-08-15) plus Dhampir (SD-32 card-11 T2b lane,
                // 2026-08-23, chassis + standard tier only).
                "race:dhampir",
                "race:fetchling",
                "race:grippli",
                "race:ifrit",
                "race:oread",
                "race:sylph",
                "race:undine",
                // Bestiary 5's 1, the Skinwalker follow-on batch (2026-08-15).
                "race:skinwalker",
                // Bestiary 6's 1, SD-31 wave-24 integration cycle
                // (2026-08-20): Rougarou, same flat chassis+standard-trait
                // shape as Bestiary 2/5 above.
                "race:rougarou",
                // Advanced Race Guide's 6, SD-31-E6-F4-002 (2026-08-16),
                // plus SD31-E6-F4-004's 4-race follow-on (2026-08-17:
                // Gillman, Nagaji, Vanara, Vishkanya) plus SD31-E6-F4-007's
                // 2-race follow-on (2026-08-17: Changeling, Samsaran --
                // closing `arg_races.lst`'s full 37-row playable-race
                // roster) -- the roster sorts by race id, so the new races
                // interleave alphabetically rather than appending at the
                // end.
                "race:catfolk",
                "race:changeling",
                "race:gillman",
                "race:kitsune",
                "race:nagaji",
                "race:ratfolk",
                "race:samsaran",
                "race:strix",
                "race:suli",
                "race:vanara",
                "race:vishkanya",
                "race:wayang",
            ]
        );
    }

    /// Every id the roster emits must be one the compute engine's own race
    /// seams recognize. `race_size_for_race_token` returning `None` is
    /// exactly what silently handed Goblin a Medium creature's carrying
    /// capacity, so a roster entry the size seam cannot resolve is a race
    /// that would be creatable and wrong.
    #[test]
    fn every_offered_race_id_resolves_in_the_engines_own_size_seam() {
        for race in build_race_creation_roster().races {
            let size = codex::rules_core::race_resolver::race_size_for_race_token(&race.race_id)
                .unwrap_or_else(|| panic!("{} must resolve a real creature size", race.race_id));
            assert_eq!(
                format!("{size:?}"),
                race.size,
                "{}: the roster's size must be the engine's own",
                race.race_id
            );
        }
    }

    /// One row of the shipped-values pin below:
    /// `(race_id, size, vision, base_speed_ft, floating_ability_bonuses,
    /// fixed_ability_adjustments)`.
    type ShippedRaceRow = (
        &'static str,
        &'static str,
        &'static str,
        i32,
        u8,
        &'static [(&'static str, i16)],
    );

    /// The 7 races creation already offered keep exactly the values the
    /// hand-maintained `RACE_OPTIONS` table shipped. This is the regression
    /// pin for the swap: corpus-driven must not mean different.
    #[test]
    fn the_seven_previously_offered_races_keep_their_shipped_values() {
        let expected: [ShippedRaceRow; 7] = [
            ("race:human", "Medium", "Normal", 30, 2, &[]),
            ("race:dwarf", "Medium", "Darkvision 60 ft.", 20, 0, &[("charisma", -2), ("constitution", 2), ("wisdom", 2)]),
            ("race:elf", "Medium", "Low-light vision", 30, 0, &[("constitution", -2), ("dexterity", 2), ("intelligence", 2)]),
            ("race:gnome", "Small", "Low-light vision", 20, 0, &[("charisma", 2), ("constitution", 2), ("strength", -2)]),
            ("race:half-elf", "Medium", "Low-light vision", 30, 2, &[]),
            ("race:half-orc", "Medium", "Darkvision 60 ft.", 30, 2, &[]),
            ("race:halfling", "Small", "Normal", 20, 0, &[("charisma", 2), ("dexterity", 2), ("strength", -2)]),
        ];
        for (race_id, size, vision, speed, floating, adjustments) in expected {
            let race = roster_race(race_id);
            assert_eq!(race.size, size, "{race_id} size");
            assert_eq!(race.vision, vision, "{race_id} vision");
            assert_eq!(race.base_speed_ft, speed, "{race_id} speed");
            assert_eq!(race.floating_bonus_points, floating, "{race_id} floating ability points");
            let expected_adjustments: BTreeMap<String, i16> =
                adjustments.iter().map(|(k, v)| ((*k).to_owned(), *v)).collect();
            assert_eq!(race.ability_adjustments, expected_adjustments, "{race_id} ability adjustments");
        }
    }

    /// The three races whose inventory `wiring_class` is `computed`, whose
    /// board `done` therefore rests on `grounded` ALONE — with no
    /// independent `corpus_literal_sweep` byte-verification behind it, the
    /// way the 27 `static` races have (SD-31 wave 14,
    /// `v06_work_inventory.rs`'s `probe_race_creation_roster`). Their
    /// magnitudes are pinned by name here so that credit cannot survive the
    /// roster quietly serving a different number.
    ///
    /// Values transcribed from the rows that declare them, not from the
    /// engine: `data/corpus/beastiary/race_trait/aasimar/
    /// aasimar_ability_scores.json` (`BONUS:STAT|WIS,CHA|2`), the matching
    /// `tiefling_ability_scores.json` (`BONUS:STAT|DEX,INT|2` +
    /// `BONUS:STAT|CHA|-2`) and `data/corpus/advanced_race_guide/race_trait/
    /// changeling/changeling_ability_scores.json` (`BONUS:STAT|WIS,CHA|2` +
    /// `BONUS:STAT|CON|-2`).
    #[test]
    fn the_computed_class_races_serve_their_real_ability_magnitudes() {
        let expected: [ShippedRaceRow; 3] = [
            ("race:aasimar", "Medium", "Darkvision 60 ft.", 30, 0, &[("charisma", 2), ("wisdom", 2)]),
            (
                "race:tiefling",
                "Medium",
                "Darkvision 60 ft.",
                30,
                0,
                &[("charisma", -2), ("dexterity", 2), ("intelligence", 2)],
            ),
            (
                "race:changeling",
                "Medium",
                "Darkvision 60 ft.",
                30,
                0,
                &[("charisma", 2), ("constitution", -2), ("wisdom", 2)],
            ),
        ];
        for (race_id, size, vision, speed, floating, adjustments) in expected {
            let race = roster_race(race_id);
            assert_eq!(race.size, size, "{race_id} size");
            assert_eq!(race.vision, vision, "{race_id} vision");
            assert_eq!(race.base_speed_ft, speed, "{race_id} speed");
            assert_eq!(race.floating_bonus_points, floating, "{race_id} floating ability points");
            let expected_adjustments: BTreeMap<String, i16> =
                adjustments.iter().map(|(k, v)| ((*k).to_owned(), *v)).collect();
            assert_eq!(
                race.ability_adjustments, expected_adjustments,
                "{race_id} ability adjustments"
            );
            assert!(
                !race.ability_adjustments.is_empty(),
                "{race_id} reaches board `done` on `grounded` alone -- an empty magnitude here \
                 would be an unverified credit"
            );
        }
    }

    /// The Bestiary 1 end-to-end case the widening exists for. Goblin is
    /// Small — the exact race whose size was defaulted to Medium at both
    /// encumbrance call sites until the size fix — so its roster entry is
    /// pinned field by field against
    /// `data/corpus/beastiary/race_trait/goblin/`.
    #[test]
    fn goblin_is_creatable_with_its_real_bestiary_1_chassis() {
        let goblin = roster_race("race:goblin");
        assert_eq!(goblin.label, "Goblin");
        assert_eq!(goblin.book, "B1");
        assert_eq!(goblin.size, "Small");
        assert_eq!(goblin.vision, "Darkvision 60 ft.");
        // The chassis row says `MOVE:Walk,0`; `Goblin ~ Speed`'s own
        // `MOVE:Walk,30` overrides it. A roster reading the chassis alone
        // would offer a Goblin that cannot move.
        assert_eq!(goblin.base_speed_ft, 30);
        assert_eq!(goblin.floating_bonus_points, 0);
        assert_eq!(
            goblin.ability_adjustments,
            BTreeMap::from([
                ("charisma".to_owned(), -2),
                ("dexterity".to_owned(), 4),
                ("strength".to_owned(), -2),
            ]),
            "Goblin ~ Ability Scores states +4 Dex in one BONUS:STAT chain and -2 Str/-2 Cha in a \
             second two-ability one"
        );
    }

    /// `BONUS:STAT|STR,CHA|-2` names two abilities in one token. Reading
    /// only up to the comma is the transcription defect that silently
    /// drifted `race_tables.rs` from the corpus on four races, so the
    /// multi-ability chains are pinned explicitly across every race that
    /// has one.
    #[test]
    fn multi_ability_bonus_stat_chains_credit_every_ability_they_name() {
        assert_eq!(roster_race("race:orc").ability_adjustments.len(), 4);
        assert_eq!(
            roster_race("race:svirfneblin").ability_adjustments,
            BTreeMap::from([
                ("charisma".to_owned(), -4),
                ("dexterity".to_owned(), 2),
                ("strength".to_owned(), -2),
                ("wisdom".to_owned(), 2),
            ])
        );
    }

    /// Aasimar and Tiefling carry `FACT:BaseSize|S` on their chassis rows and
    /// are Medium creatures; Merfolk's 5 ft. swim-bound land speed is the
    /// roster's extreme non-30 value. Both are cases where a plausible
    /// default would have been silently wrong.
    #[test]
    fn the_roster_reports_sizes_and_speeds_a_default_would_have_got_wrong() {
        assert_eq!(roster_race("race:aasimar").size, "Medium");
        assert_eq!(roster_race("race:tiefling").size, "Medium");
        assert_eq!(roster_race("race:merfolk").base_speed_ft, 5);
        assert_eq!(roster_race("race:duergar").base_speed_ft, 20);
        // Svirfneblin declares both vision kinds; neither may be dropped.
        assert_eq!(roster_race("race:svirfneblin").vision, "Darkvision 120 ft., Low-light vision");
        assert_eq!(roster_race("race:drow").vision, "Darkvision 120 ft.");
    }

    /// **The end-to-end proof.** A Goblin created through the same
    /// `compose_character_input` the `create_character` command uses must
    /// reach `Computed`, and its carrying capacity must be the Small
    /// column, not the Medium one. `SIZEMULT:S|0.75` — so a Small creature's
    /// thresholds are three quarters of a Medium creature's at the same
    /// Strength, computed on the untruncated heavy load.
    #[test]
    fn a_created_goblin_computes_and_gets_a_small_creatures_carrying_capacity() {
        let goblin_input = compose_character_input(&request_for("race:goblin", 1));
        assert_eq!(
            build_pilot_headless_receipt(&goblin_input).status,
            HeadlessReceiptStatus::Computed,
            "a level-1 Goblin Fighter must reach a fully computed build, or creation would \
             refuse to persist it: {:?}",
            claim_blocking_diagnostic_ids("race:goblin", FIGHTER_CLASS_ID, 1)
        );
        let goblin = compute_pilot_with_corpus(&goblin_input, corpus_fixture_bundle());

        // Same Strength, same loadout, Medium race: the only difference is
        // size, so the ratio isolates it.
        let hobgoblin = compute_pilot_with_corpus(
            &compose_character_input(&request_for("race:hobgoblin", 1)),
            corpus_fixture_bundle(),
        );
        assert_eq!(
            roster_race("race:hobgoblin").size,
            "Medium",
            "the control race must be Medium for this comparison to isolate size"
        );

        // Both requests carry the same Strength 16 (`request_for` is
        // race-blind), so the thresholds differ by creature size and nothing
        // else. `load.lst`'s Strength-16 heavy load is 230 lb.
        //   Medium (no SIZEMULT row, the baseline): 230/3=76, 460/3=153, 230
        //   Small   (SIZEMULT:S|0.75, applied to the load value *before* the
        //            per-tier truncation): 690/12=57, 1380/12=115, 690/4=172
        // Those are exactly PF1's published Small and Medium columns for
        // Strength 16.
        let small = &goblin.corpus_derived.encumbrance.thresholds;
        let medium = &hobgoblin.corpus_derived.encumbrance.thresholds;
        assert_eq!((medium.light_max_lbs, medium.medium_max_lbs, medium.heavy_max_lbs), (76.0, 153.0, 230.0));
        assert_eq!(
            (small.light_max_lbs, small.medium_max_lbs, small.heavy_max_lbs),
            (57.0, 115.0, 172.0),
            "a Small Goblin must not be handed a Medium creature's carrying capacity"
        );
    }

    // -----------------------------------------------------------------
    // SD-27: ARG alternate racial traits survive creation, save and load.
    // -----------------------------------------------------------------

    fn request_with_alternates(
        race_id: &str,
        alternates: &[&str],
    ) -> CreateCharacterRequest {
        CreateCharacterRequest {
            selected_alternate_trait_keys: alternates.iter().map(|key| (*key).to_string()).collect(),
            ..request_for(race_id, 1)
        }
    }

    fn saved_or_panic(response: CreateCharacterResponse) -> Box<CharacterSummaryDto> {
        match response {
            CreateCharacterResponse::Saved { summary, .. } => summary,
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!("this build must be creatable, got: {diagnostics:?}")
            }
        }
    }

    /// **The persistence half of the SD-27 alternate-racial-trait closure**,
    /// through the same `create_character` path the UI button calls: a Dwarf
    /// takes `Dwarf ~ Minesight`, is persisted, read back, and the choice is
    /// still there — carried on `chosen.selected_choices`, which
    /// `SavedCharacterStore` already round-trips, so this needed no schema
    /// change.
    #[test]
    fn a_chosen_alternate_racial_trait_survives_creation_save_and_load() {
        let root = tempdir("create-character-arg-alternate-trait");
        let request = request_with_alternates("race:dwarf", &["Dwarf ~ Minesight"]);
        let summary = saved_or_panic(
            create_character_at_root(&root, &request, "test-version".to_owned())
                .expect("create call should not error"),
        );
        assert_eq!(summary.race_id, "race:dwarf");

        let loaded = load_saved_character_at_root(&root).expect("the saved Dwarf must load back");
        assert_eq!(
            loaded.selected_alternate_trait_keys,
            vec!["Dwarf ~ Minesight".to_string()],
            "the choice must survive the round trip, not be silently dropped"
        );

        // ...and it reaches the engine: the standard 60 ft darkvision record is
        // gone and Minesight's own 90 ft record is in its place. The sheet
        // renders `explanations` verbatim, so this is the number the player
        // sees change.
        let ids: BTreeSet<&str> = loaded.explanations.iter().map(|e| e.id.as_str()).collect();
        assert!(!ids.contains("race.dwarf.trait_bundle.senses"), "the standard darkvision must stop");
        let minesight = loaded
            .explanations
            .iter()
            .find(|e| e.id == "race.dwarf.alternate_trait.minesight.senses")
            .expect("Minesight's own record must be on the loaded sheet");
        assert_eq!(minesight.value, 90);
    }

    // ----- SD-27 gap 4: resolved racial-trait prose reaches the sheet -----

    /// One applied trait's rendered prose out of a loaded character's payload.
    fn applied_trait<'a>(
        loaded: &'a LoadSavedCharacterResponse,
        key: &str,
    ) -> &'a crate::race_trait_picker::AppliedTraitDto {
        loaded
            .resolved_racial_traits
            .applied_traits
            .iter()
            .find(|applied| applied.key == key)
            .unwrap_or_else(|| {
                panic!(
                    "{key} must apply for this character; got {:?}",
                    loaded
                        .resolved_racial_traits
                        .applied_traits
                        .iter()
                        .map(|applied| applied.key.as_str())
                        .collect::<Vec<_>>()
                )
            })
    }

    /// **The character sheet's half of `decisions.md §29.1`'s
    /// producer-with-no-consumer trap.**
    ///
    /// `race_trait_picker::render_trait_description` re-renders a trait's
    /// `DESC:` tokens against the character's own display values, and the Race
    /// Traits picker was its only consumer. `load_saved_character` — the one
    /// call the sheet a player lives in actually makes — carried the chosen
    /// trait *keys* and nothing else, so the sheet could name a trait and never
    /// state what it does.
    ///
    /// The proof is a before/after pair over **one corpus record** (`§28`'s
    /// standing guard): the same `Halfling ~ Adaptable Luck` row must state a
    /// different number for the same character once it holds ARG's
    /// `Fortunate One`. A baked string cannot pass this, and neither can the
    /// stored `data.description` — which is why the racial base below has to
    /// read "Three times per day" and the fed one "4 times per day", the
    /// `PREVARLTEQ:...,3` gate ceasing to apply rather than a number being
    /// substituted.
    #[test]
    fn a_loaded_characters_racial_trait_prose_states_the_number_its_own_feats_produce() {
        let root = tempdir("sheet-racial-trait-prose");
        saved_or_panic(
            create_character_at_root(
                &root,
                &request_with_alternates("race:halfling", &["Halfling ~ Adaptable Luck"]),
                "test-version".to_owned(),
            )
            .expect("create call should not error"),
        );

        // Before: the character holds no display-value feat, so the sheet
        // states the racial base.
        let base = load_saved_character_at_root(&root).expect("the saved Halfling must load back");
        assert!(base.resolved_racial_traits.errors.is_empty(), "{:?}", base.resolved_racial_traits.errors);
        assert_eq!(base.resolved_racial_traits.race_key, "Halfling");
        let base_luck = applied_trait(&base, "Halfling ~ Adaptable Luck");
        assert!(base_luck.description.contains("Three times per day"), "{}", base_luck.description);
        assert!(base_luck.description.contains("gain the full +2 bonus"), "{}", base_luck.description);
        assert!(base_luck.description.contains("only gain a +1 bonus"), "{}", base_luck.description);
        assert!(
            base.resolved_racial_traits.display_value_feats.is_empty(),
            "a created Halfling holds no feat that moves a display value: {:?}",
            base.resolved_racial_traits.display_value_feats
        );

        // After: the very same record, for the very same character, once it
        // holds the ARG luck feat.
        let mut envelope = SavedCharacterStore::load(&root).expect("reload");
        envelope.character_input.chosen.selected_feats.push("Fortunate One".to_owned());
        SavedCharacterStore::save(&envelope, &root).expect("re-save");

        let fed = load_saved_character_at_root(&root).expect("loads");
        let fed_luck = applied_trait(&fed, "Halfling ~ Adaptable Luck");
        assert!(fed_luck.description.contains("4 times per day"), "3 + 1 = 4: {}", fed_luck.description);
        assert!(
            !fed_luck.description.contains("Three"),
            "the PREVARLTEQ gate stops applying rather than a number being swapped: {}",
            fed_luck.description
        );
        assert_ne!(base_luck.description, fed_luck.description, "same record, different sentence");
        assert_eq!(fed.resolved_racial_traits.display_value_feats, vec!["Fortunate One".to_string()]);

        // The per-record `moved_by_feats` flag is the screen's licence to say
        // *why* the number differs, and it is derived by re-rendering rather
        // than asserted from the feat list.
        let moved = fed
            .resolved_racial_traits
            .rendered_trait_descriptions
            .iter()
            .find(|row| row.key == "Halfling ~ Adaptable Luck")
            .expect("a rendered row for the applied trait");
        assert!(moved.moved_by_feats);
        assert_eq!(moved.text, fed_luck.description, "one sentence per trait, whichever list shows it");

        std::fs::remove_dir_all(&root).ok();
    }

    /// Every racial trait that reaches the sheet carries real prose, and the
    /// chosen alternate really replaced something. A name-only card — what the
    /// sheet showed before — would pass a "the key survived" test and fail this
    /// one.
    #[test]
    fn every_racial_trait_on_a_loaded_sheet_carries_rendered_prose_and_names_what_it_replaced() {
        use codex::rules_core::pcgen_desc::leaked_pcgen_syntax;

        let root = tempdir("sheet-racial-trait-coverage");
        saved_or_panic(
            create_character_at_root(
                &root,
                &request_with_alternates("race:dwarf", &["Dwarf ~ Minesight"]),
                "test-version".to_owned(),
            )
            .expect("create call should not error"),
        );
        let loaded = load_saved_character_at_root(&root).expect("loads");
        let resolved = &loaded.resolved_racial_traits;

        assert!(resolved.errors.is_empty(), "{:?}", resolved.errors);
        assert!(!resolved.applied_traits.is_empty(), "a Dwarf applies its racial traits");
        for applied in &resolved.applied_traits {
            assert!(!applied.description.trim().is_empty(), "{} has prose", applied.key);
            assert_eq!(leaked_pcgen_syntax(&applied.description), None, "{}: {}", applied.key, applied.description);
        }
        assert!(resolved.applied_traits.iter().any(|applied| applied.key == "Dwarf ~ Minesight"));

        // The swap the player made, in the resolver's own words.
        let suppressed: Vec<&str> =
            resolved.suppressions.iter().map(|s| s.suppressed_trait_key.as_str()).collect();
        assert!(
            suppressed.contains(&"Dwarf ~ Vision"),
            "Minesight replaces the standard darkvision row: {suppressed:?}"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// A character who chose no alternate still gets its race's standard
    /// traits rendered — the sheet's racial section is not an
    /// alternates-only surface, and an empty payload here would read as "this
    /// race has no traits".
    #[test]
    fn a_character_with_no_alternate_chosen_still_gets_its_standard_traits_rendered() {
        let root = tempdir("sheet-racial-trait-standard-only");
        saved_or_panic(
            create_character_at_root(&root, &request_for("race:halfling", 1), "test-version".to_owned())
                .expect("create call should not error"),
        );
        let loaded = load_saved_character_at_root(&root).expect("loads");
        assert!(loaded.selected_alternate_trait_keys.is_empty());
        let resolved = &loaded.resolved_racial_traits;
        assert!(resolved.errors.is_empty(), "{:?}", resolved.errors);
        assert!(resolved.suppressions.is_empty(), "nothing was replaced");
        assert!(
            resolved.applied_traits.iter().all(|applied| applied.role == "default"),
            "no alternate was taken, so every applied trait is a racial default"
        );
        assert!(resolved.applied_traits.len() >= 5, "{}", resolved.applied_traits.len());
        // The standard Halfling Luck row, which the alternate above replaces.
        assert!(resolved.applied_traits.iter().any(|applied| applied.key == "Halfling ~ Halfling Luck"));
    }

    /// A Dwarf who chose nothing is byte-identical to the pre-SD-27 build:
    /// no selection persisted, and the standard 60 ft darkvision intact. The
    /// opt-in half of the guard.
    #[test]
    fn a_dwarf_who_chose_no_alternate_persists_none_and_keeps_the_standard_trait() {
        let root = tempdir("create-character-no-alternate-trait");
        saved_or_panic(
            create_character_at_root(&root, &request_for("race:dwarf", 1), "test-version".to_owned())
                .expect("create call should not error"),
        );
        let loaded = load_saved_character_at_root(&root).expect("loads");
        assert!(loaded.selected_alternate_trait_keys.is_empty());
        assert!(loaded.explanations.iter().any(|e| e.id == "race.dwarf.trait_bundle.senses"));
        assert!(loaded
            .explanations
            .iter()
            .all(|e| e.id != "race.dwarf.alternate_trait.minesight.senses"));
    }

    /// **A top-level sheet number moving because of a racial-trait choice.**
    /// A Half-Elf Fighter 1 who takes `Dual Minded` (ARG p.42,
    /// `BONUS:SAVE|Will|2`) saves and loads with Will +3 where the same build
    /// without it has +1 — on `snapshot.total_saves`, which the sheet prints at
    /// the top of the page.
    #[test]
    fn dual_minded_moves_a_saved_half_elfs_total_will_save_on_the_loaded_sheet() {
        let plain_root = tempdir("create-character-half-elf-plain");
        saved_or_panic(
            create_character_at_root(&plain_root, &request_for("race:half-elf", 1), "test-version".to_owned())
                .expect("create"),
        );
        let plain = load_saved_character_at_root(&plain_root).expect("loads");

        let swapped_root = tempdir("create-character-half-elf-dual-minded");
        saved_or_panic(
            create_character_at_root(
                &swapped_root,
                &request_with_alternates("race:half-elf", &["Half-Elf ~ Dual Minded"]),
                "test-version".to_owned(),
            )
            .expect("create"),
        );
        let swapped = load_saved_character_at_root(&swapped_root).expect("loads");

        let will = |response: &LoadSavedCharacterResponse| {
            response.snapshot.as_ref().expect("a computed build has a snapshot").total_saves.will
        };
        assert_eq!(will(&plain), 1, "Fighter 1 base Will +0, Wisdom 12 (+1)");
        assert_eq!(will(&swapped), 3, "+2 from Dual Minded");
        assert_eq!(swapped.selected_alternate_trait_keys, vec!["Half-Elf ~ Dual Minded".to_string()]);
    }

    /// A selection the corpus does not recognize is refused at creation, with
    /// the resolver's own finding, rather than silently dropped into a saved
    /// character that quietly does not have the trait.
    #[test]
    fn an_alternate_trait_key_that_is_not_this_races_blocks_the_save_rather_than_vanishing() {
        let root = tempdir("create-character-wrong-race-alternate-trait");
        let response = create_character_at_root(
            &root,
            // A real ARG alternate — but a Half-Elf's, offered to a Dwarf.
            &request_with_alternates("race:dwarf", &["Half-Elf ~ Dual Minded"]),
            "test-version".to_owned(),
        )
        .expect("create call should not error");
        match response {
            CreateCharacterResponse::Saved { .. } => {
                panic!("a trait belonging to another race must not be accepted")
            }
            CreateCharacterResponse::Blocked { diagnostics } => {
                let diagnostic = diagnostics
                    .iter()
                    .find(|d| d.id == "race.alternate_trait.unmatched_selection")
                    .expect("the refusal must name why");
                assert!(diagnostic.claim_blocking);
                assert!(diagnostic.message.contains("Half-Elf ~ Dual Minded"));
            }
        }
        assert!(load_saved_character_at_root(&root).is_err(), "nothing may have been persisted");
    }

    /// Two alternates that ARG's own `PREMULT` guard excludes from each other
    /// are refused together, naming the shared flag — the picker disables the
    /// second option, and this is the backend saying no to a caller that
    /// submits it anyway.
    #[test]
    fn two_mutually_exclusive_alternates_are_refused_with_the_flag_that_excludes_them() {
        let root = tempdir("create-character-conflicting-alternate-traits");
        let response = create_character_at_root(
            &root,
            // Both fire Dwarf_ReplaceStonecunning; Sky Sentinel's guard names it.
            &request_with_alternates("race:dwarf", &["Dwarf ~ Saltbeard", "Dwarf ~ Sky Sentinel"]),
            "test-version".to_owned(),
        )
        .expect("create call should not error");
        match response {
            CreateCharacterResponse::Saved { .. } => panic!("an illegal pair must not be accepted"),
            CreateCharacterResponse::Blocked { diagnostics } => {
                let diagnostic = diagnostics
                    .iter()
                    .find(|d| d.id == "race.alternate_trait.mutually_exclusive")
                    .expect("the refusal must name the guard");
                assert!(diagnostic.claim_blocking);
                assert!(diagnostic.message.contains("Dwarf_Replace"), "{}", diagnostic.message);
            }
        }
    }

    /// Every key the picker offers for a race is one `create_character`
    /// accepts for that race. A menu item the creation path refuses would be a
    /// dead affordance — the failure `docs/governance/no-stub-mvp-doctrine.md`
    /// names directly.
    ///
    /// Run over the 7 CRB races' alternates only: those are the races whose
    /// Fighter 1 build this deterministic fixture reaches `Computed` for
    /// without further seeding, so a `Blocked` here is unambiguously the
    /// racial-trait path's fault rather than an unrelated chassis gate.
    #[test]
    fn every_alternate_the_picker_offers_for_a_crb_race_is_one_creation_accepts() {
        let menu = crate::race_trait_picker::build_alternate_racial_traits();
        assert!(menu.diagnostics.is_empty(), "{:?}", menu.diagnostics);
        let mut accepted = 0usize;
        for race in &menu.races {
            let race_id = format!("race:{}", race.race_key.to_lowercase());
            if !["race:dwarf", "race:elf", "race:gnome", "race:half-elf", "race:half-orc", "race:halfling", "race:human"]
                .contains(&race_id.as_str())
            {
                continue;
            }
            for alternate in &race.alternates {
                let root = tempdir(&format!("arg-alt-{}", accepted));
                let response = create_character_at_root(
                    &root,
                    &request_with_alternates(&race_id, &[alternate.key.as_str()]),
                    "test-version".to_owned(),
                )
                .expect("create call should not error");
                match response {
                    CreateCharacterResponse::Saved { .. } => {}
                    CreateCharacterResponse::Blocked { diagnostics } => panic!(
                        "the picker offers {} for {race_id} but creation refused it: {diagnostics:?}",
                        alternate.key
                    ),
                }
                let loaded = load_saved_character_at_root(&root).expect("loads");
                assert_eq!(loaded.selected_alternate_trait_keys, vec![alternate.key.clone()]);
                accepted += 1;
            }
        }
        assert_eq!(
            accepted, 188,
            "the 7 CRB races' alternates: 30+27+23+20+28+27+33 (**Elf 28 -> 27 on 2026-08-12**, \
             SD-29 `decisions.md` 53: ISR's `Elf ~ Sovyrian-Born` carries `NAMEISPI:YES` and is \
             dropped at ingest, because a name cannot be redacted), i.e. round 2's \
             24+21+18+16+22+20+27 plus Horror Adventures' 6+7+5+4+6+7+6 (SD-29's race-trait \
             extend lane, round 3). Round 2's own figure was the previous \
             17+13+12+9+15+13+15 plus Inner Sea Races' 7+8+6+7+7+7+12. Half-Orc's total \
             includes APG's `Half-Orc ~ Plagueborn`, landed by the same lane's round 1 -- \
             and this test is exactly the one that \
             would have caught either book being shipped without its \
             `ALTERNATE_TRAIT_REPLACE_FLAGS` rows, because it saves a real character \
             holding each alternate in turn and reloads it"
        );
    }

    /// **The full end-to-end proof**, through the same `create_character`
    /// path the button in the UI calls: a Bestiary 1 Goblin is created,
    /// persisted to disk, read back, and the carrying capacity that survives
    /// the round trip is the Small one.
    ///
    /// Distinct from
    /// `a_created_goblin_computes_and_gets_a_small_creatures_carrying_capacity`,
    /// which stops at the compute seam. `create_character_at_root` refuses to
    /// persist anything that is not `Computed`, so reaching a saved envelope
    /// at all is itself part of the claim.
    #[test]
    fn a_goblin_created_through_the_real_command_persists_a_small_creatures_carrying_capacity() {
        let root = tempdir("create-character-bestiary-1-goblin");
        let response =
            create_character_at_root(&root, &request_for("race:goblin", 1), "test-version".to_owned())
                .expect("create call should not error");
        match response {
            CreateCharacterResponse::Saved { summary, .. } => {
                assert_eq!(summary.race_id, "race:goblin", "the Bestiary 1 race reaches the saved envelope");
            }
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!("a Goblin Fighter level 1 must be creatable, got: {diagnostics:?}")
            }
        }

        let loaded = load_saved_character_at_root(&root).expect("the saved Goblin must load back");
        assert_eq!(loaded.summary.race_id, "race:goblin");
        assert_eq!(
            (
                loaded.corpus_derived.encumbrance.light_max_lbs,
                loaded.corpus_derived.encumbrance.medium_max_lbs,
                loaded.corpus_derived.encumbrance.heavy_max_lbs,
            ),
            (57.0, 115.0, 172.0),
            "PF1's Small column at Strength 16; the Medium column would be 76/153/230"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// The claim-blocking size diagnostic must fire for a race the engine
    /// cannot resolve — and must *not* fire for any race the roster offers.
    /// Without this, a roster entry that outran the engine's size seam would
    /// look identical to one that did not.
    #[test]
    fn no_offered_race_trips_the_unknown_size_diagnostic_and_an_unoffered_one_does() {
        use codex::rules_core::contract::UNKNOWN_RACE_SIZE_DIAGNOSTIC_ID;
        for race in build_race_creation_roster().races {
            let receipt = compute_pilot_with_corpus(
                &compose_character_input(&request_for(&race.race_id, 1)),
                corpus_fixture_bundle(),
            );
            assert!(
                !receipt.base.diagnostics.iter().any(|d| d.id == UNKNOWN_RACE_SIZE_DIAGNOSTIC_ID),
                "{} is offered for creation, so its size must be real data",
                race.race_id
            );
        }
        // Dhampir gained a chassis + standard-tier traits, SD-32 card-11
        // T2b lane (2026-08-23), and is now offered above -- Kasatha (ARG's
        // reprint of an Inner Sea Races race, `inner_sea_races` itself
        // un-ingested for it) stands in as the still-genuinely-un-ingested
        // example this test needs.
        let unknown = compute_pilot_with_corpus(
            &compose_character_input(&request_for("race:kasatha", 1)),
            corpus_fixture_bundle(),
        );
        assert!(
            unknown.base.diagnostics.iter().any(|d| d.id == UNKNOWN_RACE_SIZE_DIAGNOSTIC_ID),
            "an un-ingested race must report its guessed size rather than computing quietly"
        );
    }

    /// An empty-loadout `EncumbranceDto` for the *serialization-shape*
    /// tests below, which assert camelCase key naming and tag placement and
    /// care nothing about encumbrance values.
    ///
    /// The thresholds are the real Strength-10 row (`load.lst`
    /// `LOAD:10|100`, so light 33 / medium 66 / heavy 100) rather than
    /// zeroes, so this fixture never asserts a rules value that could not
    /// actually occur. Carrying nothing is genuinely a light load with no
    /// penalties. Production `CorpusDerivedDto`s are always built by
    /// `map_corpus_derived_dto` from a real computation -- this helper is
    /// reachable only from `#[cfg(test)]`.
    fn empty_encumbrance_dto() -> EncumbranceDto {
        EncumbranceDto {
            total_carried_weight_lbs: 0.0,
            total_carried_cost_gp: 0.0,
            light_max_lbs: 33.0,
            medium_max_lbs: 66.0,
            heavy_max_lbs: 100.0,
            level: "Light".to_owned(),
            load_max_dex_cap: None,
            load_armor_check_penalty: 0,
            per_item: Vec::new(),
            unresolved_item_ids: Vec::new(),
        }
    }

    /// Every race a player can actually pick, read off the roster the
    /// creation form's picker is built from rather than re-listed here.
    ///
    /// This was a hardcoded 7-entry array of the Core Rulebook races. Derived
    /// instead, it cannot fall behind the roster: the moment a race becomes
    /// creatable it also becomes something
    /// `compose_character_input_reaches_computed_status_for_supported_fighter_levels_1_to_3`
    /// has to prove computes, which is the whole point of that test.
    fn curated_race_ids() -> Vec<String> {
        let roster = build_race_creation_roster();
        assert!(
            roster.races.len() >= 7,
            "the creation roster must never shrink below the 7 Core Rulebook races: {:?}",
            roster.diagnostics
        );
        roster.races.into_iter().map(|race| race.race_id).collect()
    }
    const FIGHTER_CLASS_ID: &str = "class:fighter";

    // `GENERIC_DIAGNOSTIC_IDS` / `generic_ids()` / `generic_plus()` used to
    // live here: the 4 chassis-wide diagnostics every unrecognized class
    // tripped, plus a helper to assert "those 4, and additionally these
    // named ones". Removed 2026-07-29 with Monk's choice-picker Path A
    // closure -- Monk was the last class in this test still expecting them,
    // so every remaining assertion below is either `BTreeSet::new()` (fully
    // Computed) or a named-diagnostics-only set. They were deleted rather
    // than `#[allow(dead_code)]`-ed: a helper no assertion uses cannot go
    // stale in a way any test would catch.
    fn request_for(race_id: &str, level: u8) -> CreateCharacterRequest {
        request_for_class(race_id, FIGHTER_CLASS_ID, level)
    }

    fn request_for_class(race_id: &str, class_id: &str, level: u8) -> CreateCharacterRequest {
        CreateCharacterRequest {
            character_id: "char-test".to_owned(),
            display_label: "Test Character".to_owned(),
            race_id: race_id.to_owned(),
            class_id: class_id.to_owned(),
            level,
            ability_scores: AbilityScoresDto {
                strength: 16,
                dexterity: 14,
                constitution: 14,
                intelligence: 10,
                wisdom: 12,
                charisma: 8,
            },
            ability_bonus_target: "strength".to_owned(),
            selected_alternate_trait_keys: Vec::new(),
            companion_species: None,
            selected_traits: Vec::new(),
            trait_skill_choices: Vec::new(),
            saved_at: "2026-07-08T00:00:00Z".to_owned(),
        }
    }

    fn claim_blocking_diagnostic_ids(race_id: &str, class_id: &str, level: u8) -> BTreeSet<String> {
        let input = compose_character_input(&request_for_class(race_id, class_id, level));
        let receipt = build_pilot_headless_receipt(&input);
        receipt
            .computation
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.claim_blocking)
            .map(|diagnostic| diagnostic.id.clone())
            .collect()
    }

    #[test]
    fn compose_character_input_for_human_includes_human_only_choice_slots() {
        let input = compose_character_input(&request_for("race:human", 1));

        let has_choice = |choice_set_id: &str| {
            input
                .chosen
                .selected_choices
                .iter()
                .any(|choice| choice.choice_set_id == choice_set_id)
        };

        assert!(has_choice("choice:human_bonus_feat"));
        assert!(has_choice("choice:human_ability_bonus"));
        assert!(has_choice("choice:level_1_character_feat"));
        assert!(has_choice("choice:fighter_bonus_feat"));
    }

    #[test]
    fn compose_character_input_honors_the_requested_human_ability_bonus_target() {
        let mut request = request_for("race:human", 1);
        request.ability_bonus_target = "dexterity".to_owned();

        let input = compose_character_input(&request);

        let selection = input
            .chosen
            .selected_choices
            .iter()
            .find(|choice| choice.choice_set_id == "choice:human_ability_bonus")
            .map(|choice| choice.selection_id.as_str());

        assert_eq!(selection, Some("ability:dexterity"));
    }

    /// SD-24 Criterion 7.5's own RED sentinel: `compose_character_input`
    /// (the composer behind `create_character` / `seed_default_character_if_needed`)
    /// must not fabricate a demo spell loadout for a freshly composed
    /// character — `CreateCharacterRequest` carries no spell selections at
    /// all, so any non-empty `spells_selected` on the composed input can
    /// only be a hardcoded default, not a real caller choice. A real
    /// character's spells come from the wired `add_spell_selection` /
    /// `appendToCharacter` command surface (SD-23/SD-24 Epic 7) after
    /// creation, not from a baked-in placeholder at creation time.
    #[test]
    fn compose_character_input_does_not_fabricate_a_default_spell_loadout() {
        let input = compose_character_input(&request_for("race:human", 1));

        assert!(
            input.chosen.spells_selected.is_empty(),
            "compose_character_input must not hardcode a demo spell loadout; got {:?}",
            input.chosen.spells_selected
        );
    }

    #[test]
    fn compose_character_input_for_non_human_omits_human_only_choice_slots() {
        let input = compose_character_input(&request_for("race:half-orc", 1));

        let has_choice = |choice_set_id: &str| {
            input
                .chosen
                .selected_choices
                .iter()
                .any(|choice| choice.choice_set_id == choice_set_id)
        };

        assert!(!has_choice("choice:human_bonus_feat"));
        assert!(!has_choice("choice:human_ability_bonus"));
        // The race-agnostic slots remain present.
        assert!(has_choice("choice:level_1_character_feat"));
        assert!(has_choice("choice:fighter_bonus_feat"));
        assert_eq!(input.chosen.race_id, "race:half-orc");
    }


    #[test]
    fn compose_character_input_threads_the_requested_class_id() {
        let input = compose_character_input(&request_for_class("race:human", "class:paladin", 1));
        assert_eq!(input.chosen.class_levels[0].class_id, "class:paladin");
    }

    /// SD-32 T12 Epic 10 row 20 cycle 6: proves row 20 cycle 5's own claim
    /// ("the character-creation-time picker is already wired -- `class_id`
    /// is a free-form string dispatched by `compute_class_chassis`, not a
    /// separate `ClassId`-enum picker widget") at the REAL
    /// character-creation altitude, not just `generic_class_chassis::
    /// resolve`'s own isolated unit tests (`src/rules_core/pilot_compute/
    /// generic_class_chassis.rs`, which only proves the crate-internal
    /// function in isolation). Iterates every one of the 61 conventional PC
    /// classes `class_catalog_generic.rs` re-derives from the corpus (60 via
    /// `load_generic_class_progressions`, plus Demoniac -- named separately
    /// because THAT module's own formula evaluator does not bind the bare
    /// `classlevel()` empty-key sentinel `generic_class_chassis::resolve`
    /// binds; see that module's own doc comment, "All 61 resolve --
    /// Demoniac closed on rebase, mid-cycle") at level 1, and asserts NONE
    /// of them falls through to the `class_chassis.unsupported` diagnostic
    /// -- `compute_class_chassis`'s (`src/rules_core/pilot_compute/mod.rs`)
    /// only fallback when no dispatch arm, including `generic_class_
    /// chassis::resolve`, recognizes the class id. A class that still fell
    /// through here would surface as "Blocked: unsupported class" to a real
    /// player picking it at creation -- exactly the gap this cycle's brief
    /// asked to be either closed or precisely disproven with evidence.
    #[test]
    fn all_61_generic_classes_reach_a_real_chassis_at_character_creation_altitude() {
        let repo_root = crate::authoring_workbench::codex_repo_root().expect("repo root");
        let (records, unresolved) =
            crate::class_catalog_generic::load_generic_class_progressions(&repo_root);
        assert!(
            unresolved.is_empty() || unresolved.iter().all(|(_, name)| name == "Demoniac"),
            "class_catalog_generic.rs's own unresolved list must contain only the named \
             Demoniac gap, got: {unresolved:?}"
        );
        let mut names: Vec<String> = records.into_iter().map(|record| record.name).collect();
        assert_eq!(
            names.len(),
            60,
            "expected 60 of the 61 conventional PC classes from class_catalog_generic.rs's own \
             re-derivation (Demoniac is the one named gap in THAT module, closed instead by \
             generic_class_chassis::resolve's own CLASSLEVEL:: binding -- see this test's own \
             doc comment)"
        );
        names.push("Demoniac".to_owned());
        assert_eq!(names.len(), 61, "must cover all 61, not a partial sweep");

        let slug = |name: &str| -> String {
            name.trim().to_ascii_lowercase().split_whitespace().collect::<Vec<_>>().join("_")
        };

        let mut checked = 0usize;
        for name in &names {
            let class_id = format!("class:{}", slug(name));
            let diagnostics = claim_blocking_diagnostic_ids("race:human", &class_id, 1);
            assert!(
                !diagnostics.contains("class_chassis.unsupported"),
                "{name} ({class_id}) must resolve a real chassis at character-creation \
                 altitude, not fall through to class_chassis.unsupported -- got diagnostics: \
                 {diagnostics:?}"
            );
            checked += 1;
        }
        assert_eq!(
            checked, 61,
            "must have exercised all 61 conventional classes, not a partial sweep"
        );
    }

    /// SD-32 T12 Epic 10 row 20 cycle 7: closes cycle 6's own named wiring
    /// gap ("`ground_companion_stat_block` has zero live callers anywhere
    /// in the crate") and proves it at the real character-creation
    /// altitude, the same way `all_61_generic_classes_reach_a_real_
    /// chassis_at_character_creation_altitude` proved the class picker --
    /// through `CreateCharacterRequest` -> `compose_character_input` ->
    /// `build_pilot_headless_receipt`, never `generic_class_chassis::
    /// resolve` or `ground_companion_stat_block` called directly in
    /// isolation. A Druid whose request carries `companion_species:
    /// Some("gulper_plant")` must reach `Computed` (the default Druid
    /// nature-bond seed is unaffected -- only the species changes), and
    /// its explanation records must carry Gulper Plant's own verified
    /// stat block (`companion_base_stat_table.rs`'s table), never Wolf's.
    #[test]
    fn a_druid_who_selects_gulper_plant_grounds_gulper_plant_not_wolf_at_character_creation_altitude()
    {
        let default_request = request_for_class("race:human", "class:druid", 1);
        let default_input = compose_character_input(&default_request);
        let default_receipt = build_pilot_headless_receipt(&default_input);
        assert_eq!(
            default_receipt.status,
            HeadlessReceiptStatus::Computed,
            "precondition: a Druid with no companion_species override must still reach Computed \
             (the existing Wolf default), got: {:?}",
            default_receipt.computation.diagnostics
        );
        assert!(
            default_receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_chassis.druid.animal_companion.wolf_stat_block"),
            "precondition: the default (no override) Druid must ground Wolf, not some other \
             species, or this test is not exercising the override case: {:?}",
            default_receipt.computation.explanations.iter().map(|e| &e.id).collect::<Vec<_>>()
        );

        let gulper_plant_request = CreateCharacterRequest {
            companion_species: Some("gulper_plant".to_owned()),
            ..request_for_class("race:human", "class:druid", 1)
        };
        let gulper_plant_input = compose_character_input(&gulper_plant_request);
        let gulper_plant_receipt = build_pilot_headless_receipt(&gulper_plant_input);
        assert_eq!(
            gulper_plant_receipt.status,
            HeadlessReceiptStatus::Computed,
            "a Druid who selects a verified-but-non-default companion species must still reach \
             Computed, got: {:?}",
            gulper_plant_receipt.computation.diagnostics
        );
        let ids: std::collections::BTreeSet<&str> =
            gulper_plant_receipt.computation.explanations.iter().map(|e| e.id.as_str()).collect();
        assert!(
            ids.contains("class_chassis.druid.animal_companion.gulper_plant_stat_block"),
            "a Druid who selected gulper_plant must ground Gulper Plant's own verified stat \
             block through the real character-creation request path, got: {ids:?}"
        );
        assert!(
            !ids.contains("class_chassis.druid.animal_companion.wolf_stat_block"),
            "a Druid who selected gulper_plant must NOT also ground Wolf's stat block -- the \
             dispatch must replace the default, not merely add to it: {ids:?}"
        );
        let base_attack = gulper_plant_receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_chassis.druid.animal_companion.base_attack_bonus")
            .expect("expected a base_attack_bonus record");
        assert_eq!(
            base_attack.value, 2,
            "Gulper Plant base attack bonus at master level 1: HD*3/4 (2 HD -> 1) + Str 12 \
             modifier (+1) = 2 (row 20 cycle 9 correction: the table's own strength/constitution \
             fields hold the species' printed 1st-level total directly, matching Wolf/Horse's \
             own established precedent -- Str 12/Con 13 for Gulper Plant, not the delta-backed- \
             out Str 10/Con 11 cycles 5-8 stored; see companion_base_stat_table.rs's own cycle 9 \
             module-doc addendum), matching companion_base_stat_table.rs's own \
             gulper_plant_grounds_a_real_new_species_at_master_level_1 test"
        );

        // An unrecognized species slug must fall back to the class's own
        // prior default, never fabricate a stat block for an unverified
        // species and never block the character. Row 20 cycle 13 grounded
        // `griffon` (this test's own unknown-species example through cycle
        // 12) as part of closing the full 196-record companion population
        // (`companion_base_stat_table.rs`'s own cycle-13 addendum), so the
        // fallback example moves to a slug that is not, and never has
        // been, a real PF1 companion species -- proving the fallback path
        // still exists and still works now that the table has no real gaps
        // left to exercise it with, the same correction
        // `companion_base_stat_table.rs`'s own
        // `an_unknown_species_slug_refuses_rather_than_guesses` test made.
        let unknown_species_request = CreateCharacterRequest {
            companion_species: Some("not_a_real_companion_species".to_owned()),
            ..request_for_class("race:human", "class:druid", 1)
        };
        let unknown_species_receipt =
            build_pilot_headless_receipt(&compose_character_input(&unknown_species_request));
        assert_eq!(
            unknown_species_receipt.status,
            HeadlessReceiptStatus::Computed,
            "an unrecognized companion_species must fall back to the class default, not block \
             the character: {:?}",
            unknown_species_receipt.computation.diagnostics
        );
        assert!(
            unknown_species_receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_chassis.druid.animal_companion.wolf_stat_block"),
            "an unrecognized companion_species (not_a_real_companion_species, no verified table \
             row) must fall back to grounding Wolf, this class's own prior default, never \
             fabricate a stat block for it: {:?}",
            unknown_species_receipt.computation.explanations.iter().map(|e| &e.id).collect::<Vec<_>>()
        );

        // Positive proof, not just the refusal-path correction above:
        // `griffon` itself now grounds through the SAME real
        // character-creation request path `gulper_plant` was proven
        // through above, never falling back to Wolf.
        let griffon_request = CreateCharacterRequest {
            companion_species: Some("griffon".to_owned()),
            ..request_for_class("race:human", "class:druid", 1)
        };
        let griffon_receipt = build_pilot_headless_receipt(&compose_character_input(&griffon_request));
        assert_eq!(
            griffon_receipt.status,
            HeadlessReceiptStatus::Computed,
            "a Druid who selects griffon (grounded by row 20 cycle 13) must reach Computed: {:?}",
            griffon_receipt.computation.diagnostics
        );
        let griffon_ids: std::collections::BTreeSet<&str> =
            griffon_receipt.computation.explanations.iter().map(|e| e.id.as_str()).collect();
        assert!(
            griffon_ids.contains("class_chassis.druid.animal_companion.griffon_stat_block"),
            "a Druid who selected griffon must ground Griffon's own verified stat block, not \
             fall back to Wolf, now that row 20 cycle 13 grounded it: {griffon_ids:?}"
        );
        assert!(
            !griffon_ids.contains("class_chassis.druid.animal_companion.wolf_stat_block"),
            "a Druid who selected griffon must NOT also ground Wolf's stat block: {griffon_ids:?}"
        );
    }

    /// The single most important regression guard: proves the golden-path
    /// claim against the real engine, not just against this module's own
    /// description of it. If the compute engine's requirements ever drift,
    /// this test fails loudly instead of the character-hub UI silently
    /// showing "Blocked" for what users were told was the supported path.
    #[test]
    fn compose_character_input_reaches_computed_status_for_supported_fighter_levels_1_to_3() {
        for race_id in curated_race_ids() {
            for level in 1..=3u8 {
                let input = compose_character_input(&request_for(&race_id, level));
                let receipt = build_pilot_headless_receipt(&input);
                assert_eq!(
                    receipt.status,
                    HeadlessReceiptStatus::Computed,
                    "race {race_id} level {level} must reach Computed status, got diagnostics: {:?}",
                    receipt.computation.diagnostics
                );
            }
        }
    }

    /// Pins the exact claim-blocking diagnostic id set the picker's
    /// grouping UI is built against, for every class/race/level shape the
    /// frontend catalogue treats as distinct. A drift in the compute
    /// engine's diagnostic ids must fail this test loudly, not surface as a
    /// silently-empty "what's missing" panel in the app.
    #[test]
    fn claim_blocking_diagnostic_ids_match_the_catalogued_support_shape_per_class() {
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:fighter", 1),
            BTreeSet::new(),
            "Human Fighter L1 is the golden path and must reach Computed with zero claim-blocking diagnostics"
        );

        // v0.6 alpha swarm, risks item 8, third slice (2026-07-25): `table_class_id`
        // now recognizes Paladin too (mirroring the Ranger widening), so the 4
        // generic chassis-wide diagnostics no longer trip. Paladin's own
        // `class_spell.paladin.partial_caster.unsupported` diagnostic is also no
        // longer unconditional -- it's now a real validation
        // (`unmet_paladin_prepared_spell_conditions`) that only fires on a genuine
        // posture violation, and `compose_character_input` seeds no Paladin spell
        // selections, so the (valid, empty) posture no longer trips it. Only the
        // hybrid spell burden remains: the sibling
        // `class_feature.hybrid.paladin.unsupported` diagnostic was retired
        // (tranche/6) because it flatly claimed Smite Evil / lay on hands / divine
        // grace / mercy were unimplemented while the per-class decomposition
        // dispatched on the same input grounds those burdens for real (or as
        // correct level-1 absences) -- see
        // `tests/hybrid_diagnostic_grounded_contradiction.rs`.
        //
        // v0.6 alpha swarm (2026-07-28): the remaining hybrid SPELL diagnostic
        // (`class_spell.hybrid.paladin.unsupported`) has now been retired too, so
        // this set is empty and Human Paladin L1 joins Human Fighter L1 on the
        // golden path. Paladins gain no spellcasting until class level 4 in PF1
        // (`cr_classes.lst`'s `CLASS:Paladin` block has no `CAST:` row at all
        // before level 4), so a level-1 Paladin's absent spell posture is the
        // CORRECT computed answer, not a missing one -- and the per-class
        // decomposition already grounds it (effective caster level 0, access
        // ceiling 0, zero prepared spells). See
        // `tests/v06_hybrid_level1_no_spellcasting_is_computed.rs`.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:paladin", 1),
            BTreeSet::new(),
            "Human Paladin L1 now reaches Computed with zero claim-blocking diagnostics: \
             having no spellcasting at level 1 is a satisfied PF1 condition, not a gap"
        );

        // v0.6 alpha swarm, risks item 8 (2026-07-24): `table_class_id` now
        // recognizes Ranger (`class-multiclass-breadth-scoping.md`'s
        // recommended first slice), so Ranger no longer trips any of the 4
        // generic chassis-wide diagnostics -- same shape as Wizard's own
        // transition, below. A first version of this assertion here still
        // named `class_spell.ranger.partial_caster.unsupported` because that
        // diagnostic was unconditional at the time; the 2026-07-25 slice made
        // it a real, conditional validation (mirrors the Paladin update just
        // above), and `compose_character_input` seeds no Ranger spell
        // selections, so a Human Ranger L1's (valid, empty) posture no longer
        // trips it. Only the hybrid spell burden remains: the sibling
        // `class_feature.hybrid.ranger.unsupported` diagnostic was retired
        // (tranche/6) because it flatly claimed favored enemy / combat style /
        // tracking were unimplemented while the per-class decomposition
        // dispatched on the same input grounds Track and the Favored Enemy flat
        // surface for real -- see
        // `tests/hybrid_diagnostic_grounded_contradiction.rs`.
        //
        // v0.6 alpha swarm (2026-07-28): the remaining hybrid SPELL diagnostic
        // (`class_spell.hybrid.ranger.unsupported`) has now been retired too, so
        // this set is empty and Human Ranger L1 joins Human Fighter L1 on the
        // golden path. Rangers gain no spellcasting until class level 4 in PF1
        // (`cr_classes.lst`'s `CLASS:Ranger` block has no `CAST:` row at all
        // before level 4), so a level-1 Ranger's absent spell posture is the
        // CORRECT computed answer, not a missing one. See
        // `tests/v06_hybrid_level1_no_spellcasting_is_computed.rs`.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:ranger", 1),
            BTreeSet::new(),
            "Human Ranger L1 now reaches Computed with zero claim-blocking diagnostics: \
             having no spellcasting at level 1 is a satisfied PF1 condition, not a gap"
        );

        // v0.6 alpha swarm, risks item 8, seventh slice (2026-07-25):
        // `table_class_id` now recognizes Barbarian too, so the 4 generic
        // chassis-wide diagnostics no longer trip. The rage-execution
        // burden is also no longer unconditional -- it's a real, conditional
        // engine now (`ground_or_block_barbarian_rage`), and
        // `compose_character_input` seeds no `class_ability_activations`
        // for Barbarian, so the (valid, "not raging") posture no longer
        // trips it either. Human Barbarian L1 reaches Computed with zero
        // claim-blocking diagnostics, the same golden path as Fighter.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:barbarian", 1),
            BTreeSet::new(),
            "Human Barbarian L1 (not raging) reaches Computed with zero claim-blocking \
             diagnostics now that the rage-execution engine is real and conditional"
        );

        // v0.6 alpha swarm, risks item 8, fifth slice (2026-07-25):
        // `table_class_id` now recognizes Sorcerer too, so the 4 generic
        // chassis-wide diagnostics no longer trip. The spell-posture
        // diagnostic is also no longer unconditional -- it's a real
        // validation now, and compose_character_input seeds no Sorcerer
        // spell selections, so the (valid, empty) known-spell posture no
        // longer trips it. The bloodline-power diagnostic used to remain
        // here too, since compose_character_input seeded no bloodline or
        // Arcane Bond choice for this bare fixture. Choice-picker Path A
        // (2026-07-25) now seeds a canonical Arcane bloodline + familiar
        // Arcane Bond choice for every Sorcerer, mirroring Wizard's own
        // starter-spell precedent, so that diagnostic is genuinely cleared
        // too -- Human Sorcerer L1 now reaches Computed with zero
        // claim-blocking diagnostics, the same golden path as Wizard/Bard.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:sorcerer", 1),
            BTreeSet::new(),
            "Human Sorcerer L1 now reaches Computed with zero claim-blocking diagnostics, \
             thanks to the seeded canonical bloodline + arcane bond choice"
        );

        // SD-21 Epic 6 gave Wizard a real compute_wizard_chassis (BAB + saves, via
        // compute_class_chassis's per-class dispatch), so Wizard no longer trips the
        // two chassis-wide generic diagnostics (class_chassis.unsupported,
        // defense.total_save.unsupported) that every other still-unsupported class
        // does. Epic 6b's E6b.1 cycle then widened compute_combat_baseline and
        // compute_selected_skill_modifiers to the same has_supported_class_chassis
        // gate, so those two also no longer trip for Wizard at any input. v0.6 alpha
        // swarm (bootstrap-deadlock fix): compose_character_input now seeds one
        // canonical starter spell (Known+Prepared) for every Wizard, so the
        // previously-remaining class_spell.wizard.prepared_spellbook.unsupported and
        // class_feature.wizard.school_powers_and_opposed_school_cost.unsupported
        // (Epic 6b's E6b.2/E6b.3) are now genuinely cleared by that seeded,
        // in-budget spellbook -- Wizard L1 is now claim-blocking-diagnostic-free,
        // same golden-path shape as Fighter.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:wizard", 1),
            BTreeSet::new(),
            "Human Wizard L1 now reaches Computed with zero claim-blocking diagnostics, thanks \
             to the seeded canonical starter spell"
        );

        // v0.6 alpha swarm, risks item 8, Inspire Courage slice (2026-07-25):
        // `table_class_id` now recognizes Bard too, so the 4 generic
        // chassis-wide diagnostics no longer trip. The bardic-performance-
        // execution burden is also no longer unconditional -- it's a real,
        // conditional engine now (`ground_or_block_bard_bardic_performance`),
        // and `compose_character_input` seeds no `class_ability_activations`
        // for Bard, so the (valid, "not performing") posture no longer trips
        // it either (the "other performances not modeled" diagnostic it
        // pushes unconditionally is deliberately non-claim-blocking, so it
        // never appears in this claim-blocking-only set). The known-spell/
        // per-day spell-posture diagnostic is ALSO no longer unconditional
        // (a later slice made it real/conditional, mirroring Sorcerer's own
        // known-spell closure) -- compose_character_input seeds no Bard
        // spell selections, so the (valid, empty) known-spell posture no
        // longer trips it either. Human Bard L1 now reaches Computed with
        // zero claim-blocking diagnostics, the same golden path as Fighter.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:bard", 1),
            BTreeSet::new(),
            "Human Bard L1 (not performing, valid empty known-spell posture) reaches Computed \
             with zero claim-blocking diagnostics"
        );

        // The SD13-E5 Rogue slice grounds trapfinding, the last named Rogue
        // pillar burden, so the named Rogue set was already empty. v0.6 alpha
        // swarm task 4 then widened compute_class_chassis's per-class
        // dispatch (previously Fighter/Wizard only) to also recognize Rogue,
        // the same way Epic 6 did for Wizard above -- so the 4 generic
        // chassis-wide diagnostics (class_chassis.unsupported,
        // combat.baseline_unsupported, defense.total_save.unsupported,
        // skill.selected_modifier.unsupported) no longer trip for Rogue
        // either. Human Rogue L1 now reaches a fully Computed receipt with
        // zero claim-blocking diagnostics, the same golden-path shape as
        // Fighter.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:rogue", 1),
            BTreeSet::new(),
            "Human Rogue L1 now reaches Computed with zero claim-blocking diagnostics, matching \
             Fighter's golden path, since task 4 widened the generic chassis dispatch to Rogue"
        );

        // v0.6 alpha swarm, choice-picker Path A (Monk's own closure): Monk
        // used to sit here with the 4 generic chassis-wide diagnostics plus
        // its one real feature gap, the level-1 bonus feat -- which fired
        // only because nothing seeded `choice:monk_bonus_feat`, never
        // because the engine couldn't compute it. `compose_character_input`
        // now seeds the canonical `choice:monk_bonus_feat -> feat:dodge`
        // (see `pf1_adapter.rs`'s `DODGE_FEAT_SELECTION` doc comment for why
        // Dodge of the seven corpus options), and the fixed loadout already
        // carries `feat:dodge` on `selected_feats`, so the engine's own
        // genuinely-active cross-check passes and it emits the real
        // `...bonus_feat.dodge_active` grounding record. Human Monk L1 now
        // reaches Computed with zero claim-blocking diagnostics, the same
        // golden path as Fighter/Rogue/Cleric above.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:monk", 1),
            BTreeSet::new(),
            "Human Monk L1 now reaches Computed with zero claim-blocking diagnostics, thanks to \
             the seeded canonical Dodge bonus feat"
        );

        // v0.6 alpha swarm, risks item 8, sixth slice (2026-07-25):
        // `table_class_id` now recognizes Cleric too, so the 4 generic
        // chassis-wide diagnostics no longer trip. The spell-posture
        // diagnostic is also no longer unconditional -- it's a real
        // validation now, and compose_character_input seeds no Cleric
        // spell selections, so the (valid, empty) prepared-spell posture no
        // longer trips it. The domain-powers diagnostic used to remain
        // here too, since compose_character_input seeded no domain choice
        // for this bare fixture. Choice-picker Path A (2026-07-25) now
        // seeds a canonical Good domain choice for every Cleric, mirroring
        // Wizard's own starter-spell precedent, so that diagnostic is
        // genuinely cleared too -- Human Cleric L1 now reaches Computed
        // with zero claim-blocking diagnostics, the same golden path as
        // Sorcerer/Wizard/Bard.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:cleric", 1),
            BTreeSet::new(),
            "Human Cleric L1 now reaches Computed with zero claim-blocking diagnostics, \
             thanks to the seeded canonical Good domain choice"
        );

        // v0.6 alpha swarm, risks item 8, seventh slice (2026-07-25):
        // `table_class_id` now recognizes Druid too, so the 4 generic
        // chassis-wide diagnostics no longer trip. The spell-posture
        // diagnostic is also no longer unconditional -- it's a real
        // validation now, and compose_character_input seeds no Druid
        // spell selections, so the (valid, empty) prepared-spell posture
        // no longer trips it. The animal-companion/nature-bond diagnostic
        // used to remain here too, since compose_character_input seeded no
        // nature-bond choice for this bare fixture. Choice-picker Path A
        // (2026-07-25) now seeds a canonical animal-companion nature-bond
        // choice for every Druid (Wolf is automatic once the bond type is
        // recognized, no species picker needed), so that diagnostic is
        // genuinely cleared too -- Human Druid L1 now reaches Computed
        // with zero claim-blocking diagnostics, the same golden path as
        // Cleric/Sorcerer/Wizard/Bard.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:druid", 1),
            BTreeSet::new(),
            "Human Druid L1 now reaches Computed with zero claim-blocking diagnostics, thanks \
             to the seeded canonical animal-companion nature-bond choice"
        );

        // (v0.6 alpha swarm, risks item 8, third slice, 2026-07-25) This
        // previously proved the Human-only gate collapsed a non-Human Paladin to
        // the 4 generic diagnostics. That's no longer true: `table_class_id`
        // recognizes Paladin regardless of race (real BAB/save/HP via the generic
        // table dispatch), the F6 hybrid diagnostics are Human-gated so they don't
        // fire for a Dwarf, and the real spell-posture check is race-independent
        // and valid (empty) here too -- so a non-Human Paladin L1 now reaches
        // Computed with ZERO claim-blocking diagnostics, same as Fighter/Wizard/
        // Rogue/Ranger's golden path.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:dwarf", "class:paladin", 1),
            BTreeSet::new(),
            "non-Human Paladin L1 now reaches Computed too -- table_class_id and the real \
             spell-posture check are both race-independent"
        );

        // (v0.6 alpha swarm, risks item 8, third slice, 2026-07-25) Previously
        // proved only the spell burden remained claim-blocking at level 2. Now
        // that burden is a real, conditional validation instead of an
        // unconditional blocker, and a level-2 Paladin has no spells accessible
        // yet (access ceiling 0) so there's nothing to violate -- Paladin level 2
        // now reaches Computed with zero claim-blocking diagnostics too.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:paladin", 2),
            BTreeSet::new(),
            "Paladin level 2 now reaches Computed: chassis is grounded, and the real spell \
             posture is valid (no spells accessible yet, none prepared)"
        );
    }

    #[test]
    fn characters_root_from_app_data_dir_joins_characters_subdirectory() {
        let app_data_dir = Path::new("/tmp/example-app-data");
        let root = characters_root_from_app_data_dir(app_data_dir);
        assert_eq!(root, PathBuf::from("/tmp/example-app-data/characters"));
    }

    // ----- `mutate_saved_character` operation table (Criterion 16) -----

    #[test]
    fn saved_character_mutation_operations_table_documents_six_ops_all_wired() {
        let names: Vec<&str> = SAVED_CHARACTER_MUTATION_OPERATIONS
            .iter()
            .map(|descriptor| descriptor.name)
            .collect();
        assert_eq!(
            names,
            vec![
                "level_up_character",
                "add_equipment_selection",
                "add_spell_selection",
                "set_skill_allocations",
                "add_feat_selection",
                "record_and_prepare_spell_selection",
            ],
            "the table must enumerate exactly these six ops, in this order"
        );

        let wired: Vec<&str> = SAVED_CHARACTER_MUTATION_OPERATIONS
            .iter()
            .filter(|descriptor| descriptor.wired)
            .map(|descriptor| descriptor.name)
            .collect();
        assert_eq!(
            wired,
            vec![
                "level_up_character",
                "add_equipment_selection",
                "add_spell_selection",
                "set_skill_allocations",
                "add_feat_selection",
                "record_and_prepare_spell_selection",
            ],
            "all six ops are callable through real Tauri commands as of this cycle"
        );

        for descriptor in SAVED_CHARACTER_MUTATION_OPERATIONS.iter() {
            assert!(
                !descriptor.description.is_empty(),
                "{} must document its mutation semantics",
                descriptor.name
            );
        }
    }

    // ----- `add_equipment_selection` (Criterion 18) -----

    #[test]
    fn apply_add_equipment_selection_appends_to_equipment_selections() {
        let mut input = compose_character_input(&request_for("race:human", 1));
        let starting_len = input.chosen.equipment_selections.len();

        apply_add_equipment_selection(&mut input, "item:dagger", ActiveState::EquippedActive);

        assert_eq!(input.chosen.equipment_selections.len(), starting_len + 1);
        let added = input
            .chosen
            .equipment_selections
            .last()
            .expect("an entry was just pushed");
        assert_eq!(added.item_id, "item:dagger");
        assert_eq!(added.active_state, ActiveState::EquippedActive);
        assert!(added.equipped_or_active);
    }

    /// The single most important regression guard for Criterion 18's
    /// equipment half: proves the real load -> mutate -> recompute ->
    /// re-save -> return round trip against a real `SavedCharacterStore`
    /// fixture on disk, mirroring `level_up_character_at_root`'s own golden
    /// path test.
    #[test]
    fn add_equipment_selection_at_root_appends_and_persists_when_computed() {
        let root = tempdir("add-equipment-golden-path");
        let envelope = level_up_test_envelope("race:human", 1);
        let starting_len = envelope.character_input.chosen.equipment_selections.len();
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = add_equipment_selection_at_root(
            &root,
            "item:dagger",
            ActiveState::EquippedActive,
            "2026-07-21T00:00:00Z",
        )
        .expect("add equipment selection call should not error");

        match response {
            CreateCharacterResponse::Saved { .. } => {}
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!(
                    "Human Fighter level 1 with an added equipment selection must still \
                     reach Computed, got diagnostics: {diagnostics:?}"
                );
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.character_input.chosen.equipment_selections.len(),
            starting_len + 1,
            "the on-disk envelope must reflect the appended equipment selection"
        );
        let added = reloaded
            .character_input
            .chosen
            .equipment_selections
            .last()
            .expect("an entry was just pushed");
        assert_eq!(added.item_id, "item:dagger");
        assert_eq!(added.active_state, ActiveState::EquippedActive);
        assert_eq!(reloaded.saved_at, "2026-07-21T00:00:00Z");

        std::fs::remove_dir_all(&root).ok();
    }

    /// `add_equipment_selection_at_root`'s error path when there is nothing
    /// saved at `root` yet — must fail honestly rather than silently
    /// creating a character out of thin air.
    #[test]
    fn add_equipment_selection_at_root_fails_honestly_when_nothing_is_saved_yet() {
        let root = tempdir("add-equipment-missing-character");

        let result = add_equipment_selection_at_root(
            &root,
            "item:dagger",
            ActiveState::EquippedActive,
            "2026-07-21T00:00:00Z",
        );

        assert!(
            result.is_err(),
            "adding an equipment selection to a nonexistent saved character must fail"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- corpus_derived.equipment_effects (risks item 1, shape (c)) -----

    /// A freshly created Fighter's fixed loadout already equips a real
    /// Chain Shirt (`item:chain_shirt`, `EquippedActive`) -- this proves
    /// the new `corpus_derived.equipment_effects` section carries the real,
    /// corpus-resolved armor-check penalty for it end to end through the
    /// actual creation command, not just a unit test on the lower-level
    /// `compute_pilot_with_corpus` seam. The loadout's only other
    /// `EquippedActive` item is the Longsword -- exactly one weapon, no
    /// equipmods equipped -- so `attack_bonus_delta` must land on the
    /// unambiguous real `Some(0)`, proving the bounded single-weapon slice
    /// end to end too.
    #[test]
    fn create_character_at_root_surfaces_the_real_armor_check_penalty_for_the_fixed_loadout() {
        let root = tempdir("create-character-equipment-effects");
        let request = request_for("race:human", 1);

        let response = create_character_at_root(&root, &request, "test-version".to_owned())
            .expect("create call should not error");

        match response {
            CreateCharacterResponse::Saved { corpus_derived, .. } => {
                assert_ne!(
                    corpus_derived.equipment_effects.armor_check_penalty_total, 0,
                    "the fixed loadout's real Chain Shirt must carry a real, nonzero ACCHECK"
                );
                assert!(
                    corpus_derived.equipment_effects.armor_class_delta > 0,
                    "the fixed loadout's real Chain Shirt must carry a real AC bonus"
                );
                assert_eq!(
                    corpus_derived.equipment_effects.attack_bonus_delta,
                    Some(0),
                    "exactly one weapon (Longsword), no equipmods equipped: real, unambiguous zero"
                );
            }
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!("Human Fighter level 1 must reach Computed, got: {diagnostics:?}")
            }
        }

        std::fs::remove_dir_all(&root).ok();
    }

    /// v0.6 alpha swarm (QA finding, 2026-07-24): a real equipment
    /// selection outside the desktop app's tiny bundled demo corpus
    /// (`corpus_fixtures.rs`, ~4 records total) must be traceable in
    /// `corpus_derived.unresolvedEquipmentItemIds`, not silently vanish --
    /// end to end through the real `add_equipment_selection` command
    /// against the real bundled corpus, not a hand-built fixture.
    #[test]
    fn add_equipment_selection_surfaces_an_unresolvable_item_instead_of_silently_dropping_it() {
        let root = tempdir("create-character-unresolved-equipment");
        let request = request_for("race:human", 1);
        create_character_at_root(&root, &request, "test-version".to_owned())
            .expect("create call should not error");

        let response = add_equipment_selection_at_root(
            &root,
            "Wand of Cure Light Wounds",
            ActiveState::EquippedActive,
            "2026-07-24T00:00:00Z",
        )
        .expect("add call should not error");

        match response {
            CreateCharacterResponse::Saved { corpus_derived, .. } => {
                // The fixed loadout's own `item:shield` (Absent) and
                // `power_attack` (a synthetic feat-toggle id, never a real
                // corpus item) are pre-existing, already-unresolvable
                // entries unrelated to this test -- `contains`, not exact
                // equality, so this doesn't assert on behavior this test
                // isn't about.
                assert!(
                    corpus_derived
                        .unresolved_equipment_item_ids
                        .contains(&"Wand of Cure Light Wounds".to_string()),
                    "an item outside the bundled demo corpus must be traceable, not silently \
                     dropped: {:?}",
                    corpus_derived.unresolved_equipment_item_ids
                );
            }
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!("adding an inert extra item must not block an already-Computed build, got: {diagnostics:?}")
            }
        }

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- create_character: starting wealth (risks item 7) -----

    /// A freshly created Fighter is granted the operator-cited average
    /// starting wealth (175 gp = 17,500 cp) atomically as part of creation,
    /// not as a separate call the caller has to remember to make.
    #[test]
    fn create_character_at_root_grants_the_operator_cited_starting_wealth_for_fighter() {
        let root = tempdir("create-character-starting-wealth-fighter");
        let request = request_for("race:human", 1);

        let response = create_character_at_root(&root, &request, "test-version".to_owned())
            .expect("create call should not error");

        match response {
            CreateCharacterResponse::Saved { .. } => {}
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!("Human Fighter level 1 must reach Computed, got: {diagnostics:?}")
            }
        }

        assert_eq!(
            load_character_money_at_root(&root).unwrap().total_copper,
            17_500,
            "175 gp (5d6 x 10, operator-cited average) = 17,500 cp"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// The other two classes that currently reach `Computed` also get their
    /// own correct, distinct starting wealth -- not a single hardcoded value
    /// applied regardless of class.
    #[test]
    fn create_character_at_root_grants_the_operator_cited_starting_wealth_for_wizard_and_rogue() {
        for (class_id, expected_copper) in [("class:wizard", 7_000_u64), ("class:rogue", 14_000_u64)] {
            let root = tempdir(&format!("create-character-starting-wealth-{class_id}"));
            let request = request_for_class("race:human", class_id, 1);

            let response = create_character_at_root(&root, &request, "test-version".to_owned())
                .expect("create call should not error");

            match response {
                CreateCharacterResponse::Saved { .. } => {}
                CreateCharacterResponse::Blocked { diagnostics } => {
                    panic!("Human {class_id} level 1 must reach Computed, got: {diagnostics:?}")
                }
            }

            assert_eq!(
                load_character_money_at_root(&root).unwrap().total_copper,
                expected_copper,
                "{class_id}'s starting wealth"
            );

            std::fs::remove_dir_all(&root).ok();
        }
    }

    /// A build that does not reach `Computed` must never be granted starting
    /// wealth -- proves the wealth grant is gated on the same successful-save
    /// path as everything else, not a side effect that fires unconditionally.
    ///
    /// **Roster-independent by construction (2026-07-29).** This test used to
    /// name ONE still-blocked class as its fixture, and had to be rewritten
    /// every time that class was closed: Cleric until choice-picker Path A
    /// seeded its canonical domain (2026-07-25), then Oracle until Path A
    /// canonical narrowing seeded its Mystery/Curse pair (2026-07-29). The
    /// churn was the signal -- the invariant under test was never about any
    /// particular class. It now sweeps every class id `money::starting_wealth_gp`
    /// recognizes, asserts the invariant on whichever of them are still
    /// `Blocked`, and carries an explicit non-vacuity guard so it can never
    /// quietly degrade into a no-op once the last one is closed.
    #[test]
    fn create_character_at_root_grants_no_wealth_when_the_build_is_blocked() {
        // Every class id `money::starting_wealth_gp` returns `Some` for, in
        // that function's own match order. Ten of these (Alchemist, Cavalier,
        // Gunslinger, Inquisitor, Magus, Ninja, Oracle, Samurai, Summoner,
        // Witch) are recognized for wealth purposes only -- exactly the
        // population this invariant most needs to hold over.
        let wealth_recognized_class_ids = [
            "class:monk",
            "class:druid",
            "class:sorcerer",
            "class:wizard",
            "class:summoner",
            "class:barbarian",
            "class:bard",
            "class:alchemist",
            "class:oracle",
            "class:samurai",
            "class:witch",
            "class:cleric",
            "class:rogue",
            "class:inquisitor",
            "class:magus",
            "class:ninja",
            "class:fighter",
            "class:paladin",
            "class:ranger",
            "class:cavalier",
            "class:gunslinger",
        ];

        let mut blocked_classes_seen = Vec::new();
        for class_id in wealth_recognized_class_ids {
            assert!(
                codex::rules_core::money::starting_wealth_gp(class_id).is_some(),
                "{class_id} must still be a wealth-recognized id -- if this fails, \
                 starting_wealth_gp's roster changed and this list is stale"
            );

            let root = tempdir(&format!("create-character-starting-wealth-blocked-{class_id}"));
            let request = request_for_class("race:human", class_id, 1);
            let response = create_character_at_root(&root, &request, "test-version".to_owned())
                .expect("create call should not error");

            if let CreateCharacterResponse::Blocked { .. } = response {
                blocked_classes_seen.push(class_id);
                assert_eq!(
                    load_character_money_at_root(&root).unwrap().total_copper,
                    0,
                    "{class_id} is Blocked, so it must never be granted wealth, fabricated or \
                     otherwise"
                );
            }

            std::fs::remove_dir_all(&root).ok();
        }

        assert!(
            !blocked_classes_seen.is_empty(),
            "every wealth-recognized class now reaches Computed, so this test proves nothing \
             about the Blocked path any more -- replace it with a genuinely blocked fixture \
             (e.g. an unsupported multiclass build) rather than deleting the invariant"
        );
    }

    /// v0.6 alpha swarm item 7 (second phase, 2026-07-24), **rewritten
    /// 2026-07-29** (the four spellcasting-shaped classes).
    ///
    /// This test used to assert the OPPOSITE: that Alchemist -- a non-CRB
    /// class id `starting_wealth_gp` recognizes for wealth purposes --
    /// still reached `Blocked`, because "nothing else in the
    /// compute/chassis dispatch has ever heard of `class:alchemist`".
    /// That stopped being true: Alchemist's prepared-extract posture and
    /// its Discovery chooser are both grounded now, and
    /// `compose_character_input` seeds both canonically, so a freshly
    /// created Human Alchemist genuinely reaches `Computed`.
    ///
    /// The guard the old test provided is not lost -- it is the general
    /// case, and `create_character_at_root_grants_no_wealth_when_the_build_is_blocked`
    /// (Oracle) still holds it. What this test now proves is the other
    /// half, and it is the stronger half: the first non-CRB class to
    /// reach `Computed` through the real creation path gets its own
    /// correct, class-distinct starting wealth, and the wealth table's
    /// non-CRB entries are therefore live rather than dead data.
    #[test]
    fn create_character_at_root_grants_the_non_crb_starting_wealth_once_that_class_computes() {
        let root = tempdir("create-character-starting-wealth-non-crb-computed");
        let request = request_for_class("race:human", "class:alchemist", 1);

        let response = create_character_at_root(&root, &request, "test-version".to_owned())
            .expect("create call should not error");

        match response {
            CreateCharacterResponse::Saved { .. } => {}
            CreateCharacterResponse::Blocked { diagnostics } => panic!(
                "Human Alchemist level 1 must now reach Computed from compose_character_input's \
                 own canonical extract/Discovery seeds alone, got: {diagnostics:?}"
            ),
        }

        assert_eq!(
            load_character_money_at_root(&root).unwrap().total_copper,
            10_500,
            "105 gp (3d6 x 10, operator-cited average for Alchemist) = 10,500 cp"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// Real end-to-end regression test for the Arcanist Path A colon-
    /// convention bug (2026-07-25): frontend's own live-testing found that
    /// `compose_character_input`'s seeded `EMPOWER_SPELL_METAMAGIC_SELECTION`
    /// used to be the bare literal `"Empower Spell"` (zero colons), which
    /// `SavedCharacterStore::save`'s own `validate_character_input` call
    /// genuinely rejects -- a real `Err` from this exact
    /// `create_character_at_root` path, past the point where the compute
    /// engine had already reached `Computed`. Backend's own milestone test
    /// (`arcanist_level1_reaches_computed_from_compose_character_input_alone`
    /// in `pf1_adapter.rs`) only exercises the in-memory `CharacterInput`
    /// via `build_pilot_headless_receipt`, never this real save path, so it
    /// could not have caught this. Proves the fix (a `metamagic:`-
    /// namespaced seed, translated back to the literal feat name before the
    /// catalog lookup) by asserting the real save call succeeds AND reaches
    /// `Saved`, not just that it doesn't panic.
    #[test]
    fn create_character_at_root_saves_a_fresh_arcanist_without_a_colon_convention_error() {
        let root = tempdir("create-character-arcanist-colon-convention-fix");
        let request = request_for_class("race:human", "class:arcanist", 1);

        let response = create_character_at_root(&root, &request, "test-version".to_owned())
            .expect(
                "a fresh Human Arcanist 1 must save without error -- this call used to fail with \
                 a colon-segment validation error on the seeded Metamagic Knowledge choice",
            );

        match response {
            CreateCharacterResponse::Saved { .. } => {}
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!(
                    "a fresh Human Arcanist 1 is expected to reach Computed via \
                     compose_character_input's own Path A seeding: {diagnostics:?}"
                )
            }
        }

        std::fs::remove_dir_all(&root).ok();
    }

    /// Real end-to-end proof for the Cavalier/Inquisitor/Oracle Path A
    /// seeding (2026-07-29): each of the three saves through the actual
    /// `create_character_at_root` path, not merely through
    /// `build_pilot_headless_receipt` on an in-memory input. That
    /// distinction is not academic -- the Arcanist colon-convention bug
    /// immediately above was a real failure that lived exactly in the gap
    /// between "the engine reached Computed" and "the save call
    /// succeeded", and every seed here likewise has to survive
    /// `validate_character_input`'s own colon-segment rule.
    #[test]
    fn create_character_at_root_saves_a_fresh_cavalier_inquisitor_and_oracle() {
        for class_id in ["class:cavalier", "class:inquisitor", "class:oracle"] {
            let root = tempdir(&format!("create-character-path-a-{class_id}"));
            let request = request_for_class("race:human", class_id, 1);

            let response = create_character_at_root(&root, &request, "test-version".to_owned())
                .unwrap_or_else(|error| {
                    panic!("a fresh Human {class_id} 1 must save without error: {error:?}")
                });

            match response {
                CreateCharacterResponse::Saved { .. } => {}
                CreateCharacterResponse::Blocked { diagnostics } => panic!(
                    "a fresh Human {class_id} 1 is expected to reach Computed via \
                     compose_character_input's own Path A seeding: {diagnostics:?}"
                ),
            }

            std::fs::remove_dir_all(&root).ok();
        }
    }

    // ----- purchase_equipment: atomic money-purchase coupling (risks item 9) -----

    /// Golden path: an affordable real item is added AND its real catalog
    /// cost is deducted from the balance, atomically, in one call.
    #[test]
    fn purchase_equipment_at_root_succeeds_and_deducts_the_real_catalog_cost() {
        let root = tempdir("purchase-equipment-affordable");
        let envelope = level_up_test_envelope("race:human", 1);
        let starting_len = envelope.character_input.chosen.equipment_selections.len();
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        adjust_character_money_at_root(&root, 1_000).expect("funding the character should succeed");

        let response = purchase_equipment_at_root(
            &root,
            "item:dagger",
            ActiveState::EquippedActive,
            "2026-07-23T00:00:00Z",
        )
        .expect("purchase call should not error");

        match response {
            PurchaseEquipmentResponse::Purchased { money, .. } => {
                // A dagger's real catalog cost is 2 gp = 200 cp.
                assert_eq!(
                    money.total_copper, 800,
                    "1000 cp funded minus a dagger's real 200 cp cost must leave 800 cp"
                );
            }
            PurchaseEquipmentResponse::Blocked { diagnostics } => {
                panic!("an affordable real item must be purchased, got Blocked: {diagnostics:?}")
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.character_input.chosen.equipment_selections.len(),
            starting_len + 1,
            "the equipment must actually be added on the accepted path"
        );
        assert_eq!(load_character_money_at_root(&root).unwrap().total_copper, 800);

        std::fs::remove_dir_all(&root).ok();
    }

    /// An unaffordable item must be honestly rejected -- no equipment
    /// added, no money charged. Proves the pre-flight affordability check
    /// runs BEFORE the equipment mutation, not after.
    #[test]
    fn purchase_equipment_at_root_blocks_and_charges_nothing_when_unaffordable() {
        let root = tempdir("purchase-equipment-unaffordable");
        let envelope = level_up_test_envelope("race:human", 1);
        let starting_len = envelope.character_input.chosen.equipment_selections.len();
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        // Balance starts at 0 cp; a dagger costs 200 cp -- unaffordable.

        let response = purchase_equipment_at_root(
            &root,
            "item:dagger",
            ActiveState::EquippedActive,
            "2026-07-23T00:00:00Z",
        )
        .expect("purchase call should not error");

        match response {
            PurchaseEquipmentResponse::Blocked { diagnostics } => {
                assert!(
                    diagnostics.iter().any(|d| d.id == "money.equipment_purchase.insufficient_funds"),
                    "must carry the real insufficient-funds diagnostic: {diagnostics:?}"
                );
            }
            PurchaseEquipmentResponse::Purchased { .. } => {
                panic!("an unaffordable item must never be silently purchased")
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.character_input.chosen.equipment_selections.len(),
            starting_len,
            "nothing should have been added on the rejected path"
        );
        assert_eq!(
            load_character_money_at_root(&root).unwrap().total_copper,
            0,
            "nothing should have been charged on the rejected path"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// An item with no known catalog cost (a `(Base)` template record) must
    /// be treated the same as unaffordable -- never a free item.
    #[test]
    fn purchase_equipment_at_root_blocks_an_item_with_no_known_cost() {
        let root = tempdir("purchase-equipment-unknown-cost");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        adjust_character_money_at_root(&root, 1_000_000).expect("funding should succeed");

        // The bare "Dagger" KEY resolves to the (Base) template record,
        // which carries no independent cost_gp (None) -- a genuine corpus
        // absence, not zero.
        let response = purchase_equipment_at_root(
            &root,
            "Dagger",
            ActiveState::EquippedActive,
            "2026-07-23T00:00:00Z",
        )
        .expect("purchase call should not error");

        match response {
            PurchaseEquipmentResponse::Blocked { diagnostics } => {
                assert!(
                    diagnostics.iter().any(|d| d.id == "money.equipment_purchase.unknown_cost"),
                    "must carry the real unknown-cost diagnostic: {diagnostics:?}"
                );
            }
            PurchaseEquipmentResponse::Purchased { .. } => {
                panic!("an item with no known cost must never be treated as free")
            }
        }

        assert_eq!(
            load_character_money_at_root(&root).unwrap().total_copper,
            1_000_000,
            "nothing should have been charged when cost is unknown"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- `attach_equipment_modifier` (v0.6 alpha swarm items 1+27 sub-task 6) -----

    /// A real, known-but-formula-priced modifier (an actual magical weapon
    /// enhancement -- the headline use case this command exists for)
    /// attaches for free, since its real `cost_gp` is unknown (a formula,
    /// not a flat catalog price) -- the deliberate deviation from
    /// `purchase_equipment`'s block-on-unknown-cost rule this command's
    /// own doc comment explains.
    #[test]
    fn attach_equipment_modifier_at_root_attaches_a_real_enhancement_for_free_when_cost_is_unknown() {
        let root = tempdir("attach-modifier-free-enhancement");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        // Balance starts at 0 cp -- if this were charged at all, it would
        // be Blocked as unaffordable, proving the free-attach path for
        // real.

        let response = attach_equipment_modifier_at_root(
            &root,
            "item:longsword",
            "Special Ability ~ +1 ~ Weapon",
            "2026-07-23T00:00:00Z",
        )
        .expect("attach call should not error");

        match response {
            AttachEquipmentModifierResponse::Attached { money, .. } => {
                assert_eq!(money.total_copper, 0, "an unknown-cost modifier must attach for free");
            }
            AttachEquipmentModifierResponse::Blocked { diagnostics } => {
                panic!("a real enhancement with unknown cost must attach for free, got Blocked: {diagnostics:?}")
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        let longsword = reloaded
            .character_input
            .chosen
            .equipment_selections
            .iter()
            .find(|selection| selection.item_id == "item:longsword")
            .expect("item:longsword must still be present");
        assert_eq!(
            longsword.applied_modifiers,
            vec!["Special Ability ~ +1 ~ Weapon".to_string()],
            "the modifier must attach to the target selection's applied_modifiers, not a new top-level entry"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// A modifier with a real, known `cost_gp` is still charged (not
    /// treated as free just because it's a modifier) -- proves the
    /// free-attach path is specifically for unknown cost, not blanket.
    #[test]
    fn attach_equipment_modifier_at_root_charges_a_modifier_with_a_known_cost() {
        let root = tempdir("attach-modifier-known-cost");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        adjust_character_money_at_root(&root, 10_000).expect("funding should succeed");

        // Masterwork (Item) resolves a real, known cost_gp of 50.
        let response = attach_equipment_modifier_at_root(
            &root,
            "item:longsword",
            "Special Quality ~ Masterwork ~ Item",
            "2026-07-23T00:00:00Z",
        )
        .expect("attach call should not error");

        match response {
            AttachEquipmentModifierResponse::Attached { money, .. } => {
                assert_eq!(money.total_copper, 5_000, "10,000 cp minus a real 5,000 cp (50 gp) cost");
            }
            AttachEquipmentModifierResponse::Blocked { diagnostics } => {
                panic!("an affordable known-cost modifier must attach, got Blocked: {diagnostics:?}")
            }
        }

        std::fs::remove_dir_all(&root).ok();
    }

    /// A known-cost modifier the character cannot afford is `Blocked`,
    /// same as any other charged mutation -- the free-attach deviation
    /// only applies to genuinely unknown cost, never to unaffordability.
    #[test]
    fn attach_equipment_modifier_at_root_blocks_a_known_cost_modifier_when_unaffordable() {
        let root = tempdir("attach-modifier-unaffordable");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        // Balance starts at 0 cp; Masterwork (Item) costs 5,000 cp.

        let response = attach_equipment_modifier_at_root(
            &root,
            "item:longsword",
            "Special Quality ~ Masterwork ~ Item",
            "2026-07-23T00:00:00Z",
        )
        .expect("attach call should not error");

        match response {
            AttachEquipmentModifierResponse::Blocked { diagnostics } => {
                assert!(
                    diagnostics
                        .iter()
                        .any(|d| d.id == "money.equipment_attach_modifier.insufficient_funds"),
                    "must carry the real insufficient-funds diagnostic: {diagnostics:?}"
                );
            }
            AttachEquipmentModifierResponse::Attached { .. } => {
                panic!("an unaffordable modifier must never be silently attached")
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        let longsword = reloaded
            .character_input
            .chosen
            .equipment_selections
            .iter()
            .find(|selection| selection.item_id == "item:longsword")
            .expect("item:longsword must still be present");
        assert!(longsword.applied_modifiers.is_empty(), "nothing should have attached on the rejected path");

        std::fs::remove_dir_all(&root).ok();
    }

    /// A `modifier_item_id` that is not a real catalog item at all must be
    /// rejected before any cost/target check -- never silently attached.
    #[test]
    fn attach_equipment_modifier_at_root_rejects_an_unknown_modifier_item() {
        let root = tempdir("attach-modifier-unknown-item");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        adjust_character_money_at_root(&root, 1_000_000).expect("funding should succeed");

        let response = attach_equipment_modifier_at_root(
            &root,
            "item:longsword",
            "totally-fabricated-modifier-id",
            "2026-07-23T00:00:00Z",
        )
        .expect("attach call should not error");

        match response {
            AttachEquipmentModifierResponse::Blocked { diagnostics } => {
                assert!(
                    diagnostics.iter().any(|d| d.id == "equipment.attach_modifier.unknown_item"),
                    "must carry the real unknown-item diagnostic: {diagnostics:?}"
                );
            }
            AttachEquipmentModifierResponse::Attached { .. } => {
                panic!("a fabricated modifier id must never be silently attached")
            }
        }

        assert_eq!(
            load_character_money_at_root(&root).unwrap().total_copper,
            1_000_000,
            "nothing should have been charged when the modifier item itself is unrecognized"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// A target `item_id` that does not exist among the character's
    /// `equipment_selections` must be `Blocked` before any charge -- a
    /// not-found target is never silently a free no-op after money moves.
    #[test]
    fn attach_equipment_modifier_at_root_rejects_a_target_that_does_not_exist() {
        let root = tempdir("attach-modifier-missing-target");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        adjust_character_money_at_root(&root, 1_000_000).expect("funding should succeed");

        let response = attach_equipment_modifier_at_root(
            &root,
            "item:greatsword",
            "Special Ability ~ +1 ~ Weapon",
            "2026-07-23T00:00:00Z",
        )
        .expect("attach call should not error");

        match response {
            AttachEquipmentModifierResponse::Blocked { diagnostics } => {
                assert!(
                    diagnostics.iter().any(|d| d.id == "equipment.attach_modifier.target_not_found"),
                    "must carry the real target-not-found diagnostic: {diagnostics:?}"
                );
            }
            AttachEquipmentModifierResponse::Attached { .. } => {
                panic!("attaching to a nonexistent target must never silently succeed")
            }
        }

        assert_eq!(
            load_character_money_at_root(&root).unwrap().total_copper,
            1_000_000,
            "nothing should have been charged when the target selection does not exist"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// End-to-end proof that an attached-but-corpus-unresolvable modifier
    /// (the desktop app's bundled demo corpus has zero equipmods records)
    /// surfaces honestly through the real `corpusDerived` response rather
    /// than silently vanishing -- closes the loop this sub-task's
    /// `unresolved_equipment_item_ids` extension exists for.
    #[test]
    fn attach_equipment_modifier_at_root_surfaces_the_real_corpus_resolution_gap() {
        let root = tempdir("attach-modifier-corpus-gap");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = attach_equipment_modifier_at_root(
            &root,
            "item:longsword",
            "Special Ability ~ +1 ~ Weapon",
            "2026-07-23T00:00:00Z",
        )
        .expect("attach call should not error");

        match response {
            AttachEquipmentModifierResponse::Attached { corpus_derived, .. } => {
                assert!(
                    corpus_derived
                        .unresolved_equipment_item_ids
                        .contains(&"Special Ability ~ +1 ~ Weapon".to_string()),
                    "the bundled demo corpus has no equipmods records, so the attached modifier \
                     must be traceable as unresolved rather than silently inert: {corpus_derived:?}"
                );
            }
            AttachEquipmentModifierResponse::Blocked { diagnostics } => {
                panic!("the attach itself must succeed even though the modifier won't resolve, got Blocked: {diagnostics:?}")
            }
        }

        std::fs::remove_dir_all(&root).ok();
    }

    /// Same wire-shape proof as `purchase_equipment`'s own precedent test:
    /// the `corpusDerived` field must be camelCase (a per-field rename,
    /// not an enum-wide `rename_all`, which would also lowercase the
    /// `"Attached"`/`"Blocked"` tag values themselves).
    #[test]
    fn attach_equipment_modifier_response_attached_serializes_corpus_derived_as_camel_case_without_touching_the_tag()
    {
        let response = AttachEquipmentModifierResponse::Attached {
            summary: Box::new(CharacterSummaryDto {
                character_id: "c".to_owned(),
                display_label: "d".to_owned(),
                game_system: "pf1".to_owned(),
                schema_version: 1,
                saved_at: "2026-07-23T00:00:00Z".to_owned(),
                race_id: "race:human".to_owned(),
                class_summary: "class:fighter:1".to_owned(),
            }),
            snapshot: PilotSnapshotDto {
                ability_modifiers: AbilityModifiersDto {
                    strength: 0,
                    dexterity: 0,
                    constitution: 0,
                    intelligence: 0,
                    wisdom: 0,
                    charisma: 0,
                },
                base_attack_bonus: 0,
                base_saves: BaseSavesDto { fortitude: 0, reflex: 0, will: 0 },
                baseline_melee_attack_bonus: 0,
                baseline_armor_class: 0,
                total_saves: BaseSavesDto { fortitude: 0, reflex: 0, will: 0 },
                selected_skill_modifiers: SelectedSkillModifiersDto {
                    climb: 0,
                    intimidate: 0,
                    swim: 0,
                },
                damage_reduction: None,
                companion: None,
                spellbook: None,
            },
            corpus_derived: CorpusDerivedDto {
                school_coverage: Vec::new(),
                equipped_items: Vec::new(),
                equipment_effects: EquipmentEffectsDto {
                    per_item: Vec::new(),
                    armor_class_delta: 0,
                    armor_check_penalty_total: 0,
                    max_dex_cap: None,
                    spell_failure_chance: None,
                    attack_bonus_delta: None,
                    spell_resistance_total: None,
                },
                encumbrance: empty_encumbrance_dto(),
                unresolved_spell_ids: Vec::new(),
                unresolved_equipment_item_ids: Vec::new(),
            },
            money: money_dto_from_total(0),
        };

        let value = serde_json::to_value(&response).expect("response should serialize");
        let object = value.as_object().expect("response should serialize as a JSON object");

        assert_eq!(object.get("kind").and_then(|v| v.as_str()), Some("Attached"));
        assert!(object.contains_key("corpusDerived"), "{object:?}");
        assert!(!object.contains_key("corpus_derived"), "{object:?}");
    }

    // ----- SD-27: the Attach Modifier dead-affordance invariant -----
    //
    // 57 rows was the reported figure; the real, derived figure is 105
    // (ACG 48 + ARG 15 + PU 42) -- ACG's own equipmods were refused by the
    // same CRB-only check and had been missed. Every count in this section
    // was produced by running the catalog, never taken from a brief.

    /// The offered set: exactly what the Gear tab's "Attach Modifier"
    /// picker asks the backend for -- `buildItemPickerConfig`'s `modifier`
    /// branch calls `listEquipment({ category: 'Equipmods' })` and sends
    /// the chosen row's `key` as `modifierItemId`.
    fn offered_modifier_rows() -> Vec<crate::equipment_catalog::EquipmentCatalogEntryDto> {
        crate::equipment_catalog::filter_equipment_catalog(
            &crate::equipment_catalog::EquipmentCatalogFilter {
                name_contains: None,
                category: Some("Equipmods".to_owned()),
                book: None,
            },
        )
        .entries
    }

    /// **The invariant that stops this defect class returning: every row
    /// the picker OFFERS is a row attach ACCEPTS.**
    ///
    /// This is a pure check of the same `equipment_catalog_row_by_key`
    /// recognition gate `attach_equipment_modifier_at_root` runs (calling
    /// the full command 763 times would be a save+recompute per row); the
    /// end-to-end tests below then drive the real command for one row from
    /// each of the newly reachable books plus a CRB control.
    #[test]
    fn every_equipmods_row_the_picker_offers_is_recognized_by_the_attach_gate() {
        let offered = offered_modifier_rows();
        // 950 + UPsi's 113 real, alias-excluded equipmods (SD28 item 5 --
        // `up_equipmods.lst`'s `EQUIPMODS_TABLE`, already the corrected
        // count with the 113 `VISIBLE:NO` `.COPY=` legacy aliases excluded
        // at the table's own source; UM contributes 0, it has no
        // equipment-modifier file at all) + UC's 19 real, alias-excluded
        // equipmods (39 raw `uc_equipmods.lst` lines minus 20 VISIBLE:NO
        // .COPY= legacy aliases, the identical hazard UPsi's own table
        // found).
        // SD-29 `epic-4-proven-equip-mod`: +584 corpus gap-lane Equipmods rows
        // (CRB 332 + UPsi 113 + ACG 48 + APG 37 + UC 20 + ARG 14 + UE 10 + UI 7
        // + UW 3), every one of them an `equipment_modifier` unit
        // `docs/work-inventory.json` reported `engine-does-not-hold` until this cycle.
        // The point of this test is the assertion BELOW, not this count: an
        // offered row the attach gate refuses is a dead affordance, and 584
        // newly offered rows is 584 new chances to ship one.
        // `SD31-E6-F10-003`: +17 further corpus gap-lane Equipmods rows,
        // across 8 further already-compiled books (1666 -> 1683).
        // `SD31-E6-F10-004`: +148 further corpus gap-lane Equipmods rows,
        // across 5 further already-compiled books (1683 -> 1831),
        // re-derived fresh from the built picker, not adjusted by delta.
        // Row-19 desktop reach/catalog reds (SD-32, 2026-08-24): +63 more,
        // re-derived fresh from the built picker again (not adjusted by
        // delta, same discipline as the prior re-derivation) after the T12
        // census/class-feature lanes' corpus growth (1831 -> 1894). The
        // assertion this test exists for is the `refused` check below, not
        // this count -- it still runs against the fresh 1894 and still
        // passes empty.
        assert_eq!(offered.len(), 1894, "the picker's real offered-row count");

        let refused: Vec<&str> = offered
            .iter()
            .filter(|entry| {
                codex::rules_core::equipment_resolver::equipment_catalog_row_by_key(&entry.key)
                    .is_none()
            })
            .map(|entry| entry.key.as_str())
            .collect();

        assert!(
            refused.is_empty(),
            "{} of {} offered modifier rows would be refused as 'not a recognized equipment \
             catalog item' -- a dead affordance. First offenders: {:?}",
            refused.len(),
            offered.len(),
            refused.iter().take(5).collect::<Vec<_>>()
        );
    }

    /// Recognition alone is only half the fix: 20 of the 105 newly
    /// recognized rows carry a real, non-zero flat price, and attaching
    /// those for free would be silent mispricing -- strictly worse than the
    /// honest refusal it replaces. This pins that **every** newly reachable
    /// row charges exactly the price the picker displayed for it.
    ///
    /// It also pins, rather than hides, the places that are *not* true.
    /// Two CRB rows -- `Holy Symbol (Wooden)` and `Holy Symbol (Silver)` --
    /// carry the same `KEY:` in two different categories, so the row the
    /// `Equipmods` picker displays (1 gp / 25 gp) is not the first row in
    /// the full table for that key (which has no cost, and so attaches
    /// free). That is **pre-existing shipped behaviour**: the CRB-only
    /// `equipment_cost_gp_headless_resolve` this gate replaced took the
    /// first full-table match too, and produced the identical answer. It is
    /// a real, separate defect in `crb::equipment_tables`'s 316 duplicate
    /// keys, not fixable from this file -- disambiguating it means changing
    /// the `key` the catalog puts on the wire.
    ///
    /// SD28-E25 adds a third, of the identical shape: `Masterwork Tool` is
    /// both a real purchasable item (`ultimate_equipment::equipment_tables`'s
    /// own `General` category, 50 gp) and a real equipment modifier
    /// (`Equipmods`, no flat cost -- a `%CHOICE circumstance Bonus`),
    /// sharing a `KEY:`. `equipment_catalog_rows()` chains UE's equipment
    /// before its equipmods, so the resolver's first match is the 50 gp
    /// item, not the free modifier the picker displays -- the same
    /// first-match-wins divergence CRB's Holy Symbol pair already
    /// demonstrates, from a genuine same-book same-key collision this
    /// widening did not create (`equipment_catalog.rs`'s own
    /// `keys_do_not_collide_across_books_and_crbs_own_duplicates_are_pinned`
    /// pins the same fact).
    #[test]
    fn every_offered_modifier_row_charges_the_price_the_picker_displayed() {
        let offered = offered_modifier_rows();
        let mut divergent: Vec<(&str, &str)> = Vec::new();
        let mut priced_non_crb = 0usize;

        for entry in &offered {
            let row = codex::rules_core::equipment_resolver::equipment_catalog_row_by_key(&entry.key)
                .expect("recognition is pinned by the test above");

            // Compare against the FIRST offered row for this key: CRB
            // carries 316 duplicate keys, so the filtered list holds both
            // members of each pair and only the first is reachable by key.
            let displayed = offered
                .iter()
                .find(|candidate| candidate.key == entry.key)
                .expect("the key came from this very list")
                .cost_gp;

            if row.cost_gp != displayed {
                divergent.push((row.book, entry.key.as_str()));
            }
            if row.book != "CRB" && row.cost_gp.unwrap_or(0.0) > 0.0 {
                priced_non_crb += 1;
            }
        }

        divergent.sort_unstable();
        divergent.dedup();
        assert_eq!(
            divergent,
            vec![
                ("CRB", "Holy Symbol (Silver)"),
                ("CRB", "Holy Symbol (Wooden)"),
                ("UE", "Masterwork Tool"),
            ],
            "the display-vs-charge divergence set must stay exactly these three same-book \
             same-key rows -- named explicitly so a fourth arriving silently fails here"
        );
        assert!(
            divergent.iter().all(|(book, _)| *book == "CRB" || *book == "UE"),
            "the widening must not add a display-vs-charge divergence outside the two named books"
        );
        assert_eq!(
            priced_non_crb, 258,
            "RAISED 181 -> 258, `SD31-E6-F10-004`, 2026-08-17: 5 further already-compiled books \
             extended into the corpus gap lane, re-derived fresh from the built picker, not \
             adjusted by delta. The divergence-set assertion above is unchanged (still exactly \
             the same 3 named CRB/UE rows) -- none of this cycle's own new priced rows introduced \
             a new display-vs-charge divergence.\n\
             RAISED 137 -> 181, `SD31-E6-F6-001`, 2026-08-16: `gen_equipment_gap_tables.rs` \
             gained `.COPY=` inheritance for `cost_gp` (a `.COPY=` row with no `COST:` of its own \
             now inherits its base record's real one, resolved by the identical `KEY:`-or-bare- \
             name identity a `.COPY=` reference itself resolves against) -- 209 corpus-wide gap \
             rows recovered a real, non-fabricated `cost_gp` this cycle (verified one record deep \
             against the pinned oracle, e.g. `BOWSTR` inherits `COST:0` from `cr_equipmods.lst:34`, \
             `Amorphous` inherits `COST:4500` from `acg_equipmods.lst:10`, automating what a prior \
             cycle's 8-row ACG hand-patch did manually). The +44 delta from 137 is the subset of \
             those 209 that are (a) non-CRB, (b) genuinely non-zero-priced, and (c) reachable \
             through `offered_modifier_rows()`'s own filtering -- re-derived fresh by running this \
             test with `--nocapture`, not computed by arithmetic on the prior formula, since the \
             prior formula's own additive shape (20+69+1+24+2+13+8) does not decompose cleanly \
             against a corpus-wide mechanism change touching every book at once."
        );
    }

    /// The rules-core book codes and the desktop catalog's book codes must
    /// stay the same set: a book present in one and not the other is
    /// silently either an unofferable row or an unrecognizable one.
    #[test]
    fn the_catalog_and_the_resolver_agree_on_the_book_set() {
        use codex::rules_core::equipment_resolver::equipment_catalog_rows;
        let resolver_books: std::collections::BTreeSet<&str> =
            equipment_catalog_rows().iter().map(|row| row.book).collect();
        let catalog_books: std::collections::BTreeSet<&str> =
            crate::equipment_catalog::equipment_catalog_books().into_iter().collect();
        assert_eq!(resolver_books, catalog_books);
    }

    /// End-to-end through the real command: ARG's `Material ~ Whipwood`
    /// was refused on screen as unrecognized. It must now attach **and be
    /// charged its real `arg_equipmods.lst` `COST:500`** -- 50,000 cp.
    /// Attaching it for free would be silent mispricing, strictly worse
    /// than the refusal this replaces.
    #[test]
    fn a_previously_refused_arg_modifier_attaches_and_is_charged_its_real_corpus_cost() {
        let root = tempdir("attach-modifier-arg-whipwood");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        adjust_character_money_at_root(&root, 60_000).expect("funding should succeed");

        let response = attach_equipment_modifier_at_root(
            &root,
            "item:longsword",
            "Material ~ Whipwood",
            "2026-07-31T00:00:00Z",
        )
        .expect("attach call should not error");

        match response {
            AttachEquipmentModifierResponse::Attached { money, .. } => {
                assert_eq!(
                    money.total_copper, 10_000,
                    "60,000 cp minus ARG's real 500 gp (50,000 cp) Whipwood price"
                );
            }
            AttachEquipmentModifierResponse::Blocked { diagnostics } => {
                panic!("an offered ARG modifier must attach, got Blocked: {diagnostics:?}")
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.character_input.chosen.equipment_selections[0].applied_modifiers,
            vec!["Material ~ Whipwood".to_string()],
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// The affordability path works for the newly reachable books too: the
    /// same ARG modifier is `Blocked` -- with the real price in the
    /// message, not a recognition error -- when the character cannot
    /// afford it, and nothing is charged or attached.
    #[test]
    fn a_previously_refused_arg_modifier_blocks_on_price_when_unaffordable() {
        let root = tempdir("attach-modifier-arg-unaffordable");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        adjust_character_money_at_root(&root, 3_500).expect("funding should succeed");

        let response = attach_equipment_modifier_at_root(
            &root,
            "item:longsword",
            "Material ~ Whipwood",
            "2026-07-31T00:00:00Z",
        )
        .expect("attach call should not error");

        match response {
            AttachEquipmentModifierResponse::Blocked { diagnostics } => {
                assert!(
                    diagnostics
                        .iter()
                        .any(|d| d.id == "money.equipment_attach_modifier.insufficient_funds"),
                    "must fail on price, not on recognition: {diagnostics:?}"
                );
                assert!(
                    diagnostics.iter().any(|d| d.message.contains("50000 cp")),
                    "the real ARG price must appear in the message: {diagnostics:?}"
                );
            }
            AttachEquipmentModifierResponse::Attached { .. } => {
                panic!("an unaffordable modifier must never attach")
            }
        }

        assert_eq!(load_character_money_at_root(&root).unwrap().total_copper, 3_500);
        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert!(reloaded.character_input.chosen.equipment_selections[0]
            .applied_modifiers
            .is_empty());

        std::fs::remove_dir_all(&root).ok();
    }

    /// End-to-end through the real command: PU's `ABP ~ +3 Attunement ~
    /// Armor` was refused on screen. `pu_equipmods.lst` carries no `COST:`
    /// token on any of its 42 rows, so this attaches free -- and that is
    /// the corpus truth, identical to how CRB's own formula-priced `+1`
    /// enhancement has always behaved, not a fabricated zero.
    #[test]
    fn a_previously_refused_pu_modifier_attaches_free_because_its_corpus_row_has_no_cost_token() {
        let root = tempdir("attach-modifier-pu-abp");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        // Balance stays at 0 cp: any charge at all would Block here, so a
        // successful attach proves the free path for real.

        let response = attach_equipment_modifier_at_root(
            &root,
            "item:longsword",
            "ABP ~ +3 Attunement ~ Armor",
            "2026-07-31T00:00:00Z",
        )
        .expect("attach call should not error");

        match response {
            AttachEquipmentModifierResponse::Attached { money, .. } => {
                assert_eq!(money.total_copper, 0);
            }
            AttachEquipmentModifierResponse::Blocked { diagnostics } => {
                panic!("an offered PU modifier must attach, got Blocked: {diagnostics:?}")
            }
        }

        assert_eq!(
            codex::rules_core::equipment_resolver::equipment_catalog_row_by_key(
                "ABP ~ +3 Attunement ~ Armor"
            )
            .and_then(|row| row.cost_gp),
            None,
            "free because the corpus row genuinely has no COST: token, not because the price \
             lookup failed to find the row"
        );

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.character_input.chosen.equipment_selections[0].applied_modifiers,
            vec!["ABP ~ +3 Attunement ~ Armor".to_string()],
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// **CRB control.** The modifier that was already correctly failing on
    /// price still fails on price, with the identical diagnostic id and the
    /// identical 100,000 cp figure observed on screen before the change.
    #[test]
    fn the_crb_control_modifier_still_blocks_on_exactly_the_same_price() {
        let root = tempdir("attach-modifier-crb-control");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        adjust_character_money_at_root(&root, 3_500).expect("funding should succeed");

        let response = attach_equipment_modifier_at_root(
            &root,
            "item:longsword",
            "Material ~ Mithril ~ Armor / Light",
            "2026-07-31T00:00:00Z",
        )
        .expect("attach call should not error");

        match response {
            AttachEquipmentModifierResponse::Blocked { diagnostics } => {
                assert!(
                    diagnostics
                        .iter()
                        .any(|d| d.id == "money.equipment_attach_modifier.insufficient_funds"),
                    "{diagnostics:?}"
                );
                assert!(
                    diagnostics.iter().any(|d| d.message.contains(
                        "costs 100000 cp but the character's balance is only 3500 cp"
                    )),
                    "the CRB row's message must be byte-identical to the pre-change behaviour: \
                     {diagnostics:?}"
                );
            }
            AttachEquipmentModifierResponse::Attached { .. } => {
                panic!("the CRB control must still block on price")
            }
        }

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- `add_feat_selection` (v0.6 alpha swarm) -----

    #[test]
    fn apply_add_feat_selection_appends_to_selected_feats() {
        let mut input = compose_character_input(&request_for("race:human", 1));
        let starting_len = input.chosen.selected_feats.len();

        apply_add_feat_selection(&mut input, "feat:toughness");

        assert_eq!(input.chosen.selected_feats.len(), starting_len + 1);
        assert_eq!(
            input.chosen.selected_feats.last(),
            Some(&"feat:toughness".to_owned())
        );
    }

    /// Mirrors `add_spell_selection_at_root_appends_and_persists_when_computed`'s
    /// golden-path shape below.
    #[test]
    fn add_feat_selection_at_root_appends_and_persists_when_computed() {
        let root = tempdir("add-feat-golden-path");
        let envelope = level_up_test_envelope("race:human", 1);
        let starting_len = envelope.character_input.chosen.selected_feats.len();
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = add_feat_selection_at_root(&root, "feat:toughness", None, "2026-07-21T00:00:00Z")
            .expect("add feat selection call should not error");

        match response {
            CreateCharacterResponse::Saved { .. } => {}
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!(
                    "Human Fighter level 1 with an added feat selection must still reach \
                     Computed, got diagnostics: {diagnostics:?}"
                );
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(reloaded.character_input.chosen.selected_feats.len(), starting_len + 1);
        assert_eq!(
            reloaded.character_input.chosen.selected_feats.last(),
            Some(&"feat:toughness".to_owned())
        );
        assert_eq!(reloaded.saved_at, "2026-07-21T00:00:00Z");

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn add_feat_selection_at_root_fails_honestly_when_nothing_is_saved_yet() {
        let root = tempdir("add-feat-missing-character");

        let result = add_feat_selection_at_root(&root, "feat:toughness", None, "2026-07-21T00:00:00Z");

        assert!(
            result.is_err(),
            "adding a feat selection to a nonexistent saved character must fail"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- SD-27: feat prerequisite enforcement -----

    /// The operator's exact reported defect, at the mutation boundary:
    /// **a Fighter 1 was allowed to take Improved Two-Weapon Fighting.**
    /// The guard must refuse, name every unmet prerequisite, and leave the
    /// saved character untouched.
    #[test]
    fn a_fighter_1_is_refused_improved_two_weapon_fighting_with_the_reasons() {
        let root = tempdir("add-feat-prereq-refused");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        let before = SavedCharacterStore::load(&root).unwrap().character_input.chosen.selected_feats;

        let error = add_feat_selection_enforcing_prerequisites_at_root(
            &root,
            "Improved Two-Weapon Fighting",
            None,
            "2026-07-31T00:00:00Z",
        )
        .expect_err("a Fighter 1 must not be able to take Improved Two-Weapon Fighting");

        assert!(error.contains("Improved Two-Weapon Fighting"), "{error}");
        assert!(error.contains("base attack bonus +6"), "{error}");
        assert!(error.contains("Two-Weapon Fighting feat"), "{error}");
        assert!(error.contains("DEX 17"), "{error}");

        let after = SavedCharacterStore::load(&root).unwrap().character_input.chosen.selected_feats;
        assert_eq!(before, after, "a refused feat must not be written to disk");

        std::fs::remove_dir_all(&root).ok();
    }

    /// ...and a build that legitimately qualifies still goes through the
    /// same guarded path. A guard that refuses everything is not
    /// enforcement.
    #[test]
    fn a_qualified_fighter_6_is_allowed_improved_two_weapon_fighting() {
        let root = tempdir("add-feat-prereq-allowed");
        let mut envelope = level_up_test_envelope("race:human", 6);
        envelope.character_input.chosen.ability_scores.dexterity = 17;
        envelope
            .character_input
            .chosen
            .selected_feats
            .push("Two-Weapon Fighting".to_owned());
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        add_feat_selection_enforcing_prerequisites_at_root(
            &root,
            "Improved Two-Weapon Fighting",
            None,
            "2026-07-31T00:00:00Z",
        )
        .expect("a BAB +6 / Dex 17 / TWF fighter qualifies");

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert!(reloaded
            .character_input
            .chosen
            .selected_feats
            .iter()
            .any(|feat| feat == "Improved Two-Weapon Fighting"));

        std::fs::remove_dir_all(&root).ok();
    }

    /// A prerequisite-free feat must still be addable -- the guard must not
    /// have become a blanket refusal.
    #[test]
    fn a_feat_with_no_prerequisites_is_still_added_through_the_guard() {
        let root = tempdir("add-feat-prereq-free");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        add_feat_selection_enforcing_prerequisites_at_root(
            &root,
            "feat:toughness",
            None,
            "2026-07-31T00:00:00Z",
        )
        .expect("Toughness has no corpus prerequisites");

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert!(reloaded
            .character_input
            .chosen
            .selected_feats
            .iter()
            .any(|feat| feat == "feat:toughness"));

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- Selection removal -----

    /// The reported defect, end to end: a feat that moves a real number is
    /// added, the number moves, the feat is removed, and the number goes
    /// back — **read from a fresh load off disk**, not from the mutation's
    /// own response, so a removal that left a stale computed value on disk
    /// would fail here.
    ///
    /// Toughness is the probe because its effect is a plain arithmetic
    /// change to max HP (`+1` per character level, minimum 3), so "the
    /// number moved" is checkable rather than a matter of interpretation.
    #[test]
    fn removing_a_feat_returns_the_number_it_moved_and_survives_reload() {
        let root = tempdir("remove-feat-round-trip");
        let envelope = level_up_test_envelope("race:human", 3);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        // Read through the exact command the Defense tab reads through, off
        // a fresh load, so this asserts the number a player actually sees.
        let max_hp_now = |root: &Path| -> i16 {
            load_character_durability_at_root(root)
                .expect("durability should compute for a single-class Fighter")
                .max_hp
        };

        let baseline = max_hp_now(&root);

        add_feat_selection_enforcing_prerequisites_at_root(
            &root,
            "Toughness",
            None,
            "2026-08-01T00:00:00Z",
        )
        .expect("Toughness has no prerequisites");
        let with_feat = max_hp_now(&root);
        assert!(
            with_feat > baseline,
            "Toughness must move max HP for the removal test to mean anything: \
             baseline {baseline}, with feat {with_feat}"
        );

        remove_feat_selection_at_root(&root, "Toughness", None, "2026-08-01T00:01:00Z")
            .expect("removing a held feat with no dependents must succeed");

        let after_removal = max_hp_now(&root);
        assert_eq!(
            after_removal, baseline,
            "removing Toughness must return max HP to its pre-feat value"
        );
        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert!(
            !reloaded
                .character_input
                .chosen
                .selected_feats
                .iter()
                .any(|feat| feat == "Toughness"),
            "the removed feat must be gone from the persisted character"
        );
        assert_eq!(reloaded.saved_at, "2026-08-01T00:01:00Z");

        std::fs::remove_dir_all(&root).ok();
    }

    /// The dependency guard, on a real corpus chain: Great Cleave's own
    /// `PRE` tokens name Cleave, so removing Cleave out from under it must
    /// be refused with both feats named — and must leave the saved
    /// character untouched.
    #[test]
    fn removing_a_feat_another_held_feat_depends_on_is_refused_with_the_reason() {
        let root = tempdir("remove-feat-dependency-refused");
        let envelope = level_up_test_envelope("race:human", 6);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        add_feat_selection_enforcing_prerequisites_at_root(
            &root,
            "Cleave",
            None,
            "2026-08-01T00:00:00Z",
        )
        .expect("a Fighter 6 qualifies for Cleave");
        add_feat_selection_enforcing_prerequisites_at_root(
            &root,
            "Great Cleave",
            None,
            "2026-08-01T00:01:00Z",
        )
        .expect("Cleave is now held, so Great Cleave qualifies");

        let before = SavedCharacterStore::load(&root).unwrap().character_input;

        let error = remove_feat_selection_at_root(&root, "Cleave", None, "2026-08-01T00:02:00Z")
            .expect_err("Great Cleave depends on Cleave, so removing Cleave must be refused");

        assert!(error.contains("Cleave"), "{error}");
        assert!(error.contains("Great Cleave"), "{error}");

        let after = SavedCharacterStore::load(&root).unwrap().character_input;
        assert_eq!(
            before.chosen.selected_feats, after.chosen.selected_feats,
            "a refused removal must not touch the saved character"
        );

        // ...and the guard is a dependency guard, not a blanket refusal:
        // remove the dependent first and the prerequisite comes out fine.
        remove_feat_selection_at_root(&root, "Great Cleave", None, "2026-08-01T00:03:00Z")
            .expect("the leaf of the chain has no dependents");
        remove_feat_selection_at_root(&root, "Cleave", None, "2026-08-01T00:04:00Z")
            .expect("with Great Cleave gone, Cleave is removable");

        let reloaded = SavedCharacterStore::load(&root).unwrap().character_input;
        assert!(!reloaded.chosen.selected_feats.iter().any(|f| f == "Cleave"));
        assert!(!reloaded
            .chosen
            .selected_feats
            .iter()
            .any(|f| f == "Great Cleave"));

        std::fs::remove_dir_all(&root).ok();
    }

    /// A feat the character never took cannot be "removed" successfully.
    /// A `Saved` response for a removal that removed nothing is exactly
    /// the `success: true` lie `no-stub-mvp-doctrine.md` forbids.
    #[test]
    fn removing_a_feat_the_character_does_not_hold_fails_honestly() {
        let root = tempdir("remove-feat-not-held");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        let before = SavedCharacterStore::load(&root).unwrap().saved_at;

        let error =
            remove_feat_selection_at_root(&root, "Toughness", None, "2026-08-01T00:00:00Z")
                .expect_err("a feat that is not held cannot be removed");

        assert!(error.contains("does not hold it"), "{error}");
        assert_eq!(
            SavedCharacterStore::load(&root).unwrap().saved_at,
            before,
            "a failed removal must not re-save the character"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// Matching is by feat identity, not string equality: a feat seeded as
    /// the engine token `feat:toughness` is removable by the catalog key
    /// `Toughness` the picker uses, and vice versa. Otherwise a feat would
    /// be unremovable purely because of which path added it.
    #[test]
    fn a_feat_is_removable_in_either_id_shape() {
        let mut input = compose_character_input(&request_for("race:human", 1));
        input.chosen.selected_feats = vec!["feat:toughness".to_owned()];

        assert!(apply_remove_feat_selection(&mut input, "Toughness", None));
        assert!(input.chosen.selected_feats.is_empty());

        input.chosen.selected_feats = vec!["Toughness".to_owned()];
        assert!(apply_remove_feat_selection(&mut input, "feat:toughness", None));
        assert!(input.chosen.selected_feats.is_empty());
    }

    /// One copy comes out, not every copy — `selected_feats` legitimately
    /// holds a chooser feat twice — and the last copy takes its now-orphaned
    /// recorded target with it.
    #[test]
    fn removing_a_chooser_feat_removes_one_copy_and_finally_its_orphaned_target() {
        let mut input = compose_character_input(&request_for("race:human", 1));
        input.chosen.selected_feats.clear();
        input.chosen.selected_choices.clear();
        apply_add_feat_selection_with_target(
            &mut input,
            "Weapon Focus",
            resolve_feat_target_choice("Weapon Focus", Some("Longsword")).unwrap(),
        );
        apply_add_feat_selection_with_target(
            &mut input,
            "Weapon Focus",
            resolve_feat_target_choice("Weapon Focus", Some("Dagger")).unwrap(),
        );
        assert_eq!(input.chosen.selected_feats.len(), 2);
        assert_eq!(input.chosen.selected_choices.len(), 2);

        // Naming a target takes exactly that target, and one copy.
        assert!(apply_remove_feat_selection(&mut input, "Weapon Focus", Some("Longsword")));
        assert_eq!(input.chosen.selected_feats.len(), 1);
        assert_eq!(input.chosen.selected_choices.len(), 1);
        assert!(input.chosen.selected_choices[0]
            .selection_id
            .to_lowercase()
            .contains("dagger"));

        // The last copy leaves no orphaned target behind.
        assert!(apply_remove_feat_selection(&mut input, "Weapon Focus", None));
        assert!(input.chosen.selected_feats.is_empty());
        assert!(
            input.chosen.selected_choices.is_empty(),
            "a target for a feat the character no longer holds must not survive: {:?}",
            input.chosen.selected_choices
        );
    }

    /// Forgetting a spell removes every acquisition mode it was recorded
    /// in. `record_and_prepare_spell_selection` writes a `Known` and a
    /// `Prepared` entry together, so removing only one half would leave a
    /// spell prepared that the character no longer knows.
    #[test]
    fn removing_a_spell_forgets_every_acquisition_mode_of_it() {
        let mut input = compose_character_input(&request_for("race:human", 1));
        input.chosen.spells_selected.clear();
        apply_record_and_prepare_spell_selection(&mut input, "Magic Missile", "class:wizard");
        apply_add_spell_selection(
            &mut input,
            "Shield",
            "class:wizard",
            AcquisitionMode::Known,
        );
        assert_eq!(input.chosen.spells_selected.len(), 3);

        assert!(apply_remove_spell_selection(
            &mut input,
            "Magic Missile",
            "class:wizard"
        ));

        assert_eq!(input.chosen.spells_selected.len(), 1);
        assert_eq!(input.chosen.spells_selected[0].spell_id, "Shield");
        assert!(
            !apply_remove_spell_selection(&mut input, "Magic Missile", "class:wizard"),
            "removing an already-forgotten spell must report that it removed nothing"
        );
    }

    /// A spell the character never learned cannot be "removed" successfully.
    #[test]
    fn removing_a_spell_the_character_never_learned_fails_honestly() {
        let root = tempdir("remove-spell-not-known");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let error = remove_spell_selection_at_root(
            &root,
            "Magic Missile",
            "class:wizard",
            "2026-08-01T00:00:00Z",
        )
        .expect_err("a spell that was never learned cannot be forgotten");

        assert!(error.contains("Magic Missile"), "{error}");

        std::fs::remove_dir_all(&root).ok();
    }

    /// Dropping a carried item takes one entry with its applied equipmods,
    /// leaves the second copy alone, and survives a reload.
    #[test]
    fn removing_equipment_drops_one_entry_with_its_modifiers() {
        let root = tempdir("remove-equipment-round-trip");
        // Uses the ids `compose_character_input` really seeds
        // (`item:longsword`), not invented catalog keys — an id the corpus
        // cannot resolve would be discarded by the recompute gate and the
        // test would be measuring the gate, not the removal.
        let mut envelope = level_up_test_envelope("race:human", 1);
        apply_add_equipment_selection(
            &mut envelope.character_input,
            "item:longsword",
            ActiveState::EquippedActive,
        );
        let before = envelope.character_input.chosen.equipment_selections.len();
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response =
            remove_equipment_selection_at_root(&root, "item:longsword", "2026-08-01T00:00:00Z")
                .expect("a carried item must be removable");
        match response {
            CreateCharacterResponse::Saved { .. } => {}
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!("dropping one of two longswords must still compute: {diagnostics:?}")
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        let after = &reloaded.character_input.chosen.equipment_selections;
        assert_eq!(after.len(), before - 1, "exactly one entry comes out");
        assert_eq!(
            after
                .iter()
                .filter(|selection| selection.item_id == "item:longsword")
                .count(),
            1,
            "only the named copy comes out — the second longsword stays"
        );
        assert_eq!(reloaded.saved_at, "2026-08-01T00:00:00Z");

        std::fs::remove_dir_all(&root).ok();
    }

    /// The equipmods attached to a selection leave with the selection they
    /// live on — PCGen's `CUSTOMIZATION:EQMOD=` convention gives them no
    /// independent entry of their own, so leaving them behind would orphan
    /// a `+1` onto a weapon the character no longer carries.
    ///
    /// Asserted on the pure function rather than through the store because
    /// `SavedCharacterStore`'s `equipment_modifier` line is colon-delimited
    /// and cannot round-trip a colon-bearing modifier id — a real
    /// pre-existing persistence limit, unrelated to removal, that this test
    /// deliberately does not paper over.
    #[test]
    fn removing_equipment_takes_its_applied_modifiers_with_it() {
        let mut input = compose_character_input(&request_for("race:human", 1));
        input.chosen.equipment_selections.clear();
        apply_add_equipment_selection(&mut input, "item:longsword", ActiveState::EquippedActive);
        apply_add_equipment_selection(&mut input, "item:longsword", ActiveState::EquippedActive);
        assert!(apply_attach_equipment_modifier(
            &mut input,
            "item:longsword",
            "item:masterwork_component"
        ));
        assert_eq!(input.chosen.equipment_selections[0].applied_modifiers.len(), 1);

        assert!(apply_remove_equipment_selection(&mut input, "item:longsword"));

        assert_eq!(input.chosen.equipment_selections.len(), 1);
        assert!(
            input.chosen.equipment_selections[0]
                .applied_modifiers
                .is_empty(),
            "the modifier went out with the entry it was attached to"
        );
        assert!(apply_remove_equipment_selection(&mut input, "item:longsword"));
        assert!(!apply_remove_equipment_selection(&mut input, "item:longsword"));
    }

    /// An item the character does not carry cannot be "removed" successfully.
    #[test]
    fn removing_equipment_the_character_does_not_carry_fails_honestly() {
        let root = tempdir("remove-equipment-not-carried");
        let mut envelope = level_up_test_envelope("race:human", 1);
        envelope.character_input.chosen.equipment_selections.clear();
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let error =
            remove_equipment_selection_at_root(&root, "Longsword", "2026-08-01T00:00:00Z")
                .expect_err("an item that is not carried cannot be dropped");

        assert!(error.contains("does not carry it"), "{error}");

        std::fs::remove_dir_all(&root).ok();
    }

    /// The picker's data source: all 690 records come back, ineligible ones
    /// included and marked with a reason. Removing them would hide the
    /// rules from the player instead of explaining them.
    #[test]
    fn list_feats_for_character_marks_every_record_and_removes_none() {
        let root = tempdir("list-feats-eligibility");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response =
            list_feats_for_character_at_root(&root, &crate::feat_catalog::FeatCatalogFilter::default())
                .expect("listing should succeed");

        // 1578 hand-authored + the 649 corpus gap rows (`SD31-E6-F8-001`'s
        // original 83 + `SD31-E6-F8-002`'s 242 + `SD31-E6-F2-007`'s 199
        // Mythic Adventures rows -- SD31-W10-INTEGRATE-001 excluded 159
        // VISIBLE:EXPORT display-plumbing twins from the original 358 --
        // + `SD31-E6-F8-003`'s 7 + SD-32 Gate 0 book-onboarding
        // precondition's 9 inner_sea_taverns rows + SD-32 T9 onboarding's
        // (card 11) 109: inner_sea_combat 23 + inner_sea_gods 86). This is
        // the character sheet's own feat list, so this number moving is
        // the evidence that the gap lane's rows reach a player and not
        // only a table.
        assert_eq!(response.entries.len(), 2227, "no record may be filtered away");
        for entry in &response.entries {
            let eligibility = entry
                .eligibility
                .as_ref()
                .unwrap_or_else(|| panic!("'{}' came back with no verdict at all", entry.key));
            if eligibility.eligible {
                assert!(eligibility.unavailable_reason.is_none());
                assert!(eligibility.unmet.is_empty());
            } else {
                let reason = eligibility
                    .unavailable_reason
                    .as_deref()
                    .unwrap_or_default();
                assert!(
                    !reason.trim().is_empty(),
                    "'{}' is greyed out with no reason -- a dead affordance",
                    entry.key
                );
                assert!(!eligibility.unmet.is_empty());
            }
        }

        let improved_twf = response
            .entries
            .iter()
            .find(|entry| entry.key == "Improved Two-Weapon Fighting")
            .expect("still offered, just unavailable");
        assert!(!improved_twf.eligibility.as_ref().unwrap().eligible);

        let toughness = response
            .entries
            .iter()
            .find(|entry| entry.key == "Toughness")
            .expect("Toughness is in the catalog");
        assert!(toughness.eligibility.as_ref().unwrap().eligible);

        std::fs::remove_dir_all(&root).ok();
    }

    /// The character-less catalog commands must keep their exact previous
    /// wire shape -- no `eligibility` key at all, not a `null` one.
    #[test]
    fn the_character_less_catalog_sends_no_eligibility_key() {
        let response = crate::feat_catalog::build_feat_catalog();
        assert_eq!(response.entries.len(), 2227);
        assert!(response.entries.iter().all(|entry| entry.eligibility.is_none()));
        let json = serde_json::to_string(&response.entries[0]).expect("serialises");
        assert!(!json.contains("eligibility"), "{json}");
    }

    // ----- `add_spell_selection` (Criterion 18) -----

    #[test]
    fn apply_add_spell_selection_appends_to_spells_selected() {
        let mut input = compose_character_input(&request_for("race:human", 1));
        let starting_len = input.chosen.spells_selected.len();

        apply_add_spell_selection(
            &mut input,
            "Mage Armor",
            "class:wizard",
            AcquisitionMode::Known,
        );

        assert_eq!(input.chosen.spells_selected.len(), starting_len + 1);
        let added = input
            .chosen
            .spells_selected
            .last()
            .expect("an entry was just pushed");
        assert_eq!(added.spell_id, "Mage Armor");
        assert_eq!(added.source_class_id, "class:wizard");
        assert_eq!(added.acquisition_mode, AcquisitionMode::Known);
    }

    /// v0.6 alpha swarm item 3: the pure half of the Wizard spellbook
    /// bootstrap fix -- see `apply_record_and_prepare_spell_selection`'s
    /// own doc comment for the full deadlock analysis. The `_at_root`
    /// golden-path proof lives in `pf1_adapter.rs`'s own test module
    /// (that's where the deadlock reproduction against the real
    /// persistence layer lives too).
    #[test]
    fn apply_record_and_prepare_spell_selection_appends_both_known_and_prepared_entries() {
        let mut input = compose_character_input(&request_for("race:human", 1));
        let starting_len = input.chosen.spells_selected.len();

        apply_record_and_prepare_spell_selection(&mut input, "evocation.0.light", "class:wizard");

        assert_eq!(input.chosen.spells_selected.len(), starting_len + 2);
        let added = &input.chosen.spells_selected[starting_len..];
        assert!(added.iter().all(|s| s.spell_id == "evocation.0.light" && s.source_class_id == "class:wizard"));
        assert!(added.iter().any(|s| s.acquisition_mode == AcquisitionMode::Known));
        assert!(added.iter().any(|s| s.acquisition_mode == AcquisitionMode::Prepared));
    }

    /// The single most important regression guard for Criterion 18's spell
    /// half: proves the real load -> mutate -> recompute -> re-save ->
    /// return round trip against a real `SavedCharacterStore` fixture on
    /// disk, mirroring `level_up_character_at_root`'s own golden path test.
    #[test]
    fn add_spell_selection_at_root_appends_and_persists_when_computed() {
        let root = tempdir("add-spell-golden-path");
        let envelope = level_up_test_envelope("race:human", 1);
        let starting_len = envelope.character_input.chosen.spells_selected.len();
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = add_spell_selection_at_root(
            &root,
            "Mage Armor",
            "class:wizard",
            AcquisitionMode::Known,
            "2026-07-21T00:00:00Z",
        )
        .expect("add spell selection call should not error");

        match response {
            CreateCharacterResponse::Saved { .. } => {}
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!(
                    "Human Fighter level 1 with an added spell selection must still \
                     reach Computed, got diagnostics: {diagnostics:?}"
                );
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.character_input.chosen.spells_selected.len(),
            starting_len + 1,
            "the on-disk envelope must reflect the appended spell selection"
        );
        let added = reloaded
            .character_input
            .chosen
            .spells_selected
            .last()
            .expect("an entry was just pushed");
        assert_eq!(added.spell_id, "Mage Armor");
        assert_eq!(added.source_class_id, "class:wizard");
        assert_eq!(added.acquisition_mode, AcquisitionMode::Known);
        assert_eq!(reloaded.saved_at, "2026-07-21T00:00:00Z");

        std::fs::remove_dir_all(&root).ok();
    }

    /// `add_spell_selection_at_root`'s error path when there is nothing
    /// saved at `root` yet — must fail honestly rather than silently
    /// creating a character out of thin air.
    #[test]
    fn add_spell_selection_at_root_fails_honestly_when_nothing_is_saved_yet() {
        let root = tempdir("add-spell-missing-character");

        let result = add_spell_selection_at_root(
            &root,
            "Mage Armor",
            "class:wizard",
            AcquisitionMode::Known,
            "2026-07-21T00:00:00Z",
        );

        assert!(
            result.is_err(),
            "adding a spell selection to a nonexistent saved character must fail"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- `set_skill_allocations` (v0.6 alpha swarm, task 2) -----

    #[test]
    fn apply_set_skill_allocations_replaces_skill_allocations_wholesale() {
        let mut input = compose_character_input(&request_for("race:human", 1));
        assert_eq!(
            input.chosen.skill_allocations.len(),
            3,
            "the fixed demo loadout starts with three skill allocations"
        );

        apply_set_skill_allocations(
            &mut input,
            vec![SkillAllocation { skill_id: "skill:climb".to_owned(), ranks: 2 }],
        );

        assert_eq!(
            input.chosen.skill_allocations,
            vec![SkillAllocation { skill_id: "skill:climb".to_owned(), ranks: 2 }],
            "the whole set must be replaced, not appended to"
        );
    }

    /// The single most important regression guard for task 2: proves the
    /// real load -> mutate -> recompute -> re-save -> return round trip
    /// against a real `SavedCharacterStore` fixture on disk, mirroring
    /// `add_equipment_selection_at_root`'s own golden path test. Re-orders
    /// the supported Climb/Intimidate/Swim triple (rather than sending it
    /// back unchanged) so the assertion actually proves replacement, not a
    /// no-op.
    #[test]
    fn set_skill_allocations_at_root_replaces_and_persists_when_computed() {
        let root = tempdir("set-skill-allocations-golden-path");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = set_skill_allocations_at_root(
            &root,
            vec![
                SkillAllocation { skill_id: "skill:swim".to_owned(), ranks: 1 },
                SkillAllocation { skill_id: "skill:intimidate".to_owned(), ranks: 1 },
                SkillAllocation { skill_id: "skill:climb".to_owned(), ranks: 1 },
            ],
            "2026-07-21T00:00:00Z",
        )
        .expect("set skill allocations call should not error");

        match response {
            CreateCharacterResponse::Saved { .. } => {}
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!(
                    "Human Fighter level 1 with a re-ordered but still-supported skill \
                     triple must still reach Computed, got diagnostics: {diagnostics:?}"
                );
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded
                .character_input
                .chosen
                .skill_allocations
                .iter()
                .map(|allocation| allocation.skill_id.as_str())
                .collect::<Vec<_>>(),
            vec!["skill:swim", "skill:intimidate", "skill:climb"],
            "the on-disk envelope must reflect the caller's full replacement set/order"
        );
        assert_eq!(reloaded.saved_at, "2026-07-21T00:00:00Z");

        std::fs::remove_dir_all(&root).ok();
    }

    /// Mirrors `level_up_character_at_root`'s own "never persist an unproven
    /// build" proof: an out-of-posture skill allocation must leave the
    /// on-disk envelope exactly as it was, not silently apply.
    #[test]
    fn set_skill_allocations_at_root_does_not_persist_when_resulting_build_is_blocked() {
        let root = tempdir("set-skill-allocations-blocked");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = set_skill_allocations_at_root(
            &root,
            vec![SkillAllocation { skill_id: "skill:climb".to_owned(), ranks: 2 }],
            "2026-07-21T00:00:00Z",
        )
        .expect("set skill allocations call should not error even when blocked");

        match response {
            CreateCharacterResponse::Blocked { .. } => {}
            CreateCharacterResponse::Saved { .. } => {
                panic!("rank 2 Climb alone is outside the supported posture and must not reach Computed")
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.character_input.chosen.skill_allocations,
            envelope.character_input.chosen.skill_allocations,
            "a blocked mutation must leave the on-disk skill allocations untouched"
        );
        assert_eq!(
            reloaded.saved_at, LEVEL_UP_TEST_SAVED_AT,
            "a blocked mutation must not advance saved_at either"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn set_skill_allocations_at_root_fails_honestly_when_nothing_is_saved_yet() {
        let root = tempdir("set-skill-allocations-missing-character");

        let result = set_skill_allocations_at_root(
            &root,
            vec![SkillAllocation { skill_id: "skill:climb".to_owned(), ranks: 1 }],
            "2026-07-21T00:00:00Z",
        );

        assert!(
            result.is_err(),
            "setting skill allocations on a nonexistent saved character must fail"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- `update_character_bio` / `load_character_bio` (v0.6 alpha swarm) -----

    fn sample_bio() -> CharacterBioDto {
        CharacterBioDto {
            alignment: "Lawful Good".to_owned(),
            deity: "Iomedae".to_owned(),
            sex: "Female".to_owned(),
            age: "27".to_owned(),
            height: "5'8\"".to_owned(),
            weight: "150 lbs".to_owned(),
            hair: "Auburn".to_owned(),
            eyes: "Green".to_owned(),
        }
    }

    #[test]
    fn load_character_bio_at_root_returns_all_empty_default_when_no_bio_file_exists_yet() {
        let root = tempdir("bio-default");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let bio = load_character_bio_at_root(&root).expect("loading an absent bio should not error");

        assert_eq!(bio, CharacterBioDto::default());

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn update_character_bio_at_root_persists_and_round_trips_through_load() {
        let root = tempdir("bio-round-trip");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        save_character_bio_at_root(&root, &sample_bio()).expect("saving a bio should not error");
        let reloaded = load_character_bio_at_root(&root).expect("reloading the saved bio should not error");

        assert_eq!(reloaded, sample_bio());

        std::fs::remove_dir_all(&root).ok();
    }

    /// A second save fully replaces the first -- proves this is a real
    /// overwrite, not an append or a write-once no-op.
    #[test]
    fn update_character_bio_at_root_overwrites_a_previously_saved_bio() {
        let root = tempdir("bio-overwrite");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        save_character_bio_at_root(&root, &sample_bio()).expect("first save should not error");
        let updated = CharacterBioDto { alignment: "Chaotic Neutral".to_owned(), ..sample_bio() };
        save_character_bio_at_root(&root, &updated).expect("second save should not error");

        let reloaded = load_character_bio_at_root(&root).expect("reload should not error");
        assert_eq!(reloaded, updated);

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn update_character_bio_at_root_fails_honestly_when_nothing_is_saved_yet() {
        let root = tempdir("bio-missing-character");

        let result = save_character_bio_at_root(&root, &sample_bio());

        assert!(result.is_err(), "saving a bio for a nonexistent saved character must fail");

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- `load_character_money` / `adjust_character_money` (v0.6 alpha swarm) -----

    #[test]
    fn load_character_money_at_root_returns_a_zero_balance_when_no_money_file_exists_yet() {
        let root = tempdir("money-default");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let money = load_character_money_at_root(&root).expect("loading absent money should not error");

        assert_eq!(
            money,
            CharacterMoneyDto { total_copper: 0, platinum: 0, gold: 0, silver: 0, copper: 0 }
        );

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn adjust_character_money_at_root_adds_funds_and_persists_the_new_total() {
        let root = tempdir("money-add");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let money = adjust_character_money_at_root(&root, 1234).expect("adding funds should not error");

        assert_eq!(
            money,
            CharacterMoneyDto { total_copper: 1234, platinum: 1, gold: 2, silver: 3, copper: 4 }
        );
        let reloaded = load_character_money_at_root(&root).expect("reload should not error");
        assert_eq!(reloaded, money, "the balance must be persisted, not just returned in-memory");

        std::fs::remove_dir_all(&root).ok();
    }

    /// Two consecutive adjustments accumulate against the persisted total,
    /// not each other's in-memory return value -- proves the balance is
    /// genuinely read-modify-write, not overwritten from a stale snapshot.
    #[test]
    fn adjust_character_money_at_root_accumulates_across_repeated_calls() {
        let root = tempdir("money-accumulate");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        adjust_character_money_at_root(&root, 1000).expect("first adjustment should not error");
        let after_second =
            adjust_character_money_at_root(&root, -300).expect("second adjustment should not error");

        assert_eq!(after_second.total_copper, 700);

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn adjust_character_money_at_root_rejects_spending_more_than_the_current_balance() {
        let root = tempdir("money-insufficient-funds");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        adjust_character_money_at_root(&root, 100).expect("seed funds should not error");

        let result = adjust_character_money_at_root(&root, -200);

        assert!(result.is_err(), "spending more than the current balance must fail honestly");
        let reloaded = load_character_money_at_root(&root).expect("reload should not error");
        assert_eq!(
            reloaded.total_copper, 100,
            "a rejected spend must not partially apply or corrupt the persisted balance"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn adjust_character_money_at_root_fails_honestly_when_nothing_is_saved_yet() {
        let root = tempdir("money-missing-character");

        let result = adjust_character_money_at_root(&root, 100);

        assert!(result.is_err(), "adjusting money for a nonexistent saved character must fail");

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- `load_character_durability` / `adjust_character_hp` (v0.6 alpha swarm) -----

    #[test]
    fn load_character_durability_at_root_defaults_current_hp_to_computed_max_hp() {
        let root = tempdir("hp-default");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let durability =
            load_character_durability_at_root(&root).expect("loading durability should not error");

        // Fighter d10 level 1, CON mod +2 (score 14, unaffected by the fixture's
        // Human ability-bonus target of strength): 10 + 2 = 12.
        assert_eq!(durability.max_hp, 12);
        assert_eq!(durability.current_hp, 12);
        assert_eq!(durability.nonlethal_damage, 0);
        assert_eq!(durability.status, "Normal");

        std::fs::remove_dir_all(&root).ok();
    }

    /// v0.6 alpha swarm item 17: the feat-effects engine's first real,
    /// wired case. A Fighter with Toughness selected must show the correct
    /// 15 HP (12 base + Toughness's real +3), not the pre-fix 12 QA found
    /// (zero explanations mentioning the feat at all).
    #[test]
    fn load_character_durability_at_root_includes_toughnesss_real_plus_three_hp() {
        let root = tempdir("hp-toughness");
        let mut envelope = level_up_test_envelope("race:human", 1);
        envelope.character_input.chosen.selected_feats.push("Toughness".to_owned());
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let durability =
            load_character_durability_at_root(&root).expect("loading durability should not error");

        assert_eq!(durability.max_hp, 15, "12 base + Toughness's real flat +3 = 15");
        assert_eq!(
            durability.current_hp, 15,
            "current HP must default to the feat-inclusive max, not the pre-feat base"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn adjust_character_hp_at_root_persists_damage_and_clamps_healing_at_max_hp() {
        let root = tempdir("hp-damage-heal");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let after_damage = adjust_character_hp_at_root(&root, -5, 0).expect("damage should not error");
        assert_eq!(after_damage.current_hp, 7);

        let after_overheal =
            adjust_character_hp_at_root(&root, 100, 0).expect("healing should not error");
        assert_eq!(after_overheal.current_hp, 12, "healing must clamp at max_hp, not exceed it");

        let reloaded =
            load_character_durability_at_root(&root).expect("reload should not error");
        assert_eq!(reloaded.current_hp, 12, "the healed total must be persisted");

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn adjust_character_hp_at_root_tracks_nonlethal_damage_and_reflects_status() {
        let root = tempdir("hp-nonlethal");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let staggered =
            adjust_character_hp_at_root(&root, 0, 12).expect("nonlethal damage should not error");
        assert_eq!(
            staggered.status, "Staggered",
            "nonlethal damage exactly equal to current HP (12) is staggered"
        );

        let unconscious =
            adjust_character_hp_at_root(&root, 0, 1).expect("more nonlethal damage should not error");
        assert_eq!(unconscious.nonlethal_damage, 13);
        assert_eq!(unconscious.status, "Unconscious");

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn adjust_character_hp_at_root_reflects_dying_and_dead_thresholds() {
        let root = tempdir("hp-dying-dead");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let dying = adjust_character_hp_at_root(&root, -13, 0).expect("lethal damage should not error");
        assert_eq!(dying.current_hp, -1);
        assert_eq!(dying.status, "Dying");

        // -1 - 13 = -14, which equals -constitution_score (14) -> Dead.
        let dead = adjust_character_hp_at_root(&root, -13, 0).expect("further lethal damage should not error");
        assert_eq!(dead.current_hp, -14);
        assert_eq!(dead.status, "Dead");

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn adjust_character_hp_at_root_floors_nonlethal_recovery_at_zero() {
        let root = tempdir("hp-nonlethal-floor");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        adjust_character_hp_at_root(&root, 0, 3).expect("nonlethal damage should not error");
        let recovered =
            adjust_character_hp_at_root(&root, 0, -100).expect("nonlethal recovery should not error");

        assert_eq!(recovered.nonlethal_damage, 0, "nonlethal damage must floor at 0, not go negative");

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn load_character_durability_at_root_fails_honestly_for_a_multiclass_build() {
        let root = tempdir("hp-multiclass");
        let mut envelope = level_up_test_envelope("race:human", 2);
        envelope.character_input.chosen.class_levels.push(CharacterClassLevel {
            class_id: "class:rogue".to_owned(),
            level: 1,
        });
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let result = load_character_durability_at_root(&root);

        assert!(
            result.is_err(),
            "durability for a multiclass build must fail honestly, not fabricate a value"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn load_character_durability_at_root_fails_honestly_when_nothing_is_saved_yet() {
        let root = tempdir("hp-missing-character");

        let result = load_character_durability_at_root(&root);

        assert!(
            result.is_err(),
            "loading durability for a nonexistent saved character must fail"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- Render-staleness real root cause (v0.6 alpha swarm) -----
    //
    // Frontend traced the "Spells/Gear tab looks stale right after a
    // mutation" bug to a real serde defect, not a React timing issue:
    // CreateCharacterResponse's `Saved` variant serialized `corpus_derived`
    // literally as `"corpus_derived"` on the wire (no `#[serde(tag =
    // "kind")]`-level `rename_all`, since that would also camelCase the
    // `"Saved"`/`"Blocked"` tag strings the frontend matches on) while the
    // TS boundary's asserted-not-validated type declared `corpusDerived` --
    // silently `undefined` at runtime, invisible to any Rust-side test that
    // only checks struct field values rather than the actual JSON shape.
    // These tests serialize a real response value and assert on the JSON
    // keys directly -- the only way this exact bug class is actually
    // caught.

    #[test]
    fn create_character_response_saved_serializes_corpus_derived_as_camel_case_without_touching_the_tag(
    ) {
        let response = CreateCharacterResponse::Saved {
            summary: Box::new(CharacterSummaryDto {
                character_id: "c".to_owned(),
                display_label: "d".to_owned(),
                game_system: "pf1".to_owned(),
                schema_version: 1,
                saved_at: "2026-07-23T00:00:00Z".to_owned(),
                race_id: "race:human".to_owned(),
                class_summary: "class:fighter:1".to_owned(),
            }),
            snapshot: PilotSnapshotDto {
                ability_modifiers: AbilityModifiersDto {
                    strength: 0,
                    dexterity: 0,
                    constitution: 0,
                    intelligence: 0,
                    wisdom: 0,
                    charisma: 0,
                },
                base_attack_bonus: 0,
                base_saves: BaseSavesDto { fortitude: 0, reflex: 0, will: 0 },
                baseline_melee_attack_bonus: 0,
                baseline_armor_class: 0,
                total_saves: BaseSavesDto { fortitude: 0, reflex: 0, will: 0 },
                selected_skill_modifiers: SelectedSkillModifiersDto {
                    climb: 0,
                    intimidate: 0,
                    swim: 0,
                },
                damage_reduction: None,
                companion: None,
                spellbook: None,
            },
            corpus_derived: CorpusDerivedDto {
                school_coverage: Vec::new(),
                equipped_items: Vec::new(),
                equipment_effects: EquipmentEffectsDto {
                    per_item: Vec::new(),
                    armor_class_delta: 0,
                    armor_check_penalty_total: 0,
                    max_dex_cap: None,
                    spell_failure_chance: None,
                    attack_bonus_delta: None,
                    spell_resistance_total: None,
                },
                encumbrance: empty_encumbrance_dto(),
                unresolved_spell_ids: Vec::new(),
                unresolved_equipment_item_ids: Vec::new(),
            },
        };

        let value = serde_json::to_value(&response).expect("response should serialize");
        let object = value.as_object().expect("response should serialize as a JSON object");

        assert_eq!(
            object.get("kind").and_then(|v| v.as_str()),
            Some("Saved"),
            "the kind tag must stay exactly \"Saved\" (PascalCase, untouched) so \
             outcome.kind === 'Saved' checks on the frontend keep working: {object:?}"
        );
        assert!(
            object.contains_key("corpusDerived"),
            "corpus_derived must serialize as camelCase corpusDerived on the wire: {object:?}"
        );
        assert!(
            !object.contains_key("corpus_derived"),
            "corpus_derived must NOT also/still appear as snake_case on the wire: {object:?}"
        );
    }

    /// v0.6 alpha swarm (QA finding, 2026-07-24): `EquipmentEffectsDto`'s
    /// three `Option` fields must genuinely omit their key when `None`
    /// (mirroring `PilotSnapshotDto.damage_reduction`'s own precedent), not
    /// serialize as `"maxDexCap":null` -- a present key with a literal
    /// `null` value defeats the frontend's `!== undefined` hide-checks
    /// (`null !== undefined` is `true`), rendering garbled `"+null"`/
    /// `"null%"` strings instead of hiding the field. Only one character
    /// build (Chain Shirt + exactly one weapon) had been live-verified
    /// before this bug was found, and it happened to resolve `Some(...)`
    /// for all three fields -- any zero- or two-weapon build (a fresh
    /// character, most Wizards, a dual-wielder) would have hit this.
    #[test]
    fn equipment_effects_dto_omits_its_optional_fields_when_none_and_includes_them_when_some() {
        let with_none = EquipmentEffectsDto {
            per_item: Vec::new(),
            armor_class_delta: 4,
            armor_check_penalty_total: -2,
            max_dex_cap: None,
            spell_failure_chance: None,
            attack_bonus_delta: None,
            spell_resistance_total: None,
        };
        let json = serde_json::to_string(&with_none).expect("serialization should succeed");
        assert!(
            !json.contains("maxDexCap")
                && !json.contains("spellFailureChance")
                && !json.contains("attackBonusDelta")
                && !json.contains("spellResistanceTotal"),
            "None fields must omit their key entirely, not serialize as null: {json}"
        );
        // The two non-Option fields are unaffected by this fix -- always present.
        assert!(json.contains("\"armorClassDelta\":4"));
        assert!(json.contains("\"armorCheckPenaltyTotal\":-2"));

        let with_some = EquipmentEffectsDto {
            per_item: vec![ResolvedEquipmentEffectDto {
                item_id: "item:spell_resistance_armor".to_owned(),
                equipment_record_key: "Special Ability ~ Spell Resistance / 13 ~ Armor".to_owned(),
                category: "Equipmods".to_owned(),
                armor_class_bonus: None,
                max_dex: None,
                spell_failure: None,
                armor_check_penalty: None,
                spell_resistance_bonus: Some(13),
            }],
            armor_class_delta: 4,
            armor_check_penalty_total: -2,
            max_dex_cap: Some(4),
            spell_failure_chance: Some(20.0),
            attack_bonus_delta: Some(1),
            spell_resistance_total: Some(13),
        };
        let json = serde_json::to_string(&with_some).expect("serialization should succeed");
        assert!(json.contains("\"maxDexCap\":4"));
        assert!(json.contains("\"spellFailureChance\":20.0"));
        assert!(json.contains("\"attackBonusDelta\":1"));
        assert!(json.contains("\"spellResistanceTotal\":13"), "{json}");
        assert!(json.contains("\"spellResistanceBonus\":13"), "{json}");
    }

    /// The encumbrance DTO must cross the IPC boundary with the real
    /// engine numbers intact and in the camelCase shape the TypeScript
    /// `EncumbranceDto` declares -- the boundary where this domain's
    /// computation was previously discarded entirely.
    ///
    /// `loadMaxDexCap` specifically follows the `maxDexCap` precedent: a
    /// light load imposes no cap, and that must omit the key rather than
    /// serialize a literal `null`, which would defeat the frontend's
    /// `!== undefined` check and render "+null".
    #[test]
    fn encumbrance_dto_serializes_real_engine_values_in_the_camel_case_wire_shape() {
        use codex::rules_core::encumbrance::{
            carrying_capacity_thresholds, CarriedItem, EncumbranceComputation, EncumbranceLevel,
        };
        use codex::rules_core::size::SizeCategory;

        // A real Strength-6 medium load: Chain Shirt (25 lb / 100 gp) plus
        // Longsword (4 lb / 15 gp), both real CRB corpus values, against
        // load.lst's LOAD:6|60 row (light 20 / medium 40 / heavy 60).
        let computation = EncumbranceComputation {
            per_item: vec![
                CarriedItem {
                    item_id: "item:chain_shirt".to_owned(),
                    weight_lbs: 25.0,
                    cost_gp: Some(100.0),
                },
                CarriedItem {
                    item_id: "item:longsword".to_owned(),
                    weight_lbs: 4.0,
                    cost_gp: Some(15.0),
                },
            ],
            total_carried_weight_lbs: 29.0,
            total_carried_cost_gp: 115.0,
            thresholds: carrying_capacity_thresholds(6, SizeCategory::Medium),
            level: EncumbranceLevel::Medium,
            unresolved_item_ids: Vec::new(),
            load_max_dex_cap: EncumbranceLevel::Medium.max_dex_cap(),
            load_armor_check_penalty: EncumbranceLevel::Medium.armor_check_penalty(),
        };

        let json = serde_json::to_string(&map_encumbrance_dto(&computation))
            .expect("serialization should succeed");

        assert!(json.contains("\"totalCarriedWeightLbs\":29.0"), "{json}");
        assert!(json.contains("\"totalCarriedCostGp\":115.0"), "{json}");
        assert!(json.contains("\"lightMaxLbs\":20.0"), "{json}");
        assert!(json.contains("\"mediumMaxLbs\":40.0"), "{json}");
        assert!(json.contains("\"heavyMaxLbs\":60.0"), "{json}");
        assert!(json.contains("\"level\":\"Medium\""), "{json}");
        assert!(json.contains("\"loadMaxDexCap\":3"), "{json}");
        assert!(json.contains("\"loadArmorCheckPenalty\":-3"), "{json}");
        assert!(json.contains("\"weightLbs\":25.0") && json.contains("\"costGp\":100.0"), "{json}");
    }

    #[test]
    fn encumbrance_dto_omits_the_load_max_dex_cap_under_a_light_load_rather_than_nulling_it() {
        use codex::rules_core::encumbrance::{
            carrying_capacity_thresholds, EncumbranceComputation, EncumbranceLevel,
        };
        use codex::rules_core::size::SizeCategory;

        let computation = EncumbranceComputation {
            per_item: Vec::new(),
            total_carried_weight_lbs: 0.0,
            total_carried_cost_gp: 0.0,
            thresholds: carrying_capacity_thresholds(10, SizeCategory::Medium),
            level: EncumbranceLevel::Light,
            unresolved_item_ids: Vec::new(),
            load_max_dex_cap: EncumbranceLevel::Light.max_dex_cap(),
            load_armor_check_penalty: EncumbranceLevel::Light.armor_check_penalty(),
        };

        let json = serde_json::to_string(&map_encumbrance_dto(&computation))
            .expect("serialization should succeed");

        assert!(
            !json.contains("loadMaxDexCap"),
            "a light load imposes no cap; the key must be omitted, not null: {json}"
        );
        // The check penalty is a real, always-present `0` under a light
        // load -- an absent key here would be wrong, unlike the cap above.
        assert!(json.contains("\"loadArmorCheckPenalty\":0"), "{json}");
    }

    #[test]
    fn purchase_equipment_response_purchased_serializes_corpus_derived_as_camel_case_without_touching_the_tag(
    ) {
        let response = PurchaseEquipmentResponse::Purchased {
            summary: Box::new(CharacterSummaryDto {
                character_id: "c".to_owned(),
                display_label: "d".to_owned(),
                game_system: "pf1".to_owned(),
                schema_version: 1,
                saved_at: "2026-07-23T00:00:00Z".to_owned(),
                race_id: "race:human".to_owned(),
                class_summary: "class:fighter:1".to_owned(),
            }),
            snapshot: PilotSnapshotDto {
                ability_modifiers: AbilityModifiersDto {
                    strength: 0,
                    dexterity: 0,
                    constitution: 0,
                    intelligence: 0,
                    wisdom: 0,
                    charisma: 0,
                },
                base_attack_bonus: 0,
                base_saves: BaseSavesDto { fortitude: 0, reflex: 0, will: 0 },
                baseline_melee_attack_bonus: 0,
                baseline_armor_class: 0,
                total_saves: BaseSavesDto { fortitude: 0, reflex: 0, will: 0 },
                selected_skill_modifiers: SelectedSkillModifiersDto {
                    climb: 0,
                    intimidate: 0,
                    swim: 0,
                },
                damage_reduction: None,
                companion: None,
                spellbook: None,
            },
            corpus_derived: CorpusDerivedDto {
                school_coverage: Vec::new(),
                equipped_items: Vec::new(),
                equipment_effects: EquipmentEffectsDto {
                    per_item: Vec::new(),
                    armor_class_delta: 0,
                    armor_check_penalty_total: 0,
                    max_dex_cap: None,
                    spell_failure_chance: None,
                    attack_bonus_delta: None,
                    spell_resistance_total: None,
                },
                encumbrance: empty_encumbrance_dto(),
                unresolved_spell_ids: Vec::new(),
                unresolved_equipment_item_ids: Vec::new(),
            },
            money: money_dto_from_total(0),
        };

        let value = serde_json::to_value(&response).expect("response should serialize");
        let object = value.as_object().expect("response should serialize as a JSON object");

        assert_eq!(object.get("kind").and_then(|v| v.as_str()), Some("Purchased"));
        assert!(object.contains_key("corpusDerived"), "{object:?}");
        assert!(!object.contains_key("corpus_derived"), "{object:?}");
    }

    // ----- `level_up_character` (Criterion 17) -----

    fn tempdir(label: &str) -> PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "codex-character-hub-level-up-{label}-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir_all(&path).expect("temp dir should be creatable");
        path
    }

    const LEVEL_UP_TEST_SAVED_AT: &str = "2026-07-08T00:00:00Z";

    fn level_up_test_envelope(race_id: &str, level: u8) -> SavedCharacterEnvelope {
        let character_input = compose_character_input(&request_for(race_id, level));
        SavedCharacterEnvelope {
            character_id: "char-level-up-test".to_owned(),
            revision_id: "char-level-up-test.rev.1".to_owned(),
            revision_kind: SavedCharacterRevisionKind::Authoritative,
            saved_at: LEVEL_UP_TEST_SAVED_AT.to_owned(),
            schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
            app_or_runtime_version: "codex-dev".to_owned(),
            content_or_rules_provenance: SOURCE_PACKAGE_ID.to_owned(),
            game_system: GAME_SYSTEM_ID.to_owned(),
            latest_authoritative_revision_ref: "char-level-up-test.rev.1".to_owned(),
            display_label: "Level Up Test Character".to_owned(),
            character_input,
        }
    }

    #[test]
    fn apply_level_up_increments_existing_class_level() {
        let mut input = compose_character_input(&request_for("race:human", 1));

        apply_level_up(&mut input, FIGHTER_CLASS_ID);

        assert_eq!(input.chosen.class_levels.len(), 1);
        assert_eq!(input.chosen.class_levels[0].class_id, FIGHTER_CLASS_ID);
        assert_eq!(input.chosen.class_levels[0].level, 2);
    }

    #[test]
    fn apply_level_up_adds_a_new_class_level_entry_for_an_unheld_class() {
        let mut input = compose_character_input(&request_for("race:human", 1));

        apply_level_up(&mut input, "class:wizard");

        assert_eq!(input.chosen.class_levels.len(), 2);
        assert_eq!(input.chosen.class_levels[0].class_id, FIGHTER_CLASS_ID);
        assert_eq!(
            input.chosen.class_levels[0].level, 1,
            "the existing class level must be untouched"
        );
        assert_eq!(input.chosen.class_levels[1].class_id, "class:wizard");
        assert_eq!(input.chosen.class_levels[1].level, 1);
    }

    /// v0.6 alpha swarm: `compose_character_input`'s Wizard school-choice
    /// seeding fix only covered fresh creation -- multiclassing Wizard onto
    /// an existing character via `apply_level_up`'s new-class-entry branch
    /// hit the same unconditional spellbook-posture block, unchanged.
    /// Frontend verified this live before this fix landed.
    #[test]
    fn apply_level_up_seeds_the_canonical_wizard_school_choices_when_dipping_into_wizard() {
        let mut input = compose_character_input(&request_for("race:human", 1));

        apply_level_up(&mut input, "class:wizard");

        assert!(
            input.chosen.selected_choices.iter().any(|c| c.choice_set_id
                == "choice:wizard_school_specialization"
                && c.selection_id == "school:evocation"),
            "multiclassing into Wizard must seed the canonical Evocation specialization: {:?}",
            input.chosen.selected_choices
        );
        let opposed: Vec<&str> = input
            .chosen
            .selected_choices
            .iter()
            .filter(|c| c.choice_set_id == "choice:wizard_opposed_schools")
            .map(|c| c.selection_id.as_str())
            .collect();
        assert_eq!(opposed.len(), 2, "must seed exactly two opposed schools: {opposed:?}");
        assert!(opposed.contains(&"school:necromancy"));
        assert!(opposed.contains(&"school:transmutation"));
    }

    /// A second consecutive level-up within Wizard (not a fresh dip) must
    /// not re-seed the choices -- `wizard_has_canonical_specialization_selections`
    /// requires *exactly* two opposed-school entries, so a duplicate pair
    /// would silently break it.
    #[test]
    fn apply_level_up_does_not_reseed_wizard_choices_on_a_second_level_up_within_wizard() {
        let mut input = compose_character_input(&request_for("race:human", 1));
        apply_level_up(&mut input, "class:wizard");
        apply_level_up(&mut input, "class:wizard");

        assert_eq!(input.chosen.class_levels[1].level, 2, "the second call must increment, not re-dip");
        let opposed_count = input
            .chosen
            .selected_choices
            .iter()
            .filter(|c| c.choice_set_id == "choice:wizard_opposed_schools")
            .count();
        assert_eq!(
            opposed_count, 2,
            "a second level-up within Wizard must not duplicate the seeded opposed schools"
        );
    }

    /// The single most important regression guard for Criterion 17: proves
    /// the real load -> mutate -> recompute -> re-save -> return round trip
    /// against a real `SavedCharacterStore` fixture on disk, not a mock.
    #[test]
    fn level_up_character_at_root_increments_level_and_persists_when_computed() {
        let root = tempdir("golden-path");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = level_up_character_at_root(
            &root,
            FIGHTER_CLASS_ID,
            Vec::new(),
            None,
            "2026-07-21T00:00:00Z",
        )
        .expect("level up call should not error");

        match response {
            CreateCharacterResponse::Saved { summary, .. } => {
                assert_eq!(summary.class_summary, "class:fighter:2");
            }
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!(
                    "Human Fighter level 1 -> 2 must reach Computed, got diagnostics: {diagnostics:?}"
                );
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(reloaded.character_input.chosen.class_levels.len(), 1);
        assert_eq!(
            reloaded.character_input.chosen.class_levels[0].level, 2,
            "the on-disk envelope must reflect the leveled-up build"
        );
        assert_eq!(reloaded.saved_at, "2026-07-21T00:00:00Z");

        std::fs::remove_dir_all(&root).ok();
    }

    /// Mirrors `create_character`/`clone_character`'s "never persist an
    /// unproven build" invariant: leveling past the engine's currently
    /// supported range must return `Blocked` and must not touch the file
    /// on disk at all.
    #[test]
    fn level_up_character_at_root_does_not_persist_when_leveled_up_build_is_blocked() {
        let root = tempdir("blocked-path");
        // Level 20 is the top of the compute engine's supported Fighter range
        // (`MAX_SUPPORTED_FIGHTER_LEVEL` in `pilot_compute.rs`); leveling to 21 must
        // fall back to Blocked.
        let envelope = level_up_test_envelope("race:human", 20);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = level_up_character_at_root(
            &root,
            FIGHTER_CLASS_ID,
            Vec::new(),
            None,
            "2026-07-21T00:00:00Z",
        )
        .expect("level up call should not error even when the build is Blocked");

        match response {
            CreateCharacterResponse::Blocked { diagnostics } => {
                assert!(
                    !diagnostics.is_empty(),
                    "a Blocked response should carry real diagnostics"
                );
            }
            CreateCharacterResponse::Saved { .. } => {
                panic!(
                    "Human Fighter level 20 -> 21 is outside the compute engine's supported \
                     range and must not reach Computed"
                );
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.character_input.chosen.class_levels[0].level, 20,
            "a Blocked leveled-up build must never be persisted"
        );
        assert_eq!(
            reloaded.saved_at, LEVEL_UP_TEST_SAVED_AT,
            "saved_at must be unchanged when the mutation is not persisted"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// `level_up_character_at_root`'s error path when there is nothing saved
    /// at `root` yet — must fail honestly (via `SavedCharacterStore::load`'s
    /// own error) rather than silently creating a character out of thin air.
    #[test]
    fn level_up_character_at_root_fails_honestly_when_nothing_is_saved_yet() {
        let root = tempdir("missing-character");

        let result = level_up_character_at_root(
            &root,
            FIGHTER_CLASS_ID,
            Vec::new(),
            None,
            "2026-07-21T00:00:00Z",
        );

        assert!(result.is_err(), "leveling up a nonexistent saved character must fail");

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- `delete_character` (Criterion 22) -----

    #[test]
    fn delete_character_at_root_removes_the_directory_for_a_saved_character() {
        let root = tempdir("delete-golden-path");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        assert!(root.exists(), "precondition: the character root must exist");

        let response = delete_character_at_root(&root);

        assert!(response.ok, "delete should report ok: true, got error: {:?}", response.error);
        assert!(response.error.is_none());
        assert!(!root.exists(), "the character root must be gone after delete");
    }

    /// Builds a path guaranteed not to exist yet (unlike `tempdir`, which
    /// pre-creates its directory) — mirrors the root `codex` crate's own
    /// `list_all_returns_empty_listing_for_nonexistent_root` idiom.
    fn nonexistent_path(label: &str) -> PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "codex-character-hub-{label}-{}-{unique}",
            std::process::id()
        ))
    }

    #[test]
    fn delete_character_at_root_is_ok_when_nothing_is_saved_yet() {
        let root = nonexistent_path("delete-missing-character");
        assert!(!root.exists(), "precondition: nothing saved at this root");

        let response = delete_character_at_root(&root);

        assert!(
            response.ok,
            "deleting an already-nonexistent character must be an idempotent success, got error: {:?}",
            response.error
        );
        assert!(response.error.is_none());
    }

    // ----- `import_character` (Criterion 23) -----

    /// A known fixture shaped like a full saved-character export (extra
    /// envelope fields `characterId`/`revisionId`/`savedAt` included, to
    /// prove they are tolerated and ignored — importing always mints a
    /// fresh identity). The `characterInput` payload mirrors
    /// `compose_character_input`'s own Human Fighter level 1 golden-path
    /// shape, which the existing
    /// `compose_character_input_reaches_computed_status_for_supported_fighter_levels_1_to_3`
    /// test already proves reaches `Computed`.
    const HUMAN_FIGHTER_IMPORT_FIXTURE_JSON: &str = r#"{
        "characterId": "source-character-id-should-be-ignored",
        "revisionId": "source-character-id-should-be-ignored.rev.1",
        "savedAt": "2020-01-01T00:00:00Z",
        "displayLabel": "Imported Aldric",
        "characterInput": {
            "sourcePackageId": "pf1.core_rulebook",
            "chosen": {
                "raceId": "race:human",
                "classLevels": [{"classId": "class:fighter", "level": 1}],
                "abilityScores": {
                    "strength": 16,
                    "dexterity": 14,
                    "constitution": 14,
                    "intelligence": 10,
                    "wisdom": 12,
                    "charisma": 8
                },
                "selectedFeats": ["feat:power_attack", "feat:dodge", "feat:weapon_focus"],
                "skillAllocations": [
                    {"skillId": "skill:climb", "ranks": 1},
                    {"skillId": "skill:intimidate", "ranks": 1},
                    {"skillId": "skill:swim", "ranks": 1}
                ],
                "equipmentSelections": [
                    {"itemId": "item:longsword", "activeState": "EquippedActive"},
                    {"itemId": "item:chain_shirt", "activeState": "EquippedActive"},
                    {"itemId": "item:shield", "activeState": "Absent"},
                    {"itemId": "power_attack", "activeState": "SelectedInactive"}
                ],
                "selectedChoices": [
                    {"choiceSetId": "choice:level_1_character_feat", "selectionId": "feat:power_attack"},
                    {"choiceSetId": "choice:fighter_bonus_feat", "selectionId": "feat:weapon_focus:weapon:longsword"},
                    {"choiceSetId": "choice:human_bonus_feat", "selectionId": "feat:dodge"},
                    {"choiceSetId": "choice:human_ability_bonus", "selectionId": "ability:strength"}
                ],
                "spellsSelected": [
                    {"spellId": "Alarm", "sourceClassId": "class:demo", "acquisitionMode": "Granted"},
                    {"spellId": "Blur", "sourceClassId": "class:demo", "acquisitionMode": "Granted"}
                ]
            }
        }
    }"#;

    /// The single most important regression guard for Criterion 23: proves
    /// the real parse -> mint-fresh-id -> recompute -> save -> return round
    /// trip against a real `SavedCharacterStore` fixture on disk, not a
    /// mock. Also proves the source JSON's own `characterId` is ignored in
    /// favor of a freshly minted one.
    #[test]
    fn import_character_from_json_saves_a_fresh_character_when_computed() {
        let root = tempdir("import-golden-path");
        let fresh_id = "char-import-fresh-id";

        let response = import_character_from_json(
            HUMAN_FIGHTER_IMPORT_FIXTURE_JSON,
            &root,
            fresh_id,
            "2026-07-21T00:00:00Z",
            "codex-dev-test",
        )
        .expect("import call should not error for well-formed JSON");

        match response {
            CreateCharacterResponse::Saved { summary, .. } => {
                assert_eq!(summary.character_id, fresh_id);
                assert_eq!(summary.display_label, "Imported Aldric");
                assert_eq!(summary.class_summary, "class:fighter:1");
            }
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!(
                    "the Human Fighter level 1 import fixture must reach Computed, got \
                     diagnostics: {diagnostics:?}"
                );
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.character_id, fresh_id,
            "the saved envelope must carry the freshly minted id, not the source JSON's own \
             (ignored) characterId"
        );
        assert_ne!(
            reloaded.character_id, "source-character-id-should-be-ignored",
            "the source JSON's own characterId must never be trusted"
        );
        assert_eq!(reloaded.display_label, "Imported Aldric");
        assert_eq!(reloaded.character_input.chosen.race_id, "race:human");
        assert_eq!(
            reloaded.character_input.chosen.equipment_selections.len(),
            4,
            "the full equipment_selections payload must round-trip"
        );
        assert_eq!(
            reloaded.character_input.chosen.spells_selected.len(),
            2,
            "the full spells_selected payload must round-trip"
        );
        assert_eq!(reloaded.saved_at, "2026-07-21T00:00:00Z");

        std::fs::remove_dir_all(&root).ok();
    }

    /// Malformed JSON is a distinct failure mode from a structurally valid
    /// but non-computing import: it must be a real `Err`, not a silently
    /// empty/successful import and not a `Blocked` response (which implies
    /// the JSON was at least well-formed enough to reach the compute
    /// engine).
    #[test]
    fn import_character_from_json_rejects_malformed_json() {
        let root = nonexistent_path("import-malformed-json");

        let result = import_character_from_json(
            "{ this is not valid json",
            &root,
            "char-import-malformed",
            "2026-07-21T00:00:00Z",
            "codex-dev-test",
        );

        assert!(result.is_err(), "malformed JSON must be rejected as a real error");
        assert!(
            !root.exists(),
            "no character directory may be created for a malformed import"
        );
    }

    /// JSON that parses but is missing a required `CharacterInput` field
    /// (schema-invalid, not merely malformed) must also be rejected, not
    /// half-imported with a default/garbage value.
    #[test]
    fn import_character_from_json_rejects_json_missing_a_required_field() {
        let root = nonexistent_path("import-missing-field");
        // Missing "chosen" entirely, and "sourcePackageId" too.
        let incomplete_json = r#"{"displayLabel": "Incomplete", "characterInput": {}}"#;

        let result = import_character_from_json(
            incomplete_json,
            &root,
            "char-import-incomplete",
            "2026-07-21T00:00:00Z",
            "codex-dev-test",
        );

        assert!(
            result.is_err(),
            "JSON missing required CharacterInput fields must be rejected"
        );
        assert!(!root.exists());
    }

    /// Mirrors `create_character`/`clone_character`/`level_up_character_at_root`'s
    /// "never persist an unproven build" invariant: a structurally valid
    /// import that the compute engine cannot reach `Computed` for must
    /// return `Blocked` with real diagnostics and must not write anything to
    /// disk.
    #[test]
    fn import_character_from_json_returns_blocked_without_saving_when_the_import_does_not_compute() {
        let root = nonexistent_path("import-blocked-path");
        // A Paladin build is outside the compute engine's currently
        // supported set (see claim_blocking_diagnostic_ids_match_the_catalogued_support_shape_per_class
        // above), so this must come back Blocked, not Saved.
        let unsupported_class_json = r#"{
            "displayLabel": "Unsupported Import",
            "characterInput": {
                "sourcePackageId": "pf1.core_rulebook",
                "chosen": {
                    "raceId": "race:human",
                    "classLevels": [{"classId": "class:paladin", "level": 1}],
                    "abilityScores": {
                        "strength": 16, "dexterity": 14, "constitution": 14,
                        "intelligence": 10, "wisdom": 12, "charisma": 8
                    },
                    "selectedFeats": [],
                    "skillAllocations": [],
                    "equipmentSelections": [],
                    "selectedChoices": [],
                    "spellsSelected": []
                }
            }
        }"#;

        let response = import_character_from_json(
            unsupported_class_json,
            &root,
            "char-import-blocked",
            "2026-07-21T00:00:00Z",
            "codex-dev-test",
        )
        .expect("import call should not error even when the build is Blocked");

        match response {
            CreateCharacterResponse::Blocked { diagnostics } => {
                assert!(!diagnostics.is_empty(), "a Blocked response should carry real diagnostics");
            }
            CreateCharacterResponse::Saved { .. } => {
                panic!("a Human Paladin level 1 import is outside supported range and must not reach Computed");
            }
        }

        assert!(!root.exists(), "a Blocked import must never be persisted");
    }

    // ----- `export_character` (Criterion 24) -----

    /// The single most important regression guard for Criterion 24: proves
    /// the real save -> export -> re-import round trip against two real
    /// `SavedCharacterStore` fixtures on disk, not mocks or a hand-built
    /// `{summary, detail}` payload. `level_up_test_envelope`'s Human Fighter
    /// build carries a non-trivial `chosen` (feats, skills, all four
    /// equipment active-state variants, selected choices, and spells), so
    /// this exercises every field `character_input_to_dto`/
    /// `character_input_from_dto` touch, not just the trivial ones.
    #[test]
    fn export_character_to_json_round_trips_through_import_character_from_json() {
        let source_root = tempdir("export-round-trip-source");
        let source_envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&source_envelope, &source_root).expect("seed save should succeed");

        let exported_json = export_character_to_json(&source_root).expect("export should succeed for a real saved character");

        // The exported file is exactly what `import_character` expects:
        // parsing it back out and rebuilding a `CharacterInput` (mirroring
        // what `import_character_from_json` does internally) must produce a
        // `CharacterInput` equal to the source in every field the DTO
        // carries — proving the export DTO and import DTO are structurally
        // compatible by construction, not merely by convention.
        let reparsed: ImportedCharacterFileDto =
            serde_json::from_str(&exported_json).expect("exported JSON must itself be valid ImportedCharacterFileDto JSON");
        assert_eq!(reparsed.display_label, source_envelope.display_label);
        let rebuilt_input = character_input_from_dto(reparsed.character_input, "irrelevant-for-this-comparison");
        assert_eq!(
            rebuilt_input.source_package_id,
            source_envelope.character_input.source_package_id
        );
        assert_eq!(rebuilt_input.chosen, source_envelope.character_input.chosen, "the full `chosen` payload — race, class levels, ability scores, feats, skills, equipment, choices, and spells — must round-trip through export -> import byte-for-byte");

        // Now prove the real end-to-end command path: importing the exported
        // JSON via `import_character_from_json` (the same function
        // `import_character` calls) reaches `Computed` and saves a fresh
        // character whose on-disk `chosen` matches the source.
        let dest_root = tempdir("export-round-trip-dest");
        let response = import_character_from_json(
            &exported_json,
            &dest_root,
            "char-export-round-trip-fresh-id",
            "2026-07-22T00:00:00Z",
            "codex-dev-test",
        )
        .expect("importing a real export must not error");

        match response {
            CreateCharacterResponse::Saved { summary, .. } => {
                assert_eq!(summary.character_id, "char-export-round-trip-fresh-id");
                assert_eq!(summary.display_label, source_envelope.display_label);
            }
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!("a real exported Human Fighter level 1 build must reach Computed on re-import, got diagnostics: {diagnostics:?}");
            }
        }

        let reimported = SavedCharacterStore::load(&dest_root).expect("reimported character should reload");
        assert_eq!(
            reimported.character_input.chosen, source_envelope.character_input.chosen,
            "the character reloaded after export -> import must carry the exact same chosen build as the original"
        );
        assert_ne!(
            reimported.character_id, source_envelope.character_id,
            "import always mints a fresh identity, never reuses the exported character's own id"
        );

        std::fs::remove_dir_all(&source_root).ok();
        std::fs::remove_dir_all(&dest_root).ok();
    }

    /// `export_character_to_json`'s error path when there is nothing saved
    /// at `root` yet — must fail honestly (via `SavedCharacterStore::load`'s
    /// own error) rather than fabricating an empty export.
    #[test]
    fn export_character_to_json_fails_honestly_when_nothing_is_saved_yet() {
        let root = nonexistent_path("export-missing-character");

        let result = export_character_to_json(&root);

        assert!(result.is_err(), "exporting a nonexistent saved character must fail");
    }

    /// v0.6 alpha swarm: the animal companion / mount stat block is fully
    /// computed in the engine and now reaches `PilotSnapshot.companion`
    /// (`pilot_view_model.rs`), but a value that stops at the Tauri DTO
    /// boundary is invisible to the player -- exactly the defect shape
    /// `EquipmentEffects.per_item` had (fully populated, simply not
    /// carried, leaving an "AC breakdown by source" panel sitting as a
    /// placeholder over data that already existed). This proves the
    /// boundary genuinely carries it.
    #[test]
    fn snapshot_dto_carries_a_real_animal_companion_across_the_boundary() {
        // Deliberately `resolve_unified_pilot_snapshot`, not
        // `PilotViewModel::from_receipt`: that is the function
        // `load_saved_character` itself calls, and it assembles its own
        // `PilotSnapshot` by hand rather than going through the view
        // model. Testing the view model here would have proved nothing
        // about what the live app actually receives.
        let (snapshot, _corpus_receipt) = resolve_unified_pilot_snapshot(
            &human_druid_with_animal_companion(),
            corpus_fixture_bundle(),
        )
        .expect("a level-1 Druid with an animal companion resolves a snapshot");

        let dto = map_snapshot_dto(&snapshot);

        let companion = dto
            .companion
            .as_ref()
            .expect("the Druid's real Wolf companion must cross the DTO boundary");
        assert_eq!(companion.owner_class_label, "Druid");
        assert_eq!(companion.species, "Wolf");
        assert!(
            companion
                .stats
                .iter()
                .any(|stat| stat.label == "Armor Class" && stat.value == 12),
            "the real computed values must survive the crossing: {:?}",
            companion.stats
        );
        assert!(
            companion.advancement_note.is_some(),
            "the engine's honest not-grounded list must cross too -- a Computed load reports \
             no diagnostics at all, so this is the player's only route to it"
        );

        // camelCase on the wire, and genuinely omitted (not `null`) for a
        // companion-less class -- the same `skip_serializing_if` discipline
        // `damage_reduction` and `EquipmentEffectsDto` already needed, for
        // the same frontend `!== undefined` reason.
        let json = serde_json::to_string(&dto).expect("serialization should succeed");
        assert!(json.contains("\"companion\""), "companion must be on the wire: {json}");
        assert!(json.contains("\"ownerClassLabel\":\"Druid\""), "camelCase on the wire: {json}");
        assert!(json.contains("\"advancementNote\""), "camelCase on the wire: {json}");
    }

    #[test]
    fn snapshot_dto_omits_the_companion_key_entirely_for_a_companionless_class() {
        let result = codex::rules_core::character_input::load_character_input_fixture(
            HUMAN_FIGHTER_LEVEL_1_FIXTURE,
        );
        let input = result.character_input.expect("valid fixture");
        let (snapshot, _corpus_receipt) =
            resolve_unified_pilot_snapshot(&input, corpus_fixture_bundle())
                .expect("a Human Fighter level 1 resolves a snapshot");

        let json = serde_json::to_string(&map_snapshot_dto(&snapshot))
            .expect("serialization should succeed");

        assert!(
            !json.contains("companion"),
            "a Fighter has no companion -- the key must be absent, not a literal null the \
             frontend's `!== undefined` checks would wave through into an empty stat block: \
             {json}"
        );
    }

    const HUMAN_FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    /// Mirrors the engine's own Druid-with-companion test input: the
    /// deterministic Human fighter fixture with its class levels replaced
    /// by Druid 1, plus the animal-companion nature bond.
    fn human_druid_with_animal_companion() -> codex::rules_core::character_input::CharacterInput {
        use codex::rules_core::character_input::{CharacterClassLevel, SelectedChoice};

        let result = codex::rules_core::character_input::load_character_input_fixture(
            HUMAN_FIGHTER_LEVEL_1_FIXTURE,
        );
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels = vec![CharacterClassLevel {
            class_id: "class:druid".to_owned(),
            level: 1,
        }];
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:druid_nature_bond".to_owned(),
            selection_id: "bond:animal_companion".to_owned(),
        });
        input
    }

    // ----- Receipt-to-Sheet slice 1: the explanation channel, the weapon
    // line, and the engine-backed level-up preview -----

    /// Saves `input` under a fresh temp root and returns that root, so the
    /// `*_at_root` load/preview seams can be exercised without an
    /// `AppHandle`.
    fn saved_root_for(label: &str, input: CharacterInput) -> PathBuf {
        let root = tempdir(label).join("char-receipt-to-sheet");
        let envelope = SavedCharacterEnvelope {
            character_id: "char-receipt-to-sheet".to_owned(),
            revision_id: "char-receipt-to-sheet.rev.1".to_owned(),
            revision_kind: SavedCharacterRevisionKind::Authoritative,
            saved_at: LEVEL_UP_TEST_SAVED_AT.to_owned(),
            schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
            app_or_runtime_version: "codex-dev".to_owned(),
            content_or_rules_provenance: SOURCE_PACKAGE_ID.to_owned(),
            game_system: GAME_SYSTEM_ID.to_owned(),
            latest_authoritative_revision_ref: "char-receipt-to-sheet.rev.1".to_owned(),
            display_label: "Receipt To Sheet".to_owned(),
            character_input: input,
        };
        SavedCharacterStore::save(&envelope, &root).expect("envelope should save");
        root
    }

    /// The scoping doc's headline acceptance case: a level-11 Rogue's
    /// sneak-attack die count is computed, corpus-cited — and, before this
    /// slice, dropped at the IPC boundary because
    /// `LoadSavedCharacterResponse` had no field for it.
    #[test]
    fn load_saved_character_carries_a_level_11_rogues_sneak_attack_explanation_across_the_boundary()
    {
        let input = compose_character_input(&request_for_class("race:human", "class:rogue", 11));
        let root = saved_root_for("rogue-11-sneak-attack", input);

        let response = load_saved_character_at_root(&root).expect("saved rogue should load");

        let sneak_attack = response
            .explanations
            .iter()
            .find(|explanation| explanation.id == "class_chassis.rogue.sneak_attack")
            .expect("a level-11 Rogue must carry a sneak-attack explanation record");

        assert_eq!(
            sneak_attack.value, 6,
            "PF1 Rogue sneak attack at level 11 is (11 + 1) / 2 = 6d6"
        );
        assert!(
            sneak_attack.detail.contains("6d6"),
            "the engine's own detail text must cross verbatim: {}",
            sneak_attack.detail
        );
    }

    /// `detail` must arrive byte-identical to what the engine produced —
    /// the frontend renders it as rules prose, so any rewriting on the way
    /// across would create a second, unverified source of it.
    #[test]
    fn load_saved_character_carries_every_explanation_detail_verbatim() {
        let input = compose_character_input(&request_for_class("race:human", "class:rogue", 11));
        let root = saved_root_for("verbatim-details", input.clone());

        let response = load_saved_character_at_root(&root).expect("saved rogue should load");
        let engine = compute_pilot_with_corpus(&input, corpus_fixture_bundle());

        assert_eq!(
            response.explanations.len(),
            engine.base.explanations.len(),
            "no explanation record may be dropped at the boundary"
        );
        for (wire, engine_record) in response.explanations.iter().zip(&engine.base.explanations) {
            assert_eq!(wire.id, engine_record.id);
            assert_eq!(wire.value, engine_record.value);
            assert_eq!(
                wire.detail, engine_record.detail,
                "detail for {} must cross verbatim",
                engine_record.id
            );
        }
    }

    /// Class-feature records specifically — the set the sheet's Class
    /// Features section renders — must be non-empty for a grounded build.
    #[test]
    fn load_saved_character_carries_class_feature_records_for_a_fighter() {
        let input = compose_character_input(&request_for("race:human", 5));
        let root = saved_root_for("fighter-class-features", input);

        let response = load_saved_character_at_root(&root).expect("saved fighter should load");

        let class_records: Vec<&str> = response
            .explanations
            .iter()
            .map(|explanation| explanation.id.as_str())
            .filter(|id| id.starts_with("class_feature.") || id.starts_with("class_chassis."))
            .collect();

        assert!(
            class_records
                .iter()
                .any(|id| *id == "class_feature.fighter.bravery"),
            "a level-5 Fighter must carry its Bravery record: {class_records:?}"
        );
        assert!(
            class_records
                .iter()
                .any(|id| *id == "class_feature.fighter.armor_training"),
            "a level-5 Fighter must carry its Armor Training record: {class_records:?}"
        );
        // `class_feature.fighter.weapon_training` is deliberately NOT
        // asserted here: it is gated on the
        // `choice:fighter_weapon_training_group` selection, which
        // `compose_character_input` does not seed. Its absence is honest
        // absence, and the sheet must render the records that exist rather
        // than inventing the ones that do not.
    }

    /// The Weapons tab's acceptance case: an equipped longsword must
    /// produce a real, populated breakdown on the wire. Before this slice
    /// `damage_total.rs` computed all of this and nothing carried it.
    #[test]
    fn load_saved_character_carries_the_equipped_longswords_damage_breakdown() {
        let input = compose_character_input(&request_for("race:human", 1));
        assert!(
            input
                .chosen
                .equipment_selections
                .iter()
                .any(|selection| selection.item_id == "item:longsword"),
            "the deterministic loadout is expected to equip a longsword"
        );
        let root = saved_root_for("longsword-breakdown", input);

        let response = load_saved_character_at_root(&root).expect("saved fighter should load");

        let longsword = response
            .weapon_damage
            .iter()
            .find(|weapon| weapon.weapon_item_id == "item:longsword")
            .expect("an equipped longsword must produce a weapon-damage row");

        let base_dice = longsword.base_dice.expect("longsword carries a DAMAGE: token");
        assert_eq!((base_dice.count, base_dice.die_size), (1, 8), "1d8");
        assert_eq!(
            longsword.critical_threat_range,
            Some([19, 20]),
            "CRITRANGE:2 means a threat on a natural 19-20"
        );
        assert_eq!(longsword.critical_multiplier, Some(2), "CRITMULT:x2");
        assert_eq!(longsword.wield_category.as_deref(), Some("OneHanded"));
        assert_eq!(
            longsword.str_damage_modifier,
            Some(4),
            "STR 16 + the PF1 Standard Human +2 racial = 18 -> +4, applied in \
             full for a one-handed weapon"
        );
        assert_eq!(
            longsword.weapon_record_key.as_deref(),
            Some("Longsword (Base)")
        );
    }

    /// Honest absence, not a fabricated default: the Chain Shirt is
    /// equipped too, and it is not a weapon — it must simply be absent
    /// from `weapon_damage`, never present with zeroed facets.
    #[test]
    fn load_saved_character_omits_non_weapon_equipped_items_from_weapon_damage() {
        let input = compose_character_input(&request_for("race:human", 1));
        let root = saved_root_for("non-weapon-omitted", input);

        let response = load_saved_character_at_root(&root).expect("saved fighter should load");

        assert!(
            !response
                .weapon_damage
                .iter()
                .any(|weapon| weapon.weapon_item_id == "item:chain_shirt"),
            "armor must not appear as a weapon row: {:?}",
            response
                .weapon_damage
                .iter()
                .map(|weapon| &weapon.weapon_item_id)
                .collect::<Vec<_>>()
        );
    }

    /// The engine-backed replacement for the deleted frontend
    /// `CLASS_FEATURES` table. The hand-authored table said Fighter level 2
    /// grants `['Bravery +1', 'Bonus combat feat']` — two bare labels, no
    /// magnitude, no citation. The engine's real answer for the same
    /// transition is Bravery plus the base-attack-bonus and Fortitude-save
    /// steps, each carrying its own value and table provenance.
    ///
    /// **The bonus combat feat is genuinely not in that answer**, and this
    /// test pins that rather than papering over it:
    /// `level_up/fighter.rs`'s own module doc records that
    /// `pick_from_lists` stays empty for Fighter's ten Bonus Feat slots
    /// (composing a real candidate list needs PF1 Combat-Feat eligibility
    /// filtering plus per-candidate prerequisite evaluation — a documented,
    /// bounded scope note left as that cycle's `next_required_uplift`), and
    /// `class_feature.fighter.level_2_bonus_feat` only fires once
    /// `choice:fighter_bonus_feat_2` has actually been selected, which is
    /// after the level-up, not before it. Re-adding a hand-authored
    /// `'Bonus combat feat'` string to cover the gap would be exactly the
    /// uncited-rules-data debt this slice exists to remove.
    #[test]
    fn preview_level_up_reports_fighters_real_level_2_grants() {
        let input = compose_character_input(&request_for("race:human", 1));
        let root = saved_root_for("preview-fighter-2", input);

        let preview =
            preview_level_up_at_root(&root, FIGHTER_CLASS_ID).expect("preview should compute");

        assert_eq!(preview.from_level, 1);
        assert_eq!(preview.to_level, 2);
        assert_eq!(preview.character_level, 2);

        let names: Vec<&str> = preview
            .automatic_features
            .iter()
            .map(|grant| grant.name.as_str())
            .collect();
        assert!(
            names.iter().any(|name| name.contains("bravery")),
            "Fighter's level-2 Bravery must be a reported grant: {names:?}"
        );
        assert!(
            names
                .iter()
                .any(|name| name.contains("base_attack_bonus")),
            "the level-2 base-attack-bonus step must be reported: {names:?}"
        );
        assert!(
            preview.automatic_features.iter().all(|grant| !grant
                .effects
                .iter()
                .any(|effect| effect.description.is_empty())),
            "every reported grant effect must carry the engine's own description"
        );
    }

    /// A class the character does not hold yet previews as a level-1 dip,
    /// not as a level-up of something they have.
    #[test]
    fn preview_level_up_treats_an_unheld_class_as_a_fresh_level_1_dip() {
        let input = compose_character_input(&request_for("race:human", 3));
        let root = saved_root_for("preview-wizard-dip", input);

        let preview =
            preview_level_up_at_root(&root, "class:wizard").expect("preview should compute");

        assert_eq!(preview.from_level, 0);
        assert_eq!(preview.to_level, 1);
        assert_eq!(preview.character_level, 4);
    }
}

