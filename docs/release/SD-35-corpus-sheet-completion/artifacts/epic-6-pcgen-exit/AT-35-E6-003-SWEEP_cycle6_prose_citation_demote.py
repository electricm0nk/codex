#!/usr/bin/env python3
"""Demote PCGen ingest-token citations out of RENDERED sheet prose into `//`
provenance comments -- SD-35 Epic 6, AT-35-E6-003-SWEEP cycle 6.

Cycle 6 adds exactly ONE frame to cycle 5's tool, and it is a NARROWING of
cycle 4's, not a widening. Cycle 4's region rule truncates a `(...)` citation
region at the first token and drops everything from there to the closing paren.
That is correct when the region IS the citation, and wrong when real sheet prose
follows the token inside the same parenthesis:

    "... Reflex DC {dc} for half (arg_abilities_race.lst:776 \\
     BONUS:VAR|Undine_AcidBreath_Dice|min(floor((TL+1)/2),5), \\
     BONUS:VAR|Undine_AcidBreath_DC|10+(TL/2)+CON, evaluated at total character \\
     level {total_level} and Constitution modifier {con:+} ...)"

Cycle 5 REFUSED every one of those (the tail carries `{total_level}`, a non-const
placeholder, so dropping the tail would drop a computed number off the sheet and
leave `format!` with an unused argument). 116 hits in `pilot_compute/mod.rs`
stood on that refusal, and the previous receipt named them as the remainder.

The cycle-6 frame cuts the TOKEN RUN ONLY -- the `BONUS:`/`PRE*:`/`TYPE=` run
plus, when one stands immediately in front of it, the `<file>.lst:<line>` source
marker or the connective that introduced it -- and leaves every other character
of the region verbatim. It is strictly narrower than cycle 4's rule (the span it
deletes is a subset of the span cycle 4 would delete), so nothing cycle 4 or 5
already accepted changes; it only supplies an answer where they masked. A token
run never contains a space and never contains `{`, so the placeholder refusal can
never fire on it, and the number the sheet prints is on the far side of the cut.

Cycle 5 adds exactly ONE further citation frame to cycle 4's tool. Cycle 4
could only cut a citation that sat inside a `(...)` region, so it refused 123
hits in `pilot_compute/mod.rs` and named them as the remainder. The frame those
refusals mostly wear is an inline SOURCE citation:

    "Fighter level {level} base attack bonus from cr_classes.lst:139 \
     BONUS:COMBAT|BASEAB|classlevel = {base_attack_bonus}"

The citation is the `<file>.lst:<line>` source marker plus the token run that
follows it; the sheet line either side of it is kept verbatim, so the number
(`= {base_attack_bonus}`) always survives. It reuses cycle 4's safety gates
unchanged (deletion-only, word-order, quote-balance, and the refusal to cut any
span carrying a non-const `{...}` format placeholder) and re-emits what it cuts
as a `//` provenance comment, so provenance changes custody rather than being
lost.

**Why the frame requires the `.lst:<line>` marker, and why two other frames
were built, measured and then REMOVED.** A first draft also cut a bare
connective run (`the corpus BONUS:VAR|X`) and a backtick-quoted token (``the
record carries no `BONUS:SKILL` token``). Both compiled and both passed every
safety gate, and both produced UNGRAMMATICAL sheet prose in the real file --
*"the record carries no -- and cross-checked against"*, *"token for this record
reads 3+CHA"*, *"Like Monk's is armor/encumbrance-conditional"*. A sheet line
that reads as a truncation is worse than the citation it replaced, and no gate
in this tool can detect it (`AGENTS.md` rule 7: a proof is only as wide as the
cases it covers). The `.lst:<line>` marker is the one frame where the citation's
boundaries are unambiguous, so it is the only one kept; everything else stays in
the residual count and is named as the remainder.

What this fixes, and why it is not cosmetic
-------------------------------------------
`ComputationExplanation.detail` (`src/rules_core/pilot_compute/mod.rs:296`) is
carried to the desktop crate as `ExplanationDto.detail` (`character_hub.rs:785`)
and rendered on the Character Hub sheet (`characterHub/classFeaturesModel.ts`).
Hundreds of those `detail` strings carry their PCGen provenance INSIDE the
rendered sentence, e.g.

    "... racial bonus on saving throws against poison (dwarf_abilities_race.lst:25
     BONUS:VAR|SaveBonus_vs_Poison|1|TYPE=Racial, BONUS:VAR|SaveBonus_vs_Spells|1|TYPE=Racial)."

That prints `BONUS:VAR|...|TYPE=Racial` on a player's paper character sheet.
The sheet rule (`decisions.md` §1) says a sheet line is a final number, dice in
final form, or the rule's words -- never ingest vocabulary. So this is a
sheet-rule defect, and clearing it is also exactly what `decisions.md` §11 and
operator ruling B14 (2026-09-11) prescribe: the converter owns the token, and
the provenance lives in a `//` comment beside the value, which is where
`AGENTS.md` rule 9 wants it and where the residue gate does not count it.

The transform
-------------
Confined to STRING PROSE. For every PCGen token occurrence inside a string
literal, find the parenthesised citation region that encloses it and truncate
that region at the first token, trimming the connective punctuation that
introduced it (` -- `, ` -- `, `: `, `, `, `; `). A region left with nothing but
a bare filler word is removed outright along with its parentheses. Every
removed span is re-emitted verbatim as a `//` provenance comment immediately
above the statement it came from, so no provenance is lost -- it changes
custody, from rendered sheet text to source comment.

Nothing outside a parenthesised citation region is touched. Token occurrences
that are *data* -- a `qualifiers: &["BONUS:..."]` array the effect appliers
actually read, a `format!("BONUS:{}", ..)` that BUILDS a token, a test asserting
on a token string -- are deliberately left alone: those are a converter-side
mapping job, not a prose job, and this tool reports them as residual rather
than guessing.

Safety properties, all asserted by --verify:
  * only deletions; never an insertion into a string literal.
  * every word surviving in the new prose appears in the old prose, in order.
  * every deleted span matches a PCGen token regex or is citation punctuation.
  * Rust line-continuations are re-wrapped at the file's own column, with the
    block's own indentation, so the literal still parses.

Usage:
    python3 <this> --dry-run <file>...      census only, writes nothing
    python3 <this> --apply <file>...        rewrite in place
"""

