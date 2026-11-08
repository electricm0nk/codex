//! `CyberpunkAdapter` — the `RuleSystemAdapter` for Cyberpunk RED (`GameSystem::CyberpunkRed`,
//! wire id `cyberpunk-red`).
//!
//! A CPR character lives in its own store (`codex::rules_core::cpr::store`): a directory under
//! the shared characters root holding `envelope.txt` (`game_system = cyberpunk-red`) and
//! `cpr_character_input.txt`, plus `cpr_sheet_choices.txt` once any choice is saved and
//! `cpr_lifepath.txt` once Lifepath answers are. This adapter lists, loads and re-saves those
//! characters; a loaded character's numbers are the rows of its composed sheet
//! (`codex::rules_core::cpr::sheet::compose_sheet` over the CPR corpus), which the sheet renders
//! verbatim, and its dice, rule lines and Lifepath answers are the sheet's printed lines
//! (`cpr_sheet_print`). The sheet's critical-injuries checklist saves through
//! `save_cyberpunk_criticals`.
//!
//! The trait is Pathfinder-typed. Its two methods that take a Pathfinder `CharacterInput`
//! (`chassis_resolve`, `level_up`) and the two that answer with a Pathfinder chassis or
//! equipment (`recompute`, `append_to_character`) cannot carry a CPR character: they answer
//! with the empty plan or a refusal naming the reason, never Pathfinder numbers. CPR's own
//! actions (creation, improvement) are their own commands (`cpr_creation`, `cpr_improve`).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use codex::rules_core::character_input::CharacterInput;
use codex::rules_core::cpr::critical::{CprCriticalLocation, CprRecordedCritical};
use codex::rules_core::cpr::input::CprKitEntry;
use codex::rules_core::cpr::kit::{CprKitCatalog, CprKitChoices, CprKitItem, CprKitPick, CprKitPurchase};
use codex::rules_core::cpr::sheet::{compose_sheet, CprSheetCatalogs};
use codex::rules_core::cpr::store::{CprCharacterStore, CprSavedCharacter};
use codex::rules_core::game_system::{runtime_repo_root, GameSystem};
use codex::rules_core::level_up::LevelUpPlan;
use codex::rules_core::pilot_compute::{
    AbilityModifiers, BaseSaves, ComputationDiagnostic, PilotBaseChassisComputation, SelectedSkillModifiers,
};

use crate::character_hub::{
    AbilityScoresDto, CharacterSummaryDto, CorpusDerivedDto, EncumbranceDto, EquipmentEffectsDto, ExplanationDto,
    ListSavedCharactersResponse, LoadSavedCharacterResponse,
};
use crate::cpr_creation::{
    create_at_root, CprCreateRequest, CprCreateResponse, CprCreationMethod, CprCreationRequest, CprLifepathRequest,
    CprRoleAbilityRequest, CprRolePickRequest, CprSkillsRequest,
};
use crate::cpr_sheet_print::CprPrintCatalog;
use crate::characterHub::appendToCharacter::{AppendToCharacterResponse, ItemToAppendDto};
use crate::characterHub::recomputeCharacter::RecomputeCharacterResponse;
use crate::characterHub::reSaveCharacter::ReSaveCharacterResponse;
use crate::character_hub::{refund_copper, PriceMode};
use crate::cpr_catalog::{market_rows, CprMarketRow};
use crate::rule_system_adapter::{ClassLevelDelta, RuleSystemAdapter};
use crate::sf_catalog::{MarketShellCopyDto, MarketShellOutcome, MarketShellRequest, MarketShellStateDto};

/// Diagnostic id and error prefix: a Pathfinder chassis cannot describe a CPR character.
pub(crate) const NOT_A_PATHFINDER_CHASSIS: &str = "cpr_adapter.not_a_pathfinder_chassis";
/// Error prefix: the Pathfinder equipment append does not apply to a CPR character.
pub(crate) const NO_PATHFINDER_EQUIPMENT: &str = "cpr_adapter.no_pathfinder_equipment";

/// The Market's money label on this system.
pub const MARKET_CURRENCY: &str = "eddies";

/// The adapter `rule_system_adapter::resolve_rule_system_adapter` returns for `cyberpunk-red`.
pub struct CyberpunkAdapter;

/// The CPR corpus the running app reads (packaged-root aware).
fn corpus_root() -> PathBuf {
    GameSystem::CyberpunkRed.package_roots(&runtime_repo_root()).corpus
}

/// One Critical Injury the sheet's checklist holds ticked: its table and its name.
#[derive(Debug, Clone, Deserialize)]
pub struct CprCriticalDto {
    pub location: CprCriticalLocation,
    pub name: String,
}

/// `save_cyberpunk_criticals`: the character and every Critical Injury its checklist holds
/// ticked, in the order ticked.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprCriticalsSaveRequest {
    pub character_id: String,
    pub criticals: Vec<CprCriticalDto>,
}

/// Replaces the recorded Critical Injuries of the CPR character at `root` with `criticals` and
/// answers with the reloaded sheet (its Base Death Save Penalty row moved by the engine). The
/// new choices are composed first: an injury not on its table, or one recorded twice (p. 187),
/// is refused with the engine's reason and nothing is written. Every other saved choice stays.
pub(crate) fn save_criticals_at_root(root: &Path, criticals: &[CprRecordedCritical]) -> Result<LoadSavedCharacterResponse, String> {
    let character = CprCharacterStore::load(root)?;
    let mut choices = CprCharacterStore::load_choices(root)?;
    choices.criticals = criticals.to_vec();
    compose_sheet(&CprSheetCatalogs::load(&corpus_root())?, &character.input, &choices)?;
    CprCharacterStore::save_choices(&choices, root)?;
    CyberpunkAdapter.load_saved_character(root)
}

/// Save the Critical Injuries ticked on a Cyberpunk RED character's sheet; answers with the
/// reloaded sheet.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn save_cyberpunk_criticals(app: tauri::AppHandle, request: CprCriticalsSaveRequest) -> Result<LoadSavedCharacterResponse, String> {
    let root = crate::character_hub::resolve_character_root(&app, &request.character_id)?;
    let criticals: Vec<CprRecordedCritical> =
        request.criticals.into_iter().map(|c| CprRecordedCritical { location: c.location, name: c.name }).collect();
    save_criticals_at_root(&root, &criticals)
}

