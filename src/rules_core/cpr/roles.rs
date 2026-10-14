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

use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

use serde::Deserialize;

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

        Ok(CprRoleCatalog {
            roles,
            net_actions,
            believability,
            backup,
            team_members,
            maker_specialty_ranks_per_rank,
            medicine_points_per_rank,
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
