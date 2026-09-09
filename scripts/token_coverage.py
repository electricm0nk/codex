#!/usr/bin/env python3
"""The token-coverage ledger -- SD-35 `AT-35-E2-004`
(`docs/release/SD-35-corpus-sheet-completion/epic-breakdown.md`), enforcing
`decisions.md` §8 (every figure states its denominator) and `§9` L9 (sum the
piles, always). From this cycle on the remainder is named by token type.

Why this exists
----------------
`decisions.md` §2 makes a cycle "one mechanism, corpus-wide". A mechanism is
a token type: the converter (`src/pcgen_import/sheet_rule/`) converts or
refuses a record per mapping-table row, so the population a cycle can take
is "every non-DONE unit carrying token type X", and what is left is "every
unit refused because of token type X". This script writes that ledger and
checks its sums, so a receipt's "Refused tokens" row and a dispatch prompt's
scope are read from one derived artifact, never re-counted by eye.

What it reads (tool side -- it may read the converter's view of PCGen)
-----------------------------------------------------------------------
    data/sheet_rules/_tokens.json    the converter's token census: per record,
                                     the mapping-table row key of every token
                                     its closure carried (`BONUS:VAR`, `DESC`,
                                     `PREVARGTEQ`, ... ; `unmapped:<HEAD>` /
                                     `BONUS:<SUB>` when the table has no row)
                                     and, per refusal shape, the token type it
                                     arose under (`token-less` for a record
                                     with no source row at all)
    data/sheet_rules/_refused.json   the converter's refusal report
    data/sheet_rules/_report.json    the converter's population sums
    docs/work-inventory.json         the population; DONE exactly as
                                     `scripts/completion_atlas.py` partitions it
    artifacts/epic-2-sheet-rule/token-mapping/mapping-table.v1.json
                                     the converter mapping row per token type

The census is the converter's own reading of the closure (base row, `.COPY=`
base, corpus-wide `.MOD` rows, class level lines, the `.lst` row for the
no-`raw_tokens` units) -- `data/corpus/**`'s shipped `raw_tokens` are NOT the
population the converter converts (142 units resolve to no corpus record and
no same-book source row, SD-35 AT-35-E3-002; it was 829 before that cycle),
so a second reading here would count a different thing.

What it writes
--------------
    docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/token-coverage.json

re-derived whole, byte-identical for identical inputs (no timestamp, no HEAD
sha). Per token type: `carrying` (all statuses) / `carrying_non_done`,
`converted_non_done`, `refused_non_done` (carrying non-DONE units the
converter refused for ANY token), `refused_because_of_this_token` (all
statuses and non-DONE -- a record refused by k shapes counts under each
shape's token), the refusal shapes that arose under it, and its mapping row
(`family`, `maps_to`) or `null`. Plus the per-shape section
(`blockers.md` B10: refusals counted per shape) and the sum checks.

Sums checked (`--check` exits 1 on the first that fails)
-------------------------------------------------------
    POPULATION      census entries == inventory units == _report.records;
                    converted + refused == records
    DOUBLE_COUNT    no record twice in the census; no token twice on one
                    record; no `under` twice on one refusal
    COVERAGE        every non-DONE unit is under >= 1 token XOR `token-less`
    REFUSED_SET     the ids the census refuses == `_refused.json`'s id set,
                    shape for shape; the union over token types of "refused
                    because of this token" == that same id set
    SHAPE_TOTALS    per shape, census records == `_refused.json` by_token_type
    PARTITION       per token type, carrying_non_done == converted_non_done
                    + refused_non_done
    STALE_ARTIFACT  (`--check` only) the committed ledger equals the fresh
                    one; when it does not, the fresh one is written so the
                    fix is one commit

Modes
-----
    (no flag)   derive, write the ledger, print the sums; exit 0 unless a sum fails
    --check     the same, plus the committed-ledger freshness check
                last line:  non_done=<n> tokened=<n> token_less=<n> refused=<n>
                            refused_non_done=<n> token_types=<n> shapes=<n>
                            verdict=PASS|FAIL_<check>
                exit 0 on PASS, 1 on any FAIL, 2 on a missing/invalid input.
    --inventory / --package / --table / --out   override the four paths (tests)
"""

