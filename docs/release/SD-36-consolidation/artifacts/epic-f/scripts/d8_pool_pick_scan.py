#!/usr/bin/env python3
"""SD-36 F1c-5 (D8): ability-category pools a variable fills, measured against the oracle.

PCGen sizes an `ABILITYCATEGORY` by its `POOL:` formula. When that formula is one variable and a
record raises it with `BONUS:VAR|<var>|<n>`, the record hands the player <n> picks among the
category's members: every ability of the category's `CATEGORY:` carrying all of its `TYPE:` tags
(`apg_abilitycategories.lst:267` Summoner Class Selection, filled by `apg_abilities_class.lst:739`).

Reads the pinned oracle tree (`$PCGEN_CORPUS_ROOT`, else `~/workspace/repos/pcgen/data`; the same
book directories `closure.rs` `PinnedTree::load` reads, `_pfs/` overlays excluded) and the converted
package (`data/sheet_rules`), and reports:

  * categories: every distinct `ABILITYCATEGORY` name the oracle declares (the denominator);
  * with POOL: how many carry a `POOL:` token, split into a single variable name / a constant /
    another formula;
  * filled: variable pools some `BONUS:VAR` row raises (the fillers, as rows);
  * members, oracle side: each filled pool's oracle member rows (base ability rows -- a feat
    file's row with no CATEGORY token is CATEGORY FEAT -- whose TYPE, with their .MOD rows',
    selects them), and whether a converted principal stands for the row (the row is that
    principal's own base row, its first provenance closure row);
  * members, package side (after D8): the rules offering each pool's pick (`offers.count` is the
    pool variable), the distinct members they grant (`Granter::Choice` edges), and the
    `pool-member-unconverted` defect rows.

Usage: python3 d8_pool_pick_scan.py [<sheet_rules dir>] [--json]
"""
from __future__ import annotations

import json
import os
import re
import sys
from collections import defaultdict
from pathlib import Path

BOOKS = "pathfinder/paizo/roleplaying_game"
EXTRA = [
    "pathfinder/dreamscarred_press/ultimate_psionics",
    "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_1",
    "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_2",
    "pathfinder/paizo/campaign_setting/inner_sea_world_guide",
    "pathfinder/paizo/campaign_setting/inner_sea_combat",
    "pathfinder/paizo/campaign_setting/inner_sea_faiths",
    "pathfinder/paizo/campaign_setting/inner_sea_gods",
    "pathfinder/paizo/campaign_setting/inner_sea_magic",
    "pathfinder/paizo/campaign_setting/inner_sea_races",
    "pathfinder/paizo/campaign_setting/inner_sea_temples",
    "pathfinder/paizo/campaign_setting/inner_sea_taverns",
    "pathfinder/paizo/campaign_setting/inner_sea_bestiary",
    "pathfinder/paizo/campaign_setting/inner_sea_intrigue",
]
IDENT = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*$")


def root() -> Path:
    return Path(os.environ.get("PCGEN_CORPUS_ROOT") or Path.home() / "workspace/repos/pcgen/data")


def lst_files(r: Path):
    dirs = [p for p in sorted((r / BOOKS).iterdir()) if p.is_dir()] + [r / e for e in EXTRA]
    for d in dirs:
        for p in sorted(d.rglob("*.lst")):
            rel = p.relative_to(r).as_posix()
            if "_pfs" in rel.split("/"):
                continue
            yield rel, p.read_text(errors="replace").split("\n")


def tokens(line: str):
    fields = [f.strip() for f in line.rstrip("\r").split("\t") if f.strip()]
    if not fields:
        return "", []
    return fields[0], [tuple(f.split(":", 1)) if ":" in f else (f, "") for f in fields[1:]]


