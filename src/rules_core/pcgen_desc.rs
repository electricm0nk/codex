//! Renders a PCGen `DESC:` token into the prose a player may actually be
//! shown, and guards against the raw token reaching a screen.
//!
//! ## Why this exists as a shared module rather than a fourth private copy
//!
//! Three ingest binaries already carry their own private copy of this
//! treatment (`src/bin/ingest_races.rs`, `src/bin/ingest_race_traits_arg.rs`,
//! `src/bin/ingest_pu_classes.rs`). Each of those reads a `.lst` row and can
//! resolve `%N` against that row's own `DEFINE:`/`BONUS:VAR` literals, so the
//! copies are genuinely row-shaped and are left alone.
//!
//! Spell descriptions are different in two ways that made a fourth private
//! copy the wrong answer:
//!
//! 1. **The leak reaches the player from the compiled tables, not from
//!    `data/corpus/`.** `apps/desktop/src-tauri/src/spell_catalog.rs` serves
//!    the Spell Catalog screen (and the Character Sheet's Add Spell picker)
//!    straight out of `rules_tables::{crb,apg,acg,advanced_race_guide}::
//!    spell_list`. Fixing only the ingest binary that writes
//!    `data/corpus/<book>/spell/*.json` would leave every leaking string on
//!    screen exactly as before.
//! 2. **A spell row defines no variables.** A spell's `DESC:` arguments are
//!    caster-level expressions (`CASTERLEVEL`, `min(3,1+(CASTERLEVEL-3)/4)`),
//!    never same-row literals, so there is no per-row variable table to pass
//!    in and the row-shaped signature the binaries use buys nothing here.
//!
//! ## What the render does
//!
//! PCGen's description syntax is not prose. A `DESC:` token is
//! `<prose>|<arg1>|<arg2>...`, `%N` references argument N, and `%%` is a
//! literal-percent escape. This module:
//!
//! * collapses `%%` to one `%` (lossless — nothing is looked up, nothing can
//!   be lost);
//! * substitutes `%N` when argument N is a plain integer literal;
//! * **drops** `%N` when argument N is anything else — an expression this
//!   engine cannot evaluate without the formula interpreter `decisions.md §24`
//!   forbids — taking the `+`/`-` sign that introduced it with it, closing the
//!   whitespace, and **reporting** the dropped argument rather than guessing a
//!   value. This is the identical discipline
//!   `ingest_race_traits_arg::substitute_placeholders` already ships (its
//!   `Halfling ~ Adaptable Luck` record reads "they only gain a bonus" for
//!   exactly this reason);
//! * removes the `|`-delimited argument tail in all cases.
//!
//! ## The `|` rule differs from the race-trait binary's, deliberately
//!
//! `ingest_race_traits_arg::leaked_pcgen_syntax` treats *any* `|` as a raw
//! argument tail. That is safe for racial-trait prose and **wrong** for spell
//! prose: CRB and APG spell text renders rulebook tables inline with ` | `
//! column separators. Derived over all four spell tables rather than assumed:
//! 151 `|` characters have whitespace on at least one side (every one of them
//! a prose table separator — "Hardness and Rarity | Examples", "HD | Strength
//! | Lingering Aura Duration"), and 13 have whitespace on neither side (every
//! one of them a real PCGen argument tail — "…the cloud's effects|CASTERLEVEL").
//! So the guard here flags a `|` only when it is *tight* — no whitespace on
//! either side — which is exactly the shape PCGen writes and exactly the shape
//! rulebook prose does not.

/// One rendered `DESC:` token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedPcgenDesc {
    /// The prose a player may be shown. Byte-identical to the input when the
    /// input carried no PCGen syntax at all.
    pub text: String,
    /// The `DESC:` arguments that could not be resolved to a literal and were
    /// therefore dropped from `text`. Reported, never guessed; a caller that
    /// wants to know what a description is missing reads this.
    pub dropped_args: Vec<String>,
}

/// The highest `%N` argument index the prose references, or 0 when it
/// references none. `%%` is an escape and is skipped, so `%%1` never counts
/// as `%1`.
fn max_arg_reference(raw: &str) -> usize {
    let chars: Vec<char> = raw.chars().collect();
    let mut max = 0usize;
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '%' && chars.get(i + 1) == Some(&'%') {
            i += 2;
            continue;
        }
        if chars[i] == '%'
            && let Some(digit) = chars.get(i + 1).and_then(|c| c.to_digit(10))
            && digit >= 1
        {
            max = max.max(digit as usize);
            i += 2;
            continue;
        }
        i += 1;
    }
    max
}

