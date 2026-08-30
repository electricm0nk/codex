//! `RuleSystemAdapter` — the rule-system-agnostic seam Criterion 3.1 defines
//! (`epic-breakdown.md` §3.1, cycle doc `cycles/3_1.md`, Epic 3 "Character
//! Hub as Hub of Hubs").
//!
//! Today `character_hub.rs` and the `characterHub::{appendToCharacter,
//! recomputeCharacter, reSaveCharacter}` command modules hard-code calls
//! directly into Pf1-specific free functions
//! (`compute_pilot_base_chassis`, `compute_level_up_grants`,
//! `re_save_character_at_root`, `append_to_character_at_root`,
//! `recompute_character_at_root`, `SavedCharacterStore::list_all` /
//! `::load`). This trait is the seam every rule system implements once —
//! Pf1 today (extraction lands at `pf1_adapter.rs`, criterion 3.2), a
//! future-system stub next (`stub_adapter.rs`, criterion 3.3) — so the
//! Tauri command surface (criterion 3.4) and the UI (criterion 3.5) can
//! dispatch on a `rule_system_id: String` through `dyn RuleSystemAdapter`
//! instead of calling Pf1's free functions by name.
//!
//! This cycle only defines the trait surface and proves it is genuinely
//! implementable end-to-end (the inline test below delegates every method
//! to this crate's real, already-landed Pf1 free functions — every
//! returned value comes from real compute/persistence, none invented).
//! The actual `Pf1Adapter` extraction that
//! makes `character_hub.rs`/`characterHub::*` *route through* an
//! implementation of this trait is criterion 3.2's job, not this one's;
//! this cycle's own file-touch grant (`cycles/3_1.md`) is scoped to this
//! new file plus a `main.rs` module-declaration line.
//!
//! Object-safe by construction (no generic methods) so callers can hold
//! `Box<dyn RuleSystemAdapter>` / `&dyn RuleSystemAdapter` and dispatch on
//! a runtime-selected `rule_system_id`, per criterion 3.4's Tauri
//! command-surface routing.

use std::path::Path;

use codex::rules_core::character_input::CharacterInput;
use codex::rules_core::level_up::LevelUpPlan;
use codex::rules_core::pilot_compute::PilotBaseChassisComputation;

use crate::character_hub::ListSavedCharactersResponse;
use crate::character_hub::LoadSavedCharacterResponse;
use crate::characterHub::appendToCharacter::{AppendToCharacterResponse, ItemToAppendDto};
use crate::characterHub::recomputeCharacter::RecomputeCharacterResponse;
use crate::characterHub::reSaveCharacter::ReSaveCharacterResponse;

/// One class's from/to sub-level delta within a (possibly multiclass)
/// level-up request.
///
/// SD-24 carry-forward (`sd24-carry-forward-register.md` A2, shared with
/// criterion 3.2): `level_up::compute_level_up_grants(character,
/// from_level, to_level)` dispatches on a single implicit class read off
/// `character.chosen.class_levels` (its `[class_level]` single-element
/// slice pattern) — any multiclass mix, including a plain Fighter+Wizard
/// pair, falls through to its wildcard arm and returns an honestly-empty
/// `LevelUpPlan::default()` rather than routing to the per-class functions
/// for *either* class. This trait's `level_up` takes an explicit slice of
/// `ClassLevelDelta` instead of inferring a single class/level pair from
/// the character, so a caller can state "Fighter 3->4, Wizard 2->2" (a
/// level-up that only advances one class of a multiclass build)
/// unambiguously. `compute_level_up_grants`'s own dispatch gap is not
/// fixed by this cycle — that widening is criterion 3.2's job, done at the
/// `Pf1Adapter` implementation, once this trait's signature already makes
/// the multiclass shape expressible.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassLevelDelta {
    pub class_id: String,
    pub from_level: u8,
    pub to_level: u8,
}

/// The rule-system-agnostic hub-of-hubs seam. One implementation per rule
/// system (Pf1 today; future systems register their own).
pub trait RuleSystemAdapter {
    /// Stable identifier for the rule system this adapter implements
    /// (e.g. `"pf1"`) — the dispatch key criterion 3.4's Tauri commands
    /// use to pick an adapter for a given `rule_system_id: String`
    /// argument.
    fn rule_system_id(&self) -> &'static str;

