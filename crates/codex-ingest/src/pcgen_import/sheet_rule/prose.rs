//! The PROSE rows: `DESC` / `BENEFIT` (left-to-right `%N` arguments as slots, C5), `SPROP` /
//! `SAB` (positional bare `%`, suppressed when every slot is 0, C6), `ASPECT` (display sub-keys
//! as labelled lines with `pick_last`, structural sub-keys as bookkeeping, `CheckCount` as the
//! uses value, C21), `TEMPDESC` (`When active:`), the spell stat-block tokens, and
//! `OUTPUTNAME`. Every text piece passes the product-identity screen (C20): a term hit omits
//! the segment and stamps `provenance.pi.term_hits`.

use super::ctx::{split_gates, RecordCtx};
use super::formula::{convert_formula, convert_prose_formula, integer_literal, is_prose_formula};
use codex::rules_core::pi_screening::normalized_term_hit;
use codex::rules_core::sheet_rule::{Applies, Expr, Holdable, ProseFamily, ProsePiece, ProseSegment};

/// PCGen's eight entities (`EntityEncoder.java:42-49`).
pub fn decode_entities(s: &str) -> String {
    s.replace("&nl;", "\n")
        .replace("&cr;", "\n")
        .replace("&lf;", "\n")
        .replace("&colon;", ":")
        .replace("&pipe;", "|")
        .replace("&lbracket;", "[")
        .replace("&rbracket;", "]")
        .replace("&amp;", "&")
}

/// English text that happens to spell a source-format literal (`(NCL=%1)` in two feat
/// descriptions): the words stay, the `=`/`:` glyph becomes a space so no output file carries
/// the literal (`mod.rs` `FORMULA_LITERALS`); recorded as a `literal-in-prose` defect.
pub fn scrub_literal_glyphs(ctx: &mut RecordCtx, text: &str, field_name: &str) -> String {
    let mut out = text.to_string();
    for lit in super::FORBIDDEN_LITERALS {
        if out.contains(lit) {
            ctx.defect("literal-in-prose", format!("{}: {field_name} ({lit})", ctx.record.id));
            let replacement = lit.replace([':', '='], " ");
            out = out.replace(lit, &replacement);
        }
    }
    out
}

/// The longest bracketed aside that can still be an editorial marker, in bytes. Same span the
/// detector uses (`wiring_class::EDITORIAL_MARKER_MAX_SPAN`), so scrub and detector agree.
const EDITORIAL_MARKER_MAX_SPAN: usize = 96;

/// Whether a bracketed group's words are an upstream editorial aside rather than the rule's.
///
/// Two shapes, both addressed to the source data's maintainers and neither to a player:
///
/// 1. **The not-implemented admission** -- a `not` followed by an `implement*`, the detector's
///    own rule (`wiring_class::carries_editorial_not_implemented_marker`).
/// 2. **An annotation head** -- the group opens with the word `note` and a colon
///    (`[NOTE:SOME ARCHETYPES MAY REQUIRE MANUAL INPUT ...]`). SD-35 `AT-35-E6-003` cycle 12:
///    two corpus records ship one, and both reached a catalog screen as part of the rule's
///    prose. The head is required, so a sentence that merely contains the word "note" inside a
///    parenthetical the rule itself wrote is untouched.
fn group_is_editorial_marker(group: &str) -> bool {
    let lower = group.to_ascii_lowercase();
    if lower.trim_start().strip_prefix("note").is_some_and(|rest| rest.trim_start().starts_with(':'))
    {
        return true;
    }
    let mut saw_not = false;
    for word in lower.split(|c: char| !c.is_ascii_alphanumeric()) {
        if word == "not" {
            saw_not = true;
        } else if saw_not && word.starts_with("implement") {
            return true;
        }
    }
    false
}

/// Remove upstream PCGen's own editorial not-implemented admission from a description.
///
/// SD-35 AT-35-E5-003. The marker (`[NOT IMPLEMENTED]`, `[Not Implemented]`,
/// `(NOT IMPLEMENTED)`, `[ML bonus not implemented.]`, and the mismatched-closer
/// `[NOT IMPLEMENTED}` one corpus record ships) says something about **PCGen's** automation,
/// not about the rule. Under the sheet rule (`decisions.md §1`) the sheet prints the rule's
/// words; a source tool's editorial aside is leakage of the same class the
/// `FORBIDDEN_LITERALS` scrub already removes.
///
/// Targeted, never a general bracket remover: only a bracketed group whose own words are the
/// admission is cut, and only when its closer is present, so a bracketed aside that belongs to
/// the rule (`Skill Focus (Knowledge [Arcana])`) is untouched. The seam is closed so the
/// remaining sentence reads as it did before the marker was inserted.
pub fn strip_editorial_not_implemented_markers(text: &str) -> String {
    let mut out = text.to_string();
    loop {
        let bytes = out.as_bytes();
        let mut cut: Option<(usize, usize)> = None;
        for (i, b) in bytes.iter().enumerate() {
            if *b != b'[' && *b != b'(' {
                continue;
            }
            let start = i + 1;
            let hard_end = (start + EDITORIAL_MARKER_MAX_SPAN).min(bytes.len());
            // Only a closed group is cut: without a closer the span's end is arbitrary and
            // removing it would take the rule's own words with it.
            let Some(off) = bytes[start..hard_end].iter().position(|c| matches!(c, b']' | b')' | b'}'))
            else {
                continue;
            };
            let end = start + off;
            if end <= start {
                continue;
            }
            // Byte slicing is safe only on a char boundary; a marker is ASCII, so anything
            // that is not is not a marker.
            let Some(group) = out.get(start..end) else { continue };
            if group_is_editorial_marker(group) {
                cut = Some((i, end + 1));
                break;
            }
        }
        let Some((from, to)) = cut else { return out };
        let prefix = &out[..from];
        let suffix = &out[to..];
        // Close the seam: when the marker sat at the start of the text or just after
        // whitespace, the space that separated it from the next word goes with it.
        let joined = if prefix.is_empty() || prefix.ends_with(char::is_whitespace) {
            format!("{prefix}{}", suffix.trim_start())
        } else {
            format!("{prefix}{suffix}")
        };
        out = joined.trim_end().to_string();
    }
}

