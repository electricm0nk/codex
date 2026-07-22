//! `Pf1Adapter` — the Pathfinder 1e rule-system's adapter implementation
//! (SD-25 Epic 3 "Hub of Hubs" refactor, Criterion 3.2).
//!
//! Extracted out of `character_hub.rs`, which previously carried this
//! module's mutation logic inline alongside its Tauri command wrappers and
//! wire DTOs. `character_hub.rs` keeps its `#[tauri::command]` surface and
//! DTOs (they are also depended on by `characterHub::appendToCharacter` /
//! `characterHub::reSaveCharacter`, outside this criterion's file-touch
//! grant) and now calls into the functions this module owns; this module
//! owns the actual PF1 rules-application logic: composing a fresh
//! `CharacterInput` from a create-character request, applying one bounded
//! mutation (level up / add equipment / add spell) to an already-saved
//! character's `CharacterInput`, and the shared
//! load -> mutate -> recompute -> re-save -> return-envelope tail every
//! mutation op shares.
//!
//! `Pf1Adapter` (the zero-sized struct below) is the type this criterion's
//! own doc (`cycles/3_2.md`) names as the destination for
//! `impl RuleSystemAdapter for Pf1Adapter`. Criterion 3.1's
//! `RuleSystemAdapter` trait (`rule_system_adapter.rs`) had not landed on
//! `tranche/5-3` as of this cycle's own dispatch (parallel: yes, isolation:
//! worktree — a sibling cycle, not a prerequisite this cycle blocks on per
//! `cycles/3_2.md`'s own "Gated on: E2 complete" line) — `Pf1Adapter::level_up`
//! was written first as a real, wired, tested inherent method under the
//! trait surface's method name, ready to receive the `impl` once 3.1
//! landed. 3.1 landed mid-cycle (a later rebase during this cycle's own
//! push picked it up); the `impl RuleSystemAdapter for Pf1Adapter` block
//! below was then added in the same cycle rather than deferred, using
//! `compute_level_up_grants_for_class` (this cycle's own register-A2 fix,
//! `src/rules_core/level_up.rs`) as `level_up`'s real per-delta dispatch —
//! the fix and its production call site land together, not in two
//! separate cycles.
//!
//! ## Carry-forward register A5 (operator-confirmed, `decisions.md §11`)
//!
//! `mutate_saved_character_at_root` below now advances `envelope.revision_id`
//! on every successful call (via `next_mutation_revision_id`), not just
//! `saved_at`. Before this cycle, only `characterHub::reSaveCharacter`'s own
//! bespoke `next_revision_id` advanced the counter; every mutation routed
//! through this function (`level_up_character`, `add_equipment_selection`,
//! `add_spell_selection`, and `appendToCharacter` via
//! `characterHub::appendToCharacter`, which reuses this same function)
//! silently preserved whatever `revision_id` was already on disk, forever.
//! This is a deliberate, operator-approved behavior change, not a deferred
//! `## DISCOVERED` item.

use std::path::Path;

use codex::rules_core::character_input::{
    AbilityScores, AcquisitionMode, ActiveState, CharacterClassLevel, CharacterInput,
    ChosenCharacterState, EquipmentSelection, SelectedChoice, SkillAllocation, SpellSelection,
};
use codex::rules_core::level_up::{compute_level_up_grants_for_class, LevelUpPlan};
use codex::rules_core::pilot_compute::{
    build_pilot_headless_receipt, compute_pilot_base_chassis, HeadlessReceiptStatus,
    PilotBaseChassisComputation,
};
use codex::rules_core::pilot_compute_corpus::compute_pilot_with_corpus;
use codex::rules_core::pilot_view_model::PilotViewModel;
use codex::saved_character::local_store::SavedCharacterStore;

use crate::character_hub::{
    map_corpus_derived_dto, map_diagnostics_dto, map_snapshot_dto, map_summary_dto,
    summarize_envelope, CreateCharacterRequest, CreateCharacterResponse,
    ListSavedCharactersResponse, LoadSavedCharacterResponse, HUMAN_RACE_ID, SOURCE_PACKAGE_ID,
};
use crate::characterHub::appendToCharacter::{
    append_to_character_at_root, AppendToCharacterResponse, ItemToAppendDto,
};
use crate::characterHub::recomputeCharacter::{recompute_character_at_root, RecomputeCharacterResponse};
use crate::characterHub::reSaveCharacter::{re_save_character_at_root, ReSaveCharacterResponse};
use crate::corpus_fixtures::corpus_fixture_bundle;
use crate::rule_system_adapter::{ClassLevelDelta, RuleSystemAdapter};

