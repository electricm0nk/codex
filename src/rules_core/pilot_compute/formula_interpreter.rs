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
//! 3. **`max`/`min` are N-ary, not fixed at two arguments — INCLUDING one argument.**
//!    `plugin/jepcommands/MaxCommand.java`: `numberOfParameters = -1` (variable-arity); its `run`
//!    pops each parameter off the stack and folds via `first || param > result`, so a single
//!    parameter simply becomes the result (`first` is true on the only iteration) — the oracle
//!    accepts `max(X)` and returns `X` unchanged. `MinCommand.java` is the mirror. Confirmed
//!    directly (prior citation was `PCGen-Formula`'s `MaxFunction`/`AbstractNaryFunction`, also
//!    N-ary, but again the wrong engine). SD-32 T12 Epic 8 row 18 cycle 16: this module
//!    previously refused a single-argument `min`/`max` call as an unimplemented shape (a
//!    disclosed but INCORRECT restriction — real corpus records exist,
//!    e.g. `Cavalier Order of the Beast ~ Class Skills`'s `max(floor(CavalierLVL/2))`,
//!    `Barbarian Undead Bloodline (rage power) ~ Undead Blood (Lesser)`'s
//!    `max(floor(BarbarianLVL/2))`, `Voice of the Wild ~ Wild Knowledge`'s
//!    `max(floor(BardLVL/2))` — 3 corpus records total, sized via
//!    `python3 -c "..."` walking `data/corpus/**/*.json` for a balanced-paren `min(`/`max(` call
//!    with exactly one top-level comma-split argument, see cycle 16's own receipt for the full
//!    script). Fixed to accept 1 or more arguments, matching the oracle's variable-arity
//!    `MaxCommand`/`MinCommand` exactly — the parser already always supplies at least one
//!    argument by construction (`parse_call`'s `min`/`max`/`floor`/`ceil`/`abs` branch always
//!    pushes one `parse_expr()?` before the comma loop), so a genuine zero-argument call can
//!    never reach this arity check at all; the fix removes the now-provably-wrong `< 2` guard
//!    rather than replacing it with an unreachable `< 1` one.
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
//! - A function name outside `{min, max, floor, ceil, abs, if, classlevel, skillinfo}` (named in
//!   the error), INCLUDING real PCGen functions this module has not implemented — `var(...)`,
//!   `count(...)`, `mastervar(...)`/`MASTERVAR(...)`, `charbonusto(...)`, `cl(...)` (PJEP's
//!   deprecated alias for `classlevel`) all name real `plugin/jepcommands/*Command.java` classes
//!   in the pinned oracle and all refuse cleanly here rather than being silently mishandled.
//!   `skillinfo(...)` is now PARTIALLY implemented — see [`Expr::SkillInfoTotalRank`]'s own doc —
//!   and refuses cleanly on the five other real first-argument keywords it does not cover.
//! - A mixed-case function name (see point 5 above) — refused, not accepted.
//! - Wrong argument counts for any function, division by zero, an unterminated string literal, a
//!   character outside `[0-9a-zA-Z_." ()+-*/,<>=&]`, or trailing tokens after a complete
//!   expression. (`&` only ever appears as the pair `&&`; a lone `&` refuses — see the tokenizer's
//!   own comment.)
//! - `if(...)`'s condition still refuses a bare numeric value (real PCGen's `if(SomeVar,1,0)`
//!   form) — wave 26 widened what counts as a valid condition (a `&&`-chain of comparisons, not
//!   only a single one) but did not touch this restriction; see point 4 above and `parse_call`'s
//!   `"if"` arm.
//! - **`classlevel(...)` now verifies its class-name argument (SD-32 T12 Epic 8 row 18 cycle
//!   6) — CLOSED for the same-class case, still a clean refusal for the genuinely cross-class
//!   case.** The real oracle's `classlevel("X")` (`plugin/jepcommands/ClassLevelCommand.java`)
//!   looks up level in the SPECIFIC named class. This module now has a per-class variable
//!   environment: `Expr::ClassLevel(class_name)` looks up `CLASSLEVEL::<class_name>`, never a
//!   class-blind `__LEVEL__` slot. Every consumer in this codebase only ever knows ONE class's
//!   real level (the record's own granting class) and binds exactly that one `CLASSLEVEL::`
//!   key — so `classlevel("SameClass")` now resolves CORRECTLY (the prior version's silent
//!   coincidence, now an honest binding), while `classlevel("SomeOtherClass")` stays cleanly
//!   unbound and refuses, exactly the "refuse, never guess" contract this module holds
//!   everywhere else. **Genuine multiclass cross-referencing** (a caller that knows more than
//!   one class's level and could bind more than one `CLASSLEVEL::` key) is still not
//!   implemented by any consumer — no consumer in this codebase currently tracks multiple
//!   classes' levels at once — so a formula naming a class the caller truly does not know about
//!   still refuses, never fabricates. **No consumer may bank a value through a
//!   `classlevel(...)`-bearing formula whose class name it cannot bind** — the failure mode this
//!   guards against is exactly the "plausible number nobody checks" shape §24.1 exists to
//!   prevent.
//! - **Bare `classlevel()` with no argument now PARSES (SD-32 T12 Epic 8 row 18 cycle 9)** — a
//!   real corpus shape (`book_of_the_damned_volume_2/demoniac.json`'s BASEAB/SAVE formulas) the
//!   grammar previously refused outright. It reuses the exact same `CLASSLEVEL::<name>` lookup
//!   above with an empty string as the "no class name given" sentinel (`CLASSLEVEL::`) — this
//!   was a pure parser-shape gap, not a semantic widening: no consumer in this codebase binds
//!   the empty key yet, so evaluation still refuses cleanly until one explicitly does.
//!
//! ## Wave 26 shape closure (`OPERATOR-RULINGS-2026-08-21.md` §20 follow-on)
//!
//! Three of the four shapes wave 25b's refusal list named are now implemented, each cited above
//! at its own `Expr` variant: comparisons and `&&`-chains of comparisons as first-class
//! boolean-as-numeric values ([`Expr::Cmp`], [`Expr::And`] — closes both "boolean-to-int
//! coercion" and "the `&&` operator" as one grammar extension, since they are the same underlying
//! gap), and `skillinfo("TOTALRANK", ...)` ([`Expr::SkillInfoTotalRank`]). Measured effect:
//! `tests::corpus_shape_coverage`'s headline refusal count fell from 431 of 2,671 (16.1%) to the
//! number this test now reports — re-run it for the current figure, don't trust a number
//! transcribed into this comment.
//!
//! **The fourth named shape — "PREVARGTEQ-embedded conditional addends inside raw `BONUS:VAR`
//! text" — does not exist in the real corpus, and closing it was a correction of the wave 25
//! dispatch's own premise, not an implementation.** The dispatch brief's example,
//! `"2 (+1 PREVARGTEQ:X,8) (+1 PREVARGTEQ:X,16)"`, is literally the wave 25b module doc's own
//! prior text (see this file's git history) — which itself already disclosed that string as "a
//! HAND translation ... not the literal corpus text." Direct verification during wave 26: every
//! one of the 966 `BONUS`/`DEFINE` tokens in `data/corpus` containing `PREVARGTEQ` carries it as a
//! TRAILING pipe field (`BONUS:VAR|<target>|<formula>|PREVARGTEQ:<var>,<threshold>`), never
//! embedded inside the formula field itself (checked by splitting every such token's raw value on
//! its formula-field boundary and confirming zero contain `PREVARGTEQ` on the formula side — see
//! the wave receipt for the exact script). `extract_formula_field`'s existing positional heuristic
//! already extracts a clean, independently-parseable formula from every one of those 966 tokens
//! (e.g. `VAR|FamousPopulation|1|PREVARGTEQ:classlevel("Bard"),1` extracts `"1"`), which is why
//! this shape contributes **zero** of the 431 pre-wave-26 refusals — it was never in the refused
//! bucket to begin with. What IS real: the PRE-tag itself is silently DISCARDED by that
//! extraction (never parsed, never applied), so a naive consumer summing every `BONUS:VAR` token
//! sharing one target (`witch_ward_bonus`: three separate tokens, `2` + `1|PREVARGTEQ:...,8` +
//! `1|PREVARGTEQ:...,16`, real oracle behaviour per `PlayerCharacter.getTotalBonusTo` ->
//! `BonusManager.sumActiveBonusMap`: SUM only the entries whose own prerequisite currently passes)
//! would silently over-count at every level below the gate. That is a real gap — just a
//! summation-correctness gap for a future consumer, not a grammar-parsing refusal — and it is what
//! the sibling `bonus_stack_reader` module (this lane's "new BonusObj-shape reader module") closes:
//! given the raw `BONUS:VAR` tokens sharing one target and a character's current variable values,
//! it evaluates each token's own formula (via this module) gated by its own `PREVARGTEQ` (or
//! refuses, never silently drops, any OTHER PRE-tag kind it doesn't recognise), then sums only the
//! qualifying ones — see its own module doc for the `PreVariableTester.java`/`BonusManager.java`
//! citations.
//!
//! ## Not covered by this module (report this size in the wave receipt, not silently)
//!
//! - `BONUS:<TAG>|<target>|<formula>` envelope parsing / target-name resolution beyond the
//!   positional heuristic in [`extract_formula_field`].
//! - `%1`/`%N` parameter substitution in `DESC:` text (a text-rendering mechanism, not formula
//!   arithmetic; its consumer is `description_completion.rs`/`pcgen_desc.rs`, out of this lane's
//!   write scope).
//! - `DEFINE:` envelope parsing beyond formula-segment extraction (see [`extract_formula_field`]).
//! - Non-numeric formula results (string-valued `DEFINE:`s, `.EQUIP` object references, etc.).
//! - Genuine multiclass `classlevel("X")` resolution for X other than the caller's one known
//!   class — SD-32 T12 Epic 8 row 18 cycle 6 closed the same-class case (see the refusal list
//!   above); a caller that tracks more than one class's level at once and could therefore bind
//!   more than one `CLASSLEVEL::` key does not exist yet in this codebase.
//! - `classlevel("X", "APPLIEDAS=NONEPIC")` — SD-32 T12 Epic 8 row 18 cycle 14: implemented.
//!   Verified against `plugin/jepcommands/ClassLevelCommand.java`'s `run`: the qualifier caps the
//!   class level read at the game mode's non-epic ceiling
//!   (`cl += ";BEFORELEVEL=" + (maxNonEpicLevel+1)`), a cap that can never bind because this
//!   engine never models epic levels and every class chassis already gates its own level at its
//!   corpus-derived `max_level` (<= 20 for every real base class) — so the form is
//!   observationally identical to `classlevel("X")` for every character this engine represents.
//!   Any qualifier value other than the literal `APPLIEDAS=NONEPIC` still refuses, matching the
//!   oracle's own `ParseException` for an unrecognised `APPLIEDAS=` value.
//! - `skillinfo(...)`'s five other first-argument keywords (`modifier`, `rank`, `total`, `stat`,
//!   `misc`) — verified against `SkillInfoCommand.java` (same citation as `TOTALRANK`). CORRECTION
//!   (wave 26 integration cycle): two of the five (`rank`, `total`) ARE corpus-exercised (4 and 1
//!   real occurrences respectively) but this module does not implement them yet — refused rather than guessed at, not
//!   "not corpus-exercised" as an earlier version of this comment claimed. `modifier`/`stat`/
//!   `misc` are genuinely unexercised. See [`Expr::SkillInfoTotalRank`]'s own doc.
//! - `var(...)`, `count(...)`, `mastervar(...)`, `charbonusto(...)`, `cl(...)` — real
//!   `plugin/jepcommands/*Command.java` functions this module refuses as unimplemented: 31, 20, 3,
//!   2, and 1 corpus-formula refusals respectively (57 total; these five plus `skillinfo` summed
//!   to 80 "unrecognised function" refusals pre-wave-26 — `skillinfo` is the other 23, now
//!   implemented for its one corpus-exercised first argument, see above).
//! - Malformed corpus formula text that does not match ANY real JEP call shape regardless of this
//!   module's own grammar coverage — e.g. `if(Bloodrager_Draconic_BloodlineProgressionLVL>=7),1,0`
//!   (a real, literal `advanced_class_guide` corpus token: the closing `)` lands right after the
//!   condition, with `,1,0` trailing outside any parens at all). Confirmed this is the literal
//!   corpus byte content, not an extraction artifact. This module correctly refuses it; there is
//!   no legitimate grammar under which to accept it without guessing what the author meant.

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
    AndAnd,
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
            '&' => {
                // wave 26 shape closure (`OPERATOR-RULINGS-2026-08-21.md` §20 follow-on): the
                // real oracle's `&&` is `org.nfunk.jep.function.Logical` with id 0, registered as
                // `OP_AND` in `org.nfunk.jep.OperatorSet` (verified by decompiling the pinned
                // dependency jar itself, `org.scijava:jep:2.4.2` per `build.gradle:215` --
                // `PJEP extends org.nfunk.jep.JEP` per `pcgen/util/PJEP.java`, so this IS the
                // engine, not a guess at a third party's behaviour). `org.nfunk.jep.OperatorSet`
                // bytecode: `new Logical(0)` -> field `OP_AND`; `new Logical(1)` -> field `OP_OR`.
                // This module implements only `&&` (the only combinator any corpus formula uses --
                // verified during wave 26 by categorising every one of the pre-fix 431
                // `corpus_shape_coverage` refusal reasons: none is a bare `|`/`||` token, so there
                // is no corpus evidence either way for `||`'s grammar position, and this module
                // does not guess it) -- never guesses `||`. A single bare `&` is refused, matching
                // the bare `=`/`!`
                // pattern immediately above: real JEP has no single-`&` operator at all, so
                // accepting one here would be inventing a token the oracle doesn't define.
                if chars.get(i + 1) == Some(&'&') {
                    out.push(Tok::AndAnd);
                    i += 2;
                } else {
                    return Err(FormulaEvalError(format!("bare '&' in {s:?} — refusing rather than guessing '&&'")));
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
    /// `if(cond, then, else)`. `cond` must itself be a [`Expr::Cmp`] or [`Expr::And`] node --
    /// enforced in `parse_call`'s `"if"` arm, not here -- preserving the module's existing,
    /// documented "bare numeric condition refuses" restriction (module doc point 4) even though
    /// comparisons are now a general-purpose numeric value elsewhere in the grammar.
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    /// A bare comparison used as a value, not only as `if()`'s condition -- wave 26 shape closure.
    /// Confirmed to evaluate to a plain `Double` `1.0`/`0.0` (never a `java.lang.Boolean`) by
    /// decompiling `org.nfunk.jep.function.Comparative.run()` in the pinned `org.scijava:jep:2.4.2`
    /// dependency jar (`build.gradle:215`): the bytecode pushes `new Double(1.0)` on a pass and
    /// `new Double(0.0)` on a fail directly, with no `Boolean` boxing step at all. This is why
    /// `1+(KineticistLVL>=15)` needs no special coercion in the real engine -- the comparison
    /// operator was ALREADY numeric-valued, and `Add` never sees anything but two `Double`s.
    Cmp(Box<Expr>, CmpOp, Box<Expr>),
    /// `&&` combining two comparison values -- wave 26 shape closure. Confirmed by decompiling
    /// `org.nfunk.jep.function.Logical.run()` (id 0 = `OP_AND`, per `OperatorSet`'s bytecode) in
    /// the same pinned jar: both operands are popped and coerced to `double` (a `Number` via
    /// `doubleValue()`, or a `Boolean` via `booleanValue() ? 1.0 : 0.0` -- never reached here since
    /// `Cmp` already returns `Double`), ANDed as `(a != 0.0) && (b != 0.0)`, and the result pushed
    /// as `Double` `1.0`/`0.0`. Both operands are always evaluated -- this is a stack-based postfix
    /// evaluator, so the two operands are already popped off the stack before `Logical.run()`'s
    /// AND/OR switch ever runs; there is no short-circuit to reproduce.
    And(Box<Expr>, Box<Expr>),
    /// `skillinfo("TOTALRANK", "<skill name>")` -- wave 26 shape closure. Only the `"TOTALRANK"`
    /// first argument (case-insensitive, matching the real oracle's
    /// `"totalrank".equalsIgnoreCase(param1)`) is implemented; every other real
    /// `plugin/jepcommands/SkillInfoCommand.java` first-argument keyword (`modifier`, `rank`,
    /// `total`, `stat`, `misc`) is refused at parse time -- narrower than the real function.
    /// CORRECTION (wave 26 integration cycle, adversarial-review finding): the module doc and
    /// the parse-time refusal message both used to claim `"TOTALRANK"` is "the only one any
    /// corpus formula uses" -- that is FALSE. `grep -rhoP 'skillinfo\("[A-Za-z]+"'` over
    /// `data/corpus` finds 39 `TOTALRANK`, 4 `RANK`, and 1 `TOTAL` occurrences (20/2/1 distinct
    /// formula candidates); `RANK`/`TOTAL` are corpus-exercised too, just unimplemented so far,
    /// and correctly refuse rather than silently defaulting. `"TOTALRANK"` remains the only
    /// first argument this module implements, oracle-verified AND corpus-exercised, the same
    /// restriction already applied to `min`/`max`/`floor`/`ceil`/`abs`/`if`/`classlevel`.
    SkillInfoTotalRank(String),
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
                // wave 26 shape closure: a parenthesised group may hold a plain arithmetic
                // expression (as before) OR a boolean-valued one (`(X>=15)`, `(X==0&&Y>=3)`) used
                // as a numeric primary -- see `parse_arith_or_bool`'s own doc for the oracle
                // citations. Parens are transparent either way: the returned `Expr` is exactly
                // whatever was inside, never wrapped in an extra node.
                let v = self.parse_arith_or_bool()?;
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
                // SD-32 T12 Epic 8 row 18 cycle 9: bare `classlevel()` with NO argument is a
                // real corpus shape (`book_of_the_damned_volume_2/demoniac.json`'s BASEAB/SAVE
                // formulas: `classlevel()*3/4`, `(classlevel()+1)/2`, `(classlevel()+1)/3`) that
                // the pre-existing grammar refused outright before ever reaching evaluation — a
                // parser-shape gap, not a semantic one. Widened to accept it: `Expr::ClassLevel(
                // String::new())` reuses the SAME `CLASSLEVEL::<name>` lookup cycle 6 already
                // built, with the empty string as the "no class name given" sentinel — the exact
                // "unowned = \"\", never fabricated" convention this codebase already applies
                // elsewhere (e.g. row 18 cycle 8's bare-key header merge). No caller today binds
                // `CLASSLEVEL::` (empty key), so this still refuses cleanly at evaluation time
                // until a caller explicitly does — never silently answers with any other class's
                // level, exactly the "refuse, never guess" contract this module holds everywhere.
                if let Some(Tok::RParen) = self.peek() {
                    self.bump();
                    return Ok(Expr::ClassLevel(String::new()));
                }
                match self.bump() {
                    Some(Tok::Str(s)) => {
                        // SD-32 T12 Epic 8 row 18 cycle 14: the real 2-argument form,
                        // `classlevel("<class>","APPLIEDAS=NONEPIC")` (confirmed live corpus
                        // shape, e.g. `core_rulebook/class_feature/monk/standard_monk.json`'s
                        // `classlevel("Monk","APPLIEDAS=NONEPIC")` BASEAB/SAVE formulas, and
                        // `pathfinder_unchained/class_feature/monk/unchained_monk.json`'s same
                        // shape). Verified against the real oracle
                        // (`plugin/jepcommands/ClassLevelCommand.java`'s `run`): a second
                        // string argument starting `APPLIEDAS=` is parsed as a qualifier, not a
                        // second class name; `NONEPIC` is the only qualifier value the oracle
                        // recognises (any other value throws `ParseException("Did not
                        // understand APPLIEDAS=" + applied)` — refused there, refused here
                        // identically). Its real semantic effect is
                        // `cl += ";BEFORELEVEL=" + (mode.getMaxNonEpicLevel() + 1)` — the class
                        // level is capped at the game mode's non-epic level ceiling before
                        // being read. This engine never models epic levels at all (every class
                        // chassis this codebase resolves already gates its own level at that
                        // class's own corpus-derived `max_level`, `untabled_base_class_chassis.
                        // rs`/`generic_class_chassis.rs`, which for every real Pathfinder base
                        // class is <= 20, the same non-epic ceiling PCGen's own default game
                        // mode uses) — so the cap can never actually bind for any level this
                        // engine ever resolves, and `classlevel("<class>","APPLIEDAS=NONEPIC")`
                        // is observationally identical to `classlevel("<class>")` for every
                        // character this engine can represent. Reusing the SAME
                        // `Expr::ClassLevel` binding (not a new AST node) is therefore correct,
                        // not a shortcut around the real semantics — the qualifier is checked
                        // and REFUSED if it is anything other than the one value the oracle
                        // itself accepts, never silently ignored.
                        if let Some(Tok::Comma) = self.peek() {
                            self.bump();
                            match self.bump() {
                                Some(Tok::Str(q)) if q.eq_ignore_ascii_case("APPLIEDAS=NONEPIC") => {
                                    self.expect(&Tok::RParen)?;
                                    Ok(Expr::ClassLevel(s))
                                }
                                other => Err(FormulaEvalError(format!(
                                    "classlevel({s:?}, ...)'s second argument must be the \
                                     literal qualifier \"APPLIEDAS=NONEPIC\" (the only value the \
                                     real oracle's ClassLevelCommand.java accepts), got {other:?}"
                                ))),
                            }
                        } else {
                            self.expect(&Tok::RParen)?;
                            Ok(Expr::ClassLevel(s))
                        }
                    }
                    other => Err(FormulaEvalError(format!(
                        "classlevel(...) expects a string literal class name or no argument, got \
                         {other:?}"
                    ))),
                }
            }
            "if" => {
                // wave 26 shape closure: the condition may now be a chain of comparisons joined
                // by `&&` (`if((X==0&&Y>=3),1,0)`), not only a single bare comparison
                // (`if(X>=8,1,0)`) as before -- both are parsed by the same
                // `parse_arith_or_bool` used for arithmetic primaries. What is still refused,
                // unchanged from the module's pre-existing documented restriction (point 4): a
                // condition that reduces to a plain numeric `Expr` rather than an `Expr::Cmp`/
                // `Expr::And` -- real PCGen's bare-numeric-condition `if()` form remains
                // unimplemented and logged to `OPEN-ISSUES.md`, not silently accepted here.
                let cond = self.parse_arith_or_bool()?;
                if !matches!(cond, Expr::Cmp(..) | Expr::And(..)) {
                    return Err(FormulaEvalError(
                        "if(...)'s condition must be a comparison or `&&`-chain of comparisons \
                         — a bare numeric condition (real PCGen's if(SomeVar,1,0) form) is not yet \
                         implemented; refusing rather than guessing its truthiness"
                            .to_string(),
                    ));
                }
                self.expect(&Tok::Comma)?;
                let a = self.parse_expr()?;
                self.expect(&Tok::Comma)?;
                let b = self.parse_expr()?;
                self.expect(&Tok::RParen)?;
                Ok(Expr::If(Box::new(cond), Box::new(a), Box::new(b)))
            }
            "skillinfo" => {
                // wave 26 shape closure — see `Expr::SkillInfoTotalRank`'s own doc for scope
                // (`"TOTALRANK"` only, case-insensitive on the first argument per the real
                // `plugin/jepcommands/SkillInfoCommand.java`'s `equalsIgnoreCase` checks).
                let kind = match self.bump() {
                    Some(Tok::Str(s)) => s,
                    other => {
                        return Err(FormulaEvalError(format!(
                            "skillinfo(...) expects a string literal first argument, got {other:?}"
                        )))
                    }
                };
                self.expect(&Tok::Comma)?;
                let skill = match self.bump() {
                    Some(Tok::Str(s)) => s,
                    other => {
                        return Err(FormulaEvalError(format!(
                            "skillinfo(...) expects a string literal second argument, got {other:?}"
                        )))
                    }
                };
                self.expect(&Tok::RParen)?;
                if !kind.eq_ignore_ascii_case("totalrank") {
                    return Err(FormulaEvalError(format!(
                        "skillinfo({kind:?}, ...) — only the \"TOTALRANK\" first argument is \
                         implemented; plugin/jepcommands/SkillInfoCommand.java also defines \
                         \"modifier\", \"rank\", \"total\", \"stat\", \"misc\" -- \"rank\" and \
                         \"total\" ARE corpus-exercised (wave 26 integration cycle correction: \
                         4 and 1 real corpus occurrences respectively) but unimplemented so far, \
                         so refused rather than guessed at; \"modifier\"/\"stat\"/\"misc\" are not \
                         corpus-exercised at all"
                    )));
                }
                Ok(Expr::SkillInfoTotalRank(skill))
            }
            "min" | "max" | "floor" | "ceil" | "abs" => {
                // SD-32 T12 Epic 8 row 18 cycle 17: each comma-separated argument is now parsed
                // via `parse_arith_or_bool`, not the plain-arithmetic `parse_expr`. Real PCGen's
                // grammar is `org.nfunk.jep` (`pcgen/util/PJEP.java extends org.nfunk.jep.JEP`),
                // a standard operator-precedence expression parser: relational operators
                // (`org.nfunk.jep.function.Comparative`, confirmed pushing a plain `1.0`/`0.0`
                // Double, same citation `Expr::Cmp`'s eval already cites) sit at their own
                // precedence level and are valid anywhere an `expr` nonterminal appears --
                // including a function call's comma-separated arguments -- not gated behind a
                // parenthesised sub-expression the way this module's grammar previously required.
                // Confirmed by the real corpus shape this module previously refused:
                // `Protection Blessing ~ Increased Defense`'s
                // `1+min(WarpriestLVL>20,2,WarpriestLVL/10)` -- `WarpriestLVL>20` as a BARE,
                // unparenthesised `min()` argument. `parse_arith_or_bool` is a strict superset of
                // `parse_expr` (identical behaviour whenever no comparison/`&&` operator follows),
                // so this widening cannot change how any previously-accepted argument parses --
                // it only accepts a new shape `parse_expr` alone refused. Applies uniformly to
                // `floor`/`ceil`/`abs` too since they share this one parse branch; no oracle
                // citation restricts comparisons to only `min`/`max` positions specifically, and
                // narrowing to just two of the five functions here would be an arbitrary,
                // unverified restriction of its own.
                let mut args = vec![self.parse_arith_or_bool()?];
                while let Some(Tok::Comma) = self.peek() {
                    self.bump();
                    args.push(self.parse_arith_or_bool()?);
                }
                self.expect(&Tok::RParen)?;
                match lname.as_str() {
                    "floor" | "ceil" | "abs" if args.len() != 1 => {
                        return Err(FormulaEvalError(format!(
                            "{lname}(...) takes exactly 1 argument, got {}",
                            args.len()
                        )))
                    }
                    // `min`/`max` accept 1 or more arguments (see this module's own doc point 3
                    // — real `MaxCommand.java`/`MinCommand.java` are variable-arity and a
                    // single-argument call simply returns that argument unchanged). No arity
                    // check needed here: `args` always holds at least one element by
                    // construction (the `let mut args = vec![self.parse_arith_or_bool()?]` above), so
                    // there is no reachable "too few arguments" shape for `min`/`max` to refuse.
                    _ => {}
                }
                Ok(Expr::Call(lname, args))
            }
            other => Err(FormulaEvalError(format!(
                "unrecognised function {other:?} — refusing rather than guessing its semantics"
            ))),
        }
    }

    /// Parses a plain arithmetic expression and, if a comparison operator follows, upgrades it
    /// to a boolean-valued [`Expr::Cmp`] node, then continues folding any further `&&`-joined
    /// comparison terms into [`Expr::And`] nodes. This is the ONE grammar rule used everywhere a
    /// boolean-as-numeric value can appear: as `if()`'s condition, inside a parenthesised
    /// arithmetic primary (`1+(X>=15)`), as an entire bare top-level formula (`RangerLVL>=6`),
    /// and (SD-32 T12 Epic 8 row 18 cycle 17) as a bare, unparenthesised `min`/`max`/`floor`/
    /// `ceil`/`abs` function argument (`min(WarpriestLVL>20,2,WarpriestLVL/10)`) — see each call
    /// site's own comment for why unifying them is safe rather than an unverified precedence
    /// guess. Real PCGen's grammar (`org.nfunk.jep`, a standard operator-precedence expression
    /// parser `pcgen/util/PJEP.java` extends) treats relational operators as valid at every
    /// `expr` position, including a function call's comma-separated arguments, not only the
    /// three positions this module previously restricted them to — this module does not need to
    /// — and does not — invent a general operator precedence between comparisons and
    /// `+`/`-`/`*`//` beyond what `parse_arith_or_bool` already expresses (a comparison/`&&`
    /// chain sits ABOVE arithmetic, never nested inside one); a comparison still cannot appear
    /// nested inside a larger arithmetic term (`1+(X>=5)*2` is still refused, no corpus record
    /// exercises that shape).
    fn parse_arith_or_bool(&mut self) -> Result<Expr, FormulaEvalError> {
        let mut lhs = self.parse_expr()?;
        if let Some(op) = Self::peek_cmp_op(self.peek()) {
            self.bump();
            let rhs = self.parse_expr()?;
            lhs = Expr::Cmp(Box::new(lhs), op, Box::new(rhs));
        }
        while let Some(Tok::AndAnd) = self.peek() {
            self.bump();
            let a2 = self.parse_expr()?;
            let op2 = match self.bump() {
                Some(Tok::Ge) => CmpOp::Ge,
                Some(Tok::Le) => CmpOp::Le,
                Some(Tok::Eq) => CmpOp::Eq,
                Some(Tok::Ne) => CmpOp::Ne,
                Some(Tok::Gt) => CmpOp::Gt,
                Some(Tok::Lt) => CmpOp::Lt,
                other => {
                    return Err(FormulaEvalError(format!(
                        "expected a comparison operator after '&&' (every corpus `&&` use joins \
                         two comparisons, never a bare value), got {other:?}"
                    )))
                }
            };
            let b2 = self.parse_expr()?;
            lhs = Expr::And(Box::new(lhs), Box::new(Expr::Cmp(Box::new(a2), op2, Box::new(b2))));
        }
        Ok(lhs)
    }

    fn peek_cmp_op(tok: Option<&Tok>) -> Option<CmpOp> {
        match tok {
            Some(Tok::Ge) => Some(CmpOp::Ge),
            Some(Tok::Le) => Some(CmpOp::Le),
            Some(Tok::Eq) => Some(CmpOp::Eq),
            Some(Tok::Ne) => Some(CmpOp::Ne),
            Some(Tok::Gt) => Some(CmpOp::Gt),
            Some(Tok::Lt) => Some(CmpOp::Lt),
            _ => None,
        }
    }
}

