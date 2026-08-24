//! `StubAdapter` — the future-rule-system placeholder Criterion 3.3 defines
//! (`epic-breakdown.md` §3.3, cycle doc `cycles/3_3.md`, Epic 3 "Character
//! Hub as Hub of Hubs").
//!
//! `rule_system_adapter.rs` (criterion 3.1) defines `RuleSystemAdapter` as
//! the seam every rule system implements once. Today only Pf1 has a real
//! implementation (`pf1_adapter.rs`, criterion 3.2). Criterion 3.4's Tauri
//! command surface still needs a `dyn RuleSystemAdapter` to hand back for a
//! `rule_system_id` that names a rule system this codebase has not built
//! yet (e.g. an operator-pinned future Starfinder/5e rollout) rather than
//! panicking or refusing to compile a code path for that id at all.
//! `StubAdapter` is that placeholder: constructed with the caller-chosen
//! system id, every method honestly reports "not yet implemented" for that
//! id instead of fabricating a result.
//!
//! Wired-integration doctrine forbids "Would ..." strings in shipping code.
//! This stub is the doctrine's own named exception: it ships together with
//! `docs/governance/wired-integration-stubs-registry.md` entry 0002 in the same
//! commit, carrying the operator-granted justification. Without that
//! registry entry landing in the same commit, the §6 dual-audit failure for
//! this file is not self-healable (`cycles/3_3.md` GREEN).

use std::path::Path;

use codex::rules_core::character_input::CharacterInput;
use codex::rules_core::level_up::LevelUpPlan;
use codex::rules_core::pilot_compute::{
    AbilityModifiers, BaseSaves, ComputationDiagnostic, PilotBaseChassisComputation,
    SelectedSkillModifiers,
};

use crate::character_hub::{ListSavedCharactersResponse, LoadSavedCharacterResponse};
use crate::characterHub::appendToCharacter::{AppendToCharacterResponse, ItemToAppendDto};
use crate::characterHub::recomputeCharacter::RecomputeCharacterResponse;
use crate::characterHub::reSaveCharacter::ReSaveCharacterResponse;
use crate::rule_system_adapter::{ClassLevelDelta, RuleSystemAdapter};

/// Placeholder `RuleSystemAdapter` for a rule system this codebase has not
/// implemented yet. Holds a `&'static str` system id (mirroring
/// `RuleSystemAdapter::rule_system_id`'s own `&'static str` return type) so
/// callers register one `StubAdapter` per not-yet-built system with a
/// literal id, the same way `Pf1Adapter` will register itself with `"pf1"`.
pub struct StubAdapter {
    system_id: &'static str,
}

impl StubAdapter {
    pub fn new(system_id: &'static str) -> Self {
        Self { system_id }
    }

    /// The single message every method below reports through whatever
    /// channel its own return type honestly provides (a diagnostic, an
    /// `error` field, or an `Err`) — never fabricated data.
    fn would_render_message(&self) -> String {
        format!(
            "Would render for system {}; not yet implemented",
            self.system_id
        )
    }
}

