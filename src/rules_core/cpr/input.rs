//! `CprCharacterInput`: a Cyberpunk RED character's own choices, and the text grammar of
//! `cpr_character_input.txt` (one `key = value` line each).
//!
//! E0.4 holds the ten STATs (Cyberpunk RED core rulebook p. 72: "Ten Primary Statistics").
//! [`CprSheetChoices`] holds the rest of what the sheet composes from (Role and rank, skills,
//! armour, cyberware, criticals, Reputation, weapons, deck, Improvement Points, cash,
//! ammunition, housing) in its own file and grammar, `cpr_sheet_choices.txt`.
//! [`CprLifepath`] holds the Lifepath answers (printed text, p. 45-70) in `cpr_lifepath.txt`.
//! [`CprRoleChoices`] holds the Role Ability sub-choices (p. 142-166) in `cpr_role_choices.txt`.

use std::fmt::Write as _;

use crate::rules_core::cpr::armor::{CprArmorLocation, CprWornArmor};
use crate::rules_core::cpr::critical::{CprCriticalLocation, CprRecordedCritical};
use crate::rules_core::cpr::cyberware::{CprCyberwareInstall, CprMount};
use crate::rules_core::cpr::reputation::{CprReputationEvent, CprReputationKind};
use crate::rules_core::cpr::roles::CprRole;

/// The ten STATs, in the order the book's STAT templates list them (p. 74-77 column order).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CprStat {
    Int,
    Ref,
    Dex,
    Tech,
    Cool,
    Will,
    Luck,
    Move,
    Body,
    Emp,
}

impl CprStat {
    pub const ALL: [CprStat; 10] = [
        CprStat::Int,
        CprStat::Ref,
        CprStat::Dex,
        CprStat::Tech,
        CprStat::Cool,
        CprStat::Will,
        CprStat::Luck,
        CprStat::Move,
        CprStat::Body,
        CprStat::Emp,
    ];

    /// The lower-case id used in `cpr_character_input.txt` keys and `cpr.stat.<id>` rows.
    pub const fn id(self) -> &'static str {
        match self {
            CprStat::Int => "int",
            CprStat::Ref => "ref",
            CprStat::Dex => "dex",
            CprStat::Tech => "tech",
            CprStat::Cool => "cool",
            CprStat::Will => "will",
            CprStat::Luck => "luck",
            CprStat::Move => "move",
            CprStat::Body => "body",
            CprStat::Emp => "emp",
        }
    }

    /// The book's abbreviation.
    pub const fn label(self) -> &'static str {
        match self {
            CprStat::Int => "INT",
            CprStat::Ref => "REF",
            CprStat::Dex => "DEX",
            CprStat::Tech => "TECH",
            CprStat::Cool => "COOL",
            CprStat::Will => "WILL",
            CprStat::Luck => "LUCK",
            CprStat::Move => "MOVE",
            CprStat::Body => "BODY",
            CprStat::Emp => "EMP",
        }
    }

    /// The book's full name.
    pub const fn name(self) -> &'static str {
        match self {
            CprStat::Int => "Intelligence",
            CprStat::Ref => "Reflexes",
            CprStat::Dex => "Dexterity",
            CprStat::Tech => "Technique",
            CprStat::Cool => "Cool",
            CprStat::Will => "Willpower",
            CprStat::Luck => "Luck",
            CprStat::Move => "Movement",
            CprStat::Body => "Body",
            CprStat::Emp => "Empathy",
        }
    }

    const fn index(self) -> usize {
        self as usize
    }
}

/// One value per STAT, indexed by [`CprStat`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CprStats([u8; 10]);

impl CprStats {
    pub fn get(&self, stat: CprStat) -> u8 {
        self.0[stat.index()]
    }

    pub fn set(&mut self, stat: CprStat, value: u8) {
        self.0[stat.index()] = value;
    }
}

/// A Cyberpunk RED character's choices: what `cpr_character_input.txt` holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprCharacterInput {
    pub stats: CprStats,
}

const FILE: &str = "cpr_character_input.txt";

/// The `cpr_character_input.txt` text: `stat.<id> = <value>`, one line per STAT, book order.
pub fn render_cpr_character_input(input: &CprCharacterInput) -> String {
    let mut out = String::new();
    for stat in CprStat::ALL {
        let _ = writeln!(out, "stat.{} = {}", stat.id(), input.stats.get(stat));
    }
    out
}

