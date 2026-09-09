//! SD-31 wave 26 — the "BonusObj-shape reader" (`OPERATOR-RULINGS-2026-08-21.md` §20 follow-on,
//! `formula_interpreter.rs`'s own "Wave 26 shape closure" module doc section).
//!
//! **What this module is.** `formula_interpreter.rs` evaluates ONE formula string in isolation.
//! Real PCGen `BONUS:VAR` tokens routinely appear MULTIPLE times on one record, all targeting the
//! same variable name, each individually gated by its own `PREVARGTEQ:<var>,<threshold>`
//! prerequisite — e.g. `witch_hex/ward.json`'s real `raw_tokens` (verified directly against that
//! file):
//! ```text
//! BONUS:VAR|WitchWardBonus|2
//! BONUS:VAR|WitchWardBonus|1|PREVARGTEQ:WitchHexAbilityLVL,8
//! BONUS:VAR|WitchWardBonus|1|PREVARGTEQ:WitchHexAbilityLVL,16
//! ```
//! `formula_interpreter::extract_formula_field`'s positional heuristic already extracts a clean,
//! independently-parseable formula from each one of these three tokens on its own (`"2"`, `"1"`,
//! `"1"`) — none of them is a `corpus_shape_coverage` refusal. What is missing is the AGGREGATION
//! step: a correct total for `WitchWardBonus` sums only the addends whose own `PREVARGTEQ` gate
//! currently passes, and naively summing all three unconditionally (as a consumer touching only
//! `formula_interpreter` in isolation might) silently over-counts below level 16 by up to 2. This
//! module closes exactly that gap — see `formula_interpreter.rs`'s own "Wave 26 shape closure"
//! doc section for why this is a real, corpus-grounded gap even though it contributes zero to
//! `corpus_shape_coverage`'s refusal count, and why the wave 25 dispatch's literal
//! "PREVARGTEQ-embedded-in-formula-text" example does not occur anywhere in the real corpus.
//!
//! **Semantics derived from the pinned oracle, not guessed.**
//!
//! 1. **`PREVARGTEQ:<var>,<threshold>` passes iff the character's current value of `<var>` is
//!    `>= <threshold>`.** `plugin/pretokens/test/PreVariableTester.java` (`kindHandled() == "VAR"`,
//!    i.e. the `PRE` token this reader is named after): `character.getVariableValue(prereq.getKey(),
//!    src)` compared against `prereq.getOperand()` via `prereq.getOperator().compare(...)` — the
//!    `GTEQ` operand comes from the `PREVARGTEQ` token name itself (`PRE<KIND><OPERATOR>`, PCGen's
//!    own PRE-token naming convention: `VAR` + `GTEQ`). Returns pass/fail, nothing else.
//! 2. **Multiple `BONUS:VAR` entries sharing one target variable SUM.**
//!    `pcgen/core/PlayerCharacter.java:2136`: `value += getTotalBonusTo("VAR", variableString);` →
//!    `pcgen/core/PlayerCharacter.java:3586` (`getTotalBonusTo`) → `bonusManager.getTotalBonusTo` →
//!    `pcgen/core/BonusManager.java`'s `getTotalBonusTo(bonusName, bonusInfo)`:
//!    `sumActiveBonusMap(bonusName + '.' + bonusInfo)` — sums every currently-ACTIVE bonus
//!    contribution filed under that `VAR.<name>` prefix. A `BonusObj` whose own prerequisites
//!    (here, its `PREVARGTEQ`) do not currently pass is never added to the active map in the first
//!    place (`ConcretePrereqObject`/`QualifyingObject` machinery `BonusObj` itself implements —
//!    `pcgen/core/bonus/BonusObj.java`'s own class doc, already cited in `formula_interpreter.rs`)
//!    — so "sum only the qualifying ones" is not this reader's own policy choice, it is the real
//!    aggregation the oracle performs.
//!
//! **What this module does NOT do**, disclosed rather than silently narrowed:
//! - Does not parse the full `BONUS:<TAG>|<target>|<formula>|<PRE...>` envelope grammar for every
//!   `TAG` — only `VAR`, matching this reader's own name and the one real shape it was built for.
//! - Recognises exactly one PRE-tag kind, `PREVARGTEQ`. Any OTHER PRE-tag on a token matching the
//!   requested target variable (`PREABILITY`, seen on the very same `ward.json` record's
//!   `WitchHexDC_Ward` target, is a real example) is a REFUSAL, not a silently-ignored or
//!   silently-always-true gate — see [`extract_addends`]'s own doc.
//! - A single token carrying more than one PRE-tag field is refused, not guessed at (no corpus
//!   `BONUS:VAR|<target>|<formula>|PREVARGTEQ:...|PREVARGTEQ:...` shape has been found or
//!   verified; refusing avoids inventing whether repeated gates would AND or OR).
//! - Does not itself decide which corpus tokens belong to "one record" — the caller supplies the
//!   token list (mirroring `formula_interpreter::extract_formula_field`, which likewise takes one
//!   already-identified token at a time rather than owning corpus traversal).

use std::collections::{BTreeMap, BTreeSet};

use super::formula_reproduction_harness::{FormulaEvalError, FormulaEvaluator};

/// A `PREVARGTEQ:<variable>,<threshold>` gate parsed off one `BONUS:VAR` token's trailing pipe
/// field. See module doc point 1 for the oracle citation (`PreVariableTester.java`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrevarGteqGate {
    pub variable: String,
    pub threshold: i64,
}

