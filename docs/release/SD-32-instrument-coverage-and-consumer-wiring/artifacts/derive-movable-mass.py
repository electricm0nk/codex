#!/usr/bin/env python3
"""Re-derive every movable-mass figure this bundle cites, from first sources.

Sources (both read-only):
  * `docs/work-inventory.json` — the generator's output (`cargo run --bin v06_work_inventory`).
  * The doneness verdict table, transcribed from `doneness_verdict()` /
    `_doneness_verdict_uncapped()` in the dashboard producer
    (`~/.hermes/profiles/god-emporer/skills/release-swarm-observer/scripts/`
    `pf1e_dashboard_producer.py`).

The transcription is validated, not trusted: `validate()` recomputes
`by_doneness` and asserts it equals the live dashboard payload's
`work_inventory.by_doneness`. If the producer's table ever changes, this
script fails loudly rather than reporting a stale split.

Run:  python3 docs/release/SD-32-instrument-coverage-and-consumer-wiring/artifacts/derive-movable-mass.py
"""
import json
import collections
import os
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", ".."))
INVENTORY = os.path.join(REPO, "docs", "work-inventory.json")
DASHBOARD = "/home/ubuntu/swarm-observer/PF1e-dashboard.json"

# Both transcribed from the producer; see module docstring.
EXCLUDED_BOOKS = {"beginner_box"}
NO_GROUNDING_PROBE = ("companion", "spell")

# `probe_equipment_effect_wiring()` in src/bin/v06_work_inventory.rs builds its
# key universe from exactly four compiled equipment tables.
PROBE_KEY_BOOKS = {"core_rulebook", "advanced_players_guide",
                   "advanced_class_guide", "bestiary"}
# Books that HAVE a compiled equipment table under
# src/rules_core/rules_tables/<book>/equipment_tables.rs but whose keys the
# probe never loads. Derived by:
#   find src -path '*equipment_tables*' | sed 's|/[^/]*$||' | sort -u
COMPILED_EQUIPMENT_TABLE_BOOKS = PROBE_KEY_BOOKS | {
    "pathfinder_unchained", "advanced_race_guide", "ultimate_magic",
    "ultimate_combat", "ultimate_psionics", "ultimate_equipment",
    "ultimate_intrigue",
}


def uncapped(wiring_class, status):
    if status == "deferred-with-reason":
        return "deferred"
    if status in ("not-ingested", "not-started"):
        return "not-started"
    if status == "unknown":
        return "unmeasurable"
    if wiring_class == "ambiguous":
        if status in ("grounded", "text-complete", "ingested-magnitude"):
            return "held"
        raise ValueError((wiring_class, status))
    if wiring_class == "display":
        if status == "text-complete":
            return "done"
        if status == "grounded":
            return "held"
        return "in-progress"
    if wiring_class in ("static", "derived"):
        if status in ("ingested-magnitude", "grounded", "text-complete"):
            return "held"
        raise ValueError((wiring_class, status))
    if wiring_class == "computed":
        return "done" if status == "grounded" else "in-progress"
    raise ValueError(wiring_class)


def verdict(wiring_class, status, kind):
    v = uncapped(wiring_class, status)
    if v == "in-progress" and kind in NO_GROUNDING_PROBE:
        return "held"
    return v


def load_units():
    with open(INVENTORY) as fh:
        doc = json.load(fh)
    return [u for u in doc["units"]
            if (u.get("book") or "") not in EXCLUDED_BOOKS], doc


def validate(units, inventory_stamp):
    """Check the transcribed verdict table against the live dashboard.

    Two different things can make these disagree, and conflating them would
    make this guard useless:

      * DRIFT -- the inventory on disk was regenerated after the dashboard last
        rebuilt its doneness cache. The stamps differ, the table is fine, and
        the right response is to re-derive against the stamp the dashboard
        actually shows. Warn, do not fail.
      * TABLE CHANGE -- the stamps agree but the counts do not. The producer's
        `doneness_verdict()` table changed under this transcription and every
        figure derived from it is void. Fail hard.

    The distinction is not hypothetical: it fired for real on 2026-08-13 while
    this bundle was being scoped, when a concurrent agent regenerated
    `docs/work-inventory.json` mid-session.
    """
    mine = collections.Counter(
        verdict(u.get("wiring_class"), u["status"], u["kind"]) for u in units)
    try:
        with open(DASHBOARD) as fh:
            payload = json.load(fh)["work_inventory"]
    except OSError:
        print("WARN: dashboard payload unreadable; transcription NOT validated")
        return dict(mine), None
    live = payload["by_doneness"]
    dashboard_stamp = payload.get("doneness_source_generated_at")
    ok = all(mine.get(k, 0) == v for k, v in live.items())
    print(f"re-derived: {dict(mine)}")
    print(f"dashboard : {live}")
    print(f"inventory stamp on disk: {inventory_stamp}")
    print(f"dashboard doneness stamp: {dashboard_stamp}")
    if ok:
        print("transcription validated against live dashboard: True")
        return dict(mine), live
    if inventory_stamp != dashboard_stamp:
        print("DRIFT, not a table change: the inventory on disk is a different "
              "generation from the one the dashboard's doneness cache was built "
              "from. The verdict table is NOT implicated. Re-derive against the "
              "dashboard's stamp, or wait for the producer to catch up, before "
              "citing any figure below.")
        return dict(mine), live
    sys.exit("FAIL: same inventory generation, different counts -- the "
             "producer's verdict table changed under this transcription. "
             "Every figure below is void until the table is re-read.")


