//! The `class_feature_description_entries` half of the `derived` wiring class's
//! evaluator-vs-fixture check (SD-31 wave 26, `OPERATOR-RULINGS-2026-08-21.md` §20) --
//! the sibling of `tests/derived_evaluator_fixture_check_class_feature.rs` (structural
//! level-scaling shape) and `tests/derived_evaluator_fixture_check_companion_save_dc.rs`
//! (a DESC-argument formula resolved to concrete numbers), over `derived-evaluator-
//! fixtures.json`'s `class_feature_description_entries` array.
//!
//! # What this seam verifies
//!
//! `class_feature_grant_consumer.rs`'s own widened emission (wave 26) resolves a class
//! feature's corpus `DESC:` `%N` placeholder through the real formula interpreter
//! (`formula_interpreter::PcgenFormulaEvaluator`, proven to reproduce 22 of 22
//! hand-modelled functions) instead of leaving the record unserved. This file proves the
//! RESULTING NUMBER is right, for the live population that resolver actually reaches.
//!
//! # The guarantees (the same four independent guarantees every sibling `derived`
//! fixture-check file in this package states for its own family)
//!
//! 1. **Different source artifact.** `class_feature_description_entries`' expected values
//!    are read by `scripts/derive_class_feature_description_fixtures.py` straight from the
//!    upstream PCGen `.lst` bytes, evaluated by that script's OWN from-scratch (Python,
//!    cross-language) recursive-descent evaluator. The engine evaluates this repo's own
//!    `data/corpus/**/*.json` ingest through the real Rust interpreter.
//! 2. **Committed first** — the fixture rows and this check land in the same commit,
//!    evidenced by `git log`, not asserted here.
//! 3. **Re-derivable from the pinned corpus field** —
//!    [`class_feature_description_expected_values_are_re_derivable_from_the_pinned_corpus_field`],
//!    a THIRD, independent reference evaluator written HERE (in Rust, but structurally
//!    different from both `formula_interpreter.rs`'s parser and the Python generator's),
//!    re-parses each pinned `corpus_field`/`bonus_var_chain` and must reproduce the
//!    fixture's own expected values.
//! 4. **Anchored to the same upstream bytes the engine ingested** —
//!    [`class_feature_description_pinned_upstream_lst_still_hashes_to_the_pinned_sha256`]
//!    (re-hashes `upstream_lst` fresh) and
//!    [`class_feature_description_pinned_corpus_field_is_byte_identical_to_the_upstream_lst`]
//!    (the pinned `corpus_field`'s prose+argument-tail text appears verbatim on the pinned
//!    line).
//!
//! # The end-to-end proof: the REAL production resolver, not a parallel check
//!
//! [`every_fixture_clears_the_real_production_bar_check`] drives
//! `codex::rules_core::derived_evaluator_fixture_check::run_bar_check` -- the exact
//! function `src/bin/derived_evaluator_fixture_check.rs` (the gate CLI) and
//! `v06_work_inventory` both call -- and asserts every fixtured `unit_id` is in `cleared`,
//! not merely that some hand-rolled comparison inside this file agrees with itself.
//!
//! # SD-31 wave 27: ability-modifier-dependent targets
//!
//! `reference_resolve_chain` (guarantee 3's independent re-derivation) now also seeds the six
//! bare ability abbreviations from each fixture row's own `ability_modifier_inputs` -- empty
//! for every wave-26 level-only target, one bare abbreviation for wave 27's three new targets
//! (`Arcane Archer ~ Arrow of Death`/CHA, `Ranger ~ Master Hunter`/WIS, `Rogue ~ Master
//! Strike`/INT), mirroring production's `resolve_pcgen_var_chain`/`ability_modifier_seed_vars`
//! widening in `class_feature_grant_consumer.rs`. Confirmed RED before the widening (this file's
//! guarantee-3 test failed with `ArrowOfDeathDC=None` against the fixture's `Some(25)` the
//! instant the three new TARGETS were derived, before this file's own fix landed), GREEN after.
//!
//! # Mutation-proof
//!
//! Each guarantee is mutation-provable on the committed fixture in isolation (zero one
//! row's `upstream_lst_sha256` -> guarantee 4a red; change one `corpus_field`'s formula ->
//! guarantee 3 red; delete a row -> the one-per-unit count in guarantee-1's test goes red).
//! The end-to-end proof was additionally verified LIVE during this wave by temporarily
//! mutating `resolve_pcgen_var_chain`'s seeded level (`level + 1`) and re-running the full
//! suite: every fixture-backed test here, and the direct resolver-level unit tests in
//! `class_feature_grant_consumer.rs`, went RED (see the wave's own progress receipt for the
//! exact before/after `cargo test` output).

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use sha2::{Digest, Sha256};