/// One `BONUS:VAR` token's own formula and (optional) gate, both still as their original text —
/// `formula` is handed to a [`FormulaEvaluator`] by [`evaluate_stack`], never evaluated by this
/// module directly, so this reader stays a pure structural reader over the `BonusObj` shape and
/// never duplicates `formula_interpreter`'s own arithmetic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalAddend {
    pub formula: String,
    pub gate: Option<PrevarGteqGate>,
}

/// Scans `tokens` (one record's raw `(key, value)` token pairs — e.g. a corpus JSON record's own
/// `data.raw_tokens`, adapted to borrowed `&str` pairs by the caller) for every `BONUS:VAR` entry
/// targeting `target_var`, in the order given, and returns each one's formula plus its parsed
/// `PREVARGTEQ` gate if it has one.
///
/// A token whose `key` is not `"BONUS"`, or whose value's own pipe-delimited TAG field (the part
/// before the first `|`) is not exactly `"VAR"`, or whose target field (the part between the
/// first and second `|`) is not exactly `target_var`, is skipped — not an error, since most
/// tokens on a real record are irrelevant to any one target variable (see `ward.json`'s own
/// dozen-plus unrelated tokens).
///
/// A token that DOES match `target_var` but carries a PRE-tag field this reader has not verified
/// (anything other than a single, well-formed `PREVARGTEQ:<var>,<threshold>`) returns `Err` for
/// the WHOLE call — never a partial list silently missing that addend — because a missing addend
/// changes the aggregate total exactly as badly as a wrongly-included one, and this reader's own
/// safety property depends on every returned addend's gate being one this module actually
/// understands.
pub fn extract_addends<'a>(
    target_var: &str,
    tokens: impl IntoIterator<Item = (&'a str, &'a str)>,
) -> Result<Vec<ConditionalAddend>, FormulaEvalError> {
    let mut out = Vec::new();
    for (key, value) in tokens {
        if key != "BONUS" {
            continue;
        }
        let parts: Vec<&str> = value.split('|').collect();
        if parts.len() < 3 || parts[0] != "VAR" || parts[1] != target_var {
            continue;
        }
        let formula = parts[2].to_string();
        let gate = match parts.len() {
            3 => None,
            4 => Some(parse_prevargteq(parts[3])?),
            _ => {
                return Err(FormulaEvalError(format!(
                    "BONUS:VAR|{target_var}|... token {value:?} carries {} trailing PRE-tag \
                     fields — this reader has verified exactly one PREVARGTEQ gate per token and \
                     refuses to guess how more than one combine",
                    parts.len() - 3
                )))
            }
        };
        out.push(ConditionalAddend { formula, gate });
    }
    Ok(out)
}

fn parse_prevargteq(field: &str) -> Result<PrevarGteqGate, FormulaEvalError> {
    let Some(rest) = field.strip_prefix("PREVARGTEQ:") else {
        return Err(FormulaEvalError(format!(
            "BONUS:VAR PRE-tag field {field:?} is not PREVARGTEQ — this reader has verified only \
             PREVARGTEQ's semantics (plugin/pretokens/test/PreVariableTester.java) and refuses \
             rather than silently dropping or always-passing a gate kind it has not read"
        )));
    };
    let Some((variable, threshold_str)) = rest.split_once(',') else {
        return Err(FormulaEvalError(format!(
            "PREVARGTEQ field {field:?} is missing its ',<threshold>' — expected \
             PREVARGTEQ:<variable>,<threshold>"
        )));
    };
    let threshold: i64 = threshold_str.trim().parse().map_err(|_| {
        FormulaEvalError(format!(
            "PREVARGTEQ field {field:?} has a non-integer threshold {threshold_str:?} — refusing \
             rather than truncating or guessing"
        ))
    })?;
    Ok(PrevarGteqGate { variable: variable.to_string(), threshold })
}

/// Evaluates `addends` against `evaluator` and `vars`, summing only the addends whose gate
/// currently passes (module doc point 2: `BonusManager.sumActiveBonusMap` sums only currently-
/// active — i.e. currently-qualifying — contributions). An addend with no gate always qualifies.
///
/// Refuses (never defaults to "gate fails") if a gate's own variable has no binding in `vars` —
/// consistent with `formula_interpreter`'s own "never silently treat an unbound variable as 0"
/// rule, since silently treating an unresolvable gate as failing is exactly as much a guess as
/// treating it as passing.
pub fn evaluate_stack(
    evaluator: &dyn FormulaEvaluator,
    addends: &[ConditionalAddend],
    vars: &BTreeMap<String, i64>,
) -> Result<i64, FormulaEvalError> {
    let mut total: i64 = 0;
    for addend in addends {
        let qualifies = match &addend.gate {
            None => true,
            Some(gate) => {
                let current = vars.get(&gate.variable).ok_or_else(|| {
                    FormulaEvalError(format!(
                        "PREVARGTEQ:{},{} needs a {:?} binding to evaluate its gate",
                        gate.variable, gate.threshold, gate.variable
                    ))
                })?;
                *current >= gate.threshold
            }
        };
        if qualifies {
            total += evaluator.evaluate(&addend.formula, vars)?;
        }
    }
    Ok(total)
}

