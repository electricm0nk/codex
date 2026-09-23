//! SD-36 Epic F1c-2 (defect D2, `epic-f-class-completion.md` §3 F1, review finding 1): a
//! condition that belongs to ONE line of a record gates only that line, never the whole record.
//!
//! The shape the F1 reader exposed: `core_rulebook:class_feature:fighter_class`
//! (`cr_abilities_class.lst:102`) holds the fighter's whole class-feature closure -- every
//! proficiency grant, every class feature -- and ALSO one line
//! `BONUS:ABILITYPOOL|Weapon Mastery|1|PREVAREQ:...|PREVARGTEQ:Fighter_CFP_Level,20`. The converter
//! made that bonus the record's principal line and folded the line's own PRE tokens into the
//! principal rule's `applies`; the principal rule is what the held-set fixpoint admits and what
//! every outgoing grant edge (`Granter::Rule(fighter_class)`) hangs off, so the level-20 condition
//! shut the whole closure at levels 1-19. Medium, mesmerist and spiritualist carry the same shape
//! (`BONUS:SPELLCAST|...|PREMULT:2,[PREVAREQ:<class>_CF_Knacks,1],[PRECLASS:1,<class>=1]`).
//!
//! These tests read a FRESH conversion of the named record (`sheet_rule::convert_one`, the same
//! function `sheet_rule_convert --one` prints), overlaid onto the on-disk package, through the F1
//! reader (`class_proficiency_sheet_rules::class_weapon_proficiency_view_in`) -- so they hold the
//! converter itself to the contract, independent of whether `data/sheet_rules` was regenerated.

use std::sync::OnceLock;

use codex::rules_core::pilot_compute::class_proficiency_sheet_rules::{class_weapon_proficiency_view_in, ProficiencyAnswer};
use codex::rules_core::rules_tables::crb::weapon_tables::WeaponProficiency;
use codex::rules_core::sheet_rule::{Applies, BonusTarget, Cmp, Expr, SheetRule, SheetRulePackage};
use codex::rules_core::sheet_rule_package::load_package_from;
use codex_ingest::pcgen_import::sheet_rule::convert_one;

/// The on-disk package with each named record's rules replaced by a fresh `convert_one` of it.
/// Incoming grant edges (`granted_by`, attached to a principal from OTHER records' conversions)
/// are carried over from the on-disk principal, since one record's conversion does not carry them.
fn overlaid(record_ids: &[&str]) -> (SheetRulePackage, Vec<SheetRule>) {
    let repo = codex_ingest::repo_root();
    let mut package = load_package_from(&repo).expect("package loads");
    let mut fresh_all = Vec::new();
    for record_id in record_ids {
        let (conv, _) = convert_one(&repo, record_id).unwrap_or_else(|e| panic!("convert_one {record_id}: {e}"));
        let incoming = package.rule(record_id).map(|r| r.granted_by.clone()).unwrap_or_default();
        let sibling_prefix = format!("{record_id}#");
        package.rules.retain(|id, _| id != record_id && !id.starts_with(&sibling_prefix));
        for mut rule in conv.rules {
            if rule.id == *record_id {
                for g in &incoming {
                    if !rule.granted_by.contains(g) {
                        rule.granted_by.push(g.clone());
                    }
                }
            }
            fresh_all.push(rule.clone());
            package.insert_rule(rule);
        }
    }
    package.finish();
    (package, fresh_all)
}

fn fighter() -> &'static (SheetRulePackage, Vec<SheetRule>) {
    static P: OnceLock<(SheetRulePackage, Vec<SheetRule>)> = OnceLock::new();
    P.get_or_init(|| overlaid(&["core_rulebook:class_feature:fighter_class"]))
}

fn tiers(package: &SheetRulePackage, class: &str, level: u8) -> Vec<WeaponProficiency> {
    match class_weapon_proficiency_view_in(package, class, level) {
        ProficiencyAnswer::Known(view) => view.tiers.clone(),
        ProficiencyAnswer::Unknown { reason } => panic!("{class} L{level} must be Known, reader said Unknown: {reason}"),
    }
}

fn names_level_at_least(a: &Applies, n: i32) -> bool {
    match a {
        Applies::Compare { op: Cmp::Gte, rhs: Expr::Const(k), .. } => *k >= n,
        Applies::All(terms) | Applies::AtLeast { of: terms, .. } => terms.iter().any(|t| names_level_at_least(t, n)),
        Applies::Not(inner) => names_level_at_least(inner, n),
        _ => false,
    }
}

