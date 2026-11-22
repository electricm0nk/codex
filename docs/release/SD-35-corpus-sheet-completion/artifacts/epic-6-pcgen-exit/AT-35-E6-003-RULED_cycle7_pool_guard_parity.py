#!/usr/bin/env python3
"""AT-35-E6-003-RULED cycle 7 — re-measure the three `pool_member_tokens`
guards against a CORRECTED converted-side probe.

Cycle 6 measured the same three guards and refused the swap on the number
(`P1 agree=11549 disagree=6494`), naming the cause as a converter defect:
*"PCGen's `ABILITY:` token maps to `MapsTo::Applies` in
`src/pcgen_import/sheet_rule/table.rs`, i.e. to a prerequisite"*.

**That sentence is wrong, and this script is the measurement that shows it.**
`src/pcgen_import/sheet_rule/convert.rs:1266` converts an `ABILITY:` token
exactly as `mapping-table.v1.json`'s `ABILITY` row specifies — as a **grant
edge** — and pushes it to `out.grants_out`, which
`sheet_rule/mod.rs:691-732` folds onto the **TARGET** rule's `granted_by`.
The target rule lives in a different file, under a different record id.
Cycle 6's probe read only `rules_for_closure_row(path, line)` — the rules the
converter wrote FROM this record's own source row — so it was structurally
incapable of seeing the converted form of an `ABILITY:` token. The 6,494
"disagreements" are the probe's blind spot, not the converter's output.

Worked example, the one cycle 6 named:
`occult_adventures:class_feature:elemental_ascetic_elemental_flurry` carries
`ABILITY:Feat|AUTOMATIC|Improved Unarmed Strike`. Its own converted rule has
`value: "Text"` and no `grants` — and
`data/sheet_rules/core_rulebook/feat/improved_unarmed_strike.json` carries
`{"by": {"Rule": "occult_adventures:class_feature:elemental_ascetic_elemental_flurry"}}`
in its `granted_by`. The edge converted. It just points the other way.

The corrected converted-side probes
-----------------------------------
``P1_no_engine_effect`` — the record contributes no engine effect anywhere in
the converted package:

  * every rule written from the record's source row has ``value == "Text"``,
  * none of those rules carries a non-empty ``grants`` (Effect list),
    ``target``, ``also``, or ``offers``,
  * and **no rule anywhere in the package names this record as a granter**
    (the reverse-grant index — the half cycle 6 could not see).

``P2_archetype`` — some rule from the row has an ``applies`` naming a
``Holds`` on a rule id whose own converted record is archetype-shaped
(``pool == "archetype"`` or a ``Tag`` containing ``Archetype``), OR whose
``applies`` carries a ``Words`` clause naming ``CATEGORY=Archetype``'s target
that failed to resolve.

``P3_multi_desc`` — the row's rules state more than one ``Desc``-family prose
segment.

Ingest-side forms are computed from the record's own ``raw_tokens``, exactly
as ``src/pcgen_import/pool_member_tokens.rs`` computes them.

Output: JSON beside this script, and a summary to stdout.
"""

from __future__ import annotations

import glob
import json
import os
import sys
from collections import defaultdict

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", ".."))
CORPUS = os.path.join(REPO, "data", "corpus")
RULES = os.path.join(REPO, "data", "sheet_rules")
OUT = os.path.join(os.path.dirname(__file__), "AT-35-E6-003-RULED_cycle7_pool_guard_parity.json")

# Verbatim from src/pcgen_import/pool_member_tokens.rs::ENGINE_EFFECT_TOKEN_KEYS.
ENGINE_EFFECT_TOKEN_KEYS = {
    "ABILITY", "CSKILL", "SELECT", "AUTO", "SAB", "BONUS", "DEFINE", "ADD", "SPELLS", "DR", "SR",
}


