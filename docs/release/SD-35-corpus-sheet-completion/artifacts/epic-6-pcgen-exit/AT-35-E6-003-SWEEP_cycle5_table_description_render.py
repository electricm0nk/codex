#!/usr/bin/env python3
"""Render the ingest vocabulary out of a shipped `description:` table literal --
SD-35 Epic 6, AT-35-E6-003-SWEEP cycle 5.

What this fixes
---------------
`src/rules_core/rules_tables/**`'s `description: Some("...")` strings are
served straight onto the player's sheet: the equipment tables through
`equipment_resolver::equipment_catalog_rows()` and the desktop equipment
catalog, the archetype tables through `ArchetypeGrant.description`. Twenty-odd
of them still carried their PCGen ingest tail verbatim, e.g.

    "The martial master can use a move action to gain the benefit of a combat
     feat he doesn't possess. ...|PREVARLT:FighterLVL,9"
    "+2 enhancement bonus and DR 2/- against %CHOICE"

so `|PREVARLT:FighterLVL,9` and `%CHOICE` printed on a paper character sheet.
That is the sheet rule (`decisions.md` §1) broken in the same place cycle 4
found it in `ComputationExplanation.detail` and this cycle found it in the
generated `equipment_gap_tables.rs`, and the fix is the same one
`epic-breakdown.md` AT-35-E6-003 states: the substitution happens on the
converter side, and what ships is the rendered sentence.

`equipment_gap_tables.rs` is generated, so its fix went into
`src/bin/gen_equipment_gap_tables.rs::safe_description` and the table was
regenerated. The tables this tool touches are hand-authored or were generated
by one-time scripts that no longer exist, so the same two rules are applied to
their literals directly, once.

The two rules, and the one refusal
----------------------------------
1. **Drop a trailing `|PRE…` display gate.** A `|PRE*`/`|!PRE*` tail on a
   `DESC:`/`SPROP:` token is PCGen's *display* condition — it decides whether
   PCGen shows the sentence, and is never part of the sentence. Cycle 4 set
   this precedent exactly, stripping 101 `|PRERULE:1,DisplayFullSpell`
   qualifiers out of `ultimate_intrigue/spell_list.rs`.
2. **Drop a `%<KEYWORD>` reference** (`%CHOICE`, `%LIST`), collapsing the
   whitespace it leaves and any connective it was the object of. This is
   exactly what `rules_core::pcgen_desc::render_pcgen_desc` does — pinned by
   its own `a_percent_keyword_at_the_end_of_the_sentence_leaves_no_trailing_leak`
   and `a_different_percent_keyword_is_also_dropped` tests — reproduced here so
   the shipped literal carries the rendered text rather than the raw one.

**REFUSED: a literal carrying a numbered `%N` reference.** `%1` names a
variable whose value comes from the character, and dropping it would delete a
number off the sheet ("a +%1 enhancement bonus" -> "a + enhancement bonus").
Those literals are left exactly as they are and stay in the residue gate's
count, named as the remainder. Three literals are refused on this rule.

Deletion-only, and verified as such: every word of the new text appears in the
old text in the same order, and the tool never writes a literal that still
contains ingest vocabulary (it refuses the file instead).

Usage:
    python3 <this> <file>...            dry run, prints every proposed change
    python3 <this> --apply <file>...    rewrite in place
"""

import argparse
import os
import re
import sys

DESC_RX = re.compile(r'description: Some\("((?:[^"\\]|\\.)*)"\)')
# One `|PRE…` / `|!PRE…` display gate. It ends at the next `;` or `|`, because
# a multi-variant `SPROP:` states each variant with its OWN gate --
# `"...3 setting stones|PREVARLT:HeartstaffLVL,5; ...5 setting stones|PREVARGTEQ:
# HeartstaffLVL,5"` -- and a greedy tail would swallow the second variant whole.
# It was greedy in this tool's first draft and did exactly that, on Heartstaff
# and Crown of Chaos; the `;` boundary is what stops it. Bracketed groups
# (`PREMULT:1,[PREVARGT:…],[PREVARGT:…]`) are consumed as units.
PRE_TAIL_RX = re.compile(r"\|!?PRE[A-Z]+:(?:\[[^\]]*\]|[^|;])*")
NUMBERED_RX = re.compile(r"%\d")
KEYWORD_RX = re.compile(r"%[A-Z]+")
GATE_RX = re.compile(
    r"\bBONUS:|\bDEFINE:|\bPRE[A-Z]+:|\bSAB:|\bDESC:|%CHOICE|%LIST|\bTYPE="
)
CONNECTIVES = (
    "of", "to", "against", "with", "by", "for", "in", "on", "from", "than", "and", "or",
)


def words(s):
    return re.findall(r"[A-Za-z][A-Za-z'’-]+", s)


def survives_in_order(old, new):
    it = iter(words(old))
    for w in words(new):
        for o in it:
            if o == w:
                break
        else:
            return False
    return True


def trim_dangling_connective(text):
    """The mirror of `gen_equipment_gap_tables.rs::trim_dangling_connective`: a
    dropped keyword must not leave the sentence ending on the preposition that
    introduced it."""
    kept = text
    while True:
        probe = kept.rstrip(" ,;:")
        last = probe.rsplit(" ", 1)[-1].lower() if " " in probe else ""
        if last not in CONNECTIVES:
            break
        kept = probe[: probe.rfind(" ")].rstrip()
    return kept


def render(raw):
    """Return the rendered literal, or None when the tool refuses it."""
    if NUMBERED_RX.search(raw):
        return None  # a `%N` names a character value; dropping it drops a number
    out = PRE_TAIL_RX.sub("", raw)
    if KEYWORD_RX.search(out):
        out = KEYWORD_RX.sub("", out)
        out = re.sub(r"[ \t]{2,}", " ", out).strip()
        out = trim_dangling_connective(out)
    out = re.sub(r"\s+([,.;:])", r"\1", out).strip()
    out = re.sub(r"[ \t]{2,}", " ", out)
    if GATE_RX.search(out) or not out:
        return None
    if not survives_in_order(raw, out):
        return None
    return out


def process(path, apply):
    raw = open(path, encoding="utf-8").read()
    changed, refused = 0, 0
    out_parts, pos = [], 0
    for m in DESC_RX.finditer(raw):
        body = m.group(1)
        if not GATE_RX.search(body):
            continue
        new = render(body)
        if new is None:
            refused += 1
            print(f"  REFUSED {path}: {body[:90]!r}")
            continue
        out_parts.append(raw[pos:m.start(1)])
        out_parts.append(new)
        pos = m.end(1)
        changed += 1
        print(f"  {path}\n    -  {body[:150]}\n    +  {new[:150]}")
    out_parts.append(raw[pos:])
    if changed and apply:
        open(path, "w", encoding="utf-8").write("".join(out_parts))
    return changed, refused


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("files", nargs="+")
    ap.add_argument("--apply", action="store_true")
    args = ap.parse_args()
    total, refused = 0, 0
    for f in args.files:
        if not os.path.isfile(f):
            continue
        c, r = process(f, args.apply)
        total += c
        refused += r
    print(f"TOTAL rendered={total} refused={refused} applied={args.apply}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
