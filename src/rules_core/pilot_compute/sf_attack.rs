//! The Starfinder 1e attack totals -- the melee and ranged attack bonus, and each carried weapon's
//! attack bonus and damage bonus -- read from the CONVERTED Starfinder package
//! (`data/starfinder-1e/sheet_rules/`). SD-37 E7.1 (`decisions.md §5`: "BAB and attack bonuses"
//! are sheet totals; `progress.md` DISCOVERED E4.MC -> E7.1, E5.3 -> E7.1, E5.MC -> E7.1).
//!
//! # Where each term comes from
//!
//! | Total | Term | Source |
//! |---|---|---|
//! | every attack | base attack bonus | the E4.1 chassis total (`sf_chassis`) |
//! | melee attack | Strength modifier | SRD Basic Attack and Damage Bonuses (<https://www.aonsrd.com/Rules.aspx?ID=105>) |
//! | ranged attack | Dexterity modifier | same |
//! | thrown attack | Strength modifier | same |
//! | operative melee weapon | Dexterity modifier instead of Strength, when higher | SRD weapon special property Operative (<https://www.aonsrd.com/WeaponProperties.aspx?ItemName=Operative>) |
//! | a weapon the character is not proficient with | −4 | SRD Weapon Proficiency (<https://www.aonsrd.com/Rules.aspx?ID=108>) |
//! | melee and thrown damage | Strength modifier | SRD Basic Attack and Damage Bonuses (ID=105); Thrown (<https://www.aonsrd.com/WeaponProperties.aspx?ItemName=Thrown>) |
//! | attack / damage | every held row targeting the attack or the weapon | the package rows, folded by bonus type (`sf_defense::fold`) |
//!
//! The rows a weapon reads are the package's per-weapon rows (oracle `CATEGORY:Weapon`
//! abilities, `scr_abilities.lst`: `BONUS:WEAPONPROF=<key>|TOHIT|ToHit_<group>` and
//! `|DAMAGE|Damage_<group>`), held when the character holds the weapon's proficiency (they are
//! granted by `Weapon Prof ~ <group>`), and valued by the group's variables: Weapon Focus raises
//! `ToHit_<group>`, Weapon Specialization raises `Damage_<group>` by the character level
//! (`EffectiveLVL`, the global `Default`'s `BONUS:VAR|EffectiveLVL|TL`). Each row names its weapon
//! by the oracle's KEY (`Named("Laser rifle (azimuth)")`), and a carried equipment record's id is
//! the slug of that same KEY (the corpus unit id, `sf_corpus`), so a weapon's rows are the rows
//! whose named weapon slugs to the record's own slug. A weapon no per-weapon row names (a grenade:
//! its proficiency is the group `Grenades`) gets no total: it is listed in
//! [`SfAttacks::not_folded`] and its line prints its dice, never a total missing a term.
//!
//! A row that resolves to words or holds only in a situation (a condition's −2) is not added
//! ([`SfNotFolded`]), as for every other Starfinder total.

use std::collections::BTreeMap;

use super::sf_chassis::{SfTerm, SfTotal};
use super::sf_defense::{fold, held_rows, SfHeld, SfNotFolded};
use crate::rules_core::sheet_rule::{
    slug, split_rule_id, BonusTarget, ProseFamily, ProsePiece, RuleId, SheetRule, SheetRulePackage, SheetValue, WeaponRef,
};

const SRD_ATTACKS: &str = "SRD Basic Attack and Damage Bonuses (https://www.aonsrd.com/Rules.aspx?ID=105)";
const SRD_OPERATIVE: &str = "SRD weapon special property Operative (https://www.aonsrd.com/WeaponProperties.aspx?ItemName=Operative)";
const SRD_THROWN: &str = "SRD weapon special property Thrown (https://www.aonsrd.com/WeaponProperties.aspx?ItemName=Thrown)";
const SRD_PROFICIENCY: &str = "SRD Weapon Proficiency (https://www.aonsrd.com/Rules.aspx?ID=108)";

/// How a weapon attacks (SRD ID=105).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SfAttackKind {
    Melee,
    Ranged,
    Thrown,
}

/// One carried weapon's attack and damage bonus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SfWeaponAttack {
    /// The equipment record (`core:equipment:laser_rifle_azimuth`).
    pub item: RuleId,
    /// Its printed name (`Laser rifle, azimuth`).
    pub label: String,
    pub kind: SfAttackKind,
    /// The weapon's damage dice as the record states them (`1d8`).
    pub dice: Option<String>,
    pub attack: SfTotal,
    /// The number added to the dice.
    pub damage: SfTotal,
}