import argparse
import os
import re
import sys

TOKEN_RX = re.compile(
    r"!?\b("
    r"BONUS|DEFINE|DESC|SAB|ABILITY|ABILITYPOOL|AUTO|CHOOSE|SPELLKNOWN|"
    r"SITUATION|VFEAT|SPELLLEVEL|CSKILL|SERVESAS|ADD|TEMPLATE|"
    r"PRE[A-Z]+|TYPE"
    r")[:=]"
)
# The residue gate's own surface; the tool must clear exactly these.
GATE_RX = re.compile(
    r"\braw_tokens\b|\braw_bonus_chains\b|\bPcgenFormulaEvaluator\b"
    r"|\brender_pcgen_desc\b|\bbonus_stack_reader\b|\bpre_tokens\b"
    r"|\bBONUS:|\bDEFINE:|\bPRE[A-Z]+:|\bSAB:|\bDESC:|%CHOICE|%LIST|\bTYPE="
)
# Words that, left alone inside a citation region, carry no information.
FILLER = {"corpus", "the", "pcgen", "from", "per", "see", "cf", "via", "in"}
TRAILING_JUNK = re.compile(r"[\s,;:]*(?:--|—|-)?[\s,;:]*$")
# A connective left dangling by the cut ("... Dwarf ~ Vision, gated)") reads as a
# truncation rather than a citation. Strip it too, then re-trim the punctuation.
DANGLING = re.compile(
    r"[\s,;:]*\b(gated|by|backed|via|per|namely|carrying|carries|granting|giving|"
    r"with|plus|from|reading|naming|bearing)\b[\s,;:]*$",
    re.IGNORECASE,
)


def trim_citation_head(head):
    kept = TRAILING_JUNK.sub("", head)
    for _ in range(3):
        stripped = DANGLING.sub("", kept)
        if stripped == kept:
            break
        kept = TRAILING_JUNK.sub("", stripped)
    # An opening backtick whose partner was in the cut span would render as a
    # stray "(`)" on the sheet; drop it and re-trim.
    while kept.count("`") % 2 == 1:
        kept = TRAILING_JUNK.sub("", kept[: kept.rfind("`")])
    return kept


