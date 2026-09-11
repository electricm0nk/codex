//! Catalog-mode prose: a converted rule's words with **no character in hand**.
//!
//! # Why this module exists
//!
//! SD-35 `decisions.md §11` — "there should be nothing left of pcgen" on the live side. The
//! desktop catalog screens (Feat Catalog, Spell Catalog, Monster Catalog, Equipment Catalog,
//! the companion and intelligent-item catalogs, the reference library) each served a record's
//! description by parsing the ingest format's own description token at run time. The
//! substitution those call sites performed already happens at ingest, in
//! `src/pcgen_import/sheet_rule/`: the converted [`SheetRule::prose`] carries the record's
//! words with **typed** holes ([`ProsePiece::Slot`] over an [`Expr`]) rather than the source
//! format's positional placeholders and argument tail.
//!
//! # Why the sheet evaluator is not enough
//!
//! [`crate::rules_core::sheet_rule::evaluate`] renders a rule **for a character**: a
//! [`ProsePiece::Slot`] evaluates to a number. A catalog screen has no character, and the
//! evaluator's empty-facts answer for `CasterLevel`/`Level`/`AbilityMod` is `0` — so
//! Advanced Race Guide's *Absorbing Inhalation* would read *"for up to 0 rounds"*. A wrong
//! computed number looks right, which is the one failure mode a presence gate never catches
//! (`docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003_cycle1_receipt.md`).
//!
//! `decisions.md §1` already rules what to print instead: **a final number, dice in final
//! form, or the rule's words.** A slot the catalog cannot settle is not a number — it is the
//! rule's words. So this module renders the same prose the sheet renders, and substitutes
//! each unsettled slot with the words for the term it stands on: *"for up to your caster
//! level rounds"*. A slot whose expression is a constant still prints its number, because
//! that number **is** settled.
//!
//! # What is NOT in here
//!
//! No token, no formula string, no argument tail, no ingest-format vocabulary. The input is a
//! converted [`SheetRule`]; the output is English.

use crate::rules_core::level_up_option_filter::{describe_gate, expr_words, words_of_id};
use crate::rules_core::sheet_rule::{
    fold_dice_modifier, slug, Expr, ProseFamily, ProsePiece, ProseSegment, SheetRule,
    SheetRulePackage,
};
use std::collections::BTreeMap;

/// The families a catalog screen's `description` field is allowed to carry: the record's own
/// descriptive text, not its stat block. `Desc`, `Benefit` and `Special` are the three the
/// converter writes from a record's authored prose; `Aspect`/`StatBlock`/`WhenActive` are the
/// structured lines the sheet prints beside the description (a spell's casting time, a
/// companion's advancement aspect), and a catalog row carries those in its own typed fields.
const DESCRIPTION_FAMILIES: [&str; 3] = ["Desc", "Benefit", "Special"];

/// `rule.prose` rendered with no character: every family, in the sheet's own order.
pub fn catalog_prose(package: &SheetRulePackage, rule: &SheetRule) -> String {
    render(package, &rule.prose, None)
}

