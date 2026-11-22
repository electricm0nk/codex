//! Cyberpunk RED Skills (core rulebook p. 81-90): the 66 skills of the Master Skill List, each
//! skill's Skill Base (STAT + level, p. 81), and the creation skill spend of the three creation
//! methods (Streetrat p. 86-87, Edgerunner p. 88-89, Complete Package p. 90).
//!
//! Computed here: one `cpr.skill.<skill>` row per skill base, and the creation points spent and
//! left (`cpr.skills.creation_points_*`). Validated here: the 86-point creation budget, (x2)
//! skills at two points per level, the 13 Basic skills at level 2 or more, no skill above 6
//! (and, for Edgerunners, none below 2), and the 4 free levels of the Cultural Origin language.
//! Skill checks, Difficulty Values and modifiers are play-time rules the sheet prints.
//!
//! Every skill, its linked STAT, its (x2) flag and each Role's skill list come from the corpus
//! (`skill.json`, `role_skill_list.json`).

use std::path::Path;

use serde::Deserialize;

use crate::rules_core::cpr::input::{CprStat, CprStats};
use crate::rules_core::pilot_compute::ComputationExplanation;

/// p. 88 and p. 90: "Edgerunners get 86 Skill points"; Complete Package "86 Skill points".
pub const CREATION_SKILL_POINTS: u16 = 86;
/// p. 88 and p. 90: "No Skill can be higher than 6".
pub const CREATION_SKILL_CAP: u8 = 6;
/// p. 85: the Basic Skills "must start with at least a value of 2"; p. 88: no Edgerunner Skill
/// "lower than 2".
pub const CREATION_SKILL_FLOOR: u8 = 2;

const SOURCE_BASE: &str = "Cyberpunk RED core rulebook p. 81";
const CHOOSE_ONE_SPECIALTY: &str = "choose 1";

/// One `skill` corpus record, the fields the engine reads.
#[derive(Debug, Clone, Deserialize)]
pub struct CprSkill {
    pub name: String,
    pub category: String,
    #[serde(deserialize_with = "stat_from_label")]
    pub stat: CprStat,
    pub x2: bool,
    pub specialty_required: bool,
}

fn stat_from_label<'de, D: serde::Deserializer<'de>>(d: D) -> Result<CprStat, D::Error> {
    let label = String::deserialize(d)?;
    CprStat::ALL
        .into_iter()
        .find(|s| s.label() == label)
        .ok_or_else(|| serde::de::Error::custom(format!("unknown STAT '{label}'")))
}

/// One skill line of a Role's Streetrat list (p. 86-87).
#[derive(Debug, Clone, Deserialize)]
pub struct CprRoleSkill {
    pub name: String,
    pub level: u8,
    pub basic: bool,
}

/// One `role_skill_list` corpus record: a Role's 20 skills with their Streetrat levels; the
/// same 20 names are the Role's Edgerunner list (p. 88-89).
#[derive(Debug, Clone, Deserialize)]
pub struct CprRoleSkillList {
    pub role: String,
    pub skills: Vec<CprRoleSkill>,
    pub cultural_language_free_rank: u8,
}

/// The skills and Role skill lists of a CPR corpus directory.
#[derive(Debug, Clone)]
pub struct CprSkillCatalog {
    skills: Vec<CprSkill>,
    role_lists: Vec<CprRoleSkillList>,
    basic: Vec<String>,
    cultural_language_free: u8,
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("cannot parse {}: {e}", path.display()))
}

/// Splits `Name (Specialty)` into the name and the specialty.
fn split_key(key: &str) -> (&str, Option<&str>) {
    match key.trim().strip_suffix(')').and_then(|k| k.split_once(" (")) {
        Some((name, specialty)) => (name.trim(), Some(specialty.trim())),
        None => (key.trim(), None),
    }
}

/// The id fragment of a name: lower case, every run of other characters one `_`.
pub fn slug(name: &str) -> String {
    let mut out = String::new();
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
        } else if !out.is_empty() && !out.ends_with('_') {
            out.push('_');
        }
    }
    out.trim_end_matches('_').to_owned()
}

