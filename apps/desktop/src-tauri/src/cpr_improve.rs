//! Cyberpunk RED improvement: spending Improvement Points on Skill Levels and the Role
//! Ability Rank after creation (Cyberpunk RED core rulebook p. 410-411), the improve dialog's
//! two commands.
//!
//! - `preview_cyberpunk_improvement` answers with the dialog's options, read from the saved
//!   character: every Master Skill List skill (and each held specialty) at its Level with the
//!   engine's price of its next Level, the Role at its Rank with the price of its next Rank;
//!   and, for the plan the player has built, one `cpr.improve.purchase.<i>.cost` row per
//!   purchase, the IP available, the plan's cost and the IP left, or the engine's refusal.
//! - `apply_cyberpunk_improvement` runs the same plan, saves the new Levels, Rank and IP box
//!   through the store (`CprCharacterStore::save_choices`) and answers with the reloaded sheet.
//!
//! Every price is `codex::rules_core::cpr::improvement` over the `ip_cost` corpus (Typical
//! 20 x the next Level, Difficult (x2) 40 x, Role Ability 60 x the next Rank); a purchase is one
//! Level/Rank up, so a skipped Level is refused (p. 411). The IP entered in the dialog are added
//! to the IP earned (the GM's award, entered, not computed); spending more than earned minus
//! spent is refused. A refused plan writes nothing. A Role switch, cyberware installs, therapy
//! and Reputation are not part of this plan.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use codex::rules_core::cpr::improvement::{CprIpCostCatalog, CprIpTable, CprRoleHistory};
use codex::rules_core::cpr::input::{CprImprovementPoints, CprSheetChoices};
use codex::rules_core::cpr::sheet::{compose_sheet, CprSheetCatalogs};
use codex::rules_core::cpr::skills::{slug, CprSkill, CprSkillCatalog};
use codex::rules_core::cpr::store::CprCharacterStore;
use codex::rules_core::game_system::{runtime_repo_root, GameSystem};
use codex::rules_core::pilot_compute::ComputationExplanation;

use crate::character_hub::{ExplanationDto, LoadSavedCharacterResponse};
use crate::cpr_adapter::CyberpunkAdapter;
use crate::rule_system_adapter::RuleSystemAdapter;

/// The CPR corpus the running app reads (packaged-root aware).
fn corpus_root() -> PathBuf {
    GameSystem::CyberpunkRed.package_roots(&runtime_repo_root()).corpus
}

/// One purchase of the plan: a Skill to `to` (one Level up) or the Role Ability to Rank `to`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CprPurchase {
    /// A Skill by its key as the sheet names it (`Handgun`, `Language (Streetslang)`).
    Skill { key: String, to: u8 },
    /// The current Role's Ability Rank.
    RoleRank { to: u8 },
}

/// The improve dialog's plan: the IP entered (added to the IP earned) and the purchases, in order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprImproveRequest {
    pub character_id: String,
    pub ip_awarded: u32,
    pub purchases: Vec<CprPurchase>,
}

/// One Skill the dialog offers: its key, Level, (x2) mark and the engine's price of the next
/// Level (none at Level 10). A skill that needs a specialty is offered bare (a new specialty at
/// Level 0, `specialty_required`) and once per specialty the character holds.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CprImproveSkillOption {
    pub key: String,
    pub name: String,
    pub category: String,
    pub x2: bool,
    pub specialty_required: bool,
    pub level: u8,
    pub next: Option<ExplanationDto>,
}

/// The character's Role at its Rank and the engine's price of the next Rank (none at 10).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CprImproveRoleOption {
    pub name: String,
    pub rank: u8,
    pub next: Option<ExplanationDto>,
}

/// `preview_cyberpunk_improvement`: the options and the plan's rows, or why the plan is refused.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CprImprovePreview {
    pub skills: Vec<CprImproveSkillOption>,
    pub role: Option<CprImproveRoleOption>,
    pub rows: Vec<ExplanationDto>,
    pub refusal: Option<String>,
}

