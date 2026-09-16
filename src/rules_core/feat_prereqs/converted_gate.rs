//! Reading a CONVERTED `applies` gate as a three-outcome prerequisite verdict — SD-35
//! `AT-35-E6-001` (`epic-breakdown.md` `### AT-35-E6-001`; `decisions.md` §1, §11).
//!
//! # What this replaces
//!
//! Two live modules used to answer "does this character meet this record's prerequisites?"
//! by parsing the ingest format's `PRE*` token text at run time and evaluating it against a
//! hand-modelled fact snapshot: `feat_prereqs` for the 690-record feat catalog and
//! `pilot_compute::prestige_class_entry_gate` for the 74 census'd prestige classes. Both
//! read the ingest format on the live side, which `decisions.md` §11 rules out: the source
//! format is a converter input and a test oracle, nothing more.
//!
//! Conversion already happened. `sheet_rule_convert` writes every record's `PRE*` rows as an
//! [`Applies`] tree, and [`evaluate_applies`] — the evaluator the sheet itself renders
//! through — decides it. This module is the shared reading of that decision as the SAME
//! three outcomes the token evaluator produced, so neither consumer's contract changes.
//!
//! # Why three outcomes, still
//!
//! Collapsing them is how a checker starts lying. `Unmet` is the only verdict that makes a
//! record unavailable. `Unverified` is the honest record of a term this engine cannot decide,
//! and it never blocks — a character is never denied a feat over a fact the engine does not
//! hold. `Met` is what the character does satisfy, so a picker can say why a record IS
//! available rather than only why it is not.
//!
//! # Which terms cannot be decided, and why that list is derived rather than chosen
//!
//! [`CharacterFacts::from_character`](crate::rules_core::sheet_rule::CharacterFacts::from_character)
//! states in its own doc which facts a character record does not carry: alignment, deity,
//! gender, age category, languages, class skills and proficiencies. Its body leaves five more
//! at a hard zero for every character it builds — hit dice, challenge rating, highest spell
//! level, the master's level, and (absent a race trait bundle emitting one) speeds. A gate over
//! any of those evaluates `Exclude` for a reason that has nothing to do with the character, so
//! reporting it `Unmet` would deny a build over an unbuilt fact. [`unverifiable_reason`] names
//! each one; every other term is genuinely decided.
//!
//! `Expr::Var` is NOT on that list: a corpus variable folds through the package's own
//! [`VarTable`](crate::rules_core::sheet_rule::VarTable) contributions, which is a real
//! evaluation over what the character actually holds. A variable the package carries no table
//! for cannot fold, and only that case is unverifiable.

use crate::rules_core::level_up_option_filter::{describe_gate, unmet_words};
use crate::rules_core::sheet_rule::{
    evaluate_applies, evaluate_expr_from_facts, Applies, CharacterFacts, EvalContext, Expr, Gate,
    HeldSet, Holdable, SheetRulePackage,
};

/// One top-level prerequisite term's verdict, in the rule's own words.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TermVerdict {
    /// The character satisfies it.
    Met(String),
    /// The character definitively does not. The only verdict that blocks.
    Unmet(String),
    /// This engine cannot decide it; the words carry the reason. Never blocks.
    Unverified(String),
    /// A condition the player resolves at the table, not a gate the engine refuses.
    Informational(String),
}

impl TermVerdict {
    pub fn blocks(&self) -> bool {
        matches!(self, TermVerdict::Unmet(_))
    }
}

/// Every top-level term of `gate`, each with its verdict for this character.
///
/// `Applies::All` is flattened one level so a report names the one missing prerequisite
/// rather than restating the whole tree — the same descent [`unmet_words`] makes. A nested
/// `All` inside an `AtLeast` stays whole: "at least 1 of ..." is one requirement, not several.
pub fn verdicts(
    package: &SheetRulePackage,
    held: &HeldSet,
    facts: &CharacterFacts,
    gate: &Applies,
) -> Vec<TermVerdict> {
    let mut out = Vec::new();
    for term in top_level_terms(gate) {
        match verdict_for(package, held, facts, term) {
            Some(v) => out.push(v),
            None => continue,
        }
    }
    out
}