impl CprSkillCatalog {
    /// Load `skill.json` and `role_skill_list.json` from `corpus_dir`
    /// (`GameSystem::CyberpunkRed.package_roots(..).corpus`). The Basic skills are the lines every
    /// Role list marks `basic`; lists that disagree, or a list line naming no Master List skill,
    /// are refused.
    pub fn load(corpus_dir: &Path) -> Result<Self, String> {
        let skills: Vec<CprSkill> = read_json(&corpus_dir.join("skill.json"))?;
        let role_lists: Vec<CprRoleSkillList> = read_json(&corpus_dir.join("role_skill_list.json"))?;
        let mut catalog = CprSkillCatalog { skills, role_lists, basic: Vec::new(), cultural_language_free: 0 };
        let first = catalog.role_lists.first().ok_or("role_skill_list.json lists no Role")?;
        catalog.basic = first.skills.iter().filter(|s| s.basic).map(|s| s.name.clone()).collect();
        catalog.cultural_language_free = first.cultural_language_free_rank;
        for list in &catalog.role_lists {
            let basic: Vec<&String> = list.skills.iter().filter(|s| s.basic).map(|s| &s.name).collect();
            if basic != catalog.basic.iter().collect::<Vec<_>>() {
                return Err(format!("the {} skill list's Basic skills differ from the {} list's", list.role, first.role));
            }
            if list.cultural_language_free_rank != catalog.cultural_language_free {
                return Err(format!("the {} skill list gives a different free Cultural Origin language rank", list.role));
            }
            for line in &list.skills {
                catalog.resolve(&line.name).map_err(|e| format!("the {} skill list: {e}", list.role))?;
            }
        }
        Ok(catalog)
    }

    /// The 66 skills of the Master Skill List, in corpus order.
    pub fn skills(&self) -> &[CprSkill] {
        &self.skills
    }

    pub fn role_lists(&self) -> &[CprRoleSkillList] {
        &self.role_lists
    }

    /// The 13 Basic skills, as their list keys (`Language (Streetslang)`, ...).
    pub fn basic_skills(&self) -> &[String] {
        &self.basic
    }

    pub fn role_list(&self, role: &str) -> Result<&CprRoleSkillList, String> {
        self.role_lists
            .iter()
            .find(|l| l.role == role)
            .ok_or_else(|| format!("no Role skill list for '{role}'"))
    }

    /// Resolves a skill key (`Handgun`, `Language (Streetslang)`) to its Master List skill and
    /// its specialty. A skill that needs a specialty (Language, Local Expert, Science, Martial
    /// Arts, Play Instrument) must name one; any other skill must not.
    pub fn resolve(&self, key: &str) -> Result<(&CprSkill, Option<String>), String> {
        let exact = self.skills.iter().find(|s| s.name == key.trim());
        let (skill, specialty) = match exact {
            Some(skill) => (skill, None),
            None => {
                let (name, specialty) = split_key(key);
                let skill = self
                    .skills
                    .iter()
                    .find(|s| s.name == name)
                    .ok_or_else(|| format!("'{key}' is not a skill of the Master Skill List"))?;
                (skill, specialty.filter(|s| !s.is_empty()).map(str::to_owned))
            }
        };
        match (skill.specialty_required, &specialty) {
            (true, None) => Err(format!("'{key}': {} needs a specialty, as '{} (<specialty>)'", skill.name, skill.name)),
            (false, Some(_)) => Err(format!("'{key}': {} takes no specialty", skill.name)),
            _ => Ok((skill, specialty)),
        }
    }

    /// The Streetrat cost of a Role's list: level x 1, or x 2 for an (x2) skill (p. 81), with
    /// the (x2) flag read from the Master Skill List.
    pub fn role_list_cost(&self, role: &str) -> Result<u16, String> {
        let list = self.role_list(role)?;
        let mut total = 0;
        for line in &list.skills {
            let (skill, _) = self.resolve(&line.name)?;
            total += skill_cost(skill, line.level);
        }
        Ok(total)
    }
}

/// p. 81: "Any Skill marked this way costs twice the" points; a level costs 1 point, 2 for (x2).
pub fn skill_cost(skill: &CprSkill, level: u8) -> u16 {
    u16::from(level) * if skill.x2 { 2 } else { 1 }
}