use codex::rules_core::derived_evaluator_fixture_check::{
    load_class_feature_description_fixtures, run_bar_check,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

/// Same resolution rule every sibling guarantee file uses.
fn pcgen_data_root() -> Option<PathBuf> {
    if let Ok(root) = std::env::var("PCGEN_CORPUS_ROOT") {
        return Some(PathBuf::from(root));
    }
    let home = std::env::var("HOME").ok()?;
    Some(PathBuf::from(home).join("workspace").join("repos").join("pcgen").join("data"))
}

// -------------------------------------------------------------------------------------------
// A THIRD, independent reference evaluator: not `formula_interpreter.rs` (the engine, Rust),
// not `scripts/derive_class_feature_description_fixtures.py`'s evaluator (the generator,
// Python) -- a small recursive-descent parser/evaluator written HERE, in Rust, from the same
// grammar description (identifiers, integer literals, `+ - * /`, parens, commas, N-ary
// `min`/`max`/`floor`/`ceil`), never by reading either sibling's implementation. Agreement of
// all three is what makes this seam meaningful.
// -------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum RefTok {
    Num(f64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Comma,
}

fn ref_tokenize(s: &str) -> Option<Vec<RefTok>> {
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut out = Vec::new();
    while i < chars.len() {
        let c = chars[i];
        match c {
            ' ' | '\t' => i += 1,
            '+' => {
                out.push(RefTok::Plus);
                i += 1;
            }
            '-' => {
                out.push(RefTok::Minus);
                i += 1;
            }
            '*' => {
                out.push(RefTok::Star);
                i += 1;
            }
            '/' => {
                out.push(RefTok::Slash);
                i += 1;
            }
            '(' => {
                out.push(RefTok::LParen);
                i += 1;
            }
            ')' => {
                out.push(RefTok::RParen);
                i += 1;
            }
            ',' => {
                out.push(RefTok::Comma);
                i += 1;
            }
            c if c.is_ascii_digit() => {
                let start = i;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
                let text: String = chars[start..i].iter().collect();
                out.push(RefTok::Num(text.parse().ok()?));
            }
            c if c.is_alphabetic() || c == '_' => {
                let start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                out.push(RefTok::Ident(chars[start..i].iter().collect()));
            }
            _ => return None,
        }
    }
    Some(out)
}

struct RefParser<'a> {
    toks: &'a [RefTok],
    pos: usize,
    vars: &'a BTreeMap<String, i64>,
}

