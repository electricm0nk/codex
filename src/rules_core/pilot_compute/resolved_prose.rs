//! A converted rule's own words, with **this character's numbers** in them.
//!
//! # Why this module exists
//!
//! SD-35 `decisions.md` §11 — "there should be nothing left of pcgen" on the live side — and
//! `decisions.md` §1, the sheet rule: what reaches a player is a final number or the rule's own
//! words, never an ingest-format expression.
//!
//! Until this module, `pilot_compute` resolved a Pathfinder Unchained class feature's description
//! by holding the record's `DESC:` **tokens verbatim** in a `const` and handing them to the PCGen
//! renderer at run time. That is an ingest-format reader in the code that prints a character
//! sheet, and `AT-35-E6-003`'s own words name it: *"`render_pcgen_desc` is deleted from the live
//! side; its `%N` substitution already happened in the converter."*
//!
//! It did. `data/sheet_rules/pathfinder_unchained/class_feature/unchained_barbarian_rage.json`
//! already carries that record's prose as
//! [`ProseSegment`](crate::rules_core::sheet_rule::ProseSegment)s of plain English
//! [`Text`](crate::rules_core::sheet_rule::ProsePiece::Text) pieces and typed
//! [`Slot`](crate::rules_core::sheet_rule::ProsePiece::Slot) holes over our own
//! [`Expr`](crate::rules_core::sheet_rule::Expr), with each segment's `PREVAR*` gate already
//! converted to an [`Applies`](crate::rules_core::sheet_rule::Applies). Nothing had to be built
//! to make that true — it only had to be **read** instead of the tokens.
//!
//! # What it does, and the three rules it keeps
//!
//! 1. **No fabrication.** A slot whose variable this engine has not computed makes the WHOLE
//!    description `None`. A description that has lost a number reads as a defect (*"You can rage
//!    for rounds per day"*), and no line at all is honest where the mangled sentence is not.
//!    This is the same contract `render_pcgen_desc_with_values`' `dropped_args` enforced, stated
//!    as a return value instead of a report.
//! 2. **A gate is decided, or the prose survives.** A segment is dropped only on a gate this
//!    module can decide and that comes out FALSE. A gate naming a fact we do not hold — every
//!    [`Applies::Holds`](crate::rules_core::sheet_rule::Applies::Holds), which is what the
//!    converter writes for the `PREABILITY` family — keeps its prose, exactly as
//!    `eval_desc_gate`'s `Undecided` did. Deleting real rulebook text on the strength of an
//!    unknown is the worse error.
//! 3. **One truncation, at the slot.** `Expr` evaluation is exact (rational, `i128`), and the
//!    single truncation toward zero happens where the number is printed — the boundary
//!    [`SheetValue::Number`](crate::rules_core::sheet_rule::SheetValue::Number) documents, and
//!    PCGen's own integer-division semantics, so `RogueLVL/5` at level 7 prints `1`.
//!
//! # The proof this is the same text
//!
//! `tests/sd27_pu_class_feature_descriptions_carry_the_characters_numbers.rs` renders every
//! Pathfinder Unchained record that carries a `%N` **both ways** — the converted prose through
//! this module, and the record's own `DESC:` tokens read off `data/corpus/` through the tool-side
//! PCGen renderer — over a matrix of levels and ability modifiers, and asserts the two are
//! byte-identical. The oracle stayed; only the live reader changed.

use std::collections::BTreeMap;

use crate::rules_core::sheet_rule::{
    var_id, Applies, Cmp, Expr, ProseFamily, ProsePiece, ProseSegment, VarId,
};

/// The numbers this engine has resolved for one character, keyed by the converted
/// [`VarId`] that stands for the source variable.
///
/// Keyed by id rather than by name because that is what the converted package addresses a
/// variable by; [`set`](DisplayValues::set) mints the id from the name with the schema's own
/// [`var_id`], which is a pure hash and knows nothing about PCGen.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DisplayValues {
    values: BTreeMap<VarId, i64>,
}

impl DisplayValues {
    /// An empty table. Rendering against it resolves no slot, so every description that carries
    /// one comes back `None`.
    pub fn new() -> Self {
        Self::default()
    }

