#!/usr/bin/env python3
"""Classify the 75 still-unrowed literal-verified units by shape (first
raw_bonus_chains qualifier), reading each unit's real corpus record."""
import json, glob, collections

BOOK_ALIASES = {"bestiary": "beastiary"}


def build_index(kind):
    idx = {}
    for path in glob.glob(f"data/corpus/*/{kind}/**/*.json", recursive=True):
        parts = path.split("/")
        book = parts[2]
        key = parts[-1][:-5]
        idx[(book, key)] = path
    for canon, actual in BOOK_ALIASES.items():
        for (book, key), path in list(idx.items()):
            if book == actual:
                idx.setdefault((canon, key), path)
    return idx


eq_idx = build_index("equipment")
eqmod_idx = build_index("equipment/equipmods")

missing = json.load(open(
    "docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/finalize-wave3-missing-literal.json"
))["missing_unit_ids"]

shape_counts = collections.Counter()
detail = []
for uid in missing:
    book, kind, key = uid.split(":", 2)
    idx = eq_idx if kind == "equipment" else eqmod_idx if kind == "equipment_modifier" else None
    path = idx.get((book, key)) if idx else None
    if path is None:
        shape_counts["MISSING_CORPUS_FILE"] += 1
        detail.append({"unit_id": uid, "shape": "MISSING_CORPUS_FILE"})
        continue
    rec = json.load(open(path))["data"]
    chains = rec.get("raw_bonus_chains") or []
    if not chains:
        shape_counts["NO_BONUS_CHAIN"] += 1
        detail.append({"unit_id": uid, "shape": "NO_BONUS_CHAIN"})
        continue
    label = chains[0]["qualifiers"][0]
    # collapse to the family (WEAPONPROF=<x> -> WEAPONPROF, STAT|<ability> -> STAT, etc.)
    fam = label.split("=")[0].split("|")[0].split(".")[0]
    shape_counts[fam] += 1
    detail.append({"unit_id": uid, "shape": fam, "raw_label": label})

print("Missing-unit shape breakdown (of", len(missing), "):")
for shape, count in shape_counts.most_common():
    print(f"  {shape}: {count}")

with open(
    "docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/finalize-wave3-missing-literal-shapes.json",
    "w"
) as f:
    json.dump({"missing_by_shape": dict(shape_counts), "detail": detail}, f, indent=2, sort_keys=True)
    f.write("\n")
