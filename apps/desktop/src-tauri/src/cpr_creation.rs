//! Cyberpunk RED creation commands: preview and create a character's STATs by the book's three
//! methods (Cyberpunk RED core rulebook p. 74-79).
//!
//! - **Streetrat** (p. 74): the Role's template row, rolled on 1d10 or picked; the whole row is
//!   copied ("You may not move your STATs around").
//! - **Edgerunner** (p. 78): one 1d10 per STAT, INT to EMP, "comparing the roll for that STAT
//!   against the value on the column for that STAT" of the Role's template table.
//! - **Complete Package** (p. 79): the player divides the default STAT points (62, the
//!   `complete_package_points` record marked `default`) between the ten STATs; "no STAT may be
//!   higher than 8 or lower than 2".
//!
//! The dice come from one seedable roller, [`CprRoller`]: a preview answers with the seed it
//! used, and a create sent with that seed draws the same dice, so the saved character has the
//! previewed STATs. Templates, Roles and the STAT-point total are read from the Cyberpunk RED
//! corpus (packaged-root aware). The STAT and derived numbers are the engine's `cpr.` rows
//! (`codex::rules_core::cpr::{sheet, stats}`); nothing here computes a sheet number.
//!
//! Lifepath, skills, Role Ability sub-choices, kit and cyberware extend these payloads later.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use codex::rules_core::cpr::input::{CprCharacterInput, CprStat, CprStats};
use codex::rules_core::cpr::sheet::stat_rows;
use codex::rules_core::cpr::stats::{derived_rows, humanity_max};
use codex::rules_core::cpr::store::{CprCharacterStore, CprSavedCharacter, CPR_SAVED_CHARACTER_SCHEMA_VERSION};
use codex::rules_core::game_system::{runtime_repo_root, GameSystem};

use crate::character_hub::{CharacterSummaryDto, DiagnosticDto, ExplanationDto};

/// The Complete Package allocation does not spend exactly the STAT points.
pub(crate) const REFUSED_STAT_POINTS: &str = "cpr_creation.stat_points";
/// A Complete Package STAT is above 8 or below 2.
pub(crate) const REFUSED_STAT_RANGE: &str = "cpr_creation.stat_range";
/// A Complete Package allocation leaves a STAT out.
pub(crate) const REFUSED_STAT_MISSING: &str = "cpr_creation.stat_missing";
/// A Complete Package allocation names a STAT the book does not have.
pub(crate) const REFUSED_STAT_UNKNOWN: &str = "cpr_creation.stat_unknown";
/// Create without a character name.
pub(crate) const REFUSED_NO_NAME: &str = "cpr_creation.no_name";
/// Create over a character that is already saved under that id.
pub(crate) const REFUSED_EXISTS: &str = "cpr_creation.character_exists";

/// p. 79: "no STAT may be higher than 8 or lower than 2".
const STAT_MIN: u8 = 2;
const STAT_MAX: u8 = 8;

/// One seedable dice roller for every creation-time table roll. SplitMix64 over a 32-bit seed
/// (a seed survives the JSON round trip to the frontend exactly); faces are drawn by rejection,
/// so every face of a die is equally likely.
#[derive(Debug, Clone)]
pub(crate) struct CprRoller {
    state: u64,
}