def bucket(u):
    """(bucket_id, why) for one held/in-progress unit."""
    kind, wc, st, book = u["kind"], u.get("wiring_class"), u["status"], u["book"]
    v = verdict(wc, st, kind)
    if v == "in-progress":
        if kind in ("equipment", "equipment_modifier"):
            if book in PROBE_KEY_BOOKS:
                return "A1", "probe examines it; no mechanical effect observed"
            if book in COMPILED_EQUIPMENT_TABLE_BOOKS:
                return "A2", "book has a compiled table the probe's key universe omits"
            return "A3", "no compiled equipment table for this book"
        return "A4", "other kind, existing probe"
    if kind == "spell":
        return ("C1" if wc == "computed" or (wc == "display" and st == "ingested-magnitude")
                else "C2"), "no consumer reads a spell magnitude (grounded==0 corpus-wide)"
    if wc in ("static", "derived"):
        return "B3", "needs the missing check AND a done rung the verdict table lacks"
    if wc == "ambiguous":
        return "B1", "needs the wiring-class classifier (class unresolved)"
    if wc == "display":
        return "B2", "needs the wiring-class classifier (grounded contradicts display)"
    return "??", f"{wc}/{st}"


def main():
    units, doc = load_units()
    print(f"work-inventory.json generated_at: {doc['generated_at']}")
    print(f"units after EXCLUDED_BOOKS: {len(units)}\n")
    validate(units, doc["generated_at"])
    print()

    movable = [u for u in units
               if verdict(u.get("wiring_class"), u["status"], u["kind"])
               in ("held", "in-progress")]
    print(f"=== movable mass (held + in-progress): {len(movable)} ===\n")

    by_bucket = collections.Counter()
    detail = collections.defaultdict(collections.Counter)
    for u in movable:
        b, _ = bucket(u)
        by_bucket[b] += 1
        detail[b][(u["kind"], u.get("wiring_class"), u["status"])] += 1

    for b in sorted(by_bucket, key=lambda x: -by_bucket[x]):
        print(f"{by_bucket[b]:6}  {b}  {bucket([u for u in movable if bucket(u)[0] == b][0])[1]}")
        for cell, n in sorted(detail[b].items(), key=lambda x: -x[1]):
            print(f"          {n:6}  {cell[0]}/{cell[1]}/{cell[2]}")
    print(f"{sum(by_bucket.values()):6}  TOTAL\n")

    print("=== movable mass by kind x doneness ===")
    c = collections.Counter((u["kind"], verdict(u.get("wiring_class"), u["status"], u["kind"]))
                            for u in movable)
    for cell, n in sorted(c.items(), key=lambda x: -x[1]):
        print(f"  {n:6}  {cell[0]:20} {cell[1]}")
    print()

    print("=== A1/A2 split by book (equipment + equipment_modifier in-progress) ===")
    c = collections.Counter((bucket(u)[0], u["kind"], u["book"]) for u in movable
                            if bucket(u)[0] in ("A1", "A2", "A3"))
    for cell, n in sorted(c.items(), key=lambda x: -x[1]):
        print(f"  {n:6}  {cell[0]}  {cell[1]:20} {cell[2]}")
    print()

    print("=== proof the probe reaches a kind: computed+grounded by kind ===")
    c = collections.Counter(u["kind"] for u in units
                            if u.get("wiring_class") == "computed" and u["status"] == "grounded")
    for k, n in sorted(c.items(), key=lambda x: -x[1]):
        print(f"  {n:6}  {k}")
    print()

    print("=== any grounded at all, by kind (NO_GROUNDING_PROBE audit) ===")
    c = collections.Counter(u["kind"] for u in units if u["status"] == "grounded")
    for k in sorted({u["kind"] for u in units}):
        print(f"  {c.get(k, 0):6}  {k}"
              f"{'   <-- listed in NO_GROUNDING_PROBE' if k in NO_GROUNDING_PROBE else ''}")
    print()

    print("=== units the NO_GROUNDING_PROBE cap actually moves (in-progress -> held) ===")
    c = collections.Counter(u["kind"] for u in units if u["kind"] in NO_GROUNDING_PROBE
                            and uncapped(u.get("wiring_class"), u["status"]) == "in-progress")
    for k in NO_GROUNDING_PROBE:
        print(f"  {c.get(k, 0):6}  {k}")
    print()

    print("=== static/derived held by kind (capped by the verdict table) ===")
    c = collections.Counter((u["kind"], u["wiring_class"]) for u in movable
                            if u.get("wiring_class") in ("static", "derived"))
    for cell, n in sorted(c.items(), key=lambda x: -x[1]):
        print(f"  {n:6}  {cell[0]}/{cell[1]}")
    print(f"  {sum(c.values()):6}  TOTAL static+derived held")
    print()

    print("=== classifier lever by kind (ambiguous, or display+grounded) ===")
    c = collections.Counter((u["kind"], u["wiring_class"], u["status"]) for u in movable
                            if u.get("wiring_class") == "ambiguous"
                            or (u.get("wiring_class") == "display" and u["status"] == "grounded"))
    for cell, n in sorted(c.items(), key=lambda x: -x[1]):
        print(f"  {n:6}  {cell[0]}/{cell[1]}/{cell[2]}")
    print(f"  {sum(c.values()):6}  TOTAL")
    print()

    print("=== unmeasurable (status==unknown), adjacent lever, NOT in this bundle's mass ===")
    c = collections.Counter(u["kind"] for u in units if u["status"] == "unknown")
    for k, n in sorted(c.items(), key=lambda x: -x[1]):
        print(f"  {n:6}  {k}")


if __name__ == "__main__":
    main()
