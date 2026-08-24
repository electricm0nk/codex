//! SD-31 wave 25 — the interpreter reproduction harness (`OPERATOR-RULINGS-2026-08-21.md` §20,
//! "RULED, 2026-08-21: §24.1 IS OVERTURNED. Build the interpreter.").
//!
//! **What this module is.** The operator authorised reading PCGen's own formula tokens with a
//! general evaluator instead of hand-writing one pure function per feature, on one condition:
//! "every interpreted value must clear `derived_evaluator_fixture_check`... An interpreted value
//! with no fixture is not done." This module is the OTHER half of that condition — the wide,
//! standing regression proof the ruling calls for: "roughly 27 classes' worth of hand-modelled
//! functions already exist... The interpreter must REPRODUCE ALL OF THEM."
//!
//! **What this module is NOT.** It is not the interpreter. No interpreter exists in this
//! worktree yet (interpreter-core is a separate wave-25 lane). This module therefore ships three
//! things that stand on their own and do not require the interpreter to exist to be true today:
//!
//! 1. [`enumerate_hand_modelled_citations`] — a MECHANICAL count of hand-modelled formula
//!    functions in `pilot_compute`, derived by scanning the real source text for a doc comment
//!    citing a PCGen `BONUS:`/`DEFINE:` token immediately above a `fn` definition. Not a
//!    hand-maintained list: delete a citation, rename a function, or remove a `fn` and the count
//!    changes on the next run, by construction (`enumeration_is_mechanical_not_a_fixed_number`
//!    proves this against embedded fixtures below, without editing any real hand-modelled
//!    function to do it).
//! 2. [`FormulaEvaluator`] — the interface contract this harness calls, written so the
//!    interpreter-core lane's evaluator can implement it directly at integration: given the raw
//!    RHS of a `BONUS:`-family token exactly as the corpus states it, and the PCGen variable
//!    bindings for one sample point, return the integer the token evaluates to.
//! 3. [`REPRODUCTION_CASES`] and [`run_reproduction`] — a real, working comparison harness:
//!    for each case, call the SAME hand-modelled function the corpus already ships (imported
//!    unmodified from `super::*` / `class_slayer`, never edited by this module) and the supplied
//!    `FormulaEvaluator` over the SAME formula text and the SAME sample points, and report every
//!    disagreement with its exact inputs and both outputs. Nothing here adjusts either side to
//!    make them agree — a disagreement is a finding to report, not a defect in this file to fix.
//!
//! **Proving the harness itself, not the interpreter, per Decision 1(a).** With no real
//! interpreter to hand yet, this module's own tests use a `#[cfg(test)]`-only
//! [`tests::ToyReferenceEvaluator`] — a small, real recursive-descent evaluator for the specific
//! arithmetic/`max`/`min`/`floor`/`if`/`classlevel` grammar these 21 dispatched tokens actually
//! use — SOLELY to prove [`run_reproduction`]'s comparison and reporting logic is correct and can
//! fail loudly. `tests::harness_detects_a_deliberately_wrong_evaluator` mutates that toy
//! evaluator's output for one case and asserts the mismatch is caught, the anti-gaming
//! mutation-proof Decision 1(a) requires of a gate. The toy evaluator is never exported outside
//! `#[cfg(test)]`, is never on any production path, and its agreement with the hand functions is
//! NOT evidence the real PCGen interpreter will agree — it only proves this harness would catch
//! it if the real interpreter did not.
//!
//! **Coordination note for the interpreter-core lane.** Implement [`FormulaEvaluator`] for the
//! real evaluator and pass `&real_evaluator` to [`run_reproduction`] with
//! [`REPRODUCTION_CASES`] at integration; nothing else in this file needs to change. Two open
//! semantics questions this harness could not resolve from the hand-modelled functions' own
//! sampled range (all non-negative divisors and dividends): (a) whether PCGen's `/` truncates
//! toward zero or floors for a negative operand — every existing hand function uses Rust's `/`,
//! which truncates toward zero, but none of them are ever called with a value that makes this
//! observable; (b) `classlevel("X")` is only ever called here with the record's own single class,
//! so this harness cannot distinguish "current level in class X" from "total character level" —
//! multiclass sample points are out of this lane's scope.

use std::collections::BTreeMap;

// ---------------------------------------------------------------------------------------------
// 1. Mechanical enumeration of hand-modelled formula functions
// ---------------------------------------------------------------------------------------------

/// One hand-modelled function whose doc comment cites a PCGen `BONUS:`/`DEFINE:` token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandModelledCitation {
    pub file: &'static str,
    pub function_name: String,
    /// The first doc-comment line (source-order) that contains the citation, verbatim.
    pub cited_line: String,
}

/// The fixed set of files this program's hand-modelled `class_*` per-feature formula functions
/// live in (`pilot_compute`'s own module list, `mod.rs`'s lines 166-168, plus `mod.rs` itself —
/// not a curated subset of THEIR CONTENTS, only of which files constitute "the hand-modelled
/// function home"). Adding a new `pilot_compute` submodule of hand-modelled functions means
/// adding its path here for the enumerator to see it; the enumerator does not otherwise choose
/// which functions inside these files count — that is [`citations_in_source`]'s job, driven by
/// the doc-comment text alone.
const HAND_MODELLED_FUNCTION_HOME_FILES: &[(&str, &str)] = &[
    ("pilot_compute/mod.rs", include_str!("mod.rs")),
    ("pilot_compute/class_slayer.rs", include_str!("class_slayer.rs")),
    (
        "pilot_compute/class_ultimate_combat.rs",
        include_str!("class_ultimate_combat.rs"),
    ),
    (
        "pilot_compute/class_feature_grant_consumer.rs",
        include_str!("class_feature_grant_consumer.rs"),
    ),
];

