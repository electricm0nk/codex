//! One class's sheet facts at one level -- weapon proficiency, caster level and class skills --
//! as the engine answers them, for a surface (the desktop) that must print them without keeping
//! a class table of its own (SD-36 Epic F6a).
//!
//! # Mechanism (one rule per fact, no per-class cases)
//!
//! - **Weapon proficiency**: the engine's own per-class rule
//!   (`feat_pillar_and_pool_aggregation::character_weapon_proficiency`): the static
//!   `weapon_tables::class_weapon_proficiency` row first; otherwise the converted-record reader
//!   ([`class_weapon_proficiency_view`]). The reader's `Unknown` stays Unknown with its reason.
//! - **Caster level**: every rule the class's own held set (the same `held_set` walk the
//!   proficiency and class-skill readers make) holds whose target is
//!   `BonusTarget::CasterLevel(Scope::Class(<slug>))`, evaluated by the one evaluator
//!   ([`evaluate`]). No such rule and a converter-attested closure (`closure_complete`) is
//!   [`CasterLevelFact::NotACaster`]; no such rule without that attestation is Unknown; a
//!   resolved value below 1 is "no caster level at this level" (the class casts nothing yet); a
//!   value the evaluator leaves as words, or held rules that disagree, is Unknown by name.
//!   A class-selection class (`TakenOnClass`) reads its base class line's caster-level rule.
//! - **Class skills**: [`class_skill_view`], verbatim.
//!
//! Deterministic; no I/O beyond the process-wide package handle.

use crate::rules_core::pilot_compute::class_proficiency_sheet_rules::{
    class_weapon_proficiency_view, ProficiencyAnswer, WeaponSetView,
};
use crate::rules_core::pilot_compute::class_skill_sheet_rules::{class_skill_view, ClassSkillAnswer};
use crate::rules_core::rules_tables::crb::weapon_tables::{class_weapon_proficiency, WeaponProficiency};
use crate::rules_core::sheet_rule::{
    evaluate, held_set, BonusTarget, CharacterFacts, EvalContext, HeldSeed, Scope, SheetLineValue, SheetRulePackage,
};
use crate::rules_core::sheet_rule_package;

/// Where a weapon-proficiency answer came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponFactSource {
    /// A `weapon_tables::CLASS_WEAPON_PROFICIENCIES` row (ruling 7: stays Rust until Starfinder).
    StaticRow,
    /// The converted-record reader (`class_proficiency_sheet_rules`).
    ConvertedRecord,
}

/// What one class is proficient with at one level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeaponFacts {
    Known {
        source: WeaponFactSource,
        tiers: Vec<WeaponProficiency>,
        /// Individually named weapon proficiencies.
        named: Vec<String>,
        /// Whole weapon groups (`"Close"`).
        groups: Vec<String>,
        /// Membership selectors expanded at ingest.
        sets: Vec<WeaponSetView>,
        /// Grants the class-level facts cannot decide, printed with their condition; plus the
        /// player's picks the class level cannot settle. Printed, never counted.
        printed: Vec<String>,
    },
    Unknown { reason: String },
}

/// One class's caster level at one level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CasterLevelFact {
    /// The class's caster-level rule resolved to `value` (>= 1). `rule` names it.
    Caster { value: i64, rule: String },
    /// The class casts nothing at this level: its caster-level rule resolves below 1, or the
    /// converter attests the closure complete and no rule the class holds names a caster level.
    NotACaster { reason: String },
    Unknown { reason: String },
}

/// Weapon proficiency, caster level and class skills for `class_id` (`class:<slug>`) at
/// `class_level`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassFacts {
    pub weapons: WeaponFacts,
    pub caster_level: CasterLevelFact,
    pub class_skills: ClassSkillAnswer,
}

pub fn class_facts(class_id: &str, class_level: u8) -> ClassFacts {
    let slug = crate::rules_core::sheet_rule::id_slug(class_id);
    ClassFacts {
        weapons: class_weapon_facts(class_id, class_level),
        caster_level: match sheet_rule_package::package() {
            Ok(package) => class_caster_level_in(package, &slug, class_level),
            Err(reason) => CasterLevelFact::Unknown { reason: format!("the converted rule package did not load: {reason}") },
        },
        class_skills: class_skill_view(&slug, class_level),
    }
}

