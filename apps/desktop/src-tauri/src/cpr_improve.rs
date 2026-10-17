//! Cyberpunk RED improvement: spending Improvement Points on Skill Levels and the Role
//! Ability Rank after creation (Cyberpunk RED core rulebook p. 410-411), and changing Roles
//! (p. 143), the improve dialog's two commands.
//!
//! - `preview_cyberpunk_improvement` answers with the dialog's options, read from the saved
//!   character: every Master Skill List skill (and each held specialty) at its Level with the
//!   engine's price of its next Level, the Role at its Rank with the price of its next Rank,
//!   each Role the character does not hold with the A-1 price of its Rank 1 (or the p. 143 lock);
//!   and, for the plan the player has built, one `cpr.improve.purchase.<i>.cost` row per
//!   purchase, the IP available, the plan's cost and the IP left, or the engine's refusal.
//! - `apply_cyberpunk_improvement` runs the same plan, saves the new Levels, Ranks, Roles and IP box
//!   through the store (`CprCharacterStore::save_choices`) and answers with the reloaded sheet.
//!
//! Every price is `codex::rules_core::cpr::improvement` over the `ip_cost` corpus (Typical
//! 20 x the next Level, Difficult (x2) 40 x, Role Ability 60 x the next Rank); a purchase is one
//! Level/Rank up, so a skipped Level is refused (p. 411). The IP entered in the dialog are added
//! to the IP earned (the GM's award, entered, not computed); spending more than earned minus
//! spent is refused. A refused plan writes nothing.
//!
//! A Role switch (`roleSwitch`) buys Rank 1 in a Role the character does not hold, at the Role
//! Ability Rank table's Rank 1 price (A-1, 60 IP), only while the latest Role is at Rank 4 or
//! more (p. 143); the new Role becomes the latest, the earlier Roles keep their Ranks and the sheet
//! shows each Role's rows. A Rank purchase raises the latest Role.
//!
//! A cyberware install (`cyberware`) after Character Generation costs no IP (its Eurobuck price
//! and install tier are printed, not computed): it takes the Humanity Loss the player entered, or
//! one rolled on the piece's dice (p. 111) by the creation roller (`CprRoller`) with the
//! request's seed, and the engine refuses a loss outside the dice range (A-3) and an install over
//! its container's option slots (D-11). The preview answers with the seed it used and the
//! Humanity and EMP before and after the plan; an apply sent with that seed rolls the same loss.
//!
//! Therapy (`therapy`, p. 230) costs no IP either (its Eurobuck price, Medical Tech DV and
//! materials are printed): Standard Humanity Loss regains 2d6 and Extreme Humanity Loss 4d6,
//! entered or rolled with the request's seed like an install's loss; the engine refuses an
//! entered regain outside the dice and stops the regain at the D-5 maximum Humanity (10 x EMP
//! less 2 per piece of cyberware and 4 per piece of borgware), so a restore above the cap
//! stops at the cap. The regain is saved as `humanity.regained`. Addiction therapy regains no
//! Humanity: the dialog prints its effect, and as a purchase it is refused.
//!
//! A Reputation change (`reputation`, p. 193-194) costs no IP: it records the event the GM
//! awarded, its Level 1 to 10 (outside the table refused), deed or cowardice, and what it was for.
//! The engine's `CprReputation::record` decides whether it becomes the defining event: a new event
//! replaces the old only if its Level is higher, whatever its kind, so a cowardice event of a
//! higher Level replaces a deed and Facedowns then treat the Level as negative (p. 194). Every
//! event is saved (`reputation = <level> <deed|cowardice>: <description>`); the preview answers
//! with the Level after each event and the Reputation and Facedown rows before and after the plan.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use codex::rules_core::cpr::cyberware::{
    humanity_after_loss, humanity_after_therapy, CprCyberwareCatalog, CprCyberwareInstall, CprHumanityLossDice,
};
use codex::rules_core::cpr::improvement::{CprIpCostCatalog, CprIpTable, CprRoleHistory};
use codex::rules_core::cpr::input::{CprCharacterInput, CprImprovementPoints, CprSheetChoices, CprStat};
use codex::rules_core::cpr::reputation::{CprReputation, CprReputationEvent, CprReputationKind, CprReputationOutcome};
use codex::rules_core::cpr::roles::CprRole;
use codex::rules_core::cpr::sheet::{compose_sheet, CprSheetCatalogs};
use codex::rules_core::cpr::skills::{slug, CprSkill, CprSkillCatalog};
use codex::rules_core::cpr::store::CprCharacterStore;
use codex::rules_core::game_system::{runtime_repo_root, GameSystem};
use codex::rules_core::pilot_compute::ComputationExplanation;

use crate::character_hub::{ExplanationDto, LoadSavedCharacterResponse};
use crate::cpr_adapter::CyberpunkAdapter;
use crate::cpr_creation::CprRoller;
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
    /// The latest Role's Ability Rank.
    RoleRank { to: u8 },
    /// Rank 1 in a Role the character does not hold, by its name (`Nomad`); p. 143, A-1.
    RoleSwitch { role: String },
    /// A piece of cyberware by its catalog name, with the Humanity Loss the player entered, or
    /// none to roll it on the piece's dice (p. 111, A-3). It costs no IP.
    #[serde(rename_all = "camelCase")]
    Cyberware { name: String, humanity_loss: Option<i16> },
    /// A Humanity therapy by its p. 230 name, with the Humanity regain the player entered, or
    /// none to roll it on the therapy's dice. It costs no IP.
    Therapy { name: String, regained: Option<i16> },
    /// A Reputation event the GM awarded (p. 193): its Level 1 to 10, whether it was a deed or
    /// cowardice, and what it was for. It costs no IP.
    Reputation { level: u8, event: CprReputationEventKind, description: String },
}

/// What a Reputation event was for, as the dialog sends it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CprReputationEventKind {
    Deed,
    Cowardice,
}

impl CprReputationEventKind {
    fn kind(self) -> CprReputationKind {
        match self {
            CprReputationEventKind::Deed => CprReputationKind::Deed,
            CprReputationEventKind::Cowardice => CprReputationKind::Cowardice,
        }
    }

    fn word(self) -> &'static str {
        match self {
            CprReputationEventKind::Deed => "deed",
            CprReputationEventKind::Cowardice => "cowardice",
        }
    }
}

/// The Reputation `events` resolve to, in the order recorded (p. 193: a later event replaces the
/// defining one only when its Level is higher).
fn resolved_reputation(events: &[CprReputationEvent]) -> Result<CprReputation, String> {
    events.iter().try_fold(CprReputation::default(), |held, event| {
        held.record(event.clone()).map(|(next, _)| next).map_err(|r| r.to_string())
    })
}

/// Record a Reputation event on `next`: the engine's Level after it and the row's text.
fn record_reputation(
    next: &mut CprSheetChoices,
    level: u8,
    event: CprReputationEventKind,
    description: &str,
) -> Result<(i16, String), String> {
    let description = description.trim();
    if description.is_empty() {
        return Err("a Reputation event needs what the event was for (p. 193)".to_owned());
    }
    if description.contains(['\n', '\r']) {
        return Err("a Reputation event's description must be one line".to_owned());
    }
    let held = resolved_reputation(&next.reputation)?;
    let recorded = CprReputationEvent { level, kind: event.kind(), description: description.to_owned() };
    let (after, outcome) = held.record(recorded.clone()).map_err(|r| r.to_string())?;
    next.reputation.push(recorded);
    let entry = format!("Level {level} {}: {description}", event.word());
    let detail = match outcome {
        CprReputationOutcome::Replaced => {
            let negative = match event {
                CprReputationEventKind::Deed => String::new(),
                CprReputationEventKind::Cowardice => {
                    format!("; it is for cowardice, so Facedowns treat the Level as -{level} (p. 194)")
                }
            };
            format!(
                "{entry} is the defining Reputation Event, replacing Reputation {} (p. 193){negative}",
                held.level()
            )
        }
        CprReputationOutcome::Kept => format!(
            "{entry} is recorded but does not replace the defining Reputation Level {}: a new deed's Reputation will only replace the old one if the Level is higher (p. 193)",
            held.level()
        ),
    };
    Ok((i16::from(after.level()), detail))
}

