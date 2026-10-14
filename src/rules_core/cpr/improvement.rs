//! Improvement: the Improvement Point price of raising a Skill or a Role Ability, and the
//! Role switch (multiclassing) rule (Cyberpunk RED core rulebook p. 143 and p. 411).
//!
//! - Three I.P. Cost tables (p. 411): Typical Skill 20 × the next Level, Difficult (x2) Skill
//!   40 ×, Role Ability Rank 60 × the next Rank, Level/Rank 1 to 10. Every price is read from
//!   the `ip_cost` corpus records; the loader refuses a table that is missing, short, out of
//!   order or that disagrees with its own per-level rate.
//! - "Cost is based on the next Level/Rank; you can't skip Levels/Ranks" (p. 411): a purchase
//!   is one step up; a skipped Level, a non-increase or a Level past 10 is refused.
//! - Multiclassing (p. 143): a character may change Roles once the latest Role is at least
//!   Rank 4, by buying Rank 1 in the new Role with Improvement Points; the new Role starts at
//!   Rank 1 and switching again is locked until it reaches Rank 4. Earlier Roles keep their
//!   Rank and may still be raised. The book prints no price for that Rank 1; A-1 takes the
//!   Role Ability Rank table's Rank 1 price ([`NEW_ROLE_PRICE_TABLE`]).
//!
//! Which table a Skill uses (its "(x2)" mark) is the Skill's own data, read by the caller.
//! How many IP a character has, and the GM's awards (p. 410-411), are not computed here.

use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

use serde::Deserialize;

use crate::rules_core::cpr::roles::CprRole;
use crate::rules_core::pilot_compute::ComputationExplanation;

/// The highest Level of a Skill and Rank of a Role Ability (p. 411: "Level/Rank 1 to 10").
pub const MAX_LEVEL: u8 = 10;

/// The Rank the latest Role must reach before the character may change Roles (p. 143).
pub const SWITCH_RANK: u8 = 4;

/// A-1: the table whose Rank 1 price buys Rank 1 in a new Role. p. 143 says only "buying
/// Rank 1 in the new Role with Improvement Points"; the Role Ability Rank table prices Rank 1
/// at 60 IP.
pub const NEW_ROLE_PRICE_TABLE: CprIpTable = CprIpTable::RoleAbilityRank;

/// The three I.P. Cost tables of p. 411.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CprIpTable {
    TypicalSkill,
    DifficultSkill,
    RoleAbilityRank,
}

impl CprIpTable {
    pub const ALL: [CprIpTable; 3] = [CprIpTable::TypicalSkill, CprIpTable::DifficultSkill, CprIpTable::RoleAbilityRank];

    /// The table's `table` key in `ip_cost.json`, and the id segment of its rows.
    pub fn id(self) -> &'static str {
        match self {
            CprIpTable::TypicalSkill => "typical_skill",
            CprIpTable::DifficultSkill => "difficult_skill",
            CprIpTable::RoleAbilityRank => "role_ability_rank",
        }
    }

    /// The table's heading as the book prints it.
    pub fn name(self) -> &'static str {
        match self {
            CprIpTable::TypicalSkill => "Typical Skill Improvement",
            CprIpTable::DifficultSkill => "Difficult (x2) Skill Improvement",
            CprIpTable::RoleAbilityRank => "Role Ability Rank Improvement",
        }
    }

    fn unit(self) -> &'static str {
        match self {
            CprIpTable::RoleAbilityRank => "Rank",
            CprIpTable::TypicalSkill | CprIpTable::DifficultSkill => "Level",
        }
    }

    fn from_id(id: &str) -> Option<CprIpTable> {
        CprIpTable::ALL.into_iter().find(|t| t.id() == id)
    }
}