    /// Resolves a character's base chassis (ability modifiers, BAB,
    /// saves, baseline melee attack bonus, baseline armor class,
    /// explanations, diagnostics) from its raw input. Pf1's
    /// implementation wraps `pilot_compute::compute_pilot_base_chassis`.
    fn chassis_resolve(&self, input: &CharacterInput) -> PilotBaseChassisComputation;

    /// Computes the grants one or more simultaneous class-level deltas
    /// produce (see `ClassLevelDelta`'s doc comment for why this is a
    /// slice rather than a single implied class/level pair).
    fn level_up(&self, character: &CharacterInput, deltas: &[ClassLevelDelta]) -> LevelUpPlan;

    /// Re-saves the character already persisted at `root` under a freshly
    /// minted revision, refusing the write (`success: false, error:
    /// "revision_conflict"`) if `expected_revision_id` does not match the
    /// canonical on-disk revision. Pf1's implementation wraps
    /// `characterHub::reSaveCharacter::re_save_character_at_root`.
    fn save_character(
        &self,
        root: &Path,
        expected_revision_id: &str,
        saved_at: &str,
    ) -> Result<ReSaveCharacterResponse, String>;

    /// Validates and appends a batch of items to the character persisted
    /// at `root` — either every item is real and gets appended, or none
    /// are. Pf1's implementation wraps
    /// `characterHub::appendToCharacter::append_to_character_at_root`.
    fn append_to_character(
        &self,
        root: &Path,
        items_to_append: &[ItemToAppendDto],
        saved_at: &str,
    ) -> Result<AppendToCharacterResponse, String>;

    /// Recomputes the character persisted at `root` through the real
    /// compute engine without mutating its stored input. Pf1's
    /// implementation wraps
    /// `characterHub::recomputeCharacter::recompute_character_at_root`.
    fn recompute(&self, root: &Path, character_id: &str) -> RecomputeCharacterResponse;

    /// Lists every saved character under `characters_root` that this
    /// adapter's rule system owns. Pf1's implementation wraps
    /// `SavedCharacterStore::list_all` plus `character_hub`'s own
    /// summary-DTO projection.
    fn list_saved_characters(
        &self,
        characters_root: &Path,
    ) -> Result<ListSavedCharactersResponse, String>;

    /// Loads one saved character by its root path, recomputing its
    /// current snapshot/diagnostics/corpus-derived data fresh (never
    /// cached). Pf1's implementation wraps `SavedCharacterStore::load`
    /// plus `character_hub`'s own load-and-project pipeline.
    fn load_saved_character(&self, root: &Path) -> Result<LoadSavedCharacterResponse, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    use codex::rules_core::level_up::compute_level_up_grants;
    use codex::rules_core::pilot_compute::build_pilot_headless_receipt;
    use codex::rules_core::pilot_compute_corpus::compute_pilot_with_corpus;
    use codex::rules_core::pilot_view_model::PilotViewModel;
    use codex::saved_character::local_store::SavedCharacterStore;
    use codex::saved_character::{
        SavedCharacterEnvelope, SavedCharacterRevisionKind, CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
    };

    use crate::character_hub::{
        compose_character_input, map_chosen_feat_targets_dto, map_encumbrance_dto,
        map_explanations_dto, map_resolved_equipment_dto, map_snapshot_dto,
        map_spells_selected_dto, map_weapon_damage_dto, read_alternate_trait_keys,
        resolve_racial_traits_for_character, AbilityScoresDto,
        CharacterSummaryDto, CorpusDerivedDto, CreateCharacterRequest, DiagnosticDto,
        EquipmentEffectsDto, ResolvedEquipmentEffectDto, SchoolCoverageDto,
    };
    use codex::rules_core::damage_total::resolve_weapon_damage_breakdown;
    use crate::characterHub::appendToCharacter::append_to_character_at_root;
    use crate::characterHub::recomputeCharacter::recompute_character_at_root;
    use crate::characterHub::reSaveCharacter::re_save_character_at_root;
    use crate::corpus_fixtures::corpus_fixture_bundle;

    use std::path::PathBuf;

    /// Delegates every trait method to this crate's real, already-landed
    /// Pf1 free functions — proof that the trait shape genuinely maps
    /// onto real behavior, not just a mechanical compile check. The
    /// `list_saved_characters`/`load_saved_character` bodies re-derive
    /// `character_hub.rs`'s own (private) summary/snapshot projection
    /// inline, since those helpers are not part of this cycle's
    /// file-touch grant; `Pf1Adapter` (criterion 3.2) is where that
    /// projection is formally extracted and reused instead of
    /// re-derived.
    struct TestPf1Delegate;