/// What one sale of a held kit entry takes and pays before the price mode: `(unit, price of a unit)`,
/// off the Market row that prints the same name. A starter kit can print an item under a name the
/// shop does not use (`Rope`, `Urbanflash Jacket`), with no price to go by: it sells for nothing,
/// and says so. Cyberware and a deck, its programs and hardware are not on the Market: `None`. The
/// row is read before the shop's other kinds, because a good can share a cyberware's name
/// (`Radio Communicator`, `Dartgun`); what a kit installed says so by its section.
fn market_terms(catalog: &CprKitCatalog, rows: &[CprMarketRow], entry: &CprKitEntry) -> Option<(u32, u32)> {
    if entry.section == "cyberware" || entry.name.starts_with("Program: ") || entry.name.starts_with("Cyberdeck (") {
        return None;
    }
    if let Some(row) = rows.iter().find(|r| r.item.name == entry.name) {
        return Some((row.unit, row.item.cost_eb));
    }
    let installed =
        catalog.shop().iter().any(|s| s.name == entry.name && ["cyberware", "cyberdeck", "net_hardware", "program"].contains(&s.kind.as_str()));
    (!installed).then_some((1, 0))
}

/// The balance (the sheet's Cash) and every copy the kit holds that the Market buys back: ammunition
/// by the unit a sale takes, anything else one row per piece.
pub(crate) fn market_state_at_root(root: &Path) -> Result<MarketShellStateDto, String> {
    CprCharacterStore::load(root)?;
    let choices = CprCharacterStore::load_choices(root)?;
    let kit = CprCharacterStore::load_kit(root)?;
    let catalog = CprKitCatalog::load(&corpus_root())?;
    let rows = market_rows(&catalog)?;
    let mut held = Vec::new();
    for (index, entry) in kit.items.iter().enumerate() {
        let Some((unit, _)) = market_terms(&catalog, &rows, entry) else { continue };
        let key = format!("{index}|{}", entry.name);
        if unit > 1 {
            let name = format!("{} ({} of {})", entry.name, unit.min(entry.quantity), entry.quantity);
            held.push(MarketShellCopyDto { key, name });
        } else {
            held.extend((0..entry.quantity).map(|_| MarketShellCopyDto { key: key.clone(), name: entry.name.clone() }));
        }
    }
    Ok(MarketShellStateDto { currency: MARKET_CURRENCY.to_owned(), balance: i64::from(choices.cash.unwrap_or(0)), held })
}

/// Composes the sheet with the new choices and saves them with the kit, or refuses and saves nothing.
fn market_save(
    root: &Path,
    character: &CprSavedCharacter,
    kit: &codex::rules_core::cpr::input::CprKit,
    choices: &codex::rules_core::cpr::input::CprSheetChoices,
    moved: u32,
) -> Result<MarketShellOutcome, String> {
    if let Err(reason) = compose_sheet(&CprSheetCatalogs::load(&corpus_root())?, &character.input, choices) {
        return Ok(MarketShellOutcome::blocked("cpr_market.sheet_refused", reason));
    }
    CprCharacterStore::save_kit(kit, root)?;
    CprCharacterStore::save_choices(choices, root)?;
    Ok(MarketShellOutcome::Saved { moved: i64::from(moved) })
}

/// Buys one Market row in one save: the kit holds it, the sheet shows it where the kit's reading puts
/// it, and the Cash pays for it, unless the mode is Cashless; a buy over the Cash is refused.
pub(crate) fn market_buy_at_root(root: &Path, request: &MarketShellRequest) -> Result<MarketShellOutcome, String> {
    let character = CprCharacterStore::load(root)?;
    let mut choices = CprCharacterStore::load_choices(root)?;
    let mut kit = CprCharacterStore::load_kit(root)?;
    let catalog = CprKitCatalog::load(&corpus_root())?;
    let rows = market_rows(&catalog)?;
    let Some(row) = rows.iter().find(|r| r.row.key == request.key) else {
        return Ok(MarketShellOutcome::blocked("cpr_market.not_on_offer", format!("{}: not on the Market", request.key)));
    };
    let cash = choices.cash.unwrap_or(0);
    let cost = if request.price_mode == PriceMode::Cashless { 0 } else { row.item.cost_eb };
    if cost > cash {
        return Ok(MarketShellOutcome::blocked(
            "cpr_market.over_cash",
            format!("{} costs {cost}eb; the character has {cash}eb", row.row.name),
        ));
    }
    let sheet = catalog.sheet_of_item(&row.item);
    if sheet.shield.is_some() && choices.shield.is_some() {
        return Ok(MarketShellOutcome::blocked("cpr_market.one_shield", "The character already holds a shield; sell it first"));
    }
    choices.weapons.extend(sheet.weapons);
    choices.armor.extend(sheet.armor);
    choices.shield = sheet.shield.or(choices.shield);
    for (label, count) in sheet.ammunition {
        match choices.ammunition.iter_mut().find(|(held, _)| *held == label) {
            Some((_, held)) => *held += count,
            None => choices.ammunition.push((label, count)),
        }
    }
    if cost > 0 {
        choices.cash = Some(cash - cost);
    }
    match kit.items.iter_mut().find(|e| row.unit > 1 && e.name == row.item.name) {
        Some(entry) => entry.quantity += row.unit,
        None => kit.items.push(CprKitEntry { section: row.item.section.clone(), name: row.item.name.clone(), quantity: row.unit }),
    }
    market_save(root, &character, &kit, &choices, cost)
}

/// Sells the copy the key names, at the mode's share of its price: the kit lets it go, and the sheet
/// loses what the kit's reading put there.
pub(crate) fn market_sell_at_root(root: &Path, request: &MarketShellRequest) -> Result<MarketShellOutcome, String> {
    let character = CprCharacterStore::load(root)?;
    let mut choices = CprCharacterStore::load_choices(root)?;
    let mut kit = CprCharacterStore::load_kit(root)?;
    let catalog = CprKitCatalog::load(&corpus_root())?;
    let rows = market_rows(&catalog)?;
    let not_held = || MarketShellOutcome::blocked("cpr_market.not_held", format!("{}: the character does not hold that copy", request.key));
    let Some((index, name)) = request.key.split_once('|') else { return Ok(not_held()) };
    let Ok(index) = index.parse::<usize>() else { return Ok(not_held()) };
    let Some(entry) = kit.items.get(index).filter(|e| e.name == name) else { return Ok(not_held()) };
    let Some((unit, unit_cost)) = market_terms(&catalog, &rows, entry) else { return Ok(not_held()) };
    let taken = unit.min(entry.quantity);
    let value = unit_cost * taken / unit;
    let refund = u32::try_from(refund_copper(u64::from(value), request.price_mode)).unwrap_or(0);

    let sheet = catalog.sheet_of_item(&CprKitItem {
        section: entry.section.clone(),
        kind: String::new(),
        name: entry.name.clone(),
        quantity: taken,
        cost_eb: 0,
        fashion: false,
    });
    for weapon in sheet.weapons {
        if let Some(at) = choices.weapons.iter().position(|w| *w == weapon) {
            choices.weapons.remove(at);
        }
    }
    for piece in sheet.armor {
        if let Some(at) = choices.armor.iter().position(|a| *a == piece) {
            choices.armor.remove(at);
        }
    }
    if sheet.shield.is_some() {
        choices.shield = None;
    }
    for (label, count) in sheet.ammunition {
        if let Some(at) = choices.ammunition.iter().position(|(held, _)| *held == label) {
            choices.ammunition[at].1 = choices.ammunition[at].1.saturating_sub(count);
            if choices.ammunition[at].1 == 0 {
                choices.ammunition.remove(at);
            }
        }
    }
    if refund > 0 {
        choices.cash = Some(choices.cash.unwrap_or(0) + refund);
    }
    kit.items[index].quantity -= taken;
    if kit.items[index].quantity == 0 {
        kit.items.remove(index);
    }
    market_save(root, &character, &kit, &choices, refund)
}

