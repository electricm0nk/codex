//! The Starfinder 1e spellcasting totals -- spells per day, spells known and spell save DCs for
//! spell levels 0 through 6 -- read from the CONVERTED Starfinder package
//! (`data/starfinder-1e/sheet_rules/`), for every class the package gives a spell progression.
//! SD-37 E4.4 (`epic-breakdown.md` Epic E4; `decisions.md §5`).
//!
//! There is no per-class table: a class's progression is its own `SpellCell` / `SpellsKnown`
//! rows (the converter lowers the oracle's `CAST:` / `KNOWN:` level lines onto the class record),
//! and every other term is a held row of the same targets ([`super::sf_defense::held_rows`]),
//! folded by bonus type with the one evaluator.
//!
//! # Where each term comes from
//!
//! | Total | Term | Source |
//! |---|---|---|
//! | spells per day (1st–6th) | class table | the class's `SpellCell { class, level }` row (oracle `CAST:`) |
//! | spells per day (1st–6th) | bonus spells | system rule: the key ability score's Bonus Spells Per Day table (class pages, e.g. <https://www.aonsrd.com/Classes.aspx?ItemName=Mystic>), only at a level the class table already gives; the oracle game mode states the same rule once (`statsandchecks.lst` `BONUSSPELLLEVEL:<n> BASESTATSCORE:<10+2n> STATRANGE:8`) |
//! | spells per day (1st–6th) | every other held row | `SpellCell` rows of held records (a theme's divine boon) |
//! | spells per day (0) | — | no limit: "there is no limit to how many 0-level spells you can cast each day" (class pages, Spells) -- printed, never a number |
//! | spells known (0–6) | class table | the class's `SpellsKnown { class, level }` row (oracle `KNOWN:`) |
//! | spells known (0–6) | every other held row | `SpellsKnown` rows of held records (a mystic connection's connection spells, an archetype's −1) |
//! | save DC | 10 + spell level + key ability modifier | system rule, SRD Difficulty Class (<https://www.aonsrd.com/Rules.aspx?ID=106>); oracle `SPELLBASEDC:10+SPELLLEVEL+BASESPELLSTAT` (`miscinfo.lst`) |
//! | save DC | every other held row | `SpellDc` rows of held records whose scope is every spell or this class (Spell Focus) |
//!
//! A spell level is castable when the class's own table row for it is held and its level gate
//! admits the character: the spells-known row for every level, the spells-per-day row for 1st
//! and higher. A level that is not castable has no total at all -- it prints "—", never 0.
//!
//! # Paper-sheet rule
//!
//! Each total is one number with every term that resolves added in (`decisions.md §5`); the terms
//! stay beside it. The spells themselves (names, connection spells, divine boon picks) are
//! printed, not computed. A term this reader cannot resolve is a named [`SfChassisRefusal`].

use super::sf_chassis::{ability_modifier, key_ability_options, SfChassisRefusal, SfTerm, SfTotal};
use super::sf_defense::{fold, held, held_rows, SfBuild, SfHeld, SfNotFolded};
use crate::rules_core::sheet_rule::{
    evaluate_applies, sibling_line_gate, split_rule_id, Ability, BonusTarget, Gate, Scope, SheetRulePackage,
};

/// The highest Starfinder spell level.
pub const MAX_SPELL_LEVEL: u8 = 6;

/// One spell level of one class.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SfSpellLevel {
    pub level: u8,
    /// Spells per day at this level. `None` at level 0 (no daily limit) and at a level the class
    /// cannot cast.
    pub per_day: Option<SfTotal>,
    /// Level 0 of a class that casts 0-level spells: no daily limit (printed, never a number).
    pub unlimited: bool,
    /// Spells known at this level; `None` at a level the class cannot cast.
    pub known: Option<SfTotal>,
    /// The save DC of a spell of this level; `None` at a level the class cannot cast.
    pub save_dc: Option<SfTotal>,
}

impl SfSpellLevel {
    pub fn castable(&self) -> bool {
        self.known.is_some()
    }
}

/// The spellcasting of one class of one character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SfSpellcasting {
    /// The class record id (`core:class:mystic`).
    pub class: String,
    pub key_ability: Ability,
    /// Spell levels 0 through [`MAX_SPELL_LEVEL`], in order.
    pub levels: Vec<SfSpellLevel>,
    pub not_folded: Vec<SfNotFolded>,
}

