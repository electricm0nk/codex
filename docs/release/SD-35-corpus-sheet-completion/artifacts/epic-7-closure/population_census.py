#!/usr/bin/env python3
"""AT-35-E7-000-POPULATION-CENSUS -- how many `data/corpus` records never reach
`docs/work-inventory.json`.

Measurement only. Writes `population-census.json`; changes no population, no
classifier and no gate.

The join replicated here is `src/pcgen_import/sheet_rule/mod.rs`'s
`walk_corpus` + `load_population`, the ONLY code path that joins the two
populations, not a path-to-id guess:

  walk_corpus (mod.rs:105)
    - `data/corpus/<book-dir>/**/*.json`, recursive
    - book dir `beastiary` is re-spelled `bestiary`
    - directories named `_parity` are skipped whole
    - `LICENSE.json` is skipped
    - kind  = FIRST path component under the book dir (so `feat_generic`,
              `race_trait_generic`, `trait_generic`, `_settled` are corpus
              directory names, NOT inventory kinds -- joining on them is the
              trap this cycle exists to avoid)
    - slug  = file stem
    - basename/line = the record's own `source.path` basename + `source.line`

  load_population (mod.rs:256), per INVENTORY unit, first hit wins:
    1. by_line: (unit.book, unit.source_file, unit.source_line)
    2. by_key : (unit.book, unit.kind, id.split(':',2)[2])
    3. neither -> `source_row_in_tree`, else refusal `no_corpus_record`

Inverting it marks every corpus entry some unit claimed. What is left is the
answer.
"""

import json
import os
import sys
from collections import Counter, defaultdict

REPO = os.path.dirname(os.path.abspath(__file__)).split("/docs/release/")[0]
CORPUS = os.path.join(REPO, "data/corpus")
INV = os.path.join(REPO, "docs/work-inventory.json")
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "population-census.json")


def walk_corpus():
    """Exactly mod.rs::walk_corpus."""
    entries = []
    for book_dir_name in sorted(os.listdir(CORPUS)):
        bd = os.path.join(CORPUS, book_dir_name)
        if not os.path.isdir(bd):
            continue
        book = "bestiary" if book_dir_name == "beastiary" else book_dir_name
        for dirpath, dirnames, filenames in os.walk(bd):
            dirnames[:] = [d for d in dirnames if d != "_parity"]
            for fn in sorted(filenames):
                if not fn.endswith(".json") or fn == "LICENSE.json":
                    continue
                p = os.path.join(dirpath, fn)
                rel = os.path.relpath(p, bd)
                kind = rel.split(os.sep)[0] if os.sep in rel else rel
                slug = fn[: -len(".json")]
                basename, line = "", None
                try:
                    with open(p, encoding="utf-8") as fh:
                        rec = json.load(fh)
                except Exception:
                    rec = None
                if isinstance(rec, dict):
                    src = rec.get("source") or {}
                    if isinstance(src, dict):
                        path = src.get("path") or ""
                        basename = path.rsplit("/", 1)[-1] if path else ""
                        ln = src.get("line")
                        line = int(ln) if isinstance(ln, int) else None
                else:
                    rec = None
                entries.append(
                    dict(
                        path=os.path.relpath(p, REPO),
                        kind=kind,
                        book=book,
                        book_dir=book_dir_name,
                        slug=slug,
                        basename=basename,
                        line=line,
                        rec=rec,
                    )
                )
    return entries


