#!/usr/bin/env python3
"""SD-37 E5.2 structural classifier: every move between two Starfinder packages is one of the
pinned E5.2 delta classes, or the verdict is FAIL.

usage: python3 E5.2_delta.py <new sheet_rules dir> <baseline sheet_rules dir>

Delta classes (the converter change `sf_text_repaired` / `sf_unmisdecoded` in
crates/codex-ingest/src/pcgen_import/sheet_rule/convert.rs):
  misdecoded_text   a prose `Text` piece whose new value = fix(old value), fix() being this
                    file's own re-implementation of the repair (UTF-8 read as Windows-1252);
  misdecoded_fact   any other string leaf (a deity's `FactDeclare` value) with new = fix(old);
  stray_pipe        the same, with the old piece's trailing `|` gone (the 1252 `"` in `â€"` had
                    split the DESC token, leaving a `|` and a `glued-tokens` defect row);
  glued_defect_rows the `_defects/glued-tokens.json` rows removed -- each must name a record of
                    class stray_pipe.
Anything else (a label, a value, a gate, a grant, a record added or removed, a file moved) is
other_moves, and other_moves > 0 is FAIL.
"""
import glob
import json
import os
import re
import sys

LEADS = re.compile("[âÂÃ]")


def b(ch):
    o = ord(ch)
    if o in (0x81, 0x8D, 0x8F, 0x90, 0x9D) or 0xA0 <= o <= 0xFF:
        return o
    try:
        x = ch.encode("cp1252")
    except UnicodeEncodeError:
        return None
    return x[0] if len(x) == 1 and x[0] >= 0x80 else None


def fix(t):
    if not LEADS.search(t):
        return t
    t = re.sub(r'â€"(?=\d)', "–", t).replace('â€"', "—")
    out, i, cs = [], 0, list(t)
    while i < len(cs):
        lead = b(cs[i])
        if lead is not None and 0xC2 <= lead <= 0xF4:
            n = 2 if lead < 0xE0 else 3 if lead < 0xF0 else 4
            seq = [b(c) for c in cs[i:i + n]]
            if len(seq) == n and None not in seq:
                try:
                    out.append(bytes(seq).decode("utf-8"))
                    i += n
                    continue
                except UnicodeDecodeError:
                    pass
        out.append(cs[i])
        i += 1
    return "".join(out).replace("Â ", " ").replace("Ã-", "×")


def leaf_moves(o, n, path=()):
    """(path, old, new) for every differing leaf; None for a structural difference."""
    if type(o) is not type(n):
        return None
    if isinstance(o, dict):
        if set(o) != set(n):
            return None
        out = []
        for k in o:
            sub = leaf_moves(o[k], n[k], path + (k,))
            if sub is None:
                return None
            out += sub
        return out
    if isinstance(o, list):
        if len(o) != len(n):
            return None
        out = []
        for i, (x, y) in enumerate(zip(o, n)):
            sub = leaf_moves(x, y, path + (i,))
            if sub is None:
                return None
            out += sub
        return out
    return [] if o == n else [(path, o, n)]


def files(root):
    out = {}
    for dirpath, _, names in os.walk(root):
        for n in names:
            full = os.path.join(dirpath, n)
            out[os.path.relpath(full, root)] = full
    return out


def main():
    new_root, base_root = sys.argv[1], sys.argv[2]
    new, base = files(new_root), files(base_root)
    counts = {"misdecoded_text": 0, "misdecoded_fact": 0, "stray_pipe": 0, "glued_defect_rows": 0}
    records = {"misdecoded_text": set(), "misdecoded_fact": set(), "stray_pipe": set()}
    other = []
    removed_glued = []
    for rel in sorted(set(new) | set(base)):
        if rel not in new or rel not in base:
            if rel == "_defects/glued-tokens.json" and rel not in new:
                removed_glued = json.load(open(base[rel], encoding="utf-8"))
                continue
            other.append(f"file {'added' if rel in new else 'removed'}: {rel}")
            continue
        a, c = open(base[rel], "rb").read(), open(new[rel], "rb").read()
        if a == c:
            continue
        if rel == "_defects/glued-tokens.json":
            old_rows, new_rows = json.loads(a), json.loads(c)
            if set(new_rows) <= set(old_rows):
                removed_glued = sorted(set(old_rows) - set(new_rows))
                continue
        if rel.startswith("_") or rel.count("/") != 2:
            other.append(f"file changed: {rel}")
            continue
        olds = {r["id"]: r for r in json.loads(a)}
        news = {r["id"]: r for r in json.loads(c)}
        if set(olds) != set(news):
            other.append(f"record set changed: {rel}")
            continue
        for rid in olds:
            o, n = olds[rid], news[rid]
            if o == n:
                continue
            moves = leaf_moves(o, n)
            if moves is None:
                other.append(f"structural move: {rid}")
                continue
            for path, to, tn in moves:
                if not (isinstance(to, str) and isinstance(tn, str)):
                    other.append(f"non-string move: {rid} {path}")
                    continue
                cls = "misdecoded_text" if path[-1] == "Text" and path[0] == "prose" else "misdecoded_fact"
                if tn == fix(to):
                    counts[cls] += 1
                    records[cls].add(rid)
                elif cls == "misdecoded_text" and to.endswith("|") and tn == fix(to[:-1]):
                    counts["stray_pipe"] += 1
                    records["stray_pipe"].add(rid)
                else:
                    other.append(f"text move not the repair: {rid}: {to[:60]!r} -> {tn[:60]!r}")
    glued_records = {row.split(": ")[0] for row in removed_glued}
    counts["glued_defect_rows"] = len(removed_glued)
    if glued_records != records["stray_pipe"]:
        other.append(f"glued-tokens rows removed {sorted(glued_records)} != stray_pipe records {sorted(records['stray_pipe'])}")
    left = 0
    for rel, full in new.items():
        if rel.endswith(".json") and LEADS.search(open(full, encoding="utf-8").read()):
            left += 1
    if left:
        other.append(f"{left} files still hold a mis-decoded lead character")
    moved = records["misdecoded_text"] | records["misdecoded_fact"] | records["stray_pipe"]
    print(f"records_moved={len(moved)} misdecoded_text_pieces={counts['misdecoded_text']} "
          f"misdecoded_fact_leaves={counts['misdecoded_fact']} "
          f"stray_pipe_pieces={counts['stray_pipe']} glued_defect_rows_removed={counts['glued_defect_rows']} "
          f"files_with_lead_chars_after={left} other_moves={len(other)}")
    kinds = {}
    for rid in moved:
        kinds[rid.split(":")[1]] = kinds.get(rid.split(":")[1], 0) + 1
    print(f"records_moved_by_kind={dict(sorted(kinds.items()))}")
    for line in other[:20]:
        print("  OTHER", line)
    print(f"verdict={'PASS' if not other else 'FAIL'}")
    sys.exit(0 if not other else 1)


if __name__ == "__main__":
    main()
