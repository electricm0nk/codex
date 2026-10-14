//! The `cpr.` explanation rows a Cyberpunk RED sheet renders. Every number on the sheet is
//! one row here (id `cpr.<area>.<field>`, value, derivation text); no frontend code
//! re-derives a number. Dice and rule text stay printed ([`CprPrintedLine`]).
//!
//! [`compose_sheet`] assembles one sheet from the character's STATs ([`CprCharacterInput`])
//! and its sheet choices ([`CprSheetChoices`]) through every engine module, in the order the
//! values feed each other:
//!
//! 1. cyberware (`cyberware.rs`): Humanity Loss, BODY after Grafted Muscle / Linear Frames,
//!    cyberware SP, the Initiative bonus, option slots;
//! 2. Humanity and EMP after the loss (p. 80-81: "When you lose Humanity points you will
//!    sometimes have to lower your EMP Statistic");
//! 3. armour (`armor.rs`), with the cyberware SP as a competing source per location;
//! 4. the Derived STATs (`stats.rs`) and the wound thresholds (`critical.rs`) from the
//!    cyberware BODY (p. 364: Grafted Muscle "changes a Character's HP and Death Save");
//! 5. skill bases (`skills.rs`) from REF and DEX after the armour penalty (p. 184: the
//!    penalty lowers REF, DEX and MOVE) and the current EMP, with one level row per skill;
//! 6. the Role Ability rank values (`roles.rs`) and, for a Netrunner, the deck block
//!    (`netrunning.rs`), whose Bodyweight Suit and Cyberarm slots come from steps 1 and 3;
//! 7. Reputation (`reputation.rs`), then the looked-up and ledger boxes of the sheet: weapon
//!    ROF and magazine (damage dice printed), shield HP, Improvement Points, cash, ammunition
//!    and rent.
//!
//! `improvement.rs` prices a purchase (the improve dialog's action), not a sheet number, so it
//! adds no row here. Play-time state is not simulated: the current HP and current LUCK of the
//! sheet's "out of" boxes, ablation, the Wound State and its penalties are written in by the
//! player, and the sheet prints their rules.

use std::path::Path;

use serde::Deserialize;

use crate::rules_core::cpr::armor::{CprArmorCatalog, CprArmorLocation, CprSpSource, BODYWEIGHT_SUIT};
use crate::rules_core::cpr::critical::CprCriticalCatalog;
use crate::rules_core::cpr::cyberware::{humanity_after_loss, CprCyberwareCatalog};
use crate::rules_core::cpr::input::{CprCharacterInput, CprSheetChoices, CprStat};
use crate::rules_core::cpr::netrunning::{netrunner_sheet, CprDeckExtras, CprDeckItem, CprDeckQuality};
use crate::rules_core::cpr::reputation::{reputation_rows, CprReputation, CprReputationCatalog};
use crate::rules_core::cpr::roles::{CprRole, CprRoleCatalog};
use crate::rules_core::cpr::skills::{skill_rows, slug, CprSkillCatalog};
use crate::rules_core::cpr::stats::{current_emp, derived_rows, hit_points};
use crate::rules_core::pilot_compute::ComputationExplanation;

/// The Cyberarm option that adds a deck slot (p. 365, "Cyberdeck (installed)": "Adds 1 extra
/// Program/Hardware Slot", A-7).
pub const CYBERARM_DECK: &str = "Cyberdeck (installed)";

/// One `cpr.stat.<id>` row per STAT, in book order, each equal to the entered value.
pub fn stat_rows(input: &CprCharacterInput) -> Vec<ComputationExplanation> {
    CprStat::ALL
        .into_iter()
        .map(|stat| {
            let value = input.stats.get(stat);
            ComputationExplanation {
                id: format!("cpr.stat.{}", stat.id()),
                value: i16::from(value),
                detail: format!(
                    "{} {value}: the character's {} STAT as entered (Cyberpunk RED core rulebook p. 72-73)",
                    stat.label(),
                    stat.name()
                ),
            }
        })
        .collect()
}

/// How a printed line reads: dice the player rolls, or rule words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CprPrintedForm {
    Dice,
    Words,
}

/// A line the sheet prints, not a computed number: dice stay literal, rules stay words.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprPrintedLine {
    pub id: String,
    pub label: String,
    pub form: CprPrintedForm,
    /// The dice as written (`3d6`), or the few words a sheet cell shows (`Yes`, `N/A`, a
    /// weapon's name); empty for a rule paragraph, which is the prose alone.
    pub value: String,
    pub prose: String,
}

/// One composed sheet: every number as a `cpr.` row, every dice or rule line printed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprSheet {
    pub rows: Vec<ComputationExplanation>,
    pub printed: Vec<CprPrintedLine>,
}

#[derive(Debug, Clone, Deserialize)]
struct WeaponRecord {
    name: String,
    weapon_class: String,
    damage: Option<String>,
    rof: Option<u8>,
    standard_magazine: Option<u16>,
    /// Hands Required: a number for a ranged weapon (p. 172), words for melee ("Varies by
    /// type", p. 93) and some Exotic Weapons.
    hands: Option<serde_json::Value>,
    can_be_concealed: Option<bool>,
    skill: Option<String>,
    /// The weapon type an Exotic Weapon is built on (p. 348).
    base_weapon: Option<String>,
    source_page: String,
    text: String,
}

/// "Melee weapons use the Melee Weapon Skill to attack" (Cyberpunk RED core rulebook p. 176).
const MELEE_SKILL: &str = "Melee Weapon";

impl WeaponRecord {
    fn attack_skill(&self) -> Option<String> {
        self.skill.clone().or_else(|| (self.weapon_class == "melee").then(|| MELEE_SKILL.to_owned()))
    }
}

#[derive(Debug, Clone, Deserialize)]
struct RangeBand {
    range: String,
    max_m_yds: u16,
    dv: Option<u8>,
}

/// One row of the range DV tables (p. 173): single shot, or Autofire.
#[derive(Debug, Clone, Deserialize)]
struct RangeDvRecord {
    mode: String,
    row_label: String,
    /// The weapon types the row covers (D-1 for the single-shot rows).
    weapon_types: Vec<String>,
    bands: Vec<RangeBand>,
    source_page: String,
}

/// The range DV rows, checked to share one set of range bands (the table's column heads).
fn range_dv_records(corpus_dir: &Path) -> Result<Vec<RangeDvRecord>, String> {
    let records: Vec<RangeDvRecord> = read_records(corpus_dir, "range_dv.json")?;
    let widest = records.iter().max_by_key(|r| r.bands.len()).ok_or("range_dv.json holds no row")?;
    for record in &records {
        for (band, head) in record.bands.iter().zip(&widest.bands) {
            if band.max_m_yds != head.max_m_yds {
                return Err(format!(
                    "range_dv.json: the {} row's band '{}' does not match the table's '{}'",
                    record.row_label, band.range, head.range
                ));
            }
        }
    }
    Ok(records)
}

#[derive(Debug, Clone, Deserialize)]
struct ShieldRecord {
    name: String,
    sp: Option<u8>,
    hp: Option<u8>,
    armor_penalty: String,
    armor_penalty_ref_dex_move: i8,
    source_page: String,
}