/// [`strip_editorial_not_implemented_markers`], recording the removal as a converter defect so
/// the run's `_defects` report names every record it fired on.
pub fn scrub_editorial_markers(ctx: &mut RecordCtx, text: &str, field_name: &str) -> String {
    let scrubbed = strip_editorial_not_implemented_markers(text);
    if scrubbed != text {
        ctx.defect("editorial-marker-in-prose", format!("{}: {field_name}", ctx.record.id));
    }
    scrubbed
}

/// Screen one text for product identity. `Some(term)` on a hit.
pub fn pi_hit(text: &str) -> Option<&'static str> {
    if text.contains("[redacted PI]") {
        return Some("[redacted PI]");
    }
    normalized_term_hit(text)
}

/// Split a `%N`-template into pieces; `args[N-1]` supplies slot N. `%%` -> `%`; a lone `%`
/// stays text; `%{N}` = `%N`; a missing argument prints nothing.
fn template_pieces(ctx: &mut RecordCtx, text: &str, args: &[Slot]) -> Vec<ProsePiece> {
    let chars: Vec<char> = text.chars().collect();
    let mut pieces: Vec<ProsePiece> = Vec::new();
    let mut buf = String::new();
    let mut i = 0;
    let flush = |buf: &mut String, pieces: &mut Vec<ProsePiece>| {
        if !buf.is_empty() {
            pieces.push(ProsePiece::Text(std::mem::take(buf)));
        }
    };
    while i < chars.len() {
        let c = chars[i];
        if c != '%' {
            buf.push(c);
            i += 1;
            continue;
        }
        // `%%` -> literal `%`; `%%N` is substituted like `%N` (the recorded deviation in the
        // player's favour: PCGen prints "DC %1" on the three Water Jet rows).
        if chars.get(i + 1) == Some(&'%') {
            if chars.get(i + 2).is_some_and(|d| d.is_ascii_digit()) {
                ctx.defect("double-percent-slot", ctx.record.id.clone());
                i += 1;
                continue;
            }
            buf.push('%');
            i += 2;
            continue;
        }
        // `%CHOICE` / `%LIST` inside prose.
        let rest: String = chars[i + 1..].iter().take(6).collect();
        if rest.starts_with("CHOICE") || rest.starts_with("LIST") {
            let len = if rest.starts_with("CHOICE") { 6 } else { 4 };
            flush(&mut buf, &mut pieces);
            let id = ctx.choice_id.clone().unwrap_or_else(|| {
                ctx.defect("choice-marker-without-choose", ctx.record.id.clone());
                ctx.record.id.clone()
            });
            pieces.push(ProsePiece::ChoiceName(id));
            i += 1 + len;
            continue;
        }
        // `%N` / `%{N}`.
        let mut j = i + 1;
        let braced = chars.get(j) == Some(&'{');
        if braced {
            j += 1;
        }
        let start = j;
        while j < chars.len() && chars[j].is_ascii_digit() {
            j += 1;
        }
        if j == start {
            // A lone `%` stays text.
            buf.push('%');
            i += 1;
            continue;
        }
        let n: usize = chars[start..j].iter().collect::<String>().parse().unwrap_or(0);
        if braced {
            if chars.get(j) == Some(&'}') {
                j += 1;
            } else {
                buf.push('%');
                i += 1;
                continue;
            }
        }
        flush(&mut buf, &mut pieces);
        if n >= 1
            && let Some(slot) = args.get(n - 1)
        {
            pieces.push(slot.piece());
        }
        i = j;
    }
    flush(&mut buf, &mut pieces);
    pieces
}

#[derive(Clone)]
enum Slot {
    Expr(Expr),
    Choice(String),
    Dice { dice: String, modifier: Option<Expr> },
    /// The formula side could not lower this argument, so the slot prints the term's WORDS
    /// (`decisions.md` §1 form 3) instead of deleting the whole prose row from the sheet.
    Words(String),
}

impl Slot {
    fn piece(&self) -> ProsePiece {
        match self {
            Slot::Expr(e) => ProsePiece::Slot(e.clone()),
            Slot::Choice(c) => ProsePiece::ChoiceName(c.clone()),
            Slot::Dice { dice, modifier } => ProsePiece::Dice { dice: dice.clone(), modifier: modifier.clone() },
            Slot::Words(w) => ProsePiece::Text(w.clone()),
        }
    }
}

/// The leaf vocabulary [`words_for_unlowerable`] will name on a printed sheet. Anything not
/// on this list becomes `a rules variable` -- a corpus variable's name is source-format text
/// and naming it would put the ingest format on a player's sheet (the same policy
/// `rules_core::level_up_option_filter::describe_expr` applies to `Expr::Var`).
fn leaf_words(upper: &str) -> Option<&'static str> {
    Some(match upper {
        "CL" | "CASTERLEVEL" | "%CASTERLEVEL" => "caster level",
        "TL" | "TOTALLEVELS" | "ECL" => "character level",
        "HD" => "hit dice",
        "BAB" => "base attack bonus",
        "SIZE" => "size",
        "SIZEMOD" => "size modifier",
        "CR" => "challenge rating",
        "SPELLLEVEL" | "%SPELLLEVEL" => "spell level",
        "STR" => "Strength modifier",
        "DEX" => "Dexterity modifier",
        "CON" => "Constitution modifier",
        "INT" => "Intelligence modifier",
        "WIS" => "Wisdom modifier",
        "CHA" => "Charisma modifier",
        "STRSCORE" => "Strength",
        "DEXSCORE" => "Dexterity",
        "CONSCORE" => "Constitution",
        "INTSCORE" => "Intelligence",
        "WISSCORE" => "Wisdom",
        "CHASCORE" => "Charisma",
        "MASTERLEVEL" | "MASTERVAR" => "the master's level",
        _ => return None,
    })
}

