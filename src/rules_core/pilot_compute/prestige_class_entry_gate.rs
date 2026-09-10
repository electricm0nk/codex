//! Prestige-class entry-requirement gating (SD-32 Epic 3,
//! `epic-3-class-reachability`, `acceptance-and-verification.md`
//! AT-32-E3-001): "The 77 prestige classes have entry-requirement gating
//! that exists nowhere in the codebase today; the cycle that builds it
//! cites the `compute_class_chassis` call site and proves the gating runs
//! (fixture-checked, of course)."
//!
//! # What this closes, and what it deliberately does not
//!
//! SD-31 wave 27's own investigation (`compute_class_chassis`'s doc
//! comment block above `compute_generic_table_chassis`, this module's
//! sibling) found two independent reasons a prestige class cannot reach a
//! full chassis (base attack bonus / base saves) through this dispatch
//! today: no class-id enum registers any of them, and six of the ten CRB
//! prestige classes need a caster-level-stacking mechanism this codebase
//! does not have. **Neither blocker touches entry-requirement gating** --
//! whether a character's already-chosen feats, skill ranks, alignment,
//! spellcasting type and so on satisfy a prestige class's real entry
//! requirements is a self-contained question this module answers for real,
//! reusing the exact evaluator the sheet itself renders through. This module still returns no
//! chassis magnitude (that stays claim-blocked via the caller's existing
//! `class_chassis.unsupported` diagnostic, `mod.rs`
//! `compute_pilot_base_chassis`) -- it proves and reports whether entry
//! requirements are met, which is the acceptance criterion's own scope.
//!
//! # Population: 74 of the quoted 77, and why
//!
//! `epic-breakdown.md` Epic 3 and this criterion both quote **77** prestige
//! classes. `scripts/census_prestige_class_entry_requirements.py` --
//! this module's own re-derive command, run against the pinned oracle --
//! finds **131** `TYPE:...Prestige` `CLASS:` names corpus-wide (all 158
//! oracle books) and **74** of them anchored in a book this repo has
//! actually ingested (`data/corpus/<book>/` exists). Gating logic for the
//! other 57 would be untestable fiction: no ingested corpus data exists to
//! fixture-check it against, and most of their source books are exactly
//! the "28 books-without-ruleset" `AT-32-E3-001` names as Epic 4's own
//! precondition. The 77 figure is corrected in
//! `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/`'s
//! cycle receipt for this card, via a logged `scripts/retro.py correction`.
//!
//! **62 -> 74, a REAL script-bug fix (SD-34 wave 44, 2026-09-05).** The
//! script's own `prestige_names.setdefault(name, path)` keyed purely by
//! display name across the full 158-book oracle, so whichever source file
//! `os.walk` happened to visit FIRST for a given class name won -- even a
//! non-ingested predecessor book. 13 real ingested-book prestige classes
//! (Phrenic Slayer, Thrallherd, Psychic Fist, War Mind, Elocater, Psion
//! Uncarnate, Pyrokineticist, Metamind, Cerebremancer, Pathfinder Savant,
//! Student of War, Pathfinder Delver, Gifted Blade) were silently dropped
//! by this race. Fixed by ranking every candidate source by whether its
//! own book is ingested BEFORE choosing among ties, so an ingested-book
//! match always wins regardless of walk order (`scripts/census_prestige_
//! class_entry_requirements.py`'s own `extract()`, plus a new regression
//! test, `scripts/tests/test_census_prestige_class_entry_requirements.py`,
//! that reproduces the exact collision and fails against the pre-fix
//! script). Of the 13 named, 12 were genuinely recovered this way (Gifted
//! Blade never actually carries a `TYPE:...Prestige` line anywhere in the
//! oracle under this script's own census method -- the audit's own list
//! was one name too long, corrected here rather than propagated). 62 + 12
//! = 74, confirmed by re-running the script against the pinned oracle and
//! diffing the regenerated `tests/fixtures/rules_core/prestige-class-
//! entry-requirements.json` against its pre-fix committed version: the
//! pre-existing 62 entries are byte-identical or, where they were one of
//! the 12 collision victims, now correct; zero regressions.
//!
//! # Where the requirements are read from (SD-35 `AT-35-E6-001`)
//!
//! From the CONVERTED class record, `data/sheet_rules/<book>/class/<slug>.json`, whose
//! `applies` gate is exactly this class's entry requirements plus its own level ceiling.
//! Until that cycle this module parsed the ingest format's `PRE`-family token text at run
//! time; `decisions.md` §11 rules that out, and the conversion had already happened. The
//! committed census fixture stays: it is what says WHICH class ids are prestige classes
//! and which book each one's record lives in.
//!
//! # Three outcomes per clause, unchanged
//!
//! A clause this module cannot evaluate is reported `unmodelled`, never silently passed or
//! silently failed -- `feat_prereqs::converted_gate` classifies which those are and why.
//! **Only `unmet` blocks `qualifies`.** The base attack bonus is the standing special case:
//! at this call site (`compute_class_chassis`, before this very class's own chassis has been
//! computed) a from-nothing prestige entry's base attack bonus is not a known fact, so a term
//! reading it is reported `unmodelled` explicitly rather than being decided against a `0` and
//! fabricating an `unmet` verdict against a character who may well qualify.

