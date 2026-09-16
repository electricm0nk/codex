#!/usr/bin/env python3
"""AT-35-E6-003-RULED cycle 18 — the run-time ``pcgen_import`` census, re-derived at ZERO.

Cycle 17's census is imported whole (which imports cycle 16's, … back to cycle 1's): the
``GROUPS`` table, ``classify``, ``_code_lines`` and the two exclusions the gate itself applies
(B14 comment lines, B15 ``#[cfg(test)]`` regions) stay single-sourced. **No regex is widened and
no regex is narrowed**, and ``scripts/pcgen_residue_gate.py`` /
``scripts/pcgen-residue-baseline.env`` are absent from this cycle's diff — no path exempted, no
rebaseline, no rename to duck a pattern.

What cycle 18 closed
--------------------

Both remaining groups, and with them the last four hits and the last four files.

**1. ``source_content_payload`` (3 hits, 3 files) — SPLIT, not moved.**
``rules_core::source_content`` re-exported ``pcgen_import::source_content_payload::
SourceContentPayload``, and ``rules_core::spell_resolver`` / ``rules_core::equipment_resolver``
imported it from the converter directly. Refused for **eleven** cycles, and correctly: four of
the enum's seven variants (``Class``, ``SpellcastingClass``, ``Race``/``Ability``, ``Metadata``)
borrow Slice B parser entry types, so moving the enum to the live side would have moved raw
``CLASS:``/``RACE:`` token vectors with it — the opposite of ``decisions.md`` §11.

Cycle 18 splits the type. ``rules_core::source_content`` declares its **own**
``SourceContentPayload``: the two settled kinds the live side already owns
(``Spell(&CorpusSpellRecord)`` from cycle 8, ``Equipment(&CorpusEquipmentRecord)`` from cycles
10/13) plus ``Unsettled { kind, name }`` — identity, never ingest vocabulary. The seven-variant
parser-borrowing projection keeps **every** variant under its own name,
``pcgen_import::ir_content_payload::IrContentPayload``, on the converter side §11 KEEPS. The
envelope (``SourceContentRecord`` / ``SourcePackageContent`` / ``SourceContentLoadResult``) is
generic over its payload and defaults to the live one, so ``SourceContentRecord<'a>`` still means
what it meant and not one of its ~80 consumers changed. ``record_to_live`` / ``package_to_live``
project one onto the other at the single boundary the live side consumes,
``pcc_package_loader::project_corpus_from_owned``.

Nothing was lost: no live consumer has ever matched a parser-borrowing variant (the only live
matches in the tree were ``Spell`` in ``spell_resolver``, ``Equipment`` in ``equipment_resolver``
and both in the desktop crate's ``corpus_fixtures``), and the slice-E integration suites still
prove every variant, against the IR enum.

**2. ``trait_and_pool_tokens`` (1 hit, 1 file) — SETTLED at ingest.**
``rules_core::class_feature_pool_catalog``'s top-level ``use crate::pcgen_import::
pool_member_tokens;``. Behind it, in **shipping** code, sat three gates asked of the corpus row
itself on every process start: which ingest token keys it carries, whether a ``PREABILITY`` names
``CATEGORY=Archetype``, how many ``DESC:`` segments it has. Those are questions about the ingest
format, so ``pcgen_import::pool_gate_settle`` asks them once at authoring time and
``data/converted/record_vars.json`` gains ``pool_gates``; the catalog reads the verdict. The
table is **fail-closed** — a key it does not hold is not admitted — so a missing or stale
artifact serves fewer pool options, never an unvetted one. The remaining calls to the predicates
are inside ``#[cfg(test)]``, which is not live code (``decisions.md`` §18 / ruling B15), and they
are deliberately kept: asking the converter's predicates directly is what makes the catalog's own
census an independent check on the settled table rather than a restatement of it.

The claim this cycle CORRECTS
-----------------------------

Cycle 17's receipt priced the ``source_content_payload`` remainder at "four cycles" (one settled
record kind per cycle) or "one cycle, but it re-types ``SourceContentRecord``/
``SourcePackageContent``". Both readings assumed the envelope had to keep **one** payload type.
It did not: making the envelope generic with the live payload as its default re-typed nothing at
any consumer. Re-derive the no-op: ``git diff 5bd0eda548..HEAD --stat`` lists neither
``rules_core::corpus_loader``, ``rules_core::composed_input`` nor any ``tests/sd19_*`` /
``tests/sd20_*`` file, all of which name the envelope.
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
    "at35_e6_003_ruled_cycle17_census",
    os.path.join(HERE, "AT-35-E6-003-RULED_cycle17_runtime_import_census.py"),
)
C17 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(C17)
C15 = C17.C15

SOURCE_CONTENT_WHY_CYCLE18 = (
    "CLEARED BY CYCLE 18. Three counted hits -- `rules_core::source_content`'s `pub use` of the "
    "converter's payload enum, and the two resolvers importing it from the converter directly. "
    "SPLIT, not moved: `rules_core::source_content` now declares its own "
    "`SourceContentPayload` (`Spell`, `Equipment`, `Unsettled { kind, name }`) naming only "
    "rules-core types, while the seven-variant parser-borrowing projection keeps every variant "
    "as `pcgen_import::ir_content_payload::IrContentPayload`. The envelope is generic over its "
    "payload and defaults to the live one, so `SourceContentRecord<'a>` is unchanged for every "
    "consumer; `record_to_live`/`package_to_live` project at the one boundary the live side "
    "consumes. Re-derive: `grep -rn 'pcgen_import' src/rules_core/source_content.rs "
    "src/rules_core/spell_resolver.rs src/rules_core/equipment_resolver.rs` (expect no output)."
)

POOL_TOKENS_WHY_CYCLE18 = (
    "CLEARED BY CYCLE 18. One counted hit -- `rules_core::class_feature_pool_catalog`'s "
    "top-level `use crate::pcgen_import::pool_member_tokens;` -- carrying three SHIPPING gates "
    "asked of the corpus row on every process start. Settled at ingest by "
    "`pcgen_import::pool_gate_settle` into `record_vars.json`'s `pool_gates`, with a "
    "whole-corpus test that re-reads every `class_feature` record in every book through the "
    "converter's own predicates and compares the settled verdict record for record, refusal "
    "reasons included. The table is fail-closed. Re-derive: "
    "`grep -n 'pcgen_import' src/rules_core/class_feature_pool_catalog.rs` (doc comments and a "
    "`#[cfg(test)]` import only -- B14 and B15)."
)

_REWRITTEN = {
    "source_content_payload": SOURCE_CONTENT_WHY_CYCLE18,
    "trait_and_pool_tokens": POOL_TOKENS_WHY_CYCLE18,
}

# Every regex is cycle 17's, untouched. No group is dropped; both remaining groups simply match
# nothing under a live root any more.
GROUPS = [(gid, rx, _REWRITTEN.get(gid, why)) for gid, rx, why in C17.GROUPS]
FALLBACK = C17.FALLBACK

# The four files cycle 18 clears whole -- every remaining one.
CLEARED_BY_CYCLE18 = (
    "src/rules_core/source_content.rs",
    "src/rules_core/spell_resolver.rs",
    "src/rules_core/equipment_resolver.rs",
    "src/rules_core/class_feature_pool_catalog.rs",
)

# The hits cycle 18 closed, by file and symbol.
CLOSED_BY_CYCLE18 = (
    ("src/rules_core/source_content.rs", "source_content_payload::SourceContentPayload"),
    ("src/rules_core/spell_resolver.rs", "source_content_payload::SourceContentPayload"),
    ("src/rules_core/equipment_resolver.rs", "source_content_payload::SourceContentPayload"),
    ("src/rules_core/class_feature_pool_catalog.rs", "pool_member_tokens"),
)
SPLIT_BY_CYCLE18 = (
    ("src/pcgen_import/ir_content_payload.rs", "IrContentPayload",
     "src/rules_core/source_content.rs", "SourceContentPayload"),
)
SETTLED_BY_CYCLE18 = (
    ("src/rules_core/class_feature_pool_catalog.rs", "pool_member_tokens",
     "src/pcgen_import/pool_gate_settle.rs"),
)

# Booked explicitly: this cycle relabels nothing. Neither closure moved a read from one live file
# to another -- one split a type so the live half names no parser entry, the other moved a
# run-time reading to authoring time.
RELABELLED_BY_CYCLE18 = ()

LIVE_ENVELOPE_MODULE = "src/rules_core/source_content.rs"
LIVE_POOL_CATALOG = "src/rules_core/class_feature_pool_catalog.rs"
CONVERTER_PAYLOAD_MODULE = "src/pcgen_import/ir_content_payload.rs"
CONVERTER_SETTLING_MODULE = "src/pcgen_import/pool_gate_settle.rs"
CONVERTED_ARTIFACT = "data/converted/record_vars.json"
LIVE_RESOLVERS = (
    "src/rules_core/spell_resolver.rs",
    "src/rules_core/equipment_resolver.rs",
)


def classify(text):
    for gid, rx, why in GROUPS:
        if rx.search(text):
            return gid, why
    return FALLBACK


def _read(rel):
    with open(os.path.join(ROOT, rel), encoding="utf-8") as fh:
        return fh.read()


def _shipping(rel):
    return "\n".join(t for _, t in C15._code_lines(os.path.join(ROOT, rel)))


def main():
    rx = G._COMPILED["pcgen_import"]
    rows = []
    for live_root, rel, abs_path in G._iter_live_source_files(ROOT):
        for lineno, line in C15._code_lines(abs_path):
            n = len(rx.findall(line))
            if not n:
                continue
            gid, why = classify(line)
            rows.append(dict(root=live_root, file=rel, line=lineno, hits=n,
                             group=gid, why=why, text=line.strip()[:200]))

    total = sum(r["hits"] for r in rows)
    files = {r["file"] for r in rows}
    print(f"pcgen_import_hits={total} files={len(files)}")
    by_root = collections.Counter()
    for r in rows:
        by_root[r["root"]] += r["hits"]
    print("by_root=" + (", ".join(f"{k}={v}" for k, v in sorted(by_root.items())) or "(none)"))
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
            if gid in _REWRITTEN:
                print("  " + _REWRITTEN[gid])
            print()
            continue
        gfiles = sorted({r["file"] for r in group})
        print(f"=== {gid}: hits={sizes[gid]} files={len(gfiles)} ===")
        print("  why it is still here: " + group[0]["why"])
        for r in sorted(group, key=lambda r: (r["file"], r["line"])):
            print(f"  {r['file']}:{r['line']}  {r['text']}")
        print()

    refused = ", ".join(f"{gid}={sizes[gid]}" for gid in order if sizes[gid])
    print("refused_tokens=" + (refused or "none"))

    # The census's own total must equal the gate's, or one of them is lying.
    gate_total = G.scan(ROOT).hits_by_pattern["pcgen_import"]
    assert total == gate_total, f"census {total} != gate {gate_total}"
    print(f"gate_agreement=OK ({total} == {gate_total})")

    # ---- what cycle 18 closed, asserted by file and by symbol, not inferred from the count ----
    for rel in CLEARED_BY_CYCLE18:
        ship = _shipping(rel)
        assert "pcgen_import" not in ship, f"{rel} still names the converter in shipping code"
    print(f"cleared_files_by_cycle18={len(CLEARED_BY_CYCLE18)} "
          f"closed_by_cycle18={len(CLOSED_BY_CYCLE18)} "
          f"split_by_cycle18={len(SPLIT_BY_CYCLE18)} "
          f"settled_by_cycle18={len(SETTLED_BY_CYCLE18)} "
          f"relabelled_by_cycle18={len(RELABELLED_BY_CYCLE18)}")

    # Every file cycles 10-17 cleared is still clear -- a closure that regressed is not a closure.
    for rel in (C17.CLEARED_BY_CYCLE17 + ("src/rules_core/corpus_loader.rs",)):
        assert "pcgen_import" not in _shipping(rel), f"{rel} regressed"
    print("earlier_cleared_files_still_clear=YES")

    # ---- the split: the live enum names no parser type, the converter's keeps every variant ----
    live_env = _shipping(LIVE_ENVELOPE_MODULE)
    assert "pub enum SourceContentPayload" in _read(LIVE_ENVELOPE_MODULE), \
        "the live payload enum is not declared in rules_core::source_content"
    for parser_type in ("ClassEntry", "SpellcastingClassEntry", "RaceDeclaration",
                        "AbilityDeclaration", "LstRecord"):
        assert parser_type not in live_env, \
            f"the live envelope names the parser entry type {parser_type}"
    ir = _read(CONVERTER_PAYLOAD_MODULE)
    variants = ("Class(", "SpellcastingClass(", "Race(", "Ability(", "Spell(",
                "Equipment(", "Metadata(")
    missing = [v for v in variants if f"    {v}" not in ir]
    assert not missing, f"the converter's projection lost variants: {missing}"
    assert "pub type IrContentRecord" in ir and "pub type IrPackageContent" in ir, \
        "the converter's envelope instantiation is missing"
    assert "pub fn record_to_live" in ir and "pub fn package_to_live" in ir, \
        "the boundary projection is missing"
    print(f"live_payload_variants=3 ir_payload_variants={len(variants)} "
          f"boundary_projection=present parser_types_on_live_side=0")

    # The two resolvers now import the live enum from the live module.
    for rel in LIVE_RESOLVERS:
        ship = _shipping(rel)
        assert "use crate::rules_core::source_content::SourceContentPayload;" in ship, \
            f"{rel} does not import the live payload from rules_core"
    print("live_resolvers_import_live_payload=YES")

    # ---- the settling: a real population, partitioned, and fail-closed ----
    with open(os.path.join(ROOT, CONVERTED_ARTIFACT), encoding="utf-8") as fh:
        pkg = json.load(fh)
    gates = pkg.get("pool_gates", {})
    admitted = set(gates.get("admitted", []))
    refused_map = gates.get("refused", {})
    assert len(admitted) >= 4_000, f"the admitted population collapsed: {len(admitted)}"
    assert len(refused_map) >= 4_000, f"the refused population collapsed: {len(refused_map)}"
    assert not (admitted & set(refused_map)), "a record is both admitted and refused"
    reasons = collections.Counter(refused_map.values())
    assert set(reasons) <= {"engine_effect_token_present", "archetype_locked",
                            "multi_desc_segment_not_regenerated"}, \
        f"an unexpected refusal reason appeared: {sorted(reasons)}"
    books = {k.split("|", 1)[0] for k in list(admitted) + list(refused_map)}
    assert len(books) >= 8, f"the settled table must span the ingested books, got {len(books)}"
    print(f"pool_gates_admitted={len(admitted)} pool_gates_refused={len(refused_map)} "
          f"books={len(books)} reasons=" +
          ", ".join(f"{k}={v}" for k, v in sorted(reasons.items())))

    # The catalog reads the verdict, and the predicates it used to call are test-only now.
    catalog_ship = _shipping(LIVE_POOL_CATALOG)
    assert "pool_gates.admits(" in catalog_ship, \
        "the shipping catalog walk does not read the settled verdict"
    assert "pool_member_tokens" not in catalog_ship, \
        "the shipping catalog still calls the converter's predicates"
    assert "pool_member_tokens::" in _read(LIVE_POOL_CATALOG), \
        "the independent #[cfg(test)] check on the settled table was deleted, not kept"
    print("pool_catalog_reads_settled_verdict=YES "
          "pool_catalog_shipping_pcgen_import_hits=0 cfg_test_independent_check=KEPT")

    # ---- the criterion's own bar ----
    print(f"criterion_population={total} "
          f"closure_reached={'YES' if total == 0 else 'NO'}")

    out = os.path.join(HERE, "AT-35-E6-003-RULED_cycle18_runtime_import_census.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump({"total_hits": total, "total_files": len(files),
                   "by_root": dict(by_root), "by_group": sizes, "rows": rows,
                   "closed_by_cycle18": [list(x) for x in CLOSED_BY_CYCLE18],
                   "split_by_cycle18": [list(x) for x in SPLIT_BY_CYCLE18],
                   "settled_by_cycle18": [list(x) for x in SETTLED_BY_CYCLE18],
                   "relabelled_by_cycle18": [list(x) for x in RELABELLED_BY_CYCLE18],
                   "cleared_files_by_cycle18": list(CLEARED_BY_CYCLE18),
                   "pool_gates_admitted": len(admitted),
                   "pool_gates_refused": len(refused_map),
                   "pool_gate_refusal_reasons": dict(reasons)},
                  fh, indent=2)
    print(f"census written to {os.path.relpath(out, ROOT)}")


if __name__ == "__main__":
    main()
