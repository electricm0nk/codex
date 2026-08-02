#!/usr/bin/env python3
"""Reference determinator for `wiring_class` (GE-01 artifact, NOT production code).

This file is a documentary artifact of the GE-01 STC package. It exists so that
every figure in `wiring-class-determination.md` can be reproduced by a reader,
and so that a future implementer of the real determinator (which belongs in the
work-inventory generator, not here) has an executable statement of the rules
rather than prose alone.

It is deliberately dependency-free and reads only:
  - docs/work-inventory.json  (the generator's own per-unit output)
  - the PCGen corpus tree     (for the raw `.lst` line behind each unit)

Usage, from the repo root:
    python3 docs/release/GE-01-legacy-corpus-and-conversion-matrix/artifacts/wiring-class-determination.py HELD
    python3 docs/release/GE-01-legacy-corpus-and-conversion-matrix/artifacts/wiring-class-determination.py ingested-magnitude

`HELD` means the five statuses under which the engine holds a record:
`grounded`, `ingested-magnitude`, `text-complete`, `deferred-with-reason`,
`unknown`. Any other argument is treated as a single literal status.
"""

import collections
import json
import os
import re
import sys

CORPUS = os.environ.get(
    "CODEX_CORPUS_ROOT",
    "/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game",
)
INVENTORY = os.environ.get("CODEX_WORK_INVENTORY", "docs/work-inventory.json")

# Tab-field prefixes that carry a real numeric magnitude. Kept byte-identical to
# `MAGNITUDE_TOKENS` in `src/bin/v06_work_inventory.rs`; the determinator must
# never fork that list, only classify what it already selects.
MAGNITUDE_TOKENS = (
    "BONUS:", "TEMPBONUS:", "DEFINE:", "COST:", "WT:", "CR:", "AC:", "ACCHECK:",
    "DAMAGE:", "CRITMULT:", "CRITRANGE:", "RANGE:", "REACH:", "MOVE:", "HITDIE:",
    "LEVELADJUSTMENT:", "SR:", "DR:", "SPELLFAILURE:", "STAT:",
)

# Fields whose value is prose but which PCGen allows to carry a parenthesised
# expression that the renderer substitutes, e.g. `(min(10,CASTERLEVEL))d6`.
PROSE_FIELDS = ("DESC:", "DURATION:", "TARGETAREA:", "SPROP:", "RANGE:", "SPECIALS:")

# Character/item scalars a magnitude may be a function of.
SCALARS = re.compile(
    r"CASTERLEVEL|CLASSLEVEL|TOTALLEVELS|TOTALLEVEL|\bBAB\b|\bHD\b|PLUSTOTAL"
    r"|SPELLLEVEL|\bSTR\b|\bDEX\b|\bCON\b|\bINT\b|\bWIS\b|\bCHA\b|\bTL\b|\bCL\b"
    r"|\bRACESIZE\b"
)
ARITH = re.compile(r"[*/]|\+\s*\w*[A-Z]{2,}|MIN\(|MAX\(|min\(|max\(")
PAREN = re.compile(r"\(([^()]*(?:\([^()]*\)[^()]*)*)\)")

# A conditional guard. `PRERULE` is excluded on purpose: `!PRERULE:1,DisplayFullSpell`
# is a renderer directive present on ~every spell row and is not a rules guard.
GUARD = re.compile(r"(^|\|)!?PRE(?!RULE)[A-Z]+:")

# PCGen keyword ranges whose real value is a function of caster level:
#   Close  = 25 ft + 5 ft per 2 caster levels
#   Medium = 100 ft + 10 ft per caster level
#   Long   = 400 ft + 40 ft per caster level
RANGE_KEYWORDS = ("Close", "Medium", "Long")

# Scaling stated in English prose with no machine-readable expression. NOT a
# class; a determination failure that must be surfaced.
PROSE_SCALING = re.compile(
    r"per (caster )?level|per \d+ (caster )?levels?|x your caster level"
    r"|times your caster level|per two levels|per three levels|per four levels"
    r"|per five levels|every \d+ levels|caster level \(max",
    re.I,
)

_lines = {}


def corpus_line(book, filename, lineno):
    path = os.path.join(CORPUS, book, filename)
    if path not in _lines:
        try:
            with open(path, encoding="utf-8", errors="replace") as fh:
                _lines[path] = fh.read().split("\n")
        except OSError:
            _lines[path] = []
    buf = _lines[path]
    return buf[lineno - 1] if 0 < lineno <= len(buf) else None


