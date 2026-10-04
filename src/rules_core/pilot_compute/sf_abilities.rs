//! The Starfinder 1e ability scores -- race, theme, point buy and the ability increases at
//! 5th, 10th, 15th and 20th level -- read from the CONVERTED Starfinder package
//! (`data/starfinder-1e/sheet_rules/`), for any race and theme the package holds. SD-37 E4.3
//! (`epic-breakdown.md` Epic E4; `decisions.md §5`).
//!
//! There is no per-race or per-theme table: the race and theme adjustments are the held set's
//! own `Ability` rows ([`super::sf_defense::held`]), folded by bonus type with the one evaluator.
//!
//! # Where each term comes from
//!
//! | Term | Source |
//! |---|---|
//! | base 10 | system rule, SRD Buying Ability Scores step 1 (<https://www.aonsrd.com/Rules.aspx?ID=42>) |
//! | race | the race record's `Ability` rows (oracle `BONUS:STAT` on `scr_races.lst`), a held subrace (`lashunta_subrace_damaya`) and a chosen `+2 Racial Stat Bonus` (human) |
//! | theme | the theme's held `Ability` row (oracle `BONUS:STAT\|…\|1\|TYPE=Theme`) |
//! | point buy | the build ([`SfAbilityBuild::point_buy`]); 10 points, one-for-one, no score above 18 at creation: SRD ID=42 steps 3–4, the same budget as the oracle game mode's `METHOD:Standard POINTS:10` (`pointbuymethods_system.lst`) |
//! | ability increase | system rule, SRD Step 1: Apply any Ability Increases (<https://www.aonsrd.com/Rules.aspx?ID=57>): at 5th, 10th, 15th and 20th level four different scores each rise by 2, or by 1 if the score is already 17 or higher -- the rule the oracle states once for every playable race (`Playable Race Selected`: `PCStatBoostLVL_A\|if(TL>=5,4,0)`; `PC Level 5 Stat Boost ~ STR +2` `PREVARLTEQ:STRSCORENoPMods,16`) |
//!
//! The point budget and the increase rule are the Starfinder game system's own rules, the same
//! for every character; every number that differs by race or theme is read from the package.
//!
//! # Paper-sheet rule
//!
//! Each score is one number with every term that resolves added in; the terms stay beside it so
//! the sheet line explains itself. A build the rules do not allow (points over budget, a score
//! above 18 at creation, an increase not chosen or chosen twice) is a named
//! [`SfChassisRefusal`], never a clamped number.

use std::collections::BTreeMap;

use super::sf_chassis::{ability_modifier, SfChassisRefusal, SfTerm, SfTotal};
use super::sf_defense::{fold, held, held_rows, SfBuild, SfNotFolded};
use crate::rules_core::sheet_rule::{Ability, BonusTarget, SheetRulePackage};

/// The ability-score choices of one Starfinder character (the race, theme and picks are the
/// [`SfBuild`]'s).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SfAbilityBuild {
    /// Points spent on each ability at creation, Str Dex Con Int Wis Cha.
    pub point_buy: [i64; 6],
    /// The character level of each ability increase (5, 10, 15, 20) -> the four abilities
    /// chosen at that level.
    pub increases: BTreeMap<u8, Vec<Ability>>,
}

/// The six ability scores of one Starfinder character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SfAbilityScores {
    /// Final scores (creation + every increase reached), Str Dex Con Int Wis Cha.
    pub scores: [SfTotal; 6],
    /// Scores at creation (base, race, theme, point buy), before any increase.
    pub at_creation: [i64; 6],
    /// Points of the 10-point budget left unspent (printed; never added).
    pub points_unspent: i64,
    pub not_folded: Vec<SfNotFolded>,
}

impl SfAbilityScores {
    /// The final scores as numbers, Str Dex Con Int Wis Cha.
    pub fn finals(&self) -> [i64; 6] {
        [0, 1, 2, 3, 4, 5].map(|i| self.scores[i].total)
    }

    pub fn modifiers(&self) -> [i64; 6] {
        self.finals().map(ability_modifier)
    }
}

pub const REFUSED_POINT_BUY_NEGATIVE: &str = "sf_abilities.point_buy_negative";
pub const REFUSED_POINT_BUY_OVER_BUDGET: &str = "sf_abilities.point_buy_over_budget";
pub const REFUSED_SCORE_OVER_18_AT_CREATION: &str = "sf_abilities.score_over_18_at_creation";
pub const REFUSED_INCREASE_LEVEL: &str = "sf_abilities.increase_level";
pub const REFUSED_INCREASE_NOT_FOUR_DIFFERENT: &str = "sf_abilities.increase_not_four_different";
pub const REFUSED_INCREASE_NOT_CHOSEN: &str = "sf_abilities.increase_not_chosen";
pub const REFUSED_SCORES_UNSETTLED: &str = "sf_abilities.scores_unsettled";