/// The description a catalog row serves: the `Desc`/`Benefit`/`Special` families only.
///
/// `None` when the record states no descriptive prose at all — the honest answer for the
/// template and bookkeeping rows that carry a stat block and nothing else. Never a fabricated
/// or partial sentence.
pub fn catalog_description(package: &SheetRulePackage, rule: &SheetRule) -> Option<String> {
    let text = render(package, &rule.prose, Some(&DESCRIPTION_FAMILIES));
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

/// [`catalog_description`] for the rule of `kind` whose name slugs to `name`'s slug.
///
/// The lookup is [`SheetRulePackage::find`]'s — `core_rulebook` first, then the
/// lexicographically first book id — so a name several printings share resolves to one
/// record, deterministically, the same way the sheet resolves it.
pub fn catalog_description_by_name(
    package: &SheetRulePackage,
    kind: &str,
    name: &str,
) -> Option<String> {
    let id = package.find(kind, &slug(name))?;
    catalog_description(package, package.rule(id)?)
}

/// [`catalog_prose`] for the rule of `kind` whose name slugs to `name`'s slug.
pub fn catalog_prose_by_name(
    package: &SheetRulePackage,
    kind: &str,
    name: &str,
) -> Option<String> {
    let id = package.find(kind, &slug(name))?;
    Some(catalog_prose(package, package.rule(id)?))
}

fn family_key(family: &ProseFamily) -> String {
    format!("{family:?}")
}

fn family_name(family: &ProseFamily) -> &'static str {
    match family {
        ProseFamily::Desc => "Desc",
        ProseFamily::Benefit => "Benefit",
        ProseFamily::Special => "Special",
        ProseFamily::Aspect(_) => "Aspect",
        ProseFamily::StatBlock(_) => "StatBlock",
        ProseFamily::WhenActive => "WhenActive",
    }
}

fn rank(family: &ProseFamily) -> i32 {
    match family {
        ProseFamily::Desc => 0,
        ProseFamily::Benefit => 1,
        ProseFamily::Special => 2,
        ProseFamily::Aspect(_) => 3,
        ProseFamily::StatBlock(_) => 4,
        ProseFamily::WhenActive => 5,
    }
}

fn render(
    package: &SheetRulePackage,
    segments: &[ProseSegment],
    families: Option<&[&str]>,
) -> String {
    let included: Vec<bool> = segments
        .iter()
        .map(|s| families.is_none_or(|f| f.contains(&family_name(&s.family))))
        .collect();
    // `pick_last` means "print only the LAST segment of this family whose gate passes". With no
    // character no gate is evaluated, so every gate is treated as passing and the last segment
    // of the family is the one that prints -- the same segment the evaluator picks for a
    // character who meets every condition.
    let mut last_pick: BTreeMap<String, usize> = BTreeMap::new();
    for (i, s) in segments.iter().enumerate() {
        if s.pick_last && included[i] {
            last_pick.insert(family_key(&s.family), i);
        }
    }

    let mut lines: Vec<(i32, usize, String)> = Vec::new();
    for (i, s) in segments.iter().enumerate() {
        if !included[i] {
            continue;
        }
        if s.pick_last && last_pick.get(&family_key(&s.family)) != Some(&i) {
            continue;
        }
        let mut text = String::new();
        let mut slots = 0usize;
        let mut nonzero = false;
        for p in &s.pieces {
            match p {
                ProsePiece::Text(t) => text.push_str(t),
                ProsePiece::Slot(e) => {
                    slots += 1;
                    match const_value(e) {
                        Some(n) => {
                            nonzero |= n != 0;
                            text.push_str(&n.to_string());
                        }
                        None => {
                            // Not settled without a character: the rule's words, never `0`.
                            nonzero = true;
                            text.push_str(&expr_words(package, e));
                        }
                    }
                }
                ProsePiece::ChoiceName(id) => {
                    text.push_str(&format!("your choice of {}", words_of_id(id)));
                }
                ProsePiece::Dice { dice, modifier } => match modifier.as_ref().map(const_value) {
                    None => text.push_str(dice),
                    Some(Some(n)) => text.push_str(&fold_dice_modifier(dice, n)),
                    Some(None) => {
                        let words = expr_words(package, modifier.as_ref().expect("matched Some"));
                        text.push_str(&format!("{dice} plus {words}"));
                    }
                },
            }
        }
        // The evaluator drops a line whose every slot came out zero. Here a slot is only known
        // to be zero when it is a constant zero; a slot standing on a character term is words,
        // and words are never suppressed.
        if s.suppress_when_all_zero && slots > 0 && !nonzero {
            continue;
        }
        let text = text.trim().to_string();
        if text.is_empty() {
            continue;
        }
        let text = match &s.applies {
            Some(gate) => {
                let condition = describe_gate(package, gate);
                if condition.is_empty() || condition == "no prerequisite" {
                    text
                } else {
                    format!("If {condition}: {text}")
                }
            }
            None => text,
        };
        let line = match &s.family {
            ProseFamily::Aspect(label) | ProseFamily::StatBlock(label) => format!("{label}: {text}"),
            ProseFamily::WhenActive => format!("When active: {text}"),
            _ => text,
        };
        if !lines.iter().any(|(_, _, l)| *l == line) {
            lines.push((rank(&s.family), i, line));
        }
    }
    lines.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    lines.into_iter().map(|(_, _, l)| l).collect::<Vec<_>>().join("\n")
}