fn dto(row: ComputationExplanation) -> ExplanationDto {
    ExplanationDto { id: row.id, value: row.value, detail: row.detail }
}

fn table_of(skill: &CprSkill) -> CprIpTable {
    if skill.x2 {
        CprIpTable::DifficultSkill
    } else {
        CprIpTable::TypicalSkill
    }
}

fn ip_number(label: &str, value: u32) -> Result<i16, String> {
    i16::try_from(value).map_err(|_| format!("{label} {value} IP is more than the sheet's IP box holds"))
}

/// The index in `choices.skills` of the held skill `key` names (same skill, same specialty).
fn held_index(catalog: &CprSkillCatalog, choices: &CprSheetChoices, key: &str) -> Result<Option<usize>, String> {
    let (skill, specialty) = catalog.resolve(key)?;
    for (i, (held, _)) in choices.skills.iter().enumerate() {
        let (h_skill, h_specialty) = catalog.resolve(held)?;
        if h_skill.name == skill.name && h_specialty == specialty {
            return Ok(Some(i));
        }
    }
    Ok(None)
}

/// A priced plan: the character's choices after it, and the dialog's rows.
type PricedPlan = (CprSheetChoices, Vec<ComputationExplanation>);

/// The `cpr.improve.ip.available` row: IP earned + IP entered - IP spent.
fn available_row(choices: &CprSheetChoices, ip_awarded: u32) -> Result<ComputationExplanation, String> {
    let ip = choices.improvement_points.unwrap_or_default();
    let earned = ip
        .earned
        .checked_add(ip_awarded)
        .ok_or_else(|| format!("{ip_awarded} IP entered is more than the sheet's IP box holds"))?;
    let available = earned.saturating_sub(ip.spent);
    Ok(ComputationExplanation {
        id: "cpr.improve.ip.available".to_owned(),
        value: ip_number("available", available)?,
        detail: format!(
            "Improvement Points available {available} = {} earned + {ip_awarded} entered - {} spent (p. 410-411)",
            ip.earned, ip.spent
        ),
    })
}

