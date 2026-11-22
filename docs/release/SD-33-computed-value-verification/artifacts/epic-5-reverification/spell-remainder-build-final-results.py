#!/usr/bin/env python3
"""SD-33 remediation wave 2 (`spell-remainder` slice) -- merges the real
oracle-compared `spell` rows (`fixture-compare-spell-batch.py`'s output,
reused unmodified) with the pre-classified `unverifiable` rows
(`fixture_verified_oracle_probe --remainder`'s own output) into the final
committed per-unit result set, `spell-remainder.oracle-results.json`.

Both inputs are real: the `spell` rows carry a live PCGen oracle comparison
verdict; the `unverifiable` rows each carry a real, per-unit reason derived
from a live compute_spellbook_coverage attempt (never a guess) --
`fixture_verified_oracle_probe.rs --remainder`'s own doc comment.

`fixture-compare-spell-batch.py`'s own `compare_unit` (`AT-33-E2-003`,
unmodified) normalizes BOTH "the (level,name) key is entirely absent from
the export" and "the key is present with a blank DC token" to the same
`oracle: None` on an `unverifiable` record -- by design, for its own
population, where the two never needed distinguishing. This cycle found
they must be: a key entirely absent from the export means PCGen's own
`BatchExporter` DROPPED the whole `SPELLNAME` line for the level this
cycle declared (real behavior, confirmed in `AT-33-E5-001`'s own Notes: a
wrong `SPELLLEVEL` gets silently dropped, never relocated) -- a genuine
signal that this engine's per-school generic table
(`crb::spell_list::SPELL_LIST`, read by `resolve_<school>_spell_effect`)
and its per-class table (e.g. `wizard_spell_list`) disagree about that
spell's real level, a live candidate root cause for `AT-33-E5-003`, not
an ordinary "no save" absence. A key present with a blank DC is the
ordinary case: the spell resolved at the declared level and PCGen simply
computed no DC for it (no saving throw). This script re-parses each raw
export directly (`fixture-compare-spell-batch.py`'s own `parse_class_export`,
imported unmodified) to tell the two apart per unit.
"""
from __future__ import annotations

import importlib.util
import json
import sys
from pathlib import Path

_FCSB_PATH = Path(__file__).resolve().parent / "fixture-compare-spell-batch.py"
_spec = importlib.util.spec_from_file_location("fcsb", _FCSB_PATH)
fcsb = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(fcsb)


def main() -> int:
    if len(sys.argv) != 5:
        print(
            "usage: spell-remainder-build-final-results.py <probe-output.json> "
            "<compared-spell-results.json> <oracle-txt-dir> <output.json>",
            file=sys.stderr,
        )
        return 2
    probe_path = Path(sys.argv[1])
    compared_path = Path(sys.argv[2])
    oracle_dir = Path(sys.argv[3])
    output_path = Path(sys.argv[4])

    probe = json.loads(probe_path.read_text())
    compared = json.loads(compared_path.read_text())

    # Rebuild the same per-class raw lookup fixture-compare-spell-batch.py
    # used, so a genuinely-absent key can be told apart from a
    # present-but-blank one (both collapse to `oracle: None` in `compared`).
    raw_lookup_by_class: dict[str, dict] = {}
    for row in probe["spell"]:
        cls = row["class_human"]
        if cls not in raw_lookup_by_class:
            export_path = oracle_dir / f"{cls.lower()}.export.txt"
            raw_lookup_by_class[cls] = fcsb.parse_class_export(export_path.read_text())
    probe_spell_by_id = {r["unit_id"]: r for r in probe["spell"]}

    records = []
    for r in compared["results"]:
        rec = {
            "unit_id": r["unit_id"],
            "ours": r["ours"],
            "oracle": r["oracle"],
            "verdict": r["verdict"],
            "kind": "spell",
        }
        if r["verdict"] == "unverifiable":
            src = probe_spell_by_id[r["unit_id"]]
            key = (src["level"], src["name"])
            lookup = raw_lookup_by_class[src["class_human"]]
            if key not in lookup:
                rec["reason"] = (
                    f"oracle_export_dropped_declared_level: PCGen's own BatchExporter "
                    f"produced NO SPELL.*.NAME entry at all for '{src['name']}' at level "
                    f"{src['level']} (class {src['class_human']}) -- this engine declared "
                    f"that level via the per-school generic table "
                    f"(compute_spellbook_coverage's resolved SpellEffect.level), and a "
                    f"wrongly-declared level is silently dropped by PCGen rather than "
                    f"relocated (confirmed empirically, AT-33-E5-001's own Notes); a real, "
                    f"named candidate root cause for AT-33-E5-003 (the per-school generic "
                    f"table and this class's own per-class spell-list table may disagree "
                    f"about this spell's real level) -- not an ordinary no-save absence"
                )
            else:
                rec["reason"] = (
                    f"no_save_dc_on_oracle: PCGen's own BatchExporter resolved "
                    f"'{src['name']}' at level {src['level']} (class {src['class_human']}) "
                    f"but produced a blank SPELL.*.DC token for it -- a real, live-confirmed "
                    f"absence of a saving throw on the oracle side"
                )
        records.append(rec)

    for r in probe["unverifiable"]:
        records.append(
            {
                "unit_id": r["unit_id"],
                "ours": None,
                "oracle": None,
                "verdict": "unverifiable",
                "kind": r["kind"],
                "reason": r["reason"],
            }
        )

    from collections import Counter

    counts = Counter(r["verdict"] for r in records)
    reasonless = sum(1 for r in records if r["verdict"] == "unverifiable" and not r.get("reason"))

    output_path.write_text(json.dumps({"results": records}, indent=2) + "\n")
    print(
        f"spell-remainder-build-final-results: {len(records)} total -- "
        f"agree={counts['agree']} disagree={counts['disagree']} "
        f"unverifiable={counts['unverifiable']} reasonless_unverifiable={reasonless} -> {output_path}",
        file=sys.stderr,
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
