#!/usr/bin/env python3
"""Re-derive buckets U and Z, sub-cause by sub-cause, and prove each unit's disposition.

AT-35-E5-003's Evidence sentence is "U and Z at 0; `corpus_literal_sweep` examined-count
moved by exactly the `beginner_box` record delta" (`epic-breakdown.md ### AT-35-E5-003`),
over a criterion body that asks, for U, "per sub-cause, the instrument correction or a
proven statement that the record carries nothing a player reads", and for Z, "`beginner_box`
gets a compiled rule set through the guarded generator path, then converts".

This script is the command behind every figure in `AT-35-E5-003_cycle1_receipt.md`. Like
`AT-35-E5-002_bucket_d_sub_causes.py` it reads each historical `docs/work-inventory.json`
out of git and partitions it with the LIVE `scripts/completion_atlas.py` bucket rule, so
every column is one lens applied to every state.

Its `--proof` mode is the half AT-35-E5-002 did not need: for every U and Z unit standing at
the `tranche/15` cut it opens the converted rule in `data/sheet_rules/<book>/<kind>/<key>.json`
and reports what a player actually reads off the rendered line — the label, the tag list, the
magnitude, and whether the rule carries prose. That is what turns "the record carries nothing
a player reads" from an assertion into a checked statement.

Usage:
    python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-003_buckets_u_z.py
    ... --json          machine-readable census
    ... --at <sha>      one state only
    ... --transitions   cut-state U/Z sub-causes -> their HEAD status/evidence
    ... --proof         cut-state U/Z units -> the converted rule each one renders
    ... --sweep-delta   the beginner_box record delta the Evidence sentence names
    ... --rendered <sheet_rule_bucket_v_render output.json>
                        what each unit's LIVE rendered sheet line actually shows

Exit 0 when U and Z are both 0 at HEAD (and, in the other modes, when that mode's own
verdict is PASS); exit 1 otherwise.
"""

import argparse
import collections
import glob
import json
import os
import re
import subprocess
import sys
import tempfile

REPO_ROOT = os.path.abspath(
    os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "..", "..")
)
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))

import completion_atlas as atlas  # noqa: E402

# The inventory-moving commits of SD-35, oldest first, re-derivable with:
#   git log --format='%h %s' 4c6c57eb9f..HEAD -- docs/work-inventory.json
STATES = [
    ("4c6c57eb9f", "tranche/15 cut — bundle launch state"),
    ("51f91bba11", "AT-35-E2-005 — first corpus-wide conversion"),
    ("406003afc3", "AT-35-E3-001 cycle 2 — term-level degradation replaces record refusal"),
    ("26bdfa8d5b", "AT-35-E3-002 — the whole remainder to DONE"),
    ("9bae2cfa1f", "AT-35-E4-001 — 24 mapping rows + 1 head alias"),
    ("HEAD", "HEAD"),
]

CUT = "4c6c57eb9f"

# The book whose Z units the criterion names. Z's whole population is this book.
Z_BOOK = "beginner_box"


def units_at(sha):
    blob = subprocess.run(
        ["git", "-C", REPO_ROOT, "show", f"{sha}:docs/work-inventory.json"],
        capture_output=True, text=True, check=True,
    ).stdout
    with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as fh:
        fh.write(blob)
        path = fh.name
    try:
        return atlas._load_inventory(path)["units"]
    finally:
        os.unlink(path)


def family(unit):
    """The sub-cause a U or Z unit belongs to.

    Bucket U's 202 units carry two evidence strings, but the first covers two
    materially different sub-causes — an item with no `DESC:` and a feat with no
    `DESC:` — which the `reason` field separates. Bucket Z carries one.
    """
    bucket = atlas._bucket_of(unit)
    if bucket == "Z":
        return "Z:" + (unit.get("evidence") or "")
    evidence = unit.get("evidence") or ""
    if evidence == "text_only_but_corpus_record_carries_no_description_to_show_a_player":
        reason = unit.get("reason") or ""
        return "U:" + evidence + ":" + ("feat" if reason.startswith("the feat") else "item")
    return "U:" + evidence


def census(sha):
    units = units_at(sha)
    counts = atlas.partition(units)["counts"]
    uz = [u for u in units if atlas._bucket_of(u) in ("U", "Z")]
    return {
        "sha": sha,
        "population": len(units),
        "buckets": {k: counts.get(k, 0) for k in
                    ["DONE", "A", "B", "C", "D", "M", "V", "U", "X", "Z"]},
        "u_total": counts.get("U", 0),
        "z_total": counts.get("Z", 0),
        "by_kind": dict(collections.Counter(u.get("kind") for u in uz).most_common()),
        "by_book": dict(collections.Counter(u.get("book") for u in uz).most_common()),
        "by_family": dict(collections.Counter(family(u) for u in uz).most_common()),
    }