/// The plan priced and applied to `choices`: the new choices and the dialog's rows. Refused with
/// the engine's reason: a skipped or non-increasing Level/Rank, a Level past 10, a key not on
/// the Master Skill List (or missing its specialty), no Role, an empty plan, or an overspend.
fn plan(
    costs: &CprIpCostCatalog,
    skills: &CprSkillCatalog,
    choices: &CprSheetChoices,
    request: &CprImproveRequest,
) -> Result<PricedPlan, String> {
    if request.purchases.is_empty() && request.ip_awarded == 0 {
        return Err("nothing to spend: enter IP or choose a Skill Level or Role Rank to buy".to_owned());
    }
    let ip = choices.improvement_points.unwrap_or_default();
    let earned = ip
        .earned
        .checked_add(request.ip_awarded)
        .ok_or_else(|| format!("{} IP entered is more than the sheet's IP box holds", request.ip_awarded))?;
    let available = earned.saturating_sub(ip.spent);
    let mut next = choices.clone();
    let mut rows = Vec::new();
    let mut total: u32 = 0;
    for (i, purchase) in request.purchases.iter().enumerate() {
        let (label, priced) = match purchase {
            CprPurchase::Skill { key, to } => {
                let (skill, _) = skills.resolve(key)?;
                let held = held_index(skills, &next, key)?;
                let from = held.map_or(0, |h| next.skills[h].1);
                let priced = costs.improve(table_of(skill), from, *to).map_err(|r| format!("{}: {r}", key.trim()))?;
                match held {
                    Some(h) => next.skills[h].1 = *to,
                    None => next.skills.push((key.trim().to_owned(), *to)),
                }
                (key.trim().to_owned(), priced)
            }
            CprPurchase::RoleRank { to } => {
                let (role, rank) = next
                    .role
                    .ok_or("the character has no Role, so it has no Role Ability Rank to raise")?;
                let (_, priced) = costs
                    .raise_role_rank(&CprRoleHistory::new(role, rank), role, *to)
                    .map_err(|r| r.to_string())?;
                next.role = Some((role, *to));
                (format!("{} Role Ability", role.name()), priced)
            }
        };
        total += u32::try_from(priced.value).map_err(|_| format!("{label}: price {} is not a cost", priced.value))?;
        rows.push(ComputationExplanation {
            id: format!("cpr.improve.purchase.{i}.cost"),
            value: priced.value,
            detail: format!("{label}: {}", priced.detail),
        });
    }
    if total > available {
        return Err(format!(
            "Overspend: these improvements cost {total} IP, but {available} IP are available ({} earned + {} entered - {} spent; p. 411)",
            ip.earned, request.ip_awarded, ip.spent
        ));
    }
    let spent = ip.spent + total;
    next.improvement_points = Some(CprImprovementPoints { earned, spent });
    rows.push(available_row(choices, request.ip_awarded)?);
    rows.push(ComputationExplanation {
        id: "cpr.improve.ip.cost".to_owned(),
        value: ip_number("cost", total)?,
        detail: format!("Improvement Points spent on this plan: {total}, the sum of its purchases (p. 411)"),
    });
    rows.push(ComputationExplanation {
        id: "cpr.improve.ip.remaining".to_owned(),
        value: ip_number("remaining", available - total)?,
        detail: format!("Improvement Points left {} = {available} available - {total} spent", available - total),
    });
    Ok((next, rows))
}

/// The dialog's options for the saved `choices`.
fn options(
    costs: &CprIpCostCatalog,
    skills: &CprSkillCatalog,
    choices: &CprSheetChoices,
) -> Result<(Vec<CprImproveSkillOption>, Option<CprImproveRoleOption>), String> {
    let next_row = |id: String, table: CprIpTable, level: u8| -> Option<ExplanationDto> {
        costs.improve(table, level, level + 1).ok().map(|mut row| {
            row.id = id;
            dto(row)
        })
    };
    let mut out = Vec::new();
    for skill in skills.skills() {
        let table = table_of(skill);
        let bare_level = if skill.specialty_required {
            0
        } else {
            match held_index(skills, choices, &skill.name)? {
                Some(h) => choices.skills[h].1,
                None => 0,
            }
        };
        out.push(CprImproveSkillOption {
            key: skill.name.clone(),
            name: skill.name.clone(),
            category: skill.category.clone(),
            x2: skill.x2,
            specialty_required: skill.specialty_required,
            level: bare_level,
            next: next_row(format!("cpr.improve.skill.{}.next", slug(&skill.name)), table, bare_level),
        });
        if skill.specialty_required {
            for (key, level) in &choices.skills {
                let (held, specialty) = skills.resolve(key)?;
                if held.name == skill.name {
                    let specialty = specialty.unwrap_or_default();
                    out.push(CprImproveSkillOption {
                        key: key.clone(),
                        name: skill.name.clone(),
                        category: skill.category.clone(),
                        x2: skill.x2,
                        specialty_required: false,
                        level: *level,
                        next: next_row(
                            format!("cpr.improve.skill.{}.{}.next", slug(&skill.name), slug(&specialty)),
                            table,
                            *level,
                        ),
                    });
                }
            }
        }
    }
    let role = choices.role.map(|(role, rank)| CprImproveRoleOption {
        name: role.name().to_owned(),
        rank,
        next: costs.raise_role_rank(&CprRoleHistory::new(role, rank), role, rank + 1).ok().map(|(_, mut row)| {
            row.id = format!("cpr.improve.role.{}.next", role.id());
            dto(row)
        }),
    });
    Ok((out, role))
}

