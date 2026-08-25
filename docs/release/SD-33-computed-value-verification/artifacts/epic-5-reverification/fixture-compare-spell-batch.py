#!/usr/bin/env python3
"""SD-33 AT-33-E5-001 remediation -- joins the batch `.pcg` oracle exports
(`fixture-generate-spell-batch.py`'s output, run through PCGen) back to the
`ours` values (`fixture_verified_oracle_probe`'s output) by `(level, name)`
within each class's own export text, then reuses `oracle_harness.compare`
(`AT-33-E2-003`, unmodified) for the actual per-unit verdict -- this file is
a JOIN layer in front of the proven harness, not a fork of it.

Why a name-based join and not `run.py`'s direct oracle-export-key lookup:
this cycle's batch export loops PCGen's own internal `class`/`spell` array
indices, which this cycle found are NOT guaranteed to land on `class=0`
(Paladin landed on `class=1`) and are not otherwise predictable ahead of
the export -- so the join happens AFTER the real export exists, matching by
the data the export and the `ours` side both carry: spell level + name.
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[5]))
from scripts.oracle_harness import compare as OC


def parse_class_export(text: str) -> dict[tuple[int, str], str]:
    """Returns {(level, name): dc} for every `SPELL.<class>.<level>.<idx>.NAME`
    / `.DC` pair line the export text carries, keyed by (level, name) --
    class/idx are discarded (position, not identity)."""
    names: dict[tuple[int, int, int], str] = {}
    dcs: dict[tuple[int, int, int], str] = {}
    for raw in text.splitlines():
        line = raw.strip()
        if not line or "=" not in line:
            continue
        key, _, value = line.partition("=")
        parts = key.split(".")
        if len(parts) != 5 or parts[0] != "SPELL":
            continue
        _, cls, level, idx, field = parts
        pos = (int(cls), int(level), int(idx))
        if field == "NAME":
            names[pos] = value
        elif field == "DC":
            dcs[pos] = value
    out: dict[tuple[int, str], str] = {}
    for pos, name in names.items():
        _, level, _ = pos
        # A (level, name) pair could in principle repeat within one export
        # (e.g. innate + known copies of a cantrip) -- last-write-wins is
        # fine here since every real DC for the SAME spell at the SAME
        # level is the same number by construction (10 + level + fixed
        # ability modifier, both sides pinned).
        out[(level, name)] = dcs.get(pos, "")
    return out


def main() -> int:
    if len(sys.argv) != 4:
        print(
            "usage: fixture-compare-spell-batch.py <probe-output.json> <oracle-txt-dir> <output.json>",
            file=sys.stderr,
        )
        return 2
    probe_path = Path(sys.argv[1])
    oracle_dir = Path(sys.argv[2])
    output_path = Path(sys.argv[3])

    data = json.loads(probe_path.read_text())
    spells = data["spell"]

    class_lookups: dict[str, dict[tuple[int, str], str]] = {}
    records = []
    for row in spells:
        class_human = row["class_human"]
        if class_human not in class_lookups:
            export_path = oracle_dir / f"{class_human.lower()}.export.txt"
            class_lookups[class_human] = parse_class_export(export_path.read_text())
        lookup = class_lookups[class_human]
        oracle_dc = lookup.get((row["level"], row["name"]))
        rec = OC.compare_unit(row["unit_id"], row["ours_dc"], oracle_dc)
        rec["class_human"] = class_human
        rec["level"] = row["level"]
        rec["name"] = row["name"]
        records.append(rec)

    from collections import Counter

    counts = Counter(r["verdict"] for r in records)
    output_path.write_text(json.dumps({"results": records}, indent=2) + "\n")
    print(
        f"fixture-compare-spell-batch: {len(records)} units -- "
        f"agree={counts['agree']} disagree={counts['disagree']} "
        f"unverifiable={counts['unverifiable']} -> {output_path}",
        file=sys.stderr,
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