def transitions(from_sha=CUT, to_sha="HEAD"):
    """Per U/Z sub-cause at `from_sha`, where each of its units stands at `to_sha`.

    A sub-cause is not "gone" because its evidence string stopped being emitted; it is
    gone because each of its units carries a rendered sheet line at HEAD.
    """
    src = units_at(from_sha)
    dst = {u.get("id"): u for u in units_at(to_sha)}
    uz = [u for u in src if atlas._bucket_of(u) in ("U", "Z")]
    totals = collections.Counter()
    dests = collections.defaultdict(collections.Counter)
    for u in uz:
        fam = family(u)
        totals[fam] += 1
        h = dst.get(u.get("id"))
        key = (h.get("status"), h.get("evidence")) if h else ("MISSING_AT_HEAD", None)
        dests[fam][key] += 1
    return totals, dests


def _rule_path(unit_id):
    book, kind, key = unit_id.split(":", 2)
    return os.path.join(REPO_ROOT, "data", "sheet_rules", book, kind, key + ".json")


def _value_form(value):
    if isinstance(value, str):
        return value
    if isinstance(value, dict) and value:
        return next(iter(value))
    return "?"


def proof(from_sha=CUT):
    """What a player reads off the rendered line, for every cut-state U/Z unit.

    The criterion's "proven statement that the record carries nothing a player reads" is
    checked here rather than asserted: a unit whose rule carries no prose still prints its
    label, its tags and any magnitude, and those are what a player writes on the sheet.
    """
    src = units_at(from_sha)
    uz = [u for u in src if atlas._bucket_of(u) in ("U", "Z")]
    rows = collections.defaultdict(collections.Counter)
    missing = []
    unreadable = []
    samples = {}
    for u in uz:
        fam = family(u)
        path = _rule_path(u["id"])
        if not os.path.exists(path):
            missing.append(u["id"])
            rows[fam]["NO_RULE_FILE"] += 1
            continue
        with open(path, "r", encoding="utf-8") as fh:
            rules = json.load(fh)
        forms = tuple(sorted({_value_form(r.get("value")) for r in rules}))
        has_prose = any(r.get("prose") for r in rules)
        has_label = all((r.get("label") or "").strip() for r in rules)
        has_tags = any(r.get("tags") for r in rules)
        # A line a player can read carries, at minimum, a non-empty label. Prose,
        # tags and a magnitude are each additional readable content.
        if not has_label:
            unreadable.append(u["id"])
        key = (forms, "prose" if has_prose else "no-prose",
               "tags" if has_tags else "no-tags")
        rows[fam][key] += 1
        samples.setdefault((fam,) + key, (u["id"], os.path.relpath(path, REPO_ROOT)))
    return rows, missing, unreadable, samples


def rendered(lines_path, from_sha=CUT):
    """What each cut-state U/Z unit's LIVE rendered sheet line actually shows.

    `lines_path` is a `sheet_rule_bucket_v_render --output` file. That binary runs the same
    live evaluator (`render_sheet` over `data/sheet_rules/`) the desktop's `sheet_lines_for`
    takes, so this is the sheet a player would read, not a re-reading of the package.

    Reports, per sub-cause, how much of the line is content: the label alone, the label with
    the rule's words, the label with a magnitude, or -- for a record whose own source says
    `print: false` -- no line at all. Also counts any line still carrying upstream PCGen's
    editorial not-implemented admission; that count must be 0.
    """
    src = units_at(from_sha)
    fams = {u["id"]: family(u) for u in src if atlas._bucket_of(u) in ("U", "Z")}
    with open(lines_path, "r", encoding="utf-8") as fh:
        doc = json.load(fh)
    marker = re.compile(r"not\s*implement", re.IGNORECASE)
    rows = collections.defaultdict(collections.Counter)
    markered = []
    for u in doc["units"]:
        fam = fams.get(u["id"])
        if fam is None:
            continue
        if u.get("suppressed") or not u.get("line"):
            rows[fam]["not-printed (source print:false)"] += 1
            continue
        line = u["line"]
        printed = (line.get("printed") or "").strip()
        prose = (line.get("prose") or "").strip()
        also = line.get("also") or []
        if marker.search(prose + " " + (line.get("label") or "")):
            markered.append(u["id"])
        magnitude = bool(printed or also)
        if prose and magnitude:
            cell = "label + magnitude + prose"
        elif prose:
            cell = "label + prose"
        elif magnitude:
            cell = "label + magnitude"
        else:
            cell = "label only"
        rows[fam][cell] += 1
    return rows, markered


def _corpus_records(sha, book):
    """Count the corpus records under data/corpus/<book> at `sha`.

    This is the "record delta" the Evidence sentence names. It is counted out of git so
    that the cut state and HEAD are measured the same way.
    """
    out = subprocess.run(
        ["git", "-C", REPO_ROOT, "ls-tree", "-r", "--name-only", sha,
         f"data/corpus/{book}/"],
        capture_output=True, text=True, check=True,
    ).stdout.split()
    return [p for p in out if p.endswith(".json")]