impl SfSpellcasting {
    pub fn level(&self, n: u8) -> &SfSpellLevel {
        &self.levels[usize::from(n)]
    }
}

pub const REFUSED_CLASS_MISSING: &str = "sf_spells.class_missing";

const SRD_DC: &str = "SRD Difficulty Class (https://www.aonsrd.com/Rules.aspx?ID=106)";
const SRD_BONUS_SPELLS: &str =
    "SRD Bonus Spells Per Day (class pages, e.g. https://www.aonsrd.com/Classes.aspx?ItemName=Mystic)";

fn total_of(terms: Vec<SfTerm>) -> SfTotal {
    SfTotal { total: terms.iter().map(|t| t.value).sum(), terms }
}

fn refuse(id: &'static str, message: String) -> SfChassisRefusal {
    SfChassisRefusal { id, message }
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

/// Bonus spells per day at `spell_level` (1–6) for a key ability `score`: the Bonus Spells Per
/// Day table, which is the game mode's `BONUSSPELLLEVEL:<n> BASESTATSCORE:<10+2n> STATRANGE:8`
/// -- one spell from score 10 + 2n, one more for every 8 points above it. Level 0: none.
pub fn bonus_spells(score: i64, spell_level: u8) -> i64 {
    if spell_level == 0 {
        return 0;
    }
    let base = 10 + 2 * i64::from(spell_level);
    if score < base {
        0
    } else {
        1 + (score - base) / 8
    }
}

/// Whether the class record's own row for `target` (a `#` sibling of `class_id`) is held and
/// admitted for this character.
fn own_row_admits(package: &SheetRulePackage, sf: &SfHeld, class_id: &str, target: &BonusTarget) -> bool {
    let prefix = format!("{class_id}#");
    package.rules.range(prefix.clone()..).take_while(|(k, _)| k.starts_with(&prefix)).any(|(id, rule)| {
        rule.target.as_ref() == Some(target)
            && sf.held.rules.contains_key(id)
            && !sf.held.removed.contains(id)
            && matches!(
                sibling_line_gate(package, &sf.held, &sf.facts, rule, super::sf_defense::ctx_for(&sf.held, id)),
                Gate::Include
            )
    })
}

/// The class's own principal is held (the class gate admits the character).
fn class_held(package: &SheetRulePackage, sf: &SfHeld, class_id: &str) -> bool {
    package.rule(class_id).is_some_and(|r| {
        matches!(
            evaluate_applies(&r.applies, &sf.held, package, &sf.facts, super::sf_defense::ctx_for(&sf.held, class_id)),
            Gate::Include
        )
    })
}

/// The spellcasting of every class of `build` that has a spell progression, read from `package`
/// (the Starfinder package). A class with no `SpellsKnown` row of its own casts no spells and is
/// left out.
pub fn compute(package: &SheetRulePackage, build: &SfBuild) -> Result<Vec<SfSpellcasting>, SfChassisRefusal> {
    let sf = held(package, build)?;
    compute_with(package, build, &sf)
}

/// [`compute`] over an already-held build.
pub fn compute_with(package: &SheetRulePackage, build: &SfBuild, sf: &SfHeld) -> Result<Vec<SfSpellcasting>, SfChassisRefusal> {
    let mut out = Vec::new();
    for (class_id, _) in &build.chassis.classes {
        let principal = package
            .rule(class_id)
            .ok_or_else(|| refuse(REFUSED_CLASS_MISSING, format!("{class_id}: no such record in the Starfinder package")))?;
        let class = split_rule_id(class_id).2.to_string();
        let has_progression = (0..=MAX_SPELL_LEVEL).any(|l| {
            let t = BonusTarget::SpellsKnown { class: class.clone(), level: l };
            let prefix = format!("{class_id}#");
            package.rules.range(prefix.clone()..).take_while(|(k, _)| k.starts_with(&prefix)).any(|(_, r)| r.target.as_ref() == Some(&t))
        });
        if !has_progression || !class_held(package, sf, class_id) {
            continue;
        }
        // The key ability is the class's own (`KeyAbilityScore`), as for Resolve (E4.1): it sets
        // the DCs and the bonus spells (SRD class pages, Key Ability Score).
        let key_ability = match key_ability_options(principal)?.as_slice() {
            [one] => *one,
            options => match build.chassis.key_ability_choice {
                Some(c) if options.contains(&c) => c,
                other => {
                    return Err(refuse(
                        super::sf_chassis::REFUSED_KEY_ABILITY,
                        format!("{class_id}: key ability is a choice of {options:?}; the build chose {other:?}"),
                    ))
                }
            },
        };
        let score = sf.facts.ability_scores[ability_index(key_ability)];
        let modifier = ability_modifier(score);
        let mut not_folded = Vec::new();
        let mut levels = Vec::new();
        for level in 0..=MAX_SPELL_LEVEL {
            let known_target = BonusTarget::SpellsKnown { class: class.clone(), level };
            let cell_target = BonusTarget::SpellCell { class: class.clone(), level };
            let castable = own_row_admits(package, sf, class_id, &known_target);
            if !castable {
                levels.push(SfSpellLevel { level, per_day: None, unlimited: false, known: None, save_dc: None });
                continue;
            }
            let known = total_of(fold(held_rows(package, sf, &|t| *t == known_target, &mut not_folded)));
            let (per_day, unlimited) = if level == 0 {
                (None, true)
            } else if own_row_admits(package, sf, class_id, &cell_target) {
                let mut terms = fold(held_rows(package, sf, &|t| *t == cell_target, &mut not_folded));
                let bonus = bonus_spells(score, level);
                if bonus != 0 {
                    terms.push(SfTerm {
                        label: format!("bonus spells ({key_ability:?} {score})"),
                        value: bonus,
                        source: SRD_BONUS_SPELLS.into(),
                    });
                }
                (Some(total_of(terms)), false)
            } else {
                (None, false)
            };
            let mut dc_terms = vec![
                SfTerm { label: "base".into(), value: 10, source: SRD_DC.into() },
                SfTerm { label: "spell level".into(), value: i64::from(level), source: SRD_DC.into() },
                SfTerm { label: format!("{key_ability:?} (key ability) modifier"), value: modifier, source: SRD_DC.into() },
            ];
            let class_scope = Scope::Class(class.clone());
            dc_terms.extend(fold(held_rows(
                package,
                sf,
                &|t| matches!(t, BonusTarget::SpellDc(s) if *s == Scope::All || *s == class_scope),
                &mut not_folded,
            )));
            levels.push(SfSpellLevel {
                level,
                per_day,
                unlimited,
                known: Some(known),
                save_dc: Some(total_of(dc_terms)),
            });
        }
        not_folded.sort_by(|a, b| a.rule_id.cmp(&b.rule_id));
        not_folded.dedup();
        out.push(SfSpellcasting { class: class_id.clone(), key_ability, levels, not_folded });
    }
    Ok(out)
}

#[cfg(test)]
mod sf_seed {
    use super::super::sf_defense::seed_support::*;
    use super::*;

    fn caster<'a>(all: &'a [SfSpellcasting], class: &str) -> &'a SfSpellcasting {
        all.iter().find(|c| c.class == class).unwrap_or_else(|| panic!("no spellcasting for {class}: {all:?}"))
    }

    const ORD: [&str; 7] = ["0", "1st", "2nd", "3rd", "4th", "5th", "6th"];

    /// The criterion: spells per day, spells known and save DCs of the Mystic 5 and the
    /// Technomancer 5 = the SRD hand values (`seed-hand-values.md` E0.4 rows + the E4.4 DC
    /// supplement), every row the two files carry for them.
    #[test]
    fn sf_seed_spells_per_day_known_and_dcs_match_the_srd_hand_values() {
        let hand = hand_values();
        let mut bad = Vec::new();
        let mut checked = 0;
        for (seed, build) in seeds() {
            let all = compute(package(), &build).unwrap_or_else(|r| panic!("{seed}: {r:?}"));
            let class = &build.chassis.classes[0].0;
            let rows: Vec<(&(String, String), &Hand)> = hand
                .iter()
                .filter(|((s, f), _)| s == seed && (f.starts_with("Spells per day: ") || f.starts_with("Spells known: ") || f.starts_with("Spell DC: ")))
                .collect();
            if rows.is_empty() {
                assert!(all.is_empty(), "{seed}: no spell hand values, but the engine casts: {all:?}");
                continue;
            }
            let sc = caster(&all, class);
            for ((_, field), want) in rows {
                let (kind, ord) = field.split_once(": ").unwrap();
                let level = ORD.iter().position(|o| *o == ord).unwrap_or_else(|| panic!("{field}")) as u8;
                let l = sc.level(level);
                let got = match kind {
                    "Spells per day" => l.per_day.as_ref(),
                    "Spells known" => l.known.as_ref(),
                    "Spell DC" => l.save_dc.as_ref(),
                    other => panic!("{other}"),
                };
                checked += 1;
                if got.map(|t| Hand::Value(t.total)) != Some(*want) {
                    bad.push(format!("{seed} {field}: engine {got:?}, SRD {want:?}"));
                }
            }
        }
        assert_eq!(checked, 16, "2 casters x (2 per-day + 3 known + 3 DC rows)");
        assert!(bad.is_empty(), "{} of 16 mismatch:\n{}", bad.len(), bad.join("\n"));
    }

    /// Levels 0–6 for both 5th-level casters: 0 to 2nd castable (0 unlimited, no per-day
    /// number), 3rd to 6th not castable -- no total at all, never 0. The non-casters (Soldier 3,
    /// Envoy 3) have no spellcasting.
    #[test]
    fn sf_seed_casters_cast_levels_0_to_2_and_the_others_cast_nothing() {
        let mut casters = 0;
        for (seed, build) in seeds() {
            let all = compute(package(), &build).unwrap();
            match seed {
                "SF-Soldier-3" | "SF-Envoy-3" => assert!(all.is_empty(), "{seed}: {all:?}"),
                _ => {
                    casters += 1;
                    let sc = caster(&all, &build.chassis.classes[0].0);
                    assert_eq!(sc.levels.len(), 7);
                    assert!(sc.level(0).unlimited && sc.level(0).per_day.is_none() && sc.level(0).castable(), "{seed}: {:?}", sc.level(0));
                    for l in 1..=2 {
                        assert!(sc.level(l).per_day.is_some() && sc.level(l).known.is_some() && sc.level(l).save_dc.is_some(), "{seed} {l}");
                    }
                    for l in 3..=6 {
                        assert_eq!(sc.level(l), &SfSpellLevel { level: l, per_day: None, unlimited: false, known: None, save_dc: None }, "{seed}");
                    }
                }
            }
        }
        assert_eq!(casters, 2);
    }

    /// The Mystic's 5 spells known at 1st and 4 at 2nd include one connection spell each (Empath:
    /// detect thoughts, zone of truth): the class table term alone is 4 and 3. Spell Focus is the
    /// +1 on every DC; without the feat the DC is one lower.
    #[test]
    fn sf_seed_connection_spells_and_spell_focus_are_their_own_terms() {
        let (_, mystic) = seeds().into_iter().find(|(s, _)| *s == "SF-Mystic-5").unwrap();
        let sc = compute(package(), &mystic).unwrap().remove(0);
        for (level, table) in [(1u8, 4i64), (2, 3)] {
            let known = sc.level(level).known.clone().unwrap();
            let class_term: i64 = known.terms.iter().filter(|t| t.source.starts_with("core:class:mystic#")).map(|t| t.value).sum();
            let connection: i64 = known.terms.iter().filter(|t| t.source.starts_with("core:ability:empath#")).map(|t| t.value).sum();
            assert_eq!((class_term, connection), (table, 1), "{known:?}");
        }
        let focus = sc.level(1).save_dc.clone().unwrap();
        assert!(focus.terms.iter().any(|t| t.source.starts_with("core:feat:spell_focus") && t.value == 1), "{focus:?}");
        let mut without = mystic.clone();
        without.picks.retain(|p| p != "core:feat:spell_focus");
        let sc2 = compute(package(), &without).unwrap().remove(0);
        assert_eq!(sc2.level(1).save_dc.as_ref().unwrap().total, focus.total - 1);
    }

    /// SRD Table 4–5/4–6 (mystic) and 4–11/4–12 (technomancer) are the same table: spells per day
    /// (1st–6th) and spells known (0–6) at class levels 1–20, transcribed from
    /// https://www.aonsrd.com/Classes.aspx?ItemName=Mystic and ?ItemName=Technomancer (Spells Per
    /// Day / Spells Known; fetched 2026-10-04, `E4.4-srd-fetch-log.txt`); `0` = "-". The class's
    /// own table rows (no bonus spells: a score of 10) must give exactly these at every level.
    #[test]
    fn sf_seed_class_tables_match_the_srd_at_every_level_1_to_20() {
        #[rustfmt::skip]
        const PER_DAY: [[i64; 6]; 20] = [
            [2,0,0,0,0,0],[2,0,0,0,0,0],[3,0,0,0,0,0],[3,2,0,0,0,0],[4,2,0,0,0,0],
            [4,3,0,0,0,0],[4,3,2,0,0,0],[4,4,2,0,0,0],[5,4,3,0,0,0],[5,4,3,2,0,0],
            [5,4,4,2,0,0],[5,5,4,3,0,0],[5,5,4,3,2,0],[5,5,4,4,2,0],[5,5,5,4,3,0],
            [5,5,5,4,3,2],[5,5,5,4,4,2],[5,5,5,5,4,3],[5,5,5,5,5,4],[5,5,5,5,5,5],
        ];
        #[rustfmt::skip]
        const KNOWN: [[i64; 7]; 20] = [
            [4,2,0,0,0,0,0],[5,3,0,0,0,0,0],[6,4,0,0,0,0,0],[6,4,2,0,0,0,0],[6,4,3,0,0,0,0],
            [6,4,4,0,0,0,0],[6,5,4,2,0,0,0],[6,5,4,3,0,0,0],[6,5,4,4,0,0,0],[6,5,5,4,2,0,0],
            [6,6,5,4,3,0,0],[6,6,5,4,4,0,0],[6,6,5,5,4,2,0],[6,6,6,5,4,3,0],[6,6,6,5,4,4,0],
            [6,6,6,5,5,4,2],[6,6,6,6,5,4,3],[6,6,6,6,5,4,4],[6,6,6,6,5,5,4],[6,6,6,6,6,5,5],
        ];
        let mut checked = 0;
        for class in ["mystic", "technomancer"] {
            let class_id = format!("core:class:{class}");
            for lvl in 1..=20u8 {
                let build = SfBuild {
                    chassis: super::super::sf_chassis::SfChassisBuild {
                        classes: vec![(class_id.clone(), lvl)],
                        race: "core:race:human".into(),
                        ability_scores: [10, 10, 10, 10, 10, 10],
                        key_ability_choice: None,
                    },
                    theme: None,
                    armor: None,
                    picks: vec![],
                    skill_ranks: Default::default(),
                    choices: Default::default(),
                };
                let sc = compute(package(), &build).unwrap().remove(0);
                for sl in 0..=6u8 {
                    let row = usize::from(lvl - 1);
                    let own = |t: &Option<SfTotal>| -> i64 {
                        t.as_ref().map_or(0, |t| t.terms.iter().filter(|x| x.source.starts_with(&format!("{class_id}#"))).map(|x| x.value).sum())
                    };
                    assert_eq!(own(&sc.level(sl).known), KNOWN[row][usize::from(sl)], "{class} {lvl} known {sl}");
                    assert_eq!(sc.level(sl).castable(), KNOWN[row][usize::from(sl)] > 0, "{class} {lvl} castable {sl}");
                    if sl > 0 {
                        assert_eq!(own(&sc.level(sl).per_day), PER_DAY[row][usize::from(sl) - 1], "{class} {lvl} per day {sl}");
                    }
                    checked += 1;
                }
            }
        }
        assert_eq!(checked, 2 * 20 * 7);
    }

    /// The Bonus Spells Per Day table (SRD class pages, every row 1-11 … 30-31): the formula
    /// against the transcribed rows.
    #[test]
    fn sf_seed_bonus_spells_match_the_srd_table() {
        #[rustfmt::skip]
        let table: [(i64, [i64; 6]); 11] = [
            (10, [0,0,0,0,0,0]), (12, [1,0,0,0,0,0]), (14, [1,1,0,0,0,0]), (16, [1,1,1,0,0,0]),
            (18, [1,1,1,1,0,0]), (20, [2,1,1,1,1,0]), (22, [2,2,1,1,1,1]), (24, [2,2,2,1,1,1]),
            (26, [2,2,2,2,1,1]), (28, [3,2,2,2,2,1]), (30, [3,3,2,2,2,2]),
        ];
        for (low, row) in table {
            for score in [low, low + 1] {
                let got: Vec<i64> = (1..=6).map(|l| bonus_spells(score, l)).collect();
                assert_eq!(got, row.to_vec(), "score {score}");
            }
            assert_eq!(bonus_spells(low, 0), 0);
        }
    }
}