/// One gate in the rule's own words -- the shared renderer, re-exported so a consumer needs
/// only this module.
pub fn describe(package: &SheetRulePackage, gate: &Applies) -> String {
    describe_gate(package, gate)
}

fn top_level_terms(gate: &Applies) -> Vec<&Applies> {
    match gate {
        Applies::All(terms) => terms.iter().collect(),
        other => vec![other],
    }
}

/// `None` for a term that states no requirement at all (`Applies::Always`) — the corpus
/// saying the record has no prerequisite, which is not a clause to report.
fn verdict_for(
    package: &SheetRulePackage,
    held: &HeldSet,
    facts: &CharacterFacts,
    term: &Applies,
) -> Option<TermVerdict> {
    if matches!(term, Applies::Always) {
        return None;
    }
    if let Applies::Situational { text } = term {
        return Some(TermVerdict::Informational(text.clone()));
    }
    let words = describe_gate(package, term);
    if let Some(reason) = unverifiable_reason(package, term) {
        return Some(TermVerdict::Unverified(format!("not verified: {reason} ({words})")));
    }
    match evaluate_applies(term, held, package, facts, EvalContext::default()) {
        Gate::Include => Some(TermVerdict::Met(words)),
        Gate::Situational(condition) => Some(TermVerdict::Informational(condition)),
        Gate::Exclude => {
            let mut reason = unmet_words(package, held, facts, term);
            if let Some(actual) = actual_value_note(facts, term) {
                reason.push_str(&actual);
            }
            Some(TermVerdict::Unmet(reason))
        }
    }
}

/// `" (this character: 1)"` for a failing comparison whose left side reads a fact the engine
/// genuinely holds — a refusal a player cannot act on is as bad as no refusal at all.
///
/// Deliberately narrow: only a top-level `Compare` whose left side is one plain fact leaf.
/// Anything else (a variable fold, an arithmetic tree, a nested gate) either needs the package
/// to evaluate — which [`evaluate_expr_from_facts`] deliberately does not have — or would print
/// an intermediate number the player never sees on their sheet.
fn actual_value_note(facts: &CharacterFacts, term: &Applies) -> Option<String> {
    let Applies::Compare { lhs, .. } = term else { return None };
    if !is_plain_fact_leaf(lhs) {
        return None;
    }
    let whole = evaluate_expr_from_facts(lhs, facts).trunc();
    Some(format!(" (this character: {whole})"))
}

fn is_plain_fact_leaf(e: &Expr) -> bool {
    matches!(
        e,
        Expr::Level
            | Expr::ClassLevel(_)
            | Expr::AbilityMod(_)
            | Expr::AbilityScore(_)
            | Expr::BaseAttack
            | Expr::SkillRanks(_)
            | Expr::SkillTotal(_)
            | Expr::Size
            | Expr::BaseSize
    )
}

/// The fact this term needs and the character record does not carry, or `None` when every
/// leaf it reads is one the engine genuinely holds.
///
/// Each arm names a fact by the doc comment on
/// [`CharacterFacts::from_character`](crate::rules_core::sheet_rule::CharacterFacts::from_character)
/// or by that function's body leaving the field at its default; none is a judgment call about
/// which rules matter.
pub fn unverifiable_reason(package: &SheetRulePackage, term: &Applies) -> Option<&'static str> {
    match term {
        Applies::Always | Applies::Never | Applies::Situational { .. } => None,
        Applies::All(terms) | Applies::AtLeast { of: terms, .. } => {
            terms.iter().find_map(|t| unverifiable_reason(package, t))
        }
        Applies::Not(inner) => unverifiable_reason(package, inner),
        Applies::Compare { lhs, rhs, .. } => {
            expr_unverifiable_reason(package, lhs).or_else(|| expr_unverifiable_reason(package, rhs))
        }
        Applies::Chosen { .. } => None,
        Applies::ItemHas { .. } => Some("an item's own tags are not a character fact"),
        Applies::Holds { what, .. } => holdable_unverifiable_reason(what),
    }
}