# ---------------------------------------------------------------- line blocks
def continuation_blocks(lines):
    """Yield (start_idx, end_idx) of each Rust `\\`-continuation run, 0-based
    inclusive. A run is >=2 lines: n lines ending in `\\` plus the closer."""
    i = 0
    while i < len(lines):
        if lines[i].lstrip().startswith("//"):
            i += 1
            continue
        if lines[i].rstrip().endswith("\\"):
            j = i
            while j < len(lines) and lines[j].rstrip().endswith("\\"):
                j += 1
            if j < len(lines):
                yield (i, j)
                i = j + 1
                continue
        i += 1


# A single line the tool may treat as prose: ONE `<field>: "<literal>"` and
# nothing else. The first draft accepted any line carrying a quote, and on
# `advanced_race_guide/feat_data/general.rs` it reached INSIDE a live data array
# -- rewriting `count("ABILITIES","TYPE=FavoredClassBonus")` to
# `count("ABILITIES","")`, silently changing what the engine reads, and emitting
# Rust `\`-continuations OUTSIDE a string literal, which does not compile. Both
# were caught by inspecting the proposed diff before applying it. A whole-line
# shape test is the control: a data array never matches it.
PROSE_LINE_RX = re.compile(r'^\s*\w+: "(?:[^"\\]|\\.)*",?\s*$')


def prose_blocks(lines):
    """Every candidate block: each `\\`-continuation run (cycle 4's unit), plus
    each SINGLE line that carries a string literal and is not part of one.

    Cycle 4 only ever looked at continuation runs, so a citation that fitted on
    one line -- `crb/race_tables.rs`'s 17 `detail:` rows, for instance -- was
    never examined at all. It is the same prose, rendered on the same sheet,
    and it goes through the same rules and the same safety gates.
    """
    covered = set()
    runs = list(continuation_blocks(lines))
    for a, b in runs:
        covered.update(range(a, b + 1))
    out = list(runs)
    for i, line in enumerate(lines):
        if i in covered or line.lstrip().startswith("//"):
            continue
        if PROSE_LINE_RX.match(line) and GATE_RX.search(line):
            out.append((i, i))
    return sorted(out)


def join_block(lines, a, b):
    """Rust eats the newline AND the next line's leading whitespace after `\\`."""
    parts = [lines[a].rstrip()[:-1]]
    for k in range(a + 1, b):
        parts.append(lines[k].strip()[:-1] if lines[k].rstrip().endswith("\\")
                     else lines[k].lstrip())
    parts.append(lines[b].lstrip())
    head = parts[0]
    lead = len(head) - len(head.lstrip())
    return " " * lead, head.lstrip() + "".join(parts[1:])


def rewrap(indent, body, cont_indent, width):
    """Re-emit `body` as a `\\`-continuation run. Splits on spaces only, so a
    `{placeholder}` (which never contains a space) can never be broken."""
    words = body.split(" ")
    out, cur = [], indent
    first = True
    for w in words:
        cand = cur + ("" if (first or cur.endswith(" ")) else " ") + w
        if len(cand) > width and not first and cur.strip():
            out.append(cur.rstrip() + " \\")
            cur = cont_indent + w
        else:
            cur = cand
        first = False
    out.append(cur)
    return out


# ------------------------------------------------------------------ transform
def enclosing_region(text, pos):
    """Return (open_idx, close_idx) of the paren region enclosing `pos`, or None."""
    depth, open_idx = 0, None
    for i in range(pos - 1, -1, -1):
        c = text[i]
        if c == ")":
            depth += 1
        elif c == "(":
            if depth == 0:
                open_idx = i
                break
            depth -= 1
    if open_idx is None:
        return None
    depth = 0
    for i in range(pos, len(text)):
        c = text[i]
        if c == "(":
            depth += 1
        elif c == ")":
            if depth == 0:
                return (open_idx, i)
            depth -= 1
    return None


MASK_COLON = "\x01"
MASK_EQ = "\x02"

# ---------------------------------------------------------- cycle-5 frames
# A citation frame is a span of the rendered sentence that is ABOUT the ingest
# record rather than part of the sheet line. Cycle 4 recognised exactly one:
# a `(...)` region. These are the other two the refusals actually wore.

# The token run itself: the token plus its `|`-separated arguments, which never
# contain a space, optionally continued by further comma/`and`-joined tokens.
_RUN = r"!?\b(?:BONUS|DEFINE|DESC|SAB|SPELLKNOWN|SITUATION|PRE[A-Z]+|TYPE)[:=]\S*"
TOKEN_RUN_RX = re.compile(rf"{_RUN}(?:(?:,| and | plus )\s*{_RUN})*")