/// Render a prose argument the formula side refused into plain English words.
///
/// SD-35 `AT-35-E6-003` cycle 6. Before this, one unlowerable `|`-argument made
/// `convert_desc_like` return `Err`, `convert_token`'s caller refused the **whole prose row**,
/// and the record reached the sheet with no description at all -- 30 feat rows and 2 spell
/// rows measured on the `AT-35-E6-003` cycle 5 swap, and 487 of 2,883 converted `feat` rules
/// and 634 of 3,102 `spell` rules carrying no prose at all
/// (`AT-35-E6-003_cycle5_converter-prose-blocker.md` §6). `decisions.md` §1 form 3 rules the
/// other way: a term the character does not settle **stays as words**. The description is the
/// book's own sentence and a Pathfinder book prints exactly this shape ("DC 10 + 1/2 your
/// caster level + your Wisdom modifier"), so the words are the right sheet line, not a
/// fallback.
///
/// The vocabulary is closed ([`leaf_words`]); operators become English. No ingest-format
/// identifier, token head, or formula punctuation survives -- that is what
/// `pcgen_residue_gate.py` and the `data/sheet_rules/` source-marker grep check for.
pub(crate) fn words_for_unlowerable(arg: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    let chars: Vec<char> = arg.trim().chars().collect();
    let mut i = 0;
    let mut atom = String::new();
    let push_atom = |atom: &mut String, out: &mut Vec<String>| {
        if atom.is_empty() {
            return;
        }
        let a = std::mem::take(atom);
        let word = if a.chars().all(|c| c.is_ascii_digit()) {
            a
        } else {
            leaf_words(&a.to_ascii_uppercase()).unwrap_or("a rules variable").to_string()
        };
        if out.last().map(String::as_str) != Some(word.as_str()) || word != "a rules variable" {
            out.push(word);
        }
    };
    while i < chars.len() {
        let c = chars[i];
        let op = match c {
            '+' => Some("plus"),
            '-' => Some("minus"),
            '*' => Some("times"),
            '/' => Some("divided by"),
            ',' => Some("and"),
            '(' | ')' | ' ' => Some(""),
            _ => None,
        };
        match op {
            Some(word) => {
                push_atom(&mut atom, &mut out);
                if !word.is_empty() {
                    out.push(word.to_string());
                }
            }
            None => atom.push(c),
        }
        i += 1;
    }
    push_atom(&mut atom, &mut out);
    // A leading/trailing operator word is a fragment, not a sentence.
    while out.first().is_some_and(|w| matches!(w.as_str(), "plus" | "minus" | "times" | "divided by" | "and")) {
        out.remove(0);
    }
    while out.last().is_some_and(|w| matches!(w.as_str(), "plus" | "minus" | "times" | "divided by" | "and")) {
        out.pop();
    }
    if out.is_empty() {
        return "a rules variable".to_string();
    }
    out.join(" ")
}

/// `d %%` -> `d%%`: percentile-dice notation written with a stray space in the SOURCE.
///
/// PCGen's Core Rulebook `Teleport` row states "Distance off target is d %% of the distance"
/// where its two sibling sentences state "roll d%%". The escape collapses to one `%` either
/// way, and the spaced form reaches a sheet as a bare `%` with nothing before it -- a hole,
/// as far as any reader can tell, where the rule means d100. Normalised at ingest with a
/// defect line, never by teaching a live-side reader to recognise one more shape.
fn normalize_percentile_dice(ctx: &mut RecordCtx, text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    let mut fixed = false;
    while i < chars.len() {
        let is_d = matches!(chars[i], 'd' | 'D');
        let boundary = i == 0 || !chars[i - 1].is_ascii_alphanumeric();
        if is_d && boundary && chars.get(i + 1) == Some(&' ') && chars.get(i + 2) == Some(&'%') {
            out.push(chars[i]);
            i += 2;
            fixed = true;
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    if fixed {
        ctx.defect("spaced-percentile-dice", ctx.record.id.clone());
    }
    out
}

/// SD-36 F7b: the byte ranges of every formula the source wrote into a prose TEXT itself
/// (rather than as a `%N` slot): a parenthesised group, or a function call, that
/// [`is_prose_formula`] reads as a source formula. `(min(10,CASTERLEVEL))` in Fireball's
/// short-form description; `(CASTERLEVEL) rounds` in 1,700-odd spell durations.
pub(crate) fn prose_formula_spans(text: &str) -> Vec<(usize, usize)> {
    let b = text.as_bytes();
    let matching_close = |open: usize| -> Option<usize> {
        let mut depth = 0usize;
        for (j, c) in b.iter().enumerate().skip(open) {
            match c {
                b'(' => depth += 1,
                b')' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(j);
                    }
                }
                _ => {}
            }
        }
        None
    };
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        let boundary = i == 0 || !(b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_');
        let open = if b[i] == b'(' {
            Some(i)
        } else if boundary && b[i].is_ascii_alphabetic() {
            let mut j = i;
            while j < b.len() && (b[j].is_ascii_alphanumeric() || b[j] == b'_') {
                j += 1;
            }
            (j < b.len() && b[j] == b'(').then_some(j)
        } else {
            None
        };
        if let Some(open) = open
            && let Some(close) = matching_close(open)
            && text.is_char_boundary(i)
            && is_prose_formula(&text[i..=close], text[..i].trim_end().ends_with(['+', '-']))
        {
            out.push((i, close + 1));
            i = close + 1;
            continue;
        }
        i += 1;
    }
    out
}

