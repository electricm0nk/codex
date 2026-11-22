"""E1.MC corpus field check over the whole epic (C1 -> HEAD): every data/ file the epic moved, field by
field, against the fields an E1 card was granted. Usage, from the repo root:
python3 -I docs/release/SD-41-gear-and-equipment-sets/artifacts/epic_1/E1.MC/epic_field_check.py c817ad502f
Prints per-field tallies and FIELDS_OK, or every violation and exits 1.
Granted settled-record fields: cost_gp (E1.9, E1.11); container, container_capacity (E1.1, E1.12);
uses_ammunition, ammunition_family, ammunition_kinds, base_quantity (E1.6, E1.7, E1.13).
Granted corpus-record data fields: cost_gp, weight_lbs, name, raw_tokens (E1.9, E1.13); license, pi_*,
population, source never. Granted race-trait settled field: climb_ft (E1.14). New files: the two
price_overrides lists (E1.9) and the capacity override list (E1.12)."""
import json, subprocess, sys
from collections import Counter

base = sys.argv[1]
names = subprocess.run(["git", "diff", "--name-only", base, "HEAD", "--", "data"], capture_output=True, text=True, check=True).stdout.split()
SETTLED = {"cost_gp", "container", "container_capacity", "uses_ammunition", "ammunition_family", "ammunition_kinds", "base_quantity"}
DATA = {"cost_gp", "weight_lbs", "name", "raw_tokens"}
NEW_OK = {"data/price_overrides/d20pfsrd-prices.json", "data/price_overrides/price-unknown.json", "data/capacity_overrides/container-capacities.json"}
# E1.13's disclosed move (its receipt, "Settled diff"): the 4 enriched CRB bundles take their copy line's EQMOD.
E113 = {f"arms_armor/{n}.json" for n in ("arrows_20", "bolts_crossbow_10", "bolts_crossbow_repeating_5", "bullets_sling_10")}
bad, files, fields = [], Counter(), Counter()

def at(rev, path):
    r = subprocess.run(["git", "show", f"{rev}:{path}"], capture_output=True, text=True)
    return json.loads(r.stdout) if r.returncode == 0 else None

def diff_keys(a, b):
    return {k for k in set(a) | set(b) if a.get(k) != b.get(k)}

def flat(v, prefix=""):
    if isinstance(v, dict):
        out = {}
        for k, x in v.items():
            out.update(flat(x, f"{prefix}{k}."))
        return out or {prefix.rstrip("."): {}}
    return {prefix.rstrip("."): v}

def settled(path, allowed, kind):
    files[kind] += 1
    b, n = at(base, path), at("HEAD", path)
    if diff_keys(b, n) - {"records"}:
        bad.append(f"{path}: envelope moved")
    br, nr = b["records"], n["records"]
    if set(br) != set(nr):
        bad.append(f"{path}: record population moved")
    for k in set(br) & set(nr):
        if br[k] != nr[k]:
            fb, fn = flat(br[k]), flat(nr[k])
            moved = diff_keys(fb, fn)
            fields.update(f"{kind}.{m}" for m in moved)
            ok = allowed | ({"eqmod_references"} if path.startswith("data/corpus/core_rulebook/") and k in E113 else set())
            off = sorted(m for m in moved if not set(m.split(".")) & ok)
            if off:
                bad.append(f"{path} {k}: {off}")

for path in names:
    if at(base, path) is None:
        files["new"] += 1
        if path not in NEW_OK:
            bad.append(f"{path}: new file not granted")
        continue
    if at("HEAD", path) is None:
        bad.append(f"{path}: deleted"); continue
    if path.endswith("/_settled/equipment.json"):
        settled(path, SETTLED, "settled_equipment")
    elif path.endswith("/_settled/race_trait.json"):
        settled(path, {"climb_ft"}, "settled_race_trait")
    elif path.startswith("data/corpus/") and "/equipment/" in path:
        files["equipment_records"] += 1
        b, n = at(base, path), at("HEAD", path)
        top = diff_keys(b, n)
        if top - {"data"}:
            bad.append(f"{path}: {sorted(top - {'data'})}")
        moved = diff_keys(b["data"], n["data"])
        fields.update(f"record.{m}" for m in moved)
        if moved - DATA:
            bad.append(f"{path}: data {sorted(moved - DATA)}")
    elif path.startswith("data/rules_tables/") and "equipment" in path:
        files["tables"] += 1
        b, n = at(base, path), at("HEAD", path)
        if diff_keys(b, n) - {"rows"} or len(b["rows"]) != len(n["rows"]):
            bad.append(f"{path}: envelope or row count moved")
        for rb, rn in zip(b["rows"], n["rows"]):
            if rb != rn:
                fields.update(f"table.{m}" for m in diff_keys(rb, rn))
                if diff_keys(rb, rn) - {"cost_gp"}:
                    bad.append(f"{path} {rb.get('key')}: {sorted(diff_keys(rb, rn))}")
    else:
        bad.append(f"{path}: outside every E1 data grant")
print(dict(files)); print(dict(sorted(fields.items())))
if bad:
    print("\n".join(bad)); print(f"VIOLATIONS {len(bad)}"); sys.exit(1)
print("FIELDS_OK")
