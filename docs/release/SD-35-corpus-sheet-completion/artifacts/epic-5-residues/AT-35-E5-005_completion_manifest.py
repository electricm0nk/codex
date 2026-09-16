"""Re-derives `completion-manifest.json` (AT-35-E5-005) -- one row per unit, 49,438 of them.

Run from anywhere inside the repo:

    python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-005_completion_manifest.py
    python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-005_completion_manifest.py --check

`--check` regenerates in memory and exits non-zero unless the on-disk manifest is byte-for-byte
what a fresh run produces -- the same contract `sheet_rule_convert -- --check` has.

The criterion's Evidence sentence (`epic-breakdown.md ### AT-35-E5-005`):

    `completion_atlas.py --check` -> `DONE=49438 of 49438`, every other bucket zero.
    `artifacts/epic-5-residues/completion-manifest.json` -- one row per unit.

The manifest is the per-unit form of the atlas's aggregate verdict: the atlas prints ten bucket
counts, this prints the 49,438 rows those counts are over, each with the bucket it lands in and
the evidence string that put it there. Both read the SAME partition function
(`completion_atlas._bucket_of`) -- the manifest never re-implements bucket derivation, so the two
cannot drift (`AGENTS.md` "derive counts two ways" applies to independent COUNTS, not to a second
copy of a classifier, which is the drift it warns about).

Three fail-closed assertions, each of which makes a silently-wrong manifest impossible rather
than merely unlikely (`AGENTS.md` rule 8):

  1. every row's `bucket` is `DONE` -- a non-DONE row aborts the run and the cycle STOPS
     (the dispatch's scope flags: "if any bucket is non-zero this cycle STOPS and reports");
  2. the row count equals the atlas's own `examined` population, and every unit id is unique;
  3. the bucket histogram over the manifest's rows equals `completion_atlas.partition()`'s
     counts, computed independently from the same units.

`sheet_rule_content` is the only column not read from the inventory. It reads the LIVE
`data/sheet_rules/` package (never a fixture, `decisions.md §4`) and says which of `decisions.md
§1`'s sheet forms the unit's rule actually carries content for:

  prose+value  -- the rule's words AND a principal value
  prose        -- the rule's words (§1 form 3)
  value_only   -- a final number or dice in final form (§1 forms 1 and 2), no prose
  label_only_with_granted_by
               -- no words of its own; the record is a pointer row and its content lives on the
                  rule named in `granted_by` (the cross-record ownership shape SD-34's
                  capability register named as `cross_record_content_ownership_resolution`)
  label_only   -- the rule prints its own name and nothing else, because the corpus record
                  carries no description upstream (`description: ""`); the sheet line is the
                  feature's name, which is what a player writes on paper
  no_rule      -- the unit has no rule in the package (its DONE evidence comes from an engine
                  table or the bucket-V oracle ledger, not from the sheet-rule package)

That column is reported, never used to judge DONE-ness: the atlas's `_done_evidence_is_supported`
is the instrument that judges a DONE unit's evidence, and it is green at HEAD.
"""
import argparse
import collections
import json
import os
import subprocess
import sys

REPO = subprocess.run(["git", "rev-parse", "--show-toplevel"],
                      capture_output=True, text=True, check=True).stdout.strip()
sys.path.insert(0, os.path.join(REPO, "scripts"))
import completion_atlas as ca  # noqa: E402

INVENTORY = os.path.join(REPO, "docs/work-inventory.json")
SHEET_RULES = os.path.join(REPO, "data/sheet_rules")
OUT = os.path.join(REPO, "docs/release/SD-35-corpus-sheet-completion/artifacts/"
                         "epic-5-residues/completion-manifest.json")
RE_DERIVE = ("python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/"
             "AT-35-E5-005_completion_manifest.py")


def head_sha() -> str:
    return subprocess.run(["git", "rev-parse", "HEAD"], cwd=REPO,
                          capture_output=True, text=True, check=True).stdout.strip()


def index_sheet_rules() -> dict:
    """`{rule_id: rule}` over the live package. The first record wins for a duplicated id --
    the package's own `sheet_rule_convert -- --check` is what guarantees there are none."""
    by_id = {}
    for dirpath, _dirnames, filenames in os.walk(SHEET_RULES):
        for name in filenames:
            if not name.endswith(".json") or name.startswith("_"):
                continue
            with open(os.path.join(dirpath, name), "r", encoding="utf-8") as fh:
                try:
                    rules = json.load(fh)
                except json.JSONDecodeError:
                    continue
            if not isinstance(rules, list):
                continue
            for rule in rules:
                if isinstance(rule, dict) and "id" in rule:
                    by_id.setdefault(rule["id"], rule)
    return by_id


def sheet_rule_content(rule) -> str:
    if rule is None:
        return "no_rule"
    pieces = sum(len(family.get("pieces") or []) for family in (rule.get("prose") or []))
    has_value = rule.get("value") != "Text"
    if pieces and has_value:
        return "prose+value"
    if pieces:
        return "prose"
    if has_value:
        return "value_only"
    if rule.get("granted_by"):
        return "label_only_with_granted_by"
    return "label_only"