/// The attack totals of one Starfinder character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SfAttacks {
    pub melee: SfTotal,
    pub ranged: SfTotal,
    /// One per distinct carried weapon, in carried order.
    pub weapons: Vec<SfWeaponAttack>,
    pub not_folded: Vec<SfNotFolded>,
}

fn total(terms: Vec<SfTerm>) -> SfTotal {
    SfTotal { total: terms.iter().map(|t| t.value).sum(), terms }
}

/// The text of a record's prose rows of `family` (`Special` -> `Analog.operative`).
fn prose_text(rule: &SheetRule, wanted: &ProseFamily) -> Option<String> {
    let parts: Vec<String> = rule
        .prose
        .iter()
        .filter(|seg| &seg.family == wanted)
        .flat_map(|seg| seg.pieces.iter().filter_map(|p| if let ProsePiece::Text(s) = p { Some(s.clone()) } else { None }))
        .collect();
    (!parts.is_empty()).then(|| parts.join(" "))
}

/// The weapon special properties a record states (`Special` row, oracle `SPROP:`), lower-cased.
fn special_properties(rule: &SheetRule) -> Vec<String> {
    prose_text(rule, &ProseFamily::Special)
        .unwrap_or_default()
        .split(['.', ',', ';', '/'])
        .map(|p| p.trim().to_ascii_lowercase())
        .filter(|p| !p.is_empty() && p != "-")
        .collect()
}

/// Whether the record is a weapon (it states damage dice) and how it attacks: a `Melee`-tagged
/// weapon is melee; one with a range (`StatBlock "Range"`, oracle `RANGE:`) is thrown when it has
/// the thrown property and ranged otherwise. `None` for a record that is not a weapon.
fn weapon_kind(rule: &SheetRule) -> Option<Result<SfAttackKind, String>> {
    if !matches!(rule.value, SheetValue::Dice { .. }) {
        return None;
    }
    if rule.tags.iter().any(|t| t == "Melee") {
        return Some(Ok(SfAttackKind::Melee));
    }
    if prose_text(rule, &ProseFamily::StatBlock("Range".into())).is_some() {
        let thrown = special_properties(rule).iter().any(|p| p.starts_with("thrown"));
        return Some(Ok(if thrown { SfAttackKind::Thrown } else { SfAttackKind::Ranged }));
    }
    Some(Err(format!("{}: states damage dice but neither a Melee tag nor a range", rule.id)))
}

/// Every package row naming a weapon (`WeaponAttack` / `Damage` of `Named(<key>)`), by the slug of
/// the named key.
fn per_weapon_rows(package: &SheetRulePackage) -> BTreeMap<String, Vec<RuleId>> {
    let mut out: BTreeMap<String, Vec<RuleId>> = BTreeMap::new();
    for rule in package.rules_of_kind("ability") {
        if let Some(BonusTarget::WeaponAttack(WeaponRef::Named(n)) | BonusTarget::Damage(WeaponRef::Named(n))) = &rule.target {
            out.entry(slug(n)).or_default().push(rule.id.clone());
        }
    }
    out
}

/// Whether a weapon reference matches a weapon of `kind` whose per-weapon rows are `named` and
/// whose tags are `tags`.
fn weapon_ref_matches(r: &WeaponRef, kind: Option<SfAttackKind>, named: &[String], tags: &[String]) -> bool {
    match r {
        WeaponRef::Any => true,
        WeaponRef::Melee => kind == Some(SfAttackKind::Melee),
        WeaponRef::Ranged => matches!(kind, Some(SfAttackKind::Ranged | SfAttackKind::Thrown)),
        WeaponRef::Named(n) => named.iter().any(|x| x == n),
        WeaponRef::Group(t) => tags.iter().any(|x| x == t),
        WeaponRef::Chosen(_) => false,
    }
}

