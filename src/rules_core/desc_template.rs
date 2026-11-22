//! A record description **settled into a template at ingest**, and the live renderer that fills
//! it with this character's own numbers.
//!
//! SD-35 `AT-35-E6-003-RULED` cycle 16 (`decisions.md` §11, §19/ruling B16). Before this module
//! the live side handed a corpus record's stored source `description` string to the converter's
//! own prose renderer at request time — five `crate::pcgen_import::pcgen_desc::…` calls inside
//! `pilot_compute::class_feature_grant_consumer`, and the largest remaining group the residue
//! gate counted. That is an ingest-format reader in the middle of live code, which the operator
//! ruled out. (Three shipping converter calls remain elsewhere on the live side —
//! `derived_evaluator_fixture_check.rs`'s three corpus token sweeps, measured non-relocatable by
//! cycle 3 — so this is not the last one; cycle 15's receipt said it was and is corrected in the
//! retrospective log.)
//!
//! The split this module makes is the same one `data/sheet_rules/` and cycle 15's
//! `data/corpus/<book>/_settled/` already make: **everything that depends only on the source
//! text happens once, at authoring time; everything that depends on the character happens
//! live.** The converter ([`crate::pcgen_import::desc_template_convert`]) scans the stored
//! description once and emits an ordered [`DescOp`] list; this module walks that list with a
//! plain `name -> i64` environment. Nothing here parses a source token, names a source keyword,
//! or decides what a piece of source syntax means — every such decision is already frozen into
//! an op.
//!
//! **The contract is unchanged, and that is provable rather than asserted.** The renderer's
//! "drop and report, never guess" rule, its sign-swallowing on a dropped slot, its
//! whitespace collapse and its escape decoding are reproduced here term for term, and
//! `crate::pcgen_import::desc_template_convert`'s own tests render **every** described
//! `class_feature` record in `data/corpus/` both ways, under several value environments, and
//! compare `text` **and** `dropped_args` field for field.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// How one filled slot gets its number.
///
/// Settled at authoring time from the argument text the source row supplied. `Literal` is an
/// argument that was already a plain integer and needs no character at all; `Named` is a lookup
/// by the exact name the environment binds, with the one `<name><sign><integer>` offset shape
/// the corpus actually contains as its fallback.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DescArgument {
    /// The argument was a plain integer: this number, for every character.
    Literal { value: i64 },
    /// Look `name` up in the environment. When that misses and `offset` is present, look
    /// `offset.0` up instead and add `offset.1`.
    Named {
        name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        offset: Option<(String, i64)>,
    },
}

impl DescArgument {
    /// This argument's value for one character, or `None` when the environment does not bind it.
    /// `None` is what keeps a slot dropped rather than guessed.
    pub fn value(&self, env: &BTreeMap<String, i64>) -> Option<i64> {
        match self {
            DescArgument::Literal { value } => Some(*value),
            DescArgument::Named { name, offset } => {
                if let Some(v) = env.get(name) {
                    return Some(*v);
                }
                let (base_name, delta) = offset.as_ref()?;
                Some(env.get(base_name)? + delta)
            }
        }
    }
}

/// One step of a settled description.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum DescOp {
    /// Literal prose, appended verbatim.
    Text { text: String },
    /// A value slot. On success its number is appended; on failure it is dropped, `report` is
    /// recorded, and any `+`/`-` that introduced it is swallowed.
    Arg { arg: DescArgument, report: String },
    /// A slot the source prose referenced but for which the row supplied no argument at all.
    /// Always dropped, and — matching the behaviour this replaces exactly — never reported.
    MissingArg,
    /// A slot that can never resolve (a chargen-time player selection, or a stray marker).
    /// Always dropped and reported under `report`.
    Drop { report: String },
    /// A per-cent sign that is only real when the text already built ends in a digit or in
    /// word-boundary dice notation; otherwise it is an orphan and is dropped and reported.
    PercentOrDrop,
}

/// A rendered description and what it could not state.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RenderedDescription {
    /// The prose a player may be shown.
    pub text: String,
    /// The slots that could not be resolved to a number and were therefore dropped. Reported,
    /// never guessed.
    pub dropped_args: Vec<String>,
}

/// One record's description, settled.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DescTemplate {
    /// The ordered steps.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ops: Vec<DescOp>,
    /// The argument texts this description's slots are keyed by, in source order — the names a
    /// caller that resolves slots through its own mechanism must bind in the environment.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    /// Literal escape sequences to decode in the finished text, as `(from, to)` pairs, settled
    /// at authoring time from what the source prose actually carried. Applied last, after the
    /// whitespace collapse, so a decoded newline is not squashed back into a space.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub escapes: Vec<(String, String)>,
}