def sweep_delta(from_sha=CUT, to_sha="HEAD"):
    before = _corpus_records(from_sha, Z_BOOK)
    after = _corpus_records(to_sha, Z_BOOK)
    diff = subprocess.run(
        ["git", "-C", REPO_ROOT, "diff", "--name-only", f"{from_sha}..{to_sha}",
         "--", f"data/corpus/{Z_BOOK}"],
        capture_output=True, text=True, check=True,
    ).stdout.split()
    rules = glob.glob(os.path.join(REPO_ROOT, "data", "sheet_rules", Z_BOOK, "*", "*.json"))
    return {
        "book": Z_BOOK,
        "corpus_records_before": len(before),
        "corpus_records_after": len(after),
        "record_delta": len(after) - len(before),
        "corpus_files_changed": len(diff),
        "compiled_rule_files_at_head": len(rules),
    }


def main(argv=None):
    ap = argparse.ArgumentParser()
    ap.add_argument("--json", action="store_true")
    ap.add_argument("--at")
    ap.add_argument("--transitions", action="store_true",
                    help="cut-state U/Z sub-causes -> their HEAD status/evidence")
    ap.add_argument("--proof", action="store_true",
                    help="cut-state U/Z units -> the converted rule each one renders")
    ap.add_argument("--sweep-delta", action="store_true",
                    help="the beginner_box record delta the Evidence sentence names")
    ap.add_argument("--rendered", metavar="LINES_JSON",
                    help="a sheet_rule_bucket_v_render --output file: what each unit's live "
                         "rendered sheet line actually shows, per sub-cause")
    args = ap.parse_args(argv)

    if args.transitions:
        totals, dests = transitions()
        print(f"sub_causes={len(totals)} uz_units={sum(totals.values())}")
        for fam, n in totals.most_common():
            print(f"{n:>5}  {fam}")
            for (status, ev), k in dests[fam].most_common():
                print(f"        -> {k:>5}  {status} / {ev}")
        lost = sum(k for fam in dests for (st, _), k in dests[fam].items()
                   if st != "sheet-complete")
        print(f"not_sheet_complete_at_HEAD={lost} "
              f"verdict={'PASS' if lost == 0 else 'FAIL'}")
        return 0 if lost == 0 else 1

    if args.proof:
        rows, missing, unreadable, samples = proof()
        print(f"sub_causes={len(rows)} "
              f"uz_units={sum(sum(c.values()) for c in rows.values())}")
        for fam in sorted(rows, key=lambda f: -sum(rows[f].values())):
            print(f"{sum(rows[fam].values()):>5}  {fam}")
            for key, n in rows[fam].most_common():
                print(f"        {n:>5}  {key}")
                s = samples.get((fam,) + key if isinstance(key, tuple) else (fam, key))
                if s:
                    print(f"              e.g. {s[0]}  ->  {s[1]}")
        print(f"missing_rule_file={len(missing)} rules_without_a_label={len(unreadable)} "
              f"verdict={'PASS' if not missing and not unreadable else 'FAIL'}")
        return 0 if not missing and not unreadable else 1

    if args.rendered:
        rows, markered = rendered(args.rendered)
        print(f"sub_causes={len(rows)} "
              f"uz_units={sum(sum(c.values()) for c in rows.values())}")
        for fam in sorted(rows, key=lambda f: -sum(rows[f].values())):
            print(f"{sum(rows[fam].values()):>5}  {fam}")
            for cell, n in rows[fam].most_common():
                print(f"        {n:>5}  {cell}")
        totals = collections.Counter()
        for c in rows.values():
            totals.update(c)
        print("totals " + " ".join(f"{k}={v}" for k, v in sorted(totals.items())))
        print(f"editorial_marker_on_a_rendered_line={len(markered)} "
              f"verdict={'PASS' if not markered else 'FAIL'}")
        return 0 if not markered else 1

    if args.sweep_delta:
        d = sweep_delta()
        print(" ".join(f"{k}={v}" for k, v in d.items()))
        ok = d["record_delta"] == 0 and d["corpus_files_changed"] == 0
        print(f"verdict={'PASS' if ok else 'FAIL'}")
        return 0 if ok else 1

    states = [(args.at, "requested")] if args.at else STATES
    rows = [census(sha) for sha, _ in states]

    if args.json:
        print(json.dumps({"states": rows}, indent=2, sort_keys=True))
    else:
        for (sha, label), row in zip(states, rows):
            print(f"== {sha}  {label}")
            print(f"   population={row['population']} U={row['u_total']} Z={row['z_total']}  "
                  + " ".join(f"{k}={v}" for k, v in row["buckets"].items()))
            for kind, n in row["by_kind"].items():
                print(f"     kind {kind:<20} {n}")
            for fam, n in row["by_family"].items():
                print(f"     sub-cause {n:>5}  {fam}")
            print()

    head = rows[-1]
    total = head["u_total"] + head["z_total"]
    print(f"bucket_U_at_HEAD={head['u_total']} bucket_Z_at_HEAD={head['z_total']} "
          f"verdict={'PASS' if total == 0 else 'FAIL'}")
    return 0 if total == 0 else 1


if __name__ == "__main__":
    raise SystemExit(main())