def main() -> int:
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    pkg = Path(args[0] if args else "data/sheet_rules")
    r = root()
    cats: dict[str, dict] = {}
    fillers: dict[str, list[str]] = defaultdict(list)
    abilities: dict[tuple[str, str], dict] = {}  # (CATEGORY upper, KEY upper) -> {row, tags}
    mods: list[tuple[str, str, list[str]]] = []
    for rel, lines in lst_files(r):
        base = rel.rsplit("/", 1)[-1].lower()
        # `closure.rs` `file_family`: an ability-family file (`abilit` / `feat` in the basename).
        is_ability = ("abilit" in base or "feat" in base) and "abilitycategor" not in base
        for n, line in enumerate(lines, 1):
            if line.lstrip().startswith("#"):
                continue
            name, toks = tokens(line)
            if not name:
                continue
            if name.upper().startswith("ABILITYCATEGORY:"):
                own = name.split(":", 1)[1].strip()
                d = cats.setdefault(own.upper(), {"name": own, "rows": [], "pool": None, "category": None, "type": None, "list": None})
                d["rows"].append(f"{rel}:{n}")
                for k, v in toks:
                    if k == "POOL" and d["pool"] is None:
                        d["pool"] = v.strip()
                    elif k == "CATEGORY" and d["category"] is None:
                        d["category"] = v.strip()
                    elif k == "TYPE" and d["type"] is None:
                        d["type"] = [t.strip() for t in v.split(".") if t.strip()]
                    elif k == "ABILITYLIST" and d["list"] is None:
                        d["list"] = v
            for k, v in toks:
                if k == "BONUS" and v.startswith("VAR|"):
                    parts = v.split("|")
                    if len(parts) >= 3:
                        for t in parts[1].split(","):
                            fillers[t.strip().upper()].append(f"{rel}:{n}")
            if not is_ability:
                continue
            cat = next((v.strip() for k, v in toks if k == "CATEGORY"), None)
            if cat is None and "feat" in base:
                cat = "FEAT"
            tags = [t.strip() for k, v in toks if k == "TYPE" for t in v.split(".") if t.strip()]
            if name.endswith(".MOD") or name.startswith("CATEGORY="):
                # `CATEGORY=<c>|<key>.MOD`: a .MOD row adds TYPE tags to its base row.
                head = name[:-4] if name.endswith(".MOD") else name
                if head.startswith("CATEGORY="):
                    c, _, key = head[len("CATEGORY="):].partition("|")
                    mods.append((c.strip().upper(), key.strip().upper(), tags))
                continue
            if cat is None or ".COPY=" in name:
                continue
            key = next((v.strip() for k, v in toks if k == "KEY"), name)
            abilities.setdefault((cat.upper(), key.upper()), {"row": f"{rel}:{n}", "tags": list(tags)})
    for c, key, tags in mods:
        if (c, key) in abilities:
            abilities[(c, key)]["tags"].extend(tags)

    # Converted principals by their own base row (the first provenance closure row).
    by_row: dict[str, str] = {}
    rules: dict[str, dict] = {}
    for p in sorted(pkg.glob("*/*/*.json")):
        if p.parts[-3].startswith("_"):
            continue
        for rule in json.loads(p.read_text()):
            rules[rule["id"]] = rule
            if "#" in rule["id"]:
                continue
            rows = rule.get("provenance", {}).get("closure_rows") or []
            if rows:
                by_row.setdefault(rows[0], rule["id"])
    names = json.loads((pkg.parent.parent / "scripts/oracle_harness/var_names.json").read_text()) if (pkg.parent.parent / "scripts/oracle_harness/var_names.json").exists() else {}
    choice_members: dict[str, set[str]] = defaultdict(set)
    for rule in rules.values():
        for g in rule.get("granted_by") or []:
            if isinstance(g.get("by"), dict) and "Choice" in g["by"]:
                choice_members[g["by"]["Choice"]].add(rule["id"])
    offering: dict[str, list[str]] = defaultdict(list)
    for rule in rules.values():
        o = rule.get("offers")
        if isinstance(o, dict) and o.get("id") == rule["id"] and isinstance(o.get("from"), dict) and "Rules" in o["from"] \
                and isinstance(o.get("count"), dict) and set(o["count"]) == {"Var"}:
            offering[names.get(o["count"]["Var"], o["count"]["Var"]).upper()].append(rule["id"])
    defect_path = pkg / "_defects/pool-member-unconverted.json"
    unconverted = json.loads(defect_path.read_text()) if defect_path.exists() else []

    with_pool = {k: d for k, d in cats.items() if d["pool"] is not None}
    var_pools = {k: d for k, d in with_pool.items() if IDENT.match(d["pool"]) and not d["pool"].upper() in ("YES", "NO")}
    const_pools = {k: d for k, d in with_pool.items() if re.fullmatch(r"-?\d+", d["pool"])}
    other = {k: d for k, d in with_pool.items() if k not in var_pools and k not in const_pools}
    filled = {k: d for k, d in var_pools.items() if fillers.get(d["pool"].upper())}
    report = []
    resolved_total = unresolved_total = 0
    for k, d in sorted(filled.items()):
        parent = (d["category"] or d["name"]).upper()
        tags = d["type"] or []
        if d["list"] is not None:
            selector = "ABILITYLIST"
            members = []
        else:
            selector = "TYPE" if tags else "CATEGORY"
            members = [(key, a["row"]) for (c, key), a in sorted(abilities.items())
                       if c == parent and all(any(t.lower() == o.lower() for o in a["tags"]) for t in tags)]
        res = [(key, row, by_row[row]) for key, row in members if row in by_row]
        unres = [(key, row) for key, row in members if row not in by_row]
        resolved_total += len(res)
        unresolved_total += len(unres)
        offers = sorted(offering.get(d["pool"].upper(), []))
        granted = sorted(set().union(*[choice_members[o] for o in offers])) if offers else []
        cat_slug = re.sub(r"_+", "_", re.sub(r"[^a-z0-9]", "_", d["name"].lower())).strip("_")
        report.append({"category": d["name"], "decl": d["rows"][0], "pool": d["pool"], "parent": parent, "selector": selector,
                       "tags": tags, "fillers": fillers[d["pool"].upper()], "members": len(members),
                       "resolved": [x[2] for x in res], "unresolved": [f"{key} ({row})" for key, row in unres],
                       "offering_rules": offers, "package_members": granted,
                       "package_unconverted": [l for l in unconverted if l.startswith(f"pool:{cat_slug} ")]})
    summary = {
        "categories": len(cats),
        "with_pool_token": len(with_pool),
        "pool_is_one_variable": len(var_pools),
        "pool_is_constant": len(const_pools),
        "pool_is_other": len(other),
        "variable_pools_filled_by_bonus_var": len(filled),
        "filled_by_selector": {s: sum(1 for x in report if x["selector"] == s) for s in ("TYPE", "CATEGORY", "ABILITYLIST")},
        "filled_with_no_oracle_member": sum(1 for x in report if x["members"] == 0),
        "oracle_members": resolved_total + unresolved_total,
        "members_resolved": resolved_total,
        "members_unresolved": unresolved_total,
        "filler_rows": sum(len(x["fillers"]) for x in report),
        "package_linked_pools": sum(1 for x in report if x["offering_rules"]),
        "package_offering_rules": sum(len(x["offering_rules"]) for x in report),
        "package_member_edges": sum(len(choice_members[o]) for x in report for o in x["offering_rules"]),
        "package_distinct_members": len(set().union(*[set(x["package_members"]) for x in report])),
        "package_unconverted_member_rows": len(unconverted),
        "filled_but_unlinked": sorted(f"{x['category']} [{x['pool']}] ({x['selector']})" for x in report if not x["offering_rules"]),
    }
    if "--json" in sys.argv:
        print(json.dumps({"summary": summary, "pools": report, "other_pool_formulas": {d["name"]: d["pool"] for d in other.values()}}, indent=1))
    else:
        for k, v in summary.items():
            print(f"{k}: {v}")
        for x in report:
            print(f"- {x['category']} [{x['pool']}] {x['selector']} {x['tags']}: {len(x['fillers'])} filler row(s); oracle "
                  f"{x['members']} member(s), {len(x['resolved'])} with a converted record, {len(x['unresolved'])} without; package "
                  f"{len(x['offering_rules'])} offering rule(s), {len(x['package_members'])} member(s), {len(x['package_unconverted'])} unconverted")
    return 0


if __name__ == "__main__":
    sys.exit(main())