impl DescTemplate {
    /// True when this record has no description to render at all.
    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }

    /// The argument texts this description's slots are keyed by, in source order.
    pub fn args(&self) -> &[String] {
        &self.args
    }

    /// Renders this description with one character's resolved values.
    ///
    /// `env` maps a value name to that character's number. A slot whose name `env` does not bind
    /// is dropped and reported — never estimated, never left half-filled.
    pub fn render(&self, env: &BTreeMap<String, i64>) -> RenderedDescription {
        let mut out = String::new();
        let mut dropped_args: Vec<String> = Vec::new();
        let mut dropped_any = false;

        for op in &self.ops {
            match op {
                DescOp::Text { text } => out.push_str(text),
                DescOp::Arg { arg, report } => match arg.value(env) {
                    Some(value) => out.push_str(&value.to_string()),
                    None => {
                        dropped_args.push(report.clone());
                        swallow_introducing_sign(&mut out);
                        dropped_any = true;
                    }
                },
                DescOp::MissingArg => {
                    swallow_introducing_sign(&mut out);
                    dropped_any = true;
                }
                DescOp::Drop { report } => {
                    dropped_args.push(report.clone());
                    swallow_introducing_sign(&mut out);
                    dropped_any = true;
                }
                DescOp::PercentOrDrop => {
                    if ends_in_a_real_per_cent_subject(&out) {
                        out.push('%');
                    } else {
                        dropped_args.push("%%".to_string());
                        swallow_introducing_sign(&mut out);
                        dropped_any = true;
                    }
                }
            }
        }

        let text = if dropped_any { collapse_whitespace(&out) } else { out };
        RenderedDescription { text: decode_escapes(&text, &self.escapes), dropped_args }
    }
}

/// A dropped slot takes the `+`/`-` that introduced it with it, so "a +  bonus" never reaches a
/// sheet.
fn swallow_introducing_sign(out: &mut String) {
    while out.ends_with('+') || out.ends_with('-') {
        out.pop();
    }
}

/// Whether the text built so far ends in something a per-cent sign can legitimately attach to:
/// a digit ("20%"), or word-boundary dice notation ("d%", "5d%" = d100).
fn ends_in_a_real_per_cent_subject(out: &str) -> bool {
    let chars: Vec<char> = out.chars().collect();
    let n = chars.len();
    if n == 0 {
        return false;
    }
    if chars[n - 1].is_ascii_digit() {
        return true;
    }
    matches!(chars[n - 1], 'd' | 'D') && (n < 2 || !chars[n - 2].is_alphabetic())
}

/// Collapses every whitespace run to a single space and trims the ends. Applied only when a slot
/// was dropped, so prose that needed no edit stays byte-identical to the source.
fn collapse_whitespace(text: &str) -> String {
    let mut out = String::new();
    let mut last_was_space = false;
    for ch in text.chars() {
        if ch.is_whitespace() {
            if !last_was_space {
                out.push(' ');
            }
            last_was_space = true;
        } else {
            out.push(ch);
            last_was_space = false;
        }
    }
    out.trim().to_string()
}

