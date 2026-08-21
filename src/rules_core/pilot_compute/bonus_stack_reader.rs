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

use std::collections::BTreeMap;

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
        let repo_root = std::env::var("CODEX_REPO_ROOT").unwrap_or_else(|_| ".".to_string());
        let path = std::path::Path::new(&repo_root)
            .join("data/corpus/advanced_players_guide/class_feature/witch_hex/ward.json");
        let Ok(text) = std::fs::read_to_string(&path) else {
            eprintln!("wave_26_reads_the_real_ward_json_record_unmodified: {path:?} not found — skipping");
            return;
        };
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
}
