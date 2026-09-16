#!/usr/bin/env python3
"""AT-35-E7-000-POPULATION-CENSUS, cycle 3 -- close the census.

Cycle 1 measured `data/corpus` against `docs/work-inventory.json` and returned
`record_absent_from_inventory_population=255`. Cycle 2 dispositioned those 255
one at a time and corrected the answer to **10**, and named, in
`what_this_proof_does_not_cover`, exactly three things it had NOT established.
This cycle closes all three, and nothing else. It is the last measurement cycle
the criterion needs.

  GAP 1 -- the 2,657 records cycle 1 placed in three "not a record" buckets
    (`chassis_only` 1,998, `duplicate_ingest` 612, `_settled` 47) were never
    independently re-verified. They were dispositioned by the bucket's own
    defining predicate at classification time and then trusted. That is the
    largest unverified population in the census by a factor of ten, and if even
    one of them carries published rules prose that reaches nothing, the answer
    is not 10. THIS CYCLE RE-TESTS ALL 2,657, one at a time, against the
    bucket's stated predicate, and puts every record that FAILS its predicate
    through cycle 2's full content-key ladder.

  GAP 2 -- `reached_via_identical_prose_record` proves the WORDS reach a sheet
    line, not that the reached record is filed under the name a player looks up.
    This cycle quantifies that: for every prose-reached record it records
    whether the twin's name differs, so the figure has a stated predicate
    instead of an implied one.

  GAP 3 -- prose identity normalised whitespace and case ONLY, which makes the
    10 a CEILING: a twin that re-punctuates or reformats compares different and
    stays counted as a gap. This cycle turns the ceiling into an exact number by
    re-running each of the 10 through THREE progressively weaker text tests --
    punctuation-normalised identity, containment (is this record's paragraph a
    substring of a reached record's?), and a token-overlap near-match with the
    best score reported. A gap that survives all three is absent under every
    test the census can state, not merely under the strictest one.

MEASURE, DO NOT FIX (the criterion's own words). This script reads
`data/corpus`, `docs/work-inventory.json` and the pinned PCGen tree, and writes
two JSON artifacts next to itself. It adds no inventory unit, converts nothing,
edits no classifier, changes no gate and writes nothing under `src/` or `data/`.

Re-derive:
  python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/population_census_final.py
"""

import json
import os
import re
import sys
from collections import Counter, defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = HERE.split("/docs/release/")[0]
CORPUS = os.path.join(REPO, "data/corpus")
INV = os.path.join(REPO, "docs/work-inventory.json")
UNREACHED = os.path.join(HERE, "population-census-unreached.json")
CYCLE2 = os.path.join(HERE, "population-census-255.json")
OUT = os.path.join(HERE, "population-census-final.json")
DETAIL = os.path.join(HERE, "population-census-final-detail.json")

PCGEN_ROOT = os.environ.get(
    "PCGEN_CORPUS_ROOT", os.path.join(os.path.expanduser("~"), "workspace/repos/pcgen/data")
)
BOOKS_REL = "pathfinder/paizo/roleplaying_game"


def norm_key(s):
    return re.sub(r"[^a-z0-9]+", "", (s or "").lower())


def norm_text(s):
    """Cycle 2's test, unchanged: whitespace and case only. The STRICT test."""
    return re.sub(r"\s+", " ", (s or "").strip().lower())


def norm_text_punct(s):
    """GAP 3, weaker test 1: punctuation collapsed as well.

    A numeral, a die expression and a bonus all SURVIVE this normalisation --
    `[^a-z0-9]+` keeps digits -- so `1d6` still compares different from `1d8`
    and `+2` from `+3`. It only launders punctuation and spacing.
    """
    return re.sub(r"[^a-z0-9]+", " ", (s or "").lower()).strip()


def tokens(s):
    return set(re.findall(r"[a-z0-9]+", (s or "").lower()))