    impl RuleSystemAdapter for TestPf1Delegate {
        fn rule_system_id(&self) -> &'static str {
            "pf1"
        }

        fn chassis_resolve(&self, input: &CharacterInput) -> PilotBaseChassisComputation {
            codex::rules_core::pilot_compute::compute_pilot_base_chassis(input)
        }

        fn level_up(&self, character: &CharacterInput, deltas: &[ClassLevelDelta]) -> LevelUpPlan {
            let mut plan = LevelUpPlan::default();
            for delta in deltas {
                let sub = compute_level_up_grants(character, delta.from_level, delta.to_level);
                plan.automatic_features.extend(sub.automatic_features);
                plan.pick_from_lists.extend(sub.pick_from_lists);
                plan.prerequisites_added.extend(sub.prerequisites_added);
                plan.capstone_threshold = plan.capstone_threshold || sub.capstone_threshold;
            }
            plan
        }

        fn save_character(
            &self,
            root: &Path,
            expected_revision_id: &str,
            saved_at: &str,
        ) -> Result<ReSaveCharacterResponse, String> {
            re_save_character_at_root(root, expected_revision_id, saved_at)
        }

        fn append_to_character(
            &self,
            root: &Path,
            items_to_append: &[ItemToAppendDto],
            saved_at: &str,
        ) -> Result<AppendToCharacterResponse, String> {
            append_to_character_at_root(root, items_to_append, saved_at)
        }

        fn recompute(&self, root: &Path, character_id: &str) -> RecomputeCharacterResponse {
            recompute_character_at_root(root, character_id)
        }

        fn list_saved_characters(
            &self,
            characters_root: &Path,
        ) -> Result<ListSavedCharactersResponse, String> {
            let listing =
                SavedCharacterStore::list_all(characters_root).map_err(|err| err.message)?;
            Ok(ListSavedCharactersResponse {
                characters: listing
                    .characters
                    .iter()
                    .map(|summary| CharacterSummaryDto {
                        character_id: summary.character_id.clone(),
                        display_label: summary.display_label.clone(),
                        game_system: summary.game_system.clone(),
                        schema_version: summary.schema_version,
                        saved_at: summary.saved_at.clone(),
                        race_id: summary.race_id.clone(),
                        class_summary: summary.class_summary.clone(),
                    })
                    .collect(),
                unreadable_count: listing.unreadable_entries.len(),
            })
        }