/// The pools a character record genuinely rosters: the player's own selections. A
/// `Holds` counting members of one of these is a real verdict.
///
/// The "special ability" pool -- class features counted by name, `PREABILITY:1,CATEGORY=
/// Special Ability,Channel Energy` in the source -- is deliberately NOT here, and the reason
/// is measured, not assumed. The held set grows class features through the package's own
/// grant edges, which works for a Barbarian's Rage, a Bard's Bardic Performance and a
/// Paladin's Lay on Hands (each verified by
/// `class_feature_pool_holdings_that_the_grant_fixpoint_really_reaches`) but NOT for a
/// Cleric's Channel Positive Energy, whose grant is conditioned on an alignment the
/// character record does not carry. A partial roster cannot produce an honest "you do not
/// hold it", and this path REFUSES a save rather than only greying a picker row. The token
/// evaluator this replaced drew the same line in its own words -- "only CATEGORY=FEAT
/// prerequisites are modelled; this one names a class feature, archetype or class ability
/// the character record has no roster of" -- so the contract is preserved deliberately
/// rather than tightened by accident.
///
/// A `Holdable::Rule` naming ONE concrete record is not affected: those the fixpoint either
/// grants or does not, and a race's own `IsTiefling`-shaped ability record is reached
/// through the race the character actually chose.
const ROSTERED_POOLS: &[&str] = &["feat", "trait", "spell", "equipment", "skill"];

fn holdable_unverifiable_reason(what: &Holdable) -> Option<&'static str> {
    match what {
        Holdable::Alignment(_) | Holdable::AlignmentMatchesDeity => {
            Some("the character record carries no alignment")
        }
        Holdable::Deity(_) | Holdable::DeityInPantheon(_) | Holdable::DeityGrantsDomain(_) => {
            Some("the character record carries no deity")
        }
        Holdable::DeityAlignment(_) => Some("the character record carries no deity"),
        Holdable::ClassSkill(_) => Some("the character record carries no class-skill list"),
        Holdable::Proficiency(_) => Some("proficiency is not modelled"),
        Holdable::Language(_) => Some("the character record carries no languages"),
        Holdable::Movement { .. } => Some("movement-mode prerequisites are not modelled"),
        Holdable::Vision(_) => Some("vision modes are not modelled"),
        Holdable::ClassTag(_) => Some("the character record carries no class type tags"),
        Holdable::Gender(_) => Some("the character record carries no gender"),
        Holdable::AgeCategory(_) => Some("the character record carries no age category"),
        Holdable::RaceType(_) | Holdable::RaceSubtype(_) => {
            Some("the character record carries no creature type")
        }
        Holdable::RuleTag { pool, .. } => {
            if ROSTERED_POOLS.contains(&pool.as_str()) {
                None
            } else {
                Some("the character record carries no roster of this pool")
            }
        }
        // The converter resolved this requirement to no corpus record at all (it is listed
        // in `data/sheet_rules/_defects/`). That is a statement about this repo's ingest, not
        // about the character, so it reports rather than refuses.
        Holdable::MissingRule { .. } => Some("names a record the corpus does not carry"),
        Holdable::Template(_) => Some("the character record carries no applied templates"),
        Holdable::Rule(_) | Holdable::Spell(_) | Holdable::Race(_) | Holdable::Fact { .. } => {
            None
        }
    }
}