/// Why an improvement was not priced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CprImprovementRefusal {
    /// The purchase would skip a Level/Rank (p. 411: "you can't skip Levels/Ranks").
    SkippedLevel { table: CprIpTable, from: u8, to: u8 },
    /// The target is not above the current Level/Rank.
    NotAnIncrease { table: CprIpTable, from: u8, to: u8 },
    /// The target lies outside Level/Rank 1 to 10.
    AboveMaximum { table: CprIpTable, to: u8, max: u8 },
    /// The latest Role is below Rank 4, so the character may not change Roles (p. 143).
    SwitchLocked { current: CprRole, rank: u8, needed: u8 },
    /// The character already has this Role.
    RoleAlreadyHeld { role: CprRole },
    /// The character does not have this Role, so its Rank cannot be raised.
    RoleNotHeld { role: CprRole },
}

impl fmt::Display for CprImprovementRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CprImprovementRefusal::SkippedLevel { table, from, to } => write!(
                f,
                "{}: {unit} {from} to {to} skips a {unit}; buy {unit} {} first (p. 411: you can't skip Levels/Ranks)",
                table.name(),
                from + 1,
                unit = table.unit()
            ),
            CprImprovementRefusal::NotAnIncrease { table, from, to } => {
                write!(f, "{}: {unit} {to} is not above the current {unit} {from}", table.name(), unit = table.unit())
            }
            CprImprovementRefusal::AboveMaximum { table, to, max } => {
                write!(f, "{}: {unit} {to} is outside {unit} 1 to {max} (p. 411)", table.name(), unit = table.unit())
            }
            CprImprovementRefusal::SwitchLocked { current, rank, needed } => write!(
                f,
                "cannot change Roles: the latest Role, {}, is at Rank {rank}; it must reach Rank {needed} first (p. 143)",
                current.name()
            ),
            CprImprovementRefusal::RoleAlreadyHeld { role } => write!(f, "the character already has the {} Role", role.name()),
            CprImprovementRefusal::RoleNotHeld { role } => write!(f, "the character does not have the {} Role", role.name()),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct CostCell {
    level: u8,
    ip_cost: i16,
}

/// One `ip_cost` corpus record, the fields the engine reads.
#[derive(Debug, Clone, Deserialize)]
struct IpCostRecord {
    table: String,
    per_level: i16,
    costs: Vec<CostCell>,
}

/// The three I.P. Cost tables, one price per Level/Rank 1 to 10.
#[derive(Debug, Clone)]
pub struct CprIpCostCatalog {
    costs: BTreeMap<CprIpTable, [i16; MAX_LEVEL as usize]>,
}

impl CprIpCostCatalog {
    /// Load `ip_cost.json` from `corpus_dir` (`GameSystem::CyberpunkRed.package_roots(..).corpus`).
    pub fn load(corpus_dir: &Path) -> Result<Self, String> {
        let path = corpus_dir.join("ip_cost.json");
        let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let records: Vec<IpCostRecord> = serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
        let mut costs = BTreeMap::new();
        for record in records {
            let table = CprIpTable::from_id(&record.table).ok_or_else(|| format!("ip_cost.json: unknown table {}", record.table))?;
            if record.costs.len() != usize::from(MAX_LEVEL) {
                return Err(format!("ip_cost.json: {} has {} levels, not {MAX_LEVEL}", record.table, record.costs.len()));
            }
            let mut prices = [0i16; MAX_LEVEL as usize];
            for (i, cell) in record.costs.iter().enumerate() {
                let level = i as u8 + 1;
                if cell.level != level {
                    return Err(format!("ip_cost.json: {} entry {} is level {}, not {level}", record.table, i + 1, cell.level));
                }
                if cell.ip_cost != record.per_level * i16::from(level) {
                    return Err(format!(
                        "ip_cost.json: {} level {level} costs {}, not {} x {level}",
                        record.table, cell.ip_cost, record.per_level
                    ));
                }
                prices[i] = cell.ip_cost;
            }
            if costs.insert(table, prices).is_some() {
                return Err(format!("ip_cost.json: {} is listed twice", record.table));
            }
        }
        for table in CprIpTable::ALL {
            if !costs.contains_key(&table) {
                return Err(format!("ip_cost.json: no {} table", table.id()));
            }
        }
        Ok(CprIpCostCatalog { costs })
    }