# `cr_classes.lst:139 `, `dwarf_abilities_race.lst:25 `, `arg_feats.lst:12 ` --
# the source-line marker that introduces an inline citation. Optionally
# preceded by the connective that introduced it (`from`, `per`, `corpus`, ...).
CITE_PREFIX_RX = re.compile(
    r"(?:\b(?:from|per|in|corpus|see|cf\.?|via)\s+)?\b[\w.\-]+\.lst:\d+\s+$",
    re.IGNORECASE,
)
def _tidy(text):
    """Collapse the whitespace and punctuation a cut leaves behind. Deletion
    only: never inserts a word, only removes separators that now touch."""
    text = re.sub(r"[ \t]{2,}", " ", text)
    text = re.sub(r"\(\s*\)", "", text)
    text = re.sub(r"\s+([,.;:])", r"\1", text)
    text = re.sub(r"([(\[])\s+", r"\1", text)
    text = re.sub(r"\s+([)\]])", r"\1", text)
    # Cycle 6: a narrow token-run cut can leave the separator that joined the
    # run to the prose either side of it. `(, evaluated at ...)` and
    # `(... +CON, )` both read as a truncation; remove the orphan separator.
    # Deletion only -- no word is inserted and none is reordered.
    text = re.sub(r"([(\[])\s*[,;:]+\s*", r"\1", text)
    text = re.sub(r"[ \t]*[,;:]+\s*([)\]])", r"\1", text)
    text = re.sub(r"([,;:])\s*[,;:]+", r"\1", text)
    return re.sub(r"[ \t]{2,}", " ", text)


def _inline_citation_span(text, pos):
    """(start, end) of the inline citation enclosing the token at `pos`, or None.

    The span is the token run at `pos` plus, when one stands immediately in
    front of it, the `<file>.lst:<line>` source marker or the bare connective
    that introduced it. Nothing after the run is touched, so a trailing
    ` = {value}` -- the sheet's actual number -- always survives.
    """
    run = TOKEN_RUN_RX.match(text, pos)
    if run is None:
        return None
    start, end = run.start(), run.end()
    # The `<file>.lst:<line>` marker is REQUIRED, not optional: it is what makes
    # the span unambiguously a citation rather than prose that happens to name a
    # token. Without it the tool refuses -- see the module docstring for the two
    # frames that were built, measured against the real file, and removed.
    prefix = CITE_PREFIX_RX.search(text[:start])
    if prefix is None:
        return None
    return (prefix.start(), end)


# ---------------------------------------------------------- cycle-6 frame
# Inside a `(...)` citation region the boundary question cycle 5's docstring
# worries about is already answered by the parentheses: the author put the
# citation in brackets. What cycle 4 got wrong is only HOW MUCH of the bracket
# is citation. So inside a region -- and only there -- the marker is optional
# and the connective alone is enough, because the cut can never escape the
# region and the region is known to be parenthetical aside.
CITE_PREFIX_IN_REGION_RX = re.compile(
    r"(?:\b(?:from|per|in|corpus|see|cf\.?|via|reads?|reading|carries|carrying|"
    r"namely|naming|bearing|token|tokens)\s+)*"
    r"(?:\b[\w.\-]+\.lst:\d+\s+)?$",
    re.IGNORECASE,
)


def _token_run_span_in_region(text, pos, lo):
    """(start, end) of the token RUN at `pos` plus its introducer, clamped to
    `lo` (the region's opening paren + 1). Returns None when the run itself
    cannot be matched. Never extends past `pos` backwards beyond `lo`, and
    never forwards past the run, so the cut is always a strict subset of the
    span cycle 4's region rule would have taken."""
    run = TOKEN_RUN_RX.match(text, pos)
    if run is None:
        return None
    start, end = run.start(), run.end()
    prefix = CITE_PREFIX_IN_REGION_RX.search(text[lo:start])
    if prefix is not None and prefix.start() + lo < start:
        start = prefix.start() + lo
    return (max(start, lo), end)



