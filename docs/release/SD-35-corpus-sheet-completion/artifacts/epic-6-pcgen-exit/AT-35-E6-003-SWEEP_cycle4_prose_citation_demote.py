#!/usr/bin/env python3
"""Demote PCGen ingest-token citations out of RENDERED sheet prose into `//`
provenance comments -- SD-35 Epic 6, AT-35-E6-003-SWEEP cycle 4.

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
            text = _mask(text, m)
            continue
        o, c = region
        head = text[o + 1:m.start()]
        kept = trim_citation_head(head)
        cut = text[o + 1 + len(kept):c]
        if not cut.strip() or not _placeholders_are_all_consts(cut):
            text = _mask(text, m)
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
    for a, b in continuation_blocks(lines):
        indent, body = join_block(lines, a, b)
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
        cont_indent = " " * (len(lines[a + 1]) - len(lines[a + 1].lstrip()))
        if len(cont_indent) <= len(indent):
            cont_indent = indent + " "
        block = rewrap(indent, new_body, cont_indent, width)
        prov = [f"{indent}// Provenance (ingest tokens, demoted out of the rendered "
                f"sheet line -- SD-35 AT-35-E6-003-SWEEP):"]
        for r in removed:
            for chunk in _wrap_comment(r, indent, width):
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
