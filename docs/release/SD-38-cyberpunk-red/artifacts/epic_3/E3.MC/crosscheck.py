#!/usr/bin/env python3
"""E3.MC second implementation: the headline figures of a loaded in-app sheet, re-derived from the
saved files with the BOOK's formulas in Python (no engine code), checked against the live text.

    python3 -I crosscheck.py <corpus dir> <saved character dir> <probe.json>

Figures (p. 79-81 unless named): the ten STATs as entered = the saved input; Hit Points
10 + 5 x ceil((BODY + WILL) / 2); Seriously Wounded ceil(HP / 2); Death Save = BODY; Humanity Loss =
the sum of the saved cyberware's preset HL (cyberware.json, p. 359-368); Humanity 10 x EMP - HL of
10 x EMP; current EMP = the tens place of Humanity; cash = the saved cash, and for a Complete Package
2,550 minus the corpus price of every bought item (p. 104-105). Prints one line per figure; exit 1 on
any miss.
"""
import json
import math
import sys
from pathlib import Path

corpus, saved, probe = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])


def pairs(path):
    out = []
    if path.exists():
        for line in path.read_text(encoding="utf-8").splitlines():
            if " = " in line:
                key, value = line.split(" = ", 1)
                out.append((key.strip(), value.strip()))
    return out


stats = {k.split(".", 1)[1]: int(v) for k, v in pairs(saved / "cpr_character_input.txt") if k.startswith("stat.")}
choices = pairs(saved / "cpr_sheet_choices.txt")
kit = [v.split(" | ") for k, v in pairs(saved / "cpr_kit.txt") if k == "item"]
cyberware = {r["name"]: r for r in json.loads((corpus / "cyberware.json").read_text(encoding="utf-8"))}
prices = {}
for kind in ("weapon", "armor", "gear", "cyberware", "ammunition", "fashion"):
    path = corpus / f"{kind}.json"
    if path.exists():
        for r in json.loads(path.read_text(encoding="utf-8")):
            if isinstance(r.get("cost_eb"), int):
                prices.setdefault(r["name"], r["cost_eb"])

body = json.loads(probe.read_text(encoding="utf-8"))["bodyText"]
lines = [l.strip() for l in body.split("\n")]
label = dict(pairs(saved / "envelope.txt"))["display_label"]
lines = lines[lines.index(label):]

hp = 10 + 5 * math.ceil((stats["body"] + stats["will"]) / 2)
installed = [v for k, v in choices if k == "cyberware"]
hl = sum(cyberware[name]["humanity_loss_preset"] for name in installed)
humanity_max = 10 * stats["emp"]
humanity = humanity_max - hl
saved_cash = int(dict(choices)["cash"])
bought = [item for item in kit if item[0] == "bought"]

expect = [f"{s.upper()}\t{stats[s]}" for s in ("int", "ref", "dex", "tech", "cool", "will", "luck", "move", "body")]
expect += [
    f"EMP\t{stats['emp']}\t{humanity // 10}",
    f"Hit Points\t{hp}",
    f"Seriously Wounded\t{math.ceil(hp / 2)}",
    f"Death Save\t{stats['body']}",
    f"Humanity\t{humanity} / {humanity_max}",
    f"Humanity Loss\t{hl}",
    f"cpr.cash\t{saved_cash}",
]
misses = [e for e in expect if not any(l == e or l.startswith(e + "\t") for l in lines)]
if bought:
    spent = sum(prices[name] * int(quantity) for _, quantity, name in bought)
    if saved_cash != 2550 - spent:
        misses.append(f"Complete Package cash {saved_cash} != 2550 - {spent}")
    print(f"{label}: Complete Package cash 2550 - {spent} = {2550 - spent} (saved {saved_cash})")
for e in expect:
    print(f"{label}: {'MISS' if e in misses else 'ok  '} {e!r}")
print(f"{label}: {len(expect) + (1 if bought else 0) - len(misses)} of {len(expect) + (1 if bought else 0)} figures re-derived from the book formulas")
sys.exit(1 if misses else 0)