impl CprRoller {
    pub(crate) fn new(seed: u32) -> Self {
        Self { state: u64::from(seed) }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// One die of `sides` faces: 1..=sides.
    pub(crate) fn roll(&mut self, sides: u8) -> u8 {
        let sides64 = u64::from(sides.max(1));
        let zone = u64::MAX - (u64::MAX % sides64);
        loop {
            let draw = self.next_u64();
            if draw < zone {
                return (draw % sides64) as u8 + 1;
            }
        }
    }
}

/// A seed for a request that did not send one.
fn fresh_seed() -> u32 {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or_default();
    (nanos ^ (nanos >> 32) ^ u128::from(std::process::id())) as u32
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CprCreationMethod {
    /// Template row (p. 74).
    Streetrat,
    /// Per-STAT template rolls (p. 78).
    Edgerunner,
    /// STAT-point allocation (p. 79).
    CompletePackage,
}

/// The STAT step of Cyberpunk RED creation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprCreationRequest {
    /// A `role` record name, e.g. `Solo`.
    pub role: String,
    pub method: CprCreationMethod,
    /// The roller's seed; absent, a fresh seed is drawn and returned.
    #[serde(default)]
    pub seed: Option<u32>,
    /// Streetrat only: the picked template row (1-10); absent, the row is rolled on 1d10.
    #[serde(default)]
    pub template_roll: Option<u8>,
    /// Complete Package only: the allocation, keyed by STAT id (`int`, `ref`, ... `emp`).
    #[serde(default)]
    pub stats: Option<BTreeMap<String, u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprCreateRequest {
    pub character_id: String,
    pub display_label: String,
    pub saved_at: String,
    pub creation: CprCreationRequest,
}

/// One die the roller drew.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprRollDto {
    /// `streetrat_row`, or `stat.<id>` for an Edgerunner roll.
    pub table: String,
    pub die: String,
    pub result: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprStatValueDto {
    pub id: String,
    pub label: String,
    pub value: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprCreationPreview {
    pub role: String,
    pub method: CprCreationMethod,
    /// The seed the roller used; send it back on create to keep these dice.
    pub seed: u32,
    pub rolls: Vec<CprRollDto>,
    /// Streetrat: the template row used.
    pub template_roll: Option<u8>,
    /// The ten STATs, book order (p. 72).
    pub stats: Vec<CprStatValueDto>,
    /// Complete Package: the points the allocation spends and the points to spend.
    pub stat_points_spent: Option<u16>,
    pub stat_points_total: Option<u16>,
    /// The engine's `cpr.stat.*` and `cpr.derived.*` rows for these STATs (empty while a
    /// claim-blocking problem stands).
    pub explanations: Vec<ExplanationDto>,
    /// Claim-blocking reasons create would refuse.
    pub problems: Vec<DiagnosticDto>,
}

/// The created character, or why it was not created (nothing saved).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum CprCreateResponse {
    Saved {
        summary: Box<CharacterSummaryDto>,
        seed: u32,
        /// The engine's `cpr.` rows of the saved character.
        explanations: Vec<ExplanationDto>,
    },
    Blocked { diagnostics: Vec<DiagnosticDto> },
}

fn problem(id: &str, message: impl Into<String>) -> DiagnosticDto {
    DiagnosticDto { id: id.to_owned(), message: message.into(), claim_blocking: true }
}

fn corpus_root() -> PathBuf {
    GameSystem::CyberpunkRed.package_roots(&runtime_repo_root()).corpus
}

fn read_records(root: &Path, file: &str) -> Result<Vec<Value>, String> {
    let path = root.join(file);
    let text = std::fs::read_to_string(&path).map_err(|e| format!("cpr_creation.read_failed: {}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("cpr_creation.parse_failed: {}: {e}", path.display()))
}

/// The Role's ten template rows (roll 1 first), each the ten STATs in book column order.
fn role_templates(root: &Path, role: &str) -> Result<Vec<[u8; 10]>, String> {
    let roles = read_records(root, "role.json")?;
    if !roles.iter().any(|r| r["name"].as_str() == Some(role)) {
        return Err(format!("cpr_creation.unknown_role: no Cyberpunk RED Role named '{role}'"));
    }
    let mut rows: Vec<Option<[u8; 10]>> = vec![None; 10];
    for record in read_records(root, "streetrat_template.json")?.iter().filter(|r| r["role"].as_str() == Some(role)) {
        let roll = record["roll"].as_u64().filter(|n| (1..=10).contains(n)).ok_or_else(|| {
            format!("cpr_creation.corpus_malformed: a {role} template row has no roll 1-10")
        })?;
        let mut stats = [0u8; 10];
        for (slot, stat) in stats.iter_mut().zip(CprStat::ALL) {
            *slot = record[stat.label()]
                .as_u64()
                .and_then(|v| u8::try_from(v).ok())
                .ok_or_else(|| format!("cpr_creation.corpus_malformed: {role} roll {roll} has no {}", stat.label()))?;
        }
        if rows[roll as usize - 1].replace(stats).is_some() {
            return Err(format!("cpr_creation.corpus_malformed: {role} roll {roll} appears twice"));
        }
    }
    rows.into_iter()
        .enumerate()
        .map(|(i, r)| r.ok_or_else(|| format!("cpr_creation.corpus_malformed: {role} has no template roll {}", i + 1)))
        .collect()
}

/// The default Complete Package STAT points (p. 79: "normally 62").
fn complete_package_points(root: &Path) -> Result<u16, String> {
    read_records(root, "complete_package_points.json")?
        .iter()
        .find(|r| r["default"].as_bool() == Some(true))
        .and_then(|r| r["stat_points"].as_u64())
        .and_then(|v| u16::try_from(v).ok())
        .ok_or_else(|| "cpr_creation.corpus_malformed: no default complete_package_points record".to_owned())
}

/// Edgerunner (p. 78): STAT `i` is column `i` of the row its own roll names.
pub(crate) fn edgerunner_stats(templates: &[[u8; 10]], rolls: &[u8; 10]) -> [u8; 10] {
    std::array::from_fn(|i| templates[usize::from(rolls[i]) - 1][i])
}

fn input_of(values: [u8; 10]) -> CprCharacterInput {
    let mut stats = CprStats::default();
    for (stat, value) in CprStat::ALL.into_iter().zip(values) {
        stats.set(stat, value);
    }
    CprCharacterInput { stats }
}

/// The engine's rows for a character with these STATs and no cyberware yet (Humanity = EMP x 10).
fn engine_rows(input: &CprCharacterInput) -> Vec<ExplanationDto> {
    let humanity = humanity_max(input.stats.get(CprStat::Emp));
    stat_rows(input)
        .into_iter()
        .chain(derived_rows(input, humanity))
        .map(|e| ExplanationDto { id: e.id, value: e.value, detail: e.detail })
        .collect()
}

/// The STAT step over the corpus at `root`. An unknown Role or a template row outside 1-10 is
/// an error; an allocation create would refuse comes back as `problems`.
pub(crate) fn preview_at(root: &Path, request: &CprCreationRequest) -> Result<CprCreationPreview, String> {
    let seed = request.seed.unwrap_or_else(fresh_seed);
    let mut roller = CprRoller::new(seed);
    let mut rolls = Vec::new();
    let mut problems = Vec::new();
    let mut template_roll = None;
    let mut points = (None, None);

    let values: [u8; 10] = match request.method {
        CprCreationMethod::Streetrat => {
            let templates = role_templates(root, &request.role)?;
            let roll = match request.template_roll {
                Some(picked) if (1..=10).contains(&picked) => picked,
                Some(picked) => {
                    return Err(format!("cpr_creation.template_roll: a template row is 1 to 10, not {picked}"))
                }
                None => {
                    let result = roller.roll(10);
                    rolls.push(CprRollDto { table: "streetrat_row".to_owned(), die: "1d10".to_owned(), result });
                    result
                }
            };
            template_roll = Some(roll);
            templates[usize::from(roll) - 1]
        }
        CprCreationMethod::Edgerunner => {
            let templates = role_templates(root, &request.role)?;
            let dice: [u8; 10] = std::array::from_fn(|_| roller.roll(10));
            for (stat, result) in CprStat::ALL.into_iter().zip(dice) {
                rolls.push(CprRollDto { table: format!("stat.{}", stat.id()), die: "1d10".to_owned(), result });
            }
            edgerunner_stats(&templates, &dice)
        }
        CprCreationMethod::CompletePackage => {
            let roles = read_records(root, "role.json")?;
            if !roles.iter().any(|r| r["name"].as_str() == Some(request.role.as_str())) {
                return Err(format!("cpr_creation.unknown_role: no Cyberpunk RED Role named '{}'", request.role));
            }
            let total = complete_package_points(root)?;
            let empty = BTreeMap::new();
            let allocation = request.stats.as_ref().unwrap_or(&empty);
            for key in allocation.keys().filter(|k| !CprStat::ALL.iter().any(|s| s.id() == k.as_str())) {
                problems.push(problem(REFUSED_STAT_UNKNOWN, format!("'{key}' is not a Cyberpunk RED STAT")));
            }
            let mut values = [0u8; 10];
            for (slot, stat) in values.iter_mut().zip(CprStat::ALL) {
                match allocation.get(stat.id()) {
                    Some(&value) => {
                        if !(STAT_MIN..=STAT_MAX).contains(&value) {
                            problems.push(problem(
                                REFUSED_STAT_RANGE,
                                format!("{} {value}: no STAT may be higher than 8 or lower than 2 (p. 79)", stat.label()),
                            ));
                        }
                        *slot = value;
                    }
                    None => problems.push(problem(REFUSED_STAT_MISSING, format!("give {} a value from 2 to 8", stat.label()))),
                }
            }
            let spent: u16 = values.iter().map(|v| u16::from(*v)).sum();
            if spent != total {
                problems.push(problem(
                    REFUSED_STAT_POINTS,
                    format!("the STATs spend {spent} points; a Complete Package spends exactly {total} (p. 79)"),
                ));
            }
            points = (Some(spent), Some(total));
            values
        }
    };

    let input = input_of(values);
    Ok(CprCreationPreview {
        role: request.role.clone(),
        method: request.method,
        seed,
        rolls,
        template_roll,
        stats: CprStat::ALL
            .into_iter()
            .zip(values)
            .map(|(s, value)| CprStatValueDto { id: s.id().to_owned(), label: s.label().to_owned(), value })
            .collect(),
        stat_points_spent: points.0,
        stat_points_total: points.1,
        explanations: if problems.is_empty() { engine_rows(&input) } else { Vec::new() },
        problems,
    })
}

/// Creates the character at `root` from the corpus at `corpus`: the same STAT step as the
/// preview (same seed, same dice), saved through the CPR store and read back through the
/// envelope router. Nothing is saved when anything is refused.
pub(crate) fn create_at_root(
    corpus: &Path,
    root: &Path,
    request: &CprCreateRequest,
    app_version: String,
) -> Result<CprCreateResponse, String> {
    let mut creation = request.creation.clone();
    let preview = preview_at(corpus, &creation)?;
    creation.seed = Some(preview.seed);
    let mut diagnostics = preview.problems;
    if request.display_label.trim().is_empty() {
        diagnostics.push(problem(REFUSED_NO_NAME, "enter the character's name"));
    }
    if root.join("envelope.txt").exists() {
        diagnostics.push(problem(REFUSED_EXISTS, format!("a character is already saved as '{}'", request.character_id)));
    }
    if !diagnostics.is_empty() {
        return Ok(CprCreateResponse::Blocked { diagnostics });
    }
    let values: [u8; 10] = std::array::from_fn(|i| preview.stats[i].value);
    let input = input_of(values);
    let character = CprSavedCharacter {
        character_id: request.character_id.clone(),
        revision_id: format!("{}.rev.1", request.character_id),
        saved_at: request.saved_at.clone(),
        schema_version: CPR_SAVED_CHARACTER_SCHEMA_VERSION,
        app_or_runtime_version: app_version,
        display_label: request.display_label.trim().to_owned(),
        input: input.clone(),
    };
    CprCharacterStore::save(&character, root)?;
    let loaded = crate::rule_system_adapter::load_saved_character_via_envelope(root)?;
    Ok(CprCreateResponse::Saved {
        summary: Box::new(loaded.summary),
        seed: preview.seed,
        explanations: engine_rows(&input),
    })
}

/// Preview the STAT step of a Cyberpunk RED character (no save).
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn preview_cyberpunk_character(request: CprCreationRequest) -> Result<CprCreationPreview, String> {
    preview_at(&corpus_root(), &request)
}

/// Create and save a Cyberpunk RED character.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn create_cyberpunk_character(app: tauri::AppHandle, request: CprCreateRequest) -> Result<CprCreateResponse, String> {
    let root = crate::character_hub::resolve_character_root(&app, &request.character_id)?;
    create_at_root(&corpus_root(), &root, &request, app.package_info().version.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    use serde_json::Value;

    fn root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(GameSystem::CyberpunkRed.corpus_relative())
    }

    fn tempdir(label: &str) -> PathBuf {
        let unique = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("time").as_nanos();
        let path = std::env::temp_dir().join(format!("codex-cpr-creation-{label}-{}-{unique}", std::process::id()));
        std::fs::create_dir_all(&path).expect("temp dir");
        path
    }

    fn request(role: &str, method: CprCreationMethod) -> CprCreationRequest {
        CprCreationRequest { role: role.to_owned(), method, seed: None, template_roll: None, stats: None }
    }

    fn create(request: CprCreationRequest, id: &str) -> CprCreateRequest {
        CprCreateRequest {
            character_id: id.to_owned(),
            display_label: format!("Created {id}"),
            saved_at: "2026-10-14T00:00:00Z".to_owned(),
            creation: request,
        }
    }

    fn values(preview: &CprCreationPreview) -> Vec<u8> {
        preview.stats.iter().map(|s| s.value).collect()
    }

    fn row(rows: &[ExplanationDto], id: &str) -> i16 {
        rows.iter().find(|r| r.id == id).unwrap_or_else(|| panic!("no row {id}")).value
    }

    const IDS: [&str; 10] = ["int", "ref", "dex", "tech", "cool", "will", "luck", "move", "body", "emp"];

    fn allocation(values: [u8; 10]) -> BTreeMap<String, u8> {
        IDS.iter().zip(values).map(|(id, v)| ((*id).to_owned(), v)).collect()
    }

    /// Every `streetrat_template` record, read as plain JSON (role, roll, the ten STATs in book
    /// column order).
    fn template_records() -> Vec<(String, u8, [u8; 10])> {
        let rows: Vec<Value> =
            serde_json::from_str(&std::fs::read_to_string(root().join("streetrat_template.json")).unwrap()).unwrap();
        rows.iter()
            .map(|r| {
                let mut stats = [0u8; 10];
                for (i, label) in ["INT", "REF", "DEX", "TECH", "COOL", "WILL", "LUCK", "MOVE", "BODY", "EMP"].iter().enumerate() {
                    stats[i] = r[*label].as_u64().unwrap() as u8;
                }
                (r["role"].as_str().unwrap().to_owned(), r["roll"].as_u64().unwrap() as u8, stats)
            })
            .collect()
    }

    /// Streetrat by pick: each of the 100 template rows previews as its STATs, saves, and loads
    /// back through the envelope router with the same ten `cpr.stat.*` rows.
    #[test]
    fn every_template_row_round_trips_through_preview_create_and_load() {
        let records = template_records();
        assert_eq!(records.len(), 100);
        let characters_root = tempdir("templates");
        let mut round_tripped = 0;
        for (role, roll, stats) in &records {
            let mut req = request(role, CprCreationMethod::Streetrat);
            req.template_roll = Some(*roll);
            let preview = preview_at(&root(), &req).unwrap_or_else(|e| panic!("{role} {roll}: {e}"));
            assert!(preview.problems.is_empty(), "{role} {roll}: {:?}", preview.problems);
            assert_eq!(values(&preview), stats.to_vec(), "{role} roll {roll}");
            assert_eq!(preview.template_roll, Some(*roll));
            assert!(preview.rolls.is_empty(), "a picked row draws no dice");

            let id = format!("cpr-{}-{roll}", role.to_lowercase());
            let dir = characters_root.join(&id);
            let saved = create_at_root(&root(), &dir, &create(req, &id), "codex-test".to_owned()).unwrap();
            let CprCreateResponse::Saved { summary, .. } = saved else { panic!("{role} {roll}: {saved:?}") };
            assert_eq!(summary.game_system, "cyberpunk-red");
            let loaded = crate::rule_system_adapter::load_saved_character_via_envelope(&dir).unwrap();
            let loaded_stats: Vec<i16> =
                IDS.iter().map(|id| row(&loaded.explanations, &format!("cpr.stat.{id}"))).collect();
            assert_eq!(loaded_stats, stats.iter().map(|v| i16::from(*v)).collect::<Vec<_>>(), "{role} {roll}");
            round_tripped += 1;
        }
        assert_eq!(round_tripped, 100);
        std::fs::remove_dir_all(&characters_root).ok();
    }

    #[test]
    fn a_rolled_streetrat_row_is_one_d10_and_that_whole_row() {
        let records = template_records();
        for seed in 0..50u32 {
            let mut req = request("Solo", CprCreationMethod::Streetrat);
            req.seed = Some(seed);
            let preview = preview_at(&root(), &req).unwrap();
            assert_eq!(preview.seed, seed);
            assert_eq!(preview.rolls.len(), 1);
            let roll = preview.rolls[0].result;
            assert!((1..=10).contains(&roll), "{roll}");
            assert_eq!(preview.template_roll, Some(roll));
            let (_, _, stats) = records.iter().find(|(r, n, _)| r == "Solo" && *n == roll).unwrap();
            assert_eq!(values(&preview), stats.to_vec());
        }
    }

    /// Edgerunner: ten 1d10 rolls, INT to EMP, each read from its own column. The same seed
    /// reproduces the same rolls and STATs.
    #[test]
    fn seeded_edgerunner_rolls_reproduce() {
        let records = template_records();
        let medtech: Vec<[u8; 10]> = (1..=10)
            .map(|n| records.iter().find(|(r, roll, _)| r == "Medtech" && *roll == n).unwrap().2)
            .collect();
        for seed in [0u32, 1, 7, 62, 2026, u32::MAX] {
            let mut req = request("Medtech", CprCreationMethod::Edgerunner);
            req.seed = Some(seed);
            let first = preview_at(&root(), &req).unwrap();
            let second = preview_at(&root(), &req).unwrap();
            assert_eq!(first.rolls, second.rolls, "seed {seed}");
            assert_eq!(values(&first), values(&second), "seed {seed}");
            assert_eq!(first.rolls.len(), 10);
            for (i, rolled) in first.rolls.iter().enumerate() {
                assert_eq!(rolled.table, format!("stat.{}", IDS[i]));
                assert_eq!(rolled.die, "1d10");
                assert_eq!(first.stats[i].value, medtech[usize::from(rolled.result) - 1][i], "seed {seed} {}", IDS[i]);
            }
            assert_eq!(first.template_roll, None);
        }
        let mut a = request("Medtech", CprCreationMethod::Edgerunner);
        a.seed = Some(1);
        let mut b = a.clone();
        b.seed = Some(2);
        assert_ne!(preview_at(&root(), &a).unwrap().rolls, preview_at(&root(), &b).unwrap().rolls);
    }

    /// p078's worked example: rolls on the Solo table that give "6 7 7 5 7 / 6 6 7 8 5"
    /// (the page's STAT block, typed from the page, not read from the corpus).
    #[test]
    fn the_p078_edgerunner_example_reads_each_stat_from_its_own_column() {
        let solo: Vec<[u8; 10]> = (1..=10)
            .map(|n| template_records().into_iter().find(|(r, roll, _)| r == "Solo" && *roll == n).unwrap().2)
            .collect();
        let stats = edgerunner_stats(&solo, &[1, 1, 1, 5, 3, 1, 3, 3, 3, 1]);
        assert_eq!(stats, [6, 7, 7, 5, 7, 6, 6, 7, 8, 5]);
    }

    #[test]
    fn the_roller_is_a_fair_d10_and_seed_stable() {
        let mut roller = CprRoller::new(42);
        let mut counts = [0u32; 10];
        for _ in 0..10_000 {
            let r = roller.roll(10);
            assert!((1..=10).contains(&r));
            counts[usize::from(r) - 1] += 1;
        }
        assert!(counts.iter().all(|c| (800..=1200).contains(c)), "{counts:?}");
        let a: Vec<u8> = (0..20).map({ let mut r = CprRoller::new(9); move |_| r.roll(10) }).collect();
        let b: Vec<u8> = (0..20).map({ let mut r = CprRoller::new(9); move |_| r.roll(10) }).collect();
        assert_eq!(a, b);
    }

    #[test]
    fn complete_package_accepts_62_and_refuses_61_63_a_9_and_a_1() {
        // p079's worked example: 5 7 8 3 6 / 7 6 8 8 4 = 62.
        let mut req = request("Solo", CprCreationMethod::CompletePackage);
        req.stats = Some(allocation([5, 7, 8, 3, 6, 7, 6, 8, 8, 4]));
        let ok = preview_at(&root(), &req).unwrap();
        assert!(ok.problems.is_empty(), "{:?}", ok.problems);
        assert_eq!((ok.stat_points_spent, ok.stat_points_total), (Some(62), Some(62)));
        assert_eq!(values(&ok), [5, 7, 8, 3, 6, 7, 6, 8, 8, 4]);
        assert!(ok.rolls.is_empty());

        let characters_root = tempdir("cp");
        let refused = [
            ("61", [5, 7, 8, 3, 6, 7, 6, 8, 8, 3], REFUSED_STAT_POINTS),
            ("63", [5, 7, 8, 3, 6, 7, 6, 8, 8, 5], REFUSED_STAT_POINTS),
            ("a 9", [9, 7, 8, 3, 6, 7, 6, 8, 4, 4], REFUSED_STAT_RANGE),
            ("a 1", [1, 7, 8, 3, 6, 7, 6, 8, 8, 8], REFUSED_STAT_RANGE),
        ];
        for (label, stats, id) in refused {
            let mut req = request("Solo", CprCreationMethod::CompletePackage);
            req.stats = Some(allocation(stats));
            let preview = preview_at(&root(), &req).unwrap();
            assert!(preview.problems.iter().any(|p| p.id == id && p.claim_blocking), "{label}: {:?}", preview.problems);
            let dir = characters_root.join(format!("cp-{}", label.replace(' ', "-")));
            let response = create_at_root(&root(), &dir, &create(req, "cp"), "codex-test".to_owned()).unwrap();
            assert!(
                matches!(&response, CprCreateResponse::Blocked { diagnostics } if diagnostics.iter().any(|d| d.id == id)),
                "{label}: {response:?}"
            );
            assert!(!dir.join("envelope.txt").exists(), "{label}: nothing saved");
        }
        std::fs::remove_dir_all(&characters_root).ok();
    }

    #[test]
    fn complete_package_refuses_a_missing_or_unknown_stat() {
        let mut req = request("Solo", CprCreationMethod::CompletePackage);
        let mut stats = allocation([6, 6, 6, 6, 6, 6, 6, 6, 6, 8]);
        stats.remove("emp");
        stats.insert("str".to_owned(), 8);
        req.stats = Some(stats);
        let preview = preview_at(&root(), &req).unwrap();
        let ids: Vec<&str> = preview.problems.iter().map(|p| p.id.as_str()).collect();
        assert!(ids.contains(&REFUSED_STAT_MISSING) && ids.contains(&REFUSED_STAT_UNKNOWN), "{ids:?}");
        req.stats = None;
        assert!(preview_at(&root(), &req).unwrap().problems.iter().any(|p| p.id == REFUSED_STAT_MISSING));
    }

    #[test]
    fn an_unknown_role_and_an_out_of_range_template_roll_are_refused() {
        assert!(preview_at(&root(), &request("Gunslinger", CprCreationMethod::Streetrat)).unwrap_err().contains("Gunslinger"));
        for bad in [0u8, 11] {
            let mut req = request("Solo", CprCreationMethod::Streetrat);
            req.template_roll = Some(bad);
            assert!(preview_at(&root(), &req).unwrap_err().contains("1 to 10"), "{bad}");
        }
    }

    /// The preview carries the engine's STAT and derived rows: a 62-point allocation with
    /// BODY 8, WILL 6 gives HP 10 + 5 x 7 = 45 (p080 table row WILL 6, column BODY 8), EMP 5
    /// gives Humanity 50.
    #[test]
    fn the_preview_carries_the_engines_stat_and_derived_rows() {
        let mut req = request("Solo", CprCreationMethod::CompletePackage);
        req.stats = Some(allocation([6, 7, 7, 5, 7, 6, 6, 5, 8, 5]));
        assert_eq!(req.stats.as_ref().unwrap().values().map(|v| u16::from(*v)).sum::<u16>(), 62);
        let preview = preview_at(&root(), &req).unwrap();
        let rows = &preview.explanations;
        assert!(rows.iter().all(|r| r.id.starts_with("cpr.")));
        assert_eq!(row(rows, "cpr.stat.body"), 8);
        assert_eq!(row(rows, "cpr.derived.hit_points"), 45);
        assert_eq!(row(rows, "cpr.derived.seriously_wounded"), 23);
        assert_eq!(row(rows, "cpr.derived.death_save"), 8);
        assert_eq!(row(rows, "cpr.derived.humanity_max"), 50);
        assert_eq!(row(rows, "cpr.derived.emp_current"), 5);
    }

    #[test]
    fn create_refuses_an_empty_name_and_an_existing_character() {
        let characters_root = tempdir("refusals");
        let mut req = request("Solo", CprCreationMethod::Streetrat);
        req.template_roll = Some(3);
        let mut unnamed = create(req.clone(), "cpr-unnamed");
        unnamed.display_label = "  ".to_owned();
        let dir = characters_root.join("cpr-unnamed");
        let response = create_at_root(&root(), &dir, &unnamed, "codex-test".to_owned()).unwrap();
        assert!(matches!(&response, CprCreateResponse::Blocked { diagnostics } if diagnostics.iter().any(|d| d.id == REFUSED_NO_NAME)));

        let dir = characters_root.join("cpr-twice");
        let first = create_at_root(&root(), &dir, &create(req.clone(), "cpr-twice"), "codex-test".to_owned()).unwrap();
        assert!(matches!(first, CprCreateResponse::Saved { .. }));
        let second = create_at_root(&root(), &dir, &create(req, "cpr-twice"), "codex-test".to_owned()).unwrap();
        assert!(matches!(&second, CprCreateResponse::Blocked { diagnostics } if diagnostics.iter().any(|d| d.id == REFUSED_EXISTS)));
        std::fs::remove_dir_all(&characters_root).ok();
    }

    /// The created character's response carries the seed, and creating with the preview's seed
    /// saves exactly the previewed STATs.
    #[test]
    fn create_with_the_previews_seed_saves_the_previewed_stats() {
        let mut req = request("Netrunner", CprCreationMethod::Edgerunner);
        let preview = preview_at(&root(), &req).unwrap();
        req.seed = Some(preview.seed);
        let characters_root = tempdir("seeded");
        let dir = characters_root.join("cpr-seeded");
        let response = create_at_root(&root(), &dir, &create(req, "cpr-seeded"), "codex-test".to_owned()).unwrap();
        let CprCreateResponse::Saved { seed, explanations, .. } = response else { panic!("{response:?}") };
        assert_eq!(seed, preview.seed);
        let saved: Vec<u8> = IDS.iter().map(|id| row(&explanations, &format!("cpr.stat.{id}")) as u8).collect();
        assert_eq!(saved, values(&preview));
        std::fs::remove_dir_all(&characters_root).ok();
    }
}
