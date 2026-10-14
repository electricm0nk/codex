//! Roles: the sheet numbers a Role Ability Rank sets (Cyberpunk RED core rulebook p. 142-165).
//!
//! Computed here, for each of the ten Roles at Role Ability Rank 1-10 (every Role starts at 4,
//! p. 142-143), one `cpr.role.<role>.<field>` row per number the rank puts on the sheet:
//! the rank itself; Charismatic Impact and Credibility check bases (Rank + 1d10, the die stays
//! literal); the Combat Awareness pool; NET Actions per Turn; Maker specialty ranks; Medicine
//! specialty points; Media Believability; Exec Team Members; the Lawman Backup threshold and
//! Backup stat block (D-8: 16/15/14 kept as printed); the Fixer's Haggle bonus; the Nomad's
//! Vehicle Familiarity bonus and Family Motorpool picks (A-2: one per rank).
//!
//! Every table value comes from the `role` and `role_ability` corpus records. Rank-band prose
//! (venues, Reach, Grease, housing, insurance, Motorpool vehicles) is printed, not computed.
//! How a player divides points among specialties or Combat Awareness abilities is a choice,
//! not a rank-derived value (D-7, E3.5).
//!
//! The Role Ability sub-choices (E3.5) are checked here too: [`CprRoleCatalog::choice_menu`]
//! lists what a Role at a rank divides or picks, read from the `role_ability` records, and
//! [`CprRoleCatalog::choice_rows`] refuses a choice the book does not allow or answers with its
//! `cpr.role.<role>.` rows:
//! - Solo, Combat Awareness (p. 147): the rank in points among the six abilities, a tiered
//!   ability only at its printed costs; points left unassigned persist.
//! - Tech, Maker (p. 148, D-7): 2 specialty ranks per Maker Rank, each rank in two different
//!   specialties, so at most one rank per Maker Rank in any one specialty; all placed.
//! - Medtech, Medicine (p. 150-151, D-7): 1 point per rank in Surgery, Pharmaceuticals or
//!   Cryosystem Operation (the last two at most 5); all placed. Surgery gives 2 Surgery skill a
//!   point, the Medical Tech specialties 1 Medical Tech skill a point (each skill at most 10);
//!   each Pharmaceuticals point picks one drug.
//! - Nomad, Family Motorpool (p. 162-166, A-2): one pick per rank, a stock vehicle of the rank
//!   or lower, or an upgrade of the rank or lower on a vehicle already in the pool, within the
//!   upgrade's printed vehicles, repeat rule and Heavy Chassis requirement.
//! - Exec, Teamwork (p. 154-156, A-2): one pick per Team Member the rank gives, the job and the
//!   row of its chart.
//!
//! The other five Roles make no choice.

use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::rules_core::cpr::input::{CprRoleChoices, CprRolePick};
use crate::rules_core::cpr::skills::slug;
use crate::rules_core::pilot_compute::ComputationExplanation;

/// The ten Roles, in the book's order (p. 29).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CprRole {
    Rockerboy,
    Solo,
    Netrunner,
    Tech,
    Medtech,
    Media,
    Exec,
    Lawman,
    Fixer,
    Nomad,
}

impl CprRole {
    pub const ALL: [CprRole; 10] = [
        CprRole::Rockerboy,
        CprRole::Solo,
        CprRole::Netrunner,
        CprRole::Tech,
        CprRole::Medtech,
        CprRole::Media,
        CprRole::Exec,
        CprRole::Lawman,
        CprRole::Fixer,
        CprRole::Nomad,
    ];

    /// The Role's name as the book prints it.
    pub fn name(self) -> &'static str {
        match self {
            CprRole::Rockerboy => "Rockerboy",
            CprRole::Solo => "Solo",
            CprRole::Netrunner => "Netrunner",
            CprRole::Tech => "Tech",
            CprRole::Medtech => "Medtech",
            CprRole::Media => "Media",
            CprRole::Exec => "Exec",
            CprRole::Lawman => "Lawman",
            CprRole::Fixer => "Fixer",
            CprRole::Nomad => "Nomad",
        }
    }

    /// The id segment of this Role's `cpr.role.<id>.` rows.
    pub fn id(self) -> &'static str {
        match self {
            CprRole::Rockerboy => "rockerboy",
            CprRole::Solo => "solo",
            CprRole::Netrunner => "netrunner",
            CprRole::Tech => "tech",
            CprRole::Medtech => "medtech",
            CprRole::Media => "media",
            CprRole::Exec => "exec",
            CprRole::Lawman => "lawman",
            CprRole::Fixer => "fixer",
            CprRole::Nomad => "nomad",
        }
    }

    pub fn from_name(name: &str) -> Option<CprRole> {
        CprRole::ALL.into_iter().find(|role| role.name() == name)
    }
}

/// Why a rank-derived value was not computed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CprRoleRefusal {
    /// The rank lies outside the Role Ability's printed range.
    RankOutOfRange { role: CprRole, rank: u8, min: u8, max: u8 },
}

impl fmt::Display for CprRoleRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CprRoleRefusal::RankOutOfRange { role, rank, min, max } => write!(
                f,
                "{} Role Ability Rank {rank} is outside the printed range {min}-{max}",
                role.name()
            ),
        }
    }
}

/// One `role` corpus record, the fields the engine reads.
#[derive(Debug, Clone, Deserialize)]
struct RoleRecord {
    name: String,
    role_ability: String,
    starting_rank: u8,
    rank_min: u8,
    rank_max: u8,
}

/// One rank band of a `role_ability` table; each table carries its own value fields.
#[derive(Debug, Clone, Deserialize)]
struct RankBand {
    rank_min: u8,
    rank_max: u8,
    net_actions: Option<i16>,
    believability_out_of_10: Option<i16>,
    combat_number: Option<i16>,
    sp: Option<i16>,
    hp: Option<i16>,
    move_and_body: Option<i16>,
}

#[derive(Debug, Clone, Deserialize)]
struct TeamMemberThreshold {
    rank: u8,
    team_members_total: i16,
}

/// One Combat Awareness ability tier: its point cost (p. 147).
#[derive(Debug, Clone, Deserialize)]
struct AbilityTier {
    points: u8,
    text: String,
}

/// One Combat Awareness ability: tiered (fixed costs) or bought a point at a time.
#[derive(Debug, Clone, Deserialize)]
struct CombatAwarenessAbility {
    name: String,
    #[serde(default)]
    tiers: Vec<AbilityTier>,
    per_point: Option<String>,
}

/// One Maker or Medicine specialty.
#[derive(Debug, Clone, Deserialize)]
struct Specialty {
    name: String,
    max_points: Option<u8>,
    skill: Option<String>,
    skill_points_per_point: Option<i16>,
    skill_max: Option<i16>,
}

#[derive(Debug, Clone, Deserialize)]
struct NamedEffect {
    name: String,
    effect: String,
}

/// One Family Motorpool band: the stock vehicles of those ranks (p. 162).
#[derive(Debug, Clone, Deserialize)]
struct MotorpoolBand {
    rank_min: u8,
    vehicles: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct UpgradeRecord {
    name: String,
    rank_required: u8,
    applies_to: String,
    text: String,
}

#[derive(Debug, Clone, Deserialize)]
struct TeamJob {
    name: String,
    true_job: String,
    stat_rows: Vec<serde_json::Value>,
}

/// One `role_ability` corpus record, the fields the engine reads.
#[derive(Debug, Clone, Deserialize)]
struct RoleAbilityRecord {
    name: String,
    role: String,
    #[serde(default)]
    rank_bands: Vec<RankBand>,
    #[serde(default)]
    net_actions_per_turn: Vec<RankBand>,
    specialty_ranks_per_maker_rank: Option<i16>,
    points_per_rank: Option<i16>,
    #[serde(default)]
    team_member_thresholds: Vec<TeamMemberThreshold>,
    #[serde(default)]
    combat_awareness_abilities: Vec<CombatAwarenessAbility>,
    #[serde(default)]
    specialties: Vec<Specialty>,
    #[serde(default)]
    pharmaceuticals: Vec<NamedEffect>,
    #[serde(default)]
    motorpool: Vec<MotorpoolBand>,
    #[serde(default)]
    upgrades: Vec<UpgradeRecord>,
    #[serde(default)]
    team_jobs: Vec<TeamJob>,
}

/// The `vehicle` records' type, the one field the Motorpool reads.
#[derive(Debug, Clone, Deserialize)]
struct VehicleRecord {
    name: String,
    vehicle_type: String,
}

/// How often one upgrade may go on one vehicle, as its printed text says (p. 164-166; "Unless
/// stated otherwise, an option can only be taken once per vehicle").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UpgradeRepeat {
    Once,
    /// "If upgraded a second time" (Bulletproof Glass).
    Twice,
    /// "Multiple upgrades can be installed", "Each additional upgrade".
    Any,
    /// "can be taken multiple times ... except Bikes, Jetskis, and Gyrocopters": once on those.
    AnyExceptSmall,
    /// "Multiple upgrades can only be installed on Cabin Cruisers, Yachts, Aerozeps, and
    /// Groundcars that already have the Housing Capacity Upgrade installed": once elsewhere.
    AnyWithHousing,
}

