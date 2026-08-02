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
    python3 docs/release/GE-01-legacy-corpus-and-conversion-matrix/artifacts/wiring-class-determination.py --selftest

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
# `BENEFIT:` is here because 2,087 corpus rows carry a record's mechanical
# benefit text in it and it appears in no magnitude-token list.
PROSE_FIELDS = ("DESC:", "DURATION:", "TARGETAREA:", "SPROP:", "RANGE:",
                "SPECIALS:", "BENEFIT:")

# Upstream's own admission that a record is not mechanically implemented in
# PCGen. Reported separately; MUST NOT feed `wiring_class` in either direction.
UPSTREAM_NOT_IMPLEMENTED = "[Not Implemented]"

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
    r"|per five levels|every \d+ levels|caster level \(max"
    # Added 2026-08-02 from the ultimate_campaign story feats, whose magnitudes
    # are stated in English on a `.MOD BENEFIT:` row: "spell resistance equal to
    # 5 + your character level", "1 temporary hit point per hit die".
    r"|your (character|class|total) level|per (hit die|hit dice|HD)\b",
    re.I,
)

# An ability-score phrase is a scaling magnitude ONLY when a granting
# construction introduces it. Bare mention is overwhelmingly a cross-reference
# to an existing rule, not a new magnitude this record computes -- PF1's
# flat-footed idiom ("you don't lose your Dexterity bonus to AC") appears
# throughout the corpus and grants nothing.
ABILITY_PHRASE = re.compile(
    r"\byour (Strength|Dexterity|Constitution|Intelligence|Wisdom|Charisma)"
    r" (score|modifier|bonus)",
    re.I,
)
_GRANT = (r"add|adds|adding|gain|gains|gaining|equal to|plus|minus|times"
          r"|increased? by|increases by|bonus of")
_REFER = (r"lose|loses|losing|lost|retain|retains|retaining|deny|denies|denied"
          r"|deprived of|instead of|rather than|in place of|whichever is")
_FILLER = r"[^.;|]{0,30}"
ABILITY_GRANT = re.compile(r"\b(?:%s)\b%s$" % (_GRANT, _FILLER), re.I)
ABILITY_REFER = re.compile(r"\b(?:%s)\b%s$" % (_REFER, _FILLER), re.I)


def ability_scaling(field):
    """True if `field` GRANTS a magnitude derived from an ability score.

    Decided per occurrence, not per field. A field may both grant one magnitude
    and reference another -- `Agile Maneuvers` adds Dex to CMB in one clause and
    names Str in the next -- so a field-wide veto would discard the real grant.
    For each occurrence, whichever construction sits NEAREST before it wins.
    """
    for m in ABILITY_PHRASE.finditer(field):
        lead = field[max(0, m.start() - 45):m.start()]
        g = ABILITY_GRANT.search(lead)
        r = ABILITY_REFER.search(lead)
        if g and (not r or g.start() > r.start()):
            return True
    return False

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


_mod_index = None


def mod_index():
    """Map (book, resolved_base_name) -> [raw `.MOD` rows].

    A `.MOD` row MODIFIES an existing base record rather than declaring one, so
    the work-inventory generator emits no unit for it (`src/bin/v06_work_inventory.rs`
    ~line 546 stashes it into `mod_targets` and returns; its magnitude count is
    consumed only by the `mod_only_rescue` path, i.e. only when the base name
    appears nowhere in the corpus). When a base declaration DOES exist, the
    `.MOD` row's magnitudes are discarded and never reach the base unit.

    Base-name resolution mirrors the generator's own, so the two agree about
    which record a `.MOD` row belongs to.
    """
    global _mod_index
    if _mod_index is not None:
        return _mod_index
    _mod_index = collections.defaultdict(list)
    for root, _, files in os.walk(CORPUS):
        book = os.path.relpath(root, CORPUS).split(os.sep)[0]
        for fn in sorted(files):
            if not fn.endswith(".lst"):
                continue
            with open(os.path.join(root, fn), encoding="utf-8", errors="replace") as fh:
                for raw in fh:
                    raw = raw.rstrip("\n")
                    if not raw.strip() or raw.lstrip().startswith("#"):
                        continue
                    head = raw.split("\t", 1)[0].strip()
                    at = head.find(".MOD")
                    if at < 0:
                        continue
                    base = head[:at]
                    if base.startswith("CATEGORY=") and "|" in base:
                        base = base.split("|", 1)[1]
                    if base.startswith("CLASS:"):
                        base = base[6:]
                    _mod_index[(book, base.strip())].append(raw)
    return _mod_index


def token_closure(unit):
    """Every corpus row that governs this unit: its base row plus its `.MOD` rows."""
    rows = [corpus_line(unit["book"], unit["source_file"], unit["source_line"])]
    idx = mod_index()
    for name in {unit.get("name"), unit.get("corpus_key")}:
        if name:
            rows.extend(idx.get((unit["book"], name), []))
    return rows