use std::sync::OnceLock;

use crate::rules_core::character_input::CharacterInput;
use crate::rules_core::feat_prereqs::converted_gate::{self, TermVerdict};
use crate::rules_core::feat_prereqs::PrereqFacts;
use crate::rules_core::sheet_rule::{Applies, Expr, SheetRule};

/// One prestige class's real, corpus-derived entry-requirement tokens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrestigeClassEntryRequirement {
    pub class_id: String,
    pub display_name: String,
    pub source_book: String,
    pub source_file: String,
}

#[derive(serde::Deserialize)]
struct FixtureFile {
    entries: Vec<FixtureEntry>,
}

/// The census fixture's own row shape. The entry-requirement rows it also carries are
/// deliberately not deserialised: SD-35 `AT-35-E6-001` decides the CONVERTED record's gate,
/// and the fixture's remaining job is to say WHICH class ids are prestige classes and which
/// book each one's record lives in. Serde ignores the fields no struct field names.
#[derive(serde::Deserialize)]
struct FixtureEntry {
    class_id: String,
    display_name: String,
    source_book: String,
    source_file: String,
}

const FIXTURE_JSON: &str =
    include_str!("../../../tests/fixtures/rules_core/prestige-class-entry-requirements.json");

static REGISTRY: OnceLock<Vec<PrestigeClassEntryRequirement>> = OnceLock::new();

/// The full registry, parsed once from the committed fixture
/// (`scripts/census_prestige_class_entry_requirements.py`'s output).
/// Panics on a malformed fixture -- a corrupt committed fixture is a build
/// defect, not a runtime condition any caller can recover from.
pub fn prestige_class_entry_requirements() -> &'static [PrestigeClassEntryRequirement] {
    REGISTRY
        .get_or_init(|| {
            let parsed: FixtureFile = serde_json::from_str(FIXTURE_JSON)
                .expect("tests/fixtures/rules_core/prestige-class-entry-requirements.json must parse");
            parsed
                .entries
                .into_iter()
                .map(|e| PrestigeClassEntryRequirement {
                    class_id: e.class_id,
                    display_name: e.display_name,
                    source_book: e.source_book,
                    source_file: e.source_file,
                })
                .collect()
        })
        .as_slice()
}

fn find_by_class_id(class_id_str: &str) -> Option<&'static PrestigeClassEntryRequirement> {
    prestige_class_entry_requirements()
        .iter()
        .find(|entry| entry.class_id == class_id_str)
}

