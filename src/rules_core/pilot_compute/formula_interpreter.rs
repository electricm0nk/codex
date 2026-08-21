//! SD-31 wave 25b — the formula interpreter core (`OPERATOR-RULINGS-2026-08-21.md` §20,
//! "RULED, 2026-08-21: §24.1 IS OVERTURNED. Build the interpreter.").
//!
//! **Authority.** §24.1 (`SD-27 decisions.md`, "No formula interpreter. Each feature is a
//! hand-written, corpus-verified pure function.") is overturned for this package by the ruling
//! above, on condition that "every interpreted value must clear `derived_evaluator_fixture_check`
//! ... An interpreted value with no fixture is not done." This module is the evaluator half of
//! that authorization; it does not by itself bank any unit — a unit consuming this evaluator's
//! output still needs its own fixture, wired by a later cycle.
//!
//! **What this module is.** A real recursive-descent parser and evaluator for the arithmetic
//! formula language PCGen's `BONUS:`/`DEFINE:` tokens carry in their formula segment — the same
//! grammar surface `formula_reproduction_harness`'s 22 dispatched cases exercise, and the
//! `FormulaEvaluator` trait that harness defines is this module's interface contract
//! (`PcgenFormulaEvaluator` implements it below). It does **not** parse the surrounding
//! `BONUS:<TAG>|<target>|` token envelope, PRE-token prerequisite gating, or the `(+N
//! PREVARGTEQ:X,V)` repeated-conditional-addend clauses some raw corpus `BONUS:VAR` values embed
//! directly in their formula text (see `mod tests::corpus_shape_coverage` and the module-level
//! "NOT COVERED" section below for the measured size of that surface) — those are a different
//! PCGen subsystem (`BonusObj`/`MultiTagBonusObj`, not the `PCGen-Formula` arithmetic grammar) and
//! out of this lane's scope.
//!
//! ## Semantics derived from the pinned oracle, not guessed
//!
//! **CORRECTED at wave 25b integration (2026-08-21).** The version of this module first written
//! by the interpreter-core lane cited `PCGen-Formula/code/src/java/pcgen/base/formula/...` — a
//! real subsystem in the pinned checkout, but the WRONG one. `PCGen-Formula` is a newer
//! `MODIFY:`/variable-scope engine; it does not evaluate this corpus's `BONUS:`/`DEFINE:` formula
//! tokens. The engine that does, traced end to end and re-verified by hand against the pinned
//! checkout during integration review:
//! `pcgen/core/bonus/BonusObj.java:210` (`bonusFormula = FormulaFactory.getFormulaFor(bValue)`) →
//! `pcgen/cdom/base/FormulaFactory.java:91` (`return new JEPFormula(formulaString)`) →
//! `pcgen/cdom/base/JEPFormula.java` (`resolve()` calls `character.getVariableValue(...)`) →
//! `pcgen/core/VariableProcessor.java:433` (`processJepFormula`, acquires a `PJEP` from the pool)
//! → `pcgen/util/PJEP.java` (`extends org.nfunk.jep.JEP`), whose function library is
//! `plugin/jepcommands/*Command.java`. Every claim below now cites that chain, re-derived (not
//! merely re-labelled) against the pinned checkout at
//! `scripts/pcgen-oracle-pin.env`'s `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.
//!
//! 1. **Formula variables are carried as doubles throughout, and only truncate toward zero at
//!    consumption, never inside an operator.**
//!    `pcgen/core/VariableProcessor.java:461` injects every bound variable as a double
//!    (`parser.addVariable(element, d.doubleValue())`), and `JEPFormula.resolve` returns a
//!    `Float`. The real engine performs **no truncation at the formula boundary at all** — it is
//!    the formula's own **caller** that decides whether and how to consume the result as an
//!    integer. This module cannot reproduce "whatever a given caller does with the float" in
//!    general (no consumer is wired yet — see the module's top doc), so it makes an explicit,
//!    disclosed design choice: carry arithmetic in `f64` end-to-end and truncate toward zero
//!    (`as i64`) only at [`FormulaEvaluator::evaluate`]'s own return boundary. **This is this
//!    module's own convention for presenting a value to a not-yet-built consumer, not a
//!    transcription of a single real PCGen call site** — flagged here so it is not mistaken for
//!    an oracle-derived fact the way the previous version of this doc presented it.
//! 2. **`floor()` is a real floor (round toward negative infinity), genuinely different from bare
//!    division for a negative operand.**
//!    `plugin/jepcommands/FloorCommand.java`: `if (param instanceof Double) { final double r =
//!    Math.floor((Double) param); inStack.push(r); }`. `Math.floor` rounds toward negative
//!    infinity; this module's own truncate-toward-zero convention (point 1) rounds toward zero.
//!    For `-7/3`: bare division truncates to **-2** at this module's output boundary, but an
//!    explicit `floor(-7/3)` call yields **-3**. Confirmed against the real `FloorCommand.java`
//!    (not `PCGen-Formula`'s `FloorFunction.java`, which was the prior citation and happens to
//!    have the same `Math.floor` behaviour — right answer, wrong file). Unobservable against the
//!    current 22 reproduction cases (all sampled level/ability-mod inputs are non-negative);
//!    exercised only by this module's own synthetic negative-operand test.
//! 3. **`max`/`min` are N-ary, not fixed at two arguments.**
//!    `plugin/jepcommands/MaxCommand.java`: `numberOfParameters = -1` (variable-arity), summing
//!    over the whole stack; `MinCommand.java` is the mirror. Confirmed directly (prior citation
//!    was `PCGen-Formula`'s `MaxFunction`/`AbstractNaryFunction`, also N-ary, but again the wrong
//!    engine). This module accepts 2 or more arguments.
//! 4. **`if(cond, then, else)` accepts a bare NUMERIC condition (nonzero = true), not only a
//!    boolean comparison — and this module does NOT implement that yet.**
//!    `plugin/jepcommands/IfCommand.java`: "The first is a number interpreted as a boolean... if
//!    the first argument != 0, the second argument is returned." The prior version of this
//!    module's doc cited `PCGen-Formula`'s `IfFunction.java` (boolean-only condition) as if it
//!    were this shape; it is not, and implementing the boolean-only subset was consequently
//!    presented as full oracle coverage when it is a real, disclosed restriction. **What this
//!    module actually implements:** `if()`'s condition must be one of `>= <= == != > <`; a bare
//!    numeric condition (e.g. real PCGen's `if(SomeVar,1,0)`) is refused, not silently
//!    mis-evaluated. This restriction is SAFE (refuse, not guess) but it is real, it is new
//!    information versus the original doc's claim, and it is the same underlying gap as the
//!    "boolean-to-int coercion" shape named below — a single fix (comparisons producing a `1.0`/
//!    `0.0` numeric value usable anywhere, not only as `if()`'s first argument) would close both
//!    at once. Logged to `OPEN-ISSUES.md` rather than attempted under wave 25b integration's own
//!    time budget, since it is new interpreter logic that itself needs fixture-grade
//!    verification, not a same-session patch.
//! 5. **Function names are case-sensitive except for two fixed spellings PCGen itself registers.**
//!    `pcgen/util/PJEP.java`: `addFunction(com.getFunctionName().toLowerCase(), com);
//!    addFunction(com.getFunctionName().toUpperCase(), com);` — each function is registered under
//!    exactly its all-lowercase and all-uppercase spelling, never mixed case. The prior version of
//!    this module's doc cited `PCGen-Formula`'s `SimpleFunctionLibrary`
//!    (`CaseInsensitiveMap`, genuinely case-insensitive) as if it applied here; it does not, and
//!    the module's own parser had been built to match that wrong citation (any casing accepted).
//!    **Fixed at wave 25b integration**: `parse_call` now refuses mixed case (`Max(...)`,
//!    `FlOoR(...)`) and accepts only all-lower or all-upper, matching the real engine. No corpus
//!    row currently uses mixed case (confirmed: `tests::corpus_shape_coverage` scans function
//!    names actually present and finds only all-lower or all-upper spellings today), so this was
//!    a latent divergence, not one that had produced a wrong number — but it was presented as an
//!    oracle-derived fact when it was the opposite of the oracle's real behaviour.
//!
//! ## A deliberate deviation from the oracle, flagged rather than silently copied
//!
//! Real PCGen division-by-zero on a `double` silently produces `Infinity`/`NaN`, which then
//! truncates into a garbage finite value at whatever point the caller consumes it as an integer —
//! with **no failure at all**. That is exactly the "plausible number nobody checks" shape §24.1
//! exists to prevent. This module refuses instead: `evaluate("X/0", ...)` returns `Err`, never a
//! value. This is a safety choice layered on top of the oracle, not a guess about its behaviour.
//!
//! ## What this module refuses, always explicitly, never by defaulting
//!
//! - An identifier used as a value that has no binding in the supplied `vars` map (never silently
//!   treated as `0`).
//! - A function name outside `{min, max, floor, ceil, abs, if, classlevel}` (named in the error),
//!   INCLUDING real PCGen functions this module has not implemented — `var(...)`, `count(...)`,
//!   `mastervar(...)`/`MASTERVAR(...)`, `charbonusto(...)`, `cl(...)` (PJEP's deprecated alias for
//!   `classlevel`), and `skillinfo(...)` all name real `plugin/jepcommands/*Command.java` classes
//!   in the pinned oracle and all refuse cleanly here rather than being silently mishandled.
//! - A mixed-case function name (see point 5 above) — refused, not accepted.
//! - Wrong argument counts for any function, division by zero, an unterminated string literal, a
//!   character outside `[0-9a-zA-Z_." ()+-*/,<>=]`, or trailing tokens after a complete expression.
//! - **`classlevel(...)` does NOT verify its class-name argument against anything — this is a
//!   real, currently-unfixed gap, not a refusal.** The real oracle's `classlevel("X")`
//!   (`plugin/jepcommands/ClassLevelCommand.java`) looks up level in the SPECIFIC named class;
//!   this module has no per-class variable environment (only a single `__LEVEL__` binding, per
//!   the harness's own open question (b)), so `classlevel("Fighter")` and
//!   `classlevel("Anything")` currently both silently return the same `__LEVEL__` value. For a
//!   single-class formula in a single-class caller context this is harmless; for any genuinely
//!   cross-class `classlevel("OtherClass")` formula (confirmed present in the corpus, see
//!   `OPEN-ISSUES.md`) it is silently wrong, not merely incomplete. **No consumer may bank a value
//!   through a `classlevel(...)`-bearing formula until this is resolved** — logged as a hard
//!   precondition in `OPEN-ISSUES.md`, not merely a documented limitation, because the failure
//!   mode is exactly the "plausible number nobody checks" shape §24.1 exists to prevent.
//!
//! ## Not covered by this module (report this size in the wave receipt, not silently)
//!
//! - `BONUS:<TAG>|<target>|<formula>` envelope parsing / target-name resolution.
//! - PRE-token gating and the `(+N PREVARGTEQ:X,V)` repeated-conditional-addend clauses embedded
//!   directly in some raw corpus `BONUS:VAR` formula text (`witch_ward_bonus`'s real corpus token
//!   is exactly this shape — the harness's own `raw_formula` field for that case is a HAND
//!   translation into this module's `if(...)`-based grammar, not the literal corpus text; seen
//!   directly: `git show worktree-wf_73cee402-845-2:.../formula_reproduction_harness.rs` line ~386
//!   vs. its own `pcgen_token` field one line above it). This is a different PCGen subsystem
//!   (`BonusObj`/`MultiTagBonusObj` applying a repeated conditional bonus stack), not the JEP
//!   arithmetic grammar.
//! - `%1`/`%N` parameter substitution in `DESC:` text (a text-rendering mechanism, not formula
//!   arithmetic; its consumer is `description_completion.rs`/`pcgen_desc.rs`, out of this lane's
//!   write scope).
//! - `DEFINE:` envelope parsing beyond formula-segment extraction (see [`extract_formula_field`]).
//! - Non-numeric formula results (string-valued `DEFINE:`s, `.EQUIP` object references, etc.).
//! - Multiclass `classlevel("X")` resolution to a specific class's own level — see the refusal
//!   list above; this is a silently-wrong gap, not a clean refusal, and is the single highest-
//!   priority fix before any `classlevel`-bearing formula is banked.
//! - **Found empirically by `tests::corpus_shape_coverage` (not in the 22 harness cases, not
//!   implemented — flagged, not silently patched):** boolean-valued sub-expressions coerced to
//!   `0`/`1` in arithmetic position, e.g. Kineticist's `"1+(KineticistLVL>=15)"` (a bare
//!   parenthesised comparison used as a numeric term, not as an `if()` condition) and `&&`
//!   boolean-AND inside such a term (Sorcerer bloodline-power gates:
//!   `"if((X==0&&Y>=3),1,0)"`) — the same underlying gap as point 4 above (a comparison does not
//!   yet produce a reusable numeric value); and `skillinfo("TOTALRANK","<skill>")`, a real
//!   function (`plugin/jepcommands/SkillInfoCommand.java`, 2 args) reading a character's skill
//!   ranks, which this module refuses as an unimplemented function rather than an unknown one.