/// `d<sides>` right after a formula (`(min(10,CASTERLEVEL))d6`): the die size and the bytes it
/// takes, when the die is a whole word.
fn dice_suffix(rest: &str) -> Option<(u32, usize)> {
    let b = rest.as_bytes();
    if b.first() != Some(&b'd') {
        return None;
    }
    let digits = b[1..].iter().take_while(|c| c.is_ascii_digit()).count();
    if digits == 0 || b.get(1 + digits).is_some_and(|c| c.is_ascii_alphanumeric()) {
        return None;
    }
    Some((rest[1..1 + digits].parse().ok()?, 1 + digits))
}

/// `NdM+` right before a formula (`1d8+(min(5,CASTERLEVEL))`): the dice and where they start.
fn dice_prefix(before: &str) -> Option<(String, usize)> {
    let trimmed = before.trim_end();
    let without_plus = trimmed.strip_suffix('+')?.trim_end();
    let b = without_plus.as_bytes();
    let sides = b.iter().rev().take_while(|c| c.is_ascii_digit()).count();
    let d_at = b.len().checked_sub(sides + 1)?;
    if sides == 0 || b[d_at] != b'd' {
        return None;
    }
    let count = b[..d_at].iter().rev().take_while(|c| c.is_ascii_digit()).count();
    let start = d_at - count;
    if count == 0 || (start > 0 && b[start - 1].is_ascii_alphanumeric()) {
        return None;
    }
    Some((without_plus[start..].to_string(), start))
}

/// SD-36 F7b: a formula the source wrote into a prose TEXT prints as the rule's words, never as
/// its source spelling -- ONE rule for every family and every record (`epic-f` stage F7,
/// `f7b-receipt.md`).
///
/// Each span [`prose_formula_spans`] finds converts through the same formula converter every
/// `%N` argument goes through ([`convert_prose_formula`]) and becomes a typed piece the live
/// printers already render: a [`ProsePiece::DiceCount`] when the formula counts dice
/// (`(min(10,CASTERLEVEL))d6`), a [`ProsePiece::Dice`] modifier when it follows `NdM+`
/// (`1d8+(min(5,CASTERLEVEL))`), else a [`ProsePiece::Slot`]. Each is one
/// `inline-formula-in-prose` defect row. A span that does not convert is an
/// `inline-formula-unconverted` row and the words print without it -- never the token.
///
/// In a spell's (or psionic power's) text the source writes the book's own parentheses as
/// brackets (`[maximum 10d6]`, `[D]`), because a parenthesis there is a formula; with the
/// formulas lowered, the brackets print as the book's parentheses (Core Rulebook p.284:
/// "(maximum 10d6)"). One `bracket-escape-in-spell-text` row per field.
pub(crate) fn lower_prose_formulas(ctx: &mut RecordCtx, pieces: Vec<ProsePiece>, field_name: &str) -> Vec<ProsePiece> {
    let spell_text = matches!(ctx.record.kind.as_str(), "spell" | "power");
    let mut escaped = false;
    let mut out: Vec<ProsePiece> = Vec::new();
    let flush = |buf: &mut String, out: &mut Vec<ProsePiece>, escaped: &mut bool| {
        if buf.is_empty() {
            return;
        }
        let mut text = std::mem::take(buf);
        if spell_text && text.contains(['[', ']']) {
            text = text.replace('[', "(").replace(']', ")");
            *escaped = true;
        }
        out.push(ProsePiece::Text(text));
    };
    for piece in pieces {
        let text = match piece {
            ProsePiece::Text(t) => t,
            other => {
                out.push(other);
                continue;
            }
        };
        let mut buf = String::new();
        let mut cursor = 0;
        for (a, b) in prose_formula_spans(&text) {
            if a < cursor {
                continue;
            }
            buf.push_str(&text[cursor..a]);
            let mut next = b;
            match convert_prose_formula(ctx, &text[a..b]) {
                Ok(expr) => {
                    ctx.defect("inline-formula-in-prose", format!("{}: {field_name}", ctx.record.id));
                    if let Some((sides, len)) = dice_suffix(&text[b..]) {
                        flush(&mut buf, &mut out, &mut escaped);
                        out.push(ProsePiece::DiceCount { count: expr, sides });
                        next = b + len;
                    } else if let Some((dice, start)) = dice_prefix(&buf) {
                        buf.truncate(start);
                        flush(&mut buf, &mut out, &mut escaped);
                        out.push(ProsePiece::Dice { dice, modifier: Some(expr) });
                    } else {
                        flush(&mut buf, &mut out, &mut escaped);
                        out.push(ProsePiece::Slot(expr));
                    }
                }
                Err(_) => {
                    ctx.defect("inline-formula-unconverted", format!("{}: {field_name}", ctx.record.id));
                    // The words without the formula; the seam keeps one space.
                    if (buf.is_empty() || buf.ends_with(' ')) && text[b..].starts_with(' ') {
                        next = b + 1;
                    }
                }
            }
            cursor = next;
        }
        buf.push_str(&text[cursor..]);
        flush(&mut buf, &mut out, &mut escaped);
    }
    if escaped {
        ctx.defect("bracket-escape-in-spell-text", format!("{}: {field_name}", ctx.record.id));
    }
    out
}

/// Whether a gate names a record outside the converted inventory (`Holdable::MissingRule`).
fn names_out_of_inventory(gate: &Applies) -> bool {
    match gate {
        Applies::Holds { what: Holdable::MissingRule { .. }, .. } => true,
        Applies::All(terms) | Applies::AtLeast { of: terms, .. } => terms.iter().any(names_out_of_inventory),
        Applies::Not(inner) => names_out_of_inventory(inner),
        _ => false,
    }
}

