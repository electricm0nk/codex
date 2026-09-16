#!/usr/bin/env python3
"""AT-35-E6-003-RULED cycle 7 — the run-time ``pcgen_import`` census, re-derived.

Cycle 6's census is imported whole (which imports cycle 5's, … back to cycle 1's): the
``GROUPS`` table, ``classify``, and the two exclusions the gate itself applies (B14 comment
lines, B15 ``#[cfg(test)]`` regions) stay single-sourced. **The gate script and
``scripts/pcgen-residue-baseline.env`` are absent from this cycle's diff** — no path
exempted, no regex weakened, no rebaseline.

What cycle 7 moved
------------------

One hit, one whole file: ``src/rules_core/skinwalker_change_shape.rs``. It obtained a
record's automatic grants from ``RaceTraitRecord::automatic_trait_grants`` — a live accessor
— and then imported ``pcgen_import::race_trait_tokens`` to strip the ingest format's own
pool-name prefix off each of them. The question it was asking (*which kin pool does this
record own?*) is a rules question; only the prefix-stripping grammar was the converter's.
That grammar stays on the converter side and is KEPT (``decisions.md`` §11); the answer is
now ``RaceTraitRecord::skinwalker_change_shape_kin``, declared beside the grants it derives
from. This is cycle 4's ARG-picker move (``exclusion_guard_flags``) applied to the one other
caller that had the same shape. Pinned over the LIVE ``bestiary_5`` Skinwalker population,
with the previous reading recomputed in-test as the oracle, so the accessor and the grammar
cannot drift silently.

What cycle 7 MEASURED, and the correction it forced
---------------------------------------------------

``trait_and_pool_tokens``' remaining ``class_feature_pool_catalog`` hit carried, from cycle
6, a *named converter defect* as its reason: *"PCGen's ``ABILITY:`` token maps to
``MapsTo::Applies``, i.e. to a prerequisite"*, so an ``ABILITY``-bearing record was said to
convert byte-identically to a prose-only one.

**That sentence is wrong.** ``mapping-table.v1.json``'s own ``ABILITY`` row states the token
is *"a GRANT edge: the rule <target> … gets ``granted_by += Grant{by: Rule(H)}``"*, and
``src/pcgen_import/sheet_rule/convert.rs:1266`` implements exactly that, pushing to
``out.grants_out``, which ``sheet_rule/mod.rs:691-732`` folds onto the **target** rule —
a different record id, in a different file. Cycle 6's probe read only
``rules_for_closure_row(this record)``, so it was structurally incapable of seeing the
converted form of the token it named. Worked example, the one cycle 6 cited:
``core_rulebook/feat/improved_unarmed_strike.json`` carries
``{"by": {"Rule": "occult_adventures:class_feature:elemental_ascetic_elemental_flurry"}}``
in its ``granted_by``. The edge converted. It points the other way.

``BONUS:`` and ``DEFINE:`` were unread for the same reason at a different address: they land
as contributor rows in ``data/sheet_rules/_vars/<var>.json``, which cite the contributing
``rule_id``.

Corrected probe (reverse-grant index + var-contributor index + a fixed ``Holds`` traversal —
the serialized shape is ``{"Holds": {"what": {"Rule": id}, "count": n}}``, two levels down):

* ``P1 has_no_engine_effect_token``: **6,494 → 1,870** of 18,043.
* ``P2 is_archetype_locked``: 919 → **864**, and the direction flipped — every disagreement
  is now the converted side refusing a row the ingest guard admits, because
  ``is_archetype_locked`` reads ``PREABILITY`` tokens only and this corpus also writes
  ``CATEGORY=Archetype`` inside a ``PREMULT`` wrapper.
* ``P3 carries_more_than_one_desc_segment``: 89, unchanged.

The swap stays **REFUSED on the number** — but on a different, smaller, correctly-attributed
number, and the converter is no longer under the accusation. Re-derive:
``AT-35-E6-003-RULED_cycle7_pool_guard_parity.py``.
"""

import collections
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", ".."))
sys.path.insert(0, os.path.join(ROOT, "scripts"))

import pcgen_residue_gate as G  # noqa: E402