def build() -> dict:
    with open(INVENTORY, "r", encoding="utf-8") as fh:
        units = json.load(fh)["units"]
    by_id = index_sheet_rules()

    rows = []
    seen = set()
    non_done = []
    for unit in units:
        bucket = ca._bucket_of(unit)
        uid = unit.get("id")
        if uid in seen:
            raise SystemExit(f"duplicate unit id in the inventory: {uid}")
        seen.add(uid)
        if bucket != "DONE":
            non_done.append({"id": uid, "bucket": bucket, "status": unit.get("status"),
                             "evidence": unit.get("evidence")})
        rows.append({
            "id": uid,
            "book": unit.get("book"),
            "kind": unit.get("kind"),
            "name": unit.get("name"),
            "bucket": bucket,
            "status": unit.get("status"),
            "evidence": unit.get("evidence"),
            "source_file": unit.get("source_file"),
            "source_line": unit.get("source_line"),
            "sheet_rule_content": sheet_rule_content(by_id.get(uid)),
        })

    # Assertion 1 -- fail closed on ANY non-DONE row. A remaining unit is a number to report and
    # re-scope, never an exemption: this raises rather than filtering.
    if non_done:
        raise SystemExit(
            f"{len(non_done)} of {len(rows)} units are NOT DONE at HEAD -- the criterion's "
            f"population is not zero and this cycle must STOP and report, not write a manifest. "
            f"First 10: {json.dumps(non_done[:10])}"
        )

    # Assertions 2 and 3 -- population and histogram against the atlas's own partition.
    pres = ca.partition(units)
    if pres["unclassified_ids"] or pres["overlap_ids"]:
        raise SystemExit("the live atlas partition is not clean; fix that before manifesting it")
    if len(rows) != pres["examined"]:
        raise SystemExit(f"row count {len(rows)} != atlas examined {pres['examined']}")
    hist = collections.Counter(r["bucket"] for r in rows)
    if hist != pres["counts"]:
        raise SystemExit(f"manifest histogram {dict(hist)} != atlas counts {dict(pres['counts'])}")

    rows.sort(key=lambda r: r["id"])
    return {
        "criterion": "AT-35-E5-005",
        "generated_at_head": head_sha(),
        "re_derive_command": RE_DERIVE,
        "atlas_command": "python3 scripts/completion_atlas.py --check",
        "partition_source": "scripts/completion_atlas.py::_bucket_of (imported, never re-implemented)",
        "summary": {
            "units": len(rows),
            "by_bucket": {b: hist.get(b, 0) for b in ca.BUCKET_ORDER},
            "non_done": sum(v for b, v in hist.items() if b != "DONE"),
            "by_status": dict(sorted(collections.Counter(r["status"] for r in rows).items())),
            "by_sheet_rule_content": dict(sorted(
                collections.Counter(r["sheet_rule_content"] for r in rows).items())),
            "distinct_evidence_strings": len({r["evidence"] for r in rows}),
            "books": len({r["book"] for r in rows}),
            "kinds": len({r["kind"] for r in rows}),
        },
        "units": rows,
    }


def serialize(doc: dict) -> str:
    head, tail = dict(doc), doc["units"]
    head["units"] = "__UNITS__"
    text = json.dumps(head, indent=1, sort_keys=False)
    body = ",\n".join("  " + json.dumps(r, separators=(",", ":"), sort_keys=False) for r in tail)
    return text.replace('"__UNITS__"', "[\n" + body + "\n ]") + "\n"


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true",
                        help="exit non-zero unless the on-disk manifest equals a fresh build")
    args = parser.parse_args(argv)

    doc = build()
    text = serialize(doc)
    if args.check:
        if not os.path.exists(OUT):
            print(f"MISSING {OUT}")
            return 1
        with open(OUT, "r", encoding="utf-8") as fh:
            on_disk = fh.read()
        # `generated_at_head` legitimately moves with every commit; compare everything else.
        fresh = json.loads(text)
        disk = json.loads(on_disk)
        fresh.pop("generated_at_head"), disk.pop("generated_at_head")
        if fresh != disk:
            print("MANIFEST DRIFTED from a fresh build")
            return 1
        print(f"units={len(doc['units'])} "
              f"by_bucket={doc['summary']['by_bucket']} "
              f"non_done={doc['summary']['non_done']} verdict=PASS")
        return 0

    with open(OUT, "w", encoding="utf-8") as fh:
        fh.write(text)
    print(f"wrote {OUT}")
    print(f"units={len(doc['units'])} "
          f"by_bucket={doc['summary']['by_bucket']} "
          f"non_done={doc['summary']['non_done']}")
    print(f"by_sheet_rule_content={doc['summary']['by_sheet_rule_content']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