/// The expression's value when **nothing about a character** is needed to know it.
///
/// Structural, not an evaluation against empty facts: `Expr::Level` against empty facts is `0`,
/// which is a wrong number wearing a right number's clothes. Only the arithmetic whose leaves
/// are all `Const` folds; every other leaf returns `None` and renders as words.
///
/// Division and the rounding wrappers follow the evaluator's own rule -- exact rational
/// arithmetic with ONE truncation at the boundary -- by folding in `i64` and refusing a
/// division that is not exact, so a folded constant can never disagree with the number the
/// sheet would print.
fn const_value(e: &Expr) -> Option<i64> {
    match e {
        Expr::Const(n) => Some(i64::from(*n)),
        Expr::Sum(terms) => terms.iter().try_fold(0i64, |acc, t| acc.checked_add(const_value(t)?)),
        Expr::Mul(a, b) => const_value(a)?.checked_mul(const_value(b)?),
        Expr::Div(a, b) => {
            let (a, b) = (const_value(a)?, const_value(b)?);
            if b != 0 && a % b == 0 {
                Some(a / b)
            } else {
                None
            }
        }
        Expr::Min(a, b) => Some(const_value(a)?.min(const_value(b)?)),
        Expr::Max(a, b) => Some(const_value(a)?.max(const_value(b)?)),
        Expr::Floor(inner) | Expr::Ceil(inner) => const_value(inner),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::sheet_rule::{
        evaluate, Ability, Applies, CharacterFacts, ClassRef, EvalContext, HeldSet, Provenance,
        SheetValue, Subject,
    };

    fn rule_with(prose: Vec<ProseSegment>) -> SheetRule {
        SheetRule {
            id: "core_rulebook:spell:test_rule".to_string(),
            label: "Test Rule".to_string(),
            value: SheetValue::Text,
            also: Vec::new(),
            prose,
            applies: Applies::Always,
            target: None,
            bonus_type: None,
            print: true,
            pool: "spell".to_string(),
            tags: Vec::new(),
            subject: Subject::Character,
            repeatable: false,
            granted_by: Vec::new(),
            offers: None,
            grants: Vec::new(),
            provenance: Provenance::default(),
        }
    }

    fn segment(family: ProseFamily, pieces: Vec<ProsePiece>) -> ProseSegment {
        ProseSegment { family, pieces, applies: None, pick_last: false, suppress_when_all_zero: false }
    }

    fn package_with(rule: SheetRule) -> SheetRulePackage {
        let mut p = SheetRulePackage::new();
        p.insert_rule(rule);
        p.finish();
        p
    }

    /// The defect this module exists to fix, stated as a test: the sheet evaluator with no
    /// character prints the ARG shape as `0`, and a catalog screen has no character.
    #[test]
    fn an_unsettled_slot_prints_the_rules_words_where_the_evaluator_prints_zero() {
        let rule = rule_with(vec![segment(
            ProseFamily::Desc,
            vec![
                ProsePiece::Text("contained within you for up to ".into()),
                ProsePiece::Slot(Expr::CasterLevel(ClassRef::Holder)),
                ProsePiece::Text(" rounds".into()),
            ],
        )]);
        let package = package_with(rule.clone());

        let line = evaluate(
            &rule,
            &HeldSet::default(),
            &package,
            &CharacterFacts::default(),
            EvalContext::default(),
        );
        assert_eq!(
            line.prose, "contained within you for up to 0 rounds",
            "the evaluator's characterless answer is the wrong number this module replaces"
        );

        assert_eq!(
            catalog_prose(&package, &rule),
            "contained within you for up to caster level rounds"
        );
    }

    #[test]
    fn a_constant_slot_still_prints_its_number() {
        let rule = rule_with(vec![segment(
            ProseFamily::Desc,
            vec![
                ProsePiece::Text("a DC ".into()),
                ProsePiece::Slot(Expr::Sum(vec![Expr::Const(10), Expr::Const(5)])),
                ProsePiece::Text(" Fortitude save".into()),
            ],
        )]);
        let package = package_with(rule.clone());
        assert_eq!(catalog_prose(&package, &rule), "a DC 15 Fortitude save");
    }

    #[test]
    fn a_slot_mixing_a_constant_and_a_character_term_is_words() {
        let rule = rule_with(vec![segment(
            ProseFamily::Desc,
            vec![
                ProsePiece::Text("DC ".into()),
                ProsePiece::Slot(Expr::Sum(vec![
                    Expr::Const(10),
                    Expr::AbilityMod(Ability::Cha),
                ])),
            ],
        )]);
        let package = package_with(rule.clone());
        assert_eq!(catalog_prose(&package, &rule), "DC 10 plus Charisma modifier");
    }

    #[test]
    fn dice_stay_dice_and_a_constant_modifier_folds() {
        let rule = rule_with(vec![segment(
            ProseFamily::Desc,
            vec![ProsePiece::Dice { dice: "1d8".into(), modifier: Some(Expr::Const(2)) }],
        )]);
        let package = package_with(rule.clone());
        assert_eq!(catalog_prose(&package, &rule), "1d8+2");
    }

    #[test]
    fn a_dice_modifier_on_a_character_term_reads_as_words_beside_the_dice() {
        let rule = rule_with(vec![segment(
            ProseFamily::Desc,
            vec![ProsePiece::Dice {
                dice: "1d6".into(),
                modifier: Some(Expr::AbilityMod(Ability::Str)),
            }],
        )]);
        let package = package_with(rule.clone());
        assert_eq!(catalog_prose(&package, &rule), "1d6 plus Strength modifier");
    }

    #[test]
    fn a_gated_segment_prints_its_condition_in_words() {
        let mut seg = segment(ProseFamily::Desc, vec![ProsePiece::Text("you fly".into())]);
        seg.applies = Some(Applies::Compare {
            lhs: Expr::Level,
            op: crate::rules_core::sheet_rule::Cmp::Gte,
            rhs: Expr::Const(5),
        });
        let rule = rule_with(vec![seg]);
        let package = package_with(rule.clone());
        assert_eq!(
            catalog_prose(&package, &rule),
            "If character level at least 5: you fly"
        );
    }

    #[test]
    fn the_description_families_exclude_the_stat_block() {
        let rule = rule_with(vec![
            segment(ProseFamily::Desc, vec![ProsePiece::Text("the words".into())]),
            segment(
                ProseFamily::StatBlock("Casting time".into()),
                vec![ProsePiece::Text("1 standard action".into())],
            ),
        ]);
        let package = package_with(rule.clone());
        assert_eq!(catalog_description(&package, &rule).as_deref(), Some("the words"));
        assert_eq!(
            catalog_prose(&package, &rule),
            "the words\nCasting time: 1 standard action"
        );
    }

    #[test]
    fn a_record_with_no_descriptive_prose_serves_no_description() {
        let rule = rule_with(vec![segment(
            ProseFamily::StatBlock("Casting time".into()),
            vec![ProsePiece::Text("1 standard action".into())],
        )]);
        let package = package_with(rule.clone());
        assert_eq!(catalog_description(&package, &rule), None);
    }

    #[test]
    fn lookup_is_by_kind_and_the_names_slug() {
        let rule = rule_with(vec![segment(
            ProseFamily::Desc,
            vec![ProsePiece::Text("the words".into())],
        )]);
        let package = package_with(rule);
        assert_eq!(
            catalog_description_by_name(&package, "spell", "Test Rule").as_deref(),
            Some("the words")
        );
        assert_eq!(catalog_description_by_name(&package, "spell", "No Such Rule"), None);
        assert_eq!(catalog_description_by_name(&package, "feat", "Test Rule"), None);
    }

    /// The corpus-wide gate, over the live `data/sheet_rules/` directory rather than a fixture
    /// (`decisions.md §4`: a per-kind gate that reads the live corpus, never a per-unit fixture
    /// with a hand-derived value).
    ///
    /// Every rule whose prose carries a slot that **no character settles** is rendered both
    /// ways: through the sheet evaluator with empty facts, and through this module. The
    /// evaluator's answer for such a slot is a number it has no character to compute -- `0` --
    /// and this module's is the term's own words. The two must differ for every one of them,
    /// which is the property a catalog screen depends on and the one a presence check could
    /// never see (`presence-gates-vs-correctness-gates`: a wrong computed number looks right).
    ///
    /// The population is reported, so a cycle that changes the converter sees the figure move.
    #[test]
    fn every_unsettled_slot_in_the_live_package_renders_as_words_not_as_the_characterless_zero() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data/sheet_rules");
        if !dir.is_dir() {
            panic!("{} is not a directory -- regenerate with `cargo run --locked --bin sheet_rule_convert`", dir.display());
        }
        let load = crate::rules_core::corpus_loader::load_sheet_rules(&dir);
        let package = load.package;
        assert!(!package.rules.is_empty(), "the converted package carries no rules");

        let held = HeldSet::default();
        let facts = CharacterFacts::default();
        let mut with_unsettled_slot = 0usize;
        let mut disagreed = 0usize;
        let mut agreed: Vec<String> = Vec::new();
        for rule in package.rules.values() {
            let unsettled = rule.prose.iter().any(|s| {
                s.pieces.iter().any(|p| match p {
                    ProsePiece::Slot(e) => const_value(e).is_none(),
                    ProsePiece::Dice { modifier: Some(e), .. } => const_value(e).is_none(),
                    _ => false,
                })
            });
            if !unsettled {
                continue;
            }
            with_unsettled_slot += 1;
            let sheet = evaluate(rule, &held, &package, &facts, EvalContext::default()).prose;
            let catalog = catalog_prose(&package, rule);
            if sheet == catalog {
                if agreed.len() < 10 {
                    agreed.push(rule.id.clone());
                }
            } else {
                disagreed += 1;
            }
        }
        println!(
            "sheet_rule_catalog live gate: rules={} with_unsettled_slot={with_unsettled_slot} \
             rendered_differently={disagreed}",
            package.rules.len()
        );
        assert!(
            with_unsettled_slot > 0,
            "no converted rule carries an unsettled slot -- this gate is measuring nothing"
        );
        assert!(
            agreed.is_empty(),
            "{} rule(s) render identically with and without a character, so their unsettled \
             slot still reaches a catalog screen as the characterless number; first few: {agreed:?}",
            with_unsettled_slot - disagreed
        );
    }

    #[test]
    fn const_value_refuses_an_inexact_division_rather_than_rounding_it() {
        assert_eq!(const_value(&Expr::Div(Box::new(Expr::Const(7)), Box::new(Expr::Const(2)))), None);
        assert_eq!(const_value(&Expr::Div(Box::new(Expr::Const(6)), Box::new(Expr::Const(2)))), Some(3));
        assert_eq!(const_value(&Expr::Level), None);
    }
}
