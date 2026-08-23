#!/usr/bin/env python3
"""One-shot, corpus-wide regenerator: re-applies the FIXED `scrub_name_pi_tokens`
(`scripts/pi_scrub.py`) to every already-shipped `codex_generated_name: true`
record anywhere under `data/corpus/**`, regardless of which ingest path wrote
it or where in the tree it lives.

**Why this exists in addition to `regen_generic_kind_pi_scrub.py`.** That
driver is scoped to `{race,monster,class,race_trait}_generic/` because that
was `ingest_generic_kind.py`'s own output surface. Since then, other cycles
(concurrent with and after this one) shipped MORE `codex_generated_name: true`
records via OTHER ingest paths (`deity`, `class_feature`, ...), some using
non-`<kind>_generic`-shaped directories (a `class_feature` unit can land
either directly under `class_feature/codex_named_unit_.../` or nested one
level deeper under a class-name subdirectory, e.g. `class_feature/rogue/
codex_named_unit_....json`). A full-corpus scan (this cycle's own `§17a`
re-derivation) found genuine identity-concatenation leaks in these newer
records too -- generated before this cycle's `pi_scrub.py` fix landed and
never regenerated since. This driver is kind-agnostic and directory-agnostic:
it walks the WHOLE `data/corpus/` tree, finds every `codex_generated_name:
true` record, and re-derives its `raw_tokens` from the record's own already-
cited `(source.path, source.line)` -- unconditionally safe because it never
computes a new file path, slug, key, or name; only `data.raw_tokens` and
`pi_field` may change.

**Original identity for a renamed record**, same join as
`regen_generic_kind_pi_scrub.py`: `rename.coordinate` (`book:source_basename:
line`) against `docs/work-inventory.json`'s `(book, source_file, source_line)`.

Run: `python3 scripts/regen_all_renamed_pi_scrub.py [--dry-run]`
`PCGEN_CORPUS_ROOT` must point at a pinned PCGen `data/` checkout.
"""
from __future__ import annotations

import json
import os
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))

from ingest_ability import corpus_root, read_row, row_tokens  # noqa: E402
from pi_scrub import scrub_name_pi_tokens  # noqa: E402

INVENTORY_PATH = os.path.join(REPO_ROOT, "docs/work-inventory.json")


def load_inventory_index() -> dict[tuple[str, str, int], dict]:
    with open(INVENTORY_PATH, encoding="utf-8") as fh:
        doc = json.load(fh)
    idx = {}
    for u in doc["units"]:
        idx[(u["book"], u["source_file"], u["source_line"])] = u
    return idx


def find_renamed_records() -> list[str]:
    out = []
    for dirpath, _dirnames, filenames in os.walk(os.path.join(REPO_ROOT, "data/corpus")):
        for fn in filenames:
            if fn.endswith(".json"):
                out.append(os.path.join(dirpath, fn))
    return sorted(out)


def main() -> int:
    dry_run = "--dry-run" in sys.argv
    root = corpus_root()
    if not os.path.isdir(root):
        print(f"PCGEN_CORPUS_ROOT ({root}) is not a directory", file=sys.stderr)
        return 1

    inv_idx = load_inventory_index()
    paths = find_renamed_records()

    report = {
        "scanned": 0,
        "renamed_reprocessed": 0,
        "non_renamed_skipped": 0,
        "changed": 0,
        "unchanged": 0,
        "unresolved_coordinate": [],
        "changed_paths": [],
    }

    for path in paths:
        with open(path, encoding="utf-8") as fh:
            try:
                rec = json.load(fh)
            except (OSError, json.JSONDecodeError):
                continue
        if not isinstance(rec, dict) or "codex_generated_name" not in rec:
            continue
        report["scanned"] += 1

        if not rec.get("codex_generated_name"):
            report["non_renamed_skipped"] += 1
            continue

        coord = (rec.get("rename") or {}).get("coordinate", "")
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

        src_path = os.path.join(root, rec["source"]["path"])
        src_line = rec["source"]["line"]
        raw_line = read_row(src_path, src_line)
        tokens = row_tokens(raw_line)

        scrubbed_tokens, extra_redacted = scrub_name_pi_tokens(tokens, orig_name, orig_key)
        report["renamed_reprocessed"] += 1

        old_tokens = rec.get("data", {}).get("raw_tokens")
        if scrubbed_tokens != old_tokens:
            report["changed"] += 1
            report["changed_paths"].append(os.path.relpath(path, REPO_ROOT))
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
