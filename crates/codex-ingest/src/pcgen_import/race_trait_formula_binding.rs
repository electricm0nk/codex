//! SD-32 — wires `formula_interpreter::PcgenFormulaEvaluator` into the
//! `DESC:`/`BONUS:VAR` same-row resolution both `src/bin/ingest_race_traits.rs`
//! and `src/bin/ingest_races.rs` implement, per `decisions.md §17`: one
//! generic binding, reused by both binaries, not a per-race patch.
//!
//! **Why this exists.** `SD-31 decisions.md` Decision 20 (2026-08-21) overturned
//! `SD-27 decisions.md §24.1`'s formula-interpreter ban: *"§24.1 IS OVERTURNED.
//! Build the interpreter."* `formula_interpreter.rs` is that interpreter,
//! fixture-proved corpus-wide (this bundle's Gate 2). It was not, until this
//! module, reachable from either race-trait ingest binary: both still refused
//! any `DESC:` `%N` argument or `BONUS:VAR` amount that was not a bare integer
//! literal or a bare same-row variable name — e.g. `arg_abilities_race.lst`'s
//! `Halfling ~ Adaptable Luck` row, whose real `%2` argument is the expression
//! `Halfling_AdaptableLuck_Bonus-1`, not a name or a literal. That refusal
//! shipped a truncated sentence to the Race Traits panel: *"...they only gain a
//! bonus"* instead of *"...they only gain a +1 bonus"* — see
//! [`resolves_the_halfling_adaptable_luck_desc_arg_against_the_real_corpus_row`]
//! below, whose expected `1` is transcribed by hand from the raw `.lst` bytes
//! (`DEFINE:Halfling_AdaptableLuck_Bonus|0` + `BONUS:VAR|Halfling_AdaptableLuck_Bonus|2`,
//! so the base is 2, and the expression subtracts 1), never read by the code
//! under test — the fixture-discipline bar `decisions.md §3` restates.
//!
//! **What this module does not change.** Both binaries' `same_row_vars`
//! already refuse (return `None`, not a value) for any variable whose base or
//! addend is not resolvable purely from the SAME row's own tokens — a
//! cross-record base, a conditional `BONUS:VAR` addend, or a formula that
//! itself references a variable this row never defines (`CHA`, `TL`,
//! `CHASCORE`, a class-feature-owned pool variable, …). This module's
//! [`resolve_same_row_formula`] preserves that refusal exactly: it only binds
//! variables the caller's own `vars` map already resolved to `Some`, so a
//! formula naming an unbound identifier still refuses via
//! `PcgenFormulaEvaluator`'s own "never silently treat an unbound variable as
//! 0" rule (`formula_interpreter.rs`'s module doc, "What this module always
//! refuses"). Wiring the interpreter in does not relax that boundary; it only
//! adds arithmetic capability *inside* it, for the case both binaries already
//! prove is real: an expression over variables the SAME row already resolved.

use std::collections::BTreeMap;

use super::formula_interpreter::{recognises_shape, PcgenFormulaEvaluator};
use super::formula_reproduction_harness::FormulaEvaluator;