/// The engine's per-class weapon-proficiency rule: the static row first, else the reader.
pub fn class_weapon_facts(class_id: &str, class_level: u8) -> WeaponFacts {
    if let Some(row) = class_weapon_proficiency(class_id) {
        return WeaponFacts::Known {
            source: WeaponFactSource::StaticRow,
            tiers: row.tiers.to_vec(),
            named: row.named.iter().map(|s| (*s).to_owned()).collect(),
            groups: row.weapon_groups.iter().map(|s| (*s).to_owned()).collect(),
            sets: Vec::new(),
            printed: Vec::new(),
        };
    }
    let slug = crate::rules_core::sheet_rule::id_slug(class_id);
    match class_weapon_proficiency_view(&slug, class_level) {
        ProficiencyAnswer::Unknown { reason } => WeaponFacts::Unknown { reason },
        ProficiencyAnswer::Known(view) => {
            let mut printed = view.printed_conditions.clone();
            printed.extend(view.weapon_picks.iter().map(|pick| {
                format!("{}: one weapon of the player's choice ({} options)", pick.label, pick.options.len())
            }));
            printed.extend(view.unresolved_picks.iter().map(|pick| format!("{pick} (unresolved pick)")));
            printed.extend(view.seeded_picks.iter().cloned());
            let mut named: Vec<String> = view.named.iter().cloned().collect();
            for conj in &view.all_of {
                named.push(conj.join(" "));
            }
            WeaponFacts::Known {
                source: WeaponFactSource::ConvertedRecord,
                tiers: view.tiers.clone(),
                named,
                groups: view.groups.iter().cloned().collect(),
                sets: view.sets.clone(),
                printed,
            }
        }
    }
}