/// SRD Buying Ability Scores: 10 points, one-for-one.
pub const POINT_BUY_BUDGET: i64 = 10;
/// SRD Buying Ability Scores step 4: no score above 18 at creation.
pub const MAX_SCORE_AT_CREATION: i64 = 18;
/// SRD Step 1: Apply any Ability Increases -- the character levels that grant one.
pub const INCREASE_LEVELS: [u8; 4] = [5, 10, 15, 20];
/// Scores raised per increase.
pub const SCORES_PER_INCREASE: usize = 4;

const SRD_POINT_BUY: &str = "SRD Buying Ability Scores (https://www.aonsrd.com/Rules.aspx?ID=42)";
const SRD_INCREASE: &str = "SRD Step 1: Apply any Ability Increases (https://www.aonsrd.com/Rules.aspx?ID=57)";

const ABILITIES: [Ability; 6] = [Ability::Str, Ability::Dex, Ability::Con, Ability::Int, Ability::Wis, Ability::Cha];

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

fn ordinal(level: u8) -> String {
    format!("{level}th")
}

/// The increase levels `level` has reached, checked against the build's choices.
fn check_increases(abilities: &SfAbilityBuild, level: i64) -> Result<(), SfChassisRefusal> {
    for (&at, chosen) in &abilities.increases {
        if !INCREASE_LEVELS.contains(&at) || i64::from(at) > level {
            return Err(refuse(
                REFUSED_INCREASE_LEVEL,
                format!("an increase at level {at}: increases come at {INCREASE_LEVELS:?}, and this character is level {level}"),
            ));
        }
        let mut distinct = chosen.clone();
        distinct.sort_by_key(|a| ability_index(*a));
        distinct.dedup();
        if chosen.len() != SCORES_PER_INCREASE || distinct.len() != SCORES_PER_INCREASE {
            return Err(refuse(
                REFUSED_INCREASE_NOT_FOUR_DIFFERENT,
                format!("the level-{at} increase chose {chosen:?}: it raises {SCORES_PER_INCREASE} different scores"),
            ));
        }
    }
    for at in INCREASE_LEVELS.into_iter().filter(|&at| i64::from(at) <= level) {
        if !abilities.increases.contains_key(&at) {
            return Err(refuse(REFUSED_INCREASE_NOT_CHOSEN, format!("level {level} reached the {} level increase; the build chose none", ordinal(at))));
        }
    }
    Ok(())
}

/// The scores of `build` with `abilities`, read from `package` (the Starfinder package). The
/// build's own `chassis.ability_scores` is not read: the held set is re-held with the scores
/// this computes until they settle (a held row's gate may read a score).
pub fn compute(package: &SheetRulePackage, build: &SfBuild, abilities: &SfAbilityBuild) -> Result<SfAbilityScores, SfChassisRefusal> {
    let level: i64 = build.chassis.classes.iter().map(|(_, l)| i64::from(*l)).sum();
    if let Some(i) = abilities.point_buy.iter().position(|p| *p < 0) {
        return Err(refuse(REFUSED_POINT_BUY_NEGATIVE, format!("{:?}: {} points spent; points only raise a score", ABILITIES[i], abilities.point_buy[i])));
    }
    let spent: i64 = abilities.point_buy.iter().sum();
    if spent > POINT_BUY_BUDGET {
        return Err(refuse(REFUSED_POINT_BUY_OVER_BUDGET, format!("{spent} points spent of {POINT_BUY_BUDGET}")));
    }
    check_increases(abilities, level)?;

    let mut guess: [i64; 6] = abilities.point_buy.map(|p| 10 + p);
    for _ in 0..4 {
        let mut b = build.clone();
        b.chassis.ability_scores = guess;
        let sf = held(package, &b)?;
        let mut not_folded = Vec::new();
        let mut terms: [Vec<SfTerm>; 6] = Default::default();
        for (i, a) in ABILITIES.into_iter().enumerate() {
            terms[i].push(SfTerm { label: "base".into(), value: 10, source: SRD_POINT_BUY.into() });
            terms[i].extend(fold(held_rows(package, &sf, &|t| *t == BonusTarget::Ability(a), &mut not_folded)));
            if abilities.point_buy[i] != 0 {
                terms[i].push(SfTerm { label: "point buy".into(), value: abilities.point_buy[i], source: SRD_POINT_BUY.into() });
            }
        }
        let at_creation: [i64; 6] = [0, 1, 2, 3, 4, 5].map(|i| terms[i].iter().map(|t| t.value).sum());
        if let Some(i) = at_creation.iter().position(|s| *s > MAX_SCORE_AT_CREATION) {
            return Err(refuse(
                REFUSED_SCORE_OVER_18_AT_CREATION,
                format!("{:?} {} at creation: no score above {MAX_SCORE_AT_CREATION} at creation ({:?})", ABILITIES[i], at_creation[i], terms[i]),
            ));
        }
        let mut current = at_creation;
        for (at, chosen) in &abilities.increases {
            for a in chosen {
                let i = ability_index(*a);
                let (value, label) = if current[i] >= 17 {
                    (1, format!("{} level ability increase (+1: score 17 or higher)", ordinal(*at)))
                } else {
                    (2, format!("{} level ability increase", ordinal(*at)))
                };
                current[i] += value;
                terms[i].push(SfTerm { label, value, source: SRD_INCREASE.into() });
            }
        }
        if current == guess {
            return Ok(SfAbilityScores {
                scores: terms.map(|t| SfTotal { total: t.iter().map(|x| x.value).sum(), terms: t }),
                at_creation,
                points_unspent: POINT_BUY_BUDGET - spent,
                not_folded,
            });
        }
        guess = current;
    }
    Err(refuse(REFUSED_SCORES_UNSETTLED, format!("the held set's ability rows did not settle in 4 passes (last {guess:?})")))
}