/// The live, re-derived denominator: every hand-modelled function across the files above whose
/// immediately-preceding `///` doc-comment block cites a `BONUS:` or `DEFINE:` PCGen token.
///
/// This is a PROXY, not a perfect census — see `tests::the_enumerator_has_a_known_blind_spot`
/// for a concrete counter-example (`monk_scorpion_style_dc`, an operator-named exemplar with NO
/// doc comment at all, invisible to this scan). Report it as a conservative FLOOR on the real
/// population, per the standing "validate a proxy where it makes its confident claim" rule.
pub fn enumerate_hand_modelled_citations() -> Vec<HandModelledCitation> {
    HAND_MODELLED_FUNCTION_HOME_FILES
        .iter()
        .flat_map(|(file, text)| citations_in_source(file, text))
        .collect()
}

/// The scan itself, factored out of [`enumerate_hand_modelled_citations`] so its correctness can
/// be tested against small embedded fixtures (`tests::` below) instead of only against the huge
/// real source files — the same "test the discriminator on hand-labelled cases before trusting
/// it on the whole corpus" discipline `decisions.md` Decision 7 REFINED used for
/// `closure_states_universal_sheet_modifier`.
fn citations_in_source(file: &'static str, text: &str) -> Vec<HandModelledCitation> {
    let mut out = Vec::new();
    let mut doc_buf: Vec<&str> = Vec::new();
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if let Some(rest) = line.strip_prefix("///") {
            doc_buf.push(rest.trim());
            continue;
        }
        if line.starts_with("#[") {
            // an attribute (e.g. `#[allow(clippy::too_many_arguments)]`) may sit between the doc
            // comment and the `fn` line; do not let it break the association.
            continue;
        }
        if let Some(function_name) = parse_fn_name(line) {
            if let Some(cited_line) = doc_buf
                .iter()
                .find(|l| l.contains("BONUS:") || l.contains("DEFINE:"))
            {
                out.push(HandModelledCitation {
                    file,
                    function_name,
                    cited_line: (*cited_line).to_string(),
                });
            }
            doc_buf.clear();
        } else if line.is_empty() {
            // a blank line inside a doc block never happens for `///`; a blank line between two
            // unrelated items just means "no doc comment for whatever comes next" — clear either
            // way so a stale block never attaches to the wrong function.
            doc_buf.clear();
        } else {
            doc_buf.clear();
        }
    }
    out
}

/// Recognises `fn name(`, `pub fn name(`, `pub(super) fn name(`, `pub(crate) fn name(` at the
/// start of a (trimmed) line. Returns `None` for anything else, including lines that merely
/// contain `fn` mid-word (e.g. a doc line) since it requires the line to START with one of the
/// recognised prefixes.
fn parse_fn_name(line: &str) -> Option<String> {
    let idx = line.find("fn ")?;
    let prefix = line[..idx].trim();
    let prefix_ok = prefix.is_empty()
        || prefix == "pub"
        || (prefix.starts_with("pub(") && prefix.ends_with(')'));
    if !prefix_ok {
        return None;
    }
    let after = &line[idx + "fn ".len()..];
    let name_end = after.find('(')?;
    let name = after[..name_end].trim();
    if name.is_empty() || !name.chars().next()?.is_ascii_alphabetic() {
        return None;
    }
    Some(name.to_string())
}

// ---------------------------------------------------------------------------------------------
// 2. The interface contract for the interpreter-core lane
// ---------------------------------------------------------------------------------------------

/// The exact interface an interpreter-core evaluator implements to be checked by this harness.
///
/// `formula` is the raw RHS of a `BONUS:<TAG>|<VarName>|<formula>` token exactly as a corpus
/// `.lst` row states it (e.g. `"(CavalierLVL+2)/3"`), never rewritten by this harness. `vars`
/// supplies every PCGen identifier that formula's own text can reference for one sample point —
/// this harness always binds the case's own level/ability variable name(s) AND a fixed
/// `"__LEVEL__"` binding (for `classlevel("...")` calls, see the module doc's open question (b)).
pub trait FormulaEvaluator {
    fn evaluate(&self, formula: &str, vars: &BTreeMap<String, i64>) -> Result<i64, FormulaEvalError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormulaEvalError(pub String);

impl std::fmt::Display for FormulaEvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------------------------------------
// 3. Reproduction cases: hand function <-> corpus token, dispatched by name
// ---------------------------------------------------------------------------------------------

/// One character-level / ability-modifier sample point. All 21 dispatched cases below use at
/// most ONE ability score alongside level, so a single generic slot covers every one of them
/// (see the module doc's open question (b) for what this deliberately does not cover).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SamplePoint {
    pub level: u8,
    pub ability_mod: i64,
}