/// The Pathfinder 1e `RuleSystemAdapter` implementation. Zero-sized today —
/// every operation below is stateless (it takes the on-disk root / mutation
/// closure it needs as parameters, exactly like this crate's existing
/// `_at_root` free-function convention) — see this module's own top-of-file
/// doc comment for the trait-impl timing note.
///
/// `#[allow(dead_code)]`: not yet constructed by any `#[tauri::command]`
/// call site — criterion 3.4 (Tauri command-surface routes through the
/// hub-of-hubs) wires real callers once criterion 3.1's `RuleSystemAdapter`
/// trait has landed and this type implements it. Exercised today by this
/// module's own `#[cfg(test)]` tests (`pf1_adapter_level_up_delegates_to_the_real_implementation`),
/// which prove `level_up` below is real, wired behavior, not a stub —
/// `cargo check`'s non-test build just cannot see that caller.
#[allow(dead_code)]
pub struct Pf1Adapter;

#[allow(dead_code)]
impl Pf1Adapter {
    /// The `level_up` operation named by `RuleSystemAdapter`'s method
    /// surface (`cycles/3_1.md`). Delegates to `level_up_character_at_root`
    /// below so there is exactly one real implementation of "level up a
    /// saved character's class", not two.
    pub fn level_up(
        &self,
        root: &Path,
        class_id: &str,
        saved_at: &str,
    ) -> Result<CreateCharacterResponse, String> {
        level_up_character_at_root(root, class_id, saved_at)
    }
}

impl RuleSystemAdapter for Pf1Adapter {
    fn rule_system_id(&self) -> &'static str {
        "pf1"
    }

    fn chassis_resolve(&self, input: &CharacterInput) -> PilotBaseChassisComputation {
        compute_pilot_base_chassis(input)
    }

    /// Dispatches each `ClassLevelDelta` through `compute_level_up_grants_for_class`
    /// (this cycle's own register-A2 fix) and merges the resulting plans —
    /// so a multi-delta call (e.g. a Fighter+Wizard mix leveling both sides
    /// in one request) produces the union of every class's real grants,
    /// never the top-level `compute_level_up_grants`'s multiclass-gap empty
    /// default this criterion's carry-forward note describes.
    fn level_up(&self, character: &CharacterInput, deltas: &[ClassLevelDelta]) -> LevelUpPlan {
        let mut plan = LevelUpPlan::default();
        for delta in deltas {
            let sub = compute_level_up_grants_for_class(
                character,
                &delta.class_id,
                delta.from_level,
                delta.to_level,
            );
            plan.automatic_features.extend(sub.automatic_features);
            plan.pick_from_lists.extend(sub.pick_from_lists);
            plan.resource_pool_change.pools.extend(sub.resource_pool_change.pools);
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
        let listing = SavedCharacterStore::list_all(characters_root).map_err(|err| err.message)?;
        Ok(ListSavedCharactersResponse {
            characters: listing.characters.iter().map(map_summary_dto).collect(),
            unreadable_count: listing.unreadable_entries.len(),
        })
    }

    fn load_saved_character(&self, root: &Path) -> Result<LoadSavedCharacterResponse, String> {
        let envelope = SavedCharacterStore::load(root).map_err(|err| err.message)?;

        let receipt = build_pilot_headless_receipt(&envelope.character_input);
        let view_model = PilotViewModel::from_receipt(&receipt);
        let corpus_receipt =
            compute_pilot_with_corpus(&envelope.character_input, corpus_fixture_bundle());

        Ok(LoadSavedCharacterResponse {
            summary: summarize_envelope(&envelope),
            snapshot: view_model.snapshot.as_ref().map(map_snapshot_dto),
            diagnostics: map_diagnostics_dto(&receipt.computation.diagnostics),
            corpus_derived: map_corpus_derived_dto(&corpus_receipt.corpus_derived),
        })
    }
}

// ----- Pure functions (unit-testable, no AppHandle / filesystem) -----

