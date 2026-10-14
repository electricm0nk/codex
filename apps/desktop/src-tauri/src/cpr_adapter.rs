//! `CyberpunkAdapter` — the `RuleSystemAdapter` for Cyberpunk RED (`GameSystem::CyberpunkRed`,
//! wire id `cyberpunk-red`).
//!
//! A CPR character lives in its own store (`codex::rules_core::cpr::store`): a directory under
//! the shared characters root holding `envelope.txt` (`game_system = cyberpunk-red`) and
//! `cpr_character_input.txt`, plus `cpr_sheet_choices.txt` once any choice is saved. This adapter
//! lists, loads and re-saves those characters; a loaded character's numbers are the rows of its
//! composed sheet (`codex::rules_core::cpr::sheet::compose_sheet` over the CPR corpus), which
//! the sheet renders verbatim, and its dice and rule lines are the sheet's printed lines.
//!
//! The trait is Pathfinder-typed. Its two methods that take a Pathfinder `CharacterInput`
//! (`chassis_resolve`, `level_up`) and the two that answer with a Pathfinder chassis or
//! equipment (`recompute`, `append_to_character`) cannot carry a CPR character: they answer
//! with the empty plan or a refusal naming the reason, never Pathfinder numbers. CPR's own
//! actions (creation, improvement) are their own commands (`cpr_creation`, `cpr_improve`).

use std::path::Path;

use codex::rules_core::character_input::CharacterInput;
use codex::rules_core::cpr::sheet::{compose_sheet, CprPrintedForm, CprSheetCatalogs};
use codex::rules_core::cpr::store::{CprCharacterStore, CprSavedCharacter};
use codex::rules_core::game_system::{runtime_repo_root, GameSystem};
use codex::rules_core::level_up::LevelUpPlan;
use codex::rules_core::pilot_compute::{
    AbilityModifiers, BaseSaves, ComputationDiagnostic, PilotBaseChassisComputation, SelectedSkillModifiers,
};

use crate::character_hub::{
    AbilityScoresDto, CharacterSummaryDto, CorpusDerivedDto, EncumbranceDto, EquipmentEffectsDto, ExplanationDto,
    ListSavedCharactersResponse, LoadSavedCharacterResponse, SheetLineDto,
};
use crate::characterHub::appendToCharacter::{AppendToCharacterResponse, ItemToAppendDto};
use crate::characterHub::recomputeCharacter::RecomputeCharacterResponse;
use crate::characterHub::reSaveCharacter::ReSaveCharacterResponse;
use crate::rule_system_adapter::{ClassLevelDelta, RuleSystemAdapter};

/// Diagnostic id and error prefix: a Pathfinder chassis cannot describe a CPR character.
pub(crate) const NOT_A_PATHFINDER_CHASSIS: &str = "cpr_adapter.not_a_pathfinder_chassis";
/// Error prefix: the Pathfinder equipment append does not apply to a CPR character.
pub(crate) const NO_PATHFINDER_EQUIPMENT: &str = "cpr_adapter.no_pathfinder_equipment";

/// The adapter `rule_system_adapter::resolve_rule_system_adapter` returns for `cyberpunk-red`.
pub struct CyberpunkAdapter;

fn summary_dto(character: &CprSavedCharacter) -> CharacterSummaryDto {
    CharacterSummaryDto {
        character_id: character.character_id.clone(),
        display_label: character.display_label.clone(),
        game_system: GameSystem::CyberpunkRed.id().to_owned(),
        schema_version: character.schema_version,
        saved_at: character.saved_at.clone(),
        // A CPR character has no Pathfinder race or class levels.
        race_id: String::new(),
        class_summary: String::new(),
    }
}

