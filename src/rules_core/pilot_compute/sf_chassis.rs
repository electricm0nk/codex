//! The Starfinder 1e chassis -- base attack bonus, Fortitude/Reflex/Will, Hit Points, Stamina
//! Points, Resolve Points and the key ability -- read from the CONVERTED Starfinder package
//! (`data/starfinder-1e/sheet_rules/<book>/{class,race}/<slug>.json`), for any class and race
//! the package holds. SD-37 E4.1 (`epic-breakdown.md` Epic E4; `decisions.md §5`, §8).
//!
//! There is no per-class module for Starfinder: every class is read the same way, off the rows
//! the converter wrote for it.
//!
//! # Where each term comes from
//!
//! | Total | Term | Source |
//! |---|---|---|
//! | BAB | class progression | the class principal's `BaseAttack` row |
//! | Fort/Ref/Will | class base save | the class's three `BaseSave` rows |
//! | Fort/Ref/Will | ability modifier (Con/Dex/Wis) | system rule, SRD Saving Throw Types (<https://www.aonsrd.com/Rules.aspx?ID=106>) |
//! | Hit Points | class Hit Points per level | the class's `Hp` row (oracle `BONUS:HP\|CURRENTMAX\|<k>*<Class>LVL` minus the die, `decisions.md §8`) |
//! | Hit Points | hit die, one point per level | the class principal's `StatBlock "Hit die"` prose row (`d1`) |
//! | Hit Points | racial Hit Points | the race's `Hp` row (`<race>#race_hp`, lowered by the converter from the race's `RaceHP` grant chain) |
//! | Stamina | class Stamina per level | the class's `Stamina` row (oracle `BONUS:HP\|ALTHP`) |
//! | Stamina | Con modifier × level | system rule, SRD Calculating Stamina Points (<https://www.aonsrd.com/Rules.aspx?ID=49>) |
//! | Resolve | ½ level (min 1) + key ability modifier | system rule, SRD Calculating Resolve Points (<https://www.aonsrd.com/Rules.aspx?ID=50>) |
//! | key ability | the class's `KeyAbilityScore` fact | the class principal's `FactDeclare` |
//!
//! The three system rules are the Starfinder game system's own formulas, the same for every
//! character (the oracle states them once, on its `Constitution` stat and `Default` rows); every
//! number that differs by class or race is read from the package.
//!
//! # Paper-sheet rule
//!
//! Each total is one number with every term that resolves added in (`decisions.md §5`). The
//! terms are kept beside the total so a sheet line can explain itself. A term this reader
//! cannot resolve is a named [`SfChassisRefusal`], never a 0.

use crate::rules_core::sheet_rule::{
    evaluate_expr_from_facts, Ability, Applies, BonusTarget, CharacterFacts, Cmp, Effect, Expr, ProseFamily,
    ProsePiece, Save, SheetRule, SheetRulePackage, SheetValue,
};

/// One Starfinder character as the chassis reads it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SfChassisBuild {
    /// `(class record id, levels)`, e.g. `("core:class:soldier", 3)`.
    pub classes: Vec<(String, u8)>,
    /// The race record id, e.g. `"core:race:human"`.
    pub race: String,
    /// Final ability scores (race, theme, point buy and increases applied), Str Dex Con Int Wis Cha.
    pub ability_scores: [i64; 6],
    /// The key ability the player chose, for a class whose key ability is a choice
    /// (`Str or Dex`). Ignored for a class that names one ability.
    pub key_ability_choice: Option<Ability>,
}

/// One term of a total: what it is, its value, and the rule id (or system rule) it came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SfTerm {
    pub label: String,
    pub value: i64,
    pub source: String,
}

/// A sheet total and the terms added into it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SfTotal {
    pub total: i64,
    pub terms: Vec<SfTerm>,
}

impl SfTotal {
    fn of(terms: Vec<SfTerm>) -> SfTotal {
        SfTotal { total: terms.iter().map(|t| t.value).sum(), terms }
    }
}

/// The chassis totals of one Starfinder character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SfChassis {
    pub base_attack_bonus: SfTotal,
    pub fortitude: SfTotal,
    pub reflex: SfTotal,
    pub will: SfTotal,
    pub hit_points: SfTotal,
    pub stamina: SfTotal,
    pub key_ability: Ability,
    pub resolve: SfTotal,
}

/// Why a chassis total cannot be computed. `id` is stable; `message` names the record and row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SfChassisRefusal {
    pub id: &'static str,
    pub message: String,
}