#[derive(Debug, Clone, Deserialize)]
struct HousingRecord {
    name: String,
    rent_per_month_eb: Option<u32>,
    rent_per_month_text: String,
    source_page: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ProgramRecord {
    name: String,
    program_class: String,
}

#[derive(Debug, Clone, Deserialize)]
struct HardwareRecord {
    name: String,
    slots: Option<u8>,
}

#[derive(Debug, Clone, Deserialize)]
struct CyberdeckRecord {
    name: String,
    slots: u8,
}

fn read_records<T: for<'de> Deserialize<'de>>(corpus_dir: &Path, file: &str) -> Result<Vec<T>, String> {
    let path = corpus_dir.join(file);
    let text = std::fs::read_to_string(&path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("cannot parse {}: {e}", path.display()))
}

/// Every catalog a sheet is composed from, loaded once from a CPR corpus directory
/// (`GameSystem::CyberpunkRed.package_roots(..).corpus`).
#[derive(Debug, Clone)]
pub struct CprSheetCatalogs {
    skills: CprSkillCatalog,
    armor: CprArmorCatalog,
    critical: CprCriticalCatalog,
    cyberware: CprCyberwareCatalog,
    roles: CprRoleCatalog,
    reputation: CprReputationCatalog,
    weapons: Vec<WeaponRecord>,
    range_dv: Vec<RangeDvRecord>,
    shields: Vec<ShieldRecord>,
    housing: Vec<HousingRecord>,
    programs: Vec<ProgramRecord>,
    hardware: Vec<HardwareRecord>,
    cyberdecks: Vec<CyberdeckRecord>,
}

impl CprSheetCatalogs {
    pub fn load(corpus_dir: &Path) -> Result<Self, String> {
        Ok(CprSheetCatalogs {
            skills: CprSkillCatalog::load(corpus_dir)?,
            armor: CprArmorCatalog::load(corpus_dir)?,
            critical: CprCriticalCatalog::load(corpus_dir)?,
            cyberware: CprCyberwareCatalog::load(corpus_dir)?,
            roles: CprRoleCatalog::load(corpus_dir)?,
            reputation: CprReputationCatalog::load(corpus_dir)?,
            weapons: read_records(corpus_dir, "weapon.json")?,
            range_dv: range_dv_records(corpus_dir)?,
            // The held armour (the Bulletproof Shield) has HP and no SP (p. 350).
            shields: read_records::<ShieldRecord>(corpus_dir, "armor.json")?
                .into_iter()
                .filter(|r| r.sp.is_none() && r.hp.is_some())
                .collect(),
            housing: read_records(corpus_dir, "housing.json")?,
            programs: read_records(corpus_dir, "program.json")?,
            hardware: read_records(corpus_dir, "net_hardware.json")?,
            cyberdecks: read_records(corpus_dir, "cyberdeck.json")?,
        })
    }
}

fn row(id: String, value: i16, detail: String) -> ComputationExplanation {
    ComputationExplanation { id, value, detail }
}

fn sheet_number(what: &str, value: u32) -> Result<i16, String> {
    i16::try_from(value).map_err(|_| format!("{what} {value} is larger than a sheet row holds ({})", i16::MAX))
}

/// The deck block's items, resolved by corpus name: a Black ICE Program takes 2 slots, any
/// other Program 1 (p. 204), Hardware its printed slots (p. 208).
fn deck_items(catalogs: &CprSheetCatalogs, programs: &[String], hardware: &[String]) -> Result<Vec<CprDeckItem>, String> {
    let mut items = Vec::new();
    for name in programs {
        let record = catalogs
            .programs
            .iter()
            .find(|p| &p.name == name)
            .ok_or_else(|| format!("'{name}' is not a Program of the Netrunning chapter (p. 204-207)"))?;
        items.push(if record.program_class == "Black ICE" { CprDeckItem::black_ice(name) } else { CprDeckItem::program(name) });
    }
    for name in hardware {
        let record = catalogs
            .hardware
            .iter()
            .find(|h| &h.name == name)
            .ok_or_else(|| format!("'{name}' is not Cyberdeck Hardware (p. 208)"))?;
        let slots = record.slots.ok_or_else(|| format!("'{name}' is a deck option, not Hardware that takes a slot (p. 208)"))?;
        items.push(CprDeckItem::hardware(name, slots));
    }
    Ok(items)
}

fn words(id: String, label: String, value: String, prose: String) -> CprPrintedLine {
    CprPrintedLine { id, label, form: CprPrintedForm::Words, value, prose }
}

/// The weapon block of weapon `i` beyond ROF, magazine and damage: its name, Hands Required,
/// Can be Concealed?, the Skill Base of its weapon skill, and its range DV rows -- the
/// single-shot row the D-1 mapping gives its type (an Exotic Weapon reads its base type's row)
/// and, for an Autofire weapon, its Autofire row. A band the table prints "N/A" is printed, not
/// a number. `range_bands` is set to the table whose bands the DV rows are read in.
fn weapon_block<'a>(
    catalogs: &'a CprSheetCatalogs,
    i: usize,
    weapon: &WeaponRecord,
    rows: &mut Vec<ComputationExplanation>,
    printed: &mut Vec<CprPrintedLine>,
    range_bands: &mut Option<&'a RangeDvRecord>,
) -> Result<(), String> {
    let name = &weapon.name;
    let page = &weapon.source_page;
    let base = weapon.base_weapon.as_deref().and_then(|b| catalogs.weapons.iter().find(|w| w.name == b));
    printed.push(words(
        format!("cpr.weapon.{i}.name"),
        "Weapon".to_owned(),
        name.clone(),
        format!("{name}: {} weapon ({page})", weapon.weapon_class),
    ));
    match &weapon.hands {
        Some(serde_json::Value::Number(hands)) => {
            let hands = hands.as_u64().and_then(|h| u32::try_from(h).ok()).ok_or_else(|| format!("{name}: Hands Required {hands} is not a count"))?;
            rows.push(row(format!("cpr.weapon.{i}.hands"), sheet_number("hands", hands)?, format!("{name} Hands Required {hands} (Cyberpunk RED core rulebook {page})")));
        }
        Some(serde_json::Value::String(hands)) => printed.push(words(
            format!("cpr.weapon.{i}.hands"),
            format!("{name} hands"),
            hands.clone(),
            format!("{name} Hands Required: {hands} ({page})"),
        )),
        _ => {}
    }
    if let Some(concealable) = weapon.can_be_concealed {
        printed.push(words(
            format!("cpr.weapon.{i}.concealable"),
            format!("{name} concealable"),
            if concealable { "Yes" } else { "No" }.to_owned(),
            format!(
                "{name} {} be concealed under clothing with the Conceal/Reveal Object Skill ({page})",
                if concealable { "can" } else { "cannot" }
            ),
        ));
    }
    if let Some(skill) = weapon.attack_skill().or_else(|| base.and_then(WeaponRecord::attack_skill)) {
        let skill_id = format!("cpr.skill.{}", slug(&skill));
        let skill_base = rows
            .iter()
            .find(|r| r.id == skill_id)
            .map(|r| r.value)
            .ok_or_else(|| format!("{name}: its skill '{skill}' has no Skill Base row ({skill_id})"))?;
        rows.push(row(
            format!("cpr.weapon.{i}.skill_base"),
            skill_base,
            format!("{name} Skill Base {skill_base}: the {skill} Skill Base ({skill_id}), the skill this weapon attacks with (Cyberpunk RED core rulebook {page}; melee, p. 176)"),
        ));
    }
    let typed = |mode: &str, of: &str| catalogs.range_dv.iter().find(|r| r.mode == mode && r.weapon_types.iter().any(|t| t == of));
    let single = typed("single_shot", name).or_else(|| base.and_then(|b| typed("single_shot", &b.name)));
    for (record, field, what) in [(single, "range_dv", "range DV"), (typed("autofire", name), "autofire_dv", "Autofire DV")] {
        let Some(record) = record else { continue };
        for (b, band) in record.bands.iter().enumerate() {
            let id = format!("cpr.weapon.{i}.{field}.{b}");
            match band.dv {
                Some(dv) => rows.push(row(
                    id,
                    i16::from(dv),
                    format!(
                        "{name} {what} {dv} at {} (the {} row, D-1; Cyberpunk RED core rulebook {})",
                        band.range, record.row_label, record.source_page
                    ),
                )),
                None => printed.push(words(
                    id,
                    format!("{name} {what} at {}", band.range),
                    "N/A".to_owned(),
                    format!("{name}: no {what} at {}; the {} row prints N/A ({})", band.range, record.row_label, record.source_page),
                )),
            }
        }
        if range_bands.is_none_or(|t| t.bands.len() < record.bands.len()) {
            *range_bands = Some(record);
        }
    }
    Ok(())
}

