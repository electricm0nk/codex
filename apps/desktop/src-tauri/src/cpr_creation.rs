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
//! **Lifepath** (p. 45-70): every general table and the chosen Role's tables (the
//! `lifepath_table` records) is rolled with the same roller or picked; a table behind a printed
//! choice (group or solo, partner or alone, land, air or sea) can be marked not taken. The
//! answers are printed text, saved with the character (`CprLifepath`, `cpr_lifepath.txt`). The one
//! number the Lifepath gives is the Cultural Origin language (p. 46): a Language skill at the
//! record's `free_language_rank` (4), saved as a skill and shown as the engine's
//! `cpr.skill_level.language.*` and `cpr.skill.language.*` rows.
//!
//! **Skills** (p. 85-90): the allocation each method allows, validated and priced by the engine
//! (`codex::rules_core::cpr::skills::validate_creation_skills`). Streetrat takes the Role's 20
//! skills at their printed levels; Edgerunner the same 20, each 2 to 6, starting from the printed
//! levels; Complete Package any of the 66, none above 6, starting from the 13 Basic skills at 2.
//! Every method spends at most 86 points ((x2) skills two a level) and adds the Cultural Origin
//! language at its 4 free levels. A `(choose 1)` line (Play Instrument, Science) takes the
//! player's named instrument or field. The points spent and left are the engine's
//! `cpr.skills.creation_points_*` rows; the saved character holds every level.
//!
//! **Role Ability** (p. 142-166): every Role starts at Rank 4, saved with the character, and
//! the rank's rows are the engine's `cpr.role.<role>.` rows. The sub-choices the rank asks for
//! are the player's (D-7, A-2): the Solo's Combat Awareness points, the Tech's Maker specialty
//! ranks, the Medtech's Medicine points and drugs, the Nomad's Family Motorpool picks and the
//! Exec's Team Member; the engine lists them (`CprRoleCatalog::choice_menu`), refuses a choice
//! the book does not allow and answers with their rows (`choice_rows`). They are saved in
//! `cpr_role_choices.txt`.
//!
//! Kit and cyberware extend these payloads later.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use codex::rules_core::cpr::input::{
    lifepath_text_ok, role_choice_text_ok, CprCharacterInput, CprLifepath, CprLifepathAnswer, CprRoleChoices, CprRolePick,
    CprSheetChoices, CprStat, CprStats,
};
use codex::rules_core::cpr::roles::{CprRole, CprRoleCatalog, CprRoleChoiceMenu};
use codex::rules_core::cpr::sheet::{compose_sheet, stat_rows, CprSheetCatalogs};
use codex::rules_core::cpr::skills::{
    slug as skill_slug, spend_rows, validate_creation_skills, CprSkillAllocation, CprSkillCatalog, CprSkillMethod,
};
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
/// A Lifepath pick, count or skip names a table this Role's Lifepath does not have.
pub(crate) const REFUSED_LIFEPATH_TABLE: &str = "cpr_creation.lifepath_table";
/// A Lifepath pick names a roll, column or entry the table does not have.
pub(crate) const REFUSED_LIFEPATH_ROLL: &str = "cpr_creation.lifepath_roll";
/// A picked count is above what the table's die can give.
pub(crate) const REFUSED_LIFEPATH_COUNT: &str = "cpr_creation.lifepath_count";
/// A table that no printed choice gates was marked not taken.
pub(crate) const REFUSED_LIFEPATH_SKIP: &str = "cpr_creation.lifepath_skip";
/// The Cultural Origin language is empty or a name the saved grammar cannot hold.
pub(crate) const REFUSED_CULTURAL_LANGUAGE: &str = "cpr_creation.cultural_language";
/// The skill allocation breaks its method's rules (the engine's refusal names the rule).
pub(crate) const REFUSED_SKILLS: &str = "cpr_creation.skills";
/// A `(choose 1)` line has no named instrument or field, or a specialty the saved grammar cannot hold.
pub(crate) const REFUSED_SKILL_SPECIALTY: &str = "cpr_creation.skill_specialty";
/// A Role Ability sub-choice breaks the book's rule (the engine's refusal names it) or names
/// something the saved grammar cannot hold.
pub(crate) const REFUSED_ROLE_ABILITY: &str = "cpr_creation.role_ability";
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
    /// The Lifepath picks; every table not picked is rolled.
    #[serde(default)]
    pub lifepath: CprLifepathRequest,
    /// The skill allocation; anything not sent starts where the method starts.
    #[serde(default)]
    pub skills: CprSkillsRequest,
    /// The Role Ability sub-choices; none sent, none made.
    #[serde(default)]
    pub role_ability: CprRoleAbilityRequest,
}

/// The player's Role Ability sub-choices at the starting rank: points by ability or specialty
/// (Combat Awareness, Maker, Medicine) and picks (Motorpool, Team Members, drugs), each pick with
/// what it goes on (`on`: the vehicle an upgrade is installed on, a Team Member's chart row).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprRoleAbilityRequest {
    #[serde(default)]
    pub points: BTreeMap<String, u8>,
    #[serde(default)]
    pub picks: Vec<CprRolePickRequest>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprRolePickRequest {
    pub name: String,
    #[serde(default)]
    pub on: Option<String>,
}

/// The player's skill choices. Streetrat and Edgerunner start from the Role's printed list and
/// key `levels` by its line (`Tactics`, `Play Instrument (choose 1)`); Complete Package starts
/// from the Basic skills at 2 and keys `levels` by skill (`Martial Arts (Karate)`), a level of 0
/// dropping the skill. A key that is the Cultural Origin language raises it above its 4 free
/// levels.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprSkillsRequest {
    #[serde(default)]
    pub levels: BTreeMap<String, u8>,
    /// The named instrument or field of each `(choose 1)` line, by line.
    #[serde(default)]
    pub specialties: BTreeMap<String, String>,
}

/// The player's Lifepath choices. Anything not chosen is rolled with the request's seed.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprLifepathRequest {
    /// Picked rows.
    #[serde(default)]
    pub picks: Vec<CprLifepathPick>,
    /// Picked counts of the "Roll 1d10 and subtract 7" tables, by table name.
    #[serde(default)]
    pub counts: BTreeMap<String, u8>,
    /// Tables behind a printed choice the player did not take, by table name.
    #[serde(default)]
    pub skips: Vec<String>,
    /// The Cultural Origin language; absent, one of the region's languages is rolled.
    #[serde(default)]
    pub cultural_language: Option<String>,
}

/// One picked row: `entry` 0 for a table answered once, 1.. for the nth friend, enemy or love
/// affair; `column` 0 unless the table is rolled "for each column"; `roll` any roll of the row
/// (a "1-2" row takes 1 or 2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprLifepathPick {
    pub table: String,
    #[serde(default)]
    pub entry: u8,
    #[serde(default)]
    pub column: u8,
    pub roll: u8,
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
    /// Every Lifepath table answered (general, then the Role's), corpus order.
    pub lifepath: Vec<CprLifepathTableDto>,
    /// The Cultural Origin language and its skill level (p. 46).
    pub cultural_language: Option<CprCulturalLanguageDto>,
    /// The skill allocation and the engine's points rows.
    pub skills: CprSkillsDto,
    /// The Role Ability at its starting rank: what it divides or picks, and the engine's rows.
    pub role_ability: Option<CprRoleAbilityDto>,
    /// The engine's `cpr.stat.*` and `cpr.derived.*` rows for these STATs, the
    /// `cpr.skill_level.*` and `cpr.skill.*` rows of every allocated skill (the Cultural Origin
    /// language among them) and the `cpr.skills.creation_points_*` rows (empty while a
    /// claim-blocking problem stands).
    pub explanations: Vec<ExplanationDto>,
    /// Claim-blocking reasons create would refuse.
    pub problems: Vec<DiagnosticDto>,
}

