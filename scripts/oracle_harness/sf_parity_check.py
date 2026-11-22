#!/usr/bin/env python3
"""SD-37 E7.1: an independent count of the Starfinder oracle-parity comparison.

The desktop test `sf_oracle_parity` (apps/desktop/src-tauri/src/sf_oracle_parity.rs) is the gate.
This script re-derives its two headline figures -- oracle fields compared, and differences -- with
its own reading of the PCGen exports (`scripts/oracle_harness/sf_parity/*.oracle.txt`) against the
engine values the test writes when `SF_PARITY_ENGINE_OUT=<dir>` is set (one `<build>.json` per
build, key "engine"), and checks every difference against `explained.tsv`.

Usage: sf_parity_check.py <engine dump dir>     (exit 0 iff every difference is explained and no
                                                 ledger row is stale)
"""
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ORACLE = os.path.join(HERE, "sf_parity")


def num(text):
    t = text.strip()
    for suffix in (" lbs.", " cr"):
        if t.endswith(suffix):
            t = t[: -len(suffix)]
    t = t.strip().lstrip("+")
    try:
        float(t)
    except ValueError:
        return None
    return t


def oracle_fields(path):
    out = {}
    for line in open(path, encoding="utf-8"):
        line = line.rstrip("\n")
        if "=" not in line:
            continue
        k, v = line.split("=", 1)
        cells = v.split("|")
        attrs = dict(c.split("=", 1) for c in cells[1:] if "=" in c)
        if k.startswith("skill."):
            ranks = attrs.get("ranks", "")
            ranks = ranks[:-2] if ranks.endswith(".0") else ranks
            if attrs.get("untrained") == "NO" and ranks == "0":
                out[k] = "untrained"
            elif num(cells[0]) is not None:
                out[k] = num(cells[0])
            if ranks:
                out[k + ".ranks"] = ranks
        elif k.startswith("weapon."):
            name = k[len("weapon."):].lstrip("*")
            if num(cells[0]) is not None:
                out[f"weapon.{name}.attack"] = num(cells[0])
            m = re.match(r"^\d*d\d+([+-]\d+)?", attrs.get("damage", ""))
            if m:
                out[f"weapon.{name}.damage"] = (m.group(1) or "+0").lstrip("+")
        elif k.startswith("spells."):
            cls, lvl = k[len("spells."):].rsplit(".", 1)
            parts = dict(c.split(":", 1) for c in cells if ":" in c)
            if parts.get("per_day") == "0" and parts.get("known") == "0":
                continue
            out[f"spells.{cls.lower()}.{lvl}.per_day"] = parts.get("per_day", "")
            out[f"spells.{cls.lower()}.{lvl}.known"] = parts.get("known", "")
            dcs = sorted({d for d in parts.get("dcs", "").split(",") if d.strip()})
            if dcs:
                out[f"spells.{cls.lower()}.{lvl}.dc"] = "|".join(dcs)
        elif k in ("var.walk", "var.fly"):
            out["speed." + k[4:]] = num(v)
        elif k in ("name", "race", "credits", "acp") or k.startswith(("abilities.", "var.", "master")):
            continue
        elif k == "class":
            out[k] = v.lower()
        elif num(v) is not None:
            out[k] = num(v)
    return out


def main():
    dump = sys.argv[1]
    ledger = set()
    for line in open(os.path.join(ORACLE, "explained.tsv"), encoding="utf-8"):
        if line.startswith("#") or not line.strip():
            continue
        c = line.rstrip("\n").split("\t")
        ledger.add(tuple(c[:4]))
    compared = 0
    diffs = set()
    names = {}
    for f in sorted(os.listdir(dump)):
        if not f.endswith(".json"):
            continue
        stem = f[:-5]
        build = "SF-Mechanic-1-drone" if stem == "sf_mechanic_1_drone" else "SF-" + "-".join(p.capitalize() for p in stem.split("_")[1:])
        engine = json.load(open(os.path.join(dump, f)))["engine"]
        oracle = oracle_fields(os.path.join(ORACLE, f"{stem}.oracle.txt"))
        if stem == "sf_mechanic_1_drone":
            oracle.pop("level", None)
        names[build] = len(oracle)
        for k, v in oracle.items():
            compared += 1
            e = engine.get(k, "absent")
            if e != v:
                diffs.add((build, k, e, v))
    unexplained = sorted(diffs - ledger)
    stale = sorted(ledger - diffs)
    print(f"builds {len(names)} compared {compared} differ {len(diffs)} explained {len(ledger)} unexplained {len(unexplained)} stale {len(stale)}")
    for d in unexplained[:20]:
        print("UNEXPLAINED", *d, sep="\t")
    for d in stale[:20]:
        print("STALE", *d, sep="\t")
    sys.exit(1 if unexplained or stale else 0)


if __name__ == "__main__":
    main()