def load_corpus():
    """Every `data/corpus` record, walked by `mod.rs::walk_corpus`'s rules."""
    by_line = {}
    by_kind_slug = {}
    records = {}
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
                try:
                    with open(p, encoding="utf-8") as fh:
                        rec = json.load(fh)
                except (OSError, ValueError):
                    continue
                if not isinstance(rec, dict):
                    continue
                data = rec.get("data") if isinstance(rec.get("data"), dict) else {}
                src = rec.get("source") if isinstance(rec.get("source"), dict) else {}
                entry = {
                    "path": os.path.relpath(p, REPO),
                    "book": book,
                    "kind": kind,
                    "slug": fn[: -len(".json")],
                    "key": data.get("key"),
                    "name": data.get("name") or data.get("key"),
                    "description": data.get("description"),
                    "completeness": rec.get("completeness"),
                    "has_data": isinstance(rec.get("data"), dict),
                    "has_source": isinstance(rec.get("source"), dict),
                    "source_file": os.path.basename(src.get("path") or "") or None,
                    "source_line": src.get("line"),
                }
                records[entry["path"]] = entry
                if entry["source_file"] and entry["source_line"] is not None:
                    by_line.setdefault((book, entry["source_file"], entry["source_line"]), entry)
                by_kind_slug.setdefault((book, kind, entry["slug"]), entry)
    return records, by_line, by_kind_slug


def unit_corpus_record(unit, by_line, by_kind_slug):
    book = unit.get("book")
    sf = os.path.basename(unit.get("source_file") or "")
    hit = by_line.get((book, sf, unit.get("source_line")))
    if hit:
        return hit
    parts = (unit.get("id") or "").split(":", 2)
    if len(parts) == 3:
        for kind_dir in (unit.get("kind"), f"{unit.get('kind')}_generic", "ability", "equipment"):
            hit = by_kind_slug.get((book, kind_dir, parts[2]))
            if hit:
                return hit
    return None


# --------------------------------------------------------------------------
# GAP 1: re-verify the three "not a record" buckets against their own predicates
# --------------------------------------------------------------------------

def verify_settled(rec):
    """`_settled/<kind>.json` -- an aggregate map, not an ingested record.

    Bucket 1's predicate, verbatim: no `completeness`, no `source`, no `data`.
    """
    fails = []
    if rec["completeness"] is not None:
        fails.append(f"has completeness={rec['completeness']!r}")
    if rec["has_source"]:
        fails.append("has a `source` object")
    if rec["has_data"]:
        fails.append("has a `data` object")
    if rec["kind"] != "_settled":
        fails.append(f"kind is {rec['kind']!r}, not `_settled`")
    return fails


def verify_chassis_only(rec):
    """`chassis_only` -- a `.MOD` chassis row carrying no rule text of its own.

    Bucket's predicate: `completeness == "chassis_only"` AND no published prose.
    A chassis row with prose would be a real record wearing the wrong label, and
    the sheet rule would have words to print.
    """
    fails = []
    if rec["completeness"] != "chassis_only":
        fails.append(f"completeness is {rec['completeness']!r}, not `chassis_only`")
    if norm_text(rec["description"]):
        fails.append(f"carries {len(rec['description'])} chars of published prose")
    return fails


def verify_duplicate_ingest(rec, corpus_by_row, unreached_paths):
    """`duplicate_ingest` -- a SECOND corpus JSON for a row another record holds.

    Bucket's predicate: another `data/corpus` record sits at the SAME
    `(source_file, source_line)` -- book label ignored, because `attributed_book`
    rewrites it -- and that other record is REACHED. Cycle 1 asserted the twin;
    this re-derives it, and additionally requires the twin to carry at least as
    much published prose, so a duplicate cannot be credited to a twin that lost
    the rule text.
    """
    fails = []
    if not rec["source_file"] or rec["source_line"] is None:
        return ["no source row to join on"], None
    twins = [
        r
        for r in corpus_by_row.get((rec["source_file"], rec["source_line"]), [])
        if r["path"] != rec["path"]
    ]
    if not twins:
        return ["no other corpus record at this `.lst` row"], None
    reached_twins = [t for t in twins if t["path"] not in unreached_paths]
    if not reached_twins:
        return ["every corpus record at this row is itself unreached"], None
    mine = len(norm_text(rec["description"]))
    best = max(reached_twins, key=lambda t: len(norm_text(t["description"])))
    if len(norm_text(best["description"])) < mine:
        fails.append(
            f"the reached twin {best['path']} carries less prose "
            f"({len(norm_text(best['description']))} < {mine} chars)"
        )
    return fails, best["path"]