/// One hand-function-vs-corpus-token reproduction case. `hand_eval` calls the REAL hand-modelled
/// function, imported unmodified — this file never redefines or re-implements any of them.
pub struct ReproductionCase {
    pub name: &'static str,
    /// The PCGen token this case reproduces, e.g. `"BONUS:VAR|CavalierChallengeTimes|(CavalierLVL+2)/3"`.
    pub pcgen_token: &'static str,
    /// The formula text alone (the token's third `|`-segment) — what gets passed to
    /// [`FormulaEvaluator::evaluate`].
    pub raw_formula: &'static str,
    /// The identifier the formula uses for level, or `""` if the formula does not reference one.
    pub level_var: &'static str,
    /// The identifier the formula uses for an ability modifier, or `""` if none.
    pub ability_var: &'static str,
    /// Where the citation came from: `Doc(file)` if it is in `enumerate_hand_modelled_citations`,
    /// `KnownGapNoDocComment` for functions the mechanical scan cannot see (documented, not
    /// silently dropped — see the module doc).
    pub citation_source: CitationSource,
    pub hand_eval: fn(SamplePoint) -> i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CitationSource {
    Doc(&'static str),
    KnownGapNoDocComment,
}

/// Builds the variable bindings [`FormulaEvaluator::evaluate`] receives for one case/sample pair:
/// the case's own level/ability identifiers (if it uses them) plus the fixed `"__LEVEL__"` slot
/// every `classlevel("...")` call in this grammar resolves to.
pub fn vars_for(case: &ReproductionCase, sample: SamplePoint) -> BTreeMap<String, i64> {
    let mut vars = BTreeMap::new();
    vars.insert("__LEVEL__".to_string(), i64::from(sample.level));
    if !case.level_var.is_empty() {
        vars.insert(case.level_var.to_string(), i64::from(sample.level));
    }
    if !case.ability_var.is_empty() {
        vars.insert(case.ability_var.to_string(), sample.ability_mod);
    }
    // SD-32 T12 Epic 8 row 18 cycle 6: `formula_interpreter.rs`'s production evaluator now keys
    // `classlevel("X")` on `CLASSLEVEL::X`, never the class-blind `__LEVEL__` slot this harness
    // still also binds above (kept for `tests::ToyReferenceEvaluator`, this module's own
    // `#[cfg(test)]`-only comparison fixture, unaffected by that production change). Every
    // `classlevel("...")` call this harness's 21 dispatched cases carry self-references "the
    // record's own single class" (module doc open question (b)) -- so binding EVERY class name
    // literally named inside `raw_formula` to this same sample's level reproduces that self-
    // reference correctly without inventing a multiclass model this harness was never scoped to
    // cover.
    let mut rest = case.raw_formula;
    while let Some(start) = rest.find("classlevel(\"") {
        let after_open = &rest[start + "classlevel(\"".len()..];
        if let Some(end) = after_open.find('"') {
            let class_name = &after_open[..end];
            vars.insert(format!("CLASSLEVEL::{class_name}"), i64::from(sample.level));
            rest = &after_open[end + 1..];
        } else {
            break;
        }
    }
    vars
}

macro_rules! case {
    ($name:ident, $token:expr, $formula:expr, $level_var:expr, $ability_var:expr, $doc_file:expr, $hand_fn:expr) => {
        ReproductionCase {
            name: stringify!($name),
            pcgen_token: $token,
            raw_formula: $formula,
            level_var: $level_var,
            ability_var: $ability_var,
            citation_source: CitationSource::Doc($doc_file),
            hand_eval: $hand_fn,
        }
    };
}

/// 21 dispatched reproduction cases, spanning 8 classes (Cavalier, Alchemist, Slayer, Monk,
/// Inquisitor, Witch, Animal Companion, Summoner, Oracle) and every arithmetic shape this
/// harness's toy self-test grammar covers: flat passthrough, division, `max`, `floor`,
/// multiplication, stepped `if`, and `classlevel(...)`. Every `hand_eval` calls the real
/// `pilot_compute` function — none are reimplemented here. All 21 names are asserted, by test, to
/// be either present in [`enumerate_hand_modelled_citations`] or explicitly flagged
/// `KnownGapNoDocComment` — nothing is dispatched silently outside the enumerator's ledger.
///
/// This is 21 of the >=166 the mechanical scan finds (12.7%), not all of them — see the module's
/// own report on what the remaining ~87% looks like (piecewise level-gated sums, `DESC:`-only
/// facts with no `BONUS:` token at all, multi-token discrete grants, and spells-known lookup
/// tables), named honestly rather than silently omitted.
pub fn reproduction_cases() -> Vec<ReproductionCase> {
    vec![
        case!(
            cavalier_challenge_uses_per_day,
            "BONUS:VAR|CavalierChallengeTimes|(CavalierLVL+2)/3",
            "(CavalierLVL+2)/3",
            "CavalierLVL",
            "",
            "pilot_compute/mod.rs",
            |s| i64::from(super::cavalier_challenge_uses_per_day(s.level))
        ),
        case!(
            cavalier_bonus_combat_feat_count,
            "BONUS:ABILITYPOOL|Cavalier Feat|CavalierLVL/6",
            "CavalierLVL/6",
            "CavalierLVL",
            "",
            "pilot_compute/mod.rs",
            |s| i64::from(super::cavalier_bonus_combat_feat_count(s.level))
        ),
        case!(
            alchemist_bomb_damage_dice,
            "BONUS:VAR|AlchemistBombAdditionalDice|(AlchemistBombLVL-1)/2",
            "1+(AlchemistBombLVL-1)/2",
            "AlchemistBombLVL",
            "",
            "pilot_compute/mod.rs",
            |s| i64::from(super::alchemist_bomb_damage_dice(s.level))
        ),
        case!(
            alchemist_bomb_damage_bonus,
            "BONUS:VAR|AlchemistBombDamageBonus|INT",
            "INT",
            "",
            "INT",
            "pilot_compute/mod.rs",
            |s| super::alchemist_bomb_damage_bonus(s.ability_mod as i16) as i64
        ),
        case!(
            alchemist_bomb_dc,
            "BONUS:VAR|AlchemistBombDC|10+(AlchemistBombLVL/2)+INT",
            "10+(AlchemistBombLVL/2)+INT",
            "AlchemistBombLVL",
            "INT",
            "pilot_compute/mod.rs",
            |s| i64::from(super::alchemist_bomb_dc(s.level, s.ability_mod as i16))
        ),
        case!(
            slayer_sneak_attack_dice,
            "BONUS:VAR|SneakAttackDice|SlayerSneakAttackLVL/3",
            "SlayerSneakAttackLVL/3",
            "SlayerSneakAttackLVL",
            "",
            "pilot_compute/class_slayer.rs",
            |s| i64::from(super::slayer_sneak_attack_dice(s.level))
        ),
        case!(
            slayer_trap_sense_bonus,
            "BONUS:VAR|TrapSenseBonus|max(1,SlayerTrapSenseLVL/3)",
            "max(1,SlayerTrapSenseLVL/3)",
            "SlayerTrapSenseLVL",
            "",
            "pilot_compute/class_slayer.rs",
            |s| i64::from(super::slayer_trap_sense_bonus(s.level))
        ),
        case!(
            monk_high_jump_acrobatics_bonus,
            "BONUS:SITUATION|Acrobatics=When Jumping|HighJumpBonus (HighJumpLVL=MonkLVL)",
            "MonkLVL",
            "MonkLVL",
            "",
            "pilot_compute/mod.rs",
            |s| i64::from(super::monk_high_jump_acrobatics_bonus(s.level))
        ),
        case!(
            monk_wholeness_of_body_healing,
            "BONUS:VAR|WholenessOfBody|WholenessOfBodyLVL (=MonkLVL)",
            "MonkLVL",
            "MonkLVL",
            "",
            "pilot_compute/mod.rs",
            |s| i64::from(super::monk_wholeness_of_body_healing(s.level))
        ),
        case!(
            inquisitor_bane_pool_rounds,
            "BONUS:VAR|InquisitorBanePool|InquisitorLVL",
            "InquisitorLVL",
            "InquisitorLVL",
            "",
            "pilot_compute/mod.rs",
            |s| i64::from(super::inquisitor_bane_pool_rounds(s.level))
        ),
        case!(
            inquisitor_monster_lore_bonus,
            "BONUS:VAR|MonsterLoreBonus|WIS",
            "WIS",
            "",
            "WIS",
            "pilot_compute/mod.rs",
            |s| i64::from(super::inquisitor_monster_lore_bonus(s.ability_mod as i16))
        ),
        case!(
            inquisitor_cunning_initiative_bonus,
            "BONUS:COMBAT|INITIATIVE|WIS",
            "WIS",
            "",
            "WIS",
            "pilot_compute/mod.rs",
            |s| i64::from(super::inquisitor_cunning_initiative_bonus(s.ability_mod as i16))
        ),
        case!(
            witch_ward_bonus,
            "BONUS:VAR|WitchWardBonus|2 (+1 PREVARGTEQ:WitchHexAbilityLVL,8) (+1 PREVARGTEQ:WitchHexAbilityLVL,16)",
            "2+if(WitchLVL>=8,1,0)+if(WitchLVL>=16,1,0)",
            "WitchLVL",
            "",
            "pilot_compute/mod.rs",
            |s| i64::from(super::witch_ward_bonus(s.level))
        ),
        case!(
            animal_companion_natural_armor_bonus,
            "BONUS:COMBAT|AC|2*floor(MasterLevel/3)|TYPE=NaturalArmor.STACK",
            "2*floor(MasterLevel/3)",
            "MasterLevel",
            "",
            "pilot_compute/mod.rs",
            |s| i64::from(super::animal_companion_natural_armor_bonus(s.level))
        ),
        case!(
            animal_companion_stat_bonus,
            "BONUS:STAT|STR,DEX|floor(MasterLevel/3)",
            "floor(MasterLevel/3)",
            "MasterLevel",
            "",
            "pilot_compute/mod.rs",
            |s| i64::from(super::animal_companion_stat_bonus(s.level))
        ),
        case!(
            summoner_bond_senses_rounds_per_day,
            "BONUS:VAR|BondSensesRounds|classlevel(\"Summoner\")",
            "classlevel(\"Summoner\")",
            "",
            "",
            "pilot_compute/mod.rs",
            |s| i64::from(super::summoner_bond_senses_rounds_per_day(s.level))
        ),
        case!(
            summoner_makers_call_uses_per_day,
            "BONUS:VAR|MakersCallTimes|((classlevel(\"Summoner\")-2)/4)",
            "(classlevel(\"Summoner\")-2)/4",
            "",
            "",
            "pilot_compute/mod.rs",
            |s| i64::from(super::summoner_makers_call_uses_per_day(s.level))
        ),
        case!(
            summoner_merge_forms_rounds_per_day,
            "BONUS:VAR|MergeFormsRounds|classlevel(\"Summoner\")",
            "classlevel(\"Summoner\")",
            "",
            "",
            "pilot_compute/mod.rs",
            |s| i64::from(super::summoner_merge_forms_rounds_per_day(s.level))
        ),
        case!(
            summoner_twin_eidolon_minutes_per_day,
            "BONUS:VAR|TwinEidolonMinutes|classlevel(\"Summoner\")",
            "classlevel(\"Summoner\")",
            "",
            "",
            "pilot_compute/mod.rs",
            |s| i64::from(super::summoner_twin_eidolon_minutes_per_day(s.level))
        ),
        case!(
            summoner_summon_monster_duration_minutes,
            "BONUS:VAR|SummonMonsterDuration|classlevel(\"Summoner\")",
            "classlevel(\"Summoner\")",
            "",
            "",
            "pilot_compute/mod.rs",
            |s| i64::from(super::summoner_summon_monster_duration_minutes(s.level))
        ),
        case!(
            oracle_battlecry_bonus,
            "BONUS:VAR|OracleBattlecryBonus|1 (+1 PRECLASS:1,Oracle=10)",
            "if(OracleLVL>=10,2,1)",
            "OracleLVL",
            "",
            "pilot_compute/mod.rs",
            |s| i64::from(super::oracle_battlecry_bonus(s.level))
        ),
    ]
}

/// `monk_scorpion_style_dc` (`mod.rs:33563`) — operator-named in `OPERATOR-RULINGS-2026-08-21.md`
/// §20 itself ("`monk_scorpion_style_dc` and siblings") as one of the ~27-classes-worth of proof
/// this ruling rests on — but it carries NO doc comment at all, so
/// [`enumerate_hand_modelled_citations`] cannot see it. Dispatched separately, with its citation
/// source honestly marked [`CitationSource::KnownGapNoDocComment`] rather than folded silently
/// into the doc-cited 21, so the mechanical enumerator's known blind spot has a live witness a
/// test can assert against (`tests::the_enumerator_has_a_known_blind_spot`).
pub fn scorpion_style_case() -> ReproductionCase {
    ReproductionCase {
        // Verified directly against the oracle (`core_rulebook/cr_feats.lst:142`,
        // `KEY:Scorpion Style` -- not a `BONUS:` token but the feat's own `BENEFIT:` DC
        // substitution parameter): `10+(TL/2)+WIS`. `TL` is PCGen's TOTAL character level, not
        // a per-class `classlevel(...)` — the hand function's `level` parameter is only correct
        // here because its only caller (`mod.rs:35005`) is a single-classed-Monk code path where
        // total level and Monk level coincide; a multiclassed caller would silently diverge.
        // Worth flagging as a finding for the interpreter-core lane, not something this harness
        // fixes.
        name: "monk_scorpion_style_dc",
        pcgen_token: "BENEFIT:...DC %1.|10+(TL/2)+WIS (Scorpion Style feat, core_rulebook/cr_feats.lst:142)",
        raw_formula: "10+(TL/2)+WIS",
        level_var: "TL",
        ability_var: "WIS",
        citation_source: CitationSource::KnownGapNoDocComment,
        hand_eval: |s| i64::from(super::monk_scorpion_style_dc(s.level, s.ability_mod as i16)),
    }
}

/// All 22 dispatched cases (21 doc-cited + the 1 known-gap exemplar), the set
/// [`run_reproduction`] is meant to be called with.
pub fn all_cases() -> Vec<ReproductionCase> {
    let mut cases = reproduction_cases();
    cases.push(scorpion_style_case());
    cases
}

// ---------------------------------------------------------------------------------------------
// 4. The comparison harness
// ---------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Disagreement {
    pub case_name: &'static str,
    pub sample: SamplePoint,
    pub hand_value: i64,
    pub evaluator_result: Result<i64, FormulaEvalError>,
}

#[derive(Debug, Clone)]
pub struct CaseOutcome {
    pub case_name: &'static str,
    pub samples_checked: usize,
    pub disagreements: Vec<Disagreement>,
}

impl CaseOutcome {
    pub fn agrees_everywhere(&self) -> bool {
        self.disagreements.is_empty()
    }
}

/// A real range of character levels (1..=20, PF1's full progression) and ability modifiers
/// (-5..=10, score 1 through score 30) — "a real range of character levels and ability scores"
/// per the dispatch brief. Level 0 and negative-operand division are deliberately excluded; see
/// the module doc's open question (a).
pub fn default_levels() -> Vec<u8> {
    (1..=20u8).collect()
}

pub fn default_ability_mods() -> Vec<i64> {
    (-5..=10i64).collect()
}

/// For every case, evaluate the SAME formula through both the real hand-modelled function and
/// the supplied [`FormulaEvaluator`] across the full level x ability-modifier grid, and report
/// every point of disagreement. Never adjusts either side — a disagreement is returned, not
/// resolved.
pub fn run_reproduction(
    evaluator: &dyn FormulaEvaluator,
    cases: &[ReproductionCase],
    levels: &[u8],
    ability_mods: &[i64],
) -> Vec<CaseOutcome> {
    cases
        .iter()
        .map(|case| {
            let mut disagreements = Vec::new();
            let mut samples_checked = 0usize;
            for &level in levels {
                for &ability_mod in ability_mods {
                    let sample = SamplePoint { level, ability_mod };
                    let hand_value = (case.hand_eval)(sample);
                    let vars = vars_for(case, sample);
                    let evaluator_result = evaluator.evaluate(case.raw_formula, &vars);
                    samples_checked += 1;
                    let agrees = matches!(&evaluator_result, Ok(v) if *v == hand_value);
                    if !agrees {
                        disagreements.push(Disagreement {
                            case_name: case.name,
                            sample,
                            hand_value,
                            evaluator_result,
                        });
                    }
                }
            }
            CaseOutcome {
                case_name: case.name,
                samples_checked,
                disagreements,
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ReproductionSummary {
    pub cases_total: usize,
    pub cases_fully_agree: usize,
    pub cases_with_disagreement: usize,
    pub samples_total: usize,
    pub samples_disagreeing: usize,
}

pub fn summarize(outcomes: &[CaseOutcome]) -> ReproductionSummary {
    let mut s = ReproductionSummary {
        cases_total: outcomes.len(),
        ..Default::default()
    };
    for o in outcomes {
        s.samples_total += o.samples_checked;
        s.samples_disagreeing += o.disagreements.len();
        if o.agrees_everywhere() {
            s.cases_fully_agree += 1;
        } else {
            s.cases_with_disagreement += 1;
        }
    }
    s
}

// ---------------------------------------------------------------------------------------------
// 5. Tests: enumeration correctness, dispatch-table/enumeration cross-check, and the harness's
//    own self-proof via a test-only toy evaluator.
// ---------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- 5a. enumeration correctness, against small embedded fixtures (not the real 67k-line
    //    file), so a change to the scanner's own logic is provable independent of the real
    //    corpus of functions ------------------------------------------------------------------

    #[test]
    fn enumeration_finds_a_doc_cited_function_and_skips_an_undoc_cited_one() {
        let fixture = "\
/// verified against `BONUS:VAR|Foo|Bar/2`.
fn foo(level: u8) -> i16 {
    i16::from(level) / 2
}

fn bar_with_no_doc(level: u8) -> i16 {
    i16::from(level)
}

/// a doc comment that never mentions a PCGen token at all.
fn baz_doc_but_no_token(level: u8) -> i16 {
    i16::from(level)
}
";
        let found = citations_in_source("fixture.rs", fixture);
        let names: Vec<&str> = found.iter().map(|c| c.function_name.as_str()).collect();
        assert_eq!(names, vec!["foo"], "only the doc-cited function should be found");
    }

    #[test]
    fn enumeration_survives_an_attribute_between_doc_and_fn() {
        let fixture = "\
/// verified against `BONUS:VAR|Foo|Bar/2`.
#[allow(clippy::too_many_arguments)]
pub(super) fn foo(level: u8) -> i16 {
    i16::from(level) / 2
}
";
        let found = citations_in_source("fixture.rs", fixture);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].function_name, "foo");
    }

    #[test]
    fn enumeration_is_mechanical_not_a_fixed_number() {
        // Removing the citation (mutating the fixture, not any real hand-modelled function)
        // must shrink the count by exactly one -- proof this is a live scan, not a constant.
        let with_citation = "\
/// verified against `BONUS:VAR|Foo|Bar/2`.
fn foo(level: u8) -> i16 { i16::from(level) / 2 }
";
        let without_citation = "\
/// just a description, no PCGen token here.
fn foo(level: u8) -> i16 { i16::from(level) / 2 }
";
        assert_eq!(citations_in_source("f.rs", with_citation).len(), 1);
        assert_eq!(citations_in_source("f.rs", without_citation).len(), 0);
    }

    #[test]
    fn live_enumeration_over_the_real_pilot_compute_source_is_a_real_and_substantial_population() {
        let found = enumerate_hand_modelled_citations();
        // A floor, re-derived at the time this module was written (166) with slack for drift in
        // either direction as the corpus grows; the exact number belongs in the cycle's own
        // progress receipt, re-run at report time, never hand-transcribed here as a constant to
        // assert equality against (that would silently stop tracking the real file).
        assert!(
            found.len() >= 120,
            "expected at least 120 doc-cited hand-modelled functions in pilot_compute, found {}",
            found.len()
        );
        // Positive control: a case this file dispatches must actually be in the live scan.
        assert!(
            found
                .iter()
                .any(|c| c.function_name == "cavalier_challenge_uses_per_day"),
            "the scan must find a function known to carry a BONUS: doc citation"
        );
    }

    #[test]
    fn the_enumerator_has_a_known_blind_spot() {
        // `monk_scorpion_style_dc` is operator-named in OPERATOR-RULINGS-2026-08-21.md itself as
        // a ground-truth exemplar, but carries no doc comment at all -- so the mechanical scan
        // (correctly, per its own stated rule) does not find it. Documented here, not silently
        // dropped: `enumerate_hand_modelled_citations`'s count is a FLOOR, not a census.
        let found = enumerate_hand_modelled_citations();
        assert!(
            !found.iter().any(|c| c.function_name == "monk_scorpion_style_dc"),
            "if this now passes, monk_scorpion_style_dc gained a doc comment -- move it out of \
             scorpion_style_case()'s KnownGapNoDocComment citation and into reproduction_cases()"
        );
    }

    // -- 5b. every dispatched case's citation-source claim is honest -------------------------

    #[test]
    fn every_doc_cited_dispatch_case_really_is_in_the_live_enumeration() {
        let live = enumerate_hand_modelled_citations();
        let live_names: std::collections::BTreeSet<&str> =
            live.iter().map(|c| c.function_name.as_str()).collect();
        for case in reproduction_cases() {
            match case.citation_source {
                CitationSource::Doc(_) => assert!(
                    live_names.contains(case.name),
                    "case {} claims Doc(..) citation but is not in enumerate_hand_modelled_citations()",
                    case.name
                ),
                CitationSource::KnownGapNoDocComment => panic!(
                    "case {} is in reproduction_cases() but marked KnownGapNoDocComment -- \
                     move it to scorpion_style_case()-style dispatch",
                    case.name
                ),
            }
        }
    }

    #[test]
    fn all_cases_names_are_unique() {
        let cases = all_cases();
        let mut names: Vec<&str> = cases.iter().map(|c| c.name).collect();
        let before = names.len();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), before, "duplicate case name in the dispatch table");
    }

