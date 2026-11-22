#!/usr/bin/env python3
"""E5.1 structural delta + race-speed check for the Starfinder package.

usage: E5.1_delta.py <new sheet_rules dir> <baseline sheet_rules dir> <oracle data root>

1. Delta classes over every differing rule (rule ids per file must be unchanged):
   - `race_speed`: a race principal whose only change is its `StatBlock "Speed"` prose segment;
   - `skill_words`: a rule whose only change is its label, the old label with every
     `Display ~ ` removed;
   anything else is `other_moves` (must be 0). Files added or removed are other moves.
2. Second implementation of the race speed: for every race principal in the new package, the
   oracle row(s) its `provenance.closure_rows` cite, read straight from the `.lst` files: the
   `MOVE:` pairs in order, a pair whose speed is 0 replaced by the row's `BONUS:VAR|<mode>|<n>`,
   then every other `BONUS:VAR|<mode>|<n>` (n > 0) whose mode the game data declares as a
   `MOVEMENT:` in a `*_dynamic.lst` file, sorted by mode. Compared with the package text.
Exit 0 only if other_moves == 0 and speed mismatches == 0.
"""
import glob
import json
import os
import sys
from collections import Counter


def load(path):
    with open(path, encoding="utf-8") as f:
        return json.load(f)


def speed_text(rule):
    for seg in rule.get("prose") or []:
        if seg.get("family") == {"StatBlock": "Speed"}:
            return "".join(p.get("Text", "?") for p in seg["pieces"])
    return None


def without_speed(rule):
    r = dict(rule)
    r["prose"] = [s for s in rule.get("prose") or [] if s.get("family") != {"StatBlock": "Speed"}]
    return r


def classify(rn, ro):
    keys = {k for k in set(rn) | set(ro) if rn.get(k) != ro.get(k)}
    if keys == {"label"} and rn["label"] == ro["label"].replace("Display ~ ", "") and "Display ~ " in ro["label"]:
        return "skill_words"
    if keys == {"prose"} and ":race:" in rn["id"] and "#" not in rn["id"] and without_speed(rn) == without_speed(ro):
        return "race_speed"
    return None


def main():
    new_dir, base_dir, oracle = sys.argv[1], sys.argv[2], sys.argv[3]
    classes, other, files = Counter(), [], 0
    for root, _, names in os.walk(new_dir):
        for n in names:
            if not n.endswith(".json"):
                continue
            p = os.path.join(root, n)
            rel = os.path.relpath(p, new_dir)
            b = os.path.join(base_dir, rel)
            if not os.path.exists(b):
                other.append(f"{rel}: new file")
                continue
            with open(p, "rb") as f1, open(b, "rb") as f2:
                if f1.read() == f2.read():
                    continue
            files += 1
            new, old = load(p), load(b)
            if not isinstance(new, list) or [r["id"] for r in new] != [r["id"] for r in old]:
                other.append(f"{rel}: not a rule file, or rule ids moved")
                continue
            for rn, ro in zip(new, old):
                if rn == ro:
                    continue
                c = classify(rn, ro)
                if c is None:
                    other.append(f"{rel}: {rn['id']} changed outside the E5.1 classes")
                else:
                    classes[c] += 1
    for root, _, names in os.walk(base_dir):
        for n in names:
            rel = os.path.relpath(os.path.join(root, n), base_dir)
            if not os.path.exists(os.path.join(new_dir, rel)):
                other.append(f"{rel}: file removed")

    # Second implementation: the race speed read off the oracle rows.
    modes = set()
    for f in glob.glob(os.path.join(oracle, "starfinder", "**", "*_dynamic.lst"), recursive=True):
        for line in open(f, encoding="utf-8", errors="replace"):
            first = line.split("\t")[0].strip()
            if first.startswith("MOVEMENT:"):
                modes.add(first[len("MOVEMENT:"):].strip().lower())
    cache = {}

    def row(spec):
        path, _, num = spec.rpartition(":")
        if path not in cache:
            with open(os.path.join(oracle, path), encoding="utf-8", errors="replace") as fh:
                cache[path] = fh.read().split("\n")
        return cache[path][int(num) - 1]

    races, mismatches = 0, []
    for f in sorted(glob.glob(os.path.join(new_dir, "*", "race", "*.json"))):
        for rule in load(f):
            if "#" in rule["id"]:
                continue
            races += 1
            move, var = [], {}
            for spec in rule.get("provenance", {}).get("closure_rows", []):
                for tok in row(spec).split("\t"):
                    tok = tok.strip()
                    if tok.startswith("MOVE:"):
                        parts = [x.strip() for x in tok[5:].split(",")]
                        move = list(zip(parts[0::2], parts[1::2]))
                    elif tok.startswith("BONUS:VAR|"):
                        fields = tok.split("|")
                        if len(fields) == 3 and fields[1].lower() in modes and fields[2].lstrip("-").isdigit():
                            var.setdefault(fields[1], set()).add(int(fields[2]))
            single = {m: next(iter(v)) for m, v in var.items() if len(v) == 1}
            pieces = []
            for mode, speed in move:
                stated = next((m for m in single if m.lower() == mode.lower()), None)
                value = single.pop(stated) if stated is not None else None
                pieces.append(f"{mode} {value if value is not None and speed == '0' else speed} ft.")
            pieces += [f"{m} {n} ft." for m, n in sorted(single.items()) if n > 0]
            want = ", ".join(pieces) if pieces else None
            got = speed_text(rule)
            if got != want:
                mismatches.append(f"{rule['id']}: package {got!r} oracle {want!r}")

    display_new = sum("Display ~ " in r["label"] for f in glob.glob(os.path.join(new_dir, "*", "*", "*.json")) for r in load(f))
    display_old = sum("Display ~ " in r["label"] for f in glob.glob(os.path.join(base_dir, "*", "*", "*.json")) for r in load(f))
    print(f"differing_files={files} " + " ".join(f"{k}={v}" for k, v in sorted(classes.items())) + f" other_moves={len(other)}")
    for o in other[:40]:
        print("  OTHER", o)
    print(f"movement_modes={sorted(modes)} race_principals={races} speed_mismatches={len(mismatches)}")
    for m in mismatches[:40]:
        print("  MISMATCH", m)
    print(f"labels_with_Display~ before={display_old} after={display_new}")
    verdict = "PASS" if not other and not mismatches else "FAIL"
    print(f"verdict={verdict}")
    sys.exit(0 if verdict == "PASS" else 1)


if __name__ == "__main__":
    main()