/// Where an upgrade needs a Heavy Chassis on the same vehicle ("Requires Heavy Chassis").
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum HeavyChassisRule {
    All,
    Always,
    On(Vec<String>),
}

/// The vehicles an upgrade's printed "applies to" names.
#[derive(Debug, Clone, PartialEq, Eq)]
enum UpgradeFits {
    AllVehicles,
    AllExceptSmall,
    LandAndSea,
    Land,
    LandAndSeaExceptBikesJetskis,
    Bikes,
    Groundcars,
    Listed(Vec<String>),
}

#[derive(Debug, Clone)]
pub(crate) struct Upgrade {
    pub(crate) name: String,
    rank_required: u8,
    applies_to: String,
    fits: UpgradeFits,
    pub(crate) repeat: UpgradeRepeat,
    pub(crate) requires_heavy_chassis: HeavyChassisRule,
    text: String,
}

/// A Motorpool vehicle's class, for an upgrade's printed vehicle list.
#[derive(Debug, Clone)]
struct MotorpoolVehicle {
    name: String,
    rank_min: u8,
    vehicle_type: String,
}

impl MotorpoolVehicle {
    fn bike(&self) -> bool {
        self.name.ends_with("bike")
    }
    fn small(&self) -> bool {
        self.bike() || self.name == "Jetski" || self.name == "Gyrocopter"
    }
    fn groundcar(&self) -> bool {
        self.name.ends_with("Groundcar")
    }
    fn land(&self) -> bool {
        self.vehicle_type == "Land"
    }
    fn sea(&self) -> bool {
        self.vehicle_type == "Sea"
    }
}

fn upgrade_fits(applies_to: &str, motorpool: &[MotorpoolVehicle]) -> Result<UpgradeFits, String> {
    Ok(match applies_to {
        "All Vehicles" => UpgradeFits::AllVehicles,
        "All Vehicles Except Bikes, Jetskis, Gyrocopters" => UpgradeFits::AllExceptSmall,
        "All Land and Sea Vehicles" => UpgradeFits::LandAndSea,
        "All Land Vehicles" => UpgradeFits::Land,
        "All Land and Sea Vehicles Except Bikes and Jetskies" => UpgradeFits::LandAndSeaExceptBikesJetskis,
        "All Bikes" => UpgradeFits::Bikes,
        "All Groundcards [sic: Groundcars]" => UpgradeFits::Groundcars,
        list => {
            let items: Vec<String> = list.split(',').map(|s| s.trim().to_owned()).collect();
            for item in &items {
                if item != "Groundcars" && !motorpool.iter().any(|v| &v.name == item) {
                    return Err(format!("role_ability.json: an upgrade applies to '{list}', and '{item}' is no Motorpool vehicle"));
                }
            }
            UpgradeFits::Listed(items)
        }
    })
}

impl UpgradeFits {
    fn takes(&self, v: &MotorpoolVehicle) -> bool {
        match self {
            UpgradeFits::AllVehicles => true,
            UpgradeFits::AllExceptSmall => !v.small(),
            UpgradeFits::LandAndSea => v.land() || v.sea(),
            UpgradeFits::Land => v.land(),
            UpgradeFits::LandAndSeaExceptBikesJetskis => (v.land() || v.sea()) && !v.bike() && v.name != "Jetski",
            UpgradeFits::Bikes => v.bike(),
            UpgradeFits::Groundcars => v.groundcar(),
            UpgradeFits::Listed(items) => items.iter().any(|i| i == &v.name || (i == "Groundcars" && v.groundcar())),
        }
    }
}

fn upgrade_repeat(text: &str) -> UpgradeRepeat {
    if text.contains("If upgraded a second time") {
        UpgradeRepeat::Twice
    } else if text.contains("Multiple upgrades can only be installed on") && text.contains("Housing Capacity") {
        UpgradeRepeat::AnyWithHousing
    } else if text.contains("can be taken multiple times") && text.contains("except") {
        UpgradeRepeat::AnyExceptSmall
    } else if text.contains("Multiple upgrades") || text.contains("Each additional upgrade") {
        UpgradeRepeat::Any
    } else {
        UpgradeRepeat::Once
    }
}

fn heavy_chassis_rule(text: &str) -> HeavyChassisRule {
    const ON: &str = "Requires a Heavy Chassis on ";
    if let Some(at) = text.find(ON) {
        let rest = text[at + ON.len()..].trim_end_matches('.');
        HeavyChassisRule::On(rest.split(" and ").map(|s| s.trim().to_owned()).collect())
    } else if text.contains("Requires Heavy Chassis") {
        HeavyChassisRule::Always
    } else {
        HeavyChassisRule::All
    }
}

/// What a Role at a rank divides or picks for its Role Ability, for the creation form.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CprRoleChoiceMenu {
    pub ability: String,
    pub rank: u8,
    pub points: Option<CprPointsMenu>,
    pub picks: Option<CprPicksMenu>,
}

/// Points the rank gives to divide among the options.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CprPointsMenu {
    pub label: String,
    pub total: u8,
    /// Every point must be placed (Maker, Medicine); else points may stay unassigned.
    pub exact: bool,
    pub source_page: String,
    pub options: Vec<CprPointsOption>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CprPointsOption {
    pub name: String,
    pub max: u8,
    /// The values this option takes, 0 first.
    pub steps: Vec<u8>,
    pub text: String,
}

/// Picks the rank gives.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CprPicksMenu {
    pub label: String,
    /// The most picks the choices allow.
    pub count: u8,
    pub source_page: String,
    pub options: Vec<CprPickOption>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CprPickOption {
    pub name: String,
    /// `vehicle`, `upgrade`, `pharmaceutical` or `team_member`.
    pub kind: String,
    /// The pick names what it goes on (an upgrade's vehicle, a Team Member's chart row).
    pub needs_target: bool,
    /// The fixed targets (a Team Member's chart rows); empty when the targets are earlier picks.
    pub targets: Vec<String>,
    pub text: String,
}

/// The Lawman's Backup stat block at one rank (p. 158-159).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BackupBlock {
    combat_number: i16,
    sp: i16,
    hp: i16,
    move_and_body: i16,
}

#[derive(Debug, Clone)]
struct RoleEntry {
    role_ability: String,
    starting_rank: u8,
    rank_min: u8,
    rank_max: u8,
}

/// The Role and Role Ability records, with every banded table resolved to one value per rank.
#[derive(Debug, Clone)]
pub struct CprRoleCatalog {
    roles: BTreeMap<CprRole, RoleEntry>,
    net_actions: BTreeMap<u8, i16>,
    believability: BTreeMap<u8, i16>,
    backup: BTreeMap<u8, BackupBlock>,
    team_members: BTreeMap<u8, i16>,
    maker_specialty_ranks_per_rank: i16,
    medicine_points_per_rank: i16,
    combat_awareness: Vec<CombatAwarenessAbility>,
    maker_specialties: Vec<Specialty>,
    medicine_specialties: Vec<Specialty>,
    pharmaceuticals: Vec<NamedEffect>,
    motorpool: Vec<MotorpoolVehicle>,
    pub(crate) upgrades: Vec<Upgrade>,
    team_jobs: Vec<TeamJob>,
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}