import argparse
import collections
import json
import os
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import completion_atlas as _atlas  # noqa: E402  (the partition is the atlas's, not a copy)

INVENTORY_PATH = os.path.join(REPO_ROOT, "docs", "work-inventory.json")
PACKAGE_DIR = os.path.join(REPO_ROOT, "data", "sheet_rules")
BUNDLE_DIR = os.path.join(REPO_ROOT, "docs", "release", "SD-35-corpus-sheet-completion")
TABLE_PATH = os.path.join(BUNDLE_DIR, "artifacts", "epic-2-sheet-rule", "token-mapping", "mapping-table.v1.json")
OUT_PATH = os.path.join(BUNDLE_DIR, "artifacts", "epic-2-sheet-rule", "token-coverage.json")

TOKEN_LESS = "token-less"
SCHEMA = "token-coverage.v1"


class InputError(Exception):
    """A missing or malformed input: exit 2, never a pass."""


def _load_json(path, what):
    try:
        with open(path, encoding="utf-8") as fh:
            return json.load(fh)
    except FileNotFoundError:
        raise InputError(f"{what} missing at {path}")
    except json.JSONDecodeError as exc:
        raise InputError(f"{what} is not valid JSON ({path}): {exc}")


def is_done(unit):
    return _atlas._bucket_of(unit) == "DONE"


def _mapping_rows(table):
    rows = {}
    for row in table.get("rows") or []:
        tt = row.get("token_type")
        if tt:
            rows[tt] = {"family": row.get("family"), "maps_to": row.get("maps_to")}
    return rows


# The verdict names the most SPECIFIC failed check, not the first detected: a planted
# duplicate also throws the coarse population sum off, and the verdict must say
# DOUBLE_COUNT, not POPULATION. Every failure is printed regardless.
VERDICT_PRIORITY = ("DOUBLE_COUNT", "COVERAGE", "REFUSED_SET", "SHAPE_TOTALS", "PARTITION", "POPULATION")


class Check:
    def __init__(self):
        self.failures = []   # (name, detail)

    def fail(self, name, detail):
        self.failures.append((name, detail))

    @property
    def verdict(self):
        if not self.failures:
            return "PASS"
        names = {n for n, _ in self.failures}
        for name in VERDICT_PRIORITY:
            if name in names:
                return f"FAIL_{name}"
        return f"FAIL_{self.failures[0][0]}"