/// How the character's skills were made (p. 86-90).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CprSkillMethod {
    /// The Role's list at its printed levels (p. 86-87).
    Streetrat { role: String },
    /// The Role's 20 skills, 86 points, each skill 2-6 (p. 88).
    Edgerunner { role: String },
    /// Any skills, 86 points, none above 6, the Basic skills at 2 or more (p. 90).
    CompletePackage,
}

/// A creation skill allocation: the method, the Cultural Origin language (a `Language (<name>)`
/// key, 4 free levels, p. 45) and every bought skill as (key, level).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprSkillAllocation {
    pub method: CprSkillMethod,
    pub cultural_language: Option<String>,
    pub levels: Vec<(String, u8)>,
}

/// One skill line of a validated spend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprSkillSpendLine {
    pub key: String,
    pub level: u8,
    pub cost: u16,
}

/// A validated creation skill spend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprSkillSpend {
    pub spent: u16,
    pub remaining: u16,
    pub levels: Vec<CprSkillSpendLine>,
}

/// Whether a bought key fills a Role list line (a `(choose 1)` line takes any specialty).
fn fills(catalog: &CprSkillCatalog, line: &str, key: &str) -> bool {
    if line == key {
        return true;
    }
    let (line_name, line_spec) = split_key(line);
    line_spec == Some(CHOOSE_ONE_SPECIALTY)
        && catalog.resolve(key).is_ok_and(|(skill, spec)| skill.name == line_name && spec.is_some())
}

/// Validates a creation allocation and prices it. Refusals name their rule: over 86 points, a
/// skill above 6, a Basic skill missing or below 2, an Edgerunner skill below 2 or off the Role's
/// list, a Streetrat level that differs from the printed list, an unknown or repeated skill.
pub fn validate_creation_skills(catalog: &CprSkillCatalog, alloc: &CprSkillAllocation) -> Result<CprSkillSpend, String> {
    let free = catalog.cultural_language_free;
    let cultural = match &alloc.cultural_language {
        Some(key) => {
            let (skill, specialty) = catalog.resolve(key)?;
            if skill.name != "Language" {
                return Err(format!("the Cultural Origin language '{key}' is not a Language"));
            }
            Some(format!("{} ({})", skill.name, specialty.unwrap_or_default()))
        }
        None => None,
    };

    let mut lines: Vec<CprSkillSpendLine> = Vec::new();
    for (key, level) in &alloc.levels {
        let (skill, specialty) = catalog.resolve(key)?;
        let canonical = match &specialty {
            Some(spec) => format!("{} ({spec})", skill.name),
            None => skill.name.clone(),
        };
        if lines.iter().any(|l| l.key == canonical) {
            return Err(format!("{canonical} is listed twice"));
        }
        if *level > CREATION_SKILL_CAP {
            return Err(format!("{canonical} at level {level} is above 6, the creation cap (p. 88, p. 90)"));
        }
        // The Cultural Origin language starts at its free levels; only levels above them cost.
        let cost = if cultural.as_deref() == Some(canonical.as_str()) {
            if *level < free {
                return Err(format!("{canonical} is the Cultural Origin language and starts at level {free}"));
            }
            skill_cost(skill, level - free)
        } else {
            skill_cost(skill, *level)
        };
        lines.push(CprSkillSpendLine { key: canonical, level: *level, cost });
    }
    if let Some(key) = &cultural
        && !lines.iter().any(|l| &l.key == key)
    {
        lines.push(CprSkillSpendLine { key: key.clone(), level: free, cost: 0 });
    }

    let is_cultural = |key: &str| cultural.as_deref() == Some(key);
    match &alloc.method {
        CprSkillMethod::Streetrat { role } | CprSkillMethod::Edgerunner { role } => {
            let list = catalog.role_list(role)?;
            for line in lines.iter().filter(|l| !is_cultural(&l.key)) {
                let Some(entry) = list.skills.iter().find(|e| fills(catalog, &e.name, &line.key)) else {
                    return Err(format!("{} is not on the {role} skill list (p. 86-89)", line.key));
                };
                if let CprSkillMethod::Streetrat { .. } = alloc.method {
                    if line.level != entry.level {
                        return Err(format!(
                            "{} is level {} on the {role} Streetrat list, not {} (p. 86-87)",
                            line.key, entry.level, line.level
                        ));
                    }
                } else if line.level < CREATION_SKILL_FLOOR {
                    return Err(format!("{} at level {} is lower than 2, the Edgerunner floor (p. 88)", line.key, line.level));
                }
            }
            for entry in &list.skills {
                if !lines.iter().any(|l| !is_cultural(&l.key) && fills(catalog, &entry.name, &l.key)) {
                    return Err(format!("the {role} skill list's {} is missing", entry.name));
                }
            }
        }
        CprSkillMethod::CompletePackage => {}
    }
    for basic in catalog.basic_skills() {
        match lines.iter().find(|l| &l.key == basic) {
            None => return Err(format!("the Basic skill {basic} is missing")),
            Some(l) if l.level < CREATION_SKILL_FLOOR => {
                return Err(format!("the Basic skill {basic} at level {} must be at least 2 (p. 85)", l.level));
            }
            Some(_) => {}
        }
    }

    let spent: u16 = lines.iter().map(|l| l.cost).sum();
    if spent > CREATION_SKILL_POINTS {
        return Err(format!("the skills spend {spent} of {CREATION_SKILL_POINTS} creation skill points"));
    }
    Ok(CprSkillSpend { spent, remaining: CREATION_SKILL_POINTS - spent, levels: lines })
}

