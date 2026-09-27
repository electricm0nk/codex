//! `list_class_facts` -- one read-only command answering, for each `(class id, class level)` a
//! character holds, the sheet facts the desktop used to keep in hand-typed class tables (SD-36
//! Epic F6a): weapon proficiency tiers + named weapons + groups + sets, caster level with the rule
//! that states it, class skills, and the hit die the hit-point fold reads.
//!
//! Every answer is the engine's (`codex::rules_core::pilot_compute::class_facts_sheet_rules`);
//! this module only shapes it for the wire. An answer the engine cannot give comes back as
//! `status: "unknown"` with its reason -- the sheet prints Unknown by name, never a default.

use serde::{Deserialize, Serialize};

use codex::rules_core::pilot_compute::class_facts_sheet_rules::{
    class_facts, CasterLevelFact, WeaponFactSource, WeaponFacts,
};
use codex::rules_core::pilot_compute::class_skill_sheet_rules::ClassSkillAnswer;
use codex::rules_core::rules_tables::crb::weapon_tables::WeaponProficiency;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassFactsQueryDto {
    pub class_id: String,
    pub level: u8,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListClassFactsRequest {
    pub classes: Vec<ClassFactsQueryDto>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponSetDto {
    pub label: String,
    pub members: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponProficiencyFactsDto {
    /// `known | unknown`.
    pub status: String,
    /// `staticRow | convertedRecord`; `None` when unknown.
    pub source: Option<String>,
    /// `Simple`, `Martial`, `Exotic`, each at most once.
    pub tiers: Vec<String>,
    pub named: Vec<String>,
    pub groups: Vec<String>,
    pub sets: Vec<WeaponSetDto>,
    /// Printed, never counted: gated grants and player picks the class level cannot settle.
    pub printed: Vec<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CasterLevelFactsDto {
    /// `caster | notACaster | unknown`.
    pub status: String,
    pub value: Option<i64>,
    /// The rule id stating it (`caster`), or the reason (`notACaster` / `unknown`).
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassSkillFactsDto {
    /// `known | unknown`.
    pub status: String,
    /// Skill ids as the converted package spells them (`climb`, `knowledge_nature`).
    pub skills: Vec<String>,
    /// Whole families (`Craft`, `Knowledge`).
    pub groups: Vec<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassFactsDto {
    pub class_id: String,
    pub level: u8,
    pub weapon_proficiency: WeaponProficiencyFactsDto,
    pub caster_level: CasterLevelFactsDto,
    pub class_skills: ClassSkillFactsDto,
    /// The hit die the hit-point fold reads (`None`: no record states one).
    pub hit_die: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListClassFactsResponse {
    /// One entry per requested class, in request order.
    pub classes: Vec<ClassFactsDto>,
}

fn tier_word(tier: WeaponProficiency) -> String {
    match tier {
        WeaponProficiency::Simple => "Simple",
        WeaponProficiency::Martial => "Martial",
        WeaponProficiency::Exotic => "Exotic",
    }
    .to_owned()
}

fn weapon_dto(facts: WeaponFacts) -> WeaponProficiencyFactsDto {
    match facts {
        WeaponFacts::Known { source, tiers, named, groups, sets, printed } => WeaponProficiencyFactsDto {
            status: "known".to_owned(),
            source: Some(
                match source {
                    WeaponFactSource::StaticRow => "staticRow",
                    WeaponFactSource::ConvertedRecord => "convertedRecord",
                }
                .to_owned(),
            ),
            tiers: tiers.into_iter().map(tier_word).collect(),
            named,
            groups,
            sets: sets.into_iter().map(|set| WeaponSetDto { label: set.label, members: set.members }).collect(),
            printed,
            reason: None,
        },
        WeaponFacts::Unknown { reason } => WeaponProficiencyFactsDto {
            status: "unknown".to_owned(),
            source: None,
            tiers: Vec::new(),
            named: Vec::new(),
            groups: Vec::new(),
            sets: Vec::new(),
            printed: Vec::new(),
            reason: Some(reason),
        },
    }
}

fn caster_dto(fact: CasterLevelFact) -> CasterLevelFactsDto {
    match fact {
        CasterLevelFact::Caster { value, rule } => {
            CasterLevelFactsDto { status: "caster".to_owned(), value: Some(value), source: rule }
        }
        CasterLevelFact::NotACaster { reason } => {
            CasterLevelFactsDto { status: "notACaster".to_owned(), value: None, source: reason }
        }
        CasterLevelFact::Unknown { reason } => CasterLevelFactsDto { status: "unknown".to_owned(), value: None, source: reason },
    }
}

fn class_skill_dto(answer: ClassSkillAnswer) -> ClassSkillFactsDto {
    match answer {
        ClassSkillAnswer::Known(view) => ClassSkillFactsDto {
            status: "known".to_owned(),
            skills: view.skills.into_iter().collect(),
            groups: view.groups.into_iter().collect(),
            reason: None,
        },
        ClassSkillAnswer::Unknown { reason } => {
            ClassSkillFactsDto { status: "unknown".to_owned(), skills: Vec::new(), groups: Vec::new(), reason: Some(reason) }
        }
    }
}

/// The pure form of [`list_class_facts`].
pub fn build_class_facts(request: &ListClassFactsRequest) -> ListClassFactsResponse {
    let classes = request
        .classes
        .iter()
        .map(|query| {
            let facts = class_facts(&query.class_id, query.level);
            ClassFactsDto {
                class_id: query.class_id.clone(),
                level: query.level,
                weapon_proficiency: weapon_dto(facts.weapons),
                caster_level: caster_dto(facts.caster_level),
                class_skills: class_skill_dto(facts.class_skills),
                hit_die: crate::character_hub::class_hit_die_for(&query.class_id),
            }
        })
        .collect();
    ListClassFactsResponse { classes }
}

/// Read-only: the engine's weapon proficiency, caster level, class skills and hit die for each
/// requested `(class id, class level)`. `async` so the first call (which loads the converted
/// package unless startup already has) runs off the UI thread.
#[tauri::command(async)]
pub fn list_class_facts(request: ListClassFactsRequest) -> ListClassFactsResponse {
    build_class_facts(&request)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn one(class_id: &str, level: u8) -> ClassFactsDto {
        build_class_facts(&ListClassFactsRequest { classes: vec![ClassFactsQueryDto { class_id: class_id.to_owned(), level }] })
            .classes
            .remove(0)
    }

    #[test]
    fn samurai_and_warrior_are_martial_and_magus_casts_at_its_level() {
        for class_id in ["class:samurai", "class:warrior", "class:magus"] {
            let facts = one(class_id, 3);
            assert_eq!(facts.weapon_proficiency.status, "known", "{class_id}: {:?}", facts.weapon_proficiency);
            assert!(facts.weapon_proficiency.tiers.contains(&"Martial".to_owned()), "{class_id}: {:?}", facts.weapon_proficiency);
        }
        let magus = one("class:magus", 7);
        assert_eq!((magus.caster_level.status.as_str(), magus.caster_level.value), ("caster", Some(7)));
        assert_eq!(one("class:samurai", 7).caster_level.status, "notACaster");
    }

    #[test]
    fn the_wire_shape_is_camel_case() {
        let json = serde_json::to_value(one("class:barbarian", 1)).expect("serialize");
        for key in ["classId", "weaponProficiency", "casterLevel", "classSkills", "hitDie"] {
            assert!(json.get(key).is_some(), "{key}: {json}");
        }
        assert_eq!(json["classSkills"]["status"], "known");
        assert_eq!(json["hitDie"], 12);
    }

    /// The wire the frontend tests read (`testSupport/classFactsWire.ts`): every roster class at
    /// levels 1 and 7, as `list_class_facts` serves it. Fails on any drift between the live
    /// command and the committed file; `CODEX_WRITE_F6A_WIRE=1` rewrites it.
    #[test]
    fn class_facts_wire_for_every_roster_class_matches_the_committed_artifact() {
        let roster = crate::character_hub::build_class_creation_roster().expect("roster");
        assert_eq!(roster.classes.len(), 59);
        let classes = roster
            .classes
            .iter()
            .flat_map(|c| [1u8, 7].map(|level| ClassFactsQueryDto { class_id: c.class_id.clone(), level }))
            .collect();
        let response = build_class_facts(&ListClassFactsRequest { classes });
        let path = crate::authoring_workbench::codex_repo_root()
            .expect("repo root")
            .join("docs/release/SD-36-consolidation/artifacts/epic-f/stage-f6/f6a-class-facts-wire.json");
        let live = serde_json::to_string_pretty(&response).expect("serialize") + "\n";
        if std::env::var_os("CODEX_WRITE_F6A_WIRE").is_some() {
            std::fs::write(&path, &live).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            return;
        }
        let committed = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        assert!(committed == live, "{} drifted from the live command; rerun with CODEX_WRITE_F6A_WIRE=1", path.display());
    }
}