    /// States one resolved value under the source variable name that stands for it. Overwrites
    /// rather than accumulates: the caller's hand-modelled function already produced the
    /// *total*, and adding to it here would double-count.
    pub fn set(&mut self, name: &str, value: i64) {
        self.values.insert(var_id(name), value);
    }

    /// The resolved value for one converted variable id, or `None` when this engine has not
    /// computed it. `None` is what keeps a description unrendered rather than guessed.
    pub fn get(&self, id: &VarId) -> Option<i64> {
        self.values.get(id).copied()
    }

    /// True when nothing has been resolved.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

/// An exact rational, so a chain of divisions truncates once — at the slot — rather than at every
/// step. `i128` because the corpus's own expressions are small and this can then never overflow
/// in practice while still being checked rather than assumed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Exact {
    num: i128,
    den: i128,
}

impl Exact {
    fn int(n: i64) -> Self {
        Self { num: i128::from(n), den: 1 }
    }

    fn add(self, other: Self) -> Option<Self> {
        Some(Self {
            num: self.num.checked_mul(other.den)?.checked_add(other.num.checked_mul(self.den)?)?,
            den: self.den.checked_mul(other.den)?,
        })
    }

    fn mul(self, other: Self) -> Option<Self> {
        Some(Self { num: self.num.checked_mul(other.num)?, den: self.den.checked_mul(other.den)? })
    }

    fn div(self, other: Self) -> Option<Self> {
        if other.num == 0 {
            return None;
        }
        Some(Self { num: self.num.checked_mul(other.den)?, den: self.den.checked_mul(other.num)? })
    }

    /// Toward zero — PCGen's own integer-division semantics, and the one truncation
    /// [`SheetValue::Number`](crate::rules_core::sheet_rule::SheetValue::Number) permits.
    fn trunc(self) -> Option<i64> {
        if self.den == 0 {
            return None;
        }
        i64::try_from(self.num / self.den).ok()
    }

    fn cmp_to(self, other: Self) -> Option<std::cmp::Ordering> {
        // Cross-multiply, normalising the sign of both denominators first so the comparison is
        // not silently inverted by a negative one.
        let (a, b) = (self.normalised()?, other.normalised()?);
        Some(a.num.checked_mul(b.den)?.cmp(&b.num.checked_mul(a.den)?))
    }

    fn normalised(self) -> Option<Self> {
        if self.den == 0 {
            return None;
        }
        if self.den < 0 {
            Some(Self { num: self.num.checked_neg()?, den: self.den.checked_neg()? })
        } else {
            Some(self)
        }
    }
}

/// Evaluates the [`Expr`] subset a converted class-feature description can carry, exactly.
///
/// `None` for a variable this engine has not resolved **and** for any variant outside the
/// subset — a fact about the character that is not in [`DisplayValues`] is not a fact this
/// function may invent. Every unsupported variant therefore reads as "not resolved", which the
/// two callers turn into "no description" and "gate undecided" respectively; neither ever
/// fabricates.
fn eval(expr: &Expr, values: &DisplayValues) -> Option<Exact> {
    match expr {
        Expr::Const(n) => Some(Exact::int(i64::from(*n))),
        Expr::Var(id) => values.get(id).map(Exact::int),
        Expr::Sum(terms) => {
            let mut total = Exact::int(0);
            for term in terms {
                total = total.add(eval(term, values)?)?;
            }
            Some(total)
        }
        Expr::Mul(a, b) => eval(a, values)?.mul(eval(b, values)?),
        Expr::Div(a, b) => eval(a, values)?.div(eval(b, values)?),
        Expr::Min(a, b) => {
            let (a, b) = (eval(a, values)?, eval(b, values)?);
            Some(if a.cmp_to(b)? == std::cmp::Ordering::Greater { b } else { a })
        }
        Expr::Max(a, b) => {
            let (a, b) = (eval(a, values)?, eval(b, values)?);
            Some(if a.cmp_to(b)? == std::cmp::Ordering::Less { b } else { a })
        }
        _ => None,
    }
}