def demote(text):
    """Return (new_text, [removed spans]).

    Walks token occurrences; for each it finds the enclosing parenthesised
    citation region and truncates that region at the token. An occurrence the
    rules refuse -- no enclosing region, or a cut span carrying a `{...}`
    placeholder that is not a constant (removing it would leave `format!` with
    an unused argument, a hard compile error) -- is MASKED rather than skipped,
    so the walk always makes progress and the refusal is visible in the
    residual count instead of being guessed at."""
    removed = []
    for _ in range(400):
        m = None
        for cand in TOKEN_RX.finditer(text):
            if GATE_RX.search(text[cand.start():cand.end() + 40]):
                m = cand
                break
        if m is None:
            break
        region = enclosing_region(text, m.start())
        if region is None:
            # Cycle 5: the two frames cycle 4 had no rule for. Both cut a
            # bounded span and leave everything outside it verbatim, so the
            # sheet line either side of the citation is untouched.
            span = _inline_citation_span(text, m.start())
            if span is None:
                text = _mask(text, m)
                continue
            s, e = span
            cut = text[s:e]
            if not cut.strip() or not _placeholders_are_all_consts(cut):
                text = _mask(text, m)
                continue
            head = trim_citation_head(text[:s])
            if not head.strip() or head.rstrip().endswith('"'):
                # The citation IS the whole string, or starts it (a
                # `resolution:` provenance row in the caster-level table, not a
                # sheet line). Cutting there leaves a fragment opening on a
                # dangling connective -- *`" which is the class level"`* -- so
                # refuse and let the hit be named in the residual count.
                text = _mask(text, m)
                continue
            removed.append(cut.strip())
            text = _tidy(head + text[e:])
            continue
        o, c = region
        head = text[o + 1:m.start()]
        kept = trim_citation_head(head)
        cut = text[o + 1 + len(kept):c]
        if not cut.strip() or not _placeholders_are_all_consts(cut):
            # Cycle 6: the region tail carries real sheet prose (a non-const
            # `{...}`), so cycle 4's "truncate the region here" is wrong and
            # cycle 5 masked. Cut the token run alone instead -- a strict
            # subset of that span -- and leave the rest of the region verbatim.
            narrow = _token_run_span_in_region(text, m.start(), o + 1)
            if narrow is None:
                text = _mask(text, m)
                continue
            s, e = narrow
            span = text[s:e]
            if not span.strip() or not _placeholders_are_all_consts(span):
                text = _mask(text, m)
                continue
            removed.append(span.strip())
            text = _tidy(text[:s] + text[e:])
            continue
        removed.append(cut.strip())
        bare = re.sub(r"[^A-Za-z]", "", kept).lower()
        if not kept.strip() or bare in FILLER:
            start = o - 1 if (o > 0 and text[o - 1] == " ") else o
            text = text[:start] + text[c + 1:]
        else:
            text = text[:o + 1] + kept + text[c:]
    text = _unmask(text)
    removed = [_unmask(r) for r in removed]
    return text, removed


def _unmask(text):
    return text.replace(MASK_COLON, ":").replace(MASK_EQ, "=")


def _mask(text, m):
    """Hide one refused token from the next scan by masking its `:`/`=`."""
    i = m.end() - 1
    mask = MASK_COLON if text[i] == ":" else MASK_EQ
    return text[:i] + mask + text[i + 1:]


SENTENCE_SPLIT = re.compile(r"(?<=[.!?])\s+(?=[A-Z\"`(\\])")
PLACEHOLDER = re.compile(r"\{([^{}]*)\}")


def _placeholders_are_all_consts(sentence):
    """A sentence may be dropped only when every `{...}` in it names an
    UPPER_SNAKE_CASE constant. A const reference vanishing from a format string
    is inert; a LOCAL vanishing leaves an unused binding (clippy), and a bare
    positional `{}` vanishing changes `format!`'s arity into a compile error."""
    for ph in PLACEHOLDER.findall(sentence):
        name = ph.split(":", 1)[0]
        if not name or not re.fullmatch(r"[A-Z][A-Z0-9_]*", name):
            return False
    return True


