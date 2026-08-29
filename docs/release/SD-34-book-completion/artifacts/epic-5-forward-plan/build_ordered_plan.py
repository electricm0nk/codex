"""Build artifacts/epic-5-forward-plan/ordered-plan.json (AT-34-E5-004).

Re-derives, every run, from forward-plan.json (AT-34-E5-001) -- never hand-edited.

Ordering basis (stated here, and echoed into the artifact's own `ordering_basis` field):

  Only buckets A, B and U carry a measured rate that reaches DONE (bucket C's only measured
  rate reaches V, a different endpoint -- see forward-plan.json's own C-bucket note; buckets
  D, M, V, X, Z carry no rate at all -- zero dedicated clearing cycles ran in either vehicle
  book). "Real cost, cheapest-first" can therefore only be computed over each book's
  priced-to-DONE slice (its A+B+U units), never over a book's full remaining population --
  doing the latter would silently blend priced and unpriced units into one fabricated number,
  the exact failure `decisions.md` and `AGENTS.md` rule 9 warn against.

  Books are ranked ascending by the MIDPOINT of their priced_to_done_hours range (bucket B
  carries a measured range, not a point estimate -- see forward-plan.json measured_rates.B).
  Every ranked row also states what fraction of that book's remaining population the priced
  slice covers, so "cheapest first" is never read as "book finishes soonest" for a book that
  is mostly unpriced.

  Two books (whichever carry zero A/B/U units) have no priced-to-DONE cost at all and cannot
  be placed on a real-cost ranking; they are listed separately, alphabetically, each naming
  which unpriced buckets make up its entire remaining population.

  Single-bucket books -- the genuine low-hanging fruit (ultimate_campaign's own shape, per
  Epic 4) -- are flagged by name regardless of which list they fall into, since a single
  remaining bucket is a book-shape property, not a pricing property.
"""
import json, subprocess, sys, os

REPO = subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True, text=True, check=True).stdout.strip()
HEAD_SHA = subprocess.run(["git", "rev-parse", "HEAD"], capture_output=True, text=True, check=True).stdout.strip()
EPIC_DIR = os.path.join(REPO, "docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan")
PLAN_PATH = os.path.join(EPIC_DIR, "forward-plan.json")
ORDERED_PATH = os.path.join(EPIC_DIR, "ordered-plan.json")

ORDERING_BASIS = (
    "Ascending by the midpoint of each book's priced-to-DONE projected-cost range, where "
    "priced-to-DONE = the sum of its bucket A + bucket B + bucket U projected_cost_hours "
    "(the only three buckets with a measured rate that reaches DONE; bucket C's only measured "
    "rate reaches V, not DONE, and buckets D/M/V/X/Z carry no rate at all -- see "
    "forward-plan.json's measured_rates). This ranks the PRICED SLICE of each book's remaining "
    "work, never the whole remaining population; every row states the priced slice's own "
    "fraction of that book's total remaining units so a low rank is never misread as 'this book "
    "finishes soonest'. Books with zero priced-to-DONE units have no real cost to sort by and "
    "are listed separately, alphabetically, not interleaved into the ranking."
)


def priced_to_done(book_row):
    lo = hi = 0.0
    units = 0
    for k in ("A", "B", "U"):
        row = book_row["buckets"].get(k)
        if not row:
            continue
        cost = row.get("projected_cost_hours")
        if cost is None:
            continue
        units += row["unit_count"]
        if isinstance(cost, list):
            lo += cost[0]
            hi += cost[1]
        else:
            lo += cost
            hi += cost
    return units, lo, hi


def main():
    fwd = json.load(open(PLAN_PATH))
    books = fwd["books"]

    ranked = []
    unrankable = []
    single_bucket_books = []

    for b in books:
        name = b["book"]
        remaining = b["remaining_non_done"]
        buckets_present = sorted(b["buckets"].keys())
        is_single_bucket = len(buckets_present) == 1

        if is_single_bucket:
            only_bucket = buckets_present[0]
            row = b["buckets"][only_bucket]
            single_bucket_books.append({
                "book": name,
                "bucket": only_bucket,
                "unit_count": row["unit_count"],
                "mechanism": row["mechanism"],
                "priced": row.get("rate") is not None or row.get("projected_cost_hours") is not None,
            })

        units, lo, hi = priced_to_done(b)
        unpriced_units = remaining - units
        if units == 0:
            unrankable.append({
                "book": name,
                "remaining_non_done": remaining,
                "buckets_present": buckets_present,
                "single_bucket": is_single_bucket,
                "reason": (
                    "zero units in buckets A/B/U (the only rate-to-DONE buckets); every "
                    f"remaining unit is in {buckets_present} which carry no measured "
                    "rate reaching DONE"
                ),
            })
        else:
            ranked.append({
                "book": name,
                "remaining_non_done": remaining,
                "priced_to_done_units": units,
                "priced_to_done_units_pct_of_remaining": round(100.0 * units / remaining, 1),
                "unpriced_units": unpriced_units,
                "unpriced_units_pct_of_remaining": round(100.0 * unpriced_units / remaining, 1),
                "priced_to_done_hours_low": round(lo, 3),
                "priced_to_done_hours_high": round(hi, 3),
                "priced_to_done_hours_midpoint": round((lo + hi) / 2, 3),
                "single_bucket": is_single_bucket,
                "buckets_present": buckets_present,
            })

    ranked.sort(key=lambda r: r["priced_to_done_hours_midpoint"])
    for i, r in enumerate(ranked, start=1):
        r["rank"] = i
    unrankable.sort(key=lambda r: r["book"])
    single_bucket_books.sort(key=lambda r: r["book"])

    out = {
        "criterion": "AT-34-E5-004",
        "re_derive_command": "python3 docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/build_ordered_plan.py",
        "generated_at_head": HEAD_SHA,
        "source_artifact": "docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/forward-plan.json",
        "population": {
            "books_count": len(books),
            "books_with_priced_to_done_units": len(ranked),
            "books_with_zero_priced_to_done_units": len(unrankable),
        },
        "ordering_basis": ORDERING_BASIS,
        "ranked_by_priced_to_done_cost": ranked,
        "unrankable_zero_priced_to_done_units": unrankable,
        "single_bucket_books": single_bucket_books,
        "notes": (
            "single_bucket_books is a book-SHAPE property (exactly one non-DONE bucket occupies "
            "the book's entire remaining population), independent of whether that bucket is "
            "priced. It intentionally may include a book that also appears in "
            "unrankable_zero_priced_to_done_units, if that book's sole remaining bucket has no "
            "measured rate (a single-bucket book is still the cheapest possible SHAPE to close "
            "-- one mechanism clears the whole book -- even before a rate exists to price it, "
            "which is exactly how ultimate_campaign (Epic 4's vehicle book) was found)."
        ),
    }

    with open(ORDERED_PATH, "w") as f:
        json.dump(out, f, indent=2)
        f.write("\n")

    print(f"wrote {ORDERED_PATH}: {len(ranked)} ranked, {len(unrankable)} unrankable, "
          f"{len(single_bucket_books)} single-bucket book(s)")


if __name__ == "__main__":
    main()