        fn load_saved_character(&self, root: &Path) -> Result<LoadSavedCharacterResponse, String> {
            let envelope = SavedCharacterStore::load(root).map_err(|err| err.message)?;

            let summary = CharacterSummaryDto {
                character_id: envelope.character_id.clone(),
                display_label: envelope.display_label.clone(),
                game_system: envelope.game_system.clone(),
                schema_version: envelope.schema_version,
                saved_at: envelope.saved_at.clone(),
                race_id: envelope
                    .character_input
                    .chosen
                    .race_id
                    .clone(),
                class_summary: envelope
                    .character_input
                    .chosen
                    .class_levels
                    .iter()
                    .map(|class_level| format!("{}:{}", class_level.class_id, class_level.level))
                    .collect::<Vec<_>>()
                    .join(", "),
            };

            let receipt = build_pilot_headless_receipt(&envelope.character_input);
            let view_model = PilotViewModel::from_receipt(&receipt);
            let corpus_receipt =
                compute_pilot_with_corpus(&envelope.character_input, corpus_fixture_bundle());

            // Routed through `character_hub`'s own `map_snapshot_dto`
            // rather than the hand-rolled field-by-field mirror this used
            // to be. That mirror was a silent drop hazard by construction:
            // every field added to `PilotSnapshotDto` had to be remembered
            // in two places, and a forgotten one here would leave this
            // adapter's callers reading a snapshot missing data the engine
            // had computed -- the same class of gap the companion field
            // itself was added to close.
            let snapshot = view_model.snapshot.as_ref().map(map_snapshot_dto);

            let diagnostics = receipt
                .computation
                .diagnostics
                .iter()
                .map(|diagnostic| DiagnosticDto {
                    id: diagnostic.id.clone(),
                    message: diagnostic.message.clone(),
                    claim_blocking: diagnostic.claim_blocking,
                })
                .collect();

            let corpus_derived = CorpusDerivedDto {
                school_coverage: corpus_receipt
                    .corpus_derived
                    .school_coverage
                    .values()
                    .map(|coverage| SchoolCoverageDto {
                        school: format!("{:?}", coverage.school),
                        spells: coverage.spells.clone(),
                        grounded: coverage.table_cell.is_some(),
                    })
                    .collect(),
                equipped_items: corpus_receipt
                    .corpus_derived
                    .equipped_items
                    .iter()
                    .map(map_resolved_equipment_dto)
                    .collect(),
                equipment_effects: EquipmentEffectsDto {
                    per_item: corpus_receipt
                        .corpus_derived
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
                    armor_class_delta: corpus_receipt.corpus_derived.equipment_effects.armor_class_delta,
                    armor_check_penalty_total: corpus_receipt
                        .corpus_derived
                        .equipment_effects
                        .armor_check_penalty_total,
                    max_dex_cap: corpus_receipt.corpus_derived.equipment_effects.max_dex_cap,
                    spell_failure_chance: corpus_receipt
                        .corpus_derived
                        .equipment_effects
                        .spell_failure_chance,
                    attack_bonus_delta: corpus_receipt
                        .corpus_derived
                        .equipment_effects
                        .attack_bonus_delta,
                    spell_resistance_total: corpus_receipt
                        .corpus_derived
                        .equipment_effects
                        .spell_resistance_total,
                },
                encumbrance: map_encumbrance_dto(&corpus_receipt.corpus_derived.encumbrance),
                unresolved_spell_ids: corpus_receipt.corpus_derived.unresolved_spell_ids.clone(),
                unresolved_equipment_item_ids: corpus_receipt
                    .corpus_derived
                    .unresolved_equipment_item_ids
                    .clone(),
            };

            Ok(LoadSavedCharacterResponse {
                summary,
                snapshot,
                diagnostics,
                corpus_derived,
                selected_feats: envelope.character_input.chosen.selected_feats.clone(),
                spells_selected: map_spells_selected_dto(&envelope.character_input.chosen.spells_selected),
                chosen_feat_targets: map_chosen_feat_targets_dto(&envelope.character_input),
                explanations: map_explanations_dto(&corpus_receipt.base.explanations),
                weapon_damage: map_weapon_damage_dto(&resolve_weapon_damage_breakdown(
                    &envelope.character_input,
                    corpus_fixture_bundle(),
                    &corpus_receipt.corpus_derived.equipment_effects,
                    corpus_receipt.base.ability_modifiers.strength,
                )),
                selected_alternate_trait_keys: read_alternate_trait_keys(&envelope.character_input),
                selected_traits: envelope.character_input.chosen.selected_traits.clone(),
                resolved_racial_traits: resolve_racial_traits_for_character(
                    &envelope.character_input,
                ),
            })
        }
    }

    fn tempdir(label: &str) -> PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "codex-rule-system-adapter-test-{label}-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir_all(&path).expect("temp dir should be creatable");
        path
    }

    const TEST_SAVED_AT: &str = "2026-07-21T00:00:00Z";

    fn seed_envelope(character_id: &str) -> SavedCharacterEnvelope {
        let request = CreateCharacterRequest {
            character_id: character_id.to_owned(),
            display_label: "Adapter Trait Test Character".to_owned(),
            race_id: "race:human".to_owned(),
            class_id: "class:fighter".to_owned(),
            level: 1,
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
            saved_at: TEST_SAVED_AT.to_owned(),
        };
        let character_input = compose_character_input(&request);
        SavedCharacterEnvelope {
            character_id: character_id.to_owned(),
            revision_id: format!("{character_id}.rev.1"),
            revision_kind: SavedCharacterRevisionKind::Authoritative,
            saved_at: TEST_SAVED_AT.to_owned(),
            schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
            app_or_runtime_version: "codex-dev".to_owned(),
            content_or_rules_provenance: "pf1.core_rulebook".to_owned(),
            game_system: "pf1".to_owned(),
            latest_authoritative_revision_ref: format!("{character_id}.rev.1"),
            display_label: "Adapter Trait Test Character".to_owned(),
            character_input,
        }
    }

    /// RED (per `cycles/3_1.md`): before this trait existed, this test file
    /// referenced `super::RuleSystemAdapter` in an `impl` block and a `Box<dyn
    /// super::RuleSystemAdapter>` construction with no trait definition present
    /// — `cargo check --tests` failed with `error[E0405]: cannot find trait
    /// 'RuleSystemAdapter' in module 'super'` at both reference sites. GREEN
    /// (this file, present tense): the trait above is defined with the full
    /// seven-method surface from `epic-breakdown.md` §3.1, and every method is
    /// exercised here through a `Box<dyn RuleSystemAdapter>` trait object built
    /// from a delegate wired to real crate functions.
    #[test]
    fn dyn_rule_system_adapter_constructs_and_dispatches_every_method() {
        let adapter: Box<dyn RuleSystemAdapter> = Box::new(TestPf1Delegate);
        assert_eq!(adapter.rule_system_id(), "pf1");

        let characters_root = tempdir("golden-path");
        let root = characters_root.join("adapter-trait-test-char");
        let envelope = seed_envelope("adapter-trait-test-char");
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        // chassis_resolve: real compute over a real CharacterInput.
        let chassis = adapter.chassis_resolve(&envelope.character_input);
        assert_eq!(
            chassis.base_attack_bonus, 1,
            "human fighter level 1 should reach the grounded Fighter chassis"
        );

        // level_up: explicit per-class delta slice (register A2's widened shape).
        let deltas = [ClassLevelDelta {
            class_id: "class:fighter".to_owned(),
            from_level: 1,
            to_level: 2,
        }];
        let plan = adapter.level_up(&envelope.character_input, &deltas);
        assert!(
            !plan.automatic_features.is_empty(),
            "Fighter 1->2 should grant at least one automatic feature"
        );

        // load_saved_character: real disk load + real compute projection.
        let loaded = adapter
            .load_saved_character(&root)
            .expect("load_saved_character should succeed for a seeded character");
        assert_eq!(loaded.summary.character_id, "adapter-trait-test-char");
        assert!(
            loaded.snapshot.is_some(),
            "a Computed Fighter level 1 build should carry a snapshot"
        );

        // list_saved_characters: real disk listing under the shared parent.
        let listing = adapter
            .list_saved_characters(&characters_root)
            .expect("list_saved_characters should succeed");
        assert!(
            listing
                .characters
                .iter()
                .any(|summary| summary.character_id == "adapter-trait-test-char"),
            "the seeded character should appear in the listing"
        );

        // append_to_character: validated batch append against the real corpus.
        let append_result = adapter
            .append_to_character(
                &root,
                &[ItemToAppendDto {
                    item_id: "Dagger (Base)".to_owned(),
                    active_state: crate::character_hub::ActiveStateDto::EquippedActive,
                }],
                "2026-07-21T01:00:00Z",
            )
            .expect("append_to_character should not error");
        assert!(
            append_result.success || append_result.error.is_some(),
            "append_to_character must return an honest success or a real error, never neither"
        );

        // recompute: real read-and-recompute, no mutation.
        let recomputed = adapter.recompute(&root, "adapter-trait-test-char");
        assert!(
            recomputed.success,
            "recompute over an already-Computed character should succeed: {:?}",
            recomputed.error
        );

        // save_character: real re-save via the revision-conflict-checked path.
        // Reads the on-disk revision_id fresh rather than reusing the
        // pre-append `envelope.revision_id`: SD-25 Criterion 3.2 (register
        // A5, landed in this branch's history after this test was first
        // written) folded revision-advancing into
        // `mutate_saved_character_at_root`, so `append_to_character` above
        // already advanced the on-disk revision past the seed's `.rev.1`.
        let post_append_revision_id = SavedCharacterStore::load(&root)
            .expect("reload after append should succeed")
            .revision_id;
        let saved = adapter
            .save_character(&root, &post_append_revision_id, "2026-07-21T02:00:00Z")
            .expect("save_character should not error");
        assert!(
            saved.success,
            "save_character with the correct expected_revision_id should succeed: {:?}",
            saved.error
        );
        assert_eq!(
            saved.revision_id.as_deref(),
            Some("adapter-trait-test-char.rev.3"),
            "rev.1 (seed) -> rev.2 (append_to_character's own revision-advance, register A5) \
             -> rev.3 (this save_character call)"
        );

        // save_character: the revision-conflict guard is real, not bypassed.
        let conflict = adapter
            .save_character(&root, "adapter-trait-test-char.rev.999-stale", "2026-07-21T03:00:00Z")
            .expect("save_character should not error even on a conflict rejection");
        assert!(!conflict.success);
        assert_eq!(conflict.error.as_deref(), Some("revision_conflict"));

        std::fs::remove_dir_all(&characters_root).ok();
    }
}
