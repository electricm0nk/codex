#!/usr/bin/env python3
"""One-shot, narrowly-scoped regenerator: re-applies the FIXED
`scrub_name_pi_tokens` (`scripts/pi_scrub.py`) to every already-shipped
`codex_generated_name: true` record under `data/corpus/**/{race,monster,
class,race_trait}_generic/*.json`.

**Why not just re-run `ingest_generic_kind.py`?** That script's population is
gated on `join_status == "no_record"` against a shape-ledger snapshot -- by
construction, every unit it already wrote is no longer `no_record`, so a
plain re-run finds zero units and touches nothing. This driver instead walks
the records `ingest_generic_kind.py` already wrote, re-derives their
`raw_tokens` from the SAME pinned oracle citation those records already
carry (`source.path` + `source.line`, byte-identical re-read via the
module's own `read_row`/`row_tokens`), and re-runs ONLY the fixed
`scrub_name_pi_tokens` over them -- everything else (key, name, slug, file
path, license, wiring_class) is left untouched, so no slug-collision or
file-identity risk is introduced.

**Original identity for a renamed record.** The shipped record does not
(and per `decisions.md §24b`-1/-4 must not) carry the pre-rename name/key.
This driver re-derives it the same way the `§17a` cross-check did: joining
the record's own `rename.coordinate` (`book:source_basename:line`) against
`docs/work-inventory.json`'s `(book, source_file, source_line)` -- the exact
key `scripts/shape_ledger.py` already uses.

Run: `python3 scripts/regen_generic_kind_pi_scrub.py [--dry-run]`
`PCGEN_CORPUS_ROOT` must point at a pinned PCGen `data/` checkout.
"""
from __future__ import annotations

import json
import os
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))

import ingest_generic_kind as gen  # noqa: E402

INVENTORY_PATH = os.path.join(REPO_ROOT, "docs/work-inventory.json")
KIND_DIRS = ("race_generic", "monster_generic", "class_generic", "race_trait_generic")


def load_inventory_index() -> dict[tuple[str, str, int], dict]:
    with open(INVENTORY_PATH, encoding="utf-8") as fh:
        doc = json.load(fh)
    idx = {}
    for u in doc["units"]:
        idx[(u["book"], u["source_file"], u["source_line"])] = u
    return idx


def find_generic_records() -> list[str]:
    out = []
    for dirpath, _dirnames, filenames in os.walk(os.path.join(REPO_ROOT, "data/corpus")):
        if os.path.basename(dirpath) not in KIND_DIRS:
            continue
        for fn in filenames:
            if fn.endswith(".json"):
                out.append(os.path.join(dirpath, fn))
    return sorted(out)


def main() -> int:
    dry_run = "--dry-run" in sys.argv
    root = gen.corpus_root()
    if not os.path.isdir(root):
        print(f"PCGEN_CORPUS_ROOT ({root}) is not a directory", file=sys.stderr)
        return 1

    inv_idx = load_inventory_index()
    paths = find_generic_records()

    report = {
        "scanned": 0,
        "renamed_reprocessed": 0,
        "non_renamed_skipped": 0,
        "changed": 0,
        "unchanged": 0,
        "unresolved_coordinate": [],
    }

    for path in paths:
        with open(path, encoding="utf-8") as fh:
            rec = json.load(fh)
        report["scanned"] += 1

        if not rec.get("codex_generated_name"):
            # Not a renamed record -- `scrub_name_pi_tokens` was never
            # applied to it in the first place (its own name/key isn't PI),
            # so there is nothing for this driver to re-derive.
            report["non_renamed_skipped"] += 1
            continue

        coord = rec.get("rename", {}).get("coordinate", "")
        parts = coord.split(":")
        if len(parts) != 3:
            report["unresolved_coordinate"].append(path)
            continue
        book, source_basename, line_s = parts
        try:
            line = int(line_s)
        except ValueError:
            report["unresolved_coordinate"].append(path)
            continue

        unit = inv_idx.get((book, source_basename, line))
        if unit is None:
            report["unresolved_coordinate"].append(path)
            continue

        orig_name = unit["name"]
        orig_key = unit.get("corpus_key") or unit.get("key") or unit["name"]

        # Re-read the SAME cited row from the pinned oracle -- the record's
        # own `source.path` is already root-relative and byte-identical to
        # what `ingest_generic_kind.py` cited originally.
        src_path = os.path.join(root, rec["source"]["path"])
        src_line = rec["source"]["line"]
        raw_line = gen.read_row(src_path, src_line)
        tokens = gen.row_tokens(raw_line)

        scrubbed_tokens, extra_redacted = gen.scrub_name_pi_tokens(tokens, orig_name, orig_key)

        report["renamed_reprocessed"] += 1

        old_tokens = rec["data"]["raw_tokens"]
        if scrubbed_tokens != old_tokens:
            report["changed"] += 1
            rec["data"]["raw_tokens"] = scrubbed_tokens
            fields_redacted = (rec.get("pi_field") or "").split(",") if rec.get("pi_field") else []
            fields_redacted = [f for f in fields_redacted if f]
            if extra_redacted and "raw_tokens" not in fields_redacted:
                fields_redacted.append("raw_tokens")
            rec["pi_field"] = ",".join(fields_redacted) if fields_redacted else rec.get("pi_field")
            if not dry_run:
                with open(path, "w", encoding="utf-8") as fh:
                    json.dump(rec, fh, indent=2, ensure_ascii=False)
                    fh.write("\n")
        else:
            report["unchanged"] += 1

    print(json.dumps(report, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