/// Resolve a banded table to one value per rank in `min..=max`; every rank must fall in
/// exactly one band and every band must carry the value.
fn per_rank<T: Copy>(
    table: &str,
    bands: &[RankBand],
    min: u8,
    max: u8,
    value: impl Fn(&RankBand) -> Option<T>,
) -> Result<BTreeMap<u8, T>, String> {
    let mut out = BTreeMap::new();
    for rank in min..=max {
        let mut matching = bands.iter().filter(|b| b.rank_min <= rank && rank <= b.rank_max);
        let band = matching.next().ok_or_else(|| format!("role_ability.json: {table} has no band for rank {rank}"))?;
        if matching.next().is_some() {
            return Err(format!("role_ability.json: {table} has two bands for rank {rank}"));
        }
        let v = value(band).ok_or_else(|| format!("role_ability.json: {table} band for rank {rank} has no value"))?;
        out.insert(rank, v);
    }
    Ok(out)
}

impl CprRoleCatalog {
    /// Load `role.json` and `role_ability.json` from `corpus_dir`
    /// (`GameSystem::CyberpunkRed.package_roots(..).corpus`).
    pub fn load(corpus_dir: &Path) -> Result<Self, String> {
        let role_records: Vec<RoleRecord> = read_json(&corpus_dir.join("role.json"))?;
        let mut roles = BTreeMap::new();
        for record in role_records {
            let role = CprRole::from_name(&record.name).ok_or_else(|| format!("role.json: unknown Role {}", record.name))?;
            if record.rank_min > record.starting_rank || record.starting_rank > record.rank_max {
                return Err(format!("role.json: {} starting rank {} lies outside its range", record.name, record.starting_rank));
            }
            let entry = RoleEntry {
                role_ability: record.role_ability,
                starting_rank: record.starting_rank,
                rank_min: record.rank_min,
                rank_max: record.rank_max,
            };
            if roles.insert(role, entry).is_some() {
                return Err(format!("role.json: {} is listed twice", record.name));
            }
        }
        for role in CprRole::ALL {
            if !roles.contains_key(&role) {
                return Err(format!("role.json: no {} record", role.name()));
            }
        }

        let abilities: Vec<RoleAbilityRecord> = read_json(&corpus_dir.join("role_ability.json"))?;
        let mut by_role = BTreeMap::new();
        for ability in abilities {
            let role = CprRole::from_name(&ability.role)
                .ok_or_else(|| format!("role_ability.json: {} names unknown Role {}", ability.name, ability.role))?;
            if roles[&role].role_ability != ability.name {
                return Err(format!(
                    "role_ability.json: {} is not the {} Role Ability ({} in role.json)",
                    ability.name, ability.role, roles[&role].role_ability
                ));
            }
            if by_role.insert(role, ability).is_some() {
                return Err(format!("role_ability.json: {} has two Role Abilities", role.name()));
            }
        }
        let ability = |role: CprRole| -> Result<&RoleAbilityRecord, String> {
            by_role.get(&role).ok_or_else(|| format!("role_ability.json: no {} Role Ability", role.name()))
        };
        let range = |role: CprRole| (roles[&role].rank_min, roles[&role].rank_max);

        let (min, max) = range(CprRole::Netrunner);
        let net_actions = per_rank("Interface NET Actions", &ability(CprRole::Netrunner)?.net_actions_per_turn, min, max, |b| b.net_actions)?;
        let (min, max) = range(CprRole::Media);
        let believability = per_rank("Credibility ranks", &ability(CprRole::Media)?.rank_bands, min, max, |b| b.believability_out_of_10)?;
        let (min, max) = range(CprRole::Lawman);
        let backup = per_rank("Backup ranks", &ability(CprRole::Lawman)?.rank_bands, min, max, |b| {
            Some(BackupBlock { combat_number: b.combat_number?, sp: b.sp?, hp: b.hp?, move_and_body: b.move_and_body? })
        })?;

        let thresholds = &ability(CprRole::Exec)?.team_member_thresholds;
        if thresholds.is_empty() {
            return Err("role_ability.json: Teamwork has no team_member_thresholds".to_string());
        }
        let (min, max) = range(CprRole::Exec);
        let team_members = (min..=max)
            .map(|rank| {
                let total = thresholds.iter().filter(|t| t.rank <= rank).map(|t| t.team_members_total).max().unwrap_or(0);
                (rank, total)
            })
            .collect();

        let maker_specialty_ranks_per_rank = ability(CprRole::Tech)?
            .specialty_ranks_per_maker_rank
            .ok_or("role_ability.json: Maker has no specialty_ranks_per_maker_rank")?;
        let medicine_points_per_rank =
            ability(CprRole::Medtech)?.points_per_rank.ok_or("role_ability.json: Medicine has no points_per_rank")?;

        let combat_awareness = ability(CprRole::Solo)?.combat_awareness_abilities.clone();
        if combat_awareness.is_empty() || combat_awareness.iter().any(|a| a.tiers.is_empty() == a.per_point.is_none()) {
            return Err("role_ability.json: every Combat Awareness ability is tiered or bought per point".to_string());
        }
        let maker_specialties = ability(CprRole::Tech)?.specialties.clone();
        let medicine_specialties = ability(CprRole::Medtech)?.specialties.clone();
        if maker_specialties.is_empty() || medicine_specialties.is_empty() {
            return Err("role_ability.json: Maker and Medicine list their specialties".to_string());
        }
        let pharmaceuticals = ability(CprRole::Medtech)?.pharmaceuticals.clone();
        if pharmaceuticals.is_empty() {
            return Err("role_ability.json: Medicine lists no pharmaceuticals".to_string());
        }

        let vehicles: Vec<VehicleRecord> = read_json(&corpus_dir.join("vehicle.json"))?;
        let mut motorpool = Vec::new();
        for band in &ability(CprRole::Nomad)?.motorpool {
            for name in &band.vehicles {
                // The Motorpool table shortens two names ("AV-4" for "AV-4 Multipurpose Aerodyne").
                let record = vehicles
                    .iter()
                    .find(|v| &v.name == name || v.name.starts_with(&format!("{name} ")))
                    .ok_or_else(|| format!("role_ability.json: Motorpool vehicle {name} has no vehicle record"))?;
                motorpool.push(MotorpoolVehicle { name: name.clone(), rank_min: band.rank_min, vehicle_type: record.vehicle_type.clone() });
            }
        }
        let upgrades = ability(CprRole::Nomad)?
            .upgrades
            .iter()
            .map(|u| {
                Ok(Upgrade {
                    name: u.name.clone(),
                    rank_required: u.rank_required,
                    applies_to: u.applies_to.clone(),
                    fits: upgrade_fits(&u.applies_to, &motorpool)?,
                    repeat: upgrade_repeat(&u.text),
                    requires_heavy_chassis: heavy_chassis_rule(&u.text),
                    text: u.text.clone(),
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let team_jobs = ability(CprRole::Exec)?.team_jobs.clone();
        if motorpool.is_empty() || upgrades.is_empty() || team_jobs.is_empty() {
            return Err("role_ability.json: Moto lists its Motorpool and upgrades, Teamwork its jobs".to_string());
        }

        Ok(CprRoleCatalog {
            roles,
            net_actions,
            believability,
            backup,
            team_members,
            maker_specialty_ranks_per_rank,
            medicine_points_per_rank,
            combat_awareness,
            maker_specialties,
            medicine_specialties,
            pharmaceuticals,
            motorpool,
            upgrades,
            team_jobs,
        })
    }

    pub fn role_ability(&self, role: CprRole) -> &str {
        &self.roles[&role].role_ability
    }

    pub fn starting_rank(&self, role: CprRole) -> u8 {
        self.roles[&role].starting_rank
    }

    /// Every `cpr.role.<role>.` row that `role`'s Role Ability at `rank` puts on the sheet.
    pub fn rank_rows(&self, role: CprRole, rank: u8) -> Result<Vec<ComputationExplanation>, CprRoleRefusal> {
        let entry = &self.roles[&role];
        if rank < entry.rank_min || rank > entry.rank_max {
            return Err(CprRoleRefusal::RankOutOfRange { role, rank, min: entry.rank_min, max: entry.rank_max });
        }
        let ability = entry.role_ability.as_str();
        let r = i16::from(rank);
        let row = |field: &str, value: i16, detail: String| ComputationExplanation {
            id: format!("cpr.role.{}.{field}", role.id()),
            value,
            detail,
        };
        let mut rows = vec![row(
            "rank",
            r,
            format!(
                "{ability} Rank {rank}: the {}'s Role Ability Rank (1-10; every Role starts at Rank 4, p. 142-143)",
                role.name()
            ),
        )];
        match role {
            CprRole::Rockerboy => rows.push(row(
                "charismatic_impact_check",
                r,
                format!("Charismatic Impact check {r} + 1d10: Rank {rank} + 1d10 vs DV8 / DV10 / DV12 (p. 144)"),
            )),
            CprRole::Solo => rows.push(row(
                "combat_awareness_points",
                r,
                format!("Combat Awareness points {r}: the pool equals Rank {rank}, divided among the six abilities (p. 146)"),
            )),
            CprRole::Netrunner => {
                let actions = self.net_actions[&rank];
                rows.push(row(
                    "net_actions",
                    actions,
                    format!("NET Actions per Turn {actions}: Interface Rank {rank} on the NET Actions table (p. 147)"),
                ))
            }
            CprRole::Tech => {
                let per = self.maker_specialty_ranks_per_rank;
                rows.push(row(
                    "maker_specialty_ranks",
                    per * r,
                    format!(
                        "Maker specialty ranks {}: {per} specialty ranks for each of Maker Rank {rank}'s ranks, divided among two different specialties per rank (p. 147)",
                        per * r
                    ),
                ))
            }
            CprRole::Medtech => {
                let per = self.medicine_points_per_rank;
                rows.push(row(
                    "medicine_specialty_points",
                    per * r,
                    format!(
                        "Medicine specialty points {}: {per} point for each of Medicine Rank {rank}'s ranks, allocated among Surgery, Pharmaceuticals and Cryosystem Operation (p. 149)",
                        per * r
                    ),
                ))
            }
            CprRole::Media => {
                let chance = self.believability[&rank];
                rows.push(row(
                    "credibility_rumor_check",
                    r,
                    format!("Credibility Rumor roll {r} + 1d10: Rank {rank} + 1d10 vs Passive DV 7 / 9 / 11 / 13 (p. 151)"),
                ));
                rows.push(row(
                    "believability",
                    chance,
                    format!("Believability {chance} out of 10: Credibility Rank {rank} on the Credibility ranks table (p. 152-153)"),
                ));
            }
            CprRole::Exec => {
                let members = self.team_members[&rank];
                rows.push(row(
                    "team_members",
                    members,
                    format!("Team Members {members}: Teamwork Rank {rank}; one at Rank 3, another at Ranks 5 and 9, at most 3 (p. 154)"),
                ))
            }
            CprRole::Lawman => {
                let block = self.backup[&rank];
                rows.push(row(
                    "backup_call_threshold",
                    r,
                    format!("Backup call {r}: roll 1d10 equal to or lower than Backup Rank {rank} (p. 158)"),
                ));
                let table = format!("Backup Rank {rank} on the Backup ranks table (p. 158-159)");
                rows.push(row("backup_combat_number", block.combat_number, format!("Backup Combat Number {}: {table}", block.combat_number)));
                rows.push(row("backup_sp", block.sp, format!("Backup SP {}: {table}", block.sp)));
                rows.push(row("backup_hp", block.hp, format!("Backup HP {}: {table}", block.hp)));
                rows.push(row(
                    "backup_move_and_body",
                    block.move_and_body,
                    format!("Backup MOVE and BODY {}: {table}", block.move_and_body),
                ));
            }
            CprRole::Fixer => rows.push(row(
                "haggle_bonus",
                r,
                format!("Haggle bonus +{r}: COOL + Trading + Operator Rank {rank} + 1d10 in a Haggle contest (p. 159)"),
            )),
            CprRole::Nomad => {
                rows.push(row(
                    "vehicle_familiarity_bonus",
                    r,
                    format!(
                        "Vehicle Familiarity +{r}: Moto Rank {rank} added to Drive Land Vehicle, Pilot Air Vehicle, Pilot Sea Vehicle, Air Vehicle Tech, Land Vehicle Tech and Sea Vehicle Tech Checks (p. 161)"
                    ),
                ));
                rows.push(row(
                    "motorpool_picks",
                    r,
                    format!(
                        "Family Motorpool picks {r}: one vehicle or upgrade of Moto Rank or lower for each of Moto Rank {rank}'s ranks (p. 161; A-2)"
                    ),
                ));
            }
        }
        Ok(rows)
    }

    fn check_rank(&self, role: CprRole, rank: u8) -> Result<(), String> {
        let entry = &self.roles[&role];
        if rank < entry.rank_min || rank > entry.rank_max {
            return Err(CprRoleRefusal::RankOutOfRange { role, rank, min: entry.rank_min, max: entry.rank_max }.to_string());
        }
        Ok(())
    }

    /// The Medicine points the choices place on Pharmaceuticals: one drug pick each.
    fn pharmaceutical_points(&self, choices: &CprRoleChoices) -> u8 {
        let pharma = self.medicine_specialties.iter().find(|s| s.name.contains("Pharmaceuticals"));
        pharma.and_then(|s| choices.points.iter().find(|(n, _)| n == &s.name)).map_or(0, |(_, p)| *p)
    }

    /// What `role`'s Role Ability at `rank` divides or picks (none for five of the ten Roles).
    pub fn choice_menu(&self, role: CprRole, rank: u8, choices: &CprRoleChoices) -> Result<CprRoleChoiceMenu, String> {
        self.check_rank(role, rank)?;
        let upto = |max: u8| (0..=max).collect::<Vec<u8>>();
        let (points, picks) = match role {
            CprRole::Solo => {
                let options = self
                    .combat_awareness
                    .iter()
                    .map(|a| {
                        let steps = if a.tiers.is_empty() {
                            upto(rank)
                        } else {
                            std::iter::once(0).chain(a.tiers.iter().map(|t| t.points).filter(|p| *p <= rank)).collect()
                        };
                        let text = match &a.per_point {
                            Some(per) => format!("each point: {per}"),
                            None => a.tiers.iter().map(|t| format!("{} points: {}", t.points, t.text)).collect::<Vec<_>>().join("; "),
                        };
                        CprPointsOption { name: a.name.clone(), max: *steps.last().unwrap_or(&0), steps, text }
                    })
                    .collect();
                let menu = CprPointsMenu {
                    label: "Combat Awareness".to_owned(),
                    total: rank,
                    exact: false,
                    source_page: "p147".to_owned(),
                    options,
                };
                (Some(menu), None)
            }
            CprRole::Tech => {
                let per = u8::try_from(self.maker_specialty_ranks_per_rank).unwrap_or(0);
                let options = self
                    .maker_specialties
                    .iter()
                    .map(|s| CprPointsOption { name: s.name.clone(), max: rank, steps: upto(rank), text: String::new() })
                    .collect();
                let menu = CprPointsMenu {
                    label: "Maker specialty ranks".to_owned(),
                    total: per.saturating_mul(rank),
                    exact: true,
                    source_page: "p148".to_owned(),
                    options,
                };
                (Some(menu), None)
            }
            CprRole::Medtech => {
                let per = u8::try_from(self.medicine_points_per_rank).unwrap_or(0);
                let total = per.saturating_mul(rank);
                let options = self
                    .medicine_specialties
                    .iter()
                    .map(|s| {
                        let max = s.max_points.map_or(total, |m| m.min(total));
                        let text = match (&s.skill, s.skill_points_per_point, s.skill_max) {
                            (Some(skill), Some(per), Some(cap)) => format!("{per} {skill} skill a point, at most {cap}"),
                            _ => String::new(),
                        };
                        CprPointsOption { name: s.name.clone(), max, steps: upto(max), text }
                    })
                    .collect();
                let points =
                    CprPointsMenu { label: "Medicine points".to_owned(), total, exact: true, source_page: "p150".to_owned(), options };
                let options = self
                    .pharmaceuticals
                    .iter()
                    .map(|p| CprPickOption {
                        name: p.name.clone(),
                        kind: "pharmaceutical".to_owned(),
                        needs_target: false,
                        targets: Vec::new(),
                        text: p.effect.clone(),
                    })
                    .collect();
                let picks = CprPicksMenu {
                    label: "Pharmaceuticals".to_owned(),
                    count: self.pharmaceutical_points(choices),
                    source_page: "p150".to_owned(),
                    options,
                };
                (Some(points), Some(picks))
            }
            CprRole::Nomad => {
                let vehicles = self.motorpool.iter().filter(|v| v.rank_min <= rank).map(|v| CprPickOption {
                    name: v.name.clone(),
                    kind: "vehicle".to_owned(),
                    needs_target: false,
                    targets: Vec::new(),
                    text: format!("stock {} vehicle, Motorpool Rank {}", v.vehicle_type, v.rank_min),
                });
                let upgrades = self.upgrades.iter().filter(|u| u.rank_required <= rank).map(|u| CprPickOption {
                    name: u.name.clone(),
                    kind: "upgrade".to_owned(),
                    needs_target: true,
                    targets: Vec::new(),
                    text: format!("{}: {}", u.applies_to, u.text),
                });
                let picks = CprPicksMenu {
                    label: "Family Motorpool".to_owned(),
                    count: rank,
                    source_page: "p162".to_owned(),
                    options: vehicles.chain(upgrades).collect(),
                };
                (None, Some(picks))
            }
            CprRole::Exec => {
                let options = self
                    .team_jobs
                    .iter()
                    .map(|j| CprPickOption {
                        name: j.name.clone(),
                        kind: "team_member".to_owned(),
                        needs_target: true,
                        targets: (1..=j.stat_rows.len()).map(|r| r.to_string()).collect(),
                        text: j.true_job.clone(),
                    })
                    .collect();
                let picks = CprPicksMenu {
                    label: "Team Members".to_owned(),
                    count: u8::try_from(self.team_members[&rank]).unwrap_or(0),
                    source_page: "p154".to_owned(),
                    options,
                };
                (None, Some(picks))
            }
            CprRole::Rockerboy | CprRole::Netrunner | CprRole::Media | CprRole::Lawman | CprRole::Fixer => (None, None),
        };
        Ok(CprRoleChoiceMenu { ability: self.role_ability(role).to_owned(), rank, points, picks })
    }

    /// Checks `role`'s Role Ability sub-choices at `rank` against the book and answers with
    /// their `cpr.role.<role>.` rows; every rule the choices break is refused with its reason.
    pub fn choice_rows(&self, role: CprRole, rank: u8, choices: &CprRoleChoices) -> Result<Vec<ComputationExplanation>, Vec<String>> {
        let menu = self.choice_menu(role, rank, choices).map_err(|e| vec![e])?;
        let ability = menu.ability.as_str();
        let mut refusals = Vec::new();
        let mut placed: BTreeMap<String, u8> = BTreeMap::new();

        match &menu.points {
            None if !choices.points.is_empty() => {
                refusals.push(format!("the {} Role Ability {ability} has no points to divide", role.name()))
            }
            None => {}
            Some(pm) => {
                let mut sum: u32 = 0;
                for (name, p) in &choices.points {
                    if placed.contains_key(name) {
                        refusals.push(format!("{ability}: {name} is given points twice"));
                        continue;
                    }
                    let Some(option) = pm.options.iter().find(|o| &o.name == name) else {
                        let known: Vec<&str> = pm.options.iter().map(|o| o.name.as_str()).collect();
                        refusals.push(format!("{ability} has no '{name}' (it has {})", known.join(", ")));
                        continue;
                    };
                    if *p > option.max {
                        refusals.push(format!("{ability}: {name} {p} is above {} at Rank {rank} ({})", option.max, pm.source_page));
                    } else if !option.steps.contains(p) {
                        let shown: Vec<String> = option.steps.iter().map(u8::to_string).collect();
                        let (last, rest) = shown.split_last().expect("steps hold 0");
                        refusals.push(format!(
                            "{ability}: {name} takes {} or {last} points, not {p} ({})",
                            rest.join(", "),
                            pm.source_page
                        ));
                    }
                    sum += u32::from(*p);
                    placed.insert(name.clone(), *p);
                }
                let total = u32::from(pm.total);
                if sum > total || (pm.exact && sum != total) {
                    let rule = if pm.exact { "place all" } else { "at most" };
                    refusals.push(format!(
                        "{}: {sum} of {total} points placed; {rule} {total} at {ability} Rank {rank} ({})",
                        pm.label, pm.source_page
                    ));
                }
            }
        }

        let picks_made = u8::try_from(choices.picks.len()).unwrap_or(u8::MAX);
        match &menu.picks {
            None if !choices.picks.is_empty() => refusals.push(format!("the {} Role Ability {ability} has no picks", role.name())),
            None => {}
            Some(pk) => {
                if picks_made > pk.count {
                    refusals.push(if role == CprRole::Medtech {
                        format!(
                            "Pharmaceuticals: {picks_made} drugs picked; one per Pharmaceuticals point allows {} (p. 150)",
                            pk.count
                        )
                    } else {
                        format!("{}: {picks_made} of {} picks at {ability} Rank {rank} ({})", pk.label, pk.count, pk.source_page)
                    });
                }
                match role {
                    CprRole::Medtech => self.check_pharmaceuticals(&choices.picks, &mut refusals),
                    CprRole::Nomad => self.check_motorpool(rank, &choices.picks, &mut refusals),
                    CprRole::Exec => self.check_team(&choices.picks, &mut refusals),
                    _ => {}
                }
            }
        }
        if !refusals.is_empty() {
            return Err(refusals);
        }
        Ok(self.sub_choice_rows(role, rank, &menu, &placed, picks_made))
    }

    fn check_pharmaceuticals(&self, picks: &[CprRolePick], refusals: &mut Vec<String>) {
        let mut seen = Vec::new();
        for pick in picks {
            if !self.pharmaceuticals.iter().any(|p| p.name == pick.name) {
                let known: Vec<&str> = self.pharmaceuticals.iter().map(|p| p.name.as_str()).collect();
                refusals.push(format!("Pharmaceuticals has no '{}' (it has {}; p. 150)", pick.name, known.join(", ")));
            } else if seen.contains(&pick.name) {
                refusals.push(format!("Pharmaceuticals: {} is picked twice; each point gives a new drug (p. 150)", pick.name));
            }
            if pick.on.is_some() {
                refusals.push(format!("Pharmaceuticals: {} goes on nothing", pick.name));
            }
            seen.push(pick.name.clone());
        }
    }

    fn check_motorpool(&self, rank: u8, picks: &[CprRolePick], refusals: &mut Vec<String>) {
        let mut pool: Vec<&MotorpoolVehicle> = Vec::new();
        for pick in picks {
            let Some(v) = self.motorpool.iter().find(|v| v.name == pick.name) else { continue };
            if pick.on.is_some() {
                refusals.push(format!("Family Motorpool: the stock vehicle {} goes on nothing", v.name));
            } else if v.rank_min > rank {
                refusals.push(format!(
                    "Family Motorpool: {} is a Rank {} vehicle; Moto Rank {rank} adds vehicles of Rank {rank} or lower (p. 162)",
                    v.name, v.rank_min
                ));
            } else if pool.iter().any(|p| p.name == v.name) {
                refusals.push(format!("Family Motorpool: {} is already in the pool", v.name));
            } else {
                pool.push(v);
            }
        }
        // (vehicle, upgrade) -> installs
        let mut installs: BTreeMap<(String, String), u32> = BTreeMap::new();
        let mut checked: Vec<(&Upgrade, &MotorpoolVehicle)> = Vec::new();
        for pick in picks.iter().filter(|p| !self.motorpool.iter().any(|v| v.name == p.name)) {
            let Some(u) = self.upgrades.iter().find(|u| u.name == pick.name) else {
                refusals.push(format!("Family Motorpool has no vehicle or upgrade '{}' (p. 162-166)", pick.name));
                continue;
            };
            if u.rank_required > rank {
                refusals.push(format!(
                    "Family Motorpool: {} is a Rank {} upgrade; Moto Rank {rank} installs upgrades of Rank {rank} or lower (p. 164)",
                    u.name, u.rank_required
                ));
                continue;
            }
            let Some(on) = pick.on.as_deref() else {
                refusals.push(format!("Family Motorpool: name the Family Vehicle {} goes on", u.name));
                continue;
            };
            let Some(v) = pool.iter().find(|v| v.name == on) else {
                refusals.push(format!("Family Motorpool: {} goes on {on}, and {on} is not in the pool (pick the vehicle too)", u.name));
                continue;
            };
            if !u.fits.takes(v) {
                refusals.push(format!("Family Motorpool: {} fits {}, not a {on} (p. 164-166)", u.name, u.applies_to));
                continue;
            }
            *installs.entry((on.to_owned(), u.name.clone())).or_default() += 1;
            checked.push((u, v));
        }
        let has = |vehicle: &str, upgrade: &str| installs.contains_key(&(vehicle.to_owned(), upgrade.to_owned()));
        let mut reported = Vec::new();
        for (u, v) in checked {
            if reported.contains(&(u.name.clone(), v.name.clone())) {
                continue;
            }
            reported.push((u.name.clone(), v.name.clone()));
            let housing_host = matches!(v.name.as_str(), "Cabin Cruiser" | "Yacht" | "Aerozep") || v.groundcar();
            let limit = match u.repeat {
                UpgradeRepeat::Once => Some(1),
                UpgradeRepeat::Twice => Some(2),
                UpgradeRepeat::Any => None,
                UpgradeRepeat::AnyExceptSmall if v.small() => Some(1),
                UpgradeRepeat::AnyExceptSmall => None,
                UpgradeRepeat::AnyWithHousing if housing_host && has(&v.name, "Housing Capacity") => None,
                UpgradeRepeat::AnyWithHousing => Some(1),
            };
            let count = installs[&(v.name.clone(), u.name.clone())];
            if let Some(limit) = limit.filter(|l| count > *l) {
                let times = if limit == 1 { "once".to_owned() } else { "at most twice".to_owned() };
                refusals.push(format!("Family Motorpool: {} goes on the {} {times}, not {count} times (p. 164)", u.name, v.name));
            }
            let needs_chassis = match &u.requires_heavy_chassis {
                HeavyChassisRule::All => false,
                HeavyChassisRule::Always => true,
                HeavyChassisRule::On(list) => list.contains(&v.name),
            };
            if needs_chassis && !has(&v.name, "Heavy Chassis") {
                refusals.push(format!("Family Motorpool: {} on the {} requires Heavy Chassis on it (p. 165-166)", u.name, v.name));
            }
        }
    }

    fn check_team(&self, picks: &[CprRolePick], refusals: &mut Vec<String>) {
        for pick in picks {
            let Some(job) = self.team_jobs.iter().find(|j| j.name == pick.name) else {
                let known: Vec<&str> = self.team_jobs.iter().map(|j| j.name.as_str()).collect();
                refusals.push(format!("Teamwork has no Team Member job '{}' (it has {}; p. 156)", pick.name, known.join(", ")));
                continue;
            };
            let rows = job.stat_rows.len();
            match pick.on.as_deref().map(|r| r.parse::<usize>()) {
                None => refusals.push(format!("Teamwork: pick the row of the {} chart (1d6) the Team Member is hired at", job.name)),
                Some(Ok(row)) if (1..=rows).contains(&row) => {}
                Some(_) => refusals.push(format!(
                    "Teamwork: the {} chart rows are 1 to {rows}, not {} (p. 156)",
                    job.name,
                    pick.on.as_deref().unwrap_or_default()
                )),
            }
        }
    }

    fn sub_choice_rows(
        &self,
        role: CprRole,
        rank: u8,
        menu: &CprRoleChoiceMenu,
        placed: &BTreeMap<String, u8>,
        picks_made: u8,
    ) -> Vec<ComputationExplanation> {
        let row = |field: String, value: i16, detail: String| ComputationExplanation {
            id: format!("cpr.role.{}.{field}", role.id()),
            value,
            detail,
        };
        let points_of = |name: &str| i16::from(placed.get(name).copied().unwrap_or(0));
        let mut rows = Vec::new();
        match role {
            CprRole::Solo => {
                for a in &self.combat_awareness {
                    let p = points_of(&a.name);
                    let effect = match (&a.per_point, a.tiers.iter().find(|t| i16::from(t.points) == p)) {
                        (Some(per), _) => format!("{p} x ({per})"),
                        (None, Some(tier)) => tier.text.clone(),
                        (None, None) => "not active".to_owned(),
                    };
                    rows.push(row(
                        format!("combat_awareness.{}", slug(&a.name)),
                        p,
                        format!("{} {p}: {effect}; points from Combat Awareness Rank {rank} (p. 147)", a.name),
                    ));
                }
                let used: i16 = placed.values().map(|p| i16::from(*p)).sum();
                let left = i16::from(rank) - used;
                rows.push(row(
                    "combat_awareness_unassigned".to_owned(),
                    left,
                    format!("Combat Awareness points unassigned {left}: Rank {rank} points less {used} placed (p. 147)"),
                ));
            }
            CprRole::Tech => {
                let total = menu.points.as_ref().map_or(0, |p| p.total);
                for s in &self.maker_specialties {
                    let p = points_of(&s.name);
                    rows.push(row(
                        format!("maker.{}", slug(&s.name)),
                        p,
                        format!("{} Rank {p}: of the {total} specialty ranks Maker Rank {rank} gives (p. 148; D-7)", s.name),
                    ));
                }
            }
            CprRole::Medtech => {
                let mut skills: BTreeMap<String, (i16, i16)> = BTreeMap::new();
                for s in &self.medicine_specialties {
                    let p = points_of(&s.name);
                    rows.push(row(
                        format!("medicine.{}", slug(&s.name)),
                        p,
                        format!("{} {p}: of the {rank} points Medicine Rank {rank} gives (p. 150-151; D-7)", s.name),
                    ));
                    if let (Some(skill), Some(per), Some(cap)) = (&s.skill, s.skill_points_per_point, s.skill_max) {
                        let entry = skills.entry(skill.clone()).or_insert((0, cap));
                        entry.0 += p * per;
                    }
                }
                for (skill, (raw, cap)) in skills {
                    let level = raw.min(cap);
                    rows.push(row(
                        format!("{}_skill", slug(&skill)),
                        level,
                        format!("{skill} skill {level}: from the Medicine points at Rank {rank}, at most {cap} (p. 150-151)"),
                    ));
                }
                let count = menu.picks.as_ref().map_or(0, |p| p.count);
                let left = i16::from(count) - i16::from(picks_made);
                rows.push(row(
                    "pharmaceutical_picks_left".to_owned(),
                    left,
                    format!("Pharmaceuticals left to pick {left}: one per Pharmaceuticals point, Medicine Rank {rank} (p. 150)"),
                ));
            }
            CprRole::Nomad => {
                let left = i16::from(rank) - i16::from(picks_made);
                rows.push(row(
                    "motorpool_picks_left".to_owned(),
                    left,
                    format!("Family Motorpool picks left {left}: one per Moto Rank, Rank {rank} (p. 162; A-2)"),
                ));
            }
            CprRole::Exec => {
                let count = menu.picks.as_ref().map_or(0, |p| p.count);
                let left = i16::from(count) - i16::from(picks_made);
                rows.push(row(
                    "team_member_picks_left".to_owned(),
                    left,
                    format!("Team Members left to pick {left}: {count} at Teamwork Rank {rank} (p. 154; A-2)"),
                ));
            }
            CprRole::Rockerboy | CprRole::Netrunner | CprRole::Media | CprRole::Lawman | CprRole::Fixer => {}
        }
        rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    fn corpus_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/cyberpunk-red/corpus")
    }

    fn catalog() -> CprRoleCatalog {
        CprRoleCatalog::load(&corpus_dir()).expect("the CPR role corpus loads")
    }

    #[derive(serde::Deserialize)]
    struct FixtureRole {
        role: String,
        role_ability: String,
        starting_rank: u8,
        values: BTreeMap<String, Vec<i16>>,
    }

    #[derive(serde::Deserialize)]
    struct Fixture {
        rank_min: u8,
        rank_max: u8,
        roles: Vec<FixtureRole>,
    }

    fn fixture() -> Fixture {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cpr/role_rank_values.json");
        serde_json::from_str(&std::fs::read_to_string(path).expect("fixture reads")).expect("fixture parses")
    }

    #[test]
    fn every_rank_of_every_role_equals_the_fixture() {
        let catalog = catalog();
        let fixture = fixture();
        assert_eq!(fixture.roles.len(), 10);
        assert_eq!(CprRole::ALL.len(), 10);
        let mut compared = 0;
        for expected in &fixture.roles {
            let role = CprRole::from_name(&expected.role).expect("fixture names a Role");
            assert_eq!(catalog.role_ability(role), expected.role_ability);
            assert_eq!(catalog.starting_rank(role), expected.starting_rank);
            for rank in fixture.rank_min..=fixture.rank_max {
                let rows = catalog.rank_rows(role, rank).expect("rank 1-10 computes");
                let got: BTreeMap<String, i16> = rows.iter().map(|r| (r.id.clone(), r.value)).collect();
                assert_eq!(got.len(), rows.len(), "{} rank {rank}: duplicate row id", expected.role);
                let mut want = BTreeMap::new();
                want.insert(format!("cpr.role.{}.rank", role.id()), i16::from(rank));
                for (field, by_rank) in &expected.values {
                    assert_eq!(by_rank.len(), 10, "{} {field}: 10 ranks", expected.role);
                    want.insert(format!("cpr.role.{}.{field}", role.id()), by_rank[usize::from(rank - 1)]);
                }
                assert_eq!(got, want, "{} rank {rank}", expected.role);
                compared += 1;
            }
        }
        assert_eq!(compared, 100, "10 Roles x 10 ranks");
    }

    #[test]
    fn lawman_backup_combat_number_at_ranks_8_9_10_is_16_15_14_as_printed() {
        // D-8: keep the printed 16/15/14 (p160), even though it falls with rank.
        let catalog = catalog();
        let cn: Vec<i16> = (8..=10)
            .map(|rank| {
                catalog
                    .rank_rows(CprRole::Lawman, rank)
                    .unwrap()
                    .into_iter()
                    .find(|r| r.id == "cpr.role.lawman.backup_combat_number")
                    .expect("backup combat number row")
                    .value
            })
            .collect();
        assert_eq!(cn, [16, 15, 14]);
    }

    #[test]
    fn a_rank_outside_1_to_10_is_refused() {
        let catalog = catalog();
        for role in CprRole::ALL {
            assert_eq!(
                catalog.rank_rows(role, 0).unwrap_err(),
                CprRoleRefusal::RankOutOfRange { role, rank: 0, min: 1, max: 10 }
            );
            assert_eq!(
                catalog.rank_rows(role, 11).unwrap_err(),
                CprRoleRefusal::RankOutOfRange { role, rank: 11, min: 1, max: 10 }
            );
        }
    }

    fn choices(points: &[(&str, u8)], picks: &[(&str, Option<&str>)]) -> CprRoleChoices {
        CprRoleChoices {
            points: points.iter().map(|(n, p)| ((*n).to_owned(), *p)).collect(),
            picks: picks.iter().map(|(n, on)| CprRolePick { name: (*n).to_owned(), on: on.map(str::to_owned) }).collect(),
        }
    }

    fn rows_of(catalog: &CprRoleCatalog, role: CprRole, c: &CprRoleChoices) -> BTreeMap<String, i16> {
        catalog
            .choice_rows(role, 4, c)
            .unwrap_or_else(|e| panic!("{} at Rank 4 accepts {c:?}: {e:?}", role.name()))
            .into_iter()
            .map(|r| (r.id, r.value))
            .collect()
    }

    fn refusal(catalog: &CprRoleCatalog, role: CprRole, c: &CprRoleChoices) -> String {
        catalog.choice_rows(role, 4, c).expect_err("refused").join(" / ")
    }

    /// p. 147: the Solo divides Combat Awareness Rank (4) points among six abilities; a tiered
    /// ability takes only its printed costs.
    #[test]
    fn solo_combat_awareness_divides_4_points_at_the_printed_costs() {
        let catalog = catalog();
        let menu = catalog.choice_menu(CprRole::Solo, 4, &CprRoleChoices::default()).expect("menu");
        let points = menu.points.expect("Combat Awareness is points");
        assert_eq!(points.total, 4);
        assert!(!points.exact, "unassigned points persist (p. 147)");
        let names: Vec<&str> = points.options.iter().map(|o| o.name.as_str()).collect();
        assert_eq!(
            names,
            ["Damage Deflection", "Fumble Recovery", "Initiative Reaction", "Precision Attack", "Spot Weakness", "Threat Detection"]
        );
        let steps = |name: &str| points.options.iter().find(|o| o.name == name).unwrap().steps.clone();
        assert_eq!(steps("Damage Deflection"), [0, 2, 4]);
        assert_eq!(steps("Fumble Recovery"), [0, 4]);
        assert_eq!(steps("Precision Attack"), [0, 3]);
        assert_eq!(steps("Threat Detection"), [0, 1, 2, 3, 4]);

        let rows = rows_of(&catalog, CprRole::Solo, &choices(&[("Precision Attack", 3), ("Threat Detection", 1)], &[]));
        assert_eq!(rows["cpr.role.solo.combat_awareness.precision_attack"], 3);
        assert_eq!(rows["cpr.role.solo.combat_awareness.threat_detection"], 1);
        assert_eq!(rows["cpr.role.solo.combat_awareness.damage_deflection"], 0);
        assert_eq!(rows["cpr.role.solo.combat_awareness_unassigned"], 0);
        let partial = rows_of(&catalog, CprRole::Solo, &choices(&[("Damage Deflection", 2)], &[]));
        assert_eq!(partial["cpr.role.solo.combat_awareness_unassigned"], 2);

        assert!(refusal(&catalog, CprRole::Solo, &choices(&[("Precision Attack", 3), ("Spot Weakness", 2)], &[])).contains("5 of 4"));
        assert!(refusal(&catalog, CprRole::Solo, &choices(&[("Damage Deflection", 3)], &[])).contains("0, 2 or 4"));
        assert!(refusal(&catalog, CprRole::Solo, &choices(&[("Bullet Time", 1)], &[])).contains("Bullet Time"));
        assert!(refusal(&catalog, CprRole::Solo, &choices(&[], &[("Roadbike", None)])).contains("no picks"));
    }

    /// p. 148 + D-7: each Maker Rank gives 1 rank in two different specialties; at Rank 4 the
    /// player places 8 ranks, none above 4.
    #[test]
    fn tech_maker_places_8_specialty_ranks_none_above_4() {
        let catalog = catalog();
        let menu = catalog.choice_menu(CprRole::Tech, 4, &CprRoleChoices::default()).expect("menu");
        let points = menu.points.expect("Maker is points");
        assert_eq!((points.total, points.exact), (8, true));
        assert_eq!(points.options.len(), 4);
        assert!(points.options.iter().all(|o| o.max == 4));

        let rows = rows_of(
            &catalog,
            CprRole::Tech,
            &choices(&[("Field Expertise", 4), ("Upgrade Expertise", 2), ("Fabrication Expertise", 2)], &[]),
        );
        assert_eq!(rows["cpr.role.tech.maker.field_expertise"], 4);
        assert_eq!(rows["cpr.role.tech.maker.upgrade_expertise"], 2);
        assert_eq!(rows["cpr.role.tech.maker.fabrication_expertise"], 2);
        assert_eq!(rows["cpr.role.tech.maker.invention_expertise"], 0);

        assert!(refusal(&catalog, CprRole::Tech, &choices(&[("Field Expertise", 5), ("Upgrade Expertise", 3)], &[])).contains("above 4"));
        assert!(refusal(&catalog, CprRole::Tech, &choices(&[("Field Expertise", 4), ("Upgrade Expertise", 2)], &[])).contains("6 of 8"));
        assert!(refusal(&catalog, CprRole::Tech, &CprRoleChoices::default()).contains("0 of 8"));
    }

    /// p. 150-151 + D-7: one Medicine point per rank (4) among Surgery, Pharmaceuticals and
    /// Cryosystem Operation; Surgery gives 2 Surgery skill a point, the two Medical Tech
    /// specialties 1 Medical Tech skill a point; a Pharmaceuticals point picks one drug.
    #[test]
    fn medtech_medicine_places_4_points_and_picks_a_drug_per_pharmaceuticals_point() {
        let catalog = catalog();
        let c = choices(
            &[("Surgery", 2), ("Medical Tech (Pharmaceuticals)", 1), ("Medical Tech (Cryosystem Operation)", 1)],
            &[("Speedheal", None)],
        );
        let menu = catalog.choice_menu(CprRole::Medtech, 4, &c).expect("menu");
        assert_eq!(menu.points.as_ref().map(|p| (p.total, p.exact)), Some((4, true)));
        assert_eq!(menu.picks.as_ref().map(|p| p.count), Some(1), "one drug per Pharmaceuticals point");
        assert_eq!(menu.picks.as_ref().unwrap().options.len(), 5);

        let rows = rows_of(&catalog, CprRole::Medtech, &c);
        assert_eq!(rows["cpr.role.medtech.medicine.surgery"], 2);
        assert_eq!(rows["cpr.role.medtech.medicine.medical_tech_pharmaceuticals"], 1);
        assert_eq!(rows["cpr.role.medtech.medicine.medical_tech_cryosystem_operation"], 1);
        assert_eq!(rows["cpr.role.medtech.surgery_skill"], 4);
        assert_eq!(rows["cpr.role.medtech.medical_tech_skill"], 2);
        assert_eq!(rows["cpr.role.medtech.pharmaceutical_picks_left"], 0);

        let over = choices(&[("Surgery", 2), ("Medical Tech (Pharmaceuticals)", 2)], &[("Speedheal", None), ("Stim", None), ("Surge", None)]);
        assert!(refusal(&catalog, CprRole::Medtech, &over).contains("3 drugs"));
        assert!(refusal(&catalog, CprRole::Medtech, &choices(&[("Surgery", 3)], &[])).contains("3 of 4"));
        assert!(refusal(&catalog, CprRole::Medtech, &choices(&[("Surgery", 4)], &[("Stim", None)])).contains("Pharmaceuticals"));
        let twice = choices(&[("Medical Tech (Pharmaceuticals)", 4)], &[("Stim", None), ("Stim", None)]);
        assert!(refusal(&catalog, CprRole::Medtech, &twice).contains("twice"));
    }

    /// p. 162-166 + A-2: one Family Motorpool pick per Moto Rank (4): a stock vehicle of Rank 4
    /// or lower, or an upgrade of Rank 4 or lower on a vehicle already in the pool.
    #[test]
    fn nomad_motorpool_takes_4_picks_of_vehicles_and_upgrades_of_rank_4_or_lower() {
        let catalog = catalog();
        let menu = catalog.choice_menu(CprRole::Nomad, 4, &CprRoleChoices::default()).expect("menu");
        let picks = menu.picks.expect("Moto is picks");
        assert_eq!(picks.count, 4);
        let vehicles: Vec<&str> = picks.options.iter().filter(|o| o.kind == "vehicle").map(|o| o.name.as_str()).collect();
        assert_eq!(vehicles, ["Compact Groundcar", "Gyrocopter", "Jetski", "Roadbike"]);
        assert!(picks.options.iter().filter(|o| o.kind == "upgrade").all(|o| o.needs_target));
        assert!(!picks.options.iter().any(|o| o.name == "Armored Chassis"), "Rank 5 upgrades are not offered at Rank 4");

        let ok = choices(
            &[],
            &[("Compact Groundcar", None), ("Heavy Chassis", Some("Compact Groundcar")), ("Housing Capacity", Some("Compact Groundcar")), ("Roadbike", None)],
        );
        let rows = rows_of(&catalog, CprRole::Nomad, &ok);
        assert_eq!(rows["cpr.role.nomad.motorpool_picks_left"], 0);
        let rows = rows_of(&catalog, CprRole::Nomad, &choices(&[], &[("Roadbike", None), ("NOS", Some("Roadbike")), ("NOS", Some("Roadbike"))]));
        assert_eq!(rows["cpr.role.nomad.motorpool_picks_left"], 1, "NOS: each additional upgrade adds a tank");

        let five = choices(&[], &[("Roadbike", None), ("Jetski", None), ("Gyrocopter", None), ("Compact Groundcar", None), ("NOS", Some("Jetski"))]);
        assert!(refusal(&catalog, CprRole::Nomad, &five).contains("5 of 4"));
        assert!(refusal(&catalog, CprRole::Nomad, &choices(&[], &[("Superbike", None)])).contains("Superbike"));
        assert!(refusal(&catalog, CprRole::Nomad, &choices(&[], &[("Roadbike", None), ("Armored Chassis", Some("Roadbike"))])).contains("Rank 5"));
        assert!(refusal(&catalog, CprRole::Nomad, &choices(&[], &[("NOS", Some("Roadbike"))])).contains("not in the pool"));
        assert!(refusal(&catalog, CprRole::Nomad, &choices(&[], &[("Roadbike", None), ("Heavy Chassis", Some("Roadbike"))])).contains("Bikes"));
        assert!(refusal(&catalog, CprRole::Nomad, &choices(&[], &[("Compact Groundcar", None), ("Housing Capacity", Some("Compact Groundcar"))])).contains("Heavy Chassis"));
        let twice = choices(&[], &[("Roadbike", None), ("Communications Center", Some("Roadbike")), ("Communications Center", Some("Roadbike"))]);
        assert!(refusal(&catalog, CprRole::Nomad, &twice).contains("once"));
        assert!(refusal(&catalog, CprRole::Nomad, &choices(&[], &[("Roadbike", None), ("Roadbike", None)])).contains("already"));
    }

    /// The repeat and prerequisite rules each upgrade's printed text states (p. 164-166).
    #[test]
    fn every_motorpool_upgrade_has_its_printed_repeat_and_prerequisite_rule() {
        let catalog = catalog();
        let rules: BTreeMap<String, String> =
            catalog.upgrades.iter().map(|u| (u.name.clone(), format!("{:?} {:?}", u.repeat, u.requires_heavy_chassis))).collect();
        assert_eq!(rules.len(), 19);
        assert_eq!(rules["Bulletproof Glass"], "Twice All");
        assert_eq!(rules["NOS"], "Any All");
        assert_eq!(rules["Seating Upgrade"], "AnyExceptSmall All");
        assert_eq!(rules["Smuggling Upgrade"], "AnyExceptSmall All");
        assert_eq!(rules["Vehicle Heavy Weapon Mount"], "AnyWithHousing Always");
        assert_eq!(rules["Onboard Rocket Pod"], "Any Always");
        assert_eq!(rules["Housing Capacity"], "Once On([\"Compact Groundcar\", \"High Performance Groundcar\"])");
        assert_eq!(rules["Heavy Chassis"], "Once All");
        let once = rules.values().filter(|r| r.starts_with("Once")).count();
        let any = rules.values().filter(|r| r.starts_with("Any ")).count();
        assert_eq!((once, any), (9, 6), "{rules:?}");
    }

    /// p. 154 + A-2: Teamwork Rank 4 gives 1 Team Member; the Exec picks its job and its row on
    /// the job's chart.
    #[test]
    fn exec_picks_one_team_member_job_and_its_chart_row() {
        let catalog = catalog();
        let menu = catalog.choice_menu(CprRole::Exec, 4, &CprRoleChoices::default()).expect("menu");
        let picks = menu.picks.expect("Teamwork is picks");
        assert_eq!(picks.count, 1);
        assert_eq!(picks.options.len(), 5);
        assert!(picks.options.iter().all(|o| o.needs_target && o.targets == ["1", "2", "3", "4", "5", "6"]));

        let rows = rows_of(&catalog, CprRole::Exec, &choices(&[], &[("Company Bodyguard", Some("3"))]));
        assert_eq!(rows["cpr.role.exec.team_member_picks_left"], 0);
        let two = choices(&[], &[("Company Bodyguard", Some("3")), ("Company Driver", Some("1"))]);
        assert!(refusal(&catalog, CprRole::Exec, &two).contains("2 of 1"));
        assert!(refusal(&catalog, CprRole::Exec, &choices(&[], &[("Company Bodyguard", Some("7"))])).contains("1 to 6"));
        assert!(refusal(&catalog, CprRole::Exec, &choices(&[], &[("Company Bodyguard", None)])).contains("row"));
        assert!(refusal(&catalog, CprRole::Exec, &choices(&[], &[("Company Chef", Some("1"))])).contains("Company Chef"));
    }

    /// Rockerboy, Netrunner, Media, Lawman and Fixer make no Role Ability choice at creation.
    #[test]
    fn roles_without_sub_choices_take_none() {
        let catalog = catalog();
        for role in [CprRole::Rockerboy, CprRole::Netrunner, CprRole::Media, CprRole::Lawman, CprRole::Fixer] {
            let menu = catalog.choice_menu(role, 4, &CprRoleChoices::default()).expect("menu");
            assert!(menu.points.is_none() && menu.picks.is_none(), "{}", role.name());
            assert!(catalog.choice_rows(role, 4, &CprRoleChoices::default()).expect("none is valid").is_empty());
            assert!(refusal(&catalog, role, &choices(&[("Surgery", 1)], &[])).contains("no points"), "{}", role.name());
        }
    }

    #[test]
    fn every_row_detail_cites_the_rank_and_a_page() {
        let catalog = catalog();
        for role in CprRole::ALL {
            for row in catalog.rank_rows(role, 4).unwrap() {
                assert!(row.id.starts_with("cpr.role."), "{}", row.id);
                assert!(row.detail.contains("Rank 4"), "{}: {}", row.id, row.detail);
                assert!(row.detail.contains("p. "), "{}: {}", row.id, row.detail);
            }
        }
    }
}