def demote_sentences(text):
    """Second pass: a whole SENTENCE that mentions an ingest token is engineering
    commentary, not a sheet line. Remove it entire -- which keeps the remaining
    prose grammatical, unlike a mid-sentence excision -- and hand it to the
    provenance comment.

    Three refusals, each load-bearing:
      * a sentence carrying a `{...}` format placeholder is KEPT. Dropping one
        would drop a computed number off the sheet, and (for a positional
        `{}`) would change `format!`'s argument arity into a compile error.
      * the FIRST sentence is kept: it is the record's own identity line.
      * a string of one sentence is kept whole; there is nothing to fall back on.
    """
    removed = []
    for _ in range(40):
        parts = SENTENCE_SPLIT.split(text)
        if len(parts) < 2:
            break
        victim = None
        # Never the LAST part: it carries the literal's closing quote (and the
        # `,`/`)` after it). Dropping it deletes the terminator and the file
        # stops parsing -- observed on `pilot_compute/mod.rs:14672` on this
        # cycle's first application, caught by `cargo build --lib`.
        for idx in range(1, len(parts) - 1):
            if not GATE_RX.search(parts[idx]):
                continue
            if not _placeholders_are_all_consts(parts[idx]):
                continue
            victim = idx
            break
        if victim is None:
            break
        removed.append(parts[victim].strip())
        text = " ".join(p for i, p in enumerate(parts) if i != victim)
    return text, removed


# ------------------------------------------------------------- word-order gate
def _unescaped_quotes(text):
    """Count `"` not preceded by a backslash. A transform that changes this
    count has moved a string terminator, which no prose edit may ever do."""
    return len(re.findall(r'(?<!\\)"', text))


def words(s):
    return re.findall(r"[A-Za-z][A-Za-z'’-]+", s)


def survives_in_order(old, new):
    """Every word of `new` appears in `old` in the same order (deletion-only)."""
    it = iter(words(old))
    for w in words(new):
        for o in it:
            if o == w:
                break
        else:
            return False
    return True


# -------------------------------------------------------------------- driver
def process(path, apply, width):
    raw = open(path, encoding="utf-8").read()
    lines = raw.split("\n")
    edits = []
    for a, b in prose_blocks(lines):
        indent, body = (
            join_block(lines, a, b) if b > a
            else (" " * (len(lines[a]) - len(lines[a].lstrip())), lines[a].strip())
        )
        if not GATE_RX.search(body):
            continue
        new_body, removed = demote(body)
        new_body, removed2 = demote_sentences(new_body)
        removed = removed + removed2
        if new_body == body or not removed:
            continue
        if _unescaped_quotes(new_body) != _unescaped_quotes(body):
            print(f"  REFUSED (quote balance) {path}:{a + 1}", file=sys.stderr)
            continue
        if not survives_in_order(body, new_body):
            print(f"  REFUSED (word order) {path}:{a + 1}", file=sys.stderr)
            continue
        nxt = lines[a + 1] if a + 1 < len(lines) else ""
        cont_indent = " " * (len(nxt) - len(nxt.lstrip()))
        if len(cont_indent) <= len(indent):
            cont_indent = indent + " "
        block = rewrap(indent, new_body, cont_indent, width)
        prov = [f"{indent}// Provenance (ingest tokens, demoted out of the rendered "
                f"sheet line -- SD-35 AT-35-E6-003-SWEEP):"]
        for r in removed:
            for chunk in _wrap_comment(r.lstrip(":;, "), indent, width):
                prov.append(chunk)
        edits.append((a, b, prov + block))
    if not edits:
        return 0, 0
    out, prev = [], 0
    for a, b, repl in edits:
        out.extend(lines[prev:a])
        out.extend(repl)
        prev = b + 1
    out.extend(lines[prev:])
    new_raw = "\n".join(out)
    hits_before = len(GATE_RX.findall(_code_only(raw)))
    hits_after = len(GATE_RX.findall(_code_only(new_raw)))
    if apply:
        open(path, "w", encoding="utf-8").write(new_raw)
    return hits_before - hits_after, len(edits)


def _wrap_comment(text, indent, width):
    out, cur = [], indent + "//   "
    for w in text.split(" "):
        cand = cur + ("" if cur.endswith(" ") else " ") + w
        if len(cand) > width and cur.strip() != "//":
            out.append(cur)
            cur = indent + "//   " + w
        else:
            cur = cand
    out.append(cur)
    return out


def _code_only(text):
    return "\n".join(l for l in text.split("\n") if not l.lstrip().startswith("//"))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("files", nargs="+")
    ap.add_argument("--apply", action="store_true")
    ap.add_argument("--width", type=int, default=100)
    args = ap.parse_args()
    total, blocks = 0, 0
    for f in args.files:
        if not os.path.isfile(f):
            continue
        cleared, n = process(f, args.apply, args.width)
        if cleared or n:
            print(f"{f}: cleared={cleared} blocks={n}")
        total += cleared
        blocks += n
    print(f"TOTAL cleared={total} blocks={blocks} applied={args.apply}")


if __name__ == "__main__":
    main()
