#!/usr/bin/env python3
"""Assert the frozen PF1e public status snapshot never silently drifts.

D3/D5 (operator ruling, 2026-09-15; docs/release/SD-36-consolidation/):
the PF1e public status feed (site/status-data.json + site/status-data/
<book>.json) is now a FROZEN snapshot, not a live-regenerated one — the
producers that used to feed it (`v06_work_inventory`, the
`pf1e_dashboard_producer.py` cron regen) are retired. This is the gate that
catches the snapshot moving out from under that freeze: a hand-edit, a
stray regen, a partial write, or a stale per-book detail file all fail it
loudly. It never regenerates anything — it only reads the two committed
artifacts.

Checks:
  1. site/status-data.json exists and parses.
  2. overall.pct == 100.0, overall.denominator == FROZEN_DENOMINATOR (the
     docs/work-inventory.json total unit count at freeze time,
     docs/work-inventory.FROZEN.md), overall.not_started == 0,
     overall.partial == 0, overall.done == overall.denominator.
  3. generated_at equals FROZEN_GENERATED_AT — the exact stamp the one-time
     D5 regen wrote. Any other value means the file moved after the freeze.
  4. Every book listed in overview["books"] has a matching, existing
     site/status-data/<id>.json file whose own per-kind done/partial/
     not_started/denominator sums reconcile with that book's rollup entry.
  5. The per-book denominators (and done counts) in overview["books"] sum to
     overall.denominator (and overall.done) — every unit in the frozen
     headline has a book row behind it; a book silently dropped from the
     public grid while still counted in the 100% headline (SD-36 Epic B
     round-5 finding: 7 real books, 3,365 units) fails this loudly instead
     of only showing up as a quiet grid gap.

Usage:
    python3 scripts/site/check_frozen_status.py
Exit 0 on a still-frozen 100% snapshot; exit 1, with every violation
printed, otherwise.
"""
import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
STATUS_DATA = REPO_ROOT / "site" / "status-data.json"
BOOK_DETAIL_DIR = REPO_ROOT / "site" / "status-data"

# docs/work-inventory.json totals.units at freeze time (docs/work-inventory.FROZEN.md).
FROZEN_DENOMINATOR = 49450
# The exact `generated_at` the one-time D5 regen wrote. Changing this on
# purpose is how a deliberate, reviewed re-freeze is recorded; any other
# drift is a bug.
#
# 2026-09-17 SD-36 Epic B round-5: re-frozen once more (deliberately,
# reviewed) to add 7 real content books (mythic_adventures,
# adventurers_guide, inner_sea_magic, inner_sea_faiths, inner_sea_temples,
# inner_sea_taverns, beginner_box; 3,365 units) that were counted in the
# 100% headline but had no BOOK_TITLES entry and so no public grid row.
# FROZEN_DENOMINATOR is unchanged (100.0% of the same 49,450 units) --
# only the book breakdown changed, from 30 books to 37.
FROZEN_GENERATED_AT = "2026-09-17T07:32:59Z"


def check(status_data_path: Path = STATUS_DATA, book_dir: Path = BOOK_DETAIL_DIR) -> list[str]:
    """Returns a list of violation strings; empty means frozen and green."""
    if not status_data_path.exists():
        return [f"{status_data_path} does not exist"]
    overview = json.loads(status_data_path.read_text())
    violations = []

    overall = overview.get("overall", {})
    if overall.get("pct") != 100.0:
        violations.append(f"overall.pct is {overall.get('pct')!r}, want 100.0")
    if overall.get("denominator") != FROZEN_DENOMINATOR:
        violations.append(f"overall.denominator is {overall.get('denominator')!r}, want {FROZEN_DENOMINATOR}")
    if overall.get("not_started") != 0:
        violations.append(f"overall.not_started is {overall.get('not_started')!r}, want 0")
    if overall.get("partial") != 0:
        violations.append(f"overall.partial is {overall.get('partial')!r}, want 0")
    if overall.get("done") != overall.get("denominator"):
        violations.append(
            f"overall.done ({overall.get('done')!r}) != overall.denominator ({overall.get('denominator')!r})"
        )

    if overview.get("generated_at") != FROZEN_GENERATED_AT:
        violations.append(
            f"generated_at is {overview.get('generated_at')!r}, want the frozen stamp "
            f"{FROZEN_GENERATED_AT!r} — the snapshot moved after the freeze"
        )

    for book in overview.get("books", []):
        book_id = book["id"]
        detail_path = book_dir / f"{book_id}.json"
        if not detail_path.exists():
            violations.append(f"book {book_id!r} is listed in {status_data_path} but {detail_path} is missing")
            continue
        detail = json.loads(detail_path.read_text())
        kinds = detail.get("kinds", [])
        sums = {
            key: sum(k.get(key, 0) for k in kinds)
            for key in ("done", "partial", "not_started", "denominator")
        }
        for key, total in sums.items():
            if total != book.get(key):
                violations.append(
                    f"book {book_id!r}: sum of kind {key} ({total}) != book {key} ({book.get(key)!r})"
                )

    books = overview.get("books", [])
    for key in ("denominator", "done"):
        book_total = sum(b.get(key, 0) for b in books)
        overall_total = overall.get(key)
        if book_total != overall_total:
            violations.append(
                f"sum of book {key} across all {len(books)} listed books ({book_total}) != "
                f"overall.{key} ({overall_total!r}) — a book counted in the frozen headline "
                "has no row on the public grid"
            )

    return violations


def main():
    violations = check()
    if violations:
        for v in violations:
            print(f"FROZEN STATUS DRIFT: {v}", file=sys.stderr)
        sys.exit(1)
    print(f"OK: {STATUS_DATA} is frozen at 100% ({FROZEN_DENOMINATOR} units)")


if __name__ == "__main__":
    main()
