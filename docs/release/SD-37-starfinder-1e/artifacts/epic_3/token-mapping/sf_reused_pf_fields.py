#!/usr/bin/env python3
"""List every Pathfinder mapping-table row (SD-35 `mapping-table.v1.json`, 273 rows) whose PCGen
tag occurs in the in-scope Starfinder data, and split it into the rows SD-37 E3.3's SF mapping
table REMAPS and the rows the SF data uses with the Pathfinder meaning UNCHANGED (`decisions.md
§8`; the E3.3 receipt lists the unchanged ones so a reviewer can challenge each).

Census predicate: a tab field on a non-`#` line of any `.lst` under the 8 registered SF book
directories (`system_books::BOOK_PCCS`; the excluded `starfinder_society_rules`,
`lpj_design/infinite_space` and `paizo/core/_society` are skipped). Its tag is the text before the
first `:`; for `BONUS:` and `FACT:`/`FACTSET:` the sub-key up to the first `|` is kept too.
`sf_tag_occurrences` counts the row's tag heads (each distinct head once per row), so a PF row
keyed on a sub-key (`ASPECT:NAME`, `DESC:.CLEAR`) is counted at its head tag: coarse by design,
it says the tag is present, not that the sub-key is.

Usage:
  sf_reused_pf_fields.py --write   rewrite `reused_pf_fields` in sf-mapping-table.v1.json
  sf_reused_pf_fields.py --check   exit 1 if the committed list differs from a fresh census
The corpus root is $PCGEN_CORPUS_ROOT, else $PCGEN_REPO_DIR/data.
"""
import json
import os
import re
import sys
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[5]
PF_TABLE = REPO / "docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/token-mapping/mapping-table.v1.json"
SF_TABLE = HERE / "sf-mapping-table.v1.json"
BOOK_DIRS = [
    "starfinder/paizo/core",
    "starfinder/paizo/armory",
    "starfinder/paizo/character_operations_manual",
    "starfinder/paizo/pact_worlds",
    "starfinder/paizo/near_space",
    "starfinder/paizo/alien_archive",
    "starfinder/paizo/alien_archive_2",
    "starfinder/paizo/alien_archive_3",
]
EXCLUDED = ["starfinder/paizo/core/_society"]
# PF rows E3.3's table remaps, and the part of each it remaps (the rest of the tag keeps its PF meaning)
REMAPPED = {
    "BONUS:HP": "all of it: |CURRENTMAX (+HD, +RaceHP) is Hit Points, |ALTHP is Stamina (rows hit_points, stamina)",
    "HD (class)": "HD:1 is one Hit Point per level, a term of hit_points (row hit_points)",
    "BONUS:COMBAT": "only |AC|…|TYPE=EAC_Armor / KAC_Armor / Base / Ability (rows eac, kac); BASEAB, TOHIT, INITIATIVE etc. keep the PF meaning",
    "FACT / FACTSET": "only FACT:KeyAbilityScore (row key_ability); every other FACT keeps the PF meaning",
}
NOT_A_TAG = re.compile(r"^(FORMULA:|%|\[|\(|class LEVEL|\.MOD|trailing|feat\.prerequisites|!PRE|BONUS:\[)")


def corpus_root():
    if os.environ.get("PCGEN_CORPUS_ROOT"):
        return Path(os.environ["PCGEN_CORPUS_ROOT"])
    if os.environ.get("PCGEN_REPO_DIR"):
        return Path(os.environ["PCGEN_REPO_DIR"]) / "data"
    sys.exit("set PCGEN_CORPUS_ROOT or PCGEN_REPO_DIR (scripts/fetch-pcgen-oracle.sh)")