/// The improve dialog's plan: the IP entered (added to the IP earned) and the purchases, in order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprImproveRequest {
    pub character_id: String,
    pub ip_awarded: u32,
    pub purchases: Vec<CprPurchase>,
    /// The roller's seed for the Humanity Loss of each install left blank; absent, a fresh seed
    /// is drawn and the preview answers with it.
    #[serde(default)]
    pub seed: Option<u32>,
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

/// A Role the character does not hold: the engine's A-1 price of its Rank 1, or why the saved
/// character may not change Roles yet (p. 143).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CprImproveSwitchOption {
    pub role: String,
    pub cost: Option<ExplanationDto>,
    pub refusal: Option<String>,
}

/// A piece of cyberware the dialog offers: its catalog name and category, and the dice its
/// Humanity Loss is rolled on after Character Generation with their range (p. 111, A-3); a piece
/// with no loss dice has range 0-0.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CprImproveCyberwareOption {
    pub name: String,
    pub category: String,
    pub dice: Option<String>,
    pub min: i16,
    pub max: i16,
}

/// A therapy of the p. 230 table as the dialog offers it: the dice its Humanity regain is rolled
/// on with their range (none, 0-0, for Addiction, whose effect is printed) and its printed cost,
/// Medical Tech DV, materials, duration and effect.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CprImproveTherapyOption {
    pub name: String,
    pub dice: Option<String>,
    pub min: i16,
    pub max: i16,
    pub cost_eb: u32,
    pub price_category: String,
    pub medtech_dv: u8,
    pub materials_eb: u32,
    pub materials_price_category: String,
    pub duration: String,
    pub text: String,
}

/// `preview_cyberpunk_improvement`: the options and the plan's rows, or why the plan is refused,
/// and the seed the plan's Humanity Loss rolls used.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CprImprovePreview {
    pub skills: Vec<CprImproveSkillOption>,
    pub role: Option<CprImproveRoleOption>,
    pub switch: Vec<CprImproveSwitchOption>,
    pub cyberware: Vec<CprImproveCyberwareOption>,
    pub therapy: Vec<CprImproveTherapyOption>,
    pub rows: Vec<ExplanationDto>,
    pub refusal: Option<String>,
    pub seed: u32,
}

/// A seed for a request that did not send one.
fn fresh_seed() -> u32 {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or_default();
    (nanos ^ (nanos >> 32) ^ u128::from(std::process::id())) as u32
}

/// Each purchase rolls on its own roller (the seed and the purchase's place in the plan), so
/// adding or removing a later purchase leaves an earlier roll as it was.
fn purchase_roller(seed: u32, index: usize) -> CprRoller {
    let place = u32::try_from(index).unwrap_or(u32::MAX).wrapping_add(1);
    CprRoller::new(seed ^ place.wrapping_mul(0x85EB_CA6B).rotate_left(13))
}

/// The Humanity Loss of an install: the entered loss checked against the piece's dice (A-3), or
/// one rolled on them; the loss and how it was found.
fn install_loss(
    cyberware: &CprCyberwareCatalog,
    name: &str,
    entered: Option<i16>,
    seed: u32,
    index: usize,
) -> Result<(i16, String), String> {
    let dice = cyberware.humanity_loss_dice(name).map_err(|r| r.to_string())?;
    match entered {
        Some(loss) => {
            cyberware.check_rolled_humanity_loss(name, loss).map_err(|r| r.to_string())?;
            let range = dice.map_or("no Humanity Loss dice".to_owned(), |d| {
                let (low, high) = d.range();
                format!("{} ({low}-{high})", d.text())
            });
            Ok((loss, format!("Humanity Loss {loss}, entered; its dice are {range} (p. 111, A-3)")))
        }
        None => match dice {
            Some(dice) => {
                let mut roller = purchase_roller(seed, index);
                let (loss, faces) = dice.roll(|sides| roller.roll(sides));
                let faces: Vec<String> = faces.iter().map(u8::to_string).collect();
                Ok((loss, format!("Humanity Loss {loss}, rolled {} = {} (p. 111, A-3)", dice.text(), faces.join(" + "))))
            }
            None => Ok((0, "Humanity Loss 0: the piece has no Humanity Loss dice (p. 111)".to_owned())),
        },
    }
}