/// Resolves `name` against a `same_row_vars`-shaped table (`BTreeMap<String,
/// Option<i64>>`, `None` meaning "this row names the variable but cannot
/// finish resolving it"), trying three things in order, the first that
/// applies wins:
///
/// 1. **A direct key.** `name` is exactly a variable this row resolved —
///    covers every case both binaries already handled before this module
///    existed (`%1|Halfling_AdaptableLuck_Times`, a bare `BONUS:VAR|X|Y`
///    amount that is itself just another same-row variable's name).
/// 2. **An integer literal.** `name` parses as `i64` directly — the ordinary
///    `BONUS:VAR|X|4` / `DESC:...|4` shape.
/// 3. **A formula this module's evaluator recognises.** `name` is handed to
///    [`recognises_shape`] first (so an unrecognised token shape is a clean
///    `None`, not an evaluator panic or a misleading error swallowed the same
///    way), then evaluated by [`PcgenFormulaEvaluator`] against every
///    currently-`Some`-resolved variable in `vars` — never anything with
///    value `None`, which would silently manufacture a value for a variable
///    this row itself could not finish resolving. An unbound reference inside
///    the formula (any identifier not present as a bound `i64` — including
///    every `None` entry and every name `vars` never mentions at all) makes
///    `PcgenFormulaEvaluator::evaluate` return `Err`, which this function
///    turns into `None`, matching the "never guess" contract both callers'
///    existing (pre-interpreter) code already followed for every other
///    refusal shape.
///
/// Order 1-then-2-then-3 matters: trying the literal-key lookup before
/// invoking the parser keeps every existing resolved case byte-identical
/// (this function is a superset of the old direct-lookup-or-literal logic,
/// not a replacement that could reorder existing outcomes), and only reaches
/// the interpreter for a `name` that is genuinely a formula.
pub fn resolve_same_row_formula(name: &str, vars: &BTreeMap<String, Option<i64>>) -> Option<i64> {
    if let Some(direct) = vars.get(name).copied().flatten() {
        return Some(direct);
    }
    if let Ok(literal) = name.trim().parse::<i64>() {
        return Some(literal);
    }
    if recognises_shape(name).is_err() {
        return None;
    }
    let bound: BTreeMap<String, i64> =
        vars.iter().filter_map(|(k, v)| v.map(|resolved| (k.clone(), resolved))).collect();
    PcgenFormulaEvaluator.evaluate(name, &bound).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vars(pairs: &[(&str, Option<i64>)]) -> BTreeMap<String, Option<i64>> {
        pairs.iter().map(|(k, v)| (k.to_string(), *v)).collect()
    }

    #[test]
    fn a_direct_variable_key_resolves_without_touching_the_interpreter() {
        let v = vars(&[("Halfling_AdaptableLuck_Times", Some(3))]);
        assert_eq!(resolve_same_row_formula("Halfling_AdaptableLuck_Times", &v), Some(3));
    }

    #[test]
    fn a_bare_integer_literal_resolves() {
        let v = vars(&[]);
        assert_eq!(resolve_same_row_formula("4", &v), Some(4));
    }

    /// The real defect this module fixes, reproduced with the row's real
    /// hand-transcribed values: `DEFINE:Halfling_AdaptableLuck_Bonus|0` +
    /// `BONUS:VAR|Halfling_AdaptableLuck_Bonus|2` (both same-row literals,
    /// already resolved by `same_row_vars` before this function is ever
    /// called) makes the base variable `2`; the row's real `%2` argument is
    /// the formula `Halfling_AdaptableLuck_Bonus-1`, so the correct rendered
    /// value is `1` -- transcribed by hand from
    /// `arg_abilities_race.lst`'s raw bytes, never read by
    /// `resolve_same_row_formula` or `PcgenFormulaEvaluator`.
    #[test]
    fn resolves_the_halfling_adaptable_luck_desc_arg_against_the_real_corpus_row() {
        let v = vars(&[
            ("Halfling_AdaptableLuck_Times", Some(3)),
            ("Halfling_AdaptableLuck_Bonus", Some(2)),
        ]);
        assert_eq!(resolve_same_row_formula("Halfling_AdaptableLuck_Bonus-1", &v), Some(1));
    }

    /// A formula whose own dependency this row never resolved (`None`, not
    /// merely absent) must still refuse -- the interpreter is never handed a
    /// `None` masquerading as "not present", which would let a stale binding
    /// silently leak a wrong value.
    #[test]
    fn refuses_when_the_formula_references_an_unresolved_same_row_variable() {
        let v = vars(&[("Foo", None)]);
        assert_eq!(resolve_same_row_formula("Foo-1", &v), None);
    }

    /// A formula referencing a variable the row never mentions at all (a
    /// cross-record / character-state variable such as `CHA` or `TL`) still
    /// refuses -- this is the real shape every `..._RacialCastingMod|CHA`-
    /// style `BONUS:VAR` in the in-scope corpus carries, and wiring the
    /// interpreter in must not manufacture a value for it.
    #[test]
    fn refuses_when_the_formula_references_a_variable_this_row_never_defines() {
        let v = vars(&[("Dwarf_RacialCastingMod", None)]);
        assert_eq!(resolve_same_row_formula("CHA", &v), None);
    }

    #[test]
    fn refuses_an_unrecognised_token_shape_cleanly_rather_than_guessing() {
        let v = vars(&[]);
        assert_eq!(resolve_same_row_formula("count(\"whatever\")", &v), None);
    }

    /// Multi-operator formulas (matching the real corpus shapes found across
    /// the in-scope books, `min(...)`/`floor(...)`/division) resolve too,
    /// not only bare subtraction -- proving this is genuinely the shared
    /// interpreter and not a hand-rolled subset reimplementing it.
    #[test]
    fn resolves_a_multi_operator_formula_over_bound_same_row_variables() {
        let v = vars(&[("TL", Some(7)), ("CON", Some(3))]);
        assert_eq!(resolve_same_row_formula("min(floor((TL+1)/2),5)", &v), Some(4));
        assert_eq!(resolve_same_row_formula("10+(TL/2)+CON", &v), Some(16));
    }
}