impl<'a> RefParser<'a> {
    fn peek(&self) -> Option<&RefTok> {
        self.toks.get(self.pos)
    }
    fn next(&mut self) -> Option<&RefTok> {
        let t = self.toks.get(self.pos);
        self.pos += 1;
        t
    }
    fn expr(&mut self) -> Option<f64> {
        let mut v = self.term()?;
        loop {
            match self.peek() {
                Some(RefTok::Plus) => {
                    self.next();
                    v += self.term()?;
                }
                Some(RefTok::Minus) => {
                    self.next();
                    v -= self.term()?;
                }
                _ => break,
            }
        }
        Some(v)
    }
    fn term(&mut self) -> Option<f64> {
        let mut v = self.unary()?;
        loop {
            match self.peek() {
                Some(RefTok::Star) => {
                    self.next();
                    v *= self.unary()?;
                }
                Some(RefTok::Slash) => {
                    self.next();
                    let rhs = self.unary()?;
                    if rhs == 0.0 {
                        return None;
                    }
                    v /= rhs;
                }
                _ => break,
            }
        }
        Some(v)
    }
    fn unary(&mut self) -> Option<f64> {
        match self.peek() {
            Some(RefTok::Minus) => {
                self.next();
                Some(-self.unary()?)
            }
            Some(RefTok::Plus) => {
                self.next();
                self.unary()
            }
            _ => self.atom(),
        }
    }
    fn atom(&mut self) -> Option<f64> {
        match self.next()?.clone() {
            RefTok::Num(n) => Some(n),
            RefTok::LParen => {
                let v = self.expr()?;
                match self.next()? {
                    RefTok::RParen => Some(v),
                    _ => None,
                }
            }
            RefTok::Ident(name) => {
                if self.peek() == Some(&RefTok::LParen) {
                    return self.call(&name);
                }
                self.vars.get(&name).map(|v| *v as f64)
            }
            _ => None,
        }
    }
    fn call(&mut self, name: &str) -> Option<f64> {
        let is_min = matches!(name, "min" | "MIN");
        let is_max = matches!(name, "max" | "MAX");
        let is_floor = matches!(name, "floor" | "FLOOR");
        let is_ceil = matches!(name, "ceil" | "CEIL");
        if !is_min && !is_max && !is_floor && !is_ceil {
            return None;
        }
        self.next(); // '('
        let mut args = vec![self.expr()?];
        while self.peek() == Some(&RefTok::Comma) {
            self.next();
            args.push(self.expr()?);
        }
        match self.next()? {
            RefTok::RParen => {}
            _ => return None,
        }
        if is_min {
            return args.into_iter().fold(None, |acc: Option<f64>, x| {
                Some(acc.map_or(x, |a| a.min(x)))
            });
        }
        if is_max {
            return args.into_iter().fold(None, |acc: Option<f64>, x| {
                Some(acc.map_or(x, |a| a.max(x)))
            });
        }
        if args.len() != 1 {
            return None;
        }
        Some(if is_floor { args[0].floor() } else { args[0].ceil() })
    }
}

/// Evaluates `formula` against `vars`, truncating the final value toward zero. `None` for any
/// unsupported shape or unbound identifier -- refuse, never guess.
fn reference_evaluate(formula: &str, vars: &BTreeMap<String, i64>) -> Option<i64> {
    let toks = ref_tokenize(formula)?;
    let mut parser = RefParser { toks: &toks, pos: 0, vars };
    let v = parser.expr()?;
    if parser.pos != parser.toks.len() {
        return None;
    }
    Some(v.trunc() as i64)
}

/// The same fixed-point chain resolution `resolve_pcgen_var_chain` performs, written
/// independently here against `bonus_var_chain` as pinned in the fixture (never re-read from
/// `data/corpus/`). SD-31 wave 27: also seeds `ability_modifier_inputs` (the fixture's own
/// fixed, assumed ability-modifier test input -- empty for every wave-26 level-only target),
/// mirroring production's `ability_modifier_seed_vars`.
fn reference_resolve_chain(
    bonus_vars: &BTreeMap<String, String>,
    class_level_var: &str,
    level: u8,
    ability_modifier_inputs: &BTreeMap<String, i16>,
) -> BTreeMap<String, i64> {
    let mut vars: BTreeMap<String, i64> = BTreeMap::new();
    for abbrev in ["STR", "DEX", "CON", "INT", "WIS", "CHA"] {
        vars.insert(
            abbrev.to_string(),
            i64::from(ability_modifier_inputs.get(abbrev).copied().unwrap_or(0)),
        );
    }
    vars.insert(class_level_var.to_string(), i64::from(level));
    let mut progressed = true;
    let mut guard = 0;
    while progressed && guard < 16 {
        progressed = false;
        guard += 1;
        for (name, formula) in bonus_vars {
            if vars.contains_key(name) {
                continue;
            }
            if let Some(value) = reference_evaluate(formula, &vars) {
                vars.insert(name.clone(), value);
                progressed = true;
            }
        }
    }
    vars
}

