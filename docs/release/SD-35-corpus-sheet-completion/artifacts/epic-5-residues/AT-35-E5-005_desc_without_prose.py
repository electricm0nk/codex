"""AT-35-E5-005's residue: units whose corpus record carries a real `DESC` token whose words
never reach a sheet line.

    python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-005_desc_without_prose.py
    python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-005_desc_without_prose.py --check   # exits 1 while the count is above 0

Why this exists. AT-35-E5-005 is the closure-accounting cycle: it proves the corpus is at 49,438
of 49,438 and writes the per-unit manifest behind that number. Writing the manifest's
`sheet_rule_content` column surfaced a shape no bucket names -- a rule that renders
`sheet_rule_rendered:words` while carrying no words of its own. Almost all of those are correct
(`decisions.md §1`: the corpus record has no description upstream, so the finished sheet line is
the feature's NAME, which is what a player writes on paper). This script isolates the ones that
are NOT correct: the record HAS a `DESC` token with real published rules text, and the converted
`SheetRule` has no prose and no granting rule that carries any either. For those the sheet prints
a bare label where the rulebook prints a paragraph, so the unit's `sheet-complete` stamp is not
the sheet rule satisfied.

The fix is on the CONVERTER side (`src/pcgen_import/sheet_rule/`, `decisions.md §11`) and that
path is outside Epic 5's file-touch set (`workflow-instruction.md §3`), so AT-35-E5-005 measures
and reports it rather than editing another epic's files. `--check` is the control that keeps it
from being forgotten: it exits non-zero while the count is above zero, so the cycle that fixes
the converter has a red gate to turn green (`AGENTS.md` rule 8 -- a mechanism, not a caution).

Three narrowing steps, so the number is not an over-count:

  1. only units whose `SheetRule` carries NO prose pieces and no principal value are considered;
  2. the corpus record is joined on `(book, kind, corpus_key)` and, where a key repeats, on the
     unit's own `source_line` -- a shared name is not a shared thing
     (`corpus-identifier-scope-collisions`), and joining on `(book, key)` alone over-counts by
     matching a different kind's record;
  3. `[redacted PI]` descriptions are excluded (they are correctly absent from the package,
     `decisions.md §15` R2), and a pointer row is excluded when the rule named in its
     `granted_by` does carry prose -- the words reach the sheet through the granting rule.
"""
import argparse
import collections
import json
import os
import subprocess
import sys

REPO = subprocess.run(["git", "rev-parse", "--show-toplevel"],
                      capture_output=True, text=True, check=True).stdout.strip()
OUT = os.path.join(REPO, "docs/release/SD-35-corpus-sheet-completion/artifacts/"
                         "epic-5-residues/desc-without-prose.json")


def index_sheet_rules() -> dict:
    by_id = {}
    for dirpath, _dn, filenames in os.walk(os.path.join(REPO, "data/sheet_rules")):
        for name in filenames:
            if not name.endswith(".json") or name.startswith("_"):
                continue
            with open(os.path.join(dirpath, name), "r", encoding="utf-8") as fh:
                try:
                    rules = json.load(fh)
                except json.JSONDecodeError:
                    continue
            if isinstance(rules, list):
                for rule in rules:
                    if isinstance(rule, dict) and "id" in rule:
                        by_id.setdefault(rule["id"], rule)
    return by_id


def index_corpus() -> dict:
    """`{(book, kind, key): [record, ...]}` over the live corpus."""
    out = collections.defaultdict(list)
    root = os.path.join(REPO, "data/corpus")
    for dirpath, _dn, filenames in os.walk(root):
        rel = os.path.relpath(dirpath, root).split(os.sep)
        if len(rel) < 2:
            continue
        book, kind = rel[0], rel[1]
        for name in filenames:
            if not name.endswith(".json"):
                continue
            with open(os.path.join(dirpath, name), "r", encoding="utf-8") as fh:
                try:
                    record = json.load(fh)
                except json.JSONDecodeError:
                    continue
            data = record.get("data") if isinstance(record, dict) else None
            if not isinstance(data, dict) or data.get("key") is None:
                continue
            tokens = [t.get("key") for t in (data.get("raw_tokens") or []) if isinstance(t, dict)]
            out[(book, kind, data["key"])].append({
                "path": os.path.relpath(os.path.join(dirpath, name), REPO),
                "description": data.get("description") or "",
                "has_desc_token": "DESC" in tokens,
                "line": (record.get("source") or {}).get("line"),
            })
    return out


def prose_pieces(rule) -> int:
    return sum(len(f.get("pieces") or []) for f in (rule.get("prose") or []))


def find() -> dict:
    units = json.load(open(os.path.join(REPO, "docs/work-inventory.json"),
                           encoding="utf-8"))["units"]
    rules = index_sheet_rules()
    corpus = index_corpus()

    findings = []
    for unit in units:
        rule = rules.get(unit["id"])
        if rule is None:
            continue
        if prose_pieces(rule) or rule.get("value") != "Text":
            continue                                                    # step 1
        records = corpus.get((unit["book"], unit["kind"], unit.get("corpus_key"))) or []
        same_line = [r for r in records if r["line"] == unit.get("source_line")] or records
        real = [r for r in same_line
                if r["has_desc_token"]
                and r["description"].strip()
                and r["description"].strip() != "[redacted PI]"]        # steps 2 and 3
        if not real:
            continue
        granters = [(g.get("by") or {}).get("Rule") for g in (rule.get("granted_by") or [])]
        if any(gid and rules.get(gid) and prose_pieces(rules[gid]) for gid in granters):
            continue                                                    # step 3, pointer rows
        findings.append({
            "id": unit["id"],
            "book": unit["book"],
            "kind": unit["kind"],
            "status": unit.get("status"),
            "evidence": unit.get("evidence"),
            "corpus_record": real[0]["path"],
            "corpus_description_chars": len(real[0]["description"]),
            "granted_by": [g for g in granters if g],
        })

    findings.sort(key=lambda f: f["id"])
    return {
        "criterion": "AT-35-E5-005",
        "what": ("units whose corpus record carries a real DESC token whose words never reach a "
                 "sheet line -- the sheet prints a bare label where the rulebook prints a "
                 "paragraph"),
        "fix_side": "converter (src/pcgen_import/sheet_rule/), outside Epic 5's file-touch set",
        "re_derive_command": ("python3 docs/release/SD-35-corpus-sheet-completion/artifacts/"
                              "epic-5-residues/AT-35-E5-005_desc_without_prose.py"),
        "count": len(findings),
        "by_kind": dict(sorted(collections.Counter(f["kind"] for f in findings).items())),
        "by_status": dict(sorted(collections.Counter(f["status"] for f in findings).items())),
        "with_a_granting_rule_that_also_has_no_prose": sum(1 for f in findings if f["granted_by"]),
        "units": findings,
    }


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true",
                        help="exit 1 while any unit still loses its words on the sheet")
    args = parser.parse_args(argv)
    doc = find()
    with open(OUT, "w", encoding="utf-8") as fh:
        fh.write(json.dumps(doc, indent=1) + "\n")
    print(f"desc_token_with_real_text_but_no_prose_on_the_sheet={doc['count']} "
          f"of {49438} units  by_kind={doc['by_kind']}  "
          f"verdict={'PASS' if doc['count'] == 0 else 'RESIDUE'}")
    if args.check and doc["count"]:
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