/// Load the character at `root`, price the plan, and check the new choices compose (the
/// engine's sheet check); the new choices and the plan's rows, or the refusal.
fn priced_at_root(
    corpus: &Path,
    root: &Path,
    request: &CprImproveRequest,
) -> Result<(CprSheetChoices, Result<PricedPlan, String>), String> {
    let character = CprCharacterStore::load(root)?;
    let choices = CprCharacterStore::load_choices(root)?;
    let costs = CprIpCostCatalog::load(corpus)?;
    let skills = CprSkillCatalog::load(corpus)?;
    let planned = plan(&costs, &skills, &choices, request).and_then(|(next, rows)| {
        compose_sheet(&CprSheetCatalogs::load(corpus)?, &character.input, &next)?;
        Ok((next, rows))
    });
    Ok((choices, planned))
}

/// The dialog's options and the plan's rows (or refusal) for the CPR character at `root`. Writes
/// nothing.
pub(crate) fn preview_at_root(corpus: &Path, root: &Path, request: &CprImproveRequest) -> Result<CprImprovePreview, String> {
    let (choices, planned) = priced_at_root(corpus, root, request)?;
    let (skills, role) = options(&CprIpCostCatalog::load(corpus)?, &CprSkillCatalog::load(corpus)?, &choices)?;
    // A refused plan still shows the IP available, so the dialog can say what there is to spend.
    let (rows, refusal) = match planned {
        Ok((_, rows)) => (rows.into_iter().map(dto).collect(), None),
        Err(reason) => (available_row(&choices, request.ip_awarded).into_iter().map(dto).collect(), Some(reason)),
    };
    Ok(CprImprovePreview { skills, role, rows, refusal })
}

/// Spend the plan on the CPR character at `root`: the new Levels, Rank and IP box are saved
/// through the store and the reloaded sheet is the answer. A refused plan writes nothing.
pub(crate) fn apply_at_root(corpus: &Path, root: &Path, request: &CprImproveRequest) -> Result<LoadSavedCharacterResponse, String> {
    let (_, planned) = priced_at_root(corpus, root, request)?;
    let (next, _) = planned?;
    CprCharacterStore::save_choices(&next, root)?;
    CyberpunkAdapter.load_saved_character(root)
}

/// The improve dialog's options and its plan priced by the engine (no save).
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn preview_cyberpunk_improvement(app: tauri::AppHandle, request: CprImproveRequest) -> Result<CprImprovePreview, String> {
    let root = crate::character_hub::resolve_character_root(&app, &request.character_id)?;
    preview_at_root(&corpus_root(), &root, &request)
}

