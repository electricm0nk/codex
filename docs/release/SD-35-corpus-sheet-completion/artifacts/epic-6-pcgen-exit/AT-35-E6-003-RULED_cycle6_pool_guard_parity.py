#!/usr/bin/env python3
"""AT-35-E6-003-RULED cycle 6 — can the converted package answer the four
``pool_member_tokens`` guards? Measured over the whole live ``class_feature`` corpus.

Why this measurement exists
---------------------------

``src/rules_core/class_feature_pool_catalog.rs`` is the last live caller of
``src/pcgen_import/pool_member_tokens.rs``. Four guards decide whether a corpus record is a
standing, prose-only member of a class-feature pool, and every one of them is a question
about the **ingest format**:

1. ``has_no_engine_effect_token`` — the row carries none of
   ``ABILITY CSKILL SELECT AUTO SAB BONUS DEFINE ADD SPELLS DR SR``.
2. ``is_archetype_locked`` — a ``PREABILITY`` token whose value carries ``CATEGORY=Archetype``.
3. ``carries_more_than_one_desc_segment`` — more than one ``DESC:`` field on the row.
4. ``shipped_description_is_the_already_regenerated_safe_multi_desc_join`` — reached only when
   (3) is true.

Cycle 5's census recorded that these "do not ride on the new closure-row join". Cycle 5's own
lesson was that such a sentence is an assertion about a dependency and has to be checked
against **what the caller needs**, so this cycle checked it, with the join in hand, rather
than carrying the sentence forward a seventh time.

The join itself is not the problem: **18,043 of 18,074** live ``class_feature`` corpus records
resolve to at least one converted rule through
``SheetRulePackage::rules_for_closure_row(source.path, source.line)``. The problem is what the
converted rule carries.

What it measures, and the answer
--------------------------------

For each live ``class_feature`` corpus record this script computes the ingest predicate from
the record's own ``raw_tokens`` and the best available converted-side predicate from every
rule the package wrote from that record's source row, and reports agreement:

* ``P1_no_engine_effect`` — converted probe: every rule for the row has ``value == "Text"``,
  no ``target``, no ``grants`` and no ``offers``.
* ``P2_archetype`` — converted probe: some rule's ``applies`` names a ``Holds`` on a rule id
  whose own converted record carries an ``Archetype`` category/tag.
* ``P3_multi_desc`` — converted probe: the row's rules state more than one ``Desc``-family
  prose entry.

P1 is the one that decides the swap, because it is the guard that refuses a record carrying a
mechanic. It **fails**, and the mechanism is specific, not a difficulty: PCGen's ``ABILITY:``
token maps to ``MapsTo::Applies`` (``src/pcgen_import/sheet_rule/table.rs``), i.e. to a
*prerequisite*, and ``BONUS:VAR|...`` lands in ``data/sheet_rules/_vars/`` rather than on the
rule object. So a record that hands out two abilities automatically
(``occult_adventures:class_feature:elemental_ascetic_elemental_flurry``,
``ABILITY:Feat|AUTOMATIC|Improved Unarmed Strike``) converts to a rule with ``grants: null``,
``target: None``, ``value: "Text"`` — indistinguishable, on the converted side, from a record
that really is prose only.

**The swap is therefore refused on the number, not deferred on effort**, the same disposition
cycle 2 reached for the renderer group. The remedy is converter-side and is named in the
receipt: ``ABILITY``'s mapping row, and a ``BONUS:VAR`` reference on the rule that declares it.

Re-derive::

    python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/\
AT-35-E6-003-RULED_cycle6_pool_guard_parity.py
"""

import collections
import glob
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", ".."))

ENGINE_EFFECT_TOKEN_KEYS = {
    "ABILITY", "CSKILL", "SELECT", "AUTO", "SAB", "BONUS", "DEFINE", "ADD", "SPELLS", "DR", "SR",
}


def load_converted_index():
    """``"<source path>:<line>" -> [rule]`` over every converted ``class_feature`` rule,
    plus ``rule id -> rule`` for the P2 chain lookup."""
    by_row = collections.defaultdict(list)
    by_id = {}
    pattern = os.path.join(ROOT, "data/sheet_rules/*/class_feature/**/*.json")
    for path in glob.glob(pattern, recursive=True):
        with open(path, encoding="utf-8") as fh:
            try:
                rules = json.load(fh)
            except json.JSONDecodeError:
                continue
        for rule in rules:
            by_id[rule.get("id")] = rule
            for row in (rule.get("provenance") or {}).get("closure_rows") or []:
                by_row[row].append(rule)
    return by_row, by_id