#[test]
fn class_feature_description_entries_are_present_and_one_per_unit() {
    let fixtures = load_class_feature_description_fixtures(&repo_root());
    assert!(
        !fixtures.is_empty(),
        "class_feature_description_entries is EMPTY -- every other guarantee in this file \
         would then pass vacuously, which is the Decision 1(a) shape this seam exists to \
         refuse"
    );
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for f in &fixtures {
        assert!(
            seen.insert(f.unit_id.as_str()),
            "class_feature_description_entries carries more than one row for {} -- the seam \
             clears a unit only when its single row clears, so a duplicate would make which \
             row won undefined",
            f.unit_id
        );
        assert!(
            !f.expected_value_at_level_by_arg.is_empty(),
            "{} pins no expected values at all -- a fixture that asserts nothing cannot fail",
            f.unit_id
        );
    }
}

#[test]
fn class_feature_description_pinned_upstream_lst_still_hashes_to_the_pinned_sha256() {
    let Some(data_root) = pcgen_data_root() else {
        eprintln!("PCGEN_CORPUS_ROOT unset and no HOME; skipping");
        return;
    };
    let fixtures = load_class_feature_description_fixtures(&repo_root());
    assert!(!fixtures.is_empty(), "class_feature_description_entries must not be empty");
    let mut hashed: BTreeMap<String, String> = BTreeMap::new();
    let mut checked = 0usize;
    for f in &fixtures {
        let path = data_root.join(&f.upstream_lst);
        let Ok(bytes) = std::fs::read(&path) else {
            panic!(
                "{}: the fixture pins upstream_lst {:?}, which does not exist under the \
                 pinned oracle",
                f.unit_id, f.upstream_lst
            );
        };
        let actual =
            hashed.entry(f.upstream_lst.clone()).or_insert_with(|| sha256_hex(&bytes)).clone();
        assert_eq!(
            actual, f.upstream_lst_sha256,
            "{}: {} has moved since this fixture was derived (pinned sha256 {}, actual {}). \
             Re-derive against the current pin rather than editing the hash.",
            f.unit_id, f.upstream_lst, f.upstream_lst_sha256, actual
        );
        checked += 1;
    }
    assert!(checked > 0, "no fixture row was checked");
}

#[test]
fn class_feature_description_pinned_corpus_field_is_byte_identical_to_the_upstream_lst() {
    let Some(data_root) = pcgen_data_root() else {
        eprintln!("PCGEN_CORPUS_ROOT unset and no HOME; skipping");
        return;
    };
    let fixtures = load_class_feature_description_fixtures(&repo_root());
    assert!(!fixtures.is_empty(), "class_feature_description_entries must not be empty");
    let mut checked = 0usize;
    for f in &fixtures {
        let path = data_root.join(&f.upstream_lst);
        let Ok(text) = std::fs::read_to_string(&path) else {
            panic!("{}: cannot read {:?}", f.unit_id, f.upstream_lst);
        };
        let Some(line) = text.split('\n').nth((f.upstream_line as usize).saturating_sub(1)) else {
            panic!(
                "{}: {:?} has no line {} anymore",
                f.unit_id, f.upstream_lst, f.upstream_line
            );
        };
        // `corpus_field` is stored as `"DESC:<verbatim token value>"`.
        let field_text = f.corpus_field.strip_prefix("DESC:").unwrap_or(&f.corpus_field);
        assert!(
            line.contains(field_text),
            "{}: pinned line {} of {:?} does not contain the pinned corpus_field text \
             {field_text:?}\nline: {line:?}",
            f.unit_id,
            f.upstream_line,
            f.upstream_lst
        );
        checked += 1;
    }
    assert!(checked > 0, "no fixture row was checked");
}