/// The melee and ranged attack bonus of `build` and the attack and damage bonus of every weapon
/// it carries (`carried`: `(equipment record id, quantity)`, the worn armour excluded).
/// `base_attack` is the E4.1 chassis's base attack bonus.
pub fn compute_with(
    package: &SheetRulePackage,
    sf: &SfHeld,
    base_attack: &SfTotal,
    carried: &[(RuleId, u32)],
) -> SfAttacks {
    // The base attack bonus is a fact every attack row may read (Weapon Focus's
    // `1+(BAB<=(EffectiveLVL-3))`): evaluate the attack rows with it set.
    let sf = SfHeld {
        held: sf.held.clone(),
        facts: crate::rules_core::sheet_rule::CharacterFacts { base_attack: base_attack.total, ..sf.facts.clone() },
        class_skills: sf.class_skills.clone(),
        class_skill_groups: sf.class_skill_groups.clone(),
    };
    let sf = &sf;
    let mods = sf.facts.ability_mods;
    let (str_mod, dex_mod) = (mods[0], mods[1]);
    let bab = SfTerm { label: "base attack bonus".into(), value: base_attack.total, source: "sf.base_attack_bonus".into() };
    let mut not_folded = Vec::new();

    let base = |ability: SfTerm, kind: SfAttackKind, not_folded: &mut Vec<SfNotFolded>| -> SfTotal {
        let rows = held_rows(
            package,
            sf,
            &|t| match t {
                BonusTarget::Attack => true,
                BonusTarget::WeaponAttack(r @ (WeaponRef::Any | WeaponRef::Melee | WeaponRef::Ranged)) => {
                    weapon_ref_matches(r, Some(kind), &[], &[])
                }
                _ => false,
            },
            not_folded,
        );
        let mut terms = vec![bab.clone(), ability];
        terms.extend(fold(rows));
        total(terms)
    };
    let melee = base(
        SfTerm { label: "Strength modifier".into(), value: str_mod, source: SRD_ATTACKS.into() },
        SfAttackKind::Melee,
        &mut not_folded,
    );
    let ranged = base(
        SfTerm { label: "Dexterity modifier".into(), value: dex_mod, source: SRD_ATTACKS.into() },
        SfAttackKind::Ranged,
        &mut not_folded,
    );

    let by_slug = per_weapon_rows(package);
    let mut weapons = Vec::new();
    let mut seen = Vec::new();
    for (item, _) in carried {
        if seen.contains(item) {
            continue;
        }
        seen.push(item.clone());
        let Some(rule) = package.rule(item) else { continue };
        let kind = match weapon_kind(rule) {
            None => continue,
            Some(Ok(k)) => k,
            Some(Err(reason)) => {
                not_folded.push(SfNotFolded { rule_id: item.clone(), reason });
                continue;
            }
        };
        let rows_for_weapon = by_slug.get(split_rule_id(item).2).cloned().unwrap_or_default();
        if rows_for_weapon.is_empty() {
            not_folded.push(SfNotFolded {
                rule_id: item.clone(),
                reason: format!("no per-weapon row names {} (oracle proficiency is a group, e.g. Grenades): no total", rule.label),
            });
            continue;
        }
        let named: Vec<String> = rows_for_weapon
            .iter()
            .filter_map(|id| package.rule(id))
            .filter_map(|r| match &r.target {
                Some(BonusTarget::WeaponAttack(WeaponRef::Named(n)) | BonusTarget::Damage(WeaponRef::Named(n))) => Some(n.clone()),
                _ => None,
            })
            .collect();
        let held_attack_row = rows_for_weapon.iter().any(|id| {
            sf.held.rules.contains_key(id)
                && !sf.held.removed.contains(id)
                && matches!(package.rule(id).and_then(|r| r.target.as_ref()), Some(BonusTarget::WeaponAttack(_)))
        });
        let tags = rule.tags.clone();
        let operative = special_properties(rule).iter().any(|p| p == "operative");

        let ability = match kind {
            SfAttackKind::Melee if operative && dex_mod > str_mod => {
                SfTerm { label: "Dexterity modifier (operative)".into(), value: dex_mod, source: SRD_OPERATIVE.into() }
            }
            SfAttackKind::Melee => SfTerm { label: "Strength modifier".into(), value: str_mod, source: SRD_ATTACKS.into() },
            SfAttackKind::Ranged => SfTerm { label: "Dexterity modifier".into(), value: dex_mod, source: SRD_ATTACKS.into() },
            SfAttackKind::Thrown => SfTerm { label: "Strength modifier (thrown)".into(), value: str_mod, source: SRD_ATTACKS.into() },
        };
        let mut attack_terms = vec![bab.clone(), ability];
        if !held_attack_row {
            attack_terms.push(SfTerm { label: "not proficient".into(), value: -4, source: SRD_PROFICIENCY.into() });
        }
        attack_terms.extend(fold(held_rows(
            package,
            sf,
            &|t| match t {
                BonusTarget::Attack => true,
                BonusTarget::WeaponAttack(r) => weapon_ref_matches(r, Some(kind), &named, &tags),
                _ => false,
            },
            &mut not_folded,
        )));

        let mut damage_terms = Vec::new();
        match kind {
            SfAttackKind::Melee => damage_terms.push(SfTerm { label: "Strength modifier".into(), value: str_mod, source: SRD_ATTACKS.into() }),
            SfAttackKind::Thrown => damage_terms.push(SfTerm { label: "Strength modifier (thrown)".into(), value: str_mod, source: SRD_THROWN.into() }),
            SfAttackKind::Ranged => {}
        }
        damage_terms.extend(fold(held_rows(
            package,
            sf,
            &|t| match t {
                BonusTarget::Damage(r) => weapon_ref_matches(r, Some(kind), &named, &tags),
                _ => false,
            },
            &mut not_folded,
        )));
        let dice = match &rule.value {
            SheetValue::Dice { dice, .. } => Some(dice.clone()),
            _ => None,
        };
        weapons.push(SfWeaponAttack {
            item: item.clone(),
            label: rule.label.clone(),
            kind,
            dice,
            attack: total(attack_terms),
            damage: total(damage_terms),
        });
    }
    SfAttacks { melee, ranged, weapons, not_folded }
}

