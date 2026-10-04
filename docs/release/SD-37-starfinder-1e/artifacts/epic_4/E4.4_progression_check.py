#!/usr/bin/env python3
"""SD-37 E4.4 implementation B: the converted Starfinder spell progression vs its two sources.

Independent of the Rust converter and engine: this script evaluates the package's own
`SpellCell` / `SpellsKnown` class rows with its own tiny evaluator and compares every cell
(class level 1-20 x spell level) with
  (a) the oracle's `CAST:` / `KNOWN:` level lines (`$PCGEN_CORPUS_ROOT`, pinned tree), and
  (b) the SRD class tables (Archives of Nethys pages, fetched into SRD_DIR; sha256 in
      E4.4-srd-fetch-log.txt).
It also counts the connection-spell hop: every `SPELLKNOWN:` spell on an Internal
`<X> Connection Spell - N` row the oracle grants from a connection, vs the package's
`#spell_known_*` lines.

Usage: E4.4_progression_check.py <package dir> <srd dir>
Exit 0 = every cell agrees and the hop census agrees; 1 otherwise; 2 = an input is missing.
"""
import glob
import html
import json
import os
import re
import sys

PKG = sys.argv[1] if len(sys.argv) > 1 else "data/starfinder-1e/sheet_rules"
SRD = sys.argv[2] if len(sys.argv) > 2 else ""
ORACLE = os.environ.get("PCGEN_CORPUS_ROOT", os.path.expanduser("~/workspace/repos/pcgen/data"))
CLASSES = {
    "mystic": ("core/class/mystic.json", "starfinder/paizo/core/scr_classes.lst", "Mystic", "Classes.aspx_ItemName_Mystic.html"),
    "technomancer": ("core/class/technomancer.json", "starfinder/paizo/core/scr_classes.lst", "Technomancer", "Classes.aspx_ItemName_Technomancer.html"),
    "witchwarper": (
        "character_operations_manual/class/witchwarper.json",
        "starfinder/paizo/character_operations_manual/scom_classes.lst",
        "Witchwarper",
        "Classes.aspx_ItemName_Witchwarper.html",
    ),
}


def ev(e, cl):
    if isinstance(e, dict):
        (k, v), = e.items()
        if k == "Const":
            return v
        if k == "ClassLevel":
            return cl
        if k == "Sum":
            return sum(ev(x, cl) for x in v)
        if k == "Mul":
            return ev(v[0], cl) * ev(v[1], cl)
        if k == "Min":
            return min(ev(v[0], cl), ev(v[1], cl))
        if k == "Max":
            return max(ev(v[0], cl), ev(v[1], cl))
    raise SystemExit(f"unevaluated expression {e!r}")


def admits(a, cl):
    if a == "Always":
        return True
    (k, v), = a.items()
    if k == "All":
        return all(admits(x, cl) for x in v)
    if k == "Compare":
        lhs = ev(v["lhs"], cl)
        rhs = ev(v["rhs"], cl)
        return {"Lte": lhs <= rhs, "Gte": lhs >= rhs, "Eq": lhs == rhs}[v["op"]]
    raise SystemExit(f"unread gate {a!r}")


def package_table(path, slug):
    """(known?, spell level) -> [value at class level 1..20], 0 where the row does not admit."""
    out = {}
    for r in json.load(open(path)):
        t = r.get("target")
        if not isinstance(t, dict):
            continue
        for kind, known in (("SpellCell", False), ("SpellsKnown", True)):
            if kind in t and t[kind]["class"] == slug and "#spell" in r["id"]:
                out[(known, t[kind]["level"])] = [ev(r["value"]["Number"], cl) if admits(r["applies"], cl) else 0 for cl in range(1, 21)]
    return out


def oracle_table(path, name):
    out = {}
    cls = None
    for line in open(path, encoding="utf-8", errors="replace"):
        if line.startswith("CLASS:"):
            cls = line.split("\t", 1)[0][6:]
        m = re.match(r"^(\d+)\t", line)
        if not m or cls != name:
            continue
        cl = int(m.group(1))
        for tok in line.rstrip("\n").split("\t"):
            for kind, known in (("CAST:", False), ("KNOWN:", True)):
                if tok.startswith(kind):
                    for sl, n in enumerate(tok[len(kind):].split(",")):
                        out.setdefault((known, sl), [0] * 20)[cl - 1] = int(n)
    # A CAST column that is 0 at every level is the 0-level "no limit" column: no per-day number.
    return {k: v for k, v in out.items() if k[0] or any(v)}