    // -- 5c. spot-check a handful of hand_eval closures against manually worked values, so a
    //    typo in the dispatch table (not the real hand-modelled function) is caught -----------

    #[test]
    fn hand_eval_closures_match_manually_worked_values() {
        let by_name = |name: &str| {
            reproduction_cases()
                .into_iter()
                .find(|c| c.name == name)
                .unwrap_or_else(|| panic!("no case named {name}"))
        };
        assert_eq!(
            (by_name("cavalier_challenge_uses_per_day").hand_eval)(SamplePoint {
                level: 6,
                ability_mod: 0
            }),
            (6 + 2) / 3
        );
        assert_eq!(
            (by_name("monk_high_jump_acrobatics_bonus").hand_eval)(SamplePoint {
                level: 9,
                ability_mod: 0
            }),
            9
        );
        let oracle_battlecry = by_name("oracle_battlecry_bonus");
        assert_eq!((oracle_battlecry.hand_eval)(SamplePoint { level: 9, ability_mod: 0 }), 1);
        assert_eq!((oracle_battlecry.hand_eval)(SamplePoint { level: 10, ability_mod: 0 }), 2);
    }

    // -- 5d. a small, real, test-only formula evaluator, for the harness's self-proof only ---

    /// NOT the production interpreter. A minimal recursive-descent evaluator for exactly the
    /// grammar the 22 dispatched cases above use (`+ - * /`, `max`, `min`, `floor`, `if`,
    /// `classlevel("...")`, integer literals and identifiers). Exists solely so this module's own
    /// comparison/reporting logic ([`run_reproduction`]) can be proven correct, per Decision
    /// 1(a)'s "mutate what your gate guards and prove it goes RED" — agreement here is NOT
    /// evidence the real PCGen interpreter will agree with the hand functions.
    struct ToyReferenceEvaluator;