/// Parses `cpr_character_input.txt`. Every STAT must appear exactly once; an unknown key, a
/// line without `=`, or a value that is not a whole number 0-255 is refused with its reason.
pub fn parse_cpr_character_input(text: &str) -> Result<CprCharacterInput, String> {
    let mut seen: [Option<u8>; 10] = [None; 10];
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(format!("{FILE} line missing '=': {raw_line}"));
        };
        let (key, value) = (key.trim(), value.trim());
        let Some(stat) = key
            .strip_prefix("stat.")
            .and_then(|id| CprStat::ALL.into_iter().find(|s| s.id() == id))
        else {
            return Err(format!("{FILE} has an unknown key '{key}'"));
        };
        let parsed = value
            .parse::<u8>()
            .map_err(|_| format!("{FILE} {key} '{value}' is not a whole number from 0 to 255"))?;
        if seen[stat.index()].replace(parsed).is_some() {
            return Err(format!("{FILE} names {key} twice"));
        }
    }
    let mut stats = CprStats::default();
    for stat in CprStat::ALL {
        let value = seen[stat.index()].ok_or_else(|| format!("{FILE} is missing stat.{}", stat.id()))?;
        stats.set(stat, value);
    }
    Ok(CprCharacterInput { stats })
}

/// A Cyberpunk RED character's choices beyond the STATs: everything the composed sheet
/// (`sheet::compose_sheet`) reads. Held in its own file, `cpr_sheet_choices.txt`, beside
/// `cpr_character_input.txt` (`store::CprCharacterStore::save_choices`); a character saved with
/// its STATs only (the creation STAT step) has none of them. The grammar is one `key = value`
/// line each; list-valued keys repeat:
///
/// - `role = <Role>` and `role.rank = <rank>` (both or neither);
/// - `skill.<skill key> = <level>`, the key as the Master Skill List names it
///   (`Handgun`, `Language (Streetslang)`);
/// - `armor.head = <armour>`, `armor.body = <armour>`, `armor.shield = <shield>`;
/// - `cyberware = <name>` or `cyberware = <name> @ <eye|arm|leg|meat_arm|meat_leg> <index>`;
/// - `critical.head = <injury>`, `critical.body = <injury>`;
/// - `reputation = <level> <deed|cowardice>: <description>`;
/// - `weapon = <weapon type>`;
/// - `deck = <Cyberdeck>`, `deck.program = <Program>`, `deck.hardware = <Hardware>`;
/// - `ip.earned = <n>`, `ip.spent = <n>` (spent needs earned);
/// - `cash = <eb>`, `ammo = <count> <ammunition>`, `housing = <housing>`.
///
/// The parser checks the grammar; whether a name is in the book, a rank is in range or a
/// loadout fits is the engine's check when the sheet is composed.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CprSheetChoices {
    pub role: Option<(CprRole, u8)>,
    pub skills: Vec<(String, u8)>,
    pub armor: Vec<CprWornArmor>,
    pub shield: Option<String>,
    pub cyberware: Vec<CprCyberwareInstall>,
    pub criticals: Vec<CprRecordedCritical>,
    pub reputation: Vec<CprReputationEvent>,
    pub weapons: Vec<String>,
    pub deck: Option<CprDeckChoice>,
    pub improvement_points: Option<CprImprovementPoints>,
    pub cash: Option<u32>,
    pub ammunition: Vec<(String, u32)>,
    pub housing: Option<String>,
}

/// A Netrunner's Cyberdeck and what is installed on it, by corpus name.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CprDeckChoice {
    pub deck: String,
    pub programs: Vec<String>,
    pub hardware: Vec<String>,
}

/// The Improvement Points box: earned in total (GM awards) and spent.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CprImprovementPoints {
    pub earned: u32,
    pub spent: u32,
}

const CHOICES_FILE: &str = "cpr_sheet_choices.txt";

fn mount_text(mount: CprMount) -> String {
    let (kind, index) = match mount {
        CprMount::Eye(n) => ("eye", n),
        CprMount::Arm(n) => ("arm", n),
        CprMount::Leg(n) => ("leg", n),
        CprMount::MeatArm(n) => ("meat_arm", n),
        CprMount::MeatLeg(n) => ("meat_leg", n),
    };
    format!("{kind} {index}")
}

fn parse_mount(text: &str) -> Option<CprMount> {
    let (kind, index) = text.trim().split_once(' ')?;
    let index = index.trim().parse::<u8>().ok()?;
    Some(match kind {
        "eye" => CprMount::Eye(index),
        "arm" => CprMount::Arm(index),
        "leg" => CprMount::Leg(index),
        "meat_arm" => CprMount::MeatArm(index),
        "meat_leg" => CprMount::MeatLeg(index),
        _ => return None,
    })
}

