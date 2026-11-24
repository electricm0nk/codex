#!/usr/bin/env python3
"""Derive data/ability_categories.json: the ability categories PCGen shows on its Background and Misc
sub-tabs, as the engine's ability budget reads them (SD-43 E3.9).

PCGen titles a sub-tab with the category's display location (`AbilityCategory.getType()` returns
`String.valueOf(getDisplayLocation())`) and builds one sub-tab per distinct one (`AbilitiesInfoTab`).
The converted package carries each category's pick grants as `Pool(<category slug>)` rules but not the
category itself, so this file is the one place the engine learns which sub-tab a pool sits on, how its
members are filtered (parent category slug + type tags) and whether PCGen leaves its size to the player
(an editable pool: no rule grants it). Scope = E0.1's census: player-editable (visible YES|QUALIFY,
editable YES) definitions in the corpus's book roots and the game mode; same-slug definitions merge.

Reads the pinned oracle only through `scripts/fetch-pcgen-oracle.sh` (`$PCGEN_CORPUS_ROOT`,
`$PCGEN_REPO_DIR`); `--check` recomputes and compares (exit 1 on drift, 2 when an input is missing).
"""
import glob
import json
import os
import re
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUT = os.path.join(ROOT, "data", "ability_categories.json")
SUBTABS = {"Background": "background", "Misc": "misc"}


def slug(name: str) -> str:
    """`sheet_rule::slug`: lowercase ASCII alphanumeric runs joined by `_`."""
    return "_".join(re.findall(r"[a-z0-9]+", name.lower()))


def oracle() -> tuple[str, str, str]:
    out = subprocess.run(["bash", os.path.join(ROOT, "scripts/fetch-pcgen-oracle.sh"), "--check"], capture_output=True, text=True)
    env = dict(re.findall(r"^export (\w+)=(.*)$", out.stdout, re.M))
    pin = dict(re.findall(r"^(\w+)=(\S+)", open(os.path.join(ROOT, "scripts/pcgen-oracle-pin.env")).read(), re.M))
    repo, data = env.get("PCGEN_REPO_DIR", ""), env.get("PCGEN_CORPUS_ROOT", "")
    if out.returncode != 0 or not os.path.isdir(data):
        print(f"oracle unavailable: {out.stderr.strip()}", file=sys.stderr)
        sys.exit(2)
    head = subprocess.run(["git", "-C", repo, "rev-parse", "HEAD"], capture_output=True, text=True).stdout.strip()
    if head != pin["PCGEN_ORACLE_SHA"]:
        print(f"oracle HEAD {head} != pin {pin['PCGEN_ORACLE_SHA']}", file=sys.stderr)
        sys.exit(2)
    return repo, data, head


def book_roots() -> set[str]:
    roots = set()
    for f in glob.glob(os.path.join(ROOT, "data/corpus/*/*/**/*.json"), recursive=True):
        try:
            path = (json.load(open(f, encoding="utf-8")).get("source") or {}).get("path") or ""
        except Exception:
            continue
        if path:
            roots.add("/".join(os.path.dirname(path).split("/")[:4]))
    return roots


def build() -> dict:
    repo, data, head = oracle()
    roots = book_roots()
    files = sorted(glob.glob(os.path.join(data, "pathfinder", "**", "*.lst"), recursive=True))
    files += sorted(glob.glob(os.path.join(repo, "system", "gameModes", "Pathfinder", "*.lst")))
    merged: dict[str, dict] = {}
    for f in files:
        in_scope = not f.startswith(data) or any(os.path.relpath(f, data).startswith(r + "/") for r in roots)
        if not in_scope:
            continue
        for line in open(f, encoding="utf-8", errors="replace"):
            if not line.startswith("ABILITYCATEGORY:"):
                continue
            toks = [t for t in line.rstrip("\n").split("\t") if t.strip()]
            d = {}
            for t in toks[1:]:
                if ":" in t:
                    k, v = t.split(":", 1)
                    d.setdefault(k.strip(), v.strip())
            loc = SUBTABS.get(d.get("DISPLAYLOCATION", ""))
            if loc is None or d.get("VISIBLE", "").upper() not in ("YES", "QUALIFY") or d.get("EDITABLE", "").upper() != "YES":
                continue
            name = toks[0].split(":", 1)[1].strip()
            entry = merged.setdefault(slug(name), {
                "pool": slug(name), "name": name, "subtab": loc, "editable_pool": False,
                "member_pool": slug(d.get("CATEGORY", "")), "member_tags": [],
            })
            entry["editable_pool"] |= d.get("EDITPOOL", "").upper() == "YES"
            for t in (x for x in d.get("TYPE", "").split(".") if x):
                if t not in entry["member_tags"]:
                    entry["member_tags"].append(t)
            if d.get("ABILITYLIST"):
                # The engine filters members by parent category and tags only; a keyed list would need reading.
                print(f"{name}: a member list by key is not read by the engine", file=sys.stderr)
                sys.exit(1)
    return {
        "generator": "python3 scripts/derive_ability_categories.py",
        "oracle_pin": head,
        "categories": sorted(merged.values(), key=lambda c: (c["subtab"], c["pool"])),
    }


def main() -> None:
    doc = build()
    text = json.dumps(doc, indent=1, ensure_ascii=False) + "\n"
    if "--check" in sys.argv:
        same = os.path.exists(OUT) and open(OUT, encoding="utf-8").read() == text
        print("ABILITY_CATEGORIES_OK" if same else "ABILITY_CATEGORIES_DRIFT")
        sys.exit(0 if same else 1)
    open(OUT, "w", encoding="utf-8").write(text)
    counts = {tab: sum(c["subtab"] == tab for c in doc["categories"]) for tab in SUBTABS.values()}
    print(f"wrote {os.path.relpath(OUT, ROOT)}: {counts}")


if __name__ == "__main__":
    main()