    impl FormulaEvaluator for ToyReferenceEvaluator {
        fn evaluate(&self, formula: &str, vars: &BTreeMap<String, i64>) -> Result<i64, FormulaEvalError> {
            let tokens = tokenize(formula)?;
            let mut p = Parser { tokens: &tokens, pos: 0, vars };
            let v = p.parse_expr()?;
            if p.pos != p.tokens.len() {
                return Err(FormulaEvalError(format!("trailing tokens in {formula:?}")));
            }
            Ok(v)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    enum Tok {
        Num(i64),
        Ident(String),
        Str(String),
        Plus,
        Minus,
        Star,
        Slash,
        LParen,
        RParen,
        Comma,
        Ge,
        Le,
        Eq,
        Gt,
        Lt,
    }

    fn tokenize(s: &str) -> Result<Vec<Tok>, FormulaEvalError> {
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut out = Vec::new();
        while i < chars.len() {
            let c = chars[i];
            match c {
                ' ' | '\t' => i += 1,
                '+' => {
                    out.push(Tok::Plus);
                    i += 1;
                }
                '-' => {
                    out.push(Tok::Minus);
                    i += 1;
                }
                '*' => {
                    out.push(Tok::Star);
                    i += 1;
                }
                '/' => {
                    out.push(Tok::Slash);
                    i += 1;
                }
                '(' => {
                    out.push(Tok::LParen);
                    i += 1;
                }
                ')' => {
                    out.push(Tok::RParen);
                    i += 1;
                }
                ',' => {
                    out.push(Tok::Comma);
                    i += 1;
                }
                '>' => {
                    if chars.get(i + 1) == Some(&'=') {
                        out.push(Tok::Ge);
                        i += 2;
                    } else {
                        out.push(Tok::Gt);
                        i += 1;
                    }
                }
                '<' => {
                    if chars.get(i + 1) == Some(&'=') {
                        out.push(Tok::Le);
                        i += 2;
                    } else {
                        out.push(Tok::Lt);
                        i += 1;
                    }
                }
                '=' => {
                    if chars.get(i + 1) == Some(&'=') {
                        out.push(Tok::Eq);
                        i += 2;
                    } else {
                        return Err(FormulaEvalError(format!("bare '=' in {s:?}")));
                    }
                }
                '"' => {
                    let mut j = i + 1;
                    let mut buf = String::new();
                    while j < chars.len() && chars[j] != '"' {
                        buf.push(chars[j]);
                        j += 1;
                    }
                    if j >= chars.len() {
                        return Err(FormulaEvalError(format!("unterminated string in {s:?}")));
                    }
                    out.push(Tok::Str(buf));
                    i = j + 1;
                }
                c if c.is_ascii_digit() => {
                    let mut j = i;
                    while j < chars.len() && chars[j].is_ascii_digit() {
                        j += 1;
                    }
                    let n: String = chars[i..j].iter().collect();
                    out.push(Tok::Num(n.parse().map_err(|_| {
                        FormulaEvalError(format!("bad number in {s:?}"))
                    })?));
                    i = j;
                }
                c if c.is_ascii_alphabetic() || c == '_' => {
                    let mut j = i;
                    while j < chars.len() && (chars[j].is_ascii_alphanumeric() || chars[j] == '_') {
                        j += 1;
                    }
                    out.push(Tok::Ident(chars[i..j].iter().collect()));
                    i = j;
                }
                other => return Err(FormulaEvalError(format!("unexpected char {other:?} in {s:?}"))),
            }
        }
        Ok(out)
    }

    struct Parser<'a> {
        tokens: &'a [Tok],
        pos: usize,
        vars: &'a BTreeMap<String, i64>,
    }