fn location_text(location: CprCriticalLocation) -> &'static str {
    match location {
        CprCriticalLocation::Head => "head",
        CprCriticalLocation::Body => "body",
    }
}

/// The `cpr_sheet_choices.txt` text, in the key order of [`CprSheetChoices`]; a choice not made
/// writes no line, so no choices is an empty file.
pub fn render_cpr_sheet_choices(choices: &CprSheetChoices) -> String {
    let mut out = String::new();
    if let Some((role, rank)) = choices.role {
        let _ = writeln!(out, "role = {}\nrole.rank = {rank}", role.name());
    }
    for (key, level) in &choices.skills {
        let _ = writeln!(out, "skill.{key} = {level}");
    }
    for piece in &choices.armor {
        let location = match piece.location {
            CprArmorLocation::Head => "head",
            CprArmorLocation::Body => "body",
        };
        let _ = writeln!(out, "armor.{location} = {}", piece.name);
    }
    if let Some(shield) = &choices.shield {
        let _ = writeln!(out, "armor.shield = {shield}");
    }
    for piece in &choices.cyberware {
        match piece.mount {
            Some(mount) => {
                let _ = writeln!(out, "cyberware = {} @ {}", piece.name, mount_text(mount));
            }
            None => {
                let _ = writeln!(out, "cyberware = {}", piece.name);
            }
        }
    }
    for injury in &choices.criticals {
        let _ = writeln!(out, "critical.{} = {}", location_text(injury.location), injury.name);
    }
    for event in &choices.reputation {
        let kind = match event.kind {
            CprReputationKind::Deed => "deed",
            CprReputationKind::Cowardice => "cowardice",
        };
        let _ = writeln!(out, "reputation = {} {kind}: {}", event.level, event.description);
    }
    for weapon in &choices.weapons {
        let _ = writeln!(out, "weapon = {weapon}");
    }
    if let Some(deck) = &choices.deck {
        let _ = writeln!(out, "deck = {}", deck.deck);
        for program in &deck.programs {
            let _ = writeln!(out, "deck.program = {program}");
        }
        for hardware in &deck.hardware {
            let _ = writeln!(out, "deck.hardware = {hardware}");
        }
    }
    if let Some(ip) = choices.improvement_points {
        let _ = writeln!(out, "ip.earned = {}\nip.spent = {}", ip.earned, ip.spent);
    }
    if let Some(cash) = choices.cash {
        let _ = writeln!(out, "cash = {cash}");
    }
    for (label, count) in &choices.ammunition {
        let _ = writeln!(out, "ammo = {count} {label}");
    }
    if let Some(housing) = &choices.housing {
        let _ = writeln!(out, "housing = {housing}");
    }
    out
}

fn once<T>(slot: &mut Option<T>, key: &str, value: T) -> Result<(), String> {
    if slot.replace(value).is_some() {
        return Err(format!("{CHOICES_FILE} names {key} twice"));
    }
    Ok(())
}

fn number<T: std::str::FromStr>(key: &str, value: &str) -> Result<T, String> {
    value.parse::<T>().map_err(|_| format!("{CHOICES_FILE} {key} '{value}' is not a whole number"))
}