/// Parses `formula` into an AST without evaluating it. Public so shape-coverage scanning (does
/// this module recognise the grammar at all) can be answered without needing to also supply
/// variable bindings. Returns `Err` for anything this module does not recognise — never a partial
/// or best-guess AST.
fn parse(formula: &str) -> Result<Expr, FormulaEvalError> {
    let tokens = tokenize(formula)?;
    let mut p = Parser { tokens: &tokens, pos: 0 };
    // `parse_arith_or_bool`, not the plain arithmetic `parse_expr`, so a bare top-level boolean
    // formula (`RangerLVL>=6`, no wrapping parens — a real, corpus-confirmed shape; see
    // `tests::bare_top_level_comparison_is_a_valid_formula`) is a recognised shape rather than
    // "trailing tokens after a complete expression."
    let expr = p.parse_arith_or_bool()?;
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
        // SD-32 T12 Epic 8 row 18 cycle 6: cross-class widening. Real PCGen's `classlevel("X")`
        // (`plugin/jepcommands/ClassLevelCommand.java`) looks up level in the SPECIFIC named
        // class -- this evaluator now honours that by keying the lookup on the class name
        // itself, `CLASSLEVEL::<X>` (a caller-supplied per-class binding, never `__LEVEL__`
        // any more). A caller that only knows ONE class's level (every consumer in this
        // codebase today -- `resolve_pcgen_var_chain` seeds exactly the record's own granting
        // class) binds only that one key; `classlevel("SameClass")` then resolves correctly
        // and safely, while a GENUINELY different class name stays unbound and refuses --
        // never silently answers with the wrong class's level, unlike the prior `__LEVEL__`
        // shortcut this replaces (see this module's own doc, "not covered" section, prior
        // text: "classlevel(...) does NOT verify its class-name argument against anything").
        Expr::ClassLevel(class_name) => {
            let key = format!("CLASSLEVEL::{class_name}");
            vars.get(&key).map(|v| *v as f64).ok_or_else(|| {
                FormulaEvalError(format!(
                    "classlevel({class_name:?}) needs a {key:?} binding -- no caller-supplied \
                     level is known for this class (refusing rather than guessing another \
                     class's level)"
                ))
            })
        }
        Expr::If(cond, a, b) => {
            if eval_expr(cond, vars)? != 0.0 {
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
        // wave 26 shape closure — see `Expr::Cmp`'s own doc for the oracle citation
        // (`org.nfunk.jep.function.Comparative.run()` pushes a plain `Double` `1.0`/`0.0`).
        Expr::Cmp(a, op, b) => {
            let av = eval_expr(a, vars)?;
            let bv = eval_expr(b, vars)?;
            let truth = match op {
                CmpOp::Ge => av >= bv,
                CmpOp::Le => av <= bv,
                CmpOp::Eq => av == bv,
                CmpOp::Ne => av != bv,
                CmpOp::Gt => av > bv,
                CmpOp::Lt => av < bv,
            };
            Ok(if truth { 1.0 } else { 0.0 })
        }
        // wave 26 shape closure — see `Expr::And`'s own doc for the oracle citation
        // (`org.nfunk.jep.function.Logical.run()`, id 0 = `OP_AND`): both sides always evaluated
        // (no short-circuit — the real engine is a postfix/stack evaluator that has already
        // popped both operands before the AND/OR switch runs), ANDed as "nonzero is true".
        Expr::And(a, b) => {
            let av = eval_expr(a, vars)?;
            let bv = eval_expr(b, vars)?;
            Ok(if av != 0.0 && bv != 0.0 { 1.0 } else { 0.0 })
        }
        // wave 26 shape closure — see `Expr::SkillInfoTotalRank`'s own doc for scope. No PC-skill
        // context is threaded through this evaluator's flat `vars` map (mirrors `classlevel`'s own
        // single `__LEVEL__` convention, module doc point on `classlevel`'s per-class gap) — a
        // consumer wiring this must bind the character's total ranks in the named skill under
        // this exact key before calling `evaluate`. Unbound refuses, exactly like every other
        // variable reference in this module — the real oracle's own silent "character doesn't
        // have the skill -> 0" default (`SkillInfoCommand.java`) is a PC-state fact a caller must
        // supply, not a default this module invents.
        Expr::SkillInfoTotalRank(skill) => {
            let key = format!("SKILLINFO_TOTALRANK::{skill}");
            vars.get(&key).map(|v| *v as f64).ok_or_else(|| {
                FormulaEvalError(format!(
                    "skillinfo(\"TOTALRANK\", {skill:?}) needs a {key:?} binding"
                ))
            })
        }
    }
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
        // SD-32 T12 Epic 8 row 18 cycle 6: `classlevel("X")` now keys its lookup on `X` itself
        // (`CLASSLEVEL::Summoner`), not a class-blind `__LEVEL__` slot.
        let e = PcgenFormulaEvaluator;
        let v = vars(&[("CLASSLEVEL::Summoner", 7)]);
        assert_eq!(e.evaluate("classlevel(\"Summoner\")", &v).unwrap(), 7);
        assert_eq!(e.evaluate("10+(X/2)+INT", &vars(&[("X", 7), ("INT", 3)])).unwrap(), 16);
    }

    #[test]
    fn classlevel_refuses_a_genuinely_different_class_it_has_no_binding_for() {
        // SD-32 T12 Epic 8 row 18 cycle 6: a caller that only knows ITS OWN class's level (every
        // consumer today) must never have `classlevel("SomeOtherClass")` silently answer with
        // that level -- the exact "plausible wrong number" shape the prior `__LEVEL__` shortcut
        // produced. Binding only `CLASSLEVEL::Sorcerer` and asking about `"Bloodrager"` refuses.
        let e = PcgenFormulaEvaluator;
        let v = vars(&[("CLASSLEVEL::Sorcerer", 7)]);
        assert!(e.evaluate("classlevel(\"Bloodrager\")", &v).is_err());
    }

    #[test]
    fn classlevel_with_no_argument_parses_and_reads_the_empty_key_binding() {
        // SD-32 T12 Epic 8 row 18 cycle 9: bare `classlevel()` (real shape:
        // `book_of_the_damned_volume_2/demoniac.json`'s BASEAB/SAVE formulas) now parses instead
        // of refusing outright, reusing the SAME `CLASSLEVEL::<name>` lookup with an empty-string
        // sentinel for "no class name given" — a caller must still explicitly bind `CLASSLEVEL::`
        // (empty key) for it to resolve.
        let e = PcgenFormulaEvaluator;
        let v = vars(&[("CLASSLEVEL::", 12)]);
        assert_eq!(e.evaluate("classlevel()*3/4", &v).unwrap(), 9);
        assert_eq!(e.evaluate("(classlevel()+1)/2", &v).unwrap(), 6);
    }

    #[test]
    fn classlevel_with_no_argument_refuses_without_a_binding() {
        // Widening the grammar to PARSE `classlevel()` must never widen what it silently
        // ANSWERS — no binding means refuse, exactly like every other unbound reference in this
        // module.
        let e = PcgenFormulaEvaluator;
        assert!(e.evaluate("classlevel()", &BTreeMap::new()).is_err());
        // And a bound NAMED class must never leak into the unnamed lookup.
        let v = vars(&[("CLASSLEVEL::Summoner", 7)]);
        assert!(e.evaluate("classlevel()", &v).is_err());
    }

    #[test]
    fn classlevel_two_argument_appliedas_nonepic_form_reads_the_same_binding() {
        // SD-32 T12 Epic 8 row 18 cycle 14: the real 2-argument corpus shape,
        // `classlevel("Monk","APPLIEDAS=NONEPIC")` (`core_rulebook/class_feature/monk/
        // standard_monk.json`, `pathfinder_unchained/class_feature/monk/unchained_monk.json`
        // BASEAB/SAVE formulas). Verified against `ClassLevelCommand.java`: the qualifier caps
        // the level read at the non-epic ceiling, a cap this engine's own per-class `max_level`
        // gate already makes unreachable — so it reads the SAME `CLASSLEVEL::Monk` binding as
        // the 1-argument form.
        let e = PcgenFormulaEvaluator;
        let v = vars(&[("CLASSLEVEL::Monk", 9)]);
        assert_eq!(e.evaluate("classlevel(\"Monk\",\"APPLIEDAS=NONEPIC\")", &v).unwrap(), 9);
        assert_eq!(
            e.evaluate("classlevel(\"Monk\",\"APPLIEDAS=NONEPIC\")*3/4", &v).unwrap(),
            6
        );
    }

    #[test]
    fn classlevel_two_argument_form_still_refuses_an_unbound_or_wrong_class() {
        // The widening must not leak into answering for a class it has no binding for, exactly
        // like the 1-argument form's own refusal.
        let e = PcgenFormulaEvaluator;
        let v = vars(&[("CLASSLEVEL::Monk", 9)]);
        assert!(e.evaluate("classlevel(\"Fighter\",\"APPLIEDAS=NONEPIC\")", &v).is_err());
        assert!(e.evaluate("classlevel(\"Monk\",\"APPLIEDAS=NONEPIC\")", &BTreeMap::new()).is_err());
    }

    #[test]
    fn classlevel_two_argument_form_refuses_an_unrecognised_appliedas_qualifier() {
        // The real oracle's `ClassLevelCommand.java` throws `ParseException("Did not understand
        // APPLIEDAS=" + applied)` for any value other than `NONEPIC` — this module refuses
        // identically rather than silently accepting an unverified qualifier.
        let e = PcgenFormulaEvaluator;
        let v = vars(&[("CLASSLEVEL::Monk", 9)]);
        assert!(e.evaluate("classlevel(\"Monk\",\"APPLIEDAS=EPIC\")", &v).is_err());
        assert!(e.evaluate("classlevel(\"Monk\",\"Fighter\")", &v).is_err());
    }

    // -- 1b. wave 26 shape closure: comparisons/`&&` as first-class numeric values, and
    //    skillinfo("TOTALRANK", ...) — each grounded in the module doc's own citations above.
    // -------------------------------------------------------------------------------------------

    #[test]
    fn bare_comparison_as_a_value_matches_the_kineticist_corpus_shape() {
        // The exact corpus formula (`data/corpus` Kineticist infusion tokens):
        // "1+(KineticistLVL>=15)". Below the threshold it's 1; at/above, 2 — proving this is a
        // real coercion, not merely "the parser accepts it and returns something."
        let e = PcgenFormulaEvaluator;
        assert_eq!(e.evaluate("1+(KineticistLVL>=15)", &vars(&[("KineticistLVL", 14)])).unwrap(), 1);
        assert_eq!(e.evaluate("1+(KineticistLVL>=15)", &vars(&[("KineticistLVL", 15)])).unwrap(), 2);
        assert_eq!(e.evaluate("1+(KineticistLVL>=15)", &vars(&[("KineticistLVL", 20)])).unwrap(), 2);
        // The longer chained corpus shape: 8 additive gates.
        let f = "1+(KineticistLVL>=3)+(KineticistLVL>=5)+(KineticistLVL>=9)+(KineticistLVL>=11)+\
                 (KineticistLVL>=13)+(KineticistLVL>=17)+(KineticistLVL>=19)";
        assert_eq!(e.evaluate(f, &vars(&[("KineticistLVL", 1)])).unwrap(), 1);
        assert_eq!(e.evaluate(f, &vars(&[("KineticistLVL", 10)])).unwrap(), 4); // base+3+5+9
        assert_eq!(e.evaluate(f, &vars(&[("KineticistLVL", 19)])).unwrap(), 8); // all 7 gates pass
    }

    #[test]
    fn bare_top_level_comparison_is_a_valid_formula() {
        // Real corpus shape: an entire `DEFINE`/`BONUS` formula that IS just a comparison, e.g.
        // "RangerLVL>=6" — no wrapping parens, no surrounding arithmetic. Confirmed in
        // `advanced_players_guide`/`ultimate_combat` DEFINE tokens.
        let e = PcgenFormulaEvaluator;
        assert_eq!(e.evaluate("RangerLVL>=6", &vars(&[("RangerLVL", 5)])).unwrap(), 0);
        assert_eq!(e.evaluate("RangerLVL>=6", &vars(&[("RangerLVL", 6)])).unwrap(), 1);
    }

    #[test]
    fn and_combines_two_comparisons_matching_the_sorcerer_bloodline_gate_shape() {
        // The exact corpus formula shape (Sorcerer/Bloodrager bloodline-power gates):
        // "if((PowerAlreadyTaken==0&&ProgressionLVL>=N),1,0)" — grants the power exactly once,
        // at or after the level gate, never before, never a second time.
        let e = PcgenFormulaEvaluator;
        let f = "if((Sorcerer_CF_BloodlinePower3==0&&Sorcerer_Psychic_BloodlineProgressionLVL>=3),1,0)";
        assert_eq!(
            e.evaluate(f, &vars(&[("Sorcerer_CF_BloodlinePower3", 0), ("Sorcerer_Psychic_BloodlineProgressionLVL", 2)]))
                .unwrap(),
            0,
            "below the level gate: not yet granted"
        );
        assert_eq!(
            e.evaluate(f, &vars(&[("Sorcerer_CF_BloodlinePower3", 0), ("Sorcerer_Psychic_BloodlineProgressionLVL", 3)]))
                .unwrap(),
            1,
            "at the level gate and not already taken: granted"
        );
        assert_eq!(
            e.evaluate(f, &vars(&[("Sorcerer_CF_BloodlinePower3", 1), ("Sorcerer_Psychic_BloodlineProgressionLVL", 5)]))
                .unwrap(),
            0,
            "already taken (tracked by the other side of the &&): not granted again"
        );
    }

    #[test]
    fn and_both_sides_always_evaluate_no_short_circuit() {
        // `org.nfunk.jep.function.Logical.run()` pops both stack operands unconditionally before
        // its AND/OR switch ever runs (module doc, `Expr::And`) — there is no short-circuit to
        // reproduce. Proven here the only way that's observable: the RIGHT side references an
        // unbound variable even when the LEFT side is already false, and this must still refuse
        // (a short-circuiting implementation would instead return `0` without ever touching the
        // unbound variable).
        let e = PcgenFormulaEvaluator;
        let err = e
            .evaluate("if((X==1&&NeverBound>=1),1,0)", &vars(&[("X", 0)]))
            .unwrap_err();
        assert!(err.0.contains("unbound variable"), "got: {}", err.0);
    }

    #[test]
    fn if_condition_still_refuses_a_bare_numeric_value() {
        // Unchanged restriction (module doc point 4) — wave 26 widened WHAT KIND of comparison
        // chain a condition may be, not whether a non-comparison numeric value is accepted.
        let e = PcgenFormulaEvaluator;
        let err = e.evaluate("if(X,1,0)", &vars(&[("X", 1)])).unwrap_err();
        assert!(err.0.contains("condition must be a comparison"), "got: {}", err.0);
    }

    #[test]
    fn skillinfo_totalrank_reads_the_bound_skill_rank() {
        let e = PcgenFormulaEvaluator;
        let v = vars(&[("SKILLINFO_TOTALRANK::Knowledge (Religion)", 12)]);
        assert_eq!(e.evaluate("if(skillinfo(\"TOTALRANK\",\"Knowledge (Religion)\")>=10,4,2)", &v).unwrap(), 4);
        let v2 = vars(&[("SKILLINFO_TOTALRANK::Knowledge (Religion)", 3)]);
        assert_eq!(e.evaluate("if(skillinfo(\"TOTALRANK\",\"Knowledge (Religion)\")>=10,4,2)", &v2).unwrap(), 2);
    }

    #[test]
    fn skillinfo_totalrank_case_insensitive_first_argument_matching_the_oracle() {
        // `SkillInfoCommand.java`: `"totalrank".equalsIgnoreCase(param1)`.
        let e = PcgenFormulaEvaluator;
        let v = vars(&[("SKILLINFO_TOTALRANK::Swim", 5)]);
        assert_eq!(e.evaluate("skillinfo(\"totalrank\",\"Swim\")", &v).unwrap(), 5);
        assert_eq!(e.evaluate("skillinfo(\"TotalRank\",\"Swim\")", &v).unwrap(), 5);
    }

    #[test]
    fn skillinfo_unbound_skill_refuses_not_defaults_to_zero() {
        // The real oracle defaults to 0.0 when the character lacks the skill entirely
        // (`SkillInfoCommand.java`'s `hasSkill` check) — but that is a PC-state fact a consumer
        // must supply, not a default this evaluator invents (see `Expr::SkillInfoTotalRank`'s own
        // doc). No consumer is wired yet, so there is no such fact available here; refuse.
        let e = PcgenFormulaEvaluator;
        let err = e.evaluate("skillinfo(\"TOTALRANK\",\"Swim\")", &BTreeMap::new()).unwrap_err();
        assert!(err.0.contains("needs a") && err.0.contains("binding"), "got: {}", err.0);
    }

    #[test]
    fn skillinfo_other_first_arguments_refuse_not_implemented() {
        let e = PcgenFormulaEvaluator;
        for kind in ["modifier", "rank", "total", "stat", "misc"] {
            let err = e
                .evaluate(&format!("skillinfo(\"{kind}\",\"Swim\")"), &BTreeMap::new())
                .unwrap_err();
            assert!(err.0.contains("only the \"TOTALRANK\""), "kind {kind:?} got: {}", err.0);
        }
    }

    /// Decision 1(a) mutation proof, scoped to wave 26's own additions: a deliberately wrong `&&`
    /// (implemented as OR) MUST be caught by the concrete assertions above, not merely by "did not
    /// error." Exercised directly here rather than via the 22-case harness (none of those 22
    /// hand-modelled functions use `&&`, so the harness's own reproduction test cannot catch this
    /// class of mistake — this test is the gate for wave 26's new grammar specifically).
    #[test]
    fn mutated_and_as_or_is_caught_by_the_bloodline_gate_assertion() {
        let e = PcgenFormulaEvaluator;
        let f = "if((Sorcerer_CF_BloodlinePower3==0&&Sorcerer_Psychic_BloodlineProgressionLVL>=3),1,0)";
        // Already-taken (left side false) AND below-level (right side false): real AND -> 0.
        // A mutant OR would also give 0 here, so this input alone can't distinguish them —
        // the discriminating case is left-false/right-true, asserted next.
        let already_taken_below_level =
            vars(&[("Sorcerer_CF_BloodlinePower3", 1), ("Sorcerer_Psychic_BloodlineProgressionLVL", 1)]);
        assert_eq!(e.evaluate(f, &already_taken_below_level).unwrap(), 0);
        // Already-taken (left side false) but AT/above level (right side true): real AND -> 0
        // (correctly withholds a second grant); a mutant OR would wrongly return 1 here. This is
        // the exact case `and_combines_two_comparisons_matching_the_sorcerer_bloodline_gate_shape`
        // above already asserts against — restated here as the explicit mutation-proof case.
        let already_taken_above_level =
            vars(&[("Sorcerer_CF_BloodlinePower3", 1), ("Sorcerer_Psychic_BloodlineProgressionLVL", 5)]);
        assert_eq!(
            e.evaluate(f, &already_taken_above_level).unwrap(),
            0,
            "a mutant implementing && as || would wrongly return 1 here — proves the test discriminates"
        );
    }

    /// Adversarial-review finding (wave 26 integration cycle): `CmpOp::Gt`, `CmpOp::Lt` and
    /// `CmpOp::Le` had no boundary-sensitive test — mutating any of the three (`>` to `>=`, `<`
    /// to `<=`, `<=` to `<`) left the whole `pilot_compute::` suite green. All three are real,
    /// corpus-live shapes this commit newly accepts and evaluates (72/24/6 distinct corpus
    /// formula candidates use `>`/`</`<=` respectively, e.g. Brawler's
    /// `(BrawlerLVL>2)+(BrawlerLVL>6)+...`, Warpriest's `if(...LVL<5,4,if(...` , Monk's
    /// `if(MONKLVL<=3,-(KiPoolLVL/2),0)`) — exactly section 24.1's own feared failure mode
    /// (a misinterpreted token producing a plausible number nobody checks) one level down from
    /// the corpus-formula level. Each assertion below straddles its own operator's true boundary
    /// on both sides, so a one-off mutation of that operator is guaranteed to flip a result.
    #[test]
    fn cmp_gt_is_strictly_greater_not_greater_or_equal() {
        let e = PcgenFormulaEvaluator;
        // At the boundary (10>10): real `>` is false. A `>=` mutant would wrongly say true.
        assert_eq!(e.evaluate("if(X>10,1,0)", &vars(&[("X", 10)])).unwrap(), 0);
        // Just above the boundary (11>10): real `>` is true.
        assert_eq!(e.evaluate("if(X>10,1,0)", &vars(&[("X", 11)])).unwrap(), 1);
    }

    #[test]
    fn cmp_lt_is_strictly_less_not_less_or_equal() {
        let e = PcgenFormulaEvaluator;
        // At the boundary (5<5): real `<` is false. A `<=` mutant would wrongly say true.
        assert_eq!(e.evaluate("if(X<5,1,0)", &vars(&[("X", 5)])).unwrap(), 0);
        // Just below the boundary (4<5): real `<` is true.
        assert_eq!(e.evaluate("if(X<5,1,0)", &vars(&[("X", 4)])).unwrap(), 1);
    }

    #[test]
    fn cmp_le_is_less_or_equal_not_strictly_less() {
        let e = PcgenFormulaEvaluator;
        // At the boundary (3<=3): real `<=` is true. A `<` mutant would wrongly say false —
        // this is the live Monk shape (`MONKLVL<=3`) at exactly its own gate level.
        assert_eq!(e.evaluate("if(X<=3,1,0)", &vars(&[("X", 3)])).unwrap(), 1);
        // Just above the boundary (4<=3): real `<=` is false.
        assert_eq!(e.evaluate("if(X<=3,1,0)", &vars(&[("X", 4)])).unwrap(), 0);
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
        assert!(recognises_shape("if(X>=1,1)").is_err());
        // `max(1)` (and `min(1)`) do NOT belong here any more (SD-32 T12 Epic 8 row 18 cycle 16,
        // `scripts/retro.py correction` -- this test previously asserted `max(1)` refuses, a
        // pinned wrong assumption: real PCGen's `MaxCommand.java` is variable-arity and a
        // single-argument call is valid, returning that argument unchanged. See
        // `single_argument_min_max_now_matches_the_oracles_variable_arity_max_min_command` below
        // for the corrected, proven property.
    }

    /// SD-32 T12 Epic 8 row 18 cycle 16: corrects `wrong_arg_counts_refuse`'s own prior wrong
    /// assumption that `max(1)` refuses. Verified directly against the pinned oracle,
    /// `plugin/jepcommands/MaxCommand.java` (`numberOfParameters = -1`, `run()`'s `first ||
    /// param > result` fold returns the sole parameter unchanged when there is only one) and its
    /// mirror `MinCommand.java` -- both genuinely variable-arity, accepting 1 argument. Also
    /// proves the real corpus shape this closes: `Cavalier Order of the Beast ~ Class Skills`'s
    /// own `max(floor(CavalierLVL/2))`.
    #[test]
    fn single_argument_min_max_now_matches_the_oracles_variable_arity_max_min_command() {
        let e = PcgenFormulaEvaluator;
        assert_eq!(e.evaluate("max(7)", &BTreeMap::new()).unwrap(), 7);
        assert_eq!(e.evaluate("min(7)", &BTreeMap::new()).unwrap(), 7);
        assert_eq!(
            e.evaluate("max(floor(CavalierLVL/2))", &vars(&[("CavalierLVL", 9)])).unwrap(),
            4
        );
    }

    /// SD-32 T12 Epic 8 row 18 cycle 17: an unparenthesised comparison as a bare `min`/`max`
    /// function argument, the real corpus shape `Protection Blessing ~ Increased Defense`'s
    /// `1+min(WarpriestLVL>20,2,WarpriestLVL/10)` previously refused (cycle 16 named it,
    /// verified against `org.nfunk.jep`'s standard operator-precedence grammar — relational
    /// operators are valid at any `expr` position, not gated behind parens — and sized the
    /// blast radius: 1 corpus record). `WarpriestLVL>20` evaluates to `Expr::Cmp` -> 0.0/1.0
    /// (`org.nfunk.jep.function.Comparative.run()`, same citation `Expr::Cmp`'s own eval arm
    /// cites), so at level 20 the comparison is false (0) and `min(0,2,2)=0`, at level 21 true
    /// (1) and `min(1,2,2.1)=1`.
    #[test]
    fn bare_comparison_as_a_min_max_function_argument_matches_the_warpriest_corpus_shape() {
        let e = PcgenFormulaEvaluator;
        assert_eq!(
            e.evaluate(
                "1+min(WarpriestLVL>20,2,WarpriestLVL/10)",
                &vars(&[("WarpriestLVL", 20)])
            )
            .unwrap(),
            1
        );
        assert_eq!(
            e.evaluate(
                "1+min(WarpriestLVL>20,2,WarpriestLVL/10)",
                &vars(&[("WarpriestLVL", 21)])
            )
            .unwrap(),
            2
        );
        // Same shape for max(), and for a bare (non-min/max-wrapped) comparison mixed with a
        // plain arithmetic argument, proving this is the general `parse_arith_or_bool` widening
        // and not a min()-specific special case.
        assert_eq!(e.evaluate("max(WarpriestLVL>20,0)", &vars(&[("WarpriestLVL", 25)])).unwrap(), 1);
        assert_eq!(e.evaluate("max(WarpriestLVL>20,0)", &vars(&[("WarpriestLVL", 5)])).unwrap(), 0);
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