/// True iff `class_id_str` names a prestige class this registry census'd --
/// i.e. a class id the engine genuinely recognizes as real, even though
/// `compute_class_chassis` does not (yet) compute a BAB/save chassis for it
/// (see that function's own `prestige_class_entry_gate` branch, which always
/// returns `None` for the chassis regardless of `qualifies`).
///
/// SD-34 AT-34-E3-001 (`class_feature_owner_matched_by_name_but_record_not_
/// held_by_engine` mechanism): `pilot_compute::compute_pilot_base_chassis`'s
/// generic class_feature grant roster call site used `chassis_supported`
/// alone as its "is this a real, modelled class" precondition, which
/// silently withheld every one of a prestige-class character's own real,
/// described class features (e.g. Assassin's Hidden Weapons, Shadowdancer's
/// Darkvision) -- not because the roster couldn't ground them (it can; see
/// `class_feature_grant_consumer`'s own direct unit tests for exactly these
/// records), but because no chassis magnitude happens to exist for the
/// class that owns them. This accessor lets that call site widen its
/// precondition to "chassis_supported OR a real prestige class id", without
/// this module needing to expose its private registry lookup or say
/// anything about entry-requirement legality (a character need not qualify
/// for the prestige class to have chosen it -- the same "compute regardless
/// of legality" posture every other class id in this engine already gets).
pub fn is_registered(class_id_str: &str) -> bool {
    find_by_class_id(class_id_str).is_some()
}

/// The per-clause verdicts and overall qualification outcome for one
/// prestige class against one character's chosen input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrestigeEntryGateOutcome {
    pub class_id: String,
    pub display_name: String,
    /// True iff no clause evaluated `Unmet`. `Unmodelled` and
    /// `Informational` clauses never block, matching
    /// `converted_gate::TermVerdict::blocks`'s own contract.
    pub qualifies: bool,
    pub met: Vec<String>,
    pub unmet: Vec<String>,
    pub unmodelled: Vec<String>,
    pub informational: Vec<String>,
}

/// Evaluates `class_id_str`'s real entry requirements against `input`, or `None` when
/// `class_id_str` names no prestige class this module's registry covers (not every
/// unrecognized class id is a prestige class -- an unmodelled base class, a typo, or a class
/// this repo has not census'd at all all look identical from here, and this function only
/// speaks for the registry it actually holds).
///
/// `None` also when this checkout carries no converted package, or no converted record for
/// this registered class: an absent conversion is a statement about the repo, not a verdict
/// about the character, and reporting it as a failed gate would be a fabricated refusal.
pub fn evaluate_prestige_class_entry(
    class_id_str: &str,
    input: &CharacterInput,
) -> Option<PrestigeEntryGateOutcome> {
    let requirement = find_by_class_id(class_id_str)?;
    let package = crate::rules_core::corpus_loader::live_sheet_rules()?;
    let rule = converted_class_rule(package, requirement)?;

    // The chassis this gate's own caller (`compute_class_chassis`) is in the middle of
    // computing is not available here, and re-entering the engine would recurse. The facts
    // that needs are exactly the ones `unmodelled_reason` below reports rather than decides,
    // so the computation this gate builds carries the character's real ability modifiers and
    // nothing else.
    let computation = entry_gate_computation(input);
    let facts = PrereqFacts::new(package, input, &computation, &[]);

    let mut met = Vec::new();
    let mut unmet = Vec::new();
    let mut unmodelled = Vec::new();
    let mut informational = Vec::new();

    for term in top_level_terms(&rule.applies) {
        let words = converted_gate::describe(facts.package(), term);
        if let Some(note) = unmodelled_reason(term) {
            unmodelled.push(format!("{words} ({note})"));
            continue;
        }
        match converted_gate::verdicts(facts.package(), facts.held(), facts.facts(), term)
            .into_iter()
            .next()
        {
            Some(TermVerdict::Met(requirement)) => met.push(requirement),
            Some(TermVerdict::Unmet(reason)) => unmet.push(reason),
            Some(TermVerdict::Unverified(note)) => unmodelled.push(note),
            Some(TermVerdict::Informational(text)) => informational.push(text),
            // `Applies::Always` -- the record states no requirement here.
            None => {}
        }
    }

    Some(PrestigeEntryGateOutcome {
        class_id: requirement.class_id.clone(),
        display_name: requirement.display_name.clone(),
        qualifies: unmet.is_empty(),
        met,
        unmet,
        unmodelled,
        informational,
    })
}