/// Parses `cpr_sheet_choices.txt`. An unknown key, a line without `=`, a malformed value, a
/// single-valued key named twice, and a key without the key it belongs to (`role.rank`
/// without `role`, `ip.spent` without `ip.earned`, `deck.program` without `deck`) are refused
/// with their reason.
pub fn parse_cpr_sheet_choices(text: &str) -> Result<CprSheetChoices, String> {
    let mut choices = CprSheetChoices::default();
    let (mut role, mut rank, mut deck, mut earned, mut spent) = (None, None, None, None, None);
    let (mut programs, mut hardware) = (Vec::new(), Vec::new());
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(format!("{CHOICES_FILE} line missing '=': {raw_line}"));
        };
        let (key, value) = (key.trim(), value.trim());
        match key {
            "role" => {
                let parsed = CprRole::from_name(value)
                    .ok_or_else(|| format!("{CHOICES_FILE} role '{value}' is not one of the ten Roles"))?;
                once(&mut role, key, parsed)?;
            }
            "role.rank" => once(&mut rank, key, number::<u8>(key, value)?)?,
            "armor.head" | "armor.body" => choices.armor.push(CprWornArmor {
                name: value.to_owned(),
                location: if key == "armor.head" { CprArmorLocation::Head } else { CprArmorLocation::Body },
            }),
            "armor.shield" => once(&mut choices.shield, key, value.to_owned())?,
            "cyberware" => {
                let piece = match value.split_once('@') {
                    Some((name, mount)) => CprCyberwareInstall {
                        name: name.trim().to_owned(),
                        mount: Some(parse_mount(mount).ok_or_else(|| {
                            format!("{CHOICES_FILE} cyberware '{value}': the mount is '<eye|arm|leg|meat_arm|meat_leg> <index>'")
                        })?),
                    },
                    None => CprCyberwareInstall { name: value.to_owned(), mount: None },
                };
                choices.cyberware.push(piece);
            }
            "critical.head" | "critical.body" => choices.criticals.push(CprRecordedCritical {
                location: if key == "critical.head" { CprCriticalLocation::Head } else { CprCriticalLocation::Body },
                name: value.to_owned(),
            }),
            "reputation" => {
                let malformed = || format!("{CHOICES_FILE} reputation '{value}' is not '<level> <deed|cowardice>: <description>'");
                let (head, description) = value.split_once(':').ok_or_else(malformed)?;
                let (level, kind) = head.trim().split_once(' ').ok_or_else(malformed)?;
                let kind = match kind.trim() {
                    "deed" => CprReputationKind::Deed,
                    "cowardice" => CprReputationKind::Cowardice,
                    _ => return Err(malformed()),
                };
                let level = level.parse::<u8>().map_err(|_| malformed())?;
                choices.reputation.push(CprReputationEvent { level, kind, description: description.trim().to_owned() });
            }
            "weapon" => choices.weapons.push(value.to_owned()),
            "deck" => once(&mut deck, key, value.to_owned())?,
            "deck.program" => programs.push(value.to_owned()),
            "deck.hardware" => hardware.push(value.to_owned()),
            "ip.earned" => once(&mut earned, key, number::<u32>(key, value)?)?,
            "ip.spent" => once(&mut spent, key, number::<u32>(key, value)?)?,
            "cash" => once(&mut choices.cash, key, number::<u32>(key, value)?)?,
            "ammo" => {
                let (count, label) = value
                    .split_once(' ')
                    .ok_or_else(|| format!("{CHOICES_FILE} ammo '{value}' is not '<count> <ammunition>'"))?;
                choices.ammunition.push((label.trim().to_owned(), number::<u32>(key, count)?));
            }
            "housing" => once(&mut choices.housing, key, value.to_owned())?,
            _ => match key.strip_prefix("skill.") {
                Some(skill) if !skill.trim().is_empty() => {
                    choices.skills.push((skill.trim().to_owned(), number::<u8>(key, value)?));
                }
                _ => return Err(format!("{CHOICES_FILE} has an unknown key '{key}'")),
            },
        }
    }
    choices.role = match (role, rank) {
        (Some(role), Some(rank)) => Some((role, rank)),
        (None, None) => None,
        (Some(_), None) => return Err(format!("{CHOICES_FILE} names role without role.rank")),
        (None, Some(_)) => return Err(format!("{CHOICES_FILE} names role.rank without role")),
    };
    choices.deck = match deck {
        Some(deck) => Some(CprDeckChoice { deck, programs, hardware }),
        None if !programs.is_empty() => return Err(format!("{CHOICES_FILE} names deck.program without deck")),
        None if !hardware.is_empty() => return Err(format!("{CHOICES_FILE} names deck.hardware without deck")),
        None => None,
    };
    choices.improvement_points = match (earned, spent) {
        (Some(earned), spent) => Some(CprImprovementPoints { earned, spent: spent.unwrap_or(0) }),
        (None, None) => None,
        (None, Some(_)) => return Err(format!("{CHOICES_FILE} names ip.spent without ip.earned")),
    };
    Ok(choices)
}

/// A character's Lifepath answers (p. 45-70), in the creation panel's order: printed text,
/// no number (the Cultural Origin language is saved as a skill in [`CprSheetChoices`]). Held in
/// its own file, `cpr_lifepath.txt`, one line per answer:
/// `lifepath = <table> | <entry> | <column> | <answer> | <detail>`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CprLifepath {
    pub answers: Vec<CprLifepathAnswer>,
}