def srd_table(path):
    t = open(path, encoding="utf-8", errors="replace").read()
    t = re.sub(r"<[^>]+>", " ", t)
    t = re.sub(r"\s+", " ", html.unescape(t))
    out = {}
    per_day = t[t.find("Spells Per Day Level"):t.find("Spells Known Level")]
    for cl in range(1, 21):
        suffix = {1: "st", 2: "nd", 3: "rd"}.get(cl if cl < 20 else 0, "th")
        nxt = cl + 1
        nsuf = {1: "st", 2: "nd", 3: "rd"}.get(nxt if nxt < 20 else 0, "th")
        seg = per_day[per_day.find(f" {cl}{suffix} +"):]
        if cl < 20:
            end = seg.find(f" {nxt}{nsuf} +")
        else:
            # The 20th row ends where the table does: the six cells are the row's last
            # single-digit/dash tokens before any following prose (Witchwarper: "Enhanced Classes").
            m = re.search(r"((?:\s(?:\d|-)){6})(?=\s[A-Za-z]|$)", seg)
            end = m.end() if m else len(seg)
        cells = re.findall(r"(?:^|\s)(\d|-)(?=\s|$)", seg[:end])[-6:]
        for sl, c in enumerate(cells, start=1):
            out.setdefault((False, sl), [0] * 20)[cl - 1] = 0 if c == "-" else int(c)
    known = t[t.find("Spells Known Level"):]
    for m in re.finditer(r"(\d+)(?:st|nd|rd|th) ((?:[\d-] ){6}[\d-])", known[: known.find("Bonus Spells")]):
        cl = int(m.group(1))
        for sl, c in enumerate(m.group(2).split()):
            out.setdefault((True, sl), [0] * 20)[cl - 1] = 0 if c == "-" else int(c)
    return out


def main():
    for p in [PKG, ORACLE]:
        if not os.path.isdir(p):
            print(f"MISSING {p}")
            return 2
    bad = 0
    cells = 0
    srd_cells = 0
    for slug, (pkg_rel, lst_rel, name, srd_file) in CLASSES.items():
        pkg = package_table(os.path.join(PKG, pkg_rel), slug)
        ora = oracle_table(os.path.join(ORACLE, lst_rel), name)
        if set(pkg) != set(ora):
            print(f"{slug}: package columns {sorted(pkg)} != oracle columns {sorted(ora)}")
            bad += 1
        for k in sorted(set(pkg) | set(ora)):
            for cl in range(20):
                cells += 1
                if pkg.get(k, [None] * 20)[cl] != ora.get(k, [None] * 20)[cl]:
                    bad += 1
                    print(f"{slug} {k} level {cl + 1}: package {pkg.get(k)} oracle {ora.get(k)}")
        if SRD and os.path.exists(os.path.join(SRD, srd_file)):
            srd = srd_table(os.path.join(SRD, srd_file))
            for k in sorted(srd):
                for cl in range(20):
                    srd_cells += 1
                    if pkg.get(k, [0] * 20)[cl] != srd[k][cl]:
                        bad += 1
                        print(f"{slug} {k} level {cl + 1}: package {pkg.get(k, [0]*20)[cl]} SRD {srd[k][cl]}")
        elif SRD:
            print(f"MISSING {os.path.join(SRD, srd_file)}")
            return 2
    print(f"progression cells package=oracle checked={cells} (3 classes x 20 levels x (6 per-day + 7 known) columns)")
    print(f"progression cells package=SRD checked={srd_cells}")

    # Connection-spell hop census: oracle side (Internal rows named "<X> Connection Spell - N"
    # with SPELLKNOWN, granted from a connection row) vs package lines.
    oracle_spells = 0
    hop_rows = 0
    for f in glob.glob(os.path.join(ORACLE, "starfinder", "**", "*.lst"), recursive=True):
        for line in open(f, encoding="utf-8", errors="replace"):
            name = line.split("\t", 1)[0].strip()
            if not re.search(r" Connection Spell - \d$", name) or "CATEGORY:Internal" not in line:
                continue
            for tok in line.rstrip("\n").split("\t"):
                if tok.startswith("SPELLKNOWN:CLASS|"):
                    hop_rows += 1
                    fields = tok[len("SPELLKNOWN:CLASS|"):].split("|")
                    oracle_spells += sum(len([s for s in x.split(",") if s.strip()]) for x in fields if "=" not in x)
    pkg_lines = 0
    for f in glob.glob(os.path.join(PKG, "*", "*", "*.json")):
        for r in json.load(open(f)):
            if "#spell_known_" in r["id"]:
                pkg_lines += 1
    print(f"connection-spell hop rows (oracle, SPELLKNOWN on '<X> Connection Spell - N')={hop_rows} spells={oracle_spells} package #spell_known_ lines={pkg_lines}")
    if pkg_lines != oracle_spells:
        bad += 1
    print(f"verdict={'PASS' if bad == 0 else 'FAIL'} mismatches={bad}")
    return 0 if bad == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