impl RuleSystemAdapter for StubAdapter {
    fn rule_system_id(&self) -> &'static str {
        self.system_id
    }

    /// No compute engine exists for this system yet, so every numeric
    /// field stays at its honest zero/default value (mirroring
    /// `compute_pilot_base_chassis`'s own "unsupported chassis" arm) with
    /// a claim-blocking diagnostic carrying the stub message, rather than
    /// fabricating a chassis.
    fn chassis_resolve(&self, _input: &CharacterInput) -> PilotBaseChassisComputation {
        PilotBaseChassisComputation {
            ability_modifiers: AbilityModifiers::default(),
            base_attack_bonus: 0,
            base_saves: BaseSaves::default(),
            baseline_melee_attack_bonus: 0,
            baseline_armor_class: 0,
            total_saves: BaseSaves::default(),
            selected_skill_modifiers: SelectedSkillModifiers::default(),
            explanations: Vec::new(),
            diagnostics: vec![ComputationDiagnostic {
                id: "stub_adapter.not_yet_implemented".to_owned(),
                message: self.would_render_message(),
                claim_blocking: true,
            }],
        }
    }

    /// No level-up tables exist for this system yet; `LevelUpPlan`
    /// carries no message field of its own (per its "compose, don't
    /// fabricate" doc comment), so the honest report is the same empty
    /// default `level_up::compute_level_up_grants` itself returns for an
    /// unrecognized dispatch, not a fabricated grant list.
    fn level_up(&self, _character: &CharacterInput, _deltas: &[ClassLevelDelta]) -> LevelUpPlan {
        LevelUpPlan::default()
    }

    fn save_character(
        &self,
        _root: &Path,
        _expected_revision_id: &str,
        _saved_at: &str,
    ) -> Result<ReSaveCharacterResponse, String> {
        Err(self.would_render_message())
    }

    fn append_to_character(
        &self,
        _root: &Path,
        _items_to_append: &[ItemToAppendDto],
        _saved_at: &str,
    ) -> Result<AppendToCharacterResponse, String> {
        Err(self.would_render_message())
    }

    fn recompute(&self, _root: &Path, _character_id: &str) -> RecomputeCharacterResponse {
        RecomputeCharacterResponse {
            success: false,
            character: None,
            error: Some(self.would_render_message()),
        }
    }

    fn list_saved_characters(
        &self,
        _characters_root: &Path,
    ) -> Result<ListSavedCharactersResponse, String> {
        Err(self.would_render_message())
    }

    fn load_saved_character(&self, _root: &Path) -> Result<LoadSavedCharacterResponse, String> {
        Err(self.would_render_message())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use codex::rules_core::character_input::CharacterInput;

    use crate::character_hub::{
        compose_character_input, AbilityScoresDto, CreateCharacterRequest,
    };

    /// Builds a real `CharacterInput` via the crate's own composition
    /// entrypoint (the same one `rule_system_adapter.rs`'s trait test
    /// uses) — a stub test still exercises the trait's method signatures
    /// against a genuinely-constructed input, never a fabricated one.
    fn stub_character_input() -> CharacterInput {
        compose_character_input(&CreateCharacterRequest {
            character_id: "stub-adapter-test-char".to_owned(),
            display_label: "Stub Adapter Test Character".to_owned(),
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
            saved_at: "2026-07-21T00:00:00Z".to_owned(),
        })
    }

    /// RED (per `cycles/3_3.md`): before this file existed, a test
    /// resolving a non-Pf1 `rule_system_id` (e.g. `"starfinder"`) to a
    /// `Box<dyn RuleSystemAdapter>` had nothing to construct — there was no
    /// `StubAdapter` type in the crate, so `cargo check --tests` failed
    /// with `error[E0433]: failed to resolve: could not find 'StubAdapter'
    /// in 'stub_adapter'` (the module itself did not exist either). GREEN
    /// (this file, present tense): `StubAdapter` implements the full
    /// `RuleSystemAdapter` surface and every method below is exercised
    /// through a trait object built for a non-Pf1 id.
    #[test]
    fn non_pf1_rule_system_id_resolves_to_a_stub_adapter() {
        let adapter: Box<dyn RuleSystemAdapter> = Box::new(StubAdapter::new("starfinder"));
        assert_eq!(adapter.rule_system_id(), "starfinder");

        let input = stub_character_input();

        let chassis = adapter.chassis_resolve(&input);
        assert_eq!(chassis.base_attack_bonus, 0);
        assert_eq!(chassis.diagnostics.len(), 1);
        assert!(chassis.diagnostics[0].claim_blocking);
        assert_eq!(
            chassis.diagnostics[0].message,
            "Would render for system starfinder; not yet implemented"
        );

        let plan = adapter.level_up(&input, &[]);
        assert!(plan.automatic_features.is_empty());

        let save_err = adapter
            .save_character(Path::new("/nonexistent"), "rev.1", "2026-07-21T00:00:00Z")
            .expect_err("save_character should honestly error for an unimplemented system");
        assert_eq!(
            save_err,
            "Would render for system starfinder; not yet implemented"
        );

        let append_err = adapter
            .append_to_character(Path::new("/nonexistent"), &[], "2026-07-21T00:00:00Z")
            .expect_err("append_to_character should honestly error for an unimplemented system");
        assert_eq!(
            append_err,
            "Would render for system starfinder; not yet implemented"
        );

        let recomputed = adapter.recompute(Path::new("/nonexistent"), "some-character");
        assert!(!recomputed.success);
        assert_eq!(
            recomputed.error.as_deref(),
            Some("Would render for system starfinder; not yet implemented")
        );

        let list_err = adapter
            .list_saved_characters(Path::new("/nonexistent"))
            .expect_err("list_saved_characters should honestly error for an unimplemented system");
        assert_eq!(
            list_err,
            "Would render for system starfinder; not yet implemented"
        );

        let load_err = adapter
            .load_saved_character(Path::new("/nonexistent"))
            .expect_err("load_saved_character should honestly error for an unimplemented system");
        assert_eq!(
            load_err,
            "Would render for system starfinder; not yet implemented"
        );
    }

    /// A second non-Pf1 id proves the message is genuinely id-specific,
    /// not hardcoded to one literal.
    #[test]
    fn stub_adapter_message_names_its_own_system_id() {
        let adapter = StubAdapter::new("fifth-edition");
        let err = adapter
            .save_character(Path::new("/nonexistent"), "rev.1", "2026-07-21T00:00:00Z")
            .expect_err("save_character should error");
        assert_eq!(err, "Would render for system fifth-edition; not yet implemented");
    }
}
