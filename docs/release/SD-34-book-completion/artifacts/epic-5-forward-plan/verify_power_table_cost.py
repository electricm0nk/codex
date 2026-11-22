"""Structural GREEN check for power-table-cost.json (AT-34-E5-003).

Fails closed if:
  - the recorded population disagrees with a live re-derivation
  - the recorded population disagrees with capability-register.json's own power_engine_table row
  - `power`'s directory-match claim does not hold against the live corpus tree
  - the projected rate is drawn from the WRONG comparator tier (trait's mismatched-directory,
    dearer tier, must never be the source of the range -- it must come from the matched-directory
    kinds only)
  - the "book still needs after power" section claims the book would be fully closed by power
    alone (bucket A is the ONLY bucket power can close; if other buckets are open, the closure
    claim itself is the acceptance bar's most load-bearing sentence)

Run from anywhere inside the repo:
    python3 docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/verify_power_table_cost.py
"""
import json, subprocess, sys, os

REPO = subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True, text=True, check=True).stdout.strip()
sys.path.insert(0, os.path.join(REPO, "scripts"))
import completion_atlas as ca

PATH = os.path.join(REPO, "docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/power-table-cost.json")
CAP_PATH = os.path.join(REPO, "docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/capability-register.json")
RATE_PATH = os.path.join(REPO, "docs/release/SD-34-book-completion/artifacts/epic-2-tables/table-build-rate.json")

d = json.load(open(PATH))
cap = json.load(open(CAP_PATH))
rate = json.load(open(RATE_PATH))

violations = []

inv = json.load(open(f"{REPO}/docs/work-inventory.json"))
units = inv["units"]

pop = d["population"]
live = len([
    u for u in units
    if u.get("status") == "engine-does-not-hold" and "has_no_engine_table" in (u.get("evidence") or "") and u.get("kind") == "power"
])
if pop["count"] != live:
    violations.append(f"population.count={pop['count']} but live re-derivation says {live}")

cap_row = next((c for c in cap["capabilities"] if c["id"] == "power_engine_table"), None)
if cap_row is None:
    violations.append("capability-register.json no longer carries a power_engine_table row")
elif cap_row["population"] != pop["count"]:
    violations.append(f"population.count={pop['count']} disagrees with capability-register.json's power_engine_table population={cap_row['population']}")

power_dir = os.path.join(REPO, "data/corpus", pop["book"], "power")
if not os.path.isdir(power_dir):
    violations.append(f"power's directory-match claim is false: {power_dir} does not exist")
else:
    file_count = len([f for f in os.listdir(power_dir) if f.endswith(".json")])
    if file_count != pop["count"]:
        violations.append(f"directory file count {file_count} disagrees with population.count={pop['count']}")

rd = d["rate_derivation"]
tables_by_kind = {t["kind"]: t for t in rate["tables"]}
matched = rd["matched_directory_kinds"]
if "trait" in matched:
    violations.append("trait (mismatched directory, dearer tier) must never be a matched-directory comparator")
if "companion" in matched:
    violations.append("companion (pre-existing table) must never be a matched-directory comparator")
lo, hi = rd["projected_marginal_lines_range"]
matched_lines = [tables_by_kind[k]["marginal_lines_measured"] for k in matched]
if lo != min(matched_lines) or hi != max(matched_lines):
    violations.append(f"marginal_lines_range {[lo, hi]} does not match the matched-kinds spread {matched_lines}")
wlo, whi = rd["projected_wall_time_seconds_range"]
matched_seconds = [tables_by_kind[k]["est_wall_time_seconds"] for k in matched]
if wlo != min(matched_seconds) or whi != max(matched_seconds):
    violations.append(f"wall_time_range {[wlo, whi]} does not match the matched-kinds spread {matched_seconds}")

pres_book = ca.partition(units, book=pop["book"])
live_book_counts = dict(pres_book["counts"])
book = d["book_still_needs_after_power_exists"]
if book["live_bucket_counts"] != live_book_counts:
    violations.append(f"book_still_needs_after_power_exists.live_bucket_counts is stale: recorded {book['live_bucket_counts']} vs live {live_book_counts}")

after = set(book["occupied_non_done_buckets_after_power"])
if "A" in after:
    violations.append("bucket A must not appear in 'after power' -- power's own build clears exactly bucket A")
if not after:
    violations.append("the acceptance bar requires power NOT to close the book alone -- 'after' must be non-empty (this book has other non-DONE buckets)")
before = set(book["occupied_non_done_buckets_before"])
if after != before - {"A"}:
    violations.append(f"'after' {sorted(after)} must equal 'before' {sorted(before)} minus bucket A exactly")

if violations:
    print(f"FAIL: {len(violations)} violation(s)")
    for v in violations:
        print(" -", v)
    sys.exit(1)

print(
    f"PASS: power population={pop['count']} (4 cross-checks agree), "
    f"directory-match confirmed, rate drawn from matched-directory tier only "
    f"({len(matched)} kinds, trait/companion excluded), "
    f"book still needs {sorted(after)} after power exists (table alone does not close it)"
)