_spec = importlib.util.spec_from_file_location(
    "at35_e6_003_ruled_cycle6_census",
    os.path.join(HERE, "AT-35-E6-003-RULED_cycle6_runtime_import_census.py"),
)
C6 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(C6)

TRAIT_AND_POOL_WHY_CYCLE7 = (
    "Per-row token readers for racial traits, feature pools and bonus chains. CYCLE 7 "
    "cleared `skinwalker_change_shape.rs` whole: it asked which kin pool a record owns and "
    "imported the converter only to strip the ingest pool-name prefix off grants it already "
    "had from a live accessor; `RaceTraitRecord::skinwalker_change_shape_kin` answers it "
    "now, pinned over the live bestiary_5 population against the previous reading as "
    "oracle. CYCLE 7 also RE-MEASURED the `class_feature_pool_catalog` hit and CORRECTED "
    "cycle 6's reason for it: `ABILITY:` does NOT map to a prerequisite -- "
    "`sheet_rule/convert.rs:1266` converts it as a GRANT EDGE exactly as "
    "`mapping-table.v1.json` specifies, folded onto the TARGET rule's `granted_by` in a "
    "different file, and `BONUS:`/`DEFINE:` land as `_vars/<var>.json` contributor rows "
    "citing their `rule_id`. Cycle 6's probe read neither. Corrected: P1 disagrees on 1,870 "
    "of 18,043 (was 6,494), P2 on 864 (direction flipped -- `is_archetype_locked` reads "
    "`PREABILITY` only and misses `CATEGORY=Archetype` nested in a `PREMULT`), P3 on 89. "
    "STILL REFUSED, on the number, not on effort. `race_resolver`'s two hits are the home "
    "of this grammar's live consumer and ride on the race-trait rule shape this epic's "
    "remaining piece produces. Re-derive: `python3 docs/release/"
    "SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/"
    "AT-35-E6-003-RULED_cycle7_pool_guard_parity.py`."
)

_REWRITTEN = {"trait_and_pool_tokens": TRAIT_AND_POOL_WHY_CYCLE7}

GROUPS = [(gid, rx, _REWRITTEN.get(gid, why)) for gid, rx, why in C6.GROUPS]
FALLBACK = C6.FALLBACK


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
    files = {r["file"] for r in rows}
    print(f"pcgen_import_hits={total} files={len(files)}")
    by_root = collections.Counter()
    for r in rows:
        by_root[r["root"]] += r["hits"]
    print("by_root=" + ", ".join(f"{k}={v}" for k, v in sorted(by_root.items())))
    desktop = by_root.get("apps/desktop", 0)
    print(f"apps_desktop_hits={desktop}  "
          f"evidence_sentence_met={'YES' if desktop == 0 else 'NO'}")
    print()

    by_group = collections.defaultdict(list)
    for r in rows:
        by_group[r["group"]].append(r)
    order = [g[0] for g in GROUPS] + [FALLBACK[0]]
    sizes = {}
    for gid in order:
        group = by_group.get(gid)
        sizes[gid] = sum(r["hits"] for r in group) if group else 0
        if not group:
            print(f"=== {gid}: hits=0 files=0 === CLEARED")
            print()
            continue
        gfiles = sorted({r["file"] for r in group})
        print(f"=== {gid}: hits={sizes[gid]} files={len(gfiles)} ===")
        print("  why it is still here: " + group[0]["why"])
        for r in sorted(group, key=lambda r: (r["file"], r["line"])):
            print(f"  {r['file']}:{r['line']}  {r['text']}")
        print()

    print("refused_tokens=" + ", ".join(
        f"{gid}={sizes[gid]}" for gid in order if sizes[gid]
    ))

    # The census's own total must equal the gate's, or one of them is lying.
    gate_total = G.scan(ROOT).hits_by_pattern["pcgen_import"]
    assert total == gate_total, f"census {total} != gate {gate_total}"
    print(f"gate_agreement=OK ({total} == {gate_total})")

    out = os.path.join(HERE, "AT-35-E6-003-RULED_cycle7_runtime_import_census.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump({"total_hits": total, "total_files": len(files),
                   "by_root": dict(by_root), "by_group": sizes, "rows": rows},
                  fh, indent=1)
    print(f"census written to {os.path.relpath(out, ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
