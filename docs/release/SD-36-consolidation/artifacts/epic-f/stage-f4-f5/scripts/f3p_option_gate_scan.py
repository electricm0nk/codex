#!/usr/bin/env python3
"""SD-36 F3 polish P5 scan: every parameterised `PREABILITY` item in the pinned oracle.

Denominator: every `PREABILITY:` (and `!PREABILITY:`) token occurrence in the pinned tree's
`pathfinder/` `.lst` rows -- standalone, inside an `ABILITY:...|PRE...` grant, a `BONUS:...|PRE...`
gate or a `PREMULT:` bracket -- split into its items. An item `<Base> (<Option>)` with no nested
parenthesis is PARAMETERISED; it is split by whether any oracle row that DECLARES `<Base>` (its
first field, or its `KEY:`) carries a `CHOOSE:` other than `NOCHOICE` -- the proxy for "the base
is a chooser", which the converter decides on the resolved record's own rows
(`prereq.rs::is_chooser`). This is an oracle-side proxy; the converter-side count is the
structural diff's `Chosen` delta (`f3p-receipt.md` §P5).

Usage: python3 f3p_option_gate_scan.py [PCGEN_DATA_ROOT]   (default $HOME/workspace/repos/pcgen/data)
"""
import os
import re
import sys
from collections import Counter

root = sys.argv[1] if len(sys.argv) > 1 else os.path.join(os.environ["HOME"], "workspace/repos/pcgen/data")
base = os.path.join(root, "pathfinder")
rows = []
for dirpath, _, files in os.walk(base):
    for f in files:
        if f.endswith(".lst"):
            p = os.path.join(dirpath, f)
            with open(p, encoding="utf-8", errors="replace") as fh:
                for n, line in enumerate(fh, 1):
                    if line.strip() and not line.startswith("#"):
                        rows.append((os.path.relpath(p, root), n, line.rstrip("\n")))

choosers = set()
declared = set()
for _, _, line in rows:
    fields = [t for t in line.split("\t") if t.strip()]
    if not fields or ":" in fields[0].split(".")[0] and not fields[0].startswith("CATEGORY="):
        # a `CLASS:`/`CATEGORY=...|X.MOD` style head; only plain declarations name an object
        pass
    head = fields[0].strip() if fields else ""
    names = set()
    if head and not re.match(r"^[A-Z]+[:=]", head):
        names.add(head.upper())
    for t in fields[1:]:
        if t.startswith("KEY:"):
            names.add(t[4:].strip().upper())
    declared |= names
    if any(t.startswith("CHOOSE:") and t[7:].strip().upper() != "NOCHOICE" for t in fields):
        choosers |= names

tok = re.compile(r"!?PREABILITY:([^\t|\]]*)")
item_re = re.compile(r"^(.*\S) \(([^()]*)\)$")
occ = 0
param = Counter()
examples = {}
for rel, n, line in rows:
    for m in tok.finditer(line):
        occ += 1
        items = [i.strip() for i in m.group(1).split(",")[1:]]
        for it in items:
            if it.startswith("CATEGORY=") or it.startswith("TYPE") or it.startswith("[") or it == "CHECKMULT":
                continue
            mm = item_re.match(it)
            if not mm:
                continue
            b = mm.group(1).upper()
            kind = "chooser base" if b in choosers else ("declared, no CHOOSE" if b in declared else "base not declared")
            param[kind] += 1
            examples.setdefault(kind, []).append(f"{rel}:{n} {it}")

total = sum(param.values())
print(f"PREABILITY occurrences: {occ}; parameterised items (non-nested): {total}")
for k, v in sorted(param.items()):
    print(f"  {k}: {v}")
    for e in examples[k][:5]:
        print(f"    e.g. {e}")

# The remainder mechanism: a chooser GRANTED with a fixed option (`ABILITY:<cat>|AUTOMATIC|<Base>
# (<Option>)`, Samurai's `Exotic Weapon Proficiency (Katana)`) records no pick under the chooser,
# so an option gate reading that chooser answers Exclude for it.
grant_re = re.compile(r"ABILITY:[^|\t]*\|(?:AUTOMATIC|VIRTUAL)\|([^\t]*)")
fixed = Counter()
fixed_examples = []
for rel, n, line in rows:
    for m in grant_re.finditer(line):
        for it in m.group(1).split("|"):
            it = it.strip()
            if it.startswith("PRE") or it.startswith("!PRE"):
                break
            mm = item_re.match(it)
            if mm and mm.group(1).upper() in choosers:
                fixed["fixed-option grant of a chooser"] += 1
                if len(fixed_examples) < 5:
                    fixed_examples.append(f"{rel}:{n} {it}")
print(f"ABILITY ...|AUTOMATIC/VIRTUAL| grants of a chooser with a fixed option (non-nested): {fixed['fixed-option grant of a chooser']}")
for e in fixed_examples:
    print(f"    e.g. {e}")