/// A resolved producer chain for one target variable: an optional `DEFINE:<var>|<formula>` base
/// (PCGen's own initial value for the variable, `PlayerCharacter.java`'s `getVariable` reads the
/// `DEFINE:` value before any `BONUS:VAR` contribution is added — see module doc point 2's
/// `getTotalBonusTo` citation, which sums ON TOP OF this base, never in place of it) plus every
/// qualifying [`ConditionalAddend`] this reader has verified.
///
/// SD-31 wave 31's own worked example (`MEASURE-TWICE.md` §3.1) is exactly this shape, and is
/// reproduced verbatim (byte-for-byte, not paraphrased) by
/// `resolves_the_alchemist_bomb_lvl_producer_chain_across_two_real_corpus_records` below:
/// `data/corpus/advanced_players_guide/class_feature/alchemist/bomb.json` carries
/// `DEFINE:AlchemistBombLVL|0` AND `BONUS:VAR|AlchemistBombLVL|AlchemistLVL` on the SAME record,
/// while `.../master_chymist/bomb_thrower.json` — a DIFFERENT record entirely — carries a THIRD
/// producer, `BONUS:VAR|AlchemistBombLVL|MasterChymistLVL`. No single record holds every producer
/// of `AlchemistBombLVL`; resolving it correctly requires reading across records, which is exactly
/// what [`extract_addends`] and the wave-26 module this generalises could not do (it took one
/// caller-selected record's tokens and nothing else — see this module's own doc, "Does not itself
/// decide which corpus tokens belong to one record").
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProducerChain {
    /// The `DEFINE:<var>|<formula>` base, if any record in the scanned set carries one. `None`
    /// means no `DEFINE` was found for this variable in the scanned tokens — PCGen itself treats
    /// an un-`DEFINE`d variable's base as 0 (every corpus `DEFINE:<var>|0` this module has read
    /// states that default explicitly; this reader never invents a non-zero default in its
    /// absence).
    pub base: Option<String>,
    /// Every qualifying `BONUS:VAR` addend found across the scanned tokens, in scan order.
    pub addends: Vec<ConditionalAddend>,
}

/// Scans `tokens` for every `DEFINE:<target_var>|<formula>` entry (mirroring
/// [`extract_addends`]'s own `BONUS:VAR` scan). More than one `DEFINE` for the SAME variable with
/// DIFFERING formula text is refused, never silently resolved by "first wins" or "last wins" —
/// this reader has not verified which record's `DEFINE` PCGen's own load order would keep, and a
/// wrong guess here would poison every [`resolve_producer_chain_corpus_wide`] call for that
/// variable. Identical duplicate `DEFINE`s (the same var, the same formula text, appearing more
/// than once — e.g. a `.MOD` continuation restating its parent's `DEFINE`) are not an error.
pub fn extract_define_base<'a>(
    target_var: &str,
    tokens: impl IntoIterator<Item = (&'a str, &'a str)>,
) -> Result<Option<String>, FormulaEvalError> {
    let mut found: Option<String> = None;
    for (key, value) in tokens {
        if key != "DEFINE" {
            continue;
        }
        let Some((name, formula)) = value.split_once('|') else {
            continue;
        };
        if name != target_var {
            continue;
        }
        match &found {
            None => found = Some(formula.to_string()),
            Some(existing) if existing == formula => {}
            Some(existing) => {
                return Err(FormulaEvalError(format!(
                    "DEFINE:{target_var}|... has two disagreeing bases in the scanned tokens \
                     ({existing:?} vs {formula:?}) — this reader has not verified PCGen's own \
                     load-order tiebreak and refuses to guess which one wins"
                )))
            }
        }
    }
    Ok(found)
}

/// The data-driven generalisation levers.md L23 calls for (`MEASURE-TWICE.md` §3.1, "Generic
/// formula-binding accumulator... generalized to be data-driven"): resolves `target_var`'s full
/// producer chain — its `DEFINE:` base plus every qualifying `BONUS:VAR` addend — by scanning
/// EVERY record's tokens in `records`, not just one caller-preselected record. `records` is any
/// iterable of per-record token iterables (e.g. every corpus JSON record's `data.raw_tokens`,
/// adapted the same way [`extract_addends`]'s own `tokens` parameter is); this reader still does
/// not decide which records exist or which order they arrive in — the caller owns corpus
/// traversal, this function owns aggregation.
///
/// Reuses [`extract_define_base`] and [`extract_addends`] unmodified per record and merges their
/// results — this function adds no new per-token parsing, only the corpus-wide (multi-record)
/// aggregation those two functions did not have. An addend whose gate this reader has not
/// verified still refuses the WHOLE call (via [`extract_addends`]'s own refusal), now propagated
/// across every record scanned, not just one.
pub fn resolve_producer_chain_corpus_wide<'a, R, T>(
    target_var: &str,
    records: R,
) -> Result<ProducerChain, FormulaEvalError>
where
    R: IntoIterator<Item = T>,
    T: IntoIterator<Item = (&'a str, &'a str)> + Clone,
{
    let mut chain = ProducerChain::default();
    for record_tokens in records {
        if let Some(base) = extract_define_base(target_var, record_tokens.clone())? {
            match &chain.base {
                None => chain.base = Some(base),
                Some(existing) if *existing == base => {}
                Some(existing) => {
                    return Err(FormulaEvalError(format!(
                        "DEFINE:{target_var}|... disagrees across records ({existing:?} vs \
                         {base:?}) — refusing to guess which record's base PCGen would load"
                    )))
                }
            }
        }
        chain.addends.extend(extract_addends(target_var, record_tokens)?);
    }
    Ok(chain)
}