/// SD-36 F7b: a prose line's condition with every out-of-inventory term DECIDED, the way the sheet
/// evaluator decides it (`Holdable::MissingRule` never holds): the term is `Never`, its
/// negation `Always`, and the conjunctions fold. A line whose condition is then `Never` is a line
/// no character is ever shown, and a condition that folds to `Always` is no condition -- so no
/// "If requires Fireball from mythic spell (no record in the corpus)" sentence reaches a page.
fn decide_out_of_inventory(gate: Applies) -> Applies {
    match gate {
        Applies::Holds { what: Holdable::MissingRule { .. }, .. } => Applies::Never,
        Applies::Not(inner) => match decide_out_of_inventory(*inner) {
            Applies::Never => Applies::Always,
            Applies::Always => Applies::Never,
            other => Applies::Not(Box::new(other)),
        },
        Applies::All(terms) => Applies::all(terms.into_iter().map(decide_out_of_inventory).collect()),
        Applies::AtLeast { n, of } => {
            let decided: Vec<Applies> = of.into_iter().map(decide_out_of_inventory).collect();
            let met = decided.iter().filter(|t| **t == Applies::Always).count();
            let open: Vec<Applies> = decided.into_iter().filter(|t| *t != Applies::Always && *t != Applies::Never).collect();
            let need = usize::from(n).saturating_sub(met);
            if need == 0 {
                Applies::Always
            } else if open.len() < need {
                Applies::Never
            } else if open.len() == 1 {
                open.into_iter().next().expect("one open term")
            } else if need == open.len() {
                Applies::all(open)
            } else {
                Applies::AtLeast { n: u8::try_from(need).unwrap_or(u8::MAX), of: open }
            }
        }
        other => other,
    }
}

/// [`segment_gate`] with [`decide_out_of_inventory`] applied: `Err(())` when the line is decided
/// never to print (one `prose-line-out-of-inventory` row), else its gate (one
/// `prose-condition-out-of-inventory` row when a decided term changed it).
fn segment_gate_decided(ctx: &mut RecordCtx, gates: &[String], field_name: &str) -> Result<Result<Option<Applies>, ()>, String> {
    let Some(gate) = segment_gate(ctx, gates)? else { return Ok(Ok(None)) };
    if !names_out_of_inventory(&gate) {
        return Ok(Ok(Some(gate)));
    }
    match decide_out_of_inventory(gate) {
        Applies::Never => {
            ctx.defect("prose-line-out-of-inventory", format!("{}: {field_name}", ctx.record.id));
            Ok(Err(()))
        }
        decided => {
            ctx.defect("prose-condition-out-of-inventory", format!("{}: {field_name}", ctx.record.id));
            Ok(Ok(if decided == Applies::Always { None } else { Some(decided) }))
        }
    }
}

/// Lower one prose argument, degrading to [`Slot::Words`] when the formula side refuses it.
///
/// The degradation is recorded on the record exactly as the old `Err` path recorded it --
/// same shape, same census `under` -- so `token_coverage.py`'s ledger and `_report.json`'s
/// `degraded_by_token_type` are unchanged by this cycle. Only the prose survives that did not.
fn argument_or_words(ctx: &mut RecordCtx, arg: &str, field_name: &str) -> Result<Slot, String> {
    match convert_argument(ctx, arg) {
        Ok(slot) => Ok(slot),
        Err(tt) => {
            if super::ctx::is_record_refusal(&tt) {
                return Err(tt);
            }
            let under = ctx.current_under.clone().unwrap_or_else(|| field_name.to_string());
            ctx.refuse_under(&under, tt);
            Ok(Slot::Words(words_for_unlowerable(arg)))
        }
    }
}

fn convert_argument(ctx: &mut RecordCtx, arg: &str) -> Result<Slot, String> {
    let a = arg.trim();
    if a == "%CHOICE" || a == "%LIST" {
        let id = ctx.choice_id.clone().unwrap_or_else(|| {
            ctx.defect("choice-marker-without-choose", ctx.record.id.clone());
            ctx.record.id.clone()
        });
        return Ok(Slot::Choice(id));
    }
    if let Some(n) = integer_literal(a) {
        return Ok(Slot::Expr(Expr::Const(n)));
    }
    // A dice argument (`1d6+%2` stays dice) -- rare; a plain `NdM` literal.
    if let Some((n, rest)) = a.split_once('d')
        && n.chars().all(|c| c.is_ascii_digit())
        && !n.is_empty()
        && rest.chars().all(|c| c.is_ascii_digit())
        && !rest.is_empty()
    {
        return Ok(Slot::Dice { dice: a.to_string(), modifier: None });
    }
    Ok(Slot::Expr(convert_formula(ctx, a)?))
}

/// Convert the gate segments of a prose token to the segment's `Applies` (C5: a gate whose
/// operands are all literals is decided now; anything else stays on the segment).
fn segment_gate(ctx: &mut RecordCtx, gates: &[String]) -> Result<Option<Applies>, String> {
    if gates.is_empty() {
        return Ok(None);
    }
    let mut terms = Vec::new();
    for g in gates {
        if g == "PRE:.CLEAR" {
            terms.clear();
            continue;
        }
        terms.push(super::prereq::convert_pre_token(ctx, g)?);
    }
    Ok(match Applies::all(terms) {
        Applies::Always => None,
        other => Some(other),
    })
}