/// The class's caster level at `class_level` over an explicit package (see the module doc).
pub fn class_caster_level_in(package: &SheetRulePackage, class_slug: &str, class_level: u8) -> CasterLevelFact {
    let Some(principal) = package.find("class", class_slug) else {
        return CasterLevelFact::Unknown { reason: format!("no converted class record for `{class_slug}`") };
    };
    let level = i64::from(class_level);
    let seed = HeldSeed { classes: vec![(class_slug.to_string(), level)], ..HeldSeed::default() };
    let facts = CharacterFacts { level, class_levels: vec![(class_slug.to_string(), level)], ..CharacterFacts::default() };
    let held = held_set(package, &seed, &facts);
    if !held.rules.contains_key(principal) {
        return CasterLevelFact::Unknown {
            reason: format!("the class principal rule {principal} is not held at level {class_level}"),
        };
    }
    let mut values: Vec<(i64, String)> = Vec::new();
    for (id, entry) in &held.rules {
        if held.removed.contains(id) {
            continue;
        }
        let Some(rule) = package.rule(id) else { continue };
        let Some(BonusTarget::CasterLevel(Scope::Class(target))) = &rule.target else { continue };
        // A class-selection class (`TakenOnClass`, the Unchained Summoner on the Summoner) casts
        // on its base class's line: that line's caster-level rule is its own.
        if target != class_slug && package.base_class_of(class_slug) != Some(target.as_str()) {
            continue;
        }
        let ctx = EvalContext { holder_class: entry.holder_class.clone(), ..EvalContext::default() };
        let line = evaluate(rule, &held, package, &facts, ctx);
        match line.value {
            SheetLineValue::Resolved(n) => values.push((i64::from(n), id.clone())),
            _ => {
                return CasterLevelFact::Unknown {
                    reason: format!("{id} ({}) does not resolve to a number at level {class_level}", rule.label),
                };
            }
        }
    }
    let Some((first, rule)) = values.first().cloned() else {
        // The class's own declared `SpellType` fact is the engine's caster signal (the same one
        // `class_spell_levels` reads, v0.8 B-9). A class that declares none and holds
        // no caster-level rule casts nothing; one that declares a spell type but holds no
        // caster-level rule is Unknown, never a guessed 0.
        let spell_type = held.declared.iter().find(|(name, _)| name == "SpellType").map(|(_, value)| value.clone());
        let attested = package.rule(principal).is_some_and(|r| r.closure_complete);
        return match spell_type {
            None => CasterLevelFact::NotACaster {
                reason: format!(
                    "`{class_slug}` at level {class_level} declares no SpellType and holds no rule naming a caster level"
                ),
            },
            Some(_) if attested => CasterLevelFact::NotACaster {
                reason: format!("no rule `{class_slug}` holds at level {class_level} names a caster level (closure attested complete)"),
            },
            Some(kind) => CasterLevelFact::Unknown {
                reason: format!(
                    "`{class_slug}` declares SpellType {kind} but no rule it holds at level {class_level} names a caster level"
                ),
            },
        };
    };
    if let Some((other, other_rule)) = values.iter().find(|(v, _)| *v != first) {
        return CasterLevelFact::Unknown {
            reason: format!("caster-level rules disagree at level {class_level}: {rule} = {first}, {other_rule} = {other}"),
        };
    }
    if first < 1 {
        return CasterLevelFact::NotACaster {
            reason: format!("{rule} resolves to {first} at level {class_level}: no caster level yet"),
        };
    }
    CasterLevelFact::Caster { value: first, rule }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn caster(slug: &str, level: u8) -> CasterLevelFact {
        class_facts(&format!("class:{slug}"), level).caster_level
    }

    #[test]
    fn magus_caster_level_equals_magus_level() {
        for level in [1u8, 7, 20] {
            match caster("magus", level) {
                CasterLevelFact::Caster { value, .. } => assert_eq!(value, i64::from(level)),
                other => panic!("magus {level}: {other:?}"),
            }
        }
    }

    #[test]
    fn wizard_caster_level_equals_wizard_level() {
        match caster("wizard", 5) {
            CasterLevelFact::Caster { value, .. } => assert_eq!(value, 5),
            other => panic!("wizard 5: {other:?}"),
        }
    }

    #[test]
    fn fighter_and_samurai_cast_nothing_and_paladin_casts_from_four() {
        assert!(matches!(caster("fighter", 5), CasterLevelFact::NotACaster { .. }), "{:?}", caster("fighter", 5));
        assert!(matches!(caster("samurai", 5), CasterLevelFact::NotACaster { .. }), "{:?}", caster("samurai", 5));
        assert!(matches!(caster("paladin", 3), CasterLevelFact::NotACaster { .. }), "{:?}", caster("paladin", 3));
        assert!(matches!(caster("paladin", 4), CasterLevelFact::Caster { value: 1, .. }), "{:?}", caster("paladin", 4));
    }

    fn martial(class_id: &str) -> bool {
        match class_weapon_facts(class_id, 1) {
            WeaponFacts::Known { tiers, .. } => tiers.contains(&WeaponProficiency::Martial),
            WeaponFacts::Unknown { reason } => panic!("{class_id}: {reason}"),
        }
    }

    #[test]
    fn samurai_warrior_and_magus_are_martial() {
        for class_id in ["class:samurai", "class:warrior", "class:magus", "class:fighter"] {
            assert!(martial(class_id), "{class_id}");
        }
        assert!(!martial("class:sorcerer"));
    }

    #[test]
    fn class_skills_are_the_class_skill_reader_verbatim() {
        assert_eq!(class_facts("class:barbarian", 1).class_skills, class_skill_view("barbarian", 1));
    }

    /// SD-36 F6a measurement: every roster class (the desktop's Create picker), at every level
    /// `1..=max_level`, through [`class_facts`]. Prints one row per class (the receipt's table)
    /// and pins the counts, so a class that gains or loses an answer moves a number here.
    #[test]
    fn every_roster_class_answer_is_counted() {
        let roster = crate::rules_core::class_census::class_creation_roster().expect("roster");
        let (mut weapons_known, mut skills_known, mut caster_known) = (0usize, 0usize, 0usize);
        let mut casters = 0usize;
        for entry in &roster {
            let mut weapon_unknown: Option<String> = None;
            let mut skill_unknown: Option<String> = None;
            let mut caster_unknown: Option<String> = None;
            let mut casts_at: Option<u8> = None;
            let mut martial = false;
            for level in 1..=entry.max_level {
                let facts = class_facts(&entry.id, level);
                match &facts.weapons {
                    WeaponFacts::Known { tiers, .. } => martial |= tiers.contains(&WeaponProficiency::Martial),
                    WeaponFacts::Unknown { reason } => {
                        weapon_unknown.get_or_insert(format!("L{level}: {reason}"));
                    }
                }
                if let ClassSkillAnswer::Unknown { reason } = &facts.class_skills {
                    skill_unknown.get_or_insert(format!("L{level}: {reason}"));
                }
                match &facts.caster_level {
                    CasterLevelFact::Caster { .. } => {
                        casts_at.get_or_insert(level);
                    }
                    CasterLevelFact::NotACaster { .. } => {}
                    CasterLevelFact::Unknown { reason } => {
                        caster_unknown.get_or_insert(format!("L{level}: {reason}"));
                    }
                }
            }
            weapons_known += usize::from(weapon_unknown.is_none());
            skills_known += usize::from(skill_unknown.is_none());
            caster_known += usize::from(caster_unknown.is_none());
            casters += usize::from(casts_at.is_some());
            eprintln!(
                "ROW | {} | weapons {} | martial {} | class skills {} | caster {} |",
                entry.id,
                weapon_unknown.as_deref().unwrap_or("known"),
                martial,
                skill_unknown.as_deref().unwrap_or("known"),
                match (&caster_unknown, casts_at) {
                    (Some(r), _) => format!("Unknown {r}"),
                    (None, Some(l)) => format!("caster from L{l}"),
                    (None, None) => "not a caster".to_owned(),
                }
            );
        }
        eprintln!(
            "roster {}: weapon proficiency known at every level {weapons_known}; class skills known at every level \
             {skills_known}; caster level answered at every level {caster_known} ({casters} casters)",
            roster.len()
        );
        assert_eq!(roster.len(), 59);
        assert_eq!((weapons_known, skills_known, caster_known, casters), (59, 50, 59, 37));
    }
}