/// Whether one converted segment gate holds for this character.
///
/// `None` is `Undecided` and is a first-class outcome, not a failure: it keeps the segment's
/// prose. The decided family is [`Applies::Compare`] over the expression subset above, plus the
/// structural combinators; everything else — `Holds`, `Chosen`, `ItemHas`, `Situational`, which
/// is what the converter writes for `PREABILITY` and its siblings — is undecided here and its
/// prose survives.
fn gate_holds(applies: &Applies, values: &DisplayValues) -> Option<bool> {
    match applies {
        Applies::Always => Some(true),
        Applies::Never => Some(false),
        Applies::Not(inner) => gate_holds(inner, values).map(|held| !held),
        Applies::All(terms) => {
            let mut all = true;
            for term in terms {
                all &= gate_holds(term, values)?;
            }
            Some(all)
        }
        Applies::AtLeast { n, of } => {
            let mut held = 0u8;
            for term in of {
                if gate_holds(term, values)? {
                    held += 1;
                }
            }
            Some(held >= *n)
        }
        Applies::Compare { lhs, op, rhs } => {
            let ordering = eval(lhs, values)?.cmp_to(eval(rhs, values)?)?;
            use std::cmp::Ordering::{Equal, Greater, Less};
            Some(match op {
                Cmp::Eq => ordering == Equal,
                Cmp::Ne => ordering != Equal,
                Cmp::Lt => ordering == Less,
                Cmp::Lte => ordering != Greater,
                Cmp::Gt => ordering == Greater,
                Cmp::Gte => ordering != Less,
            })
        }
        _ => None,
    }
}

/// One converted `Desc` segment's words, or `None` when a slot it carries is unresolved.
fn render_segment(segment: &ProseSegment, values: &DisplayValues) -> Option<String> {
    let mut out = String::new();
    for piece in &segment.pieces {
        match piece {
            ProsePiece::Text(text) => out.push_str(text),
            ProsePiece::Slot(expr) => out.push_str(&eval(expr, values)?.trunc()?.to_string()),
            ProsePiece::Dice { dice, modifier } => {
                out.push_str(dice);
                if let Some(modifier) = modifier {
                    let value = eval(modifier, values)?.trunc()?;
                    if value >= 0 {
                        out.push('+');
                    }
                    out.push_str(&value.to_string());
                }
            }
            // A term the player settles at pick time. This path has no picks in hand, so it is
            // unresolved rather than guessed — the same treatment a `%CHOICE` got.
            ProsePiece::ChoiceName(_) => return None,
        }
    }
    Some(out)
}

/// The converted rule `rule_id`'s `Desc` prose with `values`' numbers rendered into it, or `None`
/// when the package is absent, the rule is not in it, the rule carries no `Desc` prose, or any
/// surviving segment holds a slot this engine has not resolved.
///
/// Segments whose gate is decided FALSE are dropped; every other segment is kept and joined with
/// a single space, which is the order and the separator the record's own source rows carry.
/// `pick_last` keeps only the last surviving segment of its family, as the schema defines.
pub fn resolved_description(rule_id: &str, values: &DisplayValues) -> Option<String> {
    let package = crate::rules_core::corpus_loader::live_sheet_rules()?;
    let rule = package.rule(rule_id)?;

    // A rule whose words carry no character-dependent hole is not a *resolved* description: its
    // stored `description` already says everything the book says, and repeating it under the
    // "with this character's own numbers resolved into it" marker would claim a resolution that
    // did not happen. This is the population predicate the deleted `PU_RESOLVABLE_DESCRIPTIONS`
    // constant used to carry as a hand list, read off the converted package instead — the
    // records whose prose states a number.
    if !rule.prose.iter().any(|segment| {
        segment.family == ProseFamily::Desc
            && segment
                .pieces
                .iter()
                .any(|piece| matches!(piece, ProsePiece::Slot(_) | ProsePiece::Dice { .. }))
    }) {
        return None;
    }

    let mut segments: Vec<String> = Vec::new();
    let mut pick_last_index: Option<usize> = None;
    for segment in &rule.prose {
        if segment.family != ProseFamily::Desc {
            continue;
        }
        if let Some(applies) = &segment.applies
            && gate_holds(applies, values) == Some(false)
        {
            continue;
        }
        let text = render_segment(segment, values)?;
        if text.is_empty() {
            continue;
        }
        if segment.pick_last {
            match pick_last_index {
                Some(index) => segments[index] = text,
                None => {
                    pick_last_index = Some(segments.len());
                    segments.push(text);
                }
            }
            continue;
        }
        segments.push(text);
    }

    if segments.is_empty() {
        return None;
    }
    Some(segments.join(" "))
}