/// The converted `<book>:class:<slug>` record for one registry row.
fn converted_class_rule<'a>(
    package: &'a crate::rules_core::sheet_rule::SheetRulePackage,
    requirement: &PrestigeClassEntryRequirement,
) -> Option<&'a SheetRule> {
    let slug = requirement.class_id.strip_prefix("class:").unwrap_or(&requirement.class_id);
    package
        .rule(&format!("{}:class:{slug}", requirement.source_book))
        .or_else(|| package.find("class", slug).and_then(|id| package.rule(id)))
}

fn top_level_terms(gate: &Applies) -> Vec<&Applies> {
    match gate {
        Applies::All(terms) => terms.iter().collect(),
        other => vec![other],
    }
}

/// The base attack bonus is this class's own not-yet-computed chassis output, not a known
/// fact at the `compute_class_chassis` entry-gate call site. A term reading it is reported,
/// never decided against a filler `0` -- the "confidently wrong" trap this module's doc
/// comment names. Everything else `converted_gate` already classifies.
fn unmodelled_reason(term: &Applies) -> Option<&'static str> {
    reads_base_attack(term).then_some(
        "base attack bonus is this class's own not-yet-computed chassis output, not a known \
         fact at the compute_class_chassis entry-gate call site",
    )
}

fn reads_base_attack(term: &Applies) -> bool {
    match term {
        Applies::All(terms) | Applies::AtLeast { of: terms, .. } => {
            terms.iter().any(reads_base_attack)
        }
        Applies::Not(inner) => reads_base_attack(inner),
        Applies::Compare { lhs, rhs, .. } => expr_reads_base_attack(lhs) || expr_reads_base_attack(rhs),
        _ => false,
    }
}

fn expr_reads_base_attack(e: &Expr) -> bool {
    match e {
        Expr::BaseAttack => true,
        Expr::Sum(terms) => terms.iter().any(expr_reads_base_attack),
        Expr::Mul(a, b) | Expr::Div(a, b) | Expr::Min(a, b) | Expr::Max(a, b) => {
            expr_reads_base_attack(a) || expr_reads_base_attack(b)
        }
        Expr::Floor(inner) | Expr::Ceil(inner) => expr_reads_base_attack(inner),
        _ => false,
    }
}

/// A computation carrying the character's real ability modifiers and no chassis -- see
/// [`evaluate_prestige_class_entry`] for why the chassis half is deliberately absent.
fn entry_gate_computation(input: &CharacterInput) -> super::PilotBaseChassisComputation {
    super::PilotBaseChassisComputation {
        ability_modifiers: super::ability_modifiers_from_scores(&input.chosen.ability_scores),
        base_attack_bonus: 0,
        base_saves: super::BaseSaves::default(),
        baseline_melee_attack_bonus: 0,
        baseline_armor_class: 0,
        total_saves: super::BaseSaves::default(),
        selected_skill_modifiers: super::SelectedSkillModifiers::default(),
        explanations: Vec::new(),
        diagnostics: Vec::new(),
        sheet_lines: Vec::new(),
    }
}

#[cfg(test)]
mod prestige_class_entry_gate_tests {
    use super::*;
    use crate::rules_core::character_input::{AbilityScores, ChosenCharacterState};

    fn empty_input(class_id: &str) -> CharacterInput {
        CharacterInput {
            case_id: None,
            source_package_id: "test".to_owned(),
            chosen: ChosenCharacterState {
                race_id: "race:human".to_owned(),
                class_levels: vec![crate::rules_core::character_input::CharacterClassLevel {
                    class_id: class_id.to_owned(),
                    level: 1,
                }],
                ability_scores: AbilityScores::default(),
                selected_feats: Vec::new(),
                skill_allocations: Vec::new(),
                equipment_selections: Vec::new(),
                selected_choices: Vec::new(),
                selected_traits: Vec::new(),
                spells_selected: Vec::new(),
                class_ability_activations: Vec::new(),
            },
            selection_provenance: Vec::new(),
        }
    }

