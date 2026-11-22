#!/usr/bin/env python3
"""SD-37 E3.4 seed deltas (R7): the converted Core Rulebook package's own lines for each SF seed.

No SF sheet renders before E4.6 (no Starfinder adapter), so this does not render a seed. It reads
the terms the generated package (`data/starfinder-1e/sheet_rules/core/`) states for each seed's
class and worn armour, adds the terms the package does not hold as record lines (the class's
`HD:1` die per level, the race's `RaceHP`, the Constitution modifier per level, the 10 base and
the Dex modifier capped by the armour's printed max Dex -- each from E3.3's mapping table and the
seed fixtures), and compares the sums with E3.3's SRD = PCGen values (E0.4 hand values).

Exit 0 when every seed's four sums equal E3.3's; 1 otherwise (each mismatch printed).
Run from the repo root: python3 docs/release/SD-37-starfinder-1e/artifacts/epic_3/E3.4_seed_terms.py
"""
import json
import os
import sys

PKG = "data/starfinder-1e/sheet_rules/core"
CORPUS = "data/starfinder-1e/corpus/core"

# name, class slug, level, race HP (RaceHP var, E3.3), scores STR DEX CON INT WIS CHA, armour KEY,
# E3.3's SRD = PCGen values (HP, Stamina, EAC, KAC). Source: crates/codex-ingest/tests/
# sf_mapping_table.rs seeds() and artifacts/epic_3/E3.3_cycle_receipt.md.
SEEDS = [
    ("SF-Soldier-3", "soldier", 3, 4, [16, 14, 12, 11, 10, 10], "Defiance Series (Squad)", (25, 24, 16, 19)),
    ("SF-Mystic-5", "mystic", 5, 4, [10, 14, 8, 14, 19, 15], "Lashunta tempweave (basic)", (34, 25, 16, 16)),
    ("SF-Technomancer-5", "technomancer", 5, 4, [10, 16, 14, 19, 13, 8], "D-suit I", (29, 35, 18, 19)),
    ("SF-Envoy-3", "envoy", 3, 2, [8, 13, 12, 12, 10, 18], "Carbon skin (graphite)", (20, 21, 14, 15)),
]


def mod(score):
    return score // 2 - 5


def coefficient(rules, target, slug):
    """The k of a `Mul [Const k, ClassLevel slug]` line on `target`."""
    for r in rules:
        if r.get("target") == target:
            v = r["value"]["Number"]
            if "Mul" in v and v["Mul"][1] == {"ClassLevel": slug}:
                return v["Mul"][0]["Const"]
            raise SystemExit(f"{slug}: {target} line is not k*level: {v}")
    raise SystemExit(f"{slug}: no {target} line")


def armour_file(key):
    for root, _, names in os.walk(os.path.join(CORPUS, "equipment")):
        for n in names:
            rec = json.load(open(os.path.join(root, n)))
            if rec["data"]["key"] == key:
                return os.path.join(PKG, "equipment", rec["unit_id"].split(":", 2)[2] + ".json")
    raise SystemExit(f"no corpus equipment record with key {key!r}")


def armour_terms(path):
    rules = json.load(open(path))
    eac = sum(r["value"]["Number"]["Const"] for r in rules if r.get("target") == "Eac")
    kac = sum(r["value"]["Number"]["Const"] for r in rules if r.get("target") == "Kac")
    max_dex = None
    for r in rules:
        for p in r.get("prose", []):
            fam = p["family"]
            if isinstance(fam, dict) and fam.get("StatBlock") == "Max Dex bonus":
                max_dex = int(p["pieces"][0]["Text"])
    return eac, kac, max_dex


def main():
    bad = 0
    for name, slug, level, race_hp, scores, armour, want in SEEDS:
        rules = json.load(open(os.path.join(PKG, "class", slug + ".json")))
        hp_k = coefficient(rules, "Hp", slug)
        sp_k = coefficient(rules, "Stamina", slug)
        apath = armour_file(armour)
        eac_a, kac_a, max_dex = armour_terms(apath)
        dex = mod(scores[1]) if max_dex is None else min(mod(scores[1]), max_dex)
        got = (
            hp_k * level + 1 * level + race_hp,
            sp_k * level + mod(scores[2]) * level,
            10 + eac_a + dex,
            10 + kac_a + dex,
        )
        ok = got == want
        bad += not ok
        print(
            f"{name}: package class lines Hp {hp_k}*L={hp_k * level}, Stamina {sp_k}*L={sp_k * level}; "
            f"armour {os.path.relpath(apath, PKG)} EAC +{eac_a} KAC +{kac_a} maxDex {max_dex} -> "
            f"HP {got[0]} SP {got[1]} EAC {got[2]} KAC {got[3]}; E3.3 {want} {'OK' if ok else 'MISMATCH'}"
        )
    print(f"seeds={len(SEEDS)} agree={len(SEEDS) - bad} disagree={bad}")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