/// One Lifepath answer: the `lifepath_table` record's name, the entry (0 for a table answered
/// once, 1.. for the nth friend, enemy or love affair), the column answered, the cell, and the
/// rest of the row (empty for a per-column answer).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CprLifepathAnswer {
    pub table: String,
    pub entry: u8,
    pub column: String,
    pub text: String,
    pub detail: String,
}

const LIFEPATH_FILE: &str = "cpr_lifepath.txt";

/// Whether a text field fits the `cpr_lifepath.txt` line grammar (no `|`, no line break).
pub fn lifepath_text_ok(text: &str) -> bool {
    !text.contains(['|', '\n', '\r'])
}

/// The `cpr_lifepath.txt` text; no answers is an empty file.
pub fn render_cpr_lifepath(lifepath: &CprLifepath) -> String {
    let mut out = String::new();
    for a in &lifepath.answers {
        let _ = writeln!(out, "lifepath = {} | {} | {} | {} | {}", a.table, a.entry, a.column, a.text, a.detail);
    }
    out
}

/// Parses `cpr_lifepath.txt`. An unknown key, a line without `=`, a line without its five
/// fields, an entry that is not a whole number, or an answer without a table is refused.
pub fn parse_cpr_lifepath(text: &str) -> Result<CprLifepath, String> {
    let mut lifepath = CprLifepath::default();
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(format!("{LIFEPATH_FILE} line missing '=': {raw_line}"));
        };
        let (key, value) = (key.trim(), value.trim());
        if key != "lifepath" {
            return Err(format!("{LIFEPATH_FILE} has an unknown key '{key}'"));
        }
        let fields: Vec<&str> = value.split('|').map(str::trim).collect();
        let [table, entry, column, answer, detail] = fields[..] else {
            return Err(format!(
                "{LIFEPATH_FILE} lifepath '{value}' is not '<table> | <entry> | <column> | <answer> | <detail>'"
            ));
        };
        if table.is_empty() {
            return Err(format!("{LIFEPATH_FILE} lifepath names no table"));
        }
        let entry = entry.parse::<u8>().map_err(|_| format!("{LIFEPATH_FILE} lifepath entry '{entry}' is not a whole number"))?;
        lifepath.answers.push(CprLifepathAnswer {
            table: table.to_owned(),
            entry,
            column: column.to_owned(),
            text: answer.to_owned(),
            detail: detail.to_owned(),
        });
    }
    Ok(lifepath)
}

/// A character's Role Ability sub-choices (p. 142-166; D-7, A-2): the points placed on each
/// ability or specialty the Role divides its rank among (Combat Awareness, Maker, Medicine), and
/// the picks the rank grants (Family Motorpool vehicles and upgrades, Team Members, the drugs of
/// Pharmaceuticals points), each with what it goes on (the vehicle an upgrade is installed on,
/// the chart row a Team Member is hired at). Held in its own file, `cpr_role_choices.txt`, one
/// line each: `points.<name> = <n>`, `pick = <name>` or `pick = <name> @ <on>`. Whether a name,
/// a count or a target is allowed is the engine's check (`roles::CprRoleCatalog::choice_rows`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CprRoleChoices {
    pub points: Vec<(String, u8)>,
    pub picks: Vec<CprRolePick>,
}

/// One Role Ability pick: the name, and what it goes on, if anything.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprRolePick {
    pub name: String,
    pub on: Option<String>,
}

const ROLE_CHOICES_FILE: &str = "cpr_role_choices.txt";

/// Whether a name fits the `cpr_role_choices.txt` line grammar (not blank; no `=`, `@`, `#` or
/// line break).
pub fn role_choice_text_ok(text: &str) -> bool {
    !text.trim().is_empty() && !text.contains(['=', '@', '#', '\n', '\r'])
}

/// The `cpr_role_choices.txt` text: the points, then the picks, each in the order made; no
/// choices is an empty file.
pub fn render_cpr_role_choices(choices: &CprRoleChoices) -> String {
    let mut out = String::new();
    for (name, points) in &choices.points {
        let _ = writeln!(out, "points.{name} = {points}");
    }
    for pick in &choices.picks {
        match &pick.on {
            Some(on) => {
                let _ = writeln!(out, "pick = {} @ {on}", pick.name);
            }
            None => {
                let _ = writeln!(out, "pick = {}", pick.name);
            }
        }
    }
    out
}

