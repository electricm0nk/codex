"""RED/GREEN check for artifacts/epic-5-forward-plan/forward-plan.json (AT-34-E5-001).

Fails closed (nonzero exit) if:
  1. the artifact is missing
  2. it does not cover exactly 35 non-vehicle books
  3. any book's bucket unit-counts disagree with a live completion_atlas.py partition
  4. any non-zero bucket row is missing a mechanism string, a sample_size field, and
     either a rate object or an explicit UNMEASURED note (never silently absent -- the
     acceptance bar: "a forward-plan projection with no stated rate or sample size" fails)
  5. core_rulebook or ultimate_campaign (the two vehicle books) appear in the plan
"""
import json, subprocess, sys, os

REPO = subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True, text=True, check=True).stdout.strip()
sys.path.insert(0, os.path.join(REPO, "scripts"))
import completion_atlas as ca

PLAN_PATH = os.path.join(REPO, "docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/forward-plan.json")

if not os.path.exists(PLAN_PATH):
    print(f"FAIL: {PLAN_PATH} does not exist")
    sys.exit(1)

plan = json.load(open(PLAN_PATH))
inv = json.load(open(os.path.join(REPO, "docs/work-inventory.json")))
units = inv["units"]

errors = []

books = plan["books"]
if len(books) != 35:
    errors.append(f"expected 35 books, got {len(books)}")

names = {b["book"] for b in books}
for vehicle in ("core_rulebook", "ultimate_campaign"):
    if vehicle in names:
        errors.append(f"vehicle book {vehicle!r} must not appear in the 35-book forward plan")

for b in books:
    live = ca.partition(units, book=b["book"])
    if live["unclassified_ids"] or live["overlap_ids"]:
        errors.append(f"{b['book']}: live partition has unclassified/overlap units")
    for bucket, n in live["counts"].items():
        if bucket == "DONE":
            continue
        plan_n = b["buckets"].get(bucket, {}).get("unit_count", 0)
        if plan_n != n:
            errors.append(f"{b['book']} bucket {bucket}: plan says {plan_n}, live atlas says {n}")
    for bucket, row in b["buckets"].items():
        if not row.get("mechanism"):
            errors.append(f"{b['book']} bucket {bucket}: missing mechanism")
        if "sample_size" not in row:
            errors.append(f"{b['book']} bucket {bucket}: missing sample_size field")
        has_rate = row.get("rate") is not None
        has_note = bool(row.get("note"))
        if not has_rate and not has_note:
            errors.append(f"{b['book']} bucket {bucket}: no rate AND no explanatory note -- silent absence, fails the acceptance bar")

if errors:
    print(f"FAIL: {len(errors)} violation(s)")
    for e in errors[:40]:
        print(" -", e)
    sys.exit(1)

print(f"PASS: {len(books)} books, all bucket counts match live completion_atlas.py, every row carries a rate-or-note")
sys.exit(0)