/// Spend Improvement Points on a Cyberpunk RED character; answers with the reloaded sheet.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn apply_cyberpunk_improvement(app: tauri::AppHandle, request: CprImproveRequest) -> Result<LoadSavedCharacterResponse, String> {
    let root = crate::character_hub::resolve_character_root(&app, &request.character_id)?;
    apply_at_root(&corpus_root(), &root, &request)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    use codex::rules_core::cpr::input::{CprCharacterInput, CprImprovementPoints, CprSheetChoices, CprStat, CprStats};
    use codex::rules_core::cpr::roles::CprRole;
    use codex::rules_core::cpr::store::{CprCharacterStore, CprSavedCharacter, CPR_SAVED_CHARACTER_SCHEMA_VERSION};
    use codex::rules_core::game_system::GameSystem;

    fn corpus() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(GameSystem::CyberpunkRed.corpus_relative())
    }

    fn tempdir(label: &str) -> PathBuf {
        let unique = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("time").as_nanos();
        let path = std::env::temp_dir().join(format!("codex-cpr-improve-{label}-{}-{unique}", std::process::id()));
        std::fs::create_dir_all(&path).expect("temp dir");
        path
    }

    /// A saved Solo at Rank 4 with Handgun 4, Language (Streetslang) 2, and `ip` in its box.
    fn saved_solo(label: &str, ip: Option<CprImprovementPoints>) -> PathBuf {
        let root = tempdir(label).join("cpr-improve");
        let mut stats = CprStats::default();
        for (stat, value) in CprStat::ALL.into_iter().zip([7, 8, 6, 5, 4, 3, 2, 6, 5, 8]) {
            stats.set(stat, value);
        }
        let character = CprSavedCharacter {
            character_id: "cpr-improve".to_owned(),
            revision_id: "cpr-improve.rev.1".to_owned(),
            saved_at: "2026-10-15T00:00:00Z".to_owned(),
            schema_version: CPR_SAVED_CHARACTER_SCHEMA_VERSION,
            app_or_runtime_version: "codex-dev".to_owned(),
            display_label: "Improve Solo".to_owned(),
            input: CprCharacterInput { stats },
        };
        CprCharacterStore::save(&character, &root).expect("saves");
        let choices = CprSheetChoices {
            role: Some((CprRole::Solo, 4)),
            skills: vec![("Handgun".to_owned(), 4), ("Language (Streetslang)".to_owned(), 2)],
            improvement_points: ip,
            ..CprSheetChoices::default()
        };
        CprCharacterStore::save_choices(&choices, &root).expect("saves the choices");
        root
    }

    fn skill(key: &str, to: u8) -> CprPurchase {
        CprPurchase::Skill { key: key.to_owned(), to }
    }

    fn request(ip_awarded: u32, purchases: Vec<CprPurchase>) -> CprImproveRequest {
        CprImproveRequest { character_id: "cpr-improve".to_owned(), ip_awarded, purchases }
    }

    fn row(rows: &[ExplanationDto], id: &str) -> i16 {
        rows.iter().find(|r| r.id == id).unwrap_or_else(|| panic!("no row {id} in {rows:?}")).value
    }

    fn choices_text(root: &Path) -> String {
        std::fs::read_to_string(root.join("cpr_sheet_choices.txt")).expect("choices file")
    }

    /// Entered IP buys Handgun 4 -> 5 (Typical: 20 x 5 = 100), Autofire 0 -> 1 ((x2): 40 x 1 =
    /// 40) and Solo Rank 4 -> 5 (60 x 5 = 300): 440 of 500. The save goes through the store and
    /// the reloaded sheet shows the new level, base, Rank and IP box.
    #[test]
    fn entered_ip_buys_skill_levels_and_a_role_rank_at_the_engine_prices_and_the_sheet_rereads() {
        let root = saved_solo("buys", None);
        let plan = request(500, vec![skill("Handgun", 5), skill("Autofire", 1), CprPurchase::RoleRank { to: 5 }]);

        let preview = preview_at_root(&corpus(), &root, &plan).expect("previews");
        assert_eq!(preview.refusal, None);
        assert_eq!(row(&preview.rows, "cpr.improve.purchase.0.cost"), 100);
        assert_eq!(row(&preview.rows, "cpr.improve.purchase.1.cost"), 40);
        assert_eq!(row(&preview.rows, "cpr.improve.purchase.2.cost"), 300);
        assert_eq!(row(&preview.rows, "cpr.improve.ip.available"), 500);
        assert_eq!(row(&preview.rows, "cpr.improve.ip.cost"), 440);
        assert_eq!(row(&preview.rows, "cpr.improve.ip.remaining"), 60);
        let before = choices_text(&root);

        let loaded = apply_at_root(&corpus(), &root, &plan).expect("applies");
        assert_ne!(choices_text(&root), before, "the store holds the new choices");
        let sheet = |id: &str| row(&loaded.explanations, id);
        assert_eq!(sheet("cpr.skill_level.handgun"), 5);
        assert_eq!(sheet("cpr.skill.handgun"), 8 + 5, "REF 8 + level 5");
        assert_eq!(sheet("cpr.skill_level.autofire"), 1);
        assert_eq!(sheet("cpr.role.solo.rank"), 5);
        assert_eq!((sheet("cpr.ip.current"), sheet("cpr.ip.total")), (60, 500));
        let saved = CprCharacterStore::load_choices(&root).expect("reloads");
        assert_eq!(saved.improvement_points, Some(CprImprovementPoints { earned: 500, spent: 440 }));
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    /// Two one-step purchases of one skill in a plan are two Levels (4 -> 5 -> 6: 100 + 120);
    /// a purchase that skips a Level (4 -> 6) is refused with the p. 411 reason and nothing is
    /// written.
    #[test]
    fn a_skipped_level_is_refused_and_nothing_is_saved_while_two_single_steps_are_priced() {
        let root = saved_solo("skip", Some(CprImprovementPoints { earned: 300, spent: 0 }));
        let before = choices_text(&root);

        let skipped = request(0, vec![skill("Handgun", 6)]);
        let preview = preview_at_root(&corpus(), &root, &skipped).expect("previews");
        let reason = preview.refusal.expect("refused");
        assert!(reason.contains("skips a Level") && reason.contains("p. 411"), "{reason}");
        assert!(apply_at_root(&corpus(), &root, &skipped).unwrap_err().contains("skips a Level"));
        assert_eq!(choices_text(&root), before, "a refused plan writes nothing");

        let stepped = request(0, vec![skill("Handgun", 5), skill("Handgun", 6)]);
        let preview = preview_at_root(&corpus(), &root, &stepped).expect("previews");
        assert_eq!(preview.refusal, None);
        assert_eq!(row(&preview.rows, "cpr.improve.ip.cost"), 220);
        let loaded = apply_at_root(&corpus(), &root, &stepped).expect("applies");
        assert_eq!(row(&loaded.explanations, "cpr.skill_level.handgun"), 6);
        assert_eq!(row(&loaded.explanations, "cpr.ip.current"), 80);
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    /// Spending more than the IP available (earned + entered - spent) is refused, naming both
    /// figures; nothing is written.
    #[test]
    fn overspend_is_refused_and_nothing_is_saved() {
        let root = saved_solo("overspend", Some(CprImprovementPoints { earned: 100, spent: 60 }));
        let before = choices_text(&root);
        // 40 left + 50 entered = 90 < Handgun 5 at 100.
        let plan = request(50, vec![skill("Handgun", 5)]);
        let reason = preview_at_root(&corpus(), &root, &plan).expect("previews").refusal.expect("refused");
        assert!(reason.contains("100 IP") && reason.contains("90 IP"), "{reason}");
        assert!(apply_at_root(&corpus(), &root, &plan).unwrap_err().contains("90 IP"));
        assert_eq!(choices_text(&root), before);
        // Exactly the available IP is not an overspend.
        let exact = request(60, vec![skill("Handgun", 5)]);
        let loaded = apply_at_root(&corpus(), &root, &exact).expect("100 of 100 applies");
        assert_eq!(row(&loaded.explanations, "cpr.ip.current"), 0);
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    /// A new specialty starts at Level 0 (Language (Spanish) 0 -> 1 = 20); a skill that needs a
    /// specialty and names none, a name not on the Master Skill List, a Role Rank past 10 and an
    /// empty plan are refused.
    #[test]
    fn new_specialties_are_priced_and_bad_purchases_are_refused() {
        let root = saved_solo("specialty", Some(CprImprovementPoints { earned: 1000, spent: 0 }));
        let preview = preview_at_root(&corpus(), &root, &request(0, vec![skill("Language (Spanish)", 1)])).expect("previews");
        assert_eq!(preview.refusal, None);
        assert_eq!(row(&preview.rows, "cpr.improve.purchase.0.cost"), 20);
        let refusal = |plan: CprImproveRequest| preview_at_root(&corpus(), &root, &plan).expect("previews").refusal.expect("refused");
        assert!(refusal(request(0, vec![skill("Language", 1)])).contains("needs a specialty"));
        assert!(refusal(request(0, vec![skill("Basket Weaving", 1)])).contains("Master Skill List"));
        assert!(refusal(request(0, vec![CprPurchase::RoleRank { to: 6 }])).contains("skips a Rank"));
        assert!(refusal(request(0, vec![])).contains("nothing to spend"));
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    /// The dialog's options: every Master List skill (66) plus each held specialty, each with
    /// its Level and the engine's next-Level price on its table; the Role with its Rank and the
    /// next-Rank price; and the IP box as saved.
    #[test]
    fn the_preview_lists_every_skill_and_the_role_with_their_next_prices() {
        let root = saved_solo("options", Some(CprImprovementPoints { earned: 40, spent: 20 }));
        let preview = preview_at_root(&corpus(), &root, &request(0, vec![skill("Handgun", 5)])).expect("previews");
        let bare = preview.skills.iter().filter(|s| !s.key.contains('(')).count();
        assert_eq!(bare, 66, "one option per Master List skill");
        let find = |key: &str| preview.skills.iter().find(|s| s.key == key).unwrap_or_else(|| panic!("no option {key}"));
        let handgun = find("Handgun");
        assert_eq!((handgun.level, handgun.x2), (4, false));
        assert_eq!(handgun.next.as_ref().map(|r| r.value), Some(100));
        let autofire = find("Autofire");
        assert_eq!((autofire.level, autofire.x2), (0, true));
        assert_eq!(autofire.next.as_ref().map(|r| r.value), Some(40));
        let streetslang = find("Language (Streetslang)");
        assert_eq!(streetslang.level, 2);
        assert_eq!(streetslang.next.as_ref().map(|r| r.value), Some(60));
        assert!(find("Language").specialty_required);
        let role = preview.role.as_ref().expect("a Solo");
        assert_eq!((role.name.as_str(), role.rank), ("Solo", 4));
        assert_eq!(role.next.as_ref().map(|r| r.value), Some(300));
        assert_eq!(row(&preview.rows, "cpr.improve.ip.available"), 20);
        // A plan never changes the store.
        assert_eq!(CprCharacterStore::load_choices(&root).unwrap().skills[0], ("Handgun".to_owned(), 4));
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    /// A character saved without a Role (the creation STAT step) has no Rank to raise.
    #[test]
    fn a_character_without_a_role_cannot_raise_a_rank() {
        let root = saved_solo("norole", Some(CprImprovementPoints { earned: 100, spent: 0 }));
        let mut choices = CprCharacterStore::load_choices(&root).unwrap();
        choices.role = None;
        CprCharacterStore::save_choices(&choices, &root).unwrap();
        let reason = preview_at_root(&corpus(), &root, &request(0, vec![CprPurchase::RoleRank { to: 5 }]))
            .expect("previews")
            .refusal
            .expect("refused");
        assert!(reason.contains("no Role"), "{reason}");
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    /// The wire shape `cprImprove.ts` sends: `{ characterId, ipAwarded, purchases: [{ kind:
    /// 'skill', key, to } | { kind: 'roleRank', to }] }`.
    #[test]
    fn the_request_deserialises_from_the_frontend_wire_shape() {
        let wire = serde_json::json!({
            "characterId": "cpr-improve",
            "ipAwarded": 200,
            "purchases": [{ "kind": "skill", "key": "Handgun", "to": 5 }, { "kind": "roleRank", "to": 5 }]
        });
        let parsed: CprImproveRequest = serde_json::from_value(wire).expect("deserialises");
        assert_eq!(parsed.ip_awarded, 200);
        assert_eq!(parsed.purchases, vec![skill("Handgun", 5), CprPurchase::RoleRank { to: 5 }]);
    }
}