/// A `PRE<KIND>:` head glued into a text field with no separator (a source defect the
/// converter re-tokenises, `blockers.md` B10): `(text, Some(gate))` when found.
pub fn split_glued_gate(text: &str) -> (String, Option<String>) {
    let bytes = text.as_bytes();
    let mut i = 0;
    while let Some(pos) = text[i..].find("PRE") {
        let start = i + pos;
        let mut j = start + 3;
        while j < bytes.len() && bytes[j].is_ascii_uppercase() {
            j += 1;
        }
        let head_ok = j > start + 3 && j < bytes.len() && bytes[j] == b':';
        let boundary_ok = start == 0 || !bytes[start - 1].is_ascii_alphanumeric();
        if head_ok && boundary_ok {
            let neg = start > 0 && bytes[start - 1] == b'!';
            let cut = if neg { start - 1 } else { start };
            return (text[..cut].to_string(), Some(text[cut..].to_string()));
        }
        i = start + 3;
    }
    (text.to_string(), None)
}

/// Rows DESC / BENEFIT: `<text>|<arg1>|...|<PRE...>` -> one segment with left-to-right slots.
pub fn convert_desc_like(ctx: &mut RecordCtx, family: ProseFamily, value: &str, field_name: &str) -> Result<Option<ProseSegment>, String> {
    let (fields, mut gates) = split_gates(value);
    let raw_text = fields.first().map(|s| s.as_str()).unwrap_or("");
    let (raw_text, glued) = split_glued_gate(raw_text);
    if let Some(g) = glued {
        ctx.defect("glued-tokens", format!("{}: {field_name}", ctx.record.id));
        gates.push(g);
    }
    let text = decode_entities(&raw_text);
    if text.trim().is_empty() {
        return Ok(None);
    }
    if let Some(_hit) = pi_hit(&text) {
        ctx.pi_term_hits.push(field_name.to_string());
        return Ok(None);
    }
    let text = scrub_literal_glyphs(ctx, &text, field_name);
    let text = scrub_editorial_markers(ctx, &text, field_name);
    let text = normalize_percentile_dice(ctx, &text);
    let mut args = Vec::new();
    for a in fields.iter().skip(1) {
        args.push(argument_or_words(ctx, a, field_name)?);
    }
    let Ok(applies) = segment_gate_decided(ctx, &gates, field_name)? else { return Ok(None) };
    let pieces = template_pieces(ctx, &text, &args);
    let pieces = lower_prose_formulas(ctx, pieces, field_name);
    Ok(Some(ProseSegment { family, pieces, applies, pick_last: false, suppress_when_all_zero: false }))
}

/// Rows SPROP / SAB (C6): positional bare `%`; suppressed when every variable is 0.
pub fn convert_positional(ctx: &mut RecordCtx, family: ProseFamily, value: &str, field_name: &str) -> Result<Option<ProseSegment>, String> {
    let (fields, mut gates) = split_gates(value);
    let raw_text = fields.first().map(|s| s.as_str()).unwrap_or("");
    let (raw_text, glued) = split_glued_gate(raw_text);
    if let Some(g) = glued {
        ctx.defect("glued-tokens", format!("{}: {field_name}", ctx.record.id));
        gates.push(g);
    }
    let text = decode_entities(&raw_text);
    if text.trim().is_empty() || text.trim() == ".CLEAR" {
        return Ok(None);
    }
    if let Some(_hit) = pi_hit(&text) {
        ctx.pi_term_hits.push(field_name.to_string());
        return Ok(None);
    }
    let text = scrub_literal_glyphs(ctx, &text, field_name);
    let text = scrub_editorial_markers(ctx, &text, field_name);
    let text = normalize_percentile_dice(ctx, &text);
    let mut vars: Vec<Slot> = Vec::new();
    for a in fields.iter().skip(1) {
        vars.push(argument_or_words(ctx, a, field_name)?);
    }
    let Ok(applies) = segment_gate_decided(ctx, &gates, field_name)? else { return Ok(None) };
    // Each bare `%` (not `%%`, not `%CHOICE`/`%LIST`) is the next slot.
    let chars: Vec<char> = text.chars().collect();
    let mut pieces: Vec<ProsePiece> = Vec::new();
    let mut buf = String::new();
    let mut next = 0usize;
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c != '%' {
            buf.push(c);
            i += 1;
            continue;
        }
        if chars.get(i + 1) == Some(&'%') {
            buf.push('%');
            i += 2;
            continue;
        }
        let rest: String = chars[i + 1..].iter().take(6).collect();
        if rest.starts_with("CHOICE") || rest.starts_with("LIST") {
            let len = if rest.starts_with("CHOICE") { 6 } else { 4 };
            if !buf.is_empty() {
                pieces.push(ProsePiece::Text(std::mem::take(&mut buf)));
            }
            let id = ctx.choice_id.clone().unwrap_or_else(|| {
                ctx.defect("choice-marker-without-choose", ctx.record.id.clone());
                ctx.record.id.clone()
            });
            pieces.push(ProsePiece::ChoiceName(id));
            i += 1 + len;
            continue;
        }
        if !buf.is_empty() {
            pieces.push(ProsePiece::Text(std::mem::take(&mut buf)));
        }
        if let Some(slot) = vars.get(next) {
            pieces.push(slot.piece());
        }
        next += 1;
        i += 1;
    }
    if !buf.is_empty() {
        pieces.push(ProsePiece::Text(buf));
    }
    // The `%` slots decide the all-zero suppression, as before F7b; a formula lowered out of the
    // words is the rule's text, never a reason to hide the line.
    let has_slots = pieces.iter().any(|p| matches!(p, ProsePiece::Slot(_)));
    let pieces = lower_prose_formulas(ctx, pieces, field_name);
    Ok(Some(ProseSegment { family, pieces, applies, pick_last: false, suppress_when_all_zero: has_slots }))
}

/// A labelled prose segment from a plain text value (stat-block lines, `TEMPDESC`, display
/// `ASPECT` lines). Same `%N` grammar as DESC.
pub fn convert_labelled(ctx: &mut RecordCtx, family: ProseFamily, value: &str, field_name: &str, pick_last: bool) -> Result<Option<ProseSegment>, String> {
    let seg = convert_desc_like(ctx, family, value, field_name)?;
    Ok(seg.map(|mut s| {
        s.pick_last = pick_last;
        s
    }))
}