def token_pairs(data):
    """(key, value) over the record's own ingest token array, as
    `pcgen_import::ingest_record::token_pairs` reads it."""
    toks = data.get("raw_tokens")
    if not isinstance(toks, list):
        return []
    out = []
    for t in toks:
        if isinstance(t, dict) and "key" in t:
            out.append((str(t["key"]), str(t.get("value", ""))))
    return out


def load_var_contributors():
    """Every rule id that declares or contributes to a converted variable.

    A `BONUS:VAR` / `DEFINE:` token does not land on the rule object at all —
    it becomes a contributor row in `data/sheet_rules/_vars/<var>.json`, which
    cites the contributing `rule_id`. A converted-side "does this record carry
    an engine effect" probe that reads only the rule object is blind to
    3,954 of them (2,304 `BONUS` + 1,650 `DEFINE` records), which is the
    second half of what this cycle corrects.
    """
    out = set()
    for path in glob.glob(os.path.join(RULES, "_vars", "*.json")):
        try:
            doc = json.load(open(path))
        except Exception:
            continue
        for rid in doc.get("declared_by") or []:
            out.add(rid)
        for c in doc.get("contributions") or []:
            if isinstance(c, dict) and isinstance(c.get("rule_id"), str):
                out.add(c["rule_id"])
    return out


def load_package():
    """Return (rules_by_source_row, rules_by_id, granter_ids).

    `granter_ids` is the reverse-grant index: every rule id that appears as a
    `Granter::Rule` / `Granter::Choice` inside SOME rule's `granted_by`.
    """
    by_row = defaultdict(list)
    by_id = {}
    granters = set()
    for path in glob.glob(os.path.join(RULES, "*", "*", "*.json")):
        try:
            doc = json.load(open(path))
        except Exception:
            continue
        if not isinstance(doc, list):
            continue
        for rule in doc:
            if not isinstance(rule, dict) or "id" not in rule:
                continue
            by_id[rule["id"]] = rule
            for row in (rule.get("provenance") or {}).get("closure_rows") or []:
                by_row[row].append(rule)
            for grant in rule.get("granted_by") or []:
                by = grant.get("by")
                if isinstance(by, dict):
                    for k in ("Rule", "Choice", "Deity"):
                        if k in by:
                            granters.add(by[k])
    return by_row, by_id, granters


def collect_rule_refs(node, out):
    """Every `{"Rule": "<id>"}` string anywhere under `node`."""
    if isinstance(node, dict):
        for k, v in node.items():
            if k == "Rule" and isinstance(v, str):
                out.add(v)
            else:
                collect_rule_refs(v, out)
    elif isinstance(node, list):
        for v in node:
            collect_rule_refs(v, out)


def holds_rule_ids(applies, out):
    """Rule ids named by a `Holds` clause.

    The serialized shape is `{"Holds": {"what": {"Rule": id}, "count": n}}` —
    the id sits two levels below the `Holds` key, not directly under it. A
    traversal that only looks for `{"Holds": {"Rule": ...}}` finds none of
    them, which is what made this probe's first run miss 197 archetype gates.
    """
    if isinstance(applies, dict):
        for k, v in applies.items():
            if k == "Holds":
                collect_rule_refs(v, out)
            else:
                holds_rule_ids(v, out)
    elif isinstance(applies, list):
        for v in applies:
            holds_rule_ids(v, out)


def words_of(applies, out):
    if isinstance(applies, dict):
        for k, v in applies.items():
            if k == "Words" and isinstance(v, str):
                out.append(v)
            else:
                words_of(v, out)
    elif isinstance(applies, list):
        for v in applies:
            words_of(v, out)


def archetype_shaped(rule):
    if rule is None:
        return False
    if str(rule.get("pool", "")).lower() == "archetype":
        return True
    return any("Archetype" in str(t) for t in rule.get("tags") or [])