/// Applies the settled escape substitutions, in the order the template states them.
fn decode_escapes(text: &str, escapes: &[(String, String)]) -> String {
    if escapes.is_empty() {
        return text.to_string();
    }
    let mut out = text.to_string();
    for (from, to) in escapes {
        if out.contains(from.as_str()) {
            out = out.replace(from.as_str(), to);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env(pairs: &[(&str, i64)]) -> BTreeMap<String, i64> {
        pairs.iter().map(|(k, v)| ((*k).to_string(), *v)).collect()
    }

    fn text(ops: Vec<DescOp>, bindings: &[(&str, i64)]) -> RenderedDescription {
        DescTemplate { ops, args: Vec::new(), escapes: Vec::new() }.render(&env(bindings))
    }

    #[test]
    fn a_bound_slot_renders_this_characters_number() {
        let out = text(
            vec![
                DescOp::Text { text: "a +".to_string() },
                DescOp::Arg {
                    arg: DescArgument::Named { name: "MonkAC".to_string(), offset: None },
                    report: "MonkAC".to_string(),
                },
                DescOp::Text { text: " bonus".to_string() },
            ],
            &[("MonkAC", 3)],
        );
        assert_eq!(out.text, "a +3 bonus");
        assert!(out.dropped_args.is_empty());
    }

    #[test]
    fn an_unbound_slot_is_dropped_reported_and_takes_its_sign_with_it() {
        let out = text(
            vec![
                DescOp::Text { text: "a +".to_string() },
                DescOp::Arg {
                    arg: DescArgument::Named { name: "MonkAC".to_string(), offset: None },
                    report: "MonkAC".to_string(),
                },
                DescOp::Text { text: " bonus".to_string() },
            ],
            &[],
        );
        assert_eq!(out.text, "a bonus");
        assert_eq!(out.dropped_args, vec!["MonkAC".to_string()]);
    }

    #[test]
    fn a_literal_slot_needs_no_character() {
        let out = text(
            vec![
                DescOp::Text { text: "DC ".to_string() },
                DescOp::Arg {
                    arg: DescArgument::Literal { value: 15 },
                    report: "15".to_string(),
                },
            ],
            &[],
        );
        assert_eq!(out.text, "DC 15");
        assert!(out.dropped_args.is_empty());
    }

    #[test]
    fn the_offset_shape_is_only_tried_after_the_exact_name_misses() {
        let arg = DescArgument::Named {
            name: "Foo-1".to_string(),
            offset: Some(("Foo".to_string(), -1)),
        };
        assert_eq!(arg.value(&env(&[("Foo-1", 9), ("Foo", 100)])), Some(9));
        assert_eq!(arg.value(&env(&[("Foo", 100)])), Some(99));
        assert_eq!(arg.value(&env(&[])), None);
    }

    #[test]
    fn a_missing_argument_is_dropped_but_never_reported() {
        let out = text(
            vec![DescOp::Text { text: "gains ".to_string() }, DescOp::MissingArg],
            &[],
        );
        assert_eq!(out.text, "gains");
        assert!(out.dropped_args.is_empty(), "a slot with no argument text has nothing to name");
    }

    #[test]
    fn a_per_cent_sign_survives_after_a_number_and_after_dice_notation() {
        assert_eq!(
            text(vec![DescOp::Text { text: "20".to_string() }, DescOp::PercentOrDrop], &[]).text,
            "20%"
        );
        assert_eq!(
            text(vec![DescOp::Text { text: "5d".to_string() }, DescOp::PercentOrDrop], &[]).text,
            "5d%"
        );
    }

    #[test]
    fn an_orphaned_per_cent_sign_is_dropped_and_reported() {
        let out = text(
            vec![
                DescOp::Text { text: "a ".to_string() },
                DescOp::Arg {
                    arg: DescArgument::Named { name: "Chance".to_string(), offset: None },
                    report: "Chance".to_string(),
                },
                DescOp::PercentOrDrop,
                DescOp::Text { text: " chance".to_string() },
            ],
            &[],
        );
        assert_eq!(out.text, "a chance");
        assert_eq!(out.dropped_args, vec!["Chance".to_string(), "%%".to_string()]);
    }

    #[test]
    fn whitespace_is_collapsed_only_when_something_was_dropped() {
        let kept = text(vec![DescOp::Text { text: "two  spaces".to_string() }], &[]);
        assert_eq!(kept.text, "two  spaces");
        let collapsed = text(
            vec![
                DescOp::Text { text: "two  ".to_string() },
                DescOp::Drop { report: "CHOICE".to_string() },
                DescOp::Text { text: "  spaces".to_string() },
            ],
            &[],
        );
        assert_eq!(collapsed.text, "two spaces");
        assert_eq!(collapsed.dropped_args, vec!["CHOICE".to_string()]);
    }

    #[test]
    fn escapes_are_decoded_after_the_whitespace_collapse() {
        let rendered = DescTemplate {
            ops: vec![
                DescOp::Text { text: "one&nl;two   three".to_string() },
                DescOp::Drop { report: "CHOICE".to_string() },
            ],
            args: Vec::new(),
            escapes: vec![("&nl;".to_string(), "\n".to_string())],
        }
        .render(&env(&[]));
        assert_eq!(rendered.text, "one\ntwo three");
    }

    #[test]
    fn an_empty_template_renders_nothing() {
        let rendered = DescTemplate::default().render(&env(&[]));
        assert!(rendered.text.is_empty() && rendered.dropped_args.is_empty());
        assert!(DescTemplate::default().is_empty());
    }
}