/// `build` with its final ability scores computed from `abilities` (what every other
/// Starfinder total reads).
pub fn apply(package: &SheetRulePackage, build: &SfBuild, abilities: &SfAbilityBuild) -> Result<SfBuild, SfChassisRefusal> {
    let scores = compute(package, build, abilities)?;
    let mut out = build.clone();
    out.chassis.ability_scores = scores.finals();
    Ok(out)
}

#[cfg(test)]
mod sf_seed {
    //! The four SD-37 Starfinder seeds' ability scores against `seed-builds.md` (E0.4, reviewed):
    //! the `At 1st level` and `**Final**` rows of each seed's ability table, bytes the engine does
    //! not read. Inputs are the same file's `Points spent` rows, race/theme picks and 5th-level
    //! increases.

    use super::*;
    use crate::rules_core::pilot_compute::sf_defense::seed_support::{ability_builds, package, seeds};

    const SEED_BUILDS: &str = "docs/release/SD-37-starfinder-1e/artifacts/epic_0/seed-builds.md";

    /// `seed -> (row label -> six scores)` for the `At 1st level` and `**Final**` rows.
    fn seed_rows() -> BTreeMap<String, BTreeMap<String, [i64; 6]>> {
        let path = crate::support::paths::repo_root().join(SEED_BUILDS);
        let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        let mut out: BTreeMap<String, BTreeMap<String, [i64; 6]>> = BTreeMap::new();
        let mut seed = None;
        for line in text.lines() {
            if let Some(h) = line.strip_prefix("## ") {
                seed = h.split_whitespace().find(|w| w.starts_with("SF-")).map(str::to_string);
                continue;
            }
            let Some(seed) = &seed else { continue };
            let cells: Vec<String> = line.split('|').map(|c| c.trim().replace("**", "")).collect();
            if cells.len() < 9 || !(cells[1] == "Final" || cells[1].starts_with("Final (") || cells[1] == "At 1st level") {
                continue;
            }
            let label = if cells[1].starts_with("Final") { "Final" } else { "At 1st level" };
            let nums: Vec<i64> = cells[2..8].iter().map(|c| c.parse().unwrap_or_else(|_| panic!("{seed}: {line}"))).collect();
            out.entry(seed.clone()).or_default().insert(label.into(), nums.try_into().unwrap());
        }
        out
    }

    #[test]
    fn sf_seed_ability_scores_match_the_seed_builds() {
        let rows = seed_rows();
        let builds = ability_builds();
        let mut checked = 0;
        let mut bad = Vec::new();
        for (seed, build) in seeds() {
            let want = rows[seed]["Final"];
            let got = compute(package(), &build, &builds[seed]).unwrap_or_else(|r| panic!("{seed}: {r:?}"));
            checked += 6;
            if got.finals() != want {
                bad.push(format!("{seed}: engine {:?}, seed-builds.md {want:?}\n  {:?}", got.finals(), got.scores));
            }
            // The fixture builds every other Starfinder test reads carry the same final scores.
            assert_eq!(build.chassis.ability_scores, want, "{seed}: seed_support final scores");
        }
        assert_eq!(checked, 24, "4 seeds x 6 scores");
        assert!(bad.is_empty(), "{} of 4 seeds mismatch:\n{}", bad.len(), bad.join("\n"));
    }