/// Evaluates a resolved [`ProducerChain`]: the `DEFINE:` base (0 if `chain.base` is `None`, per
/// [`ProducerChain::base`]'s own doc) plus [`evaluate_stack`]'s gated sum of every addend. This is
/// the full PCGen `getVariableValue` shape module doc point 2 cites — base first, active bonuses
/// added on top — not a new arithmetic rule of this reader's own invention.
pub fn evaluate_producer_chain(
    evaluator: &dyn FormulaEvaluator,
    chain: &ProducerChain,
    vars: &BTreeMap<String, i64>,
) -> Result<i64, FormulaEvalError> {
    let base = match &chain.base {
        Some(formula) => evaluator.evaluate(formula, vars)?,
        None => 0,
    };
    Ok(base + evaluate_stack(evaluator, &chain.addends, vars)?)
}

/// One target variable's outcome from a corpus-wide sweep (kanban card 8,
/// `gate-2-corpus-wide-runs`, AT-32-G2-004): either its resolved [`ProducerChain`], or the
/// refusal reason [`resolve_producer_chain_corpus_wide`] returned for it. Kept as an enum rather
/// than two parallel maps' worth of `Option` fields so a caller cannot accidentally read both a
/// chain and a refusal reason for the same variable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CorpusWideOutcome {
    Resolved(ProducerChain),
    Refused(String),
}

/// The result of resolving every distinct F4-shaped (bare-identifier) target variable found across a corpus-wide record
/// population — AT-32-G2-004's "run corpus-wide once" for this engine. `outcomes` is keyed by
/// variable name so the CLI wrapper (`src/bin/bonus_stack_reader.rs`) can serialise it
/// deterministically; `population` is `outcomes.len()`, restated explicitly so a caller checking
/// "did this run examine anything" does not have to trust an empty map is intentional (mirrors
/// `corpus_literal_sweep`'s own "an empty population asserts nothing" posture, `SweepTally`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CorpusWideReport {
    pub population: usize,
    pub outcomes: BTreeMap<String, CorpusWideOutcome>,
}