def closure_signals(rows):
    """Union the signals over a token closure.

    `display` survives only if NO row in the closure carries a magnitude-bearing
    field. That is the whole point: a magnitude on a `.MOD` row must not leave
    the base unit looking like a text-only record.
    """
    real = [r for r in rows if r is not None]
    if not real:
        return {"no_corpus_line"}
    out = set()
    for r in real:
        out |= signals(r)
    if len(real) > 1 and any(
        any(f.strip().startswith(MAGNITUDE_TOKENS) for f in r.split("\t")) for r in real
    ):
        out.discard("display:no_magnitude_token")
    out.discard("no_corpus_line")
    return out or {"no_corpus_line"}


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
            elif ability_scaling(f):
                out.add("ambiguous:prose_ability_scaling")

    if not mags:
        out.add("display:no_magnitude_token")
    elif not any(s.startswith(("computed:", "derived:")) for s in out):
        out.add("static:literal_magnitudes_only")
    return out


def wiring_class(sigs):
    """Collapse a signal set to one class. Strictly highest-bar-wins."""
    if sigs == {"no_corpus_line"}:
        return "ambiguous", "no_corpus_line"
    for prefix in ("computed:", "derived:"):
        hit = sorted(s for s in sigs if s.startswith(prefix))
        if hit:
            return prefix[:-1], hit[0].split(":", 1)[1]
    # `ambiguous` outranks `display`. A row with no magnitude TOKEN can still
    # state a magnitude in prose -- `ultimate_campaign`'s story feats carry
    # "spell resistance equal to 5 + your character level" on a `.MOD BENEFIT:`
    # row and nothing else. Letting `display` win there would mark a unit done
    # the moment its text renders, which is the exact over-claim this axis
    # exists to prevent. `display` is the LAST resort, never a short circuit.
    hit = sorted(s for s in sigs if s.startswith("ambiguous:"))
    if hit:
        return "ambiguous", hit[0].split(":", 1)[1]
    if any(s.startswith("display:") for s in sigs):
        return "display", "no_magnitude_token"
    return "static", "literal_magnitudes_only"


HELD = ("ingested-magnitude", "text-complete", "grounded", "deferred-with-reason", "unknown")

# Every string below is taken verbatim from a real corpus row. `True` means the
# field GRANTS an ability-derived magnitude; `False` means it references one
# that already exists. Run with `--selftest`.
ABILITY_CASES = [
    (True, "BENEFIT:You add your Dexterity bonus to your base attack bonus and "
           "Strength modifier when determining CMB"),
    (True, "DESC:you gain a bonus on electricity damage rolls equal to your Wisdom bonus (%1)"),
    (True, "DESC:deal 1d6 points of bludgeoning damage plus your Strength modifier"),
    (True, "BENEFIT:choose a number of spells that you already know equal to your "
           "Intelligence modifier"),
    (True, "DESC:you can add twice your Intelligence modifier in damage (minimum 2)"),
    (True, "DESC:move up to 5 feet times your Intelligence modifier (minimum 1)"),
    (True, "DESC:you recover additional hit points equal to half your Constitution "
           "modifier (minimum +1)"),
    (False, "DESC:you don't lose your Dexterity bonus to Armor Class, and the attacker "
            "doesn't get the +2 bonus"),
    (False, "DESC:While running, you retain your Dexterity bonus to your Armor Class."),
    (False, "DESC:You retain your Dexterity bonus to AC even when flat-footed"),
    (False, "DESC:you may use your Dexterity modifier instead of your Strength modifier "
            "on attack rolls"),
    (False, "DESC:While denied your Dexterity bonus to AC you are also denied this resistance"),
    (False, "DESC:A condition that makes you lose your Dexterity bonus to Armor Class "
            "also makes you lose dodge bonuses"),
    (False, "DESC:use the higher of your caster level or your Strength modifier, "
            "whichever is your Charisma modifier"),
]


def selftest():
    """Assert the D4b grant-vs-reference discriminator against real corpus text."""
    bad = 0
    for want, text in ABILITY_CASES:
        got = ability_scaling(text)
        if got != want:
            bad += 1
            print("FAIL  want=%-5s got=%-5s  %s" % (want, got, text[:72]))
    print("%d/%d ability discriminator cases pass"
          % (len(ABILITY_CASES) - bad, len(ABILITY_CASES)))
    return 1 if bad else 0


def main():
    scope = sys.argv[1] if len(sys.argv) > 1 else "HELD"
    if scope == "--selftest":
        sys.exit(selftest())
    doc = json.load(open(INVENTORY))
    units = doc["units"]
    sel = [u for u in units
           if (u["status"] in HELD if scope == "HELD" else u["status"] == scope)]

    per_class = collections.Counter()
    per_reason = collections.Counter()
    per_book = collections.defaultdict(collections.Counter)
    dual = 0
    upstream_marked = 0
    for u in sel:
        rows = token_closure(u)
        sigs = closure_signals(rows)
        cls, why = wiring_class(sigs)
        if any(r and UPSTREAM_NOT_IMPLEMENTED in r for r in rows):
            upstream_marked += 1
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
    print("  carrying upstream '%s' marker %d (reported, never classifying)"
          % (UPSTREAM_NOT_IMPLEMENTED, upstream_marked))
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