    /// The IP price of reaching `level` (1 to 10) on `table`.
    pub fn cost_to_reach(&self, table: CprIpTable, level: u8) -> Result<i16, CprImprovementRefusal> {
        if !(1..=MAX_LEVEL).contains(&level) {
            return Err(CprImprovementRefusal::AboveMaximum { table, to: level, max: MAX_LEVEL });
        }
        Ok(self.costs[&table][usize::from(level - 1)])
    }

    /// The `cpr.improvement.<table>.cost` row for raising a Skill or Role Ability from `from`
    /// to `to`. One step only: a skipped Level/Rank, a non-increase or a target past 10 is refused.
    pub fn improve(&self, table: CprIpTable, from: u8, to: u8) -> Result<ComputationExplanation, CprImprovementRefusal> {
        if to <= from {
            return Err(CprImprovementRefusal::NotAnIncrease { table, from, to });
        }
        if to > MAX_LEVEL {
            return Err(CprImprovementRefusal::AboveMaximum { table, to, max: MAX_LEVEL });
        }
        if to != from + 1 {
            return Err(CprImprovementRefusal::SkippedLevel { table, from, to });
        }
        let cost = self.cost_to_reach(table, to)?;
        let unit = table.unit();
        Ok(ComputationExplanation {
            id: format!("cpr.improvement.{}.cost", table.id()),
            value: cost,
            detail: format!(
                "{cost} IP: {} from {unit} {from} to {unit} {to}; the cost is based on the next {unit} (p. 411)",
                table.name()
            ),
        })
    }

    /// Raise a Role the character has by one Rank, priced on the Role Ability Rank table.
    pub fn raise_role_rank(
        &self,
        history: &CprRoleHistory,
        role: CprRole,
        to: u8,
    ) -> Result<(CprRoleHistory, ComputationExplanation), CprImprovementRefusal> {
        let from = history.rank(role).ok_or(CprImprovementRefusal::RoleNotHeld { role })?;
        let mut row = self.improve(CprIpTable::RoleAbilityRank, from, to)?;
        row.detail = format!("{} Role: {}", role.name(), row.detail);
        let mut next = history.clone();
        for entry in &mut next.roles {
            if entry.0 == role {
                entry.1 = to;
            }
        }
        Ok((next, row))
    }

    /// Change Roles: buy Rank 1 in `new_role` (A-1 price) once the latest Role is at Rank 4.
    /// The new Role becomes the latest; earlier Roles keep their Ranks.
    pub fn switch_role(
        &self,
        history: &CprRoleHistory,
        new_role: CprRole,
    ) -> Result<(CprRoleHistory, ComputationExplanation), CprImprovementRefusal> {
        if history.rank(new_role).is_some() {
            return Err(CprImprovementRefusal::RoleAlreadyHeld { role: new_role });
        }
        let (current, rank) = history.latest();
        if rank < SWITCH_RANK {
            return Err(CprImprovementRefusal::SwitchLocked { current, rank, needed: SWITCH_RANK });
        }
        let cost = self.cost_to_reach(NEW_ROLE_PRICE_TABLE, 1)?;
        let mut next = history.clone();
        next.roles.push((new_role, 1));
        let row = ComputationExplanation {
            id: "cpr.improvement.new_role.cost".to_string(),
            value: cost,
            detail: format!(
                "{cost} IP: Rank 1 in the {} Role, the {} Rank 1 price (A-1); {} is at Rank {rank}, at least {SWITCH_RANK}, so the character may change Roles, and switching again is locked until {} reaches Rank {SWITCH_RANK} (p. 143, p. 411)",
                new_role.name(),
                NEW_ROLE_PRICE_TABLE.name(),
                current.name(),
                new_role.name()
            ),
        };
        Ok((next, row))
    }
}

/// The Roles a character has, in the order taken, each with its Role Ability Rank. The first
/// is the starting Role; the last is the latest (the Role the Street sees, p. 143).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprRoleHistory {
    roles: Vec<(CprRole, u8)>,
}