pub const REFUSED_RECORD_MISSING: &str = "sf_chassis.record_missing";
pub const REFUSED_ROW_MISSING: &str = "sf_chassis.row_missing";
pub const REFUSED_ROW_NOT_A_NUMBER: &str = "sf_chassis.row_not_a_number";
pub const REFUSED_GATE_UNREAD: &str = "sf_chassis.gate_unread";
pub const REFUSED_HIT_DIE: &str = "sf_chassis.hit_die";
pub const REFUSED_KEY_ABILITY: &str = "sf_chassis.key_ability";
pub const REFUSED_MULTICLASS_KEY_ABILITY: &str = "sf_chassis.multiclass_key_ability";

const SRD_SAVES: &str = "SRD Saving Throw Types (https://www.aonsrd.com/Rules.aspx?ID=106)";
const SRD_STAMINA: &str = "SRD Calculating Stamina Points (https://www.aonsrd.com/Rules.aspx?ID=49)";
const SRD_RESOLVE: &str = "SRD Calculating Resolve Points (https://www.aonsrd.com/Rules.aspx?ID=50)";

fn refuse(id: &'static str, message: String) -> SfChassisRefusal {
    SfChassisRefusal { id, message }
}

/// Starfinder Table 2-1: floor(score / 2) − 5.
pub fn ability_modifier(score: i64) -> i64 {
    score.div_euclid(2) - 5
}

fn ability_index(a: Ability) -> usize {
    match a {
        Ability::Str => 0,
        Ability::Dex => 1,
        Ability::Con => 2,
        Ability::Int => 3,
        Ability::Wis => 4,
        Ability::Cha => 5,
    }
}

/// A record's principal rule and every `#` sibling, in id order.
fn record_rules<'p>(package: &'p SheetRulePackage, id: &str) -> Result<Vec<&'p SheetRule>, SfChassisRefusal> {
    let principal = package
        .rule(id)
        .ok_or_else(|| refuse(REFUSED_RECORD_MISSING, format!("{id}: no such record in the Starfinder package")))?;
    let prefix = format!("{id}#");
    let mut out = vec![principal];
    out.extend(package.rules.range(prefix.clone()..).take_while(|(k, _)| k.starts_with(&prefix)).map(|(_, r)| r));
    Ok(out)
}