def signals(raw):
    """Return the SET of signals a corpus row carries. A row may carry several."""
    if raw is None:
        return {"no_corpus_line"}
    fields = [f.strip() for f in raw.split("\t") if f.strip()]
    mags = [f for f in fields if f.startswith(MAGNITUDE_TOKENS)]
    out = set()

    # A computed signal only matters if there is a magnitude for it to govern.
    # A guard or a choice on a row that carries no magnitude token gates TEXT,
    # and text is `display` work under the operator's standing ruling.
    # `TEMPBONUS:` is itself a magnitude token, so it can never be missed here.
    if mags:
        for f in fields:
            if f.startswith("TEMPBONUS:"):
                out.add("computed:tempbonus")
            if "%CHOICE" in f or f.startswith("CHOOSE:"):
                out.add("computed:choice")
            if GUARD.search(f):
                out.add("computed:pre_guard")

    for f in mags:
        token, _, value = f.partition(":")
        if token == "RANGE" and value.strip() in RANGE_KEYWORDS:
            out.add("derived:range_keyword")
        if SCALARS.search(value) or ARITH.search(value):
            out.add("derived:%s" % token.lower())

    for f in fields:
        if f.startswith(PROSE_FIELDS):
            for m in PAREN.finditer(f):
                if SCALARS.search(m.group(1)):
                    out.add("derived:prose_expr")
            if PROSE_SCALING.search(f):
                out.add("ambiguous:prose_scaling_phrase")

    if not mags:
        out.add("display:no_magnitude_token")
    elif not any(s.startswith(("computed:", "derived:")) for s in out):
        out.add("static:literal_magnitudes_only")
    return out


def wiring_class(sigs):
    """Collapse a signal set to one class. Strictly highest-bar-wins."""
    if sigs == {"no_corpus_line"}:
        return "ambiguous", "no_corpus_line"
    if any(s.startswith("display:") for s in sigs):
        return "display", "no_magnitude_token"
    for prefix in ("computed:", "derived:"):
        hit = sorted(s for s in sigs if s.startswith(prefix))
        if hit:
            return prefix[:-1], hit[0].split(":", 1)[1]
    hit = sorted(s for s in sigs if s.startswith("ambiguous:"))
    if hit:
        return "ambiguous", hit[0].split(":", 1)[1]
    if any(s.startswith("display:") for s in sigs):
        return "display", "no_magnitude_token"
    return "static", "literal_magnitudes_only"


HELD = ("ingested-magnitude", "text-complete", "grounded", "deferred-with-reason", "unknown")


def main():
    scope = sys.argv[1] if len(sys.argv) > 1 else "HELD"
    doc = json.load(open(INVENTORY))
    units = doc["units"]
    sel = [u for u in units
           if (u["status"] in HELD if scope == "HELD" else u["status"] == scope)]

    per_class = collections.Counter()
    per_reason = collections.Counter()
    per_book = collections.defaultdict(collections.Counter)
    dual = 0
    for u in sel:
        sigs = signals(corpus_line(u["book"], u["source_file"], u["source_line"]))
        cls, why = wiring_class(sigs)
        per_class[cls] += 1
        per_reason[(cls, why)] += 1
        per_book[u["book"]][cls] += 1
        if (any(s.startswith("computed:") for s in sigs)
                and any(s.startswith("derived:") for s in sigs)):
            dual += 1

    print("inventory %s generated_at %s" % (INVENTORY, doc["generated_at"]))
    print("scope %s  n=%d" % (scope, len(sel)))
    for k, v in per_class.most_common():
        print("  %-10s %6d  %5.1f%%" % (k, v, 100.0 * v / max(len(sel), 1)))
    print("  dual-signal (derived AND computed) %d" % dual)
    print("reasons:")
    for k, v in per_reason.most_common():
        print("   %-28s %6d" % ("%s/%s" % k, v))
    print("per book:")
    hdr = ("book", "held", "display", "static", "derived", "computed", "ambiguous")
    print("   %-26s%7s%9s%8s%9s%10s%10s" % hdr)
    for b, c in sorted(per_book.items(), key=lambda kv: -sum(kv[1].values())):
        print("   %-26s%7d%9d%8d%9d%10d%10d" % (
            b, sum(c.values()), c["display"], c["static"],
            c["derived"], c["computed"], c["ambiguous"]))


if __name__ == "__main__":
    main()