def derive(inventory, census, refused, report, table):
    """Build the ledger dict and the list of failed checks from the loaded inputs."""
    check = Check()
    units = inventory.get("units") or []
    unit_by_id = {u["id"]: u for u in units}
    non_done = {u["id"] for u in units if not is_done(u)}

    entries = census.get("entries") or []

    # --- POPULATION -------------------------------------------------------
    if len(entries) != len(units) or report.get("records") != len(units):
        check.fail("POPULATION", f"census entries={len(entries)} inventory units={len(units)} report.records={report.get('records')}")
    if (report.get("converted") or 0) + (report.get("refused") or 0) != (report.get("records") or 0):
        check.fail("POPULATION", f"report converted={report.get('converted')} + refused={report.get('refused')} != records={report.get('records')}")
    census_ids = [e["id"] for e in entries]
    if set(census_ids) != set(unit_by_id):
        missing = sorted(set(unit_by_id) - set(census_ids))[:5]
        extra = sorted(set(census_ids) - set(unit_by_id))[:5]
        check.fail("POPULATION", f"census ids != inventory ids (missing e.g. {missing}, extra e.g. {extra})")

    # --- DOUBLE_COUNT -----------------------------------------------------
    dup_ids = [i for i, n in collections.Counter(census_ids).items() if n > 1]
    if dup_ids:
        check.fail("DOUBLE_COUNT", f"{len(dup_ids)} record(s) appear more than once in the census, e.g. {sorted(dup_ids)[:5]}")
    for e in entries:
        toks = e.get("tokens") or []
        if len(toks) != len(set(toks)):
            check.fail("DOUBLE_COUNT", f"{e['id']} lists a token twice: {sorted(t for t, n in collections.Counter(toks).items() if n > 1)}")
        for shape, under in (e.get("refusals") or {}).items():
            if len(under) != len(set(under)):
                check.fail("DOUBLE_COUNT", f"{e['id']} lists {shape!r} under one token twice: {under}")

    # --- per-record views (first occurrence wins when a duplicate was planted;
    # the DOUBLE_COUNT failure above already decides the verdict) -----------
    tokens_of = {}
    refusals_of = {}
    for e in entries:
        tokens_of.setdefault(e["id"], list(dict.fromkeys(e.get("tokens") or [])))
        refusals_of.setdefault(e["id"], {s: list(dict.fromkeys(u)) for s, u in (e.get("refusals") or {}).items()})
    refused_ids = {e["id"] for e in refused.get("entries") or []}
    refused_shapes_of = {e["id"]: set(e.get("token_types") or []) for e in refused.get("entries") or []}

    # --- COVERAGE ---------------------------------------------------------
    tokened_non_done, token_less_non_done, uncovered = set(), set(), []
    for uid in sorted(non_done):
        toks = tokens_of.get(uid)
        if toks is None:
            uncovered.append(uid)
        elif toks:
            tokened_non_done.add(uid)
        else:
            token_less_non_done.add(uid)
    if uncovered:
        check.fail("COVERAGE", f"{len(uncovered)} non-DONE unit(s) under no token and not token-less, e.g. {uncovered[:5]}")

    # --- REFUSED_SET ------------------------------------------------------
    census_refused = {uid for uid, r in refusals_of.items() if r}
    if census_refused != refused_ids:
        only_census = sorted(census_refused - refused_ids)[:5]
        only_report = sorted(refused_ids - census_refused)[:5]
        check.fail("REFUSED_SET", f"census refuses {len(census_refused)} record(s), _refused.json lists {len(refused_ids)} (census-only e.g. {only_census}; report-only e.g. {only_report})")
    else:
        for uid in sorted(refused_ids):
            if set(refusals_of[uid]) != refused_shapes_of[uid]:
                check.fail("REFUSED_SET", f"{uid}: census shapes {sorted(refusals_of[uid])} != _refused.json {sorted(refused_shapes_of[uid])}")
                break

    # --- the ledger rows --------------------------------------------------
    rows = collections.defaultdict(lambda: {
        "carrying": set(), "carrying_non_done": set(),
        "refused_because_of_this_token": set(), "refused_because_of_this_token_non_done": set(),
        "refusal_shapes": collections.Counter(),
    })
    shapes = collections.defaultdict(lambda: {"records": set(), "non_done": set(), "under": collections.Counter()})
    for uid, toks in tokens_of.items():
        keys = toks if toks else [TOKEN_LESS]
        for t in keys:
            rows[t]["carrying"].add(uid)
            if uid in non_done:
                rows[t]["carrying_non_done"].add(uid)
        for shape, under in refusals_of.get(uid, {}).items():
            shapes[shape]["records"].add(uid)
            if uid in non_done:
                shapes[shape]["non_done"].add(uid)
            for t in under:
                shapes[shape]["under"][t] += 1
                rows[t]["refused_because_of_this_token"].add(uid)
                rows[t]["refusal_shapes"][shape] += 1
                if uid in non_done:
                    rows[t]["refused_because_of_this_token_non_done"].add(uid)

    union_refused_by_token = set()
    for r in rows.values():
        union_refused_by_token |= r["refused_because_of_this_token"]
    if union_refused_by_token != refused_ids and not any(n == "REFUSED_SET" for n, _ in check.failures):
        check.fail("REFUSED_SET", f"union over token types of refused-because-of-this-token = {len(union_refused_by_token)} != _refused.json {len(refused_ids)}")

    # --- SHAPE_TOTALS -----------------------------------------------------
    by_token_type = refused.get("by_token_type") or {}
    census_shape_counts = {s: len(v["records"]) for s, v in shapes.items()}
    if census_shape_counts != {s: n for s, n in by_token_type.items()}:
        diffs = [(s, census_shape_counts.get(s), by_token_type.get(s)) for s in sorted(set(census_shape_counts) | set(by_token_type)) if census_shape_counts.get(s) != by_token_type.get(s)]
        check.fail("SHAPE_TOTALS", f"{len(diffs)} shape(s) do not sum (shape, census, _refused.json), e.g. {diffs[:5]}")

    # --- PARTITION --------------------------------------------------------
    mapping = _mapping_rows(table)
    token_types = {}
    for t in sorted(rows):
        r = rows[t]
        converted_nd = {u for u in r["carrying_non_done"] if u not in refused_ids}
        refused_nd = {u for u in r["carrying_non_done"] if u in refused_ids}
        if len(converted_nd) + len(refused_nd) != len(r["carrying_non_done"]):
            check.fail("PARTITION", f"{t}: carrying_non_done={len(r['carrying_non_done'])} != converted {len(converted_nd)} + refused {len(refused_nd)}")
        token_types[t] = {
            "mapping_row": mapping.get(t) if t != TOKEN_LESS else None,
            "carrying": len(r["carrying"]),
            "carrying_non_done": len(r["carrying_non_done"]),
            "converted_non_done": len(converted_nd),
            "refused_non_done": len(refused_nd),
            "refused_because_of_this_token": len(r["refused_because_of_this_token"]),
            "refused_because_of_this_token_non_done": len(r["refused_because_of_this_token_non_done"]),
            "refusal_shapes": dict(sorted(r["refusal_shapes"].items())),
        }
    # Sorted by non-DONE weight, then name: the next cycle's scope reads from the top.
    ordered = sorted(token_types.items(), key=lambda kv: (-kv[1]["carrying_non_done"], -kv[1]["refused_because_of_this_token"], kv[0]))
    token_types = dict(ordered)
    unmapped = sorted(t for t, v in token_types.items() if v["mapping_row"] is None and t != TOKEN_LESS)

    refusal_shapes = {
        s: {"records": len(v["records"]), "non_done": len(v["non_done"]), "under": dict(sorted(v["under"].items()))}
        for s, v in sorted(shapes.items(), key=lambda kv: (-len(kv[1]["records"]), kv[0]))
    }
    refused_non_done = refused_ids & non_done

    sums = {
        "population": {"census_entries": len(entries), "inventory_units": len(units), "report_records": report.get("records"), "ok": not any(n == "POPULATION" for n, _ in check.failures)},
        "double_count": {"duplicate_records": len(dup_ids), "ok": not any(n == "DOUBLE_COUNT" for n, _ in check.failures)},
        "coverage": {"non_done": len(non_done), "tokened": len(tokened_non_done), "token_less": len(token_less_non_done), "uncovered": len(uncovered), "ok": not uncovered},
        "refused_set": {"census_refused": len(census_refused), "refused_json": len(refused_ids), "union_over_token_types": len(union_refused_by_token), "ok": not any(n == "REFUSED_SET" for n, _ in check.failures)},
        "shape_totals": {"shapes": len(refusal_shapes), "ok": not any(n == "SHAPE_TOTALS" for n, _ in check.failures)},
        "partition": {"token_types": len(token_types), "ok": not any(n == "PARTITION" for n, _ in check.failures)},
    }
    ledger = {
        "schema": SCHEMA,
        "derived_by": "python3 scripts/token_coverage.py --check",
        "reading_rule": (
            "A token type is the converter mapping-table row key the converter resolved a token to "
            "(`src/pcgen_import/sheet_rule/table.rs::row_for_head`), `unmapped:<HEAD>` / `BONUS:<SUB>` "
            "when the table has no row, or `token-less` for a record whose closure has no row at all. "
            "`carrying` counts a unit once per token type it carries (a unit carrying k types is under k rows); "
            "`refused_because_of_this_token` counts a refused record under the token type each of its refusal "
            "shapes arose under; `refused_non_done` counts carrying non-DONE units the converter refused for ANY "
            "shape, so carrying_non_done = converted_non_done + refused_non_done per row. DONE is "
            "`scripts/completion_atlas.py`'s partition over docs/work-inventory.json."
        ),
        "inputs": {
            "inventory_generated_at": inventory.get("generated_at"),
            "converter_version": report.get("converter_version"),
            "oracle_pin": report.get("oracle_pin"),
            "mapping_table": table.get("schema"),
        },
        "denominators": {
            "units": len(units),
            "non_done": len(non_done),
            "records_in_census": len(entries),
            "refused_records": len(refused_ids),
            "refused_non_done": len(refused_non_done),
            "token_less_non_done": len(token_less_non_done),
            "token_types": len(token_types),
            "refusal_shapes": len(refusal_shapes),
        },
        "unmapped_token_types": unmapped,
        "token_types": token_types,
        "refusal_shapes": refusal_shapes,
        "sum_checks": sums,
        "verdict": check.verdict,
    }
    return ledger, check