    /// The 5th-level increase (Mystic 5, Technomancer 5): at level 4 the scores are the
    /// `At 1st level` row; at level 5, the `Final` row; a score already 17 or higher rises by 1.
    #[test]
    fn sf_seed_the_5th_level_increase_raises_four_scores() {
        let rows = seed_rows();
        let builds = ability_builds();
        let mut seen = 0;
        for (seed, build) in seeds() {
            let Some(first) = rows[seed].get("At 1st level").copied() else { continue };
            let abilities = &builds[seed];
            let at5 = compute(package(), &build, abilities).unwrap();
            assert_eq!(at5.at_creation, first, "{seed}: creation scores");
            assert_eq!(at5.finals(), rows[seed]["Final"], "{seed}: level 5");
            let raised: Vec<(Ability, i64)> = ABILITIES
                .into_iter()
                .enumerate()
                .filter_map(|(i, a)| at5.scores[i].terms.iter().find(|t| t.source == SRD_INCREASE).map(|t| (a, t.value)))
                .collect();
            assert_eq!(raised.len(), 4, "{seed}: four scores raised: {raised:?}");
            for (a, v) in &raised {
                let before = first[ability_index(*a)];
                assert_eq!(*v, if before >= 17 { 1 } else { 2 }, "{seed} {a:?}: {before} -> +{v}");
            }
            assert!(raised.iter().any(|(_, v)| *v == 1), "{seed}: the seed raises an 18 by 1");
            // One level lower, no increase is reached: the level-4 scores are the creation scores.
            let mut level4 = build.clone();
            level4.chassis.classes[0].1 = 4;
            let mut no_increase = abilities.clone();
            no_increase.increases.clear();
            assert_eq!(compute(package(), &level4, &no_increase).unwrap().finals(), first, "{seed}: level 4");
            // At level 5 the increase must be chosen.
            assert_eq!(compute(package(), &build, &no_increase).unwrap_err().id, REFUSED_INCREASE_NOT_CHOSEN, "{seed}");
            seen += 1;
        }
        assert_eq!(seen, 2, "Mystic 5 and Technomancer 5");
    }

    /// Race and theme terms are read from held package rows, never from a table.
    #[test]
    fn sf_seed_race_and_theme_terms_name_their_records() {
        let builds = ability_builds();
        // seed -> (ability, record id prefix) the term must come from.
        let want: [(&str, &[(Ability, &str)]); 4] = [
            ("SF-Soldier-3", &[(Ability::Str, "core:ability:2_racial_stat_bonus"), (Ability::Str, "core:ability:mercenary_theme_benefit_theme_knowledge")]),
            (
                "SF-Mystic-5",
                &[
                    (Ability::Cha, "core:race:lashunta"),
                    (Ability::Int, "core:ability:lashunta_subrace_damaya"),
                    (Ability::Con, "core:ability:lashunta_subrace_damaya"),
                    (Ability::Wis, "core:ability:priest_theme_benefit_theme_knowledge"),
                ],
            ),
            (
                "SF-Technomancer-5",
                &[
                    (Ability::Dex, "core:race:android"),
                    (Ability::Int, "core:race:android"),
                    (Ability::Cha, "core:race:android"),
                    (Ability::Int, "core:ability:scholar_theme_benefit_theme_knowledge"),
                ],
            ),
            (
                "SF-Envoy-3",
                &[
                    (Ability::Str, "core:race:ysoki"),
                    (Ability::Dex, "core:race:ysoki"),
                    (Ability::Int, "core:race:ysoki"),
                    (Ability::Cha, "core:ability:icon_theme_benefit_theme_knowledge"),
                ],
            ),
        ];
        for ((seed, build), (wseed, terms)) in seeds().into_iter().zip(want) {
            assert_eq!(seed, wseed);
            let s = compute(package(), &build, &builds[seed]).unwrap();
            for (a, prefix) in terms {
                let total = &s.scores[ability_index(*a)];
                assert!(total.terms.iter().any(|t| t.source.starts_with(prefix)), "{seed} {a:?}: no term from {prefix}: {total:?}");
            }
            for total in &s.scores {
                for t in &total.terms {
                    assert!(t.source.starts_with("core:") || t.source.starts_with("SRD "), "{seed}: {t:?}");
                }
            }
        }
    }