    #[test]
    fn registry_loads_and_matches_the_re_derive_command() {
        let entries = prestige_class_entry_requirements();
        // scripts/census_prestige_class_entry_requirements.py's own stderr
        // population figure at commit time. 62 -> 74, SD-34 wave 44: see
        // this file's own module doc comment ("62 -> 74, a REAL script-bug
        // fix") for the ingested-book-collision bug the script's own
        // extract() function had, and its fix.
        assert_eq!(entries.len(), 74, "population drifted from the fixture the script wrote");
        assert!(entries.iter().all(|e| e.class_id.starts_with("class:")));
        // SD-35 `AT-35-E6-001`: the gate decides the CONVERTED record, so a registry row
        // whose record the package does not carry is a row this gate silently stops
        // answering for. Counted, not assumed.
        let package = crate::rules_core::corpus_loader::live_sheet_rules()
            .expect("data/sheet_rules/ must be loadable in a repo checkout");
        let missing: Vec<&str> = entries
            .iter()
            .filter(|e| converted_class_rule(package, e).is_none())
            .map(|e| e.class_id.as_str())
            .collect();
        assert!(missing.is_empty(), "registry rows with no converted class record: {missing:?}");
    }

    #[test]
    fn unknown_class_id_returns_none() {
        let input = empty_input("class:not_a_real_class");
        assert!(evaluate_prestige_class_entry("class:not_a_real_class", &input).is_none());
    }

    #[test]
    fn arcane_archer_with_no_feats_is_unmet() {
        let input = empty_input("class:arcane_archer");
        let outcome = evaluate_prestige_class_entry("class:arcane_archer", &input)
            .expect("arcane_archer is in the registry");
        assert!(!outcome.qualifies, "no feats selected -- must not qualify");
        assert!(
            !outcome.unmet.is_empty(),
            "PREABILITY clauses for Point-Blank Shot/Precise Shot must be Unmet"
        );
    }

    #[test]
    fn arcane_archer_with_the_real_feats_qualifies() {
        let mut input = empty_input("class:arcane_archer");
        // The real corpus clause (`cr_classes.lst`, PREABILITY body) --
        // feat_identity::same must recognize these display names.
        input.chosen.selected_feats = vec![
            "Point-Blank Shot".to_owned(),
            "Precise Shot".to_owned(),
            "Weapon Focus (Longbow)".to_owned(),
        ];
        let outcome = evaluate_prestige_class_entry("class:arcane_archer", &input)
            .expect("arcane_archer is in the registry");
        assert!(
            outcome.unmet.is_empty(),
            "expected no Unmet clauses with the real prerequisite feats present, got: {:?}",
            outcome.unmet
        );
        assert!(outcome.qualifies);
        // PRESPELLTYPE (spellcasting) and PRETOTALAB are genuinely
        // unmodelled at this layer -- proves the gate is honest about the
        // gap rather than silently passing it.
        assert!(!outcome.unmodelled.is_empty());
    }

    #[test]
    fn mutation_proof_a_fabricated_impossible_clause_is_caught() {
        // Proves the check itself can fail: a term no character can ever satisfy must
        // surface as Unmet, not silently disappear.
        let package = crate::rules_core::corpus_loader::live_sheet_rules()
            .expect("data/sheet_rules/ must be loadable in a repo checkout");
        let input = empty_input("class:arcane_archer");
        let computation = entry_gate_computation(&input);
        let facts = PrereqFacts::new(package, &input, &computation, &[]);
        let impossible = Applies::Compare {
            lhs: Expr::SkillRanks("no_character_has_this_skill".to_owned()),
            op: crate::rules_core::sheet_rule::Cmp::Gte,
            rhs: Expr::Const(99),
        };
        let verdicts =
            converted_gate::verdicts(facts.package(), facts.held(), facts.facts(), &impossible);
        assert!(
            verdicts.iter().any(TermVerdict::blocks),
            "an impossible term must block: {verdicts:?}"
        );
    }
}