impl RuleSystemAdapter for CyberpunkAdapter {
    fn rule_system_id(&self) -> &'static str {
        GameSystem::CyberpunkRed.id()
    }

    /// A Pathfinder `CharacterInput` holds no Cyberpunk RED character, so there is no chassis
    /// to compute from it: every number stays zero and one claim-blocking diagnostic says why.
    /// A CPR character's numbers are the `cpr.` rows `load_saved_character` returns.
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
                id: NOT_A_PATHFINDER_CHASSIS.to_owned(),
                message: "A Cyberpunk RED character has no Pathfinder chassis; its numbers are the cpr. rows of its loaded sheet".to_owned(),
                claim_blocking: true,
            }],
            sheet_lines: Vec::new(),
        }
    }

    /// `LevelUpPlan` grants cite Pathfinder table cells; Cyberpunk RED has no levels (a
    /// character improves by spending Improvement Points, `cpr_improve`), so the plan is empty.
    fn level_up(&self, _character: &CharacterInput, _deltas: &[ClassLevelDelta]) -> LevelUpPlan {
        LevelUpPlan::default()
    }

    /// Re-saves the CPR character at `root` under the next revision, refusing a stale
    /// `expected_revision_id` with `revision_conflict`.
    fn save_character(
        &self,
        root: &Path,
        expected_revision_id: &str,
        saved_at: &str,
    ) -> Result<ReSaveCharacterResponse, String> {
        let mut character = CprCharacterStore::load(root)?;
        if character.revision_id != expected_revision_id {
            return Ok(ReSaveCharacterResponse {
                success: false,
                revision_id: None,
                error: Some("revision_conflict".to_owned()),
            });
        }
        let next = crate::sf_adapter::next_revision_id(&character.character_id, &character.revision_id);
        character.revision_id = next.clone();
        character.saved_at = saved_at.to_owned();
        CprCharacterStore::save(&character, root)?;
        Ok(ReSaveCharacterResponse { success: true, revision_id: Some(next), error: None })
    }

    /// The append command adds Pathfinder/Starfinder equipment records; a CPR character's kit
    /// is Cyberpunk RED gear, so nothing is appended and the reason is returned.
    fn append_to_character(
        &self,
        root: &Path,
        items_to_append: &[ItemToAppendDto],
        _saved_at: &str,
    ) -> Result<AppendToCharacterResponse, String> {
        CprCharacterStore::load(root)?;
        let ids: Vec<&str> = items_to_append.iter().map(|i| i.item_id.as_str()).collect();
        Ok(AppendToCharacterResponse {
            success: false,
            character: None,
            error: Some(format!(
                "{NO_PATHFINDER_EQUIPMENT}: a Cyberpunk RED character's kit is Cyberpunk RED gear, not the equipment records [{}]",
                ids.join(", ")
            )),
        })
    }

    /// The recompute response carries a Pathfinder chassis snapshot (BAB, saves, AC), which a
    /// CPR character does not have; the character's numbers are recomputed fresh on every
    /// `load_saved_character`. A missing character is `character_not_found`.
    fn recompute(&self, root: &Path, _character_id: &str) -> RecomputeCharacterResponse {
        let error = match CprCharacterStore::load(root) {
            Ok(_) => format!(
                "{NOT_A_PATHFINDER_CHASSIS}: a Cyberpunk RED character has no Pathfinder chassis snapshot; load_saved_character recomputes its cpr. rows"
            ),
            Err(_) => "character_not_found".to_owned(),
        };
        RecomputeCharacterResponse { success: false, character: None, error: Some(error) }
    }

    /// Every Cyberpunk RED character under `characters_root` (`CprCharacterStore::list_all`).
    fn list_saved_characters(&self, characters_root: &Path) -> Result<ListSavedCharactersResponse, String> {
        let listing = CprCharacterStore::list_all(characters_root)?;
        Ok(ListSavedCharactersResponse {
            characters: listing.characters.iter().map(summary_dto).collect(),
            unreadable_count: listing.unreadable_entries.len(),
        })
    }

    /// The CPR character at `root`, its sheet composed fresh from its STATs and saved sheet
    /// choices over the CPR corpus: `explanations` are the sheet's `cpr.` rows in order and
    /// `sheet_lines` its printed dice and rule lines (`kind` `cpr`). A saved choice the engine
    /// refuses fails the load with the engine's reason; no partial sheet is returned. The
    /// Pathfinder-only blocks of the response (snapshot, corpus-derived equipment, feats,
    /// spells, race traits, the six PF ability scores) are empty or zero; when the sheet prints
    /// no line, `sheet_rules_unavailable_reason` says why: Cyberpunk RED has no sheet-rule
    /// package.
    fn load_saved_character(&self, root: &Path) -> Result<LoadSavedCharacterResponse, String> {
        let character = CprCharacterStore::load(root)?;
        let choices = CprCharacterStore::load_choices(root)?;
        let corpus = GameSystem::CyberpunkRed.package_roots(&runtime_repo_root()).corpus;
        let sheet = compose_sheet(&CprSheetCatalogs::load(&corpus)?, &character.input, &choices)?;
        let sheet_lines: Vec<SheetLineDto> = sheet
            .printed
            .into_iter()
            .map(|line| SheetLineDto {
                id: line.id,
                kind: "cpr".to_owned(),
                label: line.label,
                form: match line.form {
                    CprPrintedForm::Dice => "dice",
                    CprPrintedForm::Words => "words",
                }
                .to_owned(),
                value: line.value,
                also: Vec::new(),
                prose: line.prose,
                condition: None,
            })
            .collect();
        let sheet_rules_unavailable_reason = sheet_lines.is_empty().then(|| {
            "Cyberpunk RED has no sheet-rule package; its sheet renders the cpr. explanation rows".to_owned()
        });
        Ok(LoadSavedCharacterResponse {
            summary: summary_dto(&character),
            snapshot: None,
            diagnostics: Vec::new(),
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
                encumbrance: EncumbranceDto {
                    total_carried_weight_lbs: 0.0,
                    total_carried_cost_gp: 0.0,
                    light_max_lbs: 0.0,
                    medium_max_lbs: 0.0,
                    heavy_max_lbs: 0.0,
                    level: String::new(),
                    load_max_dex_cap: None,
                    load_armor_check_penalty: 0,
                    per_item: Vec::new(),
                    unresolved_item_ids: Vec::new(),
                },
                unresolved_spell_ids: Vec::new(),
                unresolved_equipment_item_ids: Vec::new(),
            },
            selected_feats: Vec::new(),
            spells_selected: Vec::new(),
            chosen_feat_targets: Vec::new(),
            explanations: sheet
                .rows
                .into_iter()
                .map(|e| ExplanationDto { id: e.id, value: e.value, detail: e.detail })
                .collect(),
            weapon_damage: Vec::new(),
            selected_alternate_trait_keys: Vec::new(),
            selected_traits: Vec::new(),
            resolved_racial_traits: crate::race_trait_picker::RaceSelectionResponse {
                race_id: String::new(),
                race_key: String::new(),
                race_name: String::new(),
                book: String::new(),
                applied_traits: Vec::new(),
                suppressions: Vec::new(),
                fired_flags: Vec::new(),
                inert_flags: Vec::new(),
                unmatched_selections: Vec::new(),
                blocked_alternates: Vec::new(),
                conflicting_selections: Vec::new(),
                rendered_trait_descriptions: Vec::new(),
                display_value_feats: Vec::new(),
                errors: vec!["a Cyberpunk RED character has no Pathfinder race".to_owned()],
            },
            ability_scores: AbilityScoresDto {
                strength: 0,
                dexterity: 0,
                constitution: 0,
                intelligence: 0,
                wisdom: 0,
                charisma: 0,
            },
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            sheet_lines,
            sheet_rules_unavailable_reason,
            feat_skill_bonuses: Default::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex::rules_core::cpr::input::{CprCharacterInput, CprStat, CprStats};
    use codex::rules_core::cpr::store::{CprCharacterStore, CprSavedCharacter, CPR_SAVED_CHARACTER_SCHEMA_VERSION};
    use crate::rule_system_adapter::{load_saved_character_via_envelope, resolve_rule_system_adapter};
    use crate::rule_system_adapter::tests::is_would_message;

    fn tempdir(label: &str) -> std::path::PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("codex-cpr-adapter-{label}-{}-{unique}", std::process::id()));
        std::fs::create_dir_all(&path).expect("temp dir should be creatable");
        path
    }

    /// A Cyberpunk RED character with ten distinct STATs, so a row read from the wrong STAT
    /// cannot pass.
    fn edgerunner() -> CprSavedCharacter {
        let mut stats = CprStats::default();
        for (stat, value) in CprStat::ALL.into_iter().zip([7, 8, 6, 5, 4, 3, 2, 6, 5, 8]) {
            stats.set(stat, value);
        }
        CprSavedCharacter {
            character_id: "cpr-edgerunner".to_owned(),
            revision_id: "cpr-edgerunner.rev.1".to_owned(),
            saved_at: "2026-10-13T00:00:00Z".to_owned(),
            schema_version: CPR_SAVED_CHARACTER_SCHEMA_VERSION,
            app_or_runtime_version: "codex-dev".to_owned(),
            display_label: "Adapter Edgerunner".to_owned(),
            input: CprCharacterInput { stats },
        }
    }

    /// Save -> list (1 CPR character; the Pathfinder and Starfinder listings 0 characters
    /// and 0 unreadable) -> load through the envelope router -> the ten `cpr.stat.*` rows
    /// equal the saved STATs.
    #[test]
    fn a_cyberpunk_red_character_saves_lists_and_loads_with_its_ten_stat_rows() {
        let characters_root = tempdir("round-trip");
        let root = characters_root.join("cpr-edgerunner");
        let character = edgerunner();
        CprCharacterStore::save(&character, &root).expect("saves");

        let cpr = resolve_rule_system_adapter("cyberpunk-red").list_saved_characters(&characters_root).expect("lists");
        assert_eq!(cpr.unreadable_count, 0);
        assert_eq!(cpr.characters.len(), 1);
        assert_eq!(cpr.characters[0].character_id, "cpr-edgerunner");
        assert_eq!(cpr.characters[0].game_system, "cyberpunk-red");
        assert_eq!(cpr.characters[0].display_label, "Adapter Edgerunner");
        for other in ["pf1", "starfinder-1e"] {
            let listed = resolve_rule_system_adapter(other).list_saved_characters(&characters_root).expect("lists");
            assert_eq!((listed.characters.len(), listed.unreadable_count), (0, 0), "{other}");
        }

        let loaded = load_saved_character_via_envelope(&root).expect("loads");
        assert_eq!(loaded.summary.character_id, "cpr-edgerunner");
        assert_eq!(loaded.summary.game_system, "cyberpunk-red");
        let rows: Vec<(String, i16)> = loaded
            .explanations
            .iter()
            .filter(|e| e.id.starts_with("cpr.stat."))
            .map(|e| (e.id.clone(), e.value))
            .collect();
        let expected: Vec<(String, i16)> = CprStat::ALL
            .into_iter()
            .map(|s| (format!("cpr.stat.{}", s.id()), i16::from(character.input.stats.get(s))))
            .collect();
        assert_eq!(rows, expected);
        assert!(loaded.diagnostics.iter().all(|d| !d.claim_blocking && !is_would_message(&d.message)));

        std::fs::remove_dir_all(&characters_root).ok();
    }

    /// A character carrying a Role, skills, armour, cyberware and a weapon (its saved sheet
    /// choices) loads with the composed sheet: its explanations are exactly `compose_sheet`'s
    /// rows, in order (no row added or dropped on the way), the cyberware BODY reaches HP, the
    /// armour penalty reaches a REF skill, and the weapon's damage dice reach the sheet as a
    /// printed dice line.
    #[test]
    fn a_saved_character_loads_with_every_row_of_its_composed_sheet() {
        use codex::rules_core::cpr::armor::{CprArmorLocation, CprWornArmor};
        use codex::rules_core::cpr::cyberware::CprCyberwareInstall;
        use codex::rules_core::cpr::input::CprSheetChoices;
        use codex::rules_core::cpr::roles::CprRole;
        use codex::rules_core::cpr::sheet::{compose_sheet, CprSheetCatalogs};

        let characters_root = tempdir("composed");
        let root = characters_root.join("cpr-edgerunner");
        let character = edgerunner();
        let choices = CprSheetChoices {
            role: Some((CprRole::Solo, 4)),
            skills: vec![("Handgun".to_owned(), 4)],
            armor: vec![CprWornArmor { name: "Flak".to_owned(), location: CprArmorLocation::Body }],
            cyberware: vec![CprCyberwareInstall { name: "Grafted Muscle and Bone Lace".to_owned(), mount: None }],
            weapons: vec!["Heavy Pistol".to_owned()],
            ..CprSheetChoices::default()
        };
        CprCharacterStore::save(&character, &root).expect("saves");
        CprCharacterStore::save_choices(&choices, &root).expect("saves the choices");

        let loaded = load_saved_character_via_envelope(&root).expect("loads");
        let corpus = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join(GameSystem::CyberpunkRed.corpus_relative());
        let sheet = compose_sheet(&CprSheetCatalogs::load(&corpus).expect("corpus"), &character.input, &choices)
            .expect("composes");
        let got: Vec<(String, i16)> = loaded.explanations.iter().map(|e| (e.id.clone(), e.value)).collect();
        let want: Vec<(String, i16)> = sheet.rows.iter().map(|r| (r.id.clone(), r.value)).collect();
        assert_eq!(got, want);
        let value = |id: &str| loaded.explanations.iter().find(|e| e.id == id).unwrap_or_else(|| panic!("no {id}")).value;
        // BODY 5 + 2 = 7, WILL 3: HP 10 + 5 x ceil(10 / 2) = 35; REF 8 - 4 = 4, Handgun 4 + 4.
        assert_eq!((value("cpr.cyberware.body"), value("cpr.derived.hit_points")), (7, 35));
        assert_eq!((value("cpr.armor.ref"), value("cpr.skill.handgun")), (4, 8));
        let damage = loaded.sheet_lines.iter().find(|l| l.id == "cpr.weapon.0.damage").expect("the damage line");
        assert_eq!((damage.form.as_str(), damage.value.as_str()), ("dice", "3d6"));

        // A re-save keeps the choices: the reloaded sheet is unchanged.
        CyberpunkAdapter.save_character(&root, "cpr-edgerunner.rev.1", "2026-10-13T01:00:00Z").expect("re-saves");
        let reloaded = load_saved_character_via_envelope(&root).expect("reloads");
        let again: Vec<(String, i16)> = reloaded.explanations.iter().map(|e| (e.id.clone(), e.value)).collect();
        assert_eq!(again, want);

        std::fs::remove_dir_all(&characters_root).ok();
    }

    /// The characters whose real `load_saved_character` responses the frontend Cyberpunk RED
    /// sheet test (`cyberpunkSheetModel.test.ts`) renders: a bare Solo Streetrat (template row 1, no
    /// sheet choices) and a kitted Solo whose cyberware lowers EMP, whose armour lowers REF, DEX
    /// and MOVE, and whose two Critical Injuries (one that raises it) move the Base Death Save
    /// Penalty.
    fn sheet_fixture_characters() -> Vec<(CprSavedCharacter, codex::rules_core::cpr::input::CprSheetChoices)> {
        use codex::rules_core::cpr::armor::{CprArmorLocation, CprWornArmor};
        use codex::rules_core::cpr::critical::{CprCriticalLocation, CprRecordedCritical};
        use codex::rules_core::cpr::cyberware::CprCyberwareInstall;
        use codex::rules_core::cpr::input::CprSheetChoices;
        use codex::rules_core::cpr::roles::CprRole;

        let mut streetrat = edgerunner();
        for (stat, value) in CprStat::ALL.into_iter().zip([6, 7, 7, 3, 8, 6, 5, 5, 6, 5]) {
            streetrat.input.stats.set(stat, value);
        }
        streetrat.character_id = "cpr-solo-streetrat".to_owned();
        streetrat.revision_id = "cpr-solo-streetrat.rev.1".to_owned();
        streetrat.display_label = "Solo Streetrat".to_owned();

        let mut kitted = edgerunner();
        kitted.character_id = "cpr-solo-kitted".to_owned();
        kitted.revision_id = "cpr-solo-kitted.rev.1".to_owned();
        kitted.display_label = "Kitted Solo".to_owned();
        let choices = CprSheetChoices {
            role: Some((CprRole::Solo, 4)),
            skills: vec![("Handgun".to_owned(), 4)],
            armor: vec![CprWornArmor { name: "Flak".to_owned(), location: CprArmorLocation::Body }],
            cyberware: vec![CprCyberwareInstall { name: "Grafted Muscle and Bone Lace".to_owned(), mount: None }],
            criticals: vec![
                CprRecordedCritical { location: CprCriticalLocation::Body, name: "Collapsed Lung".to_owned() },
                CprRecordedCritical { location: CprCriticalLocation::Body, name: "Broken Ribs".to_owned() },
            ],
            weapons: vec!["Heavy Pistol".to_owned()],
            ..CprSheetChoices::default()
        };

        // E4.3: one weapon of each of the 11 ranged types (p. 171) and a melee weapon, armour on
        // both locations and the Bulletproof Shield.
        let mut arsenal = edgerunner();
        arsenal.character_id = "cpr-solo-arsenal".to_owned();
        arsenal.revision_id = "cpr-solo-arsenal.rev.1".to_owned();
        arsenal.display_label = "Arsenal Solo".to_owned();
        let arsenal_choices = CprSheetChoices {
            role: Some((CprRole::Solo, 4)),
            skills: vec![("Handgun".to_owned(), 4), ("Shoulder Arms".to_owned(), 3), ("Autofire".to_owned(), 2)],
            armor: vec![
                CprWornArmor { name: "Light Armorjack".to_owned(), location: CprArmorLocation::Head },
                CprWornArmor { name: "Medium Armorjack".to_owned(), location: CprArmorLocation::Body },
            ],
            shield: Some("Bulletproof Shield".to_owned()),
            weapons: [
                "Medium Pistol",
                "Heavy Pistol",
                "Very Heavy Pistol",
                "SMG",
                "Heavy SMG",
                "Shotgun",
                "Assault Rifle",
                "Sniper Rifle",
                "Bows & Crossbows",
                "Grenade Launcher",
                "Rocket Launcher",
                "Heavy Melee Weapon",
            ]
            .map(str::to_owned)
            .to_vec(),
            ..CprSheetChoices::default()
        };
        vec![(streetrat, CprSheetChoices::default()), (kitted, choices), (arsenal, arsenal_choices)]
    }

    /// E4.1: the frontend Cyberpunk RED sheet test renders each fixture character's real
    /// `load_saved_character` response. This keeps those files equal to what the adapter
    /// returns today, so the frontend proof never runs on a stale engine answer.
    /// Regenerate: `CPR_SHEET_FIXTURES_WRITE=1 cargo test --bin codex-desktop
    /// the_frontend_cyberpunk_sheet_fixtures_are_the_adapters_load_responses`.
    #[test]
    fn the_frontend_cyberpunk_sheet_fixtures_are_the_adapters_load_responses() {
        let write = std::env::var_os("CPR_SHEET_FIXTURES_WRITE").is_some();
        let fixture_dir =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../src/characterHub/__tests__/cyberpunkSheetFixtures");
        let characters_root = tempdir("sheet-fixtures");
        let mut stale = Vec::new();
        for (character, choices) in sheet_fixture_characters() {
            let root = characters_root.join(&character.character_id);
            CprCharacterStore::save(&character, &root).expect("saves");
            CprCharacterStore::save_choices(&choices, &root).expect("saves the choices");
            let loaded = load_saved_character_via_envelope(&root).expect("loads");
            let json = serde_json::to_string_pretty(&loaded).expect("serialises") + "\n";
            let path = fixture_dir.join(format!("{}.json", character.character_id));
            if write {
                std::fs::create_dir_all(&fixture_dir).expect("fixture dir");
                std::fs::write(&path, &json).expect("writes the fixture");
            } else if std::fs::read_to_string(&path).ok().as_deref() != Some(json.as_str()) {
                stale.push(path.display().to_string());
            }
        }
        std::fs::remove_dir_all(&characters_root).ok();
        assert!(
            stale.is_empty(),
            "stale or missing Cyberpunk RED sheet fixtures {stale:?}; regenerate with CPR_SHEET_FIXTURES_WRITE=1 cargo test --bin codex-desktop the_frontend_cyberpunk_sheet_fixtures_are_the_adapters_load_responses"
        );
    }

    /// A character whose saved choices the book refuses (Rank 11) does not load with a partial
    /// sheet: the load fails with the engine's reason.
    #[test]
    fn a_saved_character_the_engine_refuses_fails_to_load_with_the_reason() {
        use codex::rules_core::cpr::input::CprSheetChoices;
        use codex::rules_core::cpr::roles::CprRole;
        let characters_root = tempdir("refused");
        let root = characters_root.join("cpr-edgerunner");
        CprCharacterStore::save(&edgerunner(), &root).expect("saves");
        let choices = CprSheetChoices { role: Some((CprRole::Solo, 11)), ..CprSheetChoices::default() };
        CprCharacterStore::save_choices(&choices, &root).expect("saves the choices");
        let error = CyberpunkAdapter.load_saved_character(&root).unwrap_err();
        assert!(error.contains("Rank 11"), "{error}");
        std::fs::remove_dir_all(&characters_root).ok();
    }

    /// `save_character` re-saves the CPR character under the next revision and refuses a
    /// stale expected revision; the input survives unchanged.
    #[test]
    fn save_character_advances_the_revision_and_refuses_a_stale_one() {
        let characters_root = tempdir("re-save");
        let root = characters_root.join("cpr-edgerunner");
        CprCharacterStore::save(&edgerunner(), &root).expect("saves");
        let adapter = CyberpunkAdapter;

        let saved = adapter.save_character(&root, "cpr-edgerunner.rev.1", "2026-10-13T01:00:00Z").expect("re-saves");
        assert!(saved.success);
        assert_eq!(saved.revision_id.as_deref(), Some("cpr-edgerunner.rev.2"));
        let reloaded = CprCharacterStore::load(&root).expect("loads");
        assert_eq!(reloaded.revision_id, "cpr-edgerunner.rev.2");
        assert_eq!(reloaded.saved_at, "2026-10-13T01:00:00Z");
        assert_eq!(reloaded.input, edgerunner().input);

        let stale = adapter.save_character(&root, "cpr-edgerunner.rev.1", "2026-10-13T02:00:00Z").expect("answers");
        assert!(!stale.success);
        assert_eq!(stale.error.as_deref(), Some("revision_conflict"));

        std::fs::remove_dir_all(&characters_root).ok();
    }

    /// The Pathfinder-shaped methods refuse with a reason (no Would-message, no zeros
    /// presented as a computed chassis), and a Pathfinder root is refused by name.
    #[test]
    fn pathfinder_shaped_methods_refuse_with_their_reason() {
        let characters_root = tempdir("refusals");
        let root = characters_root.join("cpr-edgerunner");
        CprCharacterStore::save(&edgerunner(), &root).expect("saves");
        let adapter = CyberpunkAdapter;

        let recomputed = adapter.recompute(&root, "cpr-edgerunner");
        assert!(!recomputed.success && recomputed.character.is_none());
        assert!(recomputed.error.as_deref().is_some_and(|e| e.starts_with(NOT_A_PATHFINDER_CHASSIS)), "{recomputed:?}");
        let missing = adapter.recompute(&characters_root.join("absent"), "absent");
        assert_eq!(missing.error.as_deref(), Some("character_not_found"));

        let appended = adapter
            .append_to_character(
                &root,
                &[ItemToAppendDto { item_id: "Dagger (Base)".to_owned(), active_state: crate::character_hub::ActiveStateDto::EquippedActive }],
                "2026-10-13T01:00:00Z",
            )
            .expect("answers");
        assert!(!appended.success);
        assert!(appended.error.as_deref().is_some_and(|e| e.starts_with(NO_PATHFINDER_EQUIPMENT)), "{:?}", appended.error);

        let chassis = adapter.chassis_resolve(&crate::rule_system_adapter::tests::sf_soldier_3_input());
        assert_eq!(chassis.diagnostics.len(), 1);
        assert_eq!(chassis.diagnostics[0].id, NOT_A_PATHFINDER_CHASSIS);
        assert!(chassis.diagnostics[0].claim_blocking && !is_would_message(&chassis.diagnostics[0].message));

        let pf_root = characters_root.join("pf");
        let pf_fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../tests/fixtures/sd14/pf1_human_fighter_level1_saved_character");
        let pf = codex::saved_character::local_store::SavedCharacterStore::load(&pf_fixture).expect("the PF fixture loads");
        codex::saved_character::local_store::SavedCharacterStore::save(&pf, &pf_root).expect("saves");
        assert!(adapter.load_saved_character(&pf_root).is_err_and(|e| e.contains("not 'cyberpunk-red'")));

        std::fs::remove_dir_all(&characters_root).ok();
    }
}