/// One Lifepath table's answers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprLifepathTableDto {
    pub table: String,
    /// `general` or `role`.
    pub group: String,
    pub source_page: String,
    /// The die each answer is rolled on (`1d6`, `1d10`): the table's whole printed roll range.
    pub die: String,
    pub roll_instruction: String,
    /// Count tables: how many entries, and the most the die gives.
    pub count: Option<u8>,
    pub count_max: Option<u8>,
    pub count_picked: bool,
    /// The table sits behind a printed choice and may be marked not taken.
    pub skippable: bool,
    pub skipped: bool,
    /// Rolled "for each column": one answer per column (else one per entry, the whole row).
    pub per_column: bool,
    pub answers: Vec<CprLifepathAnswerDto>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprLifepathAnswerDto {
    pub entry: u8,
    pub column_index: u8,
    pub column: String,
    pub roll: u8,
    pub picked: bool,
    /// The cell answered.
    pub text: String,
    /// The rest of the row (its other cells joined with ` / `) for a table answered a whole row
    /// at a time; empty for a per-column answer.
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprCulturalLanguageDto {
    pub name: String,
    pub picked: bool,
    /// The `Language (<name>)` skill key it is saved under.
    pub skill: String,
    /// The record's `free_language_rank`.
    pub level: u8,
}

/// The skill step: the allocation as the method built it, and the engine's points rows.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprSkillsDto {
    /// Every allocated skill, in list order (Streetrat, Edgerunner) or Basic skills first
    /// (Complete Package), the Cultural Origin language last.
    pub lines: Vec<CprSkillLineDto>,
    /// The engine's `cpr.skills.creation_points_spent` and `_remaining` rows; empty while the
    /// allocation is refused.
    pub rows: Vec<ExplanationDto>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprSkillLineDto {
    /// The key a level change is sent under (`CprSkillsRequest::levels`).
    pub line: String,
    /// The skill key saved (`Play Instrument (Guitar)`).
    pub key: String,
    pub level: u8,
    pub basic: bool,
    pub x2: bool,
    /// The Cultural Origin language (its 4 levels are free).
    pub cultural: bool,
    /// The method lets the player change this level.
    pub adjustable: bool,
    /// A `(choose 1)` line: the player names the instrument or field.
    pub choose_one: bool,
    /// The id of this skill's `cpr.skill_level.*` row (empty for a key the engine cannot resolve).
    pub level_row: String,
}

/// The Role Ability step: the rank, the engine's menu of sub-choices for it, and the rank's and
/// the choices' `cpr.role.<role>.` rows (empty while the choices are refused).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprRoleAbilityDto {
    pub ability: String,
    pub rank: u8,
    #[serde(skip_deserializing, default = "empty_menu")]
    pub menu: CprRoleChoiceMenu,
    pub rows: Vec<ExplanationDto>,
}