/// The creation spend rows: points spent and points left of the 86.
pub fn spend_rows(spend: &CprSkillSpend) -> Vec<ComputationExplanation> {
    vec![
        ComputationExplanation {
            id: "cpr.skills.creation_points_spent".to_owned(),
            value: spend.spent as i16,
            detail: format!(
                "{} creation skill points spent: level x 1, x 2 for (x2) skills; the Cultural Origin language's 4 levels free (p. 88, p. 90)",
                spend.spent
            ),
        },
        ComputationExplanation {
            id: "cpr.skills.creation_points_remaining".to_owned(),
            value: spend.remaining as i16,
            detail: format!("{} of {CREATION_SKILL_POINTS} creation skill points left", spend.remaining),
        },
    ]
}

/// The Skill Base rows (p. 81: "STAT + Skill ... also known as a Skill Base"): one
/// `cpr.skill.<skill>` row for each of the 66 skills, at its linked STAT plus its level (0 when
/// untrained). A skill held with a specialty (`Language (Streetslang)`) is one
/// `cpr.skill.<skill>.<specialty>` row per specialty, in place of the bare row. `stats` are the
/// STATs the bases read (the sheet passes them after any armour penalty).
pub fn skill_rows(
    catalog: &CprSkillCatalog,
    stats: &CprStats,
    levels: &[(String, u8)],
) -> Result<Vec<ComputationExplanation>, String> {
    let mut held: Vec<(&CprSkill, Option<String>, u8)> = Vec::new();
    for (key, level) in levels {
        let (skill, specialty) = catalog.resolve(key)?;
        if held.iter().any(|(s, spec, _)| s.name == skill.name && *spec == specialty) {
            return Err(format!("{key} is listed twice"));
        }
        held.push((skill, specialty, *level));
    }
    let mut rows = Vec::new();
    for skill in catalog.skills() {
        let stat = stats.get(skill.stat);
        let label = skill.stat.label();
        let row = |id: String, name: String, level: u8| ComputationExplanation {
            id,
            value: i16::from(stat) + i16::from(level),
            detail: format!(
                "{name} base {} = {label} {stat} + level {level} ({SOURCE_BASE})",
                i16::from(stat) + i16::from(level)
            ),
        };
        let mine: Vec<&(&CprSkill, Option<String>, u8)> = held.iter().filter(|(s, _, _)| s.name == skill.name).collect();
        let bare = mine.iter().find(|(_, spec, _)| spec.is_none()).map(|(_, _, l)| *l);
        let with_specialty: Vec<(&String, u8)> =
            mine.iter().filter_map(|(_, spec, l)| spec.as_ref().map(|s| (s, *l))).collect();
        if with_specialty.is_empty() {
            rows.push(row(format!("cpr.skill.{}", slug(&skill.name)), skill.name.clone(), bare.unwrap_or(0)));
        }
        for (spec, level) in with_specialty {
            rows.push(row(
                format!("cpr.skill.{}.{}", slug(&skill.name), slug(spec)),
                format!("{} ({spec})", skill.name),
                level,
            ));
        }
    }
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::cpr::input::{CprStat, CprStats};
    use std::path::PathBuf;

    /// The Streetrat skill tables p087-p088 and the p088/p090/p091 worked examples, transcribed
    /// from the page and the inventory (not from the corpus the engine reads).
    const FIXTURE: &str = include_str!("../../../tests/fixtures/cpr/streetrat_skills_p087.json");

    fn fixture() -> serde_json::Value {
        serde_json::from_str(FIXTURE).unwrap()
    }

    fn catalog() -> CprSkillCatalog {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/cyberpunk-red/corpus");
        CprSkillCatalog::load(&dir).expect("the CPR skill corpus loads")
    }

    fn levels(rows: &serde_json::Value) -> Vec<(String, u8)> {
        rows.as_array()
            .unwrap()
            .iter()
            .map(|r| (r["name"].as_str().unwrap().to_owned(), r["level"].as_u64().unwrap() as u8))
            .collect()
    }

    fn example(key: &str) -> CprSkillAllocation {
        let fx = fixture();
        let ex = &fx["worked_examples"][key];
        let role = ex["role"].as_str().unwrap().to_owned();
        let method = match ex["method"].as_str().unwrap() {
            "streetrat" => CprSkillMethod::Streetrat { role },
            "edgerunner" => CprSkillMethod::Edgerunner { role },
            _ => CprSkillMethod::CompletePackage,
        };
        CprSkillAllocation {
            method,
            cultural_language: Some(ex["cultural_language"].as_str().unwrap().to_owned()),
            levels: levels(&ex["skills"]),
        }
    }

    /// One fixture Streetrat list as (key, level) with the `(choose 1)` placeholders given a
    /// specialty, since an allocation names a chosen instrument or field.
    fn fixture_list(role: &str) -> Vec<(String, u8)> {
        let fx = fixture();
        let list = fx["streetrat_lists"].as_array().unwrap().iter().find(|l| l["role"] == role).unwrap();
        let mut out = levels(&list["basic"]);
        out.extend(levels(&list["role_skills"]));
        for (name, _) in &mut out {
            *name = name.replace("(choose 1)", "(Chosen)");
        }
        out
    }

    #[test]
    fn the_catalog_holds_66_skills_7_of_them_x2_and_13_basic_skills() {
        let catalog = catalog();
        assert_eq!(catalog.skills().len(), 66);
        assert_eq!(catalog.skills().iter().filter(|s| s.x2).count(), 7);
        let basic: Vec<String> =
            fixture()["basic_skills"].as_array().unwrap().iter().map(|b| b.as_str().unwrap().to_owned()).collect();
        assert_eq!(catalog.basic_skills(), basic.as_slice());
        assert_eq!(catalog.role_lists().len(), 10);
    }

    #[test]
    fn each_of_the_10_role_skill_lists_costs_exactly_86() {
        let catalog = catalog();
        let fx = fixture();
        let mut equal = 0;
        for list in fx["streetrat_lists"].as_array().unwrap() {
            let role = list["role"].as_str().unwrap();
            // the corpus list the engine reads carries the page's levels
            let corpus: Vec<(String, u8)> = catalog
                .role_list(role)
                .unwrap()
                .skills
                .iter()
                .map(|s| (s.name.replace("(choose 1)", "(Chosen)"), s.level))
                .collect();
            assert_eq!(corpus, fixture_list(role), "{role} corpus list vs p087-p088");
            assert_eq!(catalog.role_list_cost(role).unwrap(), 86, "{role}");
            let spend = validate_creation_skills(
                &catalog,
                &CprSkillAllocation {
                    method: CprSkillMethod::Streetrat { role: role.to_owned() },
                    cultural_language: Some("Language (Cultural Origin)".to_owned()),
                    levels: fixture_list(role),
                },
            )
            .unwrap_or_else(|e| panic!("{role}: {e}"));
            assert_eq!(spend.spent, 86, "{role}");
            equal += 1;
        }
        assert_eq!(equal, 10);
    }

    #[test]
    fn the_p088_streetrat_solo_example_reproduces() {
        let catalog = catalog();
        let spend = validate_creation_skills(&catalog, &example("streetrat_solo_p088")).unwrap();
        assert_eq!((spend.spent, spend.remaining), (86, 0));
        assert_eq!(spend.levels.len(), 21, "20 listed skills + the Cultural Origin language");
        let free = spend.levels.iter().find(|l| l.key == "Language (Cultural Origin)").unwrap();
        assert_eq!((free.level, free.cost), (4, 0));
    }

    #[test]
    fn the_p090_edgerunner_and_p091_complete_package_examples_cost_86_line_by_line() {
        let catalog = catalog();
        let fx = fixture();
        for key in ["edgerunner_solo_p090", "complete_package_solo_p091"] {
            let spend = validate_creation_skills(&catalog, &example(key)).unwrap_or_else(|e| panic!("{key}: {e}"));
            assert_eq!(spend.spent, 86, "{key}");
            for row in fx["worked_examples"][key]["skills"].as_array().unwrap() {
                let name = row["name"].as_str().unwrap();
                let line = spend.levels.iter().find(|l| l.key == name).unwrap();
                assert_eq!(u64::from(line.cost), row["cost"].as_u64().unwrap(), "{key} {name} printed cost");
            }
        }
    }

    #[test]
    fn refusals_87_points_a_skill_at_7_and_a_basic_skill_at_1() {
        let catalog = catalog();
        let mut over = example("complete_package_solo_p091");
        over.levels.iter_mut().find(|(n, _)| n == "Tactics").unwrap().1 = 4;
        let err = validate_creation_skills(&catalog, &over).unwrap_err();
        assert!(err.contains("87 of 86"), "{err}");

        let mut seven = example("complete_package_solo_p091");
        seven.levels.iter_mut().find(|(n, _)| n == "Melee Weapon").unwrap().1 = 7;
        let err = validate_creation_skills(&catalog, &seven).unwrap_err();
        assert!(err.contains("Melee Weapon") && err.contains("above 6"), "{err}");

        let mut low = example("complete_package_solo_p091");
        low.levels.iter_mut().find(|(n, _)| n == "Stealth").unwrap().1 = 1;
        let err = validate_creation_skills(&catalog, &low).unwrap_err();
        assert!(err.contains("Stealth") && err.contains("at least 2"), "{err}");

        let mut edgerunner_low = example("edgerunner_solo_p090");
        edgerunner_low.levels.iter_mut().find(|(n, _)| n == "Tactics").unwrap().1 = 1;
        let err = validate_creation_skills(&catalog, &edgerunner_low).unwrap_err();
        assert!(err.contains("Tactics") && err.contains("lower than 2"), "{err}");
    }

    #[test]
    fn other_refusals_name_their_reason() {
        let catalog = catalog();
        let mut off_list = example("edgerunner_solo_p090");
        off_list.levels.push(("Archery".to_owned(), 2));
        assert!(validate_creation_skills(&catalog, &off_list).unwrap_err().contains("not on the Solo"));

        let mut missing_basic = example("complete_package_solo_p091");
        missing_basic.levels.retain(|(n, _)| n != "Persuasion");
        assert!(validate_creation_skills(&catalog, &missing_basic).unwrap_err().contains("Persuasion"));

        let mut twice = example("complete_package_solo_p091");
        twice.levels.push(("Tactics".to_owned(), 1));
        assert!(validate_creation_skills(&catalog, &twice).unwrap_err().contains("twice"));

        let mut unknown = example("complete_package_solo_p091");
        unknown.levels.push(("Hacking".to_owned(), 1));
        assert!(validate_creation_skills(&catalog, &unknown).unwrap_err().contains("Hacking"));

        let mut no_form = example("complete_package_solo_p091");
        no_form.levels.iter_mut().find(|(n, _)| n.starts_with("Martial Arts")).unwrap().0 = "Martial Arts".to_owned();
        assert!(validate_creation_skills(&catalog, &no_form).unwrap_err().contains("Martial Arts"));

        let mut changed = example("streetrat_solo_p088");
        changed.levels.iter_mut().find(|(n, _)| n == "Tactics").unwrap().1 = 5;
        assert!(validate_creation_skills(&catalog, &changed).unwrap_err().contains("Tactics"));
    }

    #[test]
    fn x2_skills_cost_two_points_per_level() {
        let catalog = catalog();
        let autofire = catalog.resolve("Autofire").unwrap().0;
        let handgun = catalog.resolve("Handgun").unwrap().0;
        assert_eq!((skill_cost(autofire, 6), skill_cost(handgun, 6)), (12, 6));
        let x2: Vec<&str> = catalog.skills().iter().filter(|s| s.x2).map(|s| s.name.as_str()).collect();
        assert_eq!(
            x2,
            ["Pilot Air Vehicle", "Martial Arts", "Autofire", "Heavy Weapons", "Demolitions", "Electronics/Security Tech", "Paramedic"]
        );
    }

    fn stats() -> CprStats {
        let mut stats = CprStats::default();
        for (stat, v) in [
            (CprStat::Int, 5),
            (CprStat::Ref, 8),
            (CprStat::Dex, 7),
            (CprStat::Tech, 3),
            (CprStat::Cool, 6),
            (CprStat::Will, 4),
            (CprStat::Luck, 6),
            (CprStat::Move, 5),
            (CprStat::Body, 8),
            (CprStat::Emp, 4),
        ] {
            stats.set(stat, v);
        }
        stats
    }

    #[test]
    fn every_one_of_the_66_skill_bases_is_stat_plus_level() {
        let catalog = catalog();
        let rows = skill_rows(&catalog, &stats(), &[]).unwrap();
        assert_eq!(rows.len(), 66);
        assert!(rows.iter().all(|r| r.id.starts_with("cpr.skill.")));
        for skill in catalog.skills() {
            let row = rows.iter().find(|r| r.id == format!("cpr.skill.{}", slug(&skill.name))).unwrap();
            assert_eq!(row.value, i16::from(stats().get(skill.stat)), "{}", skill.name);
        }

        let solo = example("complete_package_solo_p091");
        let rows = skill_rows(&catalog, &stats(), &solo.levels).unwrap();
        let get = |id: &str| rows.iter().find(|r| r.id == id).unwrap_or_else(|| panic!("{id}")).value;
        assert_eq!(get("cpr.skill.handgun"), 8 + 4);
        assert_eq!(get("cpr.skill.autofire"), 8 + 6);
        assert_eq!(get("cpr.skill.melee_weapon"), 7 + 5);
        assert_eq!(get("cpr.skill.martial_arts.karate"), 7 + 2);
        assert_eq!(get("cpr.skill.language.streetslang"), 5 + 2);
        assert_eq!(get("cpr.skill.local_expert.your_home"), 5 + 2);
        assert_eq!(get("cpr.skill.endurance"), 4 + 4);
        assert_eq!(get("cpr.skill.archery"), 8, "an untrained skill's base is its STAT");
        assert!(!rows.iter().any(|r| r.id == "cpr.skill.martial_arts"), "a held form replaces the bare row");
    }

    #[test]
    fn spend_rows_report_points_spent_and_left() {
        let catalog = catalog();
        let mut alloc = example("edgerunner_solo_p090");
        alloc.levels.iter_mut().find(|(n, _)| n == "Tactics").unwrap().1 = 2;
        let spend = validate_creation_skills(&catalog, &alloc).unwrap();
        let rows = spend_rows(&spend);
        assert_eq!(rows[0].id, "cpr.skills.creation_points_spent");
        assert_eq!((rows[0].value, rows[1].value), (84, 2));
        assert_eq!(rows[1].id, "cpr.skills.creation_points_remaining");
    }
}