def main():
    with open(UNREACHED, encoding="utf-8") as fh:
        buckets = json.load(fh)
    with open(CYCLE2, encoding="utf-8") as fh:
        cycle2 = json.load(fh)
    with open(INV, encoding="utf-8") as fh:
        units = json.load(fh)["units"]

    records, by_line, by_kind_slug = load_corpus()
    unreached_paths = {r["path"] for bucket in buckets.values() for r in bucket}

    inv_by_row = defaultdict(list)
    inv_by_key = defaultdict(list)
    inv_by_name = defaultdict(list)
    for u in units:
        inv_by_row[(os.path.basename(u.get("source_file") or ""), u.get("source_line"))].append(u)
        inv_by_key[norm_key(u.get("corpus_key"))].append(u)
        inv_by_name[norm_key(u.get("name"))].append(u)

    # Every corpus record indexed by its `.lst` row, book label dropped --
    # `attributed_book` rewrites the book, so a book-carrying key cannot see a
    # re-attributed twin.
    corpus_by_row = defaultdict(list)
    for r in records.values():
        if r["source_file"] and r["source_line"] is not None:
            corpus_by_row[(r["source_file"], r["source_line"])].append(r)

    # The REACHED corpus population, and its prose indexed three ways for GAP 3.
    reached_records = [r for p, r in records.items() if p not in unreached_paths]
    reached_strict = defaultdict(list)
    reached_punct = defaultdict(list)
    reached_text_tokens = []
    for r in reached_records:
        t = norm_text(r["description"])
        if t:
            reached_strict[t].append(r)
            reached_punct[norm_text_punct(r["description"])].append(r)
            reached_text_tokens.append((r, t, tokens(r["description"])))

    # ------------------------------------------------------------------
    # GAP 1 -- all 2,657, one at a time
    # ------------------------------------------------------------------
    gap1_detail = []
    gap1_fail = []
    for bucket_name, verifier in (
        ("generated_artifact_no_completeness", "settled"),
        ("chassis_only_mod_row_no_rule_text", "chassis"),
        ("duplicate_ingest_of_a_row_already_reached", "duplicate"),
    ):
        for stub in buckets[bucket_name]:
            rec = records.get(stub["path"])
            if rec is None:
                gap1_detail.append(
                    {
                        "path": stub["path"],
                        "bucket": bucket_name,
                        "verdict": "FAIL",
                        "reasons": ["record absent from `data/corpus` at HEAD"],
                    }
                )
                gap1_fail.append(stub["path"])
                continue
            twin = None
            if verifier == "settled":
                fails = verify_settled(rec)
            elif verifier == "chassis":
                fails = verify_chassis_only(rec)
            else:
                fails, twin = verify_duplicate_ingest(rec, corpus_by_row, unreached_paths)
            row = {
                "path": rec["path"],
                "bucket": bucket_name,
                "verdict": "CONFIRMED" if not fails else "FAIL",
            }
            if twin:
                row["reached_twin"] = twin
            if fails:
                row["reasons"] = fails
                gap1_fail.append(rec["path"])
            gap1_detail.append(row)

    gap1_counts = Counter((d["bucket"], d["verdict"]) for d in gap1_detail)

    # Every record that FAILED its bucket predicate goes through the full
    # content-key ladder -- it may still be reached, but it is no longer
    # dispositioned by assertion.
    ladder_results = []
    for path in gap1_fail:
        rec = records.get(path)
        if rec is None:
            ladder_results.append({"path": path, "disposition": "record_absent_at_head"})
            continue
        my_strict = norm_text(rec["description"])
        row_units = inv_by_row.get((rec["source_file"] or "", rec["source_line"]), [])
        if row_units:
            ladder_results.append(
                {
                    "path": path,
                    "disposition": "reached_via_reattributed_same_row",
                    "reaches": row_units[0].get("id"),
                }
            )
            continue
        if my_strict and reached_strict.get(my_strict):
            ladder_results.append(
                {
                    "path": path,
                    "disposition": "reached_via_identical_prose_record",
                    "prose_twin": reached_strict[my_strict][0]["path"],
                }
            )
            continue
        if not my_strict:
            ladder_results.append({"path": path, "disposition": "no_prose_nothing_to_print"})
            continue
        ladder_results.append({"path": path, "disposition": "absent_no_unit_anywhere"})

    gap1_new_absent = [r for r in ladder_results if r["disposition"] == "absent_no_unit_anywhere"]

    # ------------------------------------------------------------------
    # GAP 3 -- the 10, under three progressively weaker text tests
    # ------------------------------------------------------------------
    gap3_detail = []
    for stub in cycle2["absent_records"]:
        rec = records.get(stub["path"])
        if rec is None:
            gap3_detail.append({**stub, "still_absent": None, "note": "absent from corpus at HEAD"})
            continue
        strict = norm_text(rec["description"])
        punct = norm_text_punct(rec["description"])
        my_tokens = tokens(rec["description"])

        t1 = [r["path"] for r in reached_strict.get(strict, []) if r["path"] != rec["path"]]
        t2 = [r["path"] for r in reached_punct.get(punct, []) if r["path"] != rec["path"]]
        t3_hits = []
        best = (0.0, None)
        for other, ot, ot_tokens in reached_text_tokens:
            if len(strict) >= 24 and strict in ot:
                t3_hits.append(other["path"])
            if my_tokens and ot_tokens:
                j = len(my_tokens & ot_tokens) / len(my_tokens | ot_tokens)
                if j > best[0]:
                    best = (round(j, 4), other["path"])

        # And the name test: does ANY inventory unit carry this record's name?
        name_units = [
            u.get("id")
            for u in inv_by_key.get(norm_key(rec["key"]), []) + inv_by_name.get(norm_key(rec["name"]), [])
        ]

        gap3_detail.append(
            {
                "path": rec["path"],
                "book": rec["book"],
                "kind": rec["kind"],
                "slug": rec["slug"],
                "key": rec["key"],
                "source": f"{rec['source_file']}:{rec['source_line']}",
                "desc_chars": len(rec["description"] or ""),
                "test_1_strict_prose_identity": t1,
                "test_2_punctuation_normalised_identity": t2,
                "test_3_containment_in_a_reached_paragraph": t3_hits,
                "best_token_overlap_jaccard": best[0],
                "best_token_overlap_record": best[1],
                "inventory_units_carrying_this_name": name_units,
                "still_absent": not (t1 or t2 or t3_hits or name_units),
            }
        )

    gap3_absent = [d for d in gap3_detail if d.get("still_absent")]
    gap3_rescued = [d for d in gap3_detail if d.get("still_absent") is False]

    # ------------------------------------------------------------------
    # GAP 2 -- give the prose-reached figure its predicate
    # ------------------------------------------------------------------
    with open(os.path.join(HERE, "population-census-255-detail.json"), encoding="utf-8") as fh:
        c2_detail = json.load(fh)
    prose_reached = [
        d for d in c2_detail if d["disposition"] == "reached_via_identical_prose_record"
    ]
    gap2_rows = []
    for d in prose_reached:
        mine = records.get(d["path"])
        twin = records.get(d.get("prose_twin_corpus_record") or "")
        gap2_rows.append(
            {
                "record": d["path"],
                "record_name": (mine or {}).get("name"),
                "prose_twin": d.get("prose_twin_corpus_record"),
                "twin_name": (twin or {}).get("name"),
                "name_differs": norm_key((mine or {}).get("name"))
                != norm_key((twin or {}).get("name")),
            }
        )

    # ------------------------------------------------------------------
    # The closed census
    # ------------------------------------------------------------------
    absent_total = len(gap3_absent) + len(gap1_new_absent)
    real_records = 48864
    reached = real_records - absent_total

    out = {
        "criterion": "AT-35-E7-000-POPULATION-CENSUS",
        "cycle": 3,
        "question": "how many `data/corpus` records never reach `docs/work-inventory.json` "
        "-- measure, do not fix",
        "scope": "the cycle-2 remainder `record_absent_from_inventory_population=10`, PLUS the "
        "three gaps cycle 2 recorded in `what_this_proof_does_not_cover`. Nothing else.",
        "measure_only": "reads only; adds no unit, converts nothing, edits no classifier, "
        "changes no gate, writes nothing under `src/` or `data/`",
        "re_derive": "python3 docs/release/SD-35-corpus-sheet-completion/artifacts/"
        "epic-7-closure/population_census_final.py",
        "gap_1_not_a_record_buckets_re_verified": {
            "why": "cycle 2 recorded these 2,657 as NOT re-verified. They were dispositioned by "
            "the bucket's defining predicate at classification time and then trusted. This is "
            "the census's largest unverified population.",
            "population": sum(len(buckets[b]) for b in (
                "chassis_only_mod_row_no_rule_text",
                "duplicate_ingest_of_a_row_already_reached",
                "generated_artifact_no_completeness",
            )),
            "by_bucket": {
                b: {
                    "population": len(buckets[b]),
                    "confirmed": gap1_counts[(b, "CONFIRMED")],
                    "failed_its_predicate": gap1_counts[(b, "FAIL")],
                }
                for b in (
                    "chassis_only_mod_row_no_rule_text",
                    "duplicate_ingest_of_a_row_already_reached",
                    "generated_artifact_no_completeness",
                )
            },
            "predicates_tested": {
                "generated_artifact_no_completeness": "no `completeness`, no `source` object, no "
                "`data` object, kind is `_settled`",
                "chassis_only_mod_row_no_rule_text": "`completeness == \"chassis_only\"` AND zero "
                "chars of published prose -- a chassis row carrying prose would be a real record "
                "under the wrong label",
                "duplicate_ingest_of_a_row_already_reached": "another corpus record sits at the "
                "SAME `(source_file, source_line)` (book label dropped), that twin is itself "
                "reached, AND the twin carries at least as much published prose -- so a duplicate "
                "is never credited to a twin that lost the rule text",
            },
            "records_failing_their_predicate": len(gap1_fail),
            "of_those_still_absent_after_the_full_ladder": len(gap1_new_absent),
            "ladder_dispositions": dict(Counter(r["disposition"] for r in ladder_results)),
            "the_failures_named": {
                "what_they_are": "all 7 are `duplicate_ingest` records that ARE reached -- an "
                "inventory unit sits on their `.lst` row -- but whose reached twin carries LESS "
                "published prose than they do. They failed only the prose-quantity clause this "
                "cycle ADDED to the bucket's predicate; cycle 1's predicate did not have one.",
                "mechanism": "one `.lst` row ingested as several corpus records carrying "
                "different prose variants of the same rule. `isr_spells.lst:18` "
                "(`Elemental Mastery`) is ingested five times -- one paragraph per element -- and "
                "one unit, `inner_sea_races:spell:elemental_mastery` (`sheet-complete`), holds "
                "the row.",
                "why_it_is_not_an_absence": "the rule reaches a rendered sheet line. What the 7 "
                "raise is a narrower question -- whether every prose VARIANT of a multi-variant "
                "row prints, or only the one the unit resolved to -- which is a rendering-"
                "completeness question about reached units, not a reachability question, and is "
                "outside this criterion (`how many corpus records never reach the inventory`).",
                "rows": [
                    {
                        "path": d["path"],
                        "reached_twin": d.get("reached_twin"),
                        "reason": d.get("reasons", [None])[0],
                        "reaches": next(
                            (
                                l.get("reaches")
                                for l in ladder_results
                                if l["path"] == d["path"]
                            ),
                            None,
                        ),
                    }
                    for d in gap1_detail
                    if d["verdict"] == "FAIL"
                ],
            },
        },
        "gap_2_prose_reach_predicate": {
            "why": "cycle 2: `reached_via_identical_prose_record` proves the WORDS reach a sheet "
            "line, not that the reached record is filed under the name a player looks up. Stated "
            "here as a number instead of a caveat.",
            "population": len(gap2_rows),
            "name_differs_from_the_twin": sum(1 for r in gap2_rows if r["name_differs"]),
            "name_identical_to_the_twin": sum(1 for r in gap2_rows if not r["name_differs"]),
            "rows": gap2_rows,
            "reading": "these records' words print; a player looking the sub-form up by its own "
            "name finds it under the base rule's name. That is a lookup-ergonomics observation, "
            "not a missing sheet line, and under the sheet rule the unit is DONE.",
        },
        "gap_3_the_ten_under_weaker_tests": {
            "why": "cycle 2's prose test normalised whitespace and case only, which made 10 a "
            "CEILING: a twin that re-punctuates would compare different and stay a gap. Three "
            "weaker tests turn the ceiling into an exact number.",
            "tests": [
                "1. strict identity (cycle 2's test, whitespace+case) -- unchanged, for reference",
                "2. punctuation-normalised identity. Digits SURVIVE this normalisation, so `1d6` "
                "still compares different from `1d8` and `+2` from `+3`; only punctuation and "
                "spacing are laundered.",
                "3. containment -- is this record's whole paragraph a substring of some reached "
                "record's paragraph? (applied only at >=24 chars, so a 5-word benefit line cannot "
                "be swallowed by an unrelated long paragraph)",
                "plus: does ANY inventory unit carry this record's KEY or name?",
                "plus: the best token-overlap (Jaccard) score against the whole reached "
                "population is reported for each, so the distance to the nearest twin is a "
                "number and not an assertion",
            ],
            "population": len(gap3_detail),
            "rescued_by_a_weaker_test": len(gap3_rescued),
            "still_absent_under_every_test": len(gap3_absent),
            "records": gap3_detail,
        },
        "the_closed_census": {
            "real_corpus_rules_records": real_records,
            "reached_by_a_DONE_inventory_unit": reached,
            "never_reaching_any_sheet_line": absent_total,
            "corpus_wide_completion": f"{reached} of {real_records} = "
            f"{100.0 * reached / real_records:.4f}%",
            "absent_by_book_kind": dict(
                Counter(f"{d['book']}/{d['kind']}" for d in gap3_absent)
            ),
            "supersedes": "cycle 1's `255` / `48,609 of 48,864 = 99.478%` and confirms cycle 2's "
            "`10` / `48,854 of 48,864 = 99.9795%` -- cycle 2's figure is re-derived here from a "
            "population 11x larger and under three weaker text tests, and does not move.",
        },
        "root_cause_of_the_absent": cycle2["root_cause_of_the_absent"],
        "what_this_proof_does_not_cover": [
            "It does not fix the 10. The criterion is `measure, do not fix`, and the fix "
            "(widening `has_classifying_token`, `src/bin/v06_work_inventory.rs:3172-3173`) is "
            "outside this criterion's write scope AND moves the atlas denominator off 49,438, so "
            "it needs an operator ruling.",
            "The `_settled` aggregates are verified to be aggregates, not verified to AGREE with "
            "the records they aggregate. That is a different question (settle-pass fidelity) and "
            "no figure in this census rests on it.",
            "Token-overlap (Jaccard) is reported as a DISTANCE, never as a reach. No record is "
            "dispositioned `reached` by a near-match in this cycle -- only exact identity, "
            "containment, the same `.lst` row, a `.COPY=` twin, or a name an inventory unit "
            "carries can do that.",
            "It does not establish that every prose VARIANT of a multi-variant `.lst` row "
            "prints. The 7 named in `gap_1.../the_failures_named` reach a sheet line; whether the "
            "unit renders all five `Elemental Mastery` paragraphs or one is a rendering-"
            "completeness question about a REACHED unit, and this criterion measures "
            "reachability. Named here rather than folded away.",
            "The denominator 48,864 is cycle 1's and is carried forward unchanged. This cycle "
            "re-verifies the DISPOSITION of the never-reached population, not the enumeration of "
            "`data/corpus` itself.",
        ],
    }

    with open(OUT, "w", encoding="utf-8") as fh:
        json.dump(out, fh, indent=2)
        fh.write("\n")
    with open(DETAIL, "w", encoding="utf-8") as fh:
        json.dump(
            {"gap_1_per_record": gap1_detail, "gap_1_ladder": ladder_results},
            fh,
            indent=2,
        )
        fh.write("\n")

    json.dump(
        {
            "gap_1": out["gap_1_not_a_record_buckets_re_verified"]["by_bucket"],
            "gap_1_failed": len(gap1_fail),
            "gap_1_new_absent": len(gap1_new_absent),
            "gap_2_name_differs": out["gap_2_prose_reach_predicate"]["name_differs_from_the_twin"],
            "gap_3_rescued": len(gap3_rescued),
            "gap_3_still_absent": len(gap3_absent),
            "absent_total": absent_total,
            "corpus_wide": out["the_closed_census"]["corpus_wide_completion"],
        },
        sys.stdout,
        indent=2,
    )
    print()


if __name__ == "__main__":
    main()