impl CprRoleHistory {
    /// A character with one Role at `rank`.
    pub fn new(role: CprRole, rank: u8) -> Self {
        CprRoleHistory { roles: vec![(role, rank)] }
    }

    pub fn roles(&self) -> &[(CprRole, u8)] {
        &self.roles
    }

    pub fn rank(&self, role: CprRole) -> Option<u8> {
        self.roles.iter().find(|(r, _)| *r == role).map(|(_, rank)| *rank)
    }

    /// The latest Role.
    pub fn current_role(&self) -> CprRole {
        self.latest().0
    }

    fn latest(&self) -> (CprRole, u8) {
        *self.roles.last().expect("a CprRoleHistory always holds its starting Role")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::cpr::roles::CprRole;
    use std::path::PathBuf;

    fn catalog() -> CprIpCostCatalog {
        let corpus = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/cyberpunk-red/corpus");
        CprIpCostCatalog::load(&corpus).expect("the CPR ip_cost corpus loads")
    }

    /// The three I.P. Cost rows as printed on p. 411 (PDF page 412), Level/Rank 1 to 10.
    const PRINTED: [(CprIpTable, [i16; 10]); 3] = [
        (CprIpTable::TypicalSkill, [20, 40, 60, 80, 100, 120, 140, 160, 180, 200]),
        (CprIpTable::DifficultSkill, [40, 80, 120, 160, 200, 240, 280, 320, 360, 400]),
        (CprIpTable::RoleAbilityRank, [60, 120, 180, 240, 300, 360, 420, 480, 540, 600]),
    ];

    #[test]
    fn every_cell_of_the_3_by_10_ip_cost_table_equals_the_printed_page() {
        let catalog = catalog();
        let mut compared = 0;
        for (table, printed) in PRINTED {
            for (i, want) in printed.into_iter().enumerate() {
                let level = i as u8 + 1;
                assert_eq!(catalog.cost_to_reach(table, level), Ok(want), "{table:?} level {level}");
                let row = catalog.improve(table, level - 1, level).expect("one step up is priced");
                assert_eq!(row.value, want, "{table:?} {} -> {level}", level - 1);
                assert_eq!(row.id, format!("cpr.improvement.{}.cost", table.id()));
                assert!(row.detail.contains("p. 411"), "{}", row.detail);
                compared += 1;
            }
        }
        assert_eq!(compared, 30, "3 tables x 10 levels");
    }

    #[test]
    fn a_skipped_level_a_non_increase_and_a_level_past_10_are_refused() {
        let catalog = catalog();
        for (table, _) in PRINTED {
            assert_eq!(
                catalog.improve(table, 3, 5),
                Err(CprImprovementRefusal::SkippedLevel { table, from: 3, to: 5 })
            );
            assert_eq!(
                catalog.improve(table, 0, 2),
                Err(CprImprovementRefusal::SkippedLevel { table, from: 0, to: 2 })
            );
            assert_eq!(
                catalog.improve(table, 4, 4),
                Err(CprImprovementRefusal::NotAnIncrease { table, from: 4, to: 4 })
            );
            assert_eq!(
                catalog.improve(table, 5, 2),
                Err(CprImprovementRefusal::NotAnIncrease { table, from: 5, to: 2 })
            );
            assert_eq!(
                catalog.improve(table, 10, 11),
                Err(CprImprovementRefusal::AboveMaximum { table, to: 11, max: 10 })
            );
            assert_eq!(
                catalog.cost_to_reach(table, 0),
                Err(CprImprovementRefusal::AboveMaximum { table, to: 0, max: 10 })
            );
        }
    }

    #[test]
    fn a_role_switch_is_refused_at_rank_3_and_allowed_at_rank_4_at_the_a1_price() {
        let catalog = catalog();
        let at3 = CprRoleHistory::new(CprRole::Solo, 3);
        assert_eq!(
            catalog.switch_role(&at3, CprRole::Nomad),
            Err(CprImprovementRefusal::SwitchLocked { current: CprRole::Solo, rank: 3, needed: 4 })
        );

        let at4 = CprRoleHistory::new(CprRole::Solo, 4);
        let (after, row) = catalog.switch_role(&at4, CprRole::Nomad).expect("rank 4 may switch");
        assert_eq!(row.value, 60, "A-1: Rank 1 of the new Role costs the table's Rank 1 price");
        assert_eq!(row.id, "cpr.improvement.new_role.cost");
        assert!(row.detail.contains("p. 143"), "{}", row.detail);
        assert_eq!(after.current_role(), CprRole::Nomad);
        assert_eq!(after.rank(CprRole::Nomad), Some(1), "the new Role starts at Rank 1");
        assert_eq!(after.rank(CprRole::Solo), Some(4), "the previous Role keeps its Rank");
        assert_eq!(after.roles(), [(CprRole::Solo, 4), (CprRole::Nomad, 1)]);
    }

    #[test]
    fn after_a_switch_the_next_switch_is_locked_until_the_new_role_reaches_rank_4() {
        let catalog = catalog();
        let (mut history, _) = catalog.switch_role(&CprRoleHistory::new(CprRole::Solo, 4), CprRole::Nomad).unwrap();
        for (from, to, cost) in [(1u8, 2u8, 120i16), (2, 3, 180)] {
            assert_eq!(
                catalog.switch_role(&history, CprRole::Fixer),
                Err(CprImprovementRefusal::SwitchLocked { current: CprRole::Nomad, rank: from, needed: 4 })
            );
            let (next, row) = catalog.raise_role_rank(&history, CprRole::Nomad, to).unwrap();
            assert_eq!(row.value, cost, "Nomad {from} -> {to}");
            history = next;
        }
        assert_eq!(
            catalog.switch_role(&history, CprRole::Fixer),
            Err(CprImprovementRefusal::SwitchLocked { current: CprRole::Nomad, rank: 3, needed: 4 })
        );
        // Raising the previous Role does not unlock the switch: the latest Role must reach 4.
        let (history, row) = catalog.raise_role_rank(&history, CprRole::Solo, 5).unwrap();
        assert_eq!(row.value, 300);
        assert_eq!(
            catalog.switch_role(&history, CprRole::Fixer),
            Err(CprImprovementRefusal::SwitchLocked { current: CprRole::Nomad, rank: 3, needed: 4 })
        );
        let (history, _) = catalog.raise_role_rank(&history, CprRole::Nomad, 4).unwrap();
        let (history, row) = catalog.switch_role(&history, CprRole::Fixer).expect("the latest Role is at 4");
        assert_eq!(row.value, 60);
        assert_eq!(history.roles(), [(CprRole::Solo, 5), (CprRole::Nomad, 4), (CprRole::Fixer, 1)]);
        assert_eq!(history.current_role(), CprRole::Fixer);
    }

    #[test]
    fn a_held_role_cannot_be_bought_again_and_an_unheld_role_cannot_be_raised() {
        let catalog = catalog();
        let history = CprRoleHistory::new(CprRole::Solo, 4);
        assert_eq!(catalog.switch_role(&history, CprRole::Solo), Err(CprImprovementRefusal::RoleAlreadyHeld { role: CprRole::Solo }));
        assert_eq!(
            catalog.raise_role_rank(&history, CprRole::Media, 1),
            Err(CprImprovementRefusal::RoleNotHeld { role: CprRole::Media })
        );
        assert_eq!(
            catalog.raise_role_rank(&history, CprRole::Solo, 6),
            Err(CprImprovementRefusal::SkippedLevel { table: CprIpTable::RoleAbilityRank, from: 4, to: 6 })
        );
        let ten = CprRoleHistory::new(CprRole::Solo, 10);
        assert_eq!(
            catalog.raise_role_rank(&ten, CprRole::Solo, 11),
            Err(CprImprovementRefusal::AboveMaximum { table: CprIpTable::RoleAbilityRank, to: 11, max: 10 })
        );
    }
}