    /// A theme makes its skill a class skill (SRD Priest, https://www.aonsrd.com/Themes.aspx?ItemName=Priest:
    /// "Mysticism is a class skill") and raises its ability by 1, for a class that lacks the skill.
    #[test]
    fn sf_seed_a_theme_adds_its_class_skill_and_its_ability_point() {
        use crate::rules_core::pilot_compute::{sf_defense, sf_skills};
        let (seed, mercenary) = seeds().remove(0);
        let abilities = ability_builds()[seed].clone();
        let mut priest = mercenary.clone();
        priest.theme = Some("core:ability:priest".into());
        priest.skill_ranks.insert("mysticism".into(), 1);
        assert!(!sf_defense::held(package(), &mercenary).unwrap().class_skills.contains("mysticism"), "soldier: Mysticism is not a class skill");
        let held = sf_defense::held(package(), &priest).unwrap();
        assert!(held.class_skills.contains("mysticism"), "priest theme: {:?}", held.class_skills);
        let skills = sf_skills::compute(package(), &priest).unwrap();
        let mysticism = skills.skills.iter().find(|s| s.skill == "mysticism").unwrap();
        assert!(mysticism.class_skill, "{mysticism:?}");
        let m = compute(package(), &mercenary, &abilities).unwrap().finals();
        let p = compute(package(), &priest, &abilities).unwrap().finals();
        assert_eq!((p[0], p[4]), (m[0] - 1, m[4] + 1), "theme +1 moves from Str (mercenary) to Wis (priest)");
    }

    /// Point buy: 10 points, never negative, no score above 18 at creation.
    #[test]
    fn sf_seed_point_buy_limits_are_refused_by_name() {
        let (seed, soldier) = seeds().remove(0);
        let base = ability_builds()[seed].clone();
        assert_eq!(compute(package(), &soldier, &base).unwrap().points_unspent, 0);
        let mut over = base.clone();
        over.point_buy[5] += 1;
        assert_eq!(compute(package(), &soldier, &over).unwrap_err().id, REFUSED_POINT_BUY_OVER_BUDGET);
        let mut negative = base.clone();
        negative.point_buy[5] = -1;
        assert_eq!(compute(package(), &soldier, &negative).unwrap_err().id, REFUSED_POINT_BUY_NEGATIVE);
        // Str 16 at creation (race 2 + theme 1 + 3 points); 3 more points -> 19.
        let mut high = base.clone();
        high.point_buy = [6, 4, 0, 0, 0, 0];
        assert_eq!(compute(package(), &soldier, &high).unwrap_err().id, REFUSED_SCORE_OVER_18_AT_CREATION);
        let mut under = base.clone();
        under.point_buy[1] -= 2;
        let s = compute(package(), &soldier, &under).unwrap();
        assert_eq!((s.points_unspent, s.finals()[1]), (2, 12));
    }

    /// The increase rule at every level it applies, on one score crossing 17.
    #[test]
    fn sf_seed_increases_at_10_15_20_follow_the_17_rule() {
        let (seed, mut soldier) = seeds().remove(0);
        let mut abilities = ability_builds()[seed].clone();
        soldier.chassis.classes[0].1 = 20;
        let four = vec![Ability::Str, Ability::Dex, Ability::Con, Ability::Int];
        for at in INCREASE_LEVELS {
            abilities.increases.insert(at, four.clone());
        }
        // Str 16 -> 18 (+2) -> 19 (+1) -> 20 (+1) -> 21 (+1); Dex 14 -> 16 -> 18 -> 19 -> 20.
        let s = compute(package(), &soldier, &abilities).unwrap();
        assert_eq!(&s.finals()[..4], &[21, 20, 19, 18]);
        let mut twice = abilities.clone();
        twice.increases.insert(10, vec![Ability::Str, Ability::Str, Ability::Con, Ability::Int]);
        assert_eq!(compute(package(), &soldier, &twice).unwrap_err().id, REFUSED_INCREASE_NOT_FOUR_DIFFERENT);
        let mut early = abilities.clone();
        soldier.chassis.classes[0].1 = 9;
        early.increases.retain(|at, _| *at <= 10);
        assert_eq!(compute(package(), &soldier, &early).unwrap_err().id, REFUSED_INCREASE_LEVEL);
    }
}