    impl<'a> Parser<'a> {
        fn peek(&self) -> Option<&Tok> {
            self.tokens.get(self.pos)
        }

        fn bump(&mut self) -> Option<&Tok> {
            let t = self.tokens.get(self.pos);
            self.pos += 1;
            t
        }

        fn expect(&mut self, want: &Tok) -> Result<(), FormulaEvalError> {
            match self.bump() {
                Some(t) if t == want => Ok(()),
                other => Err(FormulaEvalError(format!("expected {want:?}, got {other:?}"))),
            }
        }

        fn parse_expr(&mut self) -> Result<i64, FormulaEvalError> {
            let mut v = self.parse_term()?;
            loop {
                match self.peek() {
                    Some(Tok::Plus) => {
                        self.bump();
                        v += self.parse_term()?;
                    }
                    Some(Tok::Minus) => {
                        self.bump();
                        v -= self.parse_term()?;
                    }
                    _ => break,
                }
            }
            Ok(v)
        }

        fn parse_term(&mut self) -> Result<i64, FormulaEvalError> {
            let mut v = self.parse_unary()?;
            loop {
                match self.peek() {
                    Some(Tok::Star) => {
                        self.bump();
                        v *= self.parse_unary()?;
                    }
                    Some(Tok::Slash) => {
                        self.bump();
                        let rhs = self.parse_unary()?;
                        if rhs == 0 {
                            return Err(FormulaEvalError("division by zero".to_string()));
                        }
                        v /= rhs; // Rust `/` truncates toward zero -- see module doc open question (a).
                    }
                    _ => break,
                }
            }
            Ok(v)
        }