def sf_tag_census(root):
    heads = Counter()
    for d in BOOK_DIRS:
        for dirpath, _, files in os.walk(root / d):
            rel = Path(dirpath).relative_to(root).as_posix()
            if any(rel == e or rel.startswith(e + "/") for e in EXCLUDED):
                continue
            for f in files:
                if not f.endswith(".lst"):
                    continue
                for line in (Path(dirpath) / f).read_bytes().decode("utf-8", "replace").splitlines():
                    if line.startswith("#") or not line.strip():
                        continue
                    for field in line.split("\t"):
                        m = re.match(r"^(!?[A-Z][A-Z0-9_]*):", field)
                        if not m:
                            continue
                        tag = m.group(1)
                        heads[tag] += 1
                        if tag in ("BONUS", "FACT", "FACTSET"):
                            sub = field[len(tag) + 1:].split("|", 1)[0]
                            heads[f"{tag}:{sub}"] += 1
    return heads


def pf_row_tags(token_type):
    """The tag heads a PF row stands for, or None when the row is not a tag (formula grammar,
    closure rows, placeholders)."""
    if NOT_A_TAG.match(token_type):
        return None
    head = token_type.split(" (", 1)[0]
    tags = []
    for piece in re.split(r"\s*/\s*|,\s*", head):
        m = re.match(r"^([A-Z][A-Z0-9_]*)(?::([A-Z][A-Z0-9_]*))?", piece.strip())
        if m:
            tags.append(m.group(0) if m.group(2) and m.group(1) in ("BONUS",) else m.group(1))
    return list(dict.fromkeys(tags)) or None


def census():
    heads = sf_tag_census(corpus_root())
    pf = json.loads(PF_TABLE.read_text())["rows"]
    reused, remapped, absent, not_tag = [], [], 0, 0
    seen = set()
    for row in pf:
        tt = row["token_type"]
        if tt in seen:
            continue
        seen.add(tt)
        tags = pf_row_tags(tt)
        if tags is None:
            not_tag += 1
            continue
        n = sum(heads.get(t, 0) for t in tags)
        if n == 0:
            absent += 1
            continue
        if tt in REMAPPED:
            remapped.append({"field": tt, "sf_tag_occurrences": n, "remapped_part": REMAPPED[tt]})
        else:
            reused.append({
                "field": tt,
                "sf_tag_occurrences": n,
                "why_unchanged": "same PCGen tag, loaded by the same game-mode-independent PCGen token plugin; "
                                 "E3.3 found no Starfinder rule that reads it differently (challengeable)",
            })
    return {"pf_rows_distinct": len(seen), "not_a_tag": not_tag, "absent_from_sf": absent,
            "remapped": remapped, "reused": reused}


def main():
    mode = sys.argv[1] if len(sys.argv) > 1 else "--print"
    c = census()
    table = json.loads(SF_TABLE.read_text())
    summary = (f"pf_rows_distinct={c['pf_rows_distinct']} not_a_tag={c['not_a_tag']} absent_from_sf={c['absent_from_sf']} "
               f"remapped={len(c['remapped'])} reused={len(c['reused'])}")
    total = c["not_a_tag"] + c["absent_from_sf"] + len(c["remapped"]) + len(c["reused"])
    if total != c["pf_rows_distinct"]:
        print(f"PARTITION FAIL: {total} != {c['pf_rows_distinct']}")
        sys.exit(1)
    if mode == "--write":
        table["reused_pf_fields"] = c["reused"]
        table["remapped_pf_fields"] = c["remapped"]
        SF_TABLE.write_text(json.dumps(table, indent=2, ensure_ascii=False) + "\n")
        print(summary + " verdict=WRITTEN")
    elif mode == "--check":
        ok = table.get("reused_pf_fields") == c["reused"] and table.get("remapped_pf_fields") == c["remapped"]
        print(summary + (" verdict=PASS" if ok else " verdict=STALE"))
        sys.exit(0 if ok else 1)
    else:
        for r in c["remapped"]:
            print(f"REMAPPED {r['field']}  ({r['sf_tag_occurrences']})  {r['remapped_part']}")
        for r in c["reused"]:
            print(f"REUSED   {r['field']}  ({r['sf_tag_occurrences']})")
        print(summary)


if __name__ == "__main__":
    main()
