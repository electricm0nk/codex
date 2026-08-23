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

**T9-onboarding-cause-closure (2026-08-23, row 17's remaining 21) fix: this
driver had the SAME DESC-blanking gap `regen_row17_pi_over_redaction.py`
caught in its own first draft** (that script's module docstring, "Near-miss
found and fixed live in this cycle"). This one re-derived `data.raw_tokens`
via `scrub_name_pi_tokens` ALONE -- omitting the declared-PI DESC-blanking
and blacklist-scan steps `ingest_generic_kind.py::remediate` performs first
-- so a record whose `DESC` prose does not happen to literally contain its
own PI name/key (an ordinary narrative sentence, not a restatement of the
key) would ship its FULL, un-redacted description text in `data.raw_tokens`
even when `DESCISPI:YES` declares it PI. Fixed by importing and calling the
SAME canonical pipeline (`regen_row17_pi_over_redaction.redact_tokens`) that
cycle wrote and mutation-proved, rather than re-implementing a second,
divergent copy -- closing the exact "one path screens, another doesn't"
duplication-drift shape `decisions.md §17` names. Also now threads this
record's own `§24` neutral name into the narrower (non-full-wipe)
self-reference redaction the same fix added.

Run: `python3 scripts/regen_all_renamed_pi_scrub.py [--dry-run]`
`PCGEN_CORPUS_ROOT` must point at a pinned PCGen `data/` checkout.
"""
from __future__ import annotations

import json
import os
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))

from ingest_ability import corpus_root, read_row  # noqa: E402
from codex_neutral_name import neutral_name as codex_neutral_name  # noqa: E402
from pi_scrub import REDACTED_PI_MARKER  # noqa: E402
from regen_row17_pi_over_redaction import redact_tokens  # noqa: E402

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
        # This record already carries a `§24` Codex-generated identity
        # (checked above) -- re-derive the SAME coordinate-only name (never
        # re-mint a new one) so a self-reference-only value narrows to it
        # instead of being wiped whole.
        neutral_name_hint = codex_neutral_name(unit["kind"], book, source_basename, line)

        src_path = os.path.join(root, rec["source"]["path"])
        src_line = rec["source"]["line"]
        raw_line = read_row(src_path, src_line)

        # T9-onboarding-cause-closure (2026-08-23, row 17's remaining 21):
        # the SAME canonical pipeline `regen_row17_pi_over_redaction.py`
        # mutation-proved (declared-PI detection -> DESC blanking ->
        # blacklist scan -> identity/blacklist scan), never a re-derivation
        # of only the identity/blacklist scan alone. This is what closes
        # the DESC-PI-SHIPPED-IN-RAW-TOKENS gap this driver previously had.
        scrubbed_tokens, stored_description, extra_redacted = redact_tokens(
            raw_line, orig_name, orig_key, neutral_name_hint=neutral_name_hint
        )
        report["renamed_reprocessed"] += 1

        old_tokens = rec.get("data", {}).get("raw_tokens")
        old_description = rec.get("data", {}).get("description")
        if scrubbed_tokens != old_tokens or stored_description != old_description:
            report["changed"] += 1
            report["changed_paths"].append(os.path.relpath(path, REPO_ROOT))
            rec["data"]["raw_tokens"] = scrubbed_tokens
            rec["data"]["description"] = stored_description
            fields_redacted = (rec.get("pi_field") or "").split(",") if rec.get("pi_field") else []
            fields_redacted = [f for f in fields_redacted if f]
            if stored_description == REDACTED_PI_MARKER and "description" not in fields_redacted:
                fields_redacted.append("description")
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
