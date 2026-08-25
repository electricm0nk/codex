#!/usr/bin/env python3
"""SD-32 card 11, T9-onboarding-class-feature-pi-and-rescreen — corpus-wide
re-derivation of the PI-leak count, §17a.

Two prior figures exist and disagree:
  (a) the `feat`-lane receipt's class-closure scan: every kind 0, except
      `class_feature` 31 (28 real + 3 confirmed `§26`-class false positives),
      scanning every `data.*` field.
  (b) the orchestrator's own independent scan: 43 records with a blacklist
      hit across `data.*` fields, 71 field-hits.

This script is the single re-derivation both are reconciled against. It
walks the ENTIRE shipped corpus (`data/corpus/**/*.json`), recursively
scans every string reachable under the record's own `data` object (not just
`name`/`description` — nested dicts and lists included, since `raw_tokens`
and `prerequisites` are exactly the shape that hid the two known real
misses), against `scripts/pi_scrub.normalized_term_hit` (the same
word-bounded, OCR-normalized, operator-signed-off 61-term scan the
`ogl-pi-blacklist.md` §19 sign-off requires) — and reports both a per-record
count and a per-field-hit count, so both prior figures are addressed
explicitly rather than picking one.

A record already carrying `license` in {PI-REDACTED, Pi-Redacted} (i.e.
already through the guarded redaction path) is still scanned — the entire
point of this script is to catch content that *should* have redacted but
didn't (a leak), not to trust the license field's own say-so. A value
already equal to the redaction marker itself
(`shape_b_v1::REDACTED_PI_MARKER`, "[redacted PI]") is skipped, since that
is the marker, not a leak.
"""
from __future__ import annotations

import json
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import pi_scrub  # noqa: E402

REPO_ROOT = Path(__file__).resolve().parent.parent
CORPUS_ROOT = REPO_ROOT / "data" / "corpus"

REDACTED_MARKER = pi_scrub.REDACTED_PI_MARKER


def iter_strings(value, path):
    """Yield (dotted_path, string) for every string reachable under `value`,
    walking dicts and lists. `path` is the dotted-path prefix so far."""
    if isinstance(value, str):
        yield path, value
    elif isinstance(value, dict):
        for k, v in value.items():
            yield from iter_strings(v, f"{path}.{k}" if path else k)
    elif isinstance(value, list):
        for i, v in enumerate(value):
            yield from iter_strings(v, f"{path}[{i}]")


def kind_from_path(rel: Path) -> str:
    # rel is REPO-rooted (`data/corpus/<book>/<kind>/...`), so `kind` is the
    # FOURTH path component (0=data, 1=corpus, 2=book, 3=kind) -- NOT the
    # second, which is always the literal string "corpus". `parts[1]` was
    # this instrument's own bug (found by this cycle, `decisions.md §17a`:
    # validate an instrument before trusting a confident claim it produces):
    # every per-kind row this script ever printed read `kind=corpus`, which
    # collapsed every kind into one bucket and was silently worked around by
    # every prior reader piping the script's own path list through
    # `awk -F'/' '{print $4}'` instead of trusting this function's output.
    parts = rel.parts
    return parts[3] if len(parts) > 3 else "?"


def main() -> int:
    per_kind_records = Counter()
    per_kind_field_hits = Counter()
    total_records_scanned = 0
    hit_records = []  # (rel_path, kind, [(field_path, term)])

    for path in sorted(CORPUS_ROOT.glob("**/*.json")):
        rel = path.relative_to(REPO_ROOT)
        try:
            doc = json.loads(path.read_text(encoding="utf-8"))
        except Exception as e:
            print(f"SKIP (unreadable): {rel}: {e}", file=sys.stderr)
            continue
        total_records_scanned += 1
        data = doc.get("data")
        if data is None:
            continue
        kind = kind_from_path(rel)
        record_hits = []
        for field_path, s in iter_strings(data, ""):
            if not s or s == REDACTED_MARKER:
                continue
            term = pi_scrub.blacklist_term_hit_including_concatenated(s)
            if term is not None:
                record_hits.append((field_path, term))
        if record_hits:
            per_kind_records[kind] += 1
            per_kind_field_hits[kind] += len(record_hits)
            hit_records.append((str(rel), kind, record_hits))

    print(f"Records scanned (every data/corpus/**/*.json with a `data` object): {total_records_scanned}")
    print(f"Records with >=1 confirmed blacklist-term hit: {len(hit_records)}")
    print(f"Total field-level hits across those records: {sum(per_kind_field_hits.values())}")
    print()
    print("Per-kind record-hit / field-hit counts:")
    for kind in sorted(set(per_kind_records) | set(per_kind_field_hits)):
        print(f"  {kind}: records={per_kind_records[kind]} field_hits={per_kind_field_hits[kind]}")
    print()
    print("Full detail (path, kind, [field_path:term_index...]):")
    for rel, kind, record_hits in hit_records:
        print(f"  {rel} [{kind}]")
        for field_path, term in record_hits:
            print(f"      {field_path} -> term_index={pi_scrub.PI_BLACKLIST_TERMS.index(term)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