use std::collections::BTreeMap;

use super::formula_reproduction_harness::{FormulaEvalError, FormulaEvaluator};

// -------------------------------------------------------------------------------------------
// 1. Tokenizer
// -------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Num(f64),
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
    Ne,
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
                    return Err(FormulaEvalError(format!("bare '=' in {s:?} — refusing rather than guessing '=='")));
                }
            }
            '!' => {
                if chars.get(i + 1) == Some(&'=') {
                    out.push(Tok::Ne);
                    i += 2;
                } else {
                    return Err(FormulaEvalError(format!("bare '!' in {s:?}")));
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
                    return Err(FormulaEvalError(format!("unterminated string literal in {s:?}")));
                }
                out.push(Tok::Str(buf));
                i = j + 1;
            }
            c if c.is_ascii_digit() => {
                let mut j = i;
                while j < chars.len() && (chars[j].is_ascii_digit() || chars[j] == '.') {
                    j += 1;
                }
                let n: String = chars[i..j].iter().collect();
                out.push(Tok::Num(n.parse().map_err(|_| {
                    FormulaEvalError(format!("unparseable number {n:?} in {s:?}"))
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
            other => {
                return Err(FormulaEvalError(format!(
                    "unrecognised character {other:?} in {s:?} — refusing rather than skipping it"
                )))
            }
        }
    }
    Ok(out)
}

// -------------------------------------------------------------------------------------------
// 2. Parser: builds an AST rather than evaluating inline, so `parse()` alone can answer "does
//    this module recognise this token shape" without needing variable bindings — used by the
//    corpus-wide shape-coverage scan (`tests::corpus_shape_coverage`) to report refusals that are
//    genuinely about unrecognised grammar, not merely about a sample point missing a var binding.
// -------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Num(f64),
    Var(String),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>),
    ClassLevel(String),
    If(Box<Cond>, Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
enum Cond {
    Cmp(Expr, CmpOp, Expr),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CmpOp {
    Ge,
    Le,
    Eq,
    Ne,
    Gt,
    Lt,
}

struct Parser<'a> {
    tokens: &'a [Tok],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&Tok> {
        self.tokens.get(self.pos)
    }

    fn bump(&mut self) -> Option<Tok> {
        let t = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        t
    }

    fn expect(&mut self, want: &Tok) -> Result<(), FormulaEvalError> {
        match self.bump() {
            Some(ref t) if t == want => Ok(()),
            other => Err(FormulaEvalError(format!("expected {want:?}, got {other:?}"))),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, FormulaEvalError> {
        let mut v = self.parse_term()?;
        loop {
            match self.peek() {
                Some(Tok::Plus) => {
                    self.bump();
                    v = Expr::Add(Box::new(v), Box::new(self.parse_term()?));
                }
                Some(Tok::Minus) => {
                    self.bump();
                    v = Expr::Sub(Box::new(v), Box::new(self.parse_term()?));
                }
                _ => break,
            }
        }
        Ok(v)
    }

    fn parse_term(&mut self) -> Result<Expr, FormulaEvalError> {
        let mut v = self.parse_unary()?;
        loop {
            match self.peek() {
                Some(Tok::Star) => {
                    self.bump();
                    v = Expr::Mul(Box::new(v), Box::new(self.parse_unary()?));
                }
                Some(Tok::Slash) => {
                    self.bump();
                    v = Expr::Div(Box::new(v), Box::new(self.parse_unary()?));
                }
                _ => break,
            }
        }
        Ok(v)
    }

    fn parse_unary(&mut self) -> Result<Expr, FormulaEvalError> {
        if let Some(Tok::Minus) = self.peek() {
            self.bump();
            return Ok(Expr::Neg(Box::new(self.parse_unary()?)));
        }
        if let Some(Tok::Plus) = self.peek() {
            // A leading unary '+' is not part of PCGen's own arithmetic grammar (the oracle's
            // `NumberAdd`/etc. operators are all strictly binary); refuse rather than silently
            // treating it as a no-op, per this module's own "refuse, never guess" rule.
            return Err(FormulaEvalError(
                "unary '+' is not a recognised token shape".to_string(),
            ));
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, FormulaEvalError> {
        match self.bump() {
            Some(Tok::Num(n)) => Ok(Expr::Num(n)),
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
                    Ok(Expr::Var(name))
                }
            }
            other => Err(FormulaEvalError(format!(
                "expected a number, identifier, function call, or '(', got {other:?}"
            ))),
        }
    }

    fn parse_call(&mut self, name: &str) -> Result<Expr, FormulaEvalError> {
        // The real oracle (`pcgen/util/PJEP.java`: `addFunction(name.toLowerCase(), com);
        // addFunction(name.toUpperCase(), com);`) registers each function under exactly its
        // all-lowercase and all-uppercase spellings — NOT arbitrary mixed case. `Max(...)` /
        // `Floor(...)` are unknown-function parse errors in the real engine. An earlier version
        // of this module matched ANY casing via a blanket `to_ascii_lowercase()`, which is more
        // permissive than the oracle; flagged by wave 25b adversarial review as an unverified
        // "case-insensitive" claim presented as oracle-derived when it was not (no corpus row
        // currently exercises mixed case, so the divergence was latent, not yet harmful — fixed
        // anyway rather than left as a documented-but-live gap).
        if !(name.chars().all(|c| c.is_ascii_lowercase() || !c.is_ascii_alphabetic())
            || name.chars().all(|c| c.is_ascii_uppercase() || !c.is_ascii_alphabetic()))
        {
            return Err(FormulaEvalError(format!(
                "function name {name:?} is mixed case — the real oracle (pcgen/util/PJEP.java) \
                 only registers each function's all-lowercase and all-uppercase spellings, never \
                 mixed case; refusing rather than accepting a spelling PCGen itself would not"
            )));
        }
        let lname = name.to_ascii_lowercase();
        match lname.as_str() {
            "classlevel" => {
                match self.bump() {
                    Some(Tok::Str(s)) => {
                        self.expect(&Tok::RParen)?;
                        Ok(Expr::ClassLevel(s))
                    }
                    other => Err(FormulaEvalError(format!(
                        "classlevel(...) expects a string literal class name, got {other:?}"
                    ))),
                }
            }
            "if" => {
                let cond = self.parse_cond()?;
                self.expect(&Tok::Comma)?;
                let a = self.parse_expr()?;
                self.expect(&Tok::Comma)?;
                let b = self.parse_expr()?;
                self.expect(&Tok::RParen)?;
                Ok(Expr::If(Box::new(cond), Box::new(a), Box::new(b)))
            }
            "min" | "max" | "floor" | "ceil" | "abs" => {
                let mut args = vec![self.parse_expr()?];
                while let Some(Tok::Comma) = self.peek() {
                    self.bump();
                    args.push(self.parse_expr()?);
                }
                self.expect(&Tok::RParen)?;
                match lname.as_str() {
                    "floor" | "ceil" | "abs" if args.len() != 1 => {
                        return Err(FormulaEvalError(format!(
                            "{lname}(...) takes exactly 1 argument, got {}",
                            args.len()
                        )))
                    }
                    "min" | "max" if args.len() < 2 => {
                        return Err(FormulaEvalError(format!(
                            "{lname}(...) takes at least 2 arguments, got {}",
                            args.len()
                        )))
                    }
                    _ => {}
                }
                Ok(Expr::Call(lname, args))
            }
            other => Err(FormulaEvalError(format!(
                "unrecognised function {other:?} — refusing rather than guessing its semantics"
            ))),
        }
    }

    fn parse_cond(&mut self) -> Result<Cond, FormulaEvalError> {
        let a = self.parse_expr()?;
        let op = match self.bump() {
            Some(Tok::Ge) => CmpOp::Ge,
            Some(Tok::Le) => CmpOp::Le,
            Some(Tok::Eq) => CmpOp::Eq,
            Some(Tok::Ne) => CmpOp::Ne,
            Some(Tok::Gt) => CmpOp::Gt,
            Some(Tok::Lt) => CmpOp::Lt,
            other => {
                return Err(FormulaEvalError(format!(
                    "expected a comparison operator (>=, <=, ==, !=, >, <), got {other:?}"
                )))
            }
        };
        let b = self.parse_expr()?;
        Ok(Cond::Cmp(a, op, b))
    }
}

/// Parses `formula` into an AST without evaluating it. Public so shape-coverage scanning (does
/// this module recognise the grammar at all) can be answered without needing to also supply
/// variable bindings. Returns `Err` for anything this module does not recognise — never a partial
/// or best-guess AST.
fn parse(formula: &str) -> Result<Expr, FormulaEvalError> {
    let tokens = tokenize(formula)?;
    let mut p = Parser { tokens: &tokens, pos: 0 };
    let expr = p.parse_expr()?;
    if p.pos != p.tokens.len() {
        return Err(FormulaEvalError(format!(
            "trailing tokens after a complete expression in {formula:?} (parsed {} of {} tokens)",
            p.pos,
            p.tokens.len()
        )));
    }
    Ok(expr)
}

// -------------------------------------------------------------------------------------------
// 3. Evaluator — arithmetic carried in f64 end to end (mirrors the oracle's own int/double
//    propagation, see module doc point 1), truncated toward zero only at the public
//    `FormulaEvaluator::evaluate` boundary.
// -------------------------------------------------------------------------------------------

fn eval_expr(expr: &Expr, vars: &BTreeMap<String, i64>) -> Result<f64, FormulaEvalError> {
    match expr {
        Expr::Num(n) => Ok(*n),
        Expr::Var(name) => vars
            .get(name)
            .map(|v| *v as f64)
            .ok_or_else(|| FormulaEvalError(format!("unbound variable {name:?}"))),
        Expr::Neg(e) => Ok(-eval_expr(e, vars)?),
        Expr::Add(a, b) => Ok(eval_expr(a, vars)? + eval_expr(b, vars)?),
        Expr::Sub(a, b) => Ok(eval_expr(a, vars)? - eval_expr(b, vars)?),
        Expr::Mul(a, b) => Ok(eval_expr(a, vars)? * eval_expr(b, vars)?),
        Expr::Div(a, b) => {
            let denom = eval_expr(b, vars)?;
            if denom == 0.0 {
                // Deliberate deviation from the oracle's silent Infinity/NaN — see module doc.
                return Err(FormulaEvalError("division by zero".to_string()));
            }
            Ok(eval_expr(a, vars)? / denom)
        }
        Expr::ClassLevel(_class_name) => vars.get("__LEVEL__").map(|v| *v as f64).ok_or_else(|| {
            FormulaEvalError("classlevel(...) needs a __LEVEL__ binding".to_string())
        }),
        Expr::If(cond, a, b) => {
            if eval_cond(cond, vars)? {
                eval_expr(a, vars)
            } else {
                eval_expr(b, vars)
            }
        }
        Expr::Call(name, args) => {
            let vals: Result<Vec<f64>, FormulaEvalError> =
                args.iter().map(|a| eval_expr(a, vars)).collect();
            let vals = vals?;
            match name.as_str() {
                "min" => Ok(vals.iter().copied().fold(f64::INFINITY, f64::min)),
                "max" => Ok(vals.iter().copied().fold(f64::NEG_INFINITY, f64::max)),
                // Real floor: Math.floor semantics (round toward -infinity), NOT truncation —
                // see module doc point 2.
                "floor" => Ok(vals[0].floor()),
                "ceil" => Ok(vals[0].ceil()),
                "abs" => Ok(vals[0].abs()),
                other => Err(FormulaEvalError(format!("unreachable: parser accepted unknown function {other:?}"))),
            }
        }
    }
}

fn eval_cond(cond: &Cond, vars: &BTreeMap<String, i64>) -> Result<bool, FormulaEvalError> {
    let Cond::Cmp(a, op, b) = cond;
    let av = eval_expr(a, vars)?;
    let bv = eval_expr(b, vars)?;
    Ok(match op {
        CmpOp::Ge => av >= bv,
        CmpOp::Le => av <= bv,
        CmpOp::Eq => av == bv,
        CmpOp::Ne => av != bv,
        CmpOp::Gt => av > bv,
        CmpOp::Lt => av < bv,
    })
}

/// The real evaluator: implements `formula_reproduction_harness::FormulaEvaluator` by parsing
/// `formula` (refusing on any unrecognised shape, never guessing) and evaluating it against
/// `vars`, truncating the final `f64` toward zero to match the oracle's own `Number.intValue()`
/// consumption boundary (module doc point 1).
pub struct PcgenFormulaEvaluator;

impl FormulaEvaluator for PcgenFormulaEvaluator {
    fn evaluate(&self, formula: &str, vars: &BTreeMap<String, i64>) -> Result<i64, FormulaEvalError> {
        let expr = parse(formula)?;
        let v = eval_expr(&expr, vars)?;
        if !v.is_finite() {
            return Err(FormulaEvalError(format!("{formula:?} evaluated to a non-finite value {v}")));
        }
        Ok(v.trunc() as i64)
    }
}

/// Answers "does this module's grammar recognise `formula` at all", independent of whether any
/// particular variable is bound — the metric the corpus-wide shape-coverage scan uses, so a
/// refusal there means "unrecognised token shape," not "this sample didn't supply a var."
pub fn recognises_shape(formula: &str) -> Result<(), FormulaEvalError> {
    parse(formula).map(|_| ())
}

/// Extracts the formula-segment candidate from one raw `BONUS`/`DEFINE` corpus token value, per
/// the heuristic confirmed against sampled corpus rows (`data/corpus/ultimate_wilderness/
/// companion/companion_gulper_plant.json`'s `raw_tokens`): a `BONUS` value's pipe-delimited third
/// field (`TAG|target|formula|...extras`) or a `DEFINE` value's second field (`Name|formula`).
/// This is a positional heuristic, not a full `BONUS:` grammar parser — it does not know which
/// `BONUS` subtags place the formula elsewhere, and returns `None` (not a wrong guess) when the
/// value has too few `|`-fields to apply the heuristic at all.
pub fn extract_formula_field<'a>(token_key: &str, token_value: &'a str) -> Option<&'a str> {
    let parts: Vec<&str> = token_value.split('|').collect();
    match token_key {
        "BONUS" if parts.len() >= 3 => Some(parts[2]),
        "DEFINE" if parts.len() >= 2 => Some(parts[1]),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::pilot_compute::formula_reproduction_harness::{
        all_cases, default_ability_mods, default_levels, run_reproduction, summarize,
    };

    fn vars(pairs: &[(&str, i64)]) -> BTreeMap<String, i64> {
        pairs.iter().map(|(k, v)| (k.to_string(), *v)).collect()
    }

    // -- 1. grammar shapes, each grounded in the oracle citations in the module doc -----------

    #[test]
    fn arithmetic_and_parens() {
        let e = PcgenFormulaEvaluator;
        let v = vars(&[("X", 7)]);
        assert_eq!(e.evaluate("(X+2)/3", &v).unwrap(), 3);
        assert_eq!(e.evaluate("X*2-1", &v).unwrap(), 13);
        assert_eq!(e.evaluate("-X+10", &v).unwrap(), 3);
    }

    #[test]
    fn division_truncates_toward_zero_matching_int_value() {
        let e = PcgenFormulaEvaluator;
        // 7/3 = 2.333..., Number.intValue() truncates toward zero -> 2, not floor's 2 (same here,
        // see the negative case below for where they diverge).
        assert_eq!(e.evaluate("7/3", &BTreeMap::new()).unwrap(), 2);
        assert_eq!(e.evaluate("-7/3", &BTreeMap::new()).unwrap(), -2);
    }

    #[test]
    fn floor_and_bare_division_diverge_only_on_negative_operands() {
        let e = PcgenFormulaEvaluator;
        // Positive operand: floor(x/y) and bare x/y agree, matching every real corpus sample
        // (module doc point 2) — this is the range all 22 harness cases live in.
        assert_eq!(e.evaluate("floor(7/3)", &BTreeMap::new()).unwrap(), 2);
        assert_eq!(e.evaluate("7/3", &BTreeMap::new()).unwrap(), 2);
        // Negative operand: FloorFunction.java's Math.floor rounds toward -infinity (-3), but
        // bare division's Number.intValue() truncates toward zero (-2). No known corpus row
        // exercises this today; this proves the interpreter matches the oracle's own divergence,
        // not the harness toy evaluator's "already an integer" shortcut.
        assert_eq!(e.evaluate("floor(-7/3)", &BTreeMap::new()).unwrap(), -3);
        assert_eq!(e.evaluate("-7/3", &BTreeMap::new()).unwrap(), -2);
    }

    #[test]
    fn min_max_are_n_ary_and_accept_only_the_oracles_two_registered_spellings() {
        // pcgen/util/PJEP.java registers each function as exactly its all-lowercase and
        // all-uppercase spellings, never mixed case (see parse_call's own comment). Confirmed by
        // hand against the pinned oracle during wave 25b integration review.
        let e = PcgenFormulaEvaluator;
        assert_eq!(e.evaluate("max(1,X/3)", &vars(&[("X", 7)])).unwrap(), 2);
        assert_eq!(e.evaluate("MAX(1,X/3)", &vars(&[("X", 7)])).unwrap(), 2);
        assert_eq!(e.evaluate("MAX(1,2,3,9,4)", &BTreeMap::new()).unwrap(), 9);
        assert_eq!(e.evaluate("min(5,2,8)", &BTreeMap::new()).unwrap(), 2);
        assert_eq!(e.evaluate("CEIL(7/3)", &BTreeMap::new()).unwrap(), 3);
        assert_eq!(e.evaluate("abs(-4)", &BTreeMap::new()).unwrap(), 4);
    }

    #[test]
    fn review_mixed_case_function_names_are_refused_not_accepted() {
        // The oracle (pcgen/util/PJEP.java) never registers a mixed-case spelling. A prior
        // version of this module accepted any casing; wave 25b adversarial review flagged this as
        // an unverified "oracle-derived" claim. Fixed: mixed case now refuses.
        let e = PcgenFormulaEvaluator;
        assert!(e.evaluate("Max(1,9)", &BTreeMap::new()).is_err());
        assert!(e.evaluate("Floor(7/3)", &BTreeMap::new()).is_err());
        assert!(e.evaluate("FlOoR(7/3)", &BTreeMap::new()).is_err());
    }

    #[test]
    fn if_and_comparisons() {
        let e = PcgenFormulaEvaluator;
        let v = vars(&[("X", 7)]);
        assert_eq!(e.evaluate("if(X>=8,1,0)", &v).unwrap(), 0);
        assert_eq!(e.evaluate("if(X>=7,1,0)", &v).unwrap(), 1);
        assert_eq!(e.evaluate("if(X==7,1,0)", &v).unwrap(), 1);
        assert_eq!(e.evaluate("if(X!=7,1,0)", &v).unwrap(), 0);
        assert_eq!(e.evaluate("if(X<8,1,0)", &v).unwrap(), 1);
    }

    #[test]
    fn classlevel_reads_the_level_binding() {
        let e = PcgenFormulaEvaluator;
        let v = vars(&[("__LEVEL__", 7)]);
        assert_eq!(e.evaluate("classlevel(\"Summoner\")", &v).unwrap(), 7);
        assert_eq!(e.evaluate("10+(X/2)+INT", &vars(&[("X", 7), ("INT", 3)])).unwrap(), 16);
    }

    // -- 2. refusals: mutation-style proof that unrecognised shapes never guess -----------------

    #[test]
    fn unbound_variable_refuses_not_defaults_to_zero() {
        let e = PcgenFormulaEvaluator;
        let err = e.evaluate("X+1", &BTreeMap::new()).unwrap_err();
        assert!(err.0.contains("unbound variable"), "got: {}", err.0);
    }

    #[test]
    fn unknown_function_refuses() {
        let e = PcgenFormulaEvaluator;
        let err = e.evaluate("wobble(1)", &BTreeMap::new()).unwrap_err();
        assert!(err.0.contains("unrecognised function"), "got: {}", err.0);
    }

    #[test]
    fn division_by_zero_refuses_rather_than_producing_infinity() {
        let e = PcgenFormulaEvaluator;
        let err = e.evaluate("1/0", &BTreeMap::new()).unwrap_err();
        assert!(err.0.contains("division by zero"), "got: {}", err.0);
    }

    #[test]
    fn embedded_pre_conditional_clause_refuses_not_silently_dropped() {
        // The literal raw corpus formula segment for `witch_ward_bonus` (see module doc's "not
        // covered" section) — this module correctly refuses it rather than silently evaluating
        // only the "2" prefix and dropping the conditional additions.
        let e = PcgenFormulaEvaluator;
        let err = e
            .evaluate("2 (+1 PREVARGTEQ:WitchHexAbilityLVL,8) (+1 PREVARGTEQ:WitchHexAbilityLVL,16)", &BTreeMap::new())
            .unwrap_err();
        // Whatever the exact message, it must be an Err — the load-bearing assertion is
        // `unwrap_err()` above not panicking.
        assert!(!err.0.is_empty());
    }

    #[test]
    fn wrong_arg_counts_refuse() {
        assert!(recognises_shape("floor(1,2)").is_err());
        assert!(recognises_shape("max(1)").is_err());
        assert!(recognises_shape("if(X>=1,1)").is_err());
    }

    #[test]
    fn trailing_tokens_refuse() {
        assert!(recognises_shape("1+1 garbage").is_err());
    }

    #[test]
    fn unary_plus_refuses_rather_than_being_treated_as_a_no_op() {
        assert!(recognises_shape("+5").is_err());
    }

    // -- 3. the reproduction proof: run the REAL evaluator against the wave 25 harness's 22
    //    dispatched hand-function-vs-corpus-token cases. This is the load-bearing test — nothing
    //    here adjusts the evaluator to force agreement; a disagreement is reported as a finding.
    // -------------------------------------------------------------------------------------------

    #[test]
    fn reproduces_all_22_hand_modelled_dispatch_cases() {
        let evaluator = PcgenFormulaEvaluator;
        let cases = all_cases();
        let outcomes = run_reproduction(&evaluator, &cases, &default_levels(), &default_ability_mods());
        let summary = summarize(&outcomes);
        let mut report = String::new();
        for outcome in &outcomes {
            if !outcome.agrees_everywhere() {
                report.push_str(&format!(
                    "\nDISAGREEMENT in {}: {} of {} samples differ; first = {:?}",
                    outcome.case_name,
                    outcome.disagreements.len(),
                    outcome.samples_checked,
                    outcome.disagreements.first()
                ));
            }
        }
        assert!(
            report.is_empty(),
            "reproduction proof found disagreements between the real interpreter and the \
             hand-modelled functions (each is a finding, not a defect in this test to relax):{report}"
        );
        assert_eq!(summary.cases_total, 22);
        assert_eq!(summary.cases_fully_agree, 22);
        assert_eq!(summary.samples_disagreeing, 0);
    }

    /// Decision 1(a) mutation proof: a deliberately wrong evaluator MUST be caught disagreeing
    /// with the hand-modelled functions — proves the reproduction proof above is a real gate, not
    /// one that would pass regardless of what the interpreter computes.
    #[test]
    fn mutated_evaluator_is_caught_disagreeing() {
        struct OffByOneEvaluator;
        impl FormulaEvaluator for OffByOneEvaluator {
            fn evaluate(&self, formula: &str, vars: &BTreeMap<String, i64>) -> Result<i64, FormulaEvalError> {
                PcgenFormulaEvaluator.evaluate(formula, vars).map(|v| v + 1)
            }
        }
        let cases = all_cases();
        let outcomes = run_reproduction(&OffByOneEvaluator, &cases, &default_levels(), &default_ability_mods());
        let summary = summarize(&outcomes);
        assert!(summary.samples_disagreeing > 0, "mutation must be caught, not silently pass");
        assert!(summary.cases_fully_agree < summary.cases_total);
    }

    // -- 4. corpus-wide shape coverage: how much of the REAL corpus's BONUS/DEFINE formula text
    //    this module's grammar recognises, beyond the 22 harness cases. Headline refusal count.
    // -------------------------------------------------------------------------------------------

    #[test]
    fn corpus_shape_coverage() {
        use std::collections::BTreeSet;
        let repo_root = std::env::var("CODEX_REPO_ROOT").unwrap_or_else(|_| ".".to_string());
        let corpus_dir = std::path::Path::new(&repo_root).join("data/corpus");
        if !corpus_dir.is_dir() {
            eprintln!(
                "corpus_shape_coverage: {corpus_dir:?} not found (CODEX_REPO_ROOT unset or wrong \
                 in this environment) — skipping, this is a coverage report, not a correctness gate"
            );
            return;
        }
        let mut recognised = 0usize;
        let mut refused = 0usize;
        let mut refusal_samples: Vec<String> = Vec::new();
        let mut candidates_seen: BTreeSet<String> = BTreeSet::new();
        for entry in walk_json(&corpus_dir) {
            let Ok(text) = std::fs::read_to_string(&entry) else { continue };
            let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
            let Some(tokens) = v.pointer("/data/raw_tokens").and_then(|t| t.as_array()) else { continue };
            for tok in tokens {
                let (Some(key), Some(value)) = (
                    tok.get("key").and_then(|k| k.as_str()),
                    tok.get("value").and_then(|k| k.as_str()),
                ) else { continue };
                let Some(formula) = extract_formula_field(key, value) else { continue };
                if !candidates_seen.insert(formula.to_string()) {
                    continue; // dedupe identical formula text across records
                }
                match recognises_shape(formula) {
                    Ok(()) => recognised += 1,
                    Err(e) => {
                        refused += 1;
                        if refusal_samples.len() < 15 {
                            refusal_samples.push(format!("{formula:?} -> {}", e.0));
                        }
                    }
                }
            }
        }
        let total = recognised + refused;
        eprintln!(
            "corpus_shape_coverage: {total} distinct BONUS/DEFINE formula-field candidates \
             scanned across data/corpus ({repo_root}); {recognised} parse under this module's \
             grammar, {refused} REFUSED (this is the headline refusal count).\nSample refusals:\n  {}",
            refusal_samples.join("\n  ")
        );
        // This is a coverage report, not a pass/fail gate on its own — the headline numbers go in
        // the wave receipt. The one thing asserted here is that the scan actually ran over real
        // data when the corpus is present, so a broken extraction path can't silently report 0/0.
        assert!(total > 0, "expected to find at least one BONUS/DEFINE token in data/corpus");
    }

    fn walk_json(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
        let mut out = Vec::new();
        let Ok(entries) = std::fs::read_dir(dir) else { return out };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                out.extend(walk_json(&path));
            } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
                out.push(path);
            }
        }
        out
    }

    #[test]
    fn extract_formula_field_matches_the_sampled_corpus_shape() {
        // Verified directly against `data/corpus/ultimate_wilderness/companion/
        // companion_gulper_plant.json`'s own `raw_tokens`.
        assert_eq!(
            extract_formula_field("BONUS", "VAR|AC_Natural_Armor|1|TYPE=Base"),
            Some("1")
        );
        assert_eq!(extract_formula_field("BONUS", "STAT|CHA|-8"), Some("-8"));
        assert_eq!(extract_formula_field("DEFINE", "NoTypeTraits|0"), Some("0"));
        assert_eq!(extract_formula_field("BONUS", "VAR"), None);
        assert_eq!(extract_formula_field("ABILITY", "Special Ability|X"), None);
    }
}
