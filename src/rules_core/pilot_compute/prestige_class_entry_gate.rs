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
//! spellcasting type and so on satisfy a prestige class's real PCGen
//! `PRE*`-family tokens is a self-contained question this module answers
//! for real, reusing the exact evaluator `feat_prereqs::pre_tokens` already
//! proved against 690 catalog records. This module still returns no
//! chassis magnitude (that stays claim-blocked via the caller's existing
//! `class_chassis.unsupported` diagnostic, `mod.rs`
//! `compute_pilot_base_chassis`) -- it proves and reports whether entry
//! requirements are met, which is the acceptance criterion's own scope.
//!
//! # Population: 62 of the quoted 77, and why
//!
//! `epic-breakdown.md` Epic 3 and this criterion both quote **77** prestige
//! classes. `scripts/census_prestige_class_entry_requirements.py` --
//! this module's own re-derive command, run against the pinned oracle --
//! finds **131** `TYPE:...Prestige` `CLASS:` names corpus-wide (all 158
//! oracle books) and **62** of them anchored in a book this repo has
//! actually ingested (`data/corpus/<book>/` exists). Gating logic for the
//! other 69 would be untestable fiction: no ingested corpus data exists to
//! fixture-check it against, and most of their source books are exactly
//! the "28 books-without-ruleset" `AT-32-E3-001` names as Epic 4's own
//! precondition. The 77 figure is corrected in
//! `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/`'s
//! cycle receipt for this card, via a logged `scripts/retro.py correction`.
//!
//! # Three outcomes per clause, matching `pre_tokens::ClauseOutcome`
//!
//! A clause this module cannot evaluate is reported `Unmodelled`, never
//! silently passed or silently failed -- the same discipline
//! `pre_tokens.rs` documents at length. **Only `Unmet` blocks
//! `qualifies`.** `PRETOTALAB` (total base attack bonus) is a special case:
//! at this call site (`compute_class_chassis`, before this very class's own
//! chassis has been computed) the character's base attack bonus for a
//! from-nothing prestige entry is not a known fact, so this module reports
//! it `Unmodelled` explicitly rather than defaulting to `0` and fabricating
//! an `Unmet` verdict against a character who may well qualify -- the exact
//! "confidently wrong" trap `pre_tokens.rs`'s own doc comment names.

use std::sync::OnceLock;

use crate::rules_core::character_input::CharacterInput;
use crate::rules_core::feat_prereqs::pre_tokens::{
    evaluate_prerequisite_token, CharacterPrereqFacts, ClauseOutcome,
};

/// One prestige class's real, corpus-derived entry-requirement tokens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrestigeClassEntryRequirement {
    pub class_id: String,
    pub display_name: String,
    pub source_book: String,
    pub source_file: String,
    pub raw_pre_tokens: Vec<String>,
}

#[derive(serde::Deserialize)]
struct FixtureFile {
    entries: Vec<FixtureEntry>,
}

#[derive(serde::Deserialize)]
struct FixtureEntry {
    class_id: String,
    display_name: String,
    source_book: String,
    source_file: String,
    pre_tokens: Vec<String>,
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
                    raw_pre_tokens: e.pre_tokens,
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
    /// `pre_tokens::ClauseOutcome::blocks`'s own contract.
    pub qualifies: bool,
    pub met: Vec<String>,
    pub unmet: Vec<String>,
    pub unmodelled: Vec<String>,
    pub informational: Vec<String>,
}

/// Evaluates `class_id_str`'s real entry-requirement tokens against
/// `input`, or `None` when `class_id_str` names no prestige class this
/// module's registry covers (not every unrecognized class id is a
/// prestige class -- an unmodelled base class, a typo, or a class this
/// repo has not census'd at all all look identical from here, and this
/// function only speaks for the registry it actually holds).
pub fn evaluate_prestige_class_entry(
    class_id_str: &str,
    input: &CharacterInput,
) -> Option<PrestigeEntryGateOutcome> {
    let requirement = find_by_class_id(class_id_str)?;

    // `base_attack_bonus: 0` is a filler value the loop below never trusts:
    // every `PRETOTALAB` clause is special-cased to `Unmodelled` before it
    // ever reaches `evaluate_prerequisite_token`, so this filler value can
    // never silently produce a wrong `Unmet` verdict from it. See this
    // module's doc comment.
    let facts = CharacterPrereqFacts::from_character(input, 0);

    let mut met = Vec::new();
    let mut unmet = Vec::new();
    let mut unmodelled = Vec::new();
    let mut informational = Vec::new();

    for token in &requirement.raw_pre_tokens {
        let bare_kind = token
            .trim_start_matches('!')
            .split(':')
            .next()
            .unwrap_or(token)
            .trim();
        let outcome = if bare_kind == "PRETOTALAB" {
            ClauseOutcome::Unmodelled {
                token: token.clone(),
                note: "base attack bonus is this class's own not-yet-computed chassis output, \
                       not a known fact at the compute_class_chassis entry-gate call site"
                    .to_owned(),
            }
        } else {
            evaluate_prerequisite_token(token, &facts)
        };

        match outcome {
            ClauseOutcome::Met { requirement } => met.push(requirement),
            ClauseOutcome::Unmet { requirement, reason } => {
                unmet.push(format!("{requirement} ({reason})"))
            }
            ClauseOutcome::Unmodelled { token, note } => {
                unmodelled.push(format!("{token} ({note})"))
            }
            ClauseOutcome::Informational { text } => informational.push(text),
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
        // population figure at commit time.
        assert_eq!(entries.len(), 62, "population drifted from the fixture the script wrote");
        assert!(entries.iter().all(|e| !e.raw_pre_tokens.is_empty()));
        assert!(entries.iter().all(|e| e.class_id.starts_with("class:")));
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
        // Proves the check itself can fail: a clause no character can ever
        // satisfy must surface as Unmet, not silently disappear.
        let facts = CharacterPrereqFacts::from_character(&empty_input("class:arcane_archer"), 0);
        let outcome = evaluate_prerequisite_token(
            "PRESKILL:1,Nonexistent Skill No Character Has=99",
            &facts,
        );
        assert!(outcome.blocks(), "an impossible clause must block");
    }
}
