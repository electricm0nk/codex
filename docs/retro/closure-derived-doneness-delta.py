#!/usr/bin/env python3
"""Recompute the dashboard's `done` count at arbitrary git revisions.

Answers a question no existing tool in this repo answers: *did a commit move
`done`, and which kinds moved?* `docs/release/SD-32-instrument-coverage-and-
consumer-wiring/artifacts/derive-movable-mass.py` transcribes the same verdict
table but only ever reads the working-tree inventory, so it cannot attribute a
movement to a commit.

It exists because of what it found: the `%N` wiring_class fix (99efb504,
inventory regenerated at 8d00d0b1) is a CORRECT fix that cost the dashboard 26
`done` units, because `display + text-complete -> done` while
`derived + text-complete -> held`. Reclassifying a unit into `static` or
`derived` can only ever subtract, since neither class has a `done` rung.

    python3 docs/retro/closure-derived-doneness-delta.py 5ed6bdc0 8d00d0b1

The verdict table below is transcribed from `_doneness_verdict_uncapped()` /
`doneness_verdict()` in the dashboard producer. It is NOT trusted blind: the
sibling artifact `derive-movable-mass.py` asserts the same transcription
against the live dashboard payload (`transcription validated against live
dashboard: True`), and running this script at 90bd9975 independently
reproduces that commit's known +46 spell movement.
"""
import collections
import json
import subprocess
import sys

NO_GROUNDING_PROBE = ("companion", "spell")
EXCLUDED_BOOKS = {"beginner_box"}
EVIDENCE_STATUSES = ("ingested-magnitude", "grounded", "text-complete")


def uncapped(wiring_class, status):
    if status == "deferred-with-reason":
        return "deferred"
    if status in ("not-ingested", "not-started"):
        return "not-started"
    if status == "unknown":
        return "unmeasurable"
    if wiring_class == "ambiguous":
        if status in EVIDENCE_STATUSES:
            return "held"
        raise ValueError((wiring_class, status))
    if wiring_class == "display":
        if status == "text-complete":
            return "done"
        return "held" if status == "grounded" else "in-progress"
    if wiring_class in ("static", "derived"):
        # No `done` rung exists for these two classes, at any status.
        if status in EVIDENCE_STATUSES:
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


def at(rev):
    blob = subprocess.run(
        ["git", "show", f"{rev}:docs/work-inventory.json"],
        capture_output=True, text=True, check=True).stdout
    units = [u for u in json.loads(blob)["units"]
             if u["book"] not in EXCLUDED_BOOKS]
    done = collections.Counter(
        u["kind"] for u in units
        if verdict(u["wiring_class"], u["status"], u["kind"]) == "done")
    classes = collections.Counter(u["wiring_class"] for u in units)
    return units, done, classes


def main(revs):
    if not revs:
        sys.exit(f"usage: {sys.argv[0]} <rev> [<rev> ...]")
    previous = None
    for rev in revs:
        units, done, classes = at(rev)
        subject = subprocess.run(["git", "log", "-1", "--format=%s", rev],
                                 capture_output=True, text=True).stdout.strip()
        total = sum(done.values())
        print(f"--- {rev}  {subject[:70]}")
        print(f"    units={len(units)}  DONE={total}")
        print(f"    wiring_class: {dict(classes)}")
        print(f"    done by kind: {dict(done.most_common())}")
        if previous is not None:
            prev_rev, prev_done = previous
            delta = total - sum(prev_done.values())
            moved = {k: done[k] - prev_done[k]
                     for k in set(done) | set(prev_done)
                     if done[k] != prev_done[k]}
            print(f"    DELTA vs {prev_rev}: {delta:+d} done  {moved or '(no kind moved)'}")
        previous = (rev, done)
        print()


if __name__ == "__main__":
    main(sys.argv[1:])