/// The Market's balance, currency and Sell list for a saved Cyberpunk RED character.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn cyberpunk_market_state(app: tauri::AppHandle, character_id: String) -> Result<MarketShellStateDto, String> {
    market_state_at_root(&crate::character_hub::resolve_character_root(&app, &character_id)?)
}

/// Buys one Market row for a saved Cyberpunk RED character.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn buy_cyberpunk_market(app: tauri::AppHandle, request: MarketShellRequest) -> Result<MarketShellOutcome, String> {
    market_buy_at_root(&crate::character_hub::resolve_character_root(&app, &request.character_id)?, &request)
}

/// Sells one held copy of a saved Cyberpunk RED character.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn sell_cyberpunk_market(app: tauri::AppHandle, request: MarketShellRequest) -> Result<MarketShellOutcome, String> {
    market_sell_at_root(&crate::character_hub::resolve_character_root(&app, &request.character_id)?, &request)
}

/// The Cyberpunk RED starter characters, in the order they are written: each one's marker
/// file and the creation request that builds it through the Create screen's own path
/// (`cpr_creation::create_at_root`). Every die is fixed by the request's seed or pick, so a
/// restore rebuilds the same character byte for byte. The full builds, with the arithmetic of
/// every number they put on the sheet, are `docs/release/SD-38-cyberpunk-red/artifacts/epic_7/cpr-seed-builds.md`.
///
/// **Jax Moreno**, Streetrat Solo: Solo template row 3 (p. 75), the printed skill list, Combat
/// Awareness 4 split Initiative Reaction 2 / Threat Detection 2 (p. 147), the fixed Solo kit
/// with its first options and the Solo cyberware pack (Humanity Loss 14, p. 118).
///
/// **Nyx Calder**, Complete Package Netrunner: 62 STAT points (p. 79), the 13 Basic skills and
/// eight more for 86 skill points (p. 90), Interface 4 (p. 148), and 2,090 of the 2,550eb
/// (p. 105) on a Standard Cyberdeck, six programs, a Neural Link with Interface Plugs, a Heavy
/// Pistol with 50 rounds and Light Armorjack head and body.
///
/// **Imani Okafor**, Edgerunner Medtech: ten 1d10 STAT rolls from seed 2045 (p. 78), the
/// Medtech list moved within its 86 points (Shoulder Arms 6 -> 4, First Aid 2 -> 4), Medicine
/// Surgery 2 / Pharmaceuticals 1 (Speedheal) / Cryosystem Operation 1 (D-7), the fixed Medtech
/// kit with the Assault Rifle and its rifle ammunition picked, and the Medtech cyberware pack
/// (Humanity Loss 12).
///
/// Each one's Lifepath and Cultural Origin language are rolled by its own seed (Jax 2020, Nyx
/// 2077, Imani 2045), so the three answer the tables differently.
pub(crate) fn starter_requests(saved_at: &str) -> Vec<(&'static str, CprCreateRequest)> {
    let map = |pairs: &[(&str, u8)]| -> BTreeMap<String, u8> { pairs.iter().map(|(k, v)| ((*k).to_owned(), *v)).collect() };
    let purchase = |kind: &str, name: &str, variant: Option<&str>, quantity: u32| CprKitPurchase {
        kind: kind.to_owned(),
        name: name.to_owned(),
        variant: variant.map(str::to_owned),
        quantity,
    };
    let request = |id: &str, name: &str, creation: CprCreationRequest| CprCreateRequest {
        character_id: id.to_owned(),
        display_label: name.to_owned(),
        saved_at: saved_at.to_owned(),
        creation,
    };
    let solo = CprCreationRequest {
        role: "Solo".to_owned(),
        method: CprCreationMethod::Streetrat,
        seed: Some(2020),
        template_roll: Some(3),
        stats: None,
        lifepath: CprLifepathRequest::default(),
        skills: CprSkillsRequest::default(),
        role_ability: CprRoleAbilityRequest {
            points: map(&[("Initiative Reaction", 2), ("Threat Detection", 2)]),
            picks: Vec::new(),
        },
        kit: CprKitChoices::default(),
    };
    let netrunner = CprCreationRequest {
        role: "Netrunner".to_owned(),
        method: CprCreationMethod::CompletePackage,
        seed: Some(2077),
        template_roll: None,
        stats: Some(map(&[
            ("int", 7),
            ("ref", 6),
            ("dex", 6),
            ("tech", 7),
            ("cool", 6),
            ("will", 5),
            ("luck", 6),
            ("move", 6),
            ("body", 6),
            ("emp", 7),
        ])),
        lifepath: CprLifepathRequest::default(),
        skills: CprSkillsRequest {
            levels: map(&[
                ("Concentration", 4),
                ("Education", 4),
                ("Evasion", 4),
                ("Local Expert (Your Home)", 4),
                ("Perception", 4),
                ("Stealth", 4),
                ("Electronics/Security Tech", 6),
                ("Basic Tech", 6),
                ("Cybertech", 6),
                ("Handgun", 6),
                ("Library Search", 6),
                ("Cryptography", 4),
                ("Conceal/Reveal Object", 4),
                ("Streetwise", 4),
            ]),
            specialties: BTreeMap::new(),
        },
        role_ability: CprRoleAbilityRequest::default(),
        kit: CprKitChoices {
            picks: Vec::new(),
            purchases: vec![
                purchase("cyberdeck", "Cyberdeck (Standard Quality)", None, 1),
                purchase("program", "Sword", None, 2),
                purchase("program", "Armor", None, 1),
                purchase("program", "Worm", None, 1),
                purchase("program", "See Ya", None, 1),
                purchase("program", "Eraser", None, 1),
                purchase("cyberware", "Neural Link", None, 1),
                purchase("cyberware", "Interface Plugs", None, 1),
                purchase("weapon", "Heavy Pistol", None, 1),
                purchase("ammo", "Basic Ammunition", Some("H Pistol"), 50),
                purchase("armor", "Light Armorjack", Some("Body"), 1),
                purchase("armor", "Light Armorjack", Some("Head"), 1),
            ],
        },
    };
    let medtech = CprCreationRequest {
        role: "Medtech".to_owned(),
        method: CprCreationMethod::Edgerunner,
        seed: Some(2045),
        template_roll: None,
        stats: None,
        lifepath: CprLifepathRequest::default(),
        skills: CprSkillsRequest {
            levels: map(&[("Shoulder Arms", 4), ("First Aid", 4)]),
            specialties: [("Science (choose 1)".to_owned(), "Biology".to_owned())].into_iter().collect(),
        },
        role_ability: CprRoleAbilityRequest {
            points: map(&[
                ("Surgery", 2),
                ("Medical Tech (Pharmaceuticals)", 1),
                ("Medical Tech (Cryosystem Operation)", 1),
            ]),
            picks: vec![CprRolePickRequest { name: "Speedheal".to_owned(), on: None }],
        },
        kit: CprKitChoices {
            // The fixed Medtech kit's first three lines (p. 99): "Shotgun or Assault Rifle" and
            // the two ammunition lines that follow it.
            picks: vec![
                CprKitPick { line: 0, option: "Assault Rifle".to_owned() },
                CprKitPick { line: 1, option: "Basic Rifle Ammunition".to_owned() },
                CprKitPick { line: 2, option: "Incendiary Rifle Ammunition".to_owned() },
            ],
            purchases: Vec::new(),
        },
    };
    vec![
        (".default_character_seeded_4", request("00000000-0000-0000-0000-000000000004", "Jax Moreno", solo)),
        (".default_character_seeded_5", request("00000000-0000-0000-0000-000000000005", "Nyx Calder", netrunner)),
        (".default_character_seeded_6", request("00000000-0000-0000-0000-000000000006", "Imani Okafor", medtech)),
    ]
}

