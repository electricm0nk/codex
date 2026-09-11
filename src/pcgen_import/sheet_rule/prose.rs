//! The PROSE rows: `DESC` / `BENEFIT` (left-to-right `%N` arguments as slots, C5), `SPROP` /
//! `SAB` (positional bare `%`, suppressed when every slot is 0, C6), `ASPECT` (display sub-keys
//! as labelled lines with `pick_last`, structural sub-keys as bookkeeping, `CheckCount` as the
//! uses value, C21), `TEMPDESC` (`When active:`), the spell stat-block tokens, and
//! `OUTPUTNAME`. Every text piece passes the product-identity screen (C20): a term hit omits
//! the segment and stamps `provenance.pi.term_hits`.

use super::ctx::{split_gates, RecordCtx};
use super::formula::{convert_formula, integer_literal};
use crate::rules_core::pi_screening::normalized_term_hit;
use crate::rules_core::sheet_rule::{Applies, Expr, ProseFamily, ProsePiece, ProseSegment};

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

/// Whether a bracketed group's words are upstream PCGen's not-implemented admission -- a `not`
/// followed by an `implement*`, the detector's own rule
/// (`wiring_class::carries_editorial_not_implemented_marker`).
fn group_is_editorial_marker(group: &str) -> bool {
    let lower = group.to_ascii_lowercase();
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

/// A parenthesised formula written into the prose TEXT itself, rather than as a `%N` slot,
/// printed as the rule's words.
///
/// `inner_sea_world_guide:spell:ancestral_memory` states
/// "(70+CASTERLEVEL)% chance of obtaining specific ancestral memory" in the body of its
/// description: the variable name is source-format text in a sentence, so no slot converts it
/// and it reached the Spell Catalog screen verbatim (SD-35 `AT-35-E6-003` cycle 2's
/// `correction 1789093674266`, still open at cycle 5). The rewrite is closed-vocabulary: a
/// group only qualifies when its whole content is formula punctuation AND it names at least
/// one leaf [`leaf_words`] knows, so an ordinary parenthetical aside is never touched.
fn scrub_inline_formula(ctx: &mut RecordCtx, text: &str, field_name: &str) -> String {
    match rewrite_inline_formula(text) {
        Some(rewritten) => {
            ctx.defect("inline-formula-in-prose", format!("{}: {field_name}", ctx.record.id));
            rewritten
        }
        None => text.to_string(),
    }
}

/// The pure half of [`scrub_inline_formula`]: `Some(rewritten)` when at least one group was
/// rewritten, `None` when the text carries no inline formula at all.
fn rewrite_inline_formula(text: &str) -> Option<String> {
    if !text.contains('(') {
        return None;
    }
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    let mut rewrote = false;
    while let Some(open) = rest.find('(') {
        let Some(close_rel) = rest[open + 1..].find(')') else { break };
        let close = open + 1 + close_rel;
        let inner = &rest[open + 1..close];
        let formula_shaped = !inner.is_empty()
            && inner.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || matches!(c, '+' | '-' | '*' | '/' | '.' | ' ' | '_'))
            && inner
                .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                .any(|w| !w.is_empty() && !w.chars().all(|c| c.is_ascii_digit()) && leaf_words(w).is_some());
        out.push_str(&rest[..=open]);
        if formula_shaped {
            out.push_str(&words_for_unlowerable(inner));
            rewrote = true;
        } else {
            out.push_str(inner);
        }
        out.push(')');
        rest = &rest[close + 1..];
        // `(70+CASTERLEVEL)%` -- the percent sign belongs to the rewritten term, and a `%`
        // that no digit precedes reads as a hole to every reader and every leak check. The
        // words carry it.
        if formula_shaped && rest.starts_with('%') {
            out.push_str(" percent");
            // The source writes the sign as the `%%` literal-percent escape as often as bare.
            rest = if rest.starts_with("%%") { &rest[2..] } else { &rest[1..] };
        }
    }
    out.push_str(rest);
    if rewrote { Some(out) } else { None }
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
    let text = scrub_inline_formula(ctx, &text, field_name);
    let mut args = Vec::new();
    for a in fields.iter().skip(1) {
        args.push(argument_or_words(ctx, a, field_name)?);
    }
    let applies = segment_gate(ctx, &gates)?;
    let pieces = template_pieces(ctx, &text, &args);
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
    let text = scrub_inline_formula(ctx, &text, field_name);
    let mut vars: Vec<Slot> = Vec::new();
    for a in fields.iter().skip(1) {
        vars.push(argument_or_words(ctx, a, field_name)?);
    }
    let applies = segment_gate(ctx, &gates)?;
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
    let has_slots = pieces.iter().any(|p| matches!(p, ProsePiece::Slot(_)));
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

    /// A parenthesised formula in the prose BODY (no slot converts it) prints as words; an
    /// ordinary parenthetical aside is untouched.
    #[test]
    fn an_inline_formula_in_the_prose_body_prints_as_words() {
        assert_eq!(rewrite_inline_formula("(70+CASTERLEVEL)% chance").as_deref(), Some("(70 plus caster level) percent chance"));
        assert_eq!(rewrite_inline_formula("(70+CASTERLEVEL)%% chance").as_deref(), Some("(70 plus caster level) percent chance"));
        assert_eq!(rewrite_inline_formula("Skill Focus (Knowledge [Arcana])"), None);
        assert_eq!(rewrite_inline_formula("a bonus (see below)"), None);
        assert_eq!(rewrite_inline_formula("(10)"), None);
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