/// Oracle: `Weapon and Armor Proficiency ~ Fighter` -- simple and martial weapons at level 1
/// (`cr_abilities_class.lst:102`, `Weapon Prof ~ Martial|Weapon Prof ~ Simple` gated only on
/// `PREVARGTEQ:Fighter_CFP_Level,1` and the archetype-off test).
#[test]
fn fighter_proficiencies_are_ungated_at_level_1() {
    let (package, _) = fighter();
    let t = tiers(package, "fighter", 1);
    for tier in [WeaponProficiency::Simple, WeaponProficiency::Martial] {
        assert!(t.contains(&tier), "fighter L1 must hold {tier:?}; reader tiers {t:?}");
    }
}

/// The Weapon Mastery pool line keeps its own level-20 condition: the fix moves the condition
/// off the record, never drops it.
#[test]
fn weapon_mastery_stays_gated_at_20() {
    let (package, fresh) = fighter();
    let principal = fresh.iter().find(|r| r.id == "core_rulebook:class_feature:fighter_class").expect("principal rule");
    assert!(
        !names_level_at_least(&principal.applies, 20),
        "the principal fighter_class rule must not carry the level-20 Weapon Mastery condition: {:?}",
        principal.applies
    );
    let mastery: Vec<&SheetRule> =
        fresh.iter().filter(|r| r.target == Some(BonusTarget::Pool("weapon_mastery".into()))).collect();
    assert_eq!(mastery.len(), 1, "exactly one Weapon Mastery pool line: {:?}", fresh.iter().map(|r| &r.id).collect::<Vec<_>>());
    assert!(mastery[0].id.starts_with("core_rulebook:class_feature:fighter_class#"), "the pool line is a sibling line: {}", mastery[0].id);
    assert!(names_level_at_least(&mastery[0].applies, 20), "the pool line keeps its level-20 gate: {:?}", mastery[0].applies);
    // The reader still answers at 20: moving the condition shuts nothing that was open.
    let t = tiers(package, "fighter", 20);
    assert!(t.contains(&WeaponProficiency::Martial), "fighter L20 must hold Martial; reader tiers {t:?}");
}

/// Oracle: the three occult classes' `<Class> ~ Weapon and Armor Proficiency` records grant the
/// Simple tier at level 1; the `BONUS:SPELLCAST` knacks line's `PREMULT` belongs to that line only.
#[test]
fn medium_mesmerist_spiritualist_proficiencies_are_ungated_at_level_1() {
    let (package, _) = overlaid(&[
        "occult_adventures:class_feature:medium_class",
        "occult_adventures:class_feature:mesmerist_class",
        "occult_adventures:class_feature:spiritualist_class",
    ]);
    for class in ["medium", "mesmerist", "spiritualist"] {
        let t = tiers(&package, class, 1);
        assert!(t.contains(&WeaponProficiency::Simple), "{class} L1 must hold Simple; reader tiers {t:?}");
    }
}

/// The whole-package form of the contract, over the generated `data/sheet_rules`: every line of a
/// record is converted as `record gates AND line gate`, so the record's own gates are the
/// conjuncts every one of its rules shares. A principal conjunct that some sibling line lacks is
/// one line's condition hoisted onto the record. Before F1c-2, 749 multi-line records carried one
/// (434 of them handing out a grant, an offer or a grant edge); the fix leaves none.
#[test]
fn no_principal_carries_one_lines_condition() {
    fn conjuncts(a: &Applies) -> Vec<&Applies> {
        match a {
            Applies::Always => Vec::new(),
            Applies::All(terms) => terms.iter().collect(),
            other => vec![other],
        }
    }
    let package = codex::rules_core::sheet_rule_package::package().as_ref().expect("package loads");
    let mut hoisted = Vec::new();
    for principal in package.rules.values().filter(|r| !r.id.contains('#')) {
        let prefix = format!("{}#", principal.id);
        let siblings: Vec<&SheetRule> = package.rules.range(prefix.clone()..).take_while(|(id, _)| id.starts_with(&prefix)).map(|(_, r)| r).collect();
        let lost = conjuncts(&principal.applies)
            .into_iter()
            .filter(|c| siblings.iter().any(|s| !conjuncts(&s.applies).contains(c)))
            .count();
        if lost > 0 {
            hoisted.push(principal.id.clone());
        }
    }
    assert!(hoisted.is_empty(), "{} principal rules carry a condition some sibling line does not: {:?}", hoisted.len(), &hoisted[..hoisted.len().min(20)]);
}