fn empty_menu() -> CprRoleChoiceMenu {
    CprRoleChoiceMenu { ability: String::new(), rank: 0, points: None, picks: None }
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

/// One `lifepath_table` record, the fields the panel reads.
#[derive(Debug, Clone, Deserialize)]
struct LifepathRecord {
    name: String,
    group: String,
    role: Option<String>,
    source_page: String,
    roll_instruction: String,
    columns: Vec<String>,
    rows: Vec<LifepathRow>,
    #[serde(default)]
    branch: Option<String>,
    #[serde(default)]
    free_language_rank: Option<u8>,
    #[serde(default)]
    languages_by_roll: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
struct LifepathRow {
    roll: Value,
    cells: Vec<String>,
}

impl LifepathRow {
    /// The printed roll as a range: `3` is 3-3, `"1-2"` is 1-2.
    fn range(&self) -> Option<(u8, u8)> {
        match &self.roll {
            Value::Number(n) => n.as_u64().and_then(|v| u8::try_from(v).ok()).map(|v| (v, v)),
            Value::String(text) => {
                let (low, high) = text.split_once('-')?;
                Some((low.trim().parse().ok()?, high.trim().parse().ok()?))
            }
            _ => None,
        }
    }
}

impl LifepathRecord {
    /// The die: the table's whole printed roll range (D-9: the Exec Corp table prints "1d6"
    /// over ten rows and is rolled as 1d10).
    fn sides(&self) -> Result<u8, String> {
        let mut high = 0;
        for row in &self.rows {
            let (_, h) = row.range().ok_or_else(|| format!("cpr_creation.corpus_malformed: '{}' has a row with no roll", self.name))?;
            high = high.max(h);
        }
        if high == 0 {
            return Err(format!("cpr_creation.corpus_malformed: '{}' has no rows", self.name));
        }
        Ok(high)
    }

    fn row_for(&self, roll: u8) -> Option<&LifepathRow> {
        self.rows.iter().find(|r| r.range().is_some_and(|(low, high)| (low..=high).contains(&roll)))
    }

    /// "Roll 1d10 and subtract 7 (minimum 0)": the number subtracted, for a table that first
    /// rolls how many entries it has (Friends, Enemies, Tragic Love Affairs).
    fn count_subtract(&self) -> Option<u8> {
        let after = self.roll_instruction.split_once("subtract ")?.1;
        after.split(|c: char| !c.is_ascii_digit()).next()?.parse().ok()
    }

    /// "or choose one for each column" / "roll once on each of the columns": one die per column.
    fn per_column(&self) -> bool {
        self.roll_instruction.contains("each column") || self.roll_instruction.contains("each of the columns")
    }
}

/// The Lifepath tables of `role`: every general table, then the Role's, in corpus order.
fn lifepath_tables(root: &Path, role: &str) -> Result<Vec<LifepathRecord>, String> {
    let path = root.join("lifepath_table.json");
    let text = std::fs::read_to_string(&path).map_err(|e| format!("cpr_creation.read_failed: {}: {e}", path.display()))?;
    let all: Vec<LifepathRecord> =
        serde_json::from_str(&text).map_err(|e| format!("cpr_creation.parse_failed: {}: {e}", path.display()))?;
    let (general, role_tables): (Vec<_>, Vec<_>) = all.into_iter().partition(|t| t.group == "general");
    Ok(general.into_iter().chain(role_tables.into_iter().filter(|t| t.role.as_deref() == Some(role))).collect())
}

/// Each table rolls on its own roller (the request's seed and the table's place), so a pick on
/// one table never moves another table's dice, and the Lifepath never moves the STAT dice.
fn table_roller(seed: u32, index: usize) -> CprRoller {
    let place = u32::try_from(index + 1).unwrap_or(u32::MAX);
    CprRoller::new(seed ^ place.wrapping_mul(0x9E37_79B9).rotate_left(16))
}

/// A language name the saved grammar holds as `Language (<name>)`.
fn language_name_ok(name: &str) -> bool {
    !name.trim().is_empty() && !name.chars().any(|c| matches!(c, '(' | ')' | '|' | '=' | '#' | '\n' | '\r'))
}

struct LifepathOutcome {
    tables: Vec<CprLifepathTableDto>,
    language: Option<CprCulturalLanguageDto>,
}

/// Answers every Lifepath table of `role`: picks as picked, the rest rolled.
fn answer_lifepath(
    root: &Path,
    role: &str,
    seed: u32,
    request: &CprLifepathRequest,
    problems: &mut Vec<DiagnosticDto>,
) -> Result<LifepathOutcome, String> {
    let tables = lifepath_tables(root, role)?;
    let known = |name: &str| tables.iter().any(|t| t.name == name);
    for name in request.picks.iter().map(|p| &p.table).chain(request.counts.keys()).chain(&request.skips) {
        if !known(name) {
            problems.push(problem(REFUSED_LIFEPATH_TABLE, format!("'{name}' is not a Lifepath table of the {role} (p. 45-70)")));
        }
    }

    let mut out = Vec::new();
    let mut language = None;
    for (index, table) in tables.iter().enumerate() {
        let sides = table.sides()?;
        let mut roller = table_roller(seed, index);
        let skippable = table.branch.is_some();
        let skipped = request.skips.iter().any(|s| s == &table.name);
        if skipped && !skippable {
            problems.push(problem(
                REFUSED_LIFEPATH_SKIP,
                format!("'{}' is not behind a printed choice; roll or pick it", table.name),
            ));
        }
        let per_column = table.per_column();
        let parts = if per_column { table.columns.len() } else { 1 };

        // How many entries: a count table rolls 1d10 minus its number (minimum 0) or takes a pick.
        let (count, count_max, count_picked) = match table.count_subtract() {
            Some(minus) => {
                let max = sides.saturating_sub(minus);
                let die = roller.roll(sides);
                match request.counts.get(&table.name) {
                    Some(&picked) if picked > max => {
                        problems.push(problem(
                            REFUSED_LIFEPATH_COUNT,
                            format!("'{}': {picked} is more than 1d{sides} minus {minus} can give ({max})", table.name),
                        ));
                        (Some(picked), Some(max), true)
                    }
                    Some(&picked) => (Some(picked), Some(max), true),
                    None => (Some(die.saturating_sub(minus)), Some(max), false),
                }
            }
            None => {
                if request.counts.contains_key(&table.name) {
                    problems.push(problem(REFUSED_LIFEPATH_COUNT, format!("'{}' has no count to pick", table.name)));
                }
                (None, None, false)
            }
        };
        let entries: Vec<u8> = match count {
            Some(n) => (1..=n.min(count_max.unwrap_or(0))).collect(),
            None => vec![0],
        };

        for pick in request.picks.iter().filter(|p| p.table == table.name) {
            let entry_ok = entries.contains(&pick.entry);
            let column_ok = usize::from(pick.column) < parts;
            let roll_ok = table.row_for(pick.roll).is_some() && pick.roll <= sides;
            if !(entry_ok && column_ok && roll_ok) {
                problems.push(problem(
                    REFUSED_LIFEPATH_ROLL,
                    format!(
                        "'{}' has no roll {} in column {} of entry {} (rolls 1-{sides}, {parts} column(s), entries {:?})",
                        table.name,
                        pick.roll,
                        pick.column + 1,
                        pick.entry,
                        entries
                    ),
                ));
            }
        }

        let mut answers = Vec::new();
        if !skipped {
            for &entry in &entries {
                for part in 0..parts {
                    let die = roller.roll(sides);
                    let column = u8::try_from(part).unwrap_or(u8::MAX);
                    let picked = request
                        .picks
                        .iter()
                        .find(|p| p.table == table.name && p.entry == entry && p.column == column && table.row_for(p.roll).is_some())
                        .map(|p| p.roll);
                    let roll = picked.unwrap_or(die);
                    let row = table
                        .row_for(roll)
                        .ok_or_else(|| format!("cpr_creation.corpus_malformed: '{}' has no row for {roll}", table.name))?;
                    let cell = |i: usize| row.cells.get(i).cloned().unwrap_or_default();
                    let (text, detail) = if per_column {
                        (cell(part), String::new())
                    } else {
                        (cell(0), row.cells.iter().skip(1).cloned().collect::<Vec<_>>().join(" / "))
                    };
                    let column_name = table.columns.get(part).cloned().unwrap_or_default();
                    if ![&table.name, &column_name, &text, &detail].into_iter().all(|t| lifepath_text_ok(t)) {
                        return Err(format!("cpr_creation.corpus_malformed: '{}' row {roll} holds a '|' or a line break", table.name));
                    }
                    answers.push(CprLifepathAnswerDto {
                        entry,
                        column_index: column,
                        column: column_name,
                        roll,
                        picked: picked.is_some(),
                        text,
                        detail,
                    });
                }
            }
        }

        // p. 46: the Cultural Origin language, one of the region's (or any other), at its free rank.
        if let Some(level) = table.free_language_rank {
            let region = answers.first().map(|a| a.roll);
            let listed: Vec<String> = region.and_then(|r| table.languages_by_roll.get(&r.to_string()).cloned()).unwrap_or_default();
            let die = if listed.is_empty() { 0 } else { roller.roll(u8::try_from(listed.len()).unwrap_or(u8::MAX)) };
            let (name, picked) = match &request.cultural_language {
                Some(name) => (name.trim().to_owned(), true),
                None => (listed.get(usize::from(die).saturating_sub(1)).cloned().unwrap_or_default(), false),
            };
            if language_name_ok(&name) {
                language = Some(CprCulturalLanguageDto { skill: format!("Language ({name})"), name, picked, level });
            } else {
                problems.push(problem(
                    REFUSED_CULTURAL_LANGUAGE,
                    format!("the Cultural Origin language '{name}' must be a name without ( ) | = or #"),
                ));
            }
        }

        out.push(CprLifepathTableDto {
            table: table.name.clone(),
            group: table.group.clone(),
            source_page: table.source_page.clone(),
            die: format!("1d{sides}"),
            roll_instruction: table.roll_instruction.clone(),
            count,
            count_max,
            count_picked,
            skippable,
            skipped,
            per_column,
            answers,
        });
    }
    Ok(LifepathOutcome { tables: out, language })
}

/// The choices creation saves beyond the STATs: the Role at its starting rank and every
/// allocated skill at its level (the Cultural Origin language among them).
fn creation_choices(role: Option<(CprRole, u8)>, skills: &CprSkillsDto) -> CprSheetChoices {
    CprSheetChoices {
        role,
        skills: skills.lines.iter().map(|l| (l.key.clone(), l.level)).collect(),
        ..CprSheetChoices::default()
    }
}

/// The request's Role Ability sub-choices in the saved shape.
fn role_choices(request: &CprRoleAbilityRequest) -> CprRoleChoices {
    CprRoleChoices {
        points: request.points.iter().filter(|(_, p)| **p > 0).map(|(n, p)| (n.clone(), *p)).collect(),
        picks: request.picks.iter().map(|p| CprRolePick { name: p.name.clone(), on: p.on.clone() }).collect(),
    }
}

/// The Role Ability at the Role's starting rank (p. 142-143: every Role starts at 4): the
/// engine's menu of sub-choices and, for choices it accepts, the rank's and the choices' rows.
/// A name the saved grammar cannot hold, and every rule the engine refuses, become problems.
fn role_ability_step(
    root: &Path,
    role_name: &str,
    request: &CprRoleAbilityRequest,
    problems: &mut Vec<DiagnosticDto>,
) -> Result<Option<CprRoleAbilityDto>, String> {
    let Some(role) = CprRole::from_name(role_name) else { return Ok(None) };
    let catalog = CprRoleCatalog::load(root)?;
    let rank = catalog.starting_rank(role);
    let choices = role_choices(request);
    let menu = catalog.choice_menu(role, rank, &choices)?;
    let mut rows = Vec::new();
    let bad_names: Vec<&str> = choices
        .points
        .iter()
        .map(|(n, _)| n.as_str())
        .chain(choices.picks.iter().flat_map(|p| std::iter::once(p.name.as_str()).chain(p.on.as_deref())))
        .filter(|n| !role_choice_text_ok(n))
        .collect();
    if !bad_names.is_empty() {
        for name in bad_names {
            problems.push(problem(REFUSED_ROLE_ABILITY, format!("'{name}': a Role Ability choice is a name without = @ or #")));
        }
    } else {
        match catalog.choice_rows(role, rank, &choices) {
            Ok(choice_rows) => {
                let rank_rows = catalog.rank_rows(role, rank).map_err(|e| e.to_string())?;
                rows = rank_rows
                    .into_iter()
                    .chain(choice_rows)
                    .map(|e| ExplanationDto { id: e.id, value: e.value, detail: e.detail })
                    .collect();
            }
            Err(reasons) => problems.extend(reasons.into_iter().map(|r| problem(REFUSED_ROLE_ABILITY, r))),
        }
    }
    Ok(Some(CprRoleAbilityDto { ability: menu.ability.clone(), rank, menu, rows }))
}

/// The printed `(choose 1)` suffix of a Role list line whose instrument or field the player names.
const CHOOSE_ONE: &str = "(choose 1)";

/// The id fragment of a skill key (`handgun`, `martial_arts.karate`), as the sheet's rows use it.
fn skill_row_rest(catalog: &CprSkillCatalog, key: &str) -> Option<String> {
    let (skill, specialty) = catalog.resolve(key).ok()?;
    Some(match specialty {
        Some(spec) => format!("{}.{}", skill_slug(&skill.name), skill_slug(&spec)),
        None => skill_slug(&skill.name),
    })
}

/// Builds the skill allocation the method allows from the player's choices, then has the engine
/// validate and price it. Refusals become claim-blocking `problems`; the points rows come back
/// only for an allocation the engine accepts.
fn allocate_skills(
    catalog: &CprSkillCatalog,
    role: &str,
    method: CprCreationMethod,
    request: &CprSkillsRequest,
    language: Option<&CprCulturalLanguageDto>,
    problems: &mut Vec<DiagnosticDto>,
) -> Result<CprSkillsDto, String> {
    let before = problems.len();
    let cultural = language.map(|l| l.skill.as_str());
    let x2 = |key: &str| catalog.resolve(key).is_ok_and(|(s, _)| s.x2);
    let line = |line: &str, key: String, level: u8, basic: bool, adjustable: bool, choose_one: bool| CprSkillLineDto {
        line: line.to_owned(),
        x2: x2(&key),
        cultural: cultural == Some(key.as_str()),
        level_row: skill_row_rest(catalog, &key).map(|rest| format!("cpr.skill_level.{rest}")).unwrap_or_default(),
        key,
        level,
        basic,
        adjustable,
        choose_one,
    };
    let mut lines: Vec<CprSkillLineDto> = Vec::new();
    match method {
        CprCreationMethod::Streetrat | CprCreationMethod::Edgerunner => {
            // p. 86-89: the Role's 20 skills; Streetrat at the printed levels, Edgerunner from them.
            let list = catalog.role_list(role)?;
            let adjustable = method == CprCreationMethod::Edgerunner;
            for entry in &list.skills {
                let choose_one = entry.name.ends_with(CHOOSE_ONE);
                let key = if choose_one {
                    let name = entry.name.trim_end_matches(CHOOSE_ONE).trim();
                    match request.specialties.get(&entry.name).map(|s| s.trim()).filter(|s| !s.is_empty()) {
                        Some(spec) => format!("{name} ({spec})"),
                        None => {
                            problems.push(problem(
                                REFUSED_SKILL_SPECIALTY,
                                format!("name the {name} for the {role}'s '{}' skill (p. 86-89)", entry.name),
                            ));
                            entry.name.clone()
                        }
                    }
                } else {
                    entry.name.clone()
                };
                let level = request.levels.get(&entry.name).copied().unwrap_or(entry.level);
                lines.push(line(&entry.name, key, level, entry.basic, adjustable, choose_one));
            }
            // Any other key is the engine's to refuse (off the list), or the Cultural Origin language raised.
            for (key, level) in request.levels.iter().filter(|(k, _)| !list.skills.iter().any(|e| &e.name == *k)) {
                lines.push(line(key, key.clone(), *level, false, adjustable, false));
            }
        }
        CprCreationMethod::CompletePackage => {
            // p. 90: any skills; the Basic skills start at 2; a level of 0 drops a skill.
            let basic = catalog.basic_skills();
            for key in basic {
                let level = request.levels.get(key).copied().unwrap_or(2);
                if level > 0 {
                    lines.push(line(key, key.clone(), level, true, true, false));
                }
            }
            for (key, level) in request.levels.iter().filter(|(k, l)| !basic.contains(*k) && **l > 0) {
                lines.push(line(key, key.clone(), *level, false, true, false));
            }
        }
    }
    if let Some(language) = language.filter(|_| !lines.iter().any(|l| l.cultural)) {
        let adjustable = method != CprCreationMethod::Streetrat;
        lines.push(line(&language.skill, language.skill.clone(), language.level, false, adjustable, false));
    }

    // A named specialty is saved as `Skill (<name>)`: it must be a name the grammar holds.
    for l in lines.iter().filter(|l| !l.key.ends_with(CHOOSE_ONE)) {
        let bad_specialty = catalog.resolve(&l.key).ok().and_then(|(_, spec)| spec).filter(|s| !language_name_ok(s));
        if let Some(spec) = bad_specialty {
            problems.push(problem(
                REFUSED_SKILL_SPECIALTY,
                format!("'{}': the specialty '{spec}' must be a name without ( ) | = or #", l.key),
            ));
        }
    }
    if problems.len() > before {
        return Ok(CprSkillsDto { lines, rows: Vec::new() });
    }

    let allocation = CprSkillAllocation {
        method: match method {
            CprCreationMethod::Streetrat => CprSkillMethod::Streetrat { role: role.to_owned() },
            CprCreationMethod::Edgerunner => CprSkillMethod::Edgerunner { role: role.to_owned() },
            CprCreationMethod::CompletePackage => CprSkillMethod::CompletePackage,
        },
        cultural_language: language.map(|l| l.skill.clone()),
        levels: lines.iter().map(|l| (l.key.clone(), l.level)).collect(),
    };
    let rows = match validate_creation_skills(catalog, &allocation) {
        Ok(spend) => spend_rows(&spend).into_iter().map(|e| ExplanationDto { id: e.id, value: e.value, detail: e.detail }).collect(),
        Err(reason) => {
            problems.push(problem(REFUSED_SKILLS, reason));
            Vec::new()
        }
    };
    Ok(CprSkillsDto { lines, rows })
}

/// Every Lifepath answer, in panel order, as saved.
fn saved_lifepath(lifepath: &[CprLifepathTableDto]) -> CprLifepath {
    CprLifepath {
        answers: lifepath
            .iter()
            .flat_map(|t| {
                t.answers.iter().map(|a| CprLifepathAnswer {
                    table: t.table.clone(),
                    entry: a.entry,
                    column: a.column.clone(),
                    text: a.text.clone(),
                    detail: a.detail.clone(),
                })
            })
            .collect(),
    }
}

/// The engine's rows for the allocated skills: each one's level and Skill Base, from the
/// composed sheet of these STATs and those skills.
fn allocated_skill_rows(
    root: &Path,
    catalog: &CprSkillCatalog,
    input: &CprCharacterInput,
    skills: &CprSkillsDto,
) -> Result<Vec<ExplanationDto>, String> {
    let catalogs = CprSheetCatalogs::load(root)?;
    let sheet = compose_sheet(&catalogs, input, &creation_choices(None, skills))?;
    let wanted: Vec<String> = skills
        .lines
        .iter()
        .filter_map(|l| skill_row_rest(catalog, &l.key))
        .flat_map(|rest| [format!("cpr.skill_level.{rest}"), format!("cpr.skill.{rest}")])
        .collect();
    Ok(sheet
        .rows
        .into_iter()
        .filter(|r| wanted.contains(&r.id))
        .map(|e| ExplanationDto { id: e.id, value: e.value, detail: e.detail })
        .collect())
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

    let lifepath = answer_lifepath(root, &request.role, seed, &request.lifepath, &mut problems)?;
    let catalog = CprSkillCatalog::load(root)?;
    let skills = allocate_skills(&catalog, &request.role, request.method, &request.skills, lifepath.language.as_ref(), &mut problems)?;
    let role_ability = role_ability_step(root, &request.role, &request.role_ability, &mut problems)?;
    let input = input_of(values);
    let explanations = if problems.is_empty() {
        let mut rows = engine_rows(&input);
        rows.extend(allocated_skill_rows(root, &catalog, &input, &skills)?);
        rows.extend(skills.rows.iter().cloned());
        rows.extend(role_ability.iter().flat_map(|a| a.rows.iter().cloned()));
        rows
    } else {
        Vec::new()
    };
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
        lifepath: lifepath.tables,
        cultural_language: lifepath.language,
        skills,
        role_ability,
        explanations,
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
    let role = CprRole::from_name(&creation.role)
        .ok_or_else(|| format!("cpr_creation.unknown_role: no Cyberpunk RED Role named '{}'", creation.role))?;
    let rank = preview.role_ability.as_ref().map_or(0, |a| a.rank);
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
    CprCharacterStore::save_choices(&creation_choices(Some((role, rank)), &preview.skills), root)?;
    CprCharacterStore::save_lifepath(&saved_lifepath(&preview.lifepath), root)?;
    CprCharacterStore::save_role_choices(&role_choices(&creation.role_ability), root)?;
    let loaded = crate::rule_system_adapter::load_saved_character_via_envelope(root)?;
    let mut explanations = engine_rows(&input);
    explanations.extend(allocated_skill_rows(corpus, &CprSkillCatalog::load(corpus)?, &input, &preview.skills)?);
    explanations.extend(preview.skills.rows.iter().cloned());
    explanations.extend(preview.role_ability.iter().flat_map(|a| a.rows.iter().cloned()));
    Ok(CprCreateResponse::Saved { summary: Box::new(loaded.summary), seed: preview.seed, explanations })
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
        CprCreationRequest {
            role: role.to_owned(),
            method,
            seed: None,
            template_roll: None,
            stats: None,
            lifepath: CprLifepathRequest::default(),
            // The two `(choose 1)` lines of the Role lists (Rockerboy's instrument, the Tech's
            // and the Medtech's field), named as a player would.
            skills: CprSkillsRequest {
                levels: BTreeMap::new(),
                specialties: [("Play Instrument (choose 1)", "Guitar"), ("Science (choose 1)", "Biology")]
                    .into_iter()
                    .map(|(line, name)| (line.to_owned(), name.to_owned()))
                    .collect(),
            },
            role_ability: starting_role_ability(role),
        }
    }

    /// The two Roles whose Role Ability points must all be placed (D-7) take a valid placement;
    /// every other Role starts with no sub-choice.
    fn starting_role_ability(role: &str) -> CprRoleAbilityRequest {
        let points: &[(&str, u8)] = match role {
            "Tech" => &[("Field Expertise", 2), ("Upgrade Expertise", 2), ("Fabrication Expertise", 2), ("Invention Expertise", 2)],
            "Medtech" => &[("Surgery", 2), ("Medical Tech (Pharmaceuticals)", 1), ("Medical Tech (Cryosystem Operation)", 1)],
            _ => &[],
        };
        let picks: &[(&str, Option<&str>)] = if role == "Medtech" { &[("Antibiotic", None)] } else { &[] };
        role_ability(points, picks)
    }

    fn role_ability(points: &[(&str, u8)], picks: &[(&str, Option<&str>)]) -> CprRoleAbilityRequest {
        CprRoleAbilityRequest {
            points: points.iter().map(|(n, p)| ((*n).to_owned(), *p)).collect(),
            picks: picks.iter().map(|(n, on)| CprRolePickRequest { name: (*n).to_owned(), on: on.map(str::to_owned) }).collect(),
        }
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

    // ---- Lifepath (p. 45-70) ----

    /// Every `lifepath_table` record, read as plain JSON: (name, group, role).
    fn lifepath_records() -> Vec<(String, String, Option<String>)> {
        let rows: Vec<Value> =
            serde_json::from_str(&std::fs::read_to_string(root().join("lifepath_table.json")).unwrap()).unwrap();
        rows.iter()
            .map(|r| {
                (
                    r["name"].as_str().unwrap().to_owned(),
                    r["group"].as_str().unwrap().to_owned(),
                    r["role"].as_str().map(str::to_owned),
                )
            })
            .collect()
    }

    fn role_names() -> Vec<String> {
        let rows: Vec<Value> = serde_json::from_str(&std::fs::read_to_string(root().join("role.json")).unwrap()).unwrap();
        rows.iter().map(|r| r["name"].as_str().unwrap().to_owned()).collect()
    }

    fn seeded(role: &str, seed: u32) -> CprCreationRequest {
        let mut req = request(role, CprCreationMethod::Streetrat);
        req.template_roll = Some(1);
        req.seed = Some(seed);
        req
    }

    fn table<'a>(preview: &'a CprCreationPreview, name: &str) -> &'a CprLifepathTableDto {
        preview.lifepath.iter().find(|t| t.table == name).unwrap_or_else(|| panic!("no Lifepath table {name}"))
    }

    fn answer_texts(t: &CprLifepathTableDto) -> Vec<String> {
        t.answers.iter().map(|a| a.text.clone()).collect()
    }

    /// The panel answers the 15 general tables (CUI F-26) and exactly the chosen Role's tables
    /// (CUI F-27: 52 over the ten Roles), in corpus order, for every Role.
    #[test]
    fn the_lifepath_answers_every_general_table_and_the_roles_tables() {
        let records = lifepath_records();
        let general: Vec<&String> = records.iter().filter(|(_, g, _)| g == "general").map(|(n, _, _)| n).collect();
        assert_eq!(general.len(), 15, "CUI F-26");
        let roles = role_names();
        assert_eq!(roles.len(), 10);
        let mut role_tables = 0;
        for role in &roles {
            let preview = preview_at(&root(), &seeded(role, 11)).unwrap();
            assert!(preview.problems.is_empty(), "{role}: {:?}", preview.problems);
            let mine: Vec<&String> = records
                .iter()
                .filter(|(_, g, r)| g == "general" || r.as_deref() == Some(role.as_str()))
                .map(|(n, _, _)| n)
                .collect();
            let answered: Vec<&String> = preview.lifepath.iter().map(|t| &t.table).collect();
            assert_eq!(answered, mine, "{role}");
            role_tables += mine.len() - 15;
            for t in &preview.lifepath {
                if !t.skipped && t.count != Some(0) {
                    assert!(!t.answers.is_empty(), "{role}: {} has no answer", t.table);
                }
                assert!(t.answers.iter().all(|a| !a.picked && !a.text.is_empty()), "{role}: {} rolled", t.table);
            }
        }
        assert_eq!(role_tables, 52, "CUI F-27");
    }

    /// One seed gives the same answers; a pick changes only its own table; a different seed
    /// rolls differently; the Lifepath dice never move the STAT dice.
    #[test]
    fn lifepath_rolls_are_seeded_and_a_pick_changes_only_its_table() {
        let a = preview_at(&root(), &seeded("Solo", 5)).unwrap();
        let b = preview_at(&root(), &seeded("Solo", 5)).unwrap();
        assert_eq!(a.lifepath, b.lifepath);

        let mut picked = seeded("Solo", 5);
        picked.lifepath.picks.push(CprLifepathPick { table: "Your Personality".to_owned(), entry: 0, column: 0, roll: 3 });
        let p = preview_at(&root(), &picked).unwrap();
        assert!(p.problems.is_empty(), "{:?}", p.problems);
        let personality = table(&p, "Your Personality");
        assert_eq!(personality.answers.len(), 1);
        assert!(personality.answers[0].picked);
        assert_eq!(personality.answers[0].roll, 3);
        assert_eq!(personality.answers[0].text, "Arrogant, proud, and aloof");
        for (before, after) in a.lifepath.iter().zip(&p.lifepath) {
            if before.table != "Your Personality" {
                assert_eq!(before, after, "{} moved", before.table);
            }
        }

        let mut edgerunner = request("Solo", CprCreationMethod::Edgerunner);
        edgerunner.seed = Some(5);
        let first = preview_at(&root(), &edgerunner).unwrap();
        edgerunner.lifepath = picked.lifepath.clone();
        let second = preview_at(&root(), &edgerunner).unwrap();
        assert_eq!(first.rolls, second.rolls, "the STAT dice ignore the Lifepath");

        let differs = (6..40u32).any(|s| preview_at(&root(), &seeded("Solo", s)).unwrap().lifepath != a.lifepath);
        assert!(differs);
    }

    /// Friends, Enemies and Tragic Love Affairs: "Roll 1d10 and subtract 7 (minimum 0)" for how
    /// many, then one answer per entry and column. A picked count is kept; more than the die
    /// can give is refused.
    #[test]
    fn count_tables_roll_1d10_minus_7_and_answer_each_entry() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..200u32 {
            let p = preview_at(&root(), &seeded("Fixer", seed)).unwrap();
            for (name, columns) in [("Your Friends", 1), ("Your Enemies", 3), ("Your Tragic Love Affair(s)", 1)] {
                let t = table(&p, name);
                let count = t.count.expect("a count table has a count");
                assert!(count <= 3, "{name} {count}");
                assert_eq!(t.count_max, Some(3));
                assert_eq!(t.answers.len(), usize::from(count) * columns, "{name} seed {seed}");
                seen.insert(count);
            }
        }
        assert_eq!(seen.into_iter().collect::<Vec<_>>(), [0, 1, 2, 3], "every count 0-3 occurs");

        let mut req = seeded("Fixer", 1);
        req.lifepath.counts.insert("Your Enemies".to_owned(), 2);
        req.lifepath.picks.push(CprLifepathPick { table: "Your Enemies".to_owned(), entry: 2, column: 1, roll: 7 });
        let p = preview_at(&root(), &req).unwrap();
        assert!(p.problems.is_empty(), "{:?}", p.problems);
        let enemies = table(&p, "Your Enemies");
        assert_eq!(enemies.count, Some(2));
        assert_eq!(enemies.answers.len(), 6);
        let picked = enemies.answers.iter().find(|a| a.entry == 2 && a.column_index == 1).unwrap();
        assert_eq!(picked.text, "You just don't like each other.");
        assert_eq!(picked.column, "What Caused it? Who's been Wronged? (choose)");

        let mut over = seeded("Fixer", 1);
        over.lifepath.counts.insert("Your Friends".to_owned(), 4);
        let p = preview_at(&root(), &over).unwrap();
        assert!(p.problems.iter().any(|x| x.id == REFUSED_LIFEPATH_COUNT && x.claim_blocking), "{:?}", p.problems);
    }

    /// Printed roll ranges ("1-2", Sweet Revenge!) and D-9 (Exec Corp kind printed "1d6" over
    /// ten rows: rolled as 1d10).
    #[test]
    fn range_rows_and_the_d9_exec_corp_table_roll_their_whole_range() {
        let mut req = seeded("Exec", 3);
        req.lifepath.picks.push(CprLifepathPick { table: "Sweet Revenge!".to_owned(), entry: 0, column: 0, roll: 2 });
        let p = preview_at(&root(), &req).unwrap();
        assert_eq!(answer_texts(table(&p, "Sweet Revenge!")), ["Avoid the scum."]);

        let corp = "Exec: What Kind of Corp Do You Work For?";
        assert_eq!(table(&p, corp).die, "1d10", "D-9");
        let mut rolls = std::collections::BTreeSet::new();
        for seed in 0..300u32 {
            let p = preview_at(&root(), &seeded("Exec", seed)).unwrap();
            rolls.insert(table(&p, corp).answers[0].roll);
        }
        assert_eq!(rolls.into_iter().collect::<Vec<_>>(), (1..=10).collect::<Vec<u8>>());
    }

    /// A table behind a printed choice (Rockerboy group or solo, partner or alone, Nomad land,
    /// air or sea) can be marked not taken; any other table cannot. Unknown tables and rolls
    /// off the table are refused.
    #[test]
    fn branch_tables_can_be_skipped_and_bad_picks_are_refused() {
        let mut req = seeded("Nomad", 2);
        req.lifepath.skips = vec!["Nomad: If in Air, What Do They Do?".to_owned(), "Nomad: If at Sea, What Do They Do?".to_owned()];
        let p = preview_at(&root(), &req).unwrap();
        assert!(p.problems.is_empty(), "{:?}", p.problems);
        let air = table(&p, "Nomad: If in Air, What Do They Do?");
        assert!(air.skipped && air.answers.is_empty() && air.skippable);
        assert!(!table(&p, "Nomad: If on Land, What Do They Do?").skipped);

        let refused = [
            (vec![], vec!["Your Life Goals".to_owned()], REFUSED_LIFEPATH_SKIP),
            (vec![CprLifepathPick { table: "Solo: What Kind of Solo are You?".to_owned(), entry: 0, column: 0, roll: 1 }], vec![], REFUSED_LIFEPATH_TABLE),
            (vec![CprLifepathPick { table: "Your Personality".to_owned(), entry: 0, column: 0, roll: 11 }], vec![], REFUSED_LIFEPATH_ROLL),
            (vec![CprLifepathPick { table: "Your Personality".to_owned(), entry: 0, column: 1, roll: 1 }], vec![], REFUSED_LIFEPATH_ROLL),
        ];
        for (picks, skips, id) in refused {
            let mut req = seeded("Nomad", 2);
            req.lifepath.picks = picks;
            req.lifepath.skips = skips;
            let p = preview_at(&root(), &req).unwrap();
            assert!(p.problems.iter().any(|x| x.id == id && x.claim_blocking), "{id}: {:?}", p.problems);
            assert!(p.explanations.is_empty());
        }
    }

    /// p. 46: the Cultural Origin language (one of the region's, or any other) is a Language
    /// skill at 4 (`free_language_rank`). Picked or rolled from the region's list, it is an
    /// engine row in the preview, and the created character loads with it at level 4.
    #[test]
    fn the_cultural_origin_language_becomes_a_language_skill_at_4() {
        let mut req = seeded("Solo", 9);
        req.lifepath.picks.push(CprLifepathPick { table: "Cultural Origins".to_owned(), entry: 0, column: 0, roll: 9 });
        req.lifepath.cultural_language = Some("Japanese".to_owned());
        let p = preview_at(&root(), &req).unwrap();
        assert!(p.problems.is_empty(), "{:?}", p.problems);
        assert_eq!(table(&p, "Cultural Origins").answers[0].text, "East Asian");
        let language = p.cultural_language.as_ref().expect("a language");
        assert_eq!((language.name.as_str(), language.picked), ("Japanese", true));
        assert_eq!(row(&p.explanations, "cpr.skill_level.language.japanese"), 4);
        let int = row(&p.explanations, "cpr.stat.int");
        assert_eq!(row(&p.explanations, "cpr.skill.language.japanese"), int + 4);

        // Rolled: one of the rolled region's languages.
        for seed in 0..30u32 {
            let p = preview_at(&root(), &seeded("Media", seed)).unwrap();
            let region = &table(&p, "Cultural Origins").answers[0];
            let language = p.cultural_language.as_ref().unwrap();
            assert!(!language.picked);
            assert!(region.detail.split(", ").any(|l| l == language.name), "{} not in {}", language.name, region.detail);
            assert_eq!(row(&p.explanations, &format!("cpr.skill_level.language.{}", skill_slug(&language.name))), 4);
        }

        // Any other language is the player's to choose (p. 46); a name the grammar cannot hold is refused.
        let mut other = seeded("Solo", 9);
        other.lifepath.cultural_language = Some("Esperanto".to_owned());
        assert_eq!(row(&preview_at(&root(), &other).unwrap().explanations, "cpr.skill_level.language.esperanto"), 4);
        for bad in ["", "  ", "Pidgin (Old)", "a|b"] {
            other.lifepath.cultural_language = Some(bad.to_owned());
            let p = preview_at(&root(), &other).unwrap();
            assert!(p.problems.iter().any(|x| x.id == REFUSED_CULTURAL_LANGUAGE), "{bad:?}: {:?}", p.problems);
        }

        // Created: saved with the answers, loaded through the envelope router with the row at 4.
        let characters_root = tempdir("lifepath");
        let dir = characters_root.join("cpr-lifepath");
        let response = create_at_root(&root(), &dir, &create(req, "cpr-lifepath"), "codex-test".to_owned()).unwrap();
        let CprCreateResponse::Saved { explanations, .. } = response else { panic!("{response:?}") };
        assert_eq!(row(&explanations, "cpr.skill_level.language.japanese"), 4);
        let loaded = crate::rule_system_adapter::load_saved_character_via_envelope(&dir).unwrap();
        assert_eq!(row(&loaded.explanations, "cpr.skill_level.language.japanese"), 4);
        let choices = CprCharacterStore::load_choices(&dir).unwrap();
        assert!(choices.skills.contains(&("Language (Japanese)".to_owned(), 4)), "{:?}", choices.skills);
        let saved_answers: Vec<(String, String)> =
            CprCharacterStore::load_lifepath(&dir).unwrap().answers.iter().map(|a| (a.table.clone(), a.text.clone())).collect();
        let previewed: Vec<(String, String)> =
            p.lifepath.iter().flat_map(|t| t.answers.iter().map(|a| (t.table.clone(), a.text.clone()))).collect();
        assert_eq!(saved_answers, previewed, "every previewed answer is saved, in order");
        assert!(saved_answers.iter().any(|(t, a)| t == "Cultural Origins" && a == "East Asian"));
        std::fs::remove_dir_all(&characters_root).ok();
    }

    // ---- Skills (p. 85-90) ----

    /// The p087-p091 tables and worked examples, typed from the page (the root crate's engine
    /// fixture), not read from the corpus the allocator reads.
    fn skills_fixture() -> Value {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/fixtures/cpr/streetrat_skills_p087.json");
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }

    /// Every Role skill list read as plain JSON: (role, [(line, level)]).
    fn role_lists() -> Vec<(String, Vec<(String, u8)>)> {
        let rows: Vec<Value> =
            serde_json::from_str(&std::fs::read_to_string(root().join("role_skill_list.json")).unwrap()).unwrap();
        rows.iter()
            .map(|r| {
                let lines = r["skills"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|s| (s["name"].as_str().unwrap().to_owned(), s["level"].as_u64().unwrap() as u8))
                    .collect();
                (r["role"].as_str().unwrap().to_owned(), lines)
            })
            .collect()
    }

    fn spent(rows: &[ExplanationDto]) -> (i16, i16) {
        (row(rows, "cpr.skills.creation_points_spent"), row(rows, "cpr.skills.creation_points_remaining"))
    }

    fn skill_problem(preview: &CprCreationPreview, id: &str, words: &[&str]) -> bool {
        preview
            .problems
            .iter()
            .any(|p| p.id == id && p.claim_blocking && words.iter().all(|w| p.message.contains(w)))
    }

    /// Streetrat (p. 86-87): the Role's 20 skills at their printed levels, each list 86 points
    /// (CUI F-25), plus the Cultural Origin language at 4 for free. A `(choose 1)` line takes
    /// the player's named instrument or field; a changed level is refused.
    #[test]
    fn streetrat_skills_are_the_roles_printed_list_at_86_points() {
        let mut checked = 0;
        for (role, lines) in role_lists() {
            let preview = preview_at(&root(), &seeded(&role, 4)).unwrap();
            assert!(preview.problems.is_empty(), "{role}: {:?}", preview.problems);
            assert_eq!(spent(&preview.skills.rows), (86, 0), "{role}: CUI F-25");
            let listed: Vec<(String, u8)> =
                preview.skills.lines.iter().filter(|l| !l.cultural).map(|l| (l.line.clone(), l.level)).collect();
            assert_eq!(listed, lines, "{role}: the printed list, in order, at its printed levels");
            let language = preview.skills.lines.iter().find(|l| l.cultural).expect("the Cultural Origin language line");
            assert_eq!(language.level, 4);
            assert!(preview.skills.lines.iter().all(|l| !l.adjustable), "{role}: Streetrat levels are fixed");
            for line in preview.skills.lines.iter().filter(|l| l.line.ends_with("(choose 1)")) {
                assert!(line.choose_one && !line.key.contains("choose 1"), "{role}: {} names its specialty: {}", line.line, line.key);
            }
            checked += 1;
        }
        assert_eq!(checked, 10);

        // A Streetrat level is the printed one: Solo Tactics 6 sent as 5 is refused.
        let mut changed = seeded("Solo", 4);
        changed.skills.levels.insert("Tactics".to_owned(), 5);
        let p = preview_at(&root(), &changed).unwrap();
        assert!(skill_problem(&p, REFUSED_SKILLS, &["Tactics", "Streetrat"]), "{:?}", p.problems);
        assert!(p.skills.rows.is_empty() && p.explanations.is_empty());

        // A `(choose 1)` line with no named instrument is refused, and so is a name the saved
        // grammar cannot hold.
        let mut unnamed = seeded("Rockerboy", 4);
        unnamed.skills.specialties.clear();
        let p = preview_at(&root(), &unnamed).unwrap();
        assert!(skill_problem(&p, REFUSED_SKILL_SPECIALTY, &["Play Instrument"]), "{:?}", p.problems);
        unnamed.skills.specialties.insert("Play Instrument (choose 1)".to_owned(), "Gui|tar".to_owned());
        let p = preview_at(&root(), &unnamed).unwrap();
        assert!(skill_problem(&p, REFUSED_SKILL_SPECIALTY, &["Gui|tar"]), "{:?}", p.problems);
    }

    /// Edgerunner (p. 88): the Role's 20 skills, each 2 to 6, 86 points, (x2) skills two points
    /// a level. The allocation starts at the Streetrat levels (86 spent) and the player moves it.
    #[test]
    fn edgerunner_skills_are_the_roles_20_from_2_to_6_within_86_points() {
        let mut req = request("Solo", CprCreationMethod::Edgerunner);
        req.seed = Some(4);
        let p = preview_at(&root(), &req).unwrap();
        assert!(p.problems.is_empty(), "{:?}", p.problems);
        assert_eq!(spent(&p.skills.rows), (86, 0), "the start: the printed levels");
        assert_eq!(p.skills.lines.iter().filter(|l| !l.cultural).count(), 20);
        assert!(p.skills.lines.iter().filter(|l| !l.cultural).all(|l| l.adjustable));

        // p090's worked example, typed from the page: 86 points, line by line.
        let fx = skills_fixture();
        let mut example = req.clone();
        for skill in fx["worked_examples"]["edgerunner_solo_p090"]["skills"].as_array().unwrap() {
            example.skills.levels.insert(skill["name"].as_str().unwrap().to_owned(), skill["level"].as_u64().unwrap() as u8);
        }
        let p = preview_at(&root(), &example).unwrap();
        assert!(p.problems.is_empty(), "{:?}", p.problems);
        assert_eq!(spent(&p.skills.rows), (86, 0), "p090 Total Cost 86");

        // Tactics 6 -> 2 frees 4 points; Autofire (x2) 6 -> 5 frees 2 more.
        let mut lower = req.clone();
        lower.skills.levels.insert("Tactics".to_owned(), 2);
        assert_eq!(spent(&preview_at(&root(), &lower).unwrap().skills.rows), (82, 4));
        lower.skills.levels.insert("Autofire".to_owned(), 5);
        assert_eq!(spent(&preview_at(&root(), &lower).unwrap().skills.rows), (80, 6), "(x2): two points a level");

        let refused: [(&str, u8, &[&str]); 4] = [
            ("Athletics", 3, &["87 of 86"]),
            ("Tactics", 7, &["Tactics", "above 6"]),
            ("Tactics", 1, &["Tactics", "lower than 2"]),
            ("Archery", 2, &["Archery", "not on the Solo"]),
        ];
        let characters_root = tempdir("edgerunner-skills");
        for (skill, level, words) in refused {
            let mut bad = req.clone();
            bad.skills.levels.insert(skill.to_owned(), level);
            let p = preview_at(&root(), &bad).unwrap();
            assert!(skill_problem(&p, REFUSED_SKILLS, words), "{skill} {level}: {:?}", p.problems);
            let dir = characters_root.join(format!("er-{}-{level}", skill.to_lowercase()));
            let response = create_at_root(&root(), &dir, &create(bad, "er"), "codex-test".to_owned()).unwrap();
            assert!(
                matches!(&response, CprCreateResponse::Blocked { diagnostics } if diagnostics.iter().any(|d| d.id == REFUSED_SKILLS)),
                "{skill} {level}: {response:?}"
            );
            assert!(!dir.join("envelope.txt").exists(), "{skill} {level}: nothing saved");
        }
        std::fs::remove_dir_all(&characters_root).ok();
    }

    /// Complete Package (p. 90): any of the 66 skills, none above 6, the 13 Basic skills at 2 or
    /// more, (x2) skills double. The start is the Basic skills at 2 (26 of 86, "the remaining
    /// 60", p. 90); the p091 example spends exactly 86.
    #[test]
    fn complete_package_skills_are_any_of_66_capped_at_6_with_basic_at_2() {
        let mut req = request("Solo", CprCreationMethod::CompletePackage);
        req.stats = Some(allocation([5, 7, 8, 3, 6, 7, 6, 8, 8, 4]));
        req.seed = Some(4);
        let p = preview_at(&root(), &req).unwrap();
        assert!(p.problems.is_empty(), "{:?}", p.problems);
        assert_eq!(spent(&p.skills.rows), (26, 60), "13 Basic skills at 2; p. 90: 'the remaining 60'");
        let basic: Vec<String> = skills_fixture()["basic_skills"].as_array().unwrap().iter().map(|b| b.as_str().unwrap().to_owned()).collect();
        let held: Vec<String> = p.skills.lines.iter().filter(|l| !l.cultural).map(|l| l.key.clone()).collect();
        assert_eq!(held, basic);
        assert!(p.skills.lines.iter().filter(|l| !l.cultural).all(|l| l.basic && l.adjustable));

        let fx = skills_fixture();
        let mut example = req.clone();
        for skill in fx["worked_examples"]["complete_package_solo_p091"]["skills"].as_array().unwrap() {
            example.skills.levels.insert(skill["name"].as_str().unwrap().to_owned(), skill["level"].as_u64().unwrap() as u8);
        }
        let p = preview_at(&root(), &example).unwrap();
        assert!(p.problems.is_empty(), "{:?}", p.problems);
        assert_eq!(spent(&p.skills.rows), (86, 0), "p091 Total Cost 86");
        let karate = p.skills.lines.iter().find(|l| l.key == "Martial Arts (Karate)").unwrap();
        assert!(karate.x2 && !karate.basic);
        assert_eq!(row(&p.explanations, "cpr.skill_level.martial_arts.karate"), 2);
        assert_eq!(row(&p.explanations, "cpr.skill.autofire"), 7 + 6, "REF 7 + Autofire 6");

        // (x2) doubles: Autofire 6 -> 5 frees 2, Handgun 4 -> 3 frees 1.
        let mut lower = example.clone();
        lower.skills.levels.insert("Autofire".to_owned(), 5);
        lower.skills.levels.insert("Handgun".to_owned(), 3);
        assert_eq!(spent(&preview_at(&root(), &lower).unwrap().skills.rows), (83, 3));

        // Raising the Cultural Origin language past its free 4 costs a point a level.
        let language = p.cultural_language.as_ref().unwrap().skill.clone();
        let mut fluent = lower.clone();
        fluent.skills.levels.insert(language.clone(), 6);
        let f = preview_at(&root(), &fluent).unwrap();
        assert!(f.problems.is_empty(), "{:?}", f.problems);
        assert_eq!(spent(&f.skills.rows), (85, 1));
        assert_eq!(f.skills.lines.iter().find(|l| l.key == language).unwrap().level, 6);

        // A skill dropped to 0 leaves the list; a Basic skill cannot be dropped.
        let mut dropped = example.clone();
        dropped.skills.levels.insert("Tracking".to_owned(), 0);
        let d = preview_at(&root(), &dropped).unwrap();
        assert!(d.problems.is_empty(), "{:?}", d.problems);
        assert!(!d.skills.lines.iter().any(|l| l.key == "Tracking"));
        assert_eq!(spent(&d.skills.rows), (82, 4));

        let refused: [(&str, u8, &str, &[&str]); 6] = [
            ("Tactics", 4, REFUSED_SKILLS, &["87 of 86"]),
            ("Melee Weapon", 7, REFUSED_SKILLS, &["Melee Weapon", "above 6"]),
            ("Stealth", 1, REFUSED_SKILLS, &["Stealth", "at least 2"]),
            ("Persuasion", 0, REFUSED_SKILLS, &["Persuasion", "missing"]),
            ("Hacking", 1, REFUSED_SKILLS, &["Hacking"]),
            ("Martial Arts", 1, REFUSED_SKILLS, &["Martial Arts", "specialty"]),
        ];
        for (skill, level, id, words) in refused {
            let mut bad = example.clone();
            bad.skills.levels.insert(skill.to_owned(), level);
            let p = preview_at(&root(), &bad).unwrap();
            assert!(skill_problem(&p, id, words), "{skill} {level}: {:?}", p.problems);
            assert!(p.skills.rows.is_empty(), "{skill} {level}: no spent row for a refused allocation");
        }
        let mut grammar = example.clone();
        grammar.skills.levels.insert("Science (Bio=logy)".to_owned(), 1);
        assert!(skill_problem(&preview_at(&root(), &grammar).unwrap(), REFUSED_SKILL_SPECIALTY, &["Bio=logy"]));
    }

    /// Create saves the whole allocation (with the Cultural Origin language) and the loaded
    /// character carries every level; the response carries the engine's spent row.
    #[test]
    fn create_saves_every_allocated_skill_and_the_loaded_character_has_them() {
        let mut req = seeded("Rockerboy", 8);
        req.lifepath.cultural_language = Some("Japanese".to_owned());
        let characters_root = tempdir("skills");
        let dir = characters_root.join("cpr-skills");
        let response = create_at_root(&root(), &dir, &create(req, "cpr-skills"), "codex-test".to_owned()).unwrap();
        let CprCreateResponse::Saved { explanations, .. } = response else { panic!("{response:?}") };
        assert_eq!(spent(&explanations), (86, 0));
        let choices = CprCharacterStore::load_choices(&dir).unwrap();
        assert_eq!(choices.skills.len(), 21, "20 listed skills + the Cultural Origin language");
        assert!(choices.skills.contains(&("Play Instrument (Guitar)".to_owned(), 6)), "{:?}", choices.skills);
        assert!(choices.skills.contains(&("Language (Japanese)".to_owned(), 4)));
        let loaded = crate::rule_system_adapter::load_saved_character_via_envelope(&dir).unwrap();
        assert_eq!(row(&loaded.explanations, "cpr.skill_level.play_instrument.guitar"), 6);
        assert_eq!(row(&loaded.explanations, "cpr.skill_level.composition"), 6);
        assert_eq!(row(&loaded.explanations, "cpr.skill_level.language.japanese"), 4);
        assert_eq!(row(&loaded.explanations, "cpr.skill_level.archery"), 0);
        std::fs::remove_dir_all(&characters_root).ok();
    }

    // ---- Role Ability rank 4 + sub-choices (E3.5) ----

    fn role_problem(preview: &CprCreationPreview, words: &[&str]) -> bool {
        preview.problems.iter().any(|p| p.id == REFUSED_ROLE_ABILITY && words.iter().all(|w| p.message.contains(w)))
    }

    fn with_role_ability(role: &str, points: &[(&str, u8)], picks: &[(&str, Option<&str>)]) -> CprCreationRequest {
        let mut req = seeded(role, 11);
        req.role_ability = role_ability(points, picks);
        req
    }

    type Allocation<'a> = (&'a [(&'a str, u8)], &'a [(&'a str, Option<&'a str>)]);
    /// A Role, its valid allocation with one row id and value it must give, and its refused
    /// allocation with the words the refusal must hold.
    type RoleCase<'a> = (&'a str, Allocation<'a>, &'a str, i16, Allocation<'a>, &'a [&'a str]);

    /// The acceptance command: for each Role with sub-choices, one valid allocation previews at
    /// Rank 4 with the engine's rows and creates; one refused allocation names the broken rule,
    /// shows no rows and saves nothing.
    #[test]
    fn every_role_with_sub_choices_takes_one_valid_allocation_and_refuses_one() {
        let cases: [RoleCase; 5] = [
            (
                "Solo",
                (&[("Precision Attack", 3), ("Threat Detection", 1)], &[]),
                "cpr.role.solo.combat_awareness.precision_attack",
                3,
                (&[("Damage Deflection", 3)], &[]),
                &["Damage Deflection", "0, 2 or 4"],
            ),
            (
                "Tech",
                (&[("Field Expertise", 4), ("Upgrade Expertise", 2), ("Invention Expertise", 2)], &[]),
                "cpr.role.tech.maker.field_expertise",
                4,
                (&[("Field Expertise", 5), ("Upgrade Expertise", 3)], &[]),
                &["Field Expertise 5", "above 4"],
            ),
            (
                "Medtech",
                (&[("Surgery", 3), ("Medical Tech (Pharmaceuticals)", 1)], &[("Stim", None)]),
                "cpr.role.medtech.surgery_skill",
                6,
                (&[("Surgery", 4)], &[("Stim", None)]),
                &["Pharmaceuticals", "1 drugs picked"],
            ),
            (
                "Nomad",
                (&[], &[("Compact Groundcar", None), ("Heavy Chassis", Some("Compact Groundcar")), ("Roadbike", None), ("NOS", Some("Roadbike"))]),
                "cpr.role.nomad.motorpool_picks_left",
                0,
                (&[], &[("Roadbike", None), ("Armored Chassis", Some("Roadbike"))]),
                &["Armored Chassis", "Rank 5"],
            ),
            (
                "Exec",
                (&[], &[("Company Netrunner", Some("2"))]),
                "cpr.role.exec.team_member_picks_left",
                0,
                (&[], &[("Company Netrunner", Some("9"))]),
                &["1 to 6", "not 9"],
            ),
        ];
        let characters_root = tempdir("role-ability");
        for (role, valid, id, value, refused, words) in cases {
            let ok = with_role_ability(role, valid.0, valid.1);
            let preview = preview_at(&root(), &ok).unwrap();
            assert!(preview.problems.is_empty(), "{role}: {:?}", preview.problems);
            let ability = preview.role_ability.as_ref().expect("a Role Ability");
            assert_eq!(ability.rank, 4, "{role}: every Role starts at Rank 4");
            assert!(ability.menu.points.is_some() || ability.menu.picks.is_some(), "{role} has sub-choices");
            let rank_id = format!("cpr.role.{}.rank", role.to_lowercase());
            assert_eq!(row(&ability.rows, &rank_id), 4, "{role}");
            assert_eq!(row(&ability.rows, id), value, "{role}");
            assert_eq!(row(&preview.explanations, id), value, "{role}: the preview's explanations carry the rows");

            let dir = characters_root.join(format!("cpr-{}", role.to_lowercase()));
            let saved = create_at_root(&root(), &dir, &create(ok, role), "codex-test".to_owned()).unwrap();
            let CprCreateResponse::Saved { explanations, .. } = saved else { panic!("{role}: {saved:?}") };
            assert_eq!(row(&explanations, id), value, "{role}");

            let bad = with_role_ability(role, refused.0, refused.1);
            let preview = preview_at(&root(), &bad).unwrap();
            assert!(role_problem(&preview, words), "{role}: {:?}", preview.problems);
            assert!(preview.explanations.is_empty(), "{role}: no rows while refused");
            assert!(preview.role_ability.as_ref().is_some_and(|a| a.rows.is_empty()), "{role}");
            let blocked_dir = characters_root.join(format!("cpr-{}-refused", role.to_lowercase()));
            let blocked = create_at_root(&root(), &blocked_dir, &create(bad, role), "codex-test".to_owned()).unwrap();
            let CprCreateResponse::Blocked { diagnostics } = blocked else { panic!("{role}: {blocked:?}") };
            assert!(diagnostics.iter().any(|d| d.id == REFUSED_ROLE_ABILITY), "{role}");
            assert!(!blocked_dir.join("envelope.txt").exists(), "{role}: nothing saved");
        }
        std::fs::remove_dir_all(&characters_root).ok();
    }

    /// D-7: the Tech's Maker ranks and the Medtech's Medicine points are all placed at creation;
    /// with none placed the build is refused.
    #[test]
    fn tech_and_medtech_must_place_every_point() {
        for (role, words) in [("Tech", ["Maker specialty ranks", "0 of 8"]), ("Medtech", ["Medicine points", "0 of 4"])] {
            let preview = preview_at(&root(), &with_role_ability(role, &[], &[])).unwrap();
            assert!(role_problem(&preview, &words), "{role}: {:?}", preview.problems);
        }
    }

    /// Rockerboy, Netrunner, Media, Lawman and Fixer start at Rank 4 with no sub-choice; a sent
    /// choice is refused.
    #[test]
    fn roles_without_sub_choices_start_at_rank_4_and_refuse_a_choice() {
        for role in ["Rockerboy", "Netrunner", "Media", "Lawman", "Fixer"] {
            let preview = preview_at(&root(), &seeded(role, 3)).unwrap();
            assert!(preview.problems.is_empty(), "{role}: {:?}", preview.problems);
            let ability = preview.role_ability.as_ref().expect("a Role Ability");
            assert!(ability.menu.points.is_none() && ability.menu.picks.is_none(), "{role}");
            assert_eq!(row(&preview.explanations, &format!("cpr.role.{}.rank", role.to_lowercase())), 4);
            let preview = preview_at(&root(), &with_role_ability(role, &[("Surgery", 1)], &[])).unwrap();
            assert!(role_problem(&preview, &["no points"]), "{role}: {:?}", preview.problems);
        }
    }

    /// A name the saved grammar cannot hold is refused before the engine sees it.
    #[test]
    fn a_role_choice_the_saved_grammar_cannot_hold_is_refused() {
        let preview = preview_at(&root(), &with_role_ability("Solo", &[("Threat=Detection", 1)], &[])).unwrap();
        assert!(role_problem(&preview, &["Threat=Detection"]), "{:?}", preview.problems);
        let preview = preview_at(&root(), &with_role_ability("Nomad", &[], &[("Roadbike", Some("a@b"))])).unwrap();
        assert!(role_problem(&preview, &["a@b"]), "{:?}", preview.problems);
    }

    /// Create saves the Role at Rank 4 and its sub-choices; the loaded character carries the
    /// Role's rank rows and the choices read back unchanged.
    #[test]
    fn create_saves_the_role_at_rank_4_and_its_sub_choices() {
        use codex::rules_core::cpr::input::{CprRoleChoices, CprRolePick};
        use codex::rules_core::cpr::roles::CprRole;
        let req = with_role_ability("Nomad", &[], &[("Jetski", None), ("NOS", Some("Jetski"))]);
        let characters_root = tempdir("role-save");
        let dir = characters_root.join("cpr-nomad");
        let saved = create_at_root(&root(), &dir, &create(req, "cpr-nomad"), "codex-test".to_owned()).unwrap();
        assert!(matches!(saved, CprCreateResponse::Saved { .. }), "{saved:?}");
        assert_eq!(CprCharacterStore::load_choices(&dir).unwrap().role, Some((CprRole::Nomad, 4)));
        assert_eq!(
            CprCharacterStore::load_role_choices(&dir).unwrap(),
            CprRoleChoices {
                points: Vec::new(),
                picks: vec![
                    CprRolePick { name: "Jetski".to_owned(), on: None },
                    CprRolePick { name: "NOS".to_owned(), on: Some("Jetski".to_owned()) },
                ],
            }
        );
        let loaded = crate::rule_system_adapter::load_saved_character_via_envelope(&dir).unwrap();
        assert_eq!(row(&loaded.explanations, "cpr.role.nomad.rank"), 4);
        assert_eq!(row(&loaded.explanations, "cpr.role.nomad.vehicle_familiarity_bonus"), 4);
        std::fs::remove_dir_all(&characters_root).ok();
    }
}