/// Row OUTPUTNAME: the printed name. `[NAME]` = the parenthesised part of the record name,
/// `/`-split and reversed; `[BASE]` = the part before the parenthesis; a name without a
/// parenthesis makes `[NAME]` the whole name.
pub fn expand_output_name(output_name: &str, record_name: &str) -> String {
    let (base, paren) = match record_name.find('(') {
        Some(i) => {
            let inner = record_name[i + 1..].trim_end_matches(')').to_string();
            (record_name[..i].trim().to_string(), Some(inner))
        }
        None => (record_name.trim().to_string(), None),
    };
    let name_part = match &paren {
        Some(inner) => {
            let mut parts: Vec<&str> = inner.split('/').map(|s| s.trim()).collect();
            parts.reverse();
            parts.join(" ")
        }
        None => record_name.trim().to_string(),
    };
    output_name.replace("[NAME]", &name_part).replace("[BASE]", &base)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The ACG row that named the blocker (`acg_feats.lst:20`, `Befuddling Strike`): its DC
    /// argument is `CL/2+10+WIS`, `CL` has no owning class on a feat, and before this cycle
    /// the whole `DESC:` row was refused. It now prints the book's own sentence.
    #[test]
    fn an_unlowerable_argument_prints_the_terms_words() {
        assert_eq!(words_for_unlowerable("CL/2+10+WIS"), "caster level divided by 2 plus 10 plus Wisdom modifier");
        assert_eq!(words_for_unlowerable("10+CHA"), "10 plus Charisma modifier");
        assert_eq!(words_for_unlowerable("HD/2"), "hit dice divided by 2");
        assert_eq!(words_for_unlowerable("(CL+2)*3"), "caster level plus 2 times 3");
    }

    /// A corpus variable's own name never reaches a sheet: it is source-format text, and the
    /// live side applies the same policy to `Expr::Var`.
    #[test]
    fn an_unknown_leaf_is_words_not_its_source_name() {
        assert_eq!(words_for_unlowerable("BefuddlingStrikeTimes"), "a rules variable");
        assert_eq!(words_for_unlowerable("MYSTERY_VAR+WIS"), "a rules variable plus Wisdom modifier");
        assert_eq!(words_for_unlowerable(""), "a rules variable");
        assert_eq!(words_for_unlowerable("+"), "a rules variable");
    }

    /// SD-36 F7b: the formula-shaped groups the corpus's prose carries are found; the book's
    /// own asides are not.
    #[test]
    fn prose_formula_spans_find_source_formulas_and_leave_the_books_asides() {
        let spans = |t: &str| prose_formula_spans(t).into_iter().map(|(a, b)| t[a..b].to_string()).collect::<Vec<_>>();
        assert_eq!(spans("deals (min(10,CASTERLEVEL))d6 points"), vec!["(min(10,CASTERLEVEL))"]);
        assert_eq!(spans("(CASTERLEVEL) rounds [D]"), vec!["(CASTERLEVEL)"]);
        assert_eq!(spans("takes min(10,CASTERLEVEL/2)d6 points"), vec!["min(10,CASTERLEVEL/2)"]);
        assert_eq!(spans("converts 2d6+min(5,CASTERLEVEL) points"), vec!["min(5,CASTERLEVEL)"]);
        assert_eq!(spans("(ConjurationSummonersCharmBonus+(CASTERLEVEL)) rounds"), vec!["(ConjurationSummonersCharmBonus+(CASTERLEVEL))"]);
        assert_eq!(spans("(CASTERLELVEL) creatures"), vec!["(CASTERLELVEL)"]);
        assert_eq!(spans("Heal yourself 1d8+(TL) hp"), vec!["(TL)"]);
        assert!(spans("Headband (CHA) +4").is_empty());
        assert!(spans("Dusk Kamadan (CR +1)").is_empty());
        assert_eq!(spans("(HD+2) rounds"), vec!["(HD+2)"]);
        for aside in [
            "(DC 15)",
            "(APG)",
            "(see text)",
            "(CL 12th)",
            "(Ex)",
            "(and/or)",
            "(Str/Dex)",
            "(1/2)",
            "(10)",
            "(2,500GP)",
            "(NOT IMPLEMENTED)",
            "(DC10+HD)",
            "(DC 10 + 1/2 your character level + your Wis modifier)",
        ] {
            assert!(spans(aside).is_empty(), "{aside} is the book's words");
        }
    }

    #[test]
    fn dice_around_a_formula_are_read_as_dice() {
        assert_eq!(dice_suffix("d6 points"), Some((6, 2)));
        assert_eq!(dice_suffix("d10."), Some((10, 3)));
        assert_eq!(dice_suffix("damage"), None);
        assert_eq!(dice_suffix("d6x"), None);
        assert_eq!(dice_prefix("cure 1d8+"), Some(("1d8".to_string(), 5)));
        assert_eq!(dice_prefix("converts 2d6 + "), Some(("2d6".to_string(), 9)));
        assert_eq!(dice_prefix("a +"), None);
        assert_eq!(dice_prefix("x1d8+"), None);
    }

    /// SD-36 F7b: a condition naming a record outside the inventory is decided as the evaluator
    /// decides it, so the line either never prints or prints with no such sentence.
    #[test]
    fn an_out_of_inventory_condition_is_decided_never_printed() {
        let missing = || Applies::Holds { what: Holdable::MissingRule { pool: "mythic_spell".into(), name: "Fireball".into() }, count: 1 };
        let other = Applies::Situational { text: "when active".into() };
        assert_eq!(decide_out_of_inventory(missing()), Applies::Never);
        assert_eq!(decide_out_of_inventory(Applies::Not(Box::new(missing()))), Applies::Always);
        assert_eq!(decide_out_of_inventory(Applies::All(vec![missing(), other.clone()])), Applies::Never);
        assert_eq!(decide_out_of_inventory(Applies::All(vec![Applies::Not(Box::new(missing())), other.clone()])), other);
        assert_eq!(decide_out_of_inventory(Applies::AtLeast { n: 1, of: vec![missing(), other.clone()] }), other);
        assert_eq!(decide_out_of_inventory(Applies::AtLeast { n: 1, of: vec![missing(), missing()] }), Applies::Never);
        assert!(!names_out_of_inventory(&other));
    }

    #[test]
    fn output_name_expands_name_and_base() {
        assert_eq!(expand_output_name("Black Powder, [NAME]", "Black Powder (Keg)"), "Black Powder, Keg");
        assert_eq!(expand_output_name("[NAME] Zoic Fetish", "Zoic Fetish (Mammal)"), "Mammal Zoic Fetish");
        assert_eq!(expand_output_name("[NAME]", "Genie (Janni/Noble)"), "Noble Janni");
        assert_eq!(expand_output_name("[NAME] x", "Plain"), "Plain x");
    }

    #[test]
    fn glued_gate_is_split_off_the_text() {
        let (t, g) = split_glued_gate("as a swift action.PRECLASS:1,Vigilante=10");
        assert_eq!(t, "as a swift action.");
        assert_eq!(g.as_deref(), Some("PRECLASS:1,Vigilante=10"));
        let (t, g) = split_glued_gate("PREREQUISITES are listed");
        assert_eq!(g, None);
        assert_eq!(t, "PREREQUISITES are listed");
    }

    #[test]
    fn entities_decode() {
        assert_eq!(decode_entities("a&colon; b &amp; c"), "a: b & c");
    }

    /// SD-35 AT-35-E5-003. Upstream PCGen's own editorial not-implemented admission is an
    /// annotation about PCGen's automation, not the rule's words, and a paper sheet must
    /// never print it (`decisions.md §1`, the sheet rule). Every shape below is a real
    /// corpus one: the bracketed prefix, the mismatched `}` closer that
    /// `monster_codex:feat:vampiric_companion` ships, the parenthesised spelling, and the
    /// sentence-shaped `[ML bonus not implemented.]` aside.
    #[test]
    fn upstream_editorial_not_implemented_markers_are_stripped_from_prose() {
        assert_eq!(
            strip_editorial_not_implemented_markers("[Not Implemented] Your curse weighs down your soul."),
            "Your curse weighs down your soul."
        );
        assert_eq!(
            strip_editorial_not_implemented_markers(
                "reflects the vile nature of vampirism.\n[NOT IMPLEMENTED}Your animal companion changes."
            ),
            "reflects the vile nature of vampirism.\nYour animal companion changes."
        );
        assert_eq!(
            strip_editorial_not_implemented_markers("(NOT IMPLEMENTED) You gain a +2 bonus."),
            "You gain a +2 bonus."
        );
        assert_eq!(
            strip_editorial_not_implemented_markers("You gain fast healing 1. [ML bonus not implemented.]"),
            "You gain fast healing 1."
        );
        assert_eq!(
            strip_editorial_not_implemented_markers(
                "You have mastered ancient techniques.\n[NOT IMPLEMENTED] You're proficient with them."
            ),
            "You have mastered ancient techniques.\nYou're proficient with them."
        );
    }

    /// The scrub is targeted, never a general bracket remover: a bracketed aside that is part
    /// of the rule's own words survives untouched, and so does an unmarked description.
    #[test]
    fn a_bracketed_aside_that_is_the_rules_own_words_survives_the_scrub() {
        for text in [
            "Skill Focus (Knowledge [Arcana]) applies.",
            "You gain a +2 bonus on Intimidate checks (see page 42).",
            "The implement is not usable underwater.",
            "",
        ] {
            assert_eq!(strip_editorial_not_implemented_markers(text), text);
        }
    }

    /// SD-35 `AT-35-E6-003` cycle 12: an upstream annotation addressed to the data's
    /// maintainers is the same leakage class as the not-implemented admission and is scrubbed
    /// the same way -- it says something about the source tool's automation, never about the
    /// rule, and it reached a catalog screen inside the rule's own prose.
    #[test]
    fn an_upstream_annotation_head_is_scrubbed_and_the_seam_is_closed() {
        assert_eq!(
            strip_editorial_not_implemented_markers(
                "He gains another such feat. [NOTE:SOME ARCHETYPES MAY REQUIRE MANUAL INPUT!]"
            ),
            "He gains another such feat."
        );
        assert_eq!(
            strip_editorial_not_implemented_markers(
                "[NOTE:Not fully restricted. Check domain power before choosing!]You gain a domain."
            ),
            "You gain a domain."
        );
        // The head is required: the rule's own parenthetical survives.
        for text in [
            "Note that this bonus does not stack.",
            "You gain a +2 bonus (note the duration).",
        ] {
            assert_eq!(strip_editorial_not_implemented_markers(text), text);
        }
    }

    /// The scrub and the detector agree: nothing the detector flags survives the scrub.
    #[test]
    fn nothing_the_detector_flags_survives_the_scrub() {
        for text in [
            "[Not Implemented] Your curse weighs down your soul.",
            "vampirism.\n[NOT IMPLEMENTED}Your animal companion changes.",
            "(NOT IMPLEMENTED) You gain a +2 bonus.",
            "You gain fast healing 1. [ML bonus not implemented.]",
        ] {
            assert!(crate::pcgen_import::wiring_class::carries_editorial_not_implemented_marker(text));
            let scrubbed = strip_editorial_not_implemented_markers(text);
            assert!(
                !crate::pcgen_import::wiring_class::carries_editorial_not_implemented_marker(&scrubbed),
                "marker survived the scrub: {scrubbed:?}"
            );
        }
    }
}
