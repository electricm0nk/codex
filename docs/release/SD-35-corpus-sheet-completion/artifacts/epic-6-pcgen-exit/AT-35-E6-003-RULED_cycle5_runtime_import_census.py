#!/usr/bin/env python3
"""AT-35-E6-003-RULED cycle 5 — the run-time ``pcgen_import`` census, re-derived.

Cycle 4's census is imported whole (which imports cycle 3's, which imports cycle 2's,
which imports cycle 1's): the ``GROUPS`` table, ``classify``, and the two exclusions the
gate itself applies (B14 comment lines, B15 ``#[cfg(test)]`` regions) stay single-sourced.
**The gate script and ``scripts/pcgen-residue-baseline.env`` are absent from this cycle's
diff** — no path exempted, no regex weakened, no rebaseline.

What cycle 5 moved, and the correction that let it move
-------------------------------------------------------

Cycle 4's census recorded, for the per-record token readers, that they need "the converted
package to carry the same facts keyed by ``VarId``", which "does not exist yet". **That was
wrong in the same direction cycle 4 caught its own predecessor being wrong: it is an
assertion about a dependency that was never checked against what the caller needs.**

``data/sheet_rules/`` already carries the facts. What was missing was the **join**. A live
caller holding a ``data/corpus/**/*.json`` record had no way to ask the converted package
*what did this record become?*:

* ``SheetRulePackage::find(kind, slug)`` joins on the converter's own ``slug(name)`` file
  name, which is not the corpus file name. Measured over ``kind: trait``: **131 of 487**
  corpus records do not resolve (the ``codex_named_unit_*`` rows, whose corpus ``key`` is
  synthetic).
* The corpus record states its own origin row — ``source.path`` and ``source.line`` — and
  every rule the converter wrote from that row already names the same ``path:line`` in
  ``provenance.closure_rows``. Nothing indexed it.

So cycle 5 built that index: ``SheetRulePackage::rules_for_closure_row(path, line)``, built
in ``finish()`` alongside the three indexes already there. It is **total** over every Epic-2
kind's live corpus — **8,486 of 8,486** records across ``ability``, ``template``, ``trait``,
``deity``, ``domain``, ``skill`` and ``language`` resolve to at least one converted rule.

Its first consumer is ``rules_core::trait_pool``, which had been reading a corpus record's
``TYPE:`` token array through ``pcgen_import::ingest_record::type_token_suffix`` and holding
the ingest prefix string ``"Trait.RaceTrait."`` in live code. It now reads the converted
rule's ``tags`` — the converter writes the whole ``TYPE:`` chain out as tags — and the
module names no ingest vocabulary at all. Agreement with the retired token read is
**487 of 487** live ``kind: trait`` corpus records, pinned by a corpus-wide test that
computes the retired read in-test as its oracle and was mutation-proved to fail
(``tags[2]`` → ``tags[1]`` ⇒ 20 of 487 disagree).

**What that does NOT do, stated plainly.** One hit of 37. The other 36 are unmoved, and the
join does not by itself move them: ``equipment_effects`` and its siblings do not want a fact
*about* a record, they own ``EquipmentRecord`` as their data type and read its PCGen
``BONUS:`` chains and ``KEY:VAL`` tokens directly. That clears when the converter emits an
equipment rule shape those modules can read, not when a lookup exists.
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
    "at35_e6_003_ruled_cycle4_census",
    os.path.join(HERE, "AT-35-E6-003-RULED_cycle4_runtime_import_census.py"),
)
C4 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(C4)

# The two reasons cycle 5 rewrote. Both record a MEASUREMENT this cycle made, so no later
# cycle spends itself re-discovering it.
INGEST_RECORD_WHY_CYCLE5 = (
    "Run-time token-string reads: `token_pairs`, `bonus_chain_qualifiers`, "
    "`rebuild_bonus_token`, `token_count`, and the `RaceCacheData` / `RaceTraitCacheData` "
    "payload shapes. CYCLE 5 CORRECTS cycle 4's stated reason for this group, which was "
    "that the converted package must first carry the same facts keyed by `VarId`. It "
    "already carries them; what was missing was the JOIN from a corpus record to its own "
    "converted rules. That join now exists -- "
    "`SheetRulePackage::rules_for_closure_row(path, line)`, on the record's own "
    "`source.path`/`source.line`, total over 8,486 of 8,486 corpus records across all "
    "seven Epic-2 kinds -- and `trait_pool.rs` left this group by using it. What is left "
    "is NOT blocked on the join: `corpus_loader`'s three calls REBUILD an "
    "`EquipmentRecord`'s token and BONUS arrays out of the corpus JSON, and "
    "`race_resolver`'s cache shapes carry `raw_tokens`/`raw_bonus_chains` as fields. Both "
    "want a converted equipment/race-trait rule shape to read instead, which is the same "
    "one piece of work `lst_parser_types` names. "
    "`derived_evaluator_fixture_check.rs` stays measured non-relocatable (cycle 3): three "
    "desktop catalogs import live rendering functions out of it. Re-derive: "
    "`grep -rn 'rules_core::derived_evaluator_fixture_check' --include=*.rs apps/`."
)
TRAIT_AND_POOL_WHY_CYCLE5 = (
    "Per-row token readers for racial traits, feature pools and bonus chains. CYCLE 5: "
    "this group is UNCHANGED at 4 and that is stated, not netted -- the one hit this cycle "
    "cleared (`trait_pool.rs`) was classified under `ingest_record_tokens`, not here. "
    "These four do not ride on the new closure-row join: "
    "`class_feature_pool_catalog`'s four hand-kept `pool_member_tokens` guards read a "
    "record's DESC/effect token array rather than one classified fact, and "
    "`race_resolver`/`skinwalker_change_shape` read `raw_tokens`/`raw_bonus_chains` off a "
    "cache payload whose converted equivalent is the race-trait rule shape this epic's "
    "remaining piece produces."
)

_REWRITTEN = {
    "ingest_record_tokens": INGEST_RECORD_WHY_CYCLE5,
    "trait_and_pool_tokens": TRAIT_AND_POOL_WHY_CYCLE5,
}

GROUPS = [
    (gid, rx, _REWRITTEN.get(gid, why)) for gid, rx, why in C4.GROUPS
]
FALLBACK = C4.FALLBACK


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
    assert total == G.scan(ROOT).hits_by_pattern["pcgen_import"], (
        "census disagrees with the gate it imports"
    )
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

    out = os.path.join(HERE, "AT-35-E6-003-RULED_cycle5_runtime_import_census.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump({"total_hits": total, "total_files": len(files),
                   "by_root": dict(by_root), "by_group": sizes, "rows": rows},
                  fh, indent=1)
    print(f"census written to {os.path.relpath(out, ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
