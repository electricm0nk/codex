#!/usr/bin/env python3
"""Every shipping-code line under a live root that names `pcgen_import`, with
the mechanism that holds it there -- `AT-35-E6-003-RULED`, operator ruling
B16 (2026-09-12).

Why this exists
---------------
B16 made the gate count what it had been missing: live code that calls
`src/pcgen_import::` at run time. The count jumped from `files=0 hits=0` under
`apps/desktop/` to a real number, and `pcgen_residue_gate.py --check` now
reports the honest figure. A total is not a work plan, though: the cycle's bar
is ZERO, and a call site that cannot go needs its file, its line and its
REASON written down, not a subtraction.

This script is that census. It re-derives the gate's own `pcgen_import` hits
-- by importing the gate, not by re-implementing it, so the two can never
disagree -- and groups each line by the converter symbol it names, which is
the unit of work: every hit in one group is cleared by one mechanism, and no
hit is cleared by less than its whole group.

Run
---
    python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle1_runtime_import_census.py

Nothing here mutates the repository, the gate, or the baseline.
"""

import collections
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", ".."))
sys.path.insert(0, os.path.join(ROOT, "scripts"))

import pcgen_residue_gate as G  # noqa: E402

# --- the mechanism groups --------------------------------------------------
# Ordered: the first pattern that matches a line names its group. Each entry
# is (group id, line regex, what the live side needs before the group can go).
GROUPS = [
    (
        "renderer",
        re.compile(r"pcgen_desc::"),
        "The PCGen description renderer called at run time on a corpus record's "
        "stored `description`. Needs the converted rule's own prose "
        "(`resolved_prose::render_description`) seeded from the values the caller "
        "already computes, gated by a corpus-wide parity test against the tool-side "
        "renderer. The mechanism is proved; `AT-35-E6-003-FINISH` cycle 3 did it for "
        "racial traits.",
    ),
    (
        "lst_parser_types",
        re.compile(r"lst_parser::"),
        "The ingest-format record STRUCTS (`EquipmentRecord`, `LstSpellRecord`, the "
        "class/metadata/race-ability/spellcasting rows) are declared on the converter "
        "side and used as the live side's own data type. Not a token read -- a type "
        "ownership fact. Needs the live side to own a converted equipment/spell record "
        "shape the converter emits into, which does not exist yet.",
    ),
    (
        "ingest_record_tokens",
        re.compile(r"ingest_record|ingest_payload"),
        "Run-time token-string reads: `token_pairs`, `bonus_chain_qualifiers`, "
        "`rebuild_bonus_token`, `token_count`, and the `RaceCacheData` / "
        "`RaceTraitCacheData` payload shapes. These are the literal ingest tokens the "
        "sheet rule forbids, reached through a function call so no token-syntax "
        "pattern fires. Needs the converted package to carry the same facts keyed by "
        "`VarId` rather than by source token name.",
    ),
    (
        "trait_and_pool_tokens",
        re.compile(r"race_trait_tokens|pool_member_tokens|bonus_chain_reader"),
        "Per-row token readers for racial traits, feature pools and bonus chains. The "
        "racial-trait half of this group is the one `AT-35-E6-003-FINISH` cycle 3 "
        "already proved removable; the remaining call sites are the pool/catalog half, "
        "which has no converted pool-member table yet.",
    ),
    (
        "ir_converter",
        re.compile(r"ir_converter|include_resolver|cache_gen"),
        "The converter's own conversion entry points invoked from live code at run "
        "time -- `convert_equipment_record`, `convert_spell_record`, the include "
        "resolver, and one `cache_gen::equipment_gap::RenameInfo` field type. The live "
        "side is running the conversion instead of reading its output. Needs the "
        "converted artefact to be produced at build time and read as data.",
    ),
    (
        "source_content_payload",
        re.compile(r"source_content_payload|equipment_bonus_reader"),
        "Payload and bonus-reader types re-exported or imported by the live side. "
        "Shape-only dependencies on converter-side declarations; they move when the "
        "converted equipment/source-content shape above exists.",
    ),
    (
        "provenance_prose",
        re.compile(r"`src/pcgen_import/|src/pcgen_import/"),
        "NOT a run-time read: the converter's module PATH written inside a string "
        "literal, as the provenance sentence a reach-gate row prints for where its "
        "words come from. B14 excused the same sentence in a `//` comment; in a string "
        "literal the gate counts it, correctly and deliberately -- widening the pattern "
        "to exempt string literals would be exactly the forbidden weakening. Clearing "
        "it is a prose edit, and it is left alone here on purpose: trimming it would "
        "move the total without moving any dependency.",
    ),
]
FALLBACK = (
    "unclassified",
    "No group pattern matched. Classify it before the next cycle -- an "
    "unclassified hit is an unowned one.",
)


def classify(text):
    for gid, rx, why in GROUPS:
        if rx.search(text):
            return gid, why
    return FALLBACK


def main():
    rx = G._COMPILED["pcgen_import"]
    rows = []
    for live_root, rel, abs_path in G._iter_live_source_files(ROOT):
        with open(abs_path, encoding="utf-8", errors="replace") as fh:
            lines = fh.read().splitlines()
        # The gate's own two exclusions, applied here so the real 1-based line
        # number survives: B14 comments, B15 `#[cfg(test)]` regions.
        skip = set()
        for a, b in G.cfg_test_ranges(lines):
            skip.update(range(a, b + 1))
        for i, line in enumerate(lines):
            if i in skip or line.lstrip().startswith("//"):
                continue
            n = len(rx.findall(line))
            if not n:
                continue
            gid, why = classify(line)
            rows.append(dict(root=live_root, file=rel, line=i + 1, hits=n,
                             group=gid, why=why, text=line.strip()[:200]))

    total = sum(r["hits"] for r in rows)
    assert total == G.scan(ROOT).hits_by_pattern["pcgen_import"], (
        "census disagrees with the gate it imports"
    )
    files = {r["file"] for r in rows}
    print(f"pcgen_import_hits={total} files={len(files)}")
    by_root = collections.Counter()
    for r in rows:
        by_root[r["root"]] += r["hits"]
    print("by_root=" + ", ".join(f"{k}={v}" for k, v in sorted(by_root.items())))
    print()
    by_group = collections.defaultdict(list)
    for r in rows:
        by_group[r["group"]].append(r)
    order = [g[0] for g in GROUPS] + [FALLBACK[0]]
    for gid in order:
        group = by_group.get(gid)
        if not group:
            continue
        hits = sum(r["hits"] for r in group)
        gfiles = sorted({r["file"] for r in group})
        print(f"=== {gid}: hits={hits} files={len(gfiles)} ===")
        print("  why it is still here: " + group[0]["why"])
        for r in sorted(group, key=lambda r: (r["file"], r["line"])):
            print(f"  {r['file']}:{r['line']}  {r['text']}")
        print()

    out = os.path.join(HERE, "AT-35-E6-003-RULED_cycle1_runtime_import_census.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump({"total_hits": total, "total_files": len(files),
                   "by_root": dict(by_root), "rows": rows}, fh, indent=1)
    print(f"census written to {os.path.relpath(out, ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