/// Collapses every whitespace run to a single space and trims the ends.
/// Applied only when a placeholder was dropped, so prose that needed no edit
/// stays byte-identical to the source.
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

/// Splits a raw `DESC:` token into its prose and its argument tail.
///
/// The tail is taken from the **right**, exactly as many segments as the prose
/// actually references, so a `|` inside the prose itself (a rulebook table
/// separator) is rejoined rather than mistaken for an argument boundary. A
/// token whose prose references no argument is returned whole and untouched.
fn split_prose_and_args(raw: &str) -> (String, Vec<String>) {
    let max = max_arg_reference(raw);
    if max == 0 {
        return (raw.to_string(), Vec::new());
    }
    let segments: Vec<&str> = raw.split('|').collect();
    if segments.len() <= max {
        // Fewer tail segments than the prose references: keep segment 0 as
        // prose and treat the rest as arguments. The unmatched `%N` then has
        // no argument at all and is dropped by `render_pcgen_desc`, which is
        // the honest outcome — the alternative is inventing a value.
        let prose = segments.first().copied().unwrap_or_default().to_string();
        let args = segments[1..].iter().map(|s| (*s).to_string()).collect();
        return (prose, args);
    }
    let split_at = segments.len() - max;
    let prose = segments[..split_at].join("|");
    let args = segments[split_at..].iter().map(|s| (*s).to_string()).collect();
    (prose, args)
}