def _render(ledger):
    return (json.dumps(ledger, indent=1, ensure_ascii=False, sort_keys=False) + "\n").encode("utf-8")


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--check", action="store_true", help="also fail when the committed ledger is stale")
    parser.add_argument("--inventory", default=INVENTORY_PATH)
    parser.add_argument("--package", default=PACKAGE_DIR)
    parser.add_argument("--table", default=TABLE_PATH)
    parser.add_argument("--out", default=OUT_PATH)
    args = parser.parse_args(argv)

    try:
        inventory = _load_json(args.inventory, "inventory")
        census = _load_json(os.path.join(args.package, "_tokens.json"), "token census")
        refused = _load_json(os.path.join(args.package, "_refused.json"), "refusal report")
        report = _load_json(os.path.join(args.package, "_report.json"), "converter report")
        table = _load_json(args.table, "mapping table")
    except InputError as exc:
        print(f"INPUT_ERROR: {exc}")
        print("verdict=INPUT_ERROR")
        return 2

    ledger, check = derive(inventory, census, refused, report, table)
    fresh = _render(ledger)

    stale = False
    if args.check:
        try:
            with open(args.out, "rb") as fh:
                stale = fh.read() != fresh
        except FileNotFoundError:
            stale = True
    os.makedirs(os.path.dirname(os.path.abspath(args.out)), exist_ok=True)
    with open(args.out, "wb") as fh:
        fh.write(fresh)

    d = ledger["denominators"]
    print(f"units={d['units']} non_done={d['non_done']} census_records={d['records_in_census']} token_types={d['token_types']} unmapped_token_types={len(ledger['unmapped_token_types'])}")
    print(f"refused={d['refused_records']} refused_non_done={d['refused_non_done']} shapes={d['refusal_shapes']} token_less_non_done={d['token_less_non_done']}")
    for name, s in ledger["sum_checks"].items():
        print(f"check {name}: {' '.join(f'{k}={v}' for k, v in s.items() if k != 'ok')} ok={s['ok']}")
    for name, detail in check.failures:
        print(f"FAIL {name}: {detail}")
    if args.check and stale and not check.failures:
        print(f"FAIL STALE_ARTIFACT: {args.out} differed from the fresh ledger (or was absent); rewritten -- commit it")
    verdict = check.verdict
    if verdict == "PASS" and args.check and stale:
        verdict = "FAIL_STALE_ARTIFACT"
    c = ledger["sum_checks"]["coverage"]
    print(f"non_done={d['non_done']} tokened={c['tokened']} token_less={c['token_less']} refused={d['refused_records']} refused_non_done={d['refused_non_done']} token_types={d['token_types']} shapes={d['refusal_shapes']} verdict={verdict}")
    return 0 if verdict == "PASS" else 1


if __name__ == "__main__":
    sys.exit(main())