        fn parse_unary(&mut self) -> Result<i64, FormulaEvalError> {
            if let Some(Tok::Minus) = self.peek() {
                self.bump();
                return Ok(-self.parse_unary()?);
            }
            self.parse_primary()
        }

        fn parse_primary(&mut self) -> Result<i64, FormulaEvalError> {
            match self.bump().cloned() {
                Some(Tok::Num(n)) => Ok(n),
                Some(Tok::LParen) => {
                    let v = self.parse_expr()?;
                    self.expect(&Tok::RParen)?;
                    Ok(v)
                }
                Some(Tok::Ident(name)) => {
                    if let Some(Tok::LParen) = self.peek() {
                        self.bump();
                        self.parse_call(&name)
                    } else {
                        self.vars.get(&name).copied().ok_or_else(|| {
                            FormulaEvalError(format!("unbound variable {name:?}"))
                        })
                    }
                }
                other => Err(FormulaEvalError(format!("unexpected token {other:?}"))),
            }
        }

        fn parse_call(&mut self, name: &str) -> Result<i64, FormulaEvalError> {
            match name {
                "max" => {
                    let a = self.parse_expr()?;
                    self.expect(&Tok::Comma)?;
                    let b = self.parse_expr()?;
                    self.expect(&Tok::RParen)?;
                    Ok(a.max(b))
                }
                "min" => {
                    let a = self.parse_expr()?;
                    self.expect(&Tok::Comma)?;
                    let b = self.parse_expr()?;
                    self.expect(&Tok::RParen)?;
                    Ok(a.min(b))
                }
                "floor" => {
                    let a = self.parse_expr()?;
                    self.expect(&Tok::RParen)?;
                    Ok(a) // integer already, `/` above already truncates -- see open question (a).
                }
                "if" => {
                    let cond = self.parse_cond()?;
                    self.expect(&Tok::Comma)?;
                    let a = self.parse_expr()?;
                    self.expect(&Tok::Comma)?;
                    let b = self.parse_expr()?;
                    self.expect(&Tok::RParen)?;
                    Ok(if cond { a } else { b })
                }
                "classlevel" => {
                    match self.bump() {
                        Some(Tok::Str(_)) => {}
                        other => {
                            return Err(FormulaEvalError(format!(
                                "classlevel(...) expects a string literal, got {other:?}"
                            )))
                        }
                    }
                    self.expect(&Tok::RParen)?;
                    self.vars.get("__LEVEL__").copied().ok_or_else(|| {
                        FormulaEvalError("classlevel(...) needs __LEVEL__ bound".to_string())
                    })
                }
                other => Err(FormulaEvalError(format!("unsupported function {other:?}"))),
            }
        }