/// Renders one raw PCGen `DESC:` token into player-facing prose.
///
/// See the module doc for the full contract. The short version: `%%` becomes
/// `%`, a `%N` backed by an integer literal becomes that integer, every other
/// `%N` is dropped and reported, and the argument tail never survives.
pub fn render_pcgen_desc(raw: &str) -> RenderedPcgenDesc {
    let (prose, args) = split_prose_and_args(raw);
    let chars: Vec<char> = prose.chars().collect();
    let mut out = String::new();
    let mut dropped_args: Vec<String> = Vec::new();
    let mut dropped_any = false;
    let mut i = 0;

    while i < chars.len() {
        // The escape is checked first: `%%` is never an argument reference,
        // and `%%1` would otherwise be misread as one.
        if chars[i] == '%' && chars.get(i + 1) == Some(&'%') {
            out.push('%');
            i += 2;
            continue;
        }
        if chars[i] == '%'
            && let Some(digit) = chars.get(i + 1).and_then(|c| c.to_digit(10))
            && digit >= 1
        {
            let arg = args.get(digit as usize - 1);
            match arg.and_then(|a| a.trim().parse::<i64>().ok()) {
                Some(value) => out.push_str(&value.to_string()),
                None => {
                    if let Some(name) = arg {
                        dropped_args.push(name.trim().to_string());
                    }
                    while out.ends_with('+') || out.ends_with('-') {
                        out.pop();
                    }
                    dropped_any = true;
                }
            }
            i += 2;
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }

    let text = if dropped_any { collapse_whitespace(&out) } else { out };
    RenderedPcgenDesc { text, dropped_args }
}

/// The PCGen syntax that must never reach a player: an unsubstituted `%N`
/// argument reference, an unresolved `%%` literal-percent escape, or a *tight*
/// `|` argument tail (see the module doc for why tightness, not the bare
/// character, is the test).
///
/// Returns the name of the leak so a caller can fail loudly with a reason
/// rather than a bare boolean.
pub fn leaked_pcgen_syntax(text: &str) -> Option<&'static str> {
    if text.contains("%%") {
        return Some("unescaped '%%' literal-percent escape");
    }
    let chars: Vec<char> = text.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        if *c == '%' && chars.get(i + 1).is_some_and(char::is_ascii_digit) {
            return Some("unsubstituted '%N' argument reference");
        }
        if *c == '|' {
            let left_open = i == 0 || chars[i - 1].is_whitespace();
            let right_open = chars.get(i + 1).is_none_or(|next| next.is_whitespace());
            if !left_open && !right_open {
                return Some("raw '|' argument tail");
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prose_with_no_pcgen_syntax_is_returned_byte_identical() {
        let raw = "You transform one alchemical item or firearm into another.";
        let rendered = render_pcgen_desc(raw);
        assert_eq!(rendered.text, raw);
        assert!(rendered.dropped_args.is_empty());
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
    }

    #[test]
    fn the_double_percent_escape_collapses_to_one_sign() {
        let rendered = render_pcgen_desc("any spellcasting with a verbal component has a 20%% spell failure chance.");
        assert_eq!(rendered.text, "any spellcasting with a verbal component has a 20% spell failure chance.");
        assert!(rendered.dropped_args.is_empty());
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
    }

    #[test]
    fn a_percent_n_backed_by_an_integer_literal_is_substituted() {
        let rendered = render_pcgen_desc("a +%1 luck bonus|2");
        assert_eq!(rendered.text, "a +2 luck bonus");
        assert!(rendered.dropped_args.is_empty());
    }

    #[test]
    fn a_percent_n_backed_by_a_formula_is_dropped_and_reported_never_guessed() {
        let rendered = render_pcgen_desc("contained within you for up to %1 rounds.|CASTERLEVEL");
        assert_eq!(rendered.text, "contained within you for up to rounds.");
        assert_eq!(rendered.dropped_args, vec!["CASTERLEVEL".to_string()]);
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
    }

    #[test]
    fn a_dropped_argument_takes_its_introducing_sign_with_it() {
        let rendered = render_pcgen_desc("The target gains a +%1 insight bonus.|min(6,2+CASTERLEVEL/4)");
        assert_eq!(rendered.text, "The target gains a insight bonus.");
        assert_eq!(rendered.dropped_args, vec!["min(6,2+CASTERLEVEL/4)".to_string()]);
    }

    #[test]
    fn two_arguments_are_taken_from_the_right_in_order() {
        let rendered = render_pcgen_desc("an AC equal to %1 and %2 hit points.|CASTERLEVEL/2+10|CASTERLEVEL");
        assert_eq!(rendered.text, "an AC equal to and hit points.");
        assert_eq!(
            rendered.dropped_args,
            vec!["CASTERLEVEL/2+10".to_string(), "CASTERLEVEL".to_string()]
        );
    }

    /// The reason this module exists rather than a fourth copy of the
    /// race-trait binary's helper: rulebook prose renders tables with ` | `
    /// column separators, and the race-trait binary's "any `|` is a tail" rule
    /// would shred them.
    #[test]
    fn a_prose_table_separator_is_not_an_argument_tail() {
        let raw = "as indicated on the following table. \nHardness and Rarity | Examples Duration \nVegetable matter | 2 hr./level";
        let rendered = render_pcgen_desc(raw);
        assert_eq!(rendered.text, raw, "no `%N` reference means no tail to strip");
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
    }

    #[test]
    fn a_prose_table_beside_a_real_argument_keeps_the_table_and_loses_the_tail() {
        let rendered = render_pcgen_desc("roll d%% for %1 rounds. \nRoll | Effect \n01-25 | Act normally|CASTERLEVEL");
        // Whitespace is collapsed only because an argument was dropped —
        // that is the one branch that rewrites spacing, and it is why the
        // newlines in the table are gone here but survive in
        // `a_prose_table_separator_is_not_an_argument_tail` above.
        assert_eq!(rendered.text, "roll d% for rounds. Roll | Effect 01-25 | Act normally");
        assert_eq!(rendered.dropped_args, vec!["CASTERLEVEL".to_string()]);
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
    }

    #[test]
    fn leak_guard_names_every_shape_that_must_never_reach_a_player() {
        assert_eq!(leaked_pcgen_syntax("Clean prose with 50% of something."), None);
        assert_eq!(leaked_pcgen_syntax("A +%1 bonus."), Some("unsubstituted '%N' argument reference"));
        assert_eq!(leaked_pcgen_syntax("reduced by 20%%."), Some("unescaped '%%' literal-percent escape"));
        assert_eq!(leaked_pcgen_syntax("the cloud's effects|CASTERLEVEL"), Some("raw '|' argument tail"));
        assert_eq!(leaked_pcgen_syntax("Hardness and Rarity | Examples"), None);
        assert_eq!(leaked_pcgen_syntax("trailing pipe |"), None);
    }

    /// A `%N` with no argument at all is still never shown.
    #[test]
    fn an_unmatched_percent_n_is_dropped_rather_than_rendered() {
        let rendered = render_pcgen_desc("a bonus of %3 with no tail");
        assert_eq!(rendered.text, "a bonus of with no tail");
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
    }
}