/// Whether `gate` admits a character with `facts`. Only the gate shapes the converter writes on
/// class and race chassis rows are read (always; the class's own level ceiling; an `All` of
/// those); any other gate is refused by name rather than guessed open or shut.
fn admits(gate: &Applies, facts: &CharacterFacts, rule_id: &str) -> Result<bool, SfChassisRefusal> {
    match gate {
        Applies::Always => Ok(true),
        Applies::Compare { lhs: Expr::ClassLevel(c), op: Cmp::Lte, rhs: Expr::Const(n) } => {
            let level = facts.class_levels.iter().find(|(id, _)| id == c).map_or(0, |(_, l)| *l);
            Ok(level <= i64::from(*n))
        }
        Applies::All(terms) => {
            for t in terms {
                if !admits(t, facts, rule_id)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        other => Err(refuse(REFUSED_GATE_UNREAD, format!("{rule_id}: gate {other:?} is not a chassis gate this reader reads"))),
    }
}

/// The value of `rule` for `facts`, truncated once at the boundary (`SheetValue::Number`).
fn number(rule: &SheetRule, facts: &CharacterFacts) -> Result<i64, SfChassisRefusal> {
    match &rule.value {
        SheetValue::Number(e) => Ok(evaluate_expr_from_facts(e, facts).trunc()),
        _ => Err(refuse(REFUSED_ROW_NOT_A_NUMBER, format!("{}: the {:?} row converted to words, not a number", rule.id, rule.target))),
    }
}

/// Every admitted row of `rules` targeting `target`, as terms.
fn terms_for(rules: &[&SheetRule], target: &BonusTarget, facts: &CharacterFacts) -> Result<Vec<SfTerm>, SfChassisRefusal> {
    let mut out = Vec::new();
    for rule in rules.iter().filter(|r| r.target.as_ref() == Some(target)) {
        if admits(&rule.applies, facts, &rule.id)? {
            out.push(SfTerm { label: rule.label.clone(), value: number(rule, facts)?, source: rule.id.clone() });
        }
    }
    Ok(out)
}

/// The principal's `StatBlock "Hit die"` text (`"d1"`) as its size.
fn hit_die(principal: &SheetRule) -> Result<i64, SfChassisRefusal> {
    let text: Option<String> = principal.prose.iter().find_map(|seg| match &seg.family {
        ProseFamily::StatBlock(l) if l == "Hit die" => {
            seg.pieces.iter().map(|p| if let ProsePiece::Text(s) = p { Some(s.as_str()) } else { None }).collect()
        }
        _ => None,
    });
    let text = text.ok_or_else(|| refuse(REFUSED_HIT_DIE, format!("{}: the class states no hit die", principal.id)))?;
    let size: i64 = text
        .strip_prefix('d')
        .and_then(|n| n.parse().ok())
        .ok_or_else(|| refuse(REFUSED_HIT_DIE, format!("{}: hit die {text:?} is not d<n>", principal.id)))?;
    // Starfinder's PCGen classes carry a d1: one hit point per level, nothing rolled. A larger
    // die would be a rolled die, which no Starfinder total reads.
    if size != 1 {
        return Err(refuse(REFUSED_HIT_DIE, format!("{}: hit die d{size} is a rolled die; only the Starfinder d1 is read", principal.id)));
    }
    Ok(size)
}

/// The abilities the class principal's `KeyAbilityScore` fact allows: `CHA` -> [Cha],
/// `Str or Dex` -> [Str, Dex].
pub fn key_ability_options(principal: &SheetRule) -> Result<Vec<Ability>, SfChassisRefusal> {
    let value = principal
        .grants
        .iter()
        .find_map(|g| match g {
            Effect::FactDeclare { name, value } if name == "KeyAbilityScore" => Some(value.as_str()),
            _ => None,
        })
        .ok_or_else(|| refuse(REFUSED_KEY_ABILITY, format!("{}: the class declares no KeyAbilityScore", principal.id)))?;
    value
        .split(" or ")
        .map(|p| match p.trim().to_ascii_uppercase().as_str() {
            "STR" => Ok(Ability::Str),
            "DEX" => Ok(Ability::Dex),
            "CON" => Ok(Ability::Con),
            "INT" => Ok(Ability::Int),
            "WIS" => Ok(Ability::Wis),
            "CHA" => Ok(Ability::Cha),
            _ => Err(refuse(REFUSED_KEY_ABILITY, format!("{}: KeyAbilityScore {value:?} names no ability", principal.id))),
        })
        .collect()
}

/// The chassis of `build`, read from `package` (the Starfinder package,
/// `live_sheet_rules_for(GameSystem::Starfinder1e)`).
pub fn compute(package: &SheetRulePackage, build: &SfChassisBuild) -> Result<SfChassis, SfChassisRefusal> {
    let mods: [i64; 6] = build.ability_scores.map(ability_modifier);
    let mut class_levels: Vec<(String, i64)> = Vec::new();
    let mut class_records: Vec<(Vec<&SheetRule>, i64)> = Vec::new();
    for (id, levels) in &build.classes {
        let rules = record_rules(package, id)?;
        // The class's own level variable is the slug its rows bind (`ClassLevel("soldier")`).
        let slug = crate::rules_core::sheet_rule::split_rule_id(id).2.to_string();
        class_levels.push((slug, i64::from(*levels)));
        class_records.push((rules, i64::from(*levels)));
    }
    let level: i64 = class_levels.iter().map(|(_, l)| l).sum();
    let facts = CharacterFacts {
        level,
        class_levels: class_levels.clone(),
        ability_scores: build.ability_scores,
        ability_mods: mods,
        ..CharacterFacts::default()
    };

    let mut bab = Vec::new();
    let mut saves: [Vec<SfTerm>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    let mut hp = Vec::new();
    let mut stamina = Vec::new();
    for (rules, levels) in &class_records {
        let principal = rules[0];
        let class_bab = terms_for(rules, &BonusTarget::BaseAttack, &facts)?;
        if class_bab.is_empty() {
            return Err(refuse(REFUSED_ROW_MISSING, format!("{}: no BaseAttack row", principal.id)));
        }
        bab.extend(class_bab);
        for (i, save) in [Save::Fortitude, Save::Reflex, Save::Will].into_iter().enumerate() {
            let t = terms_for(rules, &BonusTarget::BaseSave(save), &facts)?;
            if t.is_empty() {
                return Err(refuse(REFUSED_ROW_MISSING, format!("{}: no BaseSave({save:?}) row", principal.id)));
            }
            saves[i].extend(t);
        }
        let class_hp = terms_for(rules, &BonusTarget::Hp, &facts)?;
        if class_hp.is_empty() {
            return Err(refuse(REFUSED_ROW_MISSING, format!("{}: no Hp row", principal.id)));
        }
        hp.extend(class_hp);
        hp.push(SfTerm {
            label: format!("{} (hit die, 1 per level)", principal.label),
            value: hit_die(principal)? * levels,
            source: principal.id.clone(),
        });
        let class_sp = terms_for(rules, &BonusTarget::Stamina, &facts)?;
        if class_sp.is_empty() {
            return Err(refuse(REFUSED_ROW_MISSING, format!("{}: no Stamina row", principal.id)));
        }
        stamina.extend(class_sp);
    }

    let race_rules = record_rules(package, &build.race)?;
    let race_hp = terms_for(&race_rules, &BonusTarget::Hp, &facts)?;
    if race_hp.is_empty() {
        return Err(refuse(REFUSED_ROW_MISSING, format!("{}: no racial Hp row", build.race)));
    }
    hp.extend(race_hp);

    let con = mods[ability_index(Ability::Con)];
    stamina.push(SfTerm { label: "Constitution modifier × level".into(), value: con * level, source: SRD_STAMINA.into() });
    // Stamina Points never go below 0 (SRD Calculating Stamina Points).
    let mut stamina = SfTotal::of(stamina);
    stamina.total = stamina.total.max(0);

    for (i, ability) in [Ability::Con, Ability::Dex, Ability::Wis].into_iter().enumerate() {
        saves[i].push(SfTerm { label: format!("{ability:?} modifier"), value: mods[ability_index(ability)], source: SRD_SAVES.into() });
    }

    let key_ability = match class_records.as_slice() {
        [(rules, _)] => {
            let options = key_ability_options(rules[0])?;
            match options.as_slice() {
                [only] => *only,
                _ => match build.key_ability_choice {
                    Some(c) if options.contains(&c) => c,
                    other => {
                        return Err(refuse(
                            REFUSED_KEY_ABILITY,
                            format!("{}: key ability is a choice of {options:?}; the build chose {other:?}", rules[0].id),
                        ))
                    }
                },
            }
        }
        _ => {
            return Err(refuse(
                REFUSED_MULTICLASS_KEY_ABILITY,
                format!("{} classes: a multiclass key ability is not read by this chassis", class_records.len()),
            ))
        }
    };
    let resolve = SfTotal::of(vec![
        SfTerm { label: "½ character level (minimum 1)".into(), value: (level / 2).max(1), source: SRD_RESOLVE.into() },
        SfTerm { label: format!("{key_ability:?} (key ability) modifier"), value: mods[ability_index(key_ability)], source: SRD_RESOLVE.into() },
    ]);

    let [fortitude, reflex, will] = saves.map(SfTotal::of);
    Ok(SfChassis {
        base_attack_bonus: SfTotal::of(bab),
        fortitude,
        reflex,
        will,
        hit_points: SfTotal::of(hp),
        stamina,
        key_ability,
        resolve,
    })
}

#[cfg(test)]
mod sf_seed {
    //! The four SD-37 Starfinder seeds against the SRD hand values (`decisions.md §9`, §18).
    //!
    //! Build inputs (race, class, level, final scores, key ability) are `seed-builds.md`
    //! (E0.4). Expected values are read from `seed-hand-values.md`, whose every row cites its SRD
    //! URL and section: bytes the engine does not read. A fixture row without a source URL fails.

    use super::*;
    use crate::rules_core::corpus_loader::live_sheet_rules_for;
    use crate::rules_core::game_system::GameSystem;
    use std::collections::BTreeMap;

    const HAND_VALUES: &str = "docs/release/SD-37-starfinder-1e/artifacts/epic_0/seed-hand-values.md";

    /// `(seed, build)` -- `seed-builds.md` §1–§4 (final scores after race, theme and the 5th-level
    /// increase).
    fn seeds() -> Vec<(&'static str, SfChassisBuild)> {
        let b = |class: &str, level: u8, race: &str, scores: [i64; 6], choice: Option<Ability>| SfChassisBuild {
            classes: vec![(format!("core:class:{class}"), level)],
            race: format!("core:race:{race}"),
            ability_scores: scores,
            key_ability_choice: choice,
        };
        vec![
            ("SF-Soldier-3", b("soldier", 3, "human", [16, 14, 12, 11, 10, 10], Some(Ability::Str))),
            ("SF-Mystic-5", b("mystic", 5, "lashunta", [10, 14, 8, 14, 19, 15], None)),
            ("SF-Technomancer-5", b("technomancer", 5, "android", [10, 16, 14, 19, 13, 8], None)),
            ("SF-Envoy-3", b("envoy", 3, "ysoki", [8, 13, 12, 12, 10, 18], None)),
        ]
    }

    /// `(seed, field) -> value` from the `| SF-<seed> | <field> | <value> | <source> |` rows.
    fn hand_values() -> BTreeMap<(String, String), i64> {
        let path = crate::support::paths::repo_root().join(HAND_VALUES);
        let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        let mut out = BTreeMap::new();
        for line in text.lines() {
            let cells: Vec<&str> = line.split('|').map(str::trim).collect();
            if cells.len() < 6 || !cells[1].starts_with("SF-") {
                continue;
            }
            let Ok(v) = cells[3].replace('\u{2212}', "-").trim_start_matches('+').parse::<i64>() else { continue };
            assert!(cells[4].contains("https://"), "fixture row without a source URL: {line}");
            out.insert((cells[1].to_string(), cells[2].to_string()), v);
        }
        out
    }

    fn package() -> &'static SheetRulePackage {
        live_sheet_rules_for(GameSystem::Starfinder1e).expect("the Starfinder package loads (data/starfinder-1e/sheet_rules)")
    }

    #[test]
    fn sf_seed_chassis_totals_match_the_srd_hand_values() {
        let hand = hand_values();
        let mut bad = Vec::new();
        let mut checked = 0;
        for (seed, build) in seeds() {
            let sheet = compute(package(), &build).unwrap_or_else(|r| panic!("{seed}: {r:?}"));
            for (field, got) in [
                ("BAB", sheet.base_attack_bonus.total),
                ("Fort", sheet.fortitude.total),
                ("Ref", sheet.reflex.total),
                ("Will", sheet.will.total),
                ("HP", sheet.hit_points.total),
                ("Stamina", sheet.stamina.total),
                ("Resolve", sheet.resolve.total),
            ] {
                let want = *hand.get(&(seed.to_string(), field.to_string())).unwrap_or_else(|| panic!("{seed} {field}: no hand value"));
                checked += 1;
                if got != want {
                    bad.push(format!("{seed} {field}: engine {got}, SRD {want}"));
                }
            }
        }
        assert_eq!(checked, 28, "4 seeds x 7 totals");
        assert!(bad.is_empty(), "{} of 28 mismatch:\n{}", bad.len(), bad.join("\n"));
    }

    #[test]
    fn sf_seed_key_ability_is_read_from_the_class() {
        let want = [Ability::Str, Ability::Wis, Ability::Int, Ability::Cha];
        for ((seed, build), want) in seeds().into_iter().zip(want) {
            assert_eq!(compute(package(), &build).unwrap().key_ability, want, "{seed}");
        }
    }

    #[test]
    fn sf_seed_a_choice_key_ability_without_a_legal_choice_is_refused() {
        let (_, mut soldier) = seeds().remove(0);
        soldier.key_ability_choice = None;
        assert_eq!(compute(package(), &soldier).unwrap_err().id, REFUSED_KEY_ABILITY);
        soldier.key_ability_choice = Some(Ability::Wis);
        assert_eq!(compute(package(), &soldier).unwrap_err().id, REFUSED_KEY_ABILITY);
        soldier.key_ability_choice = Some(Ability::Dex);
        assert_eq!(compute(package(), &soldier).unwrap().key_ability, Ability::Dex);
    }

    /// Every term of every total names the rule id or the SRD rule it came from.
    #[test]
    fn sf_seed_every_term_names_its_source() {
        for (seed, build) in seeds() {
            let s = compute(package(), &build).unwrap();
            for total in [&s.base_attack_bonus, &s.fortitude, &s.reflex, &s.will, &s.hit_points, &s.stamina, &s.resolve] {
                for t in &total.terms {
                    assert!(t.source.starts_with("core:") || t.source.starts_with("SRD "), "{seed}: {t:?}");
                }
            }
            assert!(s.hit_points.terms.iter().any(|t| t.source == build.race.clone() + "#race_hp"), "{seed}: racial HP read from the race record");
        }
    }

    /// No per-class module: the chassis reads any class the package holds, not only the seeds'.
    #[test]
    fn sf_seed_every_core_player_class_has_a_chassis_at_level_1() {
        let mut n = 0;
        for class in ["envoy", "mechanic", "mystic", "operative", "solarian", "soldier", "technomancer"] {
            let build = SfChassisBuild {
                classes: vec![(format!("core:class:{class}"), 1)],
                race: "core:race:human".into(),
                ability_scores: [10; 6],
                key_ability_choice: Some(Ability::Str),
            };
            let s = compute(package(), &build).unwrap_or_else(|r| panic!("{class}: {r:?}"));
            assert!(s.hit_points.total > 0 && s.stamina.total > 0 && s.resolve.total >= 1, "{class}: {s:?}");
            n += 1;
        }
        assert_eq!(n, 7);
    }
}