/// Build a `CharacterInput` for the requested race/class/level. Race, class,
/// and ability scores are the caller's real choices; the feat/skill/
/// equipment loadout is fixed — `unmet_combat_posture_conditions`
/// (`pilot_compute.rs`) requires this exact equipment/feat posture verbatim
/// to reach `Computed`, so widening it would not change which combinations
/// reach `Computed`. Human additionally receives its own canonical
/// choice-slot values — the ability-bonus target is the caller's real
/// choice (`request.ability_bonus_target`); every other race omits the
/// Human-only slots. Unlike feats/skills/equipment, `spells_selected` is
/// *not* fixed to any hardcoded placeholder (SD-24 Criterion 7.5): no
/// `Computed`-status gate reads it, `CreateCharacterRequest` collects no
/// spell choices from the caller, and a freshly composed character starts
/// with an empty spellbook that only grows through the real wired
/// `add_spell_selection` / `appendToCharacter` command surface.
///
/// Moved here verbatim from `character_hub.rs` (SD-25 Criterion 3.2
/// extraction) — re-exported at `crate::character_hub::compose_character_input`
/// so `characterHub::appendToCharacter` / `characterHub::reSaveCharacter`'s
/// existing `use crate::character_hub::compose_character_input;` import
/// paths keep resolving unchanged.
pub fn compose_character_input(request: &CreateCharacterRequest) -> CharacterInput {
    let mut selected_choices = vec![
        SelectedChoice {
            choice_set_id: "choice:level_1_character_feat".to_owned(),
            selection_id: "feat:power_attack".to_owned(),
        },
        SelectedChoice {
            choice_set_id: "choice:fighter_bonus_feat".to_owned(),
            selection_id: "feat:weapon_focus:weapon:longsword".to_owned(),
        },
    ];

    if request.race_id == HUMAN_RACE_ID {
        selected_choices.push(SelectedChoice {
            choice_set_id: "choice:human_bonus_feat".to_owned(),
            selection_id: "feat:dodge".to_owned(),
        });
        selected_choices.push(SelectedChoice {
            choice_set_id: "choice:human_ability_bonus".to_owned(),
            selection_id: format!("ability:{}", request.ability_bonus_target),
        });
    }

    CharacterInput {
        case_id: Some(request.character_id.clone()),
        source_package_id: SOURCE_PACKAGE_ID.to_owned(),
        chosen: ChosenCharacterState {
            race_id: request.race_id.clone(),
            class_levels: vec![CharacterClassLevel {
                class_id: request.class_id.clone(),
                level: request.level,
            }],
            ability_scores: AbilityScores {
                strength: request.ability_scores.strength,
                dexterity: request.ability_scores.dexterity,
                constitution: request.ability_scores.constitution,
                intelligence: request.ability_scores.intelligence,
                wisdom: request.ability_scores.wisdom,
                charisma: request.ability_scores.charisma,
            },
            selected_feats: vec![
                "feat:power_attack".to_owned(),
                "feat:dodge".to_owned(),
                "feat:weapon_focus".to_owned(),
            ],
            skill_allocations: vec![
                SkillAllocation {
                    skill_id: "skill:climb".to_owned(),
                    ranks: 1,
                },
                SkillAllocation {
                    skill_id: "skill:intimidate".to_owned(),
                    ranks: 1,
                },
                SkillAllocation {
                    skill_id: "skill:swim".to_owned(),
                    ranks: 1,
                },
            ],
            equipment_selections: vec![
                EquipmentSelection {
                    item_id: "item:longsword".to_owned(),
                    equipped_or_active: true,
                    active_state: ActiveState::EquippedActive,
                },
                EquipmentSelection {
                    item_id: "item:chain_shirt".to_owned(),
                    equipped_or_active: true,
                    active_state: ActiveState::EquippedActive,
                },
                EquipmentSelection {
                    item_id: "item:shield".to_owned(),
                    equipped_or_active: false,
                    active_state: ActiveState::Absent,
                },
                EquipmentSelection {
                    item_id: "power_attack".to_owned(),
                    equipped_or_active: false,
                    active_state: ActiveState::SelectedInactive,
                },
            ],
            selected_choices,
            spells_selected: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

/// Computes the next `{character_id}.rev.N` revision id from the current
/// on-disk one — the same "increment the numeric suffix, restart at 1 for a
/// non-conforming legacy revision_id" logic
/// `characterHub::reSaveCharacter::next_revision_id` already established
/// (that function is private to a file outside this criterion's file-touch
/// grant, so this is a parallel, functionally-identical implementation
/// rather than a shared one — not a behavior fork, the same rule applied in
/// two call sites).
fn next_mutation_revision_id(character_id: &str, current_revision_id: &str) -> String {
    let prefix = format!("{character_id}.rev.");
    let next_n = current_revision_id
        .strip_prefix(prefix.as_str())
        .and_then(|suffix| suffix.parse::<u64>().ok())
        .map(|n| n + 1)
        .unwrap_or(1);
    format!("{prefix}{next_n}")
}

/// Shared load -> recompute -> re-save -> return-envelope tail for every
/// `mutate_saved_character` operation: applies `mutate` to the loaded
/// envelope's `CharacterInput`, recomputes via the real engine, and either
/// re-saves and returns `Saved` or leaves the on-disk envelope untouched and
/// returns `Blocked`. Never persists an unproven build — mirrors
/// `create_character`/`clone_character`'s own invariant. `mutate` receives
/// only the `CharacterInput`, so it cannot smuggle in a different `saved_at`
/// or bypass the recompute gate.
///
/// **SD-25 carry-forward register A5** (operator-confirmed 2026-07-21,
/// `decisions.md §11`): every successful call now also advances
/// `envelope.revision_id` (and `latest_authoritative_revision_ref`) to a
/// freshly minted `{character_id}.rev.N`, not just `saved_at` — folded in
/// as part of this extraction per the operator's explicit approval. Every
/// command routed through this function (`level_up_character`,
/// `add_equipment_selection`, `add_spell_selection`, and `appendToCharacter`
/// via `characterHub::appendToCharacter`, which calls this same function)
/// now advances the counter on every mutating call, not just
/// `reSaveCharacter`.
///
/// `pub(crate)` (rather than private) so `characterHub::appendToCharacter`
/// (SD-24 Epic 7, Criterion 7.1) can reuse this module's own
/// load -> mutate -> recompute -> re-save -> return-envelope invariant
/// instead of re-deriving it a second time — re-exported at
/// `crate::character_hub::mutate_saved_character_at_root` so that file's
/// existing `character_hub::mutate_saved_character_at_root(...)` call site
/// keeps resolving unchanged.
pub(crate) fn mutate_saved_character_at_root(
    root: &Path,
    saved_at: &str,
    mutate: impl FnOnce(&mut CharacterInput),
) -> Result<CreateCharacterResponse, String> {
    let mut envelope = SavedCharacterStore::load(root).map_err(|err| err.message)?;

    mutate(&mut envelope.character_input);

    let receipt = build_pilot_headless_receipt(&envelope.character_input);
    if receipt.status != HeadlessReceiptStatus::Computed {
        return Ok(CreateCharacterResponse::Blocked {
            diagnostics: map_diagnostics_dto(&receipt.computation.diagnostics),
        });
    }

    let view_model = PilotViewModel::from_receipt(&receipt);
    let snapshot = view_model
        .snapshot
        .as_ref()
        .expect("Computed status guarantees a snapshot");

    let corpus_receipt =
        compute_pilot_with_corpus(&envelope.character_input, corpus_fixture_bundle());

    let next_revision_id =
        next_mutation_revision_id(&envelope.character_id, &envelope.revision_id);
    envelope.revision_id = next_revision_id.clone();
    envelope.latest_authoritative_revision_ref = next_revision_id;
    envelope.saved_at = saved_at.to_owned();

    SavedCharacterStore::save(&envelope, root).map_err(|err| err.message)?;

    Ok(CreateCharacterResponse::Saved {
        summary: Box::new(summarize_envelope(&envelope)),
        snapshot: map_snapshot_dto(snapshot),
        corpus_derived: map_corpus_derived_dto(&corpus_receipt.corpus_derived),
    })
}

/// Increments `class_id`'s level by 1 on the given `CharacterInput`, or
/// adds a new level-1 entry for `class_id` if the character has none yet
/// (the multiclass "dip" case). Every other field is untouched.
pub fn apply_level_up(character_input: &mut CharacterInput, class_id: &str) {
    if let Some(class_level) = character_input
        .chosen
        .class_levels
        .iter_mut()
        .find(|class_level| class_level.class_id == class_id)
    {
        class_level.level = class_level.level.saturating_add(1);
    } else {
        character_input.chosen.class_levels.push(CharacterClassLevel {
            class_id: class_id.to_owned(),
            level: 1,
        });
    }
}

/// `level_up_character`'s real implementation: load -> mutate -> recompute
/// -> re-save -> return envelope. Split out from the `#[tauri::command]`
/// wrapper in `character_hub.rs` so it is unit-testable against a real
/// `SavedCharacterStore` fixture without an `AppHandle`.
///
/// Mirrors `create_character`/`clone_character`'s "never persist an
/// unproven build" invariant: if the leveled-up build does not reach
/// `Computed` (e.g. leveling past the range the compute engine currently
/// supports), the saved character on disk is left exactly as it was and
/// `Blocked` is returned with the real diagnostics — the mutation is never
/// silently applied on disk when the recompute fails.
pub(crate) fn level_up_character_at_root(
    root: &Path,
    class_id: &str,
    saved_at: &str,
) -> Result<CreateCharacterResponse, String> {
    mutate_saved_character_at_root(root, saved_at, |character_input| {
        apply_level_up(character_input, class_id);
    })
}

/// Appends one entry to `chosen.equipment_selections`. `equipped_or_active`
/// is derived from `active_state` (matching `EquipmentSelection`'s own doc
/// comment: the flag is a backward-compatible projection of the state, not
/// an independent choice) — true only for `ActiveState::EquippedActive`.
/// Every other field is untouched.
pub fn apply_add_equipment_selection(
    character_input: &mut CharacterInput,
    item_id: &str,
    active_state: ActiveState,
) {
    character_input.chosen.equipment_selections.push(EquipmentSelection {
        item_id: item_id.to_owned(),
        equipped_or_active: active_state == ActiveState::EquippedActive,
        active_state,
    });
}

/// `add_equipment_selection`'s real implementation — see
/// `mutate_saved_character_at_root` for the shared
/// load -> mutate -> recompute -> re-save -> return-envelope semantics.
pub(crate) fn add_equipment_selection_at_root(
    root: &Path,
    item_id: &str,
    active_state: ActiveState,
    saved_at: &str,
) -> Result<CreateCharacterResponse, String> {
    mutate_saved_character_at_root(root, saved_at, |character_input| {
        apply_add_equipment_selection(character_input, item_id, active_state);
    })
}

/// Appends one entry to `chosen.spells_selected`. Every other field is
/// untouched.
pub fn apply_add_spell_selection(
    character_input: &mut CharacterInput,
    spell_id: &str,
    source_class_id: &str,
    acquisition_mode: AcquisitionMode,
) {
    character_input.chosen.spells_selected.push(SpellSelection {
        spell_id: spell_id.to_owned(),
        source_class_id: source_class_id.to_owned(),
        acquisition_mode,
    });
}

/// `add_spell_selection`'s real implementation — see
/// `mutate_saved_character_at_root` for the shared
/// load -> mutate -> recompute -> re-save -> return-envelope semantics.
pub(crate) fn add_spell_selection_at_root(
    root: &Path,
    spell_id: &str,
    source_class_id: &str,
    acquisition_mode: AcquisitionMode,
    saved_at: &str,
) -> Result<CreateCharacterResponse, String> {
    mutate_saved_character_at_root(root, saved_at, |character_input| {
        apply_add_spell_selection(character_input, spell_id, source_class_id, acquisition_mode);
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::character_hub::AbilityScoresDto;
    use codex::saved_character::{
        SavedCharacterEnvelope, SavedCharacterRevisionKind, CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
    };
    use std::path::PathBuf;

    const TEST_SAVED_AT: &str = "2026-07-21T00:00:00Z";
    const GAME_SYSTEM_ID: &str = "pf1";
    const FIGHTER_CLASS_ID: &str = "class:fighter";

    fn tempdir(label: &str) -> PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "codex-pf1-adapter-{label}-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir_all(&path).expect("temp dir should be creatable");
        path
    }

    fn request_for(character_id: &str, level: u8) -> CreateCharacterRequest {
        CreateCharacterRequest {
            character_id: character_id.to_owned(),
            display_label: "Pf1Adapter Test Character".to_owned(),
            race_id: "race:human".to_owned(),
            class_id: FIGHTER_CLASS_ID.to_owned(),
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
            saved_at: TEST_SAVED_AT.to_owned(),
        }
    }

    fn seed_envelope(character_id: &str, level: u8) -> SavedCharacterEnvelope {
        let character_input = compose_character_input(&request_for(character_id, level));
        SavedCharacterEnvelope {
            character_id: character_id.to_owned(),
            revision_id: format!("{character_id}.rev.1"),
            revision_kind: SavedCharacterRevisionKind::Authoritative,
            saved_at: TEST_SAVED_AT.to_owned(),
            schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
            app_or_runtime_version: "codex-dev".to_owned(),
            content_or_rules_provenance: SOURCE_PACKAGE_ID.to_owned(),
            game_system: GAME_SYSTEM_ID.to_owned(),
            latest_authoritative_revision_ref: format!("{character_id}.rev.1"),
            display_label: "Pf1Adapter Test Character".to_owned(),
            character_input,
        }
    }

    /// SD-25 carry-forward register A5's RED -> GREEN proof: before this
    /// cycle, `mutate_saved_character_at_root` preserved whatever
    /// `revision_id` was already on disk on every mutating call. Now every
    /// command routed through it — proven here directly for
    /// `level_up_character`, `add_equipment_selection`, and
    /// `add_spell_selection` (the three that live in this crate's own
    /// file-touch grant; `appendToCharacter` shares this exact function so
    /// it advances too, verified indirectly — its own existing tests in
    /// `characterHub::appendToCharacter`, outside this cycle's grant,
    /// continue to pass unchanged) — advances the counter.
    #[test]
    fn level_up_character_at_root_advances_revision_id() {
        let character_id = "pf1-adapter-revision-level-up";
        let root = tempdir("revision-level-up");
        let envelope = seed_envelope(character_id, 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = level_up_character_at_root(&root, FIGHTER_CLASS_ID, "2026-07-21T01:00:00Z")
            .expect("level up call should not error");

        match response {
            CreateCharacterResponse::Saved { .. } => {}
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!("Human Fighter level 1 -> 2 must reach Computed, got: {diagnostics:?}")
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.revision_id,
            format!("{character_id}.rev.2"),
            "level_up_character must advance revision_id, not just saved_at"
        );
        assert_eq!(
            reloaded.latest_authoritative_revision_ref,
            format!("{character_id}.rev.2")
        );

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn add_equipment_selection_at_root_advances_revision_id() {
        let character_id = "pf1-adapter-revision-equipment";
        let root = tempdir("revision-equipment");
        let envelope = seed_envelope(character_id, 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = add_equipment_selection_at_root(
            &root,
            "item:dagger",
            ActiveState::EquippedActive,
            "2026-07-21T01:00:00Z",
        )
        .expect("add-equipment call should not error");

        match response {
            CreateCharacterResponse::Saved { .. } => {}
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!("adding equipment must reach Computed, got: {diagnostics:?}")
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.revision_id,
            format!("{character_id}.rev.2"),
            "add_equipment_selection must advance revision_id, not just saved_at"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn add_spell_selection_at_root_advances_revision_id() {
        let character_id = "pf1-adapter-revision-spell";
        let root = tempdir("revision-spell");
        let envelope = seed_envelope(character_id, 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = add_spell_selection_at_root(
            &root,
            "Mage Armor",
            "class:wizard",
            AcquisitionMode::Known,
            "2026-07-21T01:00:00Z",
        )
        .expect("add-spell call should not error");

        match response {
            CreateCharacterResponse::Saved { .. } => {}
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!("adding a spell must reach Computed, got: {diagnostics:?}")
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.revision_id,
            format!("{character_id}.rev.2"),
            "add_spell_selection must advance revision_id, not just saved_at"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// A second consecutive mutation keeps advancing (`.rev.2` -> `.rev.3`),
    /// proving the counter is real, not a second hardcoded constant —
    /// mirrors `reSaveCharacter`'s own equivalent proof test.
    #[test]
    fn mutate_saved_character_at_root_keeps_advancing_across_repeated_calls() {
        let character_id = "pf1-adapter-revision-repeated";
        let root = tempdir("revision-repeated");
        let envelope = seed_envelope(character_id, 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        level_up_character_at_root(&root, FIGHTER_CLASS_ID, "2026-07-21T01:00:00Z")
            .expect("first level up should not error");
        let after_first = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(after_first.revision_id, format!("{character_id}.rev.2"));

        add_equipment_selection_at_root(
            &root,
            "item:dagger",
            ActiveState::EquippedActive,
            "2026-07-21T02:00:00Z",
        )
        .expect("add-equipment should not error");
        let after_second = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(after_second.revision_id, format!("{character_id}.rev.3"));

        std::fs::remove_dir_all(&root).ok();
    }

    /// `Pf1Adapter::level_up` (the trait-surface-named inherent method) must
    /// delegate to the exact same real implementation, not a second
    /// parallel one.
    #[test]
    fn pf1_adapter_level_up_delegates_to_the_real_implementation() {
        let character_id = "pf1-adapter-struct-level-up";
        let root = tempdir("struct-level-up");
        let envelope = seed_envelope(character_id, 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let adapter = Pf1Adapter;
        let response = adapter
            .level_up(&root, FIGHTER_CLASS_ID, "2026-07-21T01:00:00Z")
            .expect("level up call should not error");

        match response {
            CreateCharacterResponse::Saved { summary, .. } => {
                assert_eq!(summary.class_summary, "class:fighter:2");
            }
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!("Human Fighter level 1 -> 2 must reach Computed, got: {diagnostics:?}")
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(reloaded.revision_id, format!("{character_id}.rev.2"));

        std::fs::remove_dir_all(&root).ok();
    }

    /// `impl RuleSystemAdapter for Pf1Adapter` — proves `Pf1Adapter` genuinely
    /// implements the full seven-method trait surface (criterion 3.1's
    /// `rule_system_adapter.rs`) with real, wired behavior end to end, the
    /// same way `rule_system_adapter.rs`'s own `TestPf1Delegate` test proved
    /// the trait was *implementable* — this proves the *real* extracted
    /// `Pf1Adapter` type is the implementation, not a parallel test-only one.
    #[test]
    fn pf1_adapter_implements_rule_system_adapter_end_to_end() {
        let adapter: Box<dyn RuleSystemAdapter> = Box::new(Pf1Adapter);
        assert_eq!(adapter.rule_system_id(), "pf1");

        let character_id = "pf1-adapter-trait-impl";
        let characters_root = tempdir("trait-impl-root");
        let root = characters_root.join(character_id);
        let envelope = seed_envelope(character_id, 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        // chassis_resolve: real compute over a real CharacterInput.
        let chassis = adapter.chassis_resolve(&envelope.character_input);
        assert_eq!(chassis.base_attack_bonus, 1, "human fighter level 1 chassis");

        // level_up: register A2's multiclass-safe dispatch, exercised via a
        // single-delta call here (the multiclass case is proven directly in
        // `src/rules_core/level_up.rs`'s own tests).
        let deltas = [ClassLevelDelta {
            class_id: FIGHTER_CLASS_ID.to_owned(),
            from_level: 1,
            to_level: 2,
        }];
        let plan = adapter.level_up(&envelope.character_input, &deltas);
        assert!(
            !plan.automatic_features.is_empty(),
            "Fighter 1 -> 2 should grant at least one automatic feature"
        );

        // list_saved_characters / load_saved_character: real disk I/O,
        // reusing `character_hub`'s own mapping instead of re-deriving it.
        let listing = adapter
            .list_saved_characters(&characters_root)
            .expect("list_saved_characters should succeed");
        assert!(listing.characters.iter().any(|c| c.character_id == character_id));

        let loaded = adapter
            .load_saved_character(&root)
            .expect("load_saved_character should succeed");
        assert_eq!(loaded.summary.character_id, character_id);
        assert!(loaded.snapshot.is_some());

        // append_to_character: real corpus-validated batch append.
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
        assert!(append_result.success, "appending a real item should succeed: {:?}", append_result.error);

        // recompute: real read-and-recompute, no mutation.
        let recomputed = adapter.recompute(&root, character_id);
        assert!(recomputed.success, "recompute should succeed: {:?}", recomputed.error);

        // save_character: real re-save via the revision-conflict-checked path.
        let reloaded_after_append =
            SavedCharacterStore::load(&root).expect("reload after append should succeed");
        let saved = adapter
            .save_character(&root, &reloaded_after_append.revision_id, "2026-07-21T02:00:00Z")
            .expect("save_character should not error");
        assert!(saved.success, "save_character should succeed: {:?}", saved.error);

        let conflict = adapter
            .save_character(&root, "stale-revision-not-on-disk", "2026-07-21T03:00:00Z")
            .expect("save_character should not error even on a conflict rejection");
        assert!(!conflict.success);
        assert_eq!(conflict.error.as_deref(), Some("revision_conflict"));

        std::fs::remove_dir_all(&characters_root).ok();
    }
}