#[cfg(test)]
mod sf_seed {
    use super::super::sf_defense::seed_support::{package, seeds};
    use super::super::sf_defense::held;
    use super::*;

    fn carried(ids: &[&str]) -> Vec<(RuleId, u32)> {
        ids.iter().map(|i| (format!("core:equipment:{i}"), 1)).collect()
    }

    fn attacks(seed: &str, items: &[&str]) -> SfAttacks {
        let (_, build) = seeds().into_iter().find(|(s, _)| *s == seed).expect("seed");
        let chassis = super::super::sf_chassis::compute(package(), &build.chassis).expect("chassis");
        let sf = held(package(), &build).expect("held");
        compute_with(package(), &sf, &chassis.base_attack_bonus, &carried(items))
    }

    fn weapon<'a>(a: &'a SfAttacks, label: &str) -> &'a SfWeaponAttack {
        a.weapons.iter().find(|w| w.label == label).unwrap_or_else(|| panic!("{label}: {:?}", a.weapons))
    }

    /// SF-Soldier-3 (Str 16, Dex 14, BAB +3; soldier weapon specialization at 3rd level):
    /// melee +3 +3 = +6, ranged +3 +2 = +5 (SRD ID=105); the azimuth laser rifle (a longarm,
    /// ranged) +5 and damage + character level 3 (Weapon Specialization, SRD FeatDisplay
    /// Weapon Specialization); the tactical baton (an operative basic melee weapon) +3 + Str 3 =
    /// +6, damage Str 3 + half the level 1 = +4. PCGen agrees on all six
    /// (`scripts/oracle_harness/sf_parity/sf_soldier_3.oracle.txt`).
    #[test]
    fn sf_seed_the_soldier_attacks() {
        let a = attacks("SF-Soldier-3", &["laser_rifle_azimuth", "baton_tactical", "battery"]);
        assert_eq!((a.melee.total, a.ranged.total), (6, 5), "{a:?}");
        let rifle = weapon(&a, "Laser rifle, azimuth");
        assert_eq!((rifle.kind, rifle.attack.total, rifle.damage.total, rifle.dice.as_deref()), (SfAttackKind::Ranged, 5, 3, Some("1d8")), "{rifle:?}");
        let baton = weapon(&a, "Baton, tactical");
        assert_eq!((baton.kind, baton.attack.total, baton.damage.total), (SfAttackKind::Melee, 6, 4), "{baton:?}");
        assert_eq!(a.weapons.len(), 2, "a battery is not a weapon");
    }

    /// SF-Envoy-3 (Str 8, Dex 13): the operative baton takes Dex +1 over Str −1 for the attack
    /// (SRD Operative) but Str for the damage; the semi-auto pistol (a small arm) adds half the
    /// level (1) to damage.
    #[test]
    fn sf_seed_the_envoy_uses_dexterity_with_an_operative_weapon() {
        let a = attacks("SF-Envoy-3", &["semi_auto_pistol_tactical", "baton_tactical"]);
        assert_eq!((a.melee.total, a.ranged.total), (1, 3), "{a:?}");
        let baton = weapon(&a, "Baton, tactical");
        assert_eq!((baton.attack.total, baton.damage.total), (3, 0), "{baton:?}");
        assert!(baton.attack.terms.iter().any(|t| t.label == "Dexterity modifier (operative)"), "{baton:?}");
        let pistol = weapon(&a, "Semi-auto pistol, tactical");
        assert_eq!((pistol.attack.total, pistol.damage.total), (3, 1), "{pistol:?}");
    }

    /// A weapon the character holds no proficiency for takes −4 (SRD ID=108): the mystic is not
    /// proficient with longarms.
    #[test]
    fn sf_seed_a_weapon_without_proficiency_takes_minus_four() {
        let a = attacks("SF-Mystic-5", &["laser_rifle_azimuth"]);
        let rifle = weapon(&a, "Laser rifle, azimuth");
        assert!(rifle.attack.terms.iter().any(|t| t.label == "not proficient" && t.value == -4), "{rifle:?}");
        assert_eq!(rifle.attack.total, 3 + 2 - 4, "{rifle:?}");
    }
}
