//! Prerequisites of the picks an Abilities pool offers (SD-43 E3.5, `technical-design.md` §3.1).
//!
//! A pool member's `PRE*` tokens were converted into its `applies` gate at ingest; the converter
//! report (`data/sheet_rules/_report.json`, `degraded_by_token_type`) lists no `PRE*` row it could
//! not convert. This module reads that gate with the feat path's evaluator
//! ([`converted_gate::verdicts`]): `Unmet` terms make an option unqualified and are the words its
//! refusal names; a term the engine cannot decide is reported, never refused (that module's doc).
//! [`coverage`] counts every member of every Class Features pool by the shape of its terms.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::rules_core::ability_budget::{is_class_feature_grant, AbilityOption};
use crate::rules_core::feat_prereqs::converted_gate::{unverifiable_reason, verdicts, TermVerdict};
use crate::rules_core::sheet_rule::{offer_selects, Applies, CharacterFacts, Choice, HeldSet, SheetRulePackage};

/// The words the converter prints for a `PRETEXT` part written in rule syntax it could not convert
/// (`codex-ingest` `sheet_rule/prereq.rs`, `pretext`): the one term shape no evaluator arm reads.
const UNCONVERTED_RULE_SYNTAX: &str = "a further requirement the source states only in rule syntax";

/// The unmet prerequisites of `gate` for this character, in the rule's own words.
pub fn unmet(package: &SheetRulePackage, held: &HeldSet, facts: &CharacterFacts, gate: &Applies) -> Vec<String> {
    verdicts(package, held, facts, gate)
        .into_iter()
        .filter_map(|verdict| match verdict {
            TermVerdict::Unmet(words) => Some(words),
            _ => None,
        })
        .collect()
}

/// Why this character may not take `option`, naming every unmet prerequisite; `None` when it qualifies.
pub fn refusal(option: &AbilityOption) -> Option<String> {
    (!option.qualified).then(|| format!("'{}' cannot be taken by this character: {}", option.label, option.unmet.join("; ")))
}

/// Every member of every Class Features pool, counted by the shape of its prerequisite terms.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PrereqCoverage {
    /// Distinct records some Class Features pool offers.
    pub members: usize,
    /// Members every term of whose gate the evaluator reads (decided, or reported with its reason).
    pub evaluated: usize,
    /// Of `evaluated`: members with no prerequisite term at all.
    pub no_prerequisite: usize,
    /// Of `evaluated`: members with a term reported, not decided, by reason -> members.
    pub reported: BTreeMap<&'static str, usize>,
    /// Members with a term of an unknown shape, by shape -> member record ids.
    pub unknown: BTreeMap<&'static str, Vec<String>>,
}

impl PrereqCoverage {
    pub fn unknown_members(&self) -> usize {
        self.unknown.values().flatten().collect::<BTreeSet<_>>().len()
    }
}

impl fmt::Display for PrereqCoverage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unknown: BTreeMap<_, _> = self.unknown.iter().map(|(shape, ids)| (shape, ids.len())).collect();
        write!(
            f,
            "class-feature pool members {}: evaluated {} (no prerequisite {}; reported not decided {:?}); unknown token shape {} {:?}",
            self.members,
            self.evaluated,
            self.no_prerequisite,
            self.reported,
            self.unknown_members(),
            unknown
        )
    }
}

/// [`PrereqCoverage`] over every record an offer of a Class Features pool grant selects.
pub fn coverage(package: &SheetRulePackage) -> PrereqCoverage {
    let mut offers: Vec<&Choice> = Vec::new();
    for rule in package.rules.values().filter(|rule| is_class_feature_grant(rule)) {
        if let Some(offer) = &rule.offers
            && !offers.iter().any(|seen| seen.from == offer.from)
        {
            offers.push(offer);
        }
    }
    let mut out = PrereqCoverage::default();
    for member in package.rules.values().filter(|rule| !rule.label.is_empty()) {
        if !offers.iter().any(|offer| offer_selects(offer, member)) {
            continue;
        }
        out.members += 1;
        let terms = top_level_terms(&member.applies);
        let mut unknown = false;
        if terms.iter().any(|term| unconverted(term)) {
            out.unknown.entry(UNCONVERTED_RULE_SYNTAX).or_default().push(member.id.clone());
            unknown = true;
        }
        if unknown {
            continue;
        }
        out.evaluated += 1;
        if terms.iter().all(|term| matches!(term, Applies::Always)) {
            out.no_prerequisite += 1;
        }
        let reasons: BTreeSet<&'static str> = terms.iter().filter_map(|term| unverifiable_reason(package, term)).collect();
        for reason in reasons {
            *out.reported.entry(reason).or_default() += 1;
        }
    }
    out
}