def main() -> int:
    by_row, by_id, granters = load_package()
    var_contributors = load_var_contributors()

    records = sorted(glob.glob(os.path.join(CORPUS, "*", "class_feature", "**", "*.json"), recursive=True))
    stats = {
        "corpus_class_feature_files": len(records),
        "records_with_raw_tokens": 0,
        "joined": 0,
        "unjoined": 0,
        "probes": {},
    }
    probes = {"P1_no_engine_effect": [0, 0], "P2_archetype": [0, 0], "P3_multi_desc": [0, 0]}
    disagreements = defaultdict(list)

    for path in records:
        try:
            doc = json.load(open(path))
        except Exception:
            continue
        data = doc.get("data") if isinstance(doc, dict) else None
        if not isinstance(data, dict) or "raw_tokens" not in data:
            continue
        stats["records_with_raw_tokens"] += 1
        src = doc.get("source") or {}
        row_key = f"{src.get('path','')}:{src.get('line','')}"
        rules = by_row.get(row_key)
        if not rules:
            stats["unjoined"] += 1
            continue
        stats["joined"] += 1

        pairs = token_pairs(data)
        keys = [k for k, _ in pairs]
        descs = [v for k, v in pairs if k == "DESC"]

        ingest = {
            "P1_no_engine_effect": not any(k in ENGINE_EFFECT_TOKEN_KEYS for k in keys),
            "P2_archetype": any(
                "CATEGORY=Archetype" in v for k, v in pairs if k == "PREABILITY"
            ),
            "P3_multi_desc": len(descs) > 1,
        }

        rule_ids = {r["id"] for r in rules}
        # --- P1, corrected: the record contributes no engine effect ANYWHERE. ---
        own_side_clean = all(
            r.get("value") == "Text"
            and not r.get("grants")
            and not r.get("also")
            and not r.get("target")
            and not r.get("offers")
            for r in rules
        )
        grants_elsewhere = bool(rule_ids & granters)
        declares_var = bool(rule_ids & var_contributors)
        conv_p1 = own_side_clean and not grants_elsewhere and not declares_var

        # --- P2 ---
        held = set()
        words = []
        for r in rules:
            holds_rule_ids(r.get("applies"), held)
            words_of(r.get("applies"), words)
        conv_p2 = any(archetype_shaped(by_id.get(h)) for h in held) or any(
            "Archetype" in w for w in words
        )

        # --- P3 ---
        conv_p3 = sum(
            1 for r in rules for s in (r.get("prose") or []) if s.get("family") == "Desc"
        ) > 1

        conv = {"P1_no_engine_effect": conv_p1, "P2_archetype": conv_p2, "P3_multi_desc": conv_p3}
        for name in probes:
            if ingest[name] == conv[name]:
                probes[name][0] += 1
            else:
                probes[name][1] += 1
                if len(disagreements[name]) < 25:
                    disagreements[name].append(
                        {"row": row_key, "ingest": ingest[name], "converted": conv[name]}
                    )

    verdicts = {}
    for name, (agree, dis) in probes.items():
        stats["probes"][name] = {"population": agree + dis, "agree": agree, "disagree": dis}
        verdicts[name] = "AGREES" if dis == 0 else "DISAGREES"
    stats["verdicts"] = verdicts
    stats["swap_verdict"] = "ACCEPTED" if all(v == "AGREES" for v in verdicts.values()) else "REFUSED"
    stats["disagreement_samples"] = {k: v for k, v in disagreements.items()}

    with open(OUT, "w") as fh:
        json.dump(stats, fh, indent=1, sort_keys=True)
        fh.write("\n")

    print(f"corpus_class_feature_records={stats['records_with_raw_tokens']} "
          f"joined={stats['joined']} unjoined={stats['unjoined']}")
    for name in ("P1_no_engine_effect", "P2_archetype", "P3_multi_desc"):
        p = stats["probes"][name]
        print(f"{name} population={p['population']} agree={p['agree']} disagree={p['disagree']} "
              f"verdict={verdicts[name]}")
    print(f"swap_verdict={stats['swap_verdict']}")
    print(f"wrote {OUT}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
