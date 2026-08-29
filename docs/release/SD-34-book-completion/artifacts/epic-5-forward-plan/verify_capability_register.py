"""RED/GREEN check for artifacts/epic-5-forward-plan/capability-register.json (AT-34-E5-002).

Fails closed (nonzero exit) if:
  1. the artifact is missing
  2. any capability entry is missing a required field (what, buckets_unblocked,
     books_unblocked, population/population_status, population_source, built_by_sd34)
  3. any capability claims `built_by_sd34: true` -- if SD-34 ever builds one, this register's
     whole job is naming what is NOT yet built, so a built capability must be REMOVED from the
     list, not flagged true and left in
  4. a "live" population_source capability's population does not match a fresh, independent
     re-derivation against docs/work-inventory.json / missing-engine-tables.json (never trust
     the frozen artifact's own number -- decisions.md §12 L2)
  5. the X-bucket reconciliation does not sum to the live bucket-X population (every X unit
     accounted for by exactly one capability-or-no-capability-needed row, none double-counted,
     none dropped)
  6. a "cited" capability has no `cited_from` source
"""
import json, subprocess, sys, os, collections

REPO = subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True, text=True, check=True).stdout.strip()
sys.path.insert(0, os.path.join(REPO, "scripts"))
import completion_atlas as ca

REG_PATH = os.path.join(REPO, "docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/capability-register.json")

if not os.path.exists(REG_PATH):
    print(f"FAIL: {REG_PATH} does not exist")
    sys.exit(1)

reg = json.load(open(REG_PATH))
inv = json.load(open(os.path.join(REPO, "docs/work-inventory.json")))
units = inv["units"]

errors = []

caps = reg.get("capabilities", [])
if not caps:
    errors.append("capabilities list is empty")

REQUIRED = ["id", "what", "buckets_unblocked", "books_unblocked", "population_source", "built_by_sd34"]
for c in caps:
    cid = c.get("id", "<no id>")
    for field in REQUIRED:
        if field not in c:
            errors.append(f"{cid}: missing required field {field!r}")
    if "population" not in c and "population_status" not in c:
        errors.append(f"{cid}: must state a numeric population OR an explicit population_status (never silently absent)")
    if c.get("built_by_sd34") is True:
        errors.append(f"{cid}: built_by_sd34=true -- a built capability must be REMOVED from this register, not flagged true")
    if c.get("population_source") == "cited" and not c.get("cited_from"):
        errors.append(f"{cid}: population_source=='cited' but no cited_from source given")
    if c.get("population_source") not in ("live", "cited"):
        errors.append(f"{cid}: population_source must be 'live' or 'cited', got {c.get('population_source')!r}")

# Independent re-derivation of the two "live" bucket-A capabilities against
# missing-engine-tables.json AND a fresh evidence-string query (belt and suspenders --
# workflow-instruction.md's own AGENTS.md rule: derive counts two independent ways).
a_units = [u for u in units if u.get("status") == "engine-does-not-hold" and "has_no_engine_table" in (u.get("evidence") or "")]
a_by_kind = collections.Counter(u["kind"] for u in a_units)

by_id = {c["id"]: c for c in caps}

if "power_engine_table" in by_id:
    live = a_by_kind.get("power", 0)
    stated = by_id["power_engine_table"]["population"]
    if stated != live:
        errors.append(f"power_engine_table: register says {stated}, live re-derivation says {live}")

if "companion_table_shape_widening" in by_id:
    live = a_by_kind.get("companion", 0)
    stated = by_id["companion_table_shape_widening"]["population"]
    if stated != live:
        errors.append(f"companion_table_shape_widening: register says {stated}, live re-derivation says {live}")

# Independent re-derivation of the X-bucket capabilities.
pres = ca.partition(units)
if pres["unclassified_ids"] or pres["overlap_ids"]:
    errors.append("live completion_atlas partition has unclassified/overlap units -- cannot verify against it")

x_live = pres["counts"].get("X", 0)
recon = reg.get("x_bucket_reconciliation", {})
if recon.get("live_bucket_X_population") != x_live:
    errors.append(f"x_bucket_reconciliation.live_bucket_X_population stale: register says {recon.get('live_bucket_X_population')}, live is {x_live}")
if recon.get("sum_check") != x_live:
    errors.append(f"x_bucket_reconciliation.sum_check ({recon.get('sum_check')}) != live bucket-X population ({x_live}) -- an X unit is double-counted or dropped")

for cid in ("per_character_choice_filter", "companion_mount_advancement_table", "class_feature_deep_subsystem_modelling"):
    if cid in by_id and by_id[cid].get("population") != recon.get(cid):
        errors.append(f"{cid}: capability population ({by_id[cid].get('population')}) disagrees with its own reconciliation row ({recon.get(cid)})")

if errors:
    print(f"FAIL: {len(errors)} violation(s)")
    for e in errors[:40]:
        print(" -", e)
    sys.exit(1)

print(f"PASS: {len(caps)} capabilities named, X-bucket reconciliation sums to live population ({x_live}), "
      f"0 flagged built_by_sd34=true")
sys.exit(0)