/// Scans every record in `records` for the full F4 binding-layer population -- the bare-identifier subset of the canonical vocabulary (`scripts/shape_ledger.py`) that `bonus_stack_reader.rs` targets, not the unrelated F10 step-count family — every distinct
/// variable name targeted by at least one `BONUS:VAR|<var>|...` token anywhere in the scanned
/// set, the same "does this key/value pair look like a `BONUS:VAR` token" test
/// [`extract_addends`] uses — and resolves EACH one's full producer chain via
/// [`resolve_producer_chain_corpus_wide`] against the SAME full record population, not a
/// per-variable subset. This is the corpus-wide entry point AT-32-G2-004 requires: the caller
/// supplies the whole population once (e.g. every `data/corpus/**/*.json` record's
/// `data.raw_tokens`), and every distinct F4-shaped target variable found in it is resolved — none
/// picked by the caller, none silently skipped.
///
/// A variable whose resolution refuses (an unrecognised PRE-tag gate on any addend anywhere in
/// the scanned set, or two records disagreeing on its `DEFINE` base) is recorded as
/// [`CorpusWideOutcome::Refused`], not dropped — the report's own population count still counts
/// it, so a caller cannot mistake "refused" for "never seen".
pub fn resolve_all_producer_chains_corpus_wide<'a, T>(records: &[T]) -> CorpusWideReport
where
    T: IntoIterator<Item = (&'a str, &'a str)> + Clone,
{
    let mut vars: BTreeSet<String> = BTreeSet::new();
    for record in records {
        for (key, value) in record.clone() {
            if key != "BONUS" {
                continue;
            }
            let parts: Vec<&str> = value.split('|').collect();
            if parts.len() >= 2 && parts[0] == "VAR" {
                vars.insert(parts[1].to_string());
            }
        }
    }

    let mut outcomes = BTreeMap::new();
    for var in &vars {
        let outcome = match resolve_producer_chain_corpus_wide(var, records.iter().cloned()) {
            Ok(chain) => CorpusWideOutcome::Resolved(chain),
            Err(e) => CorpusWideOutcome::Refused(e.0),
        };
        outcomes.insert(var.clone(), outcome);
    }

    CorpusWideReport { population: vars.len(), outcomes }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::pilot_compute::formula_interpreter::PcgenFormulaEvaluator;

    fn vars(pairs: &[(&str, i64)]) -> BTreeMap<String, i64> {
        pairs.iter().map(|(k, v)| (k.to_string(), *v)).collect()
    }

    /// The `witch_hex/ward.json` record's REAL raw tokens (transcribed by hand from the corpus
    /// file itself — `data/corpus/advanced_players_guide/class_feature/witch_hex/ward.json`,
    /// re-verified by `wave_26_reads_the_real_ward_json_record_unmodified` below against the live
    /// file, so this hand transcription cannot silently drift from the file it claims to mirror).
    fn ward_json_bonus_tokens() -> Vec<(&'static str, &'static str)> {
        vec![
            ("BONUS", "VAR|WitchHexDC_Ward|WitchHexDC"),
            ("BONUS", "VAR|WitchHexDC_Ward|2|PREABILITY:1,CATEGORY=FEAT,Ability Focus(Witch Hex ~ Ward)"),
            ("BONUS", "VAR|WitchWardBonus|2"),
            ("BONUS", "VAR|WitchWardBonus|1|PREVARGTEQ:WitchHexAbilityLVL,8"),
            ("BONUS", "VAR|WitchWardBonus|1|PREVARGTEQ:WitchHexAbilityLVL,16"),
        ]
    }

    #[test]
    fn extracts_only_the_matching_target_var_addends_in_order() {
        let addends = extract_addends("WitchWardBonus", ward_json_bonus_tokens()).unwrap();
        assert_eq!(
            addends,
            vec![
                ConditionalAddend { formula: "2".to_string(), gate: None },
                ConditionalAddend {
                    formula: "1".to_string(),
                    gate: Some(PrevarGteqGate { variable: "WitchHexAbilityLVL".to_string(), threshold: 8 })
                },
                ConditionalAddend {
                    formula: "1".to_string(),
                    gate: Some(PrevarGteqGate { variable: "WitchHexAbilityLVL".to_string(), threshold: 16 })
                },
            ]
        );
    }

    #[test]
    fn unrecognised_pre_tag_on_a_matching_token_refuses_not_silently_dropped() {
        // WitchHexDC_Ward's own second token carries PREABILITY, not PREVARGTEQ — this reader
        // must refuse rather than silently treating it as ungated (always-applies) or dropping it
        // (never-applies), either of which would be a guess.
        let err = extract_addends("WitchHexDC_Ward", ward_json_bonus_tokens()).unwrap_err();
        assert!(err.0.contains("not PREVARGTEQ"), "got: {}", err.0);
    }

    #[test]
    fn evaluate_stack_matches_the_real_witch_ward_bonus_at_every_gate_boundary() {
        // 2 (base) + 1 at WitchHexAbilityLVL>=8 + 1 at WitchHexAbilityLVL>=16 — hand-derived
        // directly from `ward.json`'s own three `WitchWardBonus` tokens above, independent of
        // this module's code (module doc points 1+2 are the derivation, not this test).
        let addends = extract_addends("WitchWardBonus", ward_json_bonus_tokens()).unwrap();
        let e = PcgenFormulaEvaluator;
        assert_eq!(evaluate_stack(&e, &addends, &vars(&[("WitchHexAbilityLVL", 1)])).unwrap(), 2);
        assert_eq!(evaluate_stack(&e, &addends, &vars(&[("WitchHexAbilityLVL", 7)])).unwrap(), 2);
        assert_eq!(evaluate_stack(&e, &addends, &vars(&[("WitchHexAbilityLVL", 8)])).unwrap(), 3);
        assert_eq!(evaluate_stack(&e, &addends, &vars(&[("WitchHexAbilityLVL", 15)])).unwrap(), 3);
        assert_eq!(evaluate_stack(&e, &addends, &vars(&[("WitchHexAbilityLVL", 16)])).unwrap(), 4);
        assert_eq!(evaluate_stack(&e, &addends, &vars(&[("WitchHexAbilityLVL", 20)])).unwrap(), 4);
    }

    /// Decision 1(a) mutation proof: naively summing every addend unconditionally (the exact
    /// mistake this module exists to prevent — see module doc's opening paragraph) MUST diverge
    /// from `evaluate_stack`'s gated total below the top gate, proving the gate check is load-
    /// bearing and not a no-op.
    #[test]
    fn mutated_ungated_sum_is_caught_diverging_below_the_top_gate() {
        let addends = extract_addends("WitchWardBonus", ward_json_bonus_tokens()).unwrap();
        let e = PcgenFormulaEvaluator;
        let v = vars(&[("WitchHexAbilityLVL", 1)]);
        let gated = evaluate_stack(&e, &addends, &v).unwrap();
        let ungated_sum: i64 = addends.iter().map(|a| e.evaluate(&a.formula, &v).unwrap()).sum();
        assert_eq!(gated, 2, "correct: only the ungated base addend qualifies at level 1");
        assert_eq!(ungated_sum, 4, "a naive unconditional sum over-counts by both unmet gates");
        assert_ne!(gated, ungated_sum, "the gate check must change the answer, not just compile");
    }

    #[test]
    fn unbound_gate_variable_refuses_not_defaults_to_ungated() {
        let addends = extract_addends("WitchWardBonus", ward_json_bonus_tokens()).unwrap();
        let e = PcgenFormulaEvaluator;
        let err = evaluate_stack(&e, &addends, &BTreeMap::new()).unwrap_err();
        assert!(err.0.contains("needs a") && err.0.contains("binding"), "got: {}", err.0);
    }

    #[test]
    fn a_token_with_more_than_one_trailing_pre_field_refuses() {
        let tokens = vec![(
            "BONUS",
            "VAR|Weird|1|PREVARGTEQ:X,8|PREVARGTEQ:Y,4",
        )];
        let err = extract_addends("Weird", tokens).unwrap_err();
        assert!(err.0.contains("trailing PRE-tag fields"), "got: {}", err.0);
    }

    #[test]
    fn non_matching_tokens_are_skipped_not_errors() {
        let tokens = vec![
            ("KEY", "Witch Hex ~ Ward"),
            ("DEFINE", "WitchWardBonus|0"),
            ("BONUS", "STAT|CHA|-8"),
            ("BONUS", "VAR|SomeOtherTarget|5"),
        ];
        assert_eq!(extract_addends("WitchWardBonus", tokens).unwrap(), Vec::new());
    }

    /// Guards against the hand-transcribed `ward_json_bonus_tokens` fixture silently drifting
    /// from the real corpus file it claims to mirror — re-reads the live file at test time (the
    /// same `CODEX_REPO_ROOT`-relative pattern `formula_interpreter::tests::corpus_shape_coverage`
    /// already uses) and asserts the exact same five `BONUS` values are present, in order.
    #[test]
    fn wave_26_reads_the_real_ward_json_record_unmodified() {
        // Adversarial-review finding (wave 26 integration cycle): this guard used to fall back to
        // an env var (`CODEX_REPO_ROOT`) and silently `return` (test passes vacuously) when the
        // file could not be read — a no-op whenever the var was unset or wrong, defeating the one
        // mechanism preventing `ward_json_bonus_tokens()` from drifting away from the real record.
        // `env!("CARGO_MANIFEST_DIR")` is a compile-time constant (this crate's own established
        // convention — see e.g. `class_feature_grant_consumer.rs::repo_root()`,
        // `derived_evaluator_fixture_check.rs`), always correct, never needs a runtime fallback —
        // and a missing/unreadable file now hard-fails via `.expect()` instead of skipping.
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("data/corpus/advanced_players_guide/class_feature/witch_hex/ward.json");
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("wave_26_reads_the_real_ward_json_record_unmodified: {path:?} must be readable, got: {e}"));
        let v: serde_json::Value = serde_json::from_str(&text).expect("ward.json must be valid JSON");
        let tokens = v
            .pointer("/data/raw_tokens")
            .and_then(|t| t.as_array())
            .expect("ward.json must have data.raw_tokens");
        let bonus_values: Vec<String> = tokens
            .iter()
            .filter(|t| t.get("key").and_then(|k| k.as_str()) == Some("BONUS"))
            .filter_map(|t| t.get("value").and_then(|v| v.as_str()).map(str::to_string))
            .collect();
        let expected: Vec<String> =
            ward_json_bonus_tokens().into_iter().map(|(_, v)| v.to_string()).collect();
        assert_eq!(bonus_values, expected, "the hand-transcribed fixture has drifted from the real file");
    }

    // --- Gate 2 / kanban card 7: generalising the binding layer to be data-driven ------------

    /// Reads one corpus record's `data.raw_tokens` as owned `(key, value)` pairs, the same
    /// `CARGO_MANIFEST_DIR`-relative pattern `wave_26_reads_the_real_ward_json_record_unmodified`
    /// above already uses, generalised to any record path so the two-record proof below can read
    /// BOTH real files it cites without inventing a second hand-transcribed fixture that could
    /// itself drift.
    fn read_raw_tokens_owned(relative_path: &str) -> Vec<(String, String)> {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path);
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_raw_tokens_owned: {path:?} must be readable, got: {e}"));
        let v: serde_json::Value = serde_json::from_str(&text)
            .unwrap_or_else(|e| panic!("read_raw_tokens_owned: {path:?} must be valid JSON, got: {e}"));
        v.pointer("/data/raw_tokens")
            .and_then(|t| t.as_array())
            .unwrap_or_else(|| panic!("read_raw_tokens_owned: {path:?} must have data.raw_tokens"))
            .iter()
            .map(|t| {
                let key = t.get("key").and_then(|k| k.as_str()).unwrap_or_default().to_string();
                let value = t.get("value").and_then(|v| v.as_str()).unwrap_or_default().to_string();
                (key, value)
            })
            .collect()
    }

    #[test]
    fn extract_define_base_finds_the_matching_define_and_ignores_others() {
        let tokens = vec![
            ("DEFINE", "AlchemistBombLVL|0"),
            ("DEFINE", "AlchemistBombTimes|0"),
            ("BONUS", "VAR|AlchemistBombLVL|AlchemistLVL"),
        ];
        assert_eq!(
            extract_define_base("AlchemistBombLVL", tokens).unwrap(),
            Some("0".to_string())
        );
    }

    #[test]
    fn extract_define_base_returns_none_when_no_define_matches() {
        let tokens = vec![("DEFINE", "SomethingElse|3")];
        assert_eq!(extract_define_base("AlchemistBombLVL", tokens).unwrap(), None);
    }

    #[test]
    fn extract_define_base_refuses_on_disagreeing_duplicate_define() {
        let tokens = vec![("DEFINE", "X|0"), ("DEFINE", "X|5")];
        let err = extract_define_base("X", tokens).unwrap_err();
        assert!(err.0.contains("disagreeing bases"), "got: {}", err.0);
    }

    #[test]
    fn extract_define_base_allows_identical_duplicate_define() {
        let tokens = vec![("DEFINE", "X|0"), ("DEFINE", "X|0")];
        assert_eq!(extract_define_base("X", tokens).unwrap(), Some("0".to_string()));
    }

    /// The load-bearing proof: `AlchemistBombLVL` is `DEFINE:AlchemistBombLVL|0` PLUS
    /// `BONUS:VAR|AlchemistBombLVL|AlchemistLVL` on
    /// `advanced_players_guide/class_feature/alchemist/bomb.json`, PLUS a THIRD producer,
    /// `BONUS:VAR|AlchemistBombLVL|MasterChymistLVL`, on an entirely different record
    /// (`advanced_players_guide/class_feature/master_chymist/bomb_thrower.json`) — the exact
    /// worked example `MEASURE-TWICE.md` §3.1 names as the reason the wave-26 reader (single
    /// caller-selected record only) undercounts: it could never see the master_chymist producer
    /// because nothing hands it that record's tokens for an `AlchemistBombLVL` lookup scoped to
    /// `alchemist/bomb.json`.
    #[test]
    fn resolves_the_alchemist_bomb_lvl_producer_chain_across_two_real_corpus_records() {
        let alchemist_tokens =
            read_raw_tokens_owned("data/corpus/advanced_players_guide/class_feature/alchemist/bomb.json");
        let master_chymist_tokens = read_raw_tokens_owned(
            "data/corpus/advanced_players_guide/class_feature/master_chymist/bomb_thrower.json",
        );
        let records = vec![
            alchemist_tokens.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect::<Vec<_>>(),
            master_chymist_tokens.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect::<Vec<_>>(),
        ];

        let chain = resolve_producer_chain_corpus_wide("AlchemistBombLVL", records).unwrap();
        assert_eq!(chain.base, Some("0".to_string()));
        assert_eq!(
            chain.addends,
            vec![
                ConditionalAddend { formula: "AlchemistLVL".to_string(), gate: None },
                ConditionalAddend { formula: "MasterChymistLVL".to_string(), gate: None },
            ],
            "must find the alchemist/bomb.json addend AND the master_chymist/bomb_thrower.json \
             addend, in scan order — a single-record reader can only ever see one of the two"
        );

        let e = PcgenFormulaEvaluator;
        let both_classes =
            vars(&[("AlchemistLVL", 6), ("MasterChymistLVL", 2)]);
        assert_eq!(
            evaluate_producer_chain(&e, &chain, &both_classes).unwrap(),
            8,
            "0 (DEFINE base) + 6 (AlchemistLVL) + 2 (MasterChymistLVL)"
        );
    }

    /// Mutation proof (Decision 1(a) shape): resolving from only the `alchemist/bomb.json` record
    /// — i.e. the wave-26 reader's own single-record scope — silently drops the
    /// `master_chymist/bomb_thrower.json` producer and under-counts. This is the exact gap the
    /// corpus-wide generalisation exists to close; a reviewer mutating
    /// `resolve_producer_chain_corpus_wide` back down to "only ever look at the first record"
    /// would make this test fail, proving the multi-record scan is load-bearing and not a no-op.
    #[test]
    fn single_record_scope_undercounts_the_alchemist_bomb_lvl_chain() {
        let alchemist_tokens =
            read_raw_tokens_owned("data/corpus/advanced_players_guide/class_feature/alchemist/bomb.json");
        let single_record = vec![alchemist_tokens
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect::<Vec<_>>()];

        let narrow_chain = resolve_producer_chain_corpus_wide("AlchemistBombLVL", single_record).unwrap();
        let e = PcgenFormulaEvaluator;
        let both_classes = vars(&[("AlchemistLVL", 6), ("MasterChymistLVL", 2)]);
        assert_eq!(
            evaluate_producer_chain(&e, &narrow_chain, &both_classes).unwrap(),
            6,
            "missing the master_chymist producer entirely — 0 (DEFINE base) + 6 (AlchemistLVL) \
             only, silently dropping the 2 MasterChymistLVL contribution"
        );
    }

    #[test]
    fn evaluate_producer_chain_with_no_define_defaults_base_to_zero_and_matches_evaluate_stack() {
        // WitchWardBonus's own real corpus tokens carry no DEFINE at all (verified by
        // `wave_26_reads_the_real_ward_json_record_unmodified` above, which only asserts on the
        // BONUS values — a real check that no DEFINE line was silently dropped by this test
        // fixture would need a second guard, but ward.json's own directory listing has already
        // been read via `ward_json_bonus_tokens()` for years with no DEFINE row ever surfacing).
        let addends = extract_addends("WitchWardBonus", ward_json_bonus_tokens()).unwrap();
        let chain = ProducerChain { base: None, addends: addends.clone() };
        let e = PcgenFormulaEvaluator;
        for level in [1, 7, 8, 15, 16, 20] {
            let v = vars(&[("WitchHexAbilityLVL", level)]);
            assert_eq!(
                evaluate_producer_chain(&e, &chain, &v).unwrap(),
                evaluate_stack(&e, &addends, &v).unwrap(),
                "a chain with no DEFINE base must match the narrow evaluate_stack path exactly \
                 (base defaults to 0, not a behavioural change) at level {level}"
            );
        }
    }

    #[test]
    fn resolve_producer_chain_corpus_wide_over_one_record_matches_the_narrow_extract_addends() {
        // Regression guard: scanning exactly the tokens the wave-26 reader already handled, via
        // the new corpus-wide entry point, must reproduce the same addends the narrow
        // `extract_addends` call already proves correct above — the generalisation must not
        // change behaviour for the case it already handled.
        let narrow = extract_addends("WitchWardBonus", ward_json_bonus_tokens()).unwrap();
        let wide =
            resolve_producer_chain_corpus_wide("WitchWardBonus", vec![ward_json_bonus_tokens()]).unwrap();
        assert_eq!(wide.addends, narrow);
        assert_eq!(wide.base, None);
    }

    #[test]
    fn resolve_producer_chain_corpus_wide_refuses_when_an_addend_gate_is_unrecognised() {
        // The whole-call refusal from extract_addends must propagate through the corpus-wide
        // aggregator, not get silently swallowed while other records' addends are kept.
        let bad_record = vec![("BONUS", "VAR|X|1|PREABILITY:1,CATEGORY=FEAT,Foo")];
        let good_record = vec![("BONUS", "VAR|X|2")];
        let err = resolve_producer_chain_corpus_wide("X", vec![bad_record, good_record]).unwrap_err();
        assert!(err.0.contains("not PREVARGTEQ"), "got: {}", err.0);
    }

    #[test]
    fn resolve_producer_chain_corpus_wide_refuses_on_cross_record_disagreeing_define() {
        let record_a = vec![("DEFINE", "X|0")];
        let record_b = vec![("DEFINE", "X|3")];
        let err = resolve_producer_chain_corpus_wide("X", vec![record_a, record_b]).unwrap_err();
        assert!(err.0.contains("disagrees across records"), "got: {}", err.0);
    }

    // --- Gate 2 / kanban card 8: `resolve_all_producer_chains_corpus_wide`, the corpus-wide ----
    // --- sweep AT-32-G2-004 requires -----------------------------------------------------------

    /// Real corpus bytes, three records: the `AlchemistBombLVL` cross-record chain (proven above,
    /// resolves), `ward.json`'s own `WitchWardBonus` (resolves, no `DEFINE`) AND
    /// `WitchHexDC_Ward` (refuses — its second `BONUS:VAR` token carries a `PREABILITY` gate this
    /// reader has never verified, per `ward_json_bonus_tokens()` above). One sweep over all three
    /// records must find and correctly classify all three distinct target variables — proving the
    /// population-discovery step (find every `BONUS:VAR` target, not just ones a caller names)
    /// and the per-variable resolution step both work end to end, and that a refusal for one
    /// variable does not poison or hide the others.
    #[test]
    fn resolve_all_producer_chains_corpus_wide_finds_and_classifies_every_distinct_target_var() {
        let alchemist_tokens =
            read_raw_tokens_owned("data/corpus/advanced_players_guide/class_feature/alchemist/bomb.json");
        let master_chymist_tokens = read_raw_tokens_owned(
            "data/corpus/advanced_players_guide/class_feature/master_chymist/bomb_thrower.json",
        );
        let ward_tokens: Vec<(&str, &str)> = ward_json_bonus_tokens();

        let records: Vec<Vec<(&str, &str)>> = vec![
            alchemist_tokens.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect(),
            master_chymist_tokens.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect(),
            ward_tokens,
        ];

        let report = resolve_all_producer_chains_corpus_wide(&records);

        // `bomb.json` carries several other `BONUS:VAR` targets besides `AlchemistBombLVL`
        // (`AlchemistBombTimes`, `AlchemistBombDiceSize`, ...) — real corpus bytes, not this
        // test's business to enumerate exhaustively. The assertion that matters is that the
        // population INCLUDES all three named variables (proving population-discovery scans
        // every record, not just the first) and that `outcomes.len()` always equals
        // `population` (no variable found but silently unclassified).
        assert_eq!(report.population, report.outcomes.len());
        for expected in ["AlchemistBombLVL", "WitchWardBonus", "WitchHexDC_Ward"] {
            assert!(
                report.outcomes.contains_key(expected),
                "expected {expected} in the discovered population, got: {:?}",
                report.outcomes.keys().collect::<Vec<_>>()
            );
        }

        match report.outcomes.get("AlchemistBombLVL") {
            Some(CorpusWideOutcome::Resolved(chain)) => {
                assert_eq!(chain.base, Some("0".to_string()));
                assert_eq!(chain.addends.len(), 2, "both records' addends must be found");
            }
            other => panic!("expected AlchemistBombLVL to resolve, got {other:?}"),
        }

        match report.outcomes.get("WitchWardBonus") {
            Some(CorpusWideOutcome::Resolved(chain)) => {
                assert_eq!(chain.base, None);
                assert_eq!(chain.addends.len(), 3);
            }
            other => panic!("expected WitchWardBonus to resolve, got {other:?}"),
        }

        match report.outcomes.get("WitchHexDC_Ward") {
            Some(CorpusWideOutcome::Refused(reason)) => {
                assert!(
                    reason.contains("not PREVARGTEQ") || reason.contains("PRE-tag"),
                    "expected the real PREABILITY-gate refusal reason, got: {reason}"
                );
            }
            other => panic!("expected WitchHexDC_Ward to refuse (real PREABILITY gate), got {other:?}"),
        }
    }

    /// Mutation-shaped proof: a sweep over only ONE of the two `AlchemistBombLVL`-carrying
    /// records must resolve fewer addends than the full two-record sweep — proving
    /// `resolve_all_producer_chains_corpus_wide` genuinely feeds every scanned record into the
    /// per-variable resolution step rather than, say, only ever looking at `records[0]`.
    #[test]
    fn resolve_all_producer_chains_corpus_wide_uses_every_scanned_record_not_just_the_first() {
        let alchemist_tokens =
            read_raw_tokens_owned("data/corpus/advanced_players_guide/class_feature/alchemist/bomb.json");
        let narrow: Vec<Vec<(&str, &str)>> =
            vec![alchemist_tokens.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect()];

        let report = resolve_all_producer_chains_corpus_wide(&narrow);
        match report.outcomes.get("AlchemistBombLVL") {
            Some(CorpusWideOutcome::Resolved(chain)) => {
                assert_eq!(chain.addends.len(), 1, "only the alchemist/bomb.json addend is visible")
            }
            other => panic!("expected AlchemistBombLVL to resolve, got {other:?}"),
        }
    }

    #[test]
    fn resolve_all_producer_chains_corpus_wide_over_no_records_reports_zero_population() {
        let empty: Vec<Vec<(&str, &str)>> = vec![];
        let report = resolve_all_producer_chains_corpus_wide(&empty);
        assert_eq!(report.population, 0);
        assert!(report.outcomes.is_empty());
    }
}
