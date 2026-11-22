#!/usr/bin/env python3
"""SD-37 E4.MC: independent re-derivation of the 34 supplement fixture rows E4.2/E4.4/E4.5 wrote
without a second reviewer (initiative 4, spell DC 6, loadout 24).

Inputs are only E0.4's reviewed `seed-builds.md` (each seed's `Final` ability row and its item
tables, re-parsed here; the prose arithmetic lines are NOT read) and the SRD rules quoted in the
receipt (initiative = Dex mod; spell DC = 10 + spell level + key mod + Spell Focus 1 at 5th;
Table 11-5 3rd 4,000 / 5th 9,000; bulk: numbers add, 10 L = 1, '—' = 0; limits Str//2 and Str).
Prints one line per row (OK / MISMATCH) and exits 1 on any mismatch or on a row count != 34.

usage: E4.MC_fixture_review.py   (from the repo root)
"""
import re, sys

PKG = "docs/release/SD-37-starfinder-1e/artifacts"
SEEDS = {"SF-Soldier-3": 3, "SF-Mystic-5": 5, "SF-Technomancer-5": 5, "SF-Envoy-3": 3}
KEY = {"SF-Mystic-5": "Wis", "SF-Technomancer-5": "Int"}
WEALTH = {3: 4000, 5: 9000}
ABIL = ["Str", "Dex", "Con", "Int", "Wis", "Cha"]


def cells(line):
    return [c.strip() for c in line.strip().strip("|").split("|")]


def num(s):
    return int(s.replace(",", "").replace("*", "").strip())


def parse_builds():
    text = open(f"{PKG}/epic_0/seed-builds.md").read()
    sections = re.split(r"^## \d+\. ", text, flags=re.M)
    out = {}
    for sec in sections:
        name = sec.split(" ", 1)[0]
        if name not in SEEDS:
            continue
        scores, items, header = None, [], None
        for line in sec.splitlines():
            if not line.startswith("|"):
                header = None
                continue
            c = cells(line)
            if c[0].startswith("**Final"):
                scores = dict(zip(ABIL, (num(x) for x in c[1:7])))
            if c[0] == "Item":
                header = c
                continue
            if header and not set(c[0]) <= set("-"):
                row = dict(zip(header, c))
                qty = num(row["Qty"]) if "Qty" in row else 1
                price = num(row.get("Price each", row.get("Price")))
                bulk = row.get("Bulk each", row.get("Bulk"))
                items.append((row["Item"], qty, price, bulk))
        out[name] = (scores, items)
    return out


def bulk_total(items):
    whole, light = 0, 0
    for _, qty, _, b in items:
        if b == "L":
            light += qty
        elif b in ("—", "-", ""):
            pass
        else:
            whole += num(b) * qty
    return whole + light // 10


def mod(score):
    return (score - 10) // 2


builds = parse_builds()
expected = {}
for seed, level in SEEDS.items():
    scores, items = builds[seed]
    expected[(seed, "Initiative")] = mod(scores["Dex"])
    if seed in KEY:
        for lvl, label in [(0, "0"), (1, "1st"), (2, "2nd")]:
            expected[(seed, f"Spell DC: {label}")] = 10 + lvl + mod(scores[KEY[seed]]) + 1
    spent = sum(q * p for _, q, p, _ in items)
    expected[(seed, "Starting credits")] = WEALTH[level]
    expected[(seed, "Credits spent")] = spent
    expected[(seed, "Credits remaining")] = WEALTH[level] - spent
    expected[(seed, "Bulk")] = bulk_total(items)
    expected[(seed, "Bulk limit unencumbered")] = scores["Str"] // 2
    expected[(seed, "Bulk limit overburdened")] = scores["Str"]

files = [f"{PKG}/epic_4/E4.2-initiative-hand-values.md", f"{PKG}/epic_4/E4.4-spell-dc-hand-values.md",
         f"{PKG}/epic_4/E4.5-loadout-hand-values.md"]
rows, bad = 0, 0
for f in files:
    for line in open(f):
        if not re.match(r"^\| *SF-", line):
            continue
        c = cells(line)
        seed, field, value = c[0], c[1], int(c[2].replace("+", ""))
        rows += 1
        want = expected.get((seed, field))
        ok = want == value
        bad += not ok
        print(f"{'OK' if ok else 'MISMATCH'} {seed} | {field} | fixture {value} | re-derived {want}")
print(f"rows={rows} expected_keys={len(expected)} mismatches={bad}")
sys.exit(0 if rows == 34 and len(expected) == 34 and bad == 0 else 1)