/// Builds and saves one Cyberpunk RED starter at `root` through the Create screen's path over
/// the running app's CPR corpus. A build creation refuses saves nothing and is an error naming
/// every refusal.
pub(crate) fn create_starter_at(root: &Path, request: &CprCreateRequest, app_version: &str) -> Result<(), String> {
    match create_at_root(&corpus_root(), root, request, app_version.to_owned())? {
        CprCreateResponse::Saved { .. } => Ok(()),
        CprCreateResponse::Blocked { diagnostics } => {
            let blocking: Vec<String> = diagnostics.iter().map(|d| format!("{}: {}", d.id, d.message)).collect();
            Err(format!(
                "starter character '{}' did not compute (blocking: {blocking:?}); not seeding",
                request.display_label
            ))
        }
    }
}

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
    /// `sheet_lines` its printed dice, words and rule lines (`cpr_sheet_print`). A saved choice the engine
    /// refuses fails the load with the engine's reason; no partial sheet is returned. The
    /// Pathfinder-only blocks of the response (snapshot, corpus-derived equipment, feats,
    /// spells, race traits, the six PF ability scores) are empty or zero; when the sheet prints
    /// no line, `sheet_rules_unavailable_reason` says why: Cyberpunk RED has no sheet-rule
    /// package.
    fn load_saved_character(&self, root: &Path) -> Result<LoadSavedCharacterResponse, String> {
        let character = CprCharacterStore::load(root)?;
        let choices = CprCharacterStore::load_choices(root)?;
        let lifepath = CprCharacterStore::load_lifepath(root)?;
        let corpus = corpus_root();
        let sheet = compose_sheet(&CprSheetCatalogs::load(&corpus)?, &character.input, &choices)?;
        // Every printed line: the sheet's dice and words, the Lifepath answers, and the rule
        // lines (Role Ability, cyberware, Humanity Loss, weapon specials, brain damage).
        let sheet_lines = crate::cpr_sheet_print::sheet_lines(&CprPrintCatalog::load(&corpus)?, &sheet, &choices, &lifepath)?;
        let sheet_rules_unavailable_reason = sheet_lines.is_empty().then(|| {
            "Cyberpunk RED has no sheet-rule package; its sheet renders the cpr. explanation rows".to_owned()
        });
        Ok(LoadSavedCharacterResponse {
            admitted_under_override: Vec::new(),
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
pub(crate) mod tests {
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
            cyberware: vec![CprCyberwareInstall { name: "Grafted Muscle and Bone Lace".to_owned(), mount: None, humanity_loss: None }],
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
    pub(crate) fn sheet_fixture_characters() -> Vec<(CprSavedCharacter, codex::rules_core::cpr::input::CprSheetChoices)> {
        use codex::rules_core::cpr::armor::{CprArmorLocation, CprWornArmor};
        use codex::rules_core::cpr::critical::{CprCriticalLocation, CprRecordedCritical};
        use codex::rules_core::cpr::cyberware::{CprCyberwareInstall, CprMount};
        use codex::rules_core::cpr::input::{CprDeckChoice, CprImprovementPoints, CprSheetChoices};
        use codex::rules_core::cpr::reputation::{CprReputationEvent, CprReputationKind};
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
            cyberware: vec![CprCyberwareInstall { name: "Grafted Muscle and Bone Lace".to_owned(), mount: None, humanity_loss: None }],
            criticals: vec![
                CprRecordedCritical { location: CprCriticalLocation::Body, name: "Collapsed Lung".to_owned() },
                CprRecordedCritical { location: CprCriticalLocation::Body, name: "Broken Ribs".to_owned() },
            ],
            weapons: vec!["Heavy Pistol".to_owned()],
            // E4.5: two Reputation events (the second, lower, does not replace the first) and
            // the Improvement Points box.
            reputation: vec![
                CprReputationEvent { level: 3, kind: CprReputationKind::Deed, description: "Held the bridge".to_owned() },
                CprReputationEvent { level: 2, kind: CprReputationKind::Cowardice, description: "Ran from the Maelstrom".to_owned() },
            ],
            improvement_points: Some(CprImprovementPoints { earned: 40, spent: 20 }),
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
        // E4.4: a Netrunner in a Bodyweight Suit with a Standard Cyberdeck in a Cyberarm (A-7),
        // loaded with the p. 157 sample deck and a Hardware piece.
        let mut netrunner = edgerunner();
        netrunner.character_id = "cpr-netrunner-decked".to_owned();
        netrunner.revision_id = "cpr-netrunner-decked.rev.1".to_owned();
        netrunner.display_label = "Decked Netrunner".to_owned();
        let netrunner_choices = CprSheetChoices {
            role: Some((CprRole::Netrunner, 4)),
            armor: vec![CprWornArmor { name: "Bodyweight Suit".to_owned(), location: CprArmorLocation::Body }],
            cyberware: vec![
                CprCyberwareInstall { name: "Neural Link".to_owned(), mount: None, humanity_loss: None },
                CprCyberwareInstall { name: "Interface Plugs".to_owned(), mount: None, humanity_loss: None },
                CprCyberwareInstall { name: "Cyberarm".to_owned(), mount: None, humanity_loss: None },
                CprCyberwareInstall { name: "Cyberdeck (installed)".to_owned(), mount: Some(CprMount::Arm(0)), humanity_loss: None },
            ],
            deck: Some(CprDeckChoice {
                deck: "Cyberdeck (Standard Quality)".to_owned(),
                programs: ["Sword", "Sword", "Killer", "Worm", "Worm", "Armor"].map(str::to_owned).to_vec(),
                hardware: vec!["Range Upgrade".to_owned()],
            }),
            ..CprSheetChoices::default()
        };
        // E4.8: the p121 sheet's Cash, Ammunition and Rent fields -- cash, two ammunition stacks
        // (the Solo kit's, p099) and housing with a rent in eb (Cargo Container, p379).
        let mut ledger = edgerunner();
        ledger.character_id = "cpr-solo-ledger".to_owned();
        ledger.revision_id = "cpr-solo-ledger.rev.1".to_owned();
        ledger.display_label = "Ledger Solo".to_owned();
        let ledger_choices = CprSheetChoices {
            cash: Some(1450),
            ammunition: vec![("Basic VH Pistol Ammunition".to_owned(), 30), ("Basic Rifle Ammunition".to_owned(), 70)],
            housing: Some("Cargo Container".to_owned()),
            ..CprSheetChoices::default()
        };
        vec![
            (streetrat, CprSheetChoices::default()),
            (kitted, choices),
            (arsenal, arsenal_choices),
            (netrunner, netrunner_choices),
            (ledger, ledger_choices),
        ]
    }

    /// E4.5: the kitted Solo's Lifepath answers, as creation saves them (`cpr_creation`'s
    /// `saved_lifepath`): cells of `lifepath_table.json` rows (p. 45-70); a whole-row table keeps
    /// the row's other cells as the detail.
    pub(crate) fn sheet_fixture_lifepath(character_id: &str) -> Option<codex::rules_core::cpr::input::CprLifepath> {
        use codex::rules_core::cpr::input::{CprLifepath, CprLifepathAnswer};
        let answer = |table: &str, entry: u8, column: &str, text: &str, detail: &str| CprLifepathAnswer {
            table: table.to_owned(),
            entry,
            column: column.to_owned(),
            text: text.to_owned(),
            detail: detail.to_owned(),
        };
        (character_id == "cpr-solo-kitted").then(|| CprLifepath {
            answers: vec![
                answer(
                    "Cultural Origins",
                    0,
                    "Your (General) Cultural Region",
                    "North American",
                    "Chinese, Cree, Creole, English, French, Navajo, Spanish",
                ),
                answer("Your Personality", 0, "What Are You Like?", "Shy and secretive", ""),
                answer("Your Friends", 1, "Friend's Relationship to You", "Like an older sibling to you.", ""),
                answer("Your Friends", 2, "Friend's Relationship to You", "Like a younger sibling to you.", ""),
                answer("Solo: What Kind of Solo are You?", 0, "Type", "Bodyguard", ""),
            ],
        })
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
            if let Some(lifepath) = sheet_fixture_lifepath(&character.character_id) {
                CprCharacterStore::save_lifepath(&lifepath, &root).expect("saves the Lifepath");
            }
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

    /// E4.5: ticking a Critical Injury on the sheet's checklist saves it through the adapter
    /// and the answer is the reloaded sheet, whose Base Death Save Penalty row moves: Collapsed
    /// Lung raises it by 1 (p. 221), Broken Ribs does not. Unticking removes it. An injury not on
    /// its table, or the same injury twice (p. 187), is refused with the engine's reason and the
    /// saved choices stay as they were. The character's other choices survive every save.
    #[test]
    fn ticking_a_critical_injury_saves_it_and_moves_the_base_death_save_penalty() {
        use codex::rules_core::cpr::critical::{CprCriticalLocation, CprRecordedCritical};
        use codex::rules_core::cpr::input::CprSheetChoices;
        use codex::rules_core::cpr::roles::CprRole;

        let characters_root = tempdir("criticals");
        let root = characters_root.join("cpr-edgerunner");
        CprCharacterStore::save(&edgerunner(), &root).expect("saves");
        let choices = CprSheetChoices { role: Some((CprRole::Solo, 4)), ..CprSheetChoices::default() };
        CprCharacterStore::save_choices(&choices, &root).expect("saves the choices");
        let penalty = |r: &LoadSavedCharacterResponse| {
            r.explanations.iter().find(|e| e.id == "cpr.wound.base_death_save_penalty").expect("the penalty row").value
        };
        let ticked = |r: &LoadSavedCharacterResponse| -> Vec<(String, String)> {
            r.sheet_lines.iter().filter(|l| l.id.starts_with("cpr.critical.")).map(|l| (l.id.clone(), l.value.clone())).collect()
        };
        let body = |name: &str| CprRecordedCritical { location: CprCriticalLocation::Body, name: name.to_owned() };

        let before = load_saved_character_via_envelope(&root).expect("loads");
        assert_eq!((penalty(&before), ticked(&before).len()), (0, 0));

        let saved = save_criticals_at_root(&root, &[body("Collapsed Lung")]).expect("saves the tick");
        assert_eq!(penalty(&saved), 1);
        assert_eq!(ticked(&saved), [("cpr.critical.body.0".to_owned(), "Collapsed Lung".to_owned())]);
        let reloaded = load_saved_character_via_envelope(&root).expect("reloads");
        assert_eq!(penalty(&reloaded), 1, "the tick is on disk, not only in the answer");
        assert_eq!(CprCharacterStore::load_choices(&root).expect("choices").criticals, [body("Collapsed Lung")]);

        let both = save_criticals_at_root(&root, &[body("Collapsed Lung"), body("Broken Ribs")]).expect("saves");
        assert_eq!((penalty(&both), ticked(&both).len()), (1, 2), "Broken Ribs does not raise the penalty");
        let unticked = save_criticals_at_root(&root, &[body("Broken Ribs")]).expect("saves the untick");
        assert_eq!(penalty(&unticked), 0);
        assert_eq!(ticked(&unticked), [("cpr.critical.body.0".to_owned(), "Broken Ribs".to_owned())]);

        let unknown = save_criticals_at_root(&root, &[body("Paper Cut")]).unwrap_err();
        assert!(unknown.contains("not on the Critical Injuries"), "{unknown}");
        let twice = save_criticals_at_root(&root, &[body("Broken Ribs"), body("Broken Ribs")]).unwrap_err();
        assert!(twice.contains("rerolled"), "{twice}");
        let kept = CprCharacterStore::load_choices(&root).expect("choices");
        assert_eq!(kept.criticals, [body("Broken Ribs")], "a refused save writes nothing");
        assert_eq!(kept.role, Some((CprRole::Solo, 4)), "the other choices survive");

        let missing = save_criticals_at_root(&characters_root.join("absent"), &[body("Broken Ribs")]);
        assert!(missing.is_err());
        std::fs::remove_dir_all(&characters_root).ok();
    }

    /// E4.5: the Lifepath answers saved at creation (`cpr_lifepath.txt`) load as printed word
    /// lines, one per answer in the saved order, labelled by table and column; a character with
    /// none loads with no Lifepath line.
    #[test]
    fn saved_lifepath_answers_load_as_printed_lines() {
        use codex::rules_core::cpr::input::{CprLifepath, CprLifepathAnswer};
        let characters_root = tempdir("lifepath");
        let root = characters_root.join("cpr-edgerunner");
        CprCharacterStore::save(&edgerunner(), &root).expect("saves");
        let none = load_saved_character_via_envelope(&root).expect("loads");
        assert!(!none.sheet_lines.iter().any(|l| l.id.starts_with("cpr.lifepath.")));

        let answer = |table: &str, entry: u8, column: &str, text: &str, detail: &str| CprLifepathAnswer {
            table: table.to_owned(),
            entry,
            column: column.to_owned(),
            text: text.to_owned(),
            detail: detail.to_owned(),
        };
        let lifepath = CprLifepath {
            answers: vec![
                answer("Cultural Origins", 0, "Your (General) Cultural Region", "East Asian", "Chinese, Cantonese, Japanese"),
                answer("Your Friends", 1, "Friend's Relationship to You", "Like an older sibling to you", ""),
                answer("Your Friends", 2, "Friend's Relationship to You", "A childhood friend", ""),
            ],
        };
        CprCharacterStore::save_lifepath(&lifepath, &root).expect("saves the answers");
        let loaded = load_saved_character_via_envelope(&root).expect("loads");
        let lines: Vec<(&str, &str, &str, &str)> = loaded
            .sheet_lines
            .iter()
            .filter(|l| l.id.starts_with("cpr.lifepath."))
            .map(|l| (l.id.as_str(), l.label.as_str(), l.form.as_str(), l.value.as_str()))
            .collect();
        assert_eq!(
            lines,
            [
                ("cpr.lifepath.0", "Cultural Origins: Your (General) Cultural Region", "words", "East Asian"),
                ("cpr.lifepath.1", "Your Friends: Friend's Relationship to You", "words", "Like an older sibling to you"),
                ("cpr.lifepath.2", "Your Friends: Friend's Relationship to You", "words", "A childhood friend"),
            ]
        );
        assert!(loaded.sheet_lines[loaded.sheet_lines.len() - 3].prose.contains("Chinese, Cantonese, Japanese"));
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

    // --- The Market ---

    fn market_root(label: &str, starter: &str) -> std::path::PathBuf {
        let root = tempdir(label).join("c");
        let (_, request) = starter_requests("2026-11-02T00:00:00Z").into_iter().find(|(_, r)| r.display_label == starter).expect("starter");
        create_starter_at(&root, &request, "test").expect("creates");
        root
    }

    /// A Cyberpunk RED Solo with no armor, no weapon and an empty kit loads: the sheet reads what
    /// is held and asks for nothing.
    #[test]
    fn no_required_gear_cyberpunk_solo_loads_with_nothing_held() {
        let root = market_root("no-required-gear", "Jax Moreno");
        let geared = load_saved_character_via_envelope(&root).expect("the starter loads");
        let mut choices = CprCharacterStore::load_choices(&root).unwrap();
        assert!(!choices.armor.is_empty() && !choices.weapons.is_empty(), "the starter holds gear: {choices:?}");
        choices.armor.clear();
        choices.weapons.clear();
        let mut kit = CprCharacterStore::load_kit(&root).unwrap();
        kit.items.clear();
        CprCharacterStore::save_choices(&choices, &root).unwrap();
        CprCharacterStore::save_kit(&kit, &root).unwrap();

        let loaded = load_saved_character_via_envelope(&root).expect("nothing held still loads");
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
        assert!(loaded.diagnostics.is_empty(), "{:?}", loaded.diagnostics);
        let row = |sheet: &LoadSavedCharacterResponse, id: &str| sheet.explanations.iter().find(|row| row.id == id).map(|row| row.value);
        assert!(row(&loaded, "cpr.stat.int").is_some(), "{:?}", loaded.explanations.iter().map(|r| &r.id).collect::<Vec<_>>());
        assert_eq!(row(&loaded, "cpr.stat.int"), row(&geared, "cpr.stat.int"));
    }

    fn market_request(key: &str, price_mode: PriceMode) -> MarketShellRequest {
        MarketShellRequest { character_id: "c".into(), key: key.into(), price_mode, saved_at: "t".into() }
    }

    fn moved(outcome: MarketShellOutcome) -> i64 {
        match outcome {
            MarketShellOutcome::Saved { moved } => moved,
            MarketShellOutcome::Blocked { diagnostics } => panic!("blocked: {diagnostics:?}"),
        }
    }

    fn sheet_row(root: &Path, id: &str) -> Option<i16> {
        CyberpunkAdapter.load_saved_character(root).unwrap().explanations.iter().find(|e| e.id == id).map(|e| e.value)
    }

    fn weapon_names(root: &Path) -> Vec<String> {
        CprCharacterStore::load_choices(root).unwrap().weapons
    }

    /// Three modes on a 500eb Heavy Pistol-class weapon: what moves, and that the sheet follows.
    #[test]
    fn market_shell_buy_and_sell_honour_the_three_price_modes() {
        let key = "weapon|Heavy Pistol|";
        // weapon.json: Heavy Pistol is 100eb (Premium), p341.
        for (mode, charged, returned) in [
            (PriceMode::Standard, 100, 50),
            (PriceMode::CharacterBuild, 100, 100),
            (PriceMode::Cashless, 0, 0),
        ] {
            let root = market_root("market-modes", "Jax Moreno");
            let start = market_state_at_root(&root).unwrap();
            assert_eq!(start.currency, "eddies");
            assert_eq!(i64::from(sheet_row(&root, "cpr.cash").unwrap()), start.balance);
            let weapons = weapon_names(&root).len();

            assert_eq!(moved(market_buy_at_root(&root, &market_request(key, mode)).unwrap()), charged, "{mode:?}");
            let bought = market_state_at_root(&root).unwrap();
            assert_eq!(bought.balance, start.balance - charged, "{mode:?}");
            assert_eq!(i64::from(sheet_row(&root, "cpr.cash").unwrap()), bought.balance, "{mode:?}: the sheet's Cash");
            assert_eq!(weapon_names(&root).len(), weapons + 1, "{mode:?}: the weapon box");
            assert_eq!(bought.held.len(), start.held.len() + 1, "{mode:?}");

            let copy = bought.held.iter().rev().find(|c| c.name == "Heavy Pistol").unwrap();
            assert_eq!(moved(market_sell_at_root(&root, &market_request(&copy.key, mode)).unwrap()), returned, "{mode:?}");
            let sold = market_state_at_root(&root).unwrap();
            assert_eq!(sold.balance, start.balance - charged + returned, "{mode:?}");
            assert_eq!(weapon_names(&root).len(), weapons, "{mode:?}");
            assert_eq!(sold.held.len(), start.held.len(), "{mode:?}");
            std::fs::remove_dir_all(root.parent().unwrap()).ok();
        }
    }

    #[test]
    fn market_shell_a_buy_over_the_cash_is_refused_in_the_paid_modes() {
        let key = "armor|Metalgear|Body"; // 5,000eb, p351.
        let root = market_root("market-refused", "Jax Moreno");
        let start = market_state_at_root(&root).unwrap();
        assert!(start.balance < 5000, "{}", start.balance);
        let choices = CprCharacterStore::load_choices(&root).unwrap();
        for mode in [PriceMode::Standard, PriceMode::CharacterBuild] {
            let outcome = market_buy_at_root(&root, &market_request(key, mode)).unwrap();
            assert!(matches!(outcome, MarketShellOutcome::Blocked { .. }), "{mode:?}: {outcome:?}");
        }
        assert_eq!(CprCharacterStore::load_choices(&root).unwrap(), choices, "a refused buy saves nothing");
        assert_eq!(market_state_at_root(&root).unwrap(), start);

        assert_eq!(moved(market_buy_at_root(&root, &market_request(key, PriceMode::Cashless)).unwrap()), 0);
        assert_eq!(market_state_at_root(&root).unwrap().balance, start.balance);
        assert!(CprCharacterStore::load_choices(&root).unwrap().armor.iter().any(|a| a.name == "Metalgear"));
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    /// Armour reaches its location's SP row, a shield its HP row, ammunition its count row; a
    /// bought-and-sold piece leaves each as it was.
    #[test]
    fn market_shell_what_is_bought_reaches_the_sheet_and_leaves_with_the_sale() {
        let root = market_root("market-sheet", "Nyx Calder");
        let before = CprCharacterStore::load_choices(&root).unwrap();
        for key in ["armor|Metalgear|Head", "armor|Bulletproof Shield|", "ammo|Basic Ammunition|Rifle", "gear|Agent|"] {
            moved(market_buy_at_root(&root, &market_request(key, PriceMode::Cashless)).unwrap());
        }
        assert_eq!(sheet_row(&root, "cpr.armor.sp_head"), Some(18), "Metalgear SP18");
        assert_eq!(sheet_row(&root, "cpr.armor.shield_hp"), Some(10), "Bulletproof Shield 10 HP");
        let held = market_state_at_root(&root).unwrap().held;
        assert!(held.iter().any(|c| c.name == "Agent"), "gear is held, though no sheet box prints it");
        let again = market_buy_at_root(&root, &market_request("armor|Bulletproof Shield|", PriceMode::Cashless)).unwrap();
        assert!(matches!(again, MarketShellOutcome::Blocked { .. }), "one shield: {again:?}");

        for name in ["Metalgear Head Armor (SP18)", "Bulletproof Shield", "Agent"] {
            let copy = market_state_at_root(&root).unwrap().held.into_iter().rev().find(|c| c.name == name).unwrap_or_else(|| panic!("{name}"));
            moved(market_sell_at_root(&root, &market_request(&copy.key, PriceMode::Cashless)).unwrap());
        }
        let ammo = market_state_at_root(&root).unwrap().held.into_iter().rev().find(|c| c.name.starts_with("Basic Rifle Ammunition")).unwrap();
        moved(market_sell_at_root(&root, &market_request(&ammo.key, PriceMode::Cashless)).unwrap());
        assert_eq!(CprCharacterStore::load_choices(&root).unwrap(), before, "every sale undid its buy on the sheet");
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    /// Ammunition sells ten rounds at a time, at the price of ten; a remainder pays its share.
    #[test]
    fn market_shell_ammunition_sells_by_the_ten() {
        let root = market_root("market-ammo", "Nyx Calder");
        let name = |held: &[crate::sf_catalog::MarketShellCopyDto]| held.iter().find(|c| c.name.starts_with("Basic H Pistol Ammunition")).unwrap().clone();
        let state = market_state_at_root(&root).unwrap();
        let copy = name(&state.held);
        assert_eq!(copy.name, "Basic H Pistol Ammunition (10 of 50)");
        assert_eq!(moved(market_sell_at_root(&root, &market_request(&copy.key, PriceMode::CharacterBuild)).unwrap()), 10, "10eb per ten, p346");
        let stack = |root: &Path| CprCharacterStore::load_choices(root).unwrap().ammunition.iter().find(|(n, _)| n == "Basic H Pistol Ammunition").map(|(_, c)| *c);
        assert_eq!(stack(&root), Some(40));
        for _ in 0..4 {
            let copy = name(&market_state_at_root(&root).unwrap().held);
            moved(market_sell_at_root(&root, &market_request(&copy.key, PriceMode::Standard)).unwrap());
        }
        assert_eq!(stack(&root), None, "the stack is gone");
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    /// Everything a Role's fixed kit gives, other than cyberware and the Netrunner's deck, can be
    /// sold: its printed name is the name the shop gives the same item.
    #[test]
    fn market_shell_every_fixed_kit_item_has_a_price() {
        let corpus = corpus_root();
        let catalog = CprKitCatalog::load(&corpus).unwrap();
        let rows = crate::cpr_catalog::market_rows(&catalog).unwrap();
        let mut unpriced = std::collections::BTreeSet::new();
        for role in codex::rules_core::cpr::roles::CprRole::ALL {
            for line in catalog.fixed_lines(role).unwrap().iter().filter(|l| l.section != "cyberware") {
                for option in &line.options {
                    if !rows.iter().any(|r| r.item.name == option.name) {
                        unpriced.insert(option.name.clone());
                    }
                }
            }
        }
        // 7 netrunning items, not on the Market; 16 starter names the shop does not use.
        let expected: std::collections::BTreeSet<String> = [
            "Cyberdeck (7 Slots)", "Program: Armor", "Program: Eraser", "Program: See Ya", "Program: Sword", "Program: Vrizzbolt", "Program: Worm",
            "Disposable Cellphone", "Disposable Phone", "Electric Guitar", "Generic Chic Contacts", "Generic Chic Tops", "Inflatable Bed & Sleep-Bag",
            "Nomad Leathers Bottom", "Nomad Leathers Hat", "Pocket Amp", "Rope", "Tent and Camping Equipment", "Urbanflash Bottoms", "Urbanflash Footwear",
            "Urbanflash Jacket", "Urbanflash Mirrorshades", "Urbanflash Top",
        ].into_iter().map(str::to_owned).collect();
        assert_eq!(unpriced, expected);
    }

    /// A starter's item the shop has no name for sells for nothing and says so; a Netrunner's deck
    /// and programs are not on the Sell list.
    #[test]
    fn market_shell_a_starter_item_with_no_shop_name_sells_for_nothing() {
        // A starter kit prints this; the shop lists `Rope (60m/yds)`.
        let root = market_root("market-unpriced", "Nyx Calder");
        let mut kit = CprCharacterStore::load_kit(&root).unwrap();
        kit.items.push(CprKitEntry { section: "outfit".to_owned(), name: "Rope".to_owned(), quantity: 1 });
        CprCharacterStore::save_kit(&kit, &root).unwrap();
        let state = market_state_at_root(&root).unwrap();
        assert!(state.held.iter().all(|c| !c.name.starts_with("Program: ") && !c.name.starts_with("Cyberdeck")), "{:?}", state.held);
        let names = CprCharacterStore::load_choices(&root).unwrap().deck.expect("Nyx holds a deck").programs;
        assert!(state.held.iter().all(|c| !names.contains(&c.name)), "a bought program is the deck's: {:?}", state.held);
        let rope = state.held.iter().find(|c| c.name == "Rope").expect("held");
        assert_eq!(moved(market_sell_at_root(&root, &market_request(&rope.key, PriceMode::CharacterBuild)).unwrap()), 0);
        let after = market_state_at_root(&root).unwrap();
        assert_eq!(after.balance, state.balance);
        assert_eq!(after.held.len(), state.held.len() - 1);
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    #[test]
    fn market_shell_refuses_a_row_not_on_offer_and_a_copy_not_held() {
        let root = market_root("market-keys", "Jax Moreno");
        for key in ["cyberware|Cybereye|", "weapon|No Such Gun|", "nonsense"] {
            let outcome = market_buy_at_root(&root, &market_request(key, PriceMode::Cashless)).unwrap();
            assert!(matches!(outcome, MarketShellOutcome::Blocked { .. }), "{key}: {outcome:?}");
        }
        for key in ["999|Heavy Pistol", "nonsense", "0|Not What Is There"] {
            let outcome = market_sell_at_root(&root, &market_request(key, PriceMode::Standard)).unwrap();
            assert!(matches!(outcome, MarketShellOutcome::Blocked { .. }), "{key}: {outcome:?}");
        }
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    /// Every row the Market sells can be sold back, whatever else in the shop shares its name: 10
    /// goods and the Dartgun share one with cyberware, and read as installed.
    #[test]
    fn market_shell_every_row_bought_is_on_the_sell_list() {
        let catalog = CprKitCatalog::load(&corpus_root()).unwrap();
        let rows = crate::cpr_catalog::market_rows(&catalog).unwrap();
        let root = market_root("market-every-row", "Jax Moreno");
        let cash = market_state_at_root(&root).unwrap().balance;
        let mut not_on_the_list = Vec::new();
        for row in &rows {
            let held_before = market_state_at_root(&root).unwrap().held.len();
            moved(market_buy_at_root(&root, &market_request(&row.row.key, PriceMode::Cashless)).unwrap());
            let state = market_state_at_root(&root).unwrap();
            let copy = state.held.iter().rev().find(|c| c.key.split_once('|').is_some_and(|(_, name)| name == row.item.name));
            match copy {
                Some(copy) if state.held.len() > held_before || row.unit > 1 => {
                    moved(market_sell_at_root(&root, &market_request(&copy.key, PriceMode::Cashless)).unwrap());
                }
                _ => not_on_the_list.push(row.row.key.clone()),
            }
        }
        println!("market rows bought and sold back: {} of {}", rows.len() - not_on_the_list.len(), rows.len());
        assert!(rows.len() > 200, "{}", rows.len());
        assert!(not_on_the_list.is_empty(), "{}: {not_on_the_list:#?}", not_on_the_list.len());
        assert_eq!(market_state_at_root(&root).unwrap().balance, cash, "Cashless moved no money");
        std::fs::remove_dir_all(&root).ok();
    }

    /// A deck takes programs and hardware on a deck with a slot count, which the Market does not
    /// keep, so it sells none, under whichever kind the shop files it: three `gear` records were
    /// listed, charged and reached no sheet.
    #[test]
    fn market_shell_lists_no_cyberdeck() {
        let catalog = CprKitCatalog::load(&corpus_root()).unwrap();
        let decks: Vec<String> = crate::cpr_catalog::market_rows(&catalog)
            .unwrap()
            .into_iter()
            .filter(|row| row.item.name.starts_with("Cyberdeck ("))
            .map(|row| row.row.key)
            .collect();
        assert!(decks.is_empty(), "{decks:#?}");
    }

    /// An installed item stays off the Sell list on every starter, even with a shop name of its own.
    #[test]
    fn market_shell_a_starter_installed_item_is_not_on_the_sell_list() {
        let (_, requests): (Vec<_>, Vec<_>) = starter_requests("t").into_iter().unzip();
        let mut cyberware = 0;
        for request in requests {
            let root = market_root("market-installed", &request.display_label);
            let kit = CprCharacterStore::load_kit(&root).unwrap();
            let held = market_state_at_root(&root).unwrap().held;
            for (index, entry) in kit.items.iter().enumerate() {
                if entry.section == "cyberware" {
                    cyberware += 1;
                    let key = format!("{index}|{}", entry.name);
                    assert!(!held.iter().any(|c| c.key == key), "{} sells {}", request.display_label, entry.name);
                }
            }
            std::fs::remove_dir_all(&root).ok();
        }
        assert!(cyberware > 0, "the starters hold cyberware, or this proves nothing");
    }
}