/// The Humanity a therapy regains: the entered regain checked against the therapy's dice, or one
/// rolled on them; the regain and how it was found. Addiction therapy regains none and is refused.
fn therapy_roll(
    cyberware: &CprCyberwareCatalog,
    name: &str,
    entered: Option<i16>,
    seed: u32,
    index: usize,
) -> Result<(String, i16, String), String> {
    let therapy = cyberware
        .therapy(name)
        .ok_or_else(|| format!("'{name}' is not one of the therapies on p. 230"))?;
    let dice: CprHumanityLossDice = therapy.dice.ok_or_else(|| {
        format!("{} therapy regains no Humanity; its effect is printed, not computed (p. 230)", therapy.name)
    })?;
    let (low, high) = dice.range();
    match entered {
        Some(regained) if (low..=high).contains(&regained) => {
            Ok((therapy.name.clone(), regained, format!("entered {regained}; its dice are {} ({low}-{high})", dice.text())))
        }
        Some(regained) => Err(format!(
            "{}: Humanity regained {regained} is outside its dice, {} ({low}-{high}) (p. 230)",
            therapy.name,
            dice.text()
        )),
        None => {
            let mut roller = purchase_roller(seed, index);
            let (regained, faces) = dice.roll(|sides| roller.roll(sides));
            let faces: Vec<String> = faces.iter().map(u8::to_string).collect();
            Ok((therapy.name.clone(), regained, format!("rolled {} = {}", dice.text(), faces.join(" + "))))
        }
    }
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

/// The Roles `choices` holds, in the order taken; none for a character saved without a Role.
fn role_history(choices: &CprSheetChoices) -> Result<Option<CprRoleHistory>, String> {
    let held = choices.held_roles();
    if held.is_empty() {
        return Ok(None);
    }
    CprRoleHistory::from_roles(held).map(Some)
}

/// `choices` holding `history`: its last Role the latest, the rest earlier, in order.
fn set_role_history(choices: &mut CprSheetChoices, history: &CprRoleHistory) {
    let roles = history.roles();
    if let Some((latest, earlier)) = roles.split_last() {
        choices.role = Some(*latest);
        choices.earlier_roles = earlier.to_vec();
    }
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
    cyberware: &CprCyberwareCatalog,
    input: &CprCharacterInput,
    choices: &CprSheetChoices,
    request: &CprImproveRequest,
    seed: u32,
) -> Result<PricedPlan, String> {
    if request.purchases.is_empty() && request.ip_awarded == 0 {
        return Err(
            "nothing to spend: enter IP or choose a Skill Level, Role Rank, Role switch, cyberware, therapy or Reputation change".to_owned()
        );
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
                let history =
                    role_history(&next)?.ok_or("the character has no Role, so it has no Role Ability Rank to raise")?;
                let role = history.current_role();
                let (after, priced) = costs.raise_role_rank(&history, role, *to).map_err(|r| r.to_string())?;
                set_role_history(&mut next, &after);
                (format!("{} Role Ability", role.name()), priced)
            }
            CprPurchase::RoleSwitch { role } => {
                let new_role = CprRole::from_name(role.trim())
                    .ok_or_else(|| format!("'{}' is not one of the ten Roles (p. 142)", role.trim()))?;
                let history = role_history(&next)?.ok_or("the character has no Role, so it cannot change Roles")?;
                let (after, priced) = costs.switch_role(&history, new_role).map_err(|r| r.to_string())?;
                set_role_history(&mut next, &after);
                (format!("Role switch to {}", new_role.name()), priced)
            }
            CprPurchase::Cyberware { name, humanity_loss } => {
                let record = cyberware
                    .record_name(name.trim())
                    .ok_or_else(|| format!("'{}' is not in the cyberware catalog (p. 112-117)", name.trim()))?
                    .to_owned();
                let (loss, detail) = install_loss(cyberware, &record, *humanity_loss, seed, i)?;
                // The piece's mount (which Cybereye, Cyberarm or Cyberleg holds it): placed as
                // creation places it, after the pieces already installed.
                let mut names: Vec<String> = next.cyberware.iter().map(|c| c.name.clone()).collect();
                names.push(record.clone());
                let placed = cyberware.place(&names).map_err(|r| r.to_string())?;
                let mount = placed.last().and_then(|p| p.mount);
                next.cyberware.push(CprCyberwareInstall { name: record.clone(), mount, humanity_loss: Some(loss) });
                rows.push(ComputationExplanation {
                    id: format!("cpr.improve.purchase.{i}.humanity_loss"),
                    value: loss,
                    detail: format!("{record}: {detail}"),
                });
                continue;
            }
            CprPurchase::Therapy { name, regained } => {
                let (therapy, roll, how) = therapy_roll(cyberware, name.trim(), *regained, seed, i)?;
                // D-5 (p. 230): the regain stops at the maximum Humanity of the cyberware the
                // plan has installed so far.
                let base_emp = input.stats.get(CprStat::Emp);
                let loadout = cyberware.check(input.stats.get(CprStat::Body), &next.cyberware).map_err(|r| r.to_string())?;
                let held = next.humanity_regained.unwrap_or(0);
                let current = humanity_after_loss(base_emp, loadout.humanity_loss()) + i16::try_from(held).unwrap_or(i16::MAX);
                let cap = loadout.humanity_cap(base_emp);
                let after = humanity_after_therapy(current, roll, cap);
                let gained = after - current;
                next.humanity_regained = Some(held + u16::try_from(gained).unwrap_or(0));
                let stop = if gained < roll {
                    format!(", stopped at Maximum Humanity {cap} (D-5)")
                } else {
                    format!(", under Maximum Humanity {cap} (D-5)")
                };
                rows.push(ComputationExplanation {
                    id: format!("cpr.improve.purchase.{i}.humanity_regained"),
                    value: gained,
                    detail: format!("{therapy} therapy: {how}; Humanity {current} -> {after}{stop}; regains {gained} (p. 230)"),
                });
                continue;
            }
            CprPurchase::Reputation { level, event, description } => {
                let (after, detail) = record_reputation(&mut next, *level, *event, description)?;
                rows.push(ComputationExplanation { id: format!("cpr.improve.purchase.{i}.reputation_level"), value: after, detail });
                continue;
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

/// The dialog's options: the skills, the latest Role's next Rank and the Role switch choices.
type ImproveOptions = (Vec<CprImproveSkillOption>, Option<CprImproveRoleOption>, Vec<CprImproveSwitchOption>);

/// The dialog's options for the saved `choices`.
fn options(
    costs: &CprIpCostCatalog,
    skills: &CprSkillCatalog,
    choices: &CprSheetChoices,
) -> Result<ImproveOptions, String> {
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
    let history = role_history(choices)?;
    let role = choices.role.map(|(role, rank)| CprImproveRoleOption {
        name: role.name().to_owned(),
        rank,
        next: history.as_ref().and_then(|h| costs.raise_role_rank(h, role, rank + 1).ok()).map(|(_, mut row)| {
            row.id = format!("cpr.improve.role.{}.next", role.id());
            dto(row)
        }),
    });
    let mut switch = Vec::new();
    if let Some(history) = &history {
        for candidate in CprRole::ALL.into_iter().filter(|r| history.rank(*r).is_none()) {
            let (cost, refusal) = match costs.switch_role(history, candidate) {
                Ok((_, mut row)) => {
                    row.id = format!("cpr.improve.switch.{}.cost", candidate.id());
                    (Some(dto(row)), None)
                }
                Err(reason) => (None, Some(reason.to_string())),
            };
            switch.push(CprImproveSwitchOption { role: candidate.name().to_owned(), cost, refusal });
        }
    }
    Ok((out, role, switch))
}

/// Load the character at `root`, price the plan, and check the new choices compose (the
/// engine's sheet check); the new choices and the plan's rows, or the refusal.
fn priced_at_root(
    corpus: &Path,
    root: &Path,
    request: &CprImproveRequest,
    seed: u32,
) -> Result<(CprSheetChoices, Result<PricedPlan, String>), String> {
    let character = CprCharacterStore::load(root)?;
    let choices = CprCharacterStore::load_choices(root)?;
    let costs = CprIpCostCatalog::load(corpus)?;
    let skills = CprSkillCatalog::load(corpus)?;
    let cyberware = CprCyberwareCatalog::load(corpus)?;
    let planned = plan(&costs, &skills, &cyberware, &character.input, &choices, request, seed).and_then(|(next, mut rows)| {
        let catalogs = CprSheetCatalogs::load(corpus)?;
        let after = compose_sheet(&catalogs, &character.input, &next)?;
        if request.purchases.iter().any(|p| matches!(p, CprPurchase::Cyberware { .. } | CprPurchase::Therapy { .. })) {
            // The Humanity and EMP the installs and therapies move (p. 80, p. 111, p. 230), read
            // from the engine's sheet before and after the plan.
            let before = compose_sheet(&catalogs, &character.input, &choices)?;
            for (sheet, when) in [(&before, "before"), (&after, "after")] {
                for (id, label) in [("cpr.derived.humanity_current", "humanity"), ("cpr.derived.emp_current", "emp")] {
                    let found = sheet.rows.iter().find(|r| r.id == id).ok_or_else(|| format!("the sheet has no {id} row"))?;
                    rows.push(ComputationExplanation {
                        id: format!("cpr.improve.{label}.{when}"),
                        value: found.value,
                        detail: format!("{when} this plan: {}", found.detail),
                    });
                }
            }
        }
        if request.purchases.iter().any(|p| matches!(p, CprPurchase::Reputation { .. })) {
            // The Reputation Level and the Facedown base (COOL + Reputation, negative for
            // cowardice; p. 193-194), read from the engine's sheet before and after the plan.
            let before = compose_sheet(&catalogs, &character.input, &choices)?;
            for (sheet, when) in [(&before, "before"), (&after, "after")] {
                for (id, label) in [("cpr.reputation.level", "reputation"), ("cpr.reputation.facedown", "facedown")] {
                    let found = sheet.rows.iter().find(|r| r.id == id).ok_or_else(|| format!("the sheet has no {id} row"))?;
                    rows.push(ComputationExplanation {
                        id: format!("cpr.improve.{label}.{when}"),
                        value: found.value,
                        detail: format!("{when} this plan: {}", found.detail),
                    });
                }
            }
        }
        Ok((next, rows))
    });
    Ok((choices, planned))
}

fn therapy_options(corpus: &Path) -> Result<Vec<CprImproveTherapyOption>, String> {
    Ok(CprCyberwareCatalog::load(corpus)?
        .therapies()
        .iter()
        .map(|t| {
            let (min, max) = t.dice.map_or((0, 0), |d| d.range());
            CprImproveTherapyOption {
                name: t.name.clone(),
                dice: t.dice.map(|d| d.text()),
                min,
                max,
                cost_eb: t.cost_eb,
                price_category: t.price_category.clone(),
                medtech_dv: t.medtech_dv,
                materials_eb: t.materials_eb,
                materials_price_category: t.materials_price_category.clone(),
                duration: t.duration.clone(),
                text: t.text.clone(),
            }
        })
        .collect())
}

fn cyberware_options(corpus: &Path) -> Result<Vec<CprImproveCyberwareOption>, String> {
    Ok(CprCyberwareCatalog::load(corpus)?
        .offers()
        .into_iter()
        .map(|offer| {
            let (min, max) = offer.dice.map_or((0, 0), |d| d.range());
            CprImproveCyberwareOption { name: offer.name, category: offer.category, dice: offer.dice.map(|d| d.text()), min, max }
        })
        .collect())
}

/// The dialog's options and the plan's rows (or refusal) for the CPR character at `root`. Writes
/// nothing.
pub(crate) fn preview_at_root(corpus: &Path, root: &Path, request: &CprImproveRequest) -> Result<CprImprovePreview, String> {
    let seed = request.seed.unwrap_or_else(fresh_seed);
    let (choices, planned) = priced_at_root(corpus, root, request, seed)?;
    let (skills, role, switch) = options(&CprIpCostCatalog::load(corpus)?, &CprSkillCatalog::load(corpus)?, &choices)?;
    // A refused plan still shows the IP available, so the dialog can say what there is to spend.
    let (rows, refusal) = match planned {
        Ok((_, rows)) => (rows.into_iter().map(dto).collect(), None),
        Err(reason) => (available_row(&choices, request.ip_awarded).into_iter().map(dto).collect(), Some(reason)),
    };
    Ok(CprImprovePreview {
        skills,
        role,
        switch,
        cyberware: cyberware_options(corpus)?,
        therapy: therapy_options(corpus)?,
        rows,
        refusal,
        seed,
    })
}

/// Spend the plan on the CPR character at `root`: the new Levels, Ranks, Roles, cyberware (with
/// its Humanity Loss) and IP box are saved through the store and the reloaded sheet is the answer.
/// A refused plan writes nothing. A blank Humanity Loss rolls with the request's seed, so an apply
/// sent with the preview's seed keeps the preview's dice.
pub(crate) fn apply_at_root(corpus: &Path, root: &Path, request: &CprImproveRequest) -> Result<LoadSavedCharacterResponse, String> {
    let seed = request.seed.unwrap_or_else(fresh_seed);
    let (_, planned) = priced_at_root(corpus, root, request, seed)?;
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

    use codex::rules_core::cpr::cyberware::CprCyberwareInstall;
    use codex::rules_core::cpr::input::{CprCharacterInput, CprImprovementPoints, CprSheetChoices, CprStat, CprStats};
    use codex::rules_core::cpr::reputation::{CprReputationEvent, CprReputationKind};
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
        CprImproveRequest { character_id: "cpr-improve".to_owned(), ip_awarded, purchases, seed: None }
    }

    /// The saved character's sheet as the adapter loads it.
    struct CprAdapterSheet(LoadSavedCharacterResponse);

    impl CprAdapterSheet {
        fn of(root: &Path) -> Self {
            Self(CyberpunkAdapter.load_saved_character(root).expect("loads"))
        }

        fn row(&self, id: &str) -> i16 {
            row(&self.0.explanations, id)
        }
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

    /// `saved_solo` with the Solo at `rank`.
    fn saved_solo_at(label: &str, rank: u8, ip: CprImprovementPoints) -> PathBuf {
        let root = saved_solo(label, Some(ip));
        let mut choices = CprCharacterStore::load_choices(&root).unwrap();
        choices.role = Some((CprRole::Solo, rank));
        CprCharacterStore::save_choices(&choices, &root).unwrap();
        root
    }

    fn switch(role: &str) -> CprPurchase {
        CprPurchase::RoleSwitch { role: role.to_owned() }
    }

    /// A-1 / p. 143: a Solo at Rank 3 may not change Roles (refused, nothing written); at Rank 4
    /// Rank 1 of Nomad costs 60 IP (the Role Ability Rank table's Rank 1 price), the Solo keeps
    /// Rank 4, and the reloaded sheet shows both Roles' rank rows and both Role Ability lines.
    #[test]
    fn a_role_switch_is_refused_at_rank_3_and_allowed_at_rank_4_and_both_roles_show() {
        let at3 = saved_solo_at("switch3", 3, CprImprovementPoints { earned: 200, spent: 0 });
        let before = choices_text(&at3);
        let plan = request(0, vec![switch("Nomad")]);
        let reason = preview_at_root(&corpus(), &at3, &plan).expect("previews").refusal.expect("refused");
        assert!(reason.contains("Rank 3") && reason.contains("p. 143"), "{reason}");
        assert!(apply_at_root(&corpus(), &at3, &plan).unwrap_err().contains("Rank 3"));
        assert_eq!(choices_text(&at3), before, "a refused switch writes nothing");
        std::fs::remove_dir_all(at3.parent().unwrap()).ok();

        let at4 = saved_solo_at("switch4", 4, CprImprovementPoints { earned: 200, spent: 0 });
        let preview = preview_at_root(&corpus(), &at4, &plan).expect("previews");
        assert_eq!(preview.refusal, None);
        assert_eq!(row(&preview.rows, "cpr.improve.purchase.0.cost"), 60, "A-1: the Rank 1 price");
        assert!(preview.rows[0].detail.contains("Nomad") && preview.rows[0].detail.contains("A-1"), "{}", preview.rows[0].detail);
        assert_eq!(row(&preview.rows, "cpr.improve.ip.remaining"), 140);

        let loaded = apply_at_root(&corpus(), &at4, &plan).expect("applies");
        let sheet = |id: &str| row(&loaded.explanations, id);
        assert_eq!(sheet("cpr.role.solo.rank"), 4, "the earlier Role keeps its Rank");
        assert_eq!(sheet("cpr.role.solo.combat_awareness_points"), 4);
        assert_eq!(sheet("cpr.role.nomad.rank"), 1, "the new Role starts at Rank 1");
        assert_eq!(sheet("cpr.role.nomad.vehicle_familiarity_bonus"), 1);
        assert_eq!((sheet("cpr.ip.current"), sheet("cpr.ip.total")), (140, 200));
        for id in ["cpr.print.role_ability.solo", "cpr.print.role_ability.nomad"] {
            assert!(loaded.sheet_lines.iter().any(|l| l.id == id), "no line {id}");
        }
        let saved = CprCharacterStore::load_choices(&at4).unwrap();
        assert_eq!(saved.role, Some((CprRole::Nomad, 1)));
        assert_eq!(saved.earlier_roles, vec![(CprRole::Solo, 4)]);
        std::fs::remove_dir_all(at4.parent().unwrap()).ok();
    }

    /// One plan may raise the Solo 3 -> 4 (240) and then switch (60); a Rank purchase after the
    /// switch raises the new Role (Nomad 1 -> 2, 120), and a second switch is locked until the
    /// latest Role reaches Rank 4. A Role already held, or not one of the ten, is refused.
    #[test]
    fn a_plan_can_reach_rank_4_then_switch_and_the_next_switch_waits_for_the_new_role() {
        let root = saved_solo_at("switchplan", 3, CprImprovementPoints { earned: 1000, spent: 0 });
        let plan = request(0, vec![CprPurchase::RoleRank { to: 4 }, switch("Nomad"), CprPurchase::RoleRank { to: 2 }]);
        let preview = preview_at_root(&corpus(), &root, &plan).expect("previews");
        assert_eq!(preview.refusal, None);
        assert_eq!(
            [0, 1, 2].map(|i| row(&preview.rows, &format!("cpr.improve.purchase.{i}.cost"))),
            [240, 60, 120]
        );
        assert_eq!(row(&preview.rows, "cpr.improve.ip.cost"), 420);
        let again = request(0, vec![CprPurchase::RoleRank { to: 4 }, switch("Nomad"), switch("Fixer")]);
        let reason = preview_at_root(&corpus(), &root, &again).expect("previews").refusal.expect("locked");
        assert!(reason.contains("Nomad, is at Rank 1"), "{reason}");
        let refusal = |plan: CprImproveRequest| preview_at_root(&corpus(), &root, &plan).expect("previews").refusal.expect("refused");
        assert!(refusal(request(0, vec![CprPurchase::RoleRank { to: 4 }, switch("Solo")])).contains("already has the Solo Role"));
        assert!(refusal(request(0, vec![switch("Cowboy")])).contains("not one of the ten Roles"));

        let loaded = apply_at_root(&corpus(), &root, &plan).expect("applies");
        assert_eq!(row(&loaded.explanations, "cpr.role.solo.rank"), 4);
        assert_eq!(row(&loaded.explanations, "cpr.role.nomad.rank"), 2);
        assert_eq!(row(&loaded.explanations, "cpr.ip.current"), 580);
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    /// The preview offers every Role the character does not hold: at Rank 4 each with the
    /// engine's A-1 price row, at Rank 3 each with the engine's refusal instead.
    #[test]
    fn the_preview_offers_each_unheld_role_with_its_price_or_the_lock() {
        let at4 = saved_solo_at("switchopts4", 4, CprImprovementPoints { earned: 0, spent: 0 });
        let preview = preview_at_root(&corpus(), &at4, &request(0, vec![switch("Nomad")])).expect("previews");
        assert_eq!(preview.switch.len(), 9, "the ten Roles less the Solo");
        assert!(preview.switch.iter().all(|o| o.role != "Solo"));
        for option in &preview.switch {
            assert_eq!(option.cost.as_ref().map(|r| r.value), Some(60), "{}", option.role);
            assert_eq!(option.refusal, None);
        }
        std::fs::remove_dir_all(at4.parent().unwrap()).ok();

        let at3 = saved_solo_at("switchopts3", 3, CprImprovementPoints { earned: 0, spent: 0 });
        let preview = preview_at_root(&corpus(), &at3, &request(0, vec![switch("Nomad")])).expect("previews");
        assert_eq!(preview.switch.len(), 9);
        for option in &preview.switch {
            assert!(option.cost.is_none(), "{}", option.role);
            assert!(option.refusal.as_deref().is_some_and(|r| r.contains("Rank 3")), "{:?}", option.refusal);
        }
        std::fs::remove_dir_all(at3.parent().unwrap()).ok();
    }

    #[test]
    fn a_role_switch_deserialises_from_the_frontend_wire_shape() {
        let wire = serde_json::json!({
            "characterId": "cpr-improve",
            "ipAwarded": 0,
            "purchases": [{ "kind": "roleSwitch", "role": "Nomad" }]
        });
        let parsed: CprImproveRequest = serde_json::from_value(wire).expect("deserialises");
        assert_eq!(parsed.purchases, vec![switch("Nomad")]);
    }

    fn cyberware(name: &str, humanity_loss: Option<i16>) -> CprPurchase {
        CprPurchase::Cyberware { name: name.to_owned(), humanity_loss }
    }

    /// A-3 / p. 111: Gills installed after creation with an entered Humanity Loss of 12 (its
    /// dice are 2d6, 2-12) costs no IP; the preview shows the loss and Humanity 80 -> 68, EMP
    /// 8 -> 6 (p. 80: EMP is the tens place of Humanity); the save keeps the entered loss, and
    /// the reloaded sheet's Humanity Loss, Humanity and EMP rows move.
    #[test]
    fn an_install_takes_an_entered_humanity_loss_and_the_humanity_and_emp_rows_move() {
        let root = saved_solo("install", Some(CprImprovementPoints { earned: 100, spent: 0 }));
        let plan = request(0, vec![cyberware("Gills", Some(12))]);
        let preview = preview_at_root(&corpus(), &root, &plan).expect("previews");
        assert_eq!(preview.refusal, None);
        assert_eq!(row(&preview.rows, "cpr.improve.purchase.0.humanity_loss"), 12);
        assert!(preview.rows.iter().all(|r| r.id != "cpr.improve.purchase.0.cost"), "an install is not an IP purchase");
        assert_eq!(row(&preview.rows, "cpr.improve.ip.cost"), 0);
        assert_eq!((row(&preview.rows, "cpr.improve.humanity.before"), row(&preview.rows, "cpr.improve.humanity.after")), (80, 68));
        assert_eq!((row(&preview.rows, "cpr.improve.emp.before"), row(&preview.rows, "cpr.improve.emp.after")), (8, 6));
        let gills = preview.cyberware.iter().find(|o| o.name == "Gills").expect("Gills offered");
        assert_eq!((gills.dice.as_deref(), gills.min, gills.max), (Some("2d6"), 2, 12));

        let before = CprAdapterSheet::of(&root);
        assert_eq!((before.row("cpr.derived.humanity_current"), before.row("cpr.derived.emp_current")), (80, 8));
        let loaded = apply_at_root(&corpus(), &root, &plan).expect("applies");
        let sheet = |id: &str| row(&loaded.explanations, id);
        assert_eq!(sheet("cpr.cyberware.humanity_loss"), 12);
        assert_eq!(sheet("cpr.derived.humanity_current"), 68);
        assert_eq!(sheet("cpr.derived.emp_current"), 6, "the EMP row moves 8 -> 6");
        assert_eq!(sheet("cpr.ip.current"), 100, "no IP spent");
        let saved = CprCharacterStore::load_choices(&root).unwrap();
        assert_eq!(saved.cyberware, vec![CprCyberwareInstall { name: "Gills".to_owned(), mount: None, humanity_loss: Some(12) }]);
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    /// A-3: an entered loss outside the piece's dice is refused naming the range (2d6 13 and 1,
    /// 4d6 3, 1d6/2 round up 4, a no-dice piece 1); nothing is written. The bounds themselves
    /// are accepted.
    #[test]
    fn an_entered_humanity_loss_outside_the_dice_is_refused_and_nothing_is_saved() {
        let root = saved_solo("hlrange", None);
        let before = choices_text(&root);
        for (name, loss, range) in
            [("Gills", 13, "2d6 (2-12)"), ("Gills", 1, "2d6 (2-12)"), ("Grafted Muscle and Bone Lace", 3, "4d6 (4-24)"), ("Nasal Filters", 4, "1d6/2 round up (1-3)"), ("Biomonitor", 1, "no Humanity Loss dice (0)")]
        {
            let plan = request(0, vec![cyberware(name, Some(loss))]);
            let reason = preview_at_root(&corpus(), &root, &plan).expect("previews").refusal.unwrap_or_else(|| panic!("{name} {loss} accepted"));
            assert!(reason.contains(&format!("Humanity Loss {loss}")) && reason.contains(range), "{reason}");
            assert!(apply_at_root(&corpus(), &root, &plan).is_err());
            assert_eq!(choices_text(&root), before, "a refused install writes nothing");
        }
        for (name, loss) in [("Gills", 2), ("Gills", 12), ("Grafted Muscle and Bone Lace", 24), ("Nasal Filters", 1), ("Biomonitor", 0)] {
            let plan = request(0, vec![cyberware(name, Some(loss))]);
            assert_eq!(preview_at_root(&corpus(), &root, &plan).expect("previews").refusal, None, "{name} {loss}");
        }
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    /// D-11: with the 7 Internal Body option slots full, an eighth Internal piece is refused as
    /// over the slots and nothing is written; a Cybereye option without a Cybereye is refused.
    #[test]
    fn an_over_slot_install_is_refused_and_nothing_is_saved() {
        let root = saved_solo("overslot", None);
        let mut choices = CprCharacterStore::load_choices(&root).unwrap();
        for name in ["AudioVox", "Enhanced Antibodies", "Gills", "Nasal Filters", "Toxin Binders", "Independent Air Supply", "Radar/Sonar Implant"] {
            choices.cyberware.push(CprCyberwareInstall { name: name.to_owned(), mount: None, humanity_loss: None });
        }
        CprCharacterStore::save_choices(&choices, &root).unwrap();
        let before = choices_text(&root);
        let plan = request(0, vec![cyberware("Vampyres", Some(14))]);
        let reason = preview_at_root(&corpus(), &root, &plan).expect("previews").refusal.expect("over the slots");
        assert!(reason.contains("Vampyres does not fit") && reason.contains("8 of 7"), "{reason}");
        assert!(apply_at_root(&corpus(), &root, &plan).unwrap_err().contains("8 of 7"));
        assert_eq!(choices_text(&root), before);
        let reason = preview_at_root(&corpus(), &root, &request(0, vec![cyberware("Chyron", Some(2))]))
            .expect("previews")
            .refusal
            .expect("no Cybereye");
        assert!(reason.contains("Chyron"), "{reason}");
        // A Cybereye first gives the option its eye.
        let plan = request(0, vec![cyberware("Cybereye", Some(7)), cyberware("Chyron", Some(2))]);
        assert_eq!(preview_at_root(&corpus(), &root, &plan).expect("previews").refusal, None);
        let loaded = apply_at_root(&corpus(), &root, &plan).expect("applies");
        assert_eq!(row(&loaded.explanations, "cpr.cyberware.slots.cybereye_1.used"), 1);
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    /// A-3: a loss left blank is rolled on the piece's dice by the creation roller with the
    /// request's seed: the preview answers with its seed, the same seed rolls the same loss on
    /// apply, and over 300 seeds every 1d6/2 round-up roll lands in 1-3 and reaches both ends.
    #[test]
    fn a_blank_humanity_loss_is_rolled_with_the_seed_and_the_same_seed_keeps_it() {
        let root = saved_solo("rolled", None);
        let plan = request(0, vec![cyberware("Gills", None)]);
        let preview = preview_at_root(&corpus(), &root, &plan).expect("previews");
        assert_eq!(preview.refusal, None);
        let seed = preview.seed;
        let rolled = row(&preview.rows, "cpr.improve.purchase.0.humanity_loss");
        assert!((2..=12).contains(&rolled), "{rolled}");
        let detail = &preview.rows.iter().find(|r| r.id == "cpr.improve.purchase.0.humanity_loss").unwrap().detail;
        assert!(detail.contains("rolled 2d6"), "{detail}");

        let mut seeded = request(0, vec![cyberware("Gills", None)]);
        seeded.seed = Some(seed);
        let again = preview_at_root(&corpus(), &root, &seeded).expect("previews");
        assert_eq!((again.seed, row(&again.rows, "cpr.improve.purchase.0.humanity_loss")), (seed, rolled));
        let loaded = apply_at_root(&corpus(), &root, &seeded).expect("applies");
        assert_eq!(row(&loaded.explanations, "cpr.cyberware.humanity_loss"), rolled);
        assert_eq!(CprCharacterStore::load_choices(&root).unwrap().cyberware[0].humanity_loss, Some(rolled));

        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..300 {
            let mut plan = request(0, vec![cyberware("Nasal Filters", None)]);
            plan.seed = Some(seed);
            let preview = preview_at_root(&corpus(), &root, &plan).expect("previews");
            seen.insert(row(&preview.rows, "cpr.improve.purchase.0.humanity_loss"));
        }
        assert_eq!(seen.into_iter().collect::<Vec<_>>(), vec![1, 2, 3]);
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    #[test]
    fn a_cyberware_install_deserialises_from_the_frontend_wire_shape() {
        let wire = serde_json::json!({
            "characterId": "cpr-improve",
            "ipAwarded": 0,
            "seed": 7,
            "purchases": [{ "kind": "cyberware", "name": "Gills", "humanityLoss": 9 }, { "kind": "cyberware", "name": "Gills", "humanityLoss": null }]
        });
        let parsed: CprImproveRequest = serde_json::from_value(wire).expect("deserialises");
        assert_eq!(parsed.seed, Some(7));
        assert_eq!(parsed.purchases, vec![cyberware("Gills", Some(9)), cyberware("Gills", None)]);
    }

    fn therapy(name: &str, regained: Option<i16>) -> CprPurchase {
        CprPurchase::Therapy { name: name.to_owned(), regained }
    }

    /// `saved_solo` (EMP 8) with Gills installed at a Humanity Loss of 12: Humanity 80 - 12 = 68,
    /// EMP 6; maximum Humanity 80 - 1 piece x 2 = 78 (D-5, p. 230).
    fn saved_solo_with_gills(label: &str) -> PathBuf {
        let root = saved_solo(label, Some(CprImprovementPoints { earned: 50, spent: 0 }));
        let mut choices = CprCharacterStore::load_choices(&root).unwrap();
        choices.cyberware.push(CprCyberwareInstall { name: "Gills".to_owned(), mount: None, humanity_loss: Some(12) });
        CprCharacterStore::save_choices(&choices, &root).unwrap();
        let sheet = CprAdapterSheet::of(&root);
        assert_eq!(
            (sheet.row("cpr.derived.humanity_current"), sheet.row("cpr.cyberware.humanity_cap"), sheet.row("cpr.derived.emp_current")),
            (68, 78, 6)
        );
        root
    }

    /// p. 230: Standard Humanity Loss therapy with 7 entered (its dice 2d6, 2-12) regains 7 below
    /// the cap: Humanity 68 -> 75, EMP 6 -> 7; it costs no IP; the save keeps the regained 7 and
    /// the reloaded sheet's Humanity and EMP rows move.
    #[test]
    fn therapy_regains_the_entered_humanity_below_the_cap_and_the_sheet_rereads() {
        let root = saved_solo_with_gills("therapy");
        let plan = request(0, vec![therapy("Standard Humanity Loss", Some(7))]);
        let preview = preview_at_root(&corpus(), &root, &plan).expect("previews");
        assert_eq!(preview.refusal, None);
        assert_eq!(row(&preview.rows, "cpr.improve.purchase.0.humanity_regained"), 7);
        assert!(preview.rows.iter().all(|r| r.id != "cpr.improve.purchase.0.cost"), "therapy is not an IP purchase");
        assert_eq!(row(&preview.rows, "cpr.improve.ip.cost"), 0);
        assert_eq!((row(&preview.rows, "cpr.improve.humanity.before"), row(&preview.rows, "cpr.improve.humanity.after")), (68, 75));
        assert_eq!((row(&preview.rows, "cpr.improve.emp.before"), row(&preview.rows, "cpr.improve.emp.after")), (6, 7));

        let loaded = apply_at_root(&corpus(), &root, &plan).expect("applies");
        let sheet = |id: &str| row(&loaded.explanations, id);
        assert_eq!((sheet("cpr.derived.humanity_current"), sheet("cpr.derived.emp_current")), (75, 7));
        assert_eq!(sheet("cpr.ip.current"), 50, "no IP spent");
        assert_eq!(CprCharacterStore::load_choices(&root).unwrap().humanity_regained, Some(7));
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    /// D-5: a restore above the cap stops at the cap. Extreme therapy with 20 entered from
    /// Humanity 68 would reach 88; it stops at maximum Humanity 78 (regained 10), and a second
    /// therapy at the cap regains 0. Two therapies in one plan add up (5 + 12 -> 68 + 5 = 73,
    /// then 73 + 12 = 85, stopped at 78).
    #[test]
    fn a_restore_above_the_cap_stops_at_the_cap() {
        let root = saved_solo_with_gills("cap");
        let plan = request(0, vec![therapy("Extreme Humanity Loss", Some(20))]);
        let preview = preview_at_root(&corpus(), &root, &plan).expect("previews");
        assert_eq!(preview.refusal, None);
        assert_eq!(row(&preview.rows, "cpr.improve.purchase.0.humanity_regained"), 10);
        let detail = &preview.rows.iter().find(|r| r.id == "cpr.improve.purchase.0.humanity_regained").unwrap().detail;
        assert!(detail.contains("Maximum Humanity 78") && detail.contains("D-5"), "{detail}");
        assert_eq!(row(&preview.rows, "cpr.improve.humanity.after"), 78);

        let two = request(0, vec![therapy("Standard Humanity Loss", Some(5)), therapy("Standard Humanity Loss", Some(12))]);
        let preview = preview_at_root(&corpus(), &root, &two).expect("previews");
        assert_eq!(
            (row(&preview.rows, "cpr.improve.purchase.0.humanity_regained"), row(&preview.rows, "cpr.improve.purchase.1.humanity_regained")),
            (5, 5)
        );

        let loaded = apply_at_root(&corpus(), &root, &plan).expect("applies");
        assert_eq!(row(&loaded.explanations, "cpr.derived.humanity_current"), 78);
        assert_eq!(row(&loaded.explanations, "cpr.derived.emp_current"), 7);
        assert_eq!(CprCharacterStore::load_choices(&root).unwrap().humanity_regained, Some(10));
        let again = preview_at_root(&corpus(), &root, &request(0, vec![therapy("Standard Humanity Loss", Some(12))])).expect("previews");
        assert_eq!(row(&again.rows, "cpr.improve.purchase.0.humanity_regained"), 0, "at the cap therapy regains nothing");
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    /// An entered regain outside the therapy's dice (Standard 13 and 1, Extreme 3 and 25) is
    /// refused naming the range; Addiction therapy regains no Humanity (its effect is printed)
    /// and is refused as a purchase; an unknown therapy is refused; nothing is written.
    #[test]
    fn an_entered_regain_outside_the_dice_and_addiction_therapy_are_refused_and_nothing_is_saved() {
        let root = saved_solo_with_gills("therapyrange");
        let before = choices_text(&root);
        for (name, regained, range) in [
            ("Standard Humanity Loss", 13, "2d6 (2-12)"),
            ("Standard Humanity Loss", 1, "2d6 (2-12)"),
            ("Extreme Humanity Loss", 3, "4d6 (4-24)"),
            ("Extreme Humanity Loss", 25, "4d6 (4-24)"),
        ] {
            let plan = request(0, vec![therapy(name, Some(regained))]);
            let reason = preview_at_root(&corpus(), &root, &plan).expect("previews").refusal.unwrap_or_else(|| panic!("{name} {regained} accepted"));
            assert!(reason.contains(&format!("Humanity regained {regained}")) && reason.contains(range), "{reason}");
            assert!(apply_at_root(&corpus(), &root, &plan).is_err());
        }
        let reason = preview_at_root(&corpus(), &root, &request(0, vec![therapy("Addiction", None)])).expect("previews").refusal.expect("refused");
        assert!(reason.contains("Addiction") && reason.contains("printed"), "{reason}");
        let reason = preview_at_root(&corpus(), &root, &request(0, vec![therapy("Primal Scream", Some(4))])).expect("previews").refusal.expect("refused");
        assert!(reason.contains("Primal Scream"), "{reason}");
        assert_eq!(choices_text(&root), before, "a refused therapy writes nothing");
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    /// A regain left blank is rolled on the therapy's dice with the request's seed; the same seed
    /// keeps it on apply; over 300 seeds every Standard roll lands in 2-12 and reaches both ends.
    #[test]
    fn a_blank_regain_is_rolled_with_the_seed_and_the_same_seed_keeps_it() {
        let root = saved_solo("therapyroll", None);
        let mut choices = CprCharacterStore::load_choices(&root).unwrap();
        // Gills 12 + Grafted Muscle and Bone Lace 24: Humanity 80 - 36 = 44; the cap (74 or more) is above 44 + 24.
        choices.cyberware.push(CprCyberwareInstall { name: "Gills".to_owned(), mount: None, humanity_loss: Some(12) });
        choices.cyberware.push(CprCyberwareInstall { name: "Grafted Muscle and Bone Lace".to_owned(), mount: None, humanity_loss: Some(24) });
        CprCharacterStore::save_choices(&choices, &root).unwrap();
        let plan = request(0, vec![therapy("Extreme Humanity Loss", None)]);
        let preview = preview_at_root(&corpus(), &root, &plan).expect("previews");
        assert_eq!(preview.refusal, None);
        let rolled = row(&preview.rows, "cpr.improve.purchase.0.humanity_regained");
        assert!((4..=24).contains(&rolled), "{rolled}");
        let detail = &preview.rows.iter().find(|r| r.id == "cpr.improve.purchase.0.humanity_regained").unwrap().detail;
        assert!(detail.contains("rolled 4d6"), "{detail}");
        let mut seeded = request(0, vec![therapy("Extreme Humanity Loss", None)]);
        seeded.seed = Some(preview.seed);
        let loaded = apply_at_root(&corpus(), &root, &seeded).expect("applies");
        assert_eq!(row(&loaded.explanations, "cpr.derived.humanity_current"), 44 + rolled);
        assert_eq!(CprCharacterStore::load_choices(&root).unwrap().humanity_regained, Some(rolled as u16));

        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..300 {
            let mut plan = request(0, vec![therapy("Standard Humanity Loss", None)]);
            plan.seed = Some(seed);
            seen.insert(row(&preview_at_root(&corpus(), &root, &plan).expect("previews").rows, "cpr.improve.purchase.0.humanity_regained"));
        }
        assert_eq!((seen.first().copied(), seen.last().copied()), (Some(2), Some(12)));
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    /// The preview offers the three p. 230 therapies: Standard 2d6 (2-12) and Extreme 4d6 (4-24)
    /// with their printed cost and DV, and Addiction with no dice and its printed effect.
    #[test]
    fn the_preview_offers_the_therapies_and_prints_addiction() {
        let root = saved_solo("therapyopts", None);
        let preview = preview_at_root(&corpus(), &root, &request(0, vec![skill("Handgun", 5)])).expect("previews");
        let names: Vec<&str> = preview.therapy.iter().map(|t| t.name.as_str()).collect();
        assert_eq!(names, ["Addiction", "Standard Humanity Loss", "Extreme Humanity Loss"]);
        let standard = &preview.therapy[1];
        assert_eq!((standard.dice.as_deref(), standard.min, standard.max, standard.cost_eb, standard.medtech_dv), (Some("2d6"), 2, 12, 500, 15));
        let extreme = &preview.therapy[2];
        assert_eq!((extreme.dice.as_deref(), extreme.min, extreme.max, extreme.cost_eb, extreme.medtech_dv), (Some("4d6"), 4, 24, 1000, 17));
        let addiction = &preview.therapy[0];
        assert_eq!((addiction.dice.as_deref(), addiction.min, addiction.max), (None, 0, 0));
        assert!(addiction.text.contains("automatically fail the roll"), "{}", addiction.text);
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    #[test]
    fn a_therapy_deserialises_from_the_frontend_wire_shape() {
        let wire = serde_json::json!({
            "characterId": "cpr-improve",
            "ipAwarded": 0,
            "purchases": [{ "kind": "therapy", "name": "Standard Humanity Loss", "regained": 7 }, { "kind": "therapy", "name": "Extreme Humanity Loss", "regained": null }]
        });
        let parsed: CprImproveRequest = serde_json::from_value(wire).expect("deserialises");
        assert_eq!(parsed.purchases, vec![therapy("Standard Humanity Loss", Some(7)), therapy("Extreme Humanity Loss", None)]);
    }

    fn reputation(level: u8, event: CprReputationEventKind, description: &str) -> CprPurchase {
        CprPurchase::Reputation { level, event, description: description.to_owned() }
    }

    use CprReputationEventKind::{Cowardice, Deed};

    /// `saved_solo` (COOL 4) with the Reputation events `events` already recorded.
    fn saved_solo_with_reputation(label: &str, events: &[(u8, CprReputationKind, &str)]) -> PathBuf {
        let root = saved_solo(label, Some(CprImprovementPoints { earned: 30, spent: 0 }));
        let mut choices = CprCharacterStore::load_choices(&root).unwrap();
        for (level, kind, description) in events {
            choices.reputation.push(CprReputationEvent { level: *level, kind: *kind, description: (*description).to_owned() });
        }
        CprCharacterStore::save_choices(&choices, &root).unwrap();
        root
    }

    fn line(loaded: &LoadSavedCharacterResponse, id: &str) -> String {
        loaded.sheet_lines.iter().find(|l| l.id == id).unwrap_or_else(|| panic!("no line {id}")).value.clone()
    }

    /// p. 193: a Reputation change sets the Level and the event. A Level 4 deed on a character at
    /// Reputation 0 costs no IP; the preview's rows give the Level after (4), the Reputation 0 -> 4
    /// and the Facedown base COOL 4 + 0 -> 4 + 4 (p. 194); the apply saves the event and the
    /// reloaded sheet's Reputation and Facedown rows and its event line move.
    #[test]
    fn a_reputation_change_sets_the_level_and_the_event_and_the_sheet_rereads() {
        let root = saved_solo_with_reputation("rep", &[]);
        let before = CprAdapterSheet::of(&root);
        assert_eq!((before.row("cpr.reputation.level"), before.row("cpr.reputation.facedown")), (0, 4), "COOL 4, Reputation 0");
        let plan = request(0, vec![reputation(4, Deed, "Played a killer show")]);
        let preview = preview_at_root(&corpus(), &root, &plan).expect("previews");
        assert_eq!(preview.refusal, None);
        assert_eq!(row(&preview.rows, "cpr.improve.purchase.0.reputation_level"), 4);
        let detail = &preview.rows.iter().find(|r| r.id == "cpr.improve.purchase.0.reputation_level").unwrap().detail;
        assert!(detail.contains("Played a killer show") && detail.contains("defining") && detail.contains("p. 193"), "{detail}");
        assert!(preview.rows.iter().all(|r| r.id != "cpr.improve.purchase.0.cost"), "a Reputation change is not an IP purchase");
        assert_eq!(row(&preview.rows, "cpr.improve.ip.cost"), 0);
        assert_eq!((row(&preview.rows, "cpr.improve.reputation.before"), row(&preview.rows, "cpr.improve.reputation.after")), (0, 4));
        assert_eq!((row(&preview.rows, "cpr.improve.facedown.before"), row(&preview.rows, "cpr.improve.facedown.after")), (4, 8));

        let loaded = apply_at_root(&corpus(), &root, &plan).expect("applies");
        let sheet = |id: &str| row(&loaded.explanations, id);
        assert_eq!((sheet("cpr.reputation.level"), sheet("cpr.reputation.facedown")), (4, 8));
        assert_eq!(sheet("cpr.ip.current"), 30, "no IP spent");
        assert_eq!(line(&loaded, "cpr.reputation.event.0"), "Level 4 deed: Played a killer show (defining)");
        assert_eq!(line(&loaded, "cpr.reputation.facedown_roll"), "8 + 1d10");
        let saved = CprCharacterStore::load_choices(&root).unwrap();
        assert_eq!(
            saved.reputation,
            vec![CprReputationEvent { level: 4, kind: CprReputationKind::Deed, description: "Played a killer show".to_owned() }]
        );
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    /// p. 193: "a new deed's Reputation will only replace the old one if the Level is higher".
    /// With a Level 5 deed defining, a Level 3 deed and an equal Level 5 deed are recorded but the
    /// Level stays 5 (the row says it does not replace); a Level 6 deed replaces it. Two deeds in
    /// one plan are taken in order (6, then 7).
    #[test]
    fn a_new_positive_deed_replaces_the_old_only_if_higher() {
        let root = saved_solo_with_reputation("rephigher", &[(5, CprReputationKind::Deed, "Held the bridge")]);
        for lower in [3u8, 5] {
            let preview = preview_at_root(&corpus(), &root, &request(0, vec![reputation(lower, Deed, "Fixed a deal")])).expect("previews");
            assert_eq!(preview.refusal, None);
            assert_eq!(row(&preview.rows, "cpr.improve.purchase.0.reputation_level"), 5, "Level {lower} does not replace 5");
            let detail = &preview.rows.iter().find(|r| r.id == "cpr.improve.purchase.0.reputation_level").unwrap().detail;
            assert!(detail.contains("does not replace") && detail.contains("Level 5") && detail.contains("p. 193"), "{detail}");
            assert_eq!(row(&preview.rows, "cpr.improve.reputation.after"), 5);
        }
        let higher = request(0, vec![reputation(6, Deed, "Took down a Maxtac squad")]);
        let preview = preview_at_root(&corpus(), &root, &higher).expect("previews");
        assert_eq!(row(&preview.rows, "cpr.improve.purchase.0.reputation_level"), 6);
        assert_eq!((row(&preview.rows, "cpr.improve.reputation.before"), row(&preview.rows, "cpr.improve.reputation.after")), (5, 6));
        let two = request(0, vec![reputation(6, Deed, "a"), reputation(7, Deed, "b")]);
        let preview = preview_at_root(&corpus(), &root, &two).expect("previews");
        assert_eq!(
            (row(&preview.rows, "cpr.improve.purchase.0.reputation_level"), row(&preview.rows, "cpr.improve.purchase.1.reputation_level")),
            (6, 7)
        );

        // The lower deed is saved (people may know of it) but the Level 5 deed stays defining.
        let loaded = apply_at_root(&corpus(), &root, &request(0, vec![reputation(3, Deed, "Fixed a deal")])).expect("applies");
        assert_eq!(row(&loaded.explanations, "cpr.reputation.level"), 5);
        assert_eq!(line(&loaded, "cpr.reputation.event.0"), "Level 5 deed: Held the bridge (defining)");
        assert_eq!(line(&loaded, "cpr.reputation.event.1"), "Level 3 deed: Fixed a deal");
        let loaded = apply_at_root(&corpus(), &root, &higher).expect("applies");
        assert_eq!(row(&loaded.explanations, "cpr.reputation.level"), 6);
        assert_eq!(line(&loaded, "cpr.reputation.event.2"), "Level 6 deed: Took down a Maxtac squad (defining)");
        assert_eq!(CprCharacterStore::load_choices(&root).unwrap().reputation.len(), 3);
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    /// p. 194: "If your defining Reputation Event is for cowardice your Reputation Level is treated
    /// as a negative number." Over a Level 3 deed (Facedown 4 + 3 = 7), a Level 6 cowardice event
    /// replaces it (p. 193: a negative event can replace a positive one) and the Facedown base
    /// becomes 4 - 6 = -2; a Level 2 cowardice event does not replace the deed and the Facedown
    /// stays 7.
    #[test]
    fn a_cowardice_event_makes_the_facedown_use_a_negative_reputation() {
        let root = saved_solo_with_reputation("repcoward", &[(3, CprReputationKind::Deed, "Held the bridge")]);
        let plan = request(0, vec![reputation(6, Cowardice, "Ran from the Maelstrom")]);
        let preview = preview_at_root(&corpus(), &root, &plan).expect("previews");
        assert_eq!(preview.refusal, None);
        assert_eq!(row(&preview.rows, "cpr.improve.purchase.0.reputation_level"), 6, "the Level itself is 6");
        let detail = &preview.rows.iter().find(|r| r.id == "cpr.improve.purchase.0.reputation_level").unwrap().detail;
        assert!(detail.contains("cowardice") && detail.contains("-6") && detail.contains("p. 194"), "{detail}");
        assert_eq!((row(&preview.rows, "cpr.improve.facedown.before"), row(&preview.rows, "cpr.improve.facedown.after")), (7, -2));

        let lower = preview_at_root(&corpus(), &root, &request(0, vec![reputation(2, Cowardice, "Hid")])).expect("previews");
        assert_eq!(row(&lower.rows, "cpr.improve.purchase.0.reputation_level"), 3);
        assert_eq!(row(&lower.rows, "cpr.improve.facedown.after"), 7, "a lower cowardice event leaves the deed defining");

        let loaded = apply_at_root(&corpus(), &root, &plan).expect("applies");
        assert_eq!((row(&loaded.explanations, "cpr.reputation.level"), row(&loaded.explanations, "cpr.reputation.facedown")), (6, -2));
        assert_eq!(line(&loaded, "cpr.reputation.facedown_roll"), "-2 + 1d10");
        assert_eq!(line(&loaded, "cpr.reputation.event.1"), "Level 6 cowardice: Ran from the Maelstrom (defining)");
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    /// A Level outside the p. 193 table (0, 11), an event with no description and one that would
    /// break the saved line (a line break) are refused; nothing is written.
    #[test]
    fn a_reputation_change_outside_the_table_or_without_an_event_is_refused_and_nothing_is_saved() {
        let root = saved_solo_with_reputation("represuse", &[(2, CprReputationKind::Deed, "Held the bridge")]);
        let before = choices_text(&root);
        let refusal = |purchase: CprPurchase| {
            let plan = request(0, vec![purchase]);
            assert!(apply_at_root(&corpus(), &root, &plan).is_err());
            preview_at_root(&corpus(), &root, &plan).expect("previews").refusal.expect("refused")
        };
        for level in [0u8, 11] {
            let reason = refusal(reputation(level, Deed, "x"));
            assert!(reason.contains(&format!("Reputation Level {level}")) && reason.contains("p. 193"), "{reason}");
        }
        assert!(refusal(reputation(4, Deed, "  ")).contains("what the event was"));
        assert!(refusal(reputation(4, Cowardice, "Ran\nreputation = 10 deed: forged")).contains("one line"));
        assert_eq!(choices_text(&root), before, "a refused Reputation change writes nothing");
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    #[test]
    fn a_reputation_change_deserialises_from_the_frontend_wire_shape() {
        let wire = serde_json::json!({
            "characterId": "cpr-improve",
            "ipAwarded": 0,
            "purchases": [
                { "kind": "reputation", "level": 4, "event": "deed", "description": "Played a killer show" },
                { "kind": "reputation", "level": 6, "event": "cowardice", "description": "Ran" }
            ]
        });
        let parsed: CprImproveRequest = serde_json::from_value(wire).expect("deserialises");
        assert_eq!(parsed.purchases, vec![reputation(4, Deed, "Played a killer show"), reputation(6, Cowardice, "Ran")]);
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