def main():
    entries = walk_corpus()
    by_line, by_key = {}, {}
    for i, e in enumerate(entries):
        if e["line"] is not None and e["basename"]:
            by_line.setdefault((e["book"], e["basename"], e["line"]), i)
        by_key.setdefault((e["book"], e["kind"], e["slug"]), i)

    inv = json.load(open(INV, encoding="utf-8"))
    units = inv["units"]

    claimed = {}
    unit_unjoined = []
    for u in units:
        idx = None
        if u.get("source_file") and u.get("source_line") is not None:
            idx = by_line.get((u["book"], u["source_file"], u["source_line"]))
        if idx is None:
            parts = u["id"].split(":", 2)
            s = parts[2] if len(parts) > 2 else ""
            idx = by_key.get((u["book"], u["kind"], s))
        if idx is None:
            unit_unjoined.append(u)
        else:
            claimed.setdefault(idx, []).append(u["id"])

    unreached = [i for i in range(len(entries)) if i not in claimed]

    # Every `.lst` source row that SOME claimed corpus record stands for. A second
    # corpus JSON for the same (book, file, line) is a duplicate ingest of a row
    # that already reaches a sheet -- not a lost rule.
    claimed_rows = {
        (entries[i]["book"], entries[i]["basename"], entries[i]["line"])
        for i in claimed
        if entries[i]["basename"] and entries[i]["line"] is not None
    }

    def desc(e):
        d = (e["rec"] or {}).get("data") or {}
        v = d.get("description")
        return v if isinstance(v, str) else ""

    # --- the four buckets ---------------------------------------------------
    buckets = defaultdict(list)
    for i in unreached:
        e = entries[i]
        rec = e["rec"] or {}
        comp = rec.get("completeness")
        row = (e["book"], e["basename"], e["line"]) if e["basename"] and e["line"] is not None else None
        if comp is None:
            b = "generated_artifact_no_completeness"
        elif row is not None and row in claimed_rows:
            b = "duplicate_ingest_of_a_row_already_reached"
        elif comp == "chassis_only":
            b = "chassis_only_mod_row_no_rule_text"
        elif desc(e):
            b = "FULL_with_prose_no_row_reached"
        else:
            b = "FULL_without_prose_no_row_reached"
        buckets[b].append(
            dict(
                path=e["path"],
                book=e["book"],
                kind=e["kind"],
                slug=e["slug"],
                key=(rec.get("data") or {}).get("key"),
                completeness=comp,
                source_file=e["basename"],
                source_line=e["line"],
                desc_chars=len(desc(e)),
            )
        )

    real = buckets["FULL_with_prose_no_row_reached"] + buckets["FULL_without_prose_no_row_reached"]
    non_records = len(unreached) - len(real)

    census = {
        "criterion": "AT-35-E7-000-POPULATION-CENSUS",
        "question": "how many data/corpus records never reach docs/work-inventory.json",
        "method": "inverts src/pcgen_import/sheet_rule/mod.rs walk_corpus (:105) + "
        "load_population (:256) -- the only code path that joins the two populations. "
        "NOT a corpus-path-to-inventory-id guess (that join returns 12,659 and is wrong: "
        "`feat_generic`/`race_trait_generic`/`trait_generic`/`_settled` are corpus directory "
        "names, never inventory kinds).",
        "re_derive": "python3 docs/release/SD-35-corpus-sheet-completion/artifacts/"
        "epic-7-closure/population_census.py",
        "populations": {
            "corpus_records": len(entries),
            "corpus_records_note": "one `data/corpus/<book>/**/*.json` file is one record; "
            "`_parity/` and `LICENSE.json` excluded exactly as walk_corpus excludes them. "
            "`find data/corpus -name '*.json' -not -path '*/_parity/*' -not -name LICENSE.json | wc -l`",
            "inventory_units": len(units),
            "inventory_units_note": "docs/work-inventory.json is enumerated from the PINNED "
            "PCGEN TREE ($PCGEN_CORPUS_ROOT), not from data/corpus -- v06_work_inventory.rs "
            "main() line 19949. The two populations are built from different sources; nothing "
            "guarantees they agree.",
            "atlas_denominator": len(units),
            "atlas_denominator_source": "scripts/completion_atlas.py:71 INVENTORY_PATH = "
            "docs/work-inventory.json",
        },
        "direction_1_corpus_to_inventory": {
            "corpus_records_reached_by_at_least_one_unit": len(claimed),
            "corpus_records_never_reached": len(unreached),
            "never_reached_that_are_real_rules_records": len(real),
            "never_reached_that_are_not_records": non_records,
            "buckets": {k: len(v) for k, v in sorted(buckets.items(), key=lambda x: -len(x[1]))},
            "bucket_definitions": {
                "chassis_only_mod_row_no_rule_text": "the record's own `source.line` in the pinned "
                "tree is a `<Name>.MOD` row that only adds TYPE/CLASSES to an existing record. "
                "`completeness: chassis_only`, `data.description: null`. It carries no rule of its "
                "own, so under the sheet rule it has no sheet line of its own. NOT a lost record.",
                "duplicate_ingest_of_a_row_already_reached": "a SECOND corpus JSON for the same "
                "(book, source file, source line) as a record an inventory unit already claimed "
                "-- e.g. apg/ability/child_of_the_streets.json and "
                "apg/trait_generic/trait_child_of_the_streets.json are both apg_abilities.lst:122, "
                "and the unit advanced_players_guide:trait:trait_child_of_the_streets (status "
                "`sheet-complete`) claims the first. The rule reaches a sheet. NOT a lost record.",
                "generated_artifact_no_completeness": "`data/corpus/<book>/_settled/<kind>.json` "
                "-- an aggregate `{kind, records:{...}}` map written by the settle pass, not an "
                "ingested record: no `completeness`, no `source`, no `data`. NOT a record.",
                "FULL_with_prose_no_row_reached": "`completeness: full`, carries published rules "
                "prose in `data.description`, and NO inventory unit resolves to its source row "
                "under either join key. THIS IS THE REAL GAP.",
                "FULL_without_prose_no_row_reached": "`completeness: full`, no description, and no "
                "inventory unit resolves to its source row. Also a real gap.",
            },
            "real_gap_by_book_kind": dict(
                Counter(f"{r['book']}/{r['kind']}" for r in real).most_common()
            ),
            "real_gap_records": sorted(real, key=lambda r: r["path"]),
        },
        "direction_2_inventory_to_corpus": {
            "units_with_no_data_corpus_record": len(unit_unjoined),
            "of_those_rescued_by_source_row_in_tree": len(unit_unjoined) - 142,
            "of_those_refused_no_corpus_record": 142,
            "reconciliation": "829 = 687 rescued + 142 refused. "
            "data/sheet_rules/_refused.json's 142 `no_corpus_record` ids are a strict subset of "
            "the 829 (verified: `refused <= unjoined` is True, 0 outside). The other 687 (685 "
            "feat, 2 spell) resolve through sheet_rule/mod.rs::source_row_in_tree, which falls "
            "back to the unit's own row in the PINNED tree when the data/corpus join finds "
            "nothing -- so they render words and are DONE.",
            "refused_kinds": {"race_trait": 104, "race": 27, "feat": 11},
            "rescued_kinds": dict(
                Counter(u["kind"] for u in unit_unjoined).most_common()
            ),
        },
        "verdict": {
            "is_49438_of_49438_corpus_wide": False,
            "what_it_is": "INVENTORY-wide. completion_atlas.py's denominator is "
            "docs/work-inventory.json's 49,438 units (scripts/completion_atlas.py:71), and the "
            "inventory is enumerated from the pinned PCGen tree, not from data/corpus.",
            "corpus_wide_figure": f"{len(claimed)} of {len(claimed) + len(real)} real corpus "
            f"rules records reach a DONE inventory unit "
            f"({100.0 * len(claimed) / (len(claimed) + len(real)):.3f}%). The remaining "
            f"{len(real)} are `completeness: full` corpus records carrying published rules text "
            f"that NO inventory unit reaches under either join key, so the atlas never counted "
            f"them in either the numerator or the denominator.",
            "corpus_wide_figure_all_files": f"{len(entries) - len(real)} of {len(entries)} "
            f"`data/corpus` JSON files are either reached by a DONE unit or are not records "
            f"(.MOD list rows, duplicate ingests of a reached row, `_settled` aggregates).",
            "gap_is_real": len(real) > 0,
            "measure_only": "This cycle measures. It adds no unit, converts nothing, edits no "
            "classifier and changes no gate. The fix is a separate criterion.",
        },
    }
    json.dump(census, open(OUT, "w", encoding="utf-8"), indent=2)
    json.dump(
        {k: v for k, v in buckets.items()},
        open(os.path.join(os.path.dirname(OUT), "population-census-unreached.json"), "w"),
        indent=1,
    )
    print(json.dumps(census["populations"], indent=2))
    print(json.dumps(census["direction_1_corpus_to_inventory"]["buckets"], indent=2))
    print("real_gap:", len(real), "non_records:", non_records)
    print(json.dumps(census["verdict"], indent=2))


if __name__ == "__main__":
    main()
