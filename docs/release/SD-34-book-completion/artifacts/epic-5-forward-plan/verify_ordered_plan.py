"""RED/GREEN check for artifacts/epic-5-forward-plan/ordered-plan.json (AT-34-E5-004).

Fails closed (nonzero exit) if:
  1. the artifact is missing
  2. `ranked_by_priced_to_done_cost` + `unrankable_zero_priced_to_done_units` do not together
     cover exactly the 35 books in forward-plan.json (no book missing, none duplicated,
     no vehicle book present)
  3. `ranked_by_priced_to_done_cost` is not sorted ascending by its own stated midpoint hours
  4. any ranked row's midpoint disagrees with (low+high)/2 computed from forward-plan.json's
     own bucket-level `projected_cost_hours` fields for that book's A/B/U buckets
  5. `single_bucket_books` does not exactly match the set of books in forward-plan.json whose
     `buckets` dict has exactly one key (by name, both directions)
  6. any single-bucket book is missing from `single_bucket_books`, or any non-single-bucket
     book is wrongly flagged
  7. the stated `ordering_basis` string is empty
"""
import json, subprocess, sys, os

REPO = subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True, text=True, check=True).stdout.strip()
EPIC_DIR = os.path.join(REPO, "docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan")
PLAN_PATH = os.path.join(EPIC_DIR, "forward-plan.json")
ORDERED_PATH = os.path.join(EPIC_DIR, "ordered-plan.json")

if not os.path.exists(ORDERED_PATH):
    print(f"FAIL: {ORDERED_PATH} does not exist")
    sys.exit(1)

fwd = json.load(open(PLAN_PATH))
ordered = json.load(open(ORDERED_PATH))

errors = []

fwd_books_by_name = {b["book"]: b for b in fwd["books"]}
fwd_names = set(fwd_books_by_name)

ranked = ordered.get("ranked_by_priced_to_done_cost", [])
unrankable = ordered.get("unrankable_zero_priced_to_done_units", [])

ranked_names = [r["book"] for r in ranked]
unrankable_names = [r["book"] for r in unrankable]
all_ordered_names = ranked_names + unrankable_names

if len(all_ordered_names) != len(fwd_names):
    errors.append(f"expected {len(fwd_names)} total books across both lists, got {len(all_ordered_names)}")
if len(set(all_ordered_names)) != len(all_ordered_names):
    errors.append("duplicate book(s) across ranked + unrankable lists")
missing = fwd_names - set(all_ordered_names)
extra = set(all_ordered_names) - fwd_names
if missing:
    errors.append(f"missing book(s): {sorted(missing)}")
if extra:
    errors.append(f"unexpected book(s) not in forward-plan.json: {sorted(extra)}")
for vehicle in ("core_rulebook", "ultimate_campaign"):
    if vehicle in all_ordered_names:
        errors.append(f"vehicle book {vehicle!r} must not appear in the ordered plan")

# Recompute priced-to-DONE low/high/midpoint independently from forward-plan.json and cross-check.
def priced_range(book_row):
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

prev_mid = None
for r in ranked:
    name = r["book"]
    fwd_row = fwd_books_by_name.get(name)
    if fwd_row is None:
        continue  # already flagged above
    exp_units, exp_lo, exp_hi = priced_range(fwd_row)
    exp_mid = (exp_lo + exp_hi) / 2
    if exp_units == 0:
        errors.append(f"{name}: ranked list must only contain books with priced_to_done_units > 0, got 0")
        continue
    if r.get("priced_to_done_units") != exp_units:
        errors.append(f"{name}: priced_to_done_units={r.get('priced_to_done_units')} but live re-derivation says {exp_units}")
    if abs(r.get("priced_to_done_hours_low", -1) - exp_lo) > 1e-6:
        errors.append(f"{name}: priced_to_done_hours_low={r.get('priced_to_done_hours_low')} but live re-derivation says {exp_lo}")
    if abs(r.get("priced_to_done_hours_high", -1) - exp_hi) > 1e-6:
        errors.append(f"{name}: priced_to_done_hours_high={r.get('priced_to_done_hours_high')} but live re-derivation says {exp_hi}")
    if abs(r.get("priced_to_done_hours_midpoint", -1) - exp_mid) > 1e-6:
        errors.append(f"{name}: priced_to_done_hours_midpoint={r.get('priced_to_done_hours_midpoint')} but live re-derivation says {exp_mid}")
    mid = r.get("priced_to_done_hours_midpoint")
    if prev_mid is not None and mid is not None and mid < prev_mid - 1e-9:
        errors.append(f"{name}: not sorted ascending by priced_to_done_hours_midpoint ({mid} follows {prev_mid})")
    prev_mid = mid

for r in unrankable:
    name = r["book"]
    fwd_row = fwd_books_by_name.get(name)
    if fwd_row is None:
        continue
    exp_units, _, _ = priced_range(fwd_row)
    if exp_units != 0:
        errors.append(f"{name}: listed as unrankable (zero priced-to-DONE units) but live re-derivation says {exp_units} priced units")

# single_bucket_books cross-check
live_single_bucket = {name for name, row in fwd_books_by_name.items() if len(row["buckets"]) == 1}
stated_single_bucket = {row["book"] for row in ordered.get("single_bucket_books", [])}
if stated_single_bucket != live_single_bucket:
    errors.append(f"single_bucket_books mismatch: stated={sorted(stated_single_bucket)} live={sorted(live_single_bucket)}")

if not ordered.get("ordering_basis"):
    errors.append("ordering_basis is empty or missing")

if errors:
    print(f"FAIL: {len(errors)} violation(s)")
    for e in errors[:40]:
        print(" -", e)
    sys.exit(1)

print(f"PASS: {len(ranked)} ranked + {len(unrankable)} unrankable = {len(all_ordered_names)} books, "
      f"sorted ascending by priced_to_done_hours_midpoint, {len(stated_single_bucket)} single-bucket book(s) flagged and confirmed live")
sys.exit(0)