/// The terms [`verdicts`] reports one by one (`All` flattened one level).
fn top_level_terms(gate: &Applies) -> Vec<&Applies> {
    match gate {
        Applies::All(terms) => terms.iter().collect(),
        other => vec![other],
    }
}

fn unconverted(term: &Applies) -> bool {
    match term {
        Applies::Situational { text } => text == UNCONVERTED_RULE_SYNTAX,
        Applies::All(terms) | Applies::AtLeast { of: terms, .. } => terms.iter().any(unconverted),
        Applies::Not(inner) => unconverted(inner),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::ability_budget::{ability_options, SubTab};
    use crate::rules_core::class_census::load_sweep_fixture;
    use crate::rules_core::class_seeds::input_for;
    use crate::rules_core::corpus_loader::load_sheet_rules;
    use crate::rules_core::pilot_compute::compute_pilot_base_chassis;
    use crate::rules_core::character_input::AbilitySelection;
    use crate::rules_core::sheet_rule::{held_set, HeldSeed};
    use std::sync::OnceLock;

    fn package() -> &'static SheetRulePackage {
        static PACKAGE: OnceLock<SheetRulePackage> = OnceLock::new();
        PACKAGE.get_or_init(|| load_sheet_rules(&std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data/sheet_rules")).package)
    }

    /// Silent Kill (Advanced Class Guide): "Prerequisites: Stealth 12 ranks, assassinate slayer talent or ninja
    /// trick". Its one class-granted carrier is the warpriest's bonus feat list, a pool PCGen displays on the Feats
    /// sub-tab (`acg_abilitycategories.lst:125`, `DISPLAYLOCATION:Feats`), so this reads its gate with the
    /// evaluator the pool options use rather than through the Class Features option list.
    #[test]
    fn ability_prereqs_silent_kill_without_assassinate_is_refused_naming_assassinate() {
        let input = input_for(&load_sweep_fixture().expect("census fixture"), "warpriest", 6);
        let base = compute_pilot_base_chassis(&input);
        let seed = HeldSeed::from_character(&input, &base);
        let facts = CharacterFacts::from_character(&input, &base).with_linked_picks(package(), &seed);
        let held = held_set(package(), &seed, &facts);
        let silent_kill = package().rule("advanced_class_guide:class_feature:warpriest_bonus_feat_silent_kill").expect("record");
        let unmet = unmet(package(), &held, &facts, &silent_kill.applies);
        assert!(unmet.iter().any(|reason| reason.contains("Assassinate")), "{unmet:?}");
    }

    /// The Class Features shape of the same rule: Major Magic (Core Rulebook, Rogue Talents): "A rogue must have the
    /// minor magic rogue talent and an Intelligence score of at least 11 before selecting this talent."
    #[test]
    fn ability_prereqs_major_magic_without_minor_magic_is_refused_naming_minor_magic() {
        let mut input = input_for(&load_sweep_fixture().expect("census fixture"), "rogue", 4);
        let base = compute_pilot_base_chassis(&input);
        let find = |options: &[AbilityOption], label: &str| {
            options.iter().find(|option| option.pool == "rogue_talent" && option.label == label).cloned().unwrap_or_else(|| panic!("no {label}"))
        };
        let options = ability_options(package(), &input, &base, SubTab::ClassFeatures);
        let major = find(&options, "Major Magic");
        let reason = refusal(&major).expect("Major Magic is refused without Minor Magic");
        assert!(reason.contains("Minor Magic"), "{reason}");

        let minor = find(&options, "Minor Magic");
        let (_, record) = minor.id.split_once(':').expect("pool:record");
        input.chosen.ability_selections.push(AbilitySelection {
            subtab: SubTab::ClassFeatures.id().to_owned(),
            pool: "rogue_talent".to_owned(),
            record_key: record.to_owned(),
            level: 2,
            target: None,
        });
        let base = compute_pilot_base_chassis(&input);
        let major = find(&ability_options(package(), &input, &base, SubTab::ClassFeatures), "Major Magic");
        assert!(!major.unmet.iter().any(|reason| reason.contains("Minor Magic")), "{:?}", major.unmet);
    }

    /// Acceptance coverage (E3.5): every member of every class-feature pool, each prerequisite term either a shape
    /// the engine evaluates or an unknown shape, by shape.
    #[test]
    fn ability_prereqs_coverage_every_pool_member_is_evaluated_or_an_unknown_shape() {
        let coverage = coverage(package());
        eprintln!("{coverage}");
        assert!(coverage.members > 0);
        assert_eq!(coverage.members, coverage.evaluated + coverage.unknown_members());
        assert_eq!(coverage.unknown_members(), 0, "each unknown shape is a discovered card: {:?}", coverage.unknown);
    }
}