/// Parses `cpr_role_choices.txt`. An unknown key, a line without `=`, points that are not a
/// whole number, a name given points twice, and a pick naming nothing are refused.
pub fn parse_cpr_role_choices(text: &str) -> Result<CprRoleChoices, String> {
    let mut choices = CprRoleChoices::default();
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(format!("{ROLE_CHOICES_FILE} line missing '=': {raw_line}"));
        };
        let (key, value) = (key.trim(), value.trim());
        if key == "pick" {
            if value.is_empty() {
                return Err(format!("{ROLE_CHOICES_FILE} pick names nothing"));
            }
            let pick = match value.split_once('@') {
                Some((name, on)) if !on.trim().is_empty() && !name.trim().is_empty() => {
                    CprRolePick { name: name.trim().to_owned(), on: Some(on.trim().to_owned()) }
                }
                Some(_) => return Err(format!("{ROLE_CHOICES_FILE} pick '{value}' names nothing after '@'")),
                None => CprRolePick { name: value.to_owned(), on: None },
            };
            choices.picks.push(pick);
            continue;
        }
        match key.strip_prefix("points.").map(str::trim) {
            Some(name) if !name.is_empty() => {
                let points =
                    value.parse::<u8>().map_err(|_| format!("{ROLE_CHOICES_FILE} {key} '{value}' is not a whole number"))?;
                if choices.points.iter().any(|(n, _)| n == name) {
                    return Err(format!("{ROLE_CHOICES_FILE} names {key} twice"));
                }
                choices.points.push((name.to_owned(), points));
            }
            _ => return Err(format!("{ROLE_CHOICES_FILE} has an unknown key '{key}'")),
        }
    }
    Ok(choices)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> CprCharacterInput {
        let mut stats = CprStats::default();
        for (i, stat) in CprStat::ALL.into_iter().enumerate() {
            stats.set(stat, 2 + i as u8 % 7);
        }
        CprCharacterInput { stats }
    }

    #[test]
    fn the_ten_stats_are_the_books_in_its_order() {
        let labels: Vec<&str> = CprStat::ALL.iter().map(|s| s.label()).collect();
        assert_eq!(labels, ["INT", "REF", "DEX", "TECH", "COOL", "WILL", "LUCK", "MOVE", "BODY", "EMP"]);
        let ids: Vec<&str> = CprStat::ALL.iter().map(|s| s.id()).collect();
        assert_eq!(ids, ["int", "ref", "dex", "tech", "cool", "will", "luck", "move", "body", "emp"]);
    }

    #[test]
    fn render_then_parse_round_trips() {
        let input = sample();
        let text = render_cpr_character_input(&input);
        assert!(text.starts_with("stat.int = 2\nstat.ref = 3\n"), "{text}");
        assert_eq!(parse_cpr_character_input(&text), Ok(input));
    }

    /// Every choice the sheet composes from, rendered then parsed back unchanged.
    fn full() -> CprSheetChoices {
        use crate::rules_core::cpr::armor::{CprArmorLocation, CprWornArmor};
        use crate::rules_core::cpr::critical::{CprCriticalLocation, CprRecordedCritical};
        use crate::rules_core::cpr::cyberware::{CprCyberwareInstall, CprMount};
        use crate::rules_core::cpr::reputation::{CprReputationEvent, CprReputationKind};
        use crate::rules_core::cpr::roles::CprRole;
        CprSheetChoices {
            role: Some((CprRole::Netrunner, 4)),
            skills: vec![("Language (Streetslang)".to_owned(), 2), ("Handgun".to_owned(), 4)],
            armor: vec![
                CprWornArmor { name: "Light Armorjack".to_owned(), location: CprArmorLocation::Head },
                CprWornArmor { name: "Bodyweight Suit".to_owned(), location: CprArmorLocation::Body },
            ],
            shield: Some("Bulletproof Shield".to_owned()),
            cyberware: vec![
                CprCyberwareInstall { name: "Neural Link".to_owned(), mount: None },
                CprCyberwareInstall { name: "Wolvers".to_owned(), mount: Some(CprMount::MeatArm(0)) },
                CprCyberwareInstall { name: "Chyron".to_owned(), mount: Some(CprMount::Eye(1)) },
            ],
            criticals: vec![CprRecordedCritical { location: CprCriticalLocation::Head, name: "Cracked Skull".to_owned() }],
            reputation: vec![
                CprReputationEvent { level: 2, kind: CprReputationKind::Deed, description: "Saved a fixer".to_owned() },
                CprReputationEvent { level: 3, kind: CprReputationKind::Cowardice, description: "Ran: from a fight".to_owned() },
            ],
            weapons: vec!["Heavy Pistol".to_owned(), "Heavy Pistol".to_owned()],
            deck: Some(CprDeckChoice {
                deck: "Cyberdeck (Standard Quality)".to_owned(),
                programs: vec!["Sword".to_owned(), "Killer".to_owned()],
                hardware: vec!["Backup Drive".to_owned()],
            }),
            improvement_points: Some(CprImprovementPoints { earned: 40, spent: 20 }),
            cash: Some(500),
            ammunition: vec![("Heavy Pistol Basic".to_owned(), 30)],
            housing: Some("Cargo Container".to_owned()),
        }
    }

    #[test]
    fn every_sheet_choice_renders_and_parses_back() {
        let choices = full();
        let text = render_cpr_sheet_choices(&choices);
        assert!(text.contains("role = Netrunner\nrole.rank = 4\n"), "{text}");
        assert!(text.contains("skill.Language (Streetslang) = 2\n"), "{text}");
        assert!(text.contains("cyberware = Chyron @ eye 1\n"), "{text}");
        assert!(text.contains("reputation = 3 cowardice: Ran: from a fight\n"), "{text}");
        assert_eq!(parse_cpr_sheet_choices(&text), Ok(choices));
    }

    /// The Lifepath answers (E3.3): one line each, in panel order, parsed back unchanged; an
    /// empty answer list is an empty file.
    #[test]
    fn lifepath_answers_render_and_parse_back() {
        let lifepath = CprLifepath {
            answers: vec![
                CprLifepathAnswer {
                    table: "Cultural Origins".to_owned(),
                    entry: 0,
                    column: "Your (General) Cultural Region".to_owned(),
                    text: "East Asian".to_owned(),
                    detail: "Cantonese Chinese, English, Japanese, Korean, Mandarin Chinese, Mongolian".to_owned(),
                },
                CprLifepathAnswer {
                    table: "Your Enemies".to_owned(),
                    entry: 2,
                    column: "What Caused it? Who's been Wronged? (choose)".to_owned(),
                    text: "You just don't like each other.".to_owned(),
                    detail: String::new(),
                },
            ],
        };
        let text = render_cpr_lifepath(&lifepath);
        assert_eq!(
            text,
            "lifepath = Cultural Origins | 0 | Your (General) Cultural Region | East Asian | Cantonese Chinese, English, Japanese, Korean, Mandarin Chinese, Mongolian\n\
             lifepath = Your Enemies | 2 | What Caused it? Who's been Wronged? (choose) | You just don't like each other. | \n"
        );
        assert_eq!(parse_cpr_lifepath(&text), Ok(lifepath));
        assert_eq!(render_cpr_lifepath(&CprLifepath::default()), "");
        assert_eq!(parse_cpr_lifepath(""), Ok(CprLifepath::default()));
    }

    #[test]
    fn parse_refuses_a_bad_lifepath_line_and_render_refuses_nothing_it_cannot_parse() {
        let refused = |text: &str| parse_cpr_lifepath(text).unwrap_err();
        assert_eq!(refused("answer = x\n"), "cpr_lifepath.txt has an unknown key 'answer'");
        assert_eq!(
            refused("lifepath = Your Friends | 1 | Friend's Relationship to You\n"),
            "cpr_lifepath.txt lifepath 'Your Friends | 1 | Friend's Relationship to You' is not '<table> | <entry> | <column> | <answer> | <detail>'"
        );
        assert_eq!(
            refused("lifepath = Your Friends | one | Who | A teacher or mentor. | \n"),
            "cpr_lifepath.txt lifepath entry 'one' is not a whole number"
        );
        assert_eq!(refused("lifepath = | 0 | C | x | \n"), "cpr_lifepath.txt lifepath names no table");
        assert!(lifepath_text_ok("A teacher or mentor."));
        assert!(!lifepath_text_ok("a | b") && !lifepath_text_ok("two\nlines"));
    }

    /// The Role Ability sub-choices (E3.5): points by ability or specialty, picks with what they
    /// go on, in the order made; none is an empty file.
    #[test]
    fn role_choices_render_and_parse_back() {
        let choices = CprRoleChoices {
            points: vec![("Damage Deflection".to_owned(), 2), ("Threat Detection".to_owned(), 2)],
            picks: vec![
                CprRolePick { name: "Compact Groundcar".to_owned(), on: None },
                CprRolePick { name: "Heavy Chassis".to_owned(), on: Some("Compact Groundcar".to_owned()) },
            ],
        };
        let text = render_cpr_role_choices(&choices);
        assert_eq!(
            text,
            "points.Damage Deflection = 2\npoints.Threat Detection = 2\npick = Compact Groundcar\npick = Heavy Chassis @ Compact Groundcar\n"
        );
        assert_eq!(parse_cpr_role_choices(&text), Ok(choices));
        assert_eq!(render_cpr_role_choices(&CprRoleChoices::default()), "");
        assert_eq!(parse_cpr_role_choices(""), Ok(CprRoleChoices::default()));

        let refused = |text: &str| parse_cpr_role_choices(text).unwrap_err();
        assert_eq!(refused("rank = 4\n"), "cpr_role_choices.txt has an unknown key 'rank'");
        assert_eq!(refused("points.Surgery = two\n"), "cpr_role_choices.txt points.Surgery 'two' is not a whole number");
        assert_eq!(refused("points.Surgery = 1\npoints.Surgery = 2\n"), "cpr_role_choices.txt names points.Surgery twice");
        assert_eq!(refused("pick = \n"), "cpr_role_choices.txt pick names nothing");
        assert_eq!(refused("pick = NOS @ \n"), "cpr_role_choices.txt pick 'NOS @' names nothing after '@'");
        assert!(role_choice_text_ok("High Performance Groundcar") && role_choice_text_ok("Medical Tech (Pharmaceuticals)"));
        assert!(!role_choice_text_ok("a @ b") && !role_choice_text_ok("a = b") && !role_choice_text_ok("x\ny") && !role_choice_text_ok(" "));
    }

    #[test]
    fn no_choices_render_as_an_empty_file_that_parses_back_to_none() {
        assert_eq!(render_cpr_sheet_choices(&CprSheetChoices::default()), "");
        assert_eq!(parse_cpr_sheet_choices(""), Ok(CprSheetChoices::default()));
    }

    #[test]
    fn parse_refuses_bad_sheet_choices_with_their_reason() {
        let refused = |text: &str| parse_cpr_sheet_choices(text).unwrap_err();
        assert_eq!(refused("role = Cowboy\n"), "cpr_sheet_choices.txt role 'Cowboy' is not one of the ten Roles");
        assert_eq!(refused("role = Solo\n"), "cpr_sheet_choices.txt names role without role.rank");
        assert_eq!(refused("role.rank = 4\n"), "cpr_sheet_choices.txt names role.rank without role");
        assert_eq!(refused("armor.arm = Leathers\n"), "cpr_sheet_choices.txt has an unknown key 'armor.arm'");
        assert_eq!(
            refused("cyberware = Chyron @ nose 1\n"),
            "cpr_sheet_choices.txt cyberware 'Chyron @ nose 1': the mount is '<eye|arm|leg|meat_arm|meat_leg> <index>'"
        );
        assert_eq!(
            refused("reputation = 3 brave: x\n"),
            "cpr_sheet_choices.txt reputation '3 brave: x' is not '<level> <deed|cowardice>: <description>'"
        );
        assert_eq!(refused("cash = 5\ncash = 6\n"), "cpr_sheet_choices.txt names cash twice");
        assert_eq!(
            refused("ip.spent = 5\n"),
            "cpr_sheet_choices.txt names ip.spent without ip.earned"
        );
        assert_eq!(refused("deck.program = Sword\n"), "cpr_sheet_choices.txt names deck.program without deck");
    }

    #[test]
    fn parse_refuses_a_missing_stat_an_unknown_key_and_a_non_number() {
        let text = render_cpr_character_input(&sample());
        let without_emp: String = text.lines().filter(|l| !l.starts_with("stat.emp")).map(|l| format!("{l}\n")).collect();
        assert_eq!(parse_cpr_character_input(&without_emp), Err("cpr_character_input.txt is missing stat.emp".to_owned()));
        assert_eq!(
            parse_cpr_character_input(&format!("{text}stat.str = 5\n")),
            Err("cpr_character_input.txt has an unknown key 'stat.str'".to_owned())
        );
        assert_eq!(
            parse_cpr_character_input(&text.replace("stat.int = 2", "stat.int = -1")),
            Err("cpr_character_input.txt stat.int '-1' is not a whole number from 0 to 255".to_owned())
        );
        assert_eq!(
            parse_cpr_character_input(&format!("{text}stat.int = 4\n")),
            Err("cpr_character_input.txt names stat.int twice".to_owned())
        );
    }
}