#[test]
fn class_feature_description_expected_values_are_re_derivable_from_the_pinned_corpus_field() {
    let Some(data_root) = pcgen_data_root() else {
        eprintln!("PCGEN_CORPUS_ROOT unset and no HOME; skipping");
        return;
    };
    let fixtures = load_class_feature_description_fixtures(&repo_root());
    assert!(!fixtures.is_empty(), "class_feature_description_entries must not be empty");
    let mut checked = 0usize;

    for f in &fixtures {
        let path = data_root.join(&f.upstream_lst);
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("{}: cannot read {:?}: {e}", f.unit_id, f.upstream_lst));
        let line = text
            .split('\n')
            .nth((f.upstream_line as usize).saturating_sub(1))
            .unwrap_or_else(|| panic!("{}: no line {}", f.unit_id, f.upstream_line));
        let fields: Vec<&str> = line.split('\t').filter(|s| !s.is_empty()).collect();

        // Independently re-extract every `BONUS:VAR|<name[,...]>|<formula>` field on the pinned
        // line -- a THIRD reading of the raw bytes, never taken from the fixture's own
        // `bonus_var_chain` (which the Python generator wrote) nor from `data/corpus/`.
        let mut bonus_vars: BTreeMap<String, String> = BTreeMap::new();
        for field in &fields {
            let Some(rest) = field.strip_prefix("BONUS:VAR|") else { continue };
            let mut parts = rest.splitn(2, '|');
            let (Some(names), Some(formula_and_tail)) = (parts.next(), parts.next()) else {
                continue;
            };
            let formula = formula_and_tail.split('|').next().unwrap_or(formula_and_tail);
            for name in names.split(',') {
                let name = name.trim();
                if !name.is_empty() {
                    bonus_vars.insert(name.to_string(), formula.to_string());
                }
            }
        }
        assert!(
            !bonus_vars.is_empty(),
            "{}: no BONUS:VAR field found on the pinned line -- fixture derivation must have \
             read a different line",
            f.unit_id
        );

        for (arg_name, by_level) in &f.expected_value_at_level_by_arg {
            for (&level, &expected) in by_level {
                let resolved = reference_resolve_chain(
                    &bonus_vars,
                    &f.class_level_var,
                    level,
                    &f.ability_modifier_inputs,
                );
                let got = resolved.get(arg_name);
                assert_eq!(
                    got,
                    Some(&expected),
                    "{}: at level {level}, this file's OWN independent re-derivation from the \
                     pinned upstream bytes computes {arg_name}={got:?}, but the fixture states \
                     {expected} -- the fixture and this Rust reference evaluator disagree",
                    f.unit_id
                );
                checked += 1;
            }
        }
    }
    assert!(checked > 10, "expected a meaningful number of (unit, arg, level) checks, got {checked}");
}

/// The end-to-end proof: every fixtured unit clears the REAL, PRODUCTION bar check --
/// `run_bar_check`, the exact function the gate CLI and `v06_work_inventory` call -- which
/// drives the real `formula_interpreter::PcgenFormulaEvaluator` through the real
/// `class_feature_grant_consumer::resolve_pcgen_var_chain` against the LIVE `data/corpus/`
/// ingest. This is what actually banks a unit; the guarantees above only prove the fixture
/// itself is trustworthy.
#[test]
fn every_fixture_clears_the_real_production_bar_check() {
    let fixtures = load_class_feature_description_fixtures(&repo_root());
    assert!(!fixtures.is_empty(), "class_feature_description_entries must not be empty");
    let report = run_bar_check(&repo_root());
    let mut uncleared = Vec::new();
    for f in &fixtures {
        if !report.cleared.contains(&f.unit_id) {
            let reason = report
                .failures
                .get(&f.unit_id)
                .or_else(|| report.engine_does_not_hold.get(&f.unit_id))
                .cloned()
                .unwrap_or_else(|| "not reported cleared, failed, or engine-does-not-hold at all -- \
                     unexpected report shape".to_string());
            uncleared.push(format!("{}: {reason}", f.unit_id));
        }
    }
    assert!(
        uncleared.is_empty(),
        "{} fixtured unit(s) did not clear the real production bar check:\n{}",
        uncleared.len(),
        uncleared.join("\n")
    );
}