        fn parse_cond(&mut self) -> Result<bool, FormulaEvalError> {
            let a = self.parse_expr()?;
            let op = self.bump().cloned();
            let b = self.parse_expr()?;
            match op {
                Some(Tok::Ge) => Ok(a >= b),
                Some(Tok::Le) => Ok(a <= b),
                Some(Tok::Eq) => Ok(a == b),
                Some(Tok::Gt) => Ok(a > b),
                Some(Tok::Lt) => Ok(a < b),
                other => Err(FormulaEvalError(format!("expected a comparison operator, got {other:?}"))),
            }
        }
    }

    #[test]
    fn toy_evaluator_parses_the_dispatched_grammar_shapes_directly() {
        let mut vars = BTreeMap::new();
        vars.insert("X".to_string(), 7i64);
        vars.insert("INT".to_string(), 3i64);
        vars.insert("__LEVEL__".to_string(), 7i64);
        let e = ToyReferenceEvaluator;
        assert_eq!(e.evaluate("(X+2)/3", &vars).unwrap(), 3);
        assert_eq!(e.evaluate("max(1,X/3)", &vars).unwrap(), 2);
        assert_eq!(e.evaluate("2*floor(X/3)", &vars).unwrap(), 4);
        assert_eq!(e.evaluate("if(X>=8,1,0)", &vars).unwrap(), 0);
        assert_eq!(e.evaluate("if(X>=7,1,0)", &vars).unwrap(), 1);
        assert_eq!(e.evaluate("classlevel(\"Summoner\")", &vars).unwrap(), 7);
        assert_eq!(e.evaluate("10+(X/2)+INT", &vars).unwrap(), 16);
    }

    // -- 5e. the harness's self-proof: agreement across the whole grid, and a MUTATION test ---
    //    that proves disagreement is actually caught, per Decision 1(a) ------------------------

    #[test]
    fn toy_evaluator_reproduces_every_dispatched_case_exactly() {
        let evaluator = ToyReferenceEvaluator;
        let cases = all_cases();
        let outcomes = run_reproduction(
            &evaluator,
            &cases,
            &default_levels(),
            &default_ability_mods(),
        );
        let summary = summarize(&outcomes);
        for outcome in &outcomes {
            assert!(
                outcome.agrees_everywhere(),
                "case {} disagreed with the toy evaluator at {} of {} samples: first = {:?}",
                outcome.case_name,
                outcome.disagreements.len(),
                outcome.samples_checked,
                outcome.disagreements.first()
            );
        }
        assert_eq!(summary.cases_total, 22);
        assert_eq!(summary.cases_fully_agree, 22);
        assert_eq!(summary.samples_disagreeing, 0);
        assert_eq!(summary.samples_total, 22 * 20 * 16); // 22 cases x 20 levels x 16 ability mods
    }

    /// Mutation-proves the comparison logic itself: an evaluator that is deliberately wrong for
    /// ONE case must be caught by [`run_reproduction`], not silently pass. This is the "mutate
    /// what your gate guards and prove it goes RED" check Decision 1(a) requires of any gate.
    struct BuggyEvaluator {
        inner: ToyReferenceEvaluator,
        break_case_formula: &'static str,
    }

    impl FormulaEvaluator for BuggyEvaluator {
        fn evaluate(&self, formula: &str, vars: &BTreeMap<String, i64>) -> Result<i64, FormulaEvalError> {
            let real = self.inner.evaluate(formula, vars)?;
            if formula == self.break_case_formula {
                Ok(real + 1) // deliberately wrong by one
            } else {
                Ok(real)
            }
        }
    }

    #[test]
    fn harness_detects_a_deliberately_wrong_evaluator() {
        let buggy = BuggyEvaluator {
            inner: ToyReferenceEvaluator,
            break_case_formula: "(CavalierLVL+2)/3",
        };
        let cases = all_cases();
        let outcomes = run_reproduction(&buggy, &cases, &default_levels(), &default_ability_mods());
        let broken = outcomes
            .iter()
            .find(|o| o.case_name == "cavalier_challenge_uses_per_day")
            .expect("the broken case must be in the outcome set");
        assert!(
            !broken.agrees_everywhere(),
            "a deliberately wrong evaluator must be caught as a disagreement, not pass silently"
        );
        assert_eq!(broken.disagreements.len(), broken.samples_checked, "wrong on every sample");
        // every OTHER case must still agree everywhere -- the mutation is scoped to one formula.
        for o in &outcomes {
            if o.case_name != "cavalier_challenge_uses_per_day" {
                assert!(o.agrees_everywhere(), "case {} should be unaffected by the mutation", o.case_name);
            }
        }
    }
}
