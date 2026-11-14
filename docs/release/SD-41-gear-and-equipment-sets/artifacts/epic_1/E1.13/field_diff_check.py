"""E1.13 field-diff check: on the re-ingested corpus equipment records only `data.raw_tokens` moved
(license, pi_*, population, source and every other field unchanged). Usage, from the repo root:
    python3 -I docs/release/SD-41-gear-and-equipment-sets/artifacts/epic_1/E1.13/field_diff_check.py <base-rev>
"""
import json, subprocess, sys

base = sys.argv[1] if len(sys.argv) > 1 else "HEAD"
files = [f for f in subprocess.run(["git", "diff", "--name-only", base, "--", "data/corpus"], capture_output=True, text=True, check=True).stdout.split() if "/equipment/" in f]
bad = 0
for f in files:
    a = json.loads(subprocess.run(["git", "show", f"{base}:{f}"], capture_output=True, text=True, check=True).stdout)
    b = json.load(open(f))
    top = {k for k in set(a) | set(b) if a.get(k) != b.get(k)}
    data = {k for k in set(a["data"]) | set(b["data"]) if a["data"].get(k) != b["data"].get(k)}
    baseqty = [t["value"] for t in b["data"].get("raw_tokens", []) if t["key"] == "BASEQTY"]
    print(f, sorted(top), sorted(data), "BASEQTY", baseqty)
    bad += bool(top - {"data"}) or bool(data - {"raw_tokens"})
print("records", len(files), "violations", bad)
print("FIELDS_OK" if bad == 0 else "FIELDS_VIOLATED")