/// Composes the whole sheet of one character. A choice the book does not allow (an unknown
/// name, a rank outside 1-10, a cyberware install over its slots, a deck without the
/// Netrunner Role, more Improvement Points spent than earned) is refused with its reason;
/// no partial sheet is returned.
pub fn compose_sheet(
    catalogs: &CprSheetCatalogs,
    input: &CprCharacterInput,
    choices: &CprSheetChoices,
) -> Result<CprSheet, String> {
    let entered = input.stats;
    let base_body = entered.get(CprStat::Body);
    let base_emp = entered.get(CprStat::Emp);
    let mut rows = stat_rows(input);
    let mut printed = Vec::new();

    // 1. Cyberware.
    let loadout = catalogs.cyberware.check(base_body, &choices.cyberware).map_err(|e| e.to_string())?;
    let cyberware_rows = loadout.rows(base_emp);
    let cyberware_sp = |id: &str| {
        cyberware_rows.iter().find(|r| r.id == id).map_or(0, |r| u8::try_from(r.value).unwrap_or(0))
    };
    let body = loadout.body();

    // 2. Humanity and EMP after the Humanity Loss.
    let humanity_loss = loadout.humanity_loss();
    let humanity_now = humanity_after_loss(base_emp, humanity_loss);
    let emp_now = current_emp(humanity_now);

    // 3. Armour, the cyberware SP competing in its location.
    let mut sources = Vec::new();
    for (location, id) in [(CprArmorLocation::Head, "cpr.cyberware.sp_head"), (CprArmorLocation::Body, "cpr.cyberware.sp_body")] {
        let sp = cyberware_sp(id);
        if sp > 0 {
            sources.push(CprSpSource { label: "cyberware".to_owned(), location, sp });
        }
    }
    let armor = catalogs.armor.wear(&choices.armor, &sources)?;

    // 4. Derived STATs and wound thresholds from the cyberware BODY.
    let mut derived_stats = entered;
    derived_stats.set(CprStat::Body, body);
    rows.extend(derived_rows(&CprCharacterInput { stats: derived_stats }, humanity_now));
    rows.push(row(
        "cpr.derived.humanity_current".to_owned(),
        humanity_now,
        format!(
            "Humanity {humanity_now} = EMP {base_emp} x 10 - Humanity Loss {humanity_loss} (Cyberpunk RED core rulebook p. 80; preset loss at creation, p. 112)"
        ),
    ));
    let hp = hit_points(body, entered.get(CprStat::Will));

    rows.extend(armor.rows(entered.get(CprStat::Ref), entered.get(CprStat::Dex), entered.get(CprStat::Move)));
    for (location, label) in [(CprArmorLocation::Head, "head"), (CprArmorLocation::Body, "body")] {
        let worn: Vec<(&str, i8)> = choices
            .armor
            .iter()
            .filter(|p| p.location == location || p.name == BODYWEIGHT_SUIT)
            .filter_map(|p| catalogs.armor.get(&p.name).map(|a| (a.name.as_str(), a.penalty)))
            .collect();
        let penalty = worn.iter().map(|(_, p)| *p).min().unwrap_or(0);
        let names: Vec<String> = worn.iter().map(|(n, p)| format!("{n} {p}")).collect();
        rows.push(row(
            format!("cpr.armor.penalty_{label}"),
            i16::from(penalty),
            format!(
                "{label} armour penalty {penalty}: the most punishing Armor Penalty worn in the {label} location ({}); the sheet takes it once, as cpr.armor.penalty (Cyberpunk RED core rulebook p. 184, p. 350)",
                if names.is_empty() { "nothing worn".to_owned() } else { names.join(", ") }
            ),
        ));
    }
    if let Some(name) = &choices.shield {
        let shield = catalogs
            .shields
            .iter()
            .find(|r| &r.name == name)
            .ok_or_else(|| format!("'{name}' is not a shield of the Master Armor List (p. 350)"))?;
        let hp = shield.hp.unwrap_or(0);
        rows.push(row(
            "cpr.armor.shield_hp".to_owned(),
            i16::from(hp),
            format!("{} {hp} HP, reduced by damage; a shield has HP instead of SP (Cyberpunk RED core rulebook {})", shield.name, shield.source_page),
        ));
        rows.push(row(
            "cpr.armor.shield_penalty".to_owned(),
            i16::from(shield.armor_penalty_ref_dex_move),
            format!("{} Armor Penalty {}: \"{}\" ({})", shield.name, shield.armor_penalty_ref_dex_move, shield.armor_penalty, shield.source_page),
        ));
    }
    rows.extend(cyberware_rows.iter().cloned());
    // Each installed piece: its name printed, its preset Humanity Loss a row (p. 112).
    for (i, (name, loss)) in loadout.installed().iter().enumerate() {
        printed.push(words(
            format!("cpr.cyberware.piece.{i}.name"),
            "Cyberware".to_owned(),
            name.clone(),
            format!("{name}: installed cyberware (Cyberpunk RED core rulebook p. 112-117, p. 359-368)"),
        ));
        rows.push(row(
            format!("cpr.cyberware.piece.{i}.humanity_loss"),
            *loss,
            format!("{name} Humanity Loss {loss}: the preset loss at Character Generation (p. 112)"),
        ));
    }
    rows.extend(catalogs.critical.rows(hp, body, &choices.criticals)?);

    // 5. Skill bases from the penalised REF and DEX and the current EMP; one level row each.
    let mut skill_stats = entered;
    skill_stats.set(CprStat::Ref, armor.adjusted(entered.get(CprStat::Ref)));
    skill_stats.set(CprStat::Dex, armor.adjusted(entered.get(CprStat::Dex)));
    skill_stats.set(CprStat::Move, armor.adjusted(entered.get(CprStat::Move)));
    skill_stats.set(CprStat::Body, body);
    skill_stats.set(CprStat::Emp, u8::try_from(emp_now).unwrap_or(0));
    let mut held: Vec<(String, String, u8)> = Vec::new();
    for (key, level) in &choices.skills {
        let (skill, specialty) = catalogs.skills.resolve(key)?;
        let (id, label) = match &specialty {
            Some(spec) => (format!("{}.{}", slug(&skill.name), slug(spec)), format!("{} ({spec})", skill.name)),
            None => (slug(&skill.name), skill.name.clone()),
        };
        held.push((id, label, *level));
    }
    for base in skill_rows(&catalogs.skills, &skill_stats, &choices.skills)? {
        let rest = base.id.trim_start_matches("cpr.skill.").to_owned();
        let (label, level) = match held.iter().find(|(id, _, _)| *id == rest) {
            Some((_, label, level)) => (label.clone(), *level),
            None => {
                let name = catalogs.skills.skills().iter().find(|s| slug(&s.name) == rest).map_or(rest.clone(), |s| s.name.clone());
                (name, 0)
            }
        };
        rows.push(base);
        rows.push(row(
            format!("cpr.skill_level.{rest}"),
            i16::from(level),
            format!("{label} level {level}: as entered; Skill Base = STAT + level (Cyberpunk RED core rulebook p. 81)"),
        ));
    }

    // 6. Role Ability and the Netrunner's deck.
    if let Some((role, rank)) = choices.role {
        rows.extend(catalogs.roles.rank_rows(role, rank).map_err(|e| e.to_string())?);
        if role == CprRole::Netrunner {
            let deck = match &choices.deck {
                Some(choice) => {
                    let quality = CprDeckQuality::from_name(&choice.deck)
                        .ok_or_else(|| format!("'{}' is not a Cyberdeck (p. 196)", choice.deck))?;
                    let record = catalogs
                        .cyberdecks
                        .iter()
                        .find(|d| d.name == choice.deck)
                        .ok_or_else(|| format!("'{}' is not a Cyberdeck of the corpus", choice.deck))?;
                    if record.slots != quality.base_slots() {
                        return Err(format!("{}: the corpus gives {} slots, the engine {}", record.name, record.slots, quality.base_slots()));
                    }
                    let extras = CprDeckExtras {
                        bodyweight_suit: armor.bodyweight_suit_worn(),
                        cyberarm_deck: choices.cyberware.iter().any(|c| c.name == CYBERARM_DECK),
                    };
                    let items = deck_items(catalogs, &choice.programs, &choice.hardware)?;
                    // The deck and what it holds, by name, in the order loaded (p. 196-208).
                    printed.push(words(
                        "cpr.netrunning.deck".to_owned(),
                        "Cyberdeck".to_owned(),
                        record.name.clone(),
                        format!("{}: the Netrunner's Cyberdeck (Cyberpunk RED core rulebook p. 196)", record.name),
                    ));
                    for (kind, label, names, pages) in
                        [("program", "Program", &choice.programs, "p. 204-207"), ("hardware", "Hardware", &choice.hardware, "p. 208")]
                    {
                        for (i, name) in names.iter().enumerate() {
                            printed.push(words(
                                format!("cpr.netrunning.{kind}.{i}"),
                                label.to_owned(),
                                name.clone(),
                                format!("{name}: {label} loaded in the {} (Cyberpunk RED core rulebook {pages})", record.name),
                            ));
                        }
                    }
                    Some((quality, extras, items))
                }
                None => None,
            };
            let block = netrunner_sheet(rank, deck.as_ref().map(|(q, e, items)| (*q, *e, items.as_slice())))
                .map_err(|e| e.to_string())?;
            rows.extend(block.rows);
            for (i, prose) in block.printed.into_iter().enumerate() {
                printed.push(CprPrintedLine {
                    id: if i == 0 { "cpr.netrunning.brain_damage".to_owned() } else { format!("cpr.netrunning.printed.{i}") },
                    label: "Brain damage".to_owned(),
                    form: CprPrintedForm::Words,
                    value: String::new(),
                    prose,
                });
            }
        }
    }
    if choices.deck.is_some() && choices.role.map(|(r, _)| r) != Some(CprRole::Netrunner) {
        return Err("a Cyberdeck block needs the Netrunner Role: its NET Actions come from the Interface rank (p. 197)".to_owned());
    }

    // 7. Reputation, then the looked-up and ledger boxes.
    let mut reputation = CprReputation::default();
    for event in &choices.reputation {
        reputation = reputation.record(event.clone()).map_err(|e| e.to_string())?.0;
    }
    rows.extend(reputation_rows(entered.get(CprStat::Cool), &reputation, &catalogs.reputation));

    let mut range_bands: Option<&RangeDvRecord> = None;
    for (i, name) in choices.weapons.iter().enumerate() {
        let weapon = catalogs
            .weapons
            .iter()
            .find(|w| &w.name == name)
            .ok_or_else(|| format!("'{name}' is not a weapon of the weapon tables (p. 341-345)"))?;
        if let Some(rof) = weapon.rof {
            rows.push(row(format!("cpr.weapon.{i}.rof"), i16::from(rof), format!("{name} ROF {rof} (Cyberpunk RED core rulebook {})", weapon.source_page)));
        }
        if let Some(magazine) = weapon.standard_magazine {
            rows.push(row(
                format!("cpr.weapon.{i}.magazine"),
                sheet_number("magazine", u32::from(magazine))?,
                format!("{name} standard magazine {magazine} (Cyberpunk RED core rulebook {})", weapon.source_page),
            ));
        }
        printed.push(match &weapon.damage {
            Some(dice) => CprPrintedLine {
                id: format!("cpr.weapon.{i}.damage"),
                label: format!("{name} damage"),
                form: CprPrintedForm::Dice,
                value: dice.clone(),
                prose: format!("{name}: {dice} single shot damage ({})", weapon.source_page),
            },
            // An Exotic Weapon prints its own rules: damage, ROF and magazine as its text gives them.
            None => CprPrintedLine {
                id: format!("cpr.weapon.{i}.damage"),
                label: format!("{name} damage"),
                form: CprPrintedForm::Words,
                value: String::new(),
                prose: format!("{name}: {} ({})", weapon.text, weapon.source_page),
            },
        });
        weapon_block(catalogs, i, weapon, &mut rows, &mut printed, &mut range_bands)?;
    }
    // The range bands the DV rows above are read in, once: each band's farthest m/yds.
    if let Some(table) = range_bands {
        for (b, band) in table.bands.iter().enumerate() {
            rows.push(row(
                format!("cpr.range_band.{b}.max"),
                sheet_number("range band", u32::from(band.max_m_yds))?,
                format!("Range band {}: {} (Cyberpunk RED core rulebook {})", b + 1, band.range, table.source_page),
            ));
        }
    }

    let ip = choices.improvement_points.unwrap_or_default();
    if ip.spent > ip.earned {
        return Err(format!("Improvement Points: spent {} is more than the {} earned", ip.spent, ip.earned));
    }
    rows.push(row(
        "cpr.ip.current".to_owned(),
        sheet_number("Improvement Points", ip.earned - ip.spent)?,
        format!("Improvement Points {} = {} earned - {} spent (Cyberpunk RED core rulebook p. 410-411)", ip.earned - ip.spent, ip.earned, ip.spent),
    ));
    rows.push(row(
        "cpr.ip.total".to_owned(),
        sheet_number("Improvement Points", ip.earned)?,
        format!("Improvement Points earned {}: the GM's awards as entered (p. 410)", ip.earned),
    ));
    if let Some(cash) = choices.cash {
        rows.push(row("cpr.cash".to_owned(), sheet_number("cash", cash)?, format!("Cash {cash}eb as entered")));
    }
    for (i, (label, count)) in choices.ammunition.iter().enumerate() {
        rows.push(row(format!("cpr.ammo.{i}"), sheet_number("ammunition", *count)?, format!("{label}: {count} as entered")));
    }
    if let Some(name) = &choices.housing {
        let housing = catalogs
            .housing
            .iter()
            .find(|h| &h.name == name)
            .ok_or_else(|| format!("'{name}' is not a Housing option (p. 378-379)"))?;
        match housing.rent_per_month_eb {
            Some(rent) => rows.push(row(
                "cpr.housing.rent".to_owned(),
                sheet_number("rent", rent)?,
                format!("{name} rent {rent}eb a month (Cyberpunk RED core rulebook {})", housing.source_page),
            )),
            None => printed.push(CprPrintedLine {
                id: "cpr.housing.rent".to_owned(),
                label: "Rent".to_owned(),
                form: CprPrintedForm::Words,
                value: String::new(),
                prose: format!("{name} rent: {} ({})", housing.rent_per_month_text, housing.source_page),
            }),
        }
    }

    Ok(CprSheet { rows, printed })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::cpr::armor::{CprArmorLocation, CprWornArmor};
    use crate::rules_core::cpr::critical::{CprCriticalLocation, CprRecordedCritical};
    use crate::rules_core::cpr::cyberware::{CprCyberwareInstall, CprMount};
    use crate::rules_core::cpr::input::{
        CprCharacterInput, CprDeckChoice, CprImprovementPoints, CprSheetChoices, CprStat, CprStats,
    };
    use crate::rules_core::cpr::reputation::{CprReputationEvent, CprReputationKind};
    use crate::rules_core::cpr::roles::CprRole;
    use crate::rules_core::cpr::skills::slug;
    use crate::rules_core::game_system::GameSystem;
    use serde_json::Value;
    use std::collections::BTreeSet;
    use std::path::Path;

    fn catalogs() -> CprSheetCatalogs {
        let corpus = Path::new(env!("CARGO_MANIFEST_DIR")).join(GameSystem::CyberpunkRed.corpus_relative());
        CprSheetCatalogs::load(&corpus).expect("the CPR corpus loads")
    }

    fn stats(values: [u8; 10]) -> CprStats {
        let mut stats = CprStats::default();
        for (stat, value) in CprStat::ALL.into_iter().zip(values) {
            stats.set(stat, value);
        }
        stats
    }

    fn install(name: &str) -> CprCyberwareInstall {
        CprCyberwareInstall { name: name.to_owned(), mount: None }
    }

    fn row(sheet: &CprSheet, id: &str) -> i16 {
        sheet.rows.iter().find(|r| r.id == id).unwrap_or_else(|| panic!("no row {id}")).value
    }

    fn has_row(sheet: &CprSheet, id: &str) -> bool {
        sheet.rows.iter().any(|r| r.id == id)
    }

    /// A Solo with cyberware that changes BODY, SP, Initiative and EMP, armour with a penalty,
    /// a shield, a weapon, a Reputation event, a critical injury and the ledger boxes. Every
    /// expected value below is read from the page (cited), never from the corpus:
    /// Humanity Loss presets Neural Link 7, Kerenzikov 14, Grafted Muscle and Bone Lace 14,
    /// Skin Weave 7 (inventory `cyberware.md` §1 rows 8, 12, 50, 59; p. 360, 364, 365);
    /// Light Armorjack SP 11 no penalty, Flak SP 15 -4 REF/DEX/MOVE, Bulletproof Shield 10 HP
    /// (file p351); Heavy Pistol 3d6, magazine 8, ROF 2 (file p342); Cargo Container rent
    /// 1,000eb (file p379).
    fn kestrel() -> (CprCharacterInput, CprSheetChoices) {
        // INT REF DEX TECH COOL WILL LUCK MOVE BODY EMP
        let input = CprCharacterInput { stats: stats([5, 8, 7, 4, 6, 5, 6, 6, 6, 6]) };
        let choices = CprSheetChoices {
            role: Some((CprRole::Solo, 4)),
            skills: [
                ("Handgun", 4),
                ("Brawling", 2),
                ("Conversation", 2),
                ("Perception", 3),
                ("Language (Streetslang)", 2),
                ("Language (Japanese)", 2),
                ("Local Expert (Your Home)", 2),
                ("Science (Chemistry)", 1),
                ("Play Instrument (Guitar)", 1),
                ("Athletics", 2),
                ("Endurance", 2),
                ("Martial Arts (Karate)", 1),
            ]
            .into_iter()
            .map(|(k, l)| (k.to_owned(), l))
            .collect(),
            armor: vec![
                CprWornArmor { name: "Light Armorjack".to_owned(), location: CprArmorLocation::Head },
                CprWornArmor { name: "Flak".to_owned(), location: CprArmorLocation::Body },
            ],
            shield: Some("Bulletproof Shield".to_owned()),
            cyberware: vec![
                install("Neural Link"),
                install("Kerenzikov"),
                install("Grafted Muscle and Bone Lace"),
                install("Skin Weave"),
            ],
            criticals: vec![CprRecordedCritical { location: CprCriticalLocation::Body, name: "Broken Ribs".to_owned() }],
            reputation: vec![CprReputationEvent { level: 3, kind: CprReputationKind::Deed, description: "Held the bridge".to_owned() }],
            weapons: vec!["Heavy Pistol".to_owned()],
            deck: None,
            improvement_points: Some(CprImprovementPoints { earned: 40, spent: 20 }),
            cash: Some(500),
            ammunition: vec![("Heavy Pistol Basic".to_owned(), 30)],
            housing: Some("Cargo Container".to_owned()),
        };
        (input, choices)
    }

    fn compose((input, choices): &(CprCharacterInput, CprSheetChoices)) -> Result<CprSheet, String> {
        compose_sheet(&catalogs(), input, choices)
    }

    #[test]
    fn the_carried_stats_are_cpr_stat_rows_equal_to_the_input() {
        let mut stats = CprStats::default();
        for (i, stat) in CprStat::ALL.into_iter().enumerate() {
            stats.set(stat, 1 + i as u8);
        }
        let rows = stat_rows(&CprCharacterInput { stats });

        let ids: Vec<&str> = rows.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(
            ids,
            [
                "cpr.stat.int", "cpr.stat.ref", "cpr.stat.dex", "cpr.stat.tech", "cpr.stat.cool",
                "cpr.stat.will", "cpr.stat.luck", "cpr.stat.move", "cpr.stat.body", "cpr.stat.emp",
            ]
        );
        let values: Vec<i16> = rows.iter().map(|r| r.value).collect();
        assert_eq!(values, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(rows[9].detail, "EMP 10: the character's Empathy STAT as entered (Cyberpunk RED core rulebook p. 72-73)");
    }

    /// p. 364: Grafted Muscle "increases their BODY by 2. This increase in BODY changes a
    /// Character's HP and Death Save." BODY 6 -> 8; HP = 10 + 5 x ceil((8 + WILL 5) / 2) = 45.
    #[test]
    fn cyberware_body_feeds_hit_points_seriously_wounded_and_both_death_saves() {
        let sheet = compose(&kestrel()).expect("composes");
        assert_eq!(row(&sheet, "cpr.stat.body"), 6, "the entered STAT stays as entered");
        assert_eq!(row(&sheet, "cpr.cyberware.body"), 8);
        assert_eq!(row(&sheet, "cpr.derived.hit_points"), 45);
        assert_eq!(row(&sheet, "cpr.derived.seriously_wounded"), 23);
        assert_eq!(row(&sheet, "cpr.wound.seriously_wounded"), 23);
        assert_eq!(row(&sheet, "cpr.derived.death_save"), 8);
        assert_eq!(row(&sheet, "cpr.wound.death_save"), 8);
        assert_eq!(row(&sheet, "cpr.wound.base_death_save_penalty"), 0, "Broken Ribs does not raise it");
        assert_eq!(row(&sheet, "cpr.cyberware.initiative_bonus"), 2, "Kerenzikov, p. 360");
    }

    /// Humanity Loss 7 + 14 + 14 + 7 = 42 from EMP 6: Humanity 60 - 42 = 18, EMP 1 (p. 81),
    /// maximum Humanity 60 - 4 pieces x 2 = 52 (p. 231). The lowered EMP is the EMP the
    /// EMP skills use (p. 81: "you will sometimes have to lower your EMP Statistic").
    #[test]
    fn cyberware_humanity_loss_feeds_current_humanity_current_emp_and_emp_skills() {
        let sheet = compose(&kestrel()).expect("composes");
        assert_eq!(row(&sheet, "cpr.cyberware.humanity_loss"), 42);
        assert_eq!(row(&sheet, "cpr.derived.humanity_max"), 60);
        assert_eq!(row(&sheet, "cpr.cyberware.humanity_cap"), 52);
        assert_eq!(row(&sheet, "cpr.derived.humanity_current"), 18);
        assert_eq!(row(&sheet, "cpr.derived.emp_current"), 1);
        assert_eq!(row(&sheet, "cpr.skill.conversation"), 1 + 2);
        assert_eq!(row(&sheet, "cpr.skill.human_perception"), 1);
    }

    /// Light Armorjack head (11) + Flak body (15, -4) + Skin Weave (7 head and body): SP is the
    /// highest source per location (p. 184, p. 365); the -4 lands on REF 8 -> 4, DEX 7 -> 3,
    /// MOVE 6 -> 2 and on every REF and DEX skill base; INT and WILL skills are untouched.
    #[test]
    fn armour_and_cyberware_sp_and_the_armour_penalty_feed_the_reflex_and_dexterity_skill_bases() {
        let sheet = compose(&kestrel()).expect("composes");
        assert_eq!(row(&sheet, "cpr.cyberware.sp_head"), 7);
        assert_eq!(row(&sheet, "cpr.armor.sp_head"), 11);
        assert_eq!(row(&sheet, "cpr.armor.sp_body"), 15);
        assert_eq!(row(&sheet, "cpr.armor.penalty"), -4);
        assert_eq!(row(&sheet, "cpr.armor.penalty_head"), 0);
        assert_eq!(row(&sheet, "cpr.armor.penalty_body"), -4);
        assert_eq!(row(&sheet, "cpr.armor.shield_hp"), 10);
        assert_eq!(row(&sheet, "cpr.armor.shield_penalty"), 0);
        assert_eq!((row(&sheet, "cpr.armor.ref"), row(&sheet, "cpr.armor.dex"), row(&sheet, "cpr.armor.move")), (4, 3, 2));
        assert_eq!(row(&sheet, "cpr.skill.handgun"), 4 + 4);
        assert_eq!(row(&sheet, "cpr.skill.archery"), 4, "an untrained REF skill is the penalised REF");
        assert_eq!(row(&sheet, "cpr.skill.brawling"), 3 + 2);
        assert_eq!(row(&sheet, "cpr.skill.martial_arts.karate"), 3 + 1);
        assert_eq!(row(&sheet, "cpr.skill.perception"), 5 + 3);
        assert_eq!(row(&sheet, "cpr.skill.endurance"), 5 + 2);
        assert_eq!(row(&sheet, "cpr.skill_level.handgun"), 4);
        assert_eq!(row(&sheet, "cpr.skill_level.archery"), 0);
        assert_eq!(row(&sheet, "cpr.skill_level.language.streetslang"), 2);

        // Leathers (SP 4, file p351) on the body and nothing on the head: Skin Weave's SP 7 is
        // the highest source in both locations, and no penalty reaches REF.
        let (input, mut choices) = kestrel();
        choices.armor = vec![CprWornArmor { name: "Leathers".to_owned(), location: CprArmorLocation::Body }];
        let sheet = compose(&(input, choices)).expect("composes");
        assert_eq!((row(&sheet, "cpr.armor.sp_head"), row(&sheet, "cpr.armor.sp_body")), (7, 7));
        assert_eq!((row(&sheet, "cpr.armor.penalty"), row(&sheet, "cpr.armor.ref"), row(&sheet, "cpr.skill.handgun")), (0, 8, 12));
    }

    #[test]
    fn role_reputation_weapons_and_the_ledger_boxes_are_rows_and_dice_stay_printed() {
        let sheet = compose(&kestrel()).expect("composes");
        assert_eq!(row(&sheet, "cpr.role.solo.rank"), 4);
        assert_eq!(row(&sheet, "cpr.reputation.level"), 3);
        assert_eq!(row(&sheet, "cpr.reputation.facedown"), 6 + 3);
        assert_eq!(row(&sheet, "cpr.weapon.0.rof"), 2);
        assert_eq!(row(&sheet, "cpr.weapon.0.magazine"), 8);
        assert_eq!((row(&sheet, "cpr.ip.current"), row(&sheet, "cpr.ip.total")), (20, 40));
        assert_eq!(row(&sheet, "cpr.cash"), 500);
        assert_eq!(row(&sheet, "cpr.ammo.0"), 30);
        assert_eq!(row(&sheet, "cpr.housing.rent"), 1000);
        let damage = sheet.printed.iter().find(|p| p.id == "cpr.weapon.0.damage").expect("the damage line");
        assert_eq!((damage.form, damage.value.as_str()), (CprPrintedForm::Dice, "3d6"));
        assert!(!sheet.rows.iter().any(|r| r.id.starts_with("cpr.netrunning.")), "a Solo carries no deck block");
    }

    /// The 11 ranged weapon types of the weapon table (file p172) and a melee weapon (file p093),
    /// carried by the armoured Kestrel (REF 8 -> 4 under Flak, Handgun 4). Every expected value is
    /// read from the page: Hands Required and Can be Concealed? (p172; melee "Varies by type",
    /// p093), the weapon's Skill (p172; "Melee weapons use the Melee Weapon Skill", file p177),
    /// and the single-shot and Autofire range DV rows (file p174), with the D-1 mapping of the
    /// 11 types onto the 8 single-shot rows (decisions.md §20.1).
    #[test]
    fn every_ranged_weapon_carries_hands_concealability_its_skill_base_and_its_d1_range_dv_row() {
        const NA: Option<i16> = None;
        let pistol = [Some(13), Some(15), Some(20), Some(25), Some(30), Some(30), NA, NA];
        let smg = [Some(15), Some(13), Some(15), Some(20), Some(25), Some(25), Some(30), NA];
        let shotgun = [Some(13), Some(15), Some(20), Some(25), Some(30), Some(35), NA, NA];
        let assault = [Some(17), Some(16), Some(15), Some(13), Some(15), Some(20), Some(25), Some(30)];
        let sniper = [Some(30), Some(25), Some(25), Some(20), Some(15), Some(16), Some(17), Some(20)];
        let bows = [Some(15), Some(13), Some(15), Some(17), Some(20), Some(22), NA, NA];
        let grenade = [Some(16), Some(15), Some(15), Some(17), Some(20), Some(22), Some(25), NA];
        let rocket = [Some(17), Some(16), Some(15), Some(15), Some(20), Some(20), Some(25), Some(30)];
        // (name, hands, concealable, skill base, single-shot row, autofire row)
        type Expected = (&'static str, i16, &'static str, i16, [Option<i16>; 8], Option<[i16; 5]>);
        let handgun = 4 + 4;
        let ref_only = 4;
        let table: [Expected; 11] = [
            ("Medium Pistol", 1, "Yes", handgun, pistol, None),
            ("Heavy Pistol", 1, "Yes", handgun, pistol, None),
            ("Very Heavy Pistol", 1, "No", handgun, pistol, None),
            ("SMG", 1, "Yes", handgun, smg, Some([20, 17, 20, 25, 30])),
            ("Heavy SMG", 1, "No", handgun, smg, Some([20, 17, 20, 25, 30])),
            ("Shotgun", 2, "No", ref_only, shotgun, None),
            ("Assault Rifle", 2, "No", ref_only, assault, Some([22, 20, 17, 20, 25])),
            ("Sniper Rifle", 2, "No", ref_only, sniper, None),
            ("Bows & Crossbows", 2, "No", ref_only, bows, None),
            ("Grenade Launcher", 2, "No", ref_only, grenade, None),
            ("Rocket Launcher", 2, "No", ref_only, rocket, None),
        ];
        let (input, mut choices) = kestrel();
        choices.weapons = table.iter().map(|w| w.0.to_owned()).chain(["Heavy Melee Weapon".to_owned()]).collect();
        let sheet = compose(&(input, choices)).expect("composes");
        let has = |id: &str| sheet.rows.iter().any(|r| r.id == id);
        let line = |id: &str| sheet.printed.iter().find(|p| p.id == id).unwrap_or_else(|| panic!("no printed line {id}"));

        for (i, (name, hands, concealable, skill_base, single, autofire)) in table.iter().enumerate() {
            let at = |field: &str| format!("cpr.weapon.{i}.{field}");
            assert_eq!(line(&at("name")).value, *name);
            assert_eq!(row(&sheet, &at("hands")), *hands, "{name} Hands Required");
            assert_eq!((line(&at("concealable")).form, line(&at("concealable")).value.as_str()), (CprPrintedForm::Words, *concealable), "{name}");
            assert_eq!(row(&sheet, &at("skill_base")), *skill_base, "{name} Skill Base");
            for (band, dv) in single.iter().enumerate() {
                let id = at(&format!("range_dv.{band}"));
                match dv {
                    Some(dv) => assert_eq!(row(&sheet, &id), *dv, "{name} band {band}"),
                    None => {
                        assert!(!has(&id), "{name} band {band} is N/A, not a number");
                        assert_eq!(line(&id).value, "N/A", "{name} band {band}");
                    }
                }
            }
            match autofire {
                Some(dvs) => {
                    for (band, dv) in dvs.iter().enumerate() {
                        assert_eq!(row(&sheet, &at(&format!("autofire_dv.{band}"))), *dv, "{name} autofire band {band}");
                    }
                }
                None => assert!(!sheet.rows.iter().any(|r| r.id.starts_with(&at("autofire_dv."))), "{name} has no Autofire"),
            }
        }
        // The eight range bands, once: the farthest m/yds of each (file p174).
        let bands: Vec<i16> = (0..8).map(|b| row(&sheet, &format!("cpr.range_band.{b}.max"))).collect();
        assert_eq!(bands, [6, 12, 25, 50, 100, 200, 400, 800]);

        // The melee weapon: Melee Weapon Skill Base (DEX 3 under Flak + 0), hands printed, no range DV.
        assert_eq!(line("cpr.weapon.11.hands").value, "Varies by type");
        assert_eq!(line("cpr.weapon.11.concealable").value, "No");
        assert_eq!(row(&sheet, "cpr.weapon.11.skill_base"), 3);
        assert!(!sheet.rows.iter().any(|r| r.id.starts_with("cpr.weapon.11.range_dv.")), "a melee weapon has no range DV");
    }

    /// Every number is one row: ids are unique, all start `cpr.`, and the twin rows two modules
    /// emit for one sheet value agree.
    #[test]
    fn every_row_is_unique_and_twin_rows_agree() {
        for character in [kestrel(), spider(), (kestrel().0, CprSheetChoices::default())] {
            let sheet = compose(&character).expect("composes");
            let mut seen = BTreeSet::new();
            for r in &sheet.rows {
                assert!(r.id.starts_with("cpr."), "{}", r.id);
                assert!(seen.insert(r.id.clone()), "duplicate row {}", r.id);
            }
            assert_eq!(row(&sheet, "cpr.derived.death_save"), row(&sheet, "cpr.wound.death_save"));
            assert_eq!(row(&sheet, "cpr.derived.seriously_wounded"), row(&sheet, "cpr.wound.seriously_wounded"));
            assert_eq!(row(&sheet, "cpr.derived.hit_points"), row(&sheet, "cpr.wound.lightly_wounded"));
            if has_row(&sheet, "cpr.weapon.0.skill_base") {
                assert_eq!(row(&sheet, "cpr.weapon.0.skill_base"), row(&sheet, "cpr.skill.handgun"), "the Heavy Pistol's Skill Base is the Handgun row");
            }
        }
    }

    /// A Netrunner in a Bodyweight Suit with a Cyberdeck in a Cyberarm: the deck's two extra
    /// slots come from the armour and the cyberware (A-7), so a Standard deck (7, p. 196) has 9;
    /// Interface Rank 4 gives 3 NET Actions (p. 197), on both the Role row and the deck block.
    fn spider() -> (CprCharacterInput, CprSheetChoices) {
        let input = CprCharacterInput { stats: stats([7, 6, 6, 5, 5, 6, 5, 5, 5, 6]) };
        let choices = CprSheetChoices {
            role: Some((CprRole::Netrunner, 4)),
            armor: vec![CprWornArmor { name: "Bodyweight Suit".to_owned(), location: CprArmorLocation::Body }],
            cyberware: vec![
                install("Neural Link"),
                install("Interface Plugs"),
                install("Cyberarm"),
                CprCyberwareInstall { name: "Cyberdeck (installed)".to_owned(), mount: Some(CprMount::Arm(0)) },
            ],
            deck: Some(CprDeckChoice {
                deck: "Cyberdeck (Standard Quality)".to_owned(),
                programs: ["Sword", "Sword", "Killer", "Worm", "Worm", "Armor"].map(str::to_owned).to_vec(),
                hardware: vec!["Range Upgrade".to_owned()],
            }),
            ..CprSheetChoices::default()
        };
        (input, choices)
    }

    #[test]
    fn the_netrunner_deck_takes_its_extra_slots_from_the_armour_and_the_cyberware() {
        let sheet = compose(&spider()).expect("composes");
        assert_eq!(row(&sheet, "cpr.role.netrunner.net_actions"), 3);
        assert_eq!(row(&sheet, "cpr.netrunning.net_actions"), 3);
        assert_eq!(row(&sheet, "cpr.netrunning.deck_slots_available"), 7 + 1 + 1);
        assert_eq!(row(&sheet, "cpr.netrunning.deck_slots_used"), 7 + 1);
        assert_eq!((row(&sheet, "cpr.armor.sp_head"), row(&sheet, "cpr.armor.sp_body")), (11, 11), "p. 350");
        assert!(sheet.printed.iter().any(|p| p.id == "cpr.netrunning.brain_damage" && p.form == CprPrintedForm::Words));
    }

    /// The cyberware block lists each installed piece: its name printed and its preset Humanity
    /// Loss a row (the page values of `kestrel()`'s doc: Neural Link 7, Kerenzikov 14, Grafted
    /// Muscle and Bone Lace 14, Skin Weave 7), summing to the Humanity Loss row. The deck block
    /// prints the deck, each Program and each Hardware by name, in the order loaded (the p. 157
    /// sample deck of `spider()`).
    #[test]
    fn each_cyberware_piece_and_each_deck_item_is_on_the_sheet_by_name() {
        let line = |sheet: &CprSheet, id: &str| {
            let printed = sheet.printed.iter().find(|p| p.id == id).unwrap_or_else(|| panic!("no printed line {id}"));
            assert_eq!(printed.form, CprPrintedForm::Words, "{id}");
            printed.value.clone()
        };
        let sheet = compose(&kestrel()).expect("composes");
        let pieces = [("Neural Link", 7), ("Kerenzikov", 14), ("Grafted Muscle and Bone Lace", 14), ("Skin Weave", 7)];
        for (i, (name, loss)) in pieces.iter().enumerate() {
            assert_eq!(line(&sheet, &format!("cpr.cyberware.piece.{i}.name")), *name);
            assert_eq!(row(&sheet, &format!("cpr.cyberware.piece.{i}.humanity_loss")), *loss, "{name}");
        }
        assert!(!has_row(&sheet, &format!("cpr.cyberware.piece.{}.humanity_loss", pieces.len())), "one row per installed piece");
        assert_eq!(pieces.iter().map(|(_, loss)| loss).sum::<i16>(), row(&sheet, "cpr.cyberware.humanity_loss"));

        let sheet = compose(&spider()).expect("composes");
        assert_eq!(line(&sheet, "cpr.netrunning.deck"), "Cyberdeck (Standard Quality)");
        for (i, name) in ["Sword", "Sword", "Killer", "Worm", "Worm", "Armor"].iter().enumerate() {
            assert_eq!(line(&sheet, &format!("cpr.netrunning.program.{i}")), *name);
        }
        assert!(!sheet.printed.iter().any(|p| p.id == "cpr.netrunning.program.6"), "one line per loaded Program");
        assert_eq!(line(&sheet, "cpr.netrunning.hardware.0"), "Range Upgrade");
        assert!(!compose(&kestrel()).expect("composes").printed.iter().any(|p| p.id.starts_with("cpr.netrunning.")), "a Solo prints no deck");
    }

    #[test]
    fn a_choice_the_book_does_not_allow_is_refused_with_its_reason() {
        let (input, base) = kestrel();
        let refused = |choices: CprSheetChoices| compose_sheet(&catalogs(), &input, &choices).unwrap_err();
        assert!(refused(CprSheetChoices { role: Some((CprRole::Solo, 11)), ..base.clone() }).contains("Rank 11"));
        let unknown_armor = vec![CprWornArmor { name: "Plate Mail".to_owned(), location: CprArmorLocation::Body }];
        assert!(refused(CprSheetChoices { armor: unknown_armor, ..base.clone() }).contains("Plate Mail"));
        let speedware = vec![install("Neural Link"), install("Kerenzikov"), install("Sandevistan")];
        assert!(refused(CprSheetChoices { cyberware: speedware, ..base.clone() }).contains("Speedware"));
        assert!(refused(CprSheetChoices { weapons: vec!["Bat'leth".to_owned()], ..base.clone() }).contains("Bat'leth"));
        assert!(refused(CprSheetChoices { housing: Some("Castle".to_owned()), ..base.clone() }).contains("Castle"));
        let overspent = Some(CprImprovementPoints { earned: 10, spent: 20 });
        assert!(refused(CprSheetChoices { improvement_points: overspent, ..base.clone() }).contains("spent 20"));
        let solo_deck = CprSheetChoices { deck: spider().1.deck, ..base };
        assert!(refused(solo_deck).contains("Netrunner"));
    }

    /// The p121 check. The Sample Character Sheet (file p121, printed p. 120) prints labels and
    /// empty boxes, no values (read from the page image), so the check is the second branch of
    /// the criterion: every numeric field it labels has a `cpr.` row on a composed sheet. The
    /// fixture is the page's own field and skill list; a label the mapping below does not know
    /// fails the test, so the fixture cannot grow a field without a row.
    #[test]
    fn every_numeric_field_the_p121_sample_sheet_labels_has_a_cpr_row() {
        let fixture: Value = serde_json::from_str(include_str!("../../../tests/fixtures/cpr/sample_sheet_p121.json"))
            .expect("the fixture parses");
        assert!(fixture["branch"].as_str().unwrap().starts_with("labels-only"));
        let sheet = compose(&kestrel()).expect("composes");
        let has = |id: &str| sheet.rows.iter().any(|r| r.id == id);
        let printed = |id: &str| sheet.printed.iter().any(|p| p.id == id);

        let fields = fixture["fields"].as_array().unwrap();
        assert_eq!(fields.len(), 29);
        for field in fields {
            let label = field["label"].as_str().unwrap();
            let ids: &[&str] = match label {
                "Role Ability RANK" => &["cpr.role.solo.rank"],
                "INT" => &["cpr.stat.int"],
                "REF" => &["cpr.stat.ref", "cpr.armor.ref"],
                "DEX" => &["cpr.stat.dex", "cpr.armor.dex"],
                "TECH" => &["cpr.stat.tech"],
                "COOL" => &["cpr.stat.cool"],
                "WILL" => &["cpr.stat.will"],
                "LUCK" => &["cpr.stat.luck", "cpr.derived.luck_pool"],
                "MOVE" => &["cpr.stat.move", "cpr.armor.move"],
                "BODY" => &["cpr.stat.body", "cpr.cyberware.body"],
                "EMP" => &["cpr.stat.emp", "cpr.derived.emp_current"],
                "Humanity" => &["cpr.derived.humanity_current", "cpr.cyberware.humanity_cap"],
                "Hit Points" => &["cpr.derived.hit_points"],
                "Seriously Wounded" => &["cpr.derived.seriously_wounded"],
                "Death Save" => &["cpr.derived.death_save"],
                "Armor Head SP" => &["cpr.armor.sp_head"],
                "Armor Head Penalty" => &["cpr.armor.penalty_head"],
                "Armor Body SP" => &["cpr.armor.sp_body"],
                "Armor Body Penalty" => &["cpr.armor.penalty_body"],
                "Armor Shield SP" => &["cpr.armor.shield_hp"],
                "Armor Shield Penalty" => &["cpr.armor.shield_penalty"],
                "Weapon DMG" => {
                    assert!(printed("cpr.weapon.0.damage"), "DMG is dice, printed literal");
                    continue;
                }
                "Weapon Ammo" => &["cpr.weapon.0.magazine"],
                "Weapon ROF" => &["cpr.weapon.0.rof"],
                "Improvement Points" => &["cpr.ip.current", "cpr.ip.total"],
                "Reputation" => &["cpr.reputation.level"],
                "Ammunition" => &["cpr.ammo.0"],
                "Cash" => &["cpr.cash"],
                "Rent" => &["cpr.housing.rent"],
                other => panic!("p121 field '{other}' has no cpr. row mapping"),
            };
            for id in ids {
                assert!(has(id), "p121 field '{label}': no row {id}");
            }
        }

        // Skills: LVL, STAT and BASE per row; BASE = STAT + LVL on the composed sheet.
        let stat_row = |stat: &str| match stat {
            "REF" => "cpr.armor.ref".to_owned(),
            "DEX" => "cpr.armor.dex".to_owned(),
            "EMP" => "cpr.derived.emp_current".to_owned(),
            other => format!("cpr.stat.{}", other.to_ascii_lowercase()),
        };
        let skills = fixture["skills"].as_array().unwrap();
        assert_eq!(skills.len(), 66);
        let mut checked = 0;
        for skill in skills {
            let label = skill["label"].as_str().unwrap();
            let stat = row(&sheet, &stat_row(skill["stat"].as_str().unwrap()));
            let base = format!("cpr.skill.{}", slug(label));
            let mut lines: Vec<String> = Vec::new();
            match skill["sub_rows"].as_array() {
                Some(sub_rows) => {
                    let held: Vec<String> = sheet
                        .rows
                        .iter()
                        .filter(|r| r.id.starts_with(&format!("{base}.")))
                        .map(|r| r.id.clone())
                        .collect();
                    for named in sub_rows.iter().filter_map(Value::as_str) {
                        let id = format!("{base}.{}", slug(named));
                        assert!(held.contains(&id), "p121 sub-row {label} / {named}: no row {id}");
                    }
                    assert!(!held.is_empty() && held.len() <= sub_rows.len(), "{label}: {held:?}");
                    lines.extend(held);
                }
                None if has(&base) => lines.push(base.clone()),
                None => lines.extend(sheet.rows.iter().filter(|r| r.id.starts_with(&format!("{base}."))).map(|r| r.id.clone())),
            }
            assert!(!lines.is_empty(), "p121 skill {label}: no BASE row");
            for line in lines {
                let level = row(&sheet, &line.replacen("cpr.skill.", "cpr.skill_level.", 1));
                assert_eq!(row(&sheet, &line), stat + level, "{line}: BASE = STAT {stat} + LVL {level}");
                checked += 1;
            }
        }
        assert_eq!(checked, 66 - 4 + 2 + 1 + 1 + 1, "62 single rows + Language 2, Local Expert 1, Science 1, Play Instrument 1");
    }
}