fn expr_unverifiable_reason(package: &SheetRulePackage, e: &Expr) -> Option<&'static str> {
    match e {
        Expr::HitDice => Some("the character record carries no hit dice"),
        Expr::ChallengeRating => Some("a player character has no challenge rating"),
        Expr::HighestSpellLevel(_) => Some("spellcasting prerequisites are not modelled"),
        Expr::CasterLevel(_) => Some("spellcasting prerequisites are not modelled"),
        Expr::MasterLevel | Expr::MasterVar(_) => Some("this character has no master"),
        Expr::Speed(_) => Some("movement-mode prerequisites are not modelled"),
        Expr::BaseSave(_) => Some("base saving-throw prerequisites are not modelled"),
        // A variable folds through the package's own contributions -- a real evaluation over
        // what the character holds. Only a variable the package carries no table for is
        // undecidable.
        Expr::Var(v) => {
            if package.vars.contains_key(v) {
                None
            } else {
                Some("the referenced rules variable has no converted table")
            }
        }
        Expr::Sum(terms) => terms.iter().find_map(|t| expr_unverifiable_reason(package, t)),
        Expr::Mul(a, b) | Expr::Div(a, b) | Expr::Min(a, b) | Expr::Max(a, b) => {
            expr_unverifiable_reason(package, a).or_else(|| expr_unverifiable_reason(package, b))
        }
        Expr::Floor(inner) | Expr::Ceil(inner) => expr_unverifiable_reason(package, inner),
        Expr::HeldCount { .. }
        | Expr::Const(_)
        | Expr::Level
        | Expr::ClassLevel(_)
        | Expr::SpellLevel
        | Expr::AbilityMod(_)
        | Expr::AbilityScore(_)
        | Expr::Size
        | Expr::BaseSize
        | Expr::SizeMod
        | Expr::BaseAttack
        | Expr::SkillRanks(_)
        | Expr::SkillTotal(_)
        | Expr::Choice(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::sheet_rule::{Ability, Cmp};

    fn facts_with_base_attack(base_attack: i64) -> CharacterFacts {
        CharacterFacts { level: 1, base_attack, ..CharacterFacts::default() }
    }

    #[test]
    fn a_failing_comparison_names_the_requirement_and_the_characters_own_value() {
        let package = SheetRulePackage::new();
        let held = HeldSet::default();
        let gate = Applies::Compare {
            lhs: Expr::BaseAttack,
            op: Cmp::Gte,
            rhs: Expr::Const(6),
        };
        let out = verdicts(&package, &held, &facts_with_base_attack(1), &gate);
        assert_eq!(out.len(), 1);
        let TermVerdict::Unmet(reason) = &out[0] else { panic!("expected Unmet, got {out:?}") };
        assert!(reason.contains("base attack bonus at least 6"), "{reason}");
        assert!(reason.contains("this character: 1"), "{reason}");
    }

    #[test]
    fn the_same_comparison_is_met_when_the_character_meets_it() {
        let package = SheetRulePackage::new();
        let held = HeldSet::default();
        let gate = Applies::Compare {
            lhs: Expr::BaseAttack,
            op: Cmp::Gte,
            rhs: Expr::Const(6),
        };
        let out = verdicts(&package, &held, &facts_with_base_attack(6), &gate);
        assert!(matches!(out.as_slice(), [TermVerdict::Met(_)]), "{out:?}");
        assert!(!out[0].blocks());
    }

    /// The property the three-outcome design exists for: a fact the engine does not hold
    /// never denies a character. `CharacterFacts::from_character` leaves alignment `None`
    /// for every character it builds, so an alignment gate must report, not refuse.
    #[test]
    fn a_fact_the_character_record_does_not_carry_is_unverified_and_never_blocks() {
        let package = SheetRulePackage::new();
        let held = HeldSet::default();
        let gate = Applies::Holds {
            what: Holdable::Alignment(vec!["LG".to_owned()]),
            count: 1,
        };
        let out = verdicts(&package, &held, &facts_with_base_attack(0), &gate);
        let TermVerdict::Unverified(note) = &out[0] else { panic!("expected Unverified: {out:?}") };
        assert!(note.contains("carries no alignment"), "{note}");
        assert!(!out[0].blocks());
    }

    /// `evaluate_applies` would return `Exclude` for every one of these, so each is a live
    /// false refusal if it is not classified. Enumerated rather than sampled.
    #[test]
    fn every_fact_the_engine_does_not_hold_is_classified_unverifiable() {
        let package = SheetRulePackage::new();
        let unheld: Vec<Applies> = vec![
            Applies::Compare { lhs: Expr::HitDice, op: Cmp::Gte, rhs: Expr::Const(4) },
            Applies::Compare {
                lhs: Expr::ChallengeRating,
                op: Cmp::Gte,
                rhs: Expr::Const(1),
            },
            Applies::Compare {
                lhs: Expr::HighestSpellLevel(crate::rules_core::sheet_rule::SpellKind::Arcane),
                op: Cmp::Gte,
                rhs: Expr::Const(1),
            },
            Applies::Compare { lhs: Expr::MasterLevel, op: Cmp::Gte, rhs: Expr::Const(1) },
            Applies::Compare {
                lhs: Expr::BaseSave(crate::rules_core::sheet_rule::Save::Fortitude),
                op: Cmp::Gte,
                rhs: Expr::Const(5),
            },
            Applies::Compare {
                lhs: Expr::Speed("Walk".to_owned()),
                op: Cmp::Gte,
                rhs: Expr::Const(30),
            },
            Applies::Holds { what: Holdable::Deity(crate::rules_core::sheet_rule::DeityRef::Any), count: 1 },
            Applies::Holds { what: Holdable::Language("Draconic".to_owned()), count: 1 },
            Applies::Holds { what: Holdable::ClassSkill("stealth".to_owned()), count: 1 },
            Applies::Holds {
                what: Holdable::Proficiency(crate::rules_core::sheet_rule::ProfRef::Weapon(
                    "longbow".to_owned(),
                )),
                count: 1,
            },
            Applies::Holds { what: Holdable::Vision("Darkvision".to_owned()), count: 1 },
            Applies::Holds { what: Holdable::Movement { mode: "Fly".to_owned(), min: 30 }, count: 1 },
            Applies::Holds { what: Holdable::ClassTag("Prestige".to_owned()), count: 1 },
            Applies::Holds { what: Holdable::Gender("Female".to_owned()), count: 1 },
            Applies::Holds { what: Holdable::AgeCategory("Venerable".to_owned()), count: 1 },
            Applies::Holds { what: Holdable::RaceType("Humanoid".to_owned()), count: 1 },
            Applies::Holds { what: Holdable::RaceSubtype("Elf".to_owned()), count: 1 },
            Applies::ItemHas { tags: vec!["Heavy".to_owned()], n: 1 },
        ];
        for term in &unheld {
            assert!(
                unverifiable_reason(&package, term).is_some(),
                "{term:?} reads a fact no character record carries and must never refuse a build"
            );
        }
    }

    /// The inverse: a fact the engine DOES hold stays decidable, or this classification would
    /// quietly stop enforcing every prerequisite in the corpus.
    #[test]
    fn the_facts_the_engine_does_hold_stay_decidable() {
        let package = SheetRulePackage::new();
        let decidable: Vec<Applies> = vec![
            Applies::Compare { lhs: Expr::BaseAttack, op: Cmp::Gte, rhs: Expr::Const(6) },
            Applies::Compare { lhs: Expr::Level, op: Cmp::Gte, rhs: Expr::Const(3) },
            Applies::Compare {
                lhs: Expr::AbilityScore(Ability::Dex),
                op: Cmp::Gte,
                rhs: Expr::Const(17),
            },
            Applies::Compare {
                lhs: Expr::SkillRanks("stealth".to_owned()),
                op: Cmp::Gte,
                rhs: Expr::Const(5),
            },
            Applies::Compare {
                lhs: Expr::ClassLevel("fighter".to_owned()),
                op: Cmp::Gte,
                rhs: Expr::Const(2),
            },
            Applies::Holds { what: Holdable::Rule("core_rulebook:feat:dodge".to_owned()), count: 1 },
            Applies::Holds { what: Holdable::Race("human".to_owned()), count: 1 },
        ];
        for term in &decidable {
            assert!(
                unverifiable_reason(&package, term).is_none(),
                "{term:?} reads a fact the engine holds and must still be enforced"
            );
        }
    }
}
