"""E1.9 field-diff check: the re-convert moved only allowed fields. Usage, from the repo root:
python3 -I docs/release/SD-41-gear-and-equipment-sets/artifacts/epic_1/E1.9/field_diff_check.py [BASE]
BASE defaults to HEAD (the working tree against it); with a commit range use the card base and run on its result.
Prints FIELDS_OK, or every violation and exits 1."""
import json, subprocess, sys

base = sys.argv[1] if len(sys.argv) > 1 else "HEAD"
names = subprocess.run(["git", "diff", "--name-only", base, "--", "data"], capture_output=True, text=True, check=True).stdout.split()
bad, counts = [], {"equipment_records": 0, "settled_bundles": 0, "settled_records": 0, "tables": 0, "table_rows": 0, "overrides": 0}

def old(path):
    r = subprocess.run(["git", "show", f"{base}:{path}"], capture_output=True, text=True)
    return json.loads(r.stdout) if r.returncode == 0 else None

def diff_keys(a, b):
    return {k for k in set(a) | set(b) if a.get(k) != b.get(k)}

for path in names:
    new = json.load(open(path))
    before = old(path)
    if path.startswith("data/price_overrides/"):
        counts["overrides"] += 1
        continue
    if before is None:
        bad.append(f"{path}: new file outside price_overrides")
        continue
    if "/_settled/equipment.json" in path:
        counts["settled_bundles"] += 1
        if diff_keys(before, new) - {"records"}:
            bad.append(f"{path}: envelope moved")
        b, n = before["records"], new["records"]
        if set(b) != set(n):
            bad.append(f"{path}: record population moved")
        for k in set(b) & set(n):
            if b[k] != n[k]:
                counts["settled_records"] += 1
                moved = diff_keys(b[k]["record"], n[k]["record"]) | (diff_keys(b[k], n[k]) - {"record"})
                if moved - {"cost_gp"}:
                    bad.append(f"{path} {k}: {sorted(moved)}")
    elif "/equipment/" in path and path.startswith("data/corpus/"):
        counts["equipment_records"] += 1
        top = diff_keys(before, new)
        if top - {"data"}:
            bad.append(f"{path}: {sorted(top)}")  # license, pi_field, pi_marker, population, source never
        moved = diff_keys(before["data"], new["data"])
        if moved - {"cost_gp", "weight_lbs", "name", "raw_tokens"}:
            bad.append(f"{path}: data {sorted(moved)}")
    elif path.startswith("data/rules_tables/"):
        counts["tables"] += 1
        if diff_keys(before, new) - {"rows"} or len(before["rows"]) != len(new["rows"]):
            bad.append(f"{path}: envelope or row count moved")
        for b, n in zip(before["rows"], new["rows"]):
            if b != n:
                counts["table_rows"] += 1
                if diff_keys(b, n) - {"cost_gp"}:
                    bad.append(f"{path} {b.get('key')}: {sorted(diff_keys(b, n))}")
    else:
        bad.append(f"{path}: outside the convert lane's data paths")
print(counts)
if bad:
    print("\n".join(bad[:50])); print(f"VIOLATIONS {len(bad)}"); sys.exit(1)
print("FIELDS_OK")