def holds_rule_ids(applies, out):
    """Every rule id named by a ``Holds`` anywhere inside an ``Applies`` tree."""
    if isinstance(applies, dict):
        for key, value in applies.items():
            if key == "Holds" and isinstance(value, dict):
                what = value.get("what")
                if isinstance(what, dict) and isinstance(what.get("Rule"), str):
                    out.append(what["Rule"])
            holds_rule_ids(value, out)
    elif isinstance(applies, list):
        for item in applies:
            holds_rule_ids(item, out)
    return out


def main():
    by_row, by_id = load_converted_index()

    totals = collections.Counter()
    agree = collections.Counter()
    disagree = collections.Counter()
    examples = collections.defaultdict(list)

    pattern = os.path.join(ROOT, "data/corpus/*/class_feature/**/*.json")
    for path in glob.glob(pattern, recursive=True):
        with open(path, encoding="utf-8") as fh:
            try:
                record = json.load(fh)
            except json.JSONDecodeError:
                continue
        data = record.get("data") or {}
        if "raw_tokens" not in data:
            continue
        totals["corpus_records"] += 1
        source = record.get("source") or {}
        row_key = f"{source.get('path')}:{source.get('line')}"
        rules = by_row.get(row_key)
        if not rules:
            totals["unjoined"] += 1
            continue
        totals["joined"] += 1

        tokens = data.get("raw_tokens") or []
        keys = [t.get("key") for t in tokens]

        def values(name):
            return [t.get("value", "") for t in tokens if t.get("key") == name]

        probes = {}
        probes["P1_no_engine_effect"] = (
            not any(k in ENGINE_EFFECT_TOKEN_KEYS for k in keys),
            all(
                r.get("value") == "Text"
                and not r.get("target")
                and not r.get("grants")
                and not r.get("offers")
                for r in rules
            ),
        )
        prereq_ids = []
        for rule in rules:
            holds_rule_ids(rule.get("applies"), prereq_ids)
        archetype_prereq = False
        for rid in prereq_ids:
            target = by_id.get(rid)
            if target and any("archetype" in str(t).lower() for t in (target.get("tags") or [])):
                archetype_prereq = True
        probes["P2_archetype"] = (
            any("CATEGORY=Archetype" in v for v in values("PREABILITY")),
            archetype_prereq,
        )
        desc_prose = sum(
            1
            for r in rules
            for p in (r.get("prose") or [])
            if p.get("family") == "Desc"
        )
        probes["P3_multi_desc"] = (len(values("DESC")) > 1, desc_prose > 1)

        for name, (ingest, converted) in probes.items():
            if ingest == converted:
                agree[name] += 1
            else:
                disagree[name] += 1
                if len(examples[name]) < 5:
                    examples[name].append(
                        {"row": row_key, "ingest": ingest, "converted": converted}
                    )

    print(f"corpus_class_feature_records={totals['corpus_records']} "
          f"joined={totals['joined']} unjoined={totals['unjoined']}")
    verdicts = {}
    for name in sorted(set(agree) | set(disagree)):
        n = agree[name] + disagree[name]
        verdicts[name] = {
            "population": n,
            "agree": agree[name],
            "disagree": disagree[name],
            "verdict": "USABLE" if disagree[name] == 0 else "REFUSED",
            "examples": examples[name],
        }
        print(f"{name}: population={n} agree={agree[name]} "
              f"disagree={disagree[name]} verdict={verdicts[name]['verdict']}")
    print()
    print("swap_verdict=" + (
        "REFUSED" if any(v["verdict"] == "REFUSED" for v in verdicts.values()) else "USABLE"
    ))

    out = os.path.join(HERE, "AT-35-E6-003-RULED_cycle6_pool_guard_parity.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump(
            {
                "corpus_class_feature_records": totals["corpus_records"],
                "joined": totals["joined"],
                "unjoined": totals["unjoined"],
                "probes": verdicts,
            },
            fh,
            indent=1,
        )
    print(f"parity written to {os.path.relpath(out, ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
